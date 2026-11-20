//! **R2 review probes for MESH-7** — the mesh side of the shape door,
//! asked in the directions the unit's own rows do not ask:
//!
//! * `TessellateError::Band` is a brand-new arm on a public enum whose
//!   only claimed reachability is a run ε within a factor K of
//!   `f64::MAX`. This file reaches it — under the run tolerance the
//!   binary was started with, so the row states which side of that it
//!   is on rather than asserting one;
//! * the split-seam donut is pinned in `iso_rectangle_door` at ONE
//!   split of ONE edge. The matrix below splits **every** edge of the
//!   donut at both of the issue-653 sweep's patterns, so "every split
//!   of the donut meshes and measures as the donut" is measured
//!   rather than asserted;
//! * the witness bodies are offered as *valid input, lane not built*
//!   (D2 addendum row 2). This asks `topo::validate` whether they are
//!   in fact valid, and where the tier gate stands on them;
//! * the door runs BEFORE the walk — including before the walk's own
//!   typed refusals, which is an ordering that changes which error a
//!   caller sees.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![allow(clippy::print_stdout)]

use crate::common;
use topo::construction::Construct;

use common::witness_bodies::{keyway, minted, oblique_lens};
use common::*;
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

/// **Every split of the donut, over the whole edge × pattern matrix.**
/// `iso_rectangle_door` pins one split of the seam; this walks every
/// edge of the donut — the two seam meridians and the two rims — at
/// both of the issue-653 sweep's patterns and prints the verdict, so
/// "a split seam meridian is folded back into its meridian and a
/// split rim was always fine" is a measurement over the matrix: every
/// configuration meshes, and measures the donut's volume bitwise.
#[test]
fn every_donut_split_meshes_and_measures_as_the_donut() {
    let tol = Tol::witness();
    let patterns: [&[f64]; 2] = [&[0.5], &[0.3129, 0.15645]];
    let n_edges = donut().edges().count();
    let v0 = topo::test_support::pinned_volume(
        "the measured body",
        &topo::test_support::finished("the measured body", donut().clone(), tol),
        tol,
    )
    .volume
    .to_bits();
    let mut wrong = Vec::new();
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
                geom::Curve3::Circle(circle) => {
                    let radius = circle.radius();
                    radius
                }
                _ => f64::NAN,
            };
            for f in *fracs {
                body.split_edge(ek, t0 + f * (t1 - t0), tol).unwrap();
            }
            let got = mesh::tessellate(&body, 0.1, tol).map(|m| m.positions.len());
            let mp = match topo::finish(body.clone(), tol) {
                Ok(finished) => topo::mass_properties(
                    &finished,
                    tol,
                    topo::test_support::pinned_resolution(tol),
                ),
                Err(errors) => {
                    assert!(
                        errors.iter().all(|e| matches!(
                            e,
                            topo::ValidationError::JoinableVertexAtRest { .. }
                        )),
                        "edge {i}, pattern {pi}: tier 3 refuses the split body for more than its joinable vertices: {errors:?}"
                    );
                    // Raw door: a split that leaves joinable vertices at rest, which tier 3 refuses (JoinableVertexAtRest), as the assert above admits.
                    topo::test_support::measure_unvalidated(
                        &body,
                        tol,
                        topo::test_support::pinned_resolution(tol),
                    )
                }
            };
            let vol = mp.map(|m| m.enclosure.volume);
            println!("edge {i} (r = {radius}), pattern {pi}: {got:?}, V = {vol:?}");
            if got.is_err() || vol.as_ref().map(|v| v.to_bits()) != Ok(v0) {
                wrong.push((i, pi, format!("{got:?} / {vol:?}")));
            }
        }
    }
    assert!(
        wrong.is_empty(),
        "every edge at both patterns meshes and measures the donut's volume bitwise: {wrong:?}"
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
        // Raw door: the measured body is a body still carrying construction scaffolding, which tier 3 refuses (ScaffoldAtRest).
        let mass = topo::test_support::measure_unvalidated(
            &body,
            Tol::witness(),
            topo::test_support::pinned_resolution(Tol::witness()),
        )
        .map(|m| m.enclosure.volume);
        println!("{name}: validate = {topo_ok:?}, closed = {closed:?}, mass = {mass:?}");
        assert!(topo_ok.is_ok(), "{name} is a topologically valid body");
    }
}

/// **Ordering.** The doors route the face before `Chart::of` and before
/// the walk, so a face they decide is no rectangle never reaches the
/// walk (whose spatial check refused the keyway `UnsupportedCurvedDomain`
/// before the doors existed). The doors decide the keyway on
/// `props_rim_level` and the route sends it to the trimmed lane where
/// that lane can read its domain:
///
/// - **with its images minted** the keyway face meshes there, and the
///   first refusal in arena order is the sheet's other face, the
///   keyway's complement on the unbounded wall, whose loop is wound
///   against it (`TrimLoopOrientation`);
/// - **as the Euler doors leave it**, with no image stored, the lane
///   has nothing to read, and the refusal is the domain read's.
#[test]
fn the_keyway_is_routed_before_the_spatial_check() {
    let (body, face) = keyway();
    let got = mesh::tessellate(&minted(body.clone()), 0.05, Tol::witness()).map(|_| ());
    assert!(
        matches!(got, Err(TessellateError::TrimLoopOrientation { face: other }) if other != face),
        "the keyway face meshes on the trimmed lane and its complement refuses: {got:?}"
    );
    let got = mesh::tessellate(&body, 0.05, Tol::witness()).map(|_| ());
    assert!(
        matches!(
            &got,
            Err(TessellateError::DomainUnread { face: f, cause })
                if *f == face && matches!(**cause, topo::pcurves::RingPlacementError::Gap(_))
        ),
        "{got:?}"
    );
}
