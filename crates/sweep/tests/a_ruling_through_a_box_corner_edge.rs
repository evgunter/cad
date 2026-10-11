//! **A wall's ruling along a box edge.** A cylinder whose wall passes
//! through an edge of a box, a ruling of the wall, with the box on the
//! wall's inner side there and crossing the wall further on: both box
//! faces at the edge meet the wall along the edge alone, so its two
//! ends rest on the wall's interior. The section divides the wall, and
//! the piece holding the rest need not be the piece that keeps the
//! wall's key:
//!
//! - `∩` keeps the edge and the wall's far piece: the rest left with
//!   the near piece, and the result records nothing.
//! - wall `∖` box keeps the edge and the near piece, the two wedges of
//!   wall material beside the box touching along the edge: two v-f
//!   records, each on the piece that holds its vertex.
//! - `∪` and box `∖` wall lose the edge, and record nothing.
//!
//! A cone's ruling along a square prism's edge is the same pose on a
//! carrier whose rulings are not parallel, and refuses at the crossing
//! layer, before any record is carried.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_core::{Band, OrthoFrame, Point2, Point3, Tol, Vec3};
use profile::{Profile, SketchPlane};
use sweep::test_support::{brick, corners, finished, prism_on, revolved_about_y, sketch_from_axes};
use sweep::{ExtrudeSide, Extrusion, Revolution, extrude};
use topo::{AtRestBody, BooleanError, BooleanResult, FaceContainment};

/// The wall's centre is `C` in from the edge along both box faces, so
/// its radius is `C√2` and it crosses each of those faces `2C` from
/// the edge.
const C: f64 = 0.3;

fn tol() -> Tol {
    Tol::witness()
}

/// The box `[0, 2]² × [0, 1]`.
fn the_box() -> AtRestBody<f64> {
    finished(
        "the box",
        brick((0.0, 2.0), (0.0, 2.0), (0.0, 1.0), tol()),
        tol(),
    )
}

/// The world axis a box edge runs along, as the exact sketch frame
/// whose normal it is, the box's extent along it, and the box's two
/// extents across it in the frame's `u`, `v` order.
#[derive(Clone, Copy, Debug)]
enum Axis {
    X,
    Y,
    Z,
}

impl Axis {
    fn frame(self, at: f64) -> OrthoFrame<f64> {
        match self {
            Self::X => OrthoFrame::axes_yz(Point3::new(at, 0.0, 0.0)),
            Self::Y => OrthoFrame::axes_zx(Point3::new(0.0, at, 0.0)),
            Self::Z => OrthoFrame::axes_xy(Point3::new(0.0, 0.0, at)),
        }
    }

    fn length(self) -> f64 {
        match self {
            Self::X | Self::Y => 2.0,
            Self::Z => 1.0,
        }
    }

    fn across(self) -> [(f64, f64); 2] {
        match self {
            Self::X => [(0.0, 2.0), (0.0, 1.0)],
            Self::Y => [(0.0, 1.0), (0.0, 2.0)],
            Self::Z => [(0.0, 2.0), (0.0, 2.0)],
        }
    }
}

/// The cylinder along `axis` whose wall holds the box edge at `(u, v)`
/// (frame coordinates), its centre `C` in from the edge along both box
/// faces there, running one past the box at both ends.
fn wall_through(axis: Axis, (u, v): (f64, f64), (su, sv): (f64, f64)) -> AtRestBody<f64> {
    let t = tol();
    let centre = Point2::new(u + su * C, v + sv * C);
    let circle = profile::circle(centre, C * 2.0_f64.sqrt(), t).unwrap();
    let p = Profile::new(
        SketchPlane::from_frame(axis.frame(-1.0)),
        vec![circle.into()],
    )
    .validate(t)
    .unwrap();
    let depth = axis.length() + 2.0;
    let side = ExtrudeSide::Along;
    let body = extrude(&p, Extrusion::Distance { depth, side }, t)
        .unwrap()
        .body;
    finished("the wall", body, t)
}

/// One op's label, its result, and the v-f records it owes.
type Run = (
    &'static str,
    Result<BooleanResult<f64>, BooleanError>,
    usize,
);

/// Every op of the wall `w` and the box `b` in both orders, with the
/// number of v-f records each owes: two for `w ∖ b`, none for the rest.
fn six(w: &AtRestBody<f64>, b: &AtRestBody<f64>) -> [Run; 6] {
    let t = tol();
    [
        ("w ∩ b", topo::intersect(w, b, t), 0),
        ("b ∩ w", topo::intersect(b, w, t), 0),
        ("w ∪ b", topo::union(w, b, t), 0),
        ("b ∪ w", topo::union(b, w, t), 0),
        ("w ∖ b", topo::subtract(w, b, t), 2),
        ("b ∖ w", topo::subtract(b, w, t), 0),
    ]
}

/// The result's volume, after checking it passes tiers 2 and 3′ against
/// its own records and carries `vf` v-f records and no other kind, each
/// on a curved face whose region holds its vertex.
fn checked(what: &str, r: Result<BooleanResult<f64>, BooleanError>, vf: usize) -> f64 {
    let t = tol();
    let bb = match r {
        Ok(BooleanResult::Body(bb)) => bb,
        other => panic!("{what}: builds a body: {other:?}"),
    };
    topo::validate_closed(&bb.body).unwrap_or_else(|e| panic!("{what}: tier 2: {e:?}"));
    topo::validate_pseudomanifold(&bb.body, &bb.contacts, t)
        .unwrap_or_else(|e| panic!("{what}: tier 3′ against its records: {e:?}"));
    let c = &bb.contacts;
    let rests: Vec<_> = c.a_on_b.iter().chain(&c.b_on_a).collect();
    assert_eq!(rests.len(), vf, "{what}: v-f records {rests:?}");
    assert!(
        c.vv.is_empty()
            && c.ve.is_empty()
            && c.ee.is_empty()
            && c.curves.is_empty()
            && c.patches.is_empty(),
        "{what}: no other record kind: {c:?}"
    );
    let band = Band::linear(t).unwrap();
    for r in rests {
        let q = bb
            .body
            .vertex_points()
            .find(|&(k, _)| k == r.vertex)
            .map(|(_, q)| q)
            .unwrap_or_else(|| panic!("{what}: {r:?}'s vertex is live"));
        assert_eq!(
            topo::curved_face_containment(&bb.body, r.face, q, band).unwrap(),
            Some(FaceContainment::In),
            "{what}: {r:?}'s face holds its vertex"
        );
    }
    topo::mass_properties(&bb.body, t).unwrap().volume
}

/// **A cylinder's ruling along each of the box's twelve edges**: every
/// op in both orders builds at its closed form, passes tier 3′ against
/// its own records, and records the rest exactly where the result
/// keeps it. Before the records were carried through the wall's
/// division, `∩` named the far piece, which holds neither end, and
/// `w ∖ b` dropped both rests with the piece that kept the key.
#[test]
fn a_wall_through_a_box_edge_records_the_rest_on_the_piece_that_holds_it() {
    let b = the_box();
    let sign = |at: f64, (lo, _): (f64, f64)| if at == lo { 1.0 } else { -1.0 };
    for axis in [Axis::X, Axis::Y, Axis::Z] {
        let [ru, rv] = axis.across();
        for u in [ru.0, ru.1] {
            for v in [rv.0, rv.1] {
                let w = wall_through(axis, (u, v), (sign(u, ru), sign(v, rv)));
                let len = axis.length();
                let vw = 2.0 * PI * C * C * (len + 2.0);
                let vb = 4.0;
                let common = (PI + 2.0) * C * C * len;
                for (op, r, vf) in six(&w, &b) {
                    let what = format!("{axis:?} edge at ({u}, {v}): {op}");
                    let want = match op {
                        "w ∩ b" | "b ∩ w" => common,
                        "w ∪ b" | "b ∪ w" => vw + vb - common,
                        "w ∖ b" => vw - common,
                        _ => vb - common,
                    };
                    let got = checked(&what, r, vf);
                    assert!(
                        (got - want).abs() < 1e-9,
                        "{what}: volume {got} against {want}"
                    );
                }
            }
        }
    }
}

/// The frustum about `y` from radius 1 at `y = 0` to `0.5` at `y = 2`,
/// and the square prism of side `s` whose edge is its ruling at azimuth
/// `az` between `y = 0.5` and `y = 1.5`, the square opening into the
/// cone at 45° to each side of the inward normal.
fn cone_and_prism(az: f64, s: f64) -> (AtRestBody<f64>, AtRestBody<f64>) {
    let t = tol();
    let cone = finished(
        "the cone",
        revolved_about_y(
            vec![
                (Point2::new(0.0, 0.0), 0.0),
                (Point2::new(1.0, 0.0), 0.0),
                (Point2::new(0.5, 2.0), 0.0),
                (Point2::new(0.0, 2.0), 0.0),
            ],
            Revolution::Full,
            t,
        ),
        t,
    );
    // The prism runs down the ruling from its top end: the sketch
    // frame's normal, `across × inward`, points down it.
    let (sin, cos) = az.sin_cos();
    let turn = |p: Vec3<f64>| Vec3::new(p.x * cos + p.z * sin, p.y, p.z * cos - p.x * sin);
    let top = turn(Vec3::new(0.625, 1.5, 0.0));
    let inward = turn(Vec3::new(-2.0, -0.5, 0.0)).normalize();
    let across = turn(Vec3::new(0.0, 0.0, 1.0));
    let plane = sketch_from_axes(Point3::origin() + top, across + inward, inward - across, t);
    let pts = [(0.0, 0.0), (s, 0.0), (s, s), (0.0, s)];
    let depth = (0.25_f64 * 0.25 + 1.0).sqrt();
    (
        cone,
        finished("the prism", prism_on(plane, corners(&pts), depth, t), t),
    )
}

/// **A prism edge along a cone's ruling refuses, typed, in every op and
/// order**: the crossing layer has no lane for a line lying along a
/// cone's generator, so the reduction refuses before any record is
/// carried (`work/roots/a-line-parallel-to-a-cone-generator-has-no-root-lane.md`).
/// When that lane lands, this pose owes what the cylinder's do.
#[test]
fn a_prism_edge_along_a_cone_ruling_refuses_at_the_crossing_layer() {
    let (cone, prism) = cone_and_prism(0.7, 1.5);
    for (op, r, _) in six(&cone, &prism) {
        assert!(
            matches!(r, Err(BooleanError::CurvedPierceUnsupported { .. })),
            "cone: {op}: {:?}",
            r.map(|r| r.body().is_some())
        );
    }
}
