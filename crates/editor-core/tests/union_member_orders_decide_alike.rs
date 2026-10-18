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
//! transverse union and an in-band sliver; vertex rows are pinned
//! beside it on an L-prism and a tilted cube that pinch, and on blocks
//! that kiss at a corner. Unions known to decide by
//! member order are pinned where their issues own them, not here: the
//! capsule's rod ending on its joint
//! (`work/tang/the-capsule-rod-ending-on-the-joint-parts-by-member-order.md`).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeMap;

use crate::blocks::{block, block_on, failure, run};
use crate::corpus::body_of;
use crate::emit_boolean_vertex_keys::named_geometry;
use crate::fixture::{against, desc, frame_at, insert, len};
use editor_core::{Evaluation, Node, NodeErrorKind, ProfileDoc, RecipeNodeId};
use geom_core::Tol;

/// The orders sampled where a union's members have more than this
/// many orders.
const SAMPLE: usize = 24;

type Members = fn(ProfileDoc) -> (ProfileDoc, Vec<RecipeNodeId>);

/// Three unit blocks in a row along x, each flush on the next: two
/// rests and the continuations of their shared walls.
fn three_in_a_row(doc: ProfileDoc) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, b) = block_on(doc, a, (1.0, 2.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, c) = block_on(doc, a, (2.0, 3.0), (0.0, 1.0), 0.0, 1.0);
    (doc, vec![a, b, c])
}

/// Three plates stacked, each smaller than the one below: two rests,
/// no continuation.
fn stepped_stack(doc: ProfileDoc) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let (doc, a) = block(doc, (0.0, 3.0), (0.0, 3.0), 1.0);
    let (doc, b) = block_on(doc, a, (0.5, 2.5), (0.5, 2.5), 1.0, 1.0);
    let (doc, c) = block_on(doc, a, (1.0, 2.0), (1.0, 2.0), 2.0, 1.0);
    (doc, vec![a, b, c])
}

/// A table: a top resting on four legs at its corners, each leg flush
/// with two of the top's walls — 120 orders, sampled.
fn table(doc: ProfileDoc) -> (ProfileDoc, Vec<RecipeNodeId>) {
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
    for line in named_geometry(ev, union).expect("a union is never empty") {
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
fn prism_and_cube(doc: ProfileDoc) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let ell = vec![
        (0.0, 0.0),
        (2.0, 0.0),
        (2.0, 1.0),
        (1.0, 1.0),
        (1.0, 2.0),
        (0.0, 2.0),
    ];
    let (doc, profile) = insert(doc, Node::Profile(desc(vec![ell])));
    let (doc, prism) = insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(1.0),
        },
    );
    // The frame through the corner whose normal is (1, 1.3, 0.7),
    // normalised: `u` its tilt of +z, `v` = normal × `u`.
    let unit = |m: [f64; 3]| {
        let n = m.iter().map(|c| c * c).sum::<f64>().sqrt();
        m.map(|c| c / n)
    };
    let m = unit([1.0, 1.3, 0.7]);
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
    let (doc, cube) = against(doc, prism, cube, &frame_at([1.0, 1.0, 1.0], u, v));
    (doc, vec![prism, cube])
}

/// [`prism_and_cube`] and a block far from both.
fn pinch_and_far(doc: ProfileDoc) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let (doc, mut m) = prism_and_cube(doc);
    let (doc, far) = block_on(doc, m[0], (6.0, 7.0), (6.0, 7.0), 0.0, 1.0);
    m.push(far);
    (doc, m)
}

/// [`prism_and_cube`] and a block over the corner, which fills the
/// pinch.
fn pinch_and_cap(doc: ProfileDoc) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let (doc, mut m) = prism_and_cube(doc);
    let (doc, cap) = block_on(doc, m[0], (0.8, 1.2), (0.8, 1.2), 0.8, 0.4);
    m.push(cap);
    (doc, m)
}

/// Where a union holds a pinch an earlier fold step made, its end pass
/// refuses naming the corner's copies.
const CARRIED_PINCH: &str = "work/refs/a-pinch-carried-through-a-later-fold-step-refuses-naming.md";

/// Two unit blocks kissing at the corner (1, 1, 1): one vertex-vertex
/// record, citing its `VertexFusion` row.
fn kiss(doc: ProfileDoc) -> (ProfileDoc, [RecipeNodeId; 2]) {
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, b) = block_on(doc, a, (1.0, 2.0), (1.0, 2.0), 1.0, 1.0);
    (doc, [a, b])
}

/// [`kiss`] and a block far from both: the touch is a fold step's, its
/// record the union's in the orders that fold it last.
fn kiss_and_far(doc: ProfileDoc) -> (ProfileDoc, Vec<RecipeNodeId>) {
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
/// statement it makes: its site and relation, and its two cells as the
/// entity kind and the fixture's index of the member they are read
/// from, the cells sorted, as [`row`] sorts them (which cell a
/// decision read first follows operand order, and a statement is the
/// same either way).
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
        .map(|r| {
            let mut cells = r.cells.each_ref().map(cell);
            cells.sort();
            format!("{:?} {:?} {} {}", r.site, r.relation, cells[0], cells[1])
        })
        .collect();
    out.sort();
    out
}

/// **A vertex row is one row of the union in every member order, once**
/// (DM4: the union's rows are its member pairs', and a fold row a judged
/// row states cites it): the L-prism and the cube pinching at its
/// corner, alone, with a far third member, and with a third member
/// filling the pinch, whose pair still decided it; two blocks kissing at
/// a corner, whose touch a record cites, with a far third member and
/// with one block read twice. Every order that builds decides alike,
/// and the first that builds publishes exactly the statements listed, so a row
/// published twice or not at all is red. The orders that fold the far
/// member last after the pinch refuse [`CARRIED_PINCH`] and are
/// excluded, each checked to refuse that way.
#[test]
fn a_vertex_row_is_the_unions_once_in_every_member_order() {
    type Row = (
        &'static str,
        Members,
        fn(&[usize]) -> bool,
        &'static [&'static str],
    );
    const PINCH: &[&str] = &["VertexFusion OnCarrier Face@m1 Vertex@m0"];
    // The kiss: the three walls through the corner, each against the
    // other block's, and the corner itself.
    const KISS: &[&str] = &[
        "PlaneLadder SameOpposite Face@m0 Face@m1",
        "PlaneLadder SameOpposite Face@m0 Face@m1",
        "PlaneLadder SameOpposite Face@m0 Face@m1",
        "VertexFusion OnCarrier Vertex@m0 Vertex@m1",
    ];
    // Read twice: each of the block's six faces against its twin (DM5),
    // and the kiss once, though two pairs state it.
    const TWICE: &[&str] = &[
        "PlaneLadder SameOpposite Face@m0 Face@m2",
        "PlaneLadder SameOpposite Face@m0 Face@m2",
        "PlaneLadder SameOpposite Face@m0 Face@m2",
        "PlaneLadder SameOriented Face@m0 Face@m0",
        "PlaneLadder SameOriented Face@m0 Face@m0",
        "PlaneLadder SameOriented Face@m0 Face@m0",
        "PlaneLadder SameOriented Face@m0 Face@m0",
        "PlaneLadder SameOriented Face@m0 Face@m0",
        "PlaneLadder SameOriented Face@m0 Face@m0",
        "VertexFusion OnCarrier Vertex@m0 Vertex@m2",
    ];
    let rows: [Row; 5] = [
        ("prism and cube", prism_and_cube, |_| false, PINCH),
        (
            "the pinch with a far member",
            pinch_and_far,
            |o| o[2] == 2,
            PINCH,
        ),
        (
            "the pinch with a cap over the corner",
            pinch_and_cap,
            |_| false,
            PINCH,
        ),
        ("the kiss with a far member", kiss_and_far, |_| false, KISS),
        (
            "the kiss with a block read twice",
            kiss_read_twice,
            |_| false,
            TWICE,
        ),
    ];
    let mut failures = Vec::new();
    for (what, fixture, carried, expected) in rows {
        let n = fixture(ProfileDoc::empty_derived("count", Tol::witness()))
            .1
            .len();
        let mut first: Option<(Vec<usize>, Decided)> = None;
        for order in orders(n) {
            let got = decide(&fixture, &order);
            if carried(&order) {
                assert!(
                    matches!(&got, Decided::Refused(e)
                        if e.contains("a copy of a corner held as several meets no recorded discard")),
                    "{what}, {order:?}: refused other than {CARRIED_PINCH}: {got:?}"
                );
                continue;
            }
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
