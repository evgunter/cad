//! **Per-half-edge pcurve caches**: minting, the face-level one-branch
//! walk, and the derive-on-demand accessor (M5 PR 6; C4).
//!
//! The cache *value* and its certification gate live in
//! [`geom_brep::pcurve_cache`]; this module is everything that needs
//! half-edges, loops, and faces — i.e. everything about **where** a
//! pcurve lives and **which branch** it takes.
//!
//! # The half-edge IS the key (spec §1)
//!
//! A pcurve belongs to an (edge, face-side) incidence, and the
//! half-edge is exactly that incidence. Seam edges are the forcing
//! case and the reason no coarser key works: a full cylinder wall
//! closed by its seam meridian has **both half-edges of one edge in
//! one loop of one face**, on one surface, with two different chart
//! curves (`u = α` and `u = α + 2π`). "Per edge" cannot hold two;
//! "per (edge, face)" cannot hold two either. Per half-edge has no
//! special case — [`crate::Body::pcurve`].
//!
//! # What gets minted, and what deliberately does not
//!
//! - **Planar faces store nothing.** M2's derive-on-demand status
//!   stands (C4 verbatim): a plane chart is affine, so its pcurve is an
//!   exact closed form with no point inversion to re-run, and C4's own
//!   argument for storing (avoiding a hidden iterative inversion per
//!   query) does not apply. An all-planar body therefore carries **zero
//!   stored pcurves** — pinned by test.
//! - **Cylinder charts mint.** They are what the C5 table's
//!   Plane×Cylinder splitting lane produces, and every carrier that
//!   lane mints (rim circle, seam/meridian line, tilted-section conic)
//!   has an exact closed-form cylinder-chart image.
//! - **Cone / sphere / torus charts mint (M6-3, walk row 4).** Their
//!   closed-form classes (cone rims/rulings; sphere polar/meridian
//!   circles; torus parallels/meridians) derive and certify exactly as
//!   the cylinder's; the sphere walk additionally knows the chart's
//!   involution twin and the pole's zero azimuth lever (see
//!   [`azimuth_arm`]/`sphere_twin`). Carriers OUTSIDE the closed-form
//!   classes refuse typed with the class named — the sphere's general
//!   circles route through the fitted lane
//!   ([`geom_brep::PcurveCache::certify_fitted`]), the cone/torus
//!   oblique classes have no honest route (no ring-computable meters
//!   composite) and stay refused.
//! - **Described non-rational NURBS charts mint** their iso lane
//!   (M6-3, `nurbs_iso_derive`); the placeholder and rational walls
//!   mint nothing.
//!
//! # The one-branch walk (spec §3)
//!
//! A [`geom_brep::Pcurve`] cannot express a branch jump: its azimuth
//! channel is `α + β·t` with one stored `α`. What remains is *which*
//! branch `α + kτ` each half-edge of a face takes, and that is decided
//! **once per loop**, by walking the loop in `next` order and pinning
//! each half-edge's entry chart point to the previous half-edge's exit
//! chart point. The pinning is then **certified**: the two chart points
//! must agree within ε *through the map* (azimuth metered at the chart
//! radius, height directly), or the walk refuses typed
//! ([`PcurveMintError::LoopDiscontinuity`]). The loop must also close
//! — with a total azimuth advance of zero, or exactly one period for a
//! loop that wraps the chart (the seam case).
//!
//! This is the M2 PR 5 meridian finding generalized (`docs/M2-LOG.md`:
//! "the junction's meridian column unwrapped nearest prev_u, but past
//! 3π/2 the wrong branch is closer"). The fix there was to anchor the
//! unwrap to a point that is *exact by construction* rather than
//! nearest-previous per sample; here the anchor is the loop's own
//! chain of shared vertices, the per-sample choice does not exist at
//! all, and the anchor's correctness is checked rather than assumed.
//!
//! # Persistence and transfer posture
//!
//! Caches are minted at construction and are immutable with the body;
//! there is no invalidation machinery and none is needed (content-keyed
//! cache transfer stays banked, C4). Persistence (M4 PR 6 / D6.1) is
//! **recipe-level**: a document stores its edit list, and loading
//! re-evaluates it — so a round-trip **re-mints** pcurves from the same
//! deterministic pipeline rather than reading stored bytes, and no
//! pcurve-shaped field enters the schema. [`crate::transform`] takes
//! the same posture as it already does for carriers and witnesses:
//! construction-fresh re-derivation against the mapped geometry, never
//! a mapped stored cache.
//!
//! ## Stale rows: which ops maintain this map, and which do not
//!
//! Read this before storing a cache from a new call site. **Only two
//! ops maintain the map**: the splitting lane (which runs
//! [`mint_pcurves`] on each side it produces, and that pass CLEARS the
//! map before re-minting) and [`crate::transform`] (which re-derives
//! when the operand carried caches). **Every other op — the Euler
//! operators, the kill ops, ring surgery, `merge_coplanar_faces`,
//! `split_edge`, the boolean graft — neither clears nor re-mints.**
//!
//! The consequence is bounded but real: a `SecondaryMap` row outlives
//! its key until the slot is reused, so surgery on a body that already
//! carries caches can leave a row attached to a half-edge that no
//! longer means what the cache says (or, once a slot is recycled, to a
//! different half-edge entirely). Nothing at M5 reaches that state on
//! the ship path — caches are minted last, at the end of the split
//! pipeline, and the pass clears first — and the tier-3 pcurve pass
//! catches it LOUD if it ever happens: a stale row re-certifies against
//! the current carrier/surface/window and fails, or breaks its face
//! loop's continuity. So the posture is fail-loud, not silent-wrong —
//! but an op that starts mutating already-minted bodies must either
//! clear the map or re-mint, and should say which in its own docs.

use geom_brep::{
    ChartWindow, Pcurve, PcurveCache, PcurveCertifyError, PcurveFittedLane, chart_pcurve,
};
use geom_core::k_stats::decide;
use geom_core::predicate::{Band, BandError};
use geom_core::{Decide, Indeterminate, Real, Sign};
use geom_surfaces::Surface;

use crate::body::Body;
use crate::entity::{FaceKey, HalfEdgeKey, LoopKey};
use crate::null::CurveGeom;

/// Typed refusal of the pcurve minting pass (D4 ¶3).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PcurveMintError {
    /// A key failed to resolve mid-pass — a structurally corrupt body
    /// (tier 1's job to report; this pass only refuses to guess).
    Corrupt,
    /// A half-edge's pcurve failed certification at mint.
    Certify {
        /// The half-edge whose cache was refused.
        half_edge: HalfEdgeKey,
        /// The typed certification failure, nested whole.
        error: PcurveCertifyError,
    },
    /// The chart image of a half-edge does not meet its predecessor's
    /// in the chart: the loop's one-branch unwrap is not continuous
    /// there. Typed, never patched by picking the nearest branch (the
    /// M2 PR 5 finding — module docs).
    LoopDiscontinuity {
        /// The half-edge whose entry point did not meet its
        /// predecessor's exit point.
        half_edge: HalfEdgeKey,
    },
    /// A loop's chart walk did not close: after one full traversal the
    /// azimuth advance is neither zero nor exactly one period, or the
    /// height does not return.
    LoopNotClosed {
        /// The face whose loop failed to close.
        face: FaceKey,
    },
    /// A half-edge carries no stored pcurve at rest although its face
    /// carries some: the face's cache set is incomplete. A face with
    /// NO caches is legal everywhere (planar faces, frontier charts,
    /// and any body that never ran the minting pass); a half-minted
    /// one is a defect.
    MissingCache {
        /// The half-edge with no stored cache.
        half_edge: HalfEdgeKey,
    },
    /// A classification escalated (sliver band or poison).
    Escalated {
        /// The half-edge under classification.
        half_edge: HalfEdgeKey,
        /// The classifier's diagnostic.
        cause: Indeterminate,
    },
    /// The run's linear band could not be built.
    Band(BandError),
}

impl core::fmt::Display for PcurveMintError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Corrupt => write!(
                f,
                "pcurve minting: the body is structurally corrupt (a key did not resolve)"
            ),
            Self::Certify { half_edge, error } => {
                write!(f, "pcurve minting at half-edge {half_edge:?}: {error}")
            }
            Self::LoopDiscontinuity { half_edge } => write!(
                f,
                "pcurve minting: half-edge {half_edge:?} does not meet its predecessor in \
                 the chart — the loop's single-branch unwrap is discontinuous there \
                 (a branch is chosen once per loop and certified, never per sample)"
            ),
            Self::LoopNotClosed { face } => write!(
                f,
                "pcurve minting: the chart walk of a loop of face {face:?} did not close \
                 (its azimuth advance is neither zero nor one full period)"
            ),
            Self::MissingCache { half_edge } => write!(
                f,
                "pcurve minting: half-edge {half_edge:?} bounds a face whose chart mints \
                 pcurve caches, but carries none at rest"
            ),
            Self::Escalated { half_edge, cause } => write!(
                f,
                "pcurve minting at half-edge {half_edge:?} escalated: {cause}"
            ),
            Self::Band(e) => write!(f, "pcurve minting: {e}"),
        }
    }
}

impl std::error::Error for PcurveMintError {}

/// Does this chart kind mint stored caches (module docs)? A
/// compile-time routing decision per surface kind, exhaustively
/// matched — adding a kind is a compiler-guided edit (D3).
fn chart_mints<T: Real>(surface: &Surface<T>) -> bool {
    match surface {
        Surface::Cylinder { .. } => true,
        // Planar faces keep M2's derive-on-demand status (C4 verbatim).
        Surface::Plane { .. } => false,
        // The analytic-chart completion (M6-3, walk row 4): cone,
        // sphere and torus charts certify their closed-form classes
        // (rim/ruling; polar/meridian; parallel/meridian) and mint
        // stored caches wherever the pass runs.
        Surface::Cone { .. } | Surface::Sphere { .. } | Surface::Torus { .. } => true,
        // NON-RATIONAL described NURBS charts mint (M6-3): every
        // loft/sweep wall boundary is an iso-parameter curve whose
        // chart image is an exact line in UV (`Pcurve::IsoLine`).
        // The placeholder mints nothing (it is not a described
        // surface), and RATIONAL walls mint nothing either — the iso
        // lane's hull bounds are polynomial convexity facts, and a
        // rational-wall body already refuses tier 3 at the volume
        // door with recourse text naming the banked rational lane;
        // minting here would only move that refusal somewhere less
        // actionable.
        Surface::Nurbs(payload) => {
            !payload.is_placeholder() && payload.weights().iter().all(|w| *w == 1.0)
        }
    }
}

/// The pcurve of `half_edge` on its own face's chart — the **stored
/// cache** when there is one, otherwise **derived on demand**.
///
/// The derived answer is on the chart's *principal* branch (there is no
/// loop context in a single-half-edge query); a caller that needs a
/// face-consistent branch reads the stored cache or walks the loop
/// through [`mint_pcurves`]. For the affine (plane) charts that are the
/// standing derive-on-demand case this distinction is vacuous — a plane
/// chart has no branches.
///
/// # Errors
///
/// [`PcurveMintError`] — a corrupt key, or a typed chart refusal for a
/// frontier chart/carrier kind.
pub fn pcurve_of<T: Decide>(
    body: &Body<T>,
    half_edge: HalfEdgeKey,
    band: Band,
) -> Result<Pcurve<T>, PcurveMintError> {
    if let Some(cache) = body.pcurve(half_edge) {
        return Ok(cache.pcurve().clone());
    }
    let (carrier, _, _) = half_edge_carrier(body, half_edge)?;
    let surface = half_edge_surface(body, half_edge)?;
    if matches!(surface, Surface::Nurbs(_)) {
        // The NURBS chart's images are description-driven (M6-3) —
        // the iso derivation, not the closed-form harmonic table.
        return nurbs_iso_derive(body, half_edge, &surface, band);
    }
    chart_pcurve(&carrier, &surface, band)
        .map_err(|error| PcurveMintError::Certify { half_edge, error })
}

/// The certified carrier and parameter interval of `half_edge`'s edge.
fn half_edge_carrier<T: Decide>(
    body: &Body<T>,
    half_edge: HalfEdgeKey,
) -> Result<(geom_curves::Curve3<T>, T, T), PcurveMintError> {
    let he = body
        .get_half_edge(half_edge)
        .ok_or(PcurveMintError::Corrupt)?;
    let edge = body.get_edge(he.edge).ok_or(PcurveMintError::Corrupt)?;
    let Some(CurveGeom::Certified(curve)) = body.get_curve_geom(edge.curve) else {
        return Err(PcurveMintError::Corrupt);
    };
    let (t0, t1) = curve.params();
    Ok((curve.carrier().clone(), t0, t1))
}

/// The **mate operand** a fitted (rung-3) pcurve's certificate needs:
/// the other surface of the pair whose intersection minted the carrier
/// (`geom_brep::PcurveCache::certify_fitted` — the uniqueness tube is a
/// statement about the PAIR, so one surface cannot produce one).
///
/// It is read from the edge's **intensional description**, not from the
/// topology, and that is the D2 answer rather than a convenience: the
/// description is what is authoritative about which two surfaces the
/// locus belongs to (`EdgeGeometry::Intersection { s1, s2 }` names them
/// by key), while "the face across the edge" is a derived fact that a
/// mid-construction body, a spur edge or a seam can perfectly well have
/// wrong. Re-read from the body at rest, never stored with the cache,
/// so it cannot drift from the body's own geometry.
///
/// `None` when the edge is not an intersection edge, or when the face's
/// own surface is neither of the pair — both of which the fitted lane
/// then refuses typed rather than inventing a second operand.
fn mate_surface<T: Decide>(body: &Body<T>, half_edge: HalfEdgeKey) -> Option<Surface<T>> {
    let he = body.get_half_edge(half_edge)?;
    let edge = body.get_edge(he.edge)?;
    let CurveGeom::Certified(curve) = body.get_curve_geom(edge.curve)? else {
        return None;
    };
    let geom_brep::EdgeGeometry::Intersection { s1, s2, .. } = *curve.description() else {
        return None;
    };
    let lp = body.get_loop(he.parent_loop)?;
    let own = body.get_face(lp.face)?.surface;
    let other = if own == s1 {
        s2
    } else if own == s2 {
        s1
    } else {
        return None;
    };
    body.get_surface(other).cloned()
}

/// The surface of the face `half_edge` bounds.
fn half_edge_surface<T: Decide>(
    body: &Body<T>,
    half_edge: HalfEdgeKey,
) -> Result<Surface<T>, PcurveMintError> {
    let he = body
        .get_half_edge(half_edge)
        .ok_or(PcurveMintError::Corrupt)?;
    let lp = body
        .get_loop(he.parent_loop)
        .ok_or(PcurveMintError::Corrupt)?;
    let face = body.get_face(lp.face).ok_or(PcurveMintError::Corrupt)?;
    body.get_surface(face.surface)
        .cloned()
        .ok_or(PcurveMintError::Corrupt)
}

/// The surface KEY of the face `half_edge` bounds (the key twin of
/// [`half_edge_surface`], for the iso derivation's own-side test).
fn half_edge_surface_key<T: Decide>(
    body: &Body<T>,
    half_edge: HalfEdgeKey,
) -> Result<geom_brep::SurfaceKey, PcurveMintError> {
    let he = body
        .get_half_edge(half_edge)
        .ok_or(PcurveMintError::Corrupt)?;
    let lp = body
        .get_loop(he.parent_loop)
        .ok_or(PcurveMintError::Corrupt)?;
    let face = body.get_face(lp.face).ok_or(PcurveMintError::Corrupt)?;
    Ok(face.surface)
}

/// The certified description of `half_edge`'s edge.
fn half_edge_description<T: Decide>(
    body: &Body<T>,
    half_edge: HalfEdgeKey,
) -> Result<geom_brep::EdgeGeometry<T>, PcurveMintError> {
    let he = body
        .get_half_edge(half_edge)
        .ok_or(PcurveMintError::Corrupt)?;
    let edge = body.get_edge(he.edge).ok_or(PcurveMintError::Corrupt)?;
    let Some(CurveGeom::Certified(curve)) = body.get_curve_geom(edge.curve) else {
        return Err(PcurveMintError::Corrupt);
    };
    Ok(*curve.description())
}

/// Derives the **exact iso-line chart image** of `half_edge` on a
/// described NURBS chart (M6-3) — the NURBS-chart counterpart of
/// `geom_brep::chart_pcurve`, driven by the edge's INTENSIONAL
/// description (D2: the description is what is authoritative about
/// which iso this locus is):
///
/// - An [`geom_brep::EdgeGeometry::IsoCurve`] naming THIS face's
///   surface maps directly: `P(t) = (u, v0 + slope·(t − t0))`.
/// - An `IsoCurve` naming the OTHER wall maps as this chart's own
///   `u = 0` or `u = 1` boundary, the side selected by a definite
///   endpoint residual (`pcurve_iso_side`) and then CERTIFIED by the
///   full iso lane — a wrong pick fails loudly, never silently.
/// - A cap–wall rim (`MappedCurve::PlacedSegment`, Line segment) maps
///   as `(u(t), v)` with `u` affine (`t0 ↦ 0`, `t1 ↦ 1` — the wall's
///   u IS the segment parameter by construction) and `v ∈ {0, 1}` by
///   the same endpoint selection.
/// - Everything else on a NURBS chart refuses typed with the class
///   named (arc rims: the chart u is the segment's rational-Bézier
///   parameter, banked with the rational-wall lane).
fn nurbs_iso_derive<T: Decide>(
    body: &Body<T>,
    half_edge: HalfEdgeKey,
    surface: &Surface<T>,
    band: Band,
) -> Result<Pcurve<T>, PcurveMintError> {
    use geom_core::{Point2, Vec2};
    let refuse = |what: &'static str| PcurveMintError::Certify {
        half_edge,
        error: PcurveCertifyError::IsoUnsupported { what },
    };
    let (carrier, t0, t1) = half_edge_carrier(body, half_edge)?;
    let span = t1 - t0;
    // A definite endpoint-side selection: which of the two candidate
    // chart values places the carrier's START on the surface. The
    // selection is structure (a two-way pick), the CHECK is the full
    // iso-lane certification that follows every derivation.
    let side_pick = |eval_at: &dyn Fn(T) -> geom_core::Point3<T>| -> Result<T, PcurveMintError> {
        let start = carrier.eval(t0);
        for cand in [T::zero(), T::one()] {
            match decide("pcurve_iso_side", start.distance(eval_at(cand)), band) {
                Ok(Sign::Zero) => return Ok(cand),
                Ok(Sign::Positive | Sign::Negative) => {}
                Err(cause) => {
                    return Err(PcurveMintError::Escalated { half_edge, cause });
                }
            }
        }
        Err(refuse(
            "the carrier's start point lies on neither chart boundary — not a boundary \
             iso of this face's chart",
        ))
    };
    match half_edge_description(body, half_edge)? {
        geom_brep::EdgeGeometry::IsoCurve {
            surface: sk,
            u,
            v0,
            v1,
        } => {
            let slope = (v1 - v0) / span;
            let p0y = v0 - slope * t0;
            let own = half_edge_surface_key(body, half_edge)?;
            let x = if sk == own {
                u
            } else {
                // The other wall's side of the seam: this chart's own
                // u-boundary, selected by the endpoint.
                side_pick(&|cand| surface.eval(cand, v0))?
            };
            Ok(Pcurve::IsoLine {
                p0: Point2::new(x, p0y),
                pl: Vec2::new(T::zero(), slope),
            })
        }
        geom_brep::EdgeGeometry::MappedCurve(geom_brep::MappedCurve::PlacedSegment {
            segment,
            ..
        }) => {
            if !matches!(segment, geom_brep::SketchSegment::Line { .. }) {
                return Err(refuse(
                    "an ARC cap rim on a NURBS chart: the chart's u is the segment's \
                     rational-Bézier parameter (not the arc angle) — banked with the \
                     rational-wall lane",
                ));
            }
            let plx = T::one() / span;
            let p0x = T::zero() - t0 / span;
            let v = side_pick(&|cand| surface.eval(p0x + plx * t0, cand))?;
            Ok(Pcurve::IsoLine {
                p0: Point2::new(p0x, v),
                pl: Vec2::new(plx, T::zero()),
            })
        }
        _ => Err(refuse(
            "no iso derivation for this description kind on a NURBS chart — only \
             IsoCurve seams and Line cap rims have exact line images (the trimmed-NURBS \
             pcurve lane is the cut-loft unit's)",
        )),
    }
}

/// Is `half_edge` the `he_plus` of its edge (so the loop traverses it
/// forward in the carrier parameter)?
fn is_plus<T: Decide>(body: &Body<T>, half_edge: HalfEdgeKey) -> Result<bool, PcurveMintError> {
    let he = body
        .get_half_edge(half_edge)
        .ok_or(PcurveMintError::Corrupt)?;
    let edge = body.get_edge(he.edge).ok_or(PcurveMintError::Corrupt)?;
    Ok(edge.he_plus == half_edge)
}

/// The azimuth lever arm of a chart (metres per radian) — how an
/// azimuth discrepancy is metered against the linear band (D4 ¶1: no
/// UV-space tolerance ever reaches ε).
///
/// NURBS charts take the unit arm deliberately: the iso lane's loop
/// corners are EXACT chart values by construction (`0`/`1` boundary
/// isos meeting cap rims whose affine maps are pinned at `t0 ↦ 0`,
/// `t1 ↦ 1`), so the continuity margins metered here are exactly zero
/// on every minted body and the arm never converts a real
/// displacement. The honest per-chart stretch arm exists
/// (`geom_brep`'s iso-lane certification uses it); threading it here
/// would change no decision on any mintable input.
/// The LOCAL azimuth lever arm of a chart at second-parameter value
/// `v` — the metres an azimuth radian moves the mapped point *at that
/// latitude*: cylinder `r`, sphere `|r·cos v|`, torus `|R + r·cos v|`,
/// cone `|v·sin α|`. This is the honest metering for a joint gap
/// (D4 ¶1): at a sphere pole or a cone apex the lever is exactly
/// zero, because the chart azimuth genuinely does not move the point
/// there — a loop meeting itself at a pole has no azimuth-continuity
/// obligation, and pretending otherwise (a global sup arm) refuses
/// every octant corner.
fn azimuth_arm<T: Real>(surface: &Surface<T>, v: T) -> T {
    match *surface {
        Surface::Cylinder { radius, .. } => radius,
        Surface::Sphere { radius, .. } => (radius * v.cos()).abs(),
        Surface::Torus {
            major_radius,
            minor_radius,
            ..
        } => (major_radius + minor_radius * v.cos()).abs(),
        Surface::Cone { half_angle, .. } => (v * half_angle.sin()).abs(),
        _ => T::one(),
    }
}

/// The meridional (second-channel) lever arm where that channel is
/// itself an angle — sphere `v` (arm `r`), torus `v` (arm `r_minor`).
/// `None` = the channel is a length and gaps in it are already metres.
fn polar_arm<T: Real>(surface: &Surface<T>) -> Option<T> {
    match *surface {
        Surface::Sphere { radius, .. } => Some(radius),
        Surface::Torus { minor_radius, .. } => Some(minor_radius),
        _ => None,
    }
}

/// A whole-period shift of the MERIDIONAL channel — the `v` twin of
/// [`geom_brep::Pcurve::shift_branch`], for the charts whose second
/// parameter is an angle (sphere/torus). Only the harmonic form lives
/// on those charts; other variants answer themselves unchanged (the
/// walk never computes a nonzero shift for them).
fn shift_polar_branch<T: Real>(pcurve: &Pcurve<T>, k: T, period: T) -> Pcurve<T> {
    match pcurve {
        Pcurve::Harmonic { p0, pa, pb, pl } => Pcurve::Harmonic {
            p0: geom_core::Point2::new(p0.x, p0.y + k * period),
            pa: *pa,
            pb: *pb,
            pl: *pl,
        },
        other => other.clone(),
    }
}

/// The sphere chart's INVOLUTION twin of a harmonic image:
/// `S(u + π, π − v) = S(u, v)` holds identically on a sphere chart
/// (`radial(u+π) = −radial(u)`, `cos(π−v) = −cos v`, `sin(π−v) =
/// sin v`), so every sphere pcurve has exactly two harmonic
/// representations and a pole-crossing walk legitimately needs the
/// OTHER one on the far side — a π azimuth step no whole-period shift
/// can produce. The torus has no such twin (R > 0 breaks the
/// symmetry), and neither does the cone (cos α > 0).
fn sphere_twin<T: Decide>(surface: &Surface<T>, pcurve: &Pcurve<T>) -> Option<Pcurve<T>> {
    if !matches!(surface, Surface::Sphere { .. }) {
        return None;
    }
    let Pcurve::Harmonic { p0, pa, pb, pl } = pcurve else {
        return None;
    };
    let pi = T::pi();
    Some(Pcurve::Harmonic {
        p0: geom_core::Point2::new(p0.x + pi, pi - p0.y),
        pa: geom_core::Vec2::new(pa.x, T::zero() - pa.y),
        pb: geom_core::Vec2::new(pb.x, T::zero() - pb.y),
        pl: geom_core::Vec2::new(pl.x, T::zero() - pl.y),
    })
}

/// The loop-closure test of a chart walk: does `end` denote the same
/// chart point as `start`, up to the chart's legitimate wraps?
///
/// - Every periodic chart may wrap the azimuth by one whole period
///   (the seam case).
/// - Where the second channel is an angle (sphere/torus) it may wrap
///   by one whole period too (a torus annulus's meridian walk).
/// - The SPHERE additionally closes through its involution
///   (`sphere_twin`): a pole-crossing loop's walk legitimately ends on
///   the twin representation of its start — azimuth off by π (mod τ)
///   with the polar channel mirrored (`end.y + start.y ≡ π mod τ`).
///
/// All margins metered in metres through the channel's lever arm; the
/// azimuth gap through the LOCAL arm at the meeting point
/// ([`azimuth_arm`] — zero at a pole, where azimuth means nothing).
fn loop_closes<T: Decide>(
    surface: &Surface<T>,
    start: geom_core::Point2<T>,
    end: geom_core::Point2<T>,
    band: Band,
) -> bool {
    let arm = azimuth_arm(surface, start.y);
    let tau = T::tau();
    let zero = |name: &'static str, m: T| matches!(decide(name, m, band), Ok(Sign::Zero));
    let wraps = |m: T, a: T, name: &'static str| {
        [m, m - tau, m + tau].into_iter().any(|c| zero(name, c * a))
    };
    let du = end.x - start.x;
    let dv = end.y - start.y;
    let direct = match polar_arm(surface) {
        None => wraps(du, arm, "pcurve_loop_closure") && zero("pcurve_loop_closure_height", dv),
        Some(v_arm) => {
            wraps(du, arm, "pcurve_loop_closure") && wraps(dv, v_arm, "pcurve_loop_closure_height")
        }
    };
    if direct {
        return true;
    }
    // The sphere involution arm (the azimuth gap still metered at the
    // local arm — a pole-closing walk has no azimuth obligation).
    let (Surface::Sphere { radius, .. }, false) = (surface, direct) else {
        return false;
    };
    let pi = T::pi();
    let mirrored_u = [du - pi, du + pi]
        .into_iter()
        .any(|m| wraps(m, arm, "pcurve_loop_closure"));
    let sv = end.y + start.y - pi;
    mirrored_u && wraps(sv, *radius, "pcurve_loop_closure_height")
}

/// One half-edge's minted chart curve, before certification.
struct Walked<T: Real> {
    half_edge: HalfEdgeKey,
    pcurve: Pcurve<T>,
    t0: T,
    t1: T,
}

/// Mints (and certifies) the pcurve caches of every curved face of
/// `body` whose chart has a certified closed-form lane — the pass the
/// C5 splitting lane runs on each side it produces (spec §1: caches are
/// minted where curved faces are minted).
///
/// Idempotent by construction on a given body: it walks faces in arena
/// order, loops in outer-then-rings order, and half-edges in `next`
/// order, and overwrites any existing row with the same derivation
/// (D9 — same body, same bits).
///
/// # Errors
///
/// [`PcurveMintError`] — a certification refusal, a discontinuous or
/// unclosed loop walk, or an escalated classification. Never a silent
/// skip of a face the lane covers.
pub fn mint_pcurves<T: Decide>(body: &mut Body<T>) -> Result<(), PcurveMintError> {
    let band = Band::linear().map_err(PcurveMintError::Band)?;
    // Start from empty. A body reaching this pass may have been carved
    // from a scratch clone that inherited rows for half-edges the
    // surgery killed (a `SecondaryMap` row outlives its key until the
    // slot is reused), and a stale cache is worse than no cache. What
    // this pass leaves behind is exactly what it minted and certified.
    body.pcurves.clear();
    let faces: Vec<FaceKey> = body.faces().map(|(k, _)| k).collect();
    for face in faces {
        mint_face(body, face, band)?;
    }
    Ok(())
}

/// Mints the caches of one face (module docs: the two-pass shape — walk
/// the loops to pin branches and build the chart window, then certify
/// every pcurve against that window).
fn mint_face<T: Decide>(
    body: &mut Body<T>,
    face: FaceKey,
    band: Band,
) -> Result<(), PcurveMintError> {
    let face_data = body.get_face(face).ok_or(PcurveMintError::Corrupt)?;
    let surface_key = face_data.surface;
    let loops: Vec<LoopKey> = core::iter::once(face_data.outer)
        .chain(face_data.rings.iter().copied())
        .collect();
    let surface = body
        .get_surface(surface_key)
        .cloned()
        .ok_or(PcurveMintError::Corrupt)?;
    if !chart_mints(&surface) {
        return Ok(());
    }
    let mut walked: Vec<Walked<T>> = Vec::new();
    for lp in loops {
        walk_loop(body, face, lp, &surface, band, &mut walked)?;
    }
    if walked.is_empty() {
        return Ok(());
    }
    // The face's chart window: the hull of its boundary's own
    // (conservative) chart boxes — the over-approximation of the trim
    // region the domain-validity limb certifies against (C4; the exact
    // point-in-region test is PR 11's tessellation consumer).
    //
    // **Where this limb has teeth, stated plainly.** At MINT time the
    // containment check is vacuous by construction: the window IS the
    // hull of exactly the boxes being checked against it, so no minted
    // pcurve can escape it. That is deliberate, not an oversight — a
    // freshly derived face has no independent prior notion of its own
    // trim region, and inventing one (say, the loop's vertex box) would
    // refuse legitimate faces whose boundary arcs bulge past their
    // endpoints. The limb bites at RE-certification
    // (`validate_pcurves`) and on the attach path, where the window
    // comes from the OTHER stored caches and from a body the checked
    // pcurve did not help build: a swapped, shifted or hand-attached
    // cache is then measured against a window it had no part in
    // setting. `TrimEscape` rows exercise exactly that direction.
    let mut window: Option<ChartWindow<T>> = None;
    for w in &walked {
        let b = w.pcurve.chart_box(w.t0, w.t1);
        window = Some(match window {
            None => b,
            Some(acc) => acc.hull(b),
        });
    }
    let Some(window) = window else {
        return Ok(());
    };
    for w in walked {
        let (carrier, _, _) = half_edge_carrier(body, w.half_edge)?;
        // The minting lane produces closed-form images only, so it goes
        // through the `Decide`-scalar door: a `Pcurve::Harmonic`
        // certifies an algebraic identity in which no second surface
        // takes part, and a dual body mints them perfectly well.
        let cache = PcurveCache::certify(
            w.pcurve.clone(),
            w.t0,
            w.t1,
            &carrier,
            &surface,
            window,
            band,
        )
        .map_err(|error| PcurveMintError::Certify {
            half_edge: w.half_edge,
            error,
        })?;
        body.pcurves.insert(w.half_edge, cache);
    }
    Ok(())
}

/// The one-branch walk of a single loop (module docs). Appends the
/// branch-pinned chart curves to `out`.
fn walk_loop<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    lp: LoopKey,
    surface: &Surface<T>,
    band: Band,
    out: &mut Vec<Walked<T>>,
) -> Result<(), PcurveMintError> {
    let loop_data = body.get_loop(lp).ok_or(PcurveMintError::Corrupt)?;
    let crate::entity::LoopBoundary::Cycle { first } = loop_data.boundary else {
        // An empty loop bounds nothing to chart.
        return Ok(());
    };
    let cycle = body.loop_cycle(first).ok_or(PcurveMintError::Corrupt)?;
    let tau = T::tau();
    // The walk's running exit point, in chart coordinates.
    let mut prev_exit: Option<geom_core::Point2<T>> = None;
    let mut first_entry: Option<geom_core::Point2<T>> = None;
    for he in cycle {
        let (carrier, t0, t1) = half_edge_carrier(body, he)?;
        let base = if matches!(surface, Surface::Nurbs(_)) {
            // NURBS charts derive from the edge's intensional
            // description (M6-3) — see `nurbs_iso_derive`.
            nurbs_iso_derive(body, he, surface, band)?
        } else {
            chart_pcurve(&carrier, surface, band).map_err(|error| PcurveMintError::Certify {
                half_edge: he,
                error,
            })?
        };
        let plus = is_plus(body, he)?;
        let (entry_t, exit_t) = if plus { (t0, t1) } else { (t1, t0) };
        // Gap metering: azimuth through the chart's lever arm; the
        // second channel directly where it is a length, through the
        // polar arm where it is an angle (sphere/torus, M6-3).
        let v_arm = polar_arm(surface);
        let v_meter = v_arm.unwrap_or_else(T::one);
        let pcurve = match prev_exit {
            None => base,
            Some(prev) => {
                // The ONE branch decision of this half-edge: the
                // representation (the derivation's own or, on a sphere
                // chart, its involution twin — `sphere_twin`) and the
                // whole number of periods per angular channel that
                // land its entry on the predecessor's exit. Exact by
                // construction — the two chart points denote the SAME
                // vertex — and CHECKED here, so a body where no
                // representation is exact refuses rather than snapping
                // to the nearest branch. Candidate order (base first)
                // is fixed: D9.
                let half = T::from_f64(0.5);
                let twin = sphere_twin(surface, &base);
                let mut chosen: Option<Pcurve<T>> = None;
                let candidates: Vec<Pcurve<T>> = core::iter::once(base).chain(twin).collect();
                let n = candidates.len();
                for (ci, cand) in candidates.into_iter().enumerate() {
                    let raw = cand.eval(entry_t);
                    let ku = ((prev.x - raw.x) / tau + half).floor();
                    // The impossible-rebuild arm (see
                    // `Pcurve::shift_branch`) surfaces as a
                    // corrupt-body finding rather than being swallowed
                    // into an unshifted branch.
                    let Some(mut shifted) = cand.shift_branch(ku, tau) else {
                        return Err(PcurveMintError::Corrupt);
                    };
                    if v_arm.is_some() {
                        let ry = shifted.eval(entry_t).y;
                        let kv = ((prev.y - ry) / tau + half).floor();
                        shifted = shift_polar_branch(&shifted, kv, tau);
                    }
                    let entry = shifted.eval(entry_t);
                    let arm = azimuth_arm(surface, prev.y);
                    let mut fits = true;
                    for margin in [(entry.x - prev.x) * arm, (entry.y - prev.y) * v_meter] {
                        match decide("pcurve_loop_continuity", margin, band) {
                            Ok(Sign::Zero) => {}
                            Ok(Sign::Positive | Sign::Negative) => fits = false,
                            Err(cause) => {
                                return Err(PcurveMintError::Escalated {
                                    half_edge: he,
                                    cause,
                                });
                            }
                        }
                    }
                    if fits {
                        chosen = Some(shifted);
                        break;
                    }
                    if ci + 1 == n {
                        return Err(PcurveMintError::LoopDiscontinuity { half_edge: he });
                    }
                }
                let Some(chosen) = chosen else {
                    return Err(PcurveMintError::LoopDiscontinuity { half_edge: he });
                };
                chosen
            }
        };
        let entry = pcurve.eval(entry_t);
        if first_entry.is_none() {
            first_entry = Some(entry);
        }
        prev_exit = Some(pcurve.eval(exit_t));
        out.push(Walked {
            half_edge: he,
            pcurve,
            t0,
            t1,
        });
    }
    // Closure: the walk returns to its start, either exactly (an
    // ordinary loop) or one full period around the chart (a loop that
    // wraps the periodic chart — the seam case, where the seam edge's
    // two half-edges take the two branches).
    if let (Some(start), Some(end)) = (first_entry, prev_exit)
        && !loop_closes(surface, start, end, band)
    {
        return Err(PcurveMintError::LoopNotClosed { face });
    }
    Ok(())
}

/// The at-rest pcurve pass the tier-3 validator runs (spec §5:
/// **certificate present + replay passes + trim containment**).
///
/// For every face that **carries at least one** stored cache (a body
/// that never ran the minting pass has none, and the pass says nothing
/// about it — absence is never a claim):
///
/// 1. the cache set of that face is COMPLETE — every half-edge of
///    every loop carries one (a half-minted face is a defect, not a
///    licence to derive the rest);
/// 2. the face's chart window is re-derived from the STORED caches and
///    each cache is **re-certified** against it — the stored
///    certificate is never consulted (re-certification re-derives, it
///    does not trust, exactly as [`geom_brep::EdgeCurve::recertify`]);
/// 3. the loop's one-branch continuity is re-checked on the stored
///    pcurves, so a body whose branches were tampered with fails here
///    even if each pcurve certifies in isolation.
///
/// Returns the findings in face-arena / loop / cycle order (D9),
/// empty when the body is clean. Bodies with no stored caches (every
/// all-planar body, and every body built before a curved face existed)
/// produce no findings — absence is not a defect.
pub fn validate_pcurves<T: PcurveFittedLane>(body: &Body<T>, band: Band) -> Vec<PcurveMintError> {
    let mut findings = Vec::new();
    for (face_key, face) in body.faces() {
        let Some(surface) = body.get_surface(face.surface) else {
            continue;
        };
        if !chart_mints(surface) {
            continue;
        }
        let surface = surface.clone();
        let loops: Vec<LoopKey> = core::iter::once(face.outer)
            .chain(face.rings.iter().copied())
            .collect();
        // Pass 0: does this face carry caches at all? A body that
        // never ran the minting pass (every sweep output, every
        // pre-M5 body) simply has none — absence is not a defect, and
        // the pass says nothing about it. Once ONE half-edge of a face
        // carries a cache, the set must be COMPLETE: a half-minted
        // face is the defect this checks for.
        let mut cycles: Vec<Vec<HalfEdgeKey>> = Vec::new();
        let mut any_cache = false;
        for lp in &loops {
            let Some(loop_data) = body.get_loop(*lp) else {
                continue;
            };
            let crate::entity::LoopBoundary::Cycle { first } = loop_data.boundary else {
                continue;
            };
            let Some(cycle) = body.loop_cycle(first) else {
                continue;
            };
            any_cache |= cycle.iter().any(|he| body.pcurve(*he).is_some());
        }
        if !any_cache {
            continue;
        }
        // Pass 1: presence + the face's window from the stored caches.
        let mut window: Option<ChartWindow<T>> = None;
        let mut complete = true;
        for lp in &loops {
            let Some(loop_data) = body.get_loop(*lp) else {
                findings.push(PcurveMintError::Corrupt);
                complete = false;
                continue;
            };
            let crate::entity::LoopBoundary::Cycle { first } = loop_data.boundary else {
                continue;
            };
            let Some(cycle) = body.loop_cycle(first) else {
                findings.push(PcurveMintError::Corrupt);
                complete = false;
                continue;
            };
            for &he in &cycle {
                match body.pcurve(he) {
                    None => {
                        findings.push(PcurveMintError::MissingCache { half_edge: he });
                        complete = false;
                    }
                    Some(cache) => {
                        let (t0, t1) = cache.params();
                        let b = cache.pcurve().chart_box(t0, t1);
                        window = Some(match window {
                            None => b,
                            Some(acc) => acc.hull(b),
                        });
                    }
                }
            }
            cycles.push(cycle);
        }
        let (Some(window), true) = (window, complete) else {
            continue;
        };
        // Pass 2: replay every stored certificate against that window.
        for cycle in &cycles {
            for &he in cycle {
                let Some(cache) = body.pcurve(he) else {
                    continue;
                };
                let carrier = match half_edge_carrier(body, he) {
                    Ok((c, _, _)) => c,
                    Err(e) => {
                        findings.push(e);
                        continue;
                    }
                };
                let mate = mate_surface(body, he);
                if let Err(error) = cache.recertify(&carrier, &surface, mate.as_ref(), window, band)
                {
                    findings.push(PcurveMintError::Certify {
                        half_edge: he,
                        error,
                    });
                }
            }
        }
        // Pass 3: the one-branch loop continuity of the STORED pcurves.
        let v_meter = polar_arm(&surface).unwrap_or_else(T::one);
        for cycle in &cycles {
            let mut prev_exit: Option<geom_core::Point2<T>> = None;
            let mut first_entry: Option<geom_core::Point2<T>> = None;
            for &he in cycle {
                let Some(cache) = body.pcurve(he) else {
                    continue;
                };
                let (t0, t1) = cache.params();
                let plus = match is_plus(body, he) {
                    Ok(v) => v,
                    Err(e) => {
                        findings.push(e);
                        continue;
                    }
                };
                let (entry_t, exit_t) = if plus { (t0, t1) } else { (t1, t0) };
                let entry = cache.pcurve().eval(entry_t);
                if let Some(prev) = prev_exit {
                    let arm = azimuth_arm(&surface, prev.y);
                    for margin in [(entry.x - prev.x) * arm, (entry.y - prev.y) * v_meter] {
                        match decide("pcurve_loop_continuity", margin, band) {
                            Ok(Sign::Zero) => {}
                            Ok(Sign::Positive | Sign::Negative) => {
                                findings.push(PcurveMintError::LoopDiscontinuity { half_edge: he });
                            }
                            Err(cause) => findings.push(PcurveMintError::Escalated {
                                half_edge: he,
                                cause,
                            }),
                        }
                    }
                }
                if first_entry.is_none() {
                    first_entry = Some(entry);
                }
                prev_exit = Some(cache.pcurve().eval(exit_t));
            }
            if let (Some(start), Some(end)) = (first_entry, prev_exit)
                && !loop_closes(&surface, start, end, band)
            {
                findings.push(PcurveMintError::LoopNotClosed { face: face_key });
            }
        }
    }
    findings
}
