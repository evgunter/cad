//! **The material decides the edges where coincident sheets meet on a
//! line** (`crates/topo/README.md`, tier 3: where more than two sheets
//! meet on a line, the material decides the edges, and no face passes
//! through an edge's material wedge). Three operations on the result
//! build what it decides:
//!
//! - **Parting** ([`part_riders`]) makes two edges of one, where an edge
//!   lies in a face whose material crosses its wedge.
//! - **Merging** ([`merge_slivers`]) makes one edge of two, where a face
//!   of no area runs between them and nothing else lies on their line.
//! - **Closing** ([`close_struts`]) makes no edge of two, where the
//!   result keeps one face on both sides of them.
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
//! - a zero-area ring, where neither end lies on the face's boundary,
//!   the face a plane;
//! - a slit of the loop an end lies on, that end parted into one vertex
//!   per wedge;
//! - a slit of the loop a vertex on an end's point lies on, the two
//!   joined ([`Parted::fused`]);
//! - two faces, where both ends lie on one loop of the face: each end
//!   parts or joins, and the face parts along the line.
//!
//! An end joins rather than parts where it is not the face's own vertex
//! but another on its point, a vertex-vertex record between them. That
//! is where the edge runs on past the face ([`divide`]): the edge, and the
//! face's boundary edge it crosses, a line or a conic, are each divided
//! where the two meet, at a crossing an edge-edge record names, or the
//! edge alone where a vertex-on-edge record puts a corner of the face on
//! it; the record becomes one between the two vertices. An edge lying
//! partly in several faces is divided against all of them at once.
//! Beyond the face the walls the edge runs on between join its two
//! wedges, so the material at the crossing is one cone, and one vertex.
//!
//! The same holds where an edge lying wholly in the face ends at a vertex
//! a record puts on the face's corner: the two vertices' cones are one
//! once the line parts.
//!
//! Where it reads an edge whose material parts it and the parting is not
//! built, the Boolean refuses ([`EdgeInFaceGap`]) rather than leave the
//! one edge: an edge one of whose faces is tangent to the face leaves a
//! cusp per edge, whose reading on a line of coincident edges is not
//! built, an arc leaves two coincident arcs the census does not read,
//! and a curved face would carry a slit or a ring that is neither meshed
//! nor measured ([`EdgeInFaceGap::Tangent`], [`EdgeInFaceGap::Curved`],
//! [`EdgeInFaceGap::CurvedFace`]). The reading
//! ([`holding`]) finds an edge whose carrier lies on the face's surface,
//! a line on a plane, cylinder or cone and a conic on a plane or a
//! quadric, where an edge-on-face record names the pair or the face holds
//! a piece of it between the crossings its records name, whatever piece
//! holds its midpoint. An edge it does not find, on a torus or a spline
//! face or touching the face where no record names it, stays one edge:
//! the result gate's line reading ([`crate::census::line_wedges`]) refuses
//! it where its wedges overlap another edge's on the line, and it ships
//! otherwise.
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
//!
//! # Closing
//!
//! Merging's mirror: where the result keeps a slit's face F on both
//! sides of the slit's line and drops both walls, F continues across the
//! line, a half-turn of material about it, and no edge lies there. The
//! zip glues F's half of each of the slit's two edges to the other
//! operand's section strut along the line (`zip::kill_pair`), so the
//! strut survives with F on both sides and a free end that bounds
//! nothing. [`close_struts`] runs in the output stage, before the
//! coplanar merge, whose entry gate refuses a strut, and deletes the
//! struts the zip reported, and its seam edges, that F lies on both
//! sides of, each with its free end, a ring closed whole with its lone
//! vertex too ([`crate::strut`], the delete the merge runs on its own
//! dangling seam edges). Where the slit runs on past both ends of the
//! stretch the other operand fills, the edge F lies on both sides of
//! there has no free end: it is no strut but a stretch of the slit
//! closed across, and deleting it parts the slit's loop in two (`kemr`,
//! [`part_mid_slit`]). F's region is unchanged by each step. A strut
//! the zip kept between two walls is a reflex edge of the result, and
//! stays.

use std::collections::{BTreeMap, BTreeSet};

use geom_brep::curve_pair::{Hit, MeetFault, Span, SpanEnd, meet};
use geom_core::{Band, Decide, Margin, Point3, Sign, Tol};

use super::ops::{DoorInBand, PieceRead, piece_reading};
use super::{BooleanDecision, BooleanError, Coincide, DeclarationRead};
use crate::Cell;
use crate::body::Body;
use crate::boolean::records::{Contact, ContactKind, ContactPairError, ContactRecords};
use crate::coincidence::Cited;
use crate::entity::{EdgeKey, EntityId, FaceKey, HalfEdgeKey, VertexKey};
use crate::euler::{FaceSurface, MefSite, MevSite};
use crate::lineage::{Fusions, LineageRows};
use crate::live::proven;

/// Why an edge of the result lying in one of its faces is not parted
/// ([`BooleanError::EdgeInFace`]).
///
/// **NOT carried to the façade's curated list, and that is a decision
/// rather than an omission** (`scripts/payload-rung-sweep.py` names
/// this rung; the disposition table cites this paragraph as its home).
/// Every case is a construction gap with no lever but the kernel's own
/// defects ([`EdgeInFaceGap::is_defect`]): nothing a person does differs
/// by case, and the refusal's sentence names it. A Rust caller
/// can already name and match it, since `pncad` re-exports `topo`
/// whole.
///
/// **The falsifier is a caller that acts on the case by itself**: a
/// driver that moves an end off the face's boundary when it reads
/// [`EdgeInFaceGap::EndOnBoundary`], say. Then carry it and delete this
/// paragraph and the disposition row with it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EdgeInFaceGap {
    /// The face the edge lies in has no normal at its carrier's foot of
    /// the edge's midpoint, or one of the edge's own faces none at the
    /// midpoint (a spline or approximating face, which this reading does
    /// not read, among them), to read the side by.
    NoNormal,
    /// One of the edge's faces is tangent to the face along the line,
    /// so each of the two edges would bound a cusp, a wedge of no angle
    /// (D1's doubled cusp), whose reading on a line of coincident edges
    /// is not built.
    Tangent,
    /// The edge is an arc: the parting would leave two coincident arcs
    /// that no contact record backs
    /// (`work/heldrecords/a-curved-slit-fails-tier-3-prime-on-its-two-coincident-arcs.md`).
    Curved,
    /// The face is no plane: the parting would leave a slit or a
    /// zero-area ring in a curved face, which the chart walk does not
    /// mesh and the curved mass-properties lane does not measure
    /// (`work/tesslie/the-walk-panics-on-a-slit-in-a-curved-face.md`,
    /// `work/nearpoint/a-zero-area-ring-in-a-cylinder-face-measures-degenerate.md`).
    /// It retires when both read it: the parting itself builds there.
    CurvedFace,
    /// An end the parting would divide into one vertex per wedge touches
    /// an edge's interior, and the census does not read that touch at
    /// the copies.
    DividedEndTouched,
    /// An end of the edge lies on the face's loops more than once, or
    /// the edge's faces go into the material of two faces it lies in,
    /// wholly in one of them.
    Shared,
    /// The edge lies in the face along part of its length and runs on
    /// past the face's boundary, and no record names a crossing of the
    /// two, or one names an edge of the boundary the edge meets other
    /// than once ([`divide`]).
    Unrecorded,
    /// A record names a crossing of the edge with an edge of the face's
    /// boundary whose meeting with it the curve-pair reader does not
    /// settle in closed form or by its walk ([`divide`]).
    CrossedCurve,
    /// A record names an edge of the face's boundary on the edge's own
    /// carrier, so the two run along each other rather than cross.
    Overlap,
    /// A record names the edge, or an edge of the face's boundary it
    /// crosses, as other than a crossing, a vertex resting on it, or the
    /// edge lying in the face, so which of the pieces [`divide`] would
    /// leave holds it is not read.
    Unplaced,
    /// Two crossings of the face's boundary, or a crossing and a vertex
    /// resting on the edge, lie at one point along the edge, which
    /// [`divide`] would divide it at once.
    AtACrossing,
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
    /// A piece of an edge [`divide`] left is read again as running past
    /// the face it was divided against. The division drops the
    /// edge-on-face record that read so, and the pieces' ends lie on the
    /// face by records or not at all: a kernel defect.
    Redivided,
    /// A point [`divide`] divided the edge at, where it meets the face's
    /// boundary, bounds no piece of it read as lying in the face: the
    /// edge touches a corner of the face from outside, say, which a
    /// vertex-on-edge record names as it names a crossing.
    Unjoined,
}

impl EdgeInFaceGap {
    /// Whether the case is the kernel's own defect rather than a
    /// parting not built: no Boolean result holds it.
    pub const fn is_defect(self) -> bool {
        matches!(self, Self::Crossed | Self::TwinIncident | Self::Redivided)
    }

    /// The words a refusal says it in: after "where" for a construction
    /// gap, after "and" for a defect.
    pub(super) const fn words(self) -> &'static str {
        match self {
            Self::NoNormal => {
                "the face has no normal at its carrier's foot of the edge's midpoint, or one of \
                 the edge's own faces none at the midpoint (a spline face among them), to read \
                 the side by"
            }
            Self::Shared => {
                "an end of the edge lies on the face's boundary more than once, or the edge's \
                 faces go into the material of two faces it lies in, wholly in one of them"
            }
            Self::Tangent => {
                "one of the edge's faces is tangent to the face along it, so each of the two \
                 edges would bound a cusp, which the reading of a line of coincident edges does \
                 not cover"
            }
            Self::Curved => {
                "the edge is an arc, which would leave two coincident arcs that no contact record \
                 backs (`work/heldrecords/a-curved-slit-fails-tier-3-prime-on-its-two-coincident-arcs.md`)"
            }
            Self::CurvedFace => {
                "the face is curved, and a slit or a zero-area ring in a curved face is neither \
                 meshed nor measured (`work/tesslie/the-walk-panics-on-a-slit-in-a-curved-face.md`)"
            }
            Self::DividedEndTouched => {
                "an end the parting would divide touches an edge, which the census does not read \
                 at the end's copies"
            }
            Self::Unrecorded => {
                "the edge runs on past the face's boundary and no record names where it crosses it"
            }
            Self::CrossedCurve => {
                "the edge crosses the face's boundary at an edge whose meeting with it is not read"
            }
            Self::Overlap => "the edge runs along an edge of the face's boundary",
            Self::Unplaced => {
                "a record names the edge, or an edge of the face's boundary it crosses, in a way \
                 that puts it on none of the pieces the parting would divide it into"
            }
            Self::AtACrossing => {
                "two of the points the edge would be divided at, where it crosses the face's \
                 boundary or where a vertex rests on it, lie at one point"
            }
            Self::EndOnBoundary => "an end of the edge touches the inside of an edge of the face",
            Self::Pocket => "both ends of the edge lie on one hole of the face",
            Self::Recorded => "a contact record names the face the parting would divide",
            Self::Crossed => "the edge's faces leave the line on opposite sides of the face",
            Self::TwinIncident => {
                "a cell the parting minted is incident to the cell a record of its original \
                 names"
            }
            Self::Redivided => {
                "a piece of the edge, divided where it crosses the face's boundary, is read \
                 again as running past the face"
            }
            Self::Unjoined => {
                "a point it is divided at, where it meets the face's boundary, bounds no piece of \
                 it lying in the face"
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
    pub(super) fused: Fusions,
}

/// **Parts every edge of `body` lying in one of its faces whose material
/// both of the edge's faces leave the line into**, and returns the
/// records that hold after it: the edge-on-face and vertex-on-face
/// records the parting makes structure are dropped, a record of a joined
/// end is its kept vertex's, and a record of the edge, or of an end it
/// divides, is held by the twin too.
///
/// An edge lies in a face `f` it does not bound where its carrier lies
/// on `f`'s surface and an edge-on-face record names the pair, or `f`
/// holds a piece of it between the crossings of `f`'s loops its records
/// name ([`riders`], [`holding`]). One that runs on past `f`, or whose
/// records name a crossing, is divided where it crosses `f`'s boundary
/// first ([`divide`]), and its pieces read again. An edge whose faces both leave the line on `f`'s
/// empty side touches `f` from outside its wedge, and stays one edge in
/// contact with it.
///
/// # Errors
///
/// [`BooleanError::Escalated`] where a reading is in band;
/// [`BooleanError::EdgeInFace`] where the parting is not built
/// ([`EdgeInFaceGap`]); the Euler operators', the splits' and the
/// re-description's refusals.
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
    let mut divided_faces: BTreeSet<FaceKey> = BTreeSet::new();
    // The records as the divisions leave them: each crossing a
    // vertex-vertex record between its two vertices.
    let mut records = contacts.clone();
    // Each piece a division left, with the face it was divided against.
    let mut pieces: BTreeSet<(EdgeKey, FaceKey)> = BTreeSet::new();
    // Each crossing's vertex on the edge a division divided, with the
    // edge and the face it ran past: each is joined by a parting.
    let mut crossings: Vec<(VertexKey, EdgeKey, FaceKey)> = Vec::new();
    let mut fused = Fusions::new(LineageRows::Fusions);
    // Read again after each parting: a parting that divides a face
    // gives one of its pieces a new key, which a later edge lying in
    // that piece is read against. A record naming a divided face refuses
    // `Recorded` below.
    while let Some((edge, faces)) = riders(body, &records, band)?
        .into_iter()
        .find(|(e, _)| !read.contains(e))
    {
        read.insert(edge);
        let mut parting = Vec::new();
        for LiesIn { face, divides } in faces {
            if let Some(tangent) = walls_inside(body, edge, face, band)? {
                parting.push((face, tangent, divides));
            }
        }
        let (face, flat, divides) = match parting[..] {
            [] => continue,
            [one] => one,
            // Pieces of the edge lie in several faces, across their
            // boundaries: divided against all of them at once, its pieces
            // are read again.
            [first, ..] if parting.iter().all(|&(.., divides)| divides) => first,
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
        // A slit or a zero-area ring in a curved face is neither meshed
        // nor measured, so the parting refuses on any face it would part
        // that is no plane.
        let plane = |f: FaceKey| {
            matches!(
                body.get_face(f).and_then(|d| body.get_surface(d.surface)),
                Some(geom::Surface::Plane { .. })
            )
        };
        if let Some(&(curved, ..)) = parting.iter().find(|&&(f, ..)| !plane(f)) {
            return Err(BooleanError::EdgeInFace {
                edge,
                face: curved,
                gap: EdgeInFaceGap::CurvedFace,
            });
        }
        if divides {
            if pieces.contains(&(edge, face)) {
                return Err(refused(EdgeInFaceGap::Redivided));
            }
            let against: Vec<FaceKey> = parting.iter().map(|&(f, ..)| f).collect();
            let division = divide(body, &mut records, edge, &against, band, tol)?;
            pieces.extend(
                division
                    .pieces
                    .iter()
                    .flat_map(|&piece| against.iter().map(move |&f| (piece, f))),
            );
            crossings.extend(division.crossings.into_iter().map(|v| (v, edge, face)));
            // The edge's key holds its first piece, read again with the
            // others.
            read.remove(&edge);
            continue;
        }
        let (start, end) = crate::query::edge_ends(body, edge);
        if [start, end]
            .into_iter()
            .any(|v| touches_boundary_between(body, &records, v, face))
        {
            return Err(refused(EdgeInFaceGap::EndOnBoundary));
        }
        if [start, end]
            .into_iter()
            .any(|v| on_face_loops(body, face, v) && touches_an_edge(&records, v))
        {
            return Err(refused(EdgeInFaceGap::DividedEndTouched));
        }
        let mut joined = Vec::new();
        for v in [start, end] {
            if on_face_loops(body, face, v) {
                continue;
            }
            match joins(body, &records, v, face)[..] {
                [] => {}
                [w] => joined.push((v, w)),
                _ => return Err(refused(EdgeInFaceGap::Shared)),
            }
        }
        let p = part(body, edge, face, &joined, tol)?;
        fused.extend(&p.parted.fused)?;
        let (kept_start, kept_end) = p.parted.ends;
        // A record's cells are in canonical order, the lesser first.
        for c in [
            Cell::Vertex(kept_start),
            Cell::Vertex(kept_end),
            Cell::Edge(edge),
        ] {
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
            divided_faces.insert(face);
        }
        parted.push(p.parted);
    }
    // A crossing no parting joined leaves its two vertices on one point
    // apart.
    if let Some(&(_, edge, face)) = crossings.iter().find(|&&(v, ..)| fused.get(v).is_none()) {
        return Err(BooleanError::EdgeInFace {
            edge,
            face,
            gap: EdgeInFaceGap::Unjoined,
        });
    }
    if parted.is_empty() {
        return Ok((parted, records));
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
    for row in records.rows() {
        let [x, y] = row.record.cells();
        let settle = |c: Cell| match c {
            Cell::Vertex(v) => Cell::Vertex(fused.survivor(v)),
            c => c,
        };
        // A record of a joined end is the kept vertex's; one the joint
        // made structure, the two vertices one or one bounding the other,
        // is dropped.
        let record = if (settle(x), settle(y)) == (x, y) {
            row.record
        } else {
            match Contact::new(body, settle(x), settle(y)) {
                Ok(contact) => contact,
                Err(ContactPairError::OneCell(_) | ContactPairError::Incident(_)) => continue,
            }
        };
        let cells = record.cells();
        if dropped.contains(&cells) {
            continue;
        }
        // A face the parting divided holds a record's cell on one of its
        // pieces, which nothing here reads.
        if let Some(&face) = divided_faces
            .iter()
            .find(|&&f| cells.contains(&Cell::Face(f)))
        {
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

/// One edge the output stage closed its face across ([`close_struts`]):
/// a strut, deleted with its free end, or a slit's stretch, deleted by
/// parting its loop ([`part_mid_slit`]).
#[derive(Clone, Debug)]
pub(super) struct ClosedStrut {
    /// The face on both sides of the edge.
    pub(super) face: FaceKey,
    /// The edge.
    pub(super) edge: EdgeKey,
    /// The vertices deleted with it: a strut's free end, then, where both
    /// its ends were free, the other; none for a slit's stretch, whose
    /// delete parts its loop ([`part_mid_slit`]).
    pub(super) vertices: Vec<VertexKey>,
}

impl ClosedStrut {
    /// The lineage rows of the closing, `(dead, holder)`: the edge and
    /// its deleted vertices lie in the face.
    pub(super) fn substitutions(&self) -> impl Iterator<Item = (Cell, Cell)> + '_ {
        core::iter::once(Cell::Edge(self.edge))
            .chain(self.vertices.iter().map(|&v| Cell::Vertex(v)))
            .map(|dead| (dead, Cell::Face(self.face)))
    }
}

/// **Closes a face across each of `candidates` it lies on both sides
/// of** (module docs, *Closing*): `candidates` are the zip's section
/// struts and seam edges (`ops::finish_output`). Each whose two halves
/// bound one face and that has a free end is a strut, deleted with its
/// free end ([`crate::strut`]), in the order given and again after each,
/// so a chain goes one free end at a time. Once none is left, one with
/// no free end, the slit's edges running on past both its ends, is a
/// stretch of the slit closed across, and its delete parts the slit in
/// two ([`part_mid_slit`]). An edge between two faces is left as it is.
///
/// # Errors
///
/// [`BooleanError::CurvedFaceClosesAcrossSlit`] where a strut's face is
/// not a plane ([`crate::strut::StrutRefusal::Curved`]);
/// [`BooleanError::SlitFaceClosesMidSlit`] where a stretch does not part
/// its loop into a ring of the slit ([`part_mid_slit`]);
/// [`BooleanError::JoinDesync`] where a strut with both ends free is its
/// face's outline, which the zip never leaves; the Euler operators'
/// refusals.
pub(super) fn close_struts<T: Decide + crate::props::AtRestPolicy>(
    body: &mut Body<T>,
    candidates: &[EdgeKey],
    tol: Tol,
) -> Result<Vec<ClosedStrut>, BooleanError> {
    let one_face = |body: &Body<T>, edge: EdgeKey| body.both_sides_face(edge).flatten();
    let mut closed = Vec::new();
    loop {
        let open: Vec<(EdgeKey, FaceKey)> = candidates
            .iter()
            .filter_map(|&edge| one_face(body, edge).map(|face| (edge, face)))
            .collect();
        let next = open
            .iter()
            .find_map(|&(edge, face)| body.toward_free_end(edge).map(|t| (edge, face, t)));
        let Some((edge, face, toward_free)) = next else {
            let Some(&(edge, face)) = open.first() else {
                return Ok(closed);
            };
            part_mid_slit(body, face, edge, tol)?;
            closed.push(ClosedStrut {
                face,
                edge,
                vertices: Vec::new(),
            });
            continue;
        };
        let plan = body
            .plan_strut_prune(face, toward_free)
            .map_err(|refusal| match refusal {
                crate::strut::StrutRefusal::Curved => {
                    BooleanError::CurvedFaceClosesAcrossSlit { face, edge }
                }
                crate::strut::StrutRefusal::LoneOutline => BooleanError::JoinDesync {
                    what: "a strut the zip kept with both ends free is its face's outline",
                },
            })?;
        let vertices = body.prune_strut(&plan, tol)?;
        closed.push(ClosedStrut {
            face,
            edge,
            vertices,
        });
    }
}

/// **Deletes a slit's stretch closed across**: `edge`, both of whose
/// halves bound `face` in one loop and neither of whose ends is free,
/// deleted with `kemr`. The loop parts in two, and `kemr` makes one part
/// a new ring of `face`: the part that is a slit's run, each of its
/// halves run back by another of its halves between the same two
/// vertices. Of the outline, the other part keeps the outline, so
/// neither part being a run refuses; of a ring, both parts are rings of
/// `face` whichever is new, so a part that is no run is taken too. No
/// edge is minted, so the face's surface does not matter.
///
/// # Errors
///
/// [`BooleanError::SlitFaceClosesMidSlit`] where the halves lie in two
/// loops, or where neither part of the outline is a run; `kemr`'s
/// refusals.
fn part_mid_slit<T: Decide + crate::props::AtRestPolicy>(
    body: &mut Body<T>,
    face: FaceKey,
    edge: EdgeKey,
    tol: Tol,
) -> Result<(), BooleanError> {
    let gap = BooleanError::SlitFaceClosesMidSlit { face, edge };
    let outline = proven(&body.faces, face, EntityId::Face).outer;
    let e = proven(&body.edges, edge, EntityId::Edge);
    let (plus, minus) = (e.he_plus, e.he_minus);
    let half = |h: HalfEdgeKey| proven(&body.half_edges, h, EntityId::HalfEdge);
    let lp = half(plus).parent_loop;
    if half(minus).parent_loop != lp {
        return Err(gap);
    }
    let cycle = body.loop_cycle(plus).ok_or(BooleanError::JoinDesync {
        what: "a slit's stretch's loop does not walk",
    })?;
    let at = cycle
        .iter()
        .position(|&h| h == minus)
        .ok_or(BooleanError::JoinDesync {
            what: "a slit's stretch's two halves in one loop do not walk to each other",
        })?;
    // Each side: the halves strictly between one half and the other, in
    // `next` order.
    let (after_plus, after_minus) = (&cycle[1..at], &cycle[at + 1..]);
    let ends = |h: HalfEdgeKey| (half(h).start, body.proven_half_edge_end(h));
    let run_back = |hs: &[HalfEdgeKey]| {
        !hs.is_empty()
            && hs.iter().all(|&h| {
                let (a, b) = ends(h);
                hs.iter().any(|&k| k != h && ends(k) == (b, a))
            })
    };
    let (he1, he2) = if run_back(after_plus) {
        (plus, minus)
    } else if lp != outline || run_back(after_minus) {
        (minus, plus)
    } else {
        return Err(gap);
    };
    body.kemr_minting(he1, he2, tol)?;
    Ok(())
}

/// A face an edge lies in ([`riders`]).
#[derive(Clone, Copy)]
struct LiesIn {
    /// The face.
    face: FaceKey,
    /// Whether [`divide`] divides the edge against it before it is
    /// parted.
    divides: bool,
}

/// Every edge of `body` lying in faces it does not bound, with those
/// faces ([`part_riders`]'s reading), in edge order, each with whether
/// [`divide`] divides the edge against it first ([`holding`]).
///
/// A face is asked of an edge where something names the two: both of
/// the edge's ends lie on it, by structure, by a vertex-on-face record,
/// by a vertex-vertex record with a vertex of its loops, or by a
/// vertex-on-edge record on an edge of its loops (a curved face carries
/// no edge-on-face record, so an edge whose ends rest on its boundary is
/// named by those alone); an edge-on-face record names the pair; or a
/// record names a crossing of the edge with the face's loops, an
/// edge-edge record with an edge of them or a vertex-on-edge record of a
/// vertex of them on the edge.
fn riders<T: Decide>(
    body: &Body<T>,
    contacts: &ContactRecords,
    band: Band,
) -> Result<Vec<(EdgeKey, Vec<LiesIn>)>, BooleanError> {
    let vf: BTreeSet<(VertexKey, FaceKey)> = contacts.vertex_face().collect();
    let ve: BTreeSet<(VertexKey, EdgeKey)> = contacts.vertex_edge().collect();
    let ef: BTreeSet<(EdgeKey, FaceKey)> = contacts.edge_face().collect();
    let edge_faces = |g: EdgeKey| {
        crate::readback::edge_sides(body, g)
            .map(|sides| {
                let (p, q) = sides.faces();
                [p, q]
            })
            .into_iter()
            .flatten()
    };
    let on = |v: VertexKey| -> BTreeSet<FaceKey> {
        core::iter::once(v)
            .chain(partners(contacts, v))
            .flat_map(|x| body.faces_of_vertex(x).unwrap_or_default())
            .chain(vf.iter().filter(|&&(w, _)| w == v).map(|&(_, f)| f))
            .chain(
                ve.iter()
                    .filter(|&&(w, _)| w == v)
                    .flat_map(|&(_, g)| edge_faces(g)),
            )
            .collect()
    };
    // The records naming a crossing of each edge: an edge-edge record
    // of it, or a vertex-on-edge record on it.
    let mut crossing: BTreeMap<EdgeKey, Vec<&Contact>> = BTreeMap::new();
    for row in contacts.rows() {
        match row.record.kind() {
            ContactKind::EdgeEdge(x, y) => {
                crossing.entry(x).or_default().push(&row.record);
                crossing.entry(y).or_default().push(&row.record);
            }
            ContactKind::VertexEdge(_, g) => crossing.entry(g).or_default().push(&row.record),
            _ => {}
        }
    }
    let mut out = Vec::new();
    for (e, _) in body.edges() {
        let (a, b) = crate::query::edge_ends(body, e);
        if a == b {
            continue;
        }
        let named = crossing.get(&e).map_or(&[][..], Vec::as_slice);
        let Ok(sides) = crate::readback::edge_sides(body, e) else {
            continue;
        };
        let (fa, fb) = sides.faces();
        let both: BTreeSet<FaceKey> = on(a).intersection(&on(b)).copied().collect();
        let recorded: BTreeSet<FaceKey> = ef
            .iter()
            .filter(|&&(g, _)| g == e)
            .map(|&(_, f)| f)
            .collect();
        let mut faces = both.clone();
        faces.extend(recorded.iter().copied());
        faces.extend(crossed_faces(body, named, e));
        faces.remove(&fa);
        faces.remove(&fb);
        // A face on the surface one of the edge's own faces lies on is
        // another piece of that sheet, which the edge bounds rather than
        // lies in.
        let sheet = |f: FaceKey| body.get_face(f).map(|d| d.surface);
        faces.retain(|&f| sheet(f).is_none() || ![sheet(fa), sheet(fb)].contains(&sheet(f)));
        let mut lies_in = Vec::new();
        for f in faces {
            let names = Names {
                ends: both.contains(&f),
                record: recorded.contains(&f),
            };
            if let Some(divides) = holding(body, named, e, f, names, band)? {
                lies_in.push(LiesIn { face: f, divides });
            }
        }
        if !lies_in.is_empty() {
            out.push((e, lies_in));
        }
    }
    Ok(out)
}

/// The faces the records `named`, those naming a crossing of edge `e`,
/// name it with: each face of an edge an edge-edge record pairs with
/// `e`, and of a vertex a vertex-on-edge record rests on `e`.
fn crossed_faces<T: Decide>(body: &Body<T>, named: &[&Contact], e: EdgeKey) -> BTreeSet<FaceKey> {
    let mut out = BTreeSet::new();
    for record in named {
        match record.kind() {
            ContactKind::EdgeEdge(x, y) => {
                let g = if x == e { y } else { x };
                if let Ok(sides) = crate::readback::edge_sides(body, g) {
                    let (p, q) = sides.faces();
                    out.extend([p, q]);
                }
            }
            ContactKind::VertexEdge(v, _) => {
                out.extend(body.faces_of_vertex(v).unwrap_or_default());
            }
            _ => {}
        }
    }
    out
}

/// What names an edge and a face [`riders`] asks of it.
#[derive(Clone, Copy)]
struct Names {
    /// Both of the edge's ends lie on the face.
    ends: bool,
    /// An edge-on-face record names the pair.
    record: bool,
}

/// **Whether edge `e` lies in face `f` along some piece of its length**,
/// and if so whether [`divide`] divides it against `f` first (`Some`
/// with that).
///
/// The edge's carrier lies on `f`'s surface ([`carrier_on_surface`]),
/// and the records naming a crossing of the edge with `f`'s loops ([`cuts`])
/// cut it into pieces each wholly in `f` or wholly out of it. The edge
/// lies in `f` where an edge-on-face record names the pair (its
/// interior meets `f`'s, and a carrier on `f`'s surface meets it along
/// a piece), or where `f` holds the midpoint of one of the pieces. It
/// is divided first unless both its ends lie on `f` and no record
/// names a crossing.
///
/// Where whether the carrier lies on `f`'s surface is in band, the
/// edge's faces are read against `f` first ([`walls_inside`]): the
/// reading matters only where both go into `f`'s material. Where they
/// leave the line on opposite sides of `f`, the edge lying in `f` would
/// be the kernel defect [`EdgeInFaceGap::Crossed`], so it does not lie
/// there.
///
/// # Errors
///
/// [`BooleanError::Escalated`] where a crossing lies, or whether
/// `f` holds a piece's midpoint, is in band and something other than a
/// crossing names the pair; or where whether the carrier lies on `f`'s
/// surface is in band, both the edge's faces go into `f`'s material and
/// something other than a crossing names the pair. [`walls_inside`]'s
/// refusals but `Crossed`, where it reads the faces.
fn holding<T: Decide>(
    body: &Body<T>,
    named: &[&Contact],
    e: EdgeKey,
    f: FaceKey,
    names: Names,
    band: Band,
) -> Result<Option<bool>, BooleanError> {
    let Some((carrier, (t0, t1))) = span_of(body, e) else {
        return Ok(None);
    };
    // A face only a crossing names, which an in-band reading cannot
    // place the edge in, is not asked: nothing else says the edge lies
    // in it.
    let crossed_only = !names.ends && !names.record;
    let unplaced = |read: BooleanError| match read {
        BooleanError::Escalated { .. } if crossed_only => Ok(None),
        read => Err(read),
    };
    match carrier_on_surface(body, &carrier, (t0, t1), f, band) {
        Ok(true) => {}
        Ok(false) => return Ok(None),
        // Whether the edge lies on the surface matters only where both
        // its faces go into the face's material: where they do not, the
        // parting is not asked whatever the answer.
        Err(read) => {
            return match walls_inside(body, e, f, band) {
                Ok(Some(_)) => unplaced(read),
                // A definite `Crossed` cannot hold of an edge lying in
                // the face's interior: a face of the edge leaving on the
                // material side would part two sectors of the face's
                // material, which no solid holds. So the edge does not
                // lie in the face.
                Ok(None)
                | Err(BooleanError::EdgeInFace {
                    gap: EdgeInFaceGap::Crossed,
                    ..
                }) => Ok(None),
                Err(other) => Err(other),
            };
        }
    }
    let along = Span::new(&carrier, (t0, t1));
    let at = match cuts(body, named, e, f, &along, band) {
        Ok(at) => at,
        Err(read) => return unplaced(read),
    };
    let divided = !names.ends || !at.is_empty();
    if names.record {
        return Ok(Some(divided));
    }
    let ends: Vec<T> = core::iter::once(t0)
        .chain(at.into_iter().rev())
        .chain(core::iter::once(t1))
        .collect();
    let mut in_band = None;
    for piece in ends.windows(2) {
        let mid = carrier.eval(geom::mid_param(piece[0], piece[1]));
        // Whether the face holds the point is read here first, so the
        // door's margin in band is the operands' geometry.
        match piece_reading(body, f, mid, band, DoorInBand::Containment) {
            Ok(PieceRead::In) => return Ok(Some(divided)),
            Ok(PieceRead::NotIn | PieceRead::Unread(_)) => {}
            Err(read) => {
                in_band.get_or_insert(read);
            }
        }
    }
    in_band.map_or(Ok(None), unplaced)
}

/// Where along `along`, edge `e`'s span, the records `named` place a
/// crossing of `e` with `f`'s loops, from its far end back, each read as
/// [`divide`] reads it ([`crossing_named`]). A record [`divide`] would
/// refuse is no cut here: where the edge is found lying in `f`,
/// [`divide`] reads it again and refuses.
fn cuts<T: Decide>(
    body: &Body<T>,
    named: &[&Contact],
    e: EdgeKey,
    f: FaceKey,
    along: &Span<'_, T>,
    band: Band,
) -> Result<Vec<T>, BooleanError> {
    let loops = [(f, face_edges(body, f))];
    let mut at: Vec<T> = Vec::new();
    for record in named {
        let crossings = match crossing_named(body, record, e, &loops, along, band) {
            Ok(crossings) => crossings,
            Err(BooleanError::EdgeInFace { .. }) => continue,
            Err(other) => return Err(other),
        };
        // From the far end back; a point read twice is one cut.
        for crossing in crossings {
            let at_one_point = || BooleanError::EdgeInFace {
                edge: e,
                face: f,
                gap: EdgeInFaceGap::AtACrossing,
            };
            match slot_from_far_end(at.iter().copied(), crossing.at, at_one_point, band) {
                Ok(slot) => at.insert(slot, crossing.at),
                Err(BooleanError::EdgeInFace { .. }) => {}
                Err(other) => return Err(other),
            }
        }
    }
    Ok(at)
}

/// The site [`carrier_on_surface`] decides a point of a line on a plane
/// under.
pub(super) const RIDER_ON_PLANE: &str = "bool_rider_on_plane";

/// The site [`carrier_on_surface`] decides a point of an edge on a curved face
/// under.
pub(super) const RIDER_ON_CURVED: &str = "bool_rider_on_curved";

/// The points of an edge [`carrier_on_surface`] reads on a face's surface.
#[derive(Clone, Copy)]
enum Samples {
    /// Its ends.
    Ends,
    /// Its ends, its midpoint and its quarter points.
    Five,
}

/// **Whether edge carrier `carrier`, over `(t0, t1)`, lies on face `f`'s
/// surface**: a line on a plane at both its ends, and on a cylinder or
/// cone by [`crate::census::line_on_quadric_carrier`]
/// (`bool_line_face_carrier_offset`); a conic in a plane by its own plane's offset
/// from it (`bool_conic_face_plane_offset`), and on a sphere, cylinder
/// or cone at five points along it (two conics of one plane meet at
/// four unless they are one). Each point's offset from the surface is
/// decided on the band ([`RIDER_ON_PLANE`], [`RIDER_ON_CURVED`]), every
/// point before the answer: one point definitely off decides that the
/// carrier is not on the surface, whatever the others read.
/// Any other pair lies on nothing here.
///
/// # Errors
///
/// [`BooleanError::Escalated`] where no point is definitely off and an
/// offset is in band.
fn carrier_on_surface<T: Decide>(
    body: &Body<T>,
    carrier: &geom::Curve3<T>,
    (t0, t1): (T, T),
    f: FaceKey,
    band: Band,
) -> Result<bool, BooleanError> {
    let Some(surface) = body.get_face(f).and_then(|d| body.get_surface(d.surface)) else {
        return Ok(false);
    };
    let (samples, site, coincide) = match (carrier, surface) {
        (geom::Curve3::Line { .. }, geom::Surface::Plane { .. }) => {
            (Samples::Ends, RIDER_ON_PLANE, Coincide::EdgeOnPlane)
        }
        (
            geom::Curve3::Circle { .. } | geom::Curve3::Ellipse { .. },
            &geom::Surface::Plane { origin, normal, .. },
        ) => {
            return match crate::splitting::plane_crossing_lane(
                carrier, t0, t1, origin, normal, band,
            ) {
                crate::splitting::PlaneCrossingLane::Conic(
                    crate::splitting::ConicPlaneMeet::Parallel { offset },
                ) => match crate::validate::decide(
                    "bool_conic_face_plane_offset",
                    Margin::of(offset),
                    band,
                ) {
                    Ok(Sign::Zero) => Ok(true),
                    Ok(Sign::Positive | Sign::Negative) => Ok(false),
                    Err(diag) => Err(BooleanError::coincidence(
                        Coincide::EdgeOnPlane,
                        DeclarationRead::Moot,
                        diag,
                    )),
                },
                _ => Ok(false),
            };
        }
        // A line on a cylinder or a cone: the one reading of it
        // ([`crate::census::line_on_quadric_carrier`]).
        (
            geom::Curve3::Line { .. },
            geom::Surface::Cylinder { .. } | geom::Surface::Cone { .. },
        ) => {
            return match crate::census::line_on_quadric_carrier(
                surface,
                carrier.eval(t0),
                carrier.eval(t1),
                "bool_line_face_carrier_offset",
                band,
            ) {
                Some(Ok(on)) => Ok(on),
                None => Ok(false),
                Some(Err(diag)) => Err(BooleanError::coincidence(
                    Coincide::EdgeOnCurvedFace,
                    DeclarationRead::Moot,
                    diag,
                )),
            };
        }
        (
            geom::Curve3::Circle { .. } | geom::Curve3::Ellipse { .. },
            geom::Surface::Sphere { .. }
            | geom::Surface::Cylinder { .. }
            | geom::Surface::Cone { .. },
        ) => (Samples::Five, RIDER_ON_CURVED, Coincide::EdgeOnCurvedFace),
        _ => return Ok(false),
    };
    let mid = geom::mid_param(t0, t1);
    let at: &[T] = match samples {
        Samples::Ends => &[t0, t1],
        Samples::Five => &[
            t0,
            geom::mid_param(t0, mid),
            mid,
            geom::mid_param(mid, t1),
            t1,
        ],
    };
    let points: Vec<Point3<T>> = at.iter().map(|&t| carrier.eval(t)).collect();
    crate::census::points_on_surface(surface, &points, site, band)
        .map_err(|diag| BooleanError::coincidence(coincide, DeclarationRead::Moot, diag))
}

/// Whether both faces of edge `e` leave its line on the material side of
/// the face `f` it lies in (`bool_rider_wall_side`): at the edge's
/// midpoint, each face's direction into itself off the line (its
/// outward normal crossed with its half's tangent: a face lies to the
/// left of its half) against `f`'s outward normal there, levered at the
/// edge's chord. `Some` where both go in, with whether either face is
/// tangent to `f` along the line; `None`
/// where both go out (the edge touches `f` from outside its wedge).
///
/// # Errors
///
/// [`BooleanError::EdgeInFace`] ([`EdgeInFaceGap::Crossed`]) where they
/// go to opposite sides, ([`EdgeInFaceGap::NoNormal`]) where `f` or a
/// face of the edge has no normal there, or ([`EdgeInFaceGap::Tangent`]) where a face is
/// tangent along the line to `f` and goes into its material, or
/// ([`EdgeInFaceGap::CurvedFace`]) where `f` is curved and a face is
/// tangent to it along the line and does not bend away from its
/// material ([`bend_across`]); [`BooleanError::Escalated`] where a side,
/// a bend, or a normal, is in band.
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
    let normal = |read: Result<
        Option<geom_brep::OutwardNormal<T>>,
        crate::face_normal::NormalAtError,
    >| match read {
        Ok(Some(n)) => Ok(n.vec()),
        Ok(None) => Err(desync()),
        Err(crate::face_normal::NormalAtError::Escalated { decision, diag }) => {
            Err(BooleanError::Escalated {
                decision: BooleanDecision::of_normal(decision),
                diag,
            })
        }
        Err(_) => Err(refused(EdgeInFaceGap::NoNormal)),
    };
    // A face the point door reads no normal on (a spline or an
    // approximating face, `Ok(None)`) is an input this reading does not
    // read; a key that no longer resolves is the parting's own desync.
    let normal_at =
        |face: FaceKey| match crate::face_normal::face_outward_normal_at(body, face, mid, band) {
            Ok(None) if body.get_face(face).is_some() => Err(refused(EdgeInFaceGap::NoNormal)),
            Ok(None) => Err(desync()),
            read => normal(read),
        };
    // The edge is read as lying on the floor's surface, or within the
    // band of it where `holding` asks whether that reading matters: the
    // floor's side is read at its carrier's foot of the midpoint, which
    // asks nothing of where the midpoint stands.
    let floor = match crate::face_normal::face_outward_normal_at_foot(body, f, mid, band) {
        Ok(Some(n)) => n.vec(),
        Ok(None) => return Err(desync()),
        Err(_) => return Err(refused(EdgeInFaceGap::NoNormal)),
    };
    let floor_flat =
        super::sectors::FaceShape::of_face(body, f, "a face an edge lies in lost its carrier")?
            == super::sectors::FaceShape::Plane;
    let mut inside = [false; 2];
    let mut flat = [false; 2];
    for (k, (he, dir)) in [(edge.he_plus, tangent), (edge.he_minus, -tangent)]
        .into_iter()
        .enumerate()
    {
        let wall = body.face_of_half_edge(he).ok_or_else(desync)?;
        let normal = normal_at(wall)?;
        let into = normal.cross(dir);
        // How far the wall leaves the floor grows across the line: the
        // lever is the wall's own reach from the line's middle, as a
        // seam's dihedral reads it ([`crate::wedge_reach`]).
        let arm = crate::wedge_reach(body, [wall, wall], curve.carrier(), (t0, t1));
        let sectors =
            |diag| BooleanError::coincidence(Coincide::Sectors, DeclarationRead::Moot, diag);
        // Bent alike, the wall's sheet and the floor's are one to the
        // order read ([`coplanar_wall_inside`]).
        let coplanar = || coplanar_wall_inside(normal, floor, arm, band, refused);
        let surface = |face: FaceKey| {
            body.get_face(face)
                .and_then(|d| body.get_surface(d.surface))
                .ok_or_else(desync)
        };
        inside[k] = match crate::validate::decide(
            "bool_rider_wall_side",
            Margin::levered(into.dot(floor), arm),
            band,
        )
        .map_err(sectors)?
        {
            Sign::Negative => true,
            Sign::Positive => false,
            // Tangent to the face along a straight line: the wall leaves
            // the line along the face's tangent plane, and its sheet lies
            // in the face's material where it bends off that plane
            // further toward the material than the face does
            // ([`RIDER_WALL_BEND_ACROSS`]): the difference of the two
            // normal sections' bends across the line ([`bend_across`], a
            // plane's none), against the face's outward normal, is the
            // second-order gap between the two sheets, `Δκ·s²/2` at arc
            // length `s` off the line, read at the wall's reach, in
            // metres. Bent alike, the two sheets are one to that order
            // ([`coplanar_wall_inside`]). On a curved face a wall going
            // in, or bent as the face is, would part the edge there,
            // which is neither meshed nor measured: it refuses
            // `CurvedFace`. A wall bending away leaves the face's material
            // alone.
            Sign::Zero => {
                let unread = if floor_flat {
                    EdgeInFaceGap::Tangent
                } else {
                    EdgeInFaceGap::CurvedFace
                };
                let (Some(wall_bend), Some(floor_bend), geom::Curve3::Line { .. }) = (
                    bend_across(surface(wall)?, mid, tangent),
                    bend_across(surface(f)?, mid, tangent),
                    curve.carrier(),
                ) else {
                    return Err(refused(unread));
                };
                let apart =
                    (floor_bend - wall_bend).dot(floor) * arm.powi(2) / (T::one() + T::one());
                let side = crate::validate::decide(RIDER_WALL_BEND_ACROSS, Margin::of(apart), band)
                    .map_err(sectors)?;
                match side {
                    Sign::Negative => false,
                    Sign::Positive | Sign::Zero if !floor_flat => {
                        return Err(refused(EdgeInFaceGap::CurvedFace));
                    }
                    Sign::Positive => {
                        flat[k] = true;
                        true
                    }
                    Sign::Zero => {
                        flat[k] = true;
                        coplanar()?
                    }
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

/// The site [`walls_inside`] decides a wall tangent to a curved face by
/// the two sheets' bends across the line under.
pub(super) const RIDER_WALL_BEND_ACROSS: &str = "bool_rider_wall_bend_across";

/// **Whether a wall lying on the floor's own sheet along the line goes
/// into the floor's material** (`bool_rider_wall_coplanar`): never.
/// Glued to the floor from outside (outward normals opposed), its
/// material is across the sheet from the floor's, and the edge keeps one
/// edge, as D1 says (`false`); facing as the floor does, it would lie
/// over the floor's own sheet, which refuses
/// [`EdgeInFaceGap::Tangent`]. The two unit normals' cosine is levered
/// at the wall's reach, `arm`, so the reading is in metres: how far the
/// wall's normal, carried across the wall, stands off the floor's.
fn coplanar_wall_inside<T: Decide>(
    normal: geom_core::Vec3<T>,
    floor: geom_core::Vec3<T>,
    arm: T,
    band: Band,
    refused: impl Fn(EdgeInFaceGap) -> BooleanError,
) -> Result<bool, BooleanError> {
    match crate::validate::decide_nonzero(
        "bool_rider_wall_coplanar",
        Margin::levered(normal.dot(floor), arm),
        band,
    )
    .map_err(|diag| BooleanError::coincidence(Coincide::Sectors, DeclarationRead::Moot, diag))?
    {
        geom_core::k_stats::NonzeroSign::Negative => Ok(false),
        geom_core::k_stats::NonzeroSign::Positive => Err(refused(EdgeInFaceGap::Tangent)),
    }
}

/// The way from `p` to its foot on the axis through `origin` along the
/// unit `axis`, square to the axis.
fn toward_axis<T: Decide>(
    origin: Point3<T>,
    axis: geom_core::Vec3<T>,
    p: Point3<T>,
) -> geom_core::Vec3<T> {
    origin + axis * (p - origin).dot(axis) - p
}

/// The bend across a straight line on `surface` at `p`, `t` the line's
/// direction: the curvature vector of the surface's normal section
/// perpendicular to the line, toward its centre of curvature, in inverse
/// metres. A plane does not bend; a line on a cylinder or cone is a
/// ruling, across which the section bends by the component, off the
/// line, of the way to the axis over that way's length squared. `None`
/// for a surface whose section is not read here.
fn bend_across<T: Decide>(
    surface: &geom::Surface<T>,
    p: Point3<T>,
    t: geom_core::Vec3<T>,
) -> Option<geom_core::Vec3<T>> {
    let toward = match *surface {
        geom::Surface::Plane { .. } => return Some(geom_core::Vec3::zero()),
        geom::Surface::Cylinder { origin, axis, .. } => toward_axis(origin, axis, p),
        geom::Surface::Cone { apex, axis, .. } => toward_axis(apex, axis, p),
        _ => return None,
    };
    let off = toward - t * toward.dot(t);
    Some(off * (T::one() / toward.dot(toward)))
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
    let (start, end) = crate::query::edge_ends(body, edge);
    let wall = body.face_of_half_edge(h).ok_or_else(desync)?;
    let surfaces = [face, wall].map(|f| body.get_face(f).map(|d| d.surface));
    let [Some(floor_surface), Some(wall_surface)] = surfaces else {
        return Err(desync());
    };
    let (pa, pb) = crate::readback::vertex_point(body, start)
        .ok()
        .zip(crate::readback::vertex_point(body, end).ok())
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
    // The two-edge face: `h` from `start` to `end`, the minted minus half
    // back.
    let gusset = made.face;
    let tip_out = |v: VertexKey| if v == start { h } else { made.he_minus };
    let mut boundary: Vec<(VertexKey, HalfEdgeKey)> = Vec::new();
    for v in [start, end] {
        match loop_leaving(body, face, v).as_slice() {
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
    let mut fused = Fusions::new(LineageRows::Fusions);
    for &(v, w) in joined {
        let leaving = |x: VertexKey| match loop_leaving(body, face, x).as_slice() {
            [he] => Ok(*he),
            _ => Err(refused(EdgeInFaceGap::Shared)),
        };
        let (hw, hv) = (leaving(w)?, leaving(v)?);
        let joint = super::zip::Joint::between(body, hw, hv).ok_or_else(desync)?;
        let outer = body.get_face(face).ok_or_else(desync)?.outer;
        if matches!(joint, super::zip::Joint::Chord { .. })
            && proven(&body.half_edges, hw, EntityId::HalfEdge).parent_loop != outer
        {
            return Err(refused(EdgeInFaceGap::Pocket));
        }
        let p = crate::readback::vertex_point(body, w).map_err(|_| desync())?;
        fused.push(super::zip::fuse_by_joint(
            body,
            joint,
            (p, w),
            joint_desync,
            tol,
        )?)?;
    }
    // A joint across one loop parted the face: the line's two sides lie
    // in its two pieces.
    let floor_of = |he: HalfEdgeKey| body.face_of_half_edge(he).ok_or_else(desync);
    let sides = [floor_of(h)?, floor_of(made.he_minus)?];
    if divided.is_none() && sides[0] != sides[1] {
        divided = sides.into_iter().find(|&f| f != face);
    }
    Ok(Parting {
        parted: Parted {
            kept: edge,
            minted: made.edge,
            face,
            divided,
            ends: (fused.survivor(start), fused.survivor(end)),
            copies,
            fused,
        },
    })
}

/// What [`divide`] left: the edge's pieces, the edge's key holding the
/// first, and its vertex at each crossing.
struct Division {
    pieces: Vec<EdgeKey>,
    crossings: Vec<VertexKey>,
}

/// **Divides `edge`, lying in `faces` along part of its length, and each
/// edge of their loops it crosses**, where a record names a crossing:
/// an edge-edge record of the edge and an edge of the loops, read where
/// the two meet ([`geom_brep::curve_pair::meet`]), both divided
/// there and their new vertices holding the meeting's point; or a
/// vertex-on-edge record of a vertex of the loops, the edge alone divided
/// at the vertex's own point. Each such record becomes a vertex-vertex
/// record of the crossing's two vertices, citing what it cited, which
/// the parting joins ([`part`]); a vertex-on-edge record of a divided
/// edge goes to the piece holding the vertex, ordered against the
/// crossings along that edge ([`BooleanDecision::PartingOrder`]); and
/// the edge's edge-on-face records with `faces` are dropped: a piece
/// lying in one has both ends on it, and [`riders`] reads it by them.
///
/// # Errors
///
/// [`BooleanError::EdgeInFace`] where the division is not built:
/// [`EdgeInFaceGap::Unrecorded`], [`EdgeInFaceGap::CrossedCurve`],
/// [`EdgeInFaceGap::Overlap`], [`EdgeInFaceGap::Unplaced`],
/// [`EdgeInFaceGap::AtACrossing`], and [`EdgeInFaceGap::EndOnBoundary`]
/// where a crossing is the edge's own end ([`crossing_named`]);
/// [`BooleanError::Escalated`] where a meeting, a vertex on an edge, or
/// an order along one is in band; the splits' refusals.
fn divide<T: Decide + crate::props::AtRestPolicy>(
    body: &mut Body<T>,
    records: &mut ContactRecords,
    edge: EdgeKey,
    faces: &[FaceKey],
    band: Band,
    tol: Tol,
) -> Result<Division, BooleanError> {
    let desync = run_past_desync;
    let face = *faces.first().ok_or_else(desync)?;
    let refused = |gap| BooleanError::EdgeInFace { edge, face, gap };
    let loops: Vec<(FaceKey, BTreeSet<EdgeKey>)> =
        faces.iter().map(|&f| (f, face_edges(body, f))).collect();
    // The face whose loops hold `cell`, which a refusal about it names.
    // The face whose loops hold `cell`, which a refusal about it names:
    // the edge itself lies in all of them, and is named against the
    // first.
    let at_fault = |cell: Cell| match cell {
        Cell::Edge(e) if e == edge => Some(face),
        _ => loops
            .iter()
            .find(|(f, edges)| match cell {
                Cell::Edge(g) => edges.contains(&g),
                Cell::Vertex(v) => on_face_loops(body, *f, v),
                Cell::Face(_) => false,
            })
            .map(|&(f, _)| f),
    };
    let (carrier, span) = span_of(body, edge).ok_or_else(desync)?;
    let along = Span::new(&carrier, span);
    // The crossings, as their records name them.
    let mut crossings: Vec<Crossing<T>> = Vec::new();
    for row in records.rows() {
        for crossing in crossing_named(body, &row.record, edge, &loops, &along, band)? {
            // A corner two records name is one crossing.
            match crossings.iter_mut().find(|c| {
                matches!((c.crossed, crossing.crossed), (Crossed::Corner(a), Crossed::Corner(b)) if a == b)
            }) {
                Some(same) => same.named.extend(crossing.named),
                None => crossings.push(crossing),
            }
        }
    }
    if crossings.is_empty() {
        return Err(refused(EdgeInFaceGap::Unrecorded));
    }
    // An edge keeps `[t0, t]` at each split, so the crossings go from its
    // far end back: along the edge, and along each edge of the loops it
    // crosses, as indices into `ordered`.
    let mut ordered: Vec<Crossing<T>> = Vec::new();
    let crossed_cell = |crossed: Crossed<T>| match crossed {
        Crossed::Edge(g, _) => Cell::Edge(g),
        Crossed::Corner(v) => Cell::Vertex(v),
    };
    // A crossed edge, a crossing's corner or a resting vertex's edge
    // outside every face's loops is a crossing read against no face:
    // the division ran past its own reading.
    let refused_at = |cell: Cell, gap| match at_fault(cell) {
        Some(face) => BooleanError::EdgeInFace { edge, face, gap },
        None => desync(),
    };
    for crossing in crossings {
        let slot = slot_from_far_end(
            ordered.iter().map(|c| c.at),
            crossing.at,
            || refused_at(crossed_cell(crossing.crossed), EdgeInFaceGap::AtACrossing),
            band,
        )?;
        ordered.insert(slot, crossing);
    }
    let mut crossed_edges: BTreeMap<EdgeKey, Vec<(T, usize)>> = BTreeMap::new();
    for (k, crossing) in ordered.iter().enumerate() {
        if let Crossed::Edge(g, at) = crossing.crossed {
            let on_g = crossed_edges.entry(g).or_default();
            let slot = slot_from_far_end(
                on_g.iter().map(|&(t, _)| t),
                at,
                || refused_at(Cell::Edge(g), EdgeInFaceGap::AtACrossing),
                band,
            )?;
            on_g.insert(slot, (at, k));
        }
    }
    let divided = |e: EdgeKey| e == edge || crossed_edges.contains_key(&e);
    // Each vertex a record rests on a divided edge, by the record, with
    // the index along that edge of the piece holding it: how many of the
    // edge's crossings it lies past. Read before any split, on the
    // undivided edges.
    let mut resting: BTreeMap<[Cell; 2], Resting> = BTreeMap::new();
    // The edge's edge-on-face records with the faces it is divided
    // against: each piece lying in one has both ends on it.
    let edge_face: BTreeSet<[Cell; 2]> = faces
        .iter()
        .filter_map(|&f| Contact::of(Cell::Edge(edge), Cell::Face(f)).ok())
        .map(Contact::cells)
        .collect();
    for row in records.rows() {
        let named = row.record.cells();
        if ordered.iter().any(|c| c.named.contains(&named)) || edge_face.contains(&named) {
            continue;
        }
        match row.record.kind() {
            ContactKind::VertexEdge(vertex, on) if divided(on) => {
                let point = crate::readback::vertex_point(body, vertex).map_err(|_| desync())?;
                let (t, crossings): (T, Vec<T>) = if on == edge {
                    (
                        on_span(&along, point, band, refused(EdgeInFaceGap::Unplaced))?,
                        ordered.iter().map(|c| c.at).collect(),
                    )
                } else {
                    let (other, other_span) = span_of(body, on).ok_or_else(desync)?;
                    (
                        on_span(
                            &Span::new(&other, other_span),
                            point,
                            band,
                            refused_at(Cell::Edge(on), EdgeInFaceGap::Unplaced),
                        )?,
                        crossed_edges
                            .get(&on)
                            .ok_or_else(desync)?
                            .iter()
                            .map(|&(at, _)| at)
                            .collect(),
                    )
                };
                let mut past = 0;
                for at in crossings {
                    past += usize::from(parting_order(t, at, band, || {
                        refused_at(Cell::Edge(on), EdgeInFaceGap::AtACrossing)
                    })?);
                }
                resting.insert(named, Resting { vertex, on, past });
            }
            _ => {
                if let Some(&c) = named
                    .iter()
                    .find(|&&c| matches!(c, Cell::Edge(e) if divided(e)))
                {
                    return Err(refused_at(c, EdgeInFaceGap::Unplaced));
                }
            }
        }
    }
    let mut pieces = vec![edge];
    let mut ends = Vec::new();
    // Each crossing's two vertices, by the records naming it.
    let mut joined: Vec<(Vec<[Cell; 2]>, VertexKey, VertexKey)> = Vec::new();
    // Each divided edge's pieces along it, from its start.
    let mut along_pieces: BTreeMap<EdgeKey, Vec<EdgeKey>> = BTreeMap::new();
    // Each crossing's vertex on the edge of the loops it crosses.
    let mut on_loops: BTreeMap<usize, VertexKey> = BTreeMap::new();
    for (&g, on_g) in &crossed_edges {
        let mut minted = Vec::new();
        for &(at, k) in on_g {
            let split = body.split_edge_onto(g, at, ordered[k].point, tol)?;
            minted.push(split.new_edge);
            on_loops.insert(k, split.vertex);
        }
        along_pieces.insert(
            g,
            core::iter::once(g)
                .chain(minted.into_iter().rev())
                .collect(),
        );
    }
    for (k, crossing) in ordered.iter().enumerate() {
        let on_boundary = match crossing.crossed {
            Crossed::Edge(..) => *on_loops.get(&k).ok_or_else(desync)?,
            Crossed::Corner(corner) => corner,
        };
        let split = body.split_edge_onto(edge, crossing.at, crossing.point, tol)?;
        pieces.push(split.new_edge);
        ends.push(split.vertex);
        joined.push((crossing.named.clone(), split.vertex, on_boundary));
    }
    along_pieces.insert(
        edge,
        core::iter::once(edge)
            .chain(pieces[1..].iter().rev().copied())
            .collect(),
    );
    let mut rewritten = ContactRecords::default();
    for row in records.rows() {
        let named = row.record.cells();
        let at_crossings: Vec<_> = joined.iter().filter(|(n, ..)| n.contains(&named)).collect();
        if !at_crossings.is_empty() {
            for &(_, on_edge, on_boundary) in at_crossings {
                let record = Contact::new(body, Cell::Vertex(on_edge), Cell::Vertex(on_boundary))
                    .map_err(|_| desync())?;
                rewritten.push(Cited::new(record, row.cites.clone()));
            }
            continue;
        }
        let record = if let Some(rest) = resting.get(&named) {
            let piece = along_pieces
                .get(&rest.on)
                .and_then(|p| p.get(rest.past))
                .ok_or_else(desync)?;
            Contact::new(body, Cell::Vertex(rest.vertex), Cell::Edge(*piece))
        } else if edge_face.contains(&named) {
            continue;
        } else {
            rewritten.push(row.clone());
            continue;
        };
        rewritten.push(Cited::new(record.map_err(|_| desync())?, row.cites.clone()));
    }
    *records = rewritten;
    Ok(Division {
        pieces,
        crossings: ends,
    })
}

/// **The crossings `record` names of `edge`, its span `along`, with the
/// loops of the faces `loops` lists with their edges** ([`divide`]): an
/// edge-edge record with an edge of the loops, read where the two meet
/// ([`geom_brep::curve_pair::meet`]), once or twice (a line across an
/// arc); or a vertex-on-edge record of a vertex of the loops on the
/// edge, at the vertex's own point. None for any other record.
///
/// # Errors
///
/// [`BooleanError::EdgeInFace`], naming the face whose loops the record
/// names, where the record names no crossing inside the edge:
/// [`EdgeInFaceGap::Overlap`] for a shared run,
/// [`EdgeInFaceGap::Unrecorded`] for no meeting,
/// [`EdgeInFaceGap::EndOnBoundary`] for one at the edge's end,
/// [`EdgeInFaceGap::Unplaced`] for a vertex off the edge's interior,
/// and [`EdgeInFaceGap::CrossedCurve`] where the meeting has no closed
/// form; [`BooleanError::Escalated`] where the meeting or the vertex's
/// place is in band.
fn crossing_named<T: Decide>(
    body: &Body<T>,
    record: &Contact,
    edge: EdgeKey,
    loops: &[(FaceKey, BTreeSet<EdgeKey>)],
    along: &Span<'_, T>,
    band: Band,
) -> Result<Vec<Crossing<T>>, BooleanError> {
    let named = record.cells();
    Ok(match record.kind() {
        ContactKind::EdgeEdge(x, y) if x == edge || y == edge => {
            let crossed = if x == edge { y } else { x };
            let Some(&(face, _)) = loops.iter().find(|(_, edges)| edges.contains(&crossed)) else {
                return Ok(Vec::new());
            };
            let refused = |gap| BooleanError::EdgeInFace { edge, face, gap };
            let (other, other_span) =
                span_of(body, crossed).ok_or_else(|| refused(EdgeInFaceGap::CrossedCurve))?;
            let met = meet(along, &Span::new(&other, other_span), band)
                .map_err(|r| meet_refused(r.fault, refused(EdgeInFaceGap::CrossedCurve)))?;
            if met.shared.is_some() {
                return Err(refused(EdgeInFaceGap::Overlap));
            }
            if met.points.is_empty() {
                return Err(refused(EdgeInFaceGap::Unrecorded));
            }
            let mut out = Vec::new();
            for m in met.points {
                let Hit::On { t, .. } = m.on_a else {
                    return Err(refused(EdgeInFaceGap::EndOnBoundary));
                };
                let crossed = match m.on_b {
                    Hit::On { t: on_crossed, .. } => Crossed::Edge(crossed, on_crossed),
                    Hit::End(which) => {
                        let (first, last) = crate::query::edge_ends(body, crossed);
                        Crossed::Corner(match which {
                            SpanEnd::Start => first,
                            SpanEnd::Stop => last,
                        })
                    }
                };
                out.push(Crossing {
                    at: t,
                    point: m.point,
                    crossed,
                    named: vec![named],
                });
            }
            out
        }
        ContactKind::VertexEdge(corner, e) if e == edge => {
            let Some(&(face, _)) = loops.iter().find(|&&(f, _)| on_face_loops(body, f, corner))
            else {
                return Ok(Vec::new());
            };
            let refused = |gap| BooleanError::EdgeInFace { edge, face, gap };
            let point =
                crate::readback::vertex_point(body, corner).map_err(|_| run_past_desync())?;
            vec![Crossing {
                at: on_span(along, point, band, refused(EdgeInFaceGap::Unplaced))?,
                point,
                crossed: Crossed::Corner(corner),
                named: vec![named],
            }]
        }
        _ => Vec::new(),
    })
}

/// Where `t` goes in `placed`, parameters along one edge from its far
/// end back ([`parting_order`]).
///
/// # Errors
///
/// `at_one_point()` where `t` is one of them; [`BooleanError::Escalated`]
/// where an order is in band.
fn slot_from_far_end<T: Decide>(
    placed: impl Iterator<Item = T>,
    t: T,
    at_one_point: impl Fn() -> BooleanError,
    band: Band,
) -> Result<usize, BooleanError> {
    let mut count = 0;
    for (k, at) in placed.enumerate() {
        if parting_order(t, at, band, &at_one_point)? {
            return Ok(k);
        }
        count = k + 1;
    }
    Ok(count)
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
    /// The records naming the crossing, by their cells.
    named: Vec<[Cell; 2]>,
}

/// The cell of a face's loops an edge crosses ([`Crossing`]).
#[derive(Clone, Copy)]
enum Crossed<T> {
    /// A vertex of the loops.
    Corner(VertexKey),
    /// An edge of the loops, with its parameter at the crossing.
    Edge(EdgeKey, T),
}

/// A vertex a record rests on an edge [`divide`] divides, and the index
/// along that edge of the piece holding it.
struct Resting {
    vertex: VertexKey,
    on: EdgeKey,
    past: usize,
}

/// An edge's certified carrier and its span.
fn span_of<T: Decide>(body: &Body<T>, e: EdgeKey) -> Option<(geom::Curve3<T>, (T, T))> {
    let curve = body
        .get_edge(e)
        .and_then(|d| body.get_curve_geom(d.curve))
        .and_then(|c| c.certified())?;
    Some((curve.carrier().clone(), curve.params()))
}

/// Where `point` sits inside `span`: its parameter there, or `outside`
/// where it reads off the span or at an end of it.
///
/// # Errors
///
/// `outside` as above; [`BooleanError::Escalated`] where the reading is
/// in band.
fn on_span<T: Decide>(
    span: &Span<'_, T>,
    point: Point3<T>,
    band: Band,
    outside: BooleanError,
) -> Result<T, BooleanError> {
    let (t0, t1) = span.params();
    match span.contact(point, geom::mid_param(t0, t1), band) {
        Ok(Some(Hit::On { t, .. })) => Ok(t),
        Ok(Some(Hit::End(_)) | None) => Err(outside),
        Err(geom::CarrierRefusal::Escalated(diag) | geom::CarrierRefusal::Straddle(diag)) => {
            Err(crossing_escalated(diag))
        }
        Err(geom::CarrierRefusal::Exhausted | geom::CarrierRefusal::Unbounded) => Err(outside),
    }
}

/// Whether `a` lies past `b` along an edge [`divide`] divides
/// ([`BooleanDecision::PartingOrder`]), two parameters of its carrier.
///
/// # Errors
///
/// `at_one_point()` ([`EdgeInFaceGap::AtACrossing`]) where the two are one
/// point; [`BooleanError::Escalated`] where the order is in band.
fn parting_order<T: Decide>(
    a: T,
    b: T,
    band: Band,
    at_one_point: impl FnOnce() -> BooleanError,
) -> Result<bool, BooleanError> {
    match crate::validate::decide(PARTING_ORDER, Margin::of(a - b), band) {
        Ok(Sign::Positive) => Ok(true),
        Ok(Sign::Negative) => Ok(false),
        Ok(Sign::Zero) => Err(at_one_point()),
        Err(diag) => Err(BooleanError::Escalated {
            decision: BooleanDecision::PartingOrder,
            diag,
        }),
    }
}

/// The site [`parting_order`] decides under.
pub(super) const PARTING_ORDER: &str = "bool_parting_order";

/// An in-band reading of where a crossing lies on an edge, or where two
/// edges meet.
fn crossing_escalated(diag: geom_core::Indeterminate) -> BooleanError {
    BooleanError::Escalated {
        decision: BooleanDecision::Crossing(crate::splitting::CrossingDecision::OnEdge),
        diag,
    }
}

/// [`divide`]'s refusal of a meeting of an edge with an edge of a
/// face's loops: `unread` where the reader's walk or root finder does
/// not settle it.
fn meet_refused(fault: MeetFault, unread: BooleanError) -> BooleanError {
    match fault {
        MeetFault::Escalated(diag) | MeetFault::Touch(diag) => crossing_escalated(diag),
        MeetFault::Unsettled(_) | MeetFault::Unsupported { .. } => unread,
        MeetFault::Desync | MeetFault::Contradiction { .. } => {
            BooleanError::ClassificationInvariant {
                what: "the meeting reader contradicted itself on two edges' carriers",
            }
        }
    }
}

/// An edge [`divide`] divides that no longer resolves.
fn run_past_desync() -> BooleanError {
    BooleanError::JoinDesync {
        what: "an edge running past a face does not resolve",
    }
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
    if on_face_loops(body, face, v) {
        return false;
    }
    let boundary = face_edges(body, face);
    contacts
        .vertex_edge()
        .any(|(x, g)| x == v && boundary.contains(&g))
}

/// The vertices a vertex-vertex record puts on `v`'s point.
fn partners(contacts: &ContactRecords, v: VertexKey) -> impl Iterator<Item = VertexKey> + '_ {
    contacts.vertex_vertex().filter_map(move |(x, y)| {
        if x == v {
            Some(y)
        } else if y == v {
            Some(x)
        } else {
            None
        }
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
    partners(contacts, v)
        .filter(|&w| on_face_loops(body, face, w))
        .collect()
}

/// Whether a loop of `face` passes through `v`.
fn on_face_loops<T: Decide>(body: &Body<T>, face: FaceKey, v: VertexKey) -> bool {
    !loop_leaving(body, face, v).is_empty()
}

/// The half-edges of `face`'s loops leaving `v`: one per pass of a loop
/// through it.
fn loop_leaving<T: Decide>(body: &Body<T>, face: FaceKey, v: VertexKey) -> Vec<HalfEdgeKey> {
    face_halves(body, face)
        .unwrap_or_default()
        .into_iter()
        .filter(|&he| body.get_half_edge(he).is_some_and(|h| h.start == v))
        .collect()
}

/// The edges of `face`'s loops.
fn face_edges<T: Decide>(body: &Body<T>, face: FaceKey) -> BTreeSet<EdgeKey> {
    face_halves(body, face)
        .unwrap_or_default()
        .into_iter()
        .filter_map(|he| body.get_half_edge(he).map(|h| h.edge))
        .collect()
}

/// Whether a record puts `v` inside an edge.
fn touches_an_edge(contacts: &ContactRecords, v: VertexKey) -> bool {
    contacts.vertex_edge().any(|(x, _)| x == v)
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
