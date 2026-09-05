use geom::{NurbsCurve3, NurbsSurface, Surface};
use geom_core::{Band, Sym};
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
