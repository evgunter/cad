//! DOCM-4 — an evaluation carries the identity of the document it is
//! of, and every door taking a (document, evaluation) pair refuses a
//! mismatch typed (`docs/DOCM-IDENTITY-DESIGN.md` DI3).
//!
//! The defect these rows close is silent: node ids are minted per
//! document by a per-document counter, so two documents built from one
//! recipe carry the SAME ids with the SAME content keys. A gather, an
//! at-rest gate or a memo lookup handed the wrong evaluation therefore
//! misses nothing — it answers, in full, about other geometry. The
//! collision rows below are built to be that case rather than to hope
//! for it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use std::collections::BTreeMap;
use std::sync::Arc;

use editor_core::{
    AssemblyError, CancelToken, ContentPin, DocEdit, DocRef, DocumentId, EvalOptions, EvalOutcome,
    Evaluation, Frame, MateFault, Node, NodeResult, PartResolver, ProductError, ProfileDoc,
    RecipeNodeId, ResolveFailure, ResolveFault, assemble, content_pin, evaluate, product,
    product_named, product_recorded, solve_document,
};
use fixture::{insert, len, on_frame, square};
use geom_core::Tol;

// ---- Fixtures ----

/// A one-solid part document under `label`'s derived id: a `side`-wide
/// square extruded 1 tall.
fn part(label: &str, side: f64) -> ProfileDoc {
    part_of(DocumentId::derive(label), side)
}

/// The SAME recipe as [`part`], under a caller-chosen id — the twin
/// that makes an id collision a construction rather than a hope.
fn part_of(id: DocumentId, side: f64) -> ProfileDoc {
    let doc = ProfileDoc::empty(id, Tol::witness());
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(0.0, 0.0, side / 2.0)],
    );
    let (doc, _) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(1.0),
        },
    );
    doc
}

/// A resolver over an in-memory map, verifying the pin exactly as the
/// document layer's does.
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

/// An assembly-shaped document under `id`: two instances of `part_ref`,
/// the second translated.
fn assembly_of(id: DocumentId, part_ref: DocRef) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let doc = ProfileDoc::empty(id, Tol::witness());
    let (doc, a) = insert(doc, Node::instantiate_part(part_ref));
    let (doc, b) = insert(doc, Node::instantiate_part(part_ref));
    let (doc, _) = fixture::step(
        doc,
        DocEdit::SetPlacement {
            node: b,
            frame: Frame::translation([4.0, 0.0, 0.0]),
        },
    );
    (doc, vec![a, b])
}

fn with_resolver(store: StubStore) -> EvalOptions {
    EvalOptions {
        resolver: Some(Arc::new(store)),
        ..EvalOptions::default()
    }
}

fn run(doc: &ProfileDoc, prior: Option<&Evaluation<f64>>, opts: &EvalOptions) -> Evaluation<f64> {
    evaluate::<f64>(doc, prior, &CancelToken::new(), opts, Tol::witness())
}

fn plain(doc: &ProfileDoc, prior: Option<&Evaluation<f64>>) -> Evaluation<f64> {
    run(doc, prior, &EvalOptions::default())
}

fn volume_bits(body: &topo::Body<f64>) -> u64 {
    topo::mass_properties(body, Tol::witness())
        .expect("mass properties")
        .volume
        .to_bits()
}

// ---- A1: the stamp ----

/// A1 — `evaluate` stamps the evaluating document's id, and nothing
/// else: two documents of one recipe are told apart by it.
#[test]
fn every_evaluation_carries_its_document() {
    let a = part("docm4-a1-a", 1.0);
    let b = part_of(DocumentId::derive("docm4-a1-b"), 1.0);
    let ev_a = plain(&a, None);
    let ev_b = plain(&b, None);

    assert_eq!(ev_a.document, a.id());
    assert_eq!(ev_b.document, b.id());
    assert_ne!(ev_a.document, ev_b.document, "two documents, two ids");
    assert_eq!(ev_a.prior_ignored, None, "no prior, nothing refused");
    assert_eq!(ev_a.order, ev_b.order, "one recipe, one set of node ids");
}

/// A1 — the OTHER construction site: an all-nodes refusal is still an
/// evaluation of the document it refused about. A recorded ε that
/// disagrees with the process ε is the reachable one.
#[test]
fn an_all_nodes_refusal_carries_its_document_too() {
    let doc = part("docm4-a1-eps", 1.0);
    let retol = editor_core::apply(
        &doc,
        &DocEdit::SetTolerance {
            eps: Tol::witness().eps() * 2.0,
        },
        Tol::witness(),
    )
    .expect("SetTolerance applies as a pure doc edit")
    .doc;
    let ev = plain(&retol, None);
    assert!(
        ev.nodes.values().all(|r| matches!(
            r,
            NodeResult::Failed(e)
                if matches!(e.kind, editor_core::NodeErrorKind::ToleranceConflict { .. })
        )),
        "the eps conflict refuses every node"
    );
    assert_eq!(ev.document, retol.id(), "a refusal is still OF a document");
    assert_eq!(ev.outcome, EvalOutcome::Completed);
    assert_eq!(ev.prior_ignored, None);
}

/// C1 — the receipt for "no fourth constructor": `eval/mod.rs` is the
/// only file in the crate that names the struct literal, and every one
/// of its sites stamps `document`. Read from the source, because the
/// claim is about the code and not about a value.
#[test]
fn every_evaluation_literal_stamps_the_document() {
    let path = test_utils::source::crate_dir(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("eval")
        .join("mod.rs");
    let text = std::fs::read_to_string(&path).expect("this crate's own eval/mod.rs");
    // The CODE view: a struct literal quoted in a comment is prose, and
    // a guard that counted it would report on prose.
    let src = test_utils::source::code_only(&text);
    let sites: Vec<String> = src
        .split("\n    Evaluation {\n")
        .skip(1)
        .map(|rest| rest.lines().take(3).collect::<Vec<_>>().join("|"))
        .collect();
    assert_eq!(sites.len(), 2, "the constructor sites moved: {sites:?}");
    for site in &sites {
        assert!(
            site.contains("document:"),
            "an `Evaluation` literal that does not stamp its document: {site}"
        );
    }
}

// ---- A2: a foreign prior is refused, not mined ----

/// A2 row 1 — a prior of another document is dropped whole: zero
/// reuse, every node recomputed, and the refusal recorded typed on the
/// value the caller already reads.
#[test]
fn a_foreign_prior_is_refused_not_mined() {
    let a = part("docm4-a2-a", 1.0);
    let b = part("docm4-a2-b", 2.0);
    let ev_a = plain(&a, None);
    let ev_b = plain(&b, Some(&ev_a));

    assert_eq!(
        ev_b.reused, 0,
        "nothing may be served from another document"
    );
    assert_eq!(ev_b.recomputed, b.len(), "every node ran its op");
    assert_eq!(
        ev_b.prior_ignored,
        Some(editor_core::PriorIgnored {
            expected: b.id(),
            found: a.id(),
        })
    );
}

/// A2 row 2 — the sharp one. Two documents of ONE recipe: the same
/// node ids, and the same content and naming keys node for node, so
/// every memo lookup would have hit. The refusal is what makes the
/// reuse zero, not luck.
#[test]
fn a_twin_recipe_collides_on_every_key_and_is_still_refused() {
    let a = part_of(DocumentId::derive("docm4-a2-twin-a"), 1.0);
    let b = part_of(DocumentId::derive("docm4-a2-twin-b"), 1.0);
    assert_ne!(a.id(), b.id(), "two documents");

    let ev_a = plain(&a, None);
    let ev_b_cold = plain(&b, None);
    assert_eq!(ev_a.order, ev_b_cold.order, "the same ids");
    for &id in &ev_a.order {
        let (Some(NodeResult::Ok(x)), Some(NodeResult::Ok(y))) =
            (ev_a.result(id), ev_b_cold.result(id))
        else {
            panic!("both documents evaluate {id:?} green");
        };
        assert_eq!(x.content_key, y.content_key, "{id:?}: content key");
        assert_eq!(x.naming_key, y.naming_key, "{id:?}: naming key");
    }

    let ev_b = plain(&b, Some(&ev_a));
    assert_eq!(ev_b.reused, 0, "a collision is not a hit");
    assert_eq!(ev_b.recomputed, b.len());
    assert_eq!(
        ev_b.prior_ignored,
        Some(editor_core::PriorIgnored {
            expected: b.id(),
            found: a.id(),
        })
    );
}

// ---- A3: the doors refuse ----

/// A3 — the three product doors refuse a foreign evaluation, naming
/// both documents, before a root is read.
#[test]
fn the_product_doors_refuse_an_evaluation_of_another_document() {
    let a = part_of(DocumentId::derive("docm4-a3-a"), 1.0);
    let b = part_of(DocumentId::derive("docm4-a3-b"), 1.0);
    let ev_a = plain(&a, None);

    let expect = |err: ProductError| match err {
        ProductError::EvaluationOfAnotherDocument { expected, found } => {
            assert_eq!(expected, b.id());
            assert_eq!(found, a.id());
            let shown = err_text(&ProductError::EvaluationOfAnotherDocument { expected, found });
            assert!(shown.contains(&expected.hex()), "{shown}");
            assert!(shown.contains(&found.hex()), "{shown}");
        }
        other => panic!("expected the pairing refusal, got {other:?}"),
    };
    expect(product(&b, &ev_a, Tol::witness()).expect_err("refuses"));
    expect(product_named(&b, &ev_a, Tol::witness()).expect_err("refuses"));
    expect(product_recorded(&b, &ev_a, Tol::witness()).expect_err("refuses"));
}

fn err_text(err: &ProductError) -> String {
    format!("{err}")
}

/// A3 — `assemble` refuses through the gather: the pairing is checked
/// on the ONE path that reaches the at-rest gate, so no predicate here
/// ever runs on a mispaired argument.
#[test]
fn assemble_refuses_an_evaluation_of_another_document() {
    let mut store = StubStore::default();
    let part_ref = store.insert(part("docm4-a3-asm-part", 1.0), Tol::witness());
    let opts = with_resolver(store);
    let (a, _) = assembly_of(DocumentId::derive("docm4-a3-asm-a"), part_ref);
    let (b, _) = assembly_of(DocumentId::derive("docm4-a3-asm-b"), part_ref);
    let ev_a = run(&a, None, &opts);

    match assemble(&b, &ev_a, Tol::witness()).expect_err("refuses") {
        AssemblyError::Product(inner) => match *inner {
            ProductError::EvaluationOfAnotherDocument { expected, found } => {
                assert_eq!((expected, found), (b.id(), a.id()));
            }
            other => panic!("expected the pairing refusal, got {other:?}"),
        },
        other => panic!("expected an inherited gather refusal, got {other:?}"),
    }
}

/// A3 — a solve is a solve OF a document, and the door that reads a
/// document back refuses another one's, naming both.
#[test]
fn solved_poses_placement_refuses_another_document() {
    let mut store = StubStore::default();
    let part_ref = store.insert(part("docm4-a3-poses-part", 1.0), Tol::witness());
    let (a, ids_a) = assembly_of(DocumentId::derive("docm4-a3-poses-a"), part_ref);
    let (b, _) = assembly_of(DocumentId::derive("docm4-a3-poses-b"), part_ref);

    let poses = solve_document(&a, Tol::witness());
    assert_eq!(poses.document(), a.id());
    poses
        .placement(&a, ids_a[1])
        .expect("its own document places");

    match *poses.placement(&b, ids_a[1]).expect_err("refuses") {
        MateFault::PosesOfAnotherDocument { expected, found } => {
            assert_eq!((expected, found), (b.id(), a.id()));
        }
        other => panic!("expected the pairing refusal, got {other:?}"),
    }
}

/// A3's other half — none of the doors refuses a MATCH, and what a
/// matched pair gathers is bit-identical run to run. The die is the
/// tour's own document; the assembly-shaped one is two instances of a
/// stored part.
#[test]
fn a_matched_pair_gathers_bit_identically() {
    let die = crate::corpus::die::document();
    let one = plain(&die.doc, None);
    let two = plain(&die.doc, Some(&one));
    assert_eq!(one.document, die.doc.id());
    assert_eq!(two.prior_ignored, None, "its own prior is no refusal");
    let a = product(&die.doc, &one, Tol::witness()).expect("gathers");
    let b = product(&die.doc, &two, Tol::witness()).expect("gathers");
    assert_eq!(volume_bits(&a), volume_bits(&b), "the die's product");

    let mut store = StubStore::default();
    let part_ref = store.insert(part("docm4-a3-match-part", 1.0), Tol::witness());
    let opts = with_resolver(store);
    let (asm, _) = assembly_of(DocumentId::derive("docm4-a3-match-asm"), part_ref);
    let cold = run(&asm, None, &opts);
    let warm = run(&asm, Some(&cold), &opts);
    let p1 = product(&asm, &cold, Tol::witness()).expect("gathers");
    let p2 = product(&asm, &warm, Tol::witness()).expect("gathers");
    assert_eq!(volume_bits(&p1), volume_bits(&p2), "the assembly's product");
    assemble(&asm, &warm, Tol::witness()).expect("the matched pair passes the gate");
}

// ---- A4: the memo still works ----

/// A4 — a same-document re-evaluation reuses exactly what it did
/// before the pairing door existed: the whole document when nothing
/// moved, the cone complement when one node did. The corpus suite pins
/// the per-document numbers; this row pins the shape at the seam the
/// change touched.
#[test]
fn the_memo_still_serves_a_same_document_re_evaluation() {
    let mut store = StubStore::default();
    let part_ref = store.insert(part("docm4-a4-part", 1.0), Tol::witness());
    let opts = with_resolver(store);
    let (asm, ids) = assembly_of(DocumentId::derive("docm4-a4-asm"), part_ref);

    let cold = run(&asm, None, &opts);
    assert_eq!(cold.reused, 0, "a cold run has nothing to reuse");
    let warm = run(&asm, Some(&cold), &opts);
    assert_eq!(warm.reused, asm.len(), "an unedited document is all memo");
    assert_eq!(warm.recomputed, 0);
    assert_eq!(warm.part_evaluations, 0, "a memo hit crosses no seam");
    assert_eq!(warm.prior_ignored, None);

    let (moved, _) = fixture::step(
        asm.clone(),
        DocEdit::SetPlacement {
            node: ids[1],
            frame: Frame::translation([9.0, 0.0, 0.0]),
        },
    );
    let after = run(&moved, Some(&warm), &opts);
    assert_eq!(after.recomputed, 1, "the moved instance re-keys");
    assert_eq!(after.reused, moved.len() - 1, "and nothing else does");
}
