//! M4 PR 4 spec D3/D5: the `Rebind` DocEdit (explicit, recorded,
//! one-shot rewrite; every refusal door typed; the auto-menu is
//! EMPTY — nothing here follows anything) and the solver-contract
//! document semantics (`ReWitness`/`ReWitnessBulk` doors, witness
//! storage under GQ3, content-key movement, replay determinism).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use editor_core::ExtrudeSide;

use editor_core::{
    BifurcationKind, BooleanCoincidence, BranchCertification, BranchMarginEvidence, CancelToken,
    CapEnd, Diagnosis, DocEdit, EditError, EntityKind, EvalOptions, Evaluation, Implicated, Node,
    ProfileDoc, RecipeNodeId, Resolution, RoleSeg, RunCtx, SitedRef, StableName, WitnessAge,
    WitnessBifurcation, WitnessDatum, evaluate, resolve,
};
use fixture::{insert, len, on_frame, step};
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
) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let (doc, p) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(x0, y0), (x1, y0), (x1, y1), (x0, y1)]],
    );
    let (doc, e) = insert(
        doc,
        Node::Extrude {
            profile: p,
            distance: len(1.0),
            side: ExtrudeSide::Along,
        },
    );
    (doc, p, e)
}

fn cap(node: RecipeNodeId) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node,
        path: vec![RoleSeg::Cap(CapEnd::End)],
    }
}

/// The same cap, read at the node that mints it — what a declaration
/// sited at that node says.
fn sited(node: RecipeNodeId) -> SitedRef {
    SitedRef::at_mint(cap(node))
}

/// Disjoint blocks A, B, C, D, plus a union `decl` of A and D whose
/// declared pair names A's cap read at A and B's cap read at D.
///
/// Every side is sited at a member and names a cap minted before the
/// union — the rule every door that writes a pair asks. Neither B nor
/// C is a member, so deleting either is allowed: a declared name is a
/// reference, not a DAG edge. Whether D's table carries B's cap is the
/// evaluation's question (`Vanished`), not these doors'.
struct Three {
    doc: ProfileDoc,
    a: RecipeNodeId,
    b: RecipeNodeId,
    c: RecipeNodeId,
    d: RecipeNodeId,
    decl: RecipeNodeId,
}

fn three() -> Three {
    let doc = ProfileDoc::empty_derived("m4_pr4_edits", Tol::witness());
    let (doc, _, a) = block(doc, (0.0, 1.0), (0.0, 1.0));
    let (doc, _, b) = block(doc, (2.0, 3.0), (0.0, 1.0));
    let (doc, _, c) = block(doc, (4.0, 5.0), (0.0, 1.0));
    let (doc, _, d) = block(doc, (6.0, 7.0), (0.0, 1.0));
    let (doc, decl) = insert(
        doc,
        Node::Union {
            members: vec![a, d],
            declare: editor_core::declare_rest(vec![(sited(a), SitedRef::new(d, cap(b)))]),
        },
    );
    Three {
        doc,
        a,
        b,
        c,
        d,
        decl,
    }
}

/// The declared-pair list of the union `decl`.
fn declared(doc: &ProfileDoc, decl: RecipeNodeId) -> &[editor_core::DeclaredPair] {
    let Some(Node::Union { declare, .. }) = doc.node(decl) else {
        panic!("the declaring union is live");
    };
    declare
}

// ---- Rebind: semantics ----

#[test]
fn rebind_rewrites_declare_sites_one_shot() {
    let t = three();
    let applied = t
        .doc
        .apply(
            &DocEdit::Rebind {
                from: cap(t.b),
                to: cap(t.c),
            },
            Tol::witness(),
            &editor_core::RefusingReach,
        )
        .unwrap();
    assert!(applied.record.structural, "declared pairs changed");
    assert_eq!(
        declared(&applied.doc, t.decl),
        [(
            (sited(t.a), SitedRef::new(t.d, cap(t.c))),
            BooleanCoincidence::REST
        )]
    );
    // One-shot: no alias table — a SECOND rebind of the same source
    // now finds no references (the site says cap(c) already).
    assert_eq!(
        applied
            .doc
            .apply(
                &DocEdit::Rebind {
                    from: cap(t.b),
                    to: cap(t.a),
                },
                Tol::witness(),
                &editor_core::RefusingReach
            )
            .unwrap_err(),
        EditError::RebindNoReferences {
            name: applied.doc.spoken_name(&cap(t.b))
        }
    );
    // Purity: the input document is untouched.
    assert_eq!(
        declared(&t.doc, t.decl),
        [(
            (sited(t.a), SitedRef::new(t.d, cap(t.b))),
            BooleanCoincidence::REST
        )]
    );
}

#[test]
fn rebind_repairs_a_stranded_name_after_node_gone() {
    let t = three();
    // Delete B (allowed: B is no member of the union) — cap(b)
    // strands as NodeGone; Rebind is THE repair.
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
    assert_eq!(
        declared(&doc, t.decl),
        [(
            (sited(t.a), SitedRef::new(t.d, cap(t.c))),
            BooleanCoincidence::REST
        )]
    );
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

// ---- SetDeclare: the whole-list replace and its refusal doors ----

#[test]
fn set_declare_replaces_the_whole_list_and_refuses_typed() {
    let t = three();
    let pair = |x, y| editor_core::declare_rest(vec![(sited(x), SitedRef::new(t.d, cap(y)))]);
    let set = |doc: &ProfileDoc, node, pairs| {
        doc.apply(
            &DocEdit::SetDeclare { node, pairs },
            Tol::witness(),
            &editor_core::RefusingReach,
        )
    };
    let applied = set(&t.doc, t.decl, pair(t.a, t.c)).expect("a live union takes a new list");
    assert_eq!(applied.record.minted, None, "SetDeclare mints nothing");
    assert!(applied.record.structural, "SetDeclare is structural");
    assert!(
        applied.maintenance.is_empty(),
        "SetDeclare maintains nothing"
    );
    assert_eq!(
        declared(&applied.doc, t.decl),
        [(
            (sited(t.a), SitedRef::new(t.d, cap(t.c))),
            BooleanCoincidence::REST
        )],
        "the list is REPLACED, not appended to"
    );
    let cleared = set(&applied.doc, t.decl, Vec::new()).expect("an empty list clears");
    assert!(
        declared(&cleared.doc, t.decl).is_empty(),
        "an empty list clears"
    );
    assert_eq!(
        set(&t.doc, t.a, pair(t.a, t.b)).unwrap_err(),
        EditError::SetDeclareOnNonDeclaring {
            node: t.doc.spoken(t.a)
        },
        "an extrude declares no contacts"
    );
    let (doc_del, _) = step(t.doc.clone(), DocEdit::DeleteNode { id: t.c });
    assert_eq!(
        set(&doc_del, t.c, Vec::new()).unwrap_err(),
        EditError::UnknownNode {
            id: editor_core::SpokenNode::absent(t.c)
        },
        "a dead id is unknown"
    );
    assert_eq!(
        set(&doc_del, t.decl, pair(t.a, t.c)).unwrap_err(),
        EditError::DeclareNamesMissingNode {
            name: doc_del.spoken_name(&cap(t.c))
        },
        "a pair naming a dead node is refused as an insert's would be"
    );
}

// ---- Rebind: every refusal door ----

#[test]
fn rebind_refusal_doors_are_typed_and_specific() {
    let t = three();
    // Identity.
    assert_eq!(
        t.doc
            .apply(
                &DocEdit::Rebind {
                    from: cap(t.b),
                    to: cap(t.b),
                },
                Tol::witness(),
                &editor_core::RefusingReach
            )
            .unwrap_err(),
        EditError::RebindIdentity {
            name: t.doc.spoken_name(&cap(t.b))
        }
    );
    // Kind mismatch: a face reference cannot become a body.
    let body_c = StableName {
        kind: EntityKind::Body,
        node: t.c,
        path: vec![RoleSeg::OutputBody],
    };
    assert_eq!(
        t.doc
            .apply(
                &DocEdit::Rebind {
                    from: cap(t.b),
                    to: body_c,
                },
                Tol::witness(),
                &editor_core::RefusingReach
            )
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
            .apply(
                &DocEdit::Rebind {
                    from: cap(t.b),
                    to: cap(t.c),
                },
                Tol::witness(),
                &editor_core::RefusingReach
            )
            .unwrap_err(),
        EditError::RebindTargetMissingNode {
            name: doc_del.spoken_name(&cap(t.c))
        }
    );
    // Never-minted source id: a typo, not a NodeGone repair.
    let foreign = cap(RecipeNodeId(9999));
    assert_eq!(
        t.doc
            .apply(
                &DocEdit::Rebind {
                    from: foreign.clone(),
                    to: cap(t.c),
                },
                Tol::witness(),
                &editor_core::RefusingReach
            )
            .unwrap_err(),
        EditError::RebindUnknownName {
            name: t.doc.spoken_name(&foreign)
        }
    );
    // A target minted AFTER the declaring union: the rewritten pair
    // would name what none of its members can hold, so the rebind
    // refuses with the union's own rule, and no document results.
    let (late, _, e) = block(t.doc.clone(), (8.0, 9.0), (0.0, 1.0));
    assert_eq!(
        late.apply(
            &DocEdit::Rebind {
                from: cap(t.b),
                to: cap(e),
            },
            Tol::witness(),
            &editor_core::RefusingReach
        )
        .unwrap_err(),
        EditError::DeclaredNameNotUpstream {
            node: late.spoken(t.decl),
            name: late.spoken_name(&cap(e)),
        }
    );
    // Zero document sites.
    assert_eq!(
        t.doc
            .apply(
                &DocEdit::Rebind {
                    from: cap(t.a), // A's cap is the LEFT of the pair; it IS referenced
                    to: cap(t.c),
                },
                Tol::witness(),
                &editor_core::RefusingReach
            )
            .map(|_| ())
            .err(),
        None,
        "left-hand pair names are references too"
    );
    assert_eq!(
        t.doc
            .apply(
                &DocEdit::Rebind {
                    from: cap(t.c), // referenced nowhere
                    to: cap(t.a),
                },
                Tol::witness(),
                &editor_core::RefusingReach
            )
            .unwrap_err(),
        EditError::RebindNoReferences {
            name: t.doc.spoken_name(&cap(t.c))
        }
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
        .apply(
            &DocEdit::ReWitness {
                node: profile,
                witness: w.clone(),
            },
            Tol::witness(),
            &editor_core::RefusingReach,
        )
        .unwrap();
    assert!(!applied.record.structural);
    assert_eq!(applied.doc.witness(profile), Some(&w));
    assert_eq!(doc.witness(profile), None, "purity: input untouched");
    // Non-sketch and unknown nodes refuse typed.
    assert_eq!(
        doc.apply(
            &DocEdit::ReWitness {
                node: extrude,
                witness: w.clone(),
            },
            Tol::witness(),
            &editor_core::RefusingReach
        )
        .unwrap_err(),
        EditError::WitnessOnNonSketch {
            node: doc.spoken(extrude)
        }
    );
    assert_eq!(
        doc.apply(
            &DocEdit::ReWitness {
                node: RecipeNodeId(9999),
                witness: w.clone(),
            },
            Tol::witness(),
            &editor_core::RefusingReach
        )
        .unwrap_err(),
        EditError::UnknownNode {
            id: editor_core::SpokenNode::absent(RecipeNodeId(9999))
        }
    );
    // Replay determinism: same edits, bit-identical document
    // (witness bytes are exact data; bit_eq covers them).
    let redo = doc
        .apply(
            &DocEdit::ReWitness {
                node: profile,
                witness: w.clone(),
            },
            Tol::witness(),
            &editor_core::RefusingReach,
        )
        .unwrap();
    assert!(applied.doc.bit_eq(&redo.doc));
    // A different witness is a DIFFERENT document.
    let other = doc
        .apply(
            &DocEdit::ReWitness {
                node: profile,
                witness: datum(1, b"assignment-v2"),
            },
            Tol::witness(),
            &editor_core::RefusingReach,
        )
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
        .apply(
            &DocEdit::ReWitnessBulk {
                entries: vec![(p1, datum(1, b"w1")), (p2, datum(1, b"w2"))],
                certification: cert.clone(),
            },
            Tol::witness(),
            &editor_core::RefusingReach,
        )
        .unwrap();
    assert!(!applied.record.structural);
    assert_eq!(applied.doc.witness(p1), Some(&datum(1, b"w1")));
    assert_eq!(applied.doc.witness(p2), Some(&datum(1, b"w2")));
    // Shape doors.
    assert_eq!(
        doc.apply(
            &DocEdit::ReWitnessBulk {
                entries: vec![],
                certification: cert.clone(),
            },
            Tol::witness(),
            &editor_core::RefusingReach
        )
        .unwrap_err(),
        EditError::EmptyWitnessBulk
    );
    assert_eq!(
        doc.apply(
            &DocEdit::ReWitnessBulk {
                entries: vec![(p1, datum(1, b"w1")), (p1, datum(1, b"w1b"))],
                certification: cert.clone(),
            },
            Tol::witness(),
            &editor_core::RefusingReach
        )
        .unwrap_err(),
        EditError::DuplicateWitnessEntry {
            node: doc.spoken(p1)
        }
    );
    assert_eq!(
        doc.apply(
            &DocEdit::ReWitnessBulk {
                entries: vec![(e1, datum(1, b"w"))],
                certification: cert,
            },
            Tol::witness(),
            &editor_core::RefusingReach
        )
        .unwrap_err(),
        EditError::WitnessOnNonSketch {
            node: doc.spoken(e1)
        }
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

// ---- SetExtrudeSide ----

/// **`SetExtrudeSide` moves an extrude's side and refuses every other
/// kind typed.** The edit is structural (the side is recipe payload no
/// slot carries), leaves its input document untouched, and at a node
/// that has no side refuses `SetExtrudeSideOnNonExtrude` naming it.
#[test]
fn set_extrude_side_flips_an_extrude_and_refuses_another_kind() {
    let doc = ProfileDoc::empty_derived("m4_pr4_edits", Tol::witness());
    let (doc, profile, extrude) = block(doc, (0.0, 1.0), (0.0, 1.0));
    let side_of = |doc: &ProfileDoc| match doc.node(extrude) {
        Some(Node::Extrude { side, .. }) => *side,
        other => panic!("an extrude, got {other:?}"),
    };
    assert_eq!(side_of(&doc), ExtrudeSide::Along);
    let applied = doc
        .apply(
            &DocEdit::SetExtrudeSide {
                node: extrude,
                side: ExtrudeSide::Against,
            },
            Tol::witness(),
            &editor_core::RefusingReach,
        )
        .unwrap();
    assert!(applied.record.structural);
    assert_eq!(side_of(&applied.doc), ExtrudeSide::Against);
    assert_eq!(side_of(&doc), ExtrudeSide::Along, "purity: input untouched");
    assert_eq!(
        doc.apply(
            &DocEdit::SetExtrudeSide {
                node: profile,
                side: ExtrudeSide::Against,
            },
            Tol::witness(),
            &editor_core::RefusingReach
        )
        .unwrap_err(),
        EditError::SetExtrudeSideOnNonExtrude {
            node: doc.spoken(profile)
        }
    );
}
