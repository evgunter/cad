//! **The loft/sweep BODY assembly** (M6-3, executing the design
//! banked as M5 PR 9c item 6).
//!
//! The topology is EXTRUDE'S with different geometry (item 6(i)):
//! bottom cap from section 0, top cap from section k−1, one NURBS wall
//! per profile segment ([`crate::skin::LoftGeometry`]), wall–wall
//! seams as the walls' `u ∈ {0, 1}` boundary iso-curves, struts swept
//! per vertex. At v-degree 1 through `k ≥ 3` sections that wall is one
//! per slab ([`fn@slab_walls`]): the body is `k − 1` such laminae swept
//! one on the next, each interior section a ring of rims between two
//! slabs' walls, each in the row `v = 1` of the wall below it. The three
//! edge classes:
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
//! fight). Every rim reads the normal of the section it lies in: the
//! bottom cap and its rims read section 0's, the top cap and its rims
//! section `k − 1`'s, and an interior section's ring its own
//! (`assemble`'s `rim_spec`). The fold has decided each of those normals
//! against the displacement of the slab or slabs it bounds, and its
//! margins are not read again.
//!
//! # Scalar posture (C6)
//!
//! The sections and placements arrive in the body's scalar `T`. A
//! section or placement holding a value that is not finite refuses
//! first, at every scalar. Structure — each section's canonical form,
//! the skin's degrees, multiplicities and parameter order — is
//! selected once, on the sections' value channel where every value is
//! a point, so every lane builds the SAME body; over a family it is
//! what every member shares, or a typed refusal. The values then
//! depend on which side of the projective partition `T` is on
//! ([`fn@build`]):
//!
//! - at `f64`, `Probe`, and `Dual` and `Sym` over them, the sections
//!   are validated at `T` along the `f64` record and the walls are
//!   skinned in `T` ([`fn@crate::skin`]), so a wall, a cap and the
//!   stacking fold's margins are the functions of the sections'
//!   variables the sections are;
//! - at `Interval`, and the wrappers over it, the body encloses the
//!   family its sections stand for. Sections and placements that are
//!   points are a family of one: the `f64` skin of their value channel
//!   is the definition, its walls are lifted through `from_computed`
//!   (a seam column both walls end on lifted once and shared), and the
//!   placements the skin was selected on are lifted the same way, so
//!   the caps (the `f64` canonical form lifted onto them), the rims and
//!   the fold read the one loft the body encloses. Wider sections or
//!   placements are validated and skinned in `T`, as at a projective
//!   scalar, and the walls enclose the family. The skin's structure is
//!   then what every value of the family shares, and a carrier's
//!   weights and knots are points, so an arc section, or an interior
//!   knot wider than a point, refuses typed
//!   (`work/inlane/an-enclosure-carrier-holds-point-weights-and-knots.md`).

use core::fmt;
use std::sync::Arc;
use topo::construction::Construct;

use geom::Curve3;
use geom::{NurbsSurface, Surface};
use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec};
use geom_core::spline::{ProjectiveScalar, ProjectiveSplit, SplineError};
use geom_core::{
    Affine3, Band, BandError, Decide, Indeterminate, Margin, Point2, Point3, Real, Sign, Tol, Vec3,
};
use profile::{CanonicalStructure, SketchPlane, ValidatedProfile};
use topo::construction::{FaceSurface, MefSite, MevCreated, MevSite};
use topo::{Body, EdgeKey, EulerOpError, FaceKey, PcurveMintError, ShellKey, SolidKey};

use crate::skin::{
    PathCarry, Section, SectionLoop, SkinError, check_counts, check_shapes, skin_validated,
    validate_at_t, validate_guided, validate_loft,
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
    /// Wall faces `[loop][segment][slab]`, loops in canonical order
    /// (outer first), segments in canonical order, slabs in stacking
    /// order. At v-degree ≥ 2, and through two sections, each segment
    /// has one wall, `side_faces[l][j][0]`, carrying
    /// `LoftGeometry.walls[l][j]`; at v-degree 1 through `k ≥ 3`
    /// sections it has one per slab, `side_faces[l][j][s]` spanning
    /// sections `s` and `s + 1` on that wall's rows `s` and `s + 1`.
    pub side_faces: Vec<Vec<Vec<FaceKey>>>,
    /// Wall–wall seam edges `[loop][vertex][slab]`: `seams[l][j][s]` is
    /// the strut at vertex `j` across slab `s` — wall `[j][s]`'s
    /// `u = 0` boundary iso (and wall `[j − 1 mod n][s]`'s `u = 1`).
    /// Struts of adjacent slabs meet at an interior section's vertex,
    /// and the edges of that section's ring are the rims between
    /// adjacent slabs' walls.
    pub seam_edges: Vec<Vec<Vec<EdgeKey>>>,
    /// **The v-parameter each input section sits at** —
    /// [`LoftGeometry::section_params`](crate::skin::LoftGeometry::section_params) carried out to the caller
    /// (`[0] = 0`, `[k − 1] = 1`) instead of dropped with the
    /// geometry, so "what spacing did the skin choose for my
    /// sections?" is answered by the result rather than re-derived by
    /// hand. It is the SKIN's parameter: every skinned wall
    /// (`LoftGeometry.walls`) agrees on it, and section `i` is that
    /// wall's iso row `v = section_params[i]`. The body's walls carry
    /// it only where each wall spans every section (v-degree ≥ 2, or
    /// two sections). At v-degree 1 through `k ≥ 3` sections each body
    /// wall is one slab on its own `v ∈ [0, 1]` (`side_faces`), so no
    /// body wall holds these values as knots. They still say where
    /// along the stack the skin put each section.
    ///
    /// This is a re-read of what the kernel chose, not a measurement
    /// — the produced surface IS the definition (DESIGN Q8), so no
    /// residual pad accompanies it. A [`loft_body`]'s is
    /// [`crate::loft_parameters`]' answer for its sections, askable
    /// BEFORE the body is built; a [`sweep_body`]'s is each station's
    /// normalised path parameter, `i / (stations − 1)`. Each is held in
    /// `T`, so at an enclosure scalar an interior parameter over a box
    /// reads as its bracket.
    pub section_params: Vec<T>,
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
    /// Section `section` holds a value that is not finite for every
    /// value it stands for: poison, an infinity, or an enclosure
    /// reaching one.
    SectionNotFinite {
        /// The section.
        section: usize,
    },
    /// Section `section`'s placement has an entry that is not finite
    /// for every value it stands for: poison, an infinity, or an
    /// enclosure reaching one.
    PlacementNotFinite {
        /// The section whose placement it is.
        section: usize,
    },
    /// An interior section's rim found no image on the row `v = 1` of the
    /// slab wall below it (degree 1 through `k ≥ 3` sections only).
    ///
    /// `None`: no arc-rim image holds the rim. Ordinary input reaches
    /// this. An arc segment's wall carries every section's sub-arc
    /// structure at once, so where the sections' arcs split into
    /// different numbers of sub-arcs (a bulge that varies enough across
    /// the sections), an arc with fewer sub-arcs reaches its row refined
    /// at the others' joints. The arc-rim class describes only a row with
    /// the arc's own uniform sub-arc knots, so it does not describe that
    /// row. The same class refuses a cap rim of a two-section loft at
    /// the pcurve mint
    /// (`work/isotail/a-loft-whose-arc-bulge-varies-per-section-refuses-its-arc-rim.md`).
    /// Recourse: loft at v-degree 2 or more, whose walls span every
    /// section, so the interior section is no rim.
    ///
    /// `Some`: the pick was undecided in band.
    SectionRim {
        /// The section.
        section: usize,
        /// The pick's escalation, where it was undecided.
        source: Option<Indeterminate>,
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
            Self::SectionRim {
                section,
                source: None,
            } => write!(
                f,
                "loft section {section}'s arc rim has no image on the degree-1 wall below it: \
                 the sections' arcs split into different numbers of sub-arcs, so this one \
                 reaches its row refined, which the arc-rim class does not describe. \
                 Recourse: loft at v-degree 2 or more, whose walls span every section"
            ),
            Self::SectionRim {
                section,
                source: Some(source),
            } => write!(
                f,
                "{}",
                source.undecided(
                    format_args!(
                        "which row of the wall below loft section {section} holds its rim"
                    ),
                    source.ending(geom_core::COINCIDENCE_RECOURSE),
                )
            ),
            Self::SectionNotFinite { section } => write!(
                f,
                "section {section} holds a value that is not finite, so it has no locus to \
                 loft. Recourse: give the section finite values"
            ),
            Self::PlacementNotFinite { section } => write!(
                f,
                "section {section}'s placement is not finite, so the section has no place in \
                 space. Recourse: give the placement finite values"
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
                "{}",
                source.undecided(
                    format_args!(
                        "whether loft section {far}'s plane faces along the stack from \
                         section {slab}",
                        far = SlabPair(*slab).far()
                    ),
                    source.ending(geom_core::COINCIDENCE_RECOURSE),
                )
            ),
            Self::StackingEscalated { slab, source } => write!(
                f,
                "{}",
                source.undecided(
                    format_args!("whether loft {} stack forward", SlabPair(*slab)),
                    source.ending(geom_core::COINCIDENCE_RECOURSE),
                )
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
/// here without an outer loop is a corrupt [`LoftGeometry`](crate::skin::LoftGeometry), not an
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
    places: &[SketchPlane<T>],
    canonical: &[ValidatedProfile<T>],
    band: Band,
) -> Result<(), LoftError> {
    if places.len() < 2 || places.len() != canonical.len() {
        return Err(LoftError::SectionStructure);
    }
    let mut outers = places
        .iter()
        .zip(canonical)
        .map(|(place, c)| outer_world(c, place.placement()).ok_or(LoftError::SectionStructure));
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
        let facing = |place: &SketchPlane<T>| {
            let margin = d.dot(place.normal().get()) / count;
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

/// The shared engine of [`loft_body`], [`sweep_body`] and
/// [`loft_profiles`]: refuse the counts, then a section or placement
/// with no finite value, then run the half of the projective partition `T` is on
/// ([`SpanLocate::split_projective`](geom_core::spline::SpanLocate::split_projective)),
/// which [`LoftBuild`]'s two halves state.
///
/// Each half decides the stacking fold BEFORE the skin, because it is
/// the one decision about whether two adjacent sections are apart,
/// banded under the run's tolerance; a pair it accepts is definitely
/// apart, and a pair it refuses (down to exact coincidence) never
/// reaches the skin's parameterization.
fn build<T: Decide + topo::AtRestPolicy, L: SectionLoop<T>>(
    sections: SectionsAtT<'_, T, L>,
    places: &[SketchPlane<T>],
    channel_places: Option<&[SketchPlane<f64>]>,
    params: Option<&[f64]>,
    v_degree: usize,
    tol: Tol,
) -> Result<Lofted<T>, LoftError> {
    check_counts(sections.len(), places.len(), v_degree).map_err(LoftError::Skin)?;
    if let Some(section) = places
        .iter()
        .position(|p| !p.placement().components().iter().all(|c| c.is_bounded()))
    {
        return Err(LoftError::PlacementNotFinite { section });
    }
    if let Some(section) = sections.first_not_finite() {
        return Err(LoftError::SectionNotFinite { section });
    }
    T::split_projective(LoftBuild {
        sections,
        places,
        channel_places,
        params,
        v_degree,
        tol,
    })
}

/// Where a loft's sections at `T` come from.
enum SectionsAtT<'a, T: Real, L> {
    /// Loops: [`loft_body`]'s and [`sweep_body`]'s.
    Loops(&'a [Section<L>]),
    /// Canonical forms the caller validated at `T`
    /// ([`loft_profiles`]).
    Validated(&'a [ValidatedProfile<T>]),
}

impl<T: Decide, L: SectionLoop<T>> SectionsAtT<'_, T, L> {
    fn len(&self) -> usize {
        match self {
            Self::Loops(s) => s.len(),
            Self::Validated(s) => s.len(),
        }
    }

    /// The first section holding a value that is not finite for every
    /// value it stands for ([`Real::is_bounded`]): poison, an
    /// infinity, or an enclosure reaching one. A value wider than a
    /// point is finite.
    fn first_not_finite(&self) -> Option<usize> {
        match self {
            Self::Loops(sections) => sections.iter().position(|loops| {
                !loops
                    .iter()
                    .all(|lp| loop_all(lp.as_profile_loop(), T::is_bounded))
            }),
            Self::Validated(sections) => {
                sections.iter().position(|p| !profile_all(p, T::is_bounded))
            }
        }
    }

    /// Whether every input to the skin is a point: each placement
    /// (`channel_places`, its point value, which [`sweep_body`] carries
    /// in `f64`, present only where every placement has one) and each
    /// section value.
    fn all_points(&self, channel_places: Option<&[SketchPlane<f64>]>) -> bool {
        let point = |v: T| v.point_value().is_some();
        channel_places.is_some()
            && match self {
                Self::Loops(sections) => sections
                    .iter()
                    .all(|loops| loops.iter().all(|lp| loop_all(lp.as_profile_loop(), point))),
                Self::Validated(sections) => sections.iter().all(|p| p.value_channel().is_ok()),
            }
    }

    /// The sections' value channel at `f64`, validated: the canonical
    /// form, and for loops the record each is validated along at `T`
    /// ([`validate_guided`]). Read only where [`Self::all_points`]
    /// holds.
    #[allow(clippy::type_complexity)]
    fn channel(
        &self,
        channel_places: &[SketchPlane<f64>],
        v_degree: usize,
        tol: Tol,
    ) -> Result<(Vec<ValidatedProfile<f64>>, Vec<CanonicalStructure>), LoftError> {
        match self {
            Self::Loops(sections) => {
                let channel: Vec<Section<L::Channel>> = sections
                    .iter()
                    .map(|s| s.iter().map(SectionLoop::channel).collect())
                    .collect();
                validate_loft(&channel, channel_places, v_degree, tol).map_err(LoftError::Skin)
            }
            Self::Validated(sections) => {
                let canonical: Vec<ValidatedProfile<f64>> = sections
                    .iter()
                    .map(ValidatedProfile::value_channel)
                    .collect::<Result<_, _>>()
                    .unwrap_or_else(|site| {
                        unreachable!(
                            "{site}: every value is a point here, by `all_points` at an \
                             enclosure scalar and by `build`'s finite check at a projective one"
                        )
                    });
                check_shapes(&canonical)
                    .map(|()| (canonical, Vec::new()))
                    .map_err(LoftError::Skin)
            }
        }
    }
}

/// The two halves of [`fn@build`], after the counts and the finite
/// check.
struct LoftBuild<'a, T: Real, L> {
    sections: SectionsAtT<'a, T, L>,
    places: &'a [SketchPlane<T>],
    channel_places: Option<&'a [SketchPlane<f64>]>,
    /// The v each section sits at where the caller states it (a path
    /// sweep's path parameters); otherwise the loft's chord-length rule.
    params: Option<&'a [f64]>,
    v_degree: usize,
    tol: Tol,
}

impl<T: Decide + topo::AtRestPolicy, L: SectionLoop<T>> LoftBuild<'_, T, L> {
    /// **The loft in `T`**, over `canonical`, the sections validated at
    /// `T`: the fold, the skin ([`skin_validated`]: the section curves,
    /// their compatibility, the parameters and the solve) and the caps
    /// all read those, so a section or placement value that is a
    /// function of the variables gives walls and caps that are
    /// functions of them, and one wider than a point gives walls and
    /// caps that enclose its family.
    fn in_t(&self, canonical: Vec<ValidatedProfile<T>>) -> Result<Lofted<T>, LoftError> {
        let band = Band::linear(self.tol).map_err(LoftError::Band)?;
        stacking_fold(self.places, &canonical, band)?;
        let geometry = skin_validated(canonical, self.places, self.v_degree, self.params)
            .map_err(LoftError::Skin)?;
        assemble(
            self.places,
            &geometry.walls,
            &geometry.canonical,
            geometry.section_params,
            self.tol,
        )
    }
}

impl<T: Decide + topo::AtRestPolicy, L: SectionLoop<T>> ProjectiveSplit<T> for LoftBuild<'_, T, L> {
    type Out = Result<Lofted<T>, LoftError>;

    /// **At a projective scalar** every finite value is a point, so the
    /// structure is selected on the sections' value channel at `f64`
    /// ([`validate_loft`]), each section of loops is validated at `T`
    /// along its `f64` record ([`validate_guided`]) or arrives so
    /// validated ([`loft_profiles`]), and the loft is built in `T`
    /// ([`Self::in_t`]).
    fn projective(self) -> Self::Out
    where
        T: ProjectiveScalar,
    {
        // Every finite value at a projective scalar is a point, and
        // `build` refused the placements that are not finite.
        let channel_places = self.channel_places.unwrap_or_else(|| {
            unreachable!("a finite placement at a projective scalar has a point value")
        });
        let (_, structures) = self
            .sections
            .channel(channel_places, self.v_degree, self.tol)?;
        let canonical = match self.sections {
            SectionsAtT::Loops(sections) => {
                validate_guided(sections, self.places, &structures, self.tol)
                    .map_err(LoftError::Skin)?
            }
            SectionsAtT::Validated(sections) => sections.to_vec(),
        };
        self.in_t(canonical)
    }

    /// **At an enclosure scalar** the body encloses the family its
    /// sections stand for.
    ///
    /// - Sections and placements that are points are a family of one,
    ///   and the body encloses its `f64` loft. The skin is the `f64`
    ///   one, the walls are lifted ([`lift_loop_walls`]), and the caps
    ///   and the fold read the `f64` canonical form lifted onto the
    ///   lane's own placements ([`ValidatedProfile::lift_onto`]), whose
    ///   point values are the planes the skin was selected on.
    /// - Otherwise the loft is built in `T` ([`Self::in_t`]), each
    ///   section of loops validated at `T` with its decisions certified
    ///   there. The skin's structure is what every value of the family
    ///   shares, and where it is not (an arc, whose weights are no
    ///   points; an interior knot wider than a point) the skin refuses
    ///   typed ([`SkinError`]).
    fn enclosure(self) -> Self::Out {
        let Some(channel_places) = self
            .channel_places
            .filter(|_| self.sections.all_points(self.channel_places))
        else {
            let canonical = match self.sections {
                SectionsAtT::Loops(sections) => {
                    validate_at_t(sections, self.places, self.tol).map_err(LoftError::Skin)?
                }
                SectionsAtT::Validated(sections) => sections.to_vec(),
            };
            check_shapes(&canonical).map_err(LoftError::Skin)?;
            return self.in_t(canonical);
        };
        let (canonical, _) = self
            .sections
            .channel(channel_places, self.v_degree, self.tol)?;
        let band = Band::linear(self.tol).map_err(LoftError::Band)?;
        let places = self.places;
        let lifted: Vec<ValidatedProfile<T>> = canonical
            .iter()
            .zip(places)
            .map(|(c, place)| {
                c.clone().lift_onto(*place).unwrap_or_else(|e| {
                    unreachable!("{e}: these placements are points, by `all_points`")
                })
            })
            .collect();
        stacking_fold(places, &lifted, band)?;
        let geometry = skin_validated(canonical, channel_places, self.v_degree, self.params)
            .map_err(LoftError::Skin)?;
        let walls = geometry
            .walls
            .iter()
            .map(|w| lift_loop_walls(w))
            .collect::<Result<Vec<_>, _>>()?;
        let params = geometry
            .section_params
            .iter()
            .map(|u| T::from_computed(*u))
            .collect();
        assemble(places, &walls, &lifted, params, self.tol)
    }
}

/// **The walls one skinned wall is built as.** At v-degree ≥ 2, and
/// through two sections, the wall itself. At v-degree 1 through
/// `sections ≥ 3`, one wall per slab: the skinned wall is not C¹ at its
/// interior sections by its knot structure, which is where DESIGN
/// \[topology\] lets the kernel cut ("the kernel adds none, except where a
/// face or edge carrier is not C¹ by its knot structure"). The net's
/// control rows are its sections, so slab `s` is rows `s` and
/// `s + 1` on `v ∈ [0, 1]`, the `u` structure and every weight copied —
/// a sub-net, with no arithmetic. Adjacent slabs share their section's
/// row bit for bit.
///
/// # Errors
///
/// [`LoftError::SectionStructure`] for a degree-1 net whose row count
/// is not the section count, and [`LoftError::SeamStructure`] if a
/// sub-net is refused; neither is reachable from the skin.
fn slab_walls<T: geom_core::spline::SpanLocate>(
    wall: &Arc<NurbsSurface<T>>,
    sections: usize,
) -> Result<Vec<Arc<NurbsSurface<T>>>, LoftError> {
    let (nu, nv) = wall.control_counts();
    if wall.knot_values_v().degree() != 1 || sections == 2 {
        return Ok(vec![Arc::clone(wall)]);
    }
    if nv != sections {
        return Err(LoftError::SectionStructure);
    }
    let two = core::num::NonZeroUsize::MIN.saturating_add(1);
    (0..nv - 1)
        .map(|s| {
            let rows = (0..nu).flat_map(|iu| [iu * nv + s, iu * nv + s + 1]);
            let control = rows.clone().map(|i| wall.control()[i]).collect();
            let weights = rows.map(|i| wall.weights()[i]).collect();
            let knots_v = geom_core::spline::Knots::new(vec![(T::zero(), two), (T::one(), two)], 1)
                .map_err(|source| LoftError::SeamStructure { source })?;
            NurbsSurface::new(wall.knot_values_u().clone(), knots_v, control, weights)
                .map(Arc::new)
                .map_err(|source| LoftError::SeamStructure { source })
        })
        .collect()
}

/// What a one-segment loop's sweep through every slab minted
/// (`assemble`'s `full_turn`), slab by slab in stacking order: the top
/// slab's [`crate::swept::FullTurn`] continued down through each slab
/// below. Like a `FullTurn` it keeps the ends of what was swept (the far
/// rim's kept half, the near rim's half in the wall, the near face), here of the whole
/// stack, and it lists a wall, a strut and a rim per slab where a
/// `FullTurn` holds one of each.
struct SlabTurn {
    /// The top rim's plus half, kept by the loop the turn was built in.
    far_kept: topo::HalfEdgeKey,
    /// The bottom rim's plus half, in slab 0's wall.
    near_in_wall: topo::HalfEdgeKey,
    /// The face holding the bottom rim's minus half.
    near_face: FaceKey,
    /// The wall of each slab.
    walls: Vec<FaceKey>,
    /// The strut of each slab.
    struts: Vec<EdgeKey>,
    /// The rim at each slab boundary, bottom (`0`) to top (`slabs`).
    rims: Vec<EdgeKey>,
}

impl crate::swept::NearFace for SlabTurn {
    fn near_face(&self) -> FaceKey {
        self.near_face
    }
}

/// Re-describes the rim `edge` an interior section `section` laid, the
/// row `v = 1` of the slab wall `wall` below it (on the chart `chart`),
/// as that row's image: a line along the row's own parameter, which is
/// linear in a placed line's, or a circle through the row's
/// rational-quadratic parameter (`geom_brep::pick_arc_rim`, the mint's
/// own pick). Its carrier is the placed sketch segment, kept verbatim,
/// and the sketch segment that declared it stays its authority.
///
/// # Errors
///
/// [`LoftError::SectionRim`] where no arc-rim image holds the circle or
/// the pick is undecided; the certified setter's refusals.
fn describe_section_rim<T: Decide + topo::AtRestPolicy>(
    body: &mut Body<T>,
    edge: EdgeKey,
    chart: topo::SurfaceKey,
    wall: &Arc<NurbsSurface<T>>,
    section: usize,
    band: Band,
    tol: Tol,
) -> Result<(), LoftError> {
    let spec = body.stored_curve(edge)?.restated_spec();
    let EdgeDescriptionSpec::Scaffold(declared) = spec.description else {
        unreachable!(
            "an interior section's rim is minted through the scaffolding door, and only this \
             pass describes it"
        )
    };
    let (t0, t1) = (spec.param_start, spec.param_end);
    let image = match spec.carrier {
        Curve3::Circle { .. } => geom_brep::pick_arc_rim(
            &spec.carrier,
            (t0, t1 - t0),
            &Surface::Nurbs(Arc::clone(wall)),
            band,
        )
        .map_err(|source| LoftError::SectionRim {
            section,
            source: Some(source),
        })?
        .ok_or(LoftError::SectionRim {
            section,
            source: None,
        })?,
        _ => {
            let slope = T::one() / (t1 - t0);
            geom_brep::Pcurve::IsoLine {
                p0: Point2::new(-slope * t0, T::one()),
                pl: geom_core::Vec2::new(slope, T::zero()),
            }
        }
    };
    body.restate_stored_edge(
        edge,
        EdgeDescriptionSpec::chart_image(chart, image).declared_by(declared),
        tol,
    )?;
    Ok(())
}

/// Assembles the loft BODY from its skinned geometry (module docs),
/// over sections the stacking fold has already accepted.
///
/// # Errors
///
/// [`LoftError`] — every door named on the enum.
fn assemble<T: Decide + topo::AtRestPolicy>(
    places: &[SketchPlane<T>],
    walls_t: &[Vec<Arc<NurbsSurface<T>>>],
    canonical: &[ValidatedProfile<T>],
    section_params: Vec<T>,
    tol: Tol,
) -> Result<Lofted<T>, LoftError> {
    let band = Band::linear(tol).map_err(LoftError::Band)?;
    let (Some(place_bottom), Some(place_top)) = (places.first(), places.last()) else {
        return Err(LoftError::SectionStructure);
    };
    let (bplace, tplace) = (place_bottom.placement(), place_top.placement());

    // ---- The walls, one per segment per slab ([`slab_walls`]). Slab
    // boundary `b ∈ 0 ..= slabs` sits at section `at(b)`. ----
    let k = canonical.len();
    let walls: Vec<Vec<Vec<Arc<NurbsSurface<T>>>>> = walls_t
        .iter()
        .map(|lp| {
            lp.iter()
                .map(|w| slab_walls(w, k))
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<_, _>>()?;
    let slabs = walls.first().and_then(|lp| lp.first()).map_or(1, Vec::len);
    let at = |b: usize| if slabs == 1 { b * (k - 1) } else { b };

    // ---- Traversals and world points, per section, per loop. ----
    let loops_at: Vec<Vec<Vec<SweptSeg<T>>>> = canonical
        .iter()
        .map(|c| {
            c.loops()
                .iter()
                .map(|lp| swept_segments(lp, false))
                .collect()
        })
        .collect();
    if places.len() < 2
        || places.len() != k
        || loops_at.iter().any(|ls| {
            ls.len() != walls.len() || ls.iter().zip(&walls).any(|(segs, w)| segs.len() != w.len())
        })
        || walls.iter().flatten().any(|w| w.len() != slabs)
    {
        return Err(LoftError::SectionStructure);
    }
    let q_at: Vec<Vec<Vec<Point3<T>>>> = loops_at
        .iter()
        .zip(places)
        .map(|(ls, p)| {
            let place = p.placement();
            ls.iter()
                .map(|segs| segs.iter().map(|s| world(&place, s.a)).collect())
                .collect()
        })
        .collect();
    let (bloops, tloops) = (&loops_at[0], &loops_at[k - 1]);
    let (bq, tq) = (&q_at[0], &q_at[k - 1]);
    // Loop `li`'s segment `j` placed at section `i`, from `q_from` to
    // `q_to`.
    let rim_spec = |i: usize, li: usize, j: usize, q_from, q_to| {
        placed_segment_spec(
            &loops_at[i][li][j],
            places[i].placement(),
            places[i].normal().get(),
            q_from,
            q_to,
            tol,
        )
    };
    let wall_surface = |li: usize, j: usize, s: usize| FaceSurface::New {
        surface: Surface::Nurbs(Arc::clone(&walls[li][j][s])),
        // The skinned chart's normal points out of the material
        // (module docs).
        sense: true,
    };

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
    // A one-segment loop (D1's full turn) is swept whole in phases 1–2:
    // its top slab first (`build_full_turn`), then down through the
    // slabs below, so the face each step opens is the next slab's wall
    // and the last is `near_cap`. Phases 3–4 skip it.
    let full_turn = |body: &mut Body<T>, li: usize, r#loop, near_cap: FaceSurface<T>| {
        let point = |b: usize| q_at[at(b)][li][0];
        let below = |s: usize| {
            if s == 0 {
                near_cap.clone()
            } else {
                wall_surface(li, 0, s - 1)
            }
        };
        let top = slabs - 1;
        let (far, near) = (point(slabs), point(top));
        let turn = crate::swept::build_full_turn(
            body,
            r#loop,
            near,
            rim_spec(at(slabs), li, 0, far, far),
            wall_surface(li, 0, top),
            EdgeCurveSpec::line_between(far, near),
            rim_spec(at(top), li, 0, near, near),
            below(top),
            tol,
        )?;
        let mut faces = vec![turn.wall];
        let mut struts = vec![turn.strut];
        let mut rims = vec![turn.far, turn.near];
        let (mut minus, mut in_wall, mut face) =
            (turn.near_minus, turn.near_in_wall, turn.near_face);
        for s in (0..top).rev() {
            let (upper, lower) = (point(s + 1), point(s));
            let strut = body.mev(
                MevSite::Fan {
                    he1: minus,
                    he2: minus,
                },
                lower,
                EdgeCurveSpec::line_between(upper, lower),
                tol,
            )?;
            let rim = body.mef(
                MefSite::Chords {
                    he1: strut.he_minus,
                    he2: strut.he_minus,
                },
                rim_spec(at(s), li, 0, lower, lower),
                below(s),
                tol,
            )?;
            faces.push(face);
            struts.push(strut.edge);
            rims.push(rim.edge);
            (minus, in_wall, face) = (rim.he_minus, rim.he_plus, rim.face);
        }
        faces.reverse();
        struts.reverse();
        rims.reverse();
        Ok::<_, LoftError>(SlabTurn {
            far_kept: turn.far_kept,
            near_in_wall: in_wall,
            near_face: face,
            walls: faces,
            struts,
            rims,
        })
    };
    let mut early: Vec<Option<SlabTurn>> = (0..bloops.len()).map(|_| None).collect();
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
            rim_spec(0, 0, 0, qs[0], qs[1]),
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
                rim_spec(0, 0, j - 1, qs[j - 1], qs[j]),
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
            rim_spec(0, 0, n - 1, qs[n - 1], qs[0]),
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
    // bottom cap) — extrude's phase verbatim, loft rim specs. A
    // one-segment hole is planted at its far vertex and swept whole
    // into the ring, its near face the transient disc the bottom cap
    // consumes. ----
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
            rim_spec(0, li, 0, hq[0], hq[1]),
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
                rim_spec(0, li, j - 1, hq[j - 1], hq[j]),
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
            rim_spec(0, li, m - 1, hq[m - 1], hq[0]),
            crate::swept::transient_disc(bottom_surface),
            tol,
        )?;
        hole_hes.push(close.he_plus);
        body.kfmrh(bottom_face, close.face)?;
        bases.push(hole_hes);
    }

    // ---- Phases 3–4: per loop, slab after slab, sweep struts and
    // close the wall quads. A slab's far rims stay in the seed face as
    // the next slab's base; the last slab's are the top cap's. Struts
    // are SCAFFOLDING lines here (mev_line) and upgrade to the seam
    // class in phase 6, once their walls' keys exist. ----
    let mut side_faces: Vec<Vec<Vec<FaceKey>>> = Vec::with_capacity(bloops.len());
    let mut seam_edges: Vec<Vec<Vec<EdgeKey>>> = Vec::with_capacity(bloops.len());
    // Each interior section's rims, with the loop, segment and slab of
    // the wall below them.
    let mut section_rims: Vec<(EdgeKey, usize, usize, usize)> = Vec::new();
    for (li, base) in bases.iter().enumerate() {
        if let Some(turn) = early[li].take() {
            section_rims.extend(
                turn.rims[1..slabs]
                    .iter()
                    .enumerate()
                    .map(|(s, &rim)| (rim, li, 0, s)),
            );
            side_faces.push(vec![turn.walls]);
            seam_edges.push(vec![turn.struts]);
            continue;
        }
        let n = base.len();
        let mut base = base.clone();
        let mut faces: Vec<Vec<FaceKey>> = vec![Vec::with_capacity(slabs); n];
        let mut seams: Vec<Vec<EdgeKey>> = vec![Vec::with_capacity(slabs); n];
        for s in 0..slabs {
            let far = &q_at[at(s + 1)][li];
            let mut struts: Vec<MevCreated> = Vec::with_capacity(n);
            for j in 0..n {
                let m = body.mev_line(
                    MevSite::Fan {
                        he1: base[j],
                        he2: base[j],
                    },
                    far[j],
                    tol,
                )?;
                struts.push(m);
            }
            let mut first_top = None;
            for j in 0..n {
                let he2 = match (j + 1 < n, first_top) {
                    (true, _) => struts[j + 1].he_minus,
                    (false, Some(top)) => top,
                    (false, None) => struts[j].he_minus,
                };
                let mef = body.mef(
                    MefSite::Chords {
                        he1: struts[j].he_minus,
                        he2,
                    },
                    rim_spec(at(s + 1), li, j, far[j], far[(j + 1) % n]),
                    wall_surface(li, j, s),
                    tol,
                )?;
                if j == 0 {
                    first_top = Some(mef.he_plus);
                }
                faces[j].push(mef.face);
                seams[j].push(struts[j].edge);
                // The far rim's plus half stays in the seed face, from
                // the far end of strut `j`: the next slab's base.
                base[j] = mef.he_plus;
                if s + 1 < slabs {
                    section_rims.push((mef.edge, li, j, s));
                }
            }
        }
        side_faces.push(faces);
        seam_edges.push(seams);
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
    // now exist, so each strut re-describes as its slab wall's `u = 0`
    // boundary iso through the certified setter (D9 — loops in
    // canonical order, vertices in swept order, slabs in stacking
    // order) — and each interior section's rims come to rest in the
    // wall below them. ----
    for (li, seams) in seam_edges.iter().enumerate() {
        let n = seams.len();
        for j in 0..n {
            for s in 0..slabs {
                let wall_key = face_surface_key(&body, side_faces[li][j][s]);
                let carrier = geom_brep::boundary_iso_u(walls[li][j][s].as_ref(), false)
                    .map_err(|source| LoftError::SeamStructure { source })?;
                if n == 1 && profile::is_full_turn(&bloops[li]) {
                    // A one-segment loop's strut is its wall's wrap edge
                    // in `u` (D1: a closed spline net's boundary column
                    // wraps `u`), run top to bottom as the turn laid it
                    // — on the reflected parameter `t ∈ [−1, 0]`,
                    // `v = −t`, the one reflection of the column that is
                    // exact.
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
                    body.set_edge_curve(seams[j][s], spec, tol)?;
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
                body.set_edge_curve(seams[j][s], spec, tol)?;
            }
        }
    }
    for &(rim, li, j, s) in &section_rims {
        let chart = face_surface_key(&body, side_faces[li][j][s]);
        describe_section_rim(
            &mut body,
            rim,
            chart,
            &walls[li][j][s],
            at(s + 1),
            band,
            tol,
        )?;
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
///
/// # Panics
///
/// On a kernel bug: an edge the operation stored whose own span or
/// winding reads certainly refused at the run's band where it is
/// re-described ([`topo::Body::restate_stored_edge`]).
pub fn loft_body<T: Decide + topo::AtRestPolicy>(
    sections: &[Section<impl SectionLoop<T>>],
    places: &[SketchPlane<T>],
    v_degree: usize,
    tol: Tol,
) -> Result<Lofted<T>, LoftError> {
    let channel: Option<Vec<SketchPlane<f64>>> = places.iter().map(|p| p.point_value()).collect();
    build(
        SectionsAtT::Loops(sections),
        places,
        channel.as_deref(),
        None,
        v_degree,
        tol,
    )
}

/// **The loft body over sections already validated at `T`** — each a
/// [`ValidatedProfile`] carrying its own sketch plane, which is its
/// placement. The door for a caller that holds its sections as
/// canonical forms in the lane scalar: a document's profile values,
/// which its own (guided) validation produced.
///
/// No section is validated again: the walls, caps and fold read the
/// given forms themselves, with the structure selected on their value
/// channel ([`ValidatedProfile::value_channel`]) where it is a point.
/// Everything else is [`loft_body`]'s.
///
/// # Errors
///
/// [`loft_body`]'s.
///
/// # Panics
///
/// On a kernel bug: an edge the operation stored whose own span or
/// winding reads certainly refused at the run's band where it is
/// re-described ([`topo::Body::restate_stored_edge`]).
pub fn loft_profiles<T: Decide + topo::AtRestPolicy>(
    sections: &[ValidatedProfile<T>],
    v_degree: usize,
    tol: Tol,
) -> Result<Lofted<T>, LoftError> {
    let places: Vec<SketchPlane<T>> = sections.iter().map(|s| *s.plane()).collect();
    let channel_places: Option<Vec<SketchPlane<f64>>> =
        places.iter().map(|p| p.point_value()).collect();
    build(
        SectionsAtT::<T, profile::ProfileLoop<T>>::Validated(sections),
        &places,
        channel_places.as_deref(),
        None,
        v_degree,
        tol,
    )
}

/// Whether `holds` of every value of `lp`.
fn loop_all<T: Real>(lp: &profile::ProfileLoop<T>, holds: impl Fn(T) -> bool + Copy) -> bool {
    lp.vertices().iter().all(|p| point_all(p, holds))
        && lp.segments().iter().all(|s| match s {
            profile::Segment::Line => true,
            profile::Segment::Arc(arc) => arc_all(arc, holds),
        })
}

/// [`loop_all`] for a canonical form.
fn profile_all<T: Real>(profile: &ValidatedProfile<T>, holds: impl Fn(T) -> bool + Copy) -> bool {
    profile.loops().iter().all(|lp| {
        lp.vertices().iter().all(|p| point_all(p, holds))
            && lp.segments().iter().all(|s| {
                point_all(&s.start, holds)
                    && point_all(&s.end, holds)
                    && match &s.kind {
                        profile::SegmentKind::Line => true,
                        profile::SegmentKind::Arc { arc, .. } => arc_all(arc, holds),
                    }
            })
    })
}

fn point_all<T: Real>(p: &Point2<T>, holds: impl Fn(T) -> bool) -> bool {
    holds(p.x) && holds(p.y)
}

fn arc_all<T: Real>(arc: &geom_core::Arc2<T>, holds: impl Fn(T) -> bool + Copy) -> bool {
    point_all(&arc.centre, holds) && holds(arc.radius) && holds(arc.sweep)
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
/// path's carry of the start frame ([`crate::sweep_places`];
/// `crates/sweep/README.md` `[sweep.frame]`), not from the sections.
/// Station `i` sits at `v = i / (stations − 1)`, its share of the
/// path's parameter, which neither the spelling nor the roll of the
/// start frame moves.
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
/// [`LoftError`] — every door named on the enum, with every refusal
/// [`crate::sweep_places`] makes arriving through [`LoftError::Skin`].
///
/// # Panics
///
/// On a kernel bug: an edge the operation stored whose own span or
/// winding reads certainly refused at the run's band where it is
/// re-described ([`topo::Body::restate_stored_edge`]).
pub fn sweep_body<T: Decide + topo::AtRestPolicy>(
    profile: &[impl SectionLoop<T>],
    plane: SketchPlane<T>,
    path: &geom::NurbsCurve3<f64>,
    stations: usize,
    v_degree: usize,
    tol: Tol,
) -> Result<Lofted<T>, LoftError> {
    // One carry, composed onto the start frame in `T` and onto its
    // point value in `f64`, where the stations' structure is selected.
    let carry = PathCarry::of(path, stations, tol).map_err(LoftError::Skin)?;
    let channel = plane
        .point_value()
        .map(|start| carry.place(start))
        .transpose()
        .map_err(LoftError::Skin)?;
    let places = carry.place(plane).map_err(LoftError::Skin)?;
    let sections: Vec<Section<_>> = core::iter::repeat_n(profile.to_vec(), places.len()).collect();
    build(
        SectionsAtT::Loops(&sections),
        &places,
        channel.as_deref(),
        Some(&carry.params()),
        v_degree,
        tol,
    )
}

#[cfg(test)]
mod undecided_tests {
    use super::*;

    /// **A loft's stacking escalation offers the tolerance a value in the
    /// band gives**: the decision is the sections' mean spacing along a
    /// normal, a length.
    #[test]
    fn a_stacking_escalation_ends_in_its_lengths_tolerance() {
        use crate::swept::undecided_fixtures::{SIZED, expected, readings};
        let near = expected("whether loft sections 1 and 2 stack forward", SIZED);
        let far = expected(
            "whether loft section 2's plane faces along the stack from section 1",
            SIZED,
        );
        for ((source, near), far) in readings("loft_stacking").into_iter().zip(near).zip(far) {
            assert_eq!(
                LoftError::StackingEscalated { slab: 1, source }.to_string(),
                near
            );
            assert_eq!(
                LoftError::FarStackingEscalated { slab: 1, source }.to_string(),
                far
            );
        }
    }
}
