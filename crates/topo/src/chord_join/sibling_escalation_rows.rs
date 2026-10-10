//! **Ring re-homing on a plane and on a cylinder's chart reads past an
//! escalated vertex** ([`super::ring_side`], [`super::chart_ring_side`]):
//! a bystander ring whose first vertex reads the run in the escalation
//! band says nothing there, and its next vertex decides, as the sphere
//! and cone readings do ([`super::first_decided`]). Each row sweeps the
//! first vertex's offset through the band, reads the rows where that
//! vertex alone escalates, and holds the ring's side to where its other
//! vertices lie — on both sides of the run. The ring has three vertices,
//! two of them decided, so the reading passes over the escalation to a
//! decided vertex that is not the last; that the decided vertices agree
//! is [`super::first_decided`]'s premise (the ring does not cross the
//! run), so which of them answers changes nothing here.

use super::*;
use crate::euler::{MefSite, MevSite};
use crate::test_support_fixtures::{CylFrame, cyl_wall_sheet, prism_z};
use geom_core::Tol;

pub(super) fn tol() -> Tol {
    Tol::witness()
}

pub(super) fn band() -> Band {
    Band::linear(tol()).unwrap()
}

/// Offsets from the band's zero threshold to ten times its escalation
/// threshold, geometrically.
fn offsets() -> impl Iterator<Item = f64> {
    let b = band();
    let (lo, hi) = (b.zero(), 10.0 * b.escalate());
    (0..=60).map(move |k| lo * (hi / lo).powf(f64::from(k) / 60.0))
}

/// An empty ring of `face` at `p`, bridged from the outer loop's first
/// vertex and the bridge killed; then a chain of struts from `p` through
/// `then`, so the ring's cycle starts at `p` and visits each point of
/// `then` in order (and back).
pub(super) fn ring_at(
    body: &mut Body<f64>,
    face: FaceKey,
    p: Point3<f64>,
    then: &[Point3<f64>],
) -> LoopKey {
    let outer = body.get_face(face).unwrap().outer;
    let LoopBoundary::Cycle { first } = body.get_loop(outer).unwrap().boundary else {
        panic!("outer is a cycle");
    };
    let u = body.half_edge_start_point(first).unwrap();
    let bridge = body
        .mev(
            MevSite::Fan {
                he1: first,
                he2: first,
            },
            p,
            EdgeCurveSpec::line_between(u, p),
            tol(),
        )
        .unwrap();
    let ring = body.kemr(bridge.he_plus, bridge.he_minus).unwrap().ring;
    let mut tip = None;
    for &q in then {
        let site = match tip {
            None => MevSite::Lone { r#loop: ring },
            Some(he) => MevSite::Fan { he1: he, he2: he },
        };
        tip = Some(body.mev_line(site, q, tol()).unwrap().he_minus);
    }
    if !then.is_empty() {
        let points: Vec<_> = ring_vertices(body, ring)
            .unwrap()
            .into_iter()
            .map(|v| vertex_point(body, v))
            .collect();
        let want: Vec<_> = core::iter::once(p)
            .chain(then.iter().copied())
            .chain(then.iter().rev().skip(1).copied())
            .collect();
        assert!(
            points.len() == want.len()
                && points
                    .iter()
                    .zip(&want)
                    .all(|(a, b)| (*a - *b).norm() == 0.0),
            "the ring's vertices, in order: {points:?}"
        );
    }
    ring
}

/// **A planar bystander whose first vertex reads a diagonal run in the
/// escalation band is re-homed by the others.** The slab's 2×2 top face
/// divided along its diagonal; the first vertex a hair off the diagonal,
/// then `(1.5, 0.5)` and `(1.7, 0.2)`, or their mirrors, each decided.
#[test]
fn a_planar_ring_vertex_escalating_hands_re_homing_to_the_next() {
    let mut escalated = [0; 2];
    for delta in offsets() {
        for (second, third) in [((1.5, 0.5), (1.7, 0.2)), ((0.5, 1.5), (0.2, 1.7))] {
            let p = prism_z::<f64>(
                &[(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)],
                0.0,
                1.0,
                tol(),
            );
            let (mut body, face, corners) = (p.body, p.top_face, p.top);
            let off = delta / 2f64.sqrt();
            let p0 = Point3::new(1.0 + off, 1.0 - off, 1.0);
            let p1 = Point3::new(second.0, second.1, 1.0);
            let p2 = Point3::new(third.0, third.1, 1.0);
            let ring = ring_at(&mut body, face, p0, &[p1, p2]);
            let outer = |v| {
                let outer = body.get_face(face).unwrap().outer;
                body.loop_cycle(match body.get_loop(outer).unwrap().boundary {
                    LoopBoundary::Cycle { first } => first,
                    LoopBoundary::Empty { .. } => unreachable!(),
                })
                .unwrap()
                .into_iter()
                .find(|&he| body.get_half_edge(he).unwrap().start == v)
                .unwrap()
            };
            let (he1, he2) = (outer(corners[0]), outer(corners[2]));
            let made = body.mef_chord(MefSite::Chords { he1, he2 }, tol()).unwrap();
            let run = body.get_face(made.face).unwrap().outer;
            let normal = Vec3::new(0.0, 0.0, 1.0);
            // The first vertex escalates against the run, alone.
            if !matches!(
                point_in_loop(&body, run, normal, p0, band()),
                Err(PointInLoopError::Escalated { .. })
            ) {
                continue;
            }
            let inside =
                point_in_loop(&body, run, normal, p1, band()).unwrap() == LoopContainment::In;
            assert_eq!(
                point_in_loop(&body, run, normal, p2, band()).unwrap(),
                if inside {
                    LoopContainment::In
                } else {
                    LoopContainment::Out
                },
                "the third vertex is decided, on the second's side"
            );
            escalated[usize::from(inside)] += 1;
            let side = ring_side(&body, ring, run, normal, band()).unwrap();
            assert_eq!(
                side,
                if inside { RingSide::In } else { RingSide::Out },
                "offset {delta:e}, second vertex {second:?}"
            );
        }
    }
    assert!(
        escalated.iter().all(|&n| n > 0),
        "escalated, out and in: {escalated:?}"
    );
}

/// A circle arc of the unit cylinder about `+z` at height `v`, from
/// azimuth `u0` to `u1` (either way round), described as the cylinder cut
/// by the plane at that height.
pub(super) fn rim_arc(
    body: &mut Body<f64>,
    cyl: SurfaceKey,
    v: f64,
    (u0, u1): (f64, f64),
) -> EdgeCurveSpec<f64> {
    let centre = Point3::new(0.0, 0.0, v);
    let plane = body.add_surface(geom::Surface::Plane {
        origin: centre,
        normal: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    });
    let circle = geom::Curve3::Circle {
        center: centre,
        axis: Vec3::unit_z(),
        radius: 1.0,
        u_ref: Vec3::unit_x(),
    };
    let (carrier, t0, t1) = if u1 > u0 {
        (circle, u0, u1)
    } else {
        (circle.reversed().unwrap(), -u0, -u1)
    };
    EdgeCurveSpec {
        description: geom_brep::EdgeDescriptionSpec::Intersection {
            s1: cyl,
            s2: plane,
            witness: carrier.mid_point(t0, t1),
        },
        carrier,
        param_start: t0,
        param_end: t1,
    }
}

/// **A bystander on a cylinder whose first vertex's ray reads the run in
/// the escalation band is re-homed by its second.** The unit wall
/// `[0.2, 1.4] × [0, 1]` holding the island `[0.5, 1] × [0.3, 0.7]`,
/// walled off by its closing ruling at 0.5; the first vertex at height
/// 0.5 a hair past azimuth 0.5, where its ray runs along the island's
/// ruling, then two at azimuths 0.75 and 0.85 (inside) or 1.2 and 1.3
/// (outside), each decided.
#[test]
fn a_chart_ring_vertex_escalating_hands_re_homing_to_the_next() {
    let at = |u: f64, v: f64| Point3::new(u.cos(), u.sin(), v);
    let mut escalated = [0; 2];
    for delta in offsets() {
        for second in [0.75, 1.2] {
            let mut body = Body::<f64>::new();
            let face = cyl_wall_sheet(
                &mut body,
                CylFrame::canonical(1.0),
                (0.2, 1.4),
                (0.0, 1.0),
                tol(),
            );
            let cyl = body.get_face(face).unwrap().surface;
            let surface = body.get_surface(cyl).unwrap().clone();
            let (p0, p1, p2) = (
                at(0.5 + delta, 0.5),
                at(second, 0.5),
                at(second + 0.1, 0.45),
            );
            let ring = ring_at(&mut body, face, p0, &[p1, p2]);
            // The island's run: the low arc, the ruling at 1, the high arc.
            let corners = [at(0.5, 0.3), at(1.0, 0.3), at(1.0, 0.7), at(0.5, 0.7)];
            let island = ring_at(&mut body, face, corners[0], &[]);
            let low = rim_arc(&mut body, cyl, 0.3, (0.5, 1.0));
            let e0 = body
                .mev(MevSite::Lone { r#loop: island }, corners[1], low, tol())
                .unwrap();
            let e1 = body
                .mev_line(
                    MevSite::Fan {
                        he1: e0.he_minus,
                        he2: e0.he_minus,
                    },
                    corners[2],
                    tol(),
                )
                .unwrap();
            let high = rim_arc(&mut body, cyl, 0.7, (1.0, 0.5));
            let e2 = body
                .mev(
                    MevSite::Fan {
                        he1: e1.he_minus,
                        he2: e1.he_minus,
                    },
                    corners[3],
                    high,
                    tol(),
                )
                .unwrap();
            let after = body.get_half_edge(e2.he_plus).unwrap().next;
            let made = body
                .mef_chord(
                    MefSite::Chords {
                        he1: e0.he_plus,
                        he2: after,
                    },
                    tol(),
                )
                .unwrap();
            // The first vertex alone escalates.
            let lone = ring_at(&mut body, face, p0, &[]);
            if !matches!(
                chart_ring_side(&body, &surface, made.face, lone, band()),
                Err(SplitJoinError::Escalated { .. })
            ) {
                continue;
            }
            let inside = second < 1.0;
            for q in [p1, p2] {
                let alone = ring_at(&mut body, face, q, &[]);
                assert_eq!(
                    chart_ring_side(&body, &surface, made.face, alone, band()).unwrap(),
                    if inside { RingSide::In } else { RingSide::Out },
                    "each later vertex is decided, on one side"
                );
            }
            escalated[usize::from(inside)] += 1;
            let side = chart_ring_side(&body, &surface, made.face, ring, band()).unwrap();
            assert_eq!(
                side,
                if inside { RingSide::In } else { RingSide::Out },
                "offset {delta:e}, second vertex at {second}"
            );
        }
    }
    assert!(
        escalated.iter().all(|&n| n > 0),
        "escalated, out and in: {escalated:?}"
    );
}
