//! **The §2c fused fillet family** — the surface half of the sealed
//! kernel (`verbs.rs`): `fillet_arc`, `arc_fillet`, `arc_fillet_arc`,
//! the arc-arrival builders, and the endpoint-free `arc_to(spec)` legs.
//!
//! Every verb here consumes only its incoming state's binding bits plus
//! its own authored arguments (the §2c axiom); the `Decide + Bounds`
//! obligation sits on the arc-involving verbs alone, which capture the
//! resolution machinery as a plain fn pointer
//! ([`verbs::ArcResolver`]) so the generic binders that later complete
//! an arrival never carry the bound.
//!
//! Admissibility is the STATE-KEYED trait matrix (§2c rounds 6–9): one
//! impl per admissible (state, mode) pair; an inadmissible pair is a
//! missing impl — unrepresentable, not refused. The full matrix:
//!
//! | mode | leg (`arc_to`) | fused incoming | arrival |
//! |---|---|---|---|
//! | `Bulge{p,b}` | Point | Point | — (no chord) |
//! | `Via{q,p}` | Point | Point | Directed anchor (director pending) |
//! | `Center{c,w,p}` | Point | Entry, Point | complete (resolves at the verb; `p: Start` closes) |
//! | `Radius{r,side}` | — | OnArc (centre DERIVED from the tip's bits — the sole OnArc mode; `Center@OnArc` is excluded by the `Center@Directed` value-match doctrine) | Directed anchor (binders pending) |
//! | `Sweep{r,side,angle}` | Directed | Directed | — |
//! | `ArcLen{r,side,len}` | Directed | Directed | — |
//!
//! # Examples (the §2c design conversation's own chains)
//!
//! **The fused entry, and the arrival that closes.** A lens: the entry
//! side rides one circle, one fillet rounds the tip, and the arrival
//! rides the other circle back to the entry — ONE authoring act,
//! because an arc and the fillet that trims it are one decision.
//!
//! ```
//! use geom_core::Point2;
//! use profile::{ArcSweep, Center, Open, Start};
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let p = Point2::new;
//! let tip = 0.75_f64.sqrt();
//! let lens = Open.arc_fillet_arc(
//!     Center { c: p(-0.5, 0.0), winding: ArcSweep::Ccw, p: p(0.0, -tip) },
//!     0.25,
//!     Center { c: p(0.5, 0.0), winding: ArcSweep::Ccw, p: Start },
//! )?;
//! assert_eq!(lens.program.len(), 1);
//! # Ok(())
//! # }
//! ```
//!
//! **Line incoming, ARC arrival.** A quarter disc: a straight side, a
//! fillet, and the carrier that closes it. The arrival's `Center` mode
//! resolves at the verb, so `p: Start` closes there and then.
//!
//! ```
//! use geom_core::Point2;
//! use profile::{ArcSweep, Center, Open, Start};
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let p = Point2::new;
//! let quarter = Open.at(p(0.0, 2.0))
//!     .line_to(p(0.0, 0.0))?
//!     .toward(1.0_f64, 0.0)?
//!     .fillet_arc(0.5, Center { c: p(0.0, 0.0), winding: ArcSweep::Ccw, p: Start })?;
//! assert_eq!(quarter.loop_.vertices.len(), 4);
//! # Ok(())
//! # }
//! ```
//!
//! **The on-carrier tip re-authors its carrier.** An interior `Center`
//! arrival leaves the tip ON that carrier, carrying position and
//! tangent and nothing else — so the verb that continues it AUTHORS
//! the carrier again, and `Radius { r, side }` is the one mode that
//! can: its centre is DERIVED from those two bits, so tangency there
//! holds by construction and nothing is value-matched.
//!
//! ```
//! use geom_core::Point2;
//! use profile::{ArcSide, ArcSweep, Center, Open, Radius, Start};
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let p = Point2::new;
//! let boss = Open.at(p(5.05, -1.6))
//!     .toward(2.1_f64, 0.8)?
//!     // Onto the boss circle, blended.
//!     .fillet_arc(0.5, Center { c: p(7.0, 0.0), winding: ArcSweep::Ccw, p: p(8.5, 0.0) })?
//!     // Off it again: r = 1.5 and Left re-derive the centre (7, 0)
//!     // from the tip's own position and tangent, exactly.
//!     .arc_fillet(Radius { r: 1.5, side: ArcSide::Left }, 0.5)?
//!     .at(p(4.05, 1.35))?
//!     .toward(-4.1, 0.3)?
//!     .line(1.0)?
//!     .line_to(Start)?;
//! assert!(boss.loop_.tangent_joints.len() >= 4);
//! # Ok(())
//! # }
//! ```
//!
//! **Ray extension, and the endpoint-free legs.** A bare `fillet(r)`
//! knows only the tangent ray its directed point defines, so after ANY
//! leg its incoming side IS that ray — here off a sharp `Sweep` arc
//! leg, whose endpoint the spec DERIVES rather than authors.
//!
//! ```
//! use core::f64::consts::FRAC_PI_2;
//! use geom_core::Point2;
//! use profile::{ArcSide, Open, Start, Sweep};
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let p = Point2::new;
//! let hook = Open.at(p(0.0, 0.0))
//!     .toward(1.0_f64, 0.0)?
//!     .arc_to(Sweep { r: 1.0, side: ArcSide::Left, angle: FRAC_PI_2 })?
//!     .fillet(0.25)?
//!     .at(p(0.0, 3.0))?
//!     .toward(-1.0, 0.0)?
//!     .line(1.0)?
//!     .line_to(Start)?;
//! assert_eq!(hook.loop_.vertices.len(), 5);
//! # Ok(())
//! # }
//! ```
//!
//! Every row is spelled by ONE verb name per site: `arc_to(spec)` is
//! the sharp arc leg from both the Point tip (`Bulge`/`Via`/`Center`,
//! [`PointLeg`]) and the Directed tip (`Sweep`/`ArcLen`,
//! [`TangentIncoming`]); the fused verbs take their incoming mode the
//! same way. There are no retired-name doors left.

use geom_core::{Point2, Real, Sign, Tolerance};

use super::arc_fillet::{self, ArcCarrierScalar, carrier_tangent};
use super::program::{ArcData, ClosedLoop, Step, Target};
use super::verbs::{self, ArcLen, Center, DirectedPoint, PendingArc, Radius, Sweep, Via};
use super::{
    ArcData as SegArc, Core, Dir, FirstSeg, Flavor, HasAng, HasPos, Incoming, NoAng, NoPos, Open,
    PartialPath, PathError, PendingMeta, Start, Tip, WithIncoming, in_state, junction_check,
    leg_end_tip, linear_band,
};

// ------------------------------------------------------------------
// The OnArc state.
// ------------------------------------------------------------------

/// The tip left by an INTERIOR arc arrival: a directed point whose side
/// runs on an arc carrier that is not yet ended. Its binding bits are
/// position + tangent, nothing else (§2c axiom) — which is exactly why
/// its only continuations are the fused verbs that AUTHOR their
/// incoming carrier (`Radius` re-derives it from these bits; `Center`
/// re-states it): the carrier run into the next trim is emitted by that
/// verb, from the chain's head, along the carrier its spec names.
#[derive(Clone, Debug)]
pub struct OnArc<T: Real> {
    pub(super) core: Core<T>,
    pub(super) at: Point2<T>,
    pub(super) dir: Dir<T>,
}

impl<T: Real> OnArc<T> {
    fn dp(&self) -> DirectedPoint<T> {
        DirectedPoint {
            at: self.at,
            dir: self.dir,
        }
    }
}

// ------------------------------------------------------------------
// Shared plumbing: `pub(super)` because the fused verbs here and the
// chain doors in `path.rs` drive the ONE construction.
// ------------------------------------------------------------------

/// Opens a fillet with a RAY incoming from a directed pose (no step
/// recorded — callers record their own verb).
pub(super) fn open_ray<T: geom_core::Decide>(
    core: &mut Core<T>,
    at: Point2<T>,
    dir: Dir<T>,
    radius: T,
    by_tangent: bool,
    origin_incoming: Option<Incoming<T>>,
) -> Result<(), PathError<T>> {
    let band = linear_band()?;
    verbs::gate_positive("path_fillet_radius", radius, band, |r| {
        PathError::NonpositiveFilletRadius { radius: r }
    })?;
    core.pending = Some(verbs::Pending::Ray(verbs::PendingRay {
        origin: at,
        dir,
        radius,
    }));
    core.pending_meta = Some(PendingMeta {
        by_tangent,
        origin_incoming,
    });
    Ok(())
}

/// Opens a fillet with an AUTHORED-ARC incoming (no step recorded).
pub(super) fn open_arc<T: ArcCarrierScalar>(
    core: &mut Core<T>,
    arc: PendingArc<T>,
) -> Result<(), PathError<T>> {
    let band = linear_band()?;
    verbs::gate_positive("path_fillet_radius", arc.radius, band, |r| {
        PathError::NonpositiveFilletRadius { radius: r }
    })?;
    core.pending = Some(verbs::Pending::Arc(arc));
    core.pending_meta = Some(PendingMeta {
        by_tangent: false,
        origin_incoming: None,
    });
    Ok(())
}

/// Resolves the open fillet against an ARC ARRIVAL about `centre`,
/// anchored at `anchor` — the interior form: the tip continues ON the
/// arrival carrier at the anchor (the run into the next trim is the
/// NEXT verb's emission). Returns the pieces so each door shapes its
/// own tip type.
pub(super) fn resolve_arc_arrival<T: geom_core::Decide>(
    core: &mut Core<T>,
    resolver: verbs::ArcResolver<T>,
    anchor: Point2<T>,
    centre: Point2<T>,
    winding: crate::sugar::ArcSweep,
) -> Result<Dir<T>, PathError<T>> {
    let band = linear_band()?;
    let dir = carrier_tangent(anchor, centre, winding, band)?;
    let (pending, meta) =
        core.take_pending("arc-carrier fillet arrival without an opened fillet")?;
    let merge = matches!(&pending, verbs::Pending::Ray(_))
        && meta.by_tangent
        && meta
            .origin_incoming
            .as_ref()
            .is_some_and(|i| i.carrier.is_none());
    let trims = resolver(
        pending.side(),
        arc_fillet::FilletSide {
            anchor,
            carrier: arc_fillet::SideCarrier::Circle { centre, winding },
        },
        pending.radius(),
        Tolerance::get(),
    )?;
    core.emit_fillet_in(&trims, merge)?;
    // The continuation rides the arrival carrier from `t2` by
    // construction, so the joint there IS a constructed tangency.
    core.emit_fillet_arc(&trims, true)?;
    Ok(dir)
}

/// Resolves the open fillet against the ARC ARRIVAL that CLOSES at the
/// entry (`p: Start`): the
/// entry vertex is KEPT as a genuine two-carrier junction, the arrival
/// carrier run (or the fillet arc itself, on an exact fit) becomes the
/// closing segment, and the seam junction check runs with both
/// directions known.
pub(super) fn resolve_arc_close<T: geom_core::Decide>(
    core: &mut Core<T>,
    resolver: verbs::ArcResolver<T>,
    centre: Point2<T>,
    winding: crate::sugar::ArcSweep,
) -> Result<ClosedLoop<T>, PathError<T>> {
    let start_pos = core.start_pos.ok_or(PathError::UnderdeterminedLeg {
        site: "close before the entry position is bound",
    })?;
    let start_ang = core.start_ang.ok_or(PathError::UnderdeterminedLeg {
        site: "close before the entry direction is bound",
    })?;
    let band = linear_band()?;
    // The arrival's END tangent is the carrier's tangent at the entry
    // point — the incoming half of the seam junction.
    let end_ang = carrier_tangent(start_pos, centre, winding, band)?;
    let (pending, meta) = core.take_pending("arc-carrier close without an opened fillet")?;
    let merge = matches!(&pending, verbs::Pending::Ray(_))
        && meta.by_tangent
        && meta
            .origin_incoming
            .as_ref()
            .is_some_and(|i| i.carrier.is_none());
    let trims = resolver(
        pending.side(),
        arc_fillet::FilletSide {
            anchor: start_pos,
            carrier: arc_fillet::SideCarrier::Circle { centre, winding },
        },
        pending.radius(),
        Tolerance::get(),
    )?;
    core.emit_fillet_in(&trims, merge)?;
    let radius = (start_pos - centre).norm_squared().sqrt();
    if trims.fit_out == Sign::Positive {
        // The arrival still has carrier run left: the fillet arc is an
        // interior segment and the run itself closes the loop.
        core.emit_fillet_arc(&trims, true)?;
        let head = core.head()?;
        let bulge = crate::sugar::bulge_from_center(head, start_pos, centre, winding);
        core.set_leaving(bulge, FirstSeg::Arc)?;
        let chord = (start_pos - head).norm_squared().sqrt();
        junction_check(
            &Incoming {
                ang: end_ang,
                arm: radius.min(chord),
                carrier: Some(SegArc {
                    center: centre,
                    radius,
                }),
            },
            start_ang,
            true,
        )?;
    } else {
        // Exact fit: the FILLET ARC is the whole arrival side and
        // closes the loop; the authored anchor is absorbed into the
        // tangent point the fit gate classified as coincident with it.
        core.set_leaving(trims.bulge, FirstSeg::Arc)?;
        junction_check(
            &Incoming {
                ang: end_ang,
                arm: trims.arc.radius,
                carrier: Some(trims.arc),
            },
            start_ang,
            true,
        )?;
    }
    Ok(core.clone().build())
}

// ------------------------------------------------------------------
// The arrival matrix: spec types applied to an OPEN fillet.
// ------------------------------------------------------------------

/// The ARRIVAL half of `fillet_arc` / `arc_fillet_arc`: what the
/// arc-arrival spec resolves to. One impl per admissible arrival mode;
/// the `Out` type is the mode's own completion story (resolved tip,
/// closed loop, or a builder awaiting the binders the mode leaves
/// free). Sealed by the crate-private argument types.
pub trait ArrivalSpec<T: ArcCarrierScalar> {
    /// The state the arrival leaves the chain in.
    type Out;
    #[doc(hidden)]
    fn apply(core: Core<T>, spec: Self) -> Self::Out;
    #[doc(hidden)]
    fn fail(err: PathError<T>) -> Self::Out;
    #[doc(hidden)]
    fn to_wire(&self) -> ArcData<T>;
}

/// `Center { c, winding, p }` with an INTERIOR anchor: complete at the
/// verb (the anchor and the derived direction are one authored act —
/// the arrival's own carrier). The tip continues ON the
/// carrier at `p`.
impl<T: ArcCarrierScalar> ArrivalSpec<T> for Center<T, Point2<T>> {
    type Out = Result<OnArc<T>, PathError<T>>;
    fn apply(mut core: Core<T>, spec: Self) -> Self::Out {
        let dir = resolve_arc_arrival(
            &mut core,
            arc_fillet::resolve::<T>,
            spec.p,
            spec.c,
            spec.winding,
        )?;
        Ok(OnArc {
            core,
            at: spec.p,
            dir,
        })
    }
    fn fail(err: PathError<T>) -> Self::Out {
        Err(err)
    }
    fn to_wire(&self) -> ArcData<T> {
        ArcData::Center {
            c: self.c,
            winding: self.winding,
            target: Target::Point(self.p),
        }
    }
}

/// `Center { c, winding, p: Start }`: the arc-arrival CLOSE (the
/// closing arrival): the entry vertex is KEPT as a genuine
/// two-carrier junction and the seam junction check runs there.
impl<T: ArcCarrierScalar> ArrivalSpec<T> for Center<T, Start> {
    type Out = Result<ClosedLoop<T>, PathError<T>>;
    fn apply(mut core: Core<T>, spec: Self) -> Self::Out {
        let Start = spec.p;
        resolve_arc_close(&mut core, arc_fillet::resolve::<T>, spec.c, spec.winding)
    }
    fn fail(err: PathError<T>) -> Self::Out {
        Err(err)
    }
    fn to_wire(&self) -> ArcData<T> {
        ArcData::Center {
            c: self.c,
            winding: self.winding,
            target: Target::Start,
        }
    }
}

/// `Radius { r, side }`: the centre is DERIVED from the arrival's own
/// directed anchor, so the mode leaves BOTH binders free — the builder
/// awaits `.at(p)` and a director, in either order.
impl<T: ArcCarrierScalar> ArrivalSpec<T> for Radius<T> {
    type Out = Result<RadiusArrival<T>, PathError<T>>;
    fn apply(core: Core<T>, spec: Self) -> Self::Out {
        Ok(RadiusArrival {
            core,
            spec,
            resolver: arc_fillet::resolve::<T>,
        })
    }
    fn fail(err: PathError<T>) -> Self::Out {
        Err(err)
    }
    fn to_wire(&self) -> ArcData<T> {
        ArcData::Radius {
            r: self.r,
            side: self.side,
        }
    }
}

/// `Via { q, p }` with an interior anchor: `q` completes the arrival's
/// DIRECTED anchor, so the anchor is the spec's own `p` and only the
/// director is left free.
impl<T: ArcCarrierScalar> ArrivalSpec<T> for Via<T, Point2<T>> {
    type Out = Result<ViaArrival<T>, PathError<T>>;
    fn apply(core: Core<T>, spec: Self) -> Self::Out {
        Ok(ViaArrival {
            core,
            q: spec.q,
            p: spec.p,
            resolver: arc_fillet::resolve::<T>,
        })
    }
    fn fail(err: PathError<T>) -> Self::Out {
        Err(err)
    }
    fn to_wire(&self) -> ArcData<T> {
        ArcData::Via {
            q: self.q,
            target: Target::Point(self.p),
        }
    }
}

/// `Via { q, p: Start }`: the via-completed CLOSE — anchor at the
/// entry, director pending, `q` picks the carrier.
impl<T: ArcCarrierScalar> ArrivalSpec<T> for Via<T, Start> {
    type Out = Result<ViaArrivalStart<T>, PathError<T>>;
    fn apply(core: Core<T>, spec: Self) -> Self::Out {
        Ok(ViaArrivalStart {
            core,
            q: spec.q,
            resolver: arc_fillet::resolve::<T>,
        })
    }
    fn fail(err: PathError<T>) -> Self::Out {
        Err(err)
    }
    fn to_wire(&self) -> ArcData<T> {
        ArcData::Via {
            q: self.q,
            target: Target::Start,
        }
    }
}

// ------------------------------------------------------------------
// Arrival builders (the binder halves the spec left free).
// ------------------------------------------------------------------

/// A `Radius` arrival awaiting both binders (either order).
#[derive(Clone, Debug)]
pub struct RadiusArrival<T: Real> {
    core: Core<T>,
    spec: Radius<T>,
    resolver: verbs::ArcResolver<T>,
}

/// A `Radius` arrival with its anchor bound, director pending.
#[derive(Clone, Debug)]
pub struct RadiusArrivalAt<T: Real> {
    core: Core<T>,
    spec: Radius<T>,
    at: Point2<T>,
    resolver: verbs::ArcResolver<T>,
}

/// A `Radius` arrival with its director bound, anchor pending.
#[derive(Clone, Debug)]
pub struct RadiusArrivalDir<T: Real> {
    core: Core<T>,
    spec: Radius<T>,
    dir: Dir<T>,
    resolver: verbs::ArcResolver<T>,
}

/// Completes a Radius arrival: derive the centre from the directed
/// anchor + the spec, then resolve exactly as the Center form does.
fn radius_complete<T: geom_core::Decide>(
    mut core: Core<T>,
    spec: Radius<T>,
    at: Point2<T>,
    dir: Dir<T>,
    resolver: verbs::ArcResolver<T>,
) -> Result<OnArc<T>, PathError<T>> {
    let band = linear_band()?;
    let (centre, winding) = verbs::radius_carrier(DirectedPoint { at, dir }, spec, band)?;
    let dir = resolve_arc_arrival(&mut core, resolver, at, centre, winding)?;
    Ok(OnArc { core, at, dir })
}

impl<T: geom_core::Decide> RadiusArrival<T> {
    /// Binds the arrival's anchor — a real on-path point on the
    /// derived carrier.
    pub fn at(mut self, p: Point2<T>) -> RadiusArrivalAt<T> {
        self.core.record(Step::At(p));
        RadiusArrivalAt {
            core: self.core,
            spec: self.spec,
            at: p,
            resolver: self.resolver,
        }
    }

    /// Binds the arrival direction (angle-first order).
    pub fn angle(mut self, theta: T) -> RadiusArrivalDir<T> {
        self.core.record(Step::Angle(theta));
        RadiusArrivalDir {
            core: self.core,
            spec: self.spec,
            dir: Dir::from_angle(theta),
            resolver: self.resolver,
        }
    }

    /// Binds the arrival direction as exact components.
    pub fn toward(mut self, dx: T, dy: T) -> Result<RadiusArrivalDir<T>, PathError<T>> {
        self.core.record(Step::Toward { dx, dy });
        let dir = verbs::director(dx, dy)?;
        Ok(RadiusArrivalDir {
            core: self.core,
            spec: self.spec,
            dir,
            resolver: self.resolver,
        })
    }
}

impl<T: geom_core::Decide> RadiusArrivalAt<T> {
    /// Completes the arrival with its direction; the fillet resolves.
    pub fn angle(mut self, theta: T) -> Result<OnArc<T>, PathError<T>> {
        self.core.record(Step::Angle(theta));
        radius_complete(
            self.core,
            self.spec,
            self.at,
            Dir::from_angle(theta),
            self.resolver,
        )
    }

    /// Completes the arrival with exact components; the fillet resolves.
    pub fn toward(mut self, dx: T, dy: T) -> Result<OnArc<T>, PathError<T>> {
        self.core.record(Step::Toward { dx, dy });
        let dir = verbs::director(dx, dy)?;
        radius_complete(self.core, self.spec, self.at, dir, self.resolver)
    }
}

impl<T: geom_core::Decide> RadiusArrivalDir<T> {
    /// Completes the arrival with its anchor; the fillet resolves.
    pub fn at(mut self, p: Point2<T>) -> Result<OnArc<T>, PathError<T>> {
        self.core.record(Step::At(p));
        radius_complete(self.core, self.spec, p, self.dir, self.resolver)
    }
}

/// A `Via` arrival: anchor authored in the spec, director pending.
#[derive(Clone, Debug)]
pub struct ViaArrival<T: Real> {
    core: Core<T>,
    q: Point2<T>,
    p: Point2<T>,
    resolver: verbs::ArcResolver<T>,
}

impl<T: geom_core::Decide> ViaArrival<T> {
    /// Completes the directed anchor with an angle; the fillet resolves.
    pub fn angle(mut self, theta: T) -> Result<OnArc<T>, PathError<T>> {
        self.core.record(Step::Angle(theta));
        via_complete(
            self.core,
            self.q,
            self.p,
            Dir::from_angle(theta),
            self.resolver,
        )
    }

    /// Completes the directed anchor with exact components.
    pub fn toward(mut self, dx: T, dy: T) -> Result<OnArc<T>, PathError<T>> {
        self.core.record(Step::Toward { dx, dy });
        let dir = verbs::director(dx, dy)?;
        via_complete(self.core, self.q, self.p, dir, self.resolver)
    }
}

/// Completes a Via arrival: the carrier is the circle tangent to the
/// bound direction at the anchor, through `q`.
fn via_complete<T: geom_core::Decide>(
    mut core: Core<T>,
    q: Point2<T>,
    p: Point2<T>,
    dir: Dir<T>,
    resolver: verbs::ArcResolver<T>,
) -> Result<OnArc<T>, PathError<T>> {
    let band = linear_band()?;
    let (centre, winding) = verbs::via_carrier(DirectedPoint { at: p, dir }, q, band)?;
    let dir = resolve_arc_arrival(&mut core, resolver, p, centre, winding)?;
    Ok(OnArc { core, at: p, dir })
}

/// A `Via` CLOSE: anchor at the entry, director pending.
#[derive(Clone, Debug)]
pub struct ViaArrivalStart<T: Real> {
    core: Core<T>,
    q: Point2<T>,
    resolver: verbs::ArcResolver<T>,
}

impl<T: geom_core::Decide> ViaArrivalStart<T> {
    /// Completes the close with an angle at the entry anchor.
    pub fn angle(mut self, theta: T) -> Result<ClosedLoop<T>, PathError<T>> {
        self.core.record(Step::Angle(theta));
        via_close(self.core, self.q, Dir::from_angle(theta), self.resolver)
    }

    /// Completes the close with exact components at the entry anchor.
    pub fn toward(mut self, dx: T, dy: T) -> Result<ClosedLoop<T>, PathError<T>> {
        self.core.record(Step::Toward { dx, dy });
        let dir = verbs::director(dx, dy)?;
        via_close(self.core, self.q, dir, self.resolver)
    }
}

fn via_close<T: geom_core::Decide>(
    mut core: Core<T>,
    q: Point2<T>,
    dir: Dir<T>,
    resolver: verbs::ArcResolver<T>,
) -> Result<ClosedLoop<T>, PathError<T>> {
    let start_pos = core.start_pos.ok_or(PathError::UnderdeterminedLeg {
        site: "close before the entry position is bound",
    })?;
    let band = linear_band()?;
    let (centre, winding) = verbs::via_carrier(DirectedPoint { at: start_pos, dir }, q, band)?;
    resolve_arc_close(&mut core, resolver, centre, winding)
}

// ------------------------------------------------------------------
// The incoming matrix: fused specs keyed by the consumed state.
// ------------------------------------------------------------------

/// A fused verb's INCOMING spec from a DIRECTED tip: tangent-departing,
/// endpoint DERIVED (the endpoint-free pair — the arc analogs of
/// `line(len)`).
pub trait TangentIncoming<T: ArcCarrierScalar> {
    #[doc(hidden)]
    fn leg(&self, dp: DirectedPoint<T>) -> Result<verbs::TangentArcLeg<T>, PathError<T>>;
    #[doc(hidden)]
    fn to_wire(&self) -> ArcData<T>;
}

impl<T: ArcCarrierScalar> TangentIncoming<T> for Sweep<T> {
    fn leg(&self, dp: DirectedPoint<T>) -> Result<verbs::TangentArcLeg<T>, PathError<T>> {
        verbs::tangent_arc_leg(dp, self.r, self.side, self.angle, linear_band()?)
    }
    fn to_wire(&self) -> ArcData<T> {
        ArcData::Sweep {
            r: self.r,
            side: self.side,
            angle: self.angle,
        }
    }
}

impl<T: ArcCarrierScalar> TangentIncoming<T> for ArcLen<T> {
    fn leg(&self, dp: DirectedPoint<T>) -> Result<verbs::TangentArcLeg<T>, PathError<T>> {
        verbs::tangent_arc_leg(dp, self.r, self.side, self.len / self.r, linear_band()?)
    }
    fn to_wire(&self) -> ArcData<T> {
        ArcData::ArcLen {
            r: self.r,
            side: self.side,
            len: self.len,
        }
    }
}

/// A fused verb's INCOMING spec from a POINT tip (bare anchor): the
/// endpoint-full modes, whose authored `p` is the incoming side's
/// anchor; the derived start tangent is junction-checked on a leg-end
/// tip exactly as the sharp arc legs check theirs.
pub trait PointIncoming<T: ArcCarrierScalar> {
    #[doc(hidden)]
    fn carrier(&self, at: Point2<T>) -> Result<PointCarrier<T>, PathError<T>>;
    #[doc(hidden)]
    fn to_wire(&self) -> ArcData<T>;
}

/// A point-mode incoming's derived pieces: (centre, winding, start
/// tangent, anchor).
type PointCarrier<T> = (Point2<T>, crate::sugar::ArcSweep, Dir<T>, Point2<T>);

/// The shared Bulge-shaped derivation: carrier from chord + bulge (the
/// existing closed form), winding from the bulge's sign, start tangent
/// γ − θ/2 (the M2 convention, exactly the sharp legs' derivation).
fn bulge_carrier<T: geom_core::Decide>(
    at: Point2<T>,
    p: Point2<T>,
    b: T,
) -> Result<(Point2<T>, crate::sugar::ArcSweep, Dir<T>), PathError<T>> {
    let band = linear_band()?;
    // The bulge's sign IS the travel sense, so the classification that
    // gates it degenerate also decides the winding — one funnel row.
    let winding = match crate::k_stats::decide("path_arc_bulge", geom_core::Margin::of(b), band) {
        Ok(geom_core::Sign::Positive) => crate::sugar::ArcSweep::Ccw,
        Ok(geom_core::Sign::Negative) => crate::sugar::ArcSweep::Cw,
        Ok(geom_core::Sign::Zero) => return Err(PathError::DegenerateArcSpec { value: b }),
        Err(source) => return Err(PathError::Escalated { source }),
    };
    let data = super::arc_carrier(at, p, b);
    let d = p - at;
    let gamma = d.y.atan2(d.x);
    let theta = b.atan() * T::from_f64(4.0);
    let start = Dir::from_angle(gamma - theta / T::from_f64(2.0));
    Ok((data.center, winding, start))
}

impl<T: ArcCarrierScalar> PointIncoming<T> for verbs::Bulge<T, Point2<T>> {
    fn carrier(&self, at: Point2<T>) -> Result<PointCarrier<T>, PathError<T>> {
        let band = linear_band()?;
        let chord = (self.p - at).norm_squared().sqrt();
        verbs::gate_positive("path_arc_chord", chord, band, |c| {
            PathError::DegenerateArcChord { chord: c }
        })?;
        let (c, w, start) = bulge_carrier(at, self.p, self.b)?;
        Ok((c, w, start, self.p))
    }
    fn to_wire(&self) -> ArcData<T> {
        ArcData::Bulge {
            target: Target::Point(self.p),
            b: self.b,
        }
    }
}

impl<T: ArcCarrierScalar> PointIncoming<T> for Via<T, Point2<T>> {
    fn carrier(&self, at: Point2<T>) -> Result<PointCarrier<T>, PathError<T>> {
        let band = linear_band()?;
        let chord_v = self.p - at;
        let chord = chord_v.norm_squared().sqrt();
        verbs::gate_positive("path_arc_chord", chord, band, |c| {
            PathError::DegenerateArcChord { chord: c }
        })?;
        // The collinear gate, then the existing closed form — the sharp
        // `Via` leg mode's own derivation, verbatim.
        let offset = chord_v.perp_dot(self.q - at) / chord;
        match crate::k_stats::decide("path_arc_via_offset", geom_core::Margin::of(offset), band) {
            Ok(geom_core::Sign::Zero) => return Err(PathError::ArcViaCollinear { offset }),
            Ok(_) => {}
            Err(source) => return Err(PathError::Escalated { source }),
        }
        let b = crate::sugar::bulge_from_via(at, self.q, self.p);
        let (c, w, start) = bulge_carrier(at, self.p, b)?;
        Ok((c, w, start, self.p))
    }
    fn to_wire(&self) -> ArcData<T> {
        ArcData::Via {
            q: self.q,
            target: Target::Point(self.p),
        }
    }
}

impl<T: ArcCarrierScalar> PointIncoming<T> for Center<T, Point2<T>> {
    fn carrier(&self, at: Point2<T>) -> Result<PointCarrier<T>, PathError<T>> {
        let band = linear_band()?;
        // The sharp `Center` leg mode's gates: both radii definitely
        // positive, equidistance definitely zero, chord non-degenerate.
        let r_tip = (at - self.c).norm_squared().sqrt();
        let r_end = (self.p - self.c).norm_squared().sqrt();
        for radius in [r_tip, r_end] {
            verbs::gate_positive("path_arc_center_radius", radius, band, |r| {
                PathError::DegenerateArcCenter { radius: r }
            })?;
        }
        match crate::k_stats::decide(
            "path_arc_center_equidistant",
            geom_core::Margin::of(r_tip - r_end),
            band,
        ) {
            Ok(geom_core::Sign::Zero) => {}
            Ok(_) => {
                return Err(PathError::ArcCenterNotEquidistant {
                    tip_radius: r_tip,
                    end_radius: r_end,
                });
            }
            Err(source) => return Err(PathError::Escalated { source }),
        }
        let chord = (self.p - at).norm_squared().sqrt();
        verbs::gate_positive("path_arc_chord", chord, band, |c| {
            PathError::DegenerateArcChord { chord: c }
        })?;
        let b = crate::sugar::bulge_from_center(at, self.p, self.c, self.winding);
        let d = self.p - at;
        let gamma = d.y.atan2(d.x);
        let theta = b.atan() * T::from_f64(4.0);
        let start = Dir::from_angle(gamma - theta / T::from_f64(2.0));
        // The AUTHORED centre is the carrier (never the re-derived one).
        Ok((self.c, self.winding, start, self.p))
    }
    fn to_wire(&self) -> ArcData<T> {
        ArcData::Center {
            c: self.c,
            winding: self.winding,
            target: Target::Point(self.p),
        }
    }
}

/// A fused verb's INCOMING spec from an [`OnArc`] tip: the side already
/// runs on a carrier the state cannot carry (§2c axiom), so the verb
/// re-authors it from the tip's own binding bits. `Radius { r, side }`
/// is the ONE admissible mode: the centre is DERIVED
/// (`at + side·r·n̂(tangent)`), so tangency at the tip holds by
/// construction and nothing is value-matched.
///
/// `Center` is EXCLUDED here by the same §2c round-6 doctrine that
/// excludes `Center@Directed`: an OnArc tip's direction is BOUND
/// (position + tangent), so an authored centre's derived tangent at
/// the anchor would have to value-match it — and unlike the Point
/// state there is no direction left for the centre to supply
/// retroactively. Authored-once decides: the pair is a missing impl,
/// unrepresentable.
pub trait OnArcIncoming<T: ArcCarrierScalar> {
    #[doc(hidden)]
    fn side(
        &self,
        dp: DirectedPoint<T>,
    ) -> Result<(Point2<T>, Point2<T>, crate::sugar::ArcSweep), PathError<T>>;
    #[doc(hidden)]
    fn to_wire(&self) -> ArcData<T>;
}

impl<T: ArcCarrierScalar> OnArcIncoming<T> for Radius<T> {
    fn side(
        &self,
        dp: DirectedPoint<T>,
    ) -> Result<(Point2<T>, Point2<T>, crate::sugar::ArcSweep), PathError<T>> {
        let (centre, winding) = verbs::radius_carrier(dp, *self, linear_band()?)?;
        Ok((dp.at, centre, winding))
    }
    fn to_wire(&self) -> ArcData<T> {
        ArcData::Radius {
            r: self.r,
            side: self.side,
        }
    }
}

// ------------------------------------------------------------------
// The fused verbs, per consumed state.
// ------------------------------------------------------------------

impl Open {
    /// **§2c, the entry fused verb**: authors the ENTRY side ON an arc
    /// carrier — the spec's `p` is the entry anchor, the direction is
    /// the carrier's tangent there (derived, never authored) — and
    /// opens a fillet of `radius` off that carrier, line arrival.
    ///
    /// The entry's carrier and the fillet that trims it are ONE
    /// authoring act, which is what the axiom demands: a fillet that
    /// needs an arc carrier cannot learn it, so it authors it.
    pub fn arc_fillet<T: ArcCarrierScalar>(
        self,
        spec: Center<T, Point2<T>>,
        radius: T,
    ) -> Result<PartialPath<T, NoPos, NoAng>, PathError<T>> {
        let mut core = Core::empty();
        core.record(Step::ArcFillet {
            spec: PointIncoming::to_wire(&spec),
            radius,
        });
        entry_arc_open(&mut core, &spec, radius)?;
        Ok(in_state(
            core,
            Tip {
                pos: None,
                ang: None,
                ang_by_tangent: false,
            },
        ))
    }

    /// The entry fused verb with an ARC arrival: `arc_fillet` whose
    /// arrival is the spec₂ mode's own completion (a `Center` interior
    /// anchor resolves at the verb; `Center { p: Start }` would close a
    /// two-sided loop; `Radius`/`Via` await their binders).
    pub fn arc_fillet_arc<T: ArcCarrierScalar, S2: ArrivalSpec<T>>(
        self,
        spec: Center<T, Point2<T>>,
        radius: T,
        spec2: S2,
    ) -> S2::Out {
        let mut core = Core::empty();
        core.record(Step::ArcFilletArc {
            spec: PointIncoming::to_wire(&spec),
            radius,
            spec2: spec2.to_wire(),
        });
        if let Err(e) = entry_arc_open(&mut core, &spec, radius) {
            return S2::fail(e);
        }
        S2::apply(core, spec2)
    }
}

/// The entry fused verbs' shared incoming half: seed the chain at the
/// spec's anchor, bind the entry direction to the carrier tangent
/// there, open the arc-incoming fillet.
fn entry_arc_open<T: ArcCarrierScalar>(
    core: &mut Core<T>,
    spec: &Center<T, Point2<T>>,
    radius: T,
) -> Result<(), PathError<T>> {
    let band = linear_band()?;
    let dir = carrier_tangent(spec.p, spec.c, spec.winding, band)?;
    core.seed(spec.p);
    core.start_ang = Some(dir);
    open_arc(
        core,
        PendingArc {
            anchor: spec.p,
            centre: spec.c,
            winding: spec.winding,
            radius,
            resolver: arc_fillet::resolve::<T>,
        },
    )
}

impl<T: ArcCarrierScalar, F: Flavor> PartialPath<T, HasPos<F>, HasAng> {
    /// **§2c**: line incoming, ARC arrival — consumes the directed tip
    /// (the incoming side is its ray) and opens/resolves the arc
    /// arrival per the spec mode's own completion story.
    pub fn fillet_arc<S: ArrivalSpec<T>>(mut self, radius: T, spec: S) -> S::Out {
        self.core.record(Step::FilletArc {
            radius,
            spec: spec.to_wire(),
        });
        let (at, ang) = match self.dep() {
            Ok(v) => v,
            Err(e) => return S::fail(e),
        };
        if let Err(e) = open_ray(
            &mut self.core,
            at,
            ang,
            radius,
            self.tip.ang_by_tangent,
            self.tip.pos.as_ref().and_then(|p| p.incoming),
        ) {
            return S::fail(e);
        }
        S::apply(self.core, spec)
    }

    /// **§2c**: fused ARC incoming from a directed tip — the
    /// endpoint-free pair departs tangentially and derives its
    /// endpoint, which becomes the incoming side's anchor; the fillet
    /// of `radius` trims off its far end. Line arrival.
    pub fn arc_fillet<S: TangentIncoming<T>>(
        mut self,
        spec: S,
        radius: T,
    ) -> Result<PartialPath<T, NoPos, NoAng>, PathError<T>> {
        self.core.record(Step::ArcFillet {
            spec: spec.to_wire(),
            radius,
        });
        self.tangent_arc_open(&spec, radius)?;
        Ok(in_state(
            self.core,
            Tip {
                pos: None,
                ang: None,
                ang_by_tangent: false,
            },
        ))
    }

    /// **§2c**: fused arc incoming AND arc arrival.
    pub fn arc_fillet_arc<Si: TangentIncoming<T>, S2: ArrivalSpec<T>>(
        mut self,
        spec: Si,
        radius: T,
        spec2: S2,
    ) -> S2::Out {
        self.core.record(Step::ArcFilletArc {
            spec: spec.to_wire(),
            radius,
            spec2: spec2.to_wire(),
        });
        if let Err(e) = self.tangent_arc_open(&spec, radius) {
            return S2::fail(e);
        }
        S2::apply(self.core, spec2)
    }

    /// The tangent-departing fused incoming: derive the leg, run the §4
    /// item 4 identity check against an inherited departure's carrier,
    /// and open the arc-incoming fillet. Nothing is emitted here — the
    /// trimmed run is the resolution's emission, from the chain's head.
    fn tangent_arc_open<S: TangentIncoming<T>>(
        &mut self,
        spec: &S,
        radius: T,
    ) -> Result<(), PathError<T>> {
        let (at, ang) = self.dep()?;
        let leg = spec.leg(DirectedPoint { at, dir: ang })?;
        if self.tip.ang_by_tangent
            && let Some(inc) = self.tip.pos.as_ref().and_then(|pd| pd.incoming.as_ref())
            && let Some(prev) = &inc.carrier
        {
            super::refuse_identical_carriers(
                prev,
                &SegArc {
                    center: leg.centre,
                    radius: (at - leg.centre).norm_squared().sqrt(),
                },
            )?;
        }
        open_arc(
            &mut self.core,
            PendingArc {
                anchor: leg.end,
                centre: leg.centre,
                winding: leg.winding,
                radius,
                resolver: arc_fillet::resolve::<T>,
            },
        )
    }

    /// **§2c**: the endpoint-free SHARP arc legs — `arc_to(spec)` with
    /// `Sweep`/`ArcLen`, the arc analogs of `line(len)`: tangent
    /// departure (already junction-checked when the director bound),
    /// endpoint derived, terminating at a directed point.
    pub fn arc_to<S: TangentIncoming<T>>(
        mut self,
        spec: S,
    ) -> Result<PartialPath<T, HasPos<WithIncoming>, NoAng>, PathError<T>> {
        self.core.record(Step::ArcTo(spec.to_wire()));
        let (at, ang) = self.dep()?;
        let leg = spec.leg(DirectedPoint { at, dir: ang })?;
        let carrier = SegArc {
            center: leg.centre,
            radius: (at - leg.centre).norm_squared().sqrt(),
        };
        if self.tip.ang_by_tangent
            && let Some(inc) = self.tip.pos.as_ref().and_then(|pd| pd.incoming.as_ref())
            && let Some(prev) = &inc.carrier
        {
            super::refuse_identical_carriers(prev, &carrier)?;
        }
        self.core.push_arc(leg.end, leg.bulge, carrier)?;
        let arm = carrier.radius.min(leg.chord);
        Ok(in_state(
            self.core,
            leg_end_tip(leg.end, leg.end_dir, arm, Some(carrier)),
        ))
    }
}

impl<T: ArcCarrierScalar, F: Flavor> PartialPath<T, HasPos<F>, NoAng> {
    /// **§2c**: fused arc incoming from a POINT tip — the endpoint-full
    /// modes author the incoming side's carrier and its anchor `p` in
    /// one act; on a leg-end tip the derived start tangent is
    /// junction-checked exactly as the sharp legs check theirs. Line
    /// arrival.
    pub fn arc_fillet<S: PointIncoming<T>>(
        mut self,
        spec: S,
        radius: T,
    ) -> Result<PartialPath<T, NoPos, NoAng>, PathError<T>> {
        self.core.record(Step::ArcFillet {
            spec: spec.to_wire(),
            radius,
        });
        self.point_arc_open(&spec, radius)?;
        Ok(in_state(
            self.core,
            Tip {
                pos: None,
                ang: None,
                ang_by_tangent: false,
            },
        ))
    }

    /// **§2c**: fused arc incoming (point modes) AND arc arrival.
    pub fn arc_fillet_arc<Si: PointIncoming<T>, S2: ArrivalSpec<T>>(
        mut self,
        spec: Si,
        radius: T,
        spec2: S2,
    ) -> S2::Out {
        self.core.record(Step::ArcFilletArc {
            spec: spec.to_wire(),
            radius,
            spec2: spec2.to_wire(),
        });
        if let Err(e) = self.point_arc_open(&spec, radius) {
            return S2::fail(e);
        }
        S2::apply(self.core, spec2)
    }

    fn point_arc_open<S: PointIncoming<T>>(
        &mut self,
        spec: &S,
        radius: T,
    ) -> Result<(), PathError<T>> {
        let pos = self.tip.pos.as_ref().ok_or(PathError::UnderdeterminedLeg {
            site: "fused arc incoming on a tip without a position",
        })?;
        let at = pos.at;
        let (centre, winding, start, anchor) = spec.carrier(at)?;
        if let Some(inc) = &pos.incoming {
            junction_check(inc, start, false)?;
        }
        if self.core.start_ang.is_none() {
            self.core.start_ang = Some(start);
        }
        open_arc(
            &mut self.core,
            PendingArc {
                anchor,
                centre,
                winding,
                radius,
                resolver: arc_fillet::resolve::<T>,
            },
        )
    }
}

impl<T: ArcCarrierScalar> PartialPath<T, HasPos<WithIncoming>, NoAng> {
    /// **§2c round 10 — RAY EXTENSION**: bare `fillet(r)` directly on a
    /// leg end. The incoming contact sits on the TANGENT RAY ahead of
    /// the directed point, as new path: the surviving ray piece is a
    /// genuine line leg extending from the leg's end (declared tangent
    /// by construction — the ray IS the tangent), whatever leg came
    /// before. Line arrival.
    pub fn fillet(mut self, radius: T) -> Result<PartialPath<T, NoPos, NoAng>, PathError<T>> {
        self.core.record(Step::Fillet { radius });
        self.ray_extend(radius)?;
        Ok(in_state(
            self.core,
            Tip {
                pos: None,
                ang: None,
                ang_by_tangent: false,
            },
        ))
    }

    /// Ray extension with an ARC arrival (`fillet_arc` off a leg end).
    pub fn fillet_arc<S: ArrivalSpec<T>>(mut self, radius: T, spec: S) -> S::Out {
        self.core.record(Step::FilletArc {
            radius,
            spec: spec.to_wire(),
        });
        if let Err(e) = self.ray_extend(radius) {
            return S::fail(e);
        }
        S::apply(self.core, spec)
    }

    /// The shared ray-extension opening: inherit the incoming end
    /// tangent, declare the (constructed) tangency at the leg end, and
    /// open the ray-incoming fillet there — `.tangent().fillet(r)`'s
    /// exact emissions, in one verb.
    fn ray_extend(&mut self, radius: T) -> Result<(), PathError<T>> {
        let pos = self.tip.pos.as_ref().ok_or(PathError::UnderdeterminedLeg {
            site: "ray extension on a tip without a position",
        })?;
        let at = pos.at;
        let inc = pos.incoming.ok_or(PathError::UnderdeterminedLeg {
            site: "ray extension on a tip without incoming data",
        })?;
        self.core.declare_last();
        open_ray(&mut self.core, at, inc.ang, radius, true, Some(inc))
    }
}

impl<T: ArcCarrierScalar> OnArc<T> {
    /// **§2c**: the fused verb off an [`OnArc`] tip — the one place the
    /// carrier run into the next trim is emitted (from the chain's
    /// head, along the carrier the spec authors). Line arrival.
    pub fn arc_fillet<S: OnArcIncoming<T>>(
        mut self,
        spec: S,
        radius: T,
    ) -> Result<PartialPath<T, NoPos, NoAng>, PathError<T>> {
        self.core.record(Step::ArcFillet {
            spec: spec.to_wire(),
            radius,
        });
        let (anchor, centre, winding) = spec.side(self.dp())?;
        open_arc(
            &mut self.core,
            PendingArc {
                anchor,
                centre,
                winding,
                radius,
                resolver: arc_fillet::resolve::<T>,
            },
        )?;
        Ok(in_state(
            self.core,
            Tip {
                pos: None,
                ang: None,
                ang_by_tangent: false,
            },
        ))
    }

    /// The [`OnArc`] fused verb with an ARC arrival.
    pub fn arc_fillet_arc<Si: OnArcIncoming<T>, S2: ArrivalSpec<T>>(
        mut self,
        spec: Si,
        radius: T,
        spec2: S2,
    ) -> S2::Out {
        self.core.record(Step::ArcFilletArc {
            spec: spec.to_wire(),
            radius,
            spec2: spec2.to_wire(),
        });
        let (anchor, centre, winding) = match spec.side(self.dp()) {
            Ok(v) => v,
            Err(e) => return S2::fail(e),
        };
        if let Err(e) = open_arc(
            &mut self.core,
            PendingArc {
                anchor,
                centre,
                winding,
                radius,
                resolver: arc_fillet::resolve::<T>,
            },
        ) {
            return S2::fail(e);
        }
        S2::apply(self.core, spec2)
    }
}

// ------------------------------------------------------------------
// The SHARP arc leg from a POINT tip: `arc_to(spec)` over the
// endpoint-full modes (§2c rounds 5–9; PATHS-DESIGN §2 "Legs").
// ------------------------------------------------------------------

/// The sharp arc leg's spec from a POINT tip — the endpoint-full modes
/// (`Bulge{p, b}`, `Via{q, p}`, `Center{c, winding, p}`), each carrying
/// its own target because the endpoint-free modes made `p` non-uniform
/// (§2c round 8). `Out` is the mode's own completion: a directed point
/// for an interior target, the closed loop for [`Start`].
///
/// Admissibility is the state-keyed matrix: the endpoint-FREE pair
/// (`Sweep`/`ArcLen`) has no impl here — from a bare point there is no
/// departure tangent to sweep about, so that pair is unrepresentable
/// rather than refused. It reaches `arc_to` from the Directed tip
/// instead ([`TangentIncoming`]).
/// SEALED, on the same rule as the lattice markers: the admissible
/// (state, mode) pairs ARE the matrix, so a foreign impl would mint a
/// row the doctrine does not have. The six mode types below are the
/// whole implementor set.
pub trait PointLeg<T: geom_core::Decide, F: Flavor>: super::sealed::Sealed {
    /// The state the leg leaves the chain in.
    type Out;
    #[doc(hidden)]
    fn leg_from(path: PartialPath<T, HasPos<F>, NoAng>, spec: Self) -> Self::Out;
}

impl<T, Tgt> super::sealed::Sealed for verbs::Bulge<T, Tgt> {}
impl<T: Real, Tgt> super::sealed::Sealed for Via<T, Tgt> {}
impl<T: Real, Tgt> super::sealed::Sealed for Center<T, Tgt> {}

impl<T: geom_core::Decide, F: Flavor> PointLeg<T, F> for verbs::Bulge<T, Point2<T>> {
    type Out = Result<PartialPath<T, HasPos<WithIncoming>, NoAng>, PathError<T>>;
    fn leg_from(mut path: PartialPath<T, HasPos<F>, NoAng>, spec: Self) -> Self::Out {
        path.core.record(Step::ArcTo(ArcData::Bulge {
            target: Target::Point(spec.p),
            b: spec.b,
        }));
        path.arc_to_point(spec.p, spec.b)
    }
}

impl<T: geom_core::Decide, F: Flavor> PointLeg<T, F> for verbs::Bulge<T, Start> {
    type Out = Result<ClosedLoop<T>, PathError<T>>;
    fn leg_from(mut path: PartialPath<T, HasPos<F>, NoAng>, spec: Self) -> Self::Out {
        path.core.record(Step::ArcTo(ArcData::Bulge {
            target: Target::Start,
            b: spec.b,
        }));
        path.arc_to_start(spec.b)
    }
}

impl<T: geom_core::Decide, F: Flavor> PointLeg<T, F> for Via<T, Point2<T>> {
    type Out = Result<PartialPath<T, HasPos<WithIncoming>, NoAng>, PathError<T>>;
    fn leg_from(mut path: PartialPath<T, HasPos<F>, NoAng>, spec: Self) -> Self::Out {
        path.core.record(Step::ArcTo(ArcData::Via {
            q: spec.q,
            target: Target::Point(spec.p),
        }));
        let bulge = path.arc_via_bulge(spec.q, spec.p)?;
        path.arc_to_point(spec.p, bulge)
    }
}

impl<T: geom_core::Decide, F: Flavor> PointLeg<T, F> for Via<T, Start> {
    type Out = Result<ClosedLoop<T>, PathError<T>>;
    fn leg_from(mut path: PartialPath<T, HasPos<F>, NoAng>, spec: Self) -> Self::Out {
        path.core.record(Step::ArcTo(ArcData::Via {
            q: spec.q,
            target: Target::Start,
        }));
        let bulge = path.arc_via_bulge(spec.q, path.start_target()?)?;
        path.arc_to_start(bulge)
    }
}

impl<T: geom_core::Decide, F: Flavor> PointLeg<T, F> for Center<T, Point2<T>> {
    type Out = Result<PartialPath<T, HasPos<WithIncoming>, NoAng>, PathError<T>>;
    fn leg_from(mut path: PartialPath<T, HasPos<F>, NoAng>, spec: Self) -> Self::Out {
        path.core.record(Step::ArcTo(ArcData::Center {
            c: spec.c,
            winding: spec.winding,
            target: Target::Point(spec.p),
        }));
        let bulge = path.arc_center_bulge(spec.c, spec.p, spec.winding)?;
        path.arc_to_point(spec.p, bulge)
    }
}

impl<T: geom_core::Decide, F: Flavor> PointLeg<T, F> for Center<T, Start> {
    type Out = Result<ClosedLoop<T>, PathError<T>>;
    fn leg_from(mut path: PartialPath<T, HasPos<F>, NoAng>, spec: Self) -> Self::Out {
        path.core.record(Step::ArcTo(ArcData::Center {
            c: spec.c,
            winding: spec.winding,
            target: Target::Start,
        }));
        let bulge = path.arc_center_bulge(spec.c, path.start_target()?, spec.winding)?;
        path.arc_to_start(bulge)
    }
}

impl<T: geom_core::Decide, F: Flavor> PartialPath<T, HasPos<F>, NoAng> {
    /// **§2c**: the SHARP arc leg from a point tip — one verb over the
    /// endpoint-full `ArcData` modes (`Bulge{p, b}` chord-relative,
    /// `Via{q, p}` through a point, `Center{c, winding, p}` about a
    /// centre). `p: Start` is the sharp arc seam.
    ///
    /// Each mode's authored data is stored VERBATIM and its bulge
    /// derived by the one closed form the raw chain uses
    /// ([`crate::bulge_from_via`] / [`crate::bulge_from_center`]), so
    /// the doors emit the same bits. On a directed point the §4 item 1
    /// junction check runs on the arc's START TANGENT.
    ///
    /// Refusals: a through-point within ε_input of the chord LINE
    /// ([`PathError::ArcViaCollinear`] — the whole collinear class);
    /// coincident endpoints ([`PathError::DegenerateArcChord`]); a
    /// centre whose two radii disagree definitely
    /// ([`PathError::ArcCenterNotEquidistant`] — checked, never
    /// repaired: re-projecting would move an authored point, which §4
    /// item 3 forbids) or sits within ε_input of an endpoint
    /// ([`PathError::DegenerateArcCenter`]).
    pub fn arc_to<S: PointLeg<T, F>>(self, spec: S) -> S::Out {
        S::leg_from(self, spec)
    }
}
