//! **What a node reads and what reads it**, as the property panel lists
//! them: one row per operand seat ([`Node::operand_rows`], named by
//! [`OperandSlot::label`]), and one row per reader of the node's
//! outputs ([`Doc::read_by`]), each entry a thing a click selects.
//!
//! A body's placements are its readers at the source seat, so a body
//! lists them and a placement lists its source with nothing about
//! placements here. An instance's part is another document: its row
//! names the file, and a click there selects nothing.

use pncad::document::{Doc, OperandSlot, ProfileProgram, RecipeNodeId, VarId};

use crate::parts::PartFiles;
use crate::session::Selection;

/// **One entry of a relation row**: what it reads as, and what a click
/// on it selects, `None` for an entry that names something this
/// document does not hold (an instance's part file, the world).
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
    pub(crate) fn named(doc: &Doc<ProfileProgram>, var: VarId) -> Option<Self> {
        Some(Self {
            said: doc.var_name(var)?.to_string(),
            selects: Some(Selection::Variable(var)),
        })
    }

    /// Words that select nothing.
    pub(crate) fn plain(said: String) -> Self {
        Self {
            said,
            selects: None,
        }
    }
}

/// **One row**: the seat (`profile`, `from`, `source of`) and what is
/// read, or reads, through it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RelationRow {
    /// The seat, as a reader says it.
    pub seat: String,
    /// What is at the seat, in read order.
    pub entries: Vec<Related>,
}

/// **A node's relations**: what it reads, seat by seat, and what reads
/// it.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Relations {
    /// One row per operand seat, in field order; an instance's part
    /// first.
    pub reads: Vec<RelationRow>,
    /// One row per reader and the seat it reads through, in document
    /// order.
    pub read_by: Vec<RelationRow>,
}

/// The seat an instance's part file is named at.
pub const PART_SEAT: &str = "part";

/// **What `node` reads and what reads it**, in `doc`; empty for a node
/// `doc` does not hold.
#[must_use]
pub fn relations(doc: &Doc<ProfileProgram>, node: RecipeNodeId, files: &PartFiles) -> Relations {
    let Some(held) = doc.node(node) else {
        return Relations::default();
    };
    let part = crate::tree::part_file(held, files).map(|file| RelationRow {
        seat: PART_SEAT.to_owned(),
        entries: vec![Related::plain(file)],
    });
    let spaces = doc.spaces_read(held);
    let seats = held
        .operand_rows()
        .into_iter()
        .map(|(seat, var)| RelationRow {
            seat: seat.label(),
            entries: read_at(doc, &spaces, seat, var, files),
        });
    let read_by = doc
        .read_by(node)
        .into_iter()
        .map(|(reader, seat)| RelationRow {
            seat: format!("{} of", seat.label()),
            entries: vec![Related::node(doc, reader, files)],
        })
        .collect();
    Relations {
        reads: part.into_iter().chain(seats).collect(),
        read_by,
    }
}

/// **What the read `var` at `seat` is**: the variable, where it is
/// named, then each node the read reaches ([`Doc::read_ports`]: an
/// output's operation, a source's body, a selection's body) — or the
/// space's definer, for a space read, which reaches no operation.
fn read_at(
    doc: &Doc<ProfileProgram>,
    spaces: &[(OperandSlot, RecipeNodeId)],
    seat: OperandSlot,
    var: VarId,
    files: &PartFiles,
) -> Vec<Related> {
    let mut entries: Vec<Related> = Vec::new();
    entries.extend(Related::named(doc, var));
    let mut reached: Vec<RecipeNodeId> = Vec::new();
    for (at, _) in doc.read_ports(var) {
        if !reached.contains(&at) {
            reached.push(at);
        }
    }
    entries.extend(reached.into_iter().map(|at| Related::node(doc, at, files)));
    if entries.is_empty() {
        let space = spaces
            .iter()
            .find(|&&(at, _)| at == seat)
            .map(|&(_, by)| by);
        entries.push(match space {
            Some(by) if by != RecipeNodeId::WORLD && doc.node(by).is_some() => {
                Related::node(doc, by, files)
            }
            Some(_) => Related::plain(doc.spoken_space(var).to_string()),
            None => Related::plain(doc.spoken_var(var).to_string()),
        });
    }
    entries
}
