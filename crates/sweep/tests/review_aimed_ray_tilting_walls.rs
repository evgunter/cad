//! Reviewer probe (second review of PR 6958): extrude-door prisms whose
//! PROFILE vertex moves with the box, so side walls tilt across it (the
//! PR's door rows box only a height or radius). No ray may read `In`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Band, Interval, ParamSymbol, Point2, Point3, Real, Sym, SymBudget, Tol, Vec3};
use profile::test_support::bulge_loop;
use profile::{Profile, SketchPlane};
use sweep::{ExtrudeSide, Extrusion, extrude};

type Boxed = Sym<Interval>;
fn c(x: f64) -> Boxed {
    Boxed::from_f64(x)
}

fn build(kind: u8, p: Boxed, tol: Tol) -> Result<topo::Body<Boxed>, String> {
    let pts: Vec<(Boxed, Boxed)> = match kind {
        0 => vec![
            (c(0.0), c(0.0)),
            (c(1.0), c(0.0)),
            (c(1.0), p),
            (c(0.0), c(1.0)),
        ],
        1 => vec![(c(0.0), c(0.0)), (c(1.0), c(0.0)), (p, c(1.0))],
        _ => vec![
            (c(0.0), c(0.0)),
            (p, c(0.0)),
            (c(1.0), c(1.0)),
            (c(0.0), c(1.0)),
        ],
    };
    let verts = pts
        .into_iter()
        .map(|(x, y)| (Point2::new(x, y), c(0.0)))
        .collect();
    let profile = Profile::new(SketchPlane::<Boxed>::xy(), vec![bulge_loop(verts)])
        .validate(tol)
        .map_err(|e| format!("{e:?}"))?;
    extrude(
        &profile,
        Extrusion::Distance {
            depth: c(1.0),
            side: ExtrudeSide::Along,
        },
        tol,
    )
    .map(|b| b.body.into_body())
    .map_err(|e| format!("{e:?}"))
}

fn kind(r: &str) -> &'static str {
    if r.contains("Side(In)") {
        "IN"
    } else if r.contains("Side(Out)") {
        "out"
    } else if r.contains("SetAside") {
        "aside"
    } else {
        "other"
    }
}

#[test]
fn review_no_ray_about_a_prism_with_a_moving_profile_vertex_reads_in() {
    let mut rng = test_utils::fuzz::start("review_tilting_walls");
    let eps = Tol::witness().eps();
    for shape in 0..3u8 {
        let mid = match shape {
            0 => 1.0,
            1 => 0.5,
            _ => 1.0,
        };
        let mut tally = std::collections::BTreeMap::<String, usize>::new();
        for half in [1e-6, 1e-4, 1e-3, 1e-2, 0.05, 0.2] {
            let (lo, hi) = (mid - half, mid + half);
            let draws: Vec<([f64; 3], [f64; 3])> = (0..50)
                .map(|j| {
                    let aim = if j % 2 == 0 {
                        [
                            rng.range(-0.3, 1.3),
                            rng.range(-0.3, 1.3),
                            rng.range(-0.3, 1.3),
                        ]
                    } else {
                        let m = rng.range(lo, hi);
                        let u = rng.unit();
                        let z = if rng.below(3) == 0 {
                            [0.0, 1.0][rng.below(2)]
                        } else {
                            rng.unit()
                        };
                        match shape {
                            0 => [1.0 - u, m + u * (1.0 - m), z],
                            1 => [1.0 + u * (m - 1.0), u, z],
                            _ => [m + u * (1.0 - m), u, z],
                        }
                    };
                    let normal = match rng.below(6) {
                        0 => [1.0, 0.0, 0.0],
                        1 => [0.0, 0.0, 1.0],
                        _ => {
                            let v = [
                                rng.range(-1.0, 1.0),
                                rng.range(-1.0, 1.0),
                                rng.range(-1.0, 1.0),
                            ];
                            let n = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt().max(1e-3);
                            [v[0] / n, v[1] / n, v[2] / n]
                        }
                    };
                    (aim, normal)
                })
                .collect();
            let tol = Tol::witness();
            let band = Band::linear(tol).unwrap();
            let read = geom_core::sym::with_session(
                SymBudget {
                    max_terms: 64,
                    max_degree: 8,
                },
                || {
                    let p = Sym::param_over(
                        ParamSymbol::new(test_utils::symbol_id("review_profile_vertex")),
                        Interval::from_bounds(lo, hi),
                        lo,
                        hi,
                    );
                    build(shape, p, tol).map(|body| {
                        let face = body.faces().next().unwrap().0;
                        let mut out = topo::test_support::each_aim(&body, band);
                        out.extend(draws.iter().map(|&(a, n)| {
                            topo::test_support::ray_at(
                                &body,
                                face,
                                Point3::new(c(a[0]), c(a[1]), c(a[2])),
                                Vec3::new(c(n[0]), c(n[1]), c(n[2])),
                                band,
                            )
                        }));
                        out
                    })
                },
            )
            .0;
            let reads = match read {
                Ok(r) => r,
                Err(e) => {
                    println!(
                        "ε {eps:e} shape {shape} ±{half}: door refuses: {}",
                        &e[..e.len().min(120)]
                    );
                    continue;
                }
            };
            let mut t = std::collections::BTreeMap::<&str, usize>::new();
            for r in &reads {
                *t.entry(kind(r)).or_default() += 1;
                *tally.entry(kind(r).to_string()).or_default() += 1;
                assert!(
                    kind(r) != "IN",
                    "shape {shape} over [{lo},{hi}] ε {eps:e}: {r}"
                );
            }
            println!("ε {eps:e} shape {shape} ±{half}: {t:?}");
        }
        println!("ε {eps:e} shape {shape} total {tally:?}");
    }
}
