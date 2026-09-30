//! PROBE ONLY (emit/borders-mechanism-probe, never for merge): the
//! kernel's discarded-fragment record, resolved into member space and
//! logged per boolean/union node, so a measurement can chain it.
//!
//! Recording is off unless `BORDERS_PROBE_LOG` is set.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::Mutex;

use crate::names::{EntityKey, EntityKind, NameTable, RoleSeg, StableName};
use crate::node::RecipeNodeId;

/// One discarded fragment, resolved.
#[derive(Clone, Debug)]
pub struct ProbeDiscard {
    /// Fold step (0 for a pair boolean).
    pub step: usize,
    /// Whether it was a B-side (member / second operand) fragment.
    pub operand_b: bool,
    /// The member faces of the operand face it is a fragment of.
    pub member_faces: BTreeSet<(RecipeNodeId, StableName)>,
    /// Result edges (keys of its step's result) it bordered a kept
    /// fragment along.
    pub seams: Vec<topo::EdgeKey>,
    /// Section segments whose result edge was not found.
    pub unresolved: usize,
    /// Earlier steps' recorded seam edges its boundary descends from.
    pub touches: Vec<topo::EdgeKey>,
    /// Its boundary edges' split chains (the edge, then its ancestors),
    /// which carry the lineage of edges that die with it.
    pub chains: Vec<Vec<topo::EdgeKey>>,
}

/// One node's log.
#[derive(Clone, Debug, Default)]
pub struct ProbeLog {
    /// Every discard, in step order.
    pub discards: Vec<ProbeDiscard>,
    /// Each step's kernel path.
    pub paths: Vec<&'static str>,
    /// Anything the resolution could not do.
    pub notes: Vec<String>,
}

/// The logs, by node.
pub static LOG: Mutex<BTreeMap<RecipeNodeId, ProbeLog>> = Mutex::new(BTreeMap::new());

/// Whether recording is on.
pub fn enabled() -> bool {
    std::env::var("BORDERS_PROBE_LOG").is_ok()
}

/// The name space an operand's names live in.
pub(crate) enum Space {
    /// A union's fold: names collapse to member space under `node`.
    Union(RecipeNodeId),
    /// A pair boolean over operands `a` and `b`.
    Pair(RecipeNodeId, RecipeNodeId),
}

fn member_faces(n: &StableName, out: &mut BTreeSet<(RecipeNodeId, StableName)>) {
    match n.path.first() {
        Some(RoleSeg::FromMember { member, of }) if of.kind == EntityKind::Face => {
            out.insert((*member, of.name().clone()));
        }
        Some(RoleSeg::Merged(set)) => {
            for c in set {
                member_faces(c, out);
            }
        }
        _ => {}
    }
}

/// Records one boolean step's discards into `node`'s log.
#[allow(clippy::too_many_arguments)]
pub(crate) fn record_step<T: geom_core::Decide>(
    node: RecipeNodeId,
    step: usize,
    space: &Space,
    naming: &topo::BooleanNaming,
    result: &topo::Body<T>,
    a_table: &NameTable,
    b_table: &NameTable,
) {
    let mut logs = LOG.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    if step == 0 {
        logs.insert(node, ProbeLog::default());
    }
    let log = logs.entry(node).or_default();
    log.paths.push(naming.probe_path);
    let a_rows: BTreeMap<topo::FaceKey, topo::FaceKey> =
        naming.face_fragments_a.iter().copied().collect();
    let b_rows: BTreeMap<topo::FaceKey, topo::FaceKey> =
        naming.face_fragments_b.iter().copied().collect();
    let chase = |rows: &BTreeMap<topo::FaceKey, topo::FaceKey>, mut f: topo::FaceKey| {
        for _ in 0..100_000 {
            match rows.get(&f) {
                Some(&p) if p != f => f = p,
                _ => break,
            }
        }
        f
    };
    let merges: BTreeMap<topo::VertexKey, topo::VertexKey> =
        naming.vertex_merges.iter().copied().collect();
    let settle = |mut v: topo::VertexKey| {
        for _ in 0..100_000 {
            match merges.get(&v) {
                Some(&k) if k != v => v = k,
                _ => break,
            }
        }
        v
    };
    let mut by_ends: BTreeMap<(topo::VertexKey, topo::VertexKey), Vec<topo::EdgeKey>> =
        BTreeMap::new();
    for (k, e) in result.edges() {
        let (Some(h), Some(t)) = (
            result.get_half_edge(e.he_plus).map(|h| h.start),
            result.half_edge_end(e.he_plus),
        ) else {
            continue;
        };
        by_ends.entry((h.min(t), h.max(t))).or_default().push(k);
    }
    let earlier: BTreeSet<topo::EdgeKey> =
        log.discards.iter().flat_map(|d| d.seams.iter().copied()).collect();
    for d in &naming.discards {
        let (rows, table) = if d.operand_b {
            (&b_rows, b_table)
        } else {
            (&a_rows, a_table)
        };
        let root = chase(rows, d.face);
        let ent = crate::names::emit::ent(0, EntityKey::Face(root));
        let mut mf = BTreeSet::new();
        match table.name_of(&ent) {
            None => log.notes.push(format!("step {step}: a discard's operand face is unnamed")),
            Some(n) => match space {
                Space::Union(u) => match crate::names::collapse_name(*u, n) {
                    Ok(c) => member_faces(&c, &mut mf),
                    Err(e) => log.notes.push(format!("step {step}: collapse: {e:?}")),
                },
                Space::Pair(a, b) => {
                    mf.insert((if d.operand_b { *b } else { *a }, n.clone()));
                }
            },
        }
        let mut seams = Vec::new();
        let mut unresolved = 0;
        for seg in &d.bordered {
            let Some((u, w)) = seg else {
                unresolved += 1;
                continue;
            };
            let (u, w) = (settle(*u), settle(*w));
            match by_ends.get(&(u.min(w), u.max(w))) {
                Some(es) => seams.extend(es.iter().copied()),
                None => {
                    unresolved += 1;
                    let why = if u == w {
                        "ends fused"
                    } else if result.get_vertex(u).is_none() || result.get_vertex(w).is_none() {
                        "an end died"
                    } else {
                        "no edge joins the live ends"
                    };
                    log.notes.push(format!("step {step}: unresolved segment: {why}"));
                }
            }
        }
        let touches = d
            .boundary_chains
            .iter()
            .flatten()
            .filter(|k| earlier.contains(k))
            .copied()
            .collect();
        log.discards.push(ProbeDiscard {
            step,
            operand_b: d.operand_b,
            member_faces: mf,
            seams,
            unresolved,
            touches,
            chains: d.boundary_chains.clone(),
        });
    }
}
