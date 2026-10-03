//! M4 PR 4 spec D1/D3: the N5 resolution ladder end to end —
//! Resolved / Ambiguous (direct tie and the order_along over-tie
//! widening) / NodeGone / Vanished with the verdict-diff diagnosis
//! (PredicateFlip, StructuralParam, Cascade) + tombstones / typed
//! Indeterminate — plus N3 offers, the rebind suggestion ladder, and
//! the R6 name-level edit-time validation door.
//!
//! SWEEP-STRATEGY NOTE (Ev's 2026-07-29 ruling): this file's pins
//! are about diff/resolve engine behavior GIVEN verdicts, so its
//! evaluator deliberately runs the idealized (verdict-rich) sweep;
//! the production-path degradation is pinned in `m4_pr4_banked`
//! (both strategies side by side) and in the re-pinned `m4_pr4_ci`
//! golden.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use std::sync::Arc;

use editor_core::NodeStanding;
use editor_core::eval::WitnessSlot;
use editor_core::{
    BooleanOp, CancelToken, CapEnd, ContentKey, Diagnosis, DocEdit, EntityKind, Entry, EvalOptions,
    EvalOutcome, Evaluation, NameTable, NamingKey, Node, ProfileDoc, Qualifier, RecipeEditRef,
    RecipeNodeId, Resolution, ResolveError, ResolveIndeterminate, RoleSeg, RunCtx, SitedRef,
    SlotId, StableName, apply_with_names, evaluate, rebind_suggestions, resolve,
    resolve_with_prior,
};
use fixture::{ang, insert, len, minted, on_frame, scl, step};
use geom_core::Tol;

/// Idealized (brute-force) boolean sweep since M5 PR 8: this file
/// pins the DIFF/RESOLVE engine's semantics, whose evidence substrate
/// is the verdict log — the idealized sweep keeps interaction-boundary
/// scenarios (overlapping ↔ disjoint) verdict-rich on both sides.
/// The realized sweep prunes the disjoint side's pair space empty
/// (its job); that production-path degradation is pinned in
/// `m4_pr4_banked` (both strategies) — see `fixture/pr4.rs`'s note.
fn run(doc: &editor_core::ProfileDoc, prior: Option<&Evaluation<f64>>) -> Evaluation<f64> {
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
    let (doc, transform) = insert(
        doc,
        Node::transform(
            b0,
            editor_core::Step::Rigid {
                translation: [len(tx), len(0.0), len(0.0)],
                axis: [scl(0.0), scl(0.0), scl(1.0)],
                angle: ang(0.0),
            },
        ),
    );
    // The B side is read at the TRANSFORM — the boolean's operand —
    // and named in `b0`'s vocabulary, which the transform carries
    // verbatim (N1).
    let decl = fixture::declare_x_offset_flush_at(&doc, (a, a), (transform, b0));
    let (doc, union) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a,
            b: transform,
            declare: decl,
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

/// A rim-edge piece name from the overlapping union's table
/// (`[FromA(RimEdge..), Fragment(Ends)]`).
fn rim_piece_name(ev: &Evaluation<f64>, union: RecipeNodeId) -> StableName {
    ev.value(union)
        .unwrap()
        .name_table
        .iter()
        .find_map(|(n, e)| {
            let piece = matches!(n.path.last(), Some(RoleSeg::Fragment(Qualifier::Ends(_))))
                && matches!(n.path.first(), Some(RoleSeg::FromA(_)))
                && n.kind == EntityKind::Edge;
            (piece && matches!(e, Entry::Unique(_))).then(|| n.clone())
        })
        .expect("overlapping union has FromA rim pieces")
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
    // N3: the declared flush caps GLUE — the A-cap
    // wrap retired into the Merged row, which resolves at the union;
    // the retired constituent name itself now fails typed with the
    // merged row among the OFFERS (N3's loud retirement, pinned in
    // the vanishing tests below).
    let cap = minted(EntityKind::Face, s.a, RoleSeg::Cap(CapEnd::End));
    let wrapped = minted(
        EntityKind::Face,
        s.union,
        RoleSeg::FromA(cap.clone().into()),
    );
    let cap_b = minted(EntityKind::Face, s.b0, RoleSeg::Cap(CapEnd::End));
    let wrapped_b = minted(
        EntityKind::Face,
        s.union,
        RoleSeg::FromB(cap_b.clone().into()),
    );
    let mut constituents = vec![wrapped.clone(), wrapped_b];
    constituents.sort_unstable();
    let merged = minted(EntityKind::Face, s.union, RoleSeg::Merged(constituents));
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
    let (doc, p) = on_frame(
        doc,
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
            declare: Vec::new(),
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
    let (doc, node) = insert(
        ProfileDoc::empty_derived("m4_pr4_resolve", Tol::witness()),
        leaf(),
    );
    let base = StableName {
        kind: EntityKind::Edge,
        node,
        path: vec![RoleSeg::AxisEdge(crate::fixture::no_piece())],
    };
    let mut table = NameTable::new();
    table.insert_tied(base.clone(), vec![e1, e2]).unwrap();
    let mut nodes = std::collections::BTreeMap::new();
    nodes.insert(
        node,
        editor_core::NodeResult::Ok(editor_core::NodeValue {
            payload: editor_core::ValuePayload::Gauge,
            name_table: Arc::new(table),
            fragment_groups: Arc::default(),
            contacts: Arc::new(topo::ContactRecords::default()),
            carried: Arc::new(editor_core::CarriedDeclarations::default()),
            gathered: 1,
            verdicts: Arc::new(vec![]),
            escalations: Arc::new(vec![]),
            placement: None,
            witness: WitnessSlot::default(),
            content_key: ContentKey(0),
            naming_key: NamingKey(0),
        }),
    );
    let ev = Evaluation::<f64> {
        epoch: editor_core::Epoch::mint(),
        unplaced: Default::default(),
        unplaced_below: Default::default(),
        document: doc.id(),
        prior_refused: None,
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
    let (doc, _) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (2.0, 3.0), (0.0, 1.0), 0.0, 1.0);
    let cap_b = minted(EntityKind::Face, b, RoleSeg::Cap(CapEnd::End));
    // b has no DAG dependents: deletion is allowed and strands cap_b —
    // N5's ratified dangling semantics.
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
    let foreign = minted(
        EntityKind::Face,
        RecipeNodeId(9999),
        RoleSeg::Cap(CapEnd::End),
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
    let probe = rim_piece_name(&ev1, s.union);
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
        minted(EntityKind::Body, s.union, RoleSeg::OutputBody)
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
    let master_body = minted(EntityKind::Body, body, RoleSeg::OutputBody);
    let inst2 = minted(
        EntityKind::Body,
        pattern,
        RoleSeg::Instance {
            i: 2,
            of: master_body.into(),
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
    let master = rim_piece_name(&ev1, s.union);
    let inst = minted(
        EntityKind::Edge,
        pattern,
        RoleSeg::Instance {
            i: 1,
            of: master.clone().into(),
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
            declare: Vec::new(),
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
    let cap_a = minted(EntityKind::Face, a, RoleSeg::Cap(CapEnd::End));
    assert_eq!(
        resolve(ctx, &cap_a),
        Resolution::Indeterminate(ResolveIndeterminate {
            standing: NodeStanding::Failed { node: a }
        })
    );
    let union_body = minted(EntityKind::Body, u, RoleSeg::OutputBody);
    assert_eq!(
        resolve(ctx, &union_body),
        Resolution::Indeterminate(ResolveIndeterminate {
            standing: NodeStanding::Poisoned {
                node: u,
                through: a
            }
        })
    );
}

// ---- The rebind suggestion ladder (the D9 operand→final gap) ----

#[test]
fn rebind_suggestions_offer_wrapping_derivations() {
    let s = slide_union(0.5);
    let ev = run(&s.doc, None);
    let cap = minted(EntityKind::Face, s.a, RoleSeg::Cap(CapEnd::End));
    let suggestions = rebind_suggestions(&ev, &cap);
    // M4 PR 5 (N3 live): the FromA(cap) wrap retired into the Merged
    // row — the suggestion ladder offers the MERGED name (whose
    // constituents embed the cap's wrap); nothing is followed
    // automatically — these are Rebind candidates only.
    let wrapped = minted(EntityKind::Face, s.union, RoleSeg::FromA(cap.into()));
    assert!(
        suggestions.iter().any(|n| matches!(
            n.path.first(),
            Some(RoleSeg::Merged(cs)) if cs.contains(&wrapped)
        )),
        "expected the Merged row embedding the FromA wrap among suggestions: {suggestions:?}"
    );
}

// ---- R6: name-level edit-time validation (banked from PR 3) ----

/// **`apply_with_names` holds a declared name to the evaluation it is
/// given where it can, and defers where it cannot**: a real pair is
/// accepted, a typo role on an evaluated node refuses
/// `NameUnresolvedInEvaluation`, a backward name the evaluation has
/// not seen passes to evaluation-time resolution, and a forward name
/// is the door's `DeclaredNameNotUpstream`, not the carve-out's.
#[test]
fn apply_with_names_refuses_unresolvable_declare_names_and_keeps_the_carveout() {
    let doc = ProfileDoc::empty_derived("m4_pr4_resolve", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (2.0, 3.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, u) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a,
            b,
            declare: Vec::new(),
        },
    );
    let ev = run(&doc, None);
    let cap_a = minted(EntityKind::Face, a, RoleSeg::Cap(CapEnd::End));
    let cap_b = minted(EntityKind::Face, b, RoleSeg::Cap(CapEnd::End));
    // A real pair: accepted.
    assert!(
        apply_with_names(
            &doc,
            &DocEdit::SetDeclare {
                node: u,
                pairs: editor_core::declare_rest(vec![(
                    SitedRef::at_mint(cap_a.clone()),
                    SitedRef::at_mint(cap_b),
                )]),
            },
            &ev,
            Tol::witness(),
            &editor_core::RefusingReach
        )
        .is_ok()
    );
    // A typo role on an EVALUATED node: refused at the edit door.
    let bogus = minted(
        EntityKind::Face,
        a,
        RoleSeg::Lateral(crate::fixture::no_piece().into()),
    );
    let err = apply_with_names(
        &doc,
        &DocEdit::SetDeclare {
            node: u,
            pairs: editor_core::declare_rest(vec![(
                SitedRef::at_mint(cap_a.clone()),
                SitedRef::at_mint(bogus.clone()),
            )]),
        },
        &ev,
        Tol::witness(),
        &editor_core::RefusingReach,
    )
    .unwrap_err();
    assert_eq!(
        err,
        editor_core::EditError::NameUnresolvedInEvaluation {
            name: doc.spoken_name(&bogus)
        }
    );
    // The carve-out: a name on a node the supplied evaluation has NOT
    // seen passes through, and is resolved at evaluation. Here that
    // evaluation predates `b`, so a role `b` does not have passes,
    // where the same typo on the evaluated `a` refused above.
    let bogus_b = minted(
        EntityKind::Face,
        b,
        RoleSeg::Lateral(crate::fixture::no_piece_of(&doc).into()),
    );
    let (early, _) = block(
        ProfileDoc::empty_derived("m4_pr4_resolve", Tol::witness()),
        (0.0, 1.0),
        (0.0, 1.0),
        0.0,
        1.0,
    );
    let ev_early = run(&early, None);
    let deferred = apply_with_names(
        &doc,
        &DocEdit::SetDeclare {
            node: u,
            pairs: editor_core::declare_rest(vec![(
                SitedRef::at_mint(cap_a.clone()),
                SitedRef::at_mint(bogus_b),
            )]),
        },
        &ev_early,
        Tol::witness(),
        &editor_core::RefusingReach,
    )
    .map(|_| ());
    assert!(
        deferred.is_ok(),
        "a name the evaluation has not seen defers to evaluation-time resolution: {deferred:?}"
    );
    // A FORWARD reference is not the carve-out's: a name minted after
    // the declaring node is refused at the door whatever the
    // evaluation has seen, since none of the node's operands can hold
    // it.
    let (doc2, c) = block(doc.clone(), (4.0, 5.0), (0.0, 1.0), 0.0, 1.0);
    let cap_c = minted(EntityKind::Face, c, RoleSeg::Cap(CapEnd::End));
    assert_eq!(
        apply_with_names(
            &doc2,
            &DocEdit::SetDeclare {
                node: u,
                pairs: editor_core::declare_rest(vec![(
                    SitedRef::at_mint(cap_a),
                    SitedRef::new(b, cap_c.clone()),
                )]),
            },
            &ev,
            Tol::witness(),
            &editor_core::RefusingReach
        )
        .unwrap_err(),
        editor_core::EditError::DeclaredNameNotUpstream {
            node: doc2.spoken(u),
            name: doc2.spoken_name(&cap_c),
        },
        "a forward reference is refused at the door"
    );
}

/// The same door, same obligation, for the OTHER payloads that carry a
/// name. A fillet's selection is checkable exactly when a declared
/// pair is — the minting node evaluated `Ok` — so a typo role on an
/// evaluated node is refused here rather than surviving to the fillet's
/// own resolution.
#[test]
fn apply_with_names_checks_a_fillet_selection_under_the_same_rule() {
    let doc = ProfileDoc::empty_derived("m4_pr4_resolve_fillet_door", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let ev = run(&doc, None);
    let rim = minted(
        EntityKind::Edge,
        a,
        RoleSeg::RimEdge(CapEnd::End, crate::fixture::piece(&doc, a, 0, 0)),
    );
    assert!(
        apply_with_names(
            &doc,
            &DocEdit::InsertNode {
                node: Box::new(Node::fillet(a, len(0.1), vec![rim.clone()]))
            },
            &ev,
            Tol::witness(),
            &editor_core::RefusingReach
        )
        .is_ok(),
        "a selection the tables carry passes"
    );
    let bogus = minted(
        EntityKind::Edge,
        a,
        RoleSeg::RimEdge(CapEnd::End, crate::fixture::no_piece()),
    );
    let err = apply_with_names(
        &doc,
        &DocEdit::InsertNode {
            node: Box::new(Node::fillet(a, len(0.1), vec![bogus.clone()])),
        },
        &ev,
        Tol::witness(),
        &editor_core::RefusingReach,
    )
    .unwrap_err();
    assert_eq!(
        err,
        editor_core::EditError::NameUnresolvedInEvaluation {
            name: doc.spoken_name(&bogus)
        }
    );
}

// ---- Review Finding 2: suggestions are structural wraps only, ----
// ---- kind-filtered (adopted reviewer probe, inverted to a pin) ----

/// Whether a walk counts discriminator PARTNERS (a piece's walls) as
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
/// "no occurrence", so [`only_wall_mention`] would report NO
/// PHANTOM for a phantom of exactly the new shape, and the row below
/// would pass while the property it names had failed.
fn occurs(hay: &StableName, needle: &StableName, partners: Partners) -> bool {
    let under = |n: &StableName| n == needle || occurs(n, needle, partners);
    hay.path.iter().any(|seg| match seg {
        // One embedded operand name: the entity derives from it.
        RoleSeg::FromA(x)
        | RoleSeg::FromB(x)
        | RoleSeg::FromMember { of: x, .. }
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
        | RoleSeg::BandCut(x)
        | RoleSeg::Inner(x)
        | RoleSeg::Rim(x)
        | RoleSeg::HoleRim { of: x, .. } => under(x),
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
        | RoleSeg::EndArc { vertex: x, edge: y } => under(x) || under(y),
        // A set.
        RoleSeg::Merged(v) | RoleSeg::BandFace(v) => v.iter().any(under),
        // A source edge (derivation) and the band that crossed or slit
        // it (a discriminator, like a piece's wall).
        RoleSeg::BandCross { edge, band } | RoleSeg::BandSlit { edge, band } => {
            under(edge) || (partners == Partners::Include && band.iter().any(under))
        }
        // ANOTHER document's id space: a local name and a part-local
        // name that print alike are different names, so a walk that
        // descended here would report occurrences that are not.
        RoleSeg::InPart { .. } => false,
        // Discrimination, not derivation: the fragment is classified
        // AGAINST these, not built from them.
        RoleSeg::Fragment(Qualifier::Borders(v) | Qualifier::Keeps(v) | Qualifier::Ends(v)) => {
            partners == Partners::Include && v.iter().any(under)
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
        | RoleSeg::SectionFace { .. }
        | RoleSeg::LoftWall(_)
        | RoleSeg::LoftSeam(_) => false,
    })
}

/// True iff `needle` occurs in `hay`'s path ONLY as a `Borders` wall (never as a structural embedding) — the
/// reviewer's phantom detector.
fn only_wall_mention(hay: &StableName, needle: &StableName) -> bool {
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
/// `needle` ONLY as a `Borders` wall, one derivation step below the
/// surface. It is a phantom, and saying so requires descending
/// through the blend segment — which is why a detector blind to that
/// segment reports the opposite.
#[test]
fn the_phantom_detector_sees_through_the_whole_vocabulary() {
    let needle = fixture::fname(RecipeNodeId(1), RoleSeg::Cap(CapEnd::End));
    let partner_only = StableName {
        kind: EntityKind::Face,
        node: RecipeNodeId(2),
        path: vec![RoleSeg::Fragment(Qualifier::Borders(vec![needle.clone()]))],
    };
    let blended = fixture::fname(
        RecipeNodeId(3),
        RoleSeg::BlendFace(partner_only.clone().into()),
    );

    assert!(
        only_wall_mention(&partner_only, &needle),
        "a bare wall mention is the phantom shape itself"
    );
    assert!(
        only_wall_mention(&blended, &needle),
        "a phantom stays a phantom under a blend segment — a detector \
         that cannot read the segment calls this NO MENTION and lets \
         the suggestion row through"
    );
    // The same segment, carrying the needle structurally: a real
    // derivation, and the detector must not call it a phantom.
    let derived = fixture::fname(RecipeNodeId(3), RoleSeg::BlendFace(needle.clone().into()));
    assert!(
        !only_wall_mention(&derived, &needle),
        "a blend OF the name is a derivation, not a phantom"
    );
}

#[test]
fn suggestions_never_offer_wall_phantoms_and_are_kind_filtered() {
    // The reviewer's band-cut rig: the subtract mints `Borders`-qualified
    // cap pieces whose walls are BARE operand names of the cutter's
    // walls — exactly the shape a user paints. Suggestions for a
    // painted wall must be derivations WRAPPING it, never pieces of the
    // OTHER body that merely border it, and never a kind Rebind
    // refuses.
    let doc = ProfileDoc::empty_derived("m4_pr4_resolve", Tol::witness());
    let (doc, _a) = block(doc, (0.0, 4.0), (0.0, 4.0), 0.0, 1.0);
    let (doc, bp) = on_frame(
        doc,
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
        Node::transform(
            band,
            editor_core::Step::Rigid {
                translation: [len(0.0), len(0.0), len(0.0)],
                axis: [scl(0.0), scl(0.0), scl(1.0)],
                angle: ang(0.0),
            },
        ),
    );
    let (doc, sub) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Subtract,
            a: _a,
            b: tr,
            declare: Vec::new(),
        },
    );
    let ev = run(&doc, None);
    // A wall recorded in some piece's `Borders` set.
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
                RoleSeg::Fragment(Qualifier::Borders(v)) => v.first().cloned(),
                _ => None,
            })
        })
        .expect("band cut mints Borders-qualified pieces");
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
            !only_wall_mention(s, &partner),
            "WALL-ONLY phantom offered as a suggestion: {s:?}"
        );
    }
}

// ---- Review Finding 3: Diagnosis::RecipeEdit constructed for a ----
// ---- real recipe edit; the single-run no-prior Vanished path ----

#[test]
fn repointed_input_diagnoses_recipe_edit_on_path() {
    // Two geometrically IDENTICAL operands b and c: re-pointing the
    // union's second member from b to c (`SetMembers`, the door that
    // re-points a node's inputs in place) changes NO verdict (the
    // computed geometry is bit-identical) and NO structural parameter
    // — the only honest evidence is the recipe edit at the union
    // node, and it is on the vanished name's path.
    let doc = ProfileDoc::empty_derived("m4_pr4_resolve", Tol::witness());
    let (doc, a) = block(doc, (0.0, 2.0), (0.0, 2.0), 0.0, 1.0);
    // General position (no coplanar planes with A): B pierces A's
    // slab, strictly inside in y, poking out above and below.
    let (doc, b) = block(doc, (1.0, 3.0), (0.5, 1.5), -0.5, 2.0);
    let (doc, c) = block(doc, (1.0, 3.0), (0.5, 1.5), -0.5, 2.0);
    let (doc1, bl) = insert(
        doc,
        Node::Union {
            members: vec![a, b],
            declare: Vec::new(),
        },
    );
    let ev1 = run(&doc1, None);
    // The union carries B's end cap as its member's.
    let member_cap = |m: RecipeNodeId| StableName {
        kind: EntityKind::Face,
        node: bl,
        path: vec![RoleSeg::FromMember {
            member: m,
            of: minted(EntityKind::Face, m, RoleSeg::Cap(CapEnd::End)).into(),
        }],
    };
    let target = member_cap(b);
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
        "the union derives b's cap before the re-point"
    );
    let (doc2, _) = step(
        doc1.clone(),
        DocEdit::SetMembers {
            node: bl,
            members: vec![a, c],
        },
    );
    // #95 disposition 2 LANDED (M4 PR 5): the memo-TRANSFERRED run
    // honestly re-derives the union's naming half — the recursive
    // naming key includes input node ids, so the b→c re-point misses
    // the memo even though the twins are bit-identical. Pinned WITH
    // memo transfer.
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
    // c's cap — the value the recipe actually denotes.
    assert!(
        matches!(
            resolve(
                RunCtx {
                    doc: &doc2,
                    eval: &ev2
                },
                &member_cap(c)
            ),
            Resolution::Resolved(_)
        ),
        "the memo-transferred run must derive c's cap"
    );
}

/// The #95 GRANDPARENT pin (the reason disposition 2's key is
/// RECURSIVE): re-point X's INPUT to a bit-identical twin two hops
/// above N — X's content key is unchanged at N's doorstep (N's direct
/// input is X either way), so a one-level context check would reuse
/// N's stale names; the recursive naming key composes X's change
/// through and N re-derives, embedding the twin's re-derived names.
/// X is a union, whose members `SetMembers` re-points in place, so X
/// and N keep their ids across the re-point.
#[test]
fn grandparent_repoint_rederives_the_grandchild_names() {
    use editor_core::{NodeResult, ValuePayload};
    let doc = ProfileDoc::empty_derived("m4_pr4_resolve", Tol::witness());
    let (doc, b) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, c) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, d) = block(doc, (5.0, 6.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, x) = insert(
        doc,
        Node::Union {
            members: vec![b, d],
            declare: Vec::new(),
        },
    );
    let (doc1, n) = insert(
        doc,
        Node::transform(
            x,
            editor_core::Step::Rigid {
                translation: [len(0.0), len(0.25), len(0.0)],
                axis: [scl(0.0), scl(0.0), scl(1.0)],
                angle: ang(0.0),
            },
        ),
    );
    let ev1 = run(&doc1, None);
    let (doc2, _) = step(
        doc1,
        DocEdit::SetMembers {
            node: x,
            members: vec![c, d],
        },
    );
    let ev2 = run(&doc2, Some(&ev1));
    // The grandchild's table must speak C's names now (transform
    // pass-through: rows keep the MINTING node = the union, whose
    // member edge is the twin).
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
    let mentions = |m: RecipeNodeId| {
        table
            .iter()
            .any(|(name, _)| editor_core::derivation_nodes(name).contains(&m))
    };
    assert!(
        mentions(c),
        "the memo-transferred grandchild table must embed the twin's names"
    );
    assert!(
        !mentions(b),
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
    let master_body = minted(EntityKind::Body, body, RoleSeg::OutputBody);
    let inst5 = minted(
        EntityKind::Body,
        pattern,
        RoleSeg::Instance {
            i: 5,
            of: master_body.into(),
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

// ---- Review Finding 1 ruling: the qualifier-delta rung, for a ----
// ---- face piece the border delta ----

/// A body-kind piece name `[FromA(f), Fragment(Borders(walls))]` at
/// `node` — the hand-built shape for the border-delta pins.
fn piece(node: RecipeNodeId, f: &StableName, walls: &[&StableName]) -> StableName {
    StableName {
        kind: EntityKind::Body,
        node,
        path: vec![
            RoleSeg::FromA(f.clone().into()),
            RoleSeg::Fragment(Qualifier::Borders(
                walls.iter().map(|&w| w.clone()).collect(),
            )),
        ],
    }
}

/// A node with no inputs and no evaluated body, for a doc whose
/// evaluation is hand-built.
fn leaf() -> Node<editor_core::ProfileProgram> {
    Node::gauge(
        None,
        editor_core::Placement::literal(&editor_core::Frame::translation([0.0; 3])),
    )
}

/// One-node hand-built evaluation whose table is `t` (the over-tie
/// pin's construction, reused).
fn one_node_eval(
    document: editor_core::DocumentId,
    node: RecipeNodeId,
    t: NameTable,
) -> Evaluation<f64> {
    let mut nodes = std::collections::BTreeMap::new();
    nodes.insert(
        node,
        editor_core::NodeResult::Ok(editor_core::NodeValue {
            payload: editor_core::ValuePayload::Gauge,
            name_table: Arc::new(t),
            fragment_groups: Arc::default(),
            contacts: Arc::new(topo::ContactRecords::default()),
            carried: Arc::new(editor_core::CarriedDeclarations::default()),
            gathered: 1,
            verdicts: Arc::new(vec![]),
            escalations: Arc::new(vec![]),
            placement: None,
            witness: WitnessSlot::default(),
            content_key: ContentKey(0),
            naming_key: NamingKey(0),
        }),
    );
    Evaluation::<f64> {
        epoch: editor_core::Epoch::mint(),
        unplaced: Default::default(),
        unplaced_below: Default::default(),
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

fn body_ent(i: u32) -> editor_core::EntityRef {
    editor_core::EntityRef {
        body: i,
        key: editor_core::EntityKey::Body,
    }
}

#[test]
fn border_delta_reads_the_walls_off_the_names_without_any_flip_set_evidence() {
    // The change is recorded IN the names: the old piece borders P, and
    // the piece that still borders P now borders S as well. Both runs
    // have EMPTY verdict logs and the doc is UNCHANGED — the
    // diff-engine and doc-diff lanes have nothing (the
    // population-cancel shape), yet the diagnosis names the wall that
    // moved.
    let (mut doc, n) = insert(
        ProfileDoc::empty_derived("m4_pr4_resolve", Tol::witness()),
        leaf(),
    );
    let mut walls = Vec::new();
    for _ in 0..7 {
        let (d, at) = insert(doc, leaf());
        doc = d;
        walls.push(minted(EntityKind::Body, at, RoleSeg::OutputBody));
    }
    let [p, q, r, s, t, r2, r3] = <[StableName; 7]>::try_from(walls).unwrap();
    let f = minted(EntityKind::Body, n, RoleSeg::OutputBody);
    let old_name = piece(n, &f, &[&p]);
    let table = |pieces: &[StableName]| {
        let mut tb = NameTable::new();
        for (i, name) in pieces.iter().enumerate() {
            tb.insert(name.clone(), body_ent(i as u32)).unwrap();
        }
        tb.insert(f.clone(), body_ent(10)).unwrap();
        tb
    };
    let eval = |pieces: &[StableName]| one_node_eval(doc.id(), n, table(pieces));
    let diagnosis_of = |res: Resolution| {
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
        (diagnosis, last_good)
    };
    let single = |ev: &Evaluation<f64>| {
        diagnosis_of(resolve(
            RunCtx {
                doc: &doc,
                eval: ev,
            },
            &old_name,
        ))
    };
    let with_prior = |ev: &Evaluation<f64>, prior: &Evaluation<f64>| {
        diagnosis_of(resolve_with_prior(
            RunCtx {
                doc: &doc,
                eval: ev,
            },
            RunCtx {
                doc: &doc,
                eval: prior,
            },
            &old_name,
        ))
    };
    let fallback = Diagnosis::RecipeEdit {
        edit: RecipeEditRef::NodeChanged { node: n },
    };

    // The untouched sibling {Q, R} borders none of the vanished walls,
    // so it is no counterpart; the piece bordering P and S is.
    let prior_ev = eval(&[old_name.clone(), piece(n, &f, &[&q, &r])]);
    let new_ev = eval(&[piece(n, &f, &[&p, &s]), piece(n, &f, &[&q, &r])]);
    let delta = Diagnosis::BorderDelta {
        node: n,
        gone: vec![],
        new: vec![s.clone()],
    };
    // Single-run: the rung is the FIRST evidence (no prior at all).
    let (d, last_good) = single(&new_ev);
    assert_eq!(d, delta);
    assert!(last_good.is_none());
    // With-prior, empty FlipSet (both logs empty), unchanged doc: every
    // earlier lane is silent; the rung still fires, and the tombstone
    // rides from the prior run.
    let (d, last_good) = with_prior(&new_ev, &prior_ev);
    assert_eq!(d, delta);
    let tomb = last_good.expect("the prior run resolved the name");
    assert_eq!(tomb.patch.node, n);
    assert_eq!(tomb.patch.entity, body_ent(0));

    // Two pieces equally near are no one counterpart: the rung does not
    // pick, and the fallback names the site without claiming an edit.
    let (d, _) = single(&eval(&[piece(n, &f, &[&p, &s]), piece(n, &f, &[&p, &t])]));
    assert_eq!(
        d, fallback,
        "two equally near pieces: choosing one would be a guess"
    );

    // A piece that borders none of the vanished walls is another piece,
    // however near its set is: no counterpart, no answer.
    let (d, _) = single(&eval(&[piece(n, &f, &[&q])]));
    assert_eq!(
        d, fallback,
        "a piece sharing no wall is not the vanished one changed"
    );

    // The review's probe: {P} vanishes beside a sibling {P, R, R2, R3}
    // that the last-good run already published, and a new piece {Q}.
    // {Q} shares no wall, and the sibling is untouched, so with the
    // prior run the rung declines; it never answers `new: [Q]`.
    let sibling = piece(n, &f, &[&p, &r, &r2, &r3]);
    let prior_ev = eval(&[old_name.clone(), sibling.clone()]);
    let new_ev = eval(&[sibling, piece(n, &f, &[&q])]);
    let (d, _) = with_prior(&new_ev, &prior_ev);
    assert_eq!(d, fallback, "an untouched sibling is not the counterpart");
    let (d, _) = single(&new_ev);
    assert!(
        !matches!(&d, Diagnosis::BorderDelta { new, .. } if new.contains(&q)),
        "a piece sharing no wall is never the counterpart: {d:?}"
    );
}
