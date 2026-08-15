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
//! | `Center{c,w,p}` | Point | Entry, Point, OnArc | complete (resolves at the verb; `p: Start` closes) |
//! | `Radius{r,side}` | — | OnArc (centre re-derived) | Directed anchor (binders pending) |
//! | `Sweep{r,side,angle}` | Directed | Directed | — |
//! | `ArcLen{r,side,len}` | Directed | Directed | — |
//!
//! "Point" legs/incomings ride the retired-name doors
//! (`arc_to(target, bulge)` / `arc_via` / `arc_center`) until the
//! consumer re-spell renames them onto `arc_to(spec)`; they already
//! record the unified [`ArcData`](super::program::ArcData) steps.

use geom_core::{Point2, Real, Sign, Tolerance};

use super::arc_fillet::{self, ArcCarrierScalar, carrier_tangent};
use super::program::{ArcData, ClosedLoop, Step, Target};
use super::verbs::{
    self, ArcLen, ArcSide, Center, DirectedPoint, Pending, PendingArc, Radius, Sweep, Via,
};
use super::{
    ArcData as SegArc, Core, Dir, FirstSeg, Flavor, HasAng, HasPos, Incoming, NoAng, NoPos, Open,
    PartialPath, PathError, PendingMeta, Plain, Start, Tip, WithIncoming, in_state,
    junction_check, leg_end_tip, linear_band,
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
// Shared plumbing (pub(super) so the compat shim drives the same code).
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
    core.pending = Some(Pending::Ray(verbs::PendingRay {
        origin: at,
        dir,
        radius,
    }));
    core.pending_meta = Some(PendingMeta {
        by_tangent,
        origin_incoming,
        compat_carrier: None,
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
    core.pending = Some(Pending::Arc(arc));
    core.pending_meta = Some(PendingMeta {
        by_tangent: false,
        origin_incoming: None,
        compat_carrier: None,
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
    let (pending, _meta) = core.take_pending("arc-carrier fillet arrival without an opened fillet")?;
    let trims = resolver(
        pending.side(),
        arc_fillet::FilletSide {
            anchor,
            carrier: arc_fillet::SideCarrier::Circle { centre, winding },
        },
        pending.radius(),
        Tolerance::get(),
    )?;
    core.emit_fillet_in(&trims)?;
    // The continuation rides the arrival carrier from `t2` by
    // construction, so the joint there IS a constructed tangency.
    core.emit_fillet_arc(&trims, true)?;
    Ok(dir)
}

/// Resolves the open fillet against the ARC ARRIVAL that CLOSES at the
/// entry (`p: Start` — the retired `to_on`'s semantics, verbatim): the
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
    let (pending, _meta) = core.take_pending("arc-carrier close without an opened fillet")?;
    let trims = resolver(
        pending.side(),
        arc_fillet::FilletSide {
            anchor: start_pos,
            carrier: arc_fillet::SideCarrier::Circle { centre, winding },
        },
        pending.radius(),
        Tolerance::get(),
    )?;
    core.emit_fillet_in(&trims)?;
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
                carrier: Some(SegArc { center: centre, radius }),
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
    fn to_wire(&self) -> ArcData<T>;
}

/// `Center { c, winding, p }` with an INTERIOR anchor: complete at the
/// verb (the anchor and the derived direction are one authored act —
/// the retired `at_on` arrival, fused). The tip continues ON the
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
    fn to_wire(&self) -> ArcData<T> {
        ArcData::Center {
            c: self.c,
            winding: self.winding,
            target: Target::Point(self.p),
        }
    }
}

/// `Center { c, winding, p: Start }`: the arc-arrival CLOSE (the
/// retired `to_on`, fused): the entry vertex is KEPT as a genuine
/// two-carrier junction and the seam junction check runs there.
impl<T: ArcCarrierScalar> ArrivalSpec<T> for Center<T, Start> {
    type Out = Result<ClosedLoop<T>, PathError<T>>;
    fn apply(mut core: Core<T>, spec: Self) -> Self::Out {
        let Start = spec.p;
        resolve_arc_close(&mut core, arc_fillet::resolve::<T>, spec.c, spec.winding)
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
    type Out = RadiusArrival<T>;
    fn apply(core: Core<T>, spec: Self) -> Self::Out {
        RadiusArrival {
            core,
            spec,
            resolver: arc_fillet::resolve::<T>,
        }
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
    type Out = ViaArrival<T>;
    fn apply(core: Core<T>, spec: Self) -> Self::Out {
        ViaArrival {
            core,
            q: spec.q,
            p: spec.p,
            resolver: arc_fillet::resolve::<T>,
        }
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
    type Out = ViaArrivalStart<T>;
    fn apply(core: Core<T>, spec: Self) -> Self::Out {
        ViaArrivalStart {
            core,
            q: spec.q,
            resolver: arc_fillet::resolve::<T>,
        }
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
        via_complete(self.core, self.q, self.p, Dir::from_angle(theta), self.resolver)
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
    let (centre, winding) = verbs::via_carrier(
        DirectedPoint {
            at: start_pos,
            dir,
        },
        q,
        band,
    )?;
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
    fn carrier(
        &self,
        at: Point2<T>,
    ) -> Result<(Point2<T>, crate::sugar::ArcSweep, Dir<T>, Point2<T>), PathError<T>>;
    #[doc(hidden)]
    fn to_wire(&self) -> ArcData<T>;
}

/// The shared Bulge-shaped derivation: carrier from chord + bulge (the
/// existing closed form), winding from the bulge's sign, start tangent
/// γ − θ/2 (the M2 convention, exactly the sharp legs' derivation).
fn bulge_carrier<T: geom_core::Decide>(
    at: Point2<T>,
    p: Point2<T>,
    b: T,
) -> (Point2<T>, crate::sugar::ArcSweep, Dir<T>) {
    let data = super::arc_carrier(at, p, b);
    let winding = if b > T::zero() {
        crate::sugar::ArcSweep::Ccw
    } else {
        crate::sugar::ArcSweep::Cw
    };
    let d = p - at;
    let gamma = d.y.atan2(d.x);
    let theta = b.atan() * T::from_f64(4.0);
    let start = Dir::from_angle(gamma - theta / T::from_f64(2.0));
    (data.center, winding, start)
}

impl<T: ArcCarrierScalar> PointIncoming<T> for verbs::Bulge<T, Point2<T>> {
    fn carrier(
        &self,
        at: Point2<T>,
    ) -> Result<(Point2<T>, crate::sugar::ArcSweep, Dir<T>, Point2<T>), PathError<T>> {
        let band = linear_band()?;
        let chord = (self.p - at).norm_squared().sqrt();
        verbs::gate_positive("path_arc_chord", chord, band, |c| {
            PathError::DegenerateArcChord { chord: c }
        })?;
        let (c, w, start) = bulge_carrier(at, self.p, self.b);
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
    fn carrier(
        &self,
        at: Point2<T>,
    ) -> Result<(Point2<T>, crate::sugar::ArcSweep, Dir<T>, Point2<T>), PathError<T>> {
        let band = linear_band()?;
        let chord_v = self.p - at;
        let chord = chord_v.norm_squared().sqrt();
        verbs::gate_positive("path_arc_chord", chord, band, |c| {
            PathError::DegenerateArcChord { chord: c }
        })?;
        // The collinear gate, then the existing closed form — the sharp
        // `arc_via` leg's own derivation, verbatim.
        let offset = chord_v.perp_dot(self.q - at) / chord;
        match crate::k_stats::decide(
            "path_arc_via_offset",
            geom_core::Margin::of(offset),
            band,
        ) {
            Ok(geom_core::Sign::Zero) => return Err(PathError::ArcViaCollinear { offset }),
            Ok(_) => {}
            Err(source) => return Err(PathError::Escalated { source }),
        }
        let b = crate::sugar::bulge_from_via(at, self.q, self.p);
        let (c, w, start) = bulge_carrier(at, self.p, b);
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
    fn carrier(
        &self,
        at: Point2<T>,
    ) -> Result<(Point2<T>, crate::sugar::ArcSweep, Dir<T>, Point2<T>), PathError<T>> {
        let band = linear_band()?;
        // The sharp `arc_center` leg's gates: both radii definitely
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
/// re-authors it — `Radius` re-derives the centre from the tip's own
/// binding bits; `Center` re-states the authored centre.
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

impl<T: ArcCarrierScalar> OnArcIncoming<T> for Center<T, Point2<T>> {
    fn side(
        &self,
        _dp: DirectedPoint<T>,
    ) -> Result<(Point2<T>, Point2<T>, crate::sugar::ArcSweep), PathError<T>> {
        Ok((self.p, self.c, self.winding))
    }
    fn to_wire(&self) -> ArcData<T> {
        ArcData::Center {
            c: self.c,
            winding: self.winding,
            target: Target::Point(self.p),
        }
    }
}
