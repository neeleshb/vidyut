use crate::args::{Artha, BaseKrt, Krt, KrtArtha};
use crate::it_samjna;
use crate::operators as op;
use crate::prakriya::{Prakriya, Rule};
use crate::tag::Tag as T;
use crate::term::Term;

impl Krt {
    /// Converts this krt-pratyaya to an appropriate `Term`.
    pub fn to_term(self) -> Term {
        let mut krt = Term::make_upadesha(self.as_str());
        krt.add_tags(&[T::Pratyaya, T::Krt]);

        if let Krt::Base(b) = self {
            use BaseKrt as K;

            // Any rule that adds a krtya-pratyaya also includes the `krtya` samjna by adhikara
            // from 3.1.95. Other samjnas, such as `Nistha`, are added in separate rules and are
            // thus modeled separately.
            if matches!(
                b,
                K::tavyat | K::tavya | K::anIyar | K::yat | K::kyap | K::Ryat
            ) {
                krt.add_tag(T::Krtya);
            }
        }

        if let Krt::Unadi(_) = self {
            krt.add_tag(T::Unadi);
        }

        krt
    }
}

/// Wrapper for `Prakriya` with the following features:
///
/// - remembers which `krt` pratyaya the caller wishes to add, which simplifies the calling API.
/// - records whether a `krt` pratyaya has been added or not, which simplifies the control flow for
///   optional rules.
pub struct KrtPrakriya<'a> {
    /// The underlying prakriya.
    pub p: &'a mut Prakriya,
    /// The first index of the dhatu.
    pub i_dhatu: usize,
    /// The krt-pratyaya that the caller wishes to add.
    pub krt: Krt,
    pub rule_artha: Option<KrtArtha>,
    pub had_match: bool,
    pub has_krt: bool,
}

impl<'a> KrtPrakriya<'a> {
    /// Creates a new `KrtPrakriya` struct.
    pub fn new(p: &'a mut Prakriya, krt: impl Into<Krt>) -> Self {
        let i_dhatu = p.find_first_where(|t| t.is_dhatu()).unwrap_or(0);
        KrtPrakriya {
            p,
            i_dhatu,
            krt: krt.into(),
            rule_artha: None,
            had_match: false,
            has_krt: false,
        }
    }

    /// Returns a reference to the underlying dhatu for this prakriya.
    pub fn dhatu(&self) -> &Term {
        self.p.get(self.i_dhatu).expect("present")
    }

    /// Returns a reference to the last dhatu term for this prakriya.
    pub fn dhatu_end(&self) -> &Term {
        let i = self.p.find_last_where(|t| t.is_dhatu()).expect("present");
        self.p.get(i).expect("present")
    }

    /// Returns a reference to the underlying upapada, if present.
    pub fn upapada(&self) -> Option<&Term> {
        if self.i_dhatu > 0 {
            self.p.get(self.i_dhatu - 1)
        } else {
            None
        }
    }

    /// Returns whether the term before the dhatu has the given upapada.
    pub fn has_upapada(&self, upadesha: &str) -> bool {
        self.i_dhatu > 0 && self.p.has(self.i_dhatu - 1, |t| t.has_u(upadesha))
    }

    /// Returns whether the term before the dhatu has one of the given upapada values.
    pub fn has_upapada_in(&self, upadeshas: &[&str]) -> bool {
        self.i_dhatu > 0 && self.p.has(self.i_dhatu - 1, |t| t.has_u_in(upadeshas))
    }

    pub fn expects_krt(&self, krt: impl Into<Krt>) -> bool {
        self.krt == krt.into()
    }

    /// Runs the rules in `closure` under the meaning condition defined in `artha`.
    ///
    /// Calls to `with_context` can be nested.
    pub fn with_context(&mut self, rule_artha: KrtArtha, closure: impl Fn(&mut Self)) {
        // If the prakriya has a specific meaning condition, try these rules only if there's an
        // artha match.
        //
        // Otherwise, allow any rule.
        if let Some(Artha::Krt(prakriya_artha)) = self.p.artha() {
            if prakriya_artha != rule_artha {
                return;
            }
        }

        // Push the `closure` context.
        let old_artha = self.rule_artha;
        let old_match = self.had_match;
        self.rule_artha = Some(rule_artha);
        self.had_match = false;

        if !self.has_krt {
            closure(self);
        }

        // Pop the `closure` context.
        self.rule_artha = old_artha;
        self.had_match = old_match;
    }

    pub fn has_upasarga_dhatu(&self, i_dhatu: usize, upa: &str, dhatu: &str) -> bool {
        i_dhatu > 0
            && self.p.has(i_dhatu - 1, |t| t.has_u(upa))
            && self.p.has(i_dhatu, |t| t.has_u(dhatu))
    }

    pub fn has_prefixes(&self, values: &[&str; 2]) -> bool {
        match self.p.find_last_where(|t| !t.is_dhatu()) {
            Some(i) => {
                i > 0
                    && self.p.has(i - 1, |t| t.has_text(values[0]))
                    && self.p.has(i, |t| t.has_text(values[1]))
            }
            None => false,
        }
    }

    /// If there's a match, adds the given `krt` pratyaya.
    ///
    /// This method does nothing if a krt pratyaya has already been added.
    pub fn try_add(&mut self, rule: impl Into<Rule>, krt: impl Into<Krt>) -> bool {
        self.try_add_with(rule, krt, |_| {})
    }

    /// If there's a match, replace the `lakAra` of the dhatu.
    ///
    /// This method does nothing if a krt pratyaya has already been added.
    pub fn try_replace_lakara(
        &mut self,
        rule: impl Into<Rule>,
        i_lakara: usize,
        krt: impl Into<Krt>,
    ) -> bool {
        let krt = krt.into();
        self.had_match = true;
        if self.krt == krt && !self.has_krt {
            op::adesha(rule, self.p, i_lakara, krt.as_str());
            self.has_krt = true;
            true
        } else {
            false
        }
    }

    pub fn do_nipatana(&mut self, rule: impl Into<Rule>, sub: &str) {
        self.p.run(rule, op::nipatana(sub));
        self.had_match = true;
        self.has_krt = true
    }

    pub fn optional_do_nipatana(&mut self, rule: impl Into<Rule>, sub: &str) -> bool {
        self.had_match = true;
        if self.p.run_optional(rule, op::nipatana(sub)) {
            self.has_krt = true;
            true
        } else {
            false
        }
    }

    /// If there's a match, adds the given `krt` pratyaya then runs `func`.
    ///
    /// This method does nothing if a krt pratyaya has already been added.
    pub fn try_add_with(
        &mut self,
        rule: impl Into<Rule>,
        krt: impl Into<Krt>,
        func: impl Fn(&mut Prakriya),
    ) -> bool {
        let rule = rule.into();
        let krt = krt.into();

        self.had_match = true;
        if self.krt == krt && !self.has_krt {
            // Insert term with it-samjna-prakarana.
            self.p.run(rule, |p| {
                p.push(krt.to_term());
                func(p);
            });
            let i_last = self.p.terms().len() - 1;
            it_samjna::run(self.p, i_last).expect("should never fail");

            // update bookkeeping.
            if let Some(a) = self.rule_artha {
                self.p.set_artha(Artha::Krt(a));
            }
            self.has_krt = true;
            true
        } else {
            false
        }
    }

    /// If there's a match, optionally adds the given `krt` pratyaya then runs `func`.
    ///
    /// This method does nothing if a krt pratyaya has already been added.
    pub fn optional_try_add_with(
        &mut self,
        rule: impl Into<Rule> + Copy,
        krt: impl Into<Krt>,
        func: impl Fn(&mut Prakriya),
    ) -> bool {
        let rule = rule.into();
        let krt = krt.into();

        if krt == self.krt && !self.has_krt {
            // TODO: resolve inconsistency with TaddhitaPratyaya::optional_try_add_with.
            if self.p.is_allowed(rule) {
                self.try_add_with(rule, krt, func);
                return true;
            } else {
                self.p.decline(rule);
            }
        }
        false
    }

    /// If there's a match, optionally adds the given `krt` pratyaya.
    ///
    /// This method does nothing if a krt pratyaya has already been added.
    pub fn optional_try_add(&mut self, rule: impl Into<Rule> + Copy, krt: BaseKrt) -> bool {
        self.optional_try_add_with(rule, krt, |_| {})
    }

    /// Like `optional` but indicates a specific choice of artha. Not sure how to use this yet, but
    /// at some point we should model specific semantic choices as different from an ordinary
    /// option.
    pub fn try_artha_add(&mut self, rule: impl Into<Rule> + Copy, krt: BaseKrt) -> bool {
        self.optional_try_add(rule, krt)
    }

    pub fn try_artha_add_with(
        &mut self,
        rule: impl Into<Rule> + Copy,
        krt: BaseKrt,
        func: impl Fn(&mut Prakriya),
    ) -> bool {
        self.optional_try_add_with(rule, krt, func)
    }
}