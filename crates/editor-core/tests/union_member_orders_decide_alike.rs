//! **A union decides alike in every member order** (REFERENCES DM4,
//! FORK-S4U as Ev approved it on PR 4323: "would be good to have either
//! good tests or possibly debug asserts to detect behavior that we
//! think should remain order independent").
//!
//! The pairwise pass judges every member pair in member space, and the
//! fold reads one verdict per carrier pair through member lineage and
//! re-glues none (`topo::nary_boolean` holds that at the verdict site
//! in every debug run). So each union below is
//! evaluated under every order of its members, a seeded sample of
//! [`SAMPLE`] orders where `n!` is larger, and these are equal across
//! the orders:
//!
//! - **the outcome's kind**: every order builds, or every order refuses;
//! - **a refusal**: the pass's own, the same in every order — or a
//!   fold step's ([`NodeErrorKind::UnionFoldStep`]), whose inner
//!   refusal is of one kind in every order (the member it names is the
//!   one each order folds in, and differs);
//! - **the judged rows**: the union's order-free rows (DM4), each the
//!   member cells it names, unordered, and the pair of members that
//!   judged it, as a multiset. The fold steps' rows are the evaluation's
//!   log of what each order met, and follow the order; the log is pinned
//!   as a deterministic function of it
//!   ([`the_log_is_a_deterministic_function_of_the_member_order`]);
//! - **the names**: every name of the union's table beside the geometry
//!   it binds, the union's own id spelled as one placeholder (each order
//!   is its own node);
//! - **the topology**: the result's face, edge and vertex counts, and
//!   its volume.
//!
//! **What follows list order on purpose, and is excluded by name:** the
//! result's description bits. A glued face keeps the description of the
//! member folded first (D10, ordinarily operand A's), so two orders hold
//! one solid in two descriptions of its glued faces; [`compare`] reads
//! geometry through the names and the volume, never through `Body`'s
//! bits.
//!
//! **Scope.** The corpus is planar blocks: rests, continuations, a
//! transverse union and an in-band sliver; vertex rows are pinned
//! beside it on an L-prism and a tilted cube that pinch, and on blocks
//! that kiss at a corner. Unions known to decide by
//! member order are pinned where their issues own them, not here: the
//! capsule's rod ending on its joint
//! (`work/tang/the-capsule-rod-ending-on-the-joint-parts-by-member-order.md`).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeMap;

use crate::blocks::{Spans, block, block_of, block_on, block_on_of, failure, run};
use crate::corpus::body_of;
use crate::emit_boolean_vertex_keys::named_geometry;
use crate::fixture::{against, axes_at, desc, insert, len};
use editor_core::{Evaluation, Node, NodeErrorKind, ProfileDoc, RecipeNodeId};
use geom_core::Tol;

/// The orders sampled where a union's members have more than this
/// many orders.
const SAMPLE: usize = 24;

type Members = fn(ProfileDoc) -> (ProfileDoc, Vec<RecipeNodeId>);

/// Three unit blocks in a row along x, each flush on the next: two
/// rests and the continuations of their shared walls.
pub(crate) fn three_in_a_row(doc: ProfileDoc) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, b) = block_on(doc, a, (1.0, 2.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, c) = block_on(doc, a, (2.0, 3.0), (0.0, 1.0), 0.0, 1.0);
    (doc, vec![a, b, c])
}

/// Three plates stacked, each smaller than the one below: two rests,
/// no continuation.
pub(crate) fn stepped_stack(doc: ProfileDoc) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let (doc, a) = block(doc, (0.0, 3.0), (0.0, 3.0), 1.0);
    let (doc, b) = block_on(doc, a, (0.5, 2.5), (0.5, 2.5), 1.0, 1.0);
    let (doc, c) = block_on(doc, a, (1.0, 2.0), (1.0, 2.0), 2.0, 1.0);
    (doc, vec![a, b, c])
}

/// A table: a top resting on four legs at its corners, each leg flush
/// with two of the top's walls — 120 orders, sampled.
pub(crate) fn table(doc: ProfileDoc) -> (ProfileDoc, Vec<RecipeNodeId>) {
    // The top's own z = 0 stands 2 up: the legs reach down to z = -2.
    let (doc, top) = block(doc, (0.0, 4.0), (0.0, 3.0), 0.25);
    let (doc, l0) = block_on(doc, top, (0.0, 0.5), (0.0, 0.5), -2.0, 2.0);
    let (doc, l1) = block_on(doc, top, (3.5, 4.0), (0.0, 0.5), -2.0, 2.0);
    let (doc, l2) = block_on(doc, top, (0.0, 0.5), (2.5, 3.0), -2.0, 2.0);
    let (doc, l3) = block_on(doc, top, (3.5, 4.0), (2.5, 3.0), -2.0, 2.0);
    (doc, vec![top, l0, l1, l2, l3])
}

/// A slab and two posts through it and a bar across both posts: every
/// meeting transverse, no coincidence.
fn posts_and_bar(doc: ProfileDoc) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let (doc, slab) = block(doc, (0.0, 4.0), (0.0, 2.0), 1.0);
    let (doc, p) = block_on(doc, slab, (0.5, 1.0), (0.75, 1.25), -0.5, 2.5);
    let (doc, q) = block_on(doc, slab, (3.0, 3.5), (0.75, 1.25), -0.5, 2.5);
    let (doc, bar) = block_on(doc, slab, (0.25, 3.75), (0.875, 1.125), 1.5, 0.25);
    (doc, vec![slab, p, q, bar])
}

/// Two blocks flush and a third standing a sliver off the second's far
/// wall: the sliver refuses, in band, whichever member meets it first.
fn a_sliver_beside_a_rest(doc: ProfileDoc) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let eps = Tol::witness().get().eps;
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, b) = block_on(doc, a, (1.0, 2.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, c) = block_on(doc, a, (2.0 + 2.0 * eps, 3.0), (0.25, 0.75), 0.25, 0.5);
    (doc, vec![a, b, c])
}

/// The union corpus: every fixture, by name.
const CORPUS: [(&str, Members); 5] = [
    ("three in a row", three_in_a_row),
    ("stepped stack", stepped_stack),
    ("table", table),
    ("posts and bar", posts_and_bar),
    ("a sliver beside a rest", a_sliver_beside_a_rest),
];

/// Every order of `0..n`, or a seeded sample of [`SAMPLE`] of them
/// where there are more, the identity first.
pub(crate) fn orders(n: usize) -> Vec<Vec<usize>> {
    fn all(n: usize) -> Vec<Vec<usize>> {
        if n == 0 {
            return vec![Vec::new()];
        }
        let mut out = Vec::new();
        for p in all(n - 1) {
            for i in 0..=p.len() {
                let mut q = p.clone();
                q.insert(i, n - 1);
                out.push(q);
            }
        }
        out
    }
    let every = all(n);
    if every.len() <= SAMPLE {
        return every;
    }
    // A fixed linear congruential walk: the same sample on every run.
    let mut state: u64 = 0x9e37_79b9_7f4a_7c15;
    let mut picked: Vec<Vec<usize>> = vec![(0..n).collect()];
    while picked.len() < SAMPLE {
        state = state
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        let p = &every[usize::try_from(state >> 33).unwrap() % every.len()];
        if !picked.contains(p) {
            picked.push(p.clone());
        }
    }
    picked
}

/// What one order of a union decides, in the terms the orders are
/// compared in.
#[derive(Debug, PartialEq)]
enum Decided {
    /// The union built.
    Built {
        rows: Vec<String>,
        names: BTreeMap<String, String>,
        topology: (usize, usize, usize),
        volume: f64,
    },
    /// The pass refused, the same in every order.
    Refused(String),
    /// A fold step refused: its inner refusal's kind. The member it
    /// names moves with the order, and is checked where it is read.
    FoldStep(String),
}

/// The union of `members` in `order`, evaluated, read as [`Decided`].
fn decide(
    fixture: &dyn Fn(ProfileDoc) -> (ProfileDoc, Vec<RecipeNodeId>),
    order: &[usize],
) -> Decided {
    let doc = ProfileDoc::empty_derived("union-member-orders", Tol::witness());
    let (doc, members) = fixture(doc);
    let (doc, union) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(
                order.iter().map(|&i| members[i].into()).collect(),
            ),
        },
    );
    let ev = run(&doc);
    match failure(&ev, union) {
        Some(NodeErrorKind::UnionFoldStep {
            member,
            read,
            refusal,
        }) => {
            // The step names the member it folds in: never the first,
            // which no step folds in, and that place's read.
            let at = usize::try_from(*member).expect("a member index");
            assert!(
                (1..order.len()).contains(&at),
                "a fold step names a member a step folds in: {member} of {order:?}"
            );
            assert_eq!(
                *read,
                doc.output(members[order[at]], 0).expect("a member's read"),
                "the fold step at member {member} names that member's read"
            );
            let inner = format!("{refusal:?}");
            let kind = inner
                .split(|c: char| !c.is_alphanumeric() && c != '_')
                .next()
                .unwrap_or_default()
                .to_string();
            Decided::FoldStep(kind)
        }
        Some(other) => Decided::Refused(format!("{other:?}")),
        None => {
            cells_are_held(&ev, union, order);
            built(&ev, union, &members, order)
        }
    }
}

/// What [`cells_are_held`] checked: published rows, their member
/// cells, the rows of three cells or more, and the rows whose cells
/// were found to share a point ([`shared_point`]).
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Held {
    rows: usize,
    cells: usize,
    meetings: usize,
    sited: usize,
}

/// **Every cell of a union's published row is a member-table row**
/// (DM4, "A row names member cells"): its name is a strict row of the
/// table of a member's read, of the cell's kind, whose key is an entity
/// of that member's body; no cell is read in the union's own table. A
/// fold row is named against the fold's intermediate tables, and a cell
/// left in their terms would name nothing a member holds.
///
/// Each row's cells must also share a point where the fixture's
/// geometry lets that be read ([`shared_point`]), so a row spelled from
/// the wrong member cells reds here though every name is held.
pub(crate) fn cells_are_held(ev: &Evaluation<f64>, union: RecipeNodeId, order: &[usize]) -> Held {
    use editor_core::{EntityKey, Entry, NamedCell};
    let value = ev.value(union).expect("the union evaluated");
    let own = ev.output_var(union, 0).expect("a union defines its output");
    let mut held = Held {
        rows: value.coincidences.len(),
        ..Held::default()
    };
    for r in value.coincidences.iter() {
        for cell in &r.cells {
            let NamedCell::Entity { input, name } = cell else {
                continue;
            };
            let at = |why: &str| {
                format!("{order:?}: a published row's cell {name:?} of {input:?} {why}: {r:?}")
            };
            assert_ne!(*input, own, "{}", at("is read in the union's own table"));
            let Some(Entry::Unique(e)) = ev.table(*input).and_then(|t| t.lookup(name)) else {
                panic!("{}", at("is no strict name of its read's table"));
            };
            assert_eq!(e.key.kind(), name.kind, "{}", at("denotes another kind"));
            let body = ev
                .read_body(*input, e.body)
                .expect("the read evaluated")
                .expect("the read holds the cell's body");
            let live = match e.key {
                EntityKey::Body => true,
                EntityKey::Face(k) => body.body().faces().any(|(f, _)| f == k),
                EntityKey::Edge(k) => body.body().edges().any(|(f, _)| f == k),
                EntityKey::Vertex(k) => body.body().vertices().any(|(f, _)| f == k),
            };
            assert!(live, "{}", at("denotes no entity of its read's body"));
            held.cells += 1;
        }
        held.meetings += usize::from(r.cells.len() > 2);
        if r.relation == topo::Relation::OnCarrier {
            assert!(
                matches!(&r.cells[0], NamedCell::Entity { name, .. }
                    if name.kind == editor_core::EntityKind::Vertex),
                "{order:?}: an `OnCarrier` row puts a point on a cell, its first cell a vertex: {r:?}"
            );
        }
        if matches!(r.relation, topo::Relation::OnCarrier | topo::Relation::Meet)
            && let Some(on) = shared_point(ev, &r.cells)
        {
            assert!(
                on,
                "{order:?}: a published row's cells share no point: {r:?}"
            );
            held.sited += 1;
        }
    }
    held
}

/// A cell's carrier, where it is one the site check reads: a vertex's
/// point, a straight edge's line (a point on it and its direction), a
/// planar face's plane (a point on it and its normal).
enum Carrier {
    Point(geom_core::Point3<f64>),
    Line(geom_core::Point3<f64>, geom_core::Vec3<f64>),
    Plane(geom_core::Point3<f64>, geom_core::Vec3<f64>),
}

fn carrier(ev: &Evaluation<f64>, cell: &editor_core::NamedCell) -> Option<Carrier> {
    let editor_core::NamedCell::Entity { input, name } = cell else {
        return None;
    };
    let Some(editor_core::Entry::Unique(e)) = ev.table(*input).and_then(|t| t.lookup(name)) else {
        return None;
    };
    // Where the cell stands in its space: the rows across cells read
    // through two reads compare there.
    let body = &crate::fixture::in_space(ev.read_body(*input, e.body).ok()?.ok()?);
    match e.key {
        editor_core::EntityKey::Vertex(v) => Some(Carrier::Point(crate::fixture::point(body, v))),
        editor_core::EntityKey::Edge(k) => {
            let edge = body.get_edge(k)?;
            let straight = body
                .get_curve_geom(edge.curve)
                .and_then(|g| g.certified())
                .is_some_and(|c| matches!(c.carrier(), geom::Curve3::Line { .. }));
            if !straight {
                return None;
            }
            let [u, w] = crate::fixture::ends(body, k).map(|v| crate::fixture::point(body, v));
            Some(Carrier::Line(u, w - u))
        }
        editor_core::EntityKey::Face(f) => {
            let (_, face) = body.faces().find(|&(k, _)| k == f)?;
            match body.get_surface(face.surface)? {
                topo::Surface::Plane { origin, normal, .. } => {
                    Some(Carrier::Plane(*origin, *normal))
                }
                _ => None,
            }
        }
        editor_core::EntityKey::Body => None,
    }
}

/// **Whether `cells` share one point**, within the witness ε: the
/// point is a vertex cell's, else a straight edge's crossing with a
/// plane it does not lie along, else three planes' meeting; it must lie
/// on every cell's carrier. `None` where some cell has a carrier the
/// check does not read, or no point is fixed.
fn shared_point(ev: &Evaluation<f64>, cells: &[editor_core::NamedCell]) -> Option<bool> {
    let carriers: Vec<Carrier> = cells
        .iter()
        .map(|c| carrier(ev, c))
        .collect::<Option<_>>()?;
    let eps = Tol::witness().eps();
    let on = |p: geom_core::Point3<f64>, c: &Carrier| match c {
        Carrier::Point(q) => (p - *q).norm() <= eps,
        Carrier::Line(u, d) => (p - *u).cross(*d).norm() / d.norm() <= eps,
        Carrier::Plane(o, n) => (p - *o).dot(*n).abs() / n.norm() <= eps,
    };
    let point = carriers
        .iter()
        .find_map(|c| match c {
            Carrier::Point(p) => Some(*p),
            _ => None,
        })
        .or_else(|| {
            carriers.iter().find_map(|l| {
                let Carrier::Line(u, d) = l else { return None };
                carriers.iter().find_map(|q| match q {
                    Carrier::Plane(o, n) if d.dot(*n).abs() > 1e-6 * d.norm() * n.norm() => {
                        Some(*u + *d * ((*o - *u).dot(*n) / d.dot(*n)))
                    }
                    _ => None,
                })
            })
        })
        .or_else(|| {
            let planes: Vec<(geom_core::Point3<f64>, geom_core::Vec3<f64>)> = carriers
                .iter()
                .filter_map(|c| match c {
                    Carrier::Plane(o, n) => Some((*o, *n)),
                    _ => None,
                })
                .collect();
            let [(o1, n1), (o2, n2), (o3, n3)] = planes.get(..3)? else {
                return None;
            };
            let det = n1.dot(n2.cross(*n3));
            if det.abs() < 1e-9 {
                return None;
            }
            let at = |o: &geom_core::Point3<f64>| *o - geom_core::Point3::origin();
            let (d1, d2, d3) = (n1.dot(at(o1)), n2.dot(at(o2)), n3.dot(at(o3)));
            let v = (n2.cross(*n3) * d1 + n3.cross(*n1) * d2 + n1.cross(*n2) * d3) / det;
            Some(geom_core::Point3::origin() + v)
        })?;
    Some(carriers.iter().all(|c| on(point, c)))
}

fn built(
    ev: &Evaluation<f64>,
    union: RecipeNodeId,
    members: &[RecipeNodeId],
    order: &[usize],
) -> Decided {
    let value = ev.value(union).expect("the union evaluated");
    let mut rows: Vec<String> = value
        .coincidences
        .iter()
        .filter_map(|r| judged(r, members, order))
        .collect();
    rows.sort();
    // Each order is its own union node, so its id is spelled as one
    // placeholder wherever a name cites it.
    let own = format!("{union:?}");
    let mut names = BTreeMap::new();
    for line in named_geometry(ev, union).expect("a union is never empty") {
        let line = line.replace(&own, "the union");
        let (n, g) = line.rsplit_once(" @ ").expect("a name @ geometry row");
        assert!(
            names.insert(n.to_string(), g.to_string()).is_none(),
            "{n} binds two entities"
        );
    }
    let body = topo::test_support::finished(
        "body",
        topo::Body::clone(&(body_of(ev, union))),
        Tol::witness(),
    );
    let topology = (
        body.faces().count(),
        body.edges().count(),
        body.vertices().count(),
    );
    let volume =
        topo::test_support::pinned_volume("a built union measures", &body, Tol::witness()).volume;
    Decided::Built {
        rows,
        names,
        topology,
        volume,
    }
}

/// A judged row as its unordered cells beside what was decided between
/// them and the two members that judged it, each by its first place in
/// the fixture's list (a read spelled twice is one member): which
/// member is folded first decides which cell the pass reads first, and
/// nothing else about the row. `None` for a fold step's row.
fn judged(
    r: &editor_core::NamedCoincidence,
    members: &[RecipeNodeId],
    order: &[usize],
) -> Option<String> {
    let editor_core::RowOrigin::Judged { pair } = r.origin else {
        return None;
    };
    let fixture = |p: u32| {
        let node = members[order[p as usize]];
        members.iter().position(|&m| m == node).expect("a member")
    };
    let mut pair = pair.map(fixture);
    pair.sort();
    let mut cells: Vec<String> = r.cells.iter().map(|c| format!("{c:?}")).collect();
    cells.sort();
    // A margin of exactly zero is read without its sign: the union
    // computes in its first member's frame (D10 `[placement.frame]`),
    // and which way a zero rounds is that frame's, not the row's.
    let margin = format!("{:?}", r.margin).replace("Value(-0.0,", "Value(0.0,");
    Some(format!(
        "{cells:?} {:?} {:?} {margin} {:?} {pair:?}",
        r.relation, r.site, r.discharge
    ))
}

/// `a` and `b` decided alike: the volumes to the summation's rounding,
/// everything else exactly.
fn compare(a: &Decided, b: &Decided) -> Result<(), String> {
    match (a, b) {
        (
            Decided::Built {
                rows: ra,
                names: na,
                topology: ta,
                volume: va,
            },
            Decided::Built {
                rows: rb,
                names: nb,
                topology: tb,
                volume: vb,
            },
        ) => {
            if ra != rb {
                return Err(format!("rows {ra:#?} vs {rb:#?}"));
            }
            if ta != tb {
                return Err(format!("topology {ta:?} vs {tb:?}"));
            }
            if (va - vb).abs() > 1e-9 * va.abs().max(1.0) {
                return Err(format!("volume {va} vs {vb}"));
            }
            let moved: Vec<_> = na
                .iter()
                .filter(|(n, g)| nb.get(*n) != Some(*g))
                .map(|(n, g)| format!("{n}: {g} vs {:?}", nb.get(n)))
                .chain(
                    nb.keys()
                        .filter(|n| !na.contains_key(*n))
                        .map(|n| format!("{n}: absent vs present")),
                )
                .collect();
            if moved.is_empty() {
                Ok(())
            } else {
                Err(format!("names {moved:#?}"))
            }
        }
        (x, y) if x == y => Ok(()),
        (x, y) => Err(format!("{x:?} vs {y:?}")),
    }
}

/// **Every union of the corpus decides alike in every member order.**
#[test]
fn every_union_decides_alike_in_every_member_order() {
    let mut failures = Vec::new();
    for (what, fixture) in CORPUS {
        let n = fixture(ProfileDoc::empty_derived("count", Tol::witness()))
            .1
            .len();
        let orders = orders(n);
        let first = decide(&fixture, &orders[0]);
        for order in &orders[1..] {
            if let Err(why) = compare(&first, &decide(&fixture, order)) {
                failures.push(format!("{what}, {:?} vs {order:?}: {why}", orders[0]));
            }
        }
    }
    assert!(failures.is_empty(), "{failures:#?}");
}

/// **The corpus reaches what it is for**: each fixture builds, refuses
/// in the pass, or refuses at a fold step as its doc says, so a
/// fixture that drifts into another class reds here rather than
/// comparing two refusals it was not written for.
#[test]
fn the_corpus_reaches_its_classes() {
    for (what, fixture) in CORPUS {
        let n = fixture(ProfileDoc::empty_derived("count", Tol::witness()))
            .1
            .len();
        let got = decide(&fixture, &(0..n).collect::<Vec<_>>());
        let sliver = what == "a sliver beside a rest";
        assert_eq!(
            matches!(got, Decided::Built { .. }),
            !sliver,
            "{what}: {got:?}"
        );
        if sliver {
            assert!(
                matches!(got, Decided::Refused(_)),
                "{what}: the sliver refuses in the pass, not at a fold step: {got:?}"
            );
        }
        if let Decided::Built { rows, .. } = &got {
            let glued = !matches!(what, "posts and bar");
            assert_eq!(!rows.is_empty(), glued, "{what}: {rows:#?}");
        }
    }
}

/// **Every union of the emit corpus decides alike in every member
/// order** (`emit_shared_rim_several::probe_corpus`: blocks flush on
/// each other, slabs cutting them and faces cut and merged by one
/// step). Each is compared as
/// [`every_union_decides_alike_in_every_member_order`] compares its own.
#[test]
fn every_union_of_the_emit_corpus_decides_alike_in_every_member_order() {
    let mut failures = Vec::new();
    let mut compared = 0;
    for (label, blocks, creation) in crate::emit_shared_rim_several::probe_corpus() {
        let fixture = |_: ProfileDoc| crate::emit_shared_rim_several::document(&blocks, &creation);
        let orders = orders(blocks.len());
        let first = decide(&fixture, &orders[0]);
        for order in &orders[1..] {
            compared += 1;
            if let Err(why) = compare(&first, &decide(&fixture, order)) {
                failures.push(format!("{label}, {:?} vs {order:?}: {why}", orders[0]));
            }
        }
    }
    assert!(compared > 0, "the emit corpus holds unions to compare");
    assert!(failures.is_empty(), "{failures:#?}");
}

/// The L-prism and the tilted cube of `topo`'s
/// `records_cite_their_decision::a_pinch_records_the_vertex_identity_that_made_it`:
/// the cube's bottom face passes through the L's reflex corner
/// (1, 1, 1), and their union pinches there.
pub(crate) fn prism_and_cube(doc: ProfileDoc) -> (ProfileDoc, Vec<RecipeNodeId>) {
    prism_and_cube_along(doc, CUBE_NORMAL)
}

/// The L the prism of [`prism_and_cube`] is drawn over; its reflex
/// corner is (1, 1).
const ELL: [(f64, f64); 6] = [
    (0.0, 0.0),
    (2.0, 0.0),
    (2.0, 1.0),
    (1.0, 1.0),
    (1.0, 2.0),
    (0.0, 2.0),
];

/// The normal of [`prism_and_cube`]'s cube's bottom face, unnormalised.
pub(crate) const CUBE_NORMAL: [f64; 3] = [1.0, 1.3, 0.7];

/// [`prism_and_cube`] with the cube's bottom face on the plane through
/// the L's reflex corner (1, 1, 1) whose normal is `normal`.
pub(crate) fn prism_and_cube_along(
    doc: ProfileDoc,
    normal: [f64; 3],
) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let (doc, profile) = insert(doc, Node::Profile(desc(vec![ELL.to_vec()])));
    let (doc, prism) = insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(1.0),
        },
    );
    // The frame through the corner whose normal is `normal`,
    // normalised: `u` its tilt of +z, `v` = normal × `u`.
    let unit = |m: [f64; 3]| {
        let n = m.iter().map(|c| c * c).sum::<f64>().sqrt();
        m.map(|c| c / n)
    };
    let m = unit(normal);
    let u = unit([-m[2] * m[0], -m[2] * m[1], 1.0 - m[2] * m[2]]);
    let v = [
        m[1] * u[2] - m[2] * u[1],
        m[2] * u[0] - m[0] * u[2],
        m[0] * u[1] - m[1] * u[0],
    ];
    let square = vec![(-2.0, -2.0), (2.0, -2.0), (2.0, 2.0), (-2.0, 2.0)];
    let (doc, profile) = insert(doc, Node::Profile(desc(vec![square])));
    let (doc, cube) = insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(4.0),
        },
    );
    // The cube's copy stands on that frame in the prism's space.
    let (doc, cube) = against(doc, prism, cube, &axes_at([1.0, 1.0, 1.0], u, v));
    (doc, vec![prism, cube])
}

/// [`prism_and_cube`] and a block far from both.
pub(crate) fn pinch_and_far(doc: ProfileDoc) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let (doc, mut m) = prism_and_cube(doc);
    let (doc, far) = block_on(doc, m[0], (6.0, 7.0), (6.0, 7.0), 0.0, 1.0);
    m.push(far);
    (doc, m)
}

/// [`prism_and_cube`] and a block over the corner, which fills the
/// pinch.
pub(crate) fn pinch_and_cap(doc: ProfileDoc) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let (doc, mut m) = prism_and_cube(doc);
    let (doc, cap) = block_on(doc, m[0], (0.8, 1.2), (0.8, 1.2), 0.8, 0.4);
    m.push(cap);
    (doc, m)
}

/// Two unit blocks kissing at the corner (1, 1, 1): one vertex-vertex
/// record, citing its `VertexFusion` row.
pub(crate) fn kiss(doc: ProfileDoc) -> (ProfileDoc, [RecipeNodeId; 2]) {
    let (doc, a) = block_of(doc, KISS[0]);
    let (doc, b) = block_on_of(doc, a, KISS[1]);
    (doc, [a, b])
}

/// [`kiss`]'s two blocks.
pub(crate) const KISS: [Spans; 2] = [
    ((0.0, 1.0), (0.0, 1.0), 0.0, 1.0),
    ((1.0, 2.0), (1.0, 2.0), 1.0, 1.0),
];

/// [`kiss`] and a block far from both: the touch is a fold step's, its
/// record the union's in the orders that fold it last.
pub(crate) fn kiss_and_far(doc: ProfileDoc) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let (doc, [a, b]) = kiss(doc);
    let (doc, far) = block_on(doc, a, (6.0, 7.0), (0.0, 1.0), 0.0, 1.0);
    (doc, vec![a, b, far])
}

/// [`kiss`] with its first block read twice: the pairs of `b` with
/// each read of `a` state one touch. A read repeated keys its cells
/// alike, so the statements name it `m0` both times.
fn kiss_read_twice(doc: ProfileDoc) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let (doc, [a, b]) = kiss(doc);
    (doc, vec![a, a, b])
}

/// The union of `fixture`'s members in `order`, each row as the
/// statement it makes: whether it is judged or which fold step's, its
/// site and relation, and its cells as the entity kind and the
/// fixture's index of the member they are read from, the cells sorted
/// (which cell a decision read first follows operand order, and a
/// statement is the same either way).
fn statements(fixture: Members, order: &[usize]) -> Vec<String> {
    let doc = ProfileDoc::empty_derived("union-member-orders", Tol::witness());
    let (doc, members) = fixture(doc);
    let (doc, union) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(
                order.iter().map(|&i| members[i].into()).collect(),
            ),
        },
    );
    let ev = run(&doc);
    assert!(failure(&ev, union).is_none(), "{order:?} builds");
    let cell = |c: &editor_core::NamedCell| match c {
        editor_core::NamedCell::Entity { input, name } => {
            let member = members
                .iter()
                .position(|&m| doc.output(m, 0) == Some(*input))
                .map_or("the union".to_string(), |i| format!("m{i}"));
            format!("{:?}@{member}", name.kind)
        }
        other => format!("{other:?}"),
    };
    let mut out: Vec<String> = ev
        .value(union)
        .expect("the union evaluated")
        .coincidences
        .iter()
        .map(|r| statement(r, &cell))
        .collect();
    out.sort();
    out
}

/// One row's statement, its cells spelled by `cell`: `judged` or the
/// fold step's place, then its site, relation and sorted cells.
fn statement(
    r: &editor_core::NamedCoincidence,
    cell: &dyn Fn(&editor_core::NamedCell) -> String,
) -> String {
    let mut cells: Vec<String> = r.cells.iter().map(cell).collect();
    cells.sort();
    let origin = match r.origin {
        editor_core::RowOrigin::Fold { step } => format!("fold {step}"),
        _ => "judged".to_string(),
    };
    format!("{origin} {:?} {:?} {}", r.site, r.relation, cells.join(" "))
}

/// **A judged vertex row is one row of the union in every member
/// order, once** (DM4: the judged rows are the union's order-free rows,
/// and a fold row stating what a judged row states is that row): the
/// L-prism and the cube pinching at its corner, alone, with a far third
/// member, and with a third member filling the pinch, whose pair still
/// decided it; two blocks kissing at a corner, whose touch a record
/// cites, with a far third member and with one block read twice; two
/// blocks pinching on a plate (`union_pinch_member_order`), where a fold
/// step meets a vertex an earlier step minted. Every order that builds
/// judges alike, and the first that builds logs exactly the statements
/// listed, its fold steps' with them, so a row published twice or not
/// at all is red. Every order builds, the far member folded last after
/// the pinch among them.
#[test]
fn a_vertex_row_is_the_unions_once_in_every_member_order() {
    type Row = (&'static str, Members, &'static [&'static str]);
    const PINCH: &[&str] = &["judged VertexFusion OnCarrier Face@m1 Vertex@m0"];
    // The kiss: the three walls through the corner, each against the
    // other block's, and the corner itself.
    const KISS: &[&str] = &[
        "judged PlaneLadder SameOpposite Face@m0 Face@m1",
        "judged PlaneLadder SameOpposite Face@m0 Face@m1",
        "judged PlaneLadder SameOpposite Face@m0 Face@m1",
        "judged VertexFusion OnCarrier Vertex@m0 Vertex@m1",
    ];
    // Read twice: each of the block's six faces and eight corners
    // against its twin (DM5), and the kiss once, though two pairs state
    // it.
    const TWICE: &[&str] = &[
        "judged PlaneLadder SameOpposite Face@m0 Face@m2",
        "judged PlaneLadder SameOpposite Face@m0 Face@m2",
        "judged PlaneLadder SameOpposite Face@m0 Face@m2",
        "judged PlaneLadder SameOriented Face@m0 Face@m0",
        "judged PlaneLadder SameOriented Face@m0 Face@m0",
        "judged PlaneLadder SameOriented Face@m0 Face@m0",
        "judged PlaneLadder SameOriented Face@m0 Face@m0",
        "judged PlaneLadder SameOriented Face@m0 Face@m0",
        "judged PlaneLadder SameOriented Face@m0 Face@m0",
        "judged VertexFusion OnCarrier Vertex@m0 Vertex@m0",
        "judged VertexFusion OnCarrier Vertex@m0 Vertex@m0",
        "judged VertexFusion OnCarrier Vertex@m0 Vertex@m0",
        "judged VertexFusion OnCarrier Vertex@m0 Vertex@m0",
        "judged VertexFusion OnCarrier Vertex@m0 Vertex@m0",
        "judged VertexFusion OnCarrier Vertex@m0 Vertex@m0",
        "judged VertexFusion OnCarrier Vertex@m0 Vertex@m0",
        "judged VertexFusion OnCarrier Vertex@m0 Vertex@m0",
        "judged VertexFusion OnCarrier Vertex@m0 Vertex@m2",
    ];
    // Two blocks on a plate, touching along the line through the
    // plate's corner: their two walls on that line, and each block's end
    // of the line on the other's edge. Folded last, in the first order
    // (the blocks, then the plate), the plate's step decides the points
    // its top made on the two blocks' legs one: the legs' meeting with
    // the top, which the legs alone meet along a line.
    const PLATE: &[&str] = &[
        "fold 2 VertexFusion Meet Edge@m1 Edge@m2 Face@m0",
        "judged PlaneLadder SameOpposite Face@m1 Face@m2",
        "judged PlaneLadder SameOpposite Face@m1 Face@m2",
        "judged VertexFusion OnCarrier Edge@m1 Vertex@m2",
        "judged VertexFusion OnCarrier Edge@m2 Vertex@m1",
    ];
    let rows: [Row; 6] = [
        ("prism and cube", prism_and_cube, PINCH),
        (
            "two blocks pinching on a plate",
            crate::union_pinch_member_order::pinch,
            PLATE,
        ),
        ("the pinch with a far member", pinch_and_far, PINCH),
        ("the pinch with a cap over the corner", pinch_and_cap, PINCH),
        ("the kiss with a far member", kiss_and_far, KISS),
        ("the kiss with a block read twice", kiss_read_twice, TWICE),
    ];
    let mut failures = Vec::new();
    for (what, fixture, expected) in rows {
        let n = fixture(ProfileDoc::empty_derived("count", Tol::witness()))
            .1
            .len();
        let mut first: Option<(Vec<usize>, Decided)> = None;
        for order in orders(n) {
            let got = decide(&fixture, &order);
            assert!(
                matches!(got, Decided::Built { .. }),
                "{what}, {order:?}: built: {got:?}"
            );
            match &first {
                None => first = Some((order, got)),
                Some((o, f)) => {
                    if let Err(why) = compare(f, &got) {
                        failures.push(format!("{what}, {o:?} vs {order:?}: {why}"));
                    }
                }
            }
        }
        let Some((order, _)) = &first else {
            panic!("{what}: no order builds");
        };
        let said = statements(fixture, order);
        if said != *expected {
            failures.push(format!("{what}: the statements {said:#?}"));
        }
    }
    assert!(failures.is_empty(), "{failures:#?}");
}

/// **A record the fold re-cites cites the rows its steps decided**
/// (DM4: the union's rows are its evaluation's decisions): in every
/// order of [`crate::union_pinch_member_order::pinch`], the blocks'
/// touching legs are one edge-edge record and the end below the plate's
/// top one vertex-edge record citing the judged row that puts it there.
/// Folded after the plate, a block's leg meets the point the other's
/// made on the plate's top, and the edge-edge record cites that step's
/// decision, the two legs and the plate's top meeting there, beside the
/// judged row putting the first block's end on the second's leg. Folded
/// before it, the step's decisions are the judged rows, and it cites
/// the two that put an end of each leg on the other.
#[test]
fn a_re_cited_record_cites_the_rows_its_steps_decided_in_every_member_order() {
    let fixture = crate::union_pinch_member_order::pinch;
    let mut failures = Vec::new();
    for order in orders(3) {
        let doc = ProfileDoc::empty_derived("union-member-orders", Tol::witness());
        let (doc, members) = fixture(doc);
        let (doc, union) = insert(
            doc,
            Node::Union {
                members: editor_core::Bodies::Spelled(
                    order.iter().map(|&i| members[i].into()).collect(),
                ),
            },
        );
        let ev = run(&doc);
        assert!(failure(&ev, union).is_none(), "{order:?} builds");
        let value = ev.value(union).expect("the union evaluated");
        let cell = |c: &editor_core::NamedCell| match c {
            editor_core::NamedCell::Entity { input, name } => {
                let member = members
                    .iter()
                    .position(|&m| doc.output(m, 0) == Some(*input))
                    .map_or("the union".to_string(), |i| format!("m{i}"));
                format!("{:?}@{member}", name.kind)
            }
            other => format!("{other:?}"),
        };
        let statement = |k: u32| statement(&value.coincidences[k as usize], &cell);
        let editor_core::ValuePayload::Boolean(editor_core::BooleanValue::Body { .. }) =
            &value.payload
        else {
            panic!("{order:?}: a union of three blocks is a boolean body");
        };
        let contacts = &value.carry.contacts;
        let mut cited: Vec<(String, Vec<String>)> = contacts
            .rows()
            .iter()
            .map(|c| {
                let kind = format!("{:?}", c.record.kind());
                let kind = kind.split('(').next().unwrap_or_default().to_string();
                let mut rows: Vec<String> = c
                    .cites
                    .iter()
                    .map(|b| match b {
                        topo::Backing::Decided(k) => statement(k),
                        other => format!("{other:?}"),
                    })
                    .collect();
                rows.sort();
                (kind, rows)
            })
            .collect();
        cited.sort();
        let legs = if order[2] == 0 {
            "judged VertexFusion OnCarrier Edge@m2 Vertex@m1"
        } else {
            "fold 2 VertexFusion Meet Edge@m1 Edge@m2 Face@m0"
        };
        let want = vec![
            (
                "EdgeEdge".to_string(),
                vec![
                    legs.to_string(),
                    "judged VertexFusion OnCarrier Edge@m1 Vertex@m2".to_string(),
                ],
            ),
            (
                "VertexEdge".to_string(),
                vec!["judged VertexFusion OnCarrier Edge@m1 Vertex@m2".to_string()],
            ),
        ];
        let mut want = want;
        for (_, rows) in &mut want {
            rows.sort();
        }
        if cited != want {
            failures.push(format!("{order:?}: the records cite {cited:#?}"));
        }
    }
    assert!(failures.is_empty(), "{failures:#?}");
}

/// Two blocks flush along x and a third touching both along one edge:
/// the flush rims are one `Merged` edge the fold mints, and the third
/// block's touch on it is a row citing that edge
/// (`union_flush_onto_edge_contact`).
pub(crate) fn flush_onto_an_edge(doc: ProfileDoc) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, b) = block_on(doc, a, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, c) = block_on(doc, a, (0.5, 1.5), (-1.0, 0.0), -1.0, 1.0);
    (doc, vec![a, b, c])
}

/// **Every published row's cells are member-table rows, in every
/// member order** ([`cells_are_held`]), and share a point where the
/// geometry lets that be read. Three fixtures:
/// - the plate pinch, whose rows name member cells two at a time;
/// - a flush pair on an edge, whose fold rows put the third block on
///   the edge the fold merged from the pair's rims, spelled as those
///   rims;
/// - a prism crossing one wedge, whose fold rows meet at the plate's
///   crossing of two wedges' seam, spelled as the faces and edges that
///   meet there.
///
/// Every order must publish rows, and each fixture reach rows of the
/// kind it is here for, so no fixture passes on nothing.
#[test]
fn published_row_cells_are_member_cells_that_meet() {
    type Row = (&'static str, Members, fn(Held) -> bool);
    let rows: [Row; 3] = [
        (
            "two blocks pinching on a plate",
            crate::union_pinch_member_order::pinch,
            |h| h.cells > 0 && h.sited > 0,
        ),
        ("a flush pair on an edge", flush_onto_an_edge, |h| {
            h.sited > 0
        }),
        (
            "a prism crossing one wedge about the line, with the plate",
            crate::union_pinch_member_order::crossing_one_wedge,
            |h| h.meetings > 0 && h.sited > 0,
        ),
    ];
    for (what, fixture, reaches) in rows {
        let n = fixture(ProfileDoc::empty_derived("count", Tol::witness()))
            .1
            .len();
        let mut total = Held::default();
        for order in orders(n) {
            let doc = ProfileDoc::empty_derived("union-member-orders", Tol::witness());
            let (doc, members) = fixture(doc);
            let (doc, union) = insert(
                doc,
                Node::Union {
                    members: editor_core::Bodies::Spelled(
                        order.iter().map(|&i| members[i].into()).collect(),
                    ),
                },
            );
            let ev = run(&doc);
            assert!(failure(&ev, union).is_none(), "{what}, {order:?}: builds");
            let held = cells_are_held(&ev, union, &order);
            assert!(held.rows > 0, "{what}, {order:?}: publishes rows");
            total.rows += held.rows;
            total.cells += held.cells;
            total.meetings += held.meetings;
            total.sited += held.sited;
        }
        assert!(reaches(total), "{what}: checked {total:?}");
    }
}

/// The union of `fixture`'s members in `order`, in a document that
/// first holds `padding` unrelated blocks (so every id the fixture mints
/// is another): its rows in decision order, each as [`statement`] spells
/// it with the fold step's place, and its records' citations.
fn log(fixture: Members, order: &[usize], padding: usize) -> Vec<String> {
    let mut doc = ProfileDoc::empty_derived("union-member-orders", Tol::witness());
    for k in 0..padding {
        let at = 10.0 + 2.0 * k as f64;
        doc = block(doc, (at, at + 1.0), (0.0, 1.0), 1.0).0;
    }
    let (doc, members) = fixture(doc);
    let (doc, union) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(
                order.iter().map(|&i| members[i].into()).collect(),
            ),
        },
    );
    let ev = run(&doc);
    assert!(failure(&ev, union).is_none(), "{order:?} builds");
    let value = ev.value(union).expect("the union evaluated");
    let cell = |c: &editor_core::NamedCell| match c {
        editor_core::NamedCell::Entity { input, name } => {
            let member = members
                .iter()
                .position(|&m| doc.output(m, 0) == Some(*input))
                .map_or("the union".to_string(), |i| format!("m{i}"));
            format!("{:?}@{member}", name.kind)
        }
        other => format!("{other:?}"),
    };
    let editor_core::ValuePayload::Boolean(editor_core::BooleanValue::Body { .. }) = &value.payload
    else {
        panic!("{order:?}: a union of bodies is a boolean body");
    };
    let contacts = &value.carry.contacts;
    value
        .coincidences
        .iter()
        .map(|r| statement(r, &cell))
        .chain(contacts.rows().iter().map(|c| format!("{:?}", c.cites)))
        .collect()
}

/// **A union's log is a function of its member order alone** (DM4: the
/// rows are its evaluation's decisions, and which steps meet a
/// coincidence follows the order): each order of each fixture whose fold
/// steps log rows, built again after unrelated blocks that move every
/// id the fixture mints, logs the same rows in the same decision order
/// and the same citations. Red if the log follows an id the document
/// mints, a read's or a node's, rather than the order the author wrote.
#[test]
fn the_log_is_a_function_of_the_member_order_alone() {
    let fixtures: [(&str, Members); 3] = [
        ("the plate pinch", crate::union_pinch_member_order::pinch),
        ("a flush pair on an edge", flush_onto_an_edge),
        (
            "a prism crossing one wedge",
            crate::union_pinch_member_order::crossing_one_wedge,
        ),
    ];
    for (what, fixture) in fixtures {
        let n = fixture(ProfileDoc::empty_derived("count", Tol::witness()))
            .1
            .len();
        for order in orders(n).into_iter().take(4) {
            assert_eq!(
                log(fixture, &order, 0),
                log(fixture, &order, 3),
                "{what}, {order:?}: one order logs alike whatever ids the document minted"
            );
        }
    }
}

/// **The member cells a fold-step cell is made of**, read here off its
/// collapsed name by this suite's own reading of the name forms
/// (`RoleSeg`'s docs): a member's entity under its `From` head; a
/// `Crossing` its edge and face; a `Seam` its two faces; a junction's
/// run of `Seam` lines, or an `EdgeCrossing` run, each line; a `Merged`
/// set each constituent. Pieces' `Fragment` qualifiers name no cell.
fn made_of(name: &editor_core::StableName, out: &mut Vec<editor_core::NamedCell>) {
    use editor_core::RoleSeg;
    let fragments = |tail: &[RoleSeg]| tail.iter().all(|s| matches!(s, RoleSeg::Fragment(_)));
    if name.path.len() >= 2 && name.path.iter().all(|s| matches!(s, RoleSeg::Seam { .. })) {
        for seg in &name.path {
            if let RoleSeg::Seam { a, b } = seg {
                made_of(a.name(), out);
                made_of(b.name(), out);
            }
        }
        return;
    }
    match name.path.split_first() {
        Some((RoleSeg::From { read, of }, tail)) if fragments(tail) => {
            out.push(editor_core::NamedCell::Entity {
                input: *read,
                name: of.name().clone(),
            });
        }
        Some((RoleSeg::Crossing { edge, face, .. }, tail)) if fragments(tail) => {
            made_of(edge.name(), out);
            made_of(face.name(), out);
        }
        Some((RoleSeg::Seam { a, b }, tail)) if fragments(tail) => {
            made_of(a.name(), out);
            made_of(b.name(), out);
        }
        Some((RoleSeg::EdgeCrossing(run), tail)) if fragments(tail) => {
            for line in run {
                made_of(line.line.name(), out);
            }
        }
        Some((RoleSeg::Merged(set), tail)) if fragments(tail) => {
            for n in set {
                made_of(n, out);
            }
        }
        _ => panic!("a fold cell's name of no form a fold step mints: {name:?}"),
    }
}

/// **Every fold-step row names every member cell its cells are made
/// of, and no other** (DM4, "A row names member cells"): each row a
/// union's fold steps spelled, held against its two cells' collapsed
/// names as the step read them ([`editor_core::test_support::fold_rows_of`])
/// resolved by [`made_of`]. A point decided on a cell is `OnCarrier`
/// between one member vertex and one member cell, in the step's order,
/// and otherwise the `Meet` of every member cell both are made of; any
/// other relation is its two member cells as read. Over every order of
/// the fixtures whose fold steps mint crossings, seams, junctions and
/// merged edges, and the corner boss's, whose step decides a point in
/// three cells; and each must log rows of three cells or more, so the
/// check is not passed on two-cell rows alone.
#[test]
fn every_fold_row_names_every_member_cell_its_cells_are_made_of() {
    use editor_core::NamedCell;
    let fixtures: [(&str, Members); 4] = [
        ("the plate pinch", crate::union_pinch_member_order::pinch),
        ("a flush pair on an edge", flush_onto_an_edge),
        (
            "a prism crossing one wedge",
            crate::union_pinch_member_order::crossing_one_wedge,
        ),
        ("the corner boss", crate::emit_union_borders::corner_boss),
    ];
    let mut meetings = 0;
    for (what, fixture) in fixtures {
        let n = fixture(ProfileDoc::empty_derived("count", Tol::witness()))
            .1
            .len();
        for order in orders(n) {
            let doc = ProfileDoc::empty_derived("union-member-orders", Tol::witness());
            let (doc, members) = fixture(doc);
            let (doc, union) = insert(
                doc,
                Node::Union {
                    members: editor_core::Bodies::Spelled(
                        order.iter().map(|&i| members[i].into()).collect(),
                    ),
                },
            );
            let (ev, logged) = editor_core::test_support::fold_rows_of(|| run(&doc));
            assert!(failure(&ev, union).is_none(), "{what}, {order:?}: builds");
            for r in &logged {
                let resolve = |name| {
                    let mut cells = Vec::new();
                    made_of(name, &mut cells);
                    cells.sort();
                    cells.dedup();
                    cells
                };
                assert!(
                    r.cells.len() == 2 || r.relation == topo::Relation::Meet,
                    "{what}, {order:?}: only a meeting holds a third cell: {r:?}"
                );
                let resolved: Vec<_> = r.cells.iter().map(resolve).collect();
                let (p, x) = (&resolved[0], &resolved[1]);
                let vertex = |c: &NamedCell| {
                    matches!(c, NamedCell::Entity { name, .. }
                        if name.kind == editor_core::EntityKind::Vertex)
                };
                let point = r.cells[0].kind == editor_core::EntityKind::Vertex;
                let (cells, relation) = match (r.relation, p.as_slice(), x.as_slice()) {
                    (topo::Relation::OnCarrier, [v], [c]) if point && vertex(v) => {
                        (vec![v.clone(), c.clone()], topo::Relation::OnCarrier)
                    }
                    (topo::Relation::OnCarrier, _, _) if point => {
                        let mut all: Vec<NamedCell> = p.iter().chain(x).cloned().collect();
                        all.sort();
                        all.dedup();
                        (all, topo::Relation::Meet)
                    }
                    (topo::Relation::Meet, _, _) => {
                        let mut all: Vec<NamedCell> = resolved.concat();
                        all.sort();
                        all.dedup();
                        (all, topo::Relation::Meet)
                    }
                    (relation, [a], [b]) => (vec![a.clone(), b.clone()], relation),
                    other => panic!("{what}, {order:?}: a fold row of no form: {other:?}"),
                };
                meetings += usize::from(cells.len() > 2);
                assert_eq!(
                    (&r.spelled.cells, r.spelled.relation),
                    (&cells, relation),
                    "{what}, {order:?}: the row the step decided between {:?}",
                    r.cells
                );
            }
        }
    }
    assert!(
        meetings > 0,
        "the fixtures log meetings of three cells or more"
    );
}

/// **A pierce held in a face is one statement in every member order**
/// ([`crate::union_pinch_member_order::edge_in_a_wall`]). Folded with the
/// wall and the wedge first, the plate's step mints a vertex inside the
/// wall's face where the wedge's edge pierces the plate's top, and
/// decides it one with the split that pierce made on the edge: a ring
/// tie, whose row names the pierced face beside the two cells it reads.
/// Folded any other way, a step meets the point on cells the members
/// already hold. Every order states that point as the meeting of the
/// wedge's edge, the wall's face and the plate's top, and the wedge's
/// two ends on the wall's face as judged.
#[test]
fn a_pierce_held_in_a_face_is_one_statement_in_every_member_order() {
    const HELD: &[&str] = &[
        "fold 2 VertexFusion Meet Edge@m2 Face@m0 Face@m1",
        "judged VertexFusion OnCarrier Face@m1 Vertex@m2",
        "judged VertexFusion OnCarrier Face@m1 Vertex@m2",
    ];
    let fixture: Members = crate::union_pinch_member_order::edge_in_a_wall;
    let failures: Vec<String> = orders(3)
        .into_iter()
        .map(|order| (statements(fixture, &order), order))
        .filter(|(said, _)| said != HELD)
        .map(|(said, order)| format!("{order:?}: the statements {said:#?}"))
        .collect();
    assert!(failures.is_empty(), "{failures:#?}");
}
