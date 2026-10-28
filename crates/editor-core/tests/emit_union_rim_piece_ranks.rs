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
//! (`emit_union::Flush`), and its faces and seam edges for their parents
//! (`emit_union::name_by_parents`), so every name is published in every
//! fused order, and every document below publishes one table in all of
//! them.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::{BTreeMap, BTreeSet};

use crate::blocks::{failure, run};
use crate::corpus::body_of;
use crate::emit_shared_rim_several::{
    Bx, construction, document, own_name, permutations, probe_corpus,
};
use crate::fixture::{face_vertices, len, table};

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
            EntityKey::Face(f) => face_vertices(&body, f).into_iter().map(point).collect(),
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
            EntityKey::Face(f) => face_vertices(&body, f)
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
    /// Blocks 0 and 1 in an inner union (both orders), and that union
    /// with the rest in an outer one (every order).
    nested: bool,
    /// A block behind a `Transform` translated along x: `(block, dx)`.
    shift: Option<(usize, f64)>,
}

impl Case {
    pub(crate) fn flat(label: &str, blocks: Vec<Bx>, creation: Vec<usize>) -> Self {
        Case {
            label: label.into(),
            blocks,
            creation,
            nested: false,
            shift: None,
        }
    }

    fn nested(label: &str, blocks: Vec<Bx>) -> Self {
        let creation = (0..blocks.len()).collect();
        Case {
            label: label.into(),
            blocks,
            creation,
            nested: true,
            shift: None,
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
/// A block `b` lies inside (`r5covered`), and one `b` pokes out of the
/// top of (`r5poke`).
const A2: Bx = ((0.0, 2.0), (0.0, 1.0), (0.0, 1.0));
const BTALL: Bx = ((0.5, 1.5), (0.0, 1.0), (0.0, 2.0));
/// `BTALL` higher (`r5pokehi`), so that `a`'s node id is the lesser:
/// its judgement runs `a` as operand A, keeps `a`'s copy of the bottom
/// and merges no face of `b` into it, so only the covered record links
/// the two bottoms. `r5poke` runs `b` as operand A; `r5covered` and
/// `r5coveredids`, created the other way round, are the same pair for
/// `b` inside `a`.
const BTALLER: Bx = ((0.5, 1.5), (0.0, 1.0), (0.0, 3.0));
/// A block flush with `a` and one swallowing it (`xmerge`).
const XB: Bx = ((0.75, 1.5), (0.0, 1.0), (0.0, 1.0));
const XC: Bx = ((0.7, 1.6), (-0.1, 1.1), (-0.1, 2.0));
/// A long block, flush with `bend` at one end and `cend` at the other.
const ALONG: Bx = ((0.0, 3.0), (0.0, 1.0), (0.0, 1.0));
const BEND: Bx = ((2.0, 4.0), (0.0, 1.0), (0.0, 1.0));
const CEND: Bx = ((-1.0, 1.0), (0.0, 1.0), (0.0, 1.0));

/// PR 3112's review corpus (`a`, `b` flush), and the review's own
/// fixtures of #3168: one member edge cut in different steps (`r1two`,
/// `r1three`), the same beside a flush partner (`r1flush`), a member
/// flush with TWO partners (`r2ends`, `r2endsg`), three members each
/// flush with the other two (`r4tri`, `r4trig`), a member touching
/// another along a line (`r4touch`), and a union nested in another
/// (`r3nest`, `r3nest2`).
pub(crate) fn cases() -> Vec<Case> {
    let g = ((0.3, 0.4), (-1.0, 2.0), (0.5, 3.0));
    let smid = ((1.4, 1.6), (-1.0, 2.0), (0.5, 3.0));
    let mut out: Vec<Case> = probe_corpus()
        .into_iter()
        .map(|(label, blocks, creation)| Case::flat(&label, blocks, creation))
        .collect();
    out.extend([
        Case::flat("r1two", vec![A, S1, S2], vec![0, 1, 2]),
        Case::flat("r1three", vec![A, S1, S2, S3], vec![0, 1, 2, 3]),
        Case::flat("r1flush", vec![A, B, S1, S2], vec![0, 1, 2, 3]),
        r2ends(),
        Case::flat("r2endsg", vec![ALONG, BEND, CEND, smid], vec![0, 1, 2, 3]),
        Case::flat("r4tri", vec![A, B, C8], vec![0, 1, 2]),
        Case::flat("r4trig", vec![A, B, C8, g], vec![0, 1, 2, 3]),
        Case::flat("r4touch", vec![A, B, TOUCH], vec![0, 1, 2]),
        // `b` inside `a`, flush on both caps and both y-walls: `b` adds
        // no surface, and its faces are held through `a`'s.
        Case::flat("r5covered", vec![A2, B], vec![0, 1]),
        // `b` out of `a`'s top, flush on the y-walls and the bottom:
        // its bottom is held through `a`'s, and its y-walls add surface.
        Case::flat("r5poke", vec![A2, BTALL], vec![0, 1]),
        Case::flat("r5coveredids", vec![A2, B], vec![1, 0]),
        Case::flat("r5pokehi", vec![A2, BTALLER], vec![0, 1]),
        // `b` flush with `a` and merged with it, then swallowed by `c`.
        Case::flat("xmerge", vec![A, XB, XC], vec![0, 1, 2]),
        Case::nested("r3nest", vec![A, B, g]),
        Case::nested("r3nest2", vec![A, B, S1, S2]),
    ]);
    out
}

/// The split cases the row also runs under a value edit: `(case, block,
/// dx)` moves that block, a slab dividing `a`'s top, `dx` along x.
const EDITED: &[(&str, usize, f64)] = &[("r1two", 1, 0.05), ("r1flush", 2, 0.05)];

impl Case {
    /// This case with block `i`'s placement slid `dx` along x: for two
    /// values of `dx`, the same recipe, node for node and step for
    /// step, with only a value changed, as a value edit leaves it.
    fn moved(&self, i: usize, dx: f64) -> Case {
        Case {
            label: format!("{} moved {dx}", self.label),
            blocks: self.blocks.clone(),
            creation: self.creation.clone(),
            nested: self.nested,
            shift: Some((i, dx)),
        }
    }
}

/// Each fused run of `case`, by order and union tag → the centroid of
/// each uniquely named entity.
fn centroids(case: &Case) -> BTreeMap<String, BTreeMap<StableName, [f64; 3]>> {
    let mut out = BTreeMap::new();
    runs(case, |at, ev, _, unions| {
        for &(tag, union) in unions {
            if failure(ev, union).is_some() {
                continue;
            }
            let at_union = geometry(ev, union)
                .into_iter()
                .map(|(n, ps)| {
                    let k = ps.len() as f64;
                    let sum = ps.iter().fold([0.0; 3], |c, p| {
                        [c[0] + p.0 as f64, c[1] + p.1 as f64, c[2] + p.2 as f64]
                    });
                    (n, sum.map(|c| c / k / 1e6))
                })
                .collect();
            out.insert(format!("{at} {tag}"), at_union);
        }
    });
    out
}

fn r2ends() -> Case {
    Case::flat("r2ends", vec![ALONG, BEND, CEND], vec![0, 1, 2])
}

/// Every run of `case`: `each` is handed the orders that built it, the
/// evaluation, the member ids by block, and the unions to read, tagged.
pub(crate) fn runs(
    case: &Case,
    mut each: impl FnMut(&str, &editor_core::Evaluation<f64>, &[RecipeNodeId], &[(&str, RecipeNodeId)]),
) {
    runs_in(case, |at, _, ev, ids, unions| each(at, ev, ids, unions));
}

/// [`runs`], handing `each` the document each run evaluated as well, so
/// a member's READ (which a carried name is keyed by) reads back as its
/// node.
pub(crate) fn runs_in(
    case: &Case,
    mut each: impl FnMut(
        &str,
        &editor_core::ProfileDoc,
        &editor_core::Evaluation<f64>,
        &[RecipeNodeId],
        &[(&str, RecipeNodeId)],
    ),
) {
    let (doc, mut ids) = document(&case.blocks, &case.creation);
    // The shift is a value edit of one placement's slide along x — a
    // slot edit — made after the unions are inserted, so every shift of
    // a case names one set of nodes. The root block is shifted through
    // a copy of it placed over itself.
    let (doc, shift) = match case.shift {
        None => (doc, None),
        Some((i, dx)) => {
            let (doc, tr) = match doc.node(ids[i]) {
                Some(Node::Place { .. }) => (doc, ids[i]),
                _ => crate::fixture::xform(doc, ids[i], [0.0; 3], [0.0, 0.0, 1.0], 0.0),
            };
            ids[i] = tr;
            let slot = crate::fixture::value_slot(
                &doc,
                tr,
                editor_core::Coord::Slide(editor_core::TargetAxis::X),
            );
            (doc, Some((tr, slot, dx)))
        }
    };
    let shifted = |doc: editor_core::ProfileDoc| match shift {
        None => doc,
        Some((node, slot, dx)) => {
            crate::fixture::step(
                doc,
                editor_core::DocEdit::SetParam {
                    node,
                    slot,
                    value: len(dx).into(),
                    fresh: Vec::new(),
                },
            )
            .0
        }
    };
    if case.nested {
        for inner in permutations(&[0, 1]) {
            let io: Vec<_> = inner.iter().map(|&i| ids[i]).collect();
            let (d1, u1) = crate::fixture::union_over(doc.clone(), &io);
            let outer: Vec<_> = std::iter::once(u1)
                .chain(ids[2..].iter().copied())
                .collect();
            for olab in permutations(&(0..outer.len()).collect::<Vec<_>>()) {
                let members: Vec<RecipeNodeId> = olab.iter().map(|&i| outer[i]).collect();
                let (docx, top) = crate::fixture::union_over(d1.clone(), &members);
                let docx = shifted(docx);
                let ev = run(&docx);
                each(
                    &format!("{inner:?}{olab:?}"),
                    &docx,
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
        let (docx, union) = crate::fixture::union_over(doc.clone(), &members);
        let docx = shifted(docx);
        let ev = run(&docx);
        each(&format!("{order:?}"), &docx, &ev, &ids, &[("U", union)]);
    }
}

/// **No name rebinds across member orders, and no order refuses what
/// another publishes.** Over every case and every pair of fused orders, a
/// name both tables publish denotes the same thing ([`signature`]: points
/// to 1e-9, shells told apart, ties compared) — for a nested case, in the
/// inner union and in the outer one. A case whose orders do not all
/// publish reds.
///
/// A name one fused order publishes and another does not fails the row,
/// whatever it names: a vertex, a piece of a member edge, a face or a
/// seam edge. Each is named for what the finished body holds — a face
/// for its parent and the parents across its dividing seams, a seam edge
/// for the parents it lies between (N2, N3) — so it is the same in every
/// order.
///
/// The split cases of [`EDITED`] run again under a value edit that
/// slides a dividing slab: in every order, the same names are published
/// before and after, and each denotes an entity the slide moved no
/// further than itself.
#[test]
fn a_name_two_member_orders_both_publish_denotes_the_same_geometry() {
    let mut compared = 0;
    let mut mixed = Vec::new();
    let mut absent: Vec<String> = Vec::new();
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
                let mut kinds: std::collections::BTreeMap<&str, usize> = Default::default();
                for (_, k) in &refusals {
                    *kinds.entry(k.as_str()).or_default() += 1;
                }
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
                    kinds
                        .into_iter()
                        .map(|(k, c)| format!("{k}:{c}"))
                        .collect::<Vec<_>>()
                        .join("/")
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
                    absent.push(format!("{} {tag} {at}: {name:?}", case.label));
                }
            }
        }
    }
    assert!(
        absent.is_empty(),
        "{} names published in one fused order and absent in another; the first: {:?}",
        absent.len(),
        absent.first()
    );
    assert!(
        mixed.is_empty(),
        "cases refuse in some orders and publish in others: {mixed:?}"
    );
    assert!(
        compared > 1000,
        "only {compared} cross-order names compared"
    );
    let mut edited = 0;
    for &(label, block, dx) in EDITED {
        let case = cases().into_iter().find(|c| c.label == label).unwrap();
        let (before, after) = (
            centroids(&case.moved(block, 0.0)),
            centroids(&case.moved(block, dx)),
        );
        assert_eq!(
            before.keys().collect::<Vec<_>>(),
            after.keys().collect::<Vec<_>>(),
            "{label}: the edit changed which orders fuse"
        );
        for (at, was) in &before {
            let now = &after[at];
            assert_eq!(
                was.keys().collect::<Vec<_>>(),
                now.keys().collect::<Vec<_>>(),
                "{label} {at}: the edit changed the published names"
            );
            for (name, p) in was {
                let q = now[name];
                let d =
                    ((p[0] - q[0]).powi(2) + (p[1] - q[1]).powi(2) + (p[2] - q[2]).powi(2)).sqrt();
                assert!(d <= dx + 1e-6, "{label} {at}: {name:?} moved {d}");
                edited += 1;
            }
        }
    }
    assert!(
        edited > 1000,
        "only {edited} names compared across the edit"
    );
}

/// A published table as [`signature`] reads it.
type Signatures = BTreeMap<StableName, String>;

/// **A union publishes one table in every member order that fuses.**
/// For every case, and each union of a nested one, with at least two
/// fused orders, every fused order's table — every name, and what each
/// denotes ([`signature`]) — is the same: a flush pair alone or with a
/// slab through, beside or across its flush stretch; a face merged and
/// cut, or cut by several members; a member flush with two others;
/// three members each flush with the other two; a member another
/// covers, wholly or on one face; a member merged and then swallowed; a
/// member touching another along a line; a union nested in another.
#[test]
fn a_flush_union_publishes_one_table_in_every_member_order() {
    let mut checked = 0;
    for case in cases() {
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
            let [(first_at, first), rest @ ..] = published.as_slice() else {
                continue;
            };
            if rest.is_empty() {
                continue;
            }
            for (at, table) in rest {
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
        checked, 52,
        "unions with two or more fused orders checked (a nested case has two unions)"
    );
}

/// **A member face another member holds is linked into its parent in
/// every order** (N2): `b` inside `a` (`r5covered`) is a constituent of
/// both caps and both y-walls; `b` out of `a`'s top (`r5poke`) of the
/// bottom, whose region `a` holds, and of the y-walls, where it adds
/// surface; `b` merged with `a` and then swallowed by `c` (`xmerge`) of
/// each face of `a`'s rest that it merged with, though no surface of it
/// is left. Counted are the published faces `Merged` of a face of `a`
/// and a face of `b`, in each fused order; the orders that fold `b`
/// last discard it whole, and those that fold it first merge it.
#[test]
fn a_member_face_another_holds_is_a_constituent_in_every_order() {
    for (label, want) in [
        ("r5covered", 4),
        ("r5coveredids", 4),
        ("r5poke", 3),
        ("r5pokehi", 3),
        ("xmerge", 4),
    ] {
        let case = cases().into_iter().find(|c| c.label == label).unwrap();
        let mut fused = 0;
        runs_in(&case, |at, doc, ev, ids, unions| {
            let union = unions[0].1;
            assert!(
                failure(ev, union).is_none(),
                "{label} {at}: {:?}",
                failure(ev, union)
            );
            fused += 1;
            let of = |n: &StableName| match n.path.as_slice() {
                [RoleSeg::From { read, .. }] => doc.operation_of(*read),
                _ => None,
            };
            let merged = table(ev, union)
                .iter()
                .filter(|(n, _)| n.kind == EntityKind::Face)
                .filter(|(n, _)| match n.path.as_slice() {
                    [RoleSeg::Merged(set)] => {
                        let held: Vec<_> = set.iter().filter_map(&of).collect();
                        held.contains(&ids[0]) && held.contains(&ids[1])
                    }
                    _ => false,
                })
                .count();
            assert_eq!(merged, want, "{label} {at}: faces merging a's and b's");
        });
        assert!(fused >= 2, "{label}: only {fused} orders fuse");
    }
}

/// **A member flush with two others names their shared rim line for
/// the set of the three rims, in every order.** `r2ends`: `a` = x 0..3
/// is flush with `b` (x 2..4) and with `c` (x −1..1). Their bottom start
/// rims (y = z = 0) lie on one line, and the output stage joins it into
/// one edge from x = −1 to 4, along all three rims and within none: it
/// is named `Merged` of the three, whichever member each fold step met
/// first.
#[test]
fn a_member_flush_with_two_others_names_its_rim_by_the_body() {
    let case = r2ends();
    let (doc, _) = document(&case.blocks, &case.creation);
    let mut fused = 0;
    let mut named = BTreeSet::new();
    runs(&r2ends(), |at, ev, ids, unions| {
        let union = unions[0].1;
        assert!(
            failure(ev, union).is_none(),
            "{at}: {:?}",
            failure(ev, union)
        );
        fused += 1;
        let mut set: Vec<StableName> = ids
            .iter()
            .map(|&m| {
                crate::fixture::member_entity(
                    union,
                    crate::fixture::out(&doc, m),
                    own_name(
                        &doc,
                        m,
                        EntityKind::Edge,
                        vec![RoleSeg::RimEdge(
                            CapEnd::Start,
                            crate::fixture::piece(&doc, construction(&doc, m), 0, 0).into(),
                        )],
                    ),
                    EntityKind::Edge,
                )
            })
            .collect();
        set.sort();
        let line = StableName {
            kind: EntityKind::Edge,
            node: union,
            path: vec![RoleSeg::Merged(set)],
        };
        let geo = geometry(ev, union);
        let x = |k: i64| (k * 1_000_000, 0, 0);
        assert_eq!(geo.get(&line), Some(&vec![x(-1), x(4)]), "{at}: {line:?}");
        named.insert(line);
    });
    assert_eq!(fused, 6);
    assert_eq!(named.len(), 1, "one name in every order");
}

/// **A vertex name cites a member edge whole, and lies on it.** A seam
/// or crossing vertex names the entities that cross at it; the fold wrote the member
/// edge a face crossed as far as it had cut it by then (`#k of n` of THAT
/// step), which is fold history and names no published piece. Over every
/// case and order, each edge a vertex's seam cites in the union's space
/// is a member edge with no rank, or the set of several flush on one
/// line through the vertex, and the vertex lies on each such edge in its
/// member's own body. On main 112 cited edges carried a fold rank;
/// ranked over the finished body without this rewrite, 318 cited a rank
/// no published row carries.
#[test]
fn a_vertex_cites_a_member_edge_whole_and_lies_on_it() {
    let mut cited = 0;
    for case in cases() {
        runs_in(&case, |at, doc, ev, _, unions| {
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
                    let p = point(&body, v);
                    for seg in &name.path {
                        let sides: Vec<&StableName> = match seg {
                            RoleSeg::Seam { a, b }
                            | RoleSeg::Crossing {
                                edge: a, face: b, ..
                            } => vec![a.name(), b.name()],
                            RoleSeg::EdgeCrossing(run) => {
                                run.iter().map(|l| l.line.name()).collect()
                            }
                            _ => continue,
                        };
                        let sides = sides
                            .into_iter()
                            .flat_map(|side| match side.path.as_slice() {
                                [RoleSeg::Merged(line)] if side.kind == EntityKind::Edge => {
                                    line.iter().collect()
                                }
                                _ => vec![side],
                            });
                        for side in sides {
                            if side.kind != EntityKind::Edge || side.node != union {
                                continue;
                            }
                            let here = format!("{} {tag} {at}: {name:?}", case.label);
                            let Some(RoleSeg::From { read, of }) = side.path.first() else {
                                continue;
                            };
                            let member = &doc
                                .operation_of(*read)
                                .unwrap_or_else(|| panic!("{here}: {read:?} is no live read"));
                            assert_eq!(side.path.len(), 1, "{here} cites a piece: {side:?}");
                            let Some(Entry::Unique(me)) = table(ev, *member).lookup(of) else {
                                panic!("{here}: its member does not name {of:?} once")
                            };
                            let EntityKey::Edge(k) = me.key else {
                                panic!("{here}: not an edge")
                            };
                            let mb = body_of(ev, *member);
                            let edge = mb.get_edge(k).unwrap();
                            let end = |he| point(&mb, mb.get_half_edge(he).unwrap().start);
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
/// x = 0 → 1 at y = 0, z = 1) is cut by `g`'s walls at 0.3 and 0.4:
/// x = 0.3..0.4 is inside `g`, x = 0.0..0.3 lies within `a`'s rim alone,
/// and x = 0.4..1.5 runs along `a`'s rim and on along `b`'s. In both
/// orders the second is named for the set of the two rims, and the
/// first, the one piece of `a`'s rim outside it, by its ends.
#[test]
fn fam010_names_a_rim_the_same_way_in_both_orders() {
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
    let rim = |m: RecipeNodeId| {
        own_name(
            &doc,
            m,
            EntityKind::Edge,
            vec![RoleSeg::RimEdge(
                CapEnd::End,
                crate::fixture::piece(&doc, construction(&doc, m), 0, 0).into(),
            )],
        )
    };
    // x-span → the name there, for every edge along the rim line.
    let along = |order: [editor_core::RecipeNodeId; 3]| {
        let (docx, union) = crate::fixture::union_over(doc.clone(), &order);
        let ev = run(&docx);
        assert!(
            failure(&ev, union).is_none(),
            "{order:?}: {:?}",
            failure(&ev, union)
        );
        let names: BTreeMap<_, _> = geometry(&ev, union)
            .into_iter()
            .filter(|(n, sig)| {
                n.kind == EntityKind::Edge && sig.iter().all(|p| p.1 == 0 && p.2 == 1_000_000)
            })
            .map(|(n, sig)| (sig.iter().map(|p| p.0).collect::<Vec<_>>(), n))
            .collect();
        (union, names)
    };
    let x = |a: f64, b: f64| vec![(a * 1e6).round() as i64, (b * 1e6).round() as i64];
    let ((u1, first), (u2, second)) = (along([a, b, c]), along([b, a, c]));
    for (union, table) in [(u1, &first), (u2, &second)] {
        let whole = crate::fixture::member_entity(
            union,
            crate::fixture::out(&doc, a),
            rim(a),
            EntityKind::Edge,
        );
        let mut set = vec![
            whole.clone(),
            crate::fixture::member_entity(
                union,
                crate::fixture::out(&doc, b),
                rim(b),
                EntityKind::Edge,
            ),
        ];
        set.sort();
        let joined = StableName {
            kind: EntityKind::Edge,
            node: union,
            path: vec![RoleSeg::Merged(set)],
        };
        assert_eq!(table.len(), 2, "the edges along a's rim line: {table:?}");
        assert_eq!(table.get(&x(0.4, 1.5)), Some(&joined), "{table:?}");
        let piece = &table[&x(0.0, 0.3)];
        assert_eq!(piece.path[..1], whole.path[..], "{piece:?}");
        assert!(
            matches!(
                piece.path[1..],
                [RoleSeg::Fragment(editor_core::Qualifier::Ends(_))]
            ),
            "{piece:?}"
        );
    }
}

/// `h` of the review corpus: x 1.0..1.1, its x = 1.0 wall against `a`'s
/// x = 1 wall, which `b` (x 0.5..1.5) covers.
const H: Bx = ((1.0, 1.1), (-1.0, 2.0), (0.5, 3.0));

/// Every order of `blocks`: the order, and its table ([`signature`])
/// where it fuses, or the refusal's variant.
fn outcomes(blocks: &[Bx], creation: &[usize]) -> Vec<(Vec<usize>, Result<Signatures, String>)> {
    let (doc, ids) = document(blocks, creation);
    permutations(&(0..blocks.len()).collect::<Vec<_>>())
        .into_iter()
        .map(|order| {
            let members: Vec<_> = order.iter().map(|&i| ids[i]).collect();
            let (docx, union) = crate::fixture::union_over(doc.clone(), &members);
            let ev = run(&docx);
            let variant = |shown: String| {
                shown
                    .split([' ', '(', '{'])
                    .next()
                    .unwrap_or("")
                    .to_string()
            };
            let got = match failure(&ev, union) {
                None => Ok(signature(&ev, union)),
                Some(editor_core::NodeErrorKind::Naming(e)) => Err(variant(format!("{e:?}"))),
                Some(e) => Err(variant(format!("{e:?}"))),
            };
            (order, got)
        })
        .collect()
}

/// **Every order of `blocks` fuses**, grouped by the table it publishes
/// ([`signature`]: each name and what it denotes), in order of first
/// appearance.
fn tables_by_order(label: &str, blocks: &[Bx], creation: &[usize]) -> Vec<Vec<Vec<usize>>> {
    let mut classes: Vec<(Signatures, Vec<Vec<usize>>)> = Vec::new();
    for (at, table) in outcomes(blocks, creation) {
        let table = table.unwrap_or_else(|e| panic!("{label} {at:?}: {e}"));
        match classes.iter_mut().find(|(t, _)| *t == table) {
            Some((_, orders)) => orders.push(at),
            None => classes.push((table, vec![at])),
        }
    }
    classes.into_iter().map(|(_, orders)| orders).collect()
}

/// **A covered contact decides alike in every creation order, and
/// every member order publishes one table** (DM4's contact rule). `row`
/// is `a`, `b`, `g`, `h`; `b` covers the `(a, h)` contact, which the
/// margins decide one carrier. The orders that fold `a` and `h` before
/// `b` meet `a`'s corner resting on `h`'s wall, which is that corner
/// (N2).
#[test]
fn a_covered_contact_decides_alike_in_every_order() {
    let g = ((0.3, 0.4), (-1.0, 2.0), (0.5, 3.0));
    for (label, creation) in [("row", [0, 1, 2, 3]), ("rowids", [3, 2, 1, 0])] {
        let classes = tables_by_order(label, &[A, B, g, H], &creation);
        assert_eq!(
            classes.len(),
            1,
            "{label}: one table in every order: {classes:?}"
        );
    }
}

/// **The covered-contact row itself: `{a, b, h}`.** Every order fuses
/// and publishes one table: `a`'s two top rims along `x`, which `h`'s
/// wall cuts from `b`'s, are each a whole member edge and take the bare
/// name, though the orders that fold `a` and `b` first meet them as
/// pieces of the rim the two members' flush walls joined.
#[test]
fn a_contact_b_covers_decides_in_every_order() {
    assert_eq!(
        tables_by_order("{a, b, h}", &[A, B, H], &[0, 1, 2]),
        [vec![
            vec![0, 1, 2],
            vec![0, 2, 1],
            vec![1, 0, 2],
            vec![1, 2, 0],
            vec![2, 0, 1],
            vec![2, 1, 0],
        ]]
    );
}
