//! Test vocabulary over this crate's types, for the suites downstream.

use crate::{Affine3, Band, Decide, ErrorTextReading, MarginDiag, RigidMotion, Tol};

/// The upper end of a reporting margin: the point margin, or the
/// enclosure's upper end; `NaN` for a poisoned reading, which no bound
/// comparison passes. The rows that pin a refusal's bound against a
/// planted miss read it here.
#[must_use]
pub fn upper(margin: MarginDiag) -> f64 {
    match margin.diagnostic_f64_for_error_text() {
        ErrorTextReading::Value(m) | ErrorTextReading::Enclosure { hi: m, .. } => m,
        ErrorTextReading::Invalid => f64::NAN,
    }
}

/// **A matrix a suite holds, admitted at the decided door** at `tol`'s
/// linear band ([`RigidMotion::check_rigid`]): the question a suite
/// that builds its map as twelve numbers asks before it moves a body
/// by it. Panics, naming the refusal, when the map is not admitted.
#[must_use]
#[allow(clippy::panic)]
pub fn rigid<T: Decide>(map: &Affine3<T>, tol: Tol) -> RigidMotion<T> {
    let band =
        Band::linear(tol).unwrap_or_else(|e| panic!("the run's tolerance forms no band: {e}"));
    RigidMotion::check_rigid(map, band)
        .unwrap_or_else(|e| panic!("the map is not admitted as a rigid motion: {e}"))
}

/// The Bernstein row on `[a, b] ⊆ [0, 1]` of a `[0, 1]` row, by the
/// arithmetic the retired `restrict_bernstein` ran: the oracle for the
/// trim bisection's bit-identity row against the door it moved to,
/// [`crate::spline::restrict_segment`] on `[0, 1]`.
///
/// # Errors
///
/// As the retired door: an empty row, or an end outside `[0, 1]`.
pub fn bernstein_restriction_as_retired(
    row: &[crate::Interval],
    ends: (crate::Interval, crate::Interval),
) -> Result<Vec<crate::Interval>, crate::spline::BlossomError> {
    crate::spline::blossom::bernstein_restriction_as_retired(row, ends)
}
