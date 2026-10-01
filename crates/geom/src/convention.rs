//! **The datum conventions a stored analytic datum must lie inside**, as
//! quantities: the margin type both representability doors return
//! ([`crate::Surface::representability_margins`],
//! [`crate::Curve3::representability_margins`]) and the one frame
//! reading both kinds share ([`frame_margins`]).
//!
//! Nothing here decides. Each margin is strictly positive exactly when
//! its datum is inside that end of its convention; whether it is, and
//! with what posture, is the consumer's question (`topo`'s tier-3
//! check 1 reads each one's lower bracket end against zero).

use geom_core::{Band, Real, Vec3};

use crate::SurfaceDatum;

/// One representability margin: the datum it bounds, which quantity of
/// the datum it measures, which end of that quantity's convention, and
/// the margin itself — strictly positive exactly when the datum is
/// inside that end. `D` is [`SurfaceDatum`] for a surface and
/// [`crate::CurveDatum`] for a carrier.
#[derive(Clone, Copy, Debug)]
pub struct RepresentabilityMargin<T, D = SurfaceDatum> {
    /// The datum the margin bounds.
    pub datum: D,
    /// Which quantity of the datum the margin measures.
    pub measure: ConventionMeasure,
    /// Which end of that quantity's convention the margin measures.
    pub end: ConventionEnd,
    /// The margin itself, at the datum's scalar.
    pub margin: T,
}

/// Which quantity of a datum a convention constrains — the refusal's
/// way of saying WHAT is wrong before it says which way.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
// Every value, for `topo`'s samples (this crate's `test-support`
// feature, test builds only).
#[cfg_attr(feature = "test-support", derive(strum::EnumIter))]
pub enum ConventionMeasure {
    /// The scalar datum itself: a radius, a semi-axis, a half-angle.
    Value,
    /// A stored direction's length, which the frame convention makes 1.
    Length,
    /// A seam direction's component along the axis (`axis · u_ref`),
    /// which the frame convention makes 0.
    Tilt,
}

/// Which end of a datum's convention a margin measures — the refusal's
/// way of saying TOO SMALL (a radius of zero, a cone closed to a line,
/// a direction short of unit, a `u_ref` leaning toward `−axis`) from
/// TOO LARGE (a cone opened to a plane, a direction past unit, a
/// `u_ref` leaning toward `+axis`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
// Every value, for `topo`'s samples (this crate's `test-support`
// feature, test builds only).
#[cfg_attr(feature = "test-support", derive(strum::EnumIter))]
pub enum ConventionEnd {
    /// The quantity must lie above this end (`radius > 0`,
    /// `half_angle > 0`, `‖axis‖ > 1 − ·`).
    Lower,
    /// The quantity must lie below this end (`half_angle < π/2`,
    /// `‖axis‖ < 1 + ·`).
    Upper,
}

/// A value convention's margin at its lower end: the datum itself,
/// positive exactly when the datum is.
pub(crate) fn lower<T, D>(datum: D, margin: T) -> RepresentabilityMargin<T, D> {
    RepresentabilityMargin {
        datum,
        measure: ConventionMeasure::Value,
        end: ConventionEnd::Lower,
        margin,
    }
}

/// **The frame's representability margins** — `axis` unit, `u_ref`
/// unit and `u_ref ⊥ axis`, each at both ends, as ε-slack quantities at
/// the kind's lever `arm` (metres):
///
/// ```text
/// ε − (‖axis‖ − 1)·arm      ε − (1 − ‖axis‖)·arm       (axis,  Length, Upper / Lower)
/// ε − (‖u_ref‖ − 1)·arm     ε − (1 − ‖u_ref‖)·arm      (u_ref, Length, Upper / Lower)
/// ε − (axis · u_ref)·arm    ε + (axis · u_ref)·arm     (u_ref, Tilt,   Upper / Lower)
/// ```
///
/// with ε the band's coincidence threshold ([`Band::zero`]). Each is
/// strictly positive exactly when the frame is inside its convention
/// to within ε of movement at the arm.
///
/// **What the frame convention protects.** Every axisymmetric evaluator
/// reads the frame through `azimuth::frame` —
/// `radial(u) = u_ref·cos u + v_ref·sin u`, `v_ref = axis × u_ref` —
/// while the implicit forms and the section arms read `axis` and the
/// radius as the geometric axis and radius.
///
/// - **Length, every kind: the locus.** A `u_ref` of length `1 + δ`
///   evaluates a cylinder, sphere, torus or circle of radius `r(1 + δ)`;
///   an `axis` of length `1 + δ` stretches `v_ref` and evaluates an
///   elliptic section. Two consumers read two loci off one datum, and
///   the movement is `δ` times the kind's largest radius — the `arm`.
/// - **Tilt, sphere, torus and every carrier: the locus.** A `u_ref`
///   with an axial component `c` gives `radial(u)` an axial component
///   `c·cos u`, which lifts the sphere's and torus's points off their
///   surface and a circle's, ellipse's or spiric's out of its plane by
///   `c` times the arm, to first order.
/// - **Tilt, cylinder: the PARAMETERIZATION, not the locus.** On a
///   cylinder the axial component slides a point along its ruling, so
///   the locus moves only `r·(1 − √(1 − c²)) ≈ r·c²/2` — second order,
///   and not what this margin bounds. What moves at first order is the
///   point at a FIXED `(u, v)`: `S(u, v)` shifts by `r·c·cos u` along the
///   axis, and every chart-described edge, stored pcurve and chart
///   window reads the surface at fixed `(u, v)`. The margin is kept on
///   the cylinder for that reason — a frame whose chart has drifted
///   first-order off its implicit form is the defect the chart
///   consumers cannot see — and it is the one tilt margin that guards
///   the chart rather than the locus.
///
/// **With the band, and not an exact compare**: a frame minted by
/// arithmetic — a rotation, a normalization, a cross product — is unit
/// and orthogonal only to rounding, so an exact compare would refuse
/// every such datum. ε is the run's own threshold below which two
/// points coincide.
///
/// One function for both kinds (the datum type is the caller's), so
/// the surface and carrier frames are read by one expression.
pub(crate) fn frame_margins<T: Real, D: Copy>(
    axis: Vec3<T>,
    u_ref: Vec3<T>,
    arm: T,
    band: Band,
    axis_datum: D,
    u_ref_datum: D,
) -> [RepresentabilityMargin<T, D>; 6] {
    let eps = T::from_f64(band.zero());
    let one = T::one();
    let margin = |datum, measure, end, deviation: T| RepresentabilityMargin {
        datum,
        measure,
        end,
        margin: match end {
            ConventionEnd::Upper => eps - deviation * arm,
            ConventionEnd::Lower => eps + deviation * arm,
        },
    };
    let axis_len = axis.norm() - one;
    let u_ref_len = u_ref.norm() - one;
    let tilt = axis.dot(u_ref);
    use ConventionEnd::{Lower, Upper};
    use ConventionMeasure::{Length, Tilt};
    [
        margin(axis_datum, Length, Upper, axis_len),
        margin(axis_datum, Length, Lower, axis_len),
        margin(u_ref_datum, Length, Upper, u_ref_len),
        margin(u_ref_datum, Length, Lower, u_ref_len),
        margin(u_ref_datum, Tilt, Upper, tilt),
        margin(u_ref_datum, Tilt, Lower, tilt),
    ]
}
