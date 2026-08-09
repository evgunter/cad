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
//!    [`PartialPath`](super::PartialPath) value, and applying a step is a
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
    ArcCarrierScalar, Flavor, HasAng, HasPos, NoAng, NoPos, Open, PartialPath, PathError, Plain,
    Start, WithIncoming,
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

/// One recorded authoring verb.
///
/// **Authored data only.** Every field is a number or a structural tag
/// the author wrote; nothing derived is stored (the §2a exactness
/// contract, item 1). In particular [`Step::ArcVia`] and
/// [`Step::ArcCenter`] keep their points and winding — the bulge is
/// re-derived at replay by the very binder the typed surface uses.
#[derive(Clone, Copy, Debug)]
pub enum Step<T: Real> {
    /// `.at(p)` — bind the position bit.
    At(Point2<T>),
    /// **G2** `.at_on(p, centre, winding)` — bind position AND the
    /// carrier's tangent there, in one act.
    AtOn {
        /// The anchor, on the carrier.
        p: Point2<T>,
        /// The carrier's centre.
        centre: Point2<T>,
        /// Travel sense about the centre (structural).
        winding: ArcSweep,
    },
    /// `.angle(θ)` — bind the outgoing direction as an angle (radians).
    Angle(T),
    /// **G1** `.toward(dx, dy)` — bind it as exact components. Only the
    /// RATIO carries meaning (dimensionless).
    Toward {
        /// x component.
        dx: T,
        /// y component.
        dy: T,
    },
    /// `.tangent()` — inherit the incoming end tangent and DECLARE the
    /// joint tangent.
    Tangent,
    /// `.turn(δ)` — depart at the incoming tangent rotated by δ.
    Turn(T),
    /// `line(len)` — a straight leg of the given length along the bound
    /// direction.
    Line(T),
    /// `line_to(target)` — a straight leg to the target.
    LineTo(Target<T>),
    /// `arc_to(target, bulge)` — an arc leg with an AUTHORED bulge.
    ArcTo {
        /// Where the leg ends.
        target: Target<T>,
        /// The authored bulge (M2 convention).
        bulge: T,
    },
    /// `arc_via(via, target)` — the arc through an authored via-point.
    /// The bulge is derived at replay.
    ArcVia {
        /// A point the arc passes through.
        via: Point2<T>,
        /// Where the leg ends.
        target: Target<T>,
    },
    /// `arc_center(centre, target, winding)` — the arc about an authored
    /// centre. The bulge is derived at replay.
    ArcCenter {
        /// The arc's centre.
        centre: Point2<T>,
        /// Where the leg ends.
        target: Target<T>,
        /// Travel sense about the centre (structural).
        winding: ArcSweep,
    },
    /// `tangent_arc_to(target)` — the unique arc leaving tangent to the
    /// bound direction and reaching the target.
    TangentArcTo(Target<T>),
    /// `arc_continue(target)` — the declared-subdivision step
    /// (LIB-SWITCH §5-1): continue the incoming ARC carrier to an
    /// authored on-carrier point, minting a STRUCTURAL subdivision
    /// vertex (a same-carrier identity, not a junction claim).
    ArcContinue(Point2<T>),
    /// `.fillet(r)` — open a fillet of radius `r`; the tip becomes an
    /// unbound arrival side.
    Fillet {
        /// The fillet radius.
        radius: T,
    },
    /// **G1** `.to(anchor)` — the far-end anchor: bind the arrival
    /// side's position and END the side there.
    FarEndTo(Point2<T>),
    /// `.to(Start)` — the SEAM FILLET close: the fillet arc becomes the
    /// closing segment and the entry vertex is retrimmed.
    CloseTo,
    /// **G2** `.to_on(Start, centre, winding)` — close with the arrival
    /// running on a carrier that differs from side 1's, so the entry
    /// vertex is KEPT as a genuine two-carrier junction.
    CloseToOn {
        /// The arrival carrier's centre.
        centre: Point2<T>,
        /// Travel sense about the centre (structural).
        winding: ArcSweep,
    },
    /// [`circle`](super::circle) — the one-step complete-loop program
    /// form. A `Circle` program is exactly one step: it authors no seam,
    /// binds nothing, and continues into nothing.
    Circle {
        /// The circle's centre.
        centre: Point2<T>,
        /// The circle's radius (must classify definitely positive).
        radius: T,
    },
    /// [`circle_split`](super::circle_split) — the declared-subdivision
    /// closed carrier: `n` equal arcs, first vertex at `phase`. Like
    /// [`Step::Circle`] a one-step complete-loop program form.
    CircleSplit {
        /// The carrier's centre.
        centre: Point2<T>,
        /// The carrier's radius (must classify definitely positive).
        radius: T,
        /// The subdivision count (STRUCTURAL — a count, ≥ 2).
        n: usize,
        /// The first vertex's angle from +x (continuous).
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
/// match is written over.
///
/// Four lattice states (PATHS-DESIGN §5), with the marker flavors named
/// where the surface distinguishes them, plus the two ends of a run:
/// [`TipState::Entry`] (nothing authored yet) and [`TipState::Closed`]
/// (the loop is finished).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TipState {
    /// Before the first verb — the `Open` value itself.
    Entry,
    /// `Open` = {}: a fillet's freshly opened arrival side.
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
    /// The loop is closed; no verb may follow.
    Closed,
}

/// Which verb a step names — the `verb` half of a
/// [`ReplayErrorKind::Transition`]. One value per [`Step`] variant.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Verb {
    /// [`Step::At`].
    At,
    /// [`Step::AtOn`].
    AtOn,
    /// [`Step::Angle`].
    Angle,
    /// [`Step::Toward`].
    Toward,
    /// [`Step::Tangent`].
    Tangent,
    /// [`Step::Turn`].
    Turn,
    /// [`Step::Line`].
    Line,
    /// [`Step::LineTo`].
    LineTo,
    /// [`Step::ArcTo`].
    ArcTo,
    /// [`Step::ArcVia`].
    ArcVia,
    /// [`Step::ArcCenter`].
    ArcCenter,
    /// [`Step::TangentArcTo`].
    TangentArcTo,
    /// [`Step::ArcContinue`].
    ArcContinue,
    /// [`Step::Fillet`].
    Fillet,
    /// [`Step::FarEndTo`].
    FarEndTo,
    /// [`Step::CloseTo`].
    CloseTo,
    /// [`Step::CloseToOn`].
    CloseToOn,
    /// [`Step::Circle`].
    Circle,
    /// [`Step::CircleSplit`].
    CircleSplit,
}

impl<T: Real> Step<T> {
    /// The verb this step names.
    pub fn verb(&self) -> Verb {
        match self {
            Step::At(_) => Verb::At,
            Step::AtOn { .. } => Verb::AtOn,
            Step::Angle(_) => Verb::Angle,
            Step::Toward { .. } => Verb::Toward,
            Step::Tangent => Verb::Tangent,
            Step::Turn(_) => Verb::Turn,
            Step::Line(_) => Verb::Line,
            Step::LineTo(_) => Verb::LineTo,
            Step::ArcTo { .. } => Verb::ArcTo,
            Step::ArcVia { .. } => Verb::ArcVia,
            Step::ArcCenter { .. } => Verb::ArcCenter,
            Step::TangentArcTo(_) => Verb::TangentArcTo,
            Step::ArcContinue(_) => Verb::ArcContinue,
            Step::Fillet { .. } => Verb::Fillet,
            Step::FarEndTo(_) => Verb::FarEndTo,
            Step::CloseTo => Verb::CloseTo,
            Step::CloseToOn { .. } => Verb::CloseToOn,
            Step::Circle { .. } => Verb::Circle,
            Step::CircleSplit { .. } => Verb::CircleSplit,
        }
    }
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
    /// **Lattice violation** — the step's verb is not well-typed at the
    /// tip's state. No authoring surface can produce this: it is the
    /// corrupt-or-hand-edited-file class, refused typed at the door.
    ///
    /// `verb` is `None` when the program simply ENDED in `state` without
    /// a closing verb — a chain that never reaches [`Start`] is the same
    /// lattice violation, one step past the end.
    Transition {
        /// The tip's lattice state.
        state: TipState,
        /// The verb that is ill-typed there, or `None` for end-of-program.
        verb: Option<Verb>,
    },
    /// **Geometry refusal** — the chain is well-typed but the geometry
    /// refuses (a tangent junction, no fillet corner, a nonpositive leg).
    /// This class CAN exist at rest: once arguments carry expressions,
    /// whether a program elaborates depends on the parameter binding.
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
/// TYPED [`PartialPath`] value for that state.
///
/// This is the drift-proofing construction. Because each variant holds
/// the real typed value, a match arm on (variant, verb) can only call a
/// binder that is well-typed at that state — there is no way to spell an
/// illegal transition, and no binder body is duplicated.
enum DynTip<T: Real> {
    Entry,
    Open(PartialPath<T, NoPos, NoAng>),
    Angle(PartialPath<T, NoPos, HasAng>),
    PlainPoint(PartialPath<T, HasPos<Plain>, NoAng>),
    DirectedPoint(PartialPath<T, HasPos<WithIncoming>, NoAng>),
    DirectedPlain(PartialPath<T, HasPos<Plain>, HasAng>),
    DirectedIncoming(PartialPath<T, HasPos<WithIncoming>, HasAng>),
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

// The flavor-generic target dispatchers. Each is generic over exactly
// the marker the surface itself is generic over, so every branch below
// still names ONE binder — the one well-typed at that (state, verb,
// target-kind).

fn do_line_to<T: Decide, F: Flavor>(
    p: PartialPath<T, HasPos<F>, NoAng>,
    t: Target<T>,
) -> Applying<T> {
    match t {
        Target::Point(q) => Ok(Applied::Tip(DynTip::DirectedPoint(p.line_to(q)?))),
        Target::Start => Ok(Applied::Closed(p.line_to(Start)?.loop_)),
    }
}

fn do_arc_to<T: Decide, F: Flavor>(
    p: PartialPath<T, HasPos<F>, NoAng>,
    t: Target<T>,
    bulge: T,
) -> Applying<T> {
    match t {
        Target::Point(q) => Ok(Applied::Tip(DynTip::DirectedPoint(p.arc_to(q, bulge)?))),
        Target::Start => Ok(Applied::Closed(p.arc_to(Start, bulge)?.loop_)),
    }
}

fn do_arc_via<T: Decide, F: Flavor>(
    p: PartialPath<T, HasPos<F>, NoAng>,
    via: Point2<T>,
    t: Target<T>,
) -> Applying<T> {
    match t {
        Target::Point(q) => Ok(Applied::Tip(DynTip::DirectedPoint(p.arc_via(via, q)?))),
        Target::Start => Ok(Applied::Closed(p.arc_via(via, Start)?.loop_)),
    }
}

fn do_arc_center<T: Decide, F: Flavor>(
    p: PartialPath<T, HasPos<F>, NoAng>,
    centre: Point2<T>,
    t: Target<T>,
    winding: ArcSweep,
) -> Applying<T> {
    match t {
        Target::Point(q) => Ok(Applied::Tip(DynTip::DirectedPoint(
            p.arc_center(centre, q, winding)?,
        ))),
        Target::Start => Ok(Applied::Closed(p.arc_center(centre, Start, winding)?.loop_)),
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

/// Applies ONE step to the tip.
///
/// Every arm calls exactly one binder — the one the typestate makes
/// well-typed at that state. The trailing wildcard is the lattice
/// violation: a (state, verb) pair the authoring surface cannot spell.
fn apply<T: ArcCarrierScalar>(tip: DynTip<T>, step: Step<T>) -> Applying<T> {
    match (tip, step) {
        // --- Entry (nothing bound; the `Open` value itself) -----------
        (DynTip::Entry, Step::At(p)) => Ok(Applied::Tip(DynTip::PlainPoint(Open.at(p)))),
        (DynTip::Entry, Step::Angle(theta)) => Ok(Applied::Tip(DynTip::Angle(Open.angle(theta)))),
        (DynTip::Entry, Step::Toward { dx, dy }) => {
            Ok(Applied::Tip(DynTip::Angle(Open.toward(dx, dy)?)))
        }
        (DynTip::Entry, Step::AtOn { p, centre, winding }) => Ok(Applied::Tip(
            DynTip::DirectedPlain(Open.at_on(p, centre, winding)?),
        )),
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

        // --- Open = {} (a fillet's freshly opened arrival side) -------
        (DynTip::Open(p0), Step::At(p)) => Ok(Applied::Tip(DynTip::PlainPoint(p0.at(p)?))),
        (DynTip::Open(p0), Step::Angle(theta)) => Ok(Applied::Tip(DynTip::Angle(p0.angle(theta)?))),
        (DynTip::Open(p0), Step::Toward { dx, dy }) => {
            Ok(Applied::Tip(DynTip::Angle(p0.toward(dx, dy)?)))
        }
        (DynTip::Open(p0), Step::AtOn { p, centre, winding }) => Ok(Applied::Tip(
            DynTip::DirectedPlain(p0.at_on(p, centre, winding)?),
        )),
        (DynTip::Open(p0), Step::CloseTo) => Ok(Applied::Closed(p0.to(Start)?.loop_)),
        (DynTip::Open(p0), Step::CloseToOn { centre, winding }) => {
            Ok(Applied::Closed(p0.to_on(Start, centre, winding)?.loop_))
        }

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
        (DynTip::PlainPoint(p0), Step::ArcTo { target, bulge }) => do_arc_to(p0, target, bulge),
        (DynTip::PlainPoint(p0), Step::ArcVia { via, target }) => do_arc_via(p0, via, target),
        (
            DynTip::PlainPoint(p0),
            Step::ArcCenter {
                centre,
                target,
                winding,
            },
        ) => do_arc_center(p0, centre, target, winding),

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
        (DynTip::DirectedPoint(p0), Step::ArcTo { target, bulge }) => do_arc_to(p0, target, bulge),
        (DynTip::DirectedPoint(p0), Step::ArcVia { via, target }) => do_arc_via(p0, via, target),
        (
            DynTip::DirectedPoint(p0),
            Step::ArcCenter {
                centre,
                target,
                winding,
            },
        ) => do_arc_center(p0, centre, target, winding),

        // --- Directed = {both} ----------------------------------------
        (DynTip::DirectedPlain(p0), Step::Line(len)) => do_line(p0, len),
        (DynTip::DirectedPlain(p0), Step::Fillet { radius }) => do_fillet(p0, radius),
        (DynTip::DirectedPlain(p0), Step::TangentArcTo(t)) => do_tangent_arc_to(p0, t),
        (DynTip::DirectedIncoming(p0), Step::Line(len)) => do_line(p0, len),
        (DynTip::DirectedIncoming(p0), Step::Fillet { radius }) => do_fillet(p0, radius),
        (DynTip::DirectedIncoming(p0), Step::TangentArcTo(t)) => do_tangent_arc_to(p0, t),

        // --- everything else is a lattice violation -------------------
        (other, unusable) => Err(ReplayErrorKind::Transition {
            state: other.state(),
            verb: Some(unusable.verb()),
        }),
    }
}

/// **The replay driver**: elaborates a recorded program into the loop it
/// describes, through the typed binders and every check they carry.
///
/// This is the only path from steps to geometry. It is the dynamic
/// mirror of the typestate lattice: every transition the markers forbid
/// statically is a typed [`ReplayErrorKind::Transition`] here, and every
/// geometric refusal the binders raise is a
/// [`ReplayErrorKind::Path`] — see those variants for why the two
/// classes are deliberately different.
///
/// A chain program must end in a `Start`-targeting verb; a
/// [`Step::Circle`] program is exactly one step. Anything else — an
/// empty program, a chain that stops mid-air, a step after the close —
/// is the transition class.
///
/// **The program replay re-records is DISCARDED.** Driving the binders
/// makes them record too, so the closing verb hands back a
/// [`ClosedLoop`] whose `program` is a re-derivation of the input; this
/// function keeps only `loop_`. That is right for today's consumers (the
/// input program is the authority, and the round-trip is pinned by the
/// differential suite rather than trusted). A caller that wants the
/// re-recorded program — a canonicalizer, say — needs a variant that
/// returns the pair; none exists yet because nothing needs one.
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
