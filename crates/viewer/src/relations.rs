//! **What a node reads and what reads it**, as the property panel lists
//! them: one row per operand seat ([`Doc::operand_reads`]), and one row
//! per node whose operand seat reaches this one ([`ReadIndex::read_by`])
//! — the same reads, inverted, so a link one way has its way back. A
//! variable's definition and the product slot read nodes too
//! ([`ReadIndex::definitions_over`], [`ReadIndex::in_product`]), and a
//! node lists them after its node readers. A named variable lists what
//! its definition reads ([`definition_reads`]), so that link has its
//! way back. The product is no selection, and is words. An unnamed
//! definition is not listed: it is spoken by its reader (VARIABLES
//! VR7), which the node lists at its own seat.
//!
//! A body's placements are its readers at the source seat, so a body
//! lists them and a placement lists its source with nothing about
//! placements here. An instance's part is another document: its row
//! names the file, and a click there selects nothing.
//!
//! Module kind: **vocabulary** — it names no driver type and no
//! `app`-only crate (`crates/viewer/README.md`, Module boundaries).

use pncad::document::{
    Doc, Mention, Mentioned, Node, OperandRead, OperandSlot, ProfileProgram, ReadIndex,
    RecipeNodeId, Said, Sentence, Speaker, VarId,
};

use crate::parts::PartFiles;
use crate::session::Selection;

/// **One entry of a relation row**: a sentence, whose mention — the
/// node or variable it names, stamped with this document — is what a
/// click on it selects. An entry that names something this document
/// does not hold (an instance's part file, the world, a node since
/// deleted) is words, and selects nothing.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Related(Sentence);

impl Related {
    /// The node `node`, said as the tree says it (its spelling, then
    /// which one of its kind it is, [`crate::tree::node_label`]), a
    /// mention in `doc`.
    pub(crate) fn node(doc: &Doc<ProfileProgram>, node: RecipeNodeId, files: &PartFiles) -> Self {
        let which = doc
            .node(node)
            .and_then(|held| crate::tree::part_file(held, files));
        let mut said = pncad::document::sentence!(Speaker::of(doc).mention(&doc.spoken(node)));
        if let Some(which) = which {
            let _ = pncad::document::say!(&mut said, format_args!(" — {which}"));
        }
        Self(said)
    }

    /// The variable `var` by its name, a mention in `doc`; `None` for
    /// an unnamed one.
    pub(crate) fn named(doc: &Doc<ProfileProgram>, var: pncad::document::VarId) -> Option<Self> {
        doc.var_name(var)?;
        Some(Self(pncad::document::sentence!(
            Speaker::of(doc).var(&doc.spoken_var(var))
        )))
    }

    /// Words that select nothing.
    pub(crate) fn plain(said: impl AsRef<str>) -> Self {
        Self(Sentence::from(said.as_ref()))
    }

    /// What the entry reads as.
    #[must_use]
    pub fn said(&self) -> &Sentence {
        &self.0
    }

    /// **What a click selects**: the node or variable the entry
    /// mentions, read off the mention; `None` for words.
    #[must_use]
    pub fn selects(&self) -> Option<Selection> {
        self.0
            .mentions()
            .next()
            .map(|mention| match mention.mentioned() {
                Mentioned::Node(node) => Selection::Node(node.id()),
                Mentioned::Var(var) => Selection::Variable(var.id()),
            })
    }
}

/// **What a click on `mention` selects in `on_screen`**, the document
/// drawn: its node or variable, when the mention's id is spelled in
/// that document and the document holds it; `None` otherwise. An id is
/// not document-scoped, so a mention from another document — an
/// instance's part, a check run on another landing — or of a node since
/// deleted links nothing, and is drawn as words.
#[must_use]
pub fn link(on_screen: &Doc<ProfileProgram>, mention: &Mention) -> Option<Selection> {
    if mention.document() != Some(on_screen.id()) {
        return None;
    }
    match mention.mentioned() {
        Mentioned::Node(node) => on_screen
            .node(node.id())
            .map(|_| Selection::Node(node.id())),
        Mentioned::Var(var) => on_screen
            .var(var.id())
            .map(|_| Selection::Variable(var.id())),
    }
}

/// **Where a row reads**: an operand seat, one side of a placement's
/// constraint, an instance's part, a variable's definition, or the
/// product slot.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Seat {
    /// The part an instance places: another document.
    Part,
    /// An operand seat that is no constraint's.
    Operand(OperandSlot),
    /// A seat of the constraint at 1-based position `at` in its
    /// placement's bundle, as the feature tree numbers it
    /// (`tree::constraint_said`): its minted id is an arena key, no word
    /// a person can find.
    Constraint {
        /// The constraint's position in its bundle, from 1.
        at: usize,
        /// The seat: a side, a direction or an axis.
        slot: OperandSlot,
    },
    /// The definition of the variable: a named source `bracket = B`, a
    /// selection, a pose.
    Definition(VarId),
    /// The product slot, which reads the body the product is.
    Product,
}

impl Seat {
    /// **The seat `slot` of `node`, as a person finds it**: a
    /// constraint's seat by the constraint's position in `node`'s
    /// bundle, any other seat as it is.
    ///
    /// # Panics
    ///
    /// On a constraint seat whose constraint `node`'s bundle does not
    /// hold, which no node's operand rows produce.
    #[must_use]
    pub fn of(node: &Node<ProfileProgram>, slot: OperandSlot) -> Self {
        let constraint = match slot {
            OperandSlot::Side { constraint, .. }
            | OperandSlot::Along { constraint }
            | OperandSlot::About { constraint } => constraint,
            _ => return Self::Operand(slot),
        };
        // A node's constraint seats are read off its own bundle, so the
        // constraint is in it; a seat said without its number would
        // spell its arena key.
        let Some((at, _)) =
            crate::tree::numbered_constraints(node).find(|(_, held)| held.id == constraint)
        else {
            unreachable!("a constraint seat of a node reads a constraint of its own bundle")
        };
        Self::Constraint { at, slot }
    }

    /// The seat as a reader says it: `profile`, `part`, `constraint 2
    /// side on`, `definition`, `body` (the product's).
    #[must_use]
    pub fn label(self) -> String {
        match self {
            Self::Part => "part".to_owned(),
            Self::Definition(_) => "definition".to_owned(),
            Self::Product => "body".to_owned(),
            Self::Constraint {
                at,
                slot: OperandSlot::Side { side, .. },
            } => format!("constraint {at} side {}", side.name()),
            Self::Constraint {
                at,
                slot: OperandSlot::Along { .. },
            } => format!("constraint {at} direction"),
            Self::Constraint {
                at,
                slot: OperandSlot::About { .. },
            } => format!("constraint {at} axis"),
            Self::Operand(slot) | Self::Constraint { slot, .. } => slot.label(),
        }
    }
}

/// **One row**: the seat and what is at it — the nodes a read reaches,
/// or the one reader of this node — and, for a read of a named
/// variable, that variable, which the entries are read through.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RelationRow {
    /// The seat.
    pub seat: Seat,
    /// What is at the seat, in read order.
    pub entries: Vec<Related>,
    /// The named variable the read goes through, if it is one.
    pub through: Option<Related>,
}

/// **A node's relations**: what it reads, seat by seat, and what reads
/// it.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Relations {
    /// One row per operand seat, in field order; an instance's part
    /// first. Drawn `seat: entries`.
    pub reads: Vec<RelationRow>,
    /// One row per reader and the seat it reads through: node readers
    /// in document order, then the named definitions over the node, then
    /// the product. Drawn `seat of: reader`.
    pub read_by: Vec<RelationRow>,
}

/// What an entry reads as where a read reaches nothing because the node
/// it read is gone.
pub const DELETED: &str = "a deleted node";

/// What an entry reads as where an instance's constraint reads its own
/// copy, which is no other node.
pub const OWN_COPY: &str = "this instance's own copy";

/// What the product slot reads as, as a reader: it is no node or
/// variable a click selects, and its control is a button on the body it
/// reads.
pub const THE_PRODUCT: &str = "the product";

/// **What `node` reads and what reads it**, in `doc`, with `index` the
/// document's [`Doc::read_index`]; empty for a node `doc` does not
/// hold.
#[must_use]
pub fn relations(
    doc: &Doc<ProfileProgram>,
    index: &ReadIndex,
    node: RecipeNodeId,
    files: &PartFiles,
) -> Relations {
    let Some(held) = doc.node(node) else {
        return Relations::default();
    };
    let part = crate::tree::part_file(held, files).map(|file| RelationRow {
        seat: Seat::Part,
        entries: vec![Related::plain(file)],
        through: None,
    });
    let seats = doc
        .operand_reads(node)
        .into_iter()
        .map(|read| read_row(doc, held, &read, files));
    let nodes = index.read_by(node).iter().filter_map(|&(reader, seat)| {
        Some(RelationRow {
            seat: Seat::of(doc.node(reader)?, seat),
            entries: vec![Related::node(doc, reader, files)],
            through: None,
        })
    });
    let definitions = index.definitions_over(node).iter().filter_map(|&var| {
        Some(RelationRow {
            seat: Seat::Definition(var),
            entries: vec![Related::named(doc, var)?],
            through: None,
        })
    });
    let product = index.in_product(node).then(|| RelationRow {
        seat: Seat::Product,
        entries: vec![Related::plain(THE_PRODUCT)],
        through: None,
    });
    let read_by = nodes.chain(definitions).chain(product).collect();
    Relations {
        reads: part.into_iter().chain(seats).collect(),
        read_by,
    }
}

/// **One read, as its row says it**: each node it reaches — the world
/// as its name, since it is no node a click selects — through the
/// variable read where that is named, and words for a read that reaches
/// no node.
fn read_row(
    doc: &Doc<ProfileProgram>,
    held: &Node<ProfileProgram>,
    read: &OperandRead,
    files: &PartFiles,
) -> RelationRow {
    let mut entries = reached(doc, read.read, &read.reaches, files);
    if entries.is_empty() {
        entries.push(Related::plain(if read.own_copy {
            OWN_COPY
        } else {
            DELETED
        }));
    }
    RelationRow {
        seat: Seat::of(held, read.seat),
        entries,
        through: Related::named(doc, read.read),
    }
}

/// **What `var`'s definition reads**, in `doc`: one row at
/// [`Seat::Definition`] per variable it reads that reaches a node
/// ([`Doc::definition_reaches`]), the nodes as entries and the variable
/// read after `through` where it is named — the way back from each node
/// listing `var` in its read-by. A read reaching no node (a free value)
/// is no relation between nodes and has no row.
#[must_use]
pub fn definition_reads(
    doc: &Doc<ProfileProgram>,
    var: VarId,
    files: &PartFiles,
) -> Vec<RelationRow> {
    doc.definition_reaches(var)
        .into_iter()
        .filter_map(|read| {
            let entries = reached(doc, read.read, &read.reaches, files);
            (!entries.is_empty()).then(|| RelationRow {
                seat: Seat::Definition(var),
                entries,
                through: Related::named(doc, read.read),
            })
        })
        .collect()
}

/// **The nodes a read of `read` reaches, as entries**: each node `doc`
/// holds, and the world as its name, since it is no node a click
/// selects.
fn reached(
    doc: &Doc<ProfileProgram>,
    read: VarId,
    reaches: &[RecipeNodeId],
    files: &PartFiles,
) -> Vec<Related> {
    reaches
        .iter()
        .filter_map(|&at| match at {
            RecipeNodeId::WORLD => Some(Related::plain(
                Said(&doc.spoken_space(read), Speaker::of(doc))
                    .sentence()
                    .to_string(),
            )),
            _ => doc.node(at).map(|_| Related::node(doc, at, files)),
        })
        .collect()
}
