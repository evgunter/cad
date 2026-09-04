use geom_core::{Band, Dual64, Point3};
use geom::Surface;
use geom_brep::{EdgeCurve, EdgeCurveSpec, keys::SurfaceKey};
fn door(spec: EdgeCurveSpec<Dual64>, p: Point3<Dual64>, q: Point3<Dual64>, s: impl Fn(SurfaceKey) -> Option<Surface<Dual64>>, band: Band) {
    let _ = EdgeCurve::certify_nurbs_lane(spec, p, q, s, band);
}
fn main() {}
