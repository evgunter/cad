//! BOOL-7 (issue 134): the shadow-execution rung of the `Vanished`
//! ladder — the empty pair population, recovered.
//!
//! The defect these rows pin: a fragment name discriminated by
//! `name_frag_side_of` vanishes on an interaction-boundary edit, and
//! the run that vanished it recorded NO verdict for the pair, so the
//! population diff has nothing to diff. The rung re-executes the
//! pair's own predicates against both contexts and reports the flip.
//!
//! The suite carries the ladder ORDER as rows too, because the rung's
//! value is entirely in where it sits: a recorded discriminator flip
//! must beat it, a non-empty population must not reach it, and an
//! incidental `bool_*` flip at the same node must lose to it.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use std::sync::Arc;

use editor_core::eval::WitnessSlot;
use editor_core::{
    Axis3, BooleanOp, CancelToken, ContentKey, Diagnosis, DocEdit, EntityKind, Entry, EvalOptions,
    EvalOutcome, Evaluation, FlipSource, NameTable, NamingKey, Node, ProfileDoc, Qualifier,
    RecipeNodeId, Resolution, ResolveError, RoleSeg, RunCtx, SHADOW_EXEC_MAX_PAIRS, SideVerdict,
    SlotId, StableName, diff_verdicts, evaluate, resolve_with_prior,
};
use fixture::{ang, insert, len, on_frame, scl, step};
use geom_core::k_stats::Verdict;
use geom_core::{Sign, Tol};

// ---------------------------------------------------------------
// The document: a bar crossing a plate's end cap, so the cap
// descends as a multi-fragment group discriminated against the
// bar's walls.
// ---------------------------------------------------------------

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

/// The scenario: plate `a` (3×3×1), bar `b` crossing its end cap
/// fully in y behind a `Transform`, subtracted at `cut`.
struct Slot {
    doc: ProfileDoc,
    tr: RecipeNodeId,
    cut: RecipeNodeId,
}

fn slot() -> Slot {
    let doc = ProfileDoc::empty_derived("bool7", Tol::witness());
    let (doc, a) = block(doc, (0.0, 3.0), (0.0, 3.0), 0.0, 1.0);
    let (doc, b0) = block(doc, (1.0, 2.0), (-1.0, 4.0), 0.5, 1.0);
    let (doc, tr) = insert(
        doc,
        Node::Transform {
            input: b0,
            translation: [len(0.0), len(0.0), len(0.0)],
            rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
            rotation_angle: ang(0.0),
        },
    );
    let (doc, cut) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Subtract,
            a,
            b: tr,
            declare: None,
        },
    );
    Slot { doc, tr, cut }
}

/// Slides the bar along `axis` — the interaction-boundary edit.
fn slide(s: &Slot, axis: Axis3, to: f64) -> ProfileDoc {
    step(
        s.doc.clone(),
        DocEdit::SetParam {
            node: s.tr,
            slot: SlotId::Translation(axis),
            expr: len(to),
        },
    )
    .0
}

/// The `SideOf`-discriminated cap fragments of `cut`'s table, in
/// table order (deterministic).
fn side_of_fragments(ev: &Evaluation<f64>, cut: RecipeNodeId) -> Vec<StableName> {
    ev.value(cut)
        .expect("the cut evaluates")
        .name_table
        .iter()
        .filter_map(|(n, e)| {
            let discriminated =
                matches!(n.path.last(), Some(RoleSeg::Fragment(Qualifier::SideOf(_))));
            (discriminated && matches!(e, Entry::Unique(_))).then(|| n.clone())
        })
        .collect()
}

fn vanished(res: &Resolution) -> &Diagnosis {
    let Resolution::Failed(f) = res else {
        panic!("expected Failed, got {res:?}");
    };
    let ResolveError::Vanished { diagnosis, .. } = &f.error else {
        panic!("expected Vanished, got {:?}", f.error);
    };
    diagnosis
}

// ---------------------------------------------------------------
// The rung itself.
// ---------------------------------------------------------------

#[test]
fn pruned_pair_vanish_recovers_the_discriminator_flip() {
    let s = slot();
    let ev1 = run(&s.doc, None);
    let frags = side_of_fragments(&ev1, s.cut);
    assert_eq!(frags.len(), 2, "the bar splits the cap into two fragments");
    // The measurement: the prior run recorded the pair, the disjoint
    // run recorded nothing for it at all.
    let pop = |ev: &Evaluation<f64>| {
        ev.value(s.cut).map_or(0, |v| {
            v.verdicts
                .iter()
                .filter(|w| w.predicate == "name_frag_side_of")
                .count()
        })
    };
    assert!(pop(&ev1) > 0, "the prior run discriminated the fragments");
    let doc2 = slide(&s, Axis3::X, 5.0); // disjoint: the sweep prunes
    let ev2 = run(&doc2, Some(&ev1));
    assert_eq!(pop(&ev2), 0, "the pruned run records no pair verdict");
    assert!(
        side_of_fragments(&ev2, s.cut).is_empty(),
        "and mints no discriminated fragment"
    );

    let res = resolve_with_prior(
        RunCtx {
            doc: &doc2,
            eval: &ev2,
        },
        RunCtx {
            doc: &s.doc,
            eval: &ev1,
        },
        &frags[0],
        Tol::witness(),
    );
    match vanished(&res) {
        Diagnosis::PredicateFlip {
            predicate,
            from,
            to,
            source,
        } => {
            assert_eq!(*predicate, "name_frag_side_of");
            assert_ne!(from, to, "a flip names two different signs");
            assert_eq!(
                *source,
                FlipSource::ShadowExec,
                "the flip is recovered, not read out of a log"
            );
        }
        other => panic!("expected the recovered discriminator flip, got {other:?}"),
    }
}

#[test]
fn the_recovered_flip_beats_an_incidental_boolean_flip_at_the_same_node() {
    // The disjoint run still walks containment and leaves a real,
    // path-local `bool_*` flip at the minting node. Before the rung,
    // THAT is what the ladder reported — a sentence about the
    // boolean's interior in answer to "why did this name vanish".
    let s = slot();
    let ev1 = run(&s.doc, None);
    let frags = side_of_fragments(&ev1, s.cut);
    let doc2 = slide(&s, Axis3::X, 5.0);
    let ev2 = run(&doc2, Some(&ev1));
    let incidental = diff_verdicts(&ev1, &ev2).flips_on_path(&frags[0]);
    assert!(
        incidental
            .iter()
            .any(|(_, f)| f.predicate.starts_with("bool_")),
        "the scenario's premise: a recorded non-discriminator flip on the path"
    );
    assert!(
        !incidental
            .iter()
            .any(|(_, f)| f.predicate.starts_with("name_frag_")),
        "and no recorded discriminator flip to prefer"
    );
    let res = resolve_with_prior(
        RunCtx {
            doc: &doc2,
            eval: &ev2,
        },
        RunCtx {
            doc: &s.doc,
            eval: &ev1,
        },
        &frags[0],
        Tol::witness(),
    );
    let Diagnosis::PredicateFlip {
        predicate, source, ..
    } = vanished(&res)
    else {
        panic!("expected a PredicateFlip");
    };
    assert_eq!(*predicate, "name_frag_side_of");
    assert_eq!(*source, FlipSource::ShadowExec);
}

#[test]
fn the_rung_reads_the_evaluations_and_never_the_recipe() {
    // Both sides are handed the SAME, and wrong, document: the bar is
    // parked 100 m away in the recipe while the evaluations still
    // hold the geometry they were built from. A rung that replayed
    // the op — or read the recipe at all — would answer differently;
    // a rung that reads the two contexts answers identically.
    let s = slot();
    let ev1 = run(&s.doc, None);
    let frags = side_of_fragments(&ev1, s.cut);
    let doc2 = slide(&s, Axis3::X, 5.0);
    let ev2 = run(&doc2, Some(&ev1));
    let honest = resolve_with_prior(
        RunCtx {
            doc: &doc2,
            eval: &ev2,
        },
        RunCtx {
            doc: &s.doc,
            eval: &ev1,
        },
        &frags[0],
        Tol::witness(),
    );
    let parked = slide(&s, Axis3::X, 100.0);
    let over_a_wrong_recipe = resolve_with_prior(
        RunCtx {
            doc: &parked,
            eval: &ev2,
        },
        RunCtx {
            doc: &parked,
            eval: &ev1,
        },
        &frags[0],
        Tol::witness(),
    );
    assert!(
        matches!(
            vanished(&honest),
            Diagnosis::PredicateFlip {
                source: FlipSource::ShadowExec,
                ..
            }
        ),
        "the row is about the recovered flip, so it has to be one"
    );
    assert_eq!(
        vanished(&honest),
        vanished(&over_a_wrong_recipe),
        "the recovered flip comes from the evaluations alone"
    );
}

#[test]
fn the_rung_writes_to_no_log() {
    // An open frame is the whole test: the shadow probes decide
    // `name_frag_side_of`, and if they decided it anywhere but in a
    // detached recording of their own, this bracket would collect it.
    let s = slot();
    let ev1 = run(&s.doc, None);
    let frags = side_of_fragments(&ev1, s.cut);
    let doc2 = slide(&s, Axis3::X, 5.0);
    let ev2 = run(&doc2, Some(&ev1));
    let bracket = geom_core::k_stats::Bracket::open();
    let res = resolve_with_prior(
        RunCtx {
            doc: &doc2,
            eval: &ev2,
        },
        RunCtx {
            doc: &s.doc,
            eval: &ev1,
        },
        &frags[0],
        Tol::witness(),
    );
    let recorded = bracket.finish();
    assert!(
        matches!(
            vanished(&res),
            Diagnosis::PredicateFlip {
                source: FlipSource::ShadowExec,
                ..
            }
        ),
        "the rung did fire, so the frame had something to catch"
    );
    assert_eq!(
        recorded,
        geom_core::k_stats::Recorded::default(),
        "diagnosis-time execution reaches no channel: {recorded:?}"
    );
}

#[test]
fn the_orderalong_half_of_the_issue_is_not_recovered() {
    // The corpus's own pruned-pair row. `OrderAlong { rank, of }`
    // records no partner, so the pair it was ranked against cannot be
    // read back out of the name — and the pruned run has no sibling
    // to rank against either. The row keeps the documented
    // evidence-free fallback, deliberately.
    let rows = fixture::pr4::diagnosis_corpus::<f64>();
    let (_, res) = rows
        .iter()
        .find(|(label, _)| *label == "flip-vanish")
        .expect("the corpus carries the flip-vanish row");
    match vanished(res) {
        Diagnosis::RecipeEdit { .. } => {}
        other => panic!("the OrderAlong half is not recovered by this rung, got {other:?}"),
    }
}

// ---------------------------------------------------------------
// Ladder order, on hand-built runs (the geometry is not the
// subject here — the rung's PLACE is).
// ---------------------------------------------------------------

fn name1(kind: EntityKind, node: RecipeNodeId, seg: RoleSeg) -> StableName {
    StableName {
        kind,
        node,
        path: vec![seg],
    }
}

fn body_ent(i: u32) -> editor_core::EntityRef {
    editor_core::EntityRef {
        body: i,
        key: editor_core::EntityKey::Body,
    }
}

/// A fragment of `of` discriminated against `partners`.
fn frag(
    node: RecipeNodeId,
    of: &StableName,
    partners: Vec<(StableName, SideVerdict)>,
) -> StableName {
    StableName {
        kind: EntityKind::Body,
        node,
        path: vec![
            RoleSeg::FromA(of.clone().into()),
            RoleSeg::Fragment(Qualifier::SideOf(partners)),
        ],
    }
}

/// One-node evaluation carrying `t` and the verdict log `log`.
fn one_node_eval(
    document: editor_core::DocumentId,
    node: RecipeNodeId,
    t: NameTable,
    log: Vec<Verdict>,
) -> Evaluation<f64> {
    let mut nodes = std::collections::BTreeMap::new();
    nodes.insert(
        node,
        editor_core::NodeResult::Ok(editor_core::NodeValue {
            payload: editor_core::ValuePayload::Declarations(vec![]),
            name_table: Arc::new(t),
            contacts: Arc::new(topo::ContactRecords::default()),
            carried: Arc::new(editor_core::CarriedDeclarations::default()),
            verdicts: Arc::new(log),
            escalations: Arc::new(vec![]),
            placement: None,
            witness: WitnessSlot::default(),
            content_key: ContentKey(0),
            naming_key: NamingKey(0),
        }),
    );
    Evaluation::<f64> {
        epoch: editor_core::Epoch::mint(),
        document,
        prior_refused: None,
        order: vec![node],
        nodes,
        outcome: EvalOutcome::Completed,
        recomputed: 1,
        reused: 0,
        part_evaluations: 0,
        appearance: editor_core::AppearanceResolution::default(),
    }
}

fn verdict(predicate: &'static str, sign: Sign) -> Verdict {
    Verdict { predicate, sign }
}

/// A two-`declare_rest` document and the vanished/base/partner names
/// over its first node. The document is deliberately geometry-free:
/// every row below decides the rung's PLACE, and none of them may
/// depend on a body existing.
struct Hand {
    doc: ProfileDoc,
    node: RecipeNodeId,
    frag: StableName,
    base: StableName,
    /// The names embedded in `frag` — its operand name and every
    /// discriminator partner.
    inner: Vec<StableName>,
}

fn hand(partners: usize) -> Hand {
    let (doc, n) = insert(
        ProfileDoc::empty_derived("bool7-hand", Tol::witness()),
        Node::declare_rest(vec![]),
    );
    let (doc, m) = insert(doc, Node::declare_rest(vec![]));
    let of = name1(EntityKind::Body, n, RoleSeg::OutputBody);
    let vector: Vec<(StableName, SideVerdict)> = (0..partners)
        .map(|i| {
            (
                StableName {
                    kind: EntityKind::Body,
                    node: m,
                    path: vec![
                        RoleSeg::OutputBody,
                        RoleSeg::Fragment(Qualifier::OrderAlong {
                            rank: i as u32,
                            of: partners as u32,
                        }),
                    ],
                },
                SideVerdict::Positive,
            )
        })
        .collect();
    let inner = core::iter::once(of.clone())
        .chain(vector.iter().map(|(p, _)| p.clone()))
        .collect();
    Hand {
        base: StableName {
            kind: EntityKind::Body,
            node: n,
            path: vec![RoleSeg::FromA(of.clone().into())],
        },
        frag: frag(n, &of, vector),
        inner,
        doc,
        node: n,
    }
}

/// Resolves `h.frag` against a new run whose table is empty (the name
/// vanished) with the given verdict logs on both sides.
fn hand_diagnosis(h: &Hand, prior_log: Vec<Verdict>, new_log: Vec<Verdict>) -> Diagnosis {
    // Every name embedded in the vanished one resolves in BOTH runs,
    // so the Cascade rung above has nothing to report and the rows
    // below are about the rung they name.
    let mut t_prior = NameTable::new();
    t_prior.insert(h.frag.clone(), body_ent(0)).unwrap();
    let mut t_new = NameTable::new();
    t_new.insert(h.base.clone(), body_ent(0)).unwrap();
    for (i, inner) in h.inner.iter().enumerate() {
        let ent = body_ent(i as u32 + 1);
        t_prior.insert(inner.clone(), ent).unwrap();
        t_new.insert(inner.clone(), ent).unwrap();
    }
    let prior_ev = one_node_eval(h.doc.id(), h.node, t_prior, prior_log);
    let new_ev = one_node_eval(h.doc.id(), h.node, t_new, new_log);
    let res = resolve_with_prior(
        RunCtx {
            doc: &h.doc,
            eval: &new_ev,
        },
        RunCtx {
            doc: &h.doc,
            eval: &prior_ev,
        },
        &h.frag,
        Tol::witness(),
    );
    vanished(&res).clone()
}

#[test]
fn a_recorded_discriminator_flip_beats_the_shadow_rung() {
    // Rung 1 over rung 2: the log HAS a `name_frag_*` flip on the
    // path, so the rung must not be reached even though the
    // `name_frag_side_of` population is empty in both runs.
    let h = hand(1);
    let d = hand_diagnosis(
        &h,
        vec![verdict("name_frag_order_along", Sign::Positive)],
        vec![verdict("name_frag_order_along", Sign::Negative)],
    );
    assert_eq!(
        d,
        Diagnosis::PredicateFlip {
            predicate: "name_frag_order_along",
            from: Sign::Positive,
            to: Sign::Negative,
            source: FlipSource::VerdictLog,
        }
    );
}

#[test]
fn a_non_empty_pair_population_never_enters_the_rung() {
    // Both runs recorded the pair, so the log is the evidence and the
    // rung must not second-guess it. Equal populations mean no flip
    // at all, and the ladder falls through to the documented
    // evidence-free rung rather than to a recovered one.
    let h = hand(1);
    let d = hand_diagnosis(
        &h,
        vec![verdict("name_frag_side_of", Sign::Positive)],
        vec![verdict("name_frag_side_of", Sign::Positive)],
    );
    assert!(
        !matches!(
            d,
            Diagnosis::PredicateFlip {
                source: FlipSource::ShadowExec,
                ..
            } | Diagnosis::ShadowExecDeclined { .. }
        ),
        "the rung is unreachable with a recorded population, got {d:?}"
    );
}

#[test]
fn a_pair_wider_than_the_ceiling_declines_typed() {
    // Above the ceiling the rung refuses WITH ITS NUMBER. A silent
    // fall-through here would read as "cause not in evidence" when
    // the cause was in evidence and merely too expensive to fetch.
    let wide = SHADOW_EXEC_MAX_PAIRS + 1;
    let h = hand(wide);
    let d = hand_diagnosis(&h, vec![], vec![]);
    assert_eq!(
        d,
        Diagnosis::ShadowExecDeclined {
            node: h.node,
            pairs: wide,
            ceiling: SHADOW_EXEC_MAX_PAIRS,
        }
    );
    // And exactly at the ceiling it does NOT decline.
    let at = hand(SHADOW_EXEC_MAX_PAIRS);
    assert!(
        !matches!(
            hand_diagnosis(&at, vec![], vec![]),
            Diagnosis::ShadowExecDeclined { .. }
        ),
        "the ceiling is inclusive"
    );
}
