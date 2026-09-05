use geom::{NurbsCurve3, NurbsSurface, Surface};
use geom_core::Band;
fn certified(
    carrier: &NurbsCurve3<geom_core::Probe>,
    plane: &Surface<geom_core::Probe>,
    wall: &NurbsSurface<geom_core::Probe>,
    extent: geom_core::Probe,
    band: Band,
) {
    let _ = geom_brep::plane_nurbs_limbs(carrier, plane, wall, extent, band);
}
fn main() {}
