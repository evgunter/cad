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
//! | `Bulge{p,b}` | Point | Point (both flavors) | — (no chord) |
//! | `Via{q,p}` | Point | Point (both flavors) | Directed anchor (director pending) |
//! | `Center{c,w,p}` | Point | Entry, Point (both flavors) | complete (resolves at the verb; interior `p` lands on a directed point; `p: Start` closes) |
//! | `Radius{r,side}` | — | directed point — ARC EXTENSION (centre DERIVED from the tip's two binding bits; a plain point has no tangent to derive from, so the pair is a missing impl) | Directed anchor (binders pending) |
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
//! use geom_core::{Point2, Tol};
//! use profile::{ArcSweep, Center, Open, Start};
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let tol = Tol::witness();
//! let p = Point2::new;
//! let tip = 0.75_f64.sqrt();
//! let lens = Open.arc_fillet_arc(
//!     Center { c: p(-0.5, 0.0), winding: ArcSweep::Ccw, p: p(0.0, -tip) },
//!     0.25,
//!     Center { c: p(0.5, 0.0), winding: ArcSweep::Ccw, p: Start },
//!     tol,
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
//! use geom_core::Tol;
//! use geom_core::Point2;
//! use profile::{ArcSweep, Center, Open, Start};
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let tol = Tol::witness();
//! let p = Point2::new;
//! let quarter = Open.at(p(0.0, 2.0))
//!     .line_to(p(0.0, 0.0), tol)?
//!     .toward(1.0_f64, 0.0, tol)?
//!     .fillet_arc(0.5, Center { c: p(0.0, 0.0), winding: ArcSweep::Ccw, p: Start }, tol)?;
//! assert_eq!(quarter.loop_.vertices().len(), 4);
//! # Ok(())
//! # }
//! ```
//!
//! **Arc extension.** An interior `Center` arrival emits its run to
//! the authored anchor and lands on an ordinary directed point there —
//! position plus incoming tangent, uniform with every leg end. A
//! `Radius { r, side }` fused incoming DERIVES its carrier from those
//! two bits, so tangency at the tip holds by construction: the same
//! `r` continues the arrival carrier (the run extends forward), and
//! any other `r` is a new tangent carrier constructed at the tip.
//!
//! ```
//! use geom_core::Point2;
//! use geom_core::Tol;
//! use profile::{ArcSide, ArcSweep, Center, Open, Radius, Start};
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let tol = Tol::witness();
//! let p = Point2::new;
//! let boss = Open.at(p(5.05, -1.6))
//!     .toward(2.1_f64, 0.8, tol)?
//!     // Onto the boss circle, blended.
//!     .fillet_arc(0.5, Center { c: p(7.0, 0.0), winding: ArcSweep::Ccw, p: p(8.5, 0.0) }, tol)?
//!     // Off it again: r = 1.5 and Left re-derive the centre (7, 0)
//!     // from the tip's own position and tangent, exactly.
//!     .arc_fillet(Radius { r: 1.5, side: ArcSide::Left }, 0.5, tol)?
//!     .at(p(4.05, 1.35), tol)?
//!     .toward(-4.1, 0.3, tol)?
//!     .line(1.0, tol)?
//!     .line_to(Start, tol)?;
//! assert!(boss.loop_.tangent_joints().len() >= 4);
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
//! use geom_core::Tol;
//! use core::f64::consts::FRAC_PI_2;
//! use geom_core::Point2;
//! use profile::{ArcSide, Open, Start, Sweep};
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let tol = Tol::witness();
//! let p = Point2::new;
//! let hook = Open.at(p(0.0, 0.0))
//!     .toward(1.0_f64, 0.0, tol)?
//!     .arc_to(Sweep { r: 1.0, side: ArcSide::Left, angle: FRAC_PI_2 }, tol)?
//!     .fillet(0.25, tol)?
//!     .at(p(0.0, 3.0), tol)?
//!     .toward(-1.0, 0.0, tol)?
//!     .line(1.0, tol)?
//!     .line_to(Start, tol)?;
//! assert_eq!(hook.loop_.vertices().len(), 5);
//! # Ok(())
//! # }
//! ```
//!
//! Every row is spelled by ONE verb name per site: `arc_to(spec)` is
//! the sharp arc leg from both the Point tip (`Bulge`/`Via`/`Center`,
//! [`PointLeg`]) and the Directed tip (`Sweep`/`ArcLen`,
//! [`TangentIncoming`]); the fused verbs take their incoming mode the
//! same way.

use geom_core::{Point2, Real, Sign, Tol};

use super::arc_fillet::{self, ArcCarrierScalar, carrier_tangent};
use super::program::{ArcData, ClosedLoop, Step, Target};
use super::verbs::{self, ArcLen, Center, DirectedPoint, PendingArc, Radius, Sweep, Via};
use super::{
    ArcData as SegArc, Core, Dir, FirstSeg, Flavor, HasAng, HasPos, Incoming, NoAng, NoPos, Open,
    PartialPath, PathError, PendingMeta, Plain, Start, Tip, WithIncoming, carriers_are_identical,
    in_state, junction_check, leg_end_tip, linear_band,
};

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
    tol: Tol,
) -> Result<(), PathError<T>> {
    let band = linear_band(tol)?;
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
        extends_carrier: false,
    });
    Ok(())
}

/// Opens a fillet with an AUTHORED-ARC incoming (no step recorded).
pub(super) fn open_arc<T: ArcCarrierScalar>(
    core: &mut Core<T>,
    arc: PendingArc<T>,
    tol: Tol,
) -> Result<(), PathError<T>> {
    open_arc_from_tip(core, arc, false, None, tol)
}

/// [`open_arc`] with the DIRECTED-POINT bookkeeping: `extends_carrier`
/// marks a same-carrier arc extension (the incoming emission moves the
/// origin leg's end vertex — the §4 item 4 exemption) and
/// `origin_incoming` records the tip the side departs from.
pub(super) fn open_arc_from_tip<T: ArcCarrierScalar>(
    core: &mut Core<T>,
    arc: PendingArc<T>,
    extends_carrier: bool,
    origin_incoming: Option<Incoming<T>>,
    tol: Tol,
) -> Result<(), PathError<T>> {
    let band = linear_band(tol)?;
    verbs::gate_positive("path_fillet_radius", arc.radius, band, |r| {
        PathError::NonpositiveFilletRadius { radius: r }
    })?;
    core.pending = Some(verbs::Pending::Arc(arc));
    core.pending_meta = Some(PendingMeta {
        by_tangent: false,
        origin_incoming,
        extends_carrier,
    });
    Ok(())
}

/// Whether the pending side's incoming emission EXTENDS the chain's
/// last leg (ray extension of a straight leg; arc extension of a
/// carrier leg) — the §4 item 4 vertex-move exemption, decided at the
/// verb and read at every resolution site.
fn merge_of<T: Real>(pending: &verbs::Pending<T>, meta: &PendingMeta<T>) -> bool {
    match pending {
        verbs::Pending::Ray(_) => {
            meta.by_tangent
                && meta
                    .origin_incoming
                    .as_ref()
                    .is_some_and(|i| i.carrier.is_none())
        }
        verbs::Pending::Arc(_) => meta.extends_carrier,
    }
}

/// Resolves the open fillet against an ARC ARRIVAL about `centre`,
/// anchored at `anchor` — the interior form (§2c dissolution): the
/// verb emits its WHOLE arrival side — the fillet arc, then the
/// carrier run to the authored anchor (the carrier is the verb's own
/// authored spec, so the emission is axiom-clean) — and the tip lands
/// as an ordinary directed point at the anchor (a HARD anchor,
/// uniform with line arrivals).
pub(super) fn resolve_arc_arrival<T: geom_core::Decide>(
    mut core: Core<T>,
    resolver: verbs::ArcResolver<T>,
    anchor: Point2<T>,
    centre: Point2<T>,
    winding: crate::sugar::ArcSweep,
    tol: Tol,
) -> Result<PartialPath<T, HasPos<WithIncoming>, NoAng>, PathError<T>> {
    let band = linear_band(tol)?;
    let dir = carrier_tangent(anchor, centre, winding, band)?;
    let (pending, meta) =
        core.take_pending("arc-carrier fillet arrival without an opened fillet")?;
    let merge = merge_of(&pending, &meta);
    let trims = resolver(
        core.guide_mut(),
        pending.side(),
        arc_fillet::FilletSide {
            anchor,
            carrier: arc_fillet::SideCarrier::Circle { centre, winding },
        },
        pending.radius(),
        tol,
    )?;
    core.emit_fillet_in(&trims, merge, tol)?;
    // The carrier run to the anchor follows the fillet arc tangentially
    // by construction, so the arc's outgoing joint is declared exactly
    // when that run exists; on an exact fit the fillet arc ends the
    // side at the anchor itself and the outgoing direction stays free.
    core.emit_fillet_arc(&trims, trims.fit_out == Sign::Positive)?;
    let tip = if trims.fit_out == Sign::Positive {
        let head = core.head()?;
        let bulge = crate::sugar::bulge_from_center(head, anchor, centre, winding);
        let radius = (anchor - centre).norm_squared().sqrt();
        let carrier = SegArc {
            center: centre,
            radius,
        };
        core.push_arc(anchor, bulge, carrier)?;
        let chord = (anchor - head).norm_squared().sqrt();
        leg_end_tip(anchor, dir, radius.min(chord), Some(carrier))
    } else {
        // Exact fit: the fillet arc IS the whole arrival side — the
        // authored anchor is absorbed into the tangent point the fit
        // gate classified as coincident with it. The tip's DIRECTION
        // comes from the ARRIVAL carrier's tangent at the arc's end
        // (the authored spec's own bits), while its recorded CARRIER
        // is the fillet arc's — the segment actually behind the tip,
        // which is what the §4 item 4 bookkeeping must compare
        // against. The two circles are tangent there by construction,
        // so the direction agrees between them to the band.
        let head = core.head()?;
        let end_dir = carrier_tangent(head, centre, winding, band)?;
        let chord = (head - trims.t1).norm_squared().sqrt();
        leg_end_tip(head, end_dir, trims.arc.radius.min(chord), Some(trims.arc))
    };
    Ok(in_state(core, tip))
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
    tol: Tol,
) -> Result<ClosedLoop<T>, PathError<T>> {
    let start_pos = core.start_pos.ok_or(PathError::UnderdeterminedLeg {
        site: "close before the entry position is bound",
    })?;
    let start_ang = core.start_ang.ok_or(PathError::UnderdeterminedLeg {
        site: "close before the entry direction is bound",
    })?;
    let band = linear_band(tol)?;
    // The arrival's END tangent is the carrier's tangent at the entry
    // point — the incoming half of the seam junction.
    let end_ang = carrier_tangent(start_pos, centre, winding, band)?;
    let (pending, meta) = core.take_pending("arc-carrier close without an opened fillet")?;
    let merge = merge_of(&pending, &meta);
    let trims = resolver(
        core.guide_mut(),
        pending.side(),
        arc_fillet::FilletSide {
            anchor: start_pos,
            carrier: arc_fillet::SideCarrier::Circle { centre, winding },
        },
        pending.radius(),
        tol,
    )?;
    core.emit_fillet_in(&trims, merge, tol)?;
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
            tol,
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
            tol,
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
    fn apply(core: Core<T>, spec: Self, tol: Tol) -> Self::Out;
    #[doc(hidden)]
    fn fail(err: PathError<T>) -> Self::Out;
    #[doc(hidden)]
    fn to_wire(&self) -> ArcData<T>;
}

/// `Center { c, winding, p }` with an INTERIOR anchor: complete at the
/// verb (the anchor and the derived direction are one authored act —
/// the arrival's own carrier, so the verb emits its whole side) and
/// land on an ordinary directed point at `p`.
impl<T: ArcCarrierScalar> ArrivalSpec<T> for Center<T, Point2<T>> {
    type Out = Result<PartialPath<T, HasPos<WithIncoming>, NoAng>, PathError<T>>;
    fn apply(core: Core<T>, spec: Self, tol: Tol) -> Self::Out {
        resolve_arc_arrival(
            core,
            arc_fillet::resolve::<T>,
            spec.p,
            spec.c,
            spec.winding,
            tol,
        )
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
    fn apply(mut core: Core<T>, spec: Self, tol: Tol) -> Self::Out {
        let Start = spec.p;
        resolve_arc_close(
            &mut core,
            arc_fillet::resolve::<T>,
            spec.c,
            spec.winding,
            tol,
        )
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
    fn apply(core: Core<T>, spec: Self, _tol: Tol) -> Self::Out {
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
    fn apply(core: Core<T>, spec: Self, _tol: Tol) -> Self::Out {
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
    fn apply(core: Core<T>, spec: Self, _tol: Tol) -> Self::Out {
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
    core: Core<T>,
    spec: Radius<T>,
    at: Point2<T>,
    dir: Dir<T>,
    resolver: verbs::ArcResolver<T>,
    tol: Tol,
) -> Result<PartialPath<T, HasPos<WithIncoming>, NoAng>, PathError<T>> {
    let band = linear_band(tol)?;
    let (centre, winding) = verbs::radius_carrier(DirectedPoint { at, dir }, spec, band)?;
    resolve_arc_arrival(core, resolver, at, centre, winding, tol)
}

impl<T: geom_core::Decide> RadiusArrival<T> {
    /// The kernel behind the table's anchor-binding row (recording is
    /// the row's, not the kernel's).
    pub(super) fn at_kernel(mut self, step: Step<T>, p: Point2<T>) -> RadiusArrivalAt<T> {
        self.core.record(step);
        RadiusArrivalAt {
            core: self.core,
            spec: self.spec,
            at: p,
            resolver: self.resolver,
        }
    }

    /// The kernel behind the table's angle-first row (recording is the
    /// row's, not the kernel's).
    pub(super) fn angle_kernel(mut self, step: Step<T>, theta: T) -> RadiusArrivalDir<T> {
        self.core.record(step);
        RadiusArrivalDir {
            core: self.core,
            spec: self.spec,
            dir: Dir::from_angle(theta),
            resolver: self.resolver,
        }
    }

    /// The kernel behind the table's components-first row (recording is
    /// the row's, not the kernel's).
    pub(super) fn toward_kernel(
        mut self,
        step: Step<T>,
        dx: T,
        dy: T,
        tol: Tol,
    ) -> Result<RadiusArrivalDir<T>, PathError<T>> {
        self.core.record(step);
        let dir = verbs::director(dx, dy, tol)?;
        Ok(RadiusArrivalDir {
            core: self.core,
            spec: self.spec,
            dir,
            resolver: self.resolver,
        })
    }
}

impl<T: geom_core::Decide> RadiusArrivalAt<T> {
    /// The kernel behind the table's arrival-completing row (recording
    /// is the row's, not the kernel's).
    pub(super) fn angle_kernel(
        mut self,
        step: Step<T>,
        theta: T,
        tol: Tol,
    ) -> Result<PartialPath<T, HasPos<WithIncoming>, NoAng>, PathError<T>> {
        self.core.record(step);
        radius_complete(
            self.core,
            self.spec,
            self.at,
            Dir::from_angle(theta),
            self.resolver,
            tol,
        )
    }

    /// The kernel behind the table's arrival-completing row (recording
    /// is the row's, not the kernel's).
    pub(super) fn toward_kernel(
        mut self,
        step: Step<T>,
        dx: T,
        dy: T,
        tol: Tol,
    ) -> Result<PartialPath<T, HasPos<WithIncoming>, NoAng>, PathError<T>> {
        self.core.record(step);
        let dir = verbs::director(dx, dy, tol)?;
        radius_complete(self.core, self.spec, self.at, dir, self.resolver, tol)
    }
}

impl<T: geom_core::Decide> RadiusArrivalDir<T> {
    /// The kernel behind the table's arrival-completing row (recording
    /// is the row's, not the kernel's).
    pub(super) fn at_kernel(
        mut self,
        step: Step<T>,
        p: Point2<T>,
        tol: Tol,
    ) -> Result<PartialPath<T, HasPos<WithIncoming>, NoAng>, PathError<T>> {
        self.core.record(step);
        radius_complete(self.core, self.spec, p, self.dir, self.resolver, tol)
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
    /// The kernel behind the table's anchor-completing row (recording is
    /// the row's, not the kernel's).
    pub(super) fn angle_kernel(
        mut self,
        step: Step<T>,
        theta: T,
        tol: Tol,
    ) -> Result<PartialPath<T, HasPos<WithIncoming>, NoAng>, PathError<T>> {
        self.core.record(step);
        via_complete(
            self.core,
            self.q,
            self.p,
            Dir::from_angle(theta),
            self.resolver,
            tol,
        )
    }

    /// The kernel behind the table's anchor-completing row (recording is
    /// the row's, not the kernel's).
    pub(super) fn toward_kernel(
        mut self,
        step: Step<T>,
        dx: T,
        dy: T,
        tol: Tol,
    ) -> Result<PartialPath<T, HasPos<WithIncoming>, NoAng>, PathError<T>> {
        self.core.record(step);
        let dir = verbs::director(dx, dy, tol)?;
        via_complete(self.core, self.q, self.p, dir, self.resolver, tol)
    }
}

/// Completes a Via arrival: the carrier is the circle tangent to the
/// bound direction at the anchor, through `q`.
fn via_complete<T: geom_core::Decide>(
    core: Core<T>,
    q: Point2<T>,
    p: Point2<T>,
    dir: Dir<T>,
    resolver: verbs::ArcResolver<T>,
    tol: Tol,
) -> Result<PartialPath<T, HasPos<WithIncoming>, NoAng>, PathError<T>> {
    let band = linear_band(tol)?;
    let (centre, winding) = verbs::via_carrier(DirectedPoint { at: p, dir }, q, band)?;
    resolve_arc_arrival(core, resolver, p, centre, winding, tol)
}

/// A `Via` CLOSE: anchor at the entry, director pending.
#[derive(Clone, Debug)]
pub struct ViaArrivalStart<T: Real> {
    core: Core<T>,
    q: Point2<T>,
    resolver: verbs::ArcResolver<T>,
}

impl<T: geom_core::Decide> ViaArrivalStart<T> {
    /// The kernel behind the table's Via-close row (recording is the
    /// row's, not the kernel's).
    pub(super) fn angle_kernel(
        mut self,
        step: Step<T>,
        theta: T,
        tol: Tol,
    ) -> Result<ClosedLoop<T>, PathError<T>> {
        self.core.record(step);
        via_close(
            self.core,
            self.q,
            Dir::from_angle(theta),
            self.resolver,
            tol,
        )
    }

    /// The kernel behind the table's Via-close row (recording is the
    /// row's, not the kernel's).
    pub(super) fn toward_kernel(
        mut self,
        step: Step<T>,
        dx: T,
        dy: T,
        tol: Tol,
    ) -> Result<ClosedLoop<T>, PathError<T>> {
        self.core.record(step);
        let dir = verbs::director(dx, dy, tol)?;
        via_close(self.core, self.q, dir, self.resolver, tol)
    }
}

fn via_close<T: geom_core::Decide>(
    mut core: Core<T>,
    q: Point2<T>,
    dir: Dir<T>,
    resolver: verbs::ArcResolver<T>,
    tol: Tol,
) -> Result<ClosedLoop<T>, PathError<T>> {
    let start_pos = core.start_pos.ok_or(PathError::UnderdeterminedLeg {
        site: "close before the entry position is bound",
    })?;
    let band = linear_band(tol)?;
    let (centre, winding) = verbs::via_carrier(DirectedPoint { at: start_pos, dir }, q, band)?;
    resolve_arc_close(&mut core, resolver, centre, winding, tol)
}

// ------------------------------------------------------------------
// The incoming matrix: fused specs keyed by the consumed state.
// ------------------------------------------------------------------

/// A fused verb's INCOMING spec from a DIRECTED tip: tangent-departing,
/// endpoint DERIVED (the endpoint-free pair — the arc analogs of
/// `line(len)`).
pub trait TangentIncoming<T: ArcCarrierScalar> {
    #[doc(hidden)]
    fn leg(&self, dp: DirectedPoint<T>, tol: Tol) -> Result<verbs::TangentArcLeg<T>, PathError<T>>;
    #[doc(hidden)]
    fn to_wire(&self) -> ArcData<T>;
}

impl<T: ArcCarrierScalar> TangentIncoming<T> for Sweep<T> {
    fn leg(&self, dp: DirectedPoint<T>, tol: Tol) -> Result<verbs::TangentArcLeg<T>, PathError<T>> {
        verbs::tangent_arc_leg(dp, self.r, self.side, self.angle, linear_band(tol)?)
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
    fn leg(&self, dp: DirectedPoint<T>, tol: Tol) -> Result<verbs::TangentArcLeg<T>, PathError<T>> {
        verbs::tangent_arc_leg(dp, self.r, self.side, self.len / self.r, linear_band(tol)?)
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
    fn carrier(&self, at: Point2<T>, tol: Tol) -> Result<PointCarrier<T>, PathError<T>>;
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
    tol: Tol,
) -> Result<(Point2<T>, crate::sugar::ArcSweep, Dir<T>), PathError<T>> {
    let band = linear_band(tol)?;
    // The bulge's sign IS the travel sense, so the classification that
    // gates it degenerate also decides the winding — one funnel row.
    let winding = match geom_core::k_stats::decide("path_arc_bulge", geom_core::Margin::of(b), band)
    {
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
    fn carrier(&self, at: Point2<T>, tol: Tol) -> Result<PointCarrier<T>, PathError<T>> {
        let band = linear_band(tol)?;
        let chord = (self.p - at).norm_squared().sqrt();
        verbs::gate_positive("path_arc_chord", chord, band, |c| {
            PathError::DegenerateArcChord { chord: c }
        })?;
        let (c, w, start) = bulge_carrier(at, self.p, self.b, tol)?;
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
    fn carrier(&self, at: Point2<T>, tol: Tol) -> Result<PointCarrier<T>, PathError<T>> {
        let band = linear_band(tol)?;
        let chord_v = self.p - at;
        let chord = chord_v.norm_squared().sqrt();
        verbs::gate_positive("path_arc_chord", chord, band, |c| {
            PathError::DegenerateArcChord { chord: c }
        })?;
        // The collinear gate, then the existing closed form — the sharp
        // `Via` leg mode's own derivation, verbatim.
        let offset = chord_v.perp_dot(self.q - at) / chord;
        match geom_core::k_stats::decide("path_arc_via_offset", geom_core::Margin::of(offset), band)
        {
            Ok(geom_core::Sign::Zero) => return Err(PathError::ArcViaCollinear { offset }),
            Ok(_) => {}
            Err(source) => return Err(PathError::Escalated { source }),
        }
        let b = crate::sugar::bulge_from_via(at, self.q, self.p);
        let (c, w, start) = bulge_carrier(at, self.p, b, tol)?;
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
    fn carrier(&self, at: Point2<T>, tol: Tol) -> Result<PointCarrier<T>, PathError<T>> {
        let band = linear_band(tol)?;
        // The sharp `Center` leg mode's gates: both radii definitely
        // positive, equidistance definitely zero, chord non-degenerate.
        let r_tip = (at - self.c).norm_squared().sqrt();
        let r_end = (self.p - self.c).norm_squared().sqrt();
        for radius in [r_tip, r_end] {
            verbs::gate_positive("path_arc_center_radius", radius, band, |r| {
                PathError::DegenerateArcCenter { radius: r }
            })?;
        }
        match geom_core::k_stats::decide(
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

/// A fused verb's INCOMING spec from a DIRECTED POINT (leg end): the
/// endpoint-full modes exactly as from any point tip, PLUS
/// `Radius { r, side }` — **arc extension**, the arc analog of ray
/// extension: the carrier is DERIVED from the tip's two binding bits
/// (centre = at + side·r·n̂(tangent)), so tangency at the tip holds by
/// construction and nothing is value-matched, and the incoming side's
/// anchor is the tip itself. When the derived carrier IS the tip's own
/// incoming carrier (the chain-side `Incoming.carrier` bookkeeping
/// decides, on the §4 item 4 identity margin) the incoming run extends
/// the arriving leg — its end vertex MOVES to the trim point (the §4
/// exemption, exactly as ray extension); a different carrier emits its
/// run from the tip with a constructed tangency there.
///
/// `Center` here stays the anchored mode (its derived START tangent is
/// junction-checked against the incoming): restating the tip's own
/// carrier through an authored centre would land in the junction
/// check's tangent band and refuse — declared tangency is CONSTRUCTED
/// (`Radius`), never value-matched (§2c round 6).
pub trait LegEndIncoming<T: ArcCarrierScalar> {
    #[doc(hidden)]
    fn incoming(&self, dp: DirectedPoint<T>, tol: Tol) -> Result<FusedIncoming<T>, PathError<T>>;
    #[doc(hidden)]
    fn to_wire(&self, tol: Tol) -> ArcData<T>;
}

/// What a directed-point fused incoming resolves to (see
/// [`LegEndIncoming`]).
#[doc(hidden)]
pub enum FusedIncoming<T: Real> {
    /// An endpoint-full mode's authored side: carrier + derived start
    /// tangent + authored anchor (junction-checked at the tip).
    Anchored(PointCarrier<T>),
    /// Arc extension: the carrier derived at the tip (centre, winding);
    /// the anchor is the tip itself.
    FromTip(Point2<T>, crate::sugar::ArcSweep),
}

impl<T: ArcCarrierScalar> LegEndIncoming<T> for verbs::Bulge<T, Point2<T>> {
    fn incoming(&self, dp: DirectedPoint<T>, tol: Tol) -> Result<FusedIncoming<T>, PathError<T>> {
        Ok(FusedIncoming::Anchored(PointIncoming::carrier(
            self, dp.at, tol,
        )?))
    }
    fn to_wire(&self, _tol: Tol) -> ArcData<T> {
        PointIncoming::to_wire(self)
    }
}

impl<T: ArcCarrierScalar> LegEndIncoming<T> for Via<T, Point2<T>> {
    fn incoming(&self, dp: DirectedPoint<T>, tol: Tol) -> Result<FusedIncoming<T>, PathError<T>> {
        Ok(FusedIncoming::Anchored(PointIncoming::carrier(
            self, dp.at, tol,
        )?))
    }
    fn to_wire(&self, _tol: Tol) -> ArcData<T> {
        PointIncoming::to_wire(self)
    }
}

impl<T: ArcCarrierScalar> LegEndIncoming<T> for Center<T, Point2<T>> {
    fn incoming(&self, dp: DirectedPoint<T>, tol: Tol) -> Result<FusedIncoming<T>, PathError<T>> {
        Ok(FusedIncoming::Anchored(PointIncoming::carrier(
            self, dp.at, tol,
        )?))
    }
    fn to_wire(&self, _tol: Tol) -> ArcData<T> {
        PointIncoming::to_wire(self)
    }
}

impl<T: ArcCarrierScalar> LegEndIncoming<T> for Radius<T> {
    fn incoming(&self, dp: DirectedPoint<T>, tol: Tol) -> Result<FusedIncoming<T>, PathError<T>> {
        let (centre, winding) = verbs::radius_carrier(dp, *self, linear_band(tol)?)?;
        Ok(FusedIncoming::FromTip(centre, winding))
    }
    fn to_wire(&self, _tol: Tol) -> ArcData<T> {
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
    /// The kernel behind the table's entry fused-arc row: the row
    /// records the step, the kernel constructs the side.
    pub(super) fn arc_fillet_kernel<T: ArcCarrierScalar>(
        self,
        step: Step<T>,
        spec: Center<T, Point2<T>>,
        radius: T,
        tol: Tol,
    ) -> Result<PartialPath<T, NoPos, NoAng>, PathError<T>> {
        let mut core = Core::empty();
        core.record(step);
        entry_arc_open(&mut core, &spec, radius, tol)?;
        Ok(in_state(
            core,
            Tip {
                pos: None,
                ang: None,
                ang_by_tangent: false,
            },
        ))
    }

    /// The kernel behind the table's entry fused-arc/arc-arrival row:
    /// the row records the step, the kernel constructs the side.
    pub(super) fn arc_fillet_arc_kernel<T: ArcCarrierScalar, S2: ArrivalSpec<T>>(
        self,
        step: Step<T>,
        spec: Center<T, Point2<T>>,
        radius: T,
        spec2: S2,
        tol: Tol,
    ) -> S2::Out {
        let mut core = Core::empty();
        core.record(step);
        if let Err(e) = entry_arc_open(&mut core, &spec, radius, tol) {
            return S2::fail(e);
        }
        S2::apply(core, spec2, tol)
    }
}

/// The entry fused verbs' shared incoming half: seed the chain at the
/// spec's anchor, bind the entry direction to the carrier tangent
/// there, open the arc-incoming fillet.
fn entry_arc_open<T: ArcCarrierScalar>(
    core: &mut Core<T>,
    spec: &Center<T, Point2<T>>,
    radius: T,
    tol: Tol,
) -> Result<(), PathError<T>> {
    let band = linear_band(tol)?;
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
        tol,
    )
}

impl<T: ArcCarrierScalar, F: Flavor> PartialPath<T, HasPos<F>, HasAng> {
    /// The kernel behind the table's line-incoming/arc-arrival row
    /// (recording is the row's, not the kernel's).
    pub(super) fn fillet_arc_kernel<S: ArrivalSpec<T>>(
        mut self,
        radius: T,
        spec: S,
        tol: Tol,
    ) -> S::Out {
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
            tol,
        ) {
            return S::fail(e);
        }
        S::apply(self.core, spec, tol)
    }

    /// The kernel behind the table's tangent-departing fused row
    /// (recording is the row's, not the kernel's).
    pub(super) fn arc_fillet_kernel<S: TangentIncoming<T>>(
        mut self,
        spec: S,
        radius: T,
        tol: Tol,
    ) -> Result<PartialPath<T, NoPos, NoAng>, PathError<T>> {
        self.tangent_arc_open(&spec, radius, tol)?;
        Ok(in_state(
            self.core,
            Tip {
                pos: None,
                ang: None,
                ang_by_tangent: false,
            },
        ))
    }

    /// The kernel behind the table's tangent-departing fused/arrival
    /// row (recording is the row's, not the kernel's).
    pub(super) fn arc_fillet_arc_kernel<Si: TangentIncoming<T>, S2: ArrivalSpec<T>>(
        mut self,
        spec: Si,
        radius: T,
        spec2: S2,
        tol: Tol,
    ) -> S2::Out {
        if let Err(e) = self.tangent_arc_open(&spec, radius, tol) {
            return S2::fail(e);
        }
        S2::apply(self.core, spec2, tol)
    }

    /// The tangent-departing fused incoming: derive the leg, run the §4
    /// item 4 identity check against an inherited departure's carrier,
    /// and open the arc-incoming fillet. Nothing is emitted here — the
    /// trimmed run is the resolution's emission, from the chain's head.
    fn tangent_arc_open<S: TangentIncoming<T>>(
        &mut self,
        spec: &S,
        radius: T,
        tol: Tol,
    ) -> Result<(), PathError<T>> {
        let (at, ang) = self.dep()?;
        let leg = spec.leg(DirectedPoint { at, dir: ang }, tol)?;
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
                tol,
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
            tol,
        )
    }

    /// The kernel behind the table's endpoint-free sharp-leg row
    /// (recording is the row's, not the kernel's).
    pub(super) fn arc_to_kernel<S: TangentIncoming<T>>(
        mut self,
        spec: S,
        tol: Tol,
    ) -> Result<PartialPath<T, HasPos<WithIncoming>, NoAng>, PathError<T>> {
        let (at, ang) = self.dep()?;
        let leg = spec.leg(DirectedPoint { at, dir: ang }, tol)?;
        let carrier = SegArc {
            center: leg.centre,
            radius: (at - leg.centre).norm_squared().sqrt(),
        };
        if self.tip.ang_by_tangent
            && let Some(inc) = self.tip.pos.as_ref().and_then(|pd| pd.incoming.as_ref())
            && let Some(prev) = &inc.carrier
        {
            super::refuse_identical_carriers(prev, &carrier, tol)?;
        }
        self.core.push_arc(leg.end, leg.bulge, carrier)?;
        let arm = carrier.radius.min(leg.chord);
        Ok(in_state(
            self.core,
            leg_end_tip(leg.end, leg.end_dir, arm, Some(carrier)),
        ))
    }
}

impl<T: ArcCarrierScalar> PartialPath<T, HasPos<Plain>, NoAng> {
    /// The kernel behind the table's plain-point fused row (recording
    /// is the row's, not the kernel's).
    pub(super) fn arc_fillet_kernel<S: PointIncoming<T>>(
        mut self,
        spec: S,
        radius: T,
        tol: Tol,
    ) -> Result<PartialPath<T, NoPos, NoAng>, PathError<T>> {
        self.point_arc_open(&spec, radius, tol)?;
        Ok(in_state(
            self.core,
            Tip {
                pos: None,
                ang: None,
                ang_by_tangent: false,
            },
        ))
    }

    /// The kernel behind the table's plain-point fused/arrival row
    /// (recording is the row's, not the kernel's).
    pub(super) fn arc_fillet_arc_kernel<Si: PointIncoming<T>, S2: ArrivalSpec<T>>(
        mut self,
        spec: Si,
        radius: T,
        spec2: S2,
        tol: Tol,
    ) -> S2::Out {
        if let Err(e) = self.point_arc_open(&spec, radius, tol) {
            return S2::fail(e);
        }
        S2::apply(self.core, spec2, tol)
    }

    fn point_arc_open<S: PointIncoming<T>>(
        &mut self,
        spec: &S,
        radius: T,
        tol: Tol,
    ) -> Result<(), PathError<T>> {
        let pos = self.tip.pos.as_ref().ok_or(PathError::UnderdeterminedLeg {
            site: "fused arc incoming on a tip without a position",
        })?;
        let at = pos.at;
        let (centre, winding, start, anchor) = spec.carrier(at, tol)?;
        // A Plain point carries no incoming tangent, so there is no
        // junction to check here — the directed-point flavor runs it in
        // `leg_end_arc_open`. What a Plain tip CAN be is the entry
        // (`Open.at(p)`), whose departure direction is bound now.
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
            tol,
        )
    }
}

impl<T: ArcCarrierScalar> PartialPath<T, HasPos<WithIncoming>, NoAng> {
    /// The kernel behind the table's ray-extension fillet row (recording
    /// is the row's, not the kernel's).
    pub(super) fn fillet_kernel(
        mut self,
        radius: T,
        tol: Tol,
    ) -> Result<PartialPath<T, NoPos, NoAng>, PathError<T>> {
        self.ray_extend(radius, tol)?;
        Ok(in_state(
            self.core,
            Tip {
                pos: None,
                ang: None,
                ang_by_tangent: false,
            },
        ))
    }

    /// The kernel behind the table's leg-end ray-extension/arrival row
    /// (recording is the row's, not the kernel's).
    pub(super) fn fillet_arc_kernel<S: ArrivalSpec<T>>(
        mut self,
        radius: T,
        spec: S,
        tol: Tol,
    ) -> S::Out {
        if let Err(e) = self.ray_extend(radius, tol) {
            return S::fail(e);
        }
        S::apply(self.core, spec, tol)
    }

    /// The shared ray-extension opening: inherit the incoming end
    /// tangent, declare the (constructed) tangency at the leg end, and
    /// open the ray-incoming fillet there — `.tangent().fillet(r)`'s
    /// exact emissions, in one verb.
    fn ray_extend(&mut self, radius: T, tol: Tol) -> Result<(), PathError<T>> {
        let pos = self.tip.pos.as_ref().ok_or(PathError::UnderdeterminedLeg {
            site: "ray extension on a tip without a position",
        })?;
        let at = pos.at;
        let inc = pos.incoming.ok_or(PathError::UnderdeterminedLeg {
            site: "ray extension on a tip without incoming data",
        })?;
        self.core.declare_last();
        open_ray(&mut self.core, at, inc.ang, radius, true, Some(inc), tol)
    }

    /// The kernel behind the table's leg-end fused row (recording is
    /// the row's, not the kernel's).
    pub(super) fn arc_fillet_kernel<S: LegEndIncoming<T>>(
        mut self,
        spec: S,
        radius: T,
        tol: Tol,
    ) -> Result<PartialPath<T, NoPos, NoAng>, PathError<T>> {
        self.leg_end_arc_open(&spec, radius, tol)?;
        Ok(in_state(
            self.core,
            Tip {
                pos: None,
                ang: None,
                ang_by_tangent: false,
            },
        ))
    }

    /// The kernel behind the table's leg-end fused/arrival row
    /// (recording is the row's, not the kernel's).
    pub(super) fn arc_fillet_arc_kernel<Si: LegEndIncoming<T>, S2: ArrivalSpec<T>>(
        mut self,
        spec: Si,
        radius: T,
        spec2: S2,
        tol: Tol,
    ) -> S2::Out {
        if let Err(e) = self.leg_end_arc_open(&spec, radius, tol) {
            return S2::fail(e);
        }
        S2::apply(self.core, spec2, tol)
    }

    /// The directed-point fused opening: an anchored mode is
    /// junction-checked at the tip and opens exactly as from a plain
    /// point; `Radius` derives its carrier from the tip's binding bits
    /// and the chain decides between EXTENDING the arriving leg (the
    /// derived carrier IS its carrier — the incoming emission moves the
    /// leg's end vertex) and a NEW tangent carrier constructed at the
    /// tip (the joint there is declared, exactly as ray extension
    /// declares its origin).
    fn leg_end_arc_open<S: LegEndIncoming<T>>(
        &mut self,
        spec: &S,
        radius: T,
        tol: Tol,
    ) -> Result<(), PathError<T>> {
        let pos = self.tip.pos.as_ref().ok_or(PathError::UnderdeterminedLeg {
            site: "fused arc incoming on a tip without a position",
        })?;
        let at = pos.at;
        let inc = pos.incoming.ok_or(PathError::UnderdeterminedLeg {
            site: "fused arc incoming on a leg end without incoming data",
        })?;
        // No `start_ang` seeding here, deliberately: a WithIncoming tip
        // exists only downstream of a leg, and every leg's chain bound
        // its entry direction before emitting — the Plain entry path
        // (`point_arc_open`) is where seeding lives.
        match spec.incoming(DirectedPoint { at, dir: inc.ang }, tol)? {
            FusedIncoming::Anchored((centre, winding, start, anchor)) => {
                junction_check(&inc, start, false, tol)?;
                open_arc(
                    &mut self.core,
                    PendingArc {
                        anchor,
                        centre,
                        winding,
                        radius,
                        resolver: arc_fillet::resolve::<T>,
                    },
                    tol,
                )
            }
            FusedIncoming::FromTip(centre, winding) => {
                let derived = SegArc {
                    center: centre,
                    radius: (at - centre).norm_squared().sqrt(),
                };
                let extends = match &inc.carrier {
                    Some(prev) => carriers_are_identical(prev, &derived, tol)?,
                    None => false,
                };
                if !extends {
                    // A new tangent carrier CONSTRUCTED at the tip:
                    // both sides share the tip's tangent by
                    // construction, so the joint is a real tangency.
                    self.core.declare_last();
                }
                open_arc_from_tip(
                    &mut self.core,
                    PendingArc {
                        anchor: at,
                        centre,
                        winding,
                        radius,
                        resolver: arc_fillet::resolve::<T>,
                    },
                    extends,
                    Some(inc),
                    tol,
                )
            }
        }
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
    fn leg_from(path: PartialPath<T, HasPos<F>, NoAng>, spec: Self, tol: Tol) -> Self::Out;
}

impl<T, Tgt> super::sealed::Sealed for verbs::Bulge<T, Tgt> {}
impl<T: Real, Tgt> super::sealed::Sealed for Via<T, Tgt> {}
impl<T: Real, Tgt> super::sealed::Sealed for Center<T, Tgt> {}

impl<T: geom_core::Decide, F: Flavor> PointLeg<T, F> for verbs::Bulge<T, Point2<T>> {
    type Out = Result<PartialPath<T, HasPos<WithIncoming>, NoAng>, PathError<T>>;
    fn leg_from(mut path: PartialPath<T, HasPos<F>, NoAng>, spec: Self, tol: Tol) -> Self::Out {
        path.core.record(Step::ArcTo(ArcData::Bulge {
            target: Target::Point(spec.p),
            b: spec.b,
        }));
        path.arc_to_point(spec.p, spec.b, tol)
    }
}

impl<T: geom_core::Decide, F: Flavor> PointLeg<T, F> for verbs::Bulge<T, Start> {
    type Out = Result<ClosedLoop<T>, PathError<T>>;
    fn leg_from(mut path: PartialPath<T, HasPos<F>, NoAng>, spec: Self, tol: Tol) -> Self::Out {
        path.core.record(Step::ArcTo(ArcData::Bulge {
            target: Target::Start,
            b: spec.b,
        }));
        path.arc_to_start(spec.b, tol)
    }
}

impl<T: geom_core::Decide, F: Flavor> PointLeg<T, F> for Via<T, Point2<T>> {
    type Out = Result<PartialPath<T, HasPos<WithIncoming>, NoAng>, PathError<T>>;
    fn leg_from(mut path: PartialPath<T, HasPos<F>, NoAng>, spec: Self, tol: Tol) -> Self::Out {
        path.core.record(Step::ArcTo(ArcData::Via {
            q: spec.q,
            target: Target::Point(spec.p),
        }));
        let bulge = path.arc_via_bulge(spec.q, spec.p, tol)?;
        path.arc_to_point(spec.p, bulge, tol)
    }
}

impl<T: geom_core::Decide, F: Flavor> PointLeg<T, F> for Via<T, Start> {
    type Out = Result<ClosedLoop<T>, PathError<T>>;
    fn leg_from(mut path: PartialPath<T, HasPos<F>, NoAng>, spec: Self, tol: Tol) -> Self::Out {
        path.core.record(Step::ArcTo(ArcData::Via {
            q: spec.q,
            target: Target::Start,
        }));
        let bulge = path.arc_via_bulge(spec.q, path.start_target()?, tol)?;
        path.arc_to_start(bulge, tol)
    }
}

impl<T: geom_core::Decide, F: Flavor> PointLeg<T, F> for Center<T, Point2<T>> {
    type Out = Result<PartialPath<T, HasPos<WithIncoming>, NoAng>, PathError<T>>;
    fn leg_from(mut path: PartialPath<T, HasPos<F>, NoAng>, spec: Self, tol: Tol) -> Self::Out {
        path.core.record(Step::ArcTo(ArcData::Center {
            c: spec.c,
            winding: spec.winding,
            target: Target::Point(spec.p),
        }));
        let bulge = path.arc_center_bulge(spec.c, spec.p, spec.winding, tol)?;
        path.arc_to_point(spec.p, bulge, tol)
    }
}

impl<T: geom_core::Decide, F: Flavor> PointLeg<T, F> for Center<T, Start> {
    type Out = Result<ClosedLoop<T>, PathError<T>>;
    fn leg_from(mut path: PartialPath<T, HasPos<F>, NoAng>, spec: Self, tol: Tol) -> Self::Out {
        path.core.record(Step::ArcTo(ArcData::Center {
            c: spec.c,
            winding: spec.winding,
            target: Target::Start,
        }));
        let bulge = path.arc_center_bulge(spec.c, path.start_target()?, spec.winding, tol)?;
        path.arc_to_start(bulge, tol)
    }
}

impl<T: geom_core::Decide, F: Flavor> PartialPath<T, HasPos<F>, NoAng> {}
