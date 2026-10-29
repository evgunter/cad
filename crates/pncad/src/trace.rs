//! **A profile drawn on a frame, tracing faces** (D10, "Constructing a
//! coincidence"): a chain written in the document's own vocabulary,
//! step by step, and the profile node it draws on a frame of a body's
//! space.
//!
//! A free profile is authored through the PATHS lattice
//! ([`crate::profile`]), which records its steps as it lowers them. A
//! step that reads a face's trace cannot be lowered there: the trace is
//! the evaluation's, read off the bodies of the frame's space. So a
//! traced chain is written as the program it is, and the document
//! replays it at every evaluation, against the traces it reads then.
//!
//! ```
//! use pncad::document::{
//!     Dimension, Formula, ProgramTarget, TraceCrossing, TracePassage, TraceSense,
//! };
//! use pncad::trace::Chain;
//!
//! let len = |m: f64| Formula::literal(m, Dimension::Length).expect("a finite length");
//! let scl = |s: f64| Formula::scalar(s).expect("a finite number");
//! // Up from (0, -1/4) until the line leaves the traced face, then
//! // along the trace, to x = 1, and home.
//! let footprint = Chain::at(len(0.0), len(-0.25))
//!     .toward(scl(0.0), scl(1.0))
//!     .line_to(ProgramTarget::OnTrace {
//!         face: 0,
//!         crossing: TraceCrossing::first(TracePassage::Leave),
//!     })
//!     .along_trace(TraceSense::Against)
//!     .line_to(ProgramTarget::AtX(len(1.0)))
//!     .line_to(ProgramTarget::Point([len(1.0), len(-0.25)]))
//!     .line_to(ProgramTarget::Start);
//! assert_eq!(footprint.steps().len(), 7);
//! ```

use editor_core::{
    AuthoredNode, DrawnOn, Formula, LoopProgram, Node, Operand, ProfileProgram, ProgramArcData,
    ProgramStep, ProgramTarget, TraceSense,
};

/// **A chain program, written step by step** in the document's
/// vocabulary ([`ProgramStep`]): each method appends the step it names
/// and nothing else, and the document's replay is what checks the
/// chain.
#[derive(Debug, Clone, PartialEq)]
pub struct Chain {
    steps: Vec<ProgramStep<Formula>>,
}

impl Chain {
    /// A chain starting at `(x, y)`.
    #[must_use]
    pub fn at(x: Formula, y: Formula) -> Self {
        Self {
            steps: vec![ProgramStep::At([x, y])],
        }
    }

    fn then(mut self, step: ProgramStep<Formula>) -> Self {
        self.steps.push(step);
        self
    }

    /// `.angle(θ)`: the departure as an angle from the sketch's `x`.
    #[must_use]
    pub fn angle(self, theta: Formula) -> Self {
        self.then(ProgramStep::Angle(theta))
    }

    /// `.toward(dx, dy)`: the departure as components.
    #[must_use]
    pub fn toward(self, dx: Formula, dy: Formula) -> Self {
        self.then(ProgramStep::Toward { dx, dy })
    }

    /// `line(len)`: a straight leg along the heading.
    #[must_use]
    pub fn line(self, len: Formula) -> Self {
        self.then(ProgramStep::Line(len))
    }

    /// `line_to(target)`: a straight leg to a point, to the start, or,
    /// along a director, to a crossing of a trace or a written
    /// coordinate.
    #[must_use]
    pub fn line_to(self, target: ProgramTarget<Formula>) -> Self {
        self.then(ProgramStep::LineTo(target))
    }

    /// `continue_to(target)`: the straight continuation of the
    /// previous piece, to a point or a crossing.
    #[must_use]
    pub fn continue_to(self, target: ProgramTarget<Formula>) -> Self {
        self.then(ProgramStep::ContinueTo(target))
    }

    /// `arc_to(Center)`: an arc about `centre`, `winding` its sense, to
    /// a point or a crossing.
    #[must_use]
    pub fn arc_about(
        self,
        centre: [Formula; 2],
        winding: crate::profile::ArcSweep,
        target: ProgramTarget<Formula>,
    ) -> Self {
        self.then(ProgramStep::ArcTo(ProgramArcData::Center {
            c: centre,
            winding,
            target,
        }))
    }

    /// The heading along the trace the previous piece ran onto, with or
    /// against it.
    #[must_use]
    pub fn along_trace(self, sense: TraceSense) -> Self {
        self.then(ProgramStep::AlongTrace { sense })
    }

    /// The steps written.
    #[must_use]
    pub fn steps(&self) -> &[ProgramStep<Formula>] {
        &self.steps
    }

    /// The loop program the chain is.
    #[must_use]
    pub fn into_loop(self) -> LoopProgram<Formula> {
        LoopProgram::Chain(self.steps)
    }
}

/// **A profile drawn on `frame`, tracing `faces`**: its loops, outer
/// first, read the traces of `faces` (by place) in the frame's `xy`.
/// The profile, and every sweep of it, is in the frame's space.
#[must_use]
pub fn drawn_profile(frame: Operand, faces: Vec<Operand>, loops: Vec<Chain>) -> AuthoredNode {
    Node::Profile(ProfileProgram {
        loops: loops.into_iter().map(Chain::into_loop).collect(),
        ids: Vec::new(),
        on: Some(DrawnOn { frame, faces }),
    })
}
