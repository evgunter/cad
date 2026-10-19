//! **A tangent joint on near-concentric carriers is the sweep's to
//! refuse, and a joint at their crossing is a corner** (Ev's ruling on
//! #5354). The profile validates a tangent cusp however far its two
//! arcs run within ε of each other past the joint; whether the joint is
//! a legal EDGE is decided where it becomes one. The extrude reads its
//! second-order arm there: walls whose curvatures differ by `d` ≈ 17ε
//! part by `d·r²/2` ≈ 8.5ε at the unit arm, in band, and the join
//! refuses `SliverJoin` naming the profile's joint, at a cusp and at a
//! smooth joint alike; at 40ε and 100ε they part definitely and the
//! body builds and passes tier 3. A joint at a crossing of such
//! carriers turns by `√(2gd)`, a definite corner at the unit arm, which
//! the profile's joint pass reads as one and the extrude builds sharp.
//! Every row runs at `f64` and at `Interval`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Arc2, Interval, Point2, Real, Tol};
use profile::{Profile, ProfileLoop, RawLoop, Segment, SketchPlane, ValidatedProfile};
use sweep::{ExtrudeError, ExtrudeSide, Extrusion, extrude};

fn rim(c: Point2<f64>, r: f64, a: f64) -> Point2<f64> {
    Point2::new(c.x + r * a.cos(), c.y + r * a.sin())
}

/// One loop: an arc of the unit circle about the origin from `at − span`
/// into the joint at polar angle `at`, an arc of the circle about
/// `(d, 0)` of radius `1 − d + g` out of it through `sweep` (back the
/// way the first came at a cusp), and a line home.
fn joint_loop(d: f64, g: f64, at: f64, span: f64, sweep: f64) -> Profile<f64> {
    let (c1, c2, r2) = (Point2::new(0.0, 0.0), Point2::new(d, 0.0), 1.0 - d + g);
    let arc = |centre, radius, sweep| {
        Segment::Arc(Arc2 {
            centre,
            radius,
            sweep,
        })
    };
    let joint = rim(c1, 1.0, at);
    let from = (joint.y - c2.y).atan2(joint.x - c2.x);
    Profile::new(
        SketchPlane::xy(),
        vec![ProfileLoop::new(vec![
            (rim(c1, 1.0, at - span), arc(c1, 1.0, span)),
            (joint, arc(c2, r2, sweep)),
            (rim(c2, r2, from + sweep), Segment::Line),
        ])],
    )
}

/// The profile validated at `f64`, and its `Interval` lift.
fn validated(p: &Profile<f64>) -> (ValidatedProfile<f64>, ValidatedProfile<Interval>) {
    let tol = Tol::witness();
    let v = p.validate(tol).expect("the profile validates");
    let lifted = Profile::new(
        SketchPlane::xy(),
        p.loops
            .iter()
            .map(|lp| lp.map_scalar(Interval::from_f64))
            .collect(),
    )
    .validate(tol)
    .expect("the profile validates at Interval");
    (v, lifted)
}

fn up<T: Real>() -> Extrusion<T> {
    Extrusion::Distance {
        depth: T::from_f64(1.0),
        side: ExtrudeSide::Along,
    }
}

/// **Walls 17ε apart in curvature refuse their join, cusp or smooth.**
/// `g = 0` is pinned at 1e-6 only, where the profile validates it (at
/// the finer rows the stored radius rounds `g` into a chord the pair
/// pass refuses).
#[test]
fn a_join_whose_walls_part_in_band_refuses_its_sliver() {
    let eps = Tol::witness().eps();
    let mut rows = vec![(-0.5, -1.6, "cusp"), (-0.5, 1.6, "smooth")];
    if format!("{eps:e}") == "1e-6" {
        rows.push((0.0, -1.6, "cusp, g = 0"));
    }
    for (g, sweep, what) in rows {
        let (v, lifted) = validated(&joint_loop(17.0 * eps, g * eps, 0.0, 2.0, sweep));
        let at_f64 = extrude(&v, up(), Tol::witness()).map(|_| ());
        let at_interval = extrude(&lifted, up(), Tol::witness()).map(|_| ());
        for (lane, got) in [("f64", at_f64), ("Interval", at_interval)] {
            match got {
                Err(ExtrudeError::SliverJoin {
                    loop_index: 0,
                    vertex_index: 1,
                    source,
                    ..
                }) => assert_eq!(
                    source.predicate,
                    Some("tangent_second_order"),
                    "{what} at {lane}: the walls' second-order arm"
                ),
                other => panic!("{what} at {lane}: the join must refuse its sliver, got {other:?}"),
            }
        }
        let margin = match extrude(&v, up(), Tol::witness()) {
            Err(ExtrudeError::SliverJoin { source, .. }) => source
                .margin
                .diagnostic_f64_for_error_text()
                .value()
                .expect("an f64 margin is a point"),
            other => panic!("{what}: {other:?}"),
        };
        assert!(
            (margin / eps - 8.5).abs() < 1e-3,
            "{what}: the walls part by d·r²/2 = 8.5ε at the unit arm, got {:e}ε",
            margin / eps
        );
    }
}

/// **Walls 40ε and 100ε apart in curvature build a legal cusp**, their
/// arcs within ε of each other for 0.27 and 0.17 rad past the joint.
#[test]
fn a_cusp_whose_walls_part_definitely_builds_legal() {
    let eps = Tol::witness().eps();
    for d in [40.0, 100.0] {
        let (v, lifted) = validated(&joint_loop(d * eps, -0.5 * eps, 0.0, 2.0, -1.6));
        assert_eq!(v.loops()[0].cusp_joints(), &[1], "d = {d}ε: a cusp");
        let body = extrude(&v, up(), Tol::witness())
            .unwrap_or_else(|e| panic!("d = {d}ε: the cusp builds, got {e:?}"))
            .body;
        topo::validate_geometric(&body, Tol::witness())
            .unwrap_or_else(|e| panic!("d = {d}ε: and passes tier 3, got {e:?}"));
        extrude(&lifted, up(), Tol::witness())
            .unwrap_or_else(|e| panic!("d = {d}ε at Interval: the cusp builds, got {e:?}"));
    }
}

/// **A joint at a crossing of near-concentric carriers builds a sharp
/// edge.** `d = 0.01`, `g = 0.5ε`: the joint turns by about
/// `√(2gd)`, 3e-6 rad at the default ε, and the profile records no
/// tangent joint there, back the way the first arc came or on round.
#[test]
fn a_joint_at_a_crossing_builds_a_corner() {
    let eps = Tol::witness().eps();
    let d: f64 = 0.01;
    let r2 = 1.0 - d + 0.5 * eps;
    let at = -((1.0 + d * d - r2 * r2) / (2.0 * d)).acos();
    for sweep in [-0.8, 0.8] {
        let (v, lifted) = validated(&joint_loop(d, 0.5 * eps, at, 1.0, sweep));
        assert!(
            v.loops()[0].tangent_joints().is_empty(),
            "sweep {sweep}: the joint is a corner"
        );
        let body = extrude(&v, up(), Tol::witness())
            .unwrap_or_else(|e| panic!("sweep {sweep}: the corner builds, got {e:?}"))
            .body;
        topo::validate_geometric(&body, Tol::witness())
            .unwrap_or_else(|e| panic!("sweep {sweep}: and passes tier 3, got {e:?}"));
        extrude(&lifted, up(), Tol::witness())
            .unwrap_or_else(|e| panic!("sweep {sweep} at Interval: the corner builds, got {e:?}"));
    }
}
