//! **A union decides alike in every member order** (REFERENCES DM4,
//! FORK-S4U as Ev approved it on PR 4323: "would be good to have either
//! good tests or possibly debug asserts to detect behavior that we
//! think should remain order independent").
//!
//! The pairwise pass judges every member pair in member space, and the
//! fold reads one verdict per carrier pair through member lineage and
//! re-glues none (`eval::wire`'s `debug_assert_given` holds that at the
//! verdict site in every debug run). So each union below is
//! evaluated under every order of its members, a seeded sample of
//! [`SAMPLE`] orders where `n!` is larger, and these are equal across
//! the orders:
//!
//! - **the outcome's kind**: every order builds, or every order refuses;
//! - **a refusal**: the pass's own, the same in every order — or a
//!   fold step's ([`NodeErrorKind::UnionFoldStep`]), whose inner
//!   refusal is of one kind in every order (the member it names is the
//!   one each order folds in, and differs);
//! - **the rows**: the union's coincidences, each an unordered pair of
//!   the member cells it names, as a multiset;
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
//! transverse union and an in-band sliver. Unions known to decide by
//! member order are pinned where their issues own them, not here:
//! `emit_shared_rim_several`'s `RESIDUES` (`row`, `rowids` and `cross`
//! reach an emitter residue in some orders and build in the others;
//! `work/wire/a-legal-declared-union-reaches-the-seam-vertex-parentage-residue-emission.md`,
//! `work/emit/a-held-edge-wholly-inside-a-dropped-face-is-recorded-nowhere.md`),
//! and the capsule's rod ending on its joint
//! (`work/tang/the-capsule-rod-ending-on-the-joint-parts-by-member-order.md`).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeMap;

use crate::corpus::body_of;
use crate::docm7_union_declare::{block, failure, run};
use crate::emit_boolean_vertex_keys::named_geometry;
use crate::fixture::insert;
use editor_core::{Evaluation, Node, NodeErrorKind, ProfileDoc, RecipeNodeId};
use geom_core::Tol;

/// The orders sampled where a union's members have more than this
/// many orders.
const SAMPLE: usize = 24;

type Members = fn(ProfileDoc) -> (ProfileDoc, Vec<RecipeNodeId>);

/// Three unit blocks in a row along x, each flush on the next: two
/// rests and the continuations of their shared walls.
fn three_in_a_row(doc: ProfileDoc) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (1.0, 2.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, c) = block(doc, (2.0, 3.0), (0.0, 1.0), 0.0, 1.0);
    (doc, vec![a, b, c])
}

/// Three plates stacked, each smaller than the one below: two rests,
/// no continuation.
fn stepped_stack(doc: ProfileDoc) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let (doc, a) = block(doc, (0.0, 3.0), (0.0, 3.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 2.5), (0.5, 2.5), 1.0, 1.0);
    let (doc, c) = block(doc, (1.0, 2.0), (1.0, 2.0), 2.0, 1.0);
    (doc, vec![a, b, c])
}

/// A table: a top resting on four legs at its corners, each leg flush
/// with two of the top's walls — 120 orders, sampled.
fn table(doc: ProfileDoc) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let (doc, top) = block(doc, (0.0, 4.0), (0.0, 3.0), 2.0, 0.25);
    let (doc, l0) = block(doc, (0.0, 0.5), (0.0, 0.5), 0.0, 2.0);
    let (doc, l1) = block(doc, (3.5, 4.0), (0.0, 0.5), 0.0, 2.0);
    let (doc, l2) = block(doc, (0.0, 0.5), (2.5, 3.0), 0.0, 2.0);
    let (doc, l3) = block(doc, (3.5, 4.0), (2.5, 3.0), 0.0, 2.0);
    (doc, vec![top, l0, l1, l2, l3])
}

/// A slab and two posts through it and a bar across both posts: every
/// meeting transverse, no coincidence.
fn posts_and_bar(doc: ProfileDoc) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let (doc, slab) = block(doc, (0.0, 4.0), (0.0, 2.0), 0.0, 1.0);
    let (doc, p) = block(doc, (0.5, 1.0), (0.75, 1.25), -0.5, 2.5);
    let (doc, q) = block(doc, (3.0, 3.5), (0.75, 1.25), -0.5, 2.5);
    let (doc, bar) = block(doc, (0.25, 3.75), (0.875, 1.125), 1.5, 0.25);
    (doc, vec![slab, p, q, bar])
}

/// Two blocks flush and a third standing a sliver off the second's far
/// wall: the sliver refuses, in band, whichever member meets it first.
fn a_sliver_beside_a_rest(doc: ProfileDoc) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let eps = Tol::witness().get().eps;
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (1.0, 2.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, c) = block(doc, (2.0 + 2.0 * eps, 3.0), (0.25, 0.75), 0.25, 0.5);
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
fn orders(n: usize) -> Vec<Vec<usize>> {
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
    /// A fold step refused: its inner refusal's kind, without the
    /// member it names.
    FoldStep(String),
}

/// The union of `members` in `order`, evaluated, read as [`Decided`].
fn decide(fixture: Members, order: &[usize]) -> Decided {
    let doc = ProfileDoc::empty_derived("union-member-orders", Tol::witness());
    let (doc, members) = fixture(doc);
    let (doc, union) = insert(
        doc,
        Node::Union {
            members: order.iter().map(|&i| members[i].into()).collect(),
            declare: Vec::new(),
        },
    );
    let ev = run(&doc);
    match failure(&ev, union) {
        Some(NodeErrorKind::UnionFoldStep { refusal, .. }) => {
            let inner = format!("{refusal:?}");
            let kind = inner
                .split(|c: char| !c.is_alphanumeric() && c != '_')
                .next()
                .unwrap_or_default()
                .to_string();
            Decided::FoldStep(kind)
        }
        Some(other) => Decided::Refused(format!("{other:?}")),
        None => built(&ev, union),
    }
}

fn built(ev: &Evaluation<f64>, union: RecipeNodeId) -> Decided {
    let value = ev.value(union).expect("the union evaluated");
    let mut rows: Vec<String> = value.coincidences.iter().map(row).collect();
    rows.sort();
    // Each order is its own union node, so its id is spelled as one
    // placeholder wherever a name cites it.
    let own = format!("{union:?}");
    let mut names = BTreeMap::new();
    for line in named_geometry(ev, union, false).expect("a union is never empty") {
        let line = line.replace(&own, "the union");
        let (n, g) = line.rsplit_once(" @ ").expect("a name @ geometry row");
        assert!(
            names.insert(n.to_string(), g.to_string()).is_none(),
            "{n} binds two entities"
        );
    }
    let body = body_of(ev, union);
    let topology = (
        body.faces().count(),
        body.edges().count(),
        body.vertices().count(),
    );
    let volume = topo::mass_properties(body, Tol::witness())
        .expect("a built union measures")
        .volume;
    Decided::Built {
        rows,
        names,
        topology,
        volume,
    }
}

/// A coincidence row as an unordered pair of cells beside what was
/// decided between them: which member is folded first decides which
/// cell the pass reads first, and nothing else about the row.
fn row(r: &editor_core::NamedCoincidence) -> String {
    let mut cells = r.cells.clone().map(|c| format!("{c:?}"));
    cells.sort();
    format!(
        "{cells:?} {:?} {:?} {:?} {:?}",
        r.relation, r.site, r.margin, r.discharge
    )
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
        let first = decide(fixture, &orders[0]);
        for order in &orders[1..] {
            if let Err(why) = compare(&first, &decide(fixture, order)) {
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
        let got = decide(fixture, &(0..n).collect::<Vec<_>>());
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
