//! Carrier-side conic **pcurve constructors** (M5 PR 5, spec §5 — the
//! C4 slice): the 2-D chart images of conic 3-D carriers, closed-form
//! where the chart map keeps the conic exact, fitted through the PR 4
//! machinery where it is transcendental.
//!
//! **Scope honesty**: PR 6 owns per-half-edge cache storage, the
//! meters-through-the-map certification apparatus, and domain-validity
//! semantics (trim containment, the one-branch periodic unwrap). This
//! module provides only the constructors PR 6 will store — pure
//! functions from carrier + chart to a [`NurbsCurve2`] pcurve.
//!
//! # The two chart classes
//!
//! - **Plane charts** ([`ellipse_pcurve_on_plane`]): the chart map is
//!   affine, so a conic maps to a conic — represented EXACTLY as the
//!   rational-quadratic NURBS arc chain (Book §7.3: per-segment middle
//!   weight `w = cos(δ/2)`, shape factor `w₀w₂/w₁² = 1/w² > 1`, the
//!   elliptic class). Affine maps commute with the rational-quadratic
//!   construction (same weights, mapped control points), so the chain
//!   is built directly in chart coordinates from the unit-circle arcs
//!   through the ellipse's frame — locus-exact in ℝ. The NURBS
//!   parameter is the conic's rational one, NOT the eccentric anomaly
//!   θ (exactly C1's grounds for keeping `Ellipse` as the 3-D carrier;
//!   a pcurve is a chart-image cache, where the locus is what
//!   certification consumes). Per-segment symmetry pins the θ
//!   correspondence at parameter 0, ½, 1 of each segment.
//! - **Cylinder charts** ([`ellipse_pcurve_on_cylinder`]): the tilted
//!   section's chart graph is `(u, v(u))` with
//!   `v(u) = v₀ + A·cos u + B·sin u` — transcendental in the chart, so
//!   the pcurve is FITTED (`NurbsCurve2::approximate`, the PR 4 Type-2
//!   loop) over a fixed uniform sample schedule (C6: structure is an
//!   f64-lane artifact, deterministic per D9 — same inputs, same
//!   knots/bits). The azimuth samples are unwrapped continuously from
//!   the arc's start (u values may leave `[0, τ)`; the one-branch
//!   unwrap pin is PR 6's).

use geom::Surface;
use geom::{Curve3, FitError, FitOutcome, NurbsCurve2};
use geom_core::Point2;
use geom_core::spline::KnotVector;

/// Typed refusal of a pcurve constructor.
#[derive(Debug)]
pub enum PcurveError {
    /// The carrier/chart pair is not the lane's (caller bug, loud).
    WrongLane {
        /// What the lane expected.
        expected: &'static str,
    },
    /// The arc span is empty or not finite — no pcurve to build.
    DegenerateSpan,
    /// The arc span exceeds one full period: the rational-quadratic
    /// chain would self-overlap (review n2 — the carrier-side
    /// constructors enforce the same |span| ≤ τ bound certification's
    /// winding gate pins on edges).
    SpanExceedsPeriod,
    /// The rational-quadratic chain's structure refused (unreachable
    /// for valid spans; typed rather than assumed).
    Structure(geom_core::spline::SplineError),
    /// The PR 4 fitting loop refused (typed, nested whole).
    Fit(FitError),
}

impl core::fmt::Display for PcurveError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::WrongLane { expected } => {
                write!(f, "pcurve: wrong lane — this constructor maps {expected}")
            }
            Self::DegenerateSpan => write!(f, "pcurve: degenerate parameter span"),
            Self::SpanExceedsPeriod => write!(
                f,
                "pcurve: the arc span exceeds one full period (a self-overlapping \
                 chain) — split the edge first; certified edges never carry one \
                 (the winding gate)"
            ),
            Self::Structure(e) => write!(f, "pcurve: {e}"),
            Self::Fit(e) => write!(f, "pcurve: {e}"),
        }
    }
}

impl std::error::Error for PcurveError {}

impl From<FitError> for PcurveError {
    fn from(e: FitError) -> Self {
        Self::Fit(e)
    }
}

/// The fixed sample count of the cylinder-chart fitting schedule (D9:
/// a named constant, never data-dependent): dense enough that the
/// chord interpolant of the graph is well below any practical fit
/// tolerance over a full period.
pub const PCURVE_FIT_SAMPLES: usize = 129;

/// The exact rational-quadratic pcurve of an **ellipse arc in a plane
/// chart** (the section plane itself, or any plane containing the
/// ellipse): the arc `θ ∈ [t0, t1]` of the carrier, as a chain of
/// ≤ 90° rational-quadratic segments in chart coordinates
/// (module docs; locus-exact in ℝ).
///
/// # Errors
///
/// [`PcurveError`] — wrong kinds, a degenerate span, or (unreachable
/// for valid spans) a structure refusal.
pub fn ellipse_pcurve_on_plane(
    carrier: &Curve3<f64>,
    plane: &Surface<f64>,
    t0: f64,
    t1: f64,
) -> Result<NurbsCurve2<f64>, PcurveError> {
    let &Curve3::Ellipse {
        center,
        axis,
        major,
        minor,
        u_ref,
    } = carrier
    else {
        return Err(PcurveError::WrongLane {
            expected: "an Ellipse carrier into a Plane chart",
        });
    };
    let &Surface::Plane {
        origin: q,
        normal: n,
        u_ref: pu,
    } = plane
    else {
        return Err(PcurveError::WrongLane {
            expected: "an Ellipse carrier into a Plane chart",
        });
    };
    let span = t1 - t0;
    if !(span.is_finite() && span > 0.0) {
        return Err(PcurveError::DegenerateSpan);
    }
    if span > core::f64::consts::TAU {
        return Err(PcurveError::SpanExceedsPeriod);
    }
    // Chart frame: S(u, v) = q + pu·u + pv·v with pv = n × pu.
    let pv = n.cross(pu);
    // The ellipse's chart-affine data: E(θ) = c₂ + A₂·cos θ + B₂·sin θ.
    let e_v = axis.cross(u_ref);
    let w0 = center - q;
    let c2 = Point2::new(w0.dot(pu), w0.dot(pv));
    let a2 = Point2::new(u_ref.dot(pu) * major, u_ref.dot(pv) * major);
    let b2 = Point2::new(e_v.dot(pu) * minor, e_v.dot(pv) * minor);
    // D9: all trig through the geom_core::Real routes (libm) — std
    // f64 trig differs cross-platform in the last ulp, and PR 6 will
    // STORE these caches (M2 fix, adversarial review).
    let at = |theta: f64| -> Point2<f64> {
        let (s, c) = geom_core::Real::sin_cos(theta);
        Point2::new(c2.x + a2.x * c + b2.x * s, c2.y + a2.y * c + b2.y * s)
    };
    // Tangent-intersection "shoulder" of the segment [α, α + δ]: the
    // unit-circle shoulder (cos(α+δ/2), sin(α+δ/2))/cos(δ/2) through
    // the same affine map — control points map, weights transfer.
    let shoulder = |alpha: f64, delta: f64| -> Point2<f64> {
        let m = alpha + delta * 0.5;
        let (s, c) = geom_core::Real::sin_cos(m);
        let k = 1.0 / geom_core::Real::cos(delta * 0.5);
        Point2::new(
            c2.x + (a2.x * c + b2.x * s) * k,
            c2.y + (a2.y * c + b2.y * s) * k,
        )
    };
    // ≤ 90° segments (Book §7.3's arbitrary-sweep chain).
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let segments = {
        let raw = (span / core::f64::consts::FRAC_PI_2).ceil();
        if raw < 1.0 { 1 } else { raw as usize }
    };
    #[allow(clippy::cast_precision_loss)]
    let delta = span / segments as f64;
    let w_mid = geom_core::Real::cos(delta * 0.5);
    let mut control = Vec::with_capacity(2 * segments + 1);
    let mut weights = Vec::with_capacity(2 * segments + 1);
    let mut knots = Vec::with_capacity(2 * segments + 4);
    knots.extend_from_slice(&[0.0, 0.0, 0.0]);
    control.push(at(t0));
    weights.push(1.0);
    for i in 0..segments {
        #[allow(clippy::cast_precision_loss)]
        let alpha = t0 + delta * i as f64;
        control.push(shoulder(alpha, delta));
        weights.push(w_mid);
        control.push(at(alpha + delta));
        weights.push(1.0);
        #[allow(clippy::cast_precision_loss)]
        let s_end = (i + 1) as f64 / segments as f64;
        if i + 1 < segments {
            knots.extend_from_slice(&[s_end, s_end]);
        }
    }
    knots.extend_from_slice(&[1.0, 1.0, 1.0]);
    let kv = KnotVector::clamped(knots, 2).map_err(PcurveError::Structure)?;
    NurbsCurve2::new(kv, control, weights).map_err(PcurveError::Structure)
}

/// The fitted pcurve of a **tilted-section ellipse on its cylinder
/// chart** (module docs): the graph `(u, v(u))` sampled on the fixed
/// [`PCURVE_FIT_SAMPLES`] schedule over the arc's azimuth interval
/// (unwrapped from the start azimuth) and fitted through the PR 4
/// Type-2 loop at `degree` / `tolerance` (chart units — radians on u,
/// meters on v; the meters-through-the-map certification is PR 6's).
///
/// The ellipse must be a section of THIS cylinder (the eccentric
/// anomaly θ then EQUALS the cylinder azimuth up to the seam offset —
/// the plane×cylinder construction's frame fact); the constructor
/// consumes that correspondence and the certification lane downstream
/// re-verifies it in meters.
///
/// # Errors
///
/// [`PcurveError`] — wrong kinds, degenerate spans, or the typed
/// fitting refusals nested whole.
pub fn ellipse_pcurve_on_cylinder(
    carrier: &Curve3<f64>,
    cylinder: &Surface<f64>,
    t0: f64,
    t1: f64,
    degree: usize,
    tolerance: f64,
) -> Result<FitOutcome<NurbsCurve2<f64>>, PcurveError> {
    let Curve3::Ellipse { .. } = carrier else {
        return Err(PcurveError::WrongLane {
            expected: "an Ellipse carrier onto its Cylinder chart",
        });
    };
    let &Surface::Cylinder {
        origin: o,
        axis: ax,
        u_ref: cu,
        ..
    } = cylinder
    else {
        return Err(PcurveError::WrongLane {
            expected: "an Ellipse carrier onto its Cylinder chart",
        });
    };
    let span = t1 - t0;
    if !(span.is_finite() && span > 0.0) {
        return Err(PcurveError::DegenerateSpan);
    }
    if span > core::f64::consts::TAU {
        return Err(PcurveError::SpanExceedsPeriod);
    }
    let cv = ax.cross(cu);
    // The chart coordinates of a carrier point: azimuth from atan2 in
    // the cylinder frame (unwrapped continuously from the previous
    // sample — the graph is monotone in θ, steps < π by the schedule),
    // height along the axis.
    let mut points = Vec::with_capacity(PCURVE_FIT_SAMPLES);
    let mut prev_u = 0.0f64;
    for i in 0..PCURVE_FIT_SAMPLES {
        #[allow(clippy::cast_precision_loss)]
        let theta = t0 + span * (i as f64 / (PCURVE_FIT_SAMPLES - 1) as f64);
        let p = carrier.eval(theta);
        let w = p - o;
        let radial = w - ax * w.dot(ax);
        let mut u = geom_core::Real::atan2(radial.dot(cv), radial.dot(cu));
        if i > 0 {
            // Unwrap: shift by whole periods to land within (−π, π]
            // of the previous sample.
            let tau = core::f64::consts::TAU;
            u += ((prev_u - u) / tau).round() * tau;
        }
        prev_u = u;
        let v = w.dot(ax);
        points.push(Point2::new(u, v));
    }
    Ok(NurbsCurve2::approximate(&points, degree, tolerance)?)
}
