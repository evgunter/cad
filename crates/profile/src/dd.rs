//! **Double-double residuals** for the test oracles: how far an `f64`
//! point lies off a circle or a line, and where the radical foot of two
//! circles truly is, to about `1e-30` of the inputs' scale. Each takes
//! its inputs as stored and evaluates the defining equation, so it
//! shares no algebra with the kernel's placement.

/// The unevaluated sum `hi + lo`, `|lo| ≤ ulp(hi)/2`.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Dd {
    pub hi: f64,
    pub lo: f64,
}

fn two_sum(a: f64, b: f64) -> Dd {
    let s = a + b;
    let v = s - a;
    Dd {
        hi: s,
        lo: (a - (s - v)) + (b - v),
    }
}

fn quick_two_sum(a: f64, b: f64) -> Dd {
    let s = a + b;
    Dd {
        hi: s,
        lo: b - (s - a),
    }
}

fn two_prod(a: f64, b: f64) -> Dd {
    let p = a * b;
    Dd {
        hi: p,
        lo: a.mul_add(b, -p),
    }
}

impl Dd {
    pub(crate) fn of(x: f64) -> Self {
        Self { hi: x, lo: 0.0 }
    }

    /// `a − b`, exactly.
    pub(crate) fn diff(a: f64, b: f64) -> Self {
        two_sum(a, -b)
    }

    pub(crate) fn add(self, o: Self) -> Self {
        let s = two_sum(self.hi, o.hi);
        let t = two_sum(self.lo, o.lo);
        let s = quick_two_sum(s.hi, s.lo + t.hi);
        quick_two_sum(s.hi, s.lo + t.lo)
    }

    pub(crate) fn neg(self) -> Self {
        Self {
            hi: -self.hi,
            lo: -self.lo,
        }
    }

    pub(crate) fn sub(self, o: Self) -> Self {
        self.add(o.neg())
    }

    pub(crate) fn mul(self, o: Self) -> Self {
        let p = two_prod(self.hi, o.hi);
        quick_two_sum(p.hi, p.lo + (self.hi * o.lo + self.lo * o.hi))
    }

    pub(crate) fn div(self, o: Self) -> Self {
        let q1 = self.hi / o.hi;
        let r = self.sub(o.mul(Self::of(q1)));
        let q2 = r.hi / o.hi;
        let r = r.sub(o.mul(Self::of(q2)));
        quick_two_sum(q1, q2).add(Self::of(r.hi / o.hi))
    }

    pub(crate) fn sq(self) -> Self {
        self.mul(self)
    }

    pub(crate) fn value(self) -> f64 {
        self.hi + self.lo
    }
}

type P = (f64, f64);

/// The signed distance of `p` from the circle `c`, `r`: `|p − c| − r`,
/// as `(|p − c|² − r²)/(|p − c| + r)` with the numerator in
/// double-double.
pub(crate) fn off_circle(p: P, c: P, r: f64) -> f64 {
    let (x, y) = (Dd::diff(p.0, c.0), Dd::diff(p.1, c.1));
    let power = x.sq().add(y.sq()).sub(Dd::of(r).sq());
    power.value() / (x.value().hypot(y.value()) + r)
}

/// The radical foot of the circles `c1`, `r1` and `c2`, `r2`:
/// `c₁ + (c₂ − c₁)·(D² + r₁² − r₂²)/2D²`, `D = |c₂ − c₁|`, in
/// double-double.
pub(crate) fn radical_foot(c1: P, r1: f64, c2: P, r2: f64) -> (Dd, Dd) {
    let (x, y) = (Dd::diff(c2.0, c1.0), Dd::diff(c2.1, c1.1));
    let dd = x.sq().add(y.sq());
    let along = dd.add(Dd::of(r1).sq()).sub(Dd::of(r2).sq()).div(dd.add(dd));
    (
        Dd::of(c1.0).add(x.mul(along)),
        Dd::of(c1.1).add(y.mul(along)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The residual separates a point an ulp off a large circle from one
    /// on it, which `f64`'s `|p − c| − r` cannot.
    #[test]
    fn the_residual_resolves_an_ulp_at_scale() {
        let (c, r) = ((1e5, -3e4), 3.0 * 2f64.powi(10));
        let on = (c.0 + r, c.1);
        let off = (f64::from_bits(on.0.to_bits() + 1), on.1);
        assert_eq!(off_circle(on, c, r), 0.0, "a point on the circle");
        assert_eq!(
            off_circle(off, c, r),
            off.0 - on.0,
            "a point one ulp off it, by that ulp"
        );
    }

    /// The foot of a 3-4-5 pair is exact: `D² = 25`, the circles meet
    /// at `x = (25 + 16 − 9)/10 = 3.2` from the first centre.
    #[test]
    fn the_radical_foot_of_a_pythagorean_pair_is_exact() {
        let (x, y) = radical_foot((1.0, 2.0), 4.0, (4.0, 6.0), 3.0);
        let want = (1.0 + 3.0 * 0.64, 2.0 + 4.0 * 0.64);
        assert!(
            (x.value() - want.0).abs() < 1e-15 && (y.value() - want.1).abs() < 1e-15,
            "foot ({}, {}) against {want:?}",
            x.value(),
            y.value()
        );
    }
}
