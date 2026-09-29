//! **A name re-mapped under an id map that reorders nodes binds the same
//! entity the re-mapped document names by it.**
//!
//! `remap_name` rewrites a name's node ids through a map that need not
//! keep their order (the split's map follows document order). Every
//! name-ordered position may then come out reordered, and a rank along
//! a seam line reads from the other end wherever that line's pair comes
//! out swapped — including a line written in a name the ranked name
//! EMBEDS: a pair boolean's pieces of a union's seam are
//! `[FromA(<union seam>), OrderAlong]`, and reordering the union's
//! members reorders that embedded seam.
//!
//! Each scenario builds one document with its blocks created in one
//! order, and the same document with them created in every other order,
//! so the ids differ and the map between the two reorders them. Every
//! name of every op's table in the first, re-mapped, must bind the same
//! geometry in the second.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus::body_of;
use crate::docm7_union_declare::{block, failure, run};
use crate::fixture::{ends, face_vertices, insert, point, table};

use std::collections::{BTreeMap, BTreeSet};

use editor_core::{
    BooleanOp, EntityKey, Entry, Evaluation, Node, NodeMap, ProfileDoc, RecipeNodeId, RoleSeg,
    StableName, StepMap, remap_name,
};
use geom_core::Tol;

type B = ((f64, f64), (f64, f64), (f64, f64));

#[derive(Clone, Copy)]
enum R {
    Blk(usize),
    Op(usize),
}

#[derive(Clone, Copy)]
enum Op {
    Union(&'static [R]),
    Pair(BooleanOp, R, R),
}

struct Built {
    doc: ProfileDoc,
    /// Every node each block and op added, keyed by that block or op.
    keyed: BTreeMap<String, Vec<RecipeNodeId>>,
    ops: Vec<RecipeNodeId>,
}

fn added(before: &ProfileDoc, after: &ProfileDoc) -> Vec<RecipeNodeId> {
    let old: BTreeSet<_> = before.order().iter().copied().collect();
    after
        .order()
        .iter()
        .copied()
        .filter(|i| !old.contains(i))
        .collect()
}

fn build(blocks: &[B], ops: &[Op], creation: &[usize]) -> Built {
    let mut doc = ProfileDoc::empty_derived("remap-reorders-ids", Tol::witness());
    let mut keyed = BTreeMap::new();
    let mut ids = vec![RecipeNodeId(0); blocks.len()];
    for &i in creation {
        let (x, y, z) = blocks[i];
        let before = doc.clone();
        let (d, id) = block(doc, x, y, z.0, z.1 - z.0);
        keyed.insert(format!("b{i}"), added(&before, &d));
        doc = d;
        ids[i] = id;
    }
    let mut out: Vec<RecipeNodeId> = Vec::new();
    for (k, op) in ops.iter().enumerate() {
        let r = |r: R, out: &[RecipeNodeId]| match r {
            R::Blk(i) => ids[i],
            R::Op(j) => out[j],
        };
        let before = doc.clone();
        let node = match *op {
            Op::Union(ms) => Node::Union {
                members: ms.iter().map(|&m| r(m, &out)).collect(),
                declare: None,
            },
            Op::Pair(op, a, b) => Node::Boolean {
                op,
                a: r(a, &out),
                b: r(b, &out),
                declare: None,
            },
        };
        let (d, id) = insert(doc, node);
        keyed.insert(format!("o{k}"), added(&before, &d));
        doc = d;
        out.push(id);
    }
    Built {
        doc,
        keyed,
        ops: out,
    }
}

fn micro(x: f64) -> i64 {
    (x * 1e6).round() as i64
}

/// What `e` binds at `id`, as sorted vertex positions.
fn geometry(ev: &Evaluation<f64>, id: RecipeNodeId, e: &Entry) -> String {
    let body = body_of(ev, id);
    let at = |v| {
        let p = point(body, v);
        (micro(p.x), micro(p.y), micro(p.z))
    };
    let keys: Vec<EntityKey> = match e {
        Entry::Unique(r) => vec![r.key],
        Entry::Tied(rs) => rs.iter().map(|r| r.key).collect(),
    };
    let mut out: Vec<String> = keys
        .into_iter()
        .map(|k| match k {
            EntityKey::Vertex(v) => format!("{:?}", at(v)),
            EntityKey::Edge(e) => {
                let mut s = ends(body, e).map(at);
                s.sort_unstable();
                format!("{s:?}")
            }
            EntityKey::Face(f) => {
                let mut s: Vec<_> = face_vertices(body, f).into_iter().map(at).collect();
                s.sort_unstable();
                format!("face {s:?}")
            }
            EntityKey::Body => "body".into(),
        })
        .collect();
    out.sort();
    out.join(" | ")
}

fn orders(n: usize) -> Vec<Vec<usize>> {
    if n == 0 {
        return vec![vec![]];
    }
    let mut out = Vec::new();
    for p in orders(n - 1) {
        for i in 0..=p.len() {
            let mut q = p.clone();
            q.insert(i, n - 1);
            out.push(q);
        }
    }
    out
}

/// Whether `n` is a union's seam (its sides are its own node's names)
/// whose sides the re-map to `n2` swapped: the scenario is shown to
/// reorder something, and the union's seams to be read by name.
fn union_seam_swapped(n: &StableName, n2: &StableName, map: &NodeMap, steps: &StepMap) -> bool {
    match (n.path.first(), n2.path.first()) {
        (Some(RoleSeg::Seam { a, b }), Some(RoleSeg::Seam { a: a2, .. })) if a.node == n.node => {
            remap_name(b, map, steps).expect("covered") == **a2 && a != b
        }
        _ => false,
    }
}

/// Whether `n` is a PAIR boolean's seam whose sides the re-map to `n2`
/// swapped. Never: a pair's seam is sided.
fn pair_seam_swapped(n: &StableName, n2: &StableName, map: &NodeMap, steps: &StepMap) -> bool {
    match (n.path.first(), n2.path.first()) {
        (Some(RoleSeg::Seam { a, .. }), Some(RoleSeg::Seam { a: a2, .. })) if a.node != n.node => {
            remap_name(a, map, steps).expect("covered") != **a2
        }
        _ => false,
    }
}

/// Re-maps every name of every op of `blocks`/`ops` from the document
/// built in id order to the one built in each other order. Answers the
/// wrong binds, the dangling names, and whether some ranked name's
/// re-map reordered a seam and some union seam's own sides swapped.
fn remap_every_order(blocks: &[B], ops: &[Op]) -> (Vec<String>, Vec<String>, bool, bool) {
    let first: Vec<usize> = (0..blocks.len()).collect();
    let b1 = build(blocks, ops, &first);
    let ev1 = run(&b1.doc);
    let (mut wrong, mut dangle) = (Vec::new(), Vec::new());
    let (mut ranked_moved, mut union_swapped) = (false, false);
    for order in orders(blocks.len()) {
        if order == first {
            continue;
        }
        let b2 = build(blocks, ops, &order);
        let ev2 = run(&b2.doc);
        let mut map = NodeMap::new();
        let mut steps = StepMap::new();
        for (k, v1) in &b1.keyed {
            for (x, y) in v1.iter().zip(&b2.keyed[k]) {
                map.insert(*x, *y);
                // A profile's steps are the same steps in both builds,
                // minted in a different order.
                if let (Some(Node::Profile(p1)), Some(Node::Profile(p2))) =
                    (b1.doc.node(*x), b2.doc.node(*y))
                {
                    steps.extend(
                        p1.ids
                            .iter()
                            .flatten()
                            .copied()
                            .zip(p2.ids.iter().flatten().copied()),
                    );
                }
            }
        }
        for (&t1, &t2) in b1.ops.iter().zip(&b2.ops) {
            assert!(failure(&ev1, t1).is_none() && failure(&ev2, t2).is_none());
            let (tab1, tab2) = (table(&ev1, t1), table(&ev2, t2));
            for (n, e) in tab1.iter() {
                let n2 = remap_name(n, &map, &steps).expect("the map covers every node");
                union_swapped |= union_seam_swapped(n, &n2, &map, &steps);
                assert!(
                    !pair_seam_swapped(n, &n2, &map, &steps),
                    "{order:?}: a pair boolean's seam swapped its sides: {n:?} -> {n2:?}"
                );
                ranked_moved |= matches!(
                    (n.path.last(), n2.path.last()),
                    (
                        Some(RoleSeg::Fragment(editor_core::Qualifier::OrderAlong { rank: r1, .. })),
                        Some(RoleSeg::Fragment(editor_core::Qualifier::OrderAlong { rank: r2, .. })),
                    ) if r1 != r2
                );
                match tab2.lookup(&n2) {
                    None => dangle.push(format!("{order:?}: {n:?} -> {n2:?}")),
                    Some(e2) => {
                        let (g1, g2) = (geometry(&ev1, t1, e), geometry(&ev2, t2, e2));
                        if g1 != g2 {
                            wrong.push(format!("{order:?}: {n:?} -> {n2:?}: {g1} vs {g2}"));
                        }
                    }
                }
            }
        }
    }
    (wrong, dangle, ranked_moved, union_swapped)
}

const A: B = ((0.0, 1.0), (0.0, 1.0), (0.0, 1.0));
const BP: B = ((0.5, 1.5), (0.2, 0.8), (0.2, 0.8));
/// A notch across the union(A, BP) seam line x = 1, y = 0.2.
const X: B = ((0.9, 1.1), (0.1, 0.3), (0.45, 0.55));
/// A second notch across it: three pieces.
const X2: B = ((0.9, 1.1), (0.1, 0.3), (0.65, 0.7));

fn assert_every_order(what: &str, blocks: &[B], ops: &[Op]) {
    let (wrong, dangle, ranked_moved, union_swapped) = remap_every_order(blocks, ops);
    assert!(
        wrong.is_empty(),
        "{what}: {} wrong binds:\n{wrong:#?}",
        wrong.len()
    );
    assert!(
        dangle.is_empty(),
        "{what}: {} dangling:\n{dangle:#?}",
        dangle.len()
    );
    assert!(
        ranked_moved,
        "{what}: no rank was re-read, so the row pins nothing"
    );
    assert!(union_swapped, "{what}: no union seam's sides swapped");
}

/// The union is the A operand of a subtract that notches its seam: the
/// subtract's pieces of that seam are ranked along a line written in
/// the union's name they embed.
#[test]
fn a_pair_booleans_pieces_of_a_union_seam_follow_it_as_the_a_operand() {
    use R::{Blk, Op as O};
    assert_every_order(
        "u_in_pairA",
        &[A, BP, X],
        &[
            Op::Union(&[Blk(0), Blk(1)]),
            Op::Pair(BooleanOp::Subtract, O(0), Blk(2)),
        ],
    );
}

/// The same with the union as the B operand.
#[test]
fn a_pair_booleans_pieces_of_a_union_seam_follow_it_as_the_b_operand() {
    use R::{Blk, Op as O};
    assert_every_order(
        "u_in_pairB",
        &[A, BP, X],
        &[
            Op::Union(&[Blk(0), Blk(1)]),
            Op::Pair(BooleanOp::Union, Blk(2), O(0)),
        ],
    );
}

/// A second cut of the pieces: three pieces, ranked through two
/// wrappers.
#[test]
fn a_second_cut_of_a_union_seams_pieces_follows_it_too() {
    use R::{Blk, Op as O};
    assert_every_order(
        "u_in_pairA2",
        &[A, BP, X, X2],
        &[
            Op::Union(&[Blk(0), Blk(1)]),
            Op::Pair(BooleanOp::Subtract, O(0), Blk(2)),
            Op::Pair(BooleanOp::Subtract, O(1), Blk(3)),
        ],
    );
}
