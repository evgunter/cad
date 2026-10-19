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
//! **Which depths.** Each face of the pair proposes the depths of its
//! own carrier (a cylinder's or cone's axis, a sphere's centre, a
//! torus's core circle) and the axis of every circle or ellipse on its
//! boundary ([`depths`]). A face on the carrier a depth came from lies
//! at exactly its radius; a torus's only on a ring torus, `R − r`
//! decided positive, since a spindle's points come nearer the core.
//!
//! **The bounds.** A face is covered by [`Piece`]s ([`face_cover`]):
//! rings (the points within `r` of a circle of radius `R`; a ball when
//! `R = 0`) and segments.
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
use crate::boolean::boxes::{EdgeBoxRule, edge_box_rule, edge_end_point, sweep_pad};
use crate::boolean::{CurvedPlacement, FaceContainment, face_placement};
use crate::entity::{EntityId, FaceKey};
use crate::geometry::SurfaceKey;
use crate::live::{BoundaryMember, proven};
use crate::validate::decide;

/// K name: the gap between one face's least depth and the other's
/// greatest, less both faces' pads (metres).
pub(crate) const DEPTH_GAP: &str = "census_backstop_depth_gap";

/// K name: a torus's `R − r` (metres), positive on a ring torus, whose
/// points all lie at depth `r` from its core.
pub(crate) const DEPTH_RING: &str = "census_backstop_depth_ring";

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

/// A face's cover ([`face_cover`]).
pub(super) struct Cover<T: geom_core::Real> {
    pub(super) pieces: Vec<Piece<T>>,
    /// The pieces hold the face itself, not only its convex hull.
    pub(super) holds_face: bool,
    /// The pieces are a planar face's boundary, held exactly.
    pub(super) plane: Option<(Point3<T>, Vec3<T>)>,
}

/// A depth ([`depths`]).
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
#[derive(Clone, Copy)]
struct Proposed<T: geom_core::Real> {
    depth: Depth<T>,
    level: Option<(SurfaceKey, T)>,
}

fn point_piece<T: Decide>(p: Point3<T>) -> Piece<T> {
    Piece::Segment(p, p)
}

/// **The pieces covering `f`** (module docs), or `None` for a boundary
/// edge with no sound bound or a NURBS net with no claim in it.
///
/// # Panics
///
/// Where `f` or a record on the walk from it does not resolve, as
/// [`super::face_reach_in`]: a torn boundary is not one with no claim.
pub(super) fn face_cover<T: Decide>(body: &Body<T>, f: FaceKey) -> Option<Cover<T>> {
    let face = proven(&body.faces, f, EntityId::Face);
    let boundary = |extra: Option<Point3<T>>| {
        let mut pieces: Vec<Piece<T>> = extra.into_iter().map(point_piece).collect();
        for member in body.face_boundary_linked(f, face) {
            match member {
                BoundaryMember::Isolated { point, .. } => pieces.push(point_piece(point)),
                BoundaryMember::Edge { ek, edge, .. } => {
                    let end = |he, field| edge_end_point(body, ek, he, field);
                    pieces.push(
                        match edge_box_rule(body.edge_curve_linked(ek, edge).certified()) {
                            EdgeBoxRule::NoSoundBox => return None,
                            EdgeBoxRule::Chord => Piece::Segment(
                                end(edge.he_plus, "he_plus"),
                                end(edge.he_minus, "he_minus"),
                            ),
                            // An ellipse lies within its semi-axes' difference
                            // of its major circle, in its plane.
                            EdgeBoxRule::ConicAmplitude(arc) => Piece::Ring(Ring {
                                center: arc.center,
                                normal: arc.axis,
                                major: arc.semi_u.abs().max(arc.semi_v.abs()),
                                minor: (arc.semi_u.abs() - arc.semi_v.abs()).abs(),
                            }),
                            EdgeBoxRule::Spiric {
                                center,
                                axis,
                                major_radius,
                                minor_radius,
                                ..
                            } => Piece::Ring(Ring {
                                center,
                                normal: axis,
                                major: major_radius.abs(),
                                minor: minor_radius.abs(),
                            }),
                        },
                    );
                }
            }
        }
        Some(pieces)
    };
    let hull = |pieces: Option<Vec<Piece<T>>>| {
        pieces.map(|pieces| Cover {
            pieces,
            holds_face: false,
            plane: None,
        })
    };
    let held = |ring: Ring<T>| Cover {
        pieces: vec![Piece::Ring(ring)],
        holds_face: true,
        plane: None,
    };
    let net = |net: &geom::surfaces::nurbs::NurbsSurface<T>| match net.net_state() {
        geom::NetState::Placeholder | geom::NetState::Poisoned => None,
        geom::NetState::Described => hull(Some(
            net.control().iter().map(|p| point_piece(*p)).collect(),
        )),
    };
    let z = Vec3::new(T::zero(), T::zero(), T::one());
    match body.face_surface_linked(f, face) {
        Surface::Plane { origin, normal, .. } => boundary(None).map(|pieces| Cover {
            pieces,
            holds_face: false,
            plane: Some((*origin, *normal)),
        }),
        Surface::Cylinder { .. } => hull(boundary(None)),
        Surface::Cone { apex, .. } => hull(boundary(Some(*apex))),
        Surface::Sphere { center, radius, .. } => Some(held(Ring {
            center: *center,
            normal: z,
            major: T::zero(),
            minor: radius.abs(),
        })),
        Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            ..
        } => Some(held(Ring {
            center: *center,
            normal: *axis,
            major: major_radius.abs(),
            minor: minor_radius.abs(),
        })),
        Surface::Nurbs(patch) => net(patch),
        Surface::Approx(a) => net(a.fit()),
    }
}

/// The depths `f` proposes (module docs): its carrier's, and the axis
/// of every conic on its boundary.
fn depths<T: Decide>(body: &Body<T>, f: FaceKey, band: Band) -> Vec<Proposed<T>> {
    let face = proven(&body.faces, f, EntityId::Face);
    let key = face.surface;
    let mut out = Vec::new();
    match body.face_surface_linked(f, face) {
        Surface::Cylinder {
            origin,
            axis,
            radius,
            ..
        } => out.push(Proposed {
            depth: Depth::Axis {
                origin: *origin,
                axis: *axis,
            },
            level: Some((key, *radius)),
        }),
        Surface::Cone { apex, axis, .. } => out.push(Proposed {
            depth: Depth::Axis {
                origin: *apex,
                axis: *axis,
            },
            level: None,
        }),
        Surface::Sphere { center, radius, .. } => out.push(Proposed {
            depth: Depth::Center { center: *center },
            level: Some((key, *radius)),
        }),
        Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            ..
        } => {
            let ring = ring_torus(*major_radius, *minor_radius, band);
            out.push(Proposed {
                depth: Depth::Core {
                    center: *center,
                    axis: *axis,
                    major: *major_radius,
                },
                level: ring.then_some((key, *minor_radius)),
            });
        }
        Surface::Plane { .. } | Surface::Nurbs(_) | Surface::Approx(_) => {}
    }
    for member in body.face_boundary_linked(f, face) {
        if let BoundaryMember::Edge { ek, edge, .. } = member
            && let EdgeBoxRule::ConicAmplitude(arc) =
                edge_box_rule(body.edge_curve_linked(ek, edge).certified())
        {
            out.push(Proposed {
                depth: Depth::Axis {
                    origin: arc.center,
                    axis: arc.axis,
                },
                level: None,
            });
        }
    }
    out
}

/// Whether a torus of radii `major`, `minor` is a ring torus, every
/// point of it at depth `minor` from its core: `R − r` decided positive.
pub(super) fn ring_torus<T: Decide>(major: T, minor: T, band: Band) -> bool {
    matches!(
        decide(DEPTH_RING, Margin::of(major - minor), band),
        Ok(Sign::Positive)
    )
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

/// Bounds `(least, greatest)` on `depth` over the face `f` with `cover`;
/// `None` for a side with no claim.
fn range<T: Decide>(
    body: &Body<T>,
    f: FaceKey,
    cover: &Cover<T>,
    proposed: &Proposed<T>,
    band: Band,
) -> (Option<T>, Option<T>) {
    let face = proven(&body.faces, f, EntityId::Face);
    if let Some((key, level)) = proposed.level
        && key == face.surface
    {
        return (Some(level), Some(level));
    }
    let depth = proposed.depth;
    let hi = upper(depth, cover);
    let on_boundary = || least(cover.pieces.iter().map(|p| inf_on(depth, p)));
    let lo = if cover.holds_face {
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
    };
    (lo, hi)
}

/// **Whether the faces `a`, `b` are apart in some depth** either
/// proposes (module docs).
///
/// # Panics
///
/// As [`face_cover`].
pub(super) fn apart_in_depth<T: Decide>(
    body: &Body<T>,
    a: FaceKey,
    b: FaceKey,
    band: Band,
) -> bool {
    let (Some(ca), Some(cb)) = (face_cover(body, a), face_cover(body, b)) else {
        return false;
    };
    depths(body, a, band)
        .into_iter()
        .chain(depths(body, b, band))
        .any(|proposed| {
            let (alo, ahi) = range(body, a, &ca, &proposed, band);
            let (blo, bhi) = range(body, b, &cb, &proposed, band);
            [(ahi, blo), (bhi, alo)].into_iter().any(|pair| match pair {
                (Some(hi), Some(lo)) => clears(hi, lo, band),
                _ => false,
            })
        })
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
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

    /// **Only a ring torus gives its faces an exact depth**: a horn or
    /// spindle torus (`R ≤ r`) has points nearer its core than `r`.
    #[test]
    fn only_a_ring_torus_lies_at_its_minor_radius() {
        let band = band();
        assert!(ring_torus(2.0, 0.5, band), "a ring torus");
        assert!(!ring_torus(0.5, 0.5, band), "a horn torus");
        assert!(!ring_torus(0.4, 0.5, band), "a spindle torus");
        assert!(
            !ring_torus(0.5 + 0.5 * band.zero(), 0.5, band),
            "a horn within the band"
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
