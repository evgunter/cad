//! **A variable defined by a formula** — INTENT-LITERALS PR A's rows of
//! the spec's §8 test plan (`docs/INTENT-LITERALS-SPEC.md`): 1 (a
//! definition cycle refuses, at the door and at load), 2 (a defined
//! variable's derivative is its inputs' pushforward, and it takes no
//! seed), 3 (an edit of an input re-runs the reader of a definition),
//! and a row per guard the definitions add: the reads a definition is
//! held to, the expansion bound, the carry-forward doors' refusal, the
//! cascading anonymous lifecycle, the environment's binding order and
//! box, the coincidence token's expansion, the split and inline carry,
//! and the load walks.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeSet;
use std::sync::Arc;

use crate::corpus::{body_of, failures};
use crate::fixture::resolver::PartStore;
use crate::fixture::{Recorder, insert, len, on_frame, prism_edges, square};
use editor_core::analysis::{AnalysisPolicy, analyzed_box, seed_env};
use editor_core::persist::SnapshotError;
use editor_core::stackup::{SensitivityOutcome, sensitivities};
use editor_core::{
    CancelToken, CarryForwardDoor, Dimension, Distribution, DocEdit, DocumentId, EditError,
    EvalError, EvalOptions, Evaluation, ExtrudeSide, Formula, FreeValue, FreeVar, InlineError,
    Maintenance, Node, NodeErrorKind, NodeResult, ParamBox, ParamValue, PersistError, ProfileDoc,
    ProfileProgram, RecipeNodeId, SeedError, UnitSym, VarDecl, VarId, VarName, apply, evaluate,
    inline, load, save, split, var_env_over,
};
use geom_core::predicate::{Band, Margin, Sign};
use geom_core::{Bounds, Interval, Real, Sym, SymBudget, SymRules, Tol};
use topo::{Body, FaceKey, SurfaceField};

/// The input's value, metres (dyadic, so `2·w` and `w + w` agree to
/// the bit).
const W: f64 = 0.0625;

fn n(name: &'static str) -> VarName {
    VarName::from_static(name)
}

fn named(name: &'static str) -> Formula {
    Formula::named(n(name), Dimension::Length)
}

/// `k · name`.
fn times(k: i64, name: &'static str) -> Formula {
    Formula::mul(Formula::ratio(k, 1).unwrap(), named(name)).unwrap()
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

fn declare(doc: &ProfileDoc, name: &'static str, def: VarDecl) -> ProfileDoc {
    step(doc, DocEdit::DeclareVar { name: n(name), def }).doc
}

fn free(value: f64) -> VarDecl {
    VarDecl::Free(FreeVar::continuous(Dimension::Length, value))
}

fn id(doc: &ProfileDoc, name: &str) -> VarId {
    doc.var_named(name).expect("declared")
}

/// `w` free at [`W`], and `h := 2·w`.
fn w_and_h(seed: &str) -> ProfileDoc {
    let doc = ProfileDoc::empty(DocumentId::derive(seed), Tol::witness());
    let doc = declare(&doc, "w", free(W));
    declare(&doc, "h", VarDecl::defined(times(2, "w")))
}

/// A unit square at `cx` extruded by `depth`: the frame, the profile
/// and the extrude.
fn block(doc: ProfileDoc, cx: f64, depth: Formula) -> (ProfileDoc, [RecipeNodeId; 3]) {
    let (doc, profile) = on_frame(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(cx, 0.0, 0.5)],
    );
    let frame = doc.ids()[doc.ids().len() - 2];
    let (doc, extrude) = insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: depth,
            side: ExtrudeSide::Along,
        },
    );
    (doc, [frame, profile, extrude])
}

/// A unit cube at `cx` with every edge blended by `radius`: the blend.
fn filleted(doc: ProfileDoc, cx: f64, radius: Formula) -> (ProfileDoc, RecipeNodeId) {
    let (doc, [_, _, cube]) = block(doc, cx, len(1.0));
    let edges = prism_edges(&doc, cube, 4);
    insert(doc, Node::fillet(cube, radius, edges))
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

/// The radius token a blend's first cylinder carries.
fn radius_token(body: &Body<f64>) -> topo::ParamSource {
    let face: FaceKey = topo::query::all_faces(body)
        .into_iter()
        .find(|&f| {
            body.get_face(f)
                .and_then(|fd| body.get_surface(fd.surface))
                .is_some_and(|s| matches!(s, geom::Surface::Cylinder { .. }))
        })
        .expect("a blended cube carries quarter-cylinder blends");
    let surface = body.get_face(face).expect("a live face").surface;
    body.surface_field_source(surface, SurfaceField::CylinderRadius)
        .expect("a document-built blend declares its radius")
        .clone()
}

/// The doc's snapshot saved, doctored by `edit`, and loaded.
fn load_doctored(doc: &ProfileDoc, edit: impl FnOnce(&mut serde_json::Value)) -> PersistError {
    let text = save(doc, &[], Tol::witness()).unwrap();
    load(&text, Tol::witness()).expect("the undoctored save loads");
    let corrupt = crate::wire::doctored(&text, |wire| edit(&mut wire["snapshot"]));
    load(&corrupt, Tol::witness()).expect_err("the doctored save must refuse")
}

/// A reader of `var` at `dim`, as the wire writes one.
fn wire_var(var: VarId, dim: &str) -> serde_json::Value {
    serde_json::json!({ "Var": { "var": var.0, "dim": dim } })
}

// --------------------------------------------------------------- row 1

/// Row 1: `w := h + 1 mm` where `h := 2·w` refuses `DefinitionCycle`
/// naming `[w, h]`, and `w` stays free; a declare reading its own id
/// is the cycle of one.
#[test]
fn a_definition_reading_itself_back_refuses_as_a_cycle() {
    let doc = w_and_h("intent-literals-a-cycle");
    let (w, h) = (id(&doc, "w"), id(&doc, "h"));
    let plus = Formula::add(named("h"), len(0.001)).unwrap();
    let err = try_step(
        &doc,
        DocEdit::DefineVar {
            var: n("w").into(),
            def: VarDecl::defined(plus),
            fresh: Vec::new(),
        },
    )
    .unwrap_err();
    let EditError::DefinitionCycle { var, through } = err else {
        panic!("not DefinitionCycle: {err:?}")
    };
    assert_eq!(var.id(), w);
    assert_eq!(
        through.iter().map(|v| v.id()).collect::<Vec<_>>(),
        vec![w, h],
        "the cycle runs w → h → w"
    );
    assert!(doc.free(w).is_some(), "w is still free");

    // A declare reading the id it would mint.
    let decl = VarDecl::defined(Formula::var(
        doc.spoken_declare(&n("s"), &free(1.0)).id(),
        Dimension::Length,
    ));
    let would = doc.spoken_declare(&n("s"), &decl).id();
    let selfish =
        VarDecl::defined(Formula::add(Formula::var(would, Dimension::Length), len(1.0)).unwrap());
    match try_step(
        &doc,
        DocEdit::DeclareVar {
            name: n("s"),
            def: selfish,
        },
    ) {
        Err(EditError::DefinitionCycle { var, through }) => {
            assert_eq!(var.id(), would);
            assert_eq!(through.len(), 1, "{through:?}");
        }
        other => panic!("a declare reading itself is a cycle of one, got {other:?}"),
    }
}

/// Row 1's load half: a file whose `w` reads `h` back refuses at the
/// cycle walk, naming the cycle from the first variable declared.
#[test]
fn a_file_holding_a_definition_cycle_refuses_at_load() {
    let doc = w_and_h("intent-literals-a-cycle-load");
    let (w, h) = (id(&doc, "w"), id(&doc, "h"));
    let err = load_doctored(&doc, |snap| {
        snap["vars"][w.0.to_string()]["def"] =
            serde_json::json!({ "Defined": wire_var(h, "Length") });
    });
    let PersistError::Snapshot(SnapshotError::DefinitionCycle { var, through }) = err else {
        panic!("not DefinitionCycle: {err:?}")
    };
    assert_eq!(var.id(), w);
    assert_eq!(
        through.iter().map(|v| v.id()).collect::<Vec<_>>(),
        vec![w, h]
    );
}

// --------------------------------------------------------------- row 2

/// Row 2: with `h := 2·w` and a measure of `h`, the sensitivity has one
/// entry, `w`'s, and it is exactly 2 — the pushforward; a seed on `h`
/// refuses, before any scalar is asked.
#[test]
fn a_defined_variable_carries_its_inputs_derivative_and_takes_no_seed() {
    let doc = w_and_h("intent-literals-a-pushforward");
    let (w, h) = (id(&doc, "w"), id(&doc, "h"));
    // A tolerance on `w`, so the stackup varies it (VR8: an untoleranced
    // variable is a constant of the analysis).
    let doc = step(
        &doc,
        DocEdit::SetVarDistribution {
            var: w.into(),
            distribution: Some(editor_core::Distribution::Normal { sigma: 1e-4 }),
        },
    )
    .doc;
    let entries = sensitivities(&doc, h, None, None, false, None, Tol::witness()).unwrap();
    assert_eq!(
        entries.iter().map(|e| e.param).collect::<Vec<_>>(),
        vec![w],
        "a defined variable is no entry of its own"
    );
    match entries[0].outcome {
        SensitivityOutcome::Derivative { value, .. } => assert_eq!(value, 2.0, "∂h/∂w"),
        ref other => panic!("{other:?}"),
    }
    match seed_env::<f64, _>(&doc, doc.var_env(), h) {
        Err(SeedError::SeedOnNonFreeVar { var }) => assert_eq!(var.id(), h),
        other => panic!("a seed on a defined variable refuses, got {other:?}"),
    }
}

// --------------------------------------------------------------- row 3

/// Row 3: `SetVarValue w` puts `h` in the document diff beside `w` —
/// the diff closes over definitions — and re-runs the reader of `h` and
/// nothing else. The closure is what this row guards: the recompute
/// count alone cannot see it go, because the memo is content-keyed and
/// a stale diff still leaves the reader of `h` to recompute.
#[test]
fn an_input_edit_closes_the_diff_over_its_definitions() {
    let doc = w_and_h("intent-literals-a-invalidation");
    let doc = declare(&doc, "v", free(0.5));
    let (doc, [_, _, reads_h]) = block(doc, 0.0, named("h"));
    let (doc, _) = block(doc, 4.0, named("v"));
    let prior = eval_after(&doc, None);
    assert!(failures(&prior).is_empty(), "{:?}", failures(&prior));
    let edited = step(
        &doc,
        DocEdit::SetVarValue {
            var: n("w").into(),
            value: FreeValue::Continuous(0.25),
        },
    )
    .doc;
    let again = eval_after(&edited, Some(&prior));
    assert_eq!(again.recomputed, 1, "the reader of h, and nothing else");
    assert_eq!(again.reused, edited.len() - 1);
    let depth = |ev: &Evaluation<f64>| {
        let body = body_of(ev, reads_h);
        body.vertex_points()
            .map(|(_, p)| p.z)
            .fold(f64::NEG_INFINITY, f64::max)
    };
    assert_eq!((depth(&prior), depth(&again)), (2.0 * W, 0.5));
    assert_eq!(
        doc.diff(&edited).vars,
        vec![id(&doc, "w"), id(&doc, "h")],
        "the diff closes over definitions"
    );
}

// ----------------------------------------------------- the token rows

/// A slot reading `h := 2·w` and a slot spelling `2·w` lower to one
/// coincidence token; `3·w` does not. A rename moves no token.
#[test]
fn a_reader_of_a_definition_lowers_as_the_definition() {
    let doc = w_and_h("intent-literals-a-token");
    let (doc, by_h) = filleted(doc, 0.0, named("h"));
    let (doc, by_formula) = filleted(doc, 4.0, times(2, "w"));
    let (doc, by_other) = filleted(doc, 8.0, times(3, "w"));
    let ev = eval_after(&doc, None);
    assert!(failures(&ev).is_empty(), "{:?}", failures(&ev));
    let token = |blend| radius_token(body_of(&ev, blend));
    assert_eq!(token(by_h), token(by_formula), "h is its formula");
    assert_ne!(token(by_h), token(by_other));
}

/// A definition respelled to the same value re-runs a flow-bearing
/// reader: the content key writes the expansion, so the memo cannot
/// serve a body whose token names the old formula.
#[test]
fn a_respelled_definition_reruns_its_flow_bearing_reader() {
    let doc = w_and_h("intent-literals-a-key");
    let (doc, blend) = filleted(doc, 0.0, named("h"));
    let prior = eval_after(&doc, None);
    let respelled = step(
        &doc,
        DocEdit::DefineVar {
            var: n("h").into(),
            def: VarDecl::defined(Formula::add(named("w"), named("w")).unwrap()),
            fresh: Vec::new(),
        },
    )
    .doc;
    let again = eval_after(&respelled, Some(&prior));
    assert_eq!(again.recomputed, 1, "the blend re-runs");
    let (fresh, by_sum) = filleted(
        w_and_h("intent-literals-a-key"),
        0.0,
        Formula::add(named("w"), named("w")).unwrap(),
    );
    let cold = eval_after(&fresh, None);
    assert_eq!(
        radius_token(body_of(&again, blend)),
        radius_token(body_of(&cold, by_sum)),
        "and carries the new formula's token"
    );
}

/// A profile's carrier radius read through a definition is in the
/// profile's key the same way: respelled to the same value, the
/// profile and its sweep re-run, and the swept wall carries the new
/// formula's token.
#[test]
fn a_respelled_definition_reruns_the_profile_whose_radius_reads_it() {
    let disc = |doc: ProfileDoc, radius: Formula| {
        let (doc, plane) = insert(
            doc,
            crate::fixture::frame([0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]),
        );
        let (doc, profile) = insert(
            doc,
            Node::Profile(ProfileProgram {
                frame: plane.into(),
                loops: vec![editor_core::LoopProgram::Circle {
                    centre: [len(0.0), len(0.0)],
                    radius,
                }],
                ids: Vec::new(),
            }),
        );
        insert(
            doc,
            Node::Extrude {
                profile: profile.into(),
                distance: len(1.0),
                side: ExtrudeSide::Along,
            },
        )
    };
    let (doc, rod) = disc(w_and_h("intent-literals-a-carrier"), named("h"));
    let prior = eval_after(&doc, None);
    assert!(failures(&prior).is_empty(), "{:?}", failures(&prior));
    let respelled = step(
        &doc,
        DocEdit::DefineVar {
            var: n("h").into(),
            def: VarDecl::defined(Formula::add(named("w"), named("w")).unwrap()),
            fresh: Vec::new(),
        },
    )
    .doc;
    let again = eval_after(&respelled, Some(&prior));
    assert_eq!(again.recomputed, 2, "the profile and its sweep re-run");
    let (fresh, by_sum) = disc(
        w_and_h("intent-literals-a-carrier"),
        Formula::add(named("w"), named("w")).unwrap(),
    );
    let cold = eval_after(&fresh, None);
    assert_eq!(
        radius_token(body_of(&again, rod)),
        radius_token(body_of(&cold, by_sum))
    );
}

// ------------------------------------------------- the door's guards

/// The carry-forward doors refuse a defined variable: it has no value,
/// notation or distribution of its own.
#[test]
fn the_carry_forward_doors_refuse_a_defined_variable() {
    let doc = w_and_h("intent-literals-a-carry");
    let h = id(&doc, "h");
    let edits = [
        (
            CarryForwardDoor::Value,
            DocEdit::SetVarValue {
                var: h.into(),
                value: FreeValue::Continuous(1.0),
            },
        ),
        (
            CarryForwardDoor::Notation,
            DocEdit::SetVarUnit {
                var: h.into(),
                unit: UnitSym::canonical_for(Dimension::Length),
            },
        ),
        (
            CarryForwardDoor::Annotation,
            DocEdit::SetVarDistribution {
                var: h.into(),
                distribution: Some(Distribution::Normal { sigma: 1e-3 }),
            },
        ),
    ];
    for (door, edit) in edits {
        match try_step(&doc, edit) {
            Err(EditError::NotAFreeVar { var, door: at }) => {
                assert_eq!((var.id(), at), (h, door));
            }
            other => panic!("{door:?}: a defined variable is not free, got {other:?}"),
        }
    }
}

/// Every read a definition makes is one the document answers: a name
/// it holds, a live variable, at its kind.
#[test]
fn a_definition_reads_only_what_the_document_holds() {
    let doc = w_and_h("intent-literals-a-reads");
    let w = id(&doc, "w");
    let gone = declare(&doc, "gone", free(1.0));
    let gone_id = id(&gone, "gone");
    let gone = step(
        &gone,
        DocEdit::DeleteVar {
            var: n("gone").into(),
        },
    )
    .doc;
    let declare_in = |doc: &ProfileDoc, expr: Formula| {
        try_step(
            doc,
            DocEdit::DeclareVar {
                name: n("d"),
                def: VarDecl::defined(expr),
            },
        )
    };
    match declare_in(&doc, named("nope")) {
        Err(EditError::DefinitionUnknownVarName { name, .. }) => assert_eq!(name, n("nope")),
        other => panic!("an unheld name, got {other:?}"),
    }
    match declare_in(&doc, Formula::var(VarId::new(0, 0x5eed), Dimension::Length)) {
        Err(EditError::DefinitionUnresolvedVar { read, .. }) => {
            assert_eq!(read.id(), VarId::new(0, 0x5eed))
        }
        other => panic!("an unminted id, got {other:?}"),
    }
    match declare_in(&gone, Formula::var(gone_id, Dimension::Length)) {
        Err(EditError::DefinitionUnresolvedVar { read, .. }) => assert_eq!(read.id(), gone_id),
        other => panic!("a deleted variable, got {other:?}"),
    }
    match declare_in(&doc, Formula::var(w, Dimension::Angle)) {
        Err(EditError::DefinitionVarKind {
            read,
            declared,
            referenced,
            ..
        }) => assert_eq!(
            (read.id(), declared, referenced),
            (w, editor_core::VarKind::Length, Dimension::Angle)
        ),
        other => panic!("a read at the wrong kind, got {other:?}"),
    }
}

/// `h_k := h_{k-1} + h_{k-1}` doubles the expansion each step: the
/// twelfth outgrows the bound and refuses. A redefinition that grows
/// the chain from below refuses naming the variable it pushed past.
#[test]
fn a_definition_past_the_expansion_bound_refuses() {
    const NAMES: [&str; 12] = [
        "h1", "h2", "h3", "h4", "h5", "h6", "h7", "h8", "h9", "h10", "h11", "h12",
    ];
    let mut doc = ProfileDoc::empty(
        DocumentId::derive("intent-literals-a-bound"),
        Tol::witness(),
    );
    doc = declare(&doc, "w", free(W));
    doc = declare(&doc, "v", free(W));
    let mut prev = "w";
    for name in &NAMES[..11] {
        doc = declare(
            &doc,
            name,
            VarDecl::defined(Formula::add(named(prev), named(prev)).unwrap()),
        );
        prev = name;
    }
    match try_step(
        &doc,
        DocEdit::DeclareVar {
            name: n(NAMES[11]),
            def: VarDecl::defined(Formula::add(named("h11"), named("h11")).unwrap()),
        },
    ) {
        Err(EditError::DefinitionTooLarge { nodes, .. }) => {
            assert_eq!(nodes, editor_core::DEFINITION_NODE_BOUND + 1, "saturated");
        }
        other => panic!("8191 nodes outgrow the bound, got {other:?}"),
    }
    match try_step(
        &doc,
        DocEdit::DefineVar {
            var: n("w").into(),
            def: VarDecl::defined(Formula::add(named("v"), named("v")).unwrap()),
            fresh: Vec::new(),
        },
    ) {
        Err(EditError::DefinitionTooLarge { var, .. }) => {
            assert_eq!(var.id(), id(&doc, "h11"), "h10 holds 4095 nodes, h11 8191");
        }
        other => panic!("growing the chain from below refuses, got {other:?}"),
    }
}

// ---------------------------------------------------- the lifecycle

/// An anonymous variable read only by an anonymous definition goes with
/// it: the edit that detaches the definition's last reader removes the
/// defined variable, then its input — and the mint log keeps both ids.
#[test]
fn the_anonymous_lifecycle_cascades_through_definitions() {
    let doc = w_and_h("intent-literals-a-cascade");
    let (w, h) = (id(&doc, "w"), id(&doc, "h"));
    let (doc, [_, _, extrude]) = block(doc, 0.0, named("h"));
    let unname = |doc: &ProfileDoc, var: VarId| {
        step(
            doc,
            DocEdit::RenameVar {
                var: var.into(),
                name: None,
            },
        )
        .doc
    };
    let doc = unname(&doc, h);
    // `w` is read by `h`'s definition, and `h` by a slot: both live.
    let doc = unname(&doc, w);
    assert!(doc.var(w).is_some() && doc.var(h).is_some());
    assert_eq!(doc.var_readers(w), vec![extrude], "a reader of h reads w");
    let applied = step(
        &doc,
        DocEdit::SetParam {
            node: extrude,
            slot: editor_core::SlotId::Distance,
            value: len(1.0).into(),
            fresh: Vec::new(),
        },
    );
    let removed: Vec<VarId> = applied
        .maintenance
        .iter()
        .map(|m| match m {
            Maintenance::AnonymousVarRemoved { var, .. } => var.id(),
            other => panic!("{other:?}"),
        })
        .collect();
    assert_eq!(removed, vec![h, w], "the definition, then its input");
    assert!(applied.doc.var(h).is_none() && applied.doc.var(w).is_none());
    assert!(applied.doc.has_minted_var(h) && applied.doc.has_minted_var(w));
}

/// A variable read only by a NAMED definition nothing reads is live:
/// its name may go.
#[test]
fn a_variable_read_by_a_named_definition_may_be_anonymous() {
    let doc = w_and_h("intent-literals-a-live");
    let applied = step(
        &doc,
        DocEdit::RenameVar {
            var: n("w").into(),
            name: None,
        },
    );
    assert!(applied.maintenance.is_empty(), "{:?}", applied.maintenance);
    assert_eq!(applied.doc.var_name(id(&doc, "w")), None);
    assert_eq!(applied.doc.vars().len(), 2);
}

/// A definition whose input was deleted refuses at each reader, wrapped
/// with the reader's slot.
#[test]
fn a_definition_reading_a_deleted_variable_refuses_at_its_readers() {
    let doc = w_and_h("intent-literals-a-refused");
    let (w, h) = (id(&doc, "w"), id(&doc, "h"));
    let (doc, [_, _, extrude]) = block(doc, 0.0, named("h"));
    let doc = step(&doc, DocEdit::DeleteVar { var: n("w").into() }).doc;
    let ev = eval_after(&doc, None);
    let Some(NodeResult::Failed(err)) = ev.nodes.get(&extrude) else {
        panic!("the reader of h refuses: {:?}", ev.nodes.get(&extrude))
    };
    match &err.kind {
        NodeErrorKind::Expr { source, .. } => assert_eq!(
            *source,
            EvalError::DefinitionRefused {
                var: h,
                source: Box::new(EvalError::UnresolvedVar { var: w }),
            }
        ),
        other => panic!("{other:?}"),
    }
}

// --------------------------------------------------- the environment

/// A definition written before its input was declared evaluates after
/// it: the environment binds in definition order, not declaration
/// order.
#[test]
fn definitions_bind_after_what_they_read() {
    let doc = ProfileDoc::empty(
        DocumentId::derive("intent-literals-a-order"),
        Tol::witness(),
    );
    let doc = declare(&doc, "h", free(1.0));
    let doc = declare(&doc, "w", free(W));
    let doc = step(
        &doc,
        DocEdit::DefineVar {
            var: n("h").into(),
            def: VarDecl::defined(times(2, "w")),
            fresh: Vec::new(),
        },
    )
    .doc;
    let (w, h) = (id(&doc, "w"), id(&doc, "h"));
    assert_eq!(doc.var_ids(), &[h, w]);
    assert_eq!(doc.definition_order(), vec![w, h]);
    let env = doc.var_env::<f64>();
    assert_eq!(
        env.bindings.get(&h),
        Some(&ParamValue::Continuous {
            dim: Dimension::Length,
            value: 2.0 * W,
        })
    );
}

/// Over a box, a defined variable binds its definition over the widened
/// input: `h := 2·w` spans exactly twice `w`'s enclosure.
#[test]
fn a_defined_variable_carries_its_inputs_enclosure() {
    let doc = w_and_h("intent-literals-a-box");
    let doc = step(
        &doc,
        DocEdit::SetVarDistribution {
            var: n("w").into(),
            distribution: Some(Distribution::Normal {
                sigma: 1.0 / 1024.0,
            }),
        },
    )
    .doc;
    let (w, h) = (id(&doc, "w"), id(&doc, "h"));
    let boxed = ParamBox::of(&analyzed_box(&doc, &AnalysisPolicy::default()));
    assert_eq!(
        boxed.axes().keys().copied().collect::<Vec<_>>(),
        vec![w],
        "only the free variable is an axis"
    );
    let env = var_env_over::<Interval, _>(&doc, &boxed).expect("the box binds");
    let span = |var| match env.bindings.get(&var) {
        Some(ParamValue::Continuous { value, .. }) => (value.lo(), value.hi()),
        other => panic!("{other:?}"),
    };
    let ((wl, wh), (hl, hh)) = (span(w), span(h));
    assert!(wl < wh, "w varies");
    assert_eq!((hl, hh), (2.0 * wl, 2.0 * wh));
}

// ------------------------------------------------------ split, inline

/// A cut node reading `h := 2·w` carries both into the part, `h`
/// defined over the part's own `w`; the part loads, and inlines back.
#[test]
fn split_and_inline_carry_definitions() {
    let doc = ProfileDoc::empty(
        DocumentId::derive("intent-literals-a-split"),
        Tol::witness(),
    );
    let doc = declare(&doc, "kept", free(1.0));
    let doc = declare(&doc, "h", free(1.0));
    let doc = declare(&doc, "w", free(W));
    // `h` was declared before `w`, so the part must declare in
    // definition order to declare `h` at all.
    let doc = step(
        &doc,
        DocEdit::DefineVar {
            var: n("h").into(),
            def: VarDecl::defined(times(2, "w")),
            fresh: Vec::new(),
        },
    )
    .doc;
    let (doc, cut) = block(doc, 0.0, named("h"));
    let (doc, _) = block(doc, 10.0, named("kept"));
    let out = split(
        &doc,
        &BTreeSet::from(cut),
        DocumentId::derive("intent-literals-a-part"),
        Tol::witness(),
        None,
    )
    .expect("the cut alone reads h, and through it w");
    let (part_w, part_h) = (id(&out.part, "w"), id(&out.part, "h"));
    assert_eq!(
        out.part.var(part_h).and_then(|v| v.def().defined()),
        Some(
            &editor_core::Expr::mul(
                editor_core::Expr::ratio(2, 1).unwrap(),
                editor_core::Expr::var(part_w, Dimension::Length)
            )
            .unwrap()
        ),
        "h reads the part's w"
    );
    let text = save(&out.part, &[], Tol::witness()).expect("the part saves");
    assert!(load(&text, Tol::witness()).unwrap().doc.bit_eq(&out.part));

    let mut store = PartStore::default();
    let doc_ref = store.insert(out.part.clone(), Tol::witness());
    let host = ProfileDoc::empty(DocumentId::derive("intent-literals-a-host"), Tol::witness());
    let (host, instance) = insert(host, Node::instantiate_part(doc_ref));
    let inlined = inline(
        &host,
        instance,
        &(Arc::new(store) as Arc<dyn editor_core::PartResolver>),
        Tol::witness(),
    )
    .expect("the part inlines");
    let (host_w, host_h) = (id(&inlined.doc, "w"), id(&inlined.doc, "h"));
    assert_eq!(
        inlined.doc.var(host_h).and_then(|v| v.def().defined()),
        Some(
            &editor_core::Expr::mul(
                editor_core::Expr::ratio(2, 1).unwrap(),
                editor_core::Expr::var(host_w, Dimension::Length)
            )
            .unwrap()
        )
    );
    assert!(failures(&eval_after(&inlined.doc, None)).is_empty());
}

// ------------------------------------------------------ persistence

/// A defined variable round-trips through the snapshot and through a
/// replayed log, its log entry read by name and lowered on replay.
#[test]
fn a_definition_round_trips_through_snapshot_and_log() {
    let doc = w_and_h("intent-literals-a-round-trip");
    let text = save(&doc, &[], Tol::witness()).unwrap();
    assert!(load(&text, Tol::witness()).unwrap().doc.bit_eq(&doc));

    let mut r = Recorder::new();
    r.push(DocEdit::DeclareVar {
        name: n("w"),
        def: free(W),
    });
    r.push(DocEdit::DeclareVar {
        name: n("h"),
        def: VarDecl::defined(times(2, "w")),
    });
    let empty = ProfileDoc::empty_derived("mod", Tol::witness());
    let text = save(&empty, &r.edits, Tol::witness()).unwrap();
    let loaded = load(&text, Tol::witness()).unwrap();
    assert!(loaded.doc.bit_eq(&r.doc), "the log replays to the document");
}

/// A definition no door wrote refuses at load: one holding a name is
/// unreadable, and the definition walk refuses one reading an id never
/// minted or a live variable at another kind.
#[test]
fn a_definition_no_door_wrote_refuses_at_load() {
    let doc = w_and_h("intent-literals-a-load-reads");
    let (w, h) = (id(&doc, "w"), id(&doc, "h"));
    let def = |snap: &mut serde_json::Value, wire: serde_json::Value| {
        snap["vars"][h.0.to_string()]["def"] = serde_json::json!({ "Defined": wire });
    };
    let err = load_doctored(&doc, |snap| {
        def(
            snap,
            serde_json::json!({ "Name": { "name": "w", "dim": "Length" } }),
        );
    });
    // The stored expression has no name leaf, so the wire refuses the
    // variant itself.
    assert!(
        matches!(&err, PersistError::Unreadable { detail, .. } if detail.contains("unknown variant `Name`")),
        "{err:?}"
    );
    let err = load_doctored(&doc, |snap| {
        def(snap, wire_var(VarId::new(0, 0x5eed), "Length"))
    });
    assert!(
        matches!(&err, PersistError::Snapshot(SnapshotError::DefinitionReadsUnmintedVar { var, read })
            if var.id() == h && *read == VarId::new(0, 0x5eed)),
        "{err:?}"
    );
    let err = load_doctored(&doc, |snap| {
        def(snap, wire_var(w, "Angle"));
        snap["vars"][h.0.to_string()]["kind"] = serde_json::json!("Angle");
    });
    assert!(
        matches!(&err, PersistError::Snapshot(SnapshotError::DefinitionVarKind { read, declared, referenced, .. })
            if read.id() == w && *declared == editor_core::VarKind::Length && *referenced == Dimension::Angle),
        "{err:?}"
    );
    // A stored kind its definition does not hold is the kind walk's.
    let err = load_doctored(&doc, |snap| {
        snap["vars"][h.0.to_string()]["kind"] = serde_json::json!("Angle");
    });
    assert!(
        matches!(&err, PersistError::Snapshot(SnapshotError::VarKind { var, .. }) if var.id() == h),
        "{err:?}"
    );
}

/// A file whose expansion outgrows the bound refuses at load, naming
/// the first variable past it.
#[test]
fn a_file_past_the_expansion_bound_refuses_at_load() {
    let mut doc = ProfileDoc::empty(
        DocumentId::derive("intent-literals-a-load-bound"),
        Tol::witness(),
    );
    doc = declare(&doc, "w", free(W));
    doc = declare(&doc, "v", free(W));
    let names = [
        "h1", "h2", "h3", "h4", "h5", "h6", "h7", "h8", "h9", "h10", "h11",
    ];
    let mut prev = "w";
    for name in names {
        doc = declare(
            &doc,
            name,
            VarDecl::defined(Formula::add(named(prev), named(prev)).unwrap()),
        );
        prev = name;
    }
    let (w, v) = (id(&doc, "w"), id(&doc, "v"));
    let err = load_doctored(&doc, |snap| {
        snap["vars"][w.0.to_string()]["def"] = serde_json::json!({
            "Defined": { "Add": [wire_var(v, "Length"), wire_var(v, "Length")] }
        });
    });
    let PersistError::Snapshot(SnapshotError::DefinitionTooLarge { var, nodes }) = err else {
        panic!("not DefinitionTooLarge: {err:?}")
    };
    assert_eq!(var.id(), id(&doc, "h11"));
    assert_eq!(nodes, editor_core::DEFINITION_NODE_BOUND + 1);
}

/// The load door's anonymous walk reads the same cascade: an input read
/// only by a named definition loads unnamed, and a definition nothing
/// reads refuses once it is unnamed too, naming the definition first.
#[test]
fn the_load_door_reads_liveness_through_definitions() {
    let doc = w_and_h("intent-literals-a-load-live");
    let (w, h) = (id(&doc, "w"), id(&doc, "h"));
    let text = save(&doc, &[], Tol::witness()).unwrap();
    let unnamed_w = crate::wire::doctored(&text, |wire| {
        let names = wire["snapshot"]["var_names"].as_object_mut().unwrap();
        names.remove(&w.0.to_string()).expect("w had a name");
    });
    load(&unnamed_w, Tol::witness()).expect("w is read by h's definition, and h is named");
    let err = load_doctored(&doc, |snap| {
        snap.as_object_mut().unwrap().remove("var_names");
    });
    let PersistError::Snapshot(SnapshotError::AnonymousVarUnread { var }) = err else {
        panic!("not AnonymousVarUnread: {err:?}")
    };
    assert_eq!(var.id(), h, "the definition is the first to go");
}

// ------------------------------------------------ the review's rows

/// A slot reading `g := h`, `h := 2·w` lowers to the token a slot
/// spelling `2·w` writes: the expansion goes all the way down, not one
/// definition deep.
#[test]
fn a_nested_definition_lowers_as_its_whole_expansion() {
    let doc = w_and_h("intent-literals-a-nested-token");
    let doc = declare(&doc, "g", VarDecl::defined(named("h")));
    let (doc, by_g) = filleted(doc, 0.0, named("g"));
    let (doc, by_formula) = filleted(doc, 4.0, times(2, "w"));
    let ev = eval_after(&doc, None);
    assert!(failures(&ev).is_empty(), "{:?}", failures(&ev));
    assert_eq!(
        radius_token(body_of(&ev, by_g)),
        radius_token(body_of(&ev, by_formula))
    );
}

/// A variable declared BEFORE the chain it is redefined to read is
/// counted after that chain: `x := c10 + c10`, each `cₖ` doubling the
/// one before, expands to 8191 nodes and refuses at `x` — the size of a
/// definition is taken in definition order, not declaration order.
#[test]
fn the_bound_counts_a_reader_declared_before_what_it_reads() {
    const CHAIN: [&str; 11] = [
        "c0", "c1", "c2", "c3", "c4", "c5", "c6", "c7", "c8", "c9", "c10",
    ];
    let doc = ProfileDoc::empty(
        DocumentId::derive("intent-literals-a-early-reader"),
        Tol::witness(),
    );
    let doc = declare(&doc, "x", free(1.0));
    let doc = declare(&doc, "w", free(W));
    let mut doc = declare(
        &doc,
        CHAIN[0],
        VarDecl::defined(Formula::add(named("w"), named("w")).unwrap()),
    );
    for pair in CHAIN.windows(2) {
        doc = declare(
            &doc,
            pair[1],
            VarDecl::defined(Formula::add(named(pair[0]), named(pair[0])).unwrap()),
        );
    }
    match try_step(
        &doc,
        DocEdit::DefineVar {
            var: n("x").into(),
            def: VarDecl::defined(Formula::add(named("c10"), named("c10")).unwrap()),
            fresh: Vec::new(),
        },
    ) {
        Err(EditError::DefinitionTooLarge { var, nodes }) => {
            assert_eq!((var.id(), nodes), (id(&doc, "x"), 4097));
        }
        other => panic!("{:?}", other.map(|applied| applied.doc.len())),
    }
}

/// `g := 2·h` declared first, `h := w + w`, and `w` deleted: `h`
/// is still ordered before `g` though its read is dead, so `g` refuses
/// with the refusal it came through, not as a read of an unbound `h`.
#[test]
fn a_refusal_through_a_dead_read_names_the_definition_it_came_through() {
    let doc = ProfileDoc::empty(
        DocumentId::derive("intent-literals-a-dead-read"),
        Tol::witness(),
    );
    let doc = declare(&doc, "g", free(1.0));
    let doc = declare(&doc, "h", free(1.0));
    let doc = declare(&doc, "w", free(W));
    let (g, h, w) = (id(&doc, "g"), id(&doc, "h"), id(&doc, "w"));
    let doc = step(
        &doc,
        DocEdit::DefineVar {
            var: n("g").into(),
            def: VarDecl::defined(times(2, "h")),
            fresh: Vec::new(),
        },
    )
    .doc;
    let doc = step(
        &doc,
        DocEdit::DefineVar {
            var: n("h").into(),
            def: VarDecl::defined(Formula::add(named("w"), named("w")).unwrap()),
            fresh: Vec::new(),
        },
    )
    .doc;
    let doc = step(&doc, DocEdit::DeleteVar { var: n("w").into() }).doc;
    assert_eq!(doc.definition_order(), vec![h, g]);
    assert_eq!(
        doc.var_env::<f64>().refused.get(&g),
        Some(&EvalError::DefinitionRefused {
            var: h,
            source: Box::new(EvalError::UnresolvedVar { var: w }),
        })
    );
}

/// A part holding `w` and `h := w + w`, reading `h`, published to a
/// store: the part, the store and its reference.
fn part_defining_h(seed: &str) -> (ProfileDoc, PartStore, editor_core::DocRef) {
    let part = ProfileDoc::empty(DocumentId::derive(seed), Tol::witness());
    let part = declare(&part, "w", free(W));
    let part = declare(
        &part,
        "h",
        VarDecl::defined(Formula::add(named("w"), named("w")).unwrap()),
    );
    let (part, _) = block(part, 0.0, named("h"));
    let mut store = PartStore::default();
    let doc_ref = store.insert(part.clone(), Tol::witness());
    (part, store, doc_ref)
}

/// Inlining into a host that already holds the part's `w` and
/// `h := w + w` under the same names, bit for bit, refuses at `w`: the
/// equal definitions are two variables, never merged.
#[test]
fn inline_refuses_a_definition_the_host_already_holds() {
    let (_, store, doc_ref) = part_defining_h("intent-literals-a-shared-part");
    let host = ProfileDoc::empty(
        DocumentId::derive("intent-literals-a-shared-host"),
        Tol::witness(),
    );
    let host = declare(&host, "w", free(W));
    let host = declare(
        &host,
        "h",
        VarDecl::defined(Formula::add(named("w"), named("w")).unwrap()),
    );
    let (host, instance) = insert(host, Node::instantiate_part(doc_ref));
    match inline(
        &host,
        instance,
        &(Arc::new(store) as Arc<dyn editor_core::PartResolver>),
        Tol::witness(),
    ) {
        Err(InlineError::VarNameConflict { name }) => assert_eq!(name, n("w")),
        other => panic!("expected VarNameConflict on w, got {other:?}"),
    }
}

/// Inlining into a host holding neither name carries `w` and `h` as
/// ids the host mints, and the carried `h` reads the carried `w`, not
/// the part's.
#[test]
fn inline_carries_a_definition_at_the_carried_ids() {
    let (part, store, doc_ref) = part_defining_h("intent-literals-a-carried-part");
    let host = ProfileDoc::empty(
        DocumentId::derive("intent-literals-a-carried-host"),
        Tol::witness(),
    );
    let host = declare(&host, "z", free(1.0));
    let (host, instance) = insert(host, Node::instantiate_part(doc_ref));
    let inlined = inline(
        &host,
        instance,
        &(Arc::new(store) as Arc<dyn editor_core::PartResolver>),
        Tol::witness(),
    )
    .expect("no name clashes");
    let (w, h) = (id(&inlined.doc, "w"), id(&inlined.doc, "h"));
    assert_ne!(w, id(&part, "w"), "the host mints its own w");
    assert_ne!(h, id(&part, "h"), "and its own h");
    let mut reads = Vec::new();
    inlined
        .doc
        .var(h)
        .and_then(|held| held.def().defined())
        .expect("h stays defined")
        .var_reads(&mut reads);
    assert_eq!(
        reads.iter().map(|(read, _)| *read).collect::<Vec<_>>(),
        vec![w, w],
        "the carried h reads the carried w"
    );
    assert!(failures(&eval_after(&inlined.doc, None)).is_empty());
}

/// Monte Carlo over a document holding `h := 2·w`, `w` toleranced, and
/// a measure whose value is each: every draw binds `h` from that draw's
/// `w`, so `h`'s summary is exactly twice `w`'s (doubling is exact in
/// binary), and no draw goes unmeasured.
#[test]
fn monte_carlo_binds_a_definition_in_every_draw() {
    let doc = w_and_h("intent-literals-a-mc");
    let doc = step(
        &doc,
        DocEdit::SetVarDistribution {
            var: n("w").into(),
            distribution: Some(Distribution::Normal { sigma: 0.001 }),
        },
    )
    .doc;
    // A measure whose value is each variable's own.
    let mut r = crate::fixture::Recorder {
        doc,
        edits: Vec::new(),
    };
    for name in ["w", "h"] {
        r.measure_of_translation(name);
    }
    let doc = r.doc;
    let config = editor_core::mc::McConfig {
        samples: 64,
        ..editor_core::mc::McConfig::default()
    };
    let report = editor_core::mc::monte_carlo(
        &doc,
        &analyzed_box(&doc, &AnalysisPolicy::default()),
        &config,
        Tol::witness(),
    )
    .expect("a normal law samples");
    let [w, h] = report.measures.as_slice() else {
        panic!("two measures: {:?}", report.measures)
    };
    assert_eq!((w.measured, h.measured), (64, 64));
    assert!(w.sigma > 0.0, "w varies: {w:?}");
    assert_eq!(
        (h.mean, h.sigma, h.min, h.max),
        (2.0 * w.mean, 2.0 * w.sigma, 2.0 * w.min, 2.0 * w.max)
    );
}

/// The symbolic tier binds a definition over its inputs' symbols:
/// with `h := w + w`, `h − 2·w` is a theorem, decided Zero by the
/// symbolic tier and not by a number.
#[test]
fn the_symbolic_tier_reads_a_definition_through_its_inputs_symbols() {
    let doc = ProfileDoc::empty(
        DocumentId::derive("intent-literals-a-symbolic"),
        Tol::witness(),
    );
    let doc = declare(&doc, "w", free(W));
    let doc = declare(
        &doc,
        "h",
        VarDecl::defined(Formula::add(named("w"), named("w")).unwrap()),
    );
    let (w, h) = (id(&doc, "w"), id(&doc, "h"));
    let leaf = ParamBox::from_axes(std::collections::BTreeMap::new());
    let session = |decide_it: bool| {
        geom_core::sym::with_session_rules(
            SymBudget {
                max_terms: 4096,
                max_degree: 128,
            },
            SymRules::shipped(),
            || {
                let env = var_env_over::<Sym<f64>, _>(&doc, &leaf).unwrap();
                let bound = |var| match env.bindings[&var] {
                    ParamValue::Continuous { value, .. } => value,
                    ref other => panic!("{var}: {other:?}"),
                };
                decide_it.then(|| {
                    geom_core::k_stats::decide(
                        "intent_literals_a",
                        Margin::of(bound(h) - Sym::<f64>::from_f64(2.0) * bound(w)),
                        Band::new(1.0e-9, 1.0e-8).unwrap(),
                    )
                })
            },
        )
    };
    let (_, binding) = session(false);
    let (decided, counts) = session(true);
    assert_eq!(decided, Some(Ok(Sign::Zero)));
    assert_eq!(
        counts.symbolic_zero,
        binding.symbolic_zero + 1,
        "h − 2·w is a theorem"
    );
}
