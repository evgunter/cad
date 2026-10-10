use geom::NurbsCurve3;
use geom_core::interval::Interval;
use geom_core::spline::KnotVector;
use geom_core::{Bounds, Point3, Real};

struct Rng(u64);
impl Rng { fn next(&mut self) -> f64 { self.0 ^= self.0 << 13; self.0 ^= self.0 >> 7; self.0 ^= self.0 << 17; (self.0 >> 11) as f64 / (1u64 << 53) as f64 } }

fn curve<T: Real>(k: &[f64], p: usize, c: &[[f64;3]], w: &[f64]) -> NurbsCurve3<T> {
    let f = T::from_f64;
    NurbsCurve3::new(KnotVector::clamped(k.to_vec(), p).unwrap(), c.iter().map(|q| Point3::new(f(q[0]),f(q[1]),f(q[2]))).collect(), w.to_vec()).unwrap()
}
fn speed_hi(c: &NurbsCurve3<Interval>, t: f64) -> f64 {
    let d = c.deriv(Interval::from_bounds(t, t));
    let n = (d.x*d.x + d.y*d.y + d.z*d.z).sqrt();
    n.hi()
}
fn main() {
    let mut rng = Rng(0x1234_5678_9abc_def1);
    let (mut bad, mut n_ok, mut worst) = (0, 0, 0.0f64);
    let mut ratios = vec![]; let mut nobr = 0;
    for trial in 0..2000 {
        let p = 2 + (rng.next()*3.0) as usize;
        let n = p + 1 + (rng.next()*5.0) as usize;
        let mut inner: Vec<f64> = (0..n-p-1).map(|_| rng.next()).collect();
        if rng.next() < 0.3 && inner.len() > 1 { inner[1] = inner[0] + 1e-7*rng.next(); }
        inner.sort_by(f64::total_cmp);
        let mut k = vec![0.0; p+1]; k.extend(inner); k.extend(vec![1.0; p+1]);
        let dec = [0.5, 1.0, 2.0, 3.0][trial % 4];
        let w: Vec<f64> = (0..n).map(|_| 10f64.powf(dec*(2.0*rng.next()-1.0))).collect();
        // A gently turning polygon; on near-stall trials two points nearly coincide.
        let mut c = vec![[0.0f64;3]]; let mut ang = 0.0f64;
        for i in 1..n {
            ang += 0.6*(rng.next()-0.5);
            let step = if trial % 5 == 0 && i == n/2 { 1e-3 } else { 0.5 + rng.next() };
            let l = c[i-1]; c.push([l[0]+step*ang.cos(), l[1]+step*ang.sin(), l[2] + 0.2*(rng.next()-0.5)]);
        }
        let cf = curve::<f64>(&k, p, &c, &w);
        let ci = curve::<Interval>(&k, p, &c, &w);
        let bf = cf.speed_lower_bound().get();
        let bi = ci.speed_lower_bound().get();
        if !(bi.lo() > 0.0) { continue; }
        n_ok += 1;
        // Sampled true-speed upper readings.
        let m = 4000;
        let (mut best, mut bt) = (f64::INFINITY, 0.0);
        for j in 0..=m { let t = j as f64 / m as f64; let s = speed_hi(&ci, t); if s < best { best = s; bt = t; } }
        let (mut a, mut b) = ((bt - 1.0/m as f64).max(0.0), (bt + 1.0/m as f64).min(1.0));
        for _ in 0..80 { let (x1, x2) = (a + (b-a)*0.382, a + (b-a)*0.618); if speed_hi(&ci,x1) < speed_hi(&ci,x2) { b = x2 } else { a = x1 } }
        best = best.min(speed_hi(&ci, 0.5*(a+b)));
        if !(bi.lo() <= bf && bf <= bi.hi()) { nobr += 1; if nobr <= 5 { println!("NOBRACKET trial {trial} f64={bf:e} itv=[{:e},{:e}]", bi.lo(), bi.hi()); } }
        let r = bi.lo()/best; ratios.push(r); worst = worst.max(r);
        if bi.lo() > best || bf > best * (1.0 + 1e-12) {
            bad += 1;
            println!("VIOLATION trial {trial} p={p} k={k:?} w={w:?} c={c:?} itv=[{:e},{:e}] f64={bf:e} sampled_min_hi={best:e}", bi.lo(), bi.hi());
        }
    }
    ratios.sort_by(f64::total_cmp);
    println!("no-bracket {nobr}; positive-at-Interval {n_ok}, violations {bad}, worst lo/min {worst:.17}, median {:.4}", ratios[ratios.len()/2]);
}
