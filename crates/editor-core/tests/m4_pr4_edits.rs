//! M4 PR 4 spec D3/D5: the `Rebind` DocEdit (explicit, recorded,
//! one-shot rewrite; every refusal door typed; the auto-menu is
//! EMPTY — nothing here follows anything) and the solver-contract
//! document semantics (`ReWitness`/`ReWitnessBulk` doors, witness
//! storage under GQ3, content-key movement, replay determinism).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod fixture;

use editor_core::{
    BifurcationKind, BranchCertification, BranchMarginEvidence, CancelToken, CapEnd, ContactClass,
    Diagnosis, DocEdit, EditError, EntityKind, EvalOptions, Evaluation, Implicated, Node,
    ProfileDoc, RecipeNodeId, Resolution, RoleSeg, RunCtx, StableName, WitnessAge,
    WitnessBifurcation, WitnessDatum, evaluate, resolve,
};
use fixture::{desc, insert, len, step};
use geom_core::Tol;

fn run(doc: &ProfileDoc, prior: Option<&Evaluation<f64>>) -> Evaluation<f64> {
    evaluate::<f64>(doc, prior, &CancelToken::new(), &EvalOptions::default(), Tol::witness())
}

fn block(
    doc: ProfileDoc,
    (x0, x1): (f64, f64),
    (y0, y1): (f64, f64),
) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let (doc, p) = insert(
        doc,
        Node::Profile(desc(
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![vec![(x0, y0), (x1, y0), (x1, y1), (x0, y1)]],
        )),
    );
    let (doc, e) = insert(
        doc,
        Node::Extrude {
            profile: p,
            distance: len(1.0),
        },
    );
    (doc, p, e)
}

fn cap(node: RecipeNodeId) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node,
        path: vec![RoleSeg::Cap(CapEnd::Top)],
    }
}

/// Three disjoint blocks + a Declare pairing A's cap with B's cap.
struct Three {
    doc: ProfileDoc,
    a: RecipeNodeId,
    b: RecipeNodeId,
    c: RecipeNodeId,
    decl: RecipeNodeId,
}

fn three() -> Three {
    let doc = ProfileDoc::empty_derived("m4_pr4_edits", Tol::witness());
    let (doc, _, a) = block(doc, (0.0, 1.0), (0.0, 1.0));
    let (doc, _, b) = block(doc, (2.0, 3.0), (0.0, 1.0));
    let (doc, _, c) = block(doc, (4.0, 5.0), (0.0, 1.0));
    let (doc, decl) = insert(doc, Node::declare_rest(vec![(cap(a), cap(b))]));
    Three { doc, a, b, c, decl }
}

// ---- Rebind: semantics ----

#[test]
fn rebind_rewrites_declare_sites_one_shot() {
    let t = three();
    let applied = t
        .doc
        .apply(&DocEdit::Rebind {
            from: cap(t.b),
            to: cap(t.c),
        }, Tol::witness())
        .unwrap();
    assert!(applied.record.structural, "Declare payloads changed");
    let Some(Node::Declare { pairs }) = applied.doc.node(t.decl) else {
        panic!("declare survives");
    };
    assert_eq!(pairs, &vec![((cap(t.a), cap(t.c)), ContactClass::Rest)]);
    // One-shot: no alias table — a SECOND rebind of the same source
    // now finds no references (the site says cap(c) already).
    assert_eq!(
        applied
            .doc
            .apply(&DocEdit::Rebind {
                from: cap(t.b),
                to: cap(t.a),
            }, Tol::witness())
            .unwrap_err(),
        EditError::RebindNoReferences { name: cap(t.b) }
    );
    // Purity: the input document is untouched.
    let Some(Node::Declare { pairs }) = t.doc.node(t.decl) else {
        panic!()
    };
    assert_eq!(pairs, &vec![((cap(t.a), cap(t.b)), ContactClass::Rest)]);
}

#[test]
fn rebind_repairs_a_stranded_name_after_node_gone() {
    let t = three();
    // Delete B (allowed: Declare names are refs, not DAG edges) —
    // cap(b) strands as NodeGone; Rebind is THE repair.
    let (doc, _) = step(t.doc, DocEdit::DeleteNode { id: t.b });
    let ev = run(&doc, None);
    assert!(matches!(
        resolve(RunCtx { doc: &doc, eval: &ev }, &cap(t.b)),
        Resolution::Failed(f) if matches!(f.error, editor_core::ResolveError::NodeGone { .. })
    ));
    // The source's node is dead-but-once-lived: the rebind APPLIES.
    let (doc, _) = step(
        doc,
        DocEdit::Rebind {
            from: cap(t.b),
            to: cap(t.c),
        },
    );
    let ev = run(&doc, None);
    let Some(Node::Declare { pairs }) = doc.node(t.decl) else {
        panic!()
    };
    assert_eq!(pairs, &vec![((cap(t.a), cap(t.c)), ContactClass::Rest)]);
    assert!(matches!(
        resolve(
            RunCtx {
                doc: &doc,
                eval: &ev
            },
            &cap(t.c)
        ),
        Resolution::Resolved(_)
    ));
}

// ---- Rebind: every refusal door ----

#[test]
fn rebind_refusal_doors_are_typed_and_specific() {
    let t = three();
    // Identity.
    assert_eq!(
        t.doc
            .apply(&DocEdit::Rebind {
                from: cap(t.b),
                to: cap(t.b),
            }, Tol::witness())
            .unwrap_err(),
        EditError::RebindIdentity { name: cap(t.b) }
    );
    // Kind mismatch: a face reference cannot become a body.
    let body_c = StableName {
        kind: EntityKind::Body,
        node: t.c,
        path: vec![RoleSeg::OutputBody],
    };
    assert_eq!(
        t.doc
            .apply(&DocEdit::Rebind {
                from: cap(t.b),
                to: body_c,
            }, Tol::witness())
            .unwrap_err(),
        EditError::RebindKindMismatch {
            from: EntityKind::Face,
            to: EntityKind::Body,
        }
    );
    // Target node not live.
    let (doc_del, _) = step(t.doc.clone(), DocEdit::DeleteNode { id: t.c });
    assert_eq!(
        doc_del
            .apply(&DocEdit::Rebind {
                from: cap(t.b),
                to: cap(t.c),
            }, Tol::witness())
            .unwrap_err(),
        EditError::RebindTargetMissingNode { name: cap(t.c) }
    );
    // Never-minted source id: a typo, not a NodeGone repair.
    let foreign = cap(RecipeNodeId(9999));
    assert_eq!(
        t.doc
            .apply(&DocEdit::Rebind {
                from: foreign.clone(),
                to: cap(t.c),
            }, Tol::witness())
            .unwrap_err(),
        EditError::RebindUnknownName { name: foreign }
    );
    // Zero document sites.
    assert_eq!(
        t.doc
            .apply(&DocEdit::Rebind {
                from: cap(t.a), // A's cap is the LEFT of the pair; it IS referenced
                to: cap(t.c),
            }, Tol::witness())
            .map(|_| ())
            .err(),
        None,
        "left-hand pair names are references too"
    );
    assert_eq!(
        t.doc
            .apply(&DocEdit::Rebind {
                from: cap(t.c), // referenced nowhere
                to: cap(t.a),
            }, Tol::witness())
            .unwrap_err(),
        EditError::RebindNoReferences { name: cap(t.c) }
    );
}

// ---- ReWitness / ReWitnessBulk ----

fn datum(schema: u32, bytes: &[u8]) -> WitnessDatum {
    WitnessDatum {
        schema,
        bytes: bytes.to_vec(),
    }
}

#[test]
fn rewitness_stores_on_sketch_nodes_only_and_replays() {
    let doc = ProfileDoc::empty_derived("m4_pr4_edits", Tol::witness());
    let (doc, profile, extrude) = block(doc, (0.0, 1.0), (0.0, 1.0));
    let w = datum(1, b"assignment-v1");
    let applied = doc
        .apply(&DocEdit::ReWitness {
            node: profile,
            witness: w.clone(),
        }, Tol::witness())
        .unwrap();
    assert!(!applied.record.structural);
    assert_eq!(applied.doc.witness(profile), Some(&w));
    assert_eq!(doc.witness(profile), None, "purity: input untouched");
    // Non-sketch and unknown nodes refuse typed.
    assert_eq!(
        doc.apply(&DocEdit::ReWitness {
            node: extrude,
            witness: w.clone(),
        }, Tol::witness())
        .unwrap_err(),
        EditError::WitnessOnNonSketch { node: extrude }
    );
    assert_eq!(
        doc.apply(&DocEdit::ReWitness {
            node: RecipeNodeId(9999),
            witness: w.clone(),
        }, Tol::witness())
        .unwrap_err(),
        EditError::UnknownNode {
            id: RecipeNodeId(9999)
        }
    );
    // Replay determinism: same edits, bit-identical document
    // (witness bytes are exact data; bit_eq covers them).
    let redo = doc
        .apply(&DocEdit::ReWitness {
            node: profile,
            witness: w.clone(),
        }, Tol::witness())
        .unwrap();
    assert!(applied.doc.bit_eq(&redo.doc));
    // A different witness is a DIFFERENT document.
    let other = doc
        .apply(&DocEdit::ReWitness {
            node: profile,
            witness: datum(1, b"assignment-v2"),
        }, Tol::witness())
        .unwrap();
    assert!(!applied.doc.bit_eq(&other.doc));
    // The doc diff reports the witness delta.
    let d = doc.diff(&applied.doc);
    assert_eq!(d.witnesses, vec![profile]);
    assert!(!d.is_empty());
    // Deleting the node buries its witness with it (ids never
    // reused — the entry could never be read again). The extrude
    // must go first (DAG edge).
    let (doc2, _) = step(applied.doc, DocEdit::DeleteNode { id: extrude });
    let (doc2, _) = step(doc2, DocEdit::DeleteNode { id: profile });
    assert!(doc2.witnesses().is_empty());
}

#[test]
fn rewitness_bulk_validates_shape_and_carries_certification_as_data() {
    let doc = ProfileDoc::empty_derived("m4_pr4_edits", Tol::witness());
    let (doc, p1, e1) = block(doc, (0.0, 1.0), (0.0, 1.0));
    let (doc, p2, _e2) = block(doc, (2.0, 3.0), (0.0, 1.0));
    let cert = BranchCertification {
        schema: 1,
        bytes: b"krawczyk-boxes".to_vec(),
    };
    let applied = doc
        .apply(&DocEdit::ReWitnessBulk {
            entries: vec![(p1, datum(1, b"w1")), (p2, datum(1, b"w2"))],
            certification: cert.clone(),
        }, Tol::witness())
        .unwrap();
    assert!(!applied.record.structural);
    assert_eq!(applied.doc.witness(p1), Some(&datum(1, b"w1")));
    assert_eq!(applied.doc.witness(p2), Some(&datum(1, b"w2")));
    // Shape doors.
    assert_eq!(
        doc.apply(&DocEdit::ReWitnessBulk {
            entries: vec![],
            certification: cert.clone(),
        }, Tol::witness())
        .unwrap_err(),
        EditError::EmptyWitnessBulk
    );
    assert_eq!(
        doc.apply(&DocEdit::ReWitnessBulk {
            entries: vec![(p1, datum(1, b"w1")), (p1, datum(1, b"w1b"))],
            certification: cert.clone(),
        }, Tol::witness())
        .unwrap_err(),
        EditError::DuplicateWitnessEntry { node: p1 }
    );
    assert_eq!(
        doc.apply(&DocEdit::ReWitnessBulk {
            entries: vec![(e1, datum(1, b"w"))],
            certification: cert,
        }, Tol::witness())
        .unwrap_err(),
        EditError::WitnessOnNonSketch { node: e1 }
    );
}

#[test]
fn witness_change_moves_the_content_key_and_reproduces_bits() {
    let doc = ProfileDoc::empty_derived("m4_pr4_edits", Tol::witness());
    let (doc, profile, extrude) = block(doc, (0.0, 1.0), (0.0, 1.0));
    let ev1 = run(&doc, None);
    let (doc2, _) = step(
        doc.clone(),
        DocEdit::ReWitness {
            node: profile,
            witness: datum(1, b"w"),
        },
    );
    let ev2 = run(&doc2, Some(&ev1));
    // W5 purity: the witness is a content-key input, so its cone
    // recomputes (profile + extrude — nothing is silently reused
    // against a changed input)...
    assert_eq!(ev2.recomputed, 2, "witness cone = profile + extrude");
    assert_ne!(
        ev1.value(profile).unwrap().content_key,
        ev2.value(profile).unwrap().content_key
    );
    // ...and W4 invisibility holds BY RE-DERIVATION: v1 evaluation
    // does not read the witness, so tables and verdicts reproduce
    // exactly.
    for id in [profile, extrude] {
        assert_eq!(
            ev1.value(id).unwrap().name_table,
            ev2.value(id).unwrap().name_table
        );
        assert_eq!(
            ev1.value(id).unwrap().verdicts,
            ev2.value(id).unwrap().verdicts
        );
    }
}

// ---- The W3 type and its N5 arm (shape pins; M6 fills the logic) ----

#[test]
fn witness_bifurcation_payload_and_diagnosis_arm_compose() {
    // Payload values derived from the ambient tolerance (discipline:
    // no hard-coded ε anywhere, even in constructed evidence).
    let tol = geom_core::Tol::witness().get();
    let bif = WitnessBifurcation {
        kind: BifurcationKind::FoldProximity,
        margin: BranchMarginEvidence {
            margin: tol.eps / 1024.0,
            band_zero: tol.eps,
            band_escalate: tol.eps * tol.k,
        },
        implicated: vec![
            Implicated::Constraint(3),
            Implicated::Entity(cap(RecipeNodeId(0))),
        ],
        witness_age: WitnessAge {
            solved_under: b"d=12".to_vec(),
            at_solve: b"d=13.999999999".to_vec(),
        },
    };
    let diag = Diagnosis::WitnessBifurcation(Box::new(bif.clone()));
    let Diagnosis::WitnessBifurcation(inner) = diag else {
        panic!()
    };
    assert_eq!(*inner, bif);
    assert_eq!(inner.kind, BifurcationKind::FoldProximity);
}
