//! Enclosures of π-family constants and the conservative grid test.
//!
//! Direction facts (verified against the MPFR oracle in the harness, and
//! standard: the 53-bit round-to-nearest of π lands BELOW π, and scaling
//! by powers of two is exact):
//!   `f64::consts::PI       < π    < next_up(PI)`
//!   `f64::consts::FRAC_PI_2 < π/2 < next_up(FRAC_PI_2)`
//!   `f64::consts::TAU      < 2π   < next_up(TAU)`

use crate::interval::{DInterval, Decoration};

/// One-ulp-wide enclosure of π.
pub fn pi() -> DInterval {
    DInterval::make(
        core::f64::consts::PI,
        core::f64::consts::PI.next_up(),
        Decoration::Com,
    )
}

/// One-ulp-wide enclosure of 2π.
pub fn tau() -> DInterval {
    DInterval::make(
        core::f64::consts::TAU,
        core::f64::consts::TAU.next_up(),
        Decoration::Com,
    )
}

/// One-ulp-wide enclosure of π/2.
pub fn frac_pi_2() -> DInterval {
    DInterval::make(
        core::f64::consts::FRAC_PI_2,
        core::f64::consts::FRAC_PI_2.next_up(),
        Decoration::Com,
    )
}

/// One-ulp-wide enclosure of −π/2, for `sin`'s minimum grid.
///
/// It IS the negation of [`frac_pi_2`] rather than a second hand-written
/// pair of endpoints, because interval negation is exact (`arith.rs`'s
/// `Neg` swaps the endpoints and negates them, and f64 negation is
/// exact). That is what puts it under `certify_constants`' cross-check
/// against the MPFR oracle: the harness certifies `frac_pi_2`, and the
/// only step between them is exact.
///
/// `pub(crate)`, unlike its three siblings: they are re-exported from
/// `lib.rs` because the kernel's `Real` trait asks for them, and this one
/// is not on that surface.
pub(crate) fn neg_frac_pi_2() -> DInterval {
    -frac_pi_2()
}

/// Exact enclosure of 0, the phase offset of `cos`'s maxima.
///
/// Here rather than inline in `trig.rs` for the same reason
/// [`neg_frac_pi_2`] is here: `grid_possibly_hits`' four phase offsets
/// (`frac_pi_2`, `neg_frac_pi_2`, `zero`, `pi`) are one family, and a
/// family is only checkable while all of it is in one module — the four
/// are, and a fifth built at a call site would be the drift this
/// placement refuses.
pub(crate) fn zero() -> DInterval {
    DInterval::make(0.0, 0.0, Decoration::Com)
}

/// Conservative test: might the arithmetic grid `{c + k·p : k ∈ ℤ}`
/// intersect the (nonempty, non-NaI) interval `x`? `c` and `p` are tiny
/// enclosures of the (irrational) offset and period, `p > 0`.
///
/// Method: `K = (x − c) / p` in outward interval arithmetic; the grid
/// intersects `x` iff K contains an integer; outward rounding only WIDENS
/// K, so "K contains an integer" can only flip from false to true —
/// errors land on the conservative "possibly" side. Callers use:
/// - `possibly == true`  → include the extremum value / declare the pole
///   (loosening or extra poison, never unsoundness);
/// - `possibly == false` → PROOF that no grid point lies in `x`
///   (monotone-branch shortcut is then valid).
///
/// For very large `|x|` (≳ 2^52·p / width-of-p, i.e. ~4·10^15 here) K is
/// wider than 1 and always contains an integer: the test degrades to
/// "always possibly", which is exactly the documented graceful refusal
/// for huge arguments. Infinite bounds likewise return `true`.
///
/// **That is TOTAL degradation. Partial degradation starts seven
/// binades earlier, near `|x| ≈ 2^32`** — K's width passes 1 ulp of an
/// integer long before it passes 1 — and a false "possibly" there costs
/// a pinned extremum where the true image is a sliver. Measured: 0
/// false captures per 200 000 thin `sin` points below 2^31, 1 at 2^32,
/// 125 at 2^40, all 200 000 at 2^52. Sound throughout; loose from 2^32.
/// **`certify.rs`'s tightness ceilings depend on that number**, not on
/// the 2^52 one — see the constraint recorded on its `WINDOWS`.
pub(crate) fn grid_possibly_hits(x: &DInterval, c: DInterval, p: DInterval) -> bool {
    debug_assert!(!x.is_empty() && !x.is_nai());
    if !x.lo.is_finite() || !x.hi.is_finite() {
        return true;
    }
    let k = (*x - c) / p;
    debug_assert!(!k.is_empty() && !k.is_nai());
    if !k.lo.is_finite() || !k.hi.is_finite() {
        return true;
    }
    k.hi.floor() >= k.lo.ceil()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The direction facts this module's header asserts, checked rather
    /// than only written down. Each constant's enclosure must be the
    /// one-ulp bracket `[c, next_up(c)]` around the f64 constant, which
    /// is what makes it an enclosure of the irrational value given the
    /// header's rounding-direction facts (certified against MPFR in
    /// `certify_constants`).
    #[test]
    fn each_constant_is_the_one_ulp_bracket_above_its_f64() {
        for (iv, c, name) in [
            (pi(), core::f64::consts::PI, "pi"),
            (tau(), core::f64::consts::TAU, "tau"),
            (frac_pi_2(), core::f64::consts::FRAC_PI_2, "frac_pi_2"),
        ] {
            assert_eq!(iv.lo(), c, "{name}: lo");
            assert_eq!(iv.hi(), c.next_up(), "{name}: hi");
            assert_eq!(iv.decoration(), Decoration::Com, "{name}: dec");
        }
    }

    /// `neg_frac_pi_2` is the exact negation of `frac_pi_2`, endpoints
    /// swapped — the property that lets it inherit `frac_pi_2`'s
    /// certification instead of needing its own oracle row.
    #[test]
    fn neg_frac_pi_2_is_the_exact_negation_of_frac_pi_2() {
        let (p, n) = (frac_pi_2(), neg_frac_pi_2());
        assert_eq!(n.lo(), -p.hi());
        assert_eq!(n.hi(), -p.lo());
        assert_eq!(n.decoration(), p.decoration());
    }
}
