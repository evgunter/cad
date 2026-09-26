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
use profile::{ArcSweep, Step, Target};
use serde::{Deserialize, Serialize};

use crate::doc::ParamName;
use crate::eval::{CanonicalSegment, LoopAnchor, ProfileNaming};
use crate::expr::{Dimension, DimensionError, EvalError, Expr, ParamEnv, UnitSym, eval};
use crate::node::{RecipeNodeId, SlotId, StepArg, StepId};
use geom_core::Tol;

/// **One declaration, two projections** — a document vocabulary's enum,
/// and the variant names it declares.
///
/// `profile` declares each of its three vocabularies once and projects
/// `Verb::ALL`, `ArcMode::ALL` and `TargetKind::ALL` from the same
/// declaration, so a census keyed on one grows with the vocabulary
/// rather than behind it. The three enums below are those vocabularies'
/// SECOND spelling — G1 layering keeps expressions and serde out of the
/// kernel crate — and they had no such projection.
///
/// What that cost is one direction of the construct hop. A variant
/// added to a document enum is forced through every match that consumes
/// it, so it cannot ship un-noticed; but every one of those arms may
/// legally resolve it into an EXISTING kernel form, and when one does
/// the kernel-anchored censuses stay green (`Verb::ALL`, `ArcMode::ALL`
/// and `TargetKind::ALL` are all still fully witnessed) while the
/// document form silently authors something nobody wrote.
///
/// `ALL_NAMES` is the anchor for that direction. It is derived from the
/// declaration at COMPILE time, so there is nothing to keep in step and
/// **nothing in FRONT of a variant's name can hide it** — a doc comment,
/// an attribute, a `cfg`, a `cfg_attr`, any stack of them: the macro
/// captures them as metas and projects the name behind them.
/// `tests/switch_program_vocabulary.rs` is keyed on it, and a variant
/// that reaches no witness there reds.
///
/// Two places where the projection and the witness side disagree, both
/// in the LOUD direction, because a census that is wrong quietly is the
/// thing this replaced:
///
/// - a `#[cfg]` that gates a variant OUT removes it from the enum and
///   leaves it in `ALL_NAMES` — the metas ride the variant, not the
///   name list. So the projection is a SUPERSET under `cfg`: it
///   over-demands a witness for a variant that is not there and reds.
///   It cannot hide one.
/// - a RAW IDENTIFIER projects as `stringify!` writes it (`r#type`)
///   while the witness side reads a `Debug` rendering (`type`), so a
///   correctly declared and correctly witnessed raw-ident variant would
///   red spuriously. No variant here is one; if one arrives, the fix is
///   to strip the `r#` on one side, and this note is the reason the red
///   will make sense.
macro_rules! document_vocabulary {
    (
        $(
            $(#[$enum_meta:meta])*
            $vis:vis enum $name:ident {
                $(
                    $(#[$variant_meta:meta])*
                    $variant:ident $(( $($tuple:tt)* ))? $({ $($named:tt)* })?
                ),* $(,)?
            }
        )*
    ) => {
        $(
            $(#[$enum_meta])*
            $vis enum $name {
                $(
                    $(#[$variant_meta])*
                    $variant $(( $($tuple)* ))? $({ $($named)* })?
                ),*
            }

            impl $name {
                /// Every variant this vocabulary declares, in
                /// declaration order — projected from the same
                /// declaration as the variants, so a census keyed on it
                /// grows with the vocabulary rather than behind it.
                #[doc(hidden)]
                pub const ALL_NAMES: &'static [&'static str] = &[$(stringify!($variant)),*];
            }
        )*

        /// **Every document vocabulary, projected rather than typed.**
        ///
        /// `ALL_NAMES` closes *a variant arrives without a witness*.
        /// This closes the same failure one level up — *a VOCABULARY
        /// arrives without a census* — and it has to be closed the same
        /// way, because a roster typed out on the test side is a second
        /// list kept in step with this one by hand, which is the defect
        /// the whole macro exists to remove.
        ///
        /// It is projected from the single invocation below, and that
        /// invocation is single by construction **within this module**:
        /// the constant is emitted once per invocation, so a second
        /// invocation in the same module is an `E0428` duplicate. The
        /// qualifier is load-bearing — `E0428` is scoped to one module's
        /// value namespace, so a second invocation inside a CHILD module
        /// compiles clean and projects a second roster that the census
        /// never reads. `program.rs` has no child modules, so the list
        /// is complete today; what makes it complete is that fact and
        /// not the macro.
        ///
        /// **What it does not cover, stated, with the live instance
        /// named:** an enum declared with a plain `pub enum` rather than
        /// through this macro has no `ALL_NAMES`, is absent from this
        /// list, and nothing detects that it should have been in either.
        /// This file holds two such enums today — [`ProgramRefusal`] and
        /// [`RecordedProgramError`] — and they are deliberately out:
        /// they are the REFUSAL and ERROR vocabularies, produced by this
        /// crate for a caller to read, with no construct hop that builds
        /// a kernel form out of them and so no laundering direction to
        /// guard. [`LoopProgram`] was the third, and it is IN: its
        /// `resolve` is a construct hop like the other three. The test
        /// for membership is that construct hop, not the naming.
        /// Closing the general case needs a walk over the file's
        /// declarations, which is a text scan, which is what this macro
        /// replaced and for a reason. Filed:
        /// `work/census/a-document-vocabulary-declared-outside-the-macro-is-uncensused.md`.
        ///
        /// **Cost, stated:** rustfmt does not format the body of a macro
        /// invocation, so every declaration below is outside its reach
        /// and no gate will report drift there. The trade was taken
        /// deliberately — the macro closes a SILENT failure class and
        /// formatting drift is visible to any reader — but it is a real
        /// cost and it is not detected. Filed:
        /// `work/ciw/rustfmt-does-not-reach-a-macro-wrapped-declaration-block.md`.
        #[doc(hidden)]
        pub const DOCUMENT_VOCABULARIES: &[(&str, &[&str])] =
            &[$((stringify!($name), $name::ALL_NAMES)),*];
    };
}

document_vocabulary! {
/// Where a target-taking step ends: an authored point (two Length
/// expressions) or the entry vertex (`Start` — structural; targeting it
/// closes the loop). Mirrors `profile::Target`.
///
/// `res_target` matches THIS vocabulary and constructs
/// [`profile::Target`], so a form added here alone can be resolved into
/// an existing kernel form and never be seen. [`Self::ALL_NAMES`] is
/// what forces it to reach a witness instead.
// No `deny_unknown_fields`: no variant here has a NAMED field, so the
// attribute would have nothing to deny (`work/census/`'s rule; the
// repo-wide census in `test-utils` reds on an inert one).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ProgramTarget {
    /// An authored absolute point in the profile frame.
    Point([Expr; 2]),
    /// The entry vertex: this step closes the loop.
    Start,
    /// The entry vertex with the seam's TANGENT JOINT declared — a
    /// structural tag, no expressions (so it contributes no slots) and
    /// no payload: there is exactly one declaration to make there.
    StartArriving,
}

/// One Expr-bearing recorded verb — the document-layer mirror of
/// [`profile::Step`], structural tags literal, continuous args [`Expr`]
/// (V2's table: coordinates/lengths/radii `Length`, angle/turn/phase
/// `Angle`, bulge and director components `Scalar`).
///
/// It is a second spelling of a vocabulary `profile` declares once,
/// and it has to be: a step here carries `Expr`s and serializes, and
/// G1 layering keeps both out of the kernel crate. It is the SECOND
/// and last: this type is also the persisted form, so a verb added
/// here is a FORMAT change and the persisted spelling of every verb
/// is pinned as literals in `tests/switch_program_vocabulary.rs` —
/// the one thing on this wire that renaming a variant does not move
/// with itself.
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
///
/// The mirror direction is [`Self::ALL_NAMES`]'s: `res_step` matches
/// THIS vocabulary and constructs [`profile::Step`], so a verb added
/// here alone can be resolved into an existing kernel verb, leaving
/// `Verb::ALL` fully witnessed and the document verb unexercised.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
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
/// refusal arm and a tag, while this enum — which is the wire — and
/// the expression-slot roles stay short: the hop that would need them,
/// `res_spec`, matches THIS type and CONSTRUCTS the kernel one, so it
/// keeps compiling. What forces arrival is the mode census in
/// `tests/switch_program_vocabulary.rs`, keyed on
/// [`profile::ArcMode::ALL`]: its witness is a match on the mode tag,
/// so a mode with no document spelling is a compile error there.
///
/// That census is keyed on the KERNEL vocabulary and says nothing about
/// a mode added HERE alone — `res_spec` would resolve it into an
/// existing kernel mode and every clause keyed on `ArcMode::ALL` would
/// stay green. [`Self::ALL_NAMES`] is that direction's anchor.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum ProgramArcData {
    /// `Radius { r, side }` — arrival mode, centre derived.
    Radius {
        /// The carrier radius.
        r: Expr,
        /// Which side of the tangent the centre sits on (structural).
        #[serde(with = "crate::persist::wire::arc_side")]
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
        #[serde(with = "crate::persist::wire::arc_sweep")]
        winding: ArcSweep,
        /// The authored anchor/endpoint (`Start` closes).
        target: ProgramTarget,
    },
    /// `Sweep { r, side, angle }` — endpoint derived at replay.
    Sweep {
        /// The carrier radius.
        r: Expr,
        /// Which side the centre sits on (structural).
        #[serde(with = "crate::persist::wire::arc_side")]
        side: profile::ArcSide,
        /// The swept central angle.
        angle: Expr,
    },
    /// `ArcLen { r, side, len }` — endpoint derived at replay.
    ArcLen {
        /// The carrier radius.
        r: Expr,
        /// Which side the centre sits on (structural).
        #[serde(with = "crate::persist::wire::arc_side")]
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
///
/// **It is a document vocabulary, and [`Self::resolve`] is its construct
/// hop**, the fourth one: it matches THIS enum and builds
/// `Step::Circle` / `Step::CircleSplit`, so a carrier form added here
/// alone can be resolved into an existing kernel step and never be
/// seen. That is [`ProgramStep`]'s hazard one level out, and
/// [`Self::ALL_NAMES`] is its anchor for the same reason.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
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
}

/// The profile node's payload: the sketch frame it is drawn on, named
/// as a document NODE, plus the loop programs, outer first then holes
/// in description order.
///
/// # The plane is a reference, not a placement
///
/// This field held a `SketchPlane<f64>` — twelve placement floats
/// inline, unshared and unnameable. It now names a frame node — a
/// [`crate::Datum::Frame`], or a [`crate::Datum::FaceFrame`] derived
/// from a face — which is the whole of what the frame
/// datum was added for: two profiles on one face are two references to
/// one frame rather than two copies of a placement that can silently
/// drift apart, an axis can be declared to lie IN a named frame, and a
/// plane a person can see in the viewport is the plane they draw on.
///
/// The consequence to know when reading the rest of this crate: a
/// profile is no longer a DAG leaf. [`crate::Node::inputs`] reports
/// the frame, so evaluation orders it first, poison propagates through
/// it, the content key takes it as an upstream key rather than as
/// inline bits, and `roots::on_insert` transfers the frame's tip when
/// a profile consumes it.
///
/// # Equality is BIT equality
///
/// `PartialEq` compares the frame by NODE IDENTITY and expressions by
/// [`Expr::bit_eq`] — the canonical payload's equality IS the D7
/// replay-identity comparator. Comparing the id rather than the
/// placement it resolves to is the same claim the rest of the DAG
/// makes about its edges: two profiles on two frames that happen to
/// coincide today are two different documents, because editing one
/// frame moves only one of them. Display units are invisible to it
/// (they are invisible to `bit_eq` itself, D7).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProfileProgram {
    /// The frame node this profile is drawn on — a
    /// [`crate::Datum::Frame`] or a [`crate::Datum::FaceFrame`], either
    /// of which lands the same frame value: sketch (0, 0) and the
    /// directions sketch +x and +y point.
    ///
    /// Typed as a plain node reference rather than a frame-only
    /// newtype for the reason every other operand reference here is:
    /// what a reference DENOTES is the evaluator's question, answered
    /// once at the door with a typed refusal, not the recipe
    /// vocabulary's.
    ///
    /// **It reads through a door, not through `u64`'s own
    /// `Deserialize`.** A document written before the sketch plane
    /// became a node carries a twelve-float placement OBJECT here, and
    /// serde's own report for that — `invalid type: map, expected u64`
    /// — says nothing about which field of which node changed shape,
    /// which is the whole job of an `Unreadable` refusal. `plane_ref`'s
    /// visitor names the placement in its `expecting`. `deny_unknown_fields`
    /// above is not what fires: `plane` is a field this build knows, so
    /// the refusal is the field type's. No migration, by `persist`'s
    /// ruling — nothing has shipped and every checked-in document is
    /// regenerable.
    #[serde(deserialize_with = "crate::persist::wire::plane_ref")]
    pub plane: RecipeNodeId,
    /// The loop programs.
    pub loops: Vec<LoopProgram>,
    /// **Every authored step's minted id**, per loop and per step in
    /// program order ([`LoopProgram::authored_steps`] of them per loop):
    /// what a profile piece's name spells (`names/README.md`, "N1, the
    /// profile pieces").
    ///
    /// The document's edit doors mint them from its monotone step
    /// counter — `InsertNode` every one, `SetProgram` each step it does
    /// not keep — so a program on its way IN carries none (`InsertNode`
    /// refuses one that does), and a program at rest carries exactly
    /// one per step, unique across the document, which the load door
    /// checks.
    pub ids: Vec<Vec<StepId>>,
}

/// The canonical `Doc` instantiation (the retired `ProfileDesc` seat).
pub type ProfileDoc = crate::doc::Doc<ProfileProgram>;

/// **What a replay of a whole program produces**: each loop as
/// replayed at f64 and its structure record, both in program order
/// and one per loop — the pair [`ProfileProgram::check`] and
/// [`ProfileProgram::pieces`] read.
type Replayed = (
    Vec<profile::ProfileLoop<f64>>,
    Vec<profile::ReplayStructure>,
);

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
    /// Whether any expression of this program reads the document
    /// parameter `name` — over [`ProfilePayload::slots`], so every
    /// payload answers it the one way.
    fn references(&self, name: &ParamName) -> bool {
        let mut refs = Vec::new();
        for slot in self.slots() {
            if let Some(e) = self.expr(slot) {
                e.param_refs(&mut refs);
            }
        }
        refs.iter().any(|(n, _)| n == name)
    }
    /// The authoring-time check (VQ9): resolve + replay + validate
    /// under the CURRENT parameter environment, refusing typed at the
    /// edit door. The evaluation-time twin re-checks under every
    /// binding that is ever evaluated. A payload with no program has
    /// nothing to check.
    fn check(&self, _env: &ParamEnv<f64>, _tol: Tol) -> Result<(), ProgramRefusal> {
        Ok(())
    }
    /// **The loop programs this payload holds**, program order — the
    /// content [`crate::DocEdit::SetProgram`] replaces. `None` for a
    /// payload with no program, which is every `Doc<P>` test payload.
    fn loops(&self) -> Option<&[LoopProgram]> {
        None
    }
    /// **Every authored step's minted id**, per loop, program order —
    /// what the whole-program edit door keeps and drops. `None` for a
    /// payload with no program.
    fn step_ids(&self) -> Option<&[Vec<StepId>]> {
        None
    }
    /// **This payload with its loops and their step ids replaced
    /// whole** and everything else it holds — the plane — kept: the
    /// value the whole-program edit door writes. `None` where there is
    /// no program to replace, which is the same payloads
    /// [`ProfilePayload::loops`] answers `None` for.
    fn with_program(&self, _loops: Vec<LoopProgram>, _ids: Vec<Vec<StepId>>) -> Option<Self>
    where
        Self: Sized,
    {
        None
    }
    /// **Mints an id for every authored step** from the document's step
    /// counter, advancing it: the insert door's half of N1's minting. A
    /// payload with no program mints nothing.
    ///
    /// # Errors
    ///
    /// [`StepIdFault::Preminted`] where the program already carries
    /// ids.
    fn mint_step_ids(&mut self, _counter: &mut u64) -> Result<(), StepIdFault> {
        Ok(())
    }
    /// **The document node this payload is drawn ON**, if it names one
    /// — the profile's one DAG edge.
    ///
    /// It rides the payload trait rather than [`crate::Node::Profile`]
    /// because that is where the plane already lived: the variant
    /// stays a one-field tuple, and the payload answers for its own
    /// content. [`crate::Node::inputs`] reads this, so a payload that
    /// names a node and does not report it here would be a node the
    /// evaluator never waits for and the cascade never deletes.
    ///
    /// `None` by default, which is the honest answer for `Doc<P>`'s
    /// slot-free test payloads: they carry no plane at all.
    fn plane_input(&self) -> Option<crate::RecipeNodeId> {
        None
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
    /// The program carries no step ids, so its pieces have no names:
    /// ids are minted by the document's insert door, and a program
    /// outside a document has none (`names/README.md`, "N1, the
    /// profile pieces"). Answered only by [`ProfileProgram::pieces`].
    Unminted,
    /// The program's ids are not one list per loop and one id per
    /// authored step ([`StepIdFault::LoopCount`],
    /// [`StepIdFault::Shape`]), so they name no step reliably.
    /// Answered only by [`ProfileProgram::pieces`].
    StepIds(StepIdFault),
    /// The program's replay record and its naming anchor do not
    /// describe one program, so its pieces cannot be named — a kernel
    /// bug. Answered only by [`ProfileProgram::pieces`].
    Pieces(crate::eval::PiecesFault),
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
                write!(
                    f,
                    "a program expression failed to resolve at slot {}",
                    slot.label()
                )
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
            Self::Unminted => f.write_str(
                "the program carries no step ids, so its pieces have no names; a document's \
                 insert door mints them",
            ),
            Self::StepIds(fault) => write!(f, "the program's step ids are refused: {fault}"),
            Self::Pieces(fault) => write!(f, "the program's pieces have no names: {fault}"),
        }
    }
}

impl core::error::Error for ProgramRefusal {}

/// One loop's program and replay record, checked against each other
/// — what [`ProfileProgram::checked_records`] hands its two doors so
/// they answer through ONE permutation.
pub(crate) struct CheckedRecords<'p, 'r> {
    /// The loop's program.
    program: &'p LoopProgram,
    /// The loop's replay record: the per-step spans and the per-radius
    /// emissions.
    replay: &'r profile::ReplayStructure,
    /// How many segments the loop has.
    segments: usize,
}

/// [`CheckedRecords`] with the loop's place in the published
/// numbering: what [`ProfileProgram::checked_records`] hands its two
/// doors, and the only thing that mints a published ref, so a ref
/// minted without an anchor is not a state the types admit.
struct AnchoredRecords<'p, 'r> {
    /// The loop's records, checked for shape.
    records: CheckedRecords<'p, 'r>,
    /// The loop's position in the CANONICAL loop order (outer first,
    /// then holes in authored order) — the loop index published names
    /// carry.
    canonical_loop: u32,
    /// The loop's naming anchor, program segment ↔ canonical segment,
    /// already checked against canonicalization's own record.
    anchor: LoopAnchor,
}

impl<'p, 'r> CheckedRecords<'p, 'r> {
    /// The pair checked for SHAPE: a record of the right LENGTH is
    /// what makes a step index mean the same thing on both sides. A
    /// record from another program can have the right loop count and
    /// the wrong step count, and then every answer read through it is
    /// about somebody else's program. The profile side guards its own
    /// records this way (`StructureRefusal::shape`).
    ///
    /// `segments` is how many segments the loop the record describes
    /// has — the length of the canonical loop its spans index into
    /// ([`ProfileProgram::checked_records`]), which canonicalization
    /// reindexes exactly from the replayed one and never lengthens.
    ///
    /// # Errors
    ///
    /// [`StepSegmentsError::RecordShape`] where the record and the
    /// program author different step counts.
    pub(crate) fn new(
        loop_: u32,
        program: &'p LoopProgram,
        replay: &'r profile::ReplayStructure,
        segments: usize,
    ) -> Result<Self, StepSegmentsError> {
        let authored = program.authored_steps();
        if replay.steps.len() != authored {
            return Err(StepSegmentsError::RecordShape {
                loop_,
                authored,
                recorded: replay.steps.len(),
            });
        }
        Ok(Self {
            program,
            replay,
            segments,
        })
    }
}

impl CheckedRecords<'_, '_> {
    /// One step's recorded span, checked against the loop's length.
    ///
    /// # Errors
    ///
    /// [`StepSegmentsError::NoSuchStep`] where the record has no such
    /// step, [`StepSegmentsError::SpanOffTheLoop`] where the span it
    /// does have reaches past the loop's last segment.
    pub(crate) fn span_of(&self, step: u32) -> Result<profile::StepSpan, StepSegmentsError> {
        let span = *self
            .replay
            .steps
            .get(step as usize)
            .ok_or(StepSegmentsError::NoSuchStep {
                steps: self.replay.steps.len(),
            })?;
        if span.end() > self.segments {
            return Err(StepSegmentsError::SpanOffTheLoop {
                step,
                end: span.end(),
                segments: self.segments,
            });
        }
        Ok(span)
    }
}

impl AnchoredRecords<'_, '_> {
    /// The published ref naming PROGRAM segment `segment` of this loop:
    /// the canonical segment it is. Published names carry canonical
    /// indices, and the canonical start is the authored one, so the
    /// hop is the identity for a loop authored in its canonical sense
    /// and the reflection `s ↦ n − 1 − s` for one authored against it
    /// — read off the anchor, which
    /// [`ProfileProgram::checked_records`] has already checked against
    /// canonicalization's own record of the permutation.
    ///
    /// The BOUND is the caller's, because the two callers hold
    /// different facts and owe the reader different sentences: a span
    /// is checked by [`CheckedRecords::span_of`] and an emission by
    /// the arm that reads it. Both check before they get here, so a
    /// segment past the end is a caller that forgot — the assertion
    /// below, not a refusal this function invents a payload for.
    fn edge_of(&self, segment: usize) -> CanonicalSegment {
        debug_assert!(
            segment < self.records.segments,
            "an unchecked segment reached `edge_of`: {segment} of {}",
            self.records.segments
        );
        CanonicalSegment {
            loop_index: self.canonical_loop,
            segment: self.anchor.canonical_segment(program_index(segment)),
        }
    }

    /// Every published ref of `step`'s recorded span — ONE walk over
    /// one checked span, which is what both of the doors below answer
    /// a step-addressed question through.
    ///
    /// # Errors
    ///
    /// [`CheckedRecords::span_of`]'s.
    fn edges_of_step(&self, step: u32) -> Result<Vec<CanonicalSegment>, StepSegmentsError> {
        Ok(self
            .records
            .span_of(step)?
            .iter()
            .map(|s| self.edge_of(s))
            .collect())
    }
}

/// Why [`ProfileProgram::profile_edges_of`] could not name a
/// step's profile edges.
///
/// Every arm is a question the door cannot answer, not a geometry that
/// refused: an address this program does not have, or a record that
/// does not describe this program. There is no arm for "probably
/// these" — a map that guessed would be exactly the second derivation
/// the door exists to replace. There is no arm either for the
/// evaluation's two records of one permutation disagreeing: one
/// evaluation produces both, so a disagreement is the kernel
/// contradicting itself and the door asserts (DM8).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepSegmentsError {
    /// The program has no such loop.
    NoSuchLoop {
        /// How many loops it does have.
        loops: usize,
    },
    /// The loop has no such step. Carrier forms (`circle`,
    /// `circle_split`) have exactly one, numbered 0 — the same step the
    /// slot vocabulary addresses their arguments at.
    NoSuchStep {
        /// How many steps the loop's record describes.
        steps: usize,
    },
    /// The structure record does not cover this loop at all: it
    /// describes a different program, or a shorter one.
    NoRecord {
        /// The program loop asked about.
        loop_: u32,
    },
    /// The record describes a different NUMBER of authored steps than
    /// the loop's program has, so no step index means the same thing on
    /// both sides and a step-addressed answer would be an answer about
    /// somebody else's program. The profile side refuses a record of
    /// the wrong shape the same way (`StructureRefusal::shape`).
    RecordShape {
        /// The program loop asked about.
        loop_: u32,
        /// How many steps the loop's program authors.
        authored: usize,
        /// How many the record describes.
        recorded: usize,
    },
    /// The naming anchor carries no entry for this program loop, so
    /// nothing says which refs its walls were named with.
    NoAnchor {
        /// The program loop asked about.
        loop_: u32,
    },
    /// The record credits a segment to a radius argument the step it
    /// names does not hold — a `CarrierRadius` on a step whose spec
    /// carries a centre, say, or a step past the end of the program.
    /// Like [`StepSegmentsError::RecordShape`], it says the record and
    /// the program are not about the same thing; unlike it, the
    /// cardinalities agree and the disagreement is one emission's.
    RadiusNotAnArgument {
        /// The step the emission names.
        step: u32,
        /// The argument role it claims that step's radius drew with.
        arg: StepArg,
    },
    /// The record credits a radius argument with a segment the loop
    /// does not have. Its span twin is
    /// [`StepSegmentsError::SpanOffTheLoop`]; they are two arms
    /// because an emission names ONE segment and a span a RANGE, and a
    /// reader chasing either wants the number the record actually
    /// carried rather than a range synthesised around it.
    EmissionOffTheLoop {
        /// The step the emission names.
        step: u32,
        /// The argument role it credits.
        arg: StepArg,
        /// The segment it claims that argument drew.
        segment: usize,
        /// How many segments the loop has.
        segments: usize,
    },
    /// The loop is a CARRIER form — one step, one radius, the whole
    /// boundary — and the record handed in carries per-radius
    /// emissions. A carrier form emits none: `circle` and
    /// `circle_split` mint their structure directly and their radius
    /// is a per-LOOP fact. So the record is a chain's, and like
    /// [`StepSegmentsError::RecordShape`] it says the record and the
    /// program are not about the same thing.
    CarrierRecordsEmissions {
        /// The program loop asked about.
        loop_: u32,
        /// How many emissions the record carries for it.
        emissions: usize,
    },
    /// The step's recorded span reaches past the end of the loop —
    /// the replay record and the canonical record disagree about how
    /// long the chain is.
    SpanOffTheLoop {
        /// The step asked about.
        step: u32,
        /// One past the last segment the span claims.
        end: usize,
        /// How many segments the loop has.
        segments: usize,
    },
}

impl core::fmt::Display for StepSegmentsError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NoSuchLoop { loops } => {
                write!(f, "the program has {loops} loops")
            }
            Self::NoSuchStep { steps } => {
                write!(f, "the loop's program has {steps} steps")
            }
            Self::NoRecord { loop_ } => {
                write!(f, "the structure record does not describe loop {loop_}")
            }
            Self::RecordShape {
                loop_,
                authored,
                recorded,
            } => write!(
                f,
                concat!(
                    "loop {} authors {} steps and its structure record ",
                    "describes {}, so the two are not about the same program"
                ),
                loop_, authored, recorded
            ),
            Self::NoAnchor { loop_ } => write!(
                f,
                concat!(
                    "the naming anchor does not describe loop {}, so nothing ",
                    "says which refs its entities were named with"
                ),
                loop_
            ),
            // `StepArg::label()` already says the word "radius" where
            // the role has one ("radius", "carrier radius", "arrival
            // carrier radius"), so the sentence names the argument and
            // does not say it twice.
            Self::RadiusNotAnArgument { step, arg } => write!(
                f,
                concat!(
                    "the record says step {}'s {} drew a segment, and that step ",
                    "holds no such argument"
                ),
                step,
                arg.label()
            ),
            Self::EmissionOffTheLoop {
                step,
                arg,
                segment,
                segments,
            } => write!(
                f,
                concat!(
                    "the record says step {}'s {} drew segment {} on a loop with ",
                    "{} of them"
                ),
                step,
                arg.label(),
                segment,
                segments
            ),
            Self::CarrierRecordsEmissions { loop_, emissions } => write!(
                f,
                concat!(
                    "loop {} is a carrier form, whose one radius is the whole ",
                    "boundary's and draws no segment of its own, and its record ",
                    "carries {} radius emissions"
                ),
                loop_, emissions
            ),
            Self::SpanOffTheLoop {
                step,
                end,
                segments,
            } => write!(
                f,
                "step {step} claims segments up to {end} on a loop with {segments} of them"
            ),
        }
    }
}

impl core::error::Error for StepSegmentsError {}

// ------------------------------------------------------------------
// Slot access
// ------------------------------------------------------------------

/// **The one home for narrowing a program address from a `usize`** —
/// a step's index in a recording, the index of the loop it sits in, or
/// a segment it produced — to the `u32` [`crate::SlotId::Profile`],
/// [`ProgramRefusal`] and [`crate::eval::CanonicalSegment`] carry it as.
///
/// D2 addendum row 4. Every caller of this holds the collection the
/// index came from, so an index past `u32` would be 2^32 elements in
/// memory at once: a typed refusal here would guard a state the
/// machine excludes, unlike [`RecordedProgramError::SubdivisionCount`],
/// whose count is one number a caller writes and whose refusal is
/// therefore real. Stated once, here, so no site has to restate it.
///
/// # Panics
///
/// Never, for the reason above; the `unreachable!` is the fail-loud
/// spelling of "the machine got there anyway".
pub(crate) fn program_index(i: usize) -> u32 {
    let Ok(narrowed) = u32::try_from(i) else {
        unreachable!("a collection of {i} elements does not fit in memory")
    };
    narrowed
}

/// The argument roles a target contributes ([] for `Start`).
///
/// Exhaustive on the target vocabulary rather than a test for one
/// form: a target form that carries expressions and enumerates no role
/// is an expression no slot addresses, which the bijection census sees
/// only where the corpus reaches it.
fn target_slots(t: &ProgramTarget, out: &mut Vec<StepArg>) {
    match t {
        ProgramTarget::Point(_) => out.extend(target_roles(false)),
        ProgramTarget::Start | ProgramTarget::StartArriving => {}
    }
}

/// A point target's two coordinate roles: the `Target*` pair, or the
/// `Target2*` twins a fused step's second spec carries. The one
/// assignment the enumeration ([`target_slots`], [`target2_slots`]),
/// the addressing ([`target_coord`], under both accessor macros) and
/// the resolution ([`res_target`]) all read, so a refusal reports at
/// the slot the census enumerates and that slot addresses the
/// coordinate that refused.
fn target_roles(second: bool) -> [StepArg; 2] {
    if second {
        [StepArg::Target2X, StepArg::Target2Y]
    } else {
        [StepArg::TargetX, StepArg::TargetY]
    }
}

/// Which coordinate of a point target `arg` addresses, when it is one
/// of [`target_roles`]' pair for `second`.
fn target_coord(arg: StepArg, second: bool) -> Option<usize> {
    target_roles(second).iter().position(|role| *role == arg)
}

/// The argument roles of one arc spec; `second` selects the arrival
/// (spec₂) role twins.
///
/// EVERY role has a twin, including the three whose modes are not
/// arrival modes (§2c: `family::ArrivalSpec` is implemented for
/// `Radius`, `Via` and `Center` alone, so no recording surface can put
/// a `Bulge`, `Sweep` or `ArcLen` in second position). Enumeration is
/// total over the data type, and a hand-built step may carry one:
/// without its own twin such a spec's argument would share the
/// incoming spec's role, which addresses the incoming argument twice
/// and the arrival's not at all.
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
        (S::Bulge { target, .. }, true) => {
            target2_slots(target, out);
            out.push(A::Bulge2);
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
        (S::Sweep { .. }, true) => out.extend([A::CarrierRadius2, A::SweepVal2]),
        (S::ArcLen { .. }, false) => out.extend([A::CarrierRadius, A::ArcLenVal]),
        (S::ArcLen { .. }, true) => out.extend([A::CarrierRadius2, A::ArcLenVal2]),
    }
}

/// The spec₂ twin of [`target_slots`], exhaustive for the same reason.
fn target2_slots(t: &ProgramTarget, out: &mut Vec<StepArg>) {
    match t {
        ProgramTarget::Point(_) => out.extend(target_roles(true)),
        ProgramTarget::Start | ProgramTarget::StartArriving => {}
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

/// **The document-layer argument role a profile-side radius role
/// names.**
///
/// `profile` records WHICH of a step's radius arguments drew an arc in
/// its own vocabulary — it has no name for a document slot — and this
/// is the one place the two are paired. The pairing mirrors
/// [`spec_slots`]'s: the incoming spec's radius is the step's
/// `CarrierRadius`, the arrival spec's twin is `CarrierRadius2`, and a
/// fillet's own is `Radius`.
fn radius_arg_of(role: profile::RadiusRole) -> StepArg {
    match role {
        profile::RadiusRole::Fillet => StepArg::Radius,
        profile::RadiusRole::Carrier => StepArg::CarrierRadius,
        profile::RadiusRole::Carrier2 => StepArg::CarrierRadius2,
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
            (S::Bulge { b, .. }, A::Bulge, false)
            | (S::Bulge { b, .. }, A::Bulge2, true) => Some(b),
            (S::Sweep { angle, .. }, A::SweepVal, false)
            | (S::Sweep { angle, .. }, A::SweepVal2, true) => Some(angle),
            (S::ArcLen { len, .. }, A::ArcLenVal, false)
            | (S::ArcLen { len, .. }, A::ArcLenVal2, true) => Some(len),
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
                S::Bulge { target: ProgramTarget::Point(p), .. }
                | S::Via { target: ProgramTarget::Point(p), .. }
                | S::Center { target: ProgramTarget::Point(p), .. },
                a,
                second,
            ) => match target_coord(a, second) {
                Some(k) => Some($($ref_kw)* p[k]),
                None => None,
            },
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
            (P::Angle(e), A::AngleVal) => Some(e),
            (P::Toward { dx, .. }, A::DirX) => Some(dx),
            (P::Toward { dy, .. }, A::DirY) => Some(dy),
            (P::Turn(e), A::TurnVal) => Some(e),
            (P::Line(e), A::Length) => Some(e),
            (
                P::LineTo(ProgramTarget::Point(p))
                | P::ContinueTo(ProgramTarget::Point(p))
                | P::TangentArcTo(ProgramTarget::Point(p)),
                a,
            ) => match target_coord(a, false) {
                Some(k) => Some($($ref_kw)* p[k]),
                None => None,
            },
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
    /// **How many authored steps this loop has** — the length of the
    /// step axis of `SlotId::Profile { loop_, step, .. }`.
    ///
    /// A carrier form authors ONE step, numbered 0: that is the step
    /// the slot vocabulary addresses its centre and radius at, and the
    /// step the replay's record describes.
    #[must_use]
    pub fn authored_steps(&self) -> usize {
        match self {
            LoopProgram::Chain(steps) => steps.len(),
            LoopProgram::Circle { .. } | LoopProgram::CircleSplit { .. } => 1,
        }
    }

    /// **The one radius every edge of this loop is drawn at**, where
    /// the loop is a CARRIER form and has one.
    ///
    /// The carrier forms — `circle(centre, r)` and
    /// `circle_split(centre, r, n, phase)` — are the loops whose whole
    /// boundary is a single arc carrier: every segment they replay to
    /// is an arc of that one radius, whatever the subdivision. So the
    /// answer is per LOOP and needs no per-segment address, and a
    /// consumer that has a canonical loop index has everything it
    /// needs.
    ///
    /// A CHAIN loop answers `None`, and that is a scope statement, not
    /// an omission: a chain's arc steps carry their own radii, each
    /// addressing its own segments, so there is no per-loop answer to
    /// give. [`LoopProgram::step_radii`] is the per-STEP question and
    /// [`ProfileProgram::segment_radii`] the per-EDGE one; both answer
    /// for a carrier form too, so a caller that wants one rule for
    /// both shapes asks them rather than this.
    ///
    /// The address is the loop's `Radius` slot
    /// (`SlotId::Profile { loop_, step: 0, arg: StepArg::Radius }`);
    /// this hands back the expression that slot holds, which is what a
    /// lowering needs to lower.
    #[must_use]
    pub fn carrier_radius(&self) -> Option<&Expr> {
        match self {
            LoopProgram::Circle { radius, .. } | LoopProgram::CircleSplit { radius, .. } => {
                Some(radius)
            }
            LoopProgram::Chain(_) => None,
        }
    }

    /// **Every radius expression this loop authors**, in program-step
    /// order and, within a step, in its own slot-enumeration order —
    /// the per-STEP question [`LoopProgram::carrier_radius`] is the
    /// per-LOOP one.
    ///
    /// A CARRIER form answers its one radius at step 0, the step that
    /// replays to the whole loop, so one entry here means one radius
    /// on every edge. A CHAIN answers one entry per radius-bearing
    /// ARGUMENT ([`StepArg::is_radius`]) — an `arc_to` in a
    /// radius-carrying mode, a `fillet`, and each of a fused step's
    /// two or three; a straight step holds none and answers nothing.
    /// A step appears as many times as it holds radii, because the
    /// question is which spellings this program authors and a fused
    /// step authors several.
    ///
    /// **This is the PROGRAM-side question and nothing else asks it.**
    /// It reads no record, so it cannot say which segment a radius
    /// draws — that is [`ProfileProgram::segment_radii`], which reads
    /// the replay's emission record and answers only where that record
    /// says an arc was drawn. Its answer is a SUBSET of this one: a
    /// radius that drew a segment is a radius the program authors, and
    /// enumerating every argument here is what makes that true by
    /// construction rather than by a rule the two doors share. The
    /// direction of the inclusion is the invariant the memo's
    /// stale-token guard rests on: the content key feeds this answer,
    /// the attach stamps that one, and a spelling that reaches a wall
    /// has therefore always reached the key.
    #[must_use]
    pub fn step_radii(&self) -> Vec<(u32, &Expr)> {
        match self {
            LoopProgram::Chain(steps) => {
                let mut out = Vec::new();
                for (i, step) in steps.iter().enumerate() {
                    let mut args = Vec::new();
                    step_slots(step, &mut args);
                    for arg in args.into_iter().filter(|a| a.is_radius()) {
                        if let Some(expr) = step_expr(step, arg) {
                            out.push((program_index(i), expr));
                        }
                    }
                }
                out
            }
            LoopProgram::Circle { radius, .. } | LoopProgram::CircleSplit { radius, .. } => {
                vec![(0, radius)]
            }
        }
    }

    /// This loop's argument roles per step, deterministic order — every
    /// address this program holds an expression at, and nothing else.
    ///
    /// The enumerator the slot walk already ran on, made public so a
    /// caller asking "which arguments does this program have?" — a
    /// notation being checked, a suite covering every role — asks the
    /// program rather than re-deriving the answer from the verb table.
    #[must_use]
    pub fn step_args(&self) -> Vec<(u32, StepArg)> {
        let mut out = Vec::new();
        match self {
            LoopProgram::Chain(steps) => {
                for (i, step) in steps.iter().enumerate() {
                    let mut args = Vec::new();
                    step_slots(step, &mut args);
                    out.extend(args.into_iter().map(|a| (program_index(i), a)));
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

/// Resolves a target's expressions, addressing its coordinates at
/// [`target_roles`]' pair for `second` — the `Target2*` twins on a
/// fused step's second spec. Every other role this module resolves is
/// still spelled three times: in the resolvers ([`res_step`],
/// [`res_spec`]), in the enumeration ([`spec_slots`], [`step_slots`])
/// and in the accessor macros behind [`step_expr`].
/// `every_enumerated_slot_is_where_its_refusal_reports`
/// (`tests/switch_program_vocabulary.rs`) is what holds them together.
///
/// This is the target vocabulary's ONE construct hop: every target a
/// document program carries — a straight leg's, a continuation's, a
/// tangent arc's, and the endpoint inside every endpoint-bearing arc
/// mode — resolves here, so the form set is matched in exactly one
/// place below the document type's own declaration. The direction the
/// compiler cannot check is the one this function runs in: it MATCHES
/// [`ProgramTarget`] and CONSTRUCTS a [`profile::Target`], so a form
/// the kernel vocabulary gains is invisible here. The census keyed on
/// `profile::TargetKind::ALL`
/// (`tests/switch_program_vocabulary.rs`) is what sees it, and it
/// checks the other half of the same arm too: that each form resolves
/// to ITS OWN form rather than being laundered into a neighbour's.
fn res_target<T: Decide>(
    t: &ProgramTarget,
    env: &ParamEnv<T>,
    loop_: u32,
    step: u32,
    second: bool,
) -> Result<profile::Target<T>, (SlotId, EvalError)> {
    let [ax, ay] = target_roles(second);
    Ok(match t {
        ProgramTarget::Start => profile::Target::Start,
        ProgramTarget::StartArriving => profile::Target::StartArriving,
        ProgramTarget::Point(p) => profile::Target::Point(Point2::new(
            res(&p[0], env, loop_, step, ax)?,
            res(&p[1], env, loop_, step, ay)?,
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
        ProgramStep::LineTo(t) => Step::LineTo(res_target(t, env, loop_, i, false)?),
        ProgramStep::ContinueTo(t) => Step::ContinueTo(res_target(t, env, loop_, i, false)?),
        ProgramStep::ArcTo(spec) => Step::ArcTo(res_spec(spec, env, loop_, i, false)?),
        ProgramStep::TangentArcTo(t) => Step::TangentArcTo(res_target(t, env, loop_, i, false)?),
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
    let tgt = |t: &ProgramTarget| res_target(t, env, loop_, i, second);
    Ok(match spec {
        ProgramArcData::Radius { r, side } => profile::ArcData::Radius {
            r: res(r, env, loop_, i, pick(A::CarrierRadius, A::CarrierRadius2))?,
            side: *side,
        },
        ProgramArcData::Bulge { target, b } => profile::ArcData::Bulge {
            target: tgt(target)?,
            b: res(b, env, loop_, i, pick(A::Bulge, A::Bulge2))?,
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
            angle: res(angle, env, loop_, i, pick(A::SweepVal, A::SweepVal2))?,
        },
        ProgramArcData::ArcLen { r, side, len } => profile::ArcData::ArcLen {
            r: res(r, env, loop_, i, pick(A::CarrierRadius, A::CarrierRadius2))?,
            side: *side,
            len: res(len, env, loop_, i, pick(A::ArcLenVal, A::ArcLenVal2))?,
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
                .map(|(i, s)| res_step(s, env, loop_, program_index(i)))
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

/// **Resolves a list of loop programs**, the evaluation pipeline's
/// first stage (then `profile::replay` per loop, then embed +
/// validate).
///
/// Free of [`ProfileProgram`] on purpose: resolution reads the LOOPS
/// and nothing else, and a caller that has loops in hand and no
/// document — a form previewing what it is about to author, a lattice
/// question about which verbs a chain admits — should not have to
/// invent a plane node to ask. Before the plane became a reference
/// those callers built a throwaway program around a world-XY constant;
/// that shortcut would now be a fabricated node id in a value nobody
/// commits.
///
/// # Errors
///
/// The failing slot plus the evaluator's refusal, unaltered.
pub fn resolve_loops<T: Decide>(
    loops: &[LoopProgram],
    env: &ParamEnv<T>,
) -> Result<Vec<Vec<Step<T>>>, (SlotId, EvalError)> {
    loops
        .iter()
        .enumerate()
        .map(|(li, lp)| lp.resolve(env, program_index(li)))
        .collect()
}

impl ProfileProgram {
    /// Whether any expression of this program reads the document
    /// parameter `name` — the question a C6/D9-pinned consumer of the
    /// program (a loft's or a sweep's section) asks before a seed on
    /// that parameter is silently embedded as a constant.
    pub fn references(&self, name: &ParamName) -> bool {
        ProfilePayload::references(self, name)
    }

    /// Resolves every loop at f64 — [`resolve_loops`] over this
    /// program's own loops.
    ///
    /// # Errors
    ///
    /// The failing slot plus the evaluator's refusal, unaltered.
    pub fn resolve<T: Decide>(
        &self,
        env: &ParamEnv<T>,
    ) -> Result<Vec<Vec<Step<T>>>, (SlotId, EvalError)> {
        resolve_loops(&self.loops, env)
    }

    /// **Which profile edges one authored step became** (DM8).
    ///
    /// The map from a document slot's `(loop_, step)` — the coordinates
    /// of `SlotId::Profile` — to the CANONICAL segments those segments
    /// are: the numbering the emitters, the loft's correspondence and
    /// the viewer's per-segment marks iterate (V3). It READS two
    /// records the evaluation already produced and never re-derives
    /// them from the geometry — a second derivation can disagree with
    /// the one the geometry came from, which is the defect this door
    /// exists to not be. The span GIVES the answer, in the program's
    /// own step order, and the profile's naming anchor carries it into
    /// canonical loop and canonical segment counted from the loop's
    /// authored start. The anchor is CHECKED against
    /// canonicalization's own record of the same permutation before it
    /// is read.
    ///
    /// A canonical segment is a position, not a name: what a published
    /// name spells is the piece's minted step and role
    /// ([`crate::ProfileEdgeRef`]), which the naming anchor pairs with
    /// each canonical segment when the table is published.
    ///
    /// # What it reads
    ///
    /// 1. **The replay's per-step segment span** — the answer
    ///    (`profile::ReplayStructure::steps`): which segments of the
    ///    PROGRAM-ORDER chain this step emitted. A step is not one
    ///    segment — an entry verb emits none, a fillet arrival emits
    ///    its straight leg and its arc, and a carrier form's single
    ///    step emits the whole loop, which is how `circle` and
    ///    `circle_split` answer here with no arm of their own.
    /// 2. **The naming anchor** (`eval::anchor`, `LoopAnchor`): which
    ///    canonical loop this program loop is, and whether its
    ///    traversal was reversed. It carries a program segment to the
    ///    canonical segment it became — the identity for a loop
    ///    authored in its canonical sense, `s ↦ n − 1 − s` for one
    ///    authored against it, the canonical start being the authored
    ///    one.
    /// 3. **The permutation canonicalization applied** — the check
    ///    (`profile::LoopCanonical`'s `reversed`): the reversal that
    ///    turns program vertex `i` of `n` into canonical vertex
    ///    `(n − i) mod n`, the only permutation it applies.
    ///
    /// # Why the answer is in CANONICAL indices
    ///
    /// Every verb that consumes a profile iterates its canonical
    /// positions, and the canonical form keeps the author's start and
    /// hole order. A loft's sections are the case this settles: the
    /// skin pairs canonical segment `k` of every section into one wall,
    /// and this door answers any section through that section's own
    /// anchor.
    ///
    /// The anchor is one record and `(3)` is another, so the two are
    /// checked against each other rather than assumed to agree: the
    /// permutation is derived from `(3)`, the decision canonicalization
    /// recorded, and the anchor independently, by bit-matching the
    /// canonical loop against the replayed one.
    ///
    /// # Why a disagreement ASSERTS rather than refusing typed
    ///
    /// One evaluation produces both records, so two derivations of one
    /// permutation that disagree are the evaluation contradicting
    /// itself — a kernel bug, and kernel bugs panic (DM8). The one
    /// cost is a caller that hands this door a `structure` and a
    /// `naming` from two DIFFERENT evaluations: that mispairing panics
    /// where it could have refused. It is a caller bug, no façade
    /// caller can make it today, and it disappears by construction
    /// once the structure record rides on the evaluation's own value
    /// rather than arriving as a second argument
    /// (`work/wire/section-of-re-derives-the-whole-f64-precompute-the-profile-node-already-made.md`).
    ///
    /// Not persisted, and not a cache: it is rebuilt from the records
    /// beside the geometry they describe.
    ///
    /// # Errors
    ///
    /// [`StepSegmentsError`] — a loop or step this program does not
    /// have, or a record that does not describe it. It refuses rather
    /// than guessing at either.
    ///
    /// # Panics
    ///
    /// If the two records describe different permutations of the loop
    /// (the section above).
    pub fn profile_edges_of(
        &self,
        structure: &profile::ProfileStructure,
        naming: &ProfileNaming,
        loop_: u32,
        step: u32,
    ) -> Result<Vec<CanonicalSegment>, StepSegmentsError> {
        self.checked_records(structure, naming, loop_)?
            .edges_of_step(step)
    }

    /// **The records one loop's answers are read from, checked against
    /// each other once.**
    ///
    /// Both doors below answer about segments of one loop through one
    /// permutation, and factoring the check out is what makes that a
    /// property of the code rather than of two copies staying in step:
    /// a per-edge answer and a per-step answer that disagreed about
    /// which permutation the evaluation applied would name two
    /// different walls for one segment.
    ///
    /// # Errors
    ///
    /// [`StepSegmentsError`] — a loop this program does not have, or a
    /// record that does not describe it.
    ///
    /// # Panics
    ///
    /// If the two records describe different permutations of the loop
    /// (see [`ProfileProgram::profile_edges_of`]).
    fn checked_records<'p, 'r>(
        &'p self,
        structure: &'r profile::ProfileStructure,
        naming: &ProfileNaming,
        loop_: u32,
    ) -> Result<AnchoredRecords<'p, 'r>, StepSegmentsError> {
        let li = loop_ as usize;
        let program = self.loops.get(li).ok_or(StepSegmentsError::NoSuchLoop {
            loops: self.loops.len(),
        })?;
        let replay = structure
            .replay
            .get(li)
            .ok_or(StepSegmentsError::NoRecord { loop_ })?;
        let canonical = structure
            .canonical
            .loops
            .get(li)
            .ok_or(StepSegmentsError::NoRecord { loop_ })?;
        // The shape check is `CheckedRecords::new`'s.
        let n = canonical.segments.len();
        let checked = CheckedRecords::new(loop_, program, replay, n)?;
        let (canonical_loop, anchor) = naming
            .loops
            .iter()
            .enumerate()
            .find(|(_, a)| a.program_loop == loop_)
            .ok_or(StepSegmentsError::NoAnchor { loop_ })?;

        // The two records must be ONE permutation: the same length and
        // the same reversal (the canonical start being the authored
        // vertex 0 on both, there is nothing else for them to say).
        let same = anchor.len as usize == n && n != 0 && anchor.reversed == canonical.reversed;
        assert!(
            same,
            concat!(
                "the evaluation's two records of loop {}'s permutation ",
                "disagree: canonicalization recorded reversed={} ",
                "over {} segments, the naming anchor recorded reversed={} ",
                "over {} vertices. One evaluation produces both, ",
                "so they describe one permutation or the kernel has ",
                "contradicted itself"
            ),
            loop_, canonical.reversed, n, anchor.reversed, anchor.len
        );
        Ok(AnchoredRecords {
            records: checked,
            canonical_loop: program_index(canonical_loop),
            anchor: *anchor,
        })
    }

    /// **Which radius each of a loop's profile edges is drawn at.**
    ///
    /// The per-EDGE door, read off the replay's own record of which
    /// segment each authored radius drew
    /// (`profile::ReplayStructure::radii`) and addressed through the
    /// same permutation [`ProfileProgram::profile_edges_of`] answers
    /// in — one `checked_records` for both, so the
    /// two cannot disagree about which permutation the evaluation
    /// applied. One door for both loop shapes, which is what lets a
    /// consumer attach a per-edge scalar without asking first which
    /// shape it is holding.
    ///
    /// **Why the record and not the span.** The step a radius is
    /// AUTHORED on is not the step its arc is credited to: a
    /// `fillet(r)` binds a radius and emits nothing, and the arc it
    /// opens is emitted by the arrival step, which holds no radius of
    /// its own. A fused step holds two or three radii and draws two or
    /// three segments with them. So a span cannot say which radius
    /// drew which segment, and the evaluation records the answer as it
    /// emits instead — DM8's rule that this map reads the records the
    /// evaluation produced and never re-derives them.
    ///
    /// **The two shapes.** A carrier form's one step replays to the
    /// whole loop and every segment of it is an arc of that one radius
    /// ([`LoopProgram::carrier_radius`] says why), so the answer is
    /// the step's radius on every edge and the record carries no
    /// emission for it — a per-segment address for a fact that is per
    /// loop. A CHAIN answers one pair per recorded emission.
    ///
    /// An arc no radius argument drew — a bulge, a through-point, a
    /// centre — is absent from the result, which is an answer and not
    /// a gap: nothing here claims an edge is drawn at a radius, so
    /// nothing downstream attaches a spelling to an edge that is not.
    ///
    /// The answer is in the numbering the published names carry, in
    /// emission order, and is a SUBSET of
    /// [`LoopProgram::step_radii`]'s — the inclusion the content key's
    /// stale-token guard rests on, stated at that feed too.
    ///
    /// # Errors
    ///
    /// [`StepSegmentsError`]: the map's own refusals unaltered — a
    /// loop this program does not have, or a record that does not
    /// describe it — plus the two an emission can be wrong in.
    /// [`StepSegmentsError::SpanOffTheLoop`] names an emission
    /// crediting a segment the loop does not have, and
    /// [`StepSegmentsError::RadiusNotAnArgument`] one crediting a
    /// radius argument the step it names does not hold (a step past
    /// the end of the program included). Both say the record and the
    /// program are not about each other; neither guesses.
    ///
    /// # Panics
    ///
    /// Where [`ProfileProgram::profile_edges_of`] does — the two
    /// records describing different permutations of the loop.
    pub fn segment_radii(
        &self,
        structure: &profile::ProfileStructure,
        naming: &ProfileNaming,
        loop_: u32,
    ) -> Result<Vec<(CanonicalSegment, &Expr)>, StepSegmentsError> {
        let checked = self.checked_records(structure, naming, loop_)?;
        // The carrier forms answer per LOOP: one step, one radius,
        // every edge of it an arc of that radius. A per-segment
        // address for a per-loop fact is what this arm exists to not
        // invent — but the EDGES it answers are step 0's recorded
        // span, walked through the same `edges_of_step` the per-step
        // door walks, so a record whose step 0 does not cover the loop
        // is refused here exactly as it is there rather than answered
        // off the canonical segment count.
        if let Some(radius) = checked.records.program.carrier_radius() {
            // And the record must be a carrier's: `circle` and
            // `circle_split` mint their structure directly and emit
            // nothing, so an emission here is a chain's record under a
            // carrier program. Read before the answer, not after the
            // arm has returned.
            if !checked.records.replay.radii.is_empty() {
                return Err(StepSegmentsError::CarrierRecordsEmissions {
                    loop_,
                    emissions: checked.records.replay.radii.len(),
                });
            }
            return Ok(checked
                .edges_of_step(0)?
                .into_iter()
                .map(|e| (e, radius))
                .collect());
        }
        let mut out = Vec::new();
        for emission in &checked.records.replay.radii {
            let step = program_index(emission.step);
            let arg = radius_arg_of(emission.role);
            let expr = checked
                .records
                .program
                .expr(step, arg)
                .ok_or(StepSegmentsError::RadiusNotAnArgument { step, arg })?;
            if emission.segment >= checked.records.segments {
                return Err(StepSegmentsError::EmissionOffTheLoop {
                    step,
                    arg,
                    segment: emission.segment,
                    segments: checked.records.segments,
                });
            }
            out.push((checked.edge_of(emission.segment), expr));
        }
        Ok(out)
    }

    /// The authoring-time check's body (VQ9): resolve under `env`,
    /// replay every loop, validate the assembled profile — all under
    /// the run tolerance (VQ6: the same `Tolerance::get()` evaluation
    /// pins). Used by the edit door; evaluation re-runs the same
    /// ladder per binding with full typed errors.
    ///
    /// # Errors
    ///
    /// [`ProgramRefusal::Resolve`], [`ProgramRefusal::Transition`],
    /// [`ProgramRefusal::Geometry`] or [`ProgramRefusal::Validate`].
    pub fn check(&self, env: &ParamEnv<f64>, tol: Tol) -> Result<(), ProgramRefusal> {
        self.validated(env, tol).map(|_| ())
    }

    /// The check's ladder, keeping what it produced: the validated
    /// profile, and each loop as replayed with its
    /// [`profile::ReplayStructure`], in program order — what
    /// [`ProfileProgram::pieces`] names the canonical positions from.
    /// Recording changes nothing about what is computed
    /// (`profile::replay_recording` is `replay` bit for bit plus the
    /// account), so this and [`ProfileProgram::check`] cannot disagree.
    fn validated(
        &self,
        env: &ParamEnv<f64>,
        tol: Tol,
    ) -> Result<(profile::ValidatedProfile<f64>, Replayed), ProgramRefusal> {
        let (loops, records) = self.replay_records(env, tol)?;
        // **The identity plane, and the check is honest about why.**
        // Validation is 2-D — `profile::validate` says so itself, and
        // the plane rides through it as conventional data — so what
        // this door checks is the LOOPS: closure, orientation, no
        // self-intersection. It could not read the real frame anyway:
        // this runs at the insert door with a payload in hand and no
        // document, and the frame is a node in one. A profile whose
        // frame reference does not denote a frame is refused where
        // every other operand's kind is, at evaluation.
        let validated = profile::Profile::new(profile::SketchPlane::xy(), loops.clone())
            .validate(tol)
            .map_err(ProgramRefusal::Validate)?;
        Ok((validated, (loops, records)))
    }

    /// **The piece every canonical position of this program is**, under
    /// `env`: per canonical loop, the locator each canonical segment and
    /// each canonical vertex is named by (`names/README.md`, "N1, the
    /// profile pieces") — what the sweeps over this profile name each
    /// wall, rim and vertex by, answered without evaluating a document.
    ///
    /// The one door for a caller that spells a name by a position it
    /// can see — "the wall this profile's third segment sweeps" —
    /// before any evaluation exists to select from. The answer spells
    /// the piece, so it stays the name of that piece whatever later
    /// moves the position.
    ///
    /// # Errors
    ///
    /// [`ProfileProgram::check`]'s; [`ProgramRefusal::Unminted`] for a
    /// program no document minted step ids for;
    /// [`ProgramRefusal::StepIds`] for ids not shaped one list per loop
    /// and one id per authored step; and [`ProgramRefusal::Pieces`]
    /// where the replay and the naming anchor disagree.
    pub fn pieces(
        &self,
        env: &ParamEnv<f64>,
        tol: Tol,
    ) -> Result<crate::eval::ProfilePieces, ProgramRefusal> {
        if !self.carries_step_ids() {
            return Err(ProgramRefusal::Unminted);
        }
        self.check_id_shape().map_err(ProgramRefusal::StepIds)?;
        let (validated, (loops, records)) = self.validated(env, tol)?;
        let naming = crate::eval::derive_naming(&validated, &loops).unwrap_or_else(|| {
            unreachable!(
                "validation reindexes its input exactly, so its canonical loops match the \
                 replayed ones they were minted from"
            )
        });
        crate::eval::ProfilePieces::publish(&naming, &records, &self.ids)
            .map_err(ProgramRefusal::Pieces)
    }

    /// **Whether this program carries step ids at all.** A program no
    /// door has minted ids for carries NO lists: the one spelling of
    /// "unminted" is an empty [`ProfileProgram::ids`]. Any list at all
    /// — even an empty one, or a malformed set — is ids someone
    /// supplied, which the insert door refuses
    /// ([`StepIdFault::Preminted`]) and every other reader checks for
    /// shape.
    #[must_use]
    pub fn carries_step_ids(&self) -> bool {
        !self.ids.is_empty()
    }

    /// **The ids are shaped like the program**: one list per loop, and
    /// one id per authored step of that loop.
    ///
    /// # Errors
    ///
    /// [`StepIdFault::LoopCount`] or [`StepIdFault::Shape`].
    pub(crate) fn check_id_shape(&self) -> Result<(), StepIdFault> {
        if self.ids.len() != self.loops.len() {
            return Err(StepIdFault::LoopCount {
                loops: self.loops.len(),
                given: self.ids.len(),
            });
        }
        for (li, (lp, ids)) in self.loops.iter().zip(&self.ids).enumerate() {
            if ids.len() != lp.authored_steps() {
                return Err(StepIdFault::Shape {
                    loop_: program_index(li),
                    authored: lp.authored_steps(),
                    given: ids.len(),
                });
            }
        }
        Ok(())
    }

    /// The check's first two rungs — resolve under `env`, replay every
    /// loop recording — and what they produce: the replayed loops and
    /// each loop's structure record. Validation is the third rung,
    /// [`ProfileProgram::validated`]'s.
    fn replay_records(&self, env: &ParamEnv<f64>, tol: Tol) -> Result<Replayed, ProgramRefusal> {
        let resolved = self
            .resolve(env)
            .map_err(|(slot, source)| ProgramRefusal::Resolve { slot, source })?;
        let mut loops = Vec::with_capacity(resolved.len());
        let mut records = Vec::with_capacity(resolved.len());
        for (li, steps) in resolved.iter().enumerate() {
            let (lp, record) = profile::replay_recording(steps, tol).map_err(|e| match e.kind {
                profile::ReplayErrorKind::Transition { state, verb } => {
                    ProgramRefusal::Transition {
                        loop_: program_index(li),
                        step: program_index(e.step),
                        state,
                        verb,
                    }
                }
                profile::ReplayErrorKind::Path(ref source) => ProgramRefusal::Geometry {
                    loop_: program_index(li),
                    step: program_index(e.step),
                    kind: source.kind(),
                    rendered: source.to_string(),
                },
            })?;
            loops.push(lp);
            records.push(record);
        }
        Ok((loops, records))
    }
}

impl PartialEq for ProfileProgram {
    /// BIT equality (struct docs): the frame by node identity,
    /// expressions by [`Expr::bit_eq`], structure structurally.
    fn eq(&self, other: &Self) -> bool {
        // A field added to `ProfileProgram` is an E0027 at the two
        // patterns below. The vocabulary underneath is held the same
        // way and by the compiler alone: `loop_bit_eq` and the three
        // functions below it match every variant by name and bind
        // every field of each, so a new loop shape is an E0004 and a
        // new field on an existing one an E0027, at each of them.
        let Self { plane, loops, ids } = self;
        let Self {
            plane: other_plane,
            loops: other_loops,
            ids: other_ids,
        } = other;
        plane == other_plane
            && ids == other_ids
            && loops.len() == other_loops.len()
            && loops
                .iter()
                .zip(other_loops)
                .all(|(a, b)| loop_bit_eq(a, b))
    }
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
        (ProgramTarget::StartArriving, ProgramTarget::StartArriving) => true,
        (ProgramTarget::Point(x), ProgramTarget::Point(y)) => pair_bit_eq(x, y),
        (ProgramTarget::Start | ProgramTarget::StartArriving | ProgramTarget::Point(_), _) => false,
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
                    loop_: program_index(li),
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
    fn plane_input(&self) -> Option<crate::RecipeNodeId> {
        Some(self.plane)
    }
    fn loops(&self) -> Option<&[LoopProgram]> {
        Some(&self.loops)
    }
    fn step_ids(&self) -> Option<&[Vec<StepId>]> {
        Some(&self.ids)
    }
    fn with_program(&self, loops: Vec<LoopProgram>, ids: Vec<Vec<StepId>>) -> Option<Self> {
        Some(Self {
            plane: self.plane,
            loops,
            ids,
        })
    }
    fn mint_step_ids(&mut self, counter: &mut u64) -> Result<(), StepIdFault> {
        if self.carries_step_ids() {
            return Err(StepIdFault::Preminted);
        }
        self.ids = self
            .loops
            .iter()
            .map(|lp| {
                (0..lp.authored_steps())
                    .map(|_| {
                        let id = StepId(*counter);
                        *counter += 1;
                        id
                    })
                    .collect()
            })
            .collect();
        Ok(())
    }
}

/// **Why a program's step ids were refused** at an edit door or the
/// load door (`names/README.md`, "N1, the profile pieces").
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepIdFault {
    /// A program entering the document by `InsertNode` already carries
    /// ids: they are minted by the door, never supplied.
    Preminted,
    /// The ids carry a different number of lists than the program has
    /// loops: one list per loop.
    LoopCount {
        /// How many loops the program has.
        loops: usize,
        /// How many lists were given.
        given: usize,
    },
    /// One loop's list is not one id per authored step.
    Shape {
        /// The loop whose list is the wrong length.
        loop_: u32,
        /// How many the program authors there.
        authored: usize,
        /// How many ids were given.
        given: usize,
    },
    /// A kept id `SetProgram` was given is not a step of the program
    /// the node holds.
    NotThisProfiles {
        /// The id.
        step: StepId,
    },
    /// One id stands for two steps.
    Repeated {
        /// The id.
        step: StepId,
    },
    /// An id at or beyond the document's step counter — one the
    /// document never minted.
    BeyondCounter {
        /// The id.
        step: StepId,
        /// The counter.
        next_step: u64,
    },
}

impl core::fmt::Display for StepIdFault {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Preminted => f.write_str(
                "the program already carries step ids; a program entering the document carries \
                 none, and the insert mints them",
            ),
            Self::LoopCount { loops, given } => write!(
                f,
                "the program has {loops} loops but {given} lists of step ids were given; the ids \
                 carry one list per loop"
            ),
            Self::Shape {
                loop_,
                authored,
                given,
            } => write!(
                f,
                "loop {loop_} authors {authored} steps but {given} step ids were given; the ids \
                 carry one list per loop and one id per authored step"
            ),
            Self::NotThisProfiles { step } => write!(
                f,
                "step id {} is not a step of the program this node holds, so there is nothing \
                 for it to keep; a new step carries no id",
                step.0
            ),
            Self::Repeated { step } => write!(f, "step id {} stands for two steps", step.0),
            Self::BeyondCounter { step, next_step } => write!(
                f,
                "step id {} is at or beyond the document's step counter {next_step}, so the \
                 document never minted it",
                step.0
            ),
        }
    }
}

impl core::error::Error for StepIdFault {}

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
        Target::StartArriving => ProgramTarget::StartArriving,
    })
}

/// Why a recorded PATHS program could not be lifted
/// ([`LoopProgram::from_recorded`]) — and, in one arm, why a notation
/// could not be WRITTEN against the recording it describes
/// ([`RecordedNotation::set_after`], which refuses before any lift so
/// that the notation door and the lift speak one vocabulary rather
/// than two).
///
/// Every verb the transition table declares now has a document
/// spelling, so there is no vocabulary arm: `from_recorded` is
/// exhaustive on [`profile::Step`], and a verb the table gains breaks
/// this file at compile rather than reaching a typed refusal.
///
/// Two arms are unreachable through the authoring algebra — they
/// exist because the door takes a `&[Step<f64>]`, which a caller can
/// also hand-build. The two notation arms are reachable from any
/// caller, because a notation is written against a recording the door
/// does not make the caller hand over at the same time.
///
/// The arm the opening sentence names is
/// [`RecordedProgramError::NotationBeforeAnyStep`]: a recording with
/// no last step, refused where the notation is written.
///
/// **A variant added here breaks `crates/pncad-py/src/tags.rs`**, whose
/// tag map is an exhaustive match over this enum, and the tag inventory
/// beside it: the binding names every refusal a caller can catch, so a
/// new arm is a compile break there and a new word there.
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
    /// A [`RecordedNotation`] entry addresses an argument this
    /// recording has no expression at — a step past the program's end,
    /// or a role this verb does not carry.
    ///
    /// The notation is a caller's SECOND description of a recording, so
    /// it can disagree with the first. It refuses rather than being
    /// dropped: a unit silently discarded is the very erasure this door
    /// exists to stop, arriving one layer up.
    NotationOffProgram {
        /// The authored step index the entry named.
        step: u32,
        /// The argument role it named.
        arg: StepArg,
    },
    /// [`RecordedNotation::set_after`] was asked to write the notation
    /// of the last recorded step, and nothing has been recorded.
    ///
    /// The derived door names no index — that is the whole point of it
    /// — so this is not
    /// [`RecordedProgramError::NotationOffProgram`] at step 0: the
    /// caller wrote no step number for that arm to report back, and
    /// step 0 of a one-step recording whose verb lacks the role is a
    /// different mistake with a different recourse.
    NotationBeforeAnyStep {
        /// The argument role the entry named.
        arg: StepArg,
    },
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
            Self::NotationOffProgram { step, arg } => write!(
                f,
                "the notation names the {} of step {step}, which this recording has no argument at",
                arg.label()
            ),
            Self::NotationBeforeAnyStep { arg } => write!(
                f,
                "the notation names the {} of the step just recorded, and nothing has been \
                 recorded yet",
                arg.label()
            ),
        }
    }
}

impl core::error::Error for RecordedProgramError {}

/// **The notation a recorded PATHS program was authored in** — one
/// display unit per argument whose author wrote one, travelling beside
/// the recording to the door that lifts it.
///
/// # Why it travels beside the recording and not inside it
///
/// **A recorded value cannot carry its own unit, and the reason is a
/// type bound rather than a layering preference.** [`profile::Step`] is
/// `Step<T: Real>`, and `Real` is an ARITHMETIC bound — `Add + Sub +
/// Mul + Div + Neg`, `sqrt`, `pi` — because the same recording is
/// replayed at interval and derivative scalars, not only at `f64`. A
/// `(f64, UnitSym)` pair does not implement it, so pairing the unit
/// with the number inside a step does not compile; and widening the
/// step's own fields instead would put [`UnitSym`] inside `profile`,
/// which is what D6's first paragraph and G1 layering forbid. Either
/// way the recording holds bare numbers.
///
/// But a value crossing INTO a document carries the unit it was
/// written in, never a bare number (DESIGN.md D6 ¶2). The crossing is
/// [`LoopProgram::from_recorded_with_notation`], and this is what an
/// author hands it there.
///
/// # A whole notation, at the one crossing
///
/// The notation is handed over as a BATCH at the lift rather than
/// written argument by argument onto a lifted program, because an
/// entry can only be checked against the program it describes: a
/// per-argument door would have to be a second public mutation door
/// onto [`LoopProgram`]'s expressions, and would raise
/// [`RecordedProgramError::NotationOffProgram`] after the program a
/// caller already holds is minted rather than instead of minting it.
///
/// # The key is the document's own address
///
/// A unit is filed under (step, [`StepArg`]) — the pair
/// [`crate::SlotId::Profile`] addresses an expression by. So the
/// notation names an argument by its ROLE in the verb's own vocabulary,
/// never by a position in an argument list, and the lift applies it
/// through the same addressing the slot doors read; there is no second
/// table of which argument is which for the two to disagree about. A
/// recorded step keeps its index through the lift (a carrier form
/// authors one step, numbered 0), so the step a recording holds is the
/// step it addresses here.
///
/// **Which half of the address the author has to know.** The ROLE is
/// the verb's own vocabulary and a caller writing `line_to` knows it
/// wrote a target. The INDEX is a position in the recording, and a
/// caller who counts it is describing the recording a second time —
/// [`Self::set_after`] derives it from the recording instead, and
/// [`Self::set`] is for the callers whose index is already derived
/// from the program.
///
/// # A unit measures what its role holds
///
/// **Both doors refuse a unit whose quantity is not the dimension
/// [`StepArg::dimension`] requires**, at the door where the caller
/// writes it, so a lift can never meet a mismatched pairing. It is
/// one predicate asked in one place: [`Self::set`] asks
/// `UnitSym::checked_for` — the same predicate `Expr::literal_with_unit`
/// asks, asked here because a notation is written before any literal
/// exists to refuse it — and [`Self::set_after`] delegates to
/// [`Self::set`] once it has derived the index, so the two doors
/// cannot drift apart on what a role admits. They differ only in
/// their refusal vocabulary: `set` returns the [`DimensionError`]
/// itself and `set_after` the [`RecordedProgramError::Literal`]
/// carrying it, which is the vocabulary the lift speaks. A Scalar role — a
/// bulge, a director component — therefore admits only the
/// dimensionless row `quantity::ONE`, which is the notation every
/// Scalar literal carries already: a ratio names no unit, and this is
/// where that stops being a convention and becomes a refusal.
///
/// # It is not persisted
///
/// Nothing stores a `RecordedNotation`. It is consumed at the lift, and
/// what survives is the display unit on each literal, which round-trips
/// through save and load exactly as every literal's does. Two recordings
/// of one leg written in different units lift to [`Expr::bit_eq`]
/// programs and evaluate to one geometry — the unit is presentation
/// metadata (DESIGN.md D6), outside expression identity.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RecordedNotation {
    /// Ordered because the LIFT'S REFUSAL is order-dependent. The
    /// writes themselves commute — each entry has its own
    /// `(step, StepArg)` address and writes one argument — but a
    /// notation with two entries off the program stops at the first
    /// one, so an unordered map would name a different
    /// [`RecordedProgramError::NotationOffProgram`] run to run. Key
    /// order makes the sentence a caller reads a function of what they
    /// wrote.
    units: std::collections::BTreeMap<(u32, StepArg), UnitSym>,
}

impl RecordedNotation {
    /// No argument was written with a notation — the recording as the
    /// path algebra makes it, and what [`LoopProgram::from_recorded`]
    /// lifts.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Records that the argument at (`step`, `arg`) was written in
    /// `unit`, replacing any notation already there.
    ///
    /// **The ADDRESSED door.** A hand-written `step` is the caller's
    /// SECOND description of the recording — the first being the
    /// recording itself — and the two can disagree. The lift catches a
    /// disagreement it can see
    /// ([`RecordedProgramError::NotationOffProgram`]), but an index
    /// that is off by one and lands on a step carrying the SAME role
    /// puts the unit on a different argument and is accepted.
    /// [`RecordedNotation::set_after`] is the door that cannot
    /// miscount: it derives the index from the recording. Write this
    /// one where the index is DERIVED from the program rather than
    /// counted — the viewer's sketch notation walks
    /// [`LoopProgram::step_args`] and addresses what that walk hands
    /// it.
    ///
    /// # Errors
    ///
    /// [`DimensionError::DisplayUnitMismatch`] when the unit's quantity
    /// is not the dimension the role holds (a `mm` on a bulge).
    pub fn set(
        &mut self,
        step: u32,
        arg: StepArg,
        unit: quantity::UnitDef,
    ) -> Result<(), DimensionError> {
        // The one predicate, asked once for both doors (see the type's
        // "A unit measures what its role holds").
        let sym = UnitSym::checked_for(arg.dimension(), unit)?;
        self.units.insert((step, arg), sym);
        Ok(())
    }

    /// Records that `arg` of the LAST step in `recorded` was written
    /// in `unit`, replacing any notation already there.
    ///
    /// **The derived door.** The index is `recorded.len() - 1` and
    /// never a number the caller wrote, so the leg a notation lands on
    /// is the leg the author had just recorded when they wrote it:
    ///
    /// ```ignore
    /// let path = path.line_to(p1, t)?;
    /// n.set_after(path.recorded(), StepArg::TargetX, MM.def())?;
    /// ```
    ///
    /// Every verb records exactly one step
    /// (`profile::PartialPath::recorded`), binders included, so "the
    /// last recorded step" is the verb just called — after `fillet(r)`
    /// it is that binder, whose radius is
    /// [`StepArg::Radius`]. A recorded step keeps its index through
    /// the lift, so `recorded.len() - 1` IS the `step` half of the
    /// address [`RecordedNotation::set`] takes.
    ///
    /// **Two moments, two spellings of the recording.** MID-CHAIN it
    /// is `recorded()`, which every path state that holds the core
    /// answers — the partial path and each arrival builder a verb
    /// hands back — so the door is reachable wherever an author has
    /// just recorded. AFTER THE CLOSER the chain is a
    /// `profile::ClosedLoop` and the recording is its public
    /// `program` field, which addresses the closing step. The two are
    /// the same slice at the same index; which one a caller writes is
    /// decided by which value they are holding.
    ///
    /// This closes the miscount, not the misnaming: a role the last
    /// step does not carry is still the lift's
    /// [`RecordedProgramError::NotationOffProgram`], now with the
    /// index certainly the author's own leg.
    ///
    /// # Errors
    ///
    /// [`RecordedProgramError::NotationBeforeAnyStep`] when `recorded`
    /// is empty — there is no last step to write against — and
    /// [`RecordedProgramError::Literal`] carrying
    /// [`DimensionError::DisplayUnitMismatch`] when the unit's
    /// quantity is not the dimension the role holds, which is
    /// [`RecordedNotation::set`]'s refusal in the vocabulary the lift
    /// speaks.
    pub fn set_after(
        &mut self,
        recorded: &[Step<f64>],
        arg: StepArg,
        unit: quantity::UnitDef,
    ) -> Result<(), RecordedProgramError> {
        let Some(last) = recorded.len().checked_sub(1) else {
            return Err(RecordedProgramError::NotationBeforeAnyStep { arg });
        };
        self.set(program_index(last), arg, unit)?;
        Ok(())
    }

    /// The notation recorded for one argument, if its author wrote one.
    #[must_use]
    pub fn get(&self, step: u32, arg: StepArg) -> Option<quantity::UnitDef> {
        self.units.get(&(step, arg)).map(|sym| sym.def())
    }

    /// Whether no argument was written with a notation — the state
    /// [`LoopProgram::from_recorded`] lifts under.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.units.is_empty()
    }

    /// How many arguments were written with a notation.
    #[must_use]
    pub fn len(&self) -> usize {
        self.units.len()
    }
}

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
    /// A polygon over EXPRESSION corners: `At(p0)`, `LineTo(p1)`, …,
    /// `LineTo(Start)` — the VQ5 expansion of the polygon builder, at
    /// arbitrary points, so a document whose corners are driven by
    /// document parameters reaches the builder rather than spelling
    /// the expansion out.
    ///
    /// Infallible: every coordinate is already an [`Expr`], so there
    /// is no literal left to refuse.
    ///
    /// This is the ONE expansion. [`LoopProgram::polygon`] is this
    /// door at literal corners, so the two spellings of a polygon
    /// cannot drift apart.
    pub fn polygon_expr(points: impl IntoIterator<Item = [Expr; 2]>) -> Self {
        let mut steps = Vec::new();
        for (i, p) in points.into_iter().enumerate() {
            steps.push(if i == 0 {
                ProgramStep::At(p)
            } else {
                ProgramStep::LineTo(ProgramTarget::Point(p))
            });
        }
        steps.push(ProgramStep::LineTo(ProgramTarget::Start));
        LoopProgram::Chain(steps)
    }

    /// A literal polygon — [`LoopProgram::polygon_expr`] at literal
    /// corners (corpus/fixture authoring).
    ///
    /// # Errors
    ///
    /// A non-finite coordinate (the literal door's refusal).
    pub fn polygon(points: impl IntoIterator<Item = (f64, f64)>) -> Result<Self, DimensionError> {
        let corners = points
            .into_iter()
            .map(|(x, y)| Ok([len_lit(x)?, len_lit(y)?]))
            .collect::<Result<Vec<_>, DimensionError>>()?;
        Ok(Self::polygon_expr(corners))
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
    /// **Every literal here is written in the canonical unit**, because
    /// that is all a recording says: a `Step<f64>` is metres and
    /// radians. Where the author wrote a notation down,
    /// [`LoopProgram::from_recorded_with_notation`] is the door that
    /// carries it across, and this one is that door at no notation.
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

    /// [`LoopProgram::from_recorded`] for a recording whose author
    /// wrote the notation down.
    ///
    /// This is the crossing D6 names: a value entering a document
    /// carries the unit it was written in. The recorded `f64`s are
    /// canonical metres and radians and stay so — the notation is
    /// presentation metadata, so the two programs a caller gets from
    /// `25 mm` and `0.025 m` hold the same bits and differ only in what
    /// they say they were written in.
    ///
    /// Each entry is applied through this type's own `expr_mut`, the
    /// addressing `SlotId::Profile` reads, so an argument whose author
    /// wrote a unit is minted with it and every other argument is the
    /// literal [`LoopProgram::from_recorded`] mints. An EMPTY notation
    /// therefore returns that door's answer unchanged, argument for
    /// argument and bit for bit, which is what lets the two doors be
    /// one door with a default.
    ///
    /// # Errors
    ///
    /// Everything [`LoopProgram::from_recorded`] refuses, plus
    /// [`RecordedProgramError::NotationOffProgram`] for an entry
    /// addressing an argument this recording has none of.
    pub fn from_recorded_with_notation(
        steps: &[Step<f64>],
        notation: &RecordedNotation,
    ) -> Result<Self, RecordedProgramError> {
        let mut program = Self::from_recorded(steps)?;
        for (&(step, arg), sym) in &notation.units {
            let Some(slot) = program.expr_mut(step, arg) else {
                return Err(RecordedProgramError::NotationOffProgram { step, arg });
            };
            // D2 addendum row 4. A recorded program is literal by
            // construction: every argument of the program this line
            // reads was minted by `from_recorded` through
            // `Expr::literal`. So a non-literal here is a kernel bug
            // rather than a caller's input, and a typed refusal would
            // be a guard for a state the construction excludes.
            let Some(value) = slot.literal_value() else {
                unreachable!(
                    "the {} of step {step} is not a literal, yet `from_recorded` minted every \
                     argument of this program through `Expr::literal`",
                    arg.label()
                )
            };
            *slot = Expr::literal_with_unit(value, arg.dimension(), sym.def())?;
        }
        Ok(program)
    }

    /// A literal circle loop.
    ///
    /// # The struct literal is the parametric door
    ///
    /// There is no `circle_expr` twin of
    /// [`LoopProgram::polygon_expr`], and that is the design rather
    /// than an omission. `polygon` EXPANDS — one authoring call
    /// becomes a chain of steps — so the expansion needs exactly one
    /// home, and the literal door reaches it by delegating to the
    /// expression door. `Circle` expands into nothing: it is a struct
    /// variant whose two fields are the whole program, so an author
    /// holding [`Expr`] arguments writes
    /// `LoopProgram::Circle { centre, radius }` (and
    /// `LoopProgram::CircleSplit { .. }`) directly. That literal IS
    /// the parametric door. A constructor over it would be a third
    /// spelling of the variant with nothing behind it to keep in
    /// step.
    ///
    /// # The literal author keeps both of this door's guarantees
    ///
    /// This door is not the check its `Result` makes it look like, so
    /// writing the variant out gives nothing up:
    ///
    /// - FINITENESS belongs to [`Expr::literal`], which is the only
    ///   way to mint a literal expression at all and refuses a
    ///   non-finite value there. That refusal is the sole error this
    ///   constructor can return.
    /// - DIMENSION belongs to the document. This door only PICKS
    ///   `Length` for the centre and radius (and `Angle` for
    ///   [`LoopProgram::circle_split`]'s phase), so the picks agree
    ///   with the roles by construction. An author supplying
    ///   expressions picks instead, and `apply` checks the pick: every
    ///   slot of an entering node is walked, the role's required
    ///   dimension ([`StepArg::dimension`], reached through
    ///   [`SlotId::dimension`]) against the expression's, and a
    ///   disagreement refuses as `EditError::SlotDimensionMismatch`
    ///   before the program joins the document. The same walk runs on
    ///   every slot write and on a parameter redeclaration, so there
    ///   is no later window in which a document's role can hold the
    ///   wrong dimension. It is the DOCUMENT's door, though: a program
    ///   built and replayed without entering one — a viewer preview —
    ///   never reaches it, and such a builder assigns the dimensions
    ///   itself exactly as this constructor does.
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
    /// Parametric authors write the `CircleSplit` variant out; see
    /// [`LoopProgram::circle`] for why there is no expression door
    /// here and where an expression's dimension is checked instead.
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
