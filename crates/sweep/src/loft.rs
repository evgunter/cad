//! **The loft/sweep BODY assembly** (M6-3, executing the design
//! banked as M5 PR 9c item 6).
//!
//! The topology is EXTRUDE'S with different geometry (item 6(i)):
//! bottom cap from section 0, top cap from section k−1, one NURBS wall
//! per profile segment ([`crate::skin::LoftGeometry`]), wall–wall
//! seams as the walls' `u ∈ {0, 1}` boundary iso-curves, struts raised
//! per vertex. The three edge classes:
//!
//! - **Cap–wall rims** need NOTHING new (item 6(ii)): the wall's
//!   `v = 0` / `v = 1` iso IS the placed sketch segment (degree
//!   elevation and knot refinement are exact), so the carrier stays
//!   `Curve3::Line`/`Circle` under `MappedCurve::PlacedSegment` and
//!   certifies today.
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
//! Sections must stack ALONG the base section's plane normal (the
//! `loft_stacking` trilean): the caps then orient exactly as extrude's
//! (bottom reversed, top forward), and every wall's chart normal
//! `S_u × S_v` points out of the material — the u direction follows
//! the material-left profile traversal and v the stacking, so
//! `t̂ × v̂` is material-right — giving `sense = true` on every wall,
//! holes and concave arcs included (unlike a cylinder chart, the
//! skinned chart's normal FOLLOWS the traversal; there is no
//! unconditionally-radial frame to fight). A definitely-reversed
//! stacking refuses typed with the reorder recourse
//! ([`LoftError::ReversedStacking`]); a sliver stacking escalates.
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
use geom_core::{
    Affine3, Band, BandError, Decide, Indeterminate, Margin, Point3, Real, Sign, Tol, Vec3,
};
use profile::{
    Profile, ProfileError, ProfileLoop, ProfileVertex, RawLoop, SketchPlane, ValidatedProfile,
};
use topo::{
    Body, EdgeKey, EulerOpError, FaceKey, FaceSurface, MefSite, MevCreated, MevSite,
    PcurveMintError, ShellKey, SolidKey,
};

use crate::skin::{LoftGeometry, Section, SkinError, lift_surface, loft_geometry, sweep_places};
use crate::swept::{SweptSeg, cap_points, face_surface_key, placed_segment_spec, swept_segments};

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
}

/// Typed refusal of the loft/sweep body assembly (closed enum, D4 ¶3).
#[derive(Debug)]
pub enum LoftError {
    /// The run's tolerance could not form a classification band.
    Band(BandError),
    /// The §10.3/§10.4 geometry construction refused (compatibility,
    /// degree, interpolation — every [`SkinError`] reason).
    Skin(SkinError),
    /// An end section failed profile validation — a section that would
    /// not extrude does not loft either (the wire-layer door, run
    /// here so the library API is gated identically).
    Profile(ProfileError),
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
    SeamStructure,
    /// The end sections' loop/segment structure disagrees with the
    /// skinned geometry's — unreachable when both come from the same
    /// inputs; surfaced rather than swallowed.
    SectionStructure,
    /// The sections definitely stack AGAINST the base section's plane
    /// normal. The canonical assembly orients caps and walls by the
    /// forward stacking (module docs); reorder the sections (the loft
    /// of the reversed list is the same solid) rather than asking the
    /// builder to guess an orientation.
    ReversedStacking,
    /// The stacking displacement is coincident with zero at tolerance:
    /// a sliver-thin (or in-plane) loft.
    DegenerateStacking,
    /// The stacking classification escalated (named predicate on the
    /// diagnostic).
    StackingEscalated {
        /// The predicate-layer escalation.
        source: Indeterminate,
    },
}

impl fmt::Display for LoftError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Band(e) => write!(f, "loft: {e}"),
            Self::Skin(e) => write!(f, "loft geometry: {e}"),
            Self::Profile(e) => write!(f, "loft section profile: {e}"),
            Self::Euler(e) => write!(f, "loft assembly: {e}"),
            Self::CapPlane(e) => write!(f, "loft cap plane: {e}"),
            Self::Pcurve(e) => write!(f, "loft pcurve mint: {e}"),
            Self::SeamStructure => write!(
                f,
                "loft seam: a wall's boundary iso-curve failed to re-wrap (corrupt \
                 skinned surface — kernel bug, not an input fault)"
            ),
            Self::SectionStructure => write!(
                f,
                "loft assembly: end-section loop/segment structure disagrees with the \
                 skinned geometry (kernel bug, not an input fault)"
            ),
            Self::ReversedStacking => write!(
                f,
                "loft sections definitely stack AGAINST the base section's plane normal; \
                 reorder the sections (the reversed list lofts the same solid) — the \
                 builder orients caps and walls by forward stacking and does not guess"
            ),
            Self::DegenerateStacking => write!(
                f,
                "loft stacking displacement is coincident with zero at tolerance — a \
                 sliver-thin or in-plane loft has no orientable assembly"
            ),
            Self::StackingEscalated { source } => {
                write!(f, "loft stacking classification escalated: {source}")
            }
        }
    }
}

impl std::error::Error for LoftError {}

impl From<EulerOpError> for LoftError {
    fn from(e: EulerOpError) -> Self {
        Self::Euler(e)
    }
}

/// Exact `f64 → T` lift of a rigid placement (C6: the placement is
/// stored structure; `from_f64` is exact at every scalar).
fn lift_affine<T: Real>(a: &Affine3<f64>) -> Affine3<T> {
    let v =
        |w: geom_core::Vec3<f64>| Vec3::new(T::from_f64(w.x), T::from_f64(w.y), T::from_f64(w.z));
    Affine3 {
        linear: geom_core::Mat3 {
            c0: v(a.linear.c0),
            c1: v(a.linear.c1),
            c2: v(a.linear.c2),
        },
        translation: v(a.translation),
    }
}

/// One end section as a profile at `T` — the section IS a profile
/// now (LIB-U3), so this is the exact `f64 → T` embedding of its
/// loops (positions, bulges, and declared-tangent joints — the
/// declarations travel and are re-verified in the evaluation scalar)
/// run through the profile crate's own validation door: a section
/// that would not extrude does not loft either.
fn end_profile<T: Decide>(
    section: &Section,
    place: &Affine3<f64>,
    tol: Tol,
) -> Result<ValidatedProfile<T>, LoftError> {
    let loops = section
        .iter()
        .map(|lp| {
            ProfileLoop::new(
                lp.vertices()
                    .iter()
                    .map(|v| {
                        ProfileVertex::new(
                            geom_core::Point2::new(T::from_f64(v.pos().x), T::from_f64(v.pos().y)),
                            T::from_f64(v.bulge()),
                        )
                    })
                    .collect(),
            )
            .with_tangent_joints(lp.tangent_joints().to_vec())
        })
        .collect();
    Profile::new(SketchPlane::new(lift_affine(place)), loops)
        .validate(tol)
        .map_err(LoftError::Profile)
}

/// The world point of a sketch-plane point under a lifted placement.
fn world<T: Real>(place: &Affine3<T>, p: geom_core::Point2<T>) -> Point3<T> {
    place.transform_point(Point3::new(p.x, p.y, T::zero()))
}

/// Assembles the loft BODY from its skinned geometry (module docs) —
/// the shared engine of [`loft_body`] and [`sweep_body`].
///
/// # Errors
///
/// [`LoftError`] — every door named on the enum.
#[allow(clippy::too_many_lines)] // one construction, kept whole like extrude's
fn assemble<T: Decide>(
    sections: &[Section],
    places: &[Affine3<f64>],
    geometry: &LoftGeometry,
    tol: Tol,
) -> Result<Lofted<T>, LoftError> {
    let band = Band::linear(tol).map_err(LoftError::Band)?;
    let (Some(sec_bottom), Some(sec_top), Some(place_bottom), Some(place_top)) = (
        sections.first(),
        sections.last(),
        places.first(),
        places.last(),
    ) else {
        return Err(LoftError::SectionStructure);
    };
    let bottom_profile: ValidatedProfile<T> = end_profile(sec_bottom, place_bottom, tol)?;
    let top_profile: ValidatedProfile<T> = end_profile(sec_top, place_top, tol)?;
    let bplace: Affine3<T> = lift_affine(place_bottom);
    let tplace: Affine3<T> = lift_affine(place_top);
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
    if bloops.len() != tloops.len()
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

    // ---- The stacking trilean (module docs): the mean top-vertex
    // displacement of the outer loop against the base normal. ----
    let mut d = Vec3::new(T::zero(), T::zero(), T::zero());
    for (qt, qb) in tq[0].iter().zip(&bq[0]) {
        d = d + (*qt - *qb);
    }
    #[allow(clippy::cast_precision_loss)]
    let margin = d.dot(n_bottom) / T::from_f64(tq[0].len() as f64);
    match geom_core::k_stats::decide("loft_stacking", Margin::of(margin), band)
        .map_err(|source| LoftError::StackingEscalated { source })?
    {
        Sign::Positive => {}
        Sign::Zero => return Err(LoftError::DegenerateStacking),
        Sign::Negative => return Err(LoftError::ReversedStacking),
    }

    // ---- Lifted walls, kept once: face surfaces AND seam carriers
    // read the same lifted structure (D9 — one lift, shared bits). ----
    let mut walls_t: Vec<Vec<Arc<NurbsSurface<T>>>> = Vec::with_capacity(geometry.walls.len());
    for loop_walls in &geometry.walls {
        let mut lifted = Vec::with_capacity(loop_walls.len());
        for w in loop_walls {
            lifted.push(Arc::new(
                lift_surface::<T>(w).map_err(|e| LoftError::Skin(SkinError::Structure(e)))?,
            ));
        }
        walls_t.push(lifted);
    }

    // ---- Phase 1: bottom lamina (the extrude shape: the seed face's
    // chain is minted at the BOTTOM vertices and survives as the TOP
    // cap after the sweep; the mef'd close face is the bottom cap,
    // its loop reversed so the outward normal opposes stacking). ----
    let outer = &bloops[0];
    let qs = &bq[0];
    let n = outer.len();
    let mut body = Body::<T>::new();
    let seed = body.mvfs(qs[0])?;
    let mut hes = Vec::with_capacity(n);
    let first = body.mev(
        MevSite::Lone {
            r#loop: seed.r#loop,
        },
        qs[1 % n],
        placed_segment_spec(&outer[0], bplace, n_bottom, qs[0], qs[1 % n]),
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
            placed_segment_spec(&outer[j - 1], bplace, n_bottom, qs[j - 1], qs[j]),
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
        placed_segment_spec(&outer[n - 1], bplace, n_bottom, qs[n - 1], qs[0]),
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
            placed_segment_spec(&segs[0], bplace, n_bottom, hq[0], hq[1 % m]),
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
                placed_segment_spec(&segs[j - 1], bplace, n_bottom, hq[j - 1], hq[j]),
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
            placed_segment_spec(&segs[m - 1], bplace, n_bottom, hq[m - 1], hq[0]),
            FaceSurface::Shared(bottom_surface),
            tol,
        )?;
        hole_hes.push(close.he_plus);
        body.kfmrh(bottom_face, close.face)?;
        bases.push(hole_hes);
    }

    // ---- Phases 3–4: raise struts and close the wall quads, per
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
                placed_segment_spec(&tsegs[j], tplace, n_top, top_q_from, top_q_to),
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
    let raised = cap_points(&tloops[0], &tq[0], tplace);
    let top_plane = newell_plane(&raised, band).map_err(LoftError::CapPlane)?;
    body.set_face_surface(top_face, FaceSurface::New(top_plane))?;

    // ---- Phase 6: strut upgrades to the seam class — the wall keys
    // now exist, so each strut re-describes as wall j's `u = 0`
    // boundary iso through the certified setter (D9 — loops in
    // canonical order, vertices in swept order). ----
    for (li, seams) in seam_edges.iter().enumerate() {
        let n = seams.len();
        for j in 0..n {
            let wall_key = face_surface_key(&body, side_faces[li][j])
                .map_err(|_| LoftError::SectionStructure)?;
            let carrier = geom_brep::boundary_iso_u(walls_t[li][j].as_ref(), false)
                .map_err(|_| LoftError::SeamStructure)?;
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

    #[cfg(debug_assertions)]
    debug_assert_eq!(
        topo::validate_closed(&body),
        Ok(()),
        "loft postcondition: result is not tier-2 valid (kernel bug)",
    );

    Ok(Lofted {
        body,
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
/// [`loft_geometry`] and assembles the closed solid around it.
///
/// `sections[i][l][j]` is section `i`, loop `l`, segment `j` in sketch
/// coordinates; `places[i]` its rigid placement; `v_degree` the
/// skinning degree in the section direction. Structure is `f64`
/// (C6); the produced body is at `T`, lifted exactly.
///
/// # Errors
///
/// [`LoftError`] — every door named on the enum.
pub fn loft_body<T: Decide>(
    sections: &[Section],
    places: &[Affine3<f64>],
    v_degree: usize,
    tol: Tol,
) -> Result<Lofted<T>, LoftError> {
    let geometry = loft_geometry(sections, places, v_degree, tol).map_err(LoftError::Skin)?;
    assemble(sections, places, &geometry, tol)
}

/// **The path-swept body** (§10.4 as a solid): places rigid copies of
/// `profile` along `path` ([`crate::sweep_geometry`]'s frame — same
/// machinery, no fork) and assembles the loft of those sections.
///
/// # Errors
///
/// [`LoftError`] — every door named on the enum, with
/// [`SkinError::PathTangentReversal`] arriving through
/// [`LoftError::Skin`].
pub fn sweep_body<T: Decide>(
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
    assemble(&sections, &places, &geometry, tol)
}
