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
    Axis3, BooleanOp, CancelToken, Datum, Diagnosis, DocEdit, Entry, EvalOptions, Evaluation,
    GroupCutters, Node, ProfileDoc, RecipeNodeId, Resolution, ResolveError, RoleSeg, RunCtx,
    SlotId, StableName, evaluate, resolve_with_prior,
};
use fixture::{ang, insert, len, on_frame, scl, step};
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
                cutters: GroupCutters::TiedParents,
            },
            "{n:?}"
        );
    }
}

/// The ladder's evidence-free fallback at `node`: every rung declined.
fn fallback(node: RecipeNodeId) -> Diagnosis {
    Diagnosis::RecipeEdit {
        edit: editor_core::RecipeEditRef::NodeChanged { node },
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
/// A 3×3×1 plate, a bar standing through its top in y (behind a
/// `Transform`), and a block far away, united in every order: the bar
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
        // Member `member`'s wall `segment` (of the block `of`), as the
        // union's seams spell it.
        let member_wall = |member, of, segment| StableName {
            kind: editor_core::EntityKind::Face,
            node: u,
            path: vec![RoleSeg::FromMember {
                member,
                of: editor_core::NameRef::new(wall(&doc, of, segment)),
            }],
        };
        let from = |n: &StableName, m: RecipeNodeId| matches!(n.path.first(), Some(RoleSeg::FromMember { member, .. }) if *member == m);
        for (axis, by, ranked, gone, new) in [
            (Axis3::Y, 2.5, false, vec![], vec![member_wall(tr, bar, 0)]),
            (Axis3::X, 1.5, true, vec![member_wall(tr, bar, 1)], vec![]),
        ] {
            let ev1 = run(&doc, None);
            let doc2 = set(doc.clone(), tr, SlotId::Translation(axis), by);
            // The slide records a containment flip at the union, the
            // answer above this rung; read the two real runs without
            // that evidence, as the split row does.
            let ev2 = silent(run(&doc2, Some(&ev1)));
            let ev1 = silent(ev1);
            let rows = vanished((&doc, &ev2), (&doc, &ev1), u, |n, e| {
                matches!(e, Entry::Unique(_))
                    && match n.path.last() {
                        Some(RoleSeg::Fragment(editor_core::Qualifier::SideOf(_))) => !ranked,
                        Some(RoleSeg::Fragment(editor_core::Qualifier::OrderAlong { .. })) => {
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
                    assert!(from(&n, tr) && !ranked, "{order:?} {axis:?}: {n:?}");
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
    let rows = vanished((&doc, &ev2), (&doc, &ev1), cut, |n, _| {
        match n.path.first() {
            Some(RoleSeg::Seam { a, .. }) => matches!(tied.lookup(a), Some(Entry::Tied(_))),
            _ => false,
        }
    });
    assert!(
        !rows.is_empty(),
        "no tied seam fragment vanished, so the row pins nothing"
    );
    // With no flip and no edit in evidence, the fallback: the rung
    // declined.
    for (n, d) in rows {
        assert_eq!(
            d,
            fallback(cut),
            "{n:?}: a tie-summed seam group was counted"
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
        let rows = vanished(
            (&doc, &ev2),
            (&doc, &ev1),
            u,
            |n, _| matches!(n.path.first(), Some(RoleSeg::FromMember { member, .. }) if *member == plate),
        );
        assert!(rows.is_empty(), "{label}: {rows:?}");
        for ev in [&ev1, &ev2] {
            let t = &ev.value(u).expect("the union evaluates").name_table;
            let plate_faces: Vec<_> = t
                .iter()
                .filter(|(n, _)| {
                    n.kind == editor_core::EntityKind::Face
                        && matches!(n.path.first(), Some(RoleSeg::FromMember { member, .. }) if *member == plate)
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
        let (doc, sub, tr, _) = tied_prongs_cut();
        let (doc, far) = block(doc, (20.0, 21.0), (0.0, 1.0), 0.0, 1.0);
        let m = [sub, tr, far];
        let (doc, u) = insert(
            doc,
            Node::Union {
                members: order.iter().map(|&i| m[i]).collect(),
                declare: None,
            },
        );
        let ev1 = run(&doc, None);
        let doc2 = set(doc.clone(), tr, SlotId::Translation(Axis3::Z), 3.2);
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

/// Wall `segment` of the block `node` extruded (`block`'s profile runs
/// counter-clockwise from `(x0, y0)`: wall 0 is y = y0, 1 is x = x1, 2
/// is y = y1, 3 is x = x0).
fn wall(doc: &ProfileDoc, node: RecipeNodeId, segment: u32) -> StableName {
    StableName {
        kind: editor_core::EntityKind::Face,
        node,
        path: vec![RoleSeg::Lateral(crate::fixture::piece(
            doc,
            node,
            0,
            segment as usize,
        ))],
    }
}

/// The plate-and-bar fixture as a PAIR union: a 3×3×1 plate, and a bar
/// standing through its top across the whole plate in y, behind a
/// `Transform`. The bar's two x walls divide the plate's top in two,
/// and each of the top's two x-running rim edges.
/// Answers (doc, the plate, the bar, the bar's transform, the union).
fn plate_and_bar() -> (
    ProfileDoc,
    RecipeNodeId,
    RecipeNodeId,
    RecipeNodeId,
    RecipeNodeId,
) {
    let doc = ProfileDoc::empty_derived("group-cutters", Tol::witness());
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
    let (doc, u) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a: plate,
            b: tr,
            declare: None,
        },
    );
    (doc, plate, bar, tr, u)
}

/// Whether `n` is the pair union's row descended from the plate's
/// entity spelled `seg`.
fn from_plate(n: &StableName, plate: RecipeNodeId, seg: &RoleSeg) -> bool {
    matches!(n.path.first(), Some(RoleSeg::FromA(p)) if p.node == plate && p.path == [seg.clone()])
}

const TOP: RoleSeg = RoleSeg::Cap(editor_core::CapEnd::End);

/// The plate's top rim edge over profile segment `segment`.
fn rim(doc: &ProfileDoc, plate: RecipeNodeId, segment: u32) -> RoleSeg {
    RoleSeg::RimEdge(
        editor_core::CapEnd::End,
        crate::fixture::piece(doc, plate, 0, segment as usize),
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
    let doc2 = set(doc.clone(), tr, SlotId::Translation(Axis3::X), dx);
    let doc2 = set(doc2, tr, SlotId::Translation(Axis3::Y), dy);
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
    let (bar, u, rows) = slid((1.5, 0.0), &[rim(&doc, plate, 0), rim(&doc, plate, 2)]);
    for (n, d) in rows {
        assert_eq!(d, two_to_one(u, vec![wall(&doc, bar, 1)], vec![]), "{n:?}");
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
    // `slid` builds this same recipe, so its pieces are these.
    let (doc, ..) = plate_and_bar();
    let (bar, u, rows) = slid((0.0, 2.5), &[TOP]);
    for (n, d) in rows {
        assert_eq!(d, two_to_one(u, vec![], vec![wall(&doc, bar, 0)]), "{n:?}");
    }
}

/// **Two cutters that change at once are both named.**
///
/// Slid 1.5 in x and 2.5 in y, the bar covers only the plate's far
/// corner: its x = x1 wall stops cutting the top and its y = y0 wall
/// starts. Of the top's two vanished fragments, the one on the far
/// side of that x wall is answered by the flip of its side verdict
/// against it, a cause, above this rung; the other by the rung, naming
/// both. Slid 5 in y, clear of the plate, both x walls stop cutting the
/// top and its rim edges. Each list holds every cutter it states.
#[test]
fn two_cutters_that_change_at_once_are_both_named() {
    // `slid` builds this same recipe, so its pieces are these.
    let (doc, plate, ..) = plate_and_bar();
    let (bar, u, rows) = slid((1.5, 2.5), &[TOP]);
    let mut answered = 0;
    for (n, d) in rows {
        if matches!(d, Diagnosis::PredicateFlip { .. }) {
            continue;
        }
        answered += 1;
        assert_eq!(
            d,
            two_to_one(u, vec![wall(&doc, bar, 1)], vec![wall(&doc, bar, 0)]),
            "{n:?}"
        );
    }
    assert_eq!(answered, 1, "the near fragment is the rung's to answer");
    let (bar, u, rows) = slid((0.0, 5.0), &[TOP, rim(&doc, plate, 0), rim(&doc, plate, 2)]);
    let mut gone = vec![wall(&doc, bar, 1), wall(&doc, bar, 3)];
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
        let (doc, c) = block(doc, (0.5, 1.5), (-0.8, -0.5), 0.0, 2.0);
        let m = [tr, c];
        let (doc, u) = insert(
            doc,
            Node::Union {
                members: vec![m[order[0]], m[order[1]], plate],
                declare: None,
            },
        );
        let ev1 = run(&doc, None);
        let doc2 = set(doc.clone(), tr, SlotId::Translation(Axis3::X), 1.5);
        let ev2 = silent(run(&doc2, Some(&ev1)));
        let ev1 = silent(ev1);
        let rows = vanished((&doc, &ev2), (&doc, &ev1), u, |n, e| {
            matches!(e, Entry::Unique(_))
                && n.kind == editor_core::EntityKind::Edge
                && matches!(n.path.first(),
                    Some(RoleSeg::FromMember { member, .. }) if *member == plate)
        });
        assert!(
            !rows.is_empty(),
            "{order:?}: no rim fragment vanished, so the row pins nothing"
        );
        let x1_wall = StableName {
            kind: editor_core::EntityKind::Face,
            node: u,
            path: vec![RoleSeg::FromMember {
                member: tr,
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
