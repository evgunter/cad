//! **A cut reads its tool as material** (`crates/topo/README.md`,
//! `[topo.cut-material]`): `t − (p ⊔ q)` is computed as
//! `t − (p ∪ q)`. Where the tool's solids rest on each other, the cut's
//! door unites them on the face–face records between them, each record's
//! meaning the union's given verdict ([`super::nary`]'s tool door), and
//! the ordinary cut runs against that material.
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

use geom_core::{Bounds, Decide, Tol};

use super::holdings::{Held, Holdings};
use super::nary::{ToolMaterial, tool_material};
use super::ops::{BooleanBody, OperandKeys, boolean_op_given};
use super::{BooleanError, BooleanInputs, BooleanOp, BooleanResult, Cell, Operand, SweepStrategy};
use crate::body::Body;
use crate::coincidence::{Backing, RowCell};
use crate::entity::{EdgeKey, FaceKey, VertexKey};
use crate::props::AtRestPolicy;
use crate::provenance::Provenance;

/// **`a − b`, `b` read as its material**, where `b`'s solids rest on each
/// other ([`tool_material`]); `None` where they do not, and the cut
/// reads `b` as it stands.
///
/// # Errors
///
/// The tool door's refusals; the cut's own; [`BooleanError::JoinDesync`]
/// where a cell or record of the material reads back to no cell or
/// record of the tool, which every one does.
pub(super) fn cut_by_material<T: Decide + Bounds + AtRestPolicy>(
    a: &T::Finished,
    b: &T::Finished,
    inputs: &BooleanInputs,
    strategy: SweepStrategy,
    tol: Tol,
) -> Result<Option<BooleanResult<T>>, BooleanError> {
    let Some(material) = tool_material::<T>(b, &inputs.records_b, strategy, tol)? else {
        return Ok(None);
    };
    let through = Through::of(&material)?;
    let fused = material.body();
    let cut = boolean_op_given(
        BooleanOp::Subtract,
        a,
        &fused.body,
        &BooleanInputs {
            records_a: inputs.records_a.clone(),
            records_b: fused.contacts.carried(),
        },
        (None, None),
        strategy,
        tol,
    )?;
    let BooleanResult::Body(mut out) = cut else {
        return Ok(Some(cut));
    };
    through.restate(&mut out, b)?;
    Ok(Some(BooleanResult::Body(out)))
}

/// What a fold refused to read back, for [`BooleanError::JoinDesync`].
const UNREAD: &str = "a cell or record of a cut's material reads back to no cell or record of its \
                      tool";

/// **The material's cells and records, read back to the tool's**: each
/// key of the material that is one cell of the tool, mapped to that
/// cell's key; each face the union minted, to the tool's face it is a
/// piece of; each edge the union split, to the edge it was split from;
/// each record, to the tool's records it carries.
struct Through {
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
    /// Each record of the material, by place → the tool's records it
    /// carries.
    records: Vec<Vec<u32>>,
}

impl Through {
    /// The read-back of `material`'s last body, through its steps.
    fn of<T: AtRestPolicy>(material: &ToolMaterial<T>) -> Result<Self, BooleanError> {
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
                _ => return Err(unread()),
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
        Ok(Self {
            faces,
            edges,
            vertices,
            pieces,
            hops,
            held: held_by,
            seams,
            records: acc_records.unwrap_or_default(),
        })
    }

    fn face(&self, f: FaceKey) -> FaceKey {
        self.faces.get(&f).copied().unwrap_or(f)
    }

    fn edge(&self, e: EdgeKey) -> EdgeKey {
        self.edges.get(&e).copied().unwrap_or(e)
    }

    fn vertex(&self, v: VertexKey) -> VertexKey {
        self.vertices.get(&v).copied().unwrap_or(v)
    }

    /// The tool's cell an operand-B cell of the cut stands for, as a row
    /// names it: a face the union minted is the tool's face it is a
    /// piece of, an edge it split the edge it was split from.
    fn input_cell(&self, cell: Cell) -> Cell {
        match cell {
            Cell::Face(f) => {
                Cell::Face(self.pieces.get(&f).copied().unwrap_or_else(|| self.face(f)))
            }
            Cell::Edge(e) => {
                let mut at = e;
                while let Some(&up) = self.hops.get(&at) {
                    at = up;
                }
                Cell::Edge(self.edge(at))
            }
            Cell::Vertex(v) => Cell::Vertex(self.vertex(v)),
        }
    }

    /// **`out`, the cut by the material, stated as the cut by the tool**
    /// (module docs).
    ///
    /// # Errors
    ///
    /// [`BooleanError::JoinDesync`] where a row or a holding names a cell
    /// of the material that is no cell of `tool`, or a record cites one of
    /// the material's records that carries none of the tool's.
    fn restate<T: AtRestPolicy>(
        &self,
        out: &mut BooleanBody<T>,
        tool: &Body<T>,
    ) -> Result<(), BooleanError> {
        let unread = || BooleanError::JoinDesync { what: UNREAD };
        let b = Operand::B;
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
            .map(|(&e, &up)| (e, up))
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
            v.b = self.input_cell(Cell::Face(v.b)).face().ok_or_else(unread)?;
        }
        for row in &mut out.coincidences {
            for cell in row.cells.iter_mut().chain(row.more.iter_mut().flatten()) {
                if let RowCell::Input {
                    input: Operand::B,
                    cell,
                } = cell
                {
                    *cell = self.input_cell(*cell);
                    if !super::records::holds(tool, *cell) {
                        return Err(unread());
                    }
                }
            }
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

impl Cell {
    /// The face, where the cell is one.
    fn face(self) -> Option<FaceKey> {
        match self {
            Self::Face(f) => Some(f),
            Self::Edge(_) | Self::Vertex(_) => None,
        }
    }
}
