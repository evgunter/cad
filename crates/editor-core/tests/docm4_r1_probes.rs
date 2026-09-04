//! DOCM-4 review lane R1 — probes against PR #1808 at `e8d0022d`.
//!
//! Built independently of the implementer's rows: a MATED twin pair
//! (instances + a `Rest` mate) and a corpus twin re-minted through the
//! persist header, rather than the PR's profile+extrude twin. The
//! `run_checks` row asserts the contract the spec's C3 states and is
//! expected RED on the head (the hole the PR body discloses at
//! `checks.rs:567`); the forged-stamp row is a demonstration, not a
//! contract, and is expected green.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use std::collections::BTreeMap;
use std::sync::Arc;

use editor_core::{
    Advisory, Alignment, AxisSense, CancelToken, CapEnd, ChecksConfig, ChecksError, ContactClass,
    ContentPin, DocEdit, DocRef, DocumentId, EntityKind, EvalOptions, Evaluation, Frame,
    MateFault, MateFrame, MatePrimitive, Node, NodeResult, PartResolver, ProductError,
    ProfileDoc, RecipeNodeId, ResolveFailure, ResolveFault, RoleSeg, StableName, assemble,
    content_pin, evaluate, product, product_recorded, run_checks, solve_document,
};
use fixture::{insert, len, on_frame, step};
use geom_core::Tol;

// ---- fixtures (own construction) ----

#[derive(Debug, Default)]
struct StubStore {
    docs: BTreeMap<DocumentId, ProfileDoc>,
}

impl StubStore {
    fn insert(&mut self, doc: ProfileDoc, tol: Tol) -> DocRef {
        let pin: ContentPin = content_pin(&doc, tol).expect("the pin computes");
        let id = doc.id();
        self.docs.insert(id, doc);
        DocRef { id, pin }
    }
}

impl PartResolver for StubStore {
    fn resolve(&self, doc_ref: &DocRef, _tol: Tol) -> Result<ProfileDoc, ResolveFailure> {
        let doc = self.docs.get(&doc_ref.id).ok_or_else(|| ResolveFailure {
            fault: ResolveFault::Unresolved,
            message: "no such document".to_string(),
        })?;
        Ok(doc.clone())
    }
}

/// A unit cube part: frame 0, profile 1, extrude 2.
fn cube_part(label: &str) -> ProfileDoc {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, p) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
    );
    let (doc, _) = insert(
        doc,
        Node::Extrude {
            profile: p,
            distance: len(1.0),
        },
    );
    doc
}

const PART_BODY: RecipeNodeId = RecipeNodeId(2);

fn in_part(instance: RecipeNodeId, cap: CapEnd) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node: instance,
        path: vec![RoleSeg::InPart {
            of: Box::new(StableName {
                kind: EntityKind::Face,
                node: PART_BODY,
                path: vec![RoleSeg::Cap(cap)],
            }),
        }],
    }
}

fn mate_frame(origin: [f64; 3]) -> MateFrame {
    MateFrame {
        origin,
        axis: [0.0, 0.0, 1.0],
        reference: [1.0, 0.0, 0.0],
    }
}

fn rest_mate(a: RecipeNodeId, b: RecipeNodeId) -> Node<editor_core::ProfileProgram> {
    Node::Mate {
        a: in_part(a, CapEnd::Top),
        b: in_part(b, CapEnd::Bottom),
        class: ContactClass::Rest,
        alignment: Alignment {
            a: mate_frame([0.0, 0.0, 1.0]),
            b: mate_frame([0.0, 0.0, 0.0]),
            primitive: MatePrimitive::FrameCoincidence,
            sense: AxisSense::Aligned,
            clocking: None,
        },
    }
}

/// Two instances of `part_ref` and a seating mate, under `id`.
fn stacked(id: DocumentId, part_ref: DocRef) -> (ProfileDoc, Vec<RecipeNodeId>, RecipeNodeId) {
    let doc = ProfileDoc::empty(id, Tol::witness());
    let (doc, a) = insert(doc, Node::instantiate_part(part_ref));
    let (doc, b) = insert(doc, Node::instantiate_part(part_ref));
    let (doc, mate) = step(
        doc,
        DocEdit::InsertNode {
            node: rest_mate(a, b),
        },
    );
    (doc, vec![a, b], mate.expect("the mate mints"))
}

fn run(doc: &ProfileDoc, prior: Option<&Evaluation<f64>>, opts: &EvalOptions) -> Evaluation<f64> {
    evaluate::<f64>(doc, prior, &CancelToken::new(), opts, Tol::witness())
}

fn opts(store: StubStore) -> EvalOptions {
    EvalOptions {
        resolver: Some(Arc::new(store)),
        ..EvalOptions::default()
    }
}

fn assert_keys_collide(a: &Evaluation<f64>, b: &Evaluation<f64>) {
    assert_eq!(a.order, b.order, "one recipe, one id sequence");
    for &id in &a.order {
        let (Some(NodeResult::Ok(x)), Some(NodeResult::Ok(y))) = (a.result(id), b.result(id))
        else {
            panic!("{id:?} is not green on both sides: {:?} / {:?}", a.result(id), b.result(id));
        };
        assert_eq!(x.content_key, y.content_key, "{id:?}: content key");
        assert_eq!(x.naming_key, y.naming_key, "{id:?}: naming key");
    }
}

// ---- R1P1: a MATED twin pair (C2, own construction) ----

/// Two assemblies of one recipe — two instances and a `Rest` mate —
/// under two ids, sharing one stored part: every node id, content key
/// and naming key agrees, the mate included. A prior from the twin is
/// refused whole; the poses door refuses the twin's document.
#[test]
fn r1p1_a_mated_twin_pair_collides_on_every_key_and_reuses_nothing() {
    let mut store = StubStore::default();
    let part_ref = store.insert(cube_part("r1p1-part"), Tol::witness());
    let o = opts(store);
    let (a, ids_a, mate_a) = stacked(DocumentId::derive("r1p1-a"), part_ref);
    let (b, ids_b, mate_b) = stacked(DocumentId::derive("r1p1-b"), part_ref);
    assert_ne!(a.id(), b.id());
    assert_eq!((ids_a, mate_a), (ids_b, mate_b), "the ids collide by construction");

    let ev_a = run(&a, None, &o);
    let ev_b_cold = run(&b, None, &o);
    assert_eq!(ev_a.reused, 0);
    assert_keys_collide(&ev_a, &ev_b_cold);
    assert!(ev_a.part_evaluations > 0, "the seam was crossed cold");

    let ev_b = run(&b, Some(&ev_a), &o);
    assert_eq!(ev_b.reused, 0, "a colliding foreign prior served a hit");
    assert_eq!(ev_b.recomputed, b.len());
    assert_eq!(ev_b.part_evaluations, ev_b_cold.part_evaluations, "the seam was crossed again");
    assert_eq!(
        ev_b.prior_ignored,
        Some(editor_core::PriorIgnored {
            expected: b.id(),
            found: a.id(),
        })
    );
    // And the same-document control still memoizes in full.
    let ev_b_warm = run(&b, Some(&ev_b), &o);
    assert_eq!(ev_b_warm.reused, b.len());
    assert_eq!(ev_b_warm.prior_ignored, None);

    // The poses door, on a mated (non-singleton) cluster.
    let poses = solve_document(&a, Tol::witness());
    poses.placement(&a, mate_a).err();
    poses.placement(&a, ids_a_of(&a)[1]).expect("its own document places the seated instance");
    match *poses.placement(&b, ids_a_of(&a)[1]).expect_err("the twin's document refuses") {
        MateFault::PosesOfAnotherDocument { expected, found } => {
            assert_eq!((expected, found), (b.id(), a.id()));
        }
        other => panic!("expected the pairing refusal, got {other:?}"),
    }
    // The gate: the twin's evaluation refuses BEFORE the at-rest gate.
    assert!(matches!(
        assemble(&b, &ev_a, Tol::witness()),
        Err(editor_core::AssemblyError::Product(ref e))
            if matches!(**e, ProductError::EvaluationOfAnotherDocument { .. })
    ));
    assemble(&b, &ev_b, Tol::witness()).expect("the matched pair passes the gate");
}

fn ids_a_of(doc: &ProfileDoc) -> Vec<RecipeNodeId> {
    doc.order()
        .iter()
        .copied()
        .filter(|&id| matches!(doc.node(id), Some(Node::InstantiatePart { .. })))
        .collect()
}

// ---- R1P2: a corpus twin re-minted through the persist header ----

/// The tour's die, saved and re-loaded under another id: the persist
/// round trip is bit-identical (D9), so the twin agrees on every key.
/// A second, differently built colliding pair for C2 and C3.
#[test]
fn r1p2_a_reminted_corpus_die_collides_and_is_refused_at_every_door() {
    let die = crate::corpus::die::document().doc;
    let text = editor_core::save(&die, &[], Tol::witness()).expect("saves");
    let old = die.id().hex();
    let fresh = DocumentId::derive("r1p2-reminted-die").hex();
    assert!(text.contains(&old));
    let twin = editor_core::load(&text.replace(&old, &fresh), Tol::witness())
        .expect("loads under the new id")
        .doc;
    assert_ne!(twin.id(), die.id());
    assert_eq!(twin.len(), die.len());

    let o = EvalOptions::default();
    let ev_die = run(&die, None, &o);
    let ev_twin_cold = run(&twin, None, &o);
    assert_keys_collide(&ev_die, &ev_twin_cold);

    let ev_twin = run(&twin, Some(&ev_die), &o);
    assert_eq!(ev_twin.reused, 0, "the die's prior served the twin");
    assert_eq!(ev_twin.recomputed, twin.len());
    assert!(ev_twin.prior_ignored.is_some());

    // Both directions of every product door refuse, naming the pair.
    for (doc, ev) in [(&twin, &ev_die), (&die, &ev_twin_cold)] {
        match product(doc, ev, Tol::witness()).expect_err("refuses") {
            ProductError::EvaluationOfAnotherDocument { expected, found } => {
                assert_eq!((expected, found), (doc.id(), ev.document));
            }
            other => panic!("{other:?}"),
        }
        assert!(product_recorded(doc, ev, Tol::witness()).is_err());
    }
    // The matched pair still gathers.
    product(&twin, &ev_twin_cold, Tol::witness()).expect("gathers");
}

// ---- R1P3: the `run_checks` door (C3) — expected RED on the head ----

/// C3 says every (document, evaluation) door refuses a mismatch.
/// `run_checks` with the separation resident `Off` reads the foreign
/// evaluation through `connectedness` and answers. This row states
/// the C3 contract and is expected to FAIL on `e8d0022d`.
#[test]
fn r1p3_run_checks_refuses_a_foreign_evaluation_with_separation_off() {
    let a = cube_part("r1p3-a");
    let b = {
        let text = editor_core::save(&a, &[], Tol::witness()).expect("saves");
        editor_core::load(
            &text.replace(&a.id().hex(), &DocumentId::derive("r1p3-b").hex()),
            Tol::witness(),
        )
        .expect("loads")
        .doc
    };
    let ev_a = run(&a, None, &EvalOptions::default());
    let cfg = ChecksConfig {
        separation: Advisory::Off,
        ..ChecksConfig::default()
    };
    let out = run_checks(&b, &ev_a, &cfg, Tol::witness());
    assert!(
        matches!(out, Err(ChecksError::Product { .. })),
        "run_checks answered about another document's evaluation: {out:?}"
    );
}

/// The control: with the separation resident on, the gather's refusal
/// is inherited (the PR's claim for the default configuration).
#[test]
fn r1p3b_run_checks_inherits_the_refusal_with_separation_on() {
    let a = cube_part("r1p3b-a");
    let b = {
        let text = editor_core::save(&a, &[], Tol::witness()).expect("saves");
        editor_core::load(
            &text.replace(&a.id().hex(), &DocumentId::derive("r1p3b-b").hex()),
            Tol::witness(),
        )
        .expect("loads")
        .doc
    };
    let ev_a = run(&a, None, &EvalOptions::default());
    let out = run_checks(&b, &ev_a, &ChecksConfig::default(), Tol::witness());
    assert!(matches!(out, Err(ChecksError::Product { .. })), "{out:?}");
    run_checks(&a, &ev_a, &ChecksConfig::default(), Tol::witness()).expect("its own evaluation");
}

// ---- R1P4: the stamp is a `pub` field (demonstration, green) ----

/// `Evaluation::document` is public and writable: a caller can re-stamp
/// a foreign evaluation and every door then answers about the other
/// document's geometry. Not a contract row — a demonstration that the
/// pairing is held by convention past the constructor.
#[test]
fn r1p4_a_restamped_foreign_evaluation_passes_every_door() {
    let a = cube_part("r1p4-a");
    let b = {
        let text = editor_core::save(&a, &[], Tol::witness()).expect("saves");
        editor_core::load(
            &text.replace(&a.id().hex(), &DocumentId::derive("r1p4-b").hex()),
            Tol::witness(),
        )
        .expect("loads")
        .doc
    };
    let mut forged = run(&a, None, &EvalOptions::default());
    forged.document = b.id();
    product(&b, &forged, Tol::witness()).expect("the forged stamp passes the gather");
    let warm = run(&b, Some(&forged), &EvalOptions::default());
    assert_eq!(warm.reused, b.len(), "the forged stamp primes the memo in full");
    let _ = Frame::translation([0.0, 0.0, 0.0]);
}
