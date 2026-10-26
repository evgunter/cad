//! **Reviewer probe (PR 4575): the tilted dipping round's tangent arc
//! at vertex-edge and vertex-vertex boolean vertices**, where the
//! first-order passes the PR leaves in place read the arc's side.
//!
//! The round of `an_arc_tangent_at_a_boolean_vertex.rs`, turned by `α`
//! about its rim vertex's tangent line (x through (·, 0, 1)), dips
//! below z = 1 only in the lens `(5/3 − π/2)/(2·tan α)`, which lies in
//! x ∈ [2, 3], y ∈ [0, 1/sin α]. Slabs z ∈ [0, 1] whose top edge or
//! corner passes through the rim vertex (2, 0, 1) put it at a
//! vertex-edge or vertex-vertex boolean vertex.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Point3;
use geom_core::{Affine3, Point2, Tol, Vec3};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::test_support::finished;
use sweep::{ExtrudeSide, Extrusion, extrude};
use topo::{AtRestBody, BooleanResult, mass_properties};

fn extrude_on(
    plane: SketchPlane<f64>,
    vertices: Vec<(Point2<f64>, f64)>,
    depth: f64,
    what: &str,
) -> AtRestBody<f64> {
    let profile = Profile::new(plane, vec![bulge_loop(vertices)])
        .validate(Tol::witness())
        .unwrap();
    let body = extrude(
        &profile,
        Extrusion::Distance {
            depth,
            side: ExtrudeSide::Along,
        },
        Tol::witness(),
    )
    .unwrap()
    .body;
    finished(what, body, Tol::witness())
}

fn tilted_dipping_round(alpha: f64) -> AtRestBody<f64> {
    tilted_round(alpha, true)
}

/// `dip`: the arc bends down (the PR's dipping round); otherwise the
/// quarter round, its arc bending up off z = 1.
fn tilted_round(alpha: f64, dip: bool) -> AtRestBody<f64> {
    let (sa, ca) = alpha.sin_cos();
    let plane = SketchPlane::from_frame(geom_core::test_support::rigid(
        &Affine3::from_parts(
            geom_core::Mat3::from_cols(
                Vec3::new(0.0, -sa, ca),
                Vec3::unit_x(),
                Vec3::new(0.0, ca, sa),
            ),
            Vec3::new(0.0, sa, 1.0 - ca),
        ),
        geom_core::Tol::witness(),
    ));
    let b90 = (core::f64::consts::PI / 8.0).tan();
    extrude_on(
        plane,
        vec![
            (Point2::new(1.0, 0.5), 0.0),
            (Point2::new(1.0, 2.0), if dip { b90 } else { -b90 }),
            (Point2::new(if dip { 0.0 } else { 2.0 }, 3.0), 0.0),
            (Point2::new(3.0, 3.0), 0.0),
            (Point2::new(3.0, 0.5), 0.0),
        ],
        4.0,
        "the tilted dipping round",
    )
}

/// x ∈ [x0, x1], y ∈ [y0, y0 + depth], z ∈ [0, 1].
fn slab(x0: f64, x1: f64, y0: f64, depth: f64) -> AtRestBody<f64> {
    let plane = SketchPlane::from_frame(geom_core::RigidMotion::axes_zx(Point3::new(0.0, y0, 0.0)));
    extrude_on(
        plane,
        vec![
            (Point2::new(0.0, x0), 0.0),
            (Point2::new(1.0, x0), 0.0),
            (Point2::new(1.0, x1), 0.0),
            (Point2::new(0.0, x1), 0.0),
        ],
        depth,
        "the slab",
    )
}

type R = Result<BooleanResult<f64>, topo::BooleanError>;

fn outcome(r: R) -> Result<f64, String> {
    match r {
        Ok(BooleanResult::Empty) => Ok(0.0),
        Ok(BooleanResult::Body(b)) => mass_properties(
            &b.body,
            Tol::witness(),
            topo::test_support::pinned_resolution(Tol::witness()),
        )
        .map(|m| m.enclosure.volume)
        .map_err(|e| format!("mass {e:?}")),
        Err(e) => Err(format!("{e:?}")),
    }
}

/// Prints every op's outcome against its closed form, and counts the
/// WRONG bodies (a refusal is not counted). Measured on PR 4575's head
/// (5ca970782b): no wrong body; `flush` answers; the slab edge x = 2
/// through the rim vertex refuses every op `Join(UnpairedLooseEnds {
/// count: 2 })`, the first-order arc's signature at the vertex-on-face
/// pass before the PR, and every posture with a slab face at y = 0
/// refuses `CurvedPierceUnsupported` first. It asserts nothing. Run:
/// `cargo test -p sweep --test all a_tilted_dip_at_vertex_edge_and_vertex_vertex_postures -- --ignored --nocapture`
#[test]
#[ignore = "reviewer probe: prints; run with --ignored --nocapture"]
fn a_tilted_dip_at_vertex_edge_and_vertex_vertex_postures() {
    let vol = |b: &AtRestBody<f64>| {
        mass_properties(
            b,
            Tol::witness(),
            topo::test_support::pinned_resolution(Tol::witness()),
        )
        .unwrap()
        .met()
        .expect("the pinned resolution is met")
        .volume
    };
    let mut wrong = 0;
    for alpha in [0.3f64, 0.6] {
        let round = tilted_dipping_round(alpha);
        let vr = vol(&round);
        let lens = (5.0 / 3.0 - core::f64::consts::FRAC_PI_2) / (2.0 * alpha.tan());
        for (posture, s, meet) in [
            ("edge x=2 (lens inside)", slab(2.0, 5.0, -6.0, 12.0), lens),
            (
                "edge y=0 along the tangent (lens inside)",
                slab(0.0, 5.0, 0.0, 6.0),
                lens,
            ),
            (
                "corner x=2,y=0 (lens inside)",
                slab(2.0, 5.0, 0.0, 6.0),
                lens,
            ),
            ("flush x<=2 (lens outside)", slab(0.0, 2.0, -6.0, 12.0), 0.0),
            (
                "corner x<=2,y>=0 (lens outside)",
                slab(0.0, 2.0, 0.0, 6.0),
                0.0,
            ),
            (
                "corner x<=2,y<=0 (lens outside)",
                slab(0.0, 2.0, -6.0, 6.0),
                0.0,
            ),
            (
                "edge y=0, y<=0 (lens outside)",
                slab(0.0, 5.0, -6.0, 6.0),
                0.0,
            ),
        ] {
            let vs = vol(&s);
            for (op, r, want) in [
                ("∪", topo::union(&s, &round, Tol::witness()), vs + vr - meet),
                (
                    "slab∖round",
                    topo::subtract(&s, &round, Tol::witness()),
                    vs - meet,
                ),
                (
                    "round∖slab",
                    topo::subtract(&round, &s, Tol::witness()),
                    vr - meet,
                ),
                ("∩", topo::intersect(&s, &round, Tol::witness()), meet),
            ] {
                let got = outcome(r);
                let verdict = match &got {
                    Ok(v) if (v - want).abs() <= 1e-6 => "ok",
                    Ok(_) => {
                        wrong += 1;
                        "WRONG"
                    }
                    Err(_) => "refused",
                };
                eprintln!("PROBE α={alpha} {posture} {op}: {verdict} want {want:.7} got {got:?}");
            }
        }
    }
    eprintln!("PROBE wrong bodies: {wrong}");
}

/// **The quarter round (its arc bending UP off z = 1), turned by
/// `α = 0.3` about the rim's tangent line, keeps its operands** on a
/// wide slab (the rim vertex inside the top face) and on a slab whose
/// top edge x = 2 runs through the rim vertex (a vertex-edge boolean
/// vertex). Turned this way it touches z = 1 only along its bottom
/// face's edge at y = 0, so every op is the operands'.
///
/// The edge posture is read by the first-order passes PR 4575 leaves in
/// place, with the arc tied `On`: it answers there, and descending the
/// arc's side in `side_code` too (as
/// `work/contacthold/conic-ties-outside-the-vertex-on-face-pass-still-read-first-order.md`
/// would) refused it, measured, `ClassificationInvariant "germ record
/// without a clean In/Out transition after rewrite"`.
#[test]
fn a_rising_round_tilted_onto_a_slab_edge_keeps_its_operands() {
    let vol = |b: &AtRestBody<f64>| {
        mass_properties(
            b,
            Tol::witness(),
            topo::test_support::pinned_resolution(Tol::witness()),
        )
        .unwrap()
        .met()
        .expect("the pinned resolution is met")
        .volume
    };
    let round = tilted_round(0.3, false);
    let vr = vol(&round);
    for (posture, s) in [
        ("wide", slab(0.0, 5.0, -6.0, 12.0)),
        ("edge x = 2", slab(2.0, 5.0, -6.0, 12.0)),
    ] {
        let vs = vol(&s);
        for (op, r, want) in [
            ("∪", topo::union(&s, &round, Tol::witness()), vs + vr),
            (
                "slab ∖ round",
                topo::subtract(&s, &round, Tol::witness()),
                vs,
            ),
            (
                "round ∖ slab",
                topo::subtract(&round, &s, Tol::witness()),
                vr,
            ),
            ("∩", topo::intersect(&s, &round, Tol::witness()), 0.0),
        ] {
            let got = outcome(r).unwrap_or_else(|e| panic!("{posture}, {op}: refused {e}"));
            assert!(
                (got - want).abs() <= 1e3 * Tol::witness().eps(),
                "{posture}, {op}: {got} vs {want}"
            );
        }
    }
}
