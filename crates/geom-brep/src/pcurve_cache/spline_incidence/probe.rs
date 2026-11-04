//! Review probe (PR 6498): f64 floor against an Interval-enclosed chart
//! point distance, at a tight band, big offsets, extreme weights, and
//! samples aimed at the patch corners, where box gap equals distance.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::NurbsSurface;
use geom_core::spline::KnotVector;
use geom_core::{Band, Bounds, Interval, Point3, Real, Vec3};

use super::{Net, Read};

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
    fn below(&mut self, n: usize) -> usize {
        ((self.next() * n as f64) as usize).min(n - 1)
    }
}

fn knots(rng: &mut Rng, p: usize) -> KnotVector {
    let mut interior: Vec<f64> = (0..rng.below(5)).map(|_| rng.range(0.05, 0.95)).collect();
    interior.sort_by(f64::total_cmp);
    let mut v = vec![0.0; p + 1];
    for x in interior {
        v.extend(core::iter::repeat_n(x, 1 + rng.below(p)));
    }
    v.extend(core::iter::repeat_n(1.0, p + 1));
    KnotVector::clamped(v, p).unwrap()
}

fn patch(rng: &mut Rng) -> (NurbsSurface<f64>, String) {
    let (pu, pv) = (1 + rng.below(4), 1 + rng.below(4));
    let (ku, kv) = (knots(rng, pu), knots(rng, pv));
    let (nu, nv) = (ku.control_count(), kv.control_count());
    let scale = 10f64.powf(rng.range(-3.0, 3.0));
    let off = 10f64.powf(rng.range(0.0, 6.0)) * scale.max(1.0);
    let offset = Vec3::new(rng.range(-1.0, 1.0), rng.range(-1.0, 1.0), rng.range(-1.0, 1.0)) * off;
    let mut control = Vec::new();
    let fold = rng.below(2) == 0;
    for i in 0..nu {
        for j in 0..nv {
            let (x, y) = (i as f64 / nu as f64, j as f64 / nv as f64);
            let z = if fold { rng.range(-1.0, 1.0) } else { 0.3 * (3.0 * x).sin() * (2.0 * y).cos() };
            control.push(Point3::new(x * scale, y * scale, z * scale) + offset);
        }
    }
    let spread = [0.0, 2.0, 4.0, 6.0, 8.0][rng.below(5)];
    let weights: Vec<f64> = (0..nu * nv).map(|_| 10f64.powf(rng.range(-spread, spread))).collect();
    let what = format!("deg ({pu},{pv}) scale {scale:e} off {off:e} spread {spread} fold {fold}");
    (NurbsSurface::new(ku, kv, control, weights).unwrap(), what)
}

#[test]
fn probe_f64_floor_at_tight_band() {
    let seed = std::env::var("PROBE_SEED").ok().and_then(|s| s.parse().ok()).unwrap_or(0x9e3779b97f4a7c15u64);
    let n: usize = std::env::var("PROBE_N").ok().and_then(|s| s.parse().ok()).unwrap_or(300);
    let bands = [Band::new(1e-12, 1e-11).unwrap(), Band::new(1e-9, 1e-8).unwrap()];
    let mut rng = Rng(seed);
    let (mut off, mut on, mut uns, mut bad, mut ibad) = (0, 0, 0, 0, 0);
    let mut worst = 0.0f64;
    for case in 0..n {
        let (chart, what) = patch(&mut rng);
        let lifted = chart.map_scalar(<Interval as Real>::from_f64);
        let band = bands[case % 2];
        // A corner or a random parameter.
        let (u, v) = match rng.below(3) {
            0 => ([0.0, 1.0][rng.below(2)], [0.0, 1.0][rng.below(2)]),
            _ => (rng.range(0.0, 1.0), rng.range(0.0, 1.0)),
        };
        let foot = chart.eval(u, v);
        // Outward from the net's centroid at a corner, else random.
        let cen = chart.control().iter().fold(Vec3::new(0.0, 0.0, 0.0), |a, p| a + (*p - Point3::origin()))
            / chart.control().len() as f64;
        let mut dir = (foot - Point3::origin()) - cen;
        if rng.below(2) == 0 || dir.norm() == 0.0 {
            dir = Vec3::new(rng.range(-1.0, 1.0), rng.range(-1.0, 1.0), rng.range(-1.0, 1.0));
        }
        let d = band.escalate() * 10f64.powf(rng.range(0.0, 3.0));
        let q = foot + dir.normalize() * d;
        let qi = q.map(<Interval as Real>::from_f64);
        let upper = (qi - lifted.eval(<Interval as Real>::from_f64(u), <Interval as Real>::from_f64(v))).norm().hi();
        match Net::of(&chart, band).expect("cuts").read(q, band) {
            Read::Off(floor) => {
                off += 1;
                if floor > upper {
                    bad += 1;
                    worst = worst.max((floor - upper) / band.zero());
                    eprintln!("VIOLATION case {case} {what}: floor {floor:e} > upper {upper:e} (band {:e})", band.zero());
                }
            }
            Read::On => on += 1,
            Read::Unsettled(_) => uns += 1,
        }
        if let Read::Off(floor) = Net::of(&lifted, band).expect("cuts").read(qi, band) {
            if floor.lo() > upper {
                ibad += 1;
                eprintln!("IVIOLATION case {case} {what}: {floor:?} > {upper:e}");
            }
        }
    }
    eprintln!("[probe] off {off} on {on} unsettled {uns} f64-bad {bad} (worst {worst:e} bands) interval-bad {ibad}");
    assert_eq!(bad + ibad, 0);
}

/// A tilted-free flat bilinear (or bicubic) patch far from the origin,
/// the sample just off a corner along the outward diagonal: the box gap
/// is the distance, so a floor charged too little reads past it.
#[test]
fn probe_corner_tight() {
    let mut rng = Rng(0x1234_5678_9abc_def1);
    let band = Band::new(1e-12, 1e-11).unwrap();
    let (mut off, mut bad) = (0, 0);
    let mut worst = 0.0f64;
    for case in 0..4000 {
        let p = 1 + rng.below(3);
        let ku = KnotVector::clamped([vec![0.0; p + 1], vec![1.0; p + 1]].concat(), p).unwrap();
        let n = p + 1;
        let scale = 10f64.powf(rng.range(2.0, 5.0));
        let o = Vec3::new(rng.range(-1.0, 1.0), rng.range(-1.0, 1.0), rng.range(-1.0, 1.0))
            * 10f64.powf(rng.range(0.0, 5.0));
        let mut control = Vec::new();
        for i in 0..n {
            for j in 0..n {
                control.push(Point3::new(
                    (i as f64 / p as f64 + rng.range(0.0, 0.01)) * scale,
                    (j as f64 / p as f64 + rng.range(0.0, 0.01)) * scale,
                    0.0,
                ) + o);
            }
        }
        // Keep the corner the box's corner.
        control[0] = Point3::new(o.x - 1e-3 * scale, o.y - 1e-3 * scale, o.z);
        let spread = rng.range(0.0, 3.0);
        let weights: Vec<f64> = (0..n * n).map(|_| 10f64.powf(rng.range(-spread, spread))).collect();
        let chart = NurbsSurface::new(ku.clone(), ku.clone(), control.clone(), weights).unwrap();
        let lifted = chart.map_scalar(<Interval as Real>::from_f64);
        let d = band.escalate() * rng.range(1.5, 20.0);
        let q = control[0] + Vec3::new(-1.0, -1.0, 1.0).normalize() * d;
        let qi = q.map(<Interval as Real>::from_f64);
        let upper = (qi - lifted.eval(<Interval as Real>::from_f64(0.0), <Interval as Real>::from_f64(0.0)))
            .norm()
            .hi();
        if let Read::Off(floor) = Net::of(&chart, band).unwrap().read(q, band) {
            off += 1;
            if floor > upper {
                bad += 1;
                worst = worst.max(floor - upper);
                if bad < 4 {
                    eprintln!("VIOLATION case {case} p {p} scale {scale:e} |o| {:e}: floor {floor:e} > {upper:e}", o.norm());
                }
            }
        }
    }
    eprintln!("[corner] off {off} bad {bad} worst excess {worst:e}");
    assert_eq!(bad, 0);
}

/// Off a straight boundary row (y, z constant there) of a knotted
/// rational patch: the box gap of the cell holding the foot is the
/// distance, and the cut's insertions round the row's points.
#[test]
fn probe_boundary_row_tight() {
    let mut rng = Rng(0x0dd_ba11_cafe_f00d);
    let band = Band::new(1e-12, 1e-11).unwrap();
    let (mut off, mut bad) = (0, 0);
    let mut worst = 0.0f64;
    for case in 0..3000 {
        let (pu, pv) = (2 + rng.below(3), 1 + rng.below(3));
        let (ku, kv) = (knots(&mut rng, pu), knots(&mut rng, pv));
        let (nu, nv) = (ku.control_count(), kv.control_count());
        let scale = 10f64.powf(rng.range(0.0, 4.0));
        let o = Vec3::new(rng.range(-1.0, 1.0), rng.range(-1.0, 1.0), rng.range(-1.0, 1.0))
            * 10f64.powf(rng.range(0.0, 4.0));
        let mut control = Vec::new();
        for i in 0..nu {
            for j in 0..nv {
                let x = (i as f64 / nu as f64) * scale;
                let (y, z) = if j == 0 { (0.0, 0.0) } else {
                    ((j as f64 / nv as f64) * scale, rng.range(-0.3, 0.3) * scale)
                };
                control.push(Point3::new(x, y, z) + o);
            }
        }
        let spread = [1.0, 2.0, 4.0, 8.0][rng.below(4)];
        let weights: Vec<f64> = (0..nu * nv).map(|_| 10f64.powf(rng.range(-spread, spread))).collect();
        let chart = NurbsSurface::new(ku, kv, control, weights).unwrap();
        let lifted = chart.map_scalar(<Interval as Real>::from_f64);
        let u = rng.range(0.0, 1.0);
        let foot = chart.eval(u, 0.0);
        let d = band.escalate() * rng.range(1.5, 50.0);
        // Outward of the row: -y, and z away from the net's z range.
        let q = foot + Vec3::new(0.0, -1.0, 0.0) * d;
        let qi = q.map(<Interval as Real>::from_f64);
        let upper = (qi - lifted.eval(<Interval as Real>::from_f64(u), <Interval as Real>::from_f64(0.0)))
            .norm()
            .hi();
        if let Read::Off(floor) = Net::of(&chart, band).unwrap().read(q, band) {
            off += 1;
            if floor > upper {
                bad += 1;
                worst = worst.max(floor - upper);
                if bad < 4 {
                    eprintln!("VIOLATION case {case} ({pu},{pv}) scale {scale:e} spread {spread}: floor {floor:e} > {upper:e}");
                }
            }
        }
    }
    eprintln!("[row] off {off} bad {bad} worst excess {worst:e}");
    assert_eq!(bad, 0);
}
