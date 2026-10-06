//! Review probe (r2, PR 4104): falsification sweeps of `refined_sign`.
//! Not for merge.

use core::cell::Cell;

use geom_core::{Bounds, Interval};
use geom_core::interval::certification::Certification;

use super::{SsiError, ratio_hull, refined_sign, sign};

thread_local! {
    pub(super) static READS: Cell<usize> = const { Cell::new(0) };
}

struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> u64 {
        self.0 = self
            .0
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        self.0 >> 11
    }
    fn int(&mut self, lo: i64, hi: i64) -> i64 {
        lo + (self.next() % ((hi - lo + 1) as u64)) as i64
    }
    fn unit(&mut self) -> f64 {
        (self.next() as f64) / ((1u64 << 53) as f64)
    }
}

fn binom(n: usize, k: usize) -> i128 {
    let mut r: i128 = 1;
    for i in 0..k {
        r = r * (n - i) as i128 / (i + 1) as i128;
    }
    r
}

/// Scaled-Bernstein (t^k (1-t)^(n-k)) integer coefficients: product is
/// convolution.
fn mul(a: &[i128], b: &[i128]) -> Vec<i128> {
    let mut out = vec![0i128; a.len() + b.len() - 1];
    for (i, x) in a.iter().enumerate() {
        for (j, y) in b.iter().enumerate() {
            out[i + j] += x * y;
        }
    }
    out
}

/// Bernstein coefficient enclosures of a scaled-Bernstein integer poly.
fn enclose(e: &[i128]) -> Option<Vec<Interval>> {
    let n = e.len() - 1;
    e.iter()
        .enumerate()
        .map(|(k, &x)| {
            if x.abs() >= 1i128 << 53 {
                return None;
            }
            Some(Interval::point(x as f64) / Interval::point(binom(n, k) as f64))
        })
        .collect()
}

fn read(p: Vec<(Vec<Interval>, Vec<Interval>)>) -> (Result<Option<bool>, SsiError>, usize) {
    READS.with(|r| r.set(0));
    let got = refined_sign(p.into_iter());
    (got, READS.with(Cell::get))
}

fn widen(c: Interval, rel: f64) -> Interval {
    let m = c.lo().abs().max(c.hi().abs()) * rel;
    c + Interval::from_bounds(-m, m)
}

/// A random positive-weight W of degree n.
fn weights(rng: &mut Lcg, n: usize, spread: f64) -> Vec<Interval> {
    (0..=n)
        .map(|_| Interval::point(1.0 + rng.unit() * spread))
        .collect()
}

/// **Soundness**: a planted zero r = p/q in [0, 1] of multiplicity k,
/// times a random integer g, over random positive W, scaled and widened:
/// the refined reading never certifies a sign.
#[test]
fn r2_planted_zero_never_reads_signed() {
    let mut rng = Lcg(0x4104_0002);
    let (mut cases, mut worst_reads, mut budget) = (0usize, 0usize, 0usize);
    let mut hist = [0usize; 8];
    for _ in 0..200_000 {
        let q = [1i128, 2, 3, 4, 7, 8, 16, 1024, 3 * 1024][rng.int(0, 8) as usize];
        let p = rng.int(0, q as i64) as i128;
        let k = rng.int(1, 4) as usize;
        let dg = rng.int(0, 4) as usize;
        let mut e = vec![1i128];
        for _ in 0..k {
            e = mul(&e, &[-p, q - p]);
        }
        let mode = rng.int(0, 2);
        let g: Vec<i128> = (0..=dg)
            .map(|j| {
                let b = match mode {
                    0 => rng.int(1, 20),
                    1 => rng.int(-20, 20),
                    _ => rng.int(-3, 20),
                } as i128;
                b * binom(dg, j)
            })
            .collect();
        e = mul(&e, &g);
        // Optional degree elevation by multiplying by (1-t + t) = [1, 1].
        for _ in 0..rng.int(0, 2) {
            e = mul(&e, &[1, 1]);
        }
        let Some(h) = enclose(&e) else { continue };
        let n = h.len() - 1;
        let scale = [1.0, 1e-12, 1e12, 1e-300, 1e300, 2f64.powi(-1060)][rng.int(0, 5) as usize];
        let rel = [0.0, 1e-16, 1e-9, 1e-3][rng.int(0, 3) as usize];
        let h: Vec<Interval> = h
            .into_iter()
            .map(|c| widen(c * Interval::point(scale), rel))
            .collect();
        let spread = [0.0, 1.0, 1e3, 1e8][rng.int(0, 3) as usize];
        let w = weights(&mut rng, n, spread);
        // Neighbours: positive pieces before and after (flush at the shared end
        // is not required: they are separate pieces of the side).
        let mut pieces = vec![];
        for _ in 0..rng.int(0, 2) {
            pieces.push((vec![Interval::point(1.0); 3], vec![Interval::point(1.0); 3]));
        }
        pieces.push((h.clone(), w));
        cases += 1;
        let (got, reads) = read(pieces);
        worst_reads = worst_reads.max(reads);
        hist[(usize::BITS - reads.leading_zeros()).min(7) as usize] += 1;
        match got {
            Ok(Some(s)) => panic!(
                "SIGNED {s} with a planted zero r = {p}/{q} k {k} g {g:?} scale {scale:e} rel {rel:e} h {h:?}"
            ),
            Ok(None) => {}
            Err(_) => budget += 1,
        }
    }
    eprintln!(
        "r2 planted: {cases} cases, none signed; worst reads {worst_reads}; budget {budget}; log2-reads histogram {hist:?}"
    );
}

/// **Precision**: exactly positive h = (q t − p)² g + D, g > 0: how often
/// is it read unsigned, by the old hull and by the refined reading, as the
/// margin D shrinks relative to the scale.
#[test]
fn r2_positive_near_touch_precision() {
    let mut rng = Lcg(0x4104_0003);
    for qexp in [2u32, 6, 10, 14, 18] {
        let (mut n_cases, mut old_none, mut new_none, mut worst) = (0, 0, 0, 0);
        let mut floor_on_positive = 0;
        for _ in 0..20_000 {
            let q = 1i128 << qexp;
            let p = rng.int(0, q as i64) as i128;
            let dg = rng.int(0, 3) as usize;
            let g: Vec<i128> = (0..=dg)
                .map(|j| rng.int(1, 8) as i128 * binom(dg, j))
                .collect();
            let mut e = mul(&mul(&[-p, q - p], &[-p, q - p]), &g);
            let n = e.len() - 1;
            let d = rng.int(1, 4) as i128;
            for (k, x) in e.iter_mut().enumerate() {
                *x += d * binom(n, k);
            }
            let Some(h) = enclose(&e) else { continue };
            let w = vec![Interval::point(1.0); h.len()];
            n_cases += 1;
            let old = sign(ratio_hull(core::iter::once((&h, &w))));
            if old.is_none() {
                old_none += 1;
            }
            let (got, reads) = read(vec![(h, w)]);
            worst = worst.max(reads);
            match got {
                Ok(Some(true)) => {}
                Ok(None) => {
                    new_none += 1;
                    floor_on_positive += 1;
                }
                other => panic!("{other:?}"),
            }
        }
        eprintln!(
            "r2 precision q 2^{qexp} (rel margin ~ 2^-{}): {n_cases} cases, old unsigned {old_none}, refined unsigned {new_none}, worst reads {worst}",
            2 * qexp
        );
        let _ = floor_on_positive;
    }
}

/// Wide coefficient enclosures (a big-coordinate side): one-signed φ read
/// unsigned because the end coefficients' enclosures straddle.
#[test]
fn r2_wide_enclosures() {
    for rel in [1e-12, 1e-6, 0.1, 0.5, 0.9, 1.1] {
        let h: Vec<Interval> = [1.0, -0.2, 1.0]
            .iter()
            .map(|&x| widen(Interval::point(x), rel))
            .collect();
        let w = vec![Interval::point(1.0); 3];
        let (got, reads) = read(vec![(h, w)]);
        eprintln!("r2 wide rel {rel:e}: {got:?} in {reads} reads");
    }
}

/// Cost: the dense wall's side, m loose pieces; and m one-signed pieces past
/// the budget (the old hull read them signed).
#[test]
fn r2_budget() {
    let pc = |c: &[f64]| {
        (
            c.iter().map(|&x| Interval::point(x)).collect::<Vec<_>>(),
            vec![Interval::point(1.0); c.len()],
        )
    };
    for m in [4096usize, 66_666, 66_667, 70_000] {
        let t = std::time::Instant::now();
        let (got, reads) = read(vec![pc(&[0.6e-9, -0.2e-9, 0.6e-9]); m]);
        eprintln!(
            "r2 loose m {m}: {:?} in {reads} reads, {:?}",
            got.map_err(|e| e.to_string()),
            t.elapsed()
        );
    }
    let m = super::super::SSI_MAX_CELLS + 1;
    let pieces = vec![pc(&[1.0, 1.0]); m];
    let old = sign(ratio_hull(pieces.iter().map(|(h, w)| (h, w))));
    let (got, reads) = read(pieces);
    eprintln!(
        "r2 signed m {m}: old {old:?}, refined {:?} in {reads} reads",
        got.map_err(|e| e.to_string())
    );
    // Slow narrowing: a high-degree loose net near a positive touch.
    for (deg, delta) in [(8usize, 1e-12), (16, 1e-15), (24, 1e-15)] {
        // (2t-1)^deg elevated + delta, Bernstein coefficients of (1-2t)^deg
        // are (-1)^k.
        let c: Vec<f64> = (0..=deg)
            .map(|k| if k % 2 == 0 { 1.0 } else { -1.0 } + delta)
            .collect();
        let t = std::time::Instant::now();
        let (got, reads) = read(vec![pc(&c)]);
        let _ = Certification::width(Interval::point(0.0));
        eprintln!("r2 alternating deg {deg} + {delta:e}: {got:?} in {reads} reads, {:?}", t.elapsed());
    }
}

/// Order dependence: a side with a piece that runs to the budget and a
/// piece of the opposite sign reads None or Err by the pieces' order.
#[test]
fn r2_order() {
    let pc = |c: &[f64]| {
        (
            c.iter().map(|&x| Interval::point(x)).collect::<Vec<_>>(),
            vec![Interval::point(1.0); c.len()],
        )
    };
    let neg = pc(&[-1.0, -1.0]);
    let mut many = vec![pc(&[0.6, -0.2, 0.6]); 70_000];
    many.insert(0, neg.clone());
    let (a, ra) = read(many.clone());
    many.remove(0);
    many.push(neg);
    let (b, rb) = read(many);
    eprintln!(
        "r2 order: negative first {:?} ({ra}), negative last {:?} ({rb})",
        a.map_err(|e| e.to_string()),
        b.map_err(|e| e.to_string())
    );
}

/// **Soundness through the stretch door**: a planted zero, read through
/// `SectionReader::stretch` on random stretches holding it (ends on it
/// included), with power-of-two weights.
#[test]
fn r2_stretch_door_planted_zero() {
    use geom_core::spline::KnotVector;
    let mut rng = Lcg(0x4104_0004);
    let (mut cases, mut worst) = (0usize, 0usize);
    for _ in 0..50_000 {
        let q = [1i128, 2, 3, 4, 5, 8, 1024][rng.int(0, 6) as usize];
        let p = rng.int(0, q as i64) as i128;
        let k = rng.int(1, 3) as usize;
        let mut e = vec![1i128];
        for _ in 0..k {
            e = mul(&e, &[-p, q - p]);
        }
        let dg = rng.int(0, 3) as usize;
        let g: Vec<i128> = (0..=dg)
            .map(|j| rng.int(-5, 20) as i128 * binom(dg, j))
            .collect();
        e = mul(&e, &g);
        let Some(h) = enclose(&e) else { continue };
        let n = h.len() - 1;
        if n == 0 {
            continue;
        }
        let w: Vec<f64> = (0..=n).map(|_| 2f64.powi(rng.int(-3, 3) as i32)).collect();
        let control: Vec<[Interval; 3]> = h
            .iter()
            .zip(&w)
            .map(|(hc, wi)| [Interval::point(0.0), Interval::point(0.0), *hc / Interval::point(*wi)])
            .collect();
        let mut kv = vec![0.0; n + 1];
        kv.extend(vec![1.0; n + 1]);
        let kv = KnotVector::clamped(kv, n).unwrap();
        let z = Interval::point(0.0);
        let Some(reader) = super::SectionReader::of(
            &kv,
            &control,
            &w,
            ([z, z, z], [z, z, Interval::point(1.0)]),
        ) else {
            continue;
        };
        let r = p as f64 / q as f64;
        let exact = (q as f64) * r == p as f64 && q.count_ones() == 1;
        let a = r * rng.unit();
        let b = r + (1.0 - r) * rng.unit();
        let mut spans = vec![(a, b), (0.0, 1.0)];
        if exact {
            spans.extend([(a, r), (r, b), (r, r)]);
        }
        for s in spans {
            cases += 1;
            READS.with(|c| c.set(0));
            let got = reader.stretch(s).sign();
            worst = worst.max(READS.with(Cell::get));
            assert!(
                !matches!(got, Ok(Some(_))),
                "SIGNED {got:?} on {s:?}: r {p}/{q} k {k} g {g:?} w {w:?}"
            );
        }
    }
    eprintln!("r2 stretch door: {cases} stretches, none signed; worst reads {worst}");
}

/// Non-dyadic near touches: the precision and reads of the refined
/// reading where halving never lands on the touch.
#[test]
fn r2_non_dyadic_touch() {
    let mut rng = Lcg(0x4104_0005);
    for qb in [3i128, 3 * 64, 3 * 4096, 5 * 65536, 7 * 262_144] {
        let (mut none, mut worst, mut tot) = (0, 0, 0);
        for _ in 0..5_000 {
            let p = rng.int(1, (qb - 1) as i64) as i128;
            let mut e = mul(&[-p, qb - p], &[-p, qb - p]);
            let n = e.len() - 1;
            for (k, x) in e.iter_mut().enumerate() {
                *x += binom(n, k);
            }
            let Some(h) = enclose(&e) else { continue };
            let w = vec![Interval::point(1.0); 3];
            tot += 1;
            let (got, reads) = read(vec![(h, w)]);
            worst = worst.max(reads);
            if got.unwrap().is_none() {
                none += 1;
            }
        }
        eprintln!("r2 non-dyadic q {qb}: {tot} cases, refined unsigned {none}, worst reads {worst}");
    }
}

#[test]
fn r2_reads_of_crossings() {
    let pc = |c: &[f64]| {
        (
            c.iter().map(|&x| Interval::point(x)).collect::<Vec<_>>(),
            vec![Interval::point(1.0); c.len()],
        )
    };
    for c in [
        vec![-0.5e-9, 0.5e-9],
        vec![0.3e-9, -0.5e-9, 0.3e-9],
        vec![1.0, -3.0, 2.0, -1.0],
        vec![-1.0, 3.0, -3.0, 1.0],
        vec![0.0, 0.0, 0.0],
        vec![1.0, -1.0, 1.0, -1.0, 1.0],
        vec![0.25, -0.25, 0.25],
        vec![1.0 / 9.0, -2.0 / 9.0, 4.0 / 9.0],
    ] {
        let (got, reads) = read(vec![pc(&c)]);
        eprintln!("r2 crossing {c:?}: {got:?} in {reads} reads");
    }
}

#[test]
fn r2_many_near_touches_to_the_budget() {
    let qb = 7i128 * 262_144;
    let p = 3 * 262_144 + 1;
    let mut e = mul(&[-p, qb - p], &[-p, qb - p]);
    let n = e.len() - 1;
    for (k, x) in e.iter_mut().enumerate() {
        *x += binom(n, k);
    }
    let h = enclose(&e).unwrap();
    let w = vec![Interval::point(1.0); 3];
    let (one, r1) = read(vec![(h.clone(), w.clone())]);
    eprintln!("r2 near touch alone: {one:?} in {r1} reads");
    for m in [1000usize, 4096, 8192] {
        let t = std::time::Instant::now();
        let (got, reads) = read(vec![(h.clone(), w.clone()); m]);
        let old = sign(ratio_hull(core::iter::once((&h, &w))));
        eprintln!(
            "r2 near touches m {m}: old {old:?}, refined {:?} in {reads} reads, {:?}",
            got.map_err(|e| e.to_string()),
            t.elapsed()
        );
    }
}
