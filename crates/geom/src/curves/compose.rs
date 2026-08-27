//! **The chain→curve composition door** (LIBRARY-DESIGN §L7, LQ3(b)):
//! an ordered chain of exact NURBS pieces — the rational forms of
//! lines, circular arcs, and authored NURBS legs — joined into ONE
//! [`NurbsCurve3`], which is the single curve `§10.4`'s sweep consumer
//! takes. This is the anti-interpolate: **the LOCUS is never sampled,
//! fitted, or approximated**. The legs' control points and weights
//! enter the result verbatim (up to the one positive weight rescale
//! each seam needs); only the KNOT VECTOR is rebuilt.
//!
//! Precisely (R2's catch — the older wording overclaimed): the G¹
//! predicate and the whole control net are pure structure, but the C¹
//! step *does* evaluate — [`endpoint_speed`] reads `deriv` at both ends
//! of every leg to size its parameter interval. That evaluation feeds
//! the KNOT VECTOR only. No control point, no weight, and therefore no
//! point of the locus depends on it, so the exactness contract below is
//! untouched by it.
//!
//! Name note: `geom_core::spline::compose` is unrelated work (ring
//! coefficient composites `f ∘ C`) that happens to share the word and
//! an error name. `ComposeError` here is the spec's bound name for this
//! door's refusal; the two never meet in one scope.
//!
//! # What the door guarantees
//!
//! - **C⁰ by construction, not by comparison.** The composed control
//!   net carries exactly ONE point at each junction: leg `i`'s last
//!   control point. Leg `i+1`'s first control point is a *redundant
//!   description* of the same junction — it is dropped, never
//!   value-matched against its twin. This is the authored-once
//!   doctrine (`docs/PATHS-DESIGN.md` §"The two exactness contracts")
//!   read at the geometry layer: the junction is shared data, so there
//!   is no second value for a tolerance to adjudicate. A
//!   [`ComposeError::SeamGap`] backstop refuses a *definitely*
//!   separated pair — it never certifies coincidence, and C⁰ does not
//!   rest on it.
//! - **G¹ verified, per seam, from control data.** The end tangent
//!   *direction* of a clamped NURBS is the direction of its last
//!   control difference (positive scalar factors only), so the seam
//!   predicate is exact structure, not an evaluation: the two control
//!   differences must be parallel ([`ComposeError::SeamNotTangent`])
//!   and co-oriented ([`ComposeError::SeamTangentReversed`], or
//!   [`ComposeError::SeamTangentUncertifiable`] when the shorter
//!   difference is itself sub-band). This is the
//!   reparameterization-INVARIANT half of C¹ — the half no
//!   parameterization can repair, hence the refusal.
//! - **C¹ arranged, exactly.** The remaining half — matching tangent
//!   *magnitudes* — is a parameterization choice, so the door makes
//!   it rather than refusing it: each leg is affinely remapped onto a
//!   parameter interval whose length equalizes the one-sided
//!   derivatives across every seam. The composed curve is delivered on
//!   `[0, 1]`.
//!
//! # Exactness contract
//!
//! Composing exact pieces yields the exact rational form of their
//! union. Concretely, for legs already of the common degree and with
//! matching seam weights (the case every exact arc/line chain is in):
//! every control point of the result is a leg control point *bit for
//! bit*, every weight is a leg weight *bit for bit*, and the knots are
//! the legs' knots under one affine map per leg. The pinned row is
//! `compose(quarter arc, quarter arc)` against the half arc's own
//! rational-quadratic chain (`tests/curves/compose.rs`), which agrees on all
//! three channels bit for bit.
//!
//! Two normalizations are stated rather than hidden, because they are
//! where "the representation does not admit it":
//!
//! 1. **Degree.** Legs of unequal degree are raised to the chain's
//!    maximum by [`NurbsCurve3::elevate_degree`] (the crate's
//!    KnotAlgebra route — Bézier decompose, binomial elevate,
//!    recompose). Elevation is evaluation-invariant in ℝ but NOT
//!    bit-preserving, so a mixed-degree chain's control net is the
//!    ELEVATED legs' verbatim, not the authored legs'.
//! 2. **Weights.** A rational curve is invariant under a positive
//!    scale of all its weights, so each leg after the first is scaled
//!    once, by the factor that makes its start weight equal the
//!    running end weight. The factor is `1.0` — hence bit-neutral —
//!    for every chain whose legs are authored with unit endpoint
//!    weights, which is every exact line and every arc chain built the
//!    standard way.
//!
//! The interior seam knot keeps multiplicity `p`, exactly as the
//! canonical rational-quadratic arc chain does at its own 90° joints:
//! the C¹ that holds there is a fact about the CONTROL DATA, not about
//! the knot structure, and dropping the multiplicity to `p − 1` would
//! have to move control points. This door never moves a control point.

use geom_core::spline::{KnotAlgebraError, KnotVector, SplineError};
use geom_core::{Band, Indeterminate, Margin, Point3, Sign, Vec3, k_stats};

use crate::curves::NurbsCurve3;

/// Which side of a seam a per-seam refusal is about.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SeamSide {
    /// The leg arriving at the seam (its last control difference).
    Incoming,
    /// The leg leaving the seam (its first control difference).
    Outgoing,
}

/// Typed refusal of [`compose_chain`]. Every seam-local variant names
/// the seam index (`seam` = `i` for the junction between leg `i` and
/// leg `i + 1`) and the property that failed there.
#[derive(Clone, Debug, PartialEq)]
pub enum ComposeError {
    /// No legs were supplied: there is no curve to compose.
    EmptyChain,
    /// Leg `leg` has no tangent to join with: fewer than two control
    /// points, or degree zero.
    DegenerateLeg {
        /// Index of the offending leg.
        leg: usize,
    },
    /// Leg `leg` refused degree elevation to the chain's common
    /// degree.
    ElevationRefused {
        /// Index of the offending leg.
        leg: usize,
        /// The knot-algebra refusal.
        source: KnotAlgebraError,
    },
    /// The control difference that carries the seam tangent has zero
    /// length on the named side, so the seam has no direction to
    /// verify.
    SeamTangentUndefined {
        /// Index of the seam (between legs `seam` and `seam + 1`).
        seam: usize,
        /// Which side is degenerate.
        side: SeamSide,
    },
    /// The two legs meeting at this seam are **definitely apart** —
    /// the dropped description of the junction is not the junction.
    /// The door composes C⁰ by construction, so this is a backstop
    /// against nonsense input, not the C⁰ mechanism.
    SeamGap {
        /// Index of the seam.
        seam: usize,
        /// The refused separation, in metres.
        gap: f64,
    },
    /// The seam is not G¹: the incoming and outgoing tangent
    /// directions are not parallel.
    SeamNotTangent {
        /// Index of the seam.
        seam: usize,
        /// The refused perpendicular deviation, in metres.
        deviation: f64,
    },
    /// The seam is tangent but **reversed**: the two directions are
    /// anti-parallel (a cusp), so the chain doubles back. Reached only
    /// on a DEFINITE negative advance — an uncertifiable one is
    /// [`ComposeError::SeamTangentUncertifiable`].
    SeamTangentReversed {
        /// Index of the seam.
        seam: usize,
    },
    /// The seam's direction cannot be certified either way: the named
    /// side's control difference is itself sub-band, so neither
    /// "forward" nor "reversed" is established. Refusing is right —
    /// asserting a cusp would not be.
    SeamTangentUncertifiable {
        /// Index of the seam.
        seam: usize,
        /// The side whose control difference is the shorter one.
        side: SeamSide,
    },
    /// A seam predicate could not be certified (the margin lies in the
    /// ambiguity band, or is invalid).
    SeamEscalated {
        /// Index of the seam.
        seam: usize,
        /// The classifier's diagnostic.
        diag: Indeterminate,
    },
    /// A leg's one-sided derivative is not finite and strictly
    /// positive, so the C¹ parameter scaling has nothing to solve for.
    SpeedNotUsable {
        /// Index of the leg.
        leg: usize,
        /// Which end of the leg.
        side: SeamSide,
    },
    /// The chain's total parameter span, after the C¹ scaling, is not
    /// finite and strictly positive — there is no interval to deliver
    /// the composed curve on.
    DegenerateSpan {
        /// The refused total.
        total: f64,
    },
    /// The accumulated parameter scaling left a knot vector the
    /// clamped-v1 invariants refuse (non-finite or non-ascending
    /// after the affine remaps).
    Knots(SplineError),
    /// The composed control net, weights, and knots did not form a
    /// valid curve.
    Structure(SplineError),
}

impl core::fmt::Display for ComposeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::EmptyChain => write!(f, "empty chain: no legs to compose"),
            Self::DegenerateLeg { leg } => {
                write!(
                    f,
                    "leg {leg} has no tangent (degree 0 or a single control point)"
                )
            }
            Self::ElevationRefused { leg, source } => {
                write!(f, "leg {leg} refused degree elevation: {source}")
            }
            Self::SeamTangentUndefined { seam, side } => {
                write!(
                    f,
                    "seam {seam}: the {side:?} control difference has zero length"
                )
            }
            Self::SeamGap { seam, gap } => {
                write!(f, "seam {seam}: the legs are definitely apart ({gap} m)")
            }
            Self::SeamNotTangent { seam, deviation } => {
                write!(
                    f,
                    "seam {seam}: tangents are not parallel ({deviation} m deviation)"
                )
            }
            Self::SeamTangentReversed { seam } => {
                write!(f, "seam {seam}: tangents are anti-parallel (a cusp)")
            }
            Self::SeamTangentUncertifiable { seam, side } => write!(
                f,
                "seam {seam}: the {side:?} control difference is sub-band, so the seam's \
                 direction cannot be certified"
            ),
            Self::SeamEscalated { seam, diag } => {
                write!(f, "seam {seam}: predicate could not be certified: {diag}")
            }
            Self::SpeedNotUsable { leg, side } => {
                write!(
                    f,
                    "leg {leg}: the {side:?} derivative is not finite and positive"
                )
            }
            Self::DegenerateSpan { total } => {
                write!(f, "the chain's total parameter span is {total}, not usable")
            }
            Self::Knots(e) => write!(f, "composed knot vector refused: {e}"),
            Self::Structure(e) => write!(f, "composed curve refused: {e}"),
        }
    }
}

impl std::error::Error for ComposeError {}

/// The composed curve of an ordered chain of exact NURBS legs — the
/// door this module documents.
///
/// `legs` is the chain in order. **The junction between consecutive
/// legs is leg `i`'s last control point**: leg `i + 1`'s first control
/// point describes the same junction and is dropped. Author each leg
/// FROM the previous leg's end (`legs[i].control().last()`) so the two
/// descriptions are one value; the door does not, and by the kernel's
/// retirement of the bit-identity coincidence channel cannot, decide
/// that two independently computed points are "the same".
///
/// `band` is the classify band the seam predicates decide against,
/// constructed once at operation entry per the [`geom_core::predicate`]
/// calling convention (`Band::linear(tol)` in production).
///
/// The result is delivered on `[0, 1]` with degree `max_i deg(leg_i)`.
///
/// # Errors
///
/// [`ComposeError`] — see each variant; every seam-local refusal names
/// its seam index and the property that failed.
pub fn compose_chain(
    legs: &[NurbsCurve3<f64>],
    band: Band,
) -> Result<NurbsCurve3<f64>, ComposeError> {
    let Some(degree) = legs.iter().map(NurbsCurve3::degree).max() else {
        return Err(ComposeError::EmptyChain);
    };

    // 1. One common degree, through the crate's KnotAlgebra route.
    let mut raised: Vec<NurbsCurve3<f64>> = Vec::with_capacity(legs.len());
    for (leg, curve) in legs.iter().enumerate() {
        if curve.degree() == 0 || curve.control().len() < 2 {
            return Err(ComposeError::DegenerateLeg { leg });
        }
        let raise = degree - curve.degree();
        if raise == 0 {
            raised.push(curve.clone());
        } else {
            raised.push(
                curve
                    .elevate_degree(raise)
                    .map_err(|source| ComposeError::ElevationRefused { leg, source })?,
            );
        }
    }

    // 2. Per-seam G¹ verification, on control data (exact structure,
    //    no evaluation): the clamped end tangent's DIRECTION is the
    //    direction of the last control difference.
    //
    //    INVARIANT: the predicates read the ELEVATED legs' nets, not
    //    the authored ones. Sound because Bézier degree elevation fixes
    //    the clamped END control points and preserves the end tangent
    //    direction exactly; only interior points move, and none of
    //    those is read here.
    for seam in 0..raised.len().saturating_sub(1) {
        let arrive = end_difference(&raised[seam]).ok_or(ComposeError::SeamTangentUndefined {
            seam,
            side: SeamSide::Incoming,
        })?;
        let depart =
            start_difference(&raised[seam + 1]).ok_or(ComposeError::SeamTangentUndefined {
                seam,
                side: SeamSide::Outgoing,
            })?;
        verify_seam(seam, arrive, depart, band)?;
    }

    // 3. The C¹ parameter scaling and the seam weight rescales.
    let scales = parameter_scales(&raised)?;
    let weight_scales = weight_scales(&raised);

    assemble(&raised, degree, &scales, &weight_scales)
}

/// One side of a seam, read off the control net: the junction point the
/// leg contributes, and the control difference whose DIRECTION is the
/// leg's end tangent there.
///
/// Carrying the two together is what lets [`verify_seam`] be total —
/// the point is known to exist exactly when the difference does, so
/// there is no second fallible lookup to mislabel.
#[derive(Clone, Copy)]
struct SeamEnd {
    /// The leg's endpoint at this seam (`P_n` arriving, `P_0` leaving).
    point: Point3<f64>,
    /// `P_n − P_{n−1}` arriving, `P_1 − P_0` leaving. Never zero.
    difference: Vec3<f64>,
}

/// The arriving side: `P_n` with `P_n − P_{n−1}`, or `None` when the
/// difference is exactly zero (a repeated end control point: no
/// direction to verify).
fn end_difference(curve: &NurbsCurve3<f64>) -> Option<SeamEnd> {
    let control = curve.control();
    let n = control.len();
    let point = *control.get(n - 1)?;
    let difference = point - *control.get(n - 2)?;
    (difference.norm() > 0.0).then_some(SeamEnd { point, difference })
}

/// The leaving side: `P_0` with `P_1 − P_0`, or `None` when the
/// difference is exactly zero.
fn start_difference(curve: &NurbsCurve3<f64>) -> Option<SeamEnd> {
    let control = curve.control();
    let point = *control.first()?;
    let difference = *control.get(1)? - point;
    (difference.norm() > 0.0).then_some(SeamEnd { point, difference })
}

/// The three per-seam decisions: the definite-separation backstop, the
/// parallelism gate, and the co-orientation gate.
fn verify_seam(
    seam: usize,
    arrive: SeamEnd,
    depart: SeamEnd,
    band: Band,
) -> Result<(), ComposeError> {
    let (incoming, outgoing) = (arrive.difference, depart.difference);

    // The backstop. `Margin::norm3`: the componentwise metre gap
    // between the two descriptions of the junction. Only a DEFINITE
    // separation refuses — `Zero` and an in-band margin both pass,
    // because C⁰ is established by construction (one control point at
    // the junction) and this decision is not allowed to claim
    // otherwise.
    let gap = depart.point - arrive.point;
    if let Ok(Sign::Positive) = k_stats::decide("compose_seam_gap", Margin::norm3(gap), band) {
        return Err(ComposeError::SeamGap {
            seam,
            gap: gap.norm(),
        });
    }

    // Parallelism. `Margin::over_lever`: the cross product's magnitude
    // is an AREA (m²); dividing by the longer of the two control
    // differences (a length) leaves the perpendicular deviation of the
    // shorter one from the longer one's line — the point displacement
    // the non-tangency subtends at this seam's own lever.
    let lever = incoming.norm().max(outgoing.norm());
    let deviation = incoming.cross(outgoing).norm();
    match k_stats::decide(
        "compose_seam_tangent",
        Margin::over_lever(deviation, lever),
        band,
    ) {
        Ok(Sign::Zero) => {}
        Ok(Sign::Positive | Sign::Negative) => {
            return Err(ComposeError::SeamNotTangent {
                seam,
                deviation: deviation / lever,
            });
        }
        Err(diag) => return Err(ComposeError::SeamEscalated { seam, diag }),
    }

    // Co-orientation. Same door, same lever: the dot product is an
    // area, and over the lever it is the signed advance of the
    // shorter difference along the longer one's direction.
    //
    // INVARIANT on the `Zero` arm: reaching it means BOTH seam margins
    // classified `Zero`, i.e. `min·|sin θ| ≤ z` and `min·|cos θ| ≤ z`
    // for the shorter difference `min`. Since `|sin| + |cos| ≥ 1` that
    // forces `min ≤ 2z` — the short side is itself sub-band, so the
    // seam's direction is UNCERTIFIABLE, not reversed. Naming it a cusp
    // would assert an anti-parallelism the margin never established
    // (the fix R1 caught); the refusal stands, the property is honest.
    match k_stats::decide(
        "compose_seam_forward",
        Margin::over_lever(incoming.dot(outgoing), lever),
        band,
    ) {
        Ok(Sign::Positive) => Ok(()),
        Ok(Sign::Negative) => Err(ComposeError::SeamTangentReversed { seam }),
        Ok(Sign::Zero) => Err(ComposeError::SeamTangentUncertifiable {
            seam,
            side: if incoming.norm() <= outgoing.norm() {
                SeamSide::Incoming
            } else {
                SeamSide::Outgoing
            },
        }),
        Err(diag) => Err(ComposeError::SeamEscalated { seam, diag }),
    }
}

/// The per-leg parameter scale `r_i` that makes the join C¹: with leg
/// `i` remapped by `u = a_i + (t − α_i)·r_i`, its global derivative is
/// its own divided by `r_i`, so equalizing across seam `i` reads
/// `r_{i+1} = r_i · h_{i+1} / g_i` (`g` the arriving speed, `h` the
/// departing one). `r_0 = 1`.
fn parameter_scales(legs: &[NurbsCurve3<f64>]) -> Result<Vec<f64>, ComposeError> {
    let mut scales = Vec::with_capacity(legs.len());
    let mut r = 1.0_f64;
    scales.push(r);
    for (index, pair) in legs.windows(2).enumerate() {
        let (arrive, depart) = (&pair[0], &pair[1]);
        let g = endpoint_speed(arrive, EndpointEnd::Last).ok_or(ComposeError::SpeedNotUsable {
            leg: index,
            side: SeamSide::Incoming,
        })?;
        let h = endpoint_speed(depart, EndpointEnd::First).ok_or(ComposeError::SpeedNotUsable {
            leg: index + 1,
            side: SeamSide::Outgoing,
        })?;
        r *= h / g;
        if !(r.is_finite() && r > 0.0) {
            return Err(ComposeError::SpeedNotUsable {
                leg: index + 1,
                side: SeamSide::Outgoing,
            });
        }
        scales.push(r);
    }
    Ok(scales)
}

/// Which end of a leg's own parameter domain a speed is read at.
#[derive(Clone, Copy)]
enum EndpointEnd {
    First,
    Last,
}

/// `|dP/dt|` at one end of the leg's own domain, or `None` when it is
/// not finite and strictly positive.
fn endpoint_speed(curve: &NurbsCurve3<f64>, end: EndpointEnd) -> Option<f64> {
    let (lo, hi) = curve.domain();
    let t = match end {
        EndpointEnd::First => lo,
        EndpointEnd::Last => hi,
    };
    let speed = curve.deriv(t).norm();
    (speed.is_finite() && speed > 0.0).then_some(speed)
}

/// The per-leg weight scale `λ_i`: a rational curve is invariant under
/// a positive scale of ALL its weights, so each leg after the first is
/// scaled once so that its start weight meets the running end weight.
/// `λ_0 = 1`, and `λ_i = 1.0` bit-exactly for every chain whose legs
/// carry unit endpoint weights.
fn weight_scales(legs: &[NurbsCurve3<f64>]) -> Vec<f64> {
    let mut scales = Vec::with_capacity(legs.len());
    let mut lambda = 1.0_f64;
    scales.push(lambda);
    for pair in legs.windows(2) {
        let arrive = pair[0].weights().last().copied().unwrap_or(1.0);
        let depart = pair[1].weights().first().copied().unwrap_or(1.0);
        lambda *= arrive / depart;
        scales.push(lambda);
    }
    scales
}

/// Builds the composed curve: knots by per-leg affine remap onto the
/// C¹ intervals, normalized to `[0, 1]`, with multiplicity `p` at each
/// seam; control points and weights verbatim (one junction point, one
/// weight scale per leg).
fn assemble(
    legs: &[NurbsCurve3<f64>],
    degree: usize,
    scales: &[f64],
    weight_scales: &[f64],
) -> Result<NurbsCurve3<f64>, ComposeError> {
    // Unnormalized leg starts `a_i` and the total span.
    let mut starts = Vec::with_capacity(legs.len());
    let mut cursor = 0.0_f64;
    for (leg, curve) in legs.iter().enumerate() {
        starts.push(cursor);
        let (lo, hi) = curve.domain();
        let r = scales.get(leg).copied().unwrap_or(1.0);
        cursor += (hi - lo) * r;
    }
    let total = cursor;
    if !(total.is_finite() && total > 0.0) {
        return Err(ComposeError::DegenerateSpan { total });
    }

    let last = legs.len() - 1;
    let mut knots: Vec<f64> = vec![0.0; degree + 1];
    let mut control: Vec<Point3<f64>> = Vec::new();
    let mut weights: Vec<f64> = Vec::new();

    for (leg, curve) in legs.iter().enumerate() {
        let (lo, _) = curve.domain();
        let a = starts.get(leg).copied().unwrap_or(0.0);
        let r = scales.get(leg).copied().unwrap_or(1.0);
        let lambda = weight_scales.get(leg).copied().unwrap_or(1.0);
        let remap = |k: f64| (a + (k - lo) * r) / total;

        // Interior knots only: the clamped end blocks are supplied by
        // the composed vector's own ends and by the seam blocks.
        let own = curve.knots().knots();
        let interior = own
            .get(degree + 1..own.len().saturating_sub(degree + 1))
            .unwrap_or_default();
        knots.extend(interior.iter().copied().map(remap));
        if leg < last {
            // The seam knot at multiplicity `p` (see the module docs on
            // why it is not dropped to `p − 1`).
            let seam_value = starts.get(leg + 1).copied().unwrap_or(total) / total;
            knots.extend(core::iter::repeat_n(seam_value, degree));
        }

        // The junction is authored once: leg `i + 1`'s first control
        // point and weight are the redundant description and are
        // dropped here.
        let skip = usize::from(leg > 0);
        control.extend(curve.control().iter().skip(skip).copied());
        weights.extend(curve.weights().iter().skip(skip).map(|w| w * lambda));
    }
    knots.extend(core::iter::repeat_n(1.0, degree + 1));

    let kv = KnotVector::clamped(knots, degree).map_err(ComposeError::Knots)?;
    NurbsCurve3::new(kv, control, weights).map_err(ComposeError::Structure)
}
