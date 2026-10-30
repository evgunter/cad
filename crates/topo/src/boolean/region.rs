//! **What the region readings share**: the skeleton
//! [`super::sphere_region`], [`super::torus_region`] and
//! [`super::ruled_region`] read a trimmed curved face with. The skeleton
//! gathers the boundary, reads the point against it first, walks the
//! paths and reads each leg to its closest crossing; a door supplies only
//! its surface's legs ([`Legs`], and [`Paths`] for a roster of them), its
//! landings, and its own rows and crossing arithmetic.

use geom_brep::roots::{RootRefusal, Roots};
use geom_core::{Band, Decide, Indeterminate, Margin, Point3, Sign, Vec3};

use super::circle_roots::FirstHarmonicRoots;
use super::solid_contain::PointInSolidError;
use crate::body::Body;
use crate::entity::{EdgeKey, FaceKey, LoopBoundary, LoopKey};
use crate::ray_walk::{self, Crossings, RayFault};
use crate::splitting::containment::{ConicArc, ConicArcError, ConicHit, ConicRows};
use crate::splitting::{PointInLoopError, Uncrossable};
use crate::validate::decide;

/// Where along each edge an aimed path's target lies, as shares of its
/// span.
pub(crate) const TARGET_SHARES: [f64; 3] = [0.5, 0.25, 0.75];

/// Why a region reading gave no answer. Each door that reads a region
/// names the face it read.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum RegionRefusal {
    /// The point is in the band of a boundary edge; or no path decided,
    /// and the first one set aside read in band.
    Escalated(Indeterminate),
    /// No path settled ([`crate::ray_walk::NoRaySettled`]): each grazed or
    /// gave nothing to read. `outer` is the face's outer loop, the loop
    /// the solid door names as the region walk's.
    RayExhausted {
        /// The face's outer loop.
        outer: LoopKey,
    },
    /// A boundary arc winds definitely past a whole turn, which bounds
    /// no region.
    WoundPastPeriod,
    /// The boundary reading of an edge on its own carrier ran out of depth
    /// or budget before it placed the point: no reading in the band, and
    /// no path starts from a point it has not placed.
    Unreadable(Uncrossable),
}

impl RegionRefusal {
    /// The refusal as the point-in-solid door states it, for a point on
    /// `face`'s carrier: a region walk no path of which settled is that
    /// face's loop walk's, as a planar face's is
    /// ([`PointInSolidError::Loop`]) — not the solid's own sweep, whose
    /// exhaustion says the point is off the solid's boundary.
    pub(crate) fn of_face(self, face: FaceKey) -> PointInSolidError {
        match self {
            Self::Escalated(diag) => PointInSolidError::Escalated { face, diag },
            Self::RayExhausted { outer } => {
                PointInSolidError::Loop(PointInLoopError::RayExhausted { r#loop: outer })
            }
            Self::WoundPastPeriod => PointInSolidError::CorruptFace { face },
            Self::Unreadable(cause) => PointInSolidError::EdgeCarrierUnsupported { face, cause },
        }
    }
}

/// **Whether `p` is on the circle arc `circle` over `(t0, t1)`**, read on
/// the arc's own carrier ([`ConicArc::hit`]): on the arc or within the
/// band of one of its ends is `true`; in the band of it refuses. `outer`
/// is named if the reading cannot place `p`.
///
/// # Errors
///
/// [`RegionRefusal::Escalated`] in band; [`RegionRefusal::WoundPastPeriod`]
/// for an arc wound past a turn; [`RegionRefusal::RayExhausted`] where the
/// reading leaves `p` unplaced (a circle's nearest foot walks nothing, so
/// this does not arise; were it to, no path starts from `p`).
pub(crate) fn circle_edge_hit<T: Decide>(
    circle: &geom::Curve3<T>,
    turn: Option<&geom_brep::CertifiedTurn<T>>,
    rows: ConicRows,
    p: Point3<T>,
    outer: LoopKey,
    band: Band,
) -> Result<bool, RegionRefusal> {
    let k = match ConicArc::of(circle, turn, rows, band) {
        Ok(Some(k)) => k,
        Err(ConicArcError::Escalated(e)) => return Err(RegionRefusal::Escalated(e.diag)),
        Ok(None) | Err(ConicArcError::WoundPastPeriod) => {
            return Err(RegionRefusal::WoundPastPeriod);
        }
    };
    match k
        .hit(p, band)
        .map_err(|e| RegionRefusal::Escalated(e.diag))?
    {
        ConicHit::On(_) | ConicHit::End => Ok(true),
        ConicHit::Off | ConicHit::Carrier => Ok(false),
        ConicHit::Unsettled => Err(RegionRefusal::RayExhausted { outer }),
    }
}

/// A path's reason to be set aside.
pub(crate) type Fault = RayFault<RegionRefusal>;

/// An in-band margin about one path, setting it aside.
pub(crate) fn in_band(diag: Indeterminate) -> Fault {
    RayFault::InBand(RegionRefusal::Escalated(diag))
}

/// **A root `t` against its edge's span `(t0, t1)`**, under `row` at
/// `lever` (the edge's speed, or a lower bound on it): `None` definitely
/// past an end; `Some(sideless)` within, `sideless` where it lies within
/// the band of an end, where no side is read.
///
/// # Errors
///
/// An in-band end sets the path aside.
pub(crate) fn span<T: Decide>(
    row: &'static str,
    t: T,
    (t0, t1): (T, T),
    lever: T,
    band: Band,
) -> Result<Option<bool>, Fault> {
    let start = decide(row, Margin::of(lever * (t - t0)), band).map_err(in_band)?;
    let end = decide(row, Margin::of(lever * (t1 - t)), band).map_err(in_band)?;
    Ok((start != Sign::Negative && end != Sign::Negative)
        .then_some(start == Sign::Zero || end == Sign::Zero))
}

/// A crossing as the verdict reads it at the closest: the path's unit
/// heading there and the face's side of the edge (`N × τ`).
pub(crate) struct Hit<T: geom_core::Real> {
    pub(crate) heading: Vec3<T>,
    pub(crate) face_side: Vec3<T>,
}

/// **Whether a path's start is in the face**, read at its closest
/// crossing: the heading against the face's side there, a sine levered by
/// `lever`, under `rows = (order, cross)`. Leaving the face (`Negative`)
/// puts the start in it.
///
/// # Errors
///
/// A path that meets nothing gives nothing to read
/// ([`RayFault::Unread`]); a tie, a sideless closest crossing or a
/// tangential one grazes; an in-band one sets the path aside.
pub(crate) fn side_at_closest<T: Decide>(
    crossings: Crossings<T, Hit<T>>,
    (order, cross): (&'static str, &'static str),
    lever: T,
    band: Band,
) -> Result<bool, Fault> {
    let hit = crossings
        .closest(order, band, |diag, _, _| RegionRefusal::Escalated(diag))?
        .ok_or(RayFault::Unread)?;
    match decide(
        cross,
        Margin::of(lever * hit.heading.dot(hit.face_side)),
        band,
    )
    .map_err(in_band)?
    {
        Sign::Negative => Ok(true),
        Sign::Positive => Ok(false),
        Sign::Zero => Err(RayFault::Graze),
    }
}

/// An angle in `(−π, π]` as a turn in `[0, 2π)`.
fn turn<T: Decide>(s: T) -> T {
    (T::zero() - s).select_le_zero(s, s + T::tau())
}

/// A first-harmonic root door's answer on one leg: its roots, none for a miss; an edge
/// in the leg's plane or a tangency grazes.
pub(crate) fn found<T: Decide>(
    roots: Result<FirstHarmonicRoots<T>, RootRefusal>,
) -> Result<Vec<T>, Fault> {
    match roots {
        Ok(Roots::Miss) => Ok(Vec::new()),
        Ok(Roots::Certified(found)) => Ok(found.roots().to_vec()),
        // A touch of the leg is the probe's, as a limit is; a fault grazes
        // too (`work/splitdoor/ray-lanes-graze-on-a-root-finders-fault.md`).
        Ok(Roots::On(_))
        | Err(RootRefusal::Touch(_) | RootRefusal::Unsettled(_) | RootRefusal::Fault(_)) => {
            Err(RayFault::Graze)
        }
        Err(RootRefusal::Escalated(diag)) => Err(in_band(diag)),
    }
}

/// Where a leg crosses an edge: the crossing's point, the edge's
/// parameter there, and whether it lies within the band of an end of the
/// edge, where no side is read.
pub(crate) type Landing<T> = (Point3<T>, T, bool);

/// One boundary edge of a region over its certified span `[t0, t1]`,
/// on its certified carrier, of a kind its door admits ([`boundary`]).
#[derive(Clone, Debug)]
pub(crate) struct RegionEdge<T: geom_core::Real> {
    pub(crate) edge: EdgeKey,
    /// The loop the edge bounds.
    pub(crate) r#loop: LoopKey,
    pub(crate) carrier: geom::Curve3<T>,
    pub(crate) t0: T,
    pub(crate) t1: T,
    /// The certified turn of `[t0, t1]` on a periodic carrier
    /// ([`geom_brep::CertifiedTurn`]), the interval and extent a conic
    /// reading reads; `None` on a line.
    pub(crate) turn: Option<geom_brep::CertifiedTurn<T>>,
    /// Whether the loop runs `t0 → t1` (its `he_plus`).
    pub(crate) forward: bool,
}

impl<T: Decide> RegionEdge<T> {
    pub(crate) fn at(&self, t: T) -> Point3<T> {
        self.carrier.eval(t)
    }

    /// The loop's unit traversal direction at `t`.
    pub(crate) fn traversal(&self, t: T) -> Vec3<T> {
        let d = self.carrier.deriv(t);
        let d = d / d.norm();
        if self.forward { d } else { -d }
    }
}

/// What a gather makes of null scaffolding: an edge with no certified
/// curve whose two ends are one point, a zero-length copy of a vertex.
#[derive(Clone, Copy, Debug)]
pub(crate) enum Scaffolding {
    /// Left out: it bounds nothing.
    Skipped,
    /// No region, as for any edge with no certified curve.
    Refused,
}

/// **`face`'s boundary**: every edge of its outer loop and its rings, in
/// loop order, each on its certified carrier. `None` where a carrier's
/// kind is not one of `admits`, or where an edge has no certified curve: an uncertified edge between two points is a gap in
/// the boundary no reading may step over, and null scaffolding is as
/// `scaffolding` says. An edge the face lies on both sides of
/// ([`Body::both_sides_face`]: a seam, a wrap edge, a spur) is no
/// boundary of it and is left out, so the edges kept are the face's
/// surface boundary ([`crate::surface_boundary::surface_cycles`]), in
/// loop order; none for a face that covers its surface.
///
/// # Errors
///
/// [`PointInSolidError::CorruptFace`] for a key that does not resolve or
/// an outer loop with no cycle.
///
/// # Panics
///
/// Where a boundary edge's curve does not resolve (D2 row 4): a torn
/// curve is not null scaffolding. The links hold at rest and, on the
/// reduction's working copies, by [`crate::live::OPERATORS_KEEP_LINKS`].
pub(crate) fn boundary<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    scaffolding: Scaffolding,
    admits: &[geom::CurveKind],
) -> Result<Option<Vec<RegionEdge<T>>>, PointInSolidError> {
    let corrupt = || PointInSolidError::CorruptFace { face };
    let f = body.get_face(face).ok_or_else(corrupt)?;
    let mut edges = Vec::new();
    for (i, &lk) in core::iter::once(&f.outer).chain(&f.rings).enumerate() {
        let first = match body.get_loop(lk).ok_or_else(corrupt)?.boundary {
            LoopBoundary::Cycle { first } => first,
            // A lone ring vertex bounds no area; an outer loop must.
            LoopBoundary::Empty { .. } if i > 0 => continue,
            LoopBoundary::Empty { .. } => return Err(corrupt()),
        };
        for he in body.loop_cycle(first).ok_or_else(corrupt)? {
            let edge_key = body.get_half_edge(he).ok_or_else(corrupt)?.edge;
            if body
                .both_sides_face(edge_key)
                .ok_or_else(corrupt)?
                .is_some()
            {
                continue;
            }
            let edge = body.get_edge(edge_key).ok_or_else(corrupt)?;
            let Some(curve) = body.edge_curve_linked(edge_key, edge).certified() else {
                if let Scaffolding::Skipped = scaffolding {
                    let point = |h| {
                        body.get_half_edge(h)
                            .and_then(|h| body.get_vertex(h.start))
                            .map(|v| v.point)
                    };
                    if point(edge.he_plus).ok_or_else(corrupt)?
                        == point(edge.he_minus).ok_or_else(corrupt)?
                    {
                        continue;
                    }
                }
                return Ok(None);
            };
            if !admits.contains(&curve.carrier().kind()) {
                return Ok(None);
            }
            let (t0, t1) = curve.params();
            edges.push(RegionEdge {
                edge: edge_key,
                r#loop: lk,
                carrier: curve.carrier().clone(),
                t0,
                t1,
                turn: curve.turn().copied(),
                forward: he == edge.he_plus,
            });
        }
    }
    Ok(Some(edges))
}

/// Three points of every edge, in loop order, at [`TARGET_SHARES`] of
/// its span: the aimed paths' targets.
pub(crate) fn targets<T: Decide>(edges: &[RegionEdge<T>]) -> impl Iterator<Item = Point3<T>> + '_ {
    edges.iter().flat_map(|edge| {
        TARGET_SHARES
            .into_iter()
            .map(|share| edge.at(edge.t0 + (edge.t1 - edge.t0) * T::from_f64(share)))
    })
}

/// **A trimmed curved face's region**, as the skeleton reads it: its
/// boundary and, per edge, the boundary reading on the edge's own
/// carrier.
pub(crate) trait Region<T: Decide> {
    /// The face's boundary ([`boundary`]).
    fn edges(&self) -> &[RegionEdge<T>];

    /// The face's outer loop, named by an exhaustion.
    fn outer(&self) -> LoopKey;

    /// Whether `p` is on `edge`, or within the band of one of its ends,
    /// read on the edge's own carrier.
    ///
    /// # Errors
    ///
    /// In the band of the edge, or a reading that cannot place `p`.
    fn on_edge(
        &self,
        edge: &RegionEdge<T>,
        p: Point3<T>,
        band: Band,
    ) -> Result<bool, RegionRefusal>;

    /// **The boundary pre-pass**: whether `p` is on an edge, read edge by
    /// edge in loop order before any path. A crossing's place along a
    /// path is no measure of `p`'s distance from the edge it crosses, so
    /// the paths never answer it.
    ///
    /// # Errors
    ///
    /// The first edge's refusal ([`Region::on_edge`]).
    fn on_boundary(&self, p: Point3<T>, band: Band) -> Result<bool, RegionRefusal> {
        for edge in self.edges() {
            if self.on_edge(edge, p, band)? {
                return Ok(true);
            }
        }
        Ok(false)
    }
}

/// **A leg's course**: the line or circle it runs along from its start
/// at place `0`, places measured in metres along it.
pub(crate) trait Course<T: Decide> {
    /// Where `y`, a crossing on the course, lies along it, on the row
    /// `at`: `Some(place)` ahead of the start, `None` behind it.
    ///
    /// # Errors
    ///
    /// A crossing at the start grazes; an in-band one sets the path aside.
    fn place(&self, y: Point3<T>, at: &'static str, band: Band) -> Result<Option<T>, Fault>;

    /// Where `y`, an end of an edge lying along the course, lies along
    /// it: as a crossing does ([`Course::place`]), unless the course
    /// says otherwise.
    ///
    /// # Errors
    ///
    /// As [`Course::place`]'s.
    fn reach(&self, y: Point3<T>, at: &'static str, band: Band) -> Result<Option<T>, Fault> {
        self.place(y, at, band)
    }

    /// The unit direction of travel at `y`.
    fn heading(&self, y: Point3<T>) -> Vec3<T>;
}

/// Where a bounded leg stops: its place along the leg, and the row a
/// place is read against it on.
#[derive(Clone, Copy)]
pub(crate) struct End<T: geom_core::Real> {
    pub(crate) place: T,
    pub(crate) row: &'static str,
}

/// One leg of a path along its course `C`, ending at `end` where it is
/// bounded.
#[derive(Clone, Copy)]
pub(crate) struct Leg<T: geom_core::Real, C> {
    pub(crate) course: C,
    pub(crate) end: Option<End<T>>,
}

impl<T: Decide, C: Course<T>> Leg<T, C> {
    /// A leg running its whole course.
    pub(crate) fn whole(course: C) -> Self {
        Self { course, end: None }
    }

    /// Whether a place ahead of the start lies short of the leg's end.
    ///
    /// # Errors
    ///
    /// A place at the end grazes; an in-band one sets the path aside.
    fn short_of_end(&self, s: T, band: Band) -> Result<bool, Fault> {
        let Some(end) = self.end else {
            return Ok(true);
        };
        match decide(end.row, Margin::of(end.place - s), band).map_err(in_band)? {
            Sign::Positive => Ok(true),
            Sign::Negative => Ok(false),
            Sign::Zero => Err(RayFault::Graze),
        }
    }

    /// Where the landing `y` lies along the leg, on the row `at`:
    /// `Some(place)` ahead of its start and short of its end, `None` off
    /// the leg.
    ///
    /// # Errors
    ///
    /// A graze or an in-band reading sets the path aside.
    pub(crate) fn place(
        &self,
        y: Point3<T>,
        at: &'static str,
        band: Band,
    ) -> Result<Option<T>, Fault> {
        let Some(s) = self.course.place(y, at, band)? else {
            return Ok(None);
        };
        Ok(self.short_of_end(s, band)?.then_some(s))
    }

    /// **An edge lying along the leg's own course**: the leg runs along
    /// it if it reaches it, which grazes. A leg reaches every point of its
    /// course from its start to its end; neither end of the edge is the
    /// leg's start, which the boundary pre-pass placed off the edge.
    ///
    /// # Errors
    ///
    /// The leg reaching the edge grazes; an in-band reading sets the path
    /// aside.
    pub(crate) fn along(
        &self,
        edge: &RegionEdge<T>,
        at: &'static str,
        band: Band,
    ) -> Result<(), Fault> {
        let mut behind = 0;
        let mut past = 0;
        for t in [edge.t0, edge.t1] {
            let Some(s) = self.course.reach(edge.at(t), at, band)? else {
                behind += 1;
                continue;
            };
            if self.short_of_end(s, band)? {
                return Err(RayFault::Graze);
            }
            past += 1;
        }
        if behind == 2 || past == 2 {
            Ok(())
        } else {
            Err(RayFault::Graze)
        }
    }
}

/// **A region read along legs**: where a leg meets the boundary, and the
/// face's side there.
pub(crate) trait Legs<T: Decide>: Region<T> {
    type Course: Course<T> + Copy;

    /// The row a crossing's place along a leg is read on.
    const AT: &'static str;

    /// The rows the closest crossing is read under: `(order, cross)`.
    const SIDE: (&'static str, &'static str);

    /// The lever of the closest crossing's sine.
    fn lever(&self) -> T;

    /// Where `leg` meets `edge`'s carrier. Where along the leg each
    /// landing lies is the skeleton's to read ([`Leg::place`]).
    ///
    /// # Errors
    ///
    /// A graze or an in-band reading sets the path aside.
    fn landings(
        &self,
        edge: &RegionEdge<T>,
        leg: &Leg<T, Self::Course>,
        band: Band,
    ) -> Result<Vec<Landing<T>>, Fault>;

    /// The face's unit outward normal at `y`.
    fn outward(&self, y: Point3<T>) -> Vec3<T>;
}

/// A path to read: a leg from `p`, or two through a corner.
#[derive(Clone, Copy)]
pub(crate) enum Path<T: geom_core::Real> {
    /// `p`'s parallel (`parallel`) or its leg of the other family, run
    /// `forward` or back.
    Single { parallel: bool, forward: bool },
    /// A first leg from `p` — its parallel where `parallel_first` — to a
    /// corner, then the other family's leg through the corner, which
    /// passes `target`.
    Aimed {
        target: Point3<T>,
        parallel_first: bool,
        first_forward: bool,
        then_forward: bool,
    },
}

/// **A region read along paths of legs**: a surface's two families of
/// curves through a point, its parallels and one other.
pub(crate) trait Paths<T: Decide>: Legs<T> {
    /// `p`'s own leg of a family ([`Path::Single`]).
    ///
    /// # Errors
    ///
    /// A leg the door cannot frame sets the path aside.
    fn single(
        &self,
        p: Point3<T>,
        parallel: bool,
        forward: bool,
        band: Band,
    ) -> Result<Leg<T, Self::Course>, Fault>;

    /// An aimed path's legs ([`Path::Aimed`]): the first, whose end is
    /// its corner, where the second starts; and the second, built only
    /// where the first leg leaves the path undecided.
    ///
    /// # Errors
    ///
    /// A leg the door cannot frame sets the path aside.
    #[allow(clippy::too_many_arguments)] // one path's spec, unpacked
    #[allow(clippy::type_complexity)] // the first course, its corner, the second leg
    fn aimed(
        &self,
        p: Point3<T>,
        target: Point3<T>,
        parallel_first: bool,
        first_forward: bool,
        then_forward: bool,
        band: Band,
    ) -> Result<(Self::Course, End<T>, impl FnOnce() -> Leg<T, Self::Course>), Fault>;
}

/// **The path walk** ([`ray_walk::walk`]): `p`'s parallel forward and
/// back and its other leg forward and back, then, at every target
/// ([`targets`]), the aimed paths turning at a corner on either family
/// first, each with its legs run each way. Whether `p` is in the face.
///
/// # Errors
///
/// [`RegionRefusal::RayExhausted`] naming the face's outer loop where no
/// path settled, or the first in-band one's refusal.
pub(crate) fn walk<T: Decide, D: Paths<T>>(
    door: &D,
    p: Point3<T>,
    band: Band,
) -> Result<bool, RegionRefusal> {
    let both = [(true, true), (true, false), (false, true), (false, false)];
    let singles = both.map(|(parallel, forward)| Path::Single { parallel, forward });
    let aimed = targets(door.edges()).flat_map(move |target| {
        [true, false].into_iter().flat_map(move |parallel_first| {
            both.map(move |(first_forward, then_forward)| Path::Aimed {
                target,
                parallel_first,
                first_forward,
                then_forward,
            })
        })
    });
    ray_walk::walk(
        singles.into_iter().chain(aimed),
        |path| read_path(door, p, path, band),
        || RegionRefusal::RayExhausted {
            outer: door.outer(),
        },
    )
}

/// One path from `p`: whether `p` is in the face.
pub(crate) fn read_path<T: Decide, D: Paths<T>>(
    door: &D,
    p: Point3<T>,
    path: Path<T>,
    band: Band,
) -> Result<bool, Fault> {
    match path {
        Path::Single { parallel, forward } => {
            read_leg(door, &door.single(p, parallel, forward, band)?, band)
        }
        Path::Aimed {
            target,
            parallel_first,
            first_forward,
            then_forward,
        } => {
            let (first, corner, then) =
                door.aimed(p, target, parallel_first, first_forward, then_forward, band)?;
            let first = Leg {
                course: first,
                end: Some(corner),
            };
            let mut crossings = Crossings::new();
            // A crossing before the corner is closer than any past it,
            // so the second leg is read only where the first meets
            // nothing, and its crossings are ordered among themselves by
            // their places along it.
            if !offer(door, &first, &mut crossings, band)? {
                offer(door, &then(), &mut crossings, band)?;
            }
            side(door, crossings, band)
        }
    }
}

/// A path of the one leg `leg`: whether its start is in the face.
pub(crate) fn read_leg<T: Decide, D: Legs<T>>(
    door: &D,
    leg: &Leg<T, D::Course>,
    band: Band,
) -> Result<bool, Fault> {
    let mut crossings = Crossings::new();
    offer(door, leg, &mut crossings, band)?;
    side(door, crossings, band)
}

/// A path's verdict at its closest crossing. A path meeting no edge
/// lies wholly on one side.
fn side<T: Decide, D: Legs<T>>(
    door: &D,
    crossings: Crossings<T, Hit<T>>,
    band: Band,
) -> Result<bool, Fault> {
    side_at_closest(crossings, D::SIDE, door.lever(), band)
}

/// **The crossing offer**: every crossing of `leg` with the boundary,
/// edge by edge, at its place along the leg ([`Leg::place`]). Each edge's landings are all read
/// ([`Legs::landings`]) before any of them is placed. Whether one was
/// offered.
fn offer<T: Decide, D: Legs<T>>(
    door: &D,
    leg: &Leg<T, D::Course>,
    crossings: &mut Crossings<T, Hit<T>>,
    band: Band,
) -> Result<bool, Fault> {
    let mut offered = false;
    for edge in door.edges() {
        for (y, t, sideless) in door.landings(edge, leg, band)? {
            let Some(s) = leg.place(y, D::AT, band)? else {
                continue;
            };
            crossings.push(
                s,
                Hit {
                    heading: leg.course.heading(y),
                    face_side: door.outward(y).cross(edge.traversal(t)),
                },
                sideless,
            );
            offered = true;
        }
    }
    Ok(offered)
}

/// A leg round a circle: `center + radius·(e1 cos s + e2 sin s)` in the
/// plane of unit normal `normal`, `e2 = normal × e1`, from its start at
/// `s = 0` along increasing `s`.
#[derive(Clone, Copy)]
pub(crate) struct Round<T: geom_core::Real> {
    pub(crate) center: Point3<T>,
    pub(crate) normal: Vec3<T>,
    pub(crate) radius: T,
    pub(crate) e1: Vec3<T>,
    pub(crate) e2: Vec3<T>,
}

impl<T: Decide> Round<T> {
    pub(crate) fn at(&self, s: T) -> Point3<T> {
        let (sn, cs) = s.sin_cos();
        self.center + (self.e1 * cs + self.e2 * sn) * self.radius
    }

    /// The angle of `y`, a point of the circle, from the start, in
    /// `(−π, π]`.
    fn angle(&self, y: Point3<T>) -> T {
        let w = y - self.center;
        w.dot(self.e2).atan2(w.dot(self.e1))
    }

    /// The arc length from the start round to `y`, a point of the
    /// circle, in `[0, 2π·radius)`.
    pub(crate) fn arc(&self, y: Point3<T>) -> T {
        self.radius * turn(self.angle(y))
    }
}

/// Places round the circle are arc lengths in `[0, 2π·radius)`: a
/// circle has no behind.
impl<T: Decide> Course<T> for Round<T> {
    /// Read the shorter way round ([`ray_walk::apart`]), then as a turn
    /// ahead.
    fn place(&self, y: Point3<T>, at: &'static str, band: Band) -> Result<Option<T>, Fault> {
        let s = self.angle(y);
        ray_walk::apart(at, Margin::of((self.radius * s).abs()), band)
            .map_err(|f| f.map(RegionRefusal::Escalated))?;
        Ok(Some(self.arc(y)))
    }

    /// Read without a decision. Only a first leg is bounded round a
    /// circle, and it starts at the query, which the boundary pre-pass
    /// placed off every edge; an unbounded leg runs along an edge on its
    /// circle wherever the edge lies.
    fn reach(&self, y: Point3<T>, _: &'static str, _: Band) -> Result<Option<T>, Fault> {
        Ok(Some(self.arc(y)))
    }

    fn heading(&self, y: Point3<T>) -> Vec3<T> {
        let h = self.normal.cross(y - self.center);
        h / h.norm()
    }
}

/// A leg along the line from `start` in unit direction `dir`.
#[derive(Clone, Copy)]
pub(crate) struct Line<T: geom_core::Real> {
    pub(crate) start: Point3<T>,
    pub(crate) dir: Vec3<T>,
}

impl<T: Decide> Course<T> for Line<T> {
    fn place(&self, y: Point3<T>, at: &'static str, band: Band) -> Result<Option<T>, Fault> {
        let s = (y - self.start).dot(self.dir);
        match ray_walk::advance(at, Margin::of(s), band)
            .map_err(|f| f.map(RegionRefusal::Escalated))?
        {
            Sign::Negative => Ok(None),
            _ => Ok(Some(s)),
        }
    }

    fn heading(&self, _: Point3<T>) -> Vec3<T> {
        self.dir
    }
}

/// `a`'s distance from the axis through `base` along unit `axis`, its
/// height from `base`, and the unit radial direction.
pub(crate) fn polar<T: Decide>(base: Point3<T>, axis: Vec3<T>, a: Point3<T>) -> (T, T, Vec3<T>) {
    let w = a - base;
    let h = w.dot(axis);
    let radial = w - axis * h;
    let rho = radial.norm();
    (rho, h, radial / rho)
}

/// The parallel through `a` about that axis, run `forward` (right-handed
/// about `axis`) or back.
pub(crate) fn parallel<T: Decide>(
    base: Point3<T>,
    axis: Vec3<T>,
    a: Point3<T>,
    forward: bool,
) -> Round<T> {
    let (rho, h, e1) = polar(base, axis, a);
    let normal = if forward { axis } else { -axis };
    Round {
        center: base + axis * h,
        normal,
        radius: rho,
        e1,
        e2: normal.cross(e1),
    }
}

/// `face` with null scaffolding added to its outer loop: a null edge
/// fanned out of the vertex its first half-edge leaves.
#[cfg(test)]
#[allow(clippy::unwrap_used)]
pub(crate) fn with_null_edge(body: &mut Body<f64>, face: FaceKey) {
    let outer = body.get_face(face).unwrap().outer;
    let LoopBoundary::Cycle { first } = body.get_loop(outer).unwrap().boundary else {
        unreachable!("the fixture's outer loop is a cycle")
    };
    body.mev_null(
        crate::construction::MevSite::Fan {
            he1: first,
            he2: first,
        },
        crate::construction::NewVertexSide::Above,
    )
    .unwrap();
}

/// A fixture edge's turn over `(t0, t1)`: a unit circle's arc over that
/// interval, certified, whose interval and extent are what a conic
/// reading of the fixture reads (its own carrier supplies the rest).
#[cfg(test)]
pub(crate) fn fixture_turn(t0: f64, t1: f64) -> Option<geom_brep::CertifiedTurn<f64>> {
    let band = Band::new(1e-9, 1e-8).ok()?;
    let circle = geom::Curve3::Circle {
        center: Point3::origin(),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: 1.0,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let spec = geom_brep::EdgeCurveSpec::arc_of_circle(circle.clone(), t0, t1)?;
    let (start, end) = (circle.eval(t0), circle.eval(t1));
    geom_brep::EdgeCurve::certify(
        spec,
        start,
        end,
        |_| None,
        1.0,
        band,
        geom_brep::NurbsLane::certified(),
    )
    .ok()?
    .turn()
    .copied()
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use core::f64::consts::{FRAC_PI_2, PI};
    use slotmap::KeyData;

    fn b() -> Band {
        Band::new(1e-9, 1e-8).unwrap()
    }

    const AT: &str = "test_region_at";

    /// A leg ending at `end`, or running its whole course.
    fn leg<C>(course: C, end: Option<f64>) -> Leg<f64, C> {
        Leg {
            course,
            end: end.map(|place| End {
                place,
                row: "test_region_end",
            }),
        }
    }

    fn edge(carrier: geom::Curve3<f64>, (t0, t1): (f64, f64)) -> RegionEdge<f64> {
        RegionEdge {
            edge: EdgeKey::from(KeyData::from_ffi(1)),
            r#loop: LoopKey::default(),
            carrier,
            t0,
            t1,
            turn: fixture_turn(t0, t1),
            forward: true,
        }
    }

    /// The `x` axis from the origin, the line every line row runs on.
    fn x_line() -> Line<f64> {
        Line {
            start: Point3::origin(),
            dir: Vec3::new(1.0, 0.0, 0.0),
        }
    }

    /// The segment `[x0, x1]` of the `x` axis.
    fn x_segment(x0: f64, x1: f64) -> RegionEdge<f64> {
        let carrier = geom::Curve3::Line {
            origin: Point3::origin(),
            dir: Vec3::new(1.0, 0.0, 0.0),
        };
        edge(carrier, (x0, x1))
    }

    /// The unit circle about `z`, run from `x` toward `y`.
    fn unit_round() -> Round<f64> {
        Round {
            center: Point3::origin(),
            normal: Vec3::new(0.0, 0.0, 1.0),
            radius: 1.0,
            e1: Vec3::new(1.0, 0.0, 0.0),
            e2: Vec3::new(0.0, 1.0, 0.0),
        }
    }

    /// The arc of [`unit_round`]'s circle over `[t0, t1]`.
    fn unit_arc(t0: f64, t1: f64) -> RegionEdge<f64> {
        let carrier = geom::Curve3::Circle {
            center: Point3::origin(),
            axis: Vec3::new(0.0, 0.0, 1.0),
            radius: 1.0,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        edge(carrier, (t0, t1))
    }

    /// **A landing is placed ahead of the start and short of the end**:
    /// on a line leg ending at `1`, a landing at `0.5` is placed, one
    /// behind the start or past the end is off the leg, and one at the
    /// end grazes. Round a circle the place is the arc length ahead,
    /// `3π/2` for a landing a quarter turn behind the start.
    #[test]
    fn a_landing_is_placed_short_of_the_legs_end() {
        let line = leg(x_line(), Some(1.0));
        let at = |x: f64| line.place(Point3::new(x, 0.0, 0.0), AT, b());
        assert!(
            matches!(at(0.5), Ok(Some(s)) if s == 0.5),
            "short of the end"
        );
        assert!(matches!(at(-0.5), Ok(None)), "behind the start");
        assert!(matches!(at(1.5), Ok(None)), "past the end");
        assert!(matches!(at(1.0), Err(RayFault::Graze)), "at the end");
        let round = leg(unit_round(), None);
        let s = round.place(Point3::new(0.0, -1.0, 0.0), AT, b());
        assert!(
            matches!(s, Ok(Some(s)) if (s - 1.5 * PI).abs() < 1e-12),
            "a quarter turn behind is three quarters ahead: {s:?}"
        );
    }

    /// **A line leg runs along an edge on its line only where it reaches
    /// it**: an edge wholly behind the start, or wholly past the end, is
    /// never reached; one inside the leg, one with an end at the leg's
    /// end, and one reaching from behind the start past the end are run
    /// along, which grazes. A leg with no end reaches every edge ahead.
    #[test]
    fn a_line_leg_runs_along_an_edge_only_where_it_reaches_it() {
        let bounded = leg(x_line(), Some(1.0));
        let along = |l: &Leg<f64, Line<f64>>, x0, x1| l.along(&x_segment(x0, x1), AT, b());
        assert!(along(&bounded, -3.0, -2.0).is_ok(), "wholly behind");
        assert!(along(&bounded, 2.0, 3.0).is_ok(), "wholly past");
        for (x0, x1, what) in [
            (0.25, 0.75, "inside"),
            (1.0, 2.0, "from the end"),
            (-1.0, 2.0, "across the leg"),
        ] {
            assert!(
                matches!(along(&bounded, x0, x1), Err(RayFault::Graze)),
                "{what}"
            );
        }
        let whole = leg(x_line(), None);
        assert!(along(&whole, -3.0, -2.0).is_ok(), "behind a whole leg");
        assert!(
            matches!(along(&whole, 2.0, 3.0), Err(RayFault::Graze)),
            "ahead of a whole leg"
        );
    }

    /// **A leg round a circle reads an edge on its circle by arc
    /// length**: a quarter-turn leg does not reach the arc over
    /// `[π, 3π/2]`, which lies past its end going round (its end at
    /// `3π/2` is a quarter turn behind the start, three quarters ahead),
    /// and runs along the arc over `[π/8, π/4]`. A whole turn runs along
    /// every arc of its circle.
    #[test]
    fn a_round_leg_reads_an_edge_on_its_circle_by_arc_length() {
        let quarter = leg(unit_round(), Some(FRAC_PI_2));
        assert!(
            quarter.along(&unit_arc(PI, 1.5 * PI), AT, b()).is_ok(),
            "past the end"
        );
        assert!(
            matches!(
                quarter.along(&unit_arc(PI / 8.0, PI / 4.0), AT, b()),
                Err(RayFault::Graze)
            ),
            "inside"
        );
        assert!(
            matches!(
                leg(unit_round(), None).along(&unit_arc(PI, 1.5 * PI), AT, b()),
                Err(RayFault::Graze)
            ),
            "a whole turn"
        );
    }
}
