//! Reviewer probe for the signed-radius unit (PR 7686, round 1): the
//! cut plate that now evaluates over its whole study box at
//! `Sym<Interval>`, read against members of that box.

mod tests {
    use std::collections::BTreeMap;

    use demo_tour::plate::{RADIUS_SIGMA, SPACING_HALF_WIDTH, WEB_BOUND, cut_plate};
    use pncad::analysis::{AnalysisPolicy, BoxAxis, ParamBox, analyzed_box};
    use pncad::document::{CancelToken, EvalOptions, Evaluation, ValuePayload, evaluate};
    use pncad::geom_core::{Bounds, Interval, Sym, SymBudget, SymRules, Tol};

    /// **The web the cut plate measures over its whole study box holds
    /// every member's.** The whole-box evaluation's asserted measure (the
    /// distance between the two bore walls less the radii, read off the
    /// CUT body) is an enclosure; each member of the box, evaluated on
    /// its own at `Interval`, measures a web inside it. A cut plate built
    /// with a hole in the wrong place, or of the wrong radius, measures
    /// outside.
    #[test]
    fn the_cut_plates_whole_box_web_holds_its_members() {
        let tol = Tol::witness();
        let p = cut_plate(SPACING_HALF_WIDTH, RADIUS_SIGMA, WEB_BOUND, tol);
        let analyzed = analyzed_box(&p.doc, &AnalysisPolicy::default());
        let whole = ParamBox::of(&analyzed);
        let budget = SymBudget {
            max_terms: 4096,
            max_degree: 128,
        };
        let web = |verdict: &ValuePayload<Sym<Interval>>| match verdict {
            ValuePayload::Assertion(a) => format!("{:?}", a.clone().map(|x| (x.lo(), x.hi()))),
            other => format!("not an assertion: {}", other.kind_name()),
        };
        let run = |b: ParamBox| {
            let opts = EvalOptions {
                param_box: Some(std::sync::Arc::new(b)),
                ..EvalOptions::default()
            };
            pncad::geom_core::sym::with_session_rules(budget, SymRules::shipped(), || {
                let ev: Evaluation<Sym<Interval>> =
                    evaluate(&p.doc, None, &CancelToken::new(), &opts, tol);
                match ev.value(p.assertion) {
                    Some(v) => web(&v.payload),
                    None => format!("no value: {:?}", ev.result(p.assertion).map(|_| "a result")),
                }
            })
            .0
        };
        println!("WHOLE: {}", run(whole.clone()));
        for s in [
            [0.0, 0.0, 0.0],
            [1.0, 1.0, 1.0],
            [0.0, 1.0, 1.0],
            [1.0, 0.0, 0.0],
            [0.5, 0.25, 0.75],
        ] {
            let axes: BTreeMap<_, _> = whole
                .axes()
                .iter()
                .zip(s.iter().cycle())
                .map(|((id, axis), s)| {
                    let axis = match *axis {
                        BoxAxis::Fixed => BoxAxis::Fixed,
                        BoxAxis::Varying { lo, hi } => {
                            let v = lo + s * (hi - lo);
                            BoxAxis::Varying {
                                lo: v,
                                hi: v + 1e-13 * (hi - lo),
                            }
                        }
                    };
                    (*id, axis)
                })
                .collect();
            println!("MEMBER {s:?}: {}", run(ParamBox::from_axes(axes)));
        }
    }
}
