//! **What a node reads and what reads it**, as the property panel lists
//! them: one row per operand seat ([`Doc::operand_reads`]), and one row
//! per node whose operand seat reaches this one ([`ReadIndex::read_by`])
//! — the same reads, inverted, so a link one way has its way back.
//!
//! A body's placements are its readers at the source seat, so a body
//! lists them and a placement lists its source with nothing about
//! placements here. An instance's part is another document: its row
//! names the file, and a click there selects nothing.
//!
//! Module kind: **vocabulary** — it names no driver type and no
//! `app`-only crate (`crates/viewer/README.md`, Module boundaries).

use pncad::document::{
    Doc, Node, OperandRead, OperandSlot, ProfileProgram, ReadIndex, RecipeNodeId,
};

use crate::parts::PartFiles;
use crate::session::Selection;

/// **One entry of a relation row**: what it reads as, and what a click
/// on it selects, `None` for an entry that names something this
/// document does not hold (an instance's part file, the world, a node
/// since deleted).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Related {
    /// What the entry reads as.
    pub said: String,
    /// What a click selects.
    pub selects: Option<Selection>,
}

impl Related {
    /// The node `node`, said as the tree says it, selecting it.
    pub(crate) fn node(doc: &Doc<ProfileProgram>, node: RecipeNodeId, files: &PartFiles) -> Self {
        Self {
            said: crate::tree::node_label(doc, node, files),
            selects: Some(Selection::Node(node)),
        }
    }

    /// The variable `var` by its name, selecting it; `None` for an
    /// unnamed one.
    pub(crate) fn named(doc: &Doc<ProfileProgram>, var: pncad::document::VarId) -> Option<Self> {
        Some(Self {
            said: doc.var_name(var)?.to_string(),
            selects: Some(Selection::Variable(var)),
        })
    }

    /// Words that select nothing.
    pub(crate) fn plain(said: impl Into<String>) -> Self {
        Self {
            said: said.into(),
            selects: None,
        }
    }
}

/// **Where a row reads**: an operand seat, one side of a placement's
/// constraint, or an instance's part.
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
    /// side on`.
    #[must_use]
    pub fn label(self) -> String {
        match self {
            Self::Part => "part".to_owned(),
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
    /// One row per reader and the seat it reads through, readers in
    /// document order. Drawn `seat of: reader`.
    pub read_by: Vec<RelationRow>,
}

/// What an entry reads as where a read reaches nothing because the node
/// it read is gone.
pub const DELETED: &str = "a deleted node";

/// What an entry reads as where an instance's constraint reads its own
/// copy, which is no other node.
pub const OWN_COPY: &str = "this instance's own copy";

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
    let read_by = index
        .read_by(node)
        .iter()
        .filter_map(|&(reader, seat)| {
            Some(RelationRow {
                seat: Seat::of(doc.node(reader)?, seat),
                entries: vec![Related::node(doc, reader, files)],
                through: None,
            })
        })
        .collect();
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
    let mut entries: Vec<Related> = read
        .reaches
        .iter()
        .filter_map(|&at| match at {
            RecipeNodeId::WORLD => Some(Related::plain(doc.spoken_space(read.read).to_string())),
            _ => doc.node(at).map(|_| Related::node(doc, at, files)),
        })
        .collect();
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
