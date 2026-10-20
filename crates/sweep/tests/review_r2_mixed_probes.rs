//! **PR 5422 review R2 probes**: point containment and check 7 on bodies
//! that mix planar and curved faces (a rounded box, a box with a boss, a
//! frustum on a box, a thin plate with a boss), run on the merge base
//! and on the frozen head and diffed. Evidence, not pins: run with
//! `cargo nextest run --release -p sweep --run-ignored only -E 'test(review_r2_mixed)' --no-capture`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::print_stdout)]

use crate::revolve_common::{axis_y, validated};
use geom_core::{Band, Point2, Point3, Tol};
use profile::{ProfileLoop, RawLoop, test_support::bulge_loop};
use sweep::{ExtrudeSide, Extrusion, Revolution, extrude, revolve};
use topo::{AtRestBody, Body};

const B90: f64 = 0.414_213_562_373_095_03; // tan(22.5°)

fn ext(lp: ProfileLoop<f64>, z0: f64, h: f64, tol: Tol) -> Body<f64> {
    let prof = profile::Profile::new(profile::SketchPlane::xy_at(Point3::new(0.0, 0.0, z0)), vec![lp])
        .validate(tol)
        .expect("a valid profile");
    extrude(&prof, Extrusion::Distance { depth: h, side: ExtrudeSide::Along }, tol)
        .expect("extrudes")
        .body
}

fn rounded_rect(x0: f64, y0: f64, x1: f64, y1: f64, r: f64) -> ProfileLoop<f64> {
    let p = |x: f64, y: f64| Point2::new(x, y);
    bulge_loop(vec![
        (p(x0 + r, y0), 0.0),
        (p(x1 - r, y0), B90),
        (p(x1, y0 + r), 0.0),
        (p(x1, y1 - r), B90),
        (p(x1 - r, y1), 0.0),
        (p(x0 + r, y1), B90),
        (p(x0, y1 - r), 0.0),
        (p(x0, y0 + r), B90),
    ])
}

fn circle(cx: f64, cy: f64, r: f64) -> ProfileLoop<f64> {
    bulge_loop(vec![(Point2::new(cx - r, cy), 1.0), (Point2::new(cx + r, cy), 1.0)])
}

fn at_rest(b: Body<f64>, tol: Tol) -> AtRestBody<f64> {
    AtRestBody::validate(b, tol).expect("an operand at rest")
}

fn bodies(tol: Tol) -> Vec<(String, Result<Body<f64>, String>, [f64; 6])> {
    let e = tol.eps();
    let mut out = Vec::new();
    out.push((
        "rounded box".to_owned(),
        Ok(ext(rounded_rect(0.0, 0.0, 2.0, 1.0, 0.3), 0.0, 0.7, tol)),
        [0.0, 0.0, 0.0, 2.0, 1.0, 0.7],
    ));
    let boss = |plate: f64| {
        let base = at_rest(topo::test_support::brick((0.0, 2.0), (0.0, 2.0), (0.0, plate), tol), tol);
        let post = at_rest(ext(circle(1.0, 1.0, 0.4), 0.5 * plate, 1.0, tol), tol);
        topo::union(&base, &post, tol)
            .map_err(|e| format!("{e:?}"))
            .map(|r| (*r.body().expect("a body").body).clone())
    };
    out.push(("box with boss".to_owned(), boss(1.0), [0.0, 0.0, 0.0, 2.0, 2.0, 1.5]));
    out.push((
        "30eps plate with boss".to_owned(),
        boss(30.0 * e),
        [0.0, 0.0, 0.0, 2.0, 2.0, 1.0 + 15.0 * e],
    ));
    let frustum = {
        let lp = ProfileLoop::polygon(
            [(0.0, 0.0), (0.6, 0.0), (0.3, 0.8), (0.0, 0.8)].map(|(x, y)| Point2::new(x, y)),
        );
        let f = revolve(&validated(vec![lp]), axis_y(), Revolution::Full, tol)
            .map(|r| r.body)
            .map_err(|e| format!("{e:?}"));
        f.and_then(|f| {
            let slab = at_rest(topo::test_support::brick((-1.0, 1.0), (-1.0, 0.1), (-1.0, 1.0), tol), tol);
            topo::union(&slab, &at_rest(f, tol), tol)
                .map_err(|e| format!("{e:?}"))
                .map(|r| (*r.body().expect("a body").body).clone())
        })
    };
    out.push(("frustum on slab".to_owned(), frustum, [-1.0, -1.0, -1.0, 1.0, 0.8, 1.0]));
    out
}

#[test]
#[ignore = "review probe"]
fn review_r2_mixed() {
    let tol = Tol::witness();
    let band = Band::linear(tol).expect("band");
    let e = tol.eps();
    for (name, body, bb) in bodies(tol) {
        let body = match body {
            Ok(b) => b,
            Err(err) => {
                println!("R2MIX eps={e:e} {name}: builds no body: {}", &err[..err.len().min(200)]);
                continue;
            }
        };
        println!(
            "R2MIX eps={e:e} {name}: check7 {:?}",
            topo::validate_geometric_structural(&body, tol)
        );
        let n = 9;
        let (mut tally, mut lines) = (std::collections::BTreeMap::new(), Vec::new());
        for i in 0..n {
            for j in 0..n {
                for k in 0..n {
                    let f = |a: usize, lo: f64, hi: f64, off: f64| {
                        let pad = 0.1 * (hi - lo).max(1e-3);
                        lo - pad + (hi - lo + 2.0 * pad) * ((a as f64 + off) / n as f64)
                    };
                    let q = Point3::new(
                        f(i, bb[0], bb[3], 0.3713),
                        f(j, bb[1], bb[4], 0.5291),
                        f(k, bb[2], bb[5], 0.4177),
                    );
                    let got = match topo::point_in_solid(&body, q, band, tol) {
                        Ok(s) => format!("{s:?}"),
                        Err(err) => {
                            let s = format!("{err:?}");
                            s.split(['{', '(']).next().unwrap_or("").to_owned()
                        }
                    };
                    *tally.entry(got.clone()).or_insert(0) += 1;
                    lines.push(format!("{i},{j},{k} {got}"));
                }
            }
        }
        println!("R2MIX eps={e:e} {name}: tally {tally:?}");
        // Points about the planar/curved seams, `k·ε` off them.
        let mut seam = Vec::new();
        for k in [-25.0, -12.0, -5.0, -1.0, 1.0, 5.0, 12.0, 25.0] {
            let o = k * e;
            let pts: Vec<Point3<f64>> = match name.as_str() {
                // The flat side y = 0 meets the corner arc at x = 1.7.
                "rounded box" => vec![
                    Point3::new(1.7, o, 0.35),
                    Point3::new(1.7 + 7.0 * e, o, 0.35),
                    Point3::new(1.7 - 7.0 * e, o, 0.35),
                    Point3::new(1.7 + o, 0.0 + 3.0 * e, 0.7 + 4.0 * e),
                    Point3::new(1.7 + o, 0.0 - 3.0 * e, 0.7 - 4.0 * e),
                ],
                // The boss rim at the plate's top, and its wall.
                "box with boss" | "30eps plate with boss" => {
                    let top = if name.starts_with("box") { 1.0 } else { 30.0 * e };
                    vec![
                        Point3::new(1.4 + o, 1.0, top + 4.0 * e),
                        Point3::new(1.4 + o, 1.0, top - 4.0 * e),
                        Point3::new(1.4 + 4.0 * e, 1.0, top + o),
                        Point3::new(1.0, 0.6 - o, top + 0.5 * top.min(0.5)),
                        Point3::new(0.3, 0.3, 0.5 * top + o.min(0.5 * top)),
                    ]
                }
                // The frustum's foot rim on the slab's top, y = 0.1.
                _ => vec![
                    Point3::new(0.6 - 0.1 * 0.375 + o, 0.1 + 4.0 * e, 0.0),
                    Point3::new(0.6 - 0.1 * 0.375 + o, 0.1 - 4.0 * e, 0.0),
                    Point3::new(0.0, 0.1 + o, 0.6 - 0.1 * 0.375 + 4.0 * e),
                ],
            };
            for q in pts {
                let got = match topo::point_in_solid(&body, q, band, tol) {
                    Ok(s) => format!("{s:?}"),
                    Err(err) => {
                        let s = format!("{err:?}");
                        s.split(['{', '(']).next().unwrap_or("").to_owned()
                    }
                };
                seam.push(got);
            }
        }
        println!("R2MIXSEAM eps={e:e} {name}: {seam:?}");

        for l in lines {
            println!("R2MIXPT eps={e:e} {name}: {l}");
        }
    }
}
