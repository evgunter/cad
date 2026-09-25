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
//! Sections must stack ALONG the plane normal of the section they
//! stack off. That is a PER-SLAB statement and [`fn@stacking_fold`] is
//! where it is made and where its shape is stated; a refusal names the
//! pair it stopped at.
//!
//! The caps then orient exactly as extrude's (bottom reversed, top
//! forward) and every wall's chart normal `S_u × S_v` points out of
//! the material — the u direction follows the material-left profile
//! traversal and v the stacking, so `t̂ × v̂` is material-right —
//! giving `sense = true` on every wall, holes and concave arcs
//! included (unlike a cylinder chart, the skinned chart's normal
//! FOLLOWS the traversal; there is no unconditionally-radial frame to
//! fight). **The bottom cap and the bottom lamina's rims read the
//! FIRST slab's base normal — section 0's — and the top cap and the
//! far rims read the LAST slab's top, section `k − 1`'s**; no other
//! reading of the stacking enters the assembly, and the fold's margin
//! is not read again once it has been decided.
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
use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec, NewellError, newell_plane};
use geom_core::spline::SplineError;
use geom_core::{
    Affine3, Band, BandError, Decide, Indeterminate, Margin, Point3, Real, Sign, Tol, Vec3,
};
use profile::{ProfileLoop, SketchPlane, ValidatedProfile};
use topo::{
    Body, DeclaredContact, EdgeKey, EulerOpError, FaceKey, FaceSurface, MefSite, MevCreated,
    MevSite, PcurveMintError, ShellKey, SolidKey,
};

use crate::skin::{LoftGeometry, Section, SkinError, loft_geometry, sweep_places};
use crate::swept::{
    SweptSeg, cap_points, describe_face_rim_at_rest, face_surface_key, placed_segment_spec,
    swept_segments,
};

/// Everything [`loft_body`]/[`sweep_body`] built, keyed — the
/// [`crate::Extruded`] bundle one operation over.
#[derive(Debug)]
pub struct Lofted<T: Real> {
    /// The closed body (tiers 1–3 valid at rest — the builder's
    /// acceptance; callers re-validate per the workspace convention).
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
    /// residual pad accompanies it. [`crate::loft_parameters`] answers the
    /// same question BEFORE the body is built.
    pub section_params: Vec<f64>,
    /// **The contacts the sections declared**: one `Tangent` pair per
    /// declared cusp joint — the walls of the two canonical segments
    /// meeting there, arriving wall first — loops in canonical order,
    /// joints ascending. A joint counts when ANY section declares a
    /// cusp there (sections pair by canonical index, so the wall pair
    /// is one pair along the loft). Empty when no section declares a
    /// cusp.
    ///
    /// The authors' declaration carried through the verb, not a
    /// discovery. Tier 3's material arm exempts an edge with a NURBS
    /// face by kind, so today a lofted cusp seam validates with or
    /// without this record; it is carried so the declaration reaches
    /// whatever reads the body's contacts, the same as the extrude's.
    pub declared_contacts: Vec<DeclaredContact>,
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
    /// A cap plane could not be certified from its boundary points.
    CapPlane(NewellError),
    /// The whole-body pcurve mint pass refused: a wall boundary's
    /// exact line-in-UV image failed its certification.
    Pcurve(PcurveMintError),
    /// The wall-boundary carrier extraction failed to re-wrap — a
    /// structurally corrupt skinned surface (unreachable from
    /// [`loft_geometry`] output; surfaced rather than swallowed).
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
    /// One SLAB's stacking displacement is coincident with zero at
    /// tolerance: a sliver-thin (or in-plane) pair of sections.
    DegenerateStacking {
        /// The slab — the pair [`SlabPair`] names.
        slab: usize,
    },
    /// The heading of a section's declared tangent joint — a
    /// continuation or a cusp ([`Lofted::declared_contacts`]) —
    /// escalated.
    ///
    /// Defense-in-depth: a verified tangent joint's headings are
    /// parallel or antiparallel, so the decision's margin is the whole
    /// lever arm — unreachable from validated sections, surfaced rather
    /// than trusted.
    CuspHeadingEscalated {
        /// The section, in input order.
        section: usize,
        /// Canonical index of the loop.
        loop_index: usize,
        /// Canonical index of the joint vertex.
        vertex_index: usize,
        /// The predicate-layer escalation.
        source: Indeterminate,
    },
    /// One SLAB's stacking classification escalated (named predicate
    /// on the diagnostic).
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

impl fmt::Display for SlabPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "sections {} and {}", self.0, self.0 + 1)
    }
}

impl fmt::Display for LoftError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Band(e) => write!(f, "{e}"),
            Self::Skin(e) => write!(f, "{e}"),
            Self::Euler(e) => write!(f, "an Euler operation of the assembly refused: {e}"),
            Self::CapPlane(e) => write!(f, "an end cap is not planar: {e}"),
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
                "loft {} are not apart at tolerance (a sliver-thin or in-plane slab), so \
                 the loft has no direction. Recourse: move the sections apart",
                SlabPair(*slab)
            ),
            Self::CuspHeadingEscalated {
                section,
                loop_index,
                vertex_index,
                source,
            } => write!(
                f,
                "whether the declared joint at section {section} loop {loop_index} vertex \
                 {vertex_index} continues or reverses its heading is too close to call: \
                 {source}"
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
        Self::Euler(e)
    }
}

/// One end section as a profile at `T`: **the canonical form
/// [`loft_geometry`] decided, lifted** ([`ValidatedProfile::lift_onto`])
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
/// [`loft_geometry`]'s, which refuses it
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
/// Slab `k` is the pair `(k, k + 1)`. Its margin is the mean
/// displacement of the outer loop's vertices between the two sections
/// against SECTION `k`'s own plane normal, and the fold decides slab
/// after slab under `loft_stacking` in slab order, **refusing at the
/// first slab whose verdict is not `Positive`**. No minimum is formed:
/// that is the ruling's "the min over slabs is Positive", because
/// `Positive` is monotone in the margin, and stopping at the first
/// non-`Positive` slab is what keeps an ambiguous slab from being
/// answered for by a definite one later in the list.
///
/// The margin is a sum of per-vertex differences over a full zip of
/// two equal-length loops, so it is the displacement of the vertex
/// CENTROID: the by-index pairing carries no information and rotating
/// one section's traversal cannot change the verdict.
///
/// A two-section loft is the fold's degenerate case — one slab whose
/// base section is the first section, over the vertices `assemble`
/// already walked, in that walk's order.
///
/// The fold reads nothing but the two sections of the slab it is
/// deciding: their authored placements and their canonical loops.
///
/// # Preconditions, and why its guards are dead through `loft_body`
///
/// The caller has already refused a `places`/`canonical` length
/// disagreement and a section count below two, and hands the first and
/// last sections' outer world loops in the traversal order it walked
/// them. What is left here — a section with no loops, and two sections
/// whose outer loops differ in vertex count — is this function's own
/// precondition, kept as a refusal rather than an assumption. Neither
/// is reachable through [`loft_body`] today: the skin refuses
/// mismatched sections first (`SkinError::SectionShapeMismatch`), and
/// a loopless section never leaves profile validation.
fn stacking_fold<T: Decide>(
    places: &[Affine3<f64>],
    geometry: &LoftGeometry,
    band: Band,
    first_outer: &[Point3<T>],
    last_outer: &[Point3<T>],
) -> Result<(), LoftError> {
    let last = places.len() - 1;
    let mut base: Vec<Point3<T>> = first_outer.to_vec();
    for slab in 0..last {
        let next: Vec<Point3<T>> = if slab + 1 == last {
            last_outer.to_vec()
        } else {
            outer_world::<T>(&geometry.canonical[slab + 1], &places[slab + 1])
                .ok_or(LoftError::SectionStructure)?
        };
        if base.is_empty() || next.len() != base.len() {
            return Err(LoftError::SectionStructure);
        }
        let mut d = Vec3::new(T::zero(), T::zero(), T::zero());
        for (qt, qb) in next.iter().zip(&base) {
            d = d + (*qt - *qb);
        }
        let base_normal = places[slab].map(T::from_f64).linear.c2;
        #[allow(clippy::cast_precision_loss)]
        let margin = d.dot(base_normal) / T::from_f64(next.len() as f64);
        match geom_core::k_stats::decide("loft_stacking", Margin::of(margin), band)
            .map_err(|source| LoftError::StackingEscalated { slab, source })?
        {
            Sign::Positive => {}
            Sign::Zero => return Err(LoftError::DegenerateStacking { slab }),
            Sign::Negative => return Err(LoftError::ReversedStacking { slab }),
        }
        base = next;
    }
    Ok(())
}

/// Assembles the loft BODY from its skinned geometry (module docs) —
/// the shared engine of [`loft_body`] and [`sweep_body`].
///
/// # Errors
///
/// [`LoftError`] — every door named on the enum.
fn assemble<T: Decide + geom_brep::PcurveFittedLane>(
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

    // ---- The stacking fold: every adjacent section pair decided
    // against ITS OWN base section's normal, in slab order. The end
    // sections' outer world loops are the ones walked just above, so
    // each section is traversed once for the whole assembly. ----
    stacking_fold::<T>(places, geometry, band, &bq[0], &tq[0])?;

    // ---- The declared joints' headings: per loop, the canonical
    // joints some section declares as a cusp (the extrude's step 7,
    // one verb over). Decided on the canonical `f64` sections, like
    // every other structural choice here (C6), sections in order. ----
    let cusps = declared_cusps(geometry, band)?;

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
    let seed = body.mvfs(qs[0])?;
    let mut hes = Vec::with_capacity(n);
    let first = body.mev(
        MevSite::Lone {
            r#loop: seed.r#loop,
        },
        qs[1 % n],
        placed_segment_spec(&outer[0], bplace, n_bottom, qs[0], qs[1 % n], tol),
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
    let forward = cap_points(outer, qs, bplace);
    let mut bottom_order: Vec<Point3<T>> = Vec::with_capacity(forward.len());
    if let Some(&p0) = forward.first() {
        bottom_order.push(p0);
    }
    for &p in forward.iter().skip(1).rev() {
        bottom_order.push(p);
    }
    let bottom_plane = newell_plane(&bottom_order, band).map_err(LoftError::CapPlane)?;
    let close = body.mef(
        MefSite::Chords {
            he1: prev.he_minus,
            he2: first.he_plus,
        },
        placed_segment_spec(&outer[n - 1], bplace, n_bottom, qs[n - 1], qs[0], tol),
        FaceSurface::New(bottom_plane),
        tol,
    )?;
    hes.push(close.he_plus);
    let top_face = seed.face;
    let bottom_face = close.face;
    let bottom_surface = body
        .get_face(bottom_face)
        .ok_or(EulerOpError::StaleKey {
            key: topo::EntityId::Face(bottom_face),
        })?
        .surface;
    let mut bases: Vec<Vec<topo::HalfEdgeKey>> = Vec::with_capacity(bloops.len());
    bases.push(hes);

    // ---- Phase 2: holes (rings in the seed face + kfmrh into the
    // bottom cap) — extrude's phase verbatim, loft rim specs. ----
    let anchor = bases[0][0];
    for (li, segs) in bloops.iter().enumerate().skip(1) {
        let hq = &bq[li];
        let m = segs.len();
        let bridge = body.mev_line(
            MevSite::Fan {
                he1: anchor,
                he2: anchor,
            },
            hq[0],
            tol,
        )?;
        let ring = body.kemr(bridge.he_plus, bridge.he_minus)?.ring;
        let mut hole_hes = Vec::with_capacity(m);
        let first = body.mev(
            MevSite::Lone { r#loop: ring },
            hq[1 % m],
            placed_segment_spec(&segs[0], bplace, n_bottom, hq[0], hq[1 % m], tol),
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
            FaceSurface::Shared(bottom_surface),
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
                FaceSurface::New(Surface::Nurbs(Arc::clone(&walls_t[li][j]))),
                tol,
            )?;
            if j == 0 {
                first_top = Some(mef.he_plus);
            }
            faces.push(mef.face);
            rims.push(
                body.get_half_edge(mef.he_plus)
                    .ok_or(EulerOpError::StaleKey {
                        key: topo::EntityId::HalfEdge(mef.he_plus),
                    })?
                    .edge,
            );
        }
        side_faces.push(faces);
        seam_edges.push(struts.iter().map(|s| s.edge).collect());
        top_rims.push(rims);
    }

    // ---- Phase 5: the swept seed face survives as the top cap. ----
    let far_loop = cap_points(&tloops[0], &tq[0], tplace);
    let top_plane = newell_plane(&far_loop, band).map_err(LoftError::CapPlane)?;
    body.set_face_surface(top_face, FaceSurface::New(top_plane))?;

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
            let wall_key = face_surface_key(&body, side_faces[li][j])?;
            let carrier = geom_brep::boundary_iso_u(walls_t[li][j].as_ref(), false)
                .map_err(|source| LoftError::SeamStructure { source })?;
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

    // Loft never reverses a traversal, so `side_faces` is already in
    // canonical segment order.
    let declared_contacts = cusps
        .iter()
        .zip(&side_faces)
        .flat_map(|(joints, walls)| {
            crate::swept::cusp_contacts(joints, walls.len(), |s| Some(walls[s]))
        })
        .collect();
    Ok(Lofted {
        body: built,
        solid: seed.solid,
        shell: seed.shell,
        top: top_face,
        bottom: bottom_face,
        side_faces,
        seam_edges,
        section_params: geometry.section_params.clone(),
        declared_contacts,
    })
}

/// Per canonical loop, ascending, the joints at which SOME section
/// declares a cusp ([`crate::swept::declared_cusp_joints`]).
///
/// A union and not a per-section list, because the wall pair is one
/// pair along the whole loft: sections are paired by canonical index,
/// so joint `v` of every section lies on the seam between walls
/// `v − 1` and `v`. A section that declares the joint smooth, or does
/// not declare it, takes nothing away from a section that declares it
/// a cusp — the declaration is about the pair, and tier 3 reads it
/// only where the wedge closes.
fn declared_cusps(geometry: &LoftGeometry, band: Band) -> Result<Vec<Vec<usize>>, LoftError> {
    let mut cusps: Vec<Vec<usize>> = vec![Vec::new(); geometry.walls.len()];
    for (section, profile) in geometry.canonical.iter().enumerate() {
        for (li, lp) in profile.loops().iter().enumerate() {
            let joints = crate::swept::declared_cusp_joints(lp, li, band).map_err(|e| {
                LoftError::CuspHeadingEscalated {
                    section,
                    loop_index: e.loop_index,
                    vertex_index: e.vertex_index,
                    source: e.source,
                }
            })?;
            cusps
                .get_mut(li)
                .ok_or(LoftError::SectionStructure)?
                .extend(joints);
        }
    }
    for joints in &mut cusps {
        joints.sort_unstable();
        joints.dedup();
    }
    Ok(cusps)
}

/// **The loft body** (M6-PLAN unit 3, spec §1): skins
/// [`loft_geometry`] and assembles the closed solid around it.
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
/// first ([`loft_geometry`], "The correspondence is the author's").
/// **A section rotated relative to its neighbour rolls the body by the
/// angle you authored**: the turning-orientation suite's authored-roll
/// row lofts a square onto the same square rotated by `theta` about its
/// own centre, each written from the image of the other's start, and
/// the body rolls by `theta`. To change the twist, start the section at
/// a different vertex.
///
/// **And the vertex order decides more than the pairing**: the whole
/// surface's v-parameterization is the FIRST STRIP's, so a section
/// spelled from a different starting vertex — or rolled about its own
/// normal by a symmetry that leaves its ring pointwise identical —
/// builds a different body. [`loft_geometry`]'s comment at the
/// parameterization is the statement of it.
///
/// `places[i]` is the caller's. For the plane normal to a curve at a
/// point, `geom_core::linalg::frame::path_start_frame(point, tangent,
/// tol)` is the door that hands one out, and a different roll is a
/// rotation composed about the tangent onto it.
///
/// # Errors
///
/// [`LoftError`] — every door named on the enum.
pub fn loft_body<T: Decide + geom_brep::PcurveFittedLane>(
    sections: &[Section],
    places: &[Affine3<f64>],
    v_degree: usize,
    tol: Tol,
) -> Result<Lofted<T>, LoftError> {
    let geometry = loft_geometry(sections, places, v_degree, tol).map_err(LoftError::Skin)?;
    assemble(places, &geometry, tol)
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
/// [`Lofted::side_faces`] is keyed in, and, through the first strip,
/// the surface's v-parameterization ([`loft_body`]). The body's roll
/// comes from the path frame ([`sweep_places`]), not from the
/// sections.
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
pub fn sweep_body<T: Decide + geom_brep::PcurveFittedLane>(
    profile: &[ProfileLoop<f64>],
    place: Affine3<f64>,
    path: &geom::NurbsCurve3<f64>,
    stations: usize,
    v_degree: usize,
    tol: Tol,
) -> Result<Lofted<T>, LoftError> {
    let places = sweep_places(place, path, stations).map_err(LoftError::Skin)?;
    let sections: Vec<Section> = core::iter::repeat_n(profile.to_vec(), places.len()).collect();
    let geometry = loft_geometry(&sections, &places, v_degree, tol).map_err(LoftError::Skin)?;
    assemble(&places, &geometry, tol)
}
