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
//! as a tie, a split's names under an edit that moves its divider, and a
//! curved divider to the pair boolean's own answer.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::{BTreeMap, BTreeSet};

use crate::corpus::body_of;
use crate::docm7_union_declare::{block, failure, run};
use crate::emit_shared_rim_several::permutations;
use crate::emit_union_flush_names::parent_of;
use crate::fixture::{ang, face_vertices, frame, insert, len, on_frame, scl, step, table};

use editor_core::{
    Axis3, BooleanOp, DocEdit, EntityKey, EntityKind, Entry, Evaluation, LoopProgram, NamingError,
    Node, NodeErrorKind, ProfileDoc, ProfileProgram, Qualifier, RecipeNodeId, RoleSeg, SlotId,
    StableName,
};
use geom_core::Tol;

/// `input` behind a rigid `Transform` at the identity, so an edit can
/// move it.
fn movable(doc: ProfileDoc, input: RecipeNodeId) -> (ProfileDoc, RecipeNodeId) {
    insert(
        doc,
        Node::Transform {
            input,
            placement: editor_core::placement::Step::Rigid {
                translation: [len(0.0), len(0.0), len(0.0)],
                axis: [scl(0.0), scl(0.0), scl(1.0)],
                angle: ang(0.0),
            }
            .into(),
        },
    )
}

/// `doc` with the transform `tr` translated to `to` along `axis`.
fn moved(doc: ProfileDoc, tr: RecipeNodeId, axis: Axis3, to: f64) -> ProfileDoc {
    step(
        doc,
        DocEdit::SetParam {
            node: tr,
            slot: SlotId::Translation(axis),
            expr: len(to),
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
    insert(
        doc,
        Node::Union {
            members: order.iter().map(|&i| members[i]).collect(),
            declare: None,
        },
    )
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
        let vs = face_vertices(body, f);
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

fn add(doc: ProfileDoc, m: &Mem) -> (ProfileDoc, RecipeNodeId) {
    let (doc, p) = match m {
        Mem::Block(x, y, (z0, dz)) => return block(doc, *x, *y, *z0, *dz),
        Mem::Prism(loops, z0, _) => on_frame(
            doc,
            [0.0, 0.0, *z0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            loops.clone(),
        ),
        Mem::Side(pts, y1, _) => on_frame(
            doc,
            [0.0, *y1, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            vec![pts.clone()],
        ),
    };
    let distance = match m {
        Mem::Prism(_, _, d) | Mem::Side(_, _, d) => *d,
        Mem::Block(..) => unreachable!(),
    };
    insert(
        doc,
        Node::Extrude {
            profile: p,
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
    let (n, o) = (
        [normal.x, normal.y, normal.z],
        [origin.x, origin.y, origin.z],
    );
    for (i, axis) in ["x", "y", "z"].into_iter().enumerate() {
        if (n[i].abs() - 1.0).abs() < 1e-9 {
            return format!("{axis}={:.2}", o[i]);
        }
    }
    "slant".to_owned()
}

/// A face name's label, `member:plane`, read in the table of the node
/// that minted what it cites: a union's `FromMember` or `Merged` of
/// them, a pair boolean's `FromA`/`FromB`, or an operand's own name.
fn label(ev: &Evaluation<f64>, labels: &BTreeMap<RecipeNodeId, &str>, n: &StableName) -> String {
    match n.path.as_slice() {
        [RoleSeg::FromMember { member, of }] => {
            format!("{}:{}", labels[member], plane_label(ev, *member, of))
        }
        [RoleSeg::Merged(set)] => set
            .iter()
            .map(|c| label(ev, labels, c))
            .collect::<Vec<_>>()
            .join("+"),
        [RoleSeg::FromA(of) | RoleSeg::FromB(of)] => label(ev, labels, of),
        _ => format!("{}:{}", labels[&n.node], plane_label(ev, n.node, n)),
    }
}

/// `node`'s split parents, each as the sorted list of its pieces' wall
/// sets (a tied row counts once per candidate).
fn splits(
    ev: &Evaluation<f64>,
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
        let parent = label(ev, labels, &parent_of(name));
        let mut set: Vec<String> = walls.iter().map(|w| label(ev, labels, w)).collect();
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
    pair: Option<BooleanOp>,
    expect: Vec<(&'static str, Vec<Vec<String>>)>,
}

/// Runs `fx` in every member order: each publishes, all publish one
/// table (every name binding a face with the same centroid), and the
/// split parents' wall sets are `fx.expect`. Returns the orders run.
fn check(fx: &Fixture) -> usize {
    let mut doc = ProfileDoc::empty_derived(fx.label, Tol::witness());
    let mut ids = Vec::new();
    let mut labels = BTreeMap::new();
    for (l, m) in &fx.members {
        let (d, id) = add(doc, m);
        doc = d;
        ids.push(id);
        labels.insert(id, *l);
    }
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
        let node = match fx.pair {
            Some(op) => Node::Boolean {
                op,
                a: ids[0],
                b: ids[1],
                declare: None,
            },
            None => Node::Union {
                members: order.iter().map(|&i| ids[i]).collect(),
                declare: None,
            },
        };
        let (d, n) = insert(doc.clone(), node);
        let ev = run(&d);
        let at = format!("{} {order:?}", fx.label);
        assert!(failure(&ev, n).is_none(), "{at}: {:?}", failure(&ev, n));
        assert_eq!(splits(&ev, n, &labels), want, "{at}: the wall sets");
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

/// A pair-boolean fixture over the first two members.
fn pair(
    label: &'static str,
    op: BooleanOp,
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
    let (doc, plate) = add(doc, &plate());
    let (doc, slab_id) = add(doc, &slab());
    let (doc, boss) = add(doc, &b((0.4, 0.8), (0.8, 1.2), (0.47, 1.03)));
    let (doc, tr) = movable(doc, boss);
    let (doc, u) = union_of(doc, &[plate, slab_id, tr], &[0, 1, 2]);
    let doc2 = moved(doc.clone(), tr, Axis3::X, 0.65);
    let pieces = |ev: &Evaluation<f64>| -> BTreeSet<StableName> {
        table(ev, u)
            .iter()
            .filter(|(n, _)| {
                n.kind == EntityKind::Face
                    && matches!(n.path.first(), Some(RoleSeg::FromMember { member, .. }) if *member == plate)
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
    let res = editor_core::resolve(
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
            |n| matches!(n.path.first(), Some(RoleSeg::FromMember { member, .. }) if *member == tr)
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
            BooleanOp::Union,
            vec![("plate", plate()), ("slab", slab())],
            vec![("plate:z=1.00", halves("slab")), slab_bottom()],
        ),
        pair(
            "pair_union_ptube",
            BooleanOp::Union,
            vec![("plate", plate()), ("tube", ptube())],
            vec![("plate:z=1.00", islanded("tube"))],
        ),
        pair(
            "sub_through_slot",
            BooleanOp::Subtract,
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
            BooleanOp::Subtract,
            vec![
                ("plate", plate()),
                ("slot", b((1.4, 1.6), (-1.0, 3.0), (0.5, 3.0))),
            ],
            vec![("plate:z=1.00", halves("slot"))],
        ),
        pair(
            "sub_through_groove",
            BooleanOp::Subtract,
            vec![("plate", plate()), ("groove", groove(-1.0))],
            vec![
                ("plate:z=0.00", islanded("groove")),
                ("plate:z=1.00", islanded("groove")),
            ],
        ),
        pair(
            "sub_blind_groove",
            BooleanOp::Subtract,
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
fn tied_prongs() -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("union-dividing-tie", Tol::witness());
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
    let (doc, u) = insert(
        doc,
        Node::Extrude {
            profile: p,
            distance: len(2.0),
        },
    );
    insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Subtract,
            a,
            b: u,
            declare: None,
        },
    )
}

/// The faces of `union`'s table that descend from a face `member`'s own
/// table ties, as (name, whether the row is tied).
fn from_tied(
    ev: &Evaluation<f64>,
    union: RecipeNodeId,
    member: RecipeNodeId,
) -> Vec<(StableName, bool)> {
    let own = table(ev, member);
    table(ev, union)
        .iter()
        .filter(|(n, _)| n.kind == EntityKind::Face)
        .filter(|(n, _)| {
            matches!(parent_of(n).path.as_slice(), [RoleSeg::FromMember { member: m, of }]
                if *m == member && matches!(own.lookup(of), Some(Entry::Tied(_))))
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
    let (doc, sub) = tied_prongs();
    for (label, y, dy) in [("notching", (0.8, 1.7), 1.5), ("closing", (0.8, 3.2), 0.1)] {
        let (doc, bar) = block(doc.clone(), (3.9, 4.1), y, 2.5, 1.0);
        let (doc, tr) = movable(doc, bar);
        for order in permutations(&[0, 1]) {
            let (doc1, u) = union_of(doc.clone(), &[sub, tr], &order);
            let doc2 = moved(doc1.clone(), tr, Axis3::Y, dy);
            let mut seen = Vec::new();
            for (at, d) in [("before", &doc1), ("after", &doc2)] {
                let ev = run(d);
                assert!(
                    failure(&ev, u).is_none(),
                    "{label} {order:?} {at}: {:?}",
                    failure(&ev, u)
                );
                let rows = from_tied(&ev, u, sub);
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
/// 0.2..0.3 and 0.6..0.7), the first behind a transform. Sliding the
/// first slab 0.05 along x keeps every piece: in every member order,
/// the same face names are published before and after, and each denotes
/// the same piece, moved by no more than the slide.
#[test]
fn a_split_keeps_its_names_when_the_splitting_feature_moves() {
    let doc = ProfileDoc::empty_derived("union-dividing-edit", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, s1) = block(doc, (0.2, 0.3), (-1.0, 2.0), 0.5, 2.5);
    let (doc, s2) = block(doc, (0.6, 0.7), (-1.0, 2.0), 0.5, 2.5);
    let (doc, tr) = movable(doc, s1);
    let mut checked = 0;
    for order in permutations(&[0, 1, 2]) {
        let (doc1, u) = union_of(doc.clone(), &[a, tr, s2], &order);
        let doc2 = moved(doc1.clone(), tr, Axis3::X, 0.05);
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
                matches!(n.path.first(), Some(RoleSeg::FromMember { member, .. }) if *member == a)
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

/// A cylinder of radius 0.3 about the axis through `c` along `x × y`,
/// from `c` for `length`.
fn cylinder(
    doc: ProfileDoc,
    c: [f64; 3],
    x: [f64; 3],
    y: [f64; 3],
    length: f64,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, plane) = insert(doc, frame(c, x, y));
    let (doc, disc) = insert(
        doc,
        Node::Profile(ProfileProgram {
            plane,
            loops: vec![LoopProgram::circle_split(0.0, 0.0, 0.3, 2, 0.0).unwrap()],
            ids: Vec::new(),
        }),
    );
    insert(
        doc,
        Node::Extrude {
            profile: disc,
            distance: len(length),
        },
    )
}

/// **A curved divider answers in the union as it does in the pair
/// boolean.** A cylinder lying along y across the plate's top divides
/// it in two with curved walls. `Borders` reads no plane, so the pieces
/// need none; whatever the lone pair boolean does with this recipe,
/// each member order of the union does the same.
///
/// What both do is refuse as a missing rule: the seam chain along the
/// cylinder's wall is ranked along `n_a × n_b`, and the wall has no
/// plane (`NamingError::SplitReference`, curved, citing the wall). In
/// the union it is the fold's pair step that refuses, so which of the
/// seam's sides is `a` follows member order; every order cites the one
/// seam.
#[test]
fn a_curved_divider_answers_as_the_pair_boolean_does() {
    let doc = ProfileDoc::empty_derived("union-dividing-across", Tol::witness());
    let (doc, cyl) = cylinder(doc, [1.5, 4.0, 1.0], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0], 5.0);
    let (doc, plate) = block(doc, (0.0, 3.0), (0.0, 3.0), 0.0, 1.0);
    let (pair_doc, pair) = insert(
        doc.clone(),
        Node::Boolean {
            op: BooleanOp::Union,
            a: plate,
            b: cyl,
            declare: None,
        },
    );
    let member = |n: &StableName| match n.path.as_slice() {
        [RoleSeg::FromMember { of, .. }] => of.name().clone(),
        _ => n.clone(),
    };
    // A curved refusal as (the curved side, the seam's two sides), each
    // read through the union's `FromMember` to the member's own name so
    // the pair boolean's and the union's spellings compare. The sides
    // are a set: which is `a` follows member order.
    let curved = |e: Option<&NodeErrorKind>| match e {
        Some(NodeErrorKind::Naming(NamingError::SplitReference {
            group,
            reference,
            curved: true,
        })) => match group.path.as_slice() {
            [RoleSeg::Seam { a, b }] => Some((
                member(reference),
                BTreeSet::from([member(a.name()), member(b.name())]),
            )),
            _ => None,
        },
        _ => None,
    };
    let pair_ev = run(&pair_doc);
    let alone = curved(failure(&pair_ev, pair));
    assert!(
        alone
            .as_ref()
            .is_some_and(|(wall, sides)| wall.node == cyl && sides.contains(wall)),
        "the pair boolean refuses the seam chain along the cylinder's wall as a missing \
         rule: {:?}",
        failure(&pair_ev, pair)
    );
    for order in permutations(&[0, 1]) {
        let (docx, u) = union_of(doc.clone(), &[plate, cyl], &order);
        let ev = run(&docx);
        assert_eq!(
            curved(failure(&ev, u)),
            alone,
            "{order:?}: {:?}",
            failure(&ev, u)
        );
    }
}
