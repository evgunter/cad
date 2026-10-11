//! **Which fragment of a divided face holds a segment.** The join's
//! chords divide faces, and a key read before a division names the face
//! the division started from; [`Lineage`] is that face and every
//! fragment divided off it, through the fragment rows the chord joiner
//! records ([`crate::chord_join::FragmentRows`]). The face holding both
//! ends of a segment is the one face the faces at its two ends share
//! ([`sole_common_face`]), among a lineage where a division may have
//! moved one ([`Lineage::holding_both`]).

use std::collections::BTreeSet;

use crate::entity::FaceKey;

/// **A cell's lineage**: the cell and every cell the division rows
/// (`(new cell, divided-from cell)`, in any order) divide off it or off
/// one of its pieces: the cells a key read before those divisions may
/// now name. A face's rows are the chord joiner's fragment rows; an
/// edge's, the edge splits an op logs.
pub(crate) struct Lineage<K = FaceKey>(BTreeSet<K>);

impl<K: Ord + Copy> Lineage<K> {
    /// `cell`'s lineage through `rows`.
    pub(crate) fn of<'r>(cell: K, rows: impl IntoIterator<Item = &'r (K, K)>) -> Self
    where
        K: 'r,
    {
        let rows: Vec<_> = rows.into_iter().collect();
        let mut cells = BTreeSet::from([cell]);
        let mut todo = vec![cell];
        while let Some(f) = todo.pop() {
            for &&(new, from) in &rows {
                if from == f && cells.insert(new) {
                    todo.push(new);
                }
            }
        }
        Self(cells)
    }

    /// Whether `cell` is of the lineage.
    pub(crate) fn contains(&self, cell: K) -> bool {
        self.0.contains(&cell)
    }

    /// The lineage's cells, the root among them, in key order.
    pub(crate) fn cells(&self) -> impl Iterator<Item = K> + '_ {
        self.0.iter().copied()
    }

    /// The cells divided off the root, the root excluded, in key order.
    pub(crate) fn pieces(&self, root: K) -> impl Iterator<Item = K> + '_ {
        self.0.iter().copied().filter(move |&c| c != root)
    }
}

/// The one face in both `xs` and `ys`; `None` when not exactly one is.
pub(super) fn sole_common_face(xs: &[FaceKey], ys: &[FaceKey]) -> Option<FaceKey> {
    match xs.iter().filter(|f| ys.contains(f)).collect::<Vec<_>>()[..] {
        [&f] => Some(f),
        _ => None,
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
    }
}
