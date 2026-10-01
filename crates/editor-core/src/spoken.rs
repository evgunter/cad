//! **How a sentence names a recipe node** (DESIGN.md Band 1, "Node
//! labels"): a person reads a node as its kind and its tag,
//! `Extrude 000000000003`; a node the document does not hold reads
//! `node 000000000003`.
//!
//! Three spellings, one home each:
//!
//! - [`SpokenNode`] — the node as a person reads it. It is built from
//!   the document that holds the node, by the frame that owns that
//!   document, when the sentence is made ([`crate::Doc::spoken`]); it
//!   is never stored in a value the evaluation memo reuses, so what it
//!   says is the document's word at the moment of speaking.
//! - The `Display` of [`RecipeNodeId`] and [`StepId`] — the bare tag,
//!   for a sentence made where no document is at hand (a refusal's own
//!   `Display`, a load door reading bytes that are not a document yet).
//! - [`FullId`] — every bit of the id, for a machine channel (a
//!   binding's `repr`, a goldened report) where two ids must never
//!   print alike.

use core::fmt;

use crate::doc::Doc;
use crate::node::{Datum, Node, RecipeNodeId, StepId};

/// How many hex digits a tag shows (`test_utils::refusal::NODE_TAG_DIGITS`
/// mirrors it, so the refusal-shape oracle does not read a tag as a
/// leaked document id).
const TAG_DIGITS: usize = 12;

/// The bits a tag shows: the low [`TAG_DIGITS`] hex digits of the id.
///
/// The LOW bits, because a counter-minted id's high bits are all zero
/// and the low ones are what tell two nodes apart. An id minted as a
/// digest prefix spreads its distinguishing bits from the top, and the
/// tag is then the high [`TAG_DIGITS`] digits instead (the `DocRef`
/// pin prefix's rule); `work/emit/node-labels-are-document-data.md`
/// carries that switch.
const TAG_MASK: u64 = (1 << (4 * TAG_DIGITS)) - 1;

fn write_tag(f: &mut fmt::Formatter<'_>, bits: u64) -> fmt::Result {
    write!(f, "{:0width$x}", bits & TAG_MASK, width = TAG_DIGITS)
}

/// The bare tag: the id's low 48 bits as 12 lowercase hex digits,
/// zero-padded (`000000000003`).
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

/// **A recipe node as a person reads it**: its kind noun and its tag
/// (`Extrude 000000000003`), or `node 000000000003` for an id the
/// document does not hold.
///
/// Built by [`Doc::spoken`] from the document that holds the node.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpokenNode {
    id: RecipeNodeId,
    /// The kind noun, `None` for an id the document does not hold.
    kind: Option<&'static str>,
}

impl SpokenNode {
    /// A spoken node with no document behind it, for a fixture that
    /// builds by hand what a document would say.
    #[cfg(any(test, feature = "test-support"))]
    pub(crate) fn forged(id: RecipeNodeId, kind: Option<&'static str>) -> Self {
        Self { id, kind }
    }

    /// A node no document at hand holds: `node <tag>`, what
    /// [`Doc::spoken`] answers for an id its document does not hold.
    #[must_use]
    pub fn absent(id: RecipeNodeId) -> Self {
        Self { id, kind: None }
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
}

impl fmt::Display for SpokenNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.kind.unwrap_or("node"), self.id)
    }
}

impl<P> Doc<P> {
    /// The node `id` as a sentence speaks it ([`SpokenNode`]), read off
    /// this document now.
    #[must_use]
    pub fn spoken(&self, id: RecipeNodeId) -> SpokenNode {
        SpokenNode {
            id,
            kind: self.node(id).map(node_kind_noun),
        }
    }
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
        Node::Mate { .. } => "Mate",
        Node::Gauge { .. } => "Gauge",
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

    /// The tag is the id's low twelve hex digits, zero-padded; the full
    /// id is all sixteen. Written as numbers whose digits a reader can
    /// check by eye, and one past 48 bits, where the two part.
    #[test]
    fn the_tag_is_twelve_low_hex_digits_and_the_full_id_sixteen() {
        assert_eq!(RecipeNodeId(3).to_string(), "000000000003");
        assert_eq!(RecipeNodeId(0xab).to_string(), "0000000000ab");
        assert_eq!(StepId(0x10).to_string(), "000000000010");
        let wide = RecipeNodeId(0x3fa9_c1d2_a0b1_0042);
        assert_eq!(
            wide.to_string(),
            "c1d2a0b10042",
            "the tag drops the high digits"
        );
        assert_eq!(wide.full().to_string(), "3fa9c1d2a0b10042");
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
        assert_eq!(spoken.to_string(), format!("Datum frame {:012x}", id.0));
        let gone = empty.spoken(id);
        assert_eq!(gone.kind(), None);
        assert_eq!(gone.to_string(), format!("node {:012x}", id.0));
    }
}
