//! Scratch (carve/pinned-corners phase 1): what today's loft door does
//! with pinned corners. Not for merging.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::print_stdout)]

use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use profile::test_support::bulge_loop;
use profile::{ProfileLoop, RawLoop, SketchPlane};
use sweep::loft_body;

fn square() -> Vec<ProfileLoop<f64>> {
    vec![ProfileLoop::polygon(
        [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)]
            .iter()
            .map(|&(x, y)| Point2::new(x, y)),
    )]
}

fn lens() -> Vec<ProfileLoop<f64>> {
    vec![bulge_loop(vec![
        (Point2::new(1.0, 0.0), -0.5),
        (Point2::new(-1.0, 0.0), -0.5),
    ])]
}

fn places(ms: &[Affine3<f64>], tol: Tol) -> Vec<SketchPlane<f64>> {
    ms.iter()
        .map(|m| SketchPlane::from_frame(geom_core::test_support::rigid(m, tol)))
        .collect()
}

fn report(what: &str, sections: &[Vec<ProfileLoop<f64>>], ps: &[SketchPlane<f64>], deg: usize, tol: Tol) {
    match loft_body::<f64>(sections, ps, deg, tol) {
        Ok(l) => {
            let v = topo::validate_pseudomanifold(&l.body, &Default::default(), tol);
            println!("{what}: BUILT; faces {}; tier3' {:?}", l.body.faces().count(), v.map(|_| ()));
        }
        Err(e) => println!("{what}: REFUSED {e:?}\n    text: {e}"),
    }
}

fn main() {
    let tol = Tol::witness();
    let rot = |p: [f64; 3], d: [f64; 3], a: f64| {
        Affine3::rotation_about_axis(Point3::new(p[0], p[1], p[2]), Vec3::new(d[0], d[1], d[2]), a)
    };
    // 1. Wedge: section 1 is section 0 turned about its edge x = -1 (a hinge), v-degree 1.
    let wedge = places(&[Affine3::identity(), rot([-1.0, 0.0, 0.0], [0.0, -1.0, 0.0], 0.5)], tol);
    report("wedge (hinge x=-1, 2 sections, deg 1)", &vec![square(); 2], &wedge, 1, tol);
    // 1b. A single corner pinned, two sections, v-degree 1.
    let one = places(&[Affine3::identity(), rot([-1.0, -1.0, 0.0], [1.0, -1.0, 0.0], 0.5)], tol);
    report("single pinned corner (2 sections, deg 1)", &vec![square(); 2], &one, 1, tol);
    for a in [0.5_f64, -0.5] {
        let one = places(&[Affine3::identity(), rot([-1.0, -1.0, 0.0], [1.0, -1.0, 0.0], a)], tol);
        report(&format!("single pinned corner angle {a} (deg 1)"), &vec![square(); 2], &one, 1, tol);
    }
    // 2. Lens turned about its chord (the x axis): 3 sections, v-degree 2.
    let lp = places(
        &[Affine3::identity(), rot([0.0; 3], [-1.0, 0.0, 0.0], 0.4), rot([0.0; 3], [-1.0, 0.0, 0.0], 0.8)],
        tol,
    );
    report("lens about its chord (3 sections, deg 2)", &vec![lens(); 3], &lp, 2, tol);
    let lp2 = places(&[Affine3::identity(), rot([0.0; 3], [-1.0, 0.0, 0.0], 0.8)], tol);
    report("lens about its chord (2 sections, deg 1)", &vec![lens(); 2], &lp2, 1, tol);
    // 3. A corner pinned across every section, v-degree 2.
    for a in [0.3_f64, -0.3] {
        let every = places(
            &[Affine3::identity(), rot([-1.0, -1.0, 0.0], [1.0, -1.0, 0.0], a), rot([-1.0, -1.0, 0.0], [1.0, -1.0, 0.0], 2.0 * a)],
            tol,
        );
        report(&format!("corner pinned across every section, step {a} (deg 2)"), &vec![square(); 3], &every, 2, tol);
        report(&format!("corner pinned across every section, step {a} (deg 1)"), &vec![square(); 3], &every, 1, tol);
    }
    // 4. Hinge off strip 0 (edge y = +1), the row's measured fixture.
    let h = places(
        &[Affine3::identity(), rot([0.0, 1.0, 0.0], [1.0, 0.0, 0.0], -0.3), Affine3::translation(Vec3::new(0.0, 0.0, 2.0))],
        tol,
    );
    report("hinge y=+1 (3 sections, deg 2)", &vec![square(); 3], &h, 2, tol);
    report("hinge y=+1 (3 sections, deg 1)", &vec![square(); 3], &h, 1, tol);
}
