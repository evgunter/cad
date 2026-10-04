//! **Readers read ids** — INTENT-VARS-1 PR 3's rows of the spec's §4
//! test plan (`docs/INTENT-VARS-1-SPEC.md`).
//!
//! Rows here: 1 (a rename moves nothing that identifies), 2 (a delete
//! leaves its readers unresolved), 3's rename half (a name stays
//! unique), 4's delete half (the mint never reuses an id), 5 (equal
//! values are not one variable), 6's rename half (the symbol survives a
//! rename), 7 (lowering), 9 (the anonymous lifecycle), 10's reader half
//! (the round trip and the named-reader refusal), 11's rename half
//! (analysis keyed by the same id across a rename, and a seed on an
//! unresolved variable) and 12 (split and inline carry readers by id).
//! Row 13 is the bindings': `crates/pncad-py/tests/test_variables.py`
//! and the viewer's headless rows.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeSet;
use std::sync::Arc;

use crate::corpus::{body_of, failures};
use crate::fixture::resolver::PartStore;
use crate::fixture::{desc, insert, len, on_frame, prism_edges, square};
use editor_core::analysis::{AnalysisPolicy, analyzed_box, seed_env};
use editor_core::persist::SnapshotError;
use editor_core::{
    CancelToken, Dimension, Distribution, DocEdit, DocumentId, EditError, EvalError, EvalOptions,
    Evaluation, Expr, ExtrudeSide, FreeVar, InlineError, Maintenance, Node, NodeErrorKind,
    NodeResult, PersistError, ProfileDoc, ProfileProgram, RecipeNodeId, SlotId, SplitError, VarDef,
    VarId, VarName, apply, evaluate, inline, load, save, split,
};
use geom_brep::RadiusEvidence;
use geom_core::predicate::{Band, Margin, Sign};
use geom_core::{ParamSymbol, Real, Sym, SymBudget, SymRules, Tol};
use topo::{Body, FaceKey, SurfaceField};

/// The blend radius, metres (dyadic).
const R: f64 = 0.125;

fn n(name: &'static str) -> VarName {
    VarName::from_static(name)
}

fn named(name: &'static str) -> Expr {
    Expr::named(n(name), Dimension::Length)
}

fn try_step(
    doc: &ProfileDoc,
    edit: DocEdit<ProfileProgram>,
) -> Result<editor_core::Applied<ProfileProgram>, EditError> {
    apply(doc, &edit, Tol::witness(), &editor_core::RefusingReach)
}

fn step(doc: &ProfileDoc, edit: DocEdit<ProfileProgram>) -> editor_core::Applied<ProfileProgram> {
    try_step(doc, edit).expect("the edit applies")
}

fn declare(doc: &ProfileDoc, name: &'static str, value: f64) -> ProfileDoc {
    step(
        doc,
        DocEdit::DeclareVar {
            name: n(name),
            def: VarDef::Free(FreeVar::continuous(Dimension::Length, value)),
        },
    )
    .doc
}

fn id(doc: &ProfileDoc, name: &str) -> VarId {
    doc.var_named(name).expect("declared")
}

/// A unit cube at `cx` with every edge blended by `radius`: the cube's
/// id and the blend's.
fn filleted(doc: ProfileDoc, cx: f64, radius: Expr) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let (doc, profile) = on_frame(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(cx, 0.0, 0.5)],
    );
    let (doc, cube) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(1.0),
            side: ExtrudeSide::Along,
        },
    );
    let edges = prism_edges(&doc, cube, 4);
    let (doc, blend) = insert(doc, Node::fillet(cube, radius, edges));
    (doc, cube, blend)
}

/// `w` declared, and a cube blended by `w`: the blend's id.
fn blended_by_w() -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let doc = ProfileDoc::empty(DocumentId::derive("intent-vars-3"), Tol::witness());
    let doc = declare(&doc, "w", R);
    filleted(doc, 0.0, named("w"))
}

fn eval_after(doc: &ProfileDoc, prev: Option<&Evaluation<f64>>) -> Evaluation<f64> {
    evaluate::<f64>(
        doc,
        prev,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

/// One cylindrical blend carrier of `body`, in deterministic arena order.
fn a_cylinder_face(body: &Body<f64>) -> FaceKey {
    topo::query::all_faces(body)
        .into_iter()
        .find(|&f| {
            body.get_face(f)
                .and_then(|fd| body.get_surface(fd.surface))
                .is_some_and(|s| matches!(s, geom::Surface::Cylinder { .. }))
        })
        .expect("a blended cube carries quarter-cylinder blends")
}

/// The radius token a blend's cylinder carries.
fn radius_token(body: &Body<f64>) -> topo::ParamSource {
    let face = a_cylinder_face(body);
    let surface = body.get_face(face).expect("a live face").surface;
    body.surface_field_source(surface, SurfaceField::CylinderRadius)
        .expect("a document-built blend declares its radius")
        .clone()
}

fn evidence(a: &Body<f64>, b: &Body<f64>) -> RadiusEvidence {
    topo::field_source_evidence(
        a,
        a_cylinder_face(a),
        b,
        a_cylinder_face(b),
        SurfaceField::CylinderRadius,
    )
}

/// The slot expression `node` holds at `slot`.
fn slot(doc: &ProfileDoc, node: RecipeNodeId, slot: SlotId) -> &Expr {
    doc.node(node)
        .and_then(|n| n.expr(slot))
        .expect("the slot is there")
}

/// Runs `f` inside one symbolic session, answering its value and the
/// session's counts.
fn session<R>(f: impl FnOnce() -> R) -> (R, geom_core::SymCounts) {
    geom_core::sym::with_session_rules(
        SymBudget {
            max_terms: 4096,
            max_degree: 128,
        },
        SymRules::shipped(),
        f,
    )
}

// --------------------------------------------------------------- row 1

/// Row 1: a rename writes the name and nothing else. The blend's radius
/// token is the same bytes, every node is a memo hit, the diff is
/// empty, the edit is not structural, and the slot reads back under the
/// new name.
#[test]
fn a_rename_moves_nothing_that_identifies() {
    let (doc, _, blend) = blended_by_w();
    let w = id(&doc, "w");
    let ev = eval_after(&doc, None);
    assert!(failures(&ev).is_empty(), "{:?}", failures(&ev));
    let before = radius_token(body_of(&ev, blend));

    let renamed = step(
        &doc,
        DocEdit::RenameVar {
            var: n("w").into(),
            name: Some(n("v")),
        },
    );
    assert!(!renamed.record.structural, "a rename is not structural");
    assert!(renamed.maintenance.is_empty());
    assert!(doc.diff(&renamed.doc).is_empty(), "a name is in no diff");
    assert_eq!(renamed.doc.var_named("v"), Some(w), "one identity, renamed");
    assert_eq!(renamed.doc.var_named("w"), None);

    let ev2 = eval_after(&renamed.doc, Some(&ev));
    assert_eq!(ev2.recomputed, 0, "every node is a memo hit");
    assert_eq!(
        radius_token(body_of(&ev2, blend)),
        before,
        "the token moved"
    );
    assert_eq!(
        renamed
            .doc
            .unparse(slot(&renamed.doc, blend, SlotId::Radius)),
        "v"
    );
    assert_eq!(doc.unparse(slot(&doc, blend, SlotId::Radius)), "w");
}

// --------------------------------------------------------------- row 2

/// Row 2: a delete removes the variable and leaves its reader holding
/// the old id, unresolved. Evaluation refuses at the reader, the node
/// below it still builds, a re-declare of the name is a new identity
/// the reader does not read, and the unresolved document round-trips.
#[test]
fn a_delete_leaves_its_readers_unresolved() {
    let (doc, cube, blend) = blended_by_w();
    let old = id(&doc, "w");
    let deleted = step(&doc, DocEdit::DeleteVar { var: n("w").into() });
    assert!(deleted.record.structural, "the variable had a reader");
    let doc = deleted.doc;
    assert!(doc.var(old).is_none() && doc.var_named("w").is_none());
    assert!(doc.has_minted_var(old), "the log keeps the id");
    assert_eq!(
        slot(&doc, blend, SlotId::Radius),
        &Expr::var(old, Dimension::Length),
        "the reader is untouched"
    );

    let ev = eval_after(&doc, None);
    match ev.result(blend) {
        Some(NodeResult::Failed(e)) => assert!(
            matches!(
                &e.kind,
                NodeErrorKind::Expr {
                    slot: SlotId::Radius,
                    source: EvalError::UnresolvedVar { var },
                } if *var == old
            ),
            "{e:?}"
        ),
        other => panic!("the reader refuses, got {other:?}"),
    }
    assert!(ev.value(cube).is_some(), "a node that reads nothing builds");

    let redeclared = declare(&doc, "w", R);
    let new = id(&redeclared, "w");
    assert_ne!(new, old, "a re-declare mints a new identity");
    assert_eq!(
        slot(&redeclared, blend, SlotId::Radius),
        &Expr::var(old, Dimension::Length),
        "the reader stays on the deleted variable"
    );

    let text = save(&doc, &[], Tol::witness()).expect("an unresolved document saves");
    let loaded = load(&text, Tol::witness()).expect("and loads").doc;
    assert!(loaded.bit_eq(&doc), "the round trip is exact");

    // A reader of an id the document never minted is a file fault.
    let forged = text.replace(&format!("\"var\": {}", old.0), "\"var\": 1");
    assert_ne!(forged, text, "the surgery is aimed at the reader");
    match load(&forged, Tol::witness()) {
        Err(PersistError::Snapshot(SnapshotError::ReaderOfUnmintedVar { node, var })) => {
            assert_eq!((node.id(), var), (blend, VarId(1)));
        }
        other => panic!("a reader of an unminted id refuses, got {other:?}"),
    }
}

// --------------------------------------------------------------- row 3

/// Row 3's rename half: a name stays unique, and a rename that writes
/// the name already held refuses.
#[test]
fn a_rename_keeps_a_name_unique() {
    let (doc, _, _) = blended_by_w();
    let doc = declare(&doc, "v", R);
    match try_step(
        &doc,
        DocEdit::RenameVar {
            var: n("w").into(),
            name: Some(n("v")),
        },
    ) {
        Err(EditError::VarNameTaken { name, holder }) => {
            assert_eq!(name, n("v"));
            assert_eq!(holder.id(), id(&doc, "v"));
        }
        other => panic!("a taken name refuses, got {other:?}"),
    }
    match try_step(
        &doc,
        DocEdit::RenameVar {
            var: n("w").into(),
            name: Some(n("w")),
        },
    ) {
        Err(EditError::VarNameUnchanged { var }) => assert_eq!(var.id(), id(&doc, "w")),
        other => panic!("a no-op rename refuses, got {other:?}"),
    }
}

// --------------------------------------------------------------- row 4

/// Row 4's delete half: declare, delete, declare the identical
/// statement — two ids, both in the log.
#[test]
fn the_mint_never_reuses_a_deleted_id() {
    let doc = ProfileDoc::empty(DocumentId::derive("intent-vars-3-mint"), Tol::witness());
    let first = declare(&doc, "w", R);
    let a = id(&first, "w");
    let gone = step(&first, DocEdit::DeleteVar { var: a.into() }).doc;
    let again = declare(&gone, "w", R);
    let b = id(&again, "w");
    assert_ne!(a, b);
    assert!(again.has_minted_var(a) && again.has_minted_var(b));
    assert_eq!(again.mint().vars().collect::<Vec<_>>().len(), 2);
}

// --------------------------------------------------------------- row 5

/// Row 5: `w` and `v` at the same value are two variables. Blends
/// reading the two lower to distinct tokens and are not `Declared`;
/// two blends reading `w` are.
#[test]
fn equal_values_are_not_one_variable() {
    let doc = ProfileDoc::empty(DocumentId::derive("intent-vars-3-twins"), Tol::witness());
    let doc = declare(&doc, "w", R);
    let doc = declare(&doc, "v", R);
    let (doc, _, by_w) = filleted(doc, 0.0, named("w"));
    let (doc, _, by_v) = filleted(doc, 4.0, named("v"));
    let (doc, _, by_w_again) = filleted(doc, 8.0, named("w"));
    let ev = eval_after(&doc, None);
    assert!(failures(&ev).is_empty(), "{:?}", failures(&ev));
    let (a, b, c) = (
        body_of(&ev, by_w),
        body_of(&ev, by_v),
        body_of(&ev, by_w_again),
    );
    assert_ne!(radius_token(a), radius_token(b));
    assert_ne!(evidence(a, b), RadiusEvidence::Declared);
    assert_eq!(radius_token(a), radius_token(c));
    assert_eq!(evidence(a, c), RadiusEvidence::Declared);
}

// --------------------------------------------------------------- row 6

/// Row 6's rename half: the symbol a variable binds to in the symbolic
/// tier is its id's, so the binding before a rename and the one after
/// subtract to a theorem.
#[test]
fn the_symbol_survives_a_rename() {
    let (doc, _, _) = blended_by_w();
    let w = id(&doc, "w");
    let renamed = step(
        &doc,
        DocEdit::RenameVar {
            var: w.into(),
            name: Some(n("v")),
        },
    )
    .doc;
    let leaf = editor_core::ParamBox::from_axes(std::collections::BTreeMap::new());
    let bound = |doc: &ProfileDoc| match editor_core::var_env_over::<Sym<f64>, _>(doc, &leaf)
        .expect("binds")
        .bindings[&w]
    {
        editor_core::ParamValue::Continuous { value, .. } => value,
        ref other => panic!("{other:?}"),
    };
    let (sign, counts) = session(|| {
        geom_core::k_stats::decide(
            "intent_vars_3",
            Margin::of(bound(&doc) - bound(&renamed)),
            Band::new(1.0e-9, 1.0e-8).unwrap(),
        )
    });
    assert_eq!(sign, Ok(Sign::Zero));
    assert_eq!(counts.symbolic_zero, 1, "one symbol before and after");
    let by_hand = Sym::<f64>::from_f64(R) + Sym::param_over(ParamSymbol::new(w.0), 0.0, 0.0, 0.0);
    let (_, counts) = session(|| {
        geom_core::k_stats::decide(
            "intent_vars_3",
            Margin::of(bound(&renamed) - by_hand),
            Band::new(1.0e-9, 1.0e-8).unwrap(),
        )
    });
    assert_eq!(counts.symbolic_zero, 1, "the symbol is the id's");
}

// --------------------------------------------------------------- row 7

/// Row 7: the edit door lowers a name to the id it names before
/// minting, so a node authored by name and the same node authored by id
/// mint one id and store one node; an unknown name and a name read at
/// the wrong kind refuse at the slot; and a log of authored edits
/// replays to the same document.
#[test]
fn the_door_lowers_names_before_it_mints() {
    let (doc, _, blend) = blended_by_w();
    let w = id(&doc, "w");
    let Some(Node::Fillet {
        target, selection, ..
    }) = doc.node(blend).cloned()
    else {
        panic!("a fillet");
    };
    let by_name = Node::fillet(target, named("w"), selection.clone());
    let by_id = Node::fillet(target, Expr::var(w, Dimension::Length), selection.clone());
    let a = step(
        &doc,
        DocEdit::InsertNode {
            node: Box::new(by_name),
        },
    );
    let b = step(
        &doc,
        DocEdit::InsertNode {
            node: Box::new(by_id),
        },
    );
    let (ia, ib) = (a.record.minted.unwrap(), b.record.minted.unwrap());
    assert_eq!(ia, ib, "one node, one id");
    assert!(a.doc.node(ia).unwrap().bit_eq(b.doc.node(ib).unwrap()));
    assert!(a.doc.bit_eq(&b.doc));

    match try_step(
        &doc,
        DocEdit::SetParam {
            node: blend,
            slot: SlotId::Radius,
            expr: named("nowhere"),
        },
    ) {
        Err(EditError::SlotUnknownVarName { name, node, slot }) => {
            assert_eq!(
                (name, node.id(), slot),
                (n("nowhere"), blend, SlotId::Radius)
            );
        }
        other => panic!("an unknown name refuses at the slot, got {other:?}"),
    }
    let with_angle = step(
        &doc,
        DocEdit::DeclareVar {
            name: n("a"),
            def: VarDef::Free(FreeVar::continuous(Dimension::Angle, 0.5)),
        },
    )
    .doc;
    // The name is read at a length and held at an angle.
    match try_step(
        &with_angle,
        DocEdit::SetParam {
            node: blend,
            slot: SlotId::Radius,
            expr: named("a"),
        },
    ) {
        Err(EditError::SlotVarKind {
            var,
            node,
            slot,
            declared,
            referenced,
        }) => {
            assert_eq!(var.id(), id(&with_angle, "a"));
            assert_eq!((node.id(), slot), (blend, SlotId::Radius));
            assert_eq!(
                (declared, referenced),
                (Dimension::Angle, Dimension::Length)
            );
        }
        other => panic!("a name of the wrong kind refuses at the slot, got {other:?}"),
    }

    // A log holding authored names replays to the same ids.
    let mut log = vec![DocEdit::DeclareVar {
        name: n("w"),
        def: VarDef::Free(FreeVar::continuous(Dimension::Length, R)),
    }];
    let mut recorded = ProfileDoc::empty(DocumentId::derive("intent-vars-3-log"), Tol::witness());
    recorded = step(&recorded, log[0].clone()).doc;
    let (next, profile) = on_frame(
        recorded,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(0.0, 0.0, 0.5)],
    );
    let frame = next.order()[0];
    log.push(DocEdit::InsertNode {
        node: Box::new(next.node(frame).unwrap().clone()),
    });
    log.push(DocEdit::InsertNode {
        node: Box::new(Node::Profile(desc(frame, vec![square(0.0, 0.0, 0.5)]))),
    });
    let extrude = Node::Extrude {
        profile,
        distance: named("w"),
        side: ExtrudeSide::Along,
    };
    let recorded = step(
        &next,
        DocEdit::InsertNode {
            node: Box::new(extrude.clone()),
        },
    )
    .doc;
    log.push(DocEdit::InsertNode {
        node: Box::new(extrude),
    });
    let replayed = ProfileDoc::replay(recorded.id(), &log, Tol::witness()).expect("replays");
    assert!(
        replayed.bit_eq(&recorded),
        "an authored log replays exactly"
    );
}

// --------------------------------------------------------------- row 9

/// Row 9: a variable with no name is one something reads. Clearing the
/// name of an unread variable refuses; deleting an anonymous one
/// refuses; and the edit that replaces its one reader removes it,
/// reporting so, with the mint log keeping its id.
#[test]
fn an_anonymous_variable_lives_as_long_as_its_readers() {
    let (doc, _, blend) = blended_by_w();
    let w = id(&doc, "w");
    let lonely = declare(&doc, "u", R);
    match try_step(
        &lonely,
        DocEdit::RenameVar {
            var: n("u").into(),
            name: None,
        },
    ) {
        Err(EditError::AnonymousVarUnread { var }) => assert_eq!(var.name(), Some(&n("u"))),
        other => panic!("an unread variable keeps its name, got {other:?}"),
    }

    let anonymous = step(
        &doc,
        DocEdit::RenameVar {
            var: n("w").into(),
            name: None,
        },
    )
    .doc;
    assert_eq!(anonymous.var_name(w), None);
    assert!(anonymous.var(w).is_some(), "read, so it stays");
    assert_eq!(
        anonymous.unparse(slot(&anonymous, blend, SlotId::Radius)),
        format!("#{}", w.full()),
        "an anonymous reader writes its full id"
    );
    match try_step(&anonymous, DocEdit::DeleteVar { var: w.into() }) {
        Err(EditError::DeleteAnonymousVar { var }) => assert_eq!(var.id(), w),
        other => panic!("an anonymous variable is not deleted by hand, got {other:?}"),
    }

    let replaced = step(
        &anonymous,
        DocEdit::SetParam {
            node: blend,
            slot: SlotId::Radius,
            expr: len(R),
        },
    );
    assert_eq!(
        replaced.maintenance,
        vec![Maintenance::AnonymousVarRemoved {
            var: anonymous.spoken_var(w),
        }]
    );
    assert!(replaced.doc.var(w).is_none() && replaced.doc.var_order().is_empty());
    assert!(replaced.doc.has_minted_var(w), "the log keeps the id");
}

// -------------------------------------------------------------- row 10

/// Row 10's reader half: a document holding a named variable, an
/// anonymous one and an unresolved reader saves and loads bit for bit;
/// a snapshot holding a name leaf refuses.
#[test]
fn readers_round_trip_and_a_stored_name_refuses() {
    let doc = ProfileDoc::empty(DocumentId::derive("intent-vars-3-wire"), Tol::witness());
    let doc = declare(&doc, "w", R);
    let doc = declare(&doc, "gone", R);
    let doc = declare(&doc, "nameless", R);
    let (doc, _, _) = filleted(doc, 0.0, named("w"));
    let (doc, _, _) = filleted(doc, 4.0, named("gone"));
    let (doc, _, by_nameless) = filleted(doc, 8.0, named("nameless"));
    let doc = step(
        &doc,
        DocEdit::DeleteVar {
            var: n("gone").into(),
        },
    )
    .doc;
    let doc = step(
        &doc,
        DocEdit::RenameVar {
            var: n("nameless").into(),
            name: None,
        },
    )
    .doc;
    let text = save(&doc, &[], Tol::witness()).expect("saves");
    let loaded = load(&text, Tol::witness()).expect("loads").doc;
    assert!(loaded.bit_eq(&doc));
    assert_eq!(loaded.var_names().len(), 1);
    assert_eq!(loaded.vars().len(), 2);

    let nameless = doc
        .vars()
        .keys()
        .copied()
        .find(|v| doc.var_name(*v).is_none())
        .expect("the anonymous variable");
    let corrupt = crate::wire::doctored(&text, |wire| {
        let radius = &mut wire["snapshot"]["nodes"][by_nameless.0.to_string()]["Fillet"]["radius"];
        assert_eq!(
            radius["Var"]["var"],
            serde_json::json!(nameless.0),
            "the surgery is aimed at the anonymous reader"
        );
        *radius = serde_json::json!({ "Name": { "name": "nameless", "dim": "Length" } });
    });
    match load(&corrupt, Tol::witness()) {
        Err(PersistError::Snapshot(SnapshotError::NamedReaderInSnapshot { node })) => {
            assert_eq!(node.id(), by_nameless);
        }
        other => panic!("a stored name refuses, got {other:?}"),
    }
}

// -------------------------------------------------------------- row 11

/// Row 11's rename half: the analysis box is keyed by the same id, and
/// equal, across a rename; a seed on a deleted variable refuses typed.
#[test]
fn analysis_keeps_its_ids_across_a_rename() {
    let (doc, _, _) = blended_by_w();
    let w = id(&doc, "w");
    let lawed = step(
        &doc,
        DocEdit::SetVarDistribution {
            var: w.into(),
            distribution: Some(Distribution::Normal { sigma: 1e-4 }),
        },
    )
    .doc;
    let renamed = step(
        &lawed,
        DocEdit::RenameVar {
            var: w.into(),
            name: Some(n("v")),
        },
    )
    .doc;
    let policy = AnalysisPolicy::default();
    let (before, after) = (
        analyzed_box(&lawed, &policy),
        analyzed_box(&renamed, &policy),
    );
    assert_eq!(after.params().keys().copied().collect::<Vec<_>>(), vec![w]);
    assert_eq!(before, after, "a rename moves no analysis");
    assert_eq!(after.spoken(w).to_string(), "v");

    let deleted = step(&renamed, DocEdit::DeleteVar { var: w.into() }).doc;
    match seed_env::<geom_core::Dual64, _>(&deleted, deleted.var_env(), w) {
        Err(editor_core::analysis::SeedError::UnknownParam { param }) => {
            assert_eq!(param.id(), w);
        }
        other => panic!("a seed on a deleted variable refuses, got {other:?}"),
    }
}

// -------------------------------------------------------------- row 12

/// A frame, a square and an extrude of depth `depth`, at `cx`: the
/// three ids.
fn block(doc: ProfileDoc, cx: f64, depth: Expr) -> (ProfileDoc, [RecipeNodeId; 3]) {
    let (doc, profile) = on_frame(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(cx, 0.0, 0.5)],
    );
    let frame = doc.order()[doc.order().len() - 2];
    let (doc, extrude) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: depth,
            side: ExtrudeSide::Along,
        },
    );
    (doc, [frame, profile, extrude])
}

/// Row 12: a split declares each variable the cut reads in the part and
/// re-points the carried readers at the part's own ids, and the part
/// loads; an anonymous variable crossing either cut refuses.
#[test]
fn split_and_inline_carry_readers_by_id() {
    let doc = ProfileDoc::empty(DocumentId::derive("intent-vars-3-split"), Tol::witness());
    // `kept` first, so `h`'s id here is not the one the part mints.
    let doc = declare(&doc, "kept", 1.0);
    let doc = declare(&doc, "h", 1.5);
    let (doc, cut) = block(doc, 0.0, named("h"));
    let (doc, _) = block(doc, 10.0, named("kept"));
    let out = split(
        &doc,
        &BTreeSet::from(cut),
        DocumentId::derive("intent-vars-3-part"),
        Tol::witness(),
        None,
    )
    .expect("the cut alone reads h");
    let part_h = id(&out.part, "h");
    assert_ne!(part_h, id(&doc, "h"), "the part mints its own id");
    let extrude = *out.part.order().last().expect("the carried extrude");
    assert_eq!(
        slot(&out.part, extrude, SlotId::Distance),
        &Expr::var(part_h, Dimension::Length),
        "the carried reader reads the part's id"
    );
    let text = save(&out.part, &[], Tol::witness()).expect("the part saves");
    assert!(
        load(&text, Tol::witness())
            .expect("and loads")
            .doc
            .bit_eq(&out.part)
    );

    let anonymous = step(
        &doc,
        DocEdit::RenameVar {
            var: n("h").into(),
            name: None,
        },
    )
    .doc;
    match split(
        &anonymous,
        &BTreeSet::from(cut),
        DocumentId::derive("intent-vars-3-part"),
        Tol::witness(),
        None,
    ) {
        Err(SplitError::AnonymousVarCrossesCut { var, node }) => {
            assert_eq!((var.id(), node.id()), (id(&doc, "h"), cut[2]));
        }
        other => panic!("an anonymous variable does not cross a split, got {other:?}"),
    }

    // The inline door: the part's anonymous variable cannot land in the
    // host.
    let part = ProfileDoc::empty(DocumentId::derive("intent-vars-3-ref"), Tol::witness());
    let part = declare(&part, "d", 1.0);
    let (part, _) = block(part, 0.0, named("d"));
    let part = step(
        &part,
        DocEdit::RenameVar {
            var: n("d").into(),
            name: None,
        },
    )
    .doc;
    let mut store = PartStore::default();
    let doc_ref = store.insert(part.clone(), Tol::witness());
    let host = ProfileDoc::empty(DocumentId::derive("intent-vars-3-host"), Tol::witness());
    let (host, instance) = insert(host, Node::instantiate_part(doc_ref));
    match inline(
        &host,
        instance,
        &(Arc::new(store) as Arc<dyn editor_core::PartResolver>),
        Tol::witness(),
    ) {
        Err(InlineError::AnonymousVarCrossesCut { var }) => {
            assert_eq!(var.id(), *part.vars().keys().next().expect("one variable"));
        }
        other => panic!("an anonymous variable does not cross an inline, got {other:?}"),
    }
}
