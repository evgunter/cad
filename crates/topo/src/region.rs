//! **What the region readings share**: the skeleton
//! [`crate::sphere_region`], [`crate::torus_region`] and
//! [`crate::ruled_region`] read a trimmed curved face with. The skeleton
//! gathers the boundary, reads the point against it first, walks the
//! paths and reads each leg to its closest crossing; a door supplies only
//! its surface's legs ([`Legs`], and [`Paths`] for a roster of them) and
//! its own rows. Where a leg meets an edge is read once for every door
//! ([`landings`]): the edge against the leg's plane, or the leg against
//! the edge's, through the plane door, and the root then on its plane's
//! section curve.
//!
//! # Co-surface carriers
//!
//! A leg and an edge of one face lie on one surface, so their gap is
//! zero in the model, and a reading of it reads only the construction's
//! rounding. Where they meet is instead a crossing of a plane: the
//! leg's plane cuts the surface in the leg and a few other curves (a
//! cylinder's meridian plane in two rulings, a cone's in four
//! half-generators, a torus's planes in two circles), so a root of the
//! edge against it is a meeting where it lies on the leg's own curve
//! ([`Section`](geom_brep::curve_pair::on_surface::Section)). That is `geom_brep`'s co-surface arm
//! ([`geom_brep::curve_pair::on_surface`]), which builds every leg from
//! the surface record and chart coordinates ([`SurfaceCurve`]), and the
//! query is read onto the surface first ([`QueryFoot`]): every leg and
//! corner is built from the query's chart coordinates, never from a point
//! off the surface.

use geom_brep::curve_face::plane_crossing::{RootPlace, SpanEnds};
use geom_brep::curve_pair::on_surface::{
    AcrossRefusal, Charted, CoSurface, NoSuchFamily, OnSurface, Shape, SurfaceCurve,
};
use geom_core::{Band, Decide, Indeterminate, Margin, Point2, Point3, Sign, Vec3};

use crate::body::Body;
use crate::entity::{EdgeKey, FaceKey, LoopKey};
use crate::live::BoundaryMember;
use crate::null::Scaffolding;
use crate::ray_walk::{self, Crossings, RayFault};
use crate::solid_contain::PointInSolidError;
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
    /// **This refusal's standing** ([`geom_core::ladder::Refusal`]): its
    /// image's in the point-in-solid door ([`Self::of_face`],
    /// [`PointInSolidError::refusal`]), derived, so the two cannot part.
    /// An escalation by its margin and a walk no path of which settled in
    /// band (`[failure.taxonomy]` row 1(b)); an edge whose boundary
    /// reading ran out of depth or budget, a limit, as the door's
    /// `EdgeCarrierUnsupported` is (row 2, a bound on the work); and an
    /// arc wound past a turn, a corrupt face, an abort (row 4). The face
    /// the image names does not move its standing.
    ///
    /// # Errors
    ///
    /// [`geom_core::ladder::Aborts`] for [`Self::WoundPastPeriod`].
    fn refusal(&self) -> Result<geom_core::ladder::Refusal, geom_core::ladder::Aborts> {
        self.of_face(FaceKey::default()).refusal()
    }

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

/// **The unsigned angle between the unit vectors `a` and `b`**, in
/// `[0, π]`: `2·atan2(|a − b|, |a + b|)`, both arguments norms, so no
/// cut lies anywhere on it and an enclosure of it is as tight at `π` as
/// at `0`.
pub(crate) fn between<T: Decide>(a: Vec3<T>, b: Vec3<T>) -> T {
    (a - b).norm().atan2((a + b).norm()) * T::from_f64(2.0)
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
pub(crate) fn side_at_closest_as<T: Decide>(
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

/// Where a leg crosses an edge: the crossing's point, the edge's
/// parameter there, and whether it lies within the band of an end of the
/// edge, where no side is read.
pub(crate) type Landing<T> = (Point3<T>, T, bool);

/// **A query read onto its surface**: its chart coordinates and its point
/// on the surface. A leg's locus is built from the coordinates (C5), so
/// every leg lies on the surface wherever the query lies off it; a circle
/// leg's parametrisation starts at the point
/// ([`geom_brep::curve_pair::on_surface::SurfaceCurve::starting_at`]), and
/// a ruling, whose origin is its locus, at the coordinates. The boundary
/// pre-pass reads the point ([`QueryFoot::read`]): off the chart's singular
/// readings, the foot read in closed form on no chart; on them, the
/// chart's own point.
#[derive(Clone, Copy, Debug)]
pub(crate) struct QueryFoot<T: geom_core::Real> {
    uv: Point2<T>,
    at: Point3<T>,
}

impl<T: Decide> QueryFoot<T> {
    /// `p`'s foot on `surface`: its coordinates read through its chart
    /// ([`geom::Surface::chart_point`]) once, and its point read in closed
    /// form on no chart ([`geom::Surface::foot`]), so a pre-pass reading
    /// `p` at the band's zero answers the same on every frame. The chart's
    /// round trip `eval(uv)` lands within rounding of that point, but in
    /// rounding that depends on the frame.
    ///
    /// Where the chart reads `p` singular, its own point stands: the
    /// coordinates there are a convention the chart chose (a cone's nappe
    /// undecided within the band of its apex plane, a point within the
    /// band of an axis or a tube circle), and the closed form may choose
    /// otherwise: the cone's foot takes the nappe by the raw sign of
    /// `p`'s height, the opening nappe's chart the decided one. Reading
    /// the chart's point there keeps the pre-pass and the legs on one
    /// foot. The same stands where `p` has no foot in closed form.
    ///
    /// # Panics
    ///
    /// On a surface with no closed chart inversion: every region door
    /// reads an analytic surface.
    pub(crate) fn read(surface: &geom::Surface<T>, p: Point3<T>, band: Band) -> Self {
        let Ok(chart) = surface.chart_point(p, band) else {
            unreachable!("a region door reads an analytic surface")
        };
        let foot = if chart.singular {
            None
        } else {
            surface.foot(p)
        };
        foot.map_or_else(
            || Self::of_chart(surface, chart.uv),
            |at| Self { uv: chart.uv, at },
        )
    }

    /// The point at chart coordinates `uv` on `surface`: a corner a door
    /// sets from two feet's coordinates.
    pub(crate) fn of_chart(surface: &geom::Surface<T>, uv: Point2<T>) -> Self {
        Self {
            uv,
            at: surface.eval(uv.x, uv.y),
        }
    }

    /// The chart coordinates.
    pub(crate) fn uv(&self) -> Point2<T> {
        self.uv
    }

    /// The point of the surface.
    pub(crate) fn at(&self) -> Point3<T> {
        self.at
    }
}

/// Which curve of a pair is read across the other's plane
/// ([`Legs::across`]).
pub(crate) enum ReadAcross {
    /// The edge across the leg's plane, in the edge's parameter.
    EdgeOverLeg,
    /// The leg across the edge's own plane ([`OnSurface::plane`]), in the
    /// leg's parameter; the door places each root on the edge
    /// ([`Legs::on_edge_plane`]).
    LegOverEdge,
}

/// **Where `leg` meets `edge`**: each meeting's point, the edge's
/// parameter there, and whether it lies within the band of an end of the
/// edge ([`Landing`]). One curve is read across the other's plane through
/// the plane door ([`span_plane_meet`](geom_brep::curve_face::plane_crossing::span_plane_meet)), the pair's orientation the
/// door's choice ([`Legs::across`]), and each root is a meeting where it
/// lies on both curves: on the leg's section of its plane
/// ([`Section`](geom_brep::curve_pair::on_surface::Section)), or where the door places it on the edge. Where along
/// the leg each landing lies is the skeleton's to read ([`Leg::place`]).
/// An edge in the leg's plane on the leg's own curve (a shared run) is no
/// crossing: the leg runs along it where it reaches it ([`Leg::along`]).
///
/// # Errors
///
/// A rung of the plane door in band, a component in band, or a touch
/// sets the path aside. No landing is read off a refusal.
pub(crate) fn landings<T: Decide, D: Legs<T>>(
    door: &D,
    edge: &RegionEdge<T>,
    leg: &Leg<T, D::Course>,
    band: Band,
) -> Result<Vec<Landing<T>>, Fault> {
    let curve = leg.course.curve();
    // An edge in the leg's plane: run along where it lies on the leg's
    // own curve, read at its middle.
    let in_plane = || -> Result<(), Fault> {
        let middle = edge.at(geom::mid_param(edge.t0, edge.t1));
        let section = curve
            .section_of(middle, D::COMPONENT, band)
            .map_err(in_band)?;
        if on_section(section)? {
            leg.along(edge, D::AT, band)?;
        }
        Ok(())
    };
    let mut out = Vec::new();
    let on = OnSurface::edge(door.charted().key(), &edge.carrier, (edge.t0, edge.t1));
    match door.across(leg, edge, band)? {
        ReadAcross::EdgeOverLeg => {
            match on.across(&curve, D::COMPONENT, band).map_err(across)? {
                CoSurface::InPlane => in_plane()?,
                CoSurface::Touch => return Err(RayFault::Graze),
                CoSurface::Unlaned => {
                    unreachable!("a region door admits closed-form carriers only")
                }
                CoSurface::Roots(roots) => {
                    for (root, y) in roots {
                        let section = curve.section_of(y, D::COMPONENT, band).map_err(in_band)?;
                        if !on_section(section)? {
                            continue;
                        }
                        let sideless = match root.place {
                            RootPlace::Inside => false,
                            RootPlace::AtEnd => true,
                            // A root past the leg's own end is no meeting,
                            // however near the edge's end it lies: a first
                            // leg ends at its corner short of a vertex
                            // every edge's root lies in band of.
                            RootPlace::Undecided(diag) => match leg.place(y, D::AT, band) {
                                Ok(None) => continue,
                                _ => return Err(in_band(diag)),
                            },
                        };
                        out.push((y, root.t, sideless));
                    }
                }
            }
        }
        ReadAcross::LegOverEdge => {
            let (params, whole) = leg.course.span(leg.end.map(|end| end.place), edge);
            let ends = if whole {
                SpanEnds::Endless
            } else {
                SpanEnds::Ended
            };
            match on
                .under(&curve, params, ends, D::COMPONENT, band)
                .map_err(across)?
            {
                CoSurface::InPlane => in_plane()?,
                CoSurface::Touch => return Err(RayFault::Graze),
                CoSurface::Unlaned => {
                    unreachable!("a door reads a leg across a conic's or a spiric's plane only")
                }
                CoSurface::Roots(roots) => {
                    // At or in band of the leg's own end: the leg's place
                    // reads it.
                    for (_, y) in roots {
                        if let Some((t, sideless)) = door.on_edge_plane(leg, edge, y, band)? {
                            out.push((y, t, sideless));
                        }
                    }
                }
            }
        }
    }
    Ok(out)
}

/// An edge's reading against a leg refused: in band sets the path aside.
///
/// # Panics
///
/// On a pair on two records: a door keys its edges and charts its legs
/// from its one record ([`Legs::charted`]), so a mismatch is a fault in
/// the door's wiring, not in the face.
fn across(refusal: AcrossRefusal) -> Fault {
    match refusal {
        AcrossRefusal::InBand(diag) => in_band(diag),
        AcrossRefusal::OffRecord { edge, curve } => unreachable!(
            "a region door keys its edges and charts its legs from one record: the edge's \
             {edge:?}, the leg's {curve:?}"
        ),
    }
}

/// Whether a root lies on the leg's own section of its plane
/// ([`SurfaceCurve::section_of`]): one the reading cannot tell from
/// another curve of the plane grazes.
fn on_section(section: Sign) -> Result<bool, Fault> {
    match section {
        Sign::Positive => Ok(true),
        Sign::Negative => Ok(false),
        Sign::Zero => Err(RayFault::Graze),
    }
}

/// **A leg's curve on the door's surface**: the arm builds it from the
/// surface record and a foot's chart coordinates, and a door asks only
/// for the families its surface has.
pub(crate) fn built<T: Decide>(curve: Result<SurfaceCurve<T>, NoSuchFamily>) -> SurfaceCurve<T> {
    match curve {
        Ok(curve) => curve,
        Err(refused) => {
            unreachable!("a region door builds only its surface's families: {refused:?}")
        }
    }
}

/// One boundary edge of a region over its certified span `[t0, t1]`,
/// on its certified carrier, of a kind its door admits ([`boundary`]).
#[derive(Clone, Debug)]
pub(crate) struct RegionEdge<T: geom_core::Real> {
    pub(crate) edge: EdgeKey,
    /// The loop the edge bounds.
    pub(crate) r#loop: LoopKey,
    /// The surface cycle the edge is on, its index in
    /// [`crate::surface_boundary::surface_cycles`]'s order.
    pub(crate) cycle: usize,
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
        let d = d.normalize();
        if self.forward { d } else { -d }
    }
}

/// **`face`'s boundary**: its surface boundary
/// ([`crate::surface_boundary::surface_cycles`]), each edge on its
/// certified carrier, cycle by cycle in walk order, each edge naming its
/// cycle ([`RegionEdge::cycle`]). An edge the face lies on both sides of
/// (a seam, a wrap edge, a spur) is no boundary of it and the walk turns
/// through it; none for a face that covers its surface. `None` where a
/// carrier's kind of any edge of its loops is not one of `admits`, or
/// where such an edge has no certified curve: an uncertified edge
/// between two points is a gap in the boundary no reading may step
/// over, and null scaffolding is as `scaffolding` says.
///
/// # Errors
///
/// [`PointInSolidError::CorruptFace`] for a key that does not resolve,
/// an outer loop with no cycle, or a surface boundary that does not walk.
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
    let f = body.face(face).ok_or_else(corrupt)?;
    let mut null = Vec::new();
    for (_, members, role) in f.boundary_by_loop() {
        for member in members {
            // A lone ring vertex bounds no area; an outer loop must.
            if role.is_outer() && matches!(member, BoundaryMember::Isolated { .. }) {
                return Err(corrupt());
            }
            let BoundaryMember::Edge {
                ek: edge_key, edge, ..
            } = member
            else {
                continue;
            };
            let Some(curve) = body.edge_curve_linked(edge_key, edge).certified() else {
                if scaffolding == Scaffolding::Skipped
                    && crate::null::is_placeholder(body, edge_key)
                {
                    null.push(edge_key);
                    continue;
                }
                return Ok(None);
            };
            if !admits.contains(&curve.carrier().kind()) {
                return Ok(None);
            }
        }
    }
    let cycles = crate::surface_boundary::surface_cycles(body, face).map_err(|_| corrupt())?;
    let mut edges = Vec::new();
    for (cycle, members) in cycles.iter().enumerate() {
        for &he in members {
            let h = body.get_half_edge(he).ok_or_else(corrupt)?;
            if null.contains(&h.edge) {
                continue;
            }
            let edge = body.get_edge(h.edge).ok_or_else(corrupt)?;
            let Some(curve) = body.edge_curve_linked(h.edge, edge).certified() else {
                unreachable!("every edge of the face's loops was read certified above")
            };
            let (t0, t1) = curve.params();
            edges.push(RegionEdge {
                edge: h.edge,
                r#loop: h.parent_loop,
                cycle,
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

/// Three points of every edge, in walk order, at [`TARGET_SHARES`] of
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

    /// The face's boundary, to cut ([`Region::keep_cycles`]).
    fn edges_mut(&mut self) -> &mut Vec<RegionEdge<T>>;

    /// **The region the surface cycles `keep` bound alone**: the boundary
    /// less every edge of another cycle ([`RegionEdge::cycle`]), the one
    /// cut a reading of a face against some of its cycles takes
    /// ([`crate::contain::cycle_side`]).
    #[must_use]
    fn keep_cycles(mut self, keep: &[usize]) -> Self
    where
        Self: Sized,
    {
        self.edges_mut().retain(|e| keep.contains(&e.cycle));
        self
    }

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
    /// edge in walk order before any path. A crossing's place along a
    /// path is no measure of `p`'s distance from the edge it crosses, so
    /// the paths never answer it.
    ///
    /// Any edge holding `p` answers, whatever an earlier edge read
    /// ([`geom_core::ladder::climb`], [`geom_core::ladder::Answering::Any`]):
    /// an edge read in band is kept by its standing
    /// ([`RegionRefusal::refusal`]), refusing only where no edge holds `p`.
    ///
    /// # Errors
    ///
    /// The kept refusal the ladder ranks first, or an edge's abort.
    fn on_boundary(&self, p: Point3<T>, band: Band) -> Result<bool, RegionRefusal> {
        use geom_core::ladder::{Answering, Climb, Rung, climb};
        let on = climb(Answering::Any, self.edges(), |edge| {
            Rung::holds(self.on_edge(edge, p, band), (), RegionRefusal::refusal)
        });
        match on {
            Climb::Answered { answer, .. } => answer.map(|()| true),
            Climb::Passed => Ok(false),
            Climb::Undecided(e) => Err(e),
        }
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

    /// The course as a curve on the door's surface: its carrier, its
    /// plane and its section of it ([`SurfaceCurve`]).
    fn curve(&self) -> SurfaceCurve<T>;

    /// The span of the course's carrier a leg runs from its start, and
    /// whether it is a whole turn, whose start is no end of it
    /// ([`SpanEnds::Endless`]): to the place `end` where it is bounded, and
    /// otherwise round the whole turn, or along a line past every point
    /// of `edge`.
    fn span(&self, end: Option<T>, edge: &RegionEdge<T>) -> ((T, T), bool);
}

/// Where a bounded leg stops: its place along the leg, the row a place
/// is read against it on, and whether the leg runs up to it and not onto
/// it (`open`): a cone's apex, where every generator meets and nothing
/// carries a side across, so a meeting there is no crossing of the leg.
/// A corner is closed: the path turns there.
#[derive(Clone, Copy)]
pub(crate) struct End<T: geom_core::Real> {
    pub(crate) place: T,
    pub(crate) row: &'static str,
    pub(crate) open: bool,
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

    /// Whether a place ahead of the start lies short of the leg's end. A
    /// place at an open end is not on the leg ([`End::open`]).
    ///
    /// # Errors
    ///
    /// A place at a closed end grazes; an in-band one sets the path
    /// aside.
    fn short_of_end(&self, s: T, band: Band) -> Result<bool, Fault> {
        let Some(end) = self.end else {
            return Ok(true);
        };
        match decide(end.row, Margin::of(end.place - s), band).map_err(in_band)? {
            Sign::Positive => Ok(true),
            Sign::Negative => Ok(false),
            Sign::Zero if end.open => Ok(false),
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
    const SIDE_ROWS: (&'static str, &'static str);

    /// The row a root's section of a leg's plane is read on
    /// ([`Section`](geom_brep::curve_pair::on_surface::Section)), and an edge's offset from a plane it is parallel
    /// to.
    const COMPONENT: &'static str;

    /// The lever of the closest crossing's sine.
    fn lever(&self) -> T;

    /// The face's surface record, its key and surface: every leg is
    /// charted on it ([`SurfaceCurve`]), every edge keyed by it
    /// ([`OnSurface::edge`]), and a query's foot read in its chart
    /// ([`QueryFoot`]).
    fn charted(&self) -> &Charted<T>;

    /// Which of `leg` and `edge` is read across the other's plane: the
    /// edge across the leg's unless the door says otherwise.
    ///
    /// # Errors
    ///
    /// A choice in band sets the path aside.
    fn across(
        &self,
        _leg: &Leg<T, Self::Course>,
        _edge: &RegionEdge<T>,
        _band: Band,
    ) -> Result<ReadAcross, Fault> {
        Ok(ReadAcross::EdgeOverLeg)
    }

    /// Where `y`, a root of `leg` across `edge`'s plane, lies on `edge`
    /// ([`ReadAcross::LegOverEdge`]): its parameter and whether it is at an
    /// end, or `None` off the edge (on its plane's other curve, or past
    /// its span).
    ///
    /// # Errors
    ///
    /// A reading in band sets the path aside.
    fn on_edge_plane(
        &self,
        _leg: &Leg<T, Self::Course>,
        _edge: &RegionEdge<T>,
        _y: Point3<T>,
        _band: Band,
    ) -> Result<Option<(T, bool)>, Fault> {
        unreachable!("the door reads no leg across an edge's plane")
    }

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
        p: QueryFoot<T>,
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
        p: QueryFoot<T>,
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
    p: QueryFoot<T>,
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
    p: QueryFoot<T>,
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
    side_at_closest_as(crossings, D::SIDE_ROWS, door.lever(), band)
}

/// **The crossing offer**: every crossing of `leg` with the boundary,
/// edge by edge, at its place along the leg ([`Leg::place`]). Each edge's landings are all read
/// ([`landings`]) before any of them is placed. Whether one was
/// offered.
fn offer<T: Decide, D: Legs<T>>(
    door: &D,
    leg: &Leg<T, D::Course>,
    crossings: &mut Crossings<T, Hit<T>>,
    band: Band,
) -> Result<bool, Fault> {
    let mut offered = false;
    for edge in door.edges() {
        for (y, t, sideless) in landings(door, edge, leg, band)? {
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

/// A leg round a circle of the door's surface ([`SurfaceCurve`]), from
/// its start at the circle's `θ = 0`, the foot it was built through,
/// along increasing `θ`, the circle's own parameter.
#[derive(Clone, Copy)]
pub(crate) struct Round<T: geom_core::Real> {
    curve: SurfaceCurve<T>,
    circle: geom::Circle<T>,
}

impl<T: Decide> Round<T> {
    /// **The leg round `curve`**, a circle the arm built on the door's
    /// surface from a foot ([`QueryFoot`]), starting at that foot.
    pub(crate) fn on(curve: SurfaceCurve<T>) -> Self {
        let Shape::Circle(circle) = curve.shape() else {
            unreachable!("a round leg runs on a circle")
        };
        Self { curve, circle }
    }

    /// The angle round the circle between the start and `y`, a point of
    /// the circle, the shorter way, in `[0, π]` ([`between`]).
    fn swept(&self, y: Point3<T>) -> T {
        let w = y - self.circle.frame.origin;
        between(w.normalize(), self.circle.frame.u_ref)
    }

    /// The arc length from the start round to `y`, a point of the
    /// circle, in `[0, 2π·radius)`: the shorter way where `y` lies ahead
    /// of the start's diameter, the longer way where it lies behind.
    pub(crate) fn arc(&self, y: Point3<T>) -> T {
        let s = self.swept(y);
        let ahead = (y - self.circle.frame.origin).dot(self.circle.frame.v_ref());
        self.circle.radius * (T::zero() - ahead).select_le_zero(s, T::tau() - s)
    }
}

/// Places round the circle are arc lengths in `[0, 2π·radius)`: a
/// circle has no behind.
impl<T: Decide> Course<T> for Round<T> {
    /// Read the shorter way round ([`ray_walk::apart_as`]), then as a turn
    /// ahead.
    fn place(&self, y: Point3<T>, at: &'static str, band: Band) -> Result<Option<T>, Fault> {
        let s = self.swept(y);
        ray_walk::apart_as(at, Margin::of((self.circle.radius * s).abs()), band)
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
        let h = self.circle.frame.axis.cross(y - self.circle.frame.origin);
        h.normalize()
    }

    fn curve(&self) -> SurfaceCurve<T> {
        self.curve
    }

    /// Places are arc lengths, so an angle is a place over the radius.
    fn span(&self, end: Option<T>, _: &RegionEdge<T>) -> ((T, T), bool) {
        match end {
            None => ((T::zero(), T::tau()), true),
            Some(place) => ((T::zero(), place / self.circle.radius), false),
        }
    }
}

/// A leg along a ruling of the door's surface ([`SurfaceCurve`]), from
/// the foot it was built at, places metres along its unit direction.
#[derive(Clone, Copy)]
pub(crate) struct Line<T: geom_core::Real> {
    curve: SurfaceCurve<T>,
    line: geom::Line<T>,
}

impl<T: Decide> Line<T> {
    /// **The leg along `curve`**, a ruling the arm built on the door's
    /// surface from a foot ([`QueryFoot`]), starting at that foot.
    pub(crate) fn on(curve: SurfaceCurve<T>) -> Self {
        let Shape::Line(line) = curve.shape() else {
            unreachable!("a line leg runs on a ruling")
        };
        Self { curve, line }
    }
}

impl<T: Decide> Course<T> for Line<T> {
    fn place(&self, y: Point3<T>, at: &'static str, band: Band) -> Result<Option<T>, Fault> {
        let s = self.line.height(y);
        match ray_walk::advance_as(at, Margin::of(s), band)
            .map_err(|f| f.map(RegionRefusal::Escalated))?
        {
            Sign::Negative => Ok(None),
            _ => Ok(Some(s)),
        }
    }

    fn heading(&self, _: Point3<T>) -> Vec3<T> {
        self.line.dir
    }

    fn curve(&self) -> SurfaceCurve<T> {
        self.curve
    }

    /// No door reads a ruling across an edge's plane: an edge is read
    /// across the ruling's own plane, through the axis ([`Legs::across`]).
    fn span(&self, _: Option<T>, _: &RegionEdge<T>) -> ((T, T), bool) {
        unreachable!("a ruling is read only across its own plane")
    }
}

/// `face` with null scaffolding added to its outer loop: a null edge
/// fanned out of the vertex its first half-edge leaves.
#[cfg(test)]
#[allow(clippy::unwrap_used)]
pub(crate) fn with_null_edge(body: &mut Body<f64>, face: FaceKey) {
    let outer = body.get_face(face).unwrap().outer;
    let crate::entity::LoopBoundary::Cycle { first } = body.get_loop(outer).unwrap().boundary
    else {
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

/// A fixture edge's turn over `(t0, t1)`, at each scalar a fixture is
/// built at: a unit circle's arc over that interval, certified, whose
/// interval and extent are what a conic reading of the fixture reads (its
/// own carrier supplies the rest).
macro_rules! fixture_turns {
    ($($name:ident: $t:ty),*) => {$(
        #[cfg(test)]
        pub(crate) fn $name(t0: f64, t1: f64) -> Option<geom_brep::CertifiedTurn<$t>> {
            let lift = <$t as geom_core::Real>::from_f64;
            let band = Band::new(1e-9, 1e-8).ok()?;
            let circle = geom::Curve3::circle(Point3::new(lift(0.0), lift(0.0), lift(0.0)), Vec3::new(lift(0.0), lift(0.0), lift(1.0)), lift(1.0), Vec3::new(lift(1.0), lift(0.0), lift(0.0)));
            let (t0, t1) = (lift(t0), lift(t1));
            let spec = geom_brep::EdgeCurveSpec::arc_of_circle(circle.clone(), t0, t1)?;
            let (start, end) = (circle.eval(t0), circle.eval(t1));
            geom_brep::EdgeCurve::certify(
                spec,
                start,
                end,
                |_| None,
                geom_brep::ExtentBall::of_circle(Point3::new(lift(0.0), lift(0.0), lift(0.0)), lift(1.0))
                    .lever(),
                band,
                geom_brep::NurbsLane::certified(),
            )
            .ok()?
            .turn()
            .copied()
        }
    )*};
}

fixture_turns!(fixture_turn: f64, fixture_turn_interval: geom_core::Interval);

/// **A door's answer as a frame pin compares it**: the answer itself,
/// or its refusal's predicate (`Some("another refusal")` for a refusal
/// that names none), so two frames agree where they refuse alike.
#[cfg(test)]
pub(crate) fn refusal_class(
    r: Result<Option<bool>, RegionRefusal>,
) -> Result<Option<bool>, Option<&'static str>> {
    r.map_err(|e| match e {
        RegionRefusal::Escalated(diag) => diag.predicate,
        _ => Some("another refusal"),
    })
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
                open: false,
            }),
        }
    }

    fn edge(carrier: geom::Curve3<f64>, (t0, t1): (f64, f64)) -> RegionEdge<f64> {
        RegionEdge {
            edge: EdgeKey::from(KeyData::from_ffi(1)),
            r#loop: LoopKey::default(),
            cycle: 0,
            carrier,
            t0,
            t1,
            turn: fixture_turn(t0, t1),
            forward: true,
        }
    }

    /// The `x` axis from the origin, the line every line row runs on: a
    /// ruling of the unit cylinder about the `x` axis through `(0, 1, 0)`.
    fn x_line() -> Line<f64> {
        let cylinder = geom::Surface::Cylinder {
            origin: Point3::new(0.0, 1.0, 0.0),
            axis: Vec3::new(1.0, 0.0, 0.0),
            radius: 1.0,
            u_ref: Vec3::new(0.0, -1.0, 0.0),
        };
        Line::on(built(SurfaceCurve::ruling(
            &Charted::record(Default::default(), &cylinder),
            Point2::new(0.0, 0.0),
            true,
            true,
        )))
    }

    /// The segment `[x0, x1]` of the `x` axis.
    fn x_segment(x0: f64, x1: f64) -> RegionEdge<f64> {
        let carrier = geom::Curve3::line(Point3::origin(), Vec3::new(1.0, 0.0, 0.0));
        edge(carrier, (x0, x1))
    }

    /// The unit circle about `z`, run from `x` toward `y`: the parallel
    /// at height `0` of the unit cylinder about `z`.
    fn unit_round() -> Round<f64> {
        let cylinder = geom::Surface::Cylinder {
            origin: Point3::origin(),
            axis: Vec3::new(0.0, 0.0, 1.0),
            radius: 1.0,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        Round::on(built(SurfaceCurve::parallel(
            &Charted::record(Default::default(), &cylinder),
            Point2::new(0.0, 0.0),
            true,
        )))
    }

    /// The arc of [`unit_round`]'s circle over `[t0, t1]`.
    fn unit_arc(t0: f64, t1: f64) -> RegionEdge<f64> {
        let carrier = geom::Curve3::circle(
            Point3::origin(),
            Vec3::new(0.0, 0.0, 1.0),
            1.0,
            Vec3::new(1.0, 0.0, 0.0),
        );
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

    /// A region whose edges read as `read` says, by the edge's cycle:
    /// the production [`Region::on_boundary`] over stubbed edge readings.
    struct Stubbed {
        edges: Vec<RegionEdge<f64>>,
        read: fn(usize) -> Result<bool, RegionRefusal>,
    }

    impl Region<f64> for Stubbed {
        fn edges(&self) -> &[RegionEdge<f64>] {
            &self.edges
        }
        fn edges_mut(&mut self) -> &mut Vec<RegionEdge<f64>> {
            &mut self.edges
        }
        fn outer(&self) -> LoopKey {
            LoopKey::default()
        }
        fn on_edge(
            &self,
            edge: &RegionEdge<f64>,
            _: Point3<f64>,
            _: Band,
        ) -> Result<bool, RegionRefusal> {
            (self.read)(edge.cycle)
        }
    }

    fn in_band() -> Indeterminate {
        Indeterminate {
            margin: geom_core::MarginDiag::value(5e-9),
            band: b(),
            // UNDECIDED-NAME: a fixture diag; the pin reads its standing, not the name
            predicate: Some("test_region_in_band"),
            terminal_sliver: false,
        }
    }

    /// **An edge holding the point answers past an edge in band**
    /// ([`Region::on_boundary`], `Any`), and an abort stops the climb.
    /// Two edges, read in order: the first in band, the second holding
    /// `p`, answers `true`; a first-refusal loop refused with the first's
    /// escalation. A wound-past-a-period edge ahead aborts whatever
    /// follows (`RegionRefusal::refusal`'s row 4), and two edges in band
    /// refuse with the escalation.
    #[test]
    fn an_edge_holding_the_point_answers_past_one_in_band() {
        let two = |read| Stubbed {
            edges: [0, 1]
                .map(|cycle| RegionEdge {
                    cycle,
                    ..x_segment(0.0, 1.0)
                })
                .into(),
            read,
        };
        let p = Point3::new(0.5, 0.0, 0.0);
        let kept_then_on: fn(usize) -> _ = |i| {
            if i == 0 {
                Err(RegionRefusal::Escalated(in_band()))
            } else {
                Ok(true)
            }
        };
        assert_eq!(two(kept_then_on).on_boundary(p, b()), Ok(true));
        let abort_then_on: fn(usize) -> _ = |i| {
            if i == 0 {
                Err(RegionRefusal::WoundPastPeriod)
            } else {
                Ok(true)
            }
        };
        assert_eq!(
            two(abort_then_on).on_boundary(p, b()),
            Err(RegionRefusal::WoundPastPeriod)
        );
        let both_kept: fn(usize) -> _ = |_| Err(RegionRefusal::Escalated(in_band()));
        assert!(matches!(
            two(both_kept).on_boundary(p, b()),
            Err(RegionRefusal::Escalated(_))
        ));
    }
}

/// [`RegionRefusal::refusal`] as each variant's image in the point-in-solid
/// door stands (QML T4).
#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod standing_rows {
    use geom_core::ladder::{Aborts, Refusal};
    use geom_core::{Band, Indeterminate, MarginDiag};

    use super::RegionRefusal;
    use crate::entity::LoopKey;

    /// **Every variant stands as its point-in-solid image does**
    /// ([`RegionRefusal::refusal`]), the expected standing a `match` with
    /// no wildcard: changing a standing, or adding a variant without one
    /// here, goes red.
    #[test]
    fn a_region_refusal_stands_as_its_point_in_loop_image() {
        let diag = Indeterminate {
            margin: MarginDiag::value(5e-9),
            band: Band::new(1e-9, 1e-8).unwrap(),
            // UNDECIDED-NAME: a fixture diag; the pin reads its standing, not the name
            predicate: Some("test_region_standing"),
            terminal_sliver: false,
        };
        let every = [
            RegionRefusal::Escalated(diag),
            RegionRefusal::RayExhausted {
                outer: LoopKey::default(),
            },
            RegionRefusal::WoundPastPeriod,
            RegionRefusal::Unreadable(crate::test_support_samples::uncrossable()),
        ];
        for e in every {
            let want = match e {
                RegionRefusal::Escalated(diag) => Ok(Refusal::Margin(diag)),
                RegionRefusal::RayExhausted { .. } => Ok(Refusal::InBand),
                RegionRefusal::Unreadable(_) => Ok(Refusal::Limit),
                RegionRefusal::WoundPastPeriod => Err(Aborts),
            };
            assert_eq!(e.refusal(), want, "{e:?}");
        }
    }
}
