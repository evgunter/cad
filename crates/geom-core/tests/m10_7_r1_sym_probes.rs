//! **M10-7 R1 probes over `geom_core::sym`** — independent derivations
//! attacking review claims 2, 3 and 5 at the scalar itself, through the
//! public doors (`Sym`, `with_session`, `k_stats::decide`).
//!
//! Sweep shape ([[test-suite-cost]]): every row is a witness that can be
//! written down, so all are static fixtures and no seed appears. Rows
//! marked EVIDENCE-ONLY print and assert nothing that gates.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::k_stats::decide;
use geom_core::predicate::{Band, Margin, Sign};
use geom_core::sym::with_session;
use geom_core::{ParamSymbol, Real, Sym, SymBudget, Tol};

fn budget() -> SymBudget {
    SymBudget {
        max_terms: 4096,
        max_degree: 128,
    }
}

fn band() -> Band {
    Band::linear(Tol::witness()).expect("linear band")
}

fn p(name: &str, v: f64) -> Sym<f64> {
    Sym::param(ParamSymbol::of(name), v)
}

fn lit(v: f64) -> Sym<f64> {
    Sym::from_f64(v)
}

fn zero(m: Sym<f64>) -> bool {
    decide("r1_probe", Margin::of(m), band()) == Ok(Sign::Zero)
}

/// Runs `f` in a fresh session and answers (decided zero?, symbolic count).
fn sym(f: impl FnOnce() -> Sym<f64>) -> (bool, u64, u64) {
    let (out, counts) = with_session(budget(), || zero(f()));
    (out, counts.symbolic_zero, counts.frozen)
}

// ------------------------------------------------------- claim 2: identities

/// `P + t·d − P` against `t·d`, componentwise, then the norm.
#[test]
fn r1_identity_p_plus_td_minus_p_is_td() {
    let (ok, s, _) = sym(|| {
        let t = p("t", 0.3);
        let d = [p("dx", 1.0), p("dy", -2.0), p("dz", 0.5)];
        let pt = [p("px", 3.0), p("py", 4.0), p("pz", -1.0)];
        let mut acc = lit(0.0);
        for k in 0..3 {
            let lhs = (pt[k] + t * d[k]) - pt[k];
            let rhs = t * d[k];
            let diff = lhs - rhs;
            acc = acc + diff * diff;
        }
        acc.sqrt()
    });
    assert!(ok && s == 1, "P + t·d − P = t·d is a theorem: sym={s}");
}

#[test]
fn r1_identity_a_plus_b_minus_b_minus_a() {
    let (ok, s, _) = sym(|| {
        let (a, b) = (p("a", 0.7), p("b", -3.1));
        (a + b) - b - a
    });
    assert!(ok && s == 1);
}

/// A coincidence: two segments collinear at the nominal only — the
/// perpendicular distance of a third vertex from a line is zero at
/// `w = 0` and NOT identically. Never symbolic, at any width.
#[test]
fn r1_coincidence_collinear_at_nominal_only_is_never_symbolic() {
    for v in [0.0, 1e-13, 0.25] {
        let (ok, s, _) = sym(|| {
            let w = p("w", v);
            // A = (0,0), B = (1,0), C = (2, w): perp_dot(B−A, C−A) = 1·w − 0·2 = w
            let (ax, ay) = (lit(0.0), lit(0.0));
            let (bx, by) = (lit(1.0), lit(0.0));
            let (cx, cy) = (lit(2.0), w);
            (bx - ax) * (cy - ay) - (by - ay) * (cx - ax)
        });
        assert_eq!(s, 0, "a coincidence never decides symbolically (w={v}, zero={ok})");
    }
}

/// A radius equal to a distance at the nominal only: `r − ‖q − c‖` with
/// `r = 1` and `q − c = (1 + w, 0)`.
#[test]
fn r1_coincidence_radius_equals_distance_at_nominal_only() {
    for v in [0.0, 0.1] {
        let (_, s, _) = sym(|| {
            let w = p("w", v);
            let r = lit(1.0);
            let dx = lit(1.0) + w;
            let dy = lit(0.0);
            r - (dx * dx + dy * dy).sqrt()
        });
        assert_eq!(s, 0, "w={v}");
    }
}

/// D4's family, reproduced at the scalar: `c + r·(q−c)/‖q−c‖ − q` with
/// `q = c + (r, 0)` is zero for every `r > 0` and the tier cannot see it
/// (`sqrt(r²) = r` needs the sign of `r`). Pins the miss as a miss.
#[test]
fn r1_the_arc_endpoint_family_is_not_reached_pinned() {
    let (ok, s, _) = sym(|| {
        let r = p("r", 1.25e-3);
        let (cx, cy) = (p("cx", 1.55e-3), lit(0.0));
        let (qx, qy) = (cx + r, cy);
        let (dx, dy) = (qx - cx, qy - cy);
        let n = (dx * dx + dy * dy).sqrt();
        let ex = cx + r * dx / n - qx;
        let ey = cy + r * dy / n - qy;
        (ex * ex + ey * ey).sqrt()
    });
    // Numerically zero at the point; symbolically NOT discharged.
    assert!(ok, "at a point the numeric channel answers Zero");
    assert_eq!(s, 0, "the arc-endpoint family is the tier's disclosed miss (D4)");
}

/// The same family with the norm written as `sqrt(r·r)` where the
/// argument's FORM is exactly `r²`: still an atom, still not `r`.
#[test]
fn r1_sqrt_of_r_squared_is_not_r() {
    let (_, s, _) = sym(|| {
        let r = p("r", 2.0);
        (r * r).sqrt() - r
    });
    assert_eq!(s, 0);
}

// --------------------------------------------- claim 2: the quotient (D1)

/// `p/q` with `q` reaching zero: `(a/b)·b − a` at `b = 0` must not
/// decide symbolically. At `f64` the channel is NaN, which is the
/// domain gate.
#[test]
fn r1_quotient_over_a_zero_denominator_never_certifies() {
    let (ok, s, _) = sym(|| {
        let (a, b) = (p("a", 1.0), p("b", 0.0));
        (a / b) * b - a
    });
    assert!(!ok && s == 0, "ok={ok} sym={s}");
}

/// A denominator whose FORM is the zero polynomial: `1/(x − x)`. The
/// reciprocal of the zero form is refused (a freeze), and the value is
/// NaN — nothing decides.
#[test]
fn r1_reciprocal_of_the_zero_form_freezes_and_never_certifies() {
    let (ok, s, f) = sym(|| {
        let x = p("x", 3.0);
        let inv = lit(1.0) / (x - x);
        inv - inv
    });
    assert!(!ok, "NaN never certifies");
    assert_eq!(s, 0);
    // Clause 1 short-circuits BEFORE the form is asked, so nothing is
    // frozen or counted here — the first cut of this row expected a
    // freeze and was wrong about the order of the two clauses.
    assert_eq!(f, 0, "no form is computed behind a domain refusal");
}

/// EVIDENCE-ONLY. `atan(1/(x−x))` at `f64` is `atan(inf) = π/2`, a
/// finite value with no real behind it; the difference of two such is
/// numerically 0 (not NaN), so clause 1 does not fire, the reciprocal
/// of the zero form freezes, and the two `atan` atoms share the frozen
/// key. Prints whether the tier claims a theorem about it. (At
/// `Interval` the division is empty and clause 1 refuses.)
#[test]
fn r1_atan_of_an_infinity_at_f64() {
    let (ok, s, f) = sym(|| {
        let x = p("x", 3.0);
        let inv = lit(1.0) / (x - x);
        inv.atan() - inv.atan()
    });
    println!("atan(1/0) - atan(1/0) at f64: zero={ok} symbolic={s} frozen={f}");
}

/// Two different fractions that are the same rational function: the
/// quotient form is NOT canonical (`x/2` against `x·½` have different
/// denominators), but subtraction cross-multiplies and the zero test is
/// exact.
#[test]
fn r1_quotient_zero_test_is_exact_across_denominators() {
    let (ok, s, _) = sym(|| {
        let x = p("x", 3.0);
        let y = p("y", 7.0);
        (x / y) - (x * lit(1.0)) / (y * lit(1.0))
    });
    assert!(ok && s == 1, "{ok} {s}");
}

/// `acos 0 = π/2` — is π/2 an exact coefficient on the π indeterminate?
/// `acos(x − x) − π/2` must decide symbolically and `acos(x − x) − π/3`
/// must not.
#[test]
fn r1_acos_of_zero_is_half_pi_exactly() {
    let (ok, s, _) = sym(|| {
        let x = p("x", 0.4);
        (x - x).acos() - <Sym<f64> as Real>::pi() / lit(2.0)
    });
    assert!(ok && s == 1, "acos 0 − π/2: ok={ok} sym={s}");
    let (_, s, _) = sym(|| {
        let x = p("x", 0.4);
        (x - x).acos() - <Sym<f64> as Real>::pi() / lit(3.0)
    });
    assert_eq!(s, 0, "acos 0 − π/3 is not zero");
}

/// `cos 0 = 1`, `sqrt 0 = 0`, `floor 0 = 0`, `abs 0 = 0`.
#[test]
fn r1_atoms_over_the_zero_form_fold_to_their_values() {
    let (ok, s, _) = sym(|| {
        let x = p("x", 0.4);
        let z = x - x;
        z.sqrt() + z.abs() + z.floor() + (z.cos() - lit(1.0)) + z.sin() + z.tan() + z.asin()
            + z.atan()
    });
    assert!(ok && s == 1, "{ok} {s}");
}

/// Atoms keyed by arguments: `min(x, x) − x`, `abs(x) − abs(−x)`,
/// `floor(1) − 1`, `max(x, y) − max(y, x)` — none is reached (the
/// conservative direction); pinned so a future fold is a visible move.
#[test]
fn r1_argument_keyed_atoms_stay_conservative() {
    let cases: [fn() -> Sym<f64>; 5] = [
        || {
            let x = p("x", 0.4);
            x.min(x) - x
        },
        || {
            let x = p("x", 0.4);
            x.abs() - (-x).abs()
        },
        || lit(1.0).floor() - lit(1.0),
        || {
            let (x, y) = (p("x", 0.4), p("y", 0.9));
            x.max(y) - y.max(x)
        },
        || {
            let x = p("x", 0.4);
            x.copysign(lit(1.0)) - x.abs()
        },
    ];
    for (i, f) in cases.into_iter().enumerate() {
        let (_, s, _) = sym(f);
        assert_eq!(s, 0, "case {i} decided symbolically");
    }
}

/// `copysign(0, s)` folds to zero for ANY sign argument — including a
/// poisoned one? `copysign(x − x, sqrt(−1))`: the value is NaN at f64
/// (copysign(0, NaN) = ±0 actually — IEEE keeps the magnitude), so the
/// domain gate does NOT fire while the sign argument has no real value.
/// EVIDENCE-ONLY: prints what happens.
#[test]
fn r1_copysign_of_zero_by_a_poisoned_sign() {
    let (ok, s, _) = sym(|| {
        let x = p("x", 0.4);
        (x - x).copysign(lit(-1.0).sqrt())
    });
    println!("copysign(0, sqrt(-1)): decided zero={ok} symbolic={s}");
}

// ----------------------------------------------- claim 3: freezing (D6)

/// A deliberate i128 overflow: a chain of products of a literal with a
/// 50-bit odd mantissa. `(m)^k` with `m ≈ 2^50` odd overflows the odd
/// part after ~2 multiplications... the coefficient is `m^k`, which
/// exceeds i128 at `k ≥ 3` for a 50-bit odd `m`. The form must freeze
/// (counted) and the identity `c·x − c·x` must STILL cancel because both
/// sides are the same frozen node — while `c·x − x·c` (different node
/// ids, both frozen) must NOT be claimed.
#[test]
fn r1_i128_overflow_freezes_and_is_counted() {
    // 0.1 has a 53-bit odd mantissa (3602879701896397 · 2^-55).
    let (out, counts) = with_session(budget(), || {
        let x = p("x", 0.5);
        let c = lit(0.1) * lit(0.1) * lit(0.1) * lit(0.1);
        let a = c * x;
        let b = x * c;
        (zero(a - a), zero(a - b))
    });
    assert!(counts.frozen >= 1, "the overflow is a counted freeze: {counts:?}");
    assert!(out.0, "same node cancels through a freeze");
    // `c·x − x·c`: `c` is ONE frozen node (one id), so the two products
    // are the same monomial and the form is zero — a frozen atom still
    // takes part in the ring algebra, which is sound (same node, same
    // real). First cut of this row expected 0 here and was wrong.
    assert!(out.1, "a frozen atom commutes like any indeterminate");
    // Two DIFFERENT overflowing constants are two atoms and never cancel.
    let (out3, counts3) = with_session(budget(), || {
        let x = p("x", 0.5);
        let c = lit(0.1) * lit(0.1) * lit(0.1) * lit(0.1);
        let d = lit(0.3) * lit(0.3) * lit(0.3) * lit(0.3);
        // c ≠ d numerically, so f64 says Negative; the tier must agree.
        decide("r1_two_frozen", Margin::of(c * x - d * x), band())
    });
    assert_eq!(out3, Ok(Sign::Negative), "{counts3:?}");
}

/// A large dyadic exponent: `2^1000 · x − 2^1000 · x` cancels (exponent
/// arithmetic is i32), and `2^1000 · x + 2^-1000 · x − (…)` needs a
/// shift of 2000 bits to align, which overflows and freezes.
#[test]
fn r1_dyadic_exponent_alignment_overflow_freezes() {
    let big = 2f64.powi(1000);
    let small = 2f64.powi(-1000);
    let (ok, s, _) = sym(|| {
        let x = p("x", 0.5);
        lit(big) * x - lit(big) * x
    });
    assert!(ok && s == 1);
    let (_, counts) = with_session(budget(), || {
        let x = p("x", 0.5);
        let sum = lit(big) * x + lit(small) * x;
        zero(sum - sum)
    });
    assert!(counts.frozen >= 1, "the alignment overflow freezes: {counts:?}");
}

/// A budget of zero terms: nothing is asked of the DAG.
#[test]
fn r1_zero_budget_is_the_tier_off() {
    let (ok, counts) = with_session(SymBudget::none(), || {
        let x = p("x", 0.5);
        zero(x - x)
    });
    assert!(ok, "f64 answers zero at the point");
    assert_eq!((counts.symbolic_zero, counts.frozen), (0, 0));
}

/// Deep products over budget: `(x+y+z)^40` at degree budget 16 freezes
/// numerically rather than silently — and the difference of the SAME
/// frozen node is still zero (same real).
#[test]
fn r1_degree_budget_freezes_and_same_node_still_cancels() {
    let (out, counts) = with_session(
        SymBudget {
            max_terms: 4096,
            max_degree: 16,
        },
        || {
            let s = p("x", 0.5) + p("y", 0.25) + p("z", 0.125);
            let big = s.powi(40);
            (zero(big - big), zero(big - s.powi(40)))
        },
    );
    assert!(counts.frozen >= 1, "{counts:?}");
    assert!(out.0, "same node");
    // `s.powi(40)` minted twice IS the same content-hashed node, so
    // this also cancels — structurally identical frozen nodes share an id.
    assert!(out.1, "structurally identical frozen nodes share an id");
}

// ---------------------------------------- claim 5 / the reserved id

/// **`Sym::opaque` values share the reserved id, so their DIFFERENCE is
/// the zero form.** `opaque(1.0) − opaque(2.0)` has value −1 (numeric:
/// Negative, definite) and form `indet(0) − indet(0) = 0`. Whichever
/// wins, the doc's claim "every decision on it is the numeric one" is
/// tested here.
#[test]
fn r1_two_opaque_values_are_not_one_indeterminate() {
    let (out, counts) = with_session(budget(), || {
        let a: Sym<f64> = Sym::opaque(1.0);
        let b: Sym<f64> = Sym::opaque(2.0);
        let d = a - b;
        decide("r1_opaque", Margin::of(d), band())
    });
    println!("opaque(1) - opaque(2): {out:?} counts={counts:?}");
    assert_eq!(
        out,
        Ok(Sign::Negative),
        "opaque(1) − opaque(2) is −1; a symbolic Zero here is a false theorem: {counts:?}"
    );
}

/// The same collision reached from a session-less node: a `Sym` built
/// OUTSIDE any session and then combined inside one is frozen keyed by
/// its own (nonzero) id — different values, different ids — so this
/// arm is sound. Pinned beside the failing one to show the hole is
/// the RESERVED id specifically.
#[test]
fn r1_session_less_nodes_do_not_collide() {
    let a = p("a", 1.0);
    let b = p("b", 2.0);
    let (out, _) = with_session(budget(), || decide("r1_outside", Margin::of(a - b), band()));
    assert_eq!(out, Ok(Sign::Negative));
}

/// D9 at the scalar: ids and forms agree across sessions built in
/// different orders and across repeats.
#[test]
fn r1_ids_agree_across_orders_and_repeats() {
    let build = |swap: bool| {
        let (x, y) = if swap {
            (p("y", 2.0), p("x", 1.0))
        } else {
            (p("x", 1.0), p("y", 2.0))
        };
        let (x, y) = if swap { (y, x) } else { (x, y) };
        ((x * y + x) / (y - lit(0.5))).sqrt().node().bits()
    };
    let (a, _) = with_session(budget(), || build(false));
    let (b, _) = with_session(budget(), || build(true));
    let (c, _) = with_session(budget(), || build(false));
    assert!(a == b && b == c);
}

/// EVIDENCE-ONLY: the cost of a 4096-term product. `Poly::mul` builds
/// the whole product before the budget is checked; this prints how long
/// a near-budget square takes.
#[test]
fn r1_near_budget_product_cost() {
    let t0 = std::time::Instant::now();
    let (_, counts) = with_session(budget(), || {
        // (a+b+c+d+e)^12 has C(16,4) = 1820 terms, degree 12.
        let s = p("a", 1.0) + p("b", 1.0) + p("c", 1.0) + p("d", 1.0) + p("e", 1.0);
        let big = s.powi(12);
        let sq = big * big; // 3.3M pairs before the budget refuses it
        zero(sq - sq)
    });
    println!("near-budget square: {:?} counts={counts:?}", t0.elapsed());
}
