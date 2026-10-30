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
//! What a binding hands this module is words and formulas: a crossing's
//! passage and a heading's sense as words ([`passage`], [`sense`]), and
//! each argument as the formula it was written. [`drawn_profile`]
//! checks each argument against its slot's dimension, at its loop and
//! step, as the insert door would.
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
    AuthoredNode, DrawnOn, EditError, Evaluation, Formula, LoopProgram, Node, NodeErrorKind,
    Operand, ProfileDoc, ProfileProgram, ProgramArcData, ProgramStep, ProgramTarget, SlotId,
    SlotPayload, TracePassage, TraceSense, TraceStepFault,
};
use geom_core::{Point2, Tol};

pub use editor_core::{TraceMark, TraceMarks};

/// **A word that names no passage or sense** ([`passage`], [`sense`]):
/// the word, and the words that would.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraceWordError {
    /// The word written.
    pub word: String,
    /// What it was read as.
    pub reads: &'static str,
    /// The words it may be.
    pub words: &'static [&'static str],
}

impl core::fmt::Display for TraceWordError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} is ", self.reads)?;
        for (i, word) in self.words.iter().enumerate() {
            if i > 0 {
                f.write_str(" or ")?;
            }
            write!(f, "{word:?}")?;
        }
        write!(f, ", not {:?}", self.word)
    }
}

impl core::error::Error for TraceWordError {}

/// **The passage `word` names**: `"enter"` or `"leave"` the face's
/// material.
///
/// # Errors
///
/// [`TraceWordError`] for any other word.
pub fn passage(word: &str) -> Result<TracePassage, TraceWordError> {
    match word {
        "enter" => Ok(TracePassage::Enter),
        "leave" => Ok(TracePassage::Leave),
        other => Err(TraceWordError {
            word: other.to_owned(),
            reads: "a crossing's passage",
            words: &["enter", "leave"],
        }),
    }
}

/// **The sense `word` names**: a heading `"with"` the trace (the face's
/// material on the left) or `"against"` it.
///
/// # Errors
///
/// [`TraceWordError`] for any other word.
pub fn sense(word: &str) -> Result<TraceSense, TraceWordError> {
    match word {
        "with" => Ok(TraceSense::With),
        "against" => Ok(TraceSense::Against),
        other => Err(TraceWordError {
            word: other.to_owned(),
            reads: "a heading along a trace",
            words: &["with", "against"],
        }),
    }
}

/// **A chain program, written step by step** in the document's
/// vocabulary ([`ProgramStep`]): each method appends the step it names
/// and nothing else. [`drawn_profile`] checks each argument's
/// dimension, and the document's replay checks the chain.
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
///
/// Each argument is checked against its slot's dimension, at its loop
/// and step, as the insert door checks it; a lone zero is the zero of
/// the slot's dimension (VR6).
///
/// # Errors
///
/// [`EditError::SlotDimensionMismatch`] naming the first argument whose
/// dimension is not its slot's.
pub fn drawn_profile(
    frame: Operand,
    faces: Vec<Operand>,
    loops: Vec<Chain>,
) -> Result<AuthoredNode, EditError> {
    let mut program = ProfileProgram {
        loops: loops.into_iter().map(Chain::into_loop).collect(),
        ids: Vec::new(),
        on: Some(DrawnOn { frame, faces }),
    };
    for ((loop_, step, arg), formula) in program.rows_mut() {
        let slot = SlotId::Profile { loop_, step, arg };
        let expected = slot.expr_dimension();
        let found = formula.dim();
        if found == expected {
            continue;
        }
        if !formula.is_zero() {
            return Err(EditError::SlotDimensionMismatch {
                slot,
                expected: slot.kind(),
                found,
            });
        }
        *formula = Formula::zero(expected);
    }
    Ok(Node::Profile(program))
}

/// **Why a face's trace in a frame's plane could not be drawn**
/// ([`sketch_trace`]).
#[derive(Debug)]
pub enum SketchTraceError {
    /// The frame or the face does not lower in the document.
    Lower(Box<EditError>),
    /// The frame or the face does not read against the evaluation, as a
    /// profile drawn on them would refuse.
    Read(Box<NodeErrorKind>),
    /// The trace refuses: the frame's plane misses the face's carrier,
    /// or the carrier has no closed form.
    Trace(TraceStepFault),
}

impl core::fmt::Display for SketchTraceError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Lower(e) => write!(f, "{e}"),
            Self::Read(e) => write!(f, "{e}"),
            Self::Trace(e) => write!(f, "{e}"),
        }
    }
}

impl core::error::Error for SketchTraceError {}

/// **`face`'s trace in `frame`'s `xy`, sampled for display** against
/// `ev`, the evaluation of `doc`: read as a profile drawn on `frame`
/// tracing `face` reads it, and sampled across the square of half side
/// `half` about `centre` in the frame's coordinates by `rays` rays each
/// way ([`editor_core::trace_marks`]). Nothing is written to `doc`.
///
/// # Errors
///
/// [`SketchTraceError`].
pub fn sketch_trace(
    doc: &ProfileDoc,
    ev: &Evaluation<f64>,
    frame: Operand,
    face: Operand,
    window: (Point2<f64>, f64),
    rays: u32,
    tol: Tol,
) -> Result<TraceMarks, SketchTraceError> {
    let mut scratch = doc.clone();
    let probe = Node::Profile(ProfileProgram {
        loops: Vec::new(),
        ids: Vec::new(),
        on: Some(DrawnOn {
            frame,
            faces: vec![face],
        }),
    });
    let Node::Profile(program) = editor_core::edit::lower_node_into(&mut scratch, &probe)
        .map_err(|e| SketchTraceError::Lower(Box::new(e)))?
    else {
        unreachable!("a profile lowers to a profile")
    };
    let faces = editor_core::traced_faces(&scratch, ev, &program, tol)
        .map_err(|e| SketchTraceError::Read(Box::new(e)))?;
    let [traced] = faces.as_slice() else {
        unreachable!("a probe drawn on one face traces one face")
    };
    let band = geom_core::Band::linear(tol)
        .map_err(|e| SketchTraceError::Read(Box::new(NodeErrorKind::Band(e))))?;
    editor_core::trace_marks(traced, window.0, window.1, rays, band)
        .map_err(SketchTraceError::Trace)
}
