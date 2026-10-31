//! **A split face's pieces are named by the divider walls they border**
//! (N2's `Borders`), in a union in every member order and in a pair
//! boolean alike.
//!
//! An obstacle is a connected part of a face's region that no piece of
//! it holds; a divider is one that borders two or more pieces; a piece
//! is qualified by the walls it meets a divider along, each cited by its
//! parent. The fixture table below states each split parent's pieces as
//! the sets of walls they border, by the wall's member and plane, and
//! holds every member order to it and to one table. The sets were
//! written against a rasterized planar truth of the same definition
//! (the obstacle-mechanism measurement), not read off the emitter.
//!
//! A boss standing on one piece, a notch in one, a feature on one piece
//! aligned with a divider elsewhere: each is an obstacle bordering one
//! piece, so none is cited. Further rows hold a tie that nothing divides
//! as a tie, a split's names under an edit that moves its divider, the
//! names of a tie's divided candidate under an edit that moves the
//! divider onto the other, a curved divider to the pair boolean's own
//! answer, and a merged wall a slot divides.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::{BTreeMap, BTreeSet};

use crate::blocks::{block, block_on, failure, run};
use crate::corpus::body_of;
use crate::emit_shared_rim_several::permutations;
use crate::emit_union_flush_names::parent_of;
use crate::fixture::{axes_at, slid, value_slot};
use crate::fixture::{face_vertices, insert, len, on_frame, out, step, table};

use editor_core::{
    Coord, DocEdit, EntityKey, EntityKind, Entry, Evaluation, LoopProgram, NameTable, NamedCell,
    NamingError, Node, NodeErrorKind, ProfileDoc, ProfileProgram, Qualifier, RecipeNodeId, RoleSeg,
    StableName, TargetAxis,
};
use geom_core::Tol;

/// `doc` with the placement `placed` slid to `to` along `axis` of its
/// target's own frame: a slide is a coordinate on the target's own axes,
/// its start cap where the identity orientation puts the copy's.
fn moved(doc: ProfileDoc, placed: RecipeNodeId, axis: TargetAxis, to: f64) -> ProfileDoc {
    let slot = value_slot(&doc, placed, Coord::Slide(axis));
    let by = to;
    step(
        doc,
        DocEdit::SetParam {
            node: placed,
            slot,
            value: len(by).into(),
            fresh: Vec::new(),
        },
    )
    .0
}

/// A union of `members` in the order `order` picks.
fn union_of(
    doc: ProfileDoc,
    members: &[RecipeNodeId],
    order: &[usize],
) -> (ProfileDoc, RecipeNodeId) {
    let ordered: Vec<RecipeNodeId> = order.iter().map(|&i| members[i]).collect();
    crate::fixture::union_over(doc, &ordered)
}

/// Each uniquely named face of `union`'s table → the centroid of its
/// vertices.
fn face_centroids(ev: &Evaluation<f64>, union: RecipeNodeId) -> BTreeMap<StableName, [f64; 3]> {
    let body = body_of(ev, union);
    let mut out = BTreeMap::new();
    for (name, entry) in table(ev, union).iter() {
        let (EntityKind::Face, Entry::Unique(e)) = (name.kind, entry) else {
            continue;
        };
        let EntityKey::Face(f) = e.key else { continue };
        let vs = face_vertices(&body, f);
        let mut c = [0.0; 3];
        for &v in &vs {
            let p = body.get_point(body.get_vertex(v).unwrap().point).unwrap();
            c[0] += p.x;
            c[1] += p.y;
            c[2] += p.z;
        }
        let n = vs.len() as f64;
        out.insert(name.clone(), [c[0] / n, c[1] / n, c[2] / n]);
    }
    out
}

/// One member of a fixture.
#[derive(Clone)]
enum Mem {
    /// An axis-aligned block: x, y, (z0, dz).
    Block((f64, f64), (f64, f64), (f64, f64)),
    /// A prism: plan loops at z0, extruded dz.
    Prism(Vec<Vec<(f64, f64)>>, f64, f64),
    /// A profile in the xz plane at y = y1, extruded toward −y by dy.
    Side(Vec<(f64, f64)>, f64, f64),
}

/// `m`, drawn in its own coordinates, and placed against `root` where it
/// stands — or, with no root, the root itself, which stands at its own
/// origin. Returns the operand: the placement, or the root.
fn add(doc: ProfileDoc, root: Option<RecipeNodeId>, m: &Mem) -> (ProfileDoc, RecipeNodeId) {
    let identity = ([0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
    let ((doc, body), at) = match m {
        Mem::Block(x, y, (z0, dz)) => (
            block(doc, *x, *y, *dz),
            axes_at([0.0, 0.0, *z0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]),
        ),
        Mem::Prism(loops, z0, dz) => {
            let (doc, p) = on_frame(doc, identity.0, identity.1, identity.2, loops.clone());
            (
                extrude(doc, p, *dz),
                axes_at([0.0, 0.0, *z0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]),
            )
        }
        Mem::Side(pts, y1, dy) => {
            let (doc, p) = on_frame(doc, identity.0, identity.1, identity.2, vec![pts.clone()]);
            (
                extrude(doc, p, *dy),
                axes_at([0.0, *y1, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0]),
            )
        }
    };
    match root {
        Some(root) => crate::fixture::against(doc, root, body, &at),
        None => {
            assert!(
                at == editor_core::Frame::IDENTITY,
                "a root member stands at its own origin"
            );
            (doc, body)
        }
    }
}

/// `profile` extruded `distance` up its own `z`.
fn extrude(doc: ProfileDoc, profile: RecipeNodeId, distance: f64) -> (ProfileDoc, RecipeNodeId) {
    insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(distance),
        },
    )
}

/// A regular `n`-gon about `(cx, cy)` of circumradius `r`, a vertex
/// half a step off the x axis; clockwise when `rev` (a bore).
fn ngon(cx: f64, cy: f64, r: f64, n: usize, rev: bool) -> Vec<(f64, f64)> {
    let mut v: Vec<(f64, f64)> = (0..n)
        .map(|k| {
            let t = 2.0 * core::f64::consts::PI * (k as f64 + 0.5) / n as f64;
            (cx + r * t.cos(), cy + r * t.sin())
        })
        .collect();
    if rev {
        v.reverse();
    }
    v
}

/// A 16-gon tube about (1.5, 1.0), radii 0.6 and 0.4, from z 0.5 for 1.5.
fn ptube() -> Mem {
    Mem::Prism(
        vec![
            ngon(1.5, 1.0, 0.6, 16, false),
            ngon(1.5, 1.0, 0.4, 16, true),
        ],
        0.5,
        1.5,
    )
}

/// The plate every union fixture stands on: `[0,3] × [0,2] × [0,1]`.
fn plate() -> Mem {
    Mem::Block((0.0, 3.0), (0.0, 2.0), (0.0, 1.0))
}

/// A member face's label: its member's label and its plane, `x=1.40`
/// for an axis-aligned one and `slant` otherwise.
fn plane_label(ev: &Evaluation<f64>, node: RecipeNodeId, of: &StableName) -> String {
    let key = match table(ev, node).lookup(of) {
        Some(Entry::Unique(e)) => e.key,
        Some(Entry::Tied(es)) => es[0].key,
        None => panic!("{of:?} is not named by node {node:?}"),
    };
    let EntityKey::Face(f) = key else {
        panic!("{of:?} is no face")
    };
    let body = body_of(ev, node);
    let s = body.get_surface(body.get_face(f).unwrap().surface).unwrap();
    let topo::Surface::Plane { origin, normal, .. } = s else {
        return "curved".to_owned();
    };
    let (n, o) = (normal.to_array(), origin.to_array());
    for (i, axis) in ["x", "y", "z"].into_iter().enumerate() {
        if (n[i].abs() - 1.0).abs() < 1e-9 {
            return format!("{axis}={:.2}", o[i]);
        }
    }
    "slant".to_owned()
}

/// A face name's label, `member:plane`, read in the table of the node
/// that minted what it cites: a `From` a labelled member's read, read
/// in that member's table; a `Merged` of them; a `From` any other read,
/// by what it carries; or an operand's own name.
fn label(
    ev: &Evaluation<f64>,
    doc: &ProfileDoc,
    labels: &BTreeMap<RecipeNodeId, &str>,
    n: &StableName,
) -> String {
    match n.path.as_slice() {
        [RoleSeg::From { read, of }] => {
            match doc
                .defined_by(*read)
                .map(|(member, _)| member)
                .filter(|member| labels.contains_key(member))
            {
                Some(member) => format!("{}:{}", labels[&member], plane_label(ev, member, of)),
                None => label(ev, doc, labels, of),
            }
        }
        [RoleSeg::Merged(set)] => set
            .iter()
            .map(|c| label(ev, doc, labels, c))
            .collect::<Vec<_>>()
            .join("+"),
        _ => {
            let at = holder(doc, labels, n);
            format!("{}:{}", labels[&at], plane_label(ev, at, n))
        }
    }
}

/// The labelled member that holds `n`, a name minted at `n.node`: that
/// node itself, or a placement of it, whose copy carries its source's
/// names verbatim.
fn holder(doc: &ProfileDoc, labels: &BTreeMap<RecipeNodeId, &str>, n: &StableName) -> RecipeNodeId {
    labels
        .keys()
        .copied()
        .find(|&id| {
            id == n.node
                || matches!(doc.node(id), Some(editor_core::Node::Place { source, .. })
                    if doc.operation_of(source.read) == Some(n.node))
        })
        .unwrap_or_else(|| panic!("no labelled member holds {n:?}"))
}

/// `node`'s split parents, each as the sorted list of its pieces' wall
/// sets (a tied row counts once per candidate).
fn splits(
    ev: &Evaluation<f64>,
    doc: &ProfileDoc,
    node: RecipeNodeId,
    labels: &BTreeMap<RecipeNodeId, &str>,
) -> BTreeMap<String, Vec<Vec<String>>> {
    let mut out: BTreeMap<String, Vec<Vec<String>>> = BTreeMap::new();
    for (name, entry) in table(ev, node).iter() {
        let Some(RoleSeg::Fragment(Qualifier::Borders(walls))) = name.path.last() else {
            continue;
        };
        if name.kind != EntityKind::Face {
            continue;
        }
        let parent = label(ev, doc, labels, &parent_of(name));
        let mut set: Vec<String> = walls.iter().map(|w| label(ev, doc, labels, w)).collect();
        set.sort();
        let count = match entry {
            Entry::Unique(_) => 1,
            Entry::Tied(es) => es.len(),
        };
        for _ in 0..count {
            out.entry(parent.clone()).or_default().push(set.clone());
        }
    }
    for sets in out.values_mut() {
        sets.sort();
    }
    out
}

/// One fixture: its members by label, a pair boolean over the first two
/// or else a union in every member order, and each split parent's
/// expected wall sets.
struct Fixture {
    label: &'static str,
    members: Vec<(&'static str, Mem)>,
    pair: Option<PairOp>,
    expect: Vec<(&'static str, Vec<Vec<String>>)>,
}

/// Runs `fx` in every member order: each publishes, all publish one
/// table (every name binding a face with the same centroid), and the
/// split parents' wall sets are `fx.expect`. Returns the orders run.
fn check(fx: &Fixture) -> usize {
    let (doc, ids) = placed(
        ProfileDoc::empty_derived(fx.label, Tol::witness()),
        &fx.members,
    );
    let labels: BTreeMap<RecipeNodeId, &str> = ids
        .iter()
        .zip(&fx.members)
        .map(|(&id, (l, _))| (id, *l))
        .collect();
    let orders: Vec<Vec<usize>> = match fx.pair {
        Some(_) => vec![vec![0, 1]],
        None => permutations(&(0..ids.len()).collect::<Vec<_>>()),
    };
    let want: BTreeMap<String, Vec<Vec<String>>> = fx
        .expect
        .iter()
        .map(|(p, sets)| {
            let mut sets: Vec<Vec<String>> = sets
                .iter()
                .map(|s| {
                    let mut s = s.clone();
                    s.sort();
                    s
                })
                .collect();
            sets.sort();
            ((*p).to_owned(), sets)
        })
        .collect();
    let mut first: Option<(String, BTreeMap<StableName, [f64; 3]>)> = None;
    for order in &orders {
        let (d, n) = match fx.pair {
            Some(op) => insert(doc.clone(), op.node(ids[0], ids[1])),
            None => {
                let ordered: Vec<RecipeNodeId> = order.iter().map(|&i| ids[i]).collect();
                crate::fixture::union_over(doc.clone(), &ordered)
            }
        };
        let ev = run(&d);
        let at = format!("{} {order:?}", fx.label);
        assert!(failure(&ev, n).is_none(), "{at}: {:?}", failure(&ev, n));
        assert_eq!(splits(&ev, &d, n, &labels), want, "{at}: the wall sets");
        let c = face_centroids(&ev, n);
        match &first {
            None => first = Some((at, c)),
            Some((first_at, fc)) => {
                assert_eq!(
                    fc.keys().collect::<Vec<_>>(),
                    c.keys().collect::<Vec<_>>(),
                    "{first_at} against {at}: the published faces"
                );
                for (name, p) in fc {
                    let q = c[name];
                    let d = (0..3).map(|i| (p[i] - q[i]).powi(2)).sum::<f64>().sqrt();
                    assert!(d < 1e-9, "{first_at} against {at}: {name:?} moved {d}");
                }
            }
        }
    }
    orders.len()
}

/// A union fixture.
fn u(
    label: &'static str,
    members: Vec<(&'static str, Mem)>,
    expect: Vec<(&'static str, Vec<Vec<String>>)>,
) -> Fixture {
    Fixture {
        label,
        members,
        pair: None,
        expect,
    }
}

/// Which two-operand boolean node a pair fixture builds.
#[derive(Clone, Copy)]
enum PairOp {
    Union,
    Subtract,
}

impl PairOp {
    /// The node of this kind over `a` and `b`.
    fn node(self, a: RecipeNodeId, b: RecipeNodeId) -> editor_core::AuthoredNode {
        match self {
            Self::Union => Node::Union {
                members: editor_core::Bodies::Spelled(vec![a.into(), b.into()]),
            },
            Self::Subtract => Node::Subtract {
                from: a.into(),
                tool: b.into(),
            },
        }
    }
}

/// A pair-boolean fixture over the first two members.
fn pair(
    label: &'static str,
    op: PairOp,
    members: Vec<(&'static str, Mem)>,
    expect: Vec<(&'static str, Vec<Vec<String>>)>,
) -> Fixture {
    Fixture {
        label,
        members,
        pair: Some(op),
        expect,
    }
}

/// `Mem::Block` shorthand.
fn b(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> Mem {
    Mem::Block(x, y, z)
}

/// A wall set, as labels.
fn w(labels: &[&str]) -> Vec<String> {
    labels.iter().map(|l| (*l).to_owned()).collect()
}

/// The walls of the 16-gon tube `member` (see [`ptube`]) a piece
/// outside it (`bore` false) or inside its bore borders: all sixteen,
/// four of them axis-aligned, plus `more`.
fn ring(member: &str, bore: bool, more: &[&str]) -> Vec<String> {
    ring_arc(member, bore, 16, more)
}

/// `slant` of a ring's slanted walls and the named axis-aligned ones.
fn ring_arc(member: &str, bore: bool, slant: usize, more: &[&str]) -> Vec<String> {
    let axis: &[&str] = match (bore, slant) {
        (false, 16) => &["x=0.91", "x=2.09", "y=0.41", "y=1.59"],
        (true, 16) => &["x=1.11", "x=1.89", "y=0.61", "y=1.39"],
        _ => &[],
    };
    let mut out: Vec<String> = (0..slant.min(12))
        .map(|_| format!("{member}:slant"))
        .collect();
    out.extend(axis.iter().map(|a| format!("{member}:{a}")));
    out.extend(more.iter().map(|m| (*m).to_owned()));
    out
}

/// The slab through the plate's top that divides it in two.
fn slab() -> Mem {
    b((1.4, 1.6), (-1.0, 3.0), (0.5, 1.5))
}

/// The slab's bottom, inside the plate but for its two ends.
fn slab_bottom() -> (&'static str, Vec<Vec<String>>) {
    (
        "slab:z=0.50",
        vec![w(&["plate:y=0.00"]), w(&["plate:y=2.00"])],
    )
}

/// **Three overlapping strips across a plate** (the review's refusing
/// case): the three strips' footprints are ONE obstacle, so the top's
/// two pieces border the outermost walls only, and each strip's bottom
/// and the middle strip's side are divided where the plate and the
/// next strip cover them.
#[test]
fn three_overlapping_strips_are_one_divider() {
    check(&u(
        "three_strips",
        vec![
            ("plate", plate()),
            ("s1", b((1.0, 1.3), (-1.0, 3.0), (0.5, 1.3))),
            ("s2", b((1.2, 1.6), (-1.1, 3.1), (0.47, 1.53))),
            ("s3", b((1.5, 1.8), (-1.2, 3.2), (0.44, 1.16))),
        ],
        vec![
            ("plate:z=1.00", vec![w(&["s1:x=1.00"]), w(&["s3:x=1.80"])]),
            (
                "s1:z=0.50",
                vec![
                    w(&["plate:y=0.00", "s2:x=1.20"]),
                    w(&["plate:y=2.00", "s2:x=1.20"]),
                ],
            ),
            (
                "s2:z=0.47",
                vec![
                    w(&["plate:y=0.00", "s3:x=1.50"]),
                    w(&["plate:y=2.00", "s3:x=1.50"]),
                ],
            ),
            (
                "s3:x=1.50",
                vec![
                    w(&["plate:y=0.00", "s2:y=-1.10", "s2:z=0.47"]),
                    w(&["plate:y=2.00", "s2:y=3.10", "s2:z=0.47"]),
                ],
            ),
            (
                "s3:z=0.44",
                vec![w(&["plate:y=0.00"]), w(&["plate:y=2.00"])],
            ),
        ],
    ));
}

/// **A boss standing on one piece is never cited**, and a boss
/// straddling the slab is part of the divider: its walls divide the top
/// with the slab's, and both are cited.
#[test]
fn a_boss_on_one_piece_is_not_cited_and_one_straddling_the_divider_is() {
    check(&u(
        "slab_boss",
        vec![
            ("plate", plate()),
            ("slab", slab()),
            ("boss", b((0.4, 0.8), (0.8, 1.2), (0.47, 1.03))),
        ],
        vec![
            (
                "plate:z=1.00",
                vec![w(&["slab:x=1.40"]), w(&["slab:x=1.60"])],
            ),
            slab_bottom(),
        ],
    ));
    check(&u(
        "straddling_boss",
        vec![
            ("plate", plate()),
            ("slab", slab()),
            ("boss", b((1.2, 1.8), (0.8, 1.2), (0.47, 1.03))),
        ],
        vec![
            (
                "plate:z=1.00",
                vec![
                    w(&["boss:x=1.20", "boss:y=0.80", "boss:y=1.20", "slab:x=1.40"]),
                    w(&["boss:x=1.80", "boss:y=0.80", "boss:y=1.20", "slab:x=1.60"]),
                ],
            ),
            (
                "boss:y=0.80",
                vec![
                    w(&["plate:z=1.00", "slab:x=1.40"]),
                    w(&["plate:z=1.00", "slab:x=1.60"]),
                ],
            ),
            (
                "boss:y=1.20",
                vec![
                    w(&["plate:z=1.00", "slab:x=1.40"]),
                    w(&["plate:z=1.00", "slab:x=1.60"]),
                ],
            ),
            (
                "boss:z=1.50",
                vec![w(&["slab:x=1.40"]), w(&["slab:x=1.60"])],
            ),
            slab_bottom(),
        ],
    ));
}

/// **A boss whose corner touches the divider is part of it.** A
/// diamond's corner rests on the slab's wall, so its footprint meets the
/// slab's only at that point, and through its closure the two are one
/// obstacle (N2): the left piece cites the diamond's walls beside the
/// slab's.
#[test]
fn a_boss_whose_corner_touches_the_divider_is_part_of_it() {
    check(&u(
        "corner_boss",
        corner_boss_members(),
        vec![
            (
                "plate:z=1.00",
                vec![
                    w(&[
                        "boss:slant",
                        "boss:slant",
                        "boss:slant",
                        "boss:slant",
                        "slab:x=1.40",
                    ]),
                    w(&["slab:x=1.60"]),
                ],
            ),
            slab_bottom(),
        ],
    ));
}

/// The corner boss's members: the plate, the slab across it, and a
/// diamond prism whose corner edge lies in the slab's `x = 1.4` wall and
/// crosses the plate's top.
fn corner_boss_members() -> Vec<(&'static str, Mem)> {
    vec![
        ("plate", plate()),
        ("slab", slab()),
        (
            "boss",
            Mem::Prism(
                vec![vec![(1.4, 1.0), (1.0, 1.4), (0.6, 1.0), (1.0, 0.6)]],
                0.47,
                1.03,
            ),
        ),
    ]
}

/// `members` in `doc`, each placed by [`add`] against the first.
fn placed(mut doc: ProfileDoc, members: &[(&'static str, Mem)]) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let mut ids = Vec::new();
    for (_, m) in members {
        let (d, id) = add(doc, ids.first().copied(), m);
        doc = d;
        ids.push(id);
    }
    (doc, ids)
}

/// The corner boss's members in `doc` ([`placed`]).
pub(crate) fn corner_boss(doc: ProfileDoc) -> (ProfileDoc, Vec<RecipeNodeId>) {
    placed(doc, &corner_boss_members())
}

/// A row cell's label, `member:place`: a face by [`label`], an edge by
/// where its two ends stand, `x=1.40,y=1.00` for one parallel to `z`.
fn cell_label(
    ev: &Evaluation<f64>,
    doc: &ProfileDoc,
    labels: &BTreeMap<RecipeNodeId, &str>,
    cell: &NamedCell,
) -> String {
    let NamedCell::Entity { name, .. } = cell else {
        panic!("a boolean's row names entities: {cell:?}")
    };
    match name.kind {
        EntityKind::Face => label(ev, doc, labels, name),
        EntityKind::Edge => edge_label(ev, doc, labels, name),
        other => panic!("a corner's meeting names a {other:?}: {name:?}"),
    }
}

/// [`label`] for an edge name: its member, and where its ends stand.
fn edge_label(
    ev: &Evaluation<f64>,
    doc: &ProfileDoc,
    labels: &BTreeMap<RecipeNodeId, &str>,
    n: &StableName,
) -> String {
    let place = |node: RecipeNodeId, of: &StableName| {
        let Some(Entry::Unique(e)) = table(ev, node).lookup(of) else {
            panic!("{of:?} is not named once by node {node:?}")
        };
        let EntityKey::Edge(k) = e.key else {
            panic!("{of:?} is no edge")
        };
        let body = body_of(ev, node);
        let edge = body.get_edge(k).unwrap();
        let [p, q] = [edge.he_plus, edge.he_minus].map(|he| {
            let v = body.get_half_edge(he).unwrap().start;
            body.get_point(body.get_vertex(v).unwrap().point)
                .unwrap()
                .to_array()
        });
        if (p[0] - q[0]).abs() < 1e-9 && (p[1] - q[1]).abs() < 1e-9 {
            format!("x={:.2},y={:.2}", p[0], p[1])
        } else {
            "slant".to_owned()
        }
    };
    match n.path.as_slice() {
        [RoleSeg::From { read, of }] => match doc
            .defined_by(*read)
            .map(|(member, _)| member)
            .filter(|member| labels.contains_key(member))
        {
            Some(member) => format!("{}:{}", labels[&member], place(member, of)),
            None => edge_label(ev, doc, labels, of),
        },
        _ => {
            let at = holder(doc, labels, n);
            format!("{}:{}", labels[&at], place(at, n))
        }
    }
}

/// **The corner boss's corner is one point, logged alike in every
/// member order and in every pair boolean.** The boss's corner edge lies
/// in the slab's wall and crosses the plate's top, so a step folding the
/// plate onto the other two decides two pierces of the plate's top, the
/// edge's and the wall's, one point. Each order's union logs that
/// decision once, its one meeting of three cells: the boss's edge at
/// (1.4, 1.0), the slab's wall `x = 1.4` and the plate's top `z = 1`.
/// The plate's union and intersection with the slab-and-boss union, in
/// both orders, and the subtract of the plate from that union build and
/// log the same meeting.
///
/// *Breaks if* a step holds the decision out of its log, spells it in
/// other cells, or a pair boolean refuses the row's cells as no input's.
#[test]
fn the_corner_boss_logs_its_corner_alike_in_every_order_and_every_pair_boolean() {
    let members = corner_boss_members();
    let corner = ["boss:x=1.40,y=1.00", "plate:z=1.00", "slab:x=1.40"];
    let meeting = |what: &str, doc: &ProfileDoc, ids: &[RecipeNodeId], n: RecipeNodeId| {
        let ev = run(doc);
        assert!(failure(&ev, n).is_none(), "{what}: {:?}", failure(&ev, n));
        let labels: BTreeMap<RecipeNodeId, &str> = ids
            .iter()
            .zip(&members)
            .map(|(&id, (l, _))| (id, *l))
            .collect();
        let meetings: Vec<Vec<String>> = ev
            .value(n)
            .expect("the node evaluates")
            .coincidences
            .iter()
            .filter(|r| r.relation == topo::Relation::Meet && r.cells.len() == 3)
            .map(|r| {
                let mut cells: Vec<String> = r
                    .cells
                    .iter()
                    .map(|c| cell_label(&ev, doc, &labels, c))
                    .collect();
                cells.sort();
                cells
            })
            .collect();
        assert_eq!(meetings, [corner], "{what}: its one meeting of three cells");
    };
    let fresh = || corner_boss(ProfileDoc::empty_derived("corner_boss", Tol::witness()));
    for order in permutations(&[0, 1, 2]) {
        let (doc, ids) = fresh();
        let ordered: Vec<RecipeNodeId> = order.iter().map(|&i| ids[i]).collect();
        let (doc, n) = crate::fixture::union_over(doc, &ordered);
        meeting(&format!("the union in order {order:?}"), &doc, &ids, n);
    }
    type Pair = fn(RecipeNodeId, RecipeNodeId) -> editor_core::AuthoredNode;
    let pairs: [(&str, Pair); 5] = [
        ("union, the boss first", |u, p| PairOp::Union.node(u, p)),
        ("union, the plate first", |u, p| PairOp::Union.node(p, u)),
        ("intersect, the boss first", |u, p| Node::Intersect {
            members: editor_core::Bodies::Spelled(vec![u.into(), p.into()]),
        }),
        ("intersect, the plate first", |u, p| Node::Intersect {
            members: editor_core::Bodies::Spelled(vec![p.into(), u.into()]),
        }),
        ("the plate subtracted", |u, p| PairOp::Subtract.node(u, p)),
    ];
    for (what, node) in pairs {
        let (doc, ids) = fresh();
        let (doc, boss_on_slab) = crate::fixture::union_over(doc, &[ids[1], ids[2]]);
        let (doc, n) = insert(doc, node(boss_on_slab, ids[0]));
        meeting(what, &doc, &ids, n);
    }
}

/// **A member that engulfs the members before it** leaves the fold a
/// result in its own keys, holding none of their region: what they
/// discarded borders nothing after it. A bar across its top divides the
/// top alike in every order.
#[test]
fn a_member_engulfing_the_others_reads_only_what_follows_it() {
    check(&u(
        "engulfed",
        vec![
            ("plate", plate()),
            ("slab", slab()),
            ("big", b((-0.5, 3.5), (-1.5, 3.5), (-0.5, 3.0))),
            ("bar", b((1.0, 1.2), (-2.0, 4.0), (2.0, 1.0))),
        ],
        vec![
            ("big:z=2.50", vec![w(&["bar:x=1.00"]), w(&["bar:x=1.20"])]),
            ("bar:z=2.00", vec![w(&["big:y=-1.50"]), w(&["big:y=3.50"])]),
        ],
    ));
}

/// The T-junction's top: three pieces, told apart by the stem's walls.
fn t_top() -> (&'static str, Vec<Vec<String>>) {
    (
        "plate:z=1.00",
        vec![
            w(&["s1:x=1.40"]),
            w(&["s1:x=1.60", "s2:y=1.45"]),
            w(&["s1:x=1.60", "s2:y=1.55"]),
        ],
    )
}

/// The T-junction's members: a square plate, a slab across it, and a
/// stem from the slab to past the plate's far side.
fn t_members() -> Vec<(&'static str, Mem)> {
    vec![
        ("plate", b((0.0, 3.0), (0.0, 3.0), (0.0, 1.0))),
        ("s1", b((1.4, 1.6), (-1.0, 4.0), (0.5, 1.5))),
        ("s2", b((1.5, 3.5), (1.45, 1.55), (0.47, 1.23))),
    ]
}

/// **A T-junction divides the top in three**, one table in all six
/// member orders — the per-PR witness of the order-free claim.
#[test]
fn a_t_junction_divides_in_three() {
    let bottom = (
        "s1:z=0.50",
        vec![w(&["plate:y=0.00"]), w(&["plate:y=3.00"])],
    );
    check(&u("t_junction", t_members(), vec![t_top(), bottom]));
}

/// **A boss aligned with the T's stem on the other piece is not cited**
/// (the aligned-feature case that reframed N2): the boss borders one
/// piece.
#[test]
fn an_aligned_boss_on_the_other_piece_is_not_cited() {
    let bottom = (
        "s1:z=0.50",
        vec![w(&["plate:y=0.00"]), w(&["plate:y=3.00"])],
    );
    let mut members = t_members();
    members.push(("r", b((0.4, 0.8), (1.45, 1.9), (0.44, 0.86))));
    check(&u("aligned_boss", members, vec![t_top(), bottom]));
}

/// **A boss moved to abut the slab joins the divider, and the border
/// delta says so.** The boss stands on the left piece; slid right until
/// it overlaps the slab, it is part of the obstacle between the pieces,
/// so the left piece's name changes. Resolving the old name answers the
/// walls it gained — the boss's three free walls — and nothing lost,
/// and the right piece, untouched, is no counterpart.
#[test]
fn a_boss_moved_onto_the_divider_is_the_border_delta() {
    let doc = ProfileDoc::empty_derived("boss-abuts", Tol::witness());
    let (doc, plate) = add(doc, None, &plate());
    let (doc, slab_id) = add(doc, Some(plate), &slab());
    let (doc, tr) = add(doc, Some(plate), &b((0.4, 0.8), (0.8, 1.2), (0.47, 1.03)));
    let (doc, u) = union_of(doc, &[plate, slab_id, tr], &[0, 1, 2]);
    let doc2 = moved(doc.clone(), tr, TargetAxis::X, 0.65);
    let (plate_read, tr_read) = (out(&doc, plate), out(&doc, tr));
    let pieces = |ev: &Evaluation<f64>| -> BTreeSet<StableName> {
        table(ev, u)
            .iter()
            .filter(|(n, _)| {
                n.kind == EntityKind::Face
                    && matches!(n.path.first(), Some(RoleSeg::From { read, .. }) if *read == plate_read)
                    && matches!(n.path.last(), Some(RoleSeg::Fragment(Qualifier::Borders(_))))
            })
            .map(|(n, _)| n.clone())
            .collect()
    };
    let (ev1, ev2) = (run(&doc), run(&doc2));
    for ev in [&ev1, &ev2] {
        assert!(failure(ev, u).is_none(), "{:?}", failure(ev, u));
    }
    let (before, after) = (pieces(&ev1), pieces(&ev2));
    let gone: Vec<&StableName> = before.difference(&after).collect();
    let [left] = gone.as_slice() else {
        panic!("exactly the left piece's name changes: {before:?} -> {after:?}");
    };
    let res = crate::fixture::resolve_at_mint(
        editor_core::RunCtx {
            doc: &doc2,
            eval: &ev2,
        },
        left,
    );
    let editor_core::Resolution::Failed(f) = res else {
        panic!("the old name no longer resolves: {res:?}");
    };
    let editor_core::ResolveError::Vanished { diagnosis, .. } = f.error else {
        panic!("{:?}", f.error)
    };
    let editor_core::Diagnosis::BorderDelta { gone, new, .. } = diagnosis else {
        panic!("expected the border delta, got {diagnosis:?}")
    };
    assert!(
        gone.is_empty(),
        "the left piece still borders the slab: {gone:?}"
    );
    assert_eq!(new.len(), 3, "the boss's three free walls: {new:?}");
    assert!(
        new.iter().all(
            |n| matches!(n.path.first(), Some(RoleSeg::From { read, .. }) if *read == tr_read)
        ),
        "every new wall is the boss's: {new:?}"
    );
}

/// **A staircase divider, one member or three**: every wall along the
/// stair's edge is cited by the piece on its side.
#[test]
fn a_staircase_cites_every_wall_along_its_side() {
    check(&u(
        "staircase",
        vec![
            ("plate", plate()),
            (
                "stair",
                Mem::Prism(
                    vec![vec![
                        (1.0, -1.0),
                        (1.2, -1.0),
                        (1.2, 0.9),
                        (1.7, 0.9),
                        (1.7, 3.0),
                        (1.5, 3.0),
                        (1.5, 1.1),
                        (1.0, 1.1),
                    ]],
                    0.5,
                    1.5,
                ),
            ),
        ],
        vec![
            (
                "plate:z=1.00",
                vec![
                    w(&["stair:x=1.00", "stair:x=1.50", "stair:y=1.10"]),
                    w(&["stair:x=1.20", "stair:x=1.70", "stair:y=0.90"]),
                ],
            ),
            (
                "stair:z=0.50",
                vec![w(&["plate:y=0.00"]), w(&["plate:y=2.00"])],
            ),
        ],
    ));
    check(&u(
        "staircase3",
        vec![
            ("plate", plate()),
            ("s1", b((1.0, 1.2), (-1.0, 1.05), (0.5, 1.5))),
            ("s2", b((1.05, 1.65), (0.9, 1.1), (0.47, 1.33))),
            ("s3", b((1.5, 1.7), (0.95, 3.0), (0.44, 1.16))),
        ],
        vec![(
            "plate:z=1.00",
            vec![
                w(&[
                    "s1:x=1.00",
                    "s1:y=1.05",
                    "s2:x=1.05",
                    "s2:y=1.10",
                    "s3:x=1.50",
                ]),
                w(&[
                    "s1:x=1.20",
                    "s2:x=1.65",
                    "s2:y=0.90",
                    "s3:x=1.70",
                    "s3:y=0.95",
                ]),
            ],
        )],
    ));
}

/// **A tube cuts an island out of the top, and a slab divides tube and
/// top alike**: the island borders the bore, the rest the outer wall;
/// with a slab across, each quarter borders the slab and the stretch of
/// tube wall on its side, and the slab's top, inside and outside the
/// bore, borders the tube walls it meets.
#[test]
fn a_tube_islands_the_top_and_a_slab_divides_tube_and_top() {
    check(&u(
        "ptube_on_plate",
        vec![("plate", plate()), ("tube", ptube())],
        vec![(
            "plate:z=1.00",
            vec![ring("tube", false, &[]), ring("tube", true, &[])],
        )],
    ));
    let quarter = |x: &str, y: [&str; 2], x_slab: &str| {
        let mut v = ring_arc("tube", false, 6, &[x_slab]);
        v.extend([x, y[0], y[1]].map(|a| format!("tube:{a}")));
        v
    };
    check(&u(
        "ptube_slab",
        vec![
            ("plate", plate()),
            ("tube", ptube()),
            ("slab", b((1.45, 1.55), (-1.0, 3.0), (0.47, 1.03))),
        ],
        vec![
            (
                "plate:z=1.00",
                vec![
                    quarter("x=0.91", ["y=0.41", "y=1.59"], "slab:x=1.45"),
                    quarter("x=1.11", ["y=0.61", "y=1.39"], "slab:x=1.45"),
                    quarter("x=1.89", ["y=0.61", "y=1.39"], "slab:x=1.55"),
                    quarter("x=2.09", ["y=0.41", "y=1.59"], "slab:x=1.55"),
                ],
            ),
            (
                "slab:x=1.45",
                vec![
                    w(&["plate:y=0.00", "plate:z=1.00", "tube:y=0.41"]),
                    w(&["plate:y=2.00", "plate:z=1.00", "tube:y=1.59"]),
                    w(&["plate:z=1.00", "tube:y=0.61", "tube:y=1.39"]),
                ],
            ),
            (
                "slab:x=1.55",
                vec![
                    w(&["plate:y=0.00", "plate:z=1.00", "tube:y=0.41"]),
                    w(&["plate:y=2.00", "plate:z=1.00", "tube:y=1.59"]),
                    w(&["plate:z=1.00", "tube:y=0.61", "tube:y=1.39"]),
                ],
            ),
            (
                "slab:z=0.47",
                vec![w(&["plate:y=0.00"]), w(&["plate:y=2.00"])],
            ),
            (
                "slab:z=1.50",
                vec![
                    w(&["tube:y=0.41"]),
                    w(&["tube:y=0.61", "tube:y=1.39"]),
                    w(&["tube:y=1.59"]),
                ],
            ),
        ],
    ));
}

/// **A ring rib and a cup**: an island's piece borders the inner walls
/// and the rest the outer ones, on the plate's top and on the lid a
/// tube holds up alike.
#[test]
fn islands_under_a_ring_rib_and_a_cup_border_their_inner_walls() {
    check(&u(
        "ring_rib",
        vec![
            ("plate", plate()),
            (
                "rib",
                Mem::Prism(
                    vec![
                        vec![(1.0, 0.5), (2.0, 0.5), (2.0, 1.5), (1.0, 1.5)],
                        vec![(1.2, 0.7), (1.2, 1.3), (1.8, 1.3), (1.8, 0.7)],
                    ],
                    0.5,
                    1.5,
                ),
            ),
        ],
        vec![(
            "plate:z=1.00",
            vec![
                w(&["rib:x=1.00", "rib:x=2.00", "rib:y=0.50", "rib:y=1.50"]),
                w(&["rib:x=1.20", "rib:x=1.80", "rib:y=0.70", "rib:y=1.30"]),
            ],
        )],
    ));
    check(&u(
        "cup_on_plate",
        vec![
            ("plate", plate()),
            ("tube", ptube()),
            ("lid", b((0.8, 2.2), (0.3, 1.7), (1.9, 0.3))),
        ],
        vec![
            (
                "lid:z=1.90",
                vec![ring("tube", false, &[]), ring("tube", true, &[])],
            ),
            (
                "plate:z=1.00",
                vec![ring("tube", false, &[]), ring("tube", true, &[])],
            ),
        ],
    ));
}

/// **Overhangs and an arch reach over a piece without touching the
/// top**: only what stands in the top's plane is an obstacle, so the
/// pieces border the tall slab alone, whatever spans above them.
#[test]
fn what_spans_above_the_top_is_not_cited() {
    let tall = || b((1.4, 1.6), (-1.0, 3.0), (0.5, 2.5));
    let top = |m: &'static str| {
        (
            "plate:z=1.00",
            vec![w(&[&format!("{m}:x=1.40")]), w(&[&format!("{m}:x=1.60")])],
        )
    };
    let bottom = |m: &'static str| {
        (
            if m == "tall" {
                "tall:z=0.50"
            } else {
                "slab:z=0.50"
            },
            vec![w(&["plate:y=0.00"]), w(&["plate:y=2.00"])],
        )
    };
    check(&u(
        "overhang",
        vec![
            ("plate", plate()),
            ("tall", tall()),
            (
                "hook",
                Mem::Side(
                    vec![
                        (0.4, 0.47),
                        (0.8, 0.47),
                        (0.8, 2.2),
                        (1.5, 2.2),
                        (1.5, 2.45),
                        (0.4, 2.45),
                    ],
                    1.2,
                    0.4,
                ),
            ),
        ],
        vec![top("tall"), bottom("tall")],
    ));
    check(&u(
        "overhang4",
        vec![
            ("plate", plate()),
            ("tall", tall()),
            ("post", b((0.4, 0.8), (0.8, 1.2), (0.47, 2.03))),
            ("beam", b((0.5, 1.5), (0.85, 1.15), (2.2, 0.25))),
        ],
        vec![top("tall"), bottom("tall")],
    ));
    check(&u(
        "arch_over_slab",
        vec![
            ("plate", plate()),
            ("slab", slab()),
            (
                "arch",
                Mem::Side(
                    vec![
                        (0.8, 0.47),
                        (1.0, 0.47),
                        (1.0, 2.1),
                        (2.0, 2.1),
                        (2.0, 0.47),
                        (2.2, 0.47),
                        (2.2, 2.4),
                        (0.8, 2.4),
                    ],
                    1.2,
                    0.4,
                ),
            ),
        ],
        vec![top("slab"), bottom("slab")],
    ));
}

/// **The pair boolean reads the same rule**: a union and the subtracts
/// that slot or groove a plate, each piece bordering the cutter's walls
/// on its side, cited by the cutter's own names.
#[test]
fn the_pair_boolean_names_its_pieces_by_the_same_rule() {
    let groove = |z0: f64| {
        Mem::Prism(
            vec![
                ngon(1.5, 1.0, 0.6, 16, false),
                ngon(1.5, 1.0, 0.4, 16, true),
            ],
            z0,
            3.0,
        )
    };
    let halves = |m: &str| vec![w(&[&format!("{m}:x=1.40")]), w(&[&format!("{m}:x=1.60")])];
    let islanded = |m: &str| vec![ring(m, false, &[]), ring(m, true, &[])];
    for fx in [
        pair(
            "pair_union_slab",
            PairOp::Union,
            vec![("plate", plate()), ("slab", slab())],
            vec![("plate:z=1.00", halves("slab")), slab_bottom()],
        ),
        pair(
            "pair_union_ptube",
            PairOp::Union,
            vec![("plate", plate()), ("tube", ptube())],
            vec![("plate:z=1.00", islanded("tube"))],
        ),
        pair(
            "sub_through_slot",
            PairOp::Subtract,
            vec![
                ("plate", plate()),
                ("slot", b((1.4, 1.6), (-1.0, 3.0), (-1.0, 3.0))),
            ],
            vec![
                ("plate:y=0.00", halves("slot")),
                ("plate:y=2.00", halves("slot")),
                ("plate:z=0.00", halves("slot")),
                ("plate:z=1.00", halves("slot")),
            ],
        ),
        pair(
            "sub_blind_slot",
            PairOp::Subtract,
            vec![
                ("plate", plate()),
                ("slot", b((1.4, 1.6), (-1.0, 3.0), (0.5, 3.0))),
            ],
            vec![("plate:z=1.00", halves("slot"))],
        ),
        pair(
            "sub_through_groove",
            PairOp::Subtract,
            vec![("plate", plate()), ("groove", groove(-1.0))],
            vec![
                ("plate:z=0.00", islanded("groove")),
                ("plate:z=1.00", islanded("groove")),
            ],
        ),
        pair(
            "sub_blind_groove",
            PairOp::Subtract,
            vec![("plate", plate()), ("groove", groove(0.5))],
            vec![("plate:z=1.00", islanded("groove"))],
        ),
    ] {
        check(&fx);
    }
}

/// The tied-prongs block (`resolve_group_membership`'s fixture): a
/// 4×4×4 block with a U-shaped fork cut through it, whose two slot
/// ceilings are ONE face of the fork, so the block's table ties them.
/// Returns the document, the cut block and the block, the root a later
/// member is placed against.
fn tied_prongs() -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("union-dividing-tie", Tol::witness());
    let (doc, a) = block(doc, (0.0, 4.0), (0.0, 4.0), 4.0);
    let (doc, p) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
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
    let (doc, u) = insert(
        doc,
        Node::Extrude {
            profile: p.into(),
            distance: len(2.0),
        },
    );
    let (doc, u) = slid(doc, a, u, [0.0, 0.0, 1.0]);
    let (doc, sub) = insert(
        doc,
        Node::Subtract {
            from: a.into(),
            tool: u.into(),
        },
    );
    (doc, sub, a)
}

/// The faces of `union`'s table that descend from a face `member`'s own
/// table ties, as (name, whether the row is tied).
fn from_tied(
    ev: &Evaluation<f64>,
    doc: &ProfileDoc,
    union: RecipeNodeId,
    member: RecipeNodeId,
) -> Vec<(StableName, bool)> {
    let own = table(ev, member);
    let member_read = out(doc, member);
    table(ev, union)
        .iter()
        .filter(|(n, _)| n.kind == EntityKind::Face)
        .filter(|(n, _)| {
            matches!(parent_of(n).path.as_slice(), [RoleSeg::From { read, of }]
                if *read == member_read && matches!(own.lookup(of), Some(Entry::Tied(_))))
        })
        .map(|(n, e)| (n.clone(), matches!(e, Entry::Tied(_))))
        .collect()
}

/// **A bar that divides neither of two tied faces leaves them tied.**
/// A notching bar at x 3.9..4.1 closes the far end of one slot, notching
/// one of the two tied slot ceilings; a closing bar there spans both
/// slots and notches both. Neither divides a ceiling: tied member faces
/// are separate parents, each held whole, so both stay the tie's
/// candidates under one name, in both member orders. Moving the notching
/// bar to the other slot (a y edit) must not hand either ceiling a
/// unique name.
#[test]
fn a_bar_that_divides_no_tied_face_leaves_the_tie() {
    let (doc, sub, root) = tied_prongs();
    for (label, y, dy) in [("notching", (0.8, 1.7), 1.5), ("closing", (0.8, 3.2), 0.1)] {
        let (doc, tr) = block_on(doc.clone(), root, (3.9, 4.1), y, 2.5, 1.0);
        for order in permutations(&[0, 1]) {
            let (doc1, u) = union_of(doc.clone(), &[sub, tr], &order);
            let doc2 = moved(doc1.clone(), tr, TargetAxis::Y, dy);
            let mut seen = Vec::new();
            for (at, d) in [("before", &doc1), ("after", &doc2)] {
                let ev = run(d);
                assert!(
                    failure(&ev, u).is_none(),
                    "{label} {order:?} {at}: {:?}",
                    failure(&ev, u)
                );
                let rows = from_tied(&ev, d, u, sub);
                assert!(
                    !rows.is_empty(),
                    "{label} {order:?} {at}: no face descends from the tie"
                );
                for (n, tied) in &rows {
                    assert!(tied, "{label} {order:?} {at}: {n:?} is unique");
                }
                seen.push(rows.into_iter().map(|(n, _)| n).collect::<BTreeSet<_>>());
            }
            assert_eq!(
                seen[0], seen[1],
                "{label} {order:?}: the edit renamed the tie"
            );
        }
    }
}

/// **A split's names hold under an edit that moves its splitting
/// feature.** A block `a` = `[0,1]³` and two slabs through its top (x
/// 0.2..0.3 and 0.6..0.7), each a placed copy. Sliding the
/// first slab 0.05 along x keeps every piece: in every member order,
/// the same face names are published before and after, and each denotes
/// the same piece, moved by no more than the slide.
#[test]
fn a_split_keeps_its_names_when_the_splitting_feature_moves() {
    let doc = ProfileDoc::empty_derived("union-dividing-edit", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, tr) = block_on(doc, a, (0.2, 0.3), (-1.0, 2.0), 0.5, 2.5);
    let (doc, s2) = block_on(doc, a, (0.6, 0.7), (-1.0, 2.0), 0.5, 2.5);
    let mut checked = 0;
    for order in permutations(&[0, 1, 2]) {
        let (doc1, u) = union_of(doc.clone(), &[a, tr, s2], &order);
        let a_read = out(&doc1, a);
        let doc2 = moved(doc1.clone(), tr, TargetAxis::X, 0.05);
        let (ev1, ev2) = (run(&doc1), run(&doc2));
        for ev in [&ev1, &ev2] {
            assert!(failure(ev, u).is_none(), "{order:?}: {:?}", failure(ev, u));
        }
        let (c1, c2) = (face_centroids(&ev1, u), face_centroids(&ev2, u));
        assert_eq!(
            c1.keys().collect::<Vec<_>>(),
            c2.keys().collect::<Vec<_>>(),
            "{order:?}: the edit changed the published faces"
        );
        for (n, p) in &c1 {
            let q = c2[n];
            let d = ((p[0] - q[0]).powi(2) + (p[1] - q[1]).powi(2) + (p[2] - q[2]).powi(2)).sqrt();
            assert!(d <= 0.05 + 1e-9, "{order:?}: {n:?} moved {d}");
            checked += 1;
        }
        // The top is three pieces, told apart by the slab walls each
        // borders.
        let top_pieces = c1
            .keys()
            .filter(|n| {
                matches!(n.path.first(), Some(RoleSeg::From { read, .. }) if *read == a_read)
                    && n.path.len() == 2
            })
            .count();
        assert!(
            top_pieces >= 3,
            "{order:?}: {top_pieces} pieces of a's faces"
        );
    }
    assert!(checked > 0);
}

/// **A divider is a discriminator among a tie's candidates, so an edit
/// that moves it onto the other candidate moves each name with its
/// role** (N2's repair by a discriminator, N4's narrowing; ratified on
/// PR 3523). A bar at x 2.9..3.1, y 0.8..1.7 crosses the lower of the
/// tied-prongs block's two tied slot ceilings and splits it: its pieces
/// are named by the bar wall each borders, and the upper ceiling, now
/// the tie's only whole candidate, keeps the bare tied name narrowed to
/// `Unique`. Moving the bar 1.5 up in y makes it cross the upper ceiling
/// instead, and the names follow: the two `Borders` pieces now lie on
/// the upper ceiling and the bare name on the lower, in both member
/// orders. (The slot floors, which nothing divides, stay tied.) Changing this is changing that ruling, so it re-baselines on
/// purpose.
#[test]
fn a_divider_moved_onto_the_other_tied_face_moves_the_names_with_it() {
    const LOWER: f64 = 1.25;
    const UPPER: f64 = 2.75;
    let (doc, sub, root) = tied_prongs();
    let (doc, tr) = block_on(doc, root, (2.9, 3.1), (0.8, 1.7), 2.5, 1.0);
    let labels = BTreeMap::from([(sub, "sub"), (tr, "bar")]);
    for order in permutations(&[0, 1]) {
        let (doc1, u) = union_of(doc.clone(), &[sub, tr], &order);
        let doc2 = moved(doc1.clone(), tr, TargetAxis::Y, 1.5);
        let mut by_role = Vec::new();
        for (at, d) in [("before", &doc1), ("after", &doc2)] {
            let ev = run(d);
            assert!(
                failure(&ev, u).is_none(),
                "{order:?} {at}: {:?}",
                failure(&ev, u)
            );
            let c = face_centroids(&ev, u);
            let mut role: BTreeMap<String, (StableName, f64)> = BTreeMap::new();
            let ceiling = from_tied(&ev, d, u, sub)
                .into_iter()
                .filter(|(n, _)| label(&ev, d, &labels, &parent_of(n)) == "sub:z=3.00");
            for (n, tied) in ceiling {
                assert!(!tied, "{order:?} {at}: {n:?} is still tied");
                let key = match n.path.last() {
                    Some(RoleSeg::Fragment(Qualifier::Borders(walls))) => walls
                        .iter()
                        .map(|w| label(&ev, d, &labels, w))
                        .collect::<Vec<_>>()
                        .join("+"),
                    _ => "bare".to_owned(),
                };
                let y = c[&n][1];
                assert!(
                    role.insert(key.clone(), (n, y)).is_none(),
                    "{order:?} {at}: two faces under {key}"
                );
            }
            assert_eq!(
                role.keys().map(String::as_str).collect::<Vec<_>>(),
                ["bar:x=2.90", "bar:x=3.10", "bare"],
                "{order:?} {at}: the faces descending from the tie"
            );
            by_role.push(role);
        }
        for (key, want) in [
            ("bar:x=2.90", [LOWER, UPPER]),
            ("bar:x=3.10", [LOWER, UPPER]),
            ("bare", [UPPER, LOWER]),
        ] {
            let (n0, y0) = &by_role[0][key];
            let (n1, y1) = &by_role[1][key];
            assert_eq!(n0, n1, "{order:?}: the edit renamed {key}");
            assert!(
                (y0 - want[0]).abs() < 1e-9 && (y1 - want[1]).abs() < 1e-9,
                "{order:?}: {key} lies at y {y0} before and {y1} after, not {want:?}"
            );
        }
    }
}

/// A cylinder of radius 0.3 about its own `z` from its own origin, for
/// `length`: the profile's two semicircles its rims.
pub(crate) fn rod(doc: ProfileDoc, length: f64) -> (ProfileDoc, RecipeNodeId) {
    let (doc, disc) = insert(
        doc,
        Node::Profile(ProfileProgram {
            loops: vec![LoopProgram::circle_split(0.0, 0.0, 0.3, 2, 0.0).unwrap()],
            ids: Vec::new(),
            on: None,
        }),
    );
    insert(
        doc,
        Node::Extrude {
            profile: disc.into(),
            distance: len(length),
        },
    )
}

/// A [`rod`] placed against `target` about the axis through `c` along
/// `x × y`, from `c` for `length`: the placement.
pub(crate) fn cylinder(
    doc: ProfileDoc,
    target: RecipeNodeId,
    c: [f64; 3],
    x: [f64; 3],
    y: [f64; 3],
    length: f64,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, cyl) = rod(doc, length);
    crate::fixture::against(doc, target, cyl, &axes_at(c, x, y))
}

/// **A curved divider names in the union as it does in the pair
/// boolean.** A cylinder lying along y across the plate's top divides
/// it in two with curved walls. `Borders` reads no plane, so the face
/// pieces need none, and the seam between the plate's top and the
/// cylinder's wall is two separate lines (x = 1.2 and x = 1.8) that no
/// direction separates: each piece is named by its two ends (N2's
/// `Ends`), which differ in the lateral edge of the cylinder each line
/// runs into. So the pair boolean names, and so does the union in
/// either member order, with one table between the two orders. (The
/// top's two pieces border the one lower half-wall, so they are N2's
/// tie; no edge is.)
#[test]
fn a_curved_divider_names_as_the_pair_boolean_does() {
    let doc = ProfileDoc::empty_derived("union-dividing-across", Tol::witness());
    let (doc, plate) = block(doc, (0.0, 3.0), (0.0, 3.0), 1.0);
    let (doc, cyl) = cylinder(
        doc,
        plate,
        [1.5, 4.0, 1.0],
        [1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0],
        5.0,
    );
    let (pair_doc, pair) = insert(
        doc.clone(),
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![plate.into(), cyl.into()]),
        },
    );
    // The edge pieces each table names by their ends: (seam head, the
    // pieces' qualifiers), the head's sides read as a set.
    let ends_groups = |t: &NameTable| -> BTreeMap<String, usize> {
        let mut out = BTreeMap::new();
        for (name, entry) in t.iter() {
            assert!(
                name.kind != EntityKind::Edge || matches!(entry, Entry::Unique(_)),
                "a tied edge: {name:?} {entry:?}"
            );
            if name.kind == EntityKind::Edge
                && matches!(
                    name.path.last(),
                    Some(RoleSeg::Fragment(Qualifier::Ends(_)))
                )
            {
                *out.entry(format!("{:?}", name.path.first())).or_default() += 1;
            }
        }
        out
    };
    let pair_ev = run(&pair_doc);
    assert!(
        failure(&pair_ev, pair).is_none(),
        "the pair boolean names: {:?}",
        failure(&pair_ev, pair)
    );
    let alone = ends_groups(table(&pair_ev, pair));
    assert!(
        alone.values().any(|&n| n == 2),
        "the seam along the wall is two pieces told apart by their ends: {alone:?}"
    );
    let mut tables = Vec::new();
    for order in permutations(&[0, 1]) {
        let (docx, u) = union_of(doc.clone(), &[plate, cyl], &order);
        let ev = run(&docx);
        assert!(
            failure(&ev, u).is_none(),
            "{order:?}: the union names: {:?}",
            failure(&ev, u)
        );
        let t = table(&ev, u);
        assert_eq!(
            ends_groups(t).values().copied().collect::<Vec<_>>(),
            alone.values().copied().collect::<Vec<_>>(),
            "{order:?}: the union cuts its edges as the pair boolean does"
        );
        tables.push(t.iter().map(|(n, _)| n.clone()).collect::<BTreeSet<_>>());
    }
    assert_eq!(tables[0], tables[1], "the names depend on member order");
}

/// **A merged face cut in two is two pieces of one parent.** A cylinder
/// boss of radius 0.3 sunk into a plate keeps its wall as the profile's
/// two semicircular pieces; a slab subtracted across it along y cuts
/// each piece in two, and the subtract merges the two cosurface pieces
/// on each side of the slot. Both merges list the same two pieces, so
/// they are one parent, `Merged` of both, held as two faces; each is
/// qualified by the slab wall it borders (N2) rather than both
/// publishing the parent's bare name.
///
/// Each seam chord where a slab wall meets a merged face cites the
/// merged face's parent ([`assert_wall_chords_cite_the_merged_parent`]).
/// The boss is joined by a pair boolean and by a union in both member
/// orders.
#[test]
fn a_slot_across_a_sunk_boss_divides_its_merged_wall_by_the_slot_walls() {
    let doc = ProfileDoc::empty_derived("sunk-boss-slot", Tol::witness());
    let (doc, plate) = block(doc, (0.0, 3.0), (0.0, 3.0), 1.0);
    let (doc, boss) = cylinder(
        doc,
        plate,
        [1.5, 1.5, 0.5],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        1.0,
    );
    let (doc, slab) = block_on(doc, plate, (1.4, 1.6), (-1.0, 4.0), 0.8, 1.2);
    let (pair_doc, pair) = insert(
        doc.clone(),
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![plate.into(), boss.into()]),
        },
    );
    let mut joins = vec![("pair".to_owned(), pair_doc, pair)];
    for order in permutations(&[0, 1]) {
        let (d, u) = union_of(doc.clone(), &[plate, boss], &order);
        joins.push((format!("union {order:?}"), d, u));
    }
    for (label, doc, joined) in joins {
        let (doc, cut) = insert(
            doc,
            Node::Subtract {
                from: joined.into(),
                tool: slab.into(),
            },
        );
        let ev = run(&doc);
        assert!(
            failure(&ev, joined).is_none(),
            "{label}: the boss joins the plate: {:?}",
            failure(&ev, joined)
        );
        assert!(
            failure(&ev, cut).is_none(),
            "{label}: the slot names: {:?}",
            failure(&ev, cut)
        );
        assert_wall_chords_cite_the_merged_parent(&label, &ev, boss, cut, 4);
    }
}

/// **A seam chord on the boss wall cites the merged wall's parent.**
/// The boss of [`cylinder`] about (1.5, 1.5), radius 0.3, keeps its
/// wall as two pieces, read off the boss's own table. Every edge of
/// `out`'s table running up that wall (both ends on it, at one x and y)
/// is a seam chord against a wall of the other operand, and the merged
/// face beside it is cited by its parent, which lists both pieces,
/// never by the piece the chord lies on (N3). The chords on one slot
/// wall are two pieces of one seam, told apart by their `Ends` (N2);
/// `chords` of them stand.
fn assert_wall_chords_cite_the_merged_parent(
    label: &str,
    ev: &Evaluation<f64>,
    boss: RecipeNodeId,
    out: RecipeNodeId,
    chords: usize,
) {
    use crate::fixture::{ends, point};
    let on_wall = |p: geom_core::Point3<f64>| ((p.x - 1.5).hypot(p.y - 1.5) - 0.3).abs() < 1e-9;
    let boss_body = body_of(ev, boss);
    let pieces: Vec<String> = table(ev, boss)
        .iter()
        .filter_map(|(name, entry)| match entry {
            Entry::Unique(ent) => match ent.key {
                EntityKey::Face(f)
                    if topo::query::face_surface_kind(&boss_body, f)
                        == Some(geom::SurfaceKind::Cylinder) =>
                {
                    Some(name.to_string())
                }
                _ => None,
            },
            _ => None,
        })
        .collect();
    assert_eq!(
        pieces.len(),
        2,
        "{label}: the boss wall is two pieces: {pieces:?}"
    );
    let body = body_of(ev, out);
    let mut seams: BTreeMap<RoleSeg, usize> = BTreeMap::new();
    for (name, entry) in table(ev, out).iter() {
        let Entry::Unique(ent) = entry else { continue };
        let EntityKey::Edge(e) = ent.key else {
            continue;
        };
        let [p0, p1] = ends(&body, e).map(|v| point(&body, v));
        if !(on_wall(p0) && on_wall(p1) && (p0.x - p1.x).abs() < 1e-9 && (p0.y - p1.y).abs() < 1e-9)
        {
            continue;
        }
        let [
            head @ RoleSeg::Seam { a, b },
            RoleSeg::Fragment(Qualifier::Ends(_)),
        ] = name.path.as_slice()
        else {
            panic!("{label}: a chord up the boss wall is a piece of a seam, by its ends: {name}");
        };
        let cites_both = |side: &StableName| pieces.iter().all(|p| side.to_string().contains(p));
        assert!(
            cites_both(a) != cites_both(b),
            "{label}: the chord at ({:.3}, {:.3}) cites, on one side, the merged wall's \
             parent over both pieces: {name}",
            p0.x,
            p0.y
        );
        *seams.entry(head.clone()).or_default() += 1;
    }
    assert_eq!(
        seams.values().sum::<usize>(),
        chords,
        "{label}: the chords up the boss wall"
    );
    assert!(
        seams.values().all(|&n| n == 2),
        "{label}: each slot wall's seam with the merged wall is two pieces: {seams:?}"
    );
}

/// **A slot across a sunk boss, turned 90°, names.** The boss of
/// [`a_slot_across_a_sunk_boss_divides_its_merged_wall_by_the_slot_walls`],
/// with the slab running along x instead (x −1..4, y 1.4..1.6, from
/// z = 0.8 up): its walls cross the boss's curved wall, and the seam
/// chains along that wall are named by their ends, where they were once
/// ranked along a plane the wall does not have.
#[test]
fn a_slot_along_x_across_a_sunk_boss_names() {
    let doc = ProfileDoc::empty_derived("sunk-boss-slot-x", Tol::witness());
    let (doc, plate) = block(doc, (0.0, 3.0), (0.0, 3.0), 1.0);
    let (doc, boss) = cylinder(
        doc,
        plate,
        [1.5, 1.5, 0.5],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        1.0,
    );
    let (doc, slab) = block_on(doc, plate, (-1.0, 4.0), (1.4, 1.6), 0.8, 1.2);
    let (pair_doc, pair) = insert(
        doc.clone(),
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![plate.into(), boss.into()]),
        },
    );
    let mut joins = vec![("pair".to_owned(), pair_doc, pair)];
    for order in permutations(&[0, 1]) {
        let (d, u) = union_of(doc.clone(), &[plate, boss], &order);
        joins.push((format!("union {order:?}"), d, u));
    }
    for (label, doc, joined) in joins {
        let (doc, cut) = insert(
            doc,
            Node::Subtract {
                from: joined.into(),
                tool: slab.into(),
            },
        );
        let ev = run(&doc);
        assert!(
            failure(&ev, cut).is_none(),
            "{label}: the slot names: {:?}",
            failure(&ev, cut)
        );
        // Seam heads → the pieces named by their ends under each.
        let mut chains: BTreeMap<String, usize> = BTreeMap::new();
        for (name, entry) in table(&ev, cut).iter() {
            if name.kind != EntityKind::Edge {
                continue;
            }
            assert!(
                matches!(entry, Entry::Unique(_)),
                "{label}: a tied edge: {name:?}"
            );
            if let ([head, ..], Some(RoleSeg::Fragment(Qualifier::Ends(_)))) =
                (name.path.as_slice(), name.path.last())
            {
                *chains.entry(format!("{head:?}")).or_default() += 1;
            }
        }
        assert!(
            chains.values().any(|&n| n >= 2),
            "{label}: some chain is two or more pieces told apart by their ends: {chains:?}"
        );
    }
}

/// A plate `[0,3] × [0,2] × [0,1]` with a V-notch cut into its far
/// side, its apex at (1.5, 1.2).
fn notched_plate() -> Mem {
    Mem::Prism(
        vec![vec![
            (0.0, 0.0),
            (3.0, 0.0),
            (3.0, 2.0),
            (1.8, 2.0),
            (1.5, 1.2),
            (1.2, 2.0),
            (0.0, 2.0),
        ]],
        0.0,
        1.0,
    )
}

/// **Lost regions that meet only at a notch's apex** are read as one
/// obstacle, in every member order. `m` covers the notched plate's far
/// side up to the apex, so the top loses a region on each side of the
/// notch, the two touching at the apex alone, and the top's edge along
/// `m`'s wall runs through the apex, joined there. The kernel discards
/// the two regions as two faces; meeting at a point, they are one
/// obstacle through their closure (N2). It and `s`'s footprint divide
/// the top, so the right piece, which borders the right region only,
/// cites `m`'s end wall `x = 2` beside it.
#[test]
fn regions_meeting_only_at_a_notch_apex_are_read_as_one_obstacle() {
    check(&u(
        "notch_apex",
        vec![
            ("plate", notched_plate()),
            ("m", b((-1.0, 2.0), (1.2, 3.0), (0.5, 1.5))),
            ("s", b((0.6, 0.8), (-1.0, 1.3), (0.47, 1.53))),
        ],
        vec![
            (
                "plate:z=1.00",
                vec![
                    w(&["m:x=2.00", "m:y=1.20", "s:x=0.80"]),
                    w(&["m:y=1.20", "s:x=0.60"]),
                ],
            ),
            (
                "m:y=1.20",
                vec![
                    w(&["plate:x=0.00", "plate:z=1.00", "s:x=0.60"]),
                    w(&["plate:z=1.00", "s:x=0.80"]),
                ],
            ),
        ],
    ));
}

/// **Two members' discards that meet only at a notch's apex** are one
/// obstacle too, as [`regions_meeting_only_at_a_notch_apex_are_read_as_one_obstacle`]:
/// `m` is two members abutting at the apex's `x`, so each region is its
/// own member's discard, and what makes them one is the join at the
/// apex, which makes one edge of both members' seams, and `s`'s later
/// split of that edge, whose pieces lead back to it.
#[test]
fn two_members_regions_meeting_only_at_a_notch_apex_are_read_as_one_obstacle() {
    check(&u(
        "notch_apex_two",
        vec![
            ("plate", notched_plate()),
            ("m1", b((-1.0, 1.5), (1.2, 3.0), (0.5, 1.5))),
            ("m2", b((1.5, 2.0), (1.2, 3.0), (0.47, 1.53))),
            ("s", b((0.6, 0.8), (-1.0, 1.3), (0.44, 1.56))),
        ],
        vec![
            (
                "m1:y=1.20+m2:y=1.20",
                vec![
                    w(&["plate:x=0.00", "plate:z=1.00", "s:x=0.60"]),
                    w(&["plate:z=1.00", "s:x=0.80"]),
                ],
            ),
            (
                "plate:z=1.00",
                vec![
                    w(&["m1:y=1.20+m2:y=1.20", "m2:x=2.00", "s:x=0.80"]),
                    w(&["m1:y=1.20+m2:y=1.20", "s:x=0.60"]),
                ],
            ),
        ],
    ));
}

/// **A wall through a notch's apex** names alike in every member order.
/// `m` covers the notched plate's far side up to the apex, and `s2`'s
/// footprint ends at the apex's `x`, so the left region beyond `m`'s
/// wall, the right one and `s2`'s footprint all meet at the apex. That
/// point is one obstacle's whether the apex survives as a vertex (when
/// `s2` folds in first) or the output stage joins it away: the left
/// piece borders `m`'s wall, and the right piece `m`'s end wall too.
#[test]
fn a_wall_through_a_notch_apex_names_alike_in_every_order() {
    check(&u(
        "notch_apex_wall",
        vec![
            ("plate", notched_plate()),
            ("m", b((-1.0, 2.0), (1.2, 3.0), (0.5, 1.5))),
            ("s2", b((1.5, 1.7), (-1.0, 1.3), (0.47, 1.53))),
        ],
        vec![
            (
                "plate:z=1.00",
                vec![
                    w(&["m:y=1.20", "s2:x=1.50"]),
                    w(&["m:x=2.00", "m:y=1.20", "s2:x=1.70"]),
                ],
            ),
            (
                "m:y=1.20",
                vec![
                    w(&["plate:x=0.00", "plate:z=1.00", "s2:x=1.50"]),
                    w(&["plate:z=1.00", "s2:x=1.70"]),
                ],
            ),
            (
                "s2:x=1.50",
                vec![
                    w(&["m:y=1.20", "plate:y=0.00", "plate:z=1.00"]),
                    w(&["m:z=0.50", "plate:slant"]),
                ],
            ),
            ("s2:z=0.47", vec![w(&["plate:slant"]), w(&["plate:y=0.00"])]),
        ],
    ));
}

/// **An earlier seam split and joined away in one step** names alike in
/// every order. `m1`'s wall meets the top along a seam that `m2`, an
/// L whose leg crosses the plate and whose arm overlaps `m1` flush with
/// its wall, cuts at the leg; the arm covers `m1`'s end wall, so the
/// piece the cut minted is joined into the arm's seam in the same step.
/// The right piece's edge along the walls reaches `m1`'s discard
/// through that piece's lineage (`topo::BooleanNaming::killed_lineage`).
#[test]
fn an_earlier_seam_split_and_joined_away_in_one_step_names_alike() {
    check(&u(
        "split_join",
        vec![
            ("plate", plate()),
            ("m1", b((1.5, 2.5), (1.2, 3.0), (0.5, 1.5))),
            (
                "m2",
                Mem::Prism(
                    vec![vec![
                        (1.9, -1.0),
                        (2.0, -1.0),
                        (2.0, 3.0),
                        (0.5, 3.0),
                        (0.5, 1.2),
                        (1.9, 1.2),
                    ]],
                    0.5,
                    1.0,
                ),
            ),
        ],
        vec![
            (
                "m1:z=0.50+m2:z=0.50",
                vec![w(&["plate:y=0.00"]), w(&["plate:y=2.00"])],
            ),
            (
                "m2:z=1.50",
                vec![w(&["m1:x=1.50"]), w(&["m1:y=1.20+m2:y=1.20"])],
            ),
            (
                "plate:z=1.00",
                vec![
                    w(&["m1:x=2.50", "m1:y=1.20+m2:y=1.20", "m2:x=2.00"]),
                    w(&["m1:y=1.20+m2:y=1.20", "m2:x=0.50", "m2:x=1.90"]),
                ],
            ),
        ],
    ));
}

/// **A held edge wholly inside a dropped face**: the plate less a
/// pocket, its floor at z 0.5, unioned with a bar `b` whose top is
/// flush with the floor and wider than it, so the floor's four edges lie
/// inside `b`'s top with both ends at vertex-on-face contacts. The pair
/// union and both member orders publish one set of names: whichever
/// copy of the coincident tops the boolean drops is cut along the
/// floor's outline, so its lost part borders the floor's edges. Red
/// before the cut: the pair union and the union `[a, b]` refused that
/// a piece borders no recorded discard.
#[test]
fn a_pocket_floor_held_inside_a_dropped_face_names_alike() {
    let doc = ProfileDoc::empty_derived("held-pocket", Tol::witness());
    let (doc, plate) = block(doc, (0.0, 3.0), (0.0, 2.0), 1.0);
    let (doc, pocket) = block_on(doc, plate, (1.0, 2.0), (0.8, 1.2), 0.5, 1.5);
    let (doc, a) = insert(
        doc,
        Node::Subtract {
            from: plate.into(),
            tool: pocket.into(),
        },
    );
    let (doc, bar) = block_on(doc, plate, (0.5, 2.5), (-1.0, 3.0), 0.0, 0.5);
    let labels: BTreeMap<RecipeNodeId, &str> =
        [(plate, "plate"), (pocket, "pocket"), (a, "a"), (bar, "b")].into();
    let mut runs = vec![(
        "pair".to_owned(),
        insert(
            doc.clone(),
            Node::Union {
                members: editor_core::Bodies::Spelled(vec![a.into(), bar.into()]),
            },
        ),
    )];
    for order in permutations(&[0, 1]) {
        runs.push((
            format!("{order:?}"),
            union_of(doc.clone(), &[a, bar], &order),
        ));
    }
    let mut sets = Vec::new();
    for (at, (d, n)) in runs {
        let ev = run(&d);
        assert!(failure(&ev, n).is_none(), "{at}: {:?}", failure(&ev, n));
        sets.push((at, splits(&ev, &d, n, &labels)));
    }
    for (at, got) in &sets[1..] {
        assert_eq!(got, &sets[0].1, "{at} against {}: the wall sets", sets[0].0);
    }
}

/// `node`'s face pieces named by their walls, each as its own plane
/// with its walls' planes, sorted (a tied row counts once per
/// candidate): [`splits`] read by geometry, for members whose names
/// [`label`] does not read.
fn plane_splits(ev: &Evaluation<f64>, node: RecipeNodeId) -> Vec<(String, Vec<String>)> {
    let mut out = Vec::new();
    for (name, entry) in table(ev, node).iter() {
        let Some(RoleSeg::Fragment(Qualifier::Borders(walls))) = name.path.last() else {
            continue;
        };
        if name.kind != EntityKind::Face {
            continue;
        }
        let mut set: Vec<String> = walls.iter().map(|w| named_plane(ev, w)).collect();
        set.sort();
        let count = match entry {
            Entry::Unique(_) => 1,
            Entry::Tied(es) => es.len(),
        };
        for _ in 0..count {
            out.push((plane_label(ev, node, name), set.clone()));
        }
    }
    out.sort();
    out
}

/// The plane of the face `n` names, read in the table of the node that
/// minted it, or through the read it was taken from where that table
/// does not hold it.
fn named_plane(ev: &Evaluation<f64>, n: &StableName) -> String {
    match (table(ev, n.node).lookup(n), n.path.as_slice()) {
        (Some(_), _) => plane_label(ev, n.node, n),
        (None, [RoleSeg::From { of, .. }]) => named_plane(ev, of),
        (None, _) => panic!("{n:?} is named by no table it cites"),
    }
}

/// One letterform prism: `poly` on its own xy, extruded `depth` along
/// its own z; with `on`, its copy placed against that body on the frame
/// `(u, v)` through the origin.
fn letter(
    doc: ProfileDoc,
    on: Option<RecipeNodeId>,
    poly: &[(f64, f64)],
    (u, v): ([f64; 3], [f64; 3]),
    depth: f64,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, p) = insert(
        doc,
        Node::Profile(crate::fixture::desc(vec![poly.to_vec()])),
    );
    let (doc, prism) = insert(
        doc,
        Node::Extrude {
            profile: p.into(),
            distance: len(depth),
        },
    );
    match on {
        None => {
            crate::fixture::own_frame([0.0; 3], u, v);
            (doc, prism)
        }
        Some(root) => crate::fixture::against(doc, root, prism, &axes_at([0.0; 3], u, v)),
    }
}

/// **The tour's silhouette names through an intersect of an
/// intersect**: an H on xy extruded +z, a T on yz extruded +x and a C
/// on zx extruded +y; `two` = H ∩ T and `three` = C ∩ `two`. In
/// `three`, `two`'s T-shaped face at x = 2 coincides with the C's two
/// arm tips there: the bar and the stem's foot are held through them,
/// and the stem between is lost and borders both. Red when a sector
/// the C holds is read off a wide corner's other half: the lost stem
/// was recorded as held, and the arm tips' pieces bordered no discard
/// between them. Each member order builds and names its pieces by the
/// same walls, read by their planes.
#[test]
fn the_silhouette_letters_name_through_an_intersect_of_an_intersect() {
    let h = [
        (0.0, 0.0),
        (0.5, 0.0),
        (0.5, 1.25),
        (1.5, 1.25),
        (1.5, 0.0),
        (2.0, 0.0),
        (2.0, 3.0),
        (1.5, 3.0),
        (1.5, 1.75),
        (0.5, 1.75),
        (0.5, 3.0),
        (0.0, 3.0),
    ];
    let t = [
        (1.25, 0.0),
        (1.75, 0.0),
        (1.75, 2.5),
        (3.0, 2.5),
        (3.0, 3.0),
        (0.0, 3.0),
        (0.0, 2.5),
        (1.25, 2.5),
    ];
    let c = [
        (0.0, 0.0),
        (3.0, 0.0),
        (3.0, 2.0),
        (2.5, 2.0),
        (2.5, 0.5),
        (0.5, 0.5),
        (0.5, 2.0),
        (0.0, 2.0),
    ];
    let doc = ProfileDoc::empty_derived("silhouette", Tol::witness());
    let (doc, h) = letter(doc, None, &h, ([1.0, 0.0, 0.0], [0.0, 1.0, 0.0]), 3.0);
    let (doc, t) = letter(doc, Some(h), &t, ([0.0, 1.0, 0.0], [0.0, 0.0, 1.0]), 2.0);
    let (doc, c) = letter(doc, Some(h), &c, ([0.0, 0.0, 1.0], [1.0, 0.0, 0.0]), 3.0);
    // Each member order of the outer intersect, and of the inner one under
    // the first. Both swapped cite an extra z 2.5 wall on one y 3 piece
    // (`work/borders/a-direct-operands-own-split-lineage-climbs-to-a-live-sibling-seam.md`).
    let mut sets = Vec::new();
    for (hs, cs) in [(false, false), (true, false), (false, true)] {
        let (doc, two) = insert(
            doc.clone(),
            Node::Intersect {
                members: editor_core::Bodies::Spelled(if hs {
                    vec![t.into(), h.into()]
                } else {
                    vec![h.into(), t.into()]
                }),
            },
        );
        let (doc, three) = insert(
            doc,
            Node::Intersect {
                members: editor_core::Bodies::Spelled(if cs {
                    vec![two.into(), c.into()]
                } else {
                    vec![c.into(), two.into()]
                }),
            },
        );
        let ev = run(&doc);
        let at = format!(
            "{}, {}",
            if hs { "T ∩ H" } else { "H ∩ T" },
            if cs { "two ∩ C" } else { "C ∩ two" }
        );
        assert!(
            failure(&ev, two).is_none(),
            "{at}: two: {:?}",
            failure(&ev, two)
        );
        assert!(
            failure(&ev, three).is_none(),
            "{at}: three: {:?}",
            failure(&ev, three)
        );
        sets.push((at, plane_splits(&ev, three)));
    }
    for (at, got) in &sets[1..] {
        assert_eq!(got, &sets[0].1, "{at} against {}: the wall sets", sets[0].0);
    }
}

/// A stepped plate, `[0,3] × [0,2] × [0,2]` less an L pocket above
/// `z = 1`: its lower top `plate:z=1.00` is `[0,2] × [0,1] ∪
/// [0,1] × [1,1.5]`, and its riser `plate:y=1.00` stands at `y = 1` for
/// `x` in `[1,2]`. Returns the document, the plate, and the members
/// `more` after it, labelled.
fn stepped_plate_and(
    label: &'static str,
    more: Vec<(&'static str, Mem)>,
) -> (
    ProfileDoc,
    Vec<RecipeNodeId>,
    BTreeMap<RecipeNodeId, &'static str>,
) {
    let doc = ProfileDoc::empty_derived(label, Tol::witness());
    let (doc, blk) = add(doc, None, &b((0.0, 3.0), (0.0, 2.0), (0.0, 2.0)));
    let pocket = vec![vec![
        (-1.0, -1.0),
        (2.0, -1.0),
        (2.0, 1.0),
        (1.0, 1.0),
        (1.0, 1.5),
        (-1.0, 1.5),
    ]];
    let (doc, pk) = add(doc, Some(blk), &Mem::Prism(pocket, 1.0, 2.0));
    let (mut doc, plate) = insert(
        doc,
        Node::Subtract {
            from: blk.into(),
            tool: pk.into(),
        },
    );
    let mut ids = vec![plate];
    let mut labels = BTreeMap::from([(plate, "plate")]);
    for (l, m) in more {
        let (d, id) = add(doc, Some(blk), &m);
        doc = d;
        ids.push(id);
        labels.insert(id, l);
    }
    (doc, ids, labels)
}

/// Each split parent's wall sets, as [`splits`] reads them.
type Splits = BTreeMap<String, Vec<Vec<String>>>;

/// What the union of `ids` in each member order decides: its split
/// parents' wall sets, or the emission it refuses.
fn splits_by_order(
    doc: &ProfileDoc,
    ids: &[RecipeNodeId],
    labels: &BTreeMap<RecipeNodeId, &str>,
) -> Vec<(Vec<usize>, Result<Splits, &'static str>)> {
    permutations(&(0..ids.len()).collect::<Vec<_>>())
        .into_iter()
        .map(|order| {
            let ordered: Vec<RecipeNodeId> = order.iter().map(|&i| ids[i]).collect();
            let (d, n) = crate::fixture::union_over(doc.clone(), &ordered);
            let ev = run(&d);
            let got = match failure(&ev, n) {
                None => Ok(splits(&ev, &d, n, labels)),
                Some(NodeErrorKind::Naming(NamingError::Emission { what })) => Err(*what),
                Some(e) => panic!("{order:?}: refuses {e:?}"),
            };
            (order, got)
        })
        .collect()
}

/// The refusal of a union whose member orders decide differently
/// (`work/borders/a-union-over-a-stepped-plate-refuses-in-the-orders-that-fold-the-plate-early.md`).
const NO_DISCARD_BETWEEN: &str =
    "a piece of a face held as several borders no recorded discard between them";

/// **A corner touching the far half of a joined seam is not cited.**
/// `m` covers the stepped plate's pocket extension, so its wall at
/// `y = 1` merges with the riser, and the lower top's edge there is
/// joined at `x = 1`: `m`'s discard borders only the half at `x ≤ 1`.
/// `t` crosses it and the lower top, so the two are one divider. A
/// diamond's corner rests on the riser's half, at `(1.6, 1, 1)`, and
/// it meets no lost region of the top, so it is an obstacle on the
/// right piece alone and is never cited. Every order that publishes
/// gives one table; the rest refuse.
#[test]
fn a_corner_touching_the_far_half_of_a_joined_seam_is_not_cited() {
    let (doc, ids, labels) = stepped_plate_and(
        "joined_far_half",
        vec![
            ("m", b((-0.5, 1.3), (1.0, 1.6), (0.5, 2.2))),
            ("t", b((0.4, 0.6), (-1.0, 3.0), (0.5, 2.5))),
            (
                "dia",
                Mem::Prism(
                    vec![vec![(1.8, 0.8), (1.6, 1.0), (1.4, 0.8), (1.6, 0.6)]],
                    0.47,
                    0.56,
                ),
            ),
        ],
    );
    let top = vec![
        w(&["plate:y=1.00+m:y=1.00", "t:x=0.40"]),
        w(&["plate:y=1.00+m:y=1.00", "t:x=0.60"]),
    ];
    let mut published: Vec<(Vec<usize>, Splits)> = Vec::new();
    for (order, got) in splits_by_order(&doc, &ids, &labels) {
        // Orders that fold the plate before `m` or `t` refuse.
        let at = |i| order.iter().position(|&o| o == i);
        if at(0) < at(1).max(at(2)) {
            assert_eq!(got, Err(NO_DISCARD_BETWEEN), "{order:?}: refuses");
            continue;
        }
        let got = got.unwrap_or_else(|e| panic!("{order:?}: refuses {e}"));
        assert_eq!(
            got.get("plate:z=1.00"),
            Some(&top),
            "{order:?}: the lower top"
        );
        if let Some((first, table)) = published.first() {
            assert_eq!(&got, table, "{order:?} against {first:?}: the wall sets");
        }
        published.push((order, got));
    }
    assert_eq!(
        published.len(),
        8,
        "the orders that fold the plate after `m` and `t` publish"
    );
}

/// **A divider beside a joined seam does not reach across it.** The
/// stepped plate and `m` as in
/// [`a_corner_touching_the_far_half_of_a_joined_seam_is_not_cited`],
/// with `t` at `x` in `[1.5, 1.7]`: `t`'s discard lies on the riser's
/// side of the join and meets neither `m`'s discard on the lower top
/// nor the region `m`'s wall lost inside the plate, so only `t`
/// divides the lower top and the merged wall.
///
/// Orders (plate, m, t) and (m, plate, t) link `t`'s discard to `m`'s
/// through its chain along the joined edge and cite the merged wall
/// (`work/borders/borders-records-a-join-twice-and-reads-lineage-three-ways.md`);
/// they are held apart here, so a fix or a further order turns it red.
#[test]
fn a_divider_beside_a_joined_seam_does_not_reach_across_it() {
    let (doc, ids, labels) = stepped_plate_and(
        "joined_beside",
        vec![
            ("m", b((-0.5, 1.3), (1.0, 1.6), (0.5, 2.2))),
            ("t", b((1.5, 1.7), (-1.0, 3.0), (0.5, 2.5))),
        ],
    );
    let apart = || vec![w(&["t:x=1.50"]), w(&["t:x=1.70"])];
    let right: Splits = [
        ("plate:y=1.00+m:y=1.00", apart()),
        ("plate:z=1.00", apart()),
        ("plate:z=2.00", apart()),
        ("t:z=0.50", vec![w(&["plate:y=0.00"]), w(&["plate:y=2.00"])]),
    ]
    .into_iter()
    .map(|(p, sets)| (p.to_owned(), sets))
    .collect();
    let over_linked = [vec![0, 1, 2], vec![1, 0, 2]];
    for (order, got) in splits_by_order(&doc, &ids, &labels) {
        let got = got.unwrap_or_else(|e| panic!("{order:?}: refuses {e}"));
        if over_linked.contains(&order) {
            assert_eq!(
                got.get("plate:z=1.00"),
                Some(&vec![
                    w(&["plate:y=1.00+m:y=1.00", "t:x=1.50"]),
                    w(&["plate:y=1.00+m:y=1.00", "t:x=1.70"]),
                ]),
                "{order:?}: the lower top, over-linked"
            );
        } else {
            assert_eq!(got, right, "{order:?}: the wall sets");
        }
    }
}

/// **A corner touching the near half of a joined seam is part of the
/// divider** it touches. The stepped plate and `m` as in
/// [`a_corner_touching_the_far_half_of_a_joined_seam_is_not_cited`],
/// `t` at `x` in `[0.2, 0.4]`, and the diamond's corner at
/// `(0.75, 1, 1)`, on the half of the joined edge `m`'s discard borders:
/// the right piece cites the diamond's walls.
///
/// Where the plate folds after `m` and `t` and before the diamond, the
/// join is made before the diamond's step and nothing in the record
/// says which half of the joined edge its corner lies on, so the corner
/// meets no discard and the right piece does not cite it
/// (`work/borders/borders-records-a-join-twice-and-reads-lineage-three-ways.md`).
/// Those orders are held apart here; the orders that fold the plate
/// before `m` or `t` refuse.
#[test]
fn a_corner_touching_the_near_half_of_a_joined_seam_is_part_of_the_divider() {
    let (doc, ids, labels) = stepped_plate_and(
        "joined_near_half",
        vec![
            ("m", b((-0.5, 1.3), (1.0, 1.6), (0.5, 2.2))),
            ("t", b((0.2, 0.4), (-1.0, 3.0), (0.5, 2.5))),
            (
                "dia",
                Mem::Prism(
                    vec![vec![(0.9, 0.8), (0.75, 1.0), (0.6, 0.8), (0.75, 0.6)]],
                    0.47,
                    0.56,
                ),
            ),
        ],
    );
    let left = w(&["plate:y=1.00+m:y=1.00", "t:x=0.20"]);
    let right = w(&[
        "dia:slant",
        "dia:slant",
        "dia:slant",
        "dia:slant",
        "plate:y=1.00+m:y=1.00",
        "t:x=0.40",
    ]);
    let under_linked = [vec![1, 2, 0, 3], vec![2, 1, 0, 3]];
    let mut published = 0;
    for (order, got) in splits_by_order(&doc, &ids, &labels) {
        let at = |i| order.iter().position(|&o| o == i);
        if at(0) < at(1).max(at(2)) {
            assert_eq!(got, Err(NO_DISCARD_BETWEEN), "{order:?}: refuses");
            continue;
        }
        let got = got.unwrap_or_else(|e| panic!("{order:?}: refuses {e}"));
        let top = if under_linked.contains(&order) {
            vec![left.clone(), w(&["plate:y=1.00+m:y=1.00", "t:x=0.40"])]
        } else {
            vec![right.clone(), left.clone()]
        };
        assert_eq!(
            got.get("plate:z=1.00"),
            Some(&top),
            "{order:?}: the lower top"
        );
        published += 1;
    }
    assert_eq!(
        published, 8,
        "the orders that fold the plate after `m` and `t` publish"
    );
}
