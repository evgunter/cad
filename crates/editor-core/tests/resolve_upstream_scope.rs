//! The with-history diagnosis lanes read two scopes: the vanished
//! name's derivation path, then the minting node's ANCESTORS outside
//! it (`Diagnosis::Upstream`). A node the minting node does not depend
//! on is never read. These rows pin both edges of that rule on real
//! documents; the sentences are pinned in `display_contract`.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use std::collections::BTreeSet;
use std::sync::Arc;

use editor_core::eval::WitnessSlot;
use editor_core::{
    Axis3, BooleanOp, CancelToken, ContentKey, Diagnosis, DocEdit, EntityKind, Entry, EvalOptions,
    EvalOutcome, Evaluation, NameTable, NamingKey, Node, ProfileDoc, Qualifier, RecipeEditRef,
    RecipeNodeId, Resolution, ResolveError, RoleSeg, RunCtx, SlotId, StableName, UpstreamCause,
    evaluate, resolve_with_prior,
};
use fixture::{ang, insert, len, on_frame, scl, step};
use geom_core::Sign;
use geom_core::Tol;
use geom_core::k_stats::Verdict;

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

fn set_members(doc: ProfileDoc, node: RecipeNodeId, members: Vec<RecipeNodeId>) -> ProfileDoc {
    step(doc, DocEdit::SetMembers { node, members }).0
}

/// The strict ancestors of `node` in ONE document — the test's own
/// reading of "fed the minting node in this run", independent of the
/// walk under test.
fn ancestors_in(doc: &ProfileDoc, node: RecipeNodeId) -> BTreeSet<RecipeNodeId> {
    let mut seen = BTreeSet::new();
    let mut stack = doc.node(node).map(|n| n.inputs()).unwrap_or_default();
    while let Some(n) = stack.pop() {
        if seen.insert(n) {
            stack.extend(doc.node(n).map(|x| x.inputs()).unwrap_or_default());
        }
    }
    seen
}

/// The first vanished ranked (`OrderAlong`) fragment of `cut`'s prior
/// table — names that mention no partner, so nothing but `cut` and
/// the plate's own extrude is on their derivation path.
fn vanished_ranked(ev1: &Evaluation<f64>, ev2: &Evaluation<f64>, cut: RecipeNodeId) -> StableName {
    fragments(ev1, cut)
        .into_iter()
        .find(|n| {
            matches!(
                n.path.last(),
                Some(RoleSeg::Fragment(Qualifier::OrderAlong { .. }))
            ) && ev2
                .value(cut)
                .expect("the cut evaluates")
                .name_table
                .lookup(n)
                .is_none()
        })
        .expect("some ranked fragment of the cut vanishes")
}

/// The reviewer's two-run chain. Last-good: `cut = a − X`, `X =
/// Union[tr, P]`, `P = Union[c1, c2]`; `R`, an unrelated plate-with-
/// bar, is built first (lowest id). The edit re-lists `X → [tr, c3]`
/// and `P → [c1, R]`, and slides `R`'s bar (a flip at `R`) and the
/// cutter along y. `R` feeds `cut` in NEITHER run — only a walk that
/// crosses from the old `X → P` edge to the new `P → R` edge reaches
/// it — while `P` fed `cut` in the last-good run and flips.
#[test]
fn an_ancestor_is_one_in_either_run_walked_within_that_run() {
    let doc = ProfileDoc::empty_derived("upstream-scope", Tol::witness());
    let (doc, r_tr, r) = slot(doc, 40.0);
    let (doc, a) = block(doc, (0.0, 3.0), (0.0, 3.0), 0.0, 1.0);
    let (doc, m) = block(doc, (1.0, 2.0), (-1.0, 4.0), 0.5, 1.0);
    let (doc, tr) = placed(doc, m);
    let (doc, c1) = block(doc, (10.0, 12.0), (0.0, 2.0), 0.0, 1.0);
    let (doc, c2) = block(doc, (11.0, 13.0), (0.5, 2.5), 0.2, 1.2);
    let (doc, c3) = block(doc, (20.0, 21.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, p) = insert(
        doc,
        Node::Union {
            members: vec![c1, c2],
            declare: None,
        },
    );
    let (doc, x) = insert(
        doc,
        Node::Union {
            members: vec![tr, p],
            declare: None,
        },
    );
    let (doc, cut) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Subtract,
            a,
            b: x,
            declare: None,
        },
    );
    let ev1 = run(&doc, None);
    let doc2 = set_members(set_members(doc.clone(), x, vec![tr, c3]), p, vec![c1, r]);
    let doc2 = slide(slide(doc2, r_tr, Axis3::X, 5.0), tr, Axis3::Y, 2.5);
    let ev2 = run(&doc2, Some(&ev1));
    // The premises, each read per document.
    assert!(!ancestors_in(&doc, cut).contains(&r) && !ancestors_in(&doc2, cut).contains(&r));
    assert!(ancestors_in(&doc, cut).contains(&p) && !ancestors_in(&doc2, cut).contains(&p));
    let flips = editor_core::diff_verdicts(&ev1, &ev2).report();
    assert!(flips.iter().any(|(n, _)| *n == r), "R flips: {flips:?}");
    assert!(
        r < p,
        "R is first in deterministic order: a walk reaching it reports it"
    );
    let name = vanished_ranked(&ev1, &ev2, cut);
    match diagnosis((&doc2, &ev2), (&doc, &ev1), &name) {
        Diagnosis::Upstream {
            node,
            cause: UpstreamCause::PredicateFlip { at, .. },
        } => {
            assert_eq!(node, cut);
            assert_eq!(at, p, "the last-good-only ancestor, never R");
        }
        other => panic!("expected an upstream flip at P, got {other:?}"),
    }
}

/// The other side of "either run": a node that feeds the minting node
/// only in the CURRENT run. Real documents, hand-built runs: `w` is a
/// block the edit swaps into the union `x` that `n = Transform(x)`
/// places, and the only recorded flip is at `w`. A walk of the
/// last-good document alone never reaches `w` and would answer the
/// union's recipe edit instead.
#[test]
fn a_node_that_feeds_the_name_only_now_is_upstream_too() {
    let doc = ProfileDoc::empty_derived("upstream-scope", Tol::witness());
    let (doc, w) = block(doc, (0.2, 0.8), (0.2, 0.8), 0.2, 0.6);
    let (doc, b1) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b2) = block(doc, (0.1, 0.9), (0.1, 0.9), 0.1, 0.8);
    let (doc, x) = insert(
        doc,
        Node::Union {
            members: vec![b1, b2],
            declare: None,
        },
    );
    let (doc, n) = placed(doc, x);
    let doc2 = set_members(doc.clone(), x, vec![b1, w]);
    assert!(!ancestors_in(&doc, n).contains(&w) && ancestors_in(&doc2, n).contains(&w));
    let body = |i: u32| editor_core::EntityRef {
        body: i,
        key: editor_core::EntityKey::Body,
    };
    let f = fixture::minted(EntityKind::Body, n, RoleSeg::OutputBody);
    let base = StableName {
        kind: EntityKind::Body,
        node: n,
        path: vec![RoleSeg::FromA(f.clone().into())],
    };
    let ranked = |rank| {
        let mut name = base.clone();
        name.path
            .push(RoleSeg::Fragment(Qualifier::OrderAlong { rank, of: 2 }));
        name
    };
    let mut before = NameTable::new();
    before.insert(ranked(0), body(0)).unwrap();
    before.insert(ranked(1), body(1)).unwrap();
    before.insert(f.clone(), body(2)).unwrap();
    let mut after = NameTable::new();
    after.insert(base.clone(), body(0)).unwrap();
    after.insert(f.clone(), body(2)).unwrap();
    let verdict = |sign| Verdict {
        predicate: "bool_point_in_solid_plane",
        sign,
    };
    let prior = two_node_eval(&doc, (w, vec![verdict(Sign::Negative)]), (n, before));
    let now = two_node_eval(&doc2, (w, vec![verdict(Sign::Positive)]), (n, after));
    assert_eq!(
        diagnosis((&doc2, &now), (&doc, &prior), &ranked(0)),
        Diagnosis::Upstream {
            node: n,
            cause: UpstreamCause::PredicateFlip {
                predicate: "bool_point_in_solid_plane",
                at: w,
                from: Sign::Negative,
                to: Sign::Positive,
            },
        }
    );
}

/// The recipe-edit lane, upstream. The cutter is `Transform(Union[bar,
/// f])` with `f` strictly inside the bar; the edit swaps `f` for an
/// identical block (a recipe edit at the union, which changes no
/// verdict) and slides the transform along y so the rim edges stop
/// being divided (no flip either). The union is upstream of the cut
/// and not in the ranked names.
#[test]
fn a_recipe_edit_upstream_is_reported_as_upstream() {
    let doc = ProfileDoc::empty_derived("upstream-scope", Tol::witness());
    let (doc, a) = block(doc, (0.0, 3.0), (0.0, 3.0), 0.0, 1.0);
    let (doc, bar) = block(doc, (1.0, 2.0), (-1.0, 4.0), 0.5, 1.0);
    let (doc, f1) = block(doc, (1.2, 1.8), (0.2, 0.8), 0.7, 0.6);
    let (doc, f2) = block(doc, (1.2, 1.8), (0.2, 0.8), 0.7, 0.6);
    let (doc, u) = insert(
        doc,
        Node::Union {
            members: vec![bar, f1],
            declare: None,
        },
    );
    let (doc, tr) = placed(doc, u);
    let (doc, cut) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Subtract,
            a,
            b: tr,
            declare: None,
        },
    );
    let ev1 = run(&doc, None);
    let doc2 = slide(
        set_members(doc.clone(), u, vec![bar, f2]),
        tr,
        Axis3::Y,
        2.5,
    );
    let ev2 = run(&doc2, Some(&ev1));
    let name = vanished_ranked(&ev1, &ev2, cut);
    assert_eq!(
        diagnosis((&doc2, &ev2), (&doc, &ev1), &name),
        Diagnosis::Upstream {
            node: cut,
            cause: UpstreamCause::RecipeEdit {
                edit: RecipeEditRef::NodeChanged { node: u },
            },
        }
    );
}

/// The structural-parameter lane, upstream. The cutter is one
/// instance of a two-instance pattern of the bar (the second placed
/// 2.5 further along y), selected by a `Part`; the edit re-selects
/// instance 1 — a structural parameter at the `Part`, upstream of the
/// cut and not in the ranked names — which lands the bar short of
/// the far rim, with no flip.
#[test]
fn a_structural_parameter_upstream_is_reported_as_upstream() {
    let doc = ProfileDoc::empty_derived("upstream-scope", Tol::witness());
    let (doc, a) = block(doc, (0.0, 3.0), (0.0, 3.0), 0.0, 1.0);
    let (doc, bar) = block(doc, (1.0, 2.0), (-1.0, 4.0), 0.5, 1.0);
    let (doc, pat) = insert(
        doc,
        Node::Pattern {
            input: bar,
            count: editor_core::Expr::count(2),
            kind: editor_core::PatternKind::Linear {
                direction: [scl(0.0), scl(1.0), scl(0.0)],
                spacing: len(2.5),
            },
        },
    );
    let (doc, part) = insert(
        doc,
        Node::Part {
            of: pat,
            select: editor_core::PartSelect::Instance(editor_core::Expr::count(0)),
        },
    );
    let (doc, cut) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Subtract,
            a,
            b: part,
            declare: None,
        },
    );
    let ev1 = run(&doc, None);
    let doc2 = step(
        doc.clone(),
        DocEdit::SetStructuralParam {
            node: part,
            slot: SlotId::Instance,
            expr: editor_core::Expr::count(1),
        },
    )
    .0;
    let ev2 = run(&doc2, Some(&ev1));
    let name = vanished_ranked(&ev1, &ev2, cut);
    assert_eq!(
        diagnosis((&doc2, &ev2), (&doc, &ev1), &name),
        Diagnosis::Upstream {
            node: cut,
            cause: UpstreamCause::StructuralParam {
                node: part,
                param: SlotId::Instance,
            },
        }
    );
}

/// The ORDER among causes: the qualifier delta — a flip of the name's
/// own qualifier, recovered from the names on its path — outranks an
/// upstream flip. Hand-built runs over a real two-node chain: the
/// name is minted at `n = Transform(u)`, its qualifier re-signed in
/// the current table (a clean one-entry `SideOf` delta), and `u`'s
/// verdict log flips. Both rungs have evidence; the path's wins.
#[test]
fn the_qualifier_delta_outranks_an_upstream_flip() {
    let doc = ProfileDoc::empty_derived("upstream-scope", Tol::witness());
    let (doc, u) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, n) = placed(doc, u);
    let (doc, m) = insert(doc, Node::declare_rest(vec![]));
    assert!(ancestors_in(&doc, n).contains(&u));
    let body = |i: u32| editor_core::EntityRef {
        body: i,
        key: editor_core::EntityKey::Body,
    };
    let f = fixture::minted(EntityKind::Body, n, RoleSeg::OutputBody);
    let p = fixture::minted(EntityKind::Body, m, RoleSeg::OutputBody);
    let frag = |v| StableName {
        kind: EntityKind::Body,
        node: n,
        path: vec![
            RoleSeg::FromA(f.clone().into()),
            RoleSeg::Fragment(Qualifier::SideOf(vec![(p.clone(), v)])),
        ],
    };
    let old = frag(editor_core::SideVerdict::Negative);
    let table = |name: &StableName| {
        let mut t = NameTable::new();
        t.insert(name.clone(), body(0)).unwrap();
        t.insert(f.clone(), body(1)).unwrap();
        t.insert(p.clone(), body(2)).unwrap();
        t
    };
    let verdict = |sign| Verdict {
        predicate: "bool_point_in_solid_plane",
        sign,
    };
    let prior = two_node_eval(&doc, (u, vec![verdict(Sign::Negative)]), (n, table(&old)));
    let now = two_node_eval(
        &doc,
        (u, vec![verdict(Sign::Positive)]),
        (n, table(&frag(editor_core::SideVerdict::Positive))),
    );
    assert!(
        editor_core::diff_verdicts(&prior, &now)
            .report()
            .iter()
            .any(|(at, _)| *at == u),
        "the upstream node flips"
    );
    assert_eq!(
        diagnosis((&doc, &now), (&doc, &prior), &old),
        Diagnosis::PredicateFlip {
            predicate: "name_frag_side_of",
            from: Sign::Negative,
            to: Sign::Positive,
            source: editor_core::FlipSource::VerdictLog,
        }
    );
}

/// A hand-built evaluation of two nodes: `upstream` with a verdict log
/// and an empty table, `named` with a table and an empty log.
fn two_node_eval(
    doc: &ProfileDoc,
    (upstream, log): (RecipeNodeId, Vec<Verdict>),
    (named, table): (RecipeNodeId, NameTable),
) -> Evaluation<f64> {
    let value = |table: NameTable, log: Vec<Verdict>| {
        editor_core::NodeResult::Ok(editor_core::NodeValue {
            payload: editor_core::ValuePayload::Declarations(vec![]),
            name_table: Arc::new(table),
            fragment_groups: Arc::default(),
            contacts: Arc::new(topo::ContactRecords::default()),
            carried: Arc::new(editor_core::CarriedDeclarations::default()),
            verdicts: Arc::new(log),
            escalations: Arc::new(vec![]),
            placement: None,
            witness: WitnessSlot::default(),
            content_key: ContentKey(0),
            naming_key: NamingKey(0),
        })
    };
    let mut nodes = std::collections::BTreeMap::new();
    nodes.insert(upstream, value(NameTable::new(), log));
    nodes.insert(named, value(table, vec![]));
    Evaluation::<f64> {
        epoch: editor_core::Epoch::mint(),
        document: doc.id(),
        prior_refused: None,
        order: vec![upstream, named],
        nodes,
        outcome: EvalOutcome::Completed,
        recomputed: 2,
        reused: 0,
        part_evaluations: 0,
        appearance: editor_core::AppearanceResolution::default(),
    }
}
