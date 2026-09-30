//! Review probe (namedepth-r1, head only): documents whose names nest
//! 1 024 `InPart` levels and 1 000 `FromA` levels, through the edit
//! door, evaluation, save, load, re-save and the pin, on a 1 MiB stack.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, missing_docs)]

use crate::fixture;
use editor_core::{
    CapEnd, EntityKind, EvalOptions, NameRef, Node, ProfileDoc, RecipeNodeId, RoleSeg,
    StableName, canonical_bytes, content_pin, load, save,
};
use fixture::{insert, len, on_frame, run, square};
use geom_core::Tol;

fn small<R: Send + 'static>(f: impl FnOnce() -> R + Send + 'static) -> R {
    std::thread::Builder::new()
        .stack_size(1 << 20)
        .spawn(f)
        .unwrap()
        .join()
        .expect("the door returns on a 1 MiB stack")
}

fn block() -> (ProfileDoc, RecipeNodeId, StableName) {
    let doc = ProfileDoc::empty_derived("rv-deep", Tol::witness());
    let (doc, profile) = on_frame(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(0.0, 0.0, 1.0)],
    );
    let (doc, extrude) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(1.0),
        },
    );
    let rim = fixture::rim_edge(extrude, CapEnd::End, fixture::piece(&doc, extrude, 0, 0));
    (doc, extrude, rim)
}

fn round_trip(label: &str, wrap: fn(NameRef) -> RoleSeg, levels: usize) -> String {
    let (doc, extrude, rim) = block();
    let deep = (0..levels).fold(rim, |n, _| StableName {
        kind: n.kind,
        node: extrude,
        path: vec![wrap(NameRef::new(n))],
    });
    let (doc, fillet) = insert(doc, Node::fillet(extrude, len(0.1), vec![deep.clone()]));
    let tol = Tol::witness();
    let shown = format!("{deep:?}");
    let ev = run(&doc, &EvalOptions::default());
    let fault = match ev.result(fillet) {
        Some(r) => format!("{:.200}", format!("{r:?}")),
        None => "none".into(),
    };
    drop(ev);
    let text = save(&doc, &[], tol).expect("saves");
    if let Some(dir) = std::env::var_os("RV_OUT") {
        std::fs::write(std::path::Path::new(&dir).join(format!("{label}.pncad")), &text).unwrap();
    }
    let loaded = load(&text, tol).expect("loads");
    assert!(loaded.doc == doc, "{label}: loaded == saved");
    let resaved = save(&loaded.doc, &[], tol).unwrap();
    assert_eq!(resaved, text, "{label}: re-save byte-equal");
    let (p1, p2) = (
        content_pin(&doc, tol).unwrap(),
        content_pin(&loaded.doc, tol).unwrap(),
    );
    assert_eq!(p1, p2, "{label}: pin stable across save/load");
    let reloaded = load(&resaved, tol).unwrap();
    assert_eq!(content_pin(&reloaded.doc, tol).unwrap(), p1, "{label}: pin after second load");
    assert_eq!(
        canonical_bytes(&loaded.doc, tol).unwrap(),
        canonical_bytes(&doc, tol).unwrap()
    );
    let cloned = doc.clone();
    assert!(cloned == doc);
    drop((doc, loaded, reloaded, cloned, deep));
    format!(
        "{label}: levels={levels} debug_len={} text_len={} fillet_ok={fault}",
        shown.len(),
        text.len()
    )
}

#[test]
fn rv_inpart_1024_round_trips_on_a_small_stack() {
    let s = small(|| round_trip("inpart", |of| RoleSeg::InPart { of }, 1024));
    eprintln!("{s}");
}

#[test]
fn rv_froma_1000_round_trips_on_a_small_stack() {
    let s = small(|| round_trip("froma", RoleSeg::FromA, 1000));
    eprintln!("{s}");
}

#[test]
fn rv_froma_3000_round_trips_on_a_small_stack() {
    let s = small(|| round_trip("froma3k", RoleSeg::FromA, 3_000));
    eprintln!("{s}");
}
