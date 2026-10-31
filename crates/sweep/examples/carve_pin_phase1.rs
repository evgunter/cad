//! Scratch (carve/pinned-corners phase 1): what today's loft door does
//! with pinned corners. Not for merging.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::print_stdout
)]

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

fn report(
    what: &str,
    sections: &[Vec<ProfileLoop<f64>>],
    ps: &[SketchPlane<f64>],
    deg: usize,
    tol: Tol,
) {
    match loft_body::<f64>(sections, ps, deg, tol) {
        Ok(l) => {
            let v = topo::validate_pseudomanifold(&l.body, &Default::default(), tol);
            println!(
                "{what}: BUILT; faces {}; tier3' {:?}",
                l.body.faces().count(),
                v.map(|_| ())
            );
        }
        Err(e) => println!("{what}: REFUSED {e:?}\n    text: {e}"),
    }
}

fn main() {
    let tol = Tol::witness();
    let rot = |p: [f64; 3], d: [f64; 3], a: f64| {
        Affine3::rotation_about_axis(
            Point3::new(p[0], p[1], p[2]),
            Vec3::new(d[0], d[1], d[2]),
            a,
        )
    };
    // 1. Wedge: section 1 is section 0 turned about its edge x = -1 (a hinge), v-degree 1.
    let wedge = places(
        &[
            Affine3::identity(),
            rot([-1.0, 0.0, 0.0], [0.0, -1.0, 0.0], 0.5),
        ],
        tol,
    );
    report(
        "wedge (hinge x=-1, 2 sections, deg 1)",
        &vec![square(); 2],
        &wedge,
        1,
        tol,
    );
    // 1b. A single corner pinned, two sections, v-degree 1.
    let one = places(
        &[
            Affine3::identity(),
            rot([-1.0, -1.0, 0.0], [1.0, -1.0, 0.0], 0.5),
        ],
        tol,
    );
    report(
        "single pinned corner (2 sections, deg 1)",
        &vec![square(); 2],
        &one,
        1,
        tol,
    );
    for a in [0.5_f64, -0.5] {
        let one = places(
            &[
                Affine3::identity(),
                rot([-1.0, -1.0, 0.0], [1.0, -1.0, 0.0], a),
            ],
            tol,
        );
        report(
            &format!("single pinned corner angle {a} (deg 1)"),
            &vec![square(); 2],
            &one,
            1,
            tol,
        );
    }
    // 2. Lens turned about its chord (the x axis): 3 sections, v-degree 2.
    let lp = places(
        &[
            Affine3::identity(),
            rot([0.0; 3], [-1.0, 0.0, 0.0], 0.4),
            rot([0.0; 3], [-1.0, 0.0, 0.0], 0.8),
        ],
        tol,
    );
    report(
        "lens about its chord (3 sections, deg 2)",
        &vec![lens(); 3],
        &lp,
        2,
        tol,
    );
    let lp2 = places(
        &[Affine3::identity(), rot([0.0; 3], [-1.0, 0.0, 0.0], 0.8)],
        tol,
    );
    report(
        "lens about its chord (2 sections, deg 1)",
        &vec![lens(); 2],
        &lp2,
        1,
        tol,
    );
    // 3. A corner pinned across every section, v-degree 2.
    for a in [0.3_f64, -0.3] {
        let every = places(
            &[
                Affine3::identity(),
                rot([-1.0, -1.0, 0.0], [1.0, -1.0, 0.0], a),
                rot([-1.0, -1.0, 0.0], [1.0, -1.0, 0.0], 2.0 * a),
            ],
            tol,
        );
        report(
            &format!("corner pinned across every section, step {a} (deg 2)"),
            &vec![square(); 3],
            &every,
            2,
            tol,
        );
        report(
            &format!("corner pinned across every section, step {a} (deg 1)"),
            &vec![square(); 3],
            &every,
            1,
            tol,
        );
    }
    // 4. Hinge off strip 0 (edge y = +1), the row's measured fixture.
    let h = places(
        &[
            Affine3::identity(),
            rot([0.0, 1.0, 0.0], [1.0, 0.0, 0.0], -0.3),
            Affine3::translation(Vec3::new(0.0, 0.0, 2.0)),
        ],
        tol,
    );
    report(
        "hinge y=+1 (3 sections, deg 2)",
        &vec![square(); 3],
        &h,
        2,
        tol,
    );
    report(
        "hinge y=+1 (3 sections, deg 1)",
        &vec![square(); 3],
        &h,
        1,
        tol,
    );

    // 5. Slid hinge, crossing sections, asymmetric lens, translated squares, exact/rounding strip-0 hinges.
    let slid = places(
        &[
            Affine3::identity(),
            Affine3::translation(Vec3::new(0.5, 0.0, 0.0))
                * rot([0.0, -1.0, 0.0], [1.0, 0.0, 0.0], 0.3),
            Affine3::translation(Vec3::new(0.0, 0.0, 2.0)),
        ],
        tol,
    );
    report("slid hinge (deg 2)", &vec![square(); 3], &slid, 2, tol);
    let cross = places(
        &[Affine3::identity(), rot([0.0; 3], [1.0, 0.0, 0.0], 0.5)],
        tol,
    );
    report("crossing (deg 1)", &vec![square(); 2], &cross, 1, tol);
    let alens = || {
        vec![bulge_loop(vec![
            (Point2::new(1.0, 0.0), -0.8),
            (Point2::new(-1.0, 0.0), -0.3),
        ])]
    };
    report(
        "asym lens about its chord (deg 2)",
        &vec![alens(); 3],
        &lp,
        2,
        tol,
    );
    report(
        "asym lens about its chord (deg 1, 2 sections)",
        &vec![alens(); 2],
        &lp2,
        1,
        tol,
    );
    let stack = places(
        &[
            Affine3::identity(),
            Affine3::translation(Vec3::new(0.0, 0.0, 1.0)),
            Affine3::translation(Vec3::new(0.0, 0.0, 2.0)),
        ],
        tol,
    );
    report("plain stack (deg 2)", &vec![square(); 3], &stack, 2, tol);
    report("plain lens stack (deg 2)", &vec![lens(); 3], &stack, 2, tol);
    let exact = places(
        &[
            Affine3::identity(),
            rot([0.0, -1.0, 0.0], [1.0, 0.0, 0.0], 0.3),
            Affine3::translation(Vec3::new(0.0, 0.0, 2.0)),
        ],
        tol,
    );
    report(
        "exact strip-0 hinge (deg 2)",
        &vec![square(); 3],
        &exact,
        2,
        tol,
    );
    report(
        "exact strip-0 hinge (deg 1)",
        &vec![square(); 3],
        &exact,
        1,
        tol,
    );
    let pp = Affine3::translation(Vec3::new(0.37, -0.21, 0.13))
        * Affine3::rotation_about_axis(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.3, 0.4, 0.866), 0.7);
    let (a, b) = (
        pp.transform_point(Point3::new(-1.0, -1.0, 0.0)),
        pp.transform_point(Point3::new(1.0, -1.0, 0.0)),
    );
    let n = pp.linear.c2;
    let round = places(
        &[
            pp,
            Affine3::rotation_about_axis(a, b - a, 0.3) * pp,
            Affine3::translation(Vec3::new(2.0 * n.x, 2.0 * n.y, 2.0 * n.z)) * pp,
        ],
        tol,
    );
    report(
        "rounding strip-0 hinge (deg 2)",
        &vec![square(); 3],
        &round,
        2,
        tol,
    );
    for step in [5e-10, 1e-12, 3e-9, 0.0] {
        let st = places(
            &[
                Affine3::identity(),
                Affine3::translation(Vec3::new(0.0, 0.0, 1.0)),
                Affine3::translation(Vec3::new(0.0, 0.0, 1.0 + step)),
                Affine3::translation(Vec3::new(0.0, 0.0, 2.0)),
            ],
            tol,
        );
        report(
            &format!("step {step:e} (deg 2)"),
            &vec![square(); 4],
            &st,
            2,
            tol,
        );
    }
}
