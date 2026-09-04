use geom_core::{Band, Dual64};
use geom::{NurbsCurve3, NurbsSurface, Surface};
fn certified(
    carrier: &NurbsCurve3<Dual64>,
    plane: &Surface<Dual64>,
    wall: &NurbsSurface<Dual64>,
    extent: Dual64,
    band: Band,
) {
    let _ = geom_brep::plane_nurbs_limbs(carrier, plane, wall, extent, band);
}
fn main() {}
