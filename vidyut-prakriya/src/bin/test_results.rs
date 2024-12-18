//! Evaluates a list of tinantas.
use clap::Parser;
use rayon::prelude::*;
use serde::Deserialize;
use std::error::Error;
use std::io::Write;
use std::path::PathBuf;
use vidyut_prakriya::args::{
    BaseKrt, Dhatu, Gana, Krdanta, Linga, Sanadi, Subanta, Tinanta, Vacana, Vibhakti,
};
use vidyut_prakriya::dhatupatha;
use vidyut_prakriya::private::check_file_hash;
use vidyut_prakriya::Vyakarana;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(long)]
    test_cases: PathBuf,

    #[arg(long)]
    data_type: String,

    #[arg(long)]
    hash: String,
}

#[derive(Debug, Deserialize)]
struct KrdantaRow {
    padas: String,
    dhatu: String,
    gana: Gana,
    number: u16,
    sanadi: String,
    krt: BaseKrt,
    linga: Linga,
    vibhakti: Vibhakti,
    vacana: Vacana,
}

fn parse_sanadi(val: &str) -> Result<Vec<Sanadi>, vidyut_prakriya::Error> {
    if val.is_empty() {
        Ok(Vec::new())
    } else {
        val.split('+')
            .map(|x| x.parse())
            .collect::<Result<Vec<Sanadi>, _>>()
    }
}

fn test_tinanta(line: &str) -> Result<(), Box<dyn Error>> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(line.as_bytes());
    let mut r = csv::StringRecord::new();
    assert!(reader.read_record(&mut r).unwrap());

    let expected: Vec<_> = r[0].split('|').collect();

    let upadesha = &r[1];
    let gana = &r[2];
    let number = &r[3];
    let dhatu = dhatupatha::create_dhatu(upadesha, gana.parse()?, number.parse()?)?;
    let sanadi = parse_sanadi(&r[4])?;
    let prayoga = r[5].parse()?;
    let lakara = r[6].parse()?;
    let purusha = r[7].parse()?;
    let vacana = r[8].parse()?;

    // TODO: this is very clumsy!
    let mut builder = Dhatu::builder()
        .aupadeshika(dhatu.aupadeshika().expect("ok"))
        .gana(dhatu.gana().expect("ok"))
        .prefixes(dhatu.prefixes())
        .sanadi(&sanadi);

    if let Some(x) = dhatu.antargana() {
        builder = builder.antargana(x);
    }

    let dhatu = builder.build()?;

    let tinanta_args = Tinanta::builder()
        .dhatu(dhatu.clone())
        .prayoga(prayoga)
        .purusha(purusha)
        .vacana(vacana)
        .lakara(lakara)
        .build()?;

    let v = Vyakarana::builder().log_steps(false).build();
    let prakriyas = v.derive_tinantas(&tinanta_args);
    let mut actual: Vec<_> = prakriyas.iter().map(|p| p.text()).collect();
    actual.sort();
    actual.dedup();

    if expected != actual {
        let lakara = &r[5];
        let purusha = &r[6];
        let vacana = &r[7];
        let code = format!("{:0>2}.{:0>4}", gana, number);
        let upadesha = dhatu.aupadeshika().expect("ok");

        let mut out = std::io::stdout().lock();
        writeln!(
            out,
            "[ FAIL ]  {code:<10} {upadesha:<10} {lakara:<10} {purusha:<10} {vacana:<10}"
        )?;
        writeln!(out, "          Expected: {:?}", expected)?;
        writeln!(out, "          Actual  : {:?}", actual)?;
    }

    Ok(())
}

fn test_krdanta(r: Result<KrdantaRow, csv::Error>) -> Result<(), Box<dyn Error>> {
    let r = r?;
    let expected: Vec<_> = r.padas.split('|').filter(|x| !x.is_empty()).collect();

    let upadesha = r.dhatu;
    let gana = r.gana;
    let number = r.number;
    let sanadi = parse_sanadi(&r.sanadi)?;
    let krt: BaseKrt = r.krt;
    let linga: Linga = r.linga;
    let vibhakti: Vibhakti = r.vibhakti;
    let vacana: Vacana = r.vacana;

    let v = Vyakarana::builder().log_steps(false).build();
    let dhatu = dhatupatha::create_dhatu(upadesha, gana, number)?.with_sanadi(&sanadi);
    let krdanta = Krdanta::builder().dhatu(dhatu.clone()).krt(krt).build()?;
    let subanta = Subanta::new(krdanta, linga, vibhakti, vacana);
    let prakriyas = v.derive_subantas(&subanta);
    let mut actual: Vec<_> = prakriyas.iter().map(|p| p.text()).collect();
    actual.sort();
    actual.dedup();

    if expected != actual {
        let code = format!("{:0>2}.{:0>4}", gana, number);
        let upadesha = dhatu.aupadeshika().expect("ok");

        let mut out = std::io::stdout().lock();
        writeln!(out, "[ FAIL ]  {code:<10} {upadesha:<10} {krt:<10}")?;
        writeln!(out, "          Expected: {:?}", expected)?;
        writeln!(out, "          Actual  : {:?}", actual)?;
    }

    Ok(())
}

fn test_dhatu(line: &str) -> Result<(), Box<dyn Error>> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(line.as_bytes());
    let mut r = csv::StringRecord::new();
    assert!(reader.read_record(&mut r).unwrap());

    let expected: Vec<_> = r[0].split('|').filter(|x| !x.is_empty()).collect();
    let aupadeshika = &r[1];
    let gana: Gana = r[2].parse()?;
    let number: u16 = r[3].parse()?;
    let sanadi = parse_sanadi(&r[4])?;

    let dhatu = dhatupatha::create_dhatu(aupadeshika, gana, number)?.with_sanadi(&sanadi);

    let v = Vyakarana::builder().log_steps(false).build();
    let prakriyas = v.derive_dhatus(&dhatu);
    let mut actual: Vec<_> = prakriyas.iter().map(|p| p.text()).collect();
    actual.sort();
    actual.dedup();

    if expected != actual {
        let code = format!("{gana:0>2}.{number:0>4}");
        let upadesha = dhatu.aupadeshika().expect("ok");

        let mut out = std::io::stdout().lock();
        writeln!(out, "[ FAIL ]  {code:<10} {upadesha:<10} {sanadi:?}")?;
        writeln!(out, "          Expected: {expected:?}")?;
        writeln!(out, "          Actual  : {actual:?}")?;
    }

    Ok(())
}

fn run(args: Args) -> Result<(), Box<dyn Error>> {
    check_file_hash(&args.test_cases, &args.hash);

    let file = std::fs::read_to_string(&args.test_cases)?;

    if args.data_type == "krdanta" {
        let mut r = csv::Reader::from_reader(file.as_bytes());
        r.deserialize().par_bridge().for_each(|row| {
            match test_krdanta(row) {
                Ok(()) => (),
                Err(e) => println!("ERROR: Row is malformed: {e}"),
            };
        });
    } else if args.data_type == "tinanta" {
        file.lines()
            .skip(1)
            .par_bridge()
            .for_each(|line| match test_tinanta(line) {
                Ok(_) => (),
                Err(_) => println!("ERROR: Row is malformed: {line}"),
            });
    } else if args.data_type == "dhatu" {
        file.lines()
            .skip(1)
            .par_bridge()
            .for_each(|line| match test_dhatu(line) {
                Ok(_) => (),
                Err(_) => println!("ERROR: Row is malformed: {line}"),
            });
    }

    Ok(())
}

fn main() {
    let args = Args::parse();

    match run(args) {
        Ok(()) => (),
        Err(err) => {
            println!("{}", err);
            std::process::exit(1);
        }
    }
}
