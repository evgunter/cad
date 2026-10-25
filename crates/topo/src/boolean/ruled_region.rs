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
//! 2. A parallel leg meets every edge in the edge's plane: a circle edge
//!    is a parallel (on these surfaces every circle is), whose plane is
//!    the leg's own or misses it; a planar section's plane meets the leg
//!    at the roots of a first harmonic in the leg's parameter, certified
//!    by the shared door
//!    ([`super::circle_roots::first_harmonic_roots_in_band`]); a line
//!    edge, a ruling, crosses the leg's plane once, in closed form. A
//!    ruling leg meets a circle or a section in the edge's own parameter
//!    against the leg's meridian plane (the plane through the axis and
//!    the leg), which holds two of the edge's points, one each side of the
//!    axis; the one on the leg's side is the crossing. A line edge is
//!    another ruling: on the leg's own line, or apart from it (a cone's
//!    generators meet only at the apex, which a leg reaches only at its
//!    end). The edge's parameter at each crossing says whether it lies in
//!    the edge's span.
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
//! end, a tangency of a leg with an edge (the root door's refusal), a leg
//! running along an edge, or a root no component reading places grazes;
//! a path that meets no edge gives nothing to read; and the next is tried.
//!
//! # Predicates (meters)
//!
//! - `bool_ruled_region_frame`: `p`'s distance from a cylinder's axis,
//!   which the legs divide by, or from a cone's apex, which reads `p`
//!   from the apex where it is `Zero`.
//! - `bool_ruled_region_nappe`: which side of a cone's apex a point lies
//!   along the axis — `p`'s, a generator's target, a conic edge's middle,
//!   a line edge's end, a conic's crossing with a generator's meridian
//!   plane.
//! - `bool_ruled_region_component`: which of a meridian plane's two
//!   points of an edge a root is: its offset from the axis along the
//!   leg's radial.
//! - `bool_ruled_region_collinear`: whether a line edge lies on a ruling
//!   leg's line: its ends' larger distance from it.
//! - `bool_ruled_region_span`: a root's parameter against its edge's span
//!   ends, at the conic's smaller semi-axis or the line's unit speed.
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
//! - `bool_ruled_region_roots_*`: the first-harmonic door's own meters.
//! - `bool_ruled_region_arc_*` and `bool_ruled_region_edge{,_length}`:
//!   the boundary readings' rows.
//!
//! # Which bound makes each verdict sound
//!
//! A verdict is read only off a path whose every decision is definite;
//! every `Zero` below abandons the path and is never read as an answer,
//! except where an edge is read as on the leg's own line or circle, which
//! also only abandons it.
//!
//! - **span**: a root's error is within the band in arc length (the root
//!   door's slack row, metered at the speed of the curve it is a
//!   parameter of — the leg's radius, or the conic's larger semi-axis),
//!   and the lever is a lower bound on the edge's speed, so a definite
//!   sign is the true root's side of the end. A line edge's crossing of a
//!   parallel's plane is closed-form arithmetic in its own metres.
//! - **component**: every sign read is a separation, never a Zero: a
//!   meridian plane's two points of an edge are `2ρ` apart across the
//!   axis, and a cone's two nappes are apart wherever a point is clear of
//!   the apex.
//! - **at, end, corner, order**: positions along the path, in metres.
//! - **cross**: a sine times the face's scale (the cylinder's radius, a
//!   bound on the cone face's reach from its apex), the displacement the
//!   crossing angle buys within that scale of the crossing.

use geom_core::{Band, Decide, Margin, Point3, Sign, Vec3};

use super::BooleanDecision;
use super::circle_roots::{
    FirstHarmonic, FirstHarmonicRows, first_harmonic_roots_in_band, near_span,
};
use super::region::{
    self, End, Fault, Landing, Legs, Line, Paths, Region, RegionEdge, RegionRefusal, Round,
    Scaffolding, circle_edge_hit, found, in_band, span, targets,
};
use super::solid_contain::PointInSolidError;
use crate::body::Body;
use crate::entity::{FaceKey, LoopKey};
use crate::ray_walk::{self, RayFault, SegmentRows};
use crate::splitting::containment::ConicRows;
use crate::validate::decide;
use geom_brep::roots::Roots;

const ROOT_ROWS: FirstHarmonicRows = FirstHarmonicRows {
    noise: "bool_ruled_region_roots_noise",
    coaxial: "bool_ruled_region_roots_coaxial",
    extreme: "bool_ruled_region_roots_extreme",
    root_slack: "bool_ruled_region_roots_slack",
    decision: BooleanDecision::CONTAINMENT_UNNAMED,
};

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

/// A conic edge's frame: `center + a·û cos θ + b·v̂ sin θ`, `v̂ = axis × û`.
#[derive(Clone, Copy)]
struct Conic<T: geom_core::Real> {
    center: Point3<T>,
    axis: Vec3<T>,
    a: T,
    b: T,
    u: Vec3<T>,
}

impl<T: Decide> Conic<T> {
    /// A circle or ellipse edge's frame.
    fn of(curve: &geom::Curve3<T>) -> Self {
        match *curve {
            geom::Curve3::Circle {
                center,
                axis,
                radius,
                u_ref,
            } => Self {
                center,
                axis,
                a: radius,
                b: radius,
                u: u_ref,
            },
            geom::Curve3::Ellipse {
                center,
                axis,
                semi_u,
                semi_v,
                u_ref,
            } => Self {
                center,
                axis,
                a: semi_u,
                b: semi_v,
                u: u_ref,
            },
            geom::Curve3::Line { .. } | geom::Curve3::Spiric { .. } | geom::Curve3::Nurbs(_) => {
                unreachable!("a conic edge of a ruled face is a circle or an ellipse")
            }
        }
    }

    /// The parameter of `y`, a point of the conic, on the branch of
    /// `span`'s middle.
    fn angle(&self, y: Point3<T>, span: (T, T)) -> T {
        let w = y - self.center;
        let v = self.axis.cross(self.u);
        near_span((w.dot(v) / self.b).atan2(w.dot(self.u) / self.a), span)
    }

    /// The smaller semi-axis: a lower bound on the conic's speed.
    fn slow(&self) -> T {
        self.a.min(self.b)
    }
}

/// A leg's course: a parallel, or a ruling.
#[derive(Clone, Copy)]
pub(crate) enum LegShape<T: geom_core::Real> {
    /// A parallel, run along increasing angle.
    Parallel(Round<T>),
    /// A ruling's `line`. `radial` is the unit direction from the axis to
    /// the leg, in its meridian plane; on a cone, `nappe` is the side of
    /// the apex the leg runs on (`true` the opening one).
    Ruling {
        line: Line<T>,
        radial: Vec3<T>,
        nappe: bool,
    },
}

impl<T: Decide> region::Course<T> for LegShape<T> {
    fn place(&self, y: Point3<T>, at: &'static str, band: Band) -> Result<Option<T>, Fault> {
        match self {
            Self::Parallel(round) => round.place(y, at, band),
            Self::Ruling { line, .. } => line.place(y, at, band),
        }
    }

    fn reach(&self, y: Point3<T>, at: &'static str, band: Band) -> Result<Option<T>, Fault> {
        match self {
            Self::Parallel(round) => round.reach(y, at, band),
            Self::Ruling { line, .. } => line.reach(y, at, band),
        }
    }

    fn heading(&self, y: Point3<T>) -> Vec3<T> {
        match self {
            Self::Parallel(round) => round.heading(y),
            Self::Ruling { line, .. } => line.heading(y),
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
/// [`PointInSolidError::CorruptFace`] for a key that does not resolve,
/// a surface that is neither a cylinder nor a cone, or an outer loop with
/// no cycle.
///
/// # Panics
///
/// Where a boundary edge's curve does not resolve (D2 row 4): a torn
/// curve is not null scaffolding. The links hold at rest and, on the
/// reduction's working copies, by
/// [`crate::live::OPERATORS_KEEP_LINKS`].
pub(crate) fn ruled_face_region<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
) -> Result<Option<RuledFaceRegion<T>>, PointInSolidError> {
    let corrupt = || PointInSolidError::CorruptFace { face };
    let f = body.get_face(face).ok_or_else(corrupt)?;
    let surface = match *body.get_surface(f.surface).ok_or_else(corrupt)? {
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
    let scale = reach_bound(&surface, &edges);
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
/// boundary reaches from the apex, never under it. It is bounded edge by
/// edge: a segment's distance from the apex is convex along it, so its
/// farther end bounds it, and a conic lies within its larger semi-axis
/// of its centre.
fn reach_bound<T: Decide>(surface: &Ruled<T>, edges: &[Edge<T>]) -> T {
    match *surface {
        Ruled::Cylinder { radius, .. } => radius,
        Ruled::Cone { apex, .. } => edges
            .iter()
            .map(|e| match e.carrier {
                geom::Curve3::Line { .. } => {
                    (e.at(e.t0) - apex).norm().max((e.at(e.t1) - apex).norm())
                }
                ref conic => {
                    let c = Conic::of(conic);
                    (c.center - apex).norm() + c.a.max(c.b)
                }
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
            let (points, at_apex_says_nothing) = match edge.carrier {
                geom::Curve3::Line { .. } => (vec![edge.at(edge.t0), edge.at(edge.t1)], true),
                _ => (vec![edge.at(geom::mid_param(edge.t0, edge.t1))], false),
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

    /// The ruling through `a`, a point clear of a cone's apex on
    /// `nappe`, run `outward` or back; back toward a cone's apex it ends
    /// there.
    fn ruling(&self, a: Point3<T>, nappe: bool, outward: bool) -> Leg<T> {
        let (_, _, radial) = self.polar(a);
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
            course: LegShape::Ruling {
                line: Line { start: a, dir },
                radial,
                nappe,
            },
            end: end.map(|place| End { place, row: END }),
        }
    }

    /// From a `p` at the cone's apex, the generator through `target`.
    fn apex_generator(&self, target: Point3<T>, band: Band) -> Result<Leg<T>, Fault> {
        let Ruled::Cone { apex, .. } = self.surface else {
            unreachable!("only a cone has an apex")
        };
        let nappe = match self.nappe(target, band) {
            Ok(nappe) => nappe,
            Err(_) => return Err(RayFault::Unread),
        };
        let (_, _, radial) = self.polar(target);
        let g = target - apex;
        Ok(Leg::whole(LegShape::Ruling {
            line: Line {
                start: apex,
                dir: g / g.norm(),
            },
            radial,
            nappe,
        }))
    }

    /// `p`'s nappe on a cone (`true` on a cylinder, which has one).
    fn p_nappe(&self, p: Point3<T>, band: Band) -> Result<bool, RegionRefusal> {
        match self.surface {
            Ruled::Cylinder { .. } => Ok(true),
            Ruled::Cone { .. } => self.nappe(p, band),
        }
    }
}

impl<T: Decide> Region<T> for RuledFaceRegion<T> {
    fn edges(&self) -> &[Edge<T>] {
        &self.edges
    }

    fn outer(&self) -> LoopKey {
        self.outer
    }

    /// A line as a segment ([`ray_walk::on_segment`]), a conic on its
    /// own carrier.
    fn on_edge(&self, edge: &Edge<T>, p: Point3<T>, band: Band) -> Result<bool, RegionRefusal> {
        match edge.carrier {
            geom::Curve3::Line { .. } => {
                ray_walk::on_segment(edge.at(edge.t0), edge.at(edge.t1), p, &LINE, band)
                    .map_err(RegionRefusal::Escalated)
            }
            _ => circle_edge_hit(
                &edge.carrier,
                edge.turn.as_ref(),
                BOUNDARY,
                p,
                self.outer,
                band,
            ),
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
        band: Band,
    ) -> Result<Leg<T>, Fault> {
        if parallel {
            Ok(self.parallel(p, forward))
        } else {
            let nappe = self.p_nappe(p, band).map_err(RayFault::InBand)?;
            Ok(self.ruling(p, nappe, forward))
        }
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
        let nappe = self.p_nappe(p, band).map_err(RayFault::InBand)?;
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
                    let gap = (target - p).dot(axis);
                    (p + axis * gap, gap)
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
            (self.ruling(p, nappe, outward).course, gap.abs(), corner)
        };
        let then = move || {
            if parallel_first {
                self.ruling(corner, nappe, then_forward)
            } else {
                self.parallel(corner, then_forward)
            }
        };
        Ok((
            first,
            End {
                place: end,
                row: END,
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

    /// Where `leg`'s line or circle meets `edge`: each crossing's point,
    /// the edge's parameter there, and whether it lies at an end of the
    /// edge (module docs, step 2). Where along the leg it lies is the
    /// caller's to read.
    fn landings(&self, edge: &Edge<T>, leg: &Leg<T>, band: Band) -> Result<Vec<Landing<T>>, Fault> {
        let row = |name, m: T| decide(name, Margin::of(m), band).map_err(in_band);
        let span =
            |t: T, lever: T| span("bool_ruled_region_span", t, (edge.t0, edge.t1), lever, band);
        let axis = self.surface.axis();
        let base = self.surface.base();
        let mut out = Vec::new();
        match (&edge.carrier, leg.course) {
            (&geom::Curve3::Line { origin, dir }, LegShape::Parallel(Round { center, .. })) => {
                // A ruling crosses the parallel's plane once, at the
                // parallel's one point on it.
                let t = (center - origin).dot(axis) / dir.dot(axis);
                if let Some(sideless) = span(t, T::one())? {
                    out.push((origin + dir * t, t, sideless));
                }
            }
            (geom::Curve3::Line { .. }, LegShape::Ruling { line, .. }) => {
                let ends = [edge.at(edge.t0), edge.at(edge.t1)];
                let off_line = |y: Point3<T>| {
                    let w = y - line.start;
                    (w - line.dir * w.dot(line.dir)).norm()
                };
                // Two rulings apart meet nowhere the leg runs: a
                // cylinder's never meet, and a cone's only at the apex,
                // which a leg reaches only at its end, where nothing is
                // read.
                if row(
                    "bool_ruled_region_collinear",
                    off_line(ends[0]).max(off_line(ends[1])),
                )? == Sign::Zero
                {
                    leg.along(edge, Self::AT, band)?;
                }
            }
            (conic, LegShape::Parallel(round)) => {
                let conic = Conic::of(conic);
                let roots = first_harmonic_roots_in_band(
                    &FirstHarmonic::circle_off_plane(
                        (round.center, round.normal, round.radius, round.e1),
                        (conic.center, conic.axis),
                    ),
                    round.radius,
                    T::zero(),
                    T::tau(),
                    &ROOT_ROWS,
                    band,
                );
                // A circle in the leg's plane is the leg's own parallel:
                // the plane holds no other circle of the surface.
                if let Ok(Roots::On(_)) = roots {
                    leg.along(edge, Self::AT, band)?;
                    return Ok(out);
                }
                for s in found(roots)? {
                    let y = round.at(s);
                    let theta = conic.angle(y, (edge.t0, edge.t1));
                    if let Some(sideless) = span(theta, conic.slow())? {
                        out.push((y, theta, sideless));
                    }
                }
            }
            (conic, LegShape::Ruling { radial, nappe, .. }) => {
                let conic = Conic::of(conic);
                let normal = axis.cross(radial);
                let roots = first_harmonic_roots_in_band(
                    &FirstHarmonic::conic_off_plane(
                        (conic.center, conic.axis, (conic.a, conic.b), conic.u),
                        (base, normal),
                    ),
                    conic.a.max(conic.b),
                    edge.t0,
                    edge.t1,
                    &ROOT_ROWS,
                    band,
                );
                for theta in found(roots)? {
                    let Some(sideless) = span(theta, conic.slow())? else {
                        continue;
                    };
                    let y = edge.at(theta);
                    let w = y - base;
                    // The meridian plane holds the edge's two points
                    // `2ρ` apart across the axis, and on a cone the leg
                    // runs on one nappe.
                    if let Ruled::Cone { .. } = self.surface {
                        match row("bool_ruled_region_nappe", w.dot(axis))? {
                            Sign::Positive if nappe => {}
                            Sign::Negative if !nappe => {}
                            Sign::Zero => return Err(RayFault::Graze),
                            _ => continue,
                        }
                    }
                    match row("bool_ruled_region_component", w.dot(radial))? {
                        Sign::Positive => out.push((y, theta, sideless)),
                        Sign::Negative => {}
                        Sign::Zero => return Err(RayFault::Graze),
                    }
                }
            }
        }
        Ok(out)
    }

    fn outward(&self, y: Point3<T>) -> Vec3<T> {
        geom_brep::implicit_outward_normal(&self.surface.surface(), self.sense, y).vec()
    }
}

#[cfg(test)]
#[path = "ruled_region_rows.rs"]
mod ruled_region_rows;
