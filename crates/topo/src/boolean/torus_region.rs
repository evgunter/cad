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
//!    ([`super::circle_roots::first_harmonic_roots_in_band`]): the leg's
//!    parameter against the edge's plane, except where that plane is
//!    tangent to the torus at the crossing (below), where a circle edge
//!    on a meridian leg is read in its own parameter against the leg's
//!    plane, as the sphere's arcs meet its rays. A plane meets the torus
//!    in two curves (two parallels, two meridians, two Villarceau circles,
//!    two ovals), so each root is then asked which one it lies on, and the
//!    edge's parameter there whether it lies in the edge's span.
//! 3. At the closest crossing along the path, the leg's heading against
//!    `N × τ` says whether the path leaves the face there (so `p` is in
//!    it) or enters it.
//!
//! **Where the plane is the circle's own.** A plane meets a curve on the
//! torus at a double root wherever the plane is tangent to the torus. A
//! parallel's plane is tangent along the tube's top and bottom circles,
//! which bound every fillet, so a parallel there (`r·cos v` not
//! definitely nonzero, `bool_torus_region_family`) is read in its own
//! parameter against a meridian leg's plane, which is normal to the torus
//! everywhere; so is every circle that is not a parallel. The cost is the
//! lever: a root in a circle's parameter is metered at the circle's
//! radius, up to `R + r`, where a root in a meridian leg's is metered at
//! `r`, so on a wide ring at a fine band a rim's crossing refuses
//! (`Unsettled::Meter`) where a leg's would not. A parallel leg meets a
//! parallel nowhere or runs along it. Elsewhere the decided plane is
//! transversal except at true grazes: a Villarceau circle's plane is
//! tangent at the two points where it meets its mirror, and a meridian
//! leg is tangent to an oval's plane where the leg touches the oval's
//! extreme in azimuth.
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
//! ([`ConicArc::hit`](crate::splitting::containment::ConicArc::hit) for a circle, [`PiecedArc::contact`] for an oval):
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
//! - `bool_torus_region_family`: whether a circle edge is a parallel (the
//!   sine between its axis and the torus's, at its radius) off the top
//!   and bottom circles (`ρ − R`): a choice between two sound readings,
//!   never a verdict.
//! - `bool_torus_region_component`: which of a plane's two curves a root
//!   lies on — a meridian plane's `(y − c)·ρ̂`, a parallel plane's
//!   `ρ − R`, a circle edge's distance from its mirror and then from
//!   itself, an oval's `(y − c)·m`; and
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
//!   meridian plane's two circles are `2(R − r)` apart, a parallel
//!   plane's `2|r·cos v|`, which `bool_torus_region_family` decided
//!   definite before the plane was read, an oval plane's
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
    FirstHarmonic, FirstHarmonicRoots, FirstHarmonicRows, first_harmonic_roots_in_band, near_span,
};
use super::region::{
    self, End, Fault, Landing, Legs, Paths, Region, RegionEdge, RegionRefusal, Round, Scaffolding,
    circle_edge_hit, found, in_band, span,
};
use super::solid_contain::PointInSolidError;
use crate::body::Body;
use crate::entity::{FaceKey, LoopKey};
use crate::ray_walk::RayFault;
use crate::splitting::containment::ConicRows;
use crate::splitting::pieced_arc::{ArcHit, ArcRefusal, ArcRows, PiecedArc};
use crate::splitting::spiric_arc::{Oval, SpiricArc};
use crate::splitting::{Uncrossable, UncrossableCarrier};
use crate::validate::decide;
use geom_brep::roots::{RootRefusal, Roots};

const ROOT_ROWS: FirstHarmonicRows = FirstHarmonicRows {
    noise: "bool_torus_region_roots_noise",
    coaxial: "bool_torus_region_roots_coaxial",
    extreme: "bool_torus_region_roots_extreme",
    root_slack: "bool_torus_region_roots_slack",
    decision: BooleanDecision::CONTAINMENT_UNNAMED,
};

/// The row a place is read against a first leg's corner on.
const CORNER: &str = "bool_torus_region_corner";

/// A circle edge's boundary reading's rows (module docs).
const BOUNDARY: ConicRows = ConicRows::bounded(
    "bool_torus_region_arc_span",
    "bool_torus_region_arc_straddle",
);

/// An oval edge's boundary reading's rows (module docs).
const SPIRIC: ArcRows = ArcRows {
    end: "bool_torus_region_spiric_end",
    clear: "bool_torus_region_spiric_clear",
    on: "bool_torus_region_spiric_on",
    leaf: "bool_torus_region_spiric_leaf",
};

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
    edges: Vec<Edge<T>>,
}

/// One boundary edge: a circle or an oval of a spiric.
type Edge<T> = RegionEdge<T>;

/// Which family a leg's circle belongs to.
#[derive(Clone, Copy)]
enum LegKind<T: geom_core::Real> {
    /// A parallel: its plane is normal to the axis.
    Parallel,
    /// A meridian: its plane holds the axis and meets the torus in two
    /// meridians; `radial` points from the axis to this one.
    Meridian { radial: Vec3<T> },
}

/// Which reading a circle edge takes on a meridian leg
/// (`bool_torus_region_family`).
#[derive(Clone, Copy)]
enum CircleFamily {
    /// A parallel whose plane is transversal to the torus (`r·cos v`
    /// definitely nonzero), read in the leg's parameter against its plane;
    /// `outer` is which of the plane's two parallels it is (`ρ > R`).
    Parallel { outer: bool },
    /// A parallel on the top or bottom circle, where its plane is tangent
    /// to the torus, or one whose `r·cos v` is in band: read in its own
    /// parameter.
    Rim,
    /// Any other circle (a meridian, a Villarceau circle), or one whose
    /// tilt from the axis is in band: read in its own parameter.
    Other,
}

/// A leg's course: a circle of the torus, of the family `kind`.
#[derive(Clone, Copy)]
pub(crate) struct TorusCircle<T: geom_core::Real> {
    kind: LegKind<T>,
    round: Round<T>,
}

impl<T: Decide> region::Course<T> for TorusCircle<T> {
    fn place(&self, y: Point3<T>, at: &'static str, band: Band) -> Result<Option<T>, Fault> {
        self.round.place(y, at, band)
    }

    fn reach(&self, y: Point3<T>, at: &'static str, band: Band) -> Result<Option<T>, Fault> {
        self.round.reach(y, at, band)
    }

    fn heading(&self, y: Point3<T>) -> Vec3<T> {
        self.round.heading(y)
    }
}

/// One leg of a path, round a circle of the torus.
type Leg<T> = region::Leg<T, TorusCircle<T>>;

/// `face`'s region on its torus, or `None` when an edge of it lies in no
/// plane — a spline — or is neither a circle nor a spiric. A circle or a
/// spiric is taken as a plane section of this torus, unchecked: every
/// such edge of a torus face is one at rest (tier 3 places each edge on
/// both its faces' carriers).
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
    let planar = [geom::CurveKind::Circle, geom::CurveKind::Spiric];
    let Some(edges) = region::boundary(body, face, Scaffolding::Skipped, &planar)? else {
        return Ok(None);
    };
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
    /// ([`crate::ray_walk::walk`]), or an arc that bounds nothing.
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
        region::walk(self, p, band).map(Some)
    }

    /// `a`'s distance from the axis and its height, and the unit radial
    /// direction.
    fn polar(&self, a: Point3<T>) -> (T, T, Vec3<T>) {
        region::polar(self.center, self.axis, a)
    }

    /// The parallel through `a`, run `forward` (the major angle
    /// increasing) or back.
    fn parallel(&self, a: Point3<T>, forward: bool) -> Leg<T> {
        Leg::whole(TorusCircle {
            kind: LegKind::Parallel,
            round: region::parallel(self.center, self.axis, a, forward),
        })
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
        Leg::whole(TorusCircle {
            kind: LegKind::Meridian { radial },
            round: Round {
                center,
                normal,
                radius: self.minor,
                e1,
                e2: normal.cross(e1),
            },
        })
    }

    /// Which reading a circle edge `(axis, radius)` takes on a meridian
    /// leg ([`CircleFamily`]). A margin in band of either question reads
    /// the circle in its own parameter, which serves every circle.
    fn circle_family(&self, axis: Vec3<T>, radius: T, band: Band) -> CircleFamily {
        let tilt = Margin::levered(axis.cross(self.axis).norm(), radius);
        if !matches!(
            decide("bool_torus_region_family", tilt, band),
            Ok(Sign::Zero)
        ) {
            return CircleFamily::Other;
        }
        match decide(
            "bool_torus_region_family",
            Margin::of(radius - self.major),
            band,
        ) {
            Ok(Sign::Positive) => CircleFamily::Parallel { outer: true },
            Ok(Sign::Negative) => CircleFamily::Parallel { outer: false },
            Ok(Sign::Zero) | Err(_) => CircleFamily::Rim,
        }
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
        edge: &Edge<T>,
        leg: &Leg<T>,
        band: Band,
    ) -> Result<FirstHarmonicRoots<T>, RootRefusal> {
        let geom::Curve3::Circle {
            center,
            axis,
            radius,
            u_ref,
        } = edge.carrier
        else {
            return Ok(Roots::Miss);
        };
        first_harmonic_roots_in_band(
            &FirstHarmonic::circle_off_plane(
                (center, axis, radius, u_ref),
                (leg.course.round.center, leg.course.round.normal),
            ),
            radius,
            edge.t0,
            edge.t1,
            &ROOT_ROWS,
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
        first_harmonic_roots_in_band(
            &FirstHarmonic::circle_off_plane(
                (
                    leg.course.round.center,
                    leg.course.round.normal,
                    leg.course.round.radius,
                    leg.course.round.e1,
                ),
                (on, n),
            ),
            leg.course.round.radius,
            T::zero(),
            T::tau(),
            &ROOT_ROWS,
            band,
        )
    }
}

impl<T: Decide> Region<T> for TorusFaceRegion<T> {
    fn edges(&self) -> &[Edge<T>] {
        &self.edges
    }

    fn outer(&self) -> LoopKey {
        self.outer
    }

    /// A circle on its own carrier, an oval by its contact
    /// ([`PiecedArc::contact`]).
    fn on_edge(&self, edge: &Edge<T>, p: Point3<T>, band: Band) -> Result<bool, RegionRefusal> {
        match edge.carrier {
            geom::Curve3::Circle { .. } => circle_edge_hit(
                &edge.carrier,
                edge.turn.as_ref(),
                BOUNDARY,
                p,
                self.outer,
                band,
            ),
            geom::Curve3::Spiric {
                center,
                axis,
                u_ref,
                major_radius: major,
                minor_radius: minor,
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
                    Ok(ArcHit::On(_) | ArcHit::End) => Ok(true),
                    Ok(ArcHit::Off) => Ok(false),
                    Err(ArcRefusal::Escalated(diag)) => Err(RegionRefusal::Escalated(diag)),
                    Err(ArcRefusal::Unsettled) => Err(RegionRefusal::Unreadable(Uncrossable {
                        r#loop: edge.r#loop,
                        edge: edge.edge,
                        carrier: UncrossableCarrier::Spiric,
                    })),
                }
            }
            _ => unreachable!("torus_face_region admits circles and spirics only"),
        }
    }
}

impl<T: Decide> Paths<T> for TorusFaceRegion<T> {
    /// `p`'s parallel, or its meridian.
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
            self.meridian(p, forward)
        })
    }

    /// A first leg from `p` to the corner at `target`'s major angle
    /// (`parallel_first`) or minor angle, then the other family's leg
    /// through the corner.
    fn aimed(
        &self,
        p: Point3<T>,
        target: Point3<T>,
        parallel_first: bool,
        first_forward: bool,
        then_forward: bool,
        _: Band,
    ) -> Result<(TorusCircle<T>, End<T>, impl FnOnce() -> Leg<T>), Fault> {
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
        let end = End {
            place: first.course.round.arc(corner),
            row: CORNER,
        };
        let then = move || {
            if parallel_first {
                self.meridian(corner, then_forward)
            } else {
                self.parallel(corner, then_forward)
            }
        };
        Ok((first.course, end, then))
    }
}

impl<T: Decide> Legs<T> for TorusFaceRegion<T> {
    type Course = TorusCircle<T>;

    const AT: &'static str = "bool_torus_region_at";

    const SIDE: (&'static str, &'static str) =
        ("bool_torus_region_order", "bool_torus_region_cross");

    fn lever(&self) -> T {
        self.minor
    }

    /// Where `leg` crosses `edge`: each crossing's point, the edge's
    /// parameter there, and whether it lies at an end of the edge
    /// (module docs, step 2).
    fn landings(&self, edge: &Edge<T>, leg: &Leg<T>, band: Band) -> Result<Vec<Landing<T>>, Fault> {
        let row = |name, m: T| decide(name, Margin::of(m), band).map_err(in_band);
        let span =
            |t: T, lever: T| span("bool_torus_region_span", t, (edge.t0, edge.t1), lever, band);
        let mut out = Vec::new();
        match (&edge.carrier, leg.course.kind) {
            (
                &geom::Curve3::Circle {
                    center,
                    axis,
                    radius,
                    u_ref,
                },
                LegKind::Meridian { .. },
            ) if let CircleFamily::Parallel { outer } = self.circle_family(axis, radius, band) => {
                // A parallel off the top and bottom circles: its plane meets
                // the meridian leg at simple roots, read in the leg's
                // parameter (speed `r`). The plane holds two parallels at
                // `ρ − R = ±r·cos v`, which the decided `outer` tells apart.
                for s in found(self.plane_roots(leg, axis, center, band))? {
                    let y = leg.course.round.at(s);
                    let (rho, _, _) = self.polar(y);
                    let side = row("bool_torus_region_component", rho - self.major)?;
                    match (side, outer) {
                        (Sign::Zero, _) => return Err(RayFault::Graze),
                        (Sign::Positive, true) | (Sign::Negative, false) => {}
                        _ => continue,
                    }
                    let w = y - center;
                    let theta = near_span(
                        w.dot(axis.cross(u_ref)).atan2(w.dot(u_ref)),
                        (edge.t0, edge.t1),
                    );
                    if let Some(sideless) = span(theta, radius)? {
                        out.push((y, theta, sideless));
                    }
                }
            }
            (&geom::Curve3::Circle { center, radius, .. }, LegKind::Meridian { radial }) => {
                let thetas = match self.circle_roots(edge, leg, band) {
                    // The edge in the leg's plane: on the leg's own
                    // meridian, or on the opposite one, which the leg
                    // never meets.
                    Ok(Roots::On(_)) => {
                        match row(
                            "bool_torus_region_component",
                            (center - self.center).dot(radial),
                        )? {
                            Sign::Positive => leg.along(edge, Self::AT, band)?,
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
                &geom::Curve3::Circle {
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
                        if row(
                            "bool_torus_region_component",
                            radius - leg.course.round.radius,
                        )? == Sign::Zero
                        {
                            leg.along(edge, Self::AT, band)?;
                        }
                        return Ok(out);
                    }
                    roots => found(roots)?,
                };
                // A meridian's or a Villarceau circle's plane meets the
                // torus in it and its mirror through the centre.
                let mirror = self.center + (self.center - center);
                for s in roots {
                    let y = leg.course.round.at(s);
                    let w = y - center;
                    let theta = near_span(
                        w.dot(axis.cross(u_ref)).atan2(w.dot(u_ref)),
                        (edge.t0, edge.t1),
                    );
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
                &geom::Curve3::Spiric {
                    center,
                    axis,
                    u_ref,
                    major_radius: major,
                    minor_radius: minor,
                    offset,
                },
                _,
            ) => {
                let on_plane = center + u_ref * offset;
                for s in found(self.plane_roots(leg, u_ref, on_plane, band))? {
                    let y = leg.course.round.at(s);
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
                    let v = near_span(h.atan2(rho - major), (edge.t0, edge.t1));
                    if let Some(sideless) = span(v, minor)? {
                        out.push((y, v, sideless));
                    }
                }
            }
            _ => unreachable!("torus_face_region admits circles and spirics only"),
        }
        Ok(out)
    }

    fn outward(&self, y: Point3<T>) -> Vec3<T> {
        geom_brep::OutwardNormal::from_chart(self.chart_normal(y), self.sense).vec()
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::entity::EdgeKey;
    use core::f64::consts::{FRAC_PI_2, PI, TAU};
    use slotmap::KeyData;

    const R: f64 = 2.0;
    const MINOR: f64 = 0.5;

    /// The parallel at minor angle `v` of the torus `(R, r)` about `z`,
    /// run with the major angle (`forward`) or against it.
    fn parallel(v: f64, forward: bool, key: u64) -> Edge<f64> {
        RegionEdge {
            edge: EdgeKey::from(KeyData::from_ffi(key)),
            r#loop: LoopKey::default(),
            carrier: geom::Curve3::Circle {
                center: Point3::new(0.0, 0.0, MINOR * v.sin()),
                axis: Vec3::new(0.0, 0.0, 1.0),
                radius: R + MINOR * v.cos(),
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            },
            t0: 0.0,
            t1: TAU,
            turn: crate::boolean::region::fixture_turn(0.0, TAU),
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
    /// The strip `0 < φ < φ₁` between two circles of one Villarceau
    /// family of the torus `(R, MINOR)`: the circle at family parameter
    /// `φ` is the one at `0` turned by `φ` about the axis.
    fn villarceau_strip(phi1: f64) -> TorusFaceRegion<f64> {
        let (c0, n0, e0) = villarceau_frame();
        let circle = |phi: f64, forward: bool, key: u64| RegionEdge {
            edge: EdgeKey::from(KeyData::from_ffi(key)),
            r#loop: LoopKey::default(),
            carrier: geom::Curve3::Circle {
                center: Point3::origin() + turned(phi, c0),
                axis: turned(phi, n0),
                radius: R,
                u_ref: turned(phi, e0),
            },
            t0: 0.0,
            t1: TAU,
            turn: crate::boolean::region::fixture_turn(0.0, TAU),
            forward,
        };
        TorusFaceRegion {
            center: Point3::origin(),
            axis: Vec3::new(0.0, 0.0, 1.0),
            major: R,
            minor: MINOR,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
            sense: true,
            outer: LoopKey::default(),
            edges: vec![circle(0.0, true, 1), circle(phi1, false, 2)],
        }
    }

    /// The family's circle at `φ = 0`: its centre, axis and `u_ref`.
    fn villarceau_frame() -> (Vec3<f64>, Vec3<f64>, Vec3<f64>) {
        let sa = MINOR / R;
        let ca = (1.0 - sa * sa).sqrt();
        (
            Vec3::new(0.0, MINOR, 0.0),
            Vec3::new(-sa, 0.0, ca),
            Vec3::new(ca, 0.0, sa),
        )
    }

    /// `v` turned by `phi` about `z`.
    fn turned(phi: f64, v: Vec3<f64>) -> Vec3<f64> {
        let (s, c) = phi.sin_cos();
        Vec3::new(c * v.x - s * v.y, s * v.x + c * v.y, v.z)
    }

    /// The family parameter of `p` on the torus: the `φ` whose circle
    /// holds it, in closed form. `p` lies in the plane of circle `φ`
    /// where `n(φ)·w = ca·w_z − sa·ρ·cos(φ − θ)` vanishes, at
    /// `φ = θ ± acos(ca·w_z / (sa·ρ))`. That cosine is at most one on
    /// the torus, and is one where a bitangent plane touches it: a
    /// double root, which no sign change brackets. Of the two planes
    /// through `p`, the one whose family circle holds `p` is kept. A
    /// rounding of the cosine near one moves the root by its square
    /// root, so the circle is asked to hold `p` within `10⁻⁶`, far
    /// inside the `10⁻⁴` the strip's readers keep off its edges.
    fn villarceau_family(p: Point3<f64>) -> f64 {
        let (c0, n0, _) = villarceau_frame();
        let w = p - Point3::origin();
        let (theta, rho) = (w.y.atan2(w.x), w.x.hypot(w.y));
        let half = (n0.z * w.z / (-n0.x * rho)).clamp(-1.0, 1.0).acos();
        let on = |phi: f64| ((w - turned(phi, c0)).norm() - R).abs();
        let best = [theta + half, theta - half]
            .map(|phi| (on(phi), phi.rem_euclid(TAU)))
            .into_iter()
            .min_by(|a, b| a.0.total_cmp(&b.0))
            .unwrap();
        assert!(best.0 < 1e-6, "no family circle holds {p:?}: {best:?}");
        best.1
    }

    /// **A strip between two Villarceau circles reads its side at random
    /// points** (a counterexample search: the seed varies and is logged,
    /// the count rides `CAD_FUZZ_EFFORT`). The circles of one family, at
    /// family parameters `0` and `φ₁`, bound the strip `0 < φ < φ₁`;
    /// each point's parameter is [`villarceau_family`]'s, and every
    /// answer is that membership.
    #[test]
    fn a_villarceau_strip_reads_its_side_at_random_points() {
        let b = Band::new(1e-9, 1e-8).unwrap();
        for phi1 in [0.7, 2.5, 4.0] {
            let region = villarceau_strip(phi1);
            let (mut agree, mut wrong) = (0, Vec::new());
            let mut rng = test_utils::fuzz::start("torus_region_villarceau");
            let mut rnd = || rng.unit();
            for _ in 0..test_utils::fuzz::scaled(60) {
                let (u, v) = (rnd() * TAU, rnd() * TAU);
                let p = at(u, v);
                let f = villarceau_family(p);
                if f.min(TAU - f) < 1e-4 || (f - phi1).abs() < 1e-4 {
                    continue;
                }
                let inside = f < phi1;
                match region.contains(p, b) {
                    Ok(Some(got)) if got == inside => agree += 1,
                    Ok(Some(got)) => wrong.push((u, v, got)),
                    _ => {}
                }
            }
            assert!(
                wrong.is_empty() && agree > 0,
                "φ₁ {phi1}: {agree} right, wrong at {wrong:?} ({})",
                test_utils::fuzz::replay()
            );
        }
    }

    /// **A point where a bitangent plane touches the torus reads the
    /// strip's side**: the chart point seed `0xaaece85fd941a95f` drew for
    /// [`a_villarceau_strip_reads_its_side_at_random_points`]. Its two
    /// plane roots lie `2·10⁻⁴` apart, inside one step of a sign scan;
    /// the family circle that holds it is the one at `φ ≈ 0.5332`,
    /// inside the strip to `0.7`.
    #[test]
    fn a_bitangent_point_reads_the_strips_side() {
        let p = at(3.674_920_436_474_422, 4.459_602_276_695_849);
        let f = villarceau_family(p);
        assert!((f - 0.533_2).abs() < 1e-4, "family parameter {f}");
        assert_eq!(
            villarceau_strip(0.7).contains(p, Band::new(1e-9, 1e-8).unwrap()),
            Ok(Some(true))
        );
    }
    /// **The Villarceau strip reads its side near its edges**: at points
    /// `10⁻⁷` to `0.1` rad of family parameter either side of each edge
    /// circle, and at the strip's middle, on four tori (the donut, a fat
    /// one, the wide ring and a thin tube), every answer is the side, and
    /// at least three in four of them answer (the thin tube reads four in
    /// five).
    #[test]
    fn a_villarceau_strip_reads_its_side_near_its_edges() {
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
                r#loop: LoopKey::default(),
                carrier: geom::Curve3::Circle {
                    center: Point3::origin() + rot(phi, c0),
                    axis: rot(phi, n0),
                    radius: big,
                    u_ref: rot(phi, e0),
                },
                t0: 0.0,
                t1: TAU,
                turn: crate::boolean::region::fixture_turn(0.0, TAU),
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
                let (mut a, mut r, mut wrong) = (0, 0, vec![]);
                for k in 0..40 {
                    let t = 0.05 + TAU * f64::from(k) / 40.0;
                    for delta in [1e-7, 1e-5, 1e-3, 0.1] {
                        for phi in [delta, -delta, phi1 - delta, phi1 + delta, phi1 * 0.5] {
                            let p = point(phi, t);
                            let inside = phi > 0.0 && phi < phi1;
                            match region.contains(p, b) {
                                Ok(Some(got)) if got == inside => a += 1,
                                Ok(Some(got)) => wrong.push((phi, t, got)),
                                _ => r += 1,
                            }
                        }
                    }
                }
                assert!(
                    wrong.is_empty() && a >= 3 * r,
                    "({big}, {small}), φ₁ {phi1}: {a} right, {r} refused, wrong at {wrong:?}"
                );
            }
        }
    }

    /// **The ring convention and the axis are decided where the region is
    /// read, not premised.** A spindle (`R < r`) refuses under
    /// `ring_torus_convention`, a tube that is not a length under
    /// `torus_tube_positive` before the ring is asked, and a point on the
    /// axis under `bool_torus_frame_radius` before a leg divides by its
    /// distance from it; the ring torus with the same band answers.
    #[test]
    fn the_region_decides_the_ring_and_the_axis() {
        let b = Band::new(1e-9, 1e-8).unwrap();
        let refusal = |major: f64, minor: f64, p: Point3<f64>| {
            let region = TorusFaceRegion {
                major,
                minor,
                ..band(0.3, 1.0)
            };
            match region.contains(p, b) {
                Err(RegionRefusal::Escalated(diag)) => diag.predicate,
                other => panic!("({major}, {minor}) at {p:?}: {other:?}"),
            }
        };
        let p = Point3::new(0.8, 0.0, 0.1);
        assert_eq!(refusal(0.2, 0.3, p), Some("ring_torus_convention"));
        assert_eq!(refusal(0.75, -0.3, p), Some("torus_tube_positive"));
        assert_eq!(
            refusal(R, MINOR, Point3::new(0.0, 0.0, 0.2)),
            Some("bool_torus_frame_radius")
        );
        assert!(
            band(0.3, 1.0).contains(at(0.4, 0.6), b).is_ok(),
            "a ring torus answers"
        );
    }

    /// A region on the torus `(R, r)` about `z` whose one edge is `edge`.
    fn holding(edge: Edge<f64>) -> TorusFaceRegion<f64> {
        TorusFaceRegion {
            edges: vec![edge],
            ..band(0.0, 1.0)
        }
    }

    /// The circle `center + radius(û cos θ + (axis × û) sin θ)` over
    /// `(t0, t1)`, run forward.
    fn arc(
        center: Point3<f64>,
        axis: Vec3<f64>,
        radius: f64,
        u_ref: Vec3<f64>,
        (t0, t1): (f64, f64),
    ) -> Edge<f64> {
        RegionEdge {
            edge: EdgeKey::from(KeyData::from_ffi(9)),
            r#loop: LoopKey::default(),
            carrier: geom::Curve3::Circle {
                center,
                axis,
                radius,
                u_ref,
            },
            t0,
            t1,
            turn: crate::boolean::region::fixture_turn(t0, t1),
            forward: true,
        }
    }

    /// The meridian at major angle `u`, minor angle over `vs`.
    fn meridian_arc(u: f64, vs: (f64, f64)) -> Edge<f64> {
        let radial = Vec3::new(u.cos(), u.sin(), 0.0);
        arc(
            Point3::origin() + radial * R,
            radial.cross(Vec3::new(0.0, 0.0, 1.0)),
            MINOR,
            radial,
            vs,
        )
    }

    /// **A meridian leg keeps the crossing on its own meridian.** A rim
    /// circle (the top circle, read in its own parameter) meets the
    /// meridian leg's plane at the leg's major angle and the opposite
    /// one; only the first is a crossing, and it lands where the leg is.
    #[test]
    fn a_meridian_leg_keeps_its_own_meridians_crossing() {
        let b = Band::new(1e-9, 1e-8).unwrap();
        let top = parallel(FRAC_PI_2, true, 1);
        let region = holding(top.clone());
        for u in [0.3, 2.0, 4.4] {
            let leg = region.meridian(at(u, 2.0), true);
            let got = region.landings(&top, &leg, b).unwrap();
            assert_eq!(got.len(), 1, "u {u}: {got:?}");
            assert!(
                (got[0].0 - at(u, FRAC_PI_2)).norm() < 1e-12,
                "u {u}: {got:?}"
            );
        }
    }

    /// **A parallel leg keeps the crossing on the edge's own meridian.** A
    /// meridian arc's plane holds the axis, so a parallel meets it at the
    /// arc's major angle and the opposite one, which lies on the mirror
    /// meridian and is no crossing of the arc.
    #[test]
    fn a_parallel_leg_keeps_the_edges_own_meridian() {
        let b = Band::new(1e-9, 1e-8).unwrap();
        let side = meridian_arc(1.0, (-1.2, 1.2));
        let region = holding(side.clone());
        for v in [-0.9, 0.3, 1.0] {
            let leg = region.parallel(at(0.2, v), true);
            let got = region.landings(&side, &leg, b).unwrap();
            assert_eq!(got.len(), 1, "v {v}: {got:?}");
            assert!((got[0].0 - at(1.0, v)).norm() < 1e-12, "v {v}: {got:?}");
        }
    }

    /// **A leg keeps the crossing on the edge's own oval.** The plane
    /// `x = 1` cuts the torus in two ovals, `y > 0` and `y < 0`; an arc of
    /// the first is crossed by the outer equator once, at `y > 0`.
    #[test]
    fn a_leg_keeps_the_crossing_on_the_edges_own_oval() {
        let b = Band::new(1e-9, 1e-8).unwrap();
        let oval = RegionEdge {
            edge: EdgeKey::from(KeyData::from_ffi(9)),
            r#loop: LoopKey::default(),
            carrier: geom::Curve3::Spiric {
                center: Point3::origin(),
                axis: Vec3::new(0.0, 0.0, 1.0),
                u_ref: Vec3::new(1.0, 0.0, 0.0),
                major_radius: R,
                minor_radius: MINOR,
                offset: 1.0,
            },
            t0: -2.5,
            t1: 2.5,
            turn: crate::boolean::region::fixture_turn(-2.5, 2.5),
            forward: true,
        };
        let region = holding(oval.clone());
        for p in [at(0.2, 0.0), at(2.0, 0.4), at(3.5, -0.3)] {
            let leg = region.parallel(p, true);
            let got = region.landings(&oval, &leg, b).unwrap();
            assert_eq!(got.len(), 1, "{p:?}: {got:?}");
            assert!(
                got[0].0.y > 0.0 && (got[0].0.x - 1.0).abs() < 1e-12,
                "{p:?}: {got:?}"
            );
        }
    }

    /// **A crossing at an edge's end reads no side there.** A meridian leg
    /// through the start of a parallel arc meets it at its end: the
    /// landing is sideless, so a path whose closest crossing it is grazes
    /// instead of reading the side of the arc's interior.
    #[test]
    fn a_crossing_at_an_edges_end_is_sideless() {
        let b = Band::new(1e-9, 1e-8).unwrap();
        let rim = arc(
            Point3::new(0.0, 0.0, MINOR * 0.5f64.sin()),
            Vec3::new(0.0, 0.0, 1.0),
            R + MINOR * 0.5f64.cos(),
            Vec3::new(1.0, 0.0, 0.0),
            (0.0, 1.0),
        );
        let region = holding(rim.clone());
        let leg = region.meridian(at(0.0, 1.4), false);
        let got = region.landings(&rim, &leg, b).unwrap();
        assert_eq!(got.len(), 1, "{got:?}");
        assert!(got[0].2, "the arc's end is sideless: {got:?}");
        let inner = region.meridian(at(0.5, 1.4), false);
        let got = region.landings(&rim, &inner, b).unwrap();
        assert!(
            got.len() == 1 && !got[0].2,
            "the arc's interior reads its side: {got:?}"
        );
        // A path whose only crossing is that end grazes.
        assert!(matches!(
            region::read_path(
                &region,
                at(0.0, 1.4),
                region::Path::Single {
                    parallel: false,
                    forward: false
                },
                b
            ),
            Err(RayFault::Graze)
        ));
    }

    /// **A point near an edge is read by its distance from it, before
    /// any path.** Round the fillet band's rim, a point within the band's
    /// zero of it is on the boundary and one in the band refuses, whatever
    /// a path from it would read.
    #[test]
    fn a_point_near_an_edge_is_read_before_any_path() {
        let b = Band::new(1e-9, 1e-8).unwrap();
        let region = band(FRAC_PI_2, PI);
        for i in 0..24 {
            let u = f64::from(i) * TAU / 24.0 + 0.01;
            for off in [-1.6e-9, -0.8e-9, 0.8e-9, 1.6e-9] {
                assert_eq!(
                    region.contains(at(u, FRAC_PI_2 + off), b),
                    Ok(None),
                    "u {u}: {off:e} off the rim"
                );
            }
            for off in [-8e-9, 6e-9] {
                assert!(
                    matches!(
                        region.contains(at(u, FRAC_PI_2 + off), b),
                        Err(RegionRefusal::Escalated(diag))
                            if diag.predicate == Some("curve_carrier_gap")
                    ),
                    "u {u}: {off:e} off the rim"
                );
            }
        }
    }

    /// **Null scaffolding is stepped over**: a wall with a null edge
    /// fanned into its outer loop has the same region as the wall
    /// without it, where an uncertified edge between two points has
    /// none.
    #[test]
    fn a_walls_null_edge_bounds_nothing() {
        let wall = || {
            crate::boolean::boxes::tests::torus_wall(
                Point3::origin(),
                Vec3::unit_z(),
                Vec3::unit_x(),
                R,
                MINOR,
                (0.3, 1.9),
                (-0.7, 0.8),
            )
        };
        let (body, face) = wall();
        let (mut nulled, nulled_face) = wall();
        super::region::with_null_edge(&mut nulled, nulled_face);
        let plain = torus_face_region(&body, face).unwrap().unwrap();
        // The null edge stepped over, not refused.
        let held = torus_face_region(&nulled, nulled_face).unwrap().unwrap();
        assert_eq!(held.edges.len(), plain.edges.len(), "the same boundary");
        let b = Band::new(1e-9, 1e-8).unwrap();
        for (u, v) in [(1.0, 0.0), (1.0, 1.5), (2.5, 0.0)] {
            let got = held.contains(at(u, v), b);
            assert!(
                matches!(got, Ok(Some(_))),
                "a definite reading at ({u}, {v})"
            );
            assert_eq!(got, plain.contains(at(u, v), b), "at ({u}, {v})");
        }
    }
}
