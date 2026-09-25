//! **A member edge's pieces are ranked once, over the finished union,
//! so one name lands on one piece in every member order.**
//!
//! The fold ranks a member edge's pieces at whichever step cuts it, and
//! which step that is — and which member keeps the pieces where two run
//! flush — depends on member order. The published table numbers each
//! member edge's pieces by the cells the finished body's vertices cut
//! that edge into (`emit_union::rank_member_edges`). These rows pin the
//! consequence over PR 3112's review corpus and the review fixtures of
//! #3168: a name two fused orders both publish denotes the same geometry
//! in both, and a vertex cites a member edge whole.
//!
//! Where members run flush, which member the fold kept a stretch for is
//! member order too; the published table names such a stretch, its
//! corners and its crossings for what the finished body holds
//! (`emit_union::Flush`), so every vertex and member-edge piece is
//! published in every fused order, and the flush documents below
//! publish one table in all of them.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeMap;

use crate::corpus::body_of;
use crate::docm7_union_declare::{declared_union, failure, flush_pairs, run};
use crate::emit_shared_rim_several::{Bx, document, permutations, probe_corpus, rim_piece};
use crate::fixture::{face_vertices, insert, table};

use editor_core::{CapEnd, EntityKey, EntityKind, Entry, Node, RecipeNodeId, RoleSeg, StableName};

/// A rounded point, comparable across two evaluations.
type P = (i64, i64, i64);

fn round(p: &geom_core::Point3<f64>) -> P {
    let r = |x: f64| (x * 1e6).round() as i64;
    (r(p.x), r(p.y), r(p.z))
}

/// A point to 1e-9, for [`signature`]: finer than any feature of the
/// cases, so two different points never share one.
fn fine(p: &geom_core::Point3<f64>) -> P {
    let r = |x: f64| (x * 1e9).round() as i64;
    (r(p.x), r(p.y), r(p.z))
}

/// Where each uniquely named entity of a published table is, independent
/// of arena keys: a vertex's point, an edge's two end points, a face's
/// vertex points.
fn geometry(
    ev: &editor_core::Evaluation<f64>,
    union: editor_core::RecipeNodeId,
) -> BTreeMap<StableName, Vec<P>> {
    let body = body_of(ev, union);
    let point = |v| round(body.get_point(body.get_vertex(v).unwrap().point).unwrap());
    let mut out = BTreeMap::new();
    for (name, entry) in table(ev, union).iter() {
        let Entry::Unique(e) = entry else { continue };
        let mut sig = match e.key {
            EntityKey::Vertex(v) => vec![point(v)],
            EntityKey::Edge(k) => {
                let edge = body.get_edge(k).unwrap();
                [edge.he_plus, edge.he_minus]
                    .iter()
                    .map(|&he| point(body.get_half_edge(he).unwrap().start))
                    .collect()
            }
            EntityKey::Face(f) => face_vertices(body, f).into_iter().map(point).collect(),
            _ => continue,
        };
        sig.sort_unstable();
        out.insert(name.clone(), sig);
    }
    out
}

/// **What each named entity of a published table IS, telling shells
/// apart** — every row, a tie included. A vertex is its point and the
/// sorted points of its edge neighbours, so two coincident vertices of
/// two touching shells differ; an edge is its two ends so described; a
/// face is its vertex points; a tie is its candidates', sorted and
/// marked.
pub(crate) fn signature(
    ev: &editor_core::Evaluation<f64>,
    union: editor_core::RecipeNodeId,
) -> BTreeMap<StableName, String> {
    let body = body_of(ev, union);
    let point = |v| fine(body.get_point(body.get_vertex(v).unwrap().point).unwrap());
    let mut neighbours: BTreeMap<_, Vec<P>> = BTreeMap::new();
    for (_, edge) in body.edges() {
        let s = body.get_half_edge(edge.he_plus).unwrap().start;
        let t = body.get_half_edge(edge.he_minus).unwrap().start;
        neighbours.entry(s).or_default().push(point(t));
        neighbours.entry(t).or_default().push(point(s));
    }
    let vertex = |v| {
        let mut n = neighbours.get(&v).cloned().unwrap_or_default();
        n.sort_unstable();
        format!("{:?}<{n:?}>", point(v))
    };
    let one = |k: EntityKey| -> String {
        let mut parts: Vec<String> = match k {
            EntityKey::Vertex(v) => vec![vertex(v)],
            EntityKey::Edge(e) => {
                let edge = body.get_edge(e).unwrap();
                [edge.he_plus, edge.he_minus]
                    .iter()
                    .map(|&he| vertex(body.get_half_edge(he).unwrap().start))
                    .collect()
            }
            EntityKey::Face(f) => face_vertices(body, f)
                .into_iter()
                .map(|v| format!("{:?}", point(v)))
                .collect(),
            other => vec![format!("{other:?}")],
        };
        parts.sort();
        parts.concat()
    };
    table(ev, union)
        .iter()
        .map(|(name, entry)| {
            let sig = match entry {
                Entry::Unique(e) => one(e.key),
                Entry::Tied(es) => {
                    let mut all: Vec<String> = es.iter().map(|e| one(e.key)).collect();
                    all.sort();
                    format!("TIED[{}]", all.join("|"))
                }
            };
            (name.clone(), sig)
        })
        .collect()
}

/// A document and how each of its member orders is built into a union.
pub(crate) struct Case {
    pub(crate) label: String,
    blocks: Vec<Bx>,
    creation: Vec<usize>,
    /// Pairs of blocks declared flush on all four families; none makes
    /// an undeclared union.
    flush: Vec<(usize, usize)>,
    /// Blocks 0 and 1 in an inner union declared flush (both orders), and
    /// that union with the rest in an outer undeclared one (every order).
    nested: bool,
}

impl Case {
    pub(crate) fn flat(
        label: &str,
        blocks: Vec<Bx>,
        creation: Vec<usize>,
        flush: Vec<(usize, usize)>,
    ) -> Self {
        Case {
            label: label.into(),
            blocks,
            creation,
            flush,
            nested: false,
        }
    }

    fn nested(label: &str, blocks: Vec<Bx>) -> Self {
        let creation = (0..blocks.len()).collect();
        Case {
            label: label.into(),
            blocks,
            creation,
            flush: vec![(0, 1)],
            nested: true,
        }
    }
}

const A: Bx = ((0.0, 1.0), (0.0, 1.0), (0.0, 1.0));
const B: Bx = ((0.5, 1.5), (0.0, 1.0), (0.0, 1.0));
/// Two slabs across `a`'s rims, and a third overlapping both cuts.
const S1: Bx = ((0.2, 0.3), (-1.0, 2.0), (0.5, 3.0));
const S2: Bx = ((0.6, 0.7), (-1.0, 2.0), (0.5, 3.0));
const S3: Bx = ((0.25, 0.65), (-1.0, 0.5), (0.8, 3.0));
/// A third block flush with both `a` and `b` (`r4tri`), and one touching
/// `a`'s top along a line (`r4touch`).
const C8: Bx = ((0.8, 2.0), (0.0, 1.0), (0.0, 1.0));
const TOUCH: Bx = ((0.3, 0.4), (1.0, 2.0), (1.0, 1.0));
/// A long block, flush with `bend` at one end and `cend` at the other.
const ALONG: Bx = ((0.0, 3.0), (0.0, 1.0), (0.0, 1.0));
const BEND: Bx = ((2.0, 4.0), (0.0, 1.0), (0.0, 1.0));
const CEND: Bx = ((-1.0, 1.0), (0.0, 1.0), (0.0, 1.0));

/// PR 3112's review corpus (`a`, `b` flush), and the review's own
/// fixtures of #3168: one member edge cut in different steps (`r1two`,
/// `r1three`), the same beside a flush partner (`r1flush`), a member
/// flush with TWO partners (`r2ends`, `r2endsg`), three members each
/// flush with the other two (`r4tri`, `r4trig`), a member touching
/// another along a line (`r4touch`), and a declared union nested in an
/// undeclared one (`r3nest`, `r3nest2`).
pub(crate) fn cases() -> Vec<Case> {
    let g = ((0.3, 0.4), (-1.0, 2.0), (0.5, 3.0));
    let smid = ((1.4, 1.6), (-1.0, 2.0), (0.5, 3.0));
    let mut out: Vec<Case> = probe_corpus()
        .into_iter()
        .map(|(label, blocks, creation)| Case::flat(&label, blocks, creation, vec![(0, 1)]))
        .collect();
    out.extend([
        Case::flat("r1two", vec![A, S1, S2], vec![0, 1, 2], vec![]),
        Case::flat("r1three", vec![A, S1, S2, S3], vec![0, 1, 2, 3], vec![]),
        Case::flat(
            "r1flush",
            vec![A, B, S1, S2],
            vec![0, 1, 2, 3],
            vec![(0, 1)],
        ),
        r2ends(),
        Case::flat(
            "r2endsg",
            vec![ALONG, BEND, CEND, smid],
            vec![0, 1, 2, 3],
            vec![(0, 1), (0, 2)],
        ),
        Case::flat("r4tri", vec![A, B, C8], vec![0, 1, 2], TRI.to_vec()),
        Case::flat("r4trig", vec![A, B, C8, g], vec![0, 1, 2, 3], TRI.to_vec()),
        Case::flat("r4touch", vec![A, B, TOUCH], vec![0, 1, 2], vec![(0, 1)]),
        Case::nested("r3nest", vec![A, B, g]),
        Case::nested("r3nest2", vec![A, B, S1, S2]),
    ]);
    out
}

/// Every pair of three blocks declared flush.
const TRI: [(usize, usize); 3] = [(0, 1), (1, 2), (0, 2)];

fn r2ends() -> Case {
    Case::flat(
        "r2ends",
        vec![ALONG, BEND, CEND],
        vec![0, 1, 2],
        vec![(0, 1), (0, 2)],
    )
}

/// Every run of `case`: `each` is handed the orders that built it, the
/// evaluation, the member ids by block, and the unions to read, tagged.
pub(crate) fn runs(
    case: &Case,
    mut each: impl FnMut(&str, &editor_core::Evaluation<f64>, &[RecipeNodeId], &[(&str, RecipeNodeId)]),
) {
    let (doc, ids) = document(&case.blocks, &case.creation);
    let flush = |ids: &[RecipeNodeId]| {
        case.flush
            .iter()
            .flat_map(|&(p, q)| flush_pairs(&doc, (ids[p], ids[p]), (ids[q], ids[q])))
            .collect::<Vec<_>>()
    };
    if case.nested {
        for inner in permutations(&[0, 1]) {
            let io: Vec<_> = inner.iter().map(|&i| ids[i]).collect();
            let (d1, u1, _) = declared_union(doc.clone(), &io, flush(&ids));
            let outer: Vec<_> = std::iter::once(u1)
                .chain(ids[2..].iter().copied())
                .collect();
            for olab in permutations(&(0..outer.len()).collect::<Vec<_>>()) {
                let members = olab.iter().map(|&i| outer[i]).collect();
                let (docx, top) = insert(
                    d1.clone(),
                    Node::Union {
                        members,
                        declare: None,
                    },
                );
                let ev = run(&docx);
                each(
                    &format!("{inner:?}{olab:?}"),
                    &ev,
                    &ids,
                    &[("U1", u1), ("U", top)],
                );
            }
        }
        return;
    }
    for order in permutations(&(0..case.blocks.len()).collect::<Vec<_>>()) {
        let members: Vec<_> = order.iter().map(|&i| ids[i]).collect();
        let (docx, union) = if case.flush.is_empty() {
            insert(
                doc.clone(),
                Node::Union {
                    members,
                    declare: None,
                },
            )
        } else {
            let (d, u, _) = declared_union(doc.clone(), &members, flush(&ids));
            (d, u)
        };
        let ev = run(&docx);
        each(&format!("{order:?}"), &ev, &ids, &[("U", union)]);
    }
}

/// **The cases that refuse in some member orders and publish in others**,
/// pinned: `(label, union, orders refusing, refusal variants)`. Each is a
/// real order dependence, owned by
/// `work/emit/union-refuses-in-some-member-orders-and-publishes-in-others.md`
/// (and, for `DeclareResolve`, by the gather row it cites); a change here
/// is a change to that row, measured.
const KNOWN_MIXED: &[(&str, &str, usize, &str)] = &[
    ("row", "U", 18, "DeclareResolve/UndeclaredContact"),
    ("rowids", "U", 18, "DeclareResolve/UndeclaredContact"),
    ("abg", "U", 2, "DeclareResolve"),
    ("abgids", "U", 2, "DeclareResolve"),
    ("abglow", "U", 2, "DeclareResolve"),
    ("abgg2", "U", 16, "DeclareResolve"),
    ("fam000", "U", 2, "DeclareResolve"),
    ("fam001", "U", 2, "DeclareResolve"),
    ("fam002", "U", 2, "DeclareResolve"),
    ("fam012", "U", 2, "DeclareResolve"),
    ("fam022", "U", 2, "DeclareResolve"),
    ("fam100", "U", 4, "DeclareResolve"),
    ("fam101", "U", 4, "DeclareResolve"),
    ("fam102", "U", 4, "DeclareResolve"),
    ("fam112", "U", 4, "DeclareResolve"),
    ("fam122", "U", 4, "DeclareResolve"),
    ("fam200", "U", 2, "DeclareResolve"),
    ("fam201", "U", 2, "DeclareResolve"),
    ("fam202", "U", 2, "DeclareResolve"),
    ("fam212", "U", 2, "DeclareResolve"),
    ("fam222", "U", 2, "DeclareResolve"),
    ("r1flush", "U", 18, "DeclareResolve"),
    ("r2endsg", "U", 8, "DeclareResolve"),
    ("r4tri", "U", 2, "Boolean"),
    ("r4trig", "U", 12, "Boolean/DeclareResolve"),
];

/// **No name rebinds across member orders, and no order refuses what
/// another publishes.** Over every case and every pair of fused orders, a
/// name both tables publish denotes the same thing ([`signature`]: points
/// to 1e-9, shells told apart, ties compared) — for a nested case, in the
/// inner union and in the outer one. A case whose orders do not all
/// publish reds, unless it is one of [`KNOWN_MIXED`] exactly as pinned
/// there (`work/emit/union-refuses-in-some-member-orders-and-publishes-in-others.md`).
///
/// A name one fused order publishes and another does not fails the row
/// when it is a vertex or a piece of a member edge ([`order_free_class`]):
/// those are named for what the finished body holds, so they are the
/// same in every order. Faces, and the seam edges that cite them, are
/// reported, not failed: how a face both merged and cut, or cut in two
/// steps, is named still follows the fold
/// (`declared-flush-union-edge-and-vertex-names-follow-member-order`, P1).
///
/// On main (the #3168 review's probe of the corpus plus the r1–r3
/// fixtures), 25 of 305 pairs of fused orders rebound 20 distinct names;
/// ranking over the pieces a member KEEPS left
/// `r2ends`, `r2endsg` and `r4tri` rebinding, because which member keeps
/// a flush stretch depends on order.
#[test]
fn a_name_two_member_orders_both_publish_denotes_the_same_geometry() {
    let mut compared = 0;
    let mut mixed = Vec::new();
    let mut absent = 0;
    let mut order_bound = Vec::new();
    for case in cases() {
        let mut seen: BTreeMap<(String, StableName), (String, String)> = BTreeMap::new();
        // tag → (order, refusal or None, published names)
        type Outcome = (String, Option<String>, Vec<StableName>);
        let mut outcomes: BTreeMap<String, Vec<Outcome>> = BTreeMap::new();
        runs(&case, |at, ev, _, unions| {
            for &(tag, union) in unions {
                if let Some(refused) = failure(ev, union) {
                    // Arena keys and node ids differ by order; the
                    // refusal's variant is what must agree.
                    let shown = format!("{refused:?}");
                    let kind = shown
                        .split([' ', '(', '{'])
                        .next()
                        .unwrap_or("")
                        .to_string();
                    outcomes.entry(tag.to_string()).or_default().push((
                        at.to_string(),
                        Some(kind),
                        Vec::new(),
                    ));
                    continue;
                }
                let sigs = signature(ev, union);
                outcomes.entry(tag.to_string()).or_default().push((
                    at.to_string(),
                    None,
                    sigs.keys().cloned().collect(),
                ));
                for (name, sig) in sigs {
                    match seen.get(&(tag.to_string(), name.clone())) {
                        Some((first, then)) => {
                            compared += 1;
                            assert_eq!(
                                first, &sig,
                                "{} {tag}: {name:?} is {first:?} in {then} and {sig:?} in {at}",
                                case.label
                            );
                        }
                        None => {
                            seen.insert((tag.to_string(), name), (sig, at.to_string()));
                        }
                    }
                }
            }
        });
        for (tag, runs) in outcomes {
            let refusals: Vec<_> = runs
                .iter()
                .filter_map(|(at, r, _)| Some((at, r.as_ref()?)))
                .collect();
            if !refusals.is_empty() && refusals.len() < runs.len() {
                let kinds: std::collections::BTreeSet<&str> =
                    refusals.iter().map(|(_, k)| k.as_str()).collect();
                eprintln!(
                    "{} {tag}: {} of {} orders refuse {refusals:?}",
                    case.label,
                    refusals.len(),
                    runs.len()
                );
                mixed.push(format!(
                    "{} {tag}: {} {}",
                    case.label,
                    refusals.len(),
                    kinds.into_iter().collect::<Vec<_>>().join("/")
                ));
            }
            let published: Vec<_> = runs.iter().filter(|(_, r, _)| r.is_none()).collect();
            let all: std::collections::BTreeSet<&StableName> =
                published.iter().flat_map(|(_, _, ns)| ns.iter()).collect();
            for (at, _, ns) in &published {
                for &name in &all {
                    if ns.contains(name) {
                        continue;
                    }
                    absent += 1;
                    if order_free_class(name) {
                        order_bound.push(format!("{} {tag} {at}: {name:?}", case.label));
                    }
                }
            }
        }
    }
    eprintln!(
        "names published in one fused order and absent in another: {absent} (order, name) pairs"
    );
    assert!(
        order_bound.is_empty(),
        "{} vertex or member-edge names are published in one fused order and absent in \
         another; the first: {:?}",
        order_bound.len(),
        order_bound.first()
    );
    let known: Vec<String> = KNOWN_MIXED
        .iter()
        .map(|(label, tag, n, kinds)| format!("{label} {tag}: {n} {kinds}"))
        .collect();
    assert_eq!(
        mixed, known,
        "the cases that refuse in some orders and publish in others changed"
    );
    assert!(
        compared > 1000,
        "only {compared} cross-order names compared"
    );
}

/// A published table as [`signature`] reads it.
type Signatures = BTreeMap<StableName, String>;

/// The names [`a_name_two_member_orders_both_publish_denotes_the_same_geometry`]
/// requires every fused order to publish: every vertex, and every piece
/// of a member edge.
fn order_free_class(name: &StableName) -> bool {
    match (name.kind, name.path.first()) {
        (EntityKind::Vertex, _) => true,
        (EntityKind::Edge, Some(RoleSeg::FromMember { of, .. })) => of.kind == EntityKind::Edge,
        _ => false,
    }
}

/// The cases whose every fused order publishes one table: a flush pair
/// alone or with a slab through, beside or across its flush stretch; a
/// member flush with two others; three members each flush with the
/// other two; a member touching another along a line; a declared union
/// nested in an undeclared one.
const ORDER_FREE: &[&str] = &[
    "abc", "fam010", "fam011", "fam020", "fam021", "fam100", "fam101", "fam102", "fam110",
    "fam111", "fam112", "fam120", "fam121", "fam122", "fam210", "fam211", "fam220", "fam221",
    "r2ends", "r4tri", "r4touch", "r3nest",
];

/// **A declared flush union publishes one table in every member order
/// that fuses.** For each of [`ORDER_FREE`], every fused order's table —
/// every name, and what each denotes ([`signature`]) — is the same.
///
/// On main these differed in which member a flush stretch and its
/// corners were named for, in whether a seam cited a member's face or
/// the merge it retired into, and in whether a slab's crossing of a
/// merged rim was a junction of three seams or a vertex on the rim.
#[test]
fn a_flush_union_publishes_one_table_in_every_member_order() {
    let mut checked = 0;
    for case in cases()
        .into_iter()
        .filter(|c| ORDER_FREE.contains(&c.label.as_str()))
    {
        // tag → (order, its table)
        let mut tables: BTreeMap<String, Vec<(String, Signatures)>> = BTreeMap::new();
        runs(&case, |at, ev, _, unions| {
            for &(tag, union) in unions {
                if failure(ev, union).is_none() {
                    tables
                        .entry(tag.to_string())
                        .or_default()
                        .push((at.to_string(), signature(ev, union)));
                }
            }
        });
        for (tag, published) in tables {
            assert!(
                published.len() >= 2,
                "{} {tag}: {} fused orders",
                case.label,
                published.len()
            );
            let (first_at, first) = &published[0];
            for (at, table) in &published[1..] {
                let only_first: Vec<_> = first.keys().filter(|n| !table.contains_key(*n)).collect();
                let only_this: Vec<_> = table.keys().filter(|n| !first.contains_key(*n)).collect();
                assert!(
                    only_first.is_empty() && only_this.is_empty(),
                    "{} {tag}: {first_at} alone publishes {only_first:?}; {at} alone publishes {only_this:?}",
                    case.label
                );
                assert_eq!(
                    first, table,
                    "{} {tag}: {first_at} against {at}",
                    case.label
                );
            }
            checked += 1;
        }
    }
    assert_eq!(
        checked,
        ORDER_FREE.len() + 1,
        "cases checked (r3nest has two unions)"
    );
}

/// **A member flush with two others numbers its rim by the body, whoever
/// holds each stretch.** `r2ends`: `a` = x 0..3 is flush with `b` over
/// x 2..3 and with `c` over x 0..1. Its bottom start rim (x 0 → 3 at
/// y = z = 0) is cut at x = 1 and 2 into three cells, so its pieces are
/// `#k of 3` spanning x = k..k + 1 in every order: `a` always holds the
/// middle one, and each end one when it was folded before that end's
/// partner. Ranked over the pieces `a` keeps, `#0 of 2` was x = 0..1 in
/// `[b, a, c]` and x = 1..2 in `[c, a, b]`.
#[test]
fn a_member_flush_with_two_others_numbers_its_rim_by_the_body() {
    let case = r2ends();
    let (doc, _) = document(&case.blocks, &case.creation);
    let mut fused = 0;
    runs(&r2ends(), |at, ev, ids, unions| {
        let union = unions[0].1;
        assert!(
            failure(ev, union).is_none(),
            "{at}: {:?}",
            failure(ev, union)
        );
        fused += 1;
        let a = ids[0];
        let rim = StableName {
            kind: EntityKind::Edge,
            node: a,
            path: vec![RoleSeg::RimEdge(
                CapEnd::Start,
                crate::fixture::piece(&doc, a, 0, 0),
            )],
        };
        let geo = geometry(ev, union);
        let x = |k: i64| (k * 1_000_000, 0, 0);
        let mut held = Vec::new();
        for k in 0..3 {
            if let Some(sig) = geo.get(&rim_piece(union, a, &rim, Some((k, 3)))) {
                assert_eq!(
                    sig,
                    &vec![x(k.into()), x(i64::from(k) + 1)],
                    "{at}: #{k} of 3"
                );
                held.push(k);
            }
        }
        assert!(
            held.contains(&1),
            "{at}: a does not hold its middle cell: {held:?}"
        );
        let pieces = geo
            .keys()
            .filter(|n| {
                matches!(n.path.first(), Some(RoleSeg::FromMember { member, of })
                    if *member == a && **of == rim)
            })
            .count();
        assert_eq!(
            pieces,
            held.len(),
            "{at}: a piece of a's rim outside the three cells"
        );
    });
    assert_eq!(fused, 6);
}

/// **A vertex name cites a member edge whole, and lies on it.** A seam
/// vertex names the entities that cross at it; the fold wrote the member
/// edge a face crossed as far as it had cut it by then (`#k of n` of THAT
/// step), which is fold history and names no published piece. Over every
/// case and order, each edge a vertex's seam cites in the union's space
/// is a member edge with no rank, and the vertex lies on that edge in
/// the member's own body. On main 112 cited edges carried a fold rank;
/// ranked over the finished body without this rewrite, 318 cited a rank
/// no published row carries.
#[test]
fn a_vertex_cites_a_member_edge_whole_and_lies_on_it() {
    let mut cited = 0;
    for case in cases() {
        runs(&case, |at, ev, _, unions| {
            for &(tag, union) in unions {
                if failure(ev, union).is_some() {
                    continue;
                }
                let body = body_of(ev, union);
                let point =
                    |b: &topo::Body<f64>, v| *b.get_point(b.get_vertex(v).unwrap().point).unwrap();
                for (name, entry) in table(ev, union).iter() {
                    let (EntityKind::Vertex, Entry::Unique(e)) = (name.kind, entry) else {
                        continue;
                    };
                    let EntityKey::Vertex(v) = e.key else {
                        panic!("{name:?} names no vertex")
                    };
                    let p = point(body, v);
                    for seg in &name.path {
                        let RoleSeg::Seam { a, b } = seg else {
                            continue;
                        };
                        for side in [a.name(), b.name()] {
                            if side.kind != EntityKind::Edge || side.node != union {
                                continue;
                            }
                            let Some(RoleSeg::FromMember { member, of }) = side.path.first() else {
                                continue;
                            };
                            let here = format!("{} {tag} {at}: {name:?}", case.label);
                            assert_eq!(side.path.len(), 1, "{here} cites a piece: {side:?}");
                            let Some(Entry::Unique(me)) = table(ev, *member).lookup(of) else {
                                panic!("{here}: its member does not name {of:?} once")
                            };
                            let EntityKey::Edge(k) = me.key else {
                                panic!("{here}: not an edge")
                            };
                            let mb = body_of(ev, *member);
                            let edge = mb.get_edge(k).unwrap();
                            let end = |he| point(mb, mb.get_half_edge(he).unwrap().start);
                            let (q0, q1) = (end(edge.he_plus), end(edge.he_minus));
                            let d = q1 - q0;
                            let off = (p - q0).cross(d).norm() / d.norm();
                            let s = (p - q0).dot(d) / d.dot(d);
                            assert!(
                                off < 1e-9 && s > -1e-9 && s < 1.0 + 1e-9,
                                "{here} at {p:?} is off the edge it cites, {q0:?}..{q1:?}"
                            );
                            cited += 1;
                        }
                    }
                }
            }
        });
    }
    assert!(cited > 1000, "only {cited} cited member edges checked");
}

/// **`fam010`, the row's own case.** `a`'s bottom-y rim (segment 0,
/// x = 0 → 1 at y = 0, z = 1) is cut by the body's vertices at 0.3, 0.4
/// and 0.5 into four cells, numbered along +x whoever holds them: cell 1
/// is inside `g`, and cell 3 is the stretch `a` runs flush with `b`,
/// which `a` holds in `[a, b, g]` and `b` in `[b, a, g]`. On main the
/// name `#1 of 2` was x = 0.5..1.0 in the first order and x = 0.4..0.5
/// in the second.
#[test]
fn fam010_ranks_a_rim_the_same_way_in_both_orders() {
    let g = ((0.3, 0.4), (-1.0, 0.5), (0.5, 3.0));
    let (doc, ids) = document(
        &[
            ((0.0, 1.0), (0.0, 1.0), (0.0, 1.0)),
            ((0.5, 1.5), (0.0, 1.0), (0.0, 1.0)),
            g,
        ],
        &[0, 1, 2],
    );
    let (a, b, c) = (ids[0], ids[1], ids[2]);
    let rim = StableName {
        kind: EntityKind::Edge,
        node: a,
        path: vec![RoleSeg::RimEdge(
            CapEnd::End,
            crate::fixture::piece(&doc, a, 0, 0),
        )],
    };
    let span = |order: [editor_core::RecipeNodeId; 3], rank| {
        let (docx, union, _) =
            declared_union(doc.clone(), &order, flush_pairs(&doc, (a, a), (b, b)));
        let ev = run(&docx);
        assert!(
            failure(&ev, union).is_none(),
            "{order:?}: {:?}",
            failure(&ev, union)
        );
        let geo = geometry(&ev, union);
        let sig = geo
            .get(&rim_piece(union, a, &rim, Some(rank)))
            .unwrap_or_else(|| panic!("{order:?}: no piece {rank:?}"))
            .clone();
        sig.iter().map(|p| p.0).collect::<Vec<_>>()
    };
    let x = |a: f64, b: f64| vec![(a * 1e6).round() as i64, (b * 1e6).round() as i64];
    for order in [[a, b, c], [b, a, c]] {
        assert_eq!(span(order, (0, 4)), x(0.0, 0.3));
        assert_eq!(span(order, (2, 4)), x(0.4, 0.5));
    }
    assert_eq!(span([a, b, c], (3, 4)), x(0.5, 1.0));
}
