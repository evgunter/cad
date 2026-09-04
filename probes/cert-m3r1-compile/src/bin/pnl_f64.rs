use geom_core::Band;
use geom::{NurbsCurve3, NurbsSurface, Surface};
fn certified(
    carrier: &NurbsCurve3<f64>,
    plane: &Surface<f64>,
    wall: &NurbsSurface<f64>,
    extent: f64,
    band: Band,
) {
    let _ = geom_brep::plane_nurbs_limbs(carrier, plane, wall, extent, band);
}
fn main() {}
