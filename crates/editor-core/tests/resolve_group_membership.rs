//! **The group-size rung counts the emitter's groups.**
//!
//! `Diagnosis::GroupResized { was, now }` states how many entities a
//! vanished fragment's group held in the last-good run and holds now.
//! The group is the one the emitter formed (`names::FragmentGroups`,
//! recorded beside the name table), not the rows whose spelling happens
//! to share the fragment's base. These rows are the two places the
//! spelling and the emitter disagree, on real documents:
//! - two TIED parents, each cut in two: the tie lane merges both
//!   groups' rows under the same names, so the spelling counts one
//!   group of four where the emitter formed two groups of two;
//! - a split that stops dividing a face: the face passes through under
//!   its UPSTREAM name, so no row is spelled from the fragment's base,
//!   while the emitter's group for that face and side holds the face.
//!
//! The same arm's `cutters` names the seams on the group's parent only
//! one run spells, by the cutter across each (`GroupCutters`), and the
//! later rows pin it on real documents: a cutter that stops cutting, one
//! that starts, two at once, a union's member-space cutters in every
//! fold order, and a cutter a fold step re-qualified that is not read as
//! gone and new.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use editor_core::{
    CancelToken, Diagnosis, DocEdit, EntityKind, Entry, EvalOptions, Evaluation, GroupCutters,
    Node, ProfileDoc, RecipeNodeId, Resolution, ResolveError, RoleSeg, RunCtx, SlotId, StableName,
    evaluate,
};
use fixture::{insert, len, prism, step};
use geom_core::Tol;

fn run(doc: &editor_core::ProfileDoc, prior: Option<&Evaluation<f64>>) -> Evaluation<f64> {
    evaluate::<f64>(
        doc,
        prior,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

fn block(
    doc: ProfileDoc,
    (x0, x1): (f64, f64),
    (y0, y1): (f64, f64),
    dz: f64,
) -> (ProfileDoc, RecipeNodeId) {
    prism(doc, vec![(x0, y0), (x1, y0), (x1, y1), (x0, y1)], dz)
}

/// A copy of a body placed against `target`, which an edit slides.
pub(crate) struct Placed {
    /// The placement: the copy.
    pub(crate) placement: RecipeNodeId,
    pub(crate) target: RecipeNodeId,
    pub(crate) body: RecipeNodeId,
    /// Where the copy's own frame stands in `target`'s coordinates.
    pub(crate) at: [f64; 3],
}

/// `body` placed against `target`, its base `z0` up `target`'s own z.
pub(crate) fn placed(
    doc: ProfileDoc,
    target: RecipeNodeId,
    body: RecipeNodeId,
    z0: f64,
) -> (ProfileDoc, Placed) {
    let (doc, placement) = crate::fixture::slid(doc, target, body, [0.0, 0.0, z0]);
    (
        doc,
        Placed {
            placement,
            target,
            body,
            at: [0.0, 0.0, z0],
        },
    )
}

/// [`block`] placed against `target` at `z0`: the placement.
fn block_on(
    doc: ProfileDoc,
    target: RecipeNodeId,
    xs: (f64, f64),
    ys: (f64, f64),
    z0: f64,
    dz: f64,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, b) = block(doc, xs, ys, dz);
    crate::fixture::slid(doc, target, b, [0.0, 0.0, z0])
}

/// **`p`'s copy slid `by` further**, by value edits alone: each of its
/// placement's values, and the slide along its target face's normal,
/// set to the numbers [`editor_core::test_support::placed_against`] gives the copy
/// there ([`moved_to`]).
pub(crate) fn moved(doc: ProfileDoc, p: &Placed, by: [f64; 3]) -> ProfileDoc {
    let at = [p.at[0] + by[0], p.at[1] + by[1], p.at[2] + by[2]];
    moved_to(doc, p.target, p.body, p.placement, at)
}

/// **`placement`'s copy of `body` moved to `at` against `target`**
/// (`at` in `target`'s own coordinates), by value edits alone: the
/// numbers [`editor_core::test_support::placed_against`] gives the copy at `at` written
/// into the variables the placement already reads — its values, and the
/// slide of its target face along that face's normal — so every node
/// id and every read is kept. A translation off the target's face
/// plane is what the slide carries, so the placement must have been
/// written off it.
///
/// # Panics
///
/// If the placement is not the one `placed_against` writes, or carries
/// no slide.
pub(crate) fn moved_to(
    doc: ProfileDoc,
    target: RecipeNodeId,
    body: RecipeNodeId,
    placement: RecipeNodeId,
    at: [f64; 3],
) -> ProfileDoc {
    moved_to_frame(
        doc,
        target,
        body,
        placement,
        &editor_core::Frame::translation(at),
    )
}

/// [`moved_to`] with the copy's frame `at` turned as well as moved.
pub(crate) fn moved_to_frame(
    doc: ProfileDoc,
    target: RecipeNodeId,
    body: RecipeNodeId,
    placement: RecipeNodeId,
    at: &editor_core::Frame,
) -> ProfileDoc {
    use editor_core::pose::PoseDef;
    use editor_core::{FreeValue, Operand, VarDef};
    let again = editor_core::test_support::placed_against(
        &doc,
        &target.into(),
        &body.into(),
        at,
        Tol::witness(),
    );
    // A mate on a face plane through the frame reads the face itself; one
    // off it, the face slid along its normal, whose slide is moved too.
    let slide = match again.mates.first().map(|mate| &mate.to) {
        Some(Operand::Pose(def)) => {
            let PoseDef::Slide { by: slide, .. } = &**def else {
                panic!("the target face slid along its normal: {def:?}")
            };
            Some(slide.clone())
        }
        _ => None,
    };
    let bundle = crate::fixture::bundle(&doc, placement);
    let mut edits = Vec::new();
    if let Some(slide) = slide {
        let to = bundle
            .iter()
            .find_map(|c| c.mate())
            .map(|m| m.to)
            .expect("the placement's mate reads a pose");
        let Some(VarDef::Pose(PoseDef::Slide { by: slide_var, .. })) =
            doc.var(to).map(|v| v.def().clone())
        else {
            panic!("the placement's target is a slid face")
        };
        edits.push((slide_var, slide));
    }
    for (coord, value) in &again.values {
        let var = bundle
            .iter()
            .find_map(|c| c.value().filter(|v| v.coord == *coord).map(|v| v.by))
            .unwrap_or_else(|| panic!("the placement holds a value on {coord:?}"));
        edits.push((var, value.clone()));
    }
    // A free variable takes the number; a defined one, the formula.
    edits.into_iter().fold(doc, |doc, (var, value)| {
        let edit = match doc.var(var).map(|v| v.def()) {
            Some(VarDef::Free(_)) => DocEdit::SetVarValue {
                var: var.into(),
                value: FreeValue::Continuous(if value.is_zero() {
                    0.0
                } else {
                    value
                        .literal_value()
                        .unwrap_or_else(|| panic!("a literal number: {value:?}"))
                }),
            },
            _ => DocEdit::DefineVar {
                var: var.into(),
                def: editor_core::VarDecl::defined(value),
                fresh: Vec::new(),
            },
        };
        step(doc, edit).0
    })
}

/// The fragment names `node` minted in `ev1` that `ev2` no longer
/// carries, each with its diagnosis.
fn vanished(
    (doc2, ev2): (&ProfileDoc, &Evaluation<f64>),
    (doc1, ev1): (&ProfileDoc, &Evaluation<f64>),
    node: RecipeNodeId,
    pick: impl Fn(&StableName, &Entry) -> bool,
) -> Vec<(StableName, Diagnosis)> {
    // Each output's table against the same output's now (NAMES N1: a
    // split's halves are two).
    let reads = ev1.named_outputs(node);
    assert!(!reads.is_empty(), "the node evaluated before");
    reads
        .into_iter()
        .flat_map(|var| {
            let now = ev2.table(var).expect("the node evaluates now");
            ev1.table(var)
                .expect("the node evaluated before")
                .iter()
                .filter(move |(n, _)| now.lookup(n).is_none())
                .map(move |(n, e)| (var, n, e))
        })
        .filter(|(_, n, e)| matches!(n.path.last(), Some(RoleSeg::Fragment(_))) && pick(n, e))
        .map(|(var, n, _)| {
            // Resolved in the read whose table held it.
            let res = editor_core::resolve_with_prior(
                RunCtx {
                    doc: doc2,
                    eval: ev2,
                },
                RunCtx {
                    doc: doc1,
                    eval: ev1,
                },
                var,
                n,
            )
            .unwrap();
            let Resolution::Failed(f) = res else {
                panic!("{n:?}: expected Failed, got {res:?}");
            };
            let ResolveError::Vanished { diagnosis, .. } = f.error else {
                panic!("{n:?}: expected Vanished, got {:?}", f.error);
            };
            (n.clone(), diagnosis)
        })
        .collect()
}

/// The tie fixture: a 4×4×4 block minus a U-shaped `cutter` profile
/// (extruded 2, one up the block) whose two prongs cross one wall — the
/// cutter's caps leave two congruent prong faces each, an N2 tie — cut
/// again by a bar across both prongs, placed against the block.
/// Answers (doc, the first cut, the bar's placement, the second cut).
fn tied_prongs_cut(cutter: &[(f64, f64)]) -> (ProfileDoc, RecipeNodeId, Placed, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("group-membership-tie", Tol::witness());
    let (doc, a) = block(doc, (0.0, 4.0), (0.0, 4.0), 4.0);
    let (doc, u) = prism(doc, cutter.to_vec(), 2.0);
    let (doc, u) = crate::fixture::slid(doc, a, u, [0.0, 0.0, 1.0]);
    let (doc, sub) = insert(
        doc,
        Node::Subtract {
            from: a.into(),
            tool: u.into(),
        },
    );
    let (doc, bar) = block(doc, (2.9, 3.1), (0.5, 3.5), 3.0);
    let (doc, tr) = placed(doc, a, bar, 0.5);
    let (doc, cut) = insert(
        doc,
        Node::Subtract {
            from: sub.into(),
            tool: tr.placement.into(),
        },
    );
    (doc, sub, tr, cut)
}

/// **Two tied parents, each cut in two, are two groups of two.**
///
/// A 4×4×4 block minus a U-shaped cutter whose two prongs cross one
/// wall: the cutter's caps leave two congruent prong faces each, an N2
/// tie. A bar across both prongs cuts each tied face in two; sliding
/// the bar up out of the cavity (it still cuts the block's top) leaves
/// them whole. The emitter formed one group per tied face, two members
/// each, and one member each after the slide: 2 → 1. The spelling
/// counts the tie lane's merged rows, one group of four: 4 → 2.
#[test]
fn two_tied_parents_each_cut_in_two_are_two_groups_of_two() {
    let (doc, _, tr, cut) = tied_prongs_cut(&fixture::U_CUTTER);
    let ev1 = run(&doc, None);
    let doc2 = moved(doc.clone(), &tr, [0.0, 0.0, 3.2]);
    let ev2 = run(&doc2, Some(&ev1));
    // The tied parents' fragments: tied rows of the cut's own groups.
    let rows = vanished(
        (&doc2, &ev2),
        (&doc, &ev1),
        cut,
        |_, e| matches!(e, Entry::Tied(c) if c.len() == 2),
    );
    assert!(
        !rows.is_empty(),
        "no tied fragment vanished, so the row pins nothing"
    );
    for (n, d) in rows {
        assert_eq!(
            d,
            Diagnosis::GroupResized {
                node: cut,
                was: 2,
                now: 1,
                cutters: GroupCutters::TiedParents,
            },
            "{n:?}"
        );
    }
}

/// [`fixture::U_CUTTER`] with each prong forked into two tines where
/// [`tied_prongs_cut`]'s bar crosses it, so each of the bar's walls
/// seams each tied cap in two chords.
const TINED_U_CUTTER: [(f64, f64); 16] = [
    (2.0, 1.0),
    (6.0, 1.0),
    (6.0, 3.0),
    (2.0, 3.0),
    (2.0, 2.8),
    (3.5, 2.8),
    (3.5, 2.7),
    (2.0, 2.7),
    (2.0, 2.5),
    (5.0, 2.5),
    (5.0, 1.5),
    (2.0, 1.5),
    (2.0, 1.3),
    (3.5, 1.3),
    (3.5, 1.2),
    (2.0, 1.2),
];

/// **A tied parent's seam group counts its own pieces.**
///
/// [`tied_prongs_cut`] over [`TINED_U_CUTTER`]: each wall of the bar seams each tied cap in two
/// chords, one per tine, and the seam lanes group them by the two
/// parents' entities, so each candidate's two chords are one group of
/// two. Sliding the bar up out of the cavity takes them all: 2 → 0 for
/// every fragment, where the tie's sum would read 4 → 0.
#[test]
fn a_tied_parents_seam_group_counts_its_own_pieces() {
    let (doc, sub, tr, cut) = tied_prongs_cut(&TINED_U_CUTTER);
    let ev1 = run(&doc, None);
    let doc2 = moved(doc.clone(), &tr, [0.0, 0.0, 3.2]);
    let ev2 = run(&doc2, Some(&ev1));
    let tied = ev1.table_at(sub).expect("the first cut evaluates");
    let is_tied = |side: &StableName| match side.path.as_slice() {
        [RoleSeg::From { of, .. }] => matches!(tied.lookup(of), Some(Entry::Tied(_))),
        _ => false,
    };
    let rows = vanished((&doc2, &ev2), (&doc, &ev1), cut, |n, _| {
        n.kind == EntityKind::Edge
            && matches!(n.path.first(), Some(RoleSeg::Seam { a, b }) if is_tied(a) || is_tied(b))
    });
    // Two walls of the bar, across two caps, of two candidates, in two
    // pieces each.
    assert_eq!(
        rows.len(),
        16,
        "the bar's walls seam each tied cap in two pieces: {rows:?}"
    );
    for (n, d) in rows {
        assert_eq!(
            d,
            Diagnosis::GroupResized {
                node: cut,
                was: 2,
                now: 0,
                cutters: GroupCutters::NotSeamBounded,
            },
            "{n:?}"
        );
    }
}

/// `ev` with every node's verdict log emptied: the flip lanes then have
/// nothing to diff.
fn silent(mut ev: Evaluation<f64>) -> Evaluation<f64> {
    for result in ev.nodes.values_mut() {
        if let editor_core::NodeResult::Ok(v) = result {
            v.verdicts = std::sync::Arc::new(Vec::new());
        }
    }
    ev
}

/// **A split that stops dividing a face leaves a group of one.**
///
/// An L-shaped prism split by the vertical plane x + y = 2.5 leaves
/// each cap in three pieces, two of them on the upper side: a ranked
/// group of two. Turning the plane horizontal at z = 0.5 still splits
/// the prism, but no longer the top cap, which passes through under
/// its upstream name. The emitter's group for (top cap, upper side)
/// holds that face: 2 → 1. No row is spelled from the fragment's base,
/// so the spelling says 2 → 0.
///
/// Every change that stops a split dividing a face moves a vertex
/// across the plane, which the split records as a
/// `split_bisector_side` flip, and a recorded flip on the path is the
/// answer above this rung. So the two real runs are read with that
/// evidence taken out: no verdict log, and one document for both, so
/// the doc-diff lanes see no edit. What is left is the rung, over the
/// tables and the groups the two real splits emitted.
#[test]
fn a_split_that_stops_dividing_a_face_leaves_a_group_of_one() {
    let doc = ProfileDoc::empty_derived("group-membership-split", Tol::witness());
    let (doc, ext) = prism(
        doc,
        vec![
            (0.0, 0.0),
            (3.0, 0.0),
            (3.0, 1.0),
            (1.0, 1.0),
            (1.0, 3.0),
            (0.0, 3.0),
        ],
        1.0,
    );
    let tool = crate::fixture::plane_on_frame(&doc, ext, [2.5, 0.0, 0.0], [1.0, 1.0, 0.0]);
    let (doc, split) = insert(
        doc,
        Node::Split {
            target: ext.into(),
            tool,
        },
    );
    let ev1 = run(&doc, None);
    // The plane turned level at z = 0.5: it divides no cap.
    let level = crate::fixture::start_cap_at(ext, crate::fixture::len(0.5));
    let (doc2, _) = step(
        doc.clone(),
        DocEdit::SetParam {
            node: split,
            slot: SlotId::Operand(editor_core::OperandSlot::Tool),
            value: editor_core::SlotValue::Read(level.read),
            fresh: Vec::new(),
        },
    );
    let ev2 = silent(run(&doc2, Some(&ev1)));
    let ev1 = silent(ev1);
    let rows = vanished((&doc, &ev2), (&doc, &ev1), split, |n, _| {
        n.kind == EntityKind::Face && matches!(n.path.first(), Some(RoleSeg::SplitFragment { .. }))
    });
    assert!(
        !rows.is_empty(),
        "no split fragment vanished, so the row pins nothing"
    );
    let mut top = 0;
    for (n, d) in rows {
        // The top cap stays on the upper side, whole; the bottom cap
        // moves wholly to the lower side, so its upper group is empty.
        let Some(RoleSeg::SplitFragment { parent, .. }) = n.path.first() else {
            unreachable!("picked by that head");
        };
        let now = if parent
            .path
            .contains(&RoleSeg::Cap(editor_core::CapEnd::End))
        {
            top += 1;
            1
        } else {
            0
        };
        assert_eq!(
            d,
            Diagnosis::GroupResized {
                node: split,
                was: 2,
                now,
                cutters: GroupCutters::NotSeamBounded,
            },
            "{n:?}"
        );
    }
    assert!(
        top > 0,
        "no top-cap fragment vanished, so the pass-through is not pinned"
    );
}

/// **A union's fragment group is read through its fold, and so are its
/// cutters.**
///
/// A 3×3×1 plate, a bar standing through its top in y (a placed copy),
/// and a block far away, united in every order: the bar
/// divides the plate's top into two fragments at whichever fold step
/// the plate and the bar meet, and every later step carries them
/// through. Sliding the bar in y so it stops short of the far edge
/// leaves the top whole. The union's record is each step's groups,
/// followed to the published names: 2 → 1 in every order. The two runs
/// are read without their verdict logs and against one document, for
/// the split row's reason: the slide's own flip is diagnosed first.
///
/// The cutters are read off the published seams, whose sides are in
/// name order and member-keyed: sliding in y, the bar's y = y0 wall
/// starts cutting the top; sliding 1.5 in x instead, its x = x1 wall
/// stops cutting the top's x-running rim edges (ranked, so no side
/// verdict answers first). Every order names the same member-space wall.
#[test]
fn a_unions_group_resized_at_any_fold_step_reads_two_to_one() {
    let orders: [[usize; 3]; 6] = [
        [0, 1, 2],
        [0, 2, 1],
        [1, 0, 2],
        [1, 2, 0],
        [2, 0, 1],
        [2, 1, 0],
    ];
    for order in orders {
        let doc = ProfileDoc::empty_derived("group-membership-union", Tol::witness());
        let (doc, plate) = block(doc, (0.0, 3.0), (0.0, 3.0), 1.0);
        let (doc, bar) = block(doc, (1.0, 2.0), (-1.0, 4.0), 1.0);
        let (doc, tr) = placed(doc, plate, bar, 0.5);
        let (doc, far) = block_on(doc, plate, (10.0, 11.0), (0.0, 1.0), 0.0, 1.0);
        let m = [plate, tr.placement, far];
        let (doc, u) = insert(
            doc,
            Node::Union {
                members: editor_core::Bodies::Spelled(order.iter().map(|&i| m[i].into()).collect()),
            },
        );
        // Member `member`'s wall `segment` (of the block `of`), as the
        // union's seams spell it: a copy carries its block's names.
        let member_wall = |member: RecipeNodeId, of, segment| StableName {
            kind: editor_core::EntityKind::Face,
            node: u,
            path: vec![RoleSeg::From {
                read: crate::fixture::out(&doc, member),
                of: editor_core::NameRef::new(wall(&doc, of, segment)),
            }],
        };
        let from = |n: &StableName, m: RecipeNodeId| {
            let m_read = crate::fixture::out(&doc, m);
            matches!(n.path.first(), Some(RoleSeg::From { read, .. }) if *read == m_read)
        };
        for (axis, by, ranked, gone, new) in [
            (
                "y",
                2.5,
                false,
                vec![],
                vec![member_wall(tr.placement, bar, 0)],
            ),
            (
                "x",
                1.5,
                true,
                vec![member_wall(tr.placement, bar, 1)],
                vec![],
            ),
        ] {
            let ev1 = run(&doc, None);
            let slide = if axis == "x" {
                [by, 0.0, 0.0]
            } else {
                [0.0, by, 0.0]
            };
            let doc2 = moved(doc.clone(), &tr, slide);
            // The slide records a containment flip at the union, the
            // answer above this rung; read the two real runs without
            // that evidence, as the split row does.
            let ev2 = silent(run(&doc2, Some(&ev1)));
            let ev1 = silent(ev1);
            let rows = vanished((&doc, &ev2), (&doc, &ev1), u, |n, e| {
                matches!(e, Entry::Unique(_))
                    && match n.path.last() {
                        Some(RoleSeg::Fragment(editor_core::Qualifier::Borders(_))) => !ranked,
                        Some(RoleSeg::Fragment(editor_core::Qualifier::Ends(_))) => {
                            ranked && from(n, plate)
                        }
                        _ => false,
                    }
            });
            assert!(
                !rows.is_empty(),
                "{order:?} {axis:?}: no fragment vanished, so the row pins nothing"
            );
            for (n, d) in rows {
                // The plate's own top, divided by the bar; or, sliding
                // in y, one of the bar's x walls, which the plate's
                // y = y0 wall divided where the bar ran out through it
                // and no longer meets.
                let (gone, new) = if from(&n, plate) {
                    (gone.clone(), new.clone())
                } else {
                    assert!(
                        from(&n, tr.placement) && !ranked,
                        "{order:?} {axis:?}: {n:?}"
                    );
                    (vec![member_wall(plate, plate, 0)], vec![])
                };
                assert_eq!(
                    d,
                    Diagnosis::GroupResized {
                        node: u,
                        was: 2,
                        now: 1,
                        cutters: GroupCutters::Read { gone, new },
                    },
                    "{order:?} {axis:?}: {n:?}"
                );
            }
        }
    }
}

/// **A seam of a tied parent is one candidate's group.**
///
/// The seam lanes group a seam edge by its two parents' entities, so
/// the bar crossing the two tied prong faces seams each candidate on
/// its own: each seam is the bare name, tied over the candidates (N4).
/// [`a_tied_parents_seam_group_counts_its_own_pieces`] reads such a
/// group's count.
#[test]
fn a_tied_parents_seams_are_bare() {
    let (doc, sub, _, cut) = tied_prongs_cut(&fixture::U_CUTTER);
    let ev = run(&doc, None);
    // A seam a side of which is one of the first cut's TIED rows, as the
    // second cut carries it in.
    let tied = ev.table_at(sub).expect("the first cut evaluates");
    let is_tied = |side: &StableName| match side.path.as_slice() {
        [RoleSeg::From { of, .. }] => matches!(tied.lookup(of), Some(Entry::Tied(_))),
        _ => false,
    };
    let cites_a_tie = |n: &StableName| match n.path.first() {
        Some(RoleSeg::Seam { a, b }) => is_tied(a) || is_tied(b),
        _ => false,
    };
    let seams: Vec<_> = ev
        .table_at(cut)
        .expect("the second cut evaluates")
        .iter()
        .filter(|(n, _)| cites_a_tie(n))
        .collect();
    assert!(
        !seams.is_empty(),
        "the bar seams no tied face, so the row pins nothing"
    );
    for (n, e) in &seams {
        assert!(
            n.path.len() == 1 && matches!(e, Entry::Tied(c) if c.len() == 2),
            "a tied parent's seam is the bare name, one edge per candidate: {n:?} => {e:?}"
        );
    }
}

/// **A union names what its later fold steps left of a group.**
///
/// The bar divides the plate's top into two at the first fold step,
/// and the block C, united at the next, swallows one of the two pieces
/// — so the published body holds ONE face of that parent, in both
/// runs, before and after the bar slides. A parent held as one face is
/// published under the parent's own name (N2), not as a fragment, so no
/// name of the plate vanishes across the slide and the rung is never
/// asked. Both layouts: C over the far piece, and C over the near one.
#[test]
fn a_union_group_a_later_step_partly_swallows_names_what_is_published() {
    for (label, c) in [
        ("far", ((1.7, 4.0), (-1.5, 4.5), 0.3, 1.4)),
        ("near", ((-1.0, 1.3), (-1.5, 4.5), 0.3, 1.4)),
    ] {
        let doc = ProfileDoc::empty_derived("group-membership-swallow", Tol::witness());
        let (doc, plate) = block(doc, (0.0, 3.0), (0.0, 3.0), 1.0);
        let (doc, bar) = block(doc, (1.0, 2.0), (-1.0, 4.0), 1.0);
        let (doc, tr) = placed(doc, plate, bar, 0.5);
        let (doc, cblock) = block_on(doc, plate, c.0, c.1, c.2, c.3);
        let (doc, u) = insert(
            doc,
            Node::Union {
                members: editor_core::Bodies::Spelled(vec![
                    plate.into(),
                    tr.placement.into(),
                    cblock.into(),
                ]),
            },
        );
        let ev1 = run(&doc, None);
        let doc2 = moved(doc.clone(), &tr, [0.0, 2.5, 0.0]);
        let ev2 = silent(run(&doc2, Some(&ev1)));
        let ev1 = silent(ev1);
        // The plate's own faces: its top's fragments. (Its rim's one
        // piece is named by its ends, and the slide moves one of them.)
        let plate_read = crate::fixture::out(&doc, plate);
        let rows = vanished((&doc, &ev2), (&doc, &ev1), u, |n, _| {
            n.kind == EntityKind::Face
                && matches!(n.path.first(), Some(RoleSeg::From { read, .. }) if *read == plate_read)
        });
        assert!(rows.is_empty(), "{label}: {rows:?}");
        for ev in [&ev1, &ev2] {
            let t = ev.table_at(u).expect("the union evaluates");
            let plate_faces: Vec<_> = t
                .iter()
                .filter(|(n, _)| {
                    n.kind == editor_core::EntityKind::Face
                        && matches!(n.path.first(), Some(RoleSeg::From { read, .. }) if *read == plate_read)
                })
                .map(|(n, _)| n.clone())
                .collect();
            assert!(
                !plate_faces.is_empty() && plate_faces.iter().all(|n| n.path.len() == 1),
                "{label}: the plate's faces are not all published whole: {plate_faces:?}"
            );
        }
    }
}

/// **A tie carried through a later fold step counts one parent at a
/// time.**
///
/// The tie fixture's first cut, the bar and a far block, united: the
/// bar divides each of the two tied prong faces at one fold step, and
/// the far block's step carries the pieces through. The two tied
/// parents are two entities, so each tied group counts its own pieces
/// in the published body, 2 → 1 as the bar slides out of the cavity —
/// never the tie's sum. `[sub, far, tr]` forms the groups at the LAST
/// step, which counts its own members: the control.
#[test]
fn a_tie_carried_through_a_later_fold_step_counts_one_parent() {
    for order in [[0usize, 1, 2], [1, 0, 2], [0, 2, 1]] {
        let (doc, sub, tr, _) = tied_prongs_cut(&fixture::U_CUTTER);
        let (doc, far) = block_on(doc, tr.target, (20.0, 21.0), (0.0, 1.0), 0.0, 1.0);
        let m = [sub, tr.placement, far];
        let (doc, u) = insert(
            doc,
            Node::Union {
                members: editor_core::Bodies::Spelled(order.iter().map(|&i| m[i].into()).collect()),
            },
        );
        let ev1 = run(&doc, None);
        let doc2 = moved(doc.clone(), &tr, [0.0, 0.0, 3.2]);
        let ev2 = silent(run(&doc2, Some(&ev1)));
        let ev1 = silent(ev1);
        let rows = vanished((&doc, &ev2), (&doc, &ev1), u, |n, e| {
            n.kind == editor_core::EntityKind::Face && matches!(e, Entry::Tied(c) if c.len() == 2)
        });
        assert!(
            !rows.is_empty(),
            "{order:?}: no tied face fragment vanished, so the row pins nothing"
        );
        for (n, d) in rows {
            assert_eq!(
                d,
                Diagnosis::GroupResized {
                    node: u,
                    was: 2,
                    now: 1,
                    cutters: GroupCutters::TiedParents,
                },
                "{order:?}: {n:?}"
            );
        }
    }
}

/// **A union's seam of a tied parent counts that parent's pieces.**
///
/// [`fixture::u_cutter_tie`]'s tied floors, a block D in one prong's
/// mouth whose wall meets that floor along one seam, and a post across
/// the seam. Lifting the post leaves the seam whole: 2 → 1.
#[test]
fn a_unions_seam_of_a_tied_parent_counts_its_pieces() {
    let (doc, sub) = crate::fixture::u_cutter_tie(ProfileDoc::empty_derived(
        "group-membership-union-tied-seam",
        Tol::witness(),
    ));
    let (doc, d) = block_on(doc, sub, (3.5, 4.5), (0.75, 1.75), 0.5, 1.0);
    let (doc, body) = block(doc, (0.0, 0.4), (0.0, 0.1), 0.9);
    let at = [3.3, 1.2, 0.5];
    let (doc, placement) = crate::fixture::slid(doc, sub, body, at);
    let post = Placed {
        placement,
        target: sub,
        body,
        at,
    };
    let (doc, u) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![
                sub.into(),
                d.into(),
                post.placement.into(),
            ]),
        },
    );
    let ev1 = run(&doc, None);
    let doc2 = moved(doc.clone(), &post, [0.0, 0.0, 0.6]);
    let ev2 = silent(run(&doc2, Some(&ev1)));
    let ev1 = silent(ev1);
    // The seams on the tied floor, as the union publishes them.
    let floor = crate::fixture::out(&doc, sub);
    let on_floor = |side: &StableName| {
        matches!(side.path.first(), Some(RoleSeg::From { read, .. }) if *read == floor)
            && ev1
                .table_at(sub)
                .and_then(|t| match side.path.first() {
                    Some(RoleSeg::From { of, .. }) => t.lookup(of),
                    _ => None,
                })
                .is_some_and(|e| matches!(e, Entry::Tied(_)))
    };
    let rows = vanished(
        (&doc2, &ev2),
        (&doc, &ev1),
        u,
        |n, _| matches!(n.path.first(), Some(RoleSeg::Seam { a, b }) if on_floor(a) || on_floor(b)),
    );
    assert_eq!(
        rows.len(),
        2,
        "both pieces of the tied floor's seam vanish: {rows:#?}"
    );
    for (n, d) in rows {
        assert!(
            matches!(d, Diagnosis::GroupResized { node, was: 2, now: 1, .. } if node == u),
            "{n:?}: {d:?}"
        );
    }
}

/// **Two tied groups that resize unequally have no one parent's count.**
///
/// [`fixture::u_cutter_tie`]'s tied floors, each divided by a bar of its
/// own; lifting the second bar leaves its floor whole while the first
/// stays divided. The union's two tied face groups go from 2 and 2 to 2
/// and 1, so the rung declines and the doc-diff lane names the edit, in
/// either member order. (Read through the fold, the latest step spells
/// only the floor it divided, and the answer would hang on the order.)
#[test]
fn tied_groups_that_resize_unequally_decline_in_every_member_order() {
    for swap in [false, true] {
        let (doc, sub) = crate::fixture::u_cutter_tie(ProfileDoc::empty_derived(
            "group-membership-union-unequal-tie",
            Tol::witness(),
        ));
        let mut doc = doc;
        let mut bars = Vec::new();
        for y in [0.9, 2.4] {
            let (d, body) = block(doc, (0.0, 0.2), (0.0, 0.7), 0.9);
            let at = [2.9, y, 0.5];
            let (d, placement) = crate::fixture::slid(d, sub, body, at);
            doc = d;
            bars.push(Placed {
                placement,
                target: sub,
                body,
                at,
            });
        }
        let mut order = vec![bars[0].placement, bars[1].placement];
        if swap {
            order.reverse();
        }
        let members = std::iter::once(sub).chain(order).map(Into::into).collect();
        let (doc, u) = insert(
            doc,
            Node::Union {
                members: editor_core::Bodies::Spelled(members),
            },
        );
        let ev1 = run(&doc, None);
        let doc2 = moved(doc.clone(), &bars[1], [0.0, 0.0, 0.6]);
        let ev2 = silent(run(&doc2, Some(&ev1)));
        let ev1 = silent(ev1);
        let rows = vanished((&doc2, &ev2), (&doc, &ev1), u, |n, _| {
            n.kind == EntityKind::Face
        });
        assert!(!rows.is_empty(), "swap {swap}: no floor fragment vanished");
        for (n, d) in rows {
            assert!(
                matches!(d, Diagnosis::RecipeEdit { .. }),
                "swap {swap}: unequal tied groups decline to the edit, not a count: {n:?}: {d:?}"
            );
        }
    }
}

/// **A union's corner held as two copies, then once, reads 2 → 1 in
/// every member order.**
///
/// [`prism_and_cube`]'s L-prism, whose reflex top corner the tilted
/// cube pinches into two copies, each a fragment of that corner, with
/// a wall and a post standing apart. Then the cube stands away and the
/// wall and the post close in on the corner: the wall's face y = 1
/// runs through it, so a fold step that meets the wall before the post
/// joins the corner away, and the post's face x = 1 makes it a vertex
/// again. The corner is held once, by its own name. The union's group
/// for it is formed over the finished body, so both copies' name
/// vanishes as `GroupResized { was: 2, now: 1 }` in every member order.
/// Read through the fold steps, it was 2 → 0 in the orders where the
/// wall joined the corner away. The runs are read as the split row
/// reads them.
#[test]
fn a_unions_corner_held_twice_then_once_reads_two_to_one_in_every_member_order() {
    use crate::union_member_orders_decide_alike::{CUBE_NORMAL, prism_and_cube};
    // The cube's frame in `prism_and_cube_along`, moved off the prism's
    // top to stand away from it.
    let unit = |m: [f64; 3]| {
        let n = m.iter().map(|c| c * c).sum::<f64>().sqrt();
        m.map(|c| c / n)
    };
    let n = unit(CUBE_NORMAL);
    let u = unit([-n[2] * n[0], -n[2] * n[1], 1.0 - n[2] * n[2]]);
    let v = [
        n[1] * u[2] - n[2] * u[1],
        n[2] * u[0] - n[0] * u[2],
        n[0] * u[1] - n[1] * u[0],
    ];
    let away = crate::fixture::axes_at([-6.0, -6.0, 1.0], u, v);
    let mut orders = vec![vec![0usize]];
    for k in 1..4 {
        orders = orders
            .into_iter()
            .flat_map(|o| {
                (0..=o.len()).map(move |i| {
                    let mut o = o.clone();
                    o.insert(i, k);
                    o
                })
            })
            .collect();
    }
    for order in orders {
        let doc = ProfileDoc::empty_derived("group-membership-union-corner", Tol::witness());
        let (doc, m) = prism_and_cube(doc);
        let (ell, cube) = (m[0], m[1]);
        let Some(Node::Place { source, .. }) = doc.node(cube) else {
            panic!("the cube is a copy")
        };
        let cube_body = doc
            .operation_of(source.read)
            .expect("a copy reads a construction");
        let (doc, wall) = block(doc, (-1.0, 2.0), (1.0, 3.0), 1.0);
        let (doc, wall) = placed(doc, ell, wall, 0.5);
        let (doc, post) = block(doc, (1.0, 1.2), (-1.0, 1.1), 1.06);
        let (doc, post) = placed(doc, ell, post, 0.47);
        let apart = [10.0, 0.0, 0.0];
        let doc = moved(moved(doc, &wall, apart), &post, apart);
        let members = [ell, cube, wall.placement, post.placement];
        let (doc, un) = insert(
            doc,
            Node::Union {
                members: editor_core::Bodies::Spelled(
                    order.iter().map(|&i| members[i].into()).collect(),
                ),
            },
        );
        let ev1 = run(&doc, None);
        let back = [-10.0, 0.0, 0.0];
        let doc2 = moved_to_frame(doc.clone(), ell, cube_body, cube, &away);
        let doc2 = moved(
            doc2,
            &Placed {
                at: [10.0, 0.0, 0.5],
                ..wall
            },
            back,
        );
        let doc2 = moved(
            doc2,
            &Placed {
                at: [10.0, 0.0, 0.47],
                ..post
            },
            back,
        );
        let ev2 = silent(run(&doc2, Some(&ev1)));
        let ev1 = silent(ev1);
        let rows = vanished((&doc, &ev2), (&doc, &ev1), un, |n, _| {
            n.kind == EntityKind::Vertex
        });
        assert_eq!(
            rows.len(),
            1,
            "{order:?}: the copies' name vanishes: {rows:#?}"
        );
        for (name, d) in rows {
            assert!(
                matches!(d, Diagnosis::GroupResized { node, was: 2, now: 1, .. } if node == un),
                "{order:?}: {name:?}: {d:?}"
            );
        }
    }
}

/// Wall `segment` of the block `node` extruded (`block`'s profile runs
/// counter-clockwise from `(x0, y0)`: wall 0 is y = y0, 1 is x = x1, 2
/// is y = y1, 3 is x = x0).
fn wall(doc: &ProfileDoc, node: RecipeNodeId, segment: u32) -> StableName {
    StableName {
        kind: editor_core::EntityKind::Face,
        node,
        path: vec![RoleSeg::Lateral(
            crate::fixture::piece(doc, node, 0, segment as usize).into(),
        )],
    }
}

/// The plate-and-bar fixture as a PAIR union: a 3×3×1 plate, and a bar
/// standing through its top across the whole plate in y, placed against
/// the plate. The bar's two x walls divide the plate's top in two,
/// and each of the top's two x-running rim edges.
/// Answers (doc, the plate, the bar, the bar's placement, the union).
fn plate_and_bar() -> (ProfileDoc, RecipeNodeId, RecipeNodeId, Placed, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("group-cutters", Tol::witness());
    let (doc, plate) = block(doc, (0.0, 3.0), (0.0, 3.0), 1.0);
    let (doc, bar) = block(doc, (1.0, 2.0), (-1.0, 4.0), 1.0);
    let (doc, tr) = placed(doc, plate, bar, 0.5);
    let (doc, u) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![plate.into(), tr.placement.into()]),
        },
    );
    (doc, plate, bar, tr, u)
}

/// Whether `n` is the pair union's row descended from the plate's
/// entity spelled `seg`.
fn from_plate(n: &StableName, plate: RecipeNodeId, seg: &RoleSeg) -> bool {
    matches!(n.path.first(), Some(RoleSeg::From { of: p, .. }) if p.node == plate && p.path == [seg.clone()])
}

const TOP: RoleSeg = RoleSeg::Cap(editor_core::CapEnd::End);

/// The plate's top rim edge over profile segment `segment`.
fn rim(doc: &ProfileDoc, plate: RecipeNodeId, segment: u32) -> RoleSeg {
    RoleSeg::RimEdge(
        editor_core::CapEnd::End,
        crate::fixture::piece(doc, plate, 0, segment as usize).into(),
    )
}

/// The plate-and-bar pair union with the bar slid by `(dx, dy)`: every
/// vanished fragment of the plate's entities spelled by one of `segs`,
/// diagnosed. Read without the verdict logs and against one document,
/// as the split row is. Answers (the bar, the union, the rows).
fn slid(
    (dx, dy): (f64, f64),
    segs: &[RoleSeg],
) -> (RecipeNodeId, RecipeNodeId, Vec<(StableName, Diagnosis)>) {
    let (doc, plate, bar, tr, u) = plate_and_bar();
    let ev1 = run(&doc, None);
    let doc2 = moved(doc.clone(), &tr, [dx, dy, 0.0]);
    let ev2 = silent(run(&doc2, Some(&ev1)));
    let ev1 = silent(ev1);
    let rows = vanished((&doc, &ev2), (&doc, &ev1), u, |n, _| {
        segs.iter().any(|s| from_plate(n, plate, s))
    });
    assert!(
        !rows.is_empty(),
        "({dx}, {dy}): no fragment of {segs:?} vanished, so the row pins nothing"
    );
    (bar, u, rows)
}

/// The bar's wall `segment` as the plate-and-bar union publishes it:
/// carried in through the read of the bar's placed copy.
fn bar_wall(segment: u32) -> StableName {
    let (doc, _, bar, tr, u) = plate_and_bar();
    StableName {
        kind: editor_core::EntityKind::Face,
        node: u,
        path: vec![RoleSeg::From {
            read: crate::fixture::out(&doc, tr.placement),
            of: editor_core::NameRef::new(wall(&doc, bar, segment)),
        }],
    }
}

/// `GroupResized { was: 2, now: 1 }` at `node` with these cutters.
fn two_to_one(node: RecipeNodeId, gone: Vec<StableName>, new: Vec<StableName>) -> Diagnosis {
    Diagnosis::GroupResized {
        node,
        was: 2,
        now: 1,
        cutters: GroupCutters::Read { gone, new },
    }
}

/// **A cutter that stops cutting is named.**
///
/// The bar slides 1.5 in x, off the plate's far edge: its x = x0 wall
/// still cuts the top's two x-running rim edges, and its x = x1 wall is
/// past the plate and no longer meets them. Each rim edge is one piece,
/// 2 → 1, and the one cutter whose seam vertex with it is gone is that
/// wall. Ranked edges, so no side verdict answers first.
#[test]
fn a_cutter_that_stops_cutting_is_named_gone() {
    // `slid` builds this same recipe, so its pieces are these.
    let (doc, plate, ..) = plate_and_bar();
    let (_, u, rows) = slid((1.5, 0.0), &[rim(&doc, plate, 0), rim(&doc, plate, 2)]);
    for (n, d) in rows {
        assert_eq!(d, two_to_one(u, vec![bar_wall(1)], vec![]), "{n:?}");
    }
}

/// **A cutter that starts cutting is named.**
///
/// The bar slides 2.5 in y, so it stops short of the plate's near edge:
/// both x walls still cut the top, and the bar's y = y0 wall now crosses
/// it too. The top is one piece around the bar, 2 → 1, with no cutter
/// gone and that wall new.
#[test]
fn a_cutter_that_starts_cutting_is_named_new() {
    let (_, u, rows) = slid((0.0, 2.5), &[TOP]);
    for (n, d) in rows {
        assert_eq!(d, two_to_one(u, vec![], vec![bar_wall(0)]), "{n:?}");
    }
}

/// **Two cutters that change at once are both named.**
///
/// Slid 1.5 in x and 2.5 in y, the bar covers only the plate's far
/// corner: its x = x1 wall stops cutting the top and its y = y0 wall
/// starts. The top's two vanished pieces are both the rung's to answer,
/// naming both walls: a piece is named by the walls it borders, and no
/// piece of the top remains to compare walls with. Slid 5 in y, clear
/// of the plate, both x walls stop cutting the top and its rim edges.
/// Each list holds every cutter it states.
#[test]
fn two_cutters_that_change_at_once_are_both_named() {
    // `slid` builds this same recipe, so its pieces are these.
    let (doc, plate, ..) = plate_and_bar();
    let (_, u, rows) = slid((1.5, 2.5), &[TOP]);
    let mut answered = 0;
    for (n, d) in rows {
        answered += 1;
        assert_eq!(
            d,
            two_to_one(u, vec![bar_wall(1)], vec![bar_wall(0)]),
            "{n:?}"
        );
    }
    assert_eq!(answered, 2, "both vanished pieces are the rung's to answer");
    let (_, u, rows) = slid((0.0, 5.0), &[TOP, rim(&doc, plate, 0), rim(&doc, plate, 2)]);
    let mut gone = vec![bar_wall(1), bar_wall(3)];
    gone.sort();
    for (n, d) in rows {
        assert_eq!(d, two_to_one(u, gone.clone(), vec![]), "{n:?}");
    }
}

/// **A cutter a fold step re-qualified is the same cutter.**
///
/// The plate, the bar and a small block C standing across the bar's
/// x = x0 wall beyond the plate's near edge, united with C before the
/// plate: C divides that wall at an earlier fold step, so the union's
/// seam vertices between the top's x-running rim edges and the wall
/// spell the wall with the fold's `Fragment` after it. Sliding the bar
/// 1.5 in x takes it off C and half off the plate: the x = x0 wall is
/// whole and still cuts each rim edge, its seam spelled without the
/// tail, and the x = x1 wall no longer meets them (2 → 1). Read with
/// the tail, the x = x0 wall would be gone and new at once; the fold's
/// tail is not the member's, so it is neither, and the one cutter gone
/// is the x = x1 wall. Ranked edges: their names embed no partner, so
/// no cascade answers first.
#[test]
fn a_cutter_a_fold_step_requalified_is_the_same_cutter() {
    for order in [[0usize, 1, 2], [1, 0, 2]] {
        let doc = ProfileDoc::empty_derived("group-cutters-refold", Tol::witness());
        let (doc, plate) = block(doc, (0.0, 3.0), (0.0, 3.0), 1.0);
        let (doc, bar) = block(doc, (1.0, 2.0), (-1.0, 4.0), 1.0);
        let (doc, tr) = placed(doc, plate, bar, 0.5);
        let (doc, c) = block_on(doc, plate, (0.5, 1.5), (-0.8, -0.5), 0.0, 2.0);
        let m = [tr.placement, c];
        let (doc, u) = insert(
            doc,
            Node::Union {
                members: editor_core::Bodies::Spelled(vec![
                    m[order[0]].into(),
                    m[order[1]].into(),
                    plate.into(),
                ]),
            },
        );
        let ev1 = run(&doc, None);
        let doc2 = moved(doc.clone(), &tr, [1.5, 0.0, 0.0]);
        let ev2 = silent(run(&doc2, Some(&ev1)));
        let ev1 = silent(ev1);
        let plate_read = crate::fixture::out(&doc, plate);
        let rows = vanished((&doc, &ev2), (&doc, &ev1), u, |n, e| {
            matches!(e, Entry::Unique(_))
                && n.kind == editor_core::EntityKind::Edge
                && matches!(n.path.first(),
                    Some(RoleSeg::From { read, .. }) if *read == plate_read)
        });
        assert!(
            !rows.is_empty(),
            "{order:?}: no rim fragment vanished, so the row pins nothing"
        );
        let x1_wall = StableName {
            kind: editor_core::EntityKind::Face,
            node: u,
            path: vec![RoleSeg::From {
                read: crate::fixture::out(&doc, tr.placement),
                of: editor_core::NameRef::new(wall(&doc, bar, 1)),
            }],
        };
        for (n, d) in rows {
            assert_eq!(
                d,
                two_to_one(u, vec![x1_wall.clone()], vec![]),
                "{order:?}: {n:?}"
            );
        }
    }
}

/// **The pieces of one line at a node are one group, whichever parent
/// edge each descends from** (N2, *Edge pieces*; N5's group-size rung).
///
/// A 20×2×1 bar is notched at x ∈ [9, 11] through its front top edge,
/// which leaves that edge two pieces, P1 and P2, of one line. A U-shaped
/// cutter then cuts both, its prongs at x ∈ [3, 4] across P1 and at
/// x ∈ [15, 16] across P2: four pieces of the line at the cut. Shortening
/// the first prong so it stops short of the bar leaves P1 whole and P2
/// cut, three. Each vanished piece of P1 reads the line's group, 4 → 3,
/// with the prong's walls as the cutters gone: two untied parents on one
/// line are not a tie, and their groups are not two groups of unequal
/// size the rung cannot state. The runs are read without their verdict
/// logs and against one document, as the split row is.
#[test]
fn the_pieces_of_one_line_from_two_parents_are_one_group() {
    let doc = ProfileDoc::empty_derived("group-membership-line", Tol::witness());
    let (doc, bar) = block(doc, (0.0, 20.0), (0.0, 2.0), 1.0);
    let (doc, notch) = block_on(doc, bar, (9.0, 11.0), (-1.0, 0.5), 0.5, 1.0);
    let (doc, notched) = insert(
        doc,
        Node::Subtract {
            from: bar.into(),
            tool: notch.into(),
        },
    );
    let u = |reach: f64| {
        vec![
            (3.0, -2.0),
            (16.0, -2.0),
            (16.0, 0.5),
            (15.0, 0.5),
            (15.0, -1.5),
            (4.0, -1.5),
            (4.0, reach),
            (3.0, reach),
        ]
    };
    let (doc, u_profile) = insert(doc, Node::Profile(crate::fixture::desc(vec![u(0.5)])));
    let (doc, cutter) = insert(
        doc,
        Node::Extrude {
            profile: u_profile.into(),
            distance: len(1.0),
        },
    );
    let (doc, cutter) = crate::fixture::slid(doc, bar, cutter, [0.0, 0.0, 0.5]);
    let (doc, cut) = insert(
        doc,
        Node::Subtract {
            from: notched.into(),
            tool: cutter.into(),
        },
    );
    let Some(Node::Profile(p)) = doc.node(u_profile) else {
        panic!("the cutter's profile");
    };
    let ids = p
        .ids
        .iter()
        .map(|l| l.iter().copied().map(Some).collect())
        .collect();
    let (doc2, _) = step(
        doc.clone(),
        DocEdit::SetProgram {
            node: u_profile,
            loops: vec![editor_core::LoopProgram::polygon(u(-0.5)).unwrap()],
            ids,
            fresh: Vec::new(),
        },
    );
    let ev1 = run(&doc, None);
    let ev2 = silent(run(&doc2, Some(&ev1)));
    let ev1 = silent(ev1);
    let from_read = crate::fixture::out(&doc, notched);
    let pieces = |ev: &Evaluation<f64>| {
        ev.table_at(cut)
            .expect("the cut evaluates")
            .iter()
            .filter(|(n, _)| {
                n.kind == EntityKind::Edge
                    && matches!(
                        n.path.as_slice(),
                        [
                            RoleSeg::From { read, .. },
                            RoleSeg::Fragment(editor_core::Qualifier::Ends(_))
                        ] if *read == from_read
                    )
            })
            .count()
    };
    let rows = vanished((&doc2, &ev2), (&doc2, &ev1), cut, |n, e| {
        matches!(e, Entry::Unique(_))
            && n.kind == EntityKind::Edge
            && matches!(
                n.path.as_slice(),
                [
                    RoleSeg::From { read, .. },
                    RoleSeg::Fragment(editor_core::Qualifier::Ends(_))
                ] if *read == from_read
            )
    });
    assert_eq!(
        rows.len(),
        2,
        "P1's two pieces vanish ({} pieces before, {} after): {rows:?}",
        pieces(&ev1),
        pieces(&ev2)
    );
    for (n, d) in rows {
        let Diagnosis::GroupResized {
            node,
            was,
            now,
            cutters: GroupCutters::Read { gone, new },
        } = d
        else {
            panic!("{n:?}: the line's group resized, not {d:?}");
        };
        assert_eq!(
            (node, was, now),
            (cut, 4, 3),
            "{n:?}: the line's four pieces became three"
        );
        assert!(
            !gone.is_empty() && new.is_empty(),
            "{n:?}: the prong's walls are the cutters gone: gone {gone:?}, new {new:?}"
        );
    }
}
