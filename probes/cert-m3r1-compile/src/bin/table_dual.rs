use geom_core::{Band, Dual64, Point3, Tol};
use geom::Surface;
use geom_brep::{EdgeCurve, EdgeCurveSpec, keys::SurfaceKey};
use topo::{Body, ContactRecords};
fn certify_doors(spec: EdgeCurveSpec<Dual64>, p: Point3<Dual64>, q: Point3<Dual64>, s: impl Fn(SurfaceKey) -> Option<Surface<Dual64>> + Copy, band: Band, ec: &EdgeCurve<Dual64>) {
    let _ = EdgeCurve::certify(spec, p, q, s, band);
    let _ = ec.recertify(p, q, s, band);
    let _ = ec.recertify_via(p, q, s, band, None);
    let _ = ec.needs_nurbs_lane(s);
}
fn validators(b: &Body<Dual64>, r: &ContactRecords, tol: Tol) {
    let _ = topo::validate_geometric_structural(b, tol);
    let _ = topo::validate_pseudomanifold(b, r, tol);
    let _ = topo::contact_marks(b, tol);
}
fn main() {}
