//! **What an op divided a cell into.** An op that divides a cell keeps
//! its key on one piece and mints the rest, each logged as a division
//! row `(new cell, divided-from cell)`; a key read before the division
//! names the cell it started from. [`Lineage`] is that cell and every
//! piece divided off it, and [`Divisions`] is one op's rows, the form a
//! surgery verb hands its record carry
//! ([`crate::ContactRecords::carried_through_pieces`]).

use std::collections::BTreeSet;

use crate::boolean::Cell;
use crate::entity::{EdgeKey, FaceKey};

/// **A cell's lineage**: the cell and every cell the division rows
/// (`(new cell, divided-from cell)`, in any order) divide off it or off
/// one of its pieces.
pub(crate) struct Lineage<K> {
    root: K,
    cells: BTreeSet<K>,
}

impl<K: Ord + Copy> Lineage<K> {
    /// `root`'s lineage through `rows`.
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
        Self { root, cells }
    }

    /// Whether `cell` is of the lineage.
    pub(crate) fn contains(&self, cell: K) -> bool {
        self.cells.contains(&cell)
    }

    /// The lineage's cells, the root among them, in key order.
    pub(crate) fn cells(&self) -> impl Iterator<Item = K> + '_ {
        self.cells.iter().copied()
    }

    /// The cells divided off the root, in key order.
    pub(crate) fn pieces(&self) -> impl Iterator<Item = K> + '_ {
        self.cells.iter().copied().filter(|&c| c != self.root)
    }
}

/// **One op's division rows**, `(new cell, divided-from cell)`, edges
/// and faces apart: what the op divided each input cell into. A row
/// whose two cells are one is no division and is read as none.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Divisions {
    /// Each piece an edge split minted, with the edge it was split from.
    pub edges: Vec<(EdgeKey, EdgeKey)>,
    /// Each piece a face division minted, with the face it was divided
    /// from.
    pub faces: Vec<(FaceKey, FaceKey)>,
}

impl Divisions {
    /// The cells the rows divide `cell` into, `cell` excluded: none for
    /// a vertex, or for a cell no row divides.
    #[must_use]
    pub fn pieces(&self, cell: Cell) -> Vec<Cell> {
        match cell {
            Cell::Vertex(_) => Vec::new(),
            Cell::Edge(e) => Lineage::of(e, &self.edges)
                .pieces()
                .map(Cell::Edge)
                .collect(),
            Cell::Face(f) => Lineage::of(f, &self.faces)
                .pieces()
                .map(Cell::Face)
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use slotmap::SlotMap;

    fn faces(n: usize) -> Vec<FaceKey> {
        let mut arena: SlotMap<FaceKey, ()> = SlotMap::with_key();
        (0..n).map(|_| arena.insert(())).collect()
    }

    #[test]
    fn a_lineage_follows_rows_listed_after_the_rows_that_read_them() {
        let f = faces(5);
        // f[0] divided into f[1], f[1] into f[2], listed child first;
        // f[3] divided off f[4], a face outside the lineage.
        let rows = [(f[2], f[1]), (f[3], f[4]), (f[1], f[0])];
        let lineage = Lineage::of(f[0], &rows);
        for (i, want) in [true, true, true, false, false].into_iter().enumerate() {
            assert_eq!(lineage.contains(f[i]), want, "face {i} of f0's lineage");
        }
        assert_eq!(
            lineage.pieces().collect::<Vec<_>>().len(),
            2,
            "f0 itself is no piece"
        );
    }
}
