//! **A boolean's seam names each side as its own table names it, and
//! puts the two in name order** (N3): an unmerged face's parent is the
//! `From` its operand's read came in through, a merged face's is its
//! `Merged` set, at a pair as at a union.
//!
//! The rows:
//! - two prisms whose union and intersection meet along a chord between
//!   two merged faces (A's top and B's front each hold a coincident face
//!   pair), in every symmetry of the square, in both member orders, in a
//!   three-member union in every order, and with one prism split first:
//!   each builds, and the chord is the seam of the two `Merged` parents;
//! - a subtract that meets the same kind of chord, in every symmetry of
//!   the square: it builds;
//! - a box less a rotated `Transform` copy of itself, whose two operands
//!   hold the same names: its four `Crossing` vertices and the edges
//!   ending at them are each one entity's name, none tied.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::blocks::{block, failure, run};
use crate::fixture::{insert, len, on_frame, scl, table, xform};
use editor_core::{
    Bodies, Datum, EntityKind, Entry, ExtrudeSide, Node, Operand, ProfileDoc, RecipeNodeId,
    RoleSeg, SplitHalf, StableName,
};
use geom_core::Tol;

/// One symmetry of the square about the z axis: `(x, y) ↦ (sx·x, sy·y)`,
/// the two coordinates swapped first where `swap` is set.
#[derive(Clone, Copy, Debug)]
struct Sym {
    swap: bool,
    sx: f64,
    sy: f64,
}

impl Sym {
    /// The eight.
    fn all() -> Vec<Self> {
        let mut out = Vec::new();
        for swap in [false, true] {
            for sx in [1.0, -1.0] {
                for sy in [1.0, -1.0] {
                    out.push(Self { swap, sx, sy });
                }
            }
        }
        out
    }

    fn apply(self, [x, y, z]: [f64; 3]) -> [f64; 3] {
        let (x, y) = if self.swap { (y, x) } else { (x, y) };
        [self.sx * x, self.sy * y, z]
    }

    /// Whether the map reverses orientation.
    fn mirrors(self) -> bool {
        (self.sx * self.sy < 0.0) != self.swap
    }
}

/// `loop_` on the frame `(o, u, v)` under `sym`, extruded `d` along the
/// frame's normal: the profile is mirrored back where `sym` mirrors, so
/// the solid is `sym`'s image of the unmapped one.
fn prism(
    doc: ProfileDoc,
    sym: Sym,
    o: [f64; 3],
    u: [f64; 3],
    v: [f64; 3],
    loop_: &[(f64, f64)],
    d: f64,
) -> (ProfileDoc, RecipeNodeId) {
    let (o, u, v) = (sym.apply(o), sym.apply(u), sym.apply(v));
    // A mirror reverses `u × v`: draw on (v, u) with the profile's
    // coordinates swapped, which keeps the extrusion on the same side,
    // and the loop reversed, which keeps its winding.
    let (u, v, pts): (_, _, Vec<(f64, f64)>) = if sym.mirrors() {
        (v, u, loop_.iter().rev().map(|&(a, b)| (b, a)).collect())
    } else {
        (u, v, loop_.to_vec())
    };
    let (doc, p) = on_frame(doc, o, u, v, vec![pts]);
    insert(
        doc,
        Node::Extrude {
            profile: p.into(),
            distance: len(d),
            side: ExtrudeSide::Along,
        },
    )
}

const X: [f64; 3] = [1.0, 0.0, 0.0];
const Y: [f64; 3] = [0.0, 1.0, 0.0];
const Z: [f64; 3] = [0.0, 0.0, 1.0];

/// The two prisms of the chord: A's top crosses B's front along y = 0,
/// z = 1, x ∈ [1, 2], and the planes z = 1 and y = 0 each hold a
/// coincident face pair, so the chord lies between two merged faces.
fn two_prisms(doc: ProfileDoc, sym: Sym) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let a_loop = [(0., -1.), (2., -1.), (2., 0.), (4., 0.), (4., 2.), (0., 2.)];
    let b_loop = [(0., -1.), (3., -1.), (3., 1.), (1., 1.), (1., 2.), (0., 2.)];
    let (doc, a) = prism(doc, sym, [0.; 3], X, Y, &a_loop, 1.0);
    let (doc, b) = prism(doc, sym, [1., 0., 0.], Y, Z, &b_loop, 2.0);
    (doc, a, b)
}

fn spelled<R>(members: Vec<R>) -> Bodies<R> {
    Bodies::Spelled(members)
}

/// The seam edges of `id`'s table whose two sides are both merged
/// parents, each checked to be in name order.
fn merged_seams(t: &editor_core::NameTable) -> Vec<StableName> {
    let mut out = Vec::new();
    for (n, _) in t.iter() {
        if n.kind != EntityKind::Edge {
            continue;
        }
        if let Some(RoleSeg::Seam { a, b }) = n.path.first() {
            assert!(a <= b, "a seam's sides are in name order: {n:?}");
            let merged = |s: &StableName| matches!(s.path.first(), Some(RoleSeg::Merged(_)));
            if merged(a) && merged(b) {
                out.push(n.clone());
            }
        }
    }
    out
}

fn assert_built(ev: &editor_core::Evaluation<f64>, id: RecipeNodeId, what: &str) {
    if let Some(e) = failure(ev, id) {
        panic!("{what} refused: {e}");
    }
}

/// **The chord between two merged faces is their seam**, at the union
/// and the intersection of the two prisms in every symmetry of the
/// square, and the union names it alike in both member orders.
#[test]
fn a_chord_between_two_merged_faces_is_the_seam_of_their_parents() {
    for sym in Sym::all() {
        let doc = ProfileDoc::empty_derived("pair_seam_name_order_union", Tol::witness());
        let (doc, a, b) = two_prisms(doc, sym);
        let (doc, ab) = insert(
            doc,
            Node::Union {
                members: spelled(vec![a.into(), b.into()]),
            },
        );
        let (doc, ba) = insert(
            doc,
            Node::Union {
                members: spelled(vec![b.into(), a.into()]),
            },
        );
        let (doc, both) = insert(
            doc,
            Node::Intersect {
                members: spelled(vec![a.into(), b.into()]),
            },
        );
        let ev = run(&doc);
        for (id, what) in [(ab, "union"), (ba, "union, b first"), (both, "intersect")] {
            assert_built(&ev, id, &format!("{what} {sym:?}"));
        }
        for (id, what) in [(ab, "union"), (both, "intersect")] {
            assert!(
                !merged_seams(table(&ev, id)).is_empty(),
                "{what} {sym:?}: no seam between two merged parents"
            );
        }
        let (ab_id, ba_id) = (format!("{ab:?}"), format!("{ba:?}"));
        let names = |id| -> Vec<String> {
            table(&ev, id)
                .iter()
                .map(|(n, _)| format!("{n:?}").replace(&ba_id, &ab_id))
                .collect()
        };
        assert_eq!(
            names(ab),
            names(ba),
            "{sym:?}: the union's names depend on member order"
        );
    }
}

/// **A three-member union meets the chord at a fold step**, in every
/// member order, and with one prism split first: each builds.
#[test]
fn a_chord_between_two_merged_faces_builds_in_a_fold_and_after_a_split() {
    let doc = ProfileDoc::empty_derived("pair_seam_name_order_fold", Tol::witness());
    let (doc, a, b) = two_prisms(doc, Sym::all()[0]);
    let (doc, c) = block(doc, (6.0, 7.0), (0.0, 1.0), 0.0, 1.0);
    let mut unions = Vec::new();
    let mut doc = doc;
    for order in [
        [a, b, c],
        [a, c, b],
        [b, a, c],
        [b, c, a],
        [c, a, b],
        [c, b, a],
    ] {
        let (d, u) = insert(
            doc,
            Node::Union {
                members: spelled(order.iter().map(|&m| m.into()).collect()),
            },
        );
        doc = d;
        unions.push((u, order));
    }
    let (doc, tool) = insert(
        doc,
        Node::Datum(Datum::Plane {
            origin: [len(3.5), len(0.0), len(0.0)],
            normal: [scl(1.0), scl(0.0), scl(0.0)],
        }),
    );
    let (doc, split) = insert(
        doc,
        Node::Split {
            target: a.into(),
            tool: tool.into(),
        },
    );
    let below = Operand::output(split, SplitHalf::Below.port());
    let (doc, after) = insert(
        doc,
        Node::Union {
            members: spelled(vec![below.into(), b.into()]),
        },
    );
    let ev = run(&doc);
    for (u, order) in &unions {
        assert_built(&ev, *u, &format!("the union in order {order:?}"));
    }
    assert_built(&ev, after, "the union of a split half");
    assert!(
        !merged_seams(table(&ev, after)).is_empty(),
        "the split half's union holds no seam between two merged parents"
    );
}

/// **A subtract meets a chord between two merged faces**, the L-prism
/// joined to a bar less a prism through both, in every symmetry of the
/// square: it builds.
#[test]
fn a_subtracts_chord_between_two_merged_faces_builds() {
    for sym in Sym::all() {
        let doc = ProfileDoc::empty_derived("pair_seam_name_order_subtract", Tol::witness());
        let l_loop = [(0., -1.), (2., -1.), (2., 0.), (4., 0.), (4., 2.), (0., 2.)];
        let (doc, l) = prism(doc, sym, [0.; 3], X, Y, &l_loop, 1.0);
        let bar_loop = [(0., 1.), (4., 1.), (4., 2.), (0., 2.)];
        let (doc, bar) = prism(doc, sym, [0., 0., 1.], X, Y, &bar_loop, 1.0);
        let (doc, from) = insert(
            doc,
            Node::Union {
                members: spelled(vec![l.into(), bar.into()]),
            },
        );
        let tool_loop = [
            (-2., -1.),
            (0., -1.),
            (0., 2.5),
            (1., 2.5),
            (1., 1.),
            (3., 1.),
            (3., 3.),
            (-2., 3.),
        ];
        let (doc, tool) = prism(doc, sym, [1., 0., 0.], Y, Z, &tool_loop, 2.0);
        let (doc, cut) = insert(
            doc,
            Node::Subtract {
                from: from.into(),
                tool: tool.into(),
            },
        );
        let ev = run(&doc);
        assert_built(&ev, cut, &format!("the subtract {sym:?}"));
    }
}

/// **A box less a rotated copy of itself names every crossing apart.**
/// `Transform` mints no name, so both operands' tables hold the same
/// names; the subtract's sides and crossings carry the read each came
/// in through, so "from's edge crosses tool's face" and "tool's edge
/// crosses from's face" are two names: the eight `Crossing` vertices,
/// four spellings without their reads, and every row of the table are
/// each one entity's.
#[test]
fn a_subtract_of_a_transform_copy_names_each_crossing_apart() {
    let doc = ProfileDoc::empty_derived("pair_seam_name_order_copy", Tol::witness());
    let (doc, x) = block(doc, (0.0, 4.0), (0.0, 2.0), 0.0, 1.0);
    let h = std::f64::consts::FRAC_1_SQRT_2;
    let (doc, copy) = insert(
        doc,
        xform(x, [1.0, -1.0, 1.5], [h, h, 0.0], std::f64::consts::PI),
    );
    let (doc, cut) = insert(
        doc,
        Node::Subtract {
            from: x.into(),
            tool: copy.into(),
        },
    );
    let ev = run(&doc);
    assert_built(&ev, cut, "the subtract");
    let t = table(&ev, cut);
    let tied: Vec<&StableName> = t
        .iter()
        .filter(|(_, e)| matches!(e, Entry::Tied(_)))
        .map(|(n, _)| n)
        .collect();
    assert!(tied.is_empty(), "tied rows: {tied:#?}");
    let crossings: Vec<&StableName> = t
        .iter()
        .map(|(n, _)| n)
        .filter(|n| matches!(n.path.first(), Some(RoleSeg::Crossing { .. })))
        .collect();
    assert_eq!(crossings.len(), 8, "eight crossings: {crossings:#?}");
    // Each read crosses the other: four of from's edges cross tool's
    // faces, and four of tool's edges cross from's.
    let read_of = |n: &StableName| match n.path.first() {
        Some(RoleSeg::From { read, .. }) => *read,
        other => panic!("a crossing cites a name carried in: {other:?}"),
    };
    let mut by_edge_read = std::collections::BTreeMap::new();
    // The same crossings with the reads taken off: what citing the
    // operands' bare names spelled, two crossings per spelling.
    let mut bare = std::collections::BTreeSet::new();
    for n in &crossings {
        let Some(RoleSeg::Crossing { edge, face, sense }) = n.path.first() else {
            unreachable!("filtered to crossings");
        };
        assert_ne!(
            read_of(edge),
            read_of(face),
            "{n:?} crosses its own operand"
        );
        *by_edge_read.entry(read_of(edge)).or_insert(0) += 1;
        let inner = |n: &StableName| match n.path.first() {
            Some(RoleSeg::From { of, .. }) => format!("{of:?}"),
            _ => unreachable!("read above"),
        };
        bare.insert((inner(edge), inner(face), format!("{sense:?}")));
    }
    assert_eq!(
        by_edge_read.values().copied().collect::<Vec<_>>(),
        vec![4, 4],
        "four crossings each way"
    );
    assert_eq!(
        bare.len(),
        4,
        "without their reads the eight are four spellings"
    );
}
