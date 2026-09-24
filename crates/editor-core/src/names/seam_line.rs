//! **Which line a rank lies along.**
//!
//! A `Fragment(OrderAlong)` rank is a place along a direction. For a
//! chain along a SEAM line, the direction is the seam pair's
//! `n_a × n_b`, with the `a` face's outward normal first. That holds for
//! the pieces of a seam minted already cut (the seam-chain ranker), for
//! the pieces of a whole seam a later step cut (the descent ranker), and
//! for a seam-vertex group ranked along a seam edge (the vertex carrier).
//! So one line gets one orientation, whichever step cut it.
//!
//! This module holds the one answer to "is this name on a seam line,
//! and which pair". The pair emitter uses it to pick the direction. The
//! union's collapse uses it to decide whether putting that pair in name
//! order reversed the direction. Because both read the same answer
//! through the same wrappers, they cannot disagree about which ranks
//! lie on a seam line.

use super::role::{EntityKind, RoleSeg, StableName};

/// The seam pair `(a, b)` whose line an EDGE name lies on, if any.
///
/// A seam edge is `Seam { a, b }`. A piece or a pass-through of one
/// keeps it: a boolean survivor (`FromA` / `FromB`), a blend's or
/// shell's survivor (`FromTarget`), a split's fragment
/// (`SplitFragment`), a pattern instance (`Instance`), a part instance
/// (`InPart`), each with any `Fragment` tail after it. A union member's
/// entity (`FromMember`) is NOT seen through: its seam belongs to the
/// member, whose pair order no union reorders, so it ranks along its own
/// carrier. Anything else is not on a seam line.
pub(super) fn seam_line_pair(name: &StableName) -> Option<(&StableName, &StableName)> {
    if name.kind != EntityKind::Edge {
        return None;
    }
    match name.path.first()? {
        RoleSeg::Seam { a, b } => Some((a, b)),
        RoleSeg::FromA(inner) | RoleSeg::FromB(inner) | RoleSeg::FromTarget(inner) => {
            seam_line_pair(inner)
        }
        RoleSeg::SplitFragment { parent, .. } => seam_line_pair(parent),
        RoleSeg::Instance { of, .. } | RoleSeg::InPart { of } => seam_line_pair(of),
        _ => None,
    }
}

/// Whether face name `n` denotes face `x`, or a face descended from it:
/// `x` itself or `x` followed by discriminators, through any number of
/// the pass-through wrappers [`seam_line_pair`] sees through, or a merged
/// face with such a constituent.
fn face_descends_from(n: &StableName, x: &StableName) -> bool {
    if n.kind == x.kind && n.node == x.node && n.path.starts_with(&x.path) {
        return true;
    }
    match n.path.first() {
        Some(
            RoleSeg::FromA(p)
            | RoleSeg::FromB(p)
            | RoleSeg::FromTarget(p)
            | RoleSeg::SplitFragment { parent: p, .. }
            | RoleSeg::Instance { of: p, .. }
            | RoleSeg::InPart { of: p },
        ) => face_descends_from(p, x),
        Some(RoleSeg::Merged(cs)) => cs.iter().any(|c| face_descends_from(c, x)),
        _ => false,
    }
}

/// Which of a seam edge's two faces, named `n0` and `n1`, is the pair's
/// `a` side: `Some(true)` for the first, `Some(false)` for the second,
/// `None` when the names do not settle it one way. A merged face whose
/// constituents come from both sides matches both, so the pairing is
/// read off the assignment that is consistent for BOTH faces.
pub(super) fn a_side_is_first(
    n0: &StableName,
    n1: &StableName,
    a: &StableName,
    b: &StableName,
) -> Option<bool> {
    let first = face_descends_from(n0, a) && face_descends_from(n1, b);
    let second = face_descends_from(n0, b) && face_descends_from(n1, a);
    match (first, second) {
        (true, false) => Some(true),
        (false, true) => Some(false),
        (true, true) | (false, false) => None,
    }
}
