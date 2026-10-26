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
//!    clear of that ball in metres, each on its own carrier
//!    (`splitting::containment::arc_distance_floor`: a line by its
//!    segment, a circle or an ellipse by its own arc, any other carrier
//!    by the ball its arc lies in, or else piece by piece). An edge's 3-D box is never read: a
//!    long diagonal edge's box covers every cell beside it. So is every
//!    singular point of the carrier, which a patch can be connected
//!    through without being connected in the face: a cone's apex, and
//!    on a torus the axis, where a spindle or horn torus meets itself.
//! 2. **Witness.** A connected patch that meets no point of the face's
//!    boundary and no singular point of its carrier lies wholly in one
//!    component of the carrier minus the boundary, so wholly in the face
//!    or wholly out of it. A seam is no boundary: the face lies on both
//!    sides of it. The centre station, placed by the face's
//!    point-in-face door ([`crate::contfp`] on a plane,
//!    [`crate::curved_face_containment`] on a curved carrier), says
//!    which.
//!
//! Anything else is [`CellRegion::Undecided`]: an edge not decided
//! clear (in the band, poisoned, or genuinely crossing), a station the
//! door places on the boundary or does not read, a refusal of either
//! door. Every one of those keeps the cell, which is the direction a
//! subdivision consumer's looseness has to run.

use geom::Surface;
use geom_core::interval::Interval;
use geom_core::{Band, Margin, Point3, Sign};

use crate::Body;
use crate::boolean::{FaceContainment, contfp, curved_face_containment};
use crate::entity::{FaceKey, LoopBoundary};
use crate::splitting::containment::{ArcFloor, arc_distance_floor};
use crate::validate::decide;

/// The rows a feature is decided clear of a patch's ball on: metres
/// from the ball's centre to the feature, less the ball's radius, one
/// row per reading so their populations stay apart. Only a definite
/// `Positive` clears.
///
/// A vertex's point.
const VERTEX: &str = "cell_region_vertex_gap";
/// A line edge's segment.
const SEGMENT: &str = "cell_region_segment_gap";
/// A circular or elliptic edge's arc floor.
const CONIC: &str = "cell_region_conic_gap";
/// The ball holding a spiric or spline edge's arc.
const BALL: &str = "cell_region_ball_gap";
/// A carrier's singular set: a cone's apex, a torus's axis.
const SINGULAR: &str = "cell_region_singular_gap";

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
        let mid = geom::mid_param(lo, hi);
        Interval::from_bounds(mid, mid)
    };
    let station = surface.eval(point(u), point(v));
    let reach = (surface.eval(span(u), span(v)) - station).norm();
    let clear = |row: &'static str, distance: Interval| {
        matches!(
            decide(row, Margin::of(distance - reach), band),
            Ok(Sign::Positive)
        )
    };
    if !(clear_of_singular(surface, station, &clear)
        && clear_of_boundary(body, face, station, &clear))
    {
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

/// Is the carrier's singular set decided clear of the ball: a cone's
/// apex, and a torus's axis, which holds every point where a spindle or
/// horn torus meets itself (a ring torus never comes near it). The
/// other carriers have none.
fn clear_of_singular(
    surface: &Surface<Interval>,
    station: Point3<Interval>,
    clear: &dyn Fn(&'static str, Interval) -> bool,
) -> bool {
    match *surface {
        Surface::Cone { apex, .. } => clear(SINGULAR, (station - apex).norm()),
        Surface::Torus { center, axis, .. } => {
            let n = axis / axis.norm();
            let w = station - center;
            clear(SINGULAR, (w - n * w.dot(n)).norm())
        }
        _ => true,
    }
}

/// Is every vertex of `face`, and every edge of it that is not a seam,
/// decided clear of the ball?
fn clear_of_boundary(
    body: &Body<Interval>,
    face: FaceKey,
    station: Point3<Interval>,
    clear: &dyn Fn(&'static str, Interval) -> bool,
) -> bool {
    let Some(f) = body.get_face(face) else {
        return false;
    };
    let on_face = |he| {
        body.get_half_edge(he)
            .and_then(|h| body.get_loop(h.parent_loop))
            .is_some_and(|l| l.face == face)
    };
    let vertex_clear = |vertex| {
        body.get_vertex(vertex)
            .and_then(|x| body.get_point(x.point))
            .is_some_and(|p| clear(VERTEX, (*p - station).norm()))
    };
    for &lk in core::iter::once(&f.outer).chain(&f.rings) {
        let Some(l) = body.get_loop(lk) else {
            return false;
        };
        let first = match l.boundary {
            LoopBoundary::Empty { vertex } => {
                if !vertex_clear(vertex) {
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
            if !vertex_clear(h.start) {
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
            let cleared =
                arc_distance_floor(curve.carrier(), curve.params(), station).is_some_and(|floor| {
                    let row = match floor {
                        ArcFloor::Segment(_) => SEGMENT,
                        ArcFloor::Conic { .. } => CONIC,
                        ArcFloor::Ball { .. } => BALL,
                    };
                    floor.clears(|d| clear(row, d))
                });
            if !cleared {
                return false;
            }
        }
    }
    true
}
