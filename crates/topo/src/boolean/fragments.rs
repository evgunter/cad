//! **Which fragment of a divided face holds a segment.** The join's
//! chords divide faces, and a key read before a division names the face
//! the division started from; [`crate::lineage::Descent`] is that face
//! and every fragment divided off it, through the fragment rows the
//! chord joiner records ([`crate::chord_join::FragmentRows`]). The face
//! holding both ends of a segment is the one face the faces at its two
//! ends share ([`sole_common_face`]).

use crate::entity::FaceKey;

/// The one face in both `xs` and `ys`; `None` when not exactly one is.
pub(super) fn sole_common_face(xs: &[FaceKey], ys: &[FaceKey]) -> Option<FaceKey> {
    match xs.iter().filter(|f| ys.contains(f)).collect::<Vec<_>>()[..] {
        [&f] => Some(f),
        _ => None,
    }
}
