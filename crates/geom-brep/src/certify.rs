//! Certified carriers: the D4 ¶2 attachment gate for edge geometry.
//!
//! An edge's concrete 3-D curve (its *carrier*) is a derived cache of
//! its intensional description ([`crate::EdgeDescription`]). This module
//! is the only way to marry the two: [`EdgeCurve::certify`] takes an
//! uncertified [`EdgeCurveSpec`] plus the edge's endpoint points and
//! either returns a certified [`EdgeCurve`] — whose fields are private,
//! so **an uncertified carrier is unrepresentable** — or fails with a
//! typed [`CertifyError`] (operation-time enforcement, D4 ¶3). The
//! tier-3 validator re-runs the same checks at rest
//! ([`EdgeCurve::recertify`]).
//!
//! # The deterministic sampling schedule (D9)
//!
//! Certification is closed-form residual evaluation at a **fixed
//! schedule**: [`CERT_SAMPLES`] = 9 parameters
//! `t_i = t₀ + (t₁ − t₀)·(i/8)`, i = 0…8 — endpoints included, the
//! fractions exact dyadics, the arithmetic a fixed association order —
//! so two runs over the same body produce byte-identical
//! [`Certificate`]s. Interior samples (i = 1…7) additionally carry the
//! transversality check for `Intersection` descriptions (endpoint
//! vertices may legitimately sit on chart/surface singularities — cone
//! apexes, sphere poles — where angular classification honestly
//! poisons; residual checks, which never need a gradient, still run at
//! the endpoints).
//!
//! # Dimensional honesty
//!
//! Every classified residual is a **length in meters** against the
//! run's linear band: point-to-point distances directly, implicit
//! surface residuals in the linearized forms of [`crate::implicit`],
//! angular transversality as displacement through its lever arm
//! ([`crate::classify_dihedral`]'s margin). No squared bands, no
//! dimensionless margins.
//!
//! # The stored parameter interval — a certified cache, not an authority
//!
//! `geom`'s ratified curve convention stands: an edge's bounds are
//! *derived from its vertices* — the authority is the vertex geometry.
//! The [`EdgeCurve`] nevertheless **stores** the parameter interval,
//! as a certified derived cache in exactly the carrier's sense: the
//! endpoint-pinning checks of every certification pin
//! `carrier(t₀)`/`carrier(t₁)` to the endpoint points within ε, the
//! span checks enforce the ratified forward direction (t₁ > t₀) and —
//! for circle carriers — the at-most-one-period winding bound, and the
//! Intersection mid-parameter pin ties the interval's interior to the
//! stored witness; all at attachment and again at tier 3. Storing
//! the certified interval is what keeps periodic carriers total
//! (a full-period scaffolding edge's `(0, τ)` is not recoverable from
//! its coincident endpoints) and keeps bound recovery evaluation-free
//! (no atan2 branch selection). It can never disagree with the vertices
//! by more than ε without failing loudly — a cache, never a peer.

use geom::Curve3;
use geom::Surface;
use geom_core::spline::SpanLocate;
use geom_core::{Band, BandError, Decide, Indeterminate, Margin, Point3, Real, Sign};

use crate::description::{
    ChartCurve, EdgeAuthority, EdgeDescription, EdgeDescriptionSpec, authority_of,
};
use crate::dihedral::{DihedralClass, classify_dihedral, decide};
use crate::implicit::{implicit_residual, seam_frame};
use crate::keys::SurfaceKey;
use crate::pcurve_cache::{Pcurve, PcurveCertifyError, chart_pcurve};

/// The fixed certification sample count (module docs): 9 uniform
/// parameters, endpoints included.
pub const CERT_SAMPLES: u32 = 9;

/// The `sample` a [`CertifyError`] carries when the failing check is
/// **not a sampled one** — currently the chart-image mint, which runs
/// once, before the schedule.
///
/// It is a value no schedule index can take ([`CERT_SAMPLES`] counts
/// 0…8), so a reader can tell "this check has no sample" from "this
/// check failed at sample 0". Reporting `0` there states a schedule
/// point that was never visited, which is a fabricated diagnostic
/// however small it looks.
///
/// **The interval checks ([`CertCheck::ParamSpan`]) still report `0`**
/// and are not changed here: their `Display` text is pinned by
/// `step-import`'s tier-gate test, so moving them is a test rewrite
/// that belongs with the consumer pass, not with this unit. Recorded
/// rather than quietly left: the same fabrication is there, one door
/// over.
pub const NOT_A_SAMPLE: u32 = u32::MAX;

/// Which certification check a [`CertifyError`] names — the residual
/// taxonomy, one variant per documented check.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CertCheck {
    /// The stored parameter interval's span checks (forward direction;
    /// circle-carrier winding bound) — named on escalations of those
    /// decisions; their definite failures are the dedicated
    /// [`CertifyError::IntervalNotForward`] /
    /// [`CertifyError::WindingExceeded`] variants.
    ParamSpan,
    /// `|carrier(t₀) − start point|` (check 1).
    EndpointStart,
    /// `|carrier(t₁) − end point|` (check 2).
    EndpointEnd,
    /// Intersection: implicit residual against `s1` at a sample.
    Surface1Residual,
    /// Intersection: implicit residual against `s2` at a sample.
    Surface2Residual,
    /// Intersection: the witness point's residual against `s1`.
    WitnessSurface1,
    /// Intersection: the witness point's residual against `s2`.
    WitnessSurface2,
    /// Intersection: `|carrier((t₀ + t₁)/2) − witness|` — the witness
    /// **is** the edge's mid-parameter point (the M2 PR 3 fix-pass
    /// sharpening of the witness contract): together with endpoint
    /// pinning and the circle winding bound this pins *which* arc of
    /// the intersection locus the interval traverses, and in which
    /// direction — the component/side selection the 9 surface-residual
    /// samples alone cannot see (every point of the wrong arc also
    /// lies on both surfaces).
    WitnessMidpoint,
    /// Intersection: the transversality margin at an interior sample
    /// (the dihedral displacement margin — must be definitely
    /// transverse).
    Transversality,
    /// TangentIntersection: the normal-parallelism defect at an
    /// interior sample — `sin θ` metered at the lever arm `1/κ_rel`
    /// (D2's derived angular threshold ε·κ_rel; C7 jet schedule,
    /// M5 PR 9).
    TangentParallel,
    /// TangentIntersection: the second-order margin at an interior
    /// sample — the relative transverse normal curvature `|κ_rel|`
    /// metered as the displacement it induces at the folded lever arm
    /// (D4 ¶1), which must be definitely positive (the jet system's
    /// IFT denominator).
    TangentSecondOrder,
    /// TangentIntersection: the C2.2 between-samples statement — the
    /// worst sampled residual plus the certified quadratic sag bound
    /// must stay within ε (a sup bound, never a sampled max
    /// pretending to be one).
    TangentHull,
    /// TangentIntersection: the C2.3-style tube statement on the JET
    /// system — the sampled `|κ_rel|` minus its certified
    /// between-samples drift stays definitely positive over every
    /// span, so the second-order separation (and with it local
    /// uniqueness of the locus) holds along the WHOLE edge, not just
    /// at the schedule points.
    TangentTube,
    /// Scaffold: `|carrier(t_i) − description(s_i)|` at a sample —
    /// the fenced scaffolding arm's meter, the one conventional
    /// residual that has no chart to state itself against (D3).
    MappedSource,
    /// Chart, periodic-seam obligation: the out-of-halfplane component
    /// `|w · v_ref|` at a sample.
    SeamHalfplane,
    /// Chart, periodic-seam obligation: the wrong-side excess
    /// `max(0, −w · u_ref)` at a sample (distinguishes the seam from
    /// the antipodal meridian — the unified meter alone cannot, since
    /// the antipodal ruling IS a chart image of the same surface).
    SeamSide,
    /// The **mint step** of the collapsed conventional description
    /// (D4): deriving the chart image the meter is stated against.
    /// Not a sampled check — like [`CertCheck::ParamSpan`] it runs
    /// once, and the `sample` field of a [`CertifyError`] naming it
    /// carries the module's not-a-sample sentinel rather than an
    /// index that was never visited.
    ChartImage,
    /// **The unified conventional meter** (D1): `|C(tᵢ) − S(P(tᵢ))|`
    /// at a sample, C4 verbatim — the ONE statement every collapsed
    /// conventional description makes, whatever certification lane its
    /// [`crate::Pcurve`] belongs to.
    ChartResidual,
    /// Intersection, plane × NURBS (M7-8): limb 1's largest sampled
    /// on-locus residual over both operands — the closed-form plane
    /// distance and the certified foot distance on the wall.
    PlaneNurbsOnLocus,
    /// Intersection, plane × NURBS (M7-8): limb 2's certified
    /// **sup-norm** bound over the whole span — the number that
    /// certifies (a bound, never a sampled max).
    PlaneNurbsHull,
    /// Intersection, plane × NURBS (M7-8): the lane's own margins as a
    /// whole, named when one of them escalates.
    PlaneNurbsCertificate,
}

#[allow(non_upper_case_globals)] // a variant's retired NAME, not a constant
impl CertCheck {
    /// **Shim** (P-1a): the pre-collapse name of the arm that already
    /// stated `|C − S(P)|`, aliasing the unified meter it became.
    ///
    /// It exists so the consumer crates and their probes read the same
    /// name through P-1a's `geom-brep`-only diff; P-1b deletes it
    /// together with the probes that spell it.
    pub const IsoResidual: Self = Self::ChartResidual;

    /// **Shim** (P-1a): the seam arm's retired implicit-form residual.
    /// The seam's on-surface statement is now the unified meter's, so
    /// this names the same check the seam obligation rides beside.
    pub const SeamSurface: Self = Self::ChartResidual;
}

/// Typed certification failure (D4 ¶3): actionable, closed enum. The
/// body-side attachment gates (`topo`'s operators and setters) wrap
/// this; the tier-3 validator carries it inside its own error.
///
/// Magnitude diagnostics ride on [`CertifyError::Escalated`]'s
/// [`Indeterminate`] (band + margin view). Definite exceedances name
/// the check and sample; the magnitude itself is scalar-typed and is
/// not extracted into the error (no `f64`-projection of a generic `T`
/// exists for every lane — the Dual lane in particular).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CertifyError {
    /// The collapsed conventional description's **chart image could
    /// not be derived** for this (chart, carrier) pair (D4's mint
    /// door, `crate::chart_pcurve`).
    ///
    /// This is a statement about the GEOMETRY, not about a missing
    /// feature: the named carrier is not a locus of any certified
    /// image class of the named chart — an ellipse on a cone, say,
    /// which no cone chart image can be. It is deliberately distinct
    /// from [`CertifyError::Unimplemented`], which means a described
    /// surface or carrier is of a kind this module refuses wholesale;
    /// collapsing the two would answer "not implemented" to a caller
    /// whose description is simply wrong.
    ChartImageUnavailable {
        /// The chart kind the description named.
        chart: &'static str,
        /// The carrier kind offered against it.
        carrier: &'static str,
    },
    /// A surface key in the description did not resolve in the owning
    /// body (stale, or the surface does not exist yet — attach the
    /// intrinsic description once its surfaces are in the arena).
    UnresolvedSurface {
        /// The unresolved key.
        key: SurfaceKey,
    },
    /// A described surface is the `Nurbs` kind (no implicit form —
    /// its residual story is the SSI foot-point machinery, not this
    /// module's), or a `Nurbs` carrier arrived under a conventional
    /// (`MappedCurve`/`Seam`) description.
    ///
    /// TWO `Intersection` rungs certify, and this variant is what is
    /// left over. The analytic rung: both operands analytic (M5 PR 9,
    /// C12.3 — the class the curved-boolean zip mints). The **plane ×
    /// NURBS** rung (M7-8): exactly one PLANE and one described NURBS
    /// wall, declare-and-check, reachable only through
    /// [`EdgeCurve::certify_nurbs_lane`] — a caller on the plain
    /// [`EdgeCurve::certify`] door injects no lane and still lands
    /// here, and NURBS × NURBS has no certificate in this build.
    Unimplemented,
    /// An `Intersection` description names one surface twice — a
    /// same-surface locus is a `Seam`, never an intersection.
    IntersectionSameSurface {
        /// The doubly-named key.
        key: SurfaceKey,
    },
    /// A `Seam` description on a non-periodic surface (a plane) — no
    /// seam exists.
    SeamOnNonPeriodic,
    /// The stored parameter interval is not forward (t₁ − t₀, metered
    /// as arc length, is definitely ≤ 0 at tolerance): the ratified
    /// vertices-derive-bounds convention — increasing parameter runs
    /// start → end of `he_plus` — is violated by a decreasing interval,
    /// and a **degenerate** (zero-span) interval is refused by the same
    /// gate (M2 PR 3 fix pass, N1/N2: no M2 construction mints
    /// zero-length edges — coincident-endpoint chords are already
    /// refused as poison, and self-loop scaffolding carries the full
    /// period — so a zero span is always a defect, not data).
    IntervalNotForward,
    /// A circle carrier's stored interval spans definitely more than
    /// one full period (arc length `(t₁ − t₀)·r > τ·r` beyond
    /// tolerance). This closes the 9-sample winding alias (M2 PR 3
    /// fix pass, S1): the uniform schedule `s = i/8` is aliased exactly
    /// by intervals wrong by `8kτ` (the per-sample discrepancy
    /// `(Δ − θ)·i/8` vanishes mod τ iff `Δ = θ + 8kτ`), and with
    /// `0 < Δt ≤ τ` enforced no `k ≠ 0` alias is representable — the
    /// pointwise sample matches then pin the winding *within* the
    /// period.
    WindingExceeded,
    /// A residual definitely exceeded the escalation threshold: the
    /// cache does not represent the description (D4 ¶2's `residual ≤ ε`
    /// kernel invariant violated beyond doubt).
    ResidualExceeded {
        /// The check that failed.
        check: CertCheck,
        /// The schedule sample index (0…[`CERT_SAMPLES`]−1; 0 for the
        /// witness/endpoint checks).
        sample: u32,
    },
    /// `Intersection` only: the tangent planes coincide at a sample —
    /// the transversality precondition fails, so the locus is not an
    /// `Intersection` (a tangential contact is `TangentIntersection`
    /// territory; a seam is `Seam`).
    NotTransverse {
        /// The interior sample index.
        sample: u32,
    },
    /// `TangentIntersection` only: the second-order margin (relative
    /// transverse normal curvature, metered at the folded lever arm)
    /// is exactly zero at a sample — the surfaces under-determine the
    /// locus there (a G2 conventional join's zero-side margin, or an
    /// osculating patch), so the intrinsic tangent description is not
    /// certifiable; the honest description is conventional
    /// (`MappedCurve`), exactly D2's G2-join split. The **definite**
    /// half of a two-tolerance pair: one band-width away the same
    /// margin escalates as [`CertifyError::Escalated`] under
    /// [`CertCheck::TangentSecondOrder`] instead (F6 — an osculating
    /// pair is a sliver at this ε).
    NotSecondOrderSeparated {
        /// The sample index.
        sample: u32,
        /// The band the margin was classified against (the
        /// two-tolerance pair's shared frame).
        band: Band,
    },
    /// `TangentIntersection` only: the (carrier kind, surface kinds)
    /// triple is outside the jet certificate's certified span-bound
    /// lane (`Line` carriers on `Plane`/`Cylinder`/`Sphere` pairs —
    /// the class the C5 tangent arms mint at M5). A routing boundary
    /// (C12.1: per-class retirement with its proof), never a runtime
    /// fallback.
    TangentCertificateUnsupported,
    /// A classification escalated: the margin landed in the sliver band
    /// or was poisoned (D4 ¶3's escalate-never-guess).
    Escalated {
        /// The check that escalated.
        check: CertCheck,
        /// The schedule sample index.
        sample: u32,
        /// The classifier's diagnostic (band + margin view).
        cause: Indeterminate,
    },
    /// The linear band could not be built from the run's tolerance
    /// (absurd ε — see `Band::linear`'s error docs).
    Band(BandError),
    /// `Intersection` of a PLANE and a described NURBS wall (M7-8):
    /// the declare-and-check lane refused, carrying its own measured
    /// bound. The file's carrier was adopted as EVIDENCE and did not
    /// hold up — this variant is the evidence's verdict, with the
    /// number.
    PlaneNurbs(crate::edge_nurbs::PlaneNurbsRefusal),
}

impl core::fmt::Display for CertifyError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::UnresolvedSurface { key } => {
                write!(f, "certification: surface key {key:?} does not resolve")
            }
            Self::ChartImageUnavailable { chart, carrier } => write!(
                f,
                "certification: a conventional description on a {chart} chart has no \
                 certified chart image for a {carrier} carrier — the locus this \
                 description claims is not one this chart can state. The description \
                 is wrong, not the build"
            ),
            Self::Unimplemented => write!(
                f,
                "certification: a Nurbs described surface, or a Nurbs carrier under a \
                 conventional description, cannot be certified in this build — rung-3 \
                 carriers certify as the Intersection of two analytic surfaces, or of \
                 one plane and one described NURBS wall through the declare-and-check \
                 lane; NURBS x NURBS has no certificate"
            ),
            Self::IntersectionSameSurface { key } => write!(
                f,
                "certification: Intersection names surface {key:?} twice (a same-surface \
                 locus is a Seam)"
            ),
            Self::SeamOnNonPeriodic => write!(
                f,
                "certification: Seam described on a non-periodic surface (a plane has no \
                 seam)"
            ),
            Self::IntervalNotForward => write!(
                f,
                "certification: the stored parameter interval is not forward — increasing \
                 parameter must run start → end of he_plus (the ratified \
                 vertices-derive-bounds convention), and a degenerate zero-span interval \
                 is refused by the same gate"
            ),
            Self::WindingExceeded => write!(
                f,
                "certification: a periodic (circle/ellipse) carrier's parameter interval \
                 spans more than one full period — |t₁ − t₀| ≤ τ is required to close the \
                 sample-schedule winding alias (8kτ family)"
            ),
            Self::ResidualExceeded { check, sample } => write!(
                f,
                "certification: {check:?} residual at sample {sample} definitely exceeds \
                 the tolerance band (the cache does not represent the description, D4 ¶2)"
            ),
            Self::PlaneNurbs(refusal) => write!(
                f,
                "certification: the plane × NURBS Intersection lane refused — {refusal}"
            ),
            Self::NotTransverse { sample } => write!(
                f,
                "certification: tangent planes coincide at interior sample {sample} — the \
                 Intersection transversality precondition fails (D2); {}",
                geom_core::COINCIDENCE_RECOURSE
            ),
            Self::NotSecondOrderSeparated { sample, band } => write!(
                f,
                "certification: the tangency's second-order margin (relative transverse \
                 normal curvature, tangent_second_order) is exactly zero at sample \
                 {sample} against band [zero {:e}, escalate {:e}] — the surfaces \
                 under-determine the locus there (a G2 conventional join keeps its \
                 MappedCurve description BY THIS PREDICATE, D2's split); the same \
                 margin one band-width away escalates as a sliver instead; {}",
                band.zero(),
                band.escalate(),
                geom_core::COINCIDENCE_RECOURSE
            ),
            Self::TangentCertificateUnsupported => write!(
                f,
                "certification: this (carrier, surface-pair) class is outside the jet \
                 certificate's certified span-bound lane — the certified class is Line \
                 carriers on Plane/Cylinder/Sphere pairs (classes retire one at a time, \
                 each with its proof; no runtime fallback)"
            ),
            // The not-a-sample sentinel renders as words, not as
            // 4294967295: a diagnostic whose whole purpose is to stop
            // claiming a schedule point it never visited should not
            // then print a number that looks like one. Sampled checks
            // keep their exact wording — `step-import`'s tier gate
            // pins the `sample 0` string, and it is still true there.
            Self::Escalated {
                check,
                sample,
                cause,
            } if *sample == NOT_A_SAMPLE => write!(
                f,
                "certification: {check:?} (not a sampled check) escalated: {cause}"
            ),
            Self::Escalated {
                check,
                sample,
                cause,
            } => write!(
                f,
                "certification: {check:?} at sample {sample} escalated: {cause}"
            ),
            Self::Band(e) => write!(f, "certification: {e}"),
        }
    }
}

impl std::error::Error for CertifyError {}

/// The uncertified input to [`EdgeCurve::certify`]: description,
/// carrier cache, and the carrier-parameter interval, exactly as the
/// construction prepared them. Plain data — the certified product is
/// [`EdgeCurve`].
#[derive(Clone, Debug)]
pub struct EdgeCurveSpec<T: Real> {
    /// The intensional description (authoritative), as the
    /// construction states it — [`EdgeDescriptionSpec`].
    pub description: EdgeDescriptionSpec<T>,
    /// The carrier cache to certify against it.
    pub carrier: Curve3<T>,
    /// Carrier parameter at `start(he_plus)` — the `he_plus` forward
    /// contract's t₀ (increasing parameter runs start → end).
    pub param_start: T,
    /// Carrier parameter at `end(he_plus)` — t₁.
    pub param_end: T,
}

impl<T: Real> EdgeCurveSpec<T> {
    /// The straight-chord spec between two points: carrier the line
    /// from `p0` to `p1` (arc-length parameters `0 … |p1 − p0|`),
    /// description the honest pushforward form — `p0`'s trajectory
    /// under the translation by `p1 − p0`
    /// ([`crate::MappedCurve::ExtrudedPoint`] with the sketch origin
    /// placed at `p0`).
    ///
    /// By calling this the caller asserts the edge's locus **is** the
    /// straight chord; a construction whose edge follows any other
    /// locus (an arc trajectory, a placed profile segment) builds its
    /// spec explicitly. Coincident endpoints yield a poison carrier
    /// that certification rejects loudly (typed, total).
    pub fn line_between(p0: Point3<T>, p1: Point3<T>) -> Self {
        use geom_core::{Affine3, Point2};
        let len = p0.distance(p1);
        Self {
            description: EdgeDescriptionSpec::Scaffold(
                crate::mapped::MappedCurve::ExtrudedPoint {
                    point: Point2::new(T::zero(), T::zero()),
                    place: Affine3::translation(p0 - Point3::origin()),
                    vec: p1 - p0,
                },
            ),
            carrier: Curve3::Line {
                origin: p0,
                dir: (p1 - p0) / len,
            },
            param_start: T::zero(),
            param_end: len,
        }
    }

    /// The conventional ARC spec along an existing CIRCLE carrier
    /// between the given parameters: the carrier and interval are kept
    /// verbatim, and the description is the honest pushforward form —
    /// the start point's trajectory under the rotation about the
    /// carrier's own axis by the swept angle
    /// ([`crate::MappedCurve::RevolvedPoint`], the same
    /// geometry-derived posture as [`Self::line_between`]'s
    /// `ExtrudedPoint`). This is the conventional description for a
    /// circular locus the adjacent surfaces UNDER-determine (D2's
    /// split — e.g. a seam between coplanar faces).
    ///
    /// `None` for a non-circle carrier: no other kind has this
    /// rotation pushforward, and the caller owns its own honest
    /// refusal there.
    pub fn arc_of_circle(carrier: Curve3<T>, t0: T, t1: T) -> Option<Self>
    where
        T: SpanLocate,
    {
        use geom_core::{Affine3, Point2, Point3};
        let Curve3::Circle { center, axis, .. } = carrier else {
            return None;
        };
        let start = carrier.eval(t0);
        Some(Self {
            description: EdgeDescriptionSpec::Scaffold(
                crate::mapped::MappedCurve::RevolvedPoint {
                    point: Point2::new(T::zero(), T::zero()),
                    place: Affine3::translation(start - Point3::origin()),
                    axis_origin: center,
                    axis_dir: axis,
                    angle: t1 - t0,
                },
            ),
            carrier,
            param_start: t0,
            param_end: t1,
        })
    }

    /// The canonical full-period self-loop spec at `p`: a unit circle
    /// through `p` (center `p + x̂`, axis `ẑ`, parameters `0 … τ`),
    /// described as `p`'s trajectory under a full revolution about
    /// that center.
    ///
    /// **Scaffolding convention**: null edges at `p` need *some*
    /// certified closed carrier during construction sequences whose
    /// real geometry arrives later (or never reaches rest — ring
    /// scaffolding is typically consumed by `kemr`). Two shapes
    /// qualify: self-loop edges (both endpoints one vertex — `mef`'s
    /// circular/lone sites), and null edges between two **distinct**
    /// bitwise-coincident vertices (sweep's zip closure; the book's
    /// `loopglue` does the same) — the endpoint-pin certification
    /// holds either way, because the full-period parameter interval
    /// makes both endpoint evaluations the *same* identity, forcing
    /// bitwise coincidence. This is that deterministic choice: honest
    /// data (the carrier truly is this circle and truly closes at
    /// `p`), deliberately arbitrary geometry — the stored parameter
    /// interval `(0, τ)` is exactly the certified-cache case the
    /// module docs cover.
    pub fn self_loop_circle_at(p: Point3<T>) -> Self {
        use geom_core::{Affine3, Point2, Vec3};
        let center = p + Vec3::unit_x();
        Self {
            description: EdgeDescriptionSpec::Scaffold(
                crate::mapped::MappedCurve::RevolvedPoint {
                    point: Point2::new(T::zero(), T::zero()),
                    place: Affine3::translation(p - Point3::origin()),
                    axis_origin: center,
                    axis_dir: Vec3::unit_z(),
                    angle: T::tau(),
                },
            ),
            carrier: Curve3::Circle {
                center,
                axis: Vec3::unit_z(),
                radius: T::one(),
                // u_ref points from the center back at p.
                u_ref: Vec3::new(T::zero() - T::one(), T::zero(), T::zero()),
            },
            param_start: T::zero(),
            param_end: T::tau(),
        }
    }
}

/// The certification record stored with a certified carrier: the
/// schedule that ran and the worst distance residual it observed
/// (meters). Byte-identical across replays of the same construction
/// (D9) — under test via its `Debug` form.
#[derive(Clone, Copy, Debug)]
pub struct Certificate<T: Real> {
    /// The sample count of the schedule that ran ([`CERT_SAMPLES`]).
    pub samples: u32,
    /// The maximum magnitude over every classified **distance** residual
    /// (endpoint, surface, scaffolding-source, chart and seam-obligation
    /// checks; transversality margins are clearance margins, not
    /// residuals, and are excluded). Certified ≤ ε by construction.
    ///
    /// **This number may MOVE at the conventional arms across the U2
    /// collapse** (D2): the three pre-collapse forms did not measure
    /// the same thing, and a `Pcurve` is a function of the carrier's
    /// own parameter where the iso arm's `v` walked the schedule
    /// fraction. The move is measured per fixture AND PER DRIFT SCALE
    /// and pinned — see `D2_SWEEP` in this module's tests, whose two
    /// arms move in opposite directions with scale — never laundered
    /// by re-associating a meter to make a number match.
    pub max_residual: T,
}

/// A certified edge carrier: the intensional description, its cached
/// [`Curve3`] carrier with the certified parameter interval, and the
/// [`Certificate`] of the attachment-time run. Constructible only
/// through [`EdgeCurve::certify`] — fields are private so an
/// uncertified value is unrepresentable (D4 ¶2 made structural).
#[derive(Clone, Debug)]
pub struct EdgeCurve<T: Real> {
    /// U2's collapsed description — ONE conventional form, minted and
    /// metered at certification (D1/D4).
    description: EdgeDescription<T>,
    /// U2 Q3's per-edge authority record: who determined the locus.
    authority: EdgeAuthority<T>,
    carrier: Curve3<T>,
    param_start: T,
    param_end: T,
    certificate: Certificate<T>,
}

impl<T: Decide> EdgeCurve<T> {
    /// Certifies `spec` against the edge's endpoint points and the
    /// owning body's surfaces, returning the certified carrier.
    ///
    /// `start`/`end` are the points of `start(he_plus)` and
    /// `end(he_plus)` (the `he_plus` forward contract); `surfaces`
    /// resolves the description's arena keys (injected by the owning
    /// body — this layer never touches arenas, see [`crate::keys`]);
    /// `band` is the run's linear band (callers build it once at
    /// operation entry via `Band::linear(tol)`).
    ///
    /// The check sequence (fixed order, D9; every check's margin is
    /// meters against `band`):
    ///
    /// 1. Implementedness: described surfaces resolve and are not
    ///    `Nurbs`; a `Nurbs` carrier certifies only under an
    ///    `Intersection` description (the rung-3 class, M5 PR 9 —
    ///    its span is metered through `speed_lower_bound`, gated
    ///    definitely-positive by `nurbs_span_meter`); `Intersection`'s
    ///    two surfaces are distinct; `Seam`'s surface is periodic.
    /// 2. Interval span (M2 PR 3 fix pass): the stored interval is
    ///    **forward** — its arc length `(t₁ − t₀)` (`·r` for circle
    ///    carriers, metering radians into meters) is definitely
    ///    positive ([`CertifyError::IntervalNotForward`] otherwise —
    ///    decreasing *and* degenerate zero-span intervals refuse); for
    ///    circle carriers additionally `(τ − Δt)·r` is not definitely
    ///    negative ([`CertifyError::WindingExceeded`]) — at most one
    ///    full period, which closes the 9-sample `8kτ` winding-alias
    ///    family (see that variant's docs). In-band/poisoned span
    ///    margins escalate under [`CertCheck::ParamSpan`].
    /// 3. Endpoint pinning: `|carrier(t₀) − start| ≤ ε`,
    ///    `|carrier(t₁) − end| ≤ ε`.
    /// 4. Per-sample description residuals, samples i = 0…8 in order
    ///    (per-sample check order as listed in [`CertCheck`]):
    ///    - `Intersection`: implicit residual vs `s1`, then `s2`; at
    ///      interior samples (i = 1…7) the transversality margin
    ///      (definitely transverse required), metered through the
    ///      edge's honest extent ([`edge_extent`] — carrier diameter,
    ///      not the collapsing chord, for closed circle carriers).
    ///    - `MappedCurve`: `|carrier(t_i) − description(i/8)|`.
    ///    - `Seam`: implicit residual, halfplane residual `|w·v_ref|`,
    ///      wrong-side excess `max(0, −w·u_ref)`.
    /// 5. `Intersection`: the witness's implicit residuals vs both
    ///    surfaces, then the **mid-parameter pin**
    ///    `|carrier((t₀+t₁)/2) − witness| ≤ ε`
    ///    ([`CertCheck::WitnessMidpoint`]): the witness contract is
    ///    that the stored witness IS the edge's mid-parameter point —
    ///    constructors mint it as `carrier(mid)` (the upgrade helpers'
    ///    chord midpoint) — which pins the traversed arc and winding
    ///    direction between the pinned endpoints. Residual freedom
    ///    after checks 2–5: for circle carriers the interval is
    ///    determined up to a joint whole-period translation of both
    ///    ends (geometrically invisible); for lines it is fully
    ///    determined. *Which connected component* of the intersection
    ///    locus the witness selects remains unverifiable before
    ///    marching exists (M3) — the mid-pin verifies the carrier
    ///    traverses the witness's arc, not that the witness sits on
    ///    the component the modeler intended.
    ///
    /// # Errors
    ///
    /// The first failing check, as a typed [`CertifyError`] (D4 ¶3).
    pub fn certify(
        spec: EdgeCurveSpec<T>,
        start: Point3<T>,
        end: Point3<T>,
        surfaces: impl Fn(SurfaceKey) -> Option<Surface<T>>,
        band: Band,
    ) -> Result<Self, CertifyError> {
        let (certificate, canonical) = run_checks(&spec, start, end, &surfaces, None, band)?;
        Ok(Self {
            authority: authority_of(&spec.description),
            description: canonical,
            carrier: spec.carrier,
            param_start: spec.param_start,
            param_end: spec.param_end,
            certificate,
        })
    }

    /// Re-runs the full certification of this carrier at rest — the
    /// tier-3 validator's per-edge pass. Same checks, same schedule,
    /// same errors as [`EdgeCurve::certify`]; the stored certificate is
    /// not consulted (re-certification re-derives, it does not trust).
    ///
    /// # Errors
    ///
    /// As [`EdgeCurve::certify`].
    pub fn recertify(
        &self,
        start: Point3<T>,
        end: Point3<T>,
        surfaces: impl Fn(SurfaceKey) -> Option<Surface<T>>,
        band: Band,
    ) -> Result<Certificate<T>, CertifyError> {
        run_checks(&self.spec(), start, end, &surfaces, None, band).map(|(cert, _)| cert)
    }
}

/// The **injected plane × NURBS lane** — the one certification duty
/// this module cannot discharge from `T: Decide` alone.
///
/// Limb 2 and limb 3 of the plane × NURBS certificate are C9-ring hull
/// bounds and the foot point is a bracket read, so the honest
/// derivation needs `T: Decide + Bounds + CertifiedEnclosure`
/// ([`crate::EdgeNurbsLane`]'s static split — since #643 the ring door
/// is `CertifiedEnclosure`, which is what a `Dual` lacks; it has had
/// `Bounds` since D1, 2026-08-19). Raising `certify`'s own
/// bound would push `Bounds` through every `T: Decide` signature in
/// `topo` — hundreds of them, for a capability three of the four
/// sealed scalars have unconditionally. So the capability is
/// **injected at the door** instead, exactly as the surface arena is:
/// a caller that can derive the certificate hands one in, and a caller
/// that cannot passes `None` and gets the same
/// [`CertifyError::Unimplemented`] refusal a described `Nurbs` operand
/// has always produced. There is no third outcome — no door accepts
/// the description without the certificate.
pub type NurbsLane<'a, T> = &'a dyn Fn(
    &geom::NurbsCurve3<T>,
    &Surface<T>,
    &geom::NurbsSurface<T>,
    T,
    Band,
) -> Result<
    crate::edge_nurbs::PlaneNurbsLimbs<T>,
    crate::edge_nurbs::PlaneNurbsRefusal,
>;

impl<T: crate::edge_nurbs::EdgeNurbsLane> EdgeCurve<T> {
    /// [`EdgeCurve::certify`] **with the plane × NURBS lane wired in**
    /// ([`NurbsLane`]): the door for callers whose scalar can derive
    /// the declare-and-check certificate of an `Intersection` between
    /// a PLANE and a described NURBS wall (M7-8).
    ///
    /// Every other check is identical, in the same order. The dual
    /// scalar reaches this door too and its refusing lane impl answers
    /// there, so the outcome is typed rather than absent.
    ///
    /// # Errors
    ///
    /// As [`EdgeCurve::certify`], plus [`CertifyError::PlaneNurbs`]
    /// carrying the lane's measured bound.
    pub fn certify_nurbs_lane(
        spec: EdgeCurveSpec<T>,
        start: Point3<T>,
        end: Point3<T>,
        surfaces: impl Fn(SurfaceKey) -> Option<Surface<T>>,
        band: Band,
    ) -> Result<Self, CertifyError> {
        let (certificate, canonical) = run_checks(
            &spec,
            start,
            end,
            &surfaces,
            Some(&T::plane_nurbs_limbs),
            band,
        )?;
        Ok(Self {
            authority: authority_of(&spec.description),
            description: canonical,
            carrier: spec.carrier,
            param_start: spec.param_start,
            param_end: spec.param_end,
            certificate,
        })
    }

    /// [`EdgeCurve::recertify`] with the plane × NURBS lane wired in
    /// — the at-rest pass for a body that may carry the M7-8 class.
    ///
    /// # Errors
    ///
    /// As [`EdgeCurve::certify_nurbs_lane`].
    pub fn recertify_nurbs_lane(
        &self,
        start: Point3<T>,
        end: Point3<T>,
        surfaces: impl Fn(SurfaceKey) -> Option<Surface<T>>,
        band: Band,
    ) -> Result<Certificate<T>, CertifyError> {
        run_checks(
            &self.spec(),
            start,
            end,
            &surfaces,
            Some(&T::plane_nurbs_limbs),
            band,
        )
        .map(|(cert, _)| cert)
    }
}

impl<T: Real> EdgeCurve<T> {
    /// The intensional description (authoritative, D2/U2): D2's two
    /// intrinsic arms, ONE conventional form, and the fenced
    /// scaffolding door.
    ///
    /// Handed out by reference, never by value: [`EdgeDescription`]
    /// carries a [`crate::Pcurve`] and is therefore not `Copy` — an
    /// edge description is read, not moved around.
    pub fn description(&self) -> &EdgeDescription<T> {
        &self.description
    }

    /// The **authority record** (U2 Q3): whether a modeler DECLARED
    /// this locus, and with what sketch source. The datum tier 3's
    /// prefer-intrinsic rules read instead of `MappedCurve`'s
    /// negative space.
    pub fn authority(&self) -> EdgeAuthority<T> {
        self.authority
    }

    /// The cached carrier curve (a certified derived cache, D4 ¶2).
    pub fn carrier(&self) -> &Curve3<T> {
        &self.carrier
    }

    /// The certified carrier-parameter interval `(t₀, t₁)` —
    /// `he_plus`-forward, a certified cache of the vertex-derived
    /// bounds (module docs).
    pub fn params(&self) -> (T, T) {
        (self.param_start, self.param_end)
    }

    /// The attachment-time certification record.
    pub fn certificate(&self) -> &Certificate<T> {
        &self.certificate
    }

    /// The same certified carrier with its description's **SURFACE
    /// KEYS** rewritten, for a transplant into another body's arenas.
    ///
    /// A surface key is an arena handle, not geometry: the two
    /// intrinsic arms and the chart arm name the surfaces their locus
    /// is stated against, and a graft that re-creates
    /// those surfaces BITWISE under fresh keys has changed the handles
    /// and nothing else. The certificate — a residual over the
    /// description, the carrier, the interval and the surfaces' VALUES
    /// — is therefore still the certificate of exactly this geometry,
    /// and travels verbatim, like provenance.
    ///
    /// This is the only door that mints an `EdgeCurve` without a run
    /// of the schedule, and it is narrow on purpose: nothing but the
    /// keys may differ, so it cannot express a geometry change. Its
    /// existence is what lets a transplant carry descriptions whose
    /// surfaces the certification lanes cannot re-certify at all (a
    /// rational NURBS wall certifies nowhere — see
    /// `CertifyError::Unimplemented`), which is not a licence to
    /// invent a certificate: the source body's run is the certificate.
    ///
    /// `None` when `remap` does not answer for a key the description
    /// names — a dangling handle is never written.
    #[must_use]
    pub fn with_remapped_surfaces(
        &self,
        mut remap: impl FnMut(crate::keys::SurfaceKey) -> Option<crate::keys::SurfaceKey>,
    ) -> Option<Self> {
        let description = match self.description {
            EdgeDescription::Intersection { s1, s2, witness } => EdgeDescription::Intersection {
                s1: remap(s1)?,
                s2: remap(s2)?,
                witness,
            },
            EdgeDescription::TangentIntersection { s1, s2, witness } => {
                EdgeDescription::TangentIntersection {
                    s1: remap(s1)?,
                    s2: remap(s2)?,
                    witness,
                }
            }
            // The chart IMAGE travels verbatim, because it is geometry
            // stated in chart COORDINATES, which a bitwise re-creation
            // of the surface leaves untouched. Only the handle moves.
            EdgeDescription::Chart(ref c) => EdgeDescription::Chart(ChartCurve {
                surface: remap(c.surface)?,
                pcurve: c.pcurve.clone(),
                seam: c.seam,
            }),
            // A scaffold names no surface — there is none yet.
            EdgeDescription::Scaffold(m) => EdgeDescription::Scaffold(m),
        };
        Some(Self {
            description,
            authority: self.authority,
            carrier: self.carrier.clone(),
            param_start: self.param_start,
            param_end: self.param_end,
            certificate: self.certificate,
        })
    }

    /// The carrier parameter at schedule sample `i` (i ∈ 0…8):
    /// `t₀ + (t₁ − t₀)·(i/8)`, the module-doc schedule. Exposed so the
    /// tier-3 validator samples the *same* parameters the certification
    /// did (D9).
    pub fn sample_param(&self, i: u32) -> T {
        sample_param(self.param_start, self.param_end, i)
    }
}

impl<T: SpanLocate> EdgeCurve<T> {
    /// The two **uncertified child specs** of splitting this certified
    /// carrier at interior parameter `t` (M3 PR 1, for `split_edge`):
    /// the carrier is unchanged and the interval splits at `t`
    /// (`[t₀, t]` / `[t, t₁]` — both forward for interior `t`, so the
    /// forward-span gate is untouched); the description splits
    /// honestly per kind — `MappedCurve` via
    /// [`crate::MappedCurve::restrict`] at the interval fractions,
    /// `Intersection` keeps its surfaces with each child's witness
    /// re-minted as its own mid-parameter carrier point (the witness
    /// contract, bitwise the certification schedule's middle sample),
    /// `Seam` is unchanged (a sub-arc of the seam locus is on the seam
    /// locus).
    ///
    /// The children are *specs*, not certified carriers: the caller
    /// must run each through [`EdgeCurve::certify`] against its own
    /// endpoints (D4 ¶2 — the restriction arithmetic is verified, not
    /// trusted). Interiority of `t` is likewise the caller's trilean
    /// obligation; this function is total arithmetic.
    pub fn split_specs(&self, t: T) -> (EdgeCurveSpec<T>, EdgeCurveSpec<T>) {
        let (t0, t1) = (self.param_start, self.param_end);
        let span = t1 - t0;
        let a = (t - t0) / span;
        let child = |s0: T, s1: T, ta: T, tb: T| -> EdgeCurveSpec<T> {
            let description = match self.description {
                EdgeDescription::Intersection { s1: k1, s2: k2, .. } => {
                    EdgeDescriptionSpec::Intersection {
                        s1: k1,
                        s2: k2,
                        // The child's mid-parameter point, computed
                        // exactly as the certification schedule's
                        // middle sample (bitwise — zero
                        // WitnessMidpoint residual).
                        witness: self
                            .carrier
                            .eval(sample_param(ta, tb, (CERT_SAMPLES - 1) / 2)),
                    }
                }
                // TangentIntersection splits exactly as Intersection:
                // surfaces kept, witness re-minted at the child's own
                // mid-parameter (the witness contract, one order up).
                EdgeDescription::TangentIntersection { s1: k1, s2: k2, .. } => {
                    EdgeDescriptionSpec::TangentIntersection {
                        s1: k1,
                        s2: k2,
                        witness: self
                            .carrier
                            .eval(sample_param(ta, tb, (CERT_SAMPLES - 1) / 2)),
                    }
                }
                EdgeDescription::Scaffold(mc) => {
                    EdgeDescriptionSpec::Scaffold(mc.restrict(s0, s1))
                }
                // A chart image is a function of the CARRIER's own
                // parameter, and splitting an edge changes the
                // interval, not the carrier — so the child's image is
                // the parent's image, verbatim. Stating it exactly is
                // what keeps the sub-arc's description the restriction
                // of its parent's rather than a re-derivation that can
                // land a few ulps away from it.
                EdgeDescription::Chart(ref c) => EdgeDescriptionSpec::Chart {
                    surface: c.surface,
                    image: Some(c.pcurve.clone()),
                    seam: c.seam,
                    declared: match self.authority {
                        EdgeAuthority::Declared(mc) => Some(mc.restrict(s0, s1)),
                        EdgeAuthority::Derived => None,
                    },
                },
            };
            EdgeCurveSpec {
                description,
                carrier: self.carrier.clone(),
                param_start: ta,
                param_end: tb,
            }
        };
        (child(T::zero(), a, t0, t), child(a, T::one(), t, t1))
    }

    /// This carrier's spec view (for re-certification): the certified
    /// description stated back as a construction states one. A chart
    /// image travels EXACTLY — a re-run must re-meter the image the
    /// edge already carries, not derive a second one and meter that.
    fn spec(&self) -> EdgeCurveSpec<T> {
        let description = match self.description {
            EdgeDescription::Intersection { s1, s2, witness } => {
                EdgeDescriptionSpec::Intersection { s1, s2, witness }
            }
            EdgeDescription::TangentIntersection { s1, s2, witness } => {
                EdgeDescriptionSpec::TangentIntersection { s1, s2, witness }
            }
            EdgeDescription::Chart(ref c) => EdgeDescriptionSpec::Chart {
                surface: c.surface,
                image: Some(c.pcurve.clone()),
                seam: c.seam,
                declared: match self.authority {
                    EdgeAuthority::Declared(mc) => Some(mc),
                    EdgeAuthority::Derived => None,
                },
            },
            EdgeDescription::Scaffold(mc) => EdgeDescriptionSpec::Scaffold(mc),
        };
        EdgeCurveSpec {
            description,
            carrier: self.carrier.clone(),
            param_start: self.param_start,
            param_end: self.param_end,
        }
    }
}

/// The schedule parameter `t₀ + (t₁ − t₀)·(i/8)` (exact dyadic
/// fraction; fixed association order, D9).
///
/// Public because the contact vocabulary's tangency verification runs
/// the SAME schedule over a locus that is not an edge yet
/// (`topo::boolean::contact_verify`): two samplers over one schedule
/// is the twin this export exists to prevent.
pub fn sample_param<T: Real>(t0: T, t1: T, i: u32) -> T {
    let frac = T::from_f64(f64::from(i) / f64::from(CERT_SAMPLES - 1));
    t0 + (t1 - t0) * frac
}

/// The honest spatial **extent** of an edge — the lever arm the
/// dihedral/transversality classification meters angles through
/// (D4 ¶1), replacing the bare chord (M2 PR 3 fix pass, B2).
///
/// # Derivation (the lever-arm rationale)
///
/// The extent an angular defect accumulates over is the **diameter of
/// the edge's point set** — the farthest distance between two of its
/// points. For open edges the chord `|end − start|` is that diameter
/// (exactly, for lines and for circular arcs up to a half period). But
/// for closed and near-closed edges the chord collapses to ~0 while the
/// true extent stays the carrier's diameter — folding the raw chord in
/// turned a full-period 90° rim into "definitely Smooth" (a definite
/// wrong answer) and made tier 3's dihedral pass vacuous on self-loops.
///
/// Per carrier kind, this returns a **certified lower bound** on the
/// point-set diameter (a lower bound is the safe direction: a smaller
/// arm escalates more, never definitely misclassifies):
///
/// - **Line** — the chord (exact).
/// - **Ellipse** (semi-axes a > b, span Δt): `max(chord,
///   b·(1 − cos(Δt/2)))` — the circle fold at the MINOR semi-axis. A
///   lower bound because the ellipse is the image of the radius-b
///   circle (same parameter θ) under a one-axis expansion by a/b ≥ 1,
///   which never shrinks a distance — so the ellipse arc's point-set
///   diameter dominates the b-circle arc's, and the circle bound below
///   applies to that circle verbatim.
/// - **Circle** (radius r, span Δt = t₁ − t₀): `max(chord,
///   r·(1 − cos(Δt/2)))`. The second term is the arc's sagitta-style
///   bulge height, an even, branch-free function of Δt with the two
///   properties the fold needs: it never exceeds the true diameter
///   (for |Δt| ≤ π, `1 − cos(u) ≤ 2·sin(u)` on `u ∈ [0, π/2]`, i.e.
///   `tan(u/2) ≤ 2`, so it is below the chord's own `2r·sin(Δt/2)`;
///   for |Δt| ∈ (π, τ] it is ≤ 2r, the diameter), and at closure
///   (Δt → τ, chord → 0) it reaches the full carrier diameter `2r` —
///   the honest extent of a full-period rim. The winding bound
///   (|Δt| ≤ τ, [`CertifyError::WindingExceeded`]) keeps the cosine in
///   its honest range.
/// - **Nurbs** — the chord: a certified lower bound on the point-set
///   diameter for the open fitted branches the zip mints (M5 PR 9);
///   closed rung-3 loops are split at crossing vertices before they
///   become edges, so the collapsing-chord case does not arise at
///   rest — and a smaller arm only escalates more (the safe
///   direction), never definitely misclassifies.
///
/// Total and comparison-free (`max` is the [`Real`] lattice operation);
/// used by certification's transversality check and re-used verbatim by
/// `topo`'s tier-3 dihedral pass (same numbers at rest, D9).
pub fn edge_extent<T: Real>(carrier: &Curve3<T>, t0: T, t1: T, chord: T) -> T {
    match *carrier {
        Curve3::Circle { radius, .. } => {
            let half_span = (t1 - t0) * T::from_f64(0.5);
            chord.max(radius * (T::one() - half_span.cos()))
        }
        // The minor semi-axis is the certified direction (doc above):
        // the ellipse dominates its minor-radius circle pointwise.
        Curve3::Ellipse { minor, .. } => {
            let half_span = (t1 - t0) * T::from_f64(0.5);
            chord.max(minor * (T::one() - half_span.cos()))
        }
        Curve3::Line { .. } | Curve3::Nurbs(_) => chord,
    }
}

/// The carrier's kind, for a refusal that has to name the pair it
/// could not state (the chart side is `chart_name`'s).
fn carrier_kind<T: Real>(carrier: &Curve3<T>) -> &'static str {
    match carrier {
        Curve3::Line { .. } => "line",
        Curve3::Circle { .. } => "circle",
        Curve3::Ellipse { .. } => "ellipse",
        Curve3::Nurbs(_) => "Nurbs",
    }
}

/// Folds a residual into the running max and classifies it: must be
/// coincident with zero (|r| ≤ ε). Positive/Negative beyond the band ⇒
/// [`CertifyError::ResidualExceeded`]; in-band or poisoned ⇒
/// [`CertifyError::Escalated`].
fn check_residual<T: Decide>(
    name: &'static str,
    check: CertCheck,
    sample: u32,
    residual: Margin<T>,
    band: Band,
    max_residual: &mut T,
) -> Result<(), CertifyError> {
    *max_residual = max_residual.max(residual.value().abs());
    match decide(name, residual, band) {
        Ok(Sign::Zero) => Ok(()),
        Ok(Sign::Positive | Sign::Negative) => {
            Err(CertifyError::ResidualExceeded { check, sample })
        }
        Err(cause) => Err(CertifyError::Escalated {
            check,
            sample,
            cause,
        }),
    }
}

/// The shared certification engine (check sequence documented on
/// [`EdgeCurve::certify`]).
fn run_checks<T: Decide>(
    spec: &EdgeCurveSpec<T>,
    start: Point3<T>,
    end: Point3<T>,
    surfaces: &impl Fn(SurfaceKey) -> Option<Surface<T>>,
    lane: Option<NurbsLane<'_, T>>,
    band: Band,
) -> Result<(Certificate<T>, EdgeDescription<T>), CertifyError> {
    // ---- Check 1: implementedness / description well-formedness. ----
    // Rung-3 (`Nurbs`) carriers certify under an intrinsic description
    // of two ANALYTIC surfaces — the class the curved boolean zip
    // mints (M5 PR 9, C12.3; the fitted SSI branch) — and under a
    // chart description whose image the CONSTRUCTION states (M6-3: the
    // loft/sweep wall–wall seam class, whose residual is the genuinely
    // metric `|C(t) − S(P(t))|`). A `Nurbs` carrier under a chart
    // description whose image would have to be DERIVED stays refused,
    // and so does one under the scaffolding door: nothing mints
    // either, and neither has a certified meter — a derived image
    // needs the analytic chart machinery, and a fitted carrier
    // "matching" a mapped source states no residual at all.
    if matches!(spec.carrier, Curve3::Nurbs(_))
        && !matches!(
            spec.description,
            EdgeDescriptionSpec::Intersection { .. }
                | EdgeDescriptionSpec::TangentIntersection { .. }
                | EdgeDescriptionSpec::Chart { image: Some(_), .. }
        )
    {
        return Err(CertifyError::Unimplemented);
    }
    // `Approx` refuses here with `Nurbs`, and for the same reason: the
    // descriptions this resolver serves (`Intersection`, `Seam`) state
    // their residual through the IMPLICIT form, which a spline
    // stand-in does not have. Admitting one would meter poison.
    let resolve = |key: SurfaceKey| -> Result<Surface<T>, CertifyError> {
        let s = surfaces(key).ok_or(CertifyError::UnresolvedSurface { key })?;
        if matches!(s, Surface::Nurbs(_) | Surface::Approx(_)) {
            return Err(CertifyError::Unimplemented);
        }
        Ok(s)
    };
    // The iso lane's resolver (M6-3): an `IsoCurve` description names
    // its surface as the CHART of the residual, so a DESCRIBED
    // `Surface::Nurbs` is exactly what it evaluates against — admitted
    // here and nowhere else. The mvfs placeholder (all-poison control)
    // is not a described surface and keeps refusing.
    let resolve_iso = |key: SurfaceKey| -> Result<Surface<T>, CertifyError> {
        let s = surfaces(key).ok_or(CertifyError::UnresolvedSurface { key })?;
        if let Surface::Nurbs(ref payload) = s
            && payload.is_placeholder()
        {
            return Err(CertifyError::Unimplemented);
        }
        Ok(s)
    };
    enum Resolved<T: Real> {
        Intersection {
            surf1: Surface<T>,
            surf2: Surface<T>,
            witness: Point3<T>,
        },
        Tangent {
            surf1: Surface<T>,
            surf2: Surface<T>,
            witness: Point3<T>,
        },
        /// D3's fenced scaffolding door: a pushforward standing in
        /// as a description while the edge is TRANSIENT.
        Scaffold(crate::mapped::MappedCurve<T>),
        /// U2's ONE conventional form: the chart, its arena key, and
        /// the chart-image data the pcurve is minted from (D4 — the
        /// mint runs after the interval checks, so a degenerate span
        /// still refuses in its own order).
        Chart {
            surface: Surface<T>,
            key: SurfaceKey,
            /// `Some` for an image the CONSTRUCTION states (D4:
            /// spline charts take the image from the caller, which
            /// every iso constructor already knows, and a
            /// re-certification restates the image the edge already
            /// carries); `None` for an analytic chart, whose image is
            /// minted here through [`crate::chart_pcurve`].
            image: Option<Pcurve<T>>,
            /// D1's obligation: this edge claims to BE the chart's
            /// seam meridian.
            seam: bool,
        },
        /// `Intersection` of a PLANE and a described NURBS wall
        /// (M7-8): the declare-and-check lane's shape.
        PlaneNurbs {
            plane: Surface<T>,
            wall: std::sync::Arc<geom::NurbsSurface<T>>,
            witness: Point3<T>,
        },
    }
    let resolved = match spec.description {
        EdgeDescriptionSpec::Intersection { s1, s2, witness } => {
            if s1 == s2 {
                return Err(CertifyError::IntersectionSameSurface { key: s1 });
            }
            // The plane × NURBS lane (M7-8) is tried FIRST, because it
            // is the only reading under which a described `Nurbs`
            // operand certifies at all: `resolve` below refuses one
            // typed, and did so unconditionally before this unit. The
            // pairing must be exactly one PLANE and one described NURBS
            // wall — a NURBS × NURBS `Intersection` still has no
            // certificate (the C5 table's general rung), and its
            // refusal is the same `Unimplemented` as ever.
            if let Some((plane, wall)) =
                lane.and_then(|_| plane_nurbs_pair(surfaces(s1), surfaces(s2)))
            {
                Resolved::PlaneNurbs {
                    plane,
                    wall,
                    witness,
                }
            } else {
                Resolved::Intersection {
                    surf1: resolve(s1)?,
                    surf2: resolve(s2)?,
                    witness,
                }
            }
        }
        EdgeDescriptionSpec::TangentIntersection { s1, s2, witness } => {
            // A same-surface "tangency" is a seam exactly as a
            // same-surface intersection is (D2's taxonomy).
            if s1 == s2 {
                return Err(CertifyError::IntersectionSameSurface { key: s1 });
            }
            Resolved::Tangent {
                surf1: resolve(s1)?,
                surf2: resolve(s2)?,
                witness,
            }
        }
        EdgeDescriptionSpec::Scaffold(mc) => Resolved::Scaffold(mc),
        // The chart arm resolves through the door its IMAGE needs. A
        // seam's image is derived from the analytic chart machinery,
        // so its surface must BE analytic and periodic — the plane is
        // the one non-periodic analytic kind (`Nurbs` already
        // rejected above). An image the construction states is
        // evaluated on the chart itself, so a described spline chart
        // is exactly what it needs and is admitted here and nowhere
        // else (the mvfs placeholder, an all-poison control, is not a
        // described surface and keeps refusing).
        EdgeDescriptionSpec::Chart {
            surface,
            ref image,
            seam,
            ..
        } => {
            let s = if seam {
                let s = resolve(surface)?;
                if matches!(s, Surface::Plane { .. }) {
                    return Err(CertifyError::SeamOnNonPeriodic);
                }
                s
            } else if image.is_some() {
                resolve_iso(surface)?
            } else {
                resolve(surface)?
            };
            Resolved::Chart {
                surface: s,
                key: surface,
                image: image.clone(),
                seam,
            }
        }
    };

    let mut max_residual = T::zero();
    let (t0, t1) = (spec.param_start, spec.param_end);

    // ---- Check 2: interval span (forward direction; circle winding
    // bound) — see the check-sequence docs. Spans are metered as arc
    // length (radians × radius for circles) so they classify against
    // the linear band like every other margin (dimensional honesty).
    let span = t1 - t0;
    let span_escalated = |cause: Indeterminate| CertifyError::Escalated {
        check: CertCheck::ParamSpan,
        sample: 0,
        cause,
    };
    match &spec.carrier {
        Curve3::Circle { radius, .. } => {
            let arc = Margin::levered(span, *radius);
            match decide("interval_span_forward", arc, band).map_err(span_escalated)? {
                Sign::Positive => {}
                Sign::Zero | Sign::Negative => return Err(CertifyError::IntervalNotForward),
            }
            // Winding bound: the remaining headroom to one full period.
            // Zero (exactly full period, the scaffolding/rim case) and
            // Positive (a partial arc) both pass; definitely negative
            // is the alias family.
            let headroom = Margin::levered(T::tau() - span, *radius);
            match decide("interval_span_winding", headroom, band).map_err(span_escalated)? {
                Sign::Positive | Sign::Zero => {}
                Sign::Negative => return Err(CertifyError::WindingExceeded),
            }
        }
        // Ellipse spans are metered at the MINOR semi-axis — the
        // conservative meter (|dP/dθ| ≥ minor, so `span·minor` is a
        // certified lower bound on the child's arc length: a span this
        // gate accepts as forward is truly forward, and near-threshold
        // spans escalate rather than sneak through). The same winding
        // bound applies: the 8kτ sample-alias argument is about the
        // parameter period, which the ellipse shares with the circle.
        Curve3::Ellipse { minor, .. } => {
            let arc = Margin::levered(span, *minor);
            match decide("interval_span_forward", arc, band).map_err(span_escalated)? {
                Sign::Positive => {}
                Sign::Zero | Sign::Negative => return Err(CertifyError::IntervalNotForward),
            }
            let headroom = Margin::levered(T::tau() - span, *minor);
            match decide("interval_span_winding", headroom, band).map_err(span_escalated)? {
                Sign::Positive | Sign::Zero => {}
                Sign::Negative => return Err(CertifyError::WindingExceeded),
            }
        }
        Curve3::Line { .. } => {
            match decide("interval_span_forward", Margin::of(span), band).map_err(span_escalated)? {
                Sign::Positive => {}
                Sign::Zero | Sign::Negative => return Err(CertifyError::IntervalNotForward),
            }
        }
        // Rung-3 carriers (M5 PR 9): the span is metered through the
        // certified speed lower bound (`m/parameter`, the split-meter
        // substrate — C12.3). The meter is gated definitely-positive
        // FIRST (the collapsed-arm idiom): a zero/negative meter (a
        // carrier whose speed genuinely collapses) or poison (a
        // malformed net) cannot convert the span to metres, and no
        // forward verdict may be fabricated from it — escalate, never
        // guess. RATIONAL carriers used to land here unconditionally;
        // since M7 they have their own arm of the meter and state a
        // real bound (`speed_lower_bound`'s rational derivation).
        //
        // The gated quantity is a LENGTH, not the bare rate: the rate
        // is metres per parameter unit, so what must be definitely
        // positive is the metre extent it subtends over the carrier's
        // OWN knot domain — a lower bound on the net's arc length.
        // That comparand is reparametrization-invariant (a carrier
        // reparametrized `t → 2t` halves the rate and doubles the
        // domain), which the bare rate is not, and it is the quantity
        // ε classifies under D4. The two failure modes stay distinct:
        // a collapsed or poison meter answers `Invalid`/escalates,
        // while a backwards span is `IntervalNotForward` below.
        Curve3::Nurbs(n) => {
            let meter = n.speed_lower_bound();
            let (d0, d1) = n.domain();
            let net_length = Margin::metered(T::from_f64(d1 - d0), meter);
            match decide("nurbs_span_meter", net_length, band).map_err(span_escalated)? {
                Sign::Positive => {}
                Sign::Zero | Sign::Negative => {
                    return Err(span_escalated(Indeterminate {
                        margin: geom_core::MarginDiag::Invalid,
                        band,
                        predicate: Some("nurbs_span_meter"),
                    }));
                }
            }
            let arc = Margin::metered(span, meter);
            match decide("interval_span_forward", arc, band).map_err(span_escalated)? {
                Sign::Positive => {}
                Sign::Zero | Sign::Negative => return Err(CertifyError::IntervalNotForward),
            }
        }
    }

    // ---- Check 3: endpoint pinning. ----
    check_residual(
        "carrier_endpoint_start",
        CertCheck::EndpointStart,
        0,
        Margin::of(spec.carrier.eval(t0).distance(start)),
        band,
        &mut max_residual,
    )?;
    check_residual(
        "carrier_endpoint_end",
        CertCheck::EndpointEnd,
        CERT_SAMPLES - 1,
        Margin::of(spec.carrier.eval(t1).distance(end)),
        band,
        &mut max_residual,
    )?;

    // ---- The chart image (U2/D4): the certified description. ----
    // A derived image is minted HERE, after the interval checks and
    // before the residual schedule, so a degenerate span still refuses
    // in its own order and no chart image is derived from an interval
    // the kernel has not yet accepted.
    let canonical: EdgeDescription<T> = match resolved {
        Resolved::Intersection { witness, .. } | Resolved::PlaneNurbs { witness, .. } => {
            let EdgeDescriptionSpec::Intersection { s1, s2, .. } = spec.description else {
                // Unreachable: exactly one spec arm resolves either
                // way. Typed rather than assumed (D4 ¶2).
                return Err(CertifyError::Unimplemented);
            };
            EdgeDescription::Intersection { s1, s2, witness }
        }
        Resolved::Tangent { witness, .. } => {
            let EdgeDescriptionSpec::TangentIntersection { s1, s2, .. } = spec.description else {
                return Err(CertifyError::Unimplemented);
            };
            EdgeDescription::TangentIntersection { s1, s2, witness }
        }
        Resolved::Scaffold(mc) => EdgeDescription::Scaffold(mc),
        Resolved::Chart {
            ref surface,
            key,
            ref image,
            seam,
        } => {
            let pcurve = match image {
                // D4, the stated half: a spline chart's image IS the
                // constructor's own iso data, exactly, and a
                // re-certification restates the image the edge already
                // carries. `v` is affine in the carrier parameter, so
                // the chart image is `P(t) = (u, v0 + slope·(t − t0))`
                // written on the carrier's own parameter — the form
                // every stored cache of this class already carries,
                // which is what makes description and cache the same
                // object.
                Some(p) => p.clone(),
                // D4, the minted half: an analytic chart's image is
                // derived from the carrier through the one door that
                // derives chart images anywhere in this kernel.
                None => chart_pcurve(&spec.carrier, surface, band).map_err(|e| match e {
                    // The mint runs ONCE, before the schedule, so the
                    // sample field carries the not-a-sample sentinel:
                    // reporting `0` would name a schedule point that
                    // was never visited.
                    PcurveCertifyError::Escalated { cause, .. } => CertifyError::Escalated {
                        check: CertCheck::ChartImage,
                        sample: NOT_A_SAMPLE,
                        cause,
                    },
                    // Every other refusal says the same thing: this
                    // (chart, carrier) pair has no certified chart
                    // image. That is a fact about the DESCRIPTION's
                    // geometry, and it is reported as one — not as
                    // `Unimplemented`, which means something else and
                    // would send the caller looking for a missing
                    // feature instead of a wrong locus.
                    _ => CertifyError::ChartImageUnavailable {
                        chart: crate::pcurve_cache::chart_name(surface),
                        carrier: carrier_kind(&spec.carrier),
                    },
                })?,
            };
            EdgeDescription::Chart(ChartCurve {
                surface: key,
                pcurve,
                seam,
            })
        }
    };

    // ---- Check 4: per-sample description residuals. ----
    // The transversality extent arm is the edge's honest spatial
    // extent — the chord where it is honest, the carrier-derived
    // diameter for closed circle carriers ([`edge_extent`]'s docs).
    let chord = start.distance(end);
    let extent = edge_extent(&spec.carrier, t0, t1, chord);
    // Jet-schedule accumulators (TangentIntersection only): the worst
    // sampled residual for the C2.2 hull statement, and the weakest
    // sampled second-order data for the tube statement.
    let mut tangent_resid_max = T::zero();
    let mut tangent_kappa_min = T::from_f64(f64::MAX);
    let mut tangent_arm_min = T::from_f64(f64::MAX);
    for i in 0..CERT_SAMPLES {
        let p = spec.carrier.eval(sample_param(t0, t1, i));
        match &resolved {
            Resolved::Intersection { surf1, surf2, .. } => {
                check_residual(
                    "carrier_on_surface_1",
                    CertCheck::Surface1Residual,
                    i,
                    Margin::of(implicit_residual(surf1, p)),
                    band,
                    &mut max_residual,
                )?;
                check_residual(
                    "carrier_on_surface_2",
                    CertCheck::Surface2Residual,
                    i,
                    Margin::of(implicit_residual(surf2, p)),
                    band,
                    &mut max_residual,
                )?;
                if i > 0 && i < CERT_SAMPLES - 1 {
                    match classify_dihedral(surf1, surf2, p, extent, band) {
                        Ok(DihedralClass::Transverse) => {}
                        Ok(DihedralClass::Smooth) => {
                            return Err(CertifyError::NotTransverse { sample: i });
                        }
                        Err(cause) => {
                            return Err(CertifyError::Escalated {
                                check: CertCheck::Transversality,
                                sample: i,
                                cause,
                            });
                        }
                    }
                }
            }
            // The C7 jet schedule (M5 PR 9), per sample: both implicit
            // residuals within ε; at interior samples the second-order
            // margin definitely positive FIRST (the IFT denominator —
            // and the parallelism lever arm's own validity gate), then
            // normal parallelism within the derived threshold at lever
            // arm r = 1/κ_rel (D2 verbatim, D4 ¶1).
            Resolved::Tangent { surf1, surf2, .. } => {
                let r1 = implicit_residual(surf1, p);
                let r2 = implicit_residual(surf2, p);
                tangent_resid_max = tangent_resid_max.max(r1.abs()).max(r2.abs());
                check_residual(
                    "tangent_on_surface_1",
                    CertCheck::Surface1Residual,
                    i,
                    Margin::of(r1),
                    band,
                    &mut max_residual,
                )?;
                check_residual(
                    "tangent_on_surface_2",
                    CertCheck::Surface2Residual,
                    i,
                    Margin::of(r2),
                    band,
                    &mut max_residual,
                )?;
                if i > 0 && i < CERT_SAMPLES - 1 {
                    let tau = spec.carrier.deriv(sample_param(t0, t1, i));
                    let jet = crate::tangent::tangent_jet(surf1, surf2, p, tau);
                    let arm = crate::implicit::curvature_lever_arm(surf1, p)
                        .min(crate::implicit::curvature_lever_arm(surf2, p))
                        .min(extent);
                    let so_margin = Margin::sagitta(jet.kappa_rel.abs(), arm);
                    match decide("tangent_second_order", so_margin, band) {
                        Ok(Sign::Positive) => {}
                        // A magnitude margin: Zero is the G2/osculating
                        // zero-side (typed, definite); Negative is
                        // unreachable for a true magnitude and refuses
                        // the same conservative way.
                        Ok(Sign::Zero | Sign::Negative) => {
                            return Err(CertifyError::NotSecondOrderSeparated { sample: i, band });
                        }
                        Err(cause) => {
                            return Err(CertifyError::Escalated {
                                check: CertCheck::TangentSecondOrder,
                                sample: i,
                                cause,
                            });
                        }
                    }
                    tangent_kappa_min = tangent_kappa_min.min(jet.kappa_rel.abs());
                    tangent_arm_min = tangent_arm_min.min(arm);
                    check_residual(
                        "tangent_normal_parallel",
                        CertCheck::TangentParallel,
                        i,
                        Margin::levered_inv(jet.sin_theta, jet.kappa_rel.abs()),
                        band,
                        &mut max_residual,
                    )?;
                }
            }
            // The fenced scaffolding arm (D3): the ONE conventional
            // residual with no chart to state itself against, because
            // a transient scaffolding edge has no surfaces yet. It
            // keeps its own meter for exactly as long as the fence
            // keeps it legal.
            Resolved::Scaffold(mc) => {
                let s = T::from_f64(f64::from(i) / f64::from(CERT_SAMPLES - 1));
                check_residual(
                    "carrier_matches_mapped_source",
                    CertCheck::MappedSource,
                    i,
                    Margin::of(p.distance(mc.eval(s))),
                    band,
                    &mut max_residual,
                )?;
            }
            // **U2's one conventional arm, D1's one meter.**
            // `|C(t) − S(P(t))| ≤ ε` (C4 verbatim), on the carrier's
            // own parameter — the same statement, the same predicate
            // name and the same dimension as every stored pcurve
            // cache's schedule (`pcurve_map_residual`), because it is
            // the same statement.
            //
            // **This is a different QUANTITY from the seam arm's
            // pre-collapse meter, deliberately, and the change is a
            // re-baseline rather than a bit move.** The old seam arm
            // metered `implicit_residual`, the PERPENDICULAR distance
            // from the carrier to the surface. This meters the
            // distance to the surface point the pcurve NAMES. On a
            // cone the chart image carries the carrier's own azimuth
            // and its own axial height (`v = h / cos α`), so the
            // displacement between them is purely RADIAL and the two
            // quantities stand in the exact ratio
            //
            //     |C − S(P)| = |perpendicular| · sec α
            //
            // — 1.1547 at a 30° half-angle. A sphere and a
            // small-radius cylinder move for the same reason in their
            // own geometry.
            //
            // Which is right? This one, on three grounds, and the
            // third is the one that makes the re-baseline safe:
            //
            // 1. It is the statement D1 ratified and C4 words, and it
            //    is the ONLY statement a collapsed description can
            //    make — `implicit_residual` needs a per-chart implicit
            //    form, which is the per-class branching the collapse
            //    exists to remove.
            // 2. It is the statement every STORED pcurve cache already
            //    certifies (`pcurve_map_residual` in
            //    `pcurve_cache::schedule_residuals`). Before the
            //    collapse an edge could certify as a description and
            //    refuse as a cache on identical geometry, because the
            //    two were measuring different things. They now agree
            //    by construction.
            // 3. **It is conservative.** `S(P(t))` is a point ON the
            //    surface, so `|C − S(P)| ≥ dist(C, surface)` always,
            //    with equality exactly when the pcurve names the foot
            //    point. The collapse can therefore only REFUSE what
            //    the old meter accepted; it can never ACCEPT what the
            //    old meter refused. The direction of the change is
            //    toward truth, and the bound is `sec α` on a cone.
            //
            // The consequence is stated rather than hidden: on a cone
            // of half-angle α, an edge whose perpendicular drift lies
            // in `(ε·cos α, ε]` certified before this unit and now
            // refuses or escalates. `d2_bit_diff` and the cone
            // re-baseline row measure it; the live minting class
            // (`sweep/src/revolve/upgrade.rs:219`) mints exact seams
            // and is unaffected, which the whole-body batteries show.
            Resolved::Chart { surface, seam, .. } => {
                let Some(chart) = canonical.chart() else {
                    return Err(CertifyError::Unimplemented);
                };
                let q = chart.pcurve.eval(sample_param(t0, t1, i));
                check_residual(
                    "pcurve_map_residual",
                    CertCheck::ChartResidual,
                    i,
                    Margin::of(p.distance(surface.eval(q.x, q.y))),
                    band,
                    &mut max_residual,
                )?;
                // D1's retained obligation. The unified meter cannot
                // see the difference between a seam meridian and its
                // antipode — both ARE chart images of the same
                // surface, and the meter is satisfied by either — so a
                // chart image that claims to be THE seam owes the two
                // half-plane/side predicates as well. They are an
                // obligation on periodic charts, never a second form.
                //
                // seam_frame is Some: plane and Nurbs were rejected in
                // check 1, and every remaining kind is axisymmetric.
                if *seam && let Some((w, u_ref, v_ref)) = seam_frame(surface, p) {
                    check_residual(
                        "carrier_in_seam_halfplane",
                        CertCheck::SeamHalfplane,
                        i,
                        Margin::of(w.dot(v_ref)),
                        band,
                        &mut max_residual,
                    )?;
                    check_residual(
                        "carrier_on_seam_side",
                        CertCheck::SeamSide,
                        i,
                        Margin::of((T::zero() - w.dot(u_ref)).max(T::zero())),
                        band,
                        &mut max_residual,
                    )?;
                }
            }
            // The plane × NURBS lane owns its OWN schedule (denser
            // than this one, and shared with the certificate's foot
            // points), so it runs once after the loop rather than
            // per-sample here — see the block below check 4.
            Resolved::PlaneNurbs { .. } => {}
        }
    }

    // ---- TangentIntersection only: the span-wide jet statements
    // (M5 PR 9). C2.2: the worst sampled residual plus the certified
    // quadratic sag stays within ε — a SUP bound, not a sampled max.
    // C2.3-style tube on the jet system: the sampled |κ_rel| minus
    // its certified drift stays definitely positive over every span,
    // so second-order separation (local uniqueness of the tangency
    // locus — the jet system's IFT margin) holds along the whole
    // edge. Outside the certified span-bound lane: typed refusal,
    // never a fallback. ----
    if let Resolved::Tangent { surf1, surf2, .. } = &resolved {
        let Some(bounds) = crate::tangent::tangent_span_bounds(surf1, surf2, &spec.carrier, t0, t1)
        else {
            return Err(CertifyError::TangentCertificateUnsupported);
        };
        check_residual(
            "tangent_hull_sup",
            CertCheck::TangentHull,
            0,
            Margin::of(tangent_resid_max + bounds.residual_sag),
            band,
            &mut max_residual,
        )?;
        let tube = Margin::sagitta(tangent_kappa_min - bounds.kappa_drift, tangent_arm_min);
        match decide("tangent_tube_margin", tube, band) {
            Ok(Sign::Positive) => {}
            Ok(Sign::Zero | Sign::Negative) => {
                return Err(CertifyError::NotSecondOrderSeparated { sample: 0, band });
            }
            Err(cause) => {
                return Err(CertifyError::Escalated {
                    check: CertCheck::TangentTube,
                    sample: 0,
                    cause,
                });
            }
        }
    }

    // ---- Intersection, plane × NURBS only (M7-8): the whole
    // declare-and-check certificate — the closed-form plane residual,
    // the certified foot residual on the wall, the between-samples
    // sup bound, the per-sample transversality and the uniqueness
    // tube. The lane refuses typed WITH its measured bound; a
    // transversality failure lands in this module's existing
    // vocabulary, exactly as the analytic arm's does. ----
    if let Resolved::PlaneNurbs { plane, wall, .. } = &resolved {
        let Curve3::Nurbs(ref carrier) = spec.carrier else {
            return Err(CertifyError::PlaneNurbs(
                crate::edge_nurbs::PlaneNurbsRefusal::Unsupported {
                    what: "the declared carrier of a plane × NURBS Intersection must be a \
                           spline: the certificate's limbs are hull statements about a \
                           control net",
                },
            ));
        };
        // `resolved` is only ever `PlaneNurbs` when the door injected
        // a lane (the resolution arm above), so this is not a fallback.
        let Some(lane) = lane else {
            return Err(CertifyError::Unimplemented);
        };
        let limbs = lane(carrier, plane, wall, extent, band).map_err(|e| match e {
            crate::edge_nurbs::PlaneNurbsRefusal::NotTransverse { sample } => {
                CertifyError::NotTransverse { sample }
            }
            crate::edge_nurbs::PlaneNurbsRefusal::Escalated(cause) => CertifyError::Escalated {
                check: CertCheck::PlaneNurbsCertificate,
                sample: 0,
                cause,
            },
            other => CertifyError::PlaneNurbs(other),
        })?;
        check_residual(
            "plane_nurbs_on_locus",
            CertCheck::PlaneNurbsOnLocus,
            0,
            Margin::of(limbs.on_locus_max),
            band,
            &mut max_residual,
        )?;
        check_residual(
            "plane_nurbs_hull_sup",
            CertCheck::PlaneNurbsHull,
            0,
            Margin::of(limbs.hull_sup),
            band,
            &mut max_residual,
        )?;
    }

    // ---- Check 5: witness residuals + mid-parameter pin
    // (Intersection and TangentIntersection; see the check-sequence
    // docs for the witness contract: the witness IS the edge's
    // mid-parameter point). ----
    if let Resolved::Intersection {
        surf1,
        surf2,
        witness,
    }
    | Resolved::Tangent {
        surf1,
        surf2,
        witness,
    } = &resolved
    {
        check_residual(
            "witness_on_surface_1",
            CertCheck::WitnessSurface1,
            0,
            Margin::of(implicit_residual(surf1, *witness)),
            band,
            &mut max_residual,
        )?;
        check_residual(
            "witness_on_surface_2",
            CertCheck::WitnessSurface2,
            0,
            Margin::of(implicit_residual(surf2, *witness)),
            band,
            &mut max_residual,
        )?;
        let mid = spec
            .carrier
            .eval(sample_param(t0, t1, (CERT_SAMPLES - 1) / 2));
        check_residual(
            "witness_at_mid_parameter",
            CertCheck::WitnessMidpoint,
            (CERT_SAMPLES - 1) / 2,
            Margin::of(mid.distance(*witness)),
            band,
            &mut max_residual,
        )?;
    }

    // The plane × NURBS witness (M7-8): the plane side is the same
    // closed-form residual the analytic arm checks; the wall side is
    // discharged by the lane's own schedule, which contains the
    // mid-parameter exactly (`PXN_FIT_SAMPLES` is odd). The
    // mid-parameter pin is unchanged — the witness contract does not
    // move at this rung.
    if let Resolved::PlaneNurbs { plane, witness, .. } = &resolved {
        check_residual(
            "witness_on_surface_1",
            CertCheck::WitnessSurface1,
            0,
            Margin::of(implicit_residual(plane, *witness)),
            band,
            &mut max_residual,
        )?;
        let mid = spec
            .carrier
            .eval(sample_param(t0, t1, (CERT_SAMPLES - 1) / 2));
        check_residual(
            "witness_at_mid_parameter",
            CertCheck::WitnessMidpoint,
            (CERT_SAMPLES - 1) / 2,
            Margin::of(mid.distance(*witness)),
            band,
            &mut max_residual,
        )?;
    }

    Ok((
        Certificate {
            samples: CERT_SAMPLES,
            max_residual,
        },
        canonical,
    ))
}

/// The plane × NURBS pairing, in either order: exactly one PLANE and
/// exactly one **described** NURBS wall (the mvfs placeholder is a
/// mid-surgery "no description yet" fact, never an operand).
///
/// `None` for every other pair, which then takes the analytic path and
/// its existing refusals verbatim.
fn plane_nurbs_pair<T: Real>(
    s1: Option<Surface<T>>,
    s2: Option<Surface<T>>,
) -> Option<(Surface<T>, std::sync::Arc<geom::NurbsSurface<T>>)> {
    let (a, b) = (s1?, s2?);
    let described = |n: &std::sync::Arc<geom::NurbsSurface<T>>| !n.is_placeholder();
    match (&a, &b) {
        (Surface::Plane { .. }, Surface::Nurbs(n)) if described(n) => Some((a.clone(), n.clone())),
        (Surface::Nurbs(n), Surface::Plane { .. }) if described(n) => Some((b.clone(), n.clone())),
        _ => None,
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use geom_core::Tol;
    use geom_core::spline::KnotVector;
    use geom_core::{Affine3, Point2, Vec3};

    use crate::mapped::{MappedCurve, SketchSegment};

    use super::*;

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    fn eps() -> f64 {
        Tol::witness().get().eps
    }

    /// A resolver over a tiny fixed table (keys minted through a local
    /// slotmap, mirroring how a Body resolves).
    fn table(
        surfs: Vec<Surface<f64>>,
    ) -> (Vec<SurfaceKey>, impl Fn(SurfaceKey) -> Option<Surface<f64>>) {
        let mut map: slotmap::SlotMap<SurfaceKey, Surface<f64>> = slotmap::SlotMap::with_key();
        let keys: Vec<SurfaceKey> = surfs.into_iter().map(|s| map.insert(s)).collect();
        (keys, move |k| map.get(k).cloned())
    }

    fn line_spec(p0: Point3<f64>, p1: Point3<f64>) -> EdgeCurveSpec<f64> {
        EdgeCurveSpec::line_between(p0, p1)
    }

    // ------------------------------------------------------------------
    // **D2's bit-diff row**: the collapse's effect on
    // `Certificate.max_residual`, MEASURED per fixture rather than
    // asserted.
    //
    // The three pre-collapse conventional forms did not measure the
    // same thing — a seam stated its on-surface residual through the
    // IMPLICIT form, an iso curve through `|C − S(u, v)|` with `v`
    // affine in the SCHEDULE FRACTION, a mapped source against a
    // pushforward with no surface at all — so a single meter cannot
    // reproduce all three bit for bit, and pinning it per lane would
    // re-import the per-class branching the collapse exists to
    // remove. What is owed instead is honesty: each fixture's delta
    // is measured here, in ULPs, and moves only when the arithmetic
    // moves.
    //
    // Each row states the legacy expression VERBATIM — not a
    // re-association chosen to make a number match — and compares it
    // against what certification now records.
    // ------------------------------------------------------------------

    /// The bit-diff row's OWN band, built from the fixture's drift
    /// rather than from the run's ε.
    ///
    /// The band's zero threshold is four times the drift at every
    /// scale, so each row sits at the same fraction of its own band
    /// and the SWEPT variable is the drift alone. Deriving the band
    /// from the run's ε instead would make the row a different
    /// measurement on every ε point in the matrix, which is how the
    /// first version of this row passed locally and failed hosted.
    fn d2_band(drift: f64) -> Band {
        Band::new(4.0 * drift, 40.0 * drift).expect("the bit-diff row's own band")
    }

    /// ULP distance between two finite same-sign `f64`s.
    fn ulps(a: f64, b: f64) -> i64 {
        let (x, y) = (a.to_bits() as i64, b.to_bits() as i64);
        (x - y).abs()
    }

    /// The pre-collapse SEAM meter, verbatim: endpoint pins, then per
    /// sample the implicit-form residual and the two seam predicates.
    fn legacy_seam_max(
        spec: &EdgeCurveSpec<f64>,
        surface: &Surface<f64>,
        start: Point3<f64>,
        end: Point3<f64>,
    ) -> f64 {
        let (t0, t1) = (spec.param_start, spec.param_end);
        let mut m = spec.carrier.eval(t0).distance(start);
        m = m.max(spec.carrier.eval(t1).distance(end));
        for i in 0..CERT_SAMPLES {
            let p = spec.carrier.eval(sample_param(t0, t1, i));
            m = m.max(implicit_residual(surface, p).abs());
            if let Some((w, u_ref, v_ref)) = seam_frame(surface, p) {
                m = m.max(w.dot(v_ref).abs());
                m = m.max((0.0 - w.dot(u_ref)).max(0.0).abs());
            }
        }
        m
    }

    /// The pre-collapse ISO meter, verbatim — `v0 + (v1 − v0)·frac`
    /// on the schedule FRACTION, which is the arithmetic order the
    /// collapse necessarily changes (a `Pcurve` is a function of the
    /// carrier's own parameter).
    fn legacy_iso_max(
        spec: &EdgeCurveSpec<f64>,
        surface: &Surface<f64>,
        u: f64,
        v0: f64,
        v1: f64,
        start: Point3<f64>,
        end: Point3<f64>,
    ) -> f64 {
        let (t0, t1) = (spec.param_start, spec.param_end);
        let mut m = spec.carrier.eval(t0).distance(start);
        m = m.max(spec.carrier.eval(t1).distance(end));
        for i in 0..CERT_SAMPLES {
            let p = spec.carrier.eval(sample_param(t0, t1, i));
            let frac = f64::from(i) / f64::from(CERT_SAMPLES - 1);
            let v = v0 + (v1 - v0) * frac;
            m = m.max(p.distance(surface.eval(u, v)));
        }
        m
    }

    /// **Acceptance 1**: `geom-brep` carries ONE conventional
    /// description form. The two pre-collapse conventional writings —
    /// a periodic seam and a chart iso curve — certify onto the same
    /// arm, and the fenced scaffolding door is the only other
    /// conventional shape that exists.
    #[test]
    fn the_collapse_leaves_one_conventional_form() {
        let r = 2.0;
        let (keys, lookup) = table(vec![Surface::Cylinder {
            origin: Point3::origin(),
            axis: Vec3::unit_z(),
            radius: r,
            u_ref: Vec3::unit_x(),
        }]);
        let (p0, p1) = (Point3::new(r, 0.0, 0.0), Point3::new(r, 0.0, 3.0));
        let seam = EdgeCurve::certify(
            EdgeCurveSpec {
                description: EdgeGeometry::Seam { surface: keys[0] },
                carrier: Curve3::Line {
                    origin: p0,
                    dir: Vec3::unit_z(),
                },
                param_start: 0.0,
                param_end: 3.0,
            },
            p0,
            p1,
            &lookup,
            band(),
        )
        .expect("the seam certifies");
        let chart = seam.canonical().chart().expect("a seam IS a chart image");
        assert_eq!(chart.surface, keys[0]);
        assert!(chart.seam, "a seam carries D1's obligation");

        let plane = Surface::Plane {
            origin: Point3::new(0.25, -0.5, 1.0),
            normal: Vec3::unit_z(),
            u_ref: Vec3::unit_x(),
        };
        let (pk, plookup) = table(vec![plane]);
        let (u, v0, v1) = (0.3_f64, 0.7_f64, 2.9_f64);
        let s_at = |v: f64| plookup(pk[0]).unwrap().eval(u, v);
        let (q0, q1) = (s_at(v0), s_at(v1));
        let len = q0.distance(q1);
        let iso = EdgeCurve::certify(
            EdgeCurveSpec {
                description: EdgeGeometry::IsoCurve {
                    surface: pk[0],
                    u,
                    v0,
                    v1,
                },
                carrier: Curve3::Line {
                    origin: q0,
                    dir: (q1 - q0) / len,
                },
                param_start: 0.0,
                param_end: len,
            },
            q0,
            q1,
            &plookup,
            band(),
        )
        .expect("the iso curve certifies");
        let iso_chart = iso
            .canonical()
            .chart()
            .expect("an iso curve IS a chart image");
        assert!(
            !iso_chart.seam,
            "an iso boundary owes the meter and nothing else"
        );

        // The fenced scaffolding door is the only other conventional
        // shape, and it is NOT a chart image — it has no surface.
        let (a, b) = (Point3::new(-1.0, 0.25, 0.5), Point3::new(2.0, -3.0, 4.0));
        let (_, empty) = table(vec![]);
        let scaffold = EdgeCurve::certify(line_spec(a, b), a, b, &empty, band())
            .expect("the scaffolding line certifies");
        assert!(scaffold.canonical().chart().is_none());
        assert!(matches!(scaffold.canonical(), EdgeDescription::Scaffold(_)));
    }

    /// **The authority record** (U2 Q3): the datum tier 3's
    /// prefer-intrinsic rules read instead of `MappedCurve`'s negative
    /// space. A declared locus answers `true` and carries its source;
    /// a derived one answers `false` — which is exactly the verdict
    /// `TransverseNotIntrinsic` needs and the only one it needs.
    #[test]
    fn the_authority_record_replaces_the_negative_space() {
        let (a, b) = (Point3::new(-1.0, 0.25, 0.5), Point3::new(2.0, -3.0, 4.0));
        let (_, empty) = table(vec![]);
        let declared = EdgeCurve::certify(line_spec(a, b), a, b, &empty, band())
            .expect("the scaffolding line certifies");
        assert!(declared.authority().is_declared());
        assert!(matches!(
            declared.authority(),
            EdgeAuthority::Declared(MappedCurve::ExtrudedPoint { .. })
        ));

        let r = 2.0;
        let (keys, lookup) = table(vec![Surface::Cylinder {
            origin: Point3::origin(),
            axis: Vec3::unit_z(),
            radius: r,
            u_ref: Vec3::unit_x(),
        }]);
        let (p0, p1) = (Point3::new(r, 0.0, 0.0), Point3::new(r, 0.0, 3.0));
        let derived = EdgeCurve::certify(
            EdgeCurveSpec {
                description: EdgeGeometry::Seam { surface: keys[0] },
                carrier: Curve3::Line {
                    origin: p0,
                    dir: Vec3::unit_z(),
                },
                param_start: 0.0,
                param_end: 3.0,
            },
            p0,
            p1,
            &lookup,
            band(),
        )
        .expect("the seam certifies");
        assert!(!derived.authority().is_declared());
    }

    /// One row of the DRIFT sweep: the seam class's ULP delta and the
    /// scaffolding control's, at a given in-band drift, in the order
    /// (cylinder-seam, mapped-line).
    ///
    /// The iso class is swept separately, over the variable it
    /// actually moves in — see [`d2_iso_delta`].
    fn d2_row(drift: f64) -> (i64, i64) {
        // ---- Fixture "cylinder-seam": the seam ruling of a radius-2
        // cylinder about +z, seam at +x. Legacy: implicit residual +
        // the two predicates. Now: |C − S(P)| + the same two
        // predicates (D1 keeps them).
        let r = 2.0;
        let cyl = Surface::Cylinder {
            origin: Point3::origin(),
            axis: Vec3::unit_z(),
            radius: r,
            u_ref: Vec3::unit_x(),
        };
        let (keys, lookup) = table(vec![cyl]);
        // Perturbed INSIDE the band: an exact ruling makes every meter
        // answer a bitwise zero, which measures nothing. `d` is the
        // radial drift a real construction leaves behind, and it is
        // what the two meters disagree about.
        let d = drift;
        let (p0, p1) = (Point3::new(r + d, 0.0, 0.0), Point3::new(r + d, 0.0, 3.0));
        let seam_spec = EdgeCurveSpec {
            description: EdgeGeometry::Seam { surface: keys[0] },
            carrier: Curve3::Line {
                origin: p0,
                dir: Vec3::unit_z(),
            },
            param_start: 0.0,
            param_end: 3.0,
        };
        let cert = EdgeCurve::certify(seam_spec.clone(), p0, p1, &lookup, d2_band(drift))
            .expect("the seam certifies")
            .certificate;
        let legacy = legacy_seam_max(&seam_spec, &lookup(keys[0]).unwrap(), p0, p1);
        let seam_delta = ulps(cert.max_residual, legacy);

        // ---- Fixture "mapped-line": the fenced scaffolding arm. Its
        // meter is untouched by the collapse, so its delta is the
        // control: it must be exactly zero.
        let (a, b) = (Point3::new(-1.0, 0.25, 0.5), Point3::new(2.0, -3.0, 4.0));
        let mapped = line_spec(a, b);
        let (_, empty) = table(vec![]);
        let mapped_cert = EdgeCurve::certify(mapped.clone(), a, b, &empty, d2_band(drift))
            .expect("the mapped line certifies")
            .certificate;
        let mut mapped_legacy = mapped.carrier.eval(mapped.param_start).distance(a);
        mapped_legacy = mapped_legacy.max(mapped.carrier.eval(mapped.param_end).distance(b));
        let EdgeGeometry::MappedCurve(mc) = mapped.description else {
            panic!("line_between describes a MappedCurve");
        };
        for i in 0..CERT_SAMPLES {
            let p = mapped
                .carrier
                .eval(sample_param(mapped.param_start, mapped.param_end, i));
            let s = f64::from(i) / f64::from(CERT_SAMPLES - 1);
            mapped_legacy = mapped_legacy.max(p.distance(mc.eval(s)));
        }
        let mapped_delta = ulps(mapped_cert.max_residual, mapped_legacy);

        (seam_delta, mapped_delta)
    }

    /// The ISO class's ULP delta at a given parameter anchor, with
    /// **no drift at all**.
    ///
    /// Zero drift is the point. With a drift the residual is
    /// `√(drift² + dz²)` and the mint's own arithmetic enters only
    /// quadratically, so a re-association of the mint is swamped and
    /// the row goes blind — which is how the first version of this
    /// fixture read 0 ULP while a laundering mutant survived. At zero
    /// drift the residual IS `|dz|`, linear in the mint's error, and
    /// any re-association shows at full strength.
    fn d2_iso_delta(anchor: f64) -> (i64, ChartCurve<f64>) {
        // Fixture "nurbs-iso": an iso curve of a described NURBS
        // wall over an interval that does NOT start at zero.
        //
        // **The chart is a spline one on purpose.** On every ANALYTIC
        // chart the v-channel is unit-speed, so a boundary iso's slope
        // `(v1 − v0)/(t1 − t0)` is exactly ±1 — and at slope 1 the
        // collapsed mint `(v0 − slope·t0) + slope·t` re-associates to
        // the legacy `v0 + (v1 − v0)·frac` bit for bit, which makes
        // the row BLIND to exactly the laundering D2 forbids. A
        // spline chart's v runs over its knot domain while the carrier
        // runs over arc length, so the slope is a real number; here it
        // is 1/3, asserted below in bits so the blindness cannot come
        // back unnoticed.
        let wall = {
            let ku = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).expect("u knots");
            let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).expect("v knots");
            let control = vec![
                Point3::new(1.0, 0.0, 0.0),
                Point3::new(1.0, 0.0, 3.0),
                Point3::new(2.0, 0.0, 0.0),
                Point3::new(2.0, 0.0, 3.0),
            ];
            Surface::Nurbs(std::sync::Arc::new(
                geom::NurbsSurface::new(ku, kv, control, vec![1.0; 4]).expect("wall builds"),
            ))
        };
        let (pk, plookup) = table(vec![wall]);
        // `v0` is deliberately NOT zero and the endpoints are
        // deliberately not round: at `v0 = 0` the mint's
        // `(v0 − slope·t0) + slope·t` collapses to `slope·t` and the
        // two expressions agree bitwise again — a second way for this
        // row to go blind, found by measuring rather than by reasoning
        // about it.
        let (u, v0, v1) = (0.0_f64, 0.13_f64, 0.83_f64);
        let s_at = |v: f64| plookup(pk[0]).unwrap().eval(u, v);
        let (q0, q1) = (s_at(v0), s_at(v1));
        let len = q0.distance(q1);
        let t1 = anchor + len;
        let dir = (q1 - q0) / len;
        assert!(
            (v1 - v0) / (t1 - anchor) != 1.0,
            "the iso fixture's slope must not be 1.0 — at slope 1 the two mint \
             expressions coincide bitwise and the row measures nothing"
        );
        // The in-band drift, off the chart along its own normal.
        let off = Vec3::zero();
        let iso_spec = EdgeCurveSpec {
            description: EdgeGeometry::IsoCurve {
                surface: pk[0],
                u,
                v0,
                v1,
            },
            carrier: Curve3::Line {
                origin: q0 + off - dir * anchor,
                dir,
            },
            param_start: anchor,
            param_end: t1,
        };
        let iso_cert = EdgeCurve::certify(
            iso_spec.clone(),
            q0 + off,
            q1 + off,
            &plookup,
            d2_band(2.5e-10),
        )
        .expect("the nurbs iso certifies");
        let iso_chart = iso_cert
            .canonical()
            .chart()
            .expect("an iso description IS a chart image")
            .clone();
        let iso_cert = iso_cert.certificate;
        let iso_legacy = legacy_iso_max(
            &iso_spec,
            &plookup(pk[0]).unwrap(),
            u,
            v0,
            v1,
            q0 + off,
            q1 + off,
        );
        let iso_delta = ulps(iso_cert.max_residual, iso_legacy);

        (iso_delta, iso_chart)
    }

    /// The iso fixture's minted chart image at a given anchor — the
    /// mint's own output, for the bit-pin row.
    fn d2_iso_chart(anchor: f64) -> (ChartCurve<f64>, f64) {
        let (delta, chart) = d2_iso_delta(anchor);
        #[allow(clippy::cast_precision_loss)]
        (chart, delta as f64)
    }

    /// **D2's bit-diff row, swept over drift scale.**
    ///
    /// A delta measured at one operating point is true where it was
    /// taken and says nothing about the meter; the sweep is what turns
    /// it into a measurement. Three decades, each pinned:
    ///
    /// | drift (m) | cylinder-seam | mapped-line (control) |
    /// |---|---|---|
    /// | 2.5e-7  | 293 601 280 | 0 |
    /// | 2.5e-10 | 0 | 0 |
    /// | 2.5e-13 | 0 | 0 |
    ///
    /// **The seam column is not scale-invariant**, so it may not be
    /// read as a statement about its meter: "the seam arm does not
    /// move" is true only below the coarse decade. Any claim about the
    /// size of the move is a claim about a scale, and this table is
    /// the only support for one.
    ///
    /// The mechanism is **not derived here** — the arm is
    /// bitwise-identical at both fine decades and moves only at the
    /// coarse one, which is the signature of a cancellation floor in
    /// the legacy `|radial| − r` rather than of the collapse.
    ///
    /// **What this table does NOT measure**, named so it is not
    /// over-read. Not the iso arm: that arm moves in the parameter
    /// ANCHOR, not in the drift, and is swept by
    /// [`d2_iso_move_is_unbounded_in_the_anchor_offset`] at zero
    /// drift. Not the seam meter across CHART KINDS: a cone's
    /// implicit residual reads the perpendicular distance to the
    /// generator where `|C − S(P)|` reads the radial chord — a change
    /// of QUANTITY, `sec α`, measured in `tests/r2_probes.rs` and
    /// disposed of at the Chart arm.
    ///
    /// The mapped-line column is the control: the fenced scaffolding
    /// arm's meter is untouched by the collapse, so it must read zero
    /// at every scale, and a nonzero entry there means the sweep
    /// itself is measuring the wrong thing.
    const D2_SWEEP: [(f64, (i64, i64)); 3] = [
        (2.5e-7, (D2_S0_SEAM, 0)),
        (2.5e-10, (D2_S1_SEAM, 0)),
        (2.5e-13, (D2_S2_SEAM, 0)),
    ];

    /// Drift 2.5e-7 m, cylinder-seam: **293 601 280 ULP**. The seam
    /// arm's two expressions do NOT agree at this scale — the legacy
    /// implicit form computes `|radial| − r`, a subtractive
    /// cancellation against the chart radius, where `|C − S(P)|`
    /// forms the same length directly. The move is toward the
    /// directly-formed one.
    const D2_S0_SEAM: i64 = 293_601_280;
    /// Drift 2.5e-10 m, cylinder-seam: **0 ULP**.
    const D2_S1_SEAM: i64 = 0;
    /// Drift 2.5e-13 m, cylinder-seam: **0 ULP**.
    const D2_S2_SEAM: i64 = 0;

    /// **R1's M4, executed**: the iso arm's move is unbounded in the
    /// ANCHOR OFFSET, so no single number is "the size of the move".
    ///
    /// The mint writes the chart image on the carrier's own parameter
    /// as `(v0 − slope·t0) + slope·t`. Both terms grow with `t0` while
    /// their difference does not, so the cancellation — and with it
    /// the departure from the legacy `v0 + (v1 − v0)·frac` — grows
    /// without bound as the edge's parameter anchor moves away from
    /// the origin. At `t0 = 0` the two expressions coincide exactly.
    ///
    /// **The direction is toward truth, and here is the bound.** The
    /// legacy expression evaluates `v` at the schedule FRACTION, which
    /// is not the quantity the description states: the description
    /// says `v` is affine in the CARRIER PARAMETER, and the carrier is
    /// what the residual is taken against. The collapsed expression
    /// evaluates the stated quantity. Both lose accuracy as `t0`
    /// grows, and they lose it in the same place — the carrier's own
    /// `origin + dir·t` suffers the identical cancellation — so the
    /// row bounds the DISAGREEMENT, not the error: it is at most the
    /// cancellation `t0·2⁻⁵³` propagated through the chart's v-scale,
    /// which is what the table below shows growing linearly in `t0`.
    /// An edge anchored 1e6 m from its own parameter origin has a
    /// representation problem the meter cannot fix and should not
    /// hide.
    /// **The mint's own bits**, pinned — the tripwire D2 actually
    /// asks for.
    ///
    /// The residual sweeps below measure the mint's EFFECT, and an
    /// effect can be swamped: a one-ULP change in the chart image
    /// survives a distance-to-surface only if nothing downstream
    /// rounds it away, which is how two earlier versions of this
    /// fixture read 0 ULP while the arithmetic underneath had moved.
    /// This row compares the mint's OUTPUT instead, so any
    /// re-expression of `(v0 − slope·t0) + slope·t` — however
    /// algebraically equal — changes a pinned bit pattern and fails
    /// here, whether or not it survives to the certificate.
    ///
    /// Demonstrated rather than hoped: a mutant that makes the
    /// collapsed meter reproduce the legacy `v0 + (v1 − v0)·frac`
    /// fails the anchor sweep below, and all three known
    /// re-associations of `v0 − slope·t0` fail here.
    ///
    /// **Two of those three were once recorded as "no-ops", wrongly.**
    /// They were checked against the four round anchors this row
    /// started with, agreed at all four, and were written up as
    /// bitwise-identical. They are not: one separates at ~50 % of
    /// anchors and the other at ~1.2 %. The separating anchors are
    /// pinned below for that reason, and the episode is recorded
    /// rather than quietly repaired — a false "we checked, it was a
    /// no-op" in the record is worse than an unexamined mutant,
    /// because it tells the next reader not to look.
    #[test]
    fn d2_the_mint_arithmetic_is_pinned_in_bits() {
        // **One anchor per mutant FAMILY, chosen by separation rather
        // than by roundness.** A re-association can be
        // bitwise-identical at some anchors and different at others,
        // so the anchors are not decoration — each is here because a
        // known mutant survives without it:
        //
        // - `1.7` separates `v1 − slope·t1` (the far-endpoint anchor),
        //   which is identical at every other entry;
        // - `84871.995…` separates `(v0·L − (v1−v0)·t0)/L` (the
        //   algebraic re-expression), which agrees at 0, 1.7, 1e3 and
        //   1e6 and separates at roughly HALF of random anchors;
        // - `4824.781…` separates the reciprocal-slope form
        //   `v0 − (v1−v0)·(1/L)·t0`, which separates at only ~1.2 % of
        //   anchors and so is the easiest of the three to miss.
        //
        // The last two were found by a reviewer executing mutants this
        // row's author had asserted were no-ops. They were not: they
        // coincided at the four anchors originally pinned. A tripwire
        // is only as wide as the cases it was checked against, and
        // "we tried it and nothing moved" is exactly what a too-narrow
        // one reports.
        const PINNED: [(f64, u64, u64); 6] = [
            (0.0, 0x3fc0_a3d7_0a3d_70a4, 0x3fd5_5555_5555_5556),
            (1.7, 0xbfdb_f258_bf25_8bf4, 0x3fd5_5555_5555_5556),
            (1.0e3, 0xc074_d340_da74_0d68, 0x3fd5_5555_5555_5514),
            (1.0e6, 0xc114_5854_d037_9507, 0x3fd5_5555_5556_5965),
            (
                84_871.995_158_921_64,
                0xc0db_a0a2_3e4e_7fe9,
                0x3fd5_5555_5555_1451,
            ),
            (
                4_824.781_053_628_38,
                0xc099_2085_7ac9_91b5,
                0x3fd5_5555_5555_5145,
            ),
        ];
        let measured: Vec<(f64, u64, u64)> = PINNED
            .iter()
            .map(|&(anchor, _, _)| {
                let (chart, _) = d2_iso_chart(anchor);
                let Pcurve::IsoLine { p0, pl } = chart.pcurve else {
                    panic!("the iso arm mints an IsoLine chart image");
                };
                (anchor, p0.y.to_bits(), pl.y.to_bits())
            })
            .collect();
        assert_eq!(
            measured,
            PINNED.to_vec(),
            "the iso mint's arithmetic moved. The mint is \
             `(v0 − slope·t0) + slope·t`, written that way ON PURPOSE — \
             re-expressing it is what D2 forbids, and restating these \
             constants to match a re-expression IS the laundering"
        );
    }

    #[test]
    fn d2_iso_move_is_unbounded_in_the_anchor_offset() {
        let measured: Vec<(f64, i64)> = D2_ANCHOR_SWEEP
            .iter()
            .map(|&(t0, _)| (t0, d2_iso_delta(t0).0))
            .collect();
        let pinned: Vec<(f64, i64)> = D2_ANCHOR_SWEEP.to_vec();
        assert_eq!(
            measured, pinned,
            "the anchor-offset sweep moved: measured (t0, iso ULP) = {measured:?}, \
             pinned {pinned:?}. Re-measure and RESTATE — and read the METRE \
             values (run with D2_DEBUG=1), not the ULP counts, for the size"
        );
    }

    /// The anchor-offset sweep's pinned table (see the test's docs),
    /// measured at zero drift so the residual IS the mint arithmetic:
    ///
    /// | `t0` (m) | legacy (m) | collapsed (m) | ULP | relative |
    /// |---|---|---|---|---|
    /// | 0     | 4.4409e-16 | 4.4409e-16 | 0 | 0 |
    /// | 1.7   | 4.4409e-16 | 4.4409e-16 | 0 | 0 |
    /// | 1e3   | 5.9064e-14 | 1.7053e-13 | 6.58e15 | **+189 %** |
    /// | 1e6   | 6.0536e-11 | 5.8208e-11 | 1.80e14 | **−3.85 %** |
    ///
    /// Read the ULP column as a tripwire and the relative column as
    /// the size: a ULP distance between two numbers of different
    /// ORDER is large by construction and says little on its own.
    ///
    /// The last row is the one to look at twice. At `t0 = 1e6` the
    /// residual is 6e-11 m — six per cent of a 1e-9 band — in BOTH
    /// meters, from parameter cancellation alone. Neither expression
    /// is trustworthy there, they disagree by 3.85 %, and the sign of
    /// the disagreement is not even constant across the sweep. That
    /// is the bound the D1 argument needs: the collapse evaluates the
    /// quantity the description STATES, but at a far enough anchor
    /// both evaluations are dominated by the carrier's own
    /// representation error, and no meter can repair that.
    const D2_ANCHOR_SWEEP: [(f64, i64); 4] =
        [(0.0, D2_A0), (1.7, D2_A1), (1.0e3, D2_A2), (1.0e6, D2_A3)];
    /// Anchor 0: the two expressions are identical (`slope·t0 = 0`).
    const D2_A0: i64 = 0;
    /// Anchor 1.7 m: still identical at this fixture's numbers.
    const D2_A1: i64 = 0;
    /// Anchor 1e3 m: the collapsed meter reads 2.9× the legacy one.
    const D2_A2: i64 = 6_579_477_580_611_584;
    /// Anchor 1e6 m: they disagree by 3.85 %, the other way.
    const D2_A3: i64 = 180_148_108_263_424;

    #[test]
    fn d2_bit_diff_row_is_measured_across_drift_scales() {
        // The WHOLE table is compared at once, deliberately: a
        // per-scale assertion stops at the first move and hides the
        // shape of the rest, and the shape is the measurement.
        let measured: Vec<(f64, (i64, i64))> =
            D2_SWEEP.iter().map(|&(d, _)| (d, d2_row(d))).collect();
        let pinned: Vec<(f64, (i64, i64))> = D2_SWEEP.to_vec();
        assert_eq!(
            measured, pinned,
            "D2 drift sweep moved — measured (drift, (cylinder-seam, mapped-line)) \
             = {measured:?}, pinned {pinned:?}. Re-measure and RESTATE the row; \
             never re-associate a meter to make a number match (D2)"
        );
    }

    #[test]
    fn self_loop_circle_certifies_at_its_anchor() {
        let p = Point3::new(-2.0, 0.5, 7.0);
        let spec = EdgeCurveSpec::self_loop_circle_at(p);
        EdgeCurve::certify(spec.clone(), p, p, |_| None, band()).unwrap();
        // Coincident-endpoint LINE specs, by contrast, poison and are
        // refused (typed, no panic) — the reason the circle convention
        // exists.
        let bad = EdgeCurveSpec::line_between(p, p);
        assert!(EdgeCurve::certify(bad, p, p, |_| None, band()).is_err());
    }

    #[test]
    fn mapped_line_certifies_and_is_deterministic() {
        let p0 = Point3::new(0.25, -1.0, 2.0);
        let p1 = Point3::new(1.25, 0.5, 3.5);
        let spec = line_spec(p0, p1);
        let a = EdgeCurve::certify(spec.clone(), p0, p1, |_| None, band()).unwrap();
        let b = EdgeCurve::certify(spec.clone(), p0, p1, |_| None, band()).unwrap();
        // D9: byte-identical certification records across runs.
        assert_eq!(
            format!("{:?}", a.certificate()),
            format!("{:?}", b.certificate())
        );
        assert_eq!(a.certificate().samples, CERT_SAMPLES);
        assert!(a.certificate().max_residual <= eps());
        // Recertification at rest agrees.
        a.recertify(p0, p1, |_| None, band()).unwrap();
    }

    #[test]
    fn wrong_cache_is_rejected() {
        // A carrier offset well beyond the escalation band (100·ε ≥ K·ε
        // at every CI ε row) must be rejected: the cache does not
        // represent the description.
        let p0 = Point3::new(0.0, 0.0, 0.0);
        let p1 = Point3::new(1.0, 0.0, 0.0);
        let mut spec = line_spec(p0, p1);
        let off = 100.0 * eps();
        spec.carrier = Curve3::Line {
            origin: Point3::new(0.0, off, 0.0),
            dir: Vec3::unit_x(),
        };
        let err = EdgeCurve::certify(spec.clone(), p0, p1, |_| None, band()).unwrap_err();
        assert_eq!(
            err,
            CertifyError::ResidualExceeded {
                check: CertCheck::EndpointStart,
                sample: 0
            }
        );
        // An in-band offset (3·ε) escalates instead of passing.
        let mut spec = line_spec(p0, p1);
        spec.carrier = Curve3::Line {
            origin: Point3::new(0.0, 3.0 * eps(), 0.0),
            dir: Vec3::unit_x(),
        };
        let err = EdgeCurve::certify(spec.clone(), p0, p1, |_| None, band()).unwrap_err();
        assert!(matches!(err, CertifyError::Escalated { .. }), "{err:?}");
    }

    /// **`with_remapped_surfaces`** (R1 MINOR-2 for PR #325): the
    /// transplant door. Only the description's surface HANDLES may
    /// differ — carrier, interval and certificate travel verbatim —
    /// and a handle the remap cannot answer for yields `None` rather
    /// than a dangling reference.
    #[test]
    fn remapping_surface_keys_changes_the_handles_and_nothing_else() {
        let planes = || {
            vec![
                Surface::Plane {
                    origin: Point3::origin(),
                    normal: Vec3::unit_z(),
                    u_ref: Vec3::unit_x(),
                },
                Surface::Plane {
                    origin: Point3::origin(),
                    normal: Vec3::unit_y(),
                    u_ref: Vec3::unit_x(),
                },
            ]
        };
        let (src_keys, src_lookup) = table(planes());
        // A SECOND table standing for the destination body's arenas:
        // the same surface VALUES under DIFFERENT keys, which is
        // exactly what a graft produces. Two fresh slotmaps mint the
        // same slots, so the destination is offset by a filler — the
        // handles have to actually differ for this row to mean
        // anything.
        let mut dst_surfaces = vec![Surface::Plane {
            origin: Point3::origin(),
            normal: Vec3::unit_x(),
            u_ref: Vec3::unit_y(),
        }];
        dst_surfaces.extend(planes());
        let (dst_all, dst_lookup) = table(dst_surfaces);
        let dst_keys = [dst_all[1], dst_all[2]];
        assert!(
            src_keys[0] != dst_keys[0] && src_keys[1] != dst_keys[1],
            "the two tables must mint distinct handles"
        );
        let p0 = Point3::origin();
        let p1 = Point3::new(1.0, 0.0, 0.0);
        let spec = EdgeCurveSpec {
            description: EdgeGeometry::Intersection {
                s1: src_keys[0],
                s2: src_keys[1],
                witness: Point3::new(0.5, 0.0, 0.0),
            },
            carrier: Curve3::Line {
                origin: p0,
                dir: Vec3::unit_x(),
            },
            param_start: 0.0,
            param_end: 1.0,
        };
        let certified = EdgeCurve::certify(spec, p0, p1, &src_lookup, band()).unwrap();

        let bridge = |k: SurfaceKey| src_keys.iter().position(|s| *s == k).map(|i| dst_keys[i]);
        let moved = certified
            .with_remapped_surfaces(bridge)
            .expect("every named surface has a destination");

        // The handles moved.
        match *moved.description() {
            EdgeGeometry::Intersection { s1, s2, witness } => {
                assert_eq!(s1, dst_keys[0]);
                assert_eq!(s2, dst_keys[1]);
                // ...and the witness, a POINT, did not.
                assert!((witness.x - 0.5).abs() < 1e-15);
            }
            ref other => panic!("the description class changed: {other:?}"),
        }
        // Nothing else did.
        assert_eq!(
            format!("{:?}", moved.carrier()),
            format!("{:?}", certified.carrier())
        );
        assert_eq!(moved.params(), certified.params());
        assert_eq!(
            format!("{:?}", moved.certificate()),
            format!("{:?}", certified.certificate()),
            "the certificate travels verbatim — the geometry did not change"
        );
        // And the moved copy is genuinely certified AGAINST the
        // destination's surfaces: re-certification there agrees, which
        // is the claim the graft's `RemapKeys` bridge relies on.
        moved.recertify(p0, p1, &dst_lookup, band()).unwrap();

        // A remap that cannot answer writes nothing.
        assert!(
            certified.with_remapped_surfaces(|_| None).is_none(),
            "a dangling handle is never written"
        );
        // A description with no surface keys survives any remap.
        let mapped_only = EdgeCurve::certify(line_spec(p0, p1), p0, p1, |_| None, band()).unwrap();
        let same = mapped_only
            .with_remapped_surfaces(|_| None)
            .expect("a MappedCurve names no surface");
        assert_eq!(
            format!("{:?}", same.description()),
            format!("{:?}", mapped_only.description())
        );
    }

    #[test]
    fn intersection_certifies_against_both_planes() {
        // The unit-cube edge x ∈ [0,1], y = 0, z = 0 as the intersection
        // of the bottom plane (z = 0) and front plane (y = 0).
        let (keys, lookup) = table(vec![
            Surface::Plane {
                origin: Point3::origin(),
                normal: Vec3::unit_z(),
                u_ref: Vec3::unit_x(),
            },
            Surface::Plane {
                origin: Point3::origin(),
                normal: Vec3::unit_y(),
                u_ref: Vec3::unit_x(),
            },
        ]);
        let p0 = Point3::origin();
        let p1 = Point3::new(1.0, 0.0, 0.0);
        let spec = EdgeCurveSpec {
            description: EdgeGeometry::Intersection {
                s1: keys[0],
                s2: keys[1],
                witness: Point3::new(0.5, 0.0, 0.0),
            },
            carrier: Curve3::Line {
                origin: p0,
                dir: Vec3::unit_x(),
            },
            param_start: 0.0,
            param_end: 1.0,
        };
        EdgeCurve::certify(spec.clone(), p0, p1, &lookup, band()).unwrap();

        // Both-surface teeth: a carrier lying ON s1 but definitely off
        // s2 fails the s2 residual specifically.
        let mut bad = spec.clone();
        bad.carrier = Curve3::Line {
            origin: Point3::new(0.0, 0.5, 0.0), // on z = 0, off y = 0
            dir: Vec3::unit_x(),
        };
        let err = EdgeCurve::certify(
            bad,
            Point3::new(0.0, 0.5, 0.0),
            Point3::new(1.0, 0.5, 0.0),
            &lookup,
            band(),
        )
        .unwrap_err();
        assert_eq!(
            err,
            CertifyError::ResidualExceeded {
                check: CertCheck::Surface2Residual,
                sample: 0
            }
        );

        // A displaced witness fails the witness checks.
        let mut bad = spec.clone();
        bad.description = EdgeGeometry::Intersection {
            s1: keys[0],
            s2: keys[1],
            witness: Point3::new(0.5, 0.0, 0.25),
        };
        let err = EdgeCurve::certify(bad, p0, p1, &lookup, band()).unwrap_err();
        assert_eq!(
            err,
            CertifyError::ResidualExceeded {
                check: CertCheck::WitnessSurface1,
                sample: 0
            }
        );

        // Same surface twice is structurally malformed.
        let mut bad = spec.clone();
        bad.description = EdgeGeometry::Intersection {
            s1: keys[0],
            s2: keys[0],
            witness: Point3::new(0.5, 0.0, 0.0),
        };
        assert_eq!(
            EdgeCurve::certify(bad, p0, p1, &lookup, band()).unwrap_err(),
            CertifyError::IntersectionSameSurface { key: keys[0] }
        );

        // A stale key is a typed error.
        let mut bad = spec.clone();
        bad.description = EdgeGeometry::Intersection {
            s1: keys[0],
            s2: SurfaceKey::default(),
            witness: Point3::new(0.5, 0.0, 0.0),
        };
        assert_eq!(
            EdgeCurve::certify(bad, p0, p1, &lookup, band()).unwrap_err(),
            CertifyError::UnresolvedSurface {
                key: SurfaceKey::default()
            }
        );
    }

    #[test]
    fn tangent_intersection_is_refused() {
        // Two planes meeting at the run-scaled sliver angle: the
        // transversality check escalates (never a guess).
        let theta = 3.0 * eps();
        let (keys, lookup) = table(vec![
            Surface::Plane {
                origin: Point3::origin(),
                normal: Vec3::unit_z(),
                u_ref: Vec3::unit_x(),
            },
            Surface::Plane {
                origin: Point3::origin(),
                normal: Vec3::new(0.0, theta.sin(), theta.cos()),
                u_ref: Vec3::unit_x(),
            },
        ]);
        let p0 = Point3::origin();
        let p1 = Point3::new(1.0, 0.0, 0.0);
        let spec = EdgeCurveSpec {
            description: EdgeGeometry::Intersection {
                s1: keys[0],
                s2: keys[1],
                witness: Point3::new(0.5, 0.0, 0.0),
            },
            carrier: Curve3::Line {
                origin: p0,
                dir: Vec3::unit_x(),
            },
            param_start: 0.0,
            param_end: 1.0,
        };
        let err = EdgeCurve::certify(spec.clone(), p0, p1, &lookup, band()).unwrap_err();
        assert!(
            matches!(
                err,
                CertifyError::Escalated {
                    check: CertCheck::Transversality,
                    ..
                }
            ),
            "{err:?}"
        );
        // Exactly coincident planes: definitely smooth ⇒ NotTransverse.
        let (keys, lookup) = table(vec![
            Surface::Plane {
                origin: Point3::origin(),
                normal: Vec3::unit_z(),
                u_ref: Vec3::unit_x(),
            },
            Surface::Plane {
                origin: Point3::origin(),
                normal: Vec3::unit_z(),
                u_ref: Vec3::unit_y(),
            },
        ]);
        let spec = EdgeCurveSpec {
            description: EdgeGeometry::Intersection {
                s1: keys[0],
                s2: keys[1],
                witness: Point3::new(0.5, 0.0, 0.0),
            },
            carrier: Curve3::Line {
                origin: p0,
                dir: Vec3::unit_x(),
            },
            param_start: 0.0,
            param_end: 1.0,
        };
        assert_eq!(
            EdgeCurve::certify(spec.clone(), p0, p1, &lookup, band()).unwrap_err(),
            CertifyError::NotTransverse { sample: 1 }
        );
    }

    #[test]
    fn seam_certifies_on_cylinder_and_rejects_antiseam() {
        // Cylinder about +z through the origin, seam (u_ref) at +x: the
        // seam ruling is the line x = r, y = 0.
        let r = 2.0;
        let (keys, lookup) = table(vec![Surface::Cylinder {
            origin: Point3::origin(),
            axis: Vec3::unit_z(),
            radius: r,
            u_ref: Vec3::unit_x(),
        }]);
        let p0 = Point3::new(r, 0.0, 0.0);
        let p1 = Point3::new(r, 0.0, 3.0);
        let spec = EdgeCurveSpec {
            description: EdgeGeometry::Seam { surface: keys[0] },
            carrier: Curve3::Line {
                origin: p0,
                dir: Vec3::unit_z(),
            },
            param_start: 0.0,
            param_end: 3.0,
        };
        EdgeCurve::certify(spec.clone(), p0, p1, &lookup, band()).unwrap();

        // The antipodal ruling (x = −r) is on the surface and in the
        // seam plane, but on the wrong side: the side check has teeth.
        let q0 = Point3::new(-r, 0.0, 0.0);
        let q1 = Point3::new(-r, 0.0, 3.0);
        let mut bad = spec.clone();
        bad.carrier = Curve3::Line {
            origin: q0,
            dir: Vec3::unit_z(),
        };
        let err = EdgeCurve::certify(bad, q0, q1, &lookup, band()).unwrap_err();
        assert_eq!(
            err,
            CertifyError::ResidualExceeded {
                check: CertCheck::SeamSide,
                sample: 0
            }
        );

        // A seam on a plane is malformed.
        let (pkeys, plookup) = table(vec![Surface::Plane {
            origin: Point3::origin(),
            normal: Vec3::unit_z(),
            u_ref: Vec3::unit_x(),
        }]);
        let mut bad = spec.clone();
        bad.description = EdgeGeometry::Seam { surface: pkeys[0] };
        assert_eq!(
            EdgeCurve::certify(bad, p0, p1, &plookup, band()).unwrap_err(),
            CertifyError::SeamOnNonPeriodic
        );
    }

    #[test]
    fn nurbs_carrier_is_refused_and_nothing_panics_on_poison() {
        let p0 = Point3::origin();
        let p1 = Point3::new(1.0, 0.0, 0.0);
        let mut spec = line_spec(p0, p1);
        spec.carrier = Curve3::nurbs_placeholder();
        assert_eq!(
            EdgeCurve::certify(spec.clone(), p0, p1, |_| None, band()).unwrap_err(),
            CertifyError::Unimplemented
        );
        // Poisoned endpoints: typed escalation, no panic (totality).
        let spec = line_spec(p0, p1);
        let nan = Point3::new(f64::NAN, 0.0, 0.0);
        let err = EdgeCurve::certify(spec.clone(), nan, p1, |_| None, band()).unwrap_err();
        assert!(matches!(err, CertifyError::Escalated { .. }));
    }

    /// A full-period circle carrier (the scaffolding self-loop case):
    /// coincident endpoints, params (0, τ) — certifiable because the
    /// interval is a stored certified cache (module docs).
    #[test]
    fn full_period_circle_certifies() {
        use core::f64::consts::TAU;
        let center = Point3::new(1.0, 2.0, 3.0);
        let p = Point3::new(2.0, 2.0, 3.0); // center + u_ref·r
        let spec = EdgeCurveSpec {
            description: EdgeGeometry::MappedCurve(MappedCurve::RevolvedPoint {
                point: Point2::new(2.0, 2.0),
                place: Affine3::translation(Vec3::new(0.0, 0.0, 3.0)),
                axis_origin: center,
                axis_dir: Vec3::unit_z(),
                angle: TAU,
            }),
            carrier: Curve3::Circle {
                center,
                axis: Vec3::unit_z(),
                radius: 1.0,
                u_ref: Vec3::unit_x(),
            },
            param_start: 0.0,
            param_end: TAU,
        };
        EdgeCurve::certify(spec.clone(), p, p, |_| None, band()).unwrap();
    }

    /// A placed-arc mapped curve against a circle carrier: the quarter
    /// arc of PR 2's bulge conventions, certified against its Circle3.
    #[test]
    fn placed_arc_certifies_against_circle_carrier() {
        use core::f64::consts::FRAC_PI_2;
        let bulge = (core::f64::consts::PI / 8.0).tan();
        let place = Affine3::translation(Vec3::new(0.0, 0.0, 1.0));
        let spec = EdgeCurveSpec {
            description: EdgeGeometry::MappedCurve(MappedCurve::PlacedSegment {
                segment: SketchSegment::Arc {
                    a: Point2::new(1.0, 0.0),
                    b: Point2::new(0.0, 1.0),
                    bulge,
                },
                place,
            }),
            carrier: Curve3::Circle {
                center: Point3::new(0.0, 0.0, 1.0),
                axis: Vec3::unit_z(),
                radius: 1.0,
                u_ref: Vec3::unit_x(),
            },
            param_start: 0.0,
            param_end: FRAC_PI_2,
        };
        let p0 = Point3::new(1.0, 0.0, 1.0);
        let p1 = Point3::new(0.0, 1.0, 1.0);
        EdgeCurve::certify(spec.clone(), p0, p1, |_| None, band()).unwrap();
    }

    /// S6 (two-tolerance, D4 ¶1 addendum): the transversality pair —
    /// exactly-coincident tangent planes (`NotTransverse`) and in-band
    /// (`Escalated`) — is one user situation; both arms carry the
    /// shared recourse fragment.
    #[test]
    fn transversality_pair_carries_the_shared_recourse() {
        let msg = CertifyError::NotTransverse { sample: 1 }.to_string();
        assert_eq!(
            msg.matches(geom_core::COINCIDENCE_RECOURSE).count(),
            1,
            "{msg}"
        );

        let msg = CertifyError::Escalated {
            check: CertCheck::Transversality,
            sample: 1,
            cause: Indeterminate {
                margin: geom_core::MarginDiag::Value(5e-9),
                band: Band::new(1e-9, 1e-8).unwrap(),
                predicate: Some("transversality"),
            },
        }
        .to_string();
        assert_eq!(
            msg.matches(geom_core::COINCIDENCE_RECOURSE).count(),
            1,
            "{msg}"
        );
    }
}
