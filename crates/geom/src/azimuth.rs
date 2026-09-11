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
//!
//! [`frame`] answers an [`AzimuthFrame`] of [`Radial`] and
//! [`Tangential`], and the reason is the arms: they ask for the radial
//! alone, for the tangential alone, and for both, so a positional pair
//! of two same-typed vectors has to be destructured three ways and a
//! transposed one compiles, runs, and moves geometry.
//!
//! **Names alone would not have fixed that** — while the two fields
//! share a type, `AzimuthFrame { radial: t, tangential: r }` compiles.
//! The newtypes make the exchange E0308 **wherever the two cross a
//! typed boundary**: the constructor here, and any site that passes one
//! where the other is expected. What no newtype can stop, and what is
//! therefore stated rather than claimed away, is a local binding
//! deliberately named for the other value and unwrapped on the spot
//! (`let AzimuthFrame { radial: tangential, .. }`); that residue is one
//! visible line at the site, and the downstream suites named on the row
//! below are its evidence. `.0` unwraps the vector where an arm's
//! arithmetic needs it.

use geom_core::{Real, Vec3};

/// `((sin u, cos u), v_ref)` — the trig pair from **one** `sin_cos`
/// call and the third frame vector `v_ref = axis × u_ref`.
pub(crate) fn basis<T: Real>(axis: Vec3<T>, u_ref: Vec3<T>, u: T) -> ((T, T), Vec3<T>) {
    ((u.sin_cos()), axis.cross(u_ref))
}

/// The unit vector at an azimuth: `u_ref·c + v_ref·s`. A newtype
/// rather than a bare `Vec3<T>` so that it and [`Tangential`] cannot be
/// exchanged (module docs).
#[derive(Clone, Copy, Debug)]
pub(crate) struct Radial<T: Real>(pub(crate) Vec3<T>);

/// The derivative of [`Radial`] in the azimuth: `u_ref·(−s) + v_ref·c`.
/// Distinct from it by type, for the reason on [`Radial`].
#[derive(Clone, Copy, Debug)]
pub(crate) struct Tangential<T: Real>(pub(crate) Vec3<T>);

/// The azimuthal frame at one angle: the unit vector at that azimuth
/// and its derivative, each in its own type.
#[derive(Clone, Copy, Debug)]
pub(crate) struct AzimuthFrame<T: Real> {
    /// The unit vector at azimuth `u`.
    pub radial: Radial<T>,
    /// Its derivative in `u`.
    pub tangential: Tangential<T>,
}

/// The azimuthal frame at angle `u`, from one `sin_cos` call, with the
/// associations exactly as written (D9); the two formulas are on
/// [`AzimuthFrame`]'s fields.
pub(crate) fn frame<T: Real>(axis: Vec3<T>, u_ref: Vec3<T>, u: T) -> AzimuthFrame<T> {
    let ((s, c), v_ref) = basis(axis, u_ref, u);
    AzimuthFrame {
        radial: Radial(u_ref * c + v_ref * s),
        tangential: Tangential(u_ref * (-s) + v_ref * c),
    }
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
    /// **What it does not have to pin, and what it still does.**
    /// Exchanging the two across a typed boundary is E0308 since they
    /// became [`Radial`] and [`Tangential`], so the shape the module
    /// used to concede is gone from every site that passes or
    /// constructs them. A binding renamed at the destructure and
    /// unwrapped immediately still compiles (module docs), and THAT
    /// residue is what stays covered indirectly, downstream, by
    /// the derivative-vs-dual rows in
    /// `tests/curves/nurbs_differential.rs`, the hand-computed
    /// partials in `tests/surfaces/review_m2_pr1.rs` and the
    /// locus/periodicity property rows on each enum. Those suites are
    /// still this body's downstream evidence; they are no longer its
    /// only guard against a transposition.
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

            let f = frame(axis, u_ref, u);
            assert_eq!(
                bits(f.radial.0),
                bits(u_ref * c + v_ref * s),
                "radial at {u}"
            );
            assert_eq!(
                bits(f.tangential.0),
                bits(u_ref * (-s) + v_ref * c),
                "tangential at {u}"
            );
        }
    }

    /// CERT-N2 R1 reviewer probe, adopted with its **expectation
    /// updated by the fix pass**, which is the record of what moved.
    /// As the reviewer wrote it the row asserted that a transposition
    /// still COMPILES — true then, because both fields were `Vec3<T>`,
    /// and the falsification of the claim that they could not be
    /// exchanged. Since [`Radial`] and [`Tangential`] are distinct
    /// types the struct-literal swap no longer compiles at all (the
    /// two arms of it are E0308 in both directions, which is why they
    /// cannot be written here). What SURVIVES the newtypes is the
    /// half the module docs now name out loud: a destructure that
    /// renames the fields and unwraps them on the spot still compiles,
    /// because after `.0` both are vectors again.
    #[test]
    fn probe_the_renaming_destructure_is_the_residue_the_types_do_not_close() {
        let axis = Vec3::new(0.0, 0.0, 1.0);
        let u_ref = Vec3::new(1.0, 0.0, 0.0);
        let f = frame(axis, u_ref, 0.3f64);
        let AzimuthFrame {
            radial: tangential,
            tangential: radial,
        } = frame(axis, u_ref, 0.3f64);
        assert_eq!(radial.0.x, f.tangential.0.x);
        assert_eq!(tangential.0.x, f.radial.0.x);
    }
}
