//! Review probe (lane `partroot-rev`): the poisoned-part-root input,
//! printed through API that exists on both this branch and main, so the
//! two can be compared by running this on each.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::print_stdout)]

#[allow(dead_code, unused_imports)]
#[path = "../tests/fixture/mod.rs"]
mod fixture;

use std::collections::BTreeMap;
use std::sync::Arc;

use editor_core::{
    CancelToken, DocRef, DocumentId, EvalOptions, Expr, Node, NodeResult, PartResolver,
    ProfileDoc, ResolveFailure, ResolveFault, content_pin, evaluate,
};
use fixture::{ang, insert, len, on_frame, scl, square};
use geom_core::Tol;

#[derive(Debug, Default)]
struct Store(BTreeMap<DocumentId, ProfileDoc>);

impl PartResolver for Store {
    fn resolve(&self, doc_ref: &DocRef, _tol: Tol) -> Result<ProfileDoc, ResolveFailure> {
        self.0.get(&doc_ref.id).cloned().ok_or(ResolveFailure {
            fault: ResolveFault::Unresolved,
            message: "no such document".to_string(),
        })
    }
}

fn main() {
    let tol = Tol::witness();
    let doc = ProfileDoc::empty(DocumentId::derive("rev-poisoned"), tol);
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(0.0, 0.0, 0.5)],
    );
    let (doc, extrude) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: Expr::div(len(1.0), scl(0.0)).unwrap(),
        },
    );
    let (part, _) = insert(
        doc,
        Node::Transform {
            input: extrude,
            translation: [len(0.01), len(0.0), len(0.0)],
            rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
            rotation_angle: ang(0.0),
        },
    );
    let r = DocRef {
        id: part.id(),
        pin: content_pin(&part, tol).unwrap(),
    };
    let mut store = Store::default();
    store.0.insert(part.id(), part);
    let asm = ProfileDoc::empty(DocumentId::derive("rev-poisoned-asm"), tol);
    let (asm, inst) = insert(asm, Node::instantiate_part(r));
    let opts = EvalOptions {
        resolver: Some(Arc::new(store)),
        ..EvalOptions::default()
    };
    let ev = evaluate::<f64>(&asm, None, &CancelToken::new(), &opts, tol);
    match ev.result(inst) {
        Some(NodeResult::Failed(e)) => {
            println!("DISPLAY {e}");
            println!("DEBUG {:?}", e.kind);
        }
        other => println!("OTHER {other:?}"),
    }
}
