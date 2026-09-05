//! What the verdict bracket's escalation channel does NOT carry, pinned
//! by name so the gap is guarded rather than asserted away.
//!
//! `enters_material` asks the funnel for its lever arm, receives a
//! DEFINITE `Zero`, and then mints an `Indeterminate` of its own for
//! the collapsed arm. The funnel never classified that escalation, so
//! the frame holds the definite verdict and an EMPTY escalation log
//! while the caller receives an escalation. Seven sibling sites share
//! the shape (`dihedral`, `pcurve_cache`, `certify`, `edge_nurbs`,
//! `ssi::march`); the unit that routes the family through the funnel is
//! `work/props/escalation-channel-misses-op-minted-indeterminates.md`,
//! and this row goes red the day it lands — which is the point.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_brep::{OutwardNormal, enters_material};
use geom_core::k_stats::Bracket;
use geom_core::{Band, Sign, Tol, Vec3};

#[test]
fn an_indeterminate_minted_after_a_definite_verdict_is_not_on_the_escalation_log() {
    let band = Band::linear(Tol::witness()).unwrap();
    let bracket = Bracket::open();
    let out = enters_material(
        Vec3::new(1.0f64, 0.0, 0.0),
        OutwardNormal::from_chart(Vec3::new(0.0f64, 0.0, 1.0), true),
        // A collapsed lever arm: the funnel decides Zero DEFINITELY.
        0.0f64,
        band,
    );
    let recorded = bracket.finish();
    let escalated = out.expect_err("the predicate escalates to its caller");
    assert_eq!(escalated.predicate, Some("enters_material_arm"));
    assert_eq!(
        recorded
            .verdicts
            .iter()
            .map(|v| (v.predicate, v.sign))
            .collect::<Vec<_>>(),
        [("enters_material_arm", Sign::Zero)],
        "the frame holds the funnel's definite verdict"
    );
    assert!(
        recorded.escalations.is_empty(),
        "an op-minted Indeterminate is not on the log; a consumer reaches it only through \
         the op's error enum: {:?}",
        recorded.escalations
    );
}
