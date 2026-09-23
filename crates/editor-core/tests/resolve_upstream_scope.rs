//! The with-history diagnosis lanes read two scopes: the vanished
//! name's derivation path, then the minting node's ANCESTORS outside
//! it (`Diagnosis::Upstream`). A node the minting node does not depend
//! on is never read. These rows pin both edges of that rule on real
//! documents; the sentences are pinned in `display_contract`.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use editor_core::{
    Axis3, BooleanOp, CancelToken, Diagnosis, DocEdit, Entry, EvalOptions, Evaluation, Node,
    ProfileDoc, Qualifier, RecipeNodeId, Resolution, ResolveError, RoleSeg, RunCtx, SlotId,
    StableName, UpstreamCause, evaluate, resolve_with_prior,
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

fn block(
    doc: ProfileDoc,
    (x0, x1): (f64, f64),
    (y0, y1): (f64, f64),
    z0: f64,
    dz: f64,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, p) = on_frame(
        doc,
        [0.0, 0.0, z0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(x0, y0), (x1, y0), (x1, y1), (x0, y1)]],
    );
    insert(
        doc,
        Node::Extrude {
            profile: p,
            distance: len(dz),
        },
    )
}

fn placed(doc: ProfileDoc, input: RecipeNodeId) -> (ProfileDoc, RecipeNodeId) {
    insert(
        doc,
        Node::Transform {
            input,
            translation: [len(0.0), len(0.0), len(0.0)],
            rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
            rotation_angle: ang(0.0),
        },
    )
}

fn slide(doc: ProfileDoc, node: RecipeNodeId, axis: Axis3, to: f64) -> ProfileDoc {
    step(
        doc,
        DocEdit::SetParam {
            node,
            slot: SlotId::Translation(axis),
            expr: len(to),
        },
    )
    .0
}

/// A 3×3×1 plate at x offset `dx` with a bar crossing its end cap
/// fully in y, behind a `Transform`, subtracted: (doc, bar's
/// transform, the cut).
fn slot(doc: ProfileDoc, dx: f64) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let (doc, a) = block(doc, (dx, dx + 3.0), (0.0, 3.0), 0.0, 1.0);
    let (doc, b0) = block(doc, (dx + 1.0, dx + 2.0), (-1.0, 4.0), 0.5, 1.0);
    let (doc, tr) = placed(doc, b0);
    let (doc, cut) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Subtract,
            a,
            b: tr,
            declare: None,
        },
    );
    (doc, tr, cut)
}

/// The unique fragment names of `node`'s table, in table order.
fn fragments(ev: &Evaluation<f64>, node: RecipeNodeId) -> Vec<StableName> {
    ev.value(node)
        .expect("the node evaluates")
        .name_table
        .iter()
        .filter(|(n, e)| {
            matches!(n.path.last(), Some(RoleSeg::Fragment(_))) && matches!(e, Entry::Unique(_))
        })
        .map(|(n, _)| n.clone())
        .collect()
}

fn diagnosis(
    (doc2, ev2): (&ProfileDoc, &Evaluation<f64>),
    (doc1, ev1): (&ProfileDoc, &Evaluation<f64>),
    name: &StableName,
) -> Diagnosis {
    let res = resolve_with_prior(
        RunCtx {
            doc: doc2,
            eval: ev2,
        },
        RunCtx {
            doc: doc1,
            eval: ev1,
        },
        name,
        Tol::witness(),
    );
    let Resolution::Failed(f) = res else {
        panic!("{name:?}: expected Failed, got {res:?}");
    };
    let ResolveError::Vanished { diagnosis, .. } = f.error else {
        panic!("{name:?}: expected Vanished, got {:?}", f.error);
    };
    diagnosis
}

#[test]
fn a_flip_at_a_node_the_name_does_not_depend_on_is_not_its_cause() {
    // Two independent plates-with-bars in one document. The first
    // bar slides along its cap in y (the cap's fragment group goes
    // from two to one, no flip); the second, unrelated one withdraws
    // in x, which records a containment flip at the SECOND cut. That
    // flip is on no path and upstream of nothing in the first scene,
    // so the first scene's fragments are answered by their own
    // evidence: the group-size fact.
    let doc = ProfileDoc::empty_derived("upstream-scope", Tol::witness());
    let (doc, tr1, cut1) = slot(doc, 0.0);
    let (doc, tr2, cut2) = slot(doc, 100.0);
    let ev1 = run(&doc, None);
    let names = fragments(&ev1, cut1);
    assert!(!names.is_empty(), "the first cut mints fragments");
    let doc2 = slide(slide(doc.clone(), tr1, Axis3::Y, 2.5), tr2, Axis3::X, 5.0);
    let ev2 = run(&doc2, Some(&ev1));
    let flips = editor_core::diff_verdicts(&ev1, &ev2).report();
    assert!(
        flips.iter().any(|(n, _)| *n == cut2),
        "the unrelated edit records a flip at the second cut: {flips:?}"
    );
    let mut vanished = 0;
    for name in &names {
        if ev2.value(cut1).unwrap().name_table.lookup(name).is_some() {
            continue;
        }
        vanished += 1;
        assert_eq!(
            diagnosis((&doc2, &ev2), (&doc, &ev1), name),
            Diagnosis::GroupResized {
                node: cut1,
                was: 2,
                now: 1,
            },
            "{name:?}"
        );
    }
    assert!(vanished > 0, "the slide vanishes some fragment");
}

#[test]
fn a_flip_upstream_of_the_minting_node_is_reported_as_upstream() {
    // The cutter is itself a union of two bars (`cutter`), and the
    // plate's rim edges are ranked fragments (`OrderAlong`, which
    // mention no partner), so the cutter's union is UPSTREAM of the
    // cut but not on those names' derivation path. Sliding the second
    // bar clear of the first records a flip at the cutter's union and
    // leaves the rim edges undivided. The flip is a candidate cause —
    // it fed the cut — and it is reported as that, with its node, in
    // the upstream scope rather than as a path flip.
    let doc = ProfileDoc::empty_derived("upstream-scope", Tol::witness());
    let (doc, a) = block(doc, (0.0, 3.0), (0.0, 3.0), 0.0, 1.0);
    let (doc, b1) = block(doc, (1.0, 2.0), (-1.0, 2.0), 0.5, 1.0);
    let (doc, b2) = block(doc, (1.2, 1.8), (1.0, 4.0), 0.4, 1.2);
    let (doc, tr) = placed(doc, b2);
    let (doc, cutter) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a: b1,
            b: tr,
            declare: None,
        },
    );
    let (doc, cut) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Subtract,
            a,
            b: cutter,
            declare: None,
        },
    );
    let ev1 = run(&doc, None);
    let doc2 = slide(doc.clone(), tr, Axis3::Y, 5.0);
    let ev2 = run(&doc2, Some(&ev1));
    let ranked: Vec<StableName> = fragments(&ev1, cut)
        .into_iter()
        .filter(|n| {
            matches!(
                n.path.last(),
                Some(RoleSeg::Fragment(Qualifier::OrderAlong { .. }))
            ) && ev2.value(cut).unwrap().name_table.lookup(n).is_none()
        })
        .collect();
    assert!(!ranked.is_empty(), "some ranked rim edge vanishes");
    for name in &ranked {
        assert!(
            !editor_core::derivation_nodes(name).contains(&cutter),
            "the cutter's union is not on {name:?}'s derivation path"
        );
        match diagnosis((&doc2, &ev2), (&doc, &ev1), name) {
            Diagnosis::Upstream {
                node,
                cause: UpstreamCause::PredicateFlip { predicate, at, .. },
            } => {
                assert_eq!(node, cut, "upstream of the minting node");
                assert_eq!(at, cutter, "the flip is the cutter's own");
                assert_eq!(predicate, "bool_point_in_solid_plane");
            }
            other => panic!("{name:?}: expected the upstream flip, got {other:?}"),
        }
    }
}
