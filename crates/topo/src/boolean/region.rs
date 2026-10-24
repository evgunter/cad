//! **What the region readings share**: the skeleton [`super::sphere_region`],
//! [`super::torus_region`] and [`super::ruled_region`] read a trimmed
//! curved face with, each door supplying only its surface's legs and
//! landings —
//! the boundary gather ([`boundary`]: every edge of every loop, null
//! scaffolding and seams left out, an uncertified edge refused), the
//! boundary pre-pass ([`Region::on_boundary`]), the path roster and its
//! walk ([`walk`], over a [`Paths`] door's legs), the crossing offer
//! ([`read_leg`]) and the leaves under them:
//! the one refusal they answer ([`RegionRefusal`]), the targets their aimed
//! paths take ([`targets`], at [`TARGET_SHARES`]), the boundary reading of
//! a circle edge on its own carrier ([`circle_edge_hit`]), a root against
//! its edge's span ([`span`]), the verdict read at a path's closest
//! crossing ([`side_at_closest`]), a leg round a parallel ([`Round`],
//! [`parallel`]), and a leg's turn and root answer ([`turn`], [`found`]).
//! Each door keeps its own rows and its crossing arithmetic.

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
    span: (T, T),
    rows: ConicRows,
    p: Point3<T>,
    outer: LoopKey,
    band: Band,
) -> Result<bool, RegionRefusal> {
    let k = match ConicArc::of(circle, span, rows, band) {
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

/// An in-band margin about one path, setting it aside.
pub(crate) fn in_band(diag: Indeterminate) -> RayFault<RegionRefusal> {
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
) -> Result<Option<bool>, RayFault<RegionRefusal>> {
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
) -> Result<bool, RayFault<RegionRefusal>> {
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
pub(crate) fn turn<T: Decide>(s: T) -> T {
    (T::zero() - s).select_le_zero(s, s + T::tau())
}

/// A first-harmonic root door's answer on one leg: its roots, none for a miss; an edge
/// in the leg's plane or a tangency grazes.
pub(crate) fn found<T: Decide>(
    roots: Result<FirstHarmonicRoots<T>, RootRefusal>,
) -> Result<Vec<T>, RayFault<RegionRefusal>> {
    match roots {
        Ok(Roots::Miss) => Ok(Vec::new()),
        Ok(Roots::Certified(found)) => Ok(found.roots().to_vec()),
        Ok(Roots::On(_)) | Err(RootRefusal::Unsettled(_)) => Err(RayFault::Graze),
        Err(RootRefusal::Escalated(diag)) => Err(in_band(diag)),
    }
}

/// A path's reason to be set aside.
pub(crate) type Fault = RayFault<RegionRefusal>;

/// Where a leg crosses an edge: the crossing's point, the edge's
/// parameter there, and whether it lies within the band of an end of the
/// edge, where no side is read.
pub(crate) type Landing<T> = (Point3<T>, T, bool);

/// An edge's carrier, as a region reads it.
pub(crate) trait EdgeCarrier<T: geom_core::Real> {
    /// The point at parameter `t`.
    fn at(&self, t: T) -> Point3<T>;

    /// A tangent at `t` along increasing `t`, of any length.
    fn tangent(&self, t: T) -> Vec3<T>;
}

impl<T: Decide> EdgeCarrier<T> for geom::Curve3<T> {
    fn at(&self, t: T) -> Point3<T> {
        self.eval(t)
    }

    fn tangent(&self, t: T) -> Vec3<T> {
        self.deriv(t)
    }
}

/// One boundary edge of a region over its certified span `[t0, t1]`.
#[derive(Clone, Debug)]
pub(crate) struct RegionEdge<T: geom_core::Real, C> {
    pub(crate) edge: EdgeKey,
    /// The loop the edge bounds.
    pub(crate) r#loop: LoopKey,
    pub(crate) carrier: C,
    pub(crate) t0: T,
    pub(crate) t1: T,
    /// Whether the loop runs `t0 → t1` (its `he_plus`).
    pub(crate) forward: bool,
}

impl<T: Decide, C: EdgeCarrier<T>> RegionEdge<T, C> {
    pub(crate) fn at(&self, t: T) -> Point3<T> {
        self.carrier.at(t)
    }

    /// The loop's unit traversal direction at `t`.
    pub(crate) fn traversal(&self, t: T) -> Vec3<T> {
        let d = self.carrier.tangent(t);
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
/// loop order, each with the carrier `carrier` reads off its certified
/// curve. `None` where `carrier` refuses a curve, or where an edge has no
/// certified curve: an uncertified edge between two points is a gap in
/// the boundary no reading may step over, and null scaffolding is as
/// `scaffolding` says. An edge both of whose sides are the face (a seam,
/// both its half-edges in the face's loops) is no boundary of it and is
/// left out.
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
pub(crate) fn boundary<T: Decide, C>(
    body: &Body<T>,
    face: FaceKey,
    scaffolding: Scaffolding,
    mut carrier: impl FnMut(&geom::Curve3<T>) -> Option<C>,
) -> Result<Option<Vec<RegionEdge<T, C>>>, PointInSolidError> {
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
            let Some(read) = carrier(curve.carrier()) else {
                return Ok(None);
            };
            let (t0, t1) = curve.params();
            edges.push(RegionEdge {
                edge: edge_key,
                r#loop: lk,
                carrier: read,
                t0,
                t1,
                forward: he == edge.he_plus,
            });
        }
    }
    let seam = |e: EdgeKey| edges.iter().filter(|a| a.edge == e).count() > 1;
    let seams: Vec<EdgeKey> = edges.iter().map(|a| a.edge).filter(|&e| seam(e)).collect();
    edges.retain(|a| !seams.contains(&a.edge));
    Ok(Some(edges))
}

/// Three points of every edge, in loop order, at [`TARGET_SHARES`] of
/// its span: the aimed paths' targets.
pub(crate) fn targets<T: Decide, C: EdgeCarrier<T>>(
    edges: &[RegionEdge<T, C>],
) -> impl Iterator<Item = Point3<T>> + '_ {
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
    type Carrier: EdgeCarrier<T>;

    /// The face's boundary ([`boundary`]).
    fn edges(&self) -> &[RegionEdge<T, Self::Carrier>];

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
        edge: &RegionEdge<T, Self::Carrier>,
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
/// curves through a point, its parallels and one other, each leg read
/// by its own landings.
pub(crate) trait Paths<T: Decide>: Region<T> {
    type Leg;

    /// The rows the closest crossing is read under: `(order, cross)`.
    const SIDE: (&'static str, &'static str);

    /// The lever of the closest crossing's sine.
    fn lever(&self) -> T;

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
    ) -> Result<Self::Leg, Fault>;

    /// An aimed path's legs ([`Path::Aimed`]): the first, ending at its
    /// corner; the corner's place along the path, where the second
    /// starts; and the second.
    ///
    /// # Errors
    ///
    /// A leg the door cannot frame sets the path aside.
    #[allow(clippy::too_many_arguments)] // one path's spec, unpacked
    fn aimed(
        &self,
        p: Point3<T>,
        target: Point3<T>,
        parallel_first: bool,
        first_forward: bool,
        then_forward: bool,
        band: Band,
    ) -> Result<(Self::Leg, T, Self::Leg), Fault>;

    /// Where `leg` meets `edge`'s carrier, every landing read before any
    /// is placed along the leg.
    ///
    /// # Errors
    ///
    /// A graze or an in-band reading sets the path aside.
    fn landings(
        &self,
        edge: &RegionEdge<T, Self::Carrier>,
        leg: &Self::Leg,
        band: Band,
    ) -> Result<Vec<Landing<T>>, Fault>;

    /// Where the landing `y` lies along `leg`: `Some(place)` ahead of its
    /// start and short of its end, `None` off the leg.
    ///
    /// # Errors
    ///
    /// A graze or an in-band reading sets the path aside.
    fn place(&self, leg: &Self::Leg, y: Point3<T>, band: Band) -> Result<Option<T>, Fault>;

    /// The unit direction of travel along `leg` at `y`.
    fn heading(&self, leg: &Self::Leg, y: Point3<T>) -> Vec3<T>;

    /// The face's unit outward normal at `y`.
    fn outward(&self, y: Point3<T>) -> Vec3<T>;
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
            let mut crossings = Crossings::new();
            // Past a crossing before the corner, every crossing of the
            // second leg lies beyond it, decided apart from the corner by
            // its own rows.
            if !offer(door, &first, T::zero(), &mut crossings, band)? {
                offer(door, &then, corner, &mut crossings, band)?;
            }
            side(door, crossings, band)
        }
    }
}

/// A path of the one leg `leg`: whether its start is in the face.
pub(crate) fn read_leg<T: Decide, D: Paths<T>>(
    door: &D,
    leg: &D::Leg,
    band: Band,
) -> Result<bool, Fault> {
    let mut crossings = Crossings::new();
    offer(door, leg, T::zero(), &mut crossings, band)?;
    side(door, crossings, band)
}

/// A path's verdict at its closest crossing. A path meeting no edge
/// lies wholly on one side.
fn side<T: Decide, D: Paths<T>>(
    door: &D,
    crossings: Crossings<T, Hit<T>>,
    band: Band,
) -> Result<bool, Fault> {
    side_at_closest(crossings, D::SIDE, door.lever(), band)
}

/// **The crossing offer**: every crossing of `leg` with the boundary,
/// edge by edge, at `offset` plus its place along the leg
/// ([`Paths::place`]). Whether one was offered.
fn offer<T: Decide, D: Paths<T>>(
    door: &D,
    leg: &D::Leg,
    offset: T,
    crossings: &mut Crossings<T, Hit<T>>,
    band: Band,
) -> Result<bool, Fault> {
    let mut offered = false;
    for edge in door.edges() {
        for (y, t, sideless) in door.landings(edge, leg, band)? {
            let Some(s) = door.place(leg, y, band)? else {
                continue;
            };
            crossings.push(
                offset + s,
                Hit {
                    heading: door.heading(leg, y),
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
    pub(crate) fn angle(&self, y: Point3<T>) -> T {
        let w = y - self.center;
        w.dot(self.e2).atan2(w.dot(self.e1))
    }

    /// The unit direction of travel at `y`.
    pub(crate) fn heading(&self, y: Point3<T>) -> Vec3<T> {
        let h = self.normal.cross(y - self.center);
        h / h.norm()
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
