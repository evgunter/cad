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

use geom_core::{Decide, Real, Vec3};
use slotmap::SecondaryMap;

use super::join::{CompletedSection, loop_points_of};
use super::{PlaneSide, SplitReduction};
use crate::body::Body;
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
    /// The section faces with their side, in section completion order
    /// (the `order` module's total exact-order sort: reorderings are
    /// recorded predicate verdicts, so the position is a function of
    /// the verdict vector — N4's covariance).
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
                "split finish: operand holds {count} solids — split takes exactly one"
            ),
            Self::DegenerateSide { shell, side } => write!(
                f,
                "split finish: component {shell:?} on the {side:?} side has no real \
                 material (only section faces) — degenerate piece refused, never emitted"
            ),
            Self::TornComponent { shell } => write!(
                f,
                "split finish: component {shell:?} carries section faces of both sides \
                 (kernel bug)"
            ),
            Self::UnclassifiableComponent { shell } => write!(
                f,
                "split finish: component {shell:?} has no section face and no off-plane \
                 vertex to classify by"
            ),
            Self::Corrupt => write!(f, "split finish: traversal failed (corrupt body)"),
            Self::Euler(e) => write!(f, "split finish: euler operation refused: {e}"),
            Self::Band(e) => write!(f, "split finish: no classification band: {e}"),
            Self::DescribeEscalated { edge, diag } => write!(
                f,
                "split finish: section-boundary dihedral escalated at edge {edge:?} \
                 while minting its description: {diag}"
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
    let mut body = red.body;
    let solid = single_solid(&body)?;

    // No section polygons: the plane did not cut — the whole operand
    // is one side (an ON-touching contact mints no null faces).
    if completed.is_empty() {
        return whole_body_side(body, &red.sides);
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
        let promoted = body.mfkrh(ring, FaceSurface::New(plane_for(ring_side)))?;
        body.set_face_surface(section.face, FaceSurface::New(plane_for(other_side)))?;
        body.clear_null_face_pair(section.face);
        section_side.insert(promoted.face, ring_side);
        section_side.insert(section.face, other_side);
        naming.sections.push((promoted.face, ring_side));
        naming.sections.push((section.face, other_side));
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
    let band = geom_core::Band::linear(tol).map_err(SplitFinishError::Band)?;
    let section_faces: Vec<FaceKey> = section_side.keys().collect();
    for face in section_faces {
        describe_section_boundary(&mut body, face, band, tol)?;
    }

    // ---- Distribution: movefac every shell of the solid. ----
    let shells: Vec<ShellKey> = body
        .get_solid(solid)
        .ok_or(SplitFinishError::Corrupt)?
        .shells
        .clone();
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
    let above = carve(&body, solid, &above_shells)?;
    let below = carve(&body, solid, &below_shells)?;
    Ok(SplitResult {
        above: SplitPart::Body(above),
        below: SplitPart::Body(below),
        naming,
    })
}

/// D6 (M3 PR 6a): describes every boundary edge of one just-promoted
/// section face as the transverse `Intersection` of its two faces'
/// surfaces (witness at the chord midpoint), through the certified
/// [`crate::Body::set_edge_curve`] lane. Smooth neighbors (flush
/// ON-faces — parallel planes under-determine the locus) carry a
/// conventional description (D2's conventional split): one already
/// drawn in an adjacent chart is kept verbatim (a stated image
/// travels exactly — deriving a replacement would trade a statement
/// for a guess), and any other description is restated as an image in
/// the section chart, which every section-boundary edge lies in by
/// construction. That covers the citation this split itself made
/// stale: on a face-coplanar cut an operand edge lands on the section
/// boundary with its transverse partner reassigned to the OTHER
/// product, so the `Intersection` it honestly carried now names a
/// surface that is not adjacent (and not even present) on this side.
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
            let other_loop = body.get_half_edge(mate).ok_or_else(corrupt)?.parent_loop;
            let other_face = body.get_loop(other_loop).ok_or_else(corrupt)?.face;
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
            // Conic section chords (M5 PR 5) keep their certified
            // carrier and interval — only the description upgrades
            // (the witness re-minted at the carrier's mid-parameter,
            // the witness contract); line chords keep the M3 path
            // byte-identically. The dihedral witness/arm likewise use
            // the carrier's honest mid-point and extent for conics
            // (the chord collapses on near-closed arcs).
            let existing = body
                .get_curve_geom(edge_data.curve)
                .and_then(crate::null::CurveGeom::certified)
                .cloned();
            let conic = existing.as_ref().and_then(|c| match c.carrier() {
                geom::Curve3::Circle { .. } | geom::Curve3::Ellipse { .. } => {
                    let (t0, t1) = c.params();
                    let mid = c.carrier().eval(t0 + (t1 - t0) * T::from_f64(0.5));
                    let arm = geom_brep::edge_extent(c.carrier(), t0, t1, p0.distance(p1));
                    Some((c.clone(), mid, arm))
                }
                geom::Curve3::Line { .. } | geom::Curve3::Nurbs(_) => None,
            });
            let (witness, arm) = match &conic {
                Some((_, mid, arm)) => (*mid, *arm),
                None => (p0.lerp(p1, T::from_f64(0.5)), p0.distance(p1)),
            };
            match geom_brep::classify_dihedral(surf_self, surf_other, witness, arm, band) {
                Ok(geom_brep::DihedralClass::Transverse) => {
                    let spec = match conic {
                        Some((curve, _, _)) => {
                            let (t0, t1) = curve.params();
                            geom_brep::EdgeCurveSpec {
                                description: geom_brep::EdgeDescriptionSpec::Intersection {
                                    s1: s_self,
                                    s2: s_other,
                                    witness,
                                },
                                carrier: curve.carrier().clone(),
                                param_start: t0,
                                param_end: t1,
                            }
                        }
                        None => {
                            let mut spec = geom_brep::EdgeCurveSpec::line_between(p0, p1);
                            spec.description = geom_brep::EdgeDescriptionSpec::Intersection {
                                s1: s_self,
                                s2: s_other,
                                witness,
                            };
                            spec
                        }
                    };
                    body.set_edge_curve(edge, spec, tol)?;
                }
                // Smooth: the surfaces under-determine the locus, so
                // the honest class is conventional (D2). A description
                // already drawn in one of the edge's two charts stays
                // verbatim; anything else — a citation whose partner
                // this split reassigned to the other product, or a
                // scaffold — is restated as an image in the section
                // chart, which the edge lies in by construction.
                // Carrier and interval travel verbatim (restated,
                // never rebuilt), as does a declared authority.
                //
                // No second-order ladder here (the boolean's smooth
                // arm runs one): a DETERMINATE smooth pair at the
                // section boundary would be the split plane tangent to
                // a curved wall, and the operand gate (planes and
                // cylinders) plus the classifier's typed refusal of
                // degenerate plane–conic crossings keep that edge from
                // ever reaching this arm — every smooth pair here is a
                // flush plane pair, whose exactly-zero jet is the
                // under-determined regime.
                Ok(geom_brep::DihedralClass::Smooth) => {
                    let coherent = existing.as_ref().is_some_and(|c| match *c.description() {
                        geom_brep::EdgeDescription::Chart(ref ch) => {
                            ch.surface == s_self || ch.surface == s_other
                        }
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
                            None => geom_brep::EdgeCurveSpec::line_between(p0, p1),
                        };
                        spec.description = geom_brep::EdgeDescriptionSpec::chart(s_self);
                        if let Some(geom_brep::EdgeAuthority::Declared(mc)) =
                            existing.as_ref().map(|c| c.authority())
                        {
                            spec.description = spec.description.declared_by(mc);
                        }
                        body.set_edge_curve(edge, spec, tol)?;
                    }
                }
                Err(diag) => return Err(SplitFinishError::DescribeEscalated { edge, diag }),
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

    let all: Vec<ShellKey> = body.get_solid(solid).ok_or_else(corrupt)?.shells.clone();
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
    }
    Ok(body)
}
