//! **The tree row names the cause, not a dangling head.**
//!
//! The finding's document — a mate onto a pattern copy whose
//! direction slot is `1e200` — used to draw a row reading "resolves
//! through node 1, which does not resolve to a live member", about a
//! pattern that resolves and is well-formed apart from one slot. The
//! row is the chrome's whole channel for a refusal, so what it says
//! is the measurement.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::common;

use std::collections::BTreeMap;
use std::sync::Arc;

use pncad::document::{
    Alignment, AxisSense, CancelToken, Doc, DocEdit, DocRef, DocumentId, EvalOptions, Expr,
    MateFrame, MatePrimitive, Node, PartResolver, PatternKind, ProfileDoc, ProfileProgram,
    RecipeNodeId, ResolveFailure, ResolveFault, SitedRef, content_pin, evaluate,
};
use pncad::geom_core::Tol;
use pncad::prelude::StableName;
use pncad::select::{CapEnd, ContactClass, EntityKind, RoleSeg};
use viewer::tree::{self, RowStatus};

/// An in-memory part store — the seam an `InstantiatePart` reaches
/// its document through, over a map.
#[derive(Debug, Default)]
struct PartStore {
    docs: BTreeMap<DocumentId, ProfileDoc>,
}

impl PartStore {
    fn insert(&mut self, doc: ProfileDoc, tol: Tol) -> DocRef {
        let pin = content_pin(&doc, tol).expect("the pin computes");
        let id = doc.id();
        self.docs.insert(id, doc);
        DocRef { id, pin }
    }
}

impl PartResolver for PartStore {
    fn resolve(&self, doc_ref: &DocRef, _tol: Tol) -> Result<ProfileDoc, ResolveFailure> {
        self.docs
            .get(&doc_ref.id)
            .cloned()
            .ok_or_else(|| ResolveFailure {
                fault: ResolveFault::Unresolved,
                message: "no such document".to_string(),
            })
    }
}

/// A small block, as a whole part document: frame, profile, extrude,
/// so the extrude is node 2.
fn block(label: &str, tol: Tol) -> ProfileDoc {
    let doc = ProfileDoc::empty(DocumentId::derive(label), tol);
    let (doc, profile) = common::framed_square(&doc, 0.02, tol);
    let (doc, _) = common::inserted(
        &doc,
        Node::Extrude {
            profile,
            distance: common::len(0.02),
        },
        tol,
    );
    doc
}

/// A cap face of `instance`'s part product, in the wrapper the
/// instantiate node mints.
fn in_part(instance: RecipeNodeId, cap: CapEnd) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node: instance,
        path: vec![RoleSeg::InPart {
            of: Box::new(StableName {
                kind: EntityKind::Face,
                node: RecipeNodeId(2),
                path: vec![RoleSeg::Cap(cap)],
            }),
        }],
    }
}

/// **The finding's document, through the tree the chrome draws.**
/// The mate's row must state the direction refusal; the pattern's row
/// is `Poisoned` through the mate, which is exactly why the row that
/// names the cause has to be this one.
#[test]
fn the_mate_row_names_the_direction_and_not_a_dangling_head() {
    let tol = Tol::witness();
    let mut store = PartStore::default();
    let leg = store.insert(block("msolve3-view-leg", tol), tol);
    let top = store.insert(block("msolve3-view-top", tol), tol);

    let doc: Doc<ProfileProgram> = ProfileDoc::empty(DocumentId::derive("msolve3-view"), tol);
    let (doc, legs) = common::inserted(&doc, Node::instantiate_part(leg), tol);
    let (doc, pattern) = common::inserted(
        &doc,
        Node::Pattern {
            input: legs,
            count: Expr::count(4),
            kind: PatternKind::Linear {
                direction: [common::scl(1e200), common::scl(0.0), common::scl(0.0)],
                spacing: common::len(0.05),
            },
        },
        tol,
    );
    let (doc, cap) = common::inserted(&doc, Node::instantiate_part(top), tol);
    let frame = |origin: [f64; 3], axis: [f64; 3]| MateFrame {
        origin,
        axis,
        reference: [1.0, 0.0, 0.0],
    };
    let (doc, mate) = common::edited(
        &doc,
        DocEdit::InsertNode {
            node: Node::Mate {
                a: SitedRef::at_mint(StableName {
                    kind: EntityKind::Face,
                    node: pattern,
                    path: vec![RoleSeg::Instance {
                        i: 1,
                        of: Box::new(in_part(legs, CapEnd::End)),
                    }],
                }),
                b: SitedRef::at_mint(in_part(cap, CapEnd::Start)),
                class: ContactClass::Rest,
                alignment: Alignment {
                    a: frame([0.0, 0.0, 0.02], [0.0, 0.0, 1.0]),
                    b: frame([0.0, 0.0, 0.0], [0.0, 0.0, -1.0]),
                    primitive: MatePrimitive::FrameCoincidence,
                    sense: AxisSense::Opposed,
                    clocking: None,
                },
            },
        },
        tol,
    );
    let mate = mate.expect("the mate mints");

    let opts = EvalOptions {
        resolver: Some(Arc::new(store) as Arc<dyn PartResolver>),
        ..EvalOptions::default()
    };
    let ev = evaluate::<f64>(&doc, None, &CancelToken::new(), &opts, tol);
    let rows = tree::rows(&doc, Some(&ev));
    let row = rows
        .iter()
        .find(|r| r.id == mate)
        .expect("the mate has a row");
    let message = row
        .status
        .message()
        .expect("a failed row carries the refusal's prose");
    assert!(
        matches!(row.status, RowStatus::Failed { .. }),
        "the mate's row is the failure: {:?}",
        row.status
    );
    // The WHOLE rendered cause, not a substring of it: a wrong role
    // word, a wrong refusal kind (a zero length instead of an
    // unmeasurable one), or a wrong node all fail here.
    assert_eq!(
        message,
        format!(
            "node {} failed: the mate solve refused: mate {}'s a reference has no derived pose: \
             node {} refuses — the pattern direction has no finite length — its components \
             overflow the norm, or one of them is not a number; scale the geometry into the \
             session's range",
            mate.0, mate.0, pattern.0
        ),
        "the row states the cause the evaluation typed"
    );

    // The compensation the old design rested on, measured: the
    // pattern's own row cannot state the cause, because the mate
    // fault poisoned it.
    let placer = rows
        .iter()
        .find(|r| r.id == pattern)
        .expect("the pattern has a row");
    assert!(
        matches!(placer.status, RowStatus::Poisoned { through, .. } if through == mate),
        "the pattern is poisoned through the mate: {:?}",
        placer.status
    );
}
