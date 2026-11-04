//! Delta probe (PR 6498, not for merge): how much of the f64 charge each
//! Off floor leaves between itself and a rigorous upper bound on the
//! distance, `(upper − floor)/(floor_uncharged − floor)`; plus a JSON dump
//! of every Off for the exact-rational checker. Run single-threaded.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, missing_docs)]

use geom::NurbsSurface;
use geom_core::spline::KnotVector;
use geom_core::{Band, Bounds, Interval, Point3, Real, Vec3};

use super::{Net, Read};

type Rng = test_utils::fuzz::Rng;

thread_local! {
    static NOCUT_FLOOR: std::cell::Cell<f64> = const { std::cell::Cell::new(0.0) };
    pub(super) static NO_CUT: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    pub(super) static RATIO: std::cell::Cell<f64> = const { std::cell::Cell::new(0.0) };
    pub(super) static NO_CHARGE: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

fn lift(x: f64) -> Interval {
    <Interval as Real>::from_f64(x)
}

fn knots(rng: &mut Rng, p: usize, nint: usize, mult1: bool) -> KnotVector {
    let mut interior: Vec<f64> = (0..nint).map(|_| rng.range(0.02, 0.98)).collect();
    interior.sort_by(f64::total_cmp);
    interior.dedup();
    let mut v = vec![0.0; p + 1];
    for x in interior {
        let m = if mult1 { 1 } else { 1 + rng.below(p) };
        v.extend(core::iter::repeat_n(x, m));
    }
    v.extend(core::iter::repeat_n(1.0, p + 1));
    KnotVector::clamped(v, p).unwrap()
}

fn read_both(chart: &NurbsSurface<f64>, q: Point3<f64>, band: Band) -> (Read<f64>, Option<f64>) {
    let net = Net::of(chart, band).unwrap();
    let charged = net.read(q, band);
    NO_CHARGE.with(|c| c.set(true));
    let raw = match net.read(q, band) {
        Read::Off(f) => Some(f),
        _ => None,
    };
    NO_CHARGE.with(|c| c.set(false));
    NO_CUT.with(|c| c.set(true));
    let nocut = match net.read(q, band) {
        Read::Off(f) => Some(f),
        _ => None,
    };
    NO_CUT.with(|c| c.set(false));
    NOCUT_FLOOR.with(|c| c.set(nocut.unwrap_or(f64::NAN)));
    (charged, raw)
}

fn dump(case: usize, chart: &NurbsSurface<f64>, u: f64, v: f64, q: Point3<f64>, f: f64, band: Band) {
    if std::env::var("DELTA_JSON").is_err() {
        return;
    }
    let hx = |x: f64| format!("\"{}\"", x.to_bits());
    let ku: Vec<String> = chart.knots_u_f64_interim().knots().iter().map(|&x| hx(x)).collect();
    let kv: Vec<String> = chart.knots_v_f64_interim().knots().iter().map(|&x| hx(x)).collect();
    let cp: Vec<String> = chart.control().iter().map(|p| format!("[{},{},{}]", hx(p.x), hx(p.y), hx(p.z))).collect();
    let w: Vec<String> = chart.weights().iter().map(|&x| hx(x)).collect();
    let (nu, nv) = chart.control_counts();
    eprintln!("F64JSON {{\"case\":{case},\"pu\":{},\"pv\":{},\"nu\":{nu},\"nv\":{nv},\"ku\":[{}],\"kv\":[{}],\"cp\":[{}],\"w\":[{}],\"u\":{},\"v\":{},\"q\":[{},{},{}],\"floor\":{},\"esc\":{}}}",
        chart.knots_u_f64_interim().degree(), chart.knots_v_f64_interim().degree(),
        ku.join(","), kv.join(","), cp.join(","), w.join(","), hx(u), hx(v), hx(q.x), hx(q.y), hx(q.z), hx(f), hx(band.escalate()));
}

#[derive(Debug, Default)]
struct Slack {
    off: usize,
    bad: usize,
    min_ratio: f64,
    min_abs: f64,
    nocut_bad: usize,
}

impl Slack {
    fn new() -> Self {
        Self { min_ratio: f64::INFINITY, min_abs: f64::INFINITY, ..Default::default() }
    }
    fn see(&mut self, case: usize, chart: &NurbsSurface<f64>, (u, v): (f64, f64), q: Point3<f64>, band: Band) {
        let upper = (q.map(lift) - chart.map_scalar(lift).eval(lift(u), lift(v))).norm().hi();
        let (r, raw) = read_both(chart, q, band);
        let nc = NOCUT_FLOOR.with(std::cell::Cell::get);
        if nc > upper {
            self.nocut_bad += 1;
        }
        if let Read::Off(f) = r {
            self.off += 1;
            dump(case, chart, u, v, q, f, band);
            if f > upper {
                self.bad += 1;
                eprintln!("FALSE OFF case {case}: floor {f:e} > {upper:e}");
            }
            self.min_abs = self.min_abs.min(upper - f);
            if let Some(raw) = raw {
                let charge = raw - f;
                if charge > 0.0 {
                    self.min_ratio = self.min_ratio.min((upper - f) / charge);
                }
            }
        }
    }
}

/// Knotted, high degree, mult-one interior knots (the most insertions),
/// weights 8 decades, placed far; samples off the straight boundary row
/// and along the normal at a random interior parameter, at the tight band.
#[test]
fn delta_knotted_slack() {
    let mut rng = test_utils::fuzz::start("delta-knotted");
    let n: usize = std::env::var("DELTA_N").ok().and_then(|s| s.parse().ok()).unwrap_or(1000);
    let band = Band::new(1e-12, 1e-11).unwrap();
    let (mut row, mut nrm) = (Slack::new(), Slack::new());
    for case in 0..n {
        let (pu, pv) = (3 + rng.below(3), 1 + rng.below(5));
        let (nku, nkv, m1) = (3 + rng.below(8), rng.below(6), rng.below(2) == 0);
        let (ku, kv) = (knots(&mut rng, pu, nku, true), knots(&mut rng, pv, nkv, m1));
        let (nu, nv) = (ku.control_count(), kv.control_count());
        let scale = 10f64.powf(rng.range(0.0, 4.0));
        let o = Vec3::new(rng.range(-1.0, 1.0), rng.range(-1.0, 1.0), rng.range(-1.0, 1.0)) * scale * [0.0, 10.0, 1e3, 1e5][rng.below(4)];
        // A tilt so that no row is axis aligned except where kept.
        let tilt = rng.below(2) == 0;
        let mut control = Vec::new();
        for i in 0..nu {
            for j in 0..nv {
                let x = i as f64 / nu as f64 + rng.range(-0.02, 0.02);
                let (y, z) = if j == 0 { (0.0, 0.0) } else { (j as f64 / nv as f64, rng.range(-0.3, 0.3)) };
                let p = Vec3::new(x, y, z);
                let p = if tilt { Vec3::new(0.6 * p.x + 0.8 * p.z, p.y, -0.8 * p.x + 0.6 * p.z) } else { p };
                control.push(Point3::origin() + p * scale + o);
            }
        }
        let spread = [1.0, 2.0, 4.0, 8.0][rng.below(4)];
        let weights = (0..nu * nv).map(|_| 10f64.powf(rng.range(-spread / 2.0, spread / 2.0))).collect();
        let chart = NurbsSurface::new(ku, kv, control, weights).unwrap();
        let u = rng.range(0.0, 1.0);
        let d = band.escalate() * 10f64.powf(rng.range(0.0, 1.5));
        if !tilt {
            let q = chart.eval(u, 0.0) + Vec3::new(0.0, -1.0, 0.0) * d;
            row.see(case, &chart, (u, 0.0), q, band);
        }
        let v = rng.range(0.0, 1.0);
        let h = 1e-6;
        let du = chart.eval((u + h).min(1.0), v) - chart.eval((u - h).max(0.0), v);
        let dv = chart.eval(u, (v + h).min(1.0)) - chart.eval(u, (v - h).max(0.0));
        let nn = du.cross(dv);
        if nn.norm() > 0.0 && nn.norm().is_finite() {
            let q = chart.eval(u, v) + nn.normalize() * d;
            nrm.see(case, &chart, (u, v), q, band);
        }
    }
    eprintln!("[delta] worst cut/read over floors {}", RATIO.with(std::cell::Cell::get));
    eprintln!("[delta-knotted] row {row:?}\n[delta-knotted] normal {nrm:?}");
    assert_eq!(row.bad + nrm.bad, 0);
}

/// The R1 corner and row families re-run with the slack measured.
#[test]
fn delta_corner_slack() {
    let mut rng = test_utils::fuzz::start("delta-corner");
    let n: usize = std::env::var("DELTA_N").ok().and_then(|s| s.parse().ok()).unwrap_or(1000);
    let band = Band::new(1e-12, 1e-11).unwrap();
    let mut s = Slack::new();
    for case in 0..n {
        let p = 1 + rng.below(5);
        let k = KnotVector::clamped([vec![0.0; p + 1], vec![1.0; p + 1]].concat(), p).unwrap();
        let n = p + 1;
        let scale = 10f64.powf(rng.range(2.0, 5.0));
        let o = Vec3::new(rng.range(-1.0, 1.0), rng.range(-1.0, 1.0), rng.range(-1.0, 1.0)) * 10f64.powf(rng.range(0.0, 6.0));
        let mut control = Vec::new();
        for i in 0..n {
            for j in 0..n {
                control.push(Point3::new((i as f64 / p as f64 + rng.range(0.0, 0.01)) * scale, (j as f64 / p as f64 + rng.range(0.0, 0.01)) * scale, 0.0) + o);
            }
        }
        control[0] = Point3::new(o.x - 1e-3 * scale, o.y - 1e-3 * scale, o.z);
        let spread = rng.range(0.0, 4.0);
        let weights = (0..n * n).map(|_| 10f64.powf(rng.range(-spread, spread))).collect();
        let corner = control[0];
        let chart = NurbsSurface::new(k.clone(), k, control, weights).unwrap();
        let q = corner + Vec3::new(-1.0, -1.0, 1.0).normalize() * (band.escalate() * rng.range(1.01, 20.0));
        s.see(case, &chart, (0.0, 0.0), q, band);
    }
    eprintln!("[delta] worst cut/read over floors {}", RATIO.with(std::cell::Cell::get));
    eprintln!("[delta-corner] {s:?}");
    assert_eq!(s.bad, 0);
}

/// Dump knotted patches' depth-0 cells (the insertions alone) with their
/// tracked bounds, for `refine_check.py` to compare against the exact
/// Bézier extraction in rationals.
#[test]
fn delta_dump_refined_cells() {
    let mut rng = test_utils::fuzz::start("delta-refine");
    let n: usize = std::env::var("DELTA_N").ok().and_then(|s| s.parse().ok()).unwrap_or(200);
    let band = Band::new(1e-12, 1e-11).unwrap();
    let hx = |x: f64| format!("\"{}\"", x.to_bits());
    for case in 0..n {
        let pv = 2 + rng.below(4);
        let nk = 2 + rng.below(8);
        let m1 = rng.below(2) == 0;
        let kv = knots(&mut rng, pv, nk, m1);
        let ku = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let nv = kv.control_count();
        let scale = 10f64.powf(rng.range(-1.0, 4.0));
        let o = Vec3::new(rng.range(-1.0, 1.0), rng.range(-1.0, 1.0), rng.range(-1.0, 1.0)) * scale * [0.0, 10.0, 1e3, 1e5][rng.below(4)];
        let control: Vec<Point3<f64>> = (0..2 * nv)
            .map(|_| Point3::origin() + Vec3::new(rng.range(-1.0, 1.0), rng.range(-1.0, 1.0), rng.range(-1.0, 1.0)) * scale + o)
            .collect();
        let decades = [0.0, 2.0, 4.0, 8.0][rng.below(4)];
        let weights: Vec<f64> = (0..2 * nv).map(|_| 10f64.powf(rng.range(-decades / 2.0, decades / 2.0))).collect();
        let chart = NurbsSurface::new(ku, kv.clone(), control.clone(), weights.clone()).unwrap();
        let net = Net::of(&chart, band).unwrap();
        let cells: Vec<String> = net
            .cells
            .iter()
            .map(|c| {
                let pts: Vec<String> = c
                    .net
                    .iter()
                    .map(|(h, e)| {
                        let b = super::point_rounding(&(*h, *e));
                        let p = super::point(*h);
                        format!("[[{},{},{}],[{},{},{}]]", hx(p.x), hx(p.y), hx(p.z), hx(b[0]), hx(b[1]), hx(b[2]))
                    })
                    .collect();
                format!("{{\"rect\":[{},{}],\"pts\":[{}]}}", hx(c.rect[2]), hx(c.rect[3]), pts.join(","))
            })
            .collect();
        let kvs: Vec<String> = kv.knots().iter().map(|&x| hx(x)).collect();
        let cp: Vec<String> = control.iter().map(|p| format!("[{},{},{}]", hx(p.x), hx(p.y), hx(p.z))).collect();
        let w: Vec<String> = weights.iter().map(|&x| hx(x)).collect();
        eprintln!(
            "REFJSON {{\"case\":{case},\"pv\":{pv},\"nv\":{nv},\"kv\":[{}],\"cp\":[{}],\"w\":[{}],\"c\":[{},{},{}],\"cells\":[{}]}}",
            kvs.join(","), cp.join(","), w.join(","), hx(net.centre.x), hx(net.centre.y), hx(net.centre.z), cells.join(",")
        );
    }
}
