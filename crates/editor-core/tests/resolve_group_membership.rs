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
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use editor_core::{
    Axis3, BooleanOp, CancelToken, Datum, Diagnosis, DocEdit, Entry, EvalOptions, Evaluation, Node,
    ProfileDoc, RecipeNodeId, Resolution, ResolveError, RoleSeg, RunCtx, SlotId, StableName,
    evaluate, resolve_with_prior,
};
use fixture::{ang, insert, len, on_frame, scl, step};
use geom_core::Tol;

fn run(doc: &ProfileDoc, prior: Option<&Evaluation<f64>>) -> Evaluation<f64> {
    evaluate::<f64>(
        doc,
        prior,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

/// A prism: the polygon `pts` on z = `z0`, extruded `dz`.
fn prism(doc: ProfileDoc, pts: Vec<(f64, f64)>, z0: f64, dz: f64) -> (ProfileDoc, RecipeNodeId) {
    let (doc, p) = on_frame(
        doc,
        [0.0, 0.0, z0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![pts],
    );
    insert(
        doc,
        Node::Extrude {
            profile: p,
            distance: len(dz),
        },
    )
}

fn block(
    doc: ProfileDoc,
    (x0, x1): (f64, f64),
    (y0, y1): (f64, f64),
    z0: f64,
    dz: f64,
) -> (ProfileDoc, RecipeNodeId) {
    prism(doc, vec![(x0, y0), (x1, y0), (x1, y1), (x0, y1)], z0, dz)
}

fn set(doc: ProfileDoc, node: RecipeNodeId, slot: SlotId, to: f64) -> ProfileDoc {
    let expr = match slot {
        SlotId::Normal(_) => scl(to),
        _ => len(to),
    };
    step(doc, DocEdit::SetParam { node, slot, expr }).0
}

/// The fragment names `node` minted in `ev1` that `ev2` no longer
/// carries, each with its diagnosis.
fn vanished(
    (doc2, ev2): (&ProfileDoc, &Evaluation<f64>),
    (doc1, ev1): (&ProfileDoc, &Evaluation<f64>),
    node: RecipeNodeId,
    pick: impl Fn(&StableName, &Entry) -> bool,
) -> Vec<(StableName, Diagnosis)> {
    let now = &ev2.value(node).expect("the node evaluates now").name_table;
    ev1.value(node)
        .expect("the node evaluated before")
        .name_table
        .iter()
        .filter(|(n, e)| matches!(n.path.last(), Some(RoleSeg::Fragment(_))) && pick(n, e))
        .filter(|(n, _)| now.lookup(n).is_none())
        .map(|(n, _)| {
            let res = resolve_with_prior(
                RunCtx {
                    doc: doc2,
                    eval: ev2,
                },
                RunCtx {
                    doc: doc1,
                    eval: ev1,
                },
                n,
                Tol::witness(),
            );
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

/// **Two tied parents, each cut in two, are two groups of two.**
///
/// A 4×4×4 block minus a U-shaped cutter whose two prongs cross one
/// wall: the cutter's caps leave two congruent prong faces each, an N2
/// tie. A bar across both prongs cuts each tied face in two; sliding
/// the bar up out of the cavity (it still cuts the block's top) leaves
/// them whole. The emitter formed one group per tied face, two members
/// each, and one member each after the slide: 2 → 1. The spelling
/// counts the tie lane's merged rows, one group of four: 4 → 2.
/// The tie fixture: a 4×4×4 block minus a U-shaped cutter whose two
/// prongs cross one wall — the cutter's caps leave two congruent prong
/// faces each, an N2 tie — cut again by a bar across both prongs,
/// behind a `Transform`. Answers (doc, the first cut, the bar's
/// transform, the second cut).
fn tied_prongs_cut() -> (ProfileDoc, RecipeNodeId, RecipeNodeId, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("group-membership-tie", Tol::witness());
    let (doc, a) = block(doc, (0.0, 4.0), (0.0, 4.0), 0.0, 4.0);
    let (doc, u) = prism(
        doc,
        vec![
            (2.0, 1.0),
            (6.0, 1.0),
            (6.0, 3.0),
            (2.0, 3.0),
            (2.0, 2.5),
            (5.0, 2.5),
            (5.0, 1.5),
            (2.0, 1.5),
        ],
        1.0,
        2.0,
    );
    let (doc, sub) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Subtract,
            a,
            b: u,
            declare: None,
        },
    );
    let (doc, bar) = block(doc, (2.9, 3.1), (0.5, 3.5), 0.5, 3.0);
    let (doc, tr) = insert(
        doc,
        Node::Transform {
            input: bar,
            translation: [len(0.0), len(0.0), len(0.0)],
            rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
            rotation_angle: ang(0.0),
        },
    );
    let (doc, cut) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Subtract,
            a: sub,
            b: tr,
            declare: None,
        },
    );
    (doc, sub, tr, cut)
}

#[test]
fn two_tied_parents_each_cut_in_two_are_two_groups_of_two() {
    let (doc, _, tr, cut) = tied_prongs_cut();
    let ev1 = run(&doc, None);
    let doc2 = set(doc.clone(), tr, SlotId::Translation(Axis3::Z), 3.2);
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
        0.0,
        1.0,
    );
    let (doc, tool) = insert(
        doc,
        Node::Datum(Datum::Plane {
            origin: [len(2.5), len(0.0), len(0.0)],
            normal: [scl(1.0), scl(1.0), scl(0.0)],
        }),
    );
    let (doc, split) = insert(doc, Node::Split { target: ext, tool });
    let ev1 = run(&doc, None);
    let mut doc2 = doc.clone();
    for (slot, to) in [
        (SlotId::Normal(Axis3::Z), 1.0),
        (SlotId::Normal(Axis3::X), 0.0),
        (SlotId::Normal(Axis3::Y), 0.0),
        (SlotId::Origin(Axis3::X), 0.0),
        (SlotId::Origin(Axis3::Z), 0.5),
    ] {
        doc2 = set(doc2, tool, slot, to);
    }
    let ev2 = silent(run(&doc2, Some(&ev1)));
    let ev1 = silent(ev1);
    let rows = vanished((&doc, &ev2), (&doc, &ev1), split, |n, _| {
        matches!(n.path.first(), Some(RoleSeg::SplitFragment { .. }))
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
            },
            "{n:?}"
        );
    }
    assert!(
        top > 0,
        "no top-cap fragment vanished, so the pass-through is not pinned"
    );
}

/// **A union's fragment group is read through its fold.**
///
/// A 3×3×1 plate, a bar standing through its top in y (behind a
/// `Transform`), and a block far away, united in every order: the bar
/// divides the plate's top into two fragments at whichever fold step
/// the plate and the bar meet, and every later step carries them
/// through. Sliding the bar in y so it stops short of the far edge
/// leaves the top whole. The union's record is each step's groups,
/// followed to the published names: 2 → 1 in every order. The two runs
/// are read without their verdict logs and against one document, for
/// the split row's reason: the slide's own flip is diagnosed first.
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
        let (doc, plate) = block(doc, (0.0, 3.0), (0.0, 3.0), 0.0, 1.0);
        let (doc, bar) = block(doc, (1.0, 2.0), (-1.0, 4.0), 0.5, 1.0);
        let (doc, tr) = insert(
            doc,
            Node::Transform {
                input: bar,
                translation: [len(0.0), len(0.0), len(0.0)],
                rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
                rotation_angle: ang(0.0),
            },
        );
        let (doc, far) = block(doc, (10.0, 11.0), (0.0, 1.0), 0.0, 1.0);
        let m = [plate, tr, far];
        let (doc, u) = insert(
            doc,
            Node::Union {
                members: order.iter().map(|&i| m[i]).collect(),
                declare: None,
            },
        );
        let ev1 = run(&doc, None);
        let doc2 = set(doc.clone(), tr, SlotId::Translation(Axis3::Y), 2.5);
        // The slide records a containment flip at the union, the answer
        // above this rung; read the two real runs without that
        // evidence, as the split row does.
        let ev2 = silent(run(&doc2, Some(&ev1)));
        let ev1 = silent(ev1);
        let rows = vanished((&doc, &ev2), (&doc, &ev1), u, |n, e| {
            matches!(e, Entry::Unique(_))
                && matches!(
                    n.path.last(),
                    Some(RoleSeg::Fragment(editor_core::Qualifier::SideOf(_)))
                )
        });
        assert!(
            !rows.is_empty(),
            "{order:?}: no fragment vanished, so the row pins nothing"
        );
        for (n, d) in rows {
            assert_eq!(
                d,
                Diagnosis::GroupResized {
                    node: u,
                    was: 2,
                    now: 1,
                },
                "{order:?}: {n:?}"
            );
        }
    }
}

/// **A seam group tied parents share has no one parent's count.**
///
/// The seam lanes group a seam edge by its two faces' NAMES, so the bar
/// crossing the two tied prong faces makes one group of the pieces of
/// both. The record marks it, and the rung declines for its vanished
/// fragments rather than state the tie's sum as a group's size.
#[test]
fn a_seam_group_tied_parents_share_is_not_counted() {
    let (doc, sub, tr, cut) = tied_prongs_cut();
    let ev1 = run(&doc, None);
    let doc2 = set(doc.clone(), tr, SlotId::Translation(Axis3::Z), 3.2);
    let ev2 = silent(run(&doc2, Some(&ev1)));
    let ev1 = silent(ev1);
    // A seam whose A face is one of the first cut's TIED rows.
    let tied = &ev1.value(sub).expect("the first cut evaluates").name_table;
    let rows = vanished((&doc, &ev2), (&doc, &ev1), cut, |n, _| match n.path.first() {
        Some(RoleSeg::Seam { a, .. }) => matches!(tied.lookup(a), Some(Entry::Tied(_))),
        _ => false,
    });
    assert!(
        !rows.is_empty(),
        "no tied seam fragment vanished, so the row pins nothing"
    );
    for (n, d) in rows {
        assert!(
            !matches!(d, Diagnosis::GroupResized { .. }),
            "{n:?}: a tie-summed seam group was counted: {d:?}"
        );
    }
}

/// **A union counts what its later fold steps left of a group.**
///
/// The bar divides the plate's top into two at the first fold step,
/// and the block C, united at the next, swallows one of the two pieces
/// — so the published body holds ONE entity of that group, in both
/// runs, before and after the bar slides. Its rim edges the same. A
/// count taken where the group was formed would say 2 → 1; the
/// published body says 1 → 1, and the rung declines. Both layouts: C
/// over the far piece, and C over the near one.
#[test]
fn a_union_group_a_later_step_partly_swallows_counts_what_is_published() {
    for (label, c) in [
        ("far", ((1.7, 4.0), (-1.5, 4.5), 0.3, 1.4)),
        ("near", ((-1.0, 1.3), (-1.5, 4.5), 0.3, 1.4)),
    ] {
        let doc = ProfileDoc::empty_derived("group-membership-swallow", Tol::witness());
        let (doc, plate) = block(doc, (0.0, 3.0), (0.0, 3.0), 0.0, 1.0);
        let (doc, bar) = block(doc, (1.0, 2.0), (-1.0, 4.0), 0.5, 1.0);
        let (doc, tr) = insert(
            doc,
            Node::Transform {
                input: bar,
                translation: [len(0.0), len(0.0), len(0.0)],
                rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
                rotation_angle: ang(0.0),
            },
        );
        let (doc, cblock) = block(doc, c.0, c.1, c.2, c.3);
        let (doc, u) = insert(
            doc,
            Node::Union {
                members: vec![plate, tr, cblock],
                declare: None,
            },
        );
        let ev1 = run(&doc, None);
        let doc2 = set(doc.clone(), tr, SlotId::Translation(Axis3::Y), 2.5);
        let ev2 = silent(run(&doc2, Some(&ev1)));
        let ev1 = silent(ev1);
        // The plate's own entities: its top's fragments and its rim.
        let rows = vanished((&doc, &ev2), (&doc, &ev1), u, |n, _| {
            matches!(n.path.first(), Some(RoleSeg::FromMember { member, .. }) if *member == plate)
        });
        assert!(
            !rows.is_empty(),
            "{label}: no fragment vanished, so the row pins nothing"
        );
        for (n, d) in rows {
            assert!(
                !matches!(d, Diagnosis::GroupResized { was: 2, now: 1, .. }),
                "{label}: {n:?} counted at the step that formed it: {d:?}"
            );
        }
    }
}
