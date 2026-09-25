//! DECIDE-5 review probes (reviewer lane, evidence-only): the value
//! channel of parameter-bulge and literal-bulge documents of BOTH signs,
//! at `f64`, `Dual64` and `Sym<Interval>` over the whole analyzed box
//! under the guided lift, read as a deep digest plus every circle
//! carrier's `param_end` bits. Run at the head and with the span spelled
//! `span_magnitude` again; the lines must be identical.
#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use editor_core::analysis::{AnalysisPolicy, ParamBox, analyzed_box};
use editor_core::{
    CancelToken, EvalOptions, Evaluation, NodeResult, ProfileDoc, ProfileLift, ValuePayload,
    evaluate,
};
use geom::Curve3;
use geom_core::sym::report::name_param;
use geom_core::{Bounds, Dual64, Interval, Sym, SymBudget, SymRules, Tol};

fn spans<T: Bounds + geom_core::Decide>(ev: &Evaluation<T>) -> Vec<String> {
    let mut out = Vec::new();
    for &id in &ev.order {
        if let Some(NodeResult::Ok(v)) = ev.result(id) {
            if let ValuePayload::Body(b) = &v.payload {
                for (_k, c) in b.curves() {
                    if let Some(ec) = c.certified() {
                        if matches!(ec.carrier(), Curve3::Circle { .. }) {
                            let (t0, t1) = ec.params();
                            out.push(format!(
                                "{:016x}:{:016x}/{:016x}:{:016x}",
                                t0.lo().to_bits(),
                                t0.hi().to_bits(),
                                t1.lo().to_bits(),
                                t1.hi().to_bits()
                            ));
                        }
                    }
                }
            }
        }
    }
    out.sort();
    out
}

fn fnv(xs: &[String]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for x in xs {
        for b in x.as_bytes() {
            h ^= u64::from(*b);
            h = h.wrapping_mul(0x0000_0100_0000_01b3);
        }
    }
    h
}

fn line(name: &str, doc: &ProfileDoc, tol: Tol) {
    let ev: Evaluation<f64> =
        evaluate(doc, None, &CancelToken::new(), &EvalOptions::default(), tol);
    let s = spans(&ev);
    let f = (crate::r1_dual_probes::eval_deep(&ev), s.len(), fnv(&s));
    let evd: Evaluation<Dual64> =
        evaluate(doc, None, &CancelToken::new(), &EvalOptions::default(), tol);
    let sd = spans(&evd);
    let d = (crate::r1_dual_probes::eval_deep(&evd), sd.len(), fnv(&sd));
    let analyzed = analyzed_box(doc, &AnalysisPolicy::default());
    for (which, pbox) in [
        ("whole", ParamBox::of(&analyzed)),
        ("nominal", crate::m10_8_harness::nominal_box(&analyzed)),
    ] {
        for n in pbox.axes().keys() {
            name_param(&n.0);
        }
        let opts = EvalOptions {
            param_box: Some(Arc::new(pbox)),
            profile_lift: ProfileLift::Guided,
            ..EvalOptions::default()
        };
        let budget = SymBudget {
            max_terms: editor_core::drive::DEFAULT_SYM_MAX_TERMS,
            max_degree: editor_core::drive::DEFAULT_SYM_MAX_DEGREE,
        };
        let ((sdeep, ss, fails), counts) =
            geom_core::sym::with_session_rules(budget, SymRules::shipped(), || {
                let ev: Evaluation<Sym<Interval>> =
                    evaluate(doc, None, &CancelToken::new(), &opts, tol);
                let fails = ev
                    .order
                    .iter()
                    .find_map(|id| match ev.result(*id) {
                        Some(NodeResult::Failed(e)) => Some(format!("{}", e.kind)),
                        _ => None,
                    })
                    .unwrap_or_default();
                let s = spans(&ev);
                (crate::r1_dual_probes::eval_deep(&ev), s, fails)
            });
        println!(
            "REVIEW-E2E {name}: f64 deep {:016x} arcs {} spans {:016x} | dual deep {:016x} arcs {} spans {:016x} | sym<iv> {which} deep {sdeep:016x} arcs {} spans {:016x} first-fail [{}] | receipt {counts:?}",
            f.0,
            f.1,
            f.2,
            d.0,
            d.1,
            d.2,
            ss.len(),
            fnv(&ss),
            &fails[..fails.len().min(120)]
        );
        for s in ss.iter().take(3) {
            println!("REVIEW-E2E {name}:   sym<iv> {which} span {s}");
        }
    }
}

#[test]
#[ignore = "review evidence: value channel at both bulge signs"]
fn review_decide_5_value_channel_of_both_signs() {
    let tol = Tol::witness();
    for param in [true, false] {
        for b in [0.4, -0.4, 0.5, -0.5] {
            let (doc, _, _) = crate::m10_10_r2_probes_interval::d_tab_at(1.0, param, b, tol);
            let name = format!("d_tab {} b={b}", if param { "PARAM" } else { "literal" });
            line(&name, &doc, tol);
        }
    }
    line(
        "r2_link",
        &crate::m10_9_r2_probes_interval::link(1.0, tol).0,
        tol,
    );
    line(
        "plate",
        &crate::m10_7_plate::plate(5.0e-5, 1.0e-5, tol).0,
        tol,
    );
}
