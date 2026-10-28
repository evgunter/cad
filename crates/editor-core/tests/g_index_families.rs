//! **Repetition is an index** (D10 `[repetition]`, INTENT stage 3 G):
//! `k = index(N)` is a `Count` over `0..N`; an operation whose reads
//! reach it is evaluated once per value, and its value is a family,
//! its members named `Member { (k), of }`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeMap;

use crate::blocks::block;
use crate::fixture::{self, insert};
use editor_core::{
    Bodies, Coord, Dimension, DocEdit, Evaluation, Formula, Node, Operand, ProfileDoc,
    RecipeNodeId, RoleSeg, TargetAxis, ValuePayload, VarDecl, VarId, VarName, parse_formula,
};
use geom_core::Tol;

/// `doc` with the index `name = index(count)` declared, and its id.
pub(crate) fn index(doc: ProfileDoc, name: &str, count: Formula) -> (ProfileDoc, VarId) {
    let name = VarName::new(name).unwrap();
    let doc = fixture::step(
        doc,
        DocEdit::DeclareVar {
            name: name.clone(),
            def: VarDecl::Index {
                count,
                within: None,
            },
        },
    )
    .0;
    let id = doc.var_named(name.as_str()).expect("the index is declared");
    (doc, id)
}

/// `text` parsed over the document's named variables.
fn formula(doc: &ProfileDoc, text: &str) -> Formula {
    let params: BTreeMap<VarName, Dimension> = doc
        .var_ids()
        .into_iter()
        .filter_map(|id| {
            let name = doc.var_name(id)?.clone();
            let dim = doc.var(id)?.kind().dimension()?;
            Some((name, dim))
        })
        .collect();
    parse_formula(text, &params).unwrap()
}

/// **A ring of `n` copies of a block off the z axis**, member `k` spun
/// `scalar(k)·turn/n` about the world's z by a mateless bundle: the
/// document, the block and the placement whose copy is the family.
pub(crate) fn ring(n: i64) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("g-ring", Tol::witness());
    let (doc, _) = index(doc, "k", Formula::count(n));
    let (doc, the_block) = block(doc, (4.0, 5.0), (-0.5, 0.5), 1.0);
    let spin = formula(&doc, &format!("scalar(k) * turn / {n}"));
    let placed = editor_core::Placed::of(&doc, Vec::new(), vec![(Coord::Spin(Some(TargetAxis::Z)), spin)]);
    let node = fixture::place_node(
        &doc,
        editor_core::BodyRead::plain(Operand::Node(the_block)),
        placed.bundle(),
    );
    let (doc, ring) = insert(doc, node);
    (doc, the_block, ring)
}

fn run(doc: &ProfileDoc) -> Evaluation<f64> {
    fixture::run(doc, &Default::default())
}

fn bodies(ev: &Evaluation<f64>, id: RecipeNodeId) -> usize {
    match &ev.value(id).unwrap_or_else(|| panic!("{id:?}: {:?}", ev.result(id))).payload {
        ValuePayload::Instances(bodies) => bodies.len(),
        ValuePayload::Body(_) | ValuePayload::Boolean(_) => 1,
        other => panic!("a body or a family, got {}", other.kind_name()),
    }
}

/// **A ring is one placement whose spin reads the index**: twelve
/// copies, each one `Arc` with the block's (built once and mapped,
/// §10 row 16), each member's names `Member { (k), of }`, and member
/// `k` turned `k/12` of a turn. Red if the placement is evaluated once,
/// a member is built anew, or a member's name drops its index.
#[test]
fn a_ring_is_one_placement_whose_spin_reads_the_index() {
    let (doc, the_block, ring) = ring(12);
    let ev = run(&doc);
    assert_eq!(bodies(&ev, ring), 12, "one copy per value of k");
    let ValuePayload::Instances(copies) = &ev.value(ring).unwrap().payload else {
        unreachable!()
    };
    let ValuePayload::Body(source) = &ev.value(the_block).unwrap().payload else {
        panic!("the block is a body")
    };
    for (k, copy) in copies.iter().enumerate() {
        assert!(
            std::sync::Arc::ptr_eq(copy.shared(), source.shared()),
            "member {k} shares the block's one body"
        );
    }
    let k = doc.var_named("k").unwrap();
    let table = fixture::table(&ev, ring);
    let mut at: Vec<u32> = table
        .iter()
        .filter_map(|(name, _)| match name.path.as_slice() {
            [RoleSeg::Member { at, .. }] => {
                assert_eq!(at.len(), 1, "one index");
                assert_eq!(at[0].0, k, "keyed by k's id");
                Some(at[0].1)
            }
            _ => None,
        })
        .collect();
    at.sort_unstable();
    at.dedup();
    assert_eq!(at, (0..12).collect::<Vec<u32>>(), "every member named");
    // Member 3 is a quarter turn, exactly: the block's span in x is
    // member 3's span in y.
    let three = fixture::in_space(&copies[3]);
    for (v, _) in three.vertices() {
        let p = fixture::point(&three, v);
        assert!((4.0..=5.0).contains(&p.y), "member 3 stands on +y: {p:?}");
        assert!((-0.5..=0.5).contains(&p.x), "and across x: {p:?}");
    }
}

/// **A union of the family fuses it**; an indexed read `ring[k]` reads
/// one member.
#[test]
fn a_union_reads_the_family_whole_and_an_indexed_read_one_member() {
    let (doc, _, ring) = ring(4);
    let (doc, fused) = insert(
        doc,
        Node::Union {
            members: Bodies::Family(ring.into()),
        },
    );
    let (doc, one) = insert(
        doc,
        Node::Union {
            members: Bodies::Spelled(vec![editor_core::BodyRead::indexed(ring, vec![Formula::count(2)])]),
        },
    );
    let ev = run(&doc);
    assert_eq!(bodies(&ev, ring), 4);
    assert_eq!(bodies(&ev, fused), 1, "the union is one body: {:?}", ev.result(fused));
    assert_eq!(bodies(&ev, one), 1, "{:?}", ev.result(one));
}
