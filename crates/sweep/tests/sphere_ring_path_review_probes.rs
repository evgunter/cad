//! PR 4246 reviewer probes (scratch, not for merge): every sphere
//! pose's six ops, one outcome line each, written to `$RP_OUT` for a
//! base/head diff. They assert nothing and are `#[ignore]`d; run with
//! `RP_OUT=<dir> cargo nextest run -p sweep --run-ignored only sphere_ring_path_review_probes`
//! on two checkouts and diff the files.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;
use std::fmt::Write as _;

use geom_core::{Affine3, Point3, Tol, Vec3};
use sweep::test_support::{ball_poled, brick, finished};
use topo::{AtRestBody, BooleanDeclarations, BooleanResult};

type Bounds = [(f64, f64); 3];

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> f64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 >> 11) as f64 / (1u64 << 53) as f64
    }
    fn range(&mut self, a: f64, b: f64) -> f64 {
        a + (b - a) * self.next()
    }
    fn unit(&mut self) -> Vec3<f64> {
        loop {
            let v = Vec3::new(self.range(-1., 1.), self.range(-1., 1.), self.range(-1., 1.));
            let n = v.norm();
            if n > 0.2 && n < 1.0 {
                return v / n;
            }
        }
    }
}

fn quadrant(rho: f64, x: f64, y: f64) -> f64 {
    let s = |t: f64| (rho * rho - t * t).max(0.0).sqrt();
    let arc = |t: f64| {
        let t = t.clamp(-rho, rho);
        0.5 * (t * s(t) + rho * rho * (t / rho).asin()) + 0.25 * PI * rho * rho
    };
    let x = x.clamp(-rho, rho);
    let a = s(y);
    let inner = |lo: f64, hi: f64| {
        let (lo, hi) = (lo.max(-a), hi.min(a));
        if hi > lo { y * (hi - lo) + arc(hi) - arc(lo) } else { 0.0 }
    };
    let outer = |lo: f64, hi: f64| {
        if y < 0.0 {
            return 0.0;
        }
        let mut total = 0.0;
        for (p, q) in [(-rho, -a), (a, rho)] {
            let (p, q) = (p.max(lo), q.min(hi));
            if q > p {
                total += 2.0 * (arc(q) - arc(p));
            }
        }
        total
    };
    inner(-rho, x) + outer(-rho, x)
}

fn simpson(f: &dyn Fn(f64) -> f64, a: f64, b: f64) -> f64 {
    fn step(f: &dyn Fn(f64) -> f64, (a, b): (f64, f64), (fa, fm, fb): (f64, f64, f64), whole: f64, depth: u32) -> f64 {
        let (m, h) = (0.5 * (a + b), b - a);
        let (flm, frm) = (f(0.5 * (a + m)), f(0.5 * (m + b)));
        let left = h / 12.0 * (fa + 4.0 * flm + fm);
        let right = h / 12.0 * (fm + 4.0 * frm + fb);
        if depth == 0 || (left + right - whole).abs() <= 1e-15 {
            return left + right + (left + right - whole) / 15.0;
        }
        step(f, (a, m), (fa, flm, fm), left, depth - 1) + step(f, (m, b), (fm, frm, fb), right, depth - 1)
    }
    let (fa, fm, fb) = (f(a), f(0.5 * (a + b)), f(b));
    step(f, (a, b), (fa, fm, fb), (b - a) / 6.0 * (fa + 4.0 * fm + fb), 40)
}

/// Volume the unit ball at the origin shares with box `b`.
fn unit_in_box(b: Bounds) -> f64 {
    let (lo, hi) = (b[2].0.max(-1.0), b[2].1.min(1.0));
    if hi <= lo {
        return 0.0;
    }
    let area = |z: f64| {
        let rho = (1.0 - z * z).max(0.0).sqrt();
        if rho == 0.0 {
            return 0.0;
        }
        let f = |x: f64, y: f64| quadrant(rho, x, y);
        f(b[0].1, b[1].1) - f(b[0].0, b[1].1) - f(b[0].1, b[1].0) + f(b[0].0, b[1].0)
    };
    simpson(&area, lo, hi)
}

/// Volume the ball (c, r) shares with box `b`.
fn ball_in_box(c: [f64; 3], r: f64, b: Bounds) -> f64 {
    let n: Bounds = core::array::from_fn(|i| ((b[i].0 - c[i]) / r, (b[i].1 - c[i]) / r));
    r.powi(3) * unit_in_box(n)
}

fn box_volume(b: Bounds) -> f64 {
    b.iter().map(|(lo, hi)| hi - lo).product()
}

fn meet(a: Bounds, b: Bounds) -> Option<Bounds> {
    let m: Bounds = core::array::from_fn(|i| (a[i].0.max(b[i].0), a[i].1.min(b[i].1)));
    m.iter().all(|(lo, hi)| hi > lo).then_some(m)
}

fn cap(r: f64, h: f64) -> f64 {
    PI * h * h * (3.0 * r - h) / 3.0
}
fn lens(r1: f64, r2: f64, d: f64) -> f64 {
    if d >= r1 + r2 {
        return 0.0;
    }
    if d <= (r1 - r2).abs() {
        let r = r1.min(r2);
        return 4.0 / 3.0 * PI * r.powi(3);
    }
    let x = (d * d + r1 * r1 - r2 * r2) / (2.0 * d);
    cap(r1, r1 - x) + cap(r2, r2 - (d - x))
}

fn tol() -> Tol {
    Tol::witness()
}

fn boxed(b: Bounds) -> AtRestBody<f64> {
    finished("box", brick(b[0], b[1], b[2], tol()), tol())
}

fn ball(c: [f64; 3], r: f64, pole: Vec3<f64>) -> AtRestBody<f64> {
    finished("ball", ball_poled(r, Vec3::new(c[0], c[1], c[2]), pole, tol()), tol())
}

fn moved(b: &AtRestBody<f64>, m: &Affine3<f64>) -> AtRestBody<f64> {
    finished("moved", topo::transform_rigid(b, m, tol()).unwrap(), tol())
}

fn tag(e: &dyn core::fmt::Debug) -> String {
    let s = format!("{e:?}");
    let head: String = s.split(" {").next().unwrap_or("").chars().take(80).collect();
    let mut t = head;
    for key in ["kind: ", "what: \"", "site: "] {
        if let Some(i) = s.find(key) {
            let rest: String = s[i + key.len()..].chars().take(50).collect();
            let rest = rest.split([',', '}', '"']).next().unwrap_or("").to_string();
            let _ = write!(t, " {key}{rest}");
        }
    }
    t
}

struct Log(String, usize);

fn six(log: &mut Log, pose: &str, a: &AtRestBody<f64>, b: &AtRestBody<f64>, (va, vb, shared): (f64, f64, f64)) {
    let none = BooleanDeclarations::none();
    let t = tol();
    let ops: [(&str, f64, Box<dyn Fn() -> Result<BooleanResult<f64>, topo::BooleanError>>); 6] = [
        ("a∪b", va + vb - shared, Box::new(|| topo::union_with(a, b, &none, t))),
        ("b∪a", va + vb - shared, Box::new(|| topo::union_with(b, a, &none, t))),
        ("a∖b", va - shared, Box::new(|| topo::subtract_with(a, b, &none, t))),
        ("b∖a", vb - shared, Box::new(|| topo::subtract_with(b, a, &none, t))),
        ("a∩b", shared, Box::new(|| topo::intersect_with(a, b, &none, t))),
        ("b∩a", shared, Box::new(|| topo::intersect_with(b, a, &none, t))),
    ];
    for (op, want, run) in ops {
        log.1 += 1;
        let out = std::panic::catch_unwind(std::panic::AssertUnwindSafe(run));
        let line = match out {
            Err(_) => "PANIC".to_string(),
            Ok(Err(e)) => {
                if std::env::var("RP_VERBOSE").is_ok() {
                    eprintln!("{pose} {op}: {e:?}");
                }
                format!("ERR {}", tag(&e))
            }
            Ok(Ok(BooleanResult::Body(bb))) => {
                let t3 = topo::validate_geometric(&bb.body, t).is_ok();
                let t3p = match topo::validate_pseudomanifold(&bb.body, &bb.contacts, t) {
                    Ok(()) => "ok".to_string(),
                    Err(es) if es.iter().all(|e| matches!(e, topo::ValidationError::CensusUndecidable { .. })) => "census".into(),
                    Err(es) => format!("bad:{}", tag(&es[0])),
                };
                match topo::mass_properties(&bb.body, t) {
                    Ok(m) => {
                        let rel = (m.volume - want).abs() / want.abs().max(1e-300);
                        let ok = (m.volume - want).abs() <= 1e-9 * want.abs().max(1e-12) + 1e-12 * want.abs().max(1e-30).powf(1.0);
                        format!("BODY t3={t3} t3p={t3p} vol={:.10e} {} rel={rel:.1e}", m.volume, if ok { "VOK" } else { "VBAD" })
                    }
                    Err(e) => format!("BODY t3={t3} t3p={t3p} massERR {}", tag(&e)),
                }
            }
            Ok(Ok(other)) => format!("OTHER {:?}", other.body().map(|_| ()).is_some()),
        };
        let _ = writeln!(log.0, "{pose} | {op} | {line}");
    }
}

fn write(log: &Log, name: &str) {
    let dir = std::env::var("RP_OUT").unwrap_or_else(|_| "/tmp/rp".into());
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(format!("{dir}/{name}.txt"), &log.0).unwrap();
    eprintln!("{name}: {} ops", log.1);
}

fn rand_box_around(rng: &mut Rng, c: [f64; 3], r: f64) -> Bounds {
    core::array::from_fn(|i| {
        let a = c[i] + r * rng.range(-1.6, 0.9);
        let b = a + r * rng.range(0.15, 2.2);
        (a, b)
    })
}

/// Random balls with arbitrary poles against random boxes, at scales
/// and offsets, some poses turned rigidly.
#[test]
#[ignore = "reviewer evidence: writes outcome files, asserts nothing"]
fn rp_random_ball_box() {
    let mut log = Log(String::new(), 0);
    let mut rng = Rng(std::env::var("RP_SEED").ok().and_then(|s| s.parse().ok()).unwrap_or(0x9e3779b97f4a7c15));
    let n: usize = std::env::var("RP_N").ok().and_then(|s| s.parse().ok()).unwrap_or(400);
    for k in 0..n {
        let (s, off) = match k % 8 {
            0 => (1e-3, 0.0),
            1 => (1e3, 0.0),
            2 => (1.0, 1e3),
            3 => (1e-3, 1.0),
            _ => (1.0, 0.0),
        };
        let r = s * rng.range(0.5, 1.5);
        let c = [off + s * rng.range(-0.3, 0.3), s * rng.range(-0.3, 0.3), off * 0.5 + s * rng.range(-0.3, 0.3)];
        let pole = rng.unit();
        let b = rand_box_around(&mut rng, c, r);
        let shared = ball_in_box(c, r, b);
        let vb = 4.0 / 3.0 * PI * r.powi(3);
        let (mut bx, mut bl) = (boxed(b), ball(c, r, pole));
        if k % 3 == 2 {
            let m = Affine3::rotation_about_axis(Point3::new(c[0], c[1], c[2]), rng.unit(), rng.range(0.0, PI));
            bx = moved(&bx, &m);
            bl = moved(&bl, &m);
        }
        six(&mut log, &format!("rbb{k} s{s:e} off{off:e}"), &bx, &bl, (box_volume(b), vb, shared));
    }
    write(&log, "random_ball_box");
}

/// Random ball pairs.
#[test]
#[ignore = "reviewer evidence: writes outcome files, asserts nothing"]
fn rp_ball_pairs() {
    let mut log = Log(String::new(), 0);
    let mut rng = Rng(std::env::var("RP_SEED").ok().and_then(|s| s.parse().ok()).unwrap_or(0x2545f4914f6cdd1d));
    let np: usize = std::env::var("RP_PAIRS").ok().and_then(|s| s.parse().ok()).unwrap_or(120);
    for k in 0..np {
        let s = [1.0, 1e-3, 1e3][k % 3];
        let r1 = s * rng.range(0.5, 1.5);
        let r2 = s * rng.range(0.3, 1.5);
        let dir = rng.unit();
        let d = rng.range((r1 - r2).abs() * 1.02 / s, (r1 + r2) * 0.98 / s) * s;
        let c2 = [dir.x * d, dir.y * d, dir.z * d];
        let (p1, p2) = (rng.unit(), rng.unit());
        if std::env::var("RP_ONLY").is_ok_and(|v| v != k.to_string()) {
            continue;
        }
        eprintln!("pair{k}: r1={r1:e} r2={r2:e} c2={c2:?} p1={p1:?} p2={p2:?}");
        let a = ball([0.0; 3], r1, p1);
        let b = ball(c2, r2, p2);
        six(&mut log, &format!("pair{k} s{s:e}"), &a, &b, (4.0 / 3.0 * PI * r1.powi(3), 4.0 / 3.0 * PI * r2.powi(3), lens(r1, r2, d)));
    }
    write(&log, "ball_pairs");
}

/// Bars through the ball, random section and pole, three axes.
#[test]
#[ignore = "reviewer evidence: writes outcome files, asserts nothing"]
fn rp_bars() {
    let mut log = Log(String::new(), 0);
    let mut rng = Rng(0xdeadbeefcafef00d);
    for k in 0..150 {
        let axis = k % 3;
        let mut b: Bounds = [(0.0, 0.0); 3];
        for i in 0..3 {
            if i == axis {
                b[i] = (-2.0, 2.0);
            } else {
                let lo = rng.range(-0.75, 0.6);
                b[i] = (lo, lo + rng.range(0.08, 0.5));
            }
        }
        let pole = rng.unit();
        six(&mut log, &format!("bar{k} ax{axis}"), &boxed(b), &ball([0.0; 3], 1.0, pole), (box_volume(b), 4.0 / 3.0 * PI, ball_in_box([0.0; 3], 1.0, b)));
    }
    write(&log, "bars");
}

/// Non-convex notched tools: a box less a notch box, against a ball;
/// the review's far-pole notches, then random notches and poles.
#[test]
#[ignore = "reviewer evidence: writes outcome files, asserts nothing"]
fn rp_notched() {
    let mut log = Log(String::new(), 0);
    let mut rng = Rng(0x1234567887654321);
    let mut poses: Vec<(String, Bounds, Bounds, Vec3<f64>, f64)> = Vec::new();
    let big: Bounds = [(-2.0, 0.6), (-2.0, 2.0), (-2.0, 2.0)];
    for (name, notch) in [
        ("n1", [(0.4, 3.0), (0.5, 3.0), (-0.3, 0.3)]),
        ("n2", [(0.3, 3.0), (0.3, 3.0), (-0.25, 0.35)]),
        ("n3", [(0.45, 3.0), (-3.0, -0.4), (-0.2, 0.3)]),
    ] {
        for k in 0..6 {
            poses.push((format!("far {name} p{k}"), big, notch, rng.unit(), 0.0));
        }
    }
    for k in 0..220 {
        let cut = rng.range(-0.5, 0.8);
        let big: Bounds = [(-2.0, cut), (-2.0, 2.0), (-2.0, 2.0)];
        let notch: Bounds = [
            (cut - rng.range(0.05, 0.6), 3.0),
            if rng.next() < 0.5 { (rng.range(-0.8, 0.7), 3.0) } else { (-3.0, rng.range(-0.7, 0.8)) },
            {
                let lo = rng.range(-0.8, 0.5);
                (lo, lo + rng.range(0.1, 0.9))
            },
        ];
        poses.push((format!("rand{k}"), big, notch, rng.unit(), 0.0));
    }
    for (name, big, notch, pole, _) in poses {
        let t = tol();
        let Ok(r) = topo::subtract(&boxed(big), &boxed(notch), t) else { continue };
        let Some(tool) = r.body().map(|b| b.body.clone()) else { continue };
        let vt = topo::mass_properties(&tool, t).unwrap().volume;
        let shared = ball_in_box([0.0; 3], 1.0, big) - meet(big, notch).map_or(0.0, |m| ball_in_box([0.0; 3], 1.0, m));
        six(&mut log, &format!("notch {name}"), &tool, &ball([0.0; 3], 1.0, pole), (vt, 4.0 / 3.0 * PI, shared));
    }
    write(&log, "notched");
}

/// Two notches (a box less two boxes): the m2 shape, where a run arc
/// may dip across a section plane between its ends.
#[test]
#[ignore = "reviewer evidence: writes outcome files, asserts nothing"]
fn rp_double_notch() {
    let mut log = Log(String::new(), 0);
    let mut rng = Rng(0x0f0f0f0f12121212);
    for k in 0..200 {
        let big: Bounds = [(-2.0, rng.range(0.0, 0.85)), (-2.0, 2.0), (-2.0, rng.range(0.0, 0.85))];
        let n1: Bounds = [(rng.range(-0.6, 0.6), 3.0), (rng.range(-0.6, 0.6), 3.0), (rng.range(-0.8, 0.4), 3.0)];
        let n2: Bounds = [(rng.range(-0.6, 0.6), 3.0), (-3.0, rng.range(-0.6, 0.6)), (-3.0, rng.range(-0.4, 0.8))];
        let t = tol();
        let Ok(r) = topo::subtract(&boxed(big), &boxed(n1), t) else { continue };
        let Some(t1) = r.body().map(|b| b.body.clone()) else { continue };
        let Ok(r) = topo::subtract(&t1, &boxed(n2), t) else { continue };
        let Some(tool) = r.body().map(|b| b.body.clone()) else { continue };
        let vt = topo::mass_properties(&tool, t).unwrap().volume;
        let o = [0.0; 3];
        let m1 = meet(big, n1);
        let m2 = meet(big, n2);
        let m12 = m1.and_then(|m| meet(m, n2));
        let f = |m: Option<Bounds>| m.map_or(0.0, |m| ball_in_box(o, 1.0, m));
        let shared = ball_in_box(o, 1.0, big) - f(m1) - f(m2) + f(m12);
        six(&mut log, &format!("dn{k}"), &tool, &ball(o, 1.0, rng.unit()), (vt, 4.0 / 3.0 * PI, shared));
    }
    write(&log, "double_notch");
}

/// Targeted: a box face through the centre (great-circle sections),
/// near-tangent faces, corners, at scales and far offsets; m1 poses.
#[test]
#[ignore = "reviewer evidence: writes outcome files, asserts nothing"]
fn rp_targeted() {
    let mut log = Log(String::new(), 0);
    let mut rng = Rng(0x5555aaaa3333cccc);
    let poles = [Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0, 0.0, 1.0), Vec3::new(0.3, 0.8, 0.52).normalize(), Vec3::new(-0.6, 0.2, 0.77).normalize()];
    for &(s, off) in &[(1.0, 0.0), (1e-3, 0.0), (1e3, 0.0), (1.0, 1e3), (1e-3, 10.0)] {
        let c = [off, -off * 0.3, off * 0.7];
        let mut boxes: Vec<(String, Bounds)> = Vec::new();
        boxes.push(("great z".into(), [(-0.4, 0.5), (-0.3, 0.6), (0.0, 2.0)]));
        boxes.push(("great xz".into(), [(0.0, 2.0), (-0.5, 0.4), (0.0, 2.0)]));
        boxes.push(("corner origin".into(), [(0.0, 2.0), (0.0, 2.0), (0.0, 2.0)]));
        boxes.push(("corner in".into(), [(0.2, 2.0), (0.3, 2.0), (-0.1, 2.0)]));
        for e in [1e-2, 1e-4, 1e-6] {
            boxes.push((format!("near tangent {e:e}"), [(-0.3, 0.3), (-0.3, 0.3), (1.0 - e, 2.0)]));
            boxes.push((format!("near tangent corner {e:e}"), [(0.3, 2.0), (0.3, 2.0), (0.9 - e, 2.0)]));
        }
        boxes.push(("m1 bar a".into(), [(-2.0, 2.0), (-0.2, 0.25), (0.1, 0.4)]));
        boxes.push(("m1 bar b".into(), [(-0.3, 0.1), (0.2, 0.45), (-2.0, 2.0)]));
        boxes.push(("m1 random".into(), [(-0.6237172865476482, 1.233346104703386), (-0.5631024588741076, 0.346881547918797), (-0.7919341754260856, -0.23917581093071405)]));
        boxes.push(("slab".into(), [(-2.0, 2.0), (-2.0, 2.0), (0.3, 0.6)]));
        boxes.push(("slab great".into(), [(-2.0, 2.0), (-2.0, 2.0), (0.0, 0.6)]));
        for (name, b0) in boxes {
            let b: Bounds = core::array::from_fn(|i| (c[i] + s * b0[i].0, c[i] + s * b0[i].1));
            for (pi, &pole) in poles.iter().enumerate() {
                let shared = ball_in_box(c, s, b);
                six(&mut log, &format!("tg {name} s{s:e} off{off:e} pole{pi}"), &boxed(b), &ball(c, s, pole), (box_volume(b), 4.0 / 3.0 * PI * s.powi(3), shared));
            }
        }
    }
    // m1 poses exactly as the 4211 review pinned them.
    let pole = Vec3::new(0.6355369378990602, -0.7198546525528592, -0.2791094404779036);
    let b: Bounds = [(-0.6237172865476482, 1.233346104703386), (-0.5631024588741076, 0.346881547918797), (-0.7919341754260856, -0.23917581093071405)];
    six(&mut log, "m1 exact random", &boxed(b), &ball([0.0; 3], 1.0, pole), (box_volume(b), 4.0 / 3.0 * PI, ball_in_box([0.0; 3], 1.0, b)));
    let _ = rng.next();
    write(&log, "targeted");
}

/// Stepped tools (the m2 shape): a box whose top is `z0` for `x < a`
/// and `z2 > z0` beyond, so a side face's circle crosses the plane
/// `z = z0` where the `z0` face is absent.
#[test]
#[ignore = "reviewer evidence: writes outcome files, asserts nothing"]
fn rp_steps() {
    let mut log = Log(String::new(), 0);
    let mut rng = Rng(0x7777123499990001);
    let n: usize = std::env::var("RP_STEPS").ok().and_then(|s| s.parse().ok()).unwrap_or(300);
    for k in 0..n {
        let z0 = rng.range(-0.6, 0.6);
        let z2 = z0 + rng.range(0.1, 0.9);
        let a = rng.range(-0.7, 0.7);
        let bb = rng.range(-0.6, 0.8);
        let lo_y = if k % 2 == 0 { -2.0 } else { rng.range(-0.9, bb - 0.15) };
        let big: Bounds = [(-2.0, 2.0), (lo_y, bb), (-2.0, z2)];
        let notch: Bounds = [(-3.0, a), (-3.0, 3.0), (z0, 3.0)];
        let t = tol();
        let Ok(r) = topo::subtract(&boxed(big), &boxed(notch), t) else { continue };
        let Some(tool) = r.body().map(|b| b.body.clone()) else { continue };
        let vt = topo::mass_properties(&tool, t).unwrap().volume;
        let o = [0.0; 3];
        let shared = ball_in_box(o, 1.0, big) - meet(big, notch).map_or(0.0, |m| ball_in_box(o, 1.0, m));
        let pole = rng.unit();
        let mut bl = ball(o, 1.0, pole);
        let mut tl = tool;
        if k % 3 == 1 {
            let m = Affine3::rotation_about_axis(Point3::origin(), rng.unit(), rng.range(0.0, PI));
            bl = moved(&bl, &m);
            tl = moved(&tl, &m);
        }
        six(&mut log, &format!("step{k} z0={z0:.3} z2={z2:.3} a={a:.3} b={bb:.3} y0={lo_y:.3} pole={:.3},{:.3},{:.3}", pole.x, pole.y, pole.z), &tl, &bl, (vt, 4.0 / 3.0 * PI, shared));
    }
    write(&log, "steps");
}

/// Pair 82 (the first review's M-1 pose), its six ops with the full
/// error printed: run at each ε to see where the stand-down's
/// "escalates on its chart before any ring is read" holds.
#[test]
#[ignore = "reviewer evidence: prints outcomes, asserts nothing"]
fn rp2_pair82_verbose() {
    let (r1, r2) = (6.098088671076322e-4, 1.4613194916300017e-3);
    let c2 = [-0.0018727710410726642, -5.7762472824686515e-5, -3.412205707271761e-5];
    let p1 = Vec3::new(-0.20107294329337744, -0.891895479861727, -0.4050828612488538);
    let p2 = Vec3::new(0.01866131486960719, -0.9760726504487204, -0.21664241591467578);
    let a = ball([0.0; 3], r1, p1);
    let b = ball(c2, r2, p2);
    let d = (c2[0] * c2[0] + c2[1] * c2[1] + c2[2] * c2[2]).sqrt();
    let mut log = Log(String::new(), 0);
    // SAFETY-free: RP_VERBOSE makes six() print each error in full.
    six(&mut log, "pair82", &a, &b, (4.0 / 3.0 * PI * r1.powi(3), 4.0 / 3.0 * PI * r2.powi(3), lens(r1, r2, d)));
    eprintln!("eps {}\n{}", tol().eps(), log.0);
}
