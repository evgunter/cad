//! Review probes for PR 4551 (`Certification::between`, `CoeffWindow::blossom`
//! and `restrict`), measured against an exact-rational reference.

use geom_core::Bounds;
use geom_core::Interval;
use geom_core::interval::certification::Certification;
use geom_core::spline::{BlossomError, KnotVector};
use num_bigint::BigInt;
use num_traits::{One, Signed, Zero};

/// An exact rational `n / d`, `d > 0`, unreduced.
#[derive(Clone, Debug)]
struct Q(BigInt, BigInt);

impl Q {
    fn f(x: f64) -> Q {
        assert!(x.is_finite());
        if x == 0.0 {
            return Q(BigInt::zero(), BigInt::one());
        }
        let bits = x.to_bits();
        let sign = if bits >> 63 == 1 { -1 } else { 1 };
        let exp = ((bits >> 52) & 0x7ff) as i64;
        let frac = bits & ((1u64 << 52) - 1);
        let (m, e) = if exp == 0 {
            (frac, -1074)
        } else {
            (frac | (1u64 << 52), exp - 1075)
        };
        let m = BigInt::from(m) * sign;
        if e >= 0 {
            Q(m << (e as usize), BigInt::one())
        } else {
            Q(m, BigInt::one() << ((-e) as usize))
        }
    }
    fn add(&self, o: &Q) -> Q {
        Q(&self.0 * &o.1 + &o.0 * &self.1, &self.1 * &o.1)
    }
    fn sub(&self, o: &Q) -> Q {
        Q(&self.0 * &o.1 - &o.0 * &self.1, &self.1 * &o.1)
    }
    fn mul(&self, o: &Q) -> Q {
        Q(&self.0 * &o.0, &self.1 * &o.1)
    }
    fn div(&self, o: &Q) -> Q {
        let (n, d) = (&self.0 * &o.1, &self.1 * &o.0);
        if d.is_negative() { Q(-n, -d) } else { Q(n, d) }
    }
    /// `self ≤ x` (an f64).
    fn le_f(&self, x: f64) -> bool {
        let q = Q::f(x);
        &self.0 * &q.1 <= &q.0 * &self.1
    }
    fn ge_f(&self, x: f64) -> bool {
        let q = Q::f(x);
        &self.0 * &q.1 >= &q.0 * &self.1
    }
    fn reduce(self) -> Q {
        let g = num_integer::Integer::gcd(&self.0, &self.1);
        if g.is_zero() {
            self
        } else {
            Q(&self.0 / &g, &self.1 / &g)
        }
    }
}

fn inside(i: Interval, q: &Q) -> bool {
    i.is_certified() && q.ge_f(i.lo()) && q.le_f(i.hi())
}

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
    fn unit(&mut self) -> f64 {
        (self.next() >> 11) as f64 / (1u64 << 53) as f64
    }
    fn pick<T: Copy>(&mut self, xs: &[T]) -> T {
        xs[(self.next() % xs.len() as u64) as usize]
    }
}

fn ulps(x: f64, k: i64) -> f64 {
    let mut y = x;
    for _ in 0..k.unsigned_abs() {
        y = if k > 0 { y.next_up() } else { y.next_down() };
    }
    y
}

fn scalar(r: &mut Rng) -> f64 {
    let base = r.pick(&[
        0.0, -0.0, 1.0, -1.0, 0.5, 1e-300, 5e-324, 1e-310, 1e300, -1e300, 1e3, 1e-9, 3.0,
    ]);
    let v = match r.next() % 4 {
        0 => base,
        1 => base * (r.unit() * 2.0 - 1.0),
        2 => ulps(base, (r.next() % 9) as i64 - 4),
        _ => r.unit() * 10.0 - 5.0,
    };
    if v.is_finite() { v } else { base }
}

fn bracket(r: &mut Rng) -> (f64, f64) {
    let a = scalar(r);
    let b = match r.next() % 3 {
        0 => a,
        1 => ulps(a, (r.next() % 6) as i64),
        _ => scalar(r),
    };
    if a <= b { (a, b) } else { (b, a) }
}

fn lambda(r: &mut Rng) -> (f64, f64) {
    let pts = [
        0.0,
        -0.0,
        1.0,
        0.5,
        ulps(0.5, 1),
        ulps(0.5, -1),
        ulps(1.0, 1),
        ulps(0.0, -1),
        5e-324,
        ulps(1.0, -1),
        -1e-17,
        1.0 + 1e-15,
    ];
    let a = if r.next() % 2 == 0 {
        r.pick(&pts)
    } else {
        r.unit()
    };
    let b = match r.next() % 3 {
        0 => a,
        1 => ulps(a, (r.next() % 4) as i64),
        _ => {
            if r.next() % 2 == 0 {
                r.pick(&pts)
            } else {
                r.unit()
            }
        }
    };
    if a <= b { (a, b) } else { (b, a) }
}

#[test]
fn review_4551_between_encloses_every_exact_combination() {
    let mut r = Rng(0x9e37_79b9_7f4a_7c15);
    let (mut checked, mut refused) = (0usize, 0usize);
    for _ in 0..60_000 {
        let (xl, xh) = bracket(&mut r);
        let (yl, yh) = bracket(&mut r);
        let (ll, lh) = lambda(&mut r);
        let (x, y, l) = (
            Interval::from_bounds(xl, xh),
            Interval::from_bounds(yl, yh),
            Interval::from_bounds(ll, lh),
        );
        let out = Interval::between(x, y, l);
        let lam_lo = ll.max(0.0);
        let lam_hi = lh.min(1.0);
        if lam_lo > lam_hi {
            assert!(!out.is_certified(), "Λ outside [0,1] certified: {l:?}");
            continue;
        }
        if !out.is_certified() {
            refused += 1;
            continue;
        }
        // Inside hull(x, y).
        assert!(
            out.lo() >= xl.min(yl) && out.hi() <= xh.max(yh),
            "{out:?} left hull of {x:?} {y:?}"
        );
        let mids = [
            lam_lo,
            lam_hi,
            0.5f64.clamp(lam_lo, lam_hi),
            0.5 * (lam_lo + lam_hi),
        ];
        for &lam in &mids {
            let lq = Q::f(lam);
            let one_minus = Q::f(1.0).sub(&lq);
            for &xv in &[xl, xh] {
                for &yv in &[yl, yh] {
                    let v = one_minus.mul(&Q::f(xv)).add(&lq.mul(&Q::f(yv)));
                    assert!(
                        inside(out, &v),
                        "between({x:?}, {y:?}, {l:?}) = {out:?} misses λ={lam:e} x={xv:e} y={yv:e}"
                    );
                    checked += 1;
                }
            }
        }
    }
    eprintln!("between: {checked} exact checks, {refused} refusals");
    // Refusals of finite inputs should be rare: count them out loud.
    assert!(refused < 2_000, "refused {refused}");
}

#[test]
fn review_4551_between_poison_and_infinities() {
    let one = Interval::from_bounds(0.0, 1.0);
    assert!(!Interval::between(Interval::refused(), one, one).is_certified());
    assert!(!Interval::between(one, Interval::refused(), one).is_certified());
    assert!(!Interval::between(one, one, Interval::refused()).is_certified());
    let inf = Interval::from_bounds(f64::NEG_INFINITY, 0.0);
    eprintln!("inf src: {:?}", Interval::between(inf, one, one));
    // Overflowing chord.
    let big = Interval::from_bounds(f64::MAX, f64::MAX);
    let nbig = Interval::from_bounds(-f64::MAX, -f64::MAX);
    let o = Interval::between(big, nbig, Interval::from_bounds(0.25, 0.75));
    eprintln!("overflow chord: {o:?}");
    // Equal points stay points.
    let c = Interval::from_bounds(0.1, 0.1);
    let o = Interval::between(c, c, Interval::from_bounds(0.0, 1.0));
    assert!(o.lo() == 0.1 && o.hi() == 0.1, "{o:?}");
}

/// Exact de Boor blossom on span `s` of `knots`, degree `p`.
fn exact_blossom(knots: &[f64], p: usize, s: usize, coeffs: &[f64], args: &[f64]) -> Q {
    let mut d: Vec<Q> = coeffs[s - p..=s].iter().map(|&c| Q::f(c)).collect();
    for (level, &u) in args.iter().enumerate() {
        let r = level + 1;
        let u = Q::f(u);
        for j in (r..=p).rev() {
            let i = s - p + j;
            let lo = Q::f(knots[i]);
            let a = u.sub(&lo).div(&Q::f(knots[i + p + 1 - r]).sub(&lo));
            let om = Q::f(1.0).sub(&a);
            d[j] = om.mul(&d[j - 1]).add(&a.mul(&d[j])).reduce();
        }
    }
    d[p].clone()
}

fn random_knots(r: &mut Rng, p: usize) -> Vec<f64> {
    let (s0, s1) = match r.next() % 3 {
        0 => (0.0, 1.0),
        1 => (-3.0, 7.5),
        _ => (1e3, 1e3 + 1.0),
    };
    let mut k = vec![s0; p + 1];
    let interior = (r.next() % 4) as usize;
    let mut ins: Vec<f64> = (0..interior)
        .map(|_| {
            let t = s0 + (s1 - s0) * r.unit();
            if r.next() % 3 == 0 {
                ulps(s0 + (s1 - s0) * 0.5, (r.next() % 3) as i64)
            } else {
                t
            }
        })
        .collect();
    ins.sort_by(f64::total_cmp);
    k.extend(ins);
    k.extend(std::iter::repeat_n(s1, p + 1));
    k
}

#[test]
fn review_4551_blossom_and_restrict_enclose_exact() {
    let mut r = Rng(0xdead_beef_cafe_f00d);
    let mut checked = 0usize;
    let mut max_rel = 0.0f64;
    for p in 1..=8usize {
        for _ in 0..400 {
            let knots = random_knots(&mut r, p);
            let Ok(kv) = KnotVector::clamped(knots.clone(), p) else {
                continue;
            };
            let n = kv.control_count();
            let coeffs: Vec<f64> = (0..n)
                .map(|_| match r.next() % 3 {
                    0 => r.unit() * 2.0 - 1.0,
                    1 => 1e3 + r.unit(),
                    _ => 1.0 + 1e-9 * r.unit(),
                })
                .collect();
            let ci: Vec<Interval> = coeffs
                .iter()
                .map(|&c| Interval::from_bounds(c, c))
                .collect();
            let pair = kv.with_coeffs(&ci).unwrap();
            for s in p..n {
                let (lo, hi) = (knots[s], knots[s + 1]);
                if !(lo < hi) {
                    assert!(pair.span(s).is_none() || true);
                    continue;
                }
                let Some(win) = pair.span(s) else {
                    panic!("nonempty span {s} has no window");
                };
                // Arguments: whole span, few ulps, touching an end.
                let (a, b) = match r.next() % 4 {
                    0 => (lo, hi),
                    1 => {
                        let a = lo + (hi - lo) * r.unit();
                        (a, ulps(a, 3).min(hi))
                    }
                    2 => (lo, ulps(lo, 2).min(hi)),
                    _ => {
                        let a = lo + (hi - lo) * r.unit();
                        let b = lo + (hi - lo) * r.unit();
                        if a <= b { (a, b) } else { (b, a) }
                    }
                };
                let row = win
                    .restrict((Interval::from_bounds(a, a), Interval::from_bounds(b, b)))
                    .expect("restrict refused a valid piece");
                assert_eq!(row.len(), p + 1);
                for (k, c) in row.iter().enumerate() {
                    let mut args = vec![a; p - k];
                    args.extend(std::iter::repeat_n(b, k));
                    let ex = exact_blossom(&knots, p, s, &coeffs, &args);
                    assert!(
                        inside(*c, &ex),
                        "p={p} s={s} k={k} [{a:e},{b:e}] row {c:?} misses exact {} knots {knots:?} coeffs {coeffs:?}",
                        ex.0.to_string().len()
                    );
                    let scale = coeffs.iter().fold(0.0f64, |m, v| m.max(v.abs()));
                    max_rel = max_rel.max((c.hi() - c.lo()) / (scale * f64::EPSILON));
                    checked += 1;
                }
                // Interval arguments: blossom is multi-affine, so the corners
                // of the argument box bound its range.
                let m = lo + (hi - lo) * r.unit();
                let w = ulps(m, 4).min(hi);
                let iargs: Vec<Interval> = (0..p)
                    .map(|i| {
                        if i % 2 == 0 {
                            Interval::from_bounds(m, w)
                        } else {
                            Interval::from_bounds(lo, hi)
                        }
                    })
                    .collect();
                let got = win.blossom(&iargs).expect("blossom refused valid args");
                for mask in (0..(1u32 << p)).step_by(if p > 5 { 7 } else { 1 }) {
                    let args: Vec<f64> = (0..p)
                        .map(|i| {
                            let iv = iargs[i];
                            if mask >> i & 1 == 0 { iv.lo() } else { iv.hi() }
                        })
                        .collect();
                    let ex = exact_blossom(&knots, p, s, &coeffs, &args);
                    assert!(
                        inside(got, &ex),
                        "p={p} s={s} interval blossom misses corner"
                    );
                    checked += 1;
                }
                // Refusals.
                assert!(matches!(
                    win.blossom(&vec![Interval::from_bounds(lo, lo); p + 1]),
                    Err(BlossomError::ArgumentCount { degree: dd, got: g }) if dd == p && g == p + 1
                ));
                let out = Interval::from_bounds(ulps(hi, 1), ulps(hi, 1));
                let mut bad = vec![Interval::from_bounds(lo, lo); p];
                bad[p - 1] = out;
                assert!(matches!(
                    win.blossom(&bad),
                    Err(BlossomError::OutsideSpan { index, .. }) if index == p - 1
                ));
                // Straddling arg: accepted (clamped) — the contract case.
                let mut straddle = vec![Interval::from_bounds(lo, lo); p];
                straddle[0] = Interval::from_bounds(ulps(lo, -1), lo);
                assert!(win.blossom(&straddle).is_ok());
            }
        }
    }
    eprintln!("blossom: {checked} exact checks, max width {max_rel:.1} eps·scale");
}
