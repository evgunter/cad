//! **R2 review probes for MESH-7** — the mesh side of the shape door,
//! asked in the directions the unit's own rows do not ask:
//!
//! * `TessellateError::Band` is a brand-new arm on a public enum whose
//!   only claimed reachability is a run ε within a factor K of
//!   `f64::MAX`. This file reaches it — under the run tolerance the
//!   binary was started with, so the row states which side of that it
//!   is on rather than asserting one;
//! * the issue-1562 refusal is pinned in the unit at ONE split of ONE
//!   edge. The matrix below splits **every** edge of the donut at both
//!   of the issue-653 sweep's patterns, so "the four new refusals are
//!   exactly the seam meridians" is measured rather than asserted;
//! * the witness bodies are offered as *valid input, lane not built*
//!   (D2 addendum row 2). This asks `topo::validate` whether they are
//!   in fact valid, and where the tier gate stands on them;
//! * the door runs BEFORE the walk — including before the walk's own
//!   typed refusals, which is an ordering that changes which error a
//!   caller sees.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![allow(clippy::print_stdout)]

mod common;

use common::witness_bodies::{keyway, oblique_lens};
use common::*;
use geom_brep::props::PropsError;
use geom_core::Tol;
use mesh::TessellateError;

/// **Reaching `TessellateError::Band`.** `Band::linear` fails only
/// when `K·ε` overflows, and ε is the run's global tolerance, so this
/// arm is a property of the PROCESS and not of any body. Under the
/// default ε no input reaches it. Under `CAD_TOLERANCE_EPS=1e308` it is
/// reached — but only by a body that was never built at that ε: every
/// fixture in `common` panics inside profile validation long before
/// `tessellate` is called, so the reachable witness is the EMPTY body,
/// on which the band is minted unconditionally at operation entry and
/// refused although nothing in the call would have used it.
#[test]
fn the_band_arm_is_a_property_of_the_run_not_of_the_body() {
    let tol = Tol::witness();
    let eps = tol.eps();
    let empty = topo::Body::<f64>::new();
    let got_empty = mesh::tessellate(&empty, 0.2, tol).map(|m| m.positions.len());
    println!(
        "run eps = {eps:e}, K = {}: empty body = {got_empty:?}",
        tol.k()
    );
    if eps * tol.k() > f64::MAX {
        assert!(
            matches!(got_empty, Err(TessellateError::Band { .. })),
            "K·eps overflows: the arm is reached with no face at all; got {got_empty:?}"
        );
    } else {
        let planar = mesh::tessellate(&l_prism(), 0.2, tol).map(|_| ());
        let curved = mesh::tessellate(&donut(), 0.2, tol).map(|_| ());
        println!("run eps = {eps:e}: planar = {planar:?}, curved = {curved:?}");
        assert!(
            !matches!(got_empty, Err(TessellateError::Band { .. }))
                && !matches!(curved, Err(TessellateError::Band { .. })),
            "under a representable band no input reaches the arm"
        );
    }
}

/// **The issue-1562 refusal, over the whole edge × pattern matrix.**
/// The unit pins one split of edge 0; the issue-653 sweep's totals
/// moved by four. This walks every edge of the donut at both of the
/// sweep's patterns and prints the verdict, so the claim "the four new
/// refusals are exactly the seam-meridian splits (edges 0 and 1, both
/// patterns)" is a measurement.
#[test]
fn the_new_refusals_are_exactly_the_two_seam_meridians() {
    let tol = Tol::witness();
    let patterns: [&[f64]; 2] = [&[0.5], &[0.3129, 0.15645]];
    let n_edges = donut().edges().count();
    let mut refused = Vec::new();
    for i in 0..n_edges {
        for (pi, fracs) in patterns.iter().enumerate() {
            let mut body = donut();
            let (ek, edge) = body.edges().nth(i).unwrap();
            let curve = body
                .get_curve_geom(edge.curve)
                .unwrap()
                .certified()
                .unwrap();
            let (t0, t1) = curve.params();
            let radius = match curve.carrier() {
                geom::Curve3::Circle { radius, .. } => *radius,
                _ => f64::NAN,
            };
            for f in *fracs {
                body.split_edge(ek, t0 + f * (t1 - t0), tol).unwrap();
            }
            let got = mesh::tessellate(&body, 0.1, tol).map(|m| m.positions.len());
            println!("edge {i} (r = {radius}), pattern {pi}: {got:?}");
            if let Err(TessellateError::UnsupportedCurvedShape { source, .. }) = &got {
                refused.push((i, pi, format!("{source:?}")));
            }
        }
    }
    assert_eq!(
        refused.iter().map(|(i, p, _)| (*i, *p)).collect::<Vec<_>>(),
        vec![(0, 0), (0, 1), (1, 0), (1, 1)],
        "exactly the two minor (seam meridian) circles, at both patterns: {refused:?}"
    );
    assert!(
        refused
            .iter()
            .all(|(_, _, w)| w.contains("props_rim_level")),
        "all four by props_rim_level: {refused:?}"
    );
}

/// **Are the witnesses valid bodies?** The refusal is offered as D2
/// addendum row 2 — *valid input, lane not built* — so what the tier
/// gates say about the keyway and the lens is part of the claim.
/// Recorded, not asserted beyond the topological tiers: tier 3 is
/// expected to refuse both through `mass_properties`, which is the
/// census's row 8.
#[test]
fn the_witness_bodies_are_topologically_valid_and_tier3_refuses_them() {
    for (name, (body, _face)) in [("keyway", keyway()), ("oblique lens", oblique_lens())] {
        let topo_ok = topo::validate(&body);
        let closed = topo::validate_closed(&body);
        let mass = topo::mass_properties(&body, Tol::witness()).map(|m| m.volume);
        println!("{name}: validate = {topo_ok:?}, closed = {closed:?}, mass = {mass:?}");
        assert!(topo_ok.is_ok(), "{name} is a topologically valid body");
    }
}

/// **Ordering.** The door runs before `Chart::of` and before the walk,
/// so on a face that would ALSO have tripped a later refusal the caller
/// now sees the shape refusal. The keyway is the case in tree: on this
/// build it refuses at the door; its walk (the same body on main)
/// refused `UnsupportedCurvedDomain`. Both are typed refusals of the
/// same body, and this row records which one a caller gets now.
#[test]
fn the_keyway_refuses_at_the_door_rather_than_at_the_spatial_check() {
    let (body, face) = keyway();
    let got = mesh::tessellate(&body, 0.05, Tol::witness()).map(|_| ());
    println!("keyway: {got:?}");
    assert_eq!(
        got,
        Err(TessellateError::UnsupportedCurvedShape {
            face,
            source: PropsError::NotIsoRectangle {
                what: "props_rim_level"
            },
        })
    );
}
