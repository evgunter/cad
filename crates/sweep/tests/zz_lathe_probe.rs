#![allow(clippy::panic, clippy::unwrap_used, clippy::expect_used)]
use geom_core::{Tol, Vec3};
use sweep::test_support::{ball_poled, brick, finished};
use topo::Body;

fn v(a: [f64; 3]) -> Vec3<f64> { Vec3::new(a[0], a[1], a[2]) }

fn report(label: &str, r: Result<topo::boolean::BooleanResult<f64>, topo::boolean::BooleanError>) {
    match r {
        Err(e) => eprintln!("PROBE {label}: ERR {e:?}"),
        Ok(res) => {
            let b = &res.body().unwrap().body;
            eprintln!("PROBE {label}: OK faces={}", b.faces().count());
            for (k, f) in b.faces() {
                let n = topo::props::loop_edges(b, f.outer).map(|l| l.0.len()).unwrap_or(999);
                eprintln!("  face {k:?} kind={:?} outer_edges={n} rings={}", topo::query::face_surface_kind(b, k), f.rings.len());
            }
        }
    }
}

#[test]
fn probe() {
    let tol = Tol::witness();
    let ball = finished("ball", ball_poled(1.0, v([0.0; 3]), v([0.0, 1.0, 0.0]), tol), tol);
    for (k, f) in ball.faces() {
        let _ = (k, f);
    }
    let slabs: [(&str, (f64,f64),(f64,f64),(f64,f64)); 5] = [
        ("x-slab", (-0.5, 0.5), (-2.0, 2.0), (-2.0, 2.0)),
        ("z-slab", (-2.0, 2.0), (-2.0, 2.0), (-0.5, 0.5)),
        ("x>0.5", (0.5, 2.0), (-2.0, 2.0), (-2.0, 2.0)),
        ("x<-0.5", (-2.0, -0.5), (-2.0, 2.0), (-2.0, 2.0)),
        ("z<-0.5", (-2.0, 2.0), (-2.0, 2.0), (-2.0, -0.5)),
    ];
    let cap = finished("cap", brick::<f64>((0.5, 2.0), (-2.0, 2.0), (-2.0, 2.0), tol), tol);
    let trimmed = finished("trimmed", Body::clone(&topo::boolean::subtract(&ball, &cap, tol).unwrap().body().unwrap().body), tol);
    for (label, x, y, z) in [("T x<-0.5", (-2.0, -0.5), (-2.0, 2.0), (-2.0, 2.0)), ("T x-slab(-0.5,0.3)", (-0.5, 0.3), (-2.0, 2.0), (-2.0, 2.0))] {
        let slab = finished("slab", brick::<f64>(x, y, z, tol), tol);
        report(&format!("{label} ∩"), topo::boolean::intersect(&trimmed, &slab, tol));
        report(&format!("{label} ∖"), topo::boolean::subtract(&trimmed, &slab, tol));
    }
    for (label, x, y, z) in slabs {
        let slab = finished("slab", brick::<f64>(x, y, z, tol), tol);
        report(&format!("{label} ∩"), topo::boolean::intersect(&ball, &slab, tol));
        report(&format!("{label} ∖"), topo::boolean::subtract(&ball, &slab, tol));
        report(&format!("{label} ∪"), topo::boolean::union(&ball, &slab, tol));
    }
}
