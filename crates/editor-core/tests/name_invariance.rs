//! **A name reads nothing the author did not write** (NAMES N1, N4;
//! DM4): each test here evaluates a recipe, perturbs something its
//! names must not read, evaluates again, and compares the two tables.
//!
//! A table is read as its names, each beside its candidates (one, or a
//! tie's several, compared as a set): an entity's kind, its geometry
//! (a vertex's point, an edge's end points, a face's boundary vertex
//! points, rounded to the micrometre) and the names of the entities it
//! is incident with. Minted ids are spelled through a [`Ledger`], so
//! two documents compare up to the bijection of what each authored. A
//! refusal is compared by its kind; an empty result as empty.
//!
//! The perturbations, each its own test over the contact-heavy corpus
//! below (pinches, flush faces, tilted wedges, prisms on a contact
//! line, notched and pentagon splits, merged chords):
//! 1. the member order of an n-ary union and intersect;
//! 2. the operand order of a two-member union and intersect;
//! 3. a rigid frame change of a split plane: its normal's sign, the
//!    halves swapped, and its origin moved along it;
//! 4. a small tilt within one topology, compared by incidence;
//! 5. two separately authored, structurally identical operands,
//!    authored in either order;
//! 6. a far feature added: every other name keeps its incidences and
//!    its vertices' points;
//! 7. one recipe under another authoring history.
//!
//! An operand's arena storage permuted is not among them: `topo` has no
//! helper that permutes a body
//! (`work/wire/topo-has-no-arena-permuting-helper-for-a-storage-order-name-check.md`).
//!
//! Each test holds an admission list: the cases a perturbation finds
//! moving today, each citing the row that owns it. A case outside the
//! list that moves, or one in it that no longer does, is red.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::{BTreeMap, BTreeSet};

use crate::blocks::{block, failure, run};
use crate::fixture::{ang, ends, face_edges, face_vertices, insert, len, on_frame, point, scl};
use crate::union_member_orders_decide_alike as decide_alike;
use crate::union_pinch_member_order as pinch_order;
use editor_core::{
    Bodies, BooleanValue, Datum, DocEdit, EntityKey, Entry, ExtrudeSide, Formula, LoopProgram,
    Node, ProfileDoc, RecipeNodeId, ValuePayload,
};
use geom_core::Tol;

// ------------------------------------------------------------------ //
// The harness
// ------------------------------------------------------------------ //

/// **What each authored span of a document is called**: the mint
/// ordinals `[start, end)` an authoring step drew, and its label. An id
/// is spelled by the span it falls in and its offset there, so two
/// documents that author the same things in another order, or after
/// something else, spell them alike.
///
/// Where a perturbation changes the order ids were minted in, a ledger
/// made [`Ledger::up_to_id_order`] also spells each name's sets in one
/// order ([`sets_sorted`]).
#[derive(Default, Clone)]
pub(crate) struct Ledger {
    spans: Vec<(u32, u32, String)>,
    sets: bool,
}

impl Ledger {
    /// `f` run on `doc`, its mints recorded as `label`.
    pub(crate) fn authored<T>(
        &mut self,
        doc: ProfileDoc,
        label: &str,
        f: impl FnOnce(ProfileDoc) -> (ProfileDoc, T),
    ) -> (ProfileDoc, T) {
        let start = next_ordinal(&doc);
        let (doc, out) = f(doc);
        self.spans
            .push((start, next_ordinal(&doc), label.to_string()));
        (doc, out)
    }

    fn spell(&self, ordinal: u32) -> String {
        self.spans
            .iter()
            .find(|(s, e, _)| (*s..*e).contains(&ordinal))
            .map_or(format!("#{ordinal}"), |(s, _, l)| {
                format!("{l}+{}", ordinal - s)
            })
    }

    /// A ledger that also spells every name's sets in one order.
    pub(crate) fn up_to_id_order() -> Self {
        Self {
            sets: true,
            ..Self::default()
        }
    }

    /// `debug` with every minted id in it spelled by its span.
    fn respell(&self, debug: &str) -> String {
        const HEAD: &str = "MintId { ordinal: ";
        let mut out = String::new();
        let mut rest = debug;
        while let Some(at) = rest.find(HEAD) {
            out.push_str(&rest[..at]);
            let tail = &rest[at + HEAD.len()..];
            let digits = tail
                .find(|c: char| !c.is_ascii_digit())
                .unwrap_or(tail.len());
            let ordinal: u32 = tail[..digits].parse().expect("an ordinal");
            let close = tail.find('}').expect("a closed MintId");
            out.push_str(&self.spell(ordinal));
            rest = &tail[close + 1..];
        }
        out.push_str(rest);
        if self.sets { sets_sorted(&out) } else { out }
    }
}

/// The index just past the bracket that closes the one at `s[0]`.
fn closing(s: &str) -> usize {
    let mut depth = 0usize;
    for (i, c) in s.char_indices() {
        match c {
            '(' | '[' | '{' => depth += 1,
            ')' | ']' | '}' => {
                depth -= 1;
                if depth == 0 {
                    return i + 1;
                }
            }
            _ => {}
        }
    }
    panic!("an unclosed bracket in {s}")
}

/// `s` split at its top-level `", "`s.
fn top_level(s: &str) -> Vec<&str> {
    let (mut out, mut depth, mut from) = (Vec::new(), 0usize, 0);
    let bytes = s.as_bytes();
    for (i, &b) in bytes.iter().enumerate() {
        match b {
            b'(' | b'[' | b'{' => depth += 1,
            b')' | b']' | b'}' => depth -= 1,
            b',' if depth == 0 && bytes.get(i + 1) == Some(&b' ') => {
                out.push(&s[from..i]);
                from = i + 2;
            }
            _ => {}
        }
    }
    if from < s.len() {
        out.push(&s[from..]);
    }
    out
}

/// **A name's sets spelled in one order**: a union seam's two sides and
/// the lists a qualifier holds as a set, each sorted by its own
/// spelling. A name orders a set by its members' names, whose ids order
/// as they were minted, so two histories spell one set in two orders.
fn sets_sorted(s: &str) -> String {
    const SETS: [&str; 5] = ["Ends(", "Borders(", "Keeps(", "Merged(", "EdgeCrossing("];
    const SEAM: &str = "Seam { ";
    let mut out = String::new();
    let mut i = 0;
    while i < s.len() {
        let rest = &s[i..];
        if let Some(set) = SETS.iter().find(|p| rest.starts_with(**p)) {
            let open = i + set.len();
            if s[open..].starts_with('[') {
                let end = open + closing(&s[open..]);
                let mut items: Vec<String> = top_level(&s[open + 1..end - 1])
                    .into_iter()
                    .map(sets_sorted)
                    .collect();
                items.sort();
                out.push_str(set);
                out.push('[');
                out.push_str(&items.join(", "));
                out.push(']');
                i = end;
                continue;
            }
        }
        if rest.starts_with(SEAM) {
            let open = i + SEAM.len() - 2;
            let end = open + closing(&s[open..]);
            let fields = top_level(&s[open + 2..end - 2]);
            if let [a, b] = fields.as_slice()
                && let (Some(a), Some(b)) = (a.strip_prefix("a: "), b.strip_prefix("b: "))
            {
                let mut sides = [sets_sorted(a), sets_sorted(b)];
                sides.sort();
                out.push_str(&format!("Seam {{ a: {}, b: {} }}", sides[0], sides[1]));
                i = end;
                continue;
            }
        }
        let c = rest.chars().next().expect("a char");
        out.push(c);
        i += c.len_utf8();
    }
    out
}

/// The ordinal `doc`'s next mint draws.
fn next_ordinal(doc: &ProfileDoc) -> u32 {
    let mint = doc.mint();
    let nodes = doc.ids().into_iter().map(|n| n.0.ordinal());
    let steps = mint.steps().map(|s| s.0.ordinal());
    let vars = mint.vars().map(|v| v.0.ordinal());
    nodes.chain(steps).chain(vars).max().unwrap_or(0) + 1
}

/// One entity a name denotes, as the comparisons read it.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Cand {
    kind: char,
    geo: Vec<(i64, i64, i64)>,
    near: BTreeSet<String>,
}

/// What one output of one node published.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Published {
    /// Its names, each beside its candidates, sorted.
    Table(BTreeMap<String, Vec<Cand>>),
    /// The empty result.
    Empty,
    /// A refusal, by its kind.
    Refused(String),
}

fn micro(x: f64) -> i64 {
    (x * 1e6).round() as i64
}

/// A refusal's kind: its debug spelling up to its first field, and the
/// sentence an emission refusal states.
fn kind_of(debug: &str) -> String {
    let head = debug.split(['{', ',']).next().unwrap_or_default().trim();
    match debug.split_once("what: \"") {
        Some((_, what)) => format!("{head} {:?}", what.split('"').next().unwrap_or_default()),
        None => head.to_string(),
    }
}

/// **What `node`'s output `port` published in `doc`**, its ids spelled
/// by `ledger`.
pub(crate) fn published(
    doc: &ProfileDoc,
    node: RecipeNodeId,
    port: u8,
    ledger: &Ledger,
) -> Published {
    published_in(&run(doc), node, port, ledger)
}

/// [`published`], read off the evaluation `ev`.
pub(crate) fn published_in(
    ev: &editor_core::Evaluation<f64>,
    node: RecipeNodeId,
    port: u8,
    ledger: &Ledger,
) -> Published {
    if let Some(e) = failure(ev, node) {
        return Published::Refused(kind_of(&ledger.respell(&format!("{e:?}"))));
    }
    let value = ev.value(node).expect("the node evaluated");
    if matches!(value.payload, ValuePayload::Boolean(BooleanValue::Empty)) {
        return Published::Empty;
    }
    let var = ev
        .output_var(node, port)
        .expect("the node defines the port");
    let table = ev.table(var).expect("the port's table");
    let mut names: BTreeMap<(u32, EntityKey), Vec<String>> = BTreeMap::new();
    let mut entries = Vec::new();
    for (n, e) in table.iter() {
        let spelled = ledger.respell(&format!("{n:?}"));
        let rows: Vec<_> = match e {
            Entry::Unique(r) => vec![*r],
            Entry::Tied(rs) => rs.clone(),
        };
        for r in &rows {
            names
                .entry((r.body, r.key))
                .or_default()
                .push(spelled.clone());
        }
        entries.push((spelled, rows));
    }
    let mut out = BTreeMap::new();
    for (spelled, rows) in entries {
        let mut cands = Vec::new();
        for r in rows {
            let body = ev
                .read_body(var, r.body)
                .expect("the read evaluated")
                .expect("the read holds the body");
            let at = |v| {
                let p = point(body, v);
                (micro(p.x), micro(p.y), micro(p.z))
            };
            let named = |k: EntityKey| -> Vec<String> {
                names.get(&(r.body, k)).cloned().unwrap_or_default()
            };
            let (kind, mut geo, adjacent): (char, Vec<_>, Vec<EntityKey>) = match r.key {
                EntityKey::Vertex(v) => {
                    let mut adj = Vec::new();
                    for (e, _) in body.edges() {
                        if ends(body, e).contains(&v) {
                            adj.push(EntityKey::Edge(e));
                        }
                    }
                    for (f, _) in body.faces() {
                        if face_vertices(body, f).contains(&v) {
                            adj.push(EntityKey::Face(f));
                        }
                    }
                    ('v', vec![at(v)], adj)
                }
                EntityKey::Edge(e) => {
                    let [a, b] = ends(body, e);
                    let mut adj = vec![EntityKey::Vertex(a), EntityKey::Vertex(b)];
                    for (f, _) in body.faces() {
                        if face_edges(body, f).contains(&e) {
                            adj.push(EntityKey::Face(f));
                        }
                    }
                    ('e', vec![at(a), at(b)], adj)
                }
                EntityKey::Face(f) => {
                    let mut adj: Vec<EntityKey> = face_edges(body, f)
                        .into_iter()
                        .map(EntityKey::Edge)
                        .collect();
                    let vs = face_vertices(body, f);
                    let geo = vs.iter().map(|&v| at(v)).collect();
                    adj.extend(vs.into_iter().map(EntityKey::Vertex));
                    ('f', geo, adj)
                }
                EntityKey::Body => ('b', Vec::new(), Vec::new()),
            };
            geo.sort_unstable();
            let near = adjacent.into_iter().flat_map(named).collect();
            cands.push(Cand { kind, geo, near });
        }
        cands.sort();
        out.insert(spelled, cands);
    }
    Published::Table(out)
}

/// The point lists `a` and `b` are one, each point within `slack`
/// micrometres of its partner.
fn same_points(a: &[(i64, i64, i64)], b: &[(i64, i64, i64)], slack: i64) -> bool {
    a.len() == b.len()
        && a.iter().zip(b).all(|(p, q)| {
            (p.0 - q.0).abs() <= slack && (p.1 - q.1).abs() <= slack && (p.2 - q.2).abs() <= slack
        })
}

/// How two tables are compared.
#[derive(Clone, Copy)]
pub(crate) enum Compare {
    /// The same names, each over the same candidates' geometry.
    Geometry,
    /// The same names, each over candidates of the same kind incident
    /// with the same names: geometry may move.
    Incidence,
    /// Every name of the first is in the second, over candidates of the
    /// same kind whose incidences hold the first's, any others being
    /// names the first lacks, and whose vertices stand where the
    /// first's did: what a far feature leaves alone.
    Kept,
}

/// What differs between `a` and `b` under `how`, one line per name.
pub(crate) fn differences(a: &Published, b: &Published, how: Compare) -> Vec<String> {
    let (Published::Table(ta), Published::Table(tb)) = (a, b) else {
        return if a == b {
            Vec::new()
        } else {
            vec![format!("{} vs {}", outline(a), outline(b))]
        };
    };
    let mut out = Vec::new();
    for (n, ca) in ta {
        let Some(cb) = tb.get(n) else {
            out.push(format!("{n}: absent from the second"));
            continue;
        };
        let alike = match how {
            Compare::Geometry => {
                let mut ga: Vec<_> = ca.iter().map(|c| (c.kind, c.geo.clone())).collect();
                let mut gb: Vec<_> = cb.iter().map(|c| (c.kind, c.geo.clone())).collect();
                ga.sort();
                gb.sort();
                ga.len() == gb.len()
                    && ga
                        .iter()
                        .zip(&gb)
                        .all(|(x, y)| x.0 == y.0 && same_points(&x.1, &y.1, 2))
            }
            Compare::Incidence => {
                let mut ia: Vec<_> = ca.iter().map(|c| (c.kind, &c.near)).collect();
                let mut ib: Vec<_> = cb.iter().map(|c| (c.kind, &c.near)).collect();
                ia.sort();
                ib.sort();
                ia == ib
            }
            Compare::Kept => {
                ca.len() == cb.len()
                    && ca.iter().all(|x| {
                        cb.iter().any(|y| {
                            x.kind == y.kind
                                && x.near.is_subset(&y.near)
                                && y.near.difference(&x.near).all(|m| !ta.contains_key(m))
                                && (x.kind != 'v' || same_points(&x.geo, &y.geo, 2))
                        })
                    })
            }
        };
        if !alike {
            out.push(format!("{n}: {ca:?} vs {cb:?}"));
        }
    }
    if !matches!(how, Compare::Kept) {
        for n in tb.keys().filter(|n| !ta.contains_key(*n)) {
            out.push(format!("{n}: absent from the first"));
        }
    }
    out
}

fn outline(p: &Published) -> String {
    match p {
        Published::Table(t) => format!("a table of {} names", t.len()),
        Published::Empty => "empty".to_string(),
        Published::Refused(k) => format!("refused {k}"),
    }
}

/// **The admission settles**: every row an admission cites exists, and
/// the cases that moved are exactly the admitted ones.
fn settle(found: &BTreeMap<String, Vec<String>>, admitted: &[(&str, &str)]) {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    for (case, row) in admitted {
        assert!(root.join(row).is_file(), "{case}: its row {row} is no file");
    }
    let admitted: BTreeMap<&str, &str> = admitted.iter().copied().collect();
    let new: Vec<_> = found
        .iter()
        .filter(|(c, _)| !admitted.contains_key(c.as_str()))
        .map(|(c, d)| {
            format!(
                "{c}: {} differences, first {:#?}",
                d.len(),
                &d[..d.len().min(3)]
            )
        })
        .collect();
    let gone: Vec<_> = admitted
        .iter()
        .filter(|(c, _)| !found.contains_key(**c))
        .map(|(c, r)| format!("{c} (admitted on {r})"))
        .collect();
    assert!(
        new.is_empty() && gone.is_empty(),
        "cases that move unadmitted: {new:#?}\nadmitted cases that no longer move: {gone:#?}"
    );
}

/// Records `case`'s differences, if any.
fn note(found: &mut BTreeMap<String, Vec<String>>, case: String, diffs: Vec<String>) {
    if !diffs.is_empty() {
        found.entry(case).or_default().extend(diffs);
    }
}

// ------------------------------------------------------------------ //
// The corpus
// ------------------------------------------------------------------ //

type Members = fn(ProfileDoc) -> (ProfileDoc, Vec<RecipeNodeId>);

/// [`pinch_order::tilted_holes`] over three tilted wedges.
fn three_tilted_wedges(doc: ProfileDoc) -> (ProfileDoc, Vec<RecipeNodeId>) {
    pinch_order::tilted_holes(doc, &topo::test_support::meeting::three_wedges())
}

/// [`pinch_order::tilted_holes`] over two tilted wedges.
fn two_tilted_wedges(doc: ProfileDoc) -> (ProfileDoc, Vec<RecipeNodeId>) {
    pinch_order::tilted_holes(doc, &topo::test_support::meeting::two_wedges())
}

/// An upright prism over `corners` on the frame at `origin` spanned by
/// `u` and `v`, extruded `d` along its normal.
fn framed_prism(
    doc: ProfileDoc,
    origin: [f64; 3],
    u: [f64; 3],
    v: [f64; 3],
    corners: &[(f64, f64)],
    d: f64,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, p) = on_frame(doc, origin, u, v, vec![corners.to_vec()]);
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

/// Two prisms whose coincident faces at `z = 1` and `y = 0` meet along
/// one line, a chord between two merged faces
/// (`work/wire/a-chord-between-two-merged-faces-within-no-rim-refuses.md`,
/// witness 1).
fn two_prisms(doc: ProfileDoc) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let ell = [
        (0.0, -1.0),
        (2.0, -1.0),
        (2.0, 0.0),
        (4.0, 0.0),
        (4.0, 2.0),
        (0.0, 2.0),
    ];
    let (doc, a) = framed_prism(doc, [0.0; 3], X, Y, &ell, 1.0);
    let step = [
        (0.0, -1.0),
        (3.0, -1.0),
        (3.0, 1.0),
        (1.0, 1.0),
        (1.0, 2.0),
        (0.0, 2.0),
    ];
    let (doc, b) = framed_prism(doc, [1.0, 0.0, 0.0], Y, Z, &step, 2.0);
    (doc, vec![a, b])
}

/// Two unit blocks kissing at a corner.
fn kiss(doc: ProfileDoc) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (1.0, 2.0), (1.0, 2.0), 1.0, 1.0);
    (doc, vec![a, b])
}

/// The n-ary corpus: members whose unions and intersections meet in
/// contact, by name.
const MEMBERS: &[(&str, Members)] = &[
    ("three in a row", decide_alike::three_in_a_row),
    ("stepped stack", decide_alike::stepped_stack),
    ("table", decide_alike::table),
    ("prism and cube", decide_alike::prism_and_cube),
    ("pinch and far", decide_alike::pinch_and_far),
    ("pinch and cap", decide_alike::pinch_and_cap),
    ("kiss", kiss),
    ("kiss and far", decide_alike::kiss_and_far),
    ("flush onto an edge", decide_alike::flush_onto_an_edge),
    ("plate pinch", pinch_order::pinch),
    ("through", pinch_order::through),
    ("two pinches", pinch_order::two_pinches),
    ("side pinch", pinch_order::side_pinch),
    ("corner holes", pinch_order::corner_holes),
    ("notch and hole", pinch_order::notch_and_hole),
    ("reflex hole", pinch_order::reflex_hole),
    ("two tilted wedges", two_tilted_wedges),
    ("three tilted wedges", three_tilted_wedges),
    ("three on a line", pinch_order::three_on_a_line),
    ("crossing one wedge", pinch_order::crossing_one_wedge),
    (
        "crossing one wedge, prisms",
        pinch_order::crossing_one_wedge_prisms,
    ),
    ("bridging", pinch_order::bridging),
    ("two bridges", pinch_order::two_bridges_short),
    ("two prisms", two_prisms),
];

/// The n-ary verbs, by name.
#[derive(Clone, Copy)]
enum Nary {
    Union,
    Intersect,
}

impl Nary {
    const BOTH: [Self; 2] = [Self::Union, Self::Intersect];

    fn word(self) -> &'static str {
        match self {
            Self::Union => "union",
            Self::Intersect => "intersect",
        }
    }

    fn node(self, members: &[RecipeNodeId]) -> editor_core::AuthoredNode {
        let members = Bodies::Spelled(members.iter().map(|&m| m.into()).collect());
        match self {
            Self::Union => Node::Union { members },
            Self::Intersect => Node::Intersect { members },
        }
    }
}

/// `fixture`'s members combined by `op` in `order`, authored after an
/// unrelated node where `before`, spelled by role (and up to id order
/// where `sets`): what the op publishes.
fn nary(fixture: Members, op: Nary, order: &[usize], before: bool, sets: bool) -> Published {
    let mut ledger = if sets {
        Ledger::up_to_id_order()
    } else {
        Ledger::default()
    };
    let mut doc = ProfileDoc::empty_derived("name-invariance", Tol::witness());
    if before {
        doc = insert(doc, crate::fixture::xy_frame()).0;
    }
    let (doc, m) = ledger.authored(doc, "fixture", fixture);
    let listed: Vec<RecipeNodeId> = order.iter().map(|&i| m[i]).collect();
    let (doc, op_id) = ledger.authored(doc, "op", |d| insert(d, op.node(&listed)));
    published(&doc, op_id, 0, &ledger)
}

fn count(fixture: Members) -> usize {
    fixture(ProfileDoc::empty_derived("count", Tol::witness()))
        .1
        .len()
}

/// **1. Member order**: every n-ary union and intersect of the corpus
/// publishes one table in every member order (all orders up to four
/// members, a fixed sample past that).
#[test]
fn a_member_order_moves_no_name() {
    let mut found = BTreeMap::new();
    for &(what, fixture) in MEMBERS {
        let orders = decide_alike::orders(count(fixture));
        for op in Nary::BOTH {
            let first = nary(fixture, op, &orders[0], false, false);
            for order in &orders[1..] {
                let got = nary(fixture, op, order, false, false);
                note(
                    &mut found,
                    format!("{what} / {}", op.word()),
                    differences(&first, &got, Compare::Geometry)
                        .into_iter()
                        .map(|d| format!("{order:?}: {d}"))
                        .collect(),
                );
            }
        }
    }
    settle(&found, ADMITTED_MEMBER_ORDER);
}

/// The member-order cases that move today.
const ADMITTED_MEMBER_ORDER: &[(&str, &str)] = &[
    ("bridging / union", pinch_order::TWO_GERMS),
    ("pinch and cap / intersect", decide_alike::CARRIED_PINCH),
    ("pinch and far / union", decide_alike::CARRIED_PINCH),
];

/// `x` and `y` combined by `op` as a pair, in that order, `y` the
/// union of the members it lists where it lists several.
fn pair(fixture: Members, op: Nary, x: usize, swapped: bool) -> Published {
    let mut ledger = Ledger::default();
    let doc = ProfileDoc::empty_derived("name-invariance", Tol::witness());
    let (doc, m) = ledger.authored(doc, "fixture", fixture);
    let rest: Vec<RecipeNodeId> = (0..m.len()).filter(|&i| i != x).map(|i| m[i]).collect();
    let (doc, y) = ledger.authored(doc, "rest", |d| match rest.as_slice() {
        [one] => (d, *one),
        many => insert(d, Nary::Union.node(many)),
    });
    let listed = if swapped { [y, m[x]] } else { [m[x], y] };
    let (doc, op_id) = ledger.authored(doc, "op", |d| insert(d, op.node(&listed)));
    published(&doc, op_id, 0, &ledger)
}

/// **2. Operand order**: a two-member union or intersect publishes one
/// table whichever operand is listed first — each member of the corpus
/// against the union of the others.
#[test]
fn an_operand_order_moves_no_name() {
    let mut found = BTreeMap::new();
    for &(what, fixture) in MEMBERS {
        let n = count(fixture);
        for x in 0..if n == 2 { 1 } else { n } {
            for op in Nary::BOTH {
                note(
                    &mut found,
                    format!("{what} / {} / m{x} against the rest", op.word()),
                    differences(
                        &pair(fixture, op, x, false),
                        &pair(fixture, op, x, true),
                        Compare::Geometry,
                    ),
                );
            }
        }
    }
    settle(&found, ADMITTED_OPERAND_ORDER);
}

/// The operand-order cases that move today.
const ADMITTED_OPERAND_ORDER: &[(&str, &str)] = &[
    (
        "two bridges / union / m2 against the rest",
        BRIDGE_LISTED_FIRST,
    ),
    (
        "two bridges / union / m3 against the rest",
        BRIDGE_LISTED_FIRST,
    ),
];

/// A bridge listed before the union of the wedges and the other bridge
/// refuses that a seam was offered one edge twice.
const BRIDGE_LISTED_FIRST: &str = "work/wire/a-pair-union-of-a-bridge-listed-first-and-the-wedges-offers-a-seam-one-edge-twice.md";

// ------------------------------------------------------------------ //
// Splits
// ------------------------------------------------------------------ //

/// A split's target: an outline drawn on `xy`, extruded 1 along `z` or
/// revolved a full turn about `y`, or a union of the n-ary corpus.
#[derive(Clone, Copy)]
enum Target {
    Prism(&'static [(f64, f64)]),
    Turned(&'static [(f64, f64)]),
    Union(Members),
}

/// `target` authored: the document, its outline's profile where it has
/// one, and the node split.
fn target(doc: ProfileDoc, t: Target) -> (ProfileDoc, (Option<RecipeNodeId>, RecipeNodeId)) {
    match t {
        Target::Prism(outline) | Target::Turned(outline) => {
            let (doc, p) = on_frame(doc, [0.0; 3], X, Y, vec![outline.to_vec()]);
            let node = if matches!(t, Target::Prism(_)) {
                Node::Extrude {
                    profile: p.into(),
                    distance: len(1.0),
                    side: ExtrudeSide::Along,
                }
            } else {
                Node::Revolve {
                    profile: p.into(),
                    axis_through: [len(0.0), len(0.0)],
                    axis_heading: Formula::turn_over(4).expect("a quarter turn"),
                    angle: ang(std::f64::consts::TAU),
                }
            };
            let (doc, s) = insert(doc, node);
            (doc, (Some(p), s))
        }
        Target::Union(members) => {
            let (doc, m) = members(doc);
            let (doc, u) = insert(doc, Nary::Union.node(&m));
            (doc, (None, u))
        }
    }
}

/// The split corpus: a target and the plane through `origin` with
/// `normal` that cuts it through its contact.
type SplitCase = (&'static str, Target, [f64; 3], [f64; 3]);

const SPLITS: &[SplitCase] = &[
    (
        "notched block",
        Target::Prism(crate::split_pinch_keeps::NOTCHED),
        [0.0, 1.0, 0.0],
        Y,
    ),
    (
        "mirrored notched block",
        Target::Prism(crate::split_pinch_keeps::MIRRORED),
        [0.0, 1.0, 0.0],
        Y,
    ),
    (
        "notched pentagon",
        Target::Prism(crate::split_pinch_keeps::PENTAGON),
        [0.0, 1.0, 0.0],
        Y,
    ),
    (
        "revolved pentagon",
        Target::Turned(crate::split_pinch_keeps::REVOLVED),
        [0.0, 1.0, 0.0],
        Y,
    ),
    (
        "plate pinch at y = 1",
        Target::Union(pinch_order::pinch),
        [0.0, 1.0, 0.0],
        Y,
    ),
    (
        "plate pinch at x = 1.5",
        Target::Union(pinch_order::pinch),
        [1.5, 0.0, 0.0],
        X,
    ),
];

/// `t` split by the plane through `origin` with `normal`, both
/// halves' tables, `edit` applied to the document first.
fn split(
    t: Target,
    origin: [f64; 3],
    normal: [f64; 3],
    edit: &dyn Fn(ProfileDoc, Option<RecipeNodeId>) -> ProfileDoc,
) -> [Published; 2] {
    let mut ledger = Ledger::default();
    let doc = ProfileDoc::empty_derived("name-invariance", Tol::witness());
    let (doc, (profile, node)) = ledger.authored(doc, "target", |d| target(d, t));
    let (doc, plane) = ledger.authored(doc, "plane", |d| {
        insert(
            d,
            Node::Datum(Datum::Plane {
                origin: origin.map(len),
                normal: normal.map(scl),
            }),
        )
    });
    let (doc, sp) = ledger.authored(doc, "split", |d| {
        insert(
            d,
            Node::Split {
                target: editor_core::Operand::output(node, 0).into(),
                tool: plane.into(),
            },
        )
    });
    let ev = run(&edit(doc, profile));
    [0, 1].map(|port| published_in(&ev, sp, port, &ledger))
}

fn unedited(doc: ProfileDoc, _: Option<RecipeNodeId>) -> ProfileDoc {
    doc
}

/// An in-plane direction of the plane with `normal`.
fn along(normal: [f64; 3]) -> [f64; 3] {
    if normal[2].abs() < 0.9 {
        [0.37 * normal[1], -0.37 * normal[0], 0.21]
    } else {
        [0.37, 0.0, 0.0]
    }
}

/// **3. A split plane's frame**: flipping its normal swaps its halves
/// and renames nothing in them, and moving its origin along it renames
/// nothing.
#[test]
fn a_split_planes_frame_moves_no_name() {
    let mut found = BTreeMap::new();
    for &(what, t, o, n) in SPLITS {
        let first = split(t, o, n, &unedited);
        assert!(
            first.iter().all(|h| matches!(h, Published::Table(_))),
            "{what}: the split names both halves: {first:?}"
        );
        let flipped = split(t, o, n.map(|c| -c), &unedited);
        let a = along(n);
        let moved = split(t, [o[0] + a[0], o[1] + a[1], o[2] + a[2]], n, &unedited);
        for port in 0..2 {
            note(
                &mut found,
                format!("{what} / flipped"),
                differences(&first[port], &flipped[1 - port], Compare::Geometry)
                    .into_iter()
                    .map(|d| format!("half {port}: {d}"))
                    .collect(),
            );
            note(
                &mut found,
                format!("{what} / moved along"),
                differences(&first[port], &moved[port], Compare::Geometry)
                    .into_iter()
                    .map(|d| format!("half {port}: {d}"))
                    .collect(),
            );
        }
    }
    settle(&found, ADMITTED_SPLIT_FRAME);
}

/// The split-frame cases that move today.
const ADMITTED_SPLIT_FRAME: &[(&str, &str)] = &[];

// ------------------------------------------------------------------ //
// Tilts
// ------------------------------------------------------------------ //

/// `v` turned by `deg` degrees about `axis` (a unit vector).
fn turned(v: [f64; 3], axis: [f64; 3], deg: f64) -> [f64; 3] {
    let (s, c) = deg.to_radians().sin_cos();
    let dot = v[0] * axis[0] + v[1] * axis[1] + v[2] * axis[2];
    let cross = [
        axis[1] * v[2] - axis[2] * v[1],
        axis[2] * v[0] - axis[0] * v[2],
        axis[0] * v[1] - axis[1] * v[0],
    ];
    [0, 1, 2].map(|i| v[i] * c + cross[i] * s + axis[i] * dot * (1.0 - c))
}

/// [`decide_alike::prism_and_cube`] with the cube's normal turned `deg`
/// about `x` through the L's reflex corner, so the pinch stays.
fn prism_and_tilted_cube(doc: ProfileDoc, deg: f64) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let ell = [
        (0.0, 0.0),
        (2.0, 0.0),
        (2.0, 1.0),
        (1.0, 1.0),
        (1.0, 2.0),
        (0.0, 2.0),
    ];
    let (doc, prism) = framed_prism(doc, [0.0; 3], X, Y, &ell, 1.0);
    let unit = |m: [f64; 3]| {
        let n = m.iter().map(|c| c * c).sum::<f64>().sqrt();
        m.map(|c| c / n)
    };
    let m = turned(unit([1.0, 1.3, 0.7]), X, deg);
    let u = unit([-m[2] * m[0], -m[2] * m[1], 1.0 - m[2] * m[2]]);
    let v = [
        m[1] * u[2] - m[2] * u[1],
        m[2] * u[0] - m[0] * u[2],
        m[0] * u[1] - m[1] * u[0],
    ];
    let square = [(-2.0, -2.0), (2.0, -2.0), (2.0, 2.0), (-2.0, 2.0)];
    let (doc, cube) = framed_prism(doc, [1.0, 1.0, 1.0], u, v, &square, 4.0);
    (doc, vec![prism, cube])
}

/// [`pinch_order::pinch`] with `p2`'s footprint turned `deg` about the
/// vertical line through the pinch corner (1.5, 1).
fn pinch_turned(doc: ProfileDoc, deg: f64) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let (doc, plate) = block(doc, (0.0, 3.0), (0.0, 2.0), 0.0, 1.0);
    let (doc, p1) = block(doc, (1.0, 1.5), (-1.0, 1.0), 0.5, 1.5);
    let u = turned(X, Z, deg);
    let v = turned(Y, Z, deg);
    let foot = [(0.0, 0.0), (0.5, 0.0), (0.5, 2.0), (0.0, 2.0)];
    let (doc, p2) = framed_prism(doc, [1.5, 1.0, 0.47], u, v, &foot, 1.23);
    (doc, vec![plate, p1, p2])
}

/// [`pinch_order::crossing_one_wedge`] with the crossing prism's
/// sector turned `deg` about the contact line.
fn crossing_turned(doc: ProfileDoc, deg: f64) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let (doc, plate) = block(doc, (0.0, 3.0), (0.0, 2.0), 0.0, 1.0);
    let (doc, a) = pinch_order::sector_prism(doc, 0.0, 50.0, (0.5, 2.0));
    let (doc, b) = pinch_order::sector_prism(doc, 120.0, 170.0, (0.47, 1.7));
    let (doc, c) = pinch_order::sector_prism(doc, 30.0 + deg, 60.0 + deg, (0.44, 1.81));
    (doc, vec![plate, a, b, c])
}

/// A union of members posed by a tilt, by name.
type Posed = fn(ProfileDoc, f64) -> (ProfileDoc, Vec<RecipeNodeId>);

const POSED: &[(&str, Posed)] = &[
    ("prism and tilted cube", prism_and_tilted_cube),
    ("plate pinch, p2 turned", pinch_turned),
    ("crossing one wedge, turned", crossing_turned),
];

/// The tilts each pose is compared at, in degrees.
const TILTS: [f64; 3] = [-0.3, 0.2, 1.0];

/// The union of `posed` at `deg`: its table, and its topology's counts.
fn posed_union(posed: Posed, deg: f64) -> (Published, Option<[usize; 3]>) {
    let mut ledger = Ledger::default();
    let doc = ProfileDoc::empty_derived("name-invariance", Tol::witness());
    let (doc, m) = ledger.authored(doc, "fixture", |d| posed(d, deg));
    let (doc, u) = ledger.authored(doc, "op", |d| insert(d, Nary::Union.node(&m)));
    let ev = run(&doc);
    let counts = ev.value(u).and_then(|v| match &v.payload {
        ValuePayload::Boolean(BooleanValue::Body { body, .. }) => Some([
            body.faces().count(),
            body.edges().count(),
            body.vertices().count(),
        ]),
        _ => None,
    });
    (published(&doc, u, 0, &ledger), counts)
}

/// The pentagon's split by the vertical plane through its apex edge,
/// turned `deg` about that edge: both halves, and their counts.
fn pentagon_split_turned(deg: f64) -> ([Published; 2], [usize; 3]) {
    let n = turned(Y, Z, deg);
    let halves = split(
        Target::Prism(crate::split_pinch_keeps::PENTAGON),
        [1.0, 1.0, 0.0],
        n,
        &unedited,
    );
    let counts = halves
        .iter()
        .map(|h| match h {
            Published::Table(t) => t.values().flatten().fold([0; 3], |mut c, x| {
                match x.kind {
                    'f' => c[0] += 1,
                    'e' => c[1] += 1,
                    'v' => c[2] += 1,
                    _ => {}
                }
                c
            }),
            _ => [0; 3],
        })
        .fold([0; 3], |a, b| [a[0] + b[0], a[1] + b[1], a[2] + b[2]]);
    (halves, counts)
}

/// **4. A tilt within one topology**: a member turned a little, or a
/// split plane turned about the edge it holds, renames nothing — each
/// name keeps its kind and its incidences. A pose whose topology
/// changes is not compared, and says so; every subject compares one
/// pose at least.
#[test]
fn a_tilt_within_one_topology_moves_no_name() {
    let mut found = BTreeMap::new();
    for &(what, posed) in POSED {
        let (first, counts) = posed_union(posed, 0.0);
        assert!(
            matches!(first, Published::Table(_)),
            "{what}: the union names: {}",
            outline(&first)
        );
        let mut compared = 0;
        for deg in TILTS {
            let (got, c) = posed_union(posed, deg);
            if c != counts {
                println!("SKIPPED {what} at {deg}°: its topology {c:?} is not {counts:?}");
                continue;
            }
            compared += 1;
            note(
                &mut found,
                what.to_string(),
                differences(&first, &got, Compare::Incidence)
                    .into_iter()
                    .map(|d| format!("{deg}°: {d}"))
                    .collect(),
            );
        }
        assert!(compared > 0, "{what}: some tilt keeps the topology");
    }
    let (first, counts) = pentagon_split_turned(0.0);
    assert!(
        first.iter().all(|h| matches!(h, Published::Table(_))),
        "the pentagon's split names both halves"
    );
    let mut compared = 0;
    for deg in TILTS {
        let (got, c) = pentagon_split_turned(deg);
        if c != counts {
            println!("SKIPPED the pentagon's split at {deg}°: {c:?} is not {counts:?}");
            continue;
        }
        compared += 1;
        for port in 0..2 {
            note(
                &mut found,
                "notched pentagon, plane turned".to_string(),
                differences(&first[port], &got[port], Compare::Incidence)
                    .into_iter()
                    .map(|d| format!("{deg}°, half {port}: {d}"))
                    .collect(),
            );
        }
    }
    assert!(
        compared > 0,
        "the pentagon's split: some tilt keeps the topology"
    );
    settle(&found, ADMITTED_TILT);
}

/// The tilt cases that move today.
const ADMITTED_TILT: &[(&str, &str)] = &[];

// ------------------------------------------------------------------ //
// Twins and histories
// ------------------------------------------------------------------ //

/// One member of a role-built fixture: its role and how it is authored.
type Member = (&'static str, fn(ProfileDoc) -> (ProfileDoc, RecipeNodeId));

/// What a role-built fixture's members are combined into.
#[derive(Clone, Copy)]
enum Combine {
    /// One n-ary op over every member, in role order.
    Nary(Nary),
    /// The first member less the union of the others.
    Cut,
}

/// A role-built fixture: its members, in the order the op lists them.
type Roles = (&'static str, &'static [Member], Combine);

/// Two twin blocks, one footprint size, touching along the vertical
/// line through (1.5, 1).
fn twin_left(doc: ProfileDoc) -> (ProfileDoc, RecipeNodeId) {
    block(doc, (1.0, 1.5), (0.5, 1.0), 0.5, 1.5)
}

fn twin_right(doc: ProfileDoc) -> (ProfileDoc, RecipeNodeId) {
    block(doc, (1.5, 2.0), (1.0, 1.5), 0.5, 1.5)
}

/// A plate under the twins.
fn plate(doc: ProfileDoc) -> (ProfileDoc, RecipeNodeId) {
    block(doc, (0.0, 3.0), (0.0, 2.0), 0.0, 1.0)
}

/// A unit block and its exact copy.
fn unit_block(doc: ProfileDoc) -> (ProfileDoc, RecipeNodeId) {
    block(doc, (1.0, 2.0), (0.5, 1.5), 0.5, 1.0)
}

/// Two wedges and a prism crossing one, about (1.5, 1).
fn wedge_a(doc: ProfileDoc) -> (ProfileDoc, RecipeNodeId) {
    pinch_order::sector_prism(doc, 0.0, 50.0, (0.5, 2.0))
}

fn wedge_b(doc: ProfileDoc) -> (ProfileDoc, RecipeNodeId) {
    pinch_order::sector_prism(doc, 120.0, 170.0, (0.47, 1.7))
}

fn wedge_c(doc: ProfileDoc) -> (ProfileDoc, RecipeNodeId) {
    pinch_order::sector_prism(doc, 30.0, 60.0, (0.44, 1.81))
}

/// The pinch's blocks.
fn pinch_p1(doc: ProfileDoc) -> (ProfileDoc, RecipeNodeId) {
    block(doc, (1.0, 1.5), (-1.0, 1.0), 0.5, 1.5)
}

fn pinch_p2(doc: ProfileDoc) -> (ProfileDoc, RecipeNodeId) {
    block(doc, (1.5, 2.0), (1.0, 3.0), 0.47, 1.23)
}

/// The kiss's blocks.
fn kiss_a(doc: ProfileDoc) -> (ProfileDoc, RecipeNodeId) {
    block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0)
}

fn kiss_b(doc: ProfileDoc) -> (ProfileDoc, RecipeNodeId) {
    block(doc, (1.0, 2.0), (1.0, 2.0), 1.0, 1.0)
}

/// Fixtures with two structurally identical operands: twin blocks and
/// exact copies.
const TWINS: &[Roles] = &[
    (
        "twins",
        &[("left", twin_left), ("right", twin_right)],
        Combine::Nary(Nary::Union),
    ),
    (
        "twins on the plate",
        &[("plate", plate), ("left", twin_left), ("right", twin_right)],
        Combine::Nary(Nary::Union),
    ),
    (
        "the plate less the twins",
        &[("plate", plate), ("left", twin_left), ("right", twin_right)],
        Combine::Cut,
    ),
    (
        "a block and its copy",
        &[("one", unit_block), ("copy", unit_block)],
        Combine::Nary(Nary::Union),
    ),
    (
        "a block and its copy, met",
        &[("one", unit_block), ("copy", unit_block)],
        Combine::Nary(Nary::Intersect),
    ),
    (
        "the plate less a block and its copy",
        &[("plate", plate), ("one", unit_block), ("copy", unit_block)],
        Combine::Cut,
    ),
];

/// Contact-heavy fixtures, by role, for the authoring history.
const HISTORIES: &[Roles] = &[
    (
        "plate pinch",
        &[("plate", plate), ("p1", pinch_p1), ("p2", pinch_p2)],
        Combine::Nary(Nary::Union),
    ),
    (
        "crossing one wedge",
        &[
            ("plate", plate),
            ("a", wedge_a),
            ("b", wedge_b),
            ("c", wedge_c),
        ],
        Combine::Nary(Nary::Union),
    ),
    (
        "kiss",
        &[("a", kiss_a), ("b", kiss_b)],
        Combine::Nary(Nary::Union),
    ),
    (
        "the plate less the pinch's blocks",
        &[("plate", plate), ("p1", pinch_p1), ("p2", pinch_p2)],
        Combine::Cut,
    ),
];

/// `roles` authored in `authoring` order (indices into it), combined
/// in role order: what the combination publishes, ids by role.
fn by_roles(roles: &Roles, authoring: &[usize]) -> Published {
    let (_, members, combine) = *roles;
    let mut ledger = Ledger::up_to_id_order();
    let mut doc = ProfileDoc::empty_derived("name-invariance", Tol::witness());
    let mut ids = vec![None; members.len()];
    for &i in authoring {
        let (role, f) = members[i];
        let (d, id) = ledger.authored(doc, role, f);
        doc = d;
        ids[i] = Some(id);
    }
    let ids: Vec<RecipeNodeId> = ids.into_iter().map(|i| i.expect("authored")).collect();
    let (doc, op) = match combine {
        Combine::Nary(op) => ledger.authored(doc, "op", |d| insert(d, op.node(&ids))),
        Combine::Cut => {
            let (doc, tool) =
                ledger.authored(doc, "tool", |d| insert(d, Nary::Union.node(&ids[1..])));
            ledger.authored(doc, "op", |d| {
                insert(
                    d,
                    Node::Subtract {
                        from: ids[0].into(),
                        tool: tool.into(),
                    },
                )
            })
        }
    };
    published(&doc, op, 0, &ledger)
}

/// **5. Twin operands**: where two operands are separately authored and
/// structurally identical, which of them was authored first renames
/// nothing — their names differ by their ids alone, and no name may
/// read those ids' order.
#[test]
fn which_twin_was_authored_first_moves_no_name() {
    let mut found = BTreeMap::new();
    for roles in TWINS {
        let n = roles.1.len();
        let forward: Vec<usize> = (0..n).collect();
        let mut swapped = forward.clone();
        swapped.swap(n - 2, n - 1);
        note(
            &mut found,
            roles.0.to_string(),
            differences(
                &by_roles(roles, &forward),
                &by_roles(roles, &swapped),
                Compare::Geometry,
            ),
        );
    }
    settle(&found, ADMITTED_TWINS);
}

/// The twin cases that move today.
const ADMITTED_TWINS: &[(&str, &str)] = &[
    ("a block and its copy", COPY_MINTED_FIRST),
    ("a block and its copy, met", COPY_MINTED_FIRST),
    ("the plate less a block and its copy", COPY_MINTED_FIRST),
    ("twins on the plate", LEG_MINTED_FIRST),
];

/// A boolean of a body and its exact copy names each entity of the
/// result from the copy minted first.
const COPY_MINTED_FIRST: &str =
    "work/wire/a-boolean-of-a-body-and-its-copy-names-its-entities-from-the-copy-minted-first.md";

/// Where two members' legs cross a face at one pinch point, the vertex
/// is named the crossing of the leg minted first.
const LEG_MINTED_FIRST: &str =
    "work/wire/a-pinch-where-two-legs-cross-one-face-is-named-by-the-leg-minted-first.md";

/// **7. Authoring history**: one recipe authored in another order, or
/// after an unrelated node, publishes one table up to the bijection of
/// its minted ids.
#[test]
fn an_authoring_history_moves_no_name() {
    let mut found = BTreeMap::new();
    for roles in HISTORIES.iter().chain(TWINS) {
        let n = roles.1.len();
        let forward: Vec<usize> = (0..n).collect();
        let first = by_roles(roles, &forward);
        for order in decide_alike::orders(n)
            .into_iter()
            .filter(|o| *o != forward)
        {
            note(
                &mut found,
                format!("{} / authored in another order", roles.0),
                differences(&first, &by_roles(roles, &order), Compare::Geometry)
                    .into_iter()
                    .map(|d| format!("{order:?}: {d}"))
                    .collect(),
            );
        }
    }
    for &(what, fixture) in MEMBERS {
        let order: Vec<usize> = (0..count(fixture)).collect();
        note(
            &mut found,
            format!("{what} / after an unrelated node"),
            differences(
                &nary(fixture, Nary::Union, &order, false, true),
                &nary(fixture, Nary::Union, &order, true, true),
                Compare::Geometry,
            ),
        );
    }
    settle(&found, ADMITTED_HISTORY);
}

/// The history cases that move today.
const ADMITTED_HISTORY: &[(&str, &str)] = &[
    (
        "a block and its copy / authored in another order",
        COPY_MINTED_FIRST,
    ),
    (
        "a block and its copy, met / authored in another order",
        COPY_MINTED_FIRST,
    ),
    (
        "the plate less a block and its copy / authored in another order",
        COPY_MINTED_FIRST,
    ),
    ("plate pinch / authored in another order", LEG_MINTED_FIRST),
    (
        "twins on the plate / authored in another order",
        LEG_MINTED_FIRST,
    ),
];

// ------------------------------------------------------------------ //
// Far features
// ------------------------------------------------------------------ //

/// `fixture`'s union, with a far block as a last member where `far`.
fn with_far_member(fixture: Members, far: bool) -> Published {
    let mut ledger = Ledger::default();
    let doc = ProfileDoc::empty_derived("name-invariance", Tol::witness());
    let (doc, mut m) = ledger.authored(doc, "fixture", fixture);
    let doc = if far {
        let (doc, b) =
            ledger.authored(doc, "far", |d| block(d, (9.0, 10.0), (9.0, 10.0), 0.0, 1.0));
        m.push(b);
        doc
    } else {
        doc
    };
    let (doc, u) = ledger.authored(doc, "op", |d| insert(d, Nary::Union.node(&m)));
    published(&doc, u, 0, &ledger)
}

/// A hole far from each split outline's plane, inside its outline.
fn far_hole(t: Target) -> Vec<(f64, f64)> {
    let (x, y) = match t {
        Target::Prism(o) | Target::Turned(o) if o == crate::split_pinch_keeps::NOTCHED => {
            (0.5, 0.2)
        }
        Target::Prism(o) | Target::Turned(o) if o == crate::split_pinch_keeps::MIRRORED => {
            (0.5, 1.5)
        }
        Target::Turned(_) => (2.6, 0.2),
        _ => (1.6, 0.2),
    };
    let s = 0.2;
    vec![(x, y), (x, y + s), (x + s, y + s), (x + s, y)]
}

/// The edit that adds `hole` to `profile`'s outline `outline`, every
/// old step kept.
fn add_hole(
    doc: ProfileDoc,
    profile: RecipeNodeId,
    outline: &[(f64, f64)],
    hole: Vec<(f64, f64)>,
) -> ProfileDoc {
    let mut ids = match doc.node(profile) {
        Some(Node::Profile(p)) => p.kept_in_place(),
        other => panic!("a profile: {other:?}"),
    };
    let hole = LoopProgram::polygon(hole).expect("finite corners");
    let LoopProgram::Chain(steps) = &hole else {
        panic!("a polygon is a chain")
    };
    ids.push(vec![None; steps.len()]);
    let loops = vec![
        LoopProgram::polygon(outline.to_vec()).expect("finite corners"),
        hole,
    ];
    crate::fixture::step(
        doc,
        DocEdit::SetProgram {
            node: profile,
            loops,
            ids,
            fresh: Vec::new(),
        },
    )
    .0
}

/// **6. A far feature**: a far block added to a union, or a hole
/// added to a split outline away from its plane, leaves every other
/// name where it was — its kind, its incidences (gaining only the
/// feature's), and its vertices' points.
#[test]
fn a_far_feature_moves_no_other_name() {
    let mut found = BTreeMap::new();
    for &(what, fixture) in MEMBERS {
        note(
            &mut found,
            format!("{what} / a far member"),
            differences(
                &with_far_member(fixture, false),
                &with_far_member(fixture, true),
                Compare::Kept,
            ),
        );
    }
    for &(what, t, o, n) in SPLITS {
        let (Target::Prism(outline) | Target::Turned(outline)) = t else {
            continue;
        };
        let first = split(t, o, n, &unedited);
        let holed = split(t, o, n, &|doc, profile| {
            add_hole(
                doc,
                profile.expect("an outline's profile"),
                outline,
                far_hole(t),
            )
        });
        let size = |h: &[Published; 2]| {
            h.iter()
                .map(|p| match p {
                    Published::Table(t) => t.len(),
                    _ => 0,
                })
                .sum::<usize>()
        };
        assert!(
            size(&holed) > size(&first),
            "{what}: the hole adds names to the halves"
        );
        for port in 0..2 {
            note(
                &mut found,
                format!("{what} / a far hole"),
                differences(&first[port], &holed[port], Compare::Kept)
                    .into_iter()
                    .map(|d| format!("half {port}: {d}"))
                    .collect(),
            );
        }
    }
    settle(&found, ADMITTED_FAR);
}

/// The far-feature cases that move today.
const ADMITTED_FAR: &[(&str, &str)] =
    &[("prism and cube / a far member", decide_alike::CARRIED_PINCH)];
