//! M4 PR 4 spec D1/D3: the N5 resolution ladder end to end —
//! Resolved / Ambiguous (direct tie and the order_along over-tie
//! widening) / NodeGone / Vanished with the verdict-diff diagnosis
//! (PredicateFlip, StructuralParam, Cascade) + tombstones / typed
//! Indeterminate — plus N3 offers, the rebind suggestion ladder, and
//! the R6 name-level edit-time validation door.
//!
//! SWEEP-STRATEGY NOTE (Evan's 2026-07-29 ruling): this file's pins
//! are about diff/resolve engine behavior GIVEN verdicts, so its
//! evaluator deliberately runs the idealized (verdict-rich) sweep;
//! the production-path degradation is pinned in `m4_pr4_banked`
//! (both strategies side by side) and in the re-pinned `m4_pr4_ci`
//! golden.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod fixture;

use std::sync::Arc;

use editor_core::eval::WitnessSlot;
use editor_core::{
    BooleanOp, CancelToken, CapEnd, ContentKey, Diagnosis, DocEdit, EntityKind, Entry, EvalOptions,
    EvalOutcome, Evaluation, NameTable, NamingKey, Node, ProfileDoc, Qualifier, RecipeEditRef,
    RecipeNodeId, Resolution, ResolveError, ResolveIndeterminate, RoleSeg, RunCtx, SlotId,
    StableName, apply_with_names, evaluate, rebind_suggestions, resolve, resolve_with_prior,
};
use fixture::{ang, desc, insert, len, scl, step};
use geom_core::Tol;

/// Idealized (brute-force) boolean sweep since M5 PR 8: this file
/// pins the DIFF/RESOLVE engine's semantics, whose evidence substrate
/// is the verdict log — the idealized sweep keeps interaction-boundary
/// scenarios (overlapping ↔ disjoint) verdict-rich on both sides.
/// The realized sweep prunes the disjoint side's pair space empty
/// (its job); that production-path degradation is pinned in
/// `m4_pr4_banked` (both strategies) — see `fixture/pr4.rs`'s note.
fn run(doc: &ProfileDoc, prior: Option<&Evaluation<f64>>) -> Evaluation<f64> {
    let opts = EvalOptions {
        boolean_sweep: topo::SweepStrategy::Idealized,
        ..EvalOptions::default()
    };
    evaluate::<f64>(doc, prior, &CancelToken::new(), &opts, Tol::witness())
}

fn block(
    doc: ProfileDoc,
    (x0, x1): (f64, f64),
    (y0, y1): (f64, f64),
    z0: f64,
    dz: f64,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, p) = insert(
        doc,
        Node::Profile(desc(
            [0.0, 0.0, z0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![vec![(x0, y0), (x1, y0), (x1, y1), (x0, y1)]],
        )),
    );
    insert(
        doc,
        Node::Extrude {
            profile: p,
            distance: len(dz),
        },
    )
}

fn name1(kind: EntityKind, node: RecipeNodeId, seg: RoleSeg) -> StableName {
    StableName {
        kind,
        node,
        path: vec![seg],
    }
}

/// The sliding union: A fixed, B on a Transform knob, A ∪ B.
struct Slide {
    doc: ProfileDoc,
    a: RecipeNodeId,
    b0: RecipeNodeId,
    transform: RecipeNodeId,
    union: RecipeNodeId,
}

fn slide_union(tx: f64) -> Slide {
    let doc = ProfileDoc::empty_derived("m4_pr4_resolve", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b0) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, decl) = fixture::declare_x_offset_flush(doc, a, b0);
    let (doc, transform) = insert(
        doc,
        Node::Transform {
            input: b0,
            translation: [len(tx), len(0.0), len(0.0)],
            rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
            rotation_angle: ang(0.0),
        },
    );
    let (doc, union) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a,
            b: transform,
            declare: Some(decl),
        },
    );
    Slide {
        doc,
        a,
        b0,
        transform,
        union,
    }
}

fn slide_to(s: &Slide, tx: f64) -> ProfileDoc {
    let (doc, _) = step(
        s.doc.clone(),
        DocEdit::SetParam {
            node: s.transform,
            slot: SlotId::Translation(editor_core::Axis3::X),
            expr: len(tx),
        },
    );
    doc
}

/// A ranked rim-edge fragment name from the overlapping union's table
/// (`[FromA(RimEdge..), Fragment(OrderAlong{of: 2})]`).
fn ranked_rim_name(ev: &Evaluation<f64>, union: RecipeNodeId) -> StableName {
    ev.value(union)
        .unwrap()
        .name_table
        .iter()
        .find_map(|(n, e)| {
            let ranked = matches!(
                n.path.last(),
                Some(RoleSeg::Fragment(Qualifier::OrderAlong { of: 2, .. }))
            ) && matches!(n.path.first(), Some(RoleSeg::FromA(_)))
                && n.kind == EntityKind::Edge;
            (ranked && matches!(e, Entry::Unique(_))).then(|| n.clone())
        })
        .expect("overlapping union has ranked FromA rim fragments")
}

// ---- Resolved ----

#[test]
fn union_names_resolve_uniquely_and_pass_through_transforms() {
    let s = slide_union(0.5);
    let ev = run(&s.doc, None);
    let ctx = RunCtx {
        doc: &s.doc,
        eval: &ev,
    };
    // M4 PR 5 (N3 live): the declared flush caps GLUE — the A-cap
    // wrap retired into the Merged row, which resolves at the union;
    // the retired constituent name itself now fails typed with the
    // merged row among the OFFERS (N3's loud retirement, pinned in
    // the vanishing tests below).
    let cap = name1(EntityKind::Face, s.a, RoleSeg::Cap(CapEnd::Top));
    let wrapped = name1(
        EntityKind::Face,
        s.union,
        RoleSeg::FromA(Box::new(cap.clone())),
    );
    let cap_b = name1(EntityKind::Face, s.b0, RoleSeg::Cap(CapEnd::Top));
    let wrapped_b = name1(
        EntityKind::Face,
        s.union,
        RoleSeg::FromB(Box::new(cap_b.clone())),
    );
    let mut constituents = vec![wrapped.clone(), wrapped_b];
    constituents.sort_unstable();
    let merged = name1(EntityKind::Face, s.union, RoleSeg::Merged(constituents));
    match resolve(ctx, &merged) {
        Resolution::Resolved(r) => assert_eq!(r.node, s.union),
        other => panic!("expected Resolved, got {other:?}"),
    }
    match resolve(ctx, &wrapped) {
        Resolution::Failed(f) => assert!(
            f.offers.contains(&merged),
            "retired constituent must offer its merge: {f:?}"
        ),
        other => panic!("expected the retired constituent to fail typed, got {other:?}"),
    }
    // The operand-level cap name resolves too — at the EXTRUDE (first
    // carrying node in evaluation order; the transform pass-through
    // carries B's names identically).
    match resolve(ctx, &cap) {
        Resolution::Resolved(r) => assert_eq!(r.node, s.a),
        other => panic!("expected Resolved, got {other:?}"),
    }
}

// ---- Ambiguous: the direct N2 tie ----

#[test]
fn tied_name_resolves_ambiguous_with_the_tie_witness() {
    // PR 3's symmetric U cutter: two prong fragments of B's caps tie.
    let doc = ProfileDoc::empty_derived("m4_pr4_resolve", Tol::witness());
    let (doc, a) = block(doc, (0.0, 4.0), (0.0, 4.0), 0.0, 4.0);
    let (doc, p) = insert(
        doc,
        Node::Profile(desc(
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![vec![
                (2.0, 1.0),
                (6.0, 1.0),
                (6.0, 3.0),
                (2.0, 3.0),
                (2.0, 2.5),
                (5.0, 2.5),
                (5.0, 1.5),
                (2.0, 1.5),
            ]],
        )),
    );
    let (doc, b) = insert(
        doc,
        Node::Extrude {
            profile: p,
            distance: len(2.0),
        },
    );
    let (doc, sub) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Subtract,
            a,
            b,
            declare: None,
        },
    );
    let ev = run(&doc, None);
    let tied: StableName = ev
        .value(sub)
        .unwrap()
        .name_table
        .iter()
        .find_map(|(n, e)| matches!(e, Entry::Tied(_)).then(|| n.clone()))
        .expect("the U fixture ties");
    match resolve(
        RunCtx {
            doc: &doc,
            eval: &ev,
        },
        &tied,
    ) {
        Resolution::Failed(f) => {
            let ResolveError::Ambiguous {
                name,
                candidates,
                tie,
            } = &f.error
            else {
                panic!("expected Ambiguous, got {:?}", f.error);
            };
            assert_eq!(*name, tied);
            assert_eq!(candidates, &vec![tied.clone()]);
            assert_eq!(tie.node, sub);
            assert_eq!(tie.at, tied);
            assert_eq!(tie.width, 2);
            assert!(f.offers.is_empty(), "a tie offers nothing to auto-pick");
        }
        other => panic!("expected Failed(Ambiguous), got {other:?}"),
    }
}

// ---- Ambiguous: the order_along over-tie widening (hand-built
// table — the emitter's over-tie row is the widened BASE name; a
// reference to a RANKED name must widen to it, never mis-bind) ----

#[test]
fn ranked_reference_widens_to_the_tied_base_row() {
    // Real edge keys to populate the synthetic table with.
    let src = ProfileDoc::empty_derived("m4_pr4_resolve", Tol::witness());
    let (src, ext) = block(src, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let sev = run(&src, None);
    let mut edges = sev
        .value(ext)
        .unwrap()
        .name_table
        .iter()
        .filter_map(|(n, e)| {
            if n.kind != EntityKind::Edge {
                return None;
            }
            match e {
                Entry::Unique(r) => Some(*r),
                Entry::Tied(_) => None,
            }
        });
    let (e1, e2) = (edges.next().unwrap(), edges.next().unwrap());

    // A one-node doc whose table we hand-build.
    let mut doc = ProfileDoc::empty_derived("m4_pr4_resolve", Tol::witness());
    let (d, node) = insert(doc, Node::declare_rest(vec![]));
    doc = d;
    let base = StableName {
        kind: EntityKind::Edge,
        node,
        path: vec![RoleSeg::AxisEdge(editor_core::ProfileEdgeRef {
            loop_index: 0,
            segment: 0,
        })],
    };
    let mut table = NameTable::new();
    table.insert_tied(base.clone(), vec![e1, e2]).unwrap();
    let mut nodes = std::collections::BTreeMap::new();
    nodes.insert(
        node,
        editor_core::NodeResult::Ok(editor_core::NodeValue {
            payload: editor_core::ValuePayload::Declarations(vec![]),
            name_table: Arc::new(table),
            contacts: Arc::new(topo::ContactRecords::default()),
            verdicts: Arc::new(vec![]),
            witness: WitnessSlot::default(),
            content_key: ContentKey(0),
            naming_key: NamingKey(0),
        }),
    );
    let ev = Evaluation::<f64> {
        epoch: editor_core::Epoch::mint(),
        order: vec![node],
        nodes,
        outcome: EvalOutcome::Completed,
        recomputed: 1,
        reused: 0,
        part_evaluations: 0,
        appearance: editor_core::AppearanceResolution::default(),
    };
    let mut ranked = base.clone();
    ranked
        .path
        .push(RoleSeg::Fragment(Qualifier::OrderAlong { rank: 1, of: 2 }));
    match resolve(
        RunCtx {
            doc: &doc,
            eval: &ev,
        },
        &ranked,
    ) {
        Resolution::Failed(f) => {
            let ResolveError::Ambiguous {
                name,
                candidates,
                tie,
            } = &f.error
            else {
                panic!("expected widened Ambiguous, got {:?}", f.error);
            };
            assert_eq!(*name, ranked, "the error names the REFERENCE");
            assert_eq!(candidates, &vec![base.clone()], "candidates = widened base");
            assert_eq!(tie.at, base);
            assert_eq!(tie.width, 2);
        }
        other => panic!("expected Failed(Ambiguous), got {other:?}"),
    }
}

// ---- NodeGone ----

#[test]
fn deleting_a_named_node_strands_names_as_node_gone() {
    let doc = ProfileDoc::empty_derived("m4_pr4_resolve", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (2.0, 3.0), (0.0, 1.0), 0.0, 1.0);
    let cap_a = name1(EntityKind::Face, a, RoleSeg::Cap(CapEnd::Top));
    let cap_b = name1(EntityKind::Face, b, RoleSeg::Cap(CapEnd::Top));
    let (doc, _decl) = insert(doc, Node::declare_rest(vec![(cap_a, cap_b.clone())]));
    // b has no DAG dependents (Declare names are refs, not edges):
    // deletion is allowed and strands cap_b — N5's ratified dangling
    // semantics.
    let (doc2, _) = step(doc, DocEdit::DeleteNode { id: b });
    let ev = run(&doc2, None);
    match resolve(
        RunCtx {
            doc: &doc2,
            eval: &ev,
        },
        &cap_b,
    ) {
        Resolution::Failed(f) => {
            let ResolveError::NodeGone { name, edit } = &f.error else {
                panic!("expected NodeGone, got {:?}", f.error);
            };
            assert_eq!(*name, cap_b);
            assert_eq!(*edit, RecipeEditRef::NodeDeleted { node: b });
        }
        other => panic!("expected Failed(NodeGone), got {other:?}"),
    }
}

#[test]
fn never_minted_node_reports_foreign_not_deleted() {
    let doc = ProfileDoc::empty_derived("m4_pr4_resolve", Tol::witness());
    let (doc, _a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let ev = run(&doc, None);
    let foreign = name1(
        EntityKind::Face,
        RecipeNodeId(9999),
        RoleSeg::Cap(CapEnd::Top),
    );
    match resolve(
        RunCtx {
            doc: &doc,
            eval: &ev,
        },
        &foreign,
    ) {
        Resolution::Failed(f) => {
            let ResolveError::NodeGone { edit, .. } = &f.error else {
                panic!("expected NodeGone, got {:?}", f.error);
            };
            assert_eq!(
                *edit,
                RecipeEditRef::ForeignNode {
                    node: RecipeNodeId(9999)
                },
                "a never-minted id must not be blamed on a delete"
            );
        }
        other => panic!("expected Failed(NodeGone), got {other:?}"),
    }
}

// ---- Vanished: PredicateFlip diagnosis + tombstone + collapse offer ----

#[test]
fn flip_vanished_name_diagnoses_the_predicate_flip_with_tombstone() {
    let s = slide_union(0.5);
    let ev1 = run(&s.doc, None);
    let probe = ranked_rim_name(&ev1, s.union);
    let doc2 = slide_to(&s, 2.5); // disjoint: fragments vanish
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
        &probe,
    );
    let Resolution::Failed(f) = res else {
        panic!("expected Failed, got {res:?}");
    };
    let ResolveError::Vanished {
        name,
        diagnosis,
        last_good,
    } = &f.error
    else {
        panic!("expected Vanished, got {:?}", f.error);
    };
    assert_eq!(*name, probe);
    // The pillar's promise: a recorded predicate flip on the
    // derivation path, with real signs.
    let Diagnosis::PredicateFlip {
        predicate,
        from,
        to,
    } = diagnosis
    else {
        panic!("expected PredicateFlip, got {diagnosis:?}");
    };
    assert!(!predicate.is_empty());
    assert_ne!(from, to);
    // The tombstone: last-good entry at the union, edge kind, owning
    // body = the union's body name.
    let t = last_good.as_ref().expect("prior run resolved the name");
    assert_eq!(t.kind, EntityKind::Edge);
    assert_eq!(t.patch.node, s.union);
    assert_eq!(
        t.body,
        name1(EntityKind::Body, s.union, RoleSeg::OutputBody)
    );
    // The over-tie/collapse offer: the disjoint union still carries
    // the UNQUALIFIED base rim edge — offered for the explicit
    // Rebind, never auto-bound.
    let mut base = probe.clone();
    base.path.pop();
    assert!(
        f.offers.contains(&base),
        "expected the collapsed base as an offer: {:?}",
        f.offers
    );
}

// ---- Vanished: StructuralParam diagnosis ----

#[test]
fn pattern_count_shrink_diagnoses_structural_param() {
    let doc = ProfileDoc::empty_derived("m4_pr4_resolve", Tol::witness());
    let (doc, body) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, pattern) = insert(
        doc,
        Node::Pattern {
            input: body,
            count: editor_core::Expr::count(3),
            kind: editor_core::PatternKind::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: len(2.0),
            },
        },
    );
    let ev1 = run(&doc, None);
    let master_body = name1(EntityKind::Body, body, RoleSeg::OutputBody);
    let inst2 = name1(
        EntityKind::Body,
        pattern,
        RoleSeg::Instance {
            i: 2,
            of: Box::new(master_body),
        },
    );
    assert!(matches!(
        resolve(
            RunCtx {
                doc: &doc,
                eval: &ev1
            },
            &inst2
        ),
        Resolution::Resolved(_)
    ));
    let (doc2, _) = step(
        doc.clone(),
        DocEdit::SetStructuralParam {
            node: pattern,
            slot: SlotId::Count,
            expr: editor_core::Expr::count(2),
        },
    );
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
        &inst2,
    );
    let Resolution::Failed(f) = res else {
        panic!("expected Failed, got {res:?}");
    };
    let ResolveError::Vanished { diagnosis, .. } = &f.error else {
        panic!("expected Vanished, got {:?}", f.error);
    };
    assert_eq!(
        *diagnosis,
        Diagnosis::StructuralParam {
            node: pattern,
            param: SlotId::Count
        },
        "a count shrink is a structural-parameter diagnosis, not a flip"
    );
}

// ---- Vanished: Cascade through a vanished operand name ----

#[test]
fn instance_of_vanished_master_name_diagnoses_cascade() {
    let s = slide_union(0.5);
    // Pattern the union so its names wrap the union's.
    let (doc, pattern) = insert(
        s.doc.clone(),
        Node::Pattern {
            input: s.union,
            count: editor_core::Expr::count(2),
            kind: editor_core::PatternKind::Linear {
                direction: [scl(0.0), scl(1.0), scl(0.0)],
                spacing: len(5.0),
            },
        },
    );
    let ev1 = run(&doc, None);
    let master = ranked_rim_name(&ev1, s.union);
    let inst = name1(
        EntityKind::Edge,
        pattern,
        RoleSeg::Instance {
            i: 1,
            of: Box::new(master.clone()),
        },
    );
    assert!(matches!(
        resolve(
            RunCtx {
                doc: &doc,
                eval: &ev1
            },
            &inst
        ),
        Resolution::Resolved(_)
    ));
    // Slide B disjoint: the master fragment name vanishes, so the
    // instance name vanishes THROUGH it.
    let (doc2, _) = step(
        doc.clone(),
        DocEdit::SetParam {
            node: s.transform,
            slot: SlotId::Translation(editor_core::Axis3::X),
            expr: len(2.5),
        },
    );
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
        &inst,
    );
    let Resolution::Failed(f) = res else {
        panic!("expected Failed, got {res:?}");
    };
    let ResolveError::Vanished { diagnosis, .. } = &f.error else {
        panic!("expected Vanished, got {:?}", f.error);
    };
    assert_eq!(
        *diagnosis,
        Diagnosis::Cascade {
            through: master.clone()
        },
        "the upstream vanish carries the root cause"
    );
    // And the through-name's own resolution chains to the flip.
    let res_master = resolve_with_prior(
        RunCtx {
            doc: &doc2,
            eval: &ev2,
        },
        RunCtx {
            doc: &doc,
            eval: &ev1,
        },
        &master,
    );
    let Resolution::Failed(fm) = res_master else {
        panic!("expected Failed, got {res_master:?}");
    };
    assert!(matches!(
        fm.error,
        ResolveError::Vanished {
            diagnosis: Diagnosis::PredicateFlip { .. },
            ..
        }
    ));
}

// ---- Indeterminate: failed and poisoned targets ----

#[test]
fn failed_and_poisoned_targets_resolve_indeterminate_not_vanished() {
    let doc = ProfileDoc::empty_derived("m4_pr4_resolve", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, u) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a,
            b,
            declare: None,
        },
    );
    // Zero the A extrude's distance: A fails, the union poisons.
    let (doc2, _) = step(
        doc,
        DocEdit::SetParam {
            node: a,
            slot: SlotId::Distance,
            expr: len(0.0),
        },
    );
    let ev = run(&doc2, None);
    let ctx = RunCtx {
        doc: &doc2,
        eval: &ev,
    };
    let cap_a = name1(EntityKind::Face, a, RoleSeg::Cap(CapEnd::Top));
    assert_eq!(
        resolve(ctx, &cap_a),
        Resolution::Indeterminate(ResolveIndeterminate::TargetFailed { node: a })
    );
    let union_body = name1(EntityKind::Body, u, RoleSeg::OutputBody);
    assert_eq!(
        resolve(ctx, &union_body),
        Resolution::Indeterminate(ResolveIndeterminate::TargetPoisoned { through: a })
    );
}

// ---- The rebind suggestion ladder (the D9 operand→final gap) ----

#[test]
fn rebind_suggestions_offer_wrapping_derivations() {
    let s = slide_union(0.5);
    let ev = run(&s.doc, None);
    let cap = name1(EntityKind::Face, s.a, RoleSeg::Cap(CapEnd::Top));
    let suggestions = rebind_suggestions(&ev, &cap);
    // M4 PR 5 (N3 live): the FromA(cap) wrap retired into the Merged
    // row — the suggestion ladder offers the MERGED name (whose
    // constituents embed the cap's wrap); nothing is followed
    // automatically — these are Rebind candidates only.
    let wrapped = name1(EntityKind::Face, s.union, RoleSeg::FromA(Box::new(cap)));
    assert!(
        suggestions.iter().any(|n| matches!(
            n.path.first(),
            Some(RoleSeg::Merged(cs)) if cs.contains(&wrapped)
        )),
        "expected the Merged row embedding the FromA wrap among suggestions: {suggestions:?}"
    );
}

// ---- R6: name-level edit-time validation (banked from PR 3) ----

#[test]
fn apply_with_names_refuses_unresolvable_declare_names_and_keeps_the_carveout() {
    let doc = ProfileDoc::empty_derived("m4_pr4_resolve", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (2.0, 3.0), (0.0, 1.0), 0.0, 1.0);
    let ev = run(&doc, None);
    let cap_a = name1(EntityKind::Face, a, RoleSeg::Cap(CapEnd::Top));
    let cap_b = name1(EntityKind::Face, b, RoleSeg::Cap(CapEnd::Top));
    // A real pair: accepted.
    assert!(
        apply_with_names(
            &doc,
            &DocEdit::InsertNode {
                node: Node::declare_rest(vec![(cap_a.clone(), cap_b.clone())])
            },
            &ev,
            Tol::witness(),
        )
        .is_ok()
    );
    // A typo role on an EVALUATED node: refused at the edit door.
    let bogus = name1(
        EntityKind::Face,
        a,
        RoleSeg::Lateral(editor_core::ProfileEdgeRef {
            loop_index: 7,
            segment: 7,
        }),
    );
    let err = apply_with_names(
        &doc,
        &DocEdit::InsertNode {
            node: Node::declare_rest(vec![(cap_a.clone(), bogus.clone())]),
        },
        &ev,
        Tol::witness(),
    )
    .unwrap_err();
    assert_eq!(
        err,
        editor_core::EditError::NameUnresolvedInEvaluation { name: bogus }
    );
    // The forward-reference carve-out: a name on a node the supplied
    // evaluation has NOT seen passes through (resolution happens at
    // evaluation).
    let (doc2, c) = block(doc.clone(), (4.0, 5.0), (0.0, 1.0), 0.0, 1.0);
    let cap_c = name1(EntityKind::Face, c, RoleSeg::Cap(CapEnd::Top));
    assert!(
        apply_with_names(
            &doc2,
            &DocEdit::InsertNode {
                node: Node::declare_rest(vec![(cap_a, cap_c)])
            },
            &ev,
            Tol::witness(),
        )
        .is_ok(),
        "forward references defer to evaluation-time resolution"
    );
}

/// The same door, same obligation, for the OTHER payloads that carry a
/// name. A fillet's selection is checkable exactly when a `Declare`
/// pair is — the minting node evaluated `Ok` — so a typo role on an
/// evaluated node is refused here rather than surviving to the fillet's
/// own resolution.
#[test]
fn apply_with_names_checks_a_fillet_selection_under_the_same_rule() {
    let doc = ProfileDoc::empty_derived("m4_pr4_resolve_fillet_door", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let ev = run(&doc, None);
    let rim = name1(
        EntityKind::Edge,
        a,
        RoleSeg::RimEdge(
            CapEnd::Top,
            editor_core::ProfileEdgeRef {
                loop_index: 0,
                segment: 0,
            },
        ),
    );
    assert!(
        apply_with_names(
            &doc,
            &DocEdit::InsertNode {
                node: Node::fillet(a, len(0.1), vec![rim.clone()])
            },
            &ev,
            Tol::witness(),
        )
        .is_ok(),
        "a selection the tables carry passes"
    );
    let bogus = name1(
        EntityKind::Edge,
        a,
        RoleSeg::RimEdge(
            CapEnd::Top,
            editor_core::ProfileEdgeRef {
                loop_index: 7,
                segment: 7,
            },
        ),
    );
    let err = apply_with_names(
        &doc,
        &DocEdit::InsertNode {
            node: Node::fillet(a, len(0.1), vec![bogus.clone()]),
        },
        &ev,
        Tol::witness(),
    )
    .unwrap_err();
    assert_eq!(
        err,
        editor_core::EditError::NameUnresolvedInEvaluation { name: bogus }
    );
}

// ---- Review Finding 2: suggestions are structural wraps only, ----
// ---- kind-filtered (adopted reviewer probe, inverted to a pin) ----

/// Whether a walk counts `SideOf` discriminator PARTNERS as
/// occurrences of a name.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Partners {
    /// Count partner positions: any mention at all.
    Include,
    /// Structural embedding only — a derivation OF the name.
    Skip,
}

/// True iff `needle` occurs anywhere under `hay`, counting
/// discriminator partners iff `partners` says so.
///
/// STRICTLY under: `hay` itself is not an occurrence of `needle`, so
/// `occurs(n, n, _)` is false unless a name contains itself, which no
/// name does. The one caller subtracts the two answers, where the
/// self-case cancels either way.
///
/// This is the test's OWN reading of the role vocabulary, spelled out
/// rather than borrowed from `resolve`'s walker: an oracle that asks
/// the code under test what the answer is pins nothing.
///
/// The match is EXHAUSTIVE, and that is what keeps it an oracle. A
/// role segment added to the vocabulary must be classified here —
/// embedding, discrimination, or neither — before this suite
/// compiles. Under a catch-all a new name-carrying segment reads as
/// "no occurrence", so [`only_sideof_mention`] would report NO
/// PHANTOM for a phantom of exactly the new shape, and the row below
/// would pass while the property it names had failed.
fn occurs(hay: &StableName, needle: &StableName, partners: Partners) -> bool {
    let under = |n: &StableName| n == needle || occurs(n, needle, partners);
    hay.path.iter().any(|seg| match seg {
        // One embedded operand name: the entity derives from it.
        RoleSeg::FromA(x)
        | RoleSeg::FromB(x)
        | RoleSeg::SectionEdge { face: x, .. }
        | RoleSeg::SplitFragment { parent: x, .. }
        | RoleSeg::CrossingVertex { edge: x, .. }
        | RoleSeg::OnToolVertex { of: x, .. }
        | RoleSeg::Instance { of: x, .. }
        | RoleSeg::FromTarget(x)
        | RoleSeg::BlendFace(x)
        | RoleSeg::CornerFace(x)
        | RoleSeg::BandTrim { edge: x, .. }
        | RoleSeg::BandFoot(x)
        | RoleSeg::BandCross(x)
        | RoleSeg::BandCut(x)
        | RoleSeg::BandSlit(x) => under(x),
        // Two.
        RoleSeg::Seam { a: x, b: y }
        | RoleSeg::TrimEdge {
            edge: x,
            support: y,
        }
        | RoleSeg::FootVertex {
            vertex: x,
            support: y,
        }
        | RoleSeg::CornerArc { vertex: x, edge: y } => under(x) || under(y),
        // A set.
        RoleSeg::Merged(v) | RoleSeg::BandFace(v) => v.iter().any(under),
        // ANOTHER document's id space: a local name and a part-local
        // name that print alike are different names, so a walk that
        // descended here would report occurrences that are not.
        RoleSeg::InPart { .. } => false,
        // Discrimination, not derivation: the fragment is classified
        // AGAINST these, not built from them.
        RoleSeg::Fragment(Qualifier::SideOf(v)) => {
            partners == Partners::Include && v.iter().any(|(p, _)| under(p))
        }
        RoleSeg::Fragment(Qualifier::OrderAlong { .. }) => false,
        // Segments that embed no name.
        RoleSeg::OutputBody
        | RoleSeg::Cap(_)
        | RoleSeg::Lateral(_)
        | RoleSeg::RimEdge(..)
        | RoleSeg::LateralEdge(_)
        | RoleSeg::CapVertex(..)
        | RoleSeg::Band(_)
        | RoleSeg::BandRim(_)
        | RoleSeg::BandRimPi(_)
        | RoleSeg::BandPi(_)
        | RoleSeg::Meridian(..)
        | RoleSeg::MeridianVertex(..)
        | RoleSeg::RevolveCap(_)
        | RoleSeg::Pole(_)
        | RoleSeg::AxisEdge(_)
        | RoleSeg::SplitBody(_)
        | RoleSeg::SectionFace { .. } => false,
    })
}

/// True iff `needle` occurs in `hay`'s path ONLY inside SideOf
/// discriminator vectors (never as a structural embedding) — the
/// reviewer's phantom detector.
fn only_sideof_mention(hay: &StableName, needle: &StableName) -> bool {
    !occurs(hay, needle, Partners::Skip) && occurs(hay, needle, Partners::Include)
}

/// The detector answers about a name reached through a segment its
/// FIRST vocabulary knew nothing about.
///
/// This row is the detector's own pin, and it is here because the
/// detector shipped for a year reading three groups of segments and
/// sweeping the rest into a catch-all. Everything the fillet emitter
/// mints — this row's `BlendFace` among them — was in that catch-all,
/// so a phantom wrapped in one read as no mention at all and the
/// suggestion row above passed by not looking.
///
/// The shape is the one that row cares about: a name that mentions
/// `needle` ONLY as a `SideOf` partner, one derivation step below the
/// surface. It is a phantom, and saying so requires descending
/// through the blend segment — which is why a detector blind to that
/// segment reports the opposite.
#[test]
fn the_phantom_detector_sees_through_the_whole_vocabulary() {
    let needle = fixture::fname(RecipeNodeId(1), RoleSeg::Cap(CapEnd::Top));
    let partner_only = StableName {
        kind: EntityKind::Face,
        node: RecipeNodeId(2),
        path: vec![RoleSeg::Fragment(Qualifier::SideOf(vec![(
            needle.clone(),
            editor_core::SideVerdict::Positive,
        )]))],
    };
    let blended = fixture::fname(
        RecipeNodeId(3),
        RoleSeg::BlendFace(Box::new(partner_only.clone())),
    );

    assert!(
        only_sideof_mention(&partner_only, &needle),
        "a bare SideOf partner mention is the phantom shape itself"
    );
    assert!(
        only_sideof_mention(&blended, &needle),
        "a phantom stays a phantom under a blend segment — a detector \
         that cannot read the segment calls this NO MENTION and lets \
         the suggestion row through"
    );
    // The same segment, carrying the needle structurally: a real
    // derivation, and the detector must not call it a phantom.
    let derived = fixture::fname(
        RecipeNodeId(3),
        RoleSeg::BlendFace(Box::new(needle.clone())),
    );
    assert!(
        !only_sideof_mention(&derived, &needle),
        "a blend OF the name is a derivation, not a phantom"
    );
}

#[test]
fn suggestions_never_offer_sideof_partner_phantoms_and_are_kind_filtered() {
    // The reviewer's band-cut rig: the subtract mints SideOf-qualified
    // cap fragments whose partners are BARE operand names of the
    // cutter's walls — exactly the shape a user paints. Suggestions
    // for a painted partner must be derivations WRAPPING it, never
    // fragments of the OTHER body that merely recorded a side-of
    // verdict against its plane, and never a kind Rebind refuses.
    let doc = ProfileDoc::empty_derived("m4_pr4_resolve", Tol::witness());
    let (doc, _a) = block(doc, (0.0, 4.0), (0.0, 4.0), 0.0, 1.0);
    let (doc, bp) = insert(
        doc,
        Node::Profile(desc(
            [0.0, 0.0, -0.5],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![vec![
                (-2.5, 1.0),
                (2.0, 1.0),
                (4.5, 0.8),
                (4.5, 0.9),
                (2.0, 1.1),
                (-2.5, 1.1),
            ]],
        )),
    );
    let (doc, band) = insert(
        doc,
        Node::Extrude {
            profile: bp,
            distance: len(2.0),
        },
    );
    let (doc, tr) = insert(
        doc,
        Node::Transform {
            input: band,
            translation: [len(0.0), len(0.0), len(0.0)],
            rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
            rotation_angle: ang(0.0),
        },
    );
    let (doc, sub) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Subtract,
            a: _a,
            b: tr,
            declare: None,
        },
    );
    let ev = run(&doc, None);
    // A partner name recorded in some fragment's SideOf vector.
    let partner: StableName = ev
        .value(sub)
        .expect("subtract evaluates")
        .name_table
        .iter()
        .find_map(|(n, e)| {
            if !matches!(e, Entry::Unique(_) | Entry::Tied(_)) {
                return None;
            }
            n.path.iter().find_map(|seg| match seg {
                RoleSeg::Fragment(Qualifier::SideOf(v)) => v.first().map(|(p, _)| p.clone()),
                _ => None,
            })
        })
        .expect("band cut mints SideOf-qualified fragments");
    let suggestions = rebind_suggestions(&ev, &partner);
    assert!(
        !suggestions.is_empty(),
        "the true structural wraps are still offered"
    );
    for s in &suggestions {
        assert_eq!(
            s.kind, partner.kind,
            "cross-kind suggestion (Rebind refuses these): {s:?}"
        );
        assert!(
            !only_sideof_mention(s, &partner),
            "SIDEOF-ONLY phantom offered as a suggestion: {s:?}"
        );
    }
}

// ---- Review Finding 3: Diagnosis::RecipeEdit constructed for a ----
// ---- real recipe edit; the single-run no-prior Vanished path ----

#[test]
fn repointed_input_diagnoses_recipe_edit_on_path() {
    // Two geometrically IDENTICAL operands b and c: re-pointing the
    // boolean's second input from b to c changes NO verdict (the
    // computed geometry is bit-identical) and NO structural
    // parameter — the only honest evidence is the recipe edit at the
    // boolean node, and it is on the vanished name's path.
    let build = |use_c: bool| {
        let doc = ProfileDoc::empty_derived("m4_pr4_resolve", Tol::witness());
        let (doc, a) = block(doc, (0.0, 2.0), (0.0, 2.0), 0.0, 1.0);
        // General position (no coplanar planes with A): B pierces A's
        // slab, strictly inside in y, poking out above and below.
        let (doc, b) = block(doc, (1.0, 3.0), (0.5, 1.5), -0.5, 2.0);
        let (doc, c) = block(doc, (1.0, 3.0), (0.5, 1.5), -0.5, 2.0);
        let (doc, bl) = insert(
            doc,
            Node::Boolean {
                op: BooleanOp::Union,
                a,
                b: if use_c { c } else { b },
                declare: None,
            },
        );
        (doc, b, c, bl)
    };
    let (doc1, b, _c, bl) = build(false);
    let ev1 = run(&doc1, None);
    // The union carries B's top cap as FromB(cap_b).
    let cap_b = name1(EntityKind::Face, b, RoleSeg::Cap(CapEnd::Top));
    let target = StableName {
        kind: EntityKind::Face,
        node: bl,
        path: vec![RoleSeg::FromB(Box::new(cap_b.clone()))],
    };
    assert!(
        matches!(
            resolve(
                RunCtx {
                    doc: &doc1,
                    eval: &ev1
                },
                &target
            ),
            Resolution::Resolved(_)
        ),
        "the union derives FromB(cap of b) before the re-point"
    );
    let (doc2, _, c, _) = build(true);
    // #95 disposition 2 LANDED (M4 PR 5): the memo-TRANSFERRED run
    // now honestly re-derives the boolean's naming half — the
    // recursive naming key includes input node ids, so the b→c
    // re-point misses the memo even though the twins are
    // bit-identical. Pinned WITH memo transfer.
    let ev2 = run(&doc2, Some(&ev1));
    let res = resolve_with_prior(
        RunCtx {
            doc: &doc2,
            eval: &ev2,
        },
        RunCtx {
            doc: &doc1,
            eval: &ev1,
        },
        &target,
    );
    let Resolution::Failed(f) = res else {
        panic!("expected Failed, got {res:?}");
    };
    let ResolveError::Vanished {
        diagnosis,
        last_good,
        ..
    } = &f.error
    else {
        panic!("expected Vanished, got {:?}", f.error);
    };
    assert_eq!(
        *diagnosis,
        Diagnosis::RecipeEdit {
            edit: RecipeEditRef::NodeChanged { node: bl }
        },
        "a re-pointed input is a recipe edit on the path — no flip, \
         no structural param to blame"
    );
    assert!(last_good.is_some(), "the prior run resolved the name");
    // The positive half of the #95 pin: the re-derived table carries
    // FromB(cap of C) — the value the recipe actually denotes.
    let cap_c = name1(EntityKind::Face, c, RoleSeg::Cap(CapEnd::Top));
    let target_c = StableName {
        kind: EntityKind::Face,
        node: bl,
        path: vec![RoleSeg::FromB(Box::new(cap_c))],
    };
    assert!(
        matches!(
            resolve(
                RunCtx {
                    doc: &doc2,
                    eval: &ev2
                },
                &target_c
            ),
            Resolution::Resolved(_)
        ),
        "the memo-transferred run must derive FromB(cap of c)"
    );
}

/// The #95 GRANDPARENT pin (the reason disposition 2's key is
/// RECURSIVE): re-point X's INPUT to a bit-identical twin two hops
/// above N — X's content key is unchanged at N's doorstep (N's direct
/// input is X either way), so a one-level context check would reuse
/// N's stale names; the recursive naming key composes X's change
/// through and N re-derives, embedding the twin's re-derived names.
#[test]
fn grandparent_repoint_rederives_the_grandchild_names() {
    use editor_core::{NodeResult, ValuePayload};
    let build = |use_c: bool| {
        let doc = ProfileDoc::empty_derived("m4_pr4_resolve", Tol::witness());
        let (doc, b) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
        let (doc, c) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
        let (doc, x) = insert(
            doc,
            Node::Transform {
                input: if use_c { c } else { b },
                translation: [len(0.25), len(0.0), len(0.0)],
                rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
                rotation_angle: ang(0.0),
            },
        );
        let (doc, n) = insert(
            doc,
            Node::Transform {
                input: x,
                translation: [len(0.0), len(0.25), len(0.0)],
                rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
                rotation_angle: ang(0.0),
            },
        );
        (doc, b, c, n)
    };
    let (doc1, b, _c, n) = build(false);
    let ev1 = run(&doc1, None);
    let (doc2, _b, c, _n) = build(true);
    let ev2 = run(&doc2, Some(&ev1));
    // The grandchild's table must speak C's names now (transform
    // pass-through: rows keep the MINTING node = the twin extrude).
    let table = match ev2.nodes.get(&n) {
        Some(NodeResult::Ok(v)) => {
            assert!(
                matches!(v.payload, ValuePayload::Body(_)),
                "grandchild is a body"
            );
            &v.name_table
        }
        other => panic!("grandchild must evaluate, got {other:?}"),
    };
    assert!(
        table.iter().any(|(name, _)| name.node == c),
        "the memo-transferred grandchild table must embed the twin's names"
    );
    assert!(
        table.iter().all(|(name, _)| name.node != b),
        "no stale name may survive the grandparent re-point"
    );
}

#[test]
fn single_run_vanished_falls_back_to_cause_not_in_evidence() {
    // No prior run, no cascade, no recorded qualifier delta: the
    // documented total fallback — the recorded reference disagrees
    // with the recipe as it stands, the cause not in evidence.
    let doc = ProfileDoc::empty_derived("m4_pr4_resolve", Tol::witness());
    let (doc, body) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, pattern) = insert(
        doc,
        Node::Pattern {
            input: body,
            count: editor_core::Expr::count(2),
            kind: editor_core::PatternKind::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: len(2.0),
            },
        },
    );
    let ev = run(&doc, None);
    // Instance 5 was never minted by this 2-count pattern.
    let master_body = name1(EntityKind::Body, body, RoleSeg::OutputBody);
    let inst5 = name1(
        EntityKind::Body,
        pattern,
        RoleSeg::Instance {
            i: 5,
            of: Box::new(master_body),
        },
    );
    let res = resolve(
        RunCtx {
            doc: &doc,
            eval: &ev,
        },
        &inst5,
    );
    let Resolution::Failed(f) = res else {
        panic!("expected Failed, got {res:?}");
    };
    let ResolveError::Vanished {
        diagnosis,
        last_good,
        ..
    } = &f.error
    else {
        panic!("expected Vanished, got {:?}", f.error);
    };
    assert_eq!(
        *diagnosis,
        Diagnosis::RecipeEdit {
            edit: RecipeEditRef::NodeChanged { node: pattern }
        }
    );
    assert!(last_good.is_none(), "no prior run, no tombstone");
    assert!(f.offers.is_empty());
}

// ---- Review Finding 1 ruling: the qualifier-delta rung ----

/// A body-kind fragment name `[FromA(f), Fragment(SideOf([(p, v)]))]`
/// at `node` — the hand-built shape for the qualifier-delta pins.
fn sideof_frag(
    node: RecipeNodeId,
    f: &StableName,
    p: &StableName,
    v: editor_core::SideVerdict,
) -> StableName {
    StableName {
        kind: EntityKind::Body,
        node,
        path: vec![
            RoleSeg::FromA(Box::new(f.clone())),
            RoleSeg::Fragment(Qualifier::SideOf(vec![(p.clone(), v)])),
        ],
    }
}

/// One-node hand-built evaluation whose table is `t` (the over-tie
/// pin's construction, reused).
fn one_node_eval(node: RecipeNodeId, t: NameTable) -> Evaluation<f64> {
    let mut nodes = std::collections::BTreeMap::new();
    nodes.insert(
        node,
        editor_core::NodeResult::Ok(editor_core::NodeValue {
            payload: editor_core::ValuePayload::Declarations(vec![]),
            name_table: Arc::new(t),
            contacts: Arc::new(topo::ContactRecords::default()),
            verdicts: Arc::new(vec![]),
            witness: WitnessSlot::default(),
            content_key: ContentKey(0),
            naming_key: NamingKey(0),
        }),
    );
    Evaluation::<f64> {
        epoch: editor_core::Epoch::mint(),
        order: vec![node],
        nodes,
        outcome: EvalOutcome::Completed,
        recomputed: 1,
        reused: 0,
        part_evaluations: 0,
        appearance: editor_core::AppearanceResolution::default(),
    }
}

fn body_ent(i: u32) -> editor_core::EntityRef {
    editor_core::EntityRef {
        body: i,
        key: editor_core::EntityKey::Body,
    }
}

#[test]
fn qualifier_delta_yields_predicate_flip_without_any_flip_set_evidence() {
    use geom_core::Sign;
    // The re-qualification is recorded IN the names: the old name
    // carries (P, Negative) where the new table's same-shape sibling
    // carries (P, Positive). Both runs have EMPTY verdict logs and
    // the doc is UNCHANGED — the diff-engine and doc-diff lanes have
    // nothing (the population-cancel shape), yet the diagnosis is an
    // honest PredicateFlip derived from recorded data.
    let (doc, n) = insert(
        ProfileDoc::empty_derived("m4_pr4_resolve", Tol::witness()),
        Node::declare_rest(vec![]),
    );
    let (doc, m) = insert(doc, Node::declare_rest(vec![]));
    let f = name1(EntityKind::Body, n, RoleSeg::OutputBody);
    let p = name1(EntityKind::Body, m, RoleSeg::OutputBody);
    let old_name = sideof_frag(n, &f, &p, editor_core::SideVerdict::Negative);
    let new_name = sideof_frag(n, &f, &p, editor_core::SideVerdict::Positive);

    let mut t_prior = NameTable::new();
    t_prior.insert(old_name.clone(), body_ent(0)).unwrap();
    t_prior.insert(f.clone(), body_ent(1)).unwrap();
    t_prior.insert(p.clone(), body_ent(2)).unwrap();
    let mut t_new = NameTable::new();
    t_new.insert(new_name.clone(), body_ent(0)).unwrap();
    t_new.insert(f.clone(), body_ent(1)).unwrap();
    t_new.insert(p.clone(), body_ent(2)).unwrap();
    let prior_ev = one_node_eval(n, t_prior);
    let new_ev = one_node_eval(n, t_new);

    let expect_flip = |res: Resolution| {
        let Resolution::Failed(fail) = res else {
            panic!("expected Failed, got {res:?}");
        };
        let ResolveError::Vanished {
            diagnosis,
            last_good,
            ..
        } = fail.error
        else {
            panic!("expected Vanished, got {:?}", fail.error);
        };
        assert_eq!(
            diagnosis,
            Diagnosis::PredicateFlip {
                predicate: "name_frag_side_of",
                from: Sign::Negative,
                to: Sign::Positive,
            },
            "the recorded qualifier delta is the honest flip"
        );
        last_good
    };

    // Single-run: the rung is the FIRST evidence (no prior at all).
    let last_good = expect_flip(resolve(
        RunCtx {
            doc: &doc,
            eval: &new_ev,
        },
        &old_name,
    ));
    assert!(last_good.is_none());

    // With-prior, empty FlipSet (both logs empty), unchanged doc:
    // every earlier lane is silent; the rung still fires, and the
    // tombstone rides from the prior run.
    let last_good = expect_flip(resolve_with_prior(
        RunCtx {
            doc: &doc,
            eval: &new_ev,
        },
        RunCtx {
            doc: &doc,
            eval: &prior_ev,
        },
        &old_name,
    ));
    let t = last_good.expect("the prior run resolved the name");
    assert_eq!(t.patch.node, n);
    assert_eq!(t.patch.entity, body_ent(0));

    // The reported boundary: an aggregate (Mixed) verdict on either
    // side has no single-Sign reading — the rung must NOT fire, and
    // the fallback names the site without claiming an edit.
    let old_mixed = sideof_frag(n, &f, &p, editor_core::SideVerdict::Mixed);
    let res = resolve(
        RunCtx {
            doc: &doc,
            eval: &new_ev,
        },
        &old_mixed,
    );
    let Resolution::Failed(fail) = res else {
        panic!("expected Failed, got {res:?}");
    };
    let ResolveError::Vanished { diagnosis, .. } = fail.error else {
        panic!("expected Vanished, got {:?}", fail.error);
    };
    assert_eq!(
        diagnosis,
        Diagnosis::RecipeEdit {
            edit: RecipeEditRef::NodeChanged { node: n }
        },
        "Mixed→Positive is not a pure-sign delta; fabricating a Sign \
         would be dishonest"
    );
}
