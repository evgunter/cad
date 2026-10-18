//! **A torus face's region, read from its boundary edges** — the one
//! reading both point doors take of a torus face outside a closed group:
//! the ray lane's hit test and boundary pre-pass
//! ([`super::solid_contain`]) and the pierce arm's landing test
//! ([`super::contain::curved_face_containment`]). It is
//! [`super::sphere_region`]'s method on the torus.
//!
//! # Method: the closest crossing along a path of circles
//!
//! Every boundary edge of such a face lies in a plane: a parallel, a
//! meridian or a Villarceau circle, or an arc of a spiric oval (the
//! section by a plane parallel to the axis). The face is the side of its
//! loops to their left under its OUTWARD normal `N` (the torus's chart
//! normal with the face's sense folded in), so at a point of an edge's
//! interior with traversal tangent `τ` the face lies along `N × τ`. The
//! closest crossing carries that reference to the query:
//!
//! 1. Walk a PATH from `p` along circles of the torus: `p`'s parallel or
//!    its meridian, either way round; or, where neither settles, a
//!    parallel then a meridian (or the mirror) through a corner, aimed
//!    at a point of the boundary. Each leg is a circle in a plane: a
//!    parallel's is normal to the axis, a meridian's holds it.
//! 2. Every edge meets a leg at the roots of a first harmonic, certified
//!    by the shared first-harmonic door
//!    ([`super::circle_roots::first_harmonic_roots_in_band`]): a leg's
//!    parameter against the edge's plane, except a circle on a meridian
//!    leg, which is read in its own parameter against the leg's plane, as
//!    the sphere's arcs meet its rays. A plane through the axis, or a
//!    meridian's or Villarceau circle's, meets the torus in two circles,
//!    and an oval's plane in two ovals, so each root is then asked which
//!    one it lies on, and the edge's parameter there whether it lies in
//!    the edge's span.
//! 3. At the closest crossing along the path, the leg's heading against
//!    `N × τ` says whether the path leaves the face there (so `p` is in
//!    it) or enters it.
//!
//! **Why the plane is chosen per pair.** A plane meets a curve on the
//! torus at a double root wherever the plane is tangent to the torus, so
//! each pair is read against the plane that is transversal to it: a
//! meridian's plane is normal to the torus everywhere, an oval's (in the
//! two-oval regime) nowhere tangent to it, and a meridian edge's holds the
//! axis. A parallel's plane is tangent along the tube's top and bottom
//! circles, which bound every fillet, so a parallel edge is read only in
//! its own parameter (on a meridian leg; a parallel leg meets it nowhere
//! or runs along it). A Villarceau circle's plane is tangent at the two
//! points where it meets its mirror, which is where a crossing of it
//! grazes.
//!
//! An edge lying on a leg's own circle — the leg runs along it — grazes
//! where the path reaches it; a first leg that turns at its corner short
//! of it does not.
//!
//! The paths read every loop of the face at once, rings included. An
//! edge both of whose sides are the face (a seam) is no boundary of it
//! and is left out, so a face whose every edge is a seam covers the
//! torus. Nothing is read in the chart: the torus's two periods never
//! enter a reading.
//!
//! # The boundary first
//!
//! Before any path, `p` is read against every edge on its own carrier
//! ([`ConicArc::hit`] for a circle, [`SpiricArc::contact`] for an oval):
//! on an edge, or within the band of one of its ends, is the point on
//! the boundary ([`None`]); in the band of one refuses.
//!
//! # Grazing
//!
//! Past that, every reading is about one path ([`crate::ray_walk`]): a
//! closest crossing at a vertex, a tie with the closest crossing, a
//! crossing at the path's start or within the band of its corner, a
//! tangency of a leg with an edge (the root door's refusal), a leg
//! running along an edge, or a root the component reading cannot place
//! grazes; a path that meets no edge gives nothing to read; and the next
//! is tried.
//!
//! # Predicates (meters)
//!
//! - `bool_torus_frame_radius`: `p`'s distance from the axis, which the
//!   legs divide by.
//! - `bool_torus_region_component`: which of a plane's two curves a root
//!   lies on — a meridian plane's `(y − c)·ρ̂`, a circle edge's distance
//!   from its mirror and then from itself, an oval's `(y − c)·m`; and
//!   whether an edge in a leg's plane is on the leg's own circle.
//! - `bool_torus_region_span`: a root's parameter against its edge's span
//!   ends, at the circle's radius or the tube's.
//! - `bool_torus_region_at`: a crossing's place along its leg, from the
//!   leg's start, at the leg's radius.
//! - `bool_torus_region_corner`: a first leg's crossing, or an end of an
//!   edge on its circle, against its corner.
//! - `bool_torus_region_order`: two crossings' places along the path.
//! - `bool_torus_region_cross`: the sine between the leg and the edge's
//!   face side at the closest crossing, at the tube's radius.
//! - `bool_torus_region_roots_*`: the first-harmonic door's own meters.
//! - `bool_torus_region_arc_*` and `bool_torus_region_spiric_*`: the
//!   boundary readings' rows.
//!
//! # Which bound makes each verdict sound
//!
//! A verdict is read only off a path whose every decision is definite;
//! every `Zero` below abandons the path and is never read as an answer,
//! except where an edge in a leg's plane is read as on the leg's circle,
//! which also only abandons it.
//!
//! - **span**: a root's error is within the band in arc length (the root
//!   door's slack row, metered at the speed of the circle it is a
//!   parameter of), and the lever is the edge's speed or a lower bound on
//!   it (the tube's radius bounds an oval's below), so a definite sign is
//!   the true root's side of the end.
//! - **component**: every sign read is a separation, never a Zero: a
//!   meridian plane's two circles are `2(R − r)` apart, an oval plane's
//!   two ovals `2√((R − r)² − o²)`; a circle edge's root definitely off
//!   its mirror is on the edge's circle, as the plane holds no third
//!   curve, and one in band of both is where they cross, which grazes.
//! - **at, corner, order**: positions on circles, each an angle times the
//!   leg's radius — arc length, the metre a root's slack is bounded in.
//! - **cross**: a sine times the tube's radius, the displacement the
//!   crossing angle buys within one tube radius of the crossing.

use geom_core::{Band, Decide, Margin, Point3, Sign, Vec3};

use super::BooleanDecision;
use super::circle_roots::{
    FirstHarmonic, FirstHarmonicRoots, FirstHarmonicRows, first_harmonic_roots_in_band,
};
use super::solid_contain::PointInSolidError;
use super::sphere_region::RegionRefusal;
use crate::body::Body;
use crate::entity::{EdgeKey, FaceKey, LoopBoundary, LoopKey};
use crate::ray_walk::{self, Crossings, RayFault};
use crate::splitting::containment::{ConicArc, ConicArcError, ConicHit, ConicRows};
use crate::splitting::spiric_arc::{Oval, SpiricArc, SpiricHit, SpiricRefusal, SpiricRows};
use crate::validate::decide;
use geom_brep::roots::{RootRefusal, Roots};
use geom_brep::rounding_charge;

const ROOT_ROWS: FirstHarmonicRows = FirstHarmonicRows {
    noise: "bool_torus_region_roots_noise",
    coaxial: "bool_torus_region_roots_coaxial",
    extreme: "bool_torus_region_roots_extreme",
    root_slack: "bool_torus_region_roots_slack",
    decision: BooleanDecision::CONTAINMENT_UNNAMED,
};

/// A circle edge's boundary reading's rows (module docs).
const BOUNDARY: ConicRows = ConicRows {
    span: "bool_torus_region_arc_span",
    straddle: "bool_torus_region_arc_straddle",
};

/// An oval edge's boundary reading's rows (module docs).
const SPIRIC: SpiricRows = SpiricRows {
    end: "bool_torus_region_spiric_end",
    clear: "bool_torus_region_spiric_clear",
    on: "bool_torus_region_spiric_on",
    leaf: "bool_torus_region_spiric_leaf",
};

/// Where along each edge an aimed path's target lies, as shares of its
/// span.
const TARGET_SHARES: [f64; 3] = [0.5, 0.25, 0.75];

/// A trimmed torus face's boundary: every edge of every loop, each with
/// the direction its loop traverses it.
#[derive(Clone, Debug)]
pub(crate) struct TorusFaceRegion<T: geom_core::Real> {
    center: Point3<T>,
    axis: Vec3<T>,
    major: T,
    minor: T,
    u_ref: Vec3<T>,
    /// The face's sense bit: `false` points its outward normal into the
    /// tube.
    sense: bool,
    /// The face's outer loop, named by an exhaustion.
    outer: LoopKey,
    edges: Vec<RegionEdge<T>>,
}

/// One boundary edge's planar carrier.
#[derive(Clone, Copy, Debug)]
enum Carrier<T: geom_core::Real> {
    /// `center + radius·(û cos θ + v̂ sin θ)`, `v̂ = axis × û`.
    Circle {
        center: Point3<T>,
        axis: Vec3<T>,
        radius: T,
        u_ref: Vec3<T>,
    },
    /// One oval of a spiric ([`geom::Curve3::Spiric`]'s data).
    Spiric {
        center: Point3<T>,
        axis: Vec3<T>,
        u_ref: Vec3<T>,
        major: T,
        minor: T,
        offset: T,
    },
}

/// One boundary edge over its certified span `[t0, t1]`.
#[derive(Clone, Debug)]
struct RegionEdge<T: geom_core::Real> {
    edge: EdgeKey,
    carrier: Carrier<T>,
    t0: T,
    t1: T,
    /// Whether the loop runs `t0 → t1` (its `he_plus`).
    forward: bool,
}

impl<T: Decide> RegionEdge<T> {
    fn curve(&self) -> geom::Curve3<T> {
        match self.carrier {
            Carrier::Circle {
                center,
                axis,
                radius,
                u_ref,
            } => geom::Curve3::Circle {
                center,
                axis,
                radius,
                u_ref,
            },
            Carrier::Spiric {
                center,
                axis,
                u_ref,
                major,
                minor,
                offset,
            } => geom::Curve3::Spiric {
                center,
                axis,
                u_ref,
                major_radius: major,
                minor_radius: minor,
                offset,
            },
        }
    }

    fn at(&self, t: T) -> Point3<T> {
        self.curve().eval(t)
    }

    /// The loop's unit traversal direction at `t`.
    fn traversal(&self, t: T) -> Vec3<T> {
        let d = self.curve().deriv(t);
        let d = d / d.norm();
        if self.forward { d } else { -d }
    }
}

/// Which family a leg's circle belongs to.
#[derive(Clone, Copy)]
enum LegKind<T: geom_core::Real> {
    /// A parallel: its plane is normal to the axis.
    Parallel,
    /// A meridian: its plane holds the axis and meets the torus in two
    /// meridians; `radial` points from the axis to this one.
    Meridian { radial: Vec3<T> },
}

/// One leg of a path: the circle `center + radius·(e1 cos s + e2 sin s)`
/// in the plane of unit normal `normal`, `e2 = normal × e1`, from its
/// start at `s = 0` along increasing `s`.
#[derive(Clone, Copy)]
struct Leg<T: geom_core::Real> {
    kind: LegKind<T>,
    center: Point3<T>,
    normal: Vec3<T>,
    radius: T,
    e1: Vec3<T>,
    e2: Vec3<T>,
}

impl<T: Decide> Leg<T> {
    fn at(&self, s: T) -> Point3<T> {
        let (sn, cs) = s.sin_cos();
        self.center + (self.e1 * cs + self.e2 * sn) * self.radius
    }

    /// The angle of `y`, a point of the leg's circle, from the start, in
    /// `(−π, π]`.
    fn angle(&self, y: Point3<T>) -> T {
        let w = y - self.center;
        w.dot(self.e2).atan2(w.dot(self.e1))
    }

    /// The unit direction of travel at `y`.
    fn heading(&self, y: Point3<T>) -> Vec3<T> {
        let h = self.normal.cross(y - self.center);
        h / h.norm()
    }
}

/// A path to read: a leg from `p`, or two through a corner.
#[derive(Clone, Copy)]
enum PathSpec<T: geom_core::Real> {
    /// `p`'s parallel (`parallel`) or meridian, run `forward` or back.
    Single { parallel: bool, forward: bool },
    /// A first leg from `p` to the corner at `target`'s major angle
    /// (`parallel_first`) or minor angle, then the other family's leg
    /// through the corner, which passes `target`.
    Aimed {
        target: Point3<T>,
        parallel_first: bool,
        first_forward: bool,
        then_forward: bool,
    },
}

/// A crossing's reading at the closest: the leg's heading and the face's
/// side of the edge there.
struct Hit<T: geom_core::Real> {
    heading: Vec3<T>,
    face_side: Vec3<T>,
}

/// `face`'s region on its torus, or `None` when an edge of it lies in no
/// plane — a spline — or is a carrier no torus holds.
///
/// Null scaffolding, a zero-length copy of a vertex (its two ends on one
/// point), bounds nothing and is left out; an uncertified edge between
/// two points answers `None`.
///
/// # Errors
///
/// [`PointInSolidError::CorruptFace`] for a key that does not resolve,
/// a surface that is not a torus, or an outer loop with no cycle.
///
/// # Panics
///
/// Where a boundary edge's curve does not resolve (D2 row 4): a torn
/// curve is not null scaffolding. The links hold at rest and, on the
/// reduction's working copies, by
/// [`crate::live::OPERATORS_KEEP_LINKS`].
pub(crate) fn torus_face_region<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
) -> Result<Option<TorusFaceRegion<T>>, PointInSolidError> {
    let corrupt = || PointInSolidError::CorruptFace { face };
    let f = body.get_face(face).ok_or_else(corrupt)?;
    let &geom::Surface::Torus {
        center,
        axis,
        major_radius,
        minor_radius,
        u_ref,
    } = body.get_surface(f.surface).ok_or_else(corrupt)?
    else {
        return Err(corrupt());
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
            let carrier = match *curve.carrier() {
                geom::Curve3::Circle {
                    center,
                    axis,
                    radius,
                    u_ref,
                } => Carrier::Circle {
                    center,
                    axis,
                    radius,
                    u_ref,
                },
                geom::Curve3::Spiric {
                    center,
                    axis,
                    u_ref,
                    major_radius,
                    minor_radius,
                    offset,
                } => Carrier::Spiric {
                    center,
                    axis,
                    u_ref,
                    major: major_radius,
                    minor: minor_radius,
                    offset,
                },
                _ => return Ok(None),
            };
            let (t0, t1) = curve.params();
            edges.push(RegionEdge {
                edge: edge_key,
                carrier,
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
    Ok(Some(TorusFaceRegion {
        center,
        axis,
        major: major_radius,
        minor: minor_radius,
        u_ref,
        sense: f.sense,
        outer: f.outer,
        edges,
    }))
}

impl<T: Decide> TorusFaceRegion<T> {
    /// Is `p`, a point on the torus, in the face: `Some(true)` inside,
    /// `Some(false)` outside, `None` on its boundary (module docs).
    ///
    /// # Errors
    ///
    /// [`RegionRefusal`]: `p` in the band of an edge, a torus off the
    /// ring convention or a point on its axis, no path decided
    /// ([`ray_walk::walk`]), or an arc that bounds nothing.
    pub(crate) fn contains(&self, p: Point3<T>, band: Band) -> Result<Option<bool>, RegionRefusal> {
        if self.edges.is_empty() {
            return Ok(Some(true));
        }
        // Every leg divides by a distance from the axis, which the ring
        // convention keeps at least `R − r` on the torus; `p`'s own is
        // decided rather than premised, as the door does not certify `p`
        // onto the tube.
        geom::require_ring_torus(self.major, self.minor, band).map_err(RegionRefusal::Escalated)?;
        let (rho, _, _) = self.polar(p);
        crate::validate::decide_positive("bool_torus_frame_radius", Margin::of(rho), band)
            .map_err(RegionRefusal::Escalated)?;
        if self.on_boundary(p, band)? {
            return Ok(None);
        }
        let both = [(true, true), (true, false), (false, true), (false, false)];
        let singles = both.map(|(parallel, forward)| PathSpec::Single { parallel, forward });
        let aimed = self.edges.iter().flat_map(move |edge| {
            TARGET_SHARES.into_iter().flat_map(move |share| {
                let target = edge.at(edge.t0 + (edge.t1 - edge.t0) * T::from_f64(share));
                [true, false].into_iter().flat_map(move |parallel_first| {
                    both.map(move |(first_forward, then_forward)| PathSpec::Aimed {
                        target,
                        parallel_first,
                        first_forward,
                        then_forward,
                    })
                })
            })
        });
        ray_walk::walk(
            singles.into_iter().chain(aimed),
            |spec| self.path(p, spec, band),
            || RegionRefusal::RayExhausted { outer: self.outer },
        )
        .map(Some)
    }

    /// The boundary pre-pass: whether `p` is on an edge, read on the
    /// edge's own carrier.
    fn on_boundary(&self, p: Point3<T>, band: Band) -> Result<bool, RegionRefusal> {
        for edge in &self.edges {
            match edge.carrier {
                Carrier::Circle { .. } => {
                    let k = match ConicArc::of(&edge.curve(), (edge.t0, edge.t1), BOUNDARY, band) {
                        Ok(Some(k)) => k,
                        Err(ConicArcError::Escalated(e)) => {
                            return Err(RegionRefusal::Escalated(e.diag));
                        }
                        Ok(None) | Err(ConicArcError::WoundPastPeriod) => {
                            return Err(RegionRefusal::WoundPastPeriod);
                        }
                    };
                    match k
                        .hit(p, band)
                        .map_err(|e| RegionRefusal::Escalated(e.diag))?
                    {
                        ConicHit::On(_) | ConicHit::End => return Ok(true),
                        ConicHit::Off | ConicHit::Carrier => {}
                        // A circle's `Past` stands on its nearest foot and
                        // walks nothing, so this does not arise; were it
                        // to, the point is unplaced and no path starts
                        // from it.
                        ConicHit::Unsettled => {
                            return Err(RegionRefusal::RayExhausted { outer: self.outer });
                        }
                    }
                }
                Carrier::Spiric {
                    center,
                    axis,
                    u_ref,
                    major,
                    minor,
                    offset,
                } => {
                    let oval = Oval {
                        center,
                        axis,
                        u_ref,
                        major,
                        minor,
                        offset,
                    };
                    match SpiricArc::new(oval, (edge.t0, edge.t1)).contact(p, SPIRIC, band) {
                        Ok(SpiricHit::On(_) | SpiricHit::End) => return Ok(true),
                        Ok(SpiricHit::Off) => {}
                        Err(SpiricRefusal::Escalated(diag)) => {
                            return Err(RegionRefusal::Escalated(diag));
                        }
                        Err(SpiricRefusal::Unsettled) => {
                            return Err(RegionRefusal::RayExhausted { outer: self.outer });
                        }
                    }
                }
            }
        }
        Ok(false)
    }

    /// `a`'s distance from the axis and its height, and the unit radial
    /// direction.
    fn polar(&self, a: Point3<T>) -> (T, T, Vec3<T>) {
        let w = a - self.center;
        let h = w.dot(self.axis);
        let radial = w - self.axis * h;
        let rho = radial.norm();
        (rho, h, radial / rho)
    }

    /// The parallel through `a`, run `forward` (the major angle
    /// increasing) or back.
    fn parallel(&self, a: Point3<T>, forward: bool) -> Leg<T> {
        let (rho, h, e1) = self.polar(a);
        let normal = if forward { self.axis } else { -self.axis };
        Leg {
            kind: LegKind::Parallel,
            center: self.center + self.axis * h,
            normal,
            radius: rho,
            e1,
            e2: normal.cross(e1),
        }
    }

    /// The meridian through `a`, run `forward` (the minor angle
    /// increasing) or back.
    fn meridian(&self, a: Point3<T>, forward: bool) -> Leg<T> {
        let (_, _, radial) = self.polar(a);
        let center = self.center + radial * self.major;
        let off = a - center;
        let e1 = off / off.norm();
        let normal = radial.cross(self.axis);
        let normal = if forward { normal } else { -normal };
        Leg {
            kind: LegKind::Meridian { radial },
            center,
            normal,
            radius: self.minor,
            e1,
            e2: normal.cross(e1),
        }
    }

    /// One path from `p` (module docs): whether `p` is in the face.
    fn path(
        &self,
        p: Point3<T>,
        spec: PathSpec<T>,
        band: Band,
    ) -> Result<bool, RayFault<RegionRefusal>> {
        let mut crossings = Crossings::new();
        match spec {
            PathSpec::Single { parallel, forward } => {
                let leg = if parallel {
                    self.parallel(p, forward)
                } else {
                    self.meridian(p, forward)
                };
                self.leg_crossings(&leg, T::zero(), None, &mut crossings, band)?;
            }
            PathSpec::Aimed {
                target,
                parallel_first,
                first_forward,
                then_forward,
            } => {
                let (rho_p, h_p, radial_p) = self.polar(p);
                let (rho_t, h_t, radial_t) = self.polar(target);
                let (first, corner) = if parallel_first {
                    (
                        self.parallel(p, first_forward),
                        self.center + self.axis * h_p + radial_t * rho_p,
                    )
                } else {
                    (
                        self.meridian(p, first_forward),
                        self.center + self.axis * h_t + radial_p * rho_t,
                    )
                };
                let to_corner = turn(first.angle(corner));
                let kept =
                    self.leg_crossings(&first, T::zero(), Some(to_corner), &mut crossings, band)?;
                // Past a crossing before the corner, every crossing of the
                // second leg lies beyond it, decided apart from the corner
                // by its own rows.
                if !kept {
                    let then = if parallel_first {
                        self.meridian(corner, then_forward)
                    } else {
                        self.parallel(corner, then_forward)
                    };
                    self.leg_crossings(
                        &then,
                        first.radius * to_corner,
                        None,
                        &mut crossings,
                        band,
                    )?;
                }
            }
        }
        let hit = crossings
            .closest("bool_torus_region_order", band, |diag, _| {
                RegionRefusal::Escalated(diag)
            })?
            // A path meeting no edge lies wholly on one side.
            .ok_or(RayFault::Unread)?;
        match decide(
            "bool_torus_region_cross",
            Margin::of(self.minor * hit.heading.dot(hit.face_side)),
            band,
        )
        .map_err(in_band)?
        {
            Sign::Negative => Ok(true),
            Sign::Positive => Ok(false),
            Sign::Zero => Err(RayFault::Graze),
        }
    }

    /// Offers every crossing of `leg` with the boundary to `crossings`,
    /// at `offset` plus its arc length along the leg, short of the angle
    /// `corner` along it where the leg ends there. Whether one was
    /// offered.
    fn leg_crossings(
        &self,
        leg: &Leg<T>,
        offset: T,
        corner: Option<T>,
        crossings: &mut Crossings<T, Hit<T>>,
        band: Band,
    ) -> Result<bool, RayFault<RegionRefusal>> {
        let row = |name, m: T| decide(name, Margin::of(m), band).map_err(in_band);
        let mut offered = false;
        for edge in &self.edges {
            for (y, t, sideless) in self.landings(edge, leg, corner, band)? {
                let s = leg.angle(y);
                ray_walk::apart(
                    "bool_torus_region_at",
                    Margin::of((leg.radius * s).abs()),
                    band,
                )
                .map_err(|f| f.map(RegionRefusal::Escalated))?;
                let s = turn(s);
                if let Some(corner) = corner {
                    match row("bool_torus_region_corner", leg.radius * (corner - s))? {
                        Sign::Positive => {}
                        Sign::Negative => continue,
                        Sign::Zero => return Err(RayFault::Graze),
                    }
                }
                let outward =
                    geom_brep::OutwardNormal::from_chart(self.chart_normal(y), self.sense);
                crossings.push(
                    offset + leg.radius * s,
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

    /// Where `leg` crosses `edge`: each crossing's point, the edge's
    /// parameter there, and whether it lies at an end of the edge
    /// (module docs, step 2).
    #[allow(clippy::type_complexity)] // the three readings of one crossing
    fn landings(
        &self,
        edge: &RegionEdge<T>,
        leg: &Leg<T>,
        corner: Option<T>,
        band: Band,
    ) -> Result<Vec<(Point3<T>, T, bool)>, RayFault<RegionRefusal>> {
        let row = |name, m: T| decide(name, Margin::of(m), band).map_err(in_band);
        // The span reading: `None` past an end, `Some(sideless)` within.
        let span = |t: T, lever: T| -> Result<Option<bool>, RayFault<RegionRefusal>> {
            let start = row("bool_torus_region_span", lever * (t - edge.t0))?;
            let end = row("bool_torus_region_span", lever * (edge.t1 - t))?;
            Ok((start != Sign::Negative && end != Sign::Negative)
                .then_some(start == Sign::Zero || end == Sign::Zero))
        };
        let mut out = Vec::new();
        match (edge.carrier, leg.kind) {
            (Carrier::Circle { center, radius, .. }, LegKind::Meridian { radial }) => {
                let thetas = match self.circle_roots(edge, leg, band) {
                    // The edge in the leg's plane: on the leg's own
                    // meridian, or on the opposite one, which the leg
                    // never meets.
                    Ok(Roots::On(_)) => {
                        match row(
                            "bool_torus_region_component",
                            (center - self.center).dot(radial),
                        )? {
                            Sign::Positive => self.along(edge, leg, corner, band)?,
                            Sign::Negative => {}
                            Sign::Zero => return Err(RayFault::Graze),
                        }
                        return Ok(out);
                    }
                    roots => found(roots)?,
                };
                for theta in thetas {
                    let Some(sideless) = span(theta, radius)? else {
                        continue;
                    };
                    let y = edge.at(theta);
                    // The leg's plane meets the torus in two meridians,
                    // `2(R − r)` apart.
                    match row("bool_torus_region_component", (y - self.center).dot(radial))? {
                        Sign::Positive => out.push((y, theta, sideless)),
                        Sign::Negative => {}
                        Sign::Zero => return Err(RayFault::Graze),
                    }
                }
            }
            (
                Carrier::Circle {
                    center,
                    axis,
                    radius,
                    u_ref,
                },
                LegKind::Parallel,
            ) => {
                let roots = match self.plane_roots(leg, axis, center, band) {
                    // A parallel at the leg's height: the leg's own circle,
                    // or the other parallel there, which the leg never
                    // meets. Within the band of each other the two are
                    // read as one, which can only set the path aside.
                    Ok(Roots::On(_)) => {
                        if row("bool_torus_region_component", radius - leg.radius)? == Sign::Zero {
                            self.along(edge, leg, corner, band)?;
                        }
                        return Ok(out);
                    }
                    roots => found(roots)?,
                };
                let v_ref = axis.cross(u_ref);
                let mid = (edge.t0 + edge.t1) * T::from_f64(0.5);
                // A meridian's or a Villarceau circle's plane meets the
                // torus in it and its mirror through the centre.
                let mirror = self.center + (self.center - center);
                for s in roots {
                    let y = leg.at(s);
                    let w = y - center;
                    let theta = (w.dot(v_ref).atan2(w.dot(u_ref)) - mid)
                        .reduce_periodic_centred(T::tau())
                        + mid;
                    let Some(sideless) = span(theta, radius)? else {
                        continue;
                    };
                    match row("bool_torus_region_component", (y - mirror).norm() - radius)? {
                        Sign::Positive | Sign::Negative => out.push((y, theta, sideless)),
                        // On the mirror, or where the two cross.
                        Sign::Zero => {
                            match row("bool_torus_region_component", w.norm() - radius)? {
                                Sign::Positive | Sign::Negative => {}
                                Sign::Zero => return Err(RayFault::Graze),
                            }
                        }
                    }
                }
            }
            (
                Carrier::Spiric {
                    center,
                    axis,
                    u_ref,
                    major,
                    minor,
                    offset,
                },
                _,
            ) => {
                let on_plane = center + u_ref * offset;
                for s in found(self.plane_roots(leg, u_ref, on_plane, band))? {
                    let y = leg.at(s);
                    let w = y - center;
                    // The oval this edge is an arc of lies on the `+m`
                    // side of its plane's trace; the other is
                    // `2√((R − r)² − o²)` across it.
                    match row("bool_torus_region_component", w.dot(axis.cross(u_ref)))? {
                        Sign::Positive => {}
                        Sign::Negative => continue,
                        Sign::Zero => return Err(RayFault::Graze),
                    }
                    let h = w.dot(axis);
                    let rho = (w - axis * h).norm();
                    let mid = (edge.t0 + edge.t1) * T::from_f64(0.5);
                    let v = (h.atan2(rho - major) - mid).reduce_periodic_centred(T::tau()) + mid;
                    if let Some(sideless) = span(v, minor)? {
                        out.push((y, v, sideless));
                    }
                }
            }
        }
        Ok(out)
    }

    /// An edge on the leg's own circle: the leg runs along it if the
    /// path reaches it, which grazes. A whole leg reaches every point of
    /// its circle; a first leg, those short of its corner. Neither end of
    /// the edge is the leg's start, which the boundary pre-pass placed off
    /// the edge.
    fn along(
        &self,
        edge: &RegionEdge<T>,
        leg: &Leg<T>,
        corner: Option<T>,
        band: Band,
    ) -> Result<(), RayFault<RegionRefusal>> {
        let Some(corner) = corner else {
            return Err(RayFault::Graze);
        };
        for t in [edge.t0, edge.t1] {
            let s = turn(leg.angle(edge.at(t)));
            match decide(
                "bool_torus_region_corner",
                Margin::of(leg.radius * (corner - s)),
                band,
            )
            .map_err(in_band)?
            {
                Sign::Negative => {}
                Sign::Positive | Sign::Zero => return Err(RayFault::Graze),
            }
        }
        Ok(())
    }

    /// The unit chart normal at `y`.
    fn chart_normal(&self, y: Point3<T>) -> Vec3<T> {
        let n = geom_brep::implicit_gradient(
            &geom::Surface::Torus {
                center: self.center,
                axis: self.axis,
                major_radius: self.major,
                minor_radius: self.minor,
                u_ref: self.u_ref,
            },
            y,
        );
        n / n.norm()
    }

    /// A circle edge's crossings with a meridian leg's plane, in the
    /// circle's own parameter (module docs, step 2).
    fn circle_roots(
        &self,
        edge: &RegionEdge<T>,
        leg: &Leg<T>,
        band: Band,
    ) -> Result<FirstHarmonicRoots<T>, RootRefusal> {
        let Carrier::Circle {
            center,
            axis,
            radius,
            u_ref,
        } = edge.carrier
        else {
            return Ok(Roots::Miss);
        };
        let g = leg.normal;
        let offset = center - leg.center;
        first_harmonic(
            g.dot(offset),
            radius * g.dot(u_ref),
            radius * g.dot(axis.cross(u_ref)),
            rounding_charge(offset.norm() + radius),
            radius,
            (edge.t0, edge.t1),
            band,
        )
    }

    /// `leg`'s crossings with the plane through `on` of unit normal `n`,
    /// in the leg's parameter over its whole turn (module docs, step 2).
    fn plane_roots(
        &self,
        leg: &Leg<T>,
        n: Vec3<T>,
        on: Point3<T>,
        band: Band,
    ) -> Result<FirstHarmonicRoots<T>, RootRefusal> {
        let offset = leg.center - on;
        first_harmonic(
            n.dot(offset),
            leg.radius * n.dot(leg.e1),
            leg.radius * n.dot(leg.e2),
            rounding_charge(offset.norm() + leg.radius),
            leg.radius,
            (T::zero(), T::tau()),
            band,
        )
    }
}

/// The certified roots of `d + cos_part·cos t + sin_part·sin t` over a
/// circle of `speed`, reported about the middle of `span`.
fn first_harmonic<T: Decide>(
    d: T,
    cos_part: T,
    sin_part: T,
    noise: T,
    speed: T,
    (t0, t1): (T, T),
    band: Band,
) -> Result<FirstHarmonicRoots<T>, RootRefusal> {
    let amplitude = (cos_part.powi(2) + sin_part.powi(2)).sqrt();
    first_harmonic_roots_in_band(
        &FirstHarmonic {
            lo: d - amplitude,
            hi: d + amplitude,
            cos_part,
            sin_part,
            lo_noise: noise,
            hi_noise: noise,
            phase_noise: T::zero(),
        },
        speed,
        t0,
        t1,
        &ROOT_ROWS,
        band,
    )
}

/// An angle in `(−π, π]` as a turn in `[0, 2π)`.
fn turn<T: Decide>(s: T) -> T {
    (T::zero() - s).select_le_zero(s, s + T::tau())
}

/// An in-band margin, setting the path aside.
fn in_band(diag: geom_core::Indeterminate) -> RayFault<RegionRefusal> {
    RayFault::InBand(RegionRefusal::Escalated(diag))
}

/// A root door's answer on one leg: its roots, none for a miss; an edge
/// in the leg's plane or a tangency grazes.
fn found<T: Decide>(
    roots: Result<FirstHarmonicRoots<T>, RootRefusal>,
) -> Result<Vec<T>, RayFault<RegionRefusal>> {
    match roots {
        Ok(Roots::Miss) => Ok(Vec::new()),
        Ok(Roots::Certified(found)) => Ok(found.roots().to_vec()),
        Ok(Roots::On(_)) | Err(RootRefusal::Unsettled(_)) => Err(RayFault::Graze),
        Err(RootRefusal::Escalated(diag)) => Err(in_band(diag)),
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use core::f64::consts::{FRAC_PI_2, PI, TAU};
    use slotmap::KeyData;

    const R: f64 = 2.0;
    const MINOR: f64 = 0.5;

    /// The parallel at minor angle `v` of the torus `(R, r)` about `z`,
    /// run with the major angle (`forward`) or against it.
    fn parallel(v: f64, forward: bool, key: u64) -> RegionEdge<f64> {
        RegionEdge {
            edge: EdgeKey::from(KeyData::from_ffi(key)),
            carrier: Carrier::Circle {
                center: Point3::new(0.0, 0.0, MINOR * v.sin()),
                axis: Vec3::new(0.0, 0.0, 1.0),
                radius: R + MINOR * v.cos(),
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            },
            t0: 0.0,
            t1: TAU,
            forward,
        }
    }

    /// The band of the torus between the minor angles `v0 < v1`, every
    /// major angle: its loops run CCW in the chart, so it lies to their
    /// left under the outward normal.
    fn band(v0: f64, v1: f64) -> TorusFaceRegion<f64> {
        TorusFaceRegion {
            center: Point3::origin(),
            axis: Vec3::new(0.0, 0.0, 1.0),
            major: R,
            minor: MINOR,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
            sense: true,
            outer: LoopKey::default(),
            edges: vec![parallel(v0, true, 1), parallel(v1, false, 2)],
        }
    }

    /// The point at chart `(u, v)`.
    fn at(u: f64, v: f64) -> Point3<f64> {
        let rho = R + MINOR * v.cos();
        Point3::new(rho * u.cos(), rho * u.sin(), MINOR * v.sin())
    }

    /// **A fillet's band reads across the tube's top circle.** The band
    /// from the top circle (`v = π/2`) to the inner equator (`v = π`), a
    /// fillet's shape: the top circle's plane is tangent to the torus
    /// along it, so no leg meets it at a simple root of that plane's
    /// residual, and a meridian leg reads it in the circle's own
    /// parameter. Every point of a lattice over the chart reads its side,
    /// on the top circle itself the boundary, and a micron either side of
    /// it its side.
    #[test]
    fn a_fillets_band_reads_across_the_top_circle() {
        let b = Band::new(1e-9, 1e-8).unwrap();
        let region = band(FRAC_PI_2, PI);
        for i in 0..12 {
            let u = f64::from(i) * TAU / 12.0 + 0.1;
            for j in 0..16 {
                let v = f64::from(j) * TAU / 16.0 + 0.05 - PI;
                let inside = (FRAC_PI_2..PI).contains(&v);
                assert_eq!(region.contains(at(u, v), b), Ok(Some(inside)), "({u}, {v})");
            }
            assert_eq!(
                region.contains(at(u, FRAC_PI_2), b),
                Ok(None),
                "u {u}: on the rim"
            );
            for (off, inside) in [(1e-6, true), (-1e-6, false)] {
                assert_eq!(
                    region.contains(at(u, FRAC_PI_2 + off), b),
                    Ok(Some(inside)),
                    "u {u}: {off:e} off the rim"
                );
            }
        }
    }
    /// R1 review probe: a strip between two Villarceau circles of one
    /// family, against a numeric oracle for the family parameter.
    #[test]
    fn r1_villarceau_strip_reads_consistently() {
        let b = Band::new(1e-9, 1e-8).unwrap();
        let sa = MINOR / R;
        let ca = (1.0 - sa * sa).sqrt();
        let rot = |phi: f64, v: Vec3<f64>| {
            let (s, c) = phi.sin_cos();
            Vec3::new(c * v.x - s * v.y, s * v.x + c * v.y, v.z)
        };
        let c0 = Vec3::new(0.0, MINOR, 0.0);
        let n0 = Vec3::new(-sa, 0.0, ca);
        let e0 = Vec3::new(ca, 0.0, sa);
        let circle = |phi: f64, forward: bool, key: u64| RegionEdge {
            edge: EdgeKey::from(KeyData::from_ffi(key)),
            carrier: Carrier::Circle {
                center: Point3::origin() + rot(phi, c0),
                axis: rot(phi, n0),
                radius: R,
                u_ref: rot(phi, e0),
            },
            t0: 0.0,
            t1: TAU,
            forward,
        };
        for phi1 in [0.7, 2.5, 4.0] {
            let region = TorusFaceRegion {
                center: Point3::origin(),
                axis: Vec3::new(0.0, 0.0, 1.0),
                major: R,
                minor: MINOR,
                u_ref: Vec3::new(1.0, 0.0, 0.0),
                sense: true,
                outer: LoopKey::default(),
                edges: vec![circle(0.0, true, 1), circle(phi1, false, 2)],
            };
            // the family parameter of p: the phi whose circle holds p
            let fam = |p: Point3<f64>| -> f64 {
                let w = p - Point3::origin();
                let g = |phi: f64| rot(phi, n0).dot(w);
                let on = |phi: f64| ((w - rot(phi, c0)).norm() - R).abs();
                let n = 20000;
                let mut best = (f64::INFINITY, 0.0);
                for i in 0..n {
                    let (a, bb) = (TAU * i as f64 / n as f64, TAU * (i + 1) as f64 / n as f64);
                    if g(a) * g(bb) <= 0.0 {
                        let (mut lo, mut hi) = (a, bb);
                        for _ in 0..60 {
                            let m = 0.5 * (lo + hi);
                            if g(lo) * g(m) <= 0.0 { hi = m } else { lo = m }
                        }
                        let r = 0.5 * (lo + hi);
                        if on(r) < best.0 {
                            best = (on(r), r);
                        }
                    }
                }
                assert!(best.0 < 1e-9, "oracle {best:?}");
                best.1
            };
            let (mut agree, mut disagree, mut refused) = (0, 0, 0);
            let mut x: u64 = 0x9e3779b97f4a7c15;
            let mut rnd = || {
                x ^= x << 13;
                x ^= x >> 7;
                x ^= x << 17;
                (x >> 11) as f64 / (1u64 << 53) as f64
            };
            for _ in 0..400 {
                let (u, v) = (rnd() * TAU, rnd() * TAU);
                let p = at(u, v);
                let f = fam(p);
                if f.min(TAU - f) < 1e-4 || (f - phi1).abs() < 1e-4 {
                    continue;
                }
                let inside = f < phi1;
                match region.contains(p, b) {
                    Ok(Some(got)) if got == inside => agree += 1,
                    Ok(Some(_)) => disagree += 1,
                    _ => refused += 1,
                }
            }
            eprintln!(
                "R1 villarceau phi1 {phi1}: agree {agree} disagree {disagree} refused {refused}"
            );
            assert!(agree == 0 || disagree == 0, "mixed: {agree} vs {disagree}");
        }
    }
    #[test]
    fn r1_villarceau_strip_near_its_edges() {
        let b = Band::new(1e-9, 1e-8).unwrap();
        for (big, small) in [(2.0_f64, 0.5_f64), (2.0, 1.9), (100.0, 1.0), (1.0, 0.05)] {
            let sa = small / big;
            let ca = (1.0 - sa * sa).sqrt();
            let rot = |phi: f64, v: Vec3<f64>| {
                let (s, c) = phi.sin_cos();
                Vec3::new(c * v.x - s * v.y, s * v.x + c * v.y, v.z)
            };
            let c0 = Vec3::new(0.0, small, 0.0);
            let n0 = Vec3::new(-sa, 0.0, ca);
            let e0 = Vec3::new(ca, 0.0, sa);
            let circle = |phi: f64, forward: bool, key: u64| RegionEdge {
                edge: EdgeKey::from(KeyData::from_ffi(key)),
                carrier: Carrier::Circle {
                    center: Point3::origin() + rot(phi, c0),
                    axis: rot(phi, n0),
                    radius: big,
                    u_ref: rot(phi, e0),
                },
                t0: 0.0,
                t1: TAU,
                forward,
            };
            let point = |phi: f64, t: f64| {
                let (s, c) = t.sin_cos();
                let y = n0.cross(e0);
                Point3::origin() + rot(phi, c0 + (e0 * c + y * s) * big)
            };
            for phi1 in [0.7, 4.0] {
                let region = TorusFaceRegion {
                    center: Point3::origin(),
                    axis: Vec3::new(0.0, 0.0, 1.0),
                    major: big,
                    minor: small,
                    u_ref: Vec3::new(1.0, 0.0, 0.0),
                    sense: true,
                    outer: LoopKey::default(),
                    edges: vec![circle(0.0, true, 1), circle(phi1, false, 2)],
                };
                let (mut a, mut d, mut r, mut wrong) = (0, 0, 0, vec![]);
                for k in 0..40 {
                    let t = 0.05 + TAU * f64::from(k) / 40.0;
                    for delta in [1e-7, 1e-5, 1e-3, 0.1] {
                        for phi in [delta, -delta, phi1 - delta, phi1 + delta, phi1 * 0.5] {
                            let p = point(phi, t);
                            let inside = phi > 0.0 && phi < phi1;
                            match region.contains(p, b) {
                                Ok(Some(got)) if got == inside => a += 1,
                                Ok(Some(got)) => {
                                    d += 1;
                                    wrong.push((phi, t, got));
                                }
                                _ => r += 1,
                            }
                        }
                    }
                }
                eprintln!(
                    "R1 vnear ({big},{small}) phi1 {phi1}: agree {a} disagree {d} refused {r} {:?}",
                    wrong.iter().take(3).collect::<Vec<_>>()
                );
                assert!(a == 0 || d == 0, "mixed");
            }
        }
    }
}
