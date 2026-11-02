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
//!
//! Both doors read a body at any certifying scalar `T`. The station is
//! evaluated at `T`, its chart point entering as
//! [`Real::from_computed`](geom_core::Real::from_computed), and the
//! point-in-face doors place it at `T`. The ball is interval
//! arithmetic: the carrier, the station and every boundary feature are
//! lifted through [`Interval::from_certified`], which keeps every bound
//! at `T = Interval`.

use geom::Surface;
use geom_core::interval::Interval;
use geom_core::{Band, CertifiedBounds, Decide, Margin, Point3, Sign};

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
pub fn cell_region<T: Decide + CertifiedBounds>(
    body: &Body<T>,
    face: FaceKey,
    u: (f64, f64),
    v: (f64, f64),
    band: Band,
) -> CellRegion {
    let Some(stored) = body
        .get_face(face)
        .and_then(|f| body.get_surface(f.surface))
    else {
        return CellRegion::Undecided;
    };
    cell_region_lifted(
        body,
        face,
        (
            stored,
            &stored.map_scalar(Interval::from_certified),
            &lift_boundary(body, face),
        ),
        u,
        v,
        band,
    )
}

/// **A cell's centre station**: the midpoint of each parameter range,
/// the one station both a cell's region verdict and the minimum walk's
/// witness are read at.
pub(crate) fn centre(u: (f64, f64), v: (f64, f64)) -> (f64, f64) {
    (geom::mid_param(u.0, u.1), geom::mid_param(v.0, v.1))
}

/// [`cell_region`] with the face's stored carrier, its lift and its
/// lifted boundary ([`lift_boundary`]) in hand, for a caller that asks
/// many cells of one face.
///
/// The station is placed at `T` only for a cell already decided clear
/// of the boundary: the clear tests read its lift, which at `T` is the
/// lift of the station evaluated at `T`, bound for bound, so a cell that
/// stops there never builds a value of `T`.
pub(crate) fn cell_region_lifted<T: Decide + CertifiedBounds>(
    body: &Body<T>,
    face: FaceKey,
    (stored, surface, boundary): (&Surface<T>, &Surface<Interval>, &[BoundaryStop]),
    u: (f64, f64),
    v: (f64, f64),
    band: Band,
) -> CellRegion {
    let span = |(lo, hi): (f64, f64)| Interval::from_bounds(lo, hi);
    let (mu, mv) = centre(u, v);
    let station = surface.eval(
        <Interval as geom_core::Real>::from_computed(mu),
        <Interval as geom_core::Real>::from_computed(mv),
    );
    let reach = (surface.eval(span(u), span(v)) - station).norm();
    let clear = |row: &'static str, distance: Interval| {
        matches!(
            decide(row, Margin::of(distance - reach), band),
            Ok(Sign::Positive)
        )
    };
    if !(clear_of_singular(surface, station, &clear)
        && clear_of_boundary(boundary, station, &clear))
    {
        return CellRegion::Undecided;
    }
    let at = stored.eval(T::from_computed(mu), T::from_computed(mv));
    match place(body, face, stored, at, band) {
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
pub fn station_region<T: Decide + CertifiedBounds>(
    body: &Body<T>,
    face: FaceKey,
    (u, v): (f64, f64),
    band: Band,
) -> Option<FaceContainment> {
    let surface = body
        .get_face(face)
        .and_then(|f| body.get_surface(f.surface))?;
    let station = surface.eval(T::from_computed(u), T::from_computed(v));
    place(body, face, surface, station, band)
}

fn place<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    surface: &Surface<T>,
    station: Point3<T>,
    band: Band,
) -> Option<FaceContainment> {
    match *surface {
        Surface::Plane { normal, .. } => contfp(body, face, normal, station, band).ok(),
        _ => curved_face_containment(body, face, station, band)
            .ok()
            .flatten(),
    }
}

/// **A cell's ball**: a station of the cell's patch, and a radius about
/// it that holds the patch's enclosure. Both are interval arithmetic, so
/// a feature this decides clear of the ball meets no point of the patch.
///
/// It reads features already lifted to [`Interval`], on the same rows
/// [`cell_region`] clears them on, so the two readings of "this cell
/// meets no point of that edge" are one reading.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Ball {
    centre: Point3<Interval>,
    reach: Interval,
}

impl Ball {
    /// The ball about `centre` that holds the box `patch`.
    pub(crate) fn holding(centre: Point3<Interval>, patch: Point3<Interval>) -> Self {
        Self {
            centre,
            reach: (patch - centre).norm(),
        }
    }

    /// The radius, in metres.
    pub(crate) fn reach(self) -> Interval {
        self.reach
    }

    fn clear(self, row: &'static str, distance: Interval, band: Band) -> bool {
        matches!(
            decide(row, Margin::of(distance - self.reach), band),
            Ok(Sign::Positive)
        )
    }

    /// Is the point decided clear of the ball?
    pub(crate) fn clear_of_point(self, p: Point3<Interval>, band: Band) -> bool {
        self.clear(VERTEX, (p - self.centre).norm(), band)
    }

    /// Is the arc of `carrier` over `params` decided clear of the ball?
    pub(crate) fn clear_of_arc(
        self,
        carrier: &geom::Curve3<Interval>,
        params: (Interval, Interval),
        band: Band,
    ) -> bool {
        arc_distance_floor(carrier, params, self.centre).is_some_and(|floor| {
            let row = match floor {
                ArcFloor::Segment(_) => SEGMENT,
                ArcFloor::Conic { .. } => CONIC,
                ArcFloor::Ball { .. } => BALL,
            };
            floor.clears(|d| self.clear(row, d, band))
        })
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

/// One stop of a face's boundary walk, lifted to the interval scalar:
/// what [`clear_of_boundary`] decides, in the order it decides it.
#[derive(Debug, Clone)]
pub(crate) enum BoundaryStop {
    /// A vertex's point.
    Vertex(Point3<Interval>),
    /// A non-seam edge's carrier and parameter span.
    Arc(Box<(geom::Curve3<Interval>, (Interval, Interval))>),
    /// Something the walk could not read: never clear.
    Unreadable,
}

/// Every vertex of `face`, and every edge of it that is not a seam, in
/// walk order and lifted, ending at the first thing the walk could not
/// read. A sweep lifts a face's boundary once and asks it of every
/// cell; the lift reads no bracket and decides nothing.
pub(crate) fn lift_boundary<T: Decide + CertifiedBounds>(
    body: &Body<T>,
    face: FaceKey,
) -> Vec<BoundaryStop> {
    let mut stops = Vec::new();
    let Some(f) = body.get_face(face) else {
        stops.push(BoundaryStop::Unreadable);
        return stops;
    };
    let on_face = |he| {
        body.get_half_edge(he)
            .and_then(|h| body.get_loop(h.parent_loop))
            .is_some_and(|l| l.face == face)
    };
    let vertex = |vertex| {
        body.get_vertex(vertex)
            .and_then(|x| body.get_point(x.point))
            .map_or(BoundaryStop::Unreadable, |p| {
                BoundaryStop::Vertex(p.map(Interval::from_certified))
            })
    };
    for &lk in core::iter::once(&f.outer).chain(&f.rings) {
        let Some(l) = body.get_loop(lk) else {
            stops.push(BoundaryStop::Unreadable);
            return stops;
        };
        let first = match l.boundary {
            LoopBoundary::Empty { vertex: v } => {
                stops.push(vertex(v));
                continue;
            }
            LoopBoundary::Cycle { first } => first,
        };
        let Some(cycle) = body.loop_cycle(first) else {
            stops.push(BoundaryStop::Unreadable);
            return stops;
        };
        for he in cycle {
            let Some(h) = body.get_half_edge(he) else {
                stops.push(BoundaryStop::Unreadable);
                return stops;
            };
            stops.push(vertex(h.start));
            let Some(e) = body.get_edge(h.edge) else {
                stops.push(BoundaryStop::Unreadable);
                return stops;
            };
            if on_face(e.he_plus) && on_face(e.he_minus) {
                continue;
            }
            let Some(geom) = body.get_curve_geom(e.curve) else {
                stops.push(BoundaryStop::Unreadable);
                return stops;
            };
            // Null scaffolding has no carrier: its locus is its vertex,
            // read above.
            let Some(curve) = geom.certified() else {
                continue;
            };
            let (t0, t1) = curve.params();
            stops.push(BoundaryStop::Arc(Box::new((
                curve.carrier().map_scalar(Interval::from_certified),
                (Interval::from_certified(t0), Interval::from_certified(t1)),
            ))));
        }
    }
    stops
}

/// Is every stop of a face's lifted boundary decided clear of the
/// ball? The first stop that is not ends the walk.
fn clear_of_boundary(
    stops: &[BoundaryStop],
    station: Point3<Interval>,
    clear: &dyn Fn(&'static str, Interval) -> bool,
) -> bool {
    stops.iter().all(|stop| match stop {
        BoundaryStop::Vertex(p) => clear(VERTEX, (*p - station).norm()),
        BoundaryStop::Arc(arc) => arc_distance_floor(&arc.0, arc.1, station).is_some_and(|floor| {
            let row = match floor {
                ArcFloor::Segment(_) => SEGMENT,
                ArcFloor::Conic { .. } => CONIC,
                ArcFloor::Ball { .. } => BALL,
            };
            floor.clears(|d| clear(row, d))
        }),
        BoundaryStop::Unreadable => false,
    })
}
