#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use geom_core::{Point2, Point3, Tolerance, Vec3};
use geom_curves::Curve3;
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane, ValidatedProfile};
use sweep::{Extrusion, extrude};
use topo::splitting::{SplitPlane, split_reduce};

fn disc() -> ValidatedProfile<f64> {
    let lp = ProfileLoop::new(vec![
        ProfileVertex { pos: Point2::new(-0.5, 0.0), bulge: 1.0 },
        ProfileVertex { pos: Point2::new(0.5, 0.0), bulge: 1.0 },
    ]);
    Profile::new(SketchPlane::xy(), vec![lp]).validate(Tolerance::get()).unwrap()
}

/// Does a split-lane section arc stay ON the finite cylinder (z in
/// [0,1]) over its whole stored interval?
#[test]
fn debug_belly_arc_extent() {
    let body = extrude(&disc(), Extrusion::Distance(1.0)).unwrap().body;
    let n = 1.0 / 5.0f64.sqrt();
    let plane = SplitPlane { origin: Point3::new(0.0, 0.1, 0.5), normal: Vec3::new(0.0, 2.0 * n, n) };
    let red = split_reduce(&body, &plane).unwrap();
    let _ = red;
    // Use the joined result: run the whole pipeline but catch the error;
    // instead inspect via plane_section if available. Fall back: rebuild
    // by calling split on the ORIGINAL (which now fails), so inspect the
    // simpler tilted cut for a control.
    for (name, pl) in [
        ("belly", SplitPlane { origin: Point3::new(0.0, 0.1, 0.5), normal: Vec3::new(0.0, 2.0 * n, n) }),
        ("simple", SplitPlane { origin: Point3::new(0.0, 0.0, 0.5), normal: Vec3::new(0.3f64.sin(), 0.0, 0.3f64.cos()) }),
    ] {
        match topo::splitting::split(&body, &pl) {
            Ok(r) => {
                for part in [&r.above, &r.below] {
                    let Some(b) = part.body() else { continue };
                    for (ek, edge) in b.edges() {
                        let Some(c) = b.get_curve_geom(edge.curve).and_then(|g| g.certified()) else { continue };
                        if !matches!(c.carrier(), Curve3::Ellipse { .. }) { continue }
                        let (t0, t1) = c.params();
                        let mut zmin = f64::INFINITY; let mut zmax = f64::NEG_INFINITY;
                        for i in 0..=16 {
                            let t = t0 + (t1 - t0) * f64::from(i) / 16.0;
                            let z = c.carrier().eval(t).z;
                            zmin = zmin.min(z); zmax = zmax.max(z);
                        }
                        println!("{name} {ek:?} span={:.4} z=[{zmin:.4},{zmax:.4}]", t1 - t0);
                    }
                }
            }
            Err(e) => println!("{name}: ERR {e}"),
        }
    }
}
