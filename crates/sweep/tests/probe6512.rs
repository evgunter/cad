#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use geom_core::{Point2, Point3, Tol};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::test_support::finished;
use sweep::{ExtrudeSide, Extrusion, extrude};
use topo::{BooleanResult, union, intersect, subtract};

fn rod(z0: f64, z1: f64, start: f64) -> topo::Body<f64> {
    let tol = Tol::witness();
    let p = |a: f64| Point2::new(a.cos(), a.sin());
    let lp = bulge_loop(vec![(p(start), 1.0), (p(start + std::f64::consts::PI), 1.0)]);
    let profile = Profile::new(SketchPlane::xy_at(Point3::new(0.0, 0.0, z0)), vec![lp]).validate(tol).unwrap();
    extrude(&profile, Extrusion::Distance { depth: z1 - z0, side: ExtrudeSide::Along }, tol).unwrap().body
}

fn report(name: &str, r: Result<BooleanResult<f64>, topo::BooleanError>) {
    match r {
        Ok(BooleanResult::Body(out)) => {
            let iso: Vec<_> = out.coincidences.iter().filter(|r| r.site == topo::DecisionSite::IsoJoin).collect();
            eprintln!("PROBE6512 {name}: joins={} iso_rows={} {:?}", out.naming.edge_joins.len(), iso.len(), iso);
        }
        Ok(other) => eprintln!("PROBE6512 {name}: non-body {:?}", std::mem::discriminant(&other)),
        Err(e) => eprintln!("PROBE6512 {name}: refused {e}"),
    }
}

#[test]
fn probe6512_rods() {
    let tol = Tol::witness();
    let f = |n: &str, b| finished(n, b, tol);
    report("overlap same seam", union(&f("a", rod(0.0, 1.0, 0.0)), &f("b", rod(0.5, 2.0, 0.0)), tol));
    report("overlap other seam", union(&f("a", rod(0.0, 1.0, 0.0)), &f("b", rod(0.5, 2.0, 1.0)), tol));
    report("stacked other seam", union(&f("a", rod(0.0, 1.0, 0.0)), &f("b", rod(1.0, 2.0, 1.0)), tol));
    report("inner nested", union(&f("a", rod(0.0, 2.0, 0.0)), &f("b", rod(0.5, 1.5, 0.0)), tol));
    report("intersect overlap", intersect(&f("a", rod(0.0, 1.0, 0.0)), &f("b", rod(0.5, 2.0, 0.0)), tol));
    report("subtract overlap", subtract(&f("a", rod(0.0, 1.0, 0.0)), &f("b", rod(0.5, 2.0, 0.0)), tol));
    report("subtract end-flush", subtract(&f("a", rod(0.0, 2.0, 0.0)), &f("b", rod(1.0, 2.0, 1.0)), tol));
}
