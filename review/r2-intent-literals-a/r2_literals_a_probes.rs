//! Reviewer r2's probes on PR #4065 (INTENT-LITERALS PR A).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus::failures;
use crate::fixture::{insert, len, on_frame, square};
use editor_core::analysis::{AnalysisPolicy, analyzed_box, seed_env};
use editor_core::stackup::{SensitivityOutcome, sensitivities};
use editor_core::{
    CancelToken, Dimension, Distribution, DocEdit, DocumentId, EditError, EvalOptions, Evaluation,
    Expr, ExtrudeSide, FreeValue, FreeVar, Maintenance, MeasureExpr, Node, ParamBox, ParamValue,
    ProfileDoc, ProfileProgram, RecipeNodeId, VarDecl, VarId, VarName, apply, evaluate, load, save,
    var_env_over,
};
use geom_core::{Bounds, Dual, Interval, Tol};

fn n(name: &'static str) -> VarName {
    VarName::from_static(name)
}
fn named(name: &'static str) -> Expr {
    Expr::named(n(name), Dimension::Length)
}
fn sc(name: &'static str) -> Expr {
    Expr::named(n(name), Dimension::Scalar)
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
fn free_len(v: f64) -> VarDecl {
    VarDecl::Free(FreeVar::continuous(Dimension::Length, v))
}
fn free_sc(v: f64) -> VarDecl {
    VarDecl::Free(FreeVar::continuous(Dimension::Scalar, v))
}
fn id(doc: &ProfileDoc, name: &str) -> VarId {
    doc.var_named(name).expect("declared")
}
fn f64_of(env: &editor_core::VarEnv<f64>, v: VarId) -> f64 {
    match env.bindings.get(&v) {
        Some(ParamValue::Continuous { value, .. }) => *value,
        other => panic!("{other:?}"),
    }
}

/// A non-linear chain: s free scalar, w free length, a := w*s,
/// b := a*s + w (Length). d b / d s = 2*w*s; d b / d w = s*s + 1.
fn chain(seed: &str, s: f64, w: f64) -> ProfileDoc {
    let doc = ProfileDoc::empty(DocumentId::derive(seed), Tol::witness());
    let doc = declare(&doc, "s", free_sc(s));
    let doc = declare(&doc, "w", free_len(w));
    let doc = declare(
        &doc,
        "a",
        VarDecl::defined(Expr::mul(named("w"), sc("s")).unwrap()),
    );
    declare(
        &doc,
        "b",
        VarDecl::defined(
            Expr::add(Expr::mul(named("a"), sc("s")).unwrap(), named("w")).unwrap(),
        ),
    )
}

/// C6: stackup sensitivity of a measure reading a non-linear chain of
/// definitions, against a central finite difference of the f64 value.
#[test]
fn r2_pushforward_matches_finite_difference() {
    let (s0, w0) = (1.3, 0.07);
    let doc = chain("r2-fd", s0, w0);
    let applied = step(
        &doc,
        DocEdit::InsertNode {
            node: Box::new(Node::measure(MeasureExpr::value(named("b")), Vec::new()).unwrap()),
        },
    );
    let measure = applied.record.minted.unwrap();
    let entries = sensitivities(&applied.doc, measure, None, None, false, None, Tol::witness())
        .unwrap();
    let (s, w) = (id(&doc, "s"), id(&doc, "w"));
    assert_eq!(
        entries.iter().map(|e| e.param).collect::<Vec<_>>(),
        vec![s, w]
    );
    let b_at = |sv: f64, wv: f64| {
        let d = chain("r2-fd", sv, wv);
        f64_of(&d.var_env::<f64>(), id(&d, "b"))
    };
    let h = 1e-6;
    let fd_s = (b_at(s0 + h, w0) - b_at(s0 - h, w0)) / (2.0 * h);
    let fd_w = (b_at(s0, w0 + h) - b_at(s0, w0 - h)) / (2.0 * h);
    let got: Vec<f64> = entries
        .iter()
        .map(|e| match e.outcome {
            SensitivityOutcome::Derivative { value, .. } => value,
            ref o => panic!("{o:?}"),
        })
        .collect();
    eprintln!("got {got:?} fd [{fd_s}, {fd_w}] exact [{}, {}]", 2.0 * w0 * s0, s0 * s0 + 1.0);
    assert!((got[0] - fd_s).abs() < 1e-6, "{} vs {}", got[0], fd_s);
    assert!((got[1] - fd_w).abs() < 1e-6, "{} vs {}", got[1], fd_w);
    assert!((got[0] - 2.0 * w0 * s0).abs() < 1e-12);
    assert!((got[1] - (s0 * s0 + 1.0)).abs() < 1e-12);
}

/// C2: Dual seed on s through the chain (seed_env directly), interval
/// box enclosure, and the Sym<Interval> lane.
#[test]
fn r2_every_lane_binds_the_chain() {
    let (s0, w0) = (1.5, 0.0625);
    let doc = chain("r2-lanes", s0, w0);
    let (s, w, a, b) = (id(&doc, "s"), id(&doc, "w"), id(&doc, "a"), id(&doc, "b"));
    // Dual
    let env = seed_env::<Dual<f64>, _>(&doc, doc.var_env::<Dual<f64>>(), s).unwrap();
    match env.bindings.get(&b) {
        Some(ParamValue::Continuous { value, .. }) => {
            assert_eq!(value.value, s0 * s0 * w0 + w0);
            assert_eq!(value.deriv, 2.0 * s0 * w0);
        }
        o => panic!("{o:?}"),
    }
    // Interval box over s and w
    let doc = step(
        &doc,
        DocEdit::SetVarDistribution {
            var: n("s").into(),
            distribution: Some(Distribution::Normal { sigma: 0.01 }),
        },
    )
    .doc;
    let doc = step(
        &doc,
        DocEdit::SetVarDistribution {
            var: n("w").into(),
            distribution: Some(Distribution::Normal { sigma: 0.001 }),
        },
    )
    .doc;
    let boxed = ParamBox::of(&analyzed_box(&doc, &AnalysisPolicy::default()));
    assert_eq!(boxed.axes().keys().copied().collect::<Vec<_>>(), vec![s, w]);
    let env = var_env_over::<Interval, _>(&doc, &boxed).unwrap();
    let iv = |v| match env.bindings.get(&v) {
        Some(ParamValue::Continuous { value, .. }) => *value,
        o => panic!("{o:?}"),
    };
    let (si, wi, bi) = (iv(s), iv(w), iv(b));
    // corners
    for sv in [si.lo(), si.hi()] {
        for wv in [wi.lo(), wi.hi()] {
            let bv = sv * sv * wv + wv;
            assert!(bi.lo() <= bv && bv <= bi.hi(), "{bv} outside {bi:?}");
        }
    }
    assert!(env.bindings.contains_key(&a));
    // Sym<Interval>: the env binds b as an expression over s and w.
    let budget = geom_core::SymBudget {
        max_terms: editor_core::drive::DEFAULT_SYM_MAX_TERMS,
        max_degree: editor_core::drive::DEFAULT_SYM_MAX_DEGREE,
    };
    let (ok, _) = geom_core::sym::with_session(budget, || {
        let env = var_env_over::<geom_core::Sym<Interval>, _>(&doc, &boxed).unwrap();
        match env.bindings.get(&b) {
            Some(ParamValue::Continuous { value, .. }) => {
                let lo = value.lo();
                let hi = value.hi();
                eprintln!("sym b in [{lo}, {hi}] vs interval {bi:?}");
                lo <= s0 * s0 * w0 + w0 && s0 * s0 * w0 + w0 <= hi
            }
            _ => false,
        }
    });
    assert!(ok);
}

/// C2/C3: a count definition read by a pattern's count slot follows
/// its free input: n := m + 1, m free count.
#[test]
fn r2_a_count_definition_drives_a_pattern() {
    let doc = ProfileDoc::empty(DocumentId::derive("r2-count"), Tol::witness());
    let doc = declare(&doc, "m", VarDecl::Free(FreeVar::Count { value: 2 }));
    let doc = declare(
        &doc,
        "k",
        VarDecl::defined(
            Expr::add(Expr::named(n("m"), Dimension::Count), Expr::count(1)).unwrap(),
        ),
    );
    let (doc, profile) = on_frame(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(0.0, 0.0, 0.5)],
    );
    let (doc, cube) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(1.0),
            side: ExtrudeSide::Along,
        },
    );
    let (doc, pat) = insert(
        doc,
        Node::Pattern {
            input: cube,
            count: Expr::named(n("k"), Dimension::Count),
            kind: editor_core::PatternKind::Linear {
                direction: [
                    Expr::literal(1.0, Dimension::Scalar).unwrap(),
                    Expr::literal(0.0, Dimension::Scalar).unwrap(),
                    Expr::literal(0.0, Dimension::Scalar).unwrap(),
                ],
                spacing: len(3.0),
            },
        },
    );
    let ev = ev(&doc, None);
    assert!(failures(&ev).is_empty(), "{:?}", failures(&ev));
    let pieces = |ev: &Evaluation<f64>| {
        match &ev.value(pat).expect("a value").payload {
            editor_core::ValuePayload::Instances(v) => v.len() * 6,
            o => panic!("{}", o.kind_name()),
        }
    };
    let before = pieces(&ev);
    let edited = step(
        &doc,
        DocEdit::SetVarValue {
            var: n("m").into(),
            value: FreeValue::Count(4),
        },
    );
    assert!(edited.record.structural, "a structural edit");
    let again = ev2(&edited.doc, &ev);
    assert!(failures(&again).is_empty(), "{:?}", failures(&again));
    let after = pieces(&again);
    eprintln!("faces {before} -> {after}");
    assert_eq!(before, 3 * 6);
    assert_eq!(after, 5 * 6);
    let k = id(&doc, "k");
    assert!(doc.diff(&edited.doc).vars.contains(&k));
}

fn ev(doc: &ProfileDoc, prev: Option<&Evaluation<f64>>) -> Evaluation<f64> {
    evaluate::<f64>(doc, prev, &CancelToken::new(), &EvalOptions::default(), Tol::witness())
}
fn ev2(doc: &ProfileDoc, prev: &Evaluation<f64>) -> Evaluation<f64> {
    ev(doc, Some(prev))
}

/// C7: deleting a NAMED definition whose input is anonymous removes the
/// input (now unread); redefining it free does the same.
#[test]
fn r2_lifecycle_edges() {
    let doc = ProfileDoc::empty(DocumentId::derive("r2-life"), Tol::witness());
    let doc = declare(&doc, "w", free_len(0.25));
    let doc = declare(
        &doc,
        "h",
        VarDecl::defined(Expr::add(named("w"), named("w")).unwrap()),
    );
    let w = id(&doc, "w");
    let doc = step(
        &doc,
        DocEdit::RenameVar {
            var: n("w").into(),
            name: None,
        },
    )
    .doc;
    // delete h: w is read by nothing now.
    let del = step(&doc, DocEdit::DeleteVar { var: n("h").into() });
    let removed: Vec<_> = del
        .maintenance
        .iter()
        .filter_map(|m| match m {
            Maintenance::AnonymousVarRemoved { var } => Some(var.id()),
            _ => None,
        })
        .collect();
    assert_eq!(removed, vec![w], "{:?}", del.maintenance);
    // redefine h free: same.
    let free = step(
        &doc,
        DocEdit::DefineVar {
            var: n("h").into(),
            def: free_len(1.0),
        },
    );
    let removed: Vec<_> = free
        .maintenance
        .iter()
        .filter_map(|m| match m {
            Maintenance::AnonymousVarRemoved { var } => Some(var.id()),
            _ => None,
        })
        .collect();
    assert_eq!(removed, vec![w], "{:?}", free.maintenance);
    // Rename-to-anonymous of h (read by nothing) refuses.
    assert!(matches!(
        try_step(
            &doc,
            DocEdit::RenameVar {
                var: n("h").into(),
                name: None
            }
        ),
        Err(EditError::AnonymousVarUnread { .. })
    ));
}

/// C3: the memo — a definition_order computed on a doc, then the doc
/// round-tripped through save/load and through edits, is never stale.
#[test]
fn r2_definition_order_memo_follows_edits() {
    let doc = ProfileDoc::empty(DocumentId::derive("r2-memo"), Tol::witness());
    let doc = declare(&doc, "x", free_len(1.0));
    let doc = declare(&doc, "y", free_len(2.0));
    let (x, y) = (id(&doc, "x"), id(&doc, "y"));
    assert_eq!(doc.definition_order(), &[x, y]);
    // x := 2y flips the order
    let doc2 = step(
        &doc,
        DocEdit::DefineVar {
            var: n("x").into(),
            def: VarDecl::defined(Expr::add(named("y"), named("y")).unwrap()),
        },
    )
    .doc;
    assert_eq!(doc2.definition_order(), &[y, x]);
    assert_eq!(f64_of(&doc2.var_env(), x), 4.0);
    let cloned = doc2.clone();
    assert_eq!(cloned.definition_order(), &[y, x]);
    let text = save(&doc2, &[], Tol::witness()).unwrap();
    let loaded = load(&text, Tol::witness()).unwrap().doc;
    assert_eq!(loaded.definition_order(), &[y, x]);
    // and back: y := 3x would be a cycle; x free again restores order
    assert!(matches!(
        try_step(
            &doc2,
            DocEdit::DefineVar {
                var: n("y").into(),
                def: VarDecl::defined(Expr::add(named("x"), len(0.0)).unwrap()),
            }
        ),
        Err(EditError::DefinitionCycle { .. })
    ));
    let doc3 = step(
        &doc2,
        DocEdit::DefineVar {
            var: n("x").into(),
            def: free_len(5.0),
        },
    )
    .doc;
    assert_eq!(doc3.definition_order(), &[x, y]);
    let doc4 = step(
        &doc3,
        DocEdit::DefineVar {
            var: n("y").into(),
            def: VarDecl::defined(Expr::add(named("x"), named("x")).unwrap()),
        },
    )
    .doc;
    assert_eq!(f64_of(&doc4.var_env(), y), 10.0);
}

/// C4: a diamond of width 2 grown by redefinition from the side — not
/// from below — still refuses; and a large but legal (<= bound) chain
/// loads and its token is written.
#[test]
fn r2_bound_by_a_side_redefinition() {
    let doc = ProfileDoc::empty(DocumentId::derive("r2-side"), Tol::witness());
    let doc = declare(&doc, "w", free_len(1.0));
    let doc = declare(&doc, "z", free_len(1.0));
    const N: [&str; 12] = [
        "d0", "d1", "d2", "d3", "d4", "d5", "d6", "d7", "d8", "d9", "d10", "d11",
    ];
    let mut doc = declare(&doc, N[0], VarDecl::defined(Expr::add(named("w"), named("z")).unwrap()));
    for k in 1..11 {
        let prev = N[k - 1];
        doc = declare(
            &doc,
            N[k],
            VarDecl::defined(Expr::add(named(prev), named(prev)).unwrap()),
        );
    }
    // d10 = 2^11 * 3/2 ... sizes: d0 = 3, dk = 2*d(k-1)+1 → d10 = 4095.
    let d10 = id(&doc, "d10");
    // z := w + w makes d0 = 5 → d10 = 2^10*6-1 = 6143 > bound.
    match try_step(
        &doc,
        DocEdit::DefineVar {
            var: n("z").into(),
            def: VarDecl::defined(Expr::add(named("w"), named("w")).unwrap()),
        },
    ) {
        Err(EditError::DefinitionTooLarge { var, nodes }) => {
            assert_eq!(var.id(), d10);
            eprintln!("nodes {nodes}");
        }
        o => panic!("{:?}", o.map(|a| a.doc.len())),
    }
    let text = save(&doc, &[], Tol::witness()).unwrap();
    load(&text, Tol::witness()).expect("4095 loads");
    // a slot reading d10 three times: 3*4095 + 2 nodes, written to a token
    let (doc2, profile) = on_frame(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(0.0, 0.0, 0.5)],
    );
    let big = Expr::add(Expr::add(named("d10"), named("d10")).unwrap(), named("d10")).unwrap();
    let small = Expr::mul(Expr::literal(1e-6, Dimension::Scalar).unwrap(), big).unwrap();
    let (doc2, _) = insert(
        doc2,
        Node::Extrude {
            profile,
            distance: small,
            side: ExtrudeSide::Along,
        },
    );
    let e = ev(&doc2, None);
    assert!(failures(&e).is_empty(), "{:?}", failures(&e));
}

/// C1: a document with no definition: definition_order == var_order,
/// and var_env binds exactly the free variables.
#[test]
fn r2_no_definition_is_the_old_env() {
    let doc = ProfileDoc::empty(DocumentId::derive("r2-none"), Tol::witness());
    let doc = declare(&doc, "x", free_len(1.0));
    let doc = declare(&doc, "y", free_len(2.0));
    assert_eq!(doc.definition_order(), doc.var_order());
    let env = doc.var_env::<f64>();
    assert_eq!(env.bindings.len(), 2);
    assert!(env.refused.is_empty());
}

#[allow(dead_code)]
fn _unused(_: RecipeNodeId) {}

/// Cost of the door on a long chain of definitions: d_k := d_{k-1} + 1 mm.
#[test]
fn r2_chain_cost() {
    use editor_core::VarRef;
    for &total in &[100usize, 200, 400] {
        let mut doc = ProfileDoc::empty(DocumentId::derive("r2-chain"), Tol::witness());
        doc = declare(&doc, "d0", free_len(1.0));
        let t = std::time::Instant::now();
        let mut prev = id(&doc, "d0");
        for k in 1..total {
            let name = VarName::new(format!("d{k}")).unwrap();
            let a = step(
                &doc,
                DocEdit::DeclareVar {
                    name,
                    def: VarDecl::defined(
                        Expr::add(Expr::var(prev, Dimension::Length), len(0.001)).unwrap(),
                    ),
                },
            );
            prev = a.record.minted_var.unwrap();
            doc = a.doc;
        }
        let build = t.elapsed();
        let t = std::time::Instant::now();
        let _ = step(
            &doc,
            DocEdit::SetVarValue {
                var: VarRef::Name(VarName::new("d0".to_string()).unwrap()),
                value: FreeValue::Continuous(2.0),
            },
        );
        eprintln!("chain {total}: build {build:?}, one SetVarValue {:?}", t.elapsed());
    }
}

/// definition_order cost with free variables only (the C-world shape).
#[test]
fn r2_free_order_cost() {
    for &total in &[500usize, 1000, 2000] {
        let mut doc = ProfileDoc::empty(DocumentId::derive("r2-free"), Tol::witness());
        for k in 0..total {
            let name = VarName::new(format!("f{k}")).unwrap();
            doc = step(&doc, DocEdit::DeclareVar { name, def: free_len(1.0) }).doc;
        }
        let fresh = doc.clone();
        let t = std::time::Instant::now();
        let _ = fresh.definition_order().len();
        let order = t.elapsed();
        let fresh = doc.clone();
        let t = std::time::Instant::now();
        let _ = fresh.var_env::<f64>();
        eprintln!("free {total}: definition_order {order:?}, var_env (cold) {:?}", t.elapsed());
    }
}

/// C5: nested expansion — a slot reading g := h, h := 2w lowers as a
/// slot spelling 2w (the rows pin only one level).
#[test]
fn r2_nested_definition_token() {
    use topo::{FaceKey, SurfaceField};
    let doc = ProfileDoc::empty(DocumentId::derive("r2-nested"), Tol::witness());
    let doc = declare(&doc, "w", free_len(0.0625));
    let doc = declare(
        &doc,
        "h",
        VarDecl::defined(Expr::mul(Expr::literal(2.0, Dimension::Scalar).unwrap(), named("w")).unwrap()),
    );
    let doc = declare(&doc, "g", VarDecl::defined(named("h")));
    let blend = |doc: ProfileDoc, cx: f64, r: Expr| {
        let (doc, profile) = on_frame(doc, [0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], vec![square(cx, 0.0, 0.5)]);
        let (doc, cube) = insert(doc, Node::Extrude { profile, distance: len(1.0), side: ExtrudeSide::Along });
        let edges = crate::fixture::prism_edges(&doc, cube, 4);
        insert(doc, Node::fillet(cube, r, edges))
    };
    let (doc, by_g) = blend(doc, 0.0, named("g"));
    let (doc, by_f) = blend(
        doc,
        4.0,
        Expr::mul(Expr::literal(2.0, Dimension::Scalar).unwrap(), named("w")).unwrap(),
    );
    let e = ev(&doc, None);
    assert!(failures(&e).is_empty(), "{:?}", failures(&e));
    let token = |id| {
        let body = crate::corpus::body_of(&e, id);
        let face: FaceKey = topo::query::all_faces(body)
            .into_iter()
            .find(|&f| {
                body.get_face(f)
                    .and_then(|fd| body.get_surface(fd.surface))
                    .is_some_and(|s| matches!(s, geom::Surface::Cylinder { .. }))
            })
            .unwrap();
        let surface = body.get_face(face).unwrap().surface;
        body.surface_field_source(surface, SurfaceField::CylinderRadius)
            .unwrap()
            .clone()
    };
    assert_eq!(token(by_g), token(by_f));
}

/// Inline into a host that already holds the same named w and h := 2w:
/// the names are shared, not a conflict.
#[test]
fn r2_inline_into_a_host_holding_the_definition() {
    use crate::fixture::resolver::PartStore;
    use std::sync::Arc;
    let mk = |seed: &str| {
        let d = ProfileDoc::empty(DocumentId::derive(seed), Tol::witness());
        let d = declare(&d, "w", free_len(0.0625));
        declare(
            &d,
            "h",
            VarDecl::defined(Expr::add(named("w"), named("w")).unwrap()),
        )
    };
    let part = mk("r2-inline-part");
    let (part, profile) = on_frame(part, [0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], vec![square(0.0, 0.0, 0.5)]);
    let (part, _) = insert(part, Node::Extrude { profile, distance: named("h"), side: ExtrudeSide::Along });
    let mut store = PartStore::default();
    let doc_ref = store.insert(part, Tol::witness());
    let host = ProfileDoc::empty(DocumentId::derive("r2-inline-host"), Tol::witness());
    let host = declare(&host, "z", free_len(1.0));
    let host = declare(&host, "w", free_len(0.0625));
    let host = declare(&host, "h", VarDecl::defined(Expr::add(named("w"), named("w")).unwrap()));
    assert_ne!(id(&host, "w"), id(&mk("x"), "w"), "distinct ids");
    let (host, instance) = insert(host, Node::instantiate_part(doc_ref));
    let inlined = editor_core::inline(
        &host,
        instance,
        &(Arc::new(store) as Arc<dyn editor_core::PartResolver>),
        Tol::witness(),
    )
    .expect("the host's w and h are the part's");
    assert_eq!(inlined.doc.vars().len(), 3);
}

/// C4: a variable declared BEFORE a large chain, redefined to read its
/// top twice, refuses (its expansion is counted in definition order).
#[test]
fn r2_bound_for_an_early_declared_reader() {
    let doc = ProfileDoc::empty(DocumentId::derive("r2-early"), Tol::witness());
    let doc = declare(&doc, "x", free_len(1.0));
    let doc = declare(&doc, "w", free_len(1.0));
    const N: [&str; 11] = ["c0", "c1", "c2", "c3", "c4", "c5", "c6", "c7", "c8", "c9", "c10"];
    let mut doc = declare(&doc, N[0], VarDecl::defined(Expr::add(named("w"), named("w")).unwrap()));
    for k in 1..11 {
        doc = declare(&doc, N[k], VarDecl::defined(Expr::add(named(N[k - 1]), named(N[k - 1])).unwrap()));
    }
    match try_step(
        &doc,
        DocEdit::DefineVar {
            var: n("x").into(),
            def: VarDecl::defined(Expr::add(named("c10"), named("c10")).unwrap()),
        },
    ) {
        Err(EditError::DefinitionTooLarge { var, .. }) => assert_eq!(var.id(), id(&doc, "x")),
        o => panic!("{:?}", o.map(|a| a.doc.len())),
    }
}

/// C2/C7: g := h + 1 mm (g declared first), h := 2w, w deleted: g's
/// refusal names h's refusal, not "h has no binding".
#[test]
fn r2_refusal_chain_through_a_dead_read() {
    let doc = ProfileDoc::empty(DocumentId::derive("r2-dead"), Tol::witness());
    let doc = declare(&doc, "g", free_len(1.0));
    let doc = declare(&doc, "h", free_len(1.0));
    let doc = declare(&doc, "w", free_len(1.0));
    let (g, h, w) = (id(&doc, "g"), id(&doc, "h"), id(&doc, "w"));
    let doc = step(&doc, DocEdit::DefineVar { var: n("g").into(), def: VarDecl::defined(Expr::add(named("h"), len(0.001)).unwrap()) }).doc;
    let doc = step(&doc, DocEdit::DefineVar { var: n("h").into(), def: VarDecl::defined(Expr::add(named("w"), named("w")).unwrap()) }).doc;
    let doc = step(&doc, DocEdit::DeleteVar { var: n("w").into() }).doc;
    let env = doc.var_env::<f64>();
    assert_eq!(
        env.refused.get(&g),
        Some(&editor_core::EvalError::DefinitionRefused {
            var: h,
            source: Box::new(editor_core::EvalError::UnresolvedVar { var: w }),
        })
    );
}
