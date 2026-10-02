//! Delta-review probe for PR 3801's fix pass (REACH). Not a cargo target:
//! copy to `crates/sweep/tests/` to run, e.g.
//!
//!     cp probes/reach_delta_3801.rs crates/sweep/tests/ && \
//!       cargo nextest run -p sweep --test reach_delta_3801 --no-capture
//!
//! Every row's oracle is derived here, independently of the PR's rows:
//!
//! - `tilted_slab_family`: the lens B(0,1) ∩ B((0,1.4,0),0.8) against a
//!   6×1×6 slab whose near plane is `n·p = s`, `n` the y axis tilted
//!   about x. lens ∩ slab is integrated along `n` as the area of two
//!   disks' overlap per slice (composite Simpson, 20000 panels), so it
//!   does not assume which face the plane cuts.
//! - `nested_family`: a ball inside the snowman / chain, or the lens
//!   inside a ball, off-axis, at scales 1e-3, 1 and 1e3, both orders,
//!   every op; nesting is checked by sampling the inner boundary, the
//!   volumes are cap closed forms.
//!
//! A row fails only on a WRONG body (invalid, or a volume off the
//! oracle); a typed refusal is printed and counted.

use core::f64::consts::PI;

use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use sweep::Revolution;
use sweep::test_support::revolved_about_y;
use topo::{Body, BooleanOp};

const OPS: [BooleanOp; 3] = [BooleanOp::Union, BooleanOp::Intersect, BooleanOp::Subtract];

fn ball(r: f64, c: Vec3<f64>) -> Body<f64> {
    let b = revolved_about_y(
        vec![(Point2::new(0.0, -r), 1.0), (Point2::new(0.0, r), 0.0)],
        Revolution::Full,
        Tol::witness(),
    );
    topo::transform_rigid(&b, &Affine3::translation(c), Tol::witness()).unwrap()
}

fn cap(r: f64, h: f64) -> f64 {
    PI * h * h * (3.0 * r - h) / 3.0
}
fn lens(r1: f64, r2: f64, d: f64) -> f64 {
    let x = (d * d + r1 * r1 - r2 * r2) / (2.0 * d);
    cap(r1, r1 - x) + cap(r2, r2 - (d - x))
}
fn vball(r: f64) -> f64 {
    4.0 / 3.0 * PI * r.powi(3)
}

fn op_run(op: BooleanOp, a: &Body<f64>, b: &Body<f64>) -> Result<Option<Body<f64>>, String> {
    match op {
        BooleanOp::Union => topo::boolean::union(a, b, Tol::witness()),
        BooleanOp::Intersect => topo::boolean::intersect(a, b, Tol::witness()),
        BooleanOp::Subtract => topo::boolean::subtract(a, b, Tol::witness()),
    }
    .map(|o| o.body().map(|b| b.body.clone()))
    .map_err(|e| format!("{e:?}"))
}

/// `Ok(volume)` for a valid closed body, `Err` with what failed.
fn check(body: &Body<f64>) -> Result<f64, String> {
    topo::validate(body).map_err(|e| format!("validate {e:?}"))?;
    topo::validate_closed(body).map_err(|e| format!("closed {e:?}"))?;
    topo::validate_geometric(body, Tol::witness()).map_err(|e| format!("geom {e:?}"))?;
    let p = topo::mass_properties(body, Tol::witness()).map_err(|e| format!("mass {e:?}"))?;
    Ok(p.volume)
}

/// Runs one row; panics-worthy rows are returned as `Some(line)`.
fn row(name: &str, op: BooleanOp, x: &Body<f64>, y: &Body<f64>, want: f64, rel: f64) -> (bool, bool, String) {
    match op_run(op, x, y) {
        Err(e) => (true, false, format!("{name}: REFUSED {}", &e[..e.len().min(160)])),
        Ok(None) => {
            let ok = want.abs() <= rel;
            (ok, true, format!("{name}: empty (want {want:.10e})"))
        }
        Ok(Some(b)) => match check(&b) {
            Err(e) => (false, true, format!("{name}: INVALID {e}")),
            Ok(v) => {
                let ok = (v - want).abs() <= rel * want.abs().max(f64::MIN_POSITIVE);
                (ok, true, format!("{name}: vol {v:.10e} want {want:.10e}"))
            }
        },
    }
}

// ---------------------------------------------------------------------
// Tilted slab.
// ---------------------------------------------------------------------

fn disk_overlap(d: f64, r1: f64, r2: f64) -> f64 {
    if r1 <= 0.0 || r2 <= 0.0 {
        return 0.0;
    }
    if d >= r1 + r2 {
        return 0.0;
    }
    if d <= (r1 - r2).abs() {
        return PI * r1.min(r2).powi(2);
    }
    let a1 = ((d * d + r1 * r1 - r2 * r2) / (2.0 * d * r1)).clamp(-1.0, 1.0).acos();
    let a2 = ((d * d + r2 * r2 - r1 * r1) / (2.0 * d * r2)).clamp(-1.0, 1.0).acos();
    r1 * r1 * (a1 - a1.sin() * a1.cos()) + r2 * r2 * (a2 - a2.sin() * a2.cos())
}

/// vol{p ∈ B(0,1) ∩ B(c2,r2) : n·p ≥ s} (the slab's far plane is past
/// the lens).
fn lens_beyond(n: Vec3<f64>, s: f64, c2: Vec3<f64>, r2: f64) -> f64 {
    let cn = n.dot(c2);
    let lateral = (c2 - n * cn).norm();
    let area = |t: f64| {
        let ra = (1.0 - t * t).max(0.0).sqrt();
        let rb = (r2 * r2 - (cn - t).powi(2)).max(0.0).sqrt();
        disk_overlap(lateral, ra, rb)
    };
    let (lo, hi) = (s, 1.0);
    if lo >= hi {
        return 0.0;
    }
    let m = 20000;
    let h = (hi - lo) / m as f64;
    let mut acc = area(lo) + area(hi);
    for i in 1..m {
        acc += area(lo + h * i as f64) * if i % 2 == 1 { 4.0 } else { 2.0 };
    }
    acc * h / 3.0
}

#[test]
fn tilted_slab_family() {
    let c2 = Vec3::new(0.0, 1.4, 0.0);
    let lens_b = op_run(BooleanOp::Intersect, &ball(1.0, Vec3::new(0.0, 0.0, 0.0)), &ball(0.8, c2))
        .unwrap()
        .unwrap();
    let vl = lens(1.0, 0.8, 1.4);
    let (mut bad, mut refused, mut built) = (Vec::new(), 0, 0);
    for tilt in [60.0f64, -60.0, 45.0, -45.0, 25.0, -25.0, 20.0, -20.0, 15.0, -15.0, 5.0] {
        for s in [0.985, 0.99, 0.97, 0.95] {
            let rot = Affine3::rotation_about_axis(
                Point3::origin(),
                Vec3::new(1.0, 0.0, 0.0),
                tilt.to_radians(),
            );
            let n = rot.transform_vec(Vec3::new(0.0, 1.0, 0.0));
            let slab0: Body<f64> =
                sweep::test_support::brick((-3.0, 3.0), (0.0, 1.0), (-3.0, 3.0), Tol::witness());
            let slab = topo::transform_rigid(&slab0, &(Affine3::translation(n * s) * rot), Tol::witness())
                .unwrap();
            let cut = lens_beyond(n, s, c2, 0.8);
            for (op, x, y, want, tag) in [
                (BooleanOp::Union, &lens_b, &slab, vl + 36.0 - cut, "L∪S"),
                (BooleanOp::Union, &slab, &lens_b, vl + 36.0 - cut, "S∪L"),
                (BooleanOp::Intersect, &lens_b, &slab, cut, "L∩S"),
                (BooleanOp::Intersect, &slab, &lens_b, cut, "S∩L"),
                (BooleanOp::Subtract, &lens_b, &slab, vl - cut, "L∖S"),
                (BooleanOp::Subtract, &slab, &lens_b, 36.0 - cut, "S∖L"),
            ] {
                let (ok, answered, line) =
                    row(&format!("tilt {tilt:>5} s {s} cut {cut:.4e} {tag}"), op, x, y, want, 1e-7);
                println!("{line}");
                if answered { built += 1 } else { refused += 1 }
                if !ok {
                    bad.push(line);
                }
            }
        }
    }
    println!("tilted slab: {built} answered, {refused} refused, {} wrong", bad.len());
    assert!(bad.is_empty(), "wrong bodies:\n{}", bad.join("\n"));
}

// ---------------------------------------------------------------------
// Nested balls, off-axis and scaled.
// ---------------------------------------------------------------------

fn fib(n: usize) -> Vec<Vec3<f64>> {
    let g = PI * (3.0 - 5f64.sqrt());
    (0..n)
        .map(|i| {
            let y = 1.0 - 2.0 * (i as f64 + 0.5) / n as f64;
            let r = (1.0 - y * y).sqrt();
            Vec3::new(r * (g * i as f64).cos(), y, r * (g * i as f64).sin())
        })
        .collect()
}

#[test]
fn nested_family() {
    let dirs = fib(4000);
    let (mut bad, mut refused, mut built) = (Vec::new(), 0, 0);
    for k in [1e-3, 1.0, 1e3] {
        let v = |x: f64, y: f64, z: f64| Vec3::new(x, y, z) * k;
        let (b1, b2, b3) = (
            (1.0 * k, v(0.0, 0.0, 0.0)),
            (0.8 * k, v(0.0, 1.4, 0.0)),
            (0.6 * k, v(0.0, 2.3, 0.0)),
        );
        let mk = |(r, c): (f64, Vec3<f64>)| ball(r, c);
        // The operands themselves are the crossing path's; a scale at
        // which the fixture cannot be built is reported and skipped.
        let setup = (|| -> Result<_, String> {
            let snow = op_run(BooleanOp::Union, &mk(b1), &mk(b2))?.ok_or("empty")?;
            let lensb = op_run(BooleanOp::Intersect, &mk(b1), &mk(b2))?.ok_or("empty")?;
            let chain = op_run(BooleanOp::Union, &snow, &mk(b3))?.ok_or("empty")?;
            Ok((snow, lensb, chain))
        })();
        let (snow, lensb, chain) = match setup {
            Ok(t) => t,
            Err(e) => {
                println!("k {k:e}: FIXTURE REFUSED {}", &e[..e.len().min(160)]);
                continue;
            }
        };
        let v_snow = vball(b1.0) + vball(b2.0) - lens(b1.0, b2.0, 1.4 * k);
        let v_lens = lens(b1.0, b2.0, 1.4 * k);
        let v_chain = v_snow + vball(b3.0) - lens(b2.0, b3.0, 0.9 * k);
        let in_ball = |p: Vec3<f64>, (r, c): (f64, Vec3<f64>)| (p - c).norm() < r - 1e-3 * k;
        // (label, big, its volume, inner?, small ball, membership)
        type Mem<'a> = Box<dyn Fn(Vec3<f64>) -> bool + 'a>;
        let cases: Vec<(&str, &Body<f64>, f64, (f64, Vec3<f64>), bool, Mem)> = vec![
            ("snowman ⊃ ball", &snow, v_snow, (0.5 * k, v(0.04, 0.75, -0.05)), true,
             Box::new(move |p| in_ball(p, b1) || in_ball(p, b2))),
            ("snowman ⊃ ball2", &snow, v_snow, (0.38 * k, v(-0.08, 0.8, 0.08)), true,
             Box::new(move |p| in_ball(p, b1) || in_ball(p, b2))),
            ("chain ⊃ ball", &chain, v_chain, (0.45 * k, v(0.05, 1.9, 0.03)), true,
             Box::new(move |p| in_ball(p, b1) || in_ball(p, b2) || in_ball(p, b3))),
            // The lens inside the ball: membership is the ball's, tested
            // on the lens's boundary (both caps sampled below).
            ("lens ⊂ ball", &lensb, v_lens, (0.65 * k, v(0.02, 0.8, 0.03)), false,
             Box::new(|_| true)),
            ("lens ⊂ ball big", &lensb, v_lens, (0.9 * k, v(0.1, 1.0, -0.1)), false,
             Box::new(|_| true)),
        ];
        for (label, big, v_big, (r, c), small_inside, mem) in cases {
            // Nesting, checked on the inner boundary.
            if small_inside {
                assert!(dirs.iter().all(|d| mem(c + *d * r)), "{label}: not nested");
            } else {
                let lens_pts = dirs.iter().flat_map(|d| {
                    [b1.1 + *d * b1.0, b2.1 + *d * b2.0]
                }).filter(|p| (*p - b1.1).norm() <= b1.0 * (1.0 + 1e-12) && (*p - b2.1).norm() <= b2.0 * (1.0 + 1e-12));
                let mut nonempty = false;
                for p in lens_pts {
                    nonempty = true;
                    assert!((p - c).norm() < r - 1e-3 * k, "{label}: lens not inside");
                }
                assert!(nonempty);
            }
            let small = ball(r, c);
            let vs = vball(r);
            let (inner, outer) = if small_inside { (vs, v_big) } else { (v_big, vs) };
            // (op, x, y, want): x = big, y = small and the swap.
            let big_minus_small = if small_inside { v_big - vs } else { 0.0 };
            let small_minus_big = if small_inside { 0.0 } else { vs - v_big };
            for op in OPS {
                for (x, y, want, tag) in [
                    (big, &small, match op {
                        BooleanOp::Union => outer,
                        BooleanOp::Intersect => inner,
                        BooleanOp::Subtract => big_minus_small,
                    }, "big·small"),
                    (&small, big, match op {
                        BooleanOp::Union => outer,
                        BooleanOp::Intersect => inner,
                        BooleanOp::Subtract => small_minus_big,
                    }, "small·big"),
                ] {
                    let (ok, answered, line) = row(
                        &format!("k {k:e} {label} {op:?} {tag}"),
                        op, x, y, want, 1e-9,
                    );
                    println!("{line}");
                    if answered { built += 1 } else { refused += 1 }
                    if !ok {
                        bad.push(line);
                    }
                }
            }
        }
    }
    println!("nested: {built} answered, {refused} refused, {} wrong", bad.len());
    assert!(bad.is_empty(), "wrong bodies:\n{}", bad.join("\n"));
}

// ---------------------------------------------------------------------
// Refusal reasons: a scraping pose, and a circle grazing the lens rim.
// ---------------------------------------------------------------------

/// Reads the refusal of a ball poking through the snowman's big face
/// off every edge (R-loop), and of balls whose circle on the unit
/// sphere passes `delta` outside the lens's rim (latitude acos 0.82857)
/// toward azimuth +z, clear of both balls' seams and poles.
#[test]
fn refusal_reasons() {
    let c2 = Vec3::new(0.0, 1.4, 0.0);
    let b1 = ball(1.0, Vec3::new(0.0, 0.0, 0.0));
    let snow = op_run(BooleanOp::Union, &b1, &ball(0.8, c2)).unwrap().unwrap();
    let lens_b = op_run(BooleanOp::Intersect, &b1, &ball(0.8, c2)).unwrap().unwrap();
    for (x, y, tag) in [
        (&snow, &ball(0.3, Vec3::new(0.0, 0.0, 1.0)), "snowman·poke"),
        (&ball(0.3, Vec3::new(0.0, 0.0, 1.0)), &snow, "poke·snowman"),
    ] {
        for op in OPS {
            println!("{tag} {op:?}: {:?}", op_run(op, x, y).map(|b| b.is_some()));
        }
    }
    let rim = (2.32f64 / 2.8).acos();
    let theta_c = 60f64.to_radians();
    let dd = 1.2f64;
    for delta in [1e-2, 1e-4, 1e-6, 1e-8, 1e-9, 1e-10, 0.0, -1e-10, -1e-8, -1e-6] {
        let rho = theta_c - rim - delta;
        let r0 = (1.0 + dd * dd - 2.0 * dd * rho.cos()).sqrt();
        let c = Vec3::new(0.0, theta_c.cos(), theta_c.sin()) * dd;
        let gap2 = (c - c2).norm() - r0 - 0.8;
        let small = ball(r0, c);
        // The true answer: does the small ball meet the lens? With the
        // circle outside the rim (delta > 0) and ball2 clear, it does
        // not, so ∩ is empty and ∪ is the sum.
        for op in OPS {
            for (x, y, tag) in [(&lens_b, &small, "lens·ball"), (&small, &lens_b, "ball·lens")] {
                let r = op_run(op, x, y);
                let s = match &r {
                    Err(e) => e[..e.len().min(200)].to_string(),
                    Ok(None) => "empty".into(),
                    Ok(Some(b)) => format!("{:?}", check(b)),
                };
                println!("graze δ {delta:e} (ball2 gap {gap2:.3}) {tag} {op:?}: {s}");
            }
        }
    }
}
