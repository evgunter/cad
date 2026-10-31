//! **A split's joined seam keys its group by candidate, so a tied
//! face's joined seam keeps the bare name** (N2, N4).
//!
//! A split names an edge its join made along a chord and a whole
//! operand edge, beside the face the merge kept, as a seam of its two
//! faces' parents (`Joined::Seam`). Two tied faces spell that seam
//! alike; the group is keyed by the parents' entities as well, so each
//! candidate's joined seam is a group of its own and, undivided, the
//! bare name tied over the candidates. Keyed by the name alone, the two
//! would be one group of two, told apart by their ends.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use editor_core::{
    Bodies, EntityKind, Entry, EvalOptions, NameTable, Node, ProfileDoc, RecipeNodeId, RoleSeg,
    SplitHalf, StableName,
};
use fixture::{insert, plane_on_frame, prism, slid, table};
use geom_core::Tol;

/// **The U-cutter tie with a pocket under its prongs, split at x = 3.**
///
/// A 4×4×4 block minus a tool: the U-cutter (prongs y 1..1.5 and
/// 2.5..3, z 1..3, crossing the wall x = 4) united with a pocket
/// x 3..5, y 1.2..2.8, z 0.5..1.5. Each prong's floor is a piece of the
/// cutter's floor bordering the block's wall x = 4 alone, an N2 tie;
/// at x = 3 it steps down into the pocket, along an edge it shares with
/// the pocket's end wall, which lies in the plane x = 3.
///
/// The split at x = 3 merges that end wall into the section of the
/// half x < 3, and joins, on each floor, its chord across the plane
/// to its edge along the end wall: one joined seam per candidate,
/// between the floor and the merged face. Answers (doc, the subtract,
/// the split).
fn pocketed_prongs_split() -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("split-joined-keys-tie", Tol::witness());
    let (doc, a) = prism(
        doc,
        vec![(0.0, 0.0), (4.0, 0.0), (4.0, 4.0), (0.0, 4.0)],
        4.0,
    );
    let (doc, u) = fixture::u_cutter(doc);
    let (doc, u) = slid(doc, a, u, [0.0, 0.0, 1.0]);
    let (doc, pocket) = prism(
        doc,
        vec![(3.0, 1.2), (5.0, 1.2), (5.0, 2.8), (3.0, 2.8)],
        1.0,
    );
    let (doc, pocket) = slid(doc, a, pocket, [0.0, 0.0, 0.5]);
    let (doc, tool) = insert(
        doc,
        Node::Union {
            members: Bodies::Spelled(vec![u.into(), pocket.into()]),
        },
    );
    let (doc, sub) = insert(
        doc,
        Node::Subtract {
            from: a.into(),
            tool: tool.into(),
        },
    );
    let plane = plane_on_frame(&doc, sub, [3.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
    let (doc, split) = insert(
        doc,
        Node::Split {
            target: sub.into(),
            tool: plane,
        },
    );
    (doc, sub, split)
}

/// The edge rows of `t` spelled `Seam` between `side` and a `Merged`
/// parent, however qualified.
fn seams_beside_a_merge<'t>(
    t: &'t NameTable,
    side: &StableName,
) -> Vec<(&'t StableName, &'t Entry)> {
    let merged = |n: &StableName| matches!(n.path.first(), Some(RoleSeg::Merged(_)));
    t.iter()
        .filter(|(n, _)| {
            n.kind == EntityKind::Edge
                && match n.path.first() {
                    Some(RoleSeg::Seam { a, b }) => {
                        (**a == *side && merged(b)) || (**b == *side && merged(a))
                    }
                    _ => false,
                }
        })
        .collect()
}

/// **Each tied floor's joined seam beside the merged section is its
/// own group: the bare name, tied over the two candidates.**
#[test]
fn a_tied_faces_joined_seam_beside_the_merge_is_the_bare_name() {
    let (doc, sub, split) = pocketed_prongs_split();
    let ev = fixture::run(&doc, &EvalOptions::default());
    let tied: Vec<&StableName> = table(&ev, sub)
        .iter()
        .filter(|(n, e)| n.kind == EntityKind::Face && matches!(e, Entry::Tied(c) if c.len() == 2))
        .map(|(n, _)| n)
        .collect();
    assert_eq!(
        tied.len(),
        2,
        "the premise: the prongs' floors and ceilings are the subtract's two ties: {tied:?}"
    );
    let t = fixture::half(&ev, split, SplitHalf::Below);
    let seams: Vec<_> = tied
        .iter()
        .flat_map(|side| seams_beside_a_merge(t, side))
        .collect();
    let [(name, entry)] = seams.as_slice() else {
        panic!("the floors' joined seams beside the merged section are one row: {seams:#?}");
    };
    assert!(
        matches!(name.path.as_slice(), [RoleSeg::Seam { .. }]),
        "each candidate's joined seam is a group of its own, undivided: the bare name: {name:?}"
    );
    assert!(
        matches!(entry, Entry::Tied(c) if c.len() == 2),
        "the bare name is tied over the two floors' joined seams: {entry:?}"
    );
}
