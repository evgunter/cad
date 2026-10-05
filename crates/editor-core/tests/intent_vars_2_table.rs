//! **The variable table, keyed by minted id** — INTENT-VARS-1 PR 2's
//! rows of the spec's §4 test plan (`docs/doc-ledger/intent-vars-1-spec.md`).
//!
//! Rows here: 3 (a declared name is unique, at the door and at load),
//! 4's document half (two declares of one definition are two
//! variables; the name is not in the mint), 6 (the symbol is keyed by
//! id), 8 (a kind is fixed), 10's persistence half (the `var` mint arm
//! round-trips; an old-format `params` file is unreadable, naming it)
//! and 11 (stackup, MC and `ParamBox` are keyed by `VarId`). Row 4's
//! collision half is `mint::tests::a_var_id_the_log_holds_refuses_and_
//! moves_nothing`, where the log can be written by hand. The rename and
//! delete halves arrive with `RenameVar` and `DeleteVar`.
//!
//! Every row reads TWO variables at EQUAL values (D10: equal values are
//! not one variable), so a table, a symbol or an analysis that keyed by
//! value would merge them and go red.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeMap;

use editor_core::analysis::{AnalysisPolicy, analyzed_box};
use editor_core::mc::{McConfig, sample_offsets};
use editor_core::persist::SnapshotError;
use editor_core::stackup::{SensitivityOutcome, sensitivities};
use editor_core::{
    Dimension, Distribution, DocEdit, EditError, Expr, FreeValue, FreeVar, MeasureExpr, Node,
    ParamBox, PersistError, ProfileDoc, ProfileProgram, RecipeNodeId, UnitSym, VarDef, VarId,
    VarKind, VarName, apply, load, save, var_env_over,
};
use geom_core::Tol;
use geom_core::predicate::{Band, Margin, Sign};
use geom_core::{ParamSymbol, Real, Sym, SymBudget, SymRules};

const VALUE: f64 = 0.005;

fn n(name: &'static str) -> VarName {
    VarName::from_static(name)
}

/// A length of `VALUE` metres, annotated so the analysis varies it.
fn law_def() -> VarDef {
    VarDef::Free(FreeVar::Continuous {
        dim: Dimension::Length,
        value: VALUE,
        display_unit: UnitSym::canonical_for(Dimension::Length),
        distribution: Some(Distribution::Normal { sigma: 1e-4 }),
    })
}

fn step(doc: &ProfileDoc, edit: DocEdit<ProfileProgram>) -> Result<ProfileDoc, EditError> {
    apply(doc, &edit, Tol::witness(), &editor_core::RefusingReach).map(|a| a.doc)
}

fn declare(doc: &ProfileDoc, name: &'static str, def: VarDef) -> ProfileDoc {
    step(doc, DocEdit::DeclareVar { name: n(name), def }).expect("the declare applies")
}

/// `w` and `v`, both `VALUE` under one law.
fn twins() -> ProfileDoc {
    let doc = ProfileDoc::empty_derived("intent-vars-2-twins", Tol::witness());
    let doc = declare(&doc, "w", law_def());
    declare(&doc, "v", law_def())
}

fn id(doc: &ProfileDoc, name: &str) -> VarId {
    doc.var_named(name).expect("declared")
}

/// The twins and a measure of `w + 2·v`, whose partials (1 and 2) tell
/// the two variables apart.
fn measured_twins() -> (ProfileDoc, RecipeNodeId) {
    let doc = twins();
    let w = Expr::named(n("w"), Dimension::Length);
    let v = Expr::named(n("v"), Dimension::Length);
    let two = Expr::literal(2.0, Dimension::Scalar).unwrap();
    let sum = Expr::add(w, Expr::mul(two, v).unwrap()).unwrap();
    let applied = apply(
        &doc,
        &DocEdit::InsertNode {
            node: Box::new(Node::measure(MeasureExpr::value(sum), Vec::new()).unwrap()),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    )
    .expect("the measure inserts");
    let measure = applied.record.minted.expect("an insert mints");
    (applied.doc, measure)
}

// ------------------------------------------------------------ row 3/4

/// Row 3: a taken name refuses, naming the HOLDER by its id, and the
/// document is untouched.
#[test]
fn a_declare_of_a_taken_name_refuses_naming_the_holder() {
    let doc = twins();
    let err = step(
        &doc,
        DocEdit::DeclareVar {
            name: n("w"),
            def: VarDef::Free(FreeVar::Count { value: 3 }),
        },
    )
    .unwrap_err();
    let EditError::VarNameTaken { name, holder } = err else {
        panic!("not VarNameTaken: {err:?}")
    };
    assert_eq!(name, n("w"));
    assert_eq!(holder.id(), id(&doc, "w"), "the holder is w, not v");
    assert_ne!(holder.id(), id(&doc, "v"));
}

/// Row 4's document half and ruling Q5: one definition declared twice
/// is two variables, and the name is not in the mint — the same
/// declare under another name, from the same document, mints the same
/// id.
#[test]
fn equal_definitions_are_two_variables_and_the_name_is_not_minted() {
    let doc = twins();
    let (w, v) = (id(&doc, "w"), id(&doc, "v"));
    assert_ne!(w, v, "equal values are not one variable");
    assert!(doc.mint().has_var(w) && doc.mint().has_var(v));
    assert_eq!(doc.mint().vars().count(), 2);
    assert!(doc.var(w).unwrap().bit_eq(doc.var(v).unwrap()));

    let empty = ProfileDoc::empty_derived("intent-vars-2-twins", Tol::witness());
    let other = declare(&empty, "some_other_name", law_def());
    assert_eq!(
        other.var_named("some_other_name"),
        Some(w),
        "the name is not in the declare's preimage (VR2)"
    );
    let moved = declare(
        &empty,
        "w",
        VarDef::Free(FreeVar::continuous(Dimension::Length, 2.0 * VALUE)),
    );
    assert_eq!(
        moved.var_named("w"),
        Some(w),
        "nor is the definition: another value and no annotation mint the same id"
    );
    let angle = declare(
        &empty,
        "w",
        VarDef::Free(FreeVar::continuous(Dimension::Angle, VALUE)),
    );
    assert_ne!(angle.var_named("w"), Some(w), "the kind is");
}

/// Row 3's load half: a file naming two ids `w` refuses `VarNameTwice`
/// with both ids, lower first.
#[test]
fn a_file_holding_one_name_twice_refuses() {
    let doc = twins();
    let (w, v) = (id(&doc, "w"), id(&doc, "v"));
    let text = save(&doc, &[], Tol::witness()).unwrap();
    let from = format!("\"{}\": \"v\"", v.0);
    assert_eq!(text.matches(&from).count(), 1, "v's name row: {text}");
    let bad = text.replace(&from, &format!("\"{}\": \"w\"", v.0));
    let err = load(&bad, Tol::witness()).unwrap_err();
    let (a, b) = if w < v { (w, v) } else { (v, w) };
    assert_eq!(
        err,
        PersistError::Snapshot(SnapshotError::VarNameTwice { name: n("w"), a, b })
    );
}

// --------------------------------------------------------------- row 6

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

fn decide(m: Sym<f64>) -> Result<Sign, geom_core::predicate::Indeterminate> {
    geom_core::k_stats::decide(
        "intent_vars_2",
        Margin::of(m),
        Band::new(1.0e-9, 1.0e-8).unwrap(),
    )
}

/// The value `env` binds `var` to.
fn bound(var: VarId, env: &editor_core::VarEnv<Sym<f64>>) -> Sym<f64> {
    match env.bindings[&var] {
        editor_core::ParamValue::Continuous { value, .. } => value,
        ref other => panic!("{var}: {other:?}"),
    }
}

/// Row 6: the symbolic tier reads a variable as its id's symbol, so
/// `w − w` is a theorem, `w − v` at equal values is not, and the
/// symbol is `ParamSymbol::new(id)` exactly.
#[test]
fn the_symbol_is_the_variables_id() {
    let doc = twins();
    let (w_id, v_id) = (id(&doc, "w"), id(&doc, "v"));
    let leaf = ParamBox::from_axes(BTreeMap::new());
    let (same, counts) = session(|| {
        let env = var_env_over::<Sym<f64>, _>(&doc, &leaf).unwrap();
        let w = bound(w_id, &env);
        decide(w - w)
    });
    assert_eq!(same, Ok(Sign::Zero));
    assert_eq!(counts.symbolic_zero, 1, "w − w is a theorem");

    let (_, counts) = session(|| {
        let env = var_env_over::<Sym<f64>, _>(&doc, &leaf).unwrap();
        decide(bound(w_id, &env) - bound(v_id, &env))
    });
    assert_eq!(
        counts.symbolic_zero, 0,
        "w − v at equal values is not a theorem: two symbols"
    );

    for (name, var) in [("w", w_id), ("v", v_id)] {
        let (_, counts) = session(|| {
            let env = var_env_over::<Sym<f64>, _>(&doc, &leaf).unwrap();
            let by_hand = Sym::<f64>::from_f64(VALUE)
                + Sym::param_over(ParamSymbol::new(var.0), 0.0, 0.0, 0.0);
            decide(bound(var, &env) - by_hand)
        });
        assert_eq!(counts.symbolic_zero, 1, "{name}'s symbol is its id");
    }
}

// --------------------------------------------------------------- row 8

/// Row 8: a kind is fixed when a variable is declared. A value of
/// another kind refuses at `SetVarValue`, a definition of another kind
/// refuses `VarKindFixed`, and neither moves the document.
#[test]
fn a_kind_is_fixed() {
    let doc = twins();
    let w = id(&doc, "w");
    let err = step(
        &doc,
        DocEdit::SetVarValue {
            var: n("w").into(),
            value: FreeValue::Count(3),
        },
    )
    .unwrap_err();
    assert!(
        matches!(&err, EditError::VarValueKindMismatch { var, .. } if var.id() == w),
        "{err:?}"
    );
    let err = step(
        &doc,
        DocEdit::DefineVar {
            var: w.into(),
            def: VarDef::Free(FreeVar::Count { value: 3 }),
        },
    )
    .unwrap_err();
    let EditError::VarKindFixed { var, kind, offered } = err else {
        panic!("not VarKindFixed: {err:?}")
    };
    assert_eq!(
        (var.id(), kind, offered),
        (w, VarKind::Length, VarKind::Count)
    );
    let angle = VarDef::Free(FreeVar::continuous(Dimension::Angle, VALUE));
    assert!(matches!(
        step(
            &doc,
            DocEdit::DefineVar {
                var: w.into(),
                def: angle
            }
        ),
        Err(EditError::VarKindFixed {
            kind: VarKind::Length,
            offered: VarKind::Angle,
            ..
        })
    ));
    let same_kind = VarDef::Free(FreeVar::continuous(Dimension::Length, 0.007));
    let defined = step(
        &doc,
        DocEdit::DefineVar {
            var: w.into(),
            def: same_kind.clone(),
        },
    )
    .expect("a definition of the variable's kind applies");
    assert_eq!(defined.var_named("w"), Some(w), "and keeps the id");
    assert!(defined.var(w).unwrap().def().bit_eq(&same_kind));
}

// -------------------------------------------------------------- row 10

/// Row 10: a save carries each declare as a `var` mint entry and the
/// table by id, and loads back `bit_eq`.
#[test]
fn the_table_round_trips_with_its_var_mint_arm() {
    let doc = twins();
    let text = save(&doc, &[], Tol::witness()).unwrap();
    for name in ["w", "v"] {
        let tag = format!("\"var\": {}", id(&doc, name).0);
        assert_eq!(text.matches(&tag).count(), 1, "{name}'s mint entry");
    }
    let back = load(&text, Tol::witness()).unwrap().doc;
    assert!(back.bit_eq(&doc), "round trip");
    assert_eq!(back.var_named("w"), doc.var_named("w"));
    assert_eq!(back.var_named("v"), doc.var_named("v"));
}

/// Row 10: a file in the format before the table — a `params` field —
/// is unreadable, and the refusal names the field.
#[test]
fn an_old_format_params_file_is_unreadable_naming_params() {
    let text = save(&twins(), &[], Tol::witness()).unwrap();
    assert_eq!(text.matches("\"vars\": {").count(), 1);
    let old = text.replace("\"vars\": {", "\"params\": {");
    match load(&old, Tol::witness()) {
        Err(PersistError::Unreadable { detail, .. }) => {
            assert!(detail.contains("unknown field `params`"), "{detail}");
        }
        other => panic!("not Unreadable: {other:?}"),
    }
}

// -------------------------------------------------------------- row 11

/// Row 11: the analysis box, the `ParamBox` the driver splits, the MC
/// draws and the stackup sensitivities are keyed by the two ids, and
/// each id carries its OWN variable's answer.
#[test]
fn analysis_is_keyed_by_var_id() {
    let (doc, measure) = measured_twins();
    let (w, v) = (id(&doc, "w"), id(&doc, "v"));
    let both = {
        let mut ids = vec![w, v];
        ids.sort();
        ids
    };
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    assert_eq!(analyzed.params().keys().copied().collect::<Vec<_>>(), both);
    assert_eq!(analyzed.spoken(w).to_string(), "w");
    let boxed = ParamBox::of(&analyzed);
    assert_eq!(boxed.axes().keys().copied().collect::<Vec<_>>(), both);

    // Different laws, so a draw handed to the wrong variable shows: w's
    // is a σ = 1e-4 normal, v's a uniform on [-2, 2].
    let lawed = step(
        &doc,
        DocEdit::SetVarDistribution {
            var: v.into(),
            distribution: Some(Distribution::Uniform { lo: -2.0, hi: 2.0 }),
        },
    )
    .unwrap();
    let lawed_box = analyzed_box(&lawed, &AnalysisPolicy::default());
    let mut widest_v = 0.0_f64;
    for i in 0..16 {
        let draws = sample_offsets(&lawed, &lawed_box, &McConfig::default(), i).unwrap();
        assert_eq!(draws.keys().copied().collect::<Vec<_>>(), both);
        assert!(draws[&w].abs() < 1e-2, "w draws its own normal: {draws:?}");
        assert!(draws[&v].abs() <= 2.0, "v its own uniform: {draws:?}");
        widest_v = widest_v.max(draws[&v].abs());
    }
    assert!(widest_v > 0.1, "v's draws span its uniform, not w's normal");

    let entries = sensitivities(&doc, measure, None, None, false, None, Tol::witness()).unwrap();
    let partial = |var: VarId| {
        let e = entries.iter().find(|e| e.param == var).expect("an entry");
        match e.outcome {
            SensitivityOutcome::Derivative { value, .. } => value,
            ref other => panic!("{other:?}"),
        }
    };
    assert_eq!(entries.len(), 2);
    assert_eq!((partial(w), partial(v)), (1.0, 2.0), "∂/∂w = 1, ∂/∂v = 2");
}

// ------------------------------------------------- the load walk's arms

/// A fresh save of the twins with `edit` applied to its snapshot, then
/// loaded: what the load door answers.
fn load_doctored(edit: impl FnOnce(&mut serde_json::Value, VarId, VarId)) -> PersistError {
    let doc = twins();
    let (w, v) = (id(&doc, "w"), id(&doc, "v"));
    let text = save(&doc, &[], Tol::witness()).unwrap();
    load(&text, Tol::witness()).expect("the undoctored save loads");
    let corrupt = crate::wire::doctored(&text, |wire| edit(&mut wire["snapshot"], w, v));
    load(&corrupt, Tol::witness()).expect_err("the doctored save must refuse")
}

/// `VarKind`: a stored kind that is not its definition's.
#[test]
fn a_kind_its_definition_does_not_hold_refuses_at_load() {
    let err = load_doctored(|snap, w, _| {
        let kind = &mut snap["vars"][w.0.to_string()]["kind"];
        assert_eq!(*kind, serde_json::json!("Length"), "aimed at w's kind");
        *kind = serde_json::json!("Angle");
    });
    let PersistError::Snapshot(SnapshotError::VarKind { var, kind, def }) = err else {
        panic!("not VarKind: {err:?}")
    };
    assert_eq!(var.name(), Some(&n("w")));
    assert_eq!((kind, def), (VarKind::Angle, VarKind::Length));
}

/// `VarNotMinted`: a variable the mint log does not hold.
#[test]
fn a_variable_the_mint_never_minted_refuses_at_load() {
    let err = load_doctored(|snap, w, _| {
        let log = snap["mint"]["log"].as_array_mut().expect("the mint log");
        let before = log.len();
        log.retain(|entry| entry != &serde_json::json!({ "var": w.0 }));
        assert_eq!(log.len(), before - 1, "w's entry was in the log");
    });
    let PersistError::Snapshot(SnapshotError::VarNotMinted { var }) = err else {
        panic!("not VarNotMinted: {err:?}")
    };
    assert_eq!(var.name(), Some(&n("w")));
}

/// `NameOnMissingVar`: a name on an id that holds no variable.
#[test]
fn a_name_on_no_variable_refuses_at_load() {
    let err = load_doctored(|snap, _, _| {
        snap["var_names"]["99"] = serde_json::json!("ghost");
    });
    assert_eq!(
        err,
        PersistError::Snapshot(SnapshotError::NameOnMissingVar {
            var: VarId(99),
            name: n("ghost"),
        })
    );
}

/// `AnonymousVarUnread`: a variable with no name that nothing reads —
/// the edit that detaches an anonymous variable's last reader removes
/// it, so the load door refuses one rather than let the lanes disagree
/// on whether it exists.
#[test]
fn an_unread_unnamed_variable_refuses_at_load() {
    let err = load_doctored(|snap, _, v| {
        let names = snap["var_names"].as_object_mut().expect("the name table");
        assert!(names.remove(&v.0.to_string()).is_some(), "v had a name");
    });
    let PersistError::Snapshot(SnapshotError::AnonymousVarUnread { var }) = err else {
        panic!("not AnonymousVarUnread: {err:?}")
    };
    let doc = twins();
    assert_eq!(var.id(), id(&doc, "v"));
    assert_eq!(var.name(), None);
}

/// `VarOrderMismatch`: a declaration order that drops a variable.
#[test]
fn a_declaration_order_missing_a_variable_refuses_at_load() {
    let err = load_doctored(|snap, w, _| {
        let order = snap["var_order"].as_array_mut().expect("the order");
        let before = order.len();
        order.retain(|id| id != &serde_json::json!(w.0));
        assert_eq!(order.len(), before - 1, "w was listed");
    });
    assert_eq!(err, PersistError::Snapshot(SnapshotError::VarOrderMismatch));
}

/// **The document's variables have ONE order, the author's**: the
/// declaration order, which every lane lists, draws and tie-breaks in.
/// The twins' ids sort AGAINST their declaration (the first declare of
/// a kind from an empty chain draws the larger id — asserted, so the
/// row cannot pass by an id order that happens to agree), and every
/// lane still says `w` first.
#[test]
fn every_lane_reads_the_declaration_order_not_the_id_order() {
    let (doc, measure) = measured_twins();
    let (w, v) = (id(&doc, "w"), id(&doc, "v"));
    assert!(w > v, "the fixture's ids sort against its declarations");
    assert_eq!(doc.var_order(), &[w, v]);
    assert_eq!(
        doc.free_vars().map(|(id, _)| id).collect::<Vec<_>>(),
        vec![w, v]
    );
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    assert_eq!(
        analyzed.varying().map(|(id, _)| id).collect::<Vec<_>>(),
        vec![w, v]
    );
    let root = ParamBox::of(&analyzed);
    assert_eq!(
        root.varying().map(|(id, _, _)| id).collect::<Vec<_>>(),
        vec![w, v]
    );
    // Equal laws, so equal relative widths: the tie goes to the
    // earlier-declared variable, never to the lower id.
    assert_eq!(root.split_axis(&root), Some(w));
    let entries = sensitivities(&doc, measure, None, None, false, None, Tol::witness()).unwrap();
    assert_eq!(
        entries.iter().map(|e| e.param).collect::<Vec<_>>(),
        vec![w, v]
    );
    // And the order survives a save.
    let text = save(&doc, &[], Tol::witness()).unwrap();
    let back = load(&text, Tol::witness()).unwrap().doc;
    assert_eq!(back.var_order(), &[w, v]);
}
