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
//!   each builds, the chord is the seam of the two `Merged` parents beside
//!   it, and the symmetries and member orders name it alike;
//! - a subtract that meets the same kind of chord, in every symmetry of
//!   the square: it builds, and names its seams beside merged parents
//!   alike;
//! - a box less a rotated copy of itself placed against it, whose
//!   copy's names are the box's qualified `Placed`: its four `Crossing`
//!   vertices and the edges ending at them are each one entity's name,
//!   none tied.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::blocks::{block, failure, run};
use crate::fixture::{
    axes_at, desc, ends, face_vertices, in_world_square, insert, len, out, plane_on, point, table,
    xform,
};
use editor_core::{
    Bodies, EntityKey, EntityKind, Entry, Node, Operand, ProfileDoc, RecipeNodeId, RoleSeg,
    SplitHalf, StableName,
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

    /// The point `p` maps to under `self`'s inverse.
    fn unapply(self, [x, y, z]: [f64; 3]) -> [f64; 3] {
        let (x, y) = (x / self.sx, y / self.sy);
        let (x, y) = if self.swap { (y, x) } else { (x, y) };
        [x, y, z]
    }

    /// Whether the map reverses orientation.
    fn mirrors(self) -> bool {
        (self.sx * self.sy < 0.0) != self.swap
    }
}

/// `loop_` on its own xy, extruded `d` along its own z, its copy standing
/// in the world on the frame `(o, u, v)` under `sym`: the profile is
/// mirrored back where `sym` mirrors, so the copy is `sym`'s image of
/// the unmapped one.
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
    let (doc, p) = insert(doc, Node::Profile(desc(vec![pts])));
    let (doc, body) = insert(
        doc,
        Node::Extrude {
            profile: p.into(),
            distance: len(d),
        },
    );
    in_world_square(doc, body, &axes_at(o, u, v))
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

/// A name with its trailing `Fragment` discriminators taken off: the
/// parent a piece is a piece of.
fn base(n: &StableName) -> StableName {
    let mut b = n.clone();
    while matches!(b.path.last(), Some(RoleSeg::Fragment(_))) && b.path.len() > 1 {
        b.path.pop();
    }
    b
}

/// One seam edge as the symmetry does not move it: its two ends and,
/// per side, the plane of the face beside it and the operand reads the
/// side's name cites, all in the unmapped frame and in a fixed order.
type SeamSig = (Vec<String>, Vec<(String, bool, Vec<char>)>);

/// **The seam edges of `id`, each checked against the body and read as
/// [`SeamSig`]s**, `pred` choosing which. Each one's two sides are the
/// parents of the two faces beside it, as `id`'s table names those
/// faces: a stable but wrong side fails here, whatever the symmetry.
/// `reads` letters each operand read a side may cite.
fn seam_sigs(
    ev: &editor_core::Evaluation<f64>,
    id: RecipeNodeId,
    sym: Sym,
    reads: &[(editor_core::VarId, char)],
    pred: impl Fn(&StableName, &StableName) -> bool,
) -> Vec<SeamSig> {
    let t = table(ev, id);
    let body = crate::corpus::body_of(ev, id);
    let face_name = |f: topo::FaceKey| -> StableName {
        let rows: Vec<&StableName> = t
            .iter()
            .filter_map(|(n, e)| match e {
                Entry::Unique(r) if r.key == EntityKey::Face(f) => Some(n),
                _ => None,
            })
            .collect();
        let [n] = rows[..] else {
            panic!("face {f:?} has {} rows", rows.len());
        };
        n.clone()
    };
    let at = |p: geom_core::Point3<f64>| {
        // `+ 0.0` reads a negative zero as zero.
        let [x, y, z] = sym.unapply(p.to_array()).map(|c| c + 0.0);
        format!("({x:.6}, {y:.6}, {z:.6})")
    };
    // The one coordinate a planar face holds constant, unmapped.
    let plane = |f: topo::FaceKey| -> String {
        let pts: Vec<[f64; 3]> = face_vertices(&body, f)
            .into_iter()
            .map(|v| sym.unapply(point(&body, v).to_array()))
            .collect();
        for (axis, i) in [('x', 0), ('y', 1), ('z', 2)] {
            if pts.iter().all(|q| (q[i] - pts[0][i]).abs() < 1e-9) {
                return format!("{axis} = {:.6}", pts[0][i] + 0.0);
            }
        }
        panic!("face {f:?} is not axis-planar: {pts:?}")
    };
    let letters = |side: &StableName| -> Vec<char> {
        // Every read the name cites, at any depth, spells itself in it.
        let said = format!("{side:?}");
        let mut out: Vec<char> = reads
            .iter()
            .filter(|(v, _)| said.contains(&format!("{v:?}")))
            .map(|&(_, c)| c)
            .collect();
        out.sort_unstable();
        out
    };
    let mut out = Vec::new();
    for (n, e) in t.iter() {
        let (Some(RoleSeg::Seam { a, b }), Entry::Unique(r)) = (n.path.first(), e) else {
            continue;
        };
        if n.kind != EntityKind::Edge || !pred(a, b) {
            continue;
        }
        assert!(a <= b, "a seam's sides are in name order: {n:?}");
        let EntityKey::Edge(edge) = r.key else {
            panic!("an edge row holds an edge: {n:?}");
        };
        let (f0, f1) = topo::readback::edge_sides(&body, edge).unwrap().faces();
        let mut beside = [base(&face_name(f0)), base(&face_name(f1))];
        beside.sort();
        assert_eq!(
            beside,
            [(**a).clone(), (**b).clone()],
            "{n:?}: the seam's sides are not the parents of the faces beside it"
        );
        let mut ends: Vec<String> = ends(&body, edge)
            .iter()
            .map(|&v| at(point(&body, v)))
            .collect();
        ends.sort();
        let mut sides: Vec<(String, bool, Vec<char>)> = [f0, f1]
            .iter()
            .map(|&f| {
                let side = base(&face_name(f));
                let merged = matches!(side.path.first(), Some(RoleSeg::Merged(_)));
                (plane(f), merged, letters(&side))
            })
            .collect();
        sides.sort();
        out.push((ends, sides));
    }
    out.sort();
    out
}

fn both_merged(a: &StableName, b: &StableName) -> bool {
    let merged = |s: &StableName| matches!(s.path.first(), Some(RoleSeg::Merged(_)));
    merged(a) && merged(b)
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
    let (mut union_sigs, mut meet_sigs) = (None, None);
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
        let reads = [(out(&doc, a), 'a'), (out(&doc, b), 'b')];
        for (id, what, slot) in [
            (ab, "union", &mut union_sigs),
            (both, "intersect", &mut meet_sigs),
        ] {
            let sigs = seam_sigs(&ev, id, sym, &reads, both_merged);
            assert!(
                !sigs.is_empty(),
                "{what} {sym:?}: no seam between two merged parents"
            );
            match slot {
                None => *slot = Some(sigs),
                Some(first) => assert_eq!(
                    *first, sigs,
                    "{what} {sym:?}: the chord is named otherwise than in the first symmetry"
                ),
            }
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
    // The chord itself: A's top meets B's front along y = 0, z = 1.
    let chord = vec![
        "(1.000000, 0.000000, 1.000000)".to_owned(),
        "(2.000000, 0.000000, 1.000000)".to_owned(),
    ];
    let union_sigs = union_sigs.unwrap();
    assert!(
        union_sigs.iter().any(|(ends, _)| *ends == chord),
        "the union names no seam along the chord: {union_sigs:#?}"
    );
}

/// **A three-member union meets the chord at a fold step**, in every
/// member order, and with one prism split first: each builds, the
/// chord is the seam of the parents beside it, and every member order
/// publishes one table.
#[test]
fn a_chord_between_two_merged_faces_builds_in_a_fold_and_after_a_split() {
    let doc = ProfileDoc::empty_derived("pair_seam_name_order_fold", Tol::witness());
    let (doc, a, b) = two_prisms(doc, Sym::all()[0]);
    let (doc, c) = block(doc, (6.0, 7.0), (0.0, 1.0), 1.0);
    let (doc, c) = in_world_square(doc, c, &editor_core::Frame::IDENTITY);
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
    // `a` stands at the identity: its own coordinates are the world's.
    let tool = plane_on(&doc, a, [3.5, 0.0, 0.0], [1.0, 0.0, 0.0]);
    let (doc, split) = insert(
        doc,
        Node::Split {
            target: a.into(),
            tool,
        },
    );
    let below = Operand::output(split, SplitHalf::Below.port());
    let below_read = doc
        .output(split, SplitHalf::Below.port())
        .expect("the split defines its lower half");
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
    // One table in every member order, read with the union's own id
    // masked, and its chord the seam of the parents beside it.
    let sym = Sym::all()[0];
    let reads = [
        (out(&doc, a), 'a'),
        (out(&doc, b), 'b'),
        (out(&doc, c), 'c'),
    ];
    let (first, _) = unions[0];
    let first_id = format!("{first:?}");
    let names = |u: RecipeNodeId| -> Vec<String> {
        let id = format!("{u:?}");
        let mut v: Vec<String> = table(&ev, u)
            .iter()
            .map(|(n, _)| format!("{n:?}").replace(&id, &first_id))
            .collect();
        v.sort();
        v
    };
    for (u, order) in &unions {
        assert!(
            !seam_sigs(&ev, *u, sym, &reads, both_merged).is_empty(),
            "the union in order {order:?} holds no seam between two merged parents"
        );
        assert_eq!(
            names(*u),
            names(first),
            "{order:?}: the union's names depend on member order"
        );
    }
    assert_built(&ev, after, "the union of a split half");
    let split_reads = [(below_read, 'a'), (out(&doc, b), 'b')];
    assert!(
        !seam_sigs(&ev, after, sym, &split_reads, both_merged).is_empty(),
        "the split half's union holds no seam between two merged parents"
    );
}

/// **A subtract meets a chord between two merged faces**, the L-prism
/// joined to a bar less a prism through both, in every symmetry of the
/// square: it builds, each seam beside a merged parent cites the
/// parents of the faces beside it, and the symmetries name them alike.
#[test]
fn a_subtracts_chord_between_two_merged_faces_is_named_alike_in_every_symmetry() {
    let mut first = None;
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
        let reads = [(out(&doc, from), 'f'), (out(&doc, tool), 't')];
        let merged_beside = |a: &StableName, b: &StableName| {
            [a, b]
                .iter()
                .any(|s| matches!(s.path.first(), Some(RoleSeg::Merged(_))))
        };
        let sigs = seam_sigs(&ev, cut, sym, &reads, merged_beside);
        assert!(!sigs.is_empty(), "{sym:?}: no seam beside a merged parent");
        match &first {
            None => first = Some(sigs),
            Some(f) => assert_eq!(
                *f, sigs,
                "{sym:?}: the seams beside merged parents are named otherwise than in the first \
                 symmetry"
            ),
        }
    }
}

/// **A box less a rotated copy of itself names every crossing apart.**
/// The copy's names are the box's, each qualified `Placed`; the
/// subtract's sides and crossings carry the read each came in through,
/// so "from's edge crosses tool's face" and "tool's edge crosses from's
/// face" are two names: the eight `Crossing` vertices, four spellings
/// without their reads, and every row of the table are each one
/// entity's.
#[test]
fn a_subtract_of_a_transform_copy_names_each_crossing_apart() {
    let doc = ProfileDoc::empty_derived("pair_seam_name_order_copy", Tol::witness());
    let (doc, x) = block(doc, (0.0, 4.0), (0.0, 2.0), 1.0);
    let h = std::f64::consts::FRAC_1_SQRT_2;
    let (doc, copy) = xform(doc, x, [1.0, -1.0, 1.5], [h, h, 0.0], std::f64::consts::PI);
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
    // The same crossings with the reads taken off, and the copy's
    // `Placed` with them: what citing the operands' bare names spelled,
    // two crossings per spelling.
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
