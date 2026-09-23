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
use fixture::{ang, insert, len, minted, on_frame, scl, step};
use geom_core::Tol;
use geom_core::k_stats::Verdict;

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

/// The `SideVerdict` `name`'s own qualifier records for `partner`.
fn recorded_verdict(name: &StableName, partner: &StableName) -> Option<SideVerdict> {
    let RoleSeg::Fragment(Qualifier::SideOf(vector)) = name.path.last()? else {
        return None;
    };
    vector.iter().find(|(p, _)| p == partner).map(|(_, v)| *v)
}

/// The kernel's own reading of a definite side verdict as a sign.
fn sign_of(v: SideVerdict) -> geom_core::Sign {
    match v {
        SideVerdict::Positive => geom_core::Sign::Positive,
        SideVerdict::Negative => geom_core::Sign::Negative,
        SideVerdict::On => geom_core::Sign::Zero,
        SideVerdict::Mixed => panic!("a Mixed verdict has no single sign"),
    }
}

fn failure(res: &Resolution) -> &editor_core::ResolutionFailure {
    let Resolution::Failed(f) = res else {
        panic!("expected Failed, got {res:?}");
    };
    f
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
    // ACROSS, not away: the bar is moved to the far side of the
    // plate, so the walls the fragment was discriminated against end
    // up on the other side of it. Moving the bar away in +x prunes the
    // pair just as thoroughly and changes NO side — the survivor still
    // satisfies the vanished name's own verdict vector — and the rung
    // honestly finds nothing there (`the_pruned_pair_whose_sides_did_
    // not_change_is_not_recovered` below).
    let doc2 = slide(&s, Axis3::X, -5.0); // disjoint AND across
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
            source: FlipSource::ShadowExec { partner },
        } => {
            assert_eq!(*predicate, "name_frag_side_of");
            assert_ne!(from, to, "a flip names two different sides");
            // `from` is the verdict the NAME ITSELF records for that
            // partner: the prior side re-executes to exactly what the
            // run wrote into the qualifier, which is the D9 replay
            // statement at the pair.
            assert_eq!(
                Some(*from),
                recorded_verdict(&frags[0], partner).map(sign_of),
                "the recovered `from` is the qualifier's own verdict for {partner}"
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
    let doc2 = slide(&s, Axis3::X, -5.0);
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
    assert!(matches!(source, FlipSource::ShadowExec { .. }));
}

#[test]
fn the_rung_reads_the_evaluations_and_the_recipes_edges_only() {
    // The rung reads the recipe for ONE thing: the minting node's
    // input EDGES, so it can find the operand body the boolean
    // consumed. It reads no parameter and replays no op. Both sides
    // are handed the SAME, and wrong, document — the bar parked 100 m
    // away, which leaves the edges untouched and every value wrong —
    // and the answer is byte-identical to the honest one.
    let s = slot();
    let ev1 = run(&s.doc, None);
    let frags = side_of_fragments(&ev1, s.cut);
    let doc2 = slide(&s, Axis3::X, -5.0);
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
    let parked = slide(&s, Axis3::X, -100.0);
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
                source: FlipSource::ShadowExec { .. },
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
    let doc2 = slide(&s, Axis3::X, -5.0);
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
                source: FlipSource::ShadowExec { .. },
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
fn the_orderalong_vanish_is_diagnosed_as_its_group_resizing() {
    // The corpus's own pruned-pair row. An `OrderAlong` group ranks
    // its members against EACH OTHER, so when the disjoint run leaves
    // the rim edge undivided there is no pair left to re-execute and
    // this rung, correctly, finds nothing. What the two tables DO say
    // is that the group went from two fragments to one, and the
    // group-size rung reports exactly that — not the evidence-free
    // fallback. The undivided rim edge rides in the offers.
    let rows = fixture::pr4::diagnosis_corpus::<f64>();
    let (_, res) = rows
        .iter()
        .find(|(label, _)| *label == "flip-vanish")
        .expect("the corpus carries the flip-vanish row");
    let f = failure(res);
    let ResolveError::Vanished { name, .. } = &f.error else {
        panic!("expected Vanished, got {:?}", f.error);
    };
    assert!(
        matches!(
            name.path.last(),
            Some(RoleSeg::Fragment(Qualifier::OrderAlong { of: 2, .. }))
        ),
        "the row is about a ranked fragment: {name:?}"
    );
    assert_eq!(
        vanished(res),
        &Diagnosis::GroupResized {
            node: name.node,
            was: 2,
            now: 1,
        }
    );
    let mut base = name.clone();
    base.path.pop();
    assert!(f.offers.contains(&base), "{:?}", f.offers);
}

// ---------------------------------------------------------------
// Ladder order, on hand-built runs (the geometry is not the
// subject here — the rung's PLACE is).
// ---------------------------------------------------------------

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
    let of = minted(EntityKind::Body, n, RoleSeg::OutputBody);
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
            reason: editor_core::ShadowExecRefusal::PairTooWide {
                pairs: wide,
                ceiling: SHADOW_EXEC_MAX_PAIRS,
            },
        }
    );
    // And exactly at the ceiling it does NOT decline.
    let at = hand(SHADOW_EXEC_MAX_PAIRS);
    assert!(
        !matches!(
            hand_diagnosis(&at, vec![], vec![]),
            Diagnosis::ShadowExecDeclined { .. }
        ),
        "the decline is STRICTLY above the ceiling: a pair of exactly \
         {SHADOW_EXEC_MAX_PAIRS} partners still runs"
    );
}

#[test]
fn the_corpus_widest_pair_is_twelve() {
    // The reason [`SHADOW_EXEC_MAX_PAIRS`] is the number it is, kept
    // as a row rather than as a sentence: a fragment group's partners
    // are the faces meeting it across seam edges, and the widest the
    // evaluation corpus mints is what sets the headroom. Pinned to the
    // measured value and its document, not to an inequality — a
    // corpus that grows a wider group is a fact someone should read,
    // and `< 32` would hide every step of that growth until the last.
    let mut widest = 0usize;
    let mut at = "";
    for cd in crate::corpus::documents() {
        let ev = run(&cd.doc, None);
        for res in ev.nodes.values() {
            let editor_core::NodeResult::Ok(v) = res else {
                continue;
            };
            for (n, _) in v.name_table.iter() {
                for seg in &n.path {
                    if let RoleSeg::Fragment(Qualifier::SideOf(vector)) = seg
                        && vector.len() > widest
                    {
                        widest = vector.len();
                        at = cd.name;
                    }
                }
            }
        }
    }
    assert_eq!(
        (widest, at),
        (12, "nested_islands_106_depth2"),
        "the corpus's widest SideOf vector moved; the ceiling's headroom \
         ({SHADOW_EXEC_MAX_PAIRS}) was chosen against this number"
    );
    assert!(
        widest <= SHADOW_EXEC_MAX_PAIRS,
        "and the rung declines STRICTLY above the ceiling, so a pair of exactly \
         {SHADOW_EXEC_MAX_PAIRS} still runs"
    );
}

#[test]
fn the_pruned_pair_whose_sides_did_not_change_is_not_recovered() {
    // The bar moves AWAY on the side it was already on. The pair is
    // pruned exactly as hard as in the recovered case — the population
    // is empty, the fragments are gone — and yet no side moved: the
    // surviving cap is still on the negative side of one wall and the
    // positive side of the other, which is the vanished name's own
    // verdict vector. Nothing flipped, so the rung reports nothing and
    // the vanish rests on the later rungs.
    //
    // This is the limit the rung's docs state, measured. A rung that
    // answered here would be naming a flip that did not happen.
    let s = slot();
    let ev1 = run(&s.doc, None);
    let frags = side_of_fragments(&ev1, s.cut);
    let doc2 = slide(&s, Axis3::X, 5.0);
    let ev2 = run(&doc2, Some(&ev1));
    assert!(
        side_of_fragments(&ev2, s.cut).is_empty(),
        "the pair is pruned in this direction too"
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
    assert!(
        !matches!(
            vanished(&res),
            Diagnosis::PredicateFlip {
                source: FlipSource::ShadowExec { .. },
                ..
            } | Diagnosis::ShadowExecDeclined { .. }
        ),
        "no side changed, so the rung has nothing honest to say: {:?}",
        vanished(&res)
    );
    // And what DOES answer it is the log: withdrawing the bar leaves a
    // recorded containment flip at the cut, and a recorded flip names
    // a predicate where the group-size rung (which this edit also
    // satisfies: two fragments became one) names only the effect. So
    // the recorded flip wins, and this row goes red if the group-size
    // rung is ever raised above the recorded flips.
    assert!(
        matches!(
            vanished(&res),
            Diagnosis::PredicateFlip {
                predicate: "bool_point_in_solid_plane",
                source: FlipSource::VerdictLog,
                ..
            }
        ),
        "the recorded flip outranks the group-size rung: {:?}",
        vanished(&res)
    );
}

#[test]
fn a_collapsed_sideof_group_is_diagnosed_group_resized_and_offers_the_survivor() {
    // The collapse: the bar stops CROSSING the cap (it lands short in
    // y), so the group is no longer multi-fragment and the qualifier
    // is not minted — while the walls have not moved relative to the
    // fragment at all. There is no flip for the shadow-exec rung to
    // recover, and none is claimed: the group-size rung states that
    // the cap's group went from two fragments to one, and the
    // undivided cap is offered for an explicit rebind.
    for to in [2.5_f64, 3.5] {
        let s = slot();
        let ev1 = run(&s.doc, None);
        let frags = side_of_fragments(&ev1, s.cut);
        let doc2 = slide(&s, Axis3::Y, to);
        let ev2 = run(&doc2, Some(&ev1));
        assert!(side_of_fragments(&ev2, s.cut).is_empty(), "y = {to}");
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
        assert_eq!(
            vanished(&res),
            &Diagnosis::GroupResized {
                node: s.cut,
                was: 2,
                now: 1,
            },
            "y = {to}"
        );
        let mut base = frags[0].clone();
        base.path.pop();
        assert!(
            failure(&res).offers.contains(&base),
            "y = {to}: the undivided cap is the offer, got {:?}",
            failure(&res).offers
        );
    }
}

#[test]
fn a_collapsed_orderalong_edge_group_at_the_cut_is_diagnosed_group_resized() {
    // The same collapse, read off the ranked EDGE fragments the cut
    // mints along the cap's rim: the second, non-flush witness of the
    // `OrderAlong` half, on a subtract rather than the corpus union.
    for to in [2.5_f64, 3.5] {
        let s = slot();
        let ev1 = run(&s.doc, None);
        let ranked: Vec<StableName> = ev1
            .value(s.cut)
            .expect("the cut evaluates")
            .name_table
            .iter()
            .filter_map(|(n, e)| {
                let hit = matches!(
                    n.path.last(),
                    Some(RoleSeg::Fragment(Qualifier::OrderAlong { .. }))
                );
                (hit && matches!(e, Entry::Unique(_))).then(|| n.clone())
            })
            .collect();
        assert!(!ranked.is_empty(), "the cut ranks some edge fragments");
        let doc2 = slide(&s, Axis3::Y, to);
        let ev2 = run(&doc2, Some(&ev1));
        let gone: Vec<&StableName> = ranked
            .iter()
            .filter(|n| {
                ev2.value(s.cut)
                    .expect("the cut evaluates")
                    .name_table
                    .lookup(n)
                    .is_none()
            })
            .collect();
        assert!(!gone.is_empty(), "y = {to}: some ranked fragment vanishes");
        for name in gone {
            let res = resolve_with_prior(
                RunCtx {
                    doc: &doc2,
                    eval: &ev2,
                },
                RunCtx {
                    doc: &s.doc,
                    eval: &ev1,
                },
                name,
                Tol::witness(),
            );
            assert_eq!(
                vanished(&res),
                &Diagnosis::GroupResized {
                    node: s.cut,
                    was: 2,
                    now: 1,
                },
                "y = {to}: {name:?}"
            );
        }
    }
}

#[test]
fn opposite_fragments_get_opposite_answers() {
    // The two fragments sit on OPPOSITE sides of the same two walls —
    // that is what their qualifiers record — so a rung that told them
    // apart must answer them differently. Moving the bar across to −x
    // re-qualifies the LEFT fragment and leaves the right one where it
    // was; moving it across to +x does the reverse; and the two
    // recovered flips name the same partner with opposite signs.
    let s = slot();
    let ev1 = run(&s.doc, None);
    let frags = side_of_fragments(&ev1, s.cut);
    assert_eq!(frags.len(), 2);
    let answer = |to: f64, name: &StableName| {
        let doc2 = slide(&s, Axis3::X, to);
        let ev2 = run(&doc2, Some(&ev1));
        let res = resolve_with_prior(
            RunCtx {
                doc: &doc2,
                eval: &ev2,
            },
            RunCtx {
                doc: &s.doc,
                eval: &ev1,
            },
            name,
            Tol::witness(),
        );
        match vanished(&res) {
            Diagnosis::PredicateFlip {
                from,
                to,
                source: FlipSource::ShadowExec { partner },
                ..
            } => Some(((*from, *to), (**partner).clone())),
            _ => None,
        }
    };
    let left_across = answer(-5.0, &frags[0]).expect("the left fragment is re-qualified");
    assert!(
        answer(-5.0, &frags[1]).is_none(),
        "the right fragment's sides did not change in this direction"
    );
    let right_across = answer(5.0, &frags[1]).expect("the right fragment is re-qualified");
    assert!(
        answer(5.0, &frags[0]).is_none(),
        "and the left one's did not change in the other"
    );
    assert_eq!(
        left_across.1, right_across.1,
        "both flips are about the same wall"
    );
    assert_eq!(
        left_across.0,
        (right_across.0.1, right_across.0.0),
        "and they name it with opposite signs"
    );
}

#[test]
fn a_partner_behind_a_pattern_and_a_part_is_probed_at_the_operand() {
    // The second pass-through kind, after `Transform`
    // (`bool7r1_probes::a_partner_behind_a_transform_is_probed_against_which_body`
    // is the first): the bar reaches the boolean through THREE
    // name-preserving placers — a transform, a linear pattern, and a
    // `Part` selecting one instance out of it. The partner names are
    // still the bar's own extrude-node names, carried unchanged the
    // whole way, so a table scan finds them on the unplaced extrude
    // and reads a carrier the boolean never saw.
    //
    // Walking the minting node's inputs lands on the `Part`, whatever
    // the chain above it is, which is the body the boolean consumed.
    let doc = ProfileDoc::empty_derived("bool7-part", Tol::witness());
    let (doc, a) = block(doc, (0.0, 3.0), (0.0, 3.0), 0.0, 1.0);
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
    let (doc, pat) = insert(
        doc,
        Node::Pattern {
            input: tr,
            count: editor_core::Expr::count(2),
            kind: editor_core::PatternKind::Linear {
                direction: [scl(0.0), scl(1.0), scl(0.0)],
                spacing: len(20.0),
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
    let frags = side_of_fragments(&ev1, cut);
    assert_eq!(frags.len(), 2, "the bar still splits the cap");
    let doc2 = step(
        doc.clone(),
        DocEdit::SetParam {
            node: tr,
            slot: SlotId::Translation(Axis3::X),
            expr: len(-5.0),
        },
    )
    .0;
    let ev2 = run(&doc2, Some(&ev1));
    let res = resolve_with_prior(
        RunCtx {
            doc: &doc2,
            eval: &ev2,
        },
        RunCtx {
            doc: &doc,
            eval: &ev1,
        },
        &frags[0],
        Tol::witness(),
    );
    let Diagnosis::PredicateFlip {
        from,
        to,
        source: FlipSource::ShadowExec { partner },
        ..
    } = vanished(&res)
    else {
        panic!("expected the recovered flip, got {:?}", vanished(&res));
    };
    assert_ne!(from, to);
    assert_eq!(
        Some(*from),
        recorded_verdict(&frags[0], partner).map(sign_of),
        "and it calibrates against the qualifier through three placers"
    );
}

// ---------------------------------------------------------------
// The group-size rung's own boundary, on hand-built runs: when it
// answers, what it counts, and when it declines to the fallback.
// Geometry-free, like the ladder-order rows above — the shadow-exec
// rung finds no face on a body-kind name and stays silent, so each
// row is about the group-size rung alone.
// ---------------------------------------------------------------

/// The fixed cast: a hand document, the vanished fragment `frag`
/// (partner verdict Positive), and its sibling (Negative) — a group
/// of two in the prior run.
fn sibling(h: &Hand, v: SideVerdict) -> StableName {
    let RoleSeg::Fragment(Qualifier::SideOf(vector)) = h.frag.path.last().unwrap() else {
        unreachable!("hand() mints a SideOf fragment");
    };
    let mut name = h.base.clone();
    name.path.push(RoleSeg::Fragment(Qualifier::SideOf(vec![(
        vector[0].0.clone(),
        v,
    )])));
    name
}

/// Resolves `name` with the prior table holding `prior` rows and the
/// new table holding `now` rows, each `(name, entities)`; every name
/// embedded in `h.frag` resolves in both runs, so no Cascade.
fn group_diagnosis(
    h: &Hand,
    name: &StableName,
    prior: Vec<(StableName, usize)>,
    now: Vec<(StableName, usize)>,
) -> editor_core::ResolutionFailure {
    let table = |rows: Vec<(StableName, usize)>| {
        let mut t = NameTable::new();
        let mut next = 0u32;
        for (row, n) in rows {
            let ents: Vec<_> = (0..n)
                .map(|_| {
                    next += 1;
                    body_ent(next)
                })
                .collect();
            if n == 1 {
                t.insert(row, ents[0]).unwrap();
            } else {
                t.insert_tied(row, ents).unwrap();
            }
        }
        for (i, inner) in h.inner.iter().enumerate() {
            t.insert(inner.clone(), body_ent(1000 + i as u32)).unwrap();
        }
        t
    };
    let prior_ev = one_node_eval(h.doc.id(), h.node, table(prior), vec![]);
    let new_ev = one_node_eval(h.doc.id(), h.node, table(now), vec![]);
    let res = resolve_with_prior(
        RunCtx {
            doc: &h.doc,
            eval: &new_ev,
        },
        RunCtx {
            doc: &h.doc,
            eval: &prior_ev,
        },
        name,
        Tol::witness(),
    );
    failure(&res).clone()
}

fn diag(f: &editor_core::ResolutionFailure) -> &Diagnosis {
    let ResolveError::Vanished { diagnosis, .. } = &f.error else {
        panic!("expected Vanished, got {:?}", f.error);
    };
    diagnosis
}

fn fallback(h: &Hand) -> Diagnosis {
    Diagnosis::RecipeEdit {
        edit: editor_core::RecipeEditRef::NodeChanged { node: h.node },
    }
}

#[test]
fn a_group_that_stops_being_divided_is_resized_to_one_and_offers_the_base() {
    let h = hand(1);
    let f = group_diagnosis(
        &h,
        &h.frag,
        vec![(h.frag.clone(), 1), (sibling(&h, SideVerdict::Negative), 1)],
        vec![(h.base.clone(), 1)],
    );
    assert_eq!(
        diag(&f),
        &Diagnosis::GroupResized {
            node: h.node,
            was: 2,
            now: 1,
        }
    );
    assert_eq!(f.offers, vec![h.base.clone()]);
}

#[test]
fn a_group_whose_parent_no_longer_descends_is_resized_to_zero() {
    let h = hand(1);
    let f = group_diagnosis(
        &h,
        &h.frag,
        vec![(h.frag.clone(), 1), (sibling(&h, SideVerdict::Negative), 1)],
        vec![],
    );
    assert_eq!(
        diag(&f),
        &Diagnosis::GroupResized {
            node: h.node,
            was: 2,
            now: 0,
        }
    );
    assert!(f.offers.is_empty(), "nothing survives to offer");
}

#[test]
fn a_group_that_grows_is_resized_too_and_a_tie_counts_each_candidate() {
    // Prior: the vanished fragment beside a TIED sibling row of two —
    // a group of three entities under two names. Now: two distinct
    // fragments that are neither — a group of two. A tie is several
    // members sharing one name, so it counts per candidate.
    let h = hand(1);
    let f = group_diagnosis(
        &h,
        &h.frag,
        vec![(h.frag.clone(), 1), (sibling(&h, SideVerdict::Negative), 2)],
        vec![
            (sibling(&h, SideVerdict::Mixed), 1),
            (sibling(&h, SideVerdict::On), 1),
        ],
    );
    assert_eq!(
        diag(&f),
        &Diagnosis::GroupResized {
            node: h.node,
            was: 3,
            now: 2,
        }
    );
    // And growth: a group of two became three. The new members are
    // spelled with a different qualifier kind on purpose — the count
    // is of the group, whatever qualifies its members, and a SideOf
    // sibling one sign away would be the qualifier-delta rung's flip.
    let ranked = |rank| {
        let mut n = h.base.clone();
        n.path
            .push(RoleSeg::Fragment(Qualifier::OrderAlong { rank, of: 3 }));
        (n, 1)
    };
    let f = group_diagnosis(
        &h,
        &h.frag,
        vec![(h.frag.clone(), 1), (sibling(&h, SideVerdict::Negative), 1)],
        vec![ranked(0), ranked(1), ranked(2)],
    );
    assert_eq!(
        diag(&f),
        &Diagnosis::GroupResized {
            node: h.node,
            was: 2,
            now: 3,
        }
    );
}

#[test]
fn a_group_that_requalified_at_the_same_size_is_not_a_resize() {
    // Two fragments before, two after, the vanished one not among
    // them. No single pure-sign delta either (Mixed/On have no sign),
    // so every rung is silent and the fallback is the honest answer.
    let h = hand(1);
    let f = group_diagnosis(
        &h,
        &h.frag,
        vec![(h.frag.clone(), 1), (sibling(&h, SideVerdict::Negative), 1)],
        vec![
            (sibling(&h, SideVerdict::Mixed), 1),
            (sibling(&h, SideVerdict::On), 1),
        ],
    );
    assert_eq!(diag(&f), &fallback(&h));
}

#[test]
fn a_name_the_prior_run_never_minted_did_not_vanish_by_resizing() {
    // The prior group had two members and the current one has one —
    // but neither was the referenced name, so its group changing size
    // is not why it does not resolve.
    let h = hand(1);
    let f = group_diagnosis(
        &h,
        &h.frag,
        vec![
            (sibling(&h, SideVerdict::Negative), 1),
            (sibling(&h, SideVerdict::Mixed), 1),
        ],
        vec![(h.base.clone(), 1)],
    );
    assert_eq!(diag(&f), &fallback(&h));
}

#[test]
fn a_name_without_a_fragment_tail_never_reaches_the_group_size_rung() {
    // The base itself vanishing: no qualifier, so no group to count,
    // even though the table it was in had company.
    let h = hand(1);
    let f = group_diagnosis(
        &h,
        &h.base,
        vec![(h.base.clone(), 1), (h.frag.clone(), 1)],
        vec![(sibling(&h, SideVerdict::Negative), 1)],
    );
    assert_eq!(diag(&f), &fallback(&h));
}

#[test]
fn without_a_prior_run_there_is_no_size_to_change_from() {
    let h = hand(1);
    let mut t = NameTable::new();
    t.insert(h.base.clone(), body_ent(0)).unwrap();
    for (i, inner) in h.inner.iter().enumerate() {
        t.insert(inner.clone(), body_ent(1000 + i as u32)).unwrap();
    }
    let ev = one_node_eval(h.doc.id(), h.node, t, vec![]);
    let res = editor_core::resolve(
        RunCtx {
            doc: &h.doc,
            eval: &ev,
        },
        &h.frag,
    );
    assert_eq!(vanished(&res), &fallback(&h));
}
