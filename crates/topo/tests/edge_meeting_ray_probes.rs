//! **Probes of the edge-meeting ray** (`work/flush/a-ray-meets-an-edge-in-band-as-half-a-crossing-for-each-face.md`):
//! what each aimed ray of the side reader reads on the needle class, and
//! what point containment reads inside and beside a needle. Evidence for
//! review, not pins: run with
//! `cargo nextest run -p topo --run-ignored only edge_meeting_ray_probes --no-capture`
//! (and `CAD_TOLERANCE_EPS=1e-6` for the other row).

#![allow(clippy::expect_used, clippy::panic, clippy::print_stdout)]

use geom_core::{Band, Point3, Tol};
use topo::{AtRestBody, Body};

use crate::common;

const NEEDLE: [f64; 5] = [10.5, 12.0, 15.0, 21.0, 30.0];

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

/// The bar `1 × w × w`, along the axes or turned as
/// `shell_role_reader`'s turned needle is.
fn needle(w: f64, turned: bool, tol: Tol) -> Body<f64> {
    if !turned {
        return common::brick::<f64>((0.0, 1.0), (0.0, w), (0.0, w), tol);
    }
    let r = rot([37.0, -21.0, 13.0], 0.7);
    let t = [0.3, -0.2, 0.5];
    topo::test_support::mapped_cube::<f64>(
        move |a, b, c| {
            let p = [a, b * w, c * w];
            let at = |i: usize| r[i][0] * p[0] + r[i][1] * p[1] + r[i][2] * p[2] + t[i];
            Point3::new(at(0), at(1), at(2))
        },
        tol,
    )
}

fn short(s: &str) -> String {
    s.chars().take(900).collect()
}

/// Each needle, either way out: every aimed ray's reading.
#[test]
#[ignore = "probe: run by hand"]
fn probe_needles_aimed_rays() {
    let tol = Tol::witness();
    for m in NEEDLE {
        let w = m * tol.eps();
        for turned in [false, true] {
            for inverted in [false, true] {
                let body = needle(w, turned, tol);
                let body = if inverted {
                    body.revert().expect("reverts")
                } else {
                    body
                };
                for r in topo::test_support::aimed_rays(
                    &body,
                    Band::linear(tol).expect("the run's band"),
                ) {
                    println!(
                        "EMR needle eps={:e} m={m} turned={turned} inverted={inverted}: {}",
                        tol.eps(),
                        short(&r)
                    );
                }
            }
        }
    }
}

/// `brick ∩ brick` leaving a needle 2Kε and 3Kε across, in
/// `shell_role_reader`'s four poses.
#[test]
#[ignore = "probe: run by hand"]
fn probe_boolean_needles_aimed_rays() {
    let tol = Tol::witness();
    let poses: [([f64; 3], f64, [f64; 3]); 4] = [
        ([0.0, 0.0, 1.0], 0.0, [0.0; 3]),
        ([1.0, 2.0, 3.0], 1.1, [0.3, -0.2, 0.5]),
        ([0.3, -0.7, 0.2], 2.4, [1.7, 0.4, -0.9]),
        ([1.0, 1.0, 0.0], core::f64::consts::FRAC_PI_4, [0.0; 3]),
    ];
    for across in [2.0, 3.0] {
        let w = across * tol.k() * tol.eps();
        for (i, &(axis, ang, c)) in poses.iter().enumerate() {
            let r = rot(axis, ang);
            let place = move |p: [f64; 3]| {
                let at = |k: usize| c[k] + r[k][0] * p[0] + r[k][1] * p[1] + r[k][2] * p[2];
                Point3::new(at(0), at(1), at(2))
            };
            let at_rest = |body| AtRestBody::validate(body, tol).expect("an operand at rest");
            let a = at_rest(common::mapped_cube::<f64>(
                move |x, y, z| place([x, y, z]),
                tol,
            ));
            let b = at_rest(common::mapped_cube::<f64>(
                move |x, y, z| place([x, 1.0 - w + y * (1.0 + w), 1.0 - w + z * (1.0 + w)]),
                tol,
            ));
            let what = format!("eps={:e} {across}Kε pose {i}", tol.eps());
            match topo::intersect(&a, &b, tol) {
                Ok(result) => match result.body() {
                    Some(body) => {
                        for r in topo::test_support::aimed_rays(
                            &body.body,
                            Band::linear(tol).expect("the run's band"),
                        ) {
                            println!("EMR brick∩brick {what}: {}", short(&r));
                        }
                    }
                    None => println!("EMR brick∩brick {what}: empty"),
                },
                Err(e) => println!(
                    "EMR brick∩brick {what}: refuses {}",
                    short(&format!("{e:?}"))
                ),
            }
        }
    }
}

/// Point containment inside, beside and beyond each needle: the
/// boolean's containment fallback over a needle operand.
#[test]
#[ignore = "probe: run by hand"]
fn probe_needles_point_in_solid() {
    let tol = Tol::witness();
    let band = Band::linear(tol).expect("the run's band");
    for m in NEEDLE {
        let w = m * tol.eps();
        let body = needle(w, false, tol);
        let e = tol.eps();
        for (name, q) in [
            ("mid", Point3::new(0.5, 0.5 * w, 0.5 * w)),
            ("off-centre", Point3::new(0.31, 0.3 * w, 0.62 * w)),
            ("beside", Point3::new(0.5, w + 11.0 * e, 0.5 * w)),
            ("above", Point3::new(0.37, 0.5 * w, w + 11.0 * e)),
            ("off-corner", Point3::new(0.61, -11.0 * e, -11.0 * e)),
            ("far", Point3::new(0.5, 0.5 * w, 3.0 * w + 30.0 * e)),
            ("beyond", Point3::new(1.5, 0.5 * w, 0.5 * w)),
        ] {
            let bracket = geom_core::k_stats::Bracket::open();
            let got = topo::point_in_solid(&body, q, band, tol);
            let log = bracket.finish();
            let mut in_band: Vec<&str> = log
                .escalations
                .iter()
                .filter_map(|x| x.source.predicate)
                .collect();
            in_band.sort_unstable();
            in_band.dedup();
            let aimed = log
                .verdicts
                .iter()
                .filter(|v| v.predicate.starts_with("side_at_infinity"))
                .count();
            let volume = log
                .verdicts
                .iter()
                .filter(|v| v.predicate == "bool_point_in_solid_infinity")
                .count();
            println!(
                "EMR pis eps={e:e} m={m} {name}: {} | in band on {} rows {in_band:?} | aimed verdicts {aimed} | volume verdicts {volume}",
                short(&format!("{got:?}")),
                log.escalations.len(),
            );
        }
    }
}

/// One needle, `EMR_M` ε across, inside out: each aimed ray's reading.
#[test]
#[ignore = "probe: run by hand"]
fn probe_one_needle() {
    let tol = Tol::witness();
    let m: f64 = std::env::var("EMR_M")
        .ok()
        .and_then(|x| x.parse().ok())
        .unwrap_or(30.0);
    let body = needle(m * tol.eps(), false, tol).revert().expect("reverts");
    for r in topo::test_support::aimed_rays(&body, Band::linear(tol).expect("the run's band")) {
        println!("EMR one needle m={m}: {}", short(&r));
    }
}
