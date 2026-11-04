//! Review probe (PR 6500): the new ψ against the old formula, bitwise.
use geom_core::{Bounds, Dual, Interval, Real, Vec3};
use geom::Quarter;
use super::{FramedCarrier, ProjectedChart, ProjectedImage, breaks_vector};
use crate::offset::Nappe;

fn old_quarter<T: Real>(m: i32, x: T, y: T) -> (T, T) {
    match m.rem_euclid(4) {
        0 => (x, y),
        1 => (y, T::zero() - x),
        2 => (T::zero() - x, T::zero() - y),
        _ => (T::zero() - y, x),
    }
}
fn old_branch<T: Real>(m: i32, x: T, y: T) -> T {
    let (xr, yr) = old_quarter(m, x, y);
    T::pi() * T::from_f64(0.5 * f64::from(m)) + yr.atan2(xr)
}
fn old_raw<T: Real>(img: &ProjectedImage<T>, p: Vec3<T>, k: usize) -> (T, T) {
    if matches!(img.chart, ProjectedChart::Plane) {
        return (p.x, p.y);
    }
    let sigma = img.chart.sigma();
    let theta = img.azimuth.get(k).copied().unwrap_or(0);
    let u = old_branch(theta, p.x * sigma, p.y * sigma);
    let rho = (p.x.powi(2) + p.y.powi(2)).sqrt();
    let v = match img.chart {
        ProjectedChart::Plane => unreachable!(),
        ProjectedChart::Cylinder => p.z,
        ProjectedChart::Cone { sin, cos, .. } => p.z * cos + sigma * rho * sin,
        ProjectedChart::Sphere => p.z.atan2(rho),
        ProjectedChart::Torus { major } => {
            let phi = img.tube.get(k).copied().unwrap_or(0);
            old_branch(phi, rho - major, p.z)
        }
    };
    (u, v)
}

thread_local! { static SIGNED_ZERO: std::cell::Cell<usize> = const { std::cell::Cell::new(0) }; static NUMEQ_OK: std::cell::Cell<bool> = const { std::cell::Cell::new(false) }; static NO_TINY: std::cell::Cell<bool> = const { std::cell::Cell::new(false) }; }
struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> f64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        ((self.0 >> 11) as f64) / ((1u64 << 53) as f64)
    }
    fn coord(&mut self) -> f64 {
        let r = self.next();
        let s = if self.next() < 0.5 { -1.0 } else { 1.0 };
        match (r * 10.0) as u32 {
            0 => s * 0.0,
            1 => if NO_TINY.with(|c| c.get()) { s * 1e-200 * self.next() + s * 1e-150 } else { s * 1e-300 * self.next() },
            2 => s * 1e-9 * self.next(),
            3 => s * 1e-15 * self.next(),
            _ => s * 3.0 * self.next(),
        }
    }
}

fn charts<T: Real>(lift: &impl Fn(f64) -> T) -> Vec<ProjectedChart<T>> {
    let mut v = vec![ProjectedChart::Plane, ProjectedChart::Cylinder, ProjectedChart::Sphere];
    for a in [0.05f64, 0.4, 1.3, 1.55] {
        for nappe in [Nappe::Opening, Nappe::Mirror] {
            v.push(ProjectedChart::Cone { sin: lift(a.sin()), cos: lift(a.cos()), nappe });
        }
    }
    for r in [2.0, 1.0, 0.5, 1e-9, 0.0] {
        v.push(ProjectedChart::Torus { major: lift(r) });
    }
    v
}

fn image<T: Real>(chart: ProjectedChart<T>, m: i32, n: i32) -> ProjectedImage<T> {
    ProjectedImage {
        chart,
        carrier: FramedCarrier::Line { point: Vec3::zero(), dir: Vec3::zero(), origin: T::zero(), span: T::one() },
        breaks: breaks_vector(&[0.0, 1.0]),
        azimuth: vec![m],
        tube: vec![n],
        u_off: T::zero(),
        v_off: T::zero(),
        v_sign: T::one(),
    }
}

fn run<T: Real>(lift: impl Fn(f64) -> T, point: impl Fn(&mut Lcg) -> Vec3<T>, bits: impl Fn(T) -> Vec<u64>, n_pts: usize, label: &str) {
    run2(lift, point, bits, |_: T| 0.0, n_pts, label, false)
}
fn run2<T: Real>(lift: impl Fn(f64) -> T, point: impl Fn(&mut Lcg) -> Vec3<T>, bits: impl Fn(T) -> Vec<u64>, rep: impl Fn(T) -> f64, n_pts: usize, label: &str, insector: bool) {
    let mut rng = Lcg(0x6500);
    let (mut reads, mut diffs, mut nonzero_diffs) = (0usize, 0usize, 0usize);
    let mut shown = 0;
    for chart in charts(&lift) {
        for _ in 0..n_pts {
            let p = point(&mut rng);
            for m in -4..8 {
                for n in [-4, -1, 0, 1, 2, 3, 7] {
                    let img = image(chart, m, n);
                    if insector {
                        let sg = match chart { ProjectedChart::Cone { nappe: Nappe::Mirror, .. } => -1.0, _ => 1.0 };
                        let (px, py, pz) = (rep(p.x), rep(p.y), rep(p.z));
                        let (rx, _) = old_quarter(m, sg * px, sg * py);
                        let plane = matches!(chart, ProjectedChart::Plane);
                        if !plane && !(rx > 1e-12) { continue; }
                        if let ProjectedChart::Torus { major } = chart {
                            let rho = (px * px + py * py).sqrt();
                            let (tx, _) = old_quarter(n, rho - rep(major), pz);
                            if !(tx > 1e-12) { continue; }
                        }
                    }
                    let new = img.raw(p, 0);
                    let old = old_raw(&img, p, 0);
                    reads += 1;
                    let (a, b) = ((bits(new.x), bits(new.y)), (bits(old.0), bits(old.1)));
                    if a != b {
                        diffs += 1;
                        // classify: only the sphere latitude at a ±0?
                        let numeq = { let f = |v: &Vec<u64>| v.iter().map(|&b| f64::from_bits(b)).collect::<Vec<_>>(); f(&a.0) == f(&b.0) && f(&a.1) == f(&b.1) };
                        if numeq { SIGNED_ZERO.with(|c| c.set(c.get() + 1)); }
                        let latzero = (matches!(chart, ProjectedChart::Sphere) && a.0 == b.0) || (NUMEQ_OK.with(|c| c.get()) && numeq);
                        if !latzero {
                            nonzero_diffs += 1;
                            if shown < 10 {
                                shown += 1;
                                eprintln!("{label} DIFF chart {:?} m {m} n {n} p {:?}: new {:?} old {:?}", chart.kind(), (bits(p.x), bits(p.y), bits(p.z)), a, b);
                            }
                        }
                    }
                }
            }
        }
    }
    eprintln!("{label}: numerically-equal (signed zero) diffs {}", SIGNED_ZERO.with(|c| c.get()));
    eprintln!("{label}: reads {reads}, differing {diffs}, differing outside the sphere latitude {nonzero_diffs}");
    assert_eq!(nonzero_diffs, 0, "{label}");
}

#[test]
fn probe_f64() {
    run(|x| x, |r| Vec3::new(r.coord(), r.coord(), r.coord()), |x: f64| vec![x.to_bits()], 20_000, "f64");
}

#[test]
fn probe_interval_points() {
    run(Interval::from_f64, |r| Vec3::new(Interval::from_f64(r.coord()), Interval::from_f64(r.coord()), Interval::from_f64(r.coord())), |x: Interval| { let (a, b, c) = x.repr_bits(); vec![a, b, u64::from(c)] }, 3_000, "interval-point");
}

#[test]
fn probe_interval_boxes() {
    let wide = |r: &mut Lcg| { let a = r.coord(); let w = [0.0, 1e-12, 1e-6, 0.3][(r.next() * 4.0) as usize]; Interval::from_bounds(a, a + w) };
    run(Interval::from_f64, |r| Vec3::new(wide(r), wide(r), wide(r)), |x: Interval| { let (a, b, c) = x.repr_bits(); vec![a, b, u64::from(c)] }, 3_000, "interval-box");
}

#[test]
fn probe_dual() {
    let d = |r: &mut Lcg| Dual::new(r.coord(), r.coord());
    run(Dual::constant, |r| Vec3::new(d(r), d(r), d(r)), |x: Dual<f64>| vec![x.value.to_bits(), x.deriv.to_bits()], 5_000, "dual");
}

#[test]
fn probe_quarter_unused() { let _ = Quarter(0); }

#[test]
fn probe_interval_mul_one() {
    let a = Interval::from_f64(f64::from_bits(4470727899158505855));
    let b = a * Interval::one();
    eprintln!("a {:?} a*1 {:?}", a.repr_bits(), b.repr_bits());
    let y = Interval::from_f64(f64::from_bits(9337297171089955258));
    eprintln!("atan2(y,a) {:?} atan2(y*1,a*1) {:?}", y.atan2(a).repr_bits(), (y * Interval::one()).atan2(b).repr_bits());
    let z = Interval::pi() * Interval::from_f64(0.0);
    eprintln!("pi*0 {:?}  0 + atan {:?}", z.repr_bits(), (z + y.atan2(a)).repr_bits());
}

#[test]
fn insector_f64() {
    run2(|x| x, |r| Vec3::new(r.coord(), r.coord(), r.coord()), |x: f64| vec![x.to_bits()], |x| x, 20_000, "f64-insector", true);
}
#[test]
fn insector_interval_boxes() {
    let wide = |r: &mut Lcg| { let a = r.coord(); let w = [0.0, 1e-12, 1e-6, 0.3][(r.next() * 4.0) as usize]; Interval::from_bounds(a, a + w) };
    run2(Interval::from_f64, |r| Vec3::new(wide(r), wide(r), wide(r)), |x: Interval| { let (a, b, c) = x.repr_bits(); vec![a, b, u64::from(c)] }, |x: Interval| x.lo(), 3_000, "interval-box-insector", true);
}
#[test]
fn insector_dual() {
    let d = |r: &mut Lcg| Dual::new(r.coord(), r.coord());
    run2(Dual::constant, |r| Vec3::new(d(r), d(r), d(r)), |x: Dual<f64>| vec![x.value.to_bits(), x.deriv.to_bits()], |x: Dual<f64>| x.value, 5_000, "dual-insector", true);
}

#[test]
fn insector_dual_numeq() {
    NUMEQ_OK.with(|c| c.set(true));
    let d = |r: &mut Lcg| Dual::new(r.coord(), r.coord());
    run2(Dual::constant, |r| Vec3::new(d(r), d(r), d(r)), |x: Dual<f64>| vec![x.value.to_bits(), x.deriv.to_bits()], |x: Dual<f64>| x.value, 5_000, "dual-insector-numeq", true);
}
#[test]
fn insector_interval_boxes_no_tiny() {
    NO_TINY.with(|c| c.set(true));
    let wide = |r: &mut Lcg| { let a = r.coord(); let w = [0.0, 1e-12, 1e-6, 0.3][(r.next() * 4.0) as usize]; Interval::from_bounds(a, a + w) };
    run2(Interval::from_f64, |r| Vec3::new(wide(r), wide(r), wide(r)), |x: Interval| { let (a, b, c) = x.repr_bits(); vec![a, b, u64::from(c)] }, |x: Interval| x.lo(), 3_000, "interval-box-insector-notiny", true);
}
#[test]
fn all_interval_boxes_no_tiny() {
    NO_TINY.with(|c| c.set(true));
    let wide = |r: &mut Lcg| { let a = r.coord(); let w = [0.0, 1e-12, 1e-6, 0.3][(r.next() * 4.0) as usize]; Interval::from_bounds(a, a + w) };
    run2(Interval::from_f64, |r| Vec3::new(wide(r), wide(r), wide(r)), |x: Interval| { let (a, b, c) = x.repr_bits(); vec![a, b, u64::from(c)] }, |x: Interval| x.lo(), 3_000, "interval-box-all-notiny", false);
}
#[test]
fn all_f64_numeq() {
    NUMEQ_OK.with(|c| c.set(true));
    run2(|x| x, |r| Vec3::new(r.coord(), r.coord(), r.coord()), |x: f64| vec![x.to_bits()], |x| x, 20_000, "f64-all-numeq", false);
}

#[test]
fn probe_box_encloses_eval() {
    use super::FrameBox;
    let mut rng = Lcg(0xb0c5);
    let (mut checked, mut bad, mut boxes) = (0usize, 0usize, 0usize);
    let ii = Interval::from_f64;
    for chart in charts(&|x| x) {
        let chart_i = match chart {
            ProjectedChart::Plane => continue,
            ProjectedChart::Cylinder => ProjectedChart::Cylinder,
            ProjectedChart::Sphere => ProjectedChart::Sphere,
            ProjectedChart::Cone { sin, cos, nappe } => ProjectedChart::Cone { sin: ii(sin), cos: ii(cos), nappe },
            ProjectedChart::Torus { major } => ProjectedChart::Torus { major: ii(major) },
        };
        let sg = match chart { ProjectedChart::Cone { nappe: Nappe::Mirror, .. } => -1.0, _ => 1.0 };
        for _ in 0..4000 {
            let m = (rng.next() * 12.0) as i32 - 4;
            let n = (rng.next() * 12.0) as i32 - 4;
            let w = [0.0, 1e-9, 1e-3, 0.5][(rng.next() * 4.0) as usize];
            let x0 = [0.0, 1e-300, 1e-12, 0.3][(rng.next() * 4.0) as usize] * rng.next() + [5e-324, 1e-15, 1e-6, 0.1][(rng.next() * 4.0) as usize];
            let y0 = rng.coord();
            let z0 = rng.coord();
            let b = FrameBox { x: (ii(x0), ii(x0 + w)), y: (ii(y0), ii(y0 + w)), z: (ii(z0), ii(z0 + w)) };
            let img_i = image(chart_i, m, n);
            if let ProjectedChart::Torus { major } = chart_i {
                let (tx, _) = img_i.tube_box(0, &b, major);
                if !(tx.0.lo() > 0.0) { continue; }
            }
            boxes += 1;
            let win = img_i.window(0, &b);
            let img = image(chart, m, n);
            for s in 0..9 {
                let pick = |lo: f64, j: u32| match j { 0 => lo, 1 => lo + w, _ => lo + w * rng_frac(s, j) };
                let (xr, yr, zr) = (pick(x0, s % 3), pick(y0, (s / 3) % 3), pick(z0, s % 2 + (s / 6)));
                let (xs, ys) = (sg * xr, sg * yr);
                let (x, y) = Quarter(-m).rotate(xs, ys);
                let p = Vec3::new(x, y, zr);
                let r = img.raw(p, 0);
                checked += 1;
                let inu = win.u_min.lo() <= r.x && r.x <= win.u_max.hi();
                let inv = win.v_min.lo() <= r.y && r.y <= win.v_max.hi();
                if !(inu && inv) {
                    bad += 1;
                    if bad < 8 {
                        eprintln!("OUT {:?} m {m} n {n} box x {:?} y {:?} z {:?} p {:?} raw {:?} win u [{}, {}] v [{}, {}]", chart.kind(), (x0, x0 + w), (y0, y0 + w), (z0, z0 + w), (x, y, zr), (r.x, r.y), win.u_min.lo(), win.u_max.hi(), win.v_min.lo(), win.v_max.hi());
                    }
                }
            }
        }
    }
    eprintln!("box probe: boxes {boxes}, points {checked}, outside {bad}");
    assert_eq!(bad, 0);
}
fn rng_frac(s: u32, j: u32) -> f64 { f64::from((s * 7 + j * 3) % 11) / 11.0 }
