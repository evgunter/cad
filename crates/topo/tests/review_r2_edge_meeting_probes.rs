//! **PR 5422 review R2 probes**: whether the edge-meeting ray's reading
//! of a needle depends on the order its body's entities were built in.
//! Each needle is built twelve ways, the same geometry each time (three
//! extrusion axes, four profile starts), along the axes and turned,
//! either way out; every build's aimed-ray outcome and point containment
//! at fixed points are printed when the twelve disagree. Evidence, not
//! pins: run with
//! `cargo nextest run --release -p topo --run-ignored only -E 'test(review_r2)' --no-capture`.

#![allow(clippy::expect_used, clippy::panic, clippy::print_stdout)]

use geom_core::{Band, Point3, Tol};
use topo::{AtRestBody, Body};

const NEEDLE: [f64; 6] = [10.5, 12.0, 15.0, 19.0, 21.0, 30.0];

fn rot(axis: [f64; 3], ang: f64) -> [[f64; 3]; 3] {
    let n = (axis[0] * axis[0] + axis[1] * axis[1] + axis[2] * axis[2]).sqrt();
    let (x, y, z) = (axis[0] / n, axis[1] / n, axis[2] / n);
    let (s, c) = ang.sin_cos();
    let k = 1.0 - c;
    [
        [c + x * x * k, x * y * k - z * s, x * z * k + y * s],
        [y * x * k + z * s, c + y * y * k, y * z * k - x * s],
        [z * x * k - y * s, z * y * k + x * s, c + z * z * k],
    ]
}

/// The bar `[0,1] × [0,w] × [0,w]`, extruded along `axis` from a profile
/// started at corner `start`, then turned when `turned`.
fn needle(w: f64, axis: usize, start: usize, turned: bool, tol: Tol) -> Body<f64> {
    let (prof, z1): ([(f64, f64); 4], f64) = match axis {
        // (y, z) square, extruded along x.
        0 => ([(0.0, 0.0), (w, 0.0), (w, w), (0.0, w)], 1.0),
        // (z, x) rectangle, extruded along y.
        1 => ([(0.0, 0.0), (w, 0.0), (w, 1.0), (0.0, 1.0)], w),
        // (x, y) rectangle, extruded along z.
        _ => ([(0.0, 0.0), (1.0, 0.0), (1.0, w), (0.0, w)], w),
    };
    let profile: Vec<(f64, f64)> = (0..4).map(|k| prof[(k + start) % 4]).collect();
    let r = rot([37.0, -21.0, 13.0], 0.7);
    let t = [0.3, -0.2, 0.5];
    let map = move |a: f64, b: f64, c: f64| {
        let p = match axis {
            0 => [c, a, b],
            1 => [b, c, a],
            _ => [a, b, c],
        };
        if turned {
            let at = |i: usize| r[i][0] * p[0] + r[i][1] * p[1] + r[i][2] * p[2] + t[i];
            Point3::new(at(0), at(1), at(2))
        } else {
            Point3::new(p[0], p[1], p[2])
        }
    };
    let mut body = Body::<f64>::new();
    topo::test_support::prism_ops(
        &mut body,
        &profile,
        (0.0, z1),
        map,
        topo::test_support::FaceGeometry::Certified,
        tol,
    );
    topo::test_support::describe_as_intersections(&mut body, tol);
    body
}

fn outcome(s: &str) -> String {
    s.split([' ', '{', '(']).filter(|t| !t.is_empty()).take(3).collect::<Vec<_>>().join(" ")
}

#[test]
#[ignore = "review probe"]
fn review_r2_needle_build_order() {
    let tol = Tol::witness();
    let band = Band::linear(tol).expect("band");
    let e = tol.eps();
    for m in NEEDLE {
        let w = m * e;
        for turned in [false, true] {
            for inverted in [false, true] {
                let mut seen = std::collections::BTreeMap::<String, Vec<String>>::new();
                let mut pis = std::collections::BTreeMap::<String, Vec<String>>::new();
                for axis in 0..3 {
                    for start in 0..4 {
                        let body = needle(w, axis, start, turned, tol);
                        let body = if inverted {
                            body.revert().expect("reverts")
                        } else {
                            body
                        };
                        for r in topo::test_support::aimed_rays(&body, band) {
                            seen.entry(outcome(&r))
                                .or_default()
                                .push(format!("{axis}/{start}"));
                        }
                        if !turned {
                            let mut row = Vec::new();
                            for q in [
                                Point3::new(0.5, 0.5 * w, 0.5 * w),
                                Point3::new(0.31, 0.3 * w, 0.62 * w),
                                Point3::new(0.5, w + 11.0 * e, 0.5 * w),
                                Point3::new(0.61, -11.0 * e, -11.0 * e),
                                Point3::new(1.5, 0.5 * w, 0.5 * w),
                                Point3::new(0.5, 0.5 * w, 3.0 * w + 30.0 * e),
                            ] {
                                let got = topo::point_in_solid(&body, q, band, tol);
                                row.push(outcome(&format!("{got:?}")));
                            }
                            pis.entry(row.join(", "))
                                .or_default()
                                .push(format!("{axis}/{start}"));
                        }
                    }
                }
                println!(
                    "R2ORD eps={e:e} m={m} turned={turned} inverted={inverted}: {} reading(s) {seen:?}",
                    seen.len()
                );
                if !turned {
                    println!(
                        "R2PIS eps={e:e} m={m} inverted={inverted}: {} row(s) {pis:?}",
                        pis.len()
                    );
                }
            }
        }
    }
}

/// `brick ∩ brick` needles in `shell_role_reader`'s four poses, both
/// operand orders, each result's aimed-ray outcome.
#[test]
#[ignore = "review probe"]
fn review_r2_boolean_needles_both_orders() {
    let tol = Tol::witness();
    let e = tol.eps();
    let poses: [([f64; 3], f64, [f64; 3]); 4] = [
        ([0.0, 0.0, 1.0], 0.0, [0.0; 3]),
        ([1.0, 2.0, 3.0], 1.1, [0.3, -0.2, 0.5]),
        ([0.3, -0.7, 0.2], 2.4, [1.7, 0.4, -0.9]),
        ([1.0, 1.0, 0.0], core::f64::consts::FRAC_PI_4, [0.0; 3]),
    ];
    for across in [1.5, 2.0, 2.5, 3.0, 4.0] {
        let w = across * tol.k() * e;
        for (i, &(axis, ang, c)) in poses.iter().enumerate() {
            let r = rot(axis, ang);
            let place = move |p: [f64; 3]| {
                let at = |k: usize| c[k] + r[k][0] * p[0] + r[k][1] * p[1] + r[k][2] * p[2];
                Point3::new(at(0), at(1), at(2))
            };
            let at_rest = |body| AtRestBody::validate(body, tol).expect("an operand at rest");
            let a = at_rest(topo::test_support::mapped_cube::<f64>(
                move |x, y, z| place([x, y, z]),
                tol,
            ));
            let b = at_rest(topo::test_support::mapped_cube::<f64>(
                move |x, y, z| place([x, 1.0 - w + y * (1.0 + w), 1.0 - w + z * (1.0 + w)]),
                tol,
            ));
            for (name, x, y) in [("a∩b", &a, &b), ("b∩a", &b, &a)] {
                let got = match topo::intersect(x, y, tol) {
                    Ok(result) => match result.body() {
                        Some(body) => {
                            let band = Band::linear(tol).expect("band");
                            let rays: Vec<String> = topo::test_support::aimed_rays(&body.body, band)
                                .iter()
                                .map(|s| outcome(s))
                                .collect();
                            format!(
                                "rays {rays:?} check7 {:?}",
                                topo::validate_geometric_structural(&body.body, tol)
                            )
                        }
                        None => "empty".to_owned(),
                    },
                    Err(err) => format!("refuses {}", outcome(&format!("{err:?}"))),
                };
                println!("R2BB eps={e:e} {across}Kε pose {i} {name}: {got}");
            }
        }
    }
}
