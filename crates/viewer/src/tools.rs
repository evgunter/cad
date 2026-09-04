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
//! So the set of tools is ONE value with one door in: [`Tools`] holds
//! an `Option<OpenTool>`, one variant per kind, so "two tools open" is
//! not a state the door has to avoid but a state that cannot be
//! written down. A tool added to the set cannot be forgotten by an
//! exclusivity rule that no longer exists.
//!
//! **The four per-tool rules here dispatch through an exhaustive
//! match** — the pick routing, the survival step, the cursor
//! narrowing, the close-on-commit edit — for the same reason: an
//! eighth tool must not be able to compile while three of its four
//! obligations are silently unmet. The READ door is not one of them:
//! each typed accessor on [`Tools`] matches its own variant and
//! answers `None` to every other, so an eighth tool that never gets
//! an accessor compiles clean. The one list a compiler cannot force
//! is [`ToolKind::ALL`], which nothing outside the test suites reads,
//! and [`ToolKind::ordinal`] is what makes its completeness checkable
//! by a row instead of by eye.
//!
//! The value is renderer-free on purpose: the pick routing, the
//! survival step and the exclusivity are all properties a headless row
//! asserts, and only the widgets that open and read the tools need a
//! window.

use pncad::document::{Doc, Evaluation, ProfileProgram, RecipeNodeId};

use crate::blend::{BlendEvent, BlendTool};
use crate::combine::{BooleanTool, PatternTool, SplitTool, TransformTool};
use crate::matetool::{MateTool, MateToolEvent};
use crate::pick::PickKinds;
use crate::revolvetool::RevolveTool;
use crate::seats::SeatEvent;
use crate::session::{Selection, SessionOp};

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
    /// The blend tool: one body and a SET of its edges.
    Blend,
}

impl ToolKind {
    /// Every kind, for the test suites that sweep them — which are
    /// its only readers. No production code reads it: the chrome names
    /// each kind it offers literally, and which tool is open is a value
    /// ([`OpenTool`]), not a scan over this list.
    ///
    /// A hand-written list, which is why [`ToolKind::ordinal`] exists:
    /// `every_tool_kind_is_listed_in_all` reads the two against each
    /// other, so a variant added to the enum and forgotten here fails a
    /// row rather than quietly narrowing every sweep.
    pub const ALL: [Self; 7] = [
        Self::Mate,
        Self::Revolve,
        Self::Boolean,
        Self::Split,
        Self::Transform,
        Self::Pattern,
        Self::Blend,
    ];

    /// A place in [`ToolKind::ALL`], as an exhaustive match — the
    /// compiler-forced half of that list's completeness.
    pub fn ordinal(self) -> usize {
        match self {
            Self::Mate => 0,
            Self::Revolve => 1,
            Self::Boolean => 2,
            Self::Split => 3,
            Self::Transform => 4,
            Self::Pattern => 5,
            Self::Blend => 6,
        }
    }

    /// The tool's name, for sentences and buttons.
    pub fn label(self) -> &'static str {
        match self {
            Self::Mate => "mate tool",
            Self::Revolve => "revolve tool",
            Self::Boolean => "boolean tool",
            Self::Split => "split tool",
            Self::Transform => "transform tool",
            Self::Pattern => "pattern tool",
            Self::Blend => "blend tool",
        }
    }

    /// **The one composition of a tool's sentence**: what the status
    /// line shows when this tool has something to say, whether the tool
    /// said it (a refusal at a commit button) or the frame did (a lost
    /// pick). Two spellings of this prefix is how the two drift.
    pub fn says(self, what: &impl core::fmt::Display) -> String {
        format!("{}: {what}", self.label())
    }

    /// **What the cursor may pick while this tool is open** — an open
    /// tool narrows the priority rule, it does not re-decide it.
    ///
    /// The mate tool takes faces, and on a real part whole faces sit
    /// within the edge radius of their own boundary — a narrow shelf, a
    /// small hole's wall — so with edges always winning those faces
    /// were unpickable for as long as the tool was open. The blend tool
    /// is the mirror case: it takes EDGES and nothing else, so a cursor
    /// no edge wins answers NOTHING rather than re-selecting the wall
    /// behind the edge the user was aiming at. Every other tool holds
    /// NODE picks, which a face and an edge answer equally well
    /// (`Selection::node` reaches the feature either way), so none of
    /// them narrows anything.
    pub fn pick_kinds(self) -> PickKinds {
        match self {
            Self::Mate => PickKinds::FacesOnly,
            Self::Blend => PickKinds::EdgesOnly,
            Self::Revolve | Self::Boolean | Self::Split | Self::Transform | Self::Pattern => {
                PickKinds::Any
            }
        }
    }

    /// **Whether this operation is this tool's one committed edit** —
    /// the rule that closes the tool that authored it, once the edit
    /// has actually landed.
    ///
    /// The mate tool answers `false` for every op deliberately: it
    /// closes at its own click, before the op is performed, which is
    /// the shipped GUI-4 behaviour and not this rule's to change.
    pub fn commits(self, op: &SessionOp) -> bool {
        match self {
            Self::Mate => false,
            Self::Revolve => matches!(op, SessionOp::AddRevolve { .. }),
            Self::Boolean => matches!(op, SessionOp::AddBoolean { .. }),
            Self::Split => matches!(op, SessionOp::AddSplit { .. }),
            Self::Transform => matches!(op, SessionOp::AddTransform { .. }),
            // Two ops, one tool, for the blend tool's reason: the
            // output choice picks the door, and either one landing is
            // this tool's edit committed.
            Self::Pattern => matches!(
                op,
                SessionOp::AddPattern { .. } | SessionOp::AddPlacedUnion { .. }
            ),
            // Two ops, one tool: the kind choice picks the door, and
            // either one landing is this tool's edit committed.
            Self::Blend => matches!(
                op,
                SessionOp::AddFillet { .. } | SessionOp::AddChamfer { .. }
            ),
        }
    }
}

/// Something a tool did on its own — a survival drop, or a pick it
/// declined — carrying which tool it was about, so the sentence a
/// chrome shows is composed once ([`ToolKind::says`]) rather than at
/// each call site.
#[derive(Debug)]
pub enum ToolNotice {
    /// The mate tool lost a pick — its own event vocabulary, which
    /// degrades by STEP rather than by seat.
    Mate(MateToolEvent),
    /// The blend tool has something to say about a pick or its
    /// target — its own event vocabulary, which is about a SET rather
    /// than about seats.
    Blend(BlendEvent),
    /// A seated tool lost a pick.
    Seated {
        /// Which tool.
        tool: ToolKind,
        /// What it lost.
        event: SeatEvent,
    },
}

impl core::fmt::Display for ToolNotice {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let said = match self {
            Self::Mate(event) => ToolKind::Mate.says(event),
            Self::Blend(event) => ToolKind::Blend.says(event),
            Self::Seated { tool, event } => tool.says(event),
        };
        f.write_str(&said)
    }
}

/// **Which tool is open, and its state** — one variant per kind, so
/// the open tool is a single value rather than seven optional ones and
/// "two tools open" has no spelling.
#[derive(Debug)]
pub enum OpenTool {
    /// The mate tool.
    Mate(MateTool),
    /// The revolve tool.
    Revolve(RevolveTool),
    /// The boolean tool.
    Boolean(BooleanTool),
    /// The split tool.
    Split(SplitTool),
    /// The transform tool.
    Transform(TransformTool),
    /// The pattern tool.
    Pattern(PatternTool),
    /// The blend tool.
    Blend(BlendTool),
}

impl OpenTool {
    /// Which kind this is — the vocabulary the rules that do not need
    /// the tool's own state are addressed in.
    pub fn kind(&self) -> ToolKind {
        match self {
            Self::Mate(_) => ToolKind::Mate,
            Self::Revolve(_) => ToolKind::Revolve,
            Self::Boolean(_) => ToolKind::Boolean,
            Self::Split(_) => ToolKind::Split,
            Self::Transform(_) => ToolKind::Transform,
            Self::Pattern(_) => ToolKind::Pattern,
            Self::Blend(_) => ToolKind::Blend,
        }
    }
}

/// **The one guard every seated tool's pick shares.** A seated tool
/// takes `Selection::node` and nothing else — a tree click directly, a
/// face or edge pick through the one viewport→tree inversion — so a
/// selection carrying no node is a click that tool does not see. The
/// arms of [`Tools::feed`] that hold seats name the tool and share
/// this; none of them re-spells it.
fn on_node_pick(selection: &Selection, pick: impl FnOnce(RecipeNodeId)) {
    if let Some(node) = selection.node() {
        pick(node);
    }
}

/// The modal tools as one value: at most one is open, and the open one
/// is the only one the selection stream reaches.
///
/// **The read door hands a tool back by value iff that tool is
/// `Copy`.** The seated tools are small `Copy` values and answer by
/// value; the mate tool holds picked faces and the blend tool holds a
/// SET of edges, so those two answer by reference. That is the whole
/// rule, and the accessors below name which side of it they are on
/// rather than each re-arguing it.
#[derive(Debug, Default)]
pub struct Tools {
    open: Option<OpenTool>,
}

impl Tools {
    /// No tool open.
    pub fn new() -> Self {
        Self::default()
    }

    /// **Open one tool, closing whatever was open.**
    ///
    /// Opening the tool that is already open starts it over: the value
    /// is replaced, so its held picks go. Nothing in the chrome can
    /// reach that path today — a tool's activation button is shown only
    /// while it is closed — so this is the door's rule rather than an
    /// affordance, and the FORM fields (an angle, a count) are drafts
    /// living outside the tool and are untouched either way.
    pub fn open(&mut self, kind: ToolKind) {
        self.open = Some(match kind {
            ToolKind::Mate => OpenTool::Mate(MateTool::new()),
            ToolKind::Revolve => OpenTool::Revolve(RevolveTool::new()),
            ToolKind::Boolean => OpenTool::Boolean(BooleanTool::new()),
            ToolKind::Split => OpenTool::Split(SplitTool::new()),
            ToolKind::Transform => OpenTool::Transform(TransformTool::new()),
            ToolKind::Pattern => OpenTool::Pattern(PatternTool::new()),
            ToolKind::Blend => OpenTool::Blend(BlendTool::new()),
        });
    }

    /// Close whatever is open (the Cancel door, and the one a committed
    /// edit takes).
    pub fn close(&mut self) {
        self.open = None;
    }

    /// Which tool is open, if any.
    pub fn open_kind(&self) -> Option<ToolKind> {
        self.open.as_ref().map(OpenTool::kind)
    }

    /// The open mate tool, by reference (the read-door rule on
    /// [`Tools`]).
    pub fn mate(&self) -> Option<&MateTool> {
        match &self.open {
            Some(OpenTool::Mate(tool)) => Some(tool),
            _ => None,
        }
    }

    /// The open revolve tool.
    pub fn revolve(&self) -> Option<RevolveTool> {
        match &self.open {
            Some(OpenTool::Revolve(tool)) => Some(*tool),
            _ => None,
        }
    }

    /// The open boolean tool.
    pub fn boolean(&self) -> Option<BooleanTool> {
        match &self.open {
            Some(OpenTool::Boolean(tool)) => Some(*tool),
            _ => None,
        }
    }

    /// The open split tool.
    pub fn split(&self) -> Option<SplitTool> {
        match &self.open {
            Some(OpenTool::Split(tool)) => Some(*tool),
            _ => None,
        }
    }

    /// The open transform tool.
    pub fn transform(&self) -> Option<TransformTool> {
        match &self.open {
            Some(OpenTool::Transform(tool)) => Some(*tool),
            _ => None,
        }
    }

    /// The open pattern tool.
    pub fn pattern(&self) -> Option<PatternTool> {
        match &self.open {
            Some(OpenTool::Pattern(tool)) => Some(*tool),
            _ => None,
        }
    }

    /// The open blend tool, by reference (the read-door rule on
    /// [`Tools`]).
    pub fn blend(&self) -> Option<&BlendTool> {
        match &self.open {
            Some(OpenTool::Blend(tool)) => Some(tool),
            _ => None,
        }
    }

    /// The open blend tool, mutably — the door the all-edges
    /// affordance loads its set through, that being a tool-state
    /// operation and not a document edit.
    pub fn blend_mut(&mut self) -> Option<&mut BlendTool> {
        match &mut self.open {
            Some(OpenTool::Blend(tool)) => Some(tool),
            _ => None,
        }
    }

    /// **What the cursor may pick right now** ([`ToolKind::pick_kinds`]
    /// carries the rule); the bare cursor's rule with nothing open.
    pub fn pick_kinds(&self) -> PickKinds {
        self.open_kind()
            .map_or(PickKinds::Any, ToolKind::pick_kinds)
    }

    /// **Whether an operation is the open tool's one committed edit**,
    /// which is what closes it. Answered against the OPEN tool rather
    /// than against the op alone, so an op that no tool authored — or
    /// one authored while a different tool is open — never closes
    /// anything.
    pub fn commits_open_tool(&self, op: &SessionOp) -> bool {
        self.open_kind().is_some_and(|kind| kind.commits(op))
    }

    /// **Feed one frame's operations to the open tool.**
    ///
    /// A selection is the only op a tool consumes, and the two
    /// vocabularies are the ones the tools were written against: the
    /// mate tool takes the FACE (its alignment frames are derived from
    /// face geometry, so an edge pick is not one of its picks), the
    /// blend tool takes the EDGE (it blends edges, and its target is
    /// the drawn body the edge was picked on rather than the feature
    /// that minted it), and every seated tool takes `Selection::node`
    /// — a tree click directly, a face or edge pick through the one
    /// viewport→tree inversion.
    ///
    /// **Feeding answers**, because a pick can be DECLINED: the blend
    /// tool refuses an edge on a second body rather than taking it or
    /// dropping it silently. The returned notices are the same values
    /// [`Tools::reconcile`] answers with and are shown the same way; a
    /// frame in which every pick landed answers with none.
    ///
    /// **`doc` is what routes a seated tool's pick**, not what
    /// judges it: a seat asks the document what KIND of node was
    /// picked so a pick that only one of two seats can hold lands in
    /// that one (`crate::seats`). The commit door still owns the
    /// verdict.
    ///
    /// `#[must_use]`: the notices ARE the refusal — dropping them is
    /// how a declined pick becomes a click that silently did nothing,
    /// which is the bug this return value exists to prevent.
    #[must_use = "a declined pick is only reported if its notice is shown"]
    pub fn feed(&mut self, doc: &Doc<ProfileProgram>, ops: &[SessionOp]) -> Vec<ToolNotice> {
        let mut notices = Vec::new();
        for op in ops {
            let SessionOp::Select(selection) = op else {
                continue;
            };
            match &mut self.open {
                None => {}
                Some(OpenTool::Mate(tool)) => {
                    if let Some(face) = selection.face() {
                        tool.pick(face.clone());
                    }
                }
                Some(OpenTool::Blend(tool)) => {
                    if let Some(edge) = selection.edge() {
                        notices.extend(tool.pick(edge).map(ToolNotice::Blend));
                    }
                }
                Some(OpenTool::Revolve(tool)) => {
                    on_node_pick(selection, |node| tool.pick(doc, node));
                }
                Some(OpenTool::Boolean(tool)) => {
                    on_node_pick(selection, |node| tool.pick(doc, node));
                }
                Some(OpenTool::Split(tool)) => {
                    on_node_pick(selection, |node| tool.pick(doc, node));
                }
                Some(OpenTool::Transform(tool)) => {
                    on_node_pick(selection, |node| tool.pick(doc, node));
                }
                Some(OpenTool::Pattern(tool)) => {
                    on_node_pick(selection, |node| tool.pick(doc, node));
                }
            }
        }
        notices
    }

    /// **The survival step, once per frame** — the consumer obligation
    /// every tool's module docs state, discharged in one place.
    ///
    /// The seated tools answer from the shown document alone (a held
    /// node is there or it is not); the mate tool needs the landed
    /// PAIR, because a face pick's survival is a question about a name
    /// and an evaluation together. A session with nothing landed yet
    /// therefore reconciles the seated tools and leaves the mate tool's
    /// picks alone, which is the honest answer: "we cannot tell" is not
    /// "it is gone".
    ///
    /// The blend tool needs BOTH: the document says whether its target
    /// still exists, and the landed evaluation says which edges that
    /// target still has — two questions, two answers, one per event
    /// arm (`crate::blend::BlendTool::reconcile`).
    #[must_use = "a dropped pick is only reported if its notice is shown"]
    pub fn reconcile(
        &mut self,
        doc: &Doc<ProfileProgram>,
        landed: Option<(&Doc<ProfileProgram>, &Evaluation<f64>)>,
    ) -> Vec<ToolNotice> {
        let Some(open) = self.open.as_mut() else {
            return Vec::new();
        };
        let kind = open.kind();
        let dropped = match open {
            OpenTool::Mate(tool) => {
                return match landed {
                    Some((landed_doc, eval)) => tool
                        .reconcile(landed_doc, eval)
                        .into_iter()
                        .map(ToolNotice::Mate)
                        .collect(),
                    None => Vec::new(),
                };
            }
            OpenTool::Revolve(tool) => tool.reconcile(doc),
            OpenTool::Boolean(tool) => tool.reconcile(doc),
            OpenTool::Split(tool) => tool.reconcile(doc),
            OpenTool::Transform(tool) => tool.reconcile(doc),
            OpenTool::Pattern(tool) => tool.reconcile(doc),
            OpenTool::Blend(tool) => {
                return tool
                    .reconcile(doc, landed)
                    .into_iter()
                    .map(ToolNotice::Blend)
                    .collect();
            }
        };
        dropped
            .into_iter()
            .map(|event| ToolNotice::Seated { tool: kind, event })
            .collect()
    }
}
