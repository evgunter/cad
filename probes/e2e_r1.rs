//! Reviewer e2e probe (PR 3847 dual, lane r1). Spliced into
//! crates/sweep/tests/all.rs as `#[path = "../../../probes/e2e_r1.rs"] mod e2e_r1;`
//! (needs snowman's helpers? no: self-contained). Near-tangent ball pairs
//! through the PUBLIC boolean API, widened: three radius pairs, three
//! scales, three depths, identity and a rotated+displaced pose, both
//! operand orders, every op, results reused as operands, and
//! point_in_solid sampled against the closed-form two-ball oracle.
//! Prints one line per case; asserts nothing (the summariser reads it).
use core::f64::consts::PI;
use geom_core::{Affine3, Band, Point2, Point3, Tol, Vec3};
use sweep::Revolution;
use sweep::test_support::revolved_about_y;
use topo::{Body, BooleanOp, SolidContainment};

fn ball(r: f64, y: f64) -> Body<f64> {
    revolved_about_y(
        vec![(Point2::new(0.0, y - r), 1.0), (Point2::new(0.0, y + r), 0.0)],
        Revolution::Full,
        Tol::witness(),
    )
}
fn cap(r: f64, h: f64) -> f64 { PI * h * h * (3.0 * r - h) / 3.0 }
fn lens(r1: f64, r2: f64, d: f64) -> f64 {
    let x = (d * d + r1 * r1 - r2 * r2) / (2.0 * d);
    cap(r1, r1 - x) + cap(r2, r2 - (d - x))
}
fn bv(r: f64) -> f64 { 4.0 / 3.0 * PI * r.powi(3) }

fn run(op: BooleanOp, a: &Body<f64>, b: &Body<f64>) -> Result<Body<f64>, String> {
    let t = Tol::witness();
    let out = match op {
        BooleanOp::Union => topo::boolean::union(a, b, t),
        BooleanOp::Intersect => topo::boolean::intersect(a, b, t),
        BooleanOp::Subtract => topo::boolean::subtract(a, b, t),
    };
    match out {
        Ok(o) => o.body().map(|b| b.body.clone()).ok_or_else(|| "EMPTY".to_string()),
        Err(e) => Err(format!("{e:?}").chars().take(60).collect::<String>().replace(' ', "_")),
    }
}

fn check(body: &Body<f64>, expected: f64, s: f64) -> String {
    let t = Tol::witness();
    if topo::validate(body).is_err() || topo::validate_closed(body).is_err() || topo::validate_geometric(body, t).is_err() {
        return "INVALID".into();
    }
    match topo::mass_properties(body, t) {
        Ok(p) => {
            let err = (p.volume - expected).abs();
            if err <= 1e-9 * expected.max(s.powi(3)) { "ok".into() } else { format!("VOLUME_OFF:{err:e}/{expected:e}") }
        }
        Err(_) => "MASSPROPS_ERR".into(),
    }
}

#[test]
fn emit() {
    let eps = Tol::witness().get().eps;
    let band = Band::linear(Tol::witness()).unwrap();
    let rot = |s: f64| Affine3::rotation_about_axis(Point3::new(0.3 * s, -1.2 * s, 2.1 * s), Vec3::new(1.0, 2.0, 3.0).normalize(), 0.7);
    for (r1u, r2u) in [(1.0, 0.8), (0.5, 2.0), (1.3, 1.3)] {
        for s in [1e-3, 1.0, 1e3] {
            for drel in [1e-4, 1e-5, 1e-6] {
                for rotated in [false, true] {
                    let (r1, r2, delta) = (r1u * s, r2u * s, drel * s);
                    let d = r1 + r2 - delta;
                    let (mut a, mut b) = (ball(r1, 0.0), ball(r2, d));
                    let map = rot(s);
                    let mut c = ball(0.6 * r1, -r1);
                    if rotated {
                        match (topo::transform_rigid(&a, &map, Tol::witness()), topo::transform_rigid(&b, &map, Tol::witness()), topo::transform_rigid(&c, &map, Tol::witness())) {
                            (Ok(x), Ok(y), Ok(z)) => { a = x; b = y; c = z; }
                            _ => { println!("E2E eps={eps:e} r=({r1u},{r2u}) s={s:e} drel={drel:e} rot=true TRANSFORM_REFUSED (not this PR)"); continue; }
                        }
                    }
                    let l = lens(r1, r2, d);
                    let (va, vb) = (bv(r1), bv(r2));
                    let mut cells = vec![];
                    let mut union = None;
                    for (name, op, x, y, want) in [
                        ("AuB", BooleanOp::Union, &a, &b, va + vb - l),
                        ("BuA", BooleanOp::Union, &b, &a, va + vb - l),
                        ("AnB", BooleanOp::Intersect, &a, &b, l),
                        ("BnA", BooleanOp::Intersect, &b, &a, l),
                        ("A-B", BooleanOp::Subtract, &a, &b, va - l),
                        ("B-A", BooleanOp::Subtract, &b, &a, vb - l),
                    ] {
                        let c = match run(op, x, y) {
                            Ok(body) => {
                                let v = check(&body, want, s);
                                if name == "AuB" { union = Some(body); }
                                v
                            }
                            Err(e) => format!("REFUSED:{e}"),
                        };
                        cells.push(format!("{name}={c}"));
                    }
                    // reuse: (A ∪ B) ∖ C with C a ball crossing A only; (A ∖ B) ∪ B = A ∪ B
                    let mut pis = (0usize, 0usize, 0usize);
                    if let Some(u) = &union {
                        let ac = lens(r1, 0.6 * r1, r1);
                        cells.push(format!("(AuB)-C={}", match run(BooleanOp::Subtract, u, &c) { Ok(x) => check(&x, va + vb - l - ac, s), Err(e) => format!("REFUSED:{e}") }));
                        cells.push(format!("(A-B)uB={}", match run(BooleanOp::Subtract, &a, &b).and_then(|amb| run(BooleanOp::Union, &amb, &b)) { Ok(x) => check(&x, va + vb - l, s), Err(e) => format!("REFUSED:{e}") }));
                        // point_in_solid on the union near the section circle,
                        // against the two-ball oracle (points nearer than
                        // 1e-3·δ to either sphere are skipped)
                        let h = delta * (2.0 * r2 - delta) / (2.0 * d);
                        let (ys, ra) = (r1 - h, (h * (2.0 * r1 - h)).sqrt());
                        for i in 0..400 {
                            let t = i as f64 * 0.7853;
                            let dy = ((i * 37 % 41) as f64 / 20.0 - 1.0) * 3.0 * h.max(delta);
                            let dr = ((i * 17 % 43) as f64 / 21.0 - 1.0) * 3.0 * ra.max(delta);
                            let rr = (ra + dr).abs();
                            let p = Point3::new(rr * t.cos(), ys + dy, rr * t.sin());
                            let d1 = (p.x.hypot(p.z)).hypot(p.y) - r1;
                            let d2 = (p.x.hypot(p.z)).hypot(p.y - d) - r2;
                            if d1.abs() < 1e-3 * delta || d2.abs() < 1e-3 * delta { continue; }
                            let inside = d1 < 0.0 || d2 < 0.0;
                            let q = if rotated { map.transform_point(p) } else { p };
                            match topo::point_in_solid(u, q, band, Tol::witness()) {
                                Ok(SolidContainment::In) if inside => pis.0 += 1,
                                Ok(SolidContainment::Out) if !inside => pis.0 += 1,
                                Ok(SolidContainment::OnBoundary) => pis.2 += 1,
                                Ok(_) => pis.1 += 1,
                                Err(_) => pis.2 += 1,
                            }
                        }
                    }
                    println!("E2E eps={eps:e} r=({r1u},{r2u}) s={s:e} drel={drel:e} rot={rotated} PIS(ok,WRONG,undecided)={pis:?} {}", cells.join(" "));
                }
            }
        }
    }
}
