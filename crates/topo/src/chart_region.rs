//! **Chart-region overlap** — the trim-region intersection predicate a
//! `PatchContact` certifies through (CONTACT-DESIGN C3): two faces on
//! ONE chart, their trim loops read in `(u, v)`, the overlap answered
//! as *definitely-positive area* / *empty* / *typed refusal or
//! escalation* — never a guess (M9-2 PR-1; the census consumer is
//! PR-2's).
//!
//! # The certified lane is same-chart BY CONSTRUCTION
//!
//! "Overlap in the shared chart" presumes a shared chart. The lane
//! admits exactly the pairs whose chart identity is structural:
//!
//! - **at rest, one body**: the two faces carry the same `SurfaceKey`
//!   — one description, one chart, trivially;
//! - **cross-body, rung 2**: both surfaces carry the same
//!   [`crate::GeomSource`] — the N6 retirement theorem gives
//!   bit-identical descriptions, hence the identical chart.
//!
//! **Rung-3 (declared) pairs have exactly ONE further authority**: the
//! shared WORLD CARRIER of a PLANAR pair ([`declared_pair_overlap`],
//! ratified as U-R2 with its justification corrected 2026-08-27). That
//! carrier is a CHOICE OF REPRESENTATIVE FRAME, not a frame-free
//! object — a plane description carries `u_ref` too, and the arm reads
//! both trims in face A's frame. What makes the choice honest is the
//! **frame-invariance lemma** stated at [`world_carrier`], gated by
//! the `chart_region_carrier_tilt` row ([`carrier_agreement`]) which
//! meters the two descriptions' disagreement at the PAIR'S OWN EXTENT.
//! The claim it earns is *certified everywhere within ε*, never
//! *exact*: `decide`'s `Ok(Zero)` means `|m| ≤ zero`, not bit-zero.
//!
//! Every other declared pair keeps the typed escalation
//! [`ChartRegionError::ChartDivergence`]. C2's caveat — two
//! descriptions of one locus may differ as charts (`u_ref`, seam) — is
//! REAL for the curved kinds, where no world embedding arbitrates the
//! in-surface frame, so "exact in chart space" is unachievable there
//! and the honest posture stays a typed escalation naming the
//! divergence, not a margined pseudo-exact test in whichever chart we
//! happened to pick.
//!
//! # The planar trim inventory, defined structurally (C6)
//!
//! The area test is *exact in chart space* on loops whose chart images
//! are straight segments, and refuses typed outside that inventory —
//! the F5 census's envelope discipline moved to `(u, v)`. Membership is
//! read as STRUCTURE, never as a scalar zero-test on `T` (the
//! [`geom_brep::Pcurve::IsoLine`]-variant rationale is the binding
//! statement):
//!
//! - [`geom_brep::Pcurve::IsoLine`] — a straight segment by variant;
//! - [`geom_brep::Pcurve::IsoArc`] — its UV image is the straight
//!   segment `p0 → p0 + pd` by variant (only its *parameterization*
//!   is transcendental, and this module reads endpoints only);
//! - [`geom_brep::Pcurve::Harmonic`] whose trig channels (`pa`, `pb`)
//!   are **exact-`f64`-structural zeros** (the `props.rs`
//!   rectangle-trim precedent: a point bracket at exactly `0.0`) — a
//!   numerically-almost-zero channel REFUSES typed;
//! - planar faces' derive-on-demand affine images (their line edges
//!   chart to exactly this zero-trig `Harmonic` form).
//!
//! Everything else — sinusoid `Harmonic`s (the **tilted cylinder cut**
//! corpus is the honest exclusion this module names), `Fitted`, conic
//! trims in a plane chart — refuses [`ChartRegionError::NonPlanarTrim`].
//! Loop points come from **pcurve endpoints gated on variant**, never
//! from a chord-polygon read that would silently accept a curved loop.
//!
//! # Area, metered honestly
//!
//! The intersection region's chart-space shoelace (IsoArc segments
//! exact) crosses to model space through the chart's lever arms, and
//! the resulting margin is `over_lever(2·A, P)` — the region's **mean
//! width** in metres, the `split_section_area` precedent. The positive
//! claim is restricted to charts with **exact constant arms**: plane
//! `(1, 1)` (chart coordinates are metres) and cylinder `(r, 1)`
//! (azimuth metres per radian is exactly `r` everywhere).
//! `chart_arms`' documented over-statement (`sup` stretch bounds) is
//! safe for escape-metering and UNSAFE here — an over-stated arm
//! inflates the margin and would certify a model-space sliver as
//! definitely positive; the safe direction needs lower stretch bounds
//! (`inf |S_u|`, `inf |S_v|`), which do not exist. NURBS, sphere,
//! torus and cone charts therefore refuse
//! [`ChartRegionError::ArmUnbounded`] (the inf-bounds extension is the
//! named follow-up issue filed with this unit's PR).
//!
//! # Seam branches
//!
//! A periodic chart's loops are minted on a pinned branch. A pair
//! whose loops cannot sit inside ONE period-wide azimuth window has no
//! common-branch region representation ([`geom_brep::ChartWindow::hull`]
//! is branchless min/max), and refuses
//! [`ChartRegionError::SeamBranch`] — branch normalization is a
//! possible later rung, not this unit's.
//!
//! # Determinism and decision honesty (D9, Q1)
//!
//! Every schedule here is fixed: loop walks in cycle order (face A
//! then face B, outer then rings), edge pairs in loop order, the 2-D
//! ray schedule a fixed constant table, crossings ordered by
//! (edge index, advance). Every decision goes through a named
//! `decide` row with a `Margin` door (no `decide_flagged` site
//! exists in this module); each row's metering derivation lives at
//! its call site. Three-outcome honesty everywhere: definite margins
//! walk on, exact zeros take their stated branch, the in-band residue
//! escalates typed.

use geom::Surface;
use geom_core::{Band, Bounds, CertifiedBounds, Decide, Indeterminate, Margin, Point2, Sign, Vec2};

use crate::body::Body;
use crate::entity::{FaceKey, HalfEdgeKey, LoopBoundary, LoopKey};
use crate::null::CurveGeom;
use crate::ray_parity::{self, ParityRows};
use crate::validate::decide;

/// The certified overlap answer (both outcomes are *definite*; every
/// non-definite configuration is a typed error).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChartOverlap {
    /// The trim regions' intersection has definitely-positive area
    /// (mean width definitely above the band) — the `PatchContact`
    /// blessing direction.
    PositiveArea,
    /// The trim regions are definitely disjoint — a `PatchContact`
    /// claiming this pair is stale.
    Empty,
}

/// Typed refusal/escalation of [`chart_region_overlap`] (closed enum,
/// D3 style: every arm names its recourse).
#[derive(Debug)]
pub enum ChartRegionError {
    /// The pair has no structural chart identity (rung 3 or below):
    /// C2's caveat — two descriptions of one locus may differ as
    /// charts — makes a chart-space test unachievable; recourse is a
    /// structural identity (shared key / same `GeomSource`), not a
    /// numeric chart comparison.
    ChartDivergence {
        /// What was missing (same-key, same-source, …).
        detail: &'static str,
    },
    /// A loop's chart image is outside the planar trim inventory
    /// (module docs) — a sinusoid `Harmonic` (the tilted-cut class),
    /// a `Fitted` image, or a trig channel that is not an
    /// exact-structural zero.
    NonPlanarTrim {
        /// The face whose loop refused.
        face: FaceKey,
        /// The half-edge carrying the refusing pcurve.
        half_edge: HalfEdgeKey,
        /// The inventory statement violated.
        what: &'static str,
    },
    /// A minting chart's stored cache is absent — the body must
    /// re-mint before region queries (the `props.rs` posture); only
    /// plane charts are derive-on-demand.
    MissingCache {
        /// The half-edge with no stored cache.
        half_edge: HalfEdgeKey,
    },
    /// The chart has no exact constant lever arms (NURBS, sphere,
    /// torus, cone): `sup` stretch bounds over-state in the unsafe
    /// direction for a positive-area claim and no `inf` bounds exist —
    /// the named follow-up extension.
    ArmUnbounded {
        /// The chart kind that refused.
        chart: &'static str,
    },
    /// The pair's loops do not fit one period-wide azimuth window on
    /// a periodic chart — no common-branch region representation
    /// exists (module docs).
    SeamBranch,
    /// A declared PLANAR pair's two carriers are definitely apart
    /// somewhere over the pair's OWN extent (`chart_region_carrier_tilt`):
    /// Door 1 certified the carriers at its pinned 1 m arm, and at this
    /// pair's actual size they do not agree, so neither description's
    /// frame is a representative of the other and the world-carrier arm
    /// has no chart to answer in. Recourse is the geometry, not the
    /// declaration: a contact the size of a table is not certified by a
    /// tilt that a peg would absorb.
    CarrierTilt,
    /// The pair's boundaries touch (collinear overlap, a crossing at
    /// an endpoint, coincident-but-not-bit-identical loops): the
    /// overlap area is not decidable at this ε in either direction.
    TouchingBoundary,
    /// A loop's chart polygon has no definite orientation (degenerate
    /// at this ε).
    DegenerateLoop {
        /// The face whose loop degenerated.
        face: FaceKey,
        /// The loop.
        r#loop: LoopKey,
    },
    /// A margin landed in the sliver band (in-band overlap, in-band
    /// crossing, in-band area) — the genuine escalation.
    Escalated(Indeterminate),
    /// Every ray of the fixed 2-D schedule grazed — an
    /// ill-conditioned containment query at this ε.
    RayExhausted,
    /// The topology could not be walked, or the crossing walk
    /// contradicted itself (fail-loud kernel-invariant class).
    Corrupt,
}

impl core::fmt::Display for ChartRegionError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::ChartDivergence { detail } => write!(
                f,
                "chart-region: no structural chart identity ({detail}) — a declared \
                 (rung-3) pair escalates: two descriptions of one locus may differ \
                 as charts, so no chart-space overlap test exists for it"
            ),
            Self::NonPlanarTrim {
                face,
                half_edge,
                what,
            } => write!(
                f,
                "chart-region: face {face:?} half-edge {half_edge:?} is outside the \
                 planar trim inventory ({what}) — the area test is exact on straight \
                 chart segments and refuses typed beyond them (the tilted-cut class \
                 is the named exclusion)"
            ),
            Self::MissingCache { half_edge } => write!(
                f,
                "chart-region: half-edge {half_edge:?} has no stored pcurve cache on \
                 a minting chart — re-mint pcurves before region queries"
            ),
            Self::ArmUnbounded { chart } => write!(
                f,
                "chart-region: a {chart} chart has no exact constant lever arms — the \
                 positive-area claim needs lower stretch bounds (inf |S_u|, inf |S_v|),\
                 which are the named follow-up extension; plane and cylinder charts \
                 are the certified lane"
            ),
            Self::SeamBranch => write!(
                f,
                "chart-region: the pair's loops do not fit one period-wide azimuth \
                 window — different periodic branches have no common region \
                 representation (branch normalization is a later rung)"
            ),
            Self::CarrierTilt => write!(
                f,
                "chart-region: a declared planar pair's two carriers are definitely \
                 apart over the pair's OWN extent — the world-carrier arm needs the \
                 two descriptions to agree everywhere the trims reach, and Door 1's \
                 1 m lever arm does not price a contact at its own size"
            ),
            Self::TouchingBoundary => write!(
                f,
                "chart-region: the trim boundaries touch at this ε — overlap area is \
                 not decidable in either direction"
            ),
            Self::DegenerateLoop { face, r#loop } => write!(
                f,
                "chart-region: loop {loop:?} of face {face:?} has no definite chart \
                 orientation (degenerate polygon at this ε)"
            ),
            Self::Escalated(diag) => write!(f, "chart-region: escalated: {diag}"),
            Self::RayExhausted => write!(
                f,
                "chart-region: every schedule ray grazed — ill-conditioned \
                 containment query at this ε"
            ),
            Self::Corrupt => write!(
                f,
                "chart-region: unwalkable topology or a \
                 self-contradictory crossing walk"
            ),
        }
    }
}

impl std::error::Error for ChartRegionError {}

/// **The per-scalar chart-region lane** — the static split that lets a
/// `Decide`-generic consumer (the census arms) hold a dual body without
/// holding a chart-region predicate: bracket-carrying scalars (`f64`,
/// `Probe`, the interval scalar) reach [`chart_region_overlap`]; the
/// dual scalar REFUSES statically, its impl instantiating none of the
/// predicate (the `PropsQuadLane` shape). The census maps that `None`
/// to its typed unsupported refusal.
///
/// **What this lane is for, and what it is not.** It is what lets a
/// MIXED pass keep going at a dual — no bound on a whole function can
/// say *"this arm certifies, the rest does not"*, so the arm carries
/// its own refusal. It is **not** what keeps a dual out of the
/// predicate: [`chart_region_overlap`]'s own bound is
/// `Decide + `[`CertifiedBounds`], which [`geom_core::Dual`] does not
/// satisfy, so the door refuses an external caller structurally whether
/// or not this lane is consulted. That matches the other three lanes'
/// doors, all of which carry [`geom_core::CertifiedEnclosure`]. See the
/// M9-2 entry in `geom-core/src/real.rs`'s `Bounds` scope rule.
pub trait ChartRegionLane: Decide {
    /// The overlap door at this scalar, or `None` when the scalar has
    /// no certified lane (dual) — the census maps `None` to its typed
    /// unsupported refusal, never to a silent skip.
    fn chart_overlap(
        body_a: &Body<Self>,
        face_a: FaceKey,
        body_b: &Body<Self>,
        face_b: FaceKey,
        band: Band,
    ) -> Option<Result<ChartOverlap, ChartRegionError>>;

    /// The same door for a pair whose chart authority may be a
    /// VERIFIED DECLARATION — [`declared_pair_overlap`], which adds the
    /// world-carrier arm below the structural rung and therefore
    /// demands Door 1's verdict in hand. `None` carries the same
    /// meaning: no certified lane at this scalar.
    fn declared_overlap(
        body_a: &Body<Self>,
        face_a: FaceKey,
        body_b: &Body<Self>,
        face_b: FaceKey,
        door_one: crate::contact::ContactVerdict,
        band: Band,
    ) -> Option<Result<ChartOverlap, ChartRegionError>>;
}

impl ChartRegionLane for f64 {
    fn chart_overlap(
        body_a: &Body<Self>,
        face_a: FaceKey,
        body_b: &Body<Self>,
        face_b: FaceKey,
        band: Band,
    ) -> Option<Result<ChartOverlap, ChartRegionError>> {
        Some(chart_region_overlap(body_a, face_a, body_b, face_b, band))
    }

    fn declared_overlap(
        body_a: &Body<Self>,
        face_a: FaceKey,
        body_b: &Body<Self>,
        face_b: FaceKey,
        door_one: crate::contact::ContactVerdict,
        band: Band,
    ) -> Option<Result<ChartOverlap, ChartRegionError>> {
        Some(declared_pair_overlap(
            body_a, face_a, body_b, face_b, door_one, band,
        ))
    }
}

#[cfg(feature = "probe")]
impl ChartRegionLane for geom_core::Probe {
    fn chart_overlap(
        body_a: &Body<Self>,
        face_a: FaceKey,
        body_b: &Body<Self>,
        face_b: FaceKey,
        band: Band,
    ) -> Option<Result<ChartOverlap, ChartRegionError>> {
        Some(chart_region_overlap(body_a, face_a, body_b, face_b, band))
    }

    fn declared_overlap(
        body_a: &Body<Self>,
        face_a: FaceKey,
        body_b: &Body<Self>,
        face_b: FaceKey,
        door_one: crate::contact::ContactVerdict,
        band: Band,
    ) -> Option<Result<ChartOverlap, ChartRegionError>> {
        Some(declared_pair_overlap(
            body_a, face_a, body_b, face_b, door_one, band,
        ))
    }
}

#[cfg(feature = "interval")]
impl ChartRegionLane for geom_core::interval::Interval {
    fn chart_overlap(
        body_a: &Body<Self>,
        face_a: FaceKey,
        body_b: &Body<Self>,
        face_b: FaceKey,
        band: Band,
    ) -> Option<Result<ChartOverlap, ChartRegionError>> {
        Some(chart_region_overlap(body_a, face_a, body_b, face_b, band))
    }

    fn declared_overlap(
        body_a: &Body<Self>,
        face_a: FaceKey,
        body_b: &Body<Self>,
        face_b: FaceKey,
        door_one: crate::contact::ContactVerdict,
        band: Band,
    ) -> Option<Result<ChartOverlap, ChartRegionError>> {
        Some(declared_pair_overlap(
            body_a, face_a, body_b, face_b, door_one, band,
        ))
    }
}

/// The dual lane: statically no chart-region predicate (trait docs).
impl<T> ChartRegionLane for geom_core::Dual<T>
where
    geom_core::Dual<T>: Decide,
{
    fn chart_overlap(
        _body_a: &Body<Self>,
        _face_a: FaceKey,
        _body_b: &Body<Self>,
        _face_b: FaceKey,
        _band: Band,
    ) -> Option<Result<ChartOverlap, ChartRegionError>> {
        None
    }

    fn declared_overlap(
        _body_a: &Body<Self>,
        _face_a: FaceKey,
        _body_b: &Body<Self>,
        _face_b: FaceKey,
        _door_one: crate::contact::ContactVerdict,
        _band: Band,
    ) -> Option<Result<ChartOverlap, ChartRegionError>> {
        None
    }
}

/// The diagnostic for a DEFINITE margin whose outcome is nevertheless
/// uncertifiable (the conservative-deduction escalations): the margin
/// was validly posed and classified — `MarginDiag::Invalid` would
/// claim otherwise — so the diag echoes the classified value itself
/// (its conservative bracket end), named to its row.
fn definite_diag<T: Bounds>(
    band: Band,
    predicate: &'static str,
    margin: Margin<T>,
) -> Indeterminate {
    Indeterminate {
        margin: geom_core::MarginDiag::Value(margin.value().lo()),
        band,
        predicate: Some(predicate),
    }
}

/// **The predicate** (module docs): trilean-honest trim-region overlap
/// of two faces on one structurally-identified chart. Pass the same
/// `&Body` twice for the at-rest (one-body) site.
///
/// # Errors
///
/// [`ChartRegionError`] — chart divergence (rung 3), inventory or arm
/// refusals, seam-branch refusal, touch/degenerate configurations,
/// in-band escalations, unwalkable topology.
///
/// # Scalars
///
/// The bound is `Decide + `[`CertifiedBounds`]: this door's `Ok` is a
/// grant, so it is open to exactly the scalars that can certify. A
/// bracket-carrying scalar passes —
///
/// ```
/// use geom_core::Band;
/// use topo::{Body, FaceKey, chart_region_overlap};
/// fn admitted(b: &Body<f64>, f: FaceKey, band: Band) {
///     let _ = chart_region_overlap(b, f, b, f, band);
/// }
/// ```
///
/// — and [`Dual`](geom_core::Dual) does not, whether or not
/// [`ChartRegionLane`] is consulted:
///
/// ```compile_fail,E0277
/// use geom_core::{Band, Dual64};
/// use topo::{Body, FaceKey, chart_region_overlap};
/// fn evicted(b: &Body<Dual64>, f: FaceKey, band: Band) {
///     let _ = chart_region_overlap(b, f, b, f, band);
/// }
/// ```
pub fn chart_region_overlap<T: Decide + CertifiedBounds>(
    body_a: &Body<T>,
    face_a: FaceKey,
    body_b: &Body<T>,
    face_b: FaceKey,
    band: Band,
) -> Result<ChartOverlap, ChartRegionError> {
    // 1. Chart identity (fixed gate order, D9: identity → inventory →
    //    arms → seam → machinery).
    let surface = same_chart(body_a, face_a, body_b, face_b)?;
    overlap_on(
        body_a,
        face_a,
        body_b,
        face_b,
        &surface,
        ChartRead::Minted,
        band,
    )
}

/// **The declared pair's overlap door** — what a `PatchContact` whose
/// chart authority may be a VERIFIED DECLARATION (rung 3) certifies
/// through, in the fixed order structural-identity-first.
///
/// Two authorities answer the same question:
///
/// - [`same_chart`] — the descriptions are structurally ONE chart
///   (shared key / same `GeomSource`), so the trims are read in it
///   directly. Strictly stronger, so it is asked first.
/// - the **shared world carrier**, PLANAR pairs only
///   ([`world_carrier`]): a representative frame, legitimate exactly
///   to the extent of that function's frame-invariance lemma, and only
///   once [`carrier_agreement`] has certified that the two
///   descriptions agree over the PAIR'S OWN EXTENT.
///
/// A pair with neither keeps [`same_chart`]'s typed divergence —
/// including a declared CURVED cross-instance pair, whose `u_ref`/seam
/// divergence is real and whose closure is a certified
/// everywhere-within-ε overlap enclosure on the shared curved carrier
/// (CONTACT-DESIGN C3), not this arm.
///
/// # `door_one` — why the verdict is an argument, not a re-derivation
///
/// The world carrier is a carrier BECAUSE the declaration was
/// verified, so this door is not independent of Door 1 and the type
/// now says so: it cannot be reached without the verdict that verified
/// the pair. It is READ in exactly one place — the interior-witness
/// rung, which runs only on [`ContactVerdict::Definite`]
/// ([`interior_witness`] states why).
///
/// [`ContactVerdict::Definite`]: crate::contact::ContactVerdict::Definite
///
/// # Errors
///
/// [`ChartRegionError`], as [`chart_region_overlap`], plus
/// [`ChartRegionError::CarrierTilt`].
pub fn declared_pair_overlap<T: Decide + CertifiedBounds>(
    body_a: &Body<T>,
    face_a: FaceKey,
    body_b: &Body<T>,
    face_b: FaceKey,
    door_one: crate::contact::ContactVerdict,
    band: Band,
) -> Result<ChartOverlap, ChartRegionError> {
    let divergence = match same_chart(body_a, face_a, body_b, face_b) {
        Ok(surface) => {
            return overlap_on(
                body_a,
                face_a,
                body_b,
                face_b,
                &surface,
                ChartRead::Minted,
                band,
            );
        }
        Err(divergence) => divergence,
    };
    let carrier = world_carrier(body_a, face_a, body_b, face_b, divergence)?;
    // The gate that makes the representative choice honest — and the
    // one that discharges `contfp`'s on-plane precondition below.
    carrier_agreement(body_a, face_a, body_b, face_b, band)?;
    let uv_a = extract_face_uv(body_a, face_a, &carrier, ChartRead::WorldCarrier, band)?;
    let uv_b = extract_face_uv(body_b, face_b, &carrier, ChartRead::WorldCarrier, band)?;
    match overlap_of_uv(body_a, face_a, body_b, face_b, &carrier, &uv_a, &uv_b, band) {
        // INVARIANT: a shared trim boundary is what a FLUSH seat is
        // made of, and it is not by itself an undecidable area. The
        // region walk cannot build intersection pieces across a
        // collinear boundary overlap and refuses honestly; a point
        // certified strictly interior to BOTH trims proves the
        // intersection contains a disc of definitely-positive radius,
        // which IS the positive claim. The rung only ever turns a
        // REFUSAL into a proof — it can neither contradict a decided
        // answer nor manufacture an `Empty`.
        Err(ChartRegionError::TouchingBoundary)
            if interior_witness(
                body_a, face_a, body_b, face_b, &carrier, &uv_a, &uv_b, door_one, band,
            ) =>
        {
            Ok(ChartOverlap::PositiveArea)
        }
        other => other,
    }
}

/// Which description a loop walk's chart polygon is a picture OF.
///
/// INVARIANT: a stored pcurve cache is the image of the face's OWN
/// minting chart, so reading it against a foreign chart would be a
/// picture of the wrong description. A carrier chart the face does not
/// itself describe admits only the image derived from each boundary
/// edge's WORLD curve.
#[derive(Clone, Copy, PartialEq, Eq)]
enum ChartRead {
    /// The face's own chart: stored cache, else the plane
    /// derive-on-demand image.
    Minted,
    /// A shared world carrier: every boundary edge's world curve read
    /// into THIS chart, caches never consulted (plane charts only —
    /// the affine image is exact and branchless).
    WorldCarrier,
}

/// The pipeline below the chart-authority gate (fixed order, D9:
/// identity → inventory → arms → seam → machinery), run on whichever
/// chart the authority produced.
fn overlap_on<T: Decide + Bounds>(
    body_a: &Body<T>,
    face_a: FaceKey,
    body_b: &Body<T>,
    face_b: FaceKey,
    surface: &Surface<T>,
    read: ChartRead,
    band: Band,
) -> Result<ChartOverlap, ChartRegionError> {
    // 2. UV loop extraction, variant-gated (face A then face B, outer
    //    then rings, cycle order).
    let uv_a = extract_face_uv(body_a, face_a, surface, read, band)?;
    let uv_b = extract_face_uv(body_b, face_b, surface, read, band)?;
    overlap_of_uv(body_a, face_a, body_b, face_b, surface, &uv_a, &uv_b, band)
}

/// The pipeline below the loop extraction: arms, seam, machinery.
#[allow(clippy::too_many_arguments)] // the pipeline's whole state, no less
fn overlap_of_uv<T: Decide + Bounds>(
    body_a: &Body<T>,
    face_a: FaceKey,
    body_b: &Body<T>,
    face_b: FaceKey,
    surface: &Surface<T>,
    uv_a: &FaceUv<T>,
    uv_b: &FaceUv<T>,
    band: Band,
) -> Result<ChartOverlap, ChartRegionError> {
    // 3. Exact constant arms or a typed refusal.
    let (arm_u, arm_v) = exact_arms(surface)?;

    // 4. Seam-branch gate (periodic charts).
    seam_gate(surface, uv_a, uv_b, band)?;

    // 5. Chart-space machinery on metred coordinates.
    let a = ScaledFace::build(body_a, face_a, uv_a, arm_u, arm_v, band)?;
    let b = ScaledFace::build(body_b, face_b, uv_b, arm_u, arm_v, band)?;
    let same_face = core::ptr::eq(body_a, body_b) && face_a == face_b;
    overlap_of_regions(&a, &b, same_face, band)
}

/// **The shared world carrier of a declared PLANAR pair** — face A's
/// plane description, taken as the pair's REPRESENTATIVE FRAME.
///
/// # This is a choice of frame, and this is the lemma that licenses it
///
/// [`geom::Surface::Plane`] carries `origin`, `normal` AND `u_ref`,
/// and `u_ref` is documented as carrying the in-plane frame
/// convention — a plane description DOES have chart parameters, so
/// "the world embedding has none" is false about the type. This
/// function returns `s_a`: it picks **A's frame**, and a real seat's
/// two descriptions disagree on all three fields (a `Rest` pair's
/// normals are opposed by construction). What licenses the choice is
/// not an absence of parameters. It is that the ANSWER does not depend
/// on it.
///
/// **Lemma (frame invariance).** Let `φ_A(u, v) = o_a + u·û_a + v·v̂_a`
/// with `v̂ = n̂ × û`, and `φ_B` likewise. Each is an ISOMETRY of the
/// Euclidean plane onto its carrier, because `û` and `v̂` are
/// orthonormal. When [`carrier_agreement`] decides `Zero`, the two
/// carriers coincide to within ε over the pair's own extent, so there
/// `ψ = φ_B⁻¹ ∘ φ_A` is — within ε — a plane isometry: a rotation, a
/// translation, and possibly a reflection.
///
/// Everything this module computes downstream of the loop walk is a
/// Euclidean invariant of the plane, hence fixed by `ψ`:
/// `loop_measures`' shoelace `|2A|` and perimeter, `proper_crossings`'
/// incidence, `polygon_relation`'s containment, [`interior_witness`]'s
/// strict interiority, and the final `over_lever(2A, P)` mean width.
/// ORIENTATION is the one thing a reflection does not fix — and it is
/// exactly the field an opposed pair differs in — but it is absorbed
/// STRUCTURALLY rather than numerically: `ScaledFace::build`
/// normalizes every loop to CCW through the
/// `chart_region_orientation` trilean before any of that machinery
/// runs. The metering is likewise unmoved, because a plane chart's
/// [`exact_arms`] are `(1, 1)` in either frame — a plane chart's
/// coordinates ARE metres. Therefore [`ChartOverlap::PositiveArea`]
/// and [`ChartOverlap::Empty`] are invariant under which description
/// is taken as representative, which is the claim. ∎
///
/// # What the lemma does NOT claim (stated because it matters)
///
/// The CERTIFIED answers are invariant. The REFUSAL boundary is not
/// exactly: [`crate::ray_parity`]'s schedule is a fixed table of
/// directions in CHART coordinates, so which ray fires — and whether a
/// near-grazing configuration escalates or decides — rotates with the
/// frame; and `ψ` is not exactly representable in `f64`, so a margin
/// within a few ulps of a band edge may classify differently in the
/// two frames. That is the module's standing posture, not a hole in
/// it: the claim earned here is *certified everywhere within ε*, and
/// `decide`'s `Ok(Zero)` has never meant bit-zero. Argument-order
/// symmetry is pinned as a row on the fixture corpus
/// (`topo/tests/m9_c1_rest_face_rung.rs`), and
/// [`carrier_agreement`]'s own margin is symmetric BY CONSTRUCTION —
/// see there.
///
/// # Planar only
///
/// A curved pair keeps the divergence it arrived with. Two
/// independently authored curved descriptions of one locus differ in
/// `u_ref` and seam, no world embedding arbitrates that, and there is
/// no isometry lemma to be had: the chart map of a cylinder is not an
/// isometry of the parameter rectangle in the azimuth direction unless
/// the two radii agree exactly, and the seam makes containment itself
/// branch-dependent.
///
/// # Errors
///
/// The `divergence` it was handed, for any pair that is not
/// plane-on-plane; [`ChartRegionError::Corrupt`] for unwalkable
/// topology.
fn world_carrier<T: Decide + Bounds>(
    body_a: &Body<T>,
    face_a: FaceKey,
    body_b: &Body<T>,
    face_b: FaceKey,
    divergence: ChartRegionError,
) -> Result<Surface<T>, ChartRegionError> {
    let s_a = face_surface(body_a, face_a)?;
    let s_b = face_surface(body_b, face_b)?;
    if matches!(s_a, Surface::Plane { .. }) && matches!(s_b, Surface::Plane { .. }) {
        Ok(s_a.clone())
    } else {
        Err(divergence)
    }
}

/// A face's own surface description.
fn face_surface<T: Decide>(body: &Body<T>, face: FaceKey) -> Result<&Surface<T>, ChartRegionError> {
    let key = body
        .get_face(face)
        .ok_or(ChartRegionError::Corrupt)?
        .surface;
    body.get_surface(key).ok_or(ChartRegionError::Corrupt)
}

/// A face's plane description as `(origin, unit normal)`; the CHART
/// normal, unsigned — a point's distance to a plane is blind to which
/// side the material is on, and the sense question is Door 1's.
fn plane_frame<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
) -> Result<(geom_core::Point3<T>, geom_core::Vec3<T>), ChartRegionError> {
    match face_surface(body, face)? {
        Surface::Plane { origin, normal, .. } => Ok((*origin, *normal)),
        _ => Err(ChartRegionError::Corrupt),
    }
}

/// Every boundary vertex of a face, outer loop then rings, cycle order
/// (fixed schedule, D9).
fn face_boundary_points<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
) -> Result<Vec<geom_core::Point3<T>>, ChartRegionError> {
    let face_data = body.get_face(face).ok_or(ChartRegionError::Corrupt)?;
    let mut out = Vec::new();
    for lk in core::iter::once(face_data.outer).chain(face_data.rings.iter().copied()) {
        let loop_data = body.get_loop(lk).ok_or(ChartRegionError::Corrupt)?;
        let LoopBoundary::Cycle { first } = loop_data.boundary else {
            return Err(ChartRegionError::Corrupt);
        };
        for he in body.loop_cycle(first).ok_or(ChartRegionError::Corrupt)? {
            let v = body
                .get_half_edge(he)
                .ok_or(ChartRegionError::Corrupt)?
                .start;
            let pk = body.get_vertex(v).ok_or(ChartRegionError::Corrupt)?.point;
            out.push(*body.get_point(pk).ok_or(ChartRegionError::Corrupt)?);
        }
    }
    Ok(out)
}

/// **`chart_region_carrier_tilt`** — the metered gate that turns
/// [`world_carrier`]'s choice of representative from convenient into
/// honest: do the pair's two plane descriptions agree, everywhere the
/// pair's own trims reach?
///
/// # The margin, and why its lever is the pair's own extent
///
/// `m = max over the union of BOTH faces' boundary vertices p of
/// max(|(p − o_a)·n̂_a|, |(p − o_b)·n̂_b|)` — metres, a point-to-plane
/// distance, [`Margin::of`]'s door.
///
/// A vertex of face A lies on A's carrier exactly, so its A-term is
/// zero and its B-term is the two carriers' SEPARATION at that
/// location; symmetrically for B's vertices. So `m` is the largest
/// separation of the two carriers anywhere on the pair's own boundary.
/// Each term is affine in `p`, so its supremum over a trim region is
/// attained at a vertex: `m` is EXACT over the region, not a
/// small-angle bound.
///
/// **The lever is not a constant.** Door 1 meters the same
/// disagreement as an angle at a PINNED 1 m arm (`bool_plane_parallel`
/// via `carrier_pair_verdict`'s `T::one()`), which prices a peg and a
/// table identically. Here the tilt's contribution to each term is
/// `r·sin θ` with `r` that vertex's own distance from the carrier
/// origin — so the pair's own extent IS the lever, per vertex, and the
/// offset term rides in the same length. A tilt a peg absorbs and a
/// tilt that opens a millimetre across a table get different answers,
/// which is the whole point.
///
/// **The margin is symmetric BY CONSTRUCTION** (the argument-order
/// obligation): the vertex set is a UNION and both plane-distance
/// functions are evaluated at every vertex, so swapping `(A, B)` for
/// `(B, A)` permutes a `max` over an unchanged multiset of `f64`s.
/// `T::max` is associative-commutative on point brackets, so the
/// margin is bit-identical in both orders — no appeal to the lemma's
/// ε framing is needed for THIS row.
///
/// # Three outcomes
///
/// - `Zero` — the carriers agree within ε over the pair's own extent.
///   The world carrier is an honest chart for this pair, and any point
///   built on it inside the region is on BOTH planes within ε, which
///   is what discharges `contfp`'s precondition in
///   [`interior_witness`].
/// - `Positive` — the carriers are definitely apart somewhere the
///   trims reach: [`ChartRegionError::CarrierTilt`]. Door 1's verdict
///   is not contradicted (it was true at its own arm); this pair is
///   simply too big for it.
/// - in-band — [`ChartRegionError::Escalated`], the genuine residue.
///
/// A definitely-NEGATIVE margin is unreachable (a max of absolute
/// values), so it is poisoned input and escalates as `Invalid` — the
/// `bool_plane_parallel` precedent.
///
/// # Errors
///
/// [`ChartRegionError`] — the tilt refusal, an escalation, or
/// unwalkable topology.
fn carrier_agreement<T: Decide + Bounds>(
    body_a: &Body<T>,
    face_a: FaceKey,
    body_b: &Body<T>,
    face_b: FaceKey,
    band: Band,
) -> Result<(), ChartRegionError> {
    let (o_a, n_a) = plane_frame(body_a, face_a)?;
    let (o_b, n_b) = plane_frame(body_b, face_b)?;
    let mut worst = T::zero();
    for (body, face) in [(body_a, face_a), (body_b, face_b)] {
        for p in face_boundary_points(body, face)? {
            worst = worst
                .max((p - o_a).dot(n_a).abs())
                .max((p - o_b).dot(n_b).abs());
        }
    }
    match decide("chart_region_carrier_tilt", Margin::of(worst), band) {
        Ok(Sign::Zero) => Ok(()),
        Ok(Sign::Positive) => Err(ChartRegionError::CarrierTilt),
        Ok(Sign::Negative) => Err(ChartRegionError::Escalated(Indeterminate {
            margin: geom_core::MarginDiag::Invalid,
            band,
            predicate: Some("chart_region_carrier_tilt"),
        })),
        Err(diag) => Err(ChartRegionError::Escalated(diag)),
    }
}

/// **The interior-witness rung** — `true` when the fixed candidate
/// schedule exhibits a point strictly inside BOTH faces, run through
/// the census's own `contfp` on the shared carrier.
///
/// The rung exists because the world carrier alone does not close a
/// FLUSH seat: a shared trim edge is a collinear boundary overlap, the
/// region walk cannot build intersection pieces across one, and it
/// refuses [`ChartRegionError::TouchingBoundary`] honestly. A point
/// strictly interior to both trims proves the intersection contains a
/// disc of definitely-positive radius, which is the positive claim.
///
/// # The precondition, discharged rather than assumed
///
/// `contfp` requires `q` **already on the plane of `face`** — its
/// contract, and the defect that made `m9/census-xid` unusable, which
/// built `q` on A's plane and fed it to B without ever discharging it.
/// Here it is discharged twice over. `q` is built by A's own frame
/// identity `o + u·û + v·v̂`, so it is on A's carrier exactly; and
/// [`carrier_agreement`] has already decided that the two carriers
/// agree within ε everywhere the trims reach, so `q` is on B's carrier
/// within ε — which is the same standard every other on-plane caller
/// in the kernel meets.
///
/// # Why Door 1's verdict gates this rung and not the region walk
///
/// The region walk reads both trims INTO the carrier and measures
/// there; the carrier is a space to measure in, and
/// [`carrier_agreement`] is its whole warrant. This rung is different:
/// it converts a refusal into a proof by asserting a point lies ON
/// both planes. A `Bridged` verdict says precisely that the carriers'
/// coincidence rests on the DECLARATION — the geometry left a residue
/// in band and the declaration bridged it — and a precondition may not
/// be discharged by the claim under test. So the rung runs only on
/// [`ContactVerdict::Definite`](crate::contact::ContactVerdict::Definite);
/// on `Bridged` it declines and the region walk's typed refusal
/// stands. Three-outcome honest: a proof, a decline, or the refusal it
/// was already carrying.
///
/// The schedule is each outer trim's vertex centroid then its ear
/// midpoints `(v[i−1] + v[i+1])/2`, face A's trim then face B's, in
/// cycle order (fixed, D9). Every `contfp` verdict but `In` — a
/// boundary coincidence, an exterior point, an in-band containment, an
/// exhausted ray schedule — means THIS candidate proves nothing and
/// the walk moves on; the rung's whole output is a proof or its
/// absence. A candidate is never counted from one face alone: `contfp`
/// reads each face's own rings, so a point in a hole of either is
/// `Out` of that face.
#[allow(clippy::too_many_arguments)] // the rung's whole state, no less
fn interior_witness<T: Decide + Bounds>(
    body_a: &Body<T>,
    face_a: FaceKey,
    body_b: &Body<T>,
    face_b: FaceKey,
    carrier: &Surface<T>,
    uv_a: &FaceUv<T>,
    uv_b: &FaceUv<T>,
    door_one: crate::contact::ContactVerdict,
    band: Band,
) -> bool {
    if door_one != crate::contact::ContactVerdict::Definite {
        return false;
    }
    let Surface::Plane {
        origin,
        normal,
        u_ref,
    } = *carrier
    else {
        return false;
    };
    let v_ref = normal.cross(u_ref);
    let Ok((_, normal_b)) = plane_frame(body_b, face_b) else {
        return false;
    };
    let inside = |body: &Body<T>, face: FaceKey, n, q| {
        matches!(
            crate::boolean::contfp(body, face, n, q, band),
            Ok(crate::boolean::FaceContainment::In)
        )
    };
    for poly in [&uv_a.outer, &uv_b.outer] {
        for c in candidate_points(poly) {
            let q = origin + u_ref * c.x + v_ref * c.y;
            if inside(body_a, face_a, normal, q) && inside(body_b, face_b, normal_b, q) {
                return true;
            }
        }
    }
    false
}

/// The witness schedule of one trim polygon (fixed order, D9): its
/// vertex centroid, then each ear midpoint in cycle order.
fn candidate_points<T: Decide>(poly: &[Point2<T>]) -> Vec<Point2<T>> {
    let n = poly.len();
    if n < 3 {
        return Vec::new();
    }
    let half = T::one() / (T::one() + T::one());
    let mut sum = Point2::new(T::zero(), T::zero());
    for p in poly {
        sum = Point2::new(sum.x + p.x, sum.y + p.y);
    }
    let count = T::from_f64(n as f64);
    let mut out = vec![Point2::new(sum.x / count, sum.y / count)];
    for i in 0..n {
        let prev = poly[(i + n - 1) % n];
        let next = poly[(i + 1) % n];
        out.push(Point2::new(
            (prev.x + next.x) * half,
            (prev.y + next.y) * half,
        ));
    }
    out
}

/// The structural chart-identity gate (module docs): shared
/// `SurfaceKey` on one body, or the same [`crate::GeomSource`] across
/// bodies (N6: bit-identical descriptions ⇒ the identical chart).
/// Anything weaker escalates typed.
fn same_chart<T: Decide + Bounds>(
    body_a: &Body<T>,
    face_a: FaceKey,
    body_b: &Body<T>,
    face_b: FaceKey,
) -> Result<Surface<T>, ChartRegionError> {
    let key_a = body_a
        .get_face(face_a)
        .ok_or(ChartRegionError::Corrupt)?
        .surface;
    let key_b = body_b
        .get_face(face_b)
        .ok_or(ChartRegionError::Corrupt)?
        .surface;
    // Arena keys are meaningful only within one arena: the key rung
    // exists only for the one-body site.
    let same_body = core::ptr::eq(body_a, body_b);
    if same_body && key_a == key_b {
        return body_a
            .get_surface(key_a)
            .cloned()
            .ok_or(ChartRegionError::Corrupt);
    }
    match (body_a.surface_source(key_a), body_b.surface_source(key_b)) {
        // Full `GeomSource` equality, orientation included: N6's
        // theorem is about the WHOLE recipe identity — a same-base
        // reverted pair describes the mirrored chart and diverges.
        //
        // The theorem's conclusion is VERIFIED, not assumed (union
        // fix U1): `set_surface_source` is a pub door, so "same
        // source" is a claim any caller can attach — and PR-2's
        // import-side declaration channel is where a wrong attachment
        // first becomes plausible. Bit-identical descriptions are
        // re-checked through the module's own exact-bracket
        // comparator; a same-source pair whose descriptions differ by
        // one bit refuses typed instead of certifying overlap in an
        // arbitrarily chosen chart.
        (Some(sa), Some(sb)) if sa == sb => {
            let s_a = body_a.get_surface(key_a).ok_or(ChartRegionError::Corrupt)?;
            let s_b = body_b.get_surface(key_b).ok_or(ChartRegionError::Corrupt)?;
            if surface_bits_equal(s_a, s_b) {
                Ok(s_a.clone())
            } else {
                Err(ChartRegionError::ChartDivergence {
                    detail: "same GeomSource with non-bit-identical descriptions — \
                             the same-source theorem violated (forged or corrupted source attachment)",
                })
            }
        }
        (Some(sa), Some(sb)) if sa.same_base(sb) => Err(ChartRegionError::ChartDivergence {
            detail: "same source base with flipped orientation — the charts mirror",
        }),
        (Some(_), Some(_)) => Err(ChartRegionError::ChartDivergence {
            detail: "distinct GeomSources — equal-but-independent descriptions do not glue",
        }),
        _ => Err(ChartRegionError::ChartDivergence {
            detail: "no shared SurfaceKey and no GeomSource on both faces",
        }),
    }
}

/// Exact-bit equality of two scalars through their brackets: both
/// point brackets, equal, finite ([`bit_equal_cyclic`]'s read — the
/// C6 comparator, so NaN never equals and a non-point enclosure never
/// verifies; both are the conservative direction, a typed divergence).
fn exact_pair<T: Bounds>(a: T, b: T) -> bool {
    a.lo() == a.hi() && b.lo() == b.hi() && a.lo() == b.lo() && a.lo().is_finite()
}

/// Bit-identity of two surface DESCRIPTIONS, read structurally (union
/// fix U1; the rung-2 verification). Analytic kinds compare every
/// scalar field; `Nurbs` payloads verify only through pointer
/// identity today (one shared description object) — an independent
/// cross-body NURBS pair conservatively fails and takes the typed
/// divergence, which costs nothing the arm gate would not refuse
/// anyway; net-level verification extends with the census/inf-bounds
/// work. Different kinds are never identical.
fn surface_bits_equal<T: Decide + Bounds>(a: &Surface<T>, b: &Surface<T>) -> bool {
    let v3 = |p: geom_core::Vec3<T>, q: geom_core::Vec3<T>| {
        exact_pair(p.x, q.x) && exact_pair(p.y, q.y) && exact_pair(p.z, q.z)
    };
    let p3 = |p: geom_core::Point3<T>, q: geom_core::Point3<T>| {
        exact_pair(p.x, q.x) && exact_pair(p.y, q.y) && exact_pair(p.z, q.z)
    };
    match (a, b) {
        (
            Surface::Plane {
                origin: o1,
                normal: n1,
                u_ref: u1,
            },
            Surface::Plane {
                origin: o2,
                normal: n2,
                u_ref: u2,
            },
        ) => p3(*o1, *o2) && v3(*n1, *n2) && v3(*u1, *u2),
        (
            Surface::Cylinder {
                origin: o1,
                axis: a1,
                radius: r1,
                u_ref: u1,
            },
            Surface::Cylinder {
                origin: o2,
                axis: a2,
                radius: r2,
                u_ref: u2,
            },
        ) => p3(*o1, *o2) && v3(*a1, *a2) && exact_pair(*r1, *r2) && v3(*u1, *u2),
        (
            Surface::Cone {
                apex: p1,
                axis: a1,
                half_angle: h1,
                u_ref: u1,
            },
            Surface::Cone {
                apex: p2,
                axis: a2,
                half_angle: h2,
                u_ref: u2,
            },
        ) => p3(*p1, *p2) && v3(*a1, *a2) && exact_pair(*h1, *h2) && v3(*u1, *u2),
        (
            Surface::Sphere {
                center: c1,
                radius: r1,
                axis: a1,
                u_ref: u1,
            },
            Surface::Sphere {
                center: c2,
                radius: r2,
                axis: a2,
                u_ref: u2,
            },
        ) => p3(*c1, *c2) && exact_pair(*r1, *r2) && v3(*a1, *a2) && v3(*u1, *u2),
        (
            Surface::Torus {
                center: c1,
                axis: a1,
                major_radius: j1,
                minor_radius: m1,
                u_ref: u1,
            },
            Surface::Torus {
                center: c2,
                axis: a2,
                major_radius: j2,
                minor_radius: m2,
                u_ref: u2,
            },
        ) => {
            p3(*c1, *c2)
                && v3(*a1, *a2)
                && exact_pair(*j1, *j2)
                && exact_pair(*m1, *m2)
                && v3(*u1, *u2)
        }
        (Surface::Nurbs(x), Surface::Nurbs(y)) => std::sync::Arc::ptr_eq(x, y),
        // Shared-payload identity, exactly as the `Nurbs` arm: two
        // faces carrying the same `Arc` carry the same chart. Distinct
        // payloads answer `false` even when structurally equal —
        // conservative in the direction this predicate needs.
        (Surface::Approx(x), Surface::Approx(y)) => std::sync::Arc::ptr_eq(x, y),
        // Mismatched kinds are never the same chart.
        _ => false,
    }
}

/// A face's trim loops as chart-space polygons (unscaled chart
/// coordinates, cycle order, entry vertex per half-edge).
struct FaceUv<T: Decide> {
    outer: Vec<Point2<T>>,
    rings: Vec<Vec<Point2<T>>>,
}

/// Exact-`f64`-structure read of a `T` scalar being EXACTLY zero: a
/// point bracket at `0.0` (the C6 pattern, `props.rs` precedent).
/// This is structure read as structure — a nearly-zero value is
/// simply `false` here, and the caller refuses typed; no scalar
/// zero-test on `T` ever runs.
fn exact_zero<T: Bounds>(x: T) -> bool {
    x.lo() == x.hi() && x.lo() == 0.0
}

/// The entry vertex of one boundary half-edge's chart image, gated on
/// the pcurve VARIANT (module docs: the planar trim inventory).
/// `Ok(point)` only when the image is a straight chart segment whose
/// endpoint this is; `Err(what)` names the violated inventory clause.
fn pcurve_entry<T: Decide + Bounds>(
    pcurve: &geom_brep::Pcurve<T>,
    t0: T,
    t1: T,
    forward: bool,
) -> Result<Point2<T>, &'static str> {
    use geom_brep::Pcurve;
    match pcurve {
        // Straight by variant: `P(t) = p0 + pl·t`.
        Pcurve::IsoLine { p0, pl } => {
            let t = if forward { t0 } else { t1 };
            Ok(Point2::new(p0.x + pl.x * t, p0.y + pl.y * t))
        }
        // The UV image is the straight segment `p0 → p0 + pd` by
        // variant; only the parameterization along it is
        // transcendental, and endpoints are read structurally
        // (`g = 0` at the carrier's `t0`, `g = 1` at `t1`).
        Pcurve::IsoArc { p0, pd, .. } => Ok(if forward {
            *p0
        } else {
            Point2::new(p0.x + pd.x, p0.y + pd.y)
        }),
        // Straight iff BOTH trig channels are exact-structural zeros
        // (C6): rim circles (`pl = (β, 0)`), meridians, and every
        // line edge of a plane chart land here; a tilted-section
        // sinusoid (`pa`/`pb` alive) refuses — including a
        // numerically-almost-zero channel, deliberately.
        Pcurve::Harmonic { p0, pa, pb, pl } => {
            if exact_zero(pa.x) && exact_zero(pa.y) && exact_zero(pb.x) && exact_zero(pb.y) {
                let t = if forward { t0 } else { t1 };
                Ok(Point2::new(p0.x + pl.x * t, p0.y + pl.y * t))
            } else {
                Err("Harmonic trig channels are not exact-structural zeros \
                     (the sinusoid / tilted-cut class)")
            }
        }
        Pcurve::Fitted(_) => Err("Fitted chart image (rung-3 trace) is not a straight segment"),
    }
}

/// Walks one loop into its chart polygon: per half-edge, the stored
/// cache (minting charts, and only under [`ChartRead::Minted`]) or the
/// derive-on-demand affine image (plane charts), then the
/// variant-gated entry vertex.
fn loop_uv_polygon<T: Decide + Bounds>(
    body: &Body<T>,
    face: FaceKey,
    lk: LoopKey,
    surface: &Surface<T>,
    read: ChartRead,
    band: Band,
) -> Result<Vec<Point2<T>>, ChartRegionError> {
    let loop_data = body.get_loop(lk).ok_or(ChartRegionError::Corrupt)?;
    let LoopBoundary::Cycle { first } = loop_data.boundary else {
        return Err(ChartRegionError::Corrupt); // an empty loop bounds no region
    };
    let mut poly = Vec::new();
    for he in body.loop_cycle(first).ok_or(ChartRegionError::Corrupt)? {
        let he_data = body.get_half_edge(he).ok_or(ChartRegionError::Corrupt)?;
        let edge = body
            .get_edge(he_data.edge)
            .ok_or(ChartRegionError::Corrupt)?;
        let forward = edge.he_plus == he;
        let refuse = |what| ChartRegionError::NonPlanarTrim {
            face,
            half_edge: he,
            what,
        };
        let cache = (read == ChartRead::Minted)
            .then(|| body.pcurve(he))
            .flatten();
        let entry = if let Some(cache) = cache {
            let (t0, t1) = cache.params();
            pcurve_entry(cache.pcurve(), t0, t1, forward).map_err(refuse)?
        } else if matches!(surface, Surface::Plane { .. }) {
            // Derive-on-demand affine image (C4's standing plane
            // status). A plane chart has no branches, so the
            // no-loop-context caveat of `pcurve_of` is vacuous here.
            let Some(CurveGeom::Certified(curve)) = body.get_curve_geom(edge.curve) else {
                return Err(ChartRegionError::Corrupt);
            };
            let (t0, t1) = curve.params();
            let pcurve = geom_brep::chart_pcurve(curve.carrier(), surface, band)
                .map_err(|_| refuse("no closed-form chart image for this carrier kind"))?;
            pcurve_entry(&pcurve, t0, t1, forward).map_err(refuse)?
        } else {
            return Err(ChartRegionError::MissingCache { half_edge: he });
        };
        poly.push(entry);
    }
    if poly.len() < 3 {
        return Err(ChartRegionError::DegenerateLoop { face, r#loop: lk });
    }
    Ok(poly)
}

/// Extracts a face's outer + ring polygons (fixed order, D9).
fn extract_face_uv<T: Decide + Bounds>(
    body: &Body<T>,
    face: FaceKey,
    surface: &Surface<T>,
    read: ChartRead,
    band: Band,
) -> Result<FaceUv<T>, ChartRegionError> {
    let face_data = body.get_face(face).ok_or(ChartRegionError::Corrupt)?;
    let outer = loop_uv_polygon(body, face, face_data.outer, surface, read, band)?;
    let mut rings = Vec::with_capacity(face_data.rings.len());
    for &rk in &face_data.rings {
        rings.push(loop_uv_polygon(body, face, rk, surface, read, band)?);
    }
    Ok(FaceUv { outer, rings })
}

/// Exact constant chart lever arms (metres per chart unit), or the
/// typed refusal (module docs: the `chart_arms` sup-bounds are unsafe
/// for a positive claim; no inf bounds exist).
fn exact_arms<T: Decide>(surface: &Surface<T>) -> Result<(T, T), ChartRegionError> {
    match *surface {
        // A plane chart's coordinates ARE metres (unit u_ref frame).
        Surface::Plane { .. } => Ok((T::one(), T::one())),
        // Azimuth radians lever by exactly r everywhere; v is metres.
        Surface::Cylinder { radius, .. } => Ok((radius, T::one())),
        Surface::Cone { .. } => Err(ChartRegionError::ArmUnbounded { chart: "cone" }),
        Surface::Sphere { .. } => Err(ChartRegionError::ArmUnbounded { chart: "sphere" }),
        Surface::Torus { .. } => Err(ChartRegionError::ArmUnbounded { chart: "torus" }),
        Surface::Nurbs(_) => Err(ChartRegionError::ArmUnbounded { chart: "NURBS" }),
        // A fitted chart's stretch has no exact constant bound either —
        // the same refusal its fit would earn, for the same reason.
        Surface::Approx(_) => Err(ChartRegionError::ArmUnbounded {
            chart: "approximating surface",
        }),
    }
}

/// The seam-branch gate: on a periodic chart, EVERY extracted vertex
/// of both faces must fit one closed period-wide azimuth window
/// (span ≤ τ admits the exact full-wrap wall; a definite excess means
/// the loops sit on different pinned branches). The span excess is
/// radians, levered by the azimuth arm to metres.
fn seam_gate<T: Decide + Bounds>(
    surface: &Surface<T>,
    a: &FaceUv<T>,
    b: &FaceUv<T>,
    band: Band,
) -> Result<(), ChartRegionError> {
    let Surface::Cylinder { radius, .. } = *surface else {
        return Ok(()); // plane charts do not wrap; other kinds refused at the arm gate
    };
    let mut u_min: Option<T> = None;
    let mut u_max: Option<T> = None;
    let mut visit = |poly: &Vec<Point2<T>>| {
        for p in poly {
            u_min = Some(match u_min {
                None => p.x,
                Some(m) => m.min(p.x),
            });
            u_max = Some(match u_max {
                None => p.x,
                Some(m) => m.max(p.x),
            });
        }
    };
    visit(&a.outer);
    a.rings.iter().for_each(&mut visit);
    visit(&b.outer);
    b.rings.iter().for_each(&mut visit);
    let (Some(lo), Some(hi)) = (u_min, u_max) else {
        return Err(ChartRegionError::Corrupt);
    };
    let excess = (hi - lo) - T::tau();
    match decide(
        "chart_region_seam_span",
        Margin::levered(excess, radius),
        band,
    ) {
        // A definite excess: no common branch. Zero (the exact
        // full-wrap wall pair) and definite negatives proceed.
        Ok(Sign::Positive) => Err(ChartRegionError::SeamBranch),
        Ok(_) => Ok(()),
        Err(diag) => Err(ChartRegionError::Escalated(diag)),
    }
}

/// Shoelace + perimeter of a chart polygon (evaluation lane): `2A`
/// signed about the first vertex (the `split_section_area`
/// conditioning trick), `P` the segment-length sum.
fn loop_measures<T: Decide>(poly: &[Point2<T>]) -> (T, T) {
    let o = poly[0];
    let mut twice_area = T::zero();
    let mut perimeter = T::zero();
    for i in 0..poly.len() {
        let p = poly[i] - o;
        let q = poly[(i + 1) % poly.len()] - o;
        twice_area = twice_area + p.perp_dot(q);
        perimeter = perimeter + (q - p).norm();
    }
    (twice_area, perimeter)
}

/// A face's region in METRED chart coordinates, outer loop normalized
/// CCW; rings kept as conservative `(|2A|, P)` deductions (module
/// docs: subtraction can only under-state the intersection area and
/// over-state its perimeter — escalation-safe, never
/// false-certifying).
struct ScaledFace<T: Decide> {
    outer: Vec<Point2<T>>,
    /// The outer polygon's `2A` (positive: CCW after normalization).
    outer_2a: T,
    /// The outer polygon's perimeter.
    outer_p: T,
    /// Per ring: (`|2A|`, `P`).
    rings: Vec<(T, T)>,
}

impl<T: Decide + Bounds> ScaledFace<T> {
    fn build(
        body: &Body<T>,
        face: FaceKey,
        uv: &FaceUv<T>,
        arm_u: T,
        arm_v: T,
        band: Band,
    ) -> Result<Self, ChartRegionError> {
        let face_data = body.get_face(face).ok_or(ChartRegionError::Corrupt)?;
        let scale = |poly: &[Point2<T>]| -> Vec<Point2<T>> {
            poly.iter()
                .map(|p| Point2::new(p.x * arm_u, p.y * arm_v))
                .collect()
        };
        // Orientation trilean per loop. Margin: `over_lever(2A, P)` is
        // the loop's signed mean width in metres (m²/m) — the sign IS
        // the winding; a Zero loop bounds no decidable region.
        let orient = |mut poly: Vec<Point2<T>>,
                      lk: LoopKey|
         -> Result<(Vec<Point2<T>>, T, T), ChartRegionError> {
            let (twice_area, perimeter) = loop_measures(&poly);
            match decide(
                "chart_region_orientation",
                Margin::over_lever(twice_area, perimeter),
                band,
            ) {
                Ok(Sign::Positive) => Ok((poly, twice_area, perimeter)),
                Ok(Sign::Negative) => {
                    poly.reverse();
                    Ok((poly, T::zero() - twice_area, perimeter))
                }
                Ok(Sign::Zero) => Err(ChartRegionError::DegenerateLoop { face, r#loop: lk }),
                Err(diag) => Err(ChartRegionError::Escalated(diag)),
            }
        };
        let (outer, outer_2a, outer_p) = orient(scale(&uv.outer), face_data.outer)?;
        let mut rings = Vec::with_capacity(uv.rings.len());
        for (poly, &rk) in uv.rings.iter().zip(&face_data.rings) {
            let (_, ring_2a, ring_p) = orient(scale(poly), rk)?;
            rings.push((ring_2a, ring_p));
        }
        Ok(Self {
            outer,
            outer_2a,
            outer_p,
            rings,
        })
    }
}

/// Exact-bit cyclic equality of two CCW chart polygons (C6: structure
/// read as structure). `true` only when every coordinate is a point
/// bracket and the vertex cycles match under some rotation — the
/// identical-region fast path (bit-identical trims are the common
/// product of rung-2 pairs; anything weaker falls through to the
/// numeric machinery and may honestly escalate as a touching
/// boundary).
fn bit_equal_cyclic<T: Decide + Bounds>(a: &[Point2<T>], b: &[Point2<T>]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let exact = |p: &Point2<T>| -> Option<(f64, f64)> {
        let (xl, xh, yl, yh) = (p.x.lo(), p.x.hi(), p.y.lo(), p.y.hi());
        (xl == xh && yl == yh && xl.is_finite() && yl.is_finite()).then_some((xl, yl))
    };
    let Some(ea) = a.iter().map(exact).collect::<Option<Vec<_>>>() else {
        return false;
    };
    let Some(eb) = b.iter().map(exact).collect::<Option<Vec<_>>>() else {
        return false;
    };
    let n = ea.len();
    (0..n).any(|shift| (0..n).all(|i| ea[i] == eb[(i + shift) % n]))
}

/// The trilean 2-D point-in-polygon verdict — [`crate::ray_parity`]'s
/// walk in chart space, with its own direction schedule and its own
/// K rows.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PolyContainment {
    In,
    Out,
    OnBoundary,
}

/// The fixed 2-D ray schedule. Sixteen constant directions — axes
/// plus oblique spread members. Distinct from the 3-D consumer's
/// table by dimension, not by drift: there is no 2-D projection of
/// the space schedule that both stays exact and keeps the spread.
/// **The 3-D `point_in_loop_arm` row is derived away here,
/// deliberately**: that gate existed
/// because a 3-D schedule member projected into the loop's plane can
/// degenerate to a near-zero in-plane direction; a 2-D schedule
/// member IS in-plane by construction and its length is fixed nonzero
/// `f64` structure, so there is no quantity for an arm predicate to
/// decide — the re-derivation's honest conclusion is three rows, not
/// four.
const SCHEDULE_2D: [[f64; 2]; 16] = [
    [1.0, 0.0],
    [0.0, 1.0],
    [0.5, 1.0],
    [1.0, 0.5],
    [-0.5, 1.0],
    [1.0, -0.5],
    [0.25, 1.0],
    [1.0, 0.25],
    [0.75, -1.0],
    [1.0, 0.75],
    [-1.0, 0.375],
    [0.375, 1.0],
    [0.9375, 0.3125],
    [0.3125, -0.9375],
    [-0.75, 1.0],
    [1.0, -0.75],
];

/// This consumer's K rows for the shared walk ([`crate::ray_parity`]),
/// and the greppable roster entry for all four (see
/// [`crate::ray_parity::ParityRows`]).
/// Chart-space margins are metered separately from the 3-D loop's —
/// the polygon is metred by the exact arms, but it is a different
/// population — so the names stay distinct even though the walk is one.
const ROWS: ParityRows = ParityRows {
    segment: "chart_region_segment",
    boundary: "chart_region_boundary",
    side: "chart_region_side",
    advance: "chart_region_advance",
};

/// Ray-parity containment of `q` in the (CCW, metred) `poly`.
///
/// The walk is [`crate::ray_parity`]'s, shared with the 3-D
/// `point_in_loop`; what this function owns is the 2-D frame, which
/// needs no arm gate (see [`SCHEDULE_2D`]).
///
/// # Rows (margins re-derived for chart space; all metres because the
/// polygon is metred by the exact arms)
///
/// - `chart_region_segment`: a closed segment's own length — the
///   degeneracy gate. A *different question* from the row below, and
///   so a different name.
/// - `chart_region_boundary`: distance of `q` to a closed segment
///   (perpendicular at an interior foot, endpoint otherwise) — Zero ⇒
///   `OnBoundary`.
/// - `chart_region_side`: signed offset of a vertex from the ray line
///   (a metre coordinate along the ray's in-plane perpendicular) —
///   Zero ⇒ grazing ⇒ next ray.
/// - `chart_region_advance`: the crossing's advance along the ray,
///   `over_lever(x_i·y_j − x_j·y_i, y_j − y_i)` — a 2×2 determinant
///   (m²) over its straddle height (m); Zero would contradict the
///   boundary pre-pass ⇒ next ray.
fn point_in_polygon<T: Decide>(
    poly: &[Point2<T>],
    q: Point2<T>,
    band: Band,
) -> Result<PolyContainment, ChartRegionError> {
    let escalate = ChartRegionError::Escalated;

    if ray_parity::on_boundary(poly, q, &ROWS, band).map_err(escalate)? {
        return Ok(PolyContainment::OnBoundary);
    }

    // Ray parity with the fixed schedule.
    for r in &SCHEDULE_2D {
        let d = Vec2::new(T::from_f64(r[0]), T::from_f64(r[1])).normalize();
        let side_axis = Vec2::new(T::zero() - d.y, d.x); // in-plane ⟂, unit
        if let Some(inside) =
            ray_parity::ray_verdict(poly, q, d, side_axis, &ROWS, band).map_err(escalate)?
        {
            return Ok(if inside {
                PolyContainment::In
            } else {
                PolyContainment::Out
            });
        }
    }
    Err(ChartRegionError::RayExhausted)
}

/// A proper (transverse, segment-interior) boundary crossing between
/// edge `ai` of polygon A and edge `bi` of polygon B.
struct Crossing<T: Decide> {
    ai: usize,
    bi: usize,
    /// Advance fraction along edge `ai` (dimensionless; ordering key).
    ta: T,
    /// Advance fraction along edge `bi`.
    tb: T,
    /// A's boundary ENTERS B's interior here (both polygons CCW:
    /// entering ⟺ `perp_dot(s, r) > 0` ⟺ the transversality
    /// determinant `perp_dot(r, s)` is Negative — read off the
    /// already-decided `chart_region_parallel` sign, no new decision).
    entering: bool,
    point: Point2<T>,
}

/// All proper crossings between two CCW metred polygons, every
/// decision through named rows (fixed pair order, D9).
///
/// # Rows
///
/// - `chart_region_parallel`: the transversality determinant over one
///   edge's length, `over_lever(perp_dot(r, s), |r|)` — the
///   perpendicular height of `s` across `r`'s line (m). Zero ⇒ the
///   parallel lane below.
/// - `chart_region_cross_span`: each of the four advance clearances
///   `t·|r|`, `(1−t)·|r|`, `u·|s|`, `(1−u)·|s|` — the crossing
///   point's distance from a segment endpoint along its segment
///   (dimensionless fraction levered by the segment length). All four
///   definitely positive ⇒ a proper crossing; any definitely negative
///   ⇒ the lines cross off-segment (no crossing); an exact Zero is a
///   boundary-touch configuration ⇒ typed refusal.
/// - `chart_region_collinear_offset` (parallel lane): the signed
///   perpendicular offset of `b`'s start from `a`'s line,
///   `over_lever(perp_dot(r, q − p), |r|)` (m). Definite ⇒ parallel
///   disjoint lines, no crossing; Zero ⇒ collinear, span check next.
/// - `chart_region_collinear_overlap` (collinear lane): the shared
///   span length along the common line (m, an interval-overlap
///   difference of metre projections). Positive ⇒ the boundaries
///   share a segment ⇒ typed touching refusal; Zero/Negative ⇒
///   disjoint spans, no crossing.
///
/// The division by the determinant in `t`/`u` is certified: the
/// parallel row's definite sign excludes zero from the enclosure.
fn proper_crossings<T: Decide>(
    a: &[Point2<T>],
    b: &[Point2<T>],
    band: Band,
) -> Result<Vec<Crossing<T>>, ChartRegionError> {
    let escalate = ChartRegionError::Escalated;
    let mut out = Vec::new();
    for ai in 0..a.len() {
        let p = a[ai];
        let r = a[(ai + 1) % a.len()] - p;
        for bi in 0..b.len() {
            let q = b[bi];
            let s = b[(bi + 1) % b.len()] - q;
            let qp = q - p;
            let denom = r.perp_dot(s);
            let d_sign = decide(
                "chart_region_parallel",
                Margin::over_lever(denom, r.norm()),
                band,
            )
            .map_err(escalate)?;
            if d_sign == Sign::Zero {
                // Parallel lane.
                match decide(
                    "chart_region_collinear_offset",
                    Margin::over_lever(r.perp_dot(qp), r.norm()),
                    band,
                )
                .map_err(escalate)?
                {
                    Sign::Zero => {
                        // Collinear: do the spans share length?
                        let rhat = r.normalize();
                        let s0 = qp.dot(rhat);
                        let s1 = (b[(bi + 1) % b.len()] - p).dot(rhat);
                        let (lo, hi) = (s0.min(s1), s0.max(s1));
                        let overlap = hi.min(r.norm()) - lo.max(T::zero());
                        match decide("chart_region_collinear_overlap", Margin::of(overlap), band)
                            .map_err(escalate)?
                        {
                            Sign::Positive => return Err(ChartRegionError::TouchingBoundary),
                            _ => continue,
                        }
                    }
                    _ => continue, // parallel, definitely offset lines
                }
            }
            // Transverse lane: intersection fractions.
            let t = qp.perp_dot(s) / denom;
            let u = qp.perp_dot(r) / denom;
            let one = T::one();
            let spans = [
                (t, r.norm()),
                (one - t, r.norm()),
                (u, s.norm()),
                (one - u, s.norm()),
            ];
            // All four clearances are decided BEFORE any verdict: a
            // definite Negative anywhere means the line intersection
            // lies definitely off a segment — no crossing and no
            // touch, whatever the other spans read (a Zero there is
            // the OTHER segment's endpoint sitting on this one's
            // LINE, far away — the shared-height rectangle corner
            // configuration). Only with no Negative do the weaker
            // outcomes speak: an in-band span escalates (a genuine
            // near-endpoint crossing), an exact Zero is a true
            // boundary touch.
            let mut outcomes = [Sign::Positive; 4];
            let mut indeterminate = None;
            for (slot, (frac, len)) in outcomes.iter_mut().zip(spans) {
                match decide("chart_region_cross_span", Margin::levered(frac, len), band) {
                    Ok(sign) => *slot = sign,
                    Err(diag) => indeterminate = indeterminate.or(Some(diag)),
                }
            }
            if outcomes.contains(&Sign::Negative) {
                continue;
            }
            if let Some(diag) = indeterminate {
                return Err(ChartRegionError::Escalated(diag));
            }
            if outcomes.contains(&Sign::Zero) {
                return Err(ChartRegionError::TouchingBoundary);
            }
            {
                out.push(Crossing {
                    ai,
                    bi,
                    ta: t,
                    tb: u,
                    entering: d_sign == Sign::Negative,
                    point: p + r * t,
                });
            }
        }
    }
    Ok(out)
}

/// The intersection pieces of two CCW polygons with proper crossings:
/// the standard clip walk — follow A while inside B, switch at every
/// crossing, follow B while inside A — with every switch point a
/// definite crossing and every inconsistency a fail-loud `Corrupt`.
///
/// Ordering along each boundary is (edge index, advance fraction),
/// D9-fixed — and CERTIFIED, not assumed (union fix U2): edge indices
/// order exactly; within one edge, each adjacent pair of sorted
/// crossings passes the `chart_region_cross_order` row — their
/// advance-fraction difference levered by the edge's own length, the
/// metre separation of the two crossing points along the boundary.
/// A definite Positive certifies the walk order; an in-band or
/// non-positive separation escalates typed (two crossings the sort
/// cannot certifiably order — the `chart_region_cross_span` rows
/// meter clearance from segment ENDPOINTS and say nothing about
/// crossing-to-crossing separation, so this row exists). Behind the
/// certificate, the walk's enter/exit alternation, parity and
/// consumption checks remain the fail-loud backstop: an adjacent
/// mis-sort breaks alternation and lands in `Corrupt`.
fn intersection_pieces<T: Decide + Bounds>(
    a: &[Point2<T>],
    b: &[Point2<T>],
    crossings: &[Crossing<T>],
    band: Band,
) -> Result<Vec<Vec<Point2<T>>>, ChartRegionError> {
    let m = crossings.len();
    if m == 0 || !m.is_multiple_of(2) {
        return Err(ChartRegionError::Corrupt); // parity: closed curves cross evenly
    }
    let sort_key = |edge: usize, frac: T| -> Result<(usize, f64), ChartRegionError> {
        let k = frac.lo();
        if k.is_finite() {
            Ok((edge, k))
        } else {
            Err(ChartRegionError::Corrupt)
        }
    };
    let order_on = |on_a: bool| -> Result<Vec<usize>, ChartRegionError> {
        let mut idx: Vec<usize> = (0..m).collect();
        let mut keys = Vec::with_capacity(m);
        for c in crossings {
            keys.push(if on_a {
                sort_key(c.ai, c.ta)?
            } else {
                sort_key(c.bi, c.tb)?
            });
        }
        idx.sort_by(|&i, &j| {
            keys[i]
                .partial_cmp(&keys[j])
                .unwrap_or(core::cmp::Ordering::Equal)
        });
        // Certify the sorted order (doc comment): same-edge neighbours
        // must be definitely separated along the boundary. The margin
        // is the advance-fraction difference (dimensionless) levered
        // by the edge's own length — the crossing points' metre
        // separation along that edge. An exact tie decides Zero and a
        // bracket-overlapping pair lands in-band: both escalate typed
        // (no silent assumption, no Corrupt masquerade — the geometry
        // is degenerate/unresolvable at this ε, not corrupt).
        let poly: &[Point2<T>] = if on_a { a } else { b };
        for w in idx.windows(2) {
            let (ei, fi) = if on_a {
                (crossings[w[0]].ai, crossings[w[0]].ta)
            } else {
                (crossings[w[0]].bi, crossings[w[0]].tb)
            };
            let (ej, fj) = if on_a {
                (crossings[w[1]].ai, crossings[w[1]].ta)
            } else {
                (crossings[w[1]].bi, crossings[w[1]].tb)
            };
            if ei != ej {
                continue; // distinct edges order exactly by index
            }
            let edge_len = (poly[(ei + 1) % poly.len()] - poly[ei]).norm();
            let sep = Margin::levered(fj - fi, edge_len);
            match decide("chart_region_cross_order", sep, band) {
                Ok(Sign::Positive) => {}
                Ok(_) => {
                    return Err(ChartRegionError::Escalated(definite_diag(
                        band,
                        "chart_region_cross_order",
                        sep,
                    )));
                }
                Err(diag) => return Err(ChartRegionError::Escalated(diag)),
            }
        }
        Ok(idx)
    };
    let a_order = order_on(true)?;
    let b_order = order_on(false)?;
    // Position of each crossing within each order.
    let mut pos_a = vec![0usize; m];
    let mut pos_b = vec![0usize; m];
    for (p, &c) in a_order.iter().enumerate() {
        pos_a[c] = p;
    }
    for (p, &c) in b_order.iter().enumerate() {
        pos_b[c] = p;
    }

    // Walk helper: push the polygon vertices strictly between crossing
    // `cur` and its boundary successor `nxt` (cyclic; a wrap or an
    // edge change walks vertex starts up to and including `nxt`'s
    // edge's start vertex).
    let push_between = |piece: &mut Vec<Point2<T>>,
                        poly: &[Point2<T>],
                        cur_edge: usize,
                        nxt_edge: usize,
                        same_edge_forward: bool| {
        if same_edge_forward {
            return;
        }
        let n = poly.len();
        let mut e = cur_edge;
        loop {
            e = (e + 1) % n;
            piece.push(poly[e]);
            if e == nxt_edge {
                break;
            }
        }
    };

    let mut used = vec![false; m];
    let mut pieces = Vec::new();
    let budget = 4 * (m + a.len() + b.len());
    let mut steps = 0usize;
    for &seed in &a_order {
        if used[seed] || !crossings[seed].entering {
            continue;
        }
        let mut piece: Vec<Point2<T>> = Vec::new();
        let mut cur = seed;
        loop {
            steps += 1;
            if steps > budget {
                return Err(ChartRegionError::Corrupt);
            }
            // At an ENTERING crossing: follow A to the next crossing.
            if !crossings[cur].entering || used[cur] {
                return Err(ChartRegionError::Corrupt);
            }
            used[cur] = true;
            piece.push(crossings[cur].point);
            let nxt = a_order[(pos_a[cur] + 1) % m];
            let wrap = pos_a[cur] + 1 == m;
            push_between(
                &mut piece,
                a,
                crossings[cur].ai,
                crossings[nxt].ai,
                !wrap && crossings[nxt].ai == crossings[cur].ai,
            );
            // The successor must be an EXIT; follow B from it.
            if crossings[nxt].entering || used[nxt] {
                return Err(ChartRegionError::Corrupt);
            }
            used[nxt] = true;
            piece.push(crossings[nxt].point);
            let nxt2 = b_order[(pos_b[nxt] + 1) % m];
            let wrap_b = pos_b[nxt] + 1 == m;
            push_between(
                &mut piece,
                b,
                crossings[nxt].bi,
                crossings[nxt2].bi,
                !wrap_b && crossings[nxt2].bi == crossings[nxt].bi,
            );
            if nxt2 == seed {
                break; // the piece closed
            }
            cur = nxt2;
        }
        if piece.len() < 3 {
            return Err(ChartRegionError::Corrupt);
        }
        pieces.push(piece);
    }
    if used.iter().any(|u| !u) || pieces.is_empty() {
        return Err(ChartRegionError::Corrupt); // every crossing belongs to a piece
    }
    Ok(pieces)
}

/// The overlap verdict on two scaled faces (module docs). `same_face`
/// marks the degenerate self-query (one face against itself), whose
/// rings must not be double-subtracted.
fn overlap_of_regions<T: Decide + Bounds>(
    a: &ScaledFace<T>,
    b: &ScaledFace<T>,
    same_face: bool,
    band: Band,
) -> Result<ChartOverlap, ChartRegionError> {
    // Identical-region fast path (C6 structure): bit-identical outer
    // cycles — the common product of the structural rungs — need no
    // numeric boundary comparison (which would honestly escalate as a
    // touching boundary).
    let (pieces_2a, pieces_p) = if bit_equal_cyclic(&a.outer, &b.outer) {
        (a.outer_2a, a.outer_p)
    } else {
        let crossings = proper_crossings(&a.outer, &b.outer, band)?;
        if crossings.is_empty() {
            // No boundary crossings: the relation is uniform — decide
            // it by vertex probes (cycle order, skipping on-boundary
            // vertices; a polygon with NO definite vertex has a
            // boundary coincident with the other's ⇒ touching).
            match polygon_relation(&a.outer, &b.outer, band)? {
                Some(PolyContainment::In) => (a.outer_2a, a.outer_p), // A ⊆ B
                // Defense-in-depth (union fix U3), stated as the
                // invariant it is: every A vertex ON B's boundary with
                // ZERO proper crossings cannot occur without a
                // `chart_region_cross_span` Zero having refused first
                // (three adversarial witness constructions all hit
                // TouchingBoundary at the crossing rows). If the arm
                // is ever reached, the pair is a touching
                // configuration — never Empty.
                None => return Err(ChartRegionError::TouchingBoundary),
                Some(_) => match polygon_relation(&b.outer, &a.outer, band)? {
                    Some(PolyContainment::In) => (b.outer_2a, b.outer_p), // B ⊆ A
                    Some(_) => return Ok(ChartOverlap::Empty),            // definitely disjoint
                    None => return Err(ChartRegionError::TouchingBoundary),
                },
            }
        } else {
            let pieces = intersection_pieces(&a.outer, &b.outer, &crossings, band)?;
            let mut sum_2a = T::zero();
            let mut sum_p = T::zero();
            for piece in &pieces {
                let (p2a, pp) = loop_measures(piece);
                sum_2a = sum_2a + p2a;
                sum_p = sum_p + pp;
            }
            (sum_2a, sum_p)
        }
    };

    // Ring deductions, conservative (module docs): true area ≥
    // pieces − Σ ring areas, true perimeter ≤ pieces + Σ ring
    // perimeters, so the margin below under-states the true mean
    // width — the safe direction for a positive claim; a hole big
    // enough to threaten the overlap drives the margin out of the
    // definite-positive range and the query escalates rather than
    // certifies.
    //
    // Stated as the deviation it is (PR deviation 5): on a
    // ring-bearing face this GLOBAL deduction narrows the letter of
    // C3's "exact in chart space" — the holes are never clipped
    // against the intersection, so the certified quantity is a LOWER
    // bound on the true mean width, exact only for ring-free faces.
    // The narrowing is one-directional by construction: 2A_true ≥
    // 2A_pieces − Σ 2A_ring and P_true ≤ P_pieces + Σ P_ring, so the
    // margin under-states — it can refuse a true overlap (escalate),
    // it can never certify a false one or answer a false Empty (Empty
    // is decided from outers alone, and holes only shrink).
    let mut net_2a = pieces_2a;
    let mut tot_p = pieces_p;
    let ring_sets: &[&Vec<(T, T)>] = if same_face {
        &[&a.rings]
    } else {
        &[&a.rings, &b.rings]
    };
    for rings in ring_sets {
        for &(r2a, rp) in rings.iter() {
            net_2a = net_2a - r2a;
            tot_p = tot_p + rp;
        }
    }

    // The area margin: `over_lever(2A, P)` — the intersection
    // region's MEAN WIDTH in metres. The dimensional argument has one
    // home, `Margin::over_lever`'s door: `2A/P` is the width of the
    // constant-width strip with this area and boundary length.
    // `net_2a` IS the shoelace sum, not half of it, so the divisor is
    // the FULL perimeter.
    //
    // `split_section_area` asks the same question in 3-D through the
    // same door, and that is where the sharing ends. The accumulator
    // could be shared ONE way — embedding these `Point2`s as
    // `(x, y, 0)` with `n̂ = ẑ` makes `a×b·n̂` reduce to `perp_dot`
    // bit for bit — but not the other, which would need the in-plane
    // basis the 3-D form exists to avoid. Past the accumulator
    // nothing lines up: conic excess there against the conservative
    // ring deduction here, `|2A|` there against a signed `net_2a`
    // here (a ring deduction may legitimately drive this one
    // negative), and one K row each.
    //
    // Positive certifies; an exact Zero or a definite Negative after
    // the conservative ring deduction cannot certify EITHER direction
    // (the region exists; only its hole-adjusted area is unresolved)
    // and escalates typed.
    let area_margin = Margin::over_lever(net_2a, tot_p);
    match decide("chart_region_area", area_margin, band) {
        Ok(Sign::Positive) => Ok(ChartOverlap::PositiveArea),
        // A definite Zero/Negative here is NOT an invalid question
        // (union fix U5: `MarginDiag::Invalid` means never-posed —
        // NaN/poison — which this is not): the margin was posed and
        // answered; the conservative ring deduction just leaves no
        // certifiable direction. Echo the classified margin itself.
        Ok(_) => Err(ChartRegionError::Escalated(definite_diag(
            band,
            "chart_region_area",
            area_margin,
        ))),
        Err(diag) => Err(ChartRegionError::Escalated(diag)),
    }
}

/// The uniform relation of polygon `of`'s region to polygon `to`'s,
/// valid only when their boundaries have no proper crossing: the
/// first definite vertex verdict in cycle order (D9), `None` when
/// every vertex sits on `to`'s boundary.
fn polygon_relation<T: Decide>(
    of: &[Point2<T>],
    to: &[Point2<T>],
    band: Band,
) -> Result<Option<PolyContainment>, ChartRegionError> {
    for v in of {
        match point_in_polygon(to, *v, band)? {
            PolyContainment::OnBoundary => continue,
            verdict => return Ok(Some(verdict)),
        }
    }
    Ok(None)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#[path = "chart_region_r2_probes.rs"]
mod r2_probes;

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use geom_core::Tol;
    use geom_core::{Point3, Vec3};

    fn band() -> Band {
        Band::new(1e-9, 1e-8).unwrap()
    }

    fn pt(x: f64, y: f64) -> Point2<f64> {
        Point2::new(x, y)
    }

    /// Exact coordinate equality (Point2 carries no PartialEq).
    fn assert_pt(p: Point2<f64>, x: f64, y: f64) {
        assert!(
            p.x == x && p.y == y,
            "expected ({x}, {y}), got ({}, {})",
            p.x,
            p.y
        );
    }

    /// CCW axis-aligned rectangle polygon.
    fn rect(x0: f64, y0: f64, x1: f64, y1: f64) -> Vec<Point2<f64>> {
        vec![pt(x0, y0), pt(x1, y0), pt(x1, y1), pt(x0, y1)]
    }

    /// A `ScaledFace` from raw polygons (already metred), rings by
    /// their measures.
    fn face_of(outer: Vec<Point2<f64>>, rings: &[Vec<Point2<f64>>]) -> ScaledFace<f64> {
        let (a2, p) = loop_measures(&outer);
        assert!(a2 > 0.0, "test polygons are CCW");
        ScaledFace {
            outer,
            outer_2a: a2,
            outer_p: p,
            rings: rings
                .iter()
                .map(|r| {
                    let (a2, p) = loop_measures(r);
                    (a2.abs(), p)
                })
                .collect(),
        }
    }

    // ------------------------------------------------------------------
    // The planar-inventory gate (item 1): structure read as structure.
    // ------------------------------------------------------------------

    #[test]
    fn iso_line_and_iso_arc_entry_points_are_exact() {
        let line = geom_brep::Pcurve::IsoLine {
            p0: pt(0.25, 0.0),
            pl: Vec2::new(0.5, 1.0),
        };
        assert_pt(pcurve_entry(&line, 0.0, 2.0, true).unwrap(), 0.25, 0.0);
        assert_pt(pcurve_entry(&line, 0.0, 2.0, false).unwrap(), 1.25, 2.0);

        let arc = geom_brep::Pcurve::IsoArc {
            p0: pt(0.0, 1.0),
            pd: Vec2::new(1.0, 0.0),
            t0: 0.0,
            angle: core::f64::consts::FRAC_PI_2,
            breaks: geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
        };
        // The UV image is the straight segment p0 → p0 + pd by
        // variant; endpoints are structural, no Bézier evaluation.
        assert_pt(pcurve_entry(&arc, 0.0, 1.0, true).unwrap(), 0.0, 1.0);
        assert_pt(pcurve_entry(&arc, 0.0, 1.0, false).unwrap(), 1.0, 1.0);
    }

    #[test]
    fn zero_trig_harmonic_passes_and_sinusoid_refuses() {
        let linear = geom_brep::Pcurve::Harmonic {
            p0: pt(0.0, 0.5),
            pa: Vec2::zero(),
            pb: Vec2::zero(),
            pl: Vec2::new(1.0, 0.0),
        };
        assert_pt(pcurve_entry(&linear, 0.0, 3.0, true).unwrap(), 0.0, 0.5);

        // The tilted-cut class: an alive sin channel refuses typed.
        let sinusoid = geom_brep::Pcurve::Harmonic {
            p0: pt(0.0, 0.5),
            pa: Vec2::zero(),
            pb: Vec2::new(0.0, 0.3),
            pl: Vec2::new(1.0, 0.0),
        };
        assert!(pcurve_entry(&sinusoid, 0.0, 3.0, true).is_err());
    }

    #[test]
    fn a_numerically_almost_zero_trig_channel_refuses_typed() {
        // The C6 statement with teeth: 1e-300 is NOT a structural
        // zero, and no scalar zero-test on T may decide otherwise.
        let nearly = geom_brep::Pcurve::Harmonic {
            p0: pt(0.0, 0.5),
            pa: Vec2::new(0.0, 1e-300),
            pb: Vec2::zero(),
            pl: Vec2::new(1.0, 0.0),
        };
        assert!(pcurve_entry(&nearly, 0.0, 3.0, true).is_err());
    }

    // ------------------------------------------------------------------
    // 2-D containment (item 3): the ported ray-parity trilean.
    // ------------------------------------------------------------------

    #[test]
    fn point_in_polygon_square_verdicts() {
        let sq = rect(0.0, 0.0, 1.0, 1.0);
        assert_eq!(
            point_in_polygon(&sq, pt(0.5, 0.5), band()).unwrap(),
            PolyContainment::In
        );
        assert_eq!(
            point_in_polygon(&sq, pt(1.5, 0.5), band()).unwrap(),
            PolyContainment::Out
        );
        assert_eq!(
            point_in_polygon(&sq, pt(1.0, 0.5), band()).unwrap(),
            PolyContainment::OnBoundary
        );
        assert_eq!(
            point_in_polygon(&sq, pt(1.0, 1.0), band()).unwrap(),
            PolyContainment::OnBoundary
        );
    }

    #[test]
    fn point_in_polygon_nonconvex() {
        // A CCW "U": the notch interior is OUT despite the bounding
        // box saying otherwise.
        let u = vec![
            pt(0.0, 0.0),
            pt(3.0, 0.0),
            pt(3.0, 2.0),
            pt(2.0, 2.0),
            pt(2.0, 0.5),
            pt(1.0, 0.5),
            pt(1.0, 2.0),
            pt(0.0, 2.0),
        ];
        assert_eq!(
            point_in_polygon(&u, pt(1.5, 1.0), band()).unwrap(),
            PolyContainment::Out
        );
        assert_eq!(
            point_in_polygon(&u, pt(0.5, 1.0), band()).unwrap(),
            PolyContainment::In
        );
    }

    // ------------------------------------------------------------------
    // Crossing + clipping (item 3) and the area margin (item 4).
    // ------------------------------------------------------------------

    #[test]
    fn overlapping_squares_cross_and_clip_to_the_exact_intersection() {
        let a = rect(0.0, 0.0, 1.0, 1.0);
        let b = rect(0.5, 0.5, 1.5, 1.5);
        let crossings = proper_crossings(&a, &b, band()).unwrap();
        assert_eq!(crossings.len(), 2);
        let pieces = intersection_pieces(&a, &b, &crossings, band()).unwrap();
        assert_eq!(pieces.len(), 1);
        let (a2, p) = loop_measures(&pieces[0]);
        assert!((a2 - 0.5).abs() < 1e-12, "2A of the 0.5×0.5 overlap");
        assert!((p - 2.0).abs() < 1e-12, "perimeter of the overlap");
    }

    #[test]
    fn a_bar_through_a_u_clips_to_two_pieces() {
        // The U from above ∩ a horizontal bar across both prongs.
        let u = vec![
            pt(0.0, 0.0),
            pt(3.0, 0.0),
            pt(3.0, 2.0),
            pt(2.0, 2.0),
            pt(2.0, 0.5),
            pt(1.0, 0.5),
            pt(1.0, 2.0),
            pt(0.0, 2.0),
        ];
        let bar = rect(-0.5, 1.0, 3.5, 1.5);
        let crossings = proper_crossings(&u, &bar, band()).unwrap();
        // Each of the four vertical U edges (x = 0, 1, 2, 3) crosses
        // both bar edges (y = 1 and y = 1.5).
        assert_eq!(crossings.len(), 8);
        let pieces = intersection_pieces(&u, &bar, &crossings, band()).unwrap();
        assert_eq!(pieces.len(), 2, "one piece per prong");
        let total: f64 = pieces.iter().map(|p| loop_measures(p).0).sum();
        assert!((total - 2.0 * 1.0).abs() < 1e-12, "2A = 2 × (1 × 0.5)");
    }

    #[test]
    fn overlap_positive_disjoint_empty() {
        let a = face_of(rect(0.0, 0.0, 2.0, 2.0), &[]);
        let b = face_of(rect(1.0, 1.0, 3.0, 3.0), &[]);
        assert_eq!(
            overlap_of_regions(&a, &b, false, band()).unwrap(),
            ChartOverlap::PositiveArea
        );
        let c = face_of(rect(5.0, 5.0, 6.0, 6.0), &[]);
        assert_eq!(
            overlap_of_regions(&a, &c, false, band()).unwrap(),
            ChartOverlap::Empty
        );
        // Containment without crossings, both directions.
        let inner = face_of(rect(0.5, 0.5, 1.5, 1.5), &[]);
        assert_eq!(
            overlap_of_regions(&a, &inner, false, band()).unwrap(),
            ChartOverlap::PositiveArea
        );
        assert_eq!(
            overlap_of_regions(&inner, &a, false, band()).unwrap(),
            ChartOverlap::PositiveArea
        );
    }

    #[test]
    fn an_in_band_sliver_escalates_never_silent() {
        // Overlap strip width 3e-9: inside the (1e-9, 1e-8) band —
        // the three-outcome honesty acceptance row.
        let a = face_of(rect(0.0, 0.0, 1.0, 1.0), &[]);
        let b = face_of(rect(1.0 - 3e-9, 0.0 - 0.3, 2.0, 1.3), &[]);
        match overlap_of_regions(&a, &b, false, band()) {
            Err(ChartRegionError::Escalated(_)) => {}
            other => panic!("a sliver must escalate, got {other:?}"),
        }
    }

    #[test]
    fn touching_boundaries_refuse_typed() {
        // Shared full edge (collinear overlap).
        let a = face_of(rect(0.0, 0.0, 1.0, 1.0), &[]);
        let b = face_of(rect(1.0, 0.0, 2.0, 1.0), &[]);
        match overlap_of_regions(&a, &b, false, band()) {
            Err(ChartRegionError::TouchingBoundary) => {}
            other => panic!("edge-sharing pair must refuse typed, got {other:?}"),
        }
        // Corner touch (a crossing at a segment endpoint).
        let c = face_of(rect(1.0, 1.0, 2.0, 2.0), &[]);
        match overlap_of_regions(&a, &c, false, band()) {
            Err(ChartRegionError::TouchingBoundary) => {}
            other => panic!("corner-touching pair must refuse typed, got {other:?}"),
        }
    }

    #[test]
    fn identical_regions_certify_through_the_structural_fast_path() {
        // Bit-identical cycles under rotation: the rung-2 product.
        let a = face_of(rect(0.0, 0.0, 1.0, 1.0), &[]);
        let rotated = vec![pt(1.0, 0.0), pt(1.0, 1.0), pt(0.0, 1.0), pt(0.0, 0.0)];
        let b = face_of(rotated, &[]);
        assert_eq!(
            overlap_of_regions(&a, &b, false, band()).unwrap(),
            ChartOverlap::PositiveArea
        );
    }

    #[test]
    fn ring_deductions_are_conservative_not_false() {
        // A small hole leaves the positive claim standing…
        let holed = face_of(
            rect(0.0, 0.0, 3.0, 3.0),
            &[vec![pt(1.2, 1.2), pt(1.2, 1.4), pt(1.4, 1.4), pt(1.4, 1.2)]],
        );
        let probe = face_of(rect(0.5, 0.5, 2.5, 2.5), &[]);
        assert_eq!(
            overlap_of_regions(&holed, &probe, false, band()).unwrap(),
            ChartOverlap::PositiveArea
        );
        // …and a hole that could swallow the overlap ESCALATES rather
        // than certifying either direction (the conservative
        // subtraction can only refuse, never bless falsely).
        let big_hole = face_of(
            rect(0.0, 0.0, 3.0, 3.0),
            &[vec![pt(0.5, 0.5), pt(0.5, 2.5), pt(2.5, 2.5), pt(2.5, 0.5)]],
        );
        let inner = face_of(rect(1.0, 1.0, 2.0, 2.0), &[]);
        match overlap_of_regions(&big_hole, &inner, false, band()) {
            Err(ChartRegionError::Escalated(_)) => {}
            other => panic!("hole-threatened overlap must escalate, got {other:?}"),
        }
    }

    // ------------------------------------------------------------------
    // The seam-branch gate (item 5).
    // ------------------------------------------------------------------

    fn cyl_surface(radius: f64) -> Surface<f64> {
        Surface::Cylinder {
            origin: Point3::origin(),
            axis: Vec3::unit_z(),
            radius,
            u_ref: Vec3::unit_x(),
        }
    }

    fn uv_of(outer: Vec<Point2<f64>>) -> FaceUv<f64> {
        FaceUv {
            outer,
            rings: Vec::new(),
        }
    }

    #[test]
    fn seam_branch_divergence_refuses_and_one_branch_passes() {
        let tau = core::f64::consts::TAU;
        let s = cyl_surface(1.0);
        // Loops a full period apart: different pinned branches.
        let a = uv_of(rect(0.0, 0.0, 1.0, 1.0));
        let b = uv_of(rect(tau + 0.1, 0.0, tau + 1.0, 1.0));
        match seam_gate(&s, &a, &b, band()) {
            Err(ChartRegionError::SeamBranch) => {}
            other => panic!("branch-divergent pair must refuse, got {other:?}"),
        }
        // Same branch: passes.
        let c = uv_of(rect(2.0, 0.0, 3.0, 1.0));
        seam_gate(&s, &a, &c, band()).unwrap();
        // The exact full-wrap wall (span exactly τ) is ONE closed
        // branch and passes on the Zero outcome.
        let full = uv_of(rect(0.0, 0.0, tau, 1.0));
        let inner = uv_of(rect(1.0, 0.2, 2.0, 0.8));
        seam_gate(&s, &full, &inner, band()).unwrap();
    }

    // ------------------------------------------------------------------
    // The red-then-green mutant (acceptance): a gate that let a
    // sinusoid Harmonic through would read its CHORD polygon and
    // certify a wrong verdict; the shipped gate refuses typed.
    // ------------------------------------------------------------------

    #[test]
    fn the_sinusoid_mutant_is_red_and_the_gate_is_green() {
        // The tilted-cut wall region on a cylinder chart (r = 1):
        // u ∈ [0, π], v from 0 up to 1 + 0.5·sin u. A mutant that
        // ignores the trig channels reads the top edge as the chord
        // v = 1 — i.e. the rectangle [0, π] × [0, 1].
        let mutant_a = face_of(rect(0.0, 0.0, core::f64::consts::PI, 1.0), &[]);
        // A probe region sitting wholly inside the sinusoid bulge,
        // wholly above the chord.
        let probe = face_of(rect(1.4, 1.2, 1.7, 1.4), &[]);
        // RED: the mutant answers definitely-disjoint…
        assert_eq!(
            overlap_of_regions(&mutant_a, &probe, false, band()).unwrap(),
            ChartOverlap::Empty
        );
        // …while an INSCRIBED polygonal subregion of the true region
        // (sin is concave on [0, π], so its chords lie below the
        // curve) already overlaps the probe with definite area — the
        // mutant's verdict is wrong, not merely weak.
        let top = |u: f64| 1.0 + 0.5 * u.sin();
        let inscribed = face_of(
            vec![
                pt(0.0, 0.0),
                pt(core::f64::consts::PI, 0.0),
                pt(core::f64::consts::PI, 1.0),
                pt(2.4, top(2.4)),
                pt(1.8, top(1.8)),
                pt(1.2, top(1.2)),
                pt(0.6, top(0.6)),
                pt(0.0, 1.0),
            ],
            &[],
        );
        assert_eq!(
            overlap_of_regions(&inscribed, &probe, false, band()).unwrap(),
            ChartOverlap::PositiveArea
        );
        // GREEN: the shipped inventory gate never lets the sinusoid
        // reach the machinery — the tilted-cut image refuses typed at
        // the variant gate (`pcurve_entry`), which is exactly the
        // exclusion the module docs name.
        let sinusoid = geom_brep::Pcurve::Harmonic {
            p0: pt(0.0, 1.0),
            pa: Vec2::zero(),
            pb: Vec2::new(0.0, 0.5),
            pl: Vec2::new(1.0, 0.0),
        };
        assert!(pcurve_entry(&sinusoid, 0.0, core::f64::consts::PI, true).is_err());
    }

    // ------------------------------------------------------------------
    // Body-level rows: the euler-built planar sheets (derive-on-demand
    // affine images) and cylinder walls (stored minted caches).
    // ------------------------------------------------------------------

    use crate::euler::{FaceSurface, MefSite, MevSite};
    use crate::source::GeomSource;
    use geom::Curve3;
    use geom_brep::{EdgeCurveSpec, EdgeGeometry};

    /// The shared test plane: chart u = x, v = y (u_ref = x̂, normal =
    /// ẑ ⇒ v_ref = ẑ × x̂ = ŷ).
    fn xy_plane() -> Surface<f64> {
        Surface::Plane {
            origin: Point3::origin(),
            normal: Vec3::unit_z(),
            u_ref: Vec3::unit_x(),
        }
    }

    /// Builds an open rectangular sheet (a pillow: the mef face is the
    /// rectangle, the mvfs seed face keeps its placeholder) whose mef
    /// face carries `surface`. Returns the rectangle face.
    fn sheet(
        body: &mut Body<f64>,
        x0: f64,
        y0: f64,
        x1: f64,
        y1: f64,
        surface: FaceSurface<f64>,
    ) -> FaceKey {
        let c = |x: f64, y: f64| Point3::new(x, y, 0.0);
        let (a, b, cc, d) = (c(x0, y0), c(x1, y0), c(x1, y1), c(x0, y1));
        let seed = body.mvfs(a).unwrap();
        let e_ab = body
            .mev_line(
                MevSite::Lone {
                    r#loop: seed.r#loop,
                },
                b,
                Tol::witness(),
            )
            .unwrap();
        let e_bc = body
            .mev_line(
                MevSite::Fan {
                    he1: e_ab.he_minus,
                    he2: e_ab.he_minus,
                },
                cc,
                Tol::witness(),
            )
            .unwrap();
        let e_cd = body
            .mev_line(
                MevSite::Fan {
                    he1: e_bc.he_minus,
                    he2: e_bc.he_minus,
                },
                d,
                Tol::witness(),
            )
            .unwrap();
        let he_dc = body
            .find_half_edge(seed.face, e_cd.vertex, e_bc.vertex)
            .unwrap();
        body.mef(
            MefSite::Chords {
                he1: he_dc,
                he2: e_ab.he_plus,
            },
            EdgeCurveSpec::line_between(d, a),
            surface,
            Tol::witness(),
        )
        .unwrap()
        .face
    }

    #[test]
    fn at_rest_planar_sheets_share_a_key_and_certify_or_answer_empty() {
        // One body, two coplanar rectangle faces on ONE SurfaceKey —
        // the at-rest site. The plane chart is derive-on-demand (C4).
        let mut body = Body::<f64>::new();
        let f1 = sheet(&mut body, 0.0, 0.0, 2.0, 2.0, FaceSurface::New(xy_plane()));
        let key = body.get_face(f1).unwrap().surface;
        let f2 = sheet(&mut body, 1.0, 1.0, 3.0, 3.0, FaceSurface::Shared(key));
        assert_eq!(
            chart_region_overlap(&body, f1, &body, f2, band()).unwrap(),
            ChartOverlap::PositiveArea
        );
        // Disjoint regions answer EMPTY — stale at the consumer.
        let f3 = sheet(&mut body, 5.0, 5.0, 6.0, 6.0, FaceSurface::Shared(key));
        assert_eq!(
            chart_region_overlap(&body, f1, &body, f3, band()).unwrap(),
            ChartOverlap::Empty
        );
        // The self-query rides the structural fast path.
        assert_eq!(
            chart_region_overlap(&body, f1, &body, f1, band()).unwrap(),
            ChartOverlap::PositiveArea
        );
    }

    #[test]
    fn rung2_shared_source_certifies_and_rung3_escalates() {
        let mut body_a = Body::<f64>::new();
        let fa = sheet(
            &mut body_a,
            0.0,
            0.0,
            2.0,
            2.0,
            FaceSurface::New(xy_plane()),
        );
        let ka = body_a.get_face(fa).unwrap().surface;
        let mut body_b = Body::<f64>::new();
        let fb = sheet(
            &mut body_b,
            1.0,
            1.0,
            3.0,
            3.0,
            FaceSurface::New(xy_plane()),
        );
        let kb = body_b.get_face(fb).unwrap().surface;

        // No sources: value-equal descriptions do NOT glue (C2).
        match chart_region_overlap(&body_a, fa, &body_b, fb, band()) {
            Err(ChartRegionError::ChartDivergence { .. }) => {}
            other => panic!("sourceless cross-body pair must escalate, got {other:?}"),
        }

        // Rung 2: the same GeomSource ⇒ bit-identical descriptions ⇒
        // the identical chart (N6) — the pair certifies.
        body_a
            .set_surface_source(ka, GeomSource::minted(7, 0))
            .unwrap();
        body_b
            .set_surface_source(kb, GeomSource::minted(7, 0))
            .unwrap();
        assert_eq!(
            chart_region_overlap(&body_a, fa, &body_b, fb, band()).unwrap(),
            ChartOverlap::PositiveArea
        );

        // Rung 3 (independent recipes): typed chart divergence.
        body_b
            .set_surface_source(kb, GeomSource::minted(8, 0))
            .unwrap();
        match chart_region_overlap(&body_a, fa, &body_b, fb, band()) {
            Err(ChartRegionError::ChartDivergence { .. }) => {}
            other => panic!("distinct sources must escalate, got {other:?}"),
        }

        // Same base, flipped orientation: the mirrored chart diverges.
        body_b
            .set_surface_source(kb, GeomSource::minted(7, 0).reverted())
            .unwrap();
        match chart_region_overlap(&body_a, fa, &body_b, fb, band()) {
            Err(ChartRegionError::ChartDivergence { .. }) => {}
            other => panic!("reverted source must escalate, got {other:?}"),
        }
    }

    // ------------------------------------------------------------------
    // Cylinder walls: stored minted caches, the (r, 1) arm lever, the
    // tilted-cut exclusion at body level.
    // ------------------------------------------------------------------

    /// A point of the unit cylinder at azimuth `u`, height `z`.
    fn cyl_pt(u: f64, z: f64) -> Point3<f64> {
        Point3::new(u.cos(), u.sin(), z)
    }

    /// A forward rim-arc spec at height `z` from azimuth `u0` to `u1`
    /// (`ccw`), or from `u1` down to `u0` (`!ccw`, carried on the
    /// −ẑ-axis circle so the parameter still runs forward).
    fn rim_spec(
        body: &mut Body<f64>,
        cyl: crate::geometry::SurfaceKey,
        z: f64,
        u0: f64,
        u1: f64,
        ccw: bool,
    ) -> EdgeCurveSpec<f64> {
        let plane = body.add_surface(Surface::Plane {
            origin: Point3::new(0.0, 0.0, z),
            normal: Vec3::unit_z(),
            u_ref: Vec3::unit_x(),
        });
        let (carrier, t1) = if ccw {
            (
                Curve3::Circle {
                    center: Point3::new(0.0, 0.0, z),
                    axis: Vec3::unit_z(),
                    radius: 1.0,
                    u_ref: Vec3::unit_x(),
                },
                u1,
            )
        } else {
            // Clockwise: angle t measured from u1 about −ẑ reaches
            // azimuth u1 − t; params [0, u1 − u0].
            (
                Curve3::Circle {
                    center: Point3::new(0.0, 0.0, z),
                    axis: Vec3::new(0.0, 0.0, -1.0),
                    radius: 1.0,
                    u_ref: Vec3::new(u1.cos(), u1.sin(), 0.0),
                },
                u1 - u0,
            )
        };
        let t0 = if ccw { u0 } else { 0.0 };
        let mid = cyl_pt((u0 + u1) * 0.5, z);
        EdgeCurveSpec {
            description: EdgeGeometry::Intersection {
                s1: cyl,
                s2: plane,
                witness: mid,
            },
            carrier,
            param_start: t0,
            param_end: t1,
        }
    }

    /// An open cylinder-wall sheet `u ∈ [u0, u1] × z ∈ [z0, z1]` on
    /// the unit cylinder about ẑ: pass `None` to mint the cylinder
    /// surface (AFTER the seed solid exists — an unreferenced surface
    /// is an orphan at the mvfs postcondition), `Some(key)` to share.
    fn cyl_sheet(
        body: &mut Body<f64>,
        cyl: Option<crate::geometry::SurfaceKey>,
        u0: f64,
        u1: f64,
        z0: f64,
        z1: f64,
    ) -> (FaceKey, crate::geometry::SurfaceKey) {
        let (p00, p10, p11, p01) = (
            cyl_pt(u0, z0),
            cyl_pt(u1, z0),
            cyl_pt(u1, z1),
            cyl_pt(u0, z1),
        );
        let seed = body.mvfs(p00).unwrap();
        let cyl = cyl.unwrap_or_else(|| body.add_surface(cyl_surface(1.0)));
        let bottom = rim_spec(body, cyl, z0, u0, u1, true);
        let e_b = body
            .mev(
                MevSite::Lone {
                    r#loop: seed.r#loop,
                },
                p10,
                bottom,
                Tol::witness(),
            )
            .unwrap();
        let e_r = body
            .mev_line(
                MevSite::Fan {
                    he1: e_b.he_minus,
                    he2: e_b.he_minus,
                },
                p11,
                Tol::witness(),
            )
            .unwrap();
        let top = rim_spec(body, cyl, z1, u0, u1, false);
        let e_t = body
            .mev(
                MevSite::Fan {
                    he1: e_r.he_minus,
                    he2: e_r.he_minus,
                },
                p01,
                top,
                Tol::witness(),
            )
            .unwrap();
        let he = body
            .find_half_edge(seed.face, e_t.vertex, e_r.vertex)
            .unwrap();
        let face = body
            .mef(
                MefSite::Chords {
                    he1: he,
                    he2: e_b.he_plus,
                },
                EdgeCurveSpec::line_between(p01, p00),
                FaceSurface::Shared(cyl),
                Tol::witness(),
            )
            .unwrap()
            .face;
        (face, cyl)
    }

    #[test]
    fn cylinder_walls_overlap_through_the_radius_lever() {
        let mut body = Body::<f64>::new();
        let (w1, cyl) = cyl_sheet(&mut body, None, 0.2, 1.6, 0.0, 1.0);
        let (w2, _) = cyl_sheet(&mut body, Some(cyl), 1.0, 2.4, 0.3, 0.7);
        // Without minted caches a minting chart refuses (props.rs
        // posture) — plane charts are the only derive-on-demand lane.
        match chart_region_overlap(&body, w1, &body, w2, band()) {
            Err(ChartRegionError::MissingCache { .. }) => {}
            other => panic!("unminted cylinder faces must refuse, got {other:?}"),
        }
        crate::pcurves::mint_pcurves(&mut body, Tol::witness()).unwrap();
        assert_eq!(
            chart_region_overlap(&body, w1, &body, w2, band()).unwrap(),
            ChartOverlap::PositiveArea
        );
        // Disjoint azimuth ranges answer EMPTY.
        let (w3, _) = cyl_sheet(&mut body, Some(cyl), 3.0, 4.0, 0.0, 1.0);
        crate::pcurves::mint_pcurves(&mut body, Tol::witness()).unwrap();
        assert_eq!(
            chart_region_overlap(&body, w1, &body, w3, band()).unwrap(),
            ChartOverlap::Empty
        );
    }

    #[test]
    fn the_tilted_cut_exclusion_is_typed_at_body_level() {
        // The honest exclusion, pinned: a wall whose stored cache is
        // the tilted-section SINUSOID (the F5 envelope discipline
        // moved to (u, v)) refuses typed — never a chord read.
        let mut body = Body::<f64>::new();
        let (wall, _) = cyl_sheet(&mut body, None, 0.2, 1.6, 0.0, 1.0);
        crate::pcurves::mint_pcurves(&mut body, Tol::witness()).unwrap();

        // The tilted section z = 0.4·x of the unit cylinder, as its
        // exact ellipse carrier, charted onto the cylinder: the
        // sinusoid Harmonic (pa's v channel = 0.4·cos t).
        let k: f64 = 0.4;
        let major_len = (1.0 + k.powi(2)).sqrt();
        let ellipse = Curve3::Ellipse {
            center: Point3::origin(),
            axis: Vec3::new(-k, 0.0, 1.0).normalize(),
            major: major_len,
            minor: 1.0,
            u_ref: Vec3::new(1.0, 0.0, k).normalize(),
        };
        let surface = cyl_surface(1.0);
        let pcurve = geom_brep::chart_pcurve(&ellipse, &surface, band()).unwrap();
        let geom_brep::Pcurve::Harmonic { pa, .. } = &pcurve else {
            panic!("the tilted section charts to a Harmonic");
        };
        assert!(
            pa.y != 0.0,
            "the tilted section's v channel is a live cosine — the class the gate excludes"
        );
        let (t0, t1) = (0.2, 1.6);
        let window = pcurve.chart_box(t0, t1);
        let cache =
            geom_brep::PcurveCache::certify(pcurve, t0, t1, &ellipse, &surface, window, band())
                .expect(
                    "the sinusoid image itself certifies (C5 row) — the exclusion is the \
                         REGION machinery's, not the cache's",
                );
        // Plant it on the wall's bottom rim: the region query must
        // refuse typed at the inventory gate.
        let bottom_he = {
            let face_data = body.get_face(wall).unwrap();
            let LoopBoundary::Cycle { first } = body.get_loop(face_data.outer).unwrap().boundary
            else {
                panic!("wall outer loop is a cycle");
            };
            body.loop_cycle(first).unwrap()[0]
        };
        body.pcurves.insert(bottom_he, cache);
        match chart_region_overlap(&body, wall, &body, wall, band()) {
            Err(ChartRegionError::NonPlanarTrim { .. }) => {}
            other => panic!("the tilted-cut class must refuse typed, got {other:?}"),
        }
    }

    #[test]
    fn nonconstant_arm_charts_refuse_the_area_claim_typed() {
        // The arm gate itself: exact constant arms exist for plane
        // and cylinder charts ONLY; every other kind refuses typed
        // (the follow-up is inf-stretch bounds, not a sup-arm
        // pseudo-certificate).
        assert_eq!(exact_arms(&xy_plane()).unwrap(), (1.0, 1.0));
        assert_eq!(exact_arms(&cyl_surface(0.25)).unwrap(), (0.25, 1.0));
        for (surface, chart) in [
            (
                Surface::Sphere {
                    center: Point3::origin(),
                    radius: 1.0,
                    axis: Vec3::unit_z(),
                    u_ref: Vec3::unit_x(),
                },
                "sphere",
            ),
            (Surface::nurbs_placeholder(), "NURBS"),
        ] {
            match exact_arms(&surface) {
                Err(ChartRegionError::ArmUnbounded { chart: c }) => assert_eq!(c, chart),
                other => panic!("{chart} must refuse the arm gate, got {other:?}"),
            }
        }
    }

    // ==================================================================
    // R1 blinded-review probes (branch kernel/m9-2a-r1-probes; review
    // evidence only, never for merge into the implementation branch).
    // ==================================================================
    mod r1_probes {
        use super::*;

        // ---- Claim 1: the structural inventory gate cannot be fooled.
        #[test]
        fn r1_near_zero_channels_refuse_at_every_magnitude() {
            for tiny in [1e-300, f64::MIN_POSITIVE, 5e-324, 1e-17, -1e-300] {
                for (pa, pb) in [
                    (Vec2::new(tiny, 0.0), Vec2::zero()),
                    (Vec2::new(0.0, tiny), Vec2::zero()),
                    (Vec2::zero(), Vec2::new(tiny, 0.0)),
                    (Vec2::zero(), Vec2::new(0.0, tiny)),
                ] {
                    let h = geom_brep::Pcurve::Harmonic {
                        p0: pt(0.0, 0.0),
                        pa,
                        pb,
                        pl: Vec2::new(1.0, 0.0),
                    };
                    assert!(
                        pcurve_entry(&h, 0.0, 1.0, true).is_err(),
                        "a {tiny:e} trig channel must refuse typed"
                    );
                }
            }
        }

        #[test]
        fn r1_negative_zero_is_the_zero_function_and_passes() {
            // -0.0 == 0.0 in value: the trig term is the exact zero
            // function, so admitting it is sound (structure, not bits).
            let h = geom_brep::Pcurve::Harmonic {
                p0: pt(0.25, 0.5),
                pa: Vec2::new(-0.0, 0.0),
                pb: Vec2::new(0.0, -0.0),
                pl: Vec2::new(1.0, 2.0),
            };
            assert_pt(pcurve_entry(&h, 0.5, 1.0, true).unwrap(), 0.75, 1.5);
        }

        // ---- Claim 2: the same-chart lane is airtight (and its trust
        // boundary is source attachment, not surface values).
        #[test]
        fn r1_value_equal_but_distinct_keys_on_one_body_escalate() {
            let mut body = Body::<f64>::new();
            let f1 = sheet(&mut body, 0.0, 0.0, 2.0, 2.0, FaceSurface::New(xy_plane()));
            let f2 = sheet(&mut body, 1.0, 1.0, 3.0, 3.0, FaceSurface::New(xy_plane()));
            match chart_region_overlap(&body, f1, &body, f2, band()) {
                Err(ChartRegionError::ChartDivergence { .. }) => {}
                other => panic!("value-equal distinct keys must escalate, got {other:?}"),
            }
        }

        #[test]
        fn r1_the_lane_trusts_source_attachment_not_surface_values() {
            // The SAME minted source attached to value-DIFFERENT plane
            // descriptions (u_ref x̂ vs ŷ). As reviewed, the lane
            // admitted on recipe identity alone (this probe recorded
            // PositiveArea); the union fix (U1) VERIFIES N6's
            // bit-identity conclusion through the module's own
            // exact-bracket comparator, so the forged pair now refuses
            // typed — the rung-2 premise is checked, never assumed.
            let mut a = Body::<f64>::new();
            let fa = sheet(&mut a, 0.0, 0.0, 2.0, 2.0, FaceSurface::New(xy_plane()));
            let ka = a.get_face(fa).unwrap().surface;
            let rotated = Surface::Plane {
                origin: Point3::origin(),
                normal: Vec3::unit_z(),
                u_ref: Vec3::unit_y(),
            };
            let mut b = Body::<f64>::new();
            let fb = sheet(&mut b, 1.0, 1.0, 3.0, 3.0, FaceSurface::New(rotated));
            let kb = b.get_face(fb).unwrap().surface;
            a.set_surface_source(ka, GeomSource::minted(3, 0)).unwrap();
            b.set_surface_source(kb, GeomSource::minted(3, 0)).unwrap();
            match chart_region_overlap(&a, fa, &b, fb, band()) {
                Err(ChartRegionError::ChartDivergence { .. }) => {}
                other => panic!("forged same-source pair must diverge, got {other:?}"),
            }
        }

        // ---- Claims 3/4: dimensional honesty and the band biting.
        #[test]
        fn r1_mm_vs_metre_twins_verdicts_track_the_band_linearly() {
            // Every chart_region_* margin is metres: scaling geometry
            // AND band by s must preserve every verdict. A hidden m² or
            // dimensionless margin flips one of these at s = 1e±3.
            for s in [1e-3, 1.0, 1e3] {
                let band_s = Band::new(1e-9 * s, 1e-8 * s).unwrap();
                let a = face_of(rect(0.0, 0.0, 2.0 * s, 2.0 * s), &[]);
                let b = face_of(rect(1.0 * s, 1.0 * s, 3.0 * s, 3.0 * s), &[]);
                assert_eq!(
                    overlap_of_regions(&a, &b, false, band_s).unwrap(),
                    ChartOverlap::PositiveArea,
                    "positive at scale {s}"
                );
                let sliver = face_of(rect(2.0 * s - 3e-9 * s, -0.3 * s, 4.0 * s, 2.3 * s), &[]);
                match overlap_of_regions(&a, &sliver, false, band_s) {
                    Err(ChartRegionError::Escalated(_)) => {}
                    other => panic!("sliver at scale {s}: {other:?}"),
                }
                let disjoint = face_of(rect(5.0 * s, 5.0 * s, 6.0 * s, 6.0 * s), &[]);
                assert_eq!(
                    overlap_of_regions(&a, &disjoint, false, band_s).unwrap(),
                    ChartOverlap::Empty,
                    "empty at scale {s}"
                );
            }
        }

        #[test]
        fn r1_the_area_row_itself_is_three_outcome_under_band_mutation() {
            // A ring eating the region down to a 3e-9 mean width: every
            // boundary decision is definite (bit-identical outers ride
            // the fast path), so the verdict is the AREA row's alone.
            let d = 3e-9;
            let holed = face_of(
                rect(0.0, 0.0, 2.0, 2.0),
                &[vec![
                    pt(0.0, 0.0),
                    pt(0.0, 2.0 - d),
                    pt(2.0 - d, 2.0 - d),
                    pt(2.0 - d, 0.0),
                ]],
            );
            let probe = face_of(rect(0.0, 0.0, 2.0, 2.0), &[]);
            // In-band mean width: escalates.
            match overlap_of_regions(&holed, &probe, false, band()) {
                Err(ChartRegionError::Escalated(_)) => {}
                other => panic!("in-band net area must escalate, got {other:?}"),
            }
            // Tighter band: the same margin is definite — the row bites.
            let tight = Band::new(1e-12, 1e-11).unwrap();
            assert_eq!(
                overlap_of_regions(&holed, &probe, false, tight).unwrap(),
                ChartOverlap::PositiveArea
            );
        }

        #[test]
        fn r1_the_shipped_sliver_certifies_below_the_band_and_escalates_in_it() {
            let a = face_of(rect(0.0, 0.0, 1.0, 1.0), &[]);
            let b = face_of(rect(1.0 - 3e-9, -0.3, 2.0, 1.3), &[]);
            let tight = Band::new(1e-11, 1e-10).unwrap();
            assert_eq!(
                overlap_of_regions(&a, &b, false, tight).unwrap(),
                ChartOverlap::PositiveArea
            );
            match overlap_of_regions(&a, &b, false, band()) {
                Err(ChartRegionError::Escalated(_)) => {}
                other => panic!("in-band sliver must escalate, got {other:?}"),
            }
        }

        // ---- Claim 5: the seam gate is metred by the azimuth arm.
        #[test]
        fn r1_the_radius_lever_meters_the_seam_row_three_ways() {
            let tau = core::f64::consts::TAU;
            // One geometry, three radii: excess 3e-12 rad reads as
            // 3e-9 m (in-band), 3e-15 m (inside one branch), 3e-6 m
            // (definite branch divergence) purely through the r arm.
            let a = uv_of(rect(0.0, 0.0, 1e-3, 1.0));
            let b = uv_of(rect(tau - 1e-3, 0.0, tau + 3e-12, 1.0));
            match seam_gate(&cyl_surface(1000.0), &a, &b, band()) {
                Err(ChartRegionError::Escalated(_)) => {}
                other => panic!("in-band seam excess must escalate, got {other:?}"),
            }
            seam_gate(&cyl_surface(1e-3), &a, &b, band()).unwrap();
            match seam_gate(&cyl_surface(1e6), &a, &b, band()) {
                Err(ChartRegionError::SeamBranch) => {}
                other => panic!("definite seam excess must refuse, got {other:?}"),
            }
        }

        // ---- Claim 7: adversarial clip-walk configurations.
        #[test]
        fn r1_shared_height_disjoint_rectangles_answer_empty() {
            // B's corners sit on A's edge LINES (the rectangle-corner
            // configuration): a Zero cross-span with a definite Negative
            // elsewhere must read as no-crossing — Negative-first — and
            // the collinear-disjoint lane must not refuse.
            let a = face_of(rect(0.0, 0.0, 1.0, 1.0), &[]);
            let b = face_of(rect(2.0, 0.0, 3.0, 1.0), &[]);
            assert_eq!(
                overlap_of_regions(&a, &b, false, band()).unwrap(),
                ChartOverlap::Empty
            );
        }

        #[test]
        fn r1_a_rotated_square_clips_to_the_octagon() {
            let a = rect(-1.0, -1.0, 1.0, 1.0);
            let b = vec![pt(1.5, 0.0), pt(0.0, 1.5), pt(-1.5, 0.0), pt(0.0, -1.5)];
            let crossings = proper_crossings(&a, &b, band()).unwrap();
            assert_eq!(crossings.len(), 8);
            let pieces = intersection_pieces(&a, &b, &crossings, band()).unwrap();
            assert_eq!(pieces.len(), 1, "one octagon piece");
            let (a2, _) = loop_measures(&pieces[0]);
            // 2A = 2·(4 − 4·0.125) = 7 exactly (dyadic coordinates).
            assert!((a2 - 7.0).abs() < 1e-12, "octagon 2A, got {a2}");
        }

        #[test]
        fn r1_a_degenerate_spike_overlap_escalates_never_silently() {
            // A 4e-9-wide needle penetrating 0.5 deep: every crossing
            // clearance is definite, the piece's mean width is in-band.
            let (w, x) = (2e-9, 0.5);
            let a = face_of(rect(0.0, 0.0, 1.0, 1.0), &[]);
            let spike = face_of(
                vec![
                    pt(x - w, -1.0),
                    pt(x + w, -1.0),
                    pt(x + w, 0.5),
                    pt(x - w, 0.5),
                ],
                &[],
            );
            match overlap_of_regions(&a, &spike, false, band()) {
                Err(ChartRegionError::Escalated(_)) => {}
                other => panic!("spike overlap must escalate, got {other:?}"),
            }
        }

        #[test]
        fn r1_a_vertex_touch_on_an_edge_interior_refuses_typed() {
            let a = face_of(rect(0.0, 0.0, 2.0, 1.0), &[]);
            let t = face_of(vec![pt(0.5, -1.0), pt(1.5, -1.0), pt(1.0, 0.0)], &[]);
            match overlap_of_regions(&a, &t, false, band()) {
                Err(ChartRegionError::TouchingBoundary) => {}
                other => panic!("apex-on-edge touch must refuse typed, got {other:?}"),
            }
        }

        #[test]
        fn r1_partial_collinear_edge_overlap_refuses_typed() {
            // Regions on opposite sides sharing a partial edge run.
            let a = face_of(rect(0.0, 0.0, 2.0, 1.0), &[]);
            let b = face_of(rect(1.0, -1.0, 3.0, 0.0), &[]);
            match overlap_of_regions(&a, &b, false, band()) {
                Err(ChartRegionError::TouchingBoundary) => {}
                other => panic!("partial shared edge must refuse typed, got {other:?}"),
            }
        }

        #[test]
        fn r1_a_repeated_vertex_polygon_never_answers_silently() {
            // A zero-length edge reaches proper_crossings ungated: its
            // parallel row is over_lever(0, 0) = NaN → MarginDiag::
            // Invalid → Escalated. Fail-loud, never a silent verdict.
            let dup = vec![
                pt(0.0, 0.0),
                pt(1.0, 0.0),
                pt(1.0, 0.0),
                pt(1.0, 1.0),
                pt(0.0, 1.0),
            ];
            let a = face_of(dup, &[]);
            let b = face_of(rect(0.5, 0.5, 1.5, 1.5), &[]);
            match overlap_of_regions(&a, &b, false, band()) {
                Err(ChartRegionError::Escalated(_)) => {}
                other => panic!("degenerate edge must escalate, got {other:?}"),
            }
        }

        // ---- Claim 6: replay determinism (same inputs, same verdicts,
        // crossing walk included).
        #[test]
        fn r1_replay_is_bit_deterministic() {
            let a = face_of(rect(-1.0, -1.0, 1.0, 1.0), &[]);
            let b = face_of(
                vec![pt(1.5, 0.0), pt(0.0, 1.5), pt(-1.5, 0.0), pt(0.0, -1.5)],
                &[],
            );
            let first = overlap_of_regions(&a, &b, false, band()).unwrap();
            for _ in 0..8 {
                assert_eq!(overlap_of_regions(&a, &b, false, band()).unwrap(), first);
            }
        }
    }
}
