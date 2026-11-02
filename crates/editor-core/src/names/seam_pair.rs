//! **Which seam pair a name lies on.**
//!
//! A crossing of a seam edge is read along each of the seam's two
//! sides, whose loops run along it opposite ways (N2), so the reader
//! needs the pair the edge's name records, and which of the edge's two
//! faces is which side.
//!
//! This module holds the one answer to "is this name on a seam line,
//! and which pair", for the readers that know a seam only by its NAME:
//! the pair emitter's and the union's reading of a crossed edge
//! (`emit_topo::edge_reading`), the union's reading of its own seams in
//! a run (`emit_union::Flush::seam_sense`), and a Split crossing's other
//! half (`role::Sense::other_half`).

use super::role::{EntityKind, Qualifier};
use super::role::{RoleSeg, StableName, name_free_seg};

/// What a name's HEAD segment means to a walk looking for a seam pair.
/// ONE exhaustive match over [`RoleSeg`]: a new segment variant is a
/// compile error here until someone says whether it passes an entity
/// through.
enum Head<'a> {
    /// A seam: the pair itself. A crossing vertex is read as one too:
    /// its edge and the face it meets.
    Seam(&'a StableName, &'a StableName),
    /// A vertex where several lines meet.
    Run,
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
        RoleSeg::Seam { a, b }
        | RoleSeg::Crossing {
            edge: a, face: b, ..
        } => Head::Seam(a, b),
        RoleSeg::EdgeCrossing(_) => Head::Run,
        // An entity carried in from an input, a split's fragment, a
        // pattern's or part's instance: the same entity (or a piece of
        // it) under one more op.
        RoleSeg::From { of: n, .. }
        | RoleSeg::SplitFragment { parent: n, .. }
        | RoleSeg::Member { of: n, .. } => Head::Through(n),
        RoleSeg::Merged(set) => Head::Merged(set),
        // New entities an op minted FROM a source — a blend face, a
        // shell's cavity twin (an offset line, not the source's), a
        // trimline, a crossing vertex — lie on no seam line of their
        // source.
        RoleSeg::BlendFace(_)
        | RoleSeg::CornerFace(_)
        | RoleSeg::TrimEdge { .. }
        | RoleSeg::FootVertex { .. }
        | RoleSeg::EndArc { .. }
        | RoleSeg::Mitre { .. }
        | RoleSeg::TurnFoot { .. }
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
        | RoleSeg::Fragment(Qualifier::Borders(_) | Qualifier::Keeps(_) | Qualifier::Ends(_))
        | name_free_seg!() => Head::Stop,
    }
}

/// The two sides `(a, b)` of the seam an EDGE name lies on, if any: a
/// seam edge is `Seam { a, b }`, and a piece or pass-through of one
/// keeps it through the wrappers [`head`] lists as `Through`, with any
/// `Fragment` tail after it. A pair whose two sides carry the SAME name
/// (two placements of one prototype, N1) is answered too.
pub(super) fn seam_edge_sides(name: &StableName) -> Option<(&StableName, &StableName)> {
    seam_through(name)
}

/// **The two sides of the seam an EDGE name lies on, each named in the
/// edge name's own space**: a side carried out through every wrapper the
/// edge's name passes through to its `Seam`, as that wrapper carries the
/// side's face (a `From`'s read, a split's fragment, an instance's or a
/// part's copy), the edge's piece qualifiers dropped. Where the name is
/// the `Seam` itself the sides are its own. `None` where
/// [`seam_edge_sides`] is.
pub(super) fn seam_edge_sides_carried(name: &StableName) -> Option<(StableName, StableName)> {
    let (a, b) = seam_through(name)?;
    let mut wrappers = Vec::new();
    let mut at = name;
    while let Some(seg) = at.path.first() {
        let Head::Through(inner) = head(seg) else {
            break;
        };
        wrappers.push((at.node, seg));
        at = inner;
    }
    let carry = |side: &StableName| {
        wrappers
            .iter()
            .rev()
            .fold(side.clone(), |inner, &(node, seg)| {
                let inner = super::role::NameRef::new(inner);
                let seg = match seg {
                    RoleSeg::From { read, .. } => RoleSeg::From {
                        read: *read,
                        of: inner,
                    },
                    RoleSeg::SplitFragment { .. } => RoleSeg::SplitFragment { parent: inner },
                    RoleSeg::Member { at, .. } => RoleSeg::Member {
                        at: at.clone(),
                        of: inner,
                    },
                    other => unreachable!(
                        "{other:?} is no wrapper `head` passes through: the walk above kept \
                         only `Head::Through` segments"
                    ),
                };
                StableName {
                    kind: EntityKind::Face,
                    node,
                    path: vec![seg],
                }
            })
    };
    Some((carry(a), carry(b)))
}

/// The `Seam` an EDGE name is minted as, through the wrappers [`head`]
/// lists as `Through`.
fn seam_through(name: &StableName) -> Option<(&StableName, &StableName)> {
    let mut at = name;
    loop {
        if at.kind != EntityKind::Edge {
            return None;
        }
        match head(at.path.first()?) {
            Head::Seam(a, b) => return Some((a, b)),
            Head::Run | Head::Merged(_) | Head::Stop => return None,
            Head::Through(inner) => at = inner,
        }
    }
}

/// Whether face name `n` denotes face `x`, or a face descended from it:
/// `x` itself or `x` followed by discriminators, through any number of
/// the wrappers [`head`] passes through, or a merged face with such a
/// constituent. Where `x` is itself a merged face, read through its
/// wrappers, a face descended from one of its constituents descends from
/// it too: a merge over `x` lists those, never `x` (N3).
pub(crate) fn face_descends_from(n: &StableName, x: &StableName) -> bool {
    let parts = super::merged::constituents_through_wrappers(x);
    let xs: Vec<&StableName> = core::iter::once(x).chain(parts.iter().flatten()).collect();
    let mut names = vec![n];
    while let Some(n) = names.pop() {
        if xs
            .iter()
            .any(|x| n.kind == x.kind && n.node == x.node && n.path.starts_with(&x.path))
        {
            return true;
        }
        match n.path.first().map(head) {
            Some(Head::Through(p)) => names.push(p),
            Some(Head::Merged(cs)) => names.extend(cs.iter().rev()),
            Some(Head::Seam(..) | Head::Run | Head::Stop) | None => {}
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

    fn from_a(of: crate::names::NameRef) -> RoleSeg {
        RoleSeg::From {
            read: crate::names::FOLD_A,
            of,
        }
    }

    fn from_b(of: crate::names::NameRef) -> RoleSeg {
        RoleSeg::From {
            read: crate::names::FOLD_B,
            of,
        }
    }

    use crate::names::role::CapEnd;
    use crate::node::RecipeNodeId;

    fn cap(node: u64, end: CapEnd) -> StableName {
        StableName {
            kind: EntityKind::Face,
            node: RecipeNodeId::new(0, node),
            path: vec![RoleSeg::Cap(end)],
        }
    }

    fn wrap(seg: fn(crate::names::role::NameRef) -> RoleSeg, inner: StableName) -> StableName {
        StableName {
            kind: inner.kind,
            node: RecipeNodeId::new(0, 9),
            path: vec![seg(inner.into())],
        }
    }

    fn merged(cs: Vec<StableName>) -> StableName {
        StableName {
            kind: EntityKind::Face,
            node: RecipeNodeId::new(0, 9),
            path: vec![RoleSeg::Merged(cs)],
        }
    }

    #[test]
    fn the_sides_are_read_off_the_faces_descent() {
        let (a, b) = (cap(1, CapEnd::End), cap(2, CapEnd::Start));
        let (fa, fb) = (wrap(from_a, a.clone()), wrap(from_b, b.clone()));
        assert_eq!(a_side_is_first(&fa, &fb, &a, &b), Some(true));
        assert_eq!(a_side_is_first(&fb, &fa, &a, &b), Some(false));
    }

    #[test]
    fn a_face_descending_from_neither_side_decides_nothing() {
        let (a, b) = (cap(1, CapEnd::End), cap(2, CapEnd::Start));
        let other = wrap(from_a, cap(3, CapEnd::End));
        let fb = wrap(from_b, b.clone());
        assert_eq!(a_side_is_first(&other, &fb, &a, &b), None);
    }

    #[test]
    fn two_faces_each_merged_across_both_sides_decide_nothing() {
        let (a, b) = (cap(1, CapEnd::End), cap(2, CapEnd::Start));
        let both = merged(vec![wrap(from_a, a.clone()), wrap(from_b, b.clone())]);
        assert_eq!(a_side_is_first(&both, &both, &a, &b), None);
    }

    #[test]
    fn a_face_merged_across_both_sides_is_resolved_by_the_other_face() {
        let (a, b) = (cap(1, CapEnd::End), cap(2, CapEnd::Start));
        let both = merged(vec![wrap(from_a, a.clone()), wrap(from_b, b.clone())]);
        let fb = wrap(from_b, b.clone());
        assert_eq!(a_side_is_first(&both, &fb, &a, &b), Some(true));
        assert_eq!(a_side_is_first(&fb, &both, &a, &b), Some(false));
    }

    #[test]
    fn a_split_fragment_is_seen_through() {
        let (a, b) = (cap(1, CapEnd::End), cap(2, CapEnd::Start));
        let frag = StableName {
            kind: EntityKind::Face,
            node: RecipeNodeId::new(0, 11),
            path: vec![RoleSeg::SplitFragment {
                parent: wrap(from_a, a.clone()).into(),
            }],
        };
        let fb = wrap(from_b, b.clone());
        assert_eq!(a_side_is_first(&frag, &fb, &a, &b), Some(true));
    }

    /// **A seam edge's sides are found through what carries it**, an
    /// equal pair included: a crossing of one reads alike along either.
    #[test]
    fn a_seam_edges_sides_are_found_through_its_carriers() {
        let x = cap(1, CapEnd::End);
        let edge = |a: &StableName, b: &StableName| StableName {
            kind: EntityKind::Edge,
            node: RecipeNodeId::new(0, 9),
            path: vec![RoleSeg::Seam {
                a: a.clone().into(),
                b: b.clone().into(),
            }],
        };
        assert_eq!(seam_edge_sides(&edge(&x, &x)), Some((&x, &x)));
        let y = cap(2, CapEnd::End);
        assert_eq!(seam_edge_sides(&edge(&x, &y)), Some((&x, &y)));
        // Through an entity carried in, a fold side's and a member's
        // read alike.
        let seam = edge(&x, &y);
        assert!(seam_edge_sides(&wrap(from_a, seam.clone())).is_some());
        let member = StableName {
            kind: EntityKind::Edge,
            node: RecipeNodeId::new(0, 9),
            path: vec![RoleSeg::From {
                read: crate::VarId::new(1, 4),
                of: seam.into(),
            }],
        };
        assert!(seam_edge_sides(&member).is_some());
    }

    /// What enters one half of a split along a seam leaves the other:
    /// the other half's sense names the other side, a pair of one name
    /// names it again, and a `Through` neither reading of the line
    /// gives refuses rather than standing for both halves.
    #[test]
    fn the_other_half_of_a_through_names_the_side_it_did_not() {
        use crate::names::role::Sense;
        let seam = |a: &StableName, b: &StableName| StableName {
            kind: EntityKind::Edge,
            node: RecipeNodeId::new(0, 9),
            path: vec![RoleSeg::Seam {
                a: a.clone().into(),
                b: b.clone().into(),
            }],
        };
        let through = |things: &[&StableName]| Sense::Through {
            entering: things.iter().map(|&n| n.clone().into()).collect(),
        };
        let (a, b) = (cap(1, CapEnd::End), cap(2, CapEnd::Start));
        let edge = seam(&a, &b);
        assert_eq!(through(&[&a]).other_half(&edge), Some(through(&[&b])));
        assert_eq!(through(&[&b]).other_half(&edge), Some(through(&[&a])));
        assert_eq!(
            through(&[&a]).other_half(&seam(&a, &a)),
            Some(through(&[&a])),
            "a pair of one name"
        );
        let other = cap(3, CapEnd::End);
        for unread in [through(&[]), through(&[&a, &b]), through(&[&other])] {
            assert_eq!(unread.other_half(&edge), None, "{unread:?} on a seam");
        }
        let plain = StableName {
            kind: EntityKind::Edge,
            node: RecipeNodeId::new(0, 4),
            path: vec![RoleSeg::SectionEdge {
                face: cap(4, CapEnd::End).into(),
            }],
        };
        assert_eq!(through(&[&plain]).other_half(&plain), None, "off a seam");
        assert_eq!(Sense::Enters.other_half(&edge), Some(Sense::Leaves));
    }
}
