//! Certified carriers: the D4 ¶2 attachment gate for edge geometry.
//!
//! An edge's concrete 3-D curve (its *carrier*) is a derived cache of
//! its intensional description ([`crate::EdgeGeometry`]). This module
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

use crate::dihedral::{DihedralClass, classify_dihedral, decide};
use crate::edge_geometry::EdgeGeometry;
use crate::implicit::{implicit_residual, seam_frame};
use crate::keys::SurfaceKey;

/// The fixed certification sample count (module docs): 9 uniform
/// parameters, endpoints included.
pub const CERT_SAMPLES: u32 = 9;

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
    /// MappedCurve: `|carrier(t_i) − description(s_i)|` at a sample.
    MappedSource,
    /// Seam: implicit residual against the seam's surface at a sample.
    SeamSurface,
    /// Seam: the out-of-halfplane component `|w · v_ref|` at a sample.
    SeamHalfplane,
    /// Seam: the wrong-side excess `max(0, −w · u_ref)` at a sample
    /// (distinguishes the seam from the antipodal meridian).
    SeamSide,
    /// IsoCurve: the genuinely metric residual
    /// `|carrier(tᵢ) − S(u, v(tᵢ))|` at a sample (M6-3; the
    /// wall–wall-seam class).
    IsoResidual,
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
    /// The intensional description (authoritative).
    pub description: EdgeGeometry<T>,
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
            description: EdgeGeometry::MappedCurve(
                crate::edge_geometry::MappedCurve::ExtrudedPoint {
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
            description: EdgeGeometry::MappedCurve(
                crate::edge_geometry::MappedCurve::RevolvedPoint {
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
            description: EdgeGeometry::MappedCurve(
                crate::edge_geometry::MappedCurve::RevolvedPoint {
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
    /// (endpoint, surface, mapped-source, seam checks; transversality
    /// margins are clearance margins, not residuals, and are excluded).
    /// Certified ≤ ε by construction.
    pub max_residual: T,
}

/// A certified edge carrier: the intensional description, its cached
/// [`Curve3`] carrier with the certified parameter interval, and the
/// [`Certificate`] of the attachment-time run. Constructible only
/// through [`EdgeCurve::certify`] — fields are private so an
/// uncertified value is unrepresentable (D4 ¶2 made structural).
#[derive(Clone, Debug)]
pub struct EdgeCurve<T: Real> {
    description: EdgeGeometry<T>,
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
        let certificate = run_checks(&spec, start, end, &surfaces, None, band)?;
        Ok(Self {
            description: spec.description,
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
        run_checks(&self.spec(), start, end, &surfaces, None, band)
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
        let certificate = run_checks(
            &spec,
            start,
            end,
            &surfaces,
            Some(&T::plane_nurbs_limbs),
            band,
        )?;
        Ok(Self {
            description: spec.description,
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
    }
}

impl<T: Real> EdgeCurve<T> {
    /// The intensional description (authoritative, D2).
    pub fn description(&self) -> &EdgeGeometry<T> {
        &self.description
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
    /// A surface key is an arena handle, not geometry: `Intersection`,
    /// `TangentIntersection`, `Seam` and `IsoCurve` name the surfaces
    /// their locus is stated against, and a graft that re-creates
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
            EdgeGeometry::Intersection { s1, s2, witness } => EdgeGeometry::Intersection {
                s1: remap(s1)?,
                s2: remap(s2)?,
                witness,
            },
            EdgeGeometry::TangentIntersection { s1, s2, witness } => {
                EdgeGeometry::TangentIntersection {
                    s1: remap(s1)?,
                    s2: remap(s2)?,
                    witness,
                }
            }
            EdgeGeometry::Seam { surface } => EdgeGeometry::Seam {
                surface: remap(surface)?,
            },
            EdgeGeometry::IsoCurve { surface, u, v0, v1 } => EdgeGeometry::IsoCurve {
                surface: remap(surface)?,
                u,
                v0,
                v1,
            },
            EdgeGeometry::MappedCurve(m) => EdgeGeometry::MappedCurve(m),
        };
        Some(Self {
            description,
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
                EdgeGeometry::Intersection { s1: k1, s2: k2, .. } => EdgeGeometry::Intersection {
                    s1: k1,
                    s2: k2,
                    // The child's mid-parameter point, computed exactly
                    // as the certification schedule's middle sample
                    // (bitwise — zero WitnessMidpoint residual).
                    witness: self
                        .carrier
                        .eval(sample_param(ta, tb, (CERT_SAMPLES - 1) / 2)),
                },
                // TangentIntersection splits exactly as Intersection:
                // surfaces kept, witness re-minted at the child's own
                // mid-parameter (the witness contract, one order up).
                EdgeGeometry::TangentIntersection { s1: k1, s2: k2, .. } => {
                    EdgeGeometry::TangentIntersection {
                        s1: k1,
                        s2: k2,
                        witness: self
                            .carrier
                            .eval(sample_param(ta, tb, (CERT_SAMPLES - 1) / 2)),
                    }
                }
                EdgeGeometry::MappedCurve(mc) => EdgeGeometry::MappedCurve(mc.restrict(s0, s1)),
                EdgeGeometry::Seam { surface } => EdgeGeometry::Seam { surface },
                // The iso description restricts exactly as MappedCurve
                // does: fixed `u` kept, the `v` window mapped through
                // the SAME interval fractions the parameter interval
                // splits at, so the child's affine t↦v map agrees with
                // the parent's on the shared sub-interval.
                EdgeGeometry::IsoCurve { surface, u, v0, v1 } => EdgeGeometry::IsoCurve {
                    surface,
                    u,
                    v0: v0 + (v1 - v0) * s0,
                    v1: v0 + (v1 - v0) * s1,
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

    /// This carrier's spec view (for re-certification).
    fn spec(&self) -> EdgeCurveSpec<T> {
        EdgeCurveSpec {
            description: self.description,
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
) -> Result<Certificate<T>, CertifyError> {
    // ---- Check 1: implementedness / description well-formedness. ----
    // Rung-3 (`Nurbs`) carriers certify under an `Intersection`
    // description of two ANALYTIC surfaces — the class the curved
    // boolean zip mints (M5 PR 9, C12.3; the fitted SSI branch) — and
    // under an `IsoCurve` description (M6-3: the loft/sweep wall–wall
    // seam class, whose residual is the genuinely metric
    // `|C(t) − S(u, v(t))|`). A `Nurbs` carrier under the OTHER
    // conventional descriptions (`MappedCurve`/`Seam`) stays refused:
    // nothing mints one, and its residual story (a fitted carrier
    // "matching" a mapped source) has no certified meter.
    if matches!(spec.carrier, Curve3::Nurbs(_))
        && !matches!(
            spec.description,
            EdgeGeometry::Intersection { .. }
                | EdgeGeometry::TangentIntersection { .. }
                | EdgeGeometry::IsoCurve { .. }
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
        Mapped(crate::edge_geometry::MappedCurve<T>),
        Seam(Surface<T>),
        Iso {
            surface: Surface<T>,
            u: T,
            v0: T,
            v1: T,
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
        EdgeGeometry::Intersection { s1, s2, witness } => {
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
        EdgeGeometry::TangentIntersection { s1, s2, witness } => {
            // A same-surface "tangency" is a Seam exactly as a
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
        EdgeGeometry::MappedCurve(mc) => Resolved::Mapped(mc),
        EdgeGeometry::Seam { surface } => {
            let s = resolve(surface)?;
            // Periodicity is structural: the plane is the one
            // non-periodic analytic kind (Nurbs already rejected).
            if matches!(s, Surface::Plane { .. }) {
                return Err(CertifyError::SeamOnNonPeriodic);
            }
            Resolved::Seam(s)
        }
        EdgeGeometry::IsoCurve { surface, u, v0, v1 } => Resolved::Iso {
            surface: resolve_iso(surface)?,
            u,
            v0,
            v1,
        },
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
            Resolved::Mapped(mc) => {
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
            Resolved::Seam(s) => {
                check_residual(
                    "carrier_on_seam_surface",
                    CertCheck::SeamSurface,
                    i,
                    Margin::of(implicit_residual(s, p)),
                    band,
                    &mut max_residual,
                )?;
                // seam_frame is Some: plane and Nurbs were rejected in
                // check 1, and every remaining kind is axisymmetric.
                if let Some((w, u_ref, v_ref)) = seam_frame(s, p) {
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
            // The iso lane (M6-3): the genuinely metric residual
            // |C(tᵢ) − S(u, v(tᵢ))| with v affine in the parameter —
            // the same schedule fraction the Mapped arm uses, so the
            // stated formula and the evaluated one share their bits.
            Resolved::Iso { surface, u, v0, v1 } => {
                let frac = T::from_f64(f64::from(i) / f64::from(CERT_SAMPLES - 1));
                let v = *v0 + (*v1 - *v0) * frac;
                check_residual(
                    "carrier_on_iso_curve",
                    CertCheck::IsoResidual,
                    i,
                    Margin::of(p.distance(surface.eval(*u, v))),
                    band,
                    &mut max_residual,
                )?;
            }
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

    Ok(Certificate {
        samples: CERT_SAMPLES,
        max_residual,
    })
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
    use geom_core::{Affine3, Point2, Vec3};

    use crate::edge_geometry::{MappedCurve, SketchSegment};

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
