//! Review probe for PR 3817: ball pairs offset in the seam plane at
//! many radii, distances and directions, under every op. Each built
//! body is held to the lens closed form; then every sphere face's sense
//! bit is flipped alone and the outcome tallied: refused typed by the
//! cross-check (`SenseContradicted`), refused otherwise, or measured
//! (and whether tier 3 catches it).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;
use std::collections::BTreeMap;

use geom_core::{Affine3, Point2, Tol, Vec3};
use sweep::Revolution;
use sweep::test_support::revolved_about_y;
use topo::{Body, BooleanOp};

fn ball(r: f64, c: Vec3<f64>) -> Body<f64> {
    let b = revolved_about_y(
        vec![(Point2::new(0.0, -r), 1.0), (Point2::new(0.0, r), 0.0)],
        Revolution::Full,
        Tol::witness(),
    );
    topo::transform_rigid(&b, &Affine3::translation(c), Tol::witness()).unwrap()
}

fn cap_volume(r: f64, h: f64) -> f64 {
    PI * h.powi(2) * (3.0 * r - h) / 3.0
}

fn lens_volume(r1: f64, r2: f64, d: f64) -> f64 {
    let x = (d.powi(2) + r1.powi(2) - r2.powi(2)) / (2.0 * d);
    cap_volume(r1, r1 - x) + cap_volume(r2, r2 - (d - x))
}

fn run(op: BooleanOp, a: &Body<f64>, b: &Body<f64>) -> Result<topo::BooleanResult<f64>, topo::BooleanError> {
    match op {
        BooleanOp::Union => topo::boolean::union(a, b, Tol::witness()),
        BooleanOp::Intersect => topo::boolean::intersect(a, b, Tol::witness()),
        BooleanOp::Subtract => topo::boolean::subtract(a, b, Tol::witness()),
    }
}

#[test]
fn review_3817_flip_every_face() {
    let base = Vec3::new(2.0, 2.0, 0.5);
    let mut tally: BTreeMap<String, usize> = BTreeMap::new();
    let mut bump = |k: String| *tally.entry(k).or_default() += 1;
    let mut bad = Vec::new();
    let mut bodies = 0;
    for r2 in [0.45, 0.8, 1.0, 1.35] {
        for frac in [0.15, 0.5, 0.85] {
            let (lo, hi) = (f64::abs(1.0 - r2), 1.0 + r2);
            let d = lo + (hi - lo) * frac;
            for deg in [0.0_f64, 17.0, 33.0, 61.0, 118.0, 152.0, 199.0, 241.0, 307.0, 340.0] {
                let ph = deg.to_radians();
                let off = Vec3::new(d * ph.cos(), d * ph.sin(), 0.0);
                let a = ball(1.0, base);
                let b = ball(r2, base + off);
                let (va, vb) = (4.0 / 3.0 * PI, 4.0 / 3.0 * PI * r2.powi(3));
                let shared = lens_volume(1.0, r2, d);
                for (label, op, x, y, want) in [
                    ("union", BooleanOp::Union, &a, &b, va + vb - shared),
                    ("intersect", BooleanOp::Intersect, &a, &b, shared),
                    ("a-b", BooleanOp::Subtract, &a, &b, va - shared),
                    ("b-a", BooleanOp::Subtract, &b, &a, vb - shared),
                ] {
                    let pose = format!("r2 {r2} d {d:.4} {deg}° {label}");
                    let body = match run(op, x, y) {
                        Ok(out) => match out.body() {
                            Some(b) => b.body.clone(),
                            None => {
                                bump("build: empty".into());
                                continue;
                            }
                        },
                        Err(e) => {
                            let s = format!("{e:?}");
                            bump(format!("build refused: {}", &s[..s.len().min(90)]));
                            continue;
                        }
                    };
                    bodies += 1;
                    if topo::validate_geometric(&body, Tol::witness()) != Ok(()) {
                        bad.push(format!("{pose}: correct body fails tier 3: {:?}", topo::validate_geometric(&body, Tol::witness())));
                    }
                    let truth = match topo::mass_properties(&body, Tol::witness()) {
                        Ok(p) => {
                            if (p.volume - want).abs() > 1e-9 {
                                bad.push(format!("{pose}: volume {} vs {want}", p.volume));
                            }
                            p.volume
                        }
                        Err(e) => {
                            bad.push(format!("{pose}: correct body refused {e:?}"));
                            continue;
                        }
                    };
                    for (k, f) in body.faces() {
                        if !matches!(body.get_surface(f.surface), Some(geom::Surface::Sphere { .. })) {
                            continue;
                        }
                        let flipped = body.flipped_face_sense_for_tests(k).unwrap();
                        let t3 = topo::validate_geometric(&flipped, Tol::witness());
                        let t3s = match &t3 {
                            Ok(()) => "t3 GREEN".to_string(),
                            Err(v) if v.iter().any(|e| matches!(e, topo::ValidationError::CurvedSenseInverted { .. })) => "t3 CurvedSenseInverted".into(),
                            Err(v) => {
                                let s = format!("{:?}", v[0]);
                                format!("t3 {}", &s[..s.find(['{', '(']).unwrap_or(s.len())])
                            }
                        };
                        let mp = match topo::mass_properties(&flipped, Tol::witness()) {
                            Err(topo::MassPropsError::Face { source: geom_brep::props::PropsError::SenseContradicted, .. }) => "mp SenseContradicted".to_string(),
                            Err(e) => {
                                let s = format!("{e:?}");
                                format!("mp refused {}", &s[..s.len().min(60)])
                            }
                            Ok(p) => {
                                if (p.volume - truth).abs() > 1e-9 {
                                    if t3.is_ok() {
                                        bad.push(format!("{pose}: face {k:?} flipped measures {} (true {truth}) and tier 3 is green", p.volume));
                                    }
                                    "mp WRONG VOLUME".into()
                                } else {
                                    "mp same volume".into()
                                }
                            }
                        };
                        bump(format!("flip: {mp} / {t3s}"));
                    }
                }
            }
        }
    }
    println!("bodies built: {bodies}");
    for (k, v) in &tally {
        println!("{v:6}  {k}");
    }
    for b in &bad {
        println!("BAD {b}");
    }
    assert!(bad.is_empty(), "{} bad", bad.len());
}
