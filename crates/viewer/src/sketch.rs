//! **The profile-authoring vocabulary**: what a form hands the
//! session when it asks for a profile node, and what that would draw.
//!
//! # Why this is not in `session`
//!
//! The add-profile door used to offer two template shapes, and their
//! spec was four fields beside the datum spec. The PATHS algebra's
//! whole verb set is not four fields — it is a vocabulary, with a
//! lowering, a replay and a flattener behind it — and `session.rs` is
//! already the crate's accretion case (issue #1386). So the
//! vocabulary lives here, and `session` re-exports the one type its
//! op vocabulary names ([`ProfileShape`]) rather than owning it.
//!
//! # Plain numbers in, `Expr` slots out
//!
//! Everything here is `f64` in canonical units (metres, radians), for
//! the reason every creation spec in this crate is: the SESSION mints
//! the expression slots, so a form hands it numbers and never an
//! `Expr` it would have had to build a second way. A path's steps are
//! the kernel's own [`Step`], lowered by the document layer's own lift
//! ([`loop_program`]), so there is no viewer spelling of the verb
//! vocabulary to fall behind it.
//!
//! # What is judged here, and what is not
//!
//! Only the literals and the program's shape: a non-finite field and a
//! complete-loop verb inside a chain refuse typed
//! ([`RecordedProgramError`], the lift's refusal).
//! Whether the verbs form a legal walk of the lattice, whether the
//! geometry closes, and whether the loops nest are all the profile
//! layer's questions, asked at replay — by the edit door on commit,
//! and by [`preview`] before it, which is the same ladder run for the
//! picture instead of for the verdict.
//!
//! **A fourth question is judged here and belongs to neither list:
//! whether a replayed loop can be DRAWN.** It is not the profile
//! layer's, because a loop can be a perfectly good profile and still
//! be a shape no point of which is a place — an arc whose radius or
//! centre is not a number, from a finite bulge near the bottom of the
//! exponent range or two vertices whose midpoint overflows; a vertex
//! the replay's own arithmetic put past the top of that range; a
//! point along an arc whose frame is finite and whose far side is
//! not. And it is not a literal's, because every literal involved
//! passed the literal door already. It is the flattener's, it is
//! answered by [`PreviewError::Unflattenable`], and it exists because
//! this module is the one place that turns a loop into coordinates.
//!
//! Module kind: **vocabulary** — it names no driver type and no
//! `app`-only crate (`crates/viewer/README.md`, Module boundaries).

use pncad::document::{
    Datum, DatumValue, Dimension, DimensionError, Doc, EvalError, Evaluation, Expr, LoopProgram,
    Node, ParamEnv, ProfileProgram, RecipeNodeId, RecordedNotation, RecordedProgramError, SlotId,
    ValuePayload, resolve_loops, unparse,
};
use pncad::geom_core::{Point2, Tol};
use pncad::profile::{
    ArcData, ArcMode, ArcSide, ArcSweep, Profile, ProfileError, ProfileLoop, ReplayError,
    ReplayErrorKind, SketchPlane, SpecForms, Step, Target, TargetKind, TipState, Verb,
    arc_specs_at, replay,
};
use pncad::quantity::{self, AngleUnit, LengthUnit, WrittenLength};

/// One loop of the add-profile door: a template shape, or a PATH
/// authored verb by verb.
///
/// **The templates are not the vocabulary; they are shortcuts into
/// it.** A circle is the one-step path `circle(centre, r)`, and it
/// lowers as exactly that; a rectangle is four `line_to`s somebody
/// would otherwise type. Everything else a profile can be is
/// [`ProfileShape::Path`], which carries the algebra's whole verb set.
///
/// [`loop_program`] lowers each arm to its [`LoopProgram`] form and
/// refuses typed what does not lower (a non-finite field, a
/// complete-loop verb inside a chain); a degenerate loop (zero radius,
/// zero width) and an ill-typed lattice walk both refuse through the
/// edit door's own authoring-time check, exactly as a hand-written
/// program would.
#[derive(Clone, Debug)]
pub enum ProfileShape {
    /// A circle (`LoopProgram::Circle`).
    Circle {
        /// The centre, in sketch coordinates (metres).
        centre: [f64; 2],
        /// The radius, metres.
        radius: f64,
    },
    /// An axis-aligned rectangle centred on the sketch origin: a
    /// `LoopProgram::Chain` with corners at `(±w/2, ±h/2)`.
    Rectangle {
        /// The width (x extent), metres.
        width: f64,
        /// The height (y extent), metres.
        height: f64,
    },
    /// A recorded PATHS program, verb by verb — the algebra's own
    /// [`Step`] at plain numbers, which is what makes it a form's
    /// currency: the chrome hands the session numbers in canonical
    /// units and the session mints the `Expr` slots, exactly as it
    /// does for every other creation door.
    ///
    /// **The kernel's type, not a mirror of it.** The whole verb set
    /// is here because it is the whole set: a form offering half a
    /// vocabulary is a form whose user has to leave it to say the
    /// other half. Which verbs are well-typed at a given tip is not
    /// this value's business — the lattice decides that at replay, and
    /// an ill-typed walk refuses typed at the edit door naming the
    /// state and the verb (`ProgramRefusal::Transition`). [`preview`],
    /// [`tip_state_at`] and [`admits_at`] are how a form asks that
    /// before committing.
    Path {
        /// The verbs, in authoring order. A chain must END in a
        /// `Start`-targeting verb, or be one complete-loop verb
        /// (`circle`, `circle_split`) alone; both are checked by the
        /// lowering and the replay, not by this representation.
        steps: Vec<Step<f64>>,
    },
}

/// **A step of `verb` with the path form's starting numbers** — what
/// a row becomes when its verb is picked.
///
/// Exhaustive on the kernel's [`Verb`], and that is what holds the
/// form to the algebra: the form offers [`Verb::ALL`], so a verb the
/// transition table gains reaches the menu by itself, and it has no
/// starting step until this match gives it one — a compile error, not
/// a verb that is silently missing.
///
/// **Millimetre-scale, never zero.** A leg of length zero and a
/// fillet of radius zero are both geometry refusals, so a fresh step
/// that carried them would put the form in a refusing state the
/// moment a verb was picked — which reads as the form rejecting the
/// verb rather than waiting for its number.
pub fn fresh_step(verb: Verb) -> Step<f64> {
    let target = fresh_target(TargetKind::Point);
    let arc = fresh_arc(ArcMode::Radius);
    match verb {
        Verb::At => Step::At(Point2::origin()),
        Verb::Angle => Step::Angle(0.0),
        Verb::Toward => Step::Toward { dx: 1.0, dy: 0.0 },
        Verb::Tangent => Step::Tangent,
        Verb::Cusp => Step::Cusp,
        Verb::Turn => Step::Turn(0.0),
        Verb::Line => Step::Line(0.01),
        Verb::LineTo => Step::LineTo(target),
        Verb::ContinueTo => Step::ContinueTo(target),
        Verb::ArcTo => Step::ArcTo(arc),
        Verb::TangentArcTo => Step::TangentArcTo(target),
        Verb::Fillet => Step::Fillet { radius: 0.001 },
        Verb::FilletArc => Step::FilletArc {
            radius: 0.001,
            spec: arc,
        },
        Verb::ArcFillet => Step::ArcFillet {
            spec: arc,
            radius: 0.001,
        },
        Verb::ArcFilletArc => Step::ArcFilletArc {
            spec: arc,
            radius: 0.001,
            spec2: arc,
        },
        Verb::FarEndTo => Step::FarEndTo(Point2::new(0.01, 0.0)),
        Verb::CloseTo => Step::CloseTo,
        Verb::Circle => Step::Circle {
            centre: Point2::origin(),
            radius: 0.01,
        },
        Verb::CircleSplit => Step::CircleSplit {
            centre: Point2::origin(),
            radius: 0.01,
            n: 4,
            phase: 0.0,
        },
    }
}

/// **An arc spec of `mode` with the form's starting numbers** —
/// millimetre-scale and never degenerate, for the reason
/// [`fresh_step`]'s are; exhaustive on the kernel's [`ArcMode`] for
/// the reason that one is on [`Verb`].
pub fn fresh_arc(mode: ArcMode) -> ArcData<f64> {
    let target = fresh_target(TargetKind::Point);
    match mode {
        ArcMode::Radius => ArcData::Radius {
            r: 0.01,
            side: ArcSide::Left,
        },
        ArcMode::Bulge => ArcData::Bulge { target, b: 0.5 },
        ArcMode::Via => ArcData::Via {
            q: Point2::new(0.005, 0.005),
            target,
        },
        ArcMode::Center => ArcData::Center {
            c: Point2::origin(),
            winding: ArcSweep::Ccw,
            target,
        },
        ArcMode::Sweep => ArcData::Sweep {
            r: 0.01,
            side: ArcSide::Left,
            angle: core::f64::consts::FRAC_PI_2,
        },
        ArcMode::ArcLen => ArcData::ArcLen {
            r: 0.01,
            side: ArcSide::Left,
            len: 0.01,
        },
    }
}

/// **A target of form `kind`**, for a target control switching form.
///
/// Every form is offered wherever a target is; inside an arc spec the
/// picker greys the forms that spec's dispatcher refuses at the tip
/// ([`SpecForms`]), and a tip the form cannot read leaves the replay
/// to refuse them typed.
pub fn fresh_target(kind: TargetKind) -> Target<f64> {
    match kind {
        TargetKind::Point => Target::Point(Point2::new(0.01, 0.0)),
        TargetKind::Start => Target::Start,
        TargetKind::StartArriving => Target::StartArriving,
    }
}

/// **The notation a form is authoring in** — one length unit and one
/// angle unit, carried into every literal a lowering mints.
///
/// It exists because the units are a fact about the PERSON at the
/// keyboard rather than about any one field (`app`'s drafts say so):
/// a form writes every literal it mints in one notation, so the
/// notation is one value handed to the lowering rather than a unit per
/// field.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Notation {
    /// Every `Length` literal is written in this.
    pub length: LengthUnit,
    /// Every `Angle` literal is written in this.
    pub angle: AngleUnit,
}

impl Notation {
    /// The canonical spellings — metres and radians, said out loud.
    /// The forms' own default is `app`'s, not this.
    pub const CANONICAL: Self = Self {
        length: quantity::M,
        angle: quantity::RAD,
    };

    /// A `Length` literal from an already-canonical value, remembering
    /// this notation — the form's shape (`WrittenLength::canonical_in`:
    /// a draft field holds metres whatever the picker shows).
    fn length(self, metres: f64) -> Result<Expr, DimensionError> {
        Expr::written_length(WrittenLength::canonical_in(metres, self.length))
    }

    /// A literal point.
    fn point(self, p: [f64; 2]) -> Result<[Expr; 2], DimensionError> {
        Ok([self.length(p[0])?, self.length(p[1])?])
    }

    /// **This notation over every argument `program` holds** — each
    /// `Length` argument written in [`Notation::length`], each `Angle`
    /// one in [`Notation::angle`], and every other argument (a bulge, a
    /// director component) left with the one spelling a dimensionless
    /// number has.
    ///
    /// Asked of the PROGRAM, through its own argument enumeration,
    /// rather than of the steps: which roles a step holds is the
    /// document layer's table, and a second copy of it here is how a
    /// verb's radius would come to be minted without its unit.
    fn over(self, program: &LoopProgram) -> Result<RecordedNotation, DimensionError> {
        let mut notation = RecordedNotation::new();
        for (step, arg) in program.step_args() {
            match arg.dimension() {
                Dimension::Length => notation.set(step, arg, self.length.def())?,
                Dimension::Angle => notation.set(step, arg, self.angle.def())?,
                Dimension::Scalar | Dimension::Count => {}
            }
        }
        Ok(notation)
    }
}

/// Lower one template shape to its loop program, minting every literal
/// in `notation`.
///
/// A path lowers through the document layer's own lift,
/// [`LoopProgram::from_recorded_with_notation`] — the door that takes
/// a PATHS recording to its document form for every other authoring
/// surface — so there is no second verb-by-verb lowering here to fall
/// behind the vocabulary. The circle template IS a path (one `circle`
/// step) and lowers as one.
///
/// The rectangle routes through [`LoopProgram::polygon_expr`] instead,
/// which takes corners that are already `Expr` and mints nothing, so
/// the notation rides through it untouched and the polygon expansion
/// is written once for the workspace.
///
/// # Errors
///
/// A non-finite field (the literal door's refusal), or a path that is
/// not a program's shape — a complete-loop verb (`circle`,
/// `circle_split`) inside a chain. Degeneracy — a zero radius, a zero
/// width — is NOT judged here: the edit door's authoring-time check
/// replays the program and refuses it typed, which is one rule for
/// authored and hand-written programs alike.
pub fn loop_program(
    shape: &ProfileShape,
    notation: Notation,
) -> Result<LoopProgram, RecordedProgramError> {
    match shape {
        ProfileShape::Circle { centre, radius } => loop_program(
            &ProfileShape::Path {
                steps: vec![Step::Circle {
                    centre: Point2::new(centre[0], centre[1]),
                    radius: *radius,
                }],
            },
            notation,
        ),
        ProfileShape::Rectangle { width, height } => {
            let (hw, hh) = (width / 2.0, height / 2.0);
            // Counter-clockwise from the lower-left corner — the same
            // winding every literal outer loop in this workspace uses.
            //
            // The halving is the FORM's arithmetic, in f64, exactly as
            // it always was: a template rectangle is authored by its
            // extents and recorded as its corners. A corner expressed
            // as `width/2` would be a different recipe, and it wants
            // the width to be a named thing first — which is the
            // expression-driven form this op vocabulary now admits but
            // no chrome yet offers.
            let corners = [(-hw, -hh), (hw, -hh), (hw, hh), (-hw, hh)];
            let corners = corners
                .into_iter()
                .map(|(x, y)| notation.point([x, y]))
                .collect::<Result<Vec<_>, DimensionError>>()?;
            Ok(LoopProgram::polygon_expr(corners))
        }
        ProfileShape::Path { steps } => {
            // Lifted once to learn which arguments the program holds,
            // then again with the notation written over them: the lift
            // is the only thing that knows the roles.
            let written = notation.over(&LoopProgram::from_recorded(steps)?)?;
            LoopProgram::from_recorded_with_notation(steps, &written)
        }
    }
}

/// **Whether two shape lists author the same loops** — what a frame
/// asks to learn whether an edit drawn during it moved the preview.
///
/// Asked of the canonical LOWERINGS rather than of the shapes, because
/// the kernel's step types carry no `PartialEq` (comparing points is
/// the predicate layer's job, `geom_core::Point2` says), and the
/// lowering is what [`preview`] draws from anyway: two lists that
/// lower alike preview alike. A list that does not lower compares by
/// its refusal, which is also what the preview shows for it.
pub fn authors_same_loops(a: &[ProfileShape], b: &[ProfileShape]) -> bool {
    loop_programs(a, Notation::CANONICAL) == loop_programs(b, Notation::CANONICAL)
}

/// **A list of shapes lowered loop by loop** — [`loop_program`] over
/// each, in description order, the whole list refusing with the first
/// loop that does. The one lowering every door of the profile editor
/// hands the session, and the one the preview's change test compares.
///
/// # Errors
///
/// [`loop_program`]'s.
pub fn loop_programs(
    shapes: &[ProfileShape],
    notation: Notation,
) -> Result<Vec<LoopProgram>, RecordedProgramError> {
    shapes
        .iter()
        .map(|shape| loop_program(shape, notation))
        .collect()
}

/// **Held loops as the shapes the preview and the lowering take** —
/// one [`ProfileShape::Path`] per loop, in authoring order. The path
/// editor holds the kernel's steps loop by loop; this is the one place
/// that list becomes the form's currency.
pub fn path_shapes(loops: &[Vec<Step<f64>>]) -> Vec<ProfileShape> {
    loops
        .iter()
        .map(|steps| ProfileShape::Path {
            steps: steps.clone(),
        })
        .collect()
}

// ------------------------------------------------------------------
// The edit door: a committed program into the editor, and back
// ------------------------------------------------------------------

/// **The loops of a committed profile as the editor holds them** —
/// the kernel's own [`Step`] at plain numbers, per loop, which is the
/// currency the create form authors in. What makes the two doors one
/// editor is that both hold this and nothing else.
///
/// The inverse of [`loop_program`], read through the document layer's
/// own resolver ([`resolve_loops`]) rather than a second verb-by-verb
/// walk: a resolved literal IS its recorded number, so a program the
/// form authored comes back as the steps it was authored from, bit
/// for bit, and a verb the vocabulary gains reaches here through the
/// resolver's own arm for it.
///
/// # Errors
///
/// [`HeldRefusal`]: the node is not a profile; an argument is driven
/// by an expression, which a `Step<f64>` has no way to hold — the
/// whole node is refused rather than shown as numbers that would be
/// written back over a computation, and every driven argument is
/// named so the reader knows which rows to edit instead; or the
/// resolver refused a stored expression.
pub fn held_loops(
    doc: &Doc<ProfileProgram>,
    node: RecipeNodeId,
) -> Result<Vec<Vec<Step<f64>>>, HeldRefusal> {
    let Some(Node::Profile(program)) = doc.node(node) else {
        return Err(HeldRefusal::NotAProfile { node });
    };
    held_program(node, program, &doc.param_env::<f64>())
}

/// [`held_loops`] of a program in hand — `node` only names it in a
/// refusal, and `env` is the parameter environment it resolves under.
///
/// # Errors
///
/// [`HeldRefusal::Driven`] or [`HeldRefusal::Resolve`], as
/// [`held_loops`].
pub fn held_program(
    node: RecipeNodeId,
    program: &ProfileProgram,
    env: &ParamEnv<f64>,
) -> Result<Vec<Vec<Step<f64>>>, HeldRefusal> {
    let held = Node::Profile(program.clone());
    // Every argument, asked of the node's own slot walk. An address
    // the walk lists and `expr` denies is the node layer's broken
    // postcondition, which `props::slot_row` reports as a row; here
    // it reads as driven, the refusing direction.
    let driven: Vec<(SlotId, String)> = held
        .slots()
        .into_iter()
        .filter_map(|slot| match held.expr(slot) {
            Some(expr) if expr.literal_value().is_some() => None,
            Some(expr) => Some((slot, unparse(expr))),
            None => Some((slot, String::new())),
        })
        .collect();
    if !driven.is_empty() {
        return Err(HeldRefusal::Driven {
            node,
            slots: driven,
        });
    }
    resolve_loops(&program.loops, env)
        .map_err(|(slot, source)| HeldRefusal::Resolve { slot, source })
}

/// Why a committed node cannot be held by the path editor.
#[derive(Clone, Debug, PartialEq)]
pub enum HeldRefusal {
    /// The node is not a profile.
    NotAProfile {
        /// The node named.
        node: RecipeNodeId,
    },
    /// One or more arguments are expressions, which the editor's
    /// plain-number steps cannot hold. Each is named with its source
    /// text; an empty source is an address the node lists and carries
    /// no expression for.
    Driven {
        /// The profile node.
        node: RecipeNodeId,
        /// Every driven argument, in slot order.
        slots: Vec<(SlotId, String)>,
    },
    /// A stored expression did not resolve under the document's
    /// parameters.
    Resolve {
        /// The argument that refused.
        slot: SlotId,
        /// The evaluator's own reason.
        source: EvalError,
    },
}

impl core::fmt::Display for HeldRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NotAProfile { node } => write!(f, "feature {} is not a profile", node.0),
            Self::Driven { node, slots } => {
                write!(
                    f,
                    "feature {}'s program is driven by expressions, which the editor's \
                     number fields cannot hold — edit those in the slot rows: ",
                    node.0
                )?;
                for (index, (slot, source)) in slots.iter().enumerate() {
                    if index > 0 {
                        f.write_str(", ")?;
                    }
                    if source.is_empty() {
                        write!(f, "{} (no expression)", slot.label())?;
                    } else {
                        write!(f, "{} = {source}", slot.label())?;
                    }
                }
                Ok(())
            }
            Self::Resolve { slot, source } => {
                write!(f, "{} did not resolve: {source}", slot.label())
            }
        }
    }
}

impl core::error::Error for HeldRefusal {}

/// **The slot writes that take a committed program to the editor's**
/// — one `(slot, expression)` per argument whose number moved, and
/// nothing for the rest.
///
/// Compared by VALUE, at the bits ([`Expr::bit_eq`]): an argument the
/// editor re-minted in the form's notation but still holding the
/// number it was loaded with is not a change, so an editor opened on
/// a node and applied untouched writes nothing — the no-op the edit
/// door owes (no edit, no history entry). An argument that moved is
/// written as the editor minted it, in the notation the picker beside
/// the fields says it writes in.
///
/// # Errors
///
/// [`Restructure`] when `loops` does not have `current`'s STRUCTURE —
/// a different loop count, or a loop whose verbs, arc modes, target
/// forms, structural tags or step count differ. The document's edit
/// vocabulary writes slots and has no door that rewrites a program's
/// shape, which is why the editor locks its structural controls on a
/// committed node; this is the door's own check behind those
/// controls, held by writing every argument of `loops` into a copy of
/// `current` and asking whether the copy then IS `loops`.
pub fn program_edits(
    current: &ProfileProgram,
    loops: &[LoopProgram],
) -> Result<Vec<(SlotId, Expr)>, Restructure> {
    if current.loops.len() != loops.len() {
        return Err(Restructure::LoopCount {
            was: current.loops.len(),
            now: loops.len(),
        });
    }
    let held = Node::Profile(ProfileProgram {
        plane: current.plane,
        loops: loops.to_vec(),
    });
    let mut probe = Node::Profile(current.clone());
    let mut edits = Vec::new();
    for slot in held.slots() {
        let Some(new) = held.expr(slot) else {
            unreachable!(
                "`Node::slots` is the domain of `Node::expr`, and {} was listed by it",
                slot.label()
            )
        };
        let SlotId::Profile { loop_, .. } = slot else {
            unreachable!(
                "a profile node lists only profile slots, and {} is not one",
                slot.label()
            )
        };
        let Some(old) = probe.expr_mut(slot) else {
            return Err(Restructure::Loop {
                loop_: loop_ as usize,
            });
        };
        if !old.bit_eq(new) {
            edits.push((slot, new.clone()));
        }
        *old = new.clone();
    }
    // Every argument of `loops` is now written into the copy, so the
    // copy and `loops` differ exactly where the STRUCTURE does: a step
    // the copy has and `loops` lacks, a tag, a target form, a mode.
    // The comparison is the program vocabulary's own equality, which
    // reads expressions by value and is blind to notation.
    let Node::Profile(probe) = probe else {
        unreachable!("the probe was built as a profile node")
    };
    for (loop_, (was, now)) in probe.loops.iter().zip(loops).enumerate() {
        if was != now {
            return Err(Restructure::Loop { loop_ });
        }
    }
    Ok(edits)
}

/// Why the editor's program cannot be written over a committed one.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Restructure {
    /// The two programs have different loop counts.
    LoopCount {
        /// The committed loop count.
        was: usize,
        /// The editor's.
        now: usize,
    },
    /// One loop's shape — its verbs, arc modes, target forms,
    /// structural tags or step count — differs.
    Loop {
        /// The loop, in authoring order.
        loop_: usize,
    },
}

impl core::fmt::Display for Restructure {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::LoopCount { was, now } => write!(
                f,
                "the committed profile has {was} loop(s) and the editor holds {now}; the \
                 document's edit vocabulary writes a program's numbers and has no door that \
                 changes its shape"
            ),
            Self::Loop { loop_ } => write!(
                f,
                "loop {loop_}'s verbs, arc forms, targets or step count differ from the \
                 committed program's; the document's edit vocabulary writes a program's \
                 numbers and has no door that changes its shape"
            ),
        }
    }
}

impl core::error::Error for Restructure {}

// ------------------------------------------------------------------
// The preview: what the loops being authored would actually draw
// ------------------------------------------------------------------

/// **The placement a frame node landed on**, or `None` when the id
/// names no node, names one that is not a frame, or names one whose
/// evaluation did not land.
///
/// This replaces the form's world-XY constant. The form used to author
/// on a fixed plane because there was nothing in a document to point
/// at; there is now, so the plane the form draws on is READ from the
/// frame the person picked — which is also the frame they can see in
/// the viewport, which is the whole point of the datum being a node.
///
/// It reads the LANDED value rather than resolving the frame's
/// expressions again: this is a picture, the evaluation already
/// produced the placement, and a second derivation is a second answer
/// waiting to disagree with the first. The kernel reads by that same
/// rule and asks a different question: `wire::profile_plane_f64` takes
/// the `f64` placement the frame's own evaluation minted and carried
/// (`NodeValue::placement`), which is for structure selection and not
/// for drawing. The two differ in WHICH answer they take off the
/// frame's value, not in whether they take one.
pub fn frame_placement(
    doc: &Doc<ProfileProgram>,
    evaluation: &Evaluation<f64>,
    frame: RecipeNodeId,
) -> Option<SketchPlane<f64>> {
    // Either frame kind: what is drawn is the landed VALUE, which both
    // produce.
    if !matches!(
        doc.node(frame),
        Some(Node::Datum(Datum::Frame { .. } | Datum::FaceFrame { .. }))
    ) {
        return None;
    }
    let ValuePayload::Datum(DatumValue::Frame(f)) = &evaluation.value(frame)?.payload else {
        return None;
    };
    Some(SketchPlane::from_frame(*f))
}

/// **Every frame datum in the document, in document order** — what the
/// creation forms' frame picker offers.
///
/// Document order rather than sorted by id or by name: the feature
/// tree lists nodes that way, so the picker and the tree name the
/// document's frames in one order.
pub fn frames(doc: &Doc<ProfileProgram>) -> Vec<RecipeNodeId> {
    doc.order()
        .iter()
        .copied()
        .filter(|id| {
            matches!(
                doc.node(*id),
                Some(Node::Datum(Datum::Frame { .. } | Datum::FaceFrame { .. }))
            )
        })
        .collect()
}

/// **One drawn loop of a preview**: its polyline, and whether the
/// chain it came from actually closed.
///
/// The pair is one value because the two facts are one drawing
/// decision. A closed loop's last point joins its first, which is what
/// [`ProfileLoop`] means by being closed by construction; an OPEN
/// one's must not, and a consumer handed a bare point list has nothing
/// to read that from — it would either invent a leg nobody authored or
/// drop one that was.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PreviewLoop {
    /// The flattened polyline, in sketch-plane metres.
    ///
    /// For an open chain these are exactly the authored legs' vertices:
    /// the provisional closing leg contributes no point of its own, so
    /// declining to wrap is all it takes to leave it undrawn.
    pub points: Vec<[f64; 2]>,
    /// Where in [`PreviewLoop::points`] the loop's OWN vertices sit —
    /// the leg ends, as against the subdivisions a flattened arc adds
    /// between them.
    ///
    /// Ascending, and `vertices[0]` is always `0`. It is carried
    /// because a consumer cannot recover it: an arc's interior points
    /// look exactly like its ends once they are a list of numbers.
    /// What it buys is the directed point at each step — a mark AT the
    /// tip, pointing the way the chain leaves it.
    pub vertices: Vec<usize>,
    /// Whether the authored chain closes on its own.
    ///
    /// `false` is a chain still being written — every template shape
    /// closes by construction, so only the path form can produce one.
    pub closed: bool,
}

/// **A candidate profile, replayed and flattened** — the picture a
/// form shows of the loops in front of it, before any of it is a
/// document.
///
/// Sketch-plane coordinates in metres, one polyline per loop. Nothing
/// here knows where the sketch plane is; the caller places the points
/// ([`SketchPlane::to_world`]) because the caller is the one drawing
/// them.
// `Default` and `PartialEq` left the derive with the plane's arrival:
// a `SketchPlane` has neither, an empty preview on an invented plane
// would be a picture of nowhere, and two previews are compared by what
// a test asks of them (their loops) rather than wholesale.
#[derive(Clone, Debug)]
pub struct ProfilePreview {
    /// **The plane the polylines below were placed on**, carried so
    /// the viewport draws them where the replay put them.
    ///
    /// Beside the drawing rather than looked up again by the drawer,
    /// for the reason the retired `form_plane` constant existed: the
    /// preview's placement and the picture's have to be one fact, and
    /// two lookups of a frame that can move between them are two
    /// places for that to stop being true.
    pub plane: SketchPlane<f64>,
    /// One polyline per loop, in authoring order.
    pub loops: Vec<PreviewLoop>,
    /// What validation said about the replayed loops — `None` when it
    /// passed.
    ///
    /// **Carried beside a drawn picture rather than in place of
    /// one.** A profile that replays but does not validate (loops
    /// that cross, a hole that is not inside its outer) has geometry,
    /// and refusing to draw it would hide exactly the shape somebody
    /// needs to look at to see what is wrong with it. The commit door
    /// still refuses it; this only declines to make that refusal a
    /// blank pane.
    ///
    /// Always `None` while any loop is OPEN: validation is a verdict
    /// on a profile, and a chain that has not closed is not one yet.
    /// The provisional close this module draws it under is the
    /// viewer's, not the author's, so validating through it would
    /// report on a shape nobody wrote.
    pub invalid: Option<ProfileError>,
}

impl ProfilePreview {
    /// Whether any drawn chain has not closed yet — the state a commit
    /// must wait on, asked once here rather than spelled at each
    /// caller.
    pub fn has_open_chain(&self) -> bool {
        self.loops.iter().any(|drawn| !drawn.closed)
    }
}

/// Why a preview could not be drawn at all.
///
/// Distinct from [`ProfilePreview::invalid`], which is a preview that
/// WAS drawn and did not validate: these are the failures with no
/// geometry behind them — a field that is not a number or a path that
/// is not a program's shape, an expression that will not resolve, a
/// walk the lattice does not admit, a leg whose geometry has no
/// answer.
#[derive(Clone, Debug, PartialEq)]
pub enum PreviewError {
    /// The shape did not lower ([`loop_program`]'s refusals): a field
    /// is not a finite number, or a complete-loop verb sits inside a
    /// chain.
    Lowering(RecordedProgramError),
    /// A program expression did not resolve.
    Resolve {
        /// The failing slot.
        slot: SlotId,
        /// The evaluator's own refusal.
        source: pncad::document::EvalError,
    },
    /// The verbs are not a legal walk of the lattice — the tip was in
    /// `state` and `verb` is not well-typed there (`None` for a chain
    /// that simply ended without closing).
    Transition {
        /// Which loop refused.
        loop_: usize,
        /// Which step of it.
        step: usize,
        /// The tip's lattice state.
        state: TipState,
        /// The ill-typed verb, `None` for end-of-program.
        verb: Option<Verb>,
    },
    /// A loop replayed and one of the points it would be drawn
    /// through is not a place: a vertex whose own position is not a
    /// pair of finite numbers, or a bulged segment whose arc is not
    /// one — its radius, its swept angle, its centre, its start
    /// angle, or a point along it.
    ///
    /// **Separate from [`PreviewError::Geometry`]**, which carries the
    /// DRIVER's refusal about a leg somebody authored. This one is the
    /// flattener's own, about the picture rather than the profile: the
    /// loop replayed and validation may well have a verdict on it, and
    /// what has no answer is how to draw it.
    Unflattenable {
        /// Which loop could not be drawn.
        loop_: usize,
        /// The ordinal of the vertex that refused — its own
        /// position, or the segment leaving it. The loop's OWN
        /// vertex, not the authored step, because one step can
        /// contribute several and the flattener walks what replay
        /// produced.
        vertex: usize,
    },
    /// A leg's geometry refused — the driver's own rendered refusal.
    Geometry {
        /// Which loop refused.
        loop_: usize,
        /// Which step of it.
        step: usize,
        /// The driver's refusal, in its own words.
        rendered: String,
    },
}

// The preview's sentence is about the step the author is looking at,
// so both halves of the (state, verb) pair are named in the author's
// vocabulary: the verb through `profile::Verb`'s own `Display` — the
// same word the row's combo shows, so a verb cannot be picked under
// one name and refused under another — and the state through
// [`tip_state_words`]. `profile`'s `ReplayError` renders the same pair
// as the table's COORDINATE and says there why that sentence differs.
impl core::fmt::Display for PreviewError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Lowering(error) => write!(f, "{error}"),
            Self::Resolve { slot, source } => write!(f, "{}: {source}", slot.label()),
            Self::Transition {
                loop_,
                step,
                state,
                verb,
            } => match verb {
                Some(verb) => write!(
                    f,
                    "loop {loop_} step {step}: {verb} is not well-typed there — the tip is {}",
                    tip_state_words(*state)
                ),
                None => write!(
                    f,
                    "loop {loop_} never closes — it ends with the tip {}, and the last verb \
                     has to target the start",
                    tip_state_words(*state)
                ),
            },
            Self::Unflattenable { loop_, vertex } => write!(
                f,
                "loop {loop_} vertex {vertex}: nothing there can be drawn — its \
                 position, or the radius, sweep, centre or points of the arc \
                 leaving it, is not a number"
            ),
            Self::Geometry {
                loop_,
                step,
                rendered,
            } => write!(f, "loop {loop_} step {step}: {rendered}"),
        }
    }
}

impl core::error::Error for PreviewError {}

/// **Replay the loops a form is holding and flatten them for
/// drawing.**
///
/// The SAME ladder the edit door runs on commit — lower, resolve,
/// replay, validate (`ProfileProgram::check`) — run for its geometry
/// instead of for its verdict, which is what makes the picture and
/// the refusal one thing rather than two that can disagree. A form
/// that draws this and shows what it refuses is showing the reason
/// its Create button is disabled.
///
/// `chord` is the display tolerance: how far a flattened arc may sag
/// from the arc it stands for, in metres. It is a δ and never an ε —
/// the loops themselves are exact, and this only decides how many
/// points are drawn along them.
///
/// **A chain that has not closed yet is drawn, not refused.** The
/// driver's contract is that a program is a loop, so a path being
/// typed in fails its replay at the end-of-program arm — and refusing
/// the whole preview there left the viewport blank until the last step
/// landed, which is exactly when nobody needs to look at it any more.
/// Such a chain is replayed again under a PROVISIONAL `line_to Start`
/// (this module's, never recorded) and the resulting
/// [`PreviewLoop`] is marked `closed: false`, which tells the consumer
/// not to draw the leg back to the start. Every other replay refusal
/// blames a step somebody actually wrote and is still reported.
///
/// # Errors
///
/// [`PreviewError`], per arm — everything that leaves no geometry to
/// draw. A profile that replays and fails VALIDATION is a success
/// here, carrying its refusal in [`ProfilePreview::invalid`]. An
/// unclosed chain whose provisional close is itself ill-typed — a tip
/// with a direction and no position, an arc arrival still waiting for
/// a binder — reports the ORIGINAL end-of-program refusal, never one
/// belonging to the appended step. A loop that replays and has a
/// point no picture can put anywhere is
/// [`PreviewError::Unflattenable`] — the one refusal here that is
/// about the PICTURE rather than the profile, and the reason it is a
/// refusal rather than a loop drawn short is that a preview is what a
/// form shows instead of the geometry.
pub fn preview(
    plane: SketchPlane<f64>,
    shapes: &[ProfileShape],
    tol: Tol,
    chord: f64,
) -> Result<ProfilePreview, PreviewError> {
    let mut programs = Vec::with_capacity(shapes.len());
    for shape in shapes {
        // CANONICAL, and it makes no difference which: a display unit
        // is presentation metadata that no evaluation reads, and this
        // program is built to be replayed and drawn, never committed.
        programs.push(loop_program(shape, Notation::CANONICAL).map_err(PreviewError::Lowering)?);
    }
    // Literals only reach this door, so an empty environment binds
    // everything it can be asked about. It is passed rather than
    // assumed because resolution is the document layer's one door and
    // a form is not a special case of it. The LOOPS resolve, not a
    // program: a preview has no plane node and does not need one — the
    // plane it draws on arrives as a placement, from the frame the
    // form is pointed at.
    let env = ParamEnv::default();
    let resolved = resolve_loops(&programs, &env)
        .map_err(|(slot, source)| PreviewError::Resolve { slot, source })?;
    let mut loops: Vec<ProfileLoop<f64>> = Vec::with_capacity(resolved.len());
    let mut closed_flags: Vec<bool> = Vec::with_capacity(resolved.len());
    for (index, steps) in resolved.iter().enumerate() {
        match replay(steps, tol) {
            Ok(replayed) => {
                loops.push(replayed);
                closed_flags.push(true);
            }
            // **A chain that has not closed YET still draws.**
            //
            // `replay` requires a closing verb — a program is a LOOP,
            // and half a loop is not one — so a path being typed in
            // refused the whole preview and the viewport stayed blank
            // until the last step landed, which is precisely when a
            // person no longer needs to see it.
            //
            // The end-of-program arm (`verb: None`) is the only one
            // that means "unfinished" rather than "wrong": every other
            // refusal blames a step that was authored. So that arm,
            // and only it, is retried under a PROVISIONAL closing leg
            // — `line_to Start`, appended here and never recorded
            // anywhere — which is enough to make the driver hand back
            // the geometry it already walked. The leg itself is not
            // drawn: it contributes no vertex, so a consumer that
            // declines to wrap an open polyline draws exactly the legs
            // that were authored and nothing else.
            //
            // Nothing about the lattice is re-implemented to do it.
            // The provisional close goes through the same `replay` as
            // everything else, and when it is ill-typed at the tip
            // (a bound direction with no position, an arc arrival
            // still waiting for a binder) the ORIGINAL refusal is
            // reported — never one belonging to a step nobody wrote.
            Err(error) if matches!(error.kind, ReplayErrorKind::Transition { verb: None, .. }) => {
                let mut provisional = steps.clone();
                provisional.push(Step::LineTo(Target::Start));
                match replay(&provisional, tol) {
                    Ok(replayed) => {
                        loops.push(replayed);
                        closed_flags.push(false);
                    }
                    Err(_) => return Err(refusal(index, &error)),
                }
            }
            Err(error) => return Err(refusal(index, &error)),
        }
    }
    let open = closed_flags.iter().any(|closed| !closed);
    let polylines = loops
        .iter()
        .zip(&closed_flags)
        .enumerate()
        .map(|(loop_, (lp, closed))| {
            let (points, vertices) = flatten(lp.vertices(), lp.bulges().iter().copied(), chord)
                .map_err(|vertex| PreviewError::Unflattenable { loop_, vertex })?;
            Ok(PreviewLoop {
                points,
                vertices,
                closed: *closed,
            })
        })
        .collect::<Result<Vec<_>, PreviewError>>()?;
    // A profile is what validation has a verdict about, and an
    // unfinished chain is not one. Validating the provisional close
    // would report on a leg the author never wrote.
    let invalid = if open {
        None
    } else {
        Profile::new(plane, loops).validate(tol).err()
    };
    Ok(ProfilePreview {
        plane,
        loops: polylines,
        invalid,
    })
}

/// **One committed profile, flattened for drawing**: the node it is,
/// the plane its evaluation placed it on, and its loops.
#[derive(Clone, Debug)]
pub struct CommittedProfile {
    /// The profile node this draws.
    pub node: RecipeNodeId,
    /// The plane the landed value is on — the VALUE's, not a second
    /// lookup of the frame the node names, for
    /// [`ProfilePreview::plane`]'s reason.
    pub plane: SketchPlane<f64>,
    /// One closed polyline per loop, in the value's canonical order
    /// (outer first). Every one is `closed: true`: a validated profile
    /// has no open chain.
    pub loops: Vec<PreviewLoop>,
}

/// **What the landed evaluation's profiles came to as drawings**: the
/// ones that flattened, and the ones that did not.
///
/// A value rather than a bare list for `crate::datums::DatumDraws`'
/// reason: a profile left out of the picture looks exactly like a
/// document without it, so a caller holding the drawings is handed
/// the refusals too.
#[derive(Clone, Debug, Default)]
pub struct CommittedProfiles {
    /// One per profile node drawn, in document order.
    pub drawn: Vec<CommittedProfile>,
    /// The profile nodes whose validated value has a point the
    /// flattener cannot draw ([`PreviewError::Unflattenable`]'s case),
    /// in document order. Drawn not at all rather than with that leg
    /// missing: a loop drawn without one of its legs is a shape the
    /// document does not have.
    pub undrawn: Vec<RecipeNodeId>,
}

/// **Every profile the landed evaluation validated**, flattened at
/// `chord` — the picture of a committed profile, drawn from the same
/// value the features built on it consume.
///
/// Walks the document's live nodes in order. The NODE says it is a
/// profile and the EVALUATION says what it came to, as in
/// `crate::datums::draws`: a profile node whose evaluation refused, or
/// one this evaluation never reached, has no value and draws nothing —
/// the tree's badge is what says why, and drawing the program's replay
/// in its place would put a shape on screen the document does not
/// have.
///
/// `except` is the profile a form is EDITING, if any: that form draws
/// its own replay of the node in the preview lane, and a loop drawn
/// both as it was committed and as it is being changed would show two
/// shapes where there is one. The create form edits no committed node,
/// so it passes `None`.
pub fn committed(
    doc: &Doc<ProfileProgram>,
    evaluation: &Evaluation<f64>,
    chord: f64,
    except: Option<RecipeNodeId>,
) -> CommittedProfiles {
    let mut out = CommittedProfiles::default();
    for &node in doc.order() {
        if Some(node) == except || !matches!(doc.node(node), Some(Node::Profile(_))) {
            continue;
        }
        let Some(value) = evaluation.value(node) else {
            continue;
        };
        let ValuePayload::Profile(profile) = &value.payload else {
            continue;
        };
        let loops = profile
            .validated
            .loops()
            .iter()
            .map(|lp| {
                let bulges = lp.segments().iter().map(|s| s.bulge);
                flatten(lp.vertices(), bulges, chord).map(|(points, vertices)| PreviewLoop {
                    points,
                    vertices,
                    closed: true,
                })
            })
            .collect::<Result<Vec<_>, usize>>();
        match loops {
            Ok(loops) => out.drawn.push(CommittedProfile {
                node,
                plane: *profile.validated.plane(),
                loops,
            }),
            Err(_) => out.undrawn.push(node),
        }
    }
    out
}

/// **A lattice tip state in words.**
///
/// [`TipState`]'s own `Debug` is the variant name — `PlainPoint`,
/// `RadiusArrivalDir` — which is the right thing in a backtrace and
/// the wrong thing in a tooltip: it names the state without saying
/// what about the chain put it there. One home for the phrasing,
/// because both places a reader meets a tip state (this module's
/// refusal sentence and the form's greyed-out verbs) have to call the
/// same state the same thing.
pub fn tip_state_words(state: TipState) -> &'static str {
    match state {
        TipState::Entry => "at the entry, before any verb",
        TipState::Open => "a freshly opened arrival side, with nothing bound",
        TipState::Angle => "a bound direction with no position yet",
        TipState::PlainPoint => "a bound position with no incoming tangent",
        TipState::DirectedPoint => "a leg end, with an incoming tangent",
        TipState::DirectedPlain => "a bound position and direction, over a plain point",
        TipState::DirectedIncoming => "a leg end with a direction bound over it",
        TipState::RadiusArrival => "a radius arrival still awaiting both binders",
        TipState::RadiusArrivalAt => "a radius arrival with its anchor bound",
        TipState::RadiusArrivalDir => "a radius arrival with its director bound",
        TipState::ViaArrival => "a via arrival awaiting its director",
        TipState::ViaArrivalStart => "a via close awaiting its director",
        TipState::Closed => "a closed loop, which no verb may follow",
    }
}

/// **The lattice state the chain is in just before `steps[at]`** —
/// asked of the replay the commit door runs, over the prefix alone.
///
/// `None` when the prefix does not say: a field that is not a number
/// (the preview beside the form reports it), or a prefix already
/// refused before `at` — that refusal is the prefix's own and the form
/// is already showing it. A prefix that closes answers
/// [`TipState::Closed`].
pub fn tip_state_at(steps: &[Step<f64>], at: usize, tol: Tol) -> Option<TipState> {
    let prefix = &steps[..at];
    // Each step is lifted ALONE, so what is asked is only whether its
    // literals are numbers — a complete-loop verb inside a chain is
    // the lattice's to refuse, and a whole-chain lift would refuse it
    // first.
    if prefix
        .iter()
        .any(|step| LoopProgram::from_recorded(core::slice::from_ref(step)).is_err())
    {
        return None;
    }
    match replay(prefix, tol) {
        Ok(_) => Some(TipState::Closed),
        Err(ReplayError {
            step,
            kind: ReplayErrorKind::Transition { state, verb: None },
        }) if step == at => Some(state),
        Err(_) => None,
    }
}

/// **Does the lattice have a row for `verb` at `state`?** — read off
/// the transition table ([`Verb::states`]), not probed. `None` (the
/// state is not known, [`tip_state_at`]) admits every verb: the form
/// cannot say more than the chain does, and the preview reports
/// whatever the replay refuses.
///
/// # Errors
///
/// The tip's state, when the table has no row for `verb` there — the
/// sentence a form greys a choice out with.
pub fn admits_at(state: Option<TipState>, verb: Verb) -> Result<(), TipState> {
    match state {
        Some(state) if !verb.states().contains(&state) => Err(state),
        _ => Ok(()),
    }
}

/// **A step of `verb` whose arc specs are ones the lattice takes at
/// `state`**: [`fresh_step`], with each arc spec replaced by the first
/// form its dispatcher admits there ([`arc_specs_at`]). A verb picked
/// from the combo is well-typed the moment it lands, rather than
/// starting in a mode its row refuses and waiting to be switched.
pub fn fresh_step_at(verb: Verb, state: Option<TipState>) -> Step<f64> {
    let mut step = fresh_step(verb);
    let Some(state) = state else {
        return step;
    };
    let forms = arc_specs_at(verb, state);
    let fresh = |at: usize, spec: &mut ArcData<f64>| {
        if let Some(fresh) = forms.get(at).and_then(|forms| fresh_spec(forms)) {
            *spec = fresh;
        }
    };
    match &mut step {
        Step::ArcTo(spec) | Step::FilletArc { spec, .. } | Step::ArcFillet { spec, .. } => {
            fresh(0, spec);
        }
        Step::ArcFilletArc { spec, spec2, .. } => {
            fresh(0, spec);
            fresh(1, spec2);
        }
        _ => {}
    }
    step
}

/// The first form `forms` admits, as a spec: its mode's starting
/// numbers ([`fresh_arc`]) at its first admitted target form.
pub fn fresh_spec(forms: &SpecForms) -> Option<ArcData<f64>> {
    let &(mode, _) = forms.forms().first()?;
    Some(fresh_arc_in(mode, forms))
}

/// [`fresh_arc`] of `mode`, its target set to the first form `forms`
/// admits for that mode — so switching a spec's mode lands on a
/// well-typed (mode, target) pair when one exists.
pub fn fresh_arc_in(mode: ArcMode, forms: &SpecForms) -> ArcData<f64> {
    let mut spec = fresh_arc(mode);
    if let (Some(kind), Some(target)) = (forms.targets(mode).next(), spec_target_mut(&mut spec)) {
        *target = fresh_target(kind);
    }
    spec
}

fn spec_target_mut(spec: &mut ArcData<f64>) -> Option<&mut Target<f64>> {
    match spec {
        ArcData::Bulge { target, .. }
        | ArcData::Via { target, .. }
        | ArcData::Center { target, .. } => Some(target),
        ArcData::Radius { .. } | ArcData::Sweep { .. } | ArcData::ArcLen { .. } => None,
    }
}

/// One replay refusal as this module's own, naming the loop it came
/// from.
///
/// Extracted because it is now read from two places — the plain
/// refusal and the one a provisional close failed to rescue — and two
/// copies of a mapping are two places for it to drift.
fn refusal(loop_: usize, error: &ReplayError<f64>) -> PreviewError {
    match error.kind {
        ReplayErrorKind::Transition { state, verb } => PreviewError::Transition {
            loop_,
            step: error.step,
            state,
            verb,
        },
        ReplayErrorKind::Path(ref source) => PreviewError::Geometry {
            loop_,
            step: error.step,
            rendered: source.to_string(),
        },
    }
}

/// **The most points one flattened arc is allowed.**
///
/// A cap, not a budget: the count comes from the chord tolerance, and
/// this only stops a radius large enough to make that arithmetic ask
/// for a million points from doing so. At 256 a full circle is drawn
/// with under a degree and a half between points, which is finer than
/// any preview pane resolves.
const MAX_ARC_POINTS: usize = 256;

/// **Whether a flattened point is a place**: both coordinates finite.
///
/// Asked of every coordinate [`flatten`] emits — the loop's own
/// vertices and an arc's interior points alike — because the drawn
/// output is what this module answers for, and a polyline carrying a
/// point that is not a pair of numbers is a picture of nowhere.
///
/// **Every is checkable**: the two `out.push` calls in [`flatten`] are
/// the only places a coordinate joins the output, so a reader holds
/// the whole population by grepping that function for `out.push`. The
/// arc frame goes through here too, one call further down, which is
/// the same question about a `[f64; 2]` and not a second one.
fn drawable(point: [f64; 2]) -> bool {
    point[0].is_finite() && point[1].is_finite()
}

/// One loop as a closed polyline: every vertex, with each bulged
/// segment subdivided finely enough that it sags less than `chord`.
///
/// The bulge convention is [`pncad::profile::ProfileVertex`]'s
/// — `b = tan(θ/4)` for the segment LEAVING each vertex, positive
/// counterclockwise, the last vertex's belonging to the closing
/// segment — so this reads the loop exactly as the kernel writes it
/// and invents no second convention.
///
/// # Errors
///
/// The vertex ordinal at which a point stopped being one: the
/// vertex's own position, the arc frame of the segment leaving it, or
/// a point along that arc. **Refused rather than skipped**: a segment
/// dropped here would leave the loop drawn with a leg it does not
/// have, and a vertex dropped would put the legs either side of it
/// through a corner nobody authored — the same defect one door
/// along.
fn flatten(
    vertices: &[Point2<f64>],
    bulges: impl IntoIterator<Item = f64>,
    chord: f64,
) -> Result<(Vec<[f64; 2]>, Vec<usize>), usize> {
    let mut out: Vec<[f64; 2]> = Vec::with_capacity(vertices.len());
    // Where each real vertex landed among the subdivisions. A caller
    // that wants to mark the loop's own points cannot recover this
    // afterwards — an arc's interior points are geometrically
    // indistinguishable from its ends — so the flattener, which is the
    // one place that knows, says it.
    let mut at: Vec<usize> = Vec::with_capacity(vertices.len());
    for (index, (&from, bulge)) in vertices.iter().zip(bulges).enumerate() {
        let to = vertices[(index + 1) % vertices.len()];
        // The loop's own vertex, asked the same question its arcs are
        // asked below and asked BEFORE it is emitted. A replay whose
        // literals are all finite can still land one past the top of
        // the exponent range, and every guard under this loop is about
        // an arc — so a loop with no bulges at all reaches none of
        // them and a polygon drawn through a point that is nowhere is
        // exactly what this module says it refuses.
        let place = [from.x, from.y];
        if !drawable(place) {
            return Err(index);
        }
        at.push(out.len());
        out.push(place);
        if bulge == 0.0 {
            continue;
        }
        // θ is the segment's included angle, signed with the bulge;
        // the carrier's centre sits on the left of travel for a
        // positive one, and the sign of `tan(θ/2)` is what carries
        // that across the half turn (a major arc's centre is on the
        // other side of its own chord).
        let theta = 4.0 * bulge.atan();
        let (dx, dy) = (to.x - from.x, to.y - from.y);
        let half = dx.hypot(dy) / 2.0;
        let sin_half = (theta / 2.0).sin();
        if half == 0.0 || sin_half == 0.0 {
            continue;
        }
        let radius = (half / sin_half).abs();
        let apothem = half / (theta / 2.0).tan();
        // The left normal of travel, unit length.
        let (nx, ny) = (-dy / (2.0 * half), dx / (2.0 * half));
        let centre = [
            (from.x + to.x) / 2.0 + nx * apothem,
            (from.y + to.y) / 2.0 + ny * apothem,
        ];
        let start = (from.y - centre[1]).atan2(from.x - centre[0]);
        // **Every point below is `centre + radius·(cos, sin)` of an
        // angle built from `start` and `theta`**, so those four are
        // asked to be numbers before any of them is used. The two
        // guards above this block — `bulge == 0.0` and `half == 0.0
        // || sin_half == 0.0` — are the degenerate segments a loop
        // legitimately holds, and a value that is not a number takes
        // neither side of either: a `NaN` is not equal to zero, so it
        // reads as an ordinary arc all the way to the coordinates.
        //
        // **A frame of four numbers does not make a point one**, so
        // each point is asked again as it is minted: a centre a few
        // hundred orders of magnitude from the origin and a radius to
        // match sum past the top of the range on the far side of the
        // arc, with every value here finite.
        //
        // `radius` carries `centre` with it. `apothem` is
        // `±radius·cos(θ/2)` written as `half / tan(θ/2)`, so it is
        // bounded by `radius`; a `half` that is not finite makes
        // `radius` not finite too. What `radius` does NOT carry is the
        // chord's own midpoint, which overflows on its own for two
        // vertices near the top of the exponent range — hence
        // `centre`, and `start` after it.
        let Some(count) =
            arc_points(radius, theta, chord).filter(|_| drawable(centre) && start.is_finite())
        else {
            return Err(index);
        };
        for ordinal in 1..count {
            let angle = start + theta * (ordinal as f64) / (count as f64);
            let place = [
                centre[0] + radius * angle.cos(),
                centre[1] + radius * angle.sin(),
            ];
            if !drawable(place) {
                return Err(index);
            }
            out.push(place);
        }
    }
    Ok((out, at))
}

/// How many chords one arc of `radius` sweeping `theta` needs to sag
/// less than `chord`, or `None` when the three are not numbers to
/// answer from.
///
/// The sagitta of a sub-arc of angle φ is `r(1 - cos(φ/2))`, so the
/// admissible φ inverts that; a `chord` at or past the diameter asks
/// for no subdivision at all and gets the one-segment floor.
///
/// **The three guards below order their inputs, and the finite test is
/// what makes that an assumption they are allowed to make.** A `NaN`
/// takes NEITHER side of `chord <= 0.0`, of `ratio <= -1.0` or of
/// `step <= 0.0`, so every one of them used to fall through; the
/// arithmetic past them then answered `(NaN).ceil() as usize`, which
/// is `0`, which `clamp(1, MAX)` lifted to the one-segment floor. An
/// arc whose subdivision could not be computed was drawn as an arc
/// that needs one segment. An unbounded radius fell through in the
/// other direction and reached the cap, and its points are `±inf` or
/// `NaN` however many of them are drawn.
///
/// [`MAX_ARC_POINTS`] is still the answer for a genuinely coarse
/// request, and `1` for a genuinely flat one; `None` is neither, which
/// is the whole point of it being a third answer rather than one of
/// those two.
fn arc_points(radius: f64, theta: f64, chord: f64) -> Option<usize> {
    if !(radius.is_finite() && theta.is_finite() && chord.is_finite()) {
        return None;
    }
    if chord <= 0.0 || radius <= 0.0 {
        return Some(MAX_ARC_POINTS);
    }
    let ratio = 1.0 - chord / radius;
    if ratio <= -1.0 {
        return Some(1);
    }
    let step = 2.0 * ratio.clamp(-1.0, 1.0).acos();
    if step <= 0.0 {
        return Some(MAX_ARC_POINTS);
    }
    Some(((theta.abs() / step).ceil() as usize).clamp(1, MAX_ARC_POINTS))
}

/// **How big a tip mark in a profile preview is, in PIXELS** — the
/// cross-tick through a vertex; the heading arrow's tip sits this far
/// ahead of it.
///
/// Screen-sized, like every datum glyph (`datums`), because a tip mark
/// is an annotation on the chain and not a part of it: it has to read
/// at whatever zoom the chain is being looked at. A mark sized against
/// the model's extent is a fixed length in metres, so zooming in on a
/// small feature of a large profile blows the marks up across it, and
/// zooming out shrinks them below a pixel.
pub const TIP_MARK_PX: f64 = 20.0;

/// **Which way the chain leaves the vertex at `at`** — the separation
/// divided by its own length, or `None` where there is none to be had.
///
/// The next flattened point, which is the tangent to within the chord
/// tolerance the preview was flattened at. At the LAST vertex of an
/// open chain there is no leaving direction, so the INCOMING one is
/// answered instead: that tip is where the chain currently ends, and
/// the heading a reader wants there is the one it arrived on.
///
/// **A separation that is not a finite length has no direction
/// either**, which is the second thing the `None` arm says. Two
/// flattened points a few hundred orders of magnitude apart differ by
/// ordinary numbers whose `hypot` overflows: the length is then `inf`,
/// which is greater than zero, and each component divided by it is
/// `0.0`. A zero vector handed out under the name of a unit one is
/// this arm not being taken — the caller draws its tip mark along
/// nothing and cannot tell that from a mark it drew.
///
/// **Unit LENGTH is a property of the separation and not of the
/// guard.** What the guard buys is that no component exceeds a finite
/// length, so each quotient lands in `[-1, 1]`; the pair is a unit
/// vector to within a rounding step only while the length is a NORMAL
/// number. Below that the division has no precision left to divide
/// with: `dx = dy = 1e-320` answers a length of 1.000129 and
/// `dx = dy = 5e-324` answers `[1.0, 1.0]`, of length 1.4142.
///
/// It is still a DIRECTION there, which is why this is stated rather
/// than refused. Both components are divided by one length, and that
/// length's own rounding is a common factor: over every separation
/// whose components are the first 400 multiples of `5e-324` the angle
/// is wrong by at most one rounding step (2.2e-16 rad) while the
/// length is wrong by up to 41%. The caller scales a screen mark by
/// the pair, so what a subnormal separation costs is a mark up to 41%
/// long and pointing the right way — and `None`, this door's only
/// other answer, would draw no mark at all.
pub fn heading(points: &[[f64; 2]], at: usize, closed: bool) -> Option<[f64; 2]> {
    let (from, to) = if at + 1 < points.len() {
        (points[at], points[at + 1])
    } else if closed && points.len() > 1 {
        (points[at], points[0])
    } else if at > 0 {
        (points[at - 1], points[at])
    } else {
        return None;
    };
    let (dx, dy) = (to[0] - from[0], to[1] - from[1]);
    let length = dx.hypot(dy);
    // Finite and non-zero is what makes each quotient below land in
    // [-1, 1]; normal is what makes the pair unit length. See above.
    (length.is_finite() && length > 0.0).then(|| [dx / length, dy / length])
}

#[cfg(test)]
mod tests {
    use super::arc_points;

    /// **A count the arithmetic could not compute is not a count.**
    ///
    /// `f64::clamp` returns `self` for a `NaN` and `NaN as usize` is
    /// `0`, so a subdivision that could not be computed used to arrive
    /// as the one-segment floor — indistinguishable from the arc that
    /// genuinely needs one segment. An infinite radius arrived as the
    /// cap, indistinguishable from the arc that genuinely needs 256.
    ///
    /// What this row holds is that DISTINCTION, not the absence of a
    /// `NaN`: the poisoned answer is compared against one of each
    /// legitimate answer the door gives — the floor, the cap, and an
    /// ordinary count between them — because an assertion that only
    /// said "not a number came back" would pin neither.
    #[test]
    fn an_arc_that_cannot_be_measured_gets_no_count() {
        let floor = arc_points(1.0, 1.0, 2.0);
        let cap = arc_points(1.0, core::f64::consts::TAU, 1.0e-6);
        let ordinary = arc_points(1.0, 1.0, 0.1);
        let legitimate = [floor, cap, ordinary];
        for (what, radius, theta, chord) in [
            ("a radius", f64::NAN, 1.0, 0.1),
            ("a swept angle", 1.0, f64::NAN, 0.1),
            ("a chord tolerance", 1.0, 1.0, f64::NAN),
            ("an unbounded radius", f64::INFINITY, 1.0, 0.1),
        ] {
            let answer = arc_points(radius, theta, chord);
            assert!(
                !legitimate.contains(&answer),
                "{what} that is not a number answered {answer:?}",
            );
        }
    }

    /// The three legitimate answers this door gives are three
    /// different answers, which is what makes the row above a test of
    /// anything: comparing against a set whose members had collapsed
    /// would pass over a door that answers one value for everything.
    #[test]
    fn the_legitimate_answers_are_distinct() {
        let floor = arc_points(1.0, 1.0, 2.0);
        let cap = arc_points(1.0, core::f64::consts::TAU, 1.0e-6);
        let ordinary = arc_points(1.0, 1.0, 0.1);
        assert_ne!(floor, ordinary, "the floor and an ordinary count");
        assert_ne!(ordinary, cap, "an ordinary count and the cap");
        // All three pairs. A set of three has three of them, and
        // checking the two adjacent ones leaves this one unread.
        assert_ne!(floor, cap, "the floor and the cap");
    }
}
