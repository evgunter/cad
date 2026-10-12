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
//! chasing; here the roles are the F9 keys
//! (`NullFacePair::Split { above_loop, below_loop }`), so promotion
//! reads the record, never the loop list.
//!
//! # Component classification and the degenerate net
//!
//! After `movefac`, a shell is classified by its section faces (seed
//! knowledge, Program 14.12) — mixed above/below section faces in one
//! shell is a typed kernel-bug error; a shell with NO section face
//! (an uncut component) falls back to its first vertex's cached side.
//! A shell consisting **only** of section faces bounds no volume. The
//! join's zero-area check refuses every section that could make one, so
//! such a shell is a kernel defect, refused loudly
//! ([`SplitFinishError::DegenerateSide`]).
//!
//! # Coplanar artifacts (documented, F7)
//!
//! Faces of the operand lying IN the split plane survive as walls of
//! one side (rule (a)), and cut faces are left as same-surface-key
//! pairs; `merge_coplanar_faces` is **deliberately not auto-run** on
//! the results — merging is never silent (the M2 ratification); the
//! caller opts in.
//!
//! A face-coplanar cut also lands OPERAND edges on the section
//! boundary with their transverse partner faces reassigned to the
//! other product, leaving intrinsic citations that no longer name the
//! edge's adjacent pair. The describe pass restates each one in what
//! the must-carry rule demands over its current pair
//! (`describe_section_boundary`'s smooth arm) — for two flush planes,
//! whose zero second order under-determines the locus, an image in the
//! section chart — so a coplanar product's boundary descriptions are
//! adjacency-coherent at rest; on a NEAR-flush operand the same
//! restatement meters the section chart's containment, and a line chord
//! the band cannot decide refuses as the edge's own reading
//! ([`SplitFinishError::EdgeOnFacesEscalated`]).

use geom_core::{Decide, Real, UnitVec3, Vec3};
use slotmap::SecondaryMap;

use super::join::{CompletedSection, loop_points_of};
use super::{KnifeEdge, KnifeEdgeSite, PlaneSide, SplitReduction, section_loops};
use crate::attach::{Named, Rechart};
use crate::body::Body;
use crate::boolean::{CarryRefusal, Cell};
use crate::chord_join::SplitJoinError;
use crate::entity::{EdgeKey, FaceKey, LoopBoundary, ShellKey, SolidKey, VertexKey};
use crate::euler::{EulerOpError, FaceSurface};
use geom::Surface;
use geom_brep::recourse::{LeverOnly, Reading, RefusedArm};
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
    /// The side's material, as an independent body.
    Body(Body<T>),
    /// No material on this side.
    Empty,
}

impl<T: Real> SplitPart<T> {
    /// The body, if this side has material.
    pub fn body(&self) -> Option<&Body<T>> {
        match self {
            Self::Body(b) => Some(b),
            Self::Empty => None,
        }
    }
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
        .filter_map(|(side, body)| body.map(|b| (side, b)))
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
    /// keys both sides were carved from, read through each side's joins:
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
            .collect();
        let section = |f: FaceKey| self.naming.sections.iter().any(|&(s, _)| s == f);
        let joins: Vec<crate::boolean::EdgeJoin> =
            self.naming.edge_joins.iter().map(|&(_, j)| j).collect();
        // Each copied vertex, and each root of the edge and face rows, is
        // stated once.
        let stated = |lineage: crate::Lineage, cell: Cell, fate: Fate| {
            lineage
                .with(cell, fate)
                .unwrap_or_else(|e| unreachable!("the split states each cell once: {e:?}"))
        };
        let mut lineage = crate::Lineage::new().through_joins(&joins).one_cut();
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
        for root in roots(&self.naming.edge_pieces) {
            let pieces: Vec<EdgeKey> = Descent::of(root, &self.naming.edge_pieces)
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
#[derive(Debug, Default)]
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
    /// mint order, call-time keys ([`crate::Body::split_edge`] keeps the
    /// divided edge's key on the piece from its start).
    pub edge_pieces: Vec<(crate::entity::EdgeKey, crate::entity::EdgeKey)>,
    /// Null-edge vertex pairs `(copy, original)` from the reduction's
    /// F9 records, in record order: each coincident copy with the
    /// vertex it was minted at (the naming layer derives the copy's
    /// parentage through the original's birth record). Which side
    /// holds which is not fixed — a copy stands for whichever piece its
    /// corner's arc holds — so consumers read a key's side from the body
    /// that holds it.
    pub vertex_pairs: Vec<(crate::entity::VertexKey, crate::entity::VertexKey)>,
    /// The joins each side ended with ([`crate::Body::join_edges`]), in
    /// the order made, with the side whose body they were made on: each
    /// row's `vertex` and `gone` edge are dead there, and `kept` holds
    /// them (`docs/DESIGN.md`, maximal edges).
    pub edge_joins: Vec<(PlaneSide, crate::boolean::EdgeJoin)>,
    /// `(killed edge, the edge it was split from)` for each edge a
    /// side's join killed that the cut had split off another
    /// (`Provenance::SplitEdge`), recorded before the kill: a kill
    /// takes its edge's birth record with it, and this is the lineage a
    /// joined edge's cover is read through to the operand edges it lies
    /// along.
    pub joined_lineage: Vec<(crate::entity::EdgeKey, crate::entity::EdgeKey)>,
}

/// Typed failure of the finish step.
#[derive(Debug)]
pub enum SplitFinishError {
    /// A component consists only of section faces, so it bounds no
    /// volume (kernel bug, loudly: the join's area certificate refuses
    /// every zero-area section before the finish runs). No degenerate
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
    /// ([`geom_brep::MustCarryRefusal::Refuted`]): the two readings
    /// disagree, so the split refuses rather than choose a description.
    SmoothJoinRefuted {
        /// The section-boundary edge.
        edge: EdgeKey,
    },
    /// A section loop's winding about its chart normal has no sign, so
    /// the section face's material side cannot be read: in the band
    /// (`diag`), or (`None`) zero, or unread because the loop carries a
    /// spiric or NURBS edge. The split's carrier gate admits only line,
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
    /// Two section faces each read as enclosing the other around a
    /// hole: their outlines were decided disjoint, and disjoint
    /// outlines cannot (kernel bug, loudly).
    NestingContradiction {
        /// The hole's section face (in the discarded scratch body).
        hole: FaceKey,
    },
    /// A finished side fails tier 2 ([`crate::validate_closed`]): a
    /// split never returns a body that is not a closed solid. Reached
    /// by a kernel defect, or by an operand that was not a closed
    /// solid to begin with: the operand is never validated, and the
    /// one tier-2 finding the reduction refuses is an empty OUTER loop
    /// on a face rule (a) measures at an ON vertex
    /// ([`super::SplitReduceError::UnboundedFace`], via
    /// `rules::face_extent`). `side` is in the caller's orientation.
    ResultInvalid {
        /// The side whose body failed.
        side: PlaneSide,
        /// The validator's findings.
        errors: Vec<crate::validate::ValidationError>,
    },
    /// The join a side ends with ([`crate::Body::join_edges`]) refused
    /// on that side's body.
    EdgeJoin {
        /// The side whose join refused.
        side: PlaneSide,
        /// Why the join refused, typed and keyless.
        refusal: crate::boolean::JoinRefusal,
    },
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

/// [`Body::set_face_surfaces_describing`] over a section re-chart, its
/// refusal for one of the `restated` edges read as [`of_attachment`]
/// reads it on that edge's carrier.
fn rechart_section<T: Decide + crate::props::AtRestPolicy>(
    body: &mut Body<T>,
    charts: Vec<Rechart<T>>,
    restated: &[(EdgeKey, geom_brep::EdgeCurveSpec<T>)],
    tol: Tol,
) -> Result<(), SplitFinishError> {
    body.set_face_surfaces_describing(charts, restated, tol)
        .map(drop)
        .map_err(|refusal| {
            let edge = match &refusal {
                EulerOpError::RechartFalsifies { edge, .. } => *edge,
                _ => return refusal.into(),
            };
            match restated.iter().find(|(e, _)| *e == edge) {
                Some((_, spec)) => of_attachment(edge, spec.carrier.kind(), refusal),
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
                    super::split_lever_ending(diag),
                )
                .fmt(f),
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
                    super::split_lever_ending(diag),
                )
                .fmt(f),
            Self::EdgeOnFacesEscalated { diag, .. } => diag
                .undecided(
                    crate::merge_faces::edge_on_faces_subject!("an edge along the cut"),
                    LeverOnly {
                        lever: crate::merge_faces::EDGE_ON_PLANE_LEVER,
                    }
                    .recourse(RefusedArm::Undecided(diag), Reading::Build),
                )
                .fmt(f),
            Self::SmoothJoinRefuted { .. } => write!(
                f,
                "two faces along the cut meet smoothly at the middle of their shared edge \
                 but at a corner elsewhere along it, so the split cannot say what that edge \
                 is. Recourse: {}",
                super::SPLIT_COINCIDENCE_RECOURSE
            ),
            Self::SectionWindingUndecided {
                diag: Some(diag), ..
            } => diag
                .undecided(
                    "which side of a cut face is material",
                    super::split_lever_ending(diag),
                )
                .fmt(f),
            Self::SectionWindingUndecided { diag: None, .. } => write!(
                f,
                "which side of a cut face is material cannot be read: its outline \
                 encloses no area, or has a spiric or NURBS edge, whose winding the \
                 kernel does not read. Recourse: {}",
                super::SPLIT_COINCIDENCE_RECOURSE
            ),
            Self::NestingContradiction { hole } => write!(
                f,
                "two cut faces each read as enclosing the other around hole {hole:?}. {}",
                geom_core::KERNEL_DEFECT_ENDING
            ),
            Self::KnifeEdge(k) => write!(f, "{k}"),
            Self::EdgeJoin { side, refusal } => {
                write!(
                    f,
                    "the piece on the {} side of the plane: {refusal}",
                    side.word()
                )
            }
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
) -> Result<SplitResult<T>, SplitFinishError> {
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
        joined_lineage: Vec::new(),
    };

    let band = geom_core::Band::linear(tol).map_err(SplitFinishError::Band)?;
    // ---- Promotion: each null face → two section faces. ----
    // Which loop is currently the ring is read from the face record
    // (the role keys), not from list position.
    let mut section_side: SecondaryMap<FaceKey, PlaneSide> = SecondaryMap::new();
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
        body.clear_null_face_pair(section.face);
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
    Ok(SplitResult {
        above: SplitPart::Body(above),
        below: SplitPart::Body(below),
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
/// keeps its own face** — sound by cancellation (volumes and
/// point-in-solid read it right; tier 3 passes it).
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
        for &(face, s) in &naming.sections {
            if s != side {
                continue;
            }
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
        let chart = body.get_face(parent).ok_or_else(corrupt)?.surface;
        let charts = vec![Rechart::shared(chart, hole, false)];
        let stranded = body.stranded_by(&charts)?;
        let restated = section_plane_restatements(body, hole, &stranded)?;
        // No pose is known to refuse here: the hole's edges were restated
        // in a section chart over the same plane at its promotion, and
        // read past the band there first; the parent's chart differs only
        // in its reference direction, so this read can differ from that
        // one by roundings.
        rechart_section(body, charts, &restated, tol)?;
        body.kfmrh(parent, hole)?;
        section_side.remove(hole);
        naming.sections.retain(|&(f, _)| f != hole);
    }
    Ok(())
}

/// The re-descriptions a section face's re-chart takes: every edge of
/// `face` among those the re-chart strands (`stranded`, from
/// [`Body::stranded_by`]) whose description names the chart the face
/// wears now, stated as an image in that chart. The re-chart reads
/// that image as the chart the face moves onto, or, where the edge's
/// other face keeps the chart, as that chart itself
/// ([`Body::set_face_surfaces_describing`]). Carrier, interval and a
/// declared authority travel verbatim.
fn section_plane_restatements<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    stranded: &[EdgeKey],
) -> Result<Vec<(EdgeKey, geom_brep::EdgeCurveSpec<T>)>, SplitFinishError> {
    let corrupt = || SplitFinishError::Corrupt;
    let face_data = body.get_face(face).ok_or_else(corrupt)?;
    let chart = face_data.surface;
    let loops: Vec<_> = core::iter::once(face_data.outer)
        .chain(face_data.rings.iter().copied())
        .collect();
    let mut out: Vec<(EdgeKey, geom_brep::EdgeCurveSpec<T>)> = Vec::new();
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
            let image = geom_brep::EdgeDescriptionSpec::chart(chart);
            let mut spec = curve.restated_spec();
            spec.description = match curve.authority() {
                geom_brep::EdgeAuthority::Declared(mc) => image.declared_by(mc),
                geom_brep::EdgeAuthority::Derived => image,
            };
            out.push((edge, spec));
        }
    }
    Ok(out)
}

/// D6 (M3 PR 6a): describes every boundary edge of one just-promoted
/// section face as the transverse `Intersection` of its two faces'
/// surfaces (read through [`geom_brep::IntersectionDraft`]: witness ON
/// the edge, at its carrier's mid-parameter), through the certified
/// [`crate::Body::set_edge_curve`] lane. A smooth neighbor stores
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
fn describe_section_boundary<T: Decide + crate::props::AtRestPolicy>(
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
            let (witness, arm) = (draft.witness, draft.extent);
            match geom_brep::classify_dihedral(surf_self, surf_other, witness, arm, band) {
                Ok(geom_brep::DihedralClass::Transverse) => {
                    let spec = draft.into_spec(geom_brep::SurfacePair::new(s_self, s_other));
                    let carrier = spec.carrier.kind();
                    body.set_edge_curve(edge, spec, tol)
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
                            geom_brep::folded_lever_arm(surf_self, surf_other, witness, arm),
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
                    let mut spec = existing.as_ref().map_or_else(
                        || {
                            unreachable!(
                                "{edge:?} read smooth at its witness has a certified carrier: \
                                 null scaffolding has coincident ends, so its zero extent \
                                 fails the dihedral's arm gate before this arm"
                            )
                        },
                        geom_brep::EdgeCurve::restated_spec,
                    );
                    // What the join stores is the must-carry rule's
                    // over the edge, and each refusal is this op's own.
                    let (pair, [sa, sb]) =
                        geom_brep::SurfacePair::sorted(s_self, surf_self, s_other, surf_other);
                    let demanded = geom_brep::must_carry_over_edge(
                        sa,
                        sb,
                        &spec.carrier,
                        spec.param_start,
                        spec.param_end,
                        arm,
                        band,
                    )
                    .description(pair, witness)
                    .map_err(|refusal| {
                        refusal.route(
                            |diag| SplitFinishError::DescribeEscalated { edge, diag },
                            |diag| SplitFinishError::DescribeBendEscalated { edge, diag },
                            || SplitFinishError::SmoothJoinRefuted { edge },
                        )
                    })?;
                    // A description of the demanded kind that names
                    // the edge's current pair stays verbatim (a stated
                    // image travels exactly); anything else — a
                    // citation whose partner this split reassigned to
                    // the other product, a scaffold, or the other kind
                    // — is restated. An edge between TWO section faces
                    // is visited once per face and ends with the FIRST
                    // visit's chart (section faces iterate in arena key
                    // order); either adjacent chart certifies.
                    let intrinsic =
                        matches!(demanded, geom_brep::MustCarryDescription::Intrinsic(_));
                    let coherent = existing.as_ref().is_some_and(|c| match *c.description() {
                        // A wrap edge's two halves bound one face (D1);
                        // this edge bounds two, so a wrap flag on it is
                        // restated.
                        geom_brep::EdgeDescription::Chart(ref ch) if ch.wrap => false,
                        geom_brep::EdgeDescription::Chart(ref ch) => {
                            !intrinsic && (ch.surface == s_self || ch.surface == s_other)
                        }
                        geom_brep::EdgeDescription::TangentIntersection { pair, .. } => {
                            intrinsic && pair == geom_brep::SurfacePair::new(s_self, s_other)
                        }
                        geom_brep::EdgeDescription::Intersection { .. }
                        | geom_brep::EdgeDescription::Scaffold(_) => false,
                    });
                    if !coherent {
                        // The conventional image rests in the section
                        // chart, which every section-boundary edge lies
                        // in to within the BAND: on a near-flush
                        // operand it is metered like any description
                        // (D4 ¶3).
                        spec.description = match demanded {
                            geom_brep::MustCarryDescription::Intrinsic(description) => description,
                            geom_brep::MustCarryDescription::Conventional => {
                                geom_brep::EdgeDescriptionSpec::chart(s_self)
                            }
                        };
                        if let Some(geom_brep::EdgeAuthority::Declared(mc)) =
                            existing.as_ref().map(|c| c.authority())
                        {
                            spec.description = spec.description.declared_by(mc);
                        }
                        let carrier = spec.carrier.kind();
                        body.set_edge_curve(edge, spec, tol)
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
) -> Result<SplitResult<T>, SplitFinishError> {
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
        Some(PlaneSide::Above) => Ok(SplitResult {
            above: SplitPart::Body(body),
            below: SplitPart::Empty,
            naming: SplitNaming::default(),
            coincidences: Vec::new(),
        }),
        Some(_) => Ok(SplitResult {
            above: SplitPart::Empty,
            below: SplitPart::Body(body),
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
fn section_sense<T: Decide>(
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
/// [`crate::Body::set_edge_curve`]'s plan, which every arm that
/// describes the edge reaches before its write. The row holds whichever
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
        let mut body = split.below.body().unwrap().clone();
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
        let mut body = split.below.body().unwrap().clone();
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
    /// rather than choose a description for an edge of neither kind.
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
            Err(SplitFinishError::SmoothJoinRefuted { edge: refused }) => {
                assert_eq!(refused, edge, "the refusal names the mixed edge");
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
                    "move the geometry so that edge is clearly longer and no face curves \
                     tightly there, or, if this length is intended, tighten the tolerance below ",
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
    /// now answers under-determined, so the edge is restated as an
    /// image in the section chart rather than kept.
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
        match stored.description() {
            geom_brep::EdgeDescription::Chart(c) => {
                assert_eq!(c.surface, s_self, "restated in the section chart");
            }
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
