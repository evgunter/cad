use geom_core::{Band, Dual64, Sym};
use geom::{NurbsCurve3, NurbsSurface, Surface};
fn symbolic(
    carrier: &NurbsCurve3<Sym<Dual64>>,
    plane: &Surface<Sym<Dual64>>,
    wall: &NurbsSurface<Sym<Dual64>>,
    extent: Sym<Dual64>,
    band: Band,
) {
    let _ = geom_brep::plane_nurbs_limbs(carrier, plane, wall, extent, band);
}
fn main() {}
