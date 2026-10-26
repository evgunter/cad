//! Review probes for PR 5748 (SetLoftSections). Each row prints what
//! the door answered and, where admitted, what evaluation answers.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::print_stdout)]

use crate::fixture::{self, insert, len};
use editor_core::{
    Axis3, DocEdit, EditError, EvalOptions, Formula, LoftSection, Node, Operand, ProfileDoc,
    RecipeNodeId, StationMove,
};
use geom_core::Tol;

fn eval_of(doc: &ProfileDoc, node: RecipeNodeId) -> String {
    let ev = editor_core::evaluate::<f64>(
        doc,
        None,
        &editor_core::CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
    match ev.result(node) {
        Some(editor_core::NodeResult::Ok(_)) => "Ok".into(),
        other => format!("{other:?}").chars().take(300).collect(),
    }
}

fn up(profile: RecipeNodeId, z: f64) -> LoftSection<Operand, Formula> {
    LoftSection {
        profile: profile.into(),
        station: vec![StationMove::Slide { along: Axis3::Z, by: len(z) }],
    }
}

fn base(n: usize, degree: i64) -> (ProfileDoc, Vec<RecipeNodeId>, RecipeNodeId) {
    let mut doc = ProfileDoc::empty_derived("d", Tol::witness());
    let mut ps = Vec::new();
    for k in 0..4 {
        let (d, p) = insert(doc, Node::Profile(fixture::desc(vec![fixture::square(0.0, 0.0, 0.5 - 0.05 * k as f64)])));
        doc = d;
        ps.push(p);
    }
    let then = (1..n).map(|k| up(ps[k], k as f64)).collect();
    let (doc, loft) = insert(doc, Node::Loft { first: ps[0].into(), then, v_degree: Formula::count(degree) });
    (doc, ps, loft)
}

fn set(doc: &ProfileDoc, node: RecipeNodeId, first: RecipeNodeId, then: Vec<LoftSection<Operand, Formula>>) -> Result<editor_core::Applied<editor_core::ProfileProgram>, EditError> {
    doc.apply(&DocEdit::SetLoftSections { node, first: first.into(), then, fresh: Vec::new() }, Tol::witness())
}

#[test]
fn p1_one_section() {
    let (doc, ps, loft) = base(2, 1);
    match set(&doc, loft, ps[0], vec![]) {
        Ok(a) => println!("P1 ADMITTED one section; eval: {}", eval_of(&a.doc, loft)),
        Err(e) => println!("P1 refused: {e:?} / {e}"),
    }
    let r = std::panic::catch_unwind(|| fixture::insert_refused(&doc, Node::Loft { first: ps[0].into(), then: vec![], v_degree: Formula::count(1) }));
    println!("P1b insert door one-section: {:?}", r.map(|e| format!("{e:?}")).map_err(|_| "insert admitted (panicked)"));
}

#[test]
fn p2_drop_leaves_degree_out_of_range() {
    let (doc, ps, loft) = base(3, 2);
    println!("P2 base eval: {}", eval_of(&doc, loft));
    match set(&doc, loft, ps[0], vec![up(ps[2], 2.0)]) {
        Ok(a) => println!("P2 ADMITTED degree-2 loft with 2 sections; eval: {}", eval_of(&a.doc, loft)),
        Err(e) => println!("P2 refused: {e:?}"),
    }
}

#[test]
fn p3_wrong_dimension_station() {
    let (doc, ps, loft) = base(2, 1);
    let bad = LoftSection { profile: ps[1].into(), station: vec![StationMove::Spin { about: Axis3::Z, by: len(1.0) }] };
    match set(&doc, loft, ps[0], vec![bad]) {
        Ok(a) => println!("P3 ADMITTED spin by a length; eval: {}", eval_of(&a.doc, loft)),
        Err(e) => println!("P3 refused: {e:?}"),
    }
}

#[test]
fn p4_empty_station_and_zero_slide() {
    let (doc, ps, loft) = base(2, 1);
    let empty = LoftSection { profile: ps[1].into(), station: vec![] };
    match set(&doc, loft, ps[0], vec![empty]) {
        Ok(a) => println!("P4 empty station ADMITTED; eval: {}", eval_of(&a.doc, loft)),
        Err(e) => println!("P4 refused: {e:?}"),
    }
    match set(&doc, loft, ps[0], vec![up(ps[1], 0.0)]) {
        Ok(a) => println!("P4b zero slide ADMITTED; eval: {}", eval_of(&a.doc, loft)),
        Err(e) => println!("P4b refused: {e:?}"),
    }
}

#[test]
fn p5_forward_read_save_load_and_replay() {
    let (doc, ps, loft) = base(2, 1);
    let (doc, late) = insert(doc, Node::Profile(fixture::desc(vec![fixture::square(0.0, 0.0, 0.2)])));
    let edit = DocEdit::SetLoftSections { node: loft, first: ps[0].into(), then: vec![up(ps[1], 1.0), up(late, 2.0)], fresh: Vec::new() };
    let applied = doc.apply(&edit, Tol::witness()).expect("forward read admitted");
    println!("P5 eval: {}", eval_of(&applied.doc, loft));
    // log replay
    let saved = editor_core::persist::save(&doc, std::slice::from_ref(&edit), Tol::witness());
    match saved {
        Ok(text) => {
            let loaded = editor_core::persist::load(&text, Tol::witness());
            println!("P5 log save ok; load: {:?}", loaded.as_ref().map(|_| "ok").map_err(|e| format!("{e}")));
            if let Ok(l) = loaded { println!("P5 replay eval: {}", eval_of(&l.doc, loft)); }
        }
        Err(e) => println!("P5 log save refused: {e}"),
    }
    // snapshot save of the forward-pointing doc
    match editor_core::persist::save(&applied.doc, &[], Tol::witness()) {
        Ok(text) => println!("P5 snapshot save ok; load: {:?}", editor_core::persist::load(&text, Tol::witness()).map(|_| "ok").map_err(|e| format!("{e}"))),
        Err(e) => println!("P5 snapshot save refused: {e}"),
    }
    // cascade delete of the late profile
    match applied.doc.apply(&DocEdit::DeleteNode { id: late }, Tol::witness()) {
        Ok(a) => println!("P5 delete late ok; loft eval: {} maint {:?}", eval_of(&a.doc, loft), a.maintenance.len()),
        Err(e) => println!("P5 delete late refused: {e}"),
    }
}

#[test]
fn p6_bound_station_var_survives_or_not() {
    // re-writing the same sections: is the doc's var table growing (orphans)?
    let (doc, ps, loft) = base(2, 1);
    let n0 = doc.definition_order().len();
    let a = set(&doc, loft, ps[0], vec![up(ps[1], 1.0)]).unwrap().doc;
    let a = set(&a, loft, ps[0], vec![up(ps[1], 1.0)]).unwrap().doc;
    println!("P6 vars before {n0}, after two identical rewrites {}", a.definition_order().len());
}
