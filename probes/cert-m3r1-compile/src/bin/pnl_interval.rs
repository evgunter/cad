use geom::{NurbsCurve3, NurbsSurface, Surface};
use geom_core::Band;
fn certified(
    carrier: &NurbsCurve3<geom_core::interval::Interval>,
    plane: &Surface<geom_core::interval::Interval>,
    wall: &NurbsSurface<geom_core::interval::Interval>,
    extent: geom_core::interval::Interval,
    band: Band,
) {
    let _ = geom_brep::plane_nurbs_limbs(carrier, plane, wall, extent, band);
}
fn main() {}
