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
    /// The cell holds itself in the result, under its key and under
    /// each copy of it the op cut (a vertex a split copied onto both of
    /// its sides, every copy on the vertex's point).
    Kept {
        /// The copies, the key excluded.
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
pub(crate) enum MintedMeeting {
    /// One cut made both, so they meet where the record puts them: the
    /// landing cites the record alone. The split's one plane.
    OneCut,
    /// The op cannot back the meeting, and the carry refuses it.
    Refused,
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
    meeting: MintedMeeting,
}

impl Default for Lineage {
    fn default() -> Self {
        Self {
            rows: BTreeMap::new(),
            joins: Vec::new(),
            meeting: MintedMeeting::Refused,
        }
    }
}

impl Lineage {
    /// The lineage of an op that kept every cell under its key.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// The lineage with `cell`'s fate stated, replacing any stated
    /// before.
    #[must_use]
    pub fn with(mut self, cell: Cell, fate: Fate) -> Self {
        self.rows.insert(cell, fate);
        self
    }

    /// The lineage read through `joins`, the joins the op ended with:
    /// each image a join killed is the edge it made.
    #[must_use]
    pub fn through_joins(mut self, joins: &[EdgeJoin]) -> Self {
        self.joins.extend_from_slice(joins);
        self
    }

    /// The lineage of an op whose one cut made every pair of cells it
    /// minted on a record's two cells ([`MintedMeeting::OneCut`]).
    #[must_use]
    pub(crate) fn one_cut(mut self) -> Self {
        self.meeting = MintedMeeting::OneCut;
        self
    }

    pub(crate) fn meeting(&self) -> MintedMeeting {
        self.meeting
    }

    /// `cell`'s fate: its row, else kept under its key.
    #[must_use]
    pub fn fate(&self, cell: Cell) -> Fate {
        self.rows
            .get(&cell)
            .cloned()
            .unwrap_or(Fate::Kept { copies: Vec::new() })
    }

    /// Every cell of the result `cell`'s fate names, read through the
    /// joins: the key and its copies, the substitute, or the pieces and
    /// minted cells; none for a removed cell.
    #[must_use]
    pub fn images(&self, cell: Cell) -> Vec<Cell> {
        let mut out: Vec<Cell> = Vec::new();
        for c in self.fate(cell).images(cell) {
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
    /// key and its copies, the substitute, or the pieces and minted
    /// cells.
    pub(crate) fn images(&self, cell: Cell) -> Vec<Cell> {
        match self {
            Self::Kept { copies } => core::iter::once(cell).chain(copies.clone()).collect(),
            Self::Substituted(by) => vec![*by],
            Self::Divided(d) => d.pieces.iter().chain(&d.minted).copied().collect(),
            Self::Removed => Vec::new(),
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
mod tests {
    use super::*;
    use crate::entity::FaceKey;
    use slotmap::SlotMap;

    fn faces(n: usize) -> Vec<FaceKey> {
        let mut arena: SlotMap<FaceKey, ()> = SlotMap::with_key();
        (0..n).map(|_| arena.insert(())).collect()
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
