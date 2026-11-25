//! **Error-free transforms** — the primitives that hand a caller the
//! exact real result of a floating-point operation as a pair of `f64`s,
//! so a decision about the REAL value is available where the rounded
//! value would decide something weaker.
//!
//! A rounded comparison answers a different question from the one a
//! structural door usually asks. `fl(a + b) == fl(c + d)` is implied by
//! `a + b = c + d` in ℝ and does not imply it: on `lo = 0`, `hi = 1`
//! the pair `(½, ½ + 2⁻⁵³)` rounds to exactly `1.0` while its real sum
//! is half an ulp above it. A door that admits structure on the rounded
//! answer therefore admits structure that is an ulp away from what it
//! claims. [`two_sum`] is how such a door decides the real identity
//! instead, in six correctly-rounded operations and without any wider
//! arithmetic type.

/// Knuth's 2Sum: the exact sum of two `f64`s as the unevaluated pair
/// `(s, e)` with `s = fl(a + b)` and `s + e = a + b` **in ℝ**.
///
/// # Exactness precondition
///
/// The identity `s + e = a + b` holds whenever `a`, `b` and the HEAD
/// `s` are all finite. When the head overflows, `e` is a NaN and the
/// pair carries no information about the real sum — a caller that can
/// meet an infinite head must say what it does about that case rather
/// than read the residual. (There is no underflow caveat: the
/// subtractions below are exact in the subnormal range too.)
///
/// # Why the pair is a function of the operands' bits
///
/// Six IEEE-754 binary64 operations, each correctly rounded, none of
/// them a sum of three terms a compiler could contract into an FMA. So
/// the result does not vary with target or optimization level, and two
/// callers on different machines reach the same verdict.
///
/// # Deciding a real identity with it
///
/// Two such pairs agree componentwise exactly when the two real sums
/// agree: rounding is a function of the real value, so equal sums give
/// equal heads, and the residual is then determined by the head and
/// that sum. `two_sum(a, b) == two_sum(c, d)` is therefore
/// `a + b == c + d` in ℝ, for finite heads — and is `false` whenever a
/// head overflows, since a NaN residual compares equal to nothing,
/// including itself.
#[must_use]
pub fn two_sum(a: f64, b: f64) -> (f64, f64) {
    let s = a + b;
    let a_head = s - b;
    let b_head = s - a_head;
    (s, (a - a_head) + (b - b_head))
}

/// Dekker's product by fused multiply-add: the exact product of two
/// `f64`s as the unevaluated pair `(p, e)` with `p = fl(a · b)` and
/// `p + e = a · b` **in ℝ**.
///
/// # Exactness precondition
///
/// Finite operands whose product neither overflows nor underflows: the
/// residual of a product below the normal range rounds. `mul_add` is
/// correctly rounded on every target (in software where the hardware has
/// no FMA), so the pair is a function of the operands' bits.
#[must_use]
pub fn two_product(a: f64, b: f64) -> (f64, f64) {
    let p = a * b;
    (p, a.mul_add(b, -p))
}

/// **An exact sum of `f64`s, kept as a non-overlapping expansion**
/// (Shewchuk, "Adaptive Precision Floating-Point Arithmetic and Fast
/// Robust Geometric Predicates", 1997): components in increasing
/// magnitude whose real sum is the value, every operation error-free.
/// Zero components are dropped, so the last component, where there is
/// one, carries the value's sign.
///
/// The same preconditions as [`two_sum`] and [`two_product`]: finite
/// operands, no overflow, and for a product no underflow below the
/// normal range. A caller that can meet either reads [`Self::is_finite`]
/// and an underflowing product's own magnitude before trusting the sign.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Expansion(Vec<f64>);

impl Expansion {
    /// The exact product `a · b`.
    #[must_use]
    pub fn product(a: f64, b: f64) -> Self {
        let (p, e) = two_product(a, b);
        let mut out = Self::default();
        out.add(e);
        out.add(p);
        out
    }

    /// The exact `(q − o)·n`, as six exact products
    /// ([`crate::Real::difference_dot`]'s value at an enclosure).
    #[must_use]
    pub fn difference_dot(q: [f64; 3], o: [f64; 3], n: [f64; 3]) -> Self {
        let mut out = Self::default();
        for i in 0..3 {
            out.add_expansion(&Self::product(q[i], n[i]));
            out.add_expansion(&Self::product(-o[i], n[i]));
        }
        out
    }

    /// Adds `b` exactly (Shewchuk's Grow-Expansion, zero-eliminating).
    /// In place: each component written is at or before the one read.
    pub fn add(&mut self, b: f64) {
        let mut q = b;
        let mut kept = 0;
        for i in 0..self.0.len() {
            let (s, h) = two_sum(q, self.0[i]);
            if h != 0.0 {
                self.0[kept] = h;
                kept += 1;
            }
            q = s;
        }
        self.0.truncate(kept);
        if q != 0.0 {
            self.0.push(q);
        }
    }

    /// **The same value in as few components** (Shewchuk's Compress):
    /// a nonoverlapping expansion whose components are each the rounding
    /// of the sum of all below them, so a value read to its digits has a
    /// handful of components however many products formed it. Exact.
    /// Products of expansions cost the product of their lengths, so a
    /// term is compressed before it is multiplied.
    #[must_use]
    pub fn compressed(&self) -> Self {
        let e = &self.0;
        let Some(&top) = e.last() else {
            return Self::default();
        };
        let fast_two_sum = |a: f64, b: f64| {
            let s = a + b;
            (s, b - (s - a))
        };
        // Downward: carry the running sum from the largest, keeping each
        // part a later component does not absorb.
        let mut g = vec![0.0; e.len()];
        let mut bottom = e.len() - 1;
        let mut big = top;
        for i in (0..e.len() - 1).rev() {
            let (sum, small) = fast_two_sum(big, e[i]);
            if small == 0.0 {
                big = sum;
            } else {
                g[bottom] = sum;
                bottom -= 1;
                big = small;
            }
        }
        g[bottom] = big;
        // Upward: sum from the smallest, keeping each nonzero error.
        let mut h = Vec::with_capacity(e.len() - bottom);
        let mut big = g[bottom];
        for &gi in &g[bottom + 1..] {
            let (sum, small) = fast_two_sum(gi, big);
            if small != 0.0 {
                h.push(small);
            }
            big = sum;
        }
        if big != 0.0 {
            h.push(big);
        }
        Self(h)
    }

    /// Adds `other` exactly.
    pub fn add_expansion(&mut self, other: &Self) {
        for &c in &other.0 {
            self.add(c);
        }
    }

    /// `self · b` exactly (Shewchuk's Scale-Expansion, zero-eliminating).
    #[must_use]
    pub fn scaled(&self, b: f64) -> Self {
        let mut out = Self::default();
        for &e in &self.0 {
            let (p, r) = two_product(e, b);
            out.add(r);
            out.add(p);
        }
        out
    }

    /// `self · other` exactly: `self` scaled by each of `other`'s
    /// components, summed.
    #[must_use]
    pub fn times(&self, other: &Self) -> Self {
        let mut out = Self::default();
        for &c in &other.0 {
            out.add_expansion(&self.scaled(c));
        }
        out
    }

    /// The exact `p·n`, as three exact products.
    #[must_use]
    pub fn dot(p: [f64; 3], n: [f64; 3]) -> Self {
        let mut out = Self::default();
        for i in 0..3 {
            out.add_expansion(&Self::product(p[i], n[i]));
        }
        out
    }

    /// **The value, rounded** (Shewchuk's Estimate): the components
    /// summed from the smallest. Grow-Expansion ([`Self::add`]) keeps an
    /// expansion nonadjacent as well as nonoverlapping (Shewchuk, Lemma
    /// 12): no two components overlap or touch in their binary digits.
    /// So the components below the largest sum to less than its last
    /// bit, each partial sum taken smallest first is exact or rounds
    /// below that bit, and the one rounding at the value's scale is the
    /// last: the result is within `2u` of the value, relatively, whatever
    /// cancellation formed it. The order matters: summed largest first, a
    /// tail that would have carried the head past a rounding tie is lost
    /// (the row `the_estimate_sums_from_the_smallest`).
    #[must_use]
    pub fn estimate(&self) -> f64 {
        self.0.iter().sum()
    }

    /// The negation, exactly.
    #[must_use]
    pub fn negated(&self) -> Self {
        Self(self.0.iter().map(|c| -c).collect())
    }

    /// Whether every component is finite: the preconditions held as far
    /// as an overflow could show.
    #[must_use]
    pub fn is_finite(&self) -> bool {
        self.0.iter().all(|c| c.is_finite())
    }

    /// The components, in increasing magnitude, none zero.
    #[must_use]
    pub fn components(&self) -> &[f64] {
        &self.0
    }
}

/// **The exponent of `x`'s lowest set bit**: `x` is an integer multiple
/// of `2^k` and no larger power; `None` for zero, which constrains
/// nothing.
fn low_bit(x: f64) -> Option<i32> {
    if x == 0.0 || !x.is_finite() {
        return None;
    }
    let bits = x.to_bits();
    let biased = i32::try_from((bits >> 52) & 0x7ff).unwrap_or(0);
    let fraction = bits & ((1u64 << 52) - 1);
    let (mantissa, exponent) = if biased == 0 {
        (fraction, -1074)
    } else {
        (fraction | (1u64 << 52), biased - 1075)
    };
    Some(exponent + i32::try_from(mantissa.trailing_zeros()).unwrap_or(0))
}

/// **Whether every digit of [`harmonic_belly`]'s expansions is kept**
/// (its docs): checked on the inputs' own lowest bits.
pub(crate) fn kept_exactly(curve: crate::StoredHarmonic<f64>, o: [f64; 3], n: [f64; 3]) -> bool {
    let crate::StoredHarmonic {
        at,
        axis,
        u_ref,
        semi: (a, b),
        phase: (c0, s0),
    } = curve;
    // The least lowest bit of each group of factors; a group of zeros
    // contributes no monomial.
    let least = |xs: &[f64]| xs.iter().filter_map(|&x| low_bit(x)).min();
    let (lb_x, lb_n, lb_w, lb_u) = (
        [least(&at), least(&o)].into_iter().flatten().min(),
        least(&n),
        least(&axis),
        least(&u_ref),
    );
    let (lb_a, lb_b, lb_c, lb_s) = (least(&[a]), least(&[b]), least(&[c0]), least(&[s0]));
    let sum = |parts: &[Option<i32>]| parts.iter().try_fold(0i32, |s, p| p.map(|p| s + p));
    let t_a = sum(&[lb_a, lb_u, lb_n]);
    let t_b = sum(&[lb_b, lb_w, lb_u, lb_n]);
    let t_d = [
        sum(&[lb_x, lb_n]),
        t_a.and_then(|t| lb_c.map(|c| t + c)),
        t_b.and_then(|t| lb_s.map(|s| t + s)),
    ]
    .into_iter()
    .flatten()
    .min();
    // Each square's factors; and the prefixes `A` and `B` are built
    // through before their semi-axis scales them, `u_ref·n` and (in
    // `triple_product`) `axis·(u_ref·n)`, deeper than half of `A²` or `B²`
    // where `a`'s or `b`'s lowest bit is high.
    let squares = [t_a, t_b, t_d].into_iter().flatten().map(|t| 2 * t);
    let prefixes = [sum(&[lb_u, lb_n]), sum(&[lb_w, lb_u, lb_n])];
    let deepest = squares.chain(prefixes.into_iter().flatten()).min();
    !deepest.is_some_and(|d| d < -1074)
}

/// **The discriminant `Δ` and the depth `D = 64s − 16p²` of the
/// depressed monic quartic `y⁴ + p y² + q̂ y + s`, exactly**, as
/// expansions of the stored coefficients
/// ([`crate::Real::quartic_sign_algebra`]):
///
/// ```text
/// Δ = 256s³ − 128p²s² + 144pq̂²s − 27q̂⁴ + 16p⁴s − 4p³q̂²
/// ```
///
/// Every product the evaluation forms, each monomial and each prefix it
/// is built through (`p⁴` before `p⁴s`), is a product of the
/// coefficients, so its exact value is a multiple of `2` to the sum of
/// their lowest bits ([`low_bit`]). Where that sum is at least `−1074`
/// for every product formed, each error-free product's error term is a
/// representable multiple of the least subnormal, so every product is
/// exact and so is the expansion. `None` where one is not, or where a
/// term overflows.
#[must_use]
pub fn quartic_sign_algebra(p: f64, q_hat: f64, s: f64) -> Option<[Expansion; 2]> {
    let (lp, lq, ls) = (low_bit(p), low_bit(q_hat), low_bit(s));
    // A product's lowest bit, `None` where a factor is zero (it is
    // absent).
    let lowest = |factors: &[(Option<i32>, i32)]| {
        factors
            .iter()
            .try_fold(0i32, |acc, &(bit, k)| bit.map(|b| acc + k * b))
    };
    // Every product below, in the order it is formed: the squares, then
    // each term's prefixes and the term. `D`'s `p²` is the first, and
    // `64s` scales `s` by a power of two, exact short of overflow.
    let deepest = [
        &[(lp, 2)][..],
        &[(lq, 2)],
        &[(ls, 2)],
        &[(ls, 3)],
        &[(lp, 2), (ls, 2)],
        &[(lp, 1), (lq, 2)],
        &[(lp, 1), (lq, 2), (ls, 1)],
        &[(lq, 4)],
        &[(lp, 4)],
        &[(lp, 4), (ls, 1)],
        &[(lp, 2), (lq, 2)],
        &[(lp, 3), (lq, 2)],
    ]
    .into_iter()
    .filter_map(lowest)
    .min();
    if deepest.is_some_and(|d| d < -1074) {
        return None;
    }
    let (p2, q2, s2) = (
        Expansion::product(p, p),
        Expansion::product(q_hat, q_hat),
        Expansion::product(s, s),
    );
    let mut disc = s2.scaled(s).scaled(256.0);
    for term in [
        p2.times(&s2).scaled(-128.0),
        q2.scaled(p).scaled(s).scaled(144.0),
        q2.times(&q2).scaled(-27.0),
        p2.times(&p2).scaled(s).scaled(16.0),
        p2.times(&q2).scaled(p).scaled(-4.0),
    ] {
        disc.add_expansion(&term.compressed());
    }
    let mut depth = p2.scaled(-16.0);
    depth.add_expansion(&Expansion::product(s, 64.0));
    (disc.is_finite() && depth.is_finite()).then(|| [disc.compressed(), depth.compressed()])
}

/// A double-double `hi + lo`, its parts formed by error-free transforms.
#[derive(Clone, Copy)]
struct Dd {
    hi: f64,
    lo: f64,
}

impl Dd {
    fn of(x: f64) -> Self {
        Self { hi: x, lo: 0.0 }
    }

    fn product(a: f64, b: f64) -> Self {
        let (hi, lo) = two_product(a, b);
        Self { hi, lo }
    }

    fn normal(hi: f64, lo: f64) -> Self {
        let (hi, lo) = two_sum(hi, lo);
        Self { hi, lo }
    }

    fn plus(self, other: Self) -> Self {
        let (s, e) = two_sum(self.hi, other.hi);
        Self::normal(s, e + (self.lo + other.lo))
    }

    fn minus(self, other: Self) -> Self {
        self.plus(Self {
            hi: -other.hi,
            lo: -other.lo,
        })
    }

    fn times(self, other: Self) -> Self {
        let (p, e) = two_product(self.hi, other.hi);
        Self::normal(p, e + (self.hi * other.lo + self.lo * other.hi))
    }
}

/// **[`harmonic_belly`]'s value where a double-double reading already
/// holds it** (the filter before the expansions; `None` where it does
/// not). `A`, `B`, `D` and `A² + B² − D²` are formed in double-double
/// beside the magnitudes `m_A`, `m_B`, `m_D` of their monomials summed
/// absolutely. Each double-double step errs by at most a few `u²` of its
/// operands' magnitudes and no path takes more than a dozen steps, so
/// `A`, `B`, `D` err by at most `2⁶u²` of their magnitudes and the
/// numerator by at most `2¹⁰u²·M`, `M = m_A² + m_B² + m_D²`. The reading
/// answers only where the numerator's head clears `2¹¹u·M`. The
/// denominator then clears its own error by as much: the numerator is
/// `(√(A² + B²) − |D|)(√(A² + B²) + |D|)`, at most the denominator's
/// square, so the denominator is at least `√(2¹¹u·M)`, far past
/// `2⁶u²·(m_A + m_B + m_D)`. So the numerator errs by at most half a
/// `u` of itself (`2¹⁰u²·M` over `2¹¹u·M`) and the denominator by far
/// less, within the `2u` the exact reading spends on its numerator, and
/// the belly is within the same `7u`. A belly
/// that cancels against the curve's size never clears it, and is read
/// from the expansions.
#[must_use]
pub fn harmonic_belly_near(
    curve: crate::StoredHarmonic<f64>,
    o: [f64; 3],
    n: [f64; 3],
) -> Option<f64> {
    let crate::StoredHarmonic {
        at,
        axis: w,
        u_ref: u,
        semi: (a, b),
        phase: (c0, s0),
    } = curve;
    let mut big_a = Dd::of(0.0);
    let mut m_a = 0.0;
    for i in 0..3 {
        big_a = big_a.plus(Dd::product(u[i], n[i]));
        m_a += (u[i] * n[i]).abs();
    }
    let big_a = big_a.times(Dd::of(a));
    let m_a = m_a * a.abs();
    // `(w × u)·n`, its six monomials.
    let mut big_b = Dd::of(0.0);
    let mut m_b = 0.0;
    for (i, j, k) in [(0, 1, 2), (1, 2, 0), (2, 0, 1)] {
        let across = Dd::product(u[j], n[k]).minus(Dd::product(u[k], n[j]));
        big_b = big_b.plus(across.times(Dd::of(w[i])));
        m_b += (w[i] * u[j] * n[k]).abs() + (w[i] * u[k] * n[j]).abs();
    }
    let big_b = big_b.times(Dd::of(b));
    let m_b = m_b * b.abs();
    let mut d = Dd::of(0.0);
    let mut m_d = 0.0;
    for i in 0..3 {
        let (diff, err) = two_sum(at[i], -o[i]);
        d = d.plus(Dd { hi: diff, lo: err }.times(Dd::of(n[i])));
        m_d += (at[i].abs() + o[i].abs()) * n[i].abs();
    }
    let d = d
        .minus(big_a.times(Dd::of(c0)))
        .minus(big_b.times(Dd::of(s0)));
    let m_d = m_d + c0.abs() * m_a + s0.abs() * m_b;
    let squares = big_a
        .times(big_a)
        .plus(big_b.times(big_b))
        .minus(d.times(d));
    let u_ = f64::EPSILON / 2.0;
    let magnitude = m_a.powi(2) + m_b.powi(2) + m_d.powi(2);
    let rise = big_a.hi.hypot(big_b.hi) + d.hi.abs();
    let clear = squares.hi.abs() > 2048.0 * u_ * magnitude && magnitude.is_finite();
    clear.then(|| squares.hi / rise)
}

/// **A first harmonic's belly across a plane, its terms exact**: for a
/// curve stored as [`crate::StoredHarmonic`] (the point `x` it stores,
/// its axis `w`, seam `u` and semi-axes `(a, b)`, `x` at the phase
/// `(c₀, s₀)` on it), its height over the plane through `o` with normal
/// `n` is `D + A cos θ + B sin θ`, with
///
/// - `A = a·(u·n)`, `B = b·((w × u)·n)`, the across-seam direction
///   crossed exactly rather than read rounded;
/// - `D = (x − o)·n − c₀·A − s₀·B`, the centre's height formed from the
///   stored point, so no derived centre is rounded.
///
/// Answers the expansions `(A² + B² − D², [A, B, D])`
/// ([`crate::Real::harmonic_belly`]'s value is the first over
/// `√(A² + B²) + |D|`), or `None` where an expansion could not be
/// exact. **When every digit is kept**: each component of each
/// expansion is an integer multiple of the product of its monomial's
/// factors' lowest set bits, so every error-free sum and product below
/// is exact while that product is at least `2^−1074`, the least
/// subnormal's (a product's error term is then a representable multiple
/// of it); the deepest monomials are `D²`'s, `(s₀·b·w·u·n)²`. That is
/// checked on the inputs' own lowest bits, and an overflow on the
/// components' finiteness.
#[must_use]
pub fn harmonic_belly(
    curve: crate::StoredHarmonic<f64>,
    o: [f64; 3],
    n: [f64; 3],
) -> Option<(Expansion, [Expansion; 3])> {
    let crate::StoredHarmonic {
        at,
        axis,
        u_ref,
        semi: (a, b),
        phase: (c0, s0),
    } = curve;
    if !kept_exactly(curve, o, n) {
        return None;
    }
    let big_a = Expansion::dot(u_ref, n).scaled(a).compressed();
    let big_b = triple_product(axis, u_ref, n).scaled(b).compressed();
    let mut d = Expansion::difference_dot(at, o, n);
    d.add_expansion(&big_a.scaled(c0).negated());
    d.add_expansion(&big_b.scaled(s0).negated());
    let d = d.compressed();
    let mut squares = big_a.times(&big_a);
    squares.add_expansion(&big_b.times(&big_b));
    squares.add_expansion(&d.times(&d).negated());
    let parts = [big_a, big_b, d];
    (squares.is_finite() && parts.iter().all(Expansion::is_finite)).then_some((squares, parts))
}

/// **The exact determinant `p · (q × r)`** of three points' coordinates,
/// six times the signed volume of the tetrahedron they span with the
/// world origin.
#[must_use]
pub fn triple_product(p: [f64; 3], q: [f64; 3], r: [f64; 3]) -> Expansion {
    let mut out = Expansion::default();
    for (i, j, k) in [(0, 1, 2), (1, 2, 0), (2, 0, 1)] {
        out.add_expansion(&Expansion::product(q[j], r[k]).scaled(p[i]));
        out.add_expansion(&Expansion::product(q[k], r[j]).scaled(p[i]).negated());
    }
    out
}

#[cfg(test)]
mod tests {
    use super::{Expansion, two_sum};

    /// **Compressing keeps the value exactly** ([`Expansion::compressed`]):
    /// over sums of products spanning many binades and cancelling, the
    /// compressed expansion less the original is exactly zero, it has no
    /// more components, they increase strictly in magnitude, and it
    /// reads the same estimate.
    #[test]
    fn compressing_keeps_the_value_exactly() {
        let mut seed = 0x2545_f491_4f6c_dd1du64;
        let mut next = || {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            #[allow(clippy::cast_precision_loss)]
            let x = (seed >> 11) as f64 / (1u64 << 53) as f64;
            x - 0.5
        };
        let mut shrank = false;
        for _ in 0..2000 {
            let mut e = Expansion::default();
            for k in 0..8 {
                let scale = 10f64.powi(4 * k - 16);
                let (x, y) = (next() * scale, next());
                e.add_expansion(&Expansion::product(x, y));
                e.add_expansion(&Expansion::product(-x, y + 1e-9 * next()));
            }
            let c = e.compressed();
            let mut diff = c.clone();
            diff.add_expansion(&e.negated());
            assert!(diff.components().is_empty(), "{e:?} compressed to {c:?}");
            assert!(c.components().len() <= e.components().len());
            assert!(
                c.components().windows(2).all(|w| w[0].abs() < w[1].abs()),
                "{c:?} is not increasing"
            );
            assert_eq!(c.estimate(), e.estimate(), "{e:?}");
            shrank |= c.components().len() < e.components().len();
        }
        assert!(shrank, "no draw had anything to compress");
    }

    /// **The estimate sums from the smallest component**
    /// ([`Expansion::estimate`]): the nonadjacent expansion
    /// `2⁻¹⁰⁵ + 2⁻⁵³ + 1` lies just past the tie between `1` and
    /// `1 + 2⁻⁵²`, so it rounds up. Summed from the smallest the tail
    /// carries the sum past the tie; summed from the largest, `1 + 2⁻⁵³`
    /// ties to `1` and the tail is lost.
    #[test]
    fn the_estimate_sums_from_the_smallest() {
        let e = Expansion(vec![2f64.powi(-105), 2f64.powi(-53), 1.0]);
        assert_eq!(e.estimate(), 1.0 + 2f64.powi(-52), "{e:?}");
    }

    /// The residual is the real remainder, and it is what separates a
    /// pair whose real sum misses a target from one that hits it even
    /// though both round to the target.
    #[test]
    fn the_residual_separates_sums_a_rounded_compare_conflates() {
        let half_up = f64::from_bits(0.5f64.to_bits() + 1);
        assert_eq!(0.5 + half_up, 1.0, "the rounded sums agree");
        assert_ne!(
            two_sum(0.5, half_up),
            two_sum(0.0, 1.0),
            "the exact sums do not: the residual carries the missing 2⁻⁵³"
        );
        assert_eq!(
            two_sum(0.4, 0.6),
            two_sum(0.0, 1.0),
            "a pair whose real sum IS 1 agrees in both components"
        );
    }

    /// An overflowing head has a NaN residual, so the pair never
    /// compares equal — not even to itself.
    #[test]
    fn an_overflowing_head_carries_a_nan_residual() {
        let r = two_sum(1e308, 1.5e308);
        assert!(r.0.is_infinite(), "the head overflows");
        assert!(r.1.is_nan(), "so the residual is meaningless, and says so");
        assert_ne!(r, r, "a pair with a NaN component equals nothing");
    }

    /// Subnormal operands need no caveat: the identity is exact there.
    #[test]
    fn the_subnormal_range_is_exact() {
        let tiny = f64::from_bits(1);
        let (s, e) = two_sum(tiny, tiny * 3.0);
        assert_eq!((s, e), (tiny * 4.0, 0.0), "exact, with no residual left");
    }

    /// The expansion holds a sum the rounded one loses, and its last
    /// component carries the sign.
    #[test]
    fn an_expansion_keeps_a_sum_rounding_loses_and_its_sign() {
        use super::{Expansion, triple_product};
        let mut e = Expansion::default();
        for x in [1.0, 1e-30, -1.0] {
            e.add(x);
        }
        assert_eq!(e.components(), &[1e-30], "1 + 1e-30 − 1 is 1e-30, exactly");
        let tiny = 1e-20;
        let det = triple_product([1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [tiny, tiny, tiny]);
        assert_eq!(
            det.components().last().copied(),
            Some(tiny),
            "the z of r, exactly"
        );
        let flat = triple_product([0.1, 0.2, 0.3], [0.2, 0.4, 0.6], [0.3, 0.5, 0.7]);
        assert!(
            flat.components().iter().all(|c| c.abs() < 1e-17),
            "two parallel rows: zero up to their own rounded inputs, {flat:?}"
        );
    }

    /// **A depressed quartic's `Δ` and `D` are read to their own
    /// digits** ([`crate::Real::quartic_sign_algebra`]). Each fixture is
    /// `(y² − 2a y + a² ± δ²)(y² + 2a y + c)`: a pair `δ` apart (or `δ`
    /// off the real line) at `a`, and two more, at a spread of 1 m to
    /// 1000 km, `δ` from a millionth of it down to a trillionth. At `f64`
    /// each value is the exact one's rounding, within `2u` of it; the
    /// plain formula, on the same coefficients, is off by more than a
    /// thousandth of `Δ` itself somewhere, and with the wrong sign
    /// somewhere, so a value read off it would fail the first assertion
    /// and certify a count the coefficients do not have. At `Interval`,
    /// stated points enclose the exact value.
    #[test]
    #[allow(clippy::panic)]
    fn a_quartics_sign_algebra_keeps_its_digits() {
        use crate::real::Bounds;
        use crate::{Interval, Real};
        let u = f64::EPSILON / 2.0;
        let (mut plain_worst, mut plain_flips, mut fixtures) = (0.0f64, 0, 0);
        for spread in [1.0, 1e3, 1e6] {
            for gap in [1e-6, 1e-9, 1e-12] {
                for (a, c) in [(0.31, -0.57), (-0.12, 0.08), (0.45, 0.2)] {
                    for pair in [1.0, -1.0] {
                        let (a, c, delta) = (a * spread, c * spread * spread, gap * spread);
                        let m = a * a + pair * delta * delta;
                        let (p, q, s) = (c + m - 4.0 * a * a, 2.0 * a * (m - c), m * c);
                        let Some([disc, depth]) = super::quartic_sign_algebra(p, q, s) else {
                            panic!("({p:e}, {q:e}, {s:e}): its digits fit an expansion");
                        };
                        let got = f64::quartic_sign_algebra(p, q, s);
                        let plain = crate::real::plain_quartic_sign_algebra(p, q, s);
                        let enclosed = Interval::quartic_sign_algebra(
                            Interval::from_f64(p),
                            Interval::from_f64(q),
                            Interval::from_f64(s),
                        );
                        for (i, exact) in [disc, depth].iter().enumerate() {
                            let want = exact.estimate();
                            let off = |x: f64| {
                                let mut gap = exact.clone();
                                gap.add(-x);
                                gap.estimate().abs()
                            };
                            assert!(
                                off(got[i]) <= 2.0 * u * want.abs(),
                                "({p:e}, {q:e}, {s:e}) part {i}: {} for exact {want:e}",
                                got[i]
                            );
                            assert!(
                                enclosed[i].lo() <= want && want <= enclosed[i].hi(),
                                "({p:e}, {q:e}, {s:e}) part {i}: {:?} misses {want:e}",
                                enclosed[i]
                            );
                            if i == 0 && want != 0.0 {
                                plain_worst = plain_worst.max(off(plain[0]) / want.abs());
                                plain_flips += usize::from(plain[0].signum() != want.signum());
                            }
                        }
                        fixtures += 1;
                    }
                }
            }
        }
        assert_eq!(fixtures, 54, "every fixture read");
        assert!(
            plain_worst > 1e-3 && plain_flips > 0,
            "the plain formula is off by {plain_worst:e} of Δ with {plain_flips} sign flips: \
             the fixtures do not cancel"
        );
    }

    test_utils::gated_to!["crates/geom-core/src/exact.rs"];

    /// **A product formed through a prefix below the least subnormal is
    /// not read as exact.** `p⁴` is formed before `p⁴s`: at
    /// `p = 3·2⁻²⁷⁰`, `p⁴`'s lowest bit is `2⁻¹⁰⁸⁰`, past the subnormals,
    /// while `p⁴s` at `s = 64` would be in range; read as exact it was off
    /// the exact `Δ` by 1.3e-321. So is `u_ref·n` before `A = a·(u_ref·n)`
    /// in [`super::kept_exactly`], whose square alone is checked
    /// otherwise.
    #[test]
    fn a_prefix_below_the_least_subnormal_poisons() {
        use crate::Real;
        let p = 3.0 * 2f64.powi(-270);
        assert!(super::quartic_sign_algebra(p, 0.0, 64.0).is_none());
        assert!(f64::quartic_sign_algebra(p, 0.0, 64.0)[0].is_nan());
        let p = -3.0 * 2f64.powi(-270);
        assert!(super::quartic_sign_algebra(p, 2f64.powi(-60), 2f64.powi(200)).is_none());
        let fine = 3.0 * 2f64.powi(-600);
        let curve = crate::StoredHarmonic {
            at: [0.0; 3],
            axis: [0.0, 0.0, 1.0],
            u_ref: [fine, 1.0, 0.0],
            semi: (2f64.powi(700), 0.0),
            phase: (0.0, 0.0),
        };
        assert!(
            !super::kept_exactly(curve, [0.0; 3], [fine, 0.0, 1.0]),
            "u_ref·n's lowest bit is 2⁻¹²⁰⁰"
        );
    }

    /// **`B`'s prefix `axis·(u_ref·n)` below the least subnormal is not
    /// kept**: `triple_product` forms it before `b` scales it, and only
    /// `B²` and `u_ref·n` were checked otherwise.
    #[test]
    fn a_triple_prefix_below_the_least_subnormal_is_not_kept() {
        let t = 3.0 * 2f64.powi(-500);
        let curve = crate::StoredHarmonic {
            at: [0.0; 3],
            axis: [3.0 * 2f64.powi(-100), 0.0, 1.0],
            u_ref: [0.0, t, 0.0],
            semi: (2f64.powi(700), 2f64.powi(700)),
            phase: (0.0, 0.0),
        };
        // w·(u·n) = 27·2⁻¹¹⁰⁰ rounds to nothing; B's exact value is 27·2⁻⁴⁰⁰.
        assert!(!super::kept_exactly(curve, [0.0; 3], [0.0, 0.0, t]));
    }

    /// **A quartic reading past the subnormals, or overflowing, is
    /// poison**, and one exactly at the least subnormal is read.
    #[test]
    fn a_quartic_reading_past_the_subnormals_is_poison() {
        use crate::Real;
        let at = |p: f64, q: f64, s: f64| f64::quartic_sign_algebra(p, q, s);
        // `2f64.powi(-1074)` is zero (it inverts an overflowing `2¹⁰⁷⁴`): the
        // least subnormal by its bits.
        assert_eq!(at(0.0, 0.0, 2f64.powi(-358))[0], 256.0 * f64::from_bits(1));
        assert!(
            at(0.0, 0.0, 2f64.powi(-359))[0].is_nan(),
            "s³ below the least subnormal"
        );
        assert!(
            at(0.0, 2f64.powi(-269), 0.0)[0].is_nan(),
            "q̂⁴ below the least subnormal"
        );
        assert!(at(1.0, 1.0, f64::MAX)[0].is_nan(), "overflow");
    }

    /// **A quartic's `Δ` and `D` against an integer oracle** (a
    /// counterexample search: the seed varies). Near-repeated-root
    /// integer quartics `(y² − 2a y + a² ± δ²)(y² + 2a y + c)`, their
    /// `Δ` and `D` exact in `i128`, rescaled by `2^(2k)`, `2^(3k)`,
    /// `2^(4k)` (`Δ` scales by `2^(12k)` and `D` by `2^(4k)`, exactly) at
    /// `k` = −80, 0 and 60: the `f64` reading has the oracle's sign, is
    /// zero exactly where it is, and is within `3u` of the oracle's own
    /// rounding (its own `2u`, and the oracle's one rounding to `f64`).
    #[test]
    fn a_quartics_sign_algebra_matches_an_integer_oracle() {
        use crate::Real;
        use test_utils::fuzz;
        let mut rng = fuzz::start("exact::quartic_sign_algebra::integer_oracle");
        let u = f64::EPSILON / 2.0;
        let int = |rng: &mut fuzz::Rng, bound: usize| {
            i128::try_from(rng.below(2 * bound + 1)).unwrap_or(0)
                - i128::try_from(bound).unwrap_or(0)
        };
        let mut checked = 0usize;
        for _ in 0..fuzz::scaled(3_000) {
            let a = int(&mut rng, 1 << 9);
            let c = int(&mut rng, 1 << 18);
            let delta = int(&mut rng, 4);
            let sign = if rng.below(2) == 0 { 1 } else { -1 };
            let m = a * a + sign * delta * delta;
            let (p, q, s) = (c + m - 4 * a * a, 2 * a * (m - c), m * c);
            let disc = 256 * s * s * s - 128 * p * p * s * s + 144 * p * q * q * s
                - 27 * q * q * q * q
                + 16 * p * p * p * p * s
                - 4 * p * p * p * q * q;
            let depth = 64 * s - 16 * p * p;
            #[allow(clippy::cast_precision_loss)]
            let f = |x: i128| x as f64;
            for k in [-80i32, 0, 60] {
                let scale = |e: i32| 2f64.powi(e * k);
                let got =
                    f64::quartic_sign_algebra(f(p) * scale(2), f(q) * scale(3), f(s) * scale(4));
                for (part, (exact, e)) in [(disc, 12), (depth, 4)].into_iter().enumerate() {
                    let want = f(exact) * scale(e);
                    let ok = if exact == 0 {
                        got[part] == 0.0
                    } else {
                        got[part].signum() == want.signum()
                            && (got[part] - want).abs() <= 3.0 * u * (1.0 + u) * want.abs()
                    };
                    assert!(
                        ok,
                        "(a, c, δ, ±) = ({a}, {c}, {delta}, {sign}) at k = {k}, part {part}: \
                         {} against {want:e}; {}",
                        got[part],
                        fuzz::replay()
                    );
                    checked += 1;
                }
            }
        }
        assert!(checked > 0, "the oracle read nothing; {}", fuzz::replay());
    }
}
