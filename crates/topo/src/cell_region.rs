//! **Is a chart rectangle of a face's carrier inside the face, outside
//! it, or neither?** The region is read off the face's edges on their
//! own carriers, in model space, and the point-in-face doors; no chart
//! polygon is drawn (`[intrinsic]`, `[one-way.readings]`).
//!
//! A cell is the image on the carrier of a rectangle of its stored
//! chart: a connected patch. [`cell_region`] answers in two steps.
//!
//! 1. **Boundary-free.** The patch lies in the ball about its centre
//!    station whose radius bounds the patch's enclosure. Every
//!    non-seam boundary edge, and every vertex of the face, is decided
//!    clear of that ball in metres, each on its own carrier: a line by
//!    the distance to its segment, a circle or an ellipse by the
//!    distance to the annulus of its plane it lies in, and any other
//!    carrier by the ball its arc lies in
//!    (`splitting::containment::carrier_ball`). An edge's 3-D box is
//!    never read: a long diagonal edge's box covers every cell beside
//!    it.
//! 2. **Witness.** A connected patch no point of the face's boundary
//!    meets lies wholly in one component of the carrier minus that
//!    boundary, so wholly in the face or wholly out of it. A seam is
//!    no boundary: the face lies on both sides of it. The centre
//!    station, placed by the face's point-in-face door
//!    ([`crate::contfp`] on a plane, [`crate::curved_face_containment`]
//!    on a curved carrier), says which.
//!
//! Anything else is [`CellRegion::Undecided`]: an edge not decided
//! clear (in the band, poisoned, or genuinely crossing), a station the
//! door places on the boundary or does not read, a refusal of either
//! door. Every one of those keeps the cell, which is the direction a
//! subdivision consumer's looseness has to run.

use geom::{Curve3, Surface};
use geom_core::interval::Interval;
use geom_core::{Band, Margin, Point3, Real, Sign};

use crate::Body;
use crate::boolean::{FaceContainment, contfp, curved_face_containment};
use crate::entity::{FaceKey, LoopBoundary};
use crate::splitting::containment::{arc_trim_margin, carrier_ball, circle_miss};
use crate::validate::decide;

/// The one row every clearance reading decides through: metres between
/// the patch's ball and one boundary feature, read on that feature's
/// carrier. One name for every carrier kind, because they are one
/// question (is this feature clear of this patch) asked of different
/// loci; a `Positive` on it is the only reading that clears.
const GAP: &str = "cell_region_gap";

/// Whether the circle's point over the station's foot lies past a
/// circular edge's ends: the chordal trim margin, metres.
const TRIM: &str = "cell_region_arc_trim";

/// What [`cell_region`] says about one cell of a face's carrier.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellRegion {
    /// The patch meets no boundary of the face and its station is in
    /// the face: every point of the patch is a point of the face's
    /// interior.
    Inside,
    /// The patch meets no boundary of the face and its station is out
    /// of the face: no point of the patch is a point of the face.
    Outside,
    /// Neither was decided.
    Undecided,
}

/// **The verdict on the cell `u × v` of `face`'s stored chart.**
///
/// `u` and `v` are closed ranges of the face's own carrier parameters,
/// the chart [`Surface::eval`] reads. A face or a surface that does
/// not resolve, a loop that does not walk, and a carrier with no point
/// door are all [`CellRegion::Undecided`].
pub fn cell_region(
    body: &Body<Interval>,
    face: FaceKey,
    u: (f64, f64),
    v: (f64, f64),
    band: Band,
) -> CellRegion {
    let Some(surface) = body
        .get_face(face)
        .and_then(|f| body.get_surface(f.surface))
    else {
        return CellRegion::Undecided;
    };
    let span = |(lo, hi): (f64, f64)| Interval::from_bounds(lo, hi);
    let point = |(lo, hi): (f64, f64)| {
        let mid = 0.5 * (lo + hi);
        Interval::from_bounds(mid, mid)
    };
    let station = surface.eval(point(u), point(v));
    let reach = (surface.eval(span(u), span(v)) - station).norm();
    if !clear_of_boundary(body, face, station, reach, band) {
        return CellRegion::Undecided;
    }
    match place(body, face, surface, station, band) {
        Some(FaceContainment::In) => CellRegion::Inside,
        Some(FaceContainment::Out) => CellRegion::Outside,
        Some(FaceContainment::OnEdge(_) | FaceContainment::OnVertex(_)) | None => {
            CellRegion::Undecided
        }
    }
}

/// **Where the face's point door places the point `(u, v)` of `face`'s
/// stored chart**: [`crate::contfp`] on a plane,
/// [`crate::curved_face_containment`] on a curved carrier. `None` for
/// a face or a surface that does not resolve, a door that refuses, and
/// a curved door's unread remainder.
pub fn station_region(
    body: &Body<Interval>,
    face: FaceKey,
    (u, v): (f64, f64),
    band: Band,
) -> Option<FaceContainment> {
    let surface = body
        .get_face(face)
        .and_then(|f| body.get_surface(f.surface))?;
    let station = surface.eval(Interval::from_bounds(u, u), Interval::from_bounds(v, v));
    place(body, face, surface, station, band)
}

fn place(
    body: &Body<Interval>,
    face: FaceKey,
    surface: &Surface<Interval>,
    station: Point3<Interval>,
    band: Band,
) -> Option<FaceContainment> {
    match *surface {
        Surface::Plane { normal, .. } => contfp(body, face, normal, station, band).ok(),
        _ => curved_face_containment(body, face, station, band)
            .ok()
            .flatten(),
    }
}

/// Is every vertex of `face`, and every edge of it that is not a seam,
/// decided clear of the ball about `station` of radius `reach`?
fn clear_of_boundary(
    body: &Body<Interval>,
    face: FaceKey,
    station: Point3<Interval>,
    reach: Interval,
    band: Band,
) -> bool {
    let clear = |distance: Interval| {
        matches!(
            decide(GAP, Margin::of(distance - reach), band),
            Ok(Sign::Positive)
        )
    };
    let Some(f) = body.get_face(face) else {
        return false;
    };
    let on_face = |he| {
        body.get_half_edge(he)
            .and_then(|h| body.get_loop(h.parent_loop))
            .is_some_and(|l| l.face == face)
    };
    for &lk in core::iter::once(&f.outer).chain(&f.rings) {
        let Some(l) = body.get_loop(lk) else {
            return false;
        };
        let first = match l.boundary {
            LoopBoundary::Empty { vertex } => {
                let Some(p) = body
                    .get_vertex(vertex)
                    .and_then(|x| body.get_point(x.point))
                else {
                    return false;
                };
                if !clear((*p - station).norm()) {
                    return false;
                }
                continue;
            }
            LoopBoundary::Cycle { first } => first,
        };
        let Some(cycle) = body.loop_cycle(first) else {
            return false;
        };
        for he in cycle {
            let Some(h) = body.get_half_edge(he) else {
                return false;
            };
            let Some(p) = body
                .get_vertex(h.start)
                .and_then(|x| body.get_point(x.point))
            else {
                return false;
            };
            if !clear((*p - station).norm()) {
                return false;
            }
            let Some(e) = body.get_edge(h.edge) else {
                return false;
            };
            if on_face(e.he_plus) && on_face(e.he_minus) {
                continue;
            }
            let Some(geom) = body.get_curve_geom(e.curve) else {
                return false;
            };
            // Null scaffolding has no carrier: its locus is its vertex,
            // read above.
            let Some(curve) = geom.certified() else {
                continue;
            };
            let Some(distance) = carrier_distance(curve.carrier(), curve.params(), station, band)
            else {
                return false;
            };
            if !clear(distance) {
                return false;
            }
        }
    }
    true
}

/// A lower bound on the distance from `q` to the arc `span` of
/// `carrier`, read on the carrier: the segment's own distance for a
/// line; the arc's own distance for a circle ([`arc_distance`]); for
/// an ellipse, the distance to the annulus of its plane between its
/// smaller and larger semi-axis, which holds the whole conic; for any
/// other carrier, the distance to the ball its arc lies in. `None` for a spline with no control points.
fn carrier_distance(
    carrier: &Curve3<Interval>,
    (t0, t1): (Interval, Interval),
    q: Point3<Interval>,
    band: Band,
) -> Option<Interval> {
    let zero = Interval::zero();
    match *carrier {
        Curve3::Line { .. } => {
            let (a, b) = (carrier.eval(t0), carrier.eval(t1));
            let (d, w) = (b - a, q - a);
            let t = (w.dot(d) / d.dot(d)).max(zero).min(Interval::one());
            Some((w - d * t).norm())
        }
        Curve3::Circle {
            center,
            axis,
            radius,
            ..
        } => Some(arc_distance(
            carrier,
            (t0, t1),
            q,
            (center, axis, radius),
            band,
        )),
        Curve3::Ellipse {
            center,
            axis,
            semi_u,
            semi_v,
            ..
        } => {
            let (major, minor) = Curve3::semi_axes_by_size(semi_u, semi_v);
            Some(annulus_distance(q, center, axis, (minor, major)))
        }
        Curve3::Spiric { .. } | Curve3::Nurbs(_) => {
            let (center, reach) = carrier_ball(carrier, (t0, t1))?;
            Some(((q - center).norm() - reach).max(zero))
        }
    }
}

/// **The distance from `q` to a circle's arc.** Along the circle the
/// distance to `q` grows with the angle from `r`, the circle's point
/// over `q`'s foot on its plane. So where `r` is decided past the
/// arc's ends (the chordal trim [`arc_trim_margin`] reads, in metres),
/// the nearer end is the arc's nearest point; anywhere else the
/// distance to the whole circle bounds the arc's from below. A foot on
/// the axis puts `r` nowhere, poisons the trim, and takes the circle.
fn arc_distance(
    carrier: &Curve3<Interval>,
    (t0, t1): (Interval, Interval),
    q: Point3<Interval>,
    (center, axis, radius): (Point3<Interval>, geom_core::Vec3<Interval>, Interval),
    band: Band,
) -> Interval {
    let n = axis / axis.norm();
    let circle = circle_miss(q, center, n, radius);
    let off = q - center;
    let foot = off - n * off.dot(n);
    let r = center + foot * (radius / foot.norm());
    let (a, b) = (carrier.eval(t0), carrier.eval(t1));
    let apex = carrier.eval((t0 + t1) * Interval::from_bounds(0.5, 0.5));
    let anti = center + (center - apex);
    match decide(TRIM, Margin::of(arc_trim_margin(r, a, apex, anti)), band) {
        Ok(Sign::Negative) => (q - a).norm().min((q - b).norm()),
        _ => circle,
    }
}

/// The distance from `q` to the annulus `inner ≤ |x − c| ≤ outer` of
/// the plane through `c` square to `axis`.
fn annulus_distance(
    q: Point3<Interval>,
    c: Point3<Interval>,
    axis: geom_core::Vec3<Interval>,
    (inner, outer): (Interval, Interval),
) -> Interval {
    let zero = Interval::zero();
    let n = axis / axis.norm();
    let w = q - c;
    let height = w.dot(n);
    let across = (w - n * height).norm();
    let off = (inner - across).max(across - outer).max(zero);
    (height.powi(2) + off.powi(2)).sqrt()
}
