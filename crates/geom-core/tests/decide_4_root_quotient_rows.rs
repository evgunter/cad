//! **Rule G's exact quotient** ([`SymRules::root_quotient`]) — a root
//! over `N/D` with `D` dividing `N` exactly is minted over the
//! polynomial quotient, before the split is asked — driven through
//! `Sym<Interval>` at the scalar door, with the dial on
//! ([`SymRules::shipped`]) and off ([`SymRules::without_root_quotient`]).
//!
//! **Every zero the tier claims is checked POINTWISE.** A shape is a
//! function of a parameter constructor, so the same expression is
//! rebuilt with every parameter pinned at points of its box (degenerate
//! boxes, tight enclosures): a claimed zero must be a zero of the real
//! residual there, to a tolerance that does not widen with the box. A
//! check against the box's own enclosure gets easier as the enclosure
//! degrades, which is the wrong direction for a soundness row.
//!
//! What is held here:
//!
//! 1. **The folds**: the boss's shape, one and several indeterminates, a
//!    non-square `Q` against its own spelling, a `Q` with a zero in the
//!    box, a negative `D` no form shows negative, a grlex tie-break, the
//!    division's reach at the budget.
//! 2. **The declines**: `N | D`, a constant `D`, a shared factor neither
//!    half divides, and two near misses — one whose remainder is a
//!    CONSTANT, the shape a division that dropped its remainder would
//!    call exact.
//! 3. **The negative rows**: a sign-carrying square's quotient root is
//!    `|R|` and never `R`, on boxes where the TIER is asked (the value
//!    channel cannot classify the residual), asserted as a refusal.
//! 4. **The trade** (`sym/root.rs`'s header): with the dial on the root
//!    is `sqrt(Q)`, so it meets `sqrt(Q)` even where the split proves
//!    `D`'s sign, and it NO LONGER meets the split spelling
//!    `sqrt(N)/sqrt(D)` it met with the dial off —
//!    `work/decide/the-exact-quotient-re-keys-a-root-the-split-met`.

#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::predicate::{Band, Margin, Sign};
use geom_core::sym::with_session_rules;
use geom_core::{Decide, Interval, ParamSymbol, Real, Sym, SymBudget, SymCounts, SymRules, Tol};

fn budget() -> SymBudget {
    SymBudget {
        max_terms: 4096,
        max_degree: 128,
    }
}

fn band() -> Band {
    Band::linear(Tol::witness()).expect("linear band")
}

fn lit(v: f64) -> Sym<Interval> {
    Sym::from_f64(v)
}

/// A parameter constructor: `(name, lo, hi)` to the scalar.
type Params<'a> = &'a dyn Fn(&str, f64, f64) -> Sym<Interval>;

/// A residual as a function of how its parameters are made.
type Shape = fn(Params) -> Sym<Interval>;

/// The parameter over its whole box.
fn over(name: &str, lo: f64, hi: f64) -> Sym<Interval> {
    Sym::param_over(ParamSymbol::of(name), Interval::from_bounds(lo, hi), lo, hi)
}

/// How the tier answered, as one word. `refused` is the tier ASKED and
/// declining: the value channel could not classify the residual.
fn label(out: Result<Sign, geom_core::predicate::Indeterminate>, c: SymCounts) -> String {
    match out {
        Ok(Sign::Zero) if c.symbolic_zero > 0 => "theorem".to_owned(),
        Ok(Sign::Zero) if c.registered > 0 => "registered".to_owned(),
        Ok(Sign::Zero) if c.sign_gated > 0 => "sign_gated".to_owned(),
        Ok(s) => format!("numeric {s:?}"),
        Err(_) => "refused".to_owned(),
    }
}

/// **The pointwise soundness check**: the residual rebuilt with every
/// parameter pinned at the low end, an interior point and the high end
/// of its box encloses zero TIGHTLY there.
fn assert_zero_pointwise(what: &str, shape: Shape) {
    // Three points on the box's diagonal, and one OFF it: `None` pins
    // each parameter at its own fraction, drawn from its name, so two
    // parameters of one shape sit at different fractions of their boxes.
    for f in [Some(0.0), Some(0.3719), Some(1.0), None] {
        let pin = |name: &str, lo: f64, hi: f64| {
            let g = f.unwrap_or_else(|| {
                let h = name.bytes().fold(7u64, |h, b| h * 31 + u64::from(b));
                ((h as f64) * 0.618_033_988_75).fract()
            });
            let v = lo + g * (hi - lo);
            over(name, v, v)
        };
        let (value, _) = with_session_rules(budget(), SymRules::shipped(), || shape(&pin).value);
        let (lo, hi) = value
            .enclosure_probe()
            .expect("a certified enclosure at a point");
        assert!(
            lo <= 0.0 && hi >= 0.0 && hi - lo < 1e-9,
            "{what}: the tier SAYS ZERO and the residual at the point {f:?} of the box encloses \
             [{lo:e}, {hi:e}] — UNSOUND"
        );
    }
}

/// One decision under `rules` over the whole box, and the pointwise
/// check behind every zero it claims.
fn decide(what: &str, rules: SymRules, shape: Shape) -> String {
    decide_under(what, budget(), rules, shape)
}

/// [`decide`] under a chosen budget.
fn decide_under(what: &str, budget: SymBudget, rules: SymRules, shape: Shape) -> String {
    let (out, counts) = with_session_rules(budget, rules, || {
        geom_core::k_stats::decide(
            "decide_4_root_quotient_rows",
            Margin::of(shape(&over)),
            band(),
        )
    });
    let l = label(out, counts);
    println!("  {what}: {l}");
    if matches!(l.as_str(), "theorem" | "sign_gated" | "registered") {
        assert_zero_pointwise(what, shape);
    }
    l
}

/// `(on, off)`: the shipped set, and the same without the quotient.
fn both(what: &str, shape: Shape) -> (String, String) {
    (
        decide(&format!("{what} [on]"), SymRules::shipped(), shape),
        decide(
            &format!("{what} [off]"),
            SymRules::without_root_quotient(),
            shape,
        ),
    )
}

/// A fold: a theorem with the dial on, and not with it off.
fn assert_folds(what: &str, shape: Shape) {
    let (on, off) = both(what, shape);
    assert_eq!(on, "theorem", "{what}: the exact quotient takes it");
    assert_ne!(off, "theorem", "{what}: without the quotient it stands");
}

/// A decline: the same answer with the dial on and off, and no theorem.
fn assert_declines(what: &str, shape: Shape) {
    let (on, off) = both(what, shape);
    assert_eq!(on, off, "{what}: not the rewrite's");
    assert_ne!(on, "theorem", "{what}: nothing may be claimed");
}

// ------------------------------------------------------------ 1. folds

/// **The boss's shape.** `h` a chord deviation, `a + h` the chord half,
/// the chord's polynomial to the fourth power in both halves, against
/// `sqrt(5)·|a + h|`. `a` is dyadic so `(a + h)⁶`'s coefficients stay
/// inside the ring — and so the split would NOT decline here: `D = (a +
/// h)⁴` is a perfect square, `manifest::nonneg` proves it, and the
/// split keys the root `sqrt(5)·|p³|/p²`, which does not meet `|p|`.
/// The quotient asked first is what takes it (on the measured boss the
/// split declines on the ring instead, and the quotient takes it too).
#[test]
fn the_bosss_shape_folds() {
    assert_folds("sqrt(5 p^6 / p^4) - sqrt(5)|p|, p = 1/2 + h", |p| {
        let q = lit(0.5) + p("h", -1.0e-4, 1.0e-4);
        (lit(5.0) * q.powi(6) / q.powi(4)).sqrt() - lit(5.0).sqrt() * q.abs()
    });
}

/// One indeterminate, a non-square `Q`: `sqrt((2 + t)(1 + t)/(1 + t))`
/// against `sqrt(2 + t)`.
#[test]
fn a_non_square_quotient_in_one_indeterminate_folds() {
    assert_folds("sqrt((2+t)(1+t)/(1+t)) - sqrt(2+t)", |p| {
        let t = p("t", 0.5, 1.0);
        let d = lit(1.0) + t;
        ((lit(2.0) + t) * d / d).sqrt() - (lit(2.0) + t).sqrt()
    });
}

/// Several indeterminates, a perfect-square `Q`:
/// `sqrt((x + y)²(1 + xy)/(1 + xy)) = |x + y|`.
#[test]
fn a_square_quotient_in_several_indeterminates_folds() {
    assert_folds("sqrt((x+y)^2 (1+xy)/(1+xy)) - |x+y|", |p| {
        let x = p("x", 0.5, 1.0);
        let y = p("y", -0.25, 0.25);
        let d = lit(1.0) + x * y;
        ((x + y).powi(2) * d / d).sqrt() - (x + y).abs()
    });
}

/// Several indeterminates, a non-square `Q`, against the same `Q`
/// spelled without a denominator — one atom per value class.
#[test]
fn a_non_square_quotient_meets_its_own_spelling() {
    assert_folds("sqrt((1+x^2+y)(2+x)/(2+x)) - sqrt(1+x^2+y)", |p| {
        let x = p("x", 0.5, 1.0);
        let y = p("y", 0.1, 0.2);
        let q = lit(1.0) + x.powi(2) + y;
        let d = lit(2.0) + x;
        (q * d / d).sqrt() - q.sqrt()
    });
    assert_folds("sqrt(Q(x-y+3)/(x-y+3)) - sqrt(Q), Q = x^2+xy+3", |p| {
        let x = p("x", 0.5, 1.0);
        let y = p("y", 0.25, 0.75);
        let d = x - y + lit(3.0);
        let q = x * x + x * y + lit(3.0);
        (q * d / d).sqrt() - q.sqrt()
    });
}

/// **The order's tie-break.** `D = y² − x² + 4` and `Q = xy + x² + y³ +
/// 3` tie in total degree between `x`-terms and `y`-terms at every
/// step, so the division cancels leading terms only if the lex part of
/// the order is a monomial order.
#[test]
fn a_division_through_grlex_ties_folds() {
    assert_folds("sqrt(Q(y^2-x^2+4)/(y^2-x^2+4)) - sqrt(Q), ties", |p| {
        let x = p("x", 0.5, 1.0);
        let y = p("y", 0.25, 0.75);
        let d = y * y - x * x + lit(4.0);
        let q = x * y + x * x + y * y * y + lit(3.0);
        (q * d / d).sqrt() - q.sqrt()
    });
}

/// A `Q` with a real ZERO inside the box: `sqrt(t²(2 + t)/(2 + t)) =
/// |t|` over `t ∈ [−0.5, 0.5]`.
#[test]
fn a_quotient_with_a_zero_in_the_box_folds_to_its_magnitude() {
    assert_folds("sqrt(t^2 (2+t)/(2+t)) - |t|, t in [-0.5, 0.5]", |p| {
        let t = p("t", -0.5, 0.5);
        let d = lit(2.0) + t;
        (t.powi(2) * d / d).sqrt() - t.abs()
    });
}

/// A NEGATIVE `D` no form shows negative (`t − 2` over `[0, 1]`): the
/// split cannot prove its sign, the quotient does not need it.
#[test]
fn a_negative_denominator_no_form_signs_folds() {
    assert_folds("sqrt((3+t)(t-2)/(t-2)) - sqrt(3+t)", |p| {
        let t = p("t", 0.0, 1.0);
        let d = t - lit(2.0);
        ((lit(3.0) + t) * d / d).sqrt() - (lit(3.0) + t).sqrt()
    });
}

/// `(1 + x + y + z)^k · (x − y)/(x − y)` against `(1 + x + y + z)^k`,
/// for `k` = 12 and 14: quotients of 455 and 680 terms, both inside the
/// budget's 4096-term cap, which is the division's only step cap.
fn powered(p: Params, k: i32) -> Sym<Interval> {
    let x = p("x", 0.5, 1.0);
    let y = p("y", 0.125, 0.25);
    let z = p("z", 0.25, 0.5);
    let q = (lit(1.0) + x + y + z).powi(k);
    let d = x - y;
    (q * d / d).sqrt() - q.sqrt()
}

#[test]
fn a_quotient_of_hundreds_of_terms_folds_inside_the_budget() {
    assert_folds("(1+x+y+z)^12 (x-y)/(x-y), 455 terms", |p| powered(p, 12));
    assert_folds("(1+x+y+z)^14 (x-y)/(x-y), 680 terms", |p| powered(p, 14));
}

/// **The division's exact remainder is its proof, and nothing checks a
/// product after it.** `N = Σ Q·d_j` over a twelve-term `D` with `Q =
/// (1 + x + y + z)^11` (364 terms): every step multiplies one term by
/// `D`, inside the pair budget, where a final `Q·D` would ask 4368 >
/// 4096 pairs and refuse a quotient the loop had found.
#[test]
fn a_quotient_whose_product_is_past_the_pair_budget_folds() {
    assert_folds("sqrt(sum_j Q d_j / D) - sqrt(Q), |Q| 364, |D| 12", |p| {
        let x = p("x", 0.5, 1.0);
        let y = p("y", 0.125, 0.25);
        let z = p("z", 0.25, 0.5);
        let q = (lit(1.0) + x + y + z).powi(11);
        let terms = [
            lit(1.0),
            x,
            y,
            z,
            x * x,
            y * y,
            z * z,
            x * y,
            y * z,
            z * x,
            x * x * x,
            y * y * y,
        ];
        let mut n = q * terms[0];
        let mut d = terms[0];
        for t in &terms[1..] {
            n = n + q * *t;
            d = d + *t;
        }
        (n / d).sqrt() - q.sqrt()
    });
}

// --------------------------------------------------------- 2. declines

/// `N | D` is not tried.
#[test]
fn n_dividing_d_is_not_the_rewrites() {
    let (on, off) = both("sqrt((1+t)/((1+t)(2+t))) - sqrt(1/(2+t))", |p| {
        let t = p("t", 0.5, 1.0);
        let n = lit(1.0) + t;
        (n / (n * (lit(2.0) + t))).sqrt() - (lit(1.0) / (lit(2.0) + t)).sqrt()
    });
    assert_eq!(on, off, "N | D is not the rewrite's");
}

/// A constant `D` is the split's (content), not the quotient's.
#[test]
fn a_constant_denominator_is_not_the_rewrites() {
    let (on, off) = both("sqrt((1+t)/3) - sqrt(1+t)/sqrt(3)", |p| {
        let t = p("t", 0.5, 1.0);
        ((lit(1.0) + t) / lit(3.0)).sqrt() - (lit(1.0) + t).sqrt() / lit(3.0).sqrt()
    });
    assert_eq!(on, off, "a constant D is not the rewrite's");
}

/// **Not a GCD.** `(1 + t)(2 + t)/((1 + t)(3 + t))` shares `(1 + t)`
/// but neither half divides the other.
#[test]
fn a_shared_factor_neither_half_divides_is_not_taken() {
    assert_declines("sqrt((1+t)(2+t)/((1+t)(3+t))) - sqrt((2+t)/(3+t))", |p| {
        let t = p("t", 0.5, 1.0);
        let q = (lit(1.0) + t) * (lit(2.0) + t) / ((lit(1.0) + t) * (lit(3.0) + t));
        q.sqrt() - ((lit(2.0) + t) / (lit(3.0) + t)).sqrt()
    });
}

/// **Two near misses.** `D` divides `N` except for one tiny term — a
/// `t` term, and a CONSTANT. The constant is the adversary: a division
/// that dropped its remainder would return `(t + 2)²`, whose root IS
/// the other term, and call the residual zero.
#[test]
fn a_near_miss_is_not_an_exact_quotient() {
    assert_declines("sqrt(((t+2)^3 + 2^-80 t)/(t+2)) - |t+2|", |p| {
        let t = p("t", 0.5, 1.0);
        let q = lit(2.0) + t;
        ((q.powi(3) + lit(2.0_f64.powi(-80)) * t) / q).sqrt() - q.abs()
    });
    assert_declines("sqrt(((t+2)^3 + 2^-80)/(t+2)) - |t+2|", |p| {
        let t = p("t", 0.5, 1.0);
        let q = lit(2.0) + t;
        ((q.powi(3) + lit(2.0_f64.powi(-80))) / q).sqrt() - q.abs()
    });
}

/// **A declined division costs nothing the budget allows.** `x³⁴`
/// over the non-divisor `x² − y − z − u − v`, at the drive's 4096-term
/// budget and at 65536: the division's trailing-term condition declines
/// it before a step (`form`'s `a_failed_necessary_condition_declines_before_any_step`
/// pins the zero step count), where a loop that rebuilt its remainder
/// every step spent the whole cap — 0.78 s at 4096 in release, review
/// r1's measurement. The row asserts the decline, not a time.
#[test]
fn a_declined_division_costs_nothing_the_budget_allows() {
    let shape: Shape = |p| {
        let x = p("x", 1.5, 2.0);
        let d = x * x - p("y", 0.1, 0.2) - p("z", 0.1, 0.2) - p("u", 0.1, 0.2) - p("v", 0.1, 0.2);
        let n = x.powi(34);
        (n / d).sqrt() - n.sqrt() / d.sqrt()
    };
    for max_terms in [4096, 65536] {
        let b = SymBudget {
            max_terms,
            max_degree: 128,
        };
        let t0 = std::time::Instant::now();
        let on = decide_under(
            &format!("budget {max_terms} [on]"),
            b,
            SymRules::shipped(),
            shape,
        );
        println!(
            "  budget {max_terms}: {on} in {:.4}s",
            t0.elapsed().as_secs_f64()
        );
        let off = decide_under(
            &format!("budget {max_terms} [off]"),
            b,
            SymRules::without_root_quotient(),
            shape,
        );
        assert_eq!(on, off, "a non-divisor is not the rewrite's");
    }
}

/// **A monomial division that ignores an indeterminate of the divisor
/// would fold a non-quotient**: `x²/y` is not a polynomial, and a
/// division reading `x² ÷ y` as `x²` would answer `sqrt(x²/y) = |x|`.
/// Both orders of the two ids are asked, so whichever id sorts first a
/// divisor-only indeterminate is met.
#[test]
fn a_divisor_indeterminate_the_numerator_lacks_declines() {
    assert_declines("sqrt(x^2/y) - |x|", |p| {
        let x = p("x", 0.5, 1.0);
        let y = p("y", 2.0, 3.0);
        (x.powi(2) / y).sqrt() - x.abs()
    });
    assert_declines("sqrt(y^2/x) - |y|", |p| {
        let x = p("x", 2.0, 3.0);
        let y = p("y", 0.5, 1.0);
        (y.powi(2) / x).sqrt() - y.abs()
    });
}

/// **A division that meets the step cap declines; it never answers the
/// quotient it had so far.** Under a 3-term budget `(x⁴ − 1)/(x − 1)`
/// needs 4 quotient terms; the first three are `x³ + x² + x`, and a
/// division that returned them would call `sqrt(N/D) − sqrt(x³ + x² +
/// x)` zero, which it is not (the pointwise check reds that).
#[test]
fn a_division_at_the_step_cap_answers_no_partial_quotient() {
    let small = SymBudget {
        max_terms: 3,
        max_degree: 128,
    };
    let l = decide_under(
        "sqrt((x^4-1)/(x-1)) - sqrt(x^3+x^2+x), 3-term budget",
        small,
        SymRules::shipped(),
        |p| {
            let x = p("x", 1.5, 2.0);
            ((x.powi(4) - lit(1.0)) / (x - lit(1.0))).sqrt() - (x.powi(3) + x.powi(2) + x).sqrt()
        },
    );
    assert_ne!(l, "theorem", "the partial quotient is not the quotient");
}

// ------------------------------------------------ 3. the negative rows

/// **A sign-carrying square's quotient root is a magnitude, never the
/// signed root.** `sqrt(R²·D/D)` against `R` is NOT a theorem, and the
/// TIER was asked — the value channel cannot classify it — on each box:
/// where `R > 0` throughout (true there, but only a read of the box
/// could say so), where `R` changes sign, and for the product `R =
/// t(t − 0.25)` on three boxes. Against `|R|` it is a theorem on every
/// one.
#[test]
fn a_sign_carrying_quotient_root_stays_a_magnitude() {
    let rows: [(&str, Shape, Shape); 5] = [
        (
            "R = t, t in [0.5, 1]",
            |p| {
                let t = p("t", 0.5, 1.0);
                let d = lit(1.0) + t;
                (t.powi(2) * d.powi(2) / d.powi(2)).sqrt() - t
            },
            |p| {
                let t = p("t", 0.5, 1.0);
                let d = lit(1.0) + t;
                (t.powi(2) * d.powi(2) / d.powi(2)).sqrt() - t.abs()
            },
        ),
        (
            "R = t, t in [-0.5, 0.5]",
            |p| {
                let t = p("t", -0.5, 0.5);
                let d = lit(2.0) + t;
                (t.powi(2) * d / d).sqrt() - t
            },
            |p| {
                let t = p("t", -0.5, 0.5);
                let d = lit(2.0) + t;
                (t.powi(2) * d / d).sqrt() - t.abs()
            },
        ),
        (
            "R = t(t - 0.25), t in [0.5, 1]",
            |p| r_product(p, 0.5, 1.0, false),
            |p| r_product(p, 0.5, 1.0, true),
        ),
        (
            "R = t(t - 0.25), t in [-1, -0.5]",
            |p| r_product(p, -1.0, -0.5, false),
            |p| r_product(p, -1.0, -0.5, true),
        ),
        (
            "R = t(t - 0.25), t in [-0.5, 0.5]",
            |p| r_product(p, -0.5, 0.5, false),
            |p| r_product(p, -0.5, 0.5, true),
        ),
    ];
    for (what, signed, magnitude) in rows {
        let l = decide(&format!("{what}: against R"), SymRules::shipped(), signed);
        assert_eq!(
            l, "refused",
            "{what}: |R| = R is a SIGN of R, which the rewrite reads nowhere; the tier is asked \
             and declines"
        );
        let l = decide(
            &format!("{what}: against |R|"),
            SymRules::shipped(),
            magnitude,
        );
        assert_eq!(l, "theorem", "{what}: the quotient's root is |R|");
    }
}

/// `sqrt(R²(3 + t)/(3 + t))` against `R` or `|R|`, `R = t(t − 0.25)`.
fn r_product(p: Params, lo: f64, hi: f64, magnitude: bool) -> Sym<Interval> {
    let t = p("t", lo, hi);
    let r = t * (t - lit(0.25));
    let d = lit(3.0) + t;
    let root = (r.powi(2) * d / d).sqrt();
    if magnitude { root - r.abs() } else { root - r }
}

// ------------------------------------------------------- 4. the trade

/// **The meeting the rule gives up — pinned as the trade.** The split
/// spelling of `sqrt(N/D)`, `sqrt(N)/sqrt(D)`, and the product
/// `sqrt(N/D)·sqrt(D)` against `sqrt(N)`, with `N = Q·D` and `D`
/// manifestly positive. With the dial off the split keys both sides
/// alike and each is a theorem; with it on `sqrt(N/D)` is `sqrt(Q)`,
/// and `sqrt(N)` for `N = Q·D` is its own atom, so each is refused.
/// Meeting both spellings needs a factorisation of `N`. A change that
/// makes these meet reds here, on purpose.
#[test]
fn the_split_spelling_is_what_the_quotient_trades() {
    let rows: [(&str, Shape); 2] = [
        ("sqrt(N/D) - sqrt(N)/sqrt(D), D = 1+x^2, Q = 2+x", |p| {
            let x = p("x", 1.0, 2.0);
            let d = lit(1.0) + x * x;
            let n = (lit(2.0) + x) * d;
            (n / d).sqrt() - n.sqrt() / d.sqrt()
        }),
        ("sqrt(QD/D) sqrt(D) - sqrt(QD), Q = 1+x^2, D = 1+y^2", |p| {
            let x = p("x", 0.5, 1.0);
            let y = p("y", 0.5, 1.0);
            let q = lit(1.0) + x.powi(2);
            let d = lit(1.0) + y.powi(2);
            let n = q * d;
            (n / d).sqrt() * d.sqrt() - n.sqrt()
        }),
    ];
    for (what, shape) in rows {
        let (on, off) = both(what, shape);
        assert_eq!(off, "theorem", "{what}: the split meets it");
        assert_eq!(
            on, "refused",
            "{what}: the quotient re-keys the root away from it"
        );
    }
}

/// **The meeting it gains in exchange**: `sqrt(Q·D/D)` against
/// `sqrt(Q)` where the split proves `D`'s sign (`D = 1 + x²`) — the
/// split's `sqrt(Q·D)/sqrt(D)` never met it.
#[test]
fn the_quotient_spelling_with_a_signed_denominator_meets() {
    assert_folds("sqrt(Q(1+x^2)/(1+x^2)) - sqrt(Q), Q = 2+x", |p| {
        let x = p("x", 1.0, 2.0);
        let d = lit(1.0) + x * x;
        let q = lit(2.0) + x;
        (q * d / d).sqrt() - q.sqrt()
    });
}

/// **Rule G's conjunction.** `root_quotient` on with `canonical_root`
/// off is inert: the step is reached only through rule G's door.
#[test]
fn the_quotient_without_rule_g_is_inert() {
    let shape: Shape = |p| {
        let t = p("t", 0.5, 1.0);
        let d = lit(1.0) + t;
        ((lit(2.0) + t) * d / d).sqrt() - (lit(2.0) + t).sqrt()
    };
    let lone = decide(
        "canonical_root off, root_quotient on",
        SymRules {
            root_quotient: true,
            ..SymRules::without_canonical_root()
        },
        shape,
    );
    let shut = decide(
        "without_canonical_root",
        SymRules::without_canonical_root(),
        shape,
    );
    assert_eq!(lone, shut);
    assert_ne!(lone, "theorem");
}
