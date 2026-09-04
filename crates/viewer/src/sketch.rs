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
//! `Expr` it would have had to build a second way. The step lowering
//! ([`program_step`]) is exhaustive on the verb vocabulary, so a verb
//! the document layer gains cannot be silently unauthorable from the
//! chrome.
//!
//! # What is judged here, and what is not
//!
//! Only the literals: a non-finite field refuses typed
//! ([`DimensionError`], the literal constructors' one refusal).
//! Whether the verbs form a legal walk of the lattice, whether the
//! geometry closes, and whether the loops nest are all the profile
//! layer's questions, asked at replay — by the edit door on commit,
//! and by [`preview`] before it, which is the same ladder run for the
//! picture instead of for the verdict.

use pncad::document::{
    Datum, DatumValue, Dimension, DimensionError, Doc, Evaluation, Expr, LoopProgram, Node,
    ParamEnv, ProfileProgram, ProgramArcData, ProgramStep, ProgramTarget, RecipeNodeId, SlotId,
    ValuePayload, resolve_loops,
};
use pncad::geom_core::Tol;
use pncad::profile::{
    ArcSide, ArcSweep, Profile, ProfileError, ProfileLoop, ReplayError, ReplayErrorKind,
    SketchPlane, Step, Target, TipState, Verb, replay,
};
use pncad::quantity::{self, AngleUnit, LengthUnit, WrittenAngle, WrittenLength};

/// **Where a path targets** — the document vocabulary's
/// [`ProgramTarget`] as plain numbers.
///
/// [`PathTarget::Start`] is not "the first point again": it is the
/// bound ENTRY, and a verb aimed at it CLOSES the loop, structurally
/// (the PATHS algebra has no `close()` alias — see
/// `pncad::profile::path`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PathTarget {
    /// An absolute point in the sketch frame, metres.
    Point([f64; 2]),
    /// The entry vertex: this verb closes the loop.
    Start,
}

/// **How an arc leg is specified** — the document vocabulary's
/// [`ProgramArcData`] as plain numbers, one arm per authoring mode.
///
/// The modes are not interchangeable and no two of them say the same
/// arc twice: each names the quantities its author actually knows,
/// and everything else is derived at replay. That is the whole reason
/// the mode is part of the recorded program rather than a bulge
/// computed once at authoring time (PROFILES-V2 §V1/§V2 — a derived
/// number recorded as authored is a number that stops moving when its
/// inputs do).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ArcSpec {
    /// `radius(r, side)` — the carrier's radius and which side of the
    /// tangent its centre sits on; the endpoint is derived.
    Radius {
        /// The carrier radius, metres.
        r: f64,
        /// Which side of travel the centre sits on.
        side: ArcSide,
    },
    /// `bulge(target, b)` — the endpoint and the AUTHORED bulge
    /// `tan(θ/4)`.
    Bulge {
        /// Where the arc ends.
        target: PathTarget,
        /// The bulge, dimensionless.
        b: f64,
    },
    /// `via(q, target)` — a point the arc passes through, and where
    /// it ends; the bulge is derived.
    Via {
        /// A point on the arc, metres.
        q: [f64; 2],
        /// Where the arc ends.
        target: PathTarget,
    },
    /// `center(c, winding, target)` — the carrier centre, the travel
    /// sense, and the endpoint.
    Center {
        /// The carrier centre, metres.
        c: [f64; 2],
        /// Which way round the arc travels.
        winding: ArcSweep,
        /// Where the arc ends.
        target: PathTarget,
    },
    /// `sweep(r, side, angle)` — the carrier and how far round it to
    /// go; the endpoint is derived.
    Sweep {
        /// The carrier radius, metres.
        r: f64,
        /// Which side of travel the centre sits on.
        side: ArcSide,
        /// The swept central angle, radians.
        angle: f64,
    },
    /// `arc_len(r, side, len)` — the carrier and the distance
    /// travelled along it.
    ArcLen {
        /// The carrier radius, metres.
        r: f64,
        /// Which side of travel the centre sits on.
        side: ArcSide,
        /// The arc length, metres.
        len: f64,
    },
}

/// **One authoring verb of a path loop** — the document vocabulary's
/// [`ProgramStep`] as plain numbers, which is what makes it a form's
/// currency: the chrome hands the session numbers in canonical units
/// and the session mints the `Expr` slots, exactly as it does for
/// every other creation door.
///
/// The whole verb set is here, and it is here BECAUSE it is the whole
/// set: a form offering half a vocabulary is a form whose user has to
/// leave it to say the other half. Which verbs are well-typed at a
/// given tip is not this value's business and is not checked here —
/// the lattice decides that at replay, and an ill-typed walk refuses
/// typed at the edit door naming the state and the verb
/// (`ProgramRefusal::Transition`). [`preview`] is how a form asks that
/// question before committing.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PathStep {
    /// `.at(p)` — bind the tip's position.
    At([f64; 2]),
    /// `.angle(θ)` — bind the tip's outgoing direction, radians from
    /// +x.
    Angle(f64),
    /// `.toward(dx, dy)` — bind the outgoing direction by exact
    /// components (a ratio; the magnitude is not used).
    Toward {
        /// x component, dimensionless.
        dx: f64,
        /// y component, dimensionless.
        dy: f64,
    },
    /// `.tangent()` — leave along the incoming tangent, exactly.
    Tangent,
    /// `.cusp()` — leave along the REVERSE of the incoming tangent,
    /// the declared reverse-tangent junction.
    Cusp,
    /// `.turn(δ)` — leave at δ from the incoming tangent, radians.
    Turn(f64),
    /// `line(len)` — a straight leg of `len` metres along the bound
    /// direction.
    Line(f64),
    /// `line_to(target)` — a straight leg to an authored point, or to
    /// `Start`, which closes.
    LineTo(PathTarget),
    /// `arc_to(spec)` — a sharp (non-tangent) arc leg.
    ArcTo(ArcSpec),
    /// `tangent_arc_to(target)` — an arc leaving along the bound
    /// direction and ending at `target`.
    TangentArcTo(PathTarget),
    /// `arc_continue(p)` — a structural vertex ON the incoming
    /// carrier: the declared-subdivision verb, which splits an arc
    /// without changing it.
    ArcContinue([f64; 2]),
    /// `.fillet(r)` — round the corner with radius `r`, line in, line
    /// out.
    Fillet(f64),
    /// `fillet_arc(r, spec)` — a fillet whose ARRIVAL side is an arc.
    FilletArc {
        /// The fillet radius, metres.
        radius: f64,
        /// The arrival arc.
        spec: ArcSpec,
    },
    /// `arc_fillet(spec, r)` — a fillet whose INCOMING side is an
    /// arc.
    ArcFillet {
        /// The incoming arc.
        spec: ArcSpec,
        /// The fillet radius, metres.
        radius: f64,
    },
    /// `arc_fillet_arc(spec, r, spec2)` — a fillet with an arc on
    /// both sides.
    ArcFilletArc {
        /// The incoming arc.
        spec: ArcSpec,
        /// The fillet radius, metres.
        radius: f64,
        /// The arrival arc.
        spec2: ArcSpec,
    },
    /// `.to(anchor)` — the far-end anchor a fillet's arrival side is
    /// aimed at.
    FarEndTo([f64; 2]),
    /// `.to(Start)` — the seam fillet's close.
    CloseTo,
}

/// One loop of the add-profile door: a template shape, or a PATH
/// authored verb by verb.
///
/// **The templates are not the vocabulary; they are shortcuts into
/// it.** A circle is its own thing — a seamless closed carrier no
/// chain of legs can spell — and a rectangle is four `line_to`s
/// somebody would otherwise type. Everything else a profile can be is
/// [`ProfileShape::Path`], which carries the algebra's whole verb set
/// ([`PathStep`]).
///
/// The session lowers each arm to its [`LoopProgram`] form and
/// refuses a non-finite field typed; a degenerate loop (zero radius,
/// zero width) and an ill-typed lattice walk both refuse through the
/// edit door's own authoring-time check, exactly as a hand-written
/// program would.
#[derive(Clone, Debug, PartialEq)]
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
    /// A chain of authoring verbs ([`LoopProgram::Chain`]) — the
    /// PATHS algebra recorded as data.
    Path {
        /// The verbs, in authoring order. A chain must END in a
        /// `Start`-targeting verb; that is checked by replay, not by
        /// this representation.
        steps: Vec<PathStep>,
    },
}

/// **The notation a form is authoring in** — one length unit and one
/// angle unit, carried into every literal a lowering mints.
///
/// It exists because the units are a fact about the PERSON at the
/// keyboard rather than about any one field (`app`'s drafts say so),
/// and threading two units through a dozen recursive lowering
/// functions as loose arguments is how one of them ends up
/// canonical by accident.
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

    /// An `Angle` literal from already-canonical radians.
    fn angle(self, radians: f64) -> Result<Expr, DimensionError> {
        Expr::written_angle(WrittenAngle::canonical_in(radians, self.angle))
    }

    /// A dimensionless literal — a bulge, a director component. No
    /// notation to CHOOSE, rather than none to remember: there is one
    /// way to write a dimensionless number, and `Expr::literal` stores
    /// that row itself.
    fn scalar(self, v: f64) -> Result<Expr, DimensionError> {
        Expr::literal(v, Dimension::Scalar)
    }

    /// A literal point.
    fn point(self, p: [f64; 2]) -> Result<[Expr; 2], DimensionError> {
        Ok([self.length(p[0])?, self.length(p[1])?])
    }
}

/// Lower one template shape to its loop program, minting every literal
/// in `notation`.
///
/// **The `LoopProgram` variants are built here rather than through
/// `LoopProgram::circle` / `polygon`**, which is the one thing in this
/// function a reader will want to fold back: those constructors take
/// `f64` and mint CANONICAL literals, so routing through them would
/// drop the notation this function exists to carry. They stay the right
/// door for a caller with nothing to remember.
///
/// # Errors
///
/// A non-finite field (the literal door's refusal). Degeneracy — a
/// zero radius, a zero width — is NOT judged here: the edit door's
/// authoring-time check replays the program and refuses it typed,
/// which is one rule for authored and hand-written programs alike.
pub fn loop_program(
    shape: &ProfileShape,
    notation: Notation,
) -> Result<LoopProgram, DimensionError> {
    match shape {
        ProfileShape::Circle { centre, radius } => Ok(LoopProgram::Circle {
            centre: notation.point(*centre)?,
            radius: notation.length(*radius)?,
        }),
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
            let mut steps = Vec::with_capacity(corners.len() + 1);
            for (i, (x, y)) in corners.into_iter().enumerate() {
                let p = notation.point([x, y])?;
                steps.push(if i == 0 {
                    ProgramStep::At(p)
                } else {
                    ProgramStep::LineTo(ProgramTarget::Point(p))
                });
            }
            steps.push(ProgramStep::LineTo(ProgramTarget::Start));
            Ok(LoopProgram::Chain(steps))
        }
        ProfileShape::Path { steps } => Ok(LoopProgram::Chain(
            steps
                .iter()
                .map(|step| program_step(step, notation))
                .collect::<Result<Vec<_>, _>>()?,
        )),
    }
}

/// Lower one authored target.
fn program_target(target: PathTarget, n: Notation) -> Result<ProgramTarget, DimensionError> {
    Ok(match target {
        PathTarget::Point(p) => ProgramTarget::Point(n.point(p)?),
        PathTarget::Start => ProgramTarget::Start,
    })
}

/// Lower one authored arc spec — the dimension of every field is the
/// vocabulary's ([`SlotId::dimension`]'s table: radii and coordinates
/// Length, the swept angle Angle, the bulge Scalar), so a form cannot
/// author a radius that is secretly an angle.
fn program_arc(spec: ArcSpec, n: Notation) -> Result<ProgramArcData, DimensionError> {
    Ok(match spec {
        ArcSpec::Radius { r, side } => ProgramArcData::Radius {
            r: n.length(r)?,
            side,
        },
        ArcSpec::Bulge { target, b } => ProgramArcData::Bulge {
            target: program_target(target, n)?,
            b: n.scalar(b)?,
        },
        ArcSpec::Via { q, target } => ProgramArcData::Via {
            q: n.point(q)?,
            target: program_target(target, n)?,
        },
        ArcSpec::Center { c, winding, target } => ProgramArcData::Center {
            c: n.point(c)?,
            winding,
            target: program_target(target, n)?,
        },
        ArcSpec::Sweep { r, side, angle: a } => ProgramArcData::Sweep {
            r: n.length(r)?,
            side,
            angle: n.angle(a)?,
        },
        ArcSpec::ArcLen { r, side, len } => ProgramArcData::ArcLen {
            r: n.length(r)?,
            side,
            len: n.length(len)?,
        },
    })
}

/// Lower one authored verb to its recorded step.
///
/// **Exhaustive on [`PathStep`], and that is the point**: the step
/// vocabulary is a mirror of `ProgramStep`, so a verb the document
/// layer gains breaks this function rather than being silently
/// unauthorable from the chrome.
///
/// # Errors
///
/// A non-finite field — the literal constructors' one refusal.
/// Nothing about the WALK is judged here (see [`PathStep`]).
fn program_step(step: &PathStep, n: Notation) -> Result<ProgramStep, DimensionError> {
    Ok(match *step {
        PathStep::At(p) => ProgramStep::At(n.point(p)?),
        PathStep::Angle(a) => ProgramStep::Angle(n.angle(a)?),
        PathStep::Toward { dx, dy } => ProgramStep::Toward {
            dx: n.scalar(dx)?,
            dy: n.scalar(dy)?,
        },
        PathStep::Tangent => ProgramStep::Tangent,
        PathStep::Cusp => ProgramStep::Cusp,
        PathStep::Turn(d) => ProgramStep::Turn(n.angle(d)?),
        PathStep::Line(len) => ProgramStep::Line(n.length(len)?),
        PathStep::LineTo(target) => ProgramStep::LineTo(program_target(target, n)?),
        PathStep::ArcTo(spec) => ProgramStep::ArcTo(program_arc(spec, n)?),
        PathStep::TangentArcTo(target) => ProgramStep::TangentArcTo(program_target(target, n)?),
        PathStep::ArcContinue(p) => ProgramStep::ArcContinue(n.point(p)?),
        PathStep::Fillet(r) => ProgramStep::Fillet(n.length(r)?),
        PathStep::FilletArc { radius, spec } => ProgramStep::FilletArc {
            radius: n.length(radius)?,
            spec: program_arc(spec, n)?,
        },
        PathStep::ArcFillet { spec, radius } => ProgramStep::ArcFillet {
            spec: program_arc(spec, n)?,
            radius: n.length(radius)?,
        },
        PathStep::ArcFilletArc {
            spec,
            radius,
            spec2,
        } => ProgramStep::ArcFilletArc {
            spec: program_arc(spec, n)?,
            radius: n.length(radius)?,
            spec2: program_arc(spec2, n)?,
        },
        PathStep::FarEndTo(p) => ProgramStep::FarEndTo(n.point(p)?),
        PathStep::CloseTo => ProgramStep::CloseTo,
    })
}

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
/// waiting to disagree with the first. (The kernel's own f64 read is a
/// different question — see `wire::profile_plane_f64` — and is about
/// structure selection, not about drawing.)
pub fn frame_placement(
    doc: &Doc<ProfileProgram>,
    evaluation: &Evaluation<f64>,
    frame: RecipeNodeId,
) -> Option<SketchPlane<f64>> {
    if !matches!(doc.node(frame), Some(Node::Datum(Datum::Frame { .. }))) {
        return None;
    }
    let ValuePayload::Datum(DatumValue::Frame { origin, u, v }) = &evaluation.value(frame)?.payload
    else {
        return None;
    };
    Some(SketchPlane::from_frame(*origin, u.get(), v.get()))
}

/// **Every frame datum in the document, in document order** — what the
/// add-profile form's plane picker offers.
///
/// Document order rather than sorted by id or by name: the feature
/// tree lists nodes that way, so the picker and the tree name the
/// document's frames in one order.
pub fn frames(doc: &Doc<ProfileProgram>) -> Vec<RecipeNodeId> {
    doc.order()
        .iter()
        .copied()
        .filter(|id| matches!(doc.node(*id), Some(Node::Datum(Datum::Frame { .. }))))
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
/// geometry behind them — a field that is not a number, an expression
/// that will not resolve, a walk the lattice does not admit, a leg
/// whose geometry has no answer.
#[derive(Clone, Debug, PartialEq)]
pub enum PreviewError {
    /// A field is not a finite number.
    Dimension(DimensionError),
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

impl core::fmt::Display for PreviewError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Dimension(error) => write!(f, "{error}"),
            Self::Resolve { slot, source } => write!(f, "{}: {source}", slot.label()),
            Self::Transition {
                loop_,
                step,
                state,
                verb,
            } => match verb {
                Some(verb) => write!(
                    f,
                    "loop {loop_} step {step}: {verb:?} is not well-typed there — the tip is {}",
                    tip_state_words(*state)
                ),
                None => write!(
                    f,
                    "loop {loop_} never closes — it ends with the tip {}, and the last verb \
                     has to target the start",
                    tip_state_words(*state)
                ),
            },
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
/// belonging to the appended step.
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
        programs.push(loop_program(shape, Notation::CANONICAL).map_err(PreviewError::Dimension)?);
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
        .map(|(lp, closed)| {
            let (points, vertices) = flatten(lp, chord);
            PreviewLoop {
                points,
                vertices,
                closed: *closed,
            }
        })
        .collect();
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

/// **Is the step at `steps[at]` well-typed where the chain leaves
/// it?** — asked OF THE LATTICE, by replaying the prefix through it.
///
/// `Err` carries the tip's state and the verb the lattice refused
/// there, which is the sentence a form greys a choice out with.
/// Everything else is `Ok`: a chain that closes, one that simply has
/// not closed yet, a leg whose NUMBERS have no answer (the fields of a
/// freshly offered step are placeholders, and refusing a verb because
/// its default radius is wrong would be judging the wrong thing), and
/// a prefix that is already ill-typed before `at` — that refusal is
/// the prefix's own and the form is already showing it.
///
/// **This is not a table.** The transition lattice is `profile`'s and
/// stays there; what this does is put a candidate step in front of the
/// same `replay` the commit door runs and report what it said. A
/// second copy of the lattice kept in step by hand is exactly what the
/// step list's docs refuse, and this is how the form offers only legal
/// verbs without becoming one.
///
/// # Errors
///
/// The tip's state and the refused verb, when the lattice refuses the
/// step at `at` for being ill-typed there.
pub fn admits_at(
    steps: &[PathStep],
    at: usize,
    notation: Notation,
    tol: Tol,
) -> Result<(), (TipState, Verb)> {
    let mut programs = Vec::with_capacity(steps.len());
    for step in steps {
        // A field that is not a number is not a lattice question, and
        // the form reports it through the preview beside this.
        let Ok(lowered) = program_step(step, notation) else {
            return Ok(());
        };
        programs.push(lowered);
    }
    let loops = [LoopProgram::Chain(programs)];
    let resolved: Vec<Vec<Step<f64>>> = match resolve_loops(&loops, &ParamEnv::default()) {
        Ok(resolved) => resolved,
        // An expression that will not resolve is likewise not a
        // lattice question.
        Err(_) => return Ok(()),
    };
    let Some(chain) = resolved.first() else {
        return Ok(());
    };
    match replay(chain, tol) {
        Err(ReplayError {
            step,
            kind:
                ReplayErrorKind::Transition {
                    state,
                    verb: Some(verb),
                },
        }) if step == at => Err((state, verb)),
        _ => Ok(()),
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

/// One loop as a closed polyline: every vertex, with each bulged
/// segment subdivided finely enough that it sags less than `chord`.
///
/// The bulge convention is [`ProfileVertex`](pncad::profile::ProfileVertex)'s
/// — `b = tan(θ/4)` for the segment LEAVING each vertex, positive
/// counterclockwise, the last vertex's belonging to the closing
/// segment — so this reads the loop exactly as the kernel writes it
/// and invents no second convention.
fn flatten(loop_: &ProfileLoop<f64>, chord: f64) -> (Vec<[f64; 2]>, Vec<usize>) {
    let vertices = loop_.vertices();
    let mut out: Vec<[f64; 2]> = Vec::with_capacity(vertices.len());
    // Where each real vertex landed among the subdivisions. A caller
    // that wants to mark the loop's own points cannot recover this
    // afterwards — an arc's interior points are geometrically
    // indistinguishable from its ends — so the flattener, which is the
    // one place that knows, says it.
    let mut at: Vec<usize> = Vec::with_capacity(vertices.len());
    for (index, vertex) in vertices.iter().enumerate() {
        let from = vertex.pos();
        let to = vertices[(index + 1) % vertices.len()].pos();
        at.push(out.len());
        out.push([from.x, from.y]);
        let bulge = vertex.bulge();
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
        for point in 1..arc_points(radius, theta, chord) {
            let angle = start + theta * (point as f64) / (arc_points(radius, theta, chord) as f64);
            out.push([
                centre[0] + radius * angle.cos(),
                centre[1] + radius * angle.sin(),
            ]);
        }
    }
    (out, at)
}

/// How many chords one arc of `radius` sweeping `theta` needs to sag
/// less than `chord`.
///
/// The sagitta of a sub-arc of angle φ is `r(1 - cos(φ/2))`, so the
/// admissible φ inverts that; a `chord` at or past the diameter asks
/// for no subdivision at all and gets the one-segment floor.
fn arc_points(radius: f64, theta: f64, chord: f64) -> usize {
    if chord <= 0.0 || radius <= 0.0 {
        return MAX_ARC_POINTS;
    }
    let ratio = 1.0 - chord / radius;
    if ratio <= -1.0 {
        return 1;
    }
    let step = 2.0 * ratio.clamp(-1.0, 1.0).acos();
    if step <= 0.0 {
        return MAX_ARC_POINTS;
    }
    ((theta.abs() / step).ceil() as usize).clamp(1, MAX_ARC_POINTS)
}
