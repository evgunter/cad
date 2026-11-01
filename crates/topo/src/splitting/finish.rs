//! Finish (ch. 14 §14.8, `splitfinish`): promote each completed null
//! face into TWO section faces (`mfkrh` — ring → face, the cross-shell
//! motion), distribute the now-disconnected components into shells
//! (`movefac`), classify each shell Above/Below, and **carve** the two
//! result bodies — functionally: the operand was never touched (the
//! pipeline runs on a clone), and both results come back as
//! independent [`Body`] values.
//!
//! # Section-face orientation
//!
//! The above body's section face carries **m = −n_SP**, the below
//! body's **m = +n_SP** (derived at [`section_loops`]). Both faces
//! carry the SAME split plane (same origin, same in-plane `u_ref`: the
//! below loop's first chord projected into the plane, or for a loop of
//! one corner — a whole section conic — the first axis of the run
//! plane's normal basis) with opposite normals; the mirror test pins
//! both signs bitwise.
//!
//! That is each face's CHART normal. Its sense is its loop's winding
//! about it, the reading tier 3's check 6 makes: a section's outer
//! boundary winds counter-clockwise (`true`). A section polygon that
//! is a hole in another — the bore's outline inside the cut through a
//! bored block — winds clockwise (`false`), and becomes a ring of the
//! section face of its side that encloses it by the section nesting
//! rule (`section_loops::nest`, applied in `nest_hole_sections`), so a
//! holed section is one face; where nothing decides it, the hole keeps
//! a face of its own that cancels the face around it.
//!
//! The book's "the 'inner' loop should appear in the part Above, and
//! the 'outer' loop in the part Below" is list-position convention
//! chasing; here the roles are the F9 keys, which promotion reads off
//! the join's `CompletedSection` (`above_loop`, `below_loop`), never
//! off the loop list.
//!
//! # Component classification and the degenerate net
//!
//! After `movefac`, a shell is classified by its section faces (seed
//! knowledge, Program 14.12) — mixed above/below section faces in one
//! shell is a typed kernel-bug error; a shell with NO section face
//! (an uncut component) falls back to its first vertex's cached side.
//! A shell consisting **only** of section faces bounds no volume. The
//! join's zero-area check refuses every area section that could make
//! one, so such a shell is a kernel defect, refused loudly
//! ([`SplitFinishError::DegenerateSide`]).
//!
//! # Slit traces
//!
//! A slit trace ([`super::join::SectionKind::Slit`]) is a tree of edges
//! of the operand lying in the plane whose other side's material wraps
//! round its every end. Its null face parts into two faces at
//! promotion. The strut loop's face, the tree walked round, is a
//! component of its own that neither side keeps: the side that held
//! the edges loses them and their vertices. The slit's face is nested
//! as a ring of the section face of its side containing it
//! (`nest_slit`), so that side carries each edge as two coincident
//! distinct edges of a zero-area ring (`crates/topo/README.md`, tier 3).
//!
//! # Maximal faces (F7)
//!
//! Faces of the operand lying IN the split plane survive as walls of
//! one side (rule (a)), beside that side's section faces, and two
//! section faces of one side can meet along an operand edge lying in
//! the plane (a pinch's tip line). Each side therefore ends with
//! [`Body::merge_coplanar_faces`] ([`end_maximal`]), which ends with
//! the join, so a side has maximal faces, except where a recorded skip
//! leaves a curved run unglued, and maximal edges (`docs/DESIGN.md`),
//! and is an operand any boolean accepts. The merge is not silent: its
//! groups, its skips and its joins are recorded in
//! [`SplitNaming::merge_groups`], [`SplitNaming::merge_skipped`] and
//! [`SplitNaming::edge_joins`].
//!
//! A face-coplanar cut also lands OPERAND edges on the section
//! boundary with their transverse partner faces reassigned to the
//! other product, leaving intrinsic citations that no longer name the
//! edge's adjacent pair. The describe pass restates each one in what
//! the must-carry rule demands over its current pair
//! (`describe_section_boundary`'s smooth arm) — for two flush planes,
//! whose zero second order under-determines the locus, the pair
//! (`OnPair`) — so a coplanar product's boundary descriptions are
//! adjacency-coherent at rest; on a NEAR-flush operand the same
//! restatement meters the edge's incidence on both planes, and a line
//! chord the band cannot decide refuses as the edge's own reading
//! ([`SplitFinishError::EdgeOnFacesEscalated`]).

use geom_core::{Decide, Real, UnitVec3, Vec3};
use slotmap::SecondaryMap;

use super::containment::{LoopContainment, point_in_loop};
use super::join::{CompletedSection, SectionKind, loop_points_of};
use super::{KnifeEdge, KnifeEdgeSite, PlaneSide, SplitReduction, section_loops};
use crate::attach::{Named, Rechart};
use crate::body::Body;
use crate::boolean::{CarryRefusal, Cell};
use crate::chord_join::SplitJoinError;
use crate::entity::{EdgeKey, FaceKey, LoopBoundary, ShellKey, SolidKey, VertexKey};
use crate::euler::{EulerOpError, FaceSurface};
use geom::Surface;
use geom_core::Tol;

/// One side of a split result: a real body, or the typed empty side
/// (the plane missed the material on that side entirely — never an
/// empty `Body` value).
// The size skew against `Empty` is inherent (a Body is ~20 words of
// arena headers) and the value is moved at most once out of `split`;
// boxing would tax every real-result access to slim the rare Empty.
#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
pub enum SplitPart<T: Real> {
    /// The side's material, as an independent body, finished: the split
    /// gated it at rest ([`crate::AtRestPolicy::gate_at_rest_kept`]) and
    /// keeps the verdict, as a Boolean's result does
    /// ([`crate::BooleanBody::body`]). A caller wanting the body by value
    /// takes [`crate::AtRestBody::into_body`].
    Body(crate::AtRestBody<T>),
    /// No material on this side.
    Empty,
}

impl<T: Real> SplitPart<T> {
    /// The body, if this side has material.
    pub fn body(&self) -> Option<&crate::AtRestBody<T>> {
        match self {
            Self::Body(b) => Some(b),
            Self::Empty => None,
        }
    }
}

/// One side of the split pipeline's own result, before the side is gated
/// at rest ([`SplitPart`] once it is).
#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
pub(crate) enum RawPart<T: Real> {
    /// The side's material, as the pipeline built it.
    Body(Body<T>),
    /// No material on this side.
    Empty,
}

/// The split pipeline's own result: [`SplitResult`] before its sides are
/// gated at rest (`super::split`).
#[derive(Debug)]
pub(crate) struct RawSplit<T: Real> {
    pub(crate) above: RawPart<T>,
    pub(crate) below: RawPart<T>,
    pub(crate) naming: SplitNaming,
    pub(crate) coincidences: Vec<crate::Coincidence>,
}

/// The result of a plane split: both sides, functionally.
#[derive(Debug)]
pub struct SplitResult<T: Real> {
    /// The material on the plane normal's side.
    pub above: SplitPart<T>,
    /// The material on the opposite side.
    pub below: SplitPart<T>,
    /// Naming emission (M4 PR 3, NAMING-DESIGN N4): the mint-time
    /// wiring facts the naming layer consumes — never reconstructed
    /// by post-hoc inspection.
    pub naming: SplitNaming,
    /// The coincidences the split decided from values (its pinches), in
    /// the operand's keys ([`crate::coincidence`]).
    pub coincidences: Vec<crate::Coincidence>,
}

/// The operand's contact records carried onto the split's two sides
/// ([`SplitResult::carry_contacts`]): each side's own list, each record
/// citing the operand's record it is.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SplitContacts {
    /// The records the above side holds.
    pub above: crate::boolean::ContactRecords,
    /// The records the below side holds.
    pub below: crate::boolean::ContactRecords,
}

impl<T: Decide> SplitResult<T> {
    /// `records`, the operand's, carried onto the split's two sides
    /// through [`Self::lineage`] ([`crate::ContactRecords::carried_through`],
    /// at `band` through `region`), each side's records citing the
    /// operand's.
    ///
    /// # Errors
    ///
    /// As [`crate::ContactRecords::carried_through`]; [`CarryRefusal::Parted`]
    /// for a record whose cells the cut put on different sides.
    pub fn carry_contacts(
        &self,
        records: &crate::boolean::ContactRecords,
        band: geom_core::Band,
        region: Option<crate::RegionLane<T>>,
    ) -> Result<SplitContacts, CarryRefusal> {
        let sides: Vec<(PlaneSide, &Body<T>)> = [
            (PlaneSide::Above, self.above.body()),
            (PlaneSide::Below, self.below.body()),
        ]
        .into_iter()
        .filter_map(|(side, body)| body.map(|b| (side, &**b)))
        .collect();
        let bodies: Vec<&Body<T>> = sides.iter().map(|&(_, b)| b).collect();
        let carried = records.carried_onto(0, &self.lineage(), &bodies, band, region)?;
        let mut out = SplitContacts::default();
        for ((side, _), list) in sides.iter().zip(carried) {
            match side {
                PlaneSide::Above => out.above = list,
                _ => out.below = list,
            }
        }
        Ok(out)
    }
}

impl<T: Real> SplitResult<T> {
    /// **What the split made of each operand cell** (D1), in the scratch
    /// keys both sides were carved from, read through each side's merges and joins:
    /// a vertex the cut copied is kept under its key and its copies
    /// (`SplitNaming::vertex_pairs`); an edge the cut split
    /// (`SplitNaming::edge_pieces`) or a face it divided
    /// (`SplitNaming::face_fragments`, the section faces excluded) is
    /// divided, exhaustively, into its pieces and the cells the cut
    /// minted between them, which are the cells of its pieces on a
    /// section face. One plane made every cell the cut minted, so two
    /// it minted on a record's two cells meet where the record puts them.
    #[must_use]
    pub fn lineage(&self) -> crate::Lineage {
        use crate::lineage::{Descent, Division, Fate, Remainder, roots};
        let bodies: Vec<&Body<T>> = [self.above.body(), self.below.body()]
            .into_iter()
            .flatten()
            .map(|b| &**b)
            .collect();
        let section = |f: FaceKey| self.naming.holds_section(f);
        let joins: Vec<crate::boolean::EdgeJoin> =
            self.naming.edge_joins.iter().map(|&(_, j)| j).collect();
        // Each copied vertex, and each root of the edge and face rows, is
        // stated once.
        let stated = |lineage: crate::Lineage, cell: Cell, fate: Fate| {
            lineage
                .with(cell, fate)
                .unwrap_or_else(|e| unreachable!("the split states each cell once: {e:?}"))
        };
        let merged = self
            .naming
            .merge_groups
            .iter()
            .flat_map(|(_, g)| g.substitutions());
        let mut lineage = crate::Lineage::new()
            .through_fusions(&self.naming.vertex_fusions)
            .through_merges(merged)
            .through_joins(&joins)
            .one_cut();
        let mut copies: Vec<(crate::entity::VertexKey, Vec<Cell>)> = Vec::new();
        for &(copy, original) in &self.naming.vertex_pairs {
            match copies.iter_mut().find(|(o, _)| *o == original) {
                Some((_, cs)) => cs.push(Cell::Vertex(copy)),
                None => copies.push((original, vec![Cell::Vertex(copy)])),
            }
        }
        for (original, copies) in copies {
            let at = Cell::Vertex(original);
            lineage = stated(lineage, at, Fate::Kept { at, copies });
        }
        for root in roots(self.naming.edge_pieces.rows()) {
            let pieces: Vec<EdgeKey> = Descent::of(root, self.naming.edge_pieces.rows())
                .cells()
                .collect();
            let mut minted = Vec::new();
            for body in &bodies {
                let on_section = |v: crate::entity::VertexKey| {
                    body.faces_of_vertex(v)
                        .is_some_and(|fs| fs.into_iter().any(section))
                };
                for &e in &pieces {
                    let Some(ends) = body.get_edge(e).and_then(|d| {
                        Some([
                            body.get_half_edge(d.he_plus)?.start,
                            body.get_half_edge(d.he_minus)?.start,
                        ])
                    }) else {
                        continue;
                    };
                    for v in ends.into_iter().filter(|&v| on_section(v)) {
                        if !minted.contains(&Cell::Vertex(v)) {
                            minted.push(Cell::Vertex(v));
                        }
                    }
                }
            }
            lineage = stated(
                lineage,
                Cell::Edge(root),
                Fate::Divided(Division {
                    pieces: pieces.into_iter().map(Cell::Edge).collect(),
                    minted,
                    remainder: Remainder::Exhaustive,
                }),
            );
        }
        let fragments: Vec<(FaceKey, FaceKey)> = self
            .naming
            .face_fragments
            .rows()
            .iter()
            .copied()
            .filter(|&(new, from)| !section(new) && !section(from))
            .collect();
        for root in roots(&fragments) {
            let pieces: Vec<FaceKey> = Descent::of(root, &fragments).cells().collect();
            let mut minted = Vec::new();
            for body in &bodies {
                for (g, _) in body.edges() {
                    let Ok(sides) = crate::readback::edge_sides(body, g) else {
                        continue;
                    };
                    let (f0, f1) = sides.faces();
                    let chord = (section(f0) && pieces.contains(&f1))
                        || (section(f1) && pieces.contains(&f0));
                    if chord && !minted.contains(&Cell::Edge(g)) {
                        minted.push(Cell::Edge(g));
                    }
                }
            }
            lineage = stated(
                lineage,
                Cell::Face(root),
                Fate::Divided(Division {
                    pieces: pieces.into_iter().map(Cell::Face).collect(),
                    minted,
                    remainder: Remainder::Exhaustive,
                }),
            );
        }
        lineage
    }
}

/// Mint-time naming facts of one split (M4 PR 3). Keys live in the
/// scratch arena both result bodies were carved from — `carve` clones,
/// so a key resolves in whichever side kept the entity (and only
/// there). Rows are historical: entries whose entity survived in
/// neither side (discarded scaffolding) simply resolve nowhere.
#[derive(Debug)]
pub struct SplitNaming {
    /// The section faces each side keeps, with their side, in section
    /// completion order (the join's sweep order and its per-face line
    /// order are recorded predicate verdicts, so the position is a
    /// function of the verdict vector — N4's covariance). A section
    /// polygon nested as a ring of another (`section_loops::nest`) has
    /// no face of its own and no row; the face that took it as a ring
    /// keeps its own row.
    pub sections: Vec<(FaceKey, PlaneSide)>,
    /// Chord-mef fragment rows: `(new face, divided-from face)` in
    /// mint order, call-time keys ([`crate::chord_join`]'s `ChordJoiner`
    /// log). Section faces appear here too (they are minted by the
    /// same mefs); consumers exclude the keys listed in `sections`.
    pub face_fragments: crate::lineage::FaceDivisions,
    /// Crossing-split piece rows: `(new edge, divided-from edge)` in
    /// mint order, call-time keys ([`crate::Body::split_edge`](crate::construction::Construct::split_edge) keeps the
    /// divided edge's key on the piece from its start), each checked as
    /// it was minted.
    pub edge_pieces: crate::lineage::SplitEdgeDivisions,
    /// Null-edge vertex pairs `(copy, original)` from the reduction's
    /// F9 records, in record order: each coincident copy with the
    /// vertex it was minted at (the naming layer derives the copy's
    /// parentage through the original's birth record). Which side
    /// holds which is not fixed — a copy stands for whichever piece its
    /// corner's arc holds — so consumers read a key's side from the body
    /// that holds it.
    pub vertex_pairs: Vec<(crate::entity::VertexKey, crate::entity::VertexKey)>,
    /// The joins each side ended with ([`crate::Body::join_edges`](crate::construction::Construct::join_edges)), in
    /// the order made, with the side whose body they were made on: each
    /// row's `vertex` and `gone` edge are dead there, and `kept` holds
    /// them (`docs/DESIGN.md`, maximal edges).
    pub edge_joins: Vec<(PlaneSide, crate::boolean::EdgeJoin)>,
    /// Killed edge → the edge it was split from, for each edge a side's
    /// join killed that held a `SplitEdge` record: a kill takes its
    /// edge's birth record with it, and this is the lineage a joined
    /// edge's cover is read through to the operand edges it lies along.
    /// A copy of the records as they stood before the kills, not a list
    /// of mints: a join can kill a child before its parent
    /// ([`crate::provenance::Provenance::SplitEdge`] says why a copy is
    /// as acyclic as the records).
    pub joined_lineage: std::collections::BTreeMap<crate::entity::EdgeKey, crate::entity::EdgeKey>,
    /// The coplanar merges each side ended with
    /// ([`Body::merge_coplanar_faces`], [`end_maximal`]), in group
    /// order, with the side whose body they were made on: each group's
    /// `absorbed` faces and `killed_edges` are dead there and its
    /// `kept` face holds them. A section face a group absorbed keeps
    /// its row in [`Self::sections`].
    pub merge_groups: Vec<(PlaneSide, crate::merge_faces::MergedGroup)>,
    /// The curved groups each side's merge left unglued as outside its
    /// inventory ([`crate::MergeCoplanarOutcome::skipped`]), with their
    /// side: a run that would close its chart's full period, such as a
    /// two-piece cylinder wall cut across its axis. Those faces stay as
    /// the cut left them, their boundaries re-described ([`end_maximal`]).
    pub merge_skipped: Vec<(PlaneSide, crate::merge_faces::SkippedMerge)>,
    /// Which ring holds which island, in each side as it ends
    /// ([`end_maximal`]): `(ring, island)` for each of the side's plane
    /// pieces ([`Self::plane_pieces`]) whose outer loop lies in a ring of
    /// another, with the innermost such ring, read by the section nesting
    /// rule ([`section_loops::nest`]). A piece in no ring has no row.
    pub islands: Vec<(crate::entity::LoopKey, FaceKey)>,
    /// The fusions of a pinch's cones each side made
    /// ([`super::section_cones`]), `(dead, kept)` in the order their
    /// `kev`s ran, before the side's merges and joins: a dead vertex is
    /// held by the vertex that survives it, a copy as well as an
    /// original ([`Self::vertex_pairs`]).
    pub vertex_fusions: crate::lineage::Fusions,
}

/// A side's faces that hold part of the tool plane
/// ([`SplitNaming::plane_pieces`]).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PlanePieces {
    /// The operand faces the side's merges linked to the plane: each
    /// face, other than a section face, that a group holding a section
    /// kept or absorbed, read through [`SplitNaming::face_fragments`] to
    /// the operand face it descends from, and transitively each such
    /// face of a group that shares one.
    pub roots: std::collections::BTreeSet<FaceKey>,
    /// The faces those groups kept.
    pub kept: std::collections::BTreeSet<FaceKey>,
    /// The side's live faces that hold part of the plane: the kept faces,
    /// every section face, and every other piece of a linked operand face
    /// that no merge kept, in the body's face order.
    pub faces: Vec<FaceKey>,
}

impl SplitNaming {
    /// **The operand vertex `v` stands for**: `v` itself, or the end of
    /// its chain of null-pair rows ([`Self::vertex_pairs`]), a copy of a
    /// copy chased to the original.
    ///
    /// # Panics
    ///
    /// Where the rows cycle: each row is a copy with the vertex the null
    /// edge that minted it was minted at, so it names an earlier key.
    #[must_use]
    pub fn original_of(&self, v: crate::entity::VertexKey) -> crate::entity::VertexKey {
        let mut at = v;
        for _ in 0..=self.vertex_pairs.len() {
            match self.vertex_pairs.iter().find(|&&(copy, _)| copy == at) {
                Some(&(_, original)) => at = original,
                None => return at,
            }
        }
        unreachable!(
            "the null-pair rows cycle from {v:?}: each copy names the vertex it was minted at, \
             an earlier key"
        )
    }

    /// Records one side's joins, with the side's `SplitEdge` records as
    /// they stood before the kills (`split_from`).
    pub(super) fn record_joins(
        &mut self,
        side: PlaneSide,
        split_from: &std::collections::BTreeMap<crate::entity::EdgeKey, crate::entity::EdgeKey>,
        joins: Vec<crate::boolean::EdgeJoin>,
    ) {
        crate::boolean::edge_join::record_killed_lineage(
            &mut self.joined_lineage,
            split_from,
            joins.iter().map(|j| j.gone),
        );
        self.edge_joins.extend(joins.into_iter().map(|j| (side, j)));
    }

    /// Whether `face` is a section face, or the face a merge kept for a
    /// group that holds one: the faces whose boundary the cut drew.
    #[must_use]
    pub fn holds_section(&self, face: FaceKey) -> bool {
        let section = |f: FaceKey| self.sections.iter().any(|&(s, _)| s == f);
        section(face)
            || self
                .merge_groups
                .iter()
                .any(|(_, g)| g.kept == face && g.absorbed.iter().copied().any(section))
    }

    /// `side`'s faces in `body` that hold part of the tool plane, and the
    /// operand faces its merges linked to the plane ([`PlanePieces`]).
    /// The plane links a merge group that holds a section as an operand
    /// face links the groups that share it, so a piece of the plane is a
    /// section face, a face a linked group kept, or a piece of a linked
    /// operand face lying in the plane beside them.
    #[must_use]
    pub fn plane_pieces<T: Real>(&self, side: PlaneSide, body: &Body<T>) -> PlanePieces {
        let section = |f: FaceKey| self.sections.iter().any(|&(s, _)| s == f);
        let groups: Vec<(bool, FaceKey, std::collections::BTreeSet<FaceKey>)> = self
            .merge_groups
            .iter()
            .filter(|(s, _)| *s == side)
            .map(|(_, g)| {
                let cells = core::iter::once(g.kept).chain(g.absorbed.iter().copied());
                let holds = cells.clone().any(section);
                let roots = cells
                    .filter(|&c| !section(c))
                    .map(|c| self.face_fragments.root(c))
                    .collect();
                (holds, g.kept, roots)
            })
            .collect();
        let mut out = PlanePieces::default();
        let mut linked = vec![false; groups.len()];
        loop {
            let mut grew = false;
            for (i, (holds, kept, roots)) in groups.iter().enumerate() {
                if !linked[i] && (*holds || !roots.is_disjoint(&out.roots)) {
                    linked[i] = true;
                    grew = true;
                    out.kept.insert(*kept);
                    out.roots.extend(roots.iter().copied());
                }
            }
            if !grew {
                break;
            }
        }
        let kept_anywhere = |f: FaceKey| self.merge_groups.iter().any(|(_, g)| g.kept == f);
        out.faces = body
            .faces()
            .map(|(f, _)| f)
            .filter(|&f| {
                out.kept.contains(&f)
                    || section(f)
                    || (!kept_anywhere(f) && out.roots.contains(&self.face_fragments.root(f)))
            })
            .collect();
        out
    }
}

/// Ends `side`'s `body` with maximal faces, except where a recorded skip
/// leaves a curved run unglued, and maximal edges (module docs,
/// *Maximal faces*): [`Body::merge_coplanar_faces`], which ends with the
/// join, its groups, skips and joins recorded in `naming`. A skipped
/// group's faces have their boundaries re-described against the
/// adjacency they keep, as the boolean's output stage re-describes its
/// skipped groups' boundaries.
///
/// # Errors
///
/// [`SplitFinishError::EdgeJoin`] where the merge's join refuses, and
/// [`SplitFinishError::Merge`] for every other refusal of the merge or
/// of the skipped faces' re-description. [`SplitFinishError::Corrupt`]
/// where the merge names a placeholder face: a side carved from a
/// finished operand has none.
pub(super) fn end_maximal<T: Decide + crate::props::AtRestPolicy>(
    body: &mut Body<T>,
    side: PlaneSide,
    naming: &mut SplitNaming,
    normal: Vec3<T>,
    tol: Tol,
) -> Result<(), SplitFinishError> {
    // Taken before the kills, which drop it (`SplitNaming::joined_lineage`).
    let split_from = crate::boolean::edge_join::split_records(body);
    let merged = body
        .merge_coplanar_faces(tol)
        .map_err(|error| match error {
            crate::MergeCoplanarError::Join { refusal } => {
                SplitFinishError::EdgeJoin { side, refusal }
            }
            error => SplitFinishError::Merge { side, error },
        })?;
    if !merged.placeholders.is_empty() {
        return Err(SplitFinishError::Corrupt);
    }
    let skipped_faces = merged.skipped.iter().flat_map(|s| s.faces.iter().copied());
    let boundary =
        crate::boolean::boundary_edges(body, skipped_faces).ok_or(SplitFinishError::Corrupt)?;
    if !boundary.is_empty() {
        let band = geom_core::Band::linear(tol).map_err(SplitFinishError::Band)?;
        crate::boolean::describe_edges(
            body,
            boundary.iter().map(|&(_, e)| e),
            &merged.skipped,
            band,
            tol,
        )
        .map_err(|refusal| SplitFinishError::Merge {
            side,
            error: crate::MergeCoplanarError::of_kept_boundary(&boundary, refusal),
        })?;
    }
    naming
        .merge_groups
        .extend(merged.groups.into_iter().map(|g| (side, g)));
    naming
        .merge_skipped
        .extend(merged.skipped.into_iter().map(|s| (side, s)));
    naming.record_joins(side, &split_from, merged.joins);
    record_islands(body, side, naming, normal, tol)
}

/// Records which ring of `side`'s finished plane pieces holds which
/// other piece ([`SplitNaming::islands`]), by the section nesting rule
/// over the rings and the pieces' outer loops. It decides only where the
/// side holds a ring and two pieces, the one case a ring's part of the
/// plane can border a piece other than its own.
///
/// # Errors
///
/// [`SplitFinishError::NestingContradiction`] where two rings each read
/// as enclosing the other around an island; [`SplitFinishError::Corrupt`]
/// where a loop does not walk.
fn record_islands<T: Decide>(
    body: &Body<T>,
    side: PlaneSide,
    naming: &mut SplitNaming,
    normal: Vec3<T>,
    tol: Tol,
) -> Result<(), SplitFinishError> {
    let pieces = naming.plane_pieces(side, body).faces;
    let mut rings = Vec::new();
    let mut outlines = Vec::with_capacity(pieces.len());
    for &f in &pieces {
        let face = body.get_face(f).ok_or(SplitFinishError::Corrupt)?;
        rings.extend(face.rings.iter().map(|&r| (r, r)));
        outlines.push((f, face.outer));
    }
    if rings.is_empty() || pieces.len() < 2 {
        return Ok(());
    }
    let band = geom_core::Band::linear(tol).map_err(SplitFinishError::Band)?;
    let side_normal = section_loops::section_normal(normal, side);
    let nesting = section_loops::nest(body, rings, outlines, side_normal, band).map_err(
        |fault| match fault {
            section_loops::NestFault::Torn => SplitFinishError::Corrupt,
            section_loops::NestFault::Contradiction(island) => {
                SplitFinishError::NestingContradiction { hole: island }
            }
        },
    )?;
    for (ring, held) in nesting.regions {
        naming
            .islands
            .extend(held.into_iter().map(|island| (ring, island)));
    }
    Ok(())
}

impl Default for SplitNaming {
    // Each lineage list names itself where it is made (`LineageRows`).
    fn default() -> Self {
        Self {
            sections: Default::default(),
            face_fragments: crate::lineage::FaceDivisions::new(
                crate::lineage::LineageRows::FaceDivisions,
            ),
            edge_pieces: crate::lineage::SplitEdgeDivisions::new(
                crate::lineage::LineageRows::EdgeDivisions(None),
            ),
            vertex_pairs: Default::default(),
            edge_joins: Default::default(),
            joined_lineage: Default::default(),
            merge_groups: Default::default(),
            merge_skipped: Default::default(),
            islands: Default::default(),
            vertex_fusions: crate::lineage::Fusions::new(crate::lineage::LineageRows::Fusions),
        }
    }
}

/// Typed failure of the finish step.
#[derive(Debug)]
pub enum SplitFinishError {
    /// A component consists only of section faces, so it bounds no
    /// volume (kernel bug, loudly: the join's area certificate refuses
    /// every zero-area section that is no slit trace before the finish
    /// runs). No degenerate
    /// body is ever emitted.
    DegenerateSide {
        /// The offending shell (in the discarded scratch body).
        shell: ShellKey,
        /// Which side it claimed.
        side: PlaneSide,
    },
    /// One shell carries both above- and below-section faces (kernel
    /// bug, loudly).
    TornComponent {
        /// The offending shell.
        shell: ShellKey,
    },
    /// A component has no section face and no off-plane vertex to
    /// classify by (kernel bug or fully-coplanar garbage).
    UnclassifiableComponent {
        /// The offending shell.
        shell: ShellKey,
    },
    /// A traversal failed (corrupt mid-surgery body).
    Corrupt,
    /// An underlying Euler/structural operation refused.
    Euler(EulerOpError),
    /// The run's tolerance could not produce a classification band
    /// (absurd ε) — the section-boundary description pass classifies.
    Band(geom_core::BandError),
    /// Describing a section-boundary edge escalated on the angle
    /// between its two faces — the dihedral at its witness or at a
    /// station of the must-carry rule: indeterminate geometry at the
    /// section boundary refuses typed, never guesses a description.
    DescribeEscalated {
        /// The section-boundary edge.
        edge: EdgeKey,
        /// The deciding reading's diagnostic.
        diag: geom_core::Indeterminate,
    },
    /// Whether a section corner at a pinch holds another side of its
    /// loop, read to fuse the pinch's cones on a side
    /// ([`super::section_cones`]), escalated. No input is known to reach
    /// it: two sides of different cones within the band of one line at
    /// the pinch refuse earlier, in the boolean that made the pinch
    /// (`bool_vertex_face_side`, `enters_material`) or in the reduction
    /// ([`super::SplitReduceError::SectionGermsEscalated`]), or read
    /// decided. Guarded by
    /// `split_through_a_union_pinch::no_tilt_in_the_band_reaches_the_cone_corner_escalation`,
    /// which goes red at a witness.
    ConeCornerEscalated {
        /// The corner's vertex.
        vertex: VertexKey,
        /// The deciding reading's diagnostic.
        source: geom_core::Indeterminate,
    },
    /// A corner of the cut where pieces of the solid touch could not be
    /// read to fuse the pinch's cones ([`super::section_cones`]): a side
    /// of it has no departure read (a NURBS edge), or its face no normal
    /// there.
    ConeCornerUnread {
        /// The corner's vertex.
        vertex: VertexKey,
    },
    /// The zero-length joint that fuses a pinch's cones on a side
    /// ([`crate::fuse::fuse`]) lost a half-edge it made (a kernel bug).
    ConeFusionDesync {
        /// What no longer resolved.
        what: &'static str,
    },
    /// A curved wall smooth against the section at an edge refused its
    /// material pairing ([`geom_brep::MATERIAL_PAIRING`]): in band, or
    /// decided zero over an arm too short to read a side.
    DescribeSideEscalated {
        /// The section-boundary edge.
        edge: EdgeKey,
        /// The pairing's diagnostic, with the margin it decided.
        diag: geom_core::Indeterminate,
    },
    /// A smooth section-boundary edge's second order escalated at a
    /// station of the must-carry rule: whether its two faces curve
    /// apart there or share their curvature is in band, so neither the
    /// intrinsic nor the conventional description is honest.
    DescribeBendEscalated {
        /// The section-boundary edge.
        edge: EdgeKey,
        /// The sagitta's diagnostic (`"tangent_second_order"`).
        diag: geom_core::Indeterminate,
    },
    /// A section-boundary edge's line chord, between vertices the split
    /// read on the plane or the operand placed on its faces, reads within
    /// the band of a surface it lies on, and not clearly on it, where the
    /// edge is described or restated in a section chart
    /// ([`crate::merge_faces::edge_on_faces_reading`]).
    EdgeOnFacesEscalated {
        /// The section-boundary edge.
        edge: EdgeKey,
        /// The residual's diagnostic.
        diag: geom_core::Indeterminate,
    },
    /// A section-boundary edge read smooth at its witness and a corner
    /// at a station of the must-carry rule
    /// ([`geom_brep::MustCarryRefusal::Refuted`],
    /// [`geom_brep::MustCarryRefusal::Mixed`]): the readings disagree,
    /// so the split refuses rather than choose a description.
    SmoothJoinRefuted {
        /// The section-boundary edge.
        edge: EdgeKey,
        /// The two neighbouring stations that disagree, where one reads
        /// smooth; `None` where every station reads a corner.
        mixed: Option<geom_brep::MixedStations>,
    },
    /// A section loop's winding about its chart normal has no sign, so
    /// the section face's material side cannot be read: in the band
    /// (`diag`), or (`None`) zero, or unread because the loop carries an
    /// edge the winding does not read. The split's carrier gate admits only line,
    /// circle and ellipse edges, so every section edge is one the
    /// winding reads.
    SectionWindingUndecided {
        /// The null face the section loop bounds.
        face: FaceKey,
        /// The winding's diagnostic, when it escalated.
        diag: Option<geom_core::Indeterminate>,
    },
    /// The plane is tangent to a curved face along the section's
    /// boundary with the two faces' materials OPPOSED: a wedge end the
    /// cut would leave, [`KnifeEdge`]. (A cut piece lies on one side of
    /// the plane, so the end it can reach is the cusp, wedge 0.) A π
    /// seam (materials aligned) cuts. The reduction refuses the same
    /// contact first wherever it reads the wall tangent at an ON vertex
    /// ([`super::SplitReduceError::KnifeEdge`]); this read, of the
    /// dihedral along the edge over the edge's own arm, is the one left
    /// where the two arms decide differently in the band.
    KnifeEdge(KnifeEdge),
    /// Two section loops each read as enclosing the other around a hole
    /// or an island: they were decided disjoint, and disjoint loops
    /// cannot (kernel bug, loudly).
    NestingContradiction {
        /// The hole's section face (in the discarded scratch body), or
        /// the island's plane piece (in its side's body).
        hole: FaceKey,
    },
    /// A slit trace's slit, read definitely, lies in no section region of
    /// its side: no outline holds a vertex of it, or the innermost one
    /// that does has it in a hole (`in_hole`), whether a ring of its own
    /// or a hole the nesting rule left on a face of its own inside it, or
    /// it lies on a loop. The slit lies on edges its side's material
    /// wraps round, which a section region of that side covers, so this
    /// is a kernel defect.
    SlitUnplaced {
        /// The slit's face (in the discarded scratch body).
        slit: FaceKey,
        /// Whether a hole held it, or its boundary met it, rather than no
        /// outline holding it.
        in_hole: bool,
    },
    /// Which section face holds a slit trace's slit cannot be read: a
    /// vertex of it read against a loop of its side's section escalated,
    /// or the walk cannot read that loop ([`point_in_loop`]). No input is
    /// known to reach it: a keel end within the band of a bore, the
    /// near case, escalates in the operand's own Boolean first.
    SlitHoming {
        /// The slit's face (in the discarded scratch body).
        slit: FaceKey,
        /// The walk's refusal.
        error: crate::splitting::PointInLoopError,
    },
    /// A finished side fails tier 2 ([`crate::validate_closed`]): a
    /// split never returns a body that is not a closed solid. Reached
    /// by a kernel defect: [`super::split`] reads its operand's tier 2
    /// at every scalar before it cuts (`gate_finished`). `side` is in the
    /// caller's orientation.
    ResultInvalid {
        /// The side whose body failed.
        side: PlaneSide,
        /// The validator's findings.
        errors: Vec<crate::validate::ValidationError>,
    },
    /// A side, sorted into its solids, fails the scalar's at-rest gate
    /// ([`crate::AtRestPolicy::gate_at_rest`]: [`crate::validate_geometric`]
    /// at a certifying scalar, nothing at a dual), the gate the Boolean's
    /// result passes, on findings that are the kernel's: at a certifying
    /// scalar a split never returns a side its validator refuses. Typed and
    /// worded as the Boolean's [`crate::BooleanError::ResultInvalid`], each
    /// finding with its own ending. `side` is in the caller's orientation.
    ResultNotAtRest {
        /// The side whose body failed.
        side: PlaneSide,
        /// The validator's findings.
        errors: Vec<crate::validate::ValidationError>,
    },
    /// A side fails the at-rest gate only on shells whose role it
    /// certified in band: the Boolean's [`crate::BooleanError::Escalated`]
    /// on [`crate::BooleanDecision::ShellRole`], at this door and in its
    /// lever ([`crate::BooleanDecision::render_at_split`]). `side` is in
    /// the caller's orientation. No split is known to reach it: the
    /// reduction's input readings refuse an in-band sliver first
    /// ([`super::SplitReduceError::SliverVertex`], the section's area
    /// certificate), so the typing's own row (`at_rest_rows`) is its
    /// guard.
    ResultEscalated {
        /// The side whose body failed.
        side: PlaneSide,
        /// [`crate::BooleanDecision::ShellRole`] on the binding shell.
        decision: crate::BooleanDecision,
        /// The binding shell's certified reading.
        diag: geom_core::Indeterminate,
    },
    /// The join a side ends with ([`crate::Body::join_edges`](crate::construction::Construct::join_edges)) refused
    /// on that side's body.
    EdgeJoin {
        /// The side whose join refused.
        side: PlaneSide,
        /// Why the join refused, typed and keyless.
        refusal: crate::boolean::JoinRefusal,
    },
    /// The coplanar merge a side ends with ([`end_maximal`]) refused on
    /// that side's body.
    Merge {
        /// The side whose merge refused.
        side: PlaneSide,
        /// Why the merge refused.
        error: crate::MergeCoplanarError,
    },
}

impl SplitFinishError {
    /// The refusal of `side`, whose body the at-rest gate refused on
    /// `errors`, typed as the Boolean types its result
    /// ([`crate::boolean::at_rest_refusal`]).
    pub(crate) fn not_at_rest(
        side: PlaneSide,
        errors: Vec<crate::validate::ValidationError>,
    ) -> Self {
        match crate::boolean::at_rest_refusal(errors) {
            crate::boolean::AtRestRefusal::InBand { decision, diag } => Self::ResultEscalated {
                side,
                decision,
                diag,
            },
            crate::boolean::AtRestRefusal::Kernel(errors) => Self::ResultNotAtRest { side, errors },
        }
    }
}

impl From<EulerOpError> for SplitFinishError {
    fn from(e: EulerOpError) -> Self {
        Self::Euler(e.from_driver())
    }
}

/// The finish's refusal for a section-boundary edge whose description
/// on a carrier of kind `carrier` the attachment gate refused, at the
/// describe pass or at a section re-chart: the edge's own reading on its
/// faces where [`crate::merge_faces::edge_on_faces_reading`] finds one,
/// and otherwise the Euler operation's.
fn of_attachment(
    edge: EdgeKey,
    carrier: geom::CurveKind,
    refusal: EulerOpError,
) -> SplitFinishError {
    match crate::merge_faces::edge_on_faces_reading(carrier, &refusal) {
        Some(diag) => SplitFinishError::EdgeOnFacesEscalated { edge, diag },
        None => refusal.into(),
    }
}

/// A section re-chart, each `restated` edge re-described on its own
/// stored carrier and interval ([`Body::rechart_restating`]), its
/// refusal for one of them read as [`of_attachment`] reads it on that
/// edge's carrier.
pub(crate) fn rechart_section<T: Decide + crate::props::AtRestPolicy>(
    body: &mut Body<T>,
    charts: Vec<Rechart<T>>,
    restated: &[(EdgeKey, geom_brep::EdgeDescriptionSpec<T>)],
    tol: Tol,
) -> Result<(), SplitFinishError> {
    body.rechart_restating(
        crate::policy_lane::KernelSite::SectionRechart,
        charts,
        restated,
        tol,
    )
    .map(drop)
    .map_err(|refusal| {
        let edge = match &refusal {
            EulerOpError::RechartFalsifies { edge, .. } => *edge,
            _ => return refusal.into(),
        };
        let carrier = restated
            .iter()
            .any(|(e, _)| *e == edge)
            .then(|| body.get_edge(edge))
            .flatten()
            .and_then(|e| body.get_curve_geom(e.curve))
            .and_then(|geom| geom.certified())
            .map(|curve| curve.carrier().kind());
        match carrier {
            Some(kind) => of_attachment(edge, kind, refusal),
            None => refusal.into(),
        }
    })
}

impl core::fmt::Display for SplitFinishError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::DegenerateSide { side, .. } => write!(
                f,
                "the piece on the {} side of the plane bounds no volume: it holds only \
                 section faces (kernel bug)",
                side.word()
            ),
            Self::TornComponent { shell } => write!(
                f,
                "component {shell:?} carries section faces of both sides (kernel bug)"
            ),
            Self::UnclassifiableComponent { shell } => write!(
                f,
                "component {shell:?} has no section face and no off-plane vertex to \
                 classify it by"
            ),
            Self::Corrupt => write!(f, "the finish traversal failed (corrupt body)"),
            Self::Euler(e) => write!(f, "an Euler operation refused: {e}"),
            Self::Band(e) => write!(f, "{e}"),
            Self::DescribeEscalated { diag, .. } => diag
                .undecided(
                    "the angle between two faces along the cut",
                    diag.lever_ending(super::SPLIT_COINCIDENCE_RECOURSE),
                )
                .fmt(f),
            Self::ConeCornerEscalated { source, .. } => source
                .undecided(
                    "whether a corner of the cut, where pieces of the solid touch, overlaps \
                     another",
                    source.lever_ending(super::SPLIT_COINCIDENCE_RECOURSE),
                )
                .fmt(f),
            Self::ConeCornerUnread { .. } => write!(
                f,
                "a corner of the cut, where pieces of the solid touch, has a side whose \
                 direction cannot be read (a NURBS edge), so whether it overlaps another \
                 cannot be decided. {}",
                geom_core::KERNEL_OR_FILE_DEFECT_ENDING
            ),
            Self::ConeFusionDesync { what } => write!(
                f,
                "the joint between two pieces of the solid at a corner of the cut lost its \
                 half-edge ({what}). {}",
                geom_core::KERNEL_DEFECT_ENDING
            ),
            Self::DescribeSideEscalated { diag, .. } => diag
                .undecided(
                    geom_brep::MATERIAL_PAIRING_CLAUSE,
                    geom_brep::MATERIAL_PAIRING.recourse(
                        geom_brep::recourse::RefusedArm::Undecided(diag),
                        geom_brep::recourse::Reading::Build,
                    ),
                )
                .fmt(f),
            Self::DescribeBendEscalated { diag, .. } => diag
                .undecided(
                    "whether two faces touching along the cut curve apart there or share their \
                     curvature",
                    diag.lever_ending(super::SPLIT_COINCIDENCE_RECOURSE),
                )
                .fmt(f),
            Self::EdgeOnFacesEscalated { diag, .. } => {
                crate::merge_faces::edge_on_plane_refusal(diag).fmt(f)
            }
            Self::SmoothJoinRefuted {
                mixed: Some(stations),
                ..
            } => write!(
                f,
                "along an edge the cut leaves between two faces, {stations}. Recourse: {}",
                super::SPLIT_COINCIDENCE_RECOURSE
            ),
            Self::SmoothJoinRefuted { mixed: None, .. } => write!(
                f,
                "two faces along the cut meet smoothly at the middle of their shared edge \
                 but at a corner at every station along it, so the split cannot say what \
                 that edge is. Recourse: {}",
                super::SPLIT_COINCIDENCE_RECOURSE
            ),
            Self::SectionWindingUndecided {
                diag: Some(diag), ..
            } => diag
                .undecided(
                    "which side of a cut face is material",
                    diag.lever_ending(super::SPLIT_COINCIDENCE_RECOURSE),
                )
                .fmt(f),
            Self::SectionWindingUndecided { diag: None, .. } => write!(
                f,
                "which side of a cut face is material cannot be read: its outline \
                 encloses no area, or has an edge whose winding the kernel does not \
                 read (a NURBS edge, or spiric edges pinched near a torus's inner \
                 equator). Recourse: {}",
                super::SPLIT_COINCIDENCE_RECOURSE
            ),
            Self::NestingContradiction { hole } => write!(
                f,
                "two cut faces each read as enclosing the other around hole {hole:?}. {}",
                geom_core::KERNEL_DEFECT_ENDING
            ),
            Self::SlitUnplaced {
                slit,
                in_hole: false,
            } => write!(
                f,
                "edges lying in the cut, wrapped round at every end by the material \
                 across them, leave a slit {slit:?} that no cut face encloses. {}",
                geom_core::KERNEL_DEFECT_ENDING
            ),
            Self::SlitUnplaced {
                slit,
                in_hole: true,
            } => write!(
                f,
                "edges lying in the cut, wrapped round at every end by the material \
                 across them, leave a slit {slit:?} inside or against a hole of the cut \
                 face around it. {}",
                geom_core::KERNEL_DEFECT_ENDING
            ),
            Self::SlitHoming { error, .. } => error.render_as(
                f,
                "placing a slit an edge in the cut leaves",
                "which cut face holds a slit an edge in the cut leaves",
                super::SPLIT_COINCIDENCE_RECOURSE,
            ),
            Self::KnifeEdge(k) => write!(f, "{k}"),
            Self::EdgeJoin { side, refusal } => {
                write!(
                    f,
                    "the piece on the {} side of the plane: {refusal}",
                    side.word()
                )
            }
            Self::Merge { side, error } => write!(
                f,
                "the piece on the {} side of the plane: {error}",
                side.word()
            ),
            Self::ResultNotAtRest { side, errors } => crate::validate::write_result_not_at_rest(
                f,
                format_args!("the piece on the {} side of the plane", side.word()),
                errors,
            ),
            Self::ResultEscalated {
                side,
                decision,
                diag,
            } => write!(
                f,
                "the piece on the {} side of the plane: {}",
                side.word(),
                decision.render_at_split(diag)
            ),
            Self::ResultInvalid { side, errors } => match errors.as_slice() {
                [first, ..] => write!(
                    f,
                    "the piece on the {} side of the plane is not a closed solid ({} \
                     finding(s)); the first: {first}",
                    side.word(),
                    errors.len(),
                ),
                [] => write!(
                    f,
                    "the piece on the {} side of the plane is not a closed solid. {}",
                    side.word(),
                    geom_core::KERNEL_OR_FILE_DEFECT_ENDING
                ),
            },
        }
    }
}

impl std::error::Error for SplitFinishError {}

/// The finish step (module docs). Consumes the joined scratch body
/// inside `red` and the completed sections; returns both sides.
///
/// # Errors
///
/// [`SplitFinishError`]; the scratch body is discarded on `Err` (the
/// operand was never touched).
pub(super) fn split_finish<T: Decide + crate::props::AtRestPolicy>(
    red: SplitReduction<T>,
    completed: &[CompletedSection],
    face_fragments: crate::lineage::FaceDivisions,
    tol: Tol,
) -> Result<RawSplit<T>, SplitFinishError> {
    // **The phase boundary, asserted.** The reduce and join phases
    // hold their scope on this same body through the one guardless
    // pair in `splitting` (`split_scratch`), and nothing about a
    // guardless pair is checked by the compiler — so a close deleted
    // there shows up here, as a body arriving still inside a scope.
    // It is a per-BODY depth and this body is the pipeline's own, so
    // the answer is 0 whatever door the pipeline itself is nested in.
    debug_assert_eq!(
        red.body.open_surgery_scopes(),
        0,
        "split_finish: the reduced body arrived with {} surgery scope(s) still open — an \
         earlier phase opened one and did not close it, and every operator run on this \
         body from here on skips D1's tier-1 postcondition",
        red.body.open_surgery_scopes(),
    );
    // The reassembly is this door's operator sequence: one scope, one
    // tier-1 sweep over the reassembled body before it is carved into
    // the two sides. The guard owns the borrow, so a refusal on the
    // way closes the scope by dropping it.
    let mut reassembled = red.body;
    let mut body = reassembled.begin_surgery();
    let solid = single_solid(&body)?;

    // No section polygons: the plane did not cut — the whole operand
    // is one side (an ON-touching contact mints no null faces).
    if completed.is_empty() {
        body.sweep_and_close();
        return whole_body_side(reassembled, &red.sides, &red.plane, tol);
    }
    let mut naming = SplitNaming {
        sections: Vec::with_capacity(completed.len() * 2),
        face_fragments,
        edge_pieces: red.edge_pieces,
        vertex_pairs: red
            .null_edges
            .iter()
            .map(|r| {
                let copy = r.attr.copy_at(r.at_vertex);
                copy.map(|c| (c, r.at_vertex))
                    .ok_or(SplitFinishError::Corrupt)
            })
            .collect::<Result<_, _>>()?,
        edge_joins: Vec::new(),
        joined_lineage: Default::default(),
        merge_groups: Vec::new(),
        merge_skipped: Vec::new(),
        islands: Vec::new(),
        vertex_fusions: crate::lineage::Fusions::new(crate::lineage::LineageRows::Fusions),
    };

    let band = geom_core::Band::linear(tol).map_err(SplitFinishError::Band)?;
    // ---- Promotion: each null face → two section faces. ----
    // Which loop is currently the ring is read from the face record
    // (the role keys), not from list position.
    let mut section_side: SecondaryMap<FaceKey, PlaneSide> = SecondaryMap::new();
    let mut slits: Vec<(FaceKey, PlaneSide)> = Vec::new();
    let mut traces: Vec<FaceKey> = Vec::new();
    for section in completed {
        let face = body
            .get_face(section.face)
            .ok_or(SplitFinishError::Corrupt)?;
        let outer = face.outer;
        let ring = if outer == section.above_loop {
            section.below_loop
        } else if outer == section.below_loop {
            section.above_loop
        } else {
            return Err(SplitFinishError::Corrupt);
        };
        // A slit trace's two loops part into two faces: the strut loop's,
        // a component of its own that neither side keeps, and the slit's,
        // nested below as a ring of the section face containing it.
        if let SectionKind::Slit { strut } = section.kind {
            let promoted = body.mfkrh(ring, FaceSurface::Inherit)?;
            let (trace, slit) = if ring == strut {
                (promoted.face, section.face)
            } else {
                (section.face, promoted.face)
            };
            let slit_side = if strut == section.above_loop {
                PlaneSide::Below
            } else {
                PlaneSide::Above
            };
            traces.push(trace);
            slits.push((slit, slit_side));
            continue;
        }
        let u_ref = below_chord_u_ref(&body, section, red.plane.normal)?;
        let normal_of =
            |side: PlaneSide| section_loops::section_normal(red.plane.normal.get(), side);
        let plane_for = |side: PlaneSide| Surface::Plane {
            origin: red.plane.origin,
            normal: normal_of(side),
            u_ref,
        };
        let ring_side = if ring == section.above_loop {
            PlaneSide::Above
        } else {
            PlaneSide::Below
        };
        let other_side = match ring_side {
            PlaneSide::Above => PlaneSide::Below,
            _ => PlaneSide::Above,
        };
        // Each section face's sense is its loop's winding about its
        // chart normal: tier 3's check 6 reading (`planar_loop_winding`),
        // taken before the re-chart and stated with it.
        let ring_sense = section_sense(&body, section.face, ring, normal_of(ring_side), band)?;
        let outer_sense = section_sense(&body, section.face, outer, normal_of(other_side), band)?;
        // Both faces of the null pair move onto their section planes in
        // one re-chart, with the edges the move strands restated
        // (`section_plane_restatements`); the boundary pass below gives
        // each its honest class. An edge between the two moving faces
        // would be listed from both and refuse typed
        // (`DuplicateRedescription`), or, naming neither's chart,
        // `RechartUndescribed`.
        let promoted = body.mfkrh(ring, FaceSurface::Inherit)?;
        let charts = vec![
            Rechart::new(plane_for(ring_side), promoted.face, ring_sense),
            Rechart::new(plane_for(other_side), section.face, outer_sense),
        ];
        let stranded = body.stranded_by(&charts)?;
        let mut restated = section_plane_restatements(&body, promoted.face, &stranded)?;
        restated.extend(section_plane_restatements(&body, section.face, &stranded)?);
        rechart_section(&mut body, charts, &restated, tol)?;
        section_side.insert(promoted.face, ring_side);
        section_side.insert(section.face, other_side);
        naming.sections.push((promoted.face, ring_side));
        naming.sections.push((section.face, other_side));
    }

    // ---- Nesting: a section that is a hole in another becomes its
    // ring, so a holed section is one face. ----
    nest_hole_sections(
        &mut body,
        &mut section_side,
        &mut naming,
        red.plane.normal.get(),
        band,
        tol,
    )?;
    for (slit, side) in slits {
        nest_slit(
            &mut body,
            slit,
            side,
            &naming,
            red.plane.normal.get(),
            band,
            tol,
        )?;
    }

    // ---- D6 (M3 PR 6a): honest descriptions on the section boundary,
    // AT MINT TIME — both parent surfaces are known here (the section
    // faces were just promoted; the other side of every boundary edge
    // is an operand face). Definitely-transverse edges get
    // `Intersection`; definitely-smooth ones (a flush ON-face
    // neighbor: the surfaces under-determine the locus) carry a
    // conventional description in an adjacent chart per D2 — kept
    // where the edge already has one, stated in the section chart
    // where it does not; escalations refuse typed. ----
    let section_faces: Vec<FaceKey> = section_side.keys().collect();
    for face in section_faces {
        describe_section_boundary(&mut body, face, band, tol)?;
    }
    // A section loop that meets a wall's wrap edge at one vertex can
    // leave that edge between two faces of the wall; it comes to rest
    // as an ordinary image there.
    body.rest_parted_wrap_edges(tol)?;

    // ---- Distribution: movefac every shell of the solid. ----
    let shells: Vec<ShellKey> = body
        .shells_of_solid(solid)
        .ok_or(SplitFinishError::Corrupt)?
        .to_vec();
    let mut all_shells = Vec::new();
    for shell in shells {
        all_shells.extend(body.movefac(shell)?);
    }

    // ---- Classification + the degenerate net. ----
    let mut above_shells = Vec::new();
    let mut below_shells = Vec::new();
    for shell in all_shells {
        let faces = &body
            .get_shell(shell)
            .ok_or(SplitFinishError::Corrupt)?
            .faces;
        if faces.iter().any(|f| traces.contains(f)) {
            // The tree's vertices hold no other edge: the other side wraps
            // every end of it, and the join cut the null edge there, so the
            // walk round it is a component alone. `carve` keeps it on
            // neither side; a shell holding more would drop material.
            assert!(
                faces.len() == 1,
                "split_finish: the walk round a slit trace's tree shares shell {shell:?} with \
                 {} other face(s)",
                faces.len() - 1
            );
            continue;
        }
        let (side, has_real_face) = classify_shell(&body, shell, &section_side, &red.sides)?;
        if !has_real_face {
            return Err(SplitFinishError::DegenerateSide { shell, side });
        }
        match side {
            PlaneSide::Above => above_shells.push(shell),
            _ => below_shells.push(shell),
        }
    }
    if above_shells.is_empty() || below_shells.is_empty() {
        // With ≥1 completed section both sides must hold material.
        return Err(SplitFinishError::Corrupt);
    }

    // ---- Carve the two independent result bodies. ----
    //
    // The scope closes here, over the reassembled body — the state
    // every operator above was checked against. `carve` itself is raw
    // arena deletion and asserted nothing before this unit either.
    body.sweep_and_close();
    let above = carve(&reassembled, solid, &above_shells)?;
    let below = carve(&reassembled, solid, &below_shells)?;
    Ok(RawSplit {
        above: RawPart::Body(above),
        below: RawPart::Body(below),
        naming,
        coincidences: red.coincidences,
    })
}

/// Each section face that is a hole — sense `false`: its loop winds
/// clockwise about its outward normal — becomes a ring of the section
/// face of its side that encloses it by the section nesting rule
/// ([`section_loops::nest`]), and the hole's face dies (`kfmrh`). A
/// section with holes is then one face whose rings are the holes'
/// sections, the encoding tier 3 reads, rather than a face over the
/// whole outline plus a coplanar face per hole that cancels it. The
/// hole moves onto that face's chart first, its boundary restated with
/// it, so the ring rides the chart of the face that keeps it.
///
/// A hole joins a face only where it is decided disjoint from every
/// hole that face took before it, in section order: tier 3 compares a
/// ring with its face's outer loop only, so a ring that met another
/// ring would pass it.
///
/// **A hole the rule leaves unplaced, or the ring guard turns away,
/// keeps its own face** — sound by cancellation for volumes,
/// point-in-solid and tier 3, though tier 3′ reads what lies inside it
/// on both faces
/// (`work/cleave/a-hole-in-an-elliptic-section-keeps-a-face-of-its-own-that-tier-3-prime-refuses.md`).
///
/// # Errors
///
/// [`SplitFinishError::NestingContradiction`]; [`SplitFinishError::Euler`]
/// and [`SplitFinishError::Corrupt`] from the surgery.
fn nest_hole_sections<T: Decide + crate::props::AtRestPolicy>(
    body: &mut Body<T>,
    section_side: &mut SecondaryMap<FaceKey, PlaneSide>,
    naming: &mut SplitNaming,
    normal: Vec3<T>,
    band: geom_core::Band,
    tol: Tol,
) -> Result<(), SplitFinishError> {
    let corrupt = || SplitFinishError::Corrupt;
    let mut parent_of: SecondaryMap<FaceKey, FaceKey> = SecondaryMap::new();
    for side in [PlaneSide::Above, PlaneSide::Below] {
        let mut outlines = Vec::new();
        let mut holes = Vec::new();
        for face in side_sections(naming, side) {
            let data = body.get_face(face).ok_or_else(corrupt)?;
            if data.sense {
                outlines.push((face, data.outer));
            } else {
                holes.push(((face, data.outer), data.outer));
            }
        }
        let side_normal = section_loops::section_normal(normal, side);
        let nesting =
            section_loops::nest(body, outlines, holes, side_normal, band).map_err(|fault| {
                match fault {
                    section_loops::NestFault::Torn => corrupt(),
                    section_loops::NestFault::Contradiction((hole, _)) => {
                        SplitFinishError::NestingContradiction { hole }
                    }
                }
            })?;
        for (parent, holes) in nesting.regions {
            let mut rings: Vec<crate::entity::LoopKey> = Vec::new();
            for (hole, hole_loop) in holes {
                let mut clear = true;
                for &r in &rings {
                    clear = clear
                        && section_loops::outlines_disjoint(body, r, hole_loop, side_normal, band)
                            .map_err(|_| corrupt())?;
                }
                if clear {
                    rings.push(hole_loop);
                    parent_of.insert(hole, parent);
                }
            }
        }
    }
    let nested: Vec<(FaceKey, FaceKey)> = naming
        .sections
        .iter()
        .filter_map(|&(f, _)| parent_of.get(f).map(|&p| (f, p)))
        .collect();
    for (hole, parent) in nested {
        ring_into(body, hole, parent, tol)?;
        section_side.remove(hole);
        naming.sections.retain(|&(f, _)| f != hole);
    }
    Ok(())
}

/// The section faces of `side`, in section order.
fn side_sections(naming: &SplitNaming, side: PlaneSide) -> impl Iterator<Item = FaceKey> + '_ {
    naming
        .sections
        .iter()
        .filter(move |&&(_, s)| s == side)
        .map(|&(f, _)| f)
}

/// Makes the one-loop section face `face` a ring of the section face
/// `parent` (`kfmrh`). It moves onto `parent`'s chart first, its
/// boundary restated with it, so the ring rides the chart of the face
/// that keeps it.
///
/// # Errors
///
/// [`SplitFinishError::Euler`] and [`SplitFinishError::Corrupt`] from
/// the surgery; the re-chart's ([`rechart_section`]).
fn ring_into<T: Decide + crate::props::AtRestPolicy>(
    body: &mut Body<T>,
    face: FaceKey,
    parent: FaceKey,
    tol: Tol,
) -> Result<(), SplitFinishError> {
    let chart = body
        .get_face(parent)
        .ok_or(SplitFinishError::Corrupt)?
        .surface;
    let charts = vec![Rechart::shared(chart, face, false)];
    let stranded = body.stranded_by(&charts)?;
    let restated = section_plane_restatements(body, face, &stranded)?;
    rechart_section(body, charts, &restated, tol)?;
    body.kfmrh(parent, face)?;
    Ok(())
}

/// Nests the slit trace's face `slit` (one loop, a zero-area slit of
/// `side`'s piece, [`super::join::SectionKind::Slit`]) as a ring of the
/// section face of `side` whose region holds it ([`ring_into`]). Its
/// winding is zero, so its place is read by containment alone: a vertex
/// of the slit is read against the loops of the side's section faces on
/// their own carriers ([`point_in_loop`]).
///
/// A face's region is its outline less its rings, and less every hole
/// the nesting rule left on a face of its own ([`nest_hole_sections`]:
/// `sense` false) inside that outline. So the holder is the innermost
/// outline holding the vertex, none of whose rings holds it, unless a
/// hole of its own holding the vertex lies inside it, which takes the
/// vertex out of its region. Section loops are disjoint, so one loop
/// lies inside another exactly when a vertex of it does. A slit lies on
/// edges its side's material wraps round, so it lies in some section
/// region, clear of every loop.
///
/// # Errors
///
/// - [`SplitFinishError::SlitHoming`] where a reading refuses: in band,
///   or on a loop the walk does not read;
/// - [`SplitFinishError::SlitUnplaced`] where every reading is definite
///   and the vertex lies in no region, or on a loop;
///   [`SplitFinishError::NestingContradiction`] where no one holding
///   outline lies inside the others;
/// - [`SplitFinishError::Euler`] and [`SplitFinishError::Corrupt`] from
///   the surgery.
fn nest_slit<T: Decide + crate::props::AtRestPolicy>(
    body: &mut Body<T>,
    slit: FaceKey,
    side: PlaneSide,
    naming: &SplitNaming,
    normal: Vec3<T>,
    band: geom_core::Band,
    tol: Tol,
) -> Result<(), SplitFinishError> {
    let corrupt = || SplitFinishError::Corrupt;
    let slit_loop = body.get_face(slit).ok_or_else(corrupt)?.outer;
    let at = |l| crate::chord_join::ring_representative(body, l).map_err(|_| corrupt());
    let q = at(slit_loop)?;
    let side_normal = section_loops::section_normal(normal, side);
    let read = |l, p| {
        point_in_loop(body, l, side_normal, p, band)
            .map_err(|error| SplitFinishError::SlitHoming { slit, error })
    };
    let unplaced = |in_hole| Err(SplitFinishError::SlitUnplaced { slit, in_hole });
    // The outlines whose own rings leave the vertex inside them, and the
    // holes on faces of their own that hold it.
    let (mut outlines, mut holes, mut in_hole) = (Vec::new(), Vec::new(), false);
    for face in side_sections(naming, side) {
        let data = body.get_face(face).ok_or_else(corrupt)?;
        match read(data.outer, q)? {
            LoopContainment::Out => continue,
            LoopContainment::OnBoundary => return unplaced(!data.sense),
            LoopContainment::In if !data.sense => {
                holes.push(data.outer);
                continue;
            }
            LoopContainment::In => {}
        }
        let mut held = true;
        for &ring in &data.rings {
            match read(ring, q)? {
                LoopContainment::Out => {}
                LoopContainment::In => (held, in_hole) = (false, true),
                LoopContainment::OnBoundary => return unplaced(true),
            }
        }
        if held {
            outlines.push((face, data.outer));
        }
    }
    // Whether loop `inner` lies inside loop `outer`.
    let inside = |inner, outer| -> Result<bool, SplitFinishError> {
        Ok(read(outer, at(inner)?)? == LoopContainment::In)
    };
    let mut innermost = Vec::new();
    for &(face, l) in &outlines {
        let mut all = true;
        for &(other, m) in &outlines {
            if other != face && !inside(l, m)? {
                all = false;
            }
        }
        if all {
            innermost.push((face, l));
        }
    }
    let (parent, outline) = match innermost[..] {
        [one] => one,
        [] if outlines.is_empty() => return unplaced(in_hole || !holes.is_empty()),
        _ => return Err(SplitFinishError::NestingContradiction { hole: slit }),
    };
    for &hole in &holes {
        if inside(hole, outline)? {
            return unplaced(true);
        }
    }
    ring_into(body, slit, parent, tol)
}

/// The re-descriptions a section face's re-chart takes: every edge of
/// `face` among those the re-chart strands (`stranded`, from
/// [`Body::stranded_by`]) whose description names the chart the face
/// wears now, stated as an image in that chart, or, for a conventional
/// pair, as the pair it is. The re-chart reads
/// that image as the chart the face moves onto, or, where the edge's
/// other face keeps the chart, as that chart itself
/// ([`Body::rechart_restating`], which restates the carrier and
/// interval). A declared authority travels verbatim.
pub(crate) fn section_plane_restatements<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    stranded: &[EdgeKey],
) -> Result<Vec<(EdgeKey, geom_brep::EdgeDescriptionSpec<T>)>, SplitFinishError> {
    let corrupt = || SplitFinishError::Corrupt;
    let face_data = body.get_face(face).ok_or_else(corrupt)?;
    let chart = face_data.surface;
    let loops: Vec<_> = core::iter::once(face_data.outer)
        .chain(face_data.rings.iter().copied())
        .collect();
    let mut out: Vec<(EdgeKey, geom_brep::EdgeDescriptionSpec<T>)> = Vec::new();
    for lk in loops {
        let LoopBoundary::Cycle { first } = body.get_loop(lk).ok_or_else(corrupt)?.boundary else {
            continue;
        };
        for he in body.loop_cycle(first).ok_or_else(corrupt)? {
            let edge = body.get_half_edge(he).ok_or_else(corrupt)?.edge;
            if !stranded.contains(&edge) || out.iter().any(|(e, _)| *e == edge) {
                continue;
            }
            let edge_data = body.get_edge(edge).ok_or_else(corrupt)?;
            let geom = body.get_curve_geom(edge_data.curve).ok_or_else(corrupt)?;
            let Some(curve) = geom.certified() else {
                continue;
            };
            if !Named::of(geom).keys().any(|k| k == chart) {
                continue;
            }
            // A pair names no chart to restate it in: listed as it
            // stands, its key for this face stands for the chart the
            // face moves onto.
            let description = if matches!(
                curve.description(),
                geom_brep::EdgeDescription::OnPair { .. }
            ) {
                curve.restated_description()
            } else {
                curve.restated_in_chart(chart).description
            };
            out.push((edge, description));
        }
    }
    Ok(out)
}

/// D6 (M3 PR 6a): describes every boundary edge of one just-promoted
/// section face as the transverse `Intersection` of its two faces'
/// surfaces (read through [`geom_brep::IntersectionDraft`]: witness ON
/// the edge, at its carrier's mid-parameter), through the certified
/// [`crate::Body::describe_intersection`] lane. A smooth neighbor stores
/// what the must-carry rule over the edge demands
/// ([`geom_brep::must_carry_over_edge`]): the intrinsic
/// `TangentIntersection` where the two surfaces determine the locus (a
/// curved wall at a π seam), else a conventional description (D2's
/// split — flush ON-faces, whose parallel planes under-determine it).
/// A description of the demanded kind that names the edge's current
/// pair is kept verbatim (a stated image travels exactly — deriving a
/// replacement would trade a statement for a guess); any other is
/// restated, the conventional one as an image in the section chart,
/// which every section-boundary edge lies in to within the band. Either
/// description's residual read in band, an edge at the zero band's
/// edge of the plane, refuses as the edge's own reading
/// ([`SplitFinishError::EdgeOnFacesEscalated`]).
/// That covers the citation this split itself made stale: on a
/// face-coplanar cut an operand edge lands on the section boundary
/// with its transverse partner reassigned to the OTHER product, so the
/// `Intersection` it honestly carried now names a surface that is not
/// adjacent (and not even present) on this side. A curved wall smooth
/// against the section is first judged by its material pairing:
/// opposed is a wedge end nothing declared and refuses
/// ([`SplitFinishError::KnifeEdge`]). The rule's refusals are this
/// op's: a station in band first-order
/// ([`SplitFinishError::DescribeEscalated`], as the witness's dihedral
/// escalates; [`SplitFinishError::DescribeSideEscalated`], as the
/// pairing refuses) or second-order
/// ([`SplitFinishError::DescribeBendEscalated`]), and a station that
/// reads the edge a corner ([`SplitFinishError::SmoothJoinRefuted`]).
///
/// The body is mid-operation, past the carve; each edge is read after
/// the writes to the edges before it. Its curve is a link
/// ([`crate::live::OPERATORS_KEEP_LINKS`]; a rewritten edge's old curve
/// goes only once orphaned, [`Body::remove_curve_if_orphaned`]): a torn
/// one panics, and is not an edge with no description.
pub(crate) fn describe_section_boundary<T: Decide + crate::props::AtRestPolicy>(
    body: &mut Body<T>,
    face: FaceKey,
    band: geom_core::Band,
    tol: Tol,
) -> Result<(), SplitFinishError> {
    let corrupt = || SplitFinishError::Corrupt;
    let face_data = body.get_face(face).ok_or_else(corrupt)?;
    let s_self = face_data.surface;
    let loops: Vec<_> = core::iter::once(face_data.outer)
        .chain(face_data.rings.iter().copied())
        .collect();
    for lk in loops {
        let LoopBoundary::Cycle { first } = body.get_loop(lk).ok_or_else(corrupt)?.boundary else {
            continue;
        };
        let hes = body.loop_cycle(first).ok_or_else(corrupt)?;
        for he in hes {
            let edge = body.get_half_edge(he).ok_or_else(corrupt)?.edge;
            let edge_data = body.get_edge(edge).cloned().ok_or_else(corrupt)?;
            let mate = if edge_data.he_plus == he {
                edge_data.he_minus
            } else {
                edge_data.he_plus
            };
            let other_face = body.face_of_half_edge(mate).ok_or_else(corrupt)?;
            let s_other = body.get_face(other_face).ok_or_else(corrupt)?.surface;
            let start = body
                .get_half_edge(edge_data.he_plus)
                .ok_or_else(corrupt)?
                .start;
            let end = body.half_edge_end(edge_data.he_plus).ok_or_else(corrupt)?;
            let p0 = *body
                .get_point(body.get_vertex(start).ok_or_else(corrupt)?.point)
                .ok_or_else(corrupt)?;
            let p1 = *body
                .get_point(body.get_vertex(end).ok_or_else(corrupt)?.point)
                .ok_or_else(corrupt)?;
            let (Some(surf_self), Some(surf_other)) =
                (body.get_surface(s_self), body.get_surface(s_other))
            else {
                return Err(corrupt());
            };
            let existing = body
                .edge_curve_linked(edge, &edge_data)
                .certified()
                .cloned();
            let draft = geom_brep::IntersectionDraft::of(existing.as_ref(), p0, p1);
            let (carrier, interval) = draft.span();
            let reach = crate::wedge_reach(body, [face, other_face], carrier, interval);
            let witness = draft.witness;
            match geom_brep::classify_dihedral(surf_self, surf_other, witness, reach, band) {
                Ok(geom_brep::DihedralClass::Transverse) => {
                    let carrier = draft.span().0.kind();
                    let drafted = crate::pcurves::seam_intersection(
                        body,
                        draft,
                        [(he, s_self, surf_self), (mate, s_other, surf_other)],
                        band,
                    );
                    body.describe_intersection(
                        crate::policy_lane::KernelSite::SectionIntersection,
                        edge,
                        drafted,
                        tol,
                    )
                    .map_err(|e| of_attachment(edge, carrier, e))?;
                }
                // A curved wall smooth against the section plane is
                // either a π seam or a wedge end, and only the material
                // pairing tells them apart — tier 3's own reading
                // (D1). The wedge end is a knife edge no input
                // declared: this op's refusal.
                Ok(geom_brep::DihedralClass::Smooth) => {
                    if !matches!(surf_other, Surface::Plane { .. }) {
                        let sense_of = |f| body.get_face(f).map(|d| d.sense).ok_or_else(corrupt);
                        let pairing = geom_brep::classify_material_pairing(
                            surf_self,
                            sense_of(face)?,
                            surf_other,
                            sense_of(other_face)?,
                            witness,
                            geom_brep::folded_lever_arm(surf_self, surf_other, witness, reach),
                            band,
                        )
                        .map_err(|diag| SplitFinishError::DescribeSideEscalated { edge, diag })?;
                        if pairing == geom_brep::MaterialPairing::Opposed {
                            return Err(SplitFinishError::KnifeEdge(KnifeEdge {
                                wall: other_face,
                                at: KnifeEdgeSite::Edge(edge),
                            }));
                        }
                    }
                    // Carrier and interval travel verbatim (restated,
                    // never rebuilt), as does a conventional image's
                    // declared authority (an intrinsic locus is derived).
                    let curve = existing.as_ref().unwrap_or_else(|| {
                        unreachable!(
                            "{edge:?} read smooth at its witness has a certified carrier: \
                             null scaffolding has coincident ends, so its poisoned \
                             chord fails the dihedral's arm gate before this arm"
                        )
                    });
                    let spec = curve.restated_spec();
                    // What the join stores is the must-carry rule's
                    // over the edge, and each refusal is this op's own.
                    let demanded = geom_brep::must_carry_description(
                        geom_brep::SmoothJoin {
                            surfaces: [(s_self, surf_self), (s_other, surf_other)],
                            carrier: &spec.carrier,
                            interval: (spec.param_start, spec.param_end),
                            reach,
                            witness,
                        },
                        band,
                        |_, diag| SplitFinishError::DescribeEscalated { edge, diag },
                        |_, diag| SplitFinishError::DescribeBendEscalated { edge, diag },
                        |mixed| SplitFinishError::SmoothJoinRefuted { edge, mixed },
                    )?;
                    // A description of the demanded kind that names
                    // the edge's current faces stays verbatim (a stated
                    // image travels exactly); anything else — a
                    // citation whose partner this split reassigned to
                    // the other product, a scaffold, or the other kind
                    // — is restated.
                    let pair = geom_brep::SurfacePair::new(s_self, s_other);
                    let rest = crate::rest_locus(
                        body,
                        [(s_self, Some(face)), (s_other, Some(other_face))],
                        &spec.carrier,
                        (spec.param_start, spec.param_end),
                        band,
                    );
                    let intrinsic =
                        matches!(demanded, geom_brep::MustCarryDescription::Intrinsic(_));
                    let coherent = existing.as_ref().is_some_and(|c| match *c.description() {
                        // A wrap edge's two halves bound one face (D1);
                        // this edge bounds two, so a wrap flag on it is
                        // restated.
                        geom_brep::EdgeDescription::Chart(ref ch) if ch.wrap => false,
                        // A conventional form is coherent only where it
                        // rests where the one home says it rests.
                        geom_brep::EdgeDescription::Chart(ref ch) => {
                            !intrinsic && rest == geom_brep::RestLocus::Chart(ch.surface)
                        }
                        geom_brep::EdgeDescription::OnPair { pair: p } => {
                            !intrinsic && rest == geom_brep::RestLocus::Pair(p)
                        }
                        geom_brep::EdgeDescription::TangentIntersection { pair: p, .. } => {
                            intrinsic && p == pair
                        }
                        geom_brep::EdgeDescription::Intersection { .. }
                        | geom_brep::EdgeDescription::Scaffold(_) => false,
                    });
                    if !coherent {
                        // Every section-boundary edge lies on the
                        // section to within the BAND: on a near-flush
                        // operand it is metered like any description
                        // (D4 ¶3).
                        let mut description = demanded.spec(rest);
                        if !intrinsic
                            && let geom_brep::EdgeAuthority::Declared(mc) = curve.authority()
                        {
                            description = description.declared_by(mc);
                        }
                        let carrier = spec.carrier.kind();
                        body.restate_edge(
                            crate::policy_lane::KernelSite::SectionSmooth,
                            edge,
                            description,
                            tol,
                        )
                        .map_err(|e| of_attachment(edge, carrier, e))?;
                    }
                }
                Err(escalation) => {
                    return Err(SplitFinishError::DescribeEscalated {
                        edge,
                        diag: escalation.diag(),
                    });
                }
            }
        }
    }
    Ok(())
}

/// The operand's single solid. The pipelines read every operand as one
/// solid (`split` and the boolean hand them over through
/// [`Body::merge_all_solids`]), so any other count is a desync.
pub(crate) fn single_solid<T: Decide>(body: &Body<T>) -> Result<SolidKey, SplitFinishError> {
    let mut it = body.solids();
    match (it.next(), it.next()) {
        (Some((k, _)), None) => Ok(k),
        _ => Err(SplitFinishError::Corrupt),
    }
}

/// The un-cut case: the whole body lands on one side, decided by the
/// first non-ON cached vertex verdict (arena order — deterministic).
/// A body whose every vertex is ON can still have material off the
/// plane, along its curved edges — a one-segment cylinder touching the
/// plane along its seam strut has its two vertices there — so then the
/// first curved edge whose mid-parameter point is definitely off the
/// plane decides (`split_edge_side`, the vertex verdict's margin at
/// that point; edge arena order).
fn whole_body_side<T: Decide>(
    body: Body<T>,
    sides: &SecondaryMap<VertexKey, PlaneSide>,
    plane: &super::SplitPlane<T>,
    tol: Tol,
) -> Result<RawSplit<T>, SplitFinishError> {
    let mut side = None;
    for (v, _) in body.vertices() {
        match sides.get(v) {
            Some(PlaneSide::Above) => {
                side = Some(PlaneSide::Above);
                break;
            }
            Some(PlaneSide::Below) => {
                side = Some(PlaneSide::Below);
                break;
            }
            _ => {}
        }
    }
    if side.is_none() {
        let band = geom_core::Band::linear(tol).map_err(SplitFinishError::Band)?;
        for (_, edge) in body.edges() {
            let Some(curve) = body
                .get_curve_geom(edge.curve)
                .and_then(crate::null::CurveGeom::certified)
            else {
                continue;
            };
            if matches!(curve.carrier(), geom::Curve3::Line { .. }) {
                continue;
            }
            let (t0, t1) = curve.params();
            let mid = curve.carrier().eval(t0 + (t1 - t0) * T::from_f64(0.5));
            let offset = crate::sector_shape::plane_offset(plane.origin, plane.normal.get(), mid);
            match crate::validate::decide("split_edge_side", geom_core::Margin::of(offset), band) {
                Ok(geom_core::Sign::Positive) => side = Some(PlaneSide::Above),
                Ok(geom_core::Sign::Negative) => side = Some(PlaneSide::Below),
                Ok(geom_core::Sign::Zero) | Err(_) => continue,
            }
            break;
        }
    }
    match side {
        Some(PlaneSide::Above) => Ok(RawSplit {
            above: RawPart::Body(body),
            below: RawPart::Empty,
            naming: SplitNaming::default(),
            coincidences: Vec::new(),
        }),
        Some(_) => Ok(RawSplit {
            above: RawPart::Empty,
            below: RawPart::Body(body),
            naming: SplitNaming::default(),
            coincidences: Vec::new(),
        }),
        // Every vertex and every curved edge ON: a zero-volume
        // operand, which no closed solid is; the operand is never
        // validated, so this refuses here.
        None => Err(SplitFinishError::Corrupt),
    }
}

/// The section's in-plane u axis ([`section_loops::chord_u_ref`] of
/// the below loop, on the split's plane). Two adjacent corners of a
/// loop are distinct certified endpoints, so a chord is nonzero.
fn below_chord_u_ref<T: Decide>(
    body: &Body<T>,
    section: &CompletedSection,
    normal: UnitVec3<T>,
) -> Result<Vec3<T>, SplitFinishError> {
    let points = loop_points_of(body, section.below_loop).map_err(|e| match e {
        SplitJoinError::Euler(err) => SplitFinishError::Euler(err),
        _ => SplitFinishError::Corrupt,
    })?;
    Ok(section_loops::chord_u_ref(
        body,
        section.below_loop,
        &points,
        normal,
    ))
}

/// The sense of the section face `face`'s loop `l` will bound, on a
/// chart with outward `normal` ([`section_loops::loop_sense`]).
///
/// # Errors
///
/// [`SplitFinishError::SectionWindingUndecided`];
/// [`SplitFinishError::Corrupt`] on a torn loop.
fn section_sense<T: crate::props::AtRestPolicy>(
    body: &Body<T>,
    face: FaceKey,
    l: crate::entity::LoopKey,
    normal: Vec3<T>,
    band: geom_core::Band,
) -> Result<bool, SplitFinishError> {
    section_loops::loop_sense(body, l, normal, band).map_err(|fault| match fault {
        section_loops::SenseFault::Undecided(diag) => {
            SplitFinishError::SectionWindingUndecided { face, diag }
        }
    })
}

/// Classifies one component shell (module docs): section-face seeds,
/// vertex-side fallback; returns (side, has-any-non-section-face).
fn classify_shell<T: Decide>(
    body: &Body<T>,
    shell: ShellKey,
    section_side: &SecondaryMap<FaceKey, PlaneSide>,
    sides: &SecondaryMap<VertexKey, PlaneSide>,
) -> Result<(PlaneSide, bool), SplitFinishError> {
    let shell_data = body.get_shell(shell).ok_or(SplitFinishError::Corrupt)?;
    let mut side: Option<PlaneSide> = None;
    let mut has_real_face = false;
    for &face in &shell_data.faces {
        match section_side.get(face) {
            Some(&s) => match side {
                None => side = Some(s),
                Some(prev) if prev != s => {
                    return Err(SplitFinishError::TornComponent { shell });
                }
                Some(_) => {}
            },
            None => has_real_face = true,
        }
    }
    if let Some(s) = side {
        return Ok((s, has_real_face));
    }
    // No section face: an uncut component — classify by its first
    // off-plane vertex (deterministic face-list/cycle order walk).
    for &face in &shell_data.faces {
        let face_data = body.get_face(face).ok_or(SplitFinishError::Corrupt)?;
        for l in core::iter::once(face_data.outer).chain(face_data.rings.iter().copied()) {
            let loop_data = body.get_loop(l).ok_or(SplitFinishError::Corrupt)?;
            let LoopBoundary::Cycle { first } = loop_data.boundary else {
                continue;
            };
            for he in body.loop_cycle(first).ok_or(SplitFinishError::Corrupt)? {
                let v = body
                    .get_half_edge(he)
                    .ok_or(SplitFinishError::Corrupt)?
                    .start;
                if let Some(&s @ (PlaneSide::Above | PlaneSide::Below)) = sides.get(v) {
                    return Ok((s, has_real_face));
                }
            }
        }
    }
    Err(SplitFinishError::UnclassifiableComponent { shell })
}

/// Carves the sub-body spanned by `keep` shells out of `src`: a clone
/// with every other shell's entities removed and orphaned geometry
/// swept. Kept entities keep their keys (lineage-scoped identity —
/// deterministic, replay-stable).
///
/// **Every surviving link resolves.** The removal is arena surgery, not
/// an Euler operator, so it keeps the links by removing only records
/// no kept record names: no kept half-edge starts at a dropped vertex
/// or shares an edge with a dropped half-edge, and no kept lone-vertex
/// loop holds a dropped vertex. That is tier-1 pass 6's "no split
/// orbits" on a body at rest; `src` may be mid-operation, so the carve
/// checks it and answers [`SplitFinishError::Corrupt`] where it fails.
pub(crate) fn carve<T: Decide>(
    src: &Body<T>,
    solid: SolidKey,
    keep: &[ShellKey],
) -> Result<Body<T>, SplitFinishError> {
    let mut body = src.clone();
    let corrupt = || SplitFinishError::Corrupt;

    let all: Vec<ShellKey> = body.shells_of_solid(solid).ok_or_else(corrupt)?.to_vec();
    let drop: Vec<ShellKey> = all.iter().copied().filter(|s| !keep.contains(s)).collect();

    // Collect the dropped entity sets (deterministic list walks).
    let mut faces = Vec::new();
    let mut loops = Vec::new();
    let mut hes = Vec::new();
    let mut edges: Vec<crate::entity::EdgeKey> = Vec::new();
    let mut vertices: SecondaryMap<VertexKey, ()> = SecondaryMap::new();
    for &shell in &drop {
        let shell_data = body.get_shell(shell).ok_or_else(corrupt)?;
        for &face in &shell_data.faces {
            faces.push(face);
            let face_data = body.get_face(face).ok_or_else(corrupt)?;
            for l in core::iter::once(face_data.outer).chain(face_data.rings.iter().copied()) {
                loops.push(l);
                match body.get_loop(l).ok_or_else(corrupt)?.boundary {
                    LoopBoundary::Empty { vertex } => {
                        vertices.insert(vertex, ());
                    }
                    LoopBoundary::Cycle { first } => {
                        for he in body.loop_cycle(first).ok_or_else(corrupt)? {
                            hes.push(he);
                            let he_data = body.get_half_edge(he).ok_or_else(corrupt)?;
                            vertices.insert(he_data.start, ());
                            // Each edge is claimed by its he_plus only
                            // (one removal per edge).
                            let edge = body.get_edge(he_data.edge).ok_or_else(corrupt)?;
                            if edge.he_plus == he {
                                edges.push(he_data.edge);
                            }
                        }
                    }
                }
            }
        }
    }

    let dropped_hes: SecondaryMap<crate::entity::HalfEdgeKey, ()> =
        hes.iter().map(|&he| (he, ())).collect();
    let dropped_loops: SecondaryMap<crate::entity::LoopKey, ()> =
        loops.iter().map(|&l| (l, ())).collect();
    for (he, he_data) in &body.half_edges {
        if dropped_hes.contains_key(he) {
            continue;
        }
        let edge = body.get_edge(he_data.edge).ok_or_else(corrupt)?;
        if vertices.contains_key(he_data.start)
            || dropped_hes.contains_key(edge.he_plus)
            || dropped_hes.contains_key(edge.he_minus)
        {
            return Err(corrupt());
        }
    }
    for (l, loop_data) in &body.loops {
        if let LoopBoundary::Empty { vertex } = loop_data.boundary
            && !dropped_loops.contains_key(l)
            && vertices.contains_key(vertex)
        {
            return Err(corrupt());
        }
    }

    // ---- Removal (crate-internal arena surgery; deterministic). ----
    let Some(solid_data) = body.solids.get_mut(solid) else {
        unreachable!("carve: `solid` resolved above and nothing has been removed yet")
    };
    solid_data.shells.retain(|s| keep.contains(s));
    for &shell in &drop {
        body.shells.remove(shell);
        body.shell_provenance.remove(shell);
    }
    for &face in &faces {
        body.faces.remove(face);
        body.face_provenance.remove(face);
        body.null_faces.remove(face);
    }
    for &l in &loops {
        body.loops.remove(l);
        body.loop_provenance.remove(l);
    }
    for &he in &hes {
        body.half_edges.remove(he);
        body.half_edge_provenance.remove(he);
    }
    for &edge in &edges {
        body.edges.remove(edge);
        body.edge_provenance.remove(edge);
    }
    let vertex_keys: Vec<VertexKey> = vertices.keys().collect();
    for &v in &vertex_keys {
        body.vertices.remove(v);
        body.vertex_provenance.remove(v);
    }

    // ---- Orphan geometry sweep (tier-1 pass 8 hygiene): keep only
    // points/curves/surfaces still referenced by survivors. ----
    let mut live_points: SecondaryMap<crate::geometry::PointKey, ()> = SecondaryMap::new();
    for (_, v) in body.vertices() {
        live_points.insert(v.point, ());
    }
    let orphan_points: Vec<_> = body
        .points
        .keys()
        .filter(|k| !live_points.contains_key(*k))
        .collect();
    for k in orphan_points {
        body.points.remove(k);
    }
    let mut live_curves: SecondaryMap<crate::geometry::CurveKey, ()> = SecondaryMap::new();
    for (_, e) in body.edges() {
        live_curves.insert(e.curve, ());
    }
    let orphan_curves: Vec<_> = body
        .curves
        .keys()
        .filter(|k| !live_curves.contains_key(*k))
        .collect();
    for k in orphan_curves {
        body.curves.remove(k);
    }
    let mut live_surfaces: SecondaryMap<crate::geometry::SurfaceKey, ()> = SecondaryMap::new();
    for (_, face) in body.faces() {
        live_surfaces.insert(face.surface, ());
    }
    // Description references keep surfaces alive exactly like faces do
    // (the `remove_surface_if_orphaned` rule): an `Intersection`/`Seam`
    // description on a surviving edge must never dangle (extrude-built
    // operands carry them — M3 PR 5).
    for (_, curve) in body.curves() {
        for s in Named::of(curve).keys() {
            live_surfaces.insert(s, ());
        }
    }
    let orphan_surfaces: Vec<_> = body
        .surfaces
        .keys()
        .filter(|k| !live_surfaces.contains_key(*k))
        .collect();
    for k in orphan_surfaces {
        body.surfaces.remove(k);
    }
    Ok(body)
}

/// **A torn section curve panics before the description writes**: on a
/// split cube's lower half, a torn curve on the section face's first
/// boundary edge panics naming the link, with the body unchanged. Two
/// reads name that link: the description's own, and
/// the plan of the door that writes it ([`crate::Body::restate_edge`]
/// or [`crate::Body::set_edge_curve`](crate::construction::Construct::set_edge_curve)), which every arm that describes
/// the edge reaches before its write. The row holds whichever
/// panics first, so it cannot tell the two apart.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod torn_hop_rows {
    use geom_core::{Band, Point3, Tol, Vec3};

    use crate::entity::{EntityId, GeomRef, LoopBoundary};
    use crate::live::OPERATORS_KEEP_LINKS;
    use crate::review_d18::{ROW_FOUR, assert_torn_op_panics};

    #[test]
    fn a_torn_section_curve_panics_before_any_write() {
        let tol = Tol::witness();
        let band = Band::linear(tol).unwrap();
        let cube = crate::test_support_fixtures::geometric_cube::<f64>(tol).body;
        let plane = crate::test_support_fixtures::split_plane(
            Point3::new(0.0, 0.0, 0.5),
            Vec3::unit_z(),
            tol,
        );
        let mut cube = cube;
        crate::test_support_fixtures::describe_as_intersections(&mut cube, tol);
        let cube = crate::test_support::finished("the cube", cube, tol);
        let split = crate::splitting::split(&cube, &plane, tol).unwrap();
        let mut body = crate::Body::clone(split.below.body().unwrap());
        let face = body
            .faces()
            .find(|(_, f)| {
                matches!(
                    body.get_surface(f.surface),
                    Some(geom::Surface::Plane { origin, .. }) if (origin.z - 0.5).abs() < 1e-12
                )
            })
            .map(|(k, _)| k)
            .unwrap();
        assert!(
            super::describe_section_boundary(&mut body.clone(), face, band, tol).is_ok(),
            "the sound section face's boundary is described"
        );
        let outer = body.get_face(face).unwrap().outer;
        let LoopBoundary::Cycle { first } = body.get_loop(outer).unwrap().boundary else {
            panic!("the section face's outer loop is a cycle");
        };
        let edge = body.get_half_edge(first).unwrap().edge;
        let curve = body.get_edge(edge).unwrap().curve;
        body.curves.remove(curve);
        let named = format!(
            "{}'s curve names {}",
            EntityId::Edge(edge),
            GeomRef::Curve(curve)
        );
        assert_torn_op_panics(
            "describe_section_boundary",
            &mut body,
            &[&named, ROW_FOUR, OPERATORS_KEEP_LINKS],
            |b| super::describe_section_boundary(b, face, band, tol),
        );
    }
}

/// `describe_section_boundary`'s smooth arm, on a section edge whose
/// neighbour's surface is swapped in place for one smooth against the
/// section plane at the edge's midpoint: the verdict of the must-carry
/// rule over the edge decides what the edge stores, or how it refuses.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod smooth_arm_rows {
    use geom_core::{Band, Point3, Tol, Vec3};

    use super::SplitFinishError;
    use crate::body::Body;
    use crate::entity::{EdgeKey, FaceKey, LoopBoundary};
    use geom::Surface;
    use geom_brep::SurfaceKey;

    struct SectionEdge {
        body: Body<f64>,
        face: FaceKey,
        edge: EdgeKey,
        s_self: SurfaceKey,
        s_other: SurfaceKey,
        mid: Point3<f64>,
        along: Vec3<f64>,
    }

    fn tol() -> Tol {
        Tol::witness()
    }

    /// The lower half of the unit cube split at `z = 0.5`, its section
    /// face, and the first edge of that face's outer loop. The section
    /// plane's normal is `+z`, and the neighbour's sense is set so its
    /// outward normal at the midpoint agrees with the section face's
    /// once its surface curves to `+z` there (a π seam, not a knife
    /// edge).
    fn section_edge() -> SectionEdge {
        let cube = crate::test_support_fixtures::geometric_cube::<f64>(tol()).body;
        let plane = crate::test_support_fixtures::split_plane(
            Point3::new(0.0, 0.0, 0.5),
            Vec3::unit_z(),
            tol(),
        );
        let mut cube = cube;
        crate::test_support_fixtures::describe_as_intersections(&mut cube, tol());
        let cube = crate::test_support::finished("the cube", cube, tol());
        let split = crate::splitting::split(&cube, &plane, tol()).unwrap();
        let mut body = crate::Body::clone(split.below.body().unwrap());
        let face = body
            .faces()
            .find(|(_, f)| {
                matches!(
                    body.get_surface(f.surface),
                    Some(Surface::Plane { origin, normal, .. })
                        if (origin.z - 0.5).abs() < 1e-12 && normal.z.abs() == 1.0
                )
            })
            .map(|(k, _)| k)
            .unwrap();
        let section = body.get_face(face).unwrap();
        let (s_self, sense_self) = (section.surface, section.sense);
        let LoopBoundary::Cycle { first } = body.get_loop(section.outer).unwrap().boundary else {
            panic!("the section face's outer loop is a cycle");
        };
        let edge = body.get_half_edge(first).unwrap().edge;
        let e = body.get_edge(edge).unwrap().clone();
        let mate = if e.he_plus == first {
            e.he_minus
        } else {
            e.he_plus
        };
        let other = body.face_of_half_edge(mate).unwrap();
        let s_other = body.get_face(other).unwrap().surface;
        let Some(Surface::Plane { normal, .. }) = body.get_surface(s_self).cloned() else {
            panic!("the section face is planar");
        };
        let outward_up = (normal.z > 0.0) == sense_self;
        body.get_face_mut(other).unwrap().sense = outward_up;
        let curve = body.get_curve_geom(e.curve).unwrap().certified().unwrap();
        let (t0, t1) = curve.params();
        let (p0, p1) = (curve.carrier().eval(t0), curve.carrier().eval(t1));
        let (mid, along) = (curve.mid_point(), (p1 - p0) / p0.distance(p1));
        SectionEdge {
            body,
            face,
            edge,
            s_self,
            s_other,
            mid,
            along,
        }
    }

    /// The unit cylinder under the section plane with axis `axis`
    /// through `mid − z`, so it touches the plane at `mid` and curves
    /// to `+z` there.
    fn cylinder_under(mid: Point3<f64>, axis: Vec3<f64>) -> Surface<f64> {
        Surface::Cylinder {
            origin: mid - Vec3::unit_z(),
            axis,
            radius: 1.0,
            u_ref: Vec3::unit_z(),
        }
    }

    /// **A corner at a station refutes the smooth premise, typed.** The
    /// neighbour's cylinder runs ACROSS the edge, tangent to the
    /// section plane at the midpoint only: the witness reads smooth,
    /// the stations either side read a corner, and the split refuses
    /// the mixed edge, naming the first two stations that disagree.
    #[test]
    fn a_corner_at_a_station_refuses_as_a_refuted_smooth_join() {
        let SectionEdge {
            mut body,
            face,
            edge,
            s_other,
            mid,
            along,
            ..
        } = section_edge();
        body.surfaces[s_other] = cylinder_under(mid, Vec3::unit_z().cross(along));
        let band = Band::linear(tol()).unwrap();
        match super::describe_section_boundary(&mut body, face, band, tol()) {
            Err(SplitFinishError::SmoothJoinRefuted {
                edge: refused,
                mixed: Some(stations),
            }) => {
                assert_eq!(refused, edge, "the refusal names the mixed edge");
                assert_eq!(
                    stations,
                    geom_brep::MixedStations {
                        transverse: 3,
                        smooth: 4
                    },
                    "the stations bracketing the tangency at the midpoint"
                );
            }
            other => panic!("a smooth-at-the-witness corner refuses typed, got {other:?}"),
        }
    }

    /// **A refused material pairing ends as the pairing, offering a
    /// tolerance that decides it.** The neighbour is a unit cylinder
    /// along the edge, tilted about it so its normal leans `tilt` off the
    /// section's: arm 1, wedge `w = sin tilt`, pairing margin
    /// `m = cos tilt`. Each row reads smooth with its pairing refused.
    /// At `K = 1.2` the 45° and 42° leans decide the pairing zero and the
    /// 30° lean leaves it in band; at `K = 10` a lean of `w = 0.0998`
    /// leaves it in band. Where the wedge reads zero at `m/K` (30°) that
    /// is the offer; elsewhere a tolerance just below `m/K` would leave
    /// the wedge in band, so the offer is `w/K`, which decides both.
    #[test]
    fn a_refused_pairing_ends_as_the_pairing_decision() {
        let deg = |d: f64| d.to_radians().sin();
        let rows = [
            ("45°, decided zero", (0.75, 0.9), deg(45.0), deg(45.0) / 1.2),
            ("42°, decided zero", (0.75, 0.9), deg(42.0), deg(42.0) / 1.2),
            (
                "30°, in band",
                (0.75, 0.9),
                deg(30.0),
                30.0_f64.to_radians().cos() / 1.2,
            ),
            ("K = 10, in band", (0.0999, 0.999), 0.0998, 0.0998 / 10.0),
        ];
        for (label, (zero, escalate), sin, offered) in rows {
            let band = Band::new(zero, escalate).unwrap();
            let SectionEdge {
                mut body,
                face,
                edge,
                s_other,
                mid,
                along,
                ..
            } = section_edge();
            let cos = (1.0 - sin * sin).sqrt();
            let lean = Vec3::unit_z() * cos + Vec3::unit_z().cross(along) * sin;
            body.surfaces[s_other] = Surface::Cylinder {
                origin: mid - lean,
                axis: along,
                radius: 1.0,
                u_ref: lean,
            };
            let refusal = super::describe_section_boundary(&mut body, face, band, tol())
                .expect_err("a refused pairing refuses the split");
            let text = refusal.to_string();
            assert!(
                text.starts_with(concat!(
                    geom_brep::material_pairing_clause!(),
                    " is undecided: "
                )),
                "{label}: the refusal names the pairing, got {text}"
            );
            let (_, ending) = text.split_once(". Recourse: ").expect("one recourse");
            let quoted: f64 = ending
                .strip_prefix(
                    "move the geometry so that edge's faces clearly run farther and none curves \
                     tightly there, or, if this length is intended, tighten the \
                     tolerance below ",
                )
                .and_then(|v| v.strip_suffix(" m"))
                .unwrap_or_else(|| panic!("{label}: the pairing's lever and offer, got {text}"))
                .parse()
                .unwrap();
            assert!(
                (quoted - offered).abs() <= 1e-9 * offered,
                "{label}: offers {quoted:e}, wants {offered:e}"
            );
            match refusal {
                SplitFinishError::DescribeSideEscalated { edge: refused, .. } => {
                    assert_eq!(refused, edge, "{label}: the refusal names the edge");
                }
                other => panic!("{label}: expected the pairing's refusal, got {other:?}"),
            }
            // The offer is true: just below it, the same pose describes.
            let (k, below) = (escalate / zero, 0.99 * quoted);
            let mut body = section_edge().body;
            body.surfaces[s_other] = Surface::Cylinder {
                origin: mid - lean,
                axis: along,
                radius: 1.0,
                u_ref: lean,
            };
            let tighter = Band::new(below, k * below).unwrap();
            if let Err(other) = super::describe_section_boundary(&mut body, face, tighter, tol()) {
                panic!("{label}: refuses again below the offer: {other}");
            }
        }
    }

    /// **A tangency that no longer determines its locus is restated.**
    /// The edge first stores a certified `TangentIntersection` over its
    /// current pair (the neighbour a cylinder ALONG the edge, κ_rel =
    /// 1), then the neighbour's surface becomes the section plane
    /// itself: the citation still names the current pair, but the rule
    /// now answers under-determined, so the edge is restated as the
    /// conventional pair rather than kept.
    #[test]
    fn a_coherent_tangency_on_an_under_determined_edge_is_restated_conventionally() {
        let SectionEdge {
            mut body,
            face,
            edge,
            s_self,
            s_other,
            mid,
            along,
        } = section_edge();
        body.surfaces[s_other] = cylinder_under(mid, along);
        let curve = body.get_edge(edge).unwrap().curve;
        let mut spec = body
            .get_curve_geom(curve)
            .unwrap()
            .certified()
            .unwrap()
            .restated_spec();
        spec.description = geom_brep::EdgeDescriptionSpec::TangentIntersection {
            pair: geom_brep::SurfacePair::new(s_self, s_other),
            witness: mid,
        };
        body.set_edge_curve(edge, spec, tol())
            .expect("plane against a unit cylinder along the edge certifies tangent");
        let Some(Surface::Plane {
            origin,
            normal,
            u_ref,
        }) = body.get_surface(s_self).cloned()
        else {
            panic!("the section face is planar");
        };
        body.surfaces[s_other] = Surface::Plane {
            origin,
            normal,
            u_ref,
        };
        let band = Band::linear(tol()).unwrap();
        super::describe_section_boundary(&mut body, face, band, tol())
            .expect("a flush neighbour describes conventionally");
        let curve = body.get_edge(edge).unwrap().curve;
        let stored = body.get_curve_geom(curve).unwrap().certified().unwrap();
        // The neighbour is now a second record of the section's plane:
        // one carrier, so the edge rests in the first record's chart.
        match stored.description() {
            geom_brep::EdgeDescription::Chart(chart) => assert_eq!(
                chart.surface,
                geom_brep::SurfacePair::new(s_self, s_other).keys()[0],
                "restated in the first record's chart"
            ),
            other => panic!("an under-determined edge keeps no tangency, got {other:?}"),
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod escalation_endings {
    use geom_core::{Band, Decide, Decided, Indeterminate, MarginDiag, Sign};

    use super::SplitFinishError;
    use crate::entity::{EdgeKey, FaceKey};

    /// **A split's angle, bend and winding escalations end in the
    /// split's lever alone.** None is a size the user chose, so a margin
    /// read in the band offers no tolerance, a straddle no box to
    /// subdivide, and only a poisoned margin adds the note.
    #[test]
    fn split_escalations_end_in_the_lever_alone() {
        let band = Band::new(1e-9, 1e-8).unwrap();
        let read = 3e-9_f64.sign_within(band).unwrap_err();
        let straddle = geom_core::k_stats::straddle(
            "finish_straddle",
            Decided {
                sign: Sign::Zero,
                margin: MarginDiag::value(0.5e-9),
            },
            Decided {
                sign: Sign::Positive,
                margin: MarginDiag::value(3e-8),
            },
            band,
        );
        let poisoned = Indeterminate {
            margin: MarginDiag::INVALID,
            ..read
        };
        let tail = ". Recourse: move the split plane or the geometry";
        for diag in [read, straddle] {
            for refusal in [
                SplitFinishError::DescribeEscalated {
                    edge: EdgeKey::default(),
                    diag,
                },
                SplitFinishError::DescribeBendEscalated {
                    edge: EdgeKey::default(),
                    diag,
                },
                SplitFinishError::SectionWindingUndecided {
                    face: FaceKey::default(),
                    diag: Some(diag),
                },
            ] {
                let text = refusal.to_string();
                assert!(
                    text.contains(" is undecided: ") && text.ends_with(tail),
                    "the lever alone, got {text}"
                );
            }
        }
        let text = SplitFinishError::DescribeEscalated {
            edge: EdgeKey::default(),
            diag: poisoned,
        }
        .to_string();
        assert!(
            text.contains(&format!("{tail}; ")),
            "a poisoned margin: the lever and the note, got {text}"
        );
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod edge_on_faces_rows {
    use geom::CurveKind;
    use geom_brep::{CertCheck, CertifyError};
    use geom_core::{Band, Indeterminate, MarginDiag};

    use super::{SplitFinishError, of_attachment};
    use crate::entity::EdgeKey;
    use crate::euler::{EulerOpError, RechartDoor};
    use crate::geometry::SurfaceKey;

    /// **A section-boundary line chord's residual read in band is the
    /// input's, and nothing else is.** A line's residual against a
    /// surface it lies on, or its image's against the section chart, read
    /// in band where the edge is described or re-charted, refuses as the
    /// edge's own reading, ending in the lever alone. The same reading on a
    /// circle, an ellipse or a spline, a definite miss, the witness's
    /// residual, an endpoint pin and any other check stay the attachment
    /// gate's refusal.
    #[test]
    fn an_in_band_residual_on_a_line_is_the_edges_reading_and_nothing_else_is() {
        let band = Band::new(1e-9, 1e-8).unwrap();
        let edge = EdgeKey::default();
        let surface = SurfaceKey::default();
        let in_band = |predicate| Indeterminate {
            margin: MarginDiag::value(1e-9 * (1.0 + 5e-8)),
            band,
            predicate: Some(predicate),
            terminal_sliver: false,
        };
        let escalated = |check, predicate| CertifyError::Escalated {
            check,
            sample: 8,
            cause: in_band(predicate),
        };
        let at_both_doors = |error: CertifyError| {
            [
                EulerOpError::Certification { error },
                EulerOpError::RechartFalsifies {
                    door: RechartDoor::SetFaceSurfacesDescribing,
                    edge,
                    error,
                },
            ]
        };
        for (check, predicate) in [
            (
                CertCheck::SurfaceResidual { surface },
                "carrier_on_surface_1",
            ),
            (CertCheck::ChartResidual, "pcurve_map_residual"),
        ] {
            for refusal in at_both_doors(escalated(check, predicate)) {
                let got = of_attachment(edge, CurveKind::Line, refusal.clone());
                assert!(
                    matches!(
                        got,
                        SplitFinishError::EdgeOnFacesEscalated { diag, .. }
                            if diag == in_band(predicate)
                    ),
                    "{check:?} in band on a line, {refusal:?}: {got:?}"
                );
                assert_eq!(
                    got.to_string(),
                    format!(
                        "whether an edge along the cut lies on both faces it bounds is \
                         undecided: {}. Recourse: move the split plane or the geometry so each \
                         edge and vertex that meets the plane lies clearly on it or clearly off \
                         it, and an edge that ends on it leaves it at a clear angle",
                        in_band(predicate).payload()
                    ),
                    "{check:?}: the reading and the lever alone"
                );
                for kind in [CurveKind::Circle, CurveKind::Ellipse, CurveKind::Nurbs] {
                    let got = of_attachment(edge, kind, refusal.clone());
                    assert!(
                        matches!(got, SplitFinishError::Euler(ref e) if *e == refusal),
                        "{check:?} on a {kind:?} stays the gate's: {got:?}"
                    );
                }
            }
        }
        let kernels = [
            CertifyError::ResidualExceeded {
                check: CertCheck::SurfaceResidual { surface },
                sample: 8,
                margin: MarginDiag::value(3e-8),
            },
            CertifyError::ResidualExceeded {
                check: CertCheck::ChartResidual,
                sample: 8,
                margin: MarginDiag::value(3e-8),
            },
            escalated(
                CertCheck::WitnessSurfaceResidual { surface },
                "witness_on_surface_1",
            ),
            escalated(CertCheck::EndpointEnd, "carrier_endpoint_end"),
            escalated(CertCheck::Transversality, "dihedral_wedge"),
        ];
        for error in kernels {
            for refusal in at_both_doors(error) {
                let got = of_attachment(edge, CurveKind::Line, refusal.clone());
                assert!(
                    matches!(got, SplitFinishError::Euler(ref e) if *e == refusal),
                    "stays the gate's: {got:?}"
                );
            }
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod joined_lineage_rows {
    use std::collections::BTreeMap;

    use slotmap::SlotMap;

    use super::{PlaneSide, SplitNaming};
    use crate::boolean::EdgeJoin;
    use crate::entity::{EdgeKey, VertexKey};

    /// **A side's joins may kill a split child before its parent.**
    /// Edge `e1` split into `e13`, and `e13` split twice (`e14`, `e15`);
    /// the join kills `e1`, then `e14`, then `e13`, so the row for `e13`
    /// comes after a row that names it as a parent. Each killed edge
    /// that held a record is recorded with its record's parent, and only
    /// those: `e1` held none.
    #[test]
    fn a_child_killed_before_its_parent_is_recorded_with_it() {
        let mut edges: SlotMap<EdgeKey, ()> = SlotMap::with_key();
        let [e1, e13, e14, e15] = [(); 4].map(|()| edges.insert(()));
        let mut vertices: SlotMap<VertexKey, ()> = SlotMap::with_key();
        let mut join = |gone, kept| EdgeJoin {
            vertex: vertices.insert(()),
            gone,
            kept,
            conventional: None,
        };
        let joins = vec![join(e1, e13), join(e14, e15), join(e13, e15)];
        let split_from = BTreeMap::from([(e13, e1), (e14, e13), (e15, e13)]);
        let mut naming = SplitNaming::default();
        naming.record_joins(PlaneSide::Above, &split_from, joins.clone());
        assert_eq!(
            naming.joined_lineage,
            BTreeMap::from([(e14, e13), (e13, e1)]),
            "each killed child, with the edge its record names"
        );
        assert_eq!(
            naming.edge_joins,
            joins
                .into_iter()
                .map(|j| (PlaneSide::Above, j))
                .collect::<Vec<_>>(),
            "every join, with its side"
        );
    }
}

/// **The split types its at-rest refusal as the Boolean does, in its own
/// lever.** A side refused only on a reading in band — an aimed ray's
/// pair — is `ResultEscalated` on it, ending in the split's lever; beside
/// a definite finding it is the kernel's, `ResultNotAtRest`, and so is a
/// solid's volume whose certified enclosure holds zero, which is never in
/// band (D10).
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod at_rest_rows {
    use super::{PlaneSide, SplitFinishError};
    use crate::entity::{FaceKey, ShellKey, SolidKey};
    use crate::{
        AimedPair, BooleanDecision, OrientationUnread, ShellClassifyError, ValidationError,
    };
    use geom_core::{Band, Indeterminate, MarginDiag};

    #[test]
    fn an_in_band_shell_escalates_in_the_splits_lever() {
        let band = Band::new(1e-9, 1e-8).unwrap();
        let read = |margin, predicate| Indeterminate {
            margin,
            band,
            predicate: Some(predicate),
            terminal_sliver: false,
        };
        let gap = read(MarginDiag::value(3.05e-9), "side_at_infinity_order");
        let shell: ShellKey = slotmap::KeyData::from_ffi((1 << 32) | 1).into();
        let near: FaceKey = slotmap::KeyData::from_ffi((1 << 32) | 2).into();
        let role = ValidationError::ShellRoleUndecided {
            solid: SolidKey::default(),
            error: ShellClassifyError::RayInBand {
                shell,
                pair: Box::new(AimedPair {
                    shell,
                    aimed: near,
                    aim: "(0.5, 0.5, 1.0)".to_owned(),
                    near,
                    far: FaceKey::default(),
                    gap,
                }),
            },
        };
        let refusal = SplitFinishError::not_at_rest(PlaneSide::Below, vec![role.clone()]);
        assert!(
            matches!(
                &refusal,
                SplitFinishError::ResultEscalated {
                    side: PlaneSide::Below,
                    decision: BooleanDecision::ShellRole { near: n, others: 0, .. },
                    diag,
                } if *n == near && *diag == gap
            ),
            "an in-band pair alone escalates on it: {refusal:?}"
        );
        let msg = refusal.to_string();
        assert!(
            msg.starts_with("the piece on the below side of the plane: ")
                && msg.contains("whether a shell of the result bounds material or a cavity")
                && msg.contains(
                    "move the split plane or the geometry so the two faces the aimed ray met are \
                     clearly apart"
                )
                && !msg.contains("move the geometry so the two"),
            "the split's lever, not the Boolean's: {msg}"
        );
        // A solid no aimed ray read, its volume's certified enclosure
        // holding zero: the kernel's.
        let holding_zero = ValidationError::OrientationUndecided {
            solid: SolidKey::default(),
            reading: OrientationUnread::Volume {
                margin: Some(read(
                    MarginDiag::enclosure(-1e-20, 3e-20),
                    "positive_volume_exact",
                )),
                aimed: 0,
                set_aside: Vec::new(),
            },
        };
        assert!(
            matches!(
                SplitFinishError::not_at_rest(PlaneSide::Above, vec![holding_zero]),
                SplitFinishError::ResultNotAtRest { side: PlaneSide::Above, errors } if errors.len() == 1
            ),
            "an enclosure holding zero is the kernel's"
        );
        let beside = SplitFinishError::not_at_rest(
            PlaneSide::Above,
            vec![
                role,
                ValidationError::NegativeVolume {
                    solid: SolidKey::default(),
                },
            ],
        );
        assert!(
            matches!(
                &beside,
                SplitFinishError::ResultNotAtRest { side: PlaneSide::Above, errors } if errors.len() == 2
            ),
            "beside a definite finding it is the kernel's: {beside:?}"
        );
    }
}
