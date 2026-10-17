//! PROBE (design/5225-frame-a): the pad's per-predicate split over the
//! measured whole box, printed for comparison across PROBE_FRAME=old/new.
use editor_core::analysis::{AnalysisPolicy, ParamBox, analyzed_box};
use geom_core::{SymRules, Tol};

#[test]
#[ignore = "probe"]
fn probe_pad_split() {
    let tol = Tol::witness();
    let eps = tol.eps();
    for (name, scale) in [("pad", 2.083e3), ("bracket", 0.0)] {
        let doc = if name == "pad" {
            crate::m10_8_r2_probes_interval::pad(scale * eps, tol).0
        } else {
            continue;
        };
        let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
        let (shapes, refusal, counts) = crate::m10_8_arc_family_interval::replay_retried(
            &doc,
            &ParamBox::of(&analyzed),
            SymRules::shipped(),
            geom_core::SymRetry::none(),
            tol,
        );
        println!("{name}: {counts:?} {refusal:?}");
        for (p, row) in crate::m10_8_harness::split(&shapes) {
            println!("  {p} {row:?}");
        }
    }
}
