//! **How an author reaches a step's id** (`names/README.md`, "N1, the
//! profile pieces", "The id."): an authored address, bound per
//! placement.
//!
//! A loop is a free value until a profile holds it, and each placement
//! mints its own ids, so what an authoring call can hand back is not an
//! id but an address: the step's index in its loop and the loop's
//! shape up to and including that step, values erased
//! ([`AuthoredStep`]). A profile's program maps the address to the id it
//! minted, against the loop the author states
//! ([`ProfileProgram::step`], [`ProfileProgram::piece`]), and refuses an
//! address whose prefix its program does not have
//! ([`StepHandleRefusal::OffProgram`]). So a value edit leaves an address
//! valid, one loop placed twice resolves to two ids, and a reshape that
//! changes the prefix refuses it.

use profile::{FormLowered, FormTarget, PieceRole, RoleList, Step, Verb};

use crate::names::ProfileEdgeRef;
use crate::node::StepId;
use crate::program::{
    LoopProgram, ProfileProgram, ProgramArcData, ProgramStep, ProgramTarget, StepIdFault,
    program_index,
};

/// Where a target-taking step ends, value erased: the kernel's target
/// forms, and the ones the pen finds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TargetShape {
    /// An authored point.
    Point,
    /// The entry vertex.
    Start,
    /// The entry vertex, with the seam's tangent joint declared.
    StartArriving,
    /// A crossing of a face's trace.
    OnTrace,
    /// The point along the heading at a written `x`.
    AtX,
    /// The point along the heading at a written `y`.
    AtY,
}

impl TargetShape {
    /// Whether the pen finds this target: a trace's crossing or a
    /// written coordinate.
    #[must_use]
    pub fn found_from_the_pen(self) -> bool {
        matches!(self, Self::OnTrace | Self::AtX | Self::AtY)
    }
}

impl FormTarget for TargetShape {
    /// A target found from the pen lowers to the point it finds.
    fn kernel(self) -> profile::TargetKind {
        match self {
            Self::Point | Self::OnTrace | Self::AtX | Self::AtY => profile::TargetKind::Point,
            Self::Start => profile::TargetKind::Start,
            Self::StartArriving => profile::TargetKind::StartArriving,
        }
    }
}

impl From<profile::TargetKind> for TargetShape {
    fn from(kind: profile::TargetKind) -> Self {
        match kind {
            profile::TargetKind::Point => Self::Point,
            profile::TargetKind::Start => Self::Start,
            profile::TargetKind::StartArriving => Self::StartArriving,
        }
    }
}

/// **A program step the kernel has no verb for**, value erased
/// ([`profile::StepForm::Lowered`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PenStep {
    /// The heading along a face's trace: the `toward` director it
    /// lowers to.
    AlongTrace,
}

impl FormLowered for PenStep {
    fn verb(self) -> Verb {
        match self {
            Self::AlongTrace => Verb::Toward,
        }
    }
}

/// An arc spec's mode and structural tags, values erased: the kernel's
/// one form ([`profile::ArcForm`]) over the editor's targets.
pub type ArcShape = profile::ArcForm<TargetShape>;

/// **One authored step, values erased**: its verb and the structural
/// tags it carries (targets, arc modes, sides, senses, a split's
/// count). It is the kernel's one value-free form
/// ([`profile::StepForm`]) over the editor's targets and its traced
/// heading, so the address a step is bound by and the walk the door
/// asks ([`step_kernel`]) read the same thing.
pub type StepShape = profile::StepForm<TargetShape, PenStep>;

/// A recorded step, values erased.
#[must_use]
pub fn shape_of_recorded<T: geom_core::Real>(step: &Step<T>) -> StepShape {
    step.form().lower(TargetShape::from, |never| match never {})
}

/// A program step, values erased.
pub(crate) fn shape_of_program<S>(step: &ProgramStep<S>) -> StepShape {
    use profile::StepForm as F;
    match step {
        ProgramStep::At(_) => F::At,
        ProgramStep::Angle(_) => F::Angle,
        ProgramStep::Toward { .. } => F::Toward,
        ProgramStep::Tangent => F::Tangent,
        ProgramStep::Cusp => F::Cusp,
        ProgramStep::Turn(_) => F::Turn,
        ProgramStep::Line(_) => F::Line,
        ProgramStep::LineTo(t) => F::LineTo(target_of_program(t)),
        ProgramStep::ContinueTo(t) => F::ContinueTo(target_of_program(t)),
        ProgramStep::ArcTo(spec) => F::ArcTo(arc_of_program(spec)),
        ProgramStep::TangentArcTo(t) => F::TangentArcTo(target_of_program(t)),
        ProgramStep::ArcOnto(t) => F::ArcOnto(target_of_program(t)),
        ProgramStep::Fillet(_) => F::Fillet,
        ProgramStep::FilletArc { spec, .. } => F::FilletArc(arc_of_program(spec)),
        ProgramStep::ArcFillet { spec, .. } => F::ArcFillet(arc_of_program(spec)),
        ProgramStep::ArcFilletArc { spec, spec2, .. } => {
            F::ArcFilletArc(arc_of_program(spec), arc_of_program(spec2))
        }
        ProgramStep::FarEndTo(_) => F::FarEndTo,
        ProgramStep::CloseTo => F::CloseTo,
        ProgramStep::AlongTrace { .. } => F::Lowered(PenStep::AlongTrace),
    }
}

/// **How a step found from the pen lowers** ([`pen_run`]): the one map
/// from a pen step's shape to what `crate::trace` lowers it to, read by
/// that lowering, by the door's lattice ([`ProfileProgram::lattice`])
/// and by an editor holding steps back ([`crate::trace::pen_order`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PenRun {
    /// `along_trace`: a `toward` along the trace the step before ran
    /// onto.
    Heading,
    /// `line_to` (`wants_director`) or `continue_to` to a trace or a
    /// coordinate: a `line` along the pen's heading, ending on the
    /// trace (`onto_trace`) or at the coordinate.
    Leg {
        /// Whether a director binds the heading (`line_to`), not the
        /// incoming tangent (`continue_to`).
        wants_director: bool,
        /// Whether it ends on a trace's crossing.
        onto_trace: bool,
    },
    /// `arc_to` about a centre onto a trace's crossing: the centre arc
    /// to the point it crosses at.
    Arc,
    /// Any other step that reads a trace or a coordinate: not a run
    /// (`TraceStepFault::NotARun`).
    NotARun,
}

impl PenRun {
    /// Whether the run ends on a trace, which an `along_trace` after it
    /// reads.
    #[must_use]
    pub fn reaches_a_trace(self) -> bool {
        matches!(
            self,
            Self::Leg {
                onto_trace: true,
                ..
            } | Self::Arc
        )
    }
}

/// **How `shape` lowers from the pen**, where it reads a trace or a
/// written coordinate; `None` for every other step.
#[must_use]
pub fn pen_run(shape: StepShape) -> Option<PenRun> {
    use profile::StepForm as F;
    let pen = TargetShape::found_from_the_pen;
    let arc_pen = |a: ArcShape| a.target().is_some_and(pen);
    Some(match shape {
        F::Lowered(PenStep::AlongTrace) => PenRun::Heading,
        F::LineTo(t) if pen(t) => PenRun::Leg {
            wants_director: true,
            onto_trace: t == TargetShape::OnTrace,
        },
        F::ContinueTo(t) if pen(t) => PenRun::Leg {
            wants_director: false,
            onto_trace: t == TargetShape::OnTrace,
        },
        F::ArcTo(profile::ArcForm::Center(_, TargetShape::OnTrace)) => PenRun::Arc,
        F::TangentArcTo(t) | F::ArcOnto(t) if pen(t) => PenRun::NotARun,
        F::ArcTo(a) | F::FilletArc(a) | F::ArcFillet(a) if arc_pen(a) => PenRun::NotARun,
        F::ArcFilletArc(a, b) if arc_pen(a) || arc_pen(b) => PenRun::NotARun,
        _ => return None,
    })
}

/// **`shape` as the kernel walks it**, values erased
/// ([`profile::chain_shape`]): a step found from the pen as the verb
/// its lowering gives it ([`pen_run`]), every other step as written,
/// each target as the kernel target it lowers to.
#[must_use]
pub fn step_kernel(shape: StepShape) -> profile::StepForm {
    use profile::StepForm as F;
    match pen_run(shape) {
        Some(PenRun::Heading) => F::Toward,
        Some(PenRun::Leg { .. }) => F::Line,
        _ => shape.lower(FormTarget::kernel, |x| match x {
            PenStep::AlongTrace => F::Toward,
        }),
    }
}

fn target_of_program<S>(t: &ProgramTarget<S>) -> TargetShape {
    match t {
        ProgramTarget::Point(_) => TargetShape::Point,
        ProgramTarget::Start => TargetShape::Start,
        ProgramTarget::StartArriving => TargetShape::StartArriving,
        ProgramTarget::OnTrace { .. } => TargetShape::OnTrace,
        ProgramTarget::AtX(_) => TargetShape::AtX,
        ProgramTarget::AtY(_) => TargetShape::AtY,
    }
}

fn arc_of_program<S>(spec: &ProgramArcData<S>) -> ArcShape {
    use profile::ArcForm as A;
    match spec {
        ProgramArcData::Radius { side, .. } => A::Radius(*side),
        ProgramArcData::Bulge { target, .. } => A::Bulge(target_of_program(target)),
        ProgramArcData::Via { target, .. } => A::Via(target_of_program(target)),
        ProgramArcData::Center {
            winding, target, ..
        } => A::Center(*winding, target_of_program(target)),
        ProgramArcData::Sweep { side, .. } => A::Sweep(*side),
        ProgramArcData::ArcLen { side, .. } => A::ArcLen(*side),
    }
}

impl<S> LoopProgram<S> {
    /// **This loop's program, values erased**: one [`StepShape`] per
    /// authored step, in program order.
    #[must_use]
    pub fn shape(&self) -> Vec<StepShape> {
        match self {
            LoopProgram::Chain(steps) => steps.iter().map(shape_of_program).collect(),
            LoopProgram::Circle { .. } => vec![StepShape::Circle],
            LoopProgram::CircleSplit { n, .. } => vec![StepShape::CircleSplit { n: *n as usize }],
            LoopProgram::Rectangle { .. } => vec![StepShape::Rectangle],
        }
    }
}

/// **An authored step's address**: its index in its loop, and the
/// loop's shape up to and including it, values erased.
///
/// An authoring call hands one back for the step it records
/// ([`AuthoredStep::after`]); a profile's program binds it to the id it
/// minted for that step ([`ProfileProgram::step`]).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AuthoredStep {
    /// The shape up to and including the step; never empty, so its
    /// last entry is the step and its length one past the index.
    prefix: Vec<StepShape>,
}

impl AuthoredStep {
    /// The LAST step of `recorded`: the one an authoring call just
    /// recorded, as `profile::PartialPath::recorded` and a closed
    /// loop's `program` answer it. `None` for an empty recording.
    #[must_use]
    pub fn after<T: geom_core::Real>(recorded: &[Step<T>]) -> Option<Self> {
        if recorded.is_empty() {
            return None;
        }
        Some(Self {
            prefix: recorded.iter().map(shape_of_recorded).collect(),
        })
    }

    /// Step `index` of `program`, or `None` past its end.
    #[must_use]
    pub fn of_program<S>(program: &LoopProgram<S>, index: usize) -> Option<Self> {
        let mut prefix = program.shape();
        if index >= prefix.len() {
            return None;
        }
        prefix.truncate(index + 1);
        Some(Self { prefix })
    }

    /// The step's index in its loop.
    #[must_use]
    pub fn index(&self) -> u32 {
        program_index(self.prefix.len() - 1)
    }

    /// The step itself, values erased.
    #[must_use]
    pub fn step(&self) -> StepShape {
        let Some(&last) = self.prefix.last() else {
            unreachable!("an authored step's prefix holds the step itself")
        };
        last
    }

    /// The loop's shape up to and including the step.
    #[must_use]
    pub fn prefix(&self) -> &[StepShape] {
        &self.prefix
    }

    /// Whether this is an address in `program`: its prefix is
    /// `program`'s own.
    #[must_use]
    pub fn is_in<S>(&self, program: &LoopProgram<S>) -> bool {
        let shape = program.shape();
        shape.get(..self.prefix.len()) == Some(self.prefix.as_slice())
    }
}

/// **Why an authored step does not bind** in a profile's program.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StepHandleRefusal {
    /// The stated loop has no step at this address: it is not a loop
    /// of the program, or its program's shape up to the step is not the
    /// address's.
    OffProgram {
        /// The loop the author stated.
        loop_: u32,
        /// The address's index.
        index: u32,
    },
    /// The program carries no minted ids: it has not entered a
    /// document.
    Unminted,
    /// The program's ids are not shaped like it.
    StepIds(StepIdFault),
    /// The step's verb never draws this role.
    RoleNotDrawn {
        /// The step's verb.
        verb: Verb,
        /// The role asked for.
        role: PieceRole,
    },
}

impl core::fmt::Display for StepHandleRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::OffProgram { loop_, index } => write!(
                f,
                "loop {loop_} has no step {index} of the shape this handle was authored with; \
                 a handle is valid for the program it was authored for, and across a \
                 program change the step is held by its id"
            ),
            Self::Unminted => f.write_str(
                "this program carries no step ids, because it is not in a document; \
                 bind the handle against the profile the document holds",
            ),
            Self::StepIds(fault) => write!(f, "this program's step ids are malformed: {fault}"),
            Self::RoleNotDrawn { verb, role } => {
                let role = role.to_string();
                write!(
                    f,
                    "a `{verb}` step never draws {} {role}; its roles are {}",
                    crate::sentence::article(&role),
                    RoleWords(RoleList::of(*verb))
                )
            }
        }
    }
}

impl core::error::Error for StepHandleRefusal {}

/// A role list as prose.
struct RoleWords(RoleList);

impl core::fmt::Display for RoleWords {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            RoleList::Bind => f.write_str("none: it binds and draws nothing"),
            RoleList::Carrier => f.write_str("piece k, for each k below its count"),
            list => {
                let words: Vec<String> = list.named().iter().map(ToString::to_string).collect();
                f.write_str(&words.join(", "))
            }
        }
    }
}

impl ProfileProgram {
    /// **The id this program minted for the step `h` addresses** in
    /// loop `loop_`, which the author states.
    ///
    /// # Errors
    ///
    /// [`StepHandleRefusal::OffProgram`] where the loop is not this
    /// program's or its shape up to the step is not `h`'s;
    /// [`StepHandleRefusal::Unminted`] and
    /// [`StepHandleRefusal::StepIds`] for a program with no ids, or ids
    /// not shaped like it.
    pub fn step(&self, loop_: u32, h: &AuthoredStep) -> Result<StepId, StepHandleRefusal> {
        let off = || StepHandleRefusal::OffProgram {
            loop_,
            index: h.index(),
        };
        let lp = self.loops.get(loop_ as usize).ok_or_else(off)?;
        if !h.is_in(lp) {
            return Err(off());
        }
        if !self.carries_step_ids() {
            return Err(StepHandleRefusal::Unminted);
        }
        self.check_id_shape().map_err(StepHandleRefusal::StepIds)?;
        let Some(&id) = self
            .ids
            .get(loop_ as usize)
            .and_then(|ids| ids.get(h.index() as usize))
        else {
            unreachable!("ids shaped like the program hold one per authored step")
        };
        Ok(id)
    }

    /// **The piece `role` of the step `h` addresses** in loop `loop_`,
    /// which the author states.
    ///
    /// A role the step's verb draws but whose current values do not
    /// draw it is still a piece: its names resolve `Vanished` (N1).
    ///
    /// # Errors
    ///
    /// Everything [`ProfileProgram::step`] refuses, and
    /// [`StepHandleRefusal::RoleNotDrawn`] for a role the step's verb
    /// never draws.
    pub fn piece(
        &self,
        loop_: u32,
        h: &AuthoredStep,
        role: PieceRole,
    ) -> Result<ProfileEdgeRef, StepHandleRefusal> {
        let step = self.step(loop_, h)?;
        if !h.step().admits(role) {
            return Err(StepHandleRefusal::RoleNotDrawn {
                verb: h.step().verb(),
                role,
            });
        }
        Ok(ProfileEdgeRef::Piece { step, role })
    }
}

/// **A `SetProgram`'s id grid from a keep map**: per new loop, in
/// program order, the old ids its steps keep, each keyed by the step's
/// address in that loop's NEW program. A step no entry addresses gets
/// `None`, which the door mints.
///
/// The grid is what `DocEdit::SetProgram` stores; this lowers to it.
/// An old id listed twice lowers to a grid the door refuses
/// ([`StepIdFault::Repeated`]), and a keep map with a different number
/// of loops to one it refuses ([`StepIdFault::LoopCount`]).
///
/// # Errors
///
/// [`StepHandleRefusal::OffProgram`] for an address that is not a step
/// of its loop's new program.
pub fn keep_grid<S>(
    loops: &[LoopProgram<S>],
    keep: &[Vec<(AuthoredStep, StepId)>],
) -> Result<Vec<Vec<Option<StepId>>>, StepHandleRefusal> {
    keep.iter()
        .enumerate()
        .map(|(li, kept)| {
            let loop_ = program_index(li);
            let lp = loops.get(li);
            let mut row = vec![None; lp.map_or(0, LoopProgram::authored_steps)];
            for (h, id) in kept {
                if !lp.is_some_and(|lp| h.is_in(lp)) {
                    return Err(StepHandleRefusal::OffProgram {
                        loop_,
                        index: h.index(),
                    });
                }
                row[h.index() as usize] = Some(*id);
            }
            Ok(row)
        })
        .collect()
}
