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
    Dimension, DimensionError, Expr, LoopProgram, ParamEnv, ProfileProgram, ProgramArcData,
    ProgramStep, ProgramTarget, SlotId,
};
use pncad::geom_core::Tol;
use pncad::profile::{
    ArcSide, ArcSweep, Profile, ProfileError, ProfileLoop, ReplayErrorKind, SketchPlane, TipState,
    Verb, replay,
};

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
    /// A circle ([`LoopProgram::circle`]).
    Circle {
        /// The centre, in sketch coordinates (metres).
        centre: [f64; 2],
        /// The radius, metres.
        radius: f64,
    },
    /// An axis-aligned rectangle centred on the sketch origin
    /// ([`LoopProgram::polygon`], corners at `(±w/2, ±h/2)`).
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

/// Lower one template shape to its loop program, through the
/// program's own literal constructors.
///
/// # Errors
///
/// A non-finite field (the constructors' refusal). Degeneracy — a
/// zero radius, a zero width — is NOT judged here: the edit door's
/// authoring-time check replays the program and refuses it typed,
/// which is one rule for authored and hand-written programs alike.
pub fn loop_program(shape: &ProfileShape) -> Result<LoopProgram, DimensionError> {
    match shape {
        ProfileShape::Circle { centre, radius } => {
            LoopProgram::circle(centre[0], centre[1], *radius)
        }
        ProfileShape::Rectangle { width, height } => {
            let (hw, hh) = (width / 2.0, height / 2.0);
            // Counter-clockwise from the lower-left corner — the same
            // winding every literal outer loop in this workspace uses.
            LoopProgram::polygon([(-hw, -hh), (hw, -hh), (hw, hh), (-hw, hh)])
        }
        ProfileShape::Path { steps } => Ok(LoopProgram::Chain(
            steps
                .iter()
                .map(program_step)
                .collect::<Result<Vec<_>, _>>()?,
        )),
    }
}

/// A Length literal, metres — the profile forms' one length door.
fn length(v: f64) -> Result<Expr, DimensionError> {
    Expr::literal(v, Dimension::Length)
}

/// An Angle literal, radians.
fn angle(v: f64) -> Result<Expr, DimensionError> {
    Expr::literal(v, Dimension::Angle)
}

/// A dimensionless literal — a bulge, a director component.
fn scalar(v: f64) -> Result<Expr, DimensionError> {
    Expr::literal(v, Dimension::Scalar)
}

/// A literal point.
fn point(p: [f64; 2]) -> Result<[Expr; 2], DimensionError> {
    Ok([length(p[0])?, length(p[1])?])
}

/// Lower one authored target.
fn program_target(target: PathTarget) -> Result<ProgramTarget, DimensionError> {
    Ok(match target {
        PathTarget::Point(p) => ProgramTarget::Point(point(p)?),
        PathTarget::Start => ProgramTarget::Start,
    })
}

/// Lower one authored arc spec — the dimension of every field is the
/// vocabulary's ([`SlotId::dimension`]'s table: radii and coordinates
/// Length, the swept angle Angle, the bulge Scalar), so a form cannot
/// author a radius that is secretly an angle.
fn program_arc(spec: ArcSpec) -> Result<ProgramArcData, DimensionError> {
    Ok(match spec {
        ArcSpec::Radius { r, side } => ProgramArcData::Radius {
            r: length(r)?,
            side,
        },
        ArcSpec::Bulge { target, b } => ProgramArcData::Bulge {
            target: program_target(target)?,
            b: scalar(b)?,
        },
        ArcSpec::Via { q, target } => ProgramArcData::Via {
            q: point(q)?,
            target: program_target(target)?,
        },
        ArcSpec::Center { c, winding, target } => ProgramArcData::Center {
            c: point(c)?,
            winding,
            target: program_target(target)?,
        },
        ArcSpec::Sweep { r, side, angle: a } => ProgramArcData::Sweep {
            r: length(r)?,
            side,
            angle: angle(a)?,
        },
        ArcSpec::ArcLen { r, side, len } => ProgramArcData::ArcLen {
            r: length(r)?,
            side,
            len: length(len)?,
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
fn program_step(step: &PathStep) -> Result<ProgramStep, DimensionError> {
    Ok(match *step {
        PathStep::At(p) => ProgramStep::At(point(p)?),
        PathStep::Angle(a) => ProgramStep::Angle(angle(a)?),
        PathStep::Toward { dx, dy } => ProgramStep::Toward {
            dx: scalar(dx)?,
            dy: scalar(dy)?,
        },
        PathStep::Tangent => ProgramStep::Tangent,
        PathStep::Cusp => ProgramStep::Cusp,
        PathStep::Turn(d) => ProgramStep::Turn(angle(d)?),
        PathStep::Line(len) => ProgramStep::Line(length(len)?),
        PathStep::LineTo(target) => ProgramStep::LineTo(program_target(target)?),
        PathStep::ArcTo(spec) => ProgramStep::ArcTo(program_arc(spec)?),
        PathStep::TangentArcTo(target) => ProgramStep::TangentArcTo(program_target(target)?),
        PathStep::ArcContinue(p) => ProgramStep::ArcContinue(point(p)?),
        PathStep::Fillet(r) => ProgramStep::Fillet(length(r)?),
        PathStep::FilletArc { radius, spec } => ProgramStep::FilletArc {
            radius: length(radius)?,
            spec: program_arc(spec)?,
        },
        PathStep::ArcFillet { spec, radius } => ProgramStep::ArcFillet {
            spec: program_arc(spec)?,
            radius: length(radius)?,
        },
        PathStep::ArcFilletArc {
            spec,
            radius,
            spec2,
        } => ProgramStep::ArcFilletArc {
            spec: program_arc(spec)?,
            radius: length(radius)?,
            spec2: program_arc(spec2)?,
        },
        PathStep::FarEndTo(p) => ProgramStep::FarEndTo(point(p)?),
        PathStep::CloseTo => ProgramStep::CloseTo,
    })
}

// ------------------------------------------------------------------
// The preview: what the loops being authored would actually draw
// ------------------------------------------------------------------

/// **The plane the add-profile form authors on.**
///
/// The world XY plane, and one home for that fact: the form's commit
/// and the form's preview have to place the loops the same way, and
/// two `SketchPlane::xy()` calls are two places for that to stop
/// being true.
///
/// It is fixed because placement on a picked face's frame is deferred
/// — the interrogation vocabulary answers no "is this face planar"
/// verdict for the door to gate on (issue #1374). A form that offered
/// a plane it could not check would be offering a refusal.
pub fn form_plane() -> SketchPlane<f64> {
    SketchPlane::xy()
}

/// **A candidate profile, replayed and flattened** — the picture a
/// form shows of the loops in front of it, before any of it is a
/// document.
///
/// Sketch-plane coordinates in metres, one CLOSED polyline per loop:
/// the last point is joined back to the first by the consumer, which
/// is the same thing [`ProfileLoop`] means by being closed by
/// construction. Nothing here knows where the sketch plane is; the
/// caller places the points ([`SketchPlane::to_world`]) because the
/// caller is the one drawing them.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ProfilePreview {
    /// One closed polyline per loop, in authoring order.
    pub loops: Vec<Vec<[f64; 2]>>,
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
    pub invalid: Option<ProfileError>,
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
                    "loop {loop_} step {step}: {verb:?} is not well-typed at a {state:?} tip"
                ),
                None => write!(
                    f,
                    "loop {loop_} ends at a {state:?} tip without closing — the last verb has \
                     to target the start"
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
/// # Errors
///
/// [`PreviewError`], per arm — everything that leaves no geometry to
/// draw. A profile that replays and fails VALIDATION is a success
/// here, carrying its refusal in [`ProfilePreview::invalid`].
pub fn preview(
    plane: SketchPlane<f64>,
    shapes: &[ProfileShape],
    tol: Tol,
    chord: f64,
) -> Result<ProfilePreview, PreviewError> {
    let mut programs = Vec::with_capacity(shapes.len());
    for shape in shapes {
        programs.push(loop_program(shape).map_err(PreviewError::Dimension)?);
    }
    let program = ProfileProgram {
        plane,
        loops: programs,
    };
    // Literals only reach this door, so an empty environment binds
    // everything it can be asked about. It is passed rather than
    // assumed because `resolve` is the document layer's one door and
    // a form is not a special case of it.
    let env = ParamEnv::default();
    let resolved = program
        .resolve(&env)
        .map_err(|(slot, source)| PreviewError::Resolve { slot, source })?;
    let mut loops: Vec<ProfileLoop<f64>> = Vec::with_capacity(resolved.len());
    for (index, steps) in resolved.iter().enumerate() {
        let replayed = replay(steps, tol).map_err(|error| match error.kind {
            ReplayErrorKind::Transition { state, verb } => PreviewError::Transition {
                loop_: index,
                step: error.step,
                state,
                verb,
            },
            ReplayErrorKind::Path(ref source) => PreviewError::Geometry {
                loop_: index,
                step: error.step,
                rendered: source.to_string(),
            },
        })?;
        loops.push(replayed);
    }
    let polylines = loops.iter().map(|lp| flatten(lp, chord)).collect();
    let invalid = Profile::new(plane, loops).validate(tol).err();
    Ok(ProfilePreview {
        loops: polylines,
        invalid,
    })
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
fn flatten(loop_: &ProfileLoop<f64>, chord: f64) -> Vec<[f64; 2]> {
    let vertices = loop_.vertices();
    let mut out: Vec<[f64; 2]> = Vec::with_capacity(vertices.len());
    for (index, vertex) in vertices.iter().enumerate() {
        let from = vertex.pos();
        let to = vertices[(index + 1) % vertices.len()].pos();
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
        let half = (dx * dx + dy * dy).sqrt() / 2.0;
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
            let at = start + theta * (point as f64) / (arc_points(radius, theta, chord) as f64);
            out.push([centre[0] + radius * at.cos(), centre[1] + radius * at.sin()]);
        }
    }
    out
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
