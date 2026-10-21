//! **The material decides the edges where coincident sheets meet on a
//! line** (`crates/topo/README.md`, tier 3: where more than two sheets
//! meet on a line, the material decides the edges, and no face passes
//! through an edge's material wedge). Two operations on the result build
//! what it decides:
//!
//! - **Parting** ([`part_riders`]) makes two edges of one, where an edge
//!   lies in a face whose material crosses its wedge.
//! - **Merging** ([`merge_slivers`]) makes one edge of two, where a face
//!   of no area runs between them and nothing else lies on their line.
//!
//! # Parting
//!
//! The join pairs each operand's own faces into an edge. Where an edge of
//! one operand lies in a face of the other and both its faces leave the
//! line on that face's material side (a V-trench whose keel rests on a
//! floor), the result's material about the line is two wedges, each
//! between the face and one of the edge's faces, and the edge as the join
//! leaves it claims the face's empty side as well. [`part_riders`] reads
//! every such edge of the result and builds two coincident distinct
//! edges, face | wall and wall | face, carried by the face as
//!
//! - a zero-area ring, where neither end lies on the face's boundary;
//! - a slit of the loop an end lies on, that end parted into one vertex
//!   per wedge;
//! - a slit of the loop a vertex on an end's point lies on, the two
//!   joined ([`Parted::fused`]);
//! - two faces, where both ends lie on one loop of the face: each end
//!   parts or joins, and the face parts along the line.
//!
//! An end joins rather than parts where it is not the face's own vertex
//! but another on its point, a vertex-vertex record between them. That
//! is where the edge runs on past the face ([`divide`]): the edge and
//! the face's boundary edge it crosses are each divided at the crossing
//! an edge-edge record names (the edge alone, where a vertex-on-edge
//! record puts a corner of the face on it), and the record becomes one
//! between the two vertices. The walls the edge runs on between join the
//! two wedges beyond the face, so the material there is one cone, and
//! one vertex.
//! The same holds where an edge lying wholly in the face ends at a
//! vertex its record puts on the face's corner: the two vertices'
//! cones are one once the line parts.
//!
//! Where it reads an edge whose material parts it and the parting is not
//! built, the Boolean refuses ([`EdgeInFaceGap`]) rather than leave the
//! one edge: an edge one of whose faces is tangent to the face leaves a
//! cusp per edge, whose reading on a line of coincident edges is not
//! built, and an arc leaves two coincident arcs the census does not read
//! ([`EdgeInFaceGap::Tangent`], [`EdgeInFaceGap::Curved`]). The reading
//! finds an edge only where its ends lie on the face, by structure or by
//! a record, or a record names the pair; an edge it does not find stays
//! one edge, and the result gate's line reading
//! ([`crate::census::line_wedges`]) refuses its overlapping wedges
//! instead.
//!
//! The parting is Euler surgery on the result. `mef` in the wall the
//! edge's plus half runs in mints the second edge, and the two-edge face
//! between them; `kfmrh` makes that face a ring of the floor, or, at an
//! end on the floor's boundary, a `mev_null` parts the end and `kef`
//! merges the two-edge face's loop into the floor's, as the zip parts a
//! pinch (`zip::split_cones`). At a joined end a zero-length joint fuses
//! the end into the vertex of the floor's loop (`zip::fuse_by_joint`):
//! `mekr` where the two lie on two loops, `mef` where on one, which
//! divides the floor. The two edges are then described on the faces they
//! bound.
//!
//! # Merging
//!
//! A sliver ([`sliver_halves`]) is a face with no rings whose outline is
//! two distinct straight edges: one segment run twice, a face of no area,
//! so no sheet. The join leaves one where it cuts a face flush with the
//! other operand along two coincident edges and keeps the piece between
//! them: across a slit whose face the other solid glues away
//! (`join::glued_slit_roles`, `join::resolve_roles_geometric`), and
//! along a pinch line two operands' faces meet on. [`merge_slivers`]
//! runs in the output stage, after the coplanar merge. Where no other
//! edge lies along the sliver's line, the only sheets there are the two
//! faces beyond its edges, and they pair into one edge between them:
//! `kef` kills one edge with the sliver, and the other is described on
//! the two faces. A sliver whose line holds another edge is left for the
//! result gate, whose line reading refuses its overlapping wedges.

use std::collections::{BTreeMap, BTreeSet};

use geom_core::{Band, Decide, Margin, Point3, Sign, Tol};

use super::ops::{PieceRead, piece_holds};
use super::{BooleanDecision, BooleanError, Coincide, DeclarationRead};
use crate::Cell;
use crate::body::Body;
use crate::boolean::records::{Contact, ContactKind, ContactRecords};
use crate::coincidence::Cited;
use crate::entity::{EdgeKey, EntityId, FaceKey, HalfEdgeKey, VertexKey};
use crate::euler::{FaceSurface, MefSite, MevSite};
use crate::live::proven;

/// Why an edge of the result lying in one of its faces is not parted
/// ([`BooleanError::EdgeInFace`]).
///
/// **NOT carried to the façade's curated list, and that is a decision
/// rather than an omission** (`scripts/payload-rung-sweep.py` names
/// this rung; the disposition table cites this paragraph as its home).
/// Every case but two is a construction gap with no lever, and
/// [`EdgeInFaceGap::Crossed`] and [`EdgeInFaceGap::TwinIncident`] are
/// kernel defects: nothing a person does
/// differs by case, and the refusal's sentence names it. A Rust caller
/// can already name and match it, since `pncad` re-exports `topo`
/// whole.
///
/// **The falsifier is a caller that acts on the case by itself**: a
/// driver that moves an end off the face's boundary when it reads
/// [`EdgeInFaceGap::EndOnBoundary`], say. Then carry it and delete this
/// paragraph and the disposition row with it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EdgeInFaceGap {
    /// One of the edge's own faces has no normal at the edge's midpoint
    /// to read its side of the face by.
    NoNormal,
    /// One of the edge's faces is tangent to the face along the line,
    /// so each of the two edges would bound a cusp, a wedge of no angle
    /// (D1's doubled cusp), whose reading on a line of coincident edges
    /// is not built.
    Tangent,
    /// The edge is an arc: the parting would leave two coincident arcs
    /// that no contact record backs
    /// (`work/cleave/a-curved-slit-fails-tier-3-prime-on-its-two-coincident-arcs.md`).
    Curved,
    /// An end the parting would divide into one vertex per wedge touches
    /// an edge's interior, and the census does not read that touch at
    /// the copies.
    DividedEndTouched,
    /// An end of the edge lies on the face's loops more than once, or
    /// the edge's faces go into the material of two faces it lies in.
    Shared,
    /// The edge lies in the face along part of its length and runs on
    /// past the face's boundary, and the division there is not built
    /// ([`divide`]): no record names a crossing with the face's
    /// boundary, a crossed edge is no line, a recorded vertex lies at a
    /// crossing, or another record names the edge or a crossed edge,
    /// which a piece would hold.
    Partly,
    /// An end of the edge touches the face's boundary inside one of its
    /// edges.
    EndOnBoundary,
    /// Both ends of the edge lie on one hole of the face, which the
    /// parting would divide into a hole and a piece of face.
    Pocket,
    /// A contact record names a face the parting divides, whose cell
    /// lies on one of its pieces.
    Recorded,
    /// The edge's two faces leave the line on opposite sides of the
    /// face: the face crosses the edge's material, which no Boolean
    /// result holds. A kernel defect.
    Crossed,
    /// A cell the parting minted, the edge's twin or a divided end's
    /// copy, is incident to the cell a record of its original names, so
    /// the twin cannot hold the record. The parting never makes that: a
    /// kernel defect.
    TwinIncident,
}

impl EdgeInFaceGap {
    /// The words a refusal says it in.
    pub(super) const fn words(self) -> &'static str {
        match self {
            Self::NoNormal => "one of the edge's faces has no normal there to read its side by",
            Self::Shared => {
                "an end of the edge lies on the face's boundary more than once, or the edge's \
                 faces go into the material of two faces it lies in"
            }
            Self::Tangent => {
                "one of the edge's faces is tangent to the face along it, so each of the two \
                 edges would bound a cusp, which the reading of a line of coincident edges does \
                 not cover"
            }
            Self::Curved => {
                "the edge is an arc, which would leave two coincident arcs that no contact record \
                 backs (`work/cleave/a-curved-slit-fails-tier-3-prime-on-its-two-coincident-arcs.md`)"
            }
            Self::DividedEndTouched => {
                "an end the parting would divide touches an edge, which the census does not read \
                 at the end's copies"
            }
            Self::Partly => {
                "the edge runs on past the face's boundary where no record names the crossing \
                 the parting would divide it at, or a record names the edge in a way the \
                 division does not place on a piece"
            }
            Self::EndOnBoundary => "an end of the edge touches the inside of an edge of the face",
            Self::Pocket => "both ends of the edge lie on one hole of the face",
            Self::Recorded => "a contact record names the face the parting would divide",
            Self::Crossed => "the edge's faces leave the line on opposite sides of the face",
            Self::TwinIncident => {
                "a cell the parting minted is incident to the cell a record of its original \
                 names"
            }
        }
    }
}

/// One edge the material parted ([`part_riders`]).
#[derive(Clone, Debug)]
pub(super) struct Parted {
    /// The edge as the join left it, now face | one wall.
    pub(super) kept: EdgeKey,
    /// Its coincident twin, face | the other wall.
    pub(super) minted: EdgeKey,
    /// The face it lay in.
    pub(super) face: FaceKey,
    /// The face the parting divided off `face`, where both ends lay on
    /// its boundary.
    pub(super) divided: Option<FaceKey>,
    /// The edge's ends, each joined end as the vertex it joined.
    pub(super) ends: (VertexKey, VertexKey),
    /// Each end the parting divided, as `(copy, end)`: one vertex per
    /// wedge, on the end's point key.
    pub(super) copies: Vec<(VertexKey, VertexKey)>,
    /// Each end the parting joined to the vertex of the face's loop on
    /// its point, as `(dead, kept)`.
    pub(super) fused: Vec<(VertexKey, VertexKey)>,
}

/// **Parts every edge of `body` lying in one of its faces whose material
/// both of the edge's faces leave the line into**, and returns the
/// records that hold after it: the edge-on-face and vertex-on-face
/// records the parting makes structure are dropped, and a record of the
/// edge, or of an end it divides, is held by the twin too.
///
/// An edge lies in a face `f` it does not bound where both its ends lie
/// on `f` — on its loops, or by a vertex-on-face record — or an
/// edge-on-face record names the pair, and the edge between them lies
/// in `f`: a line edge whose midpoint `f`, a plane, holds; a conic edge
/// whose plane is `f`'s and whose midpoint `f` holds. An edge whose
/// faces both leave the line on `f`'s empty side touches `f` from
/// outside its wedge, and stays one edge in contact with it.
///
/// # Errors
///
/// [`BooleanError::Escalated`] where a reading is in band;
/// [`BooleanError::EdgeInFace`] where the parting is not built
/// ([`EdgeInFaceGap`]); the Euler operators' and the re-description's
/// refusals.
pub(super) fn part_riders<T: Decide + crate::props::AtRestPolicy>(
    body: &mut Body<T>,
    contacts: &ContactRecords,
    band: Band,
    tol: Tol,
) -> Result<(Vec<Parted>, ContactRecords), BooleanError> {
    let mut parted = Vec::new();
    // Every edge read so far, parted or not.
    let mut read: BTreeSet<EdgeKey> = BTreeSet::new();
    let mut dropped: BTreeSet<[Cell; 2]> = BTreeSet::new();
    // Each parted edge and divided end, and its twin: every contact the
    // one holds the other holds too, at one point or along one line.
    let mut twins: Vec<(Cell, Cell, EdgeKey, FaceKey)> = Vec::new();
    let mut divided: BTreeSet<FaceKey> = BTreeSet::new();
    // The records as the divisions leave them: each crossing a
    // vertex-vertex record between its two vertices.
    let mut working = contacts.clone();
    // Each piece a division left: a piece read again as running past a
    // face refuses rather than divide twice.
    let mut pieces: BTreeSet<EdgeKey> = BTreeSet::new();
    // Each division's vertex on the edge at a crossing, with the edge
    // and face it was read on: each must be joined by a parting.
    let mut crossings: Vec<(VertexKey, EdgeKey, FaceKey)> = Vec::new();
    // Each joined end, dead, and the vertex it was joined to.
    let mut fused: BTreeMap<VertexKey, VertexKey> = BTreeMap::new();
    // Read again after each parting: a parting that divides a face
    // gives one of its pieces a new key, which a later edge lying in
    // that piece is read against. A record naming a divided face refuses
    // `Recorded` below.
    while let Some((edge, faces)) = riders(body, &working, band)?
        .into_iter()
        .find(|(e, _)| !read.contains(e))
    {
        read.insert(edge);
        let mut parting = Vec::new();
        for (face, partly) in faces {
            if let Some(tangent) = walls_inside(body, edge, face, band)? {
                parting.push((face, tangent, partly));
            }
        }
        let (face, flat, partly) = match parting[..] {
            [] => continue,
            [one] => one,
            [(face, ..), ..] => {
                return Err(BooleanError::EdgeInFace {
                    edge,
                    face,
                    gap: EdgeInFaceGap::Shared,
                });
            }
        };
        let refused = |gap| BooleanError::EdgeInFace { edge, face, gap };
        if flat {
            return Err(refused(EdgeInFaceGap::Tangent));
        }
        if !crate::census::edge_is_line(body, edge) {
            return Err(refused(EdgeInFaceGap::Curved));
        }
        if partly {
            if pieces.contains(&edge) {
                return Err(refused(EdgeInFaceGap::Partly));
            }
            let (made, crossed) = divide(body, &mut working, edge, face, band, tol)?;
            pieces.extend(made);
            crossings.extend(crossed.into_iter().map(|v| (v, edge, face)));
            // The edge's key holds its first piece, read again with the
            // others.
            read.remove(&edge);
            continue;
        }
        let (a, b) = crate::query::edge_ends(body, edge);
        if [a, b]
            .into_iter()
            .any(|v| touches_boundary_between(body, &working, v, face))
        {
            return Err(refused(EdgeInFaceGap::EndOnBoundary));
        }
        if [a, b]
            .into_iter()
            .any(|v| on_face_loops(body, face, v) && touches_an_edge(&working, v))
        {
            return Err(refused(EdgeInFaceGap::DividedEndTouched));
        }
        let mut joined = Vec::new();
        for v in [a, b] {
            if on_face_loops(body, face, v) {
                continue;
            }
            match joins(body, &working, v, face)[..] {
                [] => {}
                [w] => joined.push((v, w)),
                _ => return Err(refused(EdgeInFaceGap::Shared)),
            }
        }
        let p = part(body, edge, face, &joined, tol)?;
        let (a, b) = p.parted.ends;
        fused.extend(p.parted.fused.iter().copied());
        // A record's cells are in canonical order, the lesser first.
        for c in [Cell::Vertex(a), Cell::Vertex(b), Cell::Edge(edge)] {
            dropped.insert([c, Cell::Face(face)]);
        }
        twins.push((Cell::Edge(edge), Cell::Edge(p.parted.minted), edge, face));
        twins.extend(
            p.parted
                .copies
                .iter()
                .map(|&(copy, of)| (Cell::Vertex(of), Cell::Vertex(copy), edge, face)),
        );
        if p.parted.divided.is_some() {
            divided.insert(face);
        }
        parted.push(p.parted);
    }
    // A crossing no parting joined leaves its two vertices on one point
    // apart: no piece of the edge was read as lying in the face.
    if let Some(&(_, edge, face)) = crossings.iter().find(|(v, ..)| !fused.contains_key(v)) {
        return Err(BooleanError::EdgeInFace {
            edge,
            face,
            gap: EdgeInFaceGap::Partly,
        });
    }
    if parted.is_empty() {
        return Ok((parted, working));
    }
    let worklist: Vec<EdgeKey> = parted.iter().flat_map(|p| [p.kept, p.minted]).collect();
    super::ops::describe_edges(body, worklist, &[], band, tol).map_err(super::ops::of_describe)?;
    let shells: Vec<crate::entity::ShellKey> = body
        .solids()
        .flat_map(|(_, solid)| solid.shells.clone())
        .collect();
    for shell in shells {
        body.movefac(shell)?;
    }
    let mut kept = ContactRecords::default();
    for row in working.rows() {
        let [x, y] = row.record.cells();
        let settle = |c: Cell| match c {
            Cell::Vertex(v) => Cell::Vertex(fused.get(&v).copied().unwrap_or(v)),
            c => c,
        };
        // A record of a joined end is the kept vertex's; one the joint
        // made structure (the two vertices one, or one bounding the
        // other) is dropped.
        let record = if (settle(x), settle(y)) == (x, y) {
            row.record
        } else {
            match Contact::new(body, settle(x), settle(y)) {
                Ok(contact) => contact,
                Err(_) => continue,
            }
        };
        let cells = record.cells();
        if dropped.contains(&cells) {
            continue;
        }
        // A face the parting divided holds a record's cell on one of its
        // pieces, which nothing here reads.
        if let Some(&face) = divided.iter().find(|&&f| cells.contains(&Cell::Face(f))) {
            let edge = parted
                .iter()
                .find(|p| p.face == face)
                .map_or(parted[0].kept, |p| p.kept);
            return Err(BooleanError::EdgeInFace {
                edge,
                face,
                gap: EdgeInFaceGap::Recorded,
            });
        }
        kept.push(Cited::new(record, row.cites.clone()));
        for &(of, twin, edge, face) in &twins {
            let Some(k) = cells.iter().position(|&c| c == of) else {
                continue;
            };
            // The twin is a cell the parting minted: the copy of an end,
            // which takes none of the edges the record's other cell
            // touched in their interiors, or the second edge, on the
            // kept edge's ends or their copies, which bounds the floor
            // and a wall the kept edge bounded before. So it is neither
            // the other cell nor incident to it, and a refusal here is
            // the parting's own defect.
            let contact =
                Contact::new(body, twin, cells[1 - k]).map_err(|_| BooleanError::EdgeInFace {
                    edge,
                    face,
                    gap: EdgeInFaceGap::TwinIncident,
                })?;
            kept.push(Cited::new(contact, row.cites.clone()));
        }
    }
    Ok((parted, kept))
}

/// One sliver the output stage merged away ([`merge_slivers`]).
#[derive(Clone, Copy, Debug)]
pub(super) struct Sliver {
    /// The face of no area between the two edges.
    pub(super) face: FaceKey,
    /// The edge killed with it.
    pub(super) killed: EdgeKey,
    /// The one edge left between the two walls.
    pub(super) kept: EdgeKey,
}

impl Sliver {
    /// The lineage rows of the merge, `(dead, holder)`: the killed edge
    /// and the sliver lie along the kept edge.
    pub(super) fn substitutions(&self) -> [(Cell, Cell); 2] {
        [
            (Cell::Edge(self.killed), Cell::Edge(self.kept)),
            (Cell::Face(self.face), Cell::Edge(self.kept)),
        ]
    }
}

/// **The two halves of `face`'s outline where `face` is a sliver**: no
/// rings, and an outline of two distinct straight edges, one segment run
/// twice. `None` for any other face.
pub(super) fn sliver_halves<T: Decide>(body: &Body<T>, face: FaceKey) -> Option<[HalfEdgeKey; 2]> {
    if !body.get_face(face)?.rings.is_empty() {
        return None;
    }
    let [h, k] = face_halves(body, face)?[..] else {
        return None;
    };
    let edges = [h, k].map(|x| body.get_half_edge(x).map(|d| d.edge));
    (edges[0] != edges[1]
        && edges
            .iter()
            .all(|e| e.is_some_and(|e| crate::census::edge_is_line(body, e))))
    .then_some([h, k])
}

/// **Merges the two edges of each sliver alone on its line into one**
/// (module docs, *Merging*): for each sliver ([`sliver_halves`]) whose
/// two halves' mates lie in two distinct faces other than it, and whose
/// line holds no other edge ([`crate::census::line_holds_another_edge`]),
/// `kef` kills the second half's edge with the sliver, and the first
/// half's edge, now between the two faces, is described on them.
///
/// # Errors
///
/// [`BooleanError::Escalated`] ([`super::BooleanDecision::LineEdges`])
/// where whether an edge lies on the sliver's line is in band, as the
/// result gate's line reading escalates it; [`BooleanError::JoinDesync`]
/// where `kef` kills a face other than the sliver; the Euler operator's
/// and the re-description's refusals.
pub(super) fn merge_slivers<T: Decide + crate::props::AtRestPolicy>(
    body: &mut Body<T>,
    band: Band,
    tol: Tol,
) -> Result<Vec<Sliver>, BooleanError> {
    let mut slivers = Vec::new();
    let mut read: BTreeSet<FaceKey> = BTreeSet::new();
    loop {
        let next = body.faces().find_map(|(f, _)| {
            if read.contains(&f) {
                return None;
            }
            let halves = sliver_halves(body, f)?;
            let walls = halves.map(|x| body.mate(x).and_then(|m| body.face_of_half_edge(m)));
            match walls {
                [Some(a), Some(b)] if a != b && a != f && b != f => Some((f, halves)),
                _ => None,
            }
        });
        let Some((face, [keep, kill])) = next else {
            return Ok(slivers);
        };
        read.insert(face);
        let kept = proven(&body.half_edges, keep, EntityId::HalfEdge).edge;
        let killed = proven(&body.half_edges, kill, EntityId::HalfEdge).edge;
        let alone = !crate::census::line_holds_another_edge(body, [kept, killed], band)
            .map_err(super::ops::collinear_escalated)?;
        if !alone {
            continue;
        }
        let done = body.kef_minting(kill, tol)?;
        if done.killed_face != face || done.killed_edge != killed {
            return Err(BooleanError::JoinDesync {
                what: "the sliver's kef killed another face or edge",
            });
        }
        super::ops::describe_edges(body, [kept], &[], band, tol)
            .map_err(super::ops::of_describe)?;
        slivers.push(Sliver { face, killed, kept });
    }
}

/// A face an edge lies in, and whether the edge runs on past it
/// ([`riders`]).
type Holding = (FaceKey, bool);

/// Every edge of `body` lying in faces it does not bound, with those
/// faces ([`part_riders`]'s reading), in edge order, each with whether
/// the edge runs on past it: an edge-on-face record names the pair and
/// the edge's midpoint lies in the face, but an end of the edge does not.
/// An end lies on a face by structure, by a vertex-on-face record, or by
/// a vertex-vertex record with a vertex of the face's loops.
fn riders<T: Decide>(
    body: &Body<T>,
    contacts: &ContactRecords,
    band: Band,
) -> Result<Vec<(EdgeKey, Vec<Holding>)>, BooleanError> {
    let mut vf: BTreeSet<(VertexKey, FaceKey)> = BTreeSet::new();
    let mut ef: BTreeSet<(EdgeKey, FaceKey)> = BTreeSet::new();
    let mut vv: BTreeSet<(VertexKey, VertexKey)> = BTreeSet::new();
    for row in contacts.rows() {
        match row.record.kind() {
            ContactKind::VertexFace(v, f) => {
                vf.insert((v, f));
            }
            ContactKind::EdgeFace(e, f) => {
                ef.insert((e, f));
            }
            ContactKind::VertexVertex(v, w) => {
                vv.extend([(v, w), (w, v)]);
            }
            _ => {}
        }
    }
    let on = |v: VertexKey| -> BTreeSet<FaceKey> {
        let partners = vv.iter().filter(|&&(x, _)| x == v).map(|&(_, w)| w);
        core::iter::once(v)
            .chain(partners)
            .flat_map(|x| body.faces_of_vertex(x).unwrap_or_default())
            .chain(vf.iter().filter(|&&(w, _)| w == v).map(|&(_, f)| f))
            .collect()
    };
    let mut out = Vec::new();
    for (e, _) in body.edges() {
        let (a, b) = crate::query::edge_ends(body, e);
        if a == b {
            continue;
        }
        let Ok(sides) = crate::readback::edge_sides(body, e) else {
            continue;
        };
        let (fa, fb) = sides.faces();
        let both: BTreeSet<FaceKey> = on(a).intersection(&on(b)).copied().collect();
        let mut faces = both.clone();
        faces.extend(ef.iter().filter(|&&(g, _)| g == e).map(|&(_, f)| f));
        faces.remove(&fa);
        faces.remove(&fb);
        let mut holding = Vec::new();
        for f in faces {
            if lies_in(body, e, f, band)? {
                holding.push((f, !both.contains(&f)));
            }
        }
        if !holding.is_empty() {
            out.push((e, holding));
        }
    }
    Ok(out)
}

/// Whether edge `e`, both of whose ends lie on the plane face `f`, lies
/// in it: a line whose midpoint `f` holds (two points of a plane hold
/// the line between them), or a conic in `f`'s plane whose midpoint `f`
/// holds. A face that is no plane holds nothing here.
fn lies_in<T: Decide>(
    body: &Body<T>,
    e: EdgeKey,
    f: FaceKey,
    band: Band,
) -> Result<bool, BooleanError> {
    let Some(&geom::Surface::Plane { origin, normal, .. }) =
        body.get_face(f).and_then(|d| body.get_surface(d.surface))
    else {
        return Ok(false);
    };
    let Some(curve) = body
        .get_edge(e)
        .and_then(|d| body.get_curve_geom(d.curve))
        .and_then(|c| c.certified())
    else {
        return Ok(false);
    };
    let mid = match curve.carrier() {
        geom::Curve3::Line { .. } => curve.mid_point(),
        carrier @ (geom::Curve3::Circle { .. } | geom::Curve3::Ellipse { .. }) => {
            let (t0, t1) = curve.params();
            match crate::splitting::plane_crossing_lane(carrier, t0, t1, origin, normal, band) {
                crate::splitting::PlaneCrossingLane::Conic(
                    crate::splitting::ConicPlaneMeet::Parallel { offset },
                ) => match crate::validate::decide(
                    "bool_conic_face_plane_offset",
                    Margin::of(offset),
                    band,
                ) {
                    Ok(Sign::Zero) => curve.mid_point(),
                    Ok(Sign::Positive | Sign::Negative) => return Ok(false),
                    Err(diag) => {
                        return Err(BooleanError::coincidence(
                            Coincide::EdgeOnPlane,
                            DeclarationRead::Moot,
                            diag,
                        ));
                    }
                },
                _ => return Ok(false),
            }
        }
        _ => return Ok(false),
    };
    Ok(matches!(piece_holds(body, f, mid, band)?, PieceRead::In))
}

/// Whether both faces of edge `e` leave its line on the material side of
/// the face `f` it lies in (`bool_rider_wall_side`): at the edge's
/// midpoint, each face's direction into itself off the line (its
/// outward normal crossed with its half's tangent: a face lies to the
/// left of its half) against `f`'s outward normal, levered at the
/// edge's chord. `Some` where both go in, with whether either face is
/// tangent to `f` along the line; `None`
/// where both go out (the edge touches `f` from outside its wedge).
///
/// # Errors
///
/// [`BooleanError::EdgeInFace`] ([`EdgeInFaceGap::Crossed`]) where they
/// go to opposite sides, or ([`EdgeInFaceGap::NoNormal`]) where a face
/// has no normal there; [`BooleanError::Escalated`] where a side, or a
/// normal, is in band.
fn walls_inside<T: Decide>(
    body: &Body<T>,
    e: EdgeKey,
    f: FaceKey,
    band: Band,
) -> Result<Option<bool>, BooleanError> {
    let refused = |gap| BooleanError::EdgeInFace {
        edge: e,
        face: f,
        gap,
    };
    let desync = || BooleanError::JoinDesync {
        what: "an edge lying in a face does not resolve",
    };
    let edge = body.get_edge(e).ok_or_else(desync)?;
    let curve = body
        .get_curve_geom(edge.curve)
        .and_then(|c| c.certified())
        .ok_or_else(desync)?;
    let (t0, t1) = curve.params();
    let mid = curve.mid_point();
    let tangent = curve
        .carrier()
        .deriv((t0 + t1) * T::from_f64(0.5))
        .normalize();
    let floor = crate::face_normal::face_outward_normal(body, f)
        .ok_or_else(desync)?
        .vec();
    let mut inside = [false; 2];
    let mut flat = [false; 2];
    for (k, (he, dir)) in [(edge.he_plus, tangent), (edge.he_minus, -tangent)]
        .into_iter()
        .enumerate()
    {
        let wall = body.face_of_half_edge(he).ok_or_else(desync)?;
        let normal = match crate::face_normal::face_outward_normal_at(body, wall, mid, band) {
            Ok(Some(n)) => n.vec(),
            Ok(None) => return Err(desync()),
            Err(crate::face_normal::NormalAtError::Escalated { decision, diag }) => {
                return Err(BooleanError::Escalated {
                    decision: BooleanDecision::of_normal(decision),
                    diag,
                });
            }
            Err(_) => return Err(refused(EdgeInFaceGap::NoNormal)),
        };
        let into = normal.cross(dir);
        // How far the wall leaves the floor grows across the line: the
        // lever is the wall's own reach from the line's middle, as a
        // seam's dihedral reads it ([`crate::wedge_reach`]).
        let arm = crate::wedge_reach(body, [wall, wall], curve.carrier(), (t0, t1));
        let sectors =
            |diag| BooleanError::coincidence(Coincide::Sectors, DeclarationRead::Moot, diag);
        inside[k] = match crate::validate::decide(
            "bool_rider_wall_side",
            Margin::levered(into.dot(floor), arm),
            band,
        )
        .map_err(sectors)?
        {
            Sign::Negative => true,
            Sign::Positive => false,
            // Tangent to the face along the line: the side its sheet
            // lies on is the side it bends to, toward its centre of
            // curvature (`bool_rider_wall_bend`), in metres.
            Sign::Zero => {
                flat[k] = true;
                let surface = body
                    .get_face(wall)
                    .and_then(|d| body.get_surface(d.surface))
                    .ok_or_else(desync)?;
                let toward = match *surface {
                    geom::Surface::Cylinder { origin, axis, .. } => {
                        origin + axis * (mid - origin).dot(axis) - mid
                    }
                    geom::Surface::Cone { apex, axis, .. } => {
                        apex + axis * (mid - apex).dot(axis) - mid
                    }
                    geom::Surface::Sphere { center, .. } => center - mid,
                    // A plane wall tangent along the line lies in the
                    // floor's plane. Glued to the floor from outside
                    // (outward normals opposed), its material is across
                    // the plane from the floor's, and the edge keeps one
                    // edge, as D1 says; facing as the floor does, it
                    // would lie over the floor's own sheet. The two unit
                    // normals' cosine is levered at the wall's reach, as
                    // its side was, so the reading is in metres: how far
                    // the wall's normal, carried across the wall, stands
                    // off the floor's.
                    geom::Surface::Plane { .. } => {
                        match crate::validate::decide_nonzero(
                            "bool_rider_wall_coplanar",
                            Margin::levered(normal.dot(floor), arm),
                            band,
                        )
                        .map_err(sectors)?
                        {
                            geom_core::k_stats::NonzeroSign::Negative => {
                                inside[k] = false;
                                continue;
                            }
                            geom_core::k_stats::NonzeroSign::Positive => {
                                return Err(refused(EdgeInFaceGap::Tangent));
                            }
                        }
                    }
                    _ => return Err(refused(EdgeInFaceGap::Tangent)),
                };
                match crate::validate::decide_nonzero(
                    "bool_rider_wall_bend",
                    Margin::of(toward.dot(floor)),
                    band,
                )
                .map_err(sectors)?
                {
                    geom_core::k_stats::NonzeroSign::Negative => true,
                    geom_core::k_stats::NonzeroSign::Positive => false,
                }
            }
        };
    }
    match inside {
        [true, true] => Ok(Some(flat[0] || flat[1])),
        [false, false] => Ok(None),
        _ => Err(refused(EdgeInFaceGap::Crossed)),
    }
}

/// The surgery of one parting.
struct Parting {
    parted: Parted,
}

/// **Parts edge `edge` lying in face `face`**, joining each end of
/// `joined` to the vertex of `face`'s loops paired with it, as
/// `(end, vertex)` ([`part_riders`], module docs for the sequence).
fn part<T: Decide + crate::props::AtRestPolicy>(
    body: &mut Body<T>,
    edge: EdgeKey,
    face: FaceKey,
    joined: &[(VertexKey, VertexKey)],
    tol: Tol,
) -> Result<Parting, BooleanError> {
    let refused = |gap| BooleanError::EdgeInFace { edge, face, gap };
    let desync = || BooleanError::JoinDesync {
        what: "an edge lying in a face does not resolve",
    };
    let data = body.get_edge(edge).ok_or_else(desync)?;
    let h = data.he_plus;
    let curve = body
        .get_curve_geom(data.curve)
        .and_then(|c| c.certified())
        .cloned();
    let (a, b) = crate::query::edge_ends(body, edge);
    let wall = body.face_of_half_edge(h).ok_or_else(desync)?;
    let surfaces = [face, wall].map(|f| body.get_face(f).map(|d| d.surface));
    let [Some(floor_surface), Some(wall_surface)] = surfaces else {
        return Err(desync());
    };
    let (pa, pb) = crate::readback::vertex_point(body, a)
        .ok()
        .zip(crate::readback::vertex_point(body, b).ok())
        .ok_or_else(desync)?;
    // A line edge (an arc refuses `Curved` before parting): the draft
    // draws the chord between the ends, its own spec.
    let geom_brep::DraftedIntersection::Chord(spec) =
        geom_brep::IntersectionDraft::of(curve.as_ref(), pa, pb)
            .into_intersection(geom_brep::SurfacePair::new(floor_surface, wall_surface))
    else {
        return Err(refused(EdgeInFaceGap::Curved));
    };
    let next = proven(&body.half_edges, h, EntityId::HalfEdge).next;
    let made = body.mef(
        MefSite::Chords { he1: h, he2: next },
        spec,
        FaceSurface::Inherit,
        tol,
    )?;
    // The two-edge face: `h` from `a` to `b`, the minted minus half back.
    let gusset = made.face;
    let tip_out = |v: VertexKey| if v == a { h } else { made.he_minus };
    let on_loops = |body: &Body<T>, v: VertexKey| -> Result<Vec<HalfEdgeKey>, BooleanError> {
        let leaving: Vec<HalfEdgeKey> = face_halves(body, face)
            .ok_or_else(desync)?
            .into_iter()
            .filter(|&he| proven(&body.half_edges, he, EntityId::HalfEdge).start == v)
            .collect();
        Ok(leaving)
    };
    let mut boundary: Vec<(VertexKey, HalfEdgeKey)> = Vec::new();
    for v in [a, b] {
        match on_loops(body, v)?.as_slice() {
            [] => {}
            [he] => boundary.push((v, *he)),
            _ => return Err(refused(EdgeInFaceGap::Shared)),
        }
    }
    let mut copies = Vec::new();
    let mut divided = None;
    match boundary.as_slice() {
        [] => {
            body.kfmrh_minting(face, gusset, tol)?;
        }
        [(v, out), rest @ ..] => {
            let made_v = body.mev_null(
                MevSite::Fan {
                    he1: *out,
                    he2: tip_out(*v),
                },
                crate::null::NewVertexSide::Above,
            )?;
            copies.push((made_v.vertex, *v));
            body.kef_minting(made_v.he_minus, tol)?;
            if let [(w, out_w)] = rest {
                let outer = body.get_face(face).ok_or_else(desync)?.outer;
                let made_w = body.mev_null(
                    MevSite::Fan {
                        he1: *out_w,
                        he2: tip_out(*w),
                    },
                    crate::null::NewVertexSide::Above,
                )?;
                copies.push((made_w.vertex, *w));
                let loop_of =
                    |he: HalfEdgeKey| proven(&body.half_edges, he, EntityId::HalfEdge).parent_loop;
                let (lp, lm) = (loop_of(made_w.he_plus), loop_of(made_w.he_minus));
                if lp == lm {
                    if lp != outer {
                        return Err(refused(EdgeInFaceGap::Pocket));
                    }
                    let sense = body.get_face(face).ok_or_else(desync)?.sense;
                    let ring = body
                        .kemr_minting(made_w.he_plus, made_w.he_minus, tol)?
                        .ring;
                    let piece = body.mfkrh_minting(ring, FaceSurface::Inherit, tol)?.face;
                    // The piece is the face's own region on its own chart,
                    // not a hole of it promoted: it keeps the face's side.
                    body.set_face_sense(piece, sense)?;
                    divided = Some(piece);
                } else {
                    let (ring, he) = if lp == outer {
                        (lm, made_w.he_minus)
                    } else {
                        (lp, made_w.he_plus)
                    };
                    body.mfkrh_minting(ring, FaceSurface::Inherit, tol)?;
                    body.kef_minting(he, tol)?;
                }
            }
        }
    }
    // Each joined end: a joint from the face's vertex to the end, across
    // the two loops they lie on or, the last of a slit both of whose ends
    // reach the face's outline, across the one, which parts the face.
    let mut fused = Vec::new();
    for &(v, w) in joined {
        let leaving = |body: &Body<T>, x: VertexKey| match on_loops(body, x)?.as_slice() {
            [he] => Ok(*he),
            _ => Err(refused(EdgeInFaceGap::Shared)),
        };
        let (hw, hv) = (leaving(body, w)?, leaving(body, v)?);
        let loop_of =
            |he: HalfEdgeKey| proven(&body.half_edges, he, EntityId::HalfEdge).parent_loop;
        let outer = body.get_face(face).ok_or_else(desync)?.outer;
        let joint = if loop_of(hw) == loop_of(hv) {
            if loop_of(hw) != outer {
                return Err(refused(EdgeInFaceGap::Pocket));
            }
            super::zip::Joint::Chord { he1: hw, he2: hv }
        } else if loop_of(hv) == outer {
            super::zip::Joint::Loops {
                target: hv,
                ring: hw,
            }
        } else {
            super::zip::Joint::Loops {
                target: hw,
                ring: hv,
            }
        };
        let p = crate::readback::vertex_point(body, w).map_err(|_| desync())?;
        fused.push(super::zip::fuse_by_joint(
            body,
            joint,
            (p, w),
            joint_desync,
            tol,
        )?);
    }
    // A joint across one loop parted the face: the line's two sides lie
    // in its two pieces.
    let floor_of = |he: HalfEdgeKey| body.face_of_half_edge(he).ok_or_else(desync);
    let sides = [floor_of(h)?, floor_of(made.he_minus)?];
    if divided.is_none() && sides[0] != sides[1] {
        divided = sides.into_iter().find(|&f| f != face);
    }
    let settle = |v: VertexKey| {
        fused
            .iter()
            .find(|&&(dead, _)| dead == v)
            .map_or(v, |&(_, kept)| kept)
    };
    Ok(Parting {
        parted: Parted {
            kept: edge,
            minted: made.edge,
            face,
            divided,
            ends: (settle(a), settle(b)),
            copies,
            fused,
        },
    })
}

/// **Divides `edge`, lying in `face` along part of its length, and each
/// edge of `face`'s loops it crosses**, at each crossing an edge-edge
/// record of the two names: one point, read once off the two lines and
/// held by both new vertices, so the joint that fuses them
/// ([`part`]) certifies. Where it crosses at a vertex of the loops, a
/// vertex-on-edge record names it, and the edge alone is divided, at the
/// vertex's own point. Each such record becomes a vertex-vertex record
/// of its two vertices, citing what it cited; a vertex-on-edge
/// record of a divided edge goes to the piece holding the vertex, read
/// against the crossings along the line; and the edge's edge-on-face
/// record with `face` is dropped: the piece lying in `face` has both
/// ends on it, and [`riders`] reads it by them. Returns the edge's
/// pieces and its vertices at the crossings.
///
/// # Errors
///
/// [`EdgeInFaceGap::Partly`] where no record names a crossing with the
/// face's boundary, a crossed edge is no line, a recorded vertex lies at
/// a crossing, or a record other than these names the edge or a crossed
/// edge; [`BooleanError::Escalated`] where an order along the line is in
/// band; the split's refusals, a crossing outside either edge's span
/// among them.
fn divide<T: Decide + crate::props::AtRestPolicy>(
    body: &mut Body<T>,
    working: &mut ContactRecords,
    edge: EdgeKey,
    face: FaceKey,
    band: Band,
    tol: Tol,
) -> Result<(Vec<EdgeKey>, Vec<VertexKey>), BooleanError> {
    let refused = || BooleanError::EdgeInFace {
        edge,
        face,
        gap: EdgeInFaceGap::Partly,
    };
    let desync = || BooleanError::JoinDesync {
        what: "an edge running past a face does not resolve",
    };
    // The sign of `a − b` along a line, as a split orders two crossings
    // on an edge; one point divides nothing between them.
    let order = |a: T, b: T| match crate::validate::decide(
        "split_conic_root_order",
        Margin::of(a - b),
        band,
    ) {
        Ok(Sign::Zero) => Err(refused()),
        Ok(sign) => Ok(sign == Sign::Positive),
        Err(diag) => Err(BooleanError::Escalated {
            decision: BooleanDecision::Crossing(crate::splitting::CrossingDecision::Order),
            diag,
        }),
    };
    let boundary: BTreeSet<EdgeKey> = face_halves(body, face)
        .ok_or_else(desync)?
        .into_iter()
        .filter_map(|he| body.get_half_edge(he).map(|h| h.edge))
        .collect();
    // Each crossing's record, and the cell of the loops crossed.
    let mut crossed: Vec<(usize, Cell)> = Vec::new();
    for (k, row) in working.rows().iter().enumerate() {
        match row.record.kind() {
            ContactKind::EdgeEdge(x, g) | ContactKind::EdgeEdge(g, x)
                if x == edge && boundary.contains(&g) =>
            {
                crossed.push((k, Cell::Edge(g)));
            }
            ContactKind::VertexEdge(w, x) if x == edge && on_face_loops(body, face, w) => {
                crossed.push((k, Cell::Vertex(w)));
            }
            _ => {}
        }
    }
    if crossed.is_empty() {
        return Err(refused());
    }
    let divided: BTreeSet<EdgeKey> = crossed
        .iter()
        .filter_map(|&(_, c)| match c {
            Cell::Edge(g) => Some(g),
            _ => None,
        })
        .chain([edge])
        .collect();
    // Each vertex a record puts inside a divided edge, by the record.
    let mut touching: Vec<(usize, VertexKey, EdgeKey)> = Vec::new();
    for (k, row) in working.rows().iter().enumerate() {
        match row.record.kind() {
            _ if crossed.iter().any(|&(c, _)| c == k) => {}
            ContactKind::VertexEdge(v, e) if divided.contains(&e) => touching.push((k, v, e)),
            ContactKind::EdgeFace(x, f) if x == edge && f == face => {}
            _ if divided
                .iter()
                .any(|&e| row.record.cells().contains(&Cell::Edge(e))) =>
            {
                return Err(refused());
            }
            _ => {}
        }
    }
    let line = |body: &Body<T>, e: EdgeKey| match body
        .get_edge(e)
        .and_then(|d| body.get_curve_geom(d.curve))
        .and_then(|c| c.certified())
    {
        Some(c) => match *c.carrier() {
            geom::Curve3::Line { origin, dir } => Some((origin, dir, c.params())),
            _ => None,
        },
        None => None,
    };
    let half = T::from_f64(0.5);
    // A point's parameter on `e`'s line, anchored inside its span.
    let param = |body: &Body<T>, e: EdgeKey, p: Point3<T>| {
        let (origin, dir, (s0, s1)) = line(body, e).ok_or_else(refused)?;
        geom::Curve3::Line { origin, dir }
            .param_near(p, (s0 + s1) * half)
            .ok_or_else(refused)
    };
    let (o, d, _) = line(body, edge).ok_or_else(refused)?;
    let mut at = Vec::new();
    for &(record, cell) in &crossed {
        let (point, crossed) = match cell {
            Cell::Edge(g) => {
                let (og, dg, _) = line(body, g).ok_or_else(refused)?;
                // The closest points of two lines meeting: `o + t·d`
                // with `t = (b·e − c·f) / (a·c − b²)` over `w = o − og`.
                let w = o - og;
                let (a, b, c) = (d.dot(d), d.dot(dg), dg.dot(dg));
                let (f, e) = (d.dot(w), dg.dot(w));
                let p = o + d * ((b * e - c * f) / (a * c - b * b));
                (p, Crossed::Edge(g, param(body, g, p)?))
            }
            Cell::Vertex(w) => (
                crate::readback::vertex_point(body, w).map_err(|_| desync())?,
                Crossed::Corner(w),
            ),
            Cell::Face(_) => return Err(desync()),
        };
        at.push(Crossing {
            at: param(body, edge, point)?,
            point,
            crossed,
            record,
        });
    }
    // The edge keeps `[t0, t]` at each split, so the crossings go from
    // its far end back.
    let mut ordered: Vec<Crossing<T>> = Vec::new();
    for crossing in at {
        let mut slot = ordered.len();
        for (k, placed) in ordered.iter().enumerate() {
            if order(crossing.at, placed.at)? {
                slot = k;
                break;
            }
        }
        ordered.insert(slot, crossing);
    }
    // Where each recorded vertex lies: on the edge, after how many of
    // its crossings; on a crossed edge, past its crossing or not. Read
    // before any split, on the undivided lines.
    let mut placed: BTreeMap<usize, (EdgeKey, usize)> = BTreeMap::new();
    for &(k, v, e) in &touching {
        let p = crate::readback::vertex_point(body, v).map_err(|_| desync())?;
        let t = param(body, e, p)?;
        let past = if e == edge {
            let mut n = 0;
            for crossing in &ordered {
                n += usize::from(order(t, crossing.at)?);
            }
            n
        } else {
            let tg = ordered
                .iter()
                .find_map(|c| match c.crossed {
                    Crossed::Edge(g, tg) if g == e => Some(tg),
                    _ => None,
                })
                .ok_or_else(desync)?;
            usize::from(order(t, tg)?)
        };
        placed.insert(k, (e, past));
    }
    let mut pieces = vec![edge];
    let mut ends = Vec::new();
    let mut pairs: BTreeMap<usize, (VertexKey, VertexKey)> = BTreeMap::new();
    // Each divided edge's pieces along it, from its start.
    let mut along: BTreeMap<EdgeKey, Vec<EdgeKey>> = BTreeMap::new();
    for crossing in &ordered {
        let p = crossing.point;
        let w = match crossing.crossed {
            Crossed::Edge(g, tg) => {
                let on_g = body.split_edge_onto(g, tg, p, tol)?;
                along.insert(g, vec![g, on_g.new_edge]);
                on_g.vertex
            }
            Crossed::Corner(w) => w,
        };
        let on_edge = body.split_edge_onto(edge, crossing.at, p, tol)?;
        pieces.push(on_edge.new_edge);
        ends.push(on_edge.vertex);
        pairs.insert(crossing.record, (on_edge.vertex, w));
    }
    along.insert(
        edge,
        core::iter::once(edge)
            .chain(pieces[1..].iter().rev().copied())
            .collect(),
    );
    let mut rows = ContactRecords::default();
    for (k, row) in working.rows().iter().enumerate() {
        let record = if let Some(&(x, y)) = pairs.get(&k) {
            Contact::new(body, Cell::Vertex(x), Cell::Vertex(y))
        } else if let Some(&(e, past)) = placed.get(&k) {
            let (_, v, _) = touching
                .iter()
                .find(|&&(j, ..)| j == k)
                .copied()
                .ok_or_else(desync)?;
            let piece = along.get(&e).and_then(|p| p.get(past)).ok_or_else(desync)?;
            Contact::new(body, Cell::Vertex(v), Cell::Edge(*piece))
        } else if row.record.kind() == ContactKind::EdgeFace(edge, face) {
            continue;
        } else {
            rows.push(row.clone());
            continue;
        };
        rows.push(Cited::new(record.map_err(|_| desync())?, row.cites.clone()));
    }
    *working = rows;
    Ok((pieces, ends))
}

/// Where an edge running past a face crosses the face's loops
/// ([`divide`]).
struct Crossing<T: geom_core::Real> {
    /// The edge's parameter there.
    at: T,
    /// The point both of the crossing's vertices hold.
    point: Point3<T>,
    /// What the edge crosses.
    crossed: Crossed<T>,
    /// The record naming the crossing, by its index.
    record: usize,
}

/// The cell of a face's loops an edge crosses ([`Crossing`]).
enum Crossed<T> {
    /// A vertex of the loops.
    Corner(VertexKey),
    /// An edge of the loops, with its parameter at the crossing.
    Edge(EdgeKey, T),
}

/// A joint the parting made that no longer resolves.
fn joint_desync(what: &'static str) -> BooleanError {
    BooleanError::JoinDesync { what }
}

/// Whether a record puts vertex `v`, which no loop of `face` passes
/// through, inside an edge of `face`'s boundary.
fn touches_boundary_between<T: Decide>(
    body: &Body<T>,
    contacts: &ContactRecords,
    v: VertexKey,
    face: FaceKey,
) -> bool {
    let Some(halves) = face_halves(body, face) else {
        return false;
    };
    let on_loop = |w: VertexKey| {
        halves
            .iter()
            .any(|&he| body.get_half_edge(he).is_some_and(|h| h.start == w))
    };
    if on_loop(v) {
        return false;
    }
    contacts.rows().iter().any(|row| match row.record.kind() {
        ContactKind::VertexEdge(x, g) if x == v => halves
            .iter()
            .any(|&he| body.get_half_edge(he).is_some_and(|h| h.edge == g)),
        _ => false,
    })
}

/// The vertices of `face`'s loops a vertex-vertex record puts on `v`'s
/// point: where `v` ends an edge lying in `face`, the vertex the
/// parting joins it to ([`part`]).
fn joins<T: Decide>(
    body: &Body<T>,
    contacts: &ContactRecords,
    v: VertexKey,
    face: FaceKey,
) -> Vec<VertexKey> {
    contacts
        .rows()
        .iter()
        .filter_map(|row| match row.record.kind() {
            ContactKind::VertexVertex(x, y) if x == v => Some(y),
            ContactKind::VertexVertex(x, y) if y == v => Some(x),
            _ => None,
        })
        .filter(|&w| on_face_loops(body, face, w))
        .collect()
}

/// Whether a loop of `face` passes through `v`.
fn on_face_loops<T: Decide>(body: &Body<T>, face: FaceKey, v: VertexKey) -> bool {
    face_halves(body, face).is_some_and(|halves| {
        halves
            .iter()
            .any(|&he| body.get_half_edge(he).is_some_and(|h| h.start == v))
    })
}

/// Whether a record puts `v` inside an edge.
fn touches_an_edge(contacts: &ContactRecords, v: VertexKey) -> bool {
    contacts
        .rows()
        .iter()
        .any(|row| matches!(row.record.kind(), ContactKind::VertexEdge(x, _) if x == v))
}

/// Every half-edge of `face`'s loops.
fn face_halves<T: Decide>(body: &Body<T>, face: FaceKey) -> Option<Vec<HalfEdgeKey>> {
    let data = body.get_face(face)?;
    let mut out = Vec::new();
    for l in core::iter::once(data.outer).chain(data.rings.iter().copied()) {
        if let crate::entity::LoopBoundary::Cycle { first } = body.get_loop(l)?.boundary {
            out.extend(body.loop_cycle(first)?);
        }
    }
    Some(out)
}
