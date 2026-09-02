//! **The profile-program payload (LIB-SWITCH §4, PROFILES-V2 §§V1–V4):
//! the program IS the profile's definition.**
//!
//! [`ProfileProgram`] replaces the retired opaque `ProfileDesc` as
//! `Node::Profile`'s payload: plane placement (stored `f64` in its own
//! struct — the U4/VQ8 seam stays visible) plus one [`LoopProgram`] per
//! loop. A loop-program is the constructor-call sequence as data — the
//! chain vocabulary's steps with every CONTINUOUS argument an [`Expr`]
//! (V2's dimension table) and every STRUCTURAL argument a literal tag
//! (verb identity, winding, `Start`, the `circle_split` count).
//!
//! # The strict door (V1, wire.rs's rule at the program layer)
//!
//! Nothing here can mint a `profile::ProfileLoop`. Evaluation (and the
//! authoring-time check) RESOLVES the expressions at `f64` and hands
//! the resolved steps to `profile::replay` — the driver, hence the
//! typed binders and every check they carry. Deserialization rebuilds
//! THIS type (through `Expr`'s dimension doors); the only path from
//! steps to geometry runs through the driver.
//!
//! # f64 resolution (V2, the verified asymmetry)
//!
//! Program expressions resolve at **f64**, never at the evaluation
//! scalar: profile geometry feeds C6 structure selection (junction
//! classes, fillet fits), which must be decided once, identically for
//! every lane — exactly the stored-f64-bits behavior the retired
//! representation had. Node MAGNITUDE slots stay lane-live; the
//! asymmetry is inherited from `Doc::param_env`, not invented here.
//!
//! # Caches (V3)
//!
//! Replayed segments are the node's evaluated payload in the existing
//! per-node memo (`eval/mod.rs`'s prior-Evaluation reuse); NOTHING new
//! is persisted — the derived-value list in `persist`'s module docs
//! gains the segments. D9 makes the load-time rebuild exact.

use geom_core::{Decide, Point2};
use profile::{ArcSweep, SketchPlane, Step, Target};

use crate::expr::{Dimension, DimensionError, EvalError, Expr, ParamEnv, eval};
use crate::node::{SlotId, StepArg};
use geom_core::Tol;

/// Where a target-taking step ends: an authored point (two Length
/// expressions) or the entry vertex (`Start` — structural; targeting it
/// closes the loop). Mirrors `profile::Target`.
#[derive(Debug, Clone, PartialEq)]
pub enum ProgramTarget {
    /// An authored absolute point in the profile frame.
    Point([Expr; 2]),
    /// The entry vertex: this step closes the loop.
    Start,
    /// The entry vertex with the closing leg's ARRIVAL declared — a
    /// structural tag, no expressions (so it contributes no slots).
    StartArriving(ProgramArrival),
}

/// The document mirror of [`profile::Arrival`]: which arrival a closing
/// step declares at the seam. Structural, so it carries no [`Expr`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProgramArrival {
    /// Arrives STRAIGHT into the entry's first side.
    Straight,
    /// Arrives G1 into the entry's outgoing direction.
    Tangent,
}

/// One Expr-bearing recorded verb — the document-layer mirror of
/// [`profile::Step`], structural tags literal, continuous args [`Expr`]
/// (V2's table: coordinates/lengths/radii `Length`, angle/turn/phase
/// `Angle`, bulge and director components `Scalar`).
///
/// It is a second spelling of a vocabulary `profile` declares once,
/// and it has to be: a step here carries `Expr`s and serializes, and
/// G1 layering keeps both out of the kernel crate.
/// [`LoopProgram::from_recorded`] below
/// is exhaustive on [`profile::Step`], so a verb the transition table
/// gains breaks this file at compile, and
/// `tests/switch_program_vocabulary.rs` is the census that makes it
/// break for the right reason: the verb has to arrive HERE, not merely
/// be discharged in `from_recorded`'s error arm.
///
/// Fields are public data (the node-slot pattern: dimensions are
/// checked at the edit door via [`ProfileProgram::slots`] +
/// [`StepArg::dimension`], and at the persistence doors' shared
/// validator — never trusted from a parsed file).
#[derive(Debug, Clone, PartialEq)]
pub enum ProgramStep {
    /// `.at(p)`.
    At([Expr; 2]),
    /// `.angle(θ)` (radians).
    Angle(Expr),
    /// **G1** `.toward(dx, dy)` — exact components, ratio-only.
    Toward {
        /// x component (Scalar).
        dx: Expr,
        /// y component (Scalar).
        dy: Expr,
    },
    /// `.tangent()` — structural, no arguments.
    Tangent,
    /// `.cusp()` — structural, no arguments: the declared
    /// reverse-tangent junction (D1's wedge-0/2π authoring door).
    Cusp,
    /// `.turn(δ)`.
    Turn(Expr),
    /// `line(len)`.
    Line(Expr),
    /// `line_to(target)`.
    LineTo(ProgramTarget),
    /// `continue_to(target)` — the declared point-target straight
    /// continuation; `Start` targets close the loop.
    ContinueTo(ProgramTarget),
    /// `arc_to(spec)` — the sharp arc leg, every §2c mode in the one
    /// unified spec record (derived quantities re-derived at replay).
    ArcTo(ProgramArcData),
    /// `tangent_arc_to(target)`.
    TangentArcTo(ProgramTarget),
    /// `arc_continue(target)` — the declared-subdivision step
    /// (LIB-SWITCH §5-1): a STRUCTURAL vertex on the incoming carrier.
    ArcContinue([Expr; 2]),
    /// `.fillet(r)` — line incoming, line arrival.
    Fillet(Expr),
    /// **§2c** `fillet_arc(r, spec)` — line incoming, arc arrival.
    FilletArc {
        /// The fillet radius.
        radius: Expr,
        /// The arc-arrival spec.
        spec: ProgramArcData,
    },
    /// **§2c** `arc_fillet(spec, r)` — fused arc incoming, line arrival.
    ArcFillet {
        /// The fused incoming-arc spec.
        spec: ProgramArcData,
        /// The fillet radius.
        radius: Expr,
    },
    /// **§2c** `arc_fillet_arc(spec, r, spec₂)` — fused arc incoming,
    /// arc arrival.
    ArcFilletArc {
        /// The fused incoming-arc spec.
        spec: ProgramArcData,
        /// The fillet radius.
        radius: Expr,
        /// The arc-arrival spec.
        spec2: ProgramArcData,
    },
    /// **G1** `.to(anchor)` — the far-end anchor.
    FarEndTo([Expr; 2]),
    /// `.to(Start)` — the seam-fillet close (structural).
    CloseTo,
}

/// The document-layer mirror of [`profile::ArcData`] (§2c's unified
/// arc-spec record): continuous fields [`Expr`], structural tags
/// literal (`side`, `winding`, `Start`).
///
/// It is the arc-mode vocabulary's second spelling, and it has to be
/// for the reason [`ProgramStep`] does. A mode the kernel vocabulary
/// gains does break this crate at compile — `spec_lit` and the two
/// content-key hashers are exhaustive on `profile::ArcData` — but
/// each of those breaks can be discharged where it stands, with a
/// refusal arm and a tag, while this enum, the wire and the
/// expression-slot roles stay short: the hop that would need them,
/// `res_spec`, matches THIS type and CONSTRUCTS the kernel one, so it
/// keeps compiling. What forces arrival is the mode census in
/// `tests/switch_program_vocabulary.rs`, keyed on
/// [`profile::ArcMode::ALL`]: its witness is a match on the mode tag,
/// so a mode with no document spelling is a compile error there.
#[derive(Debug, Clone, PartialEq)]
pub enum ProgramArcData {
    /// `Radius { r, side }` — arrival mode, centre derived.
    Radius {
        /// The carrier radius.
        r: Expr,
        /// Which side of the tangent the centre sits on (structural).
        side: profile::ArcSide,
    },
    /// `Bulge { p, b }` — the bulge is AUTHORED data.
    Bulge {
        /// The authored endpoint.
        target: ProgramTarget,
        /// The authored bulge (M2 convention, Scalar).
        b: Expr,
    },
    /// `Via { q, p }` — bulge derived at replay.
    Via {
        /// A point the arc passes through.
        q: [Expr; 2],
        /// The authored endpoint.
        target: ProgramTarget,
    },
    /// `Center { c, winding, p }` — bulge derived at replay.
    Center {
        /// The carrier centre.
        c: [Expr; 2],
        /// Travel sense (structural).
        winding: ArcSweep,
        /// The authored anchor/endpoint (`Start` closes).
        target: ProgramTarget,
    },
    /// `Sweep { r, side, angle }` — endpoint derived at replay.
    Sweep {
        /// The carrier radius.
        r: Expr,
        /// Which side the centre sits on (structural).
        side: profile::ArcSide,
        /// The swept central angle.
        angle: Expr,
    },
    /// `ArcLen { r, side, len }` — endpoint derived at replay.
    ArcLen {
        /// The carrier radius.
        r: Expr,
        /// Which side the centre sits on (structural).
        side: profile::ArcSide,
        /// The arc length.
        len: Expr,
    },
}

/// One loop's program: a CHAIN step list, or one of the complete-loop
/// carrier forms (`circle` / `circle_split` — one-step programs whose
/// form is structural). The chain-vs-carrier distinction is the enum,
/// so "a circle program is exactly one step" is unrepresentable to
/// violate.
#[derive(Debug, Clone, PartialEq)]
pub enum LoopProgram {
    /// A chain-vocabulary step list (must end in a `Start`-targeting
    /// verb — checked by replay, not representation).
    Chain(Vec<ProgramStep>),
    /// `circle(centre, r)` — the seamless closed carrier.
    Circle {
        /// The circle's centre.
        centre: [Expr; 2],
        /// The circle's radius.
        radius: Expr,
    },
    /// `circle_split(centre, r, n, phase)` — the declared-subdivision
    /// closed carrier (corpus ruling (a); `n` STRUCTURAL).
    CircleSplit {
        /// The carrier's centre.
        centre: [Expr; 2],
        /// The carrier's radius.
        radius: Expr,
        /// The subdivision count (structural, ≥ 2 at replay).
        n: u32,
        /// The first vertex's angle from +x.
        phase: Expr,
    },
}

/// The profile node's payload: plane placement (stored `f64`, its own
/// struct — VQ8's visible seam) plus the loop programs, outer first
/// then holes in description order.
///
/// # Equality is BIT equality
///
/// `PartialEq` compares plane floats by BITS and expressions by
/// [`Expr::bit_eq`] — the canonical payload's equality IS the D7
/// replay-identity comparator, exactly the retired payload's contract
/// (`Node::bit_eq` inherits it). Display units are invisible to it
/// (they are invisible to `bit_eq` itself, D7).
#[derive(Debug, Clone)]
pub struct ProfileProgram {
    /// The sketch-plane placement (stored `f64`; Expr-izing placement
    /// is the U4 pose conversation, not this switch — VQ8).
    pub plane: SketchPlane<f64>,
    /// The loop programs.
    pub loops: Vec<LoopProgram>,
}

/// The canonical `Doc` instantiation (the retired `ProfileDesc` seat).
pub type ProfileDoc = crate::doc::Doc<ProfileProgram>;

// ------------------------------------------------------------------
// The payload trait (Node<P> genericity's seam)
// ------------------------------------------------------------------

/// What `Node<P>` needs from a profile payload so slot addressing and
/// the authoring-time check stay generic (`Doc<P>` keeps its fake test
/// payloads — the defaults are the slot-free, check-free behavior the
/// retired opaque payload had).
pub trait ProfilePayload {
    /// Every program expression slot, deterministic (loop, step, arg)
    /// order.
    fn slots(&self) -> Vec<SlotId> {
        Vec::new()
    }
    /// The expression a profile slot addresses, `None` off the program.
    fn expr(&self, _slot: SlotId) -> Option<&Expr> {
        None
    }
    /// Mutable twin of [`ProfilePayload::expr`].
    fn expr_mut(&mut self, _slot: SlotId) -> Option<&mut Expr> {
        None
    }
    /// The authoring-time check (VQ9): resolve + replay + validate
    /// under the CURRENT parameter environment, refusing typed at the
    /// edit door. The evaluation-time twin re-checks under every
    /// binding that is ever evaluated.
    fn check(&self, _env: &ParamEnv<f64>, _tol: Tol) -> Result<(), ProgramRefusal> {
        Ok(())
    }
}

/// A typed authoring-time program refusal (VQ9; `EditError`'s payload).
///
/// The resolve and validate classes carry their causes UNALTERED
/// (`EvalError`/`ProfileError` are `PartialEq`, as `EditError`
/// requires). The geometry-replay class cannot carry its cause whole:
/// `profile::PathError` is generic in the evaluation scalar and its
/// arms carry scalar payloads, and `Real` omits comparison. A derived
/// `PartialEq` would not be unavailable so much as useless — it would
/// exist only where the scalar supplies equality on its own, which is
/// `f64` and neither `Interval` nor `Dual`, and even at `f64` it is
/// float `==`, non-reflexive at the poison value `Real`'s totality
/// contract promises. [`ProgramRefusal::Geometry`] therefore
/// carries the part that does compare — `profile::PathErrorKind`, the
/// refusal's class — beside the driver's rendered sentence and the
/// typed coordinates. **The class is the typed interface; the prose is
/// for a reader.** A consumer asking WHICH geometry refusal fired
/// matches that variant's `kind` and never the string.
/// The full typed error remains the EVALUATION surface's contract
/// (`NodeErrorKind` carries it unaltered); the edit door is the early
/// ergonomic mirror. REPORTED shape, not silent (LIB-SWITCH §10).
#[derive(Debug, Clone, PartialEq)]
pub enum ProgramRefusal {
    /// A program expression failed to resolve at f64 (unknown param,
    /// dimension drift, non-finite result).
    Resolve {
        /// The failing slot.
        slot: SlotId,
        /// The evaluator's refusal, unaltered.
        source: EvalError,
    },
    /// The resolved steps are not a legal lattice walk (corrupt or
    /// hand-built program — no recording surface produces this).
    Transition {
        /// The loop whose replay refused.
        loop_: u32,
        /// The offending step index (one past the end for an unclosed
        /// chain).
        step: u32,
        /// The tip's lattice state.
        state: profile::TipState,
        /// The ill-typed verb, `None` for end-of-program.
        verb: Option<profile::Verb>,
    },
    /// The chain is well-typed but the geometry refuses under the
    /// current binding (V1 class 2 — legal at rest; the CURRENT env
    /// refuses it at the door for fail-loud-early ergonomics).
    Geometry {
        /// The loop whose replay refused.
        loop_: u32,
        /// The offending step index.
        step: u32,
        /// Which geometry refusal fired — the typed half, and the one
        /// a consumer branches on.
        kind: profile::PathErrorKind,
        /// The driver's rendered refusal, for a reader. Carries the
        /// scalar payloads `kind` drops; never an interface.
        rendered: String,
    },
    /// The replayed loops refused profile validation under the current
    /// binding (also V1 class 2).
    Validate(profile::ProfileError),
}

// LIB-DOORS F6 (reopened on review): a human-readable rendering. Each
// arm names the failing stage and then forwards its payload's own
// prose — the geometry class the driver's rendered refusal, the
// validate class `ProfileError`'s `Display`. `Resolve` states its
// problem instead because `EvalError` has no `Display` to forward;
// `Transition` holds a lattice state rather than a refusal.
impl core::fmt::Display for ProgramRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Resolve { slot, .. } => {
                write!(f, "a program expression failed to resolve at slot {slot:?}")
            }
            Self::Transition { loop_, step, .. } => write!(
                f,
                "loop {loop_} step {step} is not a legal chain-lattice walk"
            ),
            Self::Geometry {
                loop_,
                step,
                rendered,
                ..
            } => write!(f, "loop {loop_} step {step}: {rendered}"),
            Self::Validate(e) => write!(f, "the replayed loops failed profile validation: {e}"),
        }
    }
}

impl core::error::Error for ProgramRefusal {}

// ------------------------------------------------------------------
// Slot access
// ------------------------------------------------------------------

/// The argument roles a target contributes ([] for `Start`).
///
/// Exhaustive on the target vocabulary rather than a test for one
/// form: a target form that carries expressions and enumerates no role
/// is an expression no slot addresses, which the bijection census sees
/// only where the corpus reaches it.
fn target_slots(t: &ProgramTarget, out: &mut Vec<StepArg>) {
    match t {
        ProgramTarget::Point(_) => out.extend([StepArg::TargetX, StepArg::TargetY]),
        ProgramTarget::Start | ProgramTarget::StartArriving(_) => {}
    }
}

/// The argument roles of one arc spec; `second` selects the arrival
/// (spec₂) role twins.
///
/// The twins cover the positional roles only. `Bulge`, `Sweep` and
/// `ArcLen` have none, because none of them is an arrival mode (§2c:
/// `family::ArrivalSpec` is implemented for `Radius`, `Via` and
/// `Center` alone), so no recording surface can put one in second
/// position. Enumeration stays total over the data type regardless,
/// and for a HAND-BUILT step whose two specs are the SAME one of
/// those three the reused role addresses the incoming spec's argument
/// twice and the arrival's not at all — issue #829.
fn spec_slots(spec: &ProgramArcData, second: bool, out: &mut Vec<StepArg>) {
    use ProgramArcData as S;
    use StepArg as A;
    match (spec, second) {
        (S::Radius { .. }, false) => out.push(A::CarrierRadius),
        (S::Radius { .. }, true) => out.push(A::CarrierRadius2),
        (S::Bulge { target, .. }, false) => {
            target_slots(target, out);
            out.push(A::Bulge);
        }
        // No `Bulge2`: see the twin note above.
        (S::Bulge { target, .. }, true) => {
            target2_slots(target, out);
            out.push(A::Bulge);
        }
        (S::Via { target, .. }, false) => {
            out.extend([A::ViaX, A::ViaY]);
            target_slots(target, out);
        }
        (S::Via { target, .. }, true) => {
            out.extend([A::Via2X, A::Via2Y]);
            target2_slots(target, out);
        }
        (S::Center { target, .. }, false) => {
            out.extend([A::CenterX, A::CenterY]);
            target_slots(target, out);
        }
        (S::Center { target, .. }, true) => {
            out.extend([A::Center2X, A::Center2Y]);
            target2_slots(target, out);
        }
        (S::Sweep { .. }, false) => out.extend([A::CarrierRadius, A::SweepVal]),
        (S::Sweep { .. }, true) => out.extend([A::CarrierRadius2, A::SweepVal]),
        (S::ArcLen { .. }, false) => out.extend([A::CarrierRadius, A::ArcLenVal]),
        (S::ArcLen { .. }, true) => out.extend([A::CarrierRadius2, A::ArcLenVal]),
    }
}

/// The spec₂ twin of [`target_slots`], exhaustive for the same reason.
fn target2_slots(t: &ProgramTarget, out: &mut Vec<StepArg>) {
    match t {
        ProgramTarget::Point(_) => out.extend([StepArg::Target2X, StepArg::Target2Y]),
        ProgramTarget::Start | ProgramTarget::StartArriving(_) => {}
    }
}

/// The argument roles of one chain step, enumeration order = the
/// step's own field order (deterministic; pinned by tests).
fn step_slots(step: &ProgramStep, out: &mut Vec<StepArg>) {
    use ProgramStep as P;
    use StepArg as A;
    match step {
        P::At(_) | P::FarEndTo(_) => out.extend([A::PointX, A::PointY]),
        P::Angle(_) => out.push(A::AngleVal),
        P::Toward { .. } => out.extend([A::DirX, A::DirY]),
        P::Tangent | P::Cusp | P::CloseTo => {}
        P::Turn(_) => out.push(A::TurnVal),
        P::Line(_) => out.push(A::Length),
        P::LineTo(t) | P::ContinueTo(t) | P::TangentArcTo(t) => target_slots(t, out),
        P::ArcContinue(_) => out.extend([A::TargetX, A::TargetY]),
        P::ArcTo(spec) => spec_slots(spec, false, out),
        P::Fillet(_) => out.push(A::Radius),
        P::FilletArc { spec, .. } => {
            out.push(A::Radius);
            spec_slots(spec, true, out);
        }
        P::ArcFillet { spec, .. } => {
            spec_slots(spec, false, out);
            out.push(A::Radius);
        }
        P::ArcFilletArc { spec, spec2, .. } => {
            spec_slots(spec, false, out);
            out.push(A::Radius);
            spec_slots(spec2, true, out);
        }
    }
}

/// Shared shape of the spec accessors — one table, two borrows.
/// `second` mirrors [`spec_slots`]'s role-twin selection.
macro_rules! spec_arg_access {
    ($spec:expr, $arg:expr, $second:expr, $($ref_kw:tt)*) => {{
        use ProgramArcData as S;
        use StepArg as A;
        match ($spec, $arg, $second) {
            (S::Radius { r, .. }, A::CarrierRadius, false)
            | (S::Radius { r, .. }, A::CarrierRadius2, true)
            | (S::Sweep { r, .. }, A::CarrierRadius, false)
            | (S::Sweep { r, .. }, A::CarrierRadius2, true)
            | (S::ArcLen { r, .. }, A::CarrierRadius, false)
            | (S::ArcLen { r, .. }, A::CarrierRadius2, true) => Some(r),
            (S::Bulge { b, .. }, A::Bulge, _) => Some(b),
            (S::Sweep { angle, .. }, A::SweepVal, _) => Some(angle),
            (S::ArcLen { len, .. }, A::ArcLenVal, _) => Some(len),
            (S::Via { q, .. }, A::ViaX, false) | (S::Via { q, .. }, A::Via2X, true) => {
                Some($($ref_kw)* q[0])
            }
            (S::Via { q, .. }, A::ViaY, false) | (S::Via { q, .. }, A::Via2Y, true) => {
                Some($($ref_kw)* q[1])
            }
            (S::Center { c, .. }, A::CenterX, false)
            | (S::Center { c, .. }, A::Center2X, true) => Some($($ref_kw)* c[0]),
            (S::Center { c, .. }, A::CenterY, false)
            | (S::Center { c, .. }, A::Center2Y, true) => Some($($ref_kw)* c[1]),
            (
                S::Bulge { target: ProgramTarget::Point(p), .. },
                A::TargetX,
                false,
            )
            | (S::Bulge { target: ProgramTarget::Point(p), .. }, A::Target2X, true)
            | (S::Via { target: ProgramTarget::Point(p), .. }, A::TargetX, false)
            | (S::Via { target: ProgramTarget::Point(p), .. }, A::Target2X, true)
            | (S::Center { target: ProgramTarget::Point(p), .. }, A::TargetX, false)
            | (S::Center { target: ProgramTarget::Point(p), .. }, A::Target2X, true) => {
                Some($($ref_kw)* p[0])
            }
            (
                S::Bulge { target: ProgramTarget::Point(p), .. },
                A::TargetY,
                false,
            )
            | (S::Bulge { target: ProgramTarget::Point(p), .. }, A::Target2Y, true)
            | (S::Via { target: ProgramTarget::Point(p), .. }, A::TargetY, false)
            | (S::Via { target: ProgramTarget::Point(p), .. }, A::Target2Y, true)
            | (S::Center { target: ProgramTarget::Point(p), .. }, A::TargetY, false)
            | (S::Center { target: ProgramTarget::Point(p), .. }, A::Target2Y, true) => {
                Some($($ref_kw)* p[1])
            }
            _ => None,
        }
    }};
}

fn spec_expr(spec: &ProgramArcData, arg: StepArg, second: bool) -> Option<&Expr> {
    spec_arg_access!(spec, arg, second, &)
}

fn spec_expr_mut(spec: &mut ProgramArcData, arg: StepArg, second: bool) -> Option<&mut Expr> {
    spec_arg_access!(spec, arg, second, &mut)
}

/// Shared shape of [`step_expr`]/[`step_expr_mut`] — one table, two
/// borrows, via a macro so the (step, arg) pairing is written once.
macro_rules! step_arg_access {
    ($step:expr, $arg:expr, $spec_fn:ident, $($ref_kw:tt)*) => {{
        use ProgramStep as P;
        use StepArg as A;
        match ($step, $arg) {
            (P::At(p), A::PointX) | (P::FarEndTo(p), A::PointX) => Some($($ref_kw)* p[0]),
            (P::At(p), A::PointY) | (P::FarEndTo(p), A::PointY) => Some($($ref_kw)* p[1]),
            (P::ArcContinue(p), A::TargetX) => Some($($ref_kw)* p[0]),
            (P::ArcContinue(p), A::TargetY) => Some($($ref_kw)* p[1]),
            (P::Angle(e), A::AngleVal) => Some(e),
            (P::Toward { dx, .. }, A::DirX) => Some(dx),
            (P::Toward { dy, .. }, A::DirY) => Some(dy),
            (P::Turn(e), A::TurnVal) => Some(e),
            (P::Line(e), A::Length) => Some(e),
            (P::LineTo(ProgramTarget::Point(p)), A::TargetX)
            | (P::ContinueTo(ProgramTarget::Point(p)), A::TargetX)
            | (P::TangentArcTo(ProgramTarget::Point(p)), A::TargetX) => Some($($ref_kw)* p[0]),
            (P::LineTo(ProgramTarget::Point(p)), A::TargetY)
            | (P::ContinueTo(ProgramTarget::Point(p)), A::TargetY)
            | (P::TangentArcTo(ProgramTarget::Point(p)), A::TargetY) => Some($($ref_kw)* p[1]),
            (P::ArcTo(spec), a) => $spec_fn(spec, a, false),
            (P::Fillet(e), A::Radius)
            | (P::FilletArc { radius: e, .. }, A::Radius)
            | (P::ArcFillet { radius: e, .. }, A::Radius)
            | (P::ArcFilletArc { radius: e, .. }, A::Radius) => Some(e),
            (P::FilletArc { spec, .. }, a) => $spec_fn(spec, a, true),
            (P::ArcFillet { spec, .. }, a) => $spec_fn(spec, a, false),
            (P::ArcFilletArc { spec, spec2, .. }, a) => {
                match $spec_fn(spec, a, false) {
                    Some(e) => Some(e),
                    None => $spec_fn(spec2, a, true),
                }
            }
            _ => None,
        }
    }};
}

/// The expression a (step, arg) pair addresses.
fn step_expr(step: &ProgramStep, arg: StepArg) -> Option<&Expr> {
    step_arg_access!(step, arg, spec_expr, &)
}

/// Mutable twin of [`step_expr`].
fn step_expr_mut(step: &mut ProgramStep, arg: StepArg) -> Option<&mut Expr> {
    step_arg_access!(step, arg, spec_expr_mut, &mut)
}

impl LoopProgram {
    /// This loop's argument roles per step, deterministic order.
    fn step_args(&self) -> Vec<(u32, StepArg)> {
        let mut out = Vec::new();
        match self {
            LoopProgram::Chain(steps) => {
                for (i, step) in steps.iter().enumerate() {
                    let mut args = Vec::new();
                    step_slots(step, &mut args);
                    out.extend(args.into_iter().map(|a| (i as u32, a)));
                }
            }
            LoopProgram::Circle { .. } => {
                out.extend([
                    (0, StepArg::CenterX),
                    (0, StepArg::CenterY),
                    (0, StepArg::Radius),
                ]);
            }
            LoopProgram::CircleSplit { .. } => {
                out.extend([
                    (0, StepArg::CenterX),
                    (0, StepArg::CenterY),
                    (0, StepArg::Radius),
                    (0, StepArg::Phase),
                ]);
            }
        }
        out
    }

    /// The expression at (step, arg), `None` off the loop.
    fn expr(&self, step: u32, arg: StepArg) -> Option<&Expr> {
        use StepArg as A;
        match self {
            LoopProgram::Chain(steps) => step_expr(steps.get(step as usize)?, arg),
            LoopProgram::Circle { centre, radius } if step == 0 => match arg {
                A::CenterX => Some(&centre[0]),
                A::CenterY => Some(&centre[1]),
                A::Radius => Some(radius),
                _ => None,
            },
            LoopProgram::CircleSplit {
                centre,
                radius,
                phase,
                ..
            } if step == 0 => match arg {
                A::CenterX => Some(&centre[0]),
                A::CenterY => Some(&centre[1]),
                A::Radius => Some(radius),
                A::Phase => Some(phase),
                _ => None,
            },
            _ => None,
        }
    }

    /// Mutable twin of [`LoopProgram::expr`].
    fn expr_mut(&mut self, step: u32, arg: StepArg) -> Option<&mut Expr> {
        use StepArg as A;
        match self {
            LoopProgram::Chain(steps) => step_expr_mut(steps.get_mut(step as usize)?, arg),
            LoopProgram::Circle { centre, radius } if step == 0 => match arg {
                A::CenterX => Some(&mut centre[0]),
                A::CenterY => Some(&mut centre[1]),
                A::Radius => Some(radius),
                _ => None,
            },
            LoopProgram::CircleSplit {
                centre,
                radius,
                phase,
                ..
            } if step == 0 => match arg {
                A::CenterX => Some(&mut centre[0]),
                A::CenterY => Some(&mut centre[1]),
                A::Radius => Some(radius),
                A::Phase => Some(phase),
                _ => None,
            },
            _ => None,
        }
    }
}

// ------------------------------------------------------------------
// Resolution (the C6 lane, plus the lift's second pass)
//
// Resolution itself is scalar-generic: an expression evaluates at
// whatever scalar its environment binds. What is C6-pinned is not the
// arithmetic but the STRUCTURE the resolved values then feed — which
// is why the second pass resolves at `T` and replays GUIDED, rather
// than replaying freely at `T`.
// ------------------------------------------------------------------

/// Resolves one expression at the resolution scalar, tagging failures
/// with the slot.
fn res<T: Decide>(
    e: &Expr,
    env: &ParamEnv<T>,
    loop_: u32,
    step: u32,
    arg: StepArg,
) -> Result<T, (SlotId, EvalError)> {
    eval::<T>(e, env).map_err(|source| (SlotId::Profile { loop_, step, arg }, source))
}

/// Resolves a target's expressions.
fn res_target<T: Decide>(
    t: &ProgramTarget,
    env: &ParamEnv<T>,
    loop_: u32,
    step: u32,
) -> Result<profile::Target<T>, (SlotId, EvalError)> {
    Ok(match t {
        ProgramTarget::Start => profile::Target::Start,
        ProgramTarget::StartArriving(a) => profile::Target::StartArriving(arrival(*a)),
        ProgramTarget::Point(p) => profile::Target::Point(Point2::new(
            res(&p[0], env, loop_, step, StepArg::TargetX)?,
            res(&p[1], env, loop_, step, StepArg::TargetY)?,
        )),
    })
}

/// Resolves one chain step to its scalar-valued mirror.
///
/// This is the direction the compiler cannot check: it MATCHES
/// [`ProgramStep`] and CONSTRUCTS a [`Step`], so a verb `profile`'s
/// table gains is invisible here. The census in
/// `tests/switch_program_vocabulary.rs` is what sees it.
fn res_step<T: Decide>(
    s: &ProgramStep,
    env: &ParamEnv<T>,
    loop_: u32,
    i: u32,
) -> Result<Step<T>, (SlotId, EvalError)> {
    use StepArg as A;
    let pt = |p: &[Expr; 2], ax: StepArg, ay: StepArg| -> Result<Point2<T>, _> {
        Ok(Point2::new(
            res(&p[0], env, loop_, i, ax)?,
            res(&p[1], env, loop_, i, ay)?,
        ))
    };
    Ok(match s {
        ProgramStep::At(p) => Step::At(pt(p, A::PointX, A::PointY)?),
        ProgramStep::Angle(e) => Step::Angle(res(e, env, loop_, i, A::AngleVal)?),
        ProgramStep::Toward { dx, dy } => Step::Toward {
            dx: res(dx, env, loop_, i, A::DirX)?,
            dy: res(dy, env, loop_, i, A::DirY)?,
        },
        ProgramStep::Tangent => Step::Tangent,
        ProgramStep::Cusp => Step::Cusp,
        ProgramStep::Turn(e) => Step::Turn(res(e, env, loop_, i, A::TurnVal)?),
        ProgramStep::Line(e) => Step::Line(res(e, env, loop_, i, A::Length)?),
        ProgramStep::LineTo(t) => Step::LineTo(res_target(t, env, loop_, i)?),
        ProgramStep::ContinueTo(t) => Step::ContinueTo(res_target(t, env, loop_, i)?),
        ProgramStep::ArcTo(spec) => Step::ArcTo(res_spec(spec, env, loop_, i, false)?),
        ProgramStep::TangentArcTo(t) => Step::TangentArcTo(res_target(t, env, loop_, i)?),
        ProgramStep::ArcContinue(p) => Step::ArcContinue(pt(p, A::TargetX, A::TargetY)?),
        ProgramStep::Fillet(e) => Step::Fillet {
            radius: res(e, env, loop_, i, A::Radius)?,
        },
        ProgramStep::FilletArc { radius, spec } => Step::FilletArc {
            radius: res(radius, env, loop_, i, A::Radius)?,
            spec: res_spec(spec, env, loop_, i, true)?,
        },
        ProgramStep::ArcFillet { spec, radius } => Step::ArcFillet {
            spec: res_spec(spec, env, loop_, i, false)?,
            radius: res(radius, env, loop_, i, A::Radius)?,
        },
        ProgramStep::ArcFilletArc {
            spec,
            radius,
            spec2,
        } => Step::ArcFilletArc {
            spec: res_spec(spec, env, loop_, i, false)?,
            radius: res(radius, env, loop_, i, A::Radius)?,
            spec2: res_spec(spec2, env, loop_, i, true)?,
        },
        ProgramStep::FarEndTo(p) => Step::FarEndTo(pt(p, A::PointX, A::PointY)?),
        ProgramStep::CloseTo => Step::CloseTo,
    })
}

/// Resolves an arc spec to its scalar-valued mirror (`second` selects
/// the spec₂ role twins, exactly as [`spec_slots`] enumerates them).
///
/// This is the hop the compiler cannot check in the direction that
/// matters: it matches the document vocabulary and CONSTRUCTS the
/// kernel one, so it stays well-typed while the kernel vocabulary
/// grows past it. The mode census keyed on `profile::ArcMode::ALL`
/// (`tests/switch_program_vocabulary.rs`) is what stands there, and it
/// checks both directions of the same arm: that every kernel mode is
/// reachable from a document spec, and that each one resolves to ITS
/// OWN mode rather than being laundered into a neighbour's.
fn res_spec<T: Decide>(
    spec: &ProgramArcData,
    env: &ParamEnv<T>,
    loop_: u32,
    i: u32,
    second: bool,
) -> Result<profile::ArcData<T>, (SlotId, EvalError)> {
    use StepArg as A;
    let pick = |a: StepArg, b: StepArg| if second { b } else { a };
    let pt2 = |p: &[Expr; 2], ax: StepArg, ay: StepArg| -> Result<Point2<T>, (SlotId, EvalError)> {
        Ok(Point2::new(
            res(&p[0], env, loop_, i, ax)?,
            res(&p[1], env, loop_, i, ay)?,
        ))
    };
    let tgt = |t: &ProgramTarget| -> Result<profile::Target<T>, (SlotId, EvalError)> {
        Ok(match t {
            ProgramTarget::Start => profile::Target::Start,
            ProgramTarget::StartArriving(a) => profile::Target::StartArriving(arrival(*a)),
            ProgramTarget::Point(p) => profile::Target::Point(pt2(
                p,
                pick(A::TargetX, A::Target2X),
                pick(A::TargetY, A::Target2Y),
            )?),
        })
    };
    Ok(match spec {
        ProgramArcData::Radius { r, side } => profile::ArcData::Radius {
            r: res(r, env, loop_, i, pick(A::CarrierRadius, A::CarrierRadius2))?,
            side: *side,
        },
        ProgramArcData::Bulge { target, b } => profile::ArcData::Bulge {
            target: tgt(target)?,
            b: res(b, env, loop_, i, A::Bulge)?,
        },
        ProgramArcData::Via { q, target } => profile::ArcData::Via {
            q: pt2(q, pick(A::ViaX, A::Via2X), pick(A::ViaY, A::Via2Y))?,
            target: tgt(target)?,
        },
        ProgramArcData::Center { c, winding, target } => profile::ArcData::Center {
            c: pt2(
                c,
                pick(A::CenterX, A::Center2X),
                pick(A::CenterY, A::Center2Y),
            )?,
            winding: *winding,
            target: tgt(target)?,
        },
        ProgramArcData::Sweep { r, side, angle } => profile::ArcData::Sweep {
            r: res(r, env, loop_, i, pick(A::CarrierRadius, A::CarrierRadius2))?,
            side: *side,
            angle: res(angle, env, loop_, i, A::SweepVal)?,
        },
        ProgramArcData::ArcLen { r, side, len } => profile::ArcData::ArcLen {
            r: res(r, env, loop_, i, pick(A::CarrierRadius, A::CarrierRadius2))?,
            side: *side,
            len: res(len, env, loop_, i, A::ArcLenVal)?,
        },
    })
}

impl LoopProgram {
    /// Resolves this loop's program at f64 (module docs: the verified
    /// asymmetry — profile geometry is f64-pinned; C6 structure
    /// selection must be lane-identical).
    ///
    /// # Errors
    ///
    /// The failing slot plus the evaluator's refusal, unaltered.
    pub fn resolve<T: Decide>(
        &self,
        env: &ParamEnv<T>,
        loop_: u32,
    ) -> Result<Vec<Step<T>>, (SlotId, EvalError)> {
        use StepArg as A;
        match self {
            LoopProgram::Chain(steps) => steps
                .iter()
                .enumerate()
                .map(|(i, s)| res_step(s, env, loop_, i as u32))
                .collect(),
            LoopProgram::Circle { centre, radius } => Ok(vec![Step::Circle {
                centre: Point2::new(
                    res(&centre[0], env, loop_, 0, A::CenterX)?,
                    res(&centre[1], env, loop_, 0, A::CenterY)?,
                ),
                radius: res(radius, env, loop_, 0, A::Radius)?,
            }]),
            LoopProgram::CircleSplit {
                centre,
                radius,
                n,
                phase,
            } => Ok(vec![Step::CircleSplit {
                centre: Point2::new(
                    res(&centre[0], env, loop_, 0, A::CenterX)?,
                    res(&centre[1], env, loop_, 0, A::CenterY)?,
                ),
                radius: res(radius, env, loop_, 0, A::Radius)?,
                n: *n as usize,
                phase: res(phase, env, loop_, 0, A::Phase)?,
            }]),
        }
    }
}

// ------------------------------------------------------------------
// ProfileProgram: resolution, equality, the payload impl
// ------------------------------------------------------------------

impl ProfileProgram {
    /// Resolves every loop at f64 — the evaluation pipeline's first
    /// stage (then `profile::replay` per loop, then embed + validate).
    ///
    /// # Errors
    ///
    /// The failing slot plus the evaluator's refusal, unaltered.
    pub fn resolve<T: Decide>(
        &self,
        env: &ParamEnv<T>,
    ) -> Result<Vec<Vec<Step<T>>>, (SlotId, EvalError)> {
        self.loops
            .iter()
            .enumerate()
            .map(|(li, lp)| lp.resolve(env, li as u32))
            .collect()
    }

    /// The authoring-time check's body (VQ9): resolve under `env`,
    /// replay every loop, validate the assembled profile — all under
    /// the run tolerance (VQ6: the same `Tolerance::get()` evaluation
    /// pins). Used by the edit door; evaluation re-runs the same
    /// ladder per binding with full typed errors.
    pub fn check(&self, env: &ParamEnv<f64>, tol: Tol) -> Result<(), ProgramRefusal> {
        let resolved = self
            .resolve(env)
            .map_err(|(slot, source)| ProgramRefusal::Resolve { slot, source })?;
        let mut loops = Vec::with_capacity(resolved.len());
        for (li, steps) in resolved.iter().enumerate() {
            let lp = profile::replay(steps, tol).map_err(|e| match e.kind {
                profile::ReplayErrorKind::Transition { state, verb } => {
                    ProgramRefusal::Transition {
                        loop_: li as u32,
                        step: e.step as u32,
                        state,
                        verb,
                    }
                }
                profile::ReplayErrorKind::Path(ref source) => ProgramRefusal::Geometry {
                    loop_: li as u32,
                    step: e.step as u32,
                    kind: source.kind(),
                    rendered: source.to_string(),
                },
            })?;
            loops.push(lp);
        }
        profile::Profile::new(self.plane, loops)
            .validate(tol)
            .map(|_| ())
            .map_err(ProgramRefusal::Validate)
    }
}

impl PartialEq for ProfileProgram {
    /// BIT equality (struct docs): plane floats by bits, expressions by
    /// [`Expr::bit_eq`], structure structurally.
    fn eq(&self, other: &Self) -> bool {
        plane_bits(&self.plane) == plane_bits(&other.plane)
            && self.loops.len() == other.loops.len()
            && self
                .loops
                .iter()
                .zip(&other.loops)
                .all(|(a, b)| loop_bit_eq(a, b))
    }
}

/// The 12 placement floats as bits, deterministic column order — also
/// the content key's plane feed (crate-internal).
pub(crate) fn plane_key_bits(p: &SketchPlane<f64>) -> [u64; 12] {
    plane_bits(p)
}

/// The 12 placement floats, as bits.
fn plane_bits(p: &SketchPlane<f64>) -> [u64; 12] {
    let a = &p.placement;
    let mut out = [0u64; 12];
    for (i, v) in [a.linear.c0, a.linear.c1, a.linear.c2, a.translation]
        .iter()
        .enumerate()
    {
        out[3 * i] = v.x.to_bits();
        out[3 * i + 1] = v.y.to_bits();
        out[3 * i + 2] = v.z.to_bits();
    }
    out
}

/// Structural equality with Exprs compared by bits.
fn loop_bit_eq(a: &LoopProgram, b: &LoopProgram) -> bool {
    match (a, b) {
        (LoopProgram::Chain(x), LoopProgram::Chain(y)) => {
            x.len() == y.len() && x.iter().zip(y).all(|(s, t)| step_bit_eq(s, t))
        }
        (
            LoopProgram::Circle {
                centre: ca,
                radius: ra,
            },
            LoopProgram::Circle {
                centre: cb,
                radius: rb,
            },
        ) => pair_bit_eq(ca, cb) && ra.bit_eq(rb),
        (
            LoopProgram::CircleSplit {
                centre: ca,
                radius: ra,
                n: na,
                phase: pa,
            },
            LoopProgram::CircleSplit {
                centre: cb,
                radius: rb,
                n: nb,
                phase: pb,
            },
        ) => pair_bit_eq(ca, cb) && ra.bit_eq(rb) && na == nb && pa.bit_eq(pb),
        // Different variants are unequal — spelled over the whole
        // vocabulary by first element rather than swept up by a
        // catch-all. That is what makes the answer for a variant added
        // to `LoopProgram` a compile error here: under a catch-all it
        // would compare unequal to ITSELF, and the D7 replay identity
        // and the document diff both read this answer, so a program
        // that never changed would report as changed. The same holds
        // for the three functions below.
        (
            LoopProgram::Chain(_) | LoopProgram::Circle { .. } | LoopProgram::CircleSplit { .. },
            _,
        ) => false,
    }
}

fn pair_bit_eq(a: &[Expr; 2], b: &[Expr; 2]) -> bool {
    a[0].bit_eq(&b[0]) && a[1].bit_eq(&b[1])
}

fn target_bit_eq(a: &ProgramTarget, b: &ProgramTarget) -> bool {
    match (a, b) {
        (ProgramTarget::Start, ProgramTarget::Start) => true,
        (ProgramTarget::StartArriving(x), ProgramTarget::StartArriving(y)) => x == y,
        (ProgramTarget::Point(x), ProgramTarget::Point(y)) => pair_bit_eq(x, y),
        (ProgramTarget::Start | ProgramTarget::StartArriving(_) | ProgramTarget::Point(_), _) => {
            false
        }
    }
}

/// The document arrival tag, lowered to the kernel's.
fn arrival(a: ProgramArrival) -> profile::Arrival {
    match a {
        ProgramArrival::Straight => profile::Arrival::Straight,
        ProgramArrival::Tangent => profile::Arrival::Tangent,
    }
}

/// The kernel arrival tag, lifted to the document's.
fn arrival_lit(a: &profile::Arrival) -> ProgramArrival {
    match a {
        profile::Arrival::Straight => ProgramArrival::Straight,
        profile::Arrival::Tangent => ProgramArrival::Tangent,
    }
}

fn spec_bit_eq(a: &ProgramArcData, b: &ProgramArcData) -> bool {
    use ProgramArcData as S;
    match (a, b) {
        (S::Radius { r: ra, side: sa }, S::Radius { r: rb, side: sb }) => ra.bit_eq(rb) && sa == sb,
        (S::Bulge { target: ta, b: ba }, S::Bulge { target: tb, b: bb }) => {
            target_bit_eq(ta, tb) && ba.bit_eq(bb)
        }
        (S::Via { q: qa, target: ta }, S::Via { q: qb, target: tb }) => {
            pair_bit_eq(qa, qb) && target_bit_eq(ta, tb)
        }
        (
            S::Center {
                c: ca,
                winding: wa,
                target: ta,
            },
            S::Center {
                c: cb,
                winding: wb,
                target: tb,
            },
        ) => pair_bit_eq(ca, cb) && wa == wb && target_bit_eq(ta, tb),
        (
            S::Sweep {
                r: ra,
                side: sa,
                angle: aa,
            },
            S::Sweep {
                r: rb,
                side: sb,
                angle: ab,
            },
        ) => ra.bit_eq(rb) && sa == sb && aa.bit_eq(ab),
        (
            S::ArcLen {
                r: ra,
                side: sa,
                len: la,
            },
            S::ArcLen {
                r: rb,
                side: sb,
                len: lb,
            },
        ) => ra.bit_eq(rb) && sa == sb && la.bit_eq(lb),
        (
            S::Radius { .. }
            | S::Bulge { .. }
            | S::Via { .. }
            | S::Center { .. }
            | S::Sweep { .. }
            | S::ArcLen { .. },
            _,
        ) => false,
    }
}

fn step_bit_eq(a: &ProgramStep, b: &ProgramStep) -> bool {
    use ProgramStep as P;
    match (a, b) {
        (P::At(x), P::At(y)) | (P::FarEndTo(x), P::FarEndTo(y)) => pair_bit_eq(x, y),
        (P::Angle(x), P::Angle(y))
        | (P::Turn(x), P::Turn(y))
        | (P::Line(x), P::Line(y))
        | (P::Fillet(x), P::Fillet(y)) => x.bit_eq(y),
        (P::Toward { dx: xa, dy: ya }, P::Toward { dx: xb, dy: yb }) => {
            xa.bit_eq(xb) && ya.bit_eq(yb)
        }
        (P::Tangent, P::Tangent) | (P::Cusp, P::Cusp) | (P::CloseTo, P::CloseTo) => true,
        (P::LineTo(x), P::LineTo(y))
        | (P::ContinueTo(x), P::ContinueTo(y))
        | (P::TangentArcTo(x), P::TangentArcTo(y)) => target_bit_eq(x, y),
        (P::ArcContinue(x), P::ArcContinue(y)) => pair_bit_eq(x, y),
        (P::ArcTo(x), P::ArcTo(y)) => spec_bit_eq(x, y),
        (
            P::FilletArc {
                radius: ra,
                spec: sa,
            },
            P::FilletArc {
                radius: rb,
                spec: sb,
            },
        ) => ra.bit_eq(rb) && spec_bit_eq(sa, sb),
        (
            P::ArcFillet {
                spec: sa,
                radius: ra,
            },
            P::ArcFillet {
                spec: sb,
                radius: rb,
            },
        ) => spec_bit_eq(sa, sb) && ra.bit_eq(rb),
        (
            P::ArcFilletArc {
                spec: sa,
                radius: ra,
                spec2: s2a,
            },
            P::ArcFilletArc {
                spec: sb,
                radius: rb,
                spec2: s2b,
            },
        ) => spec_bit_eq(sa, sb) && ra.bit_eq(rb) && spec_bit_eq(s2a, s2b),
        (
            P::At(_)
            | P::Angle(_)
            | P::Toward { .. }
            | P::Tangent
            | P::Cusp
            | P::Turn(_)
            | P::Line(_)
            | P::LineTo(_)
            | P::ContinueTo(_)
            | P::ArcTo(_)
            | P::TangentArcTo(_)
            | P::ArcContinue(_)
            | P::Fillet(_)
            | P::FilletArc { .. }
            | P::ArcFillet { .. }
            | P::ArcFilletArc { .. }
            | P::FarEndTo(_)
            | P::CloseTo,
            _,
        ) => false,
    }
}

impl ProfilePayload for ProfileProgram {
    fn slots(&self) -> Vec<SlotId> {
        let mut out = Vec::new();
        for (li, lp) in self.loops.iter().enumerate() {
            for (step, arg) in lp.step_args() {
                out.push(SlotId::Profile {
                    loop_: li as u32,
                    step,
                    arg,
                });
            }
        }
        out
    }

    fn expr(&self, slot: SlotId) -> Option<&Expr> {
        let SlotId::Profile { loop_, step, arg } = slot else {
            return None;
        };
        self.loops.get(loop_ as usize)?.expr(step, arg)
    }

    fn expr_mut(&mut self, slot: SlotId) -> Option<&mut Expr> {
        let SlotId::Profile { loop_, step, arg } = slot else {
            return None;
        };
        self.loops.get_mut(loop_ as usize)?.expr_mut(step, arg)
    }

    fn check(&self, env: &ParamEnv<f64>, tol: Tol) -> Result<(), ProgramRefusal> {
        ProfileProgram::check(self, env, tol)
    }
}

// ------------------------------------------------------------------
// Authoring helpers (VQ5: builder sugar expands AT AUTHORING into core
// steps — LineTo-class chains sharing Expr subtrees; nothing here adds
// program vocabulary)
// ------------------------------------------------------------------

/// A Length literal (canonical meters), for the literal-authoring
/// helpers below.
fn len_lit(v: f64) -> Result<Expr, DimensionError> {
    Expr::literal(v, Dimension::Length)
}

/// An Angle literal (canonical radians).
fn ang_lit(v: f64) -> Result<Expr, DimensionError> {
    Expr::literal(v, Dimension::Angle)
}

/// A dimensionless literal — bulges and director components.
fn scalar_lit(v: f64) -> Result<Expr, DimensionError> {
    Expr::literal(v, Dimension::Scalar)
}

/// A literal point.
fn pt_lit(p: &Point2<f64>) -> Result<[Expr; 2], DimensionError> {
    Ok([len_lit(p.x)?, len_lit(p.y)?])
}

/// A recorded target, lifted.
fn target_lit(t: &Target<f64>) -> Result<ProgramTarget, DimensionError> {
    Ok(match t {
        Target::Point(p) => ProgramTarget::Point(pt_lit(p)?),
        Target::Start => ProgramTarget::Start,
        Target::StartArriving(a) => ProgramTarget::StartArriving(arrival_lit(a)),
    })
}

/// Why a recorded PATHS program could not be lifted
/// ([`LoopProgram::from_recorded`]).
///
/// Every verb the transition table declares now has a document
/// spelling, so there is no vocabulary arm: `from_recorded` is
/// exhaustive on [`profile::Step`], and a verb the table gains breaks
/// this file at compile rather than reaching a typed refusal.
///
/// Two of the three arms are unreachable through the authoring
/// algebra — they exist because the door takes a `&[Step<f64>]`, which
/// a caller can also hand-build.
#[derive(Debug, Clone, PartialEq)]
pub enum RecordedProgramError {
    /// A literal argument the expression layer refused.
    Literal(DimensionError),
    /// A subdivision count too large for the program's `u32` field.
    /// Unreachable from `circle_split`, whose vertices would exhaust
    /// memory first.
    SubdivisionCount(usize),
    /// A complete-loop carrier step recorded inside a chain.
    /// Unreachable from the algebra: `circle` and `circle_split` are
    /// one-step programs that bind nothing and continue into nothing.
    CarrierInChain,
}

impl From<DimensionError> for RecordedProgramError {
    fn from(err: DimensionError) -> Self {
        Self::Literal(err)
    }
}

impl core::fmt::Display for RecordedProgramError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Literal(err) => write!(f, "a recorded literal was refused: {err}"),
            Self::SubdivisionCount(n) => {
                write!(f, "the subdivision count {n} does not fit a u32")
            }
            Self::CarrierInChain => {
                write!(f, "a complete-loop carrier step appears inside a chain")
            }
        }
    }
}

impl core::error::Error for RecordedProgramError {}

/// A recorded arc spec at literal arguments.
fn spec_lit(spec: &profile::ArcData<f64>) -> Result<ProgramArcData, RecordedProgramError> {
    Ok(match spec {
        profile::ArcData::Radius { r, side } => ProgramArcData::Radius {
            r: len_lit(*r)?,
            side: *side,
        },
        profile::ArcData::Bulge { target, b } => ProgramArcData::Bulge {
            target: target_lit(target)?,
            b: scalar_lit(*b)?,
        },
        profile::ArcData::Via { q, target } => ProgramArcData::Via {
            q: pt_lit(q)?,
            target: target_lit(target)?,
        },
        profile::ArcData::Center { c, winding, target } => ProgramArcData::Center {
            c: pt_lit(c)?,
            winding: *winding,
            target: target_lit(target)?,
        },
        profile::ArcData::Sweep { r, side, angle } => ProgramArcData::Sweep {
            r: len_lit(*r)?,
            side: *side,
            angle: ang_lit(*angle)?,
        },
        profile::ArcData::ArcLen { r, side, len } => ProgramArcData::ArcLen {
            r: len_lit(*r)?,
            side: *side,
            len: len_lit(*len)?,
        },
    })
}

impl LoopProgram {
    /// A literal polygon: `At(p0)`, `LineTo(p1)`, …, `LineTo(Start)` —
    /// the VQ5 expansion of the polygon builder, at literal points
    /// (corpus/fixture authoring; parametric authors write the steps
    /// with their own Exprs).
    ///
    /// # Errors
    ///
    /// A non-finite coordinate (the literal door's refusal).
    pub fn polygon(points: impl IntoIterator<Item = (f64, f64)>) -> Result<Self, DimensionError> {
        let mut steps = Vec::new();
        for (i, (x, y)) in points.into_iter().enumerate() {
            let p = [len_lit(x)?, len_lit(y)?];
            steps.push(if i == 0 {
                ProgramStep::At(p)
            } else {
                ProgramStep::LineTo(ProgramTarget::Point(p))
            });
        }
        steps.push(ProgramStep::LineTo(ProgramTarget::Start));
        Ok(LoopProgram::Chain(steps))
    }

    /// Lift a RECORDED PATHS program to its document form — the
    /// inverse of [`LoopProgram::resolve`] at literal arguments.
    ///
    /// A `ClosedLoop`'s `program` is the verbs the author wrote, with
    /// the arguments they wrote (`profile::Step` stores authored data
    /// only — nothing derived), so this is a verb-for-verb,
    /// argument-for-argument re-spelling into the Expr-bearing
    /// vocabulary, never a second lowering: the via point, the centre
    /// and the winding ride through untouched and the bulge is derived
    /// again at replay. Dimensions come from V2's table (coordinates,
    /// lengths and radii `Length`; angle, turn and phase `Angle`;
    /// bulge and director components `Scalar`).
    ///
    /// This is the seam between the two authoring surfaces: it is what
    /// lets a chain written in the PATHS algebra become a
    /// [`ProfileProgram`] node, in either host language. Parametric
    /// authors still write the steps with their own `Expr`s — a
    /// recorded program is literal by construction.
    ///
    /// The chain-vs-carrier distinction is the enum, so the one-step
    /// complete-loop forms land in their own arms.
    ///
    /// # Errors
    ///
    /// [`RecordedProgramError`] — a refused literal, or (only from a
    /// hand-built slice) a count that overflows `u32` or a carrier
    /// step inside a chain.
    pub fn from_recorded(steps: &[Step<f64>]) -> Result<Self, RecordedProgramError> {
        if let [Step::Circle { centre, radius }] = steps {
            return Ok(Self::Circle {
                centre: pt_lit(centre)?,
                radius: len_lit(*radius)?,
            });
        }
        if let [
            Step::CircleSplit {
                centre,
                radius,
                n,
                phase,
            },
        ] = steps
        {
            return Ok(Self::CircleSplit {
                centre: pt_lit(centre)?,
                radius: len_lit(*radius)?,
                n: u32::try_from(*n).map_err(|_| RecordedProgramError::SubdivisionCount(*n))?,
                phase: ang_lit(*phase)?,
            });
        }

        let mut out = Vec::with_capacity(steps.len());
        for step in steps {
            out.push(match step {
                Step::At(p) => ProgramStep::At(pt_lit(p)?),
                Step::Angle(theta) => ProgramStep::Angle(ang_lit(*theta)?),
                Step::Toward { dx, dy } => ProgramStep::Toward {
                    dx: scalar_lit(*dx)?,
                    dy: scalar_lit(*dy)?,
                },
                Step::Tangent => ProgramStep::Tangent,
                Step::Cusp => ProgramStep::Cusp,
                Step::Turn(delta) => ProgramStep::Turn(ang_lit(*delta)?),
                Step::Line(len) => ProgramStep::Line(len_lit(*len)?),
                Step::LineTo(t) => ProgramStep::LineTo(target_lit(t)?),
                Step::ContinueTo(t) => ProgramStep::ContinueTo(target_lit(t)?),
                Step::ArcTo(spec) => ProgramStep::ArcTo(spec_lit(spec)?),
                Step::TangentArcTo(t) => ProgramStep::TangentArcTo(target_lit(t)?),
                Step::ArcContinue(p) => ProgramStep::ArcContinue(pt_lit(p)?),
                Step::Fillet { radius } => ProgramStep::Fillet(len_lit(*radius)?),
                Step::FilletArc { radius, spec } => ProgramStep::FilletArc {
                    radius: len_lit(*radius)?,
                    spec: spec_lit(spec)?,
                },
                Step::ArcFillet { spec, radius } => ProgramStep::ArcFillet {
                    spec: spec_lit(spec)?,
                    radius: len_lit(*radius)?,
                },
                Step::ArcFilletArc {
                    spec,
                    radius,
                    spec2,
                } => ProgramStep::ArcFilletArc {
                    spec: spec_lit(spec)?,
                    radius: len_lit(*radius)?,
                    spec2: spec_lit(spec2)?,
                },
                Step::FarEndTo(p) => ProgramStep::FarEndTo(pt_lit(p)?),
                Step::CloseTo => ProgramStep::CloseTo,
                Step::Circle { .. } | Step::CircleSplit { .. } => {
                    return Err(RecordedProgramError::CarrierInChain);
                }
            });
        }
        Ok(Self::Chain(out))
    }

    /// A literal circle loop.
    ///
    /// # Errors
    ///
    /// A non-finite argument.
    pub fn circle(cx: f64, cy: f64, r: f64) -> Result<Self, DimensionError> {
        Ok(LoopProgram::Circle {
            centre: [len_lit(cx)?, len_lit(cy)?],
            radius: len_lit(r)?,
        })
    }

    /// A literal declared-subdivision circle loop.
    ///
    /// # Errors
    ///
    /// A non-finite argument.
    pub fn circle_split(
        cx: f64,
        cy: f64,
        r: f64,
        n: u32,
        phase: f64,
    ) -> Result<Self, DimensionError> {
        Ok(LoopProgram::CircleSplit {
            centre: [len_lit(cx)?, len_lit(cy)?],
            radius: len_lit(r)?,
            n,
            phase: Expr::literal(phase, Dimension::Angle)?,
        })
    }
}
