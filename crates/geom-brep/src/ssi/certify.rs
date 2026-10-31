//! **The C2 certificate for a rung-3 fitted cache** — all three limbs,
//! always (OQ2: no staging, hull bounds are an entry requirement).
//!
//! A marched-and-fitted carrier has no closed form on either side of
//! the comparison, so "certified residual ≤ ε" has to answer three
//! separate questions. This module answers all of them before a cache
//! can reach an at-rest body, and refuses typed for each separately —
//! the acceptance suite plants a corruption that trips each limb alone.
//!
//! # Limb 1 — on-locus residual (nearness)
//!
//! At the fixed [`CERT_SAMPLES`] schedule over the carrier's parameter
//! span:
//!
//! - **analytic operand**: `|f(C(t))|` in meters, the existing
//!   linearized implicit residual, through `ssi_on_locus`;
//! - **NURBS operand**: no implicit form exists, so the residual is
//!   `|C(t) − S(u*, v*)|` at a **certified foot point** from
//!   `geom::surfaces::projection` (`ssi_on_locus_foot`). Any point of
//!   the surface bounds the carrier's distance from it above, so the
//!   foot is owed no limb of its own: a foot on the far sheet reads a
//!   large distance and refuses, and no foot reads less than the true
//!   distance.
//!
//! # Limb 2 — sup-norm honesty (between the samples)
//!
//! A sampled max is not a bound, and a marched-and-fitted curve lies
//! about the locus *precisely between samples*. Two mechanisms, one per
//! operand kind, both pure convexity facts about control coefficients
//! (C9's certification arithmetic: no evaluation, no root, no transcendental):
//!
//! - **analytic**: `geom_core::spline::compose` composes the surface's
//!   polynomial implicit form with the carrier and returns a certified
//!   per-span hull of the composite. Its units are the composite's, so
//!   this module converts to meters — **exactly** for the plane
//!   (identity), the sphere and the cylinder (`÷ 2R`), and per span on
//!   the torus, whose meters residual is `F/(2r·(A + 2Rρ))` for its
//!   quartic `F = A² − 4R²ρ²` ([`ToMeters::Torus`]). The cone has no
//!   conversion and refuses.
//! - **NURBS**: `sup_t |S(P(t)) − C(t)|` bounded by the
//!   **tensor-product Bernstein composition**
//!   (`geom_core::spline::compose::tensor`, M5 PR 7b): the difference
//!   is enclosed as ONE composite whose ring coefficients are hulled
//!   per span, so the cancellation that is the whole content of
//!   `S(P(t)) = C(t)` survives into the bound. Every coefficient is a
//!   certification enclosure; nothing is sampled.
//!
//!   This replaced PR 7's per-span first-order enclosure
//!   (`rad_C + |C(m) − S(P(m))| + rad_S`), which was sound but scaled
//!   like the *span width*: the two variation radii each bounded a
//!   real motion of the curve across its span and their near-perfect
//!   cancellation was thrown away by enclosing them separately — on
//!   the M5 wall fixture it reported ~1e-2 m where the true residual
//!   is ~1e-10 m, and reaching ε would have needed tens of thousands
//!   of spans. The composite tracks the residual's own scale (the
//!   tensor module's rustdoc carries the derivation note for why
//!   composition-then-hull keeps what hull-then-difference loses),
//!   which is what retired the plane×NURBS arm — see the C5 table's
//!   `(Plane, Nurbs)` note for the retirement record.
//!
//! Either composite is **subdivided until it decides** ([`subdivide`]);
//! the rule is C2's paragraph in `crates/geom-brep/README.md`.
//!
//! # Limb 3 — the uniqueness tube (branch selection, made real)
//!
//! D2's "the branch of S₁∩S₂ through the witness" is only
//! checkable if branch-uniqueness near the cache is **proved**. Over a
//! chain of boxes covering the carrier with certified radius ρ — in the
//! chart form, the pcurve's per-span windows padded along each axis by
//! ρ over that axis's chart speed, the pad [`SsiTube::Chart`] records —
//! this module proves what the enclosures actually support — stated exactly,
//! because a slightly-too-strong claim here is the one that would
//! matter:
//!
//! > On each box `B`, enclose `(∇f₁ × ∇f₂)·e` (analytic pair) or the
//! > chart form `∇φ·e⊥ / ‖chart stretch‖` (plane×NURBS). Suppose the
//! > enclosure excludes zero. The enclosure is valid at **every** point
//! > of `B`, so on any slice `{e·x = c} ∩ B` the two constraint
//! > gradients restricted to that slice stay linearly independent
//! > throughout. Take two solutions in one slice: the mean value
//! > theorem applied along the segment joining them (which lies in `B`,
//! > a convex box) forces `f₁` and `f₂` to have a common critical
//! > direction somewhere on it — which the enclosure has just excluded.
//! > So each slice holds **at most one** solution, and the solution set
//! > in `B` is a graph over the `e` axis: no branch point, no loop, no
//! > second sheet **at the same `e`-level**.
//!
//! It is a convexity/mean-value argument over the enclosure, not a bare
//! appeal to the implicit function theorem (which is local and would
//! not, by itself, cover the whole box). A graph is not yet one arc: a
//! second arc beside the first along `e`, leaving through the box's
//! sides, passes the slice argument. The solution set in the chain is
//! one arc spanning the carrier, and limb 3 proves it at every door
//! ([`super::one_arc`]): each box holds one piece, consecutive boxes
//! share it, and it reaches both ends of the carrier. A search banks
//! every cell of a tube as accounted ([`super::exhaust`]); what lies
//! outside every tube is that pass's to exclude or refuse.
//!
//! Which rung speaks when none certifies is [`limb_three`]'s: the
//! narrowest probed, as the most specific reading of the carrier.
//!
//! An enclosure that **straddles** zero at every rung escalates, typed,
//! never retried: `ssi_tube_transversality` lands in `Sign::Zero` and
//! refuses toward C7. Two branches passing within the band of each
//! other is a genuine sliver of the operand pair, where F6's ladder says
//! escalate; the enclosure's remaining slack can also straddle, and
//! escalates the same way.
//!
//! # The witness is unchanged
//!
//! `witness = carrier(mid)` (`WitnessMidpoint`), minted by the
//! constructing op from the cache this schedule sees. S2 stays
//! discharged; nothing about the witness contract moves at rung 3.

use geom::{Curve3, NurbsCurve2, NurbsCurve3};
use geom::{NurbsSurface, Surface};
use geom_core::spline::algebra::{dehomogenize, domain_grid_points, range_grid_points};
use geom_core::spline::compose::{self, CurveCertData, ImplicitSurface, tensor};
use geom_core::spline::{KnotVector, RingRefinement};
use geom_core::{
    Band, Bounds, CertifiedBounds, CertifiedEnclosure, Decide, Decided, Indeterminate, Interval,
    Margin, Point3, Real, Sign, Vec3,
};

use crate::certify::CertCheck;
use crate::certify::{CERT_SAMPLES, sample_param};
use crate::descent::{Descent, Work};
use crate::dihedral::{decide_positive, decide_reported};

use super::enclose::{
    Box3, NurbsBoxes, UvWindow, chart_transverse_margin, graph_margin, in_meters, mag_in_meters,
    torus_a_shift, torus_meters_floor, torus_meters_sup, tube_margin, zero_free_lower_bound,
};
use super::exhaust::UvRect;
use super::one_arc::{Shortfall, dominant_axis, one_arc, one_arc_r3};
use super::refine::{RefusedResidual, RoundMargin};
use super::section::{BandVerdict, band_verdict};
use super::{SsiError, SsiOperand};

/// The **largest** tube radius tried, as a fraction of the caller's
/// named extent. The ladder halves from here.
pub const SSI_TUBE_RADIUS_MAX: f64 = 1.0 / 8.0;

/// How many halvings the ladder tries before giving up. The last rung
/// is `SSI_TUBE_RADIUS_MAX · 2^−(SSI_TUBE_RUNGS−1)` of the extent.
pub const SSI_TUBE_RUNGS: usize = 20;

/// The absolute floor on the tube radius, as a multiple of ε: below
/// this the tube could not contain the carrier's own certified
/// residual, so there is nothing left to prove.
pub const SSI_TUBE_RADIUS: f64 = 8.0;

/// The certified tube radius is **searched**, not fixed, and the reason
/// is load-bearing rather than an optimization.
///
/// A tube proves one-arc-ness over a *neighborhood*; how wide a
/// neighborhood is a property of the operand pair, not a constant. A
/// tube pinned at a few ε would be technically valid and practically
/// useless: the exhaustiveness accounting counts a cell as "accounted"
/// only when the cell lies **inside** a tube box, so an ε-wide tube
/// forces the subdivision to refine every cell along the locus down to
/// ε — a number of cells linear in `locus length / ε`, which at
/// ε = 1e-9 is not a computation, it is a hang. Searching for the
/// widest radius the graph criterion certifies makes the accounting
/// terminate at the geometry's own scale, and it reports a *stronger*
/// theorem (uniqueness over a wider region) for free.
///
/// The ladder is a fixed geometric sequence tried in a fixed order
/// (D9): no value branch chooses it, the first rung that certifies
/// wins, and if none does the operation refuses typed rather than
/// shipping a carrier whose component-selection claim is unproved.
pub(crate) fn tube_ladder(extent: f64, band: Band) -> impl Iterator<Item = f64> {
    let floor = SSI_TUBE_RADIUS * band.zero();
    (0..SSI_TUBE_RUNGS).filter_map(move |k| {
        #[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
        let r = SSI_TUBE_RADIUS_MAX * extent / (2.0f64).powi(k as i32);
        if r >= floor { Some(r) } else { None }
    })
}

/// How many spans limbs 2 and 3 cut the carrier into ([`cert_grid`]).
/// More spans ⇒ tighter hulls and a tighter tube, at linear cost. The
/// grid's points are a **structure** choice, not a decision; the
/// refinement they schedule is made inside the certificate, so it is
/// held as homogeneous enclosures in the ring (C6: [`refined_net`],
/// `CurveCertData::with_breaks`).
pub const SSI_CERT_SPANS: usize = 32;

/// The region limb 3 proved one-arc-ness over, by the kind of tube
/// that proved it.
///
/// **Ladder structure** (C6's `f64` selection lane), lifted to the
/// certificate's scalar for uniformity. No trilean reads it, but it is
/// what the exhaustiveness accounting banks: a cell inside the recorded
/// region is accounted, so the region recorded must be the one proved.
#[derive(Clone, Copy, Debug)]
pub enum SsiTube<T> {
    /// The ℝ³ arm: a box chain around the carrier padded by `radius`
    /// metres, which is the proved region.
    Spatial {
        /// The pad, in metres.
        radius: T,
    },
    /// The chart arm (plane × NURBS): a per-span window chain around
    /// the wall's pcurve, padded per axis in chart units. The rung is
    /// the ladder radius in metres that was tried, a label and not a
    /// metre bound on the region; the pads are that radius divided by
    /// each axis's chart speed, and they are the proved region.
    Chart {
        /// The ladder rung tried, in metres.
        rung: T,
        /// The pad along `u`, in chart units.
        pad_u: T,
        /// The pad along `v`, in chart units.
        pad_v: T,
    },
    /// The two-chart arm (NURBS × NURBS): a per-span chain of ℝ⁴ boxes,
    /// each the product of the two pcurves' span windows, padded per
    /// axis in each chart's units. As on the chart arm, the rung is the
    /// ladder radius tried and the pads are the proved region.
    TwoChart {
        /// The ladder rung tried, in metres.
        rung: T,
        /// The first chart's pads along `u` and `v`, in its units.
        pad_a: (T, T),
        /// The second chart's pads along `u` and `v`, in its units.
        pad_b: (T, T),
    },
}

impl<T> SsiTube<T> {
    /// The tube at another scalar, field by field.
    pub(crate) fn map<U>(self, f: impl Fn(T) -> U) -> SsiTube<U> {
        match self {
            Self::Spatial { radius } => SsiTube::Spatial { radius: f(radius) },
            Self::Chart { rung, pad_u, pad_v } => SsiTube::Chart {
                rung: f(rung),
                pad_u: f(pad_u),
                pad_v: f(pad_v),
            },
            Self::TwoChart { rung, pad_a, pad_b } => SsiTube::TwoChart {
                rung: f(rung),
                pad_a: (f(pad_a.0), f(pad_a.1)),
                pad_b: (f(pad_b.0), f(pad_b.1)),
            },
        }
    }
}

/// The three-limb certificate of a rung-3 fitted carrier. Every field
/// is a bound the corresponding limb proved, in the unit its own doc
/// names: metres, except the chart tube's pads ([`SsiTube::Chart`]).
///
/// **Generic since M6-2.** The certificate is carried at the scalar it
/// was derived at, so an interval-lane body's cache holds enclosures of
/// its own bounds rather than an `f64` shadow of somebody else's run.
/// Which fields are genuinely scalar-typed and which are lifted ring or
/// ladder structure is stated per field — the distinction is the whole
/// C6/C9 boundary and folding it away would be dishonest.
#[derive(Clone, Copy, Debug)]
pub struct SsiCertificate<T: Real> {
    /// The fixed sample count of limb 1 ([`CERT_SAMPLES`]).
    pub samples: u32,
    /// Limb 1: the largest on-locus residual over the schedule, in
    /// meters (the sampled max — it steers, it does not certify).
    /// **Scalar-typed**: evaluated at `T`, so it is an enclosure of the
    /// residual on the interval lane.
    pub on_locus_max: T,
    /// Limb 2: the certified **sup-norm** bound over the whole span, in
    /// meters. This is the number that certifies. **Certification-derived**: the
    /// certification produces an `f64` upper bound (that is what a hull bound
    /// IS), lifted here so consumers band one scalar; at the interval
    /// scalar it is a thin enclosure of that bound, and the widening of
    /// a lifted operand has already been paid inside interval arithmetic.
    pub hull_sup: T,
    /// Limb 3: the region the tube proved, by kind ([`SsiTube`]).
    pub tube: SsiTube<T>,
    /// Limb 3: the smallest certified transversality margin over the
    /// box chain, in meters — the headroom of the one-arc proof. The
    /// ring's zero-free lower bound times the caller's feature extent,
    /// so it carries the extent's scalar.
    pub tube_transversality: T,
    /// Limb 3: how many boxes the chain has.
    pub tube_boxes: u32,
}

/// Which limb a certificate refusal came from — so a consumer (and the
/// acceptance suite) can tell them apart.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SsiLimb {
    /// Limb 1 — on-locus residual.
    OnLocus,
    /// Limb 2 against an analytic operand — the carrier's certified
    /// distance bound from the surface, the carrier read alone.
    HullSup,
    /// Limb 2 against a NURBS operand — the certified bound on
    /// `|S(P(t)) − C(t)|`, the carrier read through its pcurve: a bound
    /// on the carrier's miss and on the pcurve's together.
    HullSupChart,
    /// Limb 2 against an analytic operand — the composite's certified
    /// value at one of its breaks, past the band's zero: limb 1's
    /// linearized residual, measured where the bound did not clear.
    HullValue,
    /// Limb 2 against a NURBS operand — `|S(P(t)) − C(t)|` at one of the
    /// composite's breaks, past the band's zero: the carrier against the
    /// surface point its pcurve names, which bounds its miss from the
    /// surface from above, so the bound over a span holding the break
    /// cannot clear.
    HullValueChart,
    /// [`HullValueChart`](Self::HullValueChart) at a break where the
    /// carrier's own distance from the surface, read through its foot
    /// as limb 1 reads it, decides Zero: the carrier is on the surface
    /// there, so the value past the band is the pcurve's own miss of
    /// the carrier's foot path.
    ImageMiss,
    /// Limb 3 — the uniqueness tube.
    Tube,
}

impl SsiLimb {
    /// The limb's display name.
    pub fn name(self) -> &'static str {
        match self {
            Self::OnLocus => "limb 1 (on-locus residual)",
            Self::HullSup => "limb 2 (control-hull sup-norm bound)",
            Self::HullSupChart => "limb 2 (control-hull sup-norm bound through its pcurve)",
            Self::HullValue => "limb 2 (the carrier's residual at a break of its composite)",
            Self::HullValueChart => {
                "limb 2 (the carrier against its pcurve's surface point at a break of its \
                 composite)"
            }
            Self::ImageMiss => {
                "limb 2 (the pcurve's surface point against a carrier on the surface, at a \
                 break of its composite)"
            }
            Self::Tube => "limb 3 (uniqueness tube)",
        }
    }

    /// The certification check a refusal of this limb is a refused arm
    /// of where the limb judges a carrier against its surfaces, with
    /// its image a witness: the SSI door, and the edge certifier's
    /// lanes that run the limbs (their ending is
    /// [`crate::certify::recourse`]'s). What a limb's value measures is
    /// the door's to say: where the image under judgement is a STORED
    /// one, the value at a break is that image's own miss
    /// (`PcurveCheck::of_fitted_limb`). Limbs 1 and 2 are the fitted
    /// carrier's on-locus residual and sup-norm bound; limb 2's value at
    /// a break is limb 1's residual against an analytic operand, and
    /// against a NURBS operand the sup-norm bound's quantity at a point,
    /// a bound on the miss like it; limb 3's margin is a lower bound on
    /// the operands' transversality over the box chain.
    #[must_use]
    pub fn check(self) -> CertCheck {
        match self {
            Self::OnLocus => CertCheck::PlaneNurbsOnLocus,
            Self::HullSup | Self::HullSupChart => CertCheck::PlaneNurbsHull,
            Self::HullValue => CertCheck::AnalyticBreakResidual,
            Self::HullValueChart | Self::ImageMiss => CertCheck::PlaneNurbsHullValue,
            Self::Tube => CertCheck::TransversalityTube,
        }
    }
}

/// The breaks limbs 2 and 3 cut the carrier at so they have small
/// spans to work with: the domain's `SSI_CERT_SPANS` grid. Both read
/// the DESCRIBED carrier with these breaks taken in the certification
/// ring (limb 2 through [`CurveCertData::with_breaks`], the box chain
/// through [`refined_net`]), never a refined curve with `f64` weights.
fn cert_grid(kv: &KnotVector) -> Vec<f64> {
    // Already fine enough: a carrier that the marcher's step rule
    // already gave hundreds of spans gains nothing from more breaks.
    if kv.control_count() >= SSI_CERT_SPANS + kv.degree() {
        return Vec::new();
    }
    // A grid point on or near a knot is skipped: on it, refinement
    // would raise multiplicity; beside it, it would open a narrow span.
    domain_grid_points(kv, SSI_CERT_SPANS)
}

/// A carrier's control net refined at [`cert_grid`]: its homogeneous
/// channels `(w·P, w)` through [`RingRefinement::refine`], then
/// de-homogenized ([`dehomogenize`]), so every point encloses the
/// refined net of the described carrier. Paired with the refined
/// vector; `None` when the insertion chain refuses, `coords` is not
/// three channels, or a refined weight is not certainly positive. A
/// channel that is not one entry per weight comes back refused entry by
/// entry ([`RingRefinement::refine`]'s extent guard), which every hull
/// read off it refuses.
fn refined_net(
    kv: &KnotVector,
    weights: &[f64],
    coords: &[Vec<Interval>],
) -> Option<(KnotVector, [Vec<Interval>; 3])> {
    let refinement = RingRefinement::new(kv, &cert_grid(kv)).ok()?;
    let [cx, cy, cz] = coords else {
        return None;
    };
    let w: Vec<Interval> = weights
        .iter()
        .map(|w| Interval::from_bounds(*w, *w))
        .collect();
    let rw = refinement.refine(&w);
    let mut net = [cx, cy, cz].into_iter().map(|c| {
        let homogeneous: Vec<Interval> = c.iter().zip(&w).map(|(x, w)| *x * *w).collect();
        let refined = refinement.refine(&homogeneous);
        dehomogenize(&refined, &rw)
    });
    let net = [net.next()??, net.next()??, net.next()??];
    Some((refinement.knots().clone(), net))
}

/// The exact `f64` a structural surface parameter stands for, or `None`
/// when the scalar's bracket is not a point.
///
/// **Why this gate exists (M6-2).** `geom_core`'s [`ImplicitSurface`]
/// is `f64` STRUCTURE — a polynomial form with `f64` coefficients — and
/// the composite it drives is only a bound on the surface it actually
/// describes. Lifting `certify_branch` off `f64` therefore does NOT
/// make the analytic composite accept a *widened* operand: a cylinder
/// whose radius is an enclosure is a FAMILY of cylinders, and picking
/// any representative would certify the carrier against a surface the
/// body does not have. A thin bracket is exactly the case where the
/// representative is the surface, so that case is admitted and every
/// other one refuses typed (`UnsupportedCertificate`, below) — never a
/// midpoint, never a guess. Retiring the gate means giving the
/// composite enclosure-valued coefficients, which is a `geom_core`
/// change with its own unit.
fn exact<T: Bounds>(v: T) -> Option<f64> {
    let (lo, hi) = (v.lo(), v.hi());
    (lo == hi && !lo.is_nan()).then_some(lo)
}

/// Three coordinates of an exact point, or `None` if any is widened.
fn exact3<T: Bounds>(p: [T; 3]) -> Option<[f64; 3]> {
    Some([exact(p[0])?, exact(p[1])?, exact(p[2])?])
}

/// The `compose` implicit form of an analytic surface, and how its
/// composite crosses to meters.
///
/// `Err` names the reason, which the caller turns into an
/// [`SsiError::UnsupportedCertificate`] verbatim: the cone (no meters
/// conversion), NURBS (no implicit form), and — since M6-2 — an operand
/// whose structural parameters are not exact at the caller's scalar
/// (see [`exact`]).
fn composite_form<T: Bounds>(s: &Surface<T>) -> Result<(ImplicitSurface, ToMeters), &'static str> {
    const WIDENED: &str = "the analytic operand's structural parameters are not exact at this \
                           scalar — the ring composite's implicit form is f64 structure, and a \
                           widened operand is a family of surfaces, not the body's surface \
                           — refused rather than represented by a midpoint";
    match *s {
        Surface::Plane { origin, normal, .. } => {
            let (Some(point), Some(normal)) =
                (exact3(origin.to_array()), exact3(normal.to_array()))
            else {
                return Err(WIDENED);
            };
            Ok((
                ImplicitSurface::Plane { point, normal },
                // n·(P − p₀) is already meters for a unit normal.
                ToMeters::Divisor(1.0),
            ))
        }
        Surface::Sphere { center, radius, .. } => {
            let (Some(center), Some(radius)) = (exact3(center.to_array()), exact(radius)) else {
                return Err(WIDENED);
            };
            Ok((
                ImplicitSurface::Sphere { center, radius },
                // |P−c|² − R² = 2R · the linearized meters residual,
                // exactly.
                ToMeters::Divisor(2.0 * radius),
            ))
        }
        Surface::Cylinder {
            origin,
            axis,
            radius,
            ..
        } => {
            let (Some(point), Some(axis), Some(radius)) = (
                exact3(origin.to_array()),
                exact3(axis.to_array()),
                exact(radius),
            ) else {
                return Err(WIDENED);
            };
            Ok((
                ImplicitSurface::Cylinder {
                    point,
                    axis,
                    radius,
                },
                // |w|² − R² = 2R · the linearized meters residual,
                // exactly.
                ToMeters::Divisor(2.0 * radius),
            ))
        }
        Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            ..
        } => {
            let (Some(center), Some(axis), Some(major), Some(minor)) = (
                exact3(center.to_array()),
                exact3(axis.to_array()),
                exact(major_radius),
                exact(minor_radius),
            ) else {
                return Err(WIDENED);
            };
            Ok((
                ImplicitSurface::Torus {
                    center,
                    axis,
                    major_radius: major,
                    minor_radius: minor,
                },
                ToMeters::Torus {
                    center,
                    major,
                    minor,
                },
            ))
        }
        Surface::Cone { .. } | Surface::Nurbs(_) | Surface::Approx(_) => Err(
            "no meters composite for this surface kind (the cone's composite has no \
             meters conversion; a spline stand-in and its offset description have no \
             implicit form to build one from)",
        ),
    }
}

/// How an implicit composite's units cross to meters.
#[derive(Clone, Copy, Debug)]
enum ToMeters {
    /// One outward division by an exact divisor.
    Divisor(f64),
    /// The torus, per span of the carrier `C`. Its meters residual is
    /// `m = F/(2r·(A + 2Rρ))` with `A = |q|² + R² − r²` and `ρ` the
    /// distance from the axis, so a span's bound reads `F∘C` against
    /// `A∘C`, `A`'s own composite over the same spans
    /// (`enclose::torus_meters_sup`), and a break's floor uses
    /// `2Rρ ≤ R² + ρ² ≤ A + r²` (`enclose::torus_meters_floor`).
    Torus {
        center: [f64; 3],
        major: f64,
        minor: f64,
    },
}

/// A limb-2 composite's readings in meters: its sup, each span's sup,
/// and each break's floor.
fn meters_readings(
    composite: &compose::CompositeForm,
    data: &CurveCertData<'_>,
    to: ToMeters,
) -> Result<(f64, Vec<f64>, Vec<f64>), SsiError> {
    let (center, major, minor) = match to {
        ToMeters::Torus {
            center,
            major,
            minor,
        } => (center, major, minor),
        ToMeters::Divisor(per) => {
            let spans = composite
                .span_bounds()
                .into_iter()
                .map(|b| mag_in_meters(b, per))
                .collect();
            let floors = composite
                .break_values()
                .into_iter()
                .map(|v| zero_free_lower_bound(in_meters(v, per)))
                .collect();
            return Ok((mag_in_meters(composite.bound(), per), spans, floors));
        }
    };
    let a = compose::squared_distance_composite(data, &center, torus_a_shift(major, minor))
        .map_err(|_| SsiError::UnsupportedCertificate {
            what: "the torus's A composite refused the fitted carrier",
        })?;
    let (f_spans, a_spans) = (composite.span_bounds(), a.span_bounds());
    let (f_breaks, a_breaks) = (composite.break_values(), a.break_values());
    if f_spans.len() != a_spans.len() || f_breaks.len() != a_breaks.len() {
        return Err(SsiError::UnsupportedCertificate {
            what: "the torus's A composite does not share its quartic's spans",
        });
    }
    let spans: Vec<f64> = f_spans
        .into_iter()
        .zip(a_spans)
        .map(|(f, a)| torus_meters_sup(f, a, major, minor))
        .collect();
    let sup = spans
        .iter()
        .try_fold(0.0f64, |acc, &x| (!x.is_nan()).then(|| acc.max(x)))
        .unwrap_or(f64::NAN);
    let floors = f_breaks
        .into_iter()
        .zip(a_breaks)
        .map(|(f, a)| torus_meters_floor(f, a, minor))
        .collect();
    Ok((sup, spans, floors))
}

/// Limb 1 + limb 2 against one **analytic** operand. The carrier
/// enters limb 2 through the carrier-enclosure door
/// ([`crate::carrier_net::carrier_net`]) over its span
/// ([`crate::carrier_net::carrier_span`]).
fn analytic_limbs<T: Decide + Bounds + CertifiedEnclosure>(
    carrier: &Curve3<T>,
    params: (T, T),
    surface: &Surface<T>,
    band: Band,
    at: &mut Refused,
) -> Result<(T, T), SsiError> {
    let ((t0, t1), (h0, h1)) = crate::carrier_net::carrier_span(carrier, params);
    // Limb 2's hull: the implicit form composed with the described
    // carrier cut in the ring at its grid and at `cuts`, in metres.
    let grid = match carrier {
        Curve3::Nurbs(net) => cert_grid(net.knots_f64_interim()),
        _ => range_grid_points(t0, t1, SSI_CERT_SPANS, &[]),
    };
    let hull = |cuts: &[f64]| -> Result<Hull, SsiError> {
        let (form, to_meters) =
            composite_form(surface).map_err(|what| SsiError::UnsupportedCertificate { what })?;
        let mut breaks: Vec<f64> = grid.iter().chain(cuts).copied().collect();
        breaks.sort_by(f64::total_cmp);
        breaks.dedup();
        let net = crate::carrier_net::carrier_net(carrier, (t0, t1), &breaks)
            .map_err(SsiError::CarrierEnclosure)?;
        let (ck, cw, cc) = net.operand();
        let data = CurveCertData::new(ck, cw, cc)
            .map_err(|_| SsiError::UnsupportedCertificate {
                what: "the fitted carrier's enclosure data is malformed",
            })?
            .with_breaks(net.cuts());
        let composite = compose::implicit_composite(&data, &form).map_err(|_| {
            SsiError::UnsupportedCertificate {
                what: "the implicit composite refused the fitted carrier",
            }
        })?;
        let (sup, spans, floors) = meters_readings(&composite, &data, to_meters)?;
        Ok(Hull {
            sup,
            breaks: composite.num.breaks().to_vec(),
            work: spans.len(),
            spans,
            floors,
            data: (),
        })
    };
    // ---- limb 1: the fixed schedule ----
    // The schedule on the carrier's held span (its ends held, its
    // interior stations chosen points); the value channel names a
    // refused sample.
    let mut worst = T::zero();
    for i in 0..CERT_SAMPLES {
        let t = sample_param(t0, t1, i);
        let station = crate::certify::held_station((t0, t1), (h0, h1), i, CERT_SAMPLES);
        let r = crate::implicit::implicit_residual(surface, carrier.eval(station)).abs();
        // `max`, not a `>` branch: the running worst is a scalar-typed
        // quantity now, and generic evaluation code does not compare.
        worst = worst.max(r);
        let decided = decide_reported("ssi_on_locus", Margin::of(r), band);
        if locatable(&decided) {
            at.spans.push(RefusedSpan { lo: t, hi: t });
            if let Ok(hull) = hull(&[]) {
                hull.uncleared(band, &mut at.spans);
            }
        }
        limb_verdict(
            SsiLimb::OnLocus,
            decided,
            || RefusedResidual::Over(r.hi()),
            at,
        )?;
    }

    // ---- limb 2: the certified hull bound, subdivided ----
    let sup = hull_limb(&ANALYTIC_HULL, subdivide(hull, band)?, band, at)?;
    Ok((worst, sup))
}

/// Limb 1 + limb 2 against a **NURBS** operand, using the traced
/// pcurve as the parameter map (module docs).
fn nurbs_limbs<T: Decide + Bounds + CertifiedEnclosure>(
    carrier: &Curve3<T>,
    params: (T, T),
    pcurve: &NurbsCurve2<T>,
    surface: &NurbsSurface<T>,
    band: Band,
    at: &mut Refused,
) -> Result<(T, T), SsiError> {
    // Limb 2's hull: `S(P(t)) − C(t)` enclosed as one composite, cut
    // at `cuts` besides the round's grid, in metres.
    let grid = chart_grid(carrier, pcurve.knots_f64_interim());
    let (pcoords, scoords) = (pcurve.certified_coords(), surface.certified_coords());
    let hull = |cuts: &[f64]| -> Result<Hull, SsiError> {
        carrier_chart_round(
            carrier,
            (
                pcurve.knots_f64_interim(),
                pcurve.weights_f64_interim(),
                &pcoords,
            ),
            (
                surface.knots_u_f64_interim(),
                surface.knots_v_f64_interim(),
                surface.weights_f64_interim(),
                &scoords,
            ),
            &grid,
            cuts,
        )
    };
    // ---- limb 1: the fixed schedule, through certified foot points --
    // The schedule on the carrier's held span (its ends held, its
    // interior stations chosen points; the pcurve shares the carrier's
    // parameter); the value channel names a refused sample.
    let ((t0, t1), (h0, h1)) = crate::carrier_net::carrier_span(carrier, params);
    let mut worst = T::zero();
    for i in 0..CERT_SAMPLES {
        let t = sample_param(t0, t1, i);
        let th = crate::certify::held_station((t0, t1), (h0, h1), i, CERT_SAMPLES);
        let c = carrier.eval(th);
        // Warm-start from the trace's own pcurve: the projection is a
        // *check*, and starting it where the trace says the foot is
        // makes a disagreement visible rather than hidden by a global
        // seeding sweep landing somewhere else. The seed is a
        // parameter — `f64` structure on both sides of the M6-2 lift.
        let p = pcurve.eval(th);
        let proj = surface
            .project_from_seed(c, p.x.lo(), p.y.lo())
            .map_err(|e| SsiError::FootPointInconclusive {
                t,
                last_distance: e.last_distance,
            })?;
        worst = worst.max(proj.distance);
        let decided = decide_reported("ssi_on_locus_foot", Margin::of(proj.distance), band);
        if locatable(&decided) {
            at.spans.push(RefusedSpan { lo: t, hi: t });
            if let Ok(hull) = hull(&[]) {
                hull.uncleared(band, &mut at.spans);
            }
        }
        limb_verdict(
            SsiLimb::OnLocus,
            decided,
            || RefusedResidual::Over(proj.distance.hi()),
            at,
        )?;
    }

    // ---- limb 2: |S(P(t)) − C(t)| as ONE composite (M5 PR 7b) ----
    // Unlike the analytic arm, the NURBS arm needs NO exactness gate:
    // every coefficient of every operand entered interval arithmetic through its
    // own bracket (`certified_coords`), so a widened control net widens the
    // composite and the bound stays honest.
    let stopped = subdivide(hull, band)?;
    // A break past the band bounds the carrier's miss through its
    // pcurve's surface point. Where the carrier's own foot there decides
    // Zero, the miss is the pcurve's ([`SsiLimb::ImageMiss`]). The tell
    // names the refusal; it is not part of what the certificate states,
    // so it is decided outside the verdict log (ledger row F21).
    let carrier_on_surface = |tb: f64| {
        let th = if tb == t0 {
            h0
        } else if tb == t1 {
            h1
        } else {
            T::from_f64(tb)
        };
        let p = pcurve.eval(th);
        surface
            .project_from_seed(carrier.eval(th), p.x.lo(), p.y.lo())
            .is_ok_and(|proj| {
                matches!(
                    geom_core::k_stats::check_unlogged(
                        "ssi_image_miss_tell",
                        proj.distance,
                        band,
                        "F21"
                    ),
                    Ok(Sign::Zero)
                )
            })
    };
    let arm = match stopped {
        Subdivided::Value { at: tb, .. } if carrier_on_surface(tb) => &CHART_IMAGE_HULL,
        _ => &CHART_HULL,
    };
    let sup = hull_limb(arm, stopped, band, at)?;
    Ok((worst, sup))
}

/// The breaks a chart round cuts `carrier` at besides its own `cuts`,
/// against a pcurve on `image`: a spline's [`chart_breaks`]; any other
/// carrier the image's interior knots and the image domain's
/// [`SSI_CERT_SPANS`] grid clear of them.
pub(crate) fn chart_grid<T: Real>(carrier: &Curve3<T>, image: &KnotVector) -> Vec<f64> {
    if let Curve3::Nurbs(net) = carrier {
        return chart_breaks(net.knots_f64_interim(), image);
    }
    let (lo, hi) = image.domain();
    let knots: Vec<f64> = image.interior_knots().map(|(k, _)| k).collect();
    let mut grid = range_grid_points(lo, hi, SSI_CERT_SPANS, &knots);
    grid.extend(knots);
    grid
}

/// One round of limb 2 against a spline chart for any carrier: its net
/// from the carrier-enclosure door ([`crate::carrier_net::carrier_net`])
/// over the pcurve's domain, cut at `grid` and `cuts`, composed with the
/// pcurve into the chart ([`chart_round`]). The one round of the SSI
/// door's NURBS operand and of the fitted row's chart.
pub(crate) fn carrier_chart_round<T: geom_core::CertifiedBounds>(
    carrier: &Curve3<T>,
    pimage: CurveOperand<'_>,
    schart: ChartOperand<'_>,
    grid: &[f64],
    cuts: &[f64],
) -> Result<Hull, SsiError> {
    let breaks: Vec<f64> = grid.iter().chain(cuts).copied().collect();
    let net = crate::carrier_net::carrier_net(carrier, pimage.0.domain(), &breaks)
        .map_err(SsiError::CarrierEnclosure)?;
    chart_round(net.operand(), pimage, schart, net.cuts())
}

/// A curve operand of limb 2's composite as certification data: its
/// knots, weights and certified control coordinates.
pub(crate) type CurveOperand<'a> = (&'a KnotVector, &'a [f64], &'a [Vec<Interval>]);

/// A spline chart operand of limb 2's composite as certification data:
/// its two knot vectors, weights and certified control coordinates.
pub(crate) type ChartOperand<'a> = (
    &'a KnotVector,
    &'a KnotVector,
    &'a [f64],
    &'a [Vec<Interval>],
);

/// One round of limb 2 against a spline chart: the carrier and its
/// pcurve composed into `chart`, cut at `breaks` besides both curves'
/// knots ([`chart_composite`]). The one home of the round for every
/// caller — the SSI door's NURBS operand and the fitted row's chart —
/// each refusal named for its operand.
pub(crate) fn chart_round(
    (ck, cw, cc): CurveOperand<'_>,
    (pk, pw, pc): CurveOperand<'_>,
    (su, sv, sw, sc): ChartOperand<'_>,
    breaks: &[f64],
) -> Result<Hull, SsiError> {
    let malformed = |what| move |_| SsiError::UnsupportedCertificate { what };
    let cdata = CurveCertData::new(ck, cw, cc).map_err(malformed(
        "the fitted carrier's enclosure data is malformed",
    ))?;
    let pdata = CurveCertData::new(pk, pw, pc)
        .map_err(malformed("the traced pcurve's enclosure data is malformed"))?;
    let sdata = tensor::SurfaceCertData::new(su, sv, sw, sc)
        .map_err(malformed("the NURBS operand's enclosure data is malformed"))?;
    chart_composite(&cdata, &pdata, &sdata, breaks)
}

/// Limb 2's composite of one round: `S(P(t)) − C(t)` against the
/// spline chart `sdata`, enclosed as one tensor composite cut at
/// `extra` besides both curves' knots.
///
/// The tensor-product Bernstein composition encloses the difference at
/// the coefficient level, so the cancellation that IS the content of
/// `S(P(t)) = C(t)` survives into the bound (a first-order enclosure
/// adds the two variation radii and scales with the span width, ~1e-2 m
/// where the truth is ~1e-10 m). Data in, bounds out: nothing here
/// samples anything (C2.2). Both curves are decomposed onto the merged
/// break list by exact knot insertion, so a carrier and a pcurve on
/// different knot vectors are aligned structurally.
fn chart_composite(
    cdata: &CurveCertData<'_>,
    pdata: &CurveCertData<'_>,
    sdata: &tensor::SurfaceCertData<'_>,
    extra: &[f64],
) -> Result<Hull, SsiError> {
    let residual = tensor::surface_curve_residual(sdata, pdata, cdata, extra).map_err(|_| {
        SsiError::UnsupportedCertificate {
            what: "the tensor composite refused the carrier/pcurve pair (mismatched channel \
                   counts or knot domains — the shared-parameter identity is the entry \
                   requirement)",
        }
    })?;
    Ok(Hull {
        sup: residual.sup_bound(),
        breaks: residual.breaks().to_vec(),
        spans: residual.span_bounds().to_vec(),
        floors: residual
            .break_values()
            .iter()
            .map(|&v| zero_free_lower_bound(v))
            .collect(),
        work: residual.span_bounds().len(),
        data: (),
    })
}

/// A limb's verdict on its residual: `Ok` where it is zero to tolerance
/// (the `dihedral_wedge` convention), and otherwise the limb's refusal,
/// recorded in `at` as one [`LimbRefusal`] — `definite`, read off the
/// residual's enclosure, as its residual on a definite refusal. A
/// margin that is no number records nothing: no density of samples
/// answers it.
fn limb_verdict(
    limb: SsiLimb,
    decided: Result<Decided, Indeterminate>,
    definite: impl FnOnce() -> RefusedResidual,
    at: &mut Refused,
) -> Result<(), SsiError> {
    match decided {
        Ok(Decided {
            sign: Sign::Zero, ..
        }) => Ok(()),
        Ok(Decided { margin, .. }) => {
            at.refusal = Some(LimbRefusal {
                limb,
                margin,
                residual: definite(),
            });
            Err(SsiError::CertificateLimb { limb, margin })
        }
        Err(cause) => {
            if !cause.margin.is_invalid() {
                at.refusal = Some(LimbRefusal {
                    limb,
                    margin: cause.margin,
                    residual: RefusedResidual::InBand,
                });
            }
            Err(SsiError::CertificateEscalated { limb, cause })
        }
    }
}

/// Whether a limb's verdict is a refusal refinement can locate: a
/// definite one, or one undecided on a margin that is a number. A
/// margin that is no number is no residual a denser carrier answers, so
/// that refusal stands at once.
fn locatable(decided: &Result<Decided, geom_core::Indeterminate>) -> bool {
    match decided {
        Ok(Decided { sign, .. }) => *sign != Sign::Zero,
        Err(cause) => !cause.margin.is_invalid(),
    }
}

/// A limb's composite, span by span, as a round of [`subdivide`] reads
/// it: a certified upper bound in metres on the carrier's residual over
/// each span, span `j` covering `[breaks[j], breaks[j+1]]`, and a
/// certified lower bound in metres on its magnitude at each break.
/// `data` is what the limb decides on once the rounds stop.
pub(crate) struct Hull<P = ()> {
    /// The bound over the whole carrier, the composite's own.
    pub(crate) sup: f64,
    pub(crate) breaks: Vec<f64>,
    pub(crate) spans: Vec<f64>,
    /// The residual's magnitude at `breaks[j]` is at least `floors[j]`,
    /// in metres (`0` where nothing is certified).
    pub(crate) floors: Vec<f64>,
    /// How many spans the round's composite recomputed, which
    /// [`SSI_HULL_WORK`] counts: the spans above, or more where the
    /// composite runs past the carrier they cover.
    pub(crate) work: usize,
    pub(crate) data: P,
}

/// Whether a certified upper bound clears the band's zero: a selection
/// on the bound's own bracket (C6's f64 lane), never the limb's decision.
/// A refused (`NaN`) bound does not clear.
pub(crate) fn clears(bound: f64, band: Band) -> bool {
    matches!(
        bound.partial_cmp(&band.zero()),
        Some(core::cmp::Ordering::Less | core::cmp::Ordering::Equal)
    )
}

/// A span of limb 2's composite a round halves: its ends and its bound.
#[derive(Clone, Copy)]
struct Halved {
    lo: f64,
    hi: f64,
    bound: f64,
}

impl<P> Hull<P> {
    /// Pushes onto `at` every span whose bound does not clear the band's
    /// zero. A selection of where to look, never a decision: the refusal
    /// it locates is decided by the limb, and a refused (`NaN`) span is
    /// selected.
    fn uncleared(&self, band: Band, at: &mut Vec<RefusedSpan>) {
        for (w, bound) in self.breaks.windows(2).zip(&self.spans) {
            if !clears(*bound, band) {
                at.push(RefusedSpan { lo: w[0], hi: w[1] });
            }
        }
    }

    /// The spans a round may halve, worst bound first: every span whose
    /// bound does not clear the band's zero and has a midpoint strictly
    /// inside it. A refused (`NaN`) span is not among them: interval
    /// arithmetic refused its piece, and a piece of it is refused alike.
    fn to_halve(&self, band: Band) -> Vec<Halved> {
        let mut spans: Vec<Halved> = self
            .breaks
            .windows(2)
            .zip(&self.spans)
            .map(|(w, &bound)| Halved {
                lo: w[0],
                hi: w[1],
                bound,
            })
            .filter(|s| !s.bound.is_nan() && !clears(s.bound, band))
            .filter(|s| {
                let m = s.mid();
                s.lo < m && m < s.hi
            })
            .collect();
        spans.sort_by(|a, b| b.bound.total_cmp(&a.bound).then(a.lo.total_cmp(&b.lo)));
        spans
    }

    /// The break whose certified floor is largest, with that floor,
    /// where it lies past the band's zero: no bound over a span holding
    /// the break can clear.
    fn past(&self, band: Band) -> Option<(f64, f64)> {
        self.breaks
            .iter()
            .zip(&self.floors)
            .map(|(&t, &floor)| (t, floor))
            .filter(|&(_, floor)| !clears(floor, band))
            .max_by(|a, b| a.1.total_cmp(&b.1))
    }
}

impl Halved {
    /// Where the span is cut.
    fn mid(self) -> f64 {
        0.5 * (self.lo + self.hi)
    }
}

/// How many spans limb 2's rounds may recompute in all before it
/// refuses on the bound: each round rebuilds the whole composite, so a
/// round costs its composite's span count, and this is the sum over the
/// rounds after the first hull. A Bernstein hull's excess over the
/// function falls as the square of the span, so each round cuts it
/// about fourfold where the residual is smooth. A **structure** choice
/// (C6's f64 lane), not a decision: what it buys is which refusals end
/// on the bound.
///
/// Measured under an unbounded budget over the ci profiles of
/// geom-brep, topo, step-import and sweep (820 limb-2 calls, 797 cleared
/// on the first hull, 15 refused on a break's value there): every call
/// that decided by subdividing spent at most 826 spans. The adversarial
/// wall of `m` C0 spans under each composite span (`ssi_limb3_one_arc`)
/// clears at 268 (414 at its steepest P/N) for m = 64, 1273 (2018) for
/// m = 256 and 5908 (11 374) for m = 1024 at the default ε, where the
/// pcurve's values at the breaks fall on the wall's knots but for a few,
/// and at 628 (1314), 2740 (5538) and 11 188 (22 434) at ε 1e-6, where
/// the foot points' offset carries every one of them off its knot. These
/// figures were measured on 2026-10-12 with an uncommitted probe under an
/// unbounded budget; from outside, only that file's m = 64, m = 1024 and
/// m = 2048 rows guard them. This budget is three times the m = 64 rows'
/// steepest figure (1314) and five times the corpus's (826). A carrier
/// whose first composite has more spans than this is read on that hull
/// alone; the one such carrier measured, a dome arc at ε 1e-14 over
/// 4242 to 14 814 spans, reads its lowest bound there.
///
/// The analytic rung-3 limb counts every part of its lane's composite,
/// which covers the carrier's whole net whatever the edge's interval
/// reads, so a long carrier shared by many edges spends nearly the whole
/// budget on its first hull and gets few rounds or none. Measured once,
/// on 2026-10-11, with a temporary probe in `net_offset_sup` over the
/// same four ci profiles: 94 calls, all deciding on their first hull (89
/// cleared, 5 on a break's value), the largest costing 4080 parts.
pub const SSI_HULL_WORK: usize = 4096;

/// The most breaks limb 2's rounds add to its composite in all. A
/// round that would pass it halves its worst spans up to it. Under the
/// unbounded budget of [`SSI_HULL_WORK`]'s measurement the corpus's
/// calls that decided by subdividing took at most 348 cuts, and the
/// adversarial wall clears at 38 (42) cuts for m = 64, 254 (324) for
/// m = 256 and 1432 (2444) for m = 1024 at the default ε, and at 218
/// (342), 986 (1494) and 4058 (6102) at ε 1e-6 (the same probe as
/// [`SSI_HULL_WORK`]'s figures): this cap clears every m = 64 row and
/// spends itself on m = 1024, and on the steepest m = 256 rows at 1e-6.
pub const SSI_HULL_CUTS: usize = 1024;

/// Where a hull limb's subdivision stopped.
pub(crate) enum Subdivided<P = ()> {
    /// On the bound over the whole carrier: it cleared the band's zero,
    /// or the budget ran out or the bound stopped falling, first. The
    /// hull is the round's with the lowest bound.
    Bound(Hull<P>),
    /// On a break whose value is certified past the band's zero.
    Value {
        hull: Hull<P>,
        /// The break.
        at: f64,
        /// The certified lower bound on the value there, in metres.
        floor: f64,
    },
}

/// **Limb 2, subdivided until it decides** (C2.2), for every door that
/// states it: the search's two arms here, and the analytic rung-3
/// certificate's distance bound
/// ([`crate::pcurve_cache::limb2_offset_sup`]). `hull` builds a
/// round's hull with its argument's breaks added. The composite's
/// uncleared spans are halved, worst bound first, round by round, until
/// its bound clears the band's zero, or a break's certified value lies
/// past it, or it stops on the crate's stop rule ([`crate::descent`]):
/// its rounds stall against the lowest bound over the whole carrier, or
/// the next round would pass [`SSI_HULL_WORK`] (the cumulative budget),
/// or no span is left to halve under [`SSI_HULL_CUTS`] (the cap); or
/// the bound over the whole carrier is refused. An idle round takes no
/// action of its own here: the next round halves the worst spans as any
/// round does. Every round's bound is certified, so the limb is read on
/// the lowest. That is a bound which stopped falling, not a floor.
/// Every round's hull is the composite's own, cut from each Bézier segment's row
/// ([`CurveCertData::with_breaks`], the surface residual's extra
/// breaks), so each piece's coefficients enclose that piece and no cut
/// compounds another's width.
///
/// Each round recomputes the whole composite, the cleared spans too:
/// the implicit composite is channel algebra over every span of its
/// rows (`compose`'s `ch_*` steps), so a span-restricted round would
/// thread a span mask through that algebra, and the tensor residual,
/// whose spans are composed one by one, would need a public door of its
/// own that only this caller reads. A round costs its hull's
/// [`Hull::work`].
///
/// Three outcomes follow from that and are the code's, not accidents:
/// - a first hull whose composite already costs more than
///   [`SSI_HULL_WORK`] (its work plus the round's cuts) gets no round at
///   all, so a refusal on its bound is a refusal on an unsubdivided
///   carrier;
/// - a later round whose bound is NaN stops the subdivision on the
///   lowest certified bound before it, so the limb refuses on that
///   bound and does not escalate (only a NaN first hull reaches the
///   caller as NaN);
/// - a round whose bound equals the lowest counts as idle, and the
///   older hull is kept;
/// - a `+∞` bound is not idle while no round's bound is finite. It is
///   reachable where the composite's arithmetic overflows: interval
///   enclosures may be unbounded, and `mag_in_meters` and `sup_bound`
///   then answer `+∞`. A run of such hulls halves every span each round
///   until a work bound stops it: from one span, 12 hulls and 3072 spans
///   recomputed, ended by [`SSI_HULL_CUTS`]. That is what the rows
///   spending this budget cost, 0.34 to 1.22 s each in release
///   (`ssi_limb3_one_arc`'s grid at `m = 2048`, 2026-10-12).
pub(crate) fn subdivide<P, E>(
    hull: impl Fn(&[f64]) -> Result<Hull<P>, E>,
    band: Band,
) -> Result<Subdivided<P>, E> {
    subdivide_until(hull, band, |_| false)
}

/// [`subdivide`] with one stop more: a round whose hull `floored` calls
/// at its floor — a bound no halving can take lower, read off the
/// round's own data — stops on the lowest hull as the other stops do,
/// so rounds that cannot clear are not spent. It is asked after the
/// band and the break's value, so a hull that clears or shows a miss
/// still says so.
pub(crate) fn subdivide_until<P, E>(
    hull: impl Fn(&[f64]) -> Result<Hull<P>, E>,
    band: Band,
    floored: impl Fn(&Hull<P>) -> bool,
) -> Result<Subdivided<P>, E> {
    let mut cuts: Vec<f64> = Vec::new();
    let mut descent: Option<Descent<Hull<P>>> = None;
    let mut work = Work::new(SSI_HULL_WORK);
    loop {
        let h = hull(&cuts)?;
        if clears(h.sup, band) {
            return Ok(Subdivided::Bound(h));
        }
        // Against a NURBS operand the value is the bound's own quantity
        // at a point, a bound on the miss and not the miss: past the
        // band it still proves no bound over a span holding the break
        // can clear, so the limb stops here as on the analytic arm, and
        // only its ending differs ([`SsiLimb::HullValueChart`]).
        if let Some((at, floor)) = h.past(band) {
            return Ok(Subdivided::Value { hull: h, at, floor });
        }
        let mut next = h.to_halve(band);
        // Nothing left to halve, or a refused (NaN) bound, which steers
        // nothing: the loop's own exhaustion. A NaN never lowers the
        // lowest bound, so a later round's NaN stops on the earlier
        // certified hull. A bound at its floor is exhausted the same
        // way: no round would take it lower.
        let exhausted = next.is_empty() || h.sup.is_nan() || floored(&h);
        next.truncate(SSI_HULL_CUTS - cuts.len());
        // On the first hull nothing is spent, so a composite over the
        // budget stops here unsubdivided.
        let cost = h.work + next.len();
        let wall = (next.is_empty() || !work.admits(cost)).then_some(());
        let rounds = Descent::after(descent.take(), h.sup, h);
        // Every stop reads the limb on the lowest hull, so which stop
        // spoke is not read here.
        if rounds.stop(exhausted, wall).is_some() {
            return Ok(Subdivided::Bound(rounds.into_lowest().1));
        }
        descent = Some(rounds);
        cuts.extend(next.iter().map(|s| s.mid()));
        cuts.sort_by(f64::total_cmp);
        work.spend(cost);
    }
}

/// One arm of limb 2: the predicates it decides its bound and its value
/// at, and the limb a refusal on the value names.
pub(crate) struct HullArm {
    /// The bound's predicate.
    sup: &'static str,
    /// The limb a refusal on the bound names.
    sup_limb: SsiLimb,
    /// The value's predicate.
    value: &'static str,
    /// The limb a refusal on the value names.
    value_limb: SsiLimb,
}

/// Limb 2 against an analytic operand: its value at a break is limb 1's
/// residual.
const ANALYTIC_HULL: HullArm = HullArm {
    sup: "ssi_hull_sup",
    sup_limb: SsiLimb::HullSup,
    value: "ssi_hull_value",
    value_limb: SsiLimb::HullValue,
};

/// Limb 2 against a NURBS operand: its value at a break is the bound's
/// quantity at a point.
pub(crate) const CHART_HULL: HullArm = HullArm {
    sup: "ssi_hull_sup_chart",
    sup_limb: SsiLimb::HullSupChart,
    value: "ssi_hull_value_chart",
    value_limb: SsiLimb::HullValueChart,
};

/// [`CHART_HULL`] at a break where the carrier is on the surface: a
/// refusal on the value there names the pcurve's own miss.
const CHART_IMAGE_HULL: HullArm = HullArm {
    value_limb: SsiLimb::ImageMiss,
    ..CHART_HULL
};

/// Limb 2's verdict on where [`subdivide`] stopped, and its bound: the
/// bound or the break's value, each decided at `arm`'s predicate.
/// Interval arithmetic answers with `f64` bounds, lifted here so the
/// limb is banded at the caller's scalar like every other residual
/// (field docs).
pub(crate) fn hull_limb<T: Decide>(
    arm: &HullArm,
    stopped: Subdivided,
    band: Band,
    at: &mut Refused,
) -> Result<T, SsiError> {
    let hull = match stopped {
        Subdivided::Bound(hull) => hull,
        Subdivided::Value { hull, at: t, floor } => {
            let value = T::from_computed(floor);
            let decided = decide_reported(arm.value, Margin::of(value), band);
            at.spans.push(RefusedSpan { lo: t, hi: t });
            hull.uncleared(band, &mut at.spans);
            limb_verdict(
                arm.value_limb,
                decided,
                || RefusedResidual::Floor(floor),
                at,
            )?;
            hull
        }
    };
    let sup = T::from_computed(hull.sup);
    let decided = decide_reported(arm.sup, Margin::of(sup), band);
    if locatable(&decided) {
        hull.uncleared(band, &mut at.spans);
    }
    limb_verdict(
        arm.sup_limb,
        decided,
        || RefusedResidual::Over(hull.sup),
        at,
    )?;
    Ok(sup)
}

/// The uniform breaks limb 2's composite is cut at: the carrier
/// domain's `SSI_CERT_SPANS` grid, minus every point within the grid's
/// clearance ([`geom_core::spline::algebra::GRID_CLEARANCE`] of the
/// spacing) of an interior knot of EITHER curve. The composite merges
/// both curves' knots into its break list, so a grid point near either
/// one's knot would open a narrow span beside it.
pub(crate) fn chart_breaks(carrier: &KnotVector, pcurve: &KnotVector) -> Vec<f64> {
    let (lo, hi) = carrier.domain();
    let knots: Vec<f64> = carrier
        .interior_knots()
        .chain(pcurve.interior_knots())
        .map(|(k, _)| k)
        .collect();
    range_grid_points(lo, hi, SSI_CERT_SPANS, &knots)
}

/// The box chain covering a carrier: one padded box per span of its
/// [`refined_net`], from the span's control hull (containment by the
/// convex-hull property, positive weights making the rational basis a
/// partition of unity), beside the box's axis and the span it hulls.
fn box_chain<T: Decide + Bounds + CertifiedEnclosure>(
    carrier: &NurbsCurve3<T>,
) -> Vec<(Box3, Vec3<T>, (f64, f64))> {
    let mut out = Vec::new();
    // A refused insertion chain returns an EMPTY chain, which limb 3
    // reads as a definite refusal (no box, so nothing banked).
    let Some((kv, coords)) = refined_net(
        carrier.knots_f64_interim(),
        carrier.weights_f64_interim(),
        &carrier.certified_coords(),
    ) else {
        return out;
    };
    let kv = &kv;
    // One pair per coordinate channel, minted once outside the span
    // walk. The net and the vector come from one insertion chain, so
    // the count relation holds by construction and the refusal arm is
    // unreachable; it too returns an EMPTY chain.
    let (Some(cx), Some(cy), Some(cz)) = (
        kv.with_coeffs(&coords[0]),
        kv.with_coeffs(&coords[1]),
        kv.with_coeffs(&coords[2]),
    ) else {
        return out;
    };
    for index in kv.first_span()..=kv.last_span() {
        // Emptiness check and window construction are one step; the
        // three channels share the vector, so they refuse alike.
        let (Some(wx), Some(wy), Some(wz)) = (cx.span(index), cy.span(index), cz.span(index))
        else {
            continue;
        };
        let (a, b) = (kv.knots()[index], kv.knots()[index + 1]);
        let hx = wx.hull();
        let hy = wy.hull();
        let hz = wz.hull();
        let bx = Box3 {
            x: hx,
            y: hy,
            z: hz,
        };
        // The box's axis: the carrier's own tangent at the span
        // midpoint, normalized. Any direction transverse to the
        // solution set works; this is the natural one.
        //
        // No `n > 0.0` guard: it would be a value branch, which generic
        // evaluation code may not take (Q1). A degenerate span
        // (zero-length tangent) divided by zero poisons `e`, the graph
        // margin's enclosure is refused with it, and the probe reads
        // that rung as unprobed ([`tube_margin`]).
        let t = carrier.deriv(T::from_f64(0.5 * (a + b)));
        let n = t.norm();
        out.push((bx, t / n, (a, b)));
    }
    out
}

/// One rung of either arm's probe: the tube it records, the chain's
/// smallest zero-free margin (dimensionless, the `sin θ` scale), its box
/// count, and, where the margin is zero-free, what the one-arc proof
/// found. `one_arc` is `None` where the chain is no graph, so the proof
/// had no hypothesis; it never reads as proved where nothing was. The
/// margin is `NaN` where a box's transversality enclosure was refused
/// ([`Rung::refused`]).
#[derive(Clone, Copy, Debug)]
pub(crate) struct Rung {
    pub(crate) tube: SsiTube<f64>,
    pub(crate) margin: f64,
    pub(crate) boxes: u32,
    pub(crate) one_arc: Option<Result<(), Shortfall>>,
}

impl Rung {
    /// A rung at which a box's transversality enclosure was refused
    /// ([`tube_margin`]): it measured nothing, so it has no margin and
    /// no proof, and [`limb_three`] counts it rather than reading it.
    fn refused(tube: SsiTube<f64>) -> Self {
        Self {
            tube,
            margin: f64::NAN,
            boxes: 0,
            one_arc: None,
        }
    }
}

/// **One padded chain box's graph margin** for an analytic pair: the
/// zero-free reading ([`tube_margin`]) of `(∇f₁ × ∇f₂)·e` over `bx`
/// ([`graph_margin`]); `None` where the enclosure is refused. Limb 3's
/// probe and the coarse-span reading ([`coarse_tube_spans`]) read a box
/// by it.
fn box_graph_margin<T: CertifiedBounds>(
    s1: &Surface<T>,
    s2: &Surface<T>,
    bx: Box3,
    e: Vec3<T>,
) -> Option<f64> {
    tube_margin(graph_margin(s1, s2, bx, e))
}

/// Limb 3's **enclosure probe** for an analytic pair: the graph
/// criterion in ℝ³ over a box chain of the given radius, and, where the
/// margin is zero-free, the one-arc proof over the chain cut to `slab`
/// where a search clips to one ([`one_arc_r3`]), spanning the carrier
/// between its `ends`.
///
/// Pure enclosure arithmetic — **no trilean here**. Choosing the tube's
/// radius is cache *structure* (C6's f64 selection lane), and mixing
/// the ladder's rejected rungs into the K-funnel would both pollute the
/// telemetry and blur the one decision that matters. The single named
/// decision runs once, upstairs, on the chosen radius.
///
/// `None` when the chain is empty or broken, a definite structural
/// refusal; [`Rung::refused`] where a box's transversality enclosure is.
fn probe_tube_analytic<T: Decide + Bounds + CertifiedEnclosure>(
    chain: &[(Box3, Vec3<T>, (f64, f64))],
    s1: &Surface<T>,
    s2: &Surface<T>,
    radius: f64,
    (slab, ends, eps): (Option<Box3>, [Box3; 2], f64),
) -> Option<Rung> {
    if chain.is_empty() {
        return None;
    }
    let mut worst = f64::INFINITY;
    let mut prev: Option<Box3> = None;
    let mut padded = Vec::with_capacity(chain.len());
    for (raw, e, _) in chain.iter() {
        // The chain is built once (an O(n²) knot refinement) and padded
        // per ladder rung, which is a pure interval widening.
        let bx = raw.pad(radius);
        // The chain must actually be a chain: consecutive boxes share
        // the span endpoint the carrier passes through, so a definite
        // separation means the cover is not connected and the
        // concatenation argument does not hold.
        if prev.is_some_and(|p| p.definitely_disjoint(bx)) {
            return None;
        }
        prev = Some(bx);
        padded.push(bx);
        let Some(m) = box_graph_margin(s1, s2, bx, *e) else {
            return Some(Rung::refused(SsiTube::Spatial { radius }));
        };
        if m < worst {
            worst = m;
        }
    }
    let axis = |e: &Vec3<T>| dominant_axis([e.x, e.y, e.z].map(|c| 0.5 * (c.lo() + c.hi())));
    let (Some((_, e0, _)), Some((_, e1, _))) = (chain.first(), chain.last()) else {
        return None;
    };
    let one_arc = (worst > 0.0).then(|| {
        one_arc_r3(
            s1,
            s2,
            (&padded, slab),
            [(ends[0], axis(e0)), (ends[1], axis(e1))],
            eps,
        )
    });
    #[allow(clippy::cast_possible_truncation)]
    Some(Rung {
        tube: SsiTube::Spatial { radius },
        margin: worst,
        boxes: chain.len() as u32,
        one_arc,
    })
}

/// One span of a pcurve's chart tube: the span's hull padded per axis,
/// and the span's parameter midpoint, where the probe reads the
/// pcurve's tangent.
#[derive(Clone, Copy, Debug)]
pub(crate) struct ChartWindow {
    /// The padded window, in chart units.
    pub(crate) rect: UvRect,
    /// The span's parameter midpoint.
    pub(crate) mid: f64,
    /// The span's parameter ends.
    pub(crate) ends: (f64, f64),
}

/// The chart tube's windows: **one padded rectangle per nonempty span**
/// of the pcurve, the span's hull widened by `pad = (pad_u, pad_v)`.
/// Limb 3's probe proves over exactly these, and the accounting pass
/// banks exactly these, so the two read one region.
///
/// Per span, deliberately — not one rectangle around the whole pcurve:
/// a bounding box of a curve that wanders across its domain contains
/// regions the curve never enters, and the accounting pass would
/// account for cells nobody proved anything about.
///
/// `None` when any span's hull is refused: a refused hull is NaI, and
/// its NaN endpoints are no window. No span of such a pcurve is
/// bounded, so the probe has nothing to prove over and the accounting
/// pass banks nothing — the stricter answer for both. The window hull
/// maps any refusal to NaI, so `!is_certified()` and `Real::is_poison`
/// agree on it today; the check asks `is_certified` because that is the
/// question that stays right if the hull ever hands a refusal on with
/// real endpoints.
pub(crate) fn chart_tube_windows<T: geom_core::CertifiedBounds>(
    pcurve: &NurbsCurve2<T>,
    pad: (f64, f64),
) -> Option<Vec<ChartWindow>> {
    let kv = pcurve.knots_f64_interim();
    let coords = pcurve.certified_coords();
    // One pair per chart channel, minted once: the coordinates and the
    // knots are both the pcurve's, so the count is `NurbsCurve2::new`'s
    // fact and the refusal arm is unreachable by construction.
    let (cu, cv) = kv.with_coeffs(&coords[0]).zip(kv.with_coeffs(&coords[1]))?;
    let mut out = Vec::new();
    for index in kv.first_span()..=kv.last_span() {
        // Emptiness check and window construction are one step; both
        // channels share the vector, so both refuse the same indices.
        let (Some(wu), Some(wv)) = (cu.span(index), cv.span(index)) else {
            continue;
        };
        let (hu, hv) = (wu.hull(), wv.hull());
        if !hu.is_certified() || !hv.is_certified() {
            return None;
        }
        let (a, b) = (kv.knots()[index], kv.knots()[index + 1]);
        out.push(ChartWindow {
            rect: UvRect {
                u: (hu.lo() - pad.0, hu.hi() + pad.0),
                v: (hv.lo() - pad.1, hv.hi() + pad.1),
            },
            mid: 0.5 * (a + b),
            ends: (a, b),
        });
    }
    Some(out)
}

/// Limb 3's enclosure probe for the **plane × NURBS** arm: the same
/// criterion in the NURBS chart, where the locus is
/// `φ(u,v) = n·(S(u,v) − p₀) = 0` and `∇φ = (n·S_u, n·S_v)`.
///
/// - A zero-free enclosure of the component of `∇φ` transverse to the
///   pcurve's own tangent makes the zero set a graph over each window.
/// - Where it does, [`one_arc`] decides whether the chain holds one arc
///   spanning the carrier between its `ends`, and nothing else.
///
/// The region is [`chart_tube_windows`] at `pad`, the ladder's `rung`
/// over each axis's chart speed.
///
/// `Ok(None)` when no window can be probed at this pad; a smaller rung
/// may still answer.
///
/// # Errors
///
/// [`SsiError::TubeDegenerate`] when a window's transverse stretch is
/// zero, or the pcurve's tangent at a span midpoint is zero or not
/// finite: neither depends on the pad, so no smaller rung cures it.
fn probe_tube_chart<T: Decide + Bounds + CertifiedEnclosure>(
    pcurve: &NurbsCurve2<T>,
    boxes: &NurbsBoxes<'_, T>,
    (origin, normal): (Point3<T>, Vec3<T>),
    (rung, pad): (f64, (f64, f64)),
    (ends, band): ([Box3; 2], Band),
) -> Result<Option<Rung>, SsiError> {
    #[cfg(test)]
    tests::CHART_PROBES.with(|n| n.set(n.get() + 1));
    let surface = boxes.surface();
    let Some(windows) = chart_tube_windows(pcurve, pad) else {
        return Ok(None);
    };
    // The plane equation is what the whole limb certifies, so the
    // plane crosses through the CERTIFIED door: a component whose
    // computation left its domain is refused, and so is every window's
    // margin, rather than a zero-free enclosure of an equation nobody
    // evaluated.
    let n = [normal.x, normal.y, normal.z].map(Interval::from_certified);
    let mut worst = f64::INFINITY;
    let mut probed = Vec::with_capacity(windows.len());
    for w in windows {
        let ((u0, u1), (v0, v1)) = (w.rect.u, w.rect.v);
        // The transverse chart direction is a DIRECTION — structure —
        // so it is selected through the bracket, exactly as the tube
        // ladder's radius is. `powi(2)`, never `t.x * t.x`.
        let t = pcurve.deriv(T::from_f64(w.mid));
        let tn = (t.x.powi(2) + t.y.powi(2)).sqrt().hi();
        let (tx, ty) = (t.x.hi(), t.y.hi());
        // A positive finite norm and a nonzero direction: a lane whose
        // root pads an exact `0` outward reads a zero tangent as a
        // positive norm, so the zero is asked of `(tx, ty)` too.
        // The tangent is the pcurve's alone, so an unusable one refuses
        // at this rung rather than sending the ladder down rungs that
        // read the same tangent.
        if !tn.is_finite() || tn <= 0.0 || tx.abs().max(ty.abs()) <= 0.0 {
            return Err(SsiError::TubeDegenerate(
                super::TubeDegeneracy::PcurveTangentUnusable,
            ));
        }
        // A window that names no region has no stretch to read.
        let Some(rect) = UvWindow::new(u0, u1, v0, v1) else {
            return Ok(None);
        };
        let Some(margin) = chart_transverse_margin(boxes, n, rect, (tx, ty, tn))? else {
            return Ok(None);
        };
        if margin.is_nan() {
            return Ok(Some(Rung::refused(SsiTube::Chart {
                rung,
                pad_u: pad.0,
                pad_v: pad.1,
            })));
        }
        if margin < worst {
            worst = margin;
        }
        probed.push((w, (-ty / tn, tx / tn)));
    }
    if probed.is_empty() {
        return Ok(None);
    }
    let one = (worst > 0.0).then(|| {
        let p0 = [origin.x, origin.y, origin.z].map(Interval::from_certified);
        let domain = UvRect {
            u: surface.knots_u_f64_interim().domain(),
            v: surface.knots_v_f64_interim().domain(),
        };
        // An `Undecided` here stays unnamed: past the side arm it may come
        // from that arm's own in-band readings, which are indeterminate.
        one_arc(boxes, (n, p0), (domain, band), pcurve, &probed, ends)
    });
    #[allow(clippy::cast_possible_truncation)]
    Ok(Some(Rung {
        tube: SsiTube::Chart {
            rung,
            pad_u: pad.0,
            pad_v: pad.1,
        },
        margin: worst,
        boxes: probed.len() as u32,
        one_arc: one,
    }))
}

/// The most boxes a two-chart tube's chain has: consecutive span windows
/// are hulled in runs of equal length to stay within it. A structure
/// choice (C6's `f64` lane): the one-arc proof walks every box's eight
/// faces, so its cost grows with the count, and a box over several
/// spans proves the same theorem over a wider region.
pub(crate) const PAIR_TUBE_BOXES: usize = 16;

/// **The two-chart tube's chain**: each box the product of the two
/// pcurves' padded span windows ([`chart_tube_windows`]), consecutive
/// spans hulled in runs so the chain has at most [`PAIR_TUBE_BOXES`],
/// with the parameter midpoint of each run. Limb 3 proves over exactly
/// these and the section's accounting banks exactly these.
///
/// `None` when either pcurve has no windows or the two disagree on their
/// spans: both are fitted on the carrier's own parameter, so their spans
/// are one list.
pub(crate) fn pair_tube_chain<T: geom_core::CertifiedBounds>(
    (pa, pb): (&NurbsCurve2<T>, &NurbsCurve2<T>),
    (pad_a, pad_b): PairPads,
) -> Option<Vec<(super::four::Box4, f64)>> {
    let wa = chart_tube_windows(pa, pad_a)?;
    let wb = chart_tube_windows(pb, pad_b)?;
    if wa.len() != wb.len() || wa.is_empty() || wa.iter().zip(&wb).any(|(a, b)| a.ends != b.ends) {
        return None;
    }
    let run = wa.len().div_ceil(PAIR_TUBE_BOXES);
    let hull = |p: (f64, f64), q: (f64, f64)| (p.0.min(q.0), p.1.max(q.1));
    Some(
        wa.chunks(run)
            .zip(wb.chunks(run))
            .map(|(ca, cb)| {
                let side = |w: &[ChartWindow], u: bool| {
                    w.iter()
                        .map(|w| if u { w.rect.u } else { w.rect.v })
                        .reduce(hull)
                        .unwrap_or((f64::NAN, f64::NAN))
                };
                let mid = 0.5 * (ca[0].ends.0 + ca[ca.len() - 1].ends.1);
                (
                    super::four::Box4([
                        side(ca, true),
                        side(ca, false),
                        side(cb, true),
                        side(cb, false),
                    ]),
                    mid,
                )
            })
            .collect(),
    )
}

/// The two-chart tube's pads, each chart's `(u, v)` in its own units.
type PairPads = ((f64, f64), (f64, f64));

/// Limb 3's enclosure probe for the **NURBS × NURBS** arm: the graph
/// criterion in ℝ⁴ ([`super::four::graph_margin`]) over a chain of
/// boxes, each the product of the two pcurves' padded span windows
/// ([`chart_tube_windows`] on each, the spans shared because both
/// pcurves were fitted on the carrier's parameter), and, where it is
/// zero-free, the one-arc proof over the chain cut to the product of
/// the two knot rectangles ([`super::four::one_arc_pair`]).
///
/// `None` when either pcurve has no windows, the two disagree on their
/// spans, or a tangent is unusable: a definite structural refusal at
/// this rung. [`Rung::refused`] where a box's margin is.
fn probe_tube_pair<T: Decide + Bounds + CertifiedEnclosure>(
    (pa, pb): (&NurbsCurve2<T>, &NurbsCurve2<T>),
    (na, nb): (&super::ChartedNurbs<'_, T>, &super::ChartedNurbs<'_, T>),
    (rung, (pad_a, pad_b)): (f64, PairPads),
    band: Band,
) -> Option<Rung> {
    use super::four::{Box4, PairBoxes, dominant, end_reach, graph_margin, one_arc_pair};
    let (sa, sb) = (na.speeds(), nb.speeds());
    let speeds = [sa.u.get(), sa.v.get(), sb.u.get(), sb.v.get()];
    let pair = PairBoxes::new(na, nb, speeds);
    let chain = pair_tube_chain((pa, pb), (pad_a, pad_b))?;
    // The tangent is a DIRECTION — structure — selected through the
    // bracket's upper end as the chart lane's is: the graph margin is
    // certified for whatever direction it is handed, so the read only
    // steers which determinant is asked.
    let tangent = |t: T| -> Option<[f64; 4]> {
        let (da, db) = (pa.deriv(t), pb.deriv(t));
        super::four::scaled_unit([da.x.hi(), da.y.hi(), db.x.hi(), db.y.hi()], speeds)
    };
    let mut worst = f64::INFINITY;
    let mut boxes = Vec::with_capacity(chain.len());
    for (x, mid) in chain {
        // A chain box's midpoint steers which determinant is asked: a
        // point the chain chose, entered as the constant it is.
        let e = tangent(T::from_f64(mid))?;
        // No row reaches this refusal: the ℝ⁴ Jacobian is refused only
        // from a non-finite chart net, whose speeds `ChartedNurbs::mint`
        // refuses before any tube exists. It shares `tube_margin` and
        // the refused rung with the ℝ³ probe, which a row does reach.
        let Some(m) = tube_margin(graph_margin(&pair, x, e)) else {
            return Some(Rung::refused(SsiTube::TwoChart { rung, pad_a, pad_b }));
        };
        worst = worst.min(m);
        boxes.push(x);
    }
    // An end's state is a point the one-arc proof looks for inside an
    // end box `ε/8` wide on each axis: on the `f64` lane the bracket is
    // the point, and on a wider one any point of it is as good a read.
    let state = |t: T| {
        let (p, q) = (pa.eval(t), pb.eval(t));
        [p.x.lo(), p.y.lo(), q.x.lo(), q.y.lo()]
    };
    // Each end's dominant axis is read off its tangent; an unusable one
    // refuses the rung, as one inside the chain does.
    let (t0, t1) = pa.knot_values().domain();
    let [Some(e0), Some(e1)] = [t0, t1].map(tangent) else {
        return None;
    };
    let one = (worst > 0.0).then(|| {
        let ends = [(t0, e0), (t1, e1)].map(|(t, raw)| {
            let e: [f64; 4] = core::array::from_fn(|k| raw[k] / speeds[k]);
            (state(t), dominant(&pair, e))
        });
        let domain = Box4::of_charts(na.surface(), nb.surface());
        let mut along = None;
        for (k, p) in [pa, pb].into_iter().enumerate() {
            let (u, v) = (2 * k, 2 * k + 1);
            let rect = UvRect {
                u: domain.0[u],
                v: domain.0[v],
            };
            let within = (
                end_reach(band.zero(), speeds[u]),
                end_reach(band.zero(), speeds[v]),
            );
            if super::four::along_side(p, rect, within) {
                along = u32::try_from(k).ok();
                break;
            }
        }
        one_arc_pair(&pair, (&boxes, domain, along), ends, band.zero())
    });
    #[allow(clippy::cast_possible_truncation)]
    Some(Rung {
        tube: SsiTube::TwoChart { rung, pad_a, pad_b },
        margin: worst,
        boxes: boxes.len() as u32,
        one_arc: one,
    })
}

/// [`certify_branch`]'s routing boundary for a NURBS operand with no
/// traced pcurve — the first operand's, which never has one.
pub(crate) const NURBS_LIMBS_NEED_PCURVE: &str = "a NURBS operand's limbs need the traced pcurve \
     (the ℝ⁴ trace supplies it; the ℝ³ trace has none)";

/// [`certify_branch`]'s routing boundary for a closed-form carrier
/// between two analytic operands, whose tube is the ℝ³ box chain.
pub(crate) const BOX_CHAIN_NEEDS_SPLINE: &str = "the ℝ³ uniqueness tube of two analytic \
     operands is a chain of a spline carrier's control-net hulls, which a closed-form carrier \
     does not have";

/// [`certify_branch`]'s routing boundary for a NURBS operand whose
/// analytic partner is not a plane.
pub(crate) const CHART_TUBE_NEEDS_PLANE: &str = "the chart uniqueness tube is written for a PLANE \
     against a NURBS surface; another analytic kind needs its own chart form";

/// The uniqueness tube of a certified carrier, as boxes — what the
/// exhaustiveness accounting pass consumes to prove cells "accounted"
/// (spec §4's second state). Same chain limb 3 proved one-arc-ness on,
/// so a cell inside one of these boxes is inside a region where the
/// solution set is exactly the branch already found.
pub(crate) fn tube_boxes<T: Decide + Bounds + CertifiedEnclosure>(
    carrier: &NurbsCurve3<T>,
    radius: f64,
) -> Vec<Box3> {
    box_chain(carrier)
        .into_iter()
        .map(|(b, _, _)| b.pad(radius))
        .collect()
}

/// [`tube_boxes`], each beside the carrier's parameter span it hulls.
pub(crate) fn tube_chain(carrier: &NurbsCurve3<f64>, radius: f64) -> Vec<(Box3, (f64, f64))> {
    box_chain(carrier)
        .into_iter()
        .map(|(b, _, span)| (b.pad(radius), span))
        .collect()
}

/// **The spans that hold an ℝ³ tube at the rung `radius`**
/// ([`super::refine::refine_for_tube`]): with `up` the ladder's next
/// rung above `radius` ([`tube_ladder`] over `extent` and `band`), the
/// carrier's parameter intervals whose chain box ([`box_chain`]) is wider
/// than `up` and, padded by it, is not shown a graph
/// ([`box_graph_margin`]). A box no wider than the rung is held by its
/// padding, which halving its span does not narrow, so it is not named.
/// Empty where `radius` is the ladder's widest rung.
pub(crate) fn coarse_tube_spans(
    carrier: &NurbsCurve3<f64>,
    (s1, s2): (&Surface<f64>, &Surface<f64>),
    radius: f64,
    (extent, band): (f64, Band),
) -> Vec<RefusedSpan> {
    let Some(up) = tube_ladder(extent, band).filter(|r| *r > radius).last() else {
        return Vec::new();
    };
    box_chain(carrier)
        .into_iter()
        .filter(|(raw, e, _)| {
            raw.width() > up && !box_graph_margin(s1, s2, raw.pad(up), *e).is_some_and(|m| m > 0.0)
        })
        .map(|(_, _, (lo, hi))| RefusedSpan { lo, hi })
        .collect()
}

/// Why limb 3 did not prove a tube's chain one arc, read at the
/// narrowest rung whose chain was a graph but not proved one arc.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OneArcRefusal {
    /// A box of the chain, cut to the wall's domain (and to a search's
    /// slab where it clips to one), holds this many simple solutions on
    /// its boundary, certified, where one arc through it gives two. None:
    /// no arc runs through it. More: another arc of the locus lies in it,
    /// or the locus leaves the box's region and comes back inside it.
    Count {
        /// The certified count.
        solutions: u32,
    },
    /// Two consecutive boxes each hold one piece of the locus, and the
    /// reading that would show the pieces meet certifies no shared
    /// solution there.
    Unlinked,
    /// The chain's pieces stop short of an end of the carrier, certified:
    /// an arc's box holds no solution on the slice through that end and
    /// none within ε of it, or a side's stretch does not reach the end,
    /// or the boundary pass reads the side clear there. Read also where a
    /// box resolved no piece, when the end is certified unreached by its
    /// own box: no chain reaches it.
    Short,
    /// A walk of a box's boundary resolved no count, or a linking
    /// reading no sign. A solution on the boundary may be tangential to
    /// it, sit where faces of the box meet, or lie below the walk's
    /// resolution. The cause carries the escalation as the funnel
    /// recorded it. On the two-chart tube the two shapes below are
    /// renamed out of it.
    Undecided(Indeterminate),
    /// The two-chart tube's walk resolved nothing, and the carrier's
    /// image on a chart lies along a side of that chart's domain (a
    /// boundary row or column) within the end box: the locus may run on
    /// a face of every box rather than cross it, where no root is
    /// isolated. The image is placed, not the locus.
    AlongSide {
        /// The chart: `0` the first operand's, `1` the second's.
        chart: u32,
    },
    /// The two-chart tube's walk resolved nothing at an end of the
    /// carrier, which lies on this many sides of the two charts' domains
    /// at once, each within the end box: its root may sit on an edge or
    /// corner where those sides meet, which no face's walk isolates.
    EndOnSides {
        /// How many domain sides the end lies on.
        sides: u32,
    },
}

impl core::fmt::Display for OneArcRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self {
            Self::Count { solutions: 0 } => {
                "a box of the chain holds no boundary solution: no arc of the locus runs \
                 through it"
            }
            Self::Count { solutions } => {
                return write!(
                    f,
                    "a box of the chain holds {solutions} boundary solutions, not two: a \
                     second arc, or the locus leaving and re-entering the box's region"
                );
            }
            Self::Unlinked => {
                "two consecutive boxes each hold one piece, and no shared solution joins them"
            }
            Self::Short => {
                "the pieces stop short of an end of the carrier: no solution lies on the \
                 slice through that end or within the tolerance of it"
            }
            Self::Undecided(_) => {
                "a box's boundary walk resolved no count: a solution tangent to it, where \
                 its faces meet, or below the walk's resolution"
            }
            Self::AlongSide { chart } => {
                return write!(
                    f,
                    "a box's boundary walk resolved no count, and the carrier lies along a \
                     boundary row of chart {chart}'s domain: the locus may run on a face of every \
                     box, where no root is isolated"
                );
            }
            Self::EndOnSides { sides } => {
                return write!(
                    f,
                    "the walk at an end of the carrier resolved nothing, and that end lies on \
                     {sides} domain sides at once, where no face's walk isolates its root"
                );
            }
        })
    }
}

/// Limb 3's ladder: probe each rung, widest first, and take the first
/// whose chain is a graph (zero-free margin) and, where one arc was
/// asked, proved one arc. That rung's margin is decided once
/// ([`tube_transversality`]).
///
/// With no rung taken, the refusal is the narrowest rung's:
/// - a rung that straddled refuses as the band decides its margin;
/// - a rung that was a graph but not one arc refuses
///   [`SsiError::TubeNotOneArc`] with what that rung found.
///
/// A narrower rung is the more specific reading of the same carrier, so
/// it speaks over every wider one.
fn limb_three<T: Decide>(
    (widest, extent): (f64, T),
    band: Band,
    mut probe: impl FnMut(f64) -> Result<Option<Rung>, SsiError>,
) -> Result<(Rung, T), SsiError> {
    // The ladder is materialised so its EMPTINESS is a distinguishable
    // outcome. An empty ladder means every rung fell below the floor —
    // a structural fact about extent against ε, decided before any box
    // is probed — and it must refuse as itself rather than fall through
    // to the no-rung-answered path below with a manufactured margin.
    let ladder: Vec<f64> = tube_ladder(widest, band).collect();
    if ladder.is_empty() {
        return Err(SsiError::TubeLadderEmpty {
            extent: widest,
            floor: SSI_TUBE_RADIUS * band.zero(),
        });
    }
    let mut not_one_arc = 0u32;
    let mut refused = 0u32;
    let mut narrowest: Option<Rung> = None;
    for radius in ladder.iter().copied() {
        let Some(rung) = probe(radius)? else {
            continue;
        };
        if rung.margin.is_nan() {
            refused += 1;
            continue;
        }
        match (rung.margin > 0.0, rung.one_arc) {
            (true, Some(Ok(()))) => {
                let t = tube_transversality(rung.margin, extent, rung.boxes, band)?;
                return Ok((rung, t));
            }
            // A graph whose proof did not run never reads as proved.
            (true, _) => not_one_arc += 1,
            (false, _) => {}
        }
        narrowest = Some(rung);
    }
    let Some(rung) = narrowest else {
        // Rungs were offered and none answered. Structural, and it
        // carries no margin: nothing was ever measured, so there is no
        // honest number to report.
        #[allow(clippy::cast_possible_truncation)]
        return Err(SsiError::TubeProbeSilent {
            rungs: ladder.len() as u32,
            refused,
        });
    };
    let shortfall = match (rung.margin > 0.0, rung.one_arc) {
        (true, Some(Err(shortfall))) => shortfall,
        (true, _) => Shortfall::Undecided,
        (false, _) => {
            let t = tube_transversality(rung.margin, extent, rung.boxes, band)?;
            return Ok((rung, t));
        }
    };
    let cause = match shortfall {
        Shortfall::Count(solutions) => OneArcRefusal::Count { solutions },
        Shortfall::Unlinked => OneArcRefusal::Unlinked,
        Shortfall::Short => OneArcRefusal::Short,
        Shortfall::AlongSide(chart) => OneArcRefusal::AlongSide { chart },
        Shortfall::EndOnSides(sides) => OneArcRefusal::EndOnSides { sides },
        // The walk read no margin it could classify, and the funnel
        // records that as it records any unreadable one; an unreadable
        // margin never classifies positive.
        Shortfall::Undecided => {
            match decide_positive("ssi_tube_one_arc", Margin::of(T::from_f64(f64::NAN)), band) {
                Err(cause) => OneArcRefusal::Undecided(cause),
                Ok(()) => {
                    return Err(SsiError::TubeProbeSilent {
                        rungs: not_one_arc,
                        refused: 0,
                    });
                }
            }
        }
    };
    Err(SsiError::TubeNotOneArc {
        rungs: not_one_arc,
        cause,
    })
}

/// The certificate the three limbs proved.
fn certificate<T: Real>(
    (on_locus, hull_sup): (T, T),
    (rung, transversality): (Rung, T),
) -> SsiCertificate<T> {
    SsiCertificate {
        samples: CERT_SAMPLES,
        on_locus_max: on_locus,
        hull_sup,
        tube: rung.tube.map(T::from_computed),
        tube_transversality: transversality,
        tube_boxes: rung.boxes,
    }
}

/// An operand of limbs 1 and 2: what [`incidence_limbs`] reads of it,
/// and nothing a tube needs (no chart speeds).
#[derive(Clone, Copy)]
pub(crate) enum IncidenceOperand<'a, T: geom_core::CertifiedBounds> {
    /// An analytic surface, read through its implicit form.
    Analytic(&'a Surface<T>),
    /// A spline chart, read through the carrier's image on it.
    Chart(&'a NurbsSurface<T>, &'a NurbsCurve2<T>),
}

/// **Limbs 1 and 2 against each of `operands`**: the carrier's largest
/// sampled on-locus residual and its certified sup-norm distance over
/// `params`, the worst over the operands. An analytic operand reads its
/// implicit form; a chart, the carrier's image on it (the foot residual
/// at each sample, and the composite hull of `|S(P(t)) − C(t)|` between
/// them). Incidence alone: nothing here reads how the operands meet,
/// which is limb 3's and the lanes' transversality's.
///
/// # Errors
///
/// Either limb's refusal with its measured bound.
pub(crate) fn incidence_limbs<T: Decide + Bounds + CertifiedEnclosure>(
    carrier: &Curve3<T>,
    params: (T, T),
    operands: &[IncidenceOperand<'_, T>],
    band: Band,
    at: &mut Refused,
) -> Result<(T, T), SsiError> {
    let mut on_locus = T::zero();
    let mut hull_sup = T::zero();
    for op in operands {
        let (l1, l2) = match *op {
            IncidenceOperand::Analytic(s) => analytic_limbs(carrier, params, s, band, at)?,
            IncidenceOperand::Chart(s, p) => nurbs_limbs(carrier, params, p, s, band, at)?,
        };
        on_locus = on_locus.max(l1);
        hull_sup = hull_sup.max(l2);
    }
    Ok((on_locus, hull_sup))
}

/// Which limbs a certificate asks: all three in order, or limb 3 alone,
/// which refinement asks once of a carrier whose refused residual
/// stopped falling ([`super::refine::refine_by_certificate`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Limbs {
    /// Limbs 1, 2 and 3, refusing at the first that refuses.
    All,
    /// Limb 3 alone: whether the carrier's tube holds one arc.
    Tube,
}

/// What a carrier is certified against. Each arm carries its own
/// lane's inputs, so no lane reads another's; limb 3 proves the same
/// theorem on each, over the region it is cut to.
pub(crate) enum Lane<'a, T: geom_core::CertifiedBounds> {
    /// At rest, any operand pair: an ℝ³ chain is cut to nothing, a chart
    /// chain to the wall's knot rectangle.
    AtRest {
        /// The first operand.
        a: &'a SsiOperand<'a, T>,
        /// The second operand.
        b: &'a SsiOperand<'a, T>,
        /// The first operand's pcurve, where both are NURBS charts.
        pcurve_a: Option<&'a NurbsCurve2<T>>,
        /// The second operand's pcurve, where it is a NURBS wall.
        pcurve_b: Option<&'a NurbsCurve2<T>>,
    },
    /// A plane × NURBS search's branch, its chain cut to the wall's knot
    /// rectangle, which the search banks.
    Chart {
        /// The plane.
        plane: &'a Surface<T>,
        /// The wall, its chart speeds minted.
        wall: super::ChartedNurbs<'a, T>,
        /// The wall's pcurve.
        pcurve: &'a NurbsCurve2<T>,
    },
    /// A NURBS × NURBS search's branch, its chain cut to the product of
    /// the two knot rectangles, which the search banks.
    Pair {
        /// The first chart, its speeds minted.
        a: super::ChartedNurbs<'a, T>,
        /// The second chart, its speeds minted.
        b: super::ChartedNurbs<'a, T>,
        /// The first chart's pcurve.
        pcurve_a: &'a NurbsCurve2<T>,
        /// The second chart's pcurve.
        pcurve_b: &'a NurbsCurve2<T>,
    },
    /// An ℝ³ search's branch, its chain cut to the box the search is
    /// confined to and banks the tube over.
    Spatial {
        /// The two analytic surfaces.
        pair: (&'a Surface<T>, &'a Surface<T>),
        /// The search's box.
        slab: Box3,
    },
}

/// Where limb 1 or 2 refused a carrier: the parameter interval `[lo,
/// hi]`, a point for a limb-1 sample.
#[derive(Clone, Copy, Debug)]
pub(crate) struct RefusedSpan {
    /// The interval's start.
    pub(crate) lo: f64,
    /// The interval's end.
    pub(crate) hi: f64,
}

/// What limbs 1 and 2 leave refinement where they refuse a carrier.
#[derive(Debug, Default)]
pub(crate) struct Refused {
    /// The parameter intervals the refusal lies in.
    pub(crate) spans: Vec<RefusedSpan>,
    /// The refusal, once a limb refused on a margin that is a number.
    pub(crate) refusal: Option<LimbRefusal>,
}

/// A limb-1 or limb-2 refusal, as the refusing limb recorded it.
#[derive(Clone, Copy, Debug)]
pub(crate) struct LimbRefusal {
    /// The limb that refused.
    pub(crate) limb: SsiLimb,
    /// What the classifier saw, for the round's report.
    pub(crate) margin: geom_core::MarginDiag,
    /// What refinement drives down.
    pub(crate) residual: RefusedResidual,
}

impl LimbRefusal {
    /// The round's report: definite or in the band, as the residual is.
    pub(crate) fn round(self) -> RoundMargin {
        match self.residual {
            RefusedResidual::Over(_) | RefusedResidual::Floor(_) => RoundMargin::Over(self.margin),
            RefusedResidual::InBand => RoundMargin::InBand(self.margin),
        }
    }
}

/// A certificate's refusal, with where on the carrier limbs 1 and 2
/// refused it.
#[derive(Debug)]
pub(crate) struct Located {
    /// The refusal, as [`certify_branch`] reports it.
    pub(crate) error: SsiError,
    /// Where it lies and what its limb read, for a refusal of limb 1 or
    /// 2 on a margin that is a number; `None` for every other refusal,
    /// which no density of samples answers.
    pub(crate) at: Option<Box<Spans>>,
}

/// A located refusal: the limb's refusal, and the parameter intervals
/// whose residual did not clear the band's zero, a limb-1 sample as a
/// point interval.
#[derive(Debug)]
pub(crate) struct Spans {
    /// The refusal.
    pub(crate) refusal: LimbRefusal,
    /// The carrier's parameter intervals the refusal lies in.
    pub(crate) spans: Vec<RefusedSpan>,
}

/// A refusal the certificate does not locate.
impl From<SsiError> for Located {
    fn from(error: SsiError) -> Self {
        Self { error, at: None }
    }
}

/// [`certify_branch`] at `f64`, locating a limb-1 or limb-2 refusal on
/// the carrier. A limb-3 refusal is not located: no density of samples
/// answers it.
///
/// # Errors
///
/// As [`certify_branch`], with the refused intervals.
pub(crate) fn certify_located(
    carrier: &NurbsCurve3<f64>,
    lane: Lane<'_, f64>,
    extent: f64,
    band: Band,
    limbs: Limbs,
) -> Result<SsiCertificate<f64>, Located> {
    let mut refused = Refused::default();
    let (params, carrier) = (
        carrier.domain(),
        Curve3::Nurbs(std::sync::Arc::new(carrier.clone())),
    );
    certify_branch(&carrier, params, lane, extent, band, limbs, &mut refused).map_err(|error| {
        let at = match refused.refusal {
            Some(refusal) if !refused.spans.is_empty() => Some(Box::new(Spans {
                refusal,
                spans: refused.spans,
            })),
            _ => None,
        };
        Located { error, at }
    })
}

/// Certify a fitted rung-3 carrier on its [`Lane`] — all three limbs, in
/// order, refusing typed at the first failure.
///
/// # Errors
///
/// [`SsiError::CertificateLimb`] naming the limb,
/// [`SsiError::TubeStraddles`] for the sliver case,
/// [`SsiError::TubeNotOneArc`] where a search's tube was a graph but not
/// proved one arc, [`SsiError::ChartSpeed`] when the chart is constant
/// across the locus over a tube window,
/// [`SsiError::FootPointInconclusive`] when a NURBS foot will not
/// converge, [`SsiError::CertificateEscalated`] naming the limb whose trilean
/// escalated.
///
/// `extent` is the caller's named feature extent, which levers limb 3's
/// transversality clearance and sets the tube ladder's widest rung.
///
/// The tolerance is `band`'s and only `band`'s. A linear band's
/// `zero()` **is** the run's ε, and every threshold this function
/// applies — the three limbs' trileans and the tube ladder's floor —
/// reads it from there, so there is no second number a caller could
/// certify at.
///
/// A limb-1 or limb-2 refusal is located on the carrier in `at`
/// ([`Refused`]), with its refused residual, for
/// [`super::refine::refine_by_certificate`].
///
/// The carrier is of any kind, `params` its edge's interval: a spline
/// spans its own knot domain, any other carrier `params`
/// ([`crate::carrier_net::carrier_span`]), and limbs 1–2 read it through
/// the carrier-enclosure door. The ℝ³ tube of two analytic operands
/// reads a spline's control net ([`box_chain`]) and refuses any other
/// carrier ([`BOX_CHAIN_NEEDS_SPLINE`]).
pub(crate) fn certify_branch<T: Decide + Bounds + CertifiedEnclosure>(
    carrier: &Curve3<T>,
    params: (T, T),
    lane: Lane<'_, T>,
    extent: T,
    band: Band,
    limbs: Limbs,
    at: &mut Refused,
) -> Result<SsiCertificate<T>, SsiError> {
    // Limb 3 reads its theorem over the region the search is cut to: an
    // ℝ³ search's slab; the wall's knot rectangle, which every chart
    // enclosure is clipped to.
    let slab = match lane {
        Lane::Spatial { slab, .. } => Some(slab),
        Lane::AtRest { .. } | Lane::Chart { .. } | Lane::Pair { .. } => None,
    };
    // The pair the first two limbs read, the pcurve beside the second.
    let (first, second);
    let (a, b, pcurve_a, pcurve_b) = match lane {
        Lane::AtRest {
            a,
            b,
            pcurve_a,
            pcurve_b,
        } => (a, b, pcurve_a, pcurve_b),
        Lane::Chart {
            plane,
            wall,
            pcurve,
        } => {
            (first, second) = (SsiOperand::Analytic(plane), SsiOperand::Nurbs(wall));
            (&first, &second, None, Some(pcurve))
        }
        Lane::Pair {
            a,
            b,
            pcurve_a,
            pcurve_b,
        } => {
            (first, second) = (SsiOperand::Nurbs(a), SsiOperand::Nurbs(b));
            (&first, &second, Some(pcurve_a), Some(pcurve_b))
        }
        Lane::Spatial { pair, .. } => {
            (first, second) = (SsiOperand::Analytic(pair.0), SsiOperand::Analytic(pair.1));
            (&first, &second, None, None)
        }
    };
    let asked: &[_] = match limbs {
        Limbs::All => &[(a, pcurve_a), (b, pcurve_b)],
        Limbs::Tube => &[],
    };
    let operands = asked
        .iter()
        .map(|&(op, pc)| match op {
            SsiOperand::Analytic(s) => Ok(IncidenceOperand::Analytic(s)),
            SsiOperand::Nurbs(s) => pc.map(|p| IncidenceOperand::Chart(s.surface(), p)).ok_or(
                SsiError::UnsupportedCertificate {
                    what: NURBS_LIMBS_NEED_PCURVE,
                },
            ),
        })
        .collect::<Result<Vec<_>, _>>()?;
    let (on_locus, hull_sup) = incidence_limbs(carrier, params, &operands, band, at)?;
    // The carrier's two ends, as the carrier's scalar evaluates them: on
    // the f64 lane a point, not an enclosure of the exact end, which the
    // end checks then read within ε of
    // (`work/ssiarith/limb3-carrier-ends-read-at-f64-points.md`).
    let (_, (t0, t1)) = crate::carrier_net::carrier_span(carrier, params);
    let ends = [t0, t1].map(|t| {
        let p = carrier.eval(t);
        Box3 {
            x: Interval::from_certified(p.x),
            y: Interval::from_certified(p.y),
            z: Interval::from_certified(p.z),
        }
    });
    // The extent levers the clearance; its upper end is the ladder's
    // widest rung.
    let widest = Bounds::hi(extent);
    let three = match (a, b) {
        (SsiOperand::Analytic(s1), SsiOperand::Analytic(s2)) => {
            let Curve3::Nurbs(net) = carrier else {
                return Err(SsiError::UnsupportedCertificate {
                    what: BOX_CHAIN_NEEDS_SPLINE,
                });
            };
            let chain = box_chain(net);
            limb_three((widest, extent), band, |radius| {
                Ok(probe_tube_analytic(
                    &chain,
                    s1,
                    s2,
                    radius,
                    (slab, ends, band.zero()),
                ))
            })?
        }
        (SsiOperand::Analytic(plane), SsiOperand::Nurbs(n))
        | (SsiOperand::Nurbs(n), SsiOperand::Analytic(plane)) => {
            let Surface::Plane { origin, normal, .. } = **plane else {
                return Err(SsiError::UnsupportedCertificate {
                    what: CHART_TUBE_NEEDS_PLANE,
                });
            };
            let Some(p) = pcurve_b else {
                return Err(SsiError::UnsupportedCertificate {
                    what: "the chart uniqueness tube needs the traced pcurve",
                });
            };
            limb_three((widest, extent), band, |radius| {
                // The pad per axis: the rung ÷ the operand's minted chart
                // speed along that axis. The padded windows are the
                // proved region and the certificate records them; the
                // rung is the ladder's label, not a metre bound on that
                // region (a window's corner can sit farther than the rung
                // from the carrier).
                let pad = n.speeds().pad(radius);
                probe_tube_chart(p, n.boxes(), (origin, normal), (radius, pad), (ends, band))
            })?
        }
        (SsiOperand::Nurbs(na), SsiOperand::Nurbs(nb)) => {
            let (Some(pa), Some(pb)) = (pcurve_a, pcurve_b) else {
                return Err(SsiError::UnsupportedCertificate {
                    what: "the two-chart uniqueness tube needs both traced pcurves",
                });
            };
            limb_three((widest, extent), band, |radius| {
                Ok(probe_tube_pair(
                    (pa, pb),
                    (na, nb),
                    (radius, (na.speeds().pad(radius), nb.speeds().pad(radius))),
                    band,
                ))
            })?
        }
    };
    Ok(certificate((on_locus, hull_sup), three))
}

/// Limb 3's verdict on the chosen rung. `clearance` is interval
/// arithmetic's zero-free lower bound (`f64`, C9) and `arm` the caller's
/// scalar, so the product — the number the trilean classifies — is
/// scalar-typed. The refusal carries the verdict and the reporting
/// margin the classifier saw. Its `Negative` arm is unreachable while
/// `arm` is positive: `zero_free_lower_bound` never returns a negative
/// clearance.
fn tube_transversality<T: Decide>(
    clearance: f64,
    arm: T,
    boxes: u32,
    band: Band,
) -> Result<T, SsiError> {
    let transversality = Margin::levered(T::from_computed(clearance), arm);
    match band_verdict("ssi_tube_transversality", transversality, band) {
        None => Ok(transversality.value()),
        Some(BandVerdict::Refused(verdict)) => Err(SsiError::TubeStraddles { verdict, boxes }),
        Some(BandVerdict::Undecided(cause)) => Err(SsiError::CertificateEscalated {
            limb: SsiLimb::Tube,
            cause,
        }),
    }
}

#[cfg(test)]
mod tests {
    std::thread_local! {
        /// Chart-lane tube probes run on this thread (one per rung).
        pub(super) static CHART_PROBES: core::cell::Cell<usize> =
            const { core::cell::Cell::new(0) };
    }

    /// A synthetic limb-2 hull over `[0, 1]` with its `cuts` added, every
    /// span bounded by `sup` and no break value certified.
    fn flat_hull(cuts: &[f64], sup: f64) -> super::Hull {
        let breaks: Vec<f64> = core::iter::once(0.0)
            .chain(cuts.iter().copied())
            .chain(core::iter::once(1.0))
            .collect();
        let spans = vec![sup; breaks.len() - 1];
        super::Hull {
            sup,
            floors: vec![0.0; breaks.len()],
            work: spans.len(),
            breaks,
            spans,
            data: (),
        }
    }

    /// Runs [`super::subdivide`] with round `k`'s hull bounded by
    /// `bound(k)`, returning the bound it stopped on and how many hulls
    /// it built.
    #[allow(clippy::panic)]
    fn subdivided(bound: impl Fn(usize) -> f64) -> (f64, usize) {
        use super::{Subdivided, subdivide};
        let band = geom_core::Band::new(1e-9, 1e-8).unwrap_or_else(|e| panic!("{e}"));
        let calls = core::cell::Cell::new(0);
        let hull = |cuts: &[f64]| {
            let k = calls.get();
            calls.set(k + 1);
            Ok::<_, ()>(flat_hull(cuts, bound(k)))
        };
        match subdivide(hull, band) {
            Ok(Subdivided::Bound(h)) => (h.sup, calls.get()),
            Ok(Subdivided::Value { .. }) => panic!("no break value is certified"),
            Err(()) => panic!("the synthetic hull does not refuse"),
        }
    }

    /// **The rounds stop at the floor their data sets** (limb 2's round
    /// stop, [`super::subdivide_until`], with limb 2's own predicate,
    /// [`crate::pcurve_cache::projected::round_floored`]: the bound within
    /// 16× the net's rounding, that rounding 16× the band's zero, and
    /// the bound not halved by the last round): with every round's bound at twice a
    /// rounding that halving does not lower — a family scalar's carrier
    /// spread over its box — the second hull is the last one built, and
    /// the limb reads the bound; without the stop the same hulls are
    /// rebuilt round after round until the stall rule ends them. A bound
    /// still falling tenfold a round goes on below the rounding, and a
    /// hull that clears is still read as clearing.
    #[test]
    #[allow(clippy::panic)]
    fn the_rounds_stop_once_the_bound_is_within_the_floor() {
        use super::{Subdivided, subdivide_until};
        let band = geom_core::Band::new(1e-9, 1e-8).unwrap_or_else(|e| panic!("{e}"));
        let run = |stop: bool, sup: &dyn Fn(usize) -> f64, floor: f64| {
            let calls = core::cell::Cell::new(0);
            let hull = |cuts: &[f64]| {
                let k = calls.get();
                calls.set(k + 1);
                Ok::<_, ()>(flat_hull(cuts, sup(k)))
            };
            let last = core::cell::Cell::new(None::<f64>);
            let floored = |h: &super::Hull| {
                let before = last.replace(Some(h.sup));
                stop && crate::pcurve_cache::projected::round_floored(h.sup, before, floor, band)
            };
            match subdivide_until(hull, band, floored) {
                Ok(Subdivided::Bound(h)) => (h.sup, calls.get()),
                _ => panic!("the synthetic hull stops on its bound"),
            }
        };
        let flat = |_: usize| 0.8;
        assert_eq!(run(true, &flat, 0.4), (0.8, 2));
        let (sup, rounds) = run(false, &flat, 0.4);
        assert_eq!(sup, 0.8);
        assert!(rounds > 2, "without the stop the rounds go on: {rounds}");
        let falling = |k: usize| 1e-3 * 0.1_f64.powi(i32::try_from(k).unwrap_or(0));
        let (cleared, _) = run(true, &falling, 0.4);
        assert!(
            cleared <= band.zero(),
            "a bound falling tenfold a round goes on below the rounding until it clears: \
             {cleared:e}"
        );
        let (_, near) = run(true, &|_| 2e-9, 1.5e-9);
        assert!(
            near > 2,
            "a rounding near the band leaves the rounds on: {near}"
        );
        assert_eq!(
            run(true, &|_| 1e-12, 0.4),
            (1e-12, 1),
            "a hull that clears clears"
        );
    }

    /// **The subdivision stalls on the second round that does not lower
    /// the lowest bound,** and reads the limb on that lowest bound:
    /// round 1 rises, round 2 recovers part of the way, and round 3,
    /// which would clear, is never built.
    #[test]
    fn the_subdivision_stalls_on_two_idle_rounds_against_the_lowest_bound() {
        assert_eq!(subdivided(|k| [1.0, 2.0, 1.5, 1e-12][k]), (1.0, 3));
        // One idle round is let pass.
        assert_eq!(subdivided(|k| [1.0, 2.0, 0.5, 1e-12][k]), (1e-12, 4));
    }

    /// **Unbounded hulls are not idle,** so a run of them halves on
    /// until a work bound stops it, and reads the first hull's `+∞`.
    /// Every span is halved each round, so round `k` recomputes `2^k`
    /// spans: rounds 0 to 10 add 1023 cuts, round 10 is held to the one
    /// cut [`super::SSI_HULL_CUTS`] has left, and round 11 has none, so
    /// the run builds 12 hulls and recomputes 3072 spans, under
    /// [`super::SSI_HULL_WORK`].
    #[test]
    fn a_run_of_unbounded_hulls_halves_to_the_cut_cap() {
        assert_eq!(subdivided(|_| f64::INFINITY), (f64::INFINITY, 12));
    }

    /// **Limb 3's construction site carries the verdict and the margin
    /// it classified.** A clearance inside the zero band refuses `Zero`
    /// with the reporting margin the classifier saw — the levered
    /// clearance, positive or exactly zero: a point at `f64`, a point
    /// enclosure at `Interval` — and a clear one passes. `Negative` has
    /// no row: the clearance is never negative (`zero_free_lower_bound`)
    /// and the arm is positive.
    #[test]
    #[allow(clippy::unwrap_used, clippy::panic)]
    fn the_tube_site_carries_its_verdict_and_margin() {
        use super::{SsiError, tube_transversality};
        use crate::recourse::{Classified, Refused};
        use geom_core::interval::certification::Certification;
        use geom_core::{Band, Interval, MarginDiag};
        let band = Band::new(1e-9, 1e-8).unwrap();
        let verdict = |got: Result<(), SsiError>| match got {
            Err(SsiError::TubeStraddles { verdict, boxes: 4 }) => Some(verdict),
            Ok(()) => None,
            Err(other) => panic!("not the tube's refusal: {other}"),
        };
        let zero = |margin| Some(Refused::Zero(Classified { margin, band }));
        for (clearance, arm, levered) in [
            (5e-10, 1.0, Some(5e-10)),
            (2.5e-10, 2.0, Some(5e-10)),
            (0.0, 1.0, Some(0.0)),
            (1e-6, 1.0, None),
        ] {
            let at_f64 = tube_transversality(clearance, arm, 4, band);
            assert_eq!(
                verdict(at_f64.map(|_| ())),
                levered.and_then(|m| zero(MarginDiag::value(m))),
                "f64: {clearance:e} at {arm}"
            );
            let at_interval = tube_transversality(clearance, Interval::point(arm), 4, band);
            assert_eq!(
                verdict(at_interval.map(|_| ())),
                levered.and_then(|m| zero(MarginDiag::enclosure(m, m))),
                "Interval: {clearance:e} at {arm}"
            );
        }
    }

    /// **The mignitude refuses a refusal that carries real endpoints.**
    /// `zero_free_lower_bound` lives in `ssi::enclose`, a certification
    /// file with no `Real` in scope, and refuses by `!is_certified()`;
    /// this quotient (`Trv`, a strictly positive lower end) is the value
    /// an `is_poison` check would read as a sound bracket, handing its
    /// lower end back as a certified margin. The row pins that the
    /// transversality margin this file reads is `0.0` on it.
    #[test]
    fn the_mignitude_refuses_a_refusal_with_real_endpoints() {
        use super::zero_free_lower_bound;
        use geom_core::{Bounds, Interval};
        let q = Interval::from_bounds(1.0, 2.0) / Interval::from_bounds(0.0, 1.0);
        assert!(!q.is_certified(), "the fixture is a refusal: {q:?}");
        assert!(Bounds::lo(q) > 0.0, "with a positive lower end: {q:?}");
        assert_eq!(zero_free_lower_bound(q), 0.0, "{q:?}");
    }

    /// **The tube ladder's floor is the run band's own ε, exactly.**
    ///
    /// A degradation row, not a violation row. Every other statement
    /// about a uniqueness tube — that it has positive headroom, that it
    /// covers ≥ 1 box, that the accounting terminated — gets *easier*
    /// as the floor drops, so none of them can see the failure this one
    /// is for: a ladder floored at a tolerance finer than the run's
    /// certifies a THINNER tube on a pair where the honest answer is
    /// [`SsiError::CertificateLimb`] on limb 3. The uniqueness theorem
    /// shipped under the same `SsiCertificate` type is then weaker than
    /// the band it is stated in, and nothing else goes red.
    #[allow(clippy::unwrap_used, clippy::panic)]
    #[test]
    fn the_tube_ladders_floor_is_the_run_bands_own_epsilon() {
        use super::{SSI_TUBE_RADIUS, SSI_TUBE_RADIUS_MAX, tube_ladder};
        use geom_core::Band;

        for zero in [1.0e-3_f64, 1.0e-6, 1.0e-9, 1.0e-12] {
            // The escalate edge is arbitrary here: `tube_ladder` reads
            // `band.zero()` and nothing else, so this row is a statement
            // about the coincidence threshold alone. Any value above
            // `zero` satisfies `Band::new`'s `zero < escalate`; it is
            // deliberately NOT the run's K·zero, which would read as a
            // second quantity the ladder consults.
            let band = Band::new(zero, 2.0 * zero).unwrap();
            let floor = SSI_TUBE_RADIUS * zero;
            // An extent chosen so the floor BINDS: the ladder's last
            // possible rung is below it, so the row is about where the
            // ladder stops rather than about it running out of rungs.
            let extent = 1.0e6 * zero;
            let rungs: Vec<f64> = tube_ladder(extent, band).collect();
            assert!(
                !rungs.is_empty(),
                "the ladder must offer a rung at {zero:e}"
            );
            assert!(
                rungs.iter().all(|r| *r >= floor),
                "a rung below the run band's floor {floor:e}: {rungs:?}"
            );
            let last = *rungs.last().unwrap();
            assert!(
                last * 0.5 < floor,
                "the ladder stopped at {last:e} m with room above the floor {floor:e} m \
                 — it is floored at some other tolerance"
            );
            // An extent whose WIDEST rung is already under the floor
            // offers nothing at all: limb 3 refuses rather than proving
            // one-arc-ness over a tube thinner than the carrier's own
            // certified residual.
            assert!(
                tube_ladder(floor, band).next().is_none(),
                "a tube at the floor's own scale must not be certifiable"
            );
            assert!(
                (rungs[0] - SSI_TUBE_RADIUS_MAX * extent).abs() <= f64::EPSILON * extent,
                "the widest rung is the named fraction of the extent"
            );
        }
    }

    /// The chart tube's plane-normal crossing, at the `Interval` scalar.
    ///
    /// The three components of the plane normal enter certification arithmetic through
    /// their own brackets, and at `Interval` a bracket can be sound and
    /// still inadmissible: `sqrt([−1, 4]) + 1` is `[1, 3]` with decoration
    /// `Trv`. The crossing caps that decoration at `Trv`, so a normal
    /// that cannot certify is refused at the crossing — otherwise the
    /// transversality margin is a positive number computed from a plane
    /// equation that was never evaluated where it was asked for.
    #[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
    mod normal_crossing_tests {
        use geom::NurbsCurve2;
        use geom::NurbsSurface;
        use geom_core::spline::KnotVector;
        use geom_core::{Interval, Point2, Point3, Real, Vec3};

        use super::super::probe_tube_chart;
        use crate::ssi::enclose::{Box3, NurbsBoxes};
        use crate::ssi::{SsiError, TubeDegeneracy};

        fn iv(x: f64) -> Interval {
            Interval::from_f64(x)
        }

        /// [`u_line`]'s carrier ends on [`unit_patch`], and a band.
        fn at_ends() -> ([Box3; 2], geom_core::Band) {
            let at = |u: f64| Box3 {
                x: Interval::from_bounds(u, u),
                y: Interval::from_bounds(0.5, 0.5),
                z: Interval::from_bounds(0.0, 0.0),
            };
            (
                [at(0.1), at(0.9)],
                geom_core::Band::new(1e-9, 1e-8).expect("a valid band"),
            )
        }

        /// A point of the plane through [`u_line`]'s image on the unit
        /// patch, so a normal along `y` cuts the patch in that line.
        fn on_line() -> Point3<Interval> {
            Point3::new(iv(0.0), iv(0.5), iv(0.0))
        }

        /// A domain violation whose bracket is finite AND strictly positive,
        /// so the margin it produces is zero-free: the laundered answer is a
        /// *usable* certificate, not an obvious refusal.
        fn trv_pos() -> Interval {
            Interval::from_bounds(-1.0, 4.0).sqrt() + iv(1.0)
        }

        fn linear_kv() -> KnotVector {
            KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).expect("valid knots")
        }

        /// `S(u, v) = (u, v, 0)` — a bilinear patch with `S_u = (1, 0, 0)`
        /// and `S_v = (0, 1, 0)`, so `∇φ = (n·S_u, n·S_v) = (n.x, n.y)`.
        fn unit_patch() -> NurbsSurface<Interval> {
            NurbsSurface::new(
                geom_core::spline::Knots::from(linear_kv()).lift(geom_core::Real::from_f64),
                geom_core::spline::Knots::from(linear_kv()).lift(geom_core::Real::from_f64),
                vec![
                    Point3::new(iv(0.0), iv(0.0), iv(0.0)),
                    Point3::new(iv(0.0), iv(1.0), iv(0.0)),
                    Point3::new(iv(1.0), iv(0.0), iv(0.0)),
                    Point3::new(iv(1.0), iv(1.0), iv(0.0)),
                ],
                vec![geom_core::Real::one(); 4],
            )
            .expect("valid patch")
        }

        /// A `u`-aligned pcurve, so the transverse chart direction is `e_v`
        /// and the margin reads `n.y` alone.
        fn u_line() -> NurbsCurve2<Interval> {
            NurbsCurve2::new(
                geom_core::spline::Knots::from(linear_kv()).lift(geom_core::Real::from_f64),
                vec![Point2::new(iv(0.1), iv(0.5)), Point2::new(iv(0.9), iv(0.5))],
                vec![geom_core::Real::one(); 2],
            )
            .expect("valid pcurve")
        }

        /// The control: a certified normal produces the margin the geometry
        /// implies, so the refusal row below is pinning a decoration read
        /// and not a probe that fails on everything.
        #[test]
        fn a_certified_normal_produces_its_margin() {
            let normal = Vec3::new(iv(0.0), iv(2.0), iv(0.0));
            let probe = probe_tube_chart(
                &u_line(),
                &NurbsBoxes::new(&unit_patch()),
                (on_line(), normal),
                (0.0, (0.01, 0.01)),
                at_ends(),
            )
            .expect("the unit patch's chart is not degenerate")
            .expect("the probe must reach a verdict");
            assert!(
                probe.margin > 0.0,
                "certified normal gave margin {}",
                probe.margin
            );
            assert!(probe.boxes > 0, "no span was probed: the row is vacuous");
            assert_eq!(
                probe.one_arc,
                Some(Ok(())),
                "the window holds the line and nothing else"
            );
        }

        /// The row S41 is about: the same geometry with the *same endpoints*
        /// on `n.y`, differing only in the decoration, must not certify.
        #[test]
        fn a_violated_normal_cannot_certify() {
            let n = trv_pos();
            assert_eq!(
                (geom_core::Bounds::lo(n), geom_core::Bounds::hi(n)),
                (1.0, 3.0),
                "fixture drifted"
            );
            assert!(
                geom_core::CertifiedEnclosure::certified_bracket(n).is_none(),
                "fixture drifted: it certifies"
            );
            let normal = Vec3::new(iv(0.0), n, iv(0.0));
            let verdict = probe_tube_chart(
                &u_line(),
                &NurbsBoxes::new(&unit_patch()),
                (on_line(), normal),
                (0.0, (0.01, 0.01)),
                at_ends(),
            )
            .expect("the unit patch's chart is not degenerate")
            .expect("the probe answers at this rung");
            assert!(
                verdict.margin.is_nan() && verdict.one_arc.is_none(),
                "a domain-violated plane normal produced transversality \
                 margin {} — the crossing read the BRACKET door, so the \
                 chart tube certifies uniqueness from a plane equation that \
                 was clamped out of its own domain; refused, the rung \
                 measured nothing",
                verdict.margin
            );
        }

        /// The same principle on the PCURVE: a control coordinate that left
        /// its domain carries real endpoints (`sqrt([−1, 0.01]) + 0.1` is
        /// about `[0.1, 0.2]` at `Trv`), the span window's hull refuses it as NaI,
        /// and NaI's NaN endpoints must not become a chart window — a NaN
        /// end mints no `ParamRange`, where a window landed on the first
        /// span would certify the derivative boxes of an arbitrary cell.
        #[test]
        fn a_violated_pcurve_coordinate_cannot_certify() {
            let bad = Interval::from_bounds(-1.0, 0.01).sqrt() + iv(0.1);
            let (lo, hi) = (geom_core::Bounds::lo(bad), geom_core::Bounds::hi(bad));
            assert!(
                (0.09..0.21).contains(&lo) && (0.09..0.21).contains(&hi),
                "fixture drifted: [{lo}, {hi}] is not a real bracket near [0.1, 0.2]"
            );
            assert!(!bad.is_certified(), "fixture drifted: it certifies");
            let pcurve = NurbsCurve2::new(
                geom_core::spline::Knots::from(linear_kv()).lift(geom_core::Real::from_f64),
                vec![Point2::new(bad, iv(0.5)), Point2::new(iv(0.9), iv(0.5))],
                vec![geom_core::Real::one(); 2],
            )
            .expect("valid pcurve");
            let normal = Vec3::new(iv(0.0), iv(2.0), iv(0.0));
            let verdict = probe_tube_chart(
                &pcurve,
                &NurbsBoxes::new(&unit_patch()),
                (on_line(), normal),
                (0.0, (0.01, 0.01)),
                at_ends(),
            );
            assert!(
                matches!(verdict, Ok(None)),
                "a pcurve whose control coordinate left its domain produced the \
                 transversality verdict {verdict:?} — the span window's refused \
                 hull was read for its endpoints"
            );
        }

        /// **A chart constant across the locus refuses by name, at the
        /// first rung.** On a patch whose net is one point, `S_u` and
        /// `S_v` are exactly zero, so the stretch along e⊥ is zero over
        /// every window; a smaller pad's window lies inside this one, so
        /// answering "no enclosure, try the next rung" would only run the
        /// ladder down to `TubeProbeSilent`, the wrong diagnosis.
        #[test]
        fn a_chart_constant_across_the_locus_refuses_rather_than_trying_a_smaller_rung() {
            let p = Point3::new(iv(0.3), iv(0.3), iv(0.0));
            let point_patch = NurbsSurface::new(
                geom_core::spline::Knots::from(linear_kv()).lift(geom_core::Real::from_f64),
                geom_core::spline::Knots::from(linear_kv()).lift(geom_core::Real::from_f64),
                vec![p; 4],
                vec![geom_core::Real::one(); 4],
            )
            .expect("a patch a caller can build");
            let normal = Vec3::new(iv(0.0), iv(2.0), iv(0.0));
            let verdict = probe_tube_chart(
                &u_line(),
                &NurbsBoxes::new(&point_patch),
                (on_line(), normal),
                (0.0, (0.01, 0.01)),
                at_ends(),
            );
            assert!(
                matches!(
                    verdict,
                    Err(SsiError::TubeDegenerate(
                        TubeDegeneracy::WallConstantAcrossLocus
                    ))
                ),
                "a chart constant across the locus answered {verdict:?} instead of \
                 refusing by name"
            );
        }

        /// **A chart constant across the locus refuses by name when its
        /// net is not one point.** Rows `a, a` and `b, b`: `S_v` is exactly
        /// zero and `S_u` is not. The tube windows are cut below the span,
        /// and `a − c`, `b − c` do not round exactly, so the cut box alone
        /// reads `S_v` as a few ulps around zero rather than zero; met with
        /// the whole cell's exact zero, the stretch along e⊥ = `e_v` is
        /// zero again and the probe names the degeneracy rather than
        /// reporting a margin of 0.
        #[test]
        fn a_chart_constant_across_the_locus_refuses_on_a_net_that_is_not_one_point() {
            let a = Point3::new(iv(0.1), iv(0.1), iv(0.1));
            let b = Point3::new(iv(0.7), iv(0.7), iv(0.7));
            let ridge = NurbsSurface::new(
                geom_core::spline::Knots::from(linear_kv()).lift(geom_core::Real::from_f64),
                geom_core::spline::Knots::from(linear_kv()).lift(geom_core::Real::from_f64),
                vec![a, a, b, b],
                vec![geom_core::Real::one(); 4],
            )
            .expect("a patch a caller can build");
            let normal = Vec3::new(iv(0.0), iv(2.0), iv(0.0));
            let verdict = probe_tube_chart(
                &u_line(),
                &NurbsBoxes::new(&ridge),
                (on_line(), normal),
                (0.0, (0.01, 0.01)),
                at_ends(),
            );
            assert!(
                matches!(
                    verdict,
                    Err(SsiError::TubeDegenerate(
                        TubeDegeneracy::WallConstantAcrossLocus
                    ))
                ),
                "a chart constant along v answered {verdict:?} instead of refusing by name"
            );
        }

        /// **A pcurve with no tangent refuses by name, at the first
        /// rung.** A linear pcurve whose two control points coincide has
        /// a zero tangent everywhere; the tangent is the pcurve's whatever
        /// the pad, so trying the next rung reads the same zero.
        #[test]
        fn a_pcurve_with_no_tangent_refuses_rather_than_trying_a_smaller_rung() {
            let still = NurbsCurve2::new(
                geom_core::spline::Knots::from(linear_kv()).lift(geom_core::Real::from_f64),
                vec![Point2::new(iv(0.5), iv(0.5)), Point2::new(iv(0.5), iv(0.5))],
                vec![geom_core::Real::one(); 2],
            )
            .expect("a pcurve a caller can build");
            let normal = Vec3::new(iv(0.0), iv(2.0), iv(0.0));
            let verdict = probe_tube_chart(
                &still,
                &NurbsBoxes::new(&unit_patch()),
                (on_line(), normal),
                (0.0, (0.01, 0.01)),
                at_ends(),
            );
            assert!(
                matches!(
                    verdict,
                    Err(SsiError::TubeDegenerate(
                        TubeDegeneracy::PcurveTangentUnusable
                    ))
                ),
                "a pcurve with no tangent answered {verdict:?} instead of refusing by name"
            );
        }
    }

    /// A degree-2 carrier on `[0, 1]` with the given interior knots.
    #[allow(clippy::unwrap_used)]
    fn carrier(interior: &[f64]) -> geom::NurbsCurve3<f64> {
        use geom_core::Point3;
        use geom_core::spline::KnotVector;
        let mut knots = vec![0.0, 0.0, 0.0];
        knots.extend_from_slice(interior);
        knots.extend([1.0, 1.0, 1.0]);
        let kv = KnotVector::clamped(knots, 2).unwrap();
        let n = kv.control_count();
        #[allow(clippy::cast_precision_loss)]
        let control = (0..n).map(|i| Point3::new(i as f64, 0.0, 0.0)).collect();
        geom::NurbsCurve3::new(
            geom_core::spline::Knots::from(kv).lift(geom_core::Real::from_f64),
            control,
            vec![geom_core::Real::one(); n],
        )
        .unwrap()
    }

    /// `cert_grid` is the DOMAIN's 32nds, a grid point skipped up to
    /// and including `GRID_CLEARANCE` of the spacing (`2⁻¹³`) from a
    /// knot: `0.5` is a knot, and `2/32` is skipped beside a knot
    /// exactly `2⁻¹³` above it, while a knot one ulp further than that
    /// above `12/32` leaves `12/32` standing.
    #[test]
    fn cert_grid_skips_a_grid_point_up_to_the_clearance_from_a_knot() {
        let c = geom_core::spline::algebra::grid_clearance(0.0, 1.0, super::SSI_CERT_SPANS);
        let near = 0.0625 + c;
        let clear = (0.375 + c).next_up();
        let grid = super::cert_grid(carrier(&[near, clear, 0.5]).knots_f64_interim());
        let want: Vec<f64> = (1..32)
            .filter(|&k| k != 2 && k != 16)
            .map(|k| f64::from(k) / 32.0)
            .collect();
        assert_eq!(grid, want);
    }

    /// The box chain has no cliff at any distance of a stated
    /// knot from a grid point: a degree-1 carrier bent at every offset
    /// of [`crate::grid_offsets::knot_offsets`] from the second point of
    /// the [`super::SSI_CERT_SPANS`] grid gives every
    /// box the direction of the leg it lies on as its axis, to `1e-10`,
    /// and at least one box per grid span.
    #[test]
    #[allow(clippy::unwrap_used)]
    fn the_refine_grid_box_axes_have_no_cliff_at_any_knot_offset() {
        use geom_core::spline::KnotVector;
        use geom_core::{Bounds, Point3};
        let bend = [0.9, 0.1, 0.3];
        let unit = |d: [f64; 3]| {
            let n = d.iter().map(|x| x * x).sum::<f64>().sqrt();
            d.map(|x| x / n)
        };
        let legs = [unit(bend), unit([0.1, 0.9, 0.7])];
        let (_, offsets) = crate::grid_offsets::knot_offsets(super::SSI_CERT_SPANS, 2);
        let rows: Vec<(String, f64, usize)> = offsets
            .into_iter()
            .map(|(label, k)| {
                let kv = KnotVector::clamped(vec![0.0, 0.0, k, 1.0, 1.0], 1).unwrap();
                let control = vec![
                    Point3::new(0.0, 0.0, 0.0),
                    Point3::new(bend[0], bend[1], bend[2]),
                    Point3::new(1.0, 1.0, 1.0),
                ];
                let curve = geom::NurbsCurve3::new(
                    geom_core::spline::Knots::from(kv).lift(geom_core::Real::from_f64),
                    control,
                    vec![geom_core::Real::one(); 3],
                )
                .unwrap();
                let chain = super::box_chain(&curve);
                let worst = chain
                    .iter()
                    .map(|(b, axis, _)| {
                        let leg = legs[usize::from(b.x.hi() > bend[0])];
                        (axis.x - leg[0])
                            .abs()
                            .max((axis.y - leg[1]).abs())
                            .max((axis.z - leg[2]).abs())
                    })
                    // A NaN axis (a zero tangent) must surface, not fold away.
                    .fold(0.0, |a: f64, e| if e.is_nan() || e > a { e } else { a });
                (label, worst, chain.len())
            })
            .collect();
        let table: String = rows
            .iter()
            .map(|(label, worst, n)| {
                format!("\n  {label:>26}: axis error {worst:.4e} over {n} boxes")
            })
            .collect();
        for (label, worst, n) in &rows {
            assert!(
                *n >= super::SSI_CERT_SPANS,
                "knot {label}: {n} boxes{table}"
            );
            assert!(*worst < 1e-10, "knot {label}: an axis left its leg{table}");
        }
    }

    /// `chart_breaks` skips a grid point up to the clearance (`2⁻¹³`,
    /// `GRID_CLEARANCE` of the spacing) from a knot of either curve: a
    /// carrier knot that far above `2/32` drops `2/32`, a pcurve knot
    /// that far below `12/32` drops `12/32`, and a pcurve knot one ulp
    /// further below `20/32` leaves it standing with every other 32nd.
    #[test]
    fn chart_breaks_skip_a_grid_point_up_to_the_clearance_from_either_curves_knot() {
        let c = geom_core::spline::algebra::grid_clearance(0.0, 1.0, super::SSI_CERT_SPANS);
        let above = 0.0625 + c;
        let below = 0.375 - c;
        let past = (0.625 - c).next_down();
        let breaks = super::chart_breaks(
            carrier(&[above]).knots_f64_interim(),
            carrier(&[below, past]).knots_f64_interim(),
        );
        let want: Vec<f64> = (1..32)
            .filter(|&k| k != 2 && k != 12)
            .map(|k| f64::from(k) / 32.0)
            .collect();
        assert_eq!(breaks, want);
    }

    /// **`refined_net` encloses the exact refined net** of the described
    /// carrier. A rational cubic on non-dyadic knots, controls and
    /// weights, refined at [`super::cert_grid`]: every point contains the
    /// exact quotient of the refined homogeneous channels, by Boehm's
    /// insertion in exact rationals (`crate::exact_q`). The contrast arm
    /// refines the same carrier at `f64` (`refine_knots`) and lifts it
    /// thin, which misses the exact net where an insertion rounds: the
    /// shape this row is red for. Two coordinate channels, or a zero
    /// weight, refuse the net; a channel of the wrong length refuses its
    /// own entries.
    #[test]
    #[allow(clippy::unwrap_used, clippy::expect_used)]
    fn refined_net_encloses_the_exact_refined_net() {
        use geom::NurbsCurve3;
        use geom_core::spline::KnotVector;
        use geom_core::{Bounds, Interval, Point3};

        use crate::exact_q::{Q, refine_u};

        let knots = vec![0.0, 0.0, 0.0, 0.0, 0.3, 0.7, 1.0, 1.0, 1.0, 1.0];
        let kv = KnotVector::clamped(knots.clone(), 3).unwrap();
        let ctl = [
            [0.1, 0.2, 0.3],
            [0.9, 1.3, -0.3],
            [1.7, -0.6, 0.4],
            [2.3, 0.7, 1.3],
            [3.1, 0.1, 0.7],
            [3.7, 0.9, -0.1],
        ];
        let weights = vec![1.0, 0.3, 2.7, 0.6, 1.1, 0.9];
        let thin = |x: f64| Interval::from_bounds(x, x);
        let coords: Vec<Vec<Interval>> = (0..3)
            .map(|d| ctl.iter().map(|p| thin(p[d])).collect())
            .collect();
        let (fine, net) = super::refined_net(&kv, &weights, &coords).unwrap();
        let breaks = super::cert_grid(&kv);
        let exact: Vec<[Q; 4]> = ctl
            .iter()
            .zip(&weights)
            .map(|(p, &w)| {
                let w = Q::of(w);
                let [x, y, z] = p.map(|c| w.mul(&Q::of(c)));
                [x, y, z, w]
            })
            .collect();
        let (exact, exact_knots) = refine_u(&exact, (ctl.len(), 1), (&knots, 3), &breaks);
        assert_eq!(exact_knots, fine.knots());
        let misses = |pts: &dyn Fn(usize, usize) -> Interval| -> Vec<String> {
            let mut out = Vec::new();
            for (k, h) in exact.iter().enumerate() {
                for c in 0..3 {
                    let p = pts(k, c);
                    if !h[c].div(&h[3]).within(p.lo(), p.hi()) {
                        out.push(format!("{k}.{c}"));
                    }
                }
            }
            out
        };
        let missed = misses(&|k, c| net[c][k]);
        assert!(
            missed.is_empty(),
            "refined_net missed the exact net at {missed:?}"
        );
        let curve = NurbsCurve3::new(
            geom_core::spline::Knots::from(kv.clone()).lift(geom_core::Real::from_f64),
            ctl.iter().map(|p| Point3::new(p[0], p[1], p[2])).collect(),
            weights.clone(),
        )
        .unwrap();
        let lifted = curve.refine_knots(&breaks).unwrap();
        let at = |k: usize, c: usize| {
            let p = lifted.control()[k];
            thin([p.x, p.y, p.z][c])
        };
        assert!(
            !misses(&at).is_empty(),
            "the f64-refined net must miss the exact one somewhere, or this row cannot tell \
             the two apart"
        );
        assert!(super::refined_net(&kv, &weights, &coords[..2]).is_none());
        // A refined weight not certainly positive refuses the net.
        let mut zero = weights.clone();
        zero[0] = 0.0;
        assert!(super::refined_net(&kv, &zero, &coords).is_none());
        // A channel of the wrong length comes back refused, entry by entry.
        let mut short = coords.clone();
        short[1].pop();
        let (_, net) = super::refined_net(&kv, &weights, &short).unwrap();
        assert!(net[1].iter().all(|x| !x.is_certified()));
        assert!(net[0].iter().all(|x| x.is_certified()));
    }

    /// **The tube's box chain encloses the described carrier, at `f64`
    /// and at `Interval`.** The rational cubic of
    /// `refined_net_encloses_the_exact_refined_net`: each box of
    /// [`super::box_chain`] contains the exact refined control points of
    /// its span (Boehm's insertion in exact rationals), so by the convex
    /// hull property it contains the carrier's own arc over the span.
    /// A chain read off the `f64` insertion plan (`refine_knots`) misses
    /// some extremal point by its rounding.
    #[test]
    #[allow(clippy::unwrap_used)]
    fn the_box_chain_encloses_the_exact_refined_net_at_every_scalar() {
        use geom::NurbsCurve3;
        use geom_core::spline::KnotVector;
        use geom_core::{Bounds, Interval, Point3};

        use crate::exact_q::{Q, refine_u};
        use crate::ssi::enclose::Box3;

        let knots = vec![0.0, 0.0, 0.0, 0.0, 0.3, 0.7, 1.0, 1.0, 1.0, 1.0];
        let kv = KnotVector::clamped(knots.clone(), 3).unwrap();
        let ctl = [
            [0.1, 0.2, 0.3],
            [0.9, 1.3, -0.3],
            [1.7, -0.6, 0.4],
            [2.3, 0.7, 1.3],
            [3.1, 0.1, 0.7],
            [3.7, 0.9, -0.1],
        ];
        let weights = vec![1.0, 0.3, 2.7, 0.6, 1.1, 0.9];
        let curve = NurbsCurve3::new(
            geom_core::spline::Knots::from(kv.clone()).lift(geom_core::Real::from_f64),
            ctl.iter().map(|p| Point3::new(p[0], p[1], p[2])).collect(),
            weights.clone(),
        )
        .unwrap();
        let exact: Vec<[Q; 4]> = ctl
            .iter()
            .zip(&weights)
            .map(|(p, &w)| {
                let w = Q::of(w);
                let [x, y, z] = p.map(|c| w.mul(&Q::of(c)));
                [x, y, z, w]
            })
            .collect();
        let breaks = super::cert_grid(&kv);
        let (exact, fine) = refine_u(&exact, (ctl.len(), 1), (&knots, 3), &breaks);
        let spans: Vec<usize> = (3..fine.len() - 4)
            .filter(|&i| fine[i] < fine[i + 1])
            .collect();
        let misses = |chain: &[Box3]| -> Vec<String> {
            assert_eq!(chain.len(), spans.len(), "one box per non-empty span");
            let mut out = Vec::new();
            for (bx, &i) in chain.iter().zip(&spans) {
                for (k, h) in exact.iter().enumerate().take(i + 1).skip(i - 3) {
                    for (c, iv) in [bx.x, bx.y, bx.z].into_iter().enumerate() {
                        if !h[c].div(&h[3]).within(iv.lo(), iv.hi()) {
                            out.push(format!("span {i} point {k}.{c}"));
                        }
                    }
                }
            }
            out
        };
        let at_f64: Vec<Box3> = super::box_chain(&curve)
            .into_iter()
            .map(|(b, _, _)| b)
            .collect();
        let at_f64 = misses(&at_f64);
        assert!(at_f64.is_empty(), "f64 chain missed {at_f64:?}");
        let at_interval: Vec<Box3> =
            super::box_chain(&curve.map_scalar(<Interval as geom_core::Real>::from_f64))
                .into_iter()
                .map(|(b, _, _)| b)
                .collect();
        let at_interval = misses(&at_interval);
        assert!(
            at_interval.is_empty(),
            "Interval chain missed {at_interval:?}"
        );
        // Contrast: hulls of the f64-planned refinement, lifted thin.
        let planned = curve.refine_knots(&breaks).unwrap();
        let hull = |xs: &[Point3<f64>], c: usize| {
            let v = xs.iter().map(|p| [p.x, p.y, p.z][c]);
            let lo = v.clone().fold(f64::INFINITY, f64::min);
            let hi = v.fold(f64::NEG_INFINITY, f64::max);
            Interval::from_bounds(lo, hi)
        };
        let planned: Vec<Box3> = spans
            .iter()
            .map(|&i| {
                let net = &planned.control()[i - 3..=i];
                Box3 {
                    x: hull(net, 0),
                    y: hull(net, 1),
                    z: hull(net, 2),
                }
            })
            .collect();
        assert!(
            !misses(&planned).is_empty(),
            "the f64-planned chain must miss the exact net somewhere, or this row cannot tell \
             the two apart"
        );
    }

    /// `cert_grid`'s cut-off: a carrier with `SSI_CERT_SPANS + degree`
    /// control points takes no breaks, one with a control point fewer
    /// still takes the whole grid. The interior knots are odd 128ths,
    /// none on the 32nds grid.
    #[test]
    fn cert_grid_leaves_a_carrier_at_the_cut_off_alone() {
        let odd = |n: i32| -> Vec<f64> { (0..n).map(|j| f64::from(2 * j + 1) / 128.0).collect() };
        let at = carrier(&odd(31));
        assert_eq!(
            at.knots_f64_interim().control_count(),
            super::SSI_CERT_SPANS + 2
        );
        assert!(super::cert_grid(at.knots_f64_interim()).is_empty());
        let below = carrier(&odd(30));
        assert_eq!(below.knots_f64_interim().control_count(), 33);
        assert_eq!(super::cert_grid(below.knots_f64_interim()).len(), 31);
    }

    /// **Each pad widens its own axis.** A one-span linear pcurve whose
    /// hull is `[0.2, 0.6] × [0.4, 0.5]`, padded by different amounts
    /// along `u` and `v`: the window is the hull widened by `pad_u` along
    /// `u` and by `pad_v` along `v`, bit for bit, and its midpoint is the
    /// span's.
    #[test]
    #[allow(clippy::unwrap_used, clippy::expect_used)]
    fn each_pad_widens_its_own_axis_of_the_window() {
        use geom::NurbsCurve2;
        use geom_core::Point2;
        use geom_core::spline::KnotVector;

        use super::chart_tube_windows;

        let pc = NurbsCurve2::new(
            KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
            vec![Point2::new(0.2, 0.4), Point2::new(0.6, 0.5)],
            vec![geom_core::Real::one(); 2],
        )
        .unwrap();
        let (pad_u, pad_v) = (0.125, 0.375);
        let windows = chart_tube_windows(&pc, (pad_u, pad_v)).expect("certified hulls");
        assert_eq!(windows.len(), 1, "one span, one window");
        let w = windows[0];
        let got = [w.rect.u.0, w.rect.u.1, w.rect.v.0, w.rect.v.1, w.mid].map(f64::to_bits);
        let want = [0.2 - pad_u, 0.6 + pad_u, 0.4 - pad_v, 0.5 + pad_v, 0.5].map(f64::to_bits);
        assert_eq!(
            got, want,
            "window {:?} mid {}: u must widen by pad_u and v by pad_v",
            w.rect, w.mid
        );
    }

    /// **The chart tube is recorded per axis, as proved, and accounting
    /// banks the probe's windows.** One plane × NURBS run on an extruded
    /// wall whose section is slower than its 3 m extrusion, refined along
    /// `u` so a tube window narrower than the domain reads narrower
    /// derivative boxes (on a one-span wall the boxes are the span's
    /// whatever the pad, and along an extrusion `S_v` is constant, so
    /// only the `u` pad can move the margin). Per certified branch:
    ///
    /// - the recorded pad is the rung over each axis's OWN speed, bit for
    ///   bit, and the probe re-run at that pad returns the margin and box
    ///   count the certificate holds — while the probe at the max-folded
    ///   pad returns another margin, so the agreement is about the pad;
    /// - the slower axis's pad is wider than the max-folded pad that
    ///   accounting used to bank on both axes;
    /// - the windows accounting banks are [`chart_tube_windows`] at the
    ///   recorded pad, bit for bit.
    ///
    /// ε-invariant: the band is fixed, so the env row does not move it.
    #[test]
    #[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
    fn the_chart_tube_records_its_per_axis_pad_and_accounting_banks_its_windows() {
        use geom::{NurbsSurface, Surface};
        use geom_core::spline::KnotVector;
        use geom_core::{Band, Margin, Point3, Vec3};

        use super::{Box3, Interval, NurbsBoxes, SsiTube, chart_tube_windows, probe_tube_chart};
        use crate::ssi::{ChartedNurbs, SsiDomain, branch_chart_tubes, plane_nurbs_ssi};

        let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
        let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let mut control = Vec::with_capacity(8);
        for (x, y) in [(0.0, 0.0), (0.35, 0.14), (0.70, 0.24), (1.05, 0.30)] {
            control.push(Point3::new(x, y, 0.0));
            control.push(Point3::new(x, y, 3.0));
        }
        let wall = NurbsSurface::new(
            geom_core::spline::Knots::from(ku).lift(geom_core::Real::from_f64),
            geom_core::spline::Knots::from(kv).lift(geom_core::Real::from_f64),
            control,
            vec![geom_core::Real::one(); 8],
        )
        .unwrap()
        .refine_knots_u(&(1..16).map(|k| f64::from(k) / 16.0).collect::<Vec<_>>())
        .unwrap();
        let n = Vec3::new(0.0, 0.25, 1.0);
        let n = n / n.norm();
        let u = Vec3::new(1.0, 0.0, 0.0);
        let u = (u - n * u.dot(n)) / (u - n * u.dot(n)).norm();
        let origin = Point3::new(0.0, 0.0, 0.4);
        let plane = Surface::Plane {
            origin,
            normal: n,
            u_ref: u,
        };
        let domain = SsiDomain {
            center: Point3::new(0.5, 0.0, 0.4),
            half_extent: 2.0,
            extent: 1.5,
            floor_scale: 1.0,
        };
        let band = Band::new(1e-9, 1e-8).unwrap();

        let speeds = ChartedNurbs::mint(&wall).unwrap().speeds();
        let (su, sv) = (speeds.u.get(), speeds.v.get());
        assert!(
            su < sv,
            "FIXTURE: the section is shorter than the 3 m extrusion, so u is the \
             slower axis (su {su:e}, sv {sv:e})"
        );
        let out = plane_nurbs_ssi(&plane, &wall, domain, band).expect("the wall certifies");
        assert!(!out.branches.is_empty(), "FIXTURE: the plane cuts the wall");
        for (i, b) in out.branches.iter().enumerate() {
            let SsiTube::Chart { rung, pad_u, pad_v } = b.certificate.tube else {
                panic!(
                    "branch {i}: the plane × NURBS arm proved {:?}",
                    b.certificate.tube
                );
            };
            let pc = b
                .pcurve_b
                .as_ref()
                .expect("the ℝ⁴ arm fits the wall's pcurve");

            // The recorded pad is the rung over each axis's own speed.
            let want = (rung / su, rung / sv);
            assert_eq!(
                (pad_u.to_bits(), pad_v.to_bits()),
                (want.0.to_bits(), want.1.to_bits()),
                "branch {i}: recorded pad ({pad_u:e}, {pad_v:e}), rung {rung:e} over the \
                 axis speeds gives ({:e}, {:e})",
                want.0,
                want.1
            );
            // ... and it is the pad the probe proved over.
            let geom::Curve3::Nurbs(carrier) = &b.carrier else {
                panic!("branch {i}: a fitted carrier is a NURBS curve");
            };
            let (t0, t1) = carrier.domain();
            let ends = [t0, t1].map(|t| {
                let p = carrier.eval(t);
                let i = |c: f64| Interval::from_bounds(c, c);
                Box3 {
                    x: i(p.x),
                    y: i(p.y),
                    z: i(p.z),
                }
            });
            let ends = (ends, band);
            let probe = probe_tube_chart(
                pc,
                &NurbsBoxes::new(&wall),
                (origin, n),
                (rung, (pad_u, pad_v)),
                ends,
            )
            .unwrap()
            .expect("the recorded pad probes");
            assert_eq!(
                probe.one_arc,
                Some(Ok(())),
                "branch {i}: the recorded pad's chain is not one arc"
            );
            let (margin, boxes) = (probe.margin, probe.boxes);
            let levered = Margin::levered(margin, domain.extent).value();
            assert_eq!(
                (levered.to_bits(), boxes),
                (
                    b.certificate.tube_transversality.to_bits(),
                    b.certificate.tube_boxes
                ),
                "branch {i}: the probe at the recorded pad is not the certificate's"
            );
            let folded = rung / su.max(sv);
            let at_fold = probe_tube_chart(
                pc,
                &NurbsBoxes::new(&wall),
                (origin, n),
                (rung, (folded, folded)),
                ends,
            )
            .unwrap();
            assert!(
                at_fold.is_none_or(|c| c.margin.to_bits() != margin.to_bits()),
                "FIXTURE: branch {i}'s margin does not depend on the pad, so the \
                 agreement above says nothing about which pad was probed"
            );

            // The slower axis is padded wider than the max-fold pad.
            assert!(
                pad_u > folded && pad_v.to_bits() == folded.to_bits(),
                "branch {i}: per-axis pad ({pad_u:e}, {pad_v:e}) against the max-fold \
                 pad {folded:e}: the slower axis (u) must be wider"
            );

            // Accounting banks the probe's windows, bit for bit.
            let probed: Vec<_> = chart_tube_windows(pc, (pad_u, pad_v))
                .expect("a certified pcurve's hulls are certified")
                .into_iter()
                .map(|w| w.rect)
                .collect();
            let banked = branch_chart_tubes(b);
            let bits = |r: &[crate::ssi::exhaust::UvRect]| -> Vec<[u64; 4]> {
                r.iter()
                    .map(|w| [w.u.0, w.u.1, w.v.0, w.v.1].map(f64::to_bits))
                    .collect()
            };
            assert!(
                !banked.is_empty(),
                "branch {i}: accounting banked no window"
            );
            assert_eq!(
                bits(&banked),
                bits(&probed),
                "branch {i}: accounting banked other windows than limb 3 proved over"
            );
        }
    }

    /// A ladder over `extent` 1 at ε = 1e-9 whose first (widest) rung
    /// reads `first` and every narrower one `rest`.
    #[allow(clippy::unwrap_used)]
    fn ladder_of(
        first: (f64, Option<Result<(), super::Shortfall>>),
        rest: (f64, Option<Result<(), super::Shortfall>>),
    ) -> Result<(super::Rung, f64), crate::ssi::SsiError> {
        use super::{Rung, SsiTube, limb_three};
        let band = geom_core::Band::new(1e-9, 1e-8).unwrap();
        let mut widest = true;
        limb_three((1.0, 1.0), band, |radius| {
            let (margin, one_arc) = if widest { first } else { rest };
            widest = false;
            Ok(Some(Rung {
                tube: SsiTube::Spatial { radius },
                margin,
                boxes: 4,
                one_arc,
            }))
        })
    }

    /// **A graph whose one-arc proof did not run is not proved.** Every
    /// rung zero-free with no proof recorded refuses as undecided, never
    /// certifies. Red under the ladder taking a zero-free rung with no
    /// proof as proved.
    #[test]
    fn a_graph_with_no_proof_is_not_proved_one_arc() {
        let got = ladder_of((0.5, None), (0.5, None));
        assert!(
            matches!(
                got,
                Err(crate::ssi::SsiError::TubeNotOneArc {
                    cause: crate::ssi::OneArcRefusal::Undecided(_),
                    ..
                })
            ),
            "{got:?}"
        );
    }

    /// **The narrowest rung speaks.** A wide rung that is a graph but
    /// not one arc, and narrower rungs that all straddle: the straddle is
    /// the more specific reading of the same carrier, and its band
    /// verdict is the refusal.
    #[test]
    #[allow(clippy::unwrap_used)]
    fn a_narrower_straddle_speaks_over_a_wider_rung_not_one_arc() {
        use super::Shortfall;
        let got = ladder_of((0.5, Some(Err(Shortfall::Count(4)))), (0.0, None));
        assert!(
            matches!(got, Err(crate::ssi::SsiError::TubeStraddles { .. })),
            "{got:?}"
        );
    }

    /// The mirror: a wide rung that straddles, and narrower ones that are
    /// graphs but not one arc. The refusal is `TubeNotOneArc`, counting
    /// those rungs, with the narrowest's cause.
    #[test]
    #[allow(clippy::unwrap_used)]
    fn a_narrower_rung_not_one_arc_speaks_over_a_wider_straddle() {
        use super::{OneArcRefusal, Shortfall, tube_ladder};
        let band = geom_core::Band::new(1e-9, 1e-8).unwrap();
        let rungs = tube_ladder(1.0, band).count() as u32;
        let got = ladder_of((0.0, None), (0.5, Some(Err(Shortfall::Unlinked))));
        assert!(
            matches!(
                got,
                Err(crate::ssi::SsiError::TubeNotOneArc {
                    rungs: r,
                    cause: OneArcRefusal::Unlinked,
                }) if r == rungs - 1
            ),
            "{got:?}"
        );
    }

    /// A walk that resolved nothing escalates through the funnel, with
    /// the unreadable margin it read.
    #[test]
    #[allow(clippy::unwrap_used, clippy::panic)]
    fn an_undecided_walk_escalates_through_the_funnel() {
        use super::{OneArcRefusal, Shortfall};
        let got = ladder_of(
            (0.5, Some(Err(Shortfall::Undecided))),
            (0.5, Some(Err(Shortfall::Undecided))),
        );
        let Err(crate::ssi::SsiError::TubeNotOneArc {
            cause: OneArcRefusal::Undecided(cause),
            ..
        }) = got
        else {
            panic!("{got:?}");
        };
        assert!(cause.margin.is_invalid(), "{cause:?}");
    }

    /// **A limb-3 refusal is not located, so refinement leaves it
    /// unwrapped.** The fold at slope `c = 800ε`, gap height β = ½ε:
    /// the straight carrier across its gap passes limbs 1 and 2, and on
    /// the chart search's lane its tube is a graph but not one arc. The
    /// refusal is `TubeNotOneArc` with no located interval, which
    /// [`super::super::refine::refine_by_certificate`] returns as it is
    /// rather than as `RefinementExhausted`.
    /// The fold at slope `c = 800ε`, gap height β = ½ε, and the straight
    /// carrier across its gap with its pcurve: `(wall, carrier, pcurve,
    /// plane)`.
    #[allow(clippy::unwrap_used)]
    fn fold_across_its_gap() -> (
        geom::NurbsSurface<f64>,
        geom::NurbsCurve3<f64>,
        geom::NurbsCurve2<f64>,
        geom::Surface<f64>,
    ) {
        use geom::{NurbsCurve2, NurbsCurve3, NurbsSurface, Surface};
        use geom_core::spline::KnotVector;
        use geom_core::{Point2, Point3, Vec3};

        let beta = 0.5e-9;
        let c = 1600.0 * beta;
        let a = 0.28 * c * c / beta;
        let w = beta / c;
        let l = 1.2 * w;
        let (x0, x1) = (-1.5 * w, 1.8 * w);
        let g = |x: f64| c * x + a * x * x;
        let gb = [g(x0), g(x0) + 0.5 * (x1 - x0) * (c + 2.0 * a * x0), g(x1)];
        let k2 = || KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
        let (xs, ys) = ([x0, 0.5 * (x0 + x1), x1], [0.0, 0.5 * l, l]);
        let control = (0..9)
            .map(|i| {
                Point3::new(
                    xs[i / 3],
                    ys[i % 3],
                    gb[i / 3] + [0.0, 2.0 * beta, 0.0][i % 3],
                )
            })
            .collect();
        let wall = NurbsSurface::new(k2(), k2(), control, vec![geom_core::Real::one(); 9]).unwrap();
        let k1 = || KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let carrier = NurbsCurve3::new(
            k1(),
            vec![Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, l, 0.0)],
            vec![geom_core::Real::one(); 2],
        )
        .unwrap();
        let u0 = -x0 / (x1 - x0);
        let pcurve = NurbsCurve2::new(
            k1(),
            vec![Point2::new(u0, 0.0), Point2::new(u0, 1.0)],
            vec![geom_core::Real::one(); 2],
        )
        .unwrap();
        let plane = Surface::Plane {
            origin: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        (wall, carrier, pcurve, plane)
    }

    #[test]
    #[allow(clippy::unwrap_used, clippy::panic)]
    fn a_limb_three_refusal_on_the_chart_lane_is_not_located() {
        use geom_core::Band;

        use super::{Lane, certify_located};
        use crate::ssi::{ChartedNurbs, SsiError};

        let (wall, carrier, pcurve, plane) = fold_across_its_gap();
        let lane = Lane::Chart {
            plane: &plane,
            wall: ChartedNurbs::mint(&wall).unwrap(),
            pcurve: &pcurve,
        };
        let band = Band::new(1e-9, 1e-8).unwrap();
        let Err(refused) = certify_located(&carrier, lane, 1.0, band, super::Limbs::All) else {
            panic!("the carrier across the gap certified on the search's lane");
        };
        assert!(
            matches!(refused.error, SsiError::TubeNotOneArc { .. }),
            "{:?}",
            refused.error
        );
        assert!(refused.at.is_none(), "limb 3 was located: {:?}", refused.at);
    }

    /// **An operand's cells are built once for its life, not once per
    /// rung**: the mint reads the one-cell fold's speeds, limb 3 walks
    /// its ladder with every rung reading the wall's chart enclosures,
    /// and the cell's net is built once over all of it.
    #[test]
    #[allow(clippy::unwrap_used)]
    fn every_rung_reads_the_operands_one_build_of_each_cell() {
        use geom_core::Band;

        use super::{Lane, certify_located};
        use crate::ssi::ChartedNurbs;
        use crate::ssi::enclose::tests::WHOLE_NETS;

        let (wall, carrier, pcurve, plane) = fold_across_its_gap();
        WHOLE_NETS.with(|n| n.set(0));
        CHART_PROBES.with(|n| n.set(0));
        let wall_op = ChartedNurbs::mint(&wall).unwrap();
        let lane = Lane::Chart {
            plane: &plane,
            wall: wall_op,
            pcurve: &pcurve,
        };
        let band = Band::new(1e-9, 1e-8).unwrap();
        let _ = certify_located(&carrier, lane, 1.0, band, super::Limbs::Tube);
        let rungs = CHART_PROBES.with(core::cell::Cell::get);
        let built = WHOLE_NETS.with(core::cell::Cell::get);
        assert!(
            rungs >= 2,
            "the ladder probed {rungs} rungs: the row reads nothing"
        );
        assert_eq!(
            built, 1,
            "the one-cell wall's net was built {built} times over {rungs} rungs"
        );
    }

    /// **A refused transversality enclosure is no sliver.** A cone has
    /// no ℝ³ gradient enclosure, so every box of every rung of an
    /// analytic tube against one is refused: the ladder answers nothing
    /// (`TubeProbeSilent`, every rung offered), where reading each
    /// refusal as a zero margin reported a straddle at the narrowest
    /// rung.
    #[test]
    #[allow(clippy::unwrap_used)]
    fn a_tube_whose_enclosures_are_refused_answers_no_rung() {
        use super::{Lane, Limbs, Refused, certify_branch, tube_ladder};
        use crate::ssi::{SsiError, SsiOperand};
        use geom::{NurbsCurve3, Surface};
        use geom_core::spline::KnotVector;
        use geom_core::{Band, Point3, Vec3};
        let plane = Surface::Plane {
            origin: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let cone = Surface::Cone {
            apex: Point3::new(0.0, 0.0, -1.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            half_angle: core::f64::consts::FRAC_PI_4,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let carrier = NurbsCurve3::new(
            KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
            vec![Point3::new(1.0, 0.0, 0.0), Point3::new(1.0, 0.1, 0.0)],
            vec![1.0; 2],
        )
        .unwrap();
        let band = Band::new(1e-9, 1e-8).unwrap();
        let got = certify_branch(
            &geom::Curve3::Nurbs(std::sync::Arc::new(carrier.clone())),
            carrier.domain(),
            Lane::AtRest {
                a: &SsiOperand::Analytic(&plane),
                b: &SsiOperand::Analytic(&cone),
                pcurve_a: None,
                pcurve_b: None,
            },
            0.1,
            band,
            Limbs::Tube,
            &mut Refused::default(),
        )
        .map(|_| ());
        #[allow(clippy::cast_possible_truncation)]
        let rungs = tube_ladder(0.1, band).count() as u32;
        assert!(rungs > 0, "the ladder offers rungs");
        assert!(
            matches!(
                got,
                Err(SsiError::TubeProbeSilent { rungs: r, refused }) if r == rungs && refused == rungs
            ),
            "{got:?}"
        );
    }

    /// **A tube reads a refused enclosure as no reading**, a straddling
    /// one as zero, and a clear one as its distance from zero.
    #[test]
    fn the_tube_margin_tells_a_refusal_from_a_straddle() {
        use super::tube_margin;
        use geom_core::Interval;
        use geom_core::interval::certification::Certification;
        assert_eq!(tube_margin(Interval::refused()), None, "refused");
        assert_eq!(
            tube_margin(Interval::from_bounds(-1.0, 2.0)),
            Some(0.0),
            "straddle"
        );
        assert_eq!(
            tube_margin(Interval::from_bounds(-3.0, -0.5)),
            Some(0.5),
            "clear"
        );
    }

    /// **Limb 2's bound escalates at `Interval` with its margin as an
    /// enclosure** (4553's second review, MINOR-B): in band, the
    /// escalation names the arm's bound predicate and carries the bound
    /// as an `Interval`, both ends the bound's own bits, and the error
    /// text renders an enclosure. Red if the margin is flattened to one
    /// `f64` (`Margin::of(hull.sup)`), which reads and renders as a
    /// value. Driven at `hull_limb` itself: on the analytic pair the
    /// SSI door reaches, limb 2 reads under limb 1 under the monotone
    /// step (`review_ssiflat_r2_probes`' probe 5), so no band there
    /// lets limb 2 escalate first.
    #[test]
    #[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
    fn the_hull_bound_escalates_with_an_enclosure_at_interval() {
        use super::{ANALYTIC_HULL, CHART_HULL, Hull, SsiError, Subdivided, hull_limb};
        use geom_core::{Band, ErrorTextReading, Interval, MarginDiag};
        let band = Band::new(4.0e-15, 6.0e-15).unwrap();
        let sup = 5.0e-15;
        for arm in [&ANALYTIC_HULL, &CHART_HULL] {
            let hull = Hull {
                sup,
                breaks: vec![0.0, 1.0],
                spans: vec![sup],
                floors: vec![0.0; 2],
                work: 1,
                data: (),
            };
            let mut at = super::Refused::default();
            let err = hull_limb::<Interval>(arm, Subdivided::Bound(hull), band, &mut at)
                .expect_err("a bound in the band escalates");
            let SsiError::CertificateEscalated { ref cause, .. } = err else {
                panic!("{}: not the escalation: {err:?}", arm.sup);
            };
            assert_eq!(cause.predicate, Some(arm.sup));
            let ErrorTextReading::Enclosure { lo, hi } =
                MarginDiag::diagnostic_f64_for_error_text(cause.margin)
            else {
                panic!(
                    "{}: limb 2's margin at Interval must read as an enclosure: {:?}",
                    arm.sup, cause.margin
                );
            };
            assert_eq!(
                (lo.to_bits(), hi.to_bits()),
                (sup.to_bits(), sup.to_bits()),
                "{}: the escalation must carry the bound's own ends",
                arm.sup
            );
            let shown = err.to_string();
            assert!(
                shown.contains("enclosure [") && !shown.contains("NaN"),
                "{}: the text must render the enclosure: {shown}",
                arm.sup
            );
            assert_eq!(
                at.spans.len(),
                1,
                "{}: the uncleared span is located",
                arm.sup
            );
        }
    }

    /// Limb 2's torus readings of `carrier` over its spans cut at
    /// `breaks`: the composite's breaks, its sup, each span's sup and
    /// each break's floor, all in meters.
    #[allow(clippy::unwrap_used)]
    fn torus_readings(
        carrier: &geom::NurbsCurve3<f64>,
        torus: &geom::Surface<f64>,
        breaks: &[f64],
    ) -> (Vec<f64>, f64, Vec<f64>, Vec<f64>) {
        use geom_core::spline::compose::{self, CurveCertData};
        let curve = geom::Curve3::Nurbs(std::sync::Arc::new(carrier.clone()));
        let net = crate::carrier_net::carrier_net(&curve, carrier.domain(), breaks).unwrap();
        let (ck, cw, cc) = net.operand();
        let data = CurveCertData::new(ck, cw, cc)
            .unwrap()
            .with_breaks(net.cuts());
        let (form, to) = super::composite_form(torus).unwrap();
        let composite = compose::implicit_composite(&data, &form).unwrap();
        let (sup, spans, floors) = super::meters_readings(&composite, &data, to).unwrap();
        let at = composite.num.breaks().to_vec();
        assert_eq!(
            spans.len() + 1,
            at.len(),
            "one span between each two breaks"
        );
        assert_eq!(floors.len(), at.len(), "one floor per break");
        (at, sup, spans, floors)
    }

    /// **Limb 2's torus conversion bounds every sampled residual**,
    /// counterexample search: cubic carriers whose controls lie on a
    /// torus (thin to fat, horn and spindle, tilted; a span the
    /// conversion refuses reads `NaN`, which bounds nothing) displaced by up to a tenth of the
    /// tube, read over eight spans. Each span's bound holds the meters
    /// residual `implicit_residual` at points sampled in it, and each
    /// break's floor is no more than the residual there: a conversion
    /// that divided by too much, or floored by too little, reads red.
    #[test]
    #[allow(clippy::unwrap_used)]
    fn a_torus_carriers_limb_two_holds_its_sampled_residual() {
        use crate::ssi::enclose::tests::{on_torus, torus};
        use geom::NurbsCurve3;
        use geom_core::spline::KnotVector;
        use geom_core::{Point3, Vec3};
        use test_utils::fuzz;
        let mut rng = fuzz::start("torus limb 2");
        for _ in 0..fuzz::scaled(40) {
            let small = rng.log_uniform(1e-3, 1.0);
            let big = small * rng.range(0.5, 30.0);
            let c = [
                rng.range(-1.0, 1.0),
                rng.range(-1.0, 1.0),
                rng.range(-1.0, 1.0),
            ];
            let torus = torus(
                c,
                [rng.range(-1.0, 1.0), rng.range(-1.0, 1.0), 0.6],
                big,
                small,
            );
            let (u0, v0) = (rng.range(0.0, 6.3), rng.range(0.0, 6.3));
            let (du, dv) = (rng.range(-0.6, 0.6), rng.range(-2.0, 2.0));
            let lift = small * rng.log_uniform(1e-9, 0.1);
            let points: Vec<Point3<f64>> = (0..6)
                .map(|i| {
                    let s = f64::from(i) / 5.0;
                    let off = Vec3::new(
                        rng.range(-1.0, 1.0),
                        rng.range(-1.0, 1.0),
                        rng.range(-1.0, 1.0),
                    );
                    on_torus(&torus, u0 + du * s, v0 + dv * s) + off * lift
                })
                .collect();
            let knots = KnotVector::clamped(
                vec![0.0, 0.0, 0.0, 0.0, 1.0 / 3.0, 2.0 / 3.0, 1.0, 1.0, 1.0, 1.0],
                3,
            )
            .unwrap();
            let carrier = NurbsCurve3::new(knots, points, vec![1.0; 6]).unwrap();
            let cuts: Vec<f64> = (1..8).map(|k| f64::from(k) / 8.0).collect();
            let (at, sup, spans, floors) = torus_readings(&carrier, &torus, &cuts);
            let ctx = format!("{torus:?}, lift {lift:e}: {}", fuzz::replay());
            assert!(
                sup.is_finite() || big <= small,
                "a ring torus's bound: {ctx}"
            );
            let residual =
                |t: f64| crate::implicit::implicit_residual(&torus, carrier.eval(t)).abs();
            let slack = |x: f64| x * (1.0 + 1e-9) + 1e-15 * big;
            for (j, span) in spans.iter().enumerate() {
                for k in 0..=16 {
                    let t = at[j] + (at[j + 1] - at[j]) * f64::from(k) / 16.0;
                    let m = residual(t);
                    assert!(
                        span.is_nan() || m <= slack(*span),
                        "span {j} at {t}: residual {m:e} over its bound {span:e}; {ctx}"
                    );
                }
            }
            for (j, floor) in floors.iter().enumerate() {
                let m = residual(at[j]);
                assert!(
                    *floor <= slack(m),
                    "break {j}: floor {floor:e} over the residual {m:e}; {ctx}"
                );
            }
        }
    }
}
