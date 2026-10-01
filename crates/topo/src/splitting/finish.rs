//! Finish (ch. 14 §14.8, `splitfinish`): promote each completed null
//! face into TWO section faces (`mfkrh` — ring → face, the cross-shell
//! motion), distribute the now-disconnected components into shells
//! (`movefac`), classify each shell Above/Below, and **carve** the two
//! result bodies — functionally: the operand was never touched (the
//! pipeline runs on a clone), and both results come back as
//! independent [`Body`] values.
//!
//! # Section-face orientation (derived from `enters_material`, F3)
//!
//! A face's outward normal `m` points OUT of material: direction `d`
//! enters material iff `d·m < 0`. The above body's section face has
//! the above material on its `+n_SP` side, so `+n_SP` must ENTER ⇒
//! `n_SP·m < 0` ⇒ **m = −n_SP**; symmetrically the below body's
//! section face carries **m = +n_SP**. Both faces carry the SAME split
//! plane (same origin, same in-plane `u_ref` derived from the below
//! loop's first chord — deterministic data, no comparisons) with
//! opposite normals; the mirror test pins both signs bitwise.
//!
//! That is each face's CHART normal. Its sense is its loop's winding
//! about it, the reading tier 3's check 6 makes: a section's outer
//! boundary winds counter-clockwise (`true`). A section polygon that
//! is a hole in another — the bore's outline inside the cut through a
//! bored block — winds clockwise (`false`), and becomes a ring of the
//! section face of its side that encloses it wherever the two
//! outlines are decided disjoint (`nest_hole_sections`), so a holed
//! section is one face; where nothing decides it, the hole keeps a
//! face of its own that cancels the face around it.
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
//! A shell consisting **only** of section faces bounds no volume —
//! the second half of the one-sided-tangency net (the join's
//! zero-area check is the first) — and is refused typed
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
//! edge's adjacent pair. The describe pass restates those
//! conventionally in the section chart (`describe_section_boundary`'s
//! smooth arm), so a coplanar product's boundary descriptions are
//! adjacency-coherent at rest; on a NEAR-flush operand the same
//! restatement meters the section chart's containment and refuses
//! typed through certification when the band cannot decide it.

use geom_core::{Decide, Real, Vec3};
use slotmap::SecondaryMap;

use super::containment::{LoopContainment, point_in_carrier_loop};
use super::join::{CompletedSection, loop_points_of};
use super::{PlaneSide, SplitReduction};
use crate::attach::Rechart;
use crate::body::Body;
use crate::chord_join::SplitJoinError;
use crate::chord_join::ring_representative;
use crate::entity::{EdgeKey, FaceKey, LoopBoundary, ShellKey, SolidKey, VertexKey};
use crate::euler::{EulerOpError, FaceSurface};
use crate::validate::{RingOuterVerdict, ring_outer_contact};
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
    /// polygon nested as a ring of another (`nest_hole_sections`) has
    /// no face of its own and no row; the face that took it as a ring
    /// keeps its own row.
    pub sections: Vec<(FaceKey, PlaneSide)>,
    /// Chord-mef fragment rows: `(new face, divided-from face)` in
    /// mint order, call-time keys ([`crate::chord_join`]'s `ChordJoiner`
    /// log). Section faces appear here too (they are minted by the
    /// same mefs); consumers exclude the keys listed in `sections`.
    pub face_fragments: Vec<(FaceKey, FaceKey)>,
    /// Null-edge vertex pairs `(above copy, below original)` from the
    /// reduction's F9 records, in record order: the above-side
    /// coincident copies with the vertices they were minted at (the
    /// naming layer derives the above copy's parentage through the
    /// below original's birth record).
    pub vertex_pairs: Vec<(crate::entity::VertexKey, crate::entity::VertexKey)>,
}

/// Typed failure of the finish step.
#[derive(Debug)]
pub enum SplitFinishError {
    /// The operand must hold exactly one solid (the split contract;
    /// multi-solid models split solid-by-solid at the caller).
    NotSingleSolid {
        /// How many solids the operand holds.
        count: usize,
    },
    /// A component consists only of section faces — it bounds no
    /// volume (the one-sided tangency residue): no degenerate body is
    /// ever emitted.
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
    /// The section-boundary dihedral escalated while minting honest
    /// `Intersection` descriptions (M3 PR 6a, D6) — indeterminate
    /// wedge geometry at the section boundary refuses typed, never
    /// guesses a description.
    DescribeEscalated {
        /// The section-boundary edge.
        edge: EdgeKey,
        /// The classifier's diagnostic.
        diag: geom_core::Indeterminate,
    },
    /// A section loop's winding about its chart normal has no sign, so
    /// the section face's material side cannot be read: in the band
    /// (`diag`), zero, or (`None`) a loop with an edge that states no
    /// certified curve. The split's operand gate admits only line,
    /// circle and ellipse edges, so every section edge is one the
    /// winding reads.
    SectionWindingUndecided {
        /// The null face the section loop bounds.
        face: FaceKey,
        /// The winding's diagnostic, when it escalated.
        diag: Option<geom_core::Indeterminate>,
    },
    /// The split plane is tangent to a curved face along the section's
    /// boundary with the two faces' materials OPPOSED: the cut would
    /// leave a wedge end — a knife edge no input declared, which D1
    /// makes the minting op's refusal. (A cut piece lies on one side
    /// of the plane, so the end it can reach is the cusp, wedge 0.) A
    /// split has no declaration channel, so every such edge refuses; a
    /// π seam (materials aligned) cuts.
    SectionCusp {
        /// The section-boundary edge the knife edge would be.
        edge: EdgeKey,
        /// The operand's curved face the plane is tangent to.
        face: FaceKey,
    },
    /// Two section faces each read as enclosing the other around a
    /// hole: their outlines were decided disjoint, and disjoint
    /// outlines cannot (kernel bug, loudly).
    NestingContradiction {
        /// The hole's section face (in the discarded scratch body).
        hole: FaceKey,
    },
}

impl From<EulerOpError> for SplitFinishError {
    fn from(e: EulerOpError) -> Self {
        Self::Euler(e)
    }
}

impl core::fmt::Display for SplitFinishError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NotSingleSolid { count } => write!(
                f,
                "the body holds {count} solids, and a split takes exactly one"
            ),
            Self::DegenerateSide { side, .. } => write!(
                f,
                "the piece on the {} side of the plane bounds no volume (the residue of a \
                 one-sided tangency: only section faces). Recourse: move the split plane \
                 off the tangency",
                match side {
                    super::PlaneSide::Below => "below",
                    super::PlaneSide::On => "on",
                    super::PlaneSide::Above => "above",
                }
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
            Self::DescribeEscalated { diag, .. } => write!(
                f,
                "the angle between two faces along the cut is too close to call ({}). \
                 Recourse: {}",
                diag.payload(),
                super::SPLIT_COINCIDENCE_RECOURSE
            ),
            Self::SectionWindingUndecided {
                diag: Some(diag), ..
            } => write!(
                f,
                "which side of a cut face is material is too close to call ({}). \
                 Recourse: {}",
                diag.payload(),
                super::SPLIT_COINCIDENCE_RECOURSE
            ),
            Self::SectionWindingUndecided { diag: None, .. } => write!(
                f,
                "which side of a cut face is material cannot be read: its outline \
                 encloses no area, or has an edge with no curve. Recourse: move the \
                 split plane"
            ),
            Self::NestingContradiction { hole } => write!(
                f,
                "two cut faces each read as enclosing the other around hole {hole:?} (kernel bug)"
            ),
            Self::SectionCusp { .. } => write!(
                f,
                "the split plane is tangent to a curved face where it cuts, so a piece \
                 would taper to a knife edge nobody asked for. Recourse: move the split \
                 plane off the tangency"
            ),
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
pub(super) fn split_finish<T: Decide>(
    red: SplitReduction<T>,
    completed: &[CompletedSection],
    face_fragments: Vec<(FaceKey, FaceKey)>,
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
        return whole_body_side(reassembled, &red.sides);
    }
    let mut naming = SplitNaming {
        sections: Vec::with_capacity(completed.len() * 2),
        face_fragments,
        vertex_pairs: red
            .null_edges
            .iter()
            .map(|r| (r.attr.above_end, r.attr.below_end))
            .collect(),
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
        // Deterministic in-plane u axis: the below loop's first chord.
        let u_ref = below_chord_u_ref(&body, section)?;
        let plane_for = |side: PlaneSide| -> Surface<T> {
            let normal = match side {
                // Derived (module docs): above section face m = −n_SP,
                // below section face m = +n_SP.
                PlaneSide::Above => -red.plane.normal,
                _ => red.plane.normal,
            };
            Surface::Plane {
                origin: red.plane.origin,
                normal,
                u_ref,
            }
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
        let ring_sense = section_sense(&body, section.face, ring, &plane_for(ring_side), band)?;
        let outer_sense = section_sense(&body, section.face, outer, &plane_for(other_side), band)?;
        let promoted = body.mfkrh(ring, FaceSurface::Inherit)?;
        body.set_face_surfaces_describing(
            vec![Rechart::new(
                plane_for(ring_side),
                promoted.face,
                ring_sense,
            )],
            &[],
            tol,
        )?;
        // The face the null pair leaves on its old chart moves last, and
        // the edges still described against that chart move with it,
        // restated in the section plane every section boundary edge lies
        // in; the boundary pass below gives each its honest class.
        let restated = section_plane_restatements(&body, section.face)?;
        body.set_face_surfaces_describing(
            vec![Rechart::new(
                plane_for(other_side),
                section.face,
                outer_sense,
            )],
            &restated,
            tol,
        )?;
        body.clear_null_face_pair(section.face);
        section_side.insert(promoted.face, ring_side);
        section_side.insert(section.face, other_side);
        naming.sections.push((promoted.face, ring_side));
        naming.sections.push((section.face, other_side));
    }

    // ---- Nesting: a section that is a hole in another becomes its
    // ring, so a holed section is one face. ----
    nest_hole_sections(&mut body, &mut section_side, &mut naming, band, tol)?;

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
    })
}

/// Each section face that is a hole — sense `false`: its loop winds
/// clockwise about its outward normal — becomes a ring of the section
/// face of its side that immediately encloses it, and the hole's face
/// dies (`kfmrh`). A section with holes is then one face whose rings
/// are the holes' sections, the encoding tier 3 reads, rather than a
/// face over the whole outline plus a coplanar face per hole that
/// cancels it.
///
/// A face encloses the hole when the two outlines are decided
/// disjoint ([`outlines_disjoint`]), as the hole is from every ring
/// the face already holds, and the hole's anchor vertex is
/// certified inside the face's outline on the loops' own carriers
/// ([`point_in_carrier_loop`]). Disjoint outlines nest or are apart
/// (Jordan), so among several enclosing faces — an island in a hole in
/// a face — exactly one is enclosed by all the others, and the hole
/// goes to it; two such would be two outlines each enclosing the other,
/// which disjoint outlines cannot be, and that refuses as a kernel bug
/// ([`SplitFinishError::NestingContradiction`]). The hole moves onto
/// that face's chart first, its boundary restated with it, so the ring
/// rides the chart of the face that keeps it.
///
/// **Where nothing decides, the hole keeps its own face** — the
/// encoding the split produced before this step, sound by
/// cancellation (volumes and point-in-solid read it right; tier 3
/// passes it). That is the case for an outline edge on a spiric or
/// NURBS carrier, whose contacts nothing here decides, and for a
/// containment or contact reading in the band. A clockwise polygon
/// touching the outline around it would reach here too: that is what a
/// chord run outside the face it divides makes, and the join pairs a
/// face's crossings along the face's own section line or conic so that
/// none does. A face it leaves to the sweep's order — a curved face
/// whose section is straight, a planar face whose line the band cannot
/// certify — is not covered by that pairing.
///
/// # Errors
///
/// [`SplitFinishError::NestingContradiction`]; [`SplitFinishError::Euler`]
/// and [`SplitFinishError::Corrupt`] from the surgery.
fn nest_hole_sections<T: Decide>(
    body: &mut Body<T>,
    section_side: &mut SecondaryMap<FaceKey, PlaneSide>,
    naming: &mut SplitNaming,
    band: geom_core::Band,
    tol: Tol,
) -> Result<(), SplitFinishError> {
    let corrupt = || SplitFinishError::Corrupt;
    let mut holes = Vec::new();
    for &(face, side) in &naming.sections {
        if !body.get_face(face).ok_or_else(corrupt)?.sense {
            holes.push((face, side));
        }
    }
    for (hole, side) in holes {
        let encloses = |outer: FaceKey, inner: FaceKey| -> Result<bool, SplitFinishError> {
            let outer_data = body.get_face(outer).ok_or_else(corrupt)?;
            let inner_loop = body.get_face(inner).ok_or_else(corrupt)?.outer;
            let Some(&Surface::Plane { normal, .. }) = body.get_surface(outer_data.surface) else {
                return Err(corrupt());
            };
            if !outlines_disjoint(body, outer_data.outer, inner_loop, band)? {
                return Ok(false);
            }
            let q = ring_representative(body, inner_loop).map_err(|_| corrupt())?;
            Ok(matches!(
                point_in_carrier_loop(body, outer_data.outer, normal, q, band),
                Ok(Some(LoopContainment::In))
            ))
        };
        let mut enclosing = Vec::new();
        for &(f, s) in &naming.sections {
            if s == side && body.get_face(f).ok_or_else(corrupt)?.sense && encloses(f, hole)? {
                enclosing.push(f);
            }
        }
        let mut innermost = Vec::new();
        for &f in &enclosing {
            let mut inside_all = true;
            for &g in &enclosing {
                if g != f && !encloses(g, f)? {
                    inside_all = false;
                }
            }
            if inside_all {
                innermost.push(f);
            }
        }
        let parent = match innermost[..] {
            [] => continue,
            [parent] => parent,
            _ => return Err(SplitFinishError::NestingContradiction { hole }),
        };
        // Tier 3 compares a ring with its face's outer loop only, so a
        // ring that met another ring would pass it: a hole joins only
        // rings it is decided disjoint from.
        let hole_loop = body.get_face(hole).ok_or_else(corrupt)?.outer;
        let mut clear = true;
        for &r in &body.get_face(parent).ok_or_else(corrupt)?.rings {
            clear = clear && outlines_disjoint(body, r, hole_loop, band)?;
        }
        if !clear {
            continue;
        }
        let chart = body.get_face(parent).ok_or_else(corrupt)?.surface;
        let restated = section_plane_restatements(body, hole)?;
        body.set_face_surfaces_describing(
            vec![Rechart::shared(chart, hole, false)],
            &restated,
            tol,
        )?;
        body.kfmrh(parent, hole)?;
        section_side.remove(hole);
        naming.sections.retain(|&(f, _)| f != hole);
    }
    Ok(())
}

/// Whether two loops in the section plane are DECIDED disjoint: `true`
/// only where every pair of their edges is.
///
/// - **Line and circle edges**: tier 3's check 9 contact reading
///   ([`ring_outer_contact`]), which on a planar face whose loops carry
///   only these kinds decides every shared point — at a vertex of
///   either loop, along an arc, or where two edges cross.
/// - **A pair with an ellipse edge**, which check 9's edge arms skip:
///   separated on the CARRIERS, a superset of the arcs. A line against
///   a conic: the conic's whole carrier lies on one side of the line
///   through the segment (**`split_nest_line_conic`**, the offset of
///   the conic's centre from the line less its amplitude across it,
///   `√((m·a)² + (m·b)²)`). Two conics: in the unit coordinates of one,
///   the other's carrier lies wholly inside or wholly outside the unit
///   circle, either way round (**`split_nest_conic_conic`**; the bound
///   is at [`conics_clear`]).
/// - **A spiric or NURBS edge, or an edge with no certified curve**:
///   nothing decides it, so `false`.
///
/// A margin in the band is `false` too: an undecided pair is not a
/// disjoint one.
fn outlines_disjoint<T: Decide>(
    body: &Body<T>,
    a: crate::entity::LoopKey,
    b: crate::entity::LoopKey,
    band: geom_core::Band,
) -> Result<bool, SplitFinishError> {
    if !matches!(
        ring_outer_contact(body, a, b, band),
        RingOuterVerdict::Disjoint
    ) {
        return Ok(false);
    }
    let (ea, eb) = (loop_edges(body, a)?, loop_edges(body, b)?);
    for x in &ea {
        for y in &eb {
            let separated = match (x, y) {
                (OutlineEdge::Undecided, _) | (_, OutlineEdge::Undecided) => return Ok(false),
                (OutlineEdge::Line(p, q), OutlineEdge::Conic(c, true))
                | (OutlineEdge::Conic(c, true), OutlineEdge::Line(p, q)) => {
                    line_clears_conic(*p, *q, c, band)
                }
                (OutlineEdge::Conic(c, true), OutlineEdge::Conic(d, _))
                | (OutlineEdge::Conic(c, _), OutlineEdge::Conic(d, true)) => {
                    conics_clear(c, d, band)
                }
                // Lines and circles: check 9 decided them above.
                _ => true,
            };
            if !separated {
                return Ok(false);
            }
        }
    }
    Ok(true)
}

/// One outline edge, as [`outlines_disjoint`] reads it: a segment by
/// its ends; a conic by its carrier, flagged `true` for an ellipse
/// (the kind check 9's edge arms skip); or a kind nothing decides.
enum OutlineEdge<T: Real> {
    Line(geom_core::Point3<T>, geom_core::Point3<T>),
    Conic(Conic<T>, bool),
    Undecided,
}

/// A conic carrier: centre and the two semi-axis vectors.
struct Conic<T: Real> {
    centre: geom_core::Point3<T>,
    a: Vec3<T>,
    b: Vec3<T>,
}

fn loop_edges<T: Decide>(
    body: &Body<T>,
    l: crate::entity::LoopKey,
) -> Result<Vec<OutlineEdge<T>>, SplitFinishError> {
    let corrupt = || SplitFinishError::Corrupt;
    // A lone-vertex loop bounds nothing a contact reading could clear.
    let first = match body.get_loop(l).ok_or_else(corrupt)?.boundary {
        LoopBoundary::Cycle { first } => first,
        LoopBoundary::Empty { .. } => return Ok(vec![OutlineEdge::Undecided]),
    };
    let mut out = Vec::new();
    for he in body.loop_cycle(first).ok_or_else(corrupt)? {
        let h = body.get_half_edge(he).ok_or_else(corrupt)?;
        let edge = body.get_edge(h.edge).ok_or_else(corrupt)?;
        let Some(curve) = body
            .get_curve_geom(edge.curve)
            .and_then(crate::null::CurveGeom::certified)
        else {
            out.push(OutlineEdge::Undecided);
            continue;
        };
        out.push(match *curve.carrier() {
            geom::Curve3::Line { .. } => {
                let end = body.half_edge_end(he).ok_or_else(corrupt)?;
                let point = |v| -> Result<geom_core::Point3<T>, SplitFinishError> {
                    let p = body.get_vertex(v).ok_or_else(corrupt)?.point;
                    body.get_point(p).copied().ok_or_else(corrupt)
                };
                OutlineEdge::Line(point(h.start)?, point(end)?)
            }
            geom::Curve3::Circle {
                center,
                axis,
                radius,
                u_ref,
            } => OutlineEdge::Conic(
                Conic {
                    centre: center,
                    a: u_ref * radius,
                    b: axis.cross(u_ref) * radius,
                },
                false,
            ),
            geom::Curve3::Ellipse {
                center,
                axis,
                major,
                minor,
                u_ref,
            } => OutlineEdge::Conic(
                Conic {
                    centre: center,
                    a: u_ref * major,
                    b: axis.cross(u_ref) * minor,
                },
                true,
            ),
            geom::Curve3::Spiric { .. } | geom::Curve3::Nurbs(_) => OutlineEdge::Undecided,
        });
    }
    Ok(out)
}

/// The conic's whole carrier lies definitely on one side of the line
/// through `p`, `q` (both in the conic's plane).
fn line_clears_conic<T: Decide>(
    p: geom_core::Point3<T>,
    q: geom_core::Point3<T>,
    c: &Conic<T>,
    band: geom_core::Band,
) -> bool {
    let along = q - p;
    if !positive("split_nest_line_conic", along.norm(), band)
        || !positive("split_nest_line_conic", c.a.norm().min(c.b.norm()), band)
    {
        return false;
    }
    let m = c.a.cross(c.b).normalize().cross(along.normalize());
    let reach = (m.dot(c.a).powi(2) + m.dot(c.b).powi(2)).sqrt();
    positive(
        "split_nest_line_conic",
        m.dot(c.centre - p).abs() - reach,
        band,
    )
}

/// `margin` (metres) definitely positive under `band`.
fn positive<T: Decide>(name: &'static str, margin: T, band: geom_core::Band) -> bool {
    matches!(
        crate::validate::decide(name, geom_core::Margin::of(margin), band),
        Ok(geom_core::Sign::Positive)
    )
}

/// `h`'s carrier lies definitely inside or definitely outside `e`'s
/// (both in one plane).
///
/// **The bound.** In `e`'s unit coordinates (`e` the unit circle), `h`
/// is `c′ + M·(cos θ, sin θ)` with `M = [a′ b′]`, so its distance from
/// the origin lies in `[|c′| − σ, |c′| + σ]`, `σ` the largest singular
/// value of `M`: `σ² = (S + √((|a′|² − |b′|²)² + 4(a′·b′)²)) / 2`,
/// `S = |a′|² + |b′|²` — exact for the 2×2, with the radicand a sum of
/// squares so rounding cannot take it negative. `h` is inside when
/// `|c′| + σ < 1`, outside when `|c′| − σ > 1`. Both margins are levered
/// by `e`'s smaller semi-axis, the least a unit step in its coordinates
/// spans in metres. `σ` is padded up by a few ulps so the bound stays an
/// upper bound under rounding; the triangle inequality in `|c′| ± σ`
/// is the one remaining slack (exact for concentric conics).
fn conics_clear<T: Decide>(e: &Conic<T>, h: &Conic<T>, band: geom_core::Band) -> bool {
    let (ae, be) = (e.a.norm(), e.b.norm());
    let lever = ae.min(be);
    if !positive("split_nest_conic_conic", lever, band) {
        return false;
    }
    let unit = |v: Vec3<T>| (v.dot(e.a) / ae.powi(2), v.dot(e.b) / be.powi(2));
    let (cx, cy) = unit(h.centre - e.centre);
    let (ax, ay) = unit(h.a);
    let (bx, by) = unit(h.b);
    let centre = (cx.powi(2) + cy.powi(2)).sqrt();
    let (aa, bb, ab) = (
        ax.powi(2) + ay.powi(2),
        bx.powi(2) + by.powi(2),
        ax * bx + ay * by,
    );
    let spread = ((aa - bb).powi(2) + T::from_f64(4.0) * ab.powi(2)).sqrt();
    let reach =
        ((aa + bb + spread) * T::from_f64(0.5)).sqrt() * T::from_f64(1.0 + 8.0 * f64::EPSILON);
    let inside = (T::one() - centre - reach) * lever;
    let outside = (centre - reach - T::one()) * lever;
    positive("split_nest_conic_conic", inside, band)
        || positive("split_nest_conic_conic", outside, band)
}

/// The sense of the section face a promoted loop will bound, charted
/// on `plane`: the loop's winding about the chart normal (interior-left
/// ⇒ counter-clockwise about the outward normal), read by the function
/// tier 3's check 6 falsifies the bit with. A section's outer boundary
/// winds counter-clockwise about the normal `plane_for` gives it; a
/// hole's polygon winds clockwise and is `false`, which is how
/// `nest_hole_sections` finds it.
///
/// # Errors
///
/// [`SplitFinishError::SectionWindingUndecided`] where the winding has
/// no sign (in the band, zero, or a loop with an edge that states no
/// certified curve); [`SplitFinishError::Corrupt`] on a torn loop.
fn section_sense<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    l: crate::entity::LoopKey,
    plane: &Surface<T>,
    band: geom_core::Band,
) -> Result<bool, SplitFinishError> {
    let Surface::Plane { normal, .. } = *plane else {
        return Err(SplitFinishError::Corrupt);
    };
    match body
        .planar_loop_winding(l, normal, band)
        .map_err(|_| SplitFinishError::Corrupt)?
    {
        Some(Ok(geom_core::Sign::Positive)) => Ok(true),
        Some(Ok(geom_core::Sign::Negative)) => Ok(false),
        Some(Ok(geom_core::Sign::Zero)) | None => {
            Err(SplitFinishError::SectionWindingUndecided { face, diag: None })
        }
        Some(Err(diag)) => Err(SplitFinishError::SectionWindingUndecided {
            face,
            diag: Some(diag),
        }),
    }
}

/// The re-descriptions a section face's re-chart takes: every edge of
/// `face` whose description names the chart the face wears now, where
/// the edge's other face does not wear it, stated as an image in that
/// chart — which the re-chart reads as the section plane the face moves
/// onto ([`Body::set_face_surfaces_describing`]). Carrier, interval and
/// a declared authority travel verbatim; a null edge has no description
/// to restate.
fn section_plane_restatements<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
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
            if out.iter().any(|(e, _)| *e == edge) {
                continue;
            }
            let edge_data = body.get_edge(edge).ok_or_else(corrupt)?;
            let mate = if edge_data.he_plus == he {
                edge_data.he_minus
            } else {
                edge_data.he_plus
            };
            let other = body.face_of_half_edge(mate).ok_or_else(corrupt)?;
            if body.get_face(other).ok_or_else(corrupt)?.surface == chart {
                continue;
            }
            let geom = body.get_curve_geom(edge_data.curve).ok_or_else(corrupt)?;
            let Some(curve) = geom.certified() else {
                continue;
            };
            if !Body::description_surfaces(geom).contains(&chart) {
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
/// [`crate::Body::set_edge_curve`] lane. Smooth neighbors (flush
/// ON-faces — parallel planes under-determine the locus) carry a
/// conventional description (D2's conventional split): one already
/// drawn in an adjacent chart is kept verbatim (a stated image
/// travels exactly — deriving a replacement would trade a statement
/// for a guess), and any other description is restated as an image in
/// the section chart, which every section-boundary edge lies in to
/// within the band (a near-flush operand refuses typed through the
/// certification lane). That covers the citation this split itself made
/// stale: on a face-coplanar cut an operand edge lands on the section
/// boundary with its transverse partner reassigned to the OTHER
/// product, so the `Intersection` it honestly carried now names a
/// surface that is not adjacent (and not even present) on this side.
/// A curved wall smooth against the section is judged by its material
/// pairing: aligned is a π seam and takes the conventional path,
/// opposed is a wedge end nothing declared and refuses
/// ([`SplitFinishError::SectionCusp`]).
/// Escalations are typed ([`SplitFinishError::DescribeEscalated`]).
fn describe_section_boundary<T: Decide>(
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
                .get_curve_geom(edge_data.curve)
                .and_then(crate::null::CurveGeom::certified)
                .cloned();
            let draft = geom_brep::IntersectionDraft::of(existing.as_ref(), p0, p1);
            let (witness, arm) = (draft.witness, draft.extent);
            match geom_brep::classify_dihedral(surf_self, surf_other, witness, arm, band) {
                Ok(geom_brep::DihedralClass::Transverse) => {
                    body.set_edge_curve(edge, draft.into_spec(s_self, s_other), tol)?;
                }
                // Smooth: the surfaces under-determine the locus, so
                // the honest class is conventional (D2). A description
                // already drawn in one of the edge's two charts stays
                // verbatim; anything else — a citation whose partner
                // this split reassigned to the other product, or a
                // scaffold — is restated as an image in the section
                // chart. The edge lies in that chart to within the
                // BAND, not bitwise: on a near-flush operand the
                // restated image is metered like any description and
                // an in-band containment refuses through
                // certification's escalation lane instead of adopting
                // an indeterminate locus (D4 ¶3). Carrier and interval
                // travel verbatim (restated, never rebuilt), as does a
                // declared authority. An edge between TWO section
                // faces is visited once per face; the chart it ends
                // with is the FIRST visit's (the restate), the second
                // visit keeping it as coherent — deterministic
                // (section faces iterate in arena key order), and
                // legal either way since either adjacent chart
                // certifies.
                //
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
                        .map_err(|diag| SplitFinishError::DescribeEscalated { edge, diag })?;
                        if pairing == geom_brep::MaterialPairing::Opposed {
                            return Err(SplitFinishError::SectionCusp {
                                edge,
                                face: other_face,
                            });
                        }
                    }
                    let coherent = existing.as_ref().is_some_and(|c| match *c.description() {
                        // A seam image's two sides are one surface, so
                        // it is coherent only when both faces share
                        // its chart — the same clause the adjacency
                        // validators apply. No section boundary mints
                        // a seam; the clause is here so three
                        // spellings of one rule do not drift.
                        geom_brep::EdgeDescription::Chart(ref ch) if ch.seam => {
                            ch.surface == s_self && ch.surface == s_other
                        }
                        // The `s_self` half is spelled for symmetry
                        // and is unreachable: section surfaces are
                        // minted fresh by THIS pass, so a pre-existing
                        // description can only name `s_other`, and a
                        // same-pass restate is only ever re-seen from
                        // the edge's other face.
                        geom_brep::EdgeDescription::Chart(ref ch) => {
                            ch.surface == s_self || ch.surface == s_other
                        }
                        // Kept when honest for the CURRENT pair — a
                        // curved wall meeting the section at a π seam.
                        geom_brep::EdgeDescription::TangentIntersection { s1, s2, .. } => {
                            (s1 == s_self && s2 == s_other) || (s1 == s_other && s2 == s_self)
                        }
                        // A transverse citation on a definitely-smooth
                        // pair is wrong whatever it names, and a
                        // scaffold at rest is fenced — both restate.
                        geom_brep::EdgeDescription::Intersection { .. }
                        | geom_brep::EdgeDescription::Scaffold(_) => false,
                    });
                    if !coherent {
                        let mut spec = match &existing {
                            Some(c) => c.restated_spec(),
                            // Unreachable, not a licence to rebuild:
                            // the operand gate refuses uncertified
                            // edges (`ScaffoldingOperand`) and every
                            // split-minted edge certifies at its mint,
                            // so a section-boundary edge always has a
                            // carrier to restate.
                            None => geom_brep::EdgeCurveSpec::line_between(p0, p1),
                        };
                        spec.description = geom_brep::EdgeDescriptionSpec::chart(s_self);
                        // The declared carry: no committed operand
                        // puts Declared authority on a section
                        // boundary (`bool1_r1_probes`' authority
                        // census measures 0 before and after), and
                        // the carry stands because dropping a
                        // declaration would silently flip
                        // `EdgeAuthority::is_declared`, which tier 3's
                        // prefer-intrinsic rules read.
                        if let Some(geom_brep::EdgeAuthority::Declared(mc)) =
                            existing.as_ref().map(|c| c.authority())
                        {
                            spec.description = spec.description.declared_by(mc);
                        }
                        body.set_edge_curve(edge, spec, tol)?;
                    }
                }
                Err(geom_brep::LeverEscalation { diag, .. }) => {
                    return Err(SplitFinishError::DescribeEscalated { edge, diag });
                }
            }
        }
    }
    Ok(())
}

/// The operand's single solid.
pub(crate) fn single_solid<T: Decide>(body: &Body<T>) -> Result<SolidKey, SplitFinishError> {
    let mut it = body.solids();
    let first = it.next();
    let extra = it.count();
    match (first, extra) {
        (Some((k, _)), 0) => Ok(k),
        (first, extra) => Err(SplitFinishError::NotSingleSolid {
            count: usize::from(first.is_some()) + extra,
        }),
    }
}

/// The un-cut case: the whole body lands on one side, decided by the
/// first non-ON cached vertex verdict (arena order — deterministic).
fn whole_body_side<T: Decide>(
    body: Body<T>,
    sides: &SecondaryMap<VertexKey, PlaneSide>,
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
    match side {
        Some(PlaneSide::Above) => Ok(SplitResult {
            above: SplitPart::Body(body),
            below: SplitPart::Empty,
            naming: SplitNaming::default(),
        }),
        Some(_) => Ok(SplitResult {
            above: SplitPart::Empty,
            below: SplitPart::Body(body),
            naming: SplitNaming::default(),
        }),
        // Every vertex ON: a zero-volume operand — nothing legal
        // reaches here (tier 2 refused it long ago).
        None => Err(SplitFinishError::Corrupt),
    }
}

/// Deterministic in-plane u axis from the below loop's first chord
/// (two adjacent section corners — distinct certified endpoints, so
/// the chord is nonzero; evaluation lane, no comparisons).
fn below_chord_u_ref<T: Decide>(
    body: &Body<T>,
    section: &CompletedSection,
) -> Result<Vec3<T>, SplitFinishError> {
    let points = loop_points_of(body, section.below_loop).map_err(|e| match e {
        SplitJoinError::Euler(err) => SplitFinishError::Euler(err),
        _ => SplitFinishError::Corrupt,
    })?;
    if points.len() < 2 {
        return Err(SplitFinishError::Corrupt);
    }
    Ok((points[1] - points[0]).normalize())
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
        body.point_origins.remove(k);
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
        body.curve_origins.remove(k);
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
        for s in Body::description_surfaces(curve) {
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
        // A raw removal has to reach the side tables (pinned from the
        // split door in `sweep`'s `seat6_germ_channel`).
        body.drop_surface_rows(k);
    }
    Ok(body)
}
