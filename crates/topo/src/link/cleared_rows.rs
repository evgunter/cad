//! **A planar face with an edge that is not a line, read as the disc
//! its other edges clear** ([`Sector::cleared`], [`clearance`]): the
//! disc's radius, reach and side on a half-disc cap, a hole nearer than
//! the outer loop, the curve clearance as a lower bound, and
//! [`rest::judge`] over cleared sectors only, where an edge lying in a
//! face of its own solid rests with its wedge outside the face's
//! material, crosses as a keel whose wedge holds the face, and refuses
//! in band with one of its faces tilted into the band off the face —
//! read at the faces' reach, not at the radius their edges clear.

#![allow(clippy::expect_used, clippy::panic, clippy::unwrap_used)]

use geom::Curve3;
use geom_core::Tol;

use super::*;

fn band() -> Band {
    Band::linear(Tol::witness()).expect("the witness tolerance is a band")
}

fn p(x: f64, y: f64, z: f64) -> Point3<f64> {
    Point3::new(x, y, z)
}

fn v(x: f64, y: f64, z: f64) -> Vec3<f64> {
    Vec3::new(x, y, z)
}

/// A half-disc cap ([`crate::test_support_fixtures::arc_cap`]) on the
/// unit circle about `center` in the plane normal to `axis`, on the
/// `axis × u_ref` side of its chord from `center + u_ref` to
/// `center − u_ref`. The cap's loop runs that chord and then the arc
/// back, clockwise about `axis`, so as a face of a solid its sense is
/// reversed, and its outward normal, read off the body, is `−axis`.
struct Cap {
    body: Body<f64>,
    face: FaceKey,
    outward: Vec3<f64>,
}

fn cap(center: Point3<f64>, axis: Vec3<f64>, u_ref: Vec3<f64>) -> Cap {
    let carrier = Curve3::Circle {
        center,
        axis,
        radius: 1.0,
        u_ref,
    };
    let (mut body, face) = crate::test_support_fixtures::arc_cap(
        carrier,
        (0.0, core::f64::consts::PI),
        Vec3::new(0.0, 0.0, 0.0),
        Tol::witness(),
    );
    body.get_face_mut(face).unwrap().sense = false;
    let outward = crate::face_normal::face_outward_normal(&body, face)
        .expect("a planar face's outward normal")
        .vec();
    Cap {
        body,
        face,
        outward,
    }
}

impl Cap {
    /// The cap's sectors at `p`, held as `holds` say.
    fn cleared(&self, p: Point3<f64>, holds: &[Hold]) -> Result<Vec<Sector<f64>>, TouchUnread> {
        Sector::cleared(&self.body, self.face, self.outward, p, holds)
    }

    /// The hold inside the cap's chord.
    fn chord(&self) -> Hold {
        let ends: Vec<VertexKey> = self.body.vertices.keys().collect();
        assert_eq!(ends.len(), 2, "a cap has its chord's two corners");
        Hold::Edge(ends[0], ends[1])
    }
}

/// Every point of the sector's fan.
fn fan(s: &Sector<f64>) -> Vec<Point3<f64>> {
    s.wedges.iter().flat_map(|&(a, b)| [a, b]).collect()
}

/// The cap's boundary, walked: its chord and its arc, 2 000 points each.
fn boundary() -> Vec<Point3<f64>> {
    let (a, b) = (p(1.0, 0.0, 0.0), p(-1.0, 0.0, 0.0));
    (0..=2_000)
        .flat_map(|k| {
            let t = f64::from(k) / 2_000.0;
            let (s, co) = (core::f64::consts::PI * t).sin_cos();
            [a + (b - a) * t, p(co, s, 0.0)]
        })
        .collect()
}

/// **A cleared sector is the disc its other edges clear, on the face's
/// side, carried past the face.** On the half-disc cap over the unit
/// circle about the origin in `z = 0`, its chord along `x`, its outward
/// normal `−z` read off the body: inside it at `(0.1, 0.3)` the chord is
/// nearer than the arc, so the disc's radius is `0.3`, closed, eight
/// wedges; inside the chord at `(0.2, 0)` the arc alone stands off it,
/// `0.8` away, and the half-disc lies on the face's side `y ≥ 0`, four
/// wedges from `−x` round to `+x`. Each fan's every chord stands
/// further off than every point of the face's boundary. A corner hold
/// refuses, and so does a point on the arc, whose disc has no radius.
/// Red if the radius takes in the held edge, the half-disc turns off the
/// face, or the fan stops short of the face's reach.
#[test]
fn a_cleared_sector_is_the_disc_its_other_edges_clear_on_the_faces_side() {
    let c = cap(Point3::origin(), v(0.0, 0.0, 1.0), v(1.0, 0.0, 0.0));
    assert!(
        (c.outward - v(0.0, 0.0, -1.0)).norm() < 1e-12,
        "the body's outward normal: {:?}",
        c.outward
    );
    let walk = boundary();
    let past_the_face = |s: &Sector<f64>, at: Point3<f64>| {
        let far = walk.iter().map(|&q| (q - at).norm()).fold(0.0, f64::max);
        for &w in &s.wedges {
            let near = to_segment(at, w);
            assert!(near > far, "a chord at {near} within the face's {far}");
        }
    };
    let at = p(0.1, 0.3, 0.0);
    let inside = c
        .cleared(at, &[Hold::Inside])
        .expect("the disc inside the cap");
    assert_eq!(inside.len(), 1, "one sector inside the face");
    let s = &inside[0];
    assert!(s.closed, "the disc closes about the point");
    assert_eq!(s.wedges.len(), 8, "eight wedges of an eighth of a turn");
    let r = s.clear.expect("a cleared radius");
    assert!((r - 0.3).abs() < 1e-12, "the chord clears: {r}");
    past_the_face(s, at);
    let at = p(0.2, 0.0, 0.0);
    let on_chord = c
        .cleared(at, &[c.chord()])
        .expect("the half-disc inside the chord");
    assert_eq!(on_chord.len(), 1, "the chord's one pass");
    let s = &on_chord[0];
    assert!(!s.closed, "a half-disc is bounded by the chord");
    assert_eq!(s.wedges.len(), 4, "four wedges of an eighth of a turn");
    let r = s.clear.expect("a cleared radius");
    assert!((r - 0.8).abs() < 1e-12, "the arc clears: {r}");
    past_the_face(s, at);
    for q in fan(s) {
        assert!(q.y > -1e-12, "on the face's side of the chord: {q:?}");
    }
    let rim = (s.wedges[0].0 - at).norm();
    let (first, last) = (s.wedges[0].0, s.wedges[3].1);
    assert!(
        (first - (at - v(rim, 0.0, 0.0))).norm() < 1e-12
            && (last - (at + v(rim, 0.0, 0.0))).norm() < 1e-12,
        "the bounds run along the chord, counterclockwise about the outward normal: \
         {first:?} to {last:?}"
    );
    let corner = match c.chord() {
        Hold::Edge(a, _) => Hold::Corner(a),
        _ => unreachable!(),
    };
    assert_eq!(
        c.cleared(p(1.0, 0.0, 0.0), &[corner]).err(),
        Some(TouchUnread::Unreadable),
        "a corner of a face with an arc edge is not a disc"
    );
    assert_eq!(
        c.cleared(p(0.0, 1.0, 0.0), &[Hold::Inside]).err(),
        Some(TouchUnread::ChordScale),
        "on the arc the disc has no radius"
    );
}

/// **Every loop of the face bounds the disc, not only the outer one.**
/// The top of the brick over `[0, 2]² × [0, 1]` with a disc of radius
/// `0.5` about `(1, 1)` planted in it: the top face's outer loop is four
/// lines, and its circle is a ring. At `(1, 1.6, 1)`, inside the top
/// face, the circle stands `0.1` off and the outer loop `0.4`, so the
/// disc's radius is `0.1`. Red if a loop other than the outer one is
/// left out of the clearance.
#[test]
fn a_ring_nearer_than_the_outer_loop_bounds_the_disc() {
    use crate::test_support_fixtures::{brick, plant_disc_face};
    let tol = Tol::witness();
    let mut body = brick::<f64>((0.0, 2.0), (0.0, 2.0), (0.0, 1.0), tol);
    let top = body
        .faces()
        .find(|(k, _)| {
            crate::face_normal::face_outward_normal(&body, *k).is_some_and(|n| n.vec().z > 0.5)
        })
        .map(|(k, _)| k)
        .expect("the brick's top");
    let at = body
        .half_edges()
        .find(|(_, h)| body.get_loop(h.parent_loop).unwrap().face == top)
        .map(|(k, _)| k)
        .unwrap();
    plant_disc_face(&mut body, at, p(1.0, 1.0, 1.0), 0.5, tol);
    let outward = crate::face_normal::face_outward_normal(&body, top)
        .unwrap()
        .vec();
    let q = p(1.0, 1.6, 1.0);
    let s = Sector::cleared(&body, top, outward, q, &[Hold::Inside]).expect("the top's disc");
    assert_eq!(s.len(), 1, "one sector inside the face");
    let r = s[0].clear.expect("a cleared radius");
    assert!(
        (r - 0.1).abs() < 1e-12,
        "the ring clears, not the outer loop: {r}"
    );
}

/// **A curve's clearance is a lower bound on the distance from its whole
/// carrier**, as [`clearance`] reads it ([`geom::Curve3::clearance`]).
/// Over two ellipses, one long along `u_ref` and one along
/// `axis × u_ref`, tilted off the axes and off the origin, a table of
/// points in the plane and off it, inside and outside: never more than
/// the least distance to a walk of 20 000 points round the whole
/// ellipse. On a 1000 × 0.001 ellipse, `0.1` past its major vertex, it
/// reads more than the `1e-7` the contraction onto the minor circle
/// would. Red if the bound overstates, or falls back to that
/// contraction.
#[test]
fn a_curves_clearance_is_a_lower_bound_on_its_carriers_distance() {
    let axis = v(0.2, -0.3, 0.9).normalize();
    let u_ref = axis.cross(v(1.0, 0.0, 0.0)).normalize();
    let w = axis.cross(u_ref);
    let center = p(0.4, -1.1, 2.3);
    for (semi_u, semi_v) in [(2.0, 0.7), (0.5, 1.6)] {
        let e = Curve3::Ellipse {
            center,
            axis,
            semi_u,
            semi_v,
            u_ref,
        };
        let walk: Vec<Point3<f64>> = (0..20_000)
            .map(|k| {
                let t = core::f64::consts::TAU * f64::from(k) / 20_000.0;
                center + u_ref * (semi_u * t.cos()) + w * (semi_v * t.sin())
            })
            .collect();
        for a in [-2.5, -1.0, -0.3, 0.0, 0.6, 1.9, 3.0] {
            for b in [-2.0, -0.4, 0.0, 0.5, 1.7] {
                for h in [0.0, -0.8, 1.3] {
                    let q = center + u_ref * a + w * b + axis * h;
                    let bound = e.clearance(q).expect("an ellipse reads");
                    let least = walk
                        .iter()
                        .map(|&x| (x - q).norm())
                        .fold(f64::MAX, f64::min);
                    assert!(
                        (0.0..=least + 1e-9).contains(&bound),
                        "{semi_u}×{semi_v} at ({a}, {b}, {h}): {bound} against {least}"
                    );
                }
            }
        }
    }
    let long = Curve3::Ellipse {
        center: Point3::origin(),
        axis: v(0.0, 0.0, 1.0),
        semi_u: 1000.0,
        semi_v: 1e-3,
        u_ref: v(1.0, 0.0, 0.0),
    };
    let bound = long.clearance(p(1000.1, 0.0, 0.0)).unwrap();
    assert!(
        bound > 1e-6 && bound <= 0.1 + 1e-9,
        "past the major vertex: {bound}"
    );
}

/// An edge's two faces, half-disc caps about the line `x` through the
/// origin, and the point `at` inside their chord: `a` toward the angle
/// `alpha` about `x` from `+y`, its material turning counterclockwise
/// from it, and `b` toward `beta`, its material turning clockwise, so
/// the edge's wedge runs from `alpha` counterclockwise to `beta`;
/// judged against the cap in `z = 0` whose chord is `y = face_chord`,
/// holding `at` inside it, its outward normal `+z`. Each link is of
/// cleared sectors only.
fn judged_at(alpha: f64, beta: f64, at: Point3<f64>, face_chord: Point3<f64>) -> TouchVerdict {
    let (sa, ca) = alpha.sin_cos();
    let (sb, cb) = beta.sin_cos();
    let a = cap(Point3::origin(), v(0.0, -sa, ca), v(1.0, 0.0, 0.0));
    let b = cap(Point3::origin(), v(0.0, sb, -cb), v(-1.0, 0.0, 0.0));
    let face = cap(face_chord, v(0.0, 0.0, -1.0), v(-1.0, 0.0, 0.0));
    let mut edge = a.cleared(at, &[a.chord()]).expect("a's half-disc");
    edge.extend(b.cleared(at, &[b.chord()]).expect("b's half-disc"));
    let edge = Link::of(at, edge).expect("the edge's link");
    let face = Link::of(
        at,
        face.cleared(at, &[Hold::Inside]).expect("the face's disc"),
    )
    .expect("the face's link");
    rest::judge(&edge, &face, band())
}

/// [`judged_at`] at `(0.1, 0, 0)`, against the face whose chord is
/// `y = −0.5`.
fn judged(alpha: f64, beta: f64) -> TouchVerdict {
    judged_at(alpha, beta, p(0.1, 0.0, 0.0), p(0.0, -0.5, 0.0))
}

/// **An edge lying in a face of its own solid, every sheet a cleared
/// disc, reads its side** (D1 tier 3, `crates/topo/README.md`: the
/// material sectors on one line are disjoint). Its wedge from 60° to
/// 120° about the line stands above the face, outside the face's
/// material: a rest. A keel, its wedge from −60° round to 240°, holds the
/// face's sheet and a part of its material: a crossing. A wedge from 60°
/// to a band past 180° lays its face a band into the face's material, at
/// the geometric mean of the band's zero and escalation readings at the
/// fan's rim, past the face: in band. Red if a cleared sheet reads a
/// side the cone does not hold, or a reading in band decides.
#[test]
fn an_edge_in_a_face_of_cleared_sheets_rests_crosses_or_refuses_in_band() {
    use core::f64::consts::PI;
    let deg = |d: f64| d.to_radians();
    assert!(
        matches!(judged(deg(60.0), deg(120.0)), TouchVerdict::Rest(_)),
        "the wedge above the face rests"
    );
    assert_eq!(
        judged(deg(-60.0), deg(240.0)),
        TouchVerdict::Crossing,
        "the keel's wedge holds the face"
    );
    let at = p(0.1, 0.0, 0.0);
    let b = cap(Point3::origin(), v(0.0, 0.0, 1.0), v(-1.0, 0.0, 0.0));
    let rim = (b.cleared(at, &[b.chord()]).unwrap()[0].wedges[0].0 - at).norm();
    let band = band();
    let tilt = ((band.zero() * band.escalate()).sqrt() / rim).asin();
    assert_eq!(
        judged(deg(60.0), PI + tilt),
        TouchVerdict::Unread(TouchUnread::InBand),
        "a face a band into the face's material"
    );
}

/// **A cleared sheet is read at the face's reach, not at the radius its
/// edges clear.** The point `(1 − d, 0, 0)`, `d = 1e-4`, stands `d` off
/// the arcs of the edge's two caps and of the face's: every disc's
/// radius is about `d`, while each face reaches about a metre. A keel
/// tilted `t` below the face's plane on both sides, `t` from 2 to 500
/// times the band's escalation, dips its face `b` about `t` metres into
/// the face's material inside both faces, past the band, while at the
/// cleared radius it dips under the band's zero. Red if any such keel
/// rests.
#[test]
fn a_keel_off_the_arcs_is_read_at_the_faces_reach_not_the_cleared_radius() {
    use core::f64::consts::PI;
    let d = 1e-4;
    let at = p(1.0 - d, 0.0, 0.0);
    let band = band();
    for k in [2.0, 10.0, 100.0, 500.0] {
        let t = k * band.escalate();
        assert!(t * d < band.zero(), "under the band at the cleared radius");
        let got = judged_at(-t, PI + t, at, p(1.0 - d, -(1.0 - d), 0.0));
        assert!(
            !matches!(got, TouchVerdict::Rest(_)),
            "a keel tilted {t} rests: {got:?}"
        );
    }
}

/// A circular-segment cap: the arc of the unit circle over
/// `(π/2 − δ, π/2 + δ)` closed by its chord through `chord_mid` along
/// `u_ref`, bulging `1 − cos δ` toward `axis × u_ref`.
fn sliver(chord_mid: Point3<f64>, axis: Vec3<f64>, u_ref: Vec3<f64>, delta: f64) -> Cap {
    let w = axis.cross(u_ref);
    let carrier = Curve3::Circle {
        center: chord_mid - w * delta.cos(),
        axis,
        radius: 1.0,
        u_ref,
    };
    let (mut body, face) = crate::test_support_fixtures::arc_cap(
        carrier,
        (
            core::f64::consts::FRAC_PI_2 - delta,
            core::f64::consts::FRAC_PI_2 + delta,
        ),
        Vec3::new(0.0, 0.0, 0.0),
        Tol::witness(),
    );
    body.get_face_mut(face).unwrap().sense = false;
    let outward = crate::face_normal::face_outward_normal(&body, face)
        .expect("a planar face's outward normal")
        .vec();
    Cap {
        body,
        face,
        outward,
    }
}

/// **A crossing read on a cleared rim is confirmed only inside the
/// radius its edges clear.** A keel of two sliver caps, `δ = 1e-3`, each
/// reaching only `1 − cos δ ≈ 5e-7` off the edge's line, tilted `t` below
/// the face of the unit half-disc whose chord is `y = −0.5`: the real
/// overlap is about `t·5e-7`, under the band's zero, while on the
/// slivers' rims, about `1e-3` out, the dip `t·1e-3` passes the band's
/// escalation. `t` runs from `2·10⁴` to `5·10⁵` times the band's zero, so
/// both hold at every ε row. The rims there are not the slivers' points,
/// and the ball is about `5e-7`, where the dip is in band: in band, never
/// a crossing. Red if the link's ball is not held to the cleared radius.
#[test]
fn a_crossing_on_a_sliver_keels_rim_is_confirmed_only_inside_its_cleared_radius() {
    use core::f64::consts::PI;
    let at = Point3::origin();
    let delta = 1e-3;
    let bulge = 1.0 - delta.cos();
    let band = band();
    for k in [2e4, 1e5, 5e5] {
        let t = k * band.zero();
        assert!(
            t * bulge < band.zero() && t * delta > band.escalate(),
            "the premise: no real overlap, a decided dip on the rim, at {t}"
        );
        let (sa, ca) = (-t).sin_cos();
        let (sb, cb) = (PI + t).sin_cos();
        let a = sliver(at, v(0.0, -sa, ca), v(1.0, 0.0, 0.0), delta);
        let b = sliver(at, v(0.0, sb, -cb), v(-1.0, 0.0, 0.0), delta);
        let face = cap(p(0.0, -0.5, 0.0), v(0.0, 0.0, -1.0), v(-1.0, 0.0, 0.0));
        let mut edge = a.cleared(at, &[a.chord()]).expect("a's half-disc");
        edge.extend(b.cleared(at, &[b.chord()]).expect("b's half-disc"));
        let edge = Link::of(at, edge).expect("the edge's link");
        let face = Link::of(
            at,
            face.cleared(at, &[Hold::Inside]).expect("the face's disc"),
        )
        .expect("the face's link");
        assert_eq!(
            rest::judge(&edge, &face, band),
            TouchVerdict::Unread(TouchUnread::InBand),
            "a sliver keel tilted {t}"
        );
    }
}
