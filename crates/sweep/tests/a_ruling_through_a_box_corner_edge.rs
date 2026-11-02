//! **A wall's ruling along a box edge.** A cylinder whose wall passes
//! through an edge of a box, a ruling of the wall, with the box on the
//! wall's inner side there and crossing the wall further on: both box
//! faces at the edge meet the wall along the edge alone, so its two
//! ends rest on the wall's interior. The section divides the wall:
//!
//! - `∩` keeps the edge and the wall's far piece, and records nothing:
//!   the rest stays with the near piece, which `∩` drops.
//! - wall `∖` box keeps the edge and the near piece, the two wedges of
//!   wall material beside the box touching along the edge. The material
//!   parts the edge into two, a ring of no area in the near piece, which
//!   the measure of a curved face does not read: it refuses
//!   `EdgeInFace` (`CurvedFace`,
//!   `work/splitdoor/the-boolean-builds-the-slit-on-a-curved-face-once-it-is-meshed-and-measured.md`).
//! - `∪` and box `∖` wall lose the edge, and record nothing.
//!
//! A cone's ruling along a square prism's edge is the same pose on a
//! carrier whose rulings are not parallel, and refuses at the crossing
//! layer, before any record is carried.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_core::{Point2, Point3, RigidMotion, Tol, Vec3};
use profile::{Profile, SketchPlane};
use sweep::test_support::{brick, corners, finished, prism_on, revolved_about_y, sketch_from_axes};
use sweep::{ExtrudeSide, Extrusion, Revolution, extrude};
use topo::{AtRestBody, BooleanError, BooleanResult};

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
    fn frame(self, at: f64) -> RigidMotion<f64> {
        match self {
            Self::X => RigidMotion::axes_yz(Point3::new(at, 0.0, 0.0)),
            Self::Y => RigidMotion::axes_zx(Point3::new(0.0, at, 0.0)),
            Self::Z => RigidMotion::axes_xy(Point3::new(0.0, 0.0, at)),
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

    /// The world point at frame coordinates `(u, v)` and `w` along.
    fn world(self, u: f64, v: f64, w: f64) -> (f64, f64, f64) {
        match self {
            Self::X => (w, u, v),
            Self::Y => (v, w, u),
            Self::Z => (u, v, w),
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

/// One op's label and its result.
type Run = (&'static str, Result<BooleanResult<f64>, BooleanError>);

/// Every op of the wall `w` and the box `b` in both orders.
fn six(w: &AtRestBody<f64>, b: &AtRestBody<f64>) -> [Run; 6] {
    let t = tol();
    [
        ("w ∩ b", topo::intersect(w, b, t)),
        ("b ∩ w", topo::intersect(b, w, t)),
        ("w ∪ b", topo::union(w, b, t)),
        ("b ∪ w", topo::union(b, w, t)),
        ("w ∖ b", topo::subtract(w, b, t)),
        ("b ∖ w", topo::subtract(b, w, t)),
    ]
}

/// **`w ∖ b` refuses `EdgeInFace` (`CurvedFace`)**: the box's faces at
/// the corner edge both leave it into the near wall piece's material,
/// so the edge is two, a ring of no area in that piece
/// (`work/splitdoor/the-boolean-builds-the-slit-on-a-curved-face-once-it-is-meshed-and-measured.md`,
/// whose Fix lists what these rows owe back).
fn refuses_the_ring(what: &str, r: Result<BooleanResult<f64>, BooleanError>) {
    assert!(
        matches!(
            r,
            Err(BooleanError::EdgeInFace {
                gap: topo::EdgeInFaceGap::CurvedFace,
                ..
            })
        ),
        "{what}: refuses the parting on the curved wall: {:?}",
        r.map(|_| ())
    );
}

/// The result's volume, after checking it passes tiers 2 and 3′ against
/// its own records and carries none.
fn checked(what: &str, r: Result<BooleanResult<f64>, BooleanError>) -> f64 {
    let t = tol();
    let bb = match r {
        Ok(BooleanResult::Body(bb)) => bb,
        other => panic!("{what}: builds a body: {other:?}"),
    };
    topo::validate_closed(&bb.body).unwrap_or_else(|e| panic!("{what}: tier 2: {e:?}"));
    topo::validate_pseudomanifold(&bb.body, &bb.contacts, t)
        .unwrap_or_else(|e| panic!("{what}: tier 3′ against its records: {e:?}"));
    assert_eq!(bb.contacts.len(), 0, "{what}: no record: {:?}", bb.contacts);
    topo::test_support::pinned_volume("the measured body", &bb.body, t).volume
}

/// **A cylinder's ruling along each of the box's twelve edges**: every
/// op in both orders but `w ∖ b` builds at its closed form, passes tier
/// 3′ against its own records, and records nothing, `∩` naming no rest
/// on the far piece, which holds neither end; `w ∖ b` refuses the ring
/// its edge would part into ([`refuses_the_ring`]).
#[test]
fn a_wall_through_a_box_edge_builds_every_op_but_the_ring() {
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
                for (op, r) in six(&w, &b) {
                    let what = format!("{axis:?} edge at ({u}, {v}): {op}");
                    if op == "w ∖ b" {
                        refuses_the_ring(&what, r);
                        continue;
                    }
                    let want = match op {
                        "w ∩ b" | "b ∩ w" => common,
                        "w ∪ b" | "b ∪ w" => vw + vb - common,
                        "w ∖ b" => vw - common,
                        _ => vb - common,
                    };
                    let got = checked(&what, r);
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
/// When that lane lands, this pose owes what the cylinder's do: every op
/// but `w ∖ b` built, and `w ∖ b` refusing the ring
/// ([`refuses_the_ring`]) until the curved face measures it.
#[test]
fn a_prism_edge_along_a_cone_ruling_refuses_at_the_crossing_layer() {
    let (cone, prism) = cone_and_prism(0.7, 1.5);
    for (op, r) in six(&cone, &prism) {
        assert!(
            matches!(r, Err(BooleanError::CurvedPierceUnsupported { .. })),
            "cone: {op}: {:?}",
            r.map(|r| r.body().is_some())
        );
    }
}

/// The post at the box edge `(u, v)` along `axis` (`(su, sv)` into the
/// box): `u − 0.3·su` to `u + 0.1·su` across, `v + sv·[0.25, 0.35]`, over
/// the middle third along the axis. Fused to the box, it leaves the box
/// through the face at `u` and crosses the near wall piece, so that
/// piece carries a ring.
fn post(axis: Axis, (u, v): (f64, f64), (su, sv): (f64, f64)) -> AtRestBody<f64> {
    let l = axis.length();
    let a = axis.world(u - 0.3 * su, v + 0.25 * sv, l / 3.0);
    let b = axis.world(u + 0.1 * su, v + 0.35 * sv, 2.0 * l / 3.0);
    let span = |p: f64, q: f64| (p.min(q), p.max(q));
    finished(
        "the post",
        brick(span(a.0, b.0), span(a.1, b.1), span(a.2, b.2), tol()),
        tol(),
    )
}

/// **A post through the near wall piece** (the review r1 pose): wall
/// `∖` (box `∪` post), the post fused to the box and crossing the wall
/// beside the ruling, at each of the box's twelve edges. The section of
/// the post divides the wall again, and the corner edge still lies in
/// the near piece with both ends in its interior: it refuses the ring
/// as `w ∖ b` does ([`refuses_the_ring`]).
#[test]
fn a_post_through_the_near_wall_piece_refuses_the_ring() {
    let t = tol();
    let the_box = the_box();
    let sign = |at: f64, (lo, _): (f64, f64)| if at == lo { 1.0 } else { -1.0 };
    for axis in [Axis::X, Axis::Y, Axis::Z] {
        let [ru, rv] = axis.across();
        for u in [ru.0, ru.1] {
            for v in [rv.0, rv.1] {
                let s = (sign(u, ru), sign(v, rv));
                let w = wall_through(axis, (u, v), s);
                let what = format!("{axis:?} edge at ({u}, {v}): w ∖ (box ∪ post)");
                let b = match topo::union(&the_box, &post(axis, (u, v), s), t) {
                    Ok(BooleanResult::Body(bb)) => bb.body,
                    other => panic!("{what}: box ∪ post builds: {other:?}"),
                };
                refuses_the_ring(&what, topo::subtract(&w, &b, t));
            }
        }
    }
}
