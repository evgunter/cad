//! Corpus documents **nested_islands_105** and
//! **nested_islands_106_depth1** — the doubly-nested island chain
//! from the issue #93 adversarial review, as recipes.
//!
//! The chain is in GENERAL POSITION (no value-equal plane pair
//! anywhere), so nothing here is declared: these documents exercise
//! the boolean-of-boolean island machinery, not the declaration rung.
//!
//! - `nested_islands_105`: plate ∪ tube ∪ pillar. On pre-#93 main the
//!   second union SILENTLY returned 22.5 while the exact answer is
//!   22.4375 (issue #105 — a fail-loud violation); the corpus pins the
//!   exact value at the RECIPE level, so a regression shows up in the
//!   model layer, not only in `topo`.
//! - `nested_islands_106_depth1`: the same chain WITHOUT the pillar,
//!   intersected with a slab across the tube's midriff — the depth-1
//!   control that builds exactly. The depth-2 version (with the
//!   pillar) still refuses TYPED at the anchor-exhaustion arm on this
//!   branch's `main`; it is pinned as a refusal in
//!   `m4_pr8_corpus.rs::issue106_depth2_recipe_refuses_typed` rather
//!   than carried here, because every corpus document must evaluate
//!   GREEN. Retire that pin (and promote the depth-2 chain into the
//!   corpus proper) when #106 lands.
//!
//! Vocabulary: Profile, Extrude, Boolean (Subtract/Union/Intersect),
//! `InsertNode`, `SetParam`.
//!
//! Exact oracles, derived here (all dimensions dyadic):
//!
//! - tube = outer `2·2·2.5 = 10` − hole `1·1·2.5 = 2.5` = `7.5`
//! - plate ∪ tube = `16 + 7.5 − 1.5` (the tube's `z ∈ [0.5,1]` slab
//!   overlap, footprint `4−1 = 3`) = **22.0**
//! - ∪ pillar = `22 + 0.5 − 0.0625` (the pillar's `z ∈ [0.75,1]`
//!   overlap with the plate, footprint `0.25`) = **22.4375**
//! - depth-1 ∩ slab = annulus `3` × slab height `1` = **3.0**

use editor_core::{BooleanOp, Dimension, DocEdit, Expr, Node, RecipeNodeId, SlotId};

use super::{CorpusDoc, MassPin, Recorder};
use crate::fixture::{desc, len};

/// A rectangle in the xy plane at height `z`, extruded `h`.
fn block(r: &mut Recorder, x: (f64, f64), y: (f64, f64), z: f64, h: f64) -> RecipeNodeId {
    let p = r.insert(Node::Profile(desc(
        [0.0, 0.0, z],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(x.0, y.0), (x.1, y.0), (x.1, y.1), (x.0, y.1)]],
    )));
    r.insert(Node::Extrude {
        profile: p,
        distance: len(h),
    })
}

/// Plate ∪ (outer − hole): the depth-1 chain, exact 22.0. Returns the
/// union node.
fn plate_with_tube(r: &mut Recorder) -> RecipeNodeId {
    let plate = block(r, (0.0, 4.0), (0.0, 4.0), 0.0, 1.0);
    let outer = block(r, (1.0, 3.0), (1.0, 3.0), 0.5, 2.5);
    let hole = block(r, (1.5, 2.5), (1.5, 2.5), 0.25, 3.0);
    let tube = r.insert(Node::Boolean {
        op: BooleanOp::Subtract,
        a: outer,
        b: hole,
        declare: None,
    });
    r.insert(Node::Boolean {
        op: BooleanOp::Union,
        a: plate,
        b: tube,
        declare: None,
    })
}

/// The #105 exactness document: the doubly-nested union chain.
pub fn document_105() -> CorpusDoc {
    let mut r = Recorder::new();
    let u1 = plate_with_tube(&mut r);
    let pillar = block(&mut r, (1.75, 2.25), (1.75, 2.25), 0.75, 2.0);
    let u2 = r.insert(Node::Boolean {
        op: BooleanOp::Union,
        a: u1,
        b: pillar,
        declare: None,
    });
    CorpusDoc {
        name: "nested_islands_105",
        about: "doubly-nested island union chain (#105 exactness pin)",
        edits: r.edits,
        doc: r.doc,
        result: Some(u2),
        pin: Some(MassPin {
            volume: 22.4375,
            area: None, // the island rings make the area non-obvious; volume is the oracle
        }),
        bump: DocEdit::SetParam {
            node: pillar,
            slot: SlotId::Distance,
            expr: Expr::literal(1.75, Dimension::Length).expect("dyadic length literal"),
        },
        bump_root: pillar,
    }
}

/// The #106 depth-1 control document: the same chain intersected with
/// a slab across the tube's midriff.
pub fn document_106_depth1() -> CorpusDoc {
    let mut r = Recorder::new();
    let u1 = plate_with_tube(&mut r);
    let slab = block(&mut r, (-1.0, 5.0), (-1.0, 5.0), 1.375, 1.0);
    let cut = r.insert(Node::Boolean {
        op: BooleanOp::Intersect,
        a: u1,
        b: slab,
        declare: None,
    });
    CorpusDoc {
        name: "nested_islands_106_depth1",
        about: "depth-1 nested-island intersect (the #106 coverage boundary's green side)",
        edits: r.edits,
        doc: r.doc,
        result: Some(cut),
        pin: Some(MassPin {
            volume: 3.0,
            area: None,
        }),
        bump: DocEdit::SetParam {
            node: slab,
            slot: SlotId::Distance,
            expr: Expr::literal(0.75, Dimension::Length).expect("dyadic length literal"),
        },
        bump_root: slab,
    }
}

/// The depth-2 chain (u1 ∪ pillar) ∩ slab, as a document — the #106
/// residue. NOT a corpus document (it does not evaluate green on this
/// `main`); the refusal pin in `m4_pr8_corpus.rs` consumes it.
pub fn depth2_refusal_probe() -> (editor_core::ProfileDoc, RecipeNodeId) {
    let mut r = Recorder::new();
    let u1 = plate_with_tube(&mut r);
    let pillar = block(&mut r, (1.75, 2.25), (1.75, 2.25), 0.75, 2.0);
    let u2 = r.insert(Node::Boolean {
        op: BooleanOp::Union,
        a: u1,
        b: pillar,
        declare: None,
    });
    let slab = block(&mut r, (-1.0, 5.0), (-1.0, 5.0), 1.375, 1.0);
    let cut = r.insert(Node::Boolean {
        op: BooleanOp::Intersect,
        a: u2,
        b: slab,
        declare: None,
    });
    (r.doc, cut)
}
