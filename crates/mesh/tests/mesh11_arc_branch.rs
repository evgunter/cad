//! **The walk's arc premise, verified at the door** (issue 1571).
//!
//! `walk.rs` needs every boundary edge to be an iso curve of the chart
//! traversed on ONE branch. `props::require_iso_rectangle` certifies
//! the CARRIER; `props::require_one_chart_branch` certifies the
//! traversed ARC, and `curved::require_iso_rectangle_face` cites both
//! in that order. These rows are the two π-rad witnesses the walk's
//! ledger names, from the outside, through the public `tessellate`.
//!
//! Each witness is a body that used to reach the walk: with debug
//! assertions on it panicked at the cross-face identification census
//! (issue 897) and with them off the walk returned a NON-watertight
//! mesh `Ok`; at finer δ it refused `CertificateExceeded`, a refusal
//! about the chord certificate rather than about the premise. What
//! these rows pin is that neither outcome is reachable any more: the
//! refusal is typed, names the premise, and comes before any mesh is
//! minted — at EVERY δ, which is the part `CertificateExceeded` never
//! gave.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::witness_bodies::{apex_crossing_bowtie, pole_crossing_half_cap};
use geom_brep::props::PropsError;
use geom_core::Tol;
use mesh::TessellateError;
use topo::{Body, FaceKey};

/// Every face of `body` refuses at the shape door with the branch
/// predicate's own error, at every δ — the refusal is a fact about the
/// body's boundary statement, not about a chord budget.
fn refuses_the_branch_premise(name: &str, body: &Body<f64>, faces: [FaceKey; 2]) {
    let tol = Tol::witness();
    for delta in [0.5, 0.3, 0.2, 0.1, 0.05, 0.02] {
        let got = mesh::tessellate(body, delta, tol);
        match got {
            Err(TessellateError::UnsupportedCurvedShape {
                face,
                source: PropsError::NotOneChartBranch { .. },
            }) => assert!(
                faces.contains(&face),
                "{name} at δ={delta}: refused a face that is not one of {faces:?}"
            ),
            other => panic!(
                "{name} at δ={delta}: expected the branch premise refusal, got {other:?}"
            ),
        }
    }
}

/// **Witness 1, the Euler-door body** (issue 1571's own construction):
/// the unit sphere whose ONE meridian edge is a great-circle arc over
/// the north pole. Before this unit the door admitted both faces and
/// the walk ran; now the door names the premise and no mesh is minted.
#[test]
fn the_pole_crossing_half_cap_refuses_at_the_door() {
    let (body, cap, rest) = pole_crossing_half_cap();
    refuses_the_branch_premise("half-cap", &body, [cap, rest]);
}

/// **Witness 2's sibling on the other kind** — the cone bow tie, whose
/// generators run through the apex. Found by this unit's class sweep:
/// the shape door admitted it and the walk mis-read it exactly as it
/// mis-read the half-cap (a debug build panicked at the issue-897
/// census at δ = 0.5, `CertificateExceeded` below it).
#[test]
fn the_apex_crossing_bowtie_refuses_at_the_door() {
    let (body, f0, f1) = apex_crossing_bowtie();
    refuses_the_branch_premise("bow tie", &body, [f0, f1]);
}

/// **The props-side finding is NOT closed by this door, and this row
/// says so with the measurement** (issue 1598). `mass_properties` does
/// not cite the branch predicate — citing it there would retract
/// CERT-1, whose three rows measure pole-crossing arcs exactly — so
/// the half-cap body still answers, and what it answers is 0.0 for a
/// closed unit sphere: its two faces are bounded by the same two edges
/// traversed opposite ways, so one parse hands both the same levels
/// and their fluxes cancel. Tier 3 catches it only through check 6.
#[test]
fn mass_properties_still_answers_zero_on_the_half_cap() {
    let (body, _, _) = pole_crossing_half_cap();
    let mp = topo::mass_properties(&body, Tol::witness()).expect("props answers");
    assert_eq!(
        mp.volume, 0.0,
        "issue 1598: equal-and-opposite flux from one parse, on a closed sphere"
    );
}
