//! **A planar face with an edge that is not a line, read as the disc
//! its other edges clear** ([`Sector::cleared`], [`clearance`]): the
//! disc's radius and side on a half-disc cap, an ellipse's clearance
//! against its closed-form nearest points, and [`rest::judge`] over
//! cleared sectors only, where an edge lying in a face of its own solid
//! rests with its wedge outside the face's material, crosses as a keel
//! whose wedge holds the face, and refuses in band with one of its faces
//! tilted into the band off the face.

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
/// `center − u_ref`. The cap's loop runs that chord and then the arc back,
/// clockwise about `axis`, so its outward normal, which a face's loop
/// turns counterclockwise about, is `−axis`.
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
    let (body, face) = crate::test_support_fixtures::arc_cap(
        carrier,
        (0.0, core::f64::consts::PI),
        Vec3::new(0.0, 0.0, 0.0),
        Tol::witness(),
    );
    Cap {
        body,
        face,
        outward: axis * -1.0,
    }
}

impl Cap {
    /// The cap's sectors at `p`, held as `holds` say.
    fn cleared(&self, p: Point3<f64>, holds: &[Hold]) -> Result<Vec<Sector<f64>>, TouchUnread> {
        let face = self.body.get_face(self.face).unwrap();
        let surface = self.body.get_surface(face.surface).cloned().unwrap();
        Sector::cleared(&self.body, face, surface, self.outward, p, holds)
    }

    /// The hold inside the cap's chord.
    fn chord(&self) -> Hold {
        let ends: Vec<VertexKey> = self.body.vertices.keys().collect();
        assert_eq!(ends.len(), 2, "a cap has its chord's two corners");
        Hold::Edge(ends[0], ends[1])
    }
}

/// Every point of the sector's fan, and its apex's distance to each.
fn fan(s: &Sector<f64>) -> Vec<Point3<f64>> {
    s.wedges.iter().flat_map(|&(a, b)| [a, b]).collect()
}

/// **A cleared sector is the disc its other edges clear, on the face's
/// side.** On the half-disc cap over the unit circle about the origin
/// in `z = 0`, its chord along `x`: inside it at `(0.1, 0.3)` the chord
/// is nearer than the arc, so the disc's radius is `0.3`, closed, eight
/// wedges; inside the chord at `(0.2, 0)` the arc alone stands off it,
/// `0.8` away, and the half-disc lies on the face's side `y ≥ 0`, four
/// wedges from `−x` round to `+x`, counterclockwise about the outward
/// normal `−z`. A corner hold refuses. Red if the
/// radius takes in the held edge, or the half-disc turns off the face.
#[test]
fn a_cleared_sector_is_the_disc_its_other_edges_clear_on_the_faces_side() {
    let c = cap(Point3::origin(), v(0.0, 0.0, 1.0), v(1.0, 0.0, 0.0));
    let at = p(0.1, 0.3, 0.0);
    let inside = c
        .cleared(at, &[Hold::Inside])
        .expect("the disc inside the cap");
    assert_eq!(inside.len(), 1, "one sector inside the face");
    let s = &inside[0];
    assert!(s.closed, "the disc closes about the point");
    assert_eq!(s.wedges.len(), 8, "eight wedges of an eighth of a turn");
    for q in fan(s) {
        assert!(
            ((q - at).norm() - 0.3).abs() < 1e-12,
            "on the rim the chord clears: {q:?}"
        );
    }
    let at = p(0.2, 0.0, 0.0);
    let on_chord = c
        .cleared(at, &[c.chord()])
        .expect("the half-disc inside the chord");
    assert_eq!(on_chord.len(), 1, "the chord's one pass");
    let s = &on_chord[0];
    assert!(!s.closed, "a half-disc is bounded by the chord");
    assert_eq!(s.wedges.len(), 4, "four wedges of an eighth of a turn");
    for q in fan(s) {
        assert!(
            ((q - at).norm() - 0.8).abs() < 1e-12,
            "on the rim the arc clears: {q:?}"
        );
        assert!(q.y > -1e-12, "on the face's side of the chord: {q:?}");
    }
    let (first, last) = (s.wedges[0].0, s.wedges[3].1);
    assert!(
        (first - p(-0.6, 0.0, 0.0)).norm() < 1e-12 && (last - p(1.0, 0.0, 0.0)).norm() < 1e-12,
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
}

/// **An ellipse's clearance is a lower bound, exact on its minor
/// axis.** Over two ellipses, one long along `u_ref` and one along
/// `axis × u_ref`, tilted off the axes and off the origin, a table of
/// points in the plane and off it, inside and outside: the clearance is
/// never more than the least distance to a walk of 20 000 points round
/// the whole ellipse, and on the minor axis's line beyond the minor
/// vertex it is the distance to that vertex. Red if the shrinking map
/// stretches, or reads the axes the wrong way round.
#[test]
fn an_ellipses_clearance_is_a_lower_bound_exact_on_its_minor_axis() {
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
                    let bound = conic_clearance(&e, q).expect("an ellipse reads");
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
        let (minor, across) = if semi_u < semi_v {
            (semi_u, u_ref)
        } else {
            (semi_v, w)
        };
        let q = center + across * (minor + 0.3);
        let bound = conic_clearance(&e, q).expect("an ellipse reads");
        assert!(
            (bound - 0.3).abs() < 1e-12,
            "{semi_u}×{semi_v}: {bound} beyond the minor vertex"
        );
    }
}

/// The half-disc caps about the line `x`, through `p = (0.1, 0, 0)`, an
/// edge's two faces: `a` toward the angle `alpha` about `x` from `+y`,
/// its material turning counterclockwise from it, and `b` toward
/// `beta`, its material turning clockwise, so the edge's wedge runs from
/// `alpha` counterclockwise to `beta`; judged against the cap in `z = 0`
/// whose chord is `y = −0.5`, holding `p` inside it, its outward normal
/// `+z`.
fn judged(alpha: f64, beta: f64) -> TouchVerdict {
    let at = p(0.1, 0.0, 0.0);
    let (sa, ca) = alpha.sin_cos();
    let (sb, cb) = beta.sin_cos();
    let a = cap(Point3::origin(), v(0.0, -sa, ca), v(1.0, 0.0, 0.0));
    let b = cap(Point3::origin(), v(0.0, sb, -cb), v(-1.0, 0.0, 0.0));
    let face = cap(p(0.0, -0.5, 0.0), v(0.0, 0.0, -1.0), v(-1.0, 0.0, 0.0));
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

/// **An edge lying in a face of its own solid, every sheet a cleared
/// disc, reads its side** (D1 tier 3, `crates/topo/README.md`: the
/// material sectors on one line are disjoint). Its wedge from 60° to
/// 120° about the line stands above the face, outside the face's
/// material: a rest. A keel, its wedge from −60° round to 240°, holds the
/// face's sheet and a part of its material: a crossing. A wedge from 60°
/// to a band past 180° lays its face a band into the face's material, at
/// the geometric mean of the band's zero and escalation readings at the
/// disc's rim: in band. Red if a cleared sheet reads a side the cone
/// does not hold, or a reading in band decides.
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
    let b = band();
    let tilt = ((b.zero() * b.escalate()).sqrt() / 0.9).asin();
    assert_eq!(
        judged(deg(60.0), PI + tilt),
        TouchVerdict::Unread(TouchUnread::InBand),
        "a face a band into the face's material"
    );
}
