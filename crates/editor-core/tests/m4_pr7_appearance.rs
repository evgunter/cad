//! M4 PR 7 (appearance half): per-face/body appearance attributes
//! keyed by StableName — DocEdit apply validation, survival across
//! recompute (the attribute rides the name, not the topology), loud
//! typed N3/N5 loss semantics on retire/vanish, and the
//! renderer-facing resolved output.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod fixture;

use editor_core::{
    AppearanceLossCause, Attr, AttrKind, BooleanOp, CancelToken, CapEnd, Dimension, DocEdit,
    DocParam, EditError, EntityKey, EntityKind, EvalOptions, Evaluation, Expr, Node, ParamName,
    PatternKind, ProfileDoc, RecipeNodeId, Rgba8, RoleSeg, StableName, evaluate,
};
use fixture::{DEPTH, desc, die, insert, len, scl, square, step};
use geom_core::Tol;

fn run(doc: &ProfileDoc) -> Evaluation<f64> {
    evaluate::<f64>(doc, None, &CancelToken::new(), &EvalOptions::default(), Tol::witness())
}

fn rerun(doc: &ProfileDoc, prior: &Evaluation<f64>) -> Evaluation<f64> {
    evaluate::<f64>(
        doc,
        Some(prior),
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

fn name1(kind: EntityKind, node: RecipeNodeId, seg: RoleSeg) -> StableName {
    StableName {
        kind,
        node,
        path: vec![seg],
    }
}

fn red() -> Attr {
    Attr::Color(Rgba8::opaque(200, 30, 30))
}

fn set(doc: ProfileDoc, name: StableName, attr: Attr) -> ProfileDoc {
    step(doc, DocEdit::SetAppearance { name, attr }).0
}

/// A rectangular block: profile on the plane z = `z0`, extruded `dz`.
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

// ---- DocEdit apply validation ----

#[test]
fn set_appearance_validates_and_applies_purely() {
    let (doc, ext) = block(
        ProfileDoc::empty_derived("m4_pr7_appearance", Tol::witness()),
        (0.0, 1.0),
        (0.0, 1.0),
        0.0,
        1.0,
    );
    let cap = name1(EntityKind::Face, ext, RoleSeg::Cap(CapEnd::Top));

    let applied = doc
        .apply(&DocEdit::SetAppearance {
            name: cap.clone(),
            attr: red(),
        }, Tol::witness())
        .unwrap();
    // Non-structural, nothing minted; the input document untouched.
    assert!(!applied.record.structural);
    assert_eq!(applied.record.minted, None);
    assert!(doc.appearance().is_empty(), "apply must be pure");
    assert_eq!(
        applied
            .doc
            .appearance_of(&cap)
            .unwrap()
            .attrs
            .get(&AttrKind::Color),
        Some(&red())
    );

    // Edge/vertex names: typed refusal (v1 scope is face/body).
    let edge = name1(
        EntityKind::Edge,
        ext,
        RoleSeg::RimEdge(
            CapEnd::Top,
            editor_core::ProfileEdgeRef {
                loop_index: 0,
                segment: 0,
            },
        ),
    );
    assert_eq!(
        doc.apply(&DocEdit::SetAppearance {
            name: edge.clone(),
            attr: red(),
        }, Tol::witness())
        .unwrap_err(),
        EditError::AppearanceWrongKind { name: edge }
    );

    // A never-existed node id: typed refusal at the edit door.
    let bogus = name1(
        EntityKind::Face,
        RecipeNodeId(999),
        RoleSeg::Cap(CapEnd::Top),
    );
    assert_eq!(
        doc.apply(&DocEdit::SetAppearance {
            name: bogus.clone(),
            attr: red(),
        }, Tol::witness())
        .unwrap_err(),
        EditError::AppearanceNamesMissingNode { name: bogus }
    );
}

#[test]
fn multi_attribute_per_entity_and_clear_semantics() {
    let (doc, ext) = block(
        ProfileDoc::empty_derived("m4_pr7_appearance", Tol::witness()),
        (0.0, 1.0),
        (0.0, 1.0),
        0.0,
        1.0,
    );
    let body = name1(EntityKind::Body, ext, RoleSeg::OutputBody);

    // Clearing an attribute that is not set: loud.
    assert_eq!(
        doc.apply(&DocEdit::ClearAppearance {
            name: body.clone(),
            kind: AttrKind::Color,
        }, Tol::witness())
        .unwrap_err(),
        EditError::AppearanceNotSet {
            name: body.clone(),
            kind: AttrKind::Color,
        }
    );

    // Three kinds coexist on one name.
    let doc = set(doc, body.clone(), red());
    let doc = set(doc, body.clone(), Attr::Visibility(false));
    let doc = set(doc, body.clone(), Attr::Label("housing".into()));
    let attrs = doc.appearance_of(&body).unwrap();
    assert_eq!(attrs.attrs.len(), 3);
    assert_eq!(
        attrs.attrs.get(&AttrKind::Label),
        Some(&Attr::Label("housing".into()))
    );

    // Same-kind set replaces (one slot per kind).
    let doc = set(doc, body.clone(), Attr::Color(Rgba8::opaque(0, 0, 255)));
    let attrs = doc.appearance_of(&body).unwrap();
    assert_eq!(attrs.attrs.len(), 3);
    assert_eq!(
        attrs.attrs.get(&AttrKind::Color),
        Some(&Attr::Color(Rgba8::opaque(0, 0, 255)))
    );

    // Clear kind-by-kind; clearing the last one drops the entry, and
    // a second clear of the same kind is loud.
    let (doc, _) = step(
        doc,
        DocEdit::ClearAppearance {
            name: body.clone(),
            kind: AttrKind::Color,
        },
    );
    assert!(
        !doc.appearance_of(&body)
            .unwrap()
            .attrs
            .contains_key(&AttrKind::Color)
    );
    let (doc, _) = step(
        doc,
        DocEdit::ClearAppearance {
            name: body.clone(),
            kind: AttrKind::Visibility,
        },
    );
    let (doc, _) = step(
        doc,
        DocEdit::ClearAppearance {
            name: body.clone(),
            kind: AttrKind::Label,
        },
    );
    assert!(
        doc.appearance_of(&body).is_none(),
        "empty set drops the entry"
    );
    assert!(doc.appearance().is_empty());
}

#[test]
fn appearance_edits_replay_bit_identically_and_diff_reports_them() {
    let doc0 = ProfileDoc::empty_derived("m4_pr7_appearance", Tol::witness());
    let (doc1, p) = insert(
        doc0.clone(),
        Node::Profile(desc(
            [0.0; 3],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![square(0.0, 0.0, 0.5)],
        )),
    );
    let (doc2, ext) = insert(
        doc1,
        Node::Extrude {
            profile: p,
            distance: len(1.0),
        },
    );
    let cap = name1(EntityKind::Face, ext, RoleSeg::Cap(CapEnd::Top));
    let doc3 = set(doc2.clone(), cap.clone(), red());

    // diff: appearance-only change is reported, and only it.
    let d = doc2.diff(&doc3);
    assert!(d.appearance_changed);
    assert!(d.nodes.is_empty() && d.params.is_empty() && !d.metadata_changed);
    assert!(!d.is_empty());

    // Replay from empty reproduces the appearance bit-identically.
    let edits = vec![
        DocEdit::InsertNode {
            node: doc3.node(p).unwrap().clone(),
        },
        DocEdit::InsertNode {
            node: doc3.node(ext).unwrap().clone(),
        },
        DocEdit::SetAppearance {
            name: cap,
            attr: red(),
        },
    ];
    let replayed = ProfileDoc::replay(doc3.id(), &edits, Tol::witness()).unwrap();
    assert!(replayed.bit_eq(&doc3));
}

// ---- Survival across recompute ----

#[test]
fn attribute_survives_no_flip_parameter_motion_on_the_die() {
    let d = die();
    // Attribute the final body and one face of the final table by the
    // names the FINAL node mints.
    let body = name1(EntityKind::Body, d.final_node, RoleSeg::OutputBody);
    let ev0 = run(&d.doc);
    let final_table = &ev0.value(d.final_node).unwrap().name_table;
    let face = final_table
        .iter()
        .find_map(|(n, e)| {
            (n.kind == EntityKind::Face && matches!(e, editor_core::Entry::Unique(_)))
                .then(|| n.clone())
        })
        .expect("final die table has a unique face");
    let doc = set(d.doc, body.clone(), Attr::Label("die".into()));
    let doc = set(doc, face.clone(), red());

    let ev1 = run(&doc);
    assert!(
        ev1.appearance.is_lossless(),
        "losses: {:?}",
        ev1.appearance.losses
    );
    let rows = ev1.appearance.for_node(d.final_node).unwrap();
    // The body row and the face row both landed.
    assert!(rows.iter().any(|(ent, attrs)| {
        ent.key == EntityKey::Body && attrs.get(&AttrKind::Label).is_some()
    }));
    assert!(rows.iter().any(|(ent, attrs)| {
        matches!(ent.key, EntityKey::Face(_)) && attrs.get(&AttrKind::Color) == Some(&red())
    }));

    // No-flip continuous motion (the D5 invariance corpus pattern:
    // dyadic, still-shallow pip depth).
    let (doc2, _) = step(
        doc,
        DocEdit::SetDocParam {
            name: ParamName::new("pip_depth"),
            value: DocParam::Continuous {
                dim: Dimension::Length,
                value: DEPTH * 1.5,
            },
        },
    );
    let ev2 = rerun(&doc2, &ev1);
    assert!(ev2.recomputed > 0, "the edit must recompute its cone");
    // The attribute rode the name: same resolution, zero losses.
    assert!(
        ev2.appearance.is_lossless(),
        "losses: {:?}",
        ev2.appearance.losses
    );
    assert_eq!(ev1.appearance, ev2.appearance);
}

#[test]
fn appearance_only_edit_recomputes_zero_nodes() {
    let d = die();
    let ev0 = run(&d.doc);
    let body = name1(EntityKind::Body, d.final_node, RoleSeg::OutputBody);
    let doc = set(d.doc, body.clone(), red());
    let ev1 = rerun(&doc, &ev0);
    // Appearance is presentation metadata: not in any content key.
    assert_eq!(ev1.recomputed, 0, "appearance edit must not recompute");
    assert!(ev1.appearance.is_lossless());
    let rows = ev1.appearance.for_node(d.final_node).unwrap();
    assert_eq!(rows.len(), 1);
    // And the resolution reflects the edit immediately.
    let (_, attrs) = rows.iter().next().unwrap();
    assert_eq!(attrs.get(&AttrKind::Color), Some(&red()));
}

#[test]
fn transform_pass_through_carries_the_attribute_downstream() {
    let (doc, ext) = block(
        ProfileDoc::empty_derived("m4_pr7_appearance", Tol::witness()),
        (0.0, 1.0),
        (0.0, 1.0),
        0.0,
        1.0,
    );
    let (doc, moved) = insert(
        doc,
        Node::Transform {
            input: ext,
            translation: [len(4.0), len(0.0), len(0.0)],
            rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
            rotation_angle: fixture::ang(0.0),
        },
    );
    let cap = name1(EntityKind::Face, ext, RoleSeg::Cap(CapEnd::Top));
    let doc = set(doc, cap.clone(), red());
    let ev = run(&doc);
    assert!(ev.appearance.is_lossless());
    // The SAME name resolves in both tables (Transform contributes no
    // RolePath segment), so the placed copy inherits the attribute
    // with no policy involved.
    for node in [ext, moved] {
        let rows = ev.appearance.for_node(node).unwrap();
        assert_eq!(rows.len(), 1, "one attributed face at {node:?}");
        let (ent, attrs) = rows.iter().next().unwrap();
        assert!(matches!(ent.key, EntityKey::Face(_)));
        assert_eq!(attrs.get(&AttrKind::Color), Some(&red()));
    }
}

// ---- Loud, typed loss semantics (N3/N5) ----

#[test]
fn deleting_the_minting_node_strands_the_attribute_loudly() {
    let (doc, ext) = block(
        ProfileDoc::empty_derived("m4_pr7_appearance", Tol::witness()),
        (0.0, 1.0),
        (0.0, 1.0),
        0.0,
        1.0,
    );
    let cap = name1(EntityKind::Face, ext, RoleSeg::Cap(CapEnd::Top));
    let doc = set(doc, cap.clone(), red());
    // Deleting the extrude is allowed (N5 dangling semantics — the
    // appearance entry is a reference, not a DAG edge).
    let (doc, _) = step(doc, DocEdit::DeleteNode { id: ext });
    let ev = run(&doc);
    assert_eq!(ev.appearance.losses.len(), 1);
    let loss = &ev.appearance.losses[0];
    assert_eq!(loss.name, cap);
    assert_eq!(loss.cause, AppearanceLossCause::NodeGone);
    // Nothing silently dropped: the payload rides the loss and the
    // store still holds the entry.
    assert_eq!(loss.attrs.get(&AttrKind::Color), Some(&red()));
    assert!(doc.appearance_of(&cap).is_some());
    // ClearAppearance is the repair path and works on the stranded
    // name (no liveness requirement).
    let (doc, _) = step(
        doc,
        DocEdit::ClearAppearance {
            name: cap,
            kind: AttrKind::Color,
        },
    );
    assert!(doc.appearance().is_empty());
    assert!(run(&doc).appearance.is_lossless());
}

#[test]
fn failed_target_node_is_a_typed_indeterminate_loss() {
    let (doc, ext) = block(
        ProfileDoc::empty_derived("m4_pr7_appearance", Tol::witness()),
        (0.0, 1.0),
        (0.0, 1.0),
        0.0,
        1.0,
    );
    let cap = name1(EntityKind::Face, ext, RoleSeg::Cap(CapEnd::Top));
    let doc = set(doc, cap.clone(), red());
    // Degenerate the extrusion: the node fails, the attachment is
    // indeterminate (NOT retired) and typed as such.
    let (doc, _) = step(
        doc,
        DocEdit::SetParam {
            node: ext,
            slot: editor_core::SlotId::Distance,
            expr: len(0.0),
        },
    );
    let ev = run(&doc);
    assert_eq!(ev.appearance.losses.len(), 1);
    assert_eq!(
        ev.appearance.losses[0].cause,
        AppearanceLossCause::TargetFailed { node: ext }
    );
    // Repair the parameter: the attribute resolves again, unchanged —
    // it was never dropped.
    let (doc, _) = step(
        doc,
        DocEdit::SetParam {
            node: ext,
            slot: editor_core::SlotId::Distance,
            expr: len(1.0),
        },
    );
    let ev = run(&doc);
    assert!(ev.appearance.is_lossless());
}

#[test]
fn poisoned_target_node_reports_the_failed_ancestor() {
    let doc = ProfileDoc::empty_derived("m4_pr7_appearance", Tol::witness());
    // Decoupled overlap (M4 PR 5: coincident planes demand a Declare;
    // this test wants a plain transversal union).
    let (doc, a) = block(doc, (0.0, 2.0), (0.0, 2.0), 0.0, 1.0);
    let (doc, b) = block(doc, (1.0, 3.0), (0.25, 1.75), 0.125, 0.75);
    let (doc, uni) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a,
            b,
            declare: None,
        },
    );
    // Attribute a UNION-minted face name (node = the boolean).
    let ev = run(&doc);
    let uni_face = ev
        .value(uni)
        .unwrap()
        .name_table
        .iter()
        .find_map(|(n, e)| {
            (n.kind == EntityKind::Face
                && n.node == uni
                && matches!(e, editor_core::Entry::Unique(_)))
            .then(|| n.clone())
        })
        .expect("union mints face names");
    let doc = set(doc, uni_face.clone(), red());
    assert!(run(&doc).appearance.is_lossless());
    // Break operand A: the union is poisoned through it.
    let (doc, _) = step(
        doc,
        DocEdit::SetParam {
            node: a,
            slot: editor_core::SlotId::Distance,
            expr: len(0.0),
        },
    );
    let ev = run(&doc);
    assert_eq!(ev.appearance.losses.len(), 1);
    assert_eq!(
        ev.appearance.losses[0].cause,
        AppearanceLossCause::TargetPoisoned { through: a }
    );
}

#[test]
fn structural_count_reduction_vanishes_the_instance_name_loudly() {
    let (doc, ext) = block(
        ProfileDoc::empty_derived("m4_pr7_appearance", Tol::witness()),
        (0.0, 1.0),
        (0.0, 1.0),
        0.0,
        1.0,
    );
    let (doc, pat) = insert(
        doc,
        Node::Pattern {
            input: ext,
            count: Expr::count(3),
            kind: PatternKind::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: len(2.0),
            },
        },
    );
    // Attribute a face of instance 2 by its pattern-minted name.
    let ev = run(&doc);
    let inst2_face = ev
        .value(pat)
        .unwrap()
        .name_table
        .iter()
        .find_map(|(n, _)| {
            (n.kind == EntityKind::Face
                && matches!(n.path.first(), Some(RoleSeg::Instance { i: 2, .. })))
            .then(|| n.clone())
        })
        .expect("pattern mints instance-2 face names");
    let doc = set(doc, inst2_face.clone(), red());
    assert!(run(&doc).appearance.is_lossless());

    // Structural edit: count 3 -> 2. Instance 2 no longer exists; the
    // name vanishes and the loss is typed, never silent.
    let (doc, _) = step(
        doc,
        DocEdit::SetStructuralParam {
            node: pat,
            slot: editor_core::SlotId::Count,
            expr: Expr::count(2),
        },
    );
    let ev = run(&doc);
    assert_eq!(ev.appearance.losses.len(), 1);
    let loss = &ev.appearance.losses[0];
    assert_eq!(loss.name, inst2_face);
    assert_eq!(
        loss.cause,
        AppearanceLossCause::Vanished {
            candidates: Vec::new()
        }
    );
    // Restoring the count restores resolution (the attribute rode the
    // name through the round trip).
    let (doc, _) = step(
        doc,
        DocEdit::SetStructuralParam {
            node: pat,
            slot: editor_core::SlotId::Count,
            expr: Expr::count(3),
        },
    );
    assert!(run(&doc).appearance.is_lossless());
}

/// The PR 3 genuine-tie fixture: a U cutter through one wall; both
/// B-cap prong fragments tie (no covariant qualifier separates them).
/// Returns (doc, subtract node).
fn tie_fixture() -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("m4_pr7_appearance", Tol::witness());
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
    (doc, sub)
}

/// A tied name (width 2) from the fixture's subtract table.
fn tied_name(ev: &Evaluation<f64>, sub: RecipeNodeId) -> StableName {
    ev.value(sub)
        .unwrap()
        .name_table
        .iter()
        .find_map(|(n, e)| {
            matches!(e, editor_core::Entry::Tied(c) if c.len() == 2).then(|| n.clone())
        })
        .expect("the U-cutter fixture ties")
}

#[test]
fn tied_name_appearance_is_ambiguous_never_painted() {
    let (doc, sub) = tie_fixture();
    let ev = run(&doc);
    let tied = tied_name(&ev, sub);
    // Attaching is a legal edit (tie status is evaluation-level
    // knowledge)…
    let doc = set(doc, tied.clone(), red());
    let ev = run(&doc);
    // …but resolution refuses to auto-pick: typed Ambiguous, and the
    // tied candidates are NOT painted.
    assert_eq!(ev.appearance.losses.len(), 1);
    assert_eq!(
        ev.appearance.losses[0].cause,
        AppearanceLossCause::Ambiguous { at: sub, width: 2 }
    );
    assert!(ev.appearance.resolved.is_empty());
}

#[test]
fn ambiguous_loss_is_deduplicated_across_carrying_tables() {
    // Review A2 (adapted from the reviewer's transform-duplicate
    // probe): a tied name passed through a Transform appears in TWO
    // tables; the loss report stays per-name — exactly ONE Ambiguous
    // row, `at` = the first carrying node in id order (the subtract),
    // the rest derivable by table lookup.
    let (doc, sub) = tie_fixture();
    let (doc, _moved) = insert(
        doc,
        Node::Transform {
            input: sub,
            translation: [len(10.0), len(0.0), len(0.0)],
            rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
            rotation_angle: fixture::ang(0.0),
        },
    );
    let ev = run(&doc);
    let tied = tied_name(&ev, sub);
    let doc = set(doc, tied.clone(), red());
    let ev = run(&doc);
    assert_eq!(
        ev.appearance.losses.len(),
        1,
        "one loss row per (name, cause): {:?}",
        ev.appearance.losses
    );
    assert_eq!(ev.appearance.losses[0].name, tied);
    assert_eq!(
        ev.appearance.losses[0].cause,
        AppearanceLossCause::Ambiguous { at: sub, width: 2 }
    );
    assert!(ev.appearance.resolved.is_empty(), "ties are never painted");
}

#[test]
fn operand_paint_does_not_follow_the_face_through_a_boolean() {
    // Review A1 (RULED, upheld as correct v1 semantics; fixture
    // adapted from the reviewer's probe_boolean_final_output_
    // silently_unpainted): painting an OPERAND's cap and then
    // consuming the operand in a union paints the operand node's
    // output ONLY. The union's corresponding face is a DIFFERENT
    // derivation (`FromA(cap)` ≠ `cap`, N1 identity), so the final
    // node shows neither the paint nor a loss and the resolution is
    // lossless — auto-following the face through the boolean would be
    // a silent rebinding policy, and the N5 menu is EMPTY by ratified
    // decision. Repairs are explicit: attribute the name the
    // displayed node's table mints, or PR 4's Rebind. This test PINS
    // the ruling; changing it requires a ratified policy, not a code
    // tweak.
    let doc = ProfileDoc::empty_derived("m4_pr7_appearance", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, uni) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a,
            b,
            declare: None,
        },
    );
    let cap = name1(EntityKind::Face, a, RoleSeg::Cap(CapEnd::Top));
    let doc = set(doc, cap.clone(), red());
    let ev = run(&doc);
    // Lossless, painted at `a` — and NOTHING at the union node.
    assert!(ev.appearance.is_lossless());
    assert!(ev.appearance.for_node(a).is_some());
    assert!(
        ev.appearance.for_node(uni).is_none(),
        "the final body carries no paint and no loss row (ruled A1)"
    );
}

#[test]
fn canceled_run_reports_not_evaluated_not_vanished() {
    let (doc, ext) = block(
        ProfileDoc::empty_derived("m4_pr7_appearance", Tol::witness()),
        (0.0, 1.0),
        (0.0, 1.0),
        0.0,
        1.0,
    );
    let cap = name1(EntityKind::Face, ext, RoleSeg::Cap(CapEnd::Top));
    let doc = set(doc, cap, red());
    let cancel = CancelToken::new();
    cancel.cancel();
    let ev = evaluate::<f64>(&doc, None, &cancel, &EvalOptions::default(), Tol::witness());
    assert_eq!(ev.outcome, editor_core::EvalOutcome::Canceled);
    assert_eq!(ev.appearance.losses.len(), 1);
    assert_eq!(
        ev.appearance.losses[0].cause,
        AppearanceLossCause::TargetNotEvaluated
    );
}
