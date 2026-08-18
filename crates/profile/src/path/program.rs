//! **Profiles-as-programs (v2) — the profile side.** The PATHS algebra's
//! authoring surface, recorded as data and replayed through a driver that
//! mirrors the typestate lattice at runtime.
//!
//! Three things live here:
//!
//! 1. [`Step`] — the step vocabulary: one variant per authoring verb,
//!    storing **authored data only**. `ArcVia`/`ArcCenter` keep the
//!    points the author wrote; their bulges are DERIVED at replay, by
//!    the same binders the typed surface calls. Storing a derived bulge
//!    would re-type a computed value as authored and kill its
//!    parametricity (PROFILES-V2 §V1/§V2).
//! 2. [`ClosedLoop`] — what a closing verb now returns: the lowered
//!    [`ProfileLoop`] *and* the program that produced it. One authoring
//!    surface, two consumers; no second spelling of any verb.
//! 3. [`replay`] — the driver. It holds the in-flight tip as [`DynTip`],
//!    an enum over the lattice states each of which carries the TYPED
//!    [`super::PartialPath`] value, and applying a step is a
//!    match on (state, verb) whose arm bodies can only call the ONE typed
//!    binder that is well-typed at that state.
//!
//! # What the construction makes unwritable — precisely
//!
//! The binder bodies are never duplicated here and the lattice is never
//! re-stated as data. The property that buys is SHARP, and worth stating
//! exactly rather than generously:
//!
//! **Unwritable:** calling a binder on the CARRIED VALUE of a state
//! where that binder is not well-typed. Every arm destructures the real
//! `PartialPath`, so `.tangent()` on a `PlainPoint` tip or a leg on an
//! `Angle` tip is a compile error, not a test failure. This is the whole
//! drift class the two-surface design is exposed to: an arm that
//! *continues the actual chain* through a transition the surface forbids
//! cannot be spelled.
//!
//! **Still writable, and NOT prevented by the types:** arms that IGNORE
//! the carried value — a no-op arm returning the tip unchanged under an
//! illegal verb, or a laundering arm that MINTS a fresh tip from the
//! step's own arguments and continues from that. Those compile. They are
//! deliberate authorship rather than drift (nobody writes one by
//! accident while adding a verb), and they are backstopped downstream,
//! not upstream: the no-op shape is caught by the refusal census
//! (`lattice_violations_refuse_as_the_transition_class`), the laundering
//! shape by review of this file — which is why every arm here is one
//! line of the form "destructure, call the one binder, re-wrap".
//!
//! What remains is the safe direction — a MISSING arm, i.e. an
//! over-strict refusal — and that is exactly what the differential pin
//! catches (`tests/path_program.rs`: every typed chain's recorded
//! program must replay to a bit-identical loop).
//!
//! # Serde plays no role
//!
//! The `profile` crate stays serde-free (G1 layering). The Expr-bearing,
//! wire-shaped step type is `editor-core`'s; this one is the resolved,
//! scalar-valued form the driver consumes.

use geom_core::{Decide, Point2, Real};

use super::{
    ArcCarrierScalar, Bulge, Center, Flavor, HasAng, HasPos, NoAng, NoPos, Open, PartialPath,
    PathError, Plain, Start, Via, WithIncoming,
};
use crate::ProfileLoop;
use crate::sugar::ArcSweep;

// ------------------------------------------------------------------
// The step vocabulary
// ------------------------------------------------------------------

/// Where a target-taking verb ends: an authored absolute point, or the
/// entry vertex ([`Start`]).
///
/// The distinction is STRUCTURAL — it is the verb's shape, never a
/// value — so it stays literal in the step when the continuous
/// arguments become expressions (PROFILES-V2 §V2). Targeting `Start` IS
/// closing, here exactly as in the typed surface.
#[derive(Clone, Copy, Debug)]
pub enum Target<T: Real> {
    /// An authored absolute point in the profile frame.
    Point(Point2<T>),
    /// The entry vertex: this step closes the loop.
    Start,
}

/// **The unified arc-spec record (§2c rounds 5–9)**: one enum, every
/// authored mode, exactly as the surface's standalone spec types
/// authored it (record-as-you-lower keeps the mode; the VQ contracts
/// rely on that distinctness). The typed surface consumes the
/// standalone types ([`Radius`](super::Radius), [`Bulge`], [`Via`],
/// [`Center`], [`Sweep`](super::Sweep), [`ArcLen`](super::ArcLen))
/// through the
/// state-keyed trait matrix; the wire and the replay driver match THIS
/// enum exhaustively, which is the round-9 forcing argument for the
/// whole family shipping at once.
#[derive(Clone, Copy, Debug)]
pub enum ArcData<T: Real> {
    /// `Radius { r, side }` — arrival mode: centre derived from the
    /// arrival's directed anchor.
    Radius {
        /// The carrier radius.
        r: T,
        /// Which side of the tangent the centre sits on.
        side: super::ArcSide,
    },
    /// `Bulge { p, b }` — chord-relative: leg targets and fused
    /// incoming specs.
    Bulge {
        /// The authored endpoint.
        target: Target<T>,
        /// The authored bulge (M2 convention).
        b: T,
    },
    /// `Via { q, p }` — the arc through an authored point.
    Via {
        /// The through-point.
        q: Point2<T>,
        /// The authored endpoint.
        target: Target<T>,
    },
    /// `Center { c, winding, p }` — the arc about an authored centre.
    Center {
        /// The carrier centre.
        c: Point2<T>,
        /// Travel sense (structural).
        winding: ArcSweep,
        /// The authored anchor/endpoint (`Start` closes).
        target: Target<T>,
    },
    /// `Sweep { r, side, angle }` — endpoint-free: tangent-departing,
    /// endpoint derived.
    Sweep {
        /// The carrier radius.
        r: T,
        /// Which side of the departure tangent the centre sits on.
        side: super::ArcSide,
        /// The swept central angle, radians.
        angle: T,
    },
    /// `ArcLen { r, side, len }` — endpoint-free, extent as arc length.
    ArcLen {
        /// The carrier radius.
        r: T,
        /// Which side of the departure tangent the centre sits on.
        side: super::ArcSide,
        /// The arc length, meters.
        len: T,
    },
}

/// **The step vocabulary and its projections, from ONE declaration**
/// (§2c rounds 13–15, lean (a)): this macro is the single place a verb
/// is declared, and it expands into the [`Step`] variant, the [`Verb`]
/// tag, and the [`Step::verb`] mapping — so a missing verb is missing
/// everywhere consistently, and an inconsistent pair is unwritable
/// because there is no second place to write it. The driver arms in
/// [`replay`]'s `apply` consume these variants; an arm the driver
/// lacks is the SAFE direction (an over-strict transition refusal),
/// which the differential suite pins (see the module docs).
macro_rules! step_vocabulary {
    ($( $(#[doc = $doc:literal])* $name:ident $({ $($(#[doc = $fdoc:literal])* $f:ident : $ft:ty),* $(,)? })? $(( $($tt:ty),* ))? ),* $(,)?) => {
        /// One recorded authoring verb (**authored data only** — every
        /// field is a number or structural tag the author wrote;
        /// derived quantities are re-derived at replay by the same
        /// binders the typed surface calls).
        #[derive(Clone, Copy, Debug)]
        pub enum Step<T: Real> {
            $( $(#[doc = $doc])* $name $({ $($(#[doc = $fdoc])* $f : $ft),* })? $(( $($tt),* ))? ),*
        }

        /// Which verb a step names — the `verb` half of a
        /// [`ReplayErrorKind::Transition`]. One value per [`Step`]
        /// variant, projected from the same declaration.
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub enum Verb {
            $( $(#[doc = $doc])* $name ),*
        }

        impl<T: Real> Step<T> {
            /// The verb this step names.
            pub fn verb(&self) -> Verb {
                match self {
                    $( Step::$name { .. } => Verb::$name ),*
                }
            }
        }
    };
}

step_vocabulary! {
    #[doc = " `.at(p)` — bind the position bit."]
    At(Point2<T>),
    #[doc = " `.angle(θ)` — bind the outgoing direction as an angle (radians)."]
    Angle(T),
    #[doc = " `.toward(dx, dy)` — bind it as exact components (ratio-only)."]
    Toward {
        #[doc = " x component."]
        dx: T,
        #[doc = " y component."]
        dy: T,
    },
    #[doc = " `.tangent()` — inherit the incoming end tangent and DECLARE the joint."]
    Tangent,
    #[doc = " `.turn(δ)` — depart at the incoming tangent rotated by δ."]
    Turn(T),
    #[doc = " `line(len)` — a straight leg along the bound direction."]
    Line(T),
    #[doc = " `line_to(target)` — a straight leg to the target."]
    LineTo(Target<T>),
    #[doc = " `arc_to(spec)` — the sharp arc leg, every mode in the one"]
    #[doc = " unified [`ArcData`] record; the mode the author wrote is"]
    #[doc = " what is kept, because the VQ contracts rely on it."]
    ArcTo(ArcData<T>),
    #[doc = " `tangent_arc_to(target)` — the unique tangent arc to the target."]
    TangentArcTo(Target<T>),
    #[doc = " `arc_continue(target)` — the declared-subdivision step."]
    ArcContinue(Point2<T>),
    #[doc = " `.fillet(r)` — line incoming (the tangent ray), line arrival."]
    Fillet {
        #[doc = " The fillet radius."]
        radius: T,
    },
    #[doc = " `fillet_arc(r, spec)` — line incoming, ARC arrival per the spec."]
    FilletArc {
        #[doc = " The fillet radius."]
        radius: T,
        #[doc = " The arc-arrival spec."]
        spec: ArcData<T>,
    },
    #[doc = " `arc_fillet(spec, r)` — fused ARC incoming, line arrival."]
    ArcFillet {
        #[doc = " The fused incoming-arc spec."]
        spec: ArcData<T>,
        #[doc = " The fillet radius."]
        radius: T,
    },
    #[doc = " `arc_fillet_arc(spec, r, spec₂)` — fused arc incoming, arc arrival."]
    ArcFilletArc {
        #[doc = " The fused incoming-arc spec."]
        spec: ArcData<T>,
        #[doc = " The fillet radius."]
        radius: T,
        #[doc = " The arc-arrival spec."]
        spec2: ArcData<T>,
    },
    #[doc = " `.to(anchor)` — the far-end anchor: end the arrival side there."]
    FarEndTo(Point2<T>),
    #[doc = " `.to(Start)` — the seam-fillet close (entry vertex retrimmed)."]
    CloseTo,
    #[doc = " `circle(centre, r)` — the one-step complete-loop program form."]
    Circle {
        #[doc = " The circle's centre."]
        centre: Point2<T>,
        #[doc = " The circle's radius (definitely positive)."]
        radius: T,
    },
    #[doc = " `circle_split(centre, r, n, phase)` — the declared-subdivision closed carrier."]
    CircleSplit {
        #[doc = " The carrier's centre."]
        centre: Point2<T>,
        #[doc = " The carrier's radius (definitely positive)."]
        radius: T,
        #[doc = " The subdivision count (STRUCTURAL, ≥ 2)."]
        n: usize,
        #[doc = " The first vertex's angle from +x (continuous)."]
        phase: T,
    },
}

/// A closing verb's result: the lowered loop AND the program that
/// produced it.
///
/// Closing verbs used to return [`ProfileLoop`] directly; they return
/// this pair now so that one chain yields both consumers' values
/// (PROFILES-V2 §V1: "one authoring surface, two consumers"). Kernel-
/// direct call sites that only want the geometry adapt with
/// [`From`]/`.into()` or by reading [`ClosedLoop::loop_`].
#[derive(Clone, Debug)]
pub struct ClosedLoop<T: Real> {
    /// The lowered loop — bit-for-bit what the closing verb returned
    /// before this pair existed.
    pub loop_: ProfileLoop<T>,
    /// The recorded program: replaying it reproduces `loop_` exactly.
    pub program: Vec<Step<T>>,
}

impl<T: Real> From<ClosedLoop<T>> for ProfileLoop<T> {
    fn from(closed: ClosedLoop<T>) -> Self {
        closed.loop_
    }
}

// ------------------------------------------------------------------
// Replay refusals
// ------------------------------------------------------------------

/// Which lattice state the driver's tip was in — the `state` half of a
/// [`ReplayErrorKind::Transition`], and the vocabulary the driver's
/// match is written over: the four lattice states (with the marker
/// flavors the surface distinguishes), the §2c arc-arrival states, and
/// the two ends of a run.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TipState {
    /// Before the first verb — the `Open` value itself.
    Entry,
    /// `Open` = {}: a fillet's freshly opened LINE arrival side.
    Open,
    /// `Angle` = {angle}: direction bound, position pending.
    Angle,
    /// `Point` = {position}, plain flavor: no incoming carrier.
    PlainPoint,
    /// `Point` = {position}, directed flavor: a leg end, incoming
    /// tangent available.
    DirectedPoint,
    /// `Directed` = {both}, over a plain position.
    DirectedPlain,
    /// `Directed` = {both}, over a leg-end position.
    DirectedIncoming,
    /// **§2c**: a `Radius` arrival awaiting both binders.
    RadiusArrival,
    /// **§2c**: a `Radius` arrival with its anchor bound.
    RadiusArrivalAt,
    /// **§2c**: a `Radius` arrival with its director bound.
    RadiusArrivalDir,
    /// **§2c**: a `Via` arrival awaiting its director.
    ViaArrival,
    /// **§2c**: a `Via` CLOSE awaiting its director.
    ViaArrivalStart,
    /// The loop is closed; no verb may follow.
    Closed,
}

/// Why a replay refused, and at which step.
///
/// The `step` index is into the program slice; for a program that ENDS
/// without closing it is the length (one past the last step), and the
/// kind's `verb` is then `None`.
#[derive(Clone, Debug)]
pub struct ReplayError<T: Real> {
    /// The index of the offending step.
    pub step: usize,
    /// The refusal class.
    pub kind: ReplayErrorKind<T>,
}

/// The two classes of replay refusal, deliberately different
/// (PROFILES-V2 §V1).
#[derive(Clone, Debug)]
pub enum ReplayErrorKind<T: Real> {
    /// **Lattice violation** — the step's verb (or its spec MODE, for
    /// the ArcData-carrying steps: an inadmissible (state, mode) pair
    /// is unrepresentable at the typed surface, so at the wire it is
    /// this class) is not well-typed at the tip's state. No authoring
    /// surface can produce this: it is the corrupt-or-hand-edited-file
    /// class, refused typed at the door.
    ///
    /// `verb` is `None` when the program simply ENDED in `state`
    /// without a closing verb.
    Transition {
        /// The tip's lattice state.
        state: TipState,
        /// The verb that is ill-typed there, or `None` for
        /// end-of-program.
        verb: Option<Verb>,
    },
    /// **Geometry refusal** — the chain is well-typed but the geometry
    /// refuses. This class CAN exist at rest: once arguments carry
    /// expressions, whether a program elaborates depends on the
    /// parameter binding.
    Path(PathError<T>),
}

impl<T: Real> ReplayError<T> {
    fn transition(step: usize, state: TipState, verb: Verb) -> Self {
        Self {
            step,
            kind: ReplayErrorKind::Transition {
                state,
                verb: Some(verb),
            },
        }
    }
}

impl<T: Real> core::fmt::Display for ReplayError<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match &self.kind {
            ReplayErrorKind::Transition {
                state,
                verb: Some(verb),
            } => write!(
                f,
                "step {}: {verb:?} is not a legal continuation of a {state:?} tip \
                 (lattice violation — no authoring surface can produce this program)",
                self.step
            ),
            ReplayErrorKind::Transition { state, verb: None } => write!(
                f,
                "step {}: the program ends at a {state:?} tip without closing the loop \
                 (lattice violation — a chain must end at Start)",
                self.step
            ),
            ReplayErrorKind::Path(source) => {
                write!(f, "step {}: {source}", self.step)
            }
        }
    }
}

impl<T: Real> std::error::Error for ReplayError<T> {}

// ------------------------------------------------------------------
// The replay driver
// ------------------------------------------------------------------

impl<T: Real> From<PathError<T>> for ReplayErrorKind<T> {
    fn from(source: PathError<T>) -> Self {
        ReplayErrorKind::Path(source)
    }
}

/// The in-flight tip: one variant per lattice state, each carrying the
/// TYPED value for that state — a match arm on (variant, verb) can only
/// call a binder that is well-typed at that state, so an illegal
/// transition cannot be spelled and no binder body is duplicated.
enum DynTip<T: Real> {
    Entry,
    Open(PartialPath<T, NoPos, NoAng>),
    Angle(PartialPath<T, NoPos, HasAng>),
    PlainPoint(PartialPath<T, HasPos<Plain>, NoAng>),
    DirectedPoint(PartialPath<T, HasPos<WithIncoming>, NoAng>),
    DirectedPlain(PartialPath<T, HasPos<Plain>, HasAng>),
    DirectedIncoming(PartialPath<T, HasPos<WithIncoming>, HasAng>),
    RadiusArrival(super::family::RadiusArrival<T>),
    RadiusArrivalAt(super::family::RadiusArrivalAt<T>),
    RadiusArrivalDir(super::family::RadiusArrivalDir<T>),
    ViaArrival(super::family::ViaArrival<T>),
    ViaArrivalStart(super::family::ViaArrivalStart<T>),
}

impl<T: Real> DynTip<T> {
    fn state(&self) -> TipState {
        match self {
            DynTip::Entry => TipState::Entry,
            DynTip::Open(_) => TipState::Open,
            DynTip::Angle(_) => TipState::Angle,
            DynTip::PlainPoint(_) => TipState::PlainPoint,
            DynTip::DirectedPoint(_) => TipState::DirectedPoint,
            DynTip::DirectedPlain(_) => TipState::DirectedPlain,
            DynTip::DirectedIncoming(_) => TipState::DirectedIncoming,
            DynTip::RadiusArrival(_) => TipState::RadiusArrival,
            DynTip::RadiusArrivalAt(_) => TipState::RadiusArrivalAt,
            DynTip::RadiusArrivalDir(_) => TipState::RadiusArrivalDir,
            DynTip::ViaArrival(_) => TipState::ViaArrival,
            DynTip::ViaArrivalStart(_) => TipState::ViaArrivalStart,
        }
    }
}

/// What applying one step produced.
enum Applied<T: Real> {
    /// The chain continues at this tip.
    Tip(DynTip<T>),
    /// The step closed the loop.
    Closed(ProfileLoop<T>),
}

type Applying<T> = Result<Applied<T>, ReplayErrorKind<T>>;

fn violation<T: Real>(state: TipState, verb: Verb) -> Applying<T> {
    Err(ReplayErrorKind::Transition {
        state,
        verb: Some(verb),
    })
}

// The flavor-generic target dispatchers. Each branch names exactly ONE
// binder — the one well-typed at that (state, verb, target/mode).

fn do_line_to<T: Decide, F: Flavor>(
    p: PartialPath<T, HasPos<F>, NoAng>,
    t: Target<T>,
) -> Applying<T> {
    match t {
        Target::Point(q) => Ok(Applied::Tip(DynTip::DirectedPoint(p.line_to(q)?))),
        Target::Start => Ok(Applied::Closed(p.line_to(Start)?.loop_)),
    }
}

/// The sharp arc leg's mode dispatch: the endpoint-full modes from a
/// Point tip — one row per admissible (state, mode) pair of the §2c
/// matrix, each calling the one typed `arc_to(spec)` binder.
fn do_arc_to_point<T: ArcCarrierScalar, F: Flavor>(
    p: PartialPath<T, HasPos<F>, NoAng>,
    spec: ArcData<T>,
    state: TipState,
) -> Applying<T> {
    match spec {
        ArcData::Bulge {
            target: Target::Point(q),
            b,
        } => Ok(Applied::Tip(DynTip::DirectedPoint(
            p.arc_to(Bulge { p: q, b })?,
        ))),
        ArcData::Bulge {
            target: Target::Start,
            b,
        } => Ok(Applied::Closed(p.arc_to(Bulge { p: Start, b })?.loop_)),
        ArcData::Via {
            q,
            target: Target::Point(t),
        } => Ok(Applied::Tip(DynTip::DirectedPoint(
            p.arc_to(Via { q, p: t })?,
        ))),
        ArcData::Via {
            q,
            target: Target::Start,
        } => Ok(Applied::Closed(p.arc_to(Via { q, p: Start })?.loop_)),
        ArcData::Center {
            c,
            winding,
            target: Target::Point(t),
        } => Ok(Applied::Tip(DynTip::DirectedPoint(p.arc_to(Center {
            c,
            winding,
            p: t,
        })?))),
        ArcData::Center {
            c,
            winding,
            target: Target::Start,
        } => Ok(Applied::Closed(
            p.arc_to(Center {
                c,
                winding,
                p: Start,
            })?
            .loop_,
        )),
        ArcData::Radius { .. } | ArcData::Sweep { .. } | ArcData::ArcLen { .. } => {
            violation(state, Verb::ArcTo)
        }
    }
}

/// The endpoint-free sharp legs from a Directed tip.
fn do_arc_to_directed<T: ArcCarrierScalar, F: Flavor>(
    p: PartialPath<T, HasPos<F>, HasAng>,
    spec: ArcData<T>,
    state: TipState,
) -> Applying<T> {
    match spec {
        ArcData::Sweep { r, side, angle } => Ok(Applied::Tip(DynTip::DirectedPoint(
            p.arc_to(super::Sweep { r, side, angle })?,
        ))),
        ArcData::ArcLen { r, side, len } => Ok(Applied::Tip(DynTip::DirectedPoint(
            p.arc_to(super::ArcLen { r, side, len })?,
        ))),
        _ => violation(state, Verb::ArcTo),
    }
}

fn do_tangent_arc_to<T: Decide, F: Flavor>(
    p: PartialPath<T, HasPos<F>, HasAng>,
    t: Target<T>,
) -> Applying<T> {
    match t {
        Target::Point(q) => Ok(Applied::Tip(DynTip::DirectedPoint(p.tangent_arc_to(q)?))),
        Target::Start => Ok(Applied::Closed(p.tangent_arc_to(Start)?.loop_)),
    }
}

fn do_line<T: Decide, F: Flavor>(p: PartialPath<T, HasPos<F>, HasAng>, len: T) -> Applying<T> {
    Ok(Applied::Tip(DynTip::DirectedPoint(p.line(len)?)))
}

fn do_fillet<T: Decide, F: Flavor>(p: PartialPath<T, HasPos<F>, HasAng>, radius: T) -> Applying<T> {
    Ok(Applied::Tip(DynTip::Open(p.fillet(radius)?)))
}

/// Applies an ARC-ARRIVAL spec to an opened fillet — the shared second
/// half of the `FilletArc` / `ArcFilletArc` rows, dispatching each mode
/// to its own `ArrivalSpec` impl (the typed surface's exact code).
fn do_arrival<T: ArcCarrierScalar>(
    open: PartialPath<T, NoPos, NoAng>,
    spec: ArcData<T>,
    state: TipState,
    verb: Verb,
) -> Applying<T> {
    use super::family::ArrivalSpec;
    let core = open.core;
    match spec {
        ArcData::Center {
            c,
            winding,
            target: Target::Point(p),
        } => Ok(Applied::Tip(DynTip::DirectedPoint(ArrivalSpec::apply(
            core,
            super::Center { c, winding, p },
        )?))),
        ArcData::Center {
            c,
            winding,
            target: Target::Start,
        } => Ok(Applied::Closed(
            ArrivalSpec::apply(
                core,
                super::Center {
                    c,
                    winding,
                    p: Start,
                },
            )?
            .loop_,
        )),
        ArcData::Radius { r, side } => Ok(Applied::Tip(DynTip::RadiusArrival(ArrivalSpec::apply(
            core,
            super::Radius { r, side },
        )?))),
        ArcData::Via {
            q,
            target: Target::Point(p),
        } => Ok(Applied::Tip(DynTip::ViaArrival(ArrivalSpec::apply(
            core,
            super::Via { q, p },
        )?))),
        ArcData::Via {
            q,
            target: Target::Start,
        } => Ok(Applied::Tip(DynTip::ViaArrivalStart(ArrivalSpec::apply(
            core,
            super::Via { q, p: Start },
        )?))),
        ArcData::Bulge { .. } | ArcData::Sweep { .. } | ArcData::ArcLen { .. } => {
            violation(state, verb)
        }
    }
}

/// The fused incoming from a PLAIN point tip (the endpoint-full modes).
fn do_fused_point<T: ArcCarrierScalar>(
    p: PartialPath<T, HasPos<Plain>, NoAng>,
    spec: ArcData<T>,
    radius: T,
    state: TipState,
    verb: Verb,
) -> Result<PartialPath<T, NoPos, NoAng>, ReplayErrorKind<T>> {
    match spec {
        ArcData::Bulge {
            target: Target::Point(q),
            b,
        } => Ok(p.arc_fillet(super::Bulge { p: q, b }, radius)?),
        ArcData::Via {
            q,
            target: Target::Point(t),
        } => Ok(p.arc_fillet(super::Via { q, p: t }, radius)?),
        ArcData::Center {
            c,
            winding,
            target: Target::Point(t),
        } => Ok(p.arc_fillet(super::Center { c, winding, p: t }, radius)?),
        _ => Err(ReplayErrorKind::Transition {
            state,
            verb: Some(verb),
        }),
    }
}

/// The fused incoming from a DIRECTED POINT (leg end): the endpoint-full
/// modes plus `Radius` — arc extension (§2c dissolution).
fn do_fused_leg_end<T: ArcCarrierScalar>(
    p: PartialPath<T, HasPos<WithIncoming>, NoAng>,
    spec: ArcData<T>,
    radius: T,
    state: TipState,
    verb: Verb,
) -> Result<PartialPath<T, NoPos, NoAng>, ReplayErrorKind<T>> {
    match spec {
        ArcData::Bulge {
            target: Target::Point(q),
            b,
        } => Ok(p.arc_fillet(super::Bulge { p: q, b }, radius)?),
        ArcData::Via {
            q,
            target: Target::Point(t),
        } => Ok(p.arc_fillet(super::Via { q, p: t }, radius)?),
        ArcData::Center {
            c,
            winding,
            target: Target::Point(t),
        } => Ok(p.arc_fillet(super::Center { c, winding, p: t }, radius)?),
        ArcData::Radius { r, side } => Ok(p.arc_fillet(super::Radius { r, side }, radius)?),
        _ => Err(ReplayErrorKind::Transition {
            state,
            verb: Some(verb),
        }),
    }
}

/// The fused incoming from a DIRECTED tip (the endpoint-free modes).
fn do_fused_directed<T: ArcCarrierScalar, F: Flavor>(
    p: PartialPath<T, HasPos<F>, HasAng>,
    spec: ArcData<T>,
    radius: T,
    state: TipState,
    verb: Verb,
) -> Result<PartialPath<T, NoPos, NoAng>, ReplayErrorKind<T>> {
    match spec {
        ArcData::Sweep { r, side, angle } => {
            Ok(p.arc_fillet(super::Sweep { r, side, angle }, radius)?)
        }
        ArcData::ArcLen { r, side, len } => {
            Ok(p.arc_fillet(super::ArcLen { r, side, len }, radius)?)
        }
        _ => Err(ReplayErrorKind::Transition {
            state,
            verb: Some(verb),
        }),
    }
}

/// The fused incoming at the ENTRY (`Center` alone can seed).
fn do_fused_entry<T: ArcCarrierScalar>(
    spec: ArcData<T>,
    radius: T,
) -> Result<PartialPath<T, NoPos, NoAng>, ReplayErrorKind<T>> {
    match spec {
        ArcData::Center {
            c,
            winding,
            target: Target::Point(t),
        } => Ok(Open.arc_fillet(super::Center { c, winding, p: t }, radius)?),
        _ => Err(ReplayErrorKind::Transition {
            state: TipState::Entry,
            verb: Some(Verb::ArcFillet),
        }),
    }
}

/// Applies ONE step to the tip.
///
/// Every arm calls exactly one typed binder — the one well-typed at
/// that state. The trailing wildcard is the lattice violation: a
/// (state, verb) pair the authoring surface cannot spell. A missing
/// arm is the SAFE direction (an over-strict refusal), pinned by the
/// differential suite.
#[allow(clippy::too_many_lines)]
fn apply<T: ArcCarrierScalar>(tip: DynTip<T>, step: Step<T>) -> Applying<T> {
    match (tip, step) {
        // --- Entry (nothing bound; the `Open` value itself) -----------
        (DynTip::Entry, Step::At(p)) => Ok(Applied::Tip(DynTip::PlainPoint(Open.at(p)))),
        (DynTip::Entry, Step::Angle(theta)) => Ok(Applied::Tip(DynTip::Angle(Open.angle(theta)))),
        (DynTip::Entry, Step::Toward { dx, dy }) => {
            Ok(Applied::Tip(DynTip::Angle(Open.toward(dx, dy)?)))
        }
        (DynTip::Entry, Step::ArcFillet { spec, radius }) => {
            Ok(Applied::Tip(DynTip::Open(do_fused_entry(spec, radius)?)))
        }
        (
            DynTip::Entry,
            Step::ArcFilletArc {
                spec,
                radius,
                spec2,
            },
        ) => do_arrival(
            do_fused_entry(spec, radius)?,
            spec2,
            TipState::Entry,
            Verb::ArcFilletArc,
        ),
        (DynTip::Entry, Step::Circle { centre, radius }) => {
            Ok(Applied::Closed(super::circle(centre, radius)?.loop_))
        }
        (
            DynTip::Entry,
            Step::CircleSplit {
                centre,
                radius,
                n,
                phase,
            },
        ) => Ok(Applied::Closed(
            super::circle_split(centre, radius, n, phase)?.loop_,
        )),

        // --- Open = {} (a fillet's freshly opened LINE arrival) -------
        (DynTip::Open(p0), Step::At(p)) => Ok(Applied::Tip(DynTip::PlainPoint(p0.at(p)?))),
        (DynTip::Open(p0), Step::Angle(theta)) => Ok(Applied::Tip(DynTip::Angle(p0.angle(theta)?))),
        (DynTip::Open(p0), Step::Toward { dx, dy }) => {
            Ok(Applied::Tip(DynTip::Angle(p0.toward(dx, dy)?)))
        }
        (DynTip::Open(p0), Step::CloseTo) => Ok(Applied::Closed(p0.to(Start)?.loop_)),

        // --- Angle = {angle} ------------------------------------------
        (DynTip::Angle(p0), Step::At(p)) => Ok(Applied::Tip(DynTip::DirectedPlain(p0.at(p)?))),
        (DynTip::Angle(p0), Step::FarEndTo(anchor)) => {
            Ok(Applied::Tip(DynTip::DirectedPoint(p0.to(anchor)?)))
        }

        // --- Point = {position}, plain flavor -------------------------
        (DynTip::PlainPoint(p0), Step::Angle(theta)) => {
            Ok(Applied::Tip(DynTip::DirectedPlain(p0.angle(theta)?)))
        }
        (DynTip::PlainPoint(p0), Step::Toward { dx, dy }) => {
            Ok(Applied::Tip(DynTip::DirectedPlain(p0.toward(dx, dy)?)))
        }
        (DynTip::PlainPoint(p0), Step::LineTo(t)) => do_line_to(p0, t),
        (DynTip::PlainPoint(p0), Step::ArcTo(spec)) => {
            do_arc_to_point(p0, spec, TipState::PlainPoint)
        }
        (DynTip::PlainPoint(p0), Step::ArcFillet { spec, radius }) => {
            Ok(Applied::Tip(DynTip::Open(do_fused_point(
                p0,
                spec,
                radius,
                TipState::PlainPoint,
                Verb::ArcFillet,
            )?)))
        }
        (
            DynTip::PlainPoint(p0),
            Step::ArcFilletArc {
                spec,
                radius,
                spec2,
            },
        ) => do_arrival(
            do_fused_point(p0, spec, radius, TipState::PlainPoint, Verb::ArcFilletArc)?,
            spec2,
            TipState::PlainPoint,
            Verb::ArcFilletArc,
        ),

        // --- Point = {position}, directed flavor (a leg end) ----------
        (DynTip::DirectedPoint(p0), Step::Angle(theta)) => {
            Ok(Applied::Tip(DynTip::DirectedIncoming(p0.angle(theta)?)))
        }
        (DynTip::DirectedPoint(p0), Step::Toward { dx, dy }) => {
            Ok(Applied::Tip(DynTip::DirectedIncoming(p0.toward(dx, dy)?)))
        }
        (DynTip::DirectedPoint(p0), Step::Tangent) => {
            Ok(Applied::Tip(DynTip::DirectedIncoming(p0.tangent())))
        }
        (DynTip::DirectedPoint(p0), Step::Turn(delta)) => {
            Ok(Applied::Tip(DynTip::DirectedIncoming(p0.turn(delta)?)))
        }
        (DynTip::DirectedPoint(p0), Step::LineTo(t)) => do_line_to(p0, t),
        (DynTip::DirectedPoint(p0), Step::ArcContinue(p)) => {
            Ok(Applied::Tip(DynTip::DirectedPoint(p0.arc_continue(p)?)))
        }
        (DynTip::DirectedPoint(p0), Step::ArcTo(spec)) => {
            do_arc_to_point(p0, spec, TipState::DirectedPoint)
        }
        // §2c round 10: RAY EXTENSION — bare fillet directly on a leg end.
        (DynTip::DirectedPoint(p0), Step::Fillet { radius }) => {
            Ok(Applied::Tip(DynTip::Open(p0.fillet(radius)?)))
        }
        (DynTip::DirectedPoint(p0), Step::FilletArc { radius, spec }) => do_arrival(
            p0.fillet(radius)?,
            spec,
            TipState::DirectedPoint,
            Verb::FilletArc,
        ),
        (DynTip::DirectedPoint(p0), Step::ArcFillet { spec, radius }) => {
            Ok(Applied::Tip(DynTip::Open(do_fused_leg_end(
                p0,
                spec,
                radius,
                TipState::DirectedPoint,
                Verb::ArcFillet,
            )?)))
        }
        (
            DynTip::DirectedPoint(p0),
            Step::ArcFilletArc {
                spec,
                radius,
                spec2,
            },
        ) => do_arrival(
            do_fused_leg_end(
                p0,
                spec,
                radius,
                TipState::DirectedPoint,
                Verb::ArcFilletArc,
            )?,
            spec2,
            TipState::DirectedPoint,
            Verb::ArcFilletArc,
        ),

        // --- Directed = {both} ----------------------------------------
        (DynTip::DirectedPlain(p0), Step::Line(len)) => do_line(p0, len),
        (DynTip::DirectedPlain(p0), Step::Fillet { radius }) => do_fillet(p0, radius),
        (DynTip::DirectedPlain(p0), Step::TangentArcTo(t)) => do_tangent_arc_to(p0, t),
        (DynTip::DirectedPlain(p0), Step::ArcTo(spec)) => {
            do_arc_to_directed(p0, spec, TipState::DirectedPlain)
        }
        (DynTip::DirectedPlain(p0), Step::FilletArc { radius, spec }) => do_arrival(
            p0.fillet(radius)?,
            spec,
            TipState::DirectedPlain,
            Verb::FilletArc,
        ),
        (DynTip::DirectedPlain(p0), Step::ArcFillet { spec, radius }) => {
            Ok(Applied::Tip(DynTip::Open(do_fused_directed(
                p0,
                spec,
                radius,
                TipState::DirectedPlain,
                Verb::ArcFillet,
            )?)))
        }
        (
            DynTip::DirectedPlain(p0),
            Step::ArcFilletArc {
                spec,
                radius,
                spec2,
            },
        ) => do_arrival(
            do_fused_directed(
                p0,
                spec,
                radius,
                TipState::DirectedPlain,
                Verb::ArcFilletArc,
            )?,
            spec2,
            TipState::DirectedPlain,
            Verb::ArcFilletArc,
        ),
        (DynTip::DirectedIncoming(p0), Step::Line(len)) => do_line(p0, len),
        (DynTip::DirectedIncoming(p0), Step::Fillet { radius }) => do_fillet(p0, radius),
        (DynTip::DirectedIncoming(p0), Step::TangentArcTo(t)) => do_tangent_arc_to(p0, t),
        (DynTip::DirectedIncoming(p0), Step::ArcTo(spec)) => {
            do_arc_to_directed(p0, spec, TipState::DirectedIncoming)
        }
        (DynTip::DirectedIncoming(p0), Step::FilletArc { radius, spec }) => do_arrival(
            p0.fillet(radius)?,
            spec,
            TipState::DirectedIncoming,
            Verb::FilletArc,
        ),
        (DynTip::DirectedIncoming(p0), Step::ArcFillet { spec, radius }) => {
            Ok(Applied::Tip(DynTip::Open(do_fused_directed(
                p0,
                spec,
                radius,
                TipState::DirectedIncoming,
                Verb::ArcFillet,
            )?)))
        }
        (
            DynTip::DirectedIncoming(p0),
            Step::ArcFilletArc {
                spec,
                radius,
                spec2,
            },
        ) => do_arrival(
            do_fused_directed(
                p0,
                spec,
                radius,
                TipState::DirectedIncoming,
                Verb::ArcFilletArc,
            )?,
            spec2,
            TipState::DirectedIncoming,
            Verb::ArcFilletArc,
        ),

        // --- The arc-arrival builders (§2c) ---------------------------
        (DynTip::RadiusArrival(p0), Step::At(p)) => {
            Ok(Applied::Tip(DynTip::RadiusArrivalAt(p0.at(p))))
        }
        (DynTip::RadiusArrival(p0), Step::Angle(theta)) => {
            Ok(Applied::Tip(DynTip::RadiusArrivalDir(p0.angle(theta))))
        }
        (DynTip::RadiusArrival(p0), Step::Toward { dx, dy }) => {
            Ok(Applied::Tip(DynTip::RadiusArrivalDir(p0.toward(dx, dy)?)))
        }
        (DynTip::RadiusArrivalAt(p0), Step::Angle(theta)) => {
            Ok(Applied::Tip(DynTip::DirectedPoint(p0.angle(theta)?)))
        }
        (DynTip::RadiusArrivalAt(p0), Step::Toward { dx, dy }) => {
            Ok(Applied::Tip(DynTip::DirectedPoint(p0.toward(dx, dy)?)))
        }
        (DynTip::RadiusArrivalDir(p0), Step::At(p)) => {
            Ok(Applied::Tip(DynTip::DirectedPoint(p0.at(p)?)))
        }
        (DynTip::ViaArrival(p0), Step::Angle(theta)) => {
            Ok(Applied::Tip(DynTip::DirectedPoint(p0.angle(theta)?)))
        }
        (DynTip::ViaArrival(p0), Step::Toward { dx, dy }) => {
            Ok(Applied::Tip(DynTip::DirectedPoint(p0.toward(dx, dy)?)))
        }
        (DynTip::ViaArrivalStart(p0), Step::Angle(theta)) => {
            Ok(Applied::Closed(p0.angle(theta)?.loop_))
        }
        (DynTip::ViaArrivalStart(p0), Step::Toward { dx, dy }) => {
            Ok(Applied::Closed(p0.toward(dx, dy)?.loop_))
        }

        // --- everything else is a lattice violation -------------------
        (other, unusable) => Err(ReplayErrorKind::Transition {
            state: other.state(),
            verb: Some(unusable.verb()),
        }),
    }
}

/// **The replay driver**: elaborates a recorded program into the loop it
/// describes, through the typed binders and every check they carry —
/// the only path from steps to geometry, and the dynamic mirror of the
/// typestate lattice (every transition the markers forbid statically is
/// a typed [`ReplayErrorKind::Transition`] here).
///
/// A chain program must end in a `Start`-targeting verb; a
/// [`Step::Circle`] program is exactly one step. **The program replay
/// re-records is DISCARDED** — the input program is the authority, and
/// the round-trip is pinned by the differential suite rather than
/// trusted.
///
/// # Errors
///
/// [`ReplayError`], carrying the offending step index.
pub fn replay<T: ArcCarrierScalar>(steps: &[Step<T>]) -> Result<ProfileLoop<T>, ReplayError<T>> {
    let mut tip = DynTip::Entry;
    for (i, step) in steps.iter().enumerate() {
        let applied = apply(tip, *step).map_err(|kind| ReplayError { step: i, kind })?;
        match applied {
            Applied::Tip(next) => tip = next,
            Applied::Closed(lowered) => {
                return match steps.get(i + 1) {
                    Some(extra) => Err(ReplayError::transition(
                        i + 1,
                        TipState::Closed,
                        extra.verb(),
                    )),
                    None => Ok(lowered),
                };
            }
        }
    }
    Err(ReplayError {
        step: steps.len(),
        kind: ReplayErrorKind::Transition {
            state: tip.state(),
            verb: None,
        },
    })
}
