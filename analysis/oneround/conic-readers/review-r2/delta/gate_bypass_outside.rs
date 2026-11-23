// Review probe: a production module whose source lives outside crates/*/src.
#[allow(clippy::disallowed_methods)]
pub(crate) fn g5_outside(e: &crate::Ellipse<f64>) -> geom_core::Point3<f64> {
    crate::Conic::frame(e).origin()
}
