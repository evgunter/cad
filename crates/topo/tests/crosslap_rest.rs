//! The mated cross-lap union — the REST-contact frontier wire
//! (#91 C1; re-armed at M4 PR 5 on the stage the blocker actually
//! lives at now).
//!
//! History: the original `demo_tripwires.rs` crosslap wire expected
//! M4 PR 5's Declare to glue the mate. PR 5 opened the
//! CLASSIFICATION half exactly as predicted — but the mate is a pure
//! REST contact (the half-depth notches interlock exactly; the two
//! interiors are DISJOINT), so the union then refuses typed at the
//! JOIN: every mate segment lies ON existing operand edges, no chord
//! has a facing partner — the M3 envelope's boundary-on-boundary
//! class (iii), a join-stage gap distinct from the declared-rung
//! opener (the corner-aligned table glues because its legs OVERLAP
//! into the top; here there is no overlap to seam). Same frontier as
//! `m3_pr6_tier3prime`'s declared corner-flush REST pin.
//!
//! THIS WIRE fires when the join-stage REST lane lands: if the
//! declared union ever BUILDS, the test fails with upgrade
//! instructions for the demos/tour crosslap stop.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::{flush_declarations, prism_z};
use topo::{BooleanError, BooleanResult, mass_properties, subtract, union, union_with};

const NOTCH_VOL: f64 = 0.5 * 0.5 * 0.25;
const BEAM_VOL: f64 = 4.0 * 0.5 * 0.5;

fn notched_beams() -> (topo::Body<f64>, topo::Body<f64>) {
    let beam_a = prism_z::<f64>(
        &[(0.0, 1.75), (4.0, 1.75), (4.0, 2.25), (0.0, 2.25)],
        0.0,
        0.5,
    );
    let cut_a = prism_z::<f64>(
        &[(1.75, 1.5), (2.25, 1.5), (2.25, 2.5), (1.75, 2.5)],
        0.25,
        0.75,
    );
    let BooleanResult::Body(a) = subtract(&beam_a.body, &cut_a.body).expect("notch A") else {
        panic!("notch A yields a body");
    };
    let beam_b = prism_z::<f64>(
        &[(1.75, 0.0), (2.25, 0.0), (2.25, 4.0), (1.75, 4.0)],
        0.0,
        0.5,
    );
    let cut_b = prism_z::<f64>(
        &[(1.5, 1.75), (2.5, 1.75), (2.5, 2.25), (1.5, 2.25)],
        -0.25,
        0.25,
    );
    let BooleanResult::Body(b) = subtract(&beam_b.body, &cut_b.body).expect("notch B") else {
        panic!("notch B yields a body");
    };
    for (label, notched) in [("A", &a), ("B", &b)] {
        assert_eq!(
            mass_properties(&notched.body).unwrap().volume,
            BEAM_VOL - NOTCH_VOL,
            "notched beam {label}: exact dyadic volume"
        );
    }
    (a.body, b.body)
}

/// The narrowing pin: UNDECLARED, the mate refuses at the coincidence
/// door (rung (b) — post-PR 5, value equality never classifies; the
/// pre-PR 5 refusal was the later JoinDesync).
#[test]
fn undeclared_crosslap_refuses_at_the_coincidence_door() {
    let (a, b) = notched_beams();
    match union(&a, &b) {
        Err(BooleanError::UndeclaredCoincidence { .. }) => {}
        other => panic!("expected UndeclaredCoincidence, got {other:?}"),
    }
}

/// THE WIRE: the DECLARED mate classifies (M4 PR 5) and then refuses
/// typed at the JOIN — the boundary-on-boundary REST frontier. Fires
/// when the join-stage REST lane lands.
#[test]
fn tripwire_declared_crosslap_rest_union() {
    let (a, b) = notched_beams();
    match union_with(&a, &b, &flush_declarations(&a, &b)) {
        Err(e) => {
            assert!(
                format!("{e:?}").contains("JoinDesync") || format!("{e:?}").contains("Join("),
                "declared-mate refusal moved off the documented join-stage \
                 class — update this wire AND the demos/tour crosslap \
                 narration: {e:?}"
            );
        }
        Ok(BooleanResult::Body(glued)) => {
            let vol = mass_properties(&glued.body).map(|m| m.volume);
            panic!(
                "TRIPWIRE FIRED (not a regression): the DECLARED mated \
                 cross-lap now unions (volume {vol:?}, expected {}). The \
                 join-stage REST lane landed — upgrade the demos/tour \
                 `crosslap` stop to ship the GLUED union (exact volume + \
                 watertight STL/STEP there), then retire this wire.",
                2.0 * (BEAM_VOL - NOTCH_VOL)
            );
        }
        Ok(BooleanResult::Empty) => panic!("overlapping mate returned Empty"),
    }
}
