// Reviewer probes for #4359 (not built). zz_rv_prec: at B tip b57e0376; zz_rv_c: at head 1ed6477f2.
// ---- zz_rv_prec.rs ----
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use crate::corpus;
use crate::fixture::value_channel::body_digest;
use editor_core::product;
use geom_core::Tol;
const FILES: [&str; 5] = [
    "crates/editor-core/tests/golden/golden.cad",
    "crates/editor-core/tests/corpus/die_tool.pncad",
    "crates/editor-core/tests/corpus/tour/die_composed_tour.pncad",
    "crates/viewer/tests/gallery_ring.pncad",
    "crates/pncad/tests/plate_param.pncad",
];
fn row(doc: &editor_core::ProfileDoc) -> String {
    let ev = corpus::eval::<f64>(doc);
    match product(doc, &ev, Tol::witness()) {
        Ok(body) => { let (w, f) = body_digest(&body); format!("{w:016x}/{f}") }
        Err(e) => format!("refused {:?}", e.kind()),
    }
}
#[test]
fn rv_prec_rows() {
    for d in corpus::documents() { println!("RV (\"{}\", \"{}\"),", d.name, row(&d.doc)); }
    let here = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    for file in FILES {
        let eps = std::env::var("CAD_TOLERANCE_EPS").ok().filter(|e| !e.is_empty());
        let text: String = std::fs::read_to_string(here.join(file)).unwrap().lines().map(|l| match (&eps, l.trim_start().starts_with("\"epsilon\":")) { (Some(e), true) => format!("  \"epsilon\": {e},\n"), _ => format!("{l}\n") }).collect();
        match editor_core::persist::load(&text, Tol::witness()) {
            Ok(l) => println!("RV (\"{file}\", \"{}\"),", row(&l.doc)),
            Err(e) => println!("RV (\"{file}\", \"load refused {e}\"),"),
        }
    }
}
// ---- zz_rv_c.rs ----
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use crate::corpus;
use crate::fixture::value_channel::body_digest;
use crate::fixture::{insert, len, on_frame, place, square, step};
use editor_core::{BooleanOp, DocEdit, ExtrudeSide, Node, NodeResult, ProfileDoc, RecipeNodeId, product};
use geom_core::Tol;

fn block(doc: ProfileDoc, cx: f64) -> (ProfileDoc, RecipeNodeId) { blockz(doc, cx, 0.0) }
fn blockz(doc: ProfileDoc, cx: f64, z: f64) -> (ProfileDoc, RecipeNodeId) {
    let (doc, profile) = on_frame(doc, [0.0, 0.0, z], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], vec![if z == 0.0 { square(cx, 0.0, 0.5) } else { square(cx, 0.17, 0.41) }]);
    insert(doc, Node::Extrude { profile: profile.into(), distance: len(1.0), side: ExtrudeSide::Along })
}

fn posed(dz: f64) -> u64 {
    let doc = ProfileDoc::empty_derived("rv-c-reads-world", Tol::witness());
    let (doc, a) = block(doc, 0.0);
    let (doc, b) = blockz(doc, 0.25, 0.3);
    let pose = editor_core::Placement::literal(&editor_core::Frame::translation([0.0, 0.0, dz]));
    let (doc, p) = step(doc, DocEdit::place(a, Some(pose)));
    let p = p.unwrap();
    let copy = doc.output(p, 0).unwrap();
    // A construction reading the world copy.
    let (doc, u) = step(doc, DocEdit::InsertNode { node: Box::new(Node::Boolean { op: BooleanOp::Union, a: copy.into(), b: b.into(), declare: Vec::new() }), fresh: Vec::new() });
    let u = u.expect("RV: the edit door ADMITS a boolean reading a world copy");
    let run = corpus::eval::<f64>(&doc);
    let Some(NodeResult::Ok(v)) = run.result(u) else { panic!("RV: boolean over a copy: {:?}", run.result(u).map(|r| format!("{r:?}").chars().take(300).collect::<String>())) };
    let editor_core::ValuePayload::Boolean(editor_core::BooleanValue::Body { body, .. }) = &v.payload else { panic!("body") };
    body_digest(body.as_ref()).0
}

#[test]
fn rv_a_construction_reads_the_world() {
    let (d0, d1) = (posed(0.0), posed(0.45));
    println!("RV boolean-over-copy digests: pose dz=0 -> {d0:016x}, dz=0.5 -> {d1:016x}");
    assert_ne!(d0, d1, "RV: the boolean's geometry depends on the placement's pose");
}

#[test]
fn rv_unplaced_lists_no_bodies_output() {
    let doc = ProfileDoc::empty_derived("rv-c-pattern", Tol::witness());
    let (doc, a) = block(doc, 0.0);
    let (doc, pat) = insert(doc, Node::Pattern { input: a.into(), count: editor_core::Formula::count(3), kind: editor_core::PatternKind::Linear { direction: [crate::fixture::scl(1.0), crate::fixture::scl(0.0), crate::fixture::scl(0.0)], spacing: len(3.0) } });
    let un = doc.unplaced();
    println!("RV unplaced = {un:?}; pattern output = {:?}", doc.output(pat, 0));
    let _ = place;
    let run = corpus::eval::<f64>(&doc);
    println!("RV product = {:?}", product(&doc, &run, Tol::witness()).err().map(|e| e.to_string()));
}
