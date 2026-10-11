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
//!   scaffolding door as `MappedSource::PlacedSegment` (the cap plane
//!   is fitted through the rim, so it does not exist yet), it is
//!   re-stated as an image in the cap's own chart once the plane does
//!   ([`crate::swept::describe_face_rim_at_rest`] — D3's transience
//!   fence). No cap–wall dihedral is classified here: unlike extrude,
//!   loft does not upgrade these rims to `Intersection`, because the
//!   plane × NURBS certificate refuses many of their at-rest rows
//!   (`work/ssiedge/plane-nurbs-certificate-refuses-at-rest-rows-of-lofts-and-sweeps.md`).
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
//! The sections and placements arrive in the body's scalar `T`.
//! Structure — each section's canonical form, the skin's degrees,
//! multiplicities and parameter order — is selected once, at `f64`, on
//! their value channel, so every lane builds the SAME body. The values
//! then depend on which side of the projective partition `T` is on
//! ([`fn@build`]):
//!
//! - at `f64`, `Probe`, and `Dual` and `Sym` over them, the sections
//!   are validated at `T` along the `f64` record and the walls are
//!   skinned in `T` ([`fn@crate::skin`]), so a wall, a cap and the
//!   stacking fold's margins are the functions of the sections'
//!   variables the sections are;
//! - at `Interval`, and the wrappers over it, the `f64` skin of the
//!   sections' value channel is the definition, and the body encloses
//!   it: the walls' control points, knots and weights are lifted
//!   through `from_computed`, a seam column both walls end on lifted
//!   once and shared, and the placements the skin was selected on are
//!   lifted the same way, so the caps (the `f64` canonical form lifted
//!   onto them), the rims and the fold read the one loft the body
//!   encloses. A section or placement wider than a point has no `f64`
//!   skin and refuses
//!   (`work/inlane/the-skin-at-interval-encloses-the-nominal-skin.md`).

use core::fmt;
use std::sync::Arc;

use geom::Curve3;
use geom::{NurbsSurface, Surface};
use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec};
use geom_core::spline::{ProjectiveScalar, ProjectiveSplit, SplineError};
use geom_core::{
    Affine3, Band, BandError, Decide, Indeterminate, Margin, Point3, Real, Sign, Tol, Vec3,
};
use profile::{CanonicalStructure, SketchPlane, ValidatedProfile};
use topo::{
    Body, EdgeKey, EulerOpError, FaceKey, FaceSurface, MefSite, MevCreated, MevSite,
    PcurveMintError, ShellKey, SolidKey,
};

use crate::skin::{
    Section, SectionLoop, SkinError, check_counts, check_shapes, skin_validated, sweep_places,
    validate_guided, validate_loft,
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
    /// Section `section`'s canonical form holds a value with no one
    /// `f64` (poison, or an enclosure wider than a point), so the loft
    /// has no `f64` section to select its structure on (C6), and at an
    /// enclosure scalar no `f64` skin to enclose
    /// (`work/inlane/the-skin-at-interval-encloses-the-nominal-skin.md`).
    SectionNotAPoint {
        /// The section.
        section: usize,
    },
    /// Section `section`'s placement has an entry with no one `f64`
    /// value (poison, or an enclosure wider than a point), so there is
    /// no placement for the loft's structure to be selected on (C6).
    PlacementNotAPoint {
        /// The section whose placement it is.
        section: usize,
    },
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
            Self::SectionNotAPoint { section } => write!(
                f,
                "section {section} is not a single point in every value, so the loft has no \
                 section to select its structure on. Recourse: loft the section at a point; a \
                 section over a range of values is not lofted yet"
            ),
            Self::PlacementNotAPoint { section } => write!(
                f,
                "section {section}'s placement is not a single point, so the loft has no \
                 placement to select its structure on. Recourse: place the section at a \
                 point; a placement over a range is not lofted yet"
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

/// One loop's skinned walls, lifted from `f64` to the enclosure scalar
/// `T` ([`LoftBuild::enclosure`]): the `f64` skin is the definition
/// there, so its net, knots and weights enter as the computed values
/// they are ([`Real::from_computed`]), and wall `j`'s `u = 1` column is
/// wall `j + 1 mod n`'s `u = 0` column (the seam both walls end on), so
/// a column the two nets hold bit for bit is lifted once and shared.
///
/// # Errors
///
/// [`LoftError::SeamStructure`] if a rebuilt net is refused, which a
/// net of the counts it was lifted from cannot be.
fn lift_loop_walls<T: geom_core::spline::SpanLocate>(
    walls: &[Arc<NurbsSurface<f64>>],
) -> Result<Vec<Arc<NurbsSurface<T>>>, LoftError> {
    let mut nets: Vec<Vec<Point3<T>>> = walls
        .iter()
        .map(|w| {
            w.control()
                .iter()
                .map(|p| p.map(T::from_computed))
                .collect()
        })
        .collect();
    let bits = |p: &Point3<f64>| [p.x.to_bits(), p.y.to_bits(), p.z.to_bits()];
    let n = walls.len();
    for j in 0..n {
        let k = (j + 1) % n;
        let ((nu, nv), (_, nv_k)) = (walls[j].control_counts(), walls[k].control_counts());
        let last = (nu - 1) * nv;
        let shared = nv == nv_k
            && (0..nv)
                .all(|iv| bits(&walls[j].control()[last + iv]) == bits(&walls[k].control()[iv]));
        if shared {
            let column: Vec<Point3<T>> = nets[j][last..last + nv].to_vec();
            nets[k][..nv].copy_from_slice(&column);
        }
    }
    walls
        .iter()
        .zip(nets)
        .map(|(w, net)| {
            NurbsSurface::new(
                w.knot_values_u().lift(T::from_computed),
                w.knot_values_v().lift(T::from_computed),
                net,
                w.weights().iter().map(|&x| T::from_computed(x)).collect(),
            )
            .map(Arc::new)
            .map_err(|source| LoftError::SeamStructure { source })
        })
        .collect()
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
    profile: &ValidatedProfile<T>,
    place: Affine3<T>,
) -> Option<Vec<Point3<T>>> {
    let outer = profile.loops().first()?;
    Some(
        swept_segments(outer, false)
            .iter()
            .map(|s| world(&place, s.a))
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
    places: &[Affine3<T>],
    canonical: &[ValidatedProfile<T>],
    band: Band,
) -> Result<(), LoftError> {
    if places.len() < 2 || places.len() != canonical.len() {
        return Err(LoftError::SectionStructure);
    }
    let mut outers = places
        .iter()
        .zip(canonical)
        .map(|(place, c)| outer_world(c, *place).ok_or(LoftError::SectionStructure));
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
        let facing = |place: &Affine3<T>| {
            let margin = d.dot(place.linear.c2) / count;
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
/// The sections' structure is decided once, on their value channel at
/// `f64` ([`validate_loft`]): the canonical form, its record, and every
/// count the skin checks. What happens at `T` then depends on which
/// side of the projective partition `T` is on
/// ([`SpanLocate::split_projective`](geom_core::spline::SpanLocate::split_projective)), which [`LoftBuild`]'s two halves
/// state.
///
/// The fold runs BEFORE the skin because it is the one decision about
/// whether two adjacent sections are apart, banded under the run's
/// tolerance; a pair it accepts is definitely apart, and a pair it
/// refuses (down to exact coincidence) never reaches the skin's
/// parameterization.
fn build<T: Decide + topo::AtRestPolicy, L: SectionLoop<T>>(
    sections: &[Section<L>],
    places: &[Affine3<T>],
    channel_places: &[Affine3<f64>],
    v_degree: usize,
    tol: Tol,
) -> Result<Lofted<T>, LoftError> {
    if let Some(section) = channel_places
        .iter()
        .position(|p| !p.components().iter().all(|c| c.is_finite()))
    {
        return Err(LoftError::PlacementNotAPoint { section });
    }
    let channel_sections: Vec<Section<L::Channel>> = sections
        .iter()
        .map(|s| s.iter().map(SectionLoop::channel).collect())
        .collect();
    let (canonical, structures) =
        validate_loft(&channel_sections, channel_places, v_degree, tol).map_err(LoftError::Skin)?;
    T::split_projective(LoftBuild {
        at_t: SectionsAtT::Guided {
            sections,
            structures,
        },
        places,
        channel_places,
        canonical,
        v_degree,
        tol,
    })
}

/// Where a loft's sections at `T` come from.
enum SectionsAtT<'a, T: Real, L> {
    /// Loops, validated at `T` along their `f64` records
    /// ([`validate_guided`]).
    Guided {
        sections: &'a [Section<L>],
        structures: Vec<CanonicalStructure>,
    },
    /// Canonical forms the caller validated at `T`
    /// ([`loft_profiles`]).
    Validated(&'a [ValidatedProfile<T>]),
}

/// [`build`]'s two halves, after the sections' structure is decided.
struct LoftBuild<'a, T: Real, L> {
    at_t: SectionsAtT<'a, T, L>,
    places: &'a [Affine3<T>],
    channel_places: &'a [Affine3<f64>],
    canonical: Vec<ValidatedProfile<f64>>,
    v_degree: usize,
    tol: Tol,
}

impl<T: Decide + topo::AtRestPolicy, L: SectionLoop<T>> ProjectiveSplit<T> for LoftBuild<'_, T, L> {
    type Out = Result<Lofted<T>, LoftError>;

    /// **The loft in `T`.** Each section is validated at `T` along its
    /// `f64` record ([`validate_guided`]), so its canonical form is the
    /// one decided and its values are the caller's; the fold, the skin
    /// ([`skin_validated`]: the section curves, their compatibility,
    /// the parameters and the solve) and the caps all read those, so a
    /// section or placement value that is a function of the variables
    /// gives walls and caps that are functions of them.
    fn projective(self) -> Self::Out
    where
        T: ProjectiveScalar,
    {
        let band = Band::linear(self.tol).map_err(LoftError::Band)?;
        let canonical = match self.at_t {
            SectionsAtT::Guided {
                sections,
                structures,
            } => validate_guided(sections, self.places, &structures, self.tol)
                .map_err(LoftError::Skin)?,
            SectionsAtT::Validated(sections) => sections.to_vec(),
        };
        stacking_fold(self.places, &canonical, band)?;
        let geometry =
            skin_validated(canonical, self.places, self.v_degree).map_err(LoftError::Skin)?;
        let params = geometry
            .section_params
            .iter()
            .map(|u| u.value_channel())
            .collect();
        assemble(
            self.places,
            &geometry.walls,
            &geometry.canonical,
            params,
            self.tol,
        )
    }

    /// **The loft at an enclosure scalar**: the `f64` loft its sections'
    /// value channel describes, enclosed. The knot algebra the skin
    /// runs does not typecheck at an enclosure scalar
    /// ([`ProjectiveScalar`]), and the averaged knots a solve there
    /// would form are no single points, which a carrier's knots must be
    /// ([`geom_core::spline::Knots::new`]). So the skin is the `f64`
    /// one, the walls are lifted ([`lift_loop_walls`]), and the caps
    /// and the fold read the `f64` canonical form lifted onto the
    /// placements the skin was selected on, themselves lifted
    /// ([`ValidatedProfile::lift_onto`]). A section whose values are
    /// wider than a point refuses at the `f64` validation, whose value
    /// channel is then NaN, and a placement wider than a point refuses
    /// in [`fn@build`] ([`LoftError::PlacementNotAPoint`]).
    fn enclosure(self) -> Self::Out {
        let band = Band::linear(self.tol).map_err(LoftError::Band)?;
        // The placements the `f64` skin was selected on, entered as the
        // computed values they are: every reader of this half — the
        // fold, the caps, the rims — reads the one loft it encloses.
        let places: Vec<Affine3<T>> = self
            .channel_places
            .iter()
            .map(|p| p.map(T::from_computed))
            .collect();
        let lifted: Vec<ValidatedProfile<T>> = self
            .canonical
            .iter()
            .zip(&places)
            .map(|(c, place)| c.clone().lift_onto(SketchPlane::new(*place)))
            .collect();
        stacking_fold(&places, &lifted, band)?;
        let geometry = skin_validated(self.canonical, self.channel_places, self.v_degree)
            .map_err(LoftError::Skin)?;
        let walls = geometry
            .walls
            .iter()
            .map(|w| lift_loop_walls(w))
            .collect::<Result<Vec<_>, _>>()?;
        assemble(&places, &walls, &lifted, geometry.section_params, self.tol)
    }
}

/// Assembles the loft BODY from its skinned geometry (module docs),
/// over sections the stacking fold has already accepted.
///
/// # Errors
///
/// [`LoftError`] — every door named on the enum.
fn assemble<T: Decide + topo::AtRestPolicy>(
    places: &[Affine3<T>],
    walls_t: &[Vec<Arc<NurbsSurface<T>>>],
    canonical: &[ValidatedProfile<T>],
    section_params: Vec<f64>,
    tol: Tol,
) -> Result<Lofted<T>, LoftError> {
    let band = Band::linear(tol).map_err(LoftError::Band)?;
    let (Some(bottom_profile), Some(top_profile), Some(place_bottom), Some(place_top)) = (
        canonical.first(),
        canonical.last(),
        places.first(),
        places.last(),
    ) else {
        return Err(LoftError::SectionStructure);
    };
    let (bplace, tplace) = (*place_bottom, *place_top);
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
        || places.len() != canonical.len()
        || bloops.len() != tloops.len()
        || bloops.len() != walls_t.len()
        || bloops
            .iter()
            .zip(&tloops)
            .zip(walls_t)
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
                // `u`), run top to bottom as the turn laid it — on the
                // reflected parameter `t ∈ [−1, 0]`, `v = −t`, the one
                // reflection of the column that is exact.
                let carrier = geom_brep::reversed_column(&carrier);
                let spec = EdgeCurveSpec {
                    description: EdgeDescriptionSpec::wrap_iso(
                        wall_key,
                        T::zero(),
                        T::one(),
                        T::zero(),
                        -T::one(),
                        T::zero(),
                    ),
                    carrier: Curve3::Nurbs(Arc::new(carrier)),
                    param_start: -T::one(),
                    param_end: T::zero(),
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
        section_params,
    })
}

/// **The loft body** (M6-PLAN unit 3, spec §1): skins
/// [`loft_geometry`](crate::loft_geometry) and assembles the closed solid around it.
///
/// `sections[i][l][j]` is section `i`, loop `l`, segment `j` in sketch
/// coordinates; `places[i]` its rigid placement; `v_degree` the
/// skinning degree in the section direction, all in `T`. Structure is
/// selected at `f64` on their value channel (C6), and the body is
/// computed in `T` (module docs, "Scalar posture").
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
    sections: &[Section<impl SectionLoop<T>>],
    places: &[Affine3<T>],
    v_degree: usize,
    tol: Tol,
) -> Result<Lofted<T>, LoftError> {
    let channel: Vec<Affine3<f64>> = places.iter().map(|p| p.map(T::value_channel)).collect();
    build(sections, places, &channel, v_degree, tol)
}

/// **The loft body over sections already validated at `T`** — each a
/// [`ValidatedProfile`] carrying its own sketch plane, which is its
/// placement. The door for a caller that holds its sections as
/// canonical forms in the lane scalar: a document's profile values,
/// which its own (guided) validation produced.
///
/// The structure is selected on the sections' value channel
/// ([`ValidatedProfile::value_channel`]); no section is validated
/// again, and at a projective scalar the walls, caps and fold read the
/// given forms themselves. Everything else is [`loft_body`]'s.
///
/// # Errors
///
/// [`loft_body`]'s, with a section or placement whose values are no
/// single points refused as [`LoftError::SectionNotAPoint`] or
/// [`LoftError::PlacementNotAPoint`].
pub fn loft_profiles<T: Decide + topo::AtRestPolicy>(
    sections: &[ValidatedProfile<T>],
    v_degree: usize,
    tol: Tol,
) -> Result<Lofted<T>, LoftError> {
    let places: Vec<Affine3<T>> = sections.iter().map(|s| s.plane().placement).collect();
    let canonical: Vec<ValidatedProfile<f64>> = sections
        .iter()
        .map(ValidatedProfile::value_channel)
        .collect();
    let channel_places: Vec<Affine3<f64>> = canonical.iter().map(|c| c.plane().placement).collect();
    check_counts(sections.len(), places.len(), v_degree).map_err(LoftError::Skin)?;
    if let Some(section) = channel_places
        .iter()
        .position(|p| !p.components().iter().all(|c| c.is_finite()))
    {
        return Err(LoftError::PlacementNotAPoint { section });
    }
    if let Some(section) = canonical.iter().position(|c| !profile_is_finite(c)) {
        return Err(LoftError::SectionNotAPoint { section });
    }
    check_shapes(&canonical).map_err(LoftError::Skin)?;
    T::split_projective(LoftBuild::<T, profile::ProfileLoop<T>> {
        at_t: SectionsAtT::Validated(sections),
        places: &places,
        channel_places: &channel_places,
        canonical,
        v_degree,
        tol,
    })
}

/// Whether every value of an `f64` canonical form is finite.
fn profile_is_finite(profile: &ValidatedProfile<f64>) -> bool {
    let point = |p: &geom_core::Point2<f64>| p.x.is_finite() && p.y.is_finite();
    profile.loops().iter().all(|lp| {
        lp.vertices().iter().all(point)
            && lp.segments().iter().all(|s| {
                point(&s.start)
                    && point(&s.end)
                    && match s.kind {
                        profile::SegmentKind::Line => true,
                        profile::SegmentKind::Arc { arc, .. } => {
                            point(&arc.centre) && arc.radius.is_finite() && arc.sweep.is_finite()
                        }
                    }
            })
    })
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
    profile: &[impl SectionLoop<T>],
    place: Affine3<T>,
    path: &geom::NurbsCurve3<f64>,
    stations: usize,
    v_degree: usize,
    tol: Tol,
) -> Result<Lofted<T>, LoftError> {
    // The stations' structure is selected on the start frame's value
    // channel, composed in `f64`; the stations in `T` compose the
    // caller's frame with the same path frames.
    let start = place.map(T::value_channel);
    if !start.components().iter().all(|c| c.is_finite()) {
        return Err(LoftError::PlacementNotAPoint { section: 0 });
    }
    let channel = sweep_places(start, path, stations).map_err(LoftError::Skin)?;
    let places = sweep_places(place, path, stations).map_err(LoftError::Skin)?;
    let sections: Vec<Section<_>> = core::iter::repeat_n(profile.to_vec(), places.len()).collect();
    build(&sections, &places, &channel, v_degree, tol)
}
