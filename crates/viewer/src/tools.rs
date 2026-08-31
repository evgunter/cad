//! **The modal tools, and the rule that only one of them is open.**
//!
//! Every modal tool here holds picks in tool state and commits exactly
//! one `DocEdit` (G1's preview-vs-commit rule; the mate tool set the
//! shape and the creation tools took it). They all consume the SAME
//! selection stream, which is what makes the one-at-a-time rule a rule
//! rather than a preference: with two open, one click fills a seat in
//! each, and the picks a user believes they are making are not the
//! picks the tools hold.
//!
//! So the set of tools is ONE value with one door in. [`Tools::open`]
//! closes whatever was open before it opens the next, by REPLACING the
//! whole value rather than by clearing the other fields one at a time
//! — a tool added to this struct cannot be forgotten by that rule,
//! where a chain of assignments grows a missing line per tool and only
//! fails in the field.
//!
//! The value is renderer-free on purpose: the pick routing, the
//! survival step and the exclusivity are all properties a headless row
//! asserts, and only the widgets that open and read the tools need a
//! window.

use pncad::document::{Doc, Evaluation, ProfileProgram};

use crate::combine::{BooleanTool, CombineToolEvent, PatternTool, SplitTool, TransformTool};
use crate::matetool::{MateTool, MateToolEvent};
use crate::revolvetool::{RevolveTool, RevolveToolEvent};
use crate::session::SessionOp;

/// Which modal tool — the vocabulary the open/close door and every
/// notice are addressed in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ToolKind {
    /// The mate tool (GUI-4): two face picks.
    Mate,
    /// The revolve tool: a profile and an axis.
    Revolve,
    /// The boolean tool: two bodies.
    Boolean,
    /// The split tool: a body and a datum plane.
    Split,
    /// The transform tool: one body.
    Transform,
    /// The pattern tool: a body and (circular only) an axis.
    Pattern,
}

impl ToolKind {
    /// Every kind, for a chrome that offers them and a test that
    /// sweeps them.
    pub const ALL: [Self; 6] = [
        Self::Mate,
        Self::Revolve,
        Self::Boolean,
        Self::Split,
        Self::Transform,
        Self::Pattern,
    ];

    /// The tool's name, for sentences and buttons.
    pub fn label(self) -> &'static str {
        match self {
            Self::Mate => "mate tool",
            Self::Revolve => "revolve tool",
            Self::Boolean => "boolean tool",
            Self::Split => "split tool",
            Self::Transform => "transform tool",
            Self::Pattern => "pattern tool",
        }
    }
}

/// Something a tool did on its own — always a survival drop today —
/// carrying which tool it was about, so the sentence a chrome shows is
/// composed here rather than at each call site.
#[derive(Debug)]
pub enum ToolNotice {
    /// The mate tool lost a pick.
    Mate(MateToolEvent),
    /// The revolve tool lost a pick.
    Revolve(RevolveToolEvent),
    /// A combining tool lost a pick.
    Combine {
        /// Which tool.
        tool: ToolKind,
        /// What it lost.
        event: CombineToolEvent,
    },
}

impl core::fmt::Display for ToolNotice {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Mate(event) => write!(f, "{}: {event}", ToolKind::Mate.label()),
            Self::Revolve(event) => write!(f, "{}: {event}", ToolKind::Revolve.label()),
            Self::Combine { tool, event } => write!(f, "{}: {event}", tool.label()),
        }
    }
}

/// The modal tools as one value: at most one is open, and the open one
/// is the only one the selection stream reaches.
#[derive(Debug, Default)]
pub struct Tools {
    mate: Option<MateTool>,
    revolve: Option<RevolveTool>,
    boolean: Option<BooleanTool>,
    split: Option<SplitTool>,
    transform: Option<TransformTool>,
    pattern: Option<PatternTool>,
}

impl Tools {
    /// No tool open.
    pub fn new() -> Self {
        Self::default()
    }

    /// **Open one tool, closing whatever was open.** Re-opening the
    /// tool that is already open RESTARTS it — the button reads as
    /// "begin this tool", and a user who clicks it again is asking for
    /// a fresh start rather than for the held picks to be preserved.
    pub fn open(&mut self, kind: ToolKind) {
        // Replace the whole value: this is the one line that has to
        // know the tool set, and it knows it structurally.
        *self = Self::default();
        match kind {
            ToolKind::Mate => self.mate = Some(MateTool::new()),
            ToolKind::Revolve => self.revolve = Some(RevolveTool::new()),
            ToolKind::Boolean => self.boolean = Some(BooleanTool::new()),
            ToolKind::Split => self.split = Some(SplitTool::new()),
            ToolKind::Transform => self.transform = Some(TransformTool::new()),
            ToolKind::Pattern => self.pattern = Some(PatternTool::new()),
        }
    }

    /// Close whatever is open (the Cancel door, and the one a
    /// committed edit takes).
    pub fn close(&mut self) {
        *self = Self::default();
    }

    /// Which tool is open, if any.
    pub fn open_kind(&self) -> Option<ToolKind> {
        if self.mate.is_some() {
            Some(ToolKind::Mate)
        } else if self.revolve.is_some() {
            Some(ToolKind::Revolve)
        } else if self.boolean.is_some() {
            Some(ToolKind::Boolean)
        } else if self.split.is_some() {
            Some(ToolKind::Split)
        } else if self.transform.is_some() {
            Some(ToolKind::Transform)
        } else {
            self.pattern.is_some().then_some(ToolKind::Pattern)
        }
    }

    /// The open mate tool.
    pub fn mate(&self) -> Option<&MateTool> {
        self.mate.as_ref()
    }

    /// The open revolve tool.
    pub fn revolve(&self) -> Option<RevolveTool> {
        self.revolve
    }

    /// The open boolean tool.
    pub fn boolean(&self) -> Option<BooleanTool> {
        self.boolean
    }

    /// The open split tool.
    pub fn split(&self) -> Option<SplitTool> {
        self.split
    }

    /// The open transform tool.
    pub fn transform(&self) -> Option<TransformTool> {
        self.transform
    }

    /// The open pattern tool.
    pub fn pattern(&self) -> Option<PatternTool> {
        self.pattern
    }

    /// **Feed one frame's operations to the open tool.**
    ///
    /// A selection is the only op a tool consumes, and the two
    /// vocabularies are the ones the tools were written against: the
    /// mate tool takes the FACE (its alignment frames are derived from
    /// face geometry), and every node tool takes `Selection::node` —
    /// a tree click directly, a face pick through the one
    /// viewport→tree inversion.
    pub fn feed(&mut self, ops: &[SessionOp]) {
        for op in ops {
            let SessionOp::Select(selection) = op else {
                continue;
            };
            if let Some(tool) = self.mate.as_mut()
                && let Some(face) = selection.face()
            {
                tool.pick(face.clone());
            }
            let Some(node) = selection.node() else {
                continue;
            };
            if let Some(tool) = self.revolve.as_mut() {
                tool.pick(node);
            }
            if let Some(tool) = self.boolean.as_mut() {
                tool.pick(node);
            }
            if let Some(tool) = self.split.as_mut() {
                tool.pick(node);
            }
            if let Some(tool) = self.transform.as_mut() {
                tool.pick(node);
            }
            if let Some(tool) = self.pattern.as_mut() {
                tool.pick(node);
            }
        }
    }

    /// **The survival step, once per frame** — the consumer obligation
    /// every tool's module docs state, discharged in one place.
    ///
    /// The node tools answer from the document alone (a held node is
    /// there or it is not); the mate tool needs the landed PAIR,
    /// because a face pick's survival is a question about a name and
    /// an evaluation together. A session with nothing landed yet
    /// therefore reconciles the node tools and leaves the mate tool's
    /// picks alone, which is the honest answer: "we cannot tell" is
    /// not "it is gone".
    pub fn reconcile(
        &mut self,
        doc: &Doc<ProfileProgram>,
        landed: Option<(&Doc<ProfileProgram>, &Evaluation<f64>)>,
    ) -> Vec<ToolNotice> {
        let mut notices = Vec::new();
        if let (Some(tool), Some((landed_doc, eval))) = (self.mate.as_mut(), landed) {
            notices.extend(
                tool.reconcile(landed_doc, eval)
                    .into_iter()
                    .map(ToolNotice::Mate),
            );
        }
        if let Some(tool) = self.revolve.as_mut() {
            notices.extend(tool.reconcile(doc).into_iter().map(ToolNotice::Revolve));
        }
        let mut combining: Vec<(ToolKind, Vec<CombineToolEvent>)> = Vec::new();
        if let Some(tool) = self.boolean.as_mut() {
            combining.push((ToolKind::Boolean, tool.reconcile(doc)));
        }
        if let Some(tool) = self.split.as_mut() {
            combining.push((ToolKind::Split, tool.reconcile(doc)));
        }
        if let Some(tool) = self.transform.as_mut() {
            combining.push((ToolKind::Transform, tool.reconcile(doc)));
        }
        if let Some(tool) = self.pattern.as_mut() {
            combining.push((ToolKind::Pattern, tool.reconcile(doc)));
        }
        for (tool, events) in combining {
            notices.extend(
                events
                    .into_iter()
                    .map(|event| ToolNotice::Combine { tool, event }),
            );
        }
        notices
    }
}
