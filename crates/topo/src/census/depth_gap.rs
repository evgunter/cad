//! **Two faces apart in depth about an axis, a centre or a circle** —
//! arm 1's clear for a cross-solid pair no direction separates: nested
//! walls, a cap inside a bore, a plug in a hole, a tube inside a tube.
//!
//! A reach box reads a face's extent in linear coordinates; this reads
//! it in a radial one. A *depth* is the distance from an axis line, from
//! a centre, or from a circle (a torus's core), and each is read for
//! both faces of the pair. Where one face's greatest depth lies below
//! the other's least, no point of one is a point of the other: the pair
//! is apart. That is the verdict, and the only one.
//!
//! **Which depths.** Each face of the pair proposes the depth of its
//! own carrier (a cylinder's or cone's axis, a sphere's centre, a
//! torus's core circle) and the axis of every circle or ellipse on its
//! boundary ([`face_read`]). A cylinder or sphere face lies at exactly
//! its carrier's radius in the depth that carrier proposes, and a
//! torus face at its minor radius from its core only on a ring torus
//! (`r` and `R − r` decided positive, [`geom::require_ring_torus`]'s
//! halves), since a spindle's points come nearer the core. A cone's
//! faces have no one depth about its axis.
//!
//! **The bounds.** A face is covered by [`Piece`]s: rings (the points
//! within `r` of a circle of radius `R`; a ball when `R = 0`) and
//! segments.
//!
//! - A sphere or torus face lies in its own carrier's ring; a plane's
//!   boundary is its arcs' rings (an ellipse's within its semi-axes'
//!   difference of its major circle), its spirics' tori and its line
//!   segments. These covers hold what they cover.
//! - A plane, cylinder or cone face lies in the convex hull of its
//!   boundary (and a cone's apex): from any point its ruling (any line,
//!   on a plane) leaves the bounded face at the boundary both ways, or
//!   reaches the apex. A NURBS patch lies in the hull of its net.
//!
//! The faces are read through the one per-kind rule, [`FaceBoxRule`]:
//! a cylinder whose axis does not decide unit there takes the boundary
//! hull and proposes no depth.
//!
//! Each depth is 1-Lipschitz, so on a ring it is within `r` of its value
//! on the circle, which [`sup_on`] and [`inf_on`] bound in closed form.
//! The axis and centre depths are convex, so their greatest value on a
//! hull is their greatest on its generators: any cover bounds them
//! from above. The core depth is not convex (a disc spans a torus's
//! hole), so only a cover that holds the face bounds it. A least value
//! needs a cover that holds the face, or a plane: the axis and centre
//! depths restricted to a plane are strictly convex with their least at
//! one point (the axis's pierce, the centre's foot), so where the
//! boolean's point placement puts that point `Out` of the face, the
//! least depth on the face is the least on its boundary.
//!
//! The margin is the gap less both faces' [`sweep_pad`], the pad the
//! reach boxes' gap takes: the covers are built from vertices and
//! carriers each certified within ε of the face, and the pad dominates
//! that, a placement misread within the band, and the closed forms'
//! rounding. Only a definite Positive clears; a Zero, a Negative, a
//! bound that is not a number, an escalation or a face with no claim
//! leaves the pair to arm 1's refusal. A pair that touches has no gap
//! and is never cleared here.

use geom::Surface;
use geom_core::{Band, Decide, Margin, Point3, Sign, Vec3, is_finite_length};

use crate::body::Body;
use crate::boolean::boxes::{
    ConicArc, EdgeBoxRule, FaceBoxRule, edge_box_rule, edge_end_point, face_box_rule, sweep_pad,
};
use crate::boolean::{CurvedPlacement, FaceContainment, face_placement};
use crate::entity::{EntityId, FaceKey};
use crate::geometry::SurfaceKey;
use crate::live::{BoundaryMember, proven};
use crate::validate::decide;

/// K name: the gap between one face's least depth and the other's
/// greatest, less both faces' pads (metres).
pub(crate) const DEPTH_GAP: &str = "census_backstop_depth_gap";

/// K name: the pierce or foot point's offset from a planar face's plane,
/// read by the point placement before it places the point.
pub(crate) const DEPTH_FOOT: &str = "census_backstop_depth_foot";

/// The points within `minor` of the circle about `center`, of radius
/// `major`, in the plane normal to `normal` (any `normal` where
/// `major` is zero).
#[derive(Clone, Copy, Debug)]
pub(super) struct Ring<T: geom_core::Real> {
    pub(super) center: Point3<T>,
    pub(super) normal: Vec3<T>,
    pub(super) major: T,
    pub(super) minor: T,
}

/// One piece of a [`Cover`].
#[derive(Clone, Copy, Debug)]
pub(super) enum Piece<T: geom_core::Real> {
    Ring(Ring<T>),
    /// The segment between two points; a point where they are one.
    Segment(Point3<T>, Point3<T>),
}

/// A face's cover.
pub(super) struct Cover<T: geom_core::Real> {
    pub(super) pieces: Vec<Piece<T>>,
    /// The pieces hold the face itself, not only its convex hull.
    pub(super) holds_face: bool,
    /// The face lies on this plane (a point and a normal), and the
    /// pieces cover its boundary: its chords exactly, its arcs within
    /// their rings.
    pub(super) plane: Option<(Point3<T>, Vec3<T>)>,
}

/// A depth.
#[derive(Clone, Copy, Debug)]
pub(super) enum Depth<T: geom_core::Real> {
    /// The distance from the line through `origin` along `axis`.
    Axis { origin: Point3<T>, axis: Vec3<T> },
    /// The distance from `center`.
    Center { center: Point3<T> },
    /// The distance from the circle about `center`, of radius `major`,
    /// normal to `axis`.
    Core {
        center: Point3<T>,
        axis: Vec3<T>,
        major: T,
    },
}

/// A depth, and where it came from: the carrier at whose radius every
/// face on it lies, where there is one.
#[derive(Clone, Copy, Debug)]
pub(super) struct Proposed<T: geom_core::Real> {
    pub(super) depth: Depth<T>,
    pub(super) level: Option<(SurfaceKey, T)>,
}

/// A face as the depth gap reads it ([`face_read`]).
pub(super) struct Read<T: geom_core::Real> {
    pub(super) surface: SurfaceKey,
    pub(super) cover: Cover<T>,
    pub(super) depths: Vec<Proposed<T>>,
}

fn point_piece<T: Decide>(p: Point3<T>) -> Piece<T> {
    Piece::Segment(p, p)
}

/// The ring an arc of `arc`'s conic lies in: an ellipse lies within its
/// semi-axes' difference of its major circle, in its plane.
pub(super) fn conic_ring<T: Decide>(arc: &ConicArc<'_, T>) -> Ring<T> {
    let (u, v) = (arc.semi_u.abs(), arc.semi_v.abs());
    Ring {
        center: arc.center,
        normal: arc.axis,
        major: u.max(v),
        minor: (u - v).abs(),
    }
}

/// **The depth a face's carrier proposes**, read off its
/// [`FaceBoxRule`] `rule` on the surface `key` (module docs), with the
/// level every face on the carrier lies at where it has one. Every
/// radius is read by its magnitude, the one posture the covers take: a
/// carrier's points lie at `|r|`. `None` for a plane, a net, and a
/// cylinder whose axis did not decide unit.
pub(super) fn carrier_depth<T: Decide>(
    rule: &FaceBoxRule<'_, T>,
    key: SurfaceKey,
    band: Band,
) -> Option<Proposed<T>> {
    let positive = |d: Result<geom_core::Decided, geom_core::Indeterminate>| matches!(d, Ok(d) if d.sign == Sign::Positive);
    match *rule {
        FaceBoxRule::CylinderSlab {
            origin,
            axis,
            radius,
        } => Some(Proposed {
            depth: Depth::Axis {
                origin,
                axis: axis.get(),
            },
            level: Some((key, radius.abs())),
        }),
        FaceBoxRule::ConeSlab { apex, axis, .. } => Some(Proposed {
            depth: Depth::Axis { origin: apex, axis },
            level: None,
        }),
        FaceBoxRule::SphereWindow { center, radius } => Some(Proposed {
            depth: Depth::Center { center },
            level: Some((key, radius.abs())),
        }),
        FaceBoxRule::TorusWindow {
            center,
            axis,
            major_radius,
            minor_radius,
            ..
        } => {
            // The tube first: `R − r` alone passes a negative `r`.
            let ring = positive(geom::torus_tube(minor_radius, band))
                && positive(geom::ring_torus(major_radius, minor_radius, band));
            Some(Proposed {
                depth: Depth::Core {
                    center,
                    axis,
                    major: major_radius.abs(),
                },
                level: ring.then_some((key, minor_radius)),
            })
        }
        FaceBoxRule::BoundaryHull | FaceBoxRule::ControlNet(_) => None,
    }
}

/// **`f` read for the depth gap** (module docs): its cover, and the
/// depths it proposes — its carrier's and the axis of every conic on
/// its boundary, gathered on the one walk that covers the boundary.
/// `None` for a face with no cover: a cylinder axis that is no
/// direction, a boundary edge with no sound bound under a hull cover,
/// or a NURBS net with no claim in it.
///
/// # Panics
///
/// Where `f` or a record on the walk from it does not resolve, as
/// [`super::face_reach_in`]: a torn boundary is not one with no claim.
pub(super) fn face_read<T: Decide>(body: &Body<T>, f: FaceKey, band: Band) -> Option<Read<T>> {
    let face = proven(&body.faces, f, EntityId::Face);
    let surface = body.face_surface_linked(f, face);
    let rule = face_box_rule(surface, band).ok()?;
    let mut depths: Vec<Proposed<T>> = carrier_depth(&rule, face.surface, band)
        .into_iter()
        .collect();
    let mut boundary = Some(Vec::new());
    for member in body.face_boundary_linked(f, face) {
        let piece = match member {
            BoundaryMember::Isolated { point, .. } => Some(point_piece(point)),
            BoundaryMember::Edge { ek, edge, .. } => {
                let end = |he, field| edge_end_point(body, ek, he, field);
                match edge_box_rule(body.edge_curve_linked(ek, edge).certified()) {
                    EdgeBoxRule::NoSoundBox => None,
                    EdgeBoxRule::Chord => Some(Piece::Segment(
                        end(edge.he_plus, "he_plus"),
                        end(edge.he_minus, "he_minus"),
                    )),
                    EdgeBoxRule::ConicAmplitude(arc) => {
                        depths.push(Proposed {
                            depth: Depth::Axis {
                                origin: arc.center,
                                axis: arc.axis,
                            },
                            level: None,
                        });
                        Some(Piece::Ring(conic_ring(&arc)))
                    }
                    EdgeBoxRule::Spiric {
                        center,
                        axis,
                        major_radius,
                        minor_radius,
                        ..
                    } => Some(Piece::Ring(Ring {
                        center,
                        normal: axis,
                        major: major_radius.abs(),
                        minor: minor_radius.abs(),
                    })),
                }
            }
        };
        match (piece, boundary.as_mut()) {
            (Some(piece), Some(pieces)) => pieces.push(piece),
            (None, _) => boundary = None,
            (Some(_), None) => {}
        }
    }
    let hull = |pieces: Option<Vec<Piece<T>>>| {
        pieces.map(|pieces| Cover {
            pieces,
            holds_face: false,
            plane: None,
        })
    };
    let held = |ring: Ring<T>| {
        Some(Cover {
            pieces: vec![Piece::Ring(ring)],
            holds_face: true,
            plane: None,
        })
    };
    let cover = match rule {
        // A plane, or a cylinder whose axis did not decide unit.
        FaceBoxRule::BoundaryHull => match surface {
            Surface::Plane { origin, normal, .. } => boundary.map(|pieces| Cover {
                pieces,
                holds_face: false,
                plane: Some((*origin, *normal)),
            }),
            _ => hull(boundary),
        },
        FaceBoxRule::CylinderSlab { .. } => hull(boundary),
        FaceBoxRule::ConeSlab { apex, .. } => hull(boundary.map(|mut pieces| {
            pieces.push(point_piece(apex));
            pieces
        })),
        FaceBoxRule::SphereWindow { center, radius } => held(Ring {
            center,
            normal: Vec3::new(T::zero(), T::zero(), T::one()),
            major: T::zero(),
            minor: radius.abs(),
        }),
        FaceBoxRule::TorusWindow {
            center,
            axis,
            major_radius,
            minor_radius,
            ..
        } => held(Ring {
            center,
            normal: axis,
            major: major_radius.abs(),
            minor: minor_radius.abs(),
        }),
        FaceBoxRule::ControlNet(net) => match net.net_state() {
            geom::NetState::Placeholder | geom::NetState::Poisoned => None,
            geom::NetState::Described => hull(Some(
                net.control().iter().map(|p| point_piece(*p)).collect(),
            )),
        },
    }?;
    Some(Read {
        surface: face.surface,
        cover,
        depths,
    })
}

/// `v` less its component along the unit `n`, and that component.
pub(super) fn split<T: Decide>(v: Vec3<T>, n: Vec3<T>) -> (Vec3<T>, T) {
    let along = v.dot(n);
    (v - n * along, along)
}

/// The distance from `x` to the interval `[lo, hi]`, zero inside it.
fn outside<T: Decide>(x: T, lo: T, hi: T) -> T {
    (lo - x).max(x - hi).max(T::zero())
}

/// What a depth reads of a circle `c + R·e` (`e` a unit vector normal
/// to the unit `n`): its offset off the depth's axis and its height
/// along it, each as a range over the circle.
struct Swing<T> {
    rho: (T, T),
    height: (T, T),
}

/// `|P(c − o) + R·P·e|`, `P` the projection off the unit `a`, lies
/// between `max(R·|n·a| − |P(c − o)|, |P(c − o)| − R)` (since
/// `|P·e|² = 1 − (e·a)² ≥ (n·a)²`) and `|P(c − o)| + R`; the height
/// `(c − o)·a + R·(e·a)` swings by `R·|a_⊥|`, `a`'s part off `n`.
fn swing<T: Decide>(ring: &Ring<T>, origin: Point3<T>, axis: Vec3<T>) -> Swing<T> {
    let n = ring.normal / ring.normal.norm();
    let a = axis / axis.norm();
    let (off, height) = split(ring.center - origin, a);
    let off = off.norm();
    let tilt = split(a, n).0.norm() * ring.major;
    Swing {
        rho: (
            (ring.major * n.dot(a).abs() - off).max(off - ring.major),
            off + ring.major,
        ),
        height: (height - tilt, height + tilt),
    }
}

/// An upper bound on `depth` over `piece` (module docs); `None` for a
/// segment's core depth, which is not convex, so its ends do not bound
/// it.
pub(super) fn sup_on<T: Decide>(depth: Depth<T>, piece: &Piece<T>) -> Option<T> {
    let ring = match piece {
        Piece::Ring(ring) => ring,
        Piece::Segment(_, _) if matches!(depth, Depth::Core { .. }) => return None,
        Piece::Segment(p, q) => {
            return greatest([p, q].into_iter().map(|x| Some(exact(depth, *x))));
        }
    };
    let circle = match depth {
        Depth::Axis { origin, axis } => swing(ring, origin, axis).rho.1,
        // `|w + R·e|² = w_n² + |w_⊥ + R·e|² ≤ w_n² + (|w_⊥| + R)²`, `w`
        // from the centre to `c`, split along `n`.
        Depth::Center { center } => {
            let (perp, along) = split(ring.center - center, ring.normal / ring.normal.norm());
            (along.powi(2) + (perp.norm() + ring.major).powi(2)).sqrt()
        }
        Depth::Core {
            center,
            axis,
            major,
        } => {
            let s = swing(ring, center, axis);
            let radial = (s.rho.1 - major).abs().max((s.rho.0 - major).abs());
            let t = s.height.0.abs().max(s.height.1.abs());
            (radial.powi(2) + t.powi(2)).sqrt()
        }
    };
    Some(circle + ring.minor)
}

/// A lower bound on `depth` over `piece` (module docs); `None` for a
/// segment's core depth, which nothing reads.
pub(super) fn inf_on<T: Decide>(depth: Depth<T>, piece: &Piece<T>) -> Option<T> {
    let ring = match *piece {
        Piece::Ring(ring) => ring,
        Piece::Segment(p, q) => {
            // The nearest point of the segment to the axis or centre, in
            // the projection that depth reads.
            let (u, w) = match depth {
                Depth::Axis { origin, axis } => {
                    let a = axis / axis.norm();
                    (split(p - origin, a).0, split(q - p, a).0)
                }
                Depth::Center { center } => (p - center, q - p),
                Depth::Core { .. } => return None,
            };
            // A point (`w = 0`) has no direction to slide along.
            let s = -(u.dot(w)) / w.dot(w);
            let s = if is_finite_length(s) {
                s.max(T::zero()).min(T::one())
            } else {
                T::zero()
            };
            return Some((u + w * s).norm());
        }
    };
    let circle = match depth {
        Depth::Axis { origin, axis } => swing(&ring, origin, axis).rho.0,
        // `|w + R·e|² ≥ w_n² + (|w_⊥| − R)²`.
        Depth::Center { center } => {
            let (perp, along) = split(ring.center - center, ring.normal / ring.normal.norm());
            (along.powi(2) + (perp.norm() - ring.major).powi(2)).sqrt()
        }
        Depth::Core {
            center,
            axis,
            major,
        } => {
            let s = swing(&ring, center, axis);
            let radial = outside(major, s.rho.0, s.rho.1);
            let t = outside(T::zero(), s.height.0, s.height.1);
            (radial.powi(2) + t.powi(2)).sqrt()
        }
    };
    Some(circle - ring.minor)
}

/// The exact depth of the point `x`.
pub(super) fn exact<T: Decide>(depth: Depth<T>, x: Point3<T>) -> T {
    match depth {
        Depth::Axis { origin, axis } => split(x - origin, axis / axis.norm()).0.norm(),
        Depth::Center { center } => (x - center).norm(),
        Depth::Core {
            center,
            axis,
            major,
        } => {
            let (off, t) = split(x - center, axis / axis.norm());
            ((off.norm() - major).powi(2) + t.powi(2)).sqrt()
        }
    }
}

/// The greatest of `bounds`, or `None` where there is none or one is
/// not a number (`max` would drop it).
pub(super) fn greatest<T: Decide>(bounds: impl Iterator<Item = Option<T>>) -> Option<T> {
    let mut acc: Option<T> = None;
    for b in bounds {
        let b = b.filter(|b| is_finite_length(*b))?;
        acc = Some(acc.map_or(b, |m| m.max(b)));
    }
    acc
}

/// The least of `bounds`, as [`greatest`].
pub(super) fn least<T: Decide>(bounds: impl Iterator<Item = Option<T>>) -> Option<T> {
    greatest(bounds.map(|b| b.map(|b| -b))).map(|b| -b)
}

/// The point where a plane's depth is least: the axis's pierce of it,
/// or the centre's foot on it. `None` for a core depth.
fn least_point<T: Decide>(depth: Depth<T>, (o, n): (Point3<T>, Vec3<T>)) -> Option<Point3<T>> {
    match depth {
        Depth::Axis { origin, axis } => {
            let s = (o - origin).dot(n) / axis.dot(n);
            Some(origin + axis * s)
        }
        Depth::Center { center } => {
            let n = n / n.norm();
            Some(center - n * (center - o).dot(n))
        }
        Depth::Core { .. } => None,
    }
}

/// An upper bound on `depth` over what `cover` covers: any cover for a
/// convex depth, only one that holds its face for the core's.
pub(super) fn upper<T: Decide>(depth: Depth<T>, cover: &Cover<T>) -> Option<T> {
    let convex = !matches!(depth, Depth::Core { .. });
    (convex || cover.holds_face)
        .then(|| greatest(cover.pieces.iter().map(|p| sup_on(depth, p))))
        .flatten()
}

/// Whether `lo − hi`, less both faces' pads, is definitely positive.
pub(super) fn clears<T: Decide>(hi: T, lo: T, band: Band) -> bool {
    let pad = T::from_f64(2.0 * sweep_pad(band));
    matches!(
        decide(DEPTH_GAP, Margin::of(lo - hi - pad), band),
        Ok(Sign::Positive)
    )
}

/// The exact depth every point of the face read as `read` lies at,
/// where `proposed` came from its own carrier.
fn level_on<T: Decide>(read: &Read<T>, proposed: &Proposed<T>) -> Option<T> {
    proposed
        .level
        .filter(|(key, _)| *key == read.surface)
        .map(|(_, level)| level)
}

/// An upper bound on `proposed`'s depth over the face read as `read`.
fn greatest_on<T: Decide>(read: &Read<T>, proposed: &Proposed<T>) -> Option<T> {
    level_on(read, proposed).or_else(|| upper(proposed.depth, &read.cover))
}

/// A lower bound on `proposed`'s depth over the face `f`, read as
/// `read`; `None` for a face with no claim.
fn least_on<T: Decide>(
    body: &Body<T>,
    f: FaceKey,
    read: &Read<T>,
    proposed: &Proposed<T>,
    band: Band,
) -> Option<T> {
    if let Some(level) = level_on(read, proposed) {
        return Some(level);
    }
    let depth = proposed.depth;
    let cover = &read.cover;
    let on_boundary = || least(cover.pieces.iter().map(|p| inf_on(depth, p)));
    if cover.holds_face {
        on_boundary()
    } else {
        cover
            .plane
            .and_then(|plane| least_point(depth, plane))
            .filter(|q| [q.x, q.y, q.z].into_iter().all(is_finite_length))
            .filter(|q| {
                matches!(
                    face_placement(body, f, *q, band, DEPTH_FOOT),
                    Ok(CurvedPlacement::Trim(Some(FaceContainment::Out)))
                )
            })
            .and_then(|_| on_boundary())
    }
}

/// **Whether the faces `a`, `b` are apart in some depth** either
/// proposes (module docs).
///
/// # Panics
///
/// As [`face_read`].
pub(super) fn apart_in_depth<T: Decide>(
    body: &Body<T>,
    a: FaceKey,
    b: FaceKey,
    band: Band,
) -> bool {
    let (Some(ra), Some(rb)) = (face_read(body, a, band), face_read(body, b, band)) else {
        return false;
    };
    ra.depths.iter().chain(&rb.depths).any(|proposed| {
        // A least bound costs a point placement, so it is read only
        // against a greatest the other face has.
        let below = |lo: (FaceKey, &Read<T>), hi: &Read<T>| {
            greatest_on(hi, proposed).is_some_and(|hi| {
                least_on(body, lo.0, lo.1, proposed, band).is_some_and(|lo| clears(hi, lo, band))
            })
        };
        below((b, &rb), &ra) || below((a, &ra), &rb)
    })
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use geom_core::Tol;

    use super::*;

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    fn p(x: f64, y: f64, z: f64) -> Point3<f64> {
        Point3::new(x, y, z)
    }

    fn v(x: f64, y: f64, z: f64) -> Vec3<f64> {
        Vec3::new(x, y, z)
    }

    fn rim(r: f64, z: f64) -> Piece<f64> {
        Piece::Ring(Ring {
            center: p(0.0, 0.0, z),
            normal: v(0.0, 0.0, 1.0),
            major: r,
            minor: 0.0,
        })
    }

    fn z_axis() -> Depth<f64> {
        Depth::Axis {
            origin: p(0.0, 0.0, 0.0),
            axis: v(0.0, 0.0, 2.0),
        }
    }

    /// **The clear is a strict gap past the pad, and nothing less.** A
    /// gap of 0.1 clears; a band's width, none, or half the pads do not:
    /// those touch, and arm 1 keeps refusing them. Red where the pads
    /// are dropped or a Zero is read as apart.
    #[test]
    fn a_gap_clears_only_past_the_pad() {
        let band = band();
        let pad = 2.0 * sweep_pad(band);
        assert!(clears(0.85, 0.95, band), "0.1 clears");
        assert!(
            !clears(0.95 - band.zero(), 0.95, band),
            "a band's width is a touch"
        );
        assert!(!clears(0.95, 0.95, band), "no gap is a touch");
        assert!(!clears(0.95 - 0.5 * pad, 0.95, band), "inside the pads");
        assert!(!clears(0.95 - pad, 0.95, band), "exactly the pads, a Zero");
        assert!(!clears(1.05, 0.95, band), "an overlap");
        assert!(!clears(f64::NAN, 0.95, band), "not a number");
    }

    /// **A coaxial circle reads at its radius from both sides**, and one
    /// moved 0.05 off the axis reads `0.85 ± 0.05`. Red where the
    /// circle's offset is dropped or added the wrong way.
    #[test]
    fn a_rim_reads_at_its_radius_and_its_offset() {
        let depth = z_axis();
        assert_eq!(sup_on(depth, &rim(0.85, 3.0)), Some(0.85), "coaxial, above");
        assert_eq!(inf_on(depth, &rim(0.85, 3.0)), Some(0.85), "coaxial, below");
        let off = Piece::Ring(Ring {
            center: p(0.05, 0.0, 0.0),
            normal: v(0.0, 0.0, 1.0),
            major: 0.85,
            minor: 0.0,
        });
        assert!(
            (sup_on(depth, &off).unwrap() - 0.9).abs() < 1e-15,
            "off the axis, above"
        );
        assert!(
            (inf_on(depth, &off).unwrap() - 0.8).abs() < 1e-15,
            "off the axis, below"
        );
    }

    /// **A torus core reads only a cover that holds the face.** The core
    /// circle lies in the tube, but a disc it bounds spans the hole: as a
    /// hull cover it gives no bound. A torus face's own ring does. Red
    /// where the core depth reads a hull.
    #[test]
    fn a_core_depth_bounds_a_held_tube_and_never_a_disc_across_the_hole() {
        let core = Depth::Core {
            center: p(0.0, 0.0, 0.0),
            axis: v(0.0, 0.0, 1.0),
            major: 2.0,
        };
        let disc = Cover {
            pieces: vec![rim(2.0, 0.0)],
            holds_face: false,
            plane: Some((p(0.0, 0.0, 0.0), v(0.0, 0.0, 1.0))),
        };
        assert_eq!(upper(core, &disc), None, "a disc spans the hole");
        let tube = Cover {
            pieces: vec![Piece::Ring(Ring {
                center: p(0.0, 0.0, 0.0),
                normal: v(0.0, 0.0, 1.0),
                major: 2.0,
                minor: 0.45,
            })],
            holds_face: true,
            plane: None,
        };
        assert_eq!(upper(core, &tube), Some(0.45), "the inner tube");
        assert_eq!(upper(z_axis(), &disc), Some(2.0), "an axis reads the hull");
    }

    fn torus(major: f64, minor: f64) -> FaceBoxRule<'static, f64> {
        FaceBoxRule::TorusWindow {
            center: p(0.0, 0.0, 0.0),
            axis: v(0.0, 0.0, 1.0),
            major_radius: major,
            minor_radius: minor,
            u_ref: v(1.0, 0.0, 0.0),
        }
    }

    /// **Only a ring torus gives its faces an exact depth**: a horn or
    /// spindle torus (`R ≤ r`) has points nearer its core than `r`, and
    /// a negative tube passes `R − r` alone. A carrier's radius is read
    /// by its magnitude, as its cover reads it. Red where every torus
    /// is a ring torus, or a stored radius is taken as a level raw.
    #[test]
    fn only_a_ring_torus_lies_at_its_minor_radius() {
        let band = band();
        let key = SurfaceKey::default();
        let level = |rule: FaceBoxRule<'static, f64>| {
            carrier_depth(&rule, key, band)
                .expect("a carrier proposes its depth")
                .level
                .map(|(_, r)| r)
        };
        assert_eq!(level(torus(2.0, 0.5)), Some(0.5), "a ring torus");
        assert_eq!(level(torus(0.5, 0.5)), None, "a horn torus");
        assert_eq!(level(torus(0.4, 0.5)), None, "a spindle torus");
        assert_eq!(
            level(torus(0.5 + 0.5 * band.zero(), 0.5)),
            None,
            "a horn within the band"
        );
        assert_eq!(level(torus(0.75, -0.3)), None, "a negative tube");
        assert_eq!(
            level(FaceBoxRule::SphereWindow {
                center: p(0.0, 0.0, 0.0),
                radius: -0.7,
            }),
            Some(0.7),
            "a sphere stored with a negative radius"
        );
    }

    /// **An ellipse's ring holds the ellipse**: about its own axis an
    /// ellipse of semi-axes 2 and 1 spans depths `[1, 2]`, and its ring
    /// (the major circle, widened by the semi-axes' difference) bounds
    /// both ends. Red where the ring is read as the major circle alone,
    /// which puts the least depth at 2.
    #[test]
    fn an_ellipse_ring_holds_its_minor_vertices() {
        let carrier = geom::Curve3::Ellipse {
            center: p(0.0, 0.0, 0.0),
            axis: v(0.0, 0.0, 1.0),
            semi_u: 2.0,
            semi_v: -1.0,
            u_ref: v(1.0, 0.0, 0.0),
        };
        let ring = Piece::Ring(conic_ring(&ConicArc {
            carrier: &carrier,
            center: p(0.0, 0.0, 0.0),
            axis: v(0.0, 0.0, 1.0),
            semi_u: 2.0,
            semi_v: -1.0,
            u_ref: v(1.0, 0.0, 0.0),
            params: (0.0, core::f64::consts::TAU),
        }));
        assert!(inf_on(z_axis(), &ring).unwrap() <= 1.0, "the minor vertex");
        assert!(sup_on(z_axis(), &ring).unwrap() >= 2.0, "the major vertex");
    }

    /// **A cone face's hull holds its apex**, wherever the boundary
    /// runs: a unit square's face restated on a cone whose apex stands
    /// 5 above it reads, from the square's centre, a greatest depth past
    /// the apex's 5. Red where the apex is dropped from the hull.
    #[test]
    fn a_cone_face_hull_holds_its_apex() {
        let tol = Tol::witness();
        let mut body =
            crate::test_support_fixtures::brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol);
        let (f, key) = body
            .faces
            .iter()
            .next()
            .map(|(f, face)| (f, face.surface))
            .expect("a face");
        let apex = p(0.5, 0.5, 6.0);
        *body.surfaces.get_mut(key).expect("its surface") = Surface::Cone {
            apex,
            axis: v(0.0, 0.0, -1.0),
            half_angle: 0.3,
            u_ref: v(1.0, 0.0, 0.0),
        };
        let read = face_read(&body, f, band()).expect("a cone face is read");
        let centre = Depth::Center {
            center: p(0.5, 0.5, 1.0),
        };
        assert!(
            upper(centre, &read.cover).unwrap() >= 5.0,
            "the apex is in the hull"
        );
    }

    /// **A bound that is not a number is no bound**, though `max` and
    /// `min` would drop it; and no piece gives none.
    #[test]
    fn a_poisoned_bound_is_no_bound() {
        assert_eq!(
            greatest([Some(1.0), Some(f64::NAN)].into_iter()),
            None,
            "NaN last"
        );
        assert_eq!(
            greatest([Some(f64::NAN), Some(1.0)].into_iter()),
            None,
            "NaN first"
        );
        assert_eq!(least([Some(1.0), None].into_iter()), None, "no claim");
        assert_eq!(greatest(std::iter::empty::<Option<f64>>()), None, "nothing");
        assert_eq!(
            least([Some(2.0), Some(1.0)].into_iter()),
            Some(1.0),
            "least"
        );
    }

    /// **A segment reads its nearest point, not its ends**: a chord
    /// passing 0.3 from the axis between ends 1 away.
    #[test]
    fn a_segment_reads_its_nearest_point() {
        let chord = Piece::Segment(p(-1.0, 0.3, 0.0), p(1.0, 0.3, 5.0));
        assert!(
            (inf_on(z_axis(), &chord).unwrap() - 0.3).abs() < 1e-15,
            "the axis"
        );
        let centre = Depth::Center {
            center: p(0.0, 0.0, 2.5),
        };
        assert!(
            (inf_on(centre, &chord).unwrap() - 0.3).abs() < 1e-15,
            "a centre"
        );
        assert!(
            (sup_on(z_axis(), &chord).unwrap() - 1.09f64.sqrt()).abs() < 1e-15,
            "its ends"
        );
        let core = Depth::Core {
            center: p(0.0, 0.0, 2.5),
            axis: v(0.0, 0.0, 1.0),
            major: 0.5,
        };
        assert_eq!(sup_on(core, &chord), None, "a core reads no segment");
    }
}
