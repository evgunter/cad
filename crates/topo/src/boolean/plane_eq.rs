//! **Oriented-plane-equality** — the typed replacement for ch. 15's
//! `vecequal` on raw plane 4-vectors (Program 15.10's ⁺/⁻ gate, notes'
//! predicate inventory: "fragile on unnormalized Newell vectors").
//!
//! Verdicts: *same plane, same orientation* / *same plane, opposite
//! orientation* / *different plane* — plus the two typed refusals the
//! coincidence ladder demands. The rungs, in order:
//!
//! 1. **Declared rung (exact)**: canonicalize each plane to
//!    `(n̂, d = n̂·origin)` and compare through the sanctioned
//!    `Real`-level bit door ([`geom_core::bit_identity::eq_bits`]).
//!    Bit-equal ⇒ [`PlaneRelation::SameOriented`]; bit-equal to the
//!    negation (IEEE negation is exact) ⇒
//!    [`PlaneRelation::SameOpposite`]. Cross-body structural
//!    coincidence is impossible (different arenas), so declared
//!    bit-equality is the ONLY rung that can say "same plane".
//! 2. **Geometric trilean (definite-different only)**: parallelism
//!    margin `‖n₁ × n₂‖·arm` (`bool_plane_parallel`) definitely
//!    positive ⇒ [`PlaneRelation::Distinct`]; else offset margin
//!    `(d₁ − σ·d₂)·…` (`bool_plane_offset`, σ the orientation sign)
//!    definitely nonzero ⇒ parallel-but-offset ⇒ `Distinct`.
//! 3. Geometrically coincident-or-near **without** the bit-equal
//!    backing ⇒ [`PlaneEqError::Undeclared`] — near-coincidence NEVER
//!    silently becomes contact (F6); a scalar without a bit channel
//!    also lands here (conservative).
//!
//! In-band margins escalate typed ([`PlaneEqError::Escalated`]).

use geom_core::{Band, Decide, Indeterminate, Point3, Sign, Vec3};

use crate::validate::decide;

/// The relation between two oriented planes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlaneRelation {
    /// Same plane, same orientation (the ⁺ case of Eq. 15.3).
    SameOriented,
    /// Same plane, opposite orientation (the ⁻ case).
    SameOpposite,
    /// Definitely different planes.
    Distinct,
}

/// Typed refusal of [`oriented_plane_eq`].
#[derive(Debug)]
pub enum PlaneEqError {
    /// A margin landed in the sliver band.
    Escalated(Indeterminate),
    /// Geometrically coincident-or-near without bit-equal descriptions:
    /// an undeclared coincidence (F6).
    Undeclared(Indeterminate),
}

/// One plane's conventional description (a `Surface::Plane`'s origin
/// and unit outward normal).
#[derive(Clone, Copy, Debug)]
pub struct PlaneDesc<T: geom_core::Real> {
    /// A point on the plane.
    pub origin: Point3<T>,
    /// The unit outward normal.
    pub normal: Vec3<T>,
}

/// Canonical bits of `(n̂, d)`. Zero signs are folded (`x + 0`, exact:
/// `−0.0 + 0.0 = +0.0`, identity elsewhere) so that IEEE negation of a
/// description — which flips zero signs — still lands on the same
/// canonical bits; without this, `(0,0,−1)` negated is `(−0,−0,1)` and
/// the opposite-orientation rung would never fire on axis planes.
fn bits_of<T: Decide>(n: Vec3<T>, d: T) -> Option<[geom_core::bit_identity::ScalarBits; 4]> {
    let c = |x: T| x + T::zero();
    Some([
        geom_core::bit_identity::repr_bits(&c(n.x))?,
        geom_core::bit_identity::repr_bits(&c(n.y))?,
        geom_core::bit_identity::repr_bits(&c(n.z))?,
        geom_core::bit_identity::repr_bits(&c(d))?,
    ])
}

/// **`oriented_plane_eq`** — module docs for the ladder. `arm` is the
/// lever arm in meters metering the angular/offset margins (the extent
/// over which the verdict is consumed); `band` the run's linear band.
///
/// # Errors
///
/// [`PlaneEqError`] — sliver escalation or undeclared coincidence.
pub fn oriented_plane_eq<T: Decide>(
    p1: &PlaneDesc<T>,
    p2: &PlaneDesc<T>,
    arm: T,
    band: Band,
) -> Result<PlaneRelation, PlaneEqError> {
    // Canonical description: (n̂, d = n̂·origin). Normals are unit by
    // the Surface::Plane convention; d is computed the same way on both
    // sides, so shared recipe data yields bit-equal (n̂, d).
    let d1 = p1.normal.dot(p1.origin - Point3::origin());
    let d2 = p2.normal.dot(p2.origin - Point3::origin());

    // Rung 1: the declared (bit-equal) rung, both orientations.
    if let (Some(b1), Some(b2)) = (bits_of(p1.normal, d1), bits_of(p2.normal, d2)) {
        if b1 == b2 {
            return Ok(PlaneRelation::SameOriented);
        }
        if bits_of(-p2.normal, -d2).is_some_and(|b2n| b1 == b2n) {
            return Ok(PlaneRelation::SameOpposite);
        }
    }

    // Rung 2: definite-different by geometry. Parallelism first.
    let parallel_margin = p1.normal.cross(p2.normal).norm() * arm;
    match decide("bool_plane_parallel", parallel_margin, band) {
        Ok(Sign::Positive) => return Ok(PlaneRelation::Distinct),
        Ok(Sign::Zero) => {}
        Ok(Sign::Negative) => {
            // A norm cannot be definitely negative — poisoned input.
            return Err(PlaneEqError::Escalated(Indeterminate {
                margin: geom_core::MarginDiag::Invalid,
                band,
                predicate: Some("bool_plane_parallel"),
            }));
        }
        Err(diag) => return Err(PlaneEqError::Escalated(diag)),
    }
    // Parallel (within band): orientation sign from the normal dot
    // (definite by construction when the cross is ~0 and both unit).
    let sign_margin = p1.normal.dot(p2.normal);
    let sigma = match decide("bool_plane_orient", sign_margin, band) {
        Ok(Sign::Positive) => T::one(),
        Ok(Sign::Negative) => -T::one(),
        Ok(Sign::Zero) => {
            return Err(PlaneEqError::Escalated(Indeterminate {
                margin: geom_core::MarginDiag::Invalid,
                band,
                predicate: Some("bool_plane_orient"),
            }));
        }
        Err(diag) => return Err(PlaneEqError::Escalated(diag)),
    };
    let offset_margin = d1 - sigma * d2;
    match decide("bool_plane_offset", offset_margin, band) {
        Ok(Sign::Positive | Sign::Negative) => Ok(PlaneRelation::Distinct),
        // Rung 3: geometrically the same plane, but the declared rung
        // above did not fire — undeclared coincidence, typed.
        Ok(Sign::Zero) => Err(PlaneEqError::Undeclared(Indeterminate {
            margin: geom_core::MarginDiag::Invalid,
            band,
            predicate: Some("bool_plane_offset"),
        })),
        Err(diag) => Err(PlaneEqError::Undeclared(diag)),
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use geom_core::Band;

    fn band() -> Band {
        Band::linear().unwrap()
    }

    fn plane(o: [f64; 3], n: [f64; 3]) -> PlaneDesc<f64> {
        PlaneDesc {
            origin: Point3::new(o[0], o[1], o[2]),
            normal: Vec3::new(n[0], n[1], n[2]),
        }
    }

    /// Bit-equal canonical descriptions decide exactly — including
    /// different origins on the same axis plane (canonical d equal).
    #[test]
    fn declared_rungs() {
        let p1 = plane([1.0, 2.0, 5.0], [0.0, 0.0, 1.0]);
        let p2 = plane([-3.0, 7.0, 5.0], [0.0, 0.0, 1.0]);
        assert_eq!(
            oriented_plane_eq(&p1, &p2, 1.0, band()).unwrap(),
            PlaneRelation::SameOriented
        );
        let p3 = plane([0.0, 0.0, 5.0], [0.0, 0.0, -1.0]);
        assert_eq!(
            oriented_plane_eq(&p1, &p3, 1.0, band()).unwrap(),
            PlaneRelation::SameOpposite
        );
    }

    /// Definitely different planes: non-parallel, and parallel-offset.
    #[test]
    fn distinct_rungs() {
        let p1 = plane([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
        let tilted = plane([0.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
        assert_eq!(
            oriented_plane_eq(&p1, &tilted, 1.0, band()).unwrap(),
            PlaneRelation::Distinct
        );
        let offset = plane([0.0, 0.0, 4.0], [0.0, 0.0, 1.0]);
        assert_eq!(
            oriented_plane_eq(&p1, &offset, 1.0, band()).unwrap(),
            PlaneRelation::Distinct
        );
        // Opposite-oriented offset plane is also distinct.
        let offset_flip = plane([0.0, 0.0, 4.0], [0.0, 0.0, -1.0]);
        assert_eq!(
            oriented_plane_eq(&p1, &offset_flip, 1.0, band()).unwrap(),
            PlaneRelation::Distinct
        );
    }

    /// Geometrically coincident but bit-different (an independently
    /// renormalized normal): undeclared coincidence, typed — never
    /// silently "same", never silently "different".
    #[test]
    fn near_coincidence_is_undeclared() {
        let p1 = plane([0.0, 0.0, 5.0], [0.0, 0.0, 1.0]);
        let eps = geom_core::Tolerance::get().eps;
        // Same plane to within a fraction of ε, described differently.
        let p2 = plane([0.0, 0.0, 5.0 + 0.25 * eps], [0.0, 0.0, 1.0]);
        let err = oriented_plane_eq(&p1, &p2, 1.0, band()).unwrap_err();
        assert!(matches!(err, PlaneEqError::Undeclared(_)), "{err:?}");
    }

    /// A near-miss in the sliver band escalates typed.
    #[test]
    fn sliver_offset_escalates() {
        let p1 = plane([0.0, 0.0, 5.0], [0.0, 0.0, 1.0]);
        let eps = geom_core::Tolerance::get().eps;
        let k = geom_core::Tolerance::get().k;
        let p2 = plane([0.0, 0.0, 5.0 + 0.5 * k * eps], [0.0, 0.0, 1.0]);
        let err = oriented_plane_eq(&p1, &p2, 1.0, band()).unwrap_err();
        assert!(matches!(
            err,
            PlaneEqError::Undeclared(_) | PlaneEqError::Escalated(_)
        ));
    }
}
