//! M4 PR 4 spec D1/D3: the N5 resolution ladder end to end —
//! Resolved / Ambiguous (direct tie and the order_along over-tie
//! widening) / NodeGone / Vanished with the verdict-diff diagnosis
//! (PredicateFlip, StructuralParam, Cascade) + tombstones / typed
//! Indeterminate — plus N3 offers, the rebind suggestion ladder, and
//! the R6 name-level edit-time validation door.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod fixture;

use std::sync::Arc;

use editor_core::eval::WitnessSlot;
use editor_core::{
    BooleanOp, CancelToken, CapEnd, ContentKey, Diagnosis, DocEdit, EntityKind, Entry, EvalOptions,
    EvalOutcome, Evaluation, NameTable, Node, ProfileDoc, Qualifier, RecipeEditRef, RecipeNodeId,
    Resolution, ResolveError, ResolveIndeterminate, RoleSeg, RunCtx, SlotId, StableName,
    apply_with_names, evaluate, rebind_suggestions, resolve, resolve_with_prior,
};
use fixture::{ang, desc, insert, len, scl, step};

fn run(doc: &ProfileDoc, prior: Option<&Evaluation<f64>>) -> Evaluation<f64> {
    evaluate::<f64>(doc, prior, &CancelToken::new(), &EvalOptions::default())
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
    transform: RecipeNodeId,
    union: RecipeNodeId,
}

fn slide_union(tx: f64) -> Slide {
    let doc = ProfileDoc::empty();
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b0) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
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
            declare: None,
        },
    );
    Slide {
        doc,
        a,
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
    // The A-cap wrap resolves at the union.
    let cap = name1(EntityKind::Face, s.a, RoleSeg::Cap(CapEnd::Top));
    let wrapped = name1(
        EntityKind::Face,
        s.union,
        RoleSeg::FromA(Box::new(cap.clone())),
    );
    match resolve(ctx, &wrapped) {
        Resolution::Resolved(r) => assert_eq!(r.node, s.union),
        other => panic!("expected Resolved, got {other:?}"),
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
    let doc = ProfileDoc::empty();
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
    let src = ProfileDoc::empty();
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
    let mut doc = ProfileDoc::empty();
    let (d, node) = insert(doc, Node::Declare { pairs: vec![] });
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
            verdicts: Arc::new(vec![]),
            witness: WitnessSlot::default(),
            content_key: ContentKey(0),
        }),
    );
    let ev = Evaluation::<f64> {
        epoch: editor_core::Epoch::mint(),
        order: vec![node],
        nodes,
        outcome: EvalOutcome::Completed,
        recomputed: 1,
        reused: 0,
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
    let doc = ProfileDoc::empty();
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (2.0, 3.0), (0.0, 1.0), 0.0, 1.0);
    let cap_a = name1(EntityKind::Face, a, RoleSeg::Cap(CapEnd::Top));
    let cap_b = name1(EntityKind::Face, b, RoleSeg::Cap(CapEnd::Top));
    let (doc, _decl) = insert(
        doc,
        Node::Declare {
            pairs: vec![(cap_a, cap_b.clone())],
        },
    );
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
    let doc = ProfileDoc::empty();
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
    let doc = ProfileDoc::empty();
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
    let doc = ProfileDoc::empty();
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
    // The union's FromA(cap) wrap is offered (with the cap's
    // fragment/rim derivations that embed it); nothing is followed
    // automatically — these are Rebind candidates only.
    let wrapped = name1(EntityKind::Face, s.union, RoleSeg::FromA(Box::new(cap)));
    assert!(
        suggestions.contains(&wrapped),
        "expected the FromA wrap among suggestions: {suggestions:?}"
    );
}

// ---- R6: name-level edit-time validation (banked from PR 3) ----

#[test]
fn apply_with_names_refuses_unresolvable_declare_names_and_keeps_the_carveout() {
    let doc = ProfileDoc::empty();
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
                node: Node::Declare {
                    pairs: vec![(cap_a.clone(), cap_b.clone())]
                }
            },
            &ev,
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
            node: Node::Declare {
                pairs: vec![(cap_a.clone(), bogus.clone())],
            },
        },
        &ev,
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
                node: Node::Declare {
                    pairs: vec![(cap_a, cap_c)]
                }
            },
            &ev, // stale: c not evaluated here
        )
        .is_ok(),
        "forward references defer to evaluation-time resolution"
    );
}

// ---- Review Finding 2: suggestions are structural wraps only, ----
// ---- kind-filtered (adopted reviewer probe, inverted to a pin) ----

/// True iff `needle` occurs in `hay`'s path ONLY inside SideOf
/// discriminator vectors (never as a structural embedding) — the
/// reviewer's phantom detector.
fn only_sideof_mention(hay: &StableName, needle: &StableName) -> bool {
    fn embeds_structurally(n: &StableName, needle: &StableName) -> bool {
        if n == needle {
            return true;
        }
        for seg in &n.path {
            let hit = match seg {
                RoleSeg::FromA(x)
                | RoleSeg::FromB(x)
                | RoleSeg::SectionEdge { face: x, .. }
                | RoleSeg::SplitFragment { parent: x, .. }
                | RoleSeg::CrossingVertex { edge: x, .. }
                | RoleSeg::OnToolVertex { of: x, .. }
                | RoleSeg::Instance { of: x, .. } => embeds_structurally(x, needle),
                RoleSeg::Seam { a, b } => {
                    embeds_structurally(a, needle) || embeds_structurally(b, needle)
                }
                RoleSeg::Merged(v) => v.iter().any(|x| embeds_structurally(x, needle)),
                // SideOf partners deliberately NOT counted here.
                _ => false,
            };
            if hit {
                return true;
            }
        }
        false
    }
    fn mentions_anywhere(n: &StableName, needle: &StableName) -> bool {
        if n == needle {
            return true;
        }
        for seg in &n.path {
            let hit = match seg {
                RoleSeg::FromA(x)
                | RoleSeg::FromB(x)
                | RoleSeg::SectionEdge { face: x, .. }
                | RoleSeg::SplitFragment { parent: x, .. }
                | RoleSeg::CrossingVertex { edge: x, .. }
                | RoleSeg::OnToolVertex { of: x, .. }
                | RoleSeg::Instance { of: x, .. } => mentions_anywhere(x, needle),
                RoleSeg::Seam { a, b } => {
                    mentions_anywhere(a, needle) || mentions_anywhere(b, needle)
                }
                RoleSeg::Merged(v) => v.iter().any(|x| mentions_anywhere(x, needle)),
                RoleSeg::Fragment(Qualifier::SideOf(v)) => {
                    v.iter().any(|(p, _)| mentions_anywhere(p, needle))
                }
                _ => false,
            };
            if hit {
                return true;
            }
        }
        false
    }
    !embeds_structurally(hay, needle) && mentions_anywhere(hay, needle)
}

#[test]
fn suggestions_never_offer_sideof_partner_phantoms_and_are_kind_filtered() {
    // The reviewer's band-cut rig: the subtract mints SideOf-qualified
    // cap fragments whose partners are BARE operand names of the
    // cutter's walls — exactly the shape a user paints. Suggestions
    // for a painted partner must be derivations WRAPPING it, never
    // fragments of the OTHER body that merely recorded a side-of
    // verdict against its plane, and never a kind Rebind refuses.
    let doc = ProfileDoc::empty();
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
        let doc = ProfileDoc::empty();
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
        (doc, b, bl)
    };
    let (doc1, b, bl) = build(false);
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
    let (doc2, _, _) = build(true);
    // FRESH evaluation deliberately (REPORTED observation, fix-pass):
    // a memo-TRANSFERRED run reuses the boolean's value by content
    // key, and b/c are bit-identical twins — so the reused value
    // still carries the OLD `FromB(cap of b)` name table and the
    // name keeps resolving against a recipe that now feeds c. Name
    // tables embed minting node ids (N1) while content keys exclude
    // them (D8): the names half of a memoized value is not a pure
    // function of its key. Flagged for an orchestrator ruling; out
    // of scope for this PR's resolution machinery.
    let ev2 = run(&doc2, None);
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
}

#[test]
fn single_run_vanished_falls_back_to_cause_not_in_evidence() {
    // No prior run, no cascade, no recorded qualifier delta: the
    // documented total fallback — the recorded reference disagrees
    // with the recipe as it stands, the cause not in evidence.
    let doc = ProfileDoc::empty();
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
            verdicts: Arc::new(vec![]),
            witness: WitnessSlot::default(),
            content_key: ContentKey(0),
        }),
    );
    Evaluation::<f64> {
        epoch: editor_core::Epoch::mint(),
        order: vec![node],
        nodes,
        outcome: EvalOutcome::Completed,
        recomputed: 1,
        reused: 0,
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
    let (doc, n) = insert(ProfileDoc::empty(), Node::Declare { pairs: vec![] });
    let (doc, m) = insert(doc, Node::Declare { pairs: vec![] });
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
