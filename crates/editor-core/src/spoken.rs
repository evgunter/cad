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
//! - [`SpokenNode`] — the node as a person reads it. It is built from
//!   the document that holds the node, by the frame that owns that
//!   document, when the sentence is made ([`crate::Doc::spoken`]); it
//!   is never stored in a value the evaluation memo reuses, so the
//!   label it says is the document's at the moment of speaking, never
//!   one a later rename left stale.
//! - The `Display` of [`RecipeNodeId`] and [`StepId`] — the bare tag,
//!   for a sentence made where no document is at hand (a refusal's own
//!   `Display`, a stored reference). The edit, load and save doors all
//!   hold a document, so each speaks.
//! - [`Speaker`] — a sentence written once over the speaker, for a
//!   refusal that holds bare ids (one the evaluation memo reuses, or
//!   one a door raised from an evaluation alone): its `Display` says
//!   each node by tag ([`Speaker::TAG`]), and its `spoken(doc)` says
//!   each as the document of the frame handing it out holds it
//!   ([`Speaker::of`]). A door that holds the document and carries
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
//! name's table says it at the least detail that tells it apart there
//! ([`Speaker::within`]).

use core::fmt;

use crate::doc::Doc;
use crate::label::Label;
use crate::names::words::{Detail, least_detail};
use crate::names::{NameTable, StableName};
use crate::node::{Datum, Node, RecipeNodeId, StepId};
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
    pub(crate) fn entering<P>(id: RecipeNodeId, node: &Node<P>) -> Self {
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

    /// The node whose output holds it ([`StableName::node`]), spoken.
    #[must_use]
    pub fn minter(&self) -> &SpokenNode {
        &self.0.minter
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
/// its bare ids and is said this way by the frame that hands it out.
#[derive(Clone, Copy)]
pub struct Speaker<'a> {
    /// The document each node is read off, `None` for the tag.
    doc: Option<&'a dyn HoldsNodes>,
    /// The node the enclosing sentence is about ([`Speaker::about`]).
    subject: Option<RecipeNodeId>,
    /// The name tables a name is said within ([`Speaker::within`]),
    /// `None` for the full form.
    scope: Option<&'a dyn NameTables>,
}

/// **The name tables a [`Speaker`] says names within**: the table of
/// the node whose output holds a name, as an evaluation answers it.
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
}

impl<P: ProfilePayload> HoldsNodes for Doc<P> {
    fn speak(&self, id: RecipeNodeId) -> SpokenNode {
        self.spoken(id)
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

/// **The nodes a refusal names, as a door's document held them when the
/// door refused** ([`held_by`]): kept beside a refusal value the door
/// carries whole, so the door's refusal speaks them ([`Speaker::held`])
/// with no document at hand. Empty, every node is said by its tag.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct HeldNodes {
    nodes: Box<[SpokenNode]>,
    /// The profile steps the words name, where the document held them.
    steps: Box<[(StepId, StepAt)]>,
    /// The features the words name, each with the one profile it read.
    profiles: Box<[(RecipeNodeId, RecipeNodeId)]>,
}

impl HeldNodes {
    /// These nodes spoken again from `doc`, a later version of the
    /// document they were held from ([`SpokenNode::respoken`]); a step
    /// `doc` still draws, where it sits there now. A step `doc` no
    /// longer draws is let go, and said by its tag: the row it sat at
    /// may now be another step's.
    #[must_use]
    pub fn respoken<P: ProfilePayload>(&self, doc: &Doc<P>) -> Self {
        Self {
            nodes: self.nodes.iter().map(|node| node.respoken(doc)).collect(),
            steps: self
                .steps
                .iter()
                .filter_map(|&(id, _)| Some((id, doc.step(id)?)))
                .collect(),
            profiles: self
                .profiles
                .iter()
                .filter_map(|&(feature, profile)| match doc.node(feature) {
                    Some(_) => Some((feature, doc.sole_profile(feature)?)),
                    None => Some((feature, profile)),
                })
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

    fn step(&self, id: StepId) -> Option<StepAt> {
        self.steps
            .iter()
            .find_map(|&(step, at)| (step == id).then_some(at))
    }

    fn sole_profile(&self, feature: RecipeNodeId) -> Option<RecipeNodeId> {
        self.profiles
            .iter()
            .find_map(|&(held, profile)| (held == feature).then_some(profile))
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

    fn step(&self, id: StepId) -> Option<StepAt> {
        let at = self.doc.step(id)?;
        let mut said = self.said.borrow_mut();
        if said.steps.iter().all(|(held, _)| *held != id) {
            said.steps = said.steps.iter().copied().chain([(id, at)]).collect();
        }
        drop(said);
        // The step's profile is said where the feature does not fix
        // it, so it is kept as a node the words may name.
        let _ = self.speak(at.profile);
        Some(at)
    }

    fn sole_profile(&self, feature: RecipeNodeId) -> Option<RecipeNodeId> {
        let profile = self.doc.sole_profile(feature)?;
        let mut said = self.said.borrow_mut();
        if said.profiles.iter().all(|(held, _)| *held != feature) {
            said.profiles = said
                .profiles
                .iter()
                .copied()
                .chain([(feature, profile)])
                .collect();
        }
        Some(profile)
    }
}

/// **Every node `value`'s sentence names, as `doc` holds it now**
/// ([`HeldNodes`]). The door's refusal is never memoized, so what it
/// keeps is as of the moment it refused.
#[must_use]
pub fn held_by<T: Say + ?Sized, P: ProfilePayload>(value: &T, doc: &Doc<P>) -> HeldNodes {
    let recording = Recording {
        doc,
        said: core::cell::RefCell::default(),
    };
    let speaker = Speaker {
        doc: Some(&recording),
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
        subject: None,
        scope: None,
    };

    /// Each node as `doc` holds it now ([`Doc::spoken`]).
    #[must_use]
    pub fn of<P: ProfilePayload>(doc: &'a Doc<P>) -> Self {
        Self {
            doc: Some(doc),
            subject: None,
            scope: None,
        }
    }

    /// Each node as a door's document held it when the door refused
    /// ([`held_by`]); a node it did not keep, by its tag.
    #[must_use]
    pub fn held(nodes: &'a HeldNodes) -> Self {
        Self {
            doc: Some(nodes),
            subject: None,
            scope: None,
        }
    }

    /// **This speaker, saying each name within its table**: a name
    /// `tables` holds, in the table of the node whose output holds it,
    /// is said at the least detail no other name of that table reads
    /// alike at; any other name, in full.
    #[must_use]
    pub fn within(self, tables: &'a dyn NameTables) -> Self {
        Self {
            scope: Some(tables),
            ..self
        }
    }

    /// Whether the enclosing sentence is about `id` ([`Speaker::about`]).
    pub(crate) fn is_about(self, id: RecipeNodeId) -> bool {
        self.subject == Some(id)
    }

    /// The detail this speaker says `name` at ([`Speaker::within`]).
    pub(crate) fn detail_of(self, name: &StableName) -> Detail {
        let Some(table) = self.scope.and_then(|tables| tables.table(name.node)) else {
            return Detail::Full;
        };
        if table.lookup(name).is_none() {
            return Detail::Full;
        }
        least_detail(name, self, table)
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
        match self.doc {
            None => SpokenNode::absent(id),
            Some(doc) => doc.speak(id),
        }
    }

    /// The name `name` in words ([`crate::LeafRole`]): `the end cap of
    /// Extrude e548, joined at Boolean 1669`. The one spelling of a name
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

/// **A part's ids are spoken only from the version its reference
/// pins**: the document `doc_ref` names, by its id and its content pin.
/// Another version of the part may hold the same id as another node,
/// or under another label.
///
/// # Panics
///
/// When `part` is not that document at that version, or its pin does
/// not compute.
pub(crate) fn assert_pinned(
    what: &str,
    doc_ref: &crate::ident::DocRef,
    part: &crate::program::ProfileDoc,
    tol: geom_core::Tol,
) {
    assert_taken_of(what, doc_ref.id, part);
    let pin = crate::persist::content_pin(part, tol).ok();
    assert!(
        pin == Some(doc_ref.pin),
        "{what} is in part {:032x} at the version its reference pins, and is rendered from \
         another version of it ({pin:?}); its node ids would name another document's nodes",
        doc_ref.id.0
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
pub fn node_kind_noun<P>(node: &Node<P>) -> &'static str {
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

    use super::{FullId, node_kind_noun};
    use crate::doc::Doc;
    use crate::edit::DocEdit;
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
}
