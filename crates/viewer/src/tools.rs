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
//! **Every other per-tool rule here dispatches on [`ToolKind`] through
//! an exhaustive match** — the pick routing, the survival step, the
//! cursor narrowing, the close-on-commit edit — for the same reason:
//! a seventh tool must not be able to compile while three of its four
//! obligations are silently unmet. The one list a compiler cannot
//! force is [`ToolKind::ALL`], and [`ToolKind::ordinal`] is what makes
//! its completeness checkable by a row instead of by eye.
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
    /// The blend tool: one body and a SET of its edges.
    Blend,
}

impl ToolKind {
    /// Every kind, for a chrome that offers them and a test that sweeps
    /// them.
    ///
    /// A hand-written list, which is why [`ToolKind::ordinal`] exists:
    /// `tools::every_kind_is_listed_in_all` reads the two against each
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
            Self::Pattern => matches!(op, SessionOp::AddPattern { .. }),
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
    blend: Option<BlendTool>,
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
            ToolKind::Blend => self.blend = Some(BlendTool::new()),
        }
    }

    /// Close whatever is open (the Cancel door, and the one a committed
    /// edit takes).
    pub fn close(&mut self) {
        *self = Self::default();
    }

    /// Which tool is open, if any.
    pub fn open_kind(&self) -> Option<ToolKind> {
        ToolKind::ALL.into_iter().find(|&kind| match kind {
            ToolKind::Mate => self.mate.is_some(),
            ToolKind::Revolve => self.revolve.is_some(),
            ToolKind::Boolean => self.boolean.is_some(),
            ToolKind::Split => self.split.is_some(),
            ToolKind::Transform => self.transform.is_some(),
            ToolKind::Pattern => self.pattern.is_some(),
            ToolKind::Blend => self.blend.is_some(),
        })
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

    /// The open blend tool, by reference: it holds a SET, so it is the
    /// one tool value too large to hand back by copy.
    pub fn blend(&self) -> Option<&BlendTool> {
        self.blend.as_ref()
    }

    /// The open blend tool, mutably — the door the all-edges
    /// affordance loads its set through, that being a tool-state
    /// operation and not a document edit.
    pub fn blend_mut(&mut self) -> Option<&mut BlendTool> {
        self.blend.as_mut()
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
            match self.open_kind() {
                None => {}
                Some(ToolKind::Mate) => {
                    if let (Some(tool), Some(face)) = (self.mate.as_mut(), selection.face()) {
                        tool.pick(face.clone());
                    }
                }
                Some(ToolKind::Blend) => {
                    if let (Some(tool), Some(edge)) = (self.blend.as_mut(), selection.edge()) {
                        notices.extend(tool.pick(edge).map(ToolNotice::Blend));
                    }
                }
                Some(ToolKind::Revolve) => seat(self.revolve.as_mut(), doc, selection.node()),
                Some(ToolKind::Boolean) => seat(self.boolean.as_mut(), doc, selection.node()),
                Some(ToolKind::Split) => seat(self.split.as_mut(), doc, selection.node()),
                Some(ToolKind::Transform) => seat(self.transform.as_mut(), doc, selection.node()),
                Some(ToolKind::Pattern) => seat(self.pattern.as_mut(), doc, selection.node()),
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
        let Some(kind) = self.open_kind() else {
            return Vec::new();
        };
        let dropped = match kind {
            ToolKind::Mate => {
                return match (self.mate.as_mut(), landed) {
                    (Some(tool), Some((landed_doc, eval))) => tool
                        .reconcile(landed_doc, eval)
                        .into_iter()
                        .map(ToolNotice::Mate)
                        .collect(),
                    _ => Vec::new(),
                };
            }
            ToolKind::Revolve => drop_lost(self.revolve.as_mut(), doc),
            ToolKind::Boolean => drop_lost(self.boolean.as_mut(), doc),
            ToolKind::Split => drop_lost(self.split.as_mut(), doc),
            ToolKind::Transform => drop_lost(self.transform.as_mut(), doc),
            ToolKind::Pattern => drop_lost(self.pattern.as_mut(), doc),
            ToolKind::Blend => {
                return self
                    .blend
                    .as_mut()
                    .map(|tool| tool.reconcile(doc, landed))
                    .unwrap_or_default()
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

/// What [`Tools`] needs of a seated tool: a node pick goes in, and the
/// survival step comes back out. One trait rather than one arm per tool
/// per rule, so the routing above is a match on the KIND and nothing
/// else — and a seventh tool has to implement it before it can be
/// routed at all.
trait Seated {
    fn seat_pick(&mut self, doc: &Doc<ProfileProgram>, node: RecipeNodeId);
    fn seat_reconcile(&mut self, doc: &Doc<ProfileProgram>) -> Vec<SeatEvent>;
}

/// Feed one seated tool, if it is the one open.
fn seat<T: Seated>(tool: Option<&mut T>, doc: &Doc<ProfileProgram>, node: Option<RecipeNodeId>) {
    if let (Some(tool), Some(node)) = (tool, node) {
        tool.seat_pick(doc, node);
    }
}

/// Reconcile one seated tool, if it is the one open.
fn drop_lost<T: Seated>(tool: Option<&mut T>, doc: &Doc<ProfileProgram>) -> Vec<SeatEvent> {
    tool.map(|tool| tool.seat_reconcile(doc))
        .unwrap_or_default()
}

macro_rules! seated {
    ($($t:ty),+ $(,)?) => {
        $(impl Seated for $t {
            fn seat_pick(&mut self, doc: &Doc<ProfileProgram>, node: RecipeNodeId) {
                self.pick(doc, node);
            }
            fn seat_reconcile(&mut self, doc: &Doc<ProfileProgram>) -> Vec<SeatEvent> {
                self.reconcile(doc)
            }
        })+
    };
}

seated!(
    RevolveTool,
    BooleanTool,
    SplitTool,
    TransformTool,
    PatternTool
);
