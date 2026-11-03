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
//! 2. Every edge meets a leg where the curve-pair reading says they
//!    meet ([`region::landings`], [`geom_brep::curve_pair::meet`]), a
//!    ruling leg read over its run (to the apex where it ends there, and
//!    otherwise past every point of the edge) and a parallel over its
//!    whole turn. A meeting within the band of an edge's end reads no
//!    side there.
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
//! end, a tangency of a leg with an edge, a meeting the curve-pair
//! reading refuses short of the band, or a leg running along an edge
//! grazes; a path that meets no edge gives nothing to read; and the next
//! is tried.
//!
//! # Predicates (meters)
//!
//! - `bool_ruled_region_frame`: `p`'s distance from a cylinder's axis,
//!   which the legs divide by, or from a cone's apex, which reads `p`
//!   from the apex where it is `Zero`.
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
//! - `curve_pair_*`: where a leg meets an edge, the curve-pair reading's
//!   own rows.
//! - `bool_ruled_region_arc_*` and `bool_ruled_region_edge{,_length}`:
//!   the boundary readings' rows.
//!
//! # Which bound makes each verdict sound
//!
//! A verdict is read only off a path whose every decision is definite;
//! every `Zero` below abandons the path and is never read as an answer,
//! and so does an edge on the leg's own line or circle. Where a leg meets
//! an edge is the curve-pair reading's, sound under its own rows.
//!
//! - **at, end, corner, order**: positions along the path, in metres.
//! - **cross**: a sine times the face's scale (the cylinder's radius, a
//!   bound on the cone face's reach from its apex), the displacement the
//!   crossing angle buys within that scale of the crossing.

use geom_core::{Band, Decide, Margin, Point3, Sign, Vec3};

use super::region::{
    self, End, Fault, Legs, Line, Paths, Region, RegionEdge, RegionRefusal, Round, Scaffolding,
    circle_edge_hit, in_band, targets,
};
use super::solid_contain::PointInSolidError;
use crate::body::Body;
use crate::entity::{FaceKey, LoopKey};
use crate::ray_walk::{self, RayFault, SegmentRows};
use crate::splitting::containment::ConicRows;
use crate::validate::decide;

/// A conic edge's boundary reading's rows (module docs).
const BOUNDARY: ConicRows = ConicRows::bounded(
    "bool_ruled_region_arc_span",
    "bool_ruled_region_arc_straddle",
);

/// A line edge's boundary reading's rows ([`ray_walk::on_segment`]).
const LINE: SegmentRows = SegmentRows {
    segment: "bool_ruled_region_edge_length",
    boundary: "bool_ruled_region_edge",
};

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

    fn surface(&self) -> geom::Surface<T> {
        match *self {
            Self::Cylinder {
                origin,
                axis,
                radius,
                u_ref,
            } => geom::Surface::Cylinder {
                origin,
                axis,
                radius,
                u_ref,
            },
            Self::Cone {
                apex,
                axis,
                half_angle,
                u_ref,
            } => geom::Surface::Cone {
                apex,
                axis,
                half_angle,
                u_ref,
            },
        }
    }
}

/// A trimmed cylinder or cone face's boundary: every edge of every loop,
/// each with the direction its loop traverses it.
#[derive(Clone, Debug)]
pub(crate) struct RuledFaceRegion<T: geom_core::Real> {
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

    fn carrier(&self) -> geom::Curve3<T> {
        match self {
            Self::Parallel(round) => round.carrier(),
            Self::Ruling(line) => line.carrier(),
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
/// carriers at rest, and on the reduction's working copies an edge a
/// face gains is a section of its carrier with the other operand's, on
/// both by construction.
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
    let f = crate::live::proven(&body.faces, face, crate::entity::EntityId::Face);
    let surface = match *body.face_surface_linked(face, f) {
        geom::Surface::Cylinder {
            origin,
            axis,
            radius,
            u_ref,
        } => Ruled::Cylinder {
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
        } => Ruled::Cone {
            apex,
            axis,
            half_angle,
            u_ref,
        },
        _ => return Err(corrupt()),
    };
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
        .map(|e| super::boxes::edge_cover(body, e.edge))
        .collect();
    let scale = reach_bound(&surface, &covers);
    Ok(Some(RuledFaceRegion {
        surface,
        sense: f.sense,
        outer: f.outer,
        scale,
        edges,
    }))
}

/// The lever of a face's crossings ([`RuledFaceRegion::reach`]): the
/// cylinder's radius, or a bound on the farthest the cone face's
/// boundary reaches from the apex, never under it: edge by edge, its
/// edge cover's farthest distance from the apex
/// ([`super::boxes::edge_cover`], one per edge in `covers`), poison for an edge with no cover (its
/// carrier's reading refuses, or an end vertex is not a finite point),
/// which the readings levered at it escalate. The boundary was read
/// certified, so no edge of it is null scaffolding.
fn reach_bound<T: Decide>(
    surface: &Ruled<T>,
    covers: &[Result<geom::cover::Cover<T>, super::EdgeUncovered>],
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
        if self.on_boundary(p, band)? {
            return Ok(None);
        }
        let exhausted = || RegionRefusal::RayExhausted { outer: self.outer };
        let at_apex = match self.surface {
            // Every leg divides by `p`'s distance from the axis, decided
            // rather than premised, as the door does not certify `p` onto
            // the wall.
            Ruled::Cylinder { .. } => {
                let (rho, _, _) = self.polar(p);
                crate::validate::decide_positive("bool_ruled_region_frame", Margin::of(rho), band)
                    .map_err(esc)?;
                false
            }
            Ruled::Cone { apex, .. } => {
                decide(
                    "bool_ruled_region_frame",
                    Margin::of((p - apex).norm()),
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
            && !self.reaches_nappe(p, band)?
        {
            return Ok(Some(false));
        }
        region::walk(self, p, band).map(Some)
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
                == geom_core::k_stats::NonzeroSign::Positive,
        )
    }

    /// `a`'s distance from the axis and its height from the base, and
    /// the unit radial direction.
    fn polar(&self, a: Point3<T>) -> (T, T, Vec3<T>) {
        region::polar(self.surface.base(), self.surface.axis(), a)
    }

    /// The parallel through `a`, run `forward` (right-handed about the
    /// axis) or back.
    fn parallel(&self, a: Point3<T>, forward: bool) -> Leg<T> {
        Leg::whole(LegShape::Parallel(region::parallel(
            self.surface.base(),
            self.surface.axis(),
            a,
            forward,
        )))
    }

    /// The ruling through `a`, a point clear of a cone's apex, run
    /// `outward` or back; back toward a cone's apex it ends there.
    fn ruling(&self, a: Point3<T>, outward: bool) -> Leg<T> {
        let (dir, end) = match self.surface {
            Ruled::Cylinder { axis, .. } => (if outward { axis } else { -axis }, None),
            Ruled::Cone { apex, .. } => {
                let g = a - apex;
                let reach = g.norm();
                let g = g / reach;
                if outward {
                    (g, None)
                } else {
                    (-g, Some(reach))
                }
            }
        };
        Leg {
            course: LegShape::Ruling(Line { start: a, dir }),
            end: end.map(|place| End {
                place,
                row: END,
                open: true,
            }),
        }
    }

    /// From a `p` at the cone's apex, the generator through `target`.
    fn apex_generator(&self, target: Point3<T>, band: Band) -> Result<Leg<T>, Fault> {
        let Ruled::Cone { apex, .. } = self.surface else {
            unreachable!("only a cone has an apex")
        };
        // A target at the apex's level gives no generator to run.
        self.nappe(target, band).map_err(|_| RayFault::Unread)?;
        let g = target - apex;
        Ok(Leg::whole(LegShape::Ruling(Line {
            start: apex,
            dir: g / g.norm(),
        })))
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
        p: Point3<T>,
        parallel: bool,
        forward: bool,
        _: Band,
    ) -> Result<Leg<T>, Fault> {
        Ok(if parallel {
            self.parallel(p, forward)
        } else {
            self.ruling(p, forward)
        })
    }

    /// A first leg from `p` to a corner on `target`'s ruling
    /// (`parallel_first`, round `p`'s parallel) or parallel (along `p`'s
    /// ruling), then the other family's leg through the corner.
    fn aimed(
        &self,
        p: Point3<T>,
        target: Point3<T>,
        parallel_first: bool,
        first_forward: bool,
        then_forward: bool,
        band: Band,
    ) -> Result<(LegShape<T>, End<T>, impl FnOnce() -> Leg<T>), Fault> {
        // Both corners lie on `p`'s nappe (on its parallel, or on
        // its generator at the target's reach), so the path does
        // too, wherever the target lies: it only steers.
        let (first, end, corner) = if parallel_first {
            let (rho_p, h_p, _) = self.polar(p);
            let (_, _, radial_t) = self.polar(target);
            let corner = self.surface.base() + self.surface.axis() * h_p + radial_t * rho_p;
            let round =
                region::parallel(self.surface.base(), self.surface.axis(), p, first_forward);
            (LegShape::Parallel(round), round.arc(corner), corner)
        } else {
            // Along `p`'s ruling to `target`'s parallel: the side
            // it lies is decided, and a target on `p`'s own
            // parallel is the parallel's to read.
            let (corner, gap) = match self.surface {
                Ruled::Cylinder { axis, .. } => {
                    let ruling = geom::Line {
                        origin: p,
                        dir: axis,
                    };
                    (ruling.foot(target), ruling.param(target))
                }
                Ruled::Cone { apex, .. } => {
                    let (rp, rt) = ((p - apex).norm(), (target - apex).norm());
                    (apex + (p - apex) * (rt / rp), rt - rp)
                }
            };
            let outward =
                match decide("bool_ruled_region_corner", Margin::of(gap), band).map_err(in_band)? {
                    Sign::Positive => true,
                    Sign::Negative => false,
                    Sign::Zero => return Err(RayFault::Unread),
                };
            // The corner lies short of the apex: the leg ends there.
            (self.ruling(p, outward).course, gap.abs(), corner)
        };
        let then = move || {
            if parallel_first {
                self.ruling(corner, then_forward)
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

impl<T: Decide> Legs<T> for RuledFaceRegion<T> {
    type Course = LegShape<T>;

    const AT: &'static str = "bool_ruled_region_at";

    const SIDE: (&'static str, &'static str) =
        ("bool_ruled_region_order", "bool_ruled_region_cross");

    fn lever(&self) -> T {
        self.scale
    }

    fn outward(&self, y: Point3<T>) -> Vec3<T> {
        geom_brep::implicit_outward_normal(&self.surface.surface(), self.sense, y).vec()
    }
}

#[cfg(test)]
#[path = "ruled_region_rows.rs"]
mod ruled_region_rows;
