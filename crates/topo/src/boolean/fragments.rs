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

/// **A face's lineage**: the face and every face the fragment rows
/// (`(new face, divided-from face)`, in any order) divide off it or off
/// one of its fragments: the faces a key read before those divisions
/// may now name.
pub(super) struct Lineage(BTreeSet<FaceKey>);

impl Lineage {
    /// `face`'s lineage through `rows`.
    pub(super) fn of<'r>(
        face: FaceKey,
        rows: impl IntoIterator<Item = &'r (FaceKey, FaceKey)>,
    ) -> Self {
        let rows: Vec<_> = rows.into_iter().collect();
        let mut faces = BTreeSet::from([face]);
        let mut todo = vec![face];
        while let Some(f) = todo.pop() {
            for &&(new, from) in &rows {
                if from == f && faces.insert(new) {
                    todo.push(new);
                }
            }
        }
        Self(faces)
    }

    /// Whether `face` is of the lineage.
    pub(super) fn contains(&self, face: FaceKey) -> bool {
        self.0.contains(&face)
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
