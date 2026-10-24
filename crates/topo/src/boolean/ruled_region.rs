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
//!   along the axis — `p`'s, a target's, an edge's middle.
//! - `bool_ruled_region_component`: which of a meridian plane's two
//!   points of an edge a root is (its offset from the axis along the
//!   leg's radial, and on a cone its nappe); whether a line edge lies on
//!   the leg's line (its ends' larger distance from it); whether it
//!   reaches the apex.
//! - `bool_ruled_region_span`: a root's parameter against its edge's span
//!   ends, at the conic's smaller semi-axis or the line's unit speed.
//! - `bool_ruled_region_at`: a crossing's place along its leg, from the
//!   leg's start: arc length on a parallel, metres on a ruling.
//! - `bool_ruled_region_corner`: a crossing, or an end of an edge on a
//!   leg's line, against a first leg's corner or a generator's end at the
//!   apex; and an aimed leg's length to its corner.
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
//! - **at, corner, order**: positions along the path, in metres.
//! - **cross**: a sine times the face's scale (the cylinder's radius, the
//!   cone face's reach from its apex), the displacement the crossing
//!   angle buys within that scale of the crossing.

use geom_core::{Band, Decide, Margin, Point3, Sign, Vec3};

use super::BooleanDecision;
use super::circle_roots::{
    FirstHarmonic, FirstHarmonicRows, first_harmonic_roots_in_band, near_span,
};
use super::region::{
    Hit, RegionRefusal, TARGET_SHARES, circle_edge_hit, found, in_band, side_at_closest, span, turn,
};
use super::solid_contain::PointInSolidError;
use crate::body::Body;
use crate::entity::{EdgeKey, FaceKey, LoopBoundary, LoopKey};
use crate::ray_walk::{self, Crossings, ParityRows, RayFault};
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
const BOUNDARY: ConicRows = ConicRows {
    span: "bool_ruled_region_arc_span",
    straddle: "bool_ruled_region_arc_straddle",
};

/// A line edge's boundary reading's rows: [`ray_walk::on_segment`] reads
/// `segment` and `boundary` and nothing else, so `side` and `advance`
/// are never minted.
const LINE: ParityRows = ParityRows {
    segment: "bool_ruled_region_edge_length",
    boundary: "bool_ruled_region_edge",
    side: "bool_ruled_region_edge_side",
    advance: "bool_ruled_region_edge_advance",
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
    /// The lever of the crossing's sine: the cylinder's radius, or the
    /// farthest a cone face's boundary reaches from the apex.
    scale: T,
    edges: Vec<RegionEdge<T>>,
}

/// One boundary edge — a line, a circle or an ellipse — over its
/// certified span `[t0, t1]`.
#[derive(Clone, Debug)]
struct RegionEdge<T: geom_core::Real> {
    edge: EdgeKey,
    curve: geom::Curve3<T>,
    t0: T,
    t1: T,
    /// Whether the loop runs `t0 → t1` (its `he_plus`).
    forward: bool,
}

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

/// An edge's carrier, as the crossings read it.
#[derive(Clone, Copy)]
enum Carrier<T: geom_core::Real> {
    Line { origin: Point3<T>, dir: Vec3<T> },
    Conic(Conic<T>),
}

impl<T: Decide> RegionEdge<T> {
    fn carrier(&self) -> Carrier<T> {
        match self.curve {
            geom::Curve3::Line { origin, dir } => Carrier::Line { origin, dir },
            geom::Curve3::Circle {
                center,
                axis,
                radius,
                u_ref,
            } => Carrier::Conic(Conic {
                center,
                axis,
                a: radius,
                b: radius,
                u: u_ref,
            }),
            geom::Curve3::Ellipse {
                center,
                axis,
                semi_u,
                semi_v,
                u_ref,
            } => Carrier::Conic(Conic {
                center,
                axis,
                a: semi_u,
                b: semi_v,
                u: u_ref,
            }),
            geom::Curve3::Spiric { .. } | geom::Curve3::Nurbs(_) => {
                unreachable!("ruled_face_region keeps lines, circles and ellipses only")
            }
        }
    }

    fn at(&self, t: T) -> Point3<T> {
        self.curve.eval(t)
    }

    /// The loop's unit traversal direction at `t`.
    fn traversal(&self, t: T) -> Vec3<T> {
        let d = self.curve.deriv(t);
        let d = d / d.norm();
        if self.forward { d } else { -d }
    }
}

/// One leg of a path, from `start` at place `0` along increasing place
/// (metres), ending at the place `end` where it is bounded.
#[derive(Clone, Copy)]
struct Leg<T: geom_core::Real> {
    shape: LegShape<T>,
    start: Point3<T>,
    end: Option<T>,
}

#[derive(Clone, Copy)]
enum LegShape<T: geom_core::Real> {
    /// The circle `center + radius·(e1 cos s + e2 sin s)` in the plane of
    /// unit normal `normal`, `e2 = normal × e1`, run along increasing `s`.
    Parallel {
        center: Point3<T>,
        normal: Vec3<T>,
        radius: T,
        e1: Vec3<T>,
        e2: Vec3<T>,
    },
    /// The line from `start` along unit `dir`. `radial` is the unit
    /// direction from the axis to the leg, in its meridian plane; on a
    /// cone, `nappe` is the side of the apex the leg runs on (`true` the
    /// opening one), and `to_apex` whether its end is the apex.
    Ruling {
        dir: Vec3<T>,
        radial: Vec3<T>,
        nappe: bool,
        to_apex: bool,
    },
}

impl<T: Decide> Leg<T> {
    /// The point at place `s` of a parallel.
    fn parallel_at(&self, s: T) -> Point3<T> {
        let LegShape::Parallel {
            center,
            radius,
            e1,
            e2,
            ..
        } = self.shape
        else {
            unreachable!("only a parallel leg is read by its angle")
        };
        let (sn, cs) = s.sin_cos();
        center + (e1 * cs + e2 * sn) * radius
    }

    /// The angle of `y`, a point of a parallel, from the start, in
    /// `(−π, π]`.
    fn angle(&self, y: Point3<T>) -> T {
        let LegShape::Parallel { center, e1, e2, .. } = self.shape else {
            unreachable!("only a parallel leg is read by its angle")
        };
        let w = y - center;
        w.dot(e2).atan2(w.dot(e1))
    }

    /// The unit direction of travel at `y`.
    fn heading(&self, y: Point3<T>) -> Vec3<T> {
        match self.shape {
            LegShape::Parallel { center, normal, .. } => {
                let h = normal.cross(y - center);
                h / h.norm()
            }
            LegShape::Ruling { dir, .. } => dir,
        }
    }

    /// Where `y`, a point of the leg's line or circle, lies along it:
    /// `Some(place)` ahead of the start, `None` behind it (a ruling's
    /// back half).
    fn place(&self, y: Point3<T>, band: Band) -> Result<Option<T>, RayFault<RegionRefusal>> {
        match self.shape {
            LegShape::Parallel { radius, .. } => {
                let s = self.angle(y);
                ray_walk::apart("bool_ruled_region_at", Margin::of((radius * s).abs()), band)
                    .map_err(|f| f.map(RegionRefusal::Escalated))?;
                Ok(Some(radius * turn(s)))
            }
            LegShape::Ruling { dir, .. } => {
                let s = (y - self.start).dot(dir);
                match ray_walk::advance("bool_ruled_region_at", Margin::of(s), band)
                    .map_err(|f| f.map(RegionRefusal::Escalated))?
                {
                    Sign::Negative => Ok(None),
                    _ => Ok(Some(s)),
                }
            }
        }
    }

    /// Whether a place ahead of the start lies short of the leg's end:
    /// `Some(true)` short of it, `Some(false)` past it.
    fn short_of_end(&self, s: T, band: Band) -> Result<bool, RayFault<RegionRefusal>> {
        let Some(end) = self.end else {
            return Ok(true);
        };
        match decide("bool_ruled_region_corner", Margin::of(end - s), band).map_err(in_band)? {
            Sign::Positive => Ok(true),
            Sign::Negative => Ok(false),
            Sign::Zero => Err(RayFault::Graze),
        }
    }
}

/// A path to read.
#[derive(Clone, Copy)]
enum PathSpec<T: geom_core::Real> {
    /// `p`'s parallel, run `forward` (about the axis) or back.
    Parallel { forward: bool },
    /// `p`'s ruling, run `outward` (along the axis on a cylinder, away
    /// from the apex on a cone) or back.
    Ruling { outward: bool },
    /// A first leg from `p` to a corner on `target`'s parallel
    /// (`parallel_first: false`, along `p`'s ruling) or ruling (round
    /// `p`'s parallel, `first_forward`), then the other family's leg
    /// through the corner, `then_forward`, which passes `target`.
    Aimed {
        target: Point3<T>,
        parallel_first: bool,
        first_forward: bool,
        then_forward: bool,
    },
    /// From a `p` at the cone's apex, the generator through `target`.
    FromApex { target: Point3<T> },
}

/// `face`'s region on its cylinder or cone, or `None` when an edge of it
/// is not a line, a circle or an ellipse — a spline or a spiric — whose
/// crossings this reading has no closed form for. A line, circle or
/// ellipse is taken as a ruling, a parallel or a plane section of the
/// surface, unchecked: every such edge of such a face is one at rest
/// (tier 3 places each edge on both its faces' carriers).
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
                // Null scaffolding joins two vertices on one point, and
                // bounds nothing; an uncertified edge between two points
                // is a gap in the boundary no reading may step over.
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
                return Ok(None);
            };
            match curve.carrier() {
                geom::Curve3::Line { .. }
                | geom::Curve3::Circle { .. }
                | geom::Curve3::Ellipse { .. } => {}
                geom::Curve3::Spiric { .. } | geom::Curve3::Nurbs(_) => return Ok(None),
            }
            let (t0, t1) = curve.params();
            edges.push(RegionEdge {
                edge: edge_key,
                curve: curve.carrier().clone(),
                t0,
                t1,
                forward: he == edge.he_plus,
            });
        }
    }
    let seam = |e: EdgeKey| {
        edges
            .iter()
            .filter(|a: &&RegionEdge<T>| a.edge == e)
            .count()
            > 1
    };
    let seams: Vec<EdgeKey> = edges.iter().map(|a| a.edge).filter(|&e| seam(e)).collect();
    edges.retain(|a| !seams.contains(&a.edge));
    let scale = match surface {
        Ruled::Cylinder { radius, .. } => radius,
        Ruled::Cone { apex, .. } => edges
            .iter()
            .flat_map(|e| [e.at(e.t0), e.at(e.t1), e.at(geom::mid_param(e.t0, e.t1))])
            .map(|y| (y - apex).norm())
            .fold(T::zero(), T::max),
    };
    Ok(Some(RuledFaceRegion {
        surface,
        sense: f.sense,
        outer: f.outer,
        scale,
        edges,
    }))
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
            let specs = self.targets().map(|target| PathSpec::FromApex { target });
            return ray_walk::walk(specs, |spec| self.path(p, spec, band), exhausted).map(Some);
        }
        if let Ruled::Cone { .. } = self.surface
            && !self.reaches_nappe(p, band)?
        {
            return Ok(Some(false));
        }
        let singles = [
            PathSpec::Parallel { forward: true },
            PathSpec::Parallel { forward: false },
            PathSpec::Ruling { outward: true },
            PathSpec::Ruling { outward: false },
        ];
        let both = [(true, true), (true, false), (false, true), (false, false)];
        let aimed = self.targets().flat_map(move |target| {
            [true, false].into_iter().flat_map(move |parallel_first| {
                both.map(move |(first_forward, then_forward)| PathSpec::Aimed {
                    target,
                    parallel_first,
                    first_forward,
                    then_forward,
                })
            })
        });
        ray_walk::walk(
            singles.into_iter().chain(aimed),
            |spec| self.path(p, spec, band),
            exhausted,
        )
        .map(Some)
    }

    /// The face's scale: the cylinder's radius, or the farthest its
    /// boundary reaches from the cone's apex.
    pub(crate) fn reach(&self) -> T {
        self.scale
    }

    /// A point of the face the side-at-infinity reader may aim at: the
    /// mean of the boundary's middle points, carried onto the surface
    /// along its parallel. It need not lie in the face; it only steers.
    pub(crate) fn aim(&self) -> Point3<T> {
        let base = self.surface.base();
        let (sum, n) = self
            .edges
            .iter()
            .fold((Vec3::zero(), T::zero()), |(sum, n), e| {
                (
                    sum + (e.at(geom::mid_param(e.t0, e.t1)) - base),
                    n + T::one(),
                )
            });
        let mean = sum / n;
        let axis = self.surface.axis();
        let h = mean.dot(axis);
        let radial = mean - axis * h;
        let radial = radial / radial.norm();
        let rho = match self.surface {
            Ruled::Cylinder { radius, .. } => radius,
            Ruled::Cone { half_angle, .. } => h.abs() * half_angle.tan(),
        };
        base + axis * h + radial * rho
    }

    /// Three points of every edge, in loop order: the aimed paths'
    /// targets.
    fn targets(&self) -> impl Iterator<Item = Point3<T>> + '_ {
        self.edges.iter().flat_map(|edge| {
            TARGET_SHARES
                .into_iter()
                .map(|share| edge.at(edge.t0 + (edge.t1 - edge.t0) * T::from_f64(share)))
        })
    }

    /// Whether any edge may lie on `p`'s nappe of the cone: one whose
    /// middle is not definitely on the other.
    fn reaches_nappe(&self, p: Point3<T>, band: Band) -> Result<bool, RegionRefusal> {
        let own = self.nappe(p, band)?;
        for edge in &self.edges {
            let mid = edge.at(geom::mid_param(edge.t0, edge.t1));
            match decide(
                "bool_ruled_region_nappe",
                Margin::of((mid - self.surface.base()).dot(self.surface.axis())),
                band,
            ) {
                Ok(Sign::Positive) if !own => {}
                Ok(Sign::Negative) if own => {}
                _ => return Ok(true),
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

    /// The boundary pre-pass: whether `p` is on an edge, read on the
    /// edge's own carrier.
    fn on_boundary(&self, p: Point3<T>, band: Band) -> Result<bool, RegionRefusal> {
        for edge in &self.edges {
            let on = match edge.curve {
                geom::Curve3::Line { .. } => {
                    ray_walk::on_segment(edge.at(edge.t0), edge.at(edge.t1), p, &LINE, band)
                        .map_err(RegionRefusal::Escalated)?
                }
                _ => circle_edge_hit(
                    &edge.curve,
                    (edge.t0, edge.t1),
                    BOUNDARY,
                    p,
                    self.outer,
                    band,
                )?,
            };
            if on {
                return Ok(true);
            }
        }
        Ok(false)
    }

    /// `a`'s distance from the axis and its height from the base, and
    /// the unit radial direction.
    fn polar(&self, a: Point3<T>) -> (T, T, Vec3<T>) {
        let axis = self.surface.axis();
        let w = a - self.surface.base();
        let h = w.dot(axis);
        let radial = w - axis * h;
        let rho = radial.norm();
        (rho, h, radial / rho)
    }

    /// The parallel through `a`, run `forward` (right-handed about the
    /// axis) or back.
    fn parallel(&self, a: Point3<T>, forward: bool) -> Leg<T> {
        let (rho, h, e1) = self.polar(a);
        let axis = self.surface.axis();
        let normal = if forward { axis } else { -axis };
        Leg {
            shape: LegShape::Parallel {
                center: self.surface.base() + axis * h,
                normal,
                radius: rho,
                e1,
                e2: normal.cross(e1),
            },
            start: a,
            end: None,
        }
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
            shape: LegShape::Ruling {
                dir,
                radial,
                nappe,
                to_apex: end.is_some(),
            },
            start: a,
            end,
        }
    }

    /// One path from `p` (module docs): whether `p` is in the face.
    fn path(
        &self,
        p: Point3<T>,
        spec: PathSpec<T>,
        band: Band,
    ) -> Result<bool, RayFault<RegionRefusal>> {
        let fault = |e: RegionRefusal| RayFault::InBand(e);
        let mut crossings = Crossings::new();
        match spec {
            PathSpec::Parallel { forward } => {
                let leg = self.parallel(p, forward);
                self.leg_crossings(&leg, T::zero(), &mut crossings, band)?;
            }
            PathSpec::Ruling { outward } => {
                let nappe = self.p_nappe(p, band).map_err(fault)?;
                let leg = self.ruling(p, nappe, outward);
                self.leg_crossings(&leg, T::zero(), &mut crossings, band)?;
            }
            PathSpec::FromApex { target } => {
                let Ruled::Cone { apex, .. } = self.surface else {
                    unreachable!("only a cone has an apex")
                };
                let nappe = match self.nappe(target, band) {
                    Ok(nappe) => nappe,
                    Err(_) => return Err(RayFault::Unread),
                };
                let (_, _, radial) = self.polar(target);
                let g = target - apex;
                let leg = Leg {
                    shape: LegShape::Ruling {
                        dir: g / g.norm(),
                        radial,
                        nappe,
                        to_apex: false,
                    },
                    start: apex,
                    end: None,
                };
                self.leg_crossings(&leg, T::zero(), &mut crossings, band)?;
            }
            PathSpec::Aimed {
                target,
                parallel_first,
                first_forward,
                then_forward,
            } => {
                let nappe = self.p_nappe(p, band).map_err(fault)?;
                // A path never crosses the apex: a target on the other
                // nappe is out of its reach.
                if let Ruled::Cone { .. } = self.surface
                    && self.nappe(target, band).ok() != Some(nappe)
                {
                    return Err(RayFault::Unread);
                }
                let (first, corner) = if parallel_first {
                    let (rho_p, h_p, _) = self.polar(p);
                    let (_, _, radial_t) = self.polar(target);
                    let corner = self.surface.base() + self.surface.axis() * h_p + radial_t * rho_p;
                    let mut first = self.parallel(p, first_forward);
                    first.end = Some(rho_p * turn(first.angle(corner)));
                    (first, corner)
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
                    let outward = match decide("bool_ruled_region_corner", Margin::of(gap), band)
                        .map_err(in_band)?
                    {
                        Sign::Positive => true,
                        Sign::Negative => false,
                        Sign::Zero => return Err(RayFault::Unread),
                    };
                    let mut first = self.ruling(p, nappe, outward);
                    first.end = Some(gap.abs());
                    (first, corner)
                };
                let kept = self.leg_crossings(&first, T::zero(), &mut crossings, band)?;
                // Past a crossing before the corner, every crossing of the
                // second leg lies beyond it, decided apart from the corner
                // by its own rows.
                if !kept {
                    let offset = first.end.unwrap_or_else(T::zero);
                    let then = if parallel_first {
                        self.ruling(corner, nappe, then_forward)
                    } else {
                        self.parallel(corner, then_forward)
                    };
                    self.leg_crossings(&then, offset, &mut crossings, band)?;
                }
            }
        }
        // A path meeting no edge lies wholly on one side.
        side_at_closest(
            crossings,
            ("bool_ruled_region_order", "bool_ruled_region_cross"),
            self.scale,
            band,
        )
    }

    /// `p`'s nappe on a cone (`true` on a cylinder, which has one).
    fn p_nappe(&self, p: Point3<T>, band: Band) -> Result<bool, RegionRefusal> {
        match self.surface {
            Ruled::Cylinder { .. } => Ok(true),
            Ruled::Cone { .. } => self.nappe(p, band),
        }
    }

    /// Offers every crossing of `leg` with the boundary to `crossings`,
    /// at `offset` plus its place along the leg, short of the leg's end.
    /// Whether one was offered.
    fn leg_crossings(
        &self,
        leg: &Leg<T>,
        offset: T,
        crossings: &mut Crossings<T, Hit<T>>,
        band: Band,
    ) -> Result<bool, RayFault<RegionRefusal>> {
        let surface = self.surface.surface();
        let mut offered = false;
        for edge in &self.edges {
            for (y, t, sideless) in self.landings(edge, leg, band)? {
                let Some(s) = leg.place(y, band)? else {
                    continue;
                };
                if !leg.short_of_end(s, band)? {
                    continue;
                }
                let outward = geom_brep::implicit_outward_normal(&surface, self.sense, y);
                crossings.push(
                    offset + s,
                    Hit {
                        heading: leg.heading(y),
                        face_side: outward.vec().cross(edge.traversal(t)),
                    },
                    sideless,
                );
                offered = true;
            }
        }
        Ok(offered)
    }

    /// Where `leg`'s line or circle meets `edge`: each crossing's point,
    /// the edge's parameter there, and whether it lies at an end of the
    /// edge (module docs, step 2). Where along the leg it lies is the
    /// caller's to read.
    #[allow(clippy::type_complexity)] // the three readings of one crossing
    fn landings(
        &self,
        edge: &RegionEdge<T>,
        leg: &Leg<T>,
        band: Band,
    ) -> Result<Vec<(Point3<T>, T, bool)>, RayFault<RegionRefusal>> {
        let row = |name, m: T| decide(name, Margin::of(m), band).map_err(in_band);
        let span =
            |t: T, lever: T| span("bool_ruled_region_span", t, (edge.t0, edge.t1), lever, band);
        let axis = self.surface.axis();
        let base = self.surface.base();
        let mut out = Vec::new();
        match (edge.carrier(), leg.shape) {
            (
                Carrier::Conic(conic),
                LegShape::Parallel {
                    center,
                    radius,
                    normal,
                    e1,
                    ..
                },
            ) => {
                let roots = first_harmonic_roots_in_band(
                    &FirstHarmonic::circle_off_plane(
                        (center, normal, radius, e1),
                        (conic.center, conic.axis),
                    ),
                    radius,
                    T::zero(),
                    T::tau(),
                    &ROOT_ROWS,
                    band,
                );
                // A circle in the leg's plane is the leg's own parallel:
                // the plane holds no other circle of the surface.
                if let Ok(Roots::On(_)) = roots {
                    self.along(edge, leg, band)?;
                    return Ok(out);
                }
                for s in found(roots)? {
                    let y = leg.parallel_at(s);
                    let theta = conic.angle(y, (edge.t0, edge.t1));
                    if let Some(sideless) = span(theta, conic.slow())? {
                        out.push((y, theta, sideless));
                    }
                }
            }
            (Carrier::Line { origin, dir }, LegShape::Parallel { center, .. }) => {
                // A ruling crosses the parallel's plane once, at the
                // parallel's one point on it.
                let t = (center - origin).dot(axis) / dir.dot(axis);
                if let Some(sideless) = span(t, T::one())? {
                    out.push((origin + dir * t, t, sideless));
                }
            }
            (Carrier::Conic(conic), LegShape::Ruling { radial, nappe, .. }) => {
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
                        match row("bool_ruled_region_component", w.dot(axis))? {
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
            (Carrier::Line { .. }, LegShape::Ruling { dir, to_apex, .. }) => {
                let ends = [edge.at(edge.t0), edge.at(edge.t1)];
                let off_line = |y: Point3<T>| {
                    let w = y - leg.start;
                    (w - dir * w.dot(dir)).norm()
                };
                match row(
                    "bool_ruled_region_component",
                    off_line(ends[0]).max(off_line(ends[1])),
                )? {
                    Sign::Zero => self.along(edge, leg, band)?,
                    // Two rulings apart: a cylinder's never meet, a cone's
                    // only at the apex, where a leg reaches only its end.
                    _ => {
                        if let Ruled::Cone { apex, .. } = self.surface
                            && to_apex
                        {
                            let near = (ends[0] - apex).norm().min((ends[1] - apex).norm());
                            if row("bool_ruled_region_component", near)? == Sign::Zero {
                                return Err(RayFault::Graze);
                            }
                        }
                    }
                }
            }
        }
        Ok(out)
    }

    /// An edge on the leg's own line or circle: the leg runs along it if
    /// it reaches it, which grazes. A leg reaches every point of its
    /// circle, and of its line from its start to its end; neither end of
    /// the edge is the leg's start, which the boundary pre-pass placed off
    /// the edge.
    fn along(
        &self,
        edge: &RegionEdge<T>,
        leg: &Leg<T>,
        band: Band,
    ) -> Result<(), RayFault<RegionRefusal>> {
        let mut behind = 0;
        let mut past = 0;
        for t in [edge.t0, edge.t1] {
            let y = edge.at(t);
            let s = match leg.shape {
                LegShape::Parallel { radius, .. } => radius * turn(leg.angle(y)),
                LegShape::Ruling { dir, .. } => {
                    let s = (y - leg.start).dot(dir);
                    match decide("bool_ruled_region_corner", Margin::of(s), band)
                        .map_err(in_band)?
                    {
                        Sign::Negative => {
                            behind += 1;
                            continue;
                        }
                        Sign::Positive => s,
                        Sign::Zero => return Err(RayFault::Graze),
                    }
                }
            };
            let Some(end) = leg.end else {
                return Err(RayFault::Graze);
            };
            match decide("bool_ruled_region_corner", Margin::of(end - s), band).map_err(in_band)? {
                Sign::Negative => past += 1,
                Sign::Positive | Sign::Zero => return Err(RayFault::Graze),
            }
        }
        if behind == 2 || past == 2 {
            Ok(())
        } else {
            Err(RayFault::Graze)
        }
    }
}

#[cfg(test)]
#[path = "ruled_region_rows.rs"]
mod ruled_region_rows;
