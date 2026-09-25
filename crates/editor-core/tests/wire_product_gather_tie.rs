//! **A tie the document SEPARATES, gathered** (WIRE): a split whose
//! plane sends one candidate of an N2 tie into each half, and which is
//! the document's only root, so the gather carries the two halves as
//! two sources and meets the tied name twice.
//!
//! What the product says about it is the subject: the two faces are
//! both there and both equally admissible, so the aggregate table
//! holds ONE tied row over both — the merge `crate::names::CarriedRows`
//! performs — rather than the gather refusing `ProductError::Naming`
//! because the second half's row looked like an aliasing bug. The
//! ambiguity is not resolved here, only carried: a selection that
//! matches both candidates is what refuses (`TiedDisagrees`, GS-Q4).
//!
//! The second row is the guard that is NOT retired by that: two roots
//! that alias a STRICT name still refuse, because a row whose source
//! entry is `Unique` goes through `NameTable::insert` exactly as
//! before. Its document shares through a split's intact pass-through,
//! the one sharing the recipe cannot decide, so it is the carry that
//! refuses and not the recipe check ahead of it.
//!
//! The third row places the same tie with a placed union, the other
//! op that carries several copies of a tie onto one body: disjoint
//! instances keep every candidate, so each instance's tie stays tied.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::unreachable
)]

use crate::fixture;

use editor_core::{
    BooleanOp, Datum, EntityKey, EntityRef, Entry, EvalOptions, Evaluation, Expr, NameTable, Node,
    PatternKind, ProductError, ProfileDoc, RecipeNodeId, StableName, product_named,
};
use fixture::{ang, insert, len, on_frame, scl, table};
use geom_core::Tol;

fn run(doc: &ProfileDoc) -> Evaluation<f64> {
    fixture::run(doc, &EvalOptions::default())
}

/// A rectangular block: profile on the plane z = `z0`, extruded `dz`.
fn block(
    doc: ProfileDoc,
    (x0, x1): (f64, f64),
    (y0, y1): (f64, f64),
    z0: f64,
    dz: f64,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, p) = on_frame(
        doc,
        [0.0, 0.0, z0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(x0, y0), (x1, y0), (x1, y1), (x0, y1)]],
    );
    insert(
        doc,
        Node::Extrude {
            profile: p,
            distance: len(dz),
        },
    )
}

/// `m4_pr3_names_bool`'s U-cutter subtract, rebuilt: a 4×4×4 block
/// less a U whose two prongs cross one wall, leaving two cap
/// fragments that no covariant qualifier separates — the genuine N2
/// tie that suite pins, one candidate in each prong.
fn u_cutter_subtract() -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("wire-product-gather-tie", Tol::witness());
    let (doc, a) = block(doc, (0.0, 4.0), (0.0, 4.0), 0.0, 4.0);
    let (doc, p) = on_frame(
        doc,
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![
            (2.0, 1.0),
            (6.0, 1.0),
            (6.0, 3.0),
            (2.0, 3.0),
            (2.0, 2.5),
            (5.0, 2.5),
            (5.0, 1.5),
            (2.0, 1.5),
        ]],
    );
    let (doc, b) = insert(
        doc,
        Node::Extrude {
            profile: p,
            distance: len(2.0),
        },
    );
    let (doc, sub) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Subtract,
            a,
            b,
            declare: None,
        },
    );
    (doc, sub)
}

/// That subtract, split by y = 2 — a plane that passes BETWEEN the
/// prongs, so it separates the two tied candidates without cutting
/// either, and which consumes the subtract and becomes the only root.
fn u_cutter_split_across_the_tie() -> (ProfileDoc, RecipeNodeId) {
    let (doc, sub) = u_cutter_subtract();
    let (doc, plane) = insert(
        doc,
        Node::Datum(Datum::Plane {
            origin: [len(0.0), len(2.0), len(0.0)],
            normal: [scl(0.0), scl(1.0), scl(0.0)],
        }),
    );
    let (doc, split) = insert(
        doc,
        Node::Split {
            target: sub,
            tool: plane,
        },
    );
    (doc, split)
}

/// Every name of `t` whose entry ties candidates lying in DIFFERENT
/// output bodies — the shape the split hands the gather, and the one
/// a per-source narrowing cannot see.
fn ties_across_bodies(t: &NameTable) -> Vec<(StableName, Vec<EntityRef>)> {
    t.iter()
        .filter_map(|(n, e)| match e {
            Entry::Tied(cands) if cands.iter().any(|c| c.body != cands[0].body) => {
                Some((n.clone(), cands.clone()))
            }
            _ => None,
        })
        .collect()
}

#[test]
fn a_split_separating_a_tie_gathers_and_the_product_holds_one_tied_row() {
    let (doc, split) = u_cutter_split_across_the_tie();
    assert_eq!(doc.roots(), &[split][..], "the split is the only root");
    let ev = run(&doc);

    // The precondition, measured rather than assumed: the split's own
    // table carries at least one tie whose candidates the plane put in
    // different halves. Without one this document does not reach the
    // carry at all and every assertion below is vacuous.
    let split_table = table(&ev, split);
    let straddling = ties_across_bodies(split_table);
    assert!(
        !straddling.is_empty(),
        "the fixture no longer pins a tie straddling the split's two halves; \
         the split's tied rows are {:?}",
        split_table
            .iter()
            .filter(|(_, e)| matches!(e, Entry::Tied(_)))
            .collect::<Vec<_>>()
    );

    let (body, names) = match product_named(&doc, &ev, Tol::witness()) {
        Ok(p) => p,
        Err(e) => panic!("the gather refused a document that evaluates and names: {e}"),
    };
    let faces: std::collections::BTreeSet<_> = body.faces().map(|(k, _)| k).collect();

    for (name, split_cands) in &straddling {
        let entry = names
            .lookup(name)
            .unwrap_or_else(|| panic!("the product dropped a carried tie: {name:?}"));
        let Entry::Tied(product_cands) = entry else {
            panic!(
                "the product narrowed a tie its own body did not separate: \
                 {name:?} -> {entry:?}"
            );
        };
        // One row over BOTH candidates, re-keyed onto the aggregate:
        // the count is the split's, the bodies are all 0 (a product is
        // one body), and each key is a face the graft actually minted.
        //
        // The COUNT is what guards against two candidates collapsing
        // onto one entity, and it is the only thing that can: the
        // insert door sorts and dedups before storing, so a row that
        // lost a candidate that way comes back SHORT rather than
        // holding a repeat.
        assert_eq!(
            product_cands.len(),
            split_cands.len(),
            "the product lost a candidate of {name:?}"
        );
        for c in product_cands {
            assert_eq!(c.body, 0, "a product row outside the product's one body");
            let EntityKey::Face(f) = c.key else {
                panic!("the tie's candidates are cap fragments, so faces: {c:?}")
            };
            assert!(
                faces.contains(&f),
                "a product name points at a face the aggregate does not hold: {c:?}"
            );
        }
    }
}

/// The U-cutter subtract's tie, placed three times by a placed union
/// whose instances are disjoint: each instance's tie keeps BOTH its
/// candidates through the fuse, so the fused table carries one
/// two-candidate `Tied` row per prototype tie per instance — the
/// several-survivor branch of the placed union's narrowing, which no
/// other row reaches.
#[test]
fn a_placed_union_carries_each_instances_tie_with_both_candidates() {
    let (doc, sub) = u_cutter_subtract();
    let (doc, group) = insert(
        doc,
        Node::placed_union(
            sub,
            Expr::count(3),
            PatternKind::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: len(10.0),
            },
        )
        .unwrap(),
    );
    let ev = run(&doc);
    let proto: Vec<usize> = table(&ev, sub)
        .iter()
        .filter_map(|(_, e)| match e {
            Entry::Tied(c) => Some(c.len()),
            Entry::Unique(_) => None,
        })
        .collect();
    assert_eq!(proto, vec![2, 2], "the prototype's two-candidate ties");
    let fused: Vec<usize> = table(&ev, group)
        .iter()
        .filter_map(|(_, e)| match e {
            Entry::Tied(c) => Some(c.len()),
            Entry::Unique(_) => None,
        })
        .collect();
    assert_eq!(
        fused,
        vec![2; proto.len() * 3],
        "one two-candidate tie per prototype tie per instance"
    );
}

#[test]
fn two_roots_aliasing_a_strict_name_still_refuse() {
    // One block, split at x = 0.5 by one root and moved whole by
    // another. The plane cuts the four walls it crosses and leaves the
    // two x-facing walls intact, and an intact wall keeps the block's
    // own name through the split — so the moved block's copy of it is
    // a strict name arriving twice. Nothing in the recipe says which
    // walls the plane leaves whole, so the recipe check passes this
    // document and the carry is what refuses.
    let doc = ProfileDoc::empty_derived("wire-product-gather-tie", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, plane) = insert(
        doc,
        Node::Datum(Datum::Plane {
            origin: [len(0.5), len(0.0), len(0.0)],
            normal: [scl(1.0), scl(0.0), scl(0.0)],
        }),
    );
    let (doc, split) = insert(
        doc,
        Node::Split {
            target: a,
            tool: plane,
        },
    );
    let (doc, moved) = insert(
        doc,
        Node::Transform {
            input: a,
            translation: [len(2.0), len(0.0), len(0.0)],
            rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
            rotation_angle: ang(0.0),
        },
    );
    assert_eq!(
        doc.roots(),
        &[split, moved][..],
        "the split and the move root"
    );
    let ev = run(&doc);
    match product_named(&doc, &ev, Tol::witness()) {
        Err(ProductError::Naming { node, name }) => {
            assert_eq!(
                node, moved,
                "the refusal names the root whose rows collided"
            );
            assert_eq!(name.node, a, "and the name the block minted");
        }
        other => panic!(
            "two roots aliasing a strict name must still refuse: {:?}",
            other.map(|(b, t)| (b.faces().count(), t.len()))
        ),
    }
}
