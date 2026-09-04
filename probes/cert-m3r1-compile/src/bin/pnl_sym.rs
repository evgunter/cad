use geom_core::{Band, Sym};
use geom::{NurbsCurve3, NurbsSurface, Surface};
fn symbolic(
    carrier: &NurbsCurve3<Sym<f64>>,
    plane: &Surface<Sym<f64>>,
    wall: &NurbsSurface<Sym<f64>>,
    extent: Sym<f64>,
    band: Band,
) {
    let _ = geom_brep::plane_nurbs_limbs(carrier, plane, wall, extent, band);
}
fn main() {}
