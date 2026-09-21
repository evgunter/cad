//! R1's review probe for DECIDE-1 — two questions the unit's own
//! instrument does not answer, asked with the same harness.
//!
//! 1. **Is the zero `Invalid` count an artifact of the SCALE?** The
//!    census replays at ceiling + δ, the NARROWEST refusing box. The
//!    item's mechanism (a spurious negative lower bound from a product
//!    of one enclosure with itself) grows with width, so this row
//!    re-takes the count at wider scales up to the real study.
//! 2. **Does any `transform_rigid_*` row decide on these documents?**
//!    The PR asserts none does; the census's table keeps only BLOCKED
//!    predicates, so it cannot show that either way.
//!
//! Ignored evidence rows; nothing asserted.
#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::ProfileDoc;
use editor_core::analysis::{AnalysisPolicy, ParamBox, analyzed_box};
use geom_core::sym::report::ShapeOutcome;
use geom_core::{SymRules, Tol};

use crate::m10_8_arc_family_interval::replay;
use crate::m10_8_harness::{nominal_box, split};

fn count(label: &str, doc: &ProfileDoc, whole: bool, rules: SymRules, tol: Tol) {
    let analyzed = analyzed_box(doc, &AnalysisPolicy::default());
    let box_ = if whole {
        ParamBox::of(&analyzed)
    } else {
        nominal_box(&analyzed)
    };
    let (shapes, refusal, _) = replay(doc, &box_, rules, tol);
    let invalid = shapes
        .iter()
        .filter(|s| matches!(s.outcome, ShapeOutcome::Invalid))
        .count();
    let indet = shapes
        .iter()
        .filter(|s| matches!(s.outcome, ShapeOutcome::Indeterminate))
        .count();
    println!(
        "{label:<46} decisions {:>5}  INVALID {invalid:>4}  indeterminate {indet:>4}  refusal {}",
        shapes.len(),
        if refusal.is_some() { "YES" } else { "no" }
    );
    let rigid: Vec<_> = split(&shapes)
        .into_iter()
        .filter(|(p, _)| p.starts_with("transform_rigid"))
        .collect();
    println!("    transform_rigid rows decided here: {rigid:?}");
}

#[test]
#[ignore = "R1 review probe"]
fn r1_the_invalid_count_at_wider_scales() {
    let tol = Tol::witness();
    let rules = SymRules::shipped();
    for s in [0.2631_f64, 0.5, 1.0, 2.0] {
        let plate = crate::m10_7_plate::plate(5.0e-5 * s, 1.0e-5 * s, tol).0;
        count(&format!("plate / whole box / s={s}"), &plate, true, rules, tol);
    }
    for s in [0.8419_f64, 1.0, 2.0] {
        let ann = crate::m10_8_r1_probes_interval::annulus(s, tol).0;
        count(&format!("annulus / whole box / s={s}"), &ann, true, rules, tol);
    }
}
