//! **The loft/sweep BODY assembly** (M6-3, executing the design
//! banked as M5 PR 9c item 6).
//!
//! The topology is EXTRUDE'S with different geometry (item 6(i)):
//! bottom cap from section 0, top cap from section k−1, one NURBS wall
//! per profile segment ([`crate::skin::LoftGeometry`]), wall–wall
//! seams as the walls' `u ∈ {0, 1}` boundary iso-curves, struts swept
//! per vertex. The three edge classes:
//!
//! - **Cap–wall rims** need no new GEOMETRY (item 6(ii)): the wall's
//!   `v = 0` / `v = 1` iso IS the placed sketch segment (degree
//!   elevation and knot refinement are exact), so the carrier stays
//!   the `Curve3::Line`/`Circle` it was minted as, verbatim, and
//!   certifies today. Its DESCRIPTION still moves: minted through the
//!   scaffolding door as `MappedCurve::PlacedSegment` (the cap plane
//!   is fitted through the rim, so it does not exist yet), it is
//!   re-stated as an image in the cap's own chart once the plane does
//!   ([`crate::swept::describe_face_rim_at_rest`] — D3's transience
//!   fence). No cap–wall dihedral is classified here: unlike extrude,
//!   loft does not upgrade these rims to `Intersection`.
//! - **Wall–wall seams** are the genuinely new class (item 6(iii)):
//!   an iso image of over the wall's boundary
//!   row (`geom_brep::boundary_iso_u` — a control-net copy, no
//!   arithmetic), certified through the metric residual
//!   `|C(t) − S(u, v(t))|` at the CERT schedule.
//! - **Struts** are minted as scaffolding lines and UPGRADED to the
//!   seam class once their walls' surface keys exist — the extrude
//!   Phase-6 rim-upgrade idiom, one class over.
//!
//! The final pass re-mints the whole body's pcurves
//! ([`topo::mint_pcurves`]): every wall boundary stores the exact
//! line-in-UV image ([`geom_brep::Pcurve`]'s iso lane) — tier-3 green
//! at rest is the BUILDER's acceptance, not a follow-up.
//!
//! # Orientation (the canonical stacking arm)
//!
//! Each slab's displacement must point ALONG the plane normals of both
//! sections it spans. That is a PER-SLAB statement and
//! [`fn@stacking_fold`] is where it is made and where its shape is
//! stated; a refusal names the pair it stopped at. It is a
//! conservative orientation check, not an embedding one: it refuses
//! some embedded lofts whose far section leans back across the stack,
//! and nothing at this door yet certifies that the walls do not cross
//! each other or the caps.
//!
//! The caps then orient exactly as extrude's (bottom reversed, top
//! forward) and every wall's chart normal `S_u × S_v` points out of
//! the material — the u direction follows the material-left profile
//! traversal and v the stacking, so `t̂ × v̂` is material-right —
//! giving `sense = true` on every wall, holes and concave arcs
//! included (unlike a cylinder chart, the skinned chart's normal
//! FOLLOWS the traversal; there is no unconditionally-radial frame to
//! fight). The bottom cap and the bottom lamina's rims read section
//! 0's normal and the top cap and the far rims read section `k − 1`'s,
//! both of which the fold has decided against their slab's
//! displacement; the fold's margins are not read again.
//!
//! # Scalar posture (C6)
//!
//! Structure — the skinned walls' knots, control bits, weights, and
//! the section chains — is selected at `f64` ([`fn@crate::skin`]'s
//! contract) and lifted exactly (`from_f64`) into the requested
//! scalar, so every lane builds the SAME body and the interval lane
//! encloses the very geometry the `f64` lane defines.

use core::fmt;
use std::sync::Arc;

use geom::Curve3;
use geom::{NurbsSurface, Surface};
use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec};
use geom_core::spline::SplineError;
use geom_core::{
    Affine3, Band, BandError, Decide, Indeterminate, Margin, Point3, Real, Sign, Tol, Vec3,
};
use profile::{SketchPlane, ValidatedProfile};
use topo::{
    Body, EdgeKey, EulerOpError, FaceKey, FaceSurface, MefSite, MevCreated, MevSite,
    PcurveMintError, ShellKey, SolidKey,
};

use crate::skin::{
    LoftGeometry, Section, SectionLoop, SkinError, skin_validated, sweep_places, validate_loft,
};
use crate::swept::{
    CapEnd, CapPlaneError, SweptSeg, cap_plane, cap_points, describe_face_rim_at_rest,
    face_surface_key, placed_segment_spec, swept_segments,
};

/// Everything [`loft_body`]/[`sweep_body`] built, keyed — the
/// [`crate::Extruded`] bundle one operation over.
#[derive(Debug)]
pub struct Lofted<T: Real> {
    /// The closed body: tiers 1–3 valid at rest, the builder's
    /// acceptance (callers re-validate per the workspace convention).
    /// Tier 3 does not test embedding, and neither does this door, so
    /// a body whose walls cross each other or a cap is not refused here.
    pub body: Body<T>,
    /// The solid.
    pub solid: SolidKey,
    /// Its one shell.
    pub shell: ShellKey,
    /// The top cap (section k−1's plane, outward along the stacking).
    pub top: FaceKey,
    /// The bottom cap (section 0's plane, outward against it).
    pub bottom: FaceKey,
    /// Wall faces `[loop][segment]`, loops in canonical order (outer
    /// first), segments in canonical order — `walls[l][j]` carries
    /// `LoftGeometry.walls[l][j]`.
    pub side_faces: Vec<Vec<FaceKey>>,
    /// Wall–wall seam edges `[loop][vertex]`: `seams[l][j]` is the
    /// strut at vertex `j` — wall `j`'s `u = 0` boundary iso (and
    /// wall `j − 1 mod n`'s `u = 1`).
    pub seam_edges: Vec<Vec<EdgeKey>>,
    /// **The v-parameter each input section sits at** —
    /// [`LoftGeometry::section_params`] carried out to the caller
    /// (`[0] = 0`, `[k − 1] = 1`) instead of dropped with the
    /// geometry, so "what spacing did the skin choose for my
    /// sections?" is answered by the result rather than re-derived by
    /// hand. Every wall agrees on it: the sections are planar
    /// cross-sections of the whole loft at these values.
    ///
    /// This is a re-read of what the kernel chose, not a measurement
    /// — the produced surface IS the definition (DESIGN Q8), so no
    /// residual pad accompanies it. It is [`crate::loft_parameters`]'
    /// answer for the body's sections, askable BEFORE the body is built
    /// (a [`sweep_body`]'s sections are its stations').
    pub section_params: Vec<f64>,
}

/// Typed refusal of the loft/sweep body assembly (closed enum, D4 ¶3).
#[derive(Debug)]
pub enum LoftError {
    /// The run's tolerance could not form a classification band.
    Band(BandError),
    /// The §10.3/§10.4 geometry construction refused (compatibility,
    /// degree, interpolation — every [`SkinError`] reason).
    Skin(SkinError),
    /// An Euler operator or certified attach refused mid-assembly
    /// (D4 ¶2 reports surface inside
    /// [`EulerOpError::Certification`]).
    Euler(EulerOpError),
    /// A cap plane could not be certified or oriented from its boundary
    /// points.
    CapPlane(CapPlaneError),
    /// The whole-body pcurve mint pass refused: a wall boundary's
    /// exact line-in-UV image failed its certification.
    Pcurve(PcurveMintError),
    /// The wall-boundary carrier extraction failed to re-wrap — a
    /// structurally corrupt skinned surface (unreachable from
    /// [`loft_geometry`](crate::loft_geometry) output; surfaced rather than swallowed).
    ///
    /// The payload is `geom_brep::boundary_iso_u`'s own refusal, which
    /// says WHICH structural invariant the extracted row broke; that
    /// door's `# Errors` section promises it is surfaced, and a
    /// discarded one would make a kernel-bug report say only that a
    /// kernel bug happened.
    SeamStructure {
        /// The iso-extraction refusal, carried rather than discarded.
        source: SplineError,
    },
    /// A section's loop/segment structure disagrees with the skinned
    /// geometry's, or two sections disagree with each other —
    /// unreachable when they all come from the same inputs; surfaced
    /// rather than swallowed.
    SectionStructure,
    /// One SLAB definitely stacks AGAINST its own base section's plane
    /// normal. The canonical assembly orients caps and walls by the
    /// forward stacking (module docs) and does not guess: the named
    /// pair is where the stack turns back on itself.
    ReversedStacking {
        /// The slab — the pair [`SlabPair`] names — and the first one
        /// in section order that is not definitely forward.
        slab: usize,
    },
    /// One SLAB's stacking displacement — the step of the outer loop's
    /// vertex centroid along the slab's base normal — is coincident
    /// with zero at tolerance. Coincident, sliver-thin and in-plane
    /// pairs land here, and so does a section tilted about an in-plane
    /// axis through its centroid so that it crosses its neighbour; at
    /// every scale down to exact coincidence this is the loft's one
    /// refusal for two adjacent sections that are not apart.
    DegenerateStacking {
        /// The slab — the pair [`SlabPair`] names.
        slab: usize,
    },
    /// One SLAB stacks along its base section's plane normal but not
    /// along its FAR section's: section `slab + 1`'s normal is
    /// definitely against the slab's displacement, or edge-on to it
    /// (Zero lands here, not in [`Self::DegenerateStacking`]: the base
    /// decide was already `Positive`, so the sections are apart).
    ///
    /// The check is conservative. It refuses every two-section loft
    /// whose rings fold between sections facing opposite ways along
    /// the stack, and it also refuses some embedded, correctly
    /// oriented lofts: a 10×10 square base under a 4×4 top turned 100°
    /// about `y` (its normal leaning back across the stack) is one. It
    /// retires with the fold when the loft door certifies embedding
    /// (`work/carve/self-overlapping-spines-build-and-validate.md`).
    FarSectionNotForward {
        /// The slab — the pair [`SlabPair`] names; the far section is
        /// its second.
        slab: usize,
    },
    /// The far half of a SLAB's stacking decide escalated: section
    /// `slab + 1`'s plane normal against the slab's displacement is
    /// too close to call (named predicate on the diagnostic). The
    /// base half was already `Positive`.
    FarStackingEscalated {
        /// The slab — the pair [`SlabPair`] names; the far section is
        /// its second.
        slab: usize,
        /// The predicate-layer escalation.
        source: Indeterminate,
    },
    /// One SLAB's stacking classification against its base section's
    /// normal escalated (named predicate on the diagnostic).
    StackingEscalated {
        /// The slab — the pair [`SlabPair`] names.
        slab: usize,
        /// The predicate-layer escalation.
        source: Indeterminate,
    },
}

/// **The two sections a slab spans**, and the one place this crate
/// spells that arithmetic: slab `k` is the pair `(k, k + 1)`, and
/// every stacking refusal names its pair through this.
struct SlabPair(usize);

impl SlabPair {
    /// The slab's far section.
    fn far(&self) -> usize {
        self.0 + 1
    }
}

impl fmt::Display for SlabPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "sections {} and {}", self.0, self.far())
    }
}

impl fmt::Display for LoftError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Band(e) => write!(f, "{e}"),
            Self::Skin(e) => write!(f, "{e}"),
            Self::Euler(e) => write!(f, "an Euler operation of the assembly refused: {e}"),
            Self::CapPlane(e) => write!(f, "{e}"),
            Self::Pcurve(e) => write!(f, "{e}"),
            Self::SeamStructure { source } => write!(
                f,
                "a wall's boundary curve failed to re-wrap (kernel bug, not an input \
                 fault): {source}"
            ),
            Self::SectionStructure => write!(
                f,
                "a section's loop or segment structure disagrees with the skinned \
                 geometry or with another section (kernel bug, not an input fault)"
            ),
            Self::ReversedStacking { slab } => write!(
                f,
                "loft {} stack AGAINST section {slab}'s plane normal, and a loft does not \
                 guess its direction. Recourse: reorder the sections so they stack \
                 forward; this is the first reversed pair, and a wholly reversed list \
                 lofts the same solid once reversed",
                SlabPair(*slab)
            ),
            Self::DegenerateStacking { slab } => write!(
                f,
                "loft {} are not apart along section {slab}'s normal at tolerance, so the \
                 loft has no direction. Recourse: move them apart",
                SlabPair(*slab)
            ),
            Self::FarSectionNotForward { slab } => write!(
                f,
                "loft section {far}'s plane does not face along the stack from section \
                 {slab}. Recourse: author section {far} on a plane facing along the stack \
                 with the others",
                far = SlabPair(*slab).far()
            ),
            Self::FarStackingEscalated { slab, source } => write!(
                f,
                "whether loft section {far}'s plane faces along the stack from section \
                 {slab} is too close to call: {source}",
                far = SlabPair(*slab).far()
            ),
            Self::StackingEscalated { slab, source } => write!(
                f,
                "whether loft {} stack forward is too close to call: {source}",
                SlabPair(*slab)
            ),
        }
    }
}

impl std::error::Error for LoftError {}

impl From<EulerOpError> for LoftError {
    fn from(e: EulerOpError) -> Self {
        Self::Euler(e.from_driver())
    }
}

/// One end section as a profile at `T`: **the canonical form
/// [`loft_geometry`](crate::loft_geometry) decided, lifted** ([`ValidatedProfile::lift_onto`])
/// — the same shape the rest of this assembly has, where the walls are
/// `f64` surfaces carried to `T` by `map_scalar`.
///
/// The section was validated once, at the geometry door, and that
/// verdict is what the walls were skinned from
/// ([`LoftGeometry::canonical`]). Deciding the canonical form again
/// here — loop roles, traversal sense, the start vertex, each
/// segment's classification, each declared joint's tangency — would
/// make the caps a SECOND canonicalization of the same data, agreeing
/// with the walls' by determinism rather than by construction; and at
/// the evaluation scalar it would decide over an exact embedding of
/// the data the first verdict was made on, which at a certified scalar
/// can only agree or escalate, never disagree. Reading the decided
/// form makes the caps the walls' own sections.
///
/// The gate a section that would not extrude meets is
/// [`loft_geometry`](crate::loft_geometry)'s, which refuses it
/// [`SkinError::SectionProfile`] before any of this runs.
fn end_profile<T: Real>(
    canonical: &ValidatedProfile<f64>,
    place: &Affine3<f64>,
) -> ValidatedProfile<T> {
    canonical
        .clone()
        .lift_onto(SketchPlane::new(place.map(T::from_f64)))
}

/// The world point of a sketch-plane point under a lifted placement.
fn world<T: Real>(place: &Affine3<T>, p: geom_core::Point2<T>) -> Point3<T> {
    place.transform_point(Point3::new(p.x, p.y, T::zero()))
}

/// The outer loop's world vertices at one section, in the traversal
/// order the assembly itself walks — [`swept_segments`]'s, over the
/// canonical form the walls were skinned from.
///
/// `None` when the section has no loops at all, which the caller
/// reports as [`LoftError::SectionStructure`]: a section that reached
/// here without an outer loop is a corrupt [`LoftGeometry`], not an
/// input fault.
fn outer_world<T: Real>(
    canonical: &ValidatedProfile<f64>,
    place: &Affine3<f64>,
) -> Option<Vec<Point3<T>>> {
    let profile: ValidatedProfile<T> = end_profile(canonical, place);
    let outer = profile.loops().first()?;
    let placed: Affine3<T> = place.map(T::from_f64);
    Some(
        swept_segments(outer, false)
            .iter()
            .map(|s| world(&placed, s.a))
            .collect(),
    )
}

/// **The stacking fold** — the loft's one stacking statement.
///
/// Slab `k` is the pair `(k, k + 1)`, and its displacement `d_k` is
/// the mean displacement of the outer loop's vertices between the two
/// sections. The fold decides, slab after slab and under
/// `loft_stacking`, `n_k · d_k` and then `n_{k+1} · d_k` — both of the
/// slab's own section normals — **refusing at the first verdict that is
/// not `Positive`**. No minimum is formed: `Positive` is monotone in
/// the margin, and stopping at the first non-`Positive` verdict keeps
/// an ambiguous one from being answered for by a definite one later in
/// the list.
///
/// The margin is a sum of per-vertex differences over a full zip of
/// two equal-length loops, so it is the displacement of the vertex
/// CENTROID: the by-index pairing carries no information and rotating
/// one section's traversal cannot change the verdict.
///
/// The fold reads nothing but the two sections of the slab it is
/// deciding: their authored placements and their canonical loops.
///
/// **It is the loft's one decision about whether two adjacent sections
/// are apart.** It runs before the skin ([`fn@build`]), so a sliver
/// slab and an exactly coincident pair are both refused here, as
/// [`LoftError::DegenerateStacking`], and the skin only ever
/// parameterizes sections this fold has found definitely apart.
///
/// # Preconditions, and why its guards are dead through `loft_body`
///
/// A `places`/`canonical` length disagreement, a section count below
/// two, a section with no loops, and two sections whose outer loops
/// differ in vertex count are this function's own preconditions, kept
/// as refusals rather than assumptions. None is reachable through
/// [`loft_body`] today: `validate_loft` refuses the counts and
/// mismatched sections first (`SkinError::SectionShapeMismatch`), and
/// a loopless section never leaves profile validation.
fn stacking_fold<T: Decide>(
    places: &[Affine3<f64>],
    canonical: &[ValidatedProfile<f64>],
    band: Band,
) -> Result<(), LoftError> {
    if places.len() < 2 || places.len() != canonical.len() {
        return Err(LoftError::SectionStructure);
    }
    let mut outers = places
        .iter()
        .zip(canonical)
        .map(|(place, c)| outer_world::<T>(c, place).ok_or(LoftError::SectionStructure));
    let mut base: Vec<Point3<T>> = outers.next().ok_or(LoftError::SectionStructure)??;
    // `outers` now yields section `slab + 1` beside the slab's base and
    // far placements.
    let slabs = places.iter().zip(&places[1..]);
    for (slab, (next, (base_place, far_place))) in outers.zip(slabs).enumerate() {
        let next: Vec<Point3<T>> = next?;
        if base.is_empty() || next.len() != base.len() {
            return Err(LoftError::SectionStructure);
        }
        let mut d = Vec3::new(T::zero(), T::zero(), T::zero());
        for (qt, qb) in next.iter().zip(&base) {
            d = d + (*qt - *qb);
        }
        #[allow(clippy::cast_precision_loss)]
        let count = T::from_f64(next.len() as f64);
        let facing = |place: &Affine3<f64>| {
            let margin = d.dot(place.map(T::from_f64).linear.c2) / count;
            geom_core::k_stats::decide("loft_stacking", Margin::of(margin), band)
        };
        match facing(base_place).map_err(|source| LoftError::StackingEscalated { slab, source })? {
            Sign::Positive => {}
            Sign::Zero => return Err(LoftError::DegenerateStacking { slab }),
            Sign::Negative => return Err(LoftError::ReversedStacking { slab }),
        }
        match facing(far_place)
            .map_err(|source| LoftError::FarStackingEscalated { slab, source })?
        {
            Sign::Positive => {}
            Sign::Zero | Sign::Negative => return Err(LoftError::FarSectionNotForward { slab }),
        }
        base = next;
    }
    Ok(())
}

/// The shared engine of [`loft_body`] and [`sweep_body`]: validate the
/// sections, decide the stacking fold, skin, assemble — in that order.
///
/// The fold runs BEFORE the skin because it is the one decision about
/// whether two adjacent sections are apart, banded under the run's
/// tolerance; a pair it accepts is definitely apart, and a pair it
/// refuses (down to exact coincidence) never reaches the skin's
/// parameterization.
fn build<T: Decide + topo::AtRestPolicy>(
    sections: &[Section<impl SectionLoop>],
    places: &[Affine3<f64>],
    v_degree: usize,
    tol: Tol,
) -> Result<Lofted<T>, LoftError> {
    let canonical = validate_loft(sections, places, v_degree, tol).map_err(LoftError::Skin)?;
    stacking_fold::<T>(
        places,
        &canonical,
        Band::linear(tol).map_err(LoftError::Band)?,
    )?;
    let geometry = skin_validated(canonical, places, v_degree).map_err(LoftError::Skin)?;
    assemble(places, &geometry, tol)
}

/// Assembles the loft BODY from its skinned geometry (module docs),
/// over sections the stacking fold has already accepted.
///
/// # Errors
///
/// [`LoftError`] — every door named on the enum.
fn assemble<T: Decide + topo::AtRestPolicy>(
    places: &[Affine3<f64>],
    geometry: &LoftGeometry,
    tol: Tol,
) -> Result<Lofted<T>, LoftError> {
    let band = Band::linear(tol).map_err(LoftError::Band)?;
    let (Some(can_bottom), Some(can_top), Some(place_bottom), Some(place_top)) = (
        geometry.canonical.first(),
        geometry.canonical.last(),
        places.first(),
        places.last(),
    ) else {
        return Err(LoftError::SectionStructure);
    };
    let bottom_profile: ValidatedProfile<T> = end_profile(can_bottom, place_bottom);
    let top_profile: ValidatedProfile<T> = end_profile(can_top, place_top);
    let bplace: Affine3<T> = place_bottom.map(T::from_f64);
    let tplace: Affine3<T> = place_top.map(T::from_f64);
    let n_bottom = bplace.linear.c2;
    let n_top = tplace.linear.c2;

    // ---- Traversals and world points, both ends, per loop. ----
    let bloops: Vec<Vec<SweptSeg<T>>> = bottom_profile
        .loops()
        .iter()
        .map(|lp| swept_segments(lp, false))
        .collect();
    let tloops: Vec<Vec<SweptSeg<T>>> = top_profile
        .loops()
        .iter()
        .map(|lp| swept_segments(lp, false))
        .collect();
    if places.len() < 2
        || places.len() != geometry.canonical.len()
        || bloops.len() != tloops.len()
        || bloops.len() != geometry.walls.len()
        || bloops
            .iter()
            .zip(&tloops)
            .zip(&geometry.walls)
            .any(|((b, t), w)| b.len() != t.len() || b.len() != w.len())
    {
        return Err(LoftError::SectionStructure);
    }
    let bq: Vec<Vec<Point3<T>>> = bloops
        .iter()
        .map(|segs| segs.iter().map(|s| world(&bplace, s.a)).collect())
        .collect();
    let tq: Vec<Vec<Point3<T>>> = tloops
        .iter()
        .map(|segs| segs.iter().map(|s| world(&tplace, s.a)).collect())
        .collect();

    // ---- Lifted walls, kept once: face surfaces AND seam carriers
    // read the same lifted structure (D9 — one lift, shared bits). ----
    let walls_t: Vec<Vec<Arc<NurbsSurface<T>>>> = geometry
        .walls
        .iter()
        .map(|loop_walls| {
            loop_walls
                .iter()
                .map(|w| Arc::new(w.map_scalar(T::from_f64)))
                .collect()
        })
        .collect();

    // ---- Phase 1: bottom lamina (the extrude shape: the seed face's
    // chain is minted at the BOTTOM vertices and survives as the TOP
    // cap after the sweep; the mef'd close face is the bottom cap,
    // its loop reversed so the outward normal opposes stacking). ----
    let outer = &bloops[0];
    let qs = &bq[0];
    let n = outer.len();
    // One surgery scope for the whole assembly: the tier-1
    // postcondition is this door's, paid once over the finished body
    // (`topo::surgery`), and the tier-2 check below subsumes it.
    let mut built = Body::<T>::new();
    let mut body = built.begin_surgery();
    let bottom_plane = cap_plane(
        &cap_points(outer, qs, bplace),
        bplace,
        false,
        CapEnd::Start,
        band,
    )
    .map_err(LoftError::CapPlane)?;
    let bottom_cap = FaceSurface::New {
        surface: bottom_plane,
        sense: true,
    };
    // A one-segment loop (D1's full turn) is swept whole in phases 1–2,
    // far (top) rim first (`full_turn`); phases 3–4 skip it.
    let ends = |li: usize| (bq[li][0], tq[li][0]);
    let full_turn = |body: &mut Body<T>, li: usize, r#loop, near_cap| {
        let (near, far) = ends(li);
        let turn = crate::swept::build_full_turn(
            body,
            r#loop,
            near,
            placed_segment_spec(&tloops[li][0], tplace, n_top, far, far, tol),
            FaceSurface::New {
                surface: Surface::Nurbs(Arc::clone(&walls_t[li][0])),
                sense: true,
            },
            EdgeCurveSpec::line_between(far, near),
            placed_segment_spec(&bloops[li][0], bplace, n_bottom, near, near, tol),
            near_cap,
            tol,
        )?;
        Ok::<_, LoftError>(turn)
    };
    let mut early: Vec<Option<crate::swept::FullTurn>> = (0..bloops.len()).map(|_| None).collect();
    let mut bases: Vec<Vec<topo::HalfEdgeKey>> = Vec::with_capacity(bloops.len());
    let (seed, bottom_face, anchor) = if profile::is_full_turn(outer) {
        let seed = body.mvfs(tq[0][0], true)?;
        let turn = full_turn(&mut body, 0, seed.r#loop, bottom_cap)?;
        bases.push(vec![turn.near_in_wall]);
        let (bottom_face, anchor) = (turn.near_face, turn.far_kept);
        early[0] = Some(turn);
        (seed, bottom_face, anchor)
    } else {
        let seed = body.mvfs(qs[0], true)?;
        let mut hes = Vec::with_capacity(n);
        let first = body.mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            qs[1],
            placed_segment_spec(&outer[0], bplace, n_bottom, qs[0], qs[1], tol),
            tol,
        )?;
        hes.push(first.he_plus);
        let mut prev = first;
        for j in 2..n {
            let m = body.mev(
                MevSite::Fan {
                    he1: prev.he_minus,
                    he2: prev.he_minus,
                },
                qs[j],
                placed_segment_spec(&outer[j - 1], bplace, n_bottom, qs[j - 1], qs[j], tol),
                tol,
            )?;
            hes.push(m.he_plus);
            prev = m;
        }
        let close = body.mef(
            MefSite::Chords {
                he1: prev.he_minus,
                he2: first.he_plus,
            },
            placed_segment_spec(&outer[n - 1], bplace, n_bottom, qs[n - 1], qs[0], tol),
            bottom_cap,
            tol,
        )?;
        hes.push(close.he_plus);
        let anchor = hes[0];
        bases.push(hes);
        (seed, close.face, anchor)
    };
    let top_face = seed.face;
    let bottom_surface = face_surface_key(&body, bottom_face);

    // ---- Phase 2: holes (rings in the seed face + kfmrh into the
    // bottom cap) — extrude's phase verbatim, loft rim specs. ----
    for (li, segs) in bloops.iter().enumerate().skip(1) {
        let hq = &bq[li];
        let m = segs.len();
        if profile::is_full_turn(segs) {
            let (turn, ()) = crate::swept::full_turn_hole(
                &mut body,
                anchor,
                tq[li][0],
                bottom_face,
                tol,
                |b, ring, disc| Ok::<_, LoftError>((full_turn(b, li, ring, disc)?, ())),
            )?;
            bases.push(vec![turn.near_in_wall]);
            early[li] = Some(turn);
            continue;
        }
        let (ring, _) = crate::swept::plant_hole_ring(&mut body, anchor, hq[0], tol)?;
        let mut hole_hes = Vec::with_capacity(m);
        let first = body.mev(
            MevSite::Lone { r#loop: ring },
            hq[1],
            placed_segment_spec(&segs[0], bplace, n_bottom, hq[0], hq[1], tol),
            tol,
        )?;
        hole_hes.push(first.he_plus);
        let mut prev = first;
        for j in 2..m {
            let mv = body.mev(
                MevSite::Fan {
                    he1: prev.he_minus,
                    he2: prev.he_minus,
                },
                hq[j],
                placed_segment_spec(&segs[j - 1], bplace, n_bottom, hq[j - 1], hq[j], tol),
                tol,
            )?;
            hole_hes.push(mv.he_plus);
            prev = mv;
        }
        let close = body.mef(
            MefSite::Chords {
                he1: prev.he_minus,
                he2: first.he_plus,
            },
            placed_segment_spec(&segs[m - 1], bplace, n_bottom, hq[m - 1], hq[0], tol),
            crate::swept::transient_disc(bottom_surface),
            tol,
        )?;
        hole_hes.push(close.he_plus);
        body.kfmrh(bottom_face, close.face)?;
        bases.push(hole_hes);
    }

    // ---- Phases 3–4: sweep struts and close the wall quads, per
    // loop. Struts are SCAFFOLDING lines here (mev_line) and upgrade
    // to the seam class in phase 6, once their walls' keys exist. ----
    let mut side_faces: Vec<Vec<FaceKey>> = Vec::with_capacity(bloops.len());
    let mut seam_edges: Vec<Vec<EdgeKey>> = Vec::with_capacity(bloops.len());
    let mut top_rims: Vec<Vec<EdgeKey>> = Vec::with_capacity(bloops.len());
    for (li, base) in bases.iter().enumerate() {
        if let Some(turn) = early[li].take() {
            side_faces.push(vec![turn.wall]);
            seam_edges.push(vec![turn.strut]);
            top_rims.push(vec![turn.far]);
            continue;
        }
        let tsegs = &tloops[li];
        let n = base.len();
        let mut struts: Vec<MevCreated> = Vec::with_capacity(n);
        for j in 0..n {
            let m = body.mev_line(
                MevSite::Fan {
                    he1: base[j],
                    he2: base[j],
                },
                tq[li][j],
                tol,
            )?;
            struts.push(m);
        }
        let mut faces: Vec<FaceKey> = Vec::with_capacity(n);
        let mut rims: Vec<EdgeKey> = Vec::with_capacity(n);
        let mut first_top = None;
        for j in 0..n {
            let he2 = match (j + 1 < n, first_top) {
                (true, _) => struts[j + 1].he_minus,
                (false, Some(top)) => top,
                (false, None) => struts[j].he_minus,
            };
            let top_q_from = tq[li][j];
            let top_q_to = tq[li][(j + 1) % n];
            let mef = body.mef(
                MefSite::Chords {
                    he1: struts[j].he_minus,
                    he2,
                },
                placed_segment_spec(&tsegs[j], tplace, n_top, top_q_from, top_q_to, tol),
                // The skinned chart's normal points out of the
                // material (module docs).
                FaceSurface::New {
                    surface: Surface::Nurbs(Arc::clone(&walls_t[li][j])),
                    sense: true,
                },
                tol,
            )?;
            if j == 0 {
                first_top = Some(mef.he_plus);
            }
            faces.push(mef.face);
            rims.push(mef.edge);
        }
        side_faces.push(faces);
        seam_edges.push(struts.iter().map(|s| s.edge).collect());
        top_rims.push(rims);
    }

    // ---- Phase 5: the swept seed face survives as the top cap. ----
    let top_plane = cap_plane(
        &cap_points(&tloops[0], &tq[0], tplace),
        tplace,
        false,
        CapEnd::End,
        band,
    )
    .map_err(LoftError::CapPlane)?;
    body.set_face_surface(
        top_face,
        FaceSurface::New {
            surface: top_plane,
            sense: true,
        },
    )?;

    // Both cap planes exist now, so both rims are at REST in them and
    // stop leaning on the scaffolding door they had to be minted
    // through (D3's transience fence — a cap's plane is fitted THROUGH
    // its own rim, so the rim cannot name it at mint time).
    describe_face_rim_at_rest(&mut body, bottom_face, tol)?;
    describe_face_rim_at_rest(&mut body, top_face, tol)?;

    // ---- Phase 6: strut upgrades to the seam class — the wall keys
    // now exist, so each strut re-describes as wall j's `u = 0`
    // boundary iso through the certified setter (D9 — loops in
    // canonical order, vertices in swept order). ----
    for (li, seams) in seam_edges.iter().enumerate() {
        let n = seams.len();
        for j in 0..n {
            let wall_key = face_surface_key(&body, side_faces[li][j]);
            let carrier = geom_brep::boundary_iso_u(walls_t[li][j].as_ref(), false)
                .map_err(|source| LoftError::SeamStructure { source })?;
            if n == 1 && profile::is_full_turn(&bloops[li]) {
                // A one-segment loop's strut is its wall's wrap edge in
                // `u` (D1: a closed spline net's boundary column wraps
                // `u`), run top to bottom as the turn laid it.
                let carrier = geom_brep::reversed_column(&carrier)
                    .map_err(|source| LoftError::SeamStructure { source })?;
                let spec = EdgeCurveSpec {
                    description: EdgeDescriptionSpec::wrap_iso(
                        wall_key,
                        T::zero(),
                        T::one(),
                        T::zero(),
                        T::zero(),
                        T::one(),
                    ),
                    carrier: Curve3::Nurbs(Arc::new(carrier)),
                    param_start: T::zero(),
                    param_end: T::one(),
                };
                body.set_edge_curve(seams[j], spec, tol)?;
                continue;
            }
            let spec = EdgeCurveSpec {
                description: EdgeDescriptionSpec::iso(
                    wall_key,
                    T::zero(),
                    T::zero(),
                    T::one(),
                    T::zero(),
                    T::one(),
                ),
                carrier: Curve3::Nurbs(Arc::new(carrier)),
                param_start: T::zero(),
                param_end: T::one(),
            };
            body.set_edge_curve(seams[j], spec, tol)?;
        }
    }

    // ---- Phase 7: whole-body pcurve mint (spec §1's final pass) —
    // every wall boundary stores its exact line-in-UV image. ----
    topo::mint_pcurves(&mut body, tol).map_err(LoftError::Pcurve)?;

    body.close_already_checked();
    #[cfg(debug_assertions)]
    debug_assert_eq!(
        topo::validate_closed(&built),
        Ok(()),
        "loft postcondition: result is not tier-2 valid (kernel bug)",
    );

    Ok(Lofted {
        body: built,
        solid: seed.solid,
        shell: seed.shell,
        top: top_face,
        bottom: bottom_face,
        side_faces,
        seam_edges,
        section_params: geometry.section_params.clone(),
    })
}

/// **The loft body** (M6-PLAN unit 3, spec §1): skins
/// [`loft_geometry`](crate::loft_geometry) and assembles the closed solid around it.
///
/// `sections[i][l][j]` is section `i`, loop `l`, segment `j` in sketch
/// coordinates; `places[i]` its rigid placement; `v_degree` the
/// skinning degree in the section direction. Structure is `f64`
/// (C6); the produced body is at `T`, lifted exactly.
///
/// # Correspondence — the vertex order you wrote
///
/// Sections are paired **by index over the canonical loops**, and the
/// canonical form keeps each loop's AUTHORED start
/// ([`profile::Profile::validate`] normalizes only the traversal sense),
/// so segment `j` of every section is counted from the vertex you wrote
/// first ([`loft_geometry`](crate::loft_geometry), "The correspondence is the author's").
/// **A section rotated relative to its neighbour rolls the body by the
/// angle you authored**: the turning-orientation suite's authored-roll
/// row lofts a square onto the same square rotated by `theta` about its
/// own centre, each written from the image of the other's start, and
/// the body rolls by `theta`. To change the twist, start the section at
/// a different vertex.
///
/// The sections sit at [`crate::loft_parameters`]' v-parameters, a
/// function of the section set: a section spelled from a different
/// starting vertex, or rolled about its own normal by one of its own
/// symmetries, builds the same body.
///
/// `places[i]` is the caller's. For the plane normal to a curve at a
/// point, `geom_core::linalg::frame::path_start_frame(point, tangent,
/// tol)` is the door that hands one out, and a different roll is a
/// rotation composed about the tangent onto it.
///
/// # Errors
///
/// [`LoftError`] — every door named on the enum.
pub fn loft_body<T: Decide + topo::AtRestPolicy>(
    sections: &[Section<impl SectionLoop>],
    places: &[Affine3<f64>],
    v_degree: usize,
    tol: Tol,
) -> Result<Lofted<T>, LoftError> {
    build(sections, places, v_degree, tol)
}

/// **The path-swept body** (§10.4 as a solid): places rigid copies of
/// `profile` along `path` ([`crate::sweep_geometry`]'s frame — same
/// machinery, no fork) and assembles the loft of those sections.
///
/// # Correspondence
///
/// [`loft_body`]'s paragraph of that name applies, and lands softly
/// here for a reason worth knowing: every section is the SAME profile,
/// so every section canonicalizes identically and the index pairing is
/// the identity whatever the profile's vertex order was. What the
/// authored start still decides is which wall of the
/// built body is which — the segment order the returned
/// [`Lofted::side_faces`] is keyed in. The body's roll comes from the
/// path frame ([`sweep_places`]), not from the sections. The stations
/// sit at the loft's chord-length parameters ([`crate::loft_parameters`]),
/// which neither the spelling nor the roll of the start frame moves.
///
/// # The starting frame
///
/// `place` — the frame every station is carried from — is the
/// caller's, and `geom_core::linalg::frame::path_start_frame(path
/// start, start tangent, tol)` is where a caller gets it: the plane
/// through the start point whose local +Z is the start tangent, its
/// roll off a reference ladder decided under the tolerance band, with
/// a typed refusal when no rung decides. A caller wanting a different
/// roll composes a rotation about the tangent onto that frame; there
/// is no second door.
///
/// # Errors
///
/// [`LoftError`] — every door named on the enum, with
/// [`SkinError::PathTangentReversal`] arriving through
/// [`LoftError::Skin`].
pub fn sweep_body<T: Decide + topo::AtRestPolicy>(
    profile: &[impl SectionLoop],
    place: Affine3<f64>,
    path: &geom::NurbsCurve3<f64>,
    stations: usize,
    v_degree: usize,
    tol: Tol,
) -> Result<Lofted<T>, LoftError> {
    let places = sweep_places(place, path, stations).map_err(LoftError::Skin)?;
    let sections: Vec<Section<_>> = core::iter::repeat_n(profile.to_vec(), places.len()).collect();
    build(&sections, &places, v_degree, tol)
}
