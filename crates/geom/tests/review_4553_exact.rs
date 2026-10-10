//! Review 4553 probe (claims 2 and 5): dumps `apply_homogeneous` at
//! `Interval` and the rational speed meter at `f64`/`Interval` on random
//! and adversarial nets; an exact-rational checker reads the dump.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::NurbsCurve3;
use geom_core::Bounds;
use geom_core::Point3;
use geom_core::interval::Interval;
use geom_core::spline::KnotVector;
use geom_core::spline::algebra::refine_plan_homogeneous;

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
}

fn knots(r: &mut Rng, p: usize, n: usize) -> Vec<f64> {
    let mut inner: Vec<f64> = (0..n - p - 1).map(|_| r.range(0.02, 0.98)).collect();
    inner.sort_by(f64::total_cmp);
    let mut k = vec![0.0; p + 1];
    k.extend(inner);
    k.extend(vec![1.0; p + 1]);
    k
}

fn list(v: &[f64]) -> String {
    v.iter()
        .map(|x| format!("{x:?}"))
        .collect::<Vec<_>>()
        .join(",")
}

#[test]
fn review_4553_dump() {
    let path = std::env::var("REVIEW_4553_DUMP").unwrap_or_else(|_| "/dev/null".into());
    let mut out = String::new();
    let mut r = Rng(0x9E37_79B9_7F4A_7C15);
    // Claim 2: apply_homogeneous at Interval vs exact Boehm.
    for _ in 0..400 {
        let p = 2 + (r.next() * 2.0) as usize;
        let n = p + 1 + (r.next() * 6.0) as usize;
        let k = knots(&mut r, p, n);
        let kv = KnotVector::clamped(k.clone(), p).unwrap();
        let mut add: Vec<f64> = (0..1 + (r.next() * 6.0) as usize)
            .map(|_| r.range(0.01, 0.99))
            .collect();
        add.sort_by(f64::total_cmp);
        let Ok(plans) = refine_plan_homogeneous(&kv, &add) else {
            continue;
        };
        let chan: Vec<f64> = (0..n)
            .map(|_| {
                let w = 10f64.powf(r.range(-4.0, 4.0));
                if r.next() < 0.5 {
                    w
                } else {
                    w * r.range(-3.0, 3.0)
                }
            })
            .collect();
        let mut c: Vec<Interval> = chan.iter().map(|x| Interval::from_bounds(*x, *x)).collect();
        for plan in &plans {
            c = plan.apply_homogeneous(&c);
        }
        let lo: Vec<f64> = c.iter().map(|x| x.lo()).collect();
        let hi: Vec<f64> = c.iter().map(|x| x.hi()).collect();
        out += &format!(
            "APPLY|{p}|{}|{}|{}|{}|{}\n",
            list(&k),
            list(&add),
            list(&chan),
            list(&lo),
            list(&hi)
        );
    }
    // Claim 5: rational speed meter.
    let mut curves: Vec<(String, usize, Vec<f64>, Vec<[f64; 3]>, Vec<f64>)> = Vec::new();
    for i in 0..250 {
        let p = 2 + (r.next() * 2.0) as usize;
        let n = p + 1 + (r.next() * 4.0) as usize;
        let k = knots(&mut r, p, n);
        let ctl: Vec<[f64; 3]> = (0..n)
            .map(|j| {
                [
                    j as f64 + r.range(-0.3, 0.3),
                    r.range(-0.5, 0.5),
                    r.range(-0.2, 0.2),
                ]
            })
            .collect();
        let dec = if i % 2 == 0 { 3.0 } else { 1.0 };
        let w: Vec<f64> = (0..n).map(|_| 10f64.powf(r.range(-dec, dec))).collect();
        curves.push((format!("rand{i}"), p, k, ctl, w));
    }
    // Near-stalls: a quadratic that almost doubles back.
    for i in 0..120 {
        let a = r.range(0.05, 1.5);
        let h = 10f64.powf(r.range(-6.0, -1.0));
        let w1 = 10f64.powf(r.range(-2.5, 2.5));
        let ctl = vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0 - a, h, 0.0]];
        curves.push((
            format!("stall{i}"),
            2,
            vec![0., 0., 0., 1., 1., 1.],
            ctl,
            vec![1.0, w1, 1.0],
        ));
    }
    // Steep two-span ratios.
    for i in 0..60 {
        let wm = 10f64.powf(r.range(-3.0, 3.0));
        let ctl = vec![
            [0.0, 0.0, 0.0],
            [0.5, r.range(0.0, 1.0), 0.0],
            [1.0, r.range(-1.0, 1.0), 0.0],
            [1.5, r.range(0.0, 1.0), 0.0],
        ];
        curves.push((
            format!("steep{i}"),
            2,
            vec![0., 0., 0., r.range(0.2, 0.8), 1., 1., 1.],
            ctl,
            vec![1.0, wm, 1.0 / wm, 1.0],
        ));
    }
    for (tag, p, k, ctl, w) in &curves {
        let kv = KnotVector::clamped(k.clone(), *p).unwrap();
        let cf = NurbsCurve3::<f64>::new(
            kv.clone(),
            ctl.iter().map(|c| Point3::new(c[0], c[1], c[2])).collect(),
            w.clone(),
        )
        .unwrap();
        let ip = |x: f64| Interval::from_bounds(x, x);
        let ci = NurbsCurve3::<Interval>::new(
            kv,
            ctl.iter()
                .map(|c| Point3::new(ip(c[0]), ip(c[1]), ip(c[2])))
                .collect(),
            w.clone(),
        )
        .unwrap();
        let mf = cf.speed_lower_bound().get();
        let mi = ci.speed_lower_bound().get();
        let flat: Vec<f64> = ctl.iter().flat_map(|c| c.iter().copied()).collect();
        out += &format!(
            "SPEED|{tag}|{p}|{}|{}|{}|{mf:?}|{:?}|{:?}\n",
            list(k),
            list(&flat),
            list(w),
            mi.lo(),
            mi.hi()
        );
    }
    std::fs::write(path, out).unwrap();
}
