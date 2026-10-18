//! Review probes (PR 5259, review 1): randomized falsification of the
//! edge-pair readings' definite "no" verdicts against pairs built to
//! meet strictly inside both spans.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use super::*;
use geom_core::Interval;

struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> f64 {
        self.0 = self
            .0
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        ((self.0 >> 11) as f64) / ((1u64 << 53) as f64)
    }
    fn range(&mut self, lo: f64, hi: f64) -> f64 {
        lo + (hi - lo) * self.next()
    }
    fn unit(&mut self) -> [f64; 3] {
        loop {
            let v = [
                self.range(-1.0, 1.0),
                self.range(-1.0, 1.0),
                self.range(-1.0, 1.0),
            ];
            let n = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
            if n > 0.1 && n < 1.0 {
                return v.map(|x| x / n);
            }
        }
    }
}

fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}
fn norm(a: [f64; 3]) -> f64 {
    (a[0] * a[0] + a[1] * a[1] + a[2] * a[2]).sqrt()
}
fn add(a: [f64; 3], b: [f64; 3], s: f64) -> [f64; 3] {
    [a[0] + b[0] * s, a[1] + b[1] * s, a[2] + b[2] * s]
}

/// A direction at angle `phi` off `a`.
fn turned(r: &mut Lcg, a: [f64; 3], phi: f64) -> [f64; 3] {
    let mut w = cross(a, r.unit());
    while norm(w) < 0.1 {
        w = cross(a, r.unit());
    }
    let w = w.map(|x| x / norm(w));
    add(a.map(|x| x * phi.cos()), w, phi.sin())
}

fn seg_t<T: Decide>(a: [f64; 3], b: [f64; 3]) -> EdgeGeo<T> {
    let p = |v: [f64; 3]| Point3::new(T::from_f64(v[0]), T::from_f64(v[1]), T::from_f64(v[2]));
    segment((p(a), p(b)))
}

fn meets<T: Decide>(
    a: (&[f64; 3], &[f64; 3]),
    b: (&[f64; 3], &[f64; 3]),
    band: Band,
) -> Option<bool> {
    let mut errors = Vec::new();
    interiors_meet(
        &seg_t::<T>(*a.0, *a.1),
        &seg_t::<T>(*b.0, *b.1),
        band,
        &mut errors,
    )
}

const K: f64 = 10.0;

/// Pairs crossing at one point at least `3Kε` inside both spans, over
/// every angle from far in the parallel zero to right: never read
/// definitely apart, at f64 or at Interval, in either order.
#[test]
fn crossings_strictly_inside_both_spans_never_read_apart() {
    let mut bad = Vec::new();
    let mut f64_only = 0usize;
    for &eps in &[1e-6, 1e-9, 1e-12] {
        let band = Band::new(eps, K * eps).unwrap();
        let mut r = Lcg(0x5259 ^ eps.to_bits());
        for _ in 0..20_000 {
            let x = [r.range(-2.0, 2.0), r.range(-2.0, 2.0), r.range(-2.0, 2.0)];
            let a = r.unit();
            let phi = 10f64.powf(r.range(-14.0, 0.2));
            let b = turned(&mut r, a, phi);
            let m = 3.0 * K * eps;
            let la = 10f64.powf(r.range((7.0 * m).log10(), 0.5));
            let lb = 10f64.powf(r.range((7.0 * m).log10(), 0.5));
            let sa = r.range(m, la - m);
            let sb = r.range(m, lb - m);
            let (a0, a1) = (add(x, a, -sa), add(x, a, la - sa));
            let (b0, b1) = (add(x, b, -sb), add(x, b, lb - sb));
            for (p, q) in [((&a0, &a1), (&b0, &b1)), ((&b0, &b1), (&a0, &a1))] {
                let f = meets::<f64>(p, q, band);
                let i = meets::<Interval>(p, q, band);
                if f == Some(false) && i != Some(false) {
                    f64_only += 1;
                }
                if i == Some(false) {
                    bad.push(format!(
                        "eps {eps:e} phi {phi:e} la {la:e} lb {lb:e} sa {sa:e} sb {sb:e} f64 {f:?} iv {i:?} \
                         a {p:?} b {q:?}"
                    ));
                }
            }
        }
    }
    eprintln!("f64-only false noes (closed-form rounding, Interval does not say no): {f64_only}");
    for e in ["eps 1e-6 ", "eps 1e-9 ", "eps 1e-12 "] {
        eprintln!("{e}: {}", bad.iter().filter(|b| b.starts_with(e)).count());
    }
    assert!(
        bad.is_empty(),
        "{} false noes, first: {:#?}",
        bad.len(),
        &bad[..bad.len().min(6)]
    );
}

/// The overlap gap is the least offset over the run, not the offset at
/// the run's start: a short piece whose start stands 1.5ε off a long
/// edge and whose end stands 0.5ε off it (turn 1ε over its unit
/// length, parallel Zero) overlaps it, in either order. Read at the
/// run's start only, the gap lands in band and the pair escalates.
#[test]
fn the_overlap_gap_is_the_least_offset_over_the_run() {
    let band = Band::new(1e-9, 1e-8).unwrap();
    let long = seg_t::<f64>([-1.0, 0.0, 0.0], [2.0, 0.0, 0.0]);
    let short = seg_t::<f64>([0.0, 1.5e-9, 0.0], [1.0, 0.5e-9, 0.0]);
    for (a, b) in [(&long, &short), (&short, &long)] {
        let mut errors = Vec::new();
        let on = collinear_overlap(a, b, band, &mut errors).is_some();
        assert_eq!((on, errors.len()), (true, 0));
    }
}

/// Pairs overlapping along at least `3Kε`, every point of the overlap
/// within ε/3 of the other edge: never read definitely apart.
#[test]
fn overlaps_within_the_zero_never_read_apart() {
    let mut bad = Vec::new();
    for &eps in &[1e-6, 1e-9, 1e-12] {
        let band = Band::new(eps, K * eps).unwrap();
        let mut r = Lcg(0x5260 ^ eps.to_bits());
        for _ in 0..20_000 {
            let x = [r.range(-2.0, 2.0), r.range(-2.0, 2.0), r.range(-2.0, 2.0)];
            let a = r.unit();
            let m = 3.0 * K * eps;
            let la = 10f64.powf(r.range((3.0 * m).log10(), 0.5));
            let lb = 10f64.powf(r.range((3.0 * m).log10(), 0.5));
            // eb starts inside ea (or before it) so the overlap is ≥ m.
            let s = r.range(-lb + m, la - m);
            let over = (la.min(s + lb) - s.max(0.0)).max(0.0);
            if over < m {
                continue;
            }
            // tilt so the deviation over the longer edge stays within ε/6
            let psi = r.next() * (eps / 6.0) / la.max(lb);
            let b = turned(&mut r, a, psi);
            let off = cross(a, r.unit());
            let off = off.map(|v| v / norm(off).max(1e-300) * (r.next() * eps / 6.0));
            let b0 = add(add(x, a, s), off, 1.0);
            let b1 = add(b0, b, lb);
            let (a0, a1) = (x, add(x, a, la));
            for (p, q) in [((&a0, &a1), (&b0, &b1)), ((&b0, &b1), (&a0, &a1))] {
                let f = meets::<f64>(p, q, band);
                let i = meets::<Interval>(p, q, band);
                if f == Some(false) || i == Some(false) {
                    bad.push(format!(
                        "eps {eps:e} psi {psi:e} la {la:e} lb {lb:e} s {s:e} f64 {f:?} iv {i:?} a {p:?} b {q:?}"
                    ));
                }
            }
        }
    }
    assert!(
        bad.is_empty(),
        "{} false noes, first: {:#?}",
        bad.len(),
        &bad[..bad.len().min(6)]
    );
}
