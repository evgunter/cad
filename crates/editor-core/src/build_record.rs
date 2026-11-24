//! MEASUREMENT INSTRUMENT of the BOXFINISH build-record probe. Lives on
//! the never-merged branch `boxfinish/probe-build-record`; nothing here
//! is for main.
//!
//! The candidate RECORD OF A BUILD: per node its standing; per body it
//! outputs, its cells by name and kind, which named cells bound which
//! (face, loop, edge, vertex; shells; lumps), and the node's recorded
//! coincidences. Two builds are "the same" when these agree.
//!
//! Beside it, the EXTRAS: what the record leaves out that is cheap to
//! read off the same walk (carrier kinds, face senses, what a body is
//! held as, a coincidence's discharge and margin), so a pair of builds
//! equal in the record can be asked whether a reader would tell them
//! apart.
//!
//! Switched by `BUILD_RECORD_PROBE_DIR` (leaf, drive and zero-width
//! lines, one JSONL file per process) and `BUILD_RECORD_PARITY=1` (the
//! f64-against-Interval replay of every top-level `f64` evaluation).

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::hash::{Hash, Hasher};
use std::io::Write as _;
use std::sync::{Mutex, OnceLock};

use geom_core::Decide;
use geom_core::k_stats::{Role, Verdict};
use serde_json::{Value, json};

use crate::eval::{BooleanValue, Evaluation, NodeResult, NodeValue, SplitSide, ValuePayload};
use crate::names::{EntityKey, EntityRef, NameTable};
use crate::node::RecipeNodeId;

/// The directory lines are written to, when the probe is on.
pub(crate) fn dir() -> Option<&'static std::path::Path> {
    static DIR: OnceLock<Option<std::path::PathBuf>> = OnceLock::new();
    DIR.get_or_init(|| std::env::var_os("BUILD_RECORD_PROBE_DIR").map(Into::into))
        .as_deref()
}

/// Whether the parity hook is on.
pub(crate) fn parity_on() -> bool {
    static ON: OnceLock<bool> = OnceLock::new();
    *ON.get_or_init(|| dir().is_some() && std::env::var_os("BUILD_RECORD_PARITY").is_some())
}

/// Appends one line to this process's file.
pub(crate) fn emit(mut line: Value) {
    static FILE: OnceLock<Option<Mutex<std::fs::File>>> = OnceLock::new();
    let Some(dir) = dir() else { return };
    let file = FILE.get_or_init(|| {
        let _ = std::fs::create_dir_all(dir);
        let path = dir.join(format!("{}.jsonl", std::process::id()));
        let mut f = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .ok()?;
        let args: Vec<String> = std::env::args().collect();
        let _ = writeln!(f, "{}", json!({"k": "proc", "args": args}));
        Some(Mutex::new(f))
    });
    if let Some(f) = file {
        if let Some(o) = line.as_object_mut() {
            o.insert(
                "thread".into(),
                json!(std::thread::current().name().unwrap_or("")),
            );
        }
        if let Ok(mut f) = f.lock() {
            let _ = writeln!(f, "{line}");
        }
    }
}

/// The first identifier of a value's `Debug`: an enum's variant.
fn head<D: core::fmt::Debug>(d: &D) -> String {
    struct Head(String);
    impl core::fmt::Write for Head {
        fn write_str(&mut self, s: &str) -> core::fmt::Result {
            for c in s.chars() {
                if c.is_alphanumeric() || c == '_' {
                    self.0.push(c);
                } else {
                    return Err(core::fmt::Error);
                }
            }
            Ok(())
        }
    }
    let mut h = Head(String::new());
    let _ = write!(h, "{d:?}");
    h.0
}

/// One node's record.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct NodeRec {
    pub node: String,
    /// The record proper.
    pub lines: Vec<String>,
    /// The name tables alone: the part of the record that is each
    /// output's names, with no incidence.
    pub names: Vec<String>,
    /// What the record leaves out.
    pub extras: Vec<String>,
    /// Cells with no name: faces, edges, vertices.
    pub unnamed: [usize; 3],
    /// Lumps, shells, faces, loops, edges, vertices over its bodies.
    pub cells: [usize; 6],
    /// A measure node's value, as its scalar prints it.
    pub measure: Option<String>,
}

/// A build's record.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct Record {
    pub nodes: Vec<NodeRec>,
}

fn h64(parts: impl FnOnce(&mut std::collections::hash_map::DefaultHasher)) -> String {
    let mut h = std::collections::hash_map::DefaultHasher::new();
    parts(&mut h);
    format!("{:016x}", h.finish())
}

impl Record {
    pub(crate) fn of<T: topo::AtRestPolicy + Decide>(ev: &Evaluation<T>) -> Self {
        let nodes = ev
            .order
            .iter()
            .map(|&id| {
                let mut rec = NodeRec {
                    node: format!("{id:?}"),
                    ..NodeRec::default()
                };
                match ev.nodes.get(&id) {
                    None => rec.lines.push("standing absent".into()),
                    Some(NodeResult::Poisoned { .. }) => rec.lines.push("standing poisoned".into()),
                    Some(NodeResult::Failed(e)) => rec
                        .lines
                        .push(format!("standing failed {:?}", e.kind.class())),
                    Some(NodeResult::Ok(v)) => {
                        rec.lines.push("standing ok".into());
                        value_lines("", v, &mut rec);
                    }
                }
                rec
            })
            .collect();
        Self { nodes }
    }

    pub(crate) fn hash(&self) -> String {
        h64(|h| {
            for n in &self.nodes {
                n.node.hash(h);
                n.lines.hash(h);
            }
        })
    }

    /// Standing and name tables alone.
    pub(crate) fn names_hash(&self) -> String {
        h64(|h| {
            for n in &self.nodes {
                n.node.hash(h);
                n.lines.first().hash(h);
                n.names.hash(h);
            }
        })
    }

    pub(crate) fn extras_hash(&self) -> String {
        h64(|h| {
            for n in &self.nodes {
                n.node.hash(h);
                n.extras.hash(h);
            }
        })
    }

    pub(crate) fn unnamed(&self) -> [usize; 3] {
        let mut out = [0; 3];
        for n in &self.nodes {
            for (o, u) in out.iter_mut().zip(n.unnamed) {
                *o += u;
            }
        }
        out
    }

    pub(crate) fn cells(&self) -> [usize; 6] {
        let mut out = [0; 6];
        for n in &self.nodes {
            for (o, u) in out.iter_mut().zip(n.cells) {
                *o += u;
            }
        }
        out
    }

    pub(crate) fn measures(&self) -> Vec<(String, String)> {
        self.nodes
            .iter()
            .filter_map(|n| n.measure.clone().map(|m| (n.node.clone(), m)))
            .collect()
    }
}

fn clip(s: &str) -> String {
    if s.len() > 600 {
        let mut end = 600;
        while !s.is_char_boundary(end) {
            end -= 1;
        }
        format!("{}…(+{})", &s[..end], s.len() - end)
    } else {
        s.to_owned()
    }
}

/// The first difference of two line lists, by which a comparison names
/// what differs: `[node, a's line, b's line]`.
fn first_diff_of(
    a: &Record,
    b: &Record,
    pick: impl Fn(&NodeRec) -> &Vec<String>,
) -> Option<[String; 3]> {
    let end = "<no such line>".to_owned();
    for i in 0..a.nodes.len().max(b.nodes.len()) {
        match (a.nodes.get(i), b.nodes.get(i)) {
            (Some(x), Some(y)) => {
                if x.node != y.node {
                    return Some([x.node.clone(), "node order".into(), y.node.clone()]);
                }
                let (lx, ly) = (pick(x), pick(y));
                if lx != ly {
                    // Lines are sorted within a section, so the first
                    // line one side has and the other lacks is what a
                    // reader wants, not the first index.
                    let sx: BTreeSet<&String> = lx.iter().collect();
                    let sy: BTreeSet<&String> = ly.iter().collect();
                    let only_a = lx.iter().find(|l| !sy.contains(l));
                    let only_b = ly.iter().find(|l| !sx.contains(l));
                    return Some([
                        x.node.clone(),
                        clip(only_a.unwrap_or(&end)),
                        clip(only_b.unwrap_or(&end)),
                    ]);
                }
            }
            (Some(x), None) => return Some([x.node.clone(), "node".into(), end]),
            (None, Some(y)) => return Some([y.node.clone(), end, "node".into()]),
            (None, None) => {}
        }
    }
    None
}

pub(crate) fn first_diff(a: &Record, b: &Record) -> Option<[String; 3]> {
    first_diff_of(a, b, |n| &n.lines)
}

pub(crate) fn first_extras_diff(a: &Record, b: &Record) -> Option<[String; 3]> {
    first_diff_of(a, b, |n| &n.extras)
}

fn value_lines<T: topo::AtRestPolicy + Decide>(prefix: &str, v: &NodeValue<T>, rec: &mut NodeRec) {
    rec.lines
        .push(format!("{prefix}payload {}", v.payload.kind_name()));
    // The bodies, each with the port whose table names it and its index
    // in that table.
    let mut bodies: Vec<(u32, u8, u32, &crate::eval::Framed<T>)> = Vec::new();
    match &v.payload {
        ValuePayload::Body(b) | ValuePayload::Boolean(BooleanValue::Body { body: b, .. }) => {
            bodies.push((0, 0, 0, b));
        }
        ValuePayload::Boolean(BooleanValue::Empty) => rec.lines.push(format!("{prefix}empty")),
        ValuePayload::Split { above, below } => {
            for (i, side) in [above, below].into_iter().enumerate() {
                let half = if i == 0 {
                    crate::names::SplitHalf::Above
                } else {
                    crate::names::SplitHalf::Below
                };
                let ix = half.output_body();
                match side {
                    SplitSide::Empty => rec.lines.push(format!("{prefix}b{ix} empty half")),
                    #[allow(clippy::cast_possible_truncation)]
                    SplitSide::Body(b) => bodies.push((ix, ix as u8, 0, b)),
                }
            }
        }
        ValuePayload::Instances(members) => {
            for (i, m) in members.iter().enumerate() {
                #[allow(clippy::cast_possible_truncation)]
                let i = i as u32;
                match m {
                    None => rec.lines.push(format!("{prefix}b{i} empty member")),
                    Some(b) => bodies.push((i, 0, i, b)),
                }
            }
        }
        ValuePayload::Measure { value, dim } => {
            rec.lines.push(format!("{prefix}measure dim {dim:?}"));
            rec.measure = Some(format!("{value:?}"));
        }
        ValuePayload::Assertion(a) => {
            rec.extras.push(format!("{prefix}assertion {}", head(a)));
        }
        ValuePayload::Profile(p) => {
            rec.lines
                .push(format!("{prefix}profile naming {:?}", p.naming));
            rec.lines
                .push(format!("{prefix}profile pieces {:?}", p.pieces));
        }
        ValuePayload::Datum(_) | ValuePayload::Family => {}
    }
    // Every table's names, whether or not a cell of a body answers.
    let mut port = 0u8;
    while let Some(out) = v.output(port) {
        let mut names: Vec<String> = out
            .table()
            .iter()
            .map(|(name, entry)| {
                format!(
                    "{prefix}p{port} name {name:?} x{} bodies {:?}",
                    entry.entities().len(),
                    entry.entities().iter().map(|e| e.body).collect::<Vec<_>>()
                )
            })
            .collect();
        names.sort();
        rec.names.extend(names.iter().cloned());
        rec.lines.extend(names);
        port += 1;
    }
    for (ix, port, tb, framed) in bodies {
        let table = v.output(port).map(crate::eval::OutputNames::table);
        body_lines(&format!("{prefix}b{ix} "), framed.body(), table, tb, rec);
        rec.extras.push(format!(
            "{prefix}b{ix} held {}",
            head(&format_args!("{:?}", HeldKind::of(framed.holding())))
        ));
    }
    for (i, c) in v.coincidences.iter().enumerate() {
        rec.lines.push(format!(
            "{prefix}coin {i} {:?} at {:?} origin {:?} cells {:?}",
            c.relation, c.site, c.origin, c.cells
        ));
        // The margin itself is a number at the run's scalar and is in
        // neither list.
        rec.extras
            .push(format!("{prefix}coin {i} discharge {:?}", c.discharge));
    }
    if let Some(members) = &v.members {
        for (at, m) in &members.members {
            value_lines(&format!("{prefix}m{at:?} "), m, rec);
        }
    }
}

#[derive(Debug)]
enum HeldKind {
    Finished,
    Leaf,
}

impl HeldKind {
    fn of<T: topo::AtRestPolicy>(h: &crate::eval::Held<T>) -> Self {
        match h {
            crate::eval::Held::Finished(_) => Self::Finished,
            crate::eval::Held::Leaf(_) => Self::Leaf,
        }
    }
}

/// The least rotation of a cycle.
fn least_rotation(tokens: &[String]) -> Vec<String> {
    let n = tokens.len();
    (0..n)
        .map(|r| {
            (0..n)
                .map(|i| tokens[(r + i) % n].clone())
                .collect::<Vec<_>>()
        })
        .min()
        .unwrap_or_default()
}

fn body_lines<T: topo::AtRestPolicy + Decide>(
    prefix: &str,
    body: &topo::Body<T>,
    table: Option<&NameTable>,
    tb: u32,
    rec: &mut NodeRec,
) {
    let mut unnamed = [0usize; 3];
    let mut name = |key: EntityKey, slot: usize| -> String {
        match table.and_then(|t| t.name_of(&EntityRef { body: tb, key })) {
            Some(n) => format!("{n:?}"),
            None => {
                unnamed[slot] += 1;
                "<unnamed>".to_owned()
            }
        }
    };
    let faces: BTreeMap<_, String> = body
        .faces()
        .map(|(k, _)| (k, name(EntityKey::Face(k), 0)))
        .collect();
    let edges: BTreeMap<_, String> = body
        .edges()
        .map(|(k, _)| (k, name(EntityKey::Edge(k), 1)))
        .collect();
    let verts: BTreeMap<_, String> = body
        .vertices()
        .map(|(k, _)| (k, name(EntityKey::Vertex(k), 2)))
        .collect();
    for (o, u) in rec.unnamed.iter_mut().zip(unnamed) {
        *o += u;
    }
    let missing = "<no such cell>".to_owned();
    // Loops by face, each a canonical cycle of (edge, direction, start).
    let mut loops_of: BTreeMap<topo::FaceKey, Vec<(bool, String)>> = BTreeMap::new();
    let mut n_loops = 0usize;
    for (lk, l) in body.loops() {
        n_loops += 1;
        let outer = body.get_face(l.face).is_some_and(|f| f.outer == lk);
        let text = match l.boundary {
            topo::LoopBoundary::Empty { vertex } => {
                format!("point({})", verts.get(&vertex).unwrap_or(&missing))
            }
            topo::LoopBoundary::Cycle { first } => {
                let tokens: Vec<String> = body
                    .loop_cycle(first)
                    .unwrap_or_default()
                    .into_iter()
                    .map(|he| match body.get_half_edge(he) {
                        Some(h) => {
                            let dir = match body.get_edge(h.edge) {
                                Some(e) if e.he_plus == he => '+',
                                Some(_) => '-',
                                None => '?',
                            };
                            format!(
                                "{}{dir} from {}",
                                edges.get(&h.edge).unwrap_or(&missing),
                                verts.get(&h.start).unwrap_or(&missing)
                            )
                        }
                        None => missing.clone(),
                    })
                    .collect();
                format!("cycle({})", least_rotation(&tokens).join("; "))
            }
        };
        loops_of.entry(l.face).or_default().push((outer, text));
    }
    let mut lines: Vec<String> = Vec::new();
    let mut extras: Vec<String> = Vec::new();
    for (fk, f) in body.faces() {
        let fname = faces.get(&fk).unwrap_or(&missing);
        let mut loops = loops_of.remove(&fk).unwrap_or_default();
        loops.sort();
        let text: Vec<String> = loops
            .into_iter()
            .map(|(outer, t)| format!("{} {t}", if outer { "outer" } else { "inner" }))
            .collect();
        lines.push(format!("{prefix}face {fname} bounded by {}", text.join(" | ")));
        extras.push(format!(
            "{prefix}face {fname} carrier {} sense {}",
            body.face_surface(fk).map_or_else(|| "?".into(), head),
            f.sense
        ));
    }
    for (ek, e) in body.edges() {
        let ename = edges.get(&ek).unwrap_or(&missing);
        let ends = body.edge_vertices(ek).map_or_else(
            || "?".to_owned(),
            |(a, b)| {
                format!(
                    "{} to {}",
                    verts.get(&a).unwrap_or(&missing),
                    verts.get(&b).unwrap_or(&missing)
                )
            },
        );
        lines.push(format!("{prefix}edge {ename} runs {ends}"));
        extras.push(format!(
            "{prefix}edge {ename} carrier {}",
            match body.get_curve_geom(e.curve) {
                Some(topo::CurveGeom::Certified(c)) => format!(
                    "{} stored {} described {}",
                    head(c.carrier()),
                    head(c.stored_carrier()),
                    head(c.description())
                ),
                Some(topo::CurveGeom::NullScaffold(_)) => "NullScaffold".into(),
                None => "?".into(),
            }
        ));
    }
    for name in verts.values() {
        lines.push(format!("{prefix}vertex {name}"));
    }
    let mut n_shells = 0usize;
    let mut n_lumps = 0usize;
    for (_, solid) in body.solids() {
        n_lumps += 1;
        let mut shells: Vec<String> = solid
            .shells
            .iter()
            .map(|&sk| {
                n_shells += 1;
                let mut fs: Vec<&String> = body
                    .get_shell(sk)
                    .map(|s| {
                        s.faces
                            .iter()
                            .map(|f| faces.get(f).unwrap_or(&missing))
                            .collect()
                    })
                    .unwrap_or_default();
                fs.sort();
                format!(
                    "shell[{}]",
                    fs.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", ")
                )
            })
            .collect();
        shells.sort();
        lines.push(format!("{prefix}lump {}", shells.join(" ")));
    }
    lines.sort();
    extras.sort();
    rec.lines.extend(lines);
    rec.extras.extend(extras);
    for (o, u) in rec.cells.iter_mut().zip([
        n_lumps,
        n_shells,
        faces.len(),
        n_loops,
        edges.len(),
        verts.len(),
    ]) {
        *o += u;
    }
}

// ----------------------------------------------------------------
// The logs.
// ----------------------------------------------------------------

fn standing<T: topo::AtRestPolicy + Decide>(ev: &Evaluation<T>, id: RecipeNodeId) -> String {
    match ev.nodes.get(&id) {
        None => "absent".into(),
        Some(NodeResult::Ok(_)) => "ok".into(),
        Some(NodeResult::Failed(e)) => format!("failed {:?}", e.kind.class()),
        Some(NodeResult::Poisoned { .. }) => "poisoned".into(),
    }
}

fn say(v: Option<&Verdict>) -> String {
    v.map_or_else(
        || "<end>".to_owned(),
        |v| format!("{} {:?} {:?}", v.predicate, v.sign, v.role),
    )
}

/// The three hashes of a run's logs: its choices (standing and every
/// choice's predicate and sign), its path (the choices with the
/// candidate marks the first-turn reading walks), its whole log
/// (checks included).
pub(crate) fn log_hashes<T: topo::AtRestPolicy + Decide>(
    ev: &Evaluation<T>,
    keep: &dyn Fn(RecipeNodeId) -> bool,
) -> [String; 3] {
    let choice = h64(|h| {
        for &id in ev.order.iter().filter(|&&id| keep(id)) {
            id.0.hash(h);
            standing(ev, id).hash(h);
            if let Some(v) = ev.value(id) {
                for d in v.verdicts.iter().filter(|d| d.role == Role::Choice) {
                    d.predicate.hash(h);
                    format!("{:?}", d.sign).hash(h);
                }
            }
        }
    });
    let path = h64(|h| {
        for &id in ev.order.iter().filter(|&&id| keep(id)) {
            id.0.hash(h);
            standing(ev, id).hash(h);
            if let Some(v) = ev.value(id) {
                for d in v.verdicts.iter().filter(|d| d.role == Role::Choice) {
                    d.predicate.hash(h);
                    format!("{:?}", d.sign).hash(h);
                }
                for m in v.marks.iter().filter(|m| m.role == Role::Choice) {
                    format!("{:?} {:?} {}", m.site.name, m.item, m.before).hash(h);
                }
            }
        }
    });
    let strict = h64(|h| {
        for &id in ev.order.iter().filter(|&&id| keep(id)) {
            id.0.hash(h);
            standing(ev, id).hash(h);
            if let Some(v) = ev.value(id) {
                for d in v.verdicts.iter() {
                    say(Some(d)).hash(h);
                }
            }
        }
    });
    [choice, path, strict]
}

/// How two runs' logs compare, as JSON: `a` is the witness (or the f64
/// lane), `b` the leaf (or the box lane).
pub(crate) fn log_diff<A, B>(
    a: &Evaluation<A>,
    b: &Evaluation<B>,
    keep: &dyn Fn(RecipeNodeId) -> bool,
) -> Value
where
    A: topo::AtRestPolicy + Decide,
    B: topo::AtRestPolicy + Decide,
{
    let ids: Vec<RecipeNodeId> = {
        let mut seen = BTreeSet::new();
        a.order
            .iter()
            .chain(&b.order)
            .copied()
            .filter(|&id| keep(id) && seen.insert(id))
            .collect()
    };
    let mut standing_diffs = Vec::new();
    let mut turned = Vec::new();
    let mut parted = Vec::new();
    let mut first: Option<Value> = None;
    let mut check_turned: Option<Value> = None;
    let mut check_parted: Option<Value> = None;
    let mut strict_equal = true;
    let mut choices_equal = true;
    let mut marks_equal = true;
    let (mut esc_a, mut esc_b) = (0usize, 0usize);
    let mut first_esc_b: Option<Value> = None;
    for id in ids {
        let (sa, sb) = (standing(a, id), standing(b, id));
        for (ev_esc, n) in [(esc_of(a, id), &mut esc_a), (esc_of(b, id), &mut esc_b)] {
            *n += ev_esc.len();
        }
        if first_esc_b.is_none() {
            if let Some(p) = esc_of(b, id).first() {
                first_esc_b = Some(json!([format!("{id:?}"), p, first_escalation(b, id)]));
            }
        }
        if sa != sb {
            standing_diffs.push(json!([
                format!("{id:?}"),
                sa,
                sb,
                failure(a, id),
                failure(b, id)
            ]));
            strict_equal = false;
            choices_equal = false;
            continue;
        }
        let (Some(va), Some(vb)) = (a.value(id), b.value(id)) else {
            continue;
        };
        if va.marks[..] != vb.marks[..] {
            marks_equal = false;
        }
        if va.verdicts[..] != vb.verdicts[..] {
            strict_equal = false;
            if first.is_none() {
                let at = va
                    .verdicts
                    .iter()
                    .zip(vb.verdicts.iter())
                    .position(|(x, y)| x != y)
                    .unwrap_or_else(|| va.verdicts.len().min(vb.verdicts.len()));
                let (x, y) = (va.verdicts.get(at), vb.verdicts.get(at));
                let kind = match (x, y) {
                    (Some(x), Some(y)) if x.predicate == y.predicate && x.role == y.role => {
                        "turned"
                    }
                    (Some(x), Some(y)) if x.predicate == y.predicate => "role",
                    (Some(_), Some(_)) => "other predicate",
                    _ => "one log ends",
                };
                first = Some(json!({
                    "node": format!("{id:?}"), "at": at, "kind": kind,
                    "a": say(x), "b": say(y),
                    "len_a": va.verdicts.len(), "len_b": vb.verdicts.len(),
                }));
            }
        }
        let pick = |log: &[Verdict], role: Role| -> Vec<(&'static str, String)> {
            log.iter()
                .filter(|d| d.role == role)
                .map(|d| (d.predicate, format!("{:?}", d.sign)))
                .collect()
        };
        let (ca, cb) = (
            pick(&va.verdicts, Role::Choice),
            pick(&vb.verdicts, Role::Choice),
        );
        if ca != cb {
            choices_equal = false;
        }
        match crate::resolve::first_turn(
            crate::resolve::Path {
                verdicts: &va.verdicts,
                marks: &va.marks,
            },
            crate::resolve::Path {
                verdicts: &vb.verdicts,
                marks: &vb.marks,
            },
        ) {
            crate::resolve::FirstTurn::Agree => {}
            crate::resolve::FirstTurn::Turned(f) => turned.push(json!([
                format!("{id:?}"),
                f.predicate,
                format!("{:?}", f.from),
                format!("{:?}", f.to)
            ])),
            crate::resolve::FirstTurn::Parted(p) => {
                let (kind, site, step) = match p {
                    crate::resolve::Parting::Mark { site, step } => ("mark", site.name, step),
                    crate::resolve::Parting::Step { step } => ("step", "", step),
                    crate::resolve::Parting::Ended { step } => ("ended", "", step),
                };
                parted.push(json!([format!("{id:?}"), kind, site, step]));
            }
        }
        if ca == cb {
            let (ka, kb) = (
                pick(&va.verdicts, Role::Check),
                pick(&vb.verdicts, Role::Check),
            );
            if ka != kb {
                let at = ka
                    .iter()
                    .zip(&kb)
                    .position(|(x, y)| x != y)
                    .unwrap_or_else(|| ka.len().min(kb.len()));
                match (ka.get(at), kb.get(at)) {
                    (Some(x), Some(y)) if x.0 == y.0 => {
                        if check_turned.is_none() {
                            check_turned = Some(json!([format!("{id:?}"), x.0, x.1, y.1]));
                        }
                    }
                    (x, y) => {
                        if check_parted.is_none() {
                            check_parted = Some(json!([
                                format!("{id:?}"),
                                at,
                                x.map(|x| x.0),
                                y.map(|y| y.0),
                                ka.len(),
                                kb.len()
                            ]));
                        }
                    }
                }
            }
        }
    }
    json!({
        "strict_equal": strict_equal,
        "choices_equal": choices_equal,
        "marks_equal": marks_equal,
        "standing": standing_diffs,
        "turned": turned,
        "parted": parted,
        "first": first,
        "check_turned": check_turned,
        "check_parted": check_parted,
        "esc_a": esc_a,
        "esc_b": esc_b,
        "first_esc_b": first_esc_b,
    })
}

/// A failed node's refusal, as it prints.
fn failure<T: topo::AtRestPolicy + Decide>(ev: &Evaluation<T>, id: RecipeNodeId) -> Option<String> {
    match ev.nodes.get(&id) {
        Some(NodeResult::Failed(e)) => Some(clip(&format!("{:?}", e.kind))),
        _ => None,
    }
}

/// A node's first escalation, whole: the margin's enclosure, its
/// decoration, the band.
fn first_escalation<T: topo::AtRestPolicy + Decide>(
    ev: &Evaluation<T>,
    id: RecipeNodeId,
) -> Option<String> {
    let log = match ev.nodes.get(&id) {
        Some(NodeResult::Ok(v)) => &v.escalations,
        Some(NodeResult::Failed(e)) => &e.escalations,
        _ => return None,
    };
    log.first().map(|e| clip(&format!("{:?}", e.source)))
}

/// The geometry a body's named cells sit on, as it prints at the run's
/// scalar: each vertex's point, each face's sense and surface, each
/// edge's carrier. For holding one leaf against its witness by eye.
pub(crate) fn geometry<T: topo::AtRestPolicy + Decide>(ev: &Evaluation<T>) -> Vec<String> {
    let mut out = Vec::new();
    for &id in &ev.order {
        let Some(v) = ev.value(id) else { continue };
        let Some(Some(framed)) = v.payload.one_body() else {
            continue;
        };
        let body = framed.body();
        let table = v.output(0).map(crate::eval::OutputNames::table);
        let name = |key: EntityKey| -> String {
            table
                .and_then(|t| t.name_of(&EntityRef { body: 0, key }))
                .map_or_else(|| "<unnamed>".to_owned(), |n| format!("{:?}", n.path))
        };
        let mut lines = Vec::new();
        for (k, p) in body.vertex_points() {
            lines.push(format!("vertex {} at {p:?}", name(EntityKey::Vertex(k))));
        }
        for (k, f) in body.faces() {
            lines.push(format!(
                "face {} sense {} on {}",
                name(EntityKey::Face(k)),
                f.sense,
                body.face_surface(k)
                    .map_or_else(|| "?".into(), |s| clip(&format!("{s:?}")))
            ));
        }
        for (k, e) in body.edges() {
            if let Some(topo::CurveGeom::Certified(c)) = body.get_curve_geom(e.curve) {
                lines.push(format!(
                    "edge {} on {} over {:?}",
                    name(EntityKey::Edge(k)),
                    clip(&format!("{:?}", c.carrier())),
                    c.params()
                ));
            }
        }
        lines.sort();
        out.push(format!("node {id:?}"));
        out.extend(lines);
    }
    out
}

fn esc_of<T: topo::AtRestPolicy + Decide>(ev: &Evaluation<T>, id: RecipeNodeId) -> Vec<&'static str> {
    let log = match ev.nodes.get(&id) {
        Some(NodeResult::Ok(v)) => &v.escalations,
        Some(NodeResult::Failed(e)) => &e.escalations,
        _ => return Vec::new(),
    };
    log.iter().map(geom_core::k_stats::Escalation::predicate).collect()
}

/// Everything one comparison of two builds says, as JSON fields merged
/// into `line`: the record and the extras (equal or the first
/// difference), the logs, the cell counts, the costs.
pub(crate) fn compare_into<A, B>(
    line: &mut Value,
    a: &Evaluation<A>,
    ra: &Record,
    b: &Evaluation<B>,
    keep: &dyn Fn(RecipeNodeId) -> bool,
) where
    A: topo::AtRestPolicy + Decide,
    B: topo::AtRestPolicy + Decide,
{
    let t0 = std::time::Instant::now();
    let rb = Record::of(b);
    let t_record = t0.elapsed();
    let t1 = std::time::Instant::now();
    let rec_eq = ra
        .nodes
        .iter()
        .zip(&rb.nodes)
        .all(|(x, y)| x.node == y.node && x.lines == y.lines)
        && ra.nodes.len() == rb.nodes.len();
    let t_compare = t1.elapsed();
    let extras_eq = ra
        .nodes
        .iter()
        .zip(&rb.nodes)
        .all(|(x, y)| x.extras == y.extras)
        && ra.nodes.len() == rb.nodes.len();
    let [choice, path, strict] = log_hashes(b, keep);
    let Some(o) = line.as_object_mut() else { return };
    o.insert("rec_eq".into(), json!(rec_eq));
    o.insert("extras_eq".into(), json!(extras_eq));
    if !rec_eq {
        o.insert("rec_first".into(), json!(first_diff(ra, &rb)));
    }
    if !extras_eq {
        o.insert("extras_first".into(), json!(first_extras_diff(ra, &rb)));
    }
    o.insert("rec_hash".into(), json!(rb.hash()));
    o.insert("names_hash".into(), json!(rb.names_hash()));
    o.insert("names_eq".into(), json!(ra.names_hash() == rb.names_hash()));
    o.insert("extras_hash".into(), json!(rb.extras_hash()));
    o.insert("choice_hash".into(), json!(choice));
    o.insert("path_hash".into(), json!(path));
    o.insert("strict_hash".into(), json!(strict));
    o.insert("unnamed".into(), json!(rb.unnamed()));
    o.insert("unnamed_a".into(), json!(ra.unnamed()));
    o.insert("cells".into(), json!(rb.cells()));
    o.insert("measures".into(), json!(rb.measures()));
    o.insert("log".into(), log_diff(a, b, keep));
    #[allow(clippy::cast_possible_truncation)]
    {
        o.insert("t_record_us".into(), json!(t_record.as_micros() as u64));
        o.insert("t_compare_ns".into(), json!(t_compare.as_nanos() as u64));
    }
    o.insert(
        "rec_bytes".into(),
        json!(
            rb.nodes
                .iter()
                .map(|n| n.lines.iter().map(String::len).sum::<usize>())
                .sum::<usize>()
        ),
    );
    let ok = |ev_nodes: usize, oks: usize| json!([oks, ev_nodes]);
    o.insert(
        "ok_a".into(),
        ok(
            a.order.len(),
            a.order.iter().filter(|&&id| a.value(id).is_some()).count(),
        ),
    );
    o.insert(
        "ok_b".into(),
        ok(
            b.order.len(),
            b.order.iter().filter(|&&id| b.value(id).is_some()).count(),
        ),
    );
}

// ----------------------------------------------------------------
// The parity hook: every top-level f64 evaluation, replayed at the box
// scalar over nothing wider than what the f64 run was given.
// ----------------------------------------------------------------

thread_local! {
    static INSIDE: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    /// The wall time of the leaf evaluation `classify` just ran, for
    /// the leaf line `classify_replay` writes on the same thread.
    pub(crate) static LEAF_EVAL_US: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
}

/// Runs `f` with the parity hook off on this thread (a drive's own
/// evaluations are compared by the drive's lines).
pub(crate) fn quiet<R>(f: impl FnOnce() -> R) -> R {
    let was = INSIDE.with(|c| c.replace(true));
    let out = f();
    INSIDE.with(|c| c.set(was));
    out
}

pub(crate) fn parity_hook<T>(
    doc: &crate::Doc<crate::program::ProfileProgram>,
    ev: &Evaluation<T>,
    opts: &crate::eval::EvalOptions,
    tol: geom_core::Tol,
) where
    T: crate::eval::OwnRecord + 'static,
{
    if !parity_on() || INSIDE.with(std::cell::Cell::get) {
        return;
    }
    let Some(ev) = (ev as &dyn core::any::Any).downcast_ref::<Evaluation<f64>>() else {
        return;
    };
    if opts.seed.is_some() || opts.symbols || ev.order.is_empty() {
        return;
    }
    if crate::Stopped::of(ev.outcome).is_some() {
        return;
    }
    let all = |_: RecipeNodeId| true;
    let ra = Record::of(ev);
    static SEEN: Mutex<BTreeSet<(String, String)>> = Mutex::new(BTreeSet::new());
    let key = (log_hashes(ev, &all)[2].clone(), ra.hash());
    if !SEEN.lock().map(|mut s| s.insert(key)).unwrap_or(false) {
        return;
    }
    quiet(|| {
        let opts = crate::eval::EvalOptions {
            parallel: false,
            ..opts.clone()
        };
        let t0 = std::time::Instant::now();
        let iv: Evaluation<geom_core::Interval> = crate::eval::evaluate(
            doc,
            None,
            &crate::eval::CancelToken::new(),
            &opts,
            tol,
        );
        #[allow(clippy::cast_possible_truncation)]
        let t_eval = t0.elapsed().as_micros() as u64;
        let mut line = json!({
            "k": "parity",
            "doc": format!("{:?}", doc.id()),
            "boxed": opts.param_box.is_some(),
            "resolver": opts.resolver.is_some(),
            "a_rec_hash": ra.hash(),
            "a_strict_hash": log_hashes(ev, &all)[2],
            "t_eval_us": t_eval,
            "kinds": payload_kinds(ev),
        });
        compare_into(&mut line, ev, &ra, &iv, &all);
        emit(line);
    });
}

/// The payload kinds a build holds, with how many nodes of each: what a
/// document is made of, for reading a parity line.
pub(crate) fn payload_kinds<T: topo::AtRestPolicy + Decide>(ev: &Evaluation<T>) -> Value {
    let mut m: BTreeMap<String, usize> = BTreeMap::new();
    for &id in &ev.order {
        let k = match ev.nodes.get(&id) {
            Some(NodeResult::Ok(v)) => v.payload.kind_name().to_owned(),
            other => format!(
                "!{}",
                match other {
                    None => "absent",
                    Some(NodeResult::Failed(_)) => "failed",
                    _ => "poisoned",
                }
            ),
        };
        *m.entry(k).or_default() += 1;
    }
    json!(m)
}
