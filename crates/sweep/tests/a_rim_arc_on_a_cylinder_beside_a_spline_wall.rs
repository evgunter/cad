//! **A rim arc lying on a cylinder, beside a spline wall.** A cylinder
//! `r = 0.5`, `z ∈ [0, 1]`, and a tube of the same section swept from its
//! top: the tube's flat start cap sits flush on the cylinder's top disc,
//! so each arc of the cap's rim lies on the cylinder's wall, and one of
//! the arc's two faces is the tube's spline wall. The boolean reads the
//! arc on the wall (`LiesOn`) and takes it when both its faces are
//! decided distinct from the wall; the spline one is read distinct where
//! every Bézier piece of its net has a point off the cylinder
//! (`reduce::spline_off_carrier`).
//!
//! The union then stops further on, at every ε alike: the tube's walls
//! are cut at each half arc's sub-arc joint, four quarter walls, and a
//! seam of the tube ending on the cylinder's top rim meets the
//! cylinder's wall at that rim, a meeting exact in ℝ that the curve-pair
//! reading encloses at about 1.5 ε, in band
//! (`work/splice/a-loop-seam-meeting-the-neck-cylinder-at-its-edge-reads-in-band.md`).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom::NurbsCurve3;
use geom_core::spline::KnotVector;
use geom_core::{Arc2, Point2, Point3, Tol};
use profile::{Profile, ProfileLoop, RawLoop, Segment, SketchPlane};
use sweep::{ExtrudeSide, Extrusion, extrude, sweep_body};
use topo::{AtRestBody, BooleanError};

use crate::common::normal_start_place;

const R: f64 = 0.5;

fn tol() -> Tol {
    Tol::witness()
}

/// The circle of radius [`R`] about the sketch origin, as two half arcs.
fn circle() -> ProfileLoop<f64> {
    RawLoop::new((0..2).map(|k| {
        let at = PI * f64::from(k);
        (
            Point2::new(R * at.cos(), R * at.sin()),
            Segment::Arc(Arc2 {
                centre: Point2::new(0.0, 0.0),
                radius: R,
                sweep: PI,
            }),
        )
    }))
}

/// The cylinder `r = R`, `z ∈ [0, 1]`.
fn cylinder() -> AtRestBody<f64> {
    let profile = Profile::new(SketchPlane::<f64>::xy(), vec![circle()])
        .validate(tol())
        .unwrap();
    let body = extrude(
        &profile,
        Extrusion::Distance {
            depth: 1.0,
            side: ExtrudeSide::Along,
        },
        tol(),
    )
    .unwrap()
    .body;
    topo::test_support::finished("the cylinder", body, tol())
}

/// The quadratic spine from `(0, 0, 1)` leaving along `+z`, through
/// `(0, 0, 2)` toward `end`.
fn spine(end: Point3<f64>) -> NurbsCurve3<f64> {
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let control = vec![Point3::new(0.0, 0.0, 1.0), Point3::new(0.0, 0.0, 2.0), end];
    NurbsCurve3::new(kv, control, vec![1.0; 3]).unwrap()
}

/// The circle swept along [`spine`] to `end`, at nine stations, cubic
/// along the spine.
fn tube(end: Point3<f64>) -> AtRestBody<f64> {
    let path = spine(end);
    let body = sweep_body::<f64>(&[circle()], normal_start_place(&path), &path, 9, 3, tol())
        .unwrap()
        .body;
    topo::test_support::finished("the tube", body, tol())
}

/// The tube bends toward `+x`, so its wall leaves the cylinder above
/// the rim on every piece: the rim arc is taken, and the union goes on.
/// It stops, at every ε row, where the tube's seam `6v1` meets the
/// cylinder's wall `3v1` on the wall's top rim: the curve-pair reading
/// against that edge (`Crossing(OnEdge)`) encloses the meeting at about
/// 1.5 ε, measured 1.521e-6, 1.538e-9 and 1.556e-12 at ε 1e-6, 1e-9 and
/// 1e-12, inside the band. The same holds with every reduction taking
/// its vertex pairs and their records in reverse
/// (`test_support::with_vertex_pairs_reversed`).
#[test]
fn a_rim_arc_beside_a_bent_spline_wall_is_taken_on_the_cylinder() {
    let union = || topo::union(&cylinder(), &tube(Point3::new(1.5, 0.0, 3.0)), tol());
    let (reversed, _) = topo::test_support::with_vertex_pairs_reversed(union);
    let eps = tol().eps();
    for (order, got) in [("as read", union()), ("reversed", reversed)] {
        assert!(
            !matches!(got, Err(BooleanError::CurvedPierceUnsupported { .. })),
            "{order}: the cap's rim arc lies on the cylinder beside a spline wall off it: {got:?}"
        );
        let key = |k: &dyn core::fmt::Debug| format!("{k:?}");
        let Err(BooleanError::CurvedPierceEscalated {
            operand: topo::Operand::B,
            face,
            edge,
            decision,
            diag,
        }) = &got
        else {
            panic!("{order}: the tube's seam escalates against the cylinder's rim: {got:?}")
        };
        assert!(
            key(face) == "FaceKey(3v1)"
                && key(edge) == "EdgeKey(6v1)"
                && key(decision) == "Crossing(OnEdge)"
                && diag.predicate == Some("spline_composite_past_the_band"),
            "{order}: the seam on the cylinder's rim: {got:?}"
        );
        // The reading's farther end from zero, through the margin's one
        // door to its numbers.
        let margin = match diag.margin.diagnostic_f64_for_error_text() {
            geom_core::ErrorTextReading::Value(m) => m.abs(),
            geom_core::ErrorTextReading::Enclosure { lo, hi } => lo.abs().max(hi.abs()),
            geom_core::ErrorTextReading::Invalid => f64::NAN,
        };
        // `margin / ε ∈ [1, 2)` records TODAY's reader, which encloses an
        // exact meeting a few ε wide. It should go red when SPLICE's class
        // row (`work/splice/a-loop-seam-meeting-the-neck-cylinder-at-its-edge-reads-in-band.md`)
        // improves the reader, and be re-pinned at the stop that follows.
        assert!(
            (1.0..2.0).contains(&(margin / eps)),
            "{order}: the reading encloses an exact meeting at about 1.5 ε, read {margin:e}"
        );
    }
}

/// **The tube's sub-arc joints are edges, and smooth ones.** The tube
/// sweeps one section, two half arcs, so every station holds its joints
/// at one place on the section: the sweep cuts each half arc's wall
/// there, four quarter walls with no interior `u` knot, and the edge
/// between two of them is a 180° wedge, the one-sided normals of its two
/// walls agreeing along it. The same holds at the profile's own two
/// vertices, where the half arcs meet tangent.
#[test]
fn the_tube_walls_sub_arc_joints_are_smooth_edges() {
    let t = tube(Point3::new(1.5, 0.0, 3.0));
    let walls: Vec<_> = t
        .faces()
        .filter_map(|(k, _)| match t.face_surface(k) {
            Some(geom::Surface::Nurbs(s)) => Some(s.clone()),
            _ => None,
        })
        .collect();
    assert_eq!(walls.len(), 4, "the tube's wall is four quarter walls");
    for s in &walls {
        let ku: Vec<(f64, usize)> = s.knot_values_u().runs().collect();
        assert_eq!(ku.len(), 2, "a quarter wall is one span in u: {ku:?}");
    }
    let normal = |s: &geom::NurbsSurface<f64>, u: f64, v: f64| {
        let h = 1e-7;
        let du = (s.eval((u + h).min(1.0), v) - s.eval((u - h).max(0.0), v)) / h;
        let dv = (s.eval(u, v + h) - s.eval(u, v - h)) / (2.0 * h);
        let n = du.cross(dv);
        n / n.norm()
    };
    let mut seams = 0;
    for a in &walls {
        for b in &walls {
            if a.eval(1.0, 0.5).distance(b.eval(0.0, 0.5)) > 1e-9 {
                continue;
            }
            seams += 1;
            for v in [0.1, 0.5, 0.9] {
                let (na, nb) = (normal(a, 1.0, v), normal(b, 0.0, v));
                assert!(
                    (na - nb).norm() <= 1e-5,
                    "the two walls' normals at a seam, v = {v}: {na:?} and {nb:?}"
                );
            }
        }
    }
    assert_eq!(seams, 4, "four seams, each between two quarter walls");
}
