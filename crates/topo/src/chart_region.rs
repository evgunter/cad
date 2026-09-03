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
//! **Rung-3 (declared) pairs have exactly TWO further authorities**
//! ([`declared_pair_overlap`]):
//!
//! - the shared WORLD CARRIER of a PLANAR pair (ratified as U-R2 with
//!   its justification corrected 2026-08-27). That carrier is a CHOICE
//!   OF REPRESENTATIVE FRAME, not a frame-free object — a plane
//!   description carries `u_ref` too, and the arm reads both trims in
//!   face A's frame. What makes the choice honest is the
//!   **frame-invariance lemma** stated at [`world_carrier`], gated by
//!   the `chart_region_carrier_tilt` row ([`carrier_agreement`]) which
//!   meters the two descriptions' disagreement at the PAIR'S OWN
//!   EXTENT;
//! - the **certified everywhere-within-ε overlap enclosure of a
//!   CYLINDER pair** ([`cylinder_pair_overlap`], issue 943's residue —
//!   the sanctioned closing shape of CENSUS-REST-CLOSURE Q2). Here the
//!   two charts genuinely diverge as charts (`u_ref`, seam, axis
//!   station and direction), so the arm does not pretend they are one:
//!   it CARRIES one description's trim images across the exact affine
//!   relation between the charts — `(u, v) ↦ (δ + σ·u + kτ, c + σ·v)`
//!   — after its own carrier gates certify that the two descriptions
//!   agree within ε everywhere the pair's trims reach (bounded from
//!   the descriptions with the radius-pricing `hyp` lever, then
//!   MEASURED at the trims), and folds the angular coordinate by ONE
//!   whole period pinned per pair through
//!   [`geom_core::Real::periodic_branch`] (CERT-4's repaired fold;
//!   the [`ChartRegionError::PeriodFold`] decline is that primitive's
//!   documented half-period-tie remainder).
//!
//! For both arms the claim earned is *certified everywhere within ε*,
//! never *exact*: `decide`'s `Ok(Zero)` means `|m| ≤ zero`, not
//! bit-zero.
//!
//! Every other declared pair keeps the typed escalation
//! [`ChartRegionError::ChartDivergence`]. C2's caveat — two
//! descriptions of one locus may differ as charts (`u_ref`, seam) — is
//! REAL for the remaining curved kinds, where a certified transfer
//! between the charts has not been built, so the honest posture stays
//! a typed escalation naming the divergence, not a margined
//! pseudo-exact test in whichever chart we happened to pick. The
//! residue, restated per kind at the refusal site
//! ([`declared_pair_overlap`]'s kind gate), keeps the certified-ε
//! enclosure recorded as each kind's sanctioned closing shape.
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
//! the resulting margin is `over_lever(2·A, P)` — the **mean width**
//! of the METRED CHART region, the `split_section_area` precedent.
//! On the exact-arm charts that is the model region's own mean
//! width; on a bounded-arm chart it is a reading on a contracted
//! copy, and the claim is written to the AREA positivity that
//! transfers unconditionally rather than past it — see
//! [`ChartOverlap::PositiveArea`], which carries the transfer factor.
//!
//! Those arms are **inf-side**, and that is the whole of what makes
//! the positive claim sound: scaling a chart polygon by a certified
//! LOWER stretch bound makes it a metric contraction of the model
//! image, so the model region's area is at least the scaled
//! polygon's and every error refuses rather than certifies.
//! `geom_brep::chart_stretch_sup`'s documented over-statement (`sup`
//! stretch bounds) is safe for escape-metering and UNSAFE here — an
//! over-stated arm inflates the margin and would certify a
//! model-space sliver as definitely positive. The two bounds are
//! separate doors so the wrong one cannot be reached by accident.
//!
//! Plane `(1, 1)` and cylinder `(r, 1)` arms are EXACT constants, not
//! bounds. Every other kind takes a certified inf — sphere and cone
//! over the pair's own `v`-window, torus window-free, spline charts
//! from `geom_brep::chart_stretch_inf`'s derivative-net reading with
//! this module's skew discount — and each is gated definitely
//! positive by the `chart_region_arm_inf` row before it is used.
//! A chart with no certified positive arm still refuses
//! [`ChartRegionError::ArmUnbounded`]: a sphere window reaching a
//! pole, a cone window straddling the apex, a degenerate ring torus,
//! a zero-crossing (folded) derivative net. See [`certified_arms`]
//! for each derivation.
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
    /// The trim regions' intersection has **definitely-positive
    /// model-space area** — the `PatchContact` blessing direction.
    ///
    /// # Exactly what the margin behind this says, and about what
    ///
    /// Two rows produce this variant and the reading splits between
    /// them. `chart_region_cyl_band_area`
    /// ([`cylinder_pair_overlap`]) scales by the cylinder's EXACT
    /// arms `(r, 1)`, so its metred chart IS the model metric and its
    /// mean width is the model region's, full stop. The paragraphs
    /// below are about the general row.
    ///
    /// The decided margin (`chart_region_area`) is the region's mean
    /// width `2A/P` **in the metred chart** — the chart polygon
    /// scaled by [`certified_arms`]' lever arms. That copy is a
    /// certified metric CONTRACTION of the model image, so what
    /// transfers to model space is the AREA: `|det| ≥ 1` pointwise
    /// gives `A_model ≥ A_scaled > 0`, and area positivity is what
    /// this variant claims.
    ///
    /// **The mean width does NOT transfer as-is, and this variant
    /// does not claim it does.** Under a contraction the perimeter
    /// grows as well as the area, and `2A/P` is not monotone: the
    /// honest general relation is
    /// `mw_model ≥ (ρ/√T)·mw_scaled`, with `ρ` and `T` the assembly's
    /// own skew discount and normalized trace bound
    /// ([`certified_arms`]), a factor that is exactly **1** on a
    /// plane or cylinder chart — whose arms are exact constants, so
    /// the metred chart IS the model metric and the scaled reading is
    /// the model reading — and as small as ~0.026 on a rational
    /// spline wall. A chart whose stretch is concentrated in thin
    /// boundary layers is the shape that separates the two readings
    /// (`a_stretch_concentrating_chart_separates_the_two_widths`).
    ///
    /// So: definite area, always; a mean width that is the model
    /// region's only on the exact-arm charts, and elsewhere a reading
    /// on the contracted copy that bounds the model's only after the
    /// `ρ/√T` factor.
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
    /// The chart has no certified positive LOWER stretch bound in at
    /// least one channel over the pair's own window, so no
    /// positive-extent claim can be metred honestly there: a sphere
    /// window reaching a pole, a cone window straddling the apex, a
    /// degenerate ring torus, a spline chart whose derivative net
    /// crosses zero or whose skew swallows its own bound. The `sup`
    /// bounds exist for every chart and are the wrong side here
    /// ([`certified_arms`]), so they are not a fallback.
    ArmUnbounded {
        /// The chart kind that refused.
        chart: &'static str,
    },
    /// The pair's loops do not fit one period-wide azimuth window on
    /// a periodic chart — no common-branch region representation
    /// exists (module docs).
    SeamBranch,
    /// The whole-period fold that carries one cylinder description's
    /// chart window onto the other's could not be PINNED to a single
    /// integer at this scalar. The fold runs through
    /// [`geom_core::Real::periodic_branch`] (issue 1191's widening
    /// class was repaired by CERT-4/#1303); what this variant carries
    /// is that primitive's DOCUMENTED honest remainder — the two
    /// windows sit a genuine half-period tie apart, the enclosure
    /// spans two integers, and no branch may be picked. Conservative
    /// BY DIRECTION: the tie declines toward escalation, never toward
    /// certifying a false overlap.
    PeriodFold,
    /// A declared pair's two carrier DESCRIPTIONS are definitely apart
    /// somewhere over the pair's OWN extent. Door 1 certified the
    /// carriers at its pinned 1 m arm; at this pair's actual size they
    /// do not agree, so neither description can stand as the pair's
    /// representative chart. Emitters, each metering its own quantity:
    /// the planar arm's `chart_region_carrier_tilt` (the carriers'
    /// separation at the trims' own vertices) and the cylinder arm's
    /// `chart_region_cyl_radius` / `chart_region_cyl_tilt` /
    /// `chart_region_cyl_offset` / `chart_region_cyl_transfer` (the
    /// chart transfer's error, bounded from the descriptions and
    /// measured at the trims). Recourse is the geometry, not the
    /// declaration: a contact the size of a table is not certified by
    /// a disagreement a peg would absorb.
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
                "chart-region: a {chart} chart has no certified positive lower stretch \
                 bound over this pair's window, so the positive-area claim cannot be \
                 metred honestly — restrict the pair to a window clear of the chart's \
                 degeneracy (a pole, an apex, a fold), or read the pair on a chart that \
                 has one"
            ),
            Self::SeamBranch => write!(
                f,
                "chart-region: the pair's loops do not fit one period-wide azimuth \
                 window — different periodic branches have no common region \
                 representation (branch normalization is a later rung)"
            ),
            Self::PeriodFold => write!(
                f,
                "chart-region: the cross-description angular fold could not be \
                 pinned to one whole period at this scalar — the two windows sit \
                 a genuine half-period tie apart (periodic_branch's documented \
                 remainder); the tie declines, it never certifies"
            ),
            Self::CarrierTilt => write!(
                f,
                "chart-region: a declared pair's two carrier descriptions are \
                 definitely apart over the pair's OWN extent — the representative \
                 chart needs the two descriptions to agree everywhere the trims \
                 reach, and Door 1's 1 m lever arm does not price a contact at \
                 its own size"
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
/// of two faces on one STRUCTURALLY-identified chart. Pass the same
/// `&Body` twice for the at-rest (one-body) site.
///
/// This is the structural door and it stays structural: a pair with no
/// shared `SurfaceKey` and no shared `GeomSource` gets
/// [`ChartRegionError::ChartDivergence`] here however coincident its
/// geometry looks, because value equality never glues (F6). A pair
/// whose chart authority is a VERIFIED DECLARATION is a different
/// question, asked at [`declared_pair_overlap`] — which asks this one
/// first.
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
/// Three authorities answer the same question, in fixed order:
///
/// - [`same_chart`] — the descriptions are structurally ONE chart
///   (shared key / same `GeomSource`), so the trims are read in it
///   directly. Strictly stronger, so it is asked first.
/// - the **shared world carrier**, PLANAR pairs
///   ([`world_carrier`]): a representative frame, legitimate exactly
///   to the extent of that function's frame-invariance lemma, and only
///   once [`carrier_agreement`] has certified that the two
///   descriptions agree over the PAIR'S OWN EXTENT.
/// - the **certified-ε enclosure**, CYLINDER pairs
///   ([`cylinder_pair_overlap`], issue 943's residue closed): one
///   description's trim images carried across the exact affine chart
///   relation onto the other's, gated by that arm's own carrier
///   agreement at the pair's own extent.
///
/// A pair with none of the three keeps [`same_chart`]'s typed
/// divergence, per kind:
///
/// - **sphere** — residue: the enclosure needs the fold on BOTH chart
///   coordinates plus polar-cap branch handling, and the arm gate
///   already refuses spheres ([`ChartRegionError::ArmUnbounded`]: no
///   exact constant lever arms). Closing shape on record: the
///   certified-ε enclosure with inf-arm bounds.
/// - **cone** — residue: as the sphere (no exact constant arms; the
///   apex compounds it); same recorded closing shape.
/// - **torus** — residue: two periodic coordinates, plus the #968
///   declared-Rest lane owns the torus descent; same recorded closing
///   shape, sequenced with that lane.
///
/// # `door_one` — why the verdict is an argument, not a re-derivation
///
/// The world carrier is a carrier BECAUSE the declaration was
/// verified, so this door is not independent of Door 1 and the type
/// now says so: it cannot be reached without the verdict that verified
/// the pair. It is READ in two places: the planar interior-witness
/// rung, which runs only on [`ContactVerdict::Definite`]
/// ([`interior_witness`] states why), and the cylinder arm's premise
/// budget, which TIGHTENS under `Bridged` — a bridged carrier has
/// already spent an in-band residue at Door 1's own rows, so the
/// enclosure's gates decide against a half-zero-edge band there
/// ([`cylinder_pair_overlap`]'s door-one section). The cylinder arm
/// still has no witness rung (a flush cylinder seat's
/// collinear-boundary refusal stands, stated below).
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
    // The kind gate (fixed order: cylinder enclosure, then the planar
    // world carrier, then the per-kind divergence — the doc lists the
    // residue each refused kind keeps).
    if matches!(face_surface(body_a, face_a)?, Surface::Cylinder { .. })
        && matches!(face_surface(body_b, face_b)?, Surface::Cylinder { .. })
    {
        return cylinder_pair_overlap(body_a, face_a, body_b, face_b, door_one, band);
    }
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
    // 3. Certified LOWER arms over the pair's own v-window, or a
    //    typed refusal. Inf-side because the claim below is a
    //    POSITIVE-extent one (`certified_arms`).
    // The window is over BOTH faces' extracted loops, and neither can
    // be empty: `loop_uv_polygon` refuses `DegenerateLoop` on any loop
    // with fewer than 3 vertices — the outer loop and every ring —
    // before extraction returns, so a `FaceUv` reaching here has at
    // least three vertices per polygon. A `None` therefore means the
    // extraction gate let an empty polygon through, which is a kernel
    // bug and not a shape any input can present; it is announced, not
    // laundered into a typed refusal that would read as a bad body.
    let Some((v_lo, v_hi)) = v_window(uv_a, uv_b) else {
        unreachable!(
            "chart-region: both faces' UV loops are empty, but              `loop_uv_polygon` refuses `DegenerateLoop` below three              vertices on the outer loop and on every ring, so an              empty polygon cannot reach the arm stage"
        )
    };
    let (arm_u, arm_v) = certified_arms(surface, v_lo, v_hi, band)?;

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
/// [`certified_arms`] are `(1, 1)` in either frame — a plane chart's
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

/// One cylinder description's frame, destructured.
struct CylFrame<T: Decide> {
    origin: geom_core::Point3<T>,
    axis: geom_core::Vec3<T>,
    radius: T,
    u_ref: geom_core::Vec3<T>,
}

fn cyl_frame<T: Decide>(body: &Body<T>, face: FaceKey) -> Result<CylFrame<T>, ChartRegionError> {
    match face_surface(body, face)? {
        Surface::Cylinder {
            origin,
            axis,
            radius,
            u_ref,
        } => Ok(CylFrame {
            origin: *origin,
            axis: *axis,
            radius: *radius,
            u_ref: *u_ref,
        }),
        _ => Err(ChartRegionError::Corrupt),
    }
}

/// **The certified-ε overlap enclosure of a declared CYLINDER pair**
/// (issue 943's residue; the sanctioned closing shape of
/// CENSUS-REST-CLOSURE Q2 + latitude note 2). Door 2's cylinder arm:
/// trim overlap decided on the ONE carrier Door 1 verified, by
/// carrying face B's chart images across the exact affine relation
/// between the two descriptions' charts and running the module's own
/// region machinery in face A's chart.
///
/// # The transfer, and why it is exact modulo the gates
///
/// Write A's chart map `φ_A(u, v) = o_a + radial_a(u)·r_a + â_a·v`
/// and B's likewise. When the two descriptions describe ONE cylinder
/// — which is precisely what the carrier gates below certify to
/// within ε at the pair's own extent — the correspondence between the
/// charts is
///
/// ```text
/// u_A = δ + σ·u_B + k·τ        v_A = c + σ·v_B
/// δ  = atan2(û_b·ŵ_a, û_b·û_a)   (ŵ_a = â_a × û_a)
/// c  = (o_b − o_a)·â_a           σ = sign(â_a·â_b) ∈ {+1, −1}
/// ```
///
/// derived from the chart convention alone (`radial(u)` winds by the
/// right hand about `axis`, so an opposed axis winds the azimuth the
/// other way AND runs `v` the other way — σ flips both). In METRED
/// coordinates `(r·u, v)` the transfer is a plane isometry with
/// determinant +1 (σ = −1 is a rotation by π, never a reflection:
/// both charts' normals point radially outward, so both orient the
/// surface identically). No approximation enters the FORMULA; what is
/// approximate is the premise that the two descriptions agree, and
/// that premise is not assumed — it is DISCHARGED TWICE below: bounded
/// from the descriptions (the parametric gates, whose tilt lever
/// prices the radius) and then MEASURED at the trims themselves (the
/// `chart_region_cyl_transfer` row). The claim earned is *certified
/// everywhere within ε* (a small fixed multiple of the band, as
/// everywhere in this module), never *exact*.
///
/// # The carrier gates (the cylinder `carrier_agreement`)
///
/// Door 1's ladder decided the same data at its pinned 1 m arm
/// (`carrier_cyl_axis_parallel`·1 m, `carrier_cyl_axis_offset`,
/// `carrier_cyl_radius`); as with the planar arm, that prices a peg
/// and a table identically, so the enclosure re-decides at the PAIR'S
/// OWN EXTENT (fixed order, D9). The quantity the gates must bound is
/// the TRANSFER ERROR `E(p) = φ_A(T(u, v)) − p` — not merely the
/// carriers' radial separation — and to first order it decomposes as
/// `|E| ≤ |Δr| + g⊥ + sin θ · ‖p − o_b‖`, where the last term is the
/// displacement of `p` under the rigid rotation aligning the two
/// frames. For `p` on B's cylinder, `‖p − o_b‖² = v² + r_b²`, so the
/// rotation term is levered by **`hyp = √(reach² + r_max²)`** — the
/// RADIUS is part of the lever (the tilt's first-order axial error is
/// `r·θ·cos u`, which no axial-reach lever prices; the review's
/// bilateral finding):
///
/// - `chart_region_cyl_radius` — `r_a − r_b`, metres at unit arm (a
///   length is not re-levered; enters `E` at weight one).
/// - `chart_region_cyl_tilt` — `‖â_a × â_b‖` levered by `hyp`.
///   `reach` is the largest |axial coordinate| of any boundary vertex
///   of either face about either description's own origin, and
///   `r_max = max(r_a, r_b)` (both union-maxes over order-symmetric
///   multisets — the `chart_region_carrier_tilt` construction). `hyp`
///   over-states `‖p − o_b‖` for every trim point (|v| ≤ reach on the
///   boundary, and a straight-chart-edge region attains its v-extremes
///   on the boundary), which widens toward decline — the conservative
///   direction; the second-order remainder rides the module's standing
///   small-multiple posture.
/// - `chart_region_cyl_offset` — the larger of the two perpendicular
///   origin-to-other-axis distances (`g⊥`), metres at unit arm
///   (symmetric by the same max-over-multiset construction).
///
/// All three `Zero` bound `E` everywhere on the trims — vertices AND
/// interiors, because each term is a description-level bound, not a
/// sample. A definite nonzero refuses
/// [`ChartRegionError::CarrierTilt`]; a definite Negative on an
/// unsigned (norm) margin is poisoned input and escalates `Invalid`
/// (the `chart_region_carrier_tilt` precedent); in-band escalates
/// named.
///
/// # The measured discharge (`chart_region_cyl_transfer`)
///
/// The parametric bound argues; this row MEASURES (the planar
/// `chart_region_carrier_tilt` structural read, transferred): for
/// every boundary vertex of both trims, the world-metre distance
/// between the vertex's transferred chart position mapped through A's
/// chart map and the vertex's own arena point — `‖φ_A(T(u, v)) − p‖`,
/// exactly the quantity the claim is about, at the trims themselves.
/// Face A's term runs the identity transfer, so the row also verifies
/// A's own minted images against A's own points. Exact at vertices;
/// the between-vertex gap is what the parametric gates close (their
/// bound is global). Zero proceeds; definite Positive refuses
/// [`ChartRegionError::CarrierTilt`]; in-band escalates.
///
/// # Door 1's verdict, consumed
///
/// The transfer's premise is co-signed by the declaration Door 1
/// verified, and the verdict says HOW: `Definite` — the geometry's own
/// evidence carried every carrier row; `Bridged` — some carrier row
/// sat in band at Door 1's own arms and the declaration bridged it,
/// i.e. part of the ε budget is already spent before this arm adds its
/// transfer error on top. The arm therefore tightens its PREMISE
/// budget under `Bridged`: the gate rows decide against a band whose
/// zero edge is HALVED (escalate edge unchanged). The tightening is a
/// conservative allocation, not a derived identity — it can only move
/// a gate outcome from certify-onward into escalation, never the
/// reverse — and it is pinned by the Bridged-tightens row in the
/// acceptance suite. `ContactVerdict` has only passing variants (a
/// Door-1 refusal is `ContactRefusal`, a different type), so consuming
/// the verdict admits no refused pair either way.
///
/// # The fold (CERT-4's repaired primitive; the tie remainder)
///
/// `δ` is a principal-value angle and each polygon rides its own
/// pinned branch, so the transferred window may sit whole periods
/// away from A's. ONE integer `k` — `(mid_A − mid_B̃).periodic_branch(τ)`
/// — is pinned per pair and applied to every transferred vertex.
/// Issue 1191's floor-widening class was REPAIRED by CERT-4 (#1303),
/// and this arm consumes the repaired primitive; what remains is that
/// primitive's DOCUMENTED honest remainder — at the interval scalar
/// the enclosure spans two integers exactly when the windows sit a
/// genuine half-period tie apart — and there the integer cannot be
/// pinned and the arm declines [`ChartRegionError::PeriodFold`]. The
/// widening runs toward decline only, never toward a false
/// certification.
///
/// # Decline posture (issue 1435's disclosure obligation)
///
/// This arm's schedule is: one global fold, no branch splitting, no
/// witness rescue. It therefore DECLINES on decidable geometry in
/// three named places, each typed:
///
/// - a pair whose folded windows cannot co-inhabit one period-wide
///   window ([`ChartRegionError::SeamBranch`] via [`seam_gate`]) even
///   when the quotient overlap is decidable — e.g. two full-period
///   walls whose seams disagree by more than ε (the full-wrap BAND
///   fast path below retires the rectangular sub-class; the rest
///   stands as the stated residue, branch normalization being a later
///   rung);
/// - a cylinder seat sharing trim boundary
///   ([`ChartRegionError::TouchingBoundary`]): the interior-witness
///   rung does not run here at all;
/// - the [`ChartRegionError::PeriodFold`] fold decline above.
///
/// The incompleteness is the schedule's, not the geometry's, and it is
/// disclosed here rather than sampled around.
///
/// The middle bullet is the one issue 1435 has since moved, and it
/// moved only on the planar side: [`interior_witness`]'s schedule is
/// now complete (its own docs carry the argument), so a PLANE pair no
/// longer declines a decidable overlap for want of a candidate. None
/// of that reaches this arm, and the reason is worth naming exactly,
/// because it is no longer the schedule. The schedule's completion is
/// a decomposition of the CHART plus a certificate from
/// [`crate::boolean::contfp`], and the decomposition transfers to
/// `(θ·r, z)` unchanged — the blocker is the certificate: `contfp`
/// requires the query point already on the PLANE of the face, and a
/// cylinder pair has no plane to put it on. What this arm needs is
/// therefore a curved-carrier containment discharge, not more
/// candidates; until it has one, a cylinder seat's shared-boundary
/// refusal stands.
///
/// # The full-wrap band fast path
///
/// The ordinary mated shaft-in-bore pair is two FULL-PERIOD wall
/// rectangles whose seams need not agree. A full-period rectangle is
/// azimuth-invariant as a region of the quotient cylinder, so the
/// overlap question collapses to the axial interval intersection —
/// decided exactly, no fold needed, BEFORE the fold can decline it:
///
/// - `chart_region_cyl_wrap` — a rectangle's azimuth span less τ,
///   levered by the radius to metres; `Zero` = full wrap.
/// - `chart_region_cyl_band` — the axial overlap of the two bands in
///   metres: `Positive` proceeds to the area claim, `Negative` is the
///   certified [`ChartOverlap::Empty`] (the stale-declaration arm),
///   `Zero` is a rim-sharing touch ([`ChartRegionError::TouchingBoundary`]).
/// - `chart_region_cyl_band_area` — the overlap band's mean width
///   `over_lever(2·τ·r·h, 2(τ·r + h))` after the module's standard
///   conservative ring deduction (ring measures are isometry
///   invariants, so the transferred rings' measures are their minted
///   ones). Positive certifies; anything else escalates.
///
/// Detection is structural (C6): exactly four exact-point vertices
/// forming an axis-aligned rectangle in chart coordinates — anything
/// else falls through to the general walk.
///
/// **Lane asymmetry, disclosed**: the transferred polygon's
/// coordinates carry `δ` (an `atan2` result) and the transfer's
/// products, which are NOT point brackets at the interval scalar —
/// so the exact-point read rejects `uv_b` there and the band fast
/// path is structurally **f64-only** (pinned by
/// `r1_mate5_interval_probe.rs`). At interval, full-period pairs take
/// the general path and decline honestly at the fold or seam gate;
/// the conservative direction — the asymmetry can suppress a
/// certification, never mint one.
///
/// # Frame invariance (the lemma, cylinder form)
///
/// The metred transfer `ψ: (r·u, v) ↦ (r·u_A, v_A)` is — within the ε
/// the gates certify — a plane isometry with det +1 (above), and
/// everything computed downstream of the loop walk is a Euclidean
/// invariant of the metred chart: `loop_measures`' shoelace and
/// perimeter, `proper_crossings`' incidence, `polygon_relation`'s
/// containment, the band arm's axial heights, and the final
/// `over_lever(2A, P)` mean width. The metering matches because the
/// azimuth arm is the radius and the radii agree within ε (the radius
/// gate). Orientation needs no absorbing — det +1 — though
/// `ScaledFace::build` normalizes to CCW regardless. Therefore
/// [`ChartOverlap::PositiveArea`] and [`ChartOverlap::Empty`] are
/// invariant under which description is carried onto which, which is
/// the claim. ∎  As with the planar lemma, the REFUSAL boundary is
/// not exactly invariant: the fold integer, the window fit and the
/// ray schedule are branch- and frame-dependent near ties, and a
/// margin within a few ulps of a band edge may classify differently —
/// the module's standing posture. The both-ways row on the fixture
/// corpus (`mate5_cyl_eps_rung.rs`) pins verdict-class agreement.
///
/// # Errors
///
/// [`ChartRegionError`] — the carrier-gate refusals and escalations,
/// the fold decline, and everything the shared pipeline
/// ([`overlap_of_uv`]) can refuse.
fn cylinder_pair_overlap<T: Decide + Bounds>(
    body_a: &Body<T>,
    face_a: FaceKey,
    body_b: &Body<T>,
    face_b: FaceKey,
    door_one: crate::contact::ContactVerdict,
    band: Band,
) -> Result<ChartOverlap, ChartRegionError> {
    let a = cyl_frame(body_a, face_a)?;
    let b = cyl_frame(body_b, face_b)?;

    // --- Door 1's verdict, CONSUMED (doc above): a `Bridged` carrier
    //     has already spent an in-band residue at Door 1's own rows,
    //     so the enclosure's PREMISE budget is tightened — the zero
    //     edge halves, the escalate edge stays. Tightening is
    //     conservative by direction: it can only move a gate outcome
    //     from Zero (certify onward) into the escalation band, never
    //     the reverse, and a definite-apart refusal is unmoved.
    //     `ContactVerdict` has only PASSING variants (a Door-1 refusal
    //     is `ContactRefusal`, a different type), so no refused pair
    //     can reach here in the first place.
    let gate_band = match door_one {
        crate::contact::ContactVerdict::Definite => band,
        crate::contact::ContactVerdict::Bridged => {
            match Band::new(band.zero() / 2.0, band.escalate()) {
                Ok(tight) => tight,
                // Halving a valid band's zero edge keeps 0 < zero/2 <
                // escalate: the constructor cannot refuse (D2 row 4).
                Err(_) => unreachable!("halving a valid band's zero edge keeps it valid"),
            }
        }
    };

    // --- The carrier gates, at the pair's own extent (doc above).
    //     `reach` is the axial reach (largest |axial coordinate| of a
    //     boundary vertex about either origin, both faces — union-max
    //     over an order-symmetric multiset); `hyp` is the TRANSFER
    //     lever `√(reach² + r_max²)`: every point p of either trim
    //     satisfies ‖p − o‖² = v² + r² ≤ reach² + r_max², so a rigid
    //     rotation by θ between the two descriptions displaces trim
    //     points by at most sin θ · hyp — the radius is priced.
    let mut reach = T::zero();
    for (body, face) in [(body_a, face_a), (body_b, face_b)] {
        for p in face_boundary_points(body, face)? {
            reach = reach
                .max((p - a.origin).dot(a.axis).abs())
                .max((p - b.origin).dot(b.axis).abs());
        }
    }
    let r_max = a.radius.max(b.radius);
    let hyp = (reach.powi(2) + r_max.powi(2)).sqrt();
    // A gate over a SIGNED margin (the radius difference): either
    // definite sign is a genuine "definitely apart".
    let signed_gate = |name: &'static str, margin: Margin<T>| -> Result<(), ChartRegionError> {
        match decide(name, margin, gate_band) {
            Ok(Sign::Zero) => Ok(()),
            Ok(_) => Err(ChartRegionError::CarrierTilt),
            Err(diag) => Err(ChartRegionError::Escalated(diag)),
        }
    };
    // A gate over an UNSIGNED margin (a norm): a definite Negative is
    // unreachable, so it is poisoned input and escalates `Invalid` —
    // the `chart_region_carrier_tilt` precedent.
    let norm_gate = |name: &'static str, margin: Margin<T>| -> Result<(), ChartRegionError> {
        match decide(name, margin, gate_band) {
            Ok(Sign::Zero) => Ok(()),
            Ok(Sign::Positive) => Err(ChartRegionError::CarrierTilt),
            Ok(Sign::Negative) => Err(ChartRegionError::Escalated(Indeterminate {
                margin: geom_core::MarginDiag::Invalid,
                band: gate_band,
                predicate: Some(name),
            })),
            Err(diag) => Err(ChartRegionError::Escalated(diag)),
        }
    };
    signed_gate("chart_region_cyl_radius", Margin::of(a.radius - b.radius))?;
    norm_gate(
        "chart_region_cyl_tilt",
        Margin::levered(a.axis.cross(b.axis).norm(), hyp),
    )?;
    let d_ab = b.origin - a.origin;
    let d_ba = a.origin - b.origin;
    let perp_a = (d_ab - a.axis * d_ab.dot(a.axis)).norm();
    let perp_b = (d_ba - b.axis * d_ba.dot(b.axis)).norm();
    norm_gate("chart_region_cyl_offset", Margin::of(perp_a.max(perp_b)))?;

    // --- The transfer parameters (doc above). The sense margin is the
    //     axial-direction cosine LEVERED BY `hyp`: metres of travel at
    //     the pair's own extent per unit axial step, so the sign read
    //     goes through the metres door with an honest length.
    let sigma = match decide(
        "chart_region_cyl_axis_sense",
        Margin::levered(a.axis.dot(b.axis), hyp),
        band,
    ) {
        Ok(Sign::Positive) => T::one(),
        Ok(Sign::Negative) => T::zero() - T::one(),
        // A zero-band axis dot contradicts the tilt gate's Zero (the
        // dot of near-parallel units is near ±1): poisoned input.
        Ok(Sign::Zero) => {
            return Err(ChartRegionError::Escalated(Indeterminate {
                margin: geom_core::MarginDiag::Invalid,
                band,
                predicate: Some("chart_region_cyl_axis_sense"),
            }));
        }
        Err(diag) => return Err(ChartRegionError::Escalated(diag)),
    };
    let w_a = a.axis.cross(a.u_ref);
    let delta = b.u_ref.dot(w_a).atan2(b.u_ref.dot(a.u_ref));
    let c = (b.origin - a.origin).dot(a.axis);

    // --- Extraction (each face read in its OWN minted chart). ---
    let surface_a = face_surface(body_a, face_a)?.clone();
    let surface_b = face_surface(body_b, face_b)?.clone();
    let uv_a = extract_face_uv(body_a, face_a, &surface_a, ChartRead::Minted, band)?;
    let uv_b_own = extract_face_uv(body_b, face_b, &surface_b, ChartRead::Minted, band)?;
    let map = |p: &Point2<T>| Point2::new(delta + sigma * p.x, c + sigma * p.y);
    let mut uv_b = FaceUv {
        outer: uv_b_own.outer.iter().map(map).collect(),
        rings: uv_b_own
            .rings
            .iter()
            .map(|r| r.iter().map(map).collect())
            .collect(),
    };

    // --- The MEASURED discharge (doc above): the transfer's residual
    //     at every boundary vertex of both trims, in world metres —
    //     the quantity the arm claims is small, measured where the
    //     trims actually are, through A's own chart map. Face A's term
    //     is its identity transfer (its minted images against its own
    //     world points); face B's carries the full affine transfer.
    let phi_a = |q: &Point2<T>| -> geom_core::Point3<T> {
        let (s, cu) = q.x.sin_cos();
        a.origin + (a.u_ref * cu + w_a * s) * a.radius + a.axis * q.y
    };
    let mut residual = T::zero();
    for (body, face, uv) in [(body_a, face_a, &uv_a), (body_b, face_b, &uv_b)] {
        residual = residual.max(transfer_residual(body, face, uv, &phi_a)?);
    }
    norm_gate("chart_region_cyl_transfer", Margin::of(residual))?;

    // --- The full-wrap band fast path (doc above; asked BEFORE the
    //     fold, which cannot serve two misaligned full-period walls).
    if let (Some(band_a), Some(band_b)) = (
        wrap_band(&uv_a.outer, a.radius, band)?,
        wrap_band(&uv_b.outer, a.radius, band)?,
    ) {
        return band_overlap(&uv_a, &uv_b, band_a, band_b, a.radius, band);
    }

    // --- The fold: one whole period, pinned per pair through
    //     `Real::periodic_branch` (issue 1191's widening class was
    //     repaired by CERT-4/#1303; what remains is that primitive's
    //     DOCUMENTED honest remainder — a genuine half-period tie,
    //     where the enclosure spans two integers and the arm declines
    //     typed rather than picking a branch).
    let k = (window_mid(&uv_a) - window_mid(&uv_b)).periodic_branch(T::tau());
    if !(k.lo() == k.hi() && k.lo().is_finite()) {
        return Err(ChartRegionError::PeriodFold);
    }
    let shift = k * T::tau();
    let fold = |p: &Point2<T>| Point2::new(p.x + shift, p.y);
    uv_b = FaceUv {
        outer: uv_b.outer.iter().map(fold).collect(),
        rings: uv_b
            .rings
            .iter()
            .map(|r| r.iter().map(fold).collect())
            .collect(),
    };

    // --- The shared pipeline, in A's chart (arms (r_a, 1); the seam
    //     gate is the window-fit decline the doc names).
    overlap_of_uv(
        body_a, face_a, body_b, face_b, &surface_a, &uv_a, &uv_b, band,
    )
}

/// The largest world-metre distance between a face's transferred chart
/// vertices, mapped back through the representative chart `phi_a`, and
/// the face's OWN arena boundary points — walked in the same fixed
/// order both sides use (outer then rings, cycle order, one entry per
/// half-edge), so the pairing is positional and exact.
///
/// INVARIANT (the pairing): [`loop_uv_polygon`] pushes each half-edge's
/// ENTRY vertex image and [`face_boundary_points`] pushes each
/// half-edge's START vertex point, over the same `loop_cycle` walk —
/// index i of the one is the chart image of index i of the other. A
/// length mismatch means the two walks diverged and is a kernel
/// invariant violation, not a geometric answer.
fn transfer_residual<T: Decide + Bounds>(
    body: &Body<T>,
    face: FaceKey,
    uv: &FaceUv<T>,
    phi_a: &impl Fn(&Point2<T>) -> geom_core::Point3<T>,
) -> Result<T, ChartRegionError> {
    let points = face_boundary_points(body, face)?;
    let n_uv = uv.outer.len() + uv.rings.iter().map(Vec::len).sum::<usize>();
    if points.len() != n_uv {
        return Err(ChartRegionError::Corrupt);
    }
    let mut worst = T::zero();
    for (q, p) in uv
        .outer
        .iter()
        .chain(uv.rings.iter().flatten())
        .zip(points.iter())
    {
        worst = worst.max((phi_a(q) - *p).norm());
    }
    Ok(worst)
}

/// The azimuth midpoint of a face's chart window (all loops).
fn window_mid<T: Decide + Bounds>(uv: &FaceUv<T>) -> T {
    let mut lo: Option<T> = None;
    let mut hi: Option<T> = None;
    let mut visit = |poly: &Vec<Point2<T>>| {
        for p in poly {
            lo = Some(match lo {
                None => p.x,
                Some(m) => m.min(p.x),
            });
            hi = Some(match hi {
                None => p.x,
                Some(m) => m.max(p.x),
            });
        }
    };
    visit(&uv.outer);
    uv.rings.iter().for_each(&mut visit);
    let half = T::one() / (T::one() + T::one());
    match (lo, hi) {
        (Some(l), Some(h)) => (l + h) * half,
        // A `FaceUv` exists only past extraction, which refuses every
        // loop under three vertices — an empty window is a kernel
        // invariant violation (D2 row 4), and a poison return here
        // would misattribute it as a fold tie (`PeriodFold`).
        _ => unreachable!("a FaceUv survives extraction only with vertices"),
    }
}

/// Reads one outer polygon as a FULL-PERIOD axis-aligned rectangle:
/// `Some((v_lo, v_hi))` when the polygon is exactly four exact-point
/// vertices forming an axis-aligned rectangle in chart coordinates
/// (C6: structure read as structure) whose azimuth span is decidedly
/// one period (`chart_region_cyl_wrap`, levered by the radius);
/// `None` otherwise — the general walk's ground.
///
/// # Errors
///
/// [`ChartRegionError::Escalated`] when the span sits in the sliver
/// band — a nearly-full wrap is neither a band nor a general window
/// and must not be silently classified as either.
fn wrap_band<T: Decide + Bounds>(
    outer: &[Point2<T>],
    radius: T,
    band: Band,
) -> Result<Option<(T, T)>, ChartRegionError> {
    if outer.len() != 4 {
        return Ok(None);
    }
    // Exact-point reads (the `bit_equal_cyclic` closure's shape).
    let exact = |p: &Point2<T>| -> Option<(f64, f64)> {
        let (xl, xh, yl, yh) = (p.x.lo(), p.x.hi(), p.y.lo(), p.y.hi());
        (xl == xh && yl == yh && xl.is_finite() && yl.is_finite()).then_some((xl, yl))
    };
    let Some(pts) = outer.iter().map(exact).collect::<Option<Vec<_>>>() else {
        return Ok(None);
    };
    // Axis-aligned: every edge changes exactly one coordinate.
    for i in 0..4 {
        let (p, q) = (pts[i], pts[(i + 1) % 4]);
        let du = p.0 != q.0;
        let dv = p.1 != q.1;
        if du == dv {
            return Ok(None);
        }
    }
    let (mut ul, mut uh, mut vl, mut vh) = (pts[0].0, pts[0].0, pts[0].1, pts[0].1);
    for p in &pts {
        ul = ul.min(p.0);
        uh = uh.max(p.0);
        vl = vl.min(p.1);
        vh = vh.max(p.1);
    }
    let span = T::from_f64(uh) - T::from_f64(ul);
    match decide(
        "chart_region_cyl_wrap",
        Margin::levered(span - T::tau(), radius),
        band,
    ) {
        Ok(Sign::Zero) => Ok(Some((T::from_f64(vl), T::from_f64(vh)))),
        Ok(_) => Ok(None),
        Err(diag) => Err(ChartRegionError::Escalated(diag)),
    }
}

/// The band arm's decision (doc at [`cylinder_pair_overlap`]): axial
/// interval overlap of two full-period bands, then the mean-width
/// area claim with the module's standard conservative ring deduction.
fn band_overlap<T: Decide + Bounds>(
    uv_a: &FaceUv<T>,
    uv_b: &FaceUv<T>,
    (a_lo, a_hi): (T, T),
    (b_lo, b_hi): (T, T),
    radius: T,
    band: Band,
) -> Result<ChartOverlap, ChartRegionError> {
    let h = a_hi.min(b_hi) - a_lo.max(b_lo);
    match decide("chart_region_cyl_band", Margin::of(h), band) {
        Ok(Sign::Positive) => {}
        Ok(Sign::Negative) => return Ok(ChartOverlap::Empty),
        Ok(Sign::Zero) => return Err(ChartRegionError::TouchingBoundary),
        Err(diag) => return Err(ChartRegionError::Escalated(diag)),
    }
    // The overlap band, metred: width τ·r, height h (both faces are
    // full-period bands, so the quotient overlap IS the band — exact;
    // rings deduct conservatively as in `overlap_of_regions`, their
    // measures being isometry invariants of the metred chart).
    let circumference = T::tau() * radius;
    let two = T::one() + T::one();
    let mut net_2a = two * circumference * h;
    let mut tot_p = two * (circumference + h);
    for rings in [&uv_a.rings, &uv_b.rings] {
        for poly in rings.iter() {
            let scaled: Vec<Point2<T>> = poly
                .iter()
                .map(|p| Point2::new(p.x * radius, p.y))
                .collect();
            let (r2a, rp) = loop_measures(&scaled);
            net_2a = net_2a - r2a.abs();
            tot_p = tot_p + rp;
        }
    }
    let area_margin = Margin::over_lever(net_2a, tot_p);
    match decide("chart_region_cyl_band_area", area_margin, band) {
        Ok(Sign::Positive) => Ok(ChartOverlap::PositiveArea),
        Ok(_) => Err(ChartRegionError::Escalated(definite_diag(
            band,
            "chart_region_cyl_band_area",
            area_margin,
        ))),
        Err(diag) => Err(ChartRegionError::Escalated(diag)),
    }
}

/// **The interior-witness rung** — `true` when the candidate schedule
/// exhibits a point strictly inside BOTH faces, run through the
/// census's own `contfp` on the shared carrier.
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
/// # The schedule, and why it is two stages
///
/// Every `contfp` verdict but `In` — a boundary coincidence, an
/// exterior point, an in-band containment, an exhausted ray schedule —
/// means THIS candidate proves nothing and the walk moves on; the
/// rung's whole output is a proof or its absence. A candidate is never
/// counted from one face alone: `contfp` reads each face's own rings,
/// so a point in a hole of either is `Out` of that face.
///
/// Because every candidate is CERTIFIED at the point of use, the
/// schedule that proposes them is a hint and nothing else. No certified
/// claim rests on how a candidate was chosen, which is what lets stage
/// 2 below compute in plain `f64` off the trims' nominal coordinates:
/// a bad hint costs a wasted `contfp` pair, never a wrong answer.
///
/// **Stage 1 — the trims' own landmarks** ([`candidate_points`]): each
/// outer trim's vertex centroid then its ear midpoints, face A's trim
/// then face B's, in cycle order. A FLUSH seat — the configuration this
/// rung exists for — certifies on the very first of these, and stage 2
/// is not built at all in that case.
///
/// **Stage 2 — the pair's own arrangement**
/// ([`decomposition_witness`]): the cell centres of the vertical
/// decomposition of both trims' boundaries. This is the completion: a
/// fixed handful of landmarks is not a search, and legal seats of one
/// class bifurcated on where those landmarks happened to fall (issue
/// 1435 — a ~7.5e-3 m² overlap, seven orders above ε, missed by all
/// fourteen stage-1 candidates on a non-convex trim while a
/// geometrically equivalent seat certified on its first).
///
/// # Why the candidates are not seeded from an exact clip
///
/// The obvious seeding — clip the two regions and take a point per
/// piece — is unavailable HERE, by this rung's own entry condition:
/// the rung runs only on a [`ChartRegionError::TouchingBoundary`] out
/// of [`overlap_of_regions`], and there are exactly THREE places that
/// answer can come from. The clip is unavailable at all three, though
/// not for one reason:
///
/// - [`proper_crossings`]' collinear-overlap refusal (the boundaries
///   share a span) and its Zero-span refusal (they meet AT a vertex):
///   the clip has been asked and has DECLINED. It cannot seed what it
///   just refused to compute.
/// - [`overlap_of_regions`]' `(None, _) | (_, None)` arm — the
///   defense-in-depth arm, reached at ZERO proper crossings when
///   neither polygon has a definite vertex verdict against the other.
///   Here the clip has not declined; it has no INPUT. The walk in
///   [`intersection_pieces`] is driven entirely by crossings and
///   refuses an empty set outright, so with none there is nothing to
///   walk and no piece to take a point from. Equally unavailable, by
///   absence rather than by refusal.
///
/// What stage 2 keeps of the idea is the clip's COMBINATORICS — the
/// vertex and edge-crossing abscissae — taken as an uncertified hint
/// and verified pointwise instead, which needs no crossing to be
/// certifiable and no piece to be built. Per-piece centroids would not
/// have sufficed even where the clip does run: a non-convex piece's
/// centroid need not lie in the piece.
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
    let strictly_inside_both = |x: T, y: T| -> bool {
        let q = origin + u_ref * x + v_ref * y;
        inside(body_a, face_a, normal, q) && inside(body_b, face_b, normal_b, q)
    };
    // Stage 1: the trims' own landmarks. A flush seat lands on the
    // first candidate and stage 2 is never built.
    for poly in [&uv_a.outer, &uv_b.outer] {
        for c in candidate_points(poly) {
            if strictly_inside_both(c.x, c.y) {
                return true;
            }
        }
    }
    // Stage 2: the pair's own arrangement.
    decomposition_witness(uv_a, uv_b, |x, y| {
        strictly_inside_both(T::from_f64(x), T::from_f64(y))
    })
}

/// The most cell centres [`decomposition_witness`] probes before it
/// declines, and the most boundary segments it will decompose.
///
/// Both are the honest half of "complete or honest": inside them the
/// schedule is complete in the sense argued at
/// [`decomposition_witness`], and outside them it declines and the
/// region walk's own typed refusal stands. A trim pair with more than
/// `segments` boundary segments is refused rather than half-searched
/// because the decomposition is quadratic in that count.
///
/// # What these limits cost, stated rather than implied
///
/// **Neither is out of reach, and `cells` is reachable INSIDE
/// `segments`.** A pair of trims with ~50 stacked runs against one
/// tilted crosser exceeds 4096 cells at ~125 segments — under the
/// segment cap, so "large enough never to bind" is not a claim this
/// constant can make. What it is is a bound on the work, chosen so
/// that the trim pairs this rung is actually reached with (a few dozen
/// segments; the seats in the suite carry twelve) search exhaustively.
///
/// **A budget decline is indistinguishable from a thin-overlap
/// decline.** Both return `false` here, both leave the region walk's
/// [`ChartRegionError::TouchingBoundary`] standing, and both surface to
/// a caller as the same `CensusUnsupported` on the same face — so a
/// reader cannot tell "the overlap is too thin to certify at this ε"
/// from "the search was cut off". That is the honest gap in the rung's
/// three-outcome contract, and closing it means giving the decline a
/// TYPE. It is not closed here: the decline's type would have to reach
/// this module's error enum, whose exhaustive matches live in
/// `census.rs`, outside this unit's fence. Scheduled as issue 1478.
const WITNESS_BUDGET: WitnessBudget = WitnessBudget {
    segments: 128,
    cells: 4096,
};

struct WitnessBudget {
    segments: usize,
    cells: usize,
}

/// **The completion of the witness schedule**: the cell centres of the
/// two trims' vertical decomposition, in fixed order, each offered to
/// `probe` until one is certified.
///
/// # Why this is a complete schedule and not a bigger handful
///
/// Let `P` be the region the rung is asking about — face A's trim minus
/// its holes, intersected with face B's the same way — and let `X` be
/// the abscissae of every vertex of both boundaries together with every
/// boundary edge-pair crossing. Every vertex of `P` is one of those
/// points, so no vertex of `P` has an abscissa strictly inside a slab
/// `(x_i, x_i+1)` of consecutive members of `X`. Hence every boundary
/// edge that meets such a slab crosses it FULLY, and `P ∩ slab` is a
/// union of trapezoids each spanning the slab's whole width. The
/// slab's midline therefore meets each of them in a vertical segment of
/// positive height whose two ends are CONSECUTIVE members of the
/// midline's sorted crossing list — nothing may lie between them,
/// since anything that did would be a boundary crossing the trapezoid's
/// interior. So if `P` has interior at all, some cell centre this
/// function offers lies in it. That is the property a fixed handful of
/// landmarks does not have and cannot be given.
///
/// **The argument is exact-arithmetic; the arrangement is `f64`.** So
/// what is delivered is completeness UP TO `f64` ROUNDING OF THE
/// ARRANGEMENT: a meeting abscissa computed here can land an ulp off
/// the true one, and in the sub-ulp neighbourhood where that matters a
/// vertex of `P` can fall a hair inside a slab instead of on its
/// boundary, which is precisely the hypothesis the trapezoid step
/// needs. The failure is one-directional and that is why the rounding
/// is tolerated rather than removed: a slab whose interior holds a
/// vertex still yields cell centres, they are simply no longer
/// guaranteed to cover every component — so the rung can DECLINE where
/// an exact arrangement would have certified. It cannot certify
/// anything false, because no candidate is believed until `contfp`
/// certifies it on the lane's own arithmetic.
///
/// # What the argument does NOT claim
///
/// - **Positive area is not positive MARGIN.** The centre is the
///   deepest point of its cell in the two decomposition directions, not
///   of `P`; a real but slivered overlap can put every cell centre
///   within ε of a boundary, where `contfp` answers in-band rather than
///   `In` and the rung declines. That decline is the honest one — the
///   overlap is not certifiable at this ε — and it is the same posture
///   [`overlap_of_regions`] takes on a thin region.
/// - **The hint is nominal.** Candidates are built from each
///   coordinate's bracket midpoint, so on an enclosure lane the
///   decomposition describes the nominal trims rather than every member
///   of the enclosure. It cannot mislead: the certificate is `contfp`'s
///   and is taken on the lane's own arithmetic.
/// - **The frame still rotates.** The decomposition is a function of
///   the unordered PAIR of trims — swapping the arguments permutes
///   nothing in `X` — but not of the pair alone: it is taken along the
///   chart's own x-axis, and a declared pair's chart is the FIRST
///   face's plane. Certified answers are frame-invariant
///   ([`world_carrier`]'s lemma); which candidate certifies them is
///   not, and never was.
fn decomposition_witness<T: Decide + Bounds>(
    uv_a: &FaceUv<T>,
    uv_b: &FaceUv<T>,
    mut probe: impl FnMut(f64, f64) -> bool,
) -> bool {
    // Decline cause 1: a coordinate the arrangement cannot describe.
    // FAIL-CLOSED rather than edge-dropping — see `boundary_segments`.
    let Some(segments) = boundary_segments(uv_a, uv_b) else {
        return false;
    };
    // Decline cause 2: no arrangement to build. One segment bounds no
    // cell, and the rung's own extraction refuses loops under three
    // vertices, so this is the empty-trim guard rather than a budget.
    // Decline cause 3: the segment budget.
    if segments.len() < 2 || segments.len() > WITNESS_BUDGET.segments {
        return false;
    }
    let mut spent = 0usize;
    let abscissae = event_abscissae(&segments);
    for slab in abscissae.windows(2) {
        let x = midpoint(slab[0], slab[1]);
        // An adjacent-float slab has no interior to sample.
        if !(x > slab[0] && x < slab[1]) {
            continue;
        }
        let mut ys: Vec<f64> = segments.iter().filter_map(|s| s.at_abscissa(x)).collect();
        ys.sort_by(f64::total_cmp);
        for cell in ys.windows(2) {
            let y = midpoint(cell[0], cell[1]);
            if !(y > cell[0] && y < cell[1]) {
                continue;
            }
            spent += 1;
            if spent > WITNESS_BUDGET.cells {
                return false;
            }
            if probe(x, y) {
                return true;
            }
        }
    }
    false
}

/// One boundary segment of one trim, in nominal chart coordinates.
struct NominalSeg {
    p: [f64; 2],
    q: [f64; 2],
}

impl NominalSeg {
    /// Where this segment crosses the vertical line at `x`, when it
    /// crosses it strictly between the endpoints. A segment parallel to
    /// the line never does; nor does one whose endpoint sits on it,
    /// since every endpoint abscissa is a slab BOUNDARY and `x` is
    /// strictly interior to a slab.
    fn at_abscissa(&self, x: f64) -> Option<f64> {
        let run = self.q[0] - self.p[0];
        if run == 0.0 {
            return None;
        }
        let t = (x - self.p[0]) / run;
        if !(t > 0.0 && t < 1.0) {
            return None;
        }
        let y = self.p[1] + t * (self.q[1] - self.p[1]);
        y.is_finite().then_some(y)
    }

    /// The abscissa where this segment meets `other`, endpoints
    /// included — the remaining vertices of the intersection region.
    /// Parallel pairs (including collinear ones, whose shared span's
    /// ends are already endpoint abscissae) contribute nothing.
    ///
    /// # Two spellings of one determinant, on purpose
    ///
    /// `perp_dot(r, s)` and the two advance fractions below are the same
    /// quantities [`proper_crossings`] computes, and this is deliberately
    /// NOT a call into it. The difference is the whole hint/certificate
    /// split: there, every one of them is a DECIDED row
    /// (`chart_region_parallel`, `chart_region_cross_span`) carrying a
    /// named margin and a typed refusal, and the answer is a certificate;
    /// here they are bare `f64` with `== 0.0` and `0..=1` tests, and the
    /// answer is only a place to LOOK. Routing this through the certified
    /// rows would refuse exactly where the rung is reached (that refusal
    /// is its entry condition) and would put a decided row behind an
    /// answer nothing certifies. If either spelling changes, they are not
    /// required to agree: the certified one is the contract, this one is
    /// allowed to be wrong and is checked by `contfp` before anything
    /// rests on it.
    fn meeting_abscissa(&self, other: &Self) -> Option<f64> {
        let r = [self.q[0] - self.p[0], self.q[1] - self.p[1]];
        let s = [other.q[0] - other.p[0], other.q[1] - other.p[1]];
        let denom = r[0] * s[1] - r[1] * s[0];
        if denom == 0.0 || !denom.is_finite() {
            return None;
        }
        let d = [other.p[0] - self.p[0], other.p[1] - self.p[1]];
        let t = (d[0] * s[1] - d[1] * s[0]) / denom;
        let u = (d[0] * r[1] - d[1] * r[0]) / denom;
        if !(0.0..=1.0).contains(&t) || !(0.0..=1.0).contains(&u) {
            return None;
        }
        let x = self.p[0] + t * r[0];
        x.is_finite().then_some(x)
    }
}

/// Every boundary segment of both trims — outer then rings, face A then
/// face B, cycle order (fixed, D9). Holes are in: they bound the region
/// the rung is asking about exactly as the outers do, and a
/// decomposition that ignored them would propose cell centres inside a
/// hole (harmless, `contfp` reads rings and answers `Out`) and, worse,
/// merge two genuine cells across a hole's edge and propose neither.
///
/// `None` when any vertex's bracket has no finite midpoint. **That is
/// fail-closed on purpose**: skipping the offending edge and carrying on
/// would silently delete a boundary from the arrangement, which merges
/// two cells that the deleted edge separated and can therefore lose the
/// only witness — completeness broken with nothing said. Declining the
/// whole search says it instead, at the cost of a rescue this rung was
/// never going to make. (No committed seat reaches it: a chart
/// coordinate that is not finite means `extract_face_uv` handed back a
/// poisoned trim, and its own gates refuse first. It is guarded rather
/// than asserted unreachable because that argument is about a caller,
/// and this function should not need one.)
fn boundary_segments<T: Decide + Bounds>(
    uv_a: &FaceUv<T>,
    uv_b: &FaceUv<T>,
) -> Option<Vec<NominalSeg>> {
    let mut out = Vec::new();
    for uv in [uv_a, uv_b] {
        for poly in core::iter::once(&uv.outer).chain(uv.rings.iter()) {
            if poly.len() < 3 {
                continue;
            }
            let nominal = |p: &Point2<T>| {
                let c = [midpoint(p.x.lo(), p.x.hi()), midpoint(p.y.lo(), p.y.hi())];
                (c[0].is_finite() && c[1].is_finite()).then_some(c)
            };
            for i in 0..poly.len() {
                out.push(NominalSeg {
                    p: nominal(&poly[i])?,
                    q: nominal(&poly[(i + 1) % poly.len()])?,
                });
            }
        }
    }
    Some(out)
}

/// The decomposition's slab boundaries: every segment endpoint's
/// abscissa and every segment-pair meeting's, ascending and deduplicated
/// (`total_cmp`, so the order is total and fixed whatever the values).
fn event_abscissae(segments: &[NominalSeg]) -> Vec<f64> {
    let mut xs = Vec::with_capacity(2 * segments.len());
    for s in segments {
        xs.push(s.p[0]);
        xs.push(s.q[0]);
    }
    for i in 0..segments.len() {
        for j in (i + 1)..segments.len() {
            if let Some(x) = segments[i].meeting_abscissa(&segments[j]) {
                xs.push(x);
            }
        }
    }
    xs.sort_by(f64::total_cmp);
    xs.dedup();
    xs
}

/// A point between two `f64`s — `a + (b − a)/2` rather than
/// `(a + b)/2`.
///
/// # What it guarantees, which is less than "the midpoint"
///
/// It is NOT exact: `midpoint(1.0, 1e16)` is an ulp off the true middle,
/// because `b − a` rounds. It does not remove overflow either, it trades
/// one family for another: `(a + b)/2` overflows on two same-sign
/// halves of the range, this form overflows on OPPOSITE ends —
/// `midpoint(-1e308, 1e308)` is `inf`. What it does guarantee is what
/// both callers need and nothing more: for finite `a < b` the result is
/// finite-or-`inf` and lies in `[a, b]`, never outside the bracket.
///
/// Which is why **both call sites re-test the result** rather than
/// trusting it. [`decomposition_witness`] takes a slab or cell centre
/// only when it is STRICTLY between the two ends, so an ulp of drift or
/// a collapse onto an endpoint drops that cell instead of proposing a
/// point outside it; and a non-finite coordinate is rejected at
/// [`boundary_segments`]. The rounding therefore reaches the answer only
/// as a missed candidate, never as a bad one.
///
/// # Siblings, not one utility
///
/// Two other bracket-midpoint spellings exist — `geom_brep`'s
/// `props/quad.rs` `mid` and `topo`'s `props.rs` `mid_pad` — and this is
/// deliberately not unified with either. They compute different
/// quantities for different jobs (`mid_pad` pads, quad's `mid` is a
/// quadrature abscissa on a certified path); this one is an uncertified
/// hint that its own callers re-validate. Merging three formulas whose
/// only shared property is the word "midpoint" would give one of them
/// the wrong rounding contract.
fn midpoint(a: f64, b: f64) -> f64 {
    a + 0.5 * (b - a)
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
        // The general curve-in-UV arm (U2): a NURBS chart image with
        // no straightness fact of any kind — the same refusal as the
        // fitted arm, and separate because the inventory names the
        // class it refuses.
        Pcurve::General(_) => Err("General curve-in-UV image is not a straight segment"),
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

/// **Certified LOWER chart lever arms** (metres per chart unit) over
/// the `v`-window `[v_lo, v_hi]` the pair's own loops live in, or the
/// typed refusal.
///
/// # Why the arms must be inf-side here, and what that buys
///
/// The machinery below scales both faces' chart polygons by these
/// arms and reads a metre margin off the result. Scaling by a
/// **lower** bound makes the scaled polygon a certified metric
/// CONTRACTION of the model image: every chart displacement is at
/// least this long in metres, so the model region's area is at least
/// the scaled polygon's. Every error is therefore in the refusing
/// direction — for `PositiveArea` a shrunk region has a smaller mean
/// width and misses the band sooner, and for `Empty` a shrunk
/// separation is harder to call definite. Reading
/// `geom_brep::chart_stretch_sup` here would invert exactly that and
/// certify a model-space sliver as definitely positive, which is why
/// the two bounds are separate functions with separate names.
///
/// # The arms, per chart kind
///
/// `Plane` and `Cylinder` are EXACT constants, not bounds: a plane's
/// coordinates ARE metres, and a cylinder's azimuth levers by exactly
/// `r` at every latitude. Nothing about them is certified here and
/// nothing about them moves.
///
/// The rest are genuine bounds and every one of them is gated by the
/// `chart_region_arm_inf` row before it is used, so a collapsed arm
/// refuses `ArmUnbounded` and an in-band one escalates:
///
/// - **Sphere** `(r·inf|cos v|, r)`. The chart is orthogonal, so the
///   per-axis infs ARE the metric bound. `cos` is even and decreasing
///   in `|v|`, so the window's inf is `cos` of its largest `|v|`; a
///   window reaching `π/2` reads a non-positive number and refuses,
///   which is the pole honestly having no azimuth extent. The polar
///   arm is exactly `r` at every latitude.
/// - **Torus** `(R − r, r)`. `|R + r·cos v| ≥ R − r` everywhere, so
///   this one needs no window at all; a degenerate ring (`R ≤ r`)
///   refuses. The window would sharpen it and is deliberately not
///   used — a window-free bound cannot be wrong about a branch.
/// - **Cone** `(v_inf·sin α, 1)` with `v_inf` the window's smallest
///   `|v|`, which is zero exactly when the window straddles the apex.
///   The cone's `v` is a length along the ruling, so its arm is 1.
/// - **Spline charts** take `geom_brep::chart_stretch_inf`'s
///   derivative-net reading and finish the assembly that door
///   documents and deliberately does not do: gate the two per-axis
///   infs definitely positive FIRST (which is what makes the
///   assembly's divisions well-conditioned), then discount both by
///   the skew factor `ρ` — the smallest singular value of the
///   Jacobian in the chart those infs normalize. `ρ` is exactly 1 on
///   an orthogonal chart of constant stretch, so the arms are the
///   per-axis infs verbatim there, ANISOTROPY INCLUDED; it collapses
///   to zero on a chart whose two derivative directions can align. A
///   zero-crossing net arrives with a zero per-axis inf and refuses
///   at the first gate, and a folded chart with healthy per-axis
///   infs refuses at the discount.
fn certified_arms<T: Decide + Bounds>(
    surface: &Surface<T>,
    v_lo: T,
    v_hi: T,
    band: Band,
) -> Result<(T, T), ChartRegionError> {
    // The arm gate: a lever arm is a metre RATE, gated as a length
    // (the collapsed-arm idiom). Definite-positive walks on; a zero
    // or negative arm is the chart having no certified extent in that
    // channel; an in-band arm escalates rather than guessing.
    let gate = |arm: T, chart: &'static str| -> Result<T, ChartRegionError> {
        // **Structure first (C6): an arm with no positive FLOOR is
        // not a bound at all.** At `f64` the bracket is the value and
        // this is exactly the `Zero`/`Negative` arm below. Under the
        // interval scalar it is the case that arm carries — a folded
        // net's `min_dot/|c|` quotient divides by a bracket straddling
        // zero and comes back as the whole admissible range, whose
        // floor is 0 — and reading the floor keeps that answer TYPED
        // (`ArmUnbounded`, the chart honestly has no bound) instead of
        // laundering it into an escalation that reads as "undecided
        // measurement". Only the floor is ever leaned on downstream,
        // so this is the same question the row asks, asked first.
        if arm.lo() <= 0.0 {
            return Err(ChartRegionError::ArmUnbounded { chart });
        }
        match decide("chart_region_arm_inf", Margin::of(arm), band) {
            Ok(Sign::Positive) => Ok(arm),
            Ok(Sign::Zero | Sign::Negative) => Err(ChartRegionError::ArmUnbounded { chart }),
            Err(diag) => Err(ChartRegionError::Escalated(diag)),
        }
    };
    match *surface {
        // A plane chart's coordinates ARE metres (unit u_ref frame).
        Surface::Plane { .. } => Ok((T::one(), T::one())),
        // Azimuth radians lever by exactly r everywhere; v is metres.
        Surface::Cylinder { radius, .. } => Ok((radius, T::one())),
        Surface::Sphere { radius, .. } => {
            let v_abs = v_lo.abs().max(v_hi.abs());
            // **`cos` is monotone only on `[0, π/2]`, and the window
            // is not required to live there.** `v` is shifted by whole
            // periods of `τ` by `shift_polar_branch`, so a stored
            // branch can carry `|v| > π/2`, and past `π` the cosine
            // comes back POSITIVE — `cos 6.5 ≈ 0.977` — while the
            // window it describes has swept the pole and the true inf
            // is 0. Reading the bracket's top (C6 structure; the
            // enclosure's sup under the interval scalar) and refusing
            // outside the monotone range is the whole guard: within
            // it, `cos v_abs ≤ cos|v|` for every `v` in the window,
            // which is the claim the arm makes.
            if v_abs.hi() > core::f64::consts::FRAC_PI_2 || v_abs.hi().is_nan() {
                return Err(ChartRegionError::ArmUnbounded { chart: "sphere" });
            }
            Ok((gate(radius * v_abs.cos(), "sphere")?, radius))
        }
        Surface::Torus {
            major_radius,
            minor_radius,
            ..
        } => Ok((
            gate(major_radius - minor_radius, "torus")?,
            gate(minor_radius, "torus")?,
        )),
        Surface::Cone { half_angle, .. } => {
            // Branch-free `min |v|` over the window, zero exactly when
            // the window straddles the apex.
            let v_inf = v_lo.max(T::zero() - v_hi).max(T::zero());
            Ok((gate(v_inf * half_angle.sin(), "cone")?, T::one()))
        }
        Surface::Nurbs(_) | Surface::Approx(_) => {
            let chart = if matches!(*surface, Surface::Nurbs(_)) {
                "NURBS"
            } else {
                "approximating surface"
            };
            let inf = geom_brep::chart_stretch_inf(surface);
            // Gate the per-axis infs FIRST: they are the assembly's
            // divisors, and only a definitely-positive one makes the
            // divisions below well-conditioned.
            let (inf_u, inf_v) = (gate(inf.inf_u, chart)?, gate(inf.inf_v, chart)?);
            // The skew discount `ρ` of `chart_stretch_inf`'s
            // assembly: the smallest singular value of the Jacobian
            // in the chart normalized by those infs, from a sup trace
            // and an inf determinant (the conservative corner, since
            // `λ_min` falls with the trace and rises with the
            // determinant). `ρ` is 1 on an orthogonal chart of
            // constant stretch, so the arms are then the per-axis
            // infs verbatim; it is 0 on a chart whose derivative
            // directions can align, which is the skew case the
            // per-axis pair alone cannot see.
            let trace_sup = (inf.sup_u / inf_u).powi(2) + (inf.sup_v / inf_v).powi(2);
            let det_inf = (inf.area_inf / (inf_u * inf_v)).powi(2);
            let root = (trace_sup.powi(2) - det_inf * T::from_f64(4.0))
                .max(T::zero())
                .sqrt();
            // `2D/(T + √(T²−4D))` — the cancellation-free spelling of
            // `(T − √(T²−4D))/2`.
            // The `min(1)` is a SAFETY cap, not a tightening: `T` and
            // `D` come from different points of the chart, so their
            // corner can nominally exceed the true `λ_min ≤ 1` and an
            // arm above its own per-axis inf would be unsound. It is
            // also what makes `arm ≤ inf_u` near-tautological, so no
            // test may use that inequality as evidence of anything —
            // the swap row pins the arm's DERIVED VALUE for exactly
            // this reason.
            let rho = (det_inf * T::from_f64(2.0) / (trace_sup + root))
                .sqrt()
                .min(T::one());
            Ok((gate(inf_u * rho, chart)?, gate(inf_v * rho, chart)?))
        }
    }
}

/// The `v` reach of a pair's extracted loops — the window the
/// window-dependent arms of [`certified_arms`] are read over. Every
/// vertex of both faces is inside it by construction, so an arm that
/// lower-bounds the stretch across it lower-bounds it everywhere
/// either region lives.
fn v_window<T: Decide + Bounds>(a: &FaceUv<T>, b: &FaceUv<T>) -> Option<(T, T)> {
    let mut reach: Option<(T, T)> = None;
    for face in [a, b] {
        for poly in core::iter::once(&face.outer).chain(&face.rings) {
            for p in poly {
                reach = Some(match reach {
                    None => (p.y, p.y),
                    Some((lo, hi)) => (lo.min(p.y), hi.max(p.y)),
                });
            }
        }
    }
    reach
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
/// - `chart_region_parallel`: the transversality determinant over the
///   SHORTER edge's length, `over_lever(perp_dot(r, s), min(|r|, |s|))`
///   — the perpendicular height the LONGER edge makes across the
///   other's line (m). Zero ⇒ the parallel lane below. The lever is
///   the shorter length because that is the union-max of the two
///   descriptions' candidate margins and therefore argument-order
///   symmetric; see the invariant comment at the row itself.
/// - `chart_region_cross_span`: each of the four advance clearances
///   `t·|r|`, `(1−t)·|r|`, `u·|s|`, `(1−u)·|s|` — the crossing
///   point's distance from a segment endpoint along its segment
///   (dimensionless fraction levered by the segment length). All four
///   definitely positive ⇒ a proper crossing; any definitely negative
///   ⇒ the lines cross off-segment (no crossing); an exact Zero is a
///   boundary-touch configuration ⇒ typed refusal.
/// - `chart_region_collinear_offset` (parallel lane): how far apart the
///   two lines are, as the MAX of the two perpendicular offsets —
///   `q`'s from `r`'s line and `p`'s from `s`'s (m), again the
///   union-max form. Definite ⇒ parallel disjoint lines, no crossing;
///   Zero ⇒ collinear, span check next.
/// - `chart_region_collinear_overlap` (collinear lane): the shared
///   span length along the common line (m, an interval-overlap
///   difference of metre projections). Positive ⇒ the boundaries
///   share a segment ⇒ typed touching refusal; Zero/Negative ⇒
///   disjoint spans, no crossing.
///
/// The division by the determinant in `t`/`u` is certified: the
/// parallel row's definite sign excludes zero from the enclosure.
///
/// # Argument-order symmetry (#1063)
///
/// Every row here answers the same for `(a, b)` and `(b, a)`:
///
/// - `chart_region_parallel` and `chart_region_collinear_offset` are
///   union-max forms, symmetric by construction (see their rows);
/// - `chart_region_cross_span` already was: swapping the arguments
///   maps `(t, |r|)` to `(u, |s|)` and back, so the four clearances
///   are the same MULTISET and the `Negative`/`Zero`/in-band reads
///   over them are unchanged;
/// - `chart_region_cross_order` already was too, and is the pattern
///   the parallel row should have followed from the start: it decides
///   BOTH sides' orders, each levered by its own edge, and requires
///   both — a conjunction over an unordered pair;
/// - `chart_region_collinear_overlap` measures the two segments'
///   shared span, which is the same interval intersection either way.
///   The two frames project it onto `r̂` and onto `ŝ`, so the lengths
///   differ by a factor `cos θ` in the residual angle — and that angle
///   is already bounded by the parallel row at `max(|r|, |s|)·sin θ ≤
///   ε`, so the discrepancy is at most `overlap·θ²/2 ≤ ε²/(2·max(|r|,
///   |s|))`. At the tightest ε the matrix runs that is below `1e-24 /
///   |r|`, which no configuration can carry across a band edge. Left
///   in that form deliberately: an exactly-symmetric projection would
///   be new machinery bought for a residue twelve orders under the
///   one it would protect.
///
/// What is NOT claimed, and is the module's standing posture: the ray
/// schedule in [`crate::ray_parity`] is fixed in CHART coordinates, so
/// which configurations refuse rather than decide still rotates with
/// the frame.
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
            // ARGUMENT-ORDER INVARIANT (#1063 fix pass): both levers
            // here are UNION-MAX forms, so the margin is a `max` over a
            // multiset that swapping `(A, B)` for `(B, A)` only
            // permutes. `min(|r|, |s|)` under an `over_lever` IS that
            // max — `|denom|/min(|r|, |s|) = max(|denom|/|r|,
            // |denom|/|s|)` — and `min`/`max` are commutative, so the
            // margin is bit-identical in both orders. A one-sided
            // `|r|` lever is what made the SAME pair certify one way
            // and escalate the other.
            let lever = r.norm().min(s.norm());
            let d_sign = decide(
                "chart_region_parallel",
                Margin::over_lever(denom, lever),
                band,
            )
            .map_err(escalate)?;
            if d_sign == Sign::Zero {
                // Parallel lane. Same discipline: the two candidate
                // offsets are `q`'s distance from `r`'s line and `p`'s
                // from `s`'s, and the row takes their max. Only the
                // MAGNITUDE is read below (Zero versus anything
                // definite), so a max of magnitudes says exactly what
                // the row asks — "are these two lines definitely
                // apart" — with neither description privileged.
                //
                // Dimensional argument, the `over_lever` door's own:
                // `perp_dot(r, q − p) / |r|` is a determinant (m2) over
                // the length that levers it, i.e. `q`'s perpendicular
                // distance from `r`'s line, in metres. Both candidates
                // are that same length, so their max is a length and
                // `Margin::of` is its door (the `carrier_agreement`
                // precedent).
                let pq = Vec2::new(T::zero() - qp.x, T::zero() - qp.y);
                let off_a = (r.perp_dot(qp) / r.norm()).abs();
                let off_b = (s.perp_dot(pq) / s.norm()).abs();
                match decide(
                    "chart_region_collinear_offset",
                    Margin::of(off_a.max(off_b)),
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
            // ARGUMENT-ORDER INVARIANT (#1063 fix pass): BOTH relations
            // are read before any verdict. Asking one and refusing on
            // its `None` before the other was ever consulted made the
            // refusal depend on which face the caller named first — the
            // same one-sided shape as the crossing rows' lever, in the
            // arm that decides `Empty`.
            let ab = polygon_relation(&a.outer, &b.outer, band)?;
            let ba = polygon_relation(&b.outer, &a.outer, band)?;
            match (ab, ba) {
                // Containment either way. Both arms hold only for
                // coincident regions, where the two measures agree, so
                // the precedence between them is not a choice.
                (Some(PolyContainment::In), _) => (a.outer_2a, a.outer_p), // A ⊆ B
                (_, Some(PolyContainment::In)) => (b.outer_2a, b.outer_p), // B ⊆ A
                // Definitely disjoint, said by both descriptions.
                (Some(_), Some(_)) => return Ok(ChartOverlap::Empty),
                // Defense-in-depth (union fix U3), stated as the
                // invariant it is: a polygon with no definite vertex
                // against the other, at ZERO proper crossings, cannot
                // occur without a `chart_region_cross_span` Zero having
                // refused first (three adversarial witness
                // constructions all hit TouchingBoundary at the
                // crossing rows). If the arm is ever reached, the pair
                // is a touching configuration — never Empty.
                (None, _) | (_, None) => return Err(ChartRegionError::TouchingBoundary),
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
    // The mean width of the METRED CHART region. On an exact-arm
    // chart (plane, cylinder) that is the model region's mean width;
    // on a bounded-arm chart it is a reading on the contraction, and
    // only the AREA positivity transfers unconditionally. See
    // [`ChartOverlap::PositiveArea`] — the claim is written to what
    // this margin establishes, not past it.
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
    // MATE-5: the full-wrap band fast path's pieces (the public-door
    // rows live in `tests/mate5_cyl_eps_rung.rs` and the census's own
    // rows; no euler constructor mints a seam-doubled full-period wall
    // at test level, so the band arm is pinned here at its unit seams).
    // ------------------------------------------------------------------

    /// A full-period chart rectangle: seam at `u0`, span exactly τ.
    fn wrap_rect(u0: f64, v0: f64, v1: f64) -> Vec<Point2<f64>> {
        rect(u0, v0, u0 + core::f64::consts::TAU, v1)
    }

    #[test]
    fn mate5_wrap_band_reads_structure_and_meters_the_span() {
        let r = 2.0;
        // A full-period rectangle reads as a band.
        let got = wrap_band(&wrap_rect(0.3, 0.0, 1.0), r, band()).unwrap();
        assert_eq!(got, Some((0.0, 1.0)));
        // An arc rectangle (span < τ) is the general walk's ground.
        assert_eq!(
            wrap_band(&rect(0.0, 0.0, 3.0, 1.0), r, band()).unwrap(),
            None
        );
        // A five-vertex full-span polygon is NOT a band (a v-notch
        // breaks azimuth invariance) — structure, not span, decides.
        let tau = core::f64::consts::TAU;
        let notched = vec![
            pt(0.0, 0.0),
            pt(tau, 0.0),
            pt(tau, 1.0),
            pt(tau * 0.5, 0.4),
            pt(0.0, 1.0),
        ];
        assert_eq!(wrap_band(&notched, r, band()).unwrap(), None);
        // A near-full span INSIDE the band escalates typed — neither
        // silently a band nor silently a window.
        let sliver = rect(0.0, 0.0, tau - 2e-9, 1.0);
        assert!(matches!(
            wrap_band(&sliver, r, band()),
            Err(ChartRegionError::Escalated(_))
        ));
    }

    #[test]
    fn mate5_band_overlap_is_three_outcome_and_seam_blind() {
        let r = 1.5;
        let uv = |outer: Vec<Point2<f64>>, rings: Vec<Vec<Point2<f64>>>| FaceUv { outer, rings };
        // Misaligned seams, overlapping axial bands: the whole point
        // of the fast path — no fold, no seam gate, a certified
        // positive.
        let a = uv(wrap_rect(0.3, 0.0, 1.0), vec![]);
        let b = uv(wrap_rect(4.4, 0.4, 0.8), vec![]);
        let (ba, bb) = (
            wrap_band(&a.outer, r, band()).unwrap().unwrap(),
            wrap_band(&b.outer, r, band()).unwrap().unwrap(),
        );
        assert_eq!(
            band_overlap(&a, &b, ba, bb, r, band()).unwrap(),
            ChartOverlap::PositiveArea
        );
        // Definitely disjoint axial bands: the certified Empty — the
        // Refuted arm's currency.
        let c = uv(wrap_rect(4.4, 2.0, 2.5), vec![]);
        let bc = wrap_band(&c.outer, r, band()).unwrap().unwrap();
        assert_eq!(
            band_overlap(&a, &c, ba, bc, r, band()).unwrap(),
            ChartOverlap::Empty
        );
        // Rim-sharing bands: an exact-zero axial overlap is a touch,
        // not an area verdict in either direction.
        let d = uv(wrap_rect(4.4, 1.0, 1.5), vec![]);
        let bd = wrap_band(&d.outer, r, band()).unwrap().unwrap();
        assert!(matches!(
            band_overlap(&a, &d, ba, bd, r, band()),
            Err(ChartRegionError::TouchingBoundary)
        ));
        // A ring swallowing the thin overlap: the conservative
        // deduction drives the mean width out of the certifiable
        // range and the query escalates — never a false positive,
        // never an Empty.
        let thin = uv(wrap_rect(4.4, 0.999, 1.2), vec![]);
        let bt = wrap_band(&thin.outer, r, band()).unwrap().unwrap();
        let ringed = uv(wrap_rect(0.3, 0.0, 1.0), vec![rect(0.1, 0.99, 6.1, 1.0)]);
        assert!(matches!(
            band_overlap(&ringed, &thin, ba, bt, r, band()),
            Err(ChartRegionError::Escalated(_))
        ));
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
    use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec};

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
            description: EdgeDescriptionSpec::Intersection {
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
    fn the_arm_gate_certifies_an_inf_or_refuses_typed() {
        // Exact constant arms for plane and cylinder charts, which do
        // not move. Every other kind is a certified INF over the
        // pair's window — never a sup-arm pseudo-certificate — and
        // refuses typed exactly where that inf collapses.
        assert_eq!(
            certified_arms(&xy_plane(), -1.0, 1.0, band()).unwrap(),
            (1.0, 1.0)
        );
        assert_eq!(
            certified_arms(&cyl_surface(0.25), -1.0, 1.0, band()).unwrap(),
            (0.25, 1.0)
        );
        let sphere = Surface::Sphere {
            center: Point3::origin(),
            radius: 1.0,
            axis: Vec3::unit_z(),
            u_ref: Vec3::unit_x(),
        };
        // A window clear of the poles certifies `r·cos v_max`.
        let (arm_u, arm_v) = certified_arms(&sphere, -0.5, 0.5, band()).unwrap();
        assert!((arm_u - 0.5_f64.cos()).abs() < 1e-15, "got {arm_u}");
        assert_eq!(arm_v, 1.0);
        // A pole-reaching window, and a payload with no net at all,
        // still refuse typed.
        for (surface, v_hi, chart) in [
            (sphere, core::f64::consts::FRAC_PI_2, "sphere"),
            (Surface::nurbs_placeholder(), 0.5, "NURBS"),
        ] {
            match certified_arms(&surface, -0.5, v_hi, band()) {
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

/// The certified INF-arm rows: the per-kind lower stretch bounds the
/// positive-area lane meters by, and the direction each needs.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod inf_arms {
    use super::{ChartOverlap, ChartRegionError, FaceUv, certified_arms, overlap_of_uv, v_window};
    use crate::body::Body;
    use crate::entity::FaceKey;
    use crate::euler::FaceSurface;
    use geom::{NurbsSurface, Surface};
    use geom_core::spline::KnotVector;
    use geom_core::{Band, Point2, Point3, Vec3};
    use std::sync::Arc;

    fn band() -> Band {
        Band::new(1e-9, 1e-8).unwrap()
    }

    fn xy_plane() -> Surface<f64> {
        Surface::Plane {
            origin: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        }
    }

    fn rect(x0: f64, y0: f64, x1: f64, y1: f64) -> Vec<Point2<f64>> {
        vec![
            Point2::new(x0, y0),
            Point2::new(x1, y0),
            Point2::new(x1, y1),
            Point2::new(x0, y1),
        ]
    }

    /// A one-loop planar face: the pipeline needs a body and a face
    /// only for the loop keys its refusals name, so the cheapest
    /// well-formed sheet is the right fixture here — the CHART under
    /// test is the one passed alongside.
    fn sheet(body: &mut Body<f64>) -> FaceKey {
        use crate::euler::{MefSite, MevSite};
        use geom_brep::EdgeCurveSpec;
        use geom_core::Tol;
        let c = |x: f64, y: f64| Point3::new(x, y, 0.0);
        let (a, b, cc, d) = (c(0.0, 0.0), c(1.0, 0.0), c(1.0, 1.0), c(0.0, 1.0));
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
        let he = body
            .find_half_edge(seed.face, e_cd.vertex, e_bc.vertex)
            .unwrap();
        body.mef(
            MefSite::Chords {
                he1: he,
                he2: e_ab.he_plus,
            },
            EdgeCurveSpec::line_between(d, a),
            FaceSurface::New(xy_plane()),
            Tol::witness(),
        )
        .unwrap()
        .face
    }

    fn sphere(radius: f64) -> Surface<f64> {
        Surface::Sphere {
            center: Point3::new(0.0, 0.0, 0.0),
            radius,
            axis: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        }
    }

    fn cone(half_angle: f64) -> Surface<f64> {
        Surface::Cone {
            apex: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            half_angle,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        }
    }

    fn torus(major: f64, minor: f64) -> Surface<f64> {
        Surface::Torus {
            center: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            major_radius: major,
            minor_radius: minor,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        }
    }

    /// A bilinear chart on `[0, 1]²` mapping to the flat rectangle
    /// `[0, su] × [0, sv]`: `|S_u| = su`, `|S_v| = sv`, `S_u·S_v = 0`.
    fn flat_chart(su: f64, sv: f64) -> Surface<f64> {
        let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let control = vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.0, sv, 0.0),
            Point3::new(su, 0.0, 0.0),
            Point3::new(su, sv, 0.0),
        ];
        Surface::Nurbs(Arc::new(
            NurbsSurface::new(kv.clone(), kv, control, vec![1.0; 4]).unwrap(),
        ))
    }

    /// A chart whose `u`-derivative net CROSSES ZERO — a wall with a
    /// fold. The middle column doubles back, so `S_u` vanishes
    /// somewhere and the honest `inf |S_u|` is 0.
    fn folded_chart() -> Surface<f64> {
        let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
        let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        // Columns at x = 0, 2, 0: the second u-difference is the
        // negation of the first.
        let control = vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(2.0, 0.0, 0.0),
            Point3::new(2.0, 1.0, 0.0),
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.0, 1.0, 0.0),
        ];
        Surface::Nurbs(Arc::new(
            NurbsSurface::new(ku, kv, control, vec![1.0; 6]).unwrap(),
        ))
    }

    /// **A sphere window that used to refuse now certifies.** Over
    /// `|v| ≤ 0.3` the azimuth arm's inf is `r·cos 0.3`, and the
    /// polar arm is exactly `r`.
    #[test]
    fn a_sphere_window_certifies_its_inf_arms() {
        let (arm_u, arm_v) = certified_arms(&sphere(2.0), -0.3, 0.3, band()).unwrap();
        let expect_u = 2.0 * 0.3_f64.cos();
        assert!(
            (arm_u - expect_u).abs() < 1e-15,
            "azimuth inf over the window: {arm_u} vs {expect_u}"
        );
        assert_eq!(arm_v, 2.0);
        assert!(arm_u < 2.0, "the inf is strictly under the sup arm r");
    }

    /// **A sphere window past the monotone range of `cos` refuses**
    /// rather than reading the cosine's return to positive (review
    /// item 10). `cos 6.5 ≈ 0.977`, and a naive arm would certify a
    /// window that has swept the pole.
    #[test]
    fn a_sphere_window_beyond_the_monotone_range_refuses() {
        for (lo, hi) in [(0.0, 6.5), (-6.5, 0.0), (0.0, 3.5), (2.0, 2.5)] {
            assert!(
                matches!(
                    certified_arms(&sphere(2.0), lo, hi, band()),
                    Err(ChartRegionError::ArmUnbounded { chart: "sphere" })
                ),
                "window [{lo}, {hi}] must refuse"
            );
        }
        // And the monotone range itself still certifies, right up to
        // the edge where the arm collapses.
        assert!(certified_arms(&sphere(2.0), -1.5, 1.5, band()).is_ok());
    }

    /// A sphere window reaching a POLE has inf 0 and keeps refusing.
    #[test]
    fn a_pole_reaching_sphere_window_still_refuses() {
        let e = certified_arms(&sphere(2.0), -0.3, core::f64::consts::FRAC_PI_2, band());
        assert!(matches!(
            e,
            Err(ChartRegionError::ArmUnbounded { chart: "sphere" })
        ));
    }

    /// **A cone window clear of the apex certifies**; one straddling
    /// it does not.
    #[test]
    fn a_cone_window_certifies_off_the_apex_and_refuses_across_it() {
        let alpha = 0.4_f64;
        let (arm_u, arm_v) = certified_arms(&cone(alpha), 3.0, 5.0, band()).unwrap();
        assert!((arm_u - 3.0 * alpha.sin()).abs() < 1e-15);
        assert_eq!(arm_v, 1.0, "the cone's v is a length along the ruling");
        assert!(matches!(
            certified_arms(&cone(alpha), -1.0, 5.0, band()),
            Err(ChartRegionError::ArmUnbounded { chart: "cone" })
        ));
    }

    /// A ring torus certifies `R − r`; a degenerate one refuses.
    #[test]
    fn a_ring_torus_certifies_and_a_degenerate_one_refuses() {
        let (arm_u, arm_v) = certified_arms(&torus(5.0, 1.0), -3.0, 3.0, band()).unwrap();
        assert_eq!(arm_u, 4.0);
        assert_eq!(arm_v, 1.0);
        assert!(matches!(
            certified_arms(&torus(1.0, 1.0), -3.0, 3.0, band()),
            Err(ChartRegionError::ArmUnbounded { chart: "torus" })
        ));
    }

    /// **A NURBS chart that used to refuse now certifies**, at the
    /// net's own stretch: a `[0,1]² → 4 m × 1 m` flat chart is
    /// orthogonal, so the skew discount is exactly 1 and the arms are
    /// the per-axis infs.
    #[test]
    fn an_orthogonal_nurbs_chart_certifies_its_per_axis_infs() {
        let (arm_u, arm_v) = certified_arms(&flat_chart(4.0, 1.0), 0.0, 1.0, band()).unwrap();
        assert!((arm_u - 4.0).abs() < 1e-14, "inf |S_u| = 4, got {arm_u}");
        assert!((arm_v - 1.0).abs() < 1e-14, "inf |S_v| = 1, got {arm_v}");
    }

    /// **The zero-crossing net keeps refusing, typed.** Its
    /// `u`-derivative net brackets the origin, so `inf |S_u|` is
    /// honestly 0 and no positive bound is invented for it.
    #[test]
    fn a_folded_nurbs_net_still_refuses_typed() {
        assert!(matches!(
            certified_arms(&folded_chart(), 0.0, 1.0, band()),
            Err(ChartRegionError::ArmUnbounded { chart: "NURBS" })
        ));
    }

    /// **The swap row — the confusion this lane exists to prevent.**
    /// The inf and sup readings of one chart are different numbers on
    /// the same side of no claim: on a `[0,1]² → 4 m × 1 m` chart the
    /// sup pair is what the escape lane quotes and the inf pair is
    /// what the positive lane quotes; on a chart whose stretch VARIES
    /// they separate, and reading the sup where the inf belongs
    /// over-states the metred extent — the direction that certifies a
    /// sliver.
    #[test]
    fn the_sup_reading_over_states_where_the_inf_reading_is_owed() {
        // A chart whose u stretch runs from 0.5 (first span) to 8
        // (second span): sup 8, inf 0.5, a factor of 16 apart.
        let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
        let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let control = vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(0.25, 0.0, 0.0),
            Point3::new(0.25, 1.0, 0.0),
            Point3::new(4.25, 0.0, 0.0),
            Point3::new(4.25, 1.0, 0.0),
        ];
        let s: Surface<f64> = Surface::Nurbs(Arc::new(
            NurbsSurface::new(ku, kv, control, vec![1.0; 6]).unwrap(),
        ));
        let inf_u = geom_brep::chart_stretch_inf(&s).inf_u;
        let (sup_u, _) = geom_brep::chart_stretch_sup(&s);
        assert!((inf_u - 0.5).abs() < 1e-14, "inf |S_u| = 0.5, got {inf_u}");
        assert!((sup_u - 8.0).abs() < 1e-14, "sup |S_u| = 8, got {sup_u}");
        assert!(
            sup_u > inf_u * 15.0,
            "the two readings are 16x apart on one chart: quoting the sup \
             where the inf is owed over-states a chart-space extent by that \
             factor, and a 16x-inflated region certifies slivers"
        );
        // **And the assembled arm is pinned to its DERIVED value**,
        // not to a range a swap could satisfy. `arm_u <= inf_u` does
        // NOT bind: `sup·ρ ≈ inf·√D ≤ inf` whenever `D ≤ 1`, so an
        // assembly that read sups throughout lands at 0.5 and an
        // accidental single sup read lands at 0.49903 — both inside
        // that range, and the review measured a full sup-swap of
        // `certified_arms` passing the entire topo+sweep suite green
        // behind it. The digits below are the only thing that reds:
        // `T = (8/0.5)² + (1/1)² = 257`, `D = (0.5/(0.5·1))² = 1`,
        // `ρ = √(2/(257 + √(257²−4))) ≈ 0.062378`, so
        // `arm_u = 0.5·ρ ≈ 0.031189`. A sup-read assembly is 16×
        // larger and misses this by four orders of the tolerance.
        let (arm_u, arm_v) = certified_arms(&s, 0.0, 1.0, band()).unwrap();
        let t = (sup_u / inf_u).powi(2) + 1.0;
        let rho = (2.0 / (t + (t * t - 4.0).sqrt())).sqrt();
        assert!(
            (arm_u - inf_u * rho).abs() < 1e-12,
            "arm_u must be the derived {} , got {arm_u}",
            inf_u * rho
        );
        assert!(
            (arm_u - 0.031_189_379_189_942_3).abs() < 1e-12,
            "the fixture's published digit, got {arm_u}"
        );
        assert!((arm_v - rho).abs() < 1e-12, "arm_v = inf_v·ρ = ρ here");
        // The falsification threshold the review measured: an
        // accidental sup read of `inf.inf_u` alone assembles to
        // 0.499034, and a whole-assembly sup swap to exactly 0.5.
        // Both are more than an order above the true arm.
        assert!(
            arm_u < inf_u / 2.0,
            "a sup-read assembly lands at >= 0.499; the true arm is {arm_u}"
        );
    }

    /// **Item-1 exhibit, adopted from the review: a chart whose
    /// stretch concentrates in thin boundary layers separates the
    /// metred-chart mean width from the model region's.**
    ///
    /// The strip is `[0,1]² → 100 m × 1 m` with the `u` stretch
    /// pushed into `1e-4`-wide layers at each end. `σ_min` stays 1
    /// everywhere, so the metred copy is a legitimate contraction and
    /// the AREA claim is sound — but the model region is a long thin
    /// strip whose mean width is far under the square-ish scaled
    /// reading. This row pins the separation as a FACT about the
    /// pair, which is why `ChartOverlap::PositiveArea` claims area
    /// and not width.
    #[test]
    fn a_stretch_concentrating_chart_separates_the_two_widths() {
        // Degree 3 in u over one span, control x at 0, 0, 100, 100:
        // the derivative net is (0, 300, 0)·(3/1) scaled — stretch
        // concentrated in the middle rather than at the ends, which
        // is the same separation in a net this small can express.
        let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
        let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let xs = [0.0, 0.02, 99.98, 100.0];
        let mut control = Vec::new();
        for x in xs {
            control.push(Point3::new(x, 0.0, 0.0));
            control.push(Point3::new(x, 1.0, 0.0));
        }
        let s: Surface<f64> = Surface::Nurbs(Arc::new(
            NurbsSurface::new(ku, kv, control, vec![1.0; 8]).unwrap(),
        ));
        let inf = geom_brep::chart_stretch_inf(&s);
        // The u stretch spans three orders on ONE chart: the middle
        // control gap carries essentially all 100 m.
        assert!(
            inf.sup_u > inf.inf_u * 1000.0,
            "sup {} vs inf {}",
            inf.sup_u,
            inf.inf_u
        );
        let (arm_u, arm_v) = certified_arms(&s, 0.0, 1.0, band()).unwrap();
        // The metred unit square reads a mean width of `2A/P` on a
        // near-square `arm_u x arm_v` box; the honest transfer factor
        // to the MODEL region's mean width is `ρ/√T`, and here that
        // factor is small enough that the two readings are nowhere
        // near each other. Pin the factor, since it is the number the
        // enum's doc now quotes.
        let t = (inf.sup_u / inf.inf_u).powi(2) + (inf.sup_v / inf.inf_v).powi(2);
        let rho = arm_u / inf.inf_u;
        let transfer = rho / t.sqrt();
        assert!(
            transfer < 1e-3,
            "the width transfer factor on this chart is {transfer}, which is \
             exactly why the certified claim is AREA and not width"
        );
        assert!(arm_u > 0.0 && arm_v > 0.0, "the area claim still stands");
    }

    /// **The window WIRING, pinned at the pipeline stage that owns
    /// it** (review item 3: mutating `v_window` to read `p.x` used to
    /// pass the whole suite, because no row drove a window-DEPENDENT
    /// chart through the arms).
    ///
    /// A sphere pair whose `v` reach is pole-clear (`|v| ≤ 0.3`,
    /// arm `2·cos 0.3 ≈ 1.9107`) but whose `u` reach spans `π/2`.
    /// Reading the right axis certifies; reading `u` as the window
    /// hands `cos 1.65 < 0` to the gate and refuses `ArmUnbounded`.
    /// The two verdicts are on opposite sides of the gate, so the
    /// axis swap cannot pass this row.
    #[test]
    fn the_v_window_reads_the_second_channel_and_the_arms_follow_it() {
        let mut ba = Body::<f64>::new();
        let fa = sheet(&mut ba);
        let mut bb = Body::<f64>::new();
        let fb = sheet(&mut bb);
        let s = sphere(2.0);
        // u spans π/2 (1.5708); v stays inside |v| ≤ 0.3.
        let uv = |x0: f64, y0: f64, x1: f64, y1: f64| FaceUv {
            outer: rect(x0, y0, x1, y1),
            rings: Vec::new(),
        };
        let uv_a = uv(1.40, -0.30, 1.60, -0.10);
        let uv_b = uv(1.45, -0.25, 1.65, -0.05);
        // The honest window is the v reach [-0.30, -0.05].
        let (v_lo, v_hi) = v_window(&uv_a, &uv_b).unwrap();
        assert_eq!((v_lo, v_hi), (-0.30, -0.05), "the SECOND channel");
        let (arm_u, arm_v) = certified_arms(&s, v_lo, v_hi, band()).unwrap();
        assert!((arm_u - 2.0 * 0.30_f64.cos()).abs() < 1e-15);
        assert_eq!(arm_v, 2.0);
        // Driven through the pipeline stage that does the wiring.
        assert_eq!(
            overlap_of_uv(&ba, fa, &bb, fb, &s, &uv_a, &uv_b, band()).unwrap(),
            ChartOverlap::PositiveArea
        );
        // The axis the mutation would read is pole-crossing, and the
        // gate says so — this is the verdict the swap produces.
        assert!(matches!(
            certified_arms(&s, 1.40, 1.65, band()),
            Err(ChartRegionError::ArmUnbounded { chart: "sphere" })
        ));
    }

    /// A plane and a cylinder keep their EXACT constant arms — no
    /// bound, nothing to certify, and bit-identical to before.
    #[test]
    fn the_exact_constant_arms_do_not_move() {
        assert_eq!(
            certified_arms(&xy_plane(), -5.0, 5.0, band()).unwrap(),
            (1.0, 1.0)
        );
        let c: Surface<f64> = Surface::Cylinder {
            origin: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            radius: 3.0,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        assert_eq!(certified_arms(&c, -5.0, 5.0, band()).unwrap(), (3.0, 1.0));
    }
}

/// **The inf-arm rows under the INTERVAL scalar** (review item 4:
/// nothing in the original diff ran `net_inf`, `chart_stretch_inf` or
/// `certified_arms` under anything but `f64`, and both reviewers had
/// to write their own probes to find out whether they work at all).
///
/// Promoted from reviewer lane 8r2's interval probe, which printed
/// these values; the rows below assert them.
///
/// **Why this lane is where the bound is rigorous.** Every step of
/// the derivation — the net differences, the summed direction, the
/// `min_dot / |c|` quotient, the cross products, and the assembly's
/// trace/determinant arithmetic — is floating point. At `f64` each
/// step rounds to NEAREST, so the answer is a lower bound *up to a
/// few ulps*, not below-rounded: a bound that is exact in exact
/// arithmetic can come back a fraction of an ulp high. That is
/// immaterial against a band whose narrowest setting is 1e-12
/// relative to arms of order 1, and it is not an argument, which is
/// why the interval lane exists: there every step rounds outward and
/// the returned bracket's `lo()` is a genuine certified floor. These
/// rows check exactly that.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#[cfg(feature = "interval")]
mod inf_arms_interval {
    use super::certified_arms;
    use geom::{NurbsSurface, Surface};
    use geom_core::k_stats::decide;
    use geom_core::spline::KnotVector;
    use geom_core::{Band, Bounds, Interval, Margin, Point3, Real, Sign};
    use std::sync::Arc;

    fn band() -> Band {
        Band::new(1e-9, 1e-8).unwrap()
    }

    fn flat_chart(su: f64, sv: f64) -> Surface<Interval> {
        let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let f = Interval::from_f64;
        let control = vec![
            Point3::new(f(0.0), f(0.0), f(0.0)),
            Point3::new(f(0.0), f(sv), f(0.0)),
            Point3::new(f(su), f(0.0), f(0.0)),
            Point3::new(f(su), f(sv), f(0.0)),
        ];
        Surface::Nurbs(Arc::new(
            NurbsSurface::new(kv.clone(), kv, control, vec![1.0; 4]).unwrap(),
        ))
    }

    /// The `(4, 1)` orthogonal chart, under the interval scalar: the
    /// per-axis infs and the assembled arms bracket the `f64` answer,
    /// and the bracket's FLOOR is what a positive claim may lean on.
    #[test]
    fn the_inf_arms_are_certified_brackets_under_the_interval_scalar() {
        let s = flat_chart(4.0, 1.0);
        let i = geom_brep::chart_stretch_inf(&s);
        // Outward rounding widens the bracket by ulps, never more.
        assert!(
            i.inf_u.lo() > 3.999_999_999 && i.inf_u.hi() <= 4.0 + 1e-9,
            "inf_u bracket {:?}..{:?}",
            i.inf_u.lo(),
            i.inf_u.hi()
        );
        assert!(i.inf_v.lo() > 0.999_999_999 && i.inf_v.hi() <= 1.0 + 1e-9);
        assert!(i.area_inf.lo() > 3.999_999_999, "the area floor is 4");
        let (arm_u, arm_v) =
            certified_arms(&s, Interval::from_f64(0.0), Interval::from_f64(1.0), band()).unwrap();
        // An orthogonal chart of constant stretch: ρ = 1, so the arms
        // are the per-axis infs, and both floors are POSITIVE — which
        // is the only property a positive-area claim may use.
        assert!(
            arm_u.lo() > 3.999_999_999 && arm_v.lo() > 0.999_999_999,
            "arms ({:?}, {:?})",
            arm_u.lo(),
            arm_v.lo()
        );
        assert!(arm_u.lo() > 0.0 && arm_v.lo() > 0.0);
    }

    /// The refusals survive the scalar change: a folded net still
    /// reads a zero floor and refuses.
    #[test]
    fn a_folded_net_still_refuses_under_the_interval_scalar() {
        let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
        let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let f = Interval::from_f64;
        let p = |x: f64, y: f64| Point3::new(f(x), f(y), f(0.0));
        let control = vec![
            p(0.0, 0.0),
            p(0.0, 1.0),
            p(2.0, 0.0),
            p(2.0, 1.0),
            p(0.0, 0.0),
            p(0.0, 1.0),
        ];
        let s: Surface<Interval> = Surface::Nurbs(Arc::new(
            NurbsSurface::new(ku, kv, control, vec![1.0; 6]).unwrap(),
        ));
        // The interval quotient divides by a bracket straddling zero,
        // so the floor is 0 and the bracket is wide — the honest
        // reading of a net whose derivative vanishes somewhere. The
        // gate reads the FLOOR, so the refusal stays typed rather than
        // degrading to an escalation.
        let i = geom_brep::chart_stretch_inf(&s);
        assert_eq!(i.inf_u.lo(), 0.0, "no positive floor on a folded net");
        assert!(matches!(
            certified_arms(&s, f(0.0), f(1.0), band()),
            Err(super::ChartRegionError::ArmUnbounded { chart: "NURBS" })
        ));
    }

    /// The pole-joint gate on a SPLINE chart under the interval
    /// scalar — the path the old constant `1` arm could not reach.
    /// Promoted from the reviewer probe's printed table.
    #[test]
    fn the_spline_pole_joint_gate_answers_all_three_ways() {
        let sup = |span: f64| geom_brep::chart_stretch_sup(&flat_chart(span, span)).0;
        assert_eq!(
            decide("pcurve_loop_pole_joint", Margin::of(sup(1e-12)), band()),
            Ok(Sign::Zero)
        );
        for span in [1e-9, 5e-9] {
            assert!(
                decide("pcurve_loop_pole_joint", Margin::of(sup(span)), band()).is_err(),
                "in-band lever {span:e} escalates"
            );
        }
        assert_eq!(
            decide("pcurve_loop_pole_joint", Margin::of(sup(1.0)), band()),
            Ok(Sign::Positive)
        );
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod r2_mate8_probes {
    //! Blinded-review probes (lane R2, PR #1472): adversarial edge
    //! cases for `decomposition_witness`'s completeness argument and
    //! its budget guard. Probe-branch only; not part of the unit.
    use super::*;

    fn pt(x: f64, y: f64) -> Point2<f64> {
        Point2::new(x, y)
    }

    fn rect(x0: f64, y0: f64, x1: f64, y1: f64) -> Vec<Point2<f64>> {
        vec![pt(x0, y0), pt(x1, y0), pt(x1, y1), pt(x0, y1)]
    }

    fn uv(outer: Vec<Point2<f64>>, rings: Vec<Vec<Point2<f64>>>) -> FaceUv<f64> {
        FaceUv { outer, rings }
    }

    /// Strict even-odd containment of `(x, y)` in `poly`, with a
    /// straight-line boundary margin so "strictly inside" is honest.
    fn strictly_in(poly: &[Point2<f64>], x: f64, y: f64, margin: f64) -> bool {
        let mut inside = false;
        for i in 0..poly.len() {
            let (p, q) = (poly[i], poly[(i + 1) % poly.len()]);
            // Distance to the segment must exceed the margin.
            let (dx, dy) = (q.x - p.x, q.y - p.y);
            let len2 = dx * dx + dy * dy;
            let t = (((x - p.x) * dx + (y - p.y) * dy) / len2).clamp(0.0, 1.0);
            let (cx, cy) = (p.x + t * dx, p.y + t * dy);
            if ((x - cx).powi(2) + (y - cy).powi(2)).sqrt() <= margin {
                return false;
            }
            if (p.y > y) != (q.y > y) && x < p.x + (y - p.y) * dx / dy {
                inside = !inside;
            }
        }
        inside
    }

    fn in_region(uv: &FaceUv<f64>, x: f64, y: f64) -> bool {
        strictly_in(&uv.outer, x, y, 1e-9) && uv.rings.iter().all(|r| !strictly_in(r, x, y, -1.0))
    }

    /// P1 — every boundary edge of the overlap is VERTICAL or
    /// horizontal (the axis-aligned seat): vertical segments produce no
    /// midline crossings, and the argument says they need not.
    #[test]
    fn r2p1_axis_aligned_vertical_edges_find_a_witness() {
        let a = uv(rect(0.0, 0.0, 4.0, 4.0), vec![]);
        let b = uv(rect(1.0, -1.0, 3.0, 2.0), vec![]);
        let mut hit = None;
        let found = decomposition_witness(&a, &b, |x, y| {
            let ok = in_region(&a, x, y) && in_region(&b, x, y);
            if ok {
                hit = Some((x, y));
            }
            ok
        });
        assert!(found, "axis-aligned overlap must yield a witness");
        let (x, y) = hit.unwrap();
        assert!(x > 1.0 && x < 3.0 && y > 0.0 && y < 2.0, "({x}, {y})");
    }

    /// P2 — a COLLINEAR SHARED SPAN between the two boundaries (the
    /// TouchingBoundary trigger class itself): parallel pairs
    /// contribute no meeting abscissae and must not be needed.
    #[test]
    fn r2p2_collinear_shared_span_finds_a_witness() {
        let a = uv(rect(0.0, 0.0, 4.0, 2.0), vec![]);
        let b = uv(rect(1.0, 0.0, 3.0, 3.0), vec![]);
        assert!(decomposition_witness(&a, &b, |x, y| {
            in_region(&a, x, y) && in_region(&b, x, y)
        }));
    }

    /// P2b — bit-identical outers (every segment duplicated): the
    /// duplicate midline crossings form zero-height cells, which must
    /// be skipped, and the real cell must still be offered.
    #[test]
    fn r2p2b_identical_outers_find_a_witness() {
        let a = uv(rect(0.0, 0.0, 2.0, 2.0), vec![]);
        let b = uv(rect(0.0, 0.0, 2.0, 2.0), vec![]);
        assert!(decomposition_witness(&a, &b, |x, y| {
            in_region(&a, x, y) && in_region(&b, x, y)
        }));
    }

    /// P3 — a RING swallowing the naive centre: holes must be in the
    /// decomposition (the doc's own claim), else the two genuine cells
    /// beside the hole are merged and neither centre offered.
    #[test]
    fn r2p3_ring_blocking_the_centre_finds_a_witness() {
        let a = uv(rect(0.0, 0.0, 4.0, 4.0), vec![rect(1.4, 1.4, 2.6, 2.6)]);
        let b = uv(rect(1.0, 1.0, 3.0, 3.0), vec![]);
        let mut hit = None;
        let found = decomposition_witness(&a, &b, |x, y| {
            let ok = in_region(&a, x, y) && in_region(&b, x, y);
            if ok {
                hit = Some((x, y));
            }
            ok
        });
        assert!(found, "the region minus its hole still has interior");
        let (x, y) = hit.unwrap();
        assert!(
            !(x > 1.4 && x < 2.6 && y > 1.4 && y < 2.6),
            "witness ({x}, {y}) must not be in the hole"
        );
    }

    /// P4 — repeated abscissae (three vertices sharing x = 2.0) and a
    /// non-convex outer: dedup must not merge distinct events away.
    #[test]
    fn r2p4_repeated_abscissae_find_a_witness() {
        let a = uv(
            vec![
                pt(0.0, 0.0),
                pt(2.0, 0.0),
                pt(2.0, 1.0),
                pt(3.0, 2.0),
                pt(2.0, 3.0),
                pt(0.0, 3.0),
            ],
            vec![],
        );
        let b = uv(rect(1.0, 0.2, 1.8, 2.8), vec![]);
        assert!(decomposition_witness(&a, &b, |x, y| {
            in_region(&a, x, y) && in_region(&b, x, y)
        }));
    }

    /// P5 — the SEGMENT budget: two 70-gon "discs" in fat, decidable
    /// overlap carry 140 > 128 segments, and the schedule declines
    /// WITHOUT PROBING AT ALL — a silent `false`, spelled by the caller
    /// as the carried `TouchingBoundary`. This pins the honesty
    /// boundary the PR's deviation 2 discloses: exhaustion never
    /// mis-certifies, but nothing at the call site says "budget".
    #[test]
    fn r2p5_segment_budget_declines_a_fat_decidable_overlap_silently() {
        let ngon = |cx: f64, n: usize| -> Vec<Point2<f64>> {
            (0..n)
                .map(|i| {
                    let t = core::f64::consts::TAU * (i as f64) / (n as f64);
                    pt(cx + 2.0 * t.cos(), 2.0 * t.sin())
                })
                .collect()
        };
        let a = uv(ngon(0.0, 70), vec![]);
        let b = uv(ngon(1.0, 70), vec![]);
        let mut calls = 0usize;
        let found = decomposition_witness(&a, &b, |x, y| {
            calls += 1;
            in_region(&a, x, y) && in_region(&b, x, y)
        });
        assert!(!found, "over-budget: the schedule declines");
        assert_eq!(calls, 0, "and it declines before offering anything");
        // The same pair one segment under the cap certifies fine —
        // the decline above is the budget's, not the geometry's.
        let a64 = uv(ngon(0.0, 64), vec![]);
        let b64 = uv(ngon(1.0, 64), vec![]);
        assert!(decomposition_witness(&a64, &b64, |x, y| {
            in_region(&a64, x, y) && in_region(&b64, x, y)
        }));
    }

    /// P6 — the CELL budget is a hard cap on probe calls (structural
    /// companion to P5): an always-false probe on a busy pair is
    /// called at most `WITNESS_BUDGET.cells` times.
    #[test]
    fn r2p6_cell_budget_caps_probe_calls() {
        let ngon = |cx: f64, n: usize| -> Vec<Point2<f64>> {
            (0..n)
                .map(|i| {
                    let t = core::f64::consts::TAU * (i as f64) / (n as f64);
                    pt(cx + 2.0 * t.cos(), 2.0 * t.sin())
                })
                .collect()
        };
        let a = uv(ngon(0.0, 60), vec![]);
        let b = uv(ngon(1.0, 60), vec![]);
        let mut calls = 0usize;
        let found = decomposition_witness(&a, &b, |_, _| {
            calls += 1;
            false
        });
        assert!(!found);
        assert!(calls <= WITNESS_BUDGET.cells, "{calls} probes");
        assert!(calls > 0, "the pair is busy enough to probe at all");
    }
}
