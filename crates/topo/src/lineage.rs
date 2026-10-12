//! **What an op made of each cell it touched** (D1). An op's
//! [`Lineage`] states, for each input cell it touched, one [`Fate`]:
//! kept, substituted by a cell that holds its interior, divided into
//! pieces and the cells minted between them, or removed. The record
//! carry ([`crate::ContactRecords::carried_through`]) reads it and nothing else,
//! so a removal is stated, never read off a missing image.
//!
//! [`Descent`] is the walk an op's own division rows need: a cell and
//! every piece the rows `(new cell, divided-from cell)` divide off it.

use std::collections::{BTreeMap, BTreeSet};

use crate::boolean::{Cell, EdgeJoin};

/// **What an op made of one input cell.**
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Fate {
    /// The cell holds itself in the result, as `at` (its own key where
    /// the op keeps keys, its key across a graft) and as each copy of it
    /// the op cut (a vertex a split copied onto both of its sides, every
    /// copy on the vertex's point). A kept cell has a result cell, so a
    /// kept cell with none is not a fate.
    Kept {
        /// The cell as the result holds it.
        at: Cell,
        /// The copies, `at` excluded.
        copies: Vec<Cell>,
    },
    /// A cell that holds its interior now: a merge's kept face, a
    /// join's edge, a seam.
    Substituted(Cell),
    /// Divided into pieces and the cells the op minted between them.
    Divided(Division),
    /// Removed, with whatever lay under a touch on it.
    Removed,
}

/// **A cell's division**: its pieces, the cells the op minted between
/// them, and whether the pieces exhaust it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Division {
    /// The pieces, the one that kept the cell's key among them.
    pub pieces: Vec<Cell>,
    /// The cells the op minted in the cell's interior between its
    /// pieces: the vertex that splits an edge, the chord that divides a
    /// face.
    pub minted: Vec<Cell>,
    /// Whether the pieces and minted cells exhaust the cell.
    pub remainder: Remainder,
}

/// Whether a division exhausts its cell.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Remainder {
    /// The pieces and minted cells hold every point of the cell: a
    /// split's.
    Exhaustive,
    /// The op removed the rest: a rim's opening, a blend's carved span,
    /// a boolean's discarded fragments.
    Removed,
}

/// How a carry backs two cells an op minted, one on each of a record's
/// two cells, that meet at a point.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum MintedBacking {
    /// One cut made both, so they meet where the record puts them: the
    /// landing cites the record alone. The split's one plane.
    OneCut,
    /// The op cannot back the meeting, and the carry refuses it.
    Refused,
}

/// Why a [`Lineage`] refuses a statement.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LineageError {
    /// A different fate of `cell` was stated already.
    Restated {
        /// The cell.
        cell: Cell,
    },
    /// A removal stated for `cell` keeps all of it.
    NotARemoval {
        /// The cell.
        cell: Cell,
    },
}

/// **One op's lineage** (D1): the [`Fate`] of each input cell it
/// touched, read through the joins it ended with. A cell no row names
/// is kept under its key; where the result does not hold it there, the
/// lineage does not cover it and the carry refuses
/// ([`crate::CarryRefusal::Unmapped`]).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Lineage {
    rows: BTreeMap<Cell, Fate>,
    joins: Vec<EdgeJoin>,
    meeting: MintedBacking,
    glues: bool,
}

impl Default for Lineage {
    fn default() -> Self {
        Self {
            rows: BTreeMap::new(),
            joins: Vec::new(),
            meeting: MintedBacking::Refused,
            glues: false,
        }
    }
}

impl Lineage {
    /// The lineage of an op that kept every cell under its key.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// The lineage with `cell`'s fate stated.
    ///
    /// # Errors
    ///
    /// [`LineageError::Restated`] where a different fate of `cell` was
    /// stated before: one cell has one fate, and an op that reaches a
    /// cell by two routes says which fate it is
    /// ([`Self::with_removal`] merges two removals).
    pub fn with(mut self, cell: Cell, fate: Fate) -> Result<Self, LineageError> {
        match self.rows.entry(cell) {
            std::collections::btree_map::Entry::Occupied(before) => {
                if *before.get() == fate {
                    Ok(self)
                } else {
                    Err(LineageError::Restated { cell })
                }
            }
            std::collections::btree_map::Entry::Vacant(slot) => {
                slot.insert(fate);
                Ok(self)
            }
        }
    }

    /// The lineage with `cell` removed, whole ([`Fate::Removed`]) or in
    /// part ([`Fate::Divided`] with its remainder removed), merged with
    /// a removal of it stated before: an op that removes part of a cell
    /// by two routes removes the union, its pieces and minted cells
    /// those of both, and a cell removed whole by one route and in part
    /// by another keeps the part's pieces.
    ///
    /// # Errors
    ///
    /// [`LineageError::NotARemoval`] where `fate` keeps all of `cell`,
    /// and [`LineageError::Restated`] where `cell`'s fate was stated
    /// before as one that keeps all of it.
    pub fn with_removal(mut self, cell: Cell, fate: Fate) -> Result<Self, LineageError> {
        if !fate.removes() {
            return Err(LineageError::NotARemoval { cell });
        }
        let merged = match self.rows.remove(&cell) {
            None | Some(Fate::Removed) => fate,
            Some(Fate::Divided(mut d)) if d.remainder == Remainder::Removed => {
                if let Fate::Divided(more) = fate {
                    for p in more.pieces {
                        if !d.pieces.contains(&p) {
                            d.pieces.push(p);
                        }
                    }
                    for m in more.minted {
                        if !d.minted.contains(&m) {
                            d.minted.push(m);
                        }
                    }
                }
                Fate::Divided(d)
            }
            Some(_) => return Err(LineageError::Restated { cell }),
        };
        self.rows.insert(cell, merged);
        Ok(self)
    }

    /// The lineage read through `joins`, the joins the op ended with:
    /// each image a join killed is the edge it made.
    #[must_use]
    pub fn through_joins(mut self, joins: &[EdgeJoin]) -> Self {
        self.joins.extend_from_slice(joins);
        self
    }

    /// The lineage of an op whose one cut made every pair of cells it
    /// minted on a record's two cells ([`MintedBacking::OneCut`]).
    #[must_use]
    pub(crate) fn one_cut(mut self) -> Self {
        self.meeting = MintedBacking::OneCut;
        self
    }

    pub(crate) fn meeting(&self) -> MintedBacking {
        self.meeting
    }

    /// The lineage of an op that glues a record's cells into one cell,
    /// or into a cell and its boundary (the boolean's zip fusions and
    /// joins): a record whose every pair of images is one cell or
    /// incident is consumed, a third way beside a removal and two
    /// vertices made one. D1: "Structural sharing (same surface or point
    /// key) needs no record"; a glued pair shares its cell, or bounds
    /// it, and so is structure.
    #[must_use]
    pub(crate) fn glued(mut self) -> Self {
        self.glues = true;
        self
    }

    pub(crate) fn glues(&self) -> bool {
        self.glues
    }

    /// `cell`'s fate: its row, else kept under its key.
    #[must_use]
    pub fn fate(&self, cell: Cell) -> Fate {
        self.rows.get(&cell).cloned().unwrap_or_else(|| Fate::Kept {
            at: cell,
            copies: Vec::new(),
        })
    }

    /// Every cell of the result `cell`'s fate names, read through the
    /// joins: the cell as kept, the substitute, or the pieces and minted
    /// cells; none for a removed cell.
    #[must_use]
    pub fn images(&self, cell: Cell) -> Vec<Cell> {
        let mut out: Vec<Cell> = Vec::new();
        for c in self.fate(cell).images() {
            let c = self.joined(c);
            if !out.contains(&c) {
                out.push(c);
            }
        }
        out
    }

    /// An image read through the joins.
    pub(crate) fn joined(&self, cell: Cell) -> Cell {
        crate::boolean::joined_cell(&self.joins, cell)
    }
}

impl Fate {
    /// Every cell of the result this fate names, before the joins: the
    /// cell as kept, the substitute, or the pieces and minted cells.
    pub(crate) fn images(&self) -> Vec<Cell> {
        match self {
            Self::Kept { at, copies } => core::iter::once(*at).chain(copies.clone()).collect(),
            Self::Substituted(by) => vec![*by],
            Self::Divided(d) => d.pieces.iter().chain(&d.minted).copied().collect(),
            Self::Removed => Vec::new(),
        }
    }

    /// Every image but the minted cells, before the joins.
    pub(crate) fn pieces(&self) -> Vec<Cell> {
        match self {
            Self::Divided(d) => d.pieces.clone(),
            _ => self.images(),
        }
    }

    /// The cells the op minted in the cell, before the joins.
    pub(crate) fn minted(&self) -> &[Cell] {
        match self {
            Self::Divided(d) => &d.minted,
            _ => &[],
        }
    }

    /// Whether the op removed the cell, or part of it.
    pub(crate) fn removes(&self) -> bool {
        matches!(
            self,
            Self::Removed
                | Self::Divided(Division {
                    remainder: Remainder::Removed,
                    ..
                })
        )
    }
}

/// **A cell's descent**: the cell and every cell the division rows
/// (`(new cell, divided-from cell)`, in any order) divide off it or off
/// one of its pieces.
pub(crate) struct Descent<K> {
    cells: BTreeSet<K>,
}

impl<K: Ord + Copy> Descent<K> {
    /// `root`'s descent through `rows`.
    pub(crate) fn of<'r>(root: K, rows: impl IntoIterator<Item = &'r (K, K)>) -> Self
    where
        K: 'r,
    {
        let rows: Vec<_> = rows.into_iter().collect();
        let mut cells = BTreeSet::from([root]);
        let mut todo = vec![root];
        while let Some(c) = todo.pop() {
            for &&(new, from) in &rows {
                if from == c && cells.insert(new) {
                    todo.push(new);
                }
            }
        }
        Self { cells }
    }

    /// The descent's cells, the root among them, in key order.
    pub(crate) fn cells(&self) -> impl Iterator<Item = K> + '_ {
        self.cells.iter().copied()
    }
}

/// The roots of division rows `(new cell, divided-from cell)`: each
/// divided cell no row divides off another, in key order.
pub(crate) fn roots<K: Ord + Copy>(rows: &[(K, K)]) -> Vec<K> {
    let new: BTreeSet<K> = rows.iter().map(|&(n, _)| n).collect();
    rows.iter()
        .map(|&(_, from)| from)
        .filter(|f| !new.contains(f))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use crate::entity::FaceKey;
    use slotmap::SlotMap;

    fn faces(n: usize) -> Vec<FaceKey> {
        let mut arena: SlotMap<FaceKey, ()> = SlotMap::with_key();
        (0..n).map(|_| arena.insert(())).collect()
    }

    fn removed_part(pieces: &[FaceKey]) -> Fate {
        Fate::Divided(Division {
            pieces: pieces.iter().map(|&f| Cell::Face(f)).collect(),
            minted: Vec::new(),
            remainder: Remainder::Removed,
        })
    }

    /// One cell has one fate: a different second statement refuses, the
    /// same one is the same statement. Red where the second replaces the
    /// first.
    #[test]
    fn a_second_fate_for_a_cell_refuses() {
        let f = faces(2);
        let cell = Cell::Face(f[0]);
        let stated = Lineage::new().with(cell, Fate::Removed).unwrap();
        assert_eq!(
            stated.clone().with(cell, removed_part(&[f[1]])),
            Err(LineageError::Restated { cell }),
            "a different fate refuses"
        );
        assert_eq!(
            stated.clone().with(cell, Fate::Removed),
            Ok(stated),
            "the same fate restated is one statement"
        );
    }

    /// Two removals of one cell merge to the same fate in either order:
    /// a part removal keeps its pieces over a whole removal, and two part
    /// removals keep both's pieces. A removal over a fate that keeps the
    /// cell refuses, as does a "removal" that keeps it. Red where the
    /// later statement wins.
    #[test]
    fn removals_of_one_cell_merge_in_either_order() {
        let f = faces(3);
        let cell = Cell::Face(f[0]);
        let merged = |first: Fate, second: Fate| {
            Lineage::new()
                .with_removal(cell, first)
                .and_then(|l| l.with_removal(cell, second))
                .map(|l| l.fate(cell))
        };
        for (first, second) in [
            (Fate::Removed, removed_part(&[f[1]])),
            (removed_part(&[f[1]]), Fate::Removed),
        ] {
            assert_eq!(
                merged(first, second),
                Ok(removed_part(&[f[1]])),
                "whole and part"
            );
        }
        assert_eq!(
            merged(removed_part(&[f[1]]), removed_part(&[f[2]])),
            Ok(removed_part(&[f[1], f[2]])),
            "two parts"
        );
        let kept = Fate::Substituted(Cell::Face(f[1]));
        assert_eq!(
            Lineage::new()
                .with(cell, kept.clone())
                .and_then(|l| l.with_removal(cell, Fate::Removed)),
            Err(LineageError::Restated { cell }),
            "a removal over a kept cell"
        );
        assert_eq!(
            Lineage::new().with_removal(cell, kept),
            Err(LineageError::NotARemoval { cell }),
            "a removal that keeps the cell"
        );
    }

    #[test]
    fn a_descent_follows_rows_listed_after_the_rows_that_read_them() {
        let f = faces(5);
        // f[0] divided into f[1], f[1] into f[2], listed child first;
        // f[3] divided off f[4], a face outside the descent.
        let rows = [(f[2], f[1]), (f[3], f[4]), (f[1], f[0])];
        let descent = Descent::of(f[0], &rows);
        for (i, want) in [true, true, true, false, false].into_iter().enumerate() {
            assert_eq!(
                descent.cells().any(|g| g == f[i]),
                want,
                "face {i} of f0's descent"
            );
        }
        let mut want = vec![f[0], f[4]];
        want.sort();
        assert_eq!(roots(&rows), want, "f1 is divided off f0, so no root");
    }
}
