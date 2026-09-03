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
//!   circles have a certified route that this pass cannot reach
//!   ([`geom_brep::PcurveCache::certify_fitted`], whose docs carry the
//!   frontier), so those faces stay uncached; the cone/torus
//!   oblique classes have no honest route (no ring-computable meters
//!   composite) and stay refused.
//! - **Described NURBS charts mint** their iso lane (M6-3,
//!   `nurbs_iso_derive`) — RATIONAL ones too since M8-3, whose ARC cap
//!   rims map through the chart's own rational-quadratic parameter
//!   (`Pcurve::IsoArc`). Only the mvfs placeholder mints nothing: it
//!   is not a described surface.
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
//! This is the M2 PR 5 meridian finding generalized:
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
//! Read this before storing a cache from a new call site. Three
//! postures exist, and they classify **the op a consumer calls**, not
//! every primitive that op uses internally: an op may drive `split_edge`
//! or a bare Euler run in its own interior and still MAINTAIN, so long
//! as it re-mints before it hands the body back.
//!
//! **Maintains the map** — runs [`mint_pcurves`] on the result, and
//! that pass CLEARS the map before re-minting. In this crate: the
//! splitting lane (on each side it produces), the boolean pipeline (on
//! the finished body), [`crate::Body::merge_coplanar_faces`] (on the
//! staged result before commit, and only when the input carried
//! caches), and [`crate::transform`] (which re-derives when the operand
//! carried caches). **Downstream crates hold the same posture and are
//! part of the list**: `sweep`'s loft, fillet build and fillet surgery,
//! and `step_import`'s assembly all re-mint on the body they return.
//!
//! **Transfers the map** — the graft (`boolean::combine`, and
//! [`crate::graft_disjoint`] through it) remaps each row onto the
//! transplanted half-edge's fresh key and DROPS any row whose key the
//! graft walk did not reach, which is exactly the staleness test.
//!
//! **Neither clears nor re-mints** — the Euler operators, the kill ops,
//! ring surgery, [`crate::Body::split_edge`]. These are primitives, and
//! they are what the stale-row consequence below is about.
//!
//! The consequence is bounded but real: a `SecondaryMap` row outlives
//! its key until the slot is reused, so surgery on a body that already
//! carries caches can leave a row attached to a half-edge that no
//! longer means what the cache says (or, once a slot is recycled, to a
//! different half-edge entirely). What makes that bounded rather than
//! dangerous is the backstop: the tier-3 pcurve pass catches a stale
//! row LOUD — it re-certifies against the current
//! carrier/surface/window and fails, or breaks its face loop's
//! continuity. So the posture is fail-loud, not silent-wrong — but an
//! op that mutates an already-minted body must either clear the map or
//! re-mint before returning, and must say which.
//!
//! **Where it says which, and what checks it.** For a `&mut Body` door
//! in this crate, in
//! `staleness_posture::every_mutation_door_declares_its_pcurve_posture`
//! — a walk of `topo/src` requiring every such door to either call
//! `mint_pcurves` in its own body or carry a declared posture. It goes
//! red the day a door is added and nobody says which bucket it is in,
//! and red the day a door whose entry says it does not re-mint starts
//! calling `mint_pcurves` directly.
//!
//! **What the guard does NOT establish**, so that nothing above reads
//! as more than it is:
//!
//! - **It checks that an entry exists, not that it is true.** A door
//!   declared `Maintains` is taken at its word: no walk can see that a
//!   delegate re-mints, and `mint_pcurves`'s own entry is a claim
//!   about the pass this module defines. Only the
//!   not-`Maintains`-but-minting direction is mechanical.
//! - **Delegation is invisible to it.** The one entry that rotted
//!   historically — an op that started re-minting through a helper
//!   while its bucket entry stayed put — is caught here only if the
//!   helper is itself a `&mut Body` door on this surface, which for
//!   [`crate::Body::merge_coplanar_faces`] it happens to be. A private
//!   delegate would be a green guard over the same rot.
//! - **It reads `topo/src` only.** The pipelines that take a body and
//!   return one rather than taking `&mut Body` (the splitting lane, the
//!   boolean pipeline, [`crate::transform`]) and every downstream crate
//!   are outside the walk; over that remainder the buckets above are a
//!   survey, checked by nothing.

use geom::Surface;
use geom_brep::{
    ChartWindow, Pcurve, PcurveCache, PcurveCertifyError, PcurveFittedLane, chart_pcurve,
};
use geom_core::Tol;
use geom_core::k_stats::decide;
use geom_core::predicate::{Band, BandError};
use geom_core::{Decide, Indeterminate, Margin, Real, Sign};

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
        // Described NURBS charts mint (M6-3; RATIONAL charts since
        // M8-3): every loft/sweep wall boundary is an iso-parameter
        // curve of the wall it bounds. Seams and LINE cap rims have
        // exact line images (`Pcurve::IsoLine`); an ARC cap rim on a
        // rational wall has an exact image too — the same boundary
        // line, on the chart's own rational-quadratic parameter
        // (`Pcurve::IsoArc`). The placeholder mints nothing: it is not
        // a described surface.
        Surface::Nurbs(payload) => !payload.is_placeholder(),
        // An approximating surface's chart is its fit's, and it is
        // described by construction — there is no placeholder state to
        // exclude, so it always mints.
        Surface::Approx(_) => true,
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
pub fn pcurve_of<T: PcurveFittedLane>(
    body: &Body<T>,
    half_edge: HalfEdgeKey,
    band: Band,
) -> Result<Pcurve<T>, PcurveMintError> {
    if let Some(cache) = body.pcurve(half_edge) {
        return Ok(cache.pcurve().clone());
    }
    let (carrier, _, _) = half_edge_carrier(body, half_edge)?;
    let surface = half_edge_surface(body, half_edge)?;
    // A SPLINE chart's images are description-driven (M6-3) — the iso
    // derivation, not the closed-form harmonic table. An approximating
    // surface's chart IS its fit's, so it takes the same route.
    if surface.spline_chart().is_some() {
        return nurbs_iso_derive(body, half_edge, &surface, band);
    }
    chart_pcurve(&carrier, &surface, band)
        .map_err(|error| PcurveMintError::Certify { half_edge, error })
}

/// The certified carrier and parameter interval of `half_edge`'s edge.
fn half_edge_carrier<T: Decide>(
    body: &Body<T>,
    half_edge: HalfEdgeKey,
) -> Result<(geom::Curve3<T>, T, T), PcurveMintError> {
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
/// locus belongs to (`EdgeDescription::Intersection { s1, s2 }` names them
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
    let geom_brep::EdgeDescription::Intersection { s1, s2, .. } = *curve.description() else {
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
) -> Result<geom_brep::EdgeDescription<T>, PcurveMintError> {
    let he = body
        .get_half_edge(half_edge)
        .ok_or(PcurveMintError::Corrupt)?;
    let edge = body.get_edge(he.edge).ok_or(PcurveMintError::Corrupt)?;
    let Some(CurveGeom::Certified(curve)) = body.get_curve_geom(edge.curve) else {
        return Err(PcurveMintError::Corrupt);
    };
    Ok(curve.description().clone())
}

/// The uniform clamped degree-1 knot vector on `[0, 1]` with `spans`
/// spans — an [`Pcurve::IsoArc`]'s sub-arc locator (pure `f64`
/// structure, which is what keeps the variant `T`-generic).
fn uniform_breaks(spans: usize) -> Option<geom_core::spline::KnotVector> {
    if spans == 0 {
        return None;
    }
    let mut knots = vec![0.0, 0.0];
    #[allow(clippy::cast_precision_loss)]
    for k in 1..spans {
        knots.push(k as f64 / spans as f64);
    }
    knots.extend([1.0, 1.0]);
    geom_core::spline::KnotVector::clamped(knots, 1).ok()
}

/// Derives the **exact iso chart image** of `half_edge` on a described
/// NURBS chart (M6-3; arc rims since M8-3) — the NURBS-chart
/// counterpart of `geom_brep::chart_pcurve`, driven by the edge's
/// INTENSIONAL description (D2: the description is what is
/// authoritative about which iso this locus is):
///
/// - A chart image naming THIS face's surface IS the answer: since
///   the conventional descriptions collapsed (U2) there is nothing
///   left to derive.
/// - An iso LINE image naming the OTHER wall maps as this chart's own
///   `u = u₀` or `u = u₁` boundary, the side selected by a definite
///   endpoint residual (`pcurve_iso_side`) and then CERTIFIED by the
///   full iso lane — a wrong pick fails loudly, never silently.
/// - A cap–wall rim over a LINE carrier maps as `(u(t), v)` with `u` affine
///   (`t0 ↦ u₀`, `t1 ↦ u₁` — the wall's u IS the segment parameter by
///   construction, up to the chart's own affine scale) and
///   `v ∈ {v₀, v₁}` by the same endpoint selection.
///
/// **The chart's own domain, not the unit square (#327).** Every
/// boundary above is the payload's KNOT domain end. A chart the kernel
/// BUILT is normalized to `[0, 1]²`, so this reads the same values it
/// always did; a chart the kernel IMPORTED carries the file's
/// parameterization — dm1's cylinder wall is `u ∈ [0, 3√3]` — where
/// `u = 1` is an interior column and every pick against it silently
/// answers about the wrong locus.
/// - A cap–wall rim over a CIRCLE carrier maps to the same boundary
///   line through the chart's own rational-quadratic parameter
///   ([`Pcurve::IsoArc`], M8-3). Both mapped description forms the
///   kernel mints for a circle reach it — see the arm's own note.
/// - An [`geom_brep::EdgeDescription::Intersection`] over a SPLINE carrier
///   that lies on a boundary column maps as that column: the same iso
///   line the `IsoCurve` arm mints, recovered from the carrier because
///   the intrinsic description names no chart coordinate. The residency
///   is a pick over the columns and the two `v` directions, definite or
///   escalated. A locus that is NOT a boundary column — an INTERIOR
///   column is the executed case (#498) — has no exact closed form and
///   takes U2's `General` curve-in-UV arm at the honest Fitted grade,
///   derived from the wall's own foot schedule.
/// - Everything else on a NURBS chart refuses typed with the class
///   named.
fn nurbs_iso_derive<T: PcurveFittedLane>(
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
    // The chart's OWN domain (doc above): `[0, 1]²` for a kernel-built
    // patch, the file's parameterization for an imported one.
    // The catch-all is SPLIT: an approximating surface's domain is its
    // FIT's knot domain, not the unit square — reading `[0,1]²` off a
    // chart parameterized otherwise would place every derived image on
    // the wrong rectangle.
    let (cu0, cu1, cv0, cv1) = match surface.spline_chart() {
        Some(payload) => {
            let (a, b) = payload.knots_u().domain();
            let (c, d) = payload.knots_v().domain();
            (
                T::from_f64(a),
                T::from_f64(b),
                T::from_f64(c),
                T::from_f64(d),
            )
        }
        None => (T::zero(), T::one(), T::zero(), T::one()),
    };
    // A definite endpoint-side selection: which of the two candidate
    // chart values places the carrier's START on the surface. The
    // selection is structure (a two-way pick), the CHECK is the full
    // iso-lane certification that follows every derivation.
    //
    // `None` is "no candidate is definitely it", NOT a refusal: a
    // caller may have a wider candidate to offer (the rim arms measure
    // one when the chart is wider than the face it trims). The refusal
    // text lives at the call sites that have nothing left to try, so
    // three arms can no longer share one message for three different
    // situations.
    let side_pick = |eval_at: &dyn Fn(T) -> geom_core::Point3<T>,
                     cands: &[T]|
     -> Result<Option<T>, PcurveMintError> {
        let start = carrier.eval(t0);
        for cand in cands {
            match decide(
                "pcurve_iso_side",
                Margin::of(start.distance(eval_at(*cand))),
                band,
            ) {
                Ok(Sign::Zero) => return Ok(Some(*cand)),
                Ok(Sign::Positive | Sign::Negative) => {}
                Err(cause) => {
                    return Err(PcurveMintError::Escalated { half_edge, cause });
                }
            }
        }
        Ok(None)
    };
    let no_boundary = || {
        refuse(
            "the carrier's start point lies on neither chart boundary — not a boundary \
             iso of this face's chart",
        )
    };
    let own = half_edge_surface_key(body, half_edge)?;
    match half_edge_description(body, half_edge)? {
        // **This face's OWN chart image is the answer.** Since the
        // conventional descriptions collapsed (U2), an edge described
        // as an image in THIS chart carries the image itself — there
        // is nothing left to derive, and re-deriving it would be a
        // second opinion about a locus the description already states.
        geom_brep::EdgeDescription::Chart(ref c) if c.surface == own => Ok(c.pcurve.clone()),
        // **The wall–wall seam stated on the OTHER wall.** An iso
        // LINE image (`u` fixed, `v` moving) on the neighbour's chart
        // maps as this chart's own `u = u₀`/`u = u₁` boundary, the
        // side selected by the endpoint. The moving channel is the
        // description's own, verbatim: the two walls share the seam's
        // parameterization, which is what makes them one seam.
        geom_brep::EdgeDescription::Chart(geom_brep::ChartCurve {
            pcurve: Pcurve::IsoLine { p0, pl },
            ..
        }) => {
            let v0 = p0.y + pl.y * t0;
            let column = |cand: T| surface.eval(cand, v0);
            // **No measured fall-back here, deliberately — and this is
            // the one place the rim arm's widening must NOT be copied.**
            // A wall-wall seam's image is a COLUMN: `u` is the FIXED
            // channel. The exact iso class certifies a fixed channel
            // only on a chart boundary, because a boundary row is a
            // control-net copy and an interior one is not (the hull
            // hypothesis the bound rests on) — `pcurve_cache`'s
            // `side_of` refuses an interior column by design, and
            // `geom-brep`'s `an_interior_column_still_refuses` pins
            // that. So a definite fall-through HERE is not a position
            // this arm is missing; it is a statement that the exact
            // class does not apply, and minting the measured column
            // anyway would hand the certifier exactly the image the
            // design requires it to refuse.
            //
            // The cap-rim arm below is the opposite case and that is
            // why it DOES measure: there `u` is the MOVING channel and
            // the fixed one is `v`, still on a boundary, so only the
            // map was wrong.
            //
            // Nor is `General` the answer for this locus: the fitted
            // grade certifies against an operand PAIR, and a `Chart`
            // description names ONE surface (`mate_surface` reads the
            // pair from an `Intersection` description), so there is no
            // tube to state. An interior column reached through a chart
            // description needs the de Boor collapse extractor named in
            // the refusal `side_of` raises; it is banked, not this
            // unit's.
            let x = side_pick(&column, &[cu0, cu1])?.ok_or_else(no_boundary)?;
            Ok(Pcurve::IsoLine {
                p0: Point2::new(x, p0.y),
                pl: Vec2::new(T::zero(), pl.y),
            })
        }
        // **The M8-3 ARC-RIM arm.** An ARC cap rim's chart image is
        // the same boundary line the LINE arm mints, but its moving
        // channel is the chart's own rational-quadratic parameter
        // rather than the arc angle — the `Pcurve::IsoArc` map.
        //
        // The arm is keyed on the CARRIER, not on the description
        // form, because the carrier is what the certification reads:
        // a circle rim is a circle rim however its own chart writes
        // it down. Keying on the form would have made the SAME
        // geometry mint natively and refuse on the round trip — a
        // description-form accident, not a fact about the rim.
        //
        // The sub-arc count is read off the chart's u structure (one
        // span per sub-arc, by the loft's construction); that read is
        // a SELECTION, and the CHECK is the full arc-rim certification
        // that follows, which compares the chart's boundary column
        // against the carrier circle's own rational-quadratic form and
        // refuses a chart that is not this construction.
        geom_brep::EdgeDescription::Chart(_) | geom_brep::EdgeDescription::Scaffold(_)
            if matches!(carrier, geom::Curve3::Circle { .. }) =>
        {
            let Some(payload) = surface.spline_chart() else {
                return Err(refuse("an arc cap rim on a non-spline chart"));
            };
            let ku = payload.knots_u();
            let (d0, d1) = ku.domain();
            let spans = ku.knots().iter().filter(|k| **k > d0 && **k < d1).count() / 2 + 1;
            let breaks = uniform_breaks(spans)
                .ok_or_else(|| refuse("an arc rim whose chart has no usable sub-arc structure"))?;
            let v =
                side_pick(&|cand| surface.eval(cu0, cand), &[cv0, cv1])?.ok_or_else(no_boundary)?;
            // **The u-DIRECTION pick (#327).** M8-3's arm assumed the
            // rim's increasing carrier parameter runs with the chart's
            // increasing `u` — true by construction for a wall the
            // kernel BUILT, and false in general for one it IMPORTED:
            // a promoted rim circle's `axis` is derived from the
            // file's own NURBS winding, and a cylinder wall's two rims
            // routinely wind oppositely in the file, so exactly one of
            // them runs against the chart. Assuming `+u` there does not
            // refuse — it mints a chart image that traverses the wall
            // BACKWARDS, and the loop walk reports it as a chart
            // discontinuity or a double-period closure, naming a
            // symptom two steps from the cause.
            //
            // The pick is the `side_pick` idiom on the other axis: the
            // two candidate images (`u: 0 → 1` and `u: 1 → 0`),
            // evaluated at a fixed interior probe of the arc and
            // metered against the carrier there. The start point
            // cannot decide it — on a FULL-PERIOD rim the chart's two
            // u-boundaries are the same 3-D point — so the probe sits
            // at a quarter of the arc, where the two candidates are a
            // half-period apart. A selection, checked: the full
            // arc-rim certification still follows.
            let probe_t = t0 + span * T::from_f64(0.25);
            let probe = carrier.eval(probe_t);
            let image = |p0x: T, sign: T| Pcurve::IsoArc {
                p0: Point2::new(p0x, v),
                pd: Vec2::new(sign, T::zero()),
                t0,
                angle: span,
                breaks: breaks.clone(),
            };
            // Escalations are DEFERRED per candidate (the loop walk's
            // posture): an indeterminate first candidate must not rob
            // the second of its turn.
            let mut deferred: Option<Indeterminate> = None;
            for (p0x, sign) in [(cu0, cu1 - cu0), (cu1, cu0 - cu1)] {
                let cand = image(p0x, sign);
                let uv = cand.eval(probe_t);
                let gap = probe.distance(surface.eval(uv.x, uv.y));
                match decide("pcurve_iso_arc_direction", Margin::of(gap), band) {
                    Ok(Sign::Zero) => return Ok(cand),
                    Ok(Sign::Positive | Sign::Negative) => {}
                    Err(cause) => {
                        if deferred.is_none() {
                            deferred = Some(cause);
                        }
                    }
                }
            }
            match deferred {
                Some(cause) => Err(PcurveMintError::Escalated { half_edge, cause }),
                None => Err(refuse(
                    "an arc rim whose chart image runs in neither u direction — the \
                     rim does not lie on this chart's boundary column",
                )),
            }
        }
        geom_brep::EdgeDescription::Chart(_) | geom_brep::EdgeDescription::Scaffold(_)
            if matches!(carrier, geom::Curve3::Line { .. }) =>
        {
            // The closed form: the rim spans the chart's whole `u`
            // domain, which is true for every wall the kernel BUILT.
            let plx = (cu1 - cu0) / span;
            let p0x = cu0 - (cu1 - cu0) * t0 / span;
            let row = |p0x: T, plx: T| move |cand: T| surface.eval(p0x + plx * t0, cand);
            match side_pick(&row(p0x, plx), &[cv0, cv1])? {
                Some(v) => Ok(Pcurve::IsoLine {
                    p0: Point2::new(p0x, v),
                    pl: Vec2::new(plx, T::zero()),
                }),
                // **The rim does not span this chart.** Same cause as
                // the wall-seam arm above: a chart wider than the face
                // it trims. The `u` MAP is what the closed form got
                // wrong (the `v` side is still a boundary), so measure
                // the rim's two endpoints and build the map from them.
                // Same certified foot producer, same metre-valued check
                // on the `v` side, and the full iso certification still
                // follows every derivation.
                None => {
                    let (f0, f1) = (
                        derive_chart_foot(carrier.eval(t0), surface, half_edge)?,
                        derive_chart_foot(carrier.eval(t1), surface, half_edge)?,
                    );
                    let (Some(f0), Some(f1)) = (f0, f1) else {
                        return Err(no_boundary());
                    };
                    let (u0, u1) = (T::from_f64(f0.x), T::from_f64(f1.x));
                    let plx = (u1 - u0) / span;
                    let p0x = u0 - plx * t0;
                    let v = side_pick(&row(p0x, plx), &[cv0, cv1])?.ok_or_else(no_boundary)?;
                    Ok(Pcurve::IsoLine {
                        p0: Point2::new(p0x, v),
                        pl: Vec2::new(plx, T::zero()),
                    })
                }
            }
        }
        // **The boundary-iso INTERSECTION arm.** The same wall–wall
        // seam the `IsoCurve` arm maps, stated INTRINSICALLY: the locus
        // is `S₁ ∩ S₂`, and where that locus IS this chart's own
        // `u = u₀`/`u = u₁` boundary column its chart image is the same
        // iso line. The description carries no chart coordinates at all
        // — it names two surfaces and a witness — so the image is
        // recovered from the CARRIER against the chart's own domain.
        //
        // Keyed on the carrier and on BOUNDARY RESIDENCY, never on the
        // description form or the operand order: which of `s1`/`s2` is
        // the plane is not a fact about the locus, and the same seam
        // stated natively or restated foreign must mint the same image.
        //
        // The pick is ONE fixed schedule (D9): the chart's two boundary
        // columns × the two directions the carrier can traverse the
        // chart's `v`, each candidate image evaluated at an interior
        // probe and metered against the carrier there in METRES. The
        // start point cannot decide it alone — it fixes a chart CORNER,
        // and a column and a direction both pass through one — so the
        // probe sits a quarter span in, where the candidates separate.
        // A SELECTION, checked: the full seam-class certification
        // follows every derivation.
        //
        // **What the schedule selects is wider than what the seam class
        // CERTIFIES, and deliberately so.** The certified inventory is
        // FORWARD only: the class's parameter-map slack is metered
        // against the identity map and its control hull against the
        // boundary row's own ordering, so a carrier that traverses the
        // chart's `v` backwards refuses at certification (the envelope)
        // rather than charting. The backward candidates stay in the
        // schedule because the honest answer for such a carrier is the
        // image it actually has, judged by the certification — not a
        // forward image that fits the class by construction and states
        // the wrong traversal.
        //
        // An `Intersection` whose carrier traverses NEITHER boundary
        // column under the chart's own parameterization refuses typed
        // and permanently (C5).
        geom_brep::EdgeDescription::Intersection { .. } => {
            if !matches!(carrier, geom::Curve3::Nurbs(_)) {
                return Err(refuse(
                    "an Intersection carrier that is not a spline — the certified \
                     boundary-column class compares the carrier against the chart's own \
                     boundary ROW, which is a spline",
                ));
            }
            let probe_t = t0 + span * T::from_f64(0.25);
            let probe = carrier.eval(probe_t);
            // Escalations are DEFERRED per candidate: an indeterminate
            // first candidate must not rob the rest of their turn.
            let mut deferred: Option<Indeterminate> = None;
            for x in [cu0, cu1] {
                for (v_at_t0, v_at_t1) in [(cv0, cv1), (cv1, cv0)] {
                    let slope = (v_at_t1 - v_at_t0) / span;
                    let cand = Pcurve::IsoLine {
                        p0: Point2::new(x, v_at_t0 - slope * t0),
                        pl: Vec2::new(T::zero(), slope),
                    };
                    let uv = cand.eval(probe_t);
                    let gap = probe.distance(surface.eval(uv.x, uv.y));
                    match decide("pcurve_iso_seam_column", Margin::of(gap), band) {
                        Ok(Sign::Zero) => return Ok(cand),
                        Ok(Sign::Positive | Sign::Negative) => {}
                        Err(cause) => {
                            if deferred.is_none() {
                                deferred = Some(cause);
                            }
                        }
                    }
                }
            }
            // ---- The fixed schedule found nothing. Derive the image. ----
            // The four candidates above assume the carrier traverses
            // the chart's WHOLE v domain, because that is what a
            // natively built wall's seam does. The foot schedule
            // measures the image instead of assuming it, and what it
            // measures decides the class (P-2):
            //
            // * offered back as the two boundary columns with the v map
            //   the image MEASURED, which is what a partial or affinely
            //   reparameterized restatement of a column looks like —
            //   still judged by the same metre-valued probe, so the
            //   exact class is preferred wherever it applies and no UV
            //   quantity reaches ε (D4 ¶1);
            // * otherwise stored as it is, a `General` curve in UV at
            //   the honest Fitted grade (U2), which is where an
            //   INTERIOR column lands: it has no boundary-row closed
            //   form and never will, and `General` is #498's home for
            //   it rather than a refusal.
            //
            // A DIAGONAL locus reaches here too and is not this unit's:
            // it refuses earlier, at edge certification, on
            // `PXN_IMAGE_DEGREE` (`geom-brep/src/edge_nurbs.rs`, banked
            // to #264), so no body carrying one reaches this pass.
            let image = match derive_general_image(&carrier, surface, half_edge) {
                Ok(image) => image,
                // An escalated candidate still outranks a derivation
                // refusal: a row that escalates today keeps escalating,
                // and the new path only ever speaks on a DEFINITE
                // fall-through.
                Err(e) => match deferred {
                    Some(cause) => return Err(PcurveMintError::Escalated { half_edge, cause }),
                    None => return Err(e),
                },
            };
            // **No re-offer of the exact class here, and the spec was
            // wrong to ask for one.** Item 7 read "a partial or
            // reparameterized restatement of a column" off the refusal
            // payload and inferred the exact class applies to it. It
            // does not: the seam class's hull limb compares the image
            // against the chart's own boundary ROW, and that comparison
            // needs ONE spline space — a partial column is not a
            // control-net copy of the boundary row and cannot be made
            // into one. Offering it here would hand the certifier an
            // image it must structurally refuse, and the mint would
            // then fail with text about a boundary row for a locus that
            // is not one. That is the same defect this arm's sibling
            // (the wall–wall seam arm) was reverted for.
            //
            // So the fixed schedule above IS the exact class's whole
            // reach — a boundary column traversed end to end, in either
            // direction — and everything it does not claim goes to
            // `General`, which certifies against the operand pair and
            // has no boundary-row hypothesis to violate. `General` is
            // not a downgrade for these loci; it is the only grade that
            // can state anything true about them.
            match deferred {
                Some(cause) => Err(PcurveMintError::Escalated { half_edge, cause }),
                None => Ok(Pcurve::General(std::sync::Arc::new(image))),
            }
        }
        _ => Err(refuse(
            "no iso derivation for this locus on a NURBS chart — only chart images of \
             this chart, iso-line images of the neighbouring wall, boundary-iso \
             Intersection seams, and LINE or CIRCLE cap rims have exact chart images \
             (the trimmed-NURBS pcurve lane is the cut-loft unit's)",
        )),
    }
}

/// The **general curve-in-UV image** of a spline carrier on this face's
/// own spline chart — U2's `General` arm, at the honest Fitted grade.
///
/// The producer is `geom_brep`'s one derivation of this object
/// ([`PcurveFittedLane::general_image`], whose body is the same
/// `edge_nurbs` foot schedule the plane × NURBS edge certificate uses
/// at adopt time). Nothing is certified here: this returns EVIDENCE,
/// and the mint's next move is `PcurveCache::certify_general`, which
/// bounds `sup_t |S(P(t)) − C(t)|` over the whole span against the
/// operand pair the edge's own description names.
///
/// # Errors
///
/// [`PcurveMintError::Certify`] carrying
/// [`PcurveCertifyError::FittedLaneUnsupported`] at a scalar with no
/// certified lane (a dual body may not certify — D1), or the
/// derivation's own typed refusal.
fn derive_general_image<T: PcurveFittedLane>(
    carrier: &geom::Curve3<T>,
    surface: &Surface<T>,
    half_edge: HalfEdgeKey,
) -> Result<geom::NurbsCurve2<T>, PcurveMintError> {
    let certify = |error| PcurveMintError::Certify { half_edge, error };
    let (geom::Curve3::Nurbs(spline), Some(wall)) = (carrier, surface.spline_chart()) else {
        // Unreachable from the one caller (which has already required
        // both), and stated as a refusal rather than a panic because
        // the pair is a precondition of the DERIVATION, not of the
        // body: a second caller must not be able to reach the producer
        // without it.
        return Err(certify(PcurveCertifyError::UnsupportedCarrier));
    };
    match T::general_image(spline, wall) {
        Ok(Some(image)) => Ok(image),
        Ok(None) => Err(certify(PcurveCertifyError::FittedLaneUnsupported {
            scalar: T::lane_name(),
        })),
        Err(error) => Err(certify(error)),
    }
}

/// The **certified chart foot** of one model-space point on this face's
/// own spline chart — [`derive_general_image`]'s single-sample sibling,
/// same producer (`geom_brep`'s `PcurveFittedLane::chart_foot`).
///
/// `None` at a scalar with no certified lane, which lets a caller keep
/// its previous typed refusal rather than inventing a foot: a dual body
/// answers exactly what it answered before this widening existed.
///
/// # Errors
///
/// [`PcurveMintError::Certify`] when the projection will not converge.
fn derive_chart_foot<T: PcurveFittedLane>(
    point: geom_core::Point3<T>,
    surface: &Surface<T>,
    half_edge: HalfEdgeKey,
) -> Result<Option<geom_core::Point2<f64>>, PcurveMintError> {
    let Some(wall) = surface.spline_chart() else {
        return Ok(None);
    };
    T::chart_foot(point, wall).map_err(|error| PcurveMintError::Certify { half_edge, error })
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

/// The FIRST-CHANNEL lever arm of a chart at second-parameter value
/// `v` — the metres a unit step of the chart's `u` moves the mapped
/// point (D4 ¶1: no UV-space tolerance ever reaches ε).
///
/// On the azimuth charts this is the LOCAL lever at that latitude:
/// cylinder `r`, sphere `|r·cos v|`, torus `|R + r·cos v|`, cone
/// `|v·sin α|`. Local is the honest metering for a joint gap — at a
/// sphere pole or a cone apex the lever is exactly zero, because the
/// chart azimuth genuinely does not move the point there, so a loop
/// meeting itself at a pole has no azimuth-continuity obligation and
/// a global sup arm would refuse every octant corner.
///
/// On the non-azimuth charts there is no latitude and the arm is a
/// chart constant: a plane's `u` IS metres, so its arm is exactly 1
/// by construction; a spline chart's `u` is the net's own parameter,
/// whose metre stretch is whatever the net says.
///
/// # Direction of error: why the spline arm is the SUP bound
///
/// Every caller of this function divides a chart-space discrepancy
/// against the linear band and asks whether the mapped points are
/// within ε — an ESCAPE claim in both of its two uses, and the sup
/// bound is the conservative side of each:
///
/// - `pcurve_loop_continuity` asks *"does this joint gap keep the
///   loop closed?"*. An OVER-stated arm over-states the metre gap,
///   which can only turn a `Zero` (closed) into a definite
///   discontinuity or an escalation. It refuses; it cannot certify a
///   loop closed across a gap the model can see. An UNDER-stated arm
///   does the reverse, and `1` on a chart whose stretch is 100 m per
///   chart unit under-states by exactly that factor.
/// - `pcurve_loop_pole_joint` asks *"is this lever zero, so that no
///   azimuth shift can select a branch?"*. Here too sup is the safe
///   side: `Zero` under the SUP bound means no `u` displacement
///   anywhere on the chart moves the point past the band, so skipping
///   the branch shift is honest. Under an inf reading the same
///   verdict would claim a collapsed lever on a chart that has one.
///
/// `geom_brep::chart_stretch_sup` is that bound and states the same
/// split at the export; it is emphatically not a lower bound, and
/// nothing here may be read as one.
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
        // The plane answers exactly 1 through this door (its chart
        // parameters ARE metres), and each spline kind answers its
        // net's own `sup |S_u|` — a placeholder payload, which has no
        // net to bound, answers 1 there too.
        Surface::Plane { .. } | Surface::Nurbs(_) | Surface::Approx(_) => {
            geom_brep::chart_stretch_sup(surface).0
        }
    }
}

/// The chart's **u period** — the whole-number shift the loop walk may
/// apply to land a half-edge's entry on its predecessor's exit
/// (`walk_loop`), and the wrap [`loop_closes`] allows.
///
/// **What each chart kind gets, exactly** (stated precisely because a
/// review found the earlier wording claiming more than the code does):
///
/// * every NON-NURBS chart answers `τ`, the hardcoded value this
///   function replaced — cylinder, cone, sphere, torus AND plane
///   behave bit-identically to before. A plane's `u` is not an azimuth
///   and no gap on one is ever a multiple of `τ`, so the branch is
///   unreachable there rather than suppressed; the honest word is
///   "unchanged", not "stricter";
/// * a NURBS chart CLOSED in `u` — first and last control columns the
///   same locus at the band — answers the payload's own u knot-domain
///   length;
/// * a NURBS chart NOT closed in `u` answers `None`: no shift offered,
///   no wrapped closure accepted. That is a tightening, and the only
///   one.
///
/// **The behaviour change on BUILT charts, called out.** A kernel-built
/// chart is normalized to `[0, 1]²`, so a closed-in-u one now answers
/// `Some(1.0)` where the old code used `τ`, and a gap that used to
/// floor to `ku = 0` against `τ` can now shift by a whole chart
/// period. It is reachable only where the old path could not close the
/// loop at all, and the joint's own `decide` still certifies the
/// shifted entry — but it is a change, not an identity.
///
/// The shift is offered only when the pole-joint gate reads the arm as
/// definitely nonzero, and on a spline chart that arm is the net's own
/// `sup |S_u|` ([`azimuth_arm`]), not a constant. So the gate is LIVE
/// on these charts in all three of its outcomes: a chart whose whole
/// `u` stretch sits under the band reads `Zero` and takes no shift at
/// all (which is honest — no `u` displacement on it moves a point
/// past ε), one whose stretch lands inside the band escalates, and
/// only a definitely-metric chart reaches the periodic rounding. The
/// paragraph above describes the last of the three.
///
/// **Why a NURBS chart needs this (#327).** A full-period cylinder
/// wall stated as ONE B-spline patch with a seam generator used twice
/// is the shape every translator writes and the kernel's own band
/// re-mint produces. Its loop walks the chart's whole u range and its
/// seam edge takes the two u-boundary branches — exactly the analytic
/// seam case, but at the chart's own period rather than `τ`. Without
/// this the walk reports the wrap as a discontinuity: a symptom, not
/// the cause.
fn chart_u_period<T: Decide>(surface: &Surface<T>, band: Band) -> Option<T> {
    // The `τ` default belongs to the AZIMUTH charts. A spline chart —
    // the payload's or an approximating surface's fit — has whatever
    // period its knot domain says, and only if the net actually closes;
    // handing it `τ` would let the loop walk wrap a chart that does not.
    let Some(payload) = surface.spline_chart() else {
        return Some(T::tau());
    };
    let (u0, u1) = payload.knots_u().domain();
    let (nu, nv) = payload.control_counts();
    if nu < 2 || nv == 0 {
        return None;
    }
    let control = payload.control();
    for iv in 0..nv {
        let a = control.get(iv)?;
        let b = control.get((nu - 1) * nv + iv)?;
        match decide("pcurve_chart_u_closed", Margin::of(a.distance(*b)), band) {
            Ok(Sign::Zero) => {}
            _ => return None,
        }
    }
    Some(T::from_f64(u1 - u0))
}

/// The meridional (second-channel) lever arm where that channel is
/// itself an angle — sphere `v` (arm `r`), torus `v` (arm `r_minor`).
/// `None` = the channel is NOT an angle, which is a different claim
/// from "already metres": see [`v_meter`], which is what a caller
/// wanting the channel's metre rate must ask.
fn polar_arm<T: Real>(surface: &Surface<T>) -> Option<T> {
    match *surface {
        Surface::Sphere { radius, .. } => Some(radius),
        Surface::Torus { minor_radius, .. } => Some(minor_radius),
        // The second channel is not an ANGLE on these charts, so no
        // polar radius levers it. That does not make it metres: on a
        // plane, cylinder or cone `v` IS a length, but a spline
        // chart's `v` is the net's own parameter, whose metre rate
        // [`v_meter`] reads off the chart. `None` here means "no
        // polar arm", and `v_meter` is the door that answers what the
        // rate actually is.
        Surface::Plane { .. }
        | Surface::Cylinder { .. }
        | Surface::Cone { .. }
        | Surface::Nurbs(_)
        | Surface::Approx(_) => None,
    }
}

/// The SECOND channel's metre rate for a loop-continuity gap — the
/// `v` companion of [`azimuth_arm`], and the same escape claim.
///
/// Where the channel is an angle ([`polar_arm`]: sphere, torus) the
/// arm is that exact radius. Where it is not, the channel's metre
/// rate is the chart's `sup |S_v|`: exactly 1 on a plane, cylinder or
/// cone, where `v` IS a length, and the net's own stretch on a spline
/// chart, where it is not. The direction argument is
/// [`azimuth_arm`]'s — an over-stated rate can only refuse a closure,
/// while `1` on a chart with a 100 m/unit stretch under-states the
/// metre gap by that factor and certifies a loop closed across it.
fn v_meter<T: Real>(surface: &Surface<T>) -> T {
    polar_arm(surface).unwrap_or_else(|| geom_brep::chart_stretch_sup(surface).1)
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
    u_period: Option<T>,
    band: Band,
) -> bool {
    let arm = azimuth_arm(surface, start.y);
    // A chart that does not wrap offers no period, and `wraps` below
    // degenerates to the exact-closure test (`m ± 0`).
    let tau = u_period.unwrap_or_else(T::zero);
    let zero = |name: &'static str, m: Margin<T>| matches!(decide(name, m, band), Ok(Sign::Zero));
    let wraps = |m: T, a: T, name: &'static str| {
        [m, m - tau, m + tau]
            .into_iter()
            .any(|c| zero(name, Margin::levered(c, a)))
    };
    let du = end.x - start.x;
    let dv = end.y - start.y;
    let direct = match polar_arm(surface) {
        None => {
            wraps(du, arm, "pcurve_loop_closure")
                && zero("pcurve_loop_closure_height", Margin::of(dv))
        }
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
pub fn mint_pcurves<T: PcurveFittedLane>(
    body: &mut Body<T>,
    tol: Tol,
) -> Result<(), PcurveMintError> {
    let band = Band::linear(tol).map_err(PcurveMintError::Band)?;
    // Start from empty. A body reaching this pass may have been carved
    // from a scratch clone that inherited rows for half-edges the
    // surgery killed (a `SecondaryMap` row outlives its key until the
    // slot is reused), and a stale cache is worse than no cache. What
    // this pass leaves behind is exactly what it minted and certified.
    body.pcurves.clear();
    let faces: Vec<FaceKey> = body.faces().map(|(k, _)| k).collect();
    for face in faces {
        match mint_face(body, face, band) {
            Ok(()) => {}
            // A carrier CLASS outside every derivation route (the
            // executed case: an oblique fillet trihedron's corner
            // octant, whose boundary circles are GENERAL sphere
            // circles — neither polar nor meridian relative to the
            // stored chart axis). The face is honestly NOT COVERED by
            // the closed-form lane, and an uncached face is a legal
            // at-rest state ("absence is never a claim" —
            // `validate_pcurves`); refusing the whole construction
            // would claim a coverage the lane does not have. The
            // class's certified route EXISTS (`certify_fitted`'s
            // Circle-carrier arm, mate from the edge's description);
            // wiring it into this pass needs the `PcurveFittedLane`
            // bound on every constructor and is banked with that
            // ripple — banked in no milestone plan and in no
            // carried-items register. Every OTHER failure — a covered
            // class whose residuals, envelope, continuity or closure
            // refuse — is a genuine defect and propagates.
            Err(PcurveMintError::Certify {
                error: PcurveCertifyError::UnsupportedCarrier,
                ..
            }) => {
                clear_face_caches(body, face);
            }
            Err(e) => return Err(e),
        }
    }
    Ok(())
}

/// Drops any caches minted for `face` before its walk refused — a
/// face outside the lane's coverage stores NOTHING (a half-minted
/// face is the defect `validate_pcurves` hunts).
fn clear_face_caches<T: PcurveFittedLane>(body: &mut Body<T>, face: FaceKey) {
    let Some(face_data) = body.get_face(face) else {
        return;
    };
    let loops: Vec<LoopKey> = core::iter::once(face_data.outer)
        .chain(face_data.rings.iter().copied())
        .collect();
    let mut hes: Vec<HalfEdgeKey> = Vec::new();
    for lk in loops {
        let Some(lp) = body.get_loop(lk) else {
            continue;
        };
        let crate::entity::LoopBoundary::Cycle { first } = lp.boundary else {
            continue;
        };
        let Some(cycle) = body.loop_cycle(first) else {
            continue;
        };
        hes.extend(cycle);
    }
    for he in hes {
        body.pcurves.remove(he);
    }
}

/// Mints the caches of one face (module docs: the two-pass shape — walk
/// the loops to pin branches and build the chart window, then certify
/// every pcurve against that window).
fn mint_face<T: PcurveFittedLane>(
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
    // **Check 5 is vacuous on both of this crate's callers.** The
    // window IS the hull of exactly the boxes checked against it, so
    // no minted pcurve can escape it, and `validate_pcurves` re-derives
    // its window the same self-referential way. Check 5 is a
    // precondition the caller supplies, not a check that fires on any
    // path this crate walks; what the precondition buys is stated once,
    // at `geom_brep::PcurveCache`'s module docs.
    //
    // The vacuity at mint is deliberate: a freshly derived face has no
    // independent prior notion of its own trim region, and inventing
    // one (say, the loop's vertex box) would refuse legitimate faces
    // whose boundary arcs bulge past their endpoints.
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
        let cache = match &w.pcurve {
            // U2's `General` arm certifies at the FITTED grade: the
            // same five checks in the same order, but check 4 is the
            // full C2 certificate against the operand PAIR, so it needs
            // the mate the edge's own description names (D2 — read from
            // the description, never from the topology, and re-read
            // from the body rather than stored). The closed-form door
            // cannot state that certificate and refuses this variant.
            Pcurve::General(image) => PcurveCache::certify_general(
                std::sync::Arc::clone(image),
                w.t0,
                w.t1,
                &carrier,
                &surface,
                mate_surface(body, w.half_edge).as_ref(),
                window,
                band,
            ),
            _ => PcurveCache::certify(
                w.pcurve.clone(),
                w.t0,
                w.t1,
                &carrier,
                &surface,
                window,
                band,
            ),
        }
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
fn walk_loop<T: PcurveFittedLane>(
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
    // The chart's own u period (`chart_u_period`): `τ` on an analytic
    // azimuth chart, the knot-domain length on a NURBS chart closed in
    // u, and NO shift at all on a chart that does not wrap.
    let u_period = chart_u_period(surface, band);
    let tau = T::tau();
    // The walk's running exit point, in chart coordinates.
    let mut prev_exit: Option<geom_core::Point2<T>> = None;
    let mut first_entry: Option<geom_core::Point2<T>> = None;
    for he in cycle {
        let (carrier, t0, t1) = half_edge_carrier(body, he)?;
        let base = if surface.spline_chart().is_some() {
            // Spline charts derive from the edge's intensional
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
        let v_meter = v_meter(surface);
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
                let twin = sphere_twin(surface, &base);
                let mut chosen: Option<Pcurve<T>> = None;
                // A WRONG candidate's escalation is not the loop's
                // verdict: the base representation of a pole-crossing
                // sphere pcurve sits π off, which lands its
                // whole-period rounding EXACTLY on an integer
                // boundary — at the interval scalar that floor spans
                // two integers and the continuity margin becomes a
                // full-period enclosure. The twin still fits exactly.
                // So escalations are DEFERRED per candidate and
                // surfaced (first one, deterministically) only when
                // no candidate fits — single-candidate charts keep
                // their old escalate-immediately behavior through
                // exactly that arm.
                let mut deferred: Option<Indeterminate> = None;
                let candidates: Vec<Pcurve<T>> = core::iter::once(base).chain(twin).collect();
                for cand in candidates {
                    let raw = cand.eval(entry_t);
                    // An AZIMUTH-FREE joint (a pole / the apex / a
                    // spline chart whose whole u stretch sits under
                    // the band: the lever is zero in metres) has no
                    // branch to pick — every azimuth agrees there, and
                    // the whole-period rounding below would land on an
                    // integer boundary (the gap need not be a period at
                    // all), which the interval scalar honestly reports
                    // as a two-integer floor. Skip the shift; downstream
                    // joints anchor their own branches.
                    //
                    // **The in-band arm takes the same skip, and this
                    // is the argument for it — which is NOT that a
                    // sub-tolerance lever is harmless.** An escalated
                    // arm means the lever's own size is undecided, so
                    // whether a period shift is even meaningful is
                    // undecided with it, and rounding on an undecided
                    // lever would MANUFACTURE a branch choice from a
                    // measurement that refused. Skipping keeps the
                    // decision unmade. What makes that safe is not the
                    // skip: it is that the continuity margins below run
                    // unconditionally on the unshifted candidate and are
                    // metred by the same arm, so a joint that genuinely
                    // needed the shift fails them and the loop refuses
                    // or escalates rather than certifying. The skip
                    // defers; the margins decide.
                    let joint_arm = azimuth_arm(surface, prev.y);
                    let ku = match decide("pcurve_loop_pole_joint", Margin::of(joint_arm), band) {
                        Ok(Sign::Zero) | Err(_) => T::zero(),
                        Ok(Sign::Positive | Sign::Negative) => match u_period {
                            Some(p) => (prev.x - raw.x).periodic_branch(p),
                            None => T::zero(),
                        },
                    };
                    // The impossible-rebuild arm (see
                    // `Pcurve::shift_branch`) surfaces as a
                    // corrupt-body finding rather than being swallowed
                    // into an unshifted branch.
                    let Some(mut shifted) = cand.shift_branch(ku, u_period.unwrap_or_else(T::zero))
                    else {
                        return Err(PcurveMintError::Corrupt);
                    };
                    if v_arm.is_some() {
                        let ry = shifted.eval(entry_t).y;
                        let kv = (prev.y - ry).periodic_branch(tau);
                        shifted = shift_polar_branch(&shifted, kv, tau);
                    }
                    let entry = shifted.eval(entry_t);
                    let arm = azimuth_arm(surface, prev.y);
                    let mut fits = true;
                    for margin in [
                        Margin::levered(entry.x - prev.x, arm),
                        Margin::metered(entry.y - prev.y, v_meter),
                    ] {
                        match decide("pcurve_loop_continuity", margin, band) {
                            Ok(Sign::Zero) => {}
                            Ok(Sign::Positive | Sign::Negative) => fits = false,
                            Err(cause) => {
                                fits = false;
                                if deferred.is_none() {
                                    deferred = Some(cause);
                                }
                            }
                        }
                    }
                    if fits {
                        chosen = Some(shifted);
                        break;
                    }
                }
                match (chosen, deferred) {
                    (Some(p), _) => p,
                    (None, Some(cause)) => {
                        return Err(PcurveMintError::Escalated {
                            half_edge: he,
                            cause,
                        });
                    }
                    (None, None) => {
                        return Err(PcurveMintError::LoopDiscontinuity { half_edge: he });
                    }
                }
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
        && !loop_closes(surface, start, end, u_period, band)
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
        let v_meter = v_meter(&surface);
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
                    for margin in [
                        Margin::levered(entry.x - prev.x, arm),
                        Margin::metered(entry.y - prev.y, v_meter),
                    ] {
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
                && !loop_closes(&surface, start, end, chart_u_period(&surface, band), band)
            {
                findings.push(PcurveMintError::LoopNotClosed { face: face_key });
            }
        }
    }
    findings
}

#[cfg(test)]
pub(crate) mod staleness_posture {
    #![allow(clippy::expect_used)]

    /// Which of this module's three postures a mutation door holds.
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub(crate) enum Posture {
        /// Clears and re-mints before returning. Read out of the
        /// source (a `mint_pcurves` call in the door's own body); an
        /// entry declares it only when the re-mint is one delegation
        /// away, which a source read cannot see.
        Maintains,
        /// Remaps each row onto the surviving key and drops the rest.
        Transfers,
        /// Leaves the map exactly as it found it — a primitive, or a
        /// write the map is not keyed on. Safe because the tier-3
        /// pcurve pass catches the consequence loud.
        Neither,
    }

    /// `(door, posture, note)` — the doors that do NOT re-mint the
    /// map in their own body. The reason a posture is SAFE lives once,
    /// on the [`Posture`] variant; a note here says only what is
    /// particular to this door.
    ///
    /// Module-scoped rather than local to the guard so that
    /// [`crate::review_m1_pr5_internal::the_two_door_tables_cover_the_same_surface`]
    /// can read it. That row is the only other reader; this table
    /// stays this guard's.
    pub(crate) const DECLARED: &[(&str, Posture, &str)] = {
        use Posture::{Maintains, Neither, Transfers};
        &[
            // ---- Maintains, one delegation away from the re-mint. ----
            (
                "merge_coplanar_faces",
                Maintains,
                "calls `merge_coplanar_faces_declared`, which re-mints the staged result",
            ),
            (
                "replace_face_offset",
                Maintains,
                "the one-face spelling of `replace_faces_offset`, which re-mints the clone \
                 before adopting it",
            ),
            // ---- The pass itself. ----
            (
                "mint_pcurves",
                Maintains,
                "IS the pass: clears the map, then re-mints every row of the body it is given",
            ),
            // ---- Transfers: the graft's remap-and-drop. ----
            (
                "graft_disjoint",
                Transfers,
                "the graft, through `boolean::combine` — see `graft_disjoint_all_keyed`",
            ),
            (
                "graft_disjoint_all",
                Transfers,
                "the graft — see `graft_disjoint_all_keyed`",
            ),
            (
                "graft_disjoint_all_keyed",
                Transfers,
                "remaps each row onto the transplanted half-edge's fresh key and DROPS any \
             row the graft walk did not reach, which is the staleness test itself",
            ),
            (
                "graft_disjoint_all_onto_keyed",
                Transfers,
                "the graft — see `graft_disjoint_all_keyed`",
            ),
            (
                "insert_void",
                Transfers,
                "the void-insertion door: reverts the cavity (rows keep their keys, going \
             stale in CONTENT like any surgery) and grafts through `boolean::combine`, \
             which remaps the transplanted rows onto fresh keys; both producers' final \
             mint passes re-derive every row of the merged body",
            ),
            // ---- Neither: the primitives. Their stale rows are what
            // the tier-3 pcurve pass exists to catch. ----
            ("mvfs", Neither, "Euler operator"),
            ("mev", Neither, "Euler operator"),
            ("mev_line", Neither, "Euler operator (sugar over `mev`)"),
            ("mev_null", Neither, "Euler operator"),
            ("mef", Neither, "Euler operator"),
            ("mef_chord", Neither, "Euler operator (sugar over `mef`)"),
            ("mekr", Neither, "Euler operator"),
            ("mekr_chord", Neither, "Euler operator (sugar over `mekr`)"),
            ("mfkrh", Neither, "Euler operator"),
            ("mfkrh_plug", Neither, "Euler operator (sugar over `mfkrh`)"),
            ("kemr", Neither, "Euler operator"),
            ("kfmrh", Neither, "Euler operator"),
            ("kev", Neither, "kill op"),
            ("kef", Neither, "kill op"),
            ("kvfs", Neither, "kill op"),
            (
                "ring_move",
                Neither,
                "ring surgery: re-parents a ring, mints no half-edge",
            ),
            (
                "movefac",
                Neither,
                "re-parents faces between shells; no half-edge key changes meaning",
            ),
            (
                "split_edge",
                Neither,
                "replaces one edge's geometry with two children — the one primitive that \
             makes a row stale in CONTENT rather than by key",
            ),
            // ---- Neither: the caller's own row-level control of the
            // map, and writes the map is not keyed on. ----
            (
                "attach_pcurve",
                Neither,
                "writes ONE row the caller chose; every other row is untouched, and \
             certifying this one is the caller's",
            ),
            ("detach_pcurve", Neither, "drops ONE row the caller chose"),
            (
                "set_face_surface",
                Neither,
                "a surface swap is content staleness the tier-3 pass re-certifies against, \
             not a key the map can lose",
            ),
            (
                "set_edge_curve",
                Neither,
                "a carrier swap is content staleness the tier-3 pass re-certifies against",
            ),
            (
                "set_edge_curve_nurbs_lane",
                Neither,
                "`set_edge_curve` with the NURBS certifier injected",
            ),
            ("set_face_sense", Neither, "writes one `bool`"),
            ("set_surface_source", Neither, "GeomSource metadata"),
            ("set_curve_source", Neither, "GeomSource metadata"),
            ("set_point_source", Neither, "GeomSource metadata"),
            ("clear_geom_sources", Neither, "GeomSource metadata"),
            ("set_null_face_pair", Neither, "null-face annotation"),
            ("clear_null_face_pair", Neither, "removes that annotation"),
        ]
    };

    /// **The convention at the top of this module, checked rather than
    /// surveyed.** Every public mutation path into a [`crate::Body`] —
    /// `pub fn` taking `&mut self`, plus the free functions taking
    /// `&mut Body<T>` — either re-mints the map in its own body, which
    /// this walk reads directly, or is declared below with its posture
    /// and a note on that door.
    ///
    /// **Why a test and not a list in prose.** A prose index has no way
    /// to notice a door being added, and the previous one did not. This
    /// goes red the day one lands unsorted, which is the rot the prose
    /// could only describe.
    ///
    /// **What it checks, exactly.** Three failures, all mechanical: a
    /// door that neither calls `mint_pcurves` nor appears below; a door
    /// whose entry says anything but `Maintains` while its body calls
    /// `mint_pcurves`; and an entry naming a door that no longer
    /// exists.
    ///
    /// **Where the door set comes from, and what it cannot see:**
    /// [`crate::source_walk::mutation_doors`], shared with the tier-1
    /// postcondition guard in [`crate::review_m1_pr5_internal`], which
    /// walks the same population to ask a different question. That
    /// function's docs carry the reason the two tables do not merge
    /// and the whole inherited blind-spot list; this guard does not
    /// restate either.
    ///
    /// **"Calls `mint_pcurves`" is a read of code, not of prose.** The
    /// body arrives with comments and literals blanked. This guard
    /// used a raw `body.contains`, and a planted door whose body only
    /// *mentioned* `mint_pcurves(` in a comment was counted as
    /// re-minting, in both this guard and the tier-1 one, both green.
    ///
    /// **What it does not check.** That a `Maintains` entry is TRUE. A
    /// re-mint reached through a delegate is invisible to a source
    /// read, so those two entries are taken at their word — the guard
    /// establishes that every door is sorted and that no door has
    /// silently started minting, not that each sort is correct. The
    /// module docs' *"what the guard does NOT establish"* list carries
    /// this and the rest of the blind spot: delegation, and everything
    /// outside `topo/src`'s `&mut Body` surface. The full inherited
    /// list is on [`crate::source_walk::mutation_doors`].
    #[test]
    fn every_mutation_door_declares_its_pcurve_posture() {
        use Posture::Maintains;
        let mut minting: Vec<String> = Vec::new();
        let mut declared: Vec<&str> = Vec::new();
        let mut undeclared: Vec<String> = Vec::new();
        let mut mislabelled: Vec<String> = Vec::new();

        for door in crate::source_walk::mutation_doors() {
            let entry = DECLARED.iter().find(|(n, _, _)| *n == door.name);
            if door.code_contains("mint_pcurves(") {
                if let Some((_, posture, _)) = entry.filter(|(_, p, _)| *p != Maintains) {
                    mislabelled.push(format!("{} declared {posture:?}", door.name));
                }
                minting.push(door.name);
            } else if let Some((n, _, _)) = entry {
                declared.push(n);
            } else {
                undeclared.push(door.site());
            }
        }

        assert!(
            undeclared.is_empty(),
            "public mutation path(s) that neither re-mint the pcurve map nor declare a \
             posture in this test: {undeclared:?}. Either re-mint before returning, or add \
             the door above with the posture it holds.",
        );
        assert!(
            mislabelled.is_empty(),
            "door(s) whose body now calls `mint_pcurves` but whose entry says otherwise: \
             {mislabelled:?}. This is the rot the prose index suffered — move the entry to \
             `Maintains`, or drop it and let the walk classify the door.",
        );
        // The entries rot in the other direction too.
        for (name, _, _) in DECLARED {
            assert!(
                declared.contains(name) || minting.iter().any(|n| n == name),
                "this test declares a posture for `{name}`, which is no longer a public \
                 mutation path — it was renamed or deleted. Drop the entry.",
            );
        }
        // **Over-stripping is SILENT here**, which is why this pin is
        // by name rather than a count. A door that stops reading as
        // calling `mint_pcurves` does not red — it falls into the
        // `else if let Some(entry)` arm and is accepted as declared.
        // Only a door with no entry at all reds, and today exactly one
        // door is classified by its body, so a lexing gap that erased
        // the needle everywhere would leave this guard green over a
        // surface it had stopped reading. The walk's own floor is
        // upstream on `mutation_doors`; this is the needle's.
        //
        // **It is exactly one door wide, and that is the whole of it.**
        // It does not cover the 36 `DECLARED` doors: those land in the
        // same `else if` arm whether or not their needle survives, so a
        // declared non-`Maintains` door that STARTS minting while its
        // call is over-stripped would not reach `mislabelled`. Closing
        // that needs a second oracle for "does this body call it",
        // which a source read does not have.
        assert!(
            minting.iter().any(|n| n == "merge_coplanar_faces_declared"),
            "`merge_coplanar_faces_declared` no longer reads as calling `mint_pcurves`. \
             Either the door stopped re-minting — a finding, and its entry belongs below \
             — or the source read lost the call.",
        );
        println!(
            "[pcurve posture] {} door(s): {} re-mint, {} declared",
            declared.len() + minting.len(),
            minting.len(),
            declared.len(),
        );
    }
}

/// The chart-stretch meter rows: the arms these loop-continuity
/// margins are metred by, and the direction each claim needs.
#[cfg(test)]
mod stretch_meter {
    #![allow(clippy::unwrap_used, clippy::float_cmp)]

    use super::{azimuth_arm, v_meter};
    use geom::{NurbsSurface, Surface};
    use geom_core::k_stats::decide;
    use geom_core::spline::KnotVector;
    use geom_core::{Band, Margin, Point3, Sign, Vec3};
    use std::sync::Arc;

    /// The band every row here pins explicitly: `zero = 1e-9`,
    /// `escalate = 1e-8`. Pinned rather than read from the run's
    /// tolerance so the digits below mean the same thing at every ε
    /// the sweep runs.
    fn band() -> Band {
        Band::new(1e-9, 1e-8).unwrap()
    }

    /// A bilinear NURBS chart on `[0, 1]²` whose image is a flat
    /// `span × span` metre square: `S(u, v) = (span·u, span·v, 0)`,
    /// so `|S_u| = |S_v| = span` EVERYWHERE — the chart's metre
    /// stretch is exactly `span`, not 1.
    fn flat_chart(span: f64) -> Surface<f64> {
        let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let control = vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.0, span, 0.0),
            Point3::new(span, 0.0, 0.0),
            Point3::new(span, span, 0.0),
        ];
        Surface::Nurbs(Arc::new(
            NurbsSurface::new(kv.clone(), kv, control, vec![1.0; 4]).unwrap(),
        ))
    }

    fn plane() -> Surface<f64> {
        Surface::Plane {
            origin: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        }
    }

    /// **The red-first row, azimuth channel.** A 100× chart maps a
    /// `1e-10` chart-unit u gap to `1e-8` metres — a DEFINITE
    /// discontinuity at the pinned band. Metred by 1 it reads `1e-10`
    /// and certifies `Zero`: the loop closes on a gap the kernel can
    /// see. The arm is the whole of the defect.
    #[test]
    fn a_stretched_nurbs_chart_meters_its_azimuth_gap_in_metres() {
        let s = flat_chart(100.0);
        let arm = azimuth_arm(&s, 0.0);
        assert_eq!(arm, 100.0, "the chart's own metre stretch, not 1");
        let gap = 1e-10;
        assert_eq!(
            decide("pcurve_loop_continuity", Margin::levered(gap, arm), band()),
            Ok(Sign::Positive),
            "1e-10 chart units × 100 m/unit = 1e-8 m, at the escalate edge"
        );
        assert_eq!(
            decide("pcurve_loop_continuity", Margin::levered(gap, 1.0), band()),
            Ok(Sign::Zero),
            "the under-stated arm certifies the same loop closed"
        );
    }

    /// **The red-first row, second channel.** Same digits, `v_meter`.
    #[test]
    fn a_stretched_nurbs_chart_meters_its_second_channel_in_metres() {
        let s = flat_chart(100.0);
        let meter = v_meter(&s);
        assert_eq!(meter, 100.0);
        let gap = 1e-10;
        assert_eq!(
            decide(
                "pcurve_loop_continuity",
                Margin::metered(gap, meter),
                band()
            ),
            Ok(Sign::Positive)
        );
        assert_eq!(
            decide("pcurve_loop_continuity", Margin::metered(gap, 1.0), band()),
            Ok(Sign::Zero)
        );
    }

    /// **The scale twin.** The same loop at a uniform 1e3 scale: the
    /// arm scales exactly with the model, so a chart gap that was
    /// `1e-8` m is `1e-5` m — the metering carries the scale rather
    /// than fixing a metre size into the chart.
    #[test]
    fn the_stretch_arm_carries_a_uniform_scale() {
        let small = flat_chart(100.0);
        let large = flat_chart(100.0e3);
        assert_eq!(azimuth_arm(&large, 0.0), azimuth_arm(&small, 0.0) * 1e3);
        assert_eq!(v_meter(&large), v_meter(&small) * 1e3);
    }

    /// **The plane arm is 1 by construction, not by default**, and
    /// stays bit-identical: a plane chart's u and v ARE metres.
    #[test]
    fn a_plane_chart_keeps_its_exact_unit_arms() {
        let p = plane();
        assert_eq!(azimuth_arm(&p, 0.0), 1.0);
        assert_eq!(azimuth_arm(&p, 0.7), 1.0);
        assert_eq!(v_meter(&p), 1.0);
    }

    /// **Three-outcome posture on the newly-honest arm.** A chart gap
    /// whose metred size lands INSIDE the band escalates typed rather
    /// than picking a side: `5e-11 × 100 = 5e-9 ∈ (1e-9, 1e-8)`.
    #[test]
    fn an_in_band_metred_gap_escalates_rather_than_deciding() {
        let s = flat_chart(100.0);
        let arm = azimuth_arm(&s, 0.0);
        assert!(
            decide(
                "pcurve_loop_continuity",
                Margin::levered(5e-11, arm),
                band()
            )
            .is_err(),
            "in-band residue is the third outcome, not a verdict"
        );
        assert_eq!(
            decide(
                "pcurve_loop_continuity",
                Margin::levered(5e-12, arm),
                band()
            ),
            Ok(Sign::Zero),
            "5e-10 m is honestly closed"
        );
    }

    /// **The pole-joint gate is LIVE on spline charts, in all three
    /// outcomes** (review item 5: before this unit the arm was the
    /// constant `1` there, so the gate could only ever answer
    /// `Positive` and no row exercised it at all).
    ///
    /// The gate reads `Margin::of(azimuth_arm(..))` — the arm's own
    /// size, gated as a length (the collapsed-arm idiom) — and the
    /// walk converts `Zero` and an escalation alike to "take no
    /// branch shift".
    #[test]
    fn the_pole_joint_gate_answers_all_three_ways_on_spline_charts() {
        // A chart whose whole u stretch is 1e-12 m per chart unit:
        // no u displacement on it moves a point past the band, so
        // the lever is honestly collapsed and no branch is selectable.
        let collapsed = flat_chart(1e-12);
        assert_eq!(
            decide(
                "pcurve_loop_pole_joint",
                Margin::of(azimuth_arm(&collapsed, 0.0)),
                band()
            ),
            Ok(Sign::Zero),
            "a sub-band chart stretch is a collapsed lever"
        );
        // In-band: the lever's own size is undecided, so the walk
        // defers the shift rather than manufacturing one, and the
        // continuity margins below decide instead.
        for span in [5e-9_f64, 2e-9] {
            let s = flat_chart(span);
            assert!(
                decide(
                    "pcurve_loop_pole_joint",
                    Margin::of(azimuth_arm(&s, 0.0)),
                    band()
                )
                .is_err(),
                "an in-band lever ({span:e}) escalates rather than deciding"
            );
        }
        // And a real chart reaches the periodic rounding.
        assert_eq!(
            decide(
                "pcurve_loop_pole_joint",
                Margin::of(azimuth_arm(&flat_chart(100.0), 0.0)),
                band()
            ),
            Ok(Sign::Positive)
        );
        // The analytic poles are unmoved: a sphere pole still reads
        // an exactly-zero lever through the same door.
        let sphere: Surface<f64> = Surface::Sphere {
            center: Point3::new(0.0, 0.0, 0.0),
            radius: 2.0,
            axis: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        assert_eq!(
            azimuth_arm(&sphere, core::f64::consts::FRAC_PI_2),
            2.0 * core::f64::consts::FRAC_PI_2.cos()
        );
        assert_eq!(
            decide(
                "pcurve_loop_pole_joint",
                Margin::of(azimuth_arm(&sphere, core::f64::consts::FRAC_PI_2)),
                band()
            ),
            Ok(Sign::Zero)
        );
    }

    /// A placeholder payload has no net to bound, so it keeps the
    /// unit arms — the one NURBS chart for which 1 is not a default.
    #[test]
    fn a_placeholder_chart_keeps_unit_arms() {
        let s: Surface<f64> = Surface::Nurbs(Arc::new(NurbsSurface::placeholder()));
        assert_eq!(azimuth_arm(&s, 0.0), 1.0);
        assert_eq!(v_meter(&s), 1.0);
    }
}
