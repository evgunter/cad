//! **How a sentence names a recipe node** (DESIGN.md Band 1, "Node
//! labels"): a person reads a node as its kind, its label and its tag,
//! `Extrude "base plate" (3fa9c1d2a0b1)`, or as its kind and tag when
//! it has no label, `Extrude 3fa9c1d2a0b1`; a node the document does
//! not hold reads `node 3fa9c1d2a0b1`.
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
//! so its own `Display` says its minting node by tag; a sentence made
//! where the document is at hand says it as a [`SpokenName`].

use core::fmt;

use crate::doc::Doc;
use crate::label::Label;
use crate::names::StableName;
use crate::node::{Datum, Node, RecipeNodeId, StepId};

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
/// `node 3fa9c1d2a0b1` for an id the document does not hold.
///
/// Built by [`Doc::spoken`] from the document that holds the node.
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
    /// document did not hold the node.
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

/// **A stable name as a person reads it**: its entity noun and its
/// minting node spoken ([`SpokenNode`]), `face name minted by Extrude
/// "base plate" (3fa9c1d2a0b1)`.
///
/// Built by [`Doc::spoken_name`] from the document a sentence speaks
/// from, under [`SpokenNode`]'s rule. Boxed, so that a refusal carrying
/// one stays the width of a pointer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpokenName(Box<SpokenNameParts>);

/// [`SpokenName`]'s two halves, behind its box.
#[derive(Debug, Clone, PartialEq, Eq)]
struct SpokenNameParts {
    name: StableName,
    minter: SpokenNode,
}

impl SpokenName {
    /// A spoken name with no document behind it, for a fixture that
    /// builds by hand what a document would say.
    #[cfg(any(test, feature = "test-support"))]
    pub(crate) fn forged(name: StableName, minter: SpokenNode) -> Self {
        Self::new(name, minter)
    }

    fn new(name: StableName, minter: SpokenNode) -> Self {
        Self(Box::new(SpokenNameParts { name, minter }))
    }

    /// A name whose minting node no document at hand holds: `node
    /// <tag>`, as [`StableName`]'s own `Display` says it.
    #[must_use]
    pub fn absent(name: StableName) -> Self {
        let minter = SpokenNode::absent(name.node);
        Self::new(name, minter)
    }

    /// The name, as the document stores it.
    #[must_use]
    pub fn name(&self) -> &StableName {
        &self.0.name
    }

    /// The node that minted it, spoken.
    #[must_use]
    pub fn minter(&self) -> &SpokenNode {
        &self.0.minter
    }
}

impl fmt::Display for SpokenName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", SaidName(self.name(), self.minter()))
    }
}

impl<P> Doc<P> {
    /// The name `name` as a sentence speaks it ([`SpokenName`]), its
    /// minting node read off this document now.
    #[must_use]
    pub fn spoken_name(&self, name: &StableName) -> SpokenName {
        SpokenName::new(name.clone(), self.spoken(name.node))
    }

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
}

/// A document a [`Speaker`] reads nodes off, whatever its program.
trait HoldsNodes {
    fn speak(&self, id: RecipeNodeId) -> SpokenNode;
}

impl<P> HoldsNodes for Doc<P> {
    fn speak(&self, id: RecipeNodeId) -> SpokenNode {
        self.spoken(id)
    }
}

/// **The nodes a refusal names, as a door's document held them when the
/// door refused** ([`held_by`]): kept beside a refusal value the door
/// carries whole, so the door's refusal speaks them ([`Speaker::held`])
/// with no document at hand. Empty, every node is said by its tag.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct HeldNodes(Box<[SpokenNode]>);

impl HoldsNodes for HeldNodes {
    fn speak(&self, id: RecipeNodeId) -> SpokenNode {
        self.0
            .iter()
            .find(|node| node.id() == id)
            .cloned()
            .unwrap_or_else(|| SpokenNode::absent(id))
    }
}

/// A document that keeps each node it is asked to speak ([`held_by`]).
struct Recording<'d, P> {
    doc: &'d Doc<P>,
    said: core::cell::RefCell<Vec<SpokenNode>>,
}

impl<P> HoldsNodes for Recording<'_, P> {
    fn speak(&self, id: RecipeNodeId) -> SpokenNode {
        let node = self.doc.spoken(id);
        let mut said = self.said.borrow_mut();
        if said.iter().all(|held| held.id() != id) {
            said.push(node.clone());
        }
        node
    }
}

/// **Every node `value`'s sentence names, as `doc` holds it now**
/// ([`HeldNodes`]). The door's refusal is never memoized, so what it
/// keeps is as of the moment it refused.
#[must_use]
pub fn held_by<T: Say + ?Sized, P>(value: &T, doc: &Doc<P>) -> HeldNodes {
    let recording = Recording {
        doc,
        said: core::cell::RefCell::new(Vec::new()),
    };
    let speaker = Speaker {
        doc: Some(&recording),
        subject: None,
    };
    // The sentence is written only to be heard: the nodes it names are
    // what is kept.
    let _ = Said(value, speaker).to_string();
    HeldNodes(recording.said.into_inner().into_boxed_slice())
}

impl<'a> Speaker<'a> {
    /// Each node by its tag: `node <tag>`.
    pub const TAG: Speaker<'static> = Speaker {
        doc: None,
        subject: None,
    };

    /// Each node as `doc` holds it now ([`Doc::spoken`]).
    #[must_use]
    pub fn of<P>(doc: &'a Doc<P>) -> Self {
        Self {
            doc: Some(doc),
            subject: None,
        }
    }

    /// Each node as a door's document held it when the door refused
    /// ([`held_by`]); a node it did not keep, by its tag.
    #[must_use]
    pub fn held(nodes: &'a HeldNodes) -> Self {
        Self {
            doc: Some(nodes),
            subject: None,
        }
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

    /// The name `name`, its minting node said: `face name minted by
    /// <node>`.
    #[must_use]
    pub fn name(self, name: &StableName) -> impl fmt::Display + '_ {
        SaidName(name, self.node(name.node))
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

/// A name and its minting node said: the one spelling of `<kind> name
/// minted by <node>` ([`SpokenName`]'s too).
struct SaidName<'a, N>(&'a StableName, N);

impl<N: fmt::Display> fmt::Display for SaidName<'_, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} name minted by {}", self.0.kind.noun(), self.1)
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
pub fn spoken_by<T: Say + ?Sized, P>(value: &T, doc: &Doc<P>) -> String {
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
        Node::Declare { .. } => "Declare",
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
