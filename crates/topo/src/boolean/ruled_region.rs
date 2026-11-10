//! **A cylinder or cone face's region, read from its boundary edges** —
//! the one reading both point doors take of such a face: the ray lane's
//! hit test and boundary pre-pass ([`super::solid_contain`]) and the
//! pierce arm's landing test
//! ([`super::contain::curved_face_containment`]). It is
//! [`super::torus_region`]'s method on the two ruled quadrics, and their
//! easy case: every edge of such a face is a line, a circle or a planar
//! section, so it lies in a plane.
//!
//! # Method: the closest crossing along a path of rulings and parallels
//!
//! A cylinder's rulings are its lines parallel to the axis, a cone's are
//! its generators through the apex, and both surfaces' parallels are
//! their circles about the axis. The face is the side of its loops to
//! their left under its OUTWARD normal `N` (the implicit gradient with
//! the face's sense folded in), so at a point of an edge's interior with
//! traversal tangent `τ` the face lies along `N × τ`. The closest crossing
//! carries that reference to the query:
//!
//! 1. Walk a PATH from `p`: along its ruling either way, or round its
//!    parallel either way; or, where none settles, a ruling then a
//!    parallel (or the mirror) through a corner, aimed at a point of the
//!    boundary. A cone's generator toward the apex ENDS there: past it the
//!    line runs on the other nappe, which a path reaches only through the
//!    apex, and nothing about a face's side carries across that point.
//! 2. Every edge meets a leg where it crosses the leg's plane
//!    ([`region::landings`], through the plane door): a parallel's plane
//!    cuts the surface in the parallel alone; a ruling's, through the
//!    axis, cuts a cylinder in two rulings, the leg's and the opposite
//!    one, and a cone in two lines through the apex, each a generator on
//!    either nappe: four half-generators, of which the leg runs on one.
//!    So a root counts where it lies on the leg's own half-plane and
//!    nappe, read through the chart's channels. A cone's apex lies on every generator, and a generator
//!    toward it ends there ([`region::End::open`]), so a root at the apex
//!    is no crossing of it. A root within the band of an edge's end reads
//!    no side there. The legs start at `p`'s foot ([`region::QueryFoot`]).
//! 3. At the closest crossing along the path, the leg's heading against
//!    `N × τ` says whether the path leaves the face there (so `p` is in
//!    it) or enters it.
//!
//! An edge lying on a leg's own line or circle — the leg runs along it —
//! grazes where the leg reaches it.
//!
//! The paths read every loop of the face at once, rings included. An
//! edge both of whose sides are the face (a seam) is no boundary of it
//! and is left out. Nothing is read in the chart: no azimuth window, no
//! height range, no period.
//!
//! # The apex
//!
//! The apex is a point like any other. A `p` within the band's zero of it
//! is read from it, along the generators aimed at the boundary's points
//! on both nappes. A `p` on a nappe on which no edge of the face lies is
//! outside the face: the face is bounded, so a part of it on that nappe
//! would have a boundary there, and every path from `p` stays on its
//! nappe.
//!
//! # The boundary first
//!
//! Before any path, `p` is read against every edge on its own carrier
//! ([`super::region::circle_edge_hit`] for a circle or an ellipse,
//! [`crate::ray_walk::on_segment`] for a line): on an edge, or within the
//! band of one of its ends, is the point on the boundary ([`None`]); in
//! the band of one refuses.
//!
//! # Grazing
//!
//! Past that, every reading is about one path ([`crate::ray_walk`]): a
//! closest crossing at a vertex, a tie with the closest crossing, a
//! crossing at the path's start or within the band of its corner or its
//! end, a tangency of a leg with an edge, a root or a component in
//! band, or a leg running along an edge grazes; a path that meets no edge gives nothing to read; and the next
//! is tried.
//!
//! # Predicates (meters)
//!
//! - `bool_ruled_region_frame`: `p`'s distance from a cylinder's axis,
//!   on which every azimuth names a foot, or its foot's from a cone's
//!   apex, which reads `p` from the apex where it is `Zero`.
//! - `bool_ruled_region_nappe`: which side of a cone's apex a point lies
//!   along the axis — `p`'s, a generator's target, a conic edge's middle,
//!   a line edge's end.
//! - `bool_ruled_region_at`: a crossing's place along its leg, or an end
//!   of an edge on the leg's line, from the leg's start: arc length on a
//!   parallel, metres on a ruling.
//! - `bool_ruled_region_end`: a place along a leg against its end — a
//!   first leg's corner, or a generator's end at the apex.
//! - `bool_ruled_region_corner`: an aimed ruling's run to its corner,
//!   which says the side of `p` the target's parallel lies.
//! - `bool_ruled_region_order`: two crossings' places along the path.
//! - `bool_ruled_region_cross`: the sine between the leg and the edge's
//!   face side at the closest crossing, at the face's scale.
//! - `bool_ruled_region_component`: which of a ruling leg's plane's
//!   curves a root lies on, and an edge's offset from a leg's plane it is
//!   parallel to.
//! - `plane_crossing_*`: the roots, the plane door's own rows.
//! - `bool_ruled_region_arc_*` and `bool_ruled_region_edge{,_length}`:
//!   the boundary readings' rows.
//!
//! # Which bound makes each verdict sound
//!
//! A verdict is read only off a path whose every decision is definite;
//! every `Zero` below abandons the path and is never read as an answer,
//! and so does an edge on the leg's own line or circle. Where a leg meets
//! an edge is the plane door's root, sound under its own rows, on the
//! curve the component reads.
//!
//! - **at, end, corner, order**: positions along the path, in metres.
//! - **cross**: a sine times the face's scale (the cylinder's radius, a
//!   bound on the cone face's reach from its apex), the displacement the
//!   crossing angle buys within that scale of the crossing.

use geom_core::{Band, Decide, Margin, Point3, Sign, Vec3};

use geom_brep::curve_pair::on_surface::{Charted, SurfaceCurve};

use super::region::{
    self, End, Fault, Legs, Line, Paths, QueryFoot, Region, RegionEdge, RegionRefusal, Round,
    circle_edge_hit, in_band, targets,
};
use super::solid_contain::PointInSolidError;
use crate::body::Body;
use crate::entity::{FaceKey, LoopKey};
use crate::null::Scaffolding;
use crate::ray_walk::{self, RayFault, SegmentRows};
use crate::splitting::containment::ConicRows;
use crate::validate::decide;

/// A conic edge's boundary reading's rows (module docs).
const BOUNDARY: ConicRows = crate::rows::RULED_REGION_CONIC;

/// A line edge's boundary reading's rows ([`ray_walk::on_segment`]).
const LINE: SegmentRows = crate::rows::RULED_REGION_LINE;

/// The surface a ruled face lies on.
#[derive(Clone, Copy, Debug)]
enum Ruled<T: geom_core::Real> {
    /// Rulings parallel to the axis, at `radius` from it.
    Cylinder {
        origin: Point3<T>,
        axis: Vec3<T>,
        radius: T,
        u_ref: Vec3<T>,
    },
    /// Generators through `apex` at `half_angle` to the axis.
    Cone {
        apex: Point3<T>,
        axis: Vec3<T>,
        half_angle: T,
        u_ref: Vec3<T>,
    },
}

impl<T: Decide> Ruled<T> {
    /// The point the axis frame is measured from: the cylinder's origin,
    /// the cone's apex.
    fn base(&self) -> Point3<T> {
        match *self {
            Self::Cylinder { origin, .. } => origin,
            Self::Cone { apex, .. } => apex,
        }
    }

    fn axis(&self) -> Vec3<T> {
        match *self {
            Self::Cylinder { axis, .. } | Self::Cone { axis, .. } => axis,
        }
    }

    fn u_ref(&self) -> Vec3<T> {
        match *self {
            Self::Cylinder { u_ref, .. } | Self::Cone { u_ref, .. } => u_ref,
        }
    }

    /// A cylinder's or a cone's own fields, or `None` on any other
    /// surface.
    fn of(surface: &geom::Surface<T>) -> Option<Self> {
        Some(match *surface {
            geom::Surface::Cylinder {
                origin,
                axis,
                radius,
                u_ref,
            } => Self::Cylinder {
                origin,
                axis,
                radius,
                u_ref,
            },
            geom::Surface::Cone {
                apex,
                axis,
                half_angle,
                u_ref,
            } => Self::Cone {
                apex,
                axis,
                half_angle,
                u_ref,
            },
            _ => return None,
        })
    }
}

/// A trimmed cylinder or cone face's boundary: every edge of every loop,
/// each with the direction its loop traverses it.
#[derive(Clone, Debug)]
pub(crate) struct RuledFaceRegion<T: geom_core::Real> {
    /// The face's surface record, its key and surface: every leg is
    /// charted on it and every edge keyed by it.
    charted: Charted<T>,
    /// The record's surface's own fields ([`Ruled::of`]).
    surface: Ruled<T>,
    /// The face's sense bit: `false` points its outward normal at the
    /// axis.
    sense: bool,
    /// The face's outer loop, named by an exhaustion.
    outer: LoopKey,
    /// The lever of the crossing's sine: the cylinder's radius, or a
    /// bound on the farthest a cone face's boundary reaches from the
    /// apex, never under it.
    scale: T,
    edges: Vec<Edge<T>>,
}

/// One boundary edge: a line, a circle or an ellipse.
type Edge<T> = RegionEdge<T>;

/// The row a place is read against a leg's end on.
const END: &str = "bool_ruled_region_end";

/// A leg's course: a parallel, or a ruling.
#[derive(Clone, Copy)]
pub(crate) enum LegShape<T: geom_core::Real> {
    /// A parallel, run along increasing angle.
    Parallel(Round<T>),
    /// A ruling's line.
    Ruling(Line<T>),
}

impl<T: Decide> region::Course<T> for LegShape<T> {
    fn place(&self, y: Point3<T>, at: &'static str, band: Band) -> Result<Option<T>, Fault> {
        match self {
            Self::Parallel(round) => round.place(y, at, band),
            Self::Ruling(line) => line.place(y, at, band),
        }
    }

    fn reach(&self, y: Point3<T>, at: &'static str, band: Band) -> Result<Option<T>, Fault> {
        match self {
            Self::Parallel(round) => round.reach(y, at, band),
            Self::Ruling(line) => line.reach(y, at, band),
        }
    }

    fn heading(&self, y: Point3<T>) -> Vec3<T> {
        match self {
            Self::Parallel(round) => round.heading(y),
            Self::Ruling(line) => line.heading(y),
        }
    }

    fn curve(&self) -> SurfaceCurve<T> {
        match self {
            Self::Parallel(round) => round.curve(),
            Self::Ruling(line) => line.curve(),
        }
    }

    fn span(&self, end: Option<T>, edge: &Edge<T>) -> ((T, T), bool) {
        match self {
            Self::Parallel(round) => round.span(end, edge),
            Self::Ruling(line) => line.span(end, edge),
        }
    }
}

/// One leg of a path, along a parallel or a ruling.
type Leg<T> = region::Leg<T, LegShape<T>>;

/// `face`'s region on its cylinder or cone, or `None` when an edge of it
/// is not a line, a circle or an ellipse — a spline or a spiric — whose
/// crossings this reading has no closed form for. A line, circle or
/// ellipse is taken as a ruling, a parallel or a plane section of the
/// surface, unchecked. The premise is that every edge lies on the face's
/// surface, as a line on a cylinder or a cone is then a ruling and a
/// conic a plane section: tier 3 places each edge on both its faces'
/// carriers at rest (at a dual, [`crate::Replay`]'s "A tier-3 premise at a dual"), and on the
/// reduction's working copies an edge a face gains is a section of its carrier with the other
/// operand's, on both by construction.
///
/// Null scaffolding, a zero-length copy of a vertex (its two ends on one
/// point), bounds nothing and is left out; an uncertified edge between
/// two points answers `None`.
///
/// # Errors
///
/// [`PointInSolidError::CorruptFace`] for a surface that is neither a
/// cylinder nor a cone, or an outer loop with
/// no cycle.
///
/// # Panics
///
/// Where `face`, a face the kernel enumerated, or a boundary edge's
/// curve does not resolve (D2 row 4): a torn
/// curve is not null scaffolding. The links hold at rest and, on the
/// reduction's working copies, by
/// [`crate::live::OPERATORS_KEEP_LINKS`].
pub(crate) fn ruled_face_region<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
) -> Result<Option<RuledFaceRegion<T>>, PointInSolidError> {
    let corrupt = || PointInSolidError::CorruptFace { face };
    let f = body.proven_face(face);
    let surface = Ruled::of(f.surface()).ok_or_else(corrupt)?;
    let ruled = [
        geom::CurveKind::Line,
        geom::CurveKind::Circle,
        geom::CurveKind::Ellipse,
    ];
    let Some(edges) = region::boundary(body, face, Scaffolding::Skipped, &ruled)? else {
        return Ok(None);
    };
    let covers: Vec<_> = edges
        .iter()
        .map(|e| crate::cover::edge_cover(body, e.edge))
        .collect();
    let scale = reach_bound(&surface, &covers);
    Ok(Some(RuledFaceRegion {
        charted: Charted::record(f.record().surface, f.surface()),
        surface,
        sense: f.sense(),
        outer: f.record().outer,
        scale,
        edges,
    }))
}

/// The lever of a face's crossings ([`RuledFaceRegion::reach`]): the
/// cylinder's radius, or a bound on the farthest the cone face's
/// boundary reaches from the apex, never under it: edge by edge, its
/// edge cover's farthest distance from the apex
/// ([`crate::cover::edge_cover`], one per edge in `covers`), poison for an edge with no cover (its
/// carrier's reading refuses, or an end vertex is not a finite point),
/// which the readings levered at it escalate. The boundary was read
/// certified, so no edge of it is null scaffolding.
fn reach_bound<T: Decide>(
    surface: &Ruled<T>,
    covers: &[Result<geom::cover::Cover<T>, crate::cover::EdgeUncovered>],
) -> T {
    match *surface {
        Ruled::Cylinder { radius, .. } => radius,
        Ruled::Cone { apex, .. } => covers
            .iter()
            .map(|cover| {
                cover.as_ref().map_or_else(
                    |_| geom_core::poison(),
                    |c| c.distance_range(&geom::cover::Site::Point(apex)).1,
                )
            })
            .fold(T::zero(), T::max),
    }
}

impl<T: Decide> RuledFaceRegion<T> {
    /// Is `p`, a point on the cylinder or cone, in the face: `Some(true)`
    /// inside, `Some(false)` outside, `None` on its boundary (module docs).
    ///
    /// # Errors
    ///
    /// [`RegionRefusal`]: `p` in the band of an edge, `p`'s distance from
    /// the axis or the apex in band, no path decided ([`ray_walk::walk`]),
    /// or an arc that bounds nothing.
    pub(crate) fn contains(&self, p: Point3<T>, band: Band) -> Result<Option<bool>, RegionRefusal> {
        let esc = RegionRefusal::Escalated;
        // A point on a cylinder's axis has no foot of its own: every
        // azimuth names one, so it is refused before any foot is read.
        if let Ruled::Cylinder { .. } = self.surface {
            let (x, y, _) = self.frame().local(p);
            let rho = (x.powi(2) + y.powi(2)).sqrt();
            crate::validate::decide_positive("bool_ruled_region_frame", Margin::of(rho), band)
                .map_err(esc)?;
        }
        let foot = QueryFoot::read(self.charted.surface(), p, band);
        if self.on_boundary(foot.at(), band)? {
            return Ok(None);
        }
        let exhausted = || RegionRefusal::RayExhausted { outer: self.outer };
        let at_apex = match self.surface {
            Ruled::Cylinder { .. } => false,
            Ruled::Cone { apex, .. } => {
                decide(
                    "bool_ruled_region_frame",
                    Margin::of((foot.at() - apex).norm()),
                    band,
                )
                .map_err(esc)?
                    == Sign::Zero
            }
        };
        if at_apex {
            return ray_walk::walk(
                targets(&self.edges),
                |target| region::read_leg(self, &self.apex_generator(target, band)?, band),
                exhausted,
            )
            .map(Some);
        }
        if let Ruled::Cone { .. } = self.surface
            && !self.reaches_nappe(foot.at(), band)?
        {
            return Ok(Some(false));
        }
        region::walk(self, foot, band).map(Some)
    }

    /// The face's scale: the cylinder's radius, or a bound on the
    /// farthest its boundary reaches from the cone's apex, never under
    /// it.
    pub(crate) fn reach(&self) -> T {
        self.scale
    }

    /// A point of the face's surface the side-at-infinity reader may aim
    /// at: the mean of the boundary's middle points, carried onto the
    /// surface along its parallel, or onto the surface's reference
    /// azimuth where that mean is on the axis (a full band whose rims
    /// start half a turn apart). It need not lie in the face; it only
    /// steers. `None` for a face with no boundary edge, which offers no
    /// aim.
    pub(crate) fn aim(&self) -> Option<Point3<T>> {
        let base = self.surface.base();
        let first = self.edges.first()?;
        let middle = |e: &Edge<T>| e.at(geom::mid_param(e.t0, e.t1)) - base;
        let (sum, n) = self.edges[1..]
            .iter()
            .fold((middle(first), T::one()), |(sum, n), e| {
                (sum + middle(e), n + T::one())
            });
        let mean = sum / n;
        let axis = self.surface.axis();
        let h = mean.dot(axis);
        let radial = mean - axis * h;
        let rho = radial.norm();
        // Evaluation, not a decision: the meridian only steers.
        let on_axis = rho.select_le_zero(T::one(), T::zero());
        let dir = radial / rho.select_le_zero(T::one(), rho) + self.surface.u_ref() * on_axis;
        let at = match self.surface {
            Ruled::Cylinder { radius, .. } => radius,
            Ruled::Cone { half_angle, .. } => h.abs() * half_angle.tan(),
        };
        Some(base + axis * h + dir * at)
    }

    /// Whether any edge may lie on `p`'s nappe of the cone: one not
    /// definitely on the other. A conic edge, a bounded plane section,
    /// lies on one nappe and is read at its middle. A line edge, a
    /// generator, may run through the apex onto both and is read at its
    /// two ends: a segment lies on its ends' nappes and the apex, which
    /// is on neither, so an end within the band's zero of the apex says
    /// nothing. Whatever of the edge lies on `p`'s nappe within that zero
    /// bounds nothing as far from the apex as `p`, which its own nappe
    /// reading places clear of it.
    fn reaches_nappe(&self, p: Point3<T>, band: Band) -> Result<bool, RegionRefusal> {
        let own = self.nappe(p, band)?;
        for edge in &self.edges {
            let (points, at_apex_says_nothing) = if edge.carrier.is_curved() {
                (vec![edge.at(geom::mid_param(edge.t0, edge.t1))], false)
            } else {
                (vec![edge.at(edge.t0), edge.at(edge.t1)], true)
            };
            for y in points {
                match decide(
                    "bool_ruled_region_nappe",
                    Margin::of((y - self.surface.base()).dot(self.surface.axis())),
                    band,
                ) {
                    Ok(Sign::Positive) if !own => {}
                    Ok(Sign::Negative) if own => {}
                    Ok(Sign::Zero) if at_apex_says_nothing => {}
                    _ => return Ok(true),
                }
            }
        }
        Ok(false)
    }

    /// Which nappe of the cone `a`, a point clear of the apex, lies on:
    /// `true` the opening one.
    fn nappe(&self, a: Point3<T>, band: Band) -> Result<bool, RegionRefusal> {
        let h = (a - self.surface.base()).dot(self.surface.axis());
        Ok(
            crate::validate::decide_nonzero("bool_ruled_region_nappe", Margin::of(h), band)
                .map_err(RegionRefusal::Escalated)?
                == geom_core::NonzeroSign::Positive,
        )
    }

    /// The surface's frame.
    fn frame(&self) -> geom::Frame<T> {
        geom::Frame::new(
            self.surface.base(),
            self.surface.axis(),
            self.surface.u_ref(),
        )
    }

    /// The parallel through `p`, run `forward` (right-handed about the
    /// axis) or back: at the surface's own radius at `p`'s height.
    fn parallel(&self, p: QueryFoot<T>, forward: bool) -> Leg<T> {
        let curve =
            SurfaceCurve::parallel(&self.charted, p.uv(), forward).map(|c| c.starting_at(p.at()));
        Leg::whole(LegShape::Parallel(Round::on(region::built(curve))))
    }

    /// The ruling through `p`, a point clear of a cone's apex on the
    /// nappe `opening`, run `outward` or back; back toward a cone's apex
    /// it ends there.
    fn ruling(&self, p: QueryFoot<T>, opening: bool, outward: bool) -> Leg<T> {
        let end = match self.surface {
            Ruled::Cylinder { .. } => None,
            Ruled::Cone { apex, .. } => (!outward).then(|| (p.at() - apex).norm()),
        };
        let curve = SurfaceCurve::ruling(&self.charted, p.uv(), opening, outward);
        Leg {
            course: LegShape::Ruling(Line::on(region::built(curve))),
            end: end.map(|place| End {
                place,
                row: END,
                open: true,
            }),
        }
    }

    /// From a `p` at the cone's apex, the generator through `target`: it
    /// starts at the apex, the foot at the target's azimuth and no slant.
    fn apex_generator(&self, target: Point3<T>, band: Band) -> Result<Leg<T>, Fault> {
        if !matches!(self.surface, Ruled::Cone { .. }) {
            unreachable!("only a cone has an apex")
        }
        // A target at the apex's level gives no generator to run.
        let opening = self.nappe(target, band).map_err(|_| RayFault::Unread)?;
        let surface = self.charted.surface();
        let target = QueryFoot::read(surface, target, band);
        let curve = SurfaceCurve::from_apex(&self.charted, target.uv(), opening);
        Ok(Leg::whole(LegShape::Ruling(Line::on(region::built(curve)))))
    }
}

impl<T: Decide> Region<T> for RuledFaceRegion<T> {
    fn edges(&self) -> &[Edge<T>] {
        &self.edges
    }

    fn edges_mut(&mut self) -> &mut Vec<Edge<T>> {
        &mut self.edges
    }

    fn outer(&self) -> LoopKey {
        self.outer
    }

    /// A line as a segment ([`ray_walk::on_segment`]), a conic on its
    /// own carrier.
    fn on_edge(&self, edge: &Edge<T>, p: Point3<T>, band: Band) -> Result<bool, RegionRefusal> {
        if edge.carrier.is_curved() {
            circle_edge_hit(
                &edge.carrier,
                edge.turn.as_ref(),
                BOUNDARY,
                p,
                self.outer,
                band,
            )
        } else {
            ray_walk::on_segment(edge.at(edge.t0), edge.at(edge.t1), p, &LINE, band)
                .map_err(RegionRefusal::Escalated)
        }
    }
}

impl<T: Decide> Paths<T> for RuledFaceRegion<T> {
    /// `p`'s parallel, or its ruling run `forward` outward.
    fn single(
        &self,
        p: QueryFoot<T>,
        parallel: bool,
        forward: bool,
        band: Band,
    ) -> Result<Leg<T>, Fault> {
        Ok(if parallel {
            self.parallel(p, forward)
        } else {
            self.ruling(p, self.opening(p, band)?, forward)
        })
    }

    /// A first leg from `p` to a corner on `target`'s ruling
    /// (`parallel_first`, round `p`'s parallel) or parallel (along `p`'s
    /// ruling), then the other family's leg through the corner: the
    /// corner at `target`'s azimuth and `p`'s height, or `p`'s azimuth
    /// and `target`'s height (a cone's on `p`'s nappe at `target`'s reach
    /// from the apex).
    fn aimed(
        &self,
        p: QueryFoot<T>,
        target: Point3<T>,
        parallel_first: bool,
        first_forward: bool,
        then_forward: bool,
        band: Band,
    ) -> Result<(LegShape<T>, End<T>, impl FnOnce() -> Leg<T>), Fault> {
        let surface = self.charted.surface();
        let opening = self.opening(p, band)?;
        let target = QueryFoot::read(surface, target, band);
        let ((up, vp), (ut, vt)) = ((p.uv().x, p.uv().y), (target.uv().x, target.uv().y));
        // Both corners lie on `p`'s nappe, so the path does too, wherever
        // the target lies: it only steers.
        let (first, end, corner) = if parallel_first {
            let corner = QueryFoot::of_chart(surface, geom_core::Point2::new(ut, vp));
            let LegShape::Parallel(round) = self.parallel(p, first_forward).course else {
                unreachable!("a parallel leg is round")
            };
            (LegShape::Parallel(round), round.arc(corner.at()), corner)
        } else {
            // Along `p`'s ruling to `target`'s parallel: the side it lies
            // is decided, and a target on `p`'s own parallel is the
            // parallel's to read.
            let (v, gap) = match self.surface {
                Ruled::Cylinder { .. } => (vt, vt - vp),
                Ruled::Cone { .. } => {
                    let reach = vt.abs();
                    (
                        if opening { reach } else { T::zero() - reach },
                        reach - vp.abs(),
                    )
                }
            };
            let corner = QueryFoot::of_chart(surface, geom_core::Point2::new(up, v));
            let outward =
                match decide("bool_ruled_region_corner", Margin::of(gap), band).map_err(in_band)? {
                    Sign::Positive => true,
                    Sign::Negative => false,
                    Sign::Zero => return Err(RayFault::Unread),
                };
            // The corner lies short of the apex: the leg ends there.
            (self.ruling(p, opening, outward).course, gap.abs(), corner)
        };
        let then = move || {
            if parallel_first {
                self.ruling(corner, opening, then_forward)
            } else {
                self.parallel(corner, then_forward)
            }
        };
        Ok((
            first,
            End {
                place: end,
                row: END,
                open: false,
            },
            then,
        ))
    }
}

impl<T: Decide> RuledFaceRegion<T> {
    /// Which nappe `p` lies on (`true` the opening one, and every
    /// cylinder point).
    fn opening(&self, p: QueryFoot<T>, band: Band) -> Result<bool, Fault> {
        match self.surface {
            Ruled::Cylinder { .. } => Ok(true),
            Ruled::Cone { .. } => self.nappe(p.at(), band).map_err(RayFault::InBand),
        }
    }
}

impl<T: Decide> Legs<T> for RuledFaceRegion<T> {
    type Course = LegShape<T>;

    const AT: &'static str = "bool_ruled_region_at";

    const COMPONENT: &'static str = "bool_ruled_region_component";

    const SIDE_ROWS: (&'static str, &'static str) =
        ("bool_ruled_region_order", "bool_ruled_region_cross");

    fn lever(&self) -> T {
        self.scale
    }

    fn charted(&self) -> &Charted<T> {
        &self.charted
    }

    fn outward(&self, y: Point3<T>) -> Vec3<T> {
        geom_brep::implicit_outward_normal(self.charted.surface(), self.sense, y).vec()
    }
}

#[cfg(test)]
#[path = "ruled_region_rows.rs"]
mod ruled_region_rows;
