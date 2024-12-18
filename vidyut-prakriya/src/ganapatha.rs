/*!
Implements rules from the Gaṇapāṭha.

Most of these lists come directly from the Kashika Vrtti. If a list is unclear to us, we adjust it
using data from <https://ashtadhyayi.com/ganapath>.
*/

/// 1.1.27 sarvAdIni sarvanAmAni (1)
pub const SARVA_ADI: &[&str] = &[
    "sarva", "viSva", "uBa", "uBaya", "qatara", "qatama", "anya", "anyatara", "itara", "tvat",
    "tva", "nema", "sama", "sima", "pUrva", "para", "avara", "dakziRa", "uttara", "apara", "aDara",
    "sva", "antara", "tyad", "tad", "yad", "etad", "idam", "adas", "eka", "dvi", "yuzmad", "asmad",
    "Bavatu~", "kim",
];

/// 1.1.38 svarAdi-nipAtam avyayam (2)
pub const SVAR_ADI: &[&str] = &[
    "svar",
    "antar",
    "prAtar",
    "punar",
    "sanutar",
    "uccEs",
    "nIcEs",
    "SanEs",
    "fDak",
    "fte",
    "yugapat",
    "ArAt",
    "antikAt",
    "pfTak",
    "hyas",
    "Svas",
    "divA",
    "rAtrO",
    "sAyam",
    "ciram",
    "manAk",
    "Izat",
    "SaSvat",
    "jozam",
    "tUzRIm",
    "bahis",
    "aDas",
    "avas",
    "samayA",
    "nikazA",
    "svayam",
    "mfzA",
    "naktam",
    "naY",
    "hetO",
    "he",
    "hE",
    "idDA",
    "adDA",
    "sAmi",
    "sanA",
    "sanat",
    "sanAt",
    "upaDA",
    "tiras",
    "antarA",
    "antareRa",
    "mak",
    "jyok",
    "yok",
    "nak",
    "kam",
    "Sam",
    "sanA",
    "sahasA",
    "SradDA",
    "alam",
    "svaDA",
    "vazaw",
    "vinA",
    "nAnA",
    "svasti",
    "anyat",
    "asti",
    "upAMSu",
    "kzamA",
    "vihAyasA",
    "dozA",
    "mfzA",
    "muDA",
    "dizwyA",
    "vfTA",
    "miTyA",
    "purA",
    "miTo",
    "miTas",
    "prAyas",
    "muhus",
    "pravAhukam",
    "pravAhikA",
    "Aryahalam",
    "aBIkzRam",
    "sAyam",
    "sAkam",
    "sArDam",
    "sArTam",
    "satram",
    "samam",
    "namas",
    "hiruk",
    "aTa",
    "am",
    "Am",
    "pratAm",
    "pratAn",
    "praSAn",
    "taTAhi",
    "mA",
    "mAN",
    "Sram",
    "kAmam",
    "prakAmam",
    "BUyas",
    "param",
    "sAkzAt",
    "sAci",
    "sAvi",
    "satyam",
    "maMkzu",
    "saMvat",
    "avaSyam",
    "sapadi",
    "prAdus",
    "Avis",
    "aniSam",
    "nityam",
    "nityadA",
    "sadA",
    "ajasram",
    "santatam",
    "uzA",
    "om",
    "BUr",
    "Buvar",
    "Jawiti",
    "tarasA",
    "suzWu",
    "ku",
    "aYjasA",
    "a",
    "miTu",
    "amiTu",
    "viTak",
    "BAjak",
    "anvak",
    "cirAya",
    "ciram",
    "cirarAtrAya",
    "cirasya",
    "cireRa",
    "cirAt",
    "astam",
    "Anuzak",
    "anuzak",
    "anuzaw",
    "amnas",
    "amBas",
    "amnar",
    "amBar",
    "sTAne",
    "varam",
    "duzWu",
    "balAt",
    "Su",
    "arvAk",
    "Sudi",
    "vadi",
    "ityAdi",
];

/// 1.4.57 cAdayo sattve (3)
/// TODO: no cet, na cet
pub const CA_ADI: &[&str] = &[
    "ca", "vA", "ha", "aha", "eva", "evam", "nUnam", "SaSvat", "yugapat", "sUpat", "kUpat",
    "kuvit", "net", "cet", "caR", "kaccit", "yatra", "naha", "hanta", "mAkim", "nakim", "mAN",
    "naY", "yAvat", "tAvat", "tvA", "tvE", "dvE", "rE", "SrOzaw", "vOzaw", "svAhA", "vazaw",
    "svaDA", "om", "kila", "taTA", "aTa", "su", "sma", "asmi", "a", "i", "u", "f", "x", "e", "E",
    "o", "O", "am", "tak", "uY", "ukaY", "velAyAm", "mAtrAyAm", "yaTA", "yat", "yam", "tat", "kim",
    "purA", "adDA", "Dik", "hAhA", "he", "hE", "pyAw", "pAw", "TAw", "aho", "utAho", "ho", "tum",
    "taTAhi", "Kalu", "Am", "Aho", "aTo", "nanu", "manye", "miTyA", "asi", "brUhi", "tu", "nu",
    "iti", "iva", "vat", "cana", "bata", "iha", "Sam", "kam", "anukam", "nahikam", "hikam",
    "sukam", "satyam", "ftam", "SfadDA", "idDA", "muDA", "jAtu", "kaTam", "kutaH", "kutra", "ava",
    "anu", "hAhO", "hEhA", "IhA", "Ahosvit", "Cambaw", "Kam", "dizwyA", "paSu", "vaw", "saha",
    "Anuzak", "aNga", "Paw", "tAjak", "aye", "are", "cawu", "bAw", "kum", "Kum", "Gum", "hum",
    "AIm", "SIm", "sIm", "vE",
];

/// 1.4.58 prAdayaH (4)
pub const PRA_ADI: &[&str] = &[
    "pra", "parA", "apa", "sam", "anu", "ava", "nis", "nir", "dus", "dur", "vi", "AN", "ni", "aDi",
    "api", "ati", "su", "ud", "aBi", "prati", "pari", "upa",
];

/// 1.4.61 UryAdi-cvi-qAcaS ca (5)
pub const URI_ADI: &[&str] = &[
    "UrI",
    "urarI",
    "tanTI",
    "tAlI",
    "AttAlI",
    "vetAlI",
    "DUlI",
    "DUsa",
    "SakalA",
    "saMSakalA",
    "DvaMsakalA",
    "BraMsakalA",
    "guluguDA",
    "sajUs",
    "Pala",
    "PalI",
    "viklI",
    "AklI",
    "AlozWI",
    "kevAlI",
    "kevAsI",
    "sevAsI",
    "paryAlI",
    "SevAlI",
    "varzAlI",
    "atyUmaSA",
    "maSmaSA",
    "masmasA",
    "masamasA",
    "Ozaw",
    "SrOzaw",
    "vOzaw",
    "vazaw",
    "svAhA",
    "svaDA",
    "vanDA",
    "prAdus",
    "Srat",
    "Avis",
];

/// 1.4.64 sAkzAt-praBftIni ca (6)
pub const SAKSHAT_PRABHRTI: &[&str] = &[
    "sAkzAt",
    "miTyA",
    "cintA",
    "BadrA",
    "rocanA",
    "AsTA",
    "amA",
    "SradDA",
    "prAjaryA",
    "prAjaruhA",
    "bIjaryA",
    "bIjaruhA",
    "saMsaryA",
    "arTe",
    "lavaRam",
    "uzRam",
    "SItam",
    "udakam",
    "Ardram",
    "agnO",
    "vaSe",
    "vihasane",
    "prasahane",
    "pratapane",
    "prAdus",
    "namas",
    "Avis",
];

/// 2.1.40 saptamI SORqEH (8)
pub const SHAUNDA_ADI: &[&str] = &[
    "SORqa", "DUrta", "kitava", "vyAqa", "pravIRa", "saMvIta", "antar", "aDi", "pawu", "paRqita",
    "capala", "nipuRa",
];

/// 2.1.70 kumAraH SramaRAdiBiH ()
pub const SHRAMANA_ADI: &[&str] = &[
    "SramaRA",
    "pravrajitA",
    "kulawA",
    "garBiRI",
    "tApasI",
    "dAsI",
    "banDakI",
    "aDyApaka",
    "aBirUpaka",
    "paRqita",
    "pawu",
    "mfdu",
    "kuSala",
    "capala",
    "nipuRA",
];

/// 2.2.9 yAjakAdiBiS ca ()
pub const YAJAKA_ADI: &[&str] = &[
    "yAjaka",
    "pUjaka",
    "paricAraka",
    "parivezaka",
    "parizecaka",
    "snAtaka",
    "aDyApaka",
    "utsAha",
    "udvartaka",
    "hotf",
    "Batf",
    "raTagaRaka",
    "pattigaRaka",
];

/// 3.1.11 BrzAdiByo Buvyacver lopaSca halaH (32)
pub const BHRSHA_ADI: &[&str] = &[
    "BfSa",
    "SIGra",
    "capala",
    "manda",
    "paRqita",
    "utsuka",
    "sumanas",
    "durmanas",
    "aBimanas",
    "unmanas",
    "rahas",
    "rohat",
    "rehat",
    "saMScat",
    "tfpat",
    "SaSvat",
    "Bramat",
    "vehat",
    "Sucis",
    "Sucivarcas",
    "aRqara",
    "varcas",
    "ojas",
    "surajas",
    "arajas",
];

/// 3.1.13 lohitAdi-qAj-ByaH kyaz (33)
pub const LOHITA_ADI: &[&str] = &[
    "lohita", "carita", "nIla", "Pena", "madra", "hari", "dAsa", "man",
    // This is an Akrti-gana, so other terms are included here too.
    "carman",
];

/// 3.1.17 suKAdiByaH kartf-vedanAyAm (34)
pub const SUKHA_ADI: &[&str] = &[
    "suKa", "duHKa", "tfpta", "kfcCra", "asra", "Asra", "aloka", "pratIpa", "karuRa", "kfpaRa",
    "so",
];

/// 3.1.27 kaRqvAdiByo yak (35)
#[allow(unused)]
pub const KANDU_ADI: &[&str] = &[
    "kaRqUY", "mantu", "mantuY", "valgu", "asu~", "asU", "asUY", "lew", "low", "lelA", "iras",
    "iraj", "iraY", "uzas", "veda", "meDA", "kuzuBa", "magaDa", "tantas", "pampas", "suKa",
    "duHKa", "sapara", "arara", "BizajU", "BizRaj", "izuDa", "caraRa", "varaRa", "curaRa",
    "turaRa", "BuraRa", "gadgada", "elA", "kelA", "KelA", "ilA", "leKA", "leKa", "liwa", "lAwa",
    "hfRIN", "mahIN", "reKA", "dravas", "tiras", "agada", "uras", "taraRa", "payas", "samBUyas",
    "ambara", "saMvara",
];

/// 4.1.4 ajAdyataz wAp (54)
pub const AJA_ADI: &[&str] = &[
    // jAti
    "aja",
    "eqaka",
    "kokila",
    "cawaka",
    "aSva",
    "mUzika",
    // age
    "bAla",
    "hoqa",
    "pAka",
    "vatsa",
    "manda",
    "vilAta",
    // lyuw
    "pUrvApaharaRa",
    "aparApaharaRa",
    // Pala
    "samPala",
    "BastraPala",
    "ajinaPala",
    "SaRaPala",
    "piRqaPala",
    "triPala",
    // puzpa
    "satpuzpa",
    "prAkpuzpa",
    "kARqapuzpa",
    "prAntapuzpa",
    "Satapuzpa",
    "ekapuzpa",
    // TODO: optional for SUdra in different senses.
    "SUdra",
    // halanta
    "kruYc",
    "uzRih",
    "devaviS",
    // matrimony
    "jyezWa",
    "kanizWa",
    "maDyama",
    // naY
    "amUla",
];

/// 4.1.10 na zaw-svasrAdiByaH (46)
pub const SVASR_ADI: &[&str] = &[
    "svasf", "duhitf", "nanAndf", "yAtf", "mAtf", "tisf", "catasf",
];

/// 4.1.41 zid-gOrAdiByaS ca (48)
pub const GAURA_ADI: &[&str] = &[
    "gOra",
    "matsya",
    "manuzya",
    "SfNga",
    "piNgala",
    "haya",
    "gavaya",
    "mukaya",
    "fzya",
    "puwa",
    "dURa",
    "druRa",
    "droRa",
    "hariRa",
    "kokaRa",
    "kAkaRa",
    "pawara",
    "uRaka",
    "Amala",
    "Amalaka",
    "kubala",
    "bimba",
    "badara",
    "Parkaraka",
    "karkaraka",
    "tarkAra",
    "SarkAra",
    "puzkara",
    "SiKaRqa",
    "sala",
    "SazkaRqa",
    "sananda",
    "suzama",
    "suzava",
    "alinda",
    "gaqu",
    "zARqaSa",
    "AQaka",
    "Ananda",
    "ASvatTa",
    "sfpAwa",
    "AKaka",
    "Apaccika",
    "Sazkula",
    "sUrya",
    "sUrma",
    "SUrpa",
    "sUpa",
    "vfsa",
    "atasa",
    "Ba",
    "BfNga",
    "maha",
    "maWa",
    "Ceda",
    "peSa",
    "meda",
    "Svan",
    "takzan",
    "anaquhI",
    "anaqvAhI",
    // "ezaRaH karaRe",
    "deha",
    "dehala",
    "kAkAdana",
    "gavAdana",
    "tejana",
    "rajana",
    "lavaRa",
    "OdgAhamAni",
    "AdgAhamAni",
    "gOtama",
    "gotama",
    "pAraka",
    "AyasTURa",
    "ayaHsTURa",
    "BOrika",
    "BOliki",
    "BOliNgi",
    "yAna",
    "meGa",
    "Alambi",
    "Alaji",
    "AlabDi",
    "Alakzi",
    "kevAla",
    "Apaka",
    "Arawa",
    "nawa",
    "wowa",
    "nowa",
    "mUlAwa",
    "SAtana",
    "potana",
    "pAtana",
    "pAWana",
    "pAnaWa",
    "AstaraRa",
    "aDikaraRa",
    "aDikAra",
    "AgrahAyaRI",
    "AgrahAyaRI",
    "pratyavarohiRI",
    "secana",
    // "sumaNgalAtsaMjYAyAm",
    "aRqara",
    "sundara",
    "maRqala",
    "manTara",
    "maNgala",
    "pawa",
    "piRwa",
    "zaRqa",
    "urda",
    "garda",
    "Sama",
    "sUd",
    "Oqa",
    "Ardra",
    "hfda",
    "hrada",
    "pARqa",
    "BARqala",
    "BARqa",
    "lohARqa",
    "kadara",
    "kandara",
    "kadala",
    "taruRa",
    "taluna",
    "kalmAza",
    "bfhat",
    "mahat",
    "soma",
    "sODarma",
    // "rohiRI nakzatre",
    // "revatI nakzatre",
    "vikala",
    "nizPala",
    "puzkala",
    // "kawAcCroRivacane",
    // "pippalyAdayaSca",
    "pippalI",
    "haritakI",
    "harItakI",
    "koSAtakI",
    "SamI",
    "varI",
    "SarI",
    "pfTivI",
    "krozwu",
    "mAtAmaha",
    "pitAmaha",
];

/// 4.1.45 bahvAdiByaS ca (49)
pub const BAHU_ADI: &[&str] = &[
    "bahu",
    "padDati",
    "aNkati",
    "aYcati",
    "aMhati",
    "vaMhati",
    "Sakawi",
    "Sakti",
    "SAri",
    "vAri ",
    "rAti",
    "rADi",
    "SADi",
    "ahi",
    "kapi",
    "yazwi",
    "muni",
    "caRqa",
    "arAla",
    "kamala",
    "kfpARa",
    "vikawa",
    "viSAla",
    "viSaNkawa",
    "Baruja",
    "Dvaja",
    "kalyARa",
    "udAra",
    "purARa",
    "ahan",
];

/// 4.1.84 aSvapatyAdiByaSca (53)
pub const ASHVAPATI_ADI: &[&str] = &[
    "aSvapati",
    "Satapati",
    "Danapati",
    "gaRapati",
    "rAzwrapati",
    "kulapati",
    "gfhapati",
    "DAnyapati",
    "paSupati",
    "Darmapati",
    "saBApati",
    "prARapati",
    "kzetrapati",
];

/// 4.1.86 utsAdiByo 'Y (54)
pub const UTSA_ADI: &[&str] = &[
    "utsa",
    "udapAna",
    "vikara",
    "vinoda",
    "mahAnada",
    "mahAnasa",
    "mahAprARa",
    "taruRa",
    "taluna",
    "bazkayAse",
    "Denu",
    "pfTivI",
    "paNkti",
    "jagatI",
    "trizwuB",
    "anuzwuB",
    "janapada",
    "Barata",
    "uSInara",
    "grIzma",
    "pIlu",
    "kula",
    "udasTAna",
    "pfzadaMSe",
    "BallakIya",
    "raTantara",
    "maDyandina",
    "bfhat",
    "mahat",
    "sattvat",
    "kuru",
    "paYcAla",
    "indrAvasAna",
    "uzRih",
    "kakuB",
    "suvarRa",
    "deva",
];

/// 4.1.96 bAhvAdiByazca (55)
pub const BAAHU_ADI: &[&str] = &[
    "bAhu",
    "upabAhu",
    "upavAku",
    "nivAku",
    "SivAku",
    "vawAku",
    "upanindu",
    "upabindu",
    "vfzalI",
    "vfkalA",
    "cUqA",
    "balAkA",
    "mUzikA",
    "kuSalA",
    "BagalA",
    "CagalA",
    "DruvakA",
    "DuvakA",
    "sumitrA",
    "durmitrA",
    "puzkarasad",
    "anuharat",
    "devaSarman",
    "agniSarman",
    "BadraSarman",
    "suSarman",
    "kunAman",
    "sunAman",
    "paYcan",
    "saptan",
    "azwan",
    // amitOjasa has sa-lopa
    "amitOjasa",
    "suDAvat",
    "udaYcu",
    "Siras",
    "mAza",
    "SarAvin",
    "marIcI",
    "kSemavfdDin",
    "SfNKalatodin",
    "KaranAdin",
    "nagaramardin",
    "prAkAramardin",
    "loman",
    "ajIgarta",
    "kfzra",
    "yuDizWira",
    "arjuna",
    "sAmba",
    "gada",
    "pradyumna",
    "rAma",
    "udaNku",
];

/// 4.1.98 gotre kuYjAdiByaS cPaY (56)
pub const KUNJA_ADI: &[&str] = &[
    "kuYja", "braDna", "SaNKa", "Basman", "gaRa", "loman", "SaWa", "SAka", "SAkawa", "SuRqA",
    "SuBa", "vipASa", "skanda", "stamBa",
];

/// 4.1.99 naqAdiByaH Pak (57)
pub const NADA_ADI: &[&str] = &[
    "naqa",
    "cara",
    "baka",
    "muYja",
    "itika",
    "itiSa",
    "upaka",
    "lamaka",
    "SalaNku",
    "SalaNkam",
    "saptala",
    "vAjapya",
    "tika",
    "agniSarman",
    "prARa",
    "nara",
    "sAyaka",
    "dAsa",
    "mitra",
    "dvIpa",
    "piNgara",
    "piNgala",
    "kiNkara",
    "kiNkala",
    "kAtara",
    "kAtala",
    "kASya",
    "kASyapa",
    "kAvya",
    "aja",
    "amuzya",
    "kfzRa",
    "raRa",
    "amitra",
    "ligu",
    "citra",
    "kumAra",
    "krozwu",
    "krozwam",
    "loha",
    "durga",
    "stamBa",
    "SiMSipA",
    "agra",
    "tfRa",
    "Sakawa",
    "sumanas",
    "sumata",
    "mimata",
    "fk",
    "jat",
    "yuganDara",
    "haMsaka",
    "daRqin",
    "hastin",
    "paYcAla",
    "camasin",
    "sukftya",
    "sTiraka",
    "brAhmaRa",
    "cawaka",
    "badara",
    "aSvala",
    "Karapa",
    "kAmuka",
    "vrahmadatta",
    "udumbara",
    "SoRa",
    "aloha",
    "daRqa",
];

/// For 4.1.104.
pub const BIDA_ADI: &[&str] = &[
    "bida",
    "urva",
    "kaSyapa",
    "kuSika",
    "BaradvAja",
    "upamanyu",
    "kilAlapa",
    "kidarBa",
    "viSvAnara",
    "fzwizeRa",
    "ftaBAga",
    "haryaSva",
    "priyaka",
    "Apastamba",
    "kUcavAra",
    "Saradvat",
    "Sunaka",
    "Denu",
    "gopavana",
    "Sigru",
    "bindu",
    "BAjana",
    "aSvAvatAna",
    "SyAmAka",
    "SyamAka",
    "SyAparRa",
    "harita",
    "kindAsa",
    "vahraska",
    "arkalUza",
    "vaDyoza",
    "vizRuvfdDa",
    "pratiboDa",
    "raTAntara",
    "raTItara",
    "gavizWira",
    "nizAda",
    "maWara",
    "mfda",
    "punarBU",
    "putra",
    "duhitf",
    "nanAndf",
    "parastrI",
];

/// For 4.1.105.
pub const GARGA_ADI: &[&str] = &[
    "garga",
    "vatsa",
    "vAjAse",
    "saMskfti",
    "aja",
    "vyAGrapAt",
    "vidaBft",
    "prAcInayoga",
    "agasti",
    "pulasti",
    "reBa",
    "agniveSa",
    "SaNKa",
    "SaWa",
    "GUma",
    "avawa",
    "camasa",
    "DanaYjaya",
    "manasa",
    "vfkza",
    "viSvAvasu",
    "janamAna",
    "lohita",
    "SaMsita",
    "baBru",
    "maRqu",
    "makzu",
    "aligu",
    "SaNku",
    "ligu",
    "gulu",
    "mantu",
    "jigIzu",
    "manu",
    "tantu",
    "manAyI",
    "BUta",
    "kaTaka",
    "kaza",
    "taRqa",
    "vataRqa",
    "kapi",
    "kata",
    "kurukata",
    "anaquH",
    "kaRva",
    "Sakala",
    "gokakza",
    "agastya",
    "kuRqina",
    "yajYavalka",
    "uBaya",
    "jAta",
    "virohita",
    "vfzagaRa",
    "rahUgaRa",
    "SaRqila",
    "vaRa",
    "kaculuka",
    "mudgala",
    "musala",
    "parASara",
    "jatUkarRa",
    "mAntrita",
    "saMhita",
    "aSmaraTa",
    "SarkarAkza",
    "pUtimAza",
    "sTURa",
    "araraka",
    "piNgala",
    "kfzRa",
    "golunda",
    "ulUka",
    "titikza",
    "Bizaj",
    "Baqita",
    "BaRqita",
    "dalBa",
    "cikita",
    "devahU",
    "indrahU",
    "ekalU",
    "pippalU",
    "vfdagni",
    "jamadagni",
    "suloBin",
    "ukatTa",
    "kuwIgu",
];

/// 4.1.110 aSvAdiByaH PaY (60)
pub const ASHVA_ADI: &[&str] = &[
    "aSva",
    "aSman",
    "SaNKa",
    "pawu",
    "rohiRa",
    "KarjUra",
    "KarjUla",
    "piYjUra",
    "Baqila",
    "BaRqila",
    "Baqita",
    "BaRqita",
    "BaRqika",
    "prahfta",
    "romAda",
    "kzatra",
    "grIvA",
    "kASa",
    "golANkya",
    "arka",
    "svana",
    "Dvana",
    "pAda",
    "cakra",
    "kula",
    "pavitra",
    "gomin",
    "SyAma",
    "DUma",
    "DUmra",
    "vAgmin",
    "viSvAnara",
    "kuwa",
    "veSa",
    "Sapa",
    "natta",
    "taqa",
    "naqa",
    "grIzma",
    "arha",
    "viSamya",
    "viSAlA",
    "giri",
    "capala",
    "cunama",
    "dAsaka",
    "vElya",
    "Darma",
    "Anaquhra",
    "puMsijAta",
    "arjuna",
    "SUdraka",
    "sumanas",
    "durmanas",
    "kzAnta",
    "prAcya",
    "kita",
    "kARa",
    "cumba",
    "SravizWA",
    "vIkzya",
    "pavindA",
    "Atreya",
    "kutsa",
    "Atava",
    "kitava",
    "Siva",
    "Kadira",
    "BAradvAja",
];

/// 4.1.112 SivAdiByo 'R (61)
pub const SHIVA_ADI: &[&str] = &[
    "Siva",
    "prOzWa",
    "prOzWika",
    "caRqa",
    "jamBa",
    "muni",
    "sanDi",
    "BUri",
    "kuWAra",
    "anaBimlAna",
    "kakutsTa",
    "kahoqa",
    "leKa",
    "roDa",
    "KaYjana",
    "kohaqa",
    "pizwa",
    "hehaya",
    "KaYjAra",
    "KaYjAla",
    "surohikA",
    "parRa",
    "kahUza",
    "parila",
    "vataRqa",
    "tfRa",
    "karRa",
    "kzIrahfda",
    "jalahfda",
    "parizika",
    "jawilika",
    "goPilika",
    "baDirikA",
    "maYjIraka",
    "vfzRika",
    "reKa",
    "AleKana",
    "viSravaRa",
    "ravaRa",
    "vartanAkza",
    "piwaka",
    "piwAka",
    "tfkzAka",
    "naBAka",
    "UrRanABa",
    "jaratkAru",
    "utkzipA",
    "rohitika",
    "AryaSveta",
    "supizwa",
    "KarjUrakarRa",
    "masUrakarRa",
    "tURakarRa",
    "mayUrakarRa",
    "Kaqaraka",
    "takzan",
    "fzwizeRa",
    "gaNgA",
    "vipASa",
    "yaska",
    "lahra",
    "druhyu",
    "ayaHsTURa",
    "Balandana",
    "virUpAkza",
    "BUmi",
    "ilA",
    "sapatnI",
    "dvyaca",
];

/// 4.1.146 revatyAdiByaz Wak (65)
pub const REVATI_ADI: &[&str] = &[
    "revatI",
    "aSvapAlI",
    "maRipAlI",
    "dvArapAlI",
    "vfkavaYcin",
    "vfkabanDu",
    "vfkagrAha",
    "karRagrAha",
    "daRqagrAha",
    "kukkUwAkza",
    "kakudAkza",
    "cAmaragrAha",
];

/// For 4.1.123.
pub const SHUBHRA_ADI: &[&str] = &[
    "SuBra",
    "vizwapura",
    // TODO: many others
];

/// For 4.1.126.
pub const KALYANI_ADI: &[&str] = &[
    "kalyARI",
    "suBagA",
    "durBagA",
    "banDakI",
    "anudfzwi",
    "anusfzwi",
    "jaratI",
    "balIvardI",
    "jyezWA",
    "kanizWA",
    "maDyamA",
    "parastrI",
];

/// For 4.2.38.
pub const BHIKSHA_ADI: &[&str] = &[
    "BikzA", "garBiRI", "kzetra", "karIza", "aNgAra", "carmin", "Darmin", "sahasra", "yuvati",
    "padAti", "padDati", "aTarvan", "dakziRA", "BUta",
];

/// For 4.2.45.
pub const KHANDIKA_ADI: &[&str] = &[
    "KaRqikA",
    "vaqavA",
    "kzudrakamAla",
    "Bikzuka",
    "Suka",
    "ulUka",
    "Svan",
    "yuga",
    "ahan",
    "varatrA",
    "halabanDa",
];

/// 4.2.49 pASAdiByo yaH (74)
pub const PASHA_ADI: &[&str] = &[
    "pASa", "tfRa", "DUma", "vAta", "aNgAra", "pota", "bAlaka", "piwaka", "piwAka", "Sakawa",
    "hala", "naqa", "vana",
];

/// 4.2.53 rAjyanAdiByo vuY (76)
pub const RAJANYA_ADI: &[&str] = &[
    "rAjanya",
    "Anfta",
    "bABravya",
    "SAlaNkAyana",
    "dEvayAtava",
    "dEvayAta",
    "vrIqa",
    "varatra",
    "jAlaMDarAyaRa",
    "rAjAyana",
    "telu",
    "AtmakAmeya",
    "ambarIzaputra",
    "vasAti",
    "bElvavana",
    "SElUza",
    "udumbara",
    "tIvra",
    "bElvala",
    "ArjunAyana",
    "saMpriya",
    "dakzi",
    "UrRanABa",
];

/// 4.2.54 BorikyAdyEzukAryAdiByo viDal-BaktalO (77)
pub const BHAURIKI_ADI: &[&str] = &[
    "BOriki",
    "vEpeya",
    "BOliki",
    "cEpayata",
    "cEwayata",
    "kAReya",
    "vARijaka",
    "vARikAjya",
    "vAlikAjya",
    "sEkayata",
    "vEkayata",
];

/// 4.2.54 BorikyAdyEzukAryAdiByo viDal-BaktalO (78)
pub const AISHUKARI_ADI: &[&str] = &[
    "EzukAri",
    "sArasyAyana",
    "sArasAyana",
    "cAndrAyaRa",
    "dvyAkzAyaRa",
    "tryAkzAyaRa",
    "OqAyana",
    "jOlAyana",
    "KAqAyana",
    "dAsamitri",
    "dAsamitrAyaRa",
    "SOdrAyaRa",
    "dAkzAyaRa",
    "SayaRqAyana",
    "SAyaRqAyana",
    "tArkzyAyaRa",
    "SOBrAyaRa",
    "sOvIra",
    "sOvIrAyaRa",
    "SapaRqa",
    "SayaRqa",
    "SORqa",
    "SayARqi",
    "SayARqa",
    "vESvamAnava",
    "vESvaDyenava",
    "vESvaDenava",
    "naqa",
    "tuRqadeva",
    "viSvadeva",
    "sApiRqi",
];

/// 4.2.61 kramAdiByo vun (78)
pub const KRAMA_ADI: &[&str] = &["krama", "pada", "SikzA", "mImAMsA", "sAman"];

/// For 4.2.75.
pub const SANKALA_ADI: &[&str] = &[
    "saNkala",
    "puzkala",
    "udvapa",
    "uqupa",
    "utpuwa",
    "kumBa",
    "viDAna",
    "sudakza",
    "sudatta",
    "suBUta",
    "sunetra",
    "supiNgala",
    "sikatA",
    "pUtIkI",
    "pUlasa",
    "kUlAsa",
    "palASa",
    "niveSa",
    "gaveza",
    "gamBIra",
    "itara",
    "Sarman",
    "ahan",
    "loman",
    "veman",
    "varuRa",
    "bahula",
    "sadyoja",
    "aBizikta",
    "goBft",
    "rAjaBft",
    "gfha",
    "Bfta",
    "Balla",
    "mAla",
    "vft",
];

/// For 4.2.77.
pub const SUVASTA_ADI: &[&str] = &[
    "suvAstu",
    "varRu",
    "BaRqu",
    "KaRqu",
    "secAlin",
    "karpUrin",
    "SiKaRdin",
    "garta",
    "karkaSa",
    "SawIkarRa",
    "kfzRa",
    "karka",
    "karNkaDU matI",
    "gohra",
    "ahisakTa",
    "vft",
];

/// For 4.2.86.
pub const MADHU_ADI: &[&str] = &[
    "maDu",
    "bisa",
    "sTARu",
    "muzwi",
    "ikzu",
    "veRu",
    "ramya",
    "fkza",
    "karkanDu",
    "SamI",
    "kirIra",
    "hima",
    "kiSarA",
    "SarpaRA",
    "marut",
    "maruva",
    "dArvAGAwa",
    "Sara",
    "izwakA",
    "takzaSilA",
    "Sakti",
    "AsandI",
    "Asuti",
    "SalAkA",
    "AmiDI",
    "KaqA",
    "vewA",
    "maDvAdiH",
];

/// For 4.2.95.
pub const KATRI_ADI: &[&str] = &[
    "katri",
    "umBi",
    "puzkara",
    "modana",
    "kumBI",
    "kuRqina",
    "nagara",
    "vaYjI",
    "Bakti",
    "mAhizmatI",
    "carmaRvatI",
    "grAma",
    "uKyA",
    // kuqyA takes ya-lopa
    "kuqyA",
    "katryAdiH",
];

/// 4.2.97 nadyAdiByo Qak (106)
pub const NADI_ADI: &[&str] = &[
    "nadI",
    "mahI",
    "vArARasI",
    "SrAvastI",
    "kOSAmbI",
    "navakOSAmbI",
    "kASaParI",
    "KAdirI",
    "pUrvanagarI",
    "pAvA",
    "mAvA",
    "sAlvA",
    "dArvA",
    "dAlvA",
    "vAsenakI",
    "vaqavA",
];

/// 4.2.86 maDvAdiByaS ca (102)
pub const KASHI_ADI: &[&str] = &[
    "kASi",
    "cedi",
    "bedi",
    "saMjYA",
    "saMvAha",
    "acyuta",
    "mohamAna",
    "SakulAda",
    "hastikarzU",
    "kudAman",
    "hiraRya",
    "karaRa",
    "goDASana",
    "BOriki",
    "BOliNgi",
    "arindama",
    "sarvamitra",
    "devadatta",
    "sADumitra",
    "dAsamitra",
    "dAsagrAma",
    "sODAvatAna",
    "yuvarAja",
    "uparAja",
    "sinDumitra",
    "devarAja",
    // ApadAdipUrvapadAt kAlAt
    "ApatkAla",
    "UrDvakAla",
    "tatkAla",
];

/// 4.2.133 kacCAdiByaS ca (110)
pub const KACCHA_ADI: &[&str] = &[
    "kacCa",
    "sinDu",
    "varRu",
    "ganDAra",
    "maDumat",
    "kamboja",
    "kaSmIra",
    "sAlva",
    "kuru",
    "raNku",
    "aRu",
    "KaRqa",
    "dvIpa",
    "anUpa",
    "ajavAha",
    "vijApakaH",
    "kulUna",
];

/// 4.2.138 gahAdiByaS ca (111)
pub const GAHA_ADI: &[&str] = &[
    "gaha",
    "antaHsTa",
    "sama",
    "vizama",
    "maDya",
    "maDyama",
    "uttama",
    "aNga",
    "vaNga",
    "magaDa",
    "pUrvapkza",
    "aparapakza",
    "aDamaSAKa",
    "uttamaSAKa",
    "samAnaSAKa",
    "ekagrAma",
    "ekavfkza",
    "ekapalASa",
    "ezvagra",
    "izvanI",
    "avasyandI",
    "kAmaprasTa",
    "KAqAyani",
    "kAveraRi",
    "SONgi",
    "Asuri",
    "AhiMsi",
    "Amitri",
    "vyAqi",
    "vEdaji",
    "BOji",
    "AQyaSvi",
    "AnfSaMsi",
    "sOvi",
    "pAraki",
    "agniSarman",
    "devaSarman",
    "SrOti",
    "Arawaki",
    "vAlmIki",
    "kzemavfdDin",
    "uttara",
    "antara",
    // TODO: remainder
    "jana",
    "para",
    "deva",
    "veRukA",
];

/// For 4.3.16.
pub const SANDHIVELA_ADI: &[&str] = &[];

/// For 4.3.54.
pub const DIG_ADI: &[&str] = &[
    "diS", "varga", "pUga", "gaRa", "pakza", "DAyyA", "mitra", "meDA", "antara", "paTin", "rahas",
    "alIka", "uKA", "sAkzin", "Adi", "anta", "muKa", "jaGna", "meGa", "yUTa", "udaka", "nyAya",
    "vaMSa", "anuvaMSa", "viSa", "kAla", "ap", "AkASa", "digAdiH",
];

/// For 4.3.76.
pub const SHUNDIKA_ADI: &[&str] = &[
    "SuRqika", "kfkaRa", "sTaRqila", "udapAna", "upala", "tIrTa", "BUmi", "tfRa", "parRa",
];

/// For 4.3.92.
pub const SHANDIKA_ADI: &[&str] = &[
    "SaRqika",
    "sarvasena",
    "sarvakeSa",
    "Saka",
    "sawa",
    "raka",
    "SaNKa",
    "boDa",
];

/// For 4.3.93.
pub const SINDHU_ADI: &[&str] = &[
    "sinDu", "varRu", "ganDAra", "maDumat", "kamboja", "kaSmIra", "sAlva", "kizkinDA", "gadikA",
    "urasa", "darat",
];

/// For 4.3.93.
pub const TAKSHASHILA_ADI: &[&str] = &[
    "takzaSilA",
    "vatsodDaraRa",
    "kOmedura",
    "kaRqavAraRa",
    "grAmaRI",
    "sarAlaka",
    "kaMsa",
    "kinnara",
    "saMkucita",
    "siMhakozWa",
    "karRakozWa",
    "barbara",
    "avasAna",
];

/// For 4.3.131.
pub const RAIVATIKA_ADI: &[&str] = &[
    "rEvatika",
    "svApiSi",
    "kzEmavfdDi",
    "gOragrIvi",
    "Odameyi",
    "OdavAhi",
    "bEjavApi",
];

/// For 4.4.10.
pub const PARPA_ADI: &[&str] = &[
    "parpa", "aSva", "aSvatTa", "raTa", "jAla", "nyAsa", "vyAla", "pAda", "paYca", "padika",
];

/// For 4.3.118.
pub const KULALA_ADI: &[&str] = &[
    "kulAla",
    "varuqa",
    "caRqAla",
    "nizAda",
    "karmAra",
    "senA",
    "siraGra",
    "sendriya",
    "devarAja",
    "parizat",
    "vaDU",
    "ruru",
    "Druva",
    "rudra",
    "anaquH",
    "brahman",
    "kumBakAra",
    "SvapAka",
];

/// For 4.3.164.
pub const PLAKSHA_ADI: &[&str] = &[
    "plakza", "nyagroDa", "aSvatTa", "iNgudI", "Sigru", "kakarnDu", "vuhatI",
];

/// For 4.4.12.
pub const VETANA_ADI: &[&str] = &[
    "vetana",
    "vAha",
    "arDavAha",
    "DanurdaRqa",
    "jAla",
    "vesa",
    "upavesa",
    "prezana",
    "upasti",
    "suKa",
    "SayyA",
    "Sakti",
    "upanizad",
    "upaveza",
    "sraj",
    "pAda",
    "upasTAna",
];

/// For 4.4.19.
pub const AKSHADYUTA_ADI: &[&str] = &[
    "akzadyUta",
    "jAnuprahfta",
    "jaNGAprahfta",
    "pAdasvedana",
    "kaRwakamardana",
    "gatAgata",
    "yAtopayAta",
    "anugata",
];

/// 4.4.62 CatrAdiByo RaH (142)
pub const CHATRA_ADI: &[&str] = &[
    "Catra", "buBukzA", "SikzA", "puroha", "sTA", "curA", "upasTAna", "fzi", "karman", "viSvaDA",
    "tapas", "satya", "anfta", "SibikA",
];

/// 4.4.98 pratijanAdiByaH KaY (143)
pub const PRATIJANA_ADI: &[&str] = &[
    "pratijana",
    "idaMyuga",
    "saMyuga",
    "samayuga",
    "parayuga",
    "parakula",
    "parasyakula",
    "amuzyakula",
    "sarvajana",
    "viSvajana",
    "mahAjana",
    "paYcajana",
];

/// 4.4.102 kaTAdiByaz Wak (144)
pub const KATHA_ADI: &[&str] = &[
    "kaTA",
    "vikaTA",
    "viSvakaTA",
    "saMkaTA",
    "vitaRqA",
    "kuzwavid",
    "kuzWavid",
    "janavAda",
    "janevAda",
    "janovAda",
    "vftti",
    "saMgfha",
    "guRagaRa",
    "Ayurveda",
];

/// 4.4.103 guqAdiByaz WaY (145)
pub const GUDA_ADI: &[&str] = &[
    "guqa",
    "kulmAza",
    "saktu",
    "apUpa",
    "mAMsOdana",
    "ikzu",
    "veRu",
    "saNgrAma",
    "saMGAta",
    "saMkAma",
    "saMvAha",
    "pravAsa",
    "nivAsa",
    "upavAsa",
];

/// 5.1.2 u-gavAdiByo yat (146)
pub const GAVADI: &[&str] = &[
    "go", "havis", "akzara", "viza", "barhis", "azwakA", "svadA", "yuga", "meDA", "srac", "nABi",
    "naBa", "kUpa", "Kada", "dara", "asura", "aDvan", "aDvana", "kzara", "veda", "bIja", "dIsa",
    "dIpta",
];

/// 5.1.4 viBAzA havirapUpAdiByaH (147)
pub const APUPA_ADI: &[&str] = &[
    "apUpa",
    "taRqula",
    "aByuza",
    "aByUza",
    "aByoza",
    "avoza",
    "aByeza",
    "pfTuka",
    "odana",
    "sUpa",
    "pUpa",
    "kiRva",
    "pradIpa",
    "musala",
    "kawaka",
    "karRavezwaka",
    "irgala",
    "argala",
    "yUpa",
    "sTURA",
    "dIpa",
    "aSva",
    "patra",
];

/// 5.1.20 asamAse nizkAdiByaH (148)
pub const NISHKA_ADI: &[&str] = &["nizka", "paRa", "pAda", "mAza", "vAha", "droRa", "zazwi"];

/// 5.1.64 CedAdiByo nityam (151)
pub const CHEDA_ADI: &[&str] = &[
    "Ceda",
    "Beda",
    "droha",
    "doha",
    "narti",
    "narta",
    "karza",
    "tIrTa",
    "samprayoga",
    "viprayoga",
    "prayoga",
    "viprakarza",
    "prezaRa",
    "sampraSna",
    "vipraSna",
    "vikarza",
    "virAga",
    "viraNga",
];

/// 5.1.66 daRqAdiByaH (152)
pub const DANDA_ADI: &[&str] = &[
    "daRqa",
    "musala",
    "maDuparka",
    "kaSA",
    "arGa",
    "meGa",
    "meDA",
    "suvarRa",
    "udaka",
    "vaDa",
    "yuga",
    "guhA",
    "BAga",
    "iBa",
    "BaNga",
];

/// 5.1.122 pRTvAdiBya imanijvA (162)
pub const PRTHU_ADI: &[&str] = &[
    "pfTu", "mfdu", "mahat", "pawu", "tanu", "laGu", "bahu", "sADu", "veRu", "ASu", "bahula",
    "guru", "daRqa", "uru", "KaRqa", "caRqa", "bAla", "akiYcana", "hoqa", "pAka", "vatsa", "manda",
    "svAdu", "hrasva", "dIrGa", "priya", "vfza", "fju", "kzipra", "kzupra", "kzudra",
];

/// 5.2.36 tadasya saMjAtaM tArakAdiBya itac (172)
pub const TARAKA_ADI: &[&str] = &[
    "tArakA",
    "puzpa",
    "karRaka",
    "maYjarI",
    "fjIza",
    "kzaRa",
    "sUtra",
    "mUtra",
    "nizkramaRa",
    "purIza",
    "uccAra",
    "pracAra",
    "vicAra",
    "kuqmala",
    "kaRwaka",
    "musala",
    "mukula",
    "kusuma",
    "kutUhala",
    "stabaka",
    "stavaka",
    "kisalaya",
    "pallava",
    "KaRqa",
    "vega",
    "nidrA",
    "mudrA",
    "buBukzA",
    "DenuzyA",
    "pipAsA",
    "SradDA",
    "aBra",
    "pulaka",
    "aNgAraka",
    "varRaka",
    "droha",
    "doha",
    "suKa",
    "duHKa",
    "utkaRWA",
    "Bara",
    "vyADi",
    "varman",
    "vraRa",
    "gOrava",
    "SAstra",
    "taraNga",
    "tilaka",
    "candraka",
    "anDakAra",
    "garva",
    "kumura",
    "mukura",
    "harza",
    "utkarza",
    "raRa",
    "kuvalaya",
    "garGa",
    "kzuD",
    "sImanta",
    "jvara",
    "gara",
    "roga",
    "romAYca",
    "paRqA",
    "kujjala",
    "tfz",
    "koraka",
    "kallola",
    "sTapuwa",
    "Pala",
    "kaYcuka",
    "SfNgAra",
    "aNkura",
    "SEvala",
    "bakula",
    "Svamra",
    "ArAla",
    "kalaNka",
    "kardama",
    "kandala",
    "mUrcCA",
    "aNgAra",
    "hastaka",
    "pribimba",
    "viGnatantra",
    "pratyaya",
    "dIkzA",
    "garja",
];

/// 5.2.61 vimuktAdiByo 'R (173)
pub const VIMUKTA_ADI: &[&str] = &[
    "vimukta",
    "devAsura",
    "rakzosura",
    "upasad",
    "suvarRa",
    "parisAraka",
    "sadasat",
    "vasu",
    "marut",
    "patnIvat",
    "vasumat",
    "mahIyas",
    "satvat",
    "barhavat",
    "daSArRa",
    "daSArha",
    "vayas",
    "havirDAna",
    "patatrin",
    "mahitrI",
    "asyahatya",
    "somApUzan",
    "iqA",
    "agnAvizRU",
    "urvaSI",
    "vftrahan",
];

/// 5.2.62 gozadAdiByo vun (174)
pub const GOSHADA_ADI: &[&str] = &[
    "gozada",
    "gozad",
    "izetvA",
    "mAtariSvan",
    "devasyatvA",
    "devIrApaH",
    "kfzRa",
    "devIMDiyA",
    "devIMDiya",
    "rakzohaRa",
    "yuYjAna",
    "aYjana",
    "prasUta",
    "praBUta",
    "pratUrta",
    "kfSAnu",
    "kfSAku",
];

/// 5.2.64 AkarzAdiByaH kan (175)
pub const AKARSHA_ADI: &[&str] = &[
    "Akarza",
    "Akaza",
    "tsaru",
    "piSAca",
    "picaRqa",
    "aSani",
    "aSman",
    "nicaya",
    "caya",
    "vijaya",
    "jaya",
    "Acaya",
    "naya",
    "pAda",
    "dIpahrada",
    "hrAda",
    "hlAda",
    "gadgada",
    "Sakuni",
];

/// 5.2.95 rasAdiByaS ca (177)
pub const RASA_ADI: &[&str] = &[
    "rasa", "rUpa", "ganDa", "sparSa", "Sabda", "sneha", "guRAt", "ekAcaH",
];

// 5.2.97 siDmAdiByaS ca (178)
pub const SIDHMA_ADI: &[&str] = &[
    "siDma", "gaqu", "maRi", "nABi", "jIva", "nizpAva", "pAMsu", "saktu", "hanu", "mAMsa", "paraSu",
];

// 5.2.100 lomAdi-pAmAdi-picCAdiByaH SanelacaH (179)
pub const LOMA_ADI: &[&str] = &[
    "loman", "roman", "valgu", "baBrO", "hari", "kapi", "Suni", "taru",
];

// 5.2.100 lomAdi-pAmAdi-picCAdiByaH SanelacaH (180)
pub const PAMA_ADI: &[&str] = &[
    "pAman", "vAman", "heman", "Slezman", "kadru", "bali", "SrezWa", "palala", "sAman",
];

// 5.2.100 lomAdi-pAmAdi-picCAdiByaH SanelacaH (181)
pub const PICCHA_ADI: &[&str] = &[
    "picCa", "uras", "GruvakA", "kzuvakA", "varRa", "udaka", "paNka", "prajYA",
];

/// 5.2.117 tundAdiBya ilac ca (174)
pub const TUNDA_ADI: &[&str] = &["tunda", "udara", "picaRqa", "yava", "vrIhi"];

/// 5.3.101 SAKAdiByo yat (191)
pub const SHAKHA_ADI: &[&str] = &[
    "SAKA", "muKa", "jaGana", "SfNga", "meGa", "caraRa", "skanDa", "Siras", "uras", "agra",
    "Sarana",
];

/// 5.3.107 SarkarAdiByo 'R (192)
pub const SHARKARA_ADI: &[&str] = &[
    "SarkarA",
    "kapAlikA",
    "pizwika",
    "puRqarIka",
    "Satapatra",
    "goloman",
    "gopucCa",
    "narAcI ",
    "nakulA",
    "sikatA",
];

/// 5.3.108 aNgulyAdiByaz Wak (193)
pub const ANGULI_ADI: &[&str] = &[
    "aNguli", "Baruja", "baBru", "valgu", "maRqara", "maRqala", "Sazkula", "kapi", "udaSvit",
    "goRI", "uras", "SiKara", "kuliSa",
];

/// 5.3.116 dAmanyAditrigartazazWAc CaH (194)
pub const DAMANI_ADI: &[&str] = &[
    "dAmanI",
    "Olapi",
    "AkidantI",
    "kAkaranti",
    "kAkadanti",
    "Satruntapi",
    "sArvaseni",
    "bindu",
    "mOYjAyana",
    "ulaBa",
    "sAvitrIputra",
];

/// 5.3.117 parSvAdi-yODeyAdiByAmaRaYO (195)
pub const PARSHU_ADI: &[&str] = &[
    "parSu",
    "asura",
    "rakzas",
    "bAhlIka",
    "vayas",
    "marut",
    "daSArha",
    "piSAca",
    "viSAla",
    "aSani",
    "kArzApaRa",
    "satvat",
    "vasu",
    "parSvAdiH",
];

/// 5.3.117 parSvAdi-yODeyAdiByAmaRaYO (196)
pub const YAUDHEYA_ADI: &[&str] = &[
    "yODeya", "kOSeya", "krOSeya", "SOkreya", "SOBreya", "DArteya", "vArteya", "jAbAleya",
    "trigarta", "Barata", "uSInara",
];

/// 5.4.3 sTUlAdiByaH prakAravacane kan (197)
pub const STHULA_ADI: &[&str] = &[
    "sTUla",
    "aRu",
    "mAza",
    "izu",
    "kfzRa",
    "yava",
    "ikzu",
    "tila",
    "pAdya",
    "kAla",
    "avadAta",
    "gomUtra",
    "surA",
    "jIrRa",
    "patra",
    "mUla",
    "kumArIputra",
    "kumAra",
    "SvaSura",
    "maRi",
];

/// 5.4.3 uraH-praBftiByaH kap
pub const URAH_PRABHRTI: &[&str] = &[
    "uras", "sarpis", "upAdah", "pums", "anaquh", "payas", "nO", "lakzmI", "daDi", "maDu", "SAlI",
    "SAli",
];

/// 5.4.29 yAvAviByaH kan (252)
/// TODO: others
pub const YAVA_ADI: &[&str] = &[
    "yAva", "maRi", "asTi", "caRqa", "pIta", "stamBa", "ftu", "paSu", "aRu", "putra", "snAta",
    "SUnya", "dAna", "tanu", "jYAta",
];

/// 5.4.34 vinayAdiByaz Wak (253)
pub const VINAYA_ADI: &[&str] = &[
    "vinaya",
    "samaya",
    "upAya",
    "saNgati",
    "kaTaYcit",
    "akasmAt",
    "samayAcAra",
    "upacAra",
    "samAcAra",
    "vyavahAra",
    "sampradAna",
    "samutkarza",
    "samUha",
    "viSeza",
    "atyaya",
];

/// 6.1.203 vfzAdInAM ca (210)
/// TODO: SamaraRa
pub const VRSHA_ADI: &[&str] = &[
    "vfza", "jana", "jvara", "graha", "haya", "maya", "gaya", "tAya", "taya", "caya", "ama",
    "veda", "sUda", "aMSa", "guhA", "mantra", "SAnti", "kAma", "yAma", "ArA", "DArA", "kArA",
    "vaha", "kalpa", "pAda",
];
