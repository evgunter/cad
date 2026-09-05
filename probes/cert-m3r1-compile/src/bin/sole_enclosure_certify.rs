use geom::Surface;
use geom_brep::{EdgeCurve, EdgeCurveSpec, keys::SurfaceKey};
use geom_core::{Band, CertifiedEnclosure, Decide, Point3};
fn door<T: Decide + CertifiedEnclosure>(
    spec: EdgeCurveSpec<T>,
    p: Point3<T>,
    q: Point3<T>,
    s: impl Fn(SurfaceKey) -> Option<Surface<T>>,
    band: Band,
) {
    let _ = EdgeCurve::certify_nurbs_lane(spec, p, q, s, band);
}
fn main() {}
