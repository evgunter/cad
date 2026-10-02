//! Reviewer probe for PR #3817 (lane reach-dual3817-r1): sections through
//! a ball's POLE (a run end on a vertex), and wedge operands (partial
//! revolves, incl. a reflex 270° wedge whose pole corners are reflex),
//! against a 3-D Monte Carlo of the set algebra on closed-form
//! membership predicates — never the kernel's props.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_core::{Affine3, Band, Point2, Point3, Tol, Vec3};
use sweep::Revolution;
use sweep::test_support::revolved_about_y;
use topo::{Body, BooleanOp, SolidContainment};

fn solid(r: f64, c: Vec3<f64>, rev: Revolution<f64>) -> Body<f64> {
    let b = revolved_about_y(
        vec![(Point2::new(0.0, -r), 1.0), (Point2::new(0.0, r), 0.0)],
        rev,
        Tol::witness(),
    );
    topo::transform_rigid(&b, &Affine3::translation(c), Tol::witness()).unwrap()
}

struct Lcg(u64);
impl Lcg {
    fn f(&mut self) -> f64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        ((self.0 >> 11) as f64) / ((1u64 << 53) as f64)
    }
}

/// Membership of a wedge `{|p−c| < r, azimuth ∈ [0, θ]}`, azimuth measured
/// about +y from `+x` with sign `s` (calibrated against point_in_solid).
fn in_wedge(p: Point3<f64>, c: Vec3<f64>, r: f64, theta: Option<f64>, s: f64) -> bool {
    let d = p - (Point3::origin() + c);
    if d.norm() >= r {
        return false;
    }
    let Some(th) = theta else { return true };
    let mut phi = (s * d.z).atan2(d.x);
    if phi < 0.0 {
        phi += 2.0 * PI;
    }
    phi <= th
}

fn calibrate(theta: f64) -> f64 {
    let c = Vec3::new(0.0, 0.0, 0.0);
    let w = solid(1.0, c, Revolution::Partial(theta));
    let band = Band::linear(Tol::witness()).unwrap();
    let mut rng = Lcg(3);
    let mut score = [0, 0];
    for _ in 0..200 {
        let p = Point3::new(2.0 * rng.f() - 1.0, 2.0 * rng.f() - 1.0, 2.0 * rng.f() - 1.0);
        let Ok(k) = topo::point_in_solid(&w, p, band, Tol::witness()) else { continue };
        let inside = k == SolidContainment::In;
        for (i, s) in [1.0, -1.0].into_iter().enumerate() {
            if in_wedge(p, c, 1.0, Some(theta), s) == inside {
                score[i] += 1;
            }
        }
    }
    eprintln!("calibration θ={theta}: {score:?}");
    if score[0] > score[1] { 1.0 } else { -1.0 }
}

#[test]
fn probe_poles_and_wedges() {
    let base = Vec3::new(2.0, 2.0, 0.5);
    let pole = |c: Vec3<f64>| c + Vec3::new(0.0, 1.0, 0.0);
    let s270 = calibrate(1.5 * PI);
    let s90 = calibrate(0.5 * PI);
    let mut rows: Vec<(String, f64, Option<f64>, f64, Vec3<f64>, f64)> = Vec::new();
    // (name, rA, wedge θ of A, sign, cB, rB)
    let through = |cb: Vec3<f64>| (pole(base) - cb).norm();
    rows.push(("B through A's north pole".into(), 1.0, None, 1.0, base + Vec3::new(1.2, 0.3, 0.0), through(base + Vec3::new(1.2, 0.3, 0.0))));
    rows.push(("B through both poles (meridian section)".into(), 1.0, None, 1.0, base + Vec3::new(1.0, 0.0, 0.0), 2.0_f64.sqrt()));
    rows.push(("B near A's pole".into(), 1.0, None, 1.0, base + Vec3::new(0.3, 1.0, 0.0), 0.5));
    for (th, s) in [(1.5 * PI, s270), (0.5 * PI, s90)] {
        let tag = if th > PI { "270°" } else { "90°" };
        rows.push((format!("{tag} wedge, B in seam plane +x"), 1.0, Some(th), s, base + Vec3::new(1.2, 0.2, 0.0), 0.7));
        rows.push((format!("{tag} wedge, B near the pole"), 1.0, Some(th), s, base + Vec3::new(0.4, 0.9, 0.0), 0.5));
        rows.push((format!("{tag} wedge, B through the pole"), 1.0, Some(th), s, base + Vec3::new(0.9, 0.4, 0.0), through(base + Vec3::new(0.9, 0.4, 0.0))));
        rows.push((format!("{tag} wedge, B at -x"), 1.0, Some(th), s, base + Vec3::new(-1.2, 0.2, 0.0), 0.7));
    }
    let mut fails = Vec::new();
    for (name, ra, th, s, cb, rb) in rows {
        let a = solid(ra, base, th.map_or(Revolution::Full, Revolution::Partial));
        let b = solid(rb, cb, Revolution::Full);
        for (lab, op, x, y, first_is_a) in [
            ("∪", BooleanOp::Union, &a, &b, true),
            ("∩", BooleanOp::Intersect, &a, &b, true),
            ("A∖B", BooleanOp::Subtract, &a, &b, true),
            ("B∖A", BooleanOp::Subtract, &b, &a, false),
        ] {
            let res = match op {
                BooleanOp::Union => topo::boolean::union(x, y, Tol::witness()),
                BooleanOp::Intersect => topo::boolean::intersect(x, y, Tol::witness()),
                BooleanOp::Subtract => topo::boolean::subtract(x, y, Tol::witness()),
            };
            let out = match res {
                Err(e) => {
                    eprintln!("{name} {lab}: REFUSED {e:?}");
                    continue;
                }
                Ok(o) => o,
            };
            let Some(bb) = out.body() else {
                eprintln!("{name} {lab}: EMPTY");
                continue;
            };
            let body = &bb.body;
            let valid = topo::validate(body).is_ok()
                && topo::validate_closed(body).is_ok()
                && topo::validate_geometric(body, Tol::witness()).is_ok();
            let vol = topo::mass_properties(body, Tol::witness()).map(|p| p.volume);
            // 3-D MC over the bounding box of both.
            let lo = Vec3::new(base.x.min(cb.x - rb) - ra, base.y.min(cb.y - rb) - ra, base.z.min(cb.z - rb) - ra);
            let hi = Vec3::new((base.x + ra).max(cb.x + rb), (base.y + ra).max(cb.y + rb), (base.z + ra).max(cb.z + rb));
            let bx = hi - lo;
            let boxv = bx.x * bx.y * bx.z;
            let n = 400_000;
            let mut rng = Lcg(17);
            let mut hit = 0usize;
            for _ in 0..n {
                let p = Point3::origin() + lo + Vec3::new(rng.f() * bx.x, rng.f() * bx.y, rng.f() * bx.z);
                let ia = in_wedge(p, base, ra, th, s);
                let ib = in_wedge(p, cb, rb, None, 1.0);
                let inn = match (op, first_is_a) {
                    (BooleanOp::Union, _) => ia || ib,
                    (BooleanOp::Intersect, _) => ia && ib,
                    (BooleanOp::Subtract, true) => ia && !ib,
                    (BooleanOp::Subtract, false) => ib && !ia,
                };
                hit += usize::from(inn);
            }
            let pr = hit as f64 / n as f64;
            let mcv = pr * boxv;
            let sd = boxv * (pr * (1.0 - pr) / n as f64).sqrt();
            let line = format!("{name} {lab}: valid={valid} kernel={vol:?} mc={mcv:.5} sd={sd:.1e}");
            eprintln!("{line}");
            match vol {
                Ok(v) if valid && (v - mcv).abs() <= 5.0 * sd + 1e-6 => {}
                _ => fails.push(line),
            }
        }
    }
    assert!(fails.is_empty(), "FAILS:\n{}", fails.join("\n"));
}

/// `topo::split` of a y-poled ball by a plane tilted against its chart:
/// `chord_spec`'s wall lane is shared with the split, so the run-side
/// rule now serves it too. Each part against the cap closed form.
#[test]
fn probe_tilted_split_of_a_ball() {
    let base = Vec3::new(2.0, 2.0, 0.5);
    let ball = solid(1.0, base, Revolution::Full);
    for (n, h) in [
        (Vec3::new(1.0, 0.0, 0.0), 0.3),
        (Vec3::new(1.0, 0.5, 0.0), -0.2),
        (Vec3::new(0.3, 1.0, 0.7), 0.4),
        (Vec3::new(0.0, 0.0, 1.0), 0.1),
        (Vec3::new(1.0, 1.0, 0.0), 0.0),
    ] {
        let nn = n / n.norm();
        let origin = Point3::origin() + base + nn * h;
        let plane = topo::test_support::split_plane(origin, nn, Tol::witness());
        let cap = |hh: f64| PI * hh * hh * (3.0 - hh) / 3.0;
        match topo::split(&ball, &plane, Tol::witness()) {
            Err(e) => eprintln!("split n={n:?} h={h}: REFUSED {e:?}"),
            Ok(r) => {
                for (name, part, want) in [("above", &r.above, cap(1.0 - h)), ("below", &r.below, cap(1.0 + h))] {
                    let Some(b) = part.body() else {
                        eprintln!("split n={n:?} h={h} {name}: EMPTY");
                        continue;
                    };
                    let valid = topo::validate(b).is_ok()
                        && topo::validate_closed(b).is_ok()
                        && topo::validate_geometric(b, Tol::witness()).is_ok();
                    let vol = topo::mass_properties(b, Tol::witness()).map(|p| p.volume);
                    eprintln!("split n={n:?} h={h} {name}: tier3={valid:?} vol={vol:?} want={want}");
                }
            }
        }
    }
}
