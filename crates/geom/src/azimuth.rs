//! The azimuthal frame: the one reading of `u_ref`, `axis` and an
//! angle that every axisymmetric evaluator in the crate starts from.
//!
//! `DESIGN.md`'s parameterization row ratifies **one** convention here
//! — `v_ref = axis × u_ref`, seam at `u_ref` — and it is shared by the
//! circle and ellipse arms of [`crate::curves::Curve3`] and by every
//! axisymmetric arm of [`crate::surfaces::Surface`]. One convention
//! gets one body: a second derivation of `v_ref` is how the two halves
//! drift.
//!
//! Two doors, because the arms need two different amounts of it. An
//! arm whose combination *is* the radial or the tangential takes
//! [`frame`]; an arm that combines `sin`, `cos` and `v_ref` in its own
//! documented order (the ellipse's per-axis scaling, the circle's
//! second derivative) takes [`basis`] and writes its own combination.
//! Neither door performs a combination on the caller's behalf that the
//! caller's own doc comment does not spell out — the association
//! orders are D9-fixed per arm and stay where they are read.

use geom_core::{Real, Vec3};

/// `((sin u, cos u), v_ref)` — the trig pair from **one** `sin_cos`
/// call and the third frame vector `v_ref = axis × u_ref`.
pub(crate) fn basis<T: Real>(axis: Vec3<T>, u_ref: Vec3<T>, u: T) -> ((T, T), Vec3<T>) {
    ((u.sin_cos()), axis.cross(u_ref))
}

/// The azimuthal frame at angle `u`: `(radial, tangential)` =
/// `(u_ref·c + v_ref·s, u_ref·(−s) + v_ref·c)` from one `sin_cos`
/// call, associations exactly as written (D9).
pub(crate) fn frame<T: Real>(axis: Vec3<T>, u_ref: Vec3<T>, u: T) -> (Vec3<T>, Vec3<T>) {
    let ((s, c), v_ref) = basis(axis, u_ref, u);
    (u_ref * c + v_ref * s, u_ref * (-s) + v_ref * c)
}

#[cfg(test)]
mod tests {
    use geom_core::Real;

    use super::*;

    /// The two doors agree **bit for bit** with the formula
    /// `DESIGN.md`'s parameterization row and the two module headers
    /// state: `radial = u_ref·c + v_ref·s`,
    /// `tangential = u_ref·(−s) + v_ref·c`, `v_ref = axis × u_ref`,
    /// from one `sin_cos`.
    ///
    /// This is the row that makes the shared body safe to share. Every
    /// arm that used to spell the formula inline now reads it from
    /// here, so a "simplification" of these four lines — reassociating
    /// the sum, folding the scale in, computing `v_ref` differently —
    /// would move real geometry while looking like tidying. The
    /// comparison is written out longhand on purpose: it is an
    /// independent transcription of the documented formula, not a call
    /// back into the code under test.
    ///
    /// `Real::sin_cos` is named explicitly because on a bare `f64` the
    /// INHERENT `std` method wins over the trait one and the two differ
    /// by an ulp. Generic bodies only ever see the trait method; this
    /// row must too, or it measures the dispatch rather than the frame.
    ///
    /// **What it does NOT pin: the call sites.** A caller that
    /// destructured [`frame`] the wrong way round — taking the radial
    /// where the tangential belongs — compiles, and this row still
    /// passes. That case is caught, but only INDIRECTLY: by the
    /// derivative-vs-dual rows in `tests/curves/nurbs_differential.rs`,
    /// the hand-computed partials and finite-difference oracles in
    /// `tests/surfaces/review_m2_pr1.rs`, and the locus/periodicity
    /// property rows on each enum. Stated here so that thinning any of
    /// those suites is a visible loss of this refactor's site-level
    /// guard rather than an invisible one.
    #[test]
    fn both_doors_are_bitwise_the_documented_formula() {
        let axis = Vec3::new(0.31, -0.72, 0.61).normalize();
        let u_ref = {
            let raw = Vec3::new(0.83, 0.19, -0.52);
            (raw - axis * raw.dot(axis)).normalize()
        };
        let bits = |v: Vec3<f64>| (v.x.to_bits(), v.y.to_bits(), v.z.to_bits());

        for k in -400i32..400 {
            let u = f64::from(k) * 0.017_37;
            let (s, c) = Real::sin_cos(u);
            let v_ref = axis.cross(u_ref);

            let ((bs, bc), bv) = basis(axis, u_ref, u);
            assert_eq!(bs.to_bits(), s.to_bits(), "basis sin at {u}");
            assert_eq!(bc.to_bits(), c.to_bits(), "basis cos at {u}");
            assert_eq!(bits(bv), bits(v_ref), "basis v_ref at {u}");

            let (radial, tangential) = frame(axis, u_ref, u);
            assert_eq!(bits(radial), bits(u_ref * c + v_ref * s), "radial at {u}");
            assert_eq!(
                bits(tangential),
                bits(u_ref * (-s) + v_ref * c),
                "tangential at {u}"
            );
        }
    }
}
