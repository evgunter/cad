//! namedepth-r2 (head only): round trips at depth on the wasm32 stack.
#![allow(clippy::all, clippy::pedantic, clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![allow(unused)]

use crate::fixture;

use editor_core::{
    EntityKind, EvalOptions, NameRef, Node, ProfileDoc, RecipeNodeId, RoleSeg, StableName,
    canonical_bytes, content_pin, load, save,
};
use fixture::{insert, len, on_frame, square};
use geom_core::Tol;

fn small<R: Send + 'static>(f: impl FnOnce() -> R + Send + 'static) -> R {
    std::thread::Builder::new().stack_size(1 << 20).spawn(f).unwrap().join().unwrap()
}

fn wrap(depth: usize, node: RecipeNodeId, bottom: StableName, seg: fn(NameRef) -> RoleSeg) -> StableName {
    (0..depth).fold(bottom, |n, _| StableName { kind: n.kind, node, path: vec![seg(NameRef::new(n))] })
}

fn round_trip(tag: &str, depth: usize, seg: fn(NameRef) -> RoleSeg) -> String {
    let tol = Tol::witness();
    let doc = ProfileDoc::empty_derived(tag, tol);
    let (doc, profile) = on_frame(doc, [0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], vec![square(0.0, 0.0, 1.0)]);
    let (doc, extrude) = insert(doc, Node::Extrude { profile, distance: len(1.0) });
    let rim = fixture::rim_edge(extrude, editor_core::CapEnd::End, fixture::piece(&doc, extrude, 0, 0));
    let deep = wrap(depth, extrude, rim, seg);
    let step = editor_core::DocEdit::InsertNode { node: Node::fillet(extrude, len(0.1), vec![deep.clone()]) };
    let doc = match editor_core::apply(&doc, &step, tol, &editor_core::RefusingReach) {
        Ok(out) => out.doc,
        Err(e) => return format!("{tag}: edit door refused: {e}"),
    };
    let text = save(&doc, &[], tol).expect("saves");
    let pin1 = content_pin(&doc, tol).unwrap();
    let loaded = load(&text, tol).expect("loads");
    let pin2 = content_pin(&loaded.doc, tol).unwrap();
    let resaved = save(&loaded.doc, &[], tol).unwrap();
    let loaded2 = load(&resaved, tol).unwrap();
    let pin3 = content_pin(&loaded2.doc, tol).unwrap();
    // Evaluate: the fillet over an unresolvable name refuses; render it.
    let ev = fixture::run(&loaded.doc, &EvalOptions::default());
    let shown = format!("{:?}", ev.order.len());
    drop(ev);
    let depth_seen = format!("{deep:?}").matches("StableName {").count();
    format!(
        "{tag}: bytes={} doc_eq={} resave_eq={} pins_eq={} dbg_levels={depth_seen}",
        text.len(),
        loaded.doc == doc,
        resaved == text,
        pin1 == pin2 && pin2 == pin3
    )
}

#[test]
fn zz_r2_round_trips_on_the_wasm_stack() {
    let out = small(|| {
        vec![
            round_trip("inpart-1024", 1024, |r| RoleSeg::InPart { of: r }),
            round_trip("froma-1000", 1000, RoleSeg::FromA),
            round_trip("froma-5000", 5000, RoleSeg::FromA),
        ]
    });
    for l in out {
        eprintln!("R2 {l}");
    }
}
