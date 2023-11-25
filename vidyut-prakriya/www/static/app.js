/**
 * A simple demo interface for vidyut-prakriya.
 *
 *
 * Outline
 * =======
 *
 * Focus on `App`, which contains the main code. We use the Alpine framework,
 * which you can think of as a lightweight version of Vue.
 *
 *
 * Constraints
 * ===========
 * - This demo is served on GitHub pages. So, no databases -- everything should
 *   be done client side!
 * - This demo should use our wasm build's public API.
 * - Although this is a production site, the stakes are low -- do things the
 *   hacky way if that fixes the problem.
 */

import init, { BaseKrt, Vidyut, Gana, Lakara, Prayoga, Purusha, Vacana, DhatuPada, Sanadi, Linga, Vibhakti } from "/static/wasm/vidyut_prakriya.js";

// Krts that create ordinary nouns.
const NOMINAL_KRTS = [
    BaseKrt.GaY,
    BaseKrt.lyuw,
    BaseKrt.Rvul,
    BaseKrt.tfc,
    BaseKrt.kvip,
];

// Krts that are generally called *participles*.
const PARTICIPLE_KRTS = [
    BaseKrt.tavya,
    BaseKrt.anIyar,
    BaseKrt.yat,
    BaseKrt.Ryat,

    BaseKrt.Satf,
    BaseKrt.SAnac,

    BaseKrt.kta,
    BaseKrt.ktavatu,

    BaseKrt.kvasu,
    BaseKrt.kAnac,
];

// Krts that create avyayas.
const AVYAYA_KRTS = [
    BaseKrt.tumun,
    BaseKrt.ktvA,
    BaseKrt.Ramul,
];

// What to call these params in the URL.
const Params = {
    Dhatu: "dhatu",
    Tab: "tab",
    DhatuPada: "pada",
    Prayoga: "prayoga",
    Sanadi: "sanadi",
    ActivePada: "activePada",
    Upasarga: "upasarga",
}

// Turn the TSV file sutrapatha.tsv into a map.
function parseSutras(tsv) {
    let sutras = {};
    tsv.split(/\r?\n/).forEach(line => {
        const [id, text] = line.split(/\t/);
        sutras[id] = text;
    });
    return sutras;
}
const sutras = fetch("/static/data/sutrapatha.tsv").then(resp => resp.text()).then(text => parseSutras(text));

function setParam(url, key, value) {
    if (value) {
        url.searchParams.set(key, value);
    } else {
        url.searchParams.delete(key);
    }
}

function createKrdantasFrom(vidyut, dhatu, upasarga, sanadi, krtList) {
    let results = [];

    krtList.forEach((krt) => {
        let padas = [];
        const prakriyas = vidyut.deriveKrdantas(
            dhatu.code,
            krt,
            sanadi,
            upasarga,
        );
        prakriyas.forEach((p) => {
            padas.push({
                text: p.text,
                type: "krt",
                dhatu,
                krt,
                sanadi,
                upasarga,
            });
        });
        results.push({
            title: BaseKrt[krt],
            padas,
        });
    });
    return results;
}

function removeSlpSvaras(s) {
    return s.replaceAll(/[\^\\]/g, '');
}

// Parse a dhatupatha string into separate objects.
function parseDhatus(text) {
    let dhatus = [];
    text.split(/\r?\n/).forEach((line) => {
        const [code, upadesha, artha] = line.split(/\t/);
        if (!!code && code !== 'code') {
            dhatus.push({
                code,
                upadesha,
                upadeshaQuery: removeSlpSvaras(upadesha),
                artha
            });
        }
    });
    return dhatus;
}

// Load and initialize the Vidyut API.
async function loadVidyut() {
    await init();

    const resp = await fetch("/static/data/dhatupatha.tsv");
    const text = await resp.text();

    return {
        // Vidyut needs its own copy of the dhatupatha.
        vidyut: Vidyut.init(text),
        // For JS use
        dhatus: parseDhatus(text),
    }
}

const App = () => ({
    activeTab: 'dhatu',

    // All dhatus.
    dhatus: [],
    // The selected dhatu.
    activeDhatu: null,
    // The selected pada for the selected dhatu.
    activePada: null,
    // The prakriya for the selected pada.
    prakriya: null,

    // UI options
    // ----------
    // The desired prayoga.
    prayoga: null,
    // The desired upasarga.
    upasarga: null,
    // The desired sanAdi pratyaya.
    sanadi: null,
    // A filter to apply to the dhatu list.
    dhatuFilter: null,

    // Transliteration script (devanagari, iast, telugu, etc.)
    script: 'devanagari',

    async init() {
        const data = await loadVidyut();
        console.log("init");
        this.vidyut = data.vidyut;
        this.dhatus = data.dhatus;

        // TODO: set state earlier. But, our current implemenation needs to
        // wait for the dhatus to load so that we can set activeDhatu.
        this.readUrlState();

        // Save important properties to the URL when they change.
        this.$watch('activeDhatu', (value) => {
            this.updateUrlState();
        });
        this.$watch('tab', (value) => {
            this.updateUrlState();
        });
        this.$watch('sanadi', (value) => {
            this.updateUrlState();
        });
        this.$watch('prayoga', (value) => {
            this.updateUrlState();
        });
        this.$watch('upasarga', (value) => {
            this.updateUrlState();
        });
        this.$watch('activePada', (value) => {
            this.updateUrlState();
        });
    },

    // Mutators

    // Load the application state from the URL, if applicable.
    readUrlState() {
        const params = new URLSearchParams(window.location.search);
        const dhatuCode = params.get(Params.Dhatu);
        const tab = params.get(Params.Tab);
        const prayoga = params.get(Params.Prayoga);
        const upasarga = params.get(Params.Upasarga);
        const sanadi = params.get(Params.Sanadi);
        const activePada = params.get(Params.ActivePada);

        console.log(`realUrlState, prayoga=${prayoga}, upasarga=${upasarga}, sanadi=${sanadi}`);
        if (tab) {
            this.setTab(tab);
        }
        if (prayoga) {
            this.prayoga = parseInt(prayoga);
        }
        if (upasarga) {
            this.upasarga = upasarga;
        }
        if (sanadi) {
            this.sanadi = parseInt(sanadi);
        }
        if (dhatuCode) {
            this.setActiveDhatu(dhatuCode);
        }
        if (activePada) {
            this.setActivePada(JSON.parse(activePada));
        }
    },

    // Encode the current application state in the URL so that it can be
    // referenced later.
    updateUrlState() {
        const url = new URL(window.location.href);
        let dhatuCode = null;
        if (this.activeDhatu) {
            dhatuCode = this.activeDhatu.code;
        }
        setParam(url, Params.Dhatu, dhatuCode);
        setParam(url, Params.Tab, this.activeTab);
        setParam(url, Params.Prayoga, this.prayoga);
        setParam(url, Params.Sanadi, this.sanadi);
        setParam(url, Params.Upasarga, this.upasarga);
        if (this.activePada) {
            setParam(url, Params.ActivePada, JSON.stringify(this.activePada));
        } else {
            setParam(url, Params.ActivePada, null);
        }

        console.log("updateUrlState to: ", url.href);

        history.replaceState(null, document.title, url.toString());
    },

    // Set the active dhatu (and show its forms)
    setActiveDhatu(s) {
        this.activeDhatu = this.dhatus.find(d => d.code === s);
        // Scroll position might be off if the user has scrolled far down the dhatu list.
        window.scrollTo({ top: 0 });
    },

    // Set the active pada (and show its prakriya)
    setActivePada(p) {
        this.activePada = p;
        this.prakriya = this.createPrakriya();
        window.scrollTo({ top: 0 });
    },

    // Create the active pada (and show all forms for the dhatu)
    clearActivePada() {
        this.activePada = null;
        this.prakriya = null;
    },

    // Clear the active dhatu (and show the full dhatu list).
    clearActiveDhatu() {
        // Breaks if we clear `activeDhatu` last -- not sure why. So, clear it first.
        this.activeDhatu = null;
        this.tinantas = null;
        this.sanadi = null;
        this.prayoga = null;
        this.clearActivePada();
    },

    // Set the app's active tab.
    setTab(s) {
        // Reset the prakriya so that we don't display a krt pratyaya for tin, etc.
        // The proper fix is to have separate prakriyas for each tab.
        this.clearActivePada();
        this.activeTab = s;
    },

    // Computed properties

    tab(s) {
        if (s === this.activeTab) {
            return "font-bold p-2 bg-sky-100 rounded text-sky-800";
        } else {
            return "";
        }
    },

    /** A filtered list of dhatus according to a user query. */
    filteredDhatus() {
        if (this.dhatuFilter !== null) {
            let filter = Sanscript.t(this.dhatuFilter, 'devanagari', 'slp1');
            let hkFilter = Sanscript.t(this.dhatuFilter, 'hk', 'slp1');
            return this.dhatus.filter(d =>
                d.code.includes(filter)
                || d.upadeshaQuery.includes(filter)
                || d.artha.includes(filter)
                || d.upadeshaQuery.includes(hkFilter)
                || d.artha.includes(hkFilter)
            );
        } else {
            return this.dhatus;
        }
    },

    createPrakriya() {
        if (!this.activePada) {
            return null;
        }

        const pada = this.activePada;
        let allPrakriyas = [];
        if (pada.type === "tin") {
            allPrakriyas = this.vidyut.deriveTinantas(
                pada.dhatu.code,
                pada.lakara,
                pada.prayoga,
                pada.purusha,
                pada.vacana,
                null,
                pada.sanadi,
                pada.upasarga,
            );
        } else if (pada.type === "krt") {
            allPrakriyas = this.vidyut.deriveKrdantas(
                pada.dhatu.code,
                pada.krt,
                pada.sanadi,
                pada.upasarga,
            );
        }

        return allPrakriyas.find((p) => p.text == pada.text);
    },

    // Render the given SLP1 text in Devanagari.
    deva(s) {
        return Sanscript.t(s, 'slp1', this.script);
    },

    // Render the given SLP1 text in Devanagari without svara marks.
    devaNoSvara(s) {
        return Sanscript.t(removeSlpSvaras(s), 'slp1', this.script);
    },

    async sutraText(rule) {
        const text = (await sutras)[rule];
        return text ? this.deva(text) : '';
    },

    entryString(entries) {
        let str = entries.map(x => x.text).join(', ');
        return this.deva(str);
    },

    /// Create all tinantas allowed by the given `args`.
    createParadigm(args) {
        const { dhatu, lakara, prayoga, pada, sanadi, upasarga } = args;

        let purushas = Object.values(Purusha).filter(Number.isInteger);
        let vacanas = Object.values(Vacana).filter(Number.isInteger);

        let paradigm = [];
        for (const purusha in purushas) {
            for (const vacana in vacanas) {
                let prakriyas = this.vidyut.deriveTinantas(
                    dhatu.code,
                    lakara,
                    prayoga,
                    purusha,
                    vacana,
                    pada,
                    sanadi,
                    upasarga,
                );

                let pvPadas = [];
                let seen = new Set();
                prakriyas.forEach((p) => {
                    if (seen.has(p.text)) {
                        return;
                    }
                    seen.add(p.text);

                    pvPadas.push({
                        text: p.text,
                        type: "tin",
                        dhatu,
                        lakara,
                        prayoga,
                        purusha,
                        vacana,
                        pada,
                        sanadi,
                        upasarga,
                    });
                });

                if (pvPadas.length === 0) {
                    return [];
                }

                paradigm.push(pvPadas);
            }
        }

        return paradigm;
    },

    // Get a nice human-readable name for the given lakara.
    getLakaraTitle(value) {
        const mapping = {
            "Lat": "law",
            "Lit": "liw",
            "Lut": "luw",
            "Lrt": "lfw",
            "Let": "lew",
            "Lot": "low",
            "Lan": "laN",
            "VidhiLin": "viDi-liN",
            "AshirLin": "ASIr-liN",
            "Lun": "luN",
            "Lrn": "lfN",
        };
        const text = mapping[Lakara[value]];
        return this.deva(text);
    },

    createKrdantas() {
        if (this.activeDhatu === null) {
            return [];
        }

        const dhatu = this.activeDhatu;
        const upasarga = this.upasarga;
        const sanadi = this.sanadi;
        return [
            createKrdantasFrom(this.vidyut, dhatu, upasarga, sanadi, NOMINAL_KRTS),
            createKrdantasFrom(this.vidyut, dhatu, upasarga, sanadi, PARTICIPLE_KRTS),
            createKrdantasFrom(this.vidyut, dhatu, upasarga, sanadi, AVYAYA_KRTS),
        ];
    },

    createTinantas() {
        if (this.activeDhatu === null) {
            return [];
        }

        const dhatu = this.activeDhatu;
        const lakaras = Object.values(Lakara).filter(Number.isInteger);
        const tinPadas = Object.values(DhatuPada).filter(Number.isInteger);
        const prayoga = this.prayoga !== null ? this.prayoga : Prayoga.Kartari;
        const sanadi = this.sanadi || null;;
        const upasarga = this.upasarga || null;;

        console.log("createTinantas", prayoga, this.sanadi, upasarga);
        let results = [];
        for (const lakara in lakaras) {
            let laResults = {
                title: this.getLakaraTitle(lakara),
            };

            for (const tinPada in tinPadas) {
                const padaKey = DhatuPada[tinPada];
                const paradigm = this.createParadigm({
                    dhatu,
                    lakara,
                    prayoga,
                    pada: tinPada,
                    sanadi,
                    upasarga,
                });

                if (paradigm.length !== 0) {
                    laResults[padaKey] = paradigm;
                }
            }
            results.push(laResults);
        }

        return results;
    },
});

window.Lakara = Lakara;
window.Prayoga = Prayoga;
window.Sanadi = Sanadi;

// Initialize the app.
window.addEventListener('alpine:init', () => {
    Alpine.data("app", App)
});
