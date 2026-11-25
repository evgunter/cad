//! Review probe (PR 7783): a small closed rim in a plane face.

use core::f64::consts::PI;
use geom_core::{Point2, Tol};
use sweep::Revolution;
use sweep::test_support::{brick, finished, revolved_about_y};
use topo::{BooleanResult, subtract};

fn tol() -> Tol {
    Tol::witness()
}

fn run(s: f64) {
    run_with(s, "quad", false);
    run_with(s, "quad-rev", true);
    run_groove(s, false);
    run_groove(s, true);
}

fn run_groove(s: f64, rev: bool) {
    let slab = finished(
        "slab",
        brick::<f64>((-3.0, 3.0), (-1.0, 0.0), (-3.0, 3.0), tol()),
        tol(),
    );
    let mut pts = vec![(s, 0.0), (0.5 * s, -0.5 * s), (1.5 * s, -0.5 * s)];
    if rev {
        pts.reverse();
    }
    let want = 36.0 - PI / 2.0 * s.powi(3);
    go(&format!("groove rev={rev}"), s, &slab, &pts, want);
}

fn run_with(s: f64, what: &str, rev: bool) {
    let block = finished(
        "block",
        brick::<f64>((-2.0, 2.0), (0.0, 1.0), (-2.0, 2.0), tol()),
        tol(),
    );
    let mut pts = vec![
        (0.5 * s, 1.0 - 0.5 * s),
        (s, 1.0 - 0.5 * s),
        (s, 1.0),
        (0.6 * s, 1.0 - 0.4 * s),
    ];
    if rev {
        pts.reverse();
    }
    let want = 16.0 - 2.0 * PI * 5.0 / 48.0 * s.powi(3);
    go(what, s, &block, &pts, want);
}

fn go(what: &str, s: f64, block: &topo::AtRestBody<f64>, pts: &[(f64, f64)], want: f64) {
    let verts = pts.iter().map(|&(x, y)| (Point2::new(x, y), 0.0)).collect();
    let ring = finished("ring", revolved_about_y(verts, Revolution::Full, tol()), tol());
    go_body(what, s, block, &ring, want);
}

fn go_body(what: &str, s: f64, block: &topo::AtRestBody<f64>, ring: &topo::AtRestBody<f64>, want: f64) {
    match subtract(block, ring, tol()) {
        Ok(BooleanResult::Body(r)) => {
            let b = &r.body;
            let t1 = topo::validate(b);
            let t3 = topo::validate_geometric(b, tol());
            let t3p = topo::validate_pseudomanifold(b, r.contacts(), tol());
            let vol = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                topo::test_support::pinned_volume("p", &finished("p", b.clone(), tol()), tol()).volume
            }));
            println!(
                "PROBE {what} s={s:e} eps={:e}: BUILT solids={} t1={t1:?} t3={t3:?} t3'={t3p:?} vol={vol:?} want={want} ee={}",
                tol().eps(),
                b.solids().count(),
                r.contacts().edge_edge().count()
            );
        }
        other => println!("PROBE {what} s={s:e} eps={:e}: {:?}", tol().eps(), other.map(drop)),
    }
}

#[test]
fn probe_small_plane_rim() {
    for s in [1e-1, 3e-3, 1e-3, 5e-4] {
        run(s);
    }
}

fn sphere_case(rho: f64) {
    use core::f64::consts::SQRT_2;
    use geom_core::Vec3;
    let r = SQRT_2;
    let a = (rho / r).asin();
    let k = rho / 0.55;
    let apex = (r * a.sin(), r * a.cos());
    let b = (apex.0 - 0.1 * k, apex.1 - 0.15 * k);
    let c = (apex.0 + 0.15 * k, apex.1 - 0.25 * k);
    let verts = [apex, b, c].iter().map(|&(x, y)| (Point2::new(x, y), 0.0)).collect();
    let keel = finished("keel", revolved_about_y(verts, Revolution::Full, tol()), tol());
    let area = ((b.0 - apex.0) * (c.1 - apex.1) - (c.0 - apex.0) * (b.1 - apex.1)).abs() / 2.0;
    let vk = area * 2.0 * PI * (apex.0 + b.0 + c.0) / 3.0;
    let ball = finished("ball", sweep::test_support::ball_poled_z(r, Vec3::zero(), tol()), tol());
    let want = 4.0 / 3.0 * PI * r.powi(3) - vk;
    go_body(&format!("sphere keelvol={vk:e}"), rho, &ball, &keel, want);
}


#[test]
fn probe_small_sphere_rim() {
    for rho in [0.55, 1e-1, 3e-2, 1e-2, 3e-3, 1e-3, 5e-4] {
        sphere_case(rho);
    }
}

#[test]
fn probe_half_ball_parts_touch() {
    use core::f64::consts::SQRT_2;
    use geom_core::{Affine3, Point3, Vec3};
    let tol = tol();
    let ball = finished("ball", sweep::test_support::ball_poled_z(SQRT_2, Vec3::zero(), tol), tol);
    let cut = finished("z>=0", brick::<f64>((-3.0, 3.0), (-3.0, 3.0), (0.0, 3.0), tol), tol);
    let half = match topo::intersect(&ball, &cut, tol) {
        Ok(BooleanResult::Body(b)) => b.body,
        r => panic!("{r:?}"),
    };
    let (a, turn) = (0.7f64, 0.3f64);
    let z0 = (2.0 - a * a).sqrt();
    let circle = profile::circle(Point2::new(0.0, 0.0), a, tol).unwrap();
    let plane = profile::SketchPlane::xy_at(Point3::new(0.0, 0.0, -1.0));
    let p = profile::Profile::new(plane, vec![circle.into()]).validate(tol).unwrap();
    let side = sweep::ExtrudeSide::Along;
    let t = sweep::extrude(&p, sweep::Extrusion::Distance { depth: z0 + 1.0, side }, tol)
        .unwrap()
        .body
        .into_body();
    let m = Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_z(), turn);
    let tube = finished("tube", topo::transform_rigid(&t, &geom_core::test_support::rigid(&m, tol)).unwrap(), tol);
    let want = 4.0 / 3.0 * PI * SQRT_2.powi(3) / 2.0 - PI * a * a * z0;
    let r = subtract(&half, &tube, tol);
    match &r {
        Ok(BooleanResult::Body(b)) => println!(
            "PROBE halfball B-A: BUILT solids={} bar={:?} t3={:?} t3'={:?} want={want}",
            b.body.solids().count(),
            finished("r", b.body.clone(), tol).bar(),
            topo::validate_geometric(&b.body, tol),
            topo::validate_pseudomanifold(&b.body, b.contacts(), tol)
        ),
        other => println!("PROBE halfball B-A: {:?}", other.as_ref().map(|_| ())),
    }
    println!("PROBE halfball outcome: {}", crate::common::differential::outcome(r, want, tol));
}

#[test]
fn probe_plane_rim_in_a_holed_face() {
    use geom_core::{Affine3, Vec3};
    let tol = tol();
    let block = finished("block", brick::<f64>((-2.0, 2.0), (0.0, 1.0), (-2.0, 2.0), tol), tol);
    for (what, x) in [("hole outside the disc", 1.6), ("hole inside the disc", 0.0)] {
        let verts = [(0.0, -1.0), (0.2, -1.0), (0.2, 2.0), (0.0, 2.0)]
            .iter()
            .map(|&(x, y)| (Point2::new(x, y), 0.0))
            .collect();
        let pin = revolved_about_y(verts, Revolution::Full, tol);
        let m = Affine3::translation(Vec3::new(x, 0.0, 0.0));
        let pin = finished("pin", topo::transform_rigid(&pin, &geom_core::test_support::rigid(&m, tol)).unwrap(), tol);
        let holed = match subtract(&block, &pin, tol) {
            Ok(BooleanResult::Body(b)) => finished("holed", b.body.clone(), tol),
            r => panic!("{what}: {:?}", r.map(drop)),
        };
        let pts = [(0.5, 0.5), (1.0, 0.5), (1.0, 1.0), (0.6, 0.6)];
        let want = 16.0 - PI * 0.04 - 2.0 * PI * 5.0 / 48.0;
        go(what, 1.0, &holed, &pts, want);
    }
}
