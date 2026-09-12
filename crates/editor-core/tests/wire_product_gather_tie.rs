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
//! before.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::unreachable
)]

use crate::fixture;

use editor_core::{
    BooleanOp, Datum, EntityKey, EntityRef, Entry, EvalOptions, Evaluation, NameTable, Node,
    ProductError, ProfileDoc, RecipeNodeId, StableName, product_named,
};
use fixture::{ang, insert, len, on_frame, scl};
use geom_core::Tol;

fn run(doc: &ProfileDoc) -> Evaluation<f64> {
    fixture::run(doc, &EvalOptions::default())
}

fn table(ev: &Evaluation<f64>, id: RecipeNodeId) -> &NameTable {
    &ev.value(id)
        .unwrap_or_else(|| panic!("node {id:?} has no value: {:?}", ev.nodes.get(&id)))
        .name_table
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
        assert!(
            product_cands.windows(2).all(|w| w[0] != w[1]),
            "the merge collapsed two candidates onto one entity: {name:?}"
        );
    }
}

#[test]
fn two_roots_aliasing_a_strict_name_still_refuse() {
    // One block, two transforms of it: both roots carry the block's
    // OWN names — a transform passes its operand's table through
    // verbatim — so every carried row is a strict name arriving twice.
    let doc = ProfileDoc::empty_derived("wire-product-gather-tie", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let shift = |doc: ProfileDoc, dx: f64| {
        insert(
            doc,
            Node::Transform {
                input: a,
                translation: [len(dx), len(0.0), len(0.0)],
                rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
                rotation_angle: ang(0.0),
            },
        )
    };
    let (doc, t0) = shift(doc, 2.0);
    let (doc, t1) = shift(doc, 4.0);
    assert_eq!(doc.roots(), &[t0, t1][..], "both transforms root");
    let ev = run(&doc);
    match product_named(&doc, &ev, Tol::witness()) {
        Err(ProductError::Naming { node, name }) => {
            assert_eq!(node, t1, "the refusal names the root whose rows collided");
            assert_eq!(name.node, a, "and the name the block minted");
        }
        other => panic!(
            "two roots aliasing a strict name must still refuse: {:?}",
            other.map(|(b, t)| (b.faces().count(), t.len()))
        ),
    }
}
