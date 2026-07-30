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
//! - **Cone / sphere / torus / `Nurbs` charts mint nothing yet.** This
//!   is a compile-time routing decision per chart kind, permanent until
//!   a PR moves it — never a runtime fallback (C5). Their faces keep
//!   derive-on-demand status exactly as planar faces do, and a direct
//!   [`geom_brep::chart_pcurve`] call on one refuses typed, naming the
//!   arriving PR. Nothing consumes pcurves on a hot path before PR 11,
//!   so absence costs nothing today.
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

use geom_brep::{ChartWindow, Pcurve, PcurveCache, PcurveCertifyError, chart_pcurve};
use geom_core::predicate::{Band, BandError};
use geom_core::{Decide, Indeterminate, Real, Sign};
use geom_core::k_stats::decide;
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

/// Does this chart kind mint stored caches at M5 (module docs)? A
/// compile-time routing decision per surface kind, exhaustively
/// matched — adding a kind is a compiler-guided edit (D3).
fn chart_mints<T: Real>(surface: &Surface<T>) -> bool {
    match surface {
        Surface::Cylinder { .. } => true,
        // Planar faces keep M2's derive-on-demand status (C4 verbatim).
        Surface::Plane { .. } => false,
        // Frontier charts: no certified closed-form lane yet — their
        // pcurves arrive with the consumers that need them (PR 7/11).
        Surface::Cone { .. }
        | Surface::Sphere { .. }
        | Surface::Torus { .. }
        | Surface::Nurbs(_) => false,
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
        return Ok(*cache.pcurve());
    }
    let (carrier, _, _) = half_edge_carrier(body, half_edge)?;
    let surface = half_edge_surface(body, half_edge)?;
    chart_pcurve(&carrier, &surface, band).map_err(|error| PcurveMintError::Certify {
        half_edge,
        error,
    })
}

/// The certified carrier and parameter interval of `half_edge`'s edge.
fn half_edge_carrier<T: Decide>(
    body: &Body<T>,
    half_edge: HalfEdgeKey,
) -> Result<(geom_curves::Curve3<T>, T, T), PcurveMintError> {
    let he = body.get_half_edge(half_edge).ok_or(PcurveMintError::Corrupt)?;
    let edge = body.get_edge(he.edge).ok_or(PcurveMintError::Corrupt)?;
    let Some(CurveGeom::Certified(curve)) = body.get_curve_geom(edge.curve) else {
        return Err(PcurveMintError::Corrupt);
    };
    let (t0, t1) = curve.params();
    Ok((curve.carrier().clone(), t0, t1))
}

/// The surface of the face `half_edge` bounds.
fn half_edge_surface<T: Decide>(
    body: &Body<T>,
    half_edge: HalfEdgeKey,
) -> Result<Surface<T>, PcurveMintError> {
    let he = body.get_half_edge(half_edge).ok_or(PcurveMintError::Corrupt)?;
    let lp = body.get_loop(he.parent_loop).ok_or(PcurveMintError::Corrupt)?;
    let face = body.get_face(lp.face).ok_or(PcurveMintError::Corrupt)?;
    body.get_surface(face.surface)
        .cloned()
        .ok_or(PcurveMintError::Corrupt)
}

/// Is `half_edge` the `he_plus` of its edge (so the loop traverses it
/// forward in the carrier parameter)?
fn is_plus<T: Decide>(body: &Body<T>, half_edge: HalfEdgeKey) -> Result<bool, PcurveMintError> {
    let he = body.get_half_edge(half_edge).ok_or(PcurveMintError::Corrupt)?;
    let edge = body.get_edge(he.edge).ok_or(PcurveMintError::Corrupt)?;
    Ok(edge.he_plus == half_edge)
}

/// The azimuth lever arm of a chart (metres per radian) — how an
/// azimuth discrepancy is metered against the linear band (D4 ¶1: no
/// UV-space tolerance ever reaches ε).
fn azimuth_arm<T: Real>(surface: &Surface<T>) -> T {
    match *surface {
        Surface::Cylinder { radius, .. } => radius,
        _ => T::one(),
    }
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
        let cache =
            PcurveCache::certify(w.pcurve, w.t0, w.t1, &carrier, &surface, window, band).map_err(
                |error| PcurveMintError::Certify {
                    half_edge: w.half_edge,
                    error,
                },
            )?;
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
    let arm = azimuth_arm(surface);
    let tau = T::tau();
    // The walk's running exit point, in chart coordinates.
    let mut prev_exit: Option<geom_core::Point2<T>> = None;
    let mut first_entry: Option<geom_core::Point2<T>> = None;
    for he in cycle {
        let (carrier, t0, t1) = half_edge_carrier(body, he)?;
        let base = chart_pcurve(&carrier, surface, band).map_err(|error| {
            PcurveMintError::Certify {
                half_edge: he,
                error,
            }
        })?;
        let plus = is_plus(body, he)?;
        let (entry_t, exit_t) = if plus { (t0, t1) } else { (t1, t0) };
        let pcurve = match prev_exit {
            None => base,
            Some(prev) => {
                // The ONE branch decision of this half-edge: the whole
                // number of periods that lands its entry on the
                // predecessor's exit. Exact by construction — the two
                // chart points denote the SAME vertex, so their azimuths
                // differ by a whole period — and then CHECKED below, so
                // a body where it is not exact refuses rather than
                // snapping to the nearest branch.
                let raw = base.eval(entry_t);
                let half = T::from_f64(0.5);
                let k = ((prev.x - raw.x) / tau + half).floor();
                base.shift_branch(k, tau)
            }
        };
        // Certified continuity: the entry point meets the predecessor's
        // exit, metered in METRES (azimuth through the chart's lever
        // arm, height directly).
        let entry = pcurve.eval(entry_t);
        if let Some(prev) = prev_exit {
            for margin in [(entry.x - prev.x) * arm, entry.y - prev.y] {
                match decide("pcurve_loop_continuity", margin, band) {
                    Ok(Sign::Zero) => {}
                    Ok(Sign::Positive | Sign::Negative) => {
                        return Err(PcurveMintError::LoopDiscontinuity { half_edge: he });
                    }
                    Err(cause) => {
                        return Err(PcurveMintError::Escalated {
                            half_edge: he,
                            cause,
                        });
                    }
                }
            }
        }
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
    if let (Some(start), Some(end)) = (first_entry, prev_exit) {
        let du = end.x - start.x;
        let closes = [du, du - tau, du + tau]
            .into_iter()
            .any(|m| matches!(decide("pcurve_loop_closure", m * arm, band), Ok(Sign::Zero)));
        let height_closes = matches!(
            decide("pcurve_loop_closure_height", end.y - start.y, band),
            Ok(Sign::Zero)
        );
        if !(closes && height_closes) {
            return Err(PcurveMintError::LoopNotClosed { face });
        }
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
pub fn validate_pcurves<T: Decide>(body: &Body<T>, band: Band) -> Vec<PcurveMintError> {
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
                let Some(cache) = body.pcurve(he) else { continue };
                let carrier = match half_edge_carrier(body, he) {
                    Ok((c, _, _)) => c,
                    Err(e) => {
                        findings.push(e);
                        continue;
                    }
                };
                if let Err(error) = cache.recertify(&carrier, &surface, window, band) {
                    findings.push(PcurveMintError::Certify {
                        half_edge: he,
                        error,
                    });
                }
            }
        }
        // Pass 3: the one-branch loop continuity of the STORED pcurves.
        let arm = azimuth_arm(&surface);
        let tau = T::tau();
        for cycle in &cycles {
            let mut prev_exit: Option<geom_core::Point2<T>> = None;
            let mut first_entry: Option<geom_core::Point2<T>> = None;
            for &he in cycle {
                let Some(cache) = body.pcurve(he) else { continue };
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
                    for margin in [(entry.x - prev.x) * arm, entry.y - prev.y] {
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
            if let (Some(start), Some(end)) = (first_entry, prev_exit) {
                let du = end.x - start.x;
                let closes = [du, du - tau, du + tau].into_iter().any(|m| {
                    matches!(decide("pcurve_loop_closure", m * arm, band), Ok(Sign::Zero))
                });
                let height_closes = matches!(
                    decide("pcurve_loop_closure_height", end.y - start.y, band),
                    Ok(Sign::Zero)
                );
                if !(closes && height_closes) {
                    findings.push(PcurveMintError::LoopNotClosed { face: face_key });
                }
            }
        }
    }
    findings
}
