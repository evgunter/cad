//! **A name reads nothing the author did not write** (NAMES N1, N4;
//! DM4): each test here evaluates a recipe, perturbs something its
//! names must not read, evaluates again, and compares the two tables.
//!
//! A table is read as its names, each beside its candidates (one, or a
//! tie's several, matched one to one): an entity's kind, its geometry
//! (a vertex's point, an edge's end points, a face's boundary vertex
//! points, rounded to the micrometre) and the names of the entities it
//! is incident with. Every minted id is first carried, through
//! [`remap_name`], to the id of what its authoring step was called in a
//! [`Ledger`]: two documents compare up to the bijection of what each
//! authored, and the production rewrite puts every name back in its
//! canonical form, so a set a name orders by id is spelled alike
//! whatever order the ids were minted in. A refusal is compared by its
//! kind; an empty result as empty.
//!
//! The perturbations, each its own test over the contact-heavy corpus
//! below (pinches, flush faces, tilted wedges, prisms on a contact
//! line, notched and pentagon splits, merged chords across a sunk
//! boss):
//! 1. the member order of an n-ary union and intersect;
//! 2. the operand order of a two-member union and intersect;
//! 3. a rigid frame change of a split plane: its normal's sign, the
//!    halves swapped; its origin moved along it; and, for a frame's xy
//!    plane, the frame spun about its normal;
//! 4. a small tilt within one topology, compared by incidence;
//! 5. a far feature added: every other name keeps its incidences and
//!    its vertices' points;
//! 6. one recipe under another authoring history: its members authored
//!    in every order (twins and exact copies among them), or after an
//!    unrelated node.
//!
//! An operand's arena storage permuted is not among them: `topo` has no
//! helper that permutes a body
//! (`work/namecheck/topo-has-no-arena-permuting-helper-for-a-storage-order-name-check.md`).
//!
//! Each test settles two lists. Its admissions are the cases a
//! perturbation moves today, each with its exact count of differing
//! names and the row that owns it; a case outside the list that moves,
//! one in it that no longer does, or one that moves by another count, is
//! red. Its vacuous cases are the ones that compare no table on either
//! side in any comparison (both refuse, say), each citing the row whose
//! fix makes it compare; a case that starts comparing, or a new one that
//! compares nothing, is red too. A case whose members simply do not
//! meet (an intersect of bodies that only touch) is not in the corpus.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::{BTreeMap, BTreeSet};
use std::sync::Mutex;

use crate::blocks::{Spans, block, block_of, block_on, block_on_of, failure, run};
use crate::emit_union_seam_rank as seam_scene;
use crate::fixture::{
    against, ang, axes_at, desc, ends, face_edges, face_vertices, in_world_by, insert, len, point,
    turned_to,
};
use crate::union_member_orders_decide_alike as decide_alike;
use crate::union_pinch_member_order::{self as pinch_order, Sector};
use crate::wire_legal_union_refusals as sunk;
use editor_core::pose::{PoseRole, RoleRead};
use editor_core::{
    Bodies, BooleanValue, DocEdit, EntityKey, Entry, Formula, LoopProgram, MintId, Node, NodeMap,
    Operand, ProfileDoc, RecipeNodeId, StepId, StepMap, TargetAxis, ValuePayload, VarId,
    remap_name,
};
use geom_core::Tol;

// ------------------------------------------------------------------ //
// The harness
// ------------------------------------------------------------------ //

/// Every label a [`Ledger`] has spelled, in the order first met: a
/// label's index here is its canonical ids' block, the same in every
/// document a test compares.
static LABELS: Mutex<Vec<String>> = Mutex::new(Vec::new());

/// The ordinals one label's canonical ids may take.
const SPAN: u32 = 100_000;

/// Where the canonical ids of ordinals no span holds begin.
const UNLABELED: u32 = 3_000_000_000;

fn label_index(label: &str) -> u32 {
    let mut labels = LABELS.lock().expect("the labels");
    let at = labels.iter().position(|l| l == label).unwrap_or_else(|| {
        labels.push(label.to_string());
        labels.len() - 1
    });
    u32::try_from(at).expect("few labels")
}

/// **What each authored span of a document is called**: the mint
/// ordinals `[start, end)` an authoring step drew, and its label. An id
/// is carried to the canonical id of its span's label and its offset
/// there, so two documents that author the same things in another
/// order, or after something else, carry them to the same ids.
#[derive(Default, Clone)]
struct Ledger {
    spans: Vec<(u32, u32, String)>,
}

impl Ledger {
    /// `f` run on `doc`, its mints recorded as `label`.
    fn authored<T>(
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

    /// `node` recorded as `label` alone, ahead of the step that minted
    /// it: a node whose step mints a number of other ids the
    /// perturbation varies (a split's tool, read inline) keeps one
    /// canonical id.
    fn alone(&mut self, node: RecipeNodeId, label: &str) {
        let o = node.0.ordinal();
        self.spans.insert(0, (o, o + 1, label.to_string()));
    }

    fn place(&self, ordinal: u32) -> Option<(&str, u32)> {
        self.spans
            .iter()
            .find(|(s, e, _)| (*s..*e).contains(&ordinal))
            .map(|(s, _, l)| (l.as_str(), ordinal - s))
    }

    /// The canonical id of `m`: its label's block and its offset.
    fn canonical(&self, m: MintId) -> MintId {
        match self.place(m.ordinal()) {
            _ if m.ordinal() == 0 => m,
            Some((l, off)) => {
                assert!(off < SPAN, "a span of fewer than {SPAN} ids");
                MintId::new(1 + label_index(l) * SPAN + off, 0)
            }
            None => MintId::new(UNLABELED + m.ordinal(), 0),
        }
    }

    /// `doc`'s ids carried to their canonical ids, as [`remap_name`]
    /// takes them.
    fn maps(&self, doc: &ProfileDoc) -> (NodeMap, StepMap, BTreeMap<VarId, VarId>) {
        let mint = doc.mint();
        let nodes = doc
            .ids()
            .into_iter()
            .map(|n| (n, RecipeNodeId(self.canonical(n.0))))
            .collect();
        let steps = mint
            .steps()
            .map(|s| (s, StepId(self.canonical(s.0))))
            .collect();
        let mut reads: BTreeMap<VarId, VarId> = mint
            .vars()
            .map(|v| (v, VarId(self.canonical(v.0))))
            .collect();
        // The read no node defines, which a kernel-built name cites.
        let none = VarId(MintId::new(0, 0));
        reads.insert(none, none);
        (nodes, steps, reads)
    }

    /// `debug` with every raw minted id in it spelled by its span.
    fn respell(&self, debug: &str) -> String {
        each_mint(debug, |ordinal| match self.place(ordinal) {
            Some((l, off)) => format!("{l}+{off}"),
            None => format!("#{ordinal}"),
        })
    }
}

/// `debug` with every canonical id in it spelled by its label.
fn spelled_canonical(debug: &str) -> String {
    let labels = LABELS.lock().expect("the labels");
    each_mint(debug, |ordinal| match ordinal {
        0 => "none".to_string(),
        o if o >= UNLABELED => format!("#{}", o - UNLABELED),
        o => {
            let (block, off) = ((o - 1) / SPAN, (o - 1) % SPAN);
            format!("{}+{off}", labels[block as usize])
        }
    })
}

/// `debug` with each `MintId { .. }` in it replaced by `spell` of its
/// ordinal.
fn each_mint(debug: &str, spell: impl Fn(u32) -> String) -> String {
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
        out.push_str(&spell(ordinal));
        rest = &tail[close + 1..];
    }
    out.push_str(rest);
    out
}

/// The ordinal `doc`'s next mint draws.
fn next_ordinal(doc: &ProfileDoc) -> u32 {
    let mint = doc.mint();
    let nodes = doc.ids().into_iter().map(|n| n.0.ordinal());
    let steps = mint.steps().map(|s| s.0.ordinal());
    let vars = mint.vars().map(|v| v.0.ordinal());
    let constraints = doc
        .ids()
        .into_iter()
        .filter_map(|n| match doc.node(n) {
            Some(Node::Place { constraints, .. }) => {
                Some(constraints.iter().map(|c| c.id.0.ordinal()))
            }
            _ => None,
        })
        .flatten();
    nodes
        .chain(steps)
        .chain(vars)
        .chain(constraints)
        .max()
        .unwrap_or(0)
        + 1
}

/// One entity a name denotes, as the comparisons read it.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Cand {
    kind: char,
    geo: Vec<(i64, i64, i64)>,
    near: BTreeSet<String>,
}

/// What one output of one node published.
#[derive(Debug, Clone, PartialEq)]
enum Published {
    /// Its names, each beside its candidates, sorted.
    Table(BTreeMap<String, Vec<Cand>>),
    /// The empty result.
    Empty,
    /// A refusal, by its kind.
    Refused(String),
}

impl Published {
    fn is_table(&self) -> bool {
        matches!(self, Self::Table(_))
    }
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

/// **What `node`'s output `port` published in `doc`**, its ids carried
/// by `ledger`.
fn published(doc: &ProfileDoc, node: RecipeNodeId, port: u8, ledger: &Ledger) -> Published {
    published_in(&run(doc), doc, node, port, ledger)
}

/// [`published`], read off `doc`'s evaluation `ev`.
///
/// `emit_boolean_vertex_keys::named_geometry` reads a table the same
/// way, but panics where the node refuses, reads port 0 alone, and
/// holds neither a candidate's incidences nor a tie's candidates apart,
/// which perturbations 3 to 5 compare.
fn published_in(
    ev: &editor_core::Evaluation<f64>,
    doc: &ProfileDoc,
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
    let (nodes, steps, reads) = ledger.maps(doc);
    let var = ev
        .output_var(node, port)
        .expect("the node defines the port");
    let table = ev.table(var).expect("the port's table");
    let mut names: BTreeMap<(u32, EntityKey), Vec<String>> = BTreeMap::new();
    let mut entries = Vec::new();
    for (n, e) in table.iter() {
        let spelled = match remap_name(n, &nodes, &steps, &reads) {
            Ok(c) => spelled_canonical(&format!("{c:?}")),
            Err(err) => format!(
                "unremappable ({}): {}",
                ledger.respell(&format!("{err:?}")),
                ledger.respell(&format!("{n:?}"))
            ),
        };
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
            // Where the body stands in its space: an operation computes
            // in its first operand's frame, which a member order moves.
            let body = crate::fixture::in_space(
                ev.read_body(var, r.body)
                    .expect("the read evaluated")
                    .expect("the read holds the body"),
            );
            let at = |v| {
                let p = point(&body, v);
                (micro(p.x), micro(p.y), micro(p.z))
            };
            let named = |k: EntityKey| -> Vec<String> {
                names.get(&(r.body, k)).cloned().unwrap_or_default()
            };
            let (kind, mut geo, adjacent): (char, Vec<_>, Vec<EntityKey>) = match r.key {
                EntityKey::Vertex(v) => {
                    let mut adj = Vec::new();
                    for (e, _) in body.edges() {
                        if ends(&body, e).contains(&v) {
                            adj.push(EntityKey::Edge(e));
                        }
                    }
                    for (f, _) in body.faces() {
                        if face_vertices(&body, f).contains(&v) {
                            adj.push(EntityKey::Face(f));
                        }
                    }
                    ('v', vec![at(v)], adj)
                }
                EntityKey::Edge(e) => {
                    let [a, b] = ends(&body, e);
                    let mut adj = vec![EntityKey::Vertex(a), EntityKey::Vertex(b)];
                    for (f, _) in body.faces() {
                        if face_edges(&body, f).contains(&e) {
                            adj.push(EntityKey::Face(f));
                        }
                    }
                    ('e', vec![at(a), at(b)], adj)
                }
                EntityKey::Face(f) => {
                    let mut adj: Vec<EntityKey> = face_edges(&body, f)
                        .into_iter()
                        .map(EntityKey::Edge)
                        .collect();
                    let vs = face_vertices(&body, f);
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
        assert!(
            out.insert(spelled.clone(), cands).is_none(),
            "two names of one table spell {spelled}"
        );
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

/// A test of one candidate against another.
type Fits<'a> = &'a dyn Fn(&Cand, &Cand) -> bool;

/// `a` and `b` matched one to one by `fits`.
fn one_to_one(a: &[Cand], b: &[Cand], fits: Fits<'_>) -> bool {
    fn from(i: usize, a: &[Cand], b: &[Cand], used: &mut [bool], fits: Fits<'_>) -> bool {
        let Some(x) = a.get(i) else {
            return true;
        };
        for j in 0..b.len() {
            if !used[j] && fits(x, &b[j]) {
                used[j] = true;
                if from(i + 1, a, b, used, fits) {
                    return true;
                }
                used[j] = false;
            }
        }
        false
    }
    a.len() == b.len() && from(0, a, b, &mut vec![false; b.len()], fits)
}

/// How two tables are compared.
#[derive(Clone, Copy)]
enum Compare {
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
fn differences(a: &Published, b: &Published, how: Compare) -> Vec<String> {
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
            Compare::Geometry => one_to_one(ca, cb, &|x, y| {
                x.kind == y.kind && same_points(&x.geo, &y.geo, 2)
            }),
            Compare::Incidence => one_to_one(ca, cb, &|x, y| x.kind == y.kind && x.near == y.near),
            Compare::Kept => one_to_one(ca, cb, &|x, y| {
                x.kind == y.kind
                    && x.near.is_subset(&y.near)
                    && y.near.difference(&x.near).all(|m| !ta.contains_key(m))
                    && (x.kind != 'v' || same_points(&x.geo, &y.geo, 2))
            }),
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

/// A case that moves today: its name, its exact count of differing
/// names, and the row that owns it.
type Admitted = (&'static str, usize, &'static str);

/// A case that compares no table today, and the row whose fix makes it
/// compare one.
type Vacuous = (&'static str, &'static str);

/// **What one test's comparisons found**, case by case.
#[derive(Default)]
struct Found {
    /// Each moving case's differences.
    diffs: BTreeMap<String, Vec<String>>,
    /// Each case: whether any of its comparisons had a table on one
    /// side, and what its first comparison set side by side.
    seen: BTreeMap<String, (bool, String)>,
}

impl Found {
    /// Compares `a` with `b` under `how` as part of `case`, each
    /// difference tagged `tag`.
    fn compare(&mut self, case: &str, tag: &str, a: &Published, b: &Published, how: Compare) {
        let seen = self
            .seen
            .entry(case.to_string())
            .or_insert_with(|| (false, format!("{} vs {}", outline(a), outline(b))));
        seen.0 |= a.is_table() || b.is_table();
        let diffs = differences(a, b, how);
        if !diffs.is_empty() {
            self.diffs
                .entry(case.to_string())
                .or_default()
                .extend(diffs.into_iter().map(|d| {
                    if tag.is_empty() {
                        d
                    } else {
                        format!("{tag}: {d}")
                    }
                }));
        }
    }

    /// **The test settles**: every cited row exists; the cases that
    /// move are exactly the admitted ones, each by its admitted count;
    /// and the cases that compare no table are exactly the vacuous
    /// ones.
    fn settle(&self, admitted: &[Admitted], vacuous: &[Vacuous]) {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        for (case, row) in admitted
            .iter()
            .map(|(c, _, r)| (c, r))
            .chain(vacuous.iter().map(|(c, r)| (c, r)))
        {
            assert!(root.join(row).is_file(), "{case}: its row {row} is no file");
        }
        let mut wrong = Vec::new();
        let counts: BTreeMap<&str, usize> = admitted.iter().map(|&(c, n, _)| (c, n)).collect();
        for (c, d) in &self.diffs {
            match counts.get(c.as_str()) {
                Some(&n) if n == d.len() => {}
                Some(&n) => wrong.push(format!(
                    "{c}: admitted moving {n} names, moves {}: first {:#?}",
                    d.len(),
                    &d[..d.len().min(3)]
                )),
                None => wrong.push(format!(
                    "{c}: moves {} names unadmitted: first {:#?}",
                    d.len(),
                    &d[..d.len().min(3)]
                )),
            }
        }
        for &(c, _, r) in admitted {
            if !self.diffs.contains_key(c) {
                wrong.push(format!("{c}: admitted on {r}, moves nothing"));
            }
        }
        let listed: BTreeSet<&str> = vacuous.iter().map(|&(c, _)| c).collect();
        for (c, (tables, first)) in &self.seen {
            match (tables, listed.contains(c.as_str())) {
                (false, false) => wrong.push(format!("{c}: compares no table ({first})")),
                (true, true) => wrong.push(format!("{c}: listed vacuous, compares a table")),
                _ => {}
            }
        }
        for c in listed.iter().filter(|c| !self.seen.contains_key(**c)) {
            wrong.push(format!("{c}: listed vacuous, is no case"));
        }
        assert!(wrong.is_empty(), "{wrong:#?}");
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

/// A prism over `corners` on its own xy, extruded `d` along its own z;
/// with `on`, its copy placed against that body on the frame at
/// `origin` spanned by `u` and `v`.
fn framed_prism(
    doc: ProfileDoc,
    on: Option<RecipeNodeId>,
    origin: [f64; 3],
    u: [f64; 3],
    v: [f64; 3],
    corners: &[(f64, f64)],
    d: f64,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, p) = insert(doc, Node::Profile(desc(vec![corners.to_vec()])));
    let (doc, prism) = insert(
        doc,
        Node::Extrude {
            profile: p.into(),
            distance: len(d),
        },
    );
    match on {
        None => {
            crate::fixture::own_frame(origin, u, v);
            (doc, prism)
        }
        Some(root) => against(doc, root, prism, &axes_at(origin, u, v)),
    }
}

const X: [f64; 3] = [1.0, 0.0, 0.0];
const Y: [f64; 3] = [0.0, 1.0, 0.0];
const Z: [f64; 3] = [0.0, 0.0, 1.0];

/// Two prisms whose coincident faces at `z = 1` and `y = 0` meet along
/// one line, a chord between two merged faces
/// (`work/wire/a-chord-between-two-merged-faces-within-no-rim-refuses.md`,
/// witness 1, which no other test builds).
fn two_prisms(doc: ProfileDoc) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let ell = [
        (0.0, -1.0),
        (2.0, -1.0),
        (2.0, 0.0),
        (4.0, 0.0),
        (4.0, 2.0),
        (0.0, 2.0),
    ];
    let (doc, a) = framed_prism(doc, None, [0.0; 3], X, Y, &ell, 1.0);
    let step = [
        (0.0, -1.0),
        (3.0, -1.0),
        (3.0, 1.0),
        (1.0, 1.0),
        (1.0, 2.0),
        (0.0, 2.0),
    ];
    let (doc, b) = framed_prism(doc, Some(a), [1.0, 0.0, 0.0], Y, Z, &step, 2.0);
    (doc, vec![a, b])
}

/// [`decide_alike::kiss`], its blocks listed.
fn kiss(doc: ProfileDoc) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let (doc, m) = decide_alike::kiss(doc);
    (doc, m.to_vec())
}

/// **Which of a fixture's intersects the corpus compares**: those of
/// members that overlap, not just touch. An intersect it leaves out is
/// asserted empty, so a fixture that comes to overlap is red.
#[derive(Clone, Copy, PartialEq)]
enum Meets {
    /// Its members touch: the n-ary intersect is empty, and so is each
    /// member's meet with the union of the others.
    Touches,
    /// No point lies in every member, so the n-ary intersect is empty;
    /// each member overlaps the union of the others, and that pair
    /// intersect is compared.
    Pairwise,
    /// Every member overlaps every other: both intersects are compared.
    Overlaps,
}

impl Meets {
    /// Whether the n-ary intersect is compared.
    fn nary(self) -> bool {
        self == Self::Overlaps
    }

    /// Whether member `x`'s meet with the union of the rest is compared.
    fn pair(self, what: &str, x: usize) -> bool {
        self != Self::Touches && !EMPTY_PAIRS.contains(&(what, x))
    }
}

/// The pairs of a [`Meets::Pairwise`] fixture whose member does not
/// meet the union of the others.
const EMPTY_PAIRS: &[(&str, usize)] = &[
    ("crossing one wedge, prisms", 1),
    ("flush onto an edge", 2),
    ("pinch and far", 2),
];

/// The n-ary corpus: members whose unions and intersections meet in
/// contact, by name.
const MEMBERS: &[(&str, Members, Meets)] = &[
    (
        "three in a row",
        decide_alike::three_in_a_row,
        Meets::Touches,
    ),
    ("stepped stack", decide_alike::stepped_stack, Meets::Touches),
    ("table", decide_alike::table, Meets::Touches),
    (
        "prism and cube",
        decide_alike::prism_and_cube,
        Meets::Overlaps,
    ),
    (
        "pinch and far",
        decide_alike::pinch_and_far,
        Meets::Pairwise,
    ),
    (
        "pinch and cap",
        decide_alike::pinch_and_cap,
        Meets::Overlaps,
    ),
    ("kiss", kiss, Meets::Touches),
    ("kiss and far", decide_alike::kiss_and_far, Meets::Touches),
    (
        "flush onto an edge",
        decide_alike::flush_onto_an_edge,
        Meets::Pairwise,
    ),
    ("plate pinch", pinch_order::pinch, Meets::Pairwise),
    ("through", pinch_order::through, Meets::Pairwise),
    ("two pinches", pinch_order::two_pinches, Meets::Pairwise),
    ("side pinch", pinch_order::side_pinch, Meets::Pairwise),
    ("corner holes", pinch_order::corner_holes, Meets::Pairwise),
    (
        "notch and hole",
        pinch_order::notch_and_hole,
        Meets::Pairwise,
    ),
    ("reflex hole", pinch_order::reflex_hole, Meets::Pairwise),
    ("two tilted wedges", two_tilted_wedges, Meets::Overlaps),
    ("three tilted wedges", three_tilted_wedges, Meets::Overlaps),
    (
        "three on a line",
        pinch_order::three_on_a_line,
        Meets::Pairwise,
    ),
    (
        "crossing one wedge",
        pinch_order::crossing_one_wedge,
        Meets::Pairwise,
    ),
    (
        "crossing one wedge, prisms",
        pinch_order::crossing_one_wedge_prisms,
        Meets::Pairwise,
    ),
    ("bridging", pinch_order::bridging, Meets::Pairwise),
    (
        "two bridges",
        pinch_order::two_bridges_short,
        Meets::Pairwise,
    ),
    ("two prisms", two_prisms, Meets::Overlaps),
];

/// The n-ary verbs, by name.
#[derive(Clone, Copy, PartialEq)]
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
/// unrelated node where `before`: what the op publishes.
fn nary(fixture: Members, op: Nary, order: &[usize], before: bool) -> Published {
    let mut ledger = Ledger::default();
    let mut doc = ProfileDoc::empty_derived("name-invariance", Tol::witness());
    if before {
        doc = insert(
            doc,
            Node::Profile(desc(vec![crate::fixture::square(0.0, 0.0, 0.5)])),
        )
        .0;
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
    let mut found = Found::default();
    for &(what, fixture, meets) in MEMBERS {
        let orders = decide_alike::orders(count(fixture));
        if !meets.nary() {
            let met = nary(fixture, Nary::Intersect, &orders[0], false);
            assert_eq!(met, Published::Empty, "{what}: its intersect is empty");
        }
        for op in Nary::BOTH {
            if op == Nary::Intersect && !meets.nary() {
                continue;
            }
            let case = format!("{what} / {}", op.word());
            let first = nary(fixture, op, &orders[0], false);
            for order in &orders[1..] {
                let got = nary(fixture, op, order, false);
                found.compare(
                    &case,
                    &format!("{order:?}"),
                    &first,
                    &got,
                    Compare::Geometry,
                );
            }
        }
    }
    for &(what, tool) in CUTS.iter().filter(|(_, t)| t.len() > 1) {
        found.compare(
            &format!("{what} / its tool's member order"),
            "",
            &sunk_boss_cut(tool, CutPose::default()),
            &sunk_boss_cut(
                tool,
                CutPose {
                    tool_reversed: true,
                    ..CutPose::default()
                },
            ),
            Compare::Geometry,
        );
    }
    found.settle(ADMITTED_MEMBER_ORDER, VACUOUS_MEMBER_ORDER);
}

/// The member-order cases that move today.
const ADMITTED_MEMBER_ORDER: &[Admitted] = &[("bridging / union", 6, BRIDGE_FOLDED_LAST)];

/// The member-order cases that compare no table today.
const VACUOUS_MEMBER_ORDER: &[Vacuous] = &[];

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
    let mut found = Found::default();
    for &(what, fixture, meets) in MEMBERS {
        let n = count(fixture);
        for x in 0..if n == 2 { 1 } else { n } {
            if !meets.pair(what, x) {
                let met = pair(fixture, Nary::Intersect, x, false);
                assert_eq!(met, Published::Empty, "{what}: m{x} meets the rest nowhere");
            }
            for op in Nary::BOTH {
                if op == Nary::Intersect && !meets.pair(what, x) {
                    continue;
                }
                found.compare(
                    &format!("{what} / {} / m{x} against the rest", op.word()),
                    "",
                    &pair(fixture, op, x, false),
                    &pair(fixture, op, x, true),
                    Compare::Geometry,
                );
            }
        }
    }
    for &(what, tool) in CUTS {
        let first = sunk_boss_cut(tool, CutPose::default());
        found.compare(
            &format!("{what} / the boss listed first"),
            "",
            &first,
            &sunk_boss_cut(
                tool,
                CutPose {
                    joined_swapped: true,
                    ..CutPose::default()
                },
            ),
            Compare::Geometry,
        );
    }
    found.settle(ADMITTED_OPERAND_ORDER, VACUOUS_OPERAND_ORDER);
}

/// The operand-order cases that move today.
const ADMITTED_OPERAND_ORDER: &[Admitted] = &[];

/// The operand-order cases that compare no table today.
const VACUOUS_OPERAND_ORDER: &[Vacuous] = &[
    (
        "bridging / intersect / m3 against the rest",
        pinch_order::TWO_GERMS,
    ),
    (
        "bridging / union / m3 against the rest",
        pinch_order::TWO_GERMS,
    ),
];

/// The bridging prism folded last, after the plate and both wedges,
/// refuses the union's fold step.
const BRIDGE_FOLDED_LAST: &str =
    "work/wirerest/a-union-folding-a-bridging-prism-last-refuses-its-fold-step.md";

// ------------------------------------------------------------------ //
// Merged chords
// ------------------------------------------------------------------ //

/// The cuts of `wire_legal_union_refusals`'s sunk boss whose chords
/// cross or run along the line where its two glued wall pieces meet:
/// each tool, by name.
const CUTS: &[(&str, &[Spans])] = &[
    (
        "a chord across the glued line",
        &[sunk::SLAB, sunk::STEP_ACROSS],
    ),
    (
        "a chord across it the other way",
        &[sunk::SLAB, sunk::STEP_THE_OTHER_WAY],
    ),
    ("a chord along the glued line", &[sunk::ALONG]),
    ("two chords of one seam", &[sunk::SLAB]),
];

/// How a sunk-boss cut is perturbed.
#[derive(Clone, Copy, Default)]
struct CutPose {
    /// The tool union's members listed in reverse.
    tool_reversed: bool,
    /// The plate and the boss listed boss first.
    joined_swapped: bool,
    /// A far block appended to the tool.
    far: bool,
}

/// [`sunk::plate_and_boss`] joined, less the union of `tool`'s
/// [`sunk::tool_block`]s (`wire_legal_union_refusals`'s `SunkBossCut`),
/// posed by `pose`.
fn sunk_boss_cut(tool: &[Spans], pose: CutPose) -> Published {
    let mut ledger = Ledger::default();
    let doc = ProfileDoc::empty_derived("name-invariance", Tol::witness());
    let (doc, [plate, boss]) = ledger.authored(doc, "plate and boss", sunk::plate_and_boss);
    let (mut doc, mut blocks) = ledger.authored(doc, "tool", |mut d| {
        let mut blocks = Vec::new();
        for &spans in tool {
            let (e, b) = sunk::tool_block(d, plate, spans);
            d = e;
            blocks.push(b);
        }
        (d, blocks)
    });
    if pose.far {
        let (d, b) = ledger.authored(doc, "far", |d| {
            sunk::tool_block(d, plate, ((9.0, 10.0), (9.0, 10.0), 0.0, 1.0))
        });
        doc = d;
        blocks.push(b);
    }
    if pose.tool_reversed {
        blocks.reverse();
    }
    let (doc, tool) = ledger.authored(doc, "tool union", |d| match blocks.as_slice() {
        [one] => (d, *one),
        several => insert(d, Nary::Union.node(several)),
    });
    let joined = if pose.joined_swapped {
        [boss, plate]
    } else {
        [plate, boss]
    };
    let (doc, joined) = ledger.authored(doc, "joined", |d| insert(d, Nary::Union.node(&joined)));
    let (doc, cut) = ledger.authored(doc, "cut", |d| {
        insert(
            d,
            Node::Subtract {
                from: joined.into(),
                tool: tool.into(),
            },
        )
    });
    published(&doc, cut, 0, &ledger)
}

// ------------------------------------------------------------------ //
// Splits
// ------------------------------------------------------------------ //

/// A split's target: an outline drawn on its own `xy`, extruded 1
/// along `z` or revolved a full turn about `y`, or a union of the
/// n-ary corpus.
#[derive(Clone, Copy)]
enum Target {
    Prism(&'static [(f64, f64)]),
    /// [`Target::Prism`], its copy standing in the world spun `−turn/4`
    /// about `y`: the outline's `x` along `z`, so the prism runs along
    /// `-x`.
    PrismAlongZ(&'static [(f64, f64)]),
    Turned(&'static [(f64, f64)]),
    Union(Members),
}

/// `target` authored: the document, its outline's profile where it has
/// one, and the node split.
fn target(doc: ProfileDoc, t: Target) -> (ProfileDoc, (Option<RecipeNodeId>, RecipeNodeId)) {
    match t {
        Target::Prism(outline) | Target::PrismAlongZ(outline) | Target::Turned(outline) => {
            let (doc, p) = insert(doc, Node::Profile(desc(vec![outline.to_vec()])));
            let node = if matches!(t, Target::Prism(_) | Target::PrismAlongZ(_)) {
                Node::Extrude {
                    profile: p.into(),
                    distance: len(1.0),
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
            let (doc, s) = if matches!(t, Target::PrismAlongZ(_)) {
                let quarter = Formula::turn_over(4).expect("a quarter turn");
                let back = Formula::neg(quarter).expect("an angle negates");
                in_world_by(doc, s, turned_to([0.0; 3], &[(TargetAxis::Y, back)]))
            } else {
                (doc, s)
            };
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
    (
        "prism and cube through the pinch",
        Target::Union(decide_alike::prism_and_cube),
        [1.0, 0.0, 0.0],
        X,
    ),
];

/// A split's tool, in the world's coordinates.
#[derive(Clone, Copy)]
enum Tool {
    /// The plane through `origin` with `normal`.
    Plane([f64; 3], [f64; 3]),
    /// A frame's xy plane: the frame through `origin` spanned by `u`
    /// and `v`.
    FrameXy([f64; 3], [f64; 3], [f64; 3]),
}

impl Tool {
    /// The tool read off `t`'s node `node`: a plane off a face it is
    /// parallel to where the target has one ([`crate::fixture::plane_on`],
    /// exact), every other plane and a frame's xy off a frame on the
    /// target. A [`Target::PrismAlongZ`]'s tool is given in its prism's
    /// own coordinates, the world's turned back.
    fn read(self, doc: &ProfileDoc, t: Target, node: RecipeNodeId) -> RoleRead<Operand> {
        let own = |p: [f64; 3]| match t {
            Target::PrismAlongZ(_) => [p[2], p[1], -p[0]],
            _ => p,
        };
        let body = Operand::output(node, 0);
        let (origin, u, v) = match self {
            Self::Plane(origin, normal) => {
                let (origin, normal) = (own(origin), own(normal));
                let axis = normal.iter().filter(|c| **c != 0.0).count() == 1;
                if axis && !(matches!(t, Target::Turned(_)) && normal[1] == 0.0) {
                    return crate::fixture::plane_on(doc, body, origin, normal);
                }
                return crate::fixture::plane_on_frame(doc, body, origin, normal);
            }
            Self::FrameXy(origin, u, v) => (own(origin), own(u), own(v)),
        };
        let frame =
            editor_core::test_support::frame_on(doc, &body, &axes_at(origin, u, v), Tol::witness());
        RoleRead::through(frame, PoseRole::Xy)
    }
}

/// One split: both halves' tables, and their bodies' face, edge and
/// vertex counts where both halves hold a body.
struct Halves {
    tables: [Published; 2],
    counts: Option<[usize; 3]>,
}

/// `t` split by `tool`, `edit` applied to the document first.
fn split(
    t: Target,
    tool: Tool,
    edit: &dyn Fn(ProfileDoc, Option<RecipeNodeId>) -> ProfileDoc,
) -> Halves {
    let mut ledger = Ledger::default();
    let doc = ProfileDoc::empty_derived("name-invariance", Tol::witness());
    let (doc, (profile, node)) = ledger.authored(doc, "target", |d| target(d, t));
    let tool = tool.read(&doc, t, node);
    let (doc, sp) = ledger.authored(doc, "split", |d| {
        insert(
            d,
            Node::Split {
                target: Operand::output(node, 0).into(),
                tool,
            },
        )
    });
    ledger.alone(sp, "split node");
    let doc = edit(doc, profile);
    let ev = run(&doc);
    let tables = [0, 1].map(|port| published_in(&ev, &doc, sp, port, &ledger));
    let counts = (|| {
        let mut c = [0; 3];
        for port in 0..2 {
            let var = ev.output_var(sp, port)?;
            let body = ev.read_body(var, 0).ok()?.ok()?;
            c[0] += body.body().faces().count();
            c[1] += body.body().edges().count();
            c[2] += body.body().vertices().count();
        }
        Some(c)
    })();
    Halves { tables, counts }
}

fn unedited(doc: ProfileDoc, _: Option<RecipeNodeId>) -> ProfileDoc {
    doc
}

/// A unit in-plane direction of the plane with unit `normal`.
fn along(normal: [f64; 3]) -> [f64; 3] {
    let a = if normal[2].abs() < 0.9 {
        [0.37 * normal[1], -0.37 * normal[0], 0.21]
    } else {
        [0.37, 0.0, 0.0]
    };
    let dot = a[0] * normal[0] + a[1] * normal[1] + a[2] * normal[2];
    unit([0, 1, 2].map(|i| a[i] - dot * normal[i]))
}

fn unit(m: [f64; 3]) -> [f64; 3] {
    let n = m.iter().map(|c| c * c).sum::<f64>().sqrt();
    m.map(|c| c / n)
}

fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

/// The frame on the plane through `origin` with unit `normal` whose x
/// axis is [`along`] it turned `deg` about the normal.
fn spun(origin: [f64; 3], normal: [f64; 3], deg: f64) -> Tool {
    let u = turned(along(normal), normal, deg);
    Tool::FrameXy(origin, u, cross(normal, u))
}

/// **3. A split plane's frame**: flipping its normal swaps its halves
/// and renames nothing in them; and moving its origin along it, or
/// spinning it about the normal, renames nothing.
///
/// A plane read off a face is the face's plane slid along its normal:
/// it has no origin to move and no in-plane axis to spin, so the move
/// and the spin are asked of a frame's xy plane, read off a frame on
/// the target whose origin and axes are authored.
#[test]
fn a_split_planes_frame_moves_no_name() {
    let mut found = Found::default();
    for &(what, t, o, n) in SPLITS {
        let first = split(t, Tool::Plane(o, n), &unedited).tables;
        assert!(
            first.iter().all(Published::is_table),
            "{what}: the split names both halves: {first:?}"
        );
        let flipped = split(t, Tool::Plane(o, n.map(|c| -c)), &unedited).tables;
        let a = along(n);
        let framed = split(t, spun(o, n, 0.0), &unedited).tables;
        let moved = split(
            t,
            spun([0, 1, 2].map(|i| o[i] + 0.4 * a[i]), n, 0.0),
            &unedited,
        )
        .tables;
        let spun = split(t, spun(o, n, 37.0), &unedited).tables;
        for port in 0..2 {
            let half = format!("half {port}");
            found.compare(
                &format!("{what} / flipped"),
                &half,
                &first[port],
                &flipped[1 - port],
                Compare::Geometry,
            );
            found.compare(
                &format!("{what} / moved along"),
                &half,
                &framed[port],
                &moved[port],
                Compare::Geometry,
            );
            found.compare(
                &format!("{what} / a frame's plane spun"),
                &half,
                &framed[port],
                &spun[port],
                Compare::Geometry,
            );
        }
    }
    found.settle(ADMITTED_SPLIT_FRAME, VACUOUS_SPLIT_FRAME);
}

/// The split-frame cases that move today.
const ADMITTED_SPLIT_FRAME: &[Admitted] = &[];

/// The split-frame cases that compare no table today.
const VACUOUS_SPLIT_FRAME: &[Vacuous] = &[];

// ------------------------------------------------------------------ //
// Tilts
// ------------------------------------------------------------------ //

/// `v` turned by `deg` degrees about `axis` (a unit vector).
fn turned(v: [f64; 3], axis: [f64; 3], deg: f64) -> [f64; 3] {
    let (s, c) = deg.to_radians().sin_cos();
    let dot = v[0] * axis[0] + v[1] * axis[1] + v[2] * axis[2];
    let cross = cross(axis, v);
    [0, 1, 2].map(|i| v[i] * c + cross[i] * s + axis[i] * dot * (1.0 - c))
}

/// [`decide_alike::prism_and_cube`] with the cube's normal turned `deg`
/// about `x`, its bottom face still through the L's reflex corner, so
/// the pinch stays.
fn prism_and_tilted_cube(doc: ProfileDoc, deg: f64) -> (ProfileDoc, Vec<RecipeNodeId>) {
    decide_alike::prism_and_cube_along(doc, turned(unit(decide_alike::CUBE_NORMAL), X, deg))
}

/// [`pinch_order::pinch`] with `p2`'s footprint turned `deg` about the
/// vertical line through its corner at the pinch, (1.5, 1).
fn pinch_turned(doc: ProfileDoc, deg: f64) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let (doc, plate) = block_of(doc, pinch_order::PLATE_SPANS);
    let (doc, p1) = block_on_of(doc, plate, pinch_order::P1);
    let ((x0, x1), (y0, y1), z0, dz) = pinch_order::P2;
    let (w, h) = (x1 - x0, y1 - y0);
    let foot = [(0.0, 0.0), (w, 0.0), (w, h), (0.0, h)];
    let (doc, p2) = framed_prism(
        doc,
        Some(plate),
        [x0, y0, z0],
        turned(X, Z, deg),
        turned(Y, Z, deg),
        &foot,
        dz,
    );
    (doc, vec![plate, p1, p2])
}

/// [`pinch_order::crossing_one_wedge`] with the crossing prism's
/// sector turned `deg` about the contact line.
fn crossing_turned(doc: ProfileDoc, deg: f64) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let (doc, plate) = block_of(doc, pinch_order::PLATE_SPANS);
    let (doc, a) = pinch_order::sector_of(doc, plate, pinch_order::WEDGE_A);
    let (doc, b) = pinch_order::sector_of(doc, plate, pinch_order::WEDGE_B);
    let (a0, a1, z) = pinch_order::CROSSING_A;
    let (doc, c) = pinch_order::sector_of(doc, plate, (a0 + deg, a1 + deg, z));
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
const TILTS: [f64; 4] = [-0.3, 0.2, 0.6, 1.0];

/// How many of [`TILTS`] each subject must compare at.
const COMPARED_TILTS: usize = 2;

/// The union of `posed` at `deg`: its table, and its body's counts.
fn posed_union(posed: Posed, deg: f64) -> (Published, Option<[usize; 3]>) {
    let mut ledger = Ledger::default();
    let doc = ProfileDoc::empty_derived("name-invariance", Tol::witness());
    let (doc, m) = ledger.authored(doc, "fixture", |d| posed(d, deg));
    let (doc, u) = ledger.authored(doc, "op", |d| insert(d, Nary::Union.node(&m)));
    let ev = run(&doc);
    let counts = ev.value(u).and_then(|v| match &v.payload {
        ValuePayload::Boolean(BooleanValue::Body { body, .. }) => Some([
            body.body().faces().count(),
            body.body().edges().count(),
            body.body().vertices().count(),
        ]),
        _ => None,
    });
    (published_in(&ev, &doc, u, 0, &ledger), counts)
}

/// The pentagon's split by the vertical plane through its apex edge,
/// turned `deg` about that edge.
fn pentagon_split_turned(deg: f64) -> Halves {
    split(
        Target::Prism(crate::split_pinch_keeps::PENTAGON),
        Tool::Plane([1.0, 1.0, 0.0], turned(Y, Z, deg)),
        &unedited,
    )
}

/// The notched block, drawn along `z`, split by the plane `y = 1.5`,
/// which cuts three sections from each half, turned `deg` about `z`.
fn notched_split_turned(deg: f64) -> Halves {
    split(
        Target::PrismAlongZ(crate::split_pinch_keeps::NOTCHED),
        Tool::Plane([0.0, 1.5, 0.0], turned(Y, Z, deg)),
        &unedited,
    )
}

/// A split whose plane is posed by a tilt, by name.
type TurnedSplit = fn(f64) -> Halves;

const TURNED_SPLITS: &[(&str, TurnedSplit)] = &[
    ("notched pentagon, plane turned", pentagon_split_turned),
    (
        "notched block's three sections, plane turned",
        notched_split_turned,
    ),
];

/// Compares `subject`'s poses at [`TILTS`] with its pose at 0°, each
/// as `pose` builds it: its tables and its bodies' counts. A pose whose
/// bodies' counts differ is another topology, and is not compared but
/// printed; a pose that holds no body (it refuses, or is empty) is
/// compared, so it is red unless admitted. Returns how many poses were
/// compared.
fn tilt(
    found: &mut Found,
    subject: &str,
    pose: &dyn Fn(f64) -> (Vec<Published>, Option<[usize; 3]>),
) -> usize {
    let (first, counts) = pose(0.0);
    assert!(
        first.iter().all(Published::is_table) && counts.is_some(),
        "{subject} at 0°: {:?}",
        first.iter().map(outline).collect::<Vec<_>>()
    );
    let mut compared = 0;
    for deg in TILTS {
        let (got, c) = pose(deg);
        if c.is_some() && c != counts {
            println!("SKIPPED {subject} at {deg}°: its topology {c:?} is not {counts:?}");
            continue;
        }
        compared += 1;
        for (port, (a, b)) in first.iter().zip(&got).enumerate() {
            found.compare(
                subject,
                &format!("{deg}°, part {port}"),
                a,
                b,
                Compare::Incidence,
            );
        }
    }
    println!("{subject}: {compared} of {} tilts compared", TILTS.len());
    compared
}

/// **4. A tilt within one topology**: a member turned a little, or a
/// split plane turned about the edge it holds, renames nothing — each
/// name keeps its kind and its incidences. A pose whose topology
/// changes is not compared, and says so; every subject compares
/// [`COMPARED_TILTS`] poses at least.
#[test]
fn a_tilt_within_one_topology_moves_no_name() {
    let mut found = Found::default();
    for &(what, posed) in POSED {
        let compared = tilt(&mut found, what, &|deg| {
            let (t, c) = posed_union(posed, deg);
            (vec![t], c)
        });
        assert!(
            compared >= COMPARED_TILTS,
            "{what}: {compared} tilts keep the topology"
        );
    }
    for &(what, pose) in TURNED_SPLITS {
        let compared = tilt(&mut found, what, &|deg| {
            let h = pose(deg);
            (h.tables.to_vec(), h.counts)
        });
        assert!(
            compared >= COMPARED_TILTS,
            "{what}: {compared} tilts keep the topology"
        );
    }
    found.settle(ADMITTED_TILT, VACUOUS_TILT);
}

/// The tilt cases that move today.
const ADMITTED_TILT: &[Admitted] = &[];

/// The tilt cases that compare no table today.
const VACUOUS_TILT: &[Vacuous] = &[];

// ------------------------------------------------------------------ //
// Authoring histories
// ------------------------------------------------------------------ //

/// One member of a role-built fixture, its copy standing in the world,
/// so the members may be authored in any order.
#[derive(Clone, Copy)]
enum Part {
    /// A [`block`] over these spans, slid up to its base.
    Block(Spans),
    /// A [`pinch_order::sector_in_world`] this sector.
    Sector(Sector),
    /// The block of these spans, extruded down from its top: the same
    /// solid as [`Part::Block`]'s, its lateral edges running the other
    /// way. Drawn on its own xy with `y` reversed and turned a half turn
    /// about `x`, its top slid to `z0 + dz`.
    Downward(Spans),
    /// A [`seam_scene::disc`] extruded `dz`, slid up to `z0`.
    Disc(seam_scene::Disc, f64, f64),
}

impl Part {
    fn author(self, doc: ProfileDoc) -> (ProfileDoc, RecipeNodeId) {
        match self {
            Self::Block((x, y, z0, dz)) => {
                let (doc, b) = block(doc, x, y, dz);
                in_world_by(doc, b, turned_to([0.0, 0.0, z0], &[]))
            }
            Self::Sector(s) => pinch_order::sector_in_world(doc, s),
            Self::Disc(d, z0, dz) => {
                let (doc, b) = seam_scene::disc(doc, d, dz);
                in_world_by(doc, b, turned_to([0.0, 0.0, z0], &[]))
            }
            Self::Downward(((x0, x1), (y0, y1), z0, dz)) => {
                let (doc, p) = insert(
                    doc,
                    Node::Profile(desc(vec![vec![(x0, -y1), (x1, -y1), (x1, -y0), (x0, -y0)]])),
                );
                let (doc, b) = insert(
                    doc,
                    Node::Extrude {
                        profile: p.into(),
                        distance: len(dz),
                    },
                );
                let half = Formula::turn_over(2).expect("a half turn");
                in_world_by(
                    doc,
                    b,
                    turned_to([0.0, 0.0, z0 + dz], &[(TargetAxis::X, half)]),
                )
            }
        }
    }
}

/// What a role-built fixture's members are combined into.
#[derive(Clone, Copy)]
enum Combine {
    /// One n-ary op over every member, in role order.
    Nary(Nary),
    /// The first member less the union of the others.
    Cut,
}

/// A role-built fixture: its members, each by role, in the order the
/// op lists them.
type Roles = (&'static str, &'static [(&'static str, Part)], Combine);

/// Two twin blocks of one footprint size, touching along the vertical
/// line through (1.5, 1).
const TWIN_LEFT: Part = Part::Block(((1.0, 1.5), (0.5, 1.0), 0.5, 1.5));
const TWIN_RIGHT: Part = Part::Block(((1.5, 2.0), (1.0, 1.5), 0.5, 1.5));

/// A block, authored twice for a block and its exact copy.
const UNIT_BLOCK: Part = Part::Block(((1.0, 2.0), (0.5, 1.5), 0.5, 1.0));

const PLATE: Part = Part::Block(pinch_order::PLATE_SPANS);
const P1: Part = Part::Block(pinch_order::P1);
const P2: Part = Part::Block(pinch_order::P2);
const P2_DOWNWARD: Part = Part::Downward(pinch_order::P2);
const KISS_A: Part = Part::Block(decide_alike::KISS[0]);
const KISS_B: Part = Part::Block(decide_alike::KISS[1]);
const WEDGE_A: Part = Part::Sector(pinch_order::WEDGE_A);
const WEDGE_B: Part = Part::Sector(pinch_order::WEDGE_B);
const CROSSING_A: Part = Part::Sector(pinch_order::CROSSING_A);

/// `seam_scene`'s members: `x`, `y` under z = 0.5 and `w` slid down a
/// quarter, each standing in the world.
const fn seam_members(scene: &seam_scene::Scene) -> [(&'static str, Part); 3] {
    [
        ("x", Part::Disc(scene.x, 0.0, 1.0)),
        ("y", Part::Block((scene.y.0, scene.y.1, 0.0, 0.5))),
        ("w", Part::Disc(scene.w, -0.25, 1.0)),
    ]
}

const OPEN_ARC_SEAMS: [(&str, Part); 3] = seam_members(&seam_scene::OPEN_ARC);
const CUT_ARC_SEAMS: [(&str, Part); 3] = seam_members(&seam_scene::CUT_ARC);

/// Two blocks united as a tool and standing through the plate's top,
/// so each seam of their union crosses it: `x = 2` of the first meets
/// `y = 1` of the second, and `x = 1.5` of the second `y = 1.5` of the
/// first.
const TOOL_A: Part = Part::Block(((1.0, 2.0), (0.5, 1.5), 0.5, 1.0));
const TOOL_B: Part = Part::Block(((1.5, 2.5), (1.0, 1.75), 0.4, 1.0));

/// Contact-heavy fixtures by role: twins and exact copies, whose names
/// differ by their ids alone, and the corpus's pinches and crossings.
const HISTORIES: &[Roles] = &[
    (
        "twins",
        &[("left", TWIN_LEFT), ("right", TWIN_RIGHT)],
        Combine::Nary(Nary::Union),
    ),
    (
        "twins on the plate",
        &[("plate", PLATE), ("left", TWIN_LEFT), ("right", TWIN_RIGHT)],
        Combine::Nary(Nary::Union),
    ),
    (
        "the plate less the twins",
        &[("plate", PLATE), ("left", TWIN_LEFT), ("right", TWIN_RIGHT)],
        Combine::Cut,
    ),
    (
        "a block and its copy",
        &[("one", UNIT_BLOCK), ("copy", UNIT_BLOCK)],
        Combine::Nary(Nary::Union),
    ),
    (
        "a block and its copy, met",
        &[("one", UNIT_BLOCK), ("copy", UNIT_BLOCK)],
        Combine::Nary(Nary::Intersect),
    ),
    (
        "the plate less a block and its copy",
        &[("plate", PLATE), ("one", UNIT_BLOCK), ("copy", UNIT_BLOCK)],
        Combine::Cut,
    ),
    (
        "plate pinch",
        &[("plate", PLATE), ("p1", P1), ("p2", P2)],
        Combine::Nary(Nary::Union),
    ),
    (
        "plate pinch, p2 extruded down",
        &[("plate", PLATE), ("p1", P1), ("p2", P2_DOWNWARD)],
        Combine::Nary(Nary::Union),
    ),
    (
        "crossing one wedge",
        &[
            ("plate", PLATE),
            ("a", WEDGE_A),
            ("b", WEDGE_B),
            ("c", CROSSING_A),
        ],
        Combine::Nary(Nary::Union),
    ),
    (
        "kiss",
        &[("a", KISS_A), ("b", KISS_B)],
        Combine::Nary(Nary::Union),
    ),
    (
        "the plate less the pinch's blocks",
        &[("plate", PLATE), ("p1", P1), ("p2", P2)],
        Combine::Cut,
    ),
    (
        "seams crossing one open arc",
        &OPEN_ARC_SEAMS,
        Combine::Nary(Nary::Union),
    ),
    (
        "seams crossing an arc cut twice",
        &CUT_ARC_SEAMS,
        Combine::Nary(Nary::Union),
    ),
    (
        "the plate less a union whose seams cross its top",
        &[("plate", PLATE), ("a", TOOL_A), ("b", TOOL_B)],
        Combine::Cut,
    ),
];

/// `roles` authored in `authoring` order (indices into it), combined
/// in role order: what the combination publishes.
fn by_roles(roles: &Roles, authoring: &[usize]) -> Published {
    let (_, members, combine) = *roles;
    let mut ledger = Ledger::default();
    let mut doc = ProfileDoc::empty_derived("name-invariance", Tol::witness());
    let mut ids = vec![None; members.len()];
    for &i in authoring {
        let (role, part) = members[i];
        let (d, id) = ledger.authored(doc, role, |d| part.author(d));
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

/// **6. Authoring history**: one recipe authored in another order, or
/// after an unrelated node, publishes one table up to the bijection of
/// its minted ids. Where two members are separately authored and
/// structurally identical (twins, exact copies), their names differ by
/// their ids alone, so this is also which of them was authored first.
#[test]
fn an_authoring_history_moves_no_name() {
    let mut found = Found::default();
    for roles in HISTORIES {
        let n = roles.1.len();
        let forward: Vec<usize> = (0..n).collect();
        let first = by_roles(roles, &forward);
        let case = format!("{} / authored in another order", roles.0);
        for order in decide_alike::orders(n)
            .into_iter()
            .filter(|o| *o != forward)
        {
            found.compare(
                &case,
                &format!("{order:?}"),
                &first,
                &by_roles(roles, &order),
                Compare::Geometry,
            );
        }
    }
    for &(what, fixture, _) in MEMBERS {
        let order: Vec<usize> = (0..count(fixture)).collect();
        found.compare(
            &format!("{what} / after an unrelated node"),
            "",
            &nary(fixture, Nary::Union, &order, false),
            &nary(fixture, Nary::Union, &order, true),
            Compare::Geometry,
        );
    }
    found.settle(ADMITTED_HISTORY, VACUOUS_HISTORY);
}

/// **The raw table reads a seam crossing alike in every authoring
/// order**, with no `remap_name` between: the plate less the union of
/// [`TOOL_A`] and [`TOOL_B`], whose seam crosses the plate's top at
/// (2, 1, 1). The crossing names the side it enters along, and that side
/// is the same member's in every order. A sense read along the seam's
/// lesser side would flip with which member was authored first.
#[test]
fn a_seam_crossings_raw_sense_names_one_members_side_in_every_order() {
    use editor_core::{EntityKey, Entry, RoleSeg, Sense};
    let mut seen = BTreeSet::new();
    for order in decide_alike::orders(3) {
        let members = [PLATE, TOOL_A, TOOL_B];
        let mut doc = ProfileDoc::empty_derived("name-invariance", Tol::witness());
        let mut ids = [None; 3];
        for &i in &order {
            let (d, id) = members[i].author(doc);
            doc = d;
            ids[i] = Some(id);
        }
        let ids = ids.map(|i| i.expect("authored"));
        let (doc, tool) = insert(doc, Nary::Union.node(&ids[1..]));
        let (doc, op) = insert(
            doc,
            Node::Subtract {
                from: ids[0].into(),
                tool: tool.into(),
            },
        );
        let ev = run(&doc);
        assert!(
            failure(&ev, op).is_none(),
            "{order:?}: {:?}",
            failure(&ev, op)
        );
        let body = crate::corpus::body_of(&ev, op);
        let role = |read| {
            ["plate", "a", "b"]
                .into_iter()
                .zip(ids)
                .find(|&(_, id)| crate::fixture::out(&doc, id) == read)
                .map_or("another read", |(r, _)| r)
        };
        let mut at = Vec::new();
        for (name, entry) in crate::fixture::table(&ev, op).iter() {
            let Entry::Unique(r) = entry else { continue };
            let EntityKey::Vertex(v) = r.key else {
                continue;
            };
            let p = crate::fixture::point(&body, v);
            if (p.x - 2.0).abs() > 1e-9 || (p.y - 1.0).abs() > 1e-9 || (p.z - 1.0).abs() > 1e-9 {
                continue;
            }
            let [RoleSeg::Crossing { sense, .. }] = name.path.as_slice() else {
                panic!("{order:?}: the seam's crossing of the top: {name:?}");
            };
            at.push(match sense {
                Sense::Through { entering } => entering
                    .iter()
                    .map(|n| match n.path.as_slice() {
                        [RoleSeg::From { of, .. }] => match of.path.as_slice() {
                            [RoleSeg::From { read, .. }] => role(*read).to_owned(),
                            other => format!("{other:?}"),
                        },
                        other => format!("{other:?}"),
                    })
                    .collect::<Vec<_>>()
                    .join(", "),
                other => format!("{other:?}"),
            });
        }
        assert_eq!(at.len(), 1, "{order:?}: one vertex at the crossing: {at:?}");
        seen.insert(at.remove(0));
    }
    assert_eq!(
        seen.len(),
        1,
        "the raw sense reads alike in every authoring order: {seen:?}"
    );
}

/// **A body and its exact copy name every entity by both reads** (N0's
/// NS4: two operands with identical tables are told apart by their
/// reads, never ordered): their union is one body, each face, edge and
/// vertex the same entity in both, so each is `Merged` of two names
/// whose heads are `From` the two members' reads.
#[test]
fn a_body_and_its_copy_name_every_entity_by_both_reads() {
    let doc = ProfileDoc::empty_derived("name-invariance", Tol::witness());
    let (doc, one) = UNIT_BLOCK.author(doc);
    let (doc, copy) = UNIT_BLOCK.author(doc);
    let (doc, op) = insert(doc, Nary::Union.node(&[one, copy]));
    let ev = run(&doc);
    let reads = BTreeSet::from([
        crate::fixture::out(&doc, one),
        crate::fixture::out(&doc, copy),
    ]);
    let var = ev.output_var(op, 0).expect("the union defines its port");
    let table = ev.table(var).expect("the union's table");
    let mut named = 0;
    for (name, _) in table.iter() {
        if name.kind == editor_core::EntityKind::Body {
            continue;
        }
        let cited: BTreeSet<VarId> = match name.path.as_slice() {
            [editor_core::RoleSeg::Merged(set)] => set
                .iter()
                .filter_map(|c| match c.path.as_slice() {
                    [editor_core::RoleSeg::From { read, .. }] => Some(*read),
                    _ => None,
                })
                .collect(),
            _ => BTreeSet::new(),
        };
        assert_eq!(cited, reads, "{name:?} is not named for both copies");
        named += 1;
    }
    assert_eq!(named, 26, "a block's 6 faces, 12 edges and 8 vertices");
}

/// The history cases that move today.
const ADMITTED_HISTORY: &[Admitted] = &[];

/// The history cases that compare no table today.
const VACUOUS_HISTORY: &[Vacuous] = &[
    ("bridging / after an unrelated node", BRIDGE_FOLDED_LAST),
    (
        "the plate less the pinch's blocks / authored in another order",
        LINE_WEDGES,
    ),
    (
        "the plate less the twins / authored in another order",
        LINE_WEDGES,
    ),
];

/// A plate less blocks that meet on a line through it refuses
/// `LineWedges`: the contact is two coincident edges whose material
/// wedges overlap.
const LINE_WEDGES: &str =
    "work/topo/revert-re-pairs-the-sheets-on-a-line-where-coincident-edges-meet.md";

// ------------------------------------------------------------------ //
// Far features
// ------------------------------------------------------------------ //

/// `fixture`'s union, with a far block as a last member where `far`.
fn with_far_member(fixture: Members, far: bool) -> Published {
    let mut ledger = Ledger::default();
    let doc = ProfileDoc::empty_derived("name-invariance", Tol::witness());
    let (doc, mut m) = ledger.authored(doc, "fixture", fixture);
    let doc = if far {
        let root = m[0];
        let (doc, b) = ledger.authored(doc, "far", |d| {
            block_on(d, root, (9.0, 10.0), (9.0, 10.0), 0.0, 1.0)
        });
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

/// **5. A far feature**: a far block added to a union or to a cut's
/// tool, or a hole added to a split outline away from its plane, leaves
/// every other name where it was — its kind, its incidences (gaining
/// only the feature's), and its vertices' points.
#[test]
fn a_far_feature_moves_no_other_name() {
    let mut found = Found::default();
    for &(what, fixture, _) in MEMBERS {
        found.compare(
            &format!("{what} / a far member"),
            "",
            &with_far_member(fixture, false),
            &with_far_member(fixture, true),
            Compare::Kept,
        );
    }
    for &(what, t, o, n) in SPLITS {
        let (Target::Prism(outline) | Target::Turned(outline)) = t else {
            continue;
        };
        let first = split(t, Tool::Plane(o, n), &unedited).tables;
        let holed = split(t, Tool::Plane(o, n), &|doc, profile| {
            add_hole(
                doc,
                profile.expect("an outline's profile"),
                outline,
                far_hole(t),
            )
        })
        .tables;
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
            found.compare(
                &format!("{what} / a far hole"),
                &format!("half {port}"),
                &first[port],
                &holed[port],
                Compare::Kept,
            );
        }
    }
    // A one-block tool is the block itself; a far block would make it a
    // union, whose names cite the union.
    for &(what, tool) in CUTS.iter().filter(|(_, t)| t.len() > 1) {
        found.compare(
            &format!("{what} / a far block in the tool"),
            "",
            &sunk_boss_cut(tool, CutPose::default()),
            &sunk_boss_cut(
                tool,
                CutPose {
                    far: true,
                    ..CutPose::default()
                },
            ),
            Compare::Kept,
        );
    }
    found.settle(ADMITTED_FAR, VACUOUS_FAR);
}

/// The far-feature cases that move today.
const ADMITTED_FAR: &[Admitted] = &[];

/// The far-feature cases that compare no table today.
const VACUOUS_FAR: &[Vacuous] = &[("bridging / a far member", BRIDGE_FOLDED_LAST)];

/// Review probe (PR 5981): [`decide_alike::prism_and_cube`] with its
/// members listed the other way round.
fn cube_and_prism(doc: ProfileDoc) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let (doc, mut m) = decide_alike::prism_and_cube(doc);
    m.reverse();
    (doc, m)
}

/// Review probe (PR 5981): **a split through a union's pinch names
/// its halves alike in either member order.** The fused vertex is the
/// one standing for the smaller operand vertex; red if that choice
/// reads the union's arena order, which member order moves.
#[test]
fn review_a_pinch_split_names_alike_in_either_member_order() {
    let mut found = Found::default();
    for n in [X, X.map(|c| -c)] {
        let a = split(
            Target::Union(decide_alike::prism_and_cube),
            Tool::Plane([1.0, 0.0, 0.0], n),
            &unedited,
        )
        .tables;
        let b = split(
            Target::Union(cube_and_prism),
            Tool::Plane([1.0, 0.0, 0.0], n),
            &unedited,
        )
        .tables;
        assert!(
            a.iter().chain(&b).all(Published::is_table),
            "{n:?}: {a:?} {b:?}"
        );
        for port in 0..2 {
            found.compare(
                &format!("pinch split {n:?} / member order"),
                &format!("half {port}"),
                &a[port],
                &b[port],
                Compare::Geometry,
            );
        }
    }
    found.settle(&[], &[]);
}
