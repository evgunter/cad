//! **Which seam pair a name lies on.**
//!
//! A seam vertex group's `Fragment(OrderAlong)` rank is a place along
//! the edge it crosses. Where that edge lies on a SEAM line, it runs as
//! the loop of the pair's first side runs along it (N2), whichever step
//! cut it, so a rank needs the pair the edge's name records.
//!
//! This module holds the one answer to "is this name on a seam line,
//! and which pair", for the readers that know a seam only by its NAME.
//! The pair emitter reads it to orient a crossed edge
//! (`emit_topo::crossed_edge_orientation`); the canonical form
//! (`names::canonical`) reads it, in a name's earlier and later
//! spelling, to decide whether a rewrite — the union's collapse, or a
//! re-map of a published name — reversed that orientation. Both read the
//! same answer through the same wrappers, so they cannot disagree about
//! which ranks lie on a seam line.

use super::role::{EntityKind, Qualifier};
use super::role::{RoleSeg, StableName, name_free_seg};

/// What a name's HEAD segment means to a walk looking for a seam pair.
/// ONE exhaustive match over [`RoleSeg`]: a new segment variant is a
/// compile error here until someone says whether it passes an entity
/// through.
enum Head<'a> {
    /// A seam: the pair itself.
    Seam(&'a StableName, &'a StableName),
    /// The same entity carried through one op, or a piece of it: the
    /// argument is its name one level down.
    Through(&'a StableName),
    /// A merged face: its constituents.
    Merged(&'a [StableName]),
    /// Anything else: not an entity carried through.
    Stop,
}

fn head(seg: &RoleSeg) -> Head<'_> {
    match seg {
        RoleSeg::Seam { a, b } => Head::Seam(a, b),
        // A boolean's survivor, a blend's or shell's survivor, a split's
        // fragment, a pattern's or part's instance: the same entity (or
        // a piece of it) under one more op.
        RoleSeg::FromA(n)
        | RoleSeg::FromB(n)
        | RoleSeg::FromTarget(n)
        | RoleSeg::SplitFragment { parent: n, .. }
        | RoleSeg::Instance { of: n, .. }
        | RoleSeg::InPart { of: n } => Head::Through(n),
        RoleSeg::Merged(set) => Head::Merged(set),
        // A union member's entity is NOT seen through: its seam belongs
        // to the member, whose pair order no union reorders, and is read
        // in the member's own table.
        RoleSeg::FromMember { .. }
        // New entities an op minted FROM a source — a blend face, a
        // shell's cavity twin (an offset line, not the source's), a
        // trimline, a crossing vertex — lie on no seam line of their
        // source.
        | RoleSeg::BlendFace(_)
        | RoleSeg::CornerFace(_)
        | RoleSeg::TrimEdge { .. }
        | RoleSeg::FootVertex { .. }
        | RoleSeg::EndArc { .. }
        | RoleSeg::BandFace(_)
        | RoleSeg::BandTrim { .. }
        | RoleSeg::BandFoot(_)
        | RoleSeg::BandCross { .. }
        | RoleSeg::BandCut(_)
        | RoleSeg::BandSlit { .. }
        | RoleSeg::Inner(_)
        | RoleSeg::Rim(_)
        | RoleSeg::HoleRim { .. }
        | RoleSeg::SectionEdge { .. }
        | RoleSeg::CrossingVertex { .. }
        | RoleSeg::OnToolVertex { .. }
        | RoleSeg::Fragment(
            Qualifier::Borders(_) | Qualifier::Keeps(_) | Qualifier::Ends(_) | Qualifier::OrderAlong { .. },
        )
        | name_free_seg!() => Head::Stop,
    }
}

/// The seam pair `(a, b)` whose line an EDGE name lies on, if any.
///
/// A seam edge is `Seam { a, b }`; a piece or pass-through of one keeps
/// it through the wrappers [`head`] lists as `Through`, with any
/// `Fragment` tail after it. A pair whose two sides carry the SAME name
/// (two placements of one prototype, N1) names no side, so it is not a
/// sided line: it is answered `None` — the union's collapse, which never
/// swaps an equal pair, agrees.
pub(super) fn seam_line_pair(name: &StableName) -> Option<(&StableName, &StableName)> {
    seam_through(name, EntityKind::Edge).filter(|(a, b)| a != b)
}

/// The two sides `(a, b)` of the seam an EDGE name lies on, if any,
/// found as [`seam_line_pair`] finds them, an equal pair included.
pub(super) fn seam_edge_sides(name: &StableName) -> Option<(&StableName, &StableName)> {
    seam_through(name, EntityKind::Edge)
}

/// The two parents `(a, b)` of a seam VERTEX name, if it is one: a
/// vertex minted as `Seam { a, b }`, or a pass-through of one, found
/// the way [`seam_line_pair`] finds an edge's pair. A junction (a run
/// of lines) answers its first line; it carries no rank.
pub(super) fn seam_vertex_parents(name: &StableName) -> Option<(&StableName, &StableName)> {
    seam_through(name, EntityKind::Vertex)
}

/// The `Seam` a `kind` name is minted as, through the wrappers [`head`]
/// lists as `Through`: the one walk both answers above take.
fn seam_through(name: &StableName, kind: EntityKind) -> Option<(&StableName, &StableName)> {
    let mut at = name;
    loop {
        if at.kind != kind {
            return None;
        }
        match head(at.path.first()?) {
            Head::Seam(a, b) => return Some((a, b)),
            Head::Merged(_) | Head::Stop => return None,
            Head::Through(inner) => at = inner,
        }
    }
}

/// Whether face name `n` denotes face `x`, or a face descended from it:
/// `x` itself or `x` followed by discriminators, through any number of
/// the wrappers [`head`] passes through, or a merged face with such a
/// constituent.
pub(crate) fn face_descends_from(n: &StableName, x: &StableName) -> bool {
    let mut names = vec![n];
    while let Some(n) = names.pop() {
        if n.kind == x.kind && n.node == x.node && n.path.starts_with(&x.path) {
            return true;
        }
        match n.path.first().map(head) {
            Some(Head::Through(p)) => names.push(p),
            Some(Head::Merged(cs)) => names.extend(cs.iter().rev()),
            Some(Head::Seam(..) | Head::Stop) | None => {}
        }
    }
    false
}

/// Which of a seam edge's two faces, named `n0` and `n1`, is the pair's
/// `a` side: `Some(true)` for the first, `Some(false)` for the second.
/// `None` when the names do not settle it one way — when neither face
/// descends from a side, or when both assignments fit (a merged face
/// whose constituents come from both sides, matched against a face that
/// does too). A merged face on ONE face is resolved by the other face:
/// the assignment consistent for BOTH faces wins.
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

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use super::*;
    use crate::names::role::CapEnd;
    use crate::node::RecipeNodeId;

    fn cap(node: u64, end: CapEnd) -> StableName {
        StableName {
            kind: EntityKind::Face,
            node: RecipeNodeId(node),
            path: vec![RoleSeg::Cap(end)],
        }
    }

    fn wrap(seg: fn(crate::names::role::NameRef) -> RoleSeg, inner: StableName) -> StableName {
        StableName {
            kind: inner.kind,
            node: RecipeNodeId(9),
            path: vec![seg(inner.into())],
        }
    }

    fn merged(cs: Vec<StableName>) -> StableName {
        StableName {
            kind: EntityKind::Face,
            node: RecipeNodeId(9),
            path: vec![RoleSeg::Merged(cs)],
        }
    }

    #[test]
    fn the_sides_are_read_off_the_faces_descent() {
        let (a, b) = (cap(1, CapEnd::End), cap(2, CapEnd::Start));
        let (fa, fb) = (
            wrap(RoleSeg::FromA, a.clone()),
            wrap(RoleSeg::FromB, b.clone()),
        );
        assert_eq!(a_side_is_first(&fa, &fb, &a, &b), Some(true));
        assert_eq!(a_side_is_first(&fb, &fa, &a, &b), Some(false));
    }

    #[test]
    fn a_face_descending_from_neither_side_decides_nothing() {
        let (a, b) = (cap(1, CapEnd::End), cap(2, CapEnd::Start));
        let other = wrap(RoleSeg::FromA, cap(3, CapEnd::End));
        let fb = wrap(RoleSeg::FromB, b.clone());
        assert_eq!(a_side_is_first(&other, &fb, &a, &b), None);
    }

    #[test]
    fn two_faces_each_merged_across_both_sides_decide_nothing() {
        let (a, b) = (cap(1, CapEnd::End), cap(2, CapEnd::Start));
        let both = merged(vec![
            wrap(RoleSeg::FromA, a.clone()),
            wrap(RoleSeg::FromB, b.clone()),
        ]);
        assert_eq!(a_side_is_first(&both, &both, &a, &b), None);
    }

    #[test]
    fn a_face_merged_across_both_sides_is_resolved_by_the_other_face() {
        let (a, b) = (cap(1, CapEnd::End), cap(2, CapEnd::Start));
        let both = merged(vec![
            wrap(RoleSeg::FromA, a.clone()),
            wrap(RoleSeg::FromB, b.clone()),
        ]);
        let fb = wrap(RoleSeg::FromB, b.clone());
        assert_eq!(a_side_is_first(&both, &fb, &a, &b), Some(true));
        assert_eq!(a_side_is_first(&fb, &both, &a, &b), Some(false));
    }

    #[test]
    fn a_split_fragment_is_seen_through() {
        let (a, b) = (cap(1, CapEnd::End), cap(2, CapEnd::Start));
        let frag = StableName {
            kind: EntityKind::Face,
            node: RecipeNodeId(11),
            path: vec![RoleSeg::SplitFragment {
                side: crate::names::role::SplitHalf::Below,
                parent: wrap(RoleSeg::FromA, a.clone()).into(),
            }],
        };
        let fb = wrap(RoleSeg::FromB, b.clone());
        assert_eq!(a_side_is_first(&frag, &fb, &a, &b), Some(true));
    }

    #[test]
    fn an_equal_pair_is_not_a_sided_line() {
        let x = cap(1, CapEnd::End);
        let edge = |a: &StableName, b: &StableName| StableName {
            kind: EntityKind::Edge,
            node: RecipeNodeId(9),
            path: vec![RoleSeg::Seam {
                a: a.clone().into(),
                b: b.clone().into(),
            }],
        };
        assert!(seam_line_pair(&edge(&x, &x)).is_none());
        let y = cap(2, CapEnd::End);
        assert!(seam_line_pair(&edge(&x, &y)).is_some());
        // Through a boolean survivor, and not through a union member.
        let seam = edge(&x, &y);
        assert!(seam_line_pair(&wrap(RoleSeg::FromA, seam.clone())).is_some());
        let member = StableName {
            kind: EntityKind::Edge,
            node: RecipeNodeId(9),
            path: vec![RoleSeg::FromMember {
                member: RecipeNodeId(4),
                of: seam.into(),
            }],
        };
        assert!(seam_line_pair(&member).is_none());
    }
}
