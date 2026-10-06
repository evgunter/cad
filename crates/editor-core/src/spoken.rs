//! **How a sentence names a recipe node** (DESIGN.md Band 1, "Node
//! labels"): a person reads a node as its kind, its label and its tag,
//! `Extrude "base plate" (3fa9c1d2a0b1)`, or as its kind and tag when
//! it has no label, `Extrude 3fa9c1d2a0b1`; spoken from a document that
//! does not hold it, `node 3fa9c1d2a0b1`. A sentence spoken again from
//! a later version of its document ([`SpokenNode::respoken`]) says a
//! node that version does not hold as it first said it.
//!
//! Four spellings, one home each:
//!
//! - [`SpokenNode`] — the node as a person reads it, read off the
//!   document that holds it ([`crate::Doc::spoken`]). No label a
//!   sentence says is one its value can outlive: a value the evaluation
//!   memo reuses holds a label only when its memo key fixes that label.
//!   A document's own labels are outside its key, so what it memoizes
//!   keeps bare ids, and the frame that owns the document says them
//!   when the sentence is made. A part's labels are in its pin, which is
//!   in the instance's key, so a fault from inside a part holds the
//!   part's nodes as the pinned part says them
//!   ([`crate::PartFault::held`]).
//! - The `Display` of [`RecipeNodeId`] and [`StepId`] — the bare tag,
//!   for a sentence made where no document is at hand (a refusal's own
//!   `Display`, a stored reference). The edit, load and save doors all
//!   hold a document, so each speaks.
//! - [`Speaker`] — a sentence written once over the speaker, for a
//!   refusal that holds bare ids (one the evaluation memo reuses, or
//!   one a door raised from an evaluation alone): its `Display` says
//!   each node by tag ([`Speaker::TAG`]), and its `spoken(doc)` says
//!   each as the document of the frame handing it out holds it
//!   ([`Speaker::of`]). A site that holds the document and carries
//!   such a value whole keeps the nodes its words name as
//!   [`HeldNodes`] ([`held_by`]), and says them back with no document
//!   at hand ([`Speaker::held`]). Inside a line that already names a
//!   node ([`Speaker::about`]), that node reads `this <noun>`, so no
//!   sentence names it twice.
//! - [`FullId`] — every bit of the id, for a machine channel (a
//!   binding's `repr`, a goldened report) where two ids must never
//!   print alike.
//!
//! A [`StableName`] is a stored reference with no document behind it,
//! so its own `Display` says its nodes and its profile steps by tag; a
//! sentence made where the document is at hand says it as a
//! [`SpokenName`], each step where the profile pane shows it. Both are
//! [`Speaker::name`]'s one sentence, in full; a speaker holding the
//! name's table says it at a detail that tells it apart there
//! ([`Speaker::within`]).

use core::fmt;

use crate::doc::Doc;
use crate::label::Label;
use crate::names::words::Detail;
use crate::names::{NameTable, StableName};
use crate::node::{BooleanOp, Datum, Node, RecipeNodeId, StepId};
use crate::program::ProfilePayload;

/// How many hex digits a tag shows (`test_utils::refusal::NODE_TAG_DIGITS`
/// mirrors it, so the refusal-shape oracle does not read a tag as a
/// leaked document id).
const TAG_DIGITS: usize = 12;

/// How far a tag shifts the id: it shows the HIGH [`TAG_DIGITS`] hex
/// digits. An id is the head of a digest read big-endian
/// (`crate::mint`), so its leading digits are the hash's own, and a
/// tag is the id's prefix — the rule a `DocRef`'s pin prefix follows.
const TAG_SHIFT: u32 = 64 - 4 * TAG_DIGITS as u32;

fn write_tag(f: &mut fmt::Formatter<'_>, bits: u64) -> fmt::Result {
    write!(f, "{:0width$x}", bits >> TAG_SHIFT, width = TAG_DIGITS)
}

/// The bare tag: the id's high 48 bits as 12 lowercase hex digits, the
/// first twelve of its sixteen (`3fa9c1d2a0b1`).
impl fmt::Display for RecipeNodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_tag(f, self.0)
    }
}

/// The bare tag, spelled as a node's is.
impl fmt::Display for StepId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_tag(f, self.0)
    }
}

/// **An id with every bit shown**: 16 lowercase hex digits,
/// zero-padded. The machine channel's spelling, where a tag's
/// abbreviation could make two ids read alike.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FullId(pub u64);

impl fmt::Display for FullId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:016x}", self.0)
    }
}

impl RecipeNodeId {
    /// The id with every bit shown ([`FullId`]).
    #[must_use]
    pub fn full(self) -> FullId {
        FullId(self.0)
    }
}

impl StepId {
    /// The id with every bit shown ([`FullId`]).
    #[must_use]
    pub fn full(self) -> FullId {
        FullId(self.0)
    }
}

/// `#` and every bit, `#3fa9c1d2a0b1c3d4`: the one spelling of a
/// variable with no name, the text [`crate::unparse`] writes for its
/// reader.
impl fmt::Display for crate::var::VarId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{}", self.full())
    }
}

impl crate::var::VarId {
    /// The id with every bit shown ([`FullId`]).
    #[must_use]
    pub fn full(self) -> FullId {
        FullId(self.0)
    }
}

/// **A variable as a person reads it** (VARIABLES-DESIGN VR2): its
/// name (`w`), or `#3fa9c1d2a0b1c3d4` when it has none (the
/// [`crate::var::VarId`] spelling, which a formula's reader shares). Built by
/// [`Doc::spoken_var`] from the document that holds the variable;
/// refusals carry it the way they carry a [`SpokenNode`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpokenVar {
    id: crate::var::VarId,
    name: Option<crate::doc::VarName>,
}

impl SpokenVar {
    /// `id`, under `name` when it has one.
    #[must_use]
    pub fn new(id: crate::var::VarId, name: Option<crate::doc::VarName>) -> Self {
        Self { id, name }
    }

    /// The variable this sentence names.
    #[must_use]
    pub fn id(&self) -> crate::var::VarId {
        self.id
    }

    /// Its name, when the document held one.
    #[must_use]
    pub fn name(&self) -> Option<&crate::doc::VarName> {
        self.name.as_ref()
    }

    /// This variable spoken again from `doc`, a later version of the
    /// document it was spoken from: under the name `doc` holds for it
    /// now (none, after a clear), or as it was said when `doc` does not
    /// hold it — a deleted variable, or one a refused declare would
    /// have minted. An id names one variable within a document's
    /// history, as [`SpokenNode::respoken`] says of a node.
    #[must_use]
    pub fn respoken<P>(&self, doc: &Doc<P>) -> Self {
        if doc.var(self.id).is_some() {
            doc.spoken_var(self.id)
        } else {
            self.clone()
        }
    }
}

impl fmt::Display for SpokenVar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.name {
            Some(name) => write!(f, "{name}"),
            None => write!(f, "{}", self.id),
        }
    }
}

/// **A recipe node as a person reads it**: its kind noun, its label
/// and its tag (`Extrude "base plate" (3fa9c1d2a0b1)`, with a `"` or
/// `\` in the label escaped by a `\`), its kind and
/// tag when it has no label (`Extrude 3fa9c1d2a0b1`), or
/// `node 3fa9c1d2a0b1` for an id the document it was spoken from did
/// not hold.
///
/// Built by [`Doc::spoken`] from the document that holds the node, and
/// spoken again from a later version by [`SpokenNode::respoken`].
/// The tag is always said: labels repeat, and a kept sentence finds
/// its node after a rename by the tag.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpokenNode {
    id: RecipeNodeId,
    /// The kind noun, `None` for an id the document does not hold.
    kind: Option<&'static str>,
    /// The node's label, `None` when it has none or is not held. Boxed
    /// so that a spoken node stays 32 bytes on a 64-bit target (the id,
    /// the kind's two words, the box): the edit refusals hold up to
    /// two, and every edit door returns them by value.
    label: Option<Box<Label>>,
}

// The width the label's box buys, held where clippy measures it: an
// unboxed label makes it 48 bytes, and `PersistError`, which carries an
// `EditError`, crosses clippy's 128-byte large-`Err` line.
#[cfg(target_pointer_width = "64")]
const _: () = assert!(core::mem::size_of::<SpokenNode>() == 32);

impl SpokenNode {
    /// A spoken node with no document behind it, for a fixture that
    /// builds by hand what a document would say.
    #[cfg(any(test, feature = "test-support"))]
    pub(crate) fn forged(
        id: RecipeNodeId,
        kind: Option<&'static str>,
        label: Option<Label>,
    ) -> Self {
        Self {
            id,
            kind,
            label: label.map(Box::new),
        }
    }

    /// A node no document at hand holds: `node <tag>`, what
    /// [`Doc::spoken`] answers for an id its document does not hold.
    #[must_use]
    pub fn absent(id: RecipeNodeId) -> Self {
        Self {
            id,
            kind: None,
            label: None,
        }
    }

    /// The node an insert is minting, before the document holds it:
    /// its kind and tag. An insert carries no label, so it has none.
    pub(crate) fn entering<P, S: crate::Slot>(id: RecipeNodeId, node: &Node<P, S>) -> Self {
        Self {
            id,
            kind: Some(node_kind_noun(node)),
            label: None,
        }
    }

    /// The node this sentence names.
    #[must_use]
    pub fn id(&self) -> RecipeNodeId {
        self.id
    }

    /// The node's kind noun ([`node_kind_noun`]), `None` when the
    /// document it was first spoken from did not hold the node.
    #[must_use]
    pub fn kind(&self) -> Option<&'static str> {
        self.kind
    }

    /// The node's label as the document held it when this was built,
    /// `None` when it had none.
    #[must_use]
    pub fn label(&self) -> Option<&Label> {
        self.label.as_deref()
    }

    /// **This node spoken again from `doc`, a later version of the
    /// document it was spoken from**: as `doc` holds it now, or as it
    /// was first spoken when either document lacks it. A node `doc`
    /// does not hold — one a refused insert was minting, one a later
    /// edit deleted — keeps what it was; a node the first document did
    /// not hold stays `node <tag>`, since a sentence names a node by
    /// its tag alone only to say that it is not there.
    ///
    /// **Within one document's history an id names one node**, which
    /// is what makes this sound. An id is the head of the document's
    /// mint chain at the insert that minted it ([`crate::mint`]), a
    /// digest of every minting edit before it, so two versions that
    /// part from one value — an undo, then a different insert — mint
    /// different ids from there on (pinned by the viewer's
    /// `node_labels::an_undo_then_a_different_insert_mints_a_different_id`).
    /// An id is not document-scoped, so `doc` is never a version of
    /// another document: that document's chain says nothing about
    /// these ids. Every `respoken` in this tree and the viewer cites
    /// this paragraph.
    #[must_use]
    pub fn respoken<P>(&self, doc: &Doc<P>) -> Self {
        if self.kind.is_some() && doc.node(self.id).is_some() {
            doc.spoken(self.id)
        } else {
            self.clone()
        }
    }
}

impl fmt::Display for SpokenNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.kind, &self.label) {
            (Some(kind), Some(label)) => {
                write!(f, "{kind} \"")?;
                // The quote and the escape are escaped, so a label
                // holding `"` cannot read as the end of the quotation.
                for ch in label.as_str().chars() {
                    if matches!(ch, '"' | '\\') {
                        f.write_str("\\")?;
                    }
                    write!(f, "{ch}")?;
                }
                write!(f, "\" ({})", self.id)
            }
            (kind, _) => write!(f, "{} {}", kind.unwrap_or("node"), self.id),
        }
    }
}

/// **A stable name as a person reads it**: its words
/// ([`crate::LeafRole`]) with every node and profile step said as the
/// document held them, `the end cap of Extrude "base plate"
/// (3fa9c1d2a0b1)` — [`Speaker::name`]'s sentence, kept.
///
/// Built by [`Doc::spoken_name`] from the document a sentence speaks
/// from, under [`SpokenNode`]'s rule. Boxed, so that a refusal carrying
/// one stays the width of a pointer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpokenName(Box<SpokenNameParts>);

/// [`SpokenName`]'s parts, behind its box: the name, the node that
/// holds it, and every other node and profile step its sentence says.
#[derive(Debug, Clone, PartialEq, Eq)]
struct SpokenNameParts {
    name: StableName,
    minter: SpokenNode,
    held: HeldNodes,
}

impl HoldsNodes for SpokenNameParts {
    fn speak(&self, id: RecipeNodeId) -> SpokenNode {
        if id == self.minter.id() {
            self.minter.clone()
        } else {
            self.held.speak(id)
        }
    }

    fn step(&self, id: StepId) -> Option<StepAt> {
        self.held.step(id)
    }

    fn sole_profile(&self, feature: RecipeNodeId) -> Option<RecipeNodeId> {
        self.held.sole_profile(feature)
    }

    fn boolean_op(&self, id: RecipeNodeId) -> Option<BooleanOp> {
        self.held.boolean_op(id)
    }

    fn speak_var(&self, id: crate::var::VarId) -> Option<crate::doc::VarName> {
        self.held.speak_var(id)
    }
}

impl SpokenName {
    /// A spoken name with no document behind it, for a fixture that
    /// builds by hand what a document would say.
    #[cfg(any(test, feature = "test-support"))]
    pub(crate) fn forged(name: StableName, minter: SpokenNode) -> Self {
        Self::new(name, minter, HeldNodes::default())
    }

    fn new(name: StableName, minter: SpokenNode, held: HeldNodes) -> Self {
        Self(Box::new(SpokenNameParts { name, minter, held }))
    }

    /// A name none of whose nodes a document at hand holds: each said
    /// `node <tag>`, as [`StableName`]'s own `Display` says it.
    #[must_use]
    pub fn absent(name: StableName) -> Self {
        let minter = SpokenNode::absent(name.node);
        Self::new(name, minter, HeldNodes::default())
    }

    /// The name, as the document stores it.
    #[must_use]
    pub fn name(&self) -> &StableName {
        &self.0.name
    }

    /// The node that minted it ([`StableName::node`]), spoken.
    #[must_use]
    pub fn minter(&self) -> &SpokenNode {
        &self.0.minter
    }

    /// **This name with its profile steps said as `doc` draws them**,
    /// its nodes kept as spoken: for a row about the edit that made
    /// `doc`, read with `doc`'s program on screen. A step `doc` still
    /// draws is said where it sits now; one `doc` no longer draws, by
    /// its tag, as [`HeldNodes::respoken`] says it — the row it sat at
    /// may now be another step's, so no sentence mixes the two
    /// programs' rows.
    #[must_use]
    pub fn steps_respoken<P: ProfilePayload>(&self, doc: &Doc<P>) -> Self {
        let held = &self.0.held;
        Self::new(
            self.name().clone(),
            self.minter().clone(),
            HeldNodes {
                facts: held
                    .facts
                    .iter()
                    .filter_map(|fact| match *fact {
                        NodeFact::Step(id, _) => Some(NodeFact::Step(id, doc.step(id)?)),
                        other => Some(other),
                    })
                    .collect(),
                ..held.clone()
            },
        )
    }

    /// This name with its nodes and steps spoken again from `doc`, a
    /// later version of the document it was spoken from
    /// ([`HeldNodes::respoken`]).
    #[must_use]
    pub fn respoken<P: ProfilePayload>(&self, doc: &Doc<P>) -> Self {
        Self::new(
            self.name().clone(),
            self.minter().respoken(doc),
            self.0.held.respoken(doc),
        )
    }
}

impl fmt::Display for SpokenName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let by = Speaker {
            doc: Some(&*self.0),
            kept: None,
            subject: None,
            scope: None,
        };
        write!(f, "{}", by.name(self.name()))
    }
}

impl<P: ProfilePayload> Doc<P> {
    /// The name `name` as a sentence speaks it ([`SpokenName`]): every
    /// node and step its words say, read off this document now.
    #[must_use]
    pub fn spoken_name(&self, name: &StableName) -> SpokenName {
        let recording = Recording {
            doc: self,
            said: core::cell::RefCell::default(),
        };
        let by = Speaker {
            doc: Some(&recording),
            kept: None,
            subject: None,
            scope: None,
        };
        // Said only to be heard: the sentence is the one authority on
        // which nodes and steps it says, so what is kept is what it
        // named.
        let _ = by.name(name).to_string();
        let mut held = recording.held();
        // The minter is held apart, and a node the document does not
        // hold is said by its tag whether held or not: kept out, so a
        // name spoken from a document that holds none of its nodes is
        // the name [`SpokenName::absent`] builds.
        held.nodes = held
            .nodes
            .iter()
            .filter(|node| node.id() != name.node && node.kind().is_some())
            .cloned()
            .collect();
        SpokenName::new(name.clone(), self.spoken(name.node), held)
    }
}

impl<P> Doc<P> {
    /// The node `id` as a sentence speaks it ([`SpokenNode`]), read off
    /// this document now.
    #[must_use]
    pub fn spoken(&self, id: RecipeNodeId) -> SpokenNode {
        match self.node(id) {
            Some(node) => SpokenNode {
                id,
                kind: Some(node_kind_noun(node)),
                label: self.label(id).cloned().map(Box::new),
            },
            None => SpokenNode::absent(id),
        }
    }
}

/// **Who says a sentence's nodes**: a refusal's sentence is written
/// once and said two ways. Its own `Display` says each node by its tag
/// ([`Speaker::TAG`], where no document is at hand), and the frame that
/// holds the document the ids are spelled in says each as that
/// document holds it now ([`Speaker::of`]). A value the evaluation
/// memo reuses, or one a door raised from an evaluation alone, keeps
/// its bare ids and is said this way by the frame that hands it out,
/// unless its memo key fixes the labels too ([`Speaker::held`]).
#[derive(Clone, Copy)]
pub struct Speaker<'a> {
    /// The document each node is read off, `None` for the tag.
    doc: Option<&'a dyn HoldsNodes>,
    /// The nodes a node `doc` does not hold is said as
    /// ([`Speaker::or_held`]).
    kept: Option<&'a HeldNodes>,
    /// The node the enclosing sentence is about ([`Speaker::about`]).
    subject: Option<RecipeNodeId>,
    /// The name tables a name is said within ([`Speaker::within`]),
    /// `None` for the full form.
    scope: Option<&'a dyn NameTables>,
}

/// **The name tables a [`Speaker`] says names within**: a name is said
/// within the table of the node that minted it, as an evaluation answers it.
pub trait NameTables {
    /// The name table of `node`'s output, `None` where there is none.
    fn table(&self, node: RecipeNodeId) -> Option<&NameTable>;
}

impl<T: geom_core::Decide> NameTables for crate::Evaluation<T> {
    fn table(&self, node: RecipeNodeId) -> Option<&NameTable> {
        self.value(node).map(|value| &*value.name_table)
    }
}

/// A document a [`Speaker`] reads nodes and profile steps off.
trait HoldsNodes {
    fn speak(&self, id: RecipeNodeId) -> SpokenNode;
    /// Where the profile step `id` sits, `None` where no profile held
    /// here draws it.
    fn step(&self, id: StepId) -> Option<StepAt>;
    /// The one profile `feature` reads, `None` where it reads none or
    /// several, or is not held here.
    fn sole_profile(&self, feature: RecipeNodeId) -> Option<RecipeNodeId>;
    /// The operation of the Boolean `id`, `None` where `id` is no
    /// Boolean, or is not held here.
    fn boolean_op(&self, id: RecipeNodeId) -> Option<BooleanOp>;
    /// The name the document holds for the variable `id`, if any.
    fn speak_var(&self, id: crate::var::VarId) -> Option<crate::doc::VarName>;
    /// The slot of `node`'s payload at `reference` that holds `name`
    /// ([`Node::reference_slot`]), `None` where it is not held here. A
    /// door's held nodes keep none, so a refusal they say names the
    /// reference by its words.
    fn reference_slot(
        &self,
        _node: RecipeNodeId,
        _reference: usize,
        _name: &StableName,
    ) -> Option<(&'static str, String)> {
        None
    }
}

impl<P: ProfilePayload> HoldsNodes for Doc<P> {
    fn speak(&self, id: RecipeNodeId) -> SpokenNode {
        self.spoken(id)
    }

    fn speak_var(&self, id: crate::var::VarId) -> Option<crate::doc::VarName> {
        self.var_name(id).cloned()
    }

    fn step(&self, id: StepId) -> Option<StepAt> {
        self.order().iter().find_map(|node| {
            let Some(Node::Profile(payload)) = self.node(*node) else {
                return None;
            };
            payload
                .step_ids()?
                .iter()
                .enumerate()
                .find_map(|(loop_, ids)| {
                    ids.iter().position(|step| *step == id).map(|row| StepAt {
                        profile: *node,
                        loop_,
                        row,
                    })
                })
        })
    }

    fn sole_profile(&self, feature: RecipeNodeId) -> Option<RecipeNodeId> {
        let inputs = self.node(feature)?.inputs();
        let mut profiles = inputs
            .iter()
            .filter(|input| matches!(self.node(**input), Some(Node::Profile(_))));
        match (profiles.next(), profiles.next()) {
            (Some(one), None) => Some(*one),
            _ => None,
        }
    }

    fn boolean_op(&self, id: RecipeNodeId) -> Option<BooleanOp> {
        match self.node(id)? {
            Node::Boolean { op, .. } => Some(*op),
            _ => None,
        }
    }

    fn reference_slot(
        &self,
        node: RecipeNodeId,
        reference: usize,
        name: &StableName,
    ) -> Option<(&'static str, String)> {
        self.node(node)?.reference_slot(node, reference, name)
    }
}

/// **Where a profile step sits, as the profile pane numbers it**: its
/// profile, its loop and its row, both from zero — `loop 0 step 2`, the
/// address the pane's rows and every program refusal read. The step's
/// id is shown on no screen, so a sentence says this wherever a
/// document holds the step, and says the profile wherever the feature
/// does not fix it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct StepAt {
    profile: RecipeNodeId,
    loop_: usize,
    row: usize,
}

impl StepAt {
    /// The profile node whose program draws the step.
    pub(crate) fn profile(self) -> RecipeNodeId {
        self.profile
    }
}

impl fmt::Display for StepAt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "loop {} step {}", self.loop_, self.row)
    }
}

/// **The nodes a refusal names, as the document its ids are numbered in
/// held them** ([`held_by`]): kept beside a refusal value carried whole,
/// so it speaks them ([`Speaker::held`]) with no document at hand.
/// Empty, every node is said by its tag.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct HeldNodes {
    nodes: Box<[SpokenNode]>,
    vars: Box<[SpokenVar]>,
    /// What the words read beyond a node's name — where a profile step
    /// sits, a feature's sole profile, a Boolean's operation: one
    /// slice, so a refusal carrying these stays three pointers wide.
    facts: Box<[NodeFact]>,
}

/// One thing a sentence's words read off the document beyond a node's
/// name ([`HeldNodes::facts`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NodeFact {
    /// A profile step, where the document drew it.
    Step(StepId, StepAt),
    /// A feature, with the one profile it read.
    SoleProfile(RecipeNodeId, RecipeNodeId),
    /// A Boolean, with its operation.
    Op(RecipeNodeId, BooleanOp),
}

impl NodeFact {
    /// What the fact is about: one fact is kept per key.
    fn key(self) -> (u8, u64) {
        match self {
            Self::Step(id, _) => (0, id.0),
            Self::SoleProfile(node, _) => (1, node.0),
            Self::Op(node, _) => (2, node.0),
        }
    }

    /// This fact read again from `doc`. A step where `doc` draws it
    /// now, let go where `doc` no longer draws it — then said by its
    /// tag, since the row it sat at may now be another step's. A node's
    /// fact as `doc` holds the node now, or as it was where `doc` does
    /// not hold it; let go where `doc` holds the node and no longer
    /// answers it.
    fn respoken<P: ProfilePayload>(self, doc: &Doc<P>) -> Option<Self> {
        match self {
            Self::Step(id, _) => Some(Self::Step(id, doc.step(id)?)),
            Self::SoleProfile(feature, _) if doc.node(feature).is_some() => {
                Some(Self::SoleProfile(feature, doc.sole_profile(feature)?))
            }
            Self::Op(boolean, _) if doc.node(boolean).is_some() => {
                Some(Self::Op(boolean, doc.boolean_op(boolean)?))
            }
            held => Some(held),
        }
    }
}

impl FromIterator<SpokenNode> for HeldNodes {
    /// The nodes, each as it was spoken; of two for one id, the first.
    fn from_iter<I: IntoIterator<Item = SpokenNode>>(nodes: I) -> Self {
        let mut held: Vec<SpokenNode> = Vec::new();
        for node in nodes {
            if held.iter().all(|kept| kept.id() != node.id()) {
                held.push(node);
            }
        }
        Self {
            nodes: held.into_boxed_slice(),
            vars: Box::default(),
            facts: Box::default(),
        }
    }
}

impl HeldNodes {
    /// Keeps `fact`, once per key.
    fn keep(&mut self, fact: NodeFact) {
        if self.facts.iter().all(|held| held.key() != fact.key()) {
            self.facts = self.facts.iter().copied().chain([fact]).collect();
        }
    }

    /// The node `id` as it was kept, `node <tag>` when it was not.
    #[must_use]
    pub fn spoken(&self, id: RecipeNodeId) -> SpokenNode {
        self.speak(id)
    }

    /// These nodes spoken again from `doc`, a later version of the
    /// document they were held from ([`SpokenNode::respoken`]), and
    /// each fact read again ([`NodeFact::respoken`]).
    #[must_use]
    pub fn respoken<P: ProfilePayload>(&self, doc: &Doc<P>) -> Self {
        Self {
            nodes: self.nodes.iter().map(|node| node.respoken(doc)).collect(),
            vars: self.vars.iter().map(|var| var.respoken(doc)).collect(),
            facts: self
                .facts
                .iter()
                .filter_map(|fact| fact.respoken(doc))
                .collect(),
        }
    }
}

impl HoldsNodes for HeldNodes {
    fn speak(&self, id: RecipeNodeId) -> SpokenNode {
        self.nodes
            .iter()
            .find(|node| node.id() == id)
            .cloned()
            .unwrap_or_else(|| SpokenNode::absent(id))
    }

    fn speak_var(&self, id: crate::var::VarId) -> Option<crate::doc::VarName> {
        self.vars
            .iter()
            .find(|var| var.id() == id)
            .and_then(|var| var.name().cloned())
    }

    fn step(&self, id: StepId) -> Option<StepAt> {
        self.facts.iter().find_map(|fact| match *fact {
            NodeFact::Step(held, at) if held == id => Some(at),
            _ => None,
        })
    }

    fn sole_profile(&self, feature: RecipeNodeId) -> Option<RecipeNodeId> {
        self.facts.iter().find_map(|fact| match *fact {
            NodeFact::SoleProfile(held, profile) if held == feature => Some(profile),
            _ => None,
        })
    }

    fn boolean_op(&self, id: RecipeNodeId) -> Option<BooleanOp> {
        self.facts.iter().find_map(|fact| match *fact {
            NodeFact::Op(held, op) if held == id => Some(op),
            _ => None,
        })
    }
}

/// A document that keeps each node and step it is asked to speak
/// ([`held_by`]).
struct Recording<'d, P> {
    doc: &'d Doc<P>,
    said: core::cell::RefCell<HeldNodes>,
}

impl<P> Recording<'_, P> {
    fn held(self) -> HeldNodes {
        self.said.into_inner()
    }
}

impl<P: ProfilePayload> HoldsNodes for Recording<'_, P> {
    fn speak(&self, id: RecipeNodeId) -> SpokenNode {
        let node = self.doc.spoken(id);
        let mut said = self.said.borrow_mut();
        if said.nodes.iter().all(|held| held.id() != id) {
            said.nodes = said.nodes.iter().cloned().chain([node.clone()]).collect();
        }
        node
    }

    fn speak_var(&self, id: crate::var::VarId) -> Option<crate::doc::VarName> {
        let var = self.doc.spoken_var(id);
        let mut said = self.said.borrow_mut();
        if said.vars.iter().all(|held| held.id() != id) {
            said.vars = said.vars.iter().cloned().chain([var.clone()]).collect();
        }
        var.name().cloned()
    }

    fn step(&self, id: StepId) -> Option<StepAt> {
        let at = self.doc.step(id)?;
        self.said.borrow_mut().keep(NodeFact::Step(id, at));
        // The step's profile is said where the feature does not fix
        // it, so it is kept as a node the words may name.
        let _ = self.speak(at.profile);
        Some(at)
    }

    fn sole_profile(&self, feature: RecipeNodeId) -> Option<RecipeNodeId> {
        let profile = self.doc.sole_profile(feature)?;
        self.said
            .borrow_mut()
            .keep(NodeFact::SoleProfile(feature, profile));
        Some(profile)
    }

    fn boolean_op(&self, id: RecipeNodeId) -> Option<BooleanOp> {
        let op = self.doc.boolean_op(id)?;
        self.said.borrow_mut().keep(NodeFact::Op(id, op));
        Some(op)
    }
}

/// **Every node `value`'s sentence names, as `doc` holds it now**
/// ([`HeldNodes`]). The snapshot is as of now, so the value that keeps
/// it is one no later label of `doc` reaches: a door's refusal, which
/// is never memoized, or a part's fault, whose pin fixes the part.
#[must_use]
pub fn held_by<T: Say + ?Sized, P: ProfilePayload>(value: &T, doc: &Doc<P>) -> HeldNodes {
    let recording = Recording {
        doc,
        said: core::cell::RefCell::default(),
    };
    let speaker = Speaker {
        doc: Some(&recording),
        kept: None,
        subject: None,
        scope: None,
    };
    // The sentence is written only to be heard: the nodes it names are
    // what is kept.
    let _ = Said(value, speaker).to_string();
    recording.held()
}

impl<'a> Speaker<'a> {
    /// Each node by its tag: `node <tag>`.
    pub const TAG: Speaker<'static> = Speaker {
        doc: None,
        kept: None,
        subject: None,
        scope: None,
    };

    /// Each node as `doc` holds it now ([`Doc::spoken`]).
    #[must_use]
    pub fn of<P: ProfilePayload>(doc: &'a Doc<P>) -> Self {
        Self {
            doc: Some(doc),
            kept: None,
            subject: None,
            scope: None,
        }
    }

    /// Each node as `nodes` keeps it ([`held_by`]); a node it did not
    /// keep, by its tag.
    #[must_use]
    pub fn held(nodes: &'a HeldNodes) -> Self {
        Self {
            doc: Some(nodes),
            kept: None,
            subject: None,
            scope: None,
        }
    }

    /// **This speaker, saying each name within its table**: a name
    /// `tables` holds, in the table of the node that minted it, is said
    /// at the detail that table gives it, so that no two of its names
    /// read alike, each said at its own (found greedily, not the fewest
    /// openings, and worked out once per table); any other name, in
    /// full.
    #[must_use]
    pub fn within(self, tables: &'a dyn NameTables) -> Self {
        Self {
            scope: Some(tables),
            ..self
        }
    }

    /// **This speaker, saying a node its document does not hold as
    /// `kept` says it** — the nodes a value was spoken with when it was
    /// made, from an earlier version of the speaker's document
    /// ([`SpokenNode::respoken`]'s rule, and its soundness paragraph).
    /// A node neither holds is said by its tag.
    #[must_use]
    pub fn or_held(self, kept: &'a HeldNodes) -> Self {
        Self {
            kept: Some(kept),
            ..self
        }
    }

    /// The detail this speaker says `name` at ([`Speaker::within`]).
    pub(crate) fn detail_of(self, name: &StableName) -> Detail {
        self.scope
            .and_then(|tables| tables.table(name.node))
            .and_then(|table| table.detail(name))
            .cloned()
            .unwrap_or(Detail::Full)
    }

    /// The operation of the Boolean `id` in this speaker's document,
    /// `None` by tag or where it holds no such Boolean.
    pub(crate) fn boolean_op(self, id: RecipeNodeId) -> Option<BooleanOp> {
        self.doc?.boolean_op(id)
    }

    /// The node `id` as this speaker's document holds it, `noun` said
    /// for its kind (`Subtract 1669` for a Boolean): `this node` when it
    /// is the node the enclosing sentence is about ([`Speaker::about`]),
    /// `node <tag>` where the document does not hold it.
    pub(crate) fn node_as_kind(self, id: RecipeNodeId, noun: &'static str) -> impl fmt::Display {
        let said = (self.subject != Some(id)).then(|| {
            let held = self.spoken(id);
            SpokenNode {
                kind: held.kind.map(|_| noun),
                ..held
            }
        });
        NodeAs("node", said)
    }

    /// **The reference `name` is, as the sentence's subject holds it**:
    /// `this fillet's edge 2`, where the node the enclosing sentence is
    /// about ([`Speaker::about`]) holds `name` at `reference`, its place
    /// among the payload's names ([`crate::Node::payload_names`]), in
    /// this speaker's document; `None` by tag, or where it does not.
    pub(crate) fn reference(self, reference: usize, name: &StableName) -> Option<String> {
        let (owner, slot) = self.doc?.reference_slot(self.subject?, reference, name)?;
        Some(format!("this {owner}'s {slot}"))
    }

    /// The one profile `feature` reads in this speaker's document.
    pub(crate) fn sole_profile(self, feature: RecipeNodeId) -> Option<RecipeNodeId> {
        self.doc?.sole_profile(feature)
    }

    /// **The speaker inside a sentence that has already named `id`**
    /// (`Mate "seat" (3fa9c1d2a0b1) failed: …`): where the inner
    /// sentence names that node by what it is ([`Speaker::node_as`]),
    /// it says `this <noun>` rather than name it a second time.
    #[must_use]
    pub fn about(self, id: RecipeNodeId) -> Self {
        Self {
            subject: Some(id),
            ..self
        }
    }

    /// The node `id`, said: `this node` when it is the node the
    /// enclosing sentence is about ([`Speaker::about`]).
    #[must_use]
    pub fn node(self, id: RecipeNodeId) -> impl fmt::Display + use<> {
        NodeAs("node", (self.subject != Some(id)).then(|| self.spoken(id)))
    }

    /// The node `id` as this speaker's document holds it, its subject
    /// or not.
    fn spoken(self, id: RecipeNodeId) -> SpokenNode {
        let said = match self.doc {
            None => SpokenNode::absent(id),
            Some(doc) => doc.speak(id),
        };
        match self.kept {
            Some(kept) if said.kind().is_none() => kept.speak(id),
            Some(_) | None => said,
        }
    }

    /// **A formula, its readers said**: `expr` unparsed with each
    /// reader written by the name this speaker's document holds for it,
    /// and `#<16 hex>` where it holds none (or the speaker has no
    /// document). A refusal kept by an evaluation memo holds the
    /// formula as an [`crate::Expr`], so a rename — which recomputes
    /// nothing — still reads in the sentence.
    #[must_use]
    pub fn formula<L: crate::expr::LeafSet>(self, expr: &crate::expr::ExprTree<L>) -> String {
        let mut reads = Vec::new();
        expr.var_reads(&mut reads);
        let names: Vec<(crate::var::VarId, crate::doc::VarName)> = match self.doc {
            None => Vec::new(),
            Some(doc) => reads
                .into_iter()
                .filter_map(|(id, _)| doc.speak_var(id).map(|name| (id, name)))
                .collect(),
        };
        crate::expr::unparse(expr, &|id| {
            names
                .iter()
                .find(|(held, _)| *held == id)
                .map(|(_, name)| name)
        })
    }

    /// The name `name` in words ([`crate::LeafRole`]): `the end cap of
    /// Extrude e548, cut in at Subtract 1669`. The one spelling of a name
    /// in a sentence, article-led; [`StableName`]'s own `Display` is
    /// this, said by tag.
    #[must_use]
    pub fn name<'n>(self, name: &'n StableName) -> impl fmt::Display + 'n
    where
        'a: 'n,
    {
        SaidName(name, self)
    }

    /// Where the profile step `id` sits in this speaker's document,
    /// `None` by tag or where the document draws no such step.
    pub(crate) fn step(self, id: StepId) -> Option<StepAt> {
        self.doc?.step(id)
    }

    /// The node `id` where the sentence knows what it is: `<noun>
    /// <tag>` by its tag, or for a node the document does not hold;
    /// as the document holds it otherwise; `this <noun>` when it is
    /// the node the enclosing sentence is about ([`Speaker::about`]).
    #[must_use]
    pub fn node_as(self, id: RecipeNodeId, noun: &'static str) -> impl fmt::Display {
        let said = (self.subject != Some(id)).then(|| self.spoken(id));
        NodeAs(noun, said)
    }
}

/// A name said ([`Speaker::name`]): the one spelling of a name in
/// words ([`crate::LeafRole`]), [`SpokenName`]'s and [`StableName`]'s
/// `Display` too.
struct SaidName<'a>(&'a StableName, Speaker<'a>);

impl fmt::Display for SaidName<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self(name, by) = *self;
        Said(&crate::names::LeafRole(name), by).fmt(f)
    }
}

/// [`Speaker::node`]'s and [`Speaker::node_as`]'s answer: `None` for
/// the sentence's subject.
struct NodeAs(&'static str, Option<SpokenNode>);

impl fmt::Display for NodeAs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.1 {
            None => write!(f, "this {}", self.0),
            Some(node) if node.kind().is_some() => write!(f, "{node}"),
            Some(node) => write!(f, "{} {}", self.0, node.id()),
        }
    }
}

/// A value whose sentence a [`Speaker`] says.
pub trait Say {
    /// The sentence, each node said by `by`.
    ///
    /// # Errors
    ///
    /// The formatter's.
    fn say(&self, f: &mut fmt::Formatter<'_>, by: Speaker<'_>) -> fmt::Result;
}

/// A value said by a speaker: the `Display` of [`Say::say`].
pub struct Said<'a, T: ?Sized>(pub &'a T, pub Speaker<'a>);

impl<T: Say + ?Sized> fmt::Display for Said<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.say(f, self.1)
    }
}

/// `value`'s sentence as `doc` speaks its nodes now.
#[must_use]
pub fn spoken_by<T: Say + ?Sized, P: ProfilePayload>(value: &T, doc: &Doc<P>) -> String {
    Said(value, Speaker::of(doc)).to_string()
}

/// `value`'s sentence as `doc` speaks its nodes now, each name it
/// forwards said within the table `tables` holds it in
/// ([`Speaker::within`]): what a frame holding the evaluation says.
#[must_use]
pub fn spoken_within<T: Say + ?Sized, P: ProfilePayload>(
    value: &T,
    doc: &Doc<P>,
    tables: &dyn NameTables,
) -> String {
    Said(value, Speaker::of(doc).within(tables)).to_string()
}

/// **A report renders only from the document it was taken of.** A
/// node id is not document-scoped: another document can hold the same
/// id as a different node, so speaking a report's ids from it would
/// name nodes the report never measured.
///
/// # Panics
///
/// When `doc` is not the document `taken_of` names.
pub(crate) fn assert_taken_of<P>(what: &str, taken_of: crate::DocumentId, doc: &Doc<P>) {
    assert!(
        doc.id() == taken_of,
        "{what} was taken of document {:032x} and is rendered from document {:032x}; its \
         node ids would name another document's nodes",
        taken_of.0,
        doc.id().0
    );
}

/// **The kind noun of a recipe node** — the word a sentence, a feature
/// tree row, a delete confirmation and a kind census all say.
///
/// **The datum FLAVOURS are named apart** (`Datum plane`, not
/// `Datum`): a plane and a frame are the same surface differing only
/// in whether the spin about the normal is pinned, so a sentence that
/// called both "Datum" would ask a reader to tell them apart by
/// looking.
pub fn node_kind_noun<P, S: crate::Slot>(node: &Node<P, S>) -> &'static str {
    match node {
        Node::Profile(_) => "Profile",
        Node::Extrude { .. } => "Extrude",
        Node::Revolve { .. } => "Revolve",
        Node::Transform { .. } => "Transform",
        Node::Boolean { .. } => "Boolean",
        Node::Union { .. } => "Union",
        Node::Split { .. } => "Split",
        Node::Pattern { .. } => "Pattern",
        Node::Part { .. } => "Part",
        Node::PlacedUnion { .. } => "PlacedUnion",
        Node::Datum(Datum::Plane { .. }) => "Datum plane",
        Node::Datum(Datum::Frame { .. }) => "Datum frame",
        Node::Datum(Datum::FaceFrame { .. }) => "Datum frame (on face)",
        Node::Datum(Datum::AxisInPlane { .. }) => "Datum axis (in sketch)",
        Node::Datum(Datum::Axis { .. }) => "Datum axis",
        Node::Datum(Datum::Point { .. }) => "Datum point",
        Node::Fillet { .. } => "Fillet",
        Node::Chamfer { .. } => "Chamfer",
        Node::Shell { .. } => "Shell",
        Node::Tube { .. } => "Tube",
        Node::HollowTube { .. } => "HollowTube",
        Node::Loft { .. } => "Loft",
        Node::Sweep { .. } => "Sweep",
        Node::InstantiatePart { .. } => "InstantiatePart",
        Node::Gauge { .. } => "Gauge",
        Node::Mate { .. } => "Mate",
        Node::Measure { .. } => "Measure",
        Node::Assertion { .. } => "Assertion",
    }
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use geom_core::Tol;

    use super::{FullId, HeldNodes, Speaker, SpokenNode, node_kind_noun};
    use crate::doc::Doc;
    use crate::edit::DocEdit;
    use crate::label::Label;
    use crate::node::{RecipeNodeId, StepId};
    use crate::program::ProfileProgram;
    use crate::{RefusingReach, test_support};

    /// The tag is the id's high twelve hex digits, its prefix; the full
    /// id is all sixteen. Written as numbers whose digits a reader can
    /// check by eye: the low four digits never reach the tag, and an id
    /// below 2^16 tags as zeros.
    #[test]
    fn the_tag_is_the_twelve_high_hex_digits_and_the_full_id_sixteen() {
        let wide = RecipeNodeId(0x3fa9_c1d2_a0b1_0042);
        assert_eq!(wide.to_string(), "3fa9c1d2a0b1", "the tag is the prefix");
        assert_eq!(wide.full().to_string(), "3fa9c1d2a0b10042");
        assert_eq!(StepId(0x0000_0000_00ab_ffff).to_string(), "0000000000ab");
        assert_eq!(RecipeNodeId(0xffff).to_string(), "000000000000");
        assert_eq!(StepId(7).full(), FullId(7));
        assert_eq!(FullId(7).to_string(), "0000000000000007");
    }

    /// A node the document holds is spoken by its kind noun and tag;
    /// an id it does not hold, as `node` and the tag.
    #[test]
    fn a_held_node_is_its_kind_and_tag_and_an_absent_one_is_a_node() {
        let tol = Tol::witness();
        let empty: Doc<ProfileProgram> = Doc::empty_derived("spoken", tol);
        let node = test_support::xy_frame();
        let kind = node_kind_noun(&node);
        let doc = empty
            .apply(
                &DocEdit::InsertNode {
                    node: Box::new(node),
                },
                tol,
                &RefusingReach,
            )
            .expect("the frame inserts")
            .doc;
        let id = *doc.order().last().expect("the inserted frame");
        assert_eq!(kind, "Datum frame");
        let spoken = doc.spoken(id);
        assert_eq!((spoken.id(), spoken.kind()), (id, Some("Datum frame")));
        assert_eq!(
            spoken.to_string(),
            format!("Datum frame {}", test_utils::refusal::tag(id.0))
        );
        let gone = empty.spoken(id);
        assert_eq!(gone.kind(), None);
        assert_eq!(
            gone.to_string(),
            format!("node {}", test_utils::refusal::tag(id.0))
        );
    }

    /// A speaker that keeps nodes says a node its document holds as the
    /// document does, and one it does not hold as kept; one neither
    /// holds, by its tag.
    #[test]
    fn a_kept_node_is_said_only_where_the_document_does_not_hold_it() {
        let tol = Tol::witness();
        let empty: Doc<ProfileProgram> = Doc::empty_derived("spoken-kept", tol);
        let doc = empty
            .apply(
                &DocEdit::InsertNode {
                    node: Box::new(test_support::xy_frame()),
                },
                tol,
                &RefusingReach,
            )
            .expect("the frame inserts")
            .doc;
        let id = *doc.order().last().expect("the inserted frame");
        let stranger = RecipeNodeId(test_utils::refusal::tagged(7));
        let forged = SpokenNode::forged(
            id,
            Some("Datum frame"),
            Some(Label::new("floor").expect("a label")),
        );
        let kept: HeldNodes = [forged.clone()].into_iter().collect();
        let t = test_utils::refusal::tag(id.0);
        let said = |by: Speaker<'_>, id| by.node(id).to_string();
        assert_eq!(
            said(Speaker::of(&doc).or_held(&kept), id),
            format!("Datum frame {t}"),
            "the document holds it, so the document says it"
        );
        assert_eq!(
            said(Speaker::of(&empty).or_held(&kept), id),
            format!("Datum frame \"floor\" ({t})"),
            "the document does not hold it, so it is said as kept"
        );
        assert_eq!(
            said(Speaker::TAG.or_held(&kept), id),
            forged.to_string(),
            "with no document, as kept"
        );
        assert_eq!(
            said(Speaker::of(&empty).or_held(&kept), stranger),
            format!("node {}", test_utils::refusal::tag(stranger.0)),
            "neither holds it: its tag"
        );
    }
}
