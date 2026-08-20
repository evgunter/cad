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
