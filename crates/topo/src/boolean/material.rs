//! **A cut reads its tool as material** (`crates/topo/README.md`,
//! `[topo.cut-material]`): `t − (p ⊔ q)` is computed as
//! `t − (p ∪ q)`. Where the tool's solids rest on each other, the cut's
//! door unites them on the face–face records between them, each record's
//! meaning the union's given verdict, and the ordinary cut runs against
//! that material ([`super::nary::cut_by_material`]).
//!
//! The cut's result is then stated in the tool's own terms, as a cut by
//! the tool would state it: every operand-B cell its naming, rows,
//! holdings and records name is read back through the union to the
//! tool's cell it is ([`Through`]). A cell of the material that is one
//! cell of the tool is that cell's key; a face the union divided is a
//! piece of the tool's face it lies in, recorded as a division of the
//! cut's B clone before the cut's own; an edge the union split is a hop
//! to the edge it was split from. A record of the material cites the
//! tool's record it carried. Nothing of the tool's meetings reaches the
//! result: the union glued them, and the cut removes their material.

use std::collections::{BTreeMap, BTreeSet};

use super::holdings::{Held, Holdings};
use super::nary::ToolMaterial;
use super::ops::{BooleanBody, OperandKeys};
use super::{BooleanError, Cell, Operand, OperandRecords};
use crate::body::Body;
use crate::coincidence::{Backing, RowCell};
use crate::entity::{EdgeKey, FaceKey, VertexKey};
use crate::props::AtRestPolicy;
use crate::provenance::Provenance;

/// **The material of a cut's tool, as one body**: each fused group's
/// material and every solid that stands as it is, its records, and its
/// read-back to the tool. One group and nothing standing is that group's
/// material itself; otherwise the standing solids are carved out of the
/// tool with their keys and each group's material grafted in beside them.
/// The material's records are each group's, the standing solids' own, and
/// `riding`, the tool's records between solids that stay apart, read onto
/// the material's keys.
///
/// # Errors
///
/// The carve's, the graft's and the finish's refusals; as
/// [`Through::onto_material`].
pub(super) fn assembled<T: geom_core::Decide + AtRestPolicy>(
    tool: &T::Finished,
    standing: &[crate::SolidKey],
    standing_records: &OperandRecords,
    mut folded: Vec<(ToolMaterial<T>, Through)>,
    riding: &OperandRecords,
    tol: geom_core::Tol,
) -> Result<(T::Finished, OperandRecords, Through), BooleanError> {
    let unread = || BooleanError::JoinDesync { what: UNREAD };
    if standing.is_empty() && folded.len() == 1 {
        let (material, mut through) = folded.pop().ok_or_else(unread)?;
        let fused = material.body();
        let records = through.onto_material(riding, fused.contacts.carried(), &fused.body)?;
        return Ok((fused.body.clone(), records, through));
    }
    let mut body = crate::carve::solids_alone(tool, standing).map_err(|_| {
        BooleanError::ClassificationInvariant {
            what: "a finished tool's solids do not carve out of it",
        }
    })?;
    let (mut through, mut records) = Through::standing(&body, standing_records);
    for (material, th) in folded {
        let fused: &Body<T> = &material.body().body;
        let keys = crate::graft_disjoint_all_keyed(&mut body, fused)?;
        through.absorb(th, &keys, &material.body().contacts.carried(), &mut records)?;
    }
    let records = through.onto_material(riding, records, &body)?;
    let body = T::finish(body, tol).map_err(|findings| BooleanError::ResultInvalid { findings })?;
    Ok((body, records, through))
}

/// What a fold refused to read back, for [`BooleanError::JoinDesync`].
const UNREAD: &str = "a cell or record of a cut's material reads back to no cell or record of its \
                      tool";

/// **The material's cells and records, read back to the tool's**: each
/// key of the material that is one cell of the tool, mapped to that
/// cell's key; each face the union minted, to the tool's face it is a
/// piece of; each edge the union split, to the edge it was split from;
/// each record, to the tool's records it carries.
pub(super) struct Through {
    faces: BTreeMap<FaceKey, FaceKey>,
    edges: BTreeMap<EdgeKey, EdgeKey>,
    vertices: BTreeMap<VertexKey, VertexKey>,
    /// A face the union minted → the tool's face it is a piece of.
    pieces: BTreeMap<FaceKey, FaceKey>,
    /// An edge the union minted → the edge it was split from, in the
    /// material's keys or, where that is one of the tool's, the tool's.
    hops: BTreeMap<EdgeKey, EdgeKey>,
    /// Each face of the material → the tool's faces whose area it holds.
    held: BTreeMap<FaceKey, BTreeSet<FaceKey>>,
    /// The material's edges along which the union fused two of the
    /// tool's solids: between faces holding faces of two of them, which
    /// share no edge of the tool.
    seams: Vec<EdgeKey>,
    /// A seam of [`Self::seams`] the union split off one of the tool's
    /// edges → that edge: the seam lies along it.
    seam_edges: BTreeMap<EdgeKey, EdgeKey>,
    /// Every other seam → a face of the tool on each side of it: it lies
    /// where those two faces meet.
    seam_faces: BTreeMap<EdgeKey, [FaceKey; 2]>,
    /// A vertex the union minted where two of the tool's edges cross →
    /// every edge of the tool it lies on, sorted: the edges its own
    /// edges are pieces of.
    crossings: BTreeMap<VertexKey, Vec<EdgeKey>>,
    /// Each record of the material, by place → the tool's records it
    /// carries.
    records: Vec<Vec<u32>>,
}

impl Through {
    /// The read-back of `material`'s last body, through its steps.
    pub(super) fn of<T: AtRestPolicy>(material: &ToolMaterial<T>) -> Result<Self, BooleanError> {
        let unread = || BooleanError::JoinDesync { what: UNREAD };
        let first: &Body<T> = &material.members[0];
        let mut faces: BTreeMap<FaceKey, FaceKey> = first.faces().map(|(f, _)| (f, f)).collect();
        let mut edges: BTreeMap<EdgeKey, EdgeKey> = first.edges().map(|(e, _)| (e, e)).collect();
        let mut vertices: BTreeMap<VertexKey, VertexKey> =
            first.vertices().map(|(v, _)| (v, v)).collect();
        // The records of the accumulation each step joined to, by place,
        // as the tool's: the first member's cite the tool's own.
        let mut acc_records: Option<Vec<Vec<u32>>> = None;
        for step in &material.steps {
            let member: &Body<T> = &material.members[step.member];
            let naming = &step.body.naming;
            let result: &Body<T> = &step.body.body;
            let (mut f2, mut e2, mut v2) = (BTreeMap::new(), BTreeMap::new(), BTreeMap::new());
            // The joining member's cells: grafted, or the result arena.
            match naming.b_keys {
                OperandKeys::Grafted => {
                    for &(s, d) in &naming.graft_faces {
                        if member.get_face(s).is_some() {
                            f2.insert(d, s);
                        }
                    }
                    for &(s, d) in naming.graft_edges.iter().chain(&naming.graft_dead_edges) {
                        if member.get_edge(s).is_some() {
                            e2.insert(d, s);
                        }
                    }
                    for &(s, d) in &naming.graft_vertices {
                        if member.get_vertex(s).is_some() {
                            v2.insert(d, s);
                        }
                    }
                }
                OperandKeys::Direct => {
                    f2.extend(
                        result
                            .faces()
                            .map(|(f, _)| f)
                            .filter(|&f| member.get_face(f).is_some())
                            .map(|f| (f, f)),
                    );
                    e2.extend(
                        result
                            .edges()
                            .map(|(e, _)| e)
                            .filter(|&e| member.get_edge(e).is_some())
                            .map(|e| (e, e)),
                    );
                    v2.extend(
                        result
                            .vertices()
                            .map(|(v, _)| v)
                            .filter(|&v| member.get_vertex(v).is_some())
                            .map(|v| (v, v)),
                    );
                }
                OperandKeys::Absent => {}
            }
            // The accumulation's cells, where its clone is the result
            // arena.
            if naming.a_keys == OperandKeys::Direct {
                for (f, _) in result.faces() {
                    if let Some(&t) = faces.get(&f) {
                        f2.entry(f).or_insert(t);
                    }
                }
                for (e, _) in result.edges() {
                    if let Some(&t) = edges.get(&e) {
                        e2.entry(e).or_insert(t);
                    }
                }
                for (v, _) in result.vertices() {
                    if let Some(&t) = vertices.get(&v) {
                        v2.entry(v).or_insert(t);
                    }
                }
            }
            // A vertex the zip kept for two it fused is the tool's vertex
            // either stood for.
            for &(dead, by) in naming.vertex_merges.rows() {
                let kept = naming.vertex_merges.survivor(by);
                if v2.contains_key(&kept) {
                    continue;
                }
                if let Some(&t) = v2.get(&dead).or_else(|| vertices.get(&dead)) {
                    v2.insert(kept, t);
                }
            }
            // A copy a null edge made of a vertex is that vertex, where the
            // zip kept the copy: the accumulation's copies in its clone's
            // keys, the result's, the member's in its clone's.
            let grafted: BTreeMap<VertexKey, VertexKey> =
                naming.graft_vertices.iter().copied().collect();
            for &(operand, below, above) in &naming.null_copies {
                let (tool, at) = match (operand, naming.a_keys, naming.b_keys) {
                    (Operand::A, OperandKeys::Direct, _) => {
                        (vertices.get(&below).copied(), Some(above))
                    }
                    (Operand::B, _, OperandKeys::Grafted) => (
                        member.get_vertex(below).is_some().then_some(below),
                        grafted.get(&above).copied(),
                    ),
                    _ => (None, None),
                };
                if let (Some(t), Some(at)) = (tool, at) {
                    let kept = naming.vertex_merges.survivor(at);
                    if result.get_vertex(kept).is_some() {
                        v2.entry(kept).or_insert(t);
                    }
                }
            }
            (faces, edges, vertices) = (f2, e2, v2);
            // Each record of the step's result, as the tool's: input 0 is
            // the accumulation (the first member's records cite the tool's
            // own), input 1 the joining member, whose records cite the
            // tool's.
            let of_backing = |b: Backing| -> Result<Vec<u32>, BooleanError> {
                match (b, &acc_records) {
                    (Backing::Carried { input: 1, record }, _)
                    | (Backing::Carried { input: 0, record }, None) => Ok(vec![record]),
                    (Backing::Carried { input: 0, record }, Some(acc)) => {
                        acc.get(record as usize).cloned().ok_or_else(unread)
                    }
                    _ => Err(unread()),
                }
            };
            let mut next = Vec::with_capacity(step.body.contacts.len());
            for cites in step.body.contacts.cites() {
                let mut tool: BTreeSet<u32> = BTreeSet::new();
                for b in cites.iter() {
                    tool.extend(of_backing(b)?);
                }
                next.push(tool.into_iter().collect());
            }
            acc_records = Some(next);
        }
        let last = material.body();
        let fused: &Body<T> = &last.body;
        // A face the union minted is a piece of the first tool face it
        // holds, and a face that is a tool face's holds that face: a face
        // holding several is the merge of flush faces of two pieces,
        // which the holdings state whole.
        let mut pieces = BTreeMap::new();
        let mut held_by = BTreeMap::new();
        for (f, _) in fused.faces() {
            let held = match last_holdings(material).of(f) {
                Some(Held::Faces(held)) => held.iter().map(|&(_, t)| t).collect::<BTreeSet<_>>(),
                Some(Held::Refused(refusal)) => {
                    return Err(BooleanError::ToolFuseRefused {
                        why: Box::new(super::ToolFuseRefusal::Holding {
                            face: f,
                            refusal: refusal.clone(),
                        }),
                    });
                }
                None => return Err(unread()),
            };
            match (faces.get(&f), held.first()) {
                (Some(t), _) if held.contains(t) => {}
                (None, Some(&t)) => {
                    pieces.insert(f, t);
                }
                _ => return Err(unread()),
            }
            held_by.insert(f, held);
        }
        // An edge between faces of two members is the union's seam,
        // which no edge of the tool is, whatever key it kept.
        let member_of = |f: FaceKey| -> BTreeSet<usize> {
            match last_holdings(material).of(f) {
                Some(Held::Faces(held)) => held.iter().map(|&(m, _)| m).collect(),
                _ => BTreeSet::new(),
            }
        };
        let mut seams = Vec::new();
        for (e, _) in fused.edges() {
            let [x, y] = crate::wedge_reach::edge_faces(fused, e);
            if member_of(x).is_disjoint(&member_of(y)) {
                edges.remove(&e);
                seams.push(e);
            }
        }
        let mut hops = BTreeMap::new();
        for (e, _) in fused.edges() {
            if seams.contains(&e) || edges.contains_key(&e) {
                continue;
            }
            if let Some(&Provenance::SplitEdge { edge: parent }) = fused.edge_provenance_of(e) {
                hops.insert(e, edges.get(&parent).copied().unwrap_or(parent));
            }
        }
        // Each seam the union split off an edge lies along it; each
        // vertex it minted lies on every edge of the tool its own edges
        // are pieces of.
        let tool_edge = |e: EdgeKey| -> Option<EdgeKey> {
            let mut at = e;
            for _ in 0..=hops.len() {
                if let Some(&t) = edges.get(&at) {
                    return Some(t);
                }
                at = match fused.edge_provenance_of(at) {
                    Some(&Provenance::SplitEdge { edge: parent }) => parent,
                    _ => hops.get(&at).copied()?,
                };
            }
            None
        };
        let seam_edges: BTreeMap<EdgeKey, EdgeKey> = seams
            .iter()
            .filter_map(|&e| match fused.edge_provenance_of(e) {
                Some(&Provenance::SplitEdge { edge: parent }) => Some((e, tool_edge(parent)?)),
                _ => None,
            })
            .collect();
        let first_held = |f: FaceKey| {
            held_by
                .get(&f)
                .and_then(|h: &BTreeSet<FaceKey>| h.first().copied())
        };
        let seam_faces: BTreeMap<EdgeKey, [FaceKey; 2]> = seams
            .iter()
            .filter(|e| !seam_edges.contains_key(e))
            .filter_map(|&e| {
                let [x, y] = crate::wedge_reach::edge_faces(fused, e);
                Some((e, [first_held(x)?, first_held(y)?]))
            })
            .collect();
        let on_tool = |e: EdgeKey| tool_edge(e).or_else(|| seam_edges.get(&e).copied());
        let mut crossings: BTreeMap<VertexKey, Vec<EdgeKey>> = BTreeMap::new();
        for (e, edge) in fused.edges() {
            let Some(t) = on_tool(e) else { continue };
            let he = |h| fused.get_half_edge(h).map(|d| d.start);
            for v in [he(edge.he_plus), he(edge.he_minus)].into_iter().flatten() {
                if !vertices.contains_key(&v) {
                    let at = crossings.entry(v).or_default();
                    if !at.contains(&t) {
                        at.push(t);
                    }
                }
            }
        }
        for at in crossings.values_mut() {
            at.sort_unstable();
        }
        Ok(Self {
            faces,
            edges,
            vertices,
            pieces,
            hops,
            held: held_by,
            seams,
            seam_edges,
            seam_faces,
            crossings,
            records: acc_records.unwrap_or_default(),
        })
    }

    /// **The solids of `body` that stand as they are, read as the tool's**:
    /// each cell its own, each face holding itself, and their records
    /// `own` in the tool's keys, each in its place.
    fn standing<T: geom_core::Real>(
        body: &Body<T>,
        own: &OperandRecords,
    ) -> (Self, OperandRecords) {
        let mut through = Self {
            faces: body.faces().map(|(f, _)| (f, f)).collect(),
            edges: body.edges().map(|(e, _)| (e, e)).collect(),
            vertices: body.vertices().map(|(v, _)| (v, v)).collect(),
            pieces: BTreeMap::new(),
            hops: BTreeMap::new(),
            held: body
                .faces()
                .map(|(f, _)| (f, BTreeSet::from([f])))
                .collect(),
            seams: Vec::new(),
            seam_edges: BTreeMap::new(),
            seam_faces: BTreeMap::new(),
            crossings: BTreeMap::new(),
            records: Vec::new(),
        };
        let mut records = own.clone();
        let OperandRecords {
            vv,
            vf,
            ve,
            ee,
            ef,
            ff,
        } = &mut records;
        macro_rules! place {
            ($($list:ident),*) => {
                $(for c in $list.iter_mut() {
                    through.records.push(vec![c.record]);
                    c.record = super::reduce::index(through.records.len() - 1);
                })*
            };
        }
        place!(vv, vf, ve, ee, ef, ff);
        (through, records)
    }

    /// **`group`, one fused group's read-back, grafted into this
    /// material by `keys`**: its cells at their grafted keys, and its
    /// material's records `carried` read onto them, each citing what it
    /// carried of the tool's, appended to `records`.
    ///
    /// # Errors
    ///
    /// [`BooleanError::JoinDesync`] where a record names a cell the graft
    /// does not map, which it maps whole.
    fn absorb(
        &mut self,
        group: Through,
        keys: &crate::GraftKeys,
        carried: &OperandRecords,
        records: &mut OperandRecords,
    ) -> Result<(), BooleanError> {
        let unread = || BooleanError::JoinDesync { what: UNREAD };
        let Through {
            faces,
            edges,
            vertices,
            pieces,
            hops,
            held,
            seams,
            seam_edges,
            seam_faces,
            crossings,
            records: tool_records,
        } = group;
        self.faces.extend(
            faces
                .into_iter()
                .filter_map(|(u, t)| Some((keys.face(u)?, t))),
        );
        self.edges.extend(
            edges
                .into_iter()
                .filter_map(|(u, t)| Some((keys.edge(u)?, t))),
        );
        self.vertices.extend(
            vertices
                .into_iter()
                .filter_map(|(u, t)| Some((keys.vertex(u)?, t))),
        );
        self.pieces.extend(
            pieces
                .into_iter()
                .filter_map(|(u, t)| Some((keys.face(u)?, t))),
        );
        self.held.extend(
            held.into_iter()
                .filter_map(|(u, t)| Some((keys.face(u)?, t))),
        );
        self.seams
            .extend(seams.into_iter().filter_map(|e| keys.edge(e)));
        self.seam_faces.extend(
            seam_faces
                .into_iter()
                .filter_map(|(u, t)| Some((keys.edge(u)?, t))),
        );
        self.seam_edges.extend(
            seam_edges
                .into_iter()
                .filter_map(|(u, t)| Some((keys.edge(u)?, t))),
        );
        self.crossings.extend(
            crossings
                .into_iter()
                .filter_map(|(u, t)| Some((keys.vertex(u)?, t))),
        );
        self.hops.extend(
            hops.into_iter()
                .filter_map(|(u, up)| Some((keys.edge(u)?, keys.edge(up).unwrap_or(up)))),
        );
        let base = self.records.len();
        self.records.extend(tool_records);
        let at = |r: u32| super::reduce::index(base + r as usize);
        let face = |f| keys.face(f).ok_or_else(unread);
        let edge = |e| keys.edge(e).ok_or_else(unread);
        let vertex = |v| keys.vertex(v).ok_or_else(unread);
        let OperandRecords {
            vv,
            vf,
            ve,
            ee,
            ef,
            ff,
        } = carried;
        for c in vv {
            records.vv.push(super::CarriedRecord {
                contact: super::VvContact {
                    a: vertex(c.contact.a)?,
                    b: vertex(c.contact.b)?,
                },
                record: at(c.record),
            });
        }
        for c in vf {
            records.vf.push(super::CarriedRecord {
                contact: super::VfContact {
                    vertex: vertex(c.contact.vertex)?,
                    face: face(c.contact.face)?,
                },
                record: at(c.record),
            });
        }
        for c in ve {
            records.ve.push(super::CarriedRecord {
                contact: super::VeContact {
                    vertex: vertex(c.contact.vertex)?,
                    edge: edge(c.contact.edge)?,
                },
                record: at(c.record),
            });
        }
        for c in ee {
            records.ee.push(super::CarriedRecord {
                contact: super::EeContact {
                    a: edge(c.contact.a)?,
                    b: edge(c.contact.b)?,
                },
                record: at(c.record),
            });
        }
        for c in ef {
            records.ef.push(super::CarriedRecord {
                contact: super::EfContact {
                    edge: edge(c.contact.edge)?,
                    face: face(c.contact.face)?,
                },
                record: at(c.record),
            });
        }
        for c in ff {
            records.ff.push(super::CarriedRecord {
                contact: super::FfContact {
                    a: face(c.contact.a)?,
                    b: face(c.contact.b)?,
                },
                record: at(c.record),
            });
        }
        Ok(())
    }

    /// **The tool's records `riding` as records of the material**, each
    /// between two of its solids the fold keeps apart: its cells read
    /// forward to the material's keys, the record cited as the tool's.
    /// The material's records are `carried`, its own list carried in;
    /// these follow them, and [`Self::records`] learns their places.
    ///
    /// # Errors
    ///
    /// [`BooleanError::ToolSeatUnread`] for a record a cell of which is
    /// no one cell of the material: the fold divided or consumed it.
    pub(super) fn onto_material<T: geom_core::Real>(
        &mut self,
        riding: &OperandRecords,
        carried: OperandRecords,
        fused: &Body<T>,
    ) -> Result<OperandRecords, BooleanError> {
        fn forward<K: Copy + Ord>(
            map: &BTreeMap<K, K>,
            live: impl Fn(K) -> bool,
        ) -> BTreeMap<K, K> {
            map.iter()
                .filter(|&(&u, _)| live(u))
                .map(|(&u, &t)| (t, u))
                .collect()
        }
        let faces = forward(&self.faces, |f| fused.get_face(f).is_some());
        let edges = forward(&self.edges, |e| fused.get_edge(e).is_some());
        let vertices = forward(&self.vertices, |v| fused.get_vertex(v).is_some());
        let unread = |record, cells| BooleanError::ToolSeatUnread { record, cells };
        let face = |f| faces.get(&f).copied();
        let edge = |e| edges.get(&e).copied();
        let vertex = |v| vertices.get(&v).copied();
        let mut out = carried;
        let OperandRecords {
            vv,
            vf,
            ve,
            ee,
            ef,
            ff,
        } = riding;
        let mut place = |record: u32| {
            self.records.push(vec![record]);
            u32::try_from(self.records.len() - 1)
                .unwrap_or_else(|_| unreachable!("a record list's place fits its index"))
        };
        use Cell::{Edge, Face, Vertex};
        for c in vv {
            let (x, y) = (c.contact.a, c.contact.b);
            let (Some(a), Some(b)) = (vertex(x), vertex(y)) else {
                return Err(unread(c.record, [Vertex(x), Vertex(y)]));
            };
            out.vv.push(super::CarriedRecord {
                contact: super::VvContact { a, b },
                record: place(c.record),
            });
        }
        for c in vf {
            let (x, y) = (c.contact.vertex, c.contact.face);
            let (Some(vertex), Some(face)) = (vertex(x), face(y)) else {
                return Err(unread(c.record, [Vertex(x), Face(y)]));
            };
            out.vf.push(super::CarriedRecord {
                contact: super::VfContact { vertex, face },
                record: place(c.record),
            });
        }
        for c in ve {
            let (x, y) = (c.contact.vertex, c.contact.edge);
            let (Some(vertex), Some(edge)) = (vertex(x), edge(y)) else {
                return Err(unread(c.record, [Vertex(x), Edge(y)]));
            };
            out.ve.push(super::CarriedRecord {
                contact: super::VeContact { vertex, edge },
                record: place(c.record),
            });
        }
        for c in ee {
            let (x, y) = (c.contact.a, c.contact.b);
            let (Some(a), Some(b)) = (edge(x), edge(y)) else {
                return Err(unread(c.record, [Edge(x), Edge(y)]));
            };
            out.ee.push(super::CarriedRecord {
                contact: super::EeContact { a, b },
                record: place(c.record),
            });
        }
        for c in ef {
            let (x, y) = (c.contact.edge, c.contact.face);
            let (Some(edge), Some(face)) = (edge(x), face(y)) else {
                return Err(unread(c.record, [Edge(x), Face(y)]));
            };
            out.ef.push(super::CarriedRecord {
                contact: super::EfContact { edge, face },
                record: place(c.record),
            });
        }
        if let Some(c) = ff.first() {
            unreachable!(
                "face–face record {} rides between two solids the fold keeps apart, though a \
                 face–face record between two solids is a verdict or refuses",
                c.record
            );
        }
        Ok(out)
    }

    /// The tool's key of a B-clone face of the cut that is one face of
    /// the tool; any other key comes back as it is. That other key is a
    /// face the union or the cut minted: a key fresh in the material's
    /// arena, or in its clone, which no key of the tool equals (the
    /// material's arena is carved from the tool's, so a slot the tool
    /// used comes back only under a later version), and whose lineage is
    /// a division row ([`Self::pieces`], the cut's own).
    fn face(&self, f: FaceKey) -> FaceKey {
        self.faces.get(&f).copied().unwrap_or(f)
    }

    /// As [`Self::face`], for an edge: a minted one's lineage is a hop
    /// ([`Self::hops`], the cut's own) or a seam ([`Self::seams`]).
    fn edge(&self, e: EdgeKey) -> EdgeKey {
        self.edges.get(&e).copied().unwrap_or(e)
    }

    /// As [`Self::face`], for a vertex: a minted one is a crossing
    /// ([`Self::crossings`]) or the cut's own.
    fn vertex(&self, v: VertexKey) -> VertexKey {
        self.vertices.get(&v).copied().unwrap_or(v)
    }

    /// The tool's cell an operand-B cell of the cut stands for, as a row
    /// names it: a face the union minted is the tool's face it is a
    /// piece of, an edge it split the edge it was split from, a seam it
    /// split off an edge that edge, and any other seam the face of the
    /// tool on one side of it, `.1` the face on the other; a vertex it
    /// minted where two of the tool's edges cross, the first of them,
    /// `.1` the second and `.2` the rest. `None` for a cell of the material none of these reads.
    fn read(&self, cell: Cell) -> Option<(Cell, Option<Cell>, Vec<Cell>)> {
        Some(match cell {
            Cell::Face(f) => (
                Cell::Face(self.pieces.get(&f).copied().unwrap_or_else(|| self.face(f))),
                None,
                Vec::new(),
            ),
            Cell::Edge(e) => {
                let mut at = e;
                while let Some(&up) = self.hops.get(&at) {
                    at = up;
                }
                if let Some(&[x, y]) = self.seam_faces.get(&at) {
                    return Some((Cell::Face(x), Some(Cell::Face(y)), Vec::new()));
                }
                let t = match self.seam_edges.get(&at) {
                    Some(&t) => t,
                    None => self.edge(at),
                };
                (Cell::Edge(t), None, Vec::new())
            }
            Cell::Vertex(v) => match self.crossings.get(&v).map(Vec::as_slice) {
                Some([first, second, rest @ ..]) => (
                    Cell::Edge(*first),
                    Some(Cell::Edge(*second)),
                    rest.iter().map(|&e| Cell::Edge(e)).collect(),
                ),
                Some(_) => return None,
                None => (Cell::Vertex(self.vertex(v)), None, Vec::new()),
            },
        })
    }

    /// **`out`, the cut by the material, stated as the cut by the tool**
    /// (module docs).
    ///
    /// # Errors
    ///
    /// [`BooleanError::JoinDesync`] where a row or a holding names a cell
    /// of the material that is no cell of `tool`, or a record cites one of
    /// the material's records that carries none of the tool's.
    pub(super) fn restate<T: AtRestPolicy>(
        &self,
        out: &mut BooleanBody<T>,
        tool: &Body<T>,
    ) -> Result<(), BooleanError> {
        let unread = || BooleanError::JoinDesync { what: UNREAD };
        let b = Operand::B;
        // Every field of the naming, by the key space it is in: a field in
        // result keys, or in A's clone keys, reads nothing of the tool and
        // is left as it is; every field in B's clone keys is read back
        // below, or refused where it holds a row.
        let super::BooleanNaming {
            a_keys: _,
            b_keys: _,
            graft_vertices: _,
            graft_edges: _,
            graft_dead_edges: _,
            graft_faces: _,
            seam_edges: _,
            vertex_merges: _,
            merge_groups: _,
            substitutions: _,
            merge_skipped: _,
            face_fragments_a: _,
            face_fragments_b: _,
            reduction_contacts: _,
            discards: _,
            substituted: _,
            uncut: _,
            edge_classes: _,
            unread_partners: _,
            chart_cuts: _,
            recharted,
            null_copies: _,
            cone_splits: _,
            edge_joins: _,
            killed_lineage: _,
            carved_lineage: _,
            fused_seams: _,
        } = &out.naming;
        // A sphere shell of the material the cut re-charted holds cells
        // under keys its re-chart minted, which the read-back has no
        // lineage for.
        if !recharted[1].is_empty() {
            return Err(BooleanError::JoinDesync {
                what: "a cut by its tool's material re-charted a sphere shell of the material, \
                       whose turned cells read back to no cell of the tool",
            });
        }
        let n = &mut out.naming;
        for (s, _) in &mut n.graft_faces {
            *s = self.face(*s);
        }
        for (s, _) in n.graft_edges.iter_mut().chain(&mut n.graft_dead_edges) {
            *s = self.edge(*s);
        }
        for (s, _) in &mut n.graft_vertices {
            *s = self.vertex(*s);
        }
        // The union's divisions come before the cut's.
        n.face_fragments_b = crate::lineage::FaceDivisions::from_rows(
            self.pieces.iter().map(|(&f, &t)| (f, t)).chain(
                n.face_fragments_b
                    .rows()
                    .iter()
                    .map(|&(new, from)| (new, self.face(from))),
            ),
        )?;
        for row in &mut n.discards {
            if row.operand == b {
                row.face = self.face(row.face);
                for chain in &mut row.boundary_chains {
                    for e in chain {
                        *e = self.edge(*e);
                    }
                }
            }
        }
        for row in &mut n.substituted {
            if row.operand == b {
                row.face = self.face(row.face);
            }
        }
        for row in &mut n.uncut {
            if row.operand == b {
                row.face = self.face(row.face);
            }
        }
        for row in &mut n.edge_classes {
            if row.operand == b {
                row.vertex = self.vertex(row.vertex);
                row.edge = self.edge(row.edge);
            }
        }
        for row in &mut n.unread_partners {
            if row.operand == b {
                row.vertex = self.vertex(row.vertex);
                row.partner = self.vertex(row.partner);
            }
        }
        for (operand, x, y) in &mut n.null_copies {
            if *operand == b {
                *x = self.vertex(*x);
                *y = self.vertex(*y);
            }
        }
        for e in &mut n.chart_cuts[1] {
            *e = self.edge(*e);
        }
        n.fused_seams.clone_from(&self.seams);
        let carved = core::mem::take(&mut n.carved_lineage[1]);
        n.carved_lineage[1] = self
            .hops
            .iter()
            .map(|(&e, &up)| (e, self.edge(up)))
            .chain(
                carved
                    .into_iter()
                    .map(|(e, up)| (self.edge(e), self.edge(up))),
            )
            .collect();
        let rc = &mut n.reduction_contacts;
        for c in &mut rc.vv {
            c.record.b = self.vertex(c.record.b);
        }
        for c in &mut rc.a_on_b {
            c.record.face = self.face(c.record.face);
        }
        for c in &mut rc.b_on_a {
            c.record.vertex = self.vertex(c.record.vertex);
        }
        for c in &mut rc.ee {
            c.record.b = self.edge(c.record.b);
        }
        for c in &mut rc.ff {
            c.record.b = self.face(c.record.b);
        }
        for c in &mut rc.a_in_b {
            c.record.face = self.face(c.record.face);
        }
        for c in &mut rc.b_in_a {
            c.record.edge = self.edge(c.record.edge);
        }
        for v in &mut out.verdicts {
            v.b = match self.read(Cell::Face(v.b)) {
                Some((Cell::Face(f), None, _)) => f,
                _ => return Err(unread()),
            };
        }
        for row in &mut out.coincidences {
            self.respell(row, tool)?;
        }
        out.holdings = self.holdings(&out.holdings);
        if out
            .holdings
            .named()
            .iter()
            .any(|&(place, f)| place == 1 && tool.get_face(f).is_none())
        {
            return Err(unread());
        }
        let records = &self.records;
        out.contacts = core::mem::take(&mut out.contacts).try_recited(
            |backing| match backing {
                Backing::Carried { input: 1, record } => records
                    .get(record as usize)
                    .map(|tool| {
                        tool.iter()
                            .map(|&record| Backing::Carried { input: 1, record })
                            .collect()
                    })
                    .ok_or_else(unread),
                other => Ok(vec![other]),
            },
            |_| unread(),
        )?;
        Ok(())
    }

    /// **`row`, a row of the cut by the material, spelled in the tool's
    /// cells** ([`Self::read`]) through the one door a decision's cells
    /// pass to become a row's ([`super::ops::spell`]): a point decided on
    /// a vertex the union minted where two of the tool's edges cross is
    /// stated as the meeting of those edges.
    ///
    /// # Errors
    ///
    /// [`BooleanError::JoinDesync`] for a cell of the material no reading
    /// takes to a cell of `tool`; `spell`'s.
    fn respell<T: geom_core::Real>(
        &self,
        row: &mut crate::Coincidence,
        tool: &Body<T>,
    ) -> Result<(), BooleanError> {
        let unread = || BooleanError::JoinDesync { what: UNREAD };
        let end = |cell: RowCell| match cell {
            RowCell::Input { input, cell } => Some((input, cell)),
            RowCell::Tool | RowCell::Result { .. } => None,
        };
        let names_b = row.every_cell().any(|c| {
            matches!(
                c,
                RowCell::Input {
                    input: Operand::B,
                    ..
                }
            )
        });
        if !names_b {
            return Ok(());
        }
        let [Some(x), Some(y)] = row.cells.map(end) else {
            return Err(unread());
        };
        let mut through: Vec<super::ops::End> = row
            .more
            .into_iter()
            .flatten()
            .map(|c| end(c).ok_or_else(unread))
            .collect::<Result<_, _>>()?;
        // Each B cell's reading, every cell it names held by the tool.
        let mut reads: BTreeMap<super::ops::End, super::ops::Read> = BTreeMap::new();
        for e in [x, y].into_iter().chain(through.clone()) {
            let (input, cell) = e;
            if input != Operand::B {
                continue;
            }
            let (at, by, rest) = self.read(cell).ok_or_else(unread)?;
            if [at]
                .into_iter()
                .chain(by)
                .chain(rest.iter().copied())
                .any(|c| !super::records::holds(tool, c))
            {
                return Err(unread());
            }
            through.extend(rest.into_iter().map(|c| (Operand::B, c)));
            reads.insert(e, ((Operand::B, at), by.map(|c| (Operand::B, c))));
        }
        let read = |e: super::ops::End| reads.get(&e).copied().unwrap_or((e, None));
        (row.cells, row.more, row.relation) =
            super::ops::spell([x, y], &through, row.relation, read)?;
        Ok(())
    }

    /// `held`, each operand-B face the tool's faces the material's face
    /// holds.
    fn holdings(&self, held: &Holdings) -> Holdings {
        let one = |h: &Held| match h {
            Held::Faces(faces) => Held::Faces(
                faces
                    .iter()
                    .flat_map(|&(place, f)| match (place, self.held.get(&f)) {
                        (1, Some(tool)) => tool.iter().map(|&t| (1, t)).collect::<Vec<_>>(),
                        _ => vec![(place, f)],
                    })
                    .collect(),
            ),
            refused @ Held::Refused(_) => refused.clone(),
        };
        Holdings::from_parts(
            held.faces().map(|(f, h)| (f, one(h))).collect(),
            held.lost_rows().iter().map(one).collect(),
            held.uncut_rows().iter().map(one).collect(),
        )
    }
}

/// The last step's holdings, in member faces: each member face is the
/// tool's face.
fn last_holdings<T: AtRestPolicy>(material: &ToolMaterial<T>) -> &Holdings {
    &material
        .steps
        .last()
        .unwrap_or_else(|| unreachable!("a tool of two or more solids folds at least once"))
        .holdings
}
