//! Review r1 probe (PR 4650): the ruling-through-a-box-edge pose with a
//! post fused to the box that pierces the wall's near piece, so the
//! wall's near piece carries a ring and the curved trim reads it as
//! unreadable (`curved_face_containment` → `None` on a ringed
//! non-sphere face). Asks whether `Descendants::holder`'s unread
//! fallback drops a rest silently when the wall's key sits on a
//! discarded piece.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Band, OrthoFrame, Point2, Point3, Tol};
use profile::{Profile, SketchPlane};
use sweep::test_support::{brick, finished};
use sweep::{ExtrudeSide, Extrusion, extrude};
use topo::{AtRestBody, BooleanResult, ContactKind, FaceContainment};

const C: f64 = 0.3;

fn tol() -> Tol {
    Tol::witness()
}

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
    /// World point of frame coordinates `(u, v)` and `w` along.
    fn world(self, u: f64, v: f64, w: f64) -> [f64; 3] {
        match self {
            Self::X => [w, u, v],
            Self::Y => [v, w, u],
            Self::Z => [u, v, w],
        }
    }
}

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
    let body = extrude(
        &p,
        Extrusion::Distance {
            depth,
            side: ExtrudeSide::Along,
        },
        t,
    )
    .unwrap()
    .body;
    finished("the wall", body, t)
}

fn sorted(a: f64, b: f64) -> (f64, f64) {
    if a < b { (a, b) } else { (b, a) }
}

/// The post: frame `u` from `u − 0.3·su` to `u + 0.1·su`, `v` over
/// `v + sv·[0.25, 0.35]`, the middle third along the axis. It crosses
/// the wall once outside the box and fuses into the box.
fn post(axis: Axis, (u, v): (f64, f64), (su, sv): (f64, f64)) -> AtRestBody<f64> {
    let l = axis.length();
    let a = axis.world(u - 0.3 * su, v + 0.25 * sv, l / 3.0);
    let b = axis.world(u + 0.1 * su, v + 0.35 * sv, 2.0 * l / 3.0);
    finished(
        "the post",
        brick(
            sorted(a[0], b[0]),
            sorted(a[1], b[1]),
            sorted(a[2], b[2]),
            tol(),
        ),
        tol(),
    )
}

#[test]
#[ignore = "review probe"]
fn review_r1_a_ringed_near_piece_and_the_unread_fallback() {
    let t = tol();
    let band = Band::linear(t).unwrap();
    let the_box = finished("the box", brick((0.0, 2.0), (0.0, 2.0), (0.0, 1.0), t), t);
    let sign = |at: f64, (lo, _): (f64, f64)| if at == lo { 1.0 } else { -1.0 };
    for axis in [Axis::X, Axis::Y, Axis::Z] {
        let [ru, rv] = axis.across();
        for u in [ru.0, ru.1] {
            for v in [rv.0, rv.1] {
                let s = (sign(u, ru), sign(v, rv));
                let w = wall_through(axis, (u, v), s);
                let b = match topo::union(&the_box, &post(axis, (u, v), s), t) {
                    Ok(BooleanResult::Body(bb)) => bb.body,
                    other => {
                        println!("{axis:?} ({u}, {v}): box ∪ post {other:?}");
                        continue;
                    }
                };
                let r = topo::subtract(&w, &b, t);
                let bb = match r {
                    Ok(BooleanResult::Body(bb)) => bb,
                    other => {
                        println!("{axis:?} ({u}, {v}): w ∖ B {:?}", other.map(|_| ()));
                        continue;
                    }
                };
                let vf: Vec<_> = bb
                    .contacts
                    .kinds()
                    .filter_map(|k| match k {
                        ContactKind::VertexFace(v, f) => Some((v, f)),
                        _ => None,
                    })
                    .collect();
                // The corner edge's two ends, and what each curved face
                // of the result reads at them.
                let mut reads = Vec::new();
                for wa in [0.0, axis.length()] {
                    let e = axis.world(u, v, wa);
                    let q = Point3::new(e[0], e[1], e[2]);
                    let live = bb.body.vertex_points().any(|(_, p)| (p - q).norm() < 1e-9);
                    let mut at = Vec::new();
                    for (fk, f) in bb.body.faces() {
                        let curved = !matches!(
                            bb.body.get_surface(f.surface),
                            Some(geom::Surface::Plane { .. })
                        );
                        if curved {
                            let c = topo::curved_face_containment(&bb.body, fk, q, band);
                            if !matches!(c, Ok(Some(FaceContainment::Out))) {
                                at.push(format!("{c:?} rings={}", f.rings.len()));
                            }
                        }
                    }
                    reads.push(format!("end w={wa} live={live} faces={at:?}"));
                }
                let t3 = topo::validate_pseudomanifold(&bb.body, &bb.contacts, t);
                println!(
                    "{axis:?} ({u}, {v}): w ∖ B vf={} contacts={} t3p={} {reads:?}",
                    vf.len(),
                    bb.contacts.len(),
                    t3.is_ok()
                );
            }
        }
    }
}
