//! **The body-side coherence examination is QUIET on this crate's
//! corpus** (issue 868, the relocation of `walk`'s three detectors).
//!
//! The conditions `topo::examine_chart_coherence` states used to be
//! three `debug_assert!`s inside the boundary walk. Deleting them
//! leaves one question open that no argument can close: does the
//! relocated condition fire on ordinary bodies? A report that fires
//! everywhere is not a report.
//!
//! The corpus is this crate's own body tour — the same bodies the
//! byte-stability instrument hashes, plus the three shape witnesses
//! and the two wedges — because these are the bodies whose meshes are
//! pinned elsewhere in this suite: a finding on one of them would be a
//! finding about a body we already assert a correct mesh for. Every
//! one reports ZERO findings and nothing unexamined.
//!
//! **What this row cannot say**, in three directions.
//!
//! 1. It is a statement about bodies this workspace MINTS, not about
//!    bodies it could receive. The input class the three conditions
//!    are actually about arrives through import at an adoption
//!    tolerance looser than ε, and no test in this repo examines the
//!    wild or FreeCAD corpora — the same blind spot the deleted
//!    detectors disclosed and this row inherits unchanged. The
//!    `demos/tour` corpus is a separate cargo project and is not
//!    reachable from a workspace suite either.
//! 2. **Every body here goes through the ISO WALK, and the
//!    examination has no shape door.** It reads faces the walk never
//!    walks, and `zero findings` says nothing about those. The
//!    TRIMMED lane is the live case: a face whose outer loop carries a
//!    conic or spline trim carrier is not examined at all, and each
//!    such face's loops land in `unexamined` — measured at exactly two
//!    per body on the tilted cylinder halves of
//!    `mesh8r2_probes::r2r_the_examination_on_bodies_outside_the_units
//!    _corpus`, which mesh watertight and report nothing either way.
//!    A corpus row asserting `unexamined.is_empty()` over trimmed
//!    bodies would be asserting the opposite of the truth.
//! 3. On a face the walk never walks, the classification the
//!    conditions are read through is a nearest fit rather than the
//!    face's own premise — so a finding there measures a premise that
//!    face never had. Where the carrier is conic or spline that is
//!    visible in `unexamined`; where it is a line or a circle that
//!    happens not to be iso, it is not.
//!
//! It rides the run's ambient ε, so the three-ε matrix asks the
//! question three times: the band is the only thing that decides
//! whether a gap is reported, and the TIGHTEST band is where a
//! quiet claim is most likely to be false.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::witness_bodies::{keyway, oblique_lens, slit};
use common::*;
use geom_core::Tol;
use topo::Body;

fn corpus() -> Vec<(&'static str, Body<f64>)> {
    vec![
        ("ball", ball()),
        ("cone", cone()),
        ("cone_wedge", cone_wedge(2.0, 1.3)),
        ("donut", donut()),
        ("holed_prism", holed_prism()),
        ("l_prism", l_prism()),
        ("rounded_prism", rounded_prism()),
        ("sphere_wedge", sphere_wedge(2.0)),
        ("washer", washer()),
        ("wedge", wedge()),
        ("axis_wedge", axis_wedge()),
        ("keyway", keyway().0),
        ("oblique_lens", oblique_lens().0),
        ("slit", slit().0),
    ]
}

#[test]
fn the_coherence_examination_is_quiet_on_the_corpus() {
    let tol = Tol::witness();
    let mut noisy = Vec::new();
    for (name, body) in corpus() {
        let report = topo::examine_chart_coherence(&body, tol);
        if !report.findings.is_empty() || !report.unexamined.is_empty() {
            noisy.push(format!(
                "{name}: {} finding(s) {:?}, {} unexamined {:?}",
                report.findings.len(),
                report.findings,
                report.unexamined.len(),
                report.unexamined
            ));
        }
    }
    assert!(
        noisy.is_empty(),
        "the coherence examination fired on a body this crate mints and meshes. That \
         is a FINDING about the body or about the condition, not a threshold to \
         widen: read the metres against eps {} and decide which.\n{}",
        tol.eps(),
        noisy.join("\n")
    );
}
