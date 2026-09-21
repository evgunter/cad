//! DECIDE-3 review probes (r2): rule G's canonical root and the decision
//! read driven through `Sym<Interval>` at the scalar door, on forms the
//! unit did not measure. Every row prints what the tier answered and
//! asserts only soundness (a theorem must be numerically zero) unless a
//! row says otherwise.
#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::predicate::{Band, Margin, Sign};
use geom_core::sym::{DriveMemo, with_session_rules};
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

fn over(name: &str, lo: f64, hi: f64) -> Sym<Interval> {
    Sym::param_over(ParamSymbol::of(name), Interval::from_bounds(lo, hi), lo, hi)
}

fn label(out: Result<Sign, geom_core::predicate::Indeterminate>, c: SymCounts) -> String {
    match out {
        Ok(Sign::Zero) if c.symbolic_zero > 0 => "theorem".to_owned(),
        Ok(Sign::Zero) if c.registered > 0 => "registered".to_owned(),
        Ok(Sign::Zero) if c.sign_gated > 0 => "sign_gated".to_owned(),
        Ok(s) => format!("numeric {s:?}"),
        Err(e) => format!("refused {:?}", e.margin),
    }
}

/// One decision under `rules`; the label and the margin's enclosure.
fn how(rules: SymRules, build: impl FnOnce() -> Sym<Interval>) -> (String, Option<(f64, f64)>) {
    let ((out, value), counts) = with_session_rules(budget(), rules, || {
        let m = build();
        (
            geom_core::k_stats::decide("decide3_r2", Margin::of(m), band()),
            m.value,
        )
    });
    (label(out, counts), value.enclosure_probe())
}

fn row(what: &str, (l, v): (String, Option<(f64, f64)>)) -> String {
    println!("  {what}: {l} enclosure {v:?}");
    if l == "theorem" || l == "sign_gated" {
        let (lo, hi) = v.expect("a certified enclosure");
        assert!(
            lo <= 1e-9 && hi >= -1e-9,
            "{what}: SAYS ZERO but the enclosure is [{lo:e}, {hi:e}] — UNSOUND"
        );
    }
    l
}

/// The rational content: the odd part of a non-square denominator keys
/// `sqrt(1/k)` while the numerator keys `sqrt(k)`; powers of two go
/// through `exp2` and meet.
#[test]
fn r2_content_canonicity_over_the_odd_part() {
    for k in [2.0, 3.0, 8.0, 12.0, 17.0] {
        let l = row(
            &format!("sqrt(x/{k}) - sqrt({k}x)/{k}, x in [1,2]"),
            how(SymRules::shipped(), || {
                let x = over("x", 1.0, 2.0);
                (x / lit(k)).sqrt() - (lit(k) * x).sqrt() / lit(k)
            }),
        );
        println!("    -> {l}");
    }
    for k in [2.0, 3.0, 17.0] {
        row(
            &format!("sqrt(x/{k}) * sqrt({k}) - sqrt(x), x in [1,2]"),
            how(SymRules::shipped(), || {
                let x = over("x", 1.0, 2.0);
                (x / lit(k)).sqrt() * lit(k).sqrt() - x.sqrt()
            }),
        );
        row(
            &format!("sqrt(1/{k}) * sqrt({k}) - 1"),
            how(SymRules::shipped(), || {
                (lit(1.0) / lit(k)).sqrt() * lit(k).sqrt() - lit(1.0)
            }),
        );
    }
}

/// Non-square content, squares, nested roots.
#[test]
fn r2_content_squares_and_nesting() {
    let s = SymRules::shipped();
    row("sqrt(2x²) - sqrt(2)|x|, x in [-1,1]", how(s, || {
        let x = over("x", -1.0, 1.0);
        (lit(2.0) * x.powi(2)).sqrt() - lit(2.0).sqrt() * x.abs()
    }));
    row("sqrt(8) - 2 sqrt(2)", how(s, || lit(8.0).sqrt() - lit(2.0) * lit(2.0).sqrt()));
    row("sqrt(12x) - 2 sqrt(3) sqrt(x), x in [1,2]", how(s, || {
        let x = over("x", 1.0, 2.0);
        (lit(12.0) * x).sqrt() - lit(2.0) * lit(3.0).sqrt() * x.sqrt()
    }));
    row("sqrt(12x) - sqrt(3) sqrt(4x), x in [1,2]", how(s, || {
        let x = over("x", 1.0, 2.0);
        (lit(12.0) * x).sqrt() - lit(3.0).sqrt() * (lit(4.0) * x).sqrt()
    }));
    row("sqrt(-2x) - sqrt(2) sqrt(-x), x in [-2,-1]", how(s, || {
        let x = over("x", -2.0, -1.0);
        (lit(-2.0) * x).sqrt() - lit(2.0).sqrt() * (-x).sqrt()
    }));
    row("sqrt(x²) - |x|, x in [-1,1]", how(s, || {
        let x = over("x", -1.0, 1.0);
        x.powi(2).sqrt() - x.abs()
    }));
    let l = row("sqrt(x²) - x, x in [-1,1] (must NOT be a theorem)", how(s, || {
        let x = over("x", -1.0, 1.0);
        x.powi(2).sqrt() - x
    }));
    assert_ne!(l, "theorem");
    let l = row("sqrt(x²) - x, x in [1,2] (not manifest: not a theorem)", how(s, || {
        let x = over("x", 1.0, 2.0);
        x.powi(2).sqrt() - x
    }));
    assert_ne!(l, "theorem");
    row("sqrt(|x|²) - |x|, x in [-1,1]", how(s, || {
        let x = over("x", -1.0, 1.0);
        x.abs().powi(2).sqrt() - x.abs()
    }));
    row("sqrt(sqrt(x)²) - sqrt(x), x in [1,2]", how(s, || {
        let x = over("x", 1.0, 2.0);
        x.sqrt().powi(2).sqrt() - x.sqrt()
    }));
    row("sqrt(sqrt(x)) - sqrt(sqrt(x)) [same node], x in [1,2]", how(s, || {
        let x = over("x", 1.0, 2.0);
        let r = x.sqrt().sqrt();
        r - r
    }));
    row("sqrt(x²y) - |x| sqrt(y), x in [-1,1], y in [1,2]", how(s, || {
        let x = over("x", -1.0, 1.0);
        let y = over("y", 1.0, 2.0);
        (x.powi(2) * y).sqrt() - x.abs() * y.sqrt()
    }));
    row("sqrt((1-2x)²) - |1-2x|, x in [-1,1]", how(s, || {
        let x = over("x", -1.0, 1.0);
        let r = lit(1.0) - lit(2.0) * x;
        r.powi(2).sqrt() - r.abs()
    }));
    row("sqrt((1-2x)²) - |2x-1|, x in [-1,1]", how(s, || {
        let x = over("x", -1.0, 1.0);
        let r = lit(1.0) - lit(2.0) * x;
        r.powi(2).sqrt() - (lit(2.0) * x - lit(1.0)).abs()
    }));
    row("|x|² - x², x in [-1,1] (rule A on Abs)", how(s, || {
        let x = over("x", -1.0, 1.0);
        x.abs().powi(2) - x.powi(2)
    }));
    row("sqrt(x/y) sqrt(y/x) - 1, x,y in [-2,-1]", how(s, || {
        let x = over("x", -2.0, -1.0);
        let y = over("y", -2.0, -1.0);
        (x / y).sqrt() * (y / x).sqrt() - lit(1.0)
    }));
    row("sqrt(x/(1+y²)) - sqrt(x)/sqrt(1+y²), x in [1,2], y in [-1,1]", how(s, || {
        let x = over("x", 1.0, 2.0);
        let y = over("y", -1.0, 1.0);
        let d = lit(1.0) + y.powi(2);
        (x / d).sqrt() - x.sqrt() / d.sqrt()
    }));
}

/// Source 3 depends on minting ORDER: the walk expands the RIGHT child
/// first, so `sqrt(P²/x) · sqrt(x)` mints `sqrt(x)` before the quotient
/// and splits, while `sqrt(x) · sqrt(P²/x)` mints the quotient first
/// and keeps it opaque.
#[test]
fn r2_source_3_is_order_dependent() {
    let s = SymRules::shipped();
    let a = row("sqrt(P²/x) * sqrt(x) - |P|, P = 1+2x, x in [1,2]", how(s, || {
        let x = over("x", 1.0, 2.0);
        let p = lit(1.0) + lit(2.0) * x;
        (p.powi(2) / x).sqrt() * x.sqrt() - p.abs()
    }));
    let b = row("sqrt(x) * sqrt(P²/x) - |P|, P = 1+2x, x in [1,2]", how(s, || {
        let x = over("x", 1.0, 2.0);
        let p = lit(1.0) + lit(2.0) * x;
        x.sqrt() * (p.powi(2) / x).sqrt() - p.abs()
    }));
    let c = row("sqrt(1/x) * sqrt(x) - 1, x in [1,2]", how(s, || {
        let x = over("x", 1.0, 2.0);
        (lit(1.0) / x).sqrt() * x.sqrt() - lit(1.0)
    }));
    let d = row("sqrt(x) * sqrt(1/x) - 1, x in [1,2]", how(s, || {
        let x = over("x", 1.0, 2.0);
        x.sqrt() * (lit(1.0) / x).sqrt() - lit(1.0)
    }));
    println!("  orders: {a} / {b} / {c} / {d}");
    // With a denominator whose CONTENT is not one, the plain walk's
    // atom (`sqrt(2x)`, keyed on `2x`) is not the primitive's
    // (`sqrt(x)`), so source 3 sees only what the early walk minted
    // before it — and the early walk expands the right child first.
    let e = row("sqrt(P²/(2x)) * sqrt(2x) - |P|, x in [1,2]", how(s, || {
        let x = over("x", 1.0, 2.0);
        let p = lit(1.0) + lit(2.0) * x;
        (p.powi(2) / (lit(2.0) * x)).sqrt() * (lit(2.0) * x).sqrt() - p.abs()
    }));
    let f = row("sqrt(2x) * sqrt(P²/(2x)) - |P|, x in [1,2]", how(s, || {
        let x = over("x", 1.0, 2.0);
        let p = lit(1.0) + lit(2.0) * x;
        (lit(2.0) * x).sqrt() * (p.powi(2) / (lit(2.0) * x)).sqrt() - p.abs()
    }));
    println!("  orders with content 2: {e} / {f}");
    // With rule G off neither order reaches it.
    row("[G off] sqrt(P²/x) * sqrt(x) - |P|", how(SymRules::without_canonical_root(), || {
        let x = over("x", 1.0, 2.0);
        let p = lit(1.0) + lit(2.0) * x;
        (p.powi(2) / x).sqrt() * x.sqrt() - p.abs()
    }));
}

/// The adversary against the side condition's source 3: `x < 0` on the
/// whole box, `sqrt(x)` minted on a DEAD arm (or by an earlier decision
/// of the same session), and `sqrt(N/x)·sqrt(M/x)·x` — whose real value
/// is `−sqrt(NM)` — spelled as the product of two `Sqrt` atoms, which
/// rule F's magnitude door then calls non-negative.
fn adversary(x: Sym<Interval>) -> Sym<Interval> {
    let n = lit(1.0) + x.powi(3);
    let m = lit(1.0) + x.powi(5);
    let y = (n / x).sqrt() * (m / x).sqrt() * x;
    y.copysign(lit(1.0)) - y
}

#[test]
fn r2_source_3_adversary_dead_arm() {
    let s = SymRules::shipped();
    // Control: no `sqrt(x)` node anywhere — the split declines.
    let control = row("control: copysign(Y,1) - Y, x in [-1.5,-1.1]", how(s, || {
        adversary(over("x", -1.5, -1.1))
    }));
    println!("    control -> {control}");
    // The residual times a factor that straddles zero, so the numeric
    // channel cannot decide and the tier is asked: a false theorem here
    // comes back as `Ok(Zero)` with no debug assertion in the way.
    let straddle = |rules: SymRules, plant: bool| {
        how(rules, || {
            let x = over("x", -1.5, -1.1);
            let r = adversary(x) * (x + lit(1.3));
            if plant {
                lit(-1.0).select_le_zero(r, x.sqrt())
            } else {
                r
            }
        })
    };
    let (l0, v0) = straddle(s, false);
    println!("  [no plant]  (copysign(Y,1) - Y)(x + 1.3): {l0} {v0:?}");
    let (l1, v1) = straddle(s, true);
    println!("  [dead arm]  select(-1, (copysign(Y,1) - Y)(x + 1.3), sqrt(x)): {l1} {v1:?}");
    let (l2, v2) = straddle(SymRules::without_canonical_root(), true);
    println!("  [dead arm, G off]: {l2} {v2:?}");
    // The definite version under a caught panic: the debug assertion in
    // `sign_within` is what fires when the form says zero and the
    // enclosure says positive.
    let caught = std::panic::catch_unwind(|| {
        how(s, || {
            let x = over("x", -1.5, -1.1);
            lit(-1.0).select_le_zero(adversary(x), x.sqrt())
        })
    });
    match caught {
        Ok((l, v)) => println!("  [dead arm, definite] {l} {v:?}"),
        Err(e) => {
            let msg = e
                .downcast_ref::<String>()
                .cloned()
                .or_else(|| e.downcast_ref::<&str>().map(|s| (*s).to_owned()))
                .unwrap_or_default();
            println!("  [dead arm, definite] PANIC: {msg}");
        }
    }
    println!("  SUMMARY dead-arm: no-plant={l0} planted={l1} G-off={l2}");
}

#[test]
fn r2_source_3_adversary_earlier_decision_in_the_session() {
    let s = SymRules::shipped();
    let ((first, second, v), _counts) = with_session_rules(budget(), s, || {
        let x = over("x", -1.5, -1.1);
        // An earlier decision of this session mints `sqrt(x)` — and is
        // refused by clause 1 (no real value), as a document that goes
        // on to be Invalid would.
        let first = geom_core::k_stats::decide("decide3_r2_first", Margin::of(x.sqrt() - x.sqrt()), band());
        let r = adversary(x) * (x + lit(1.3));
        let second = geom_core::k_stats::decide("decide3_r2_second", Margin::of(r), band());
        (format!("{first:?}"), format!("{second:?}"), r.value.enclosure_probe())
    });
    println!("  first (sqrt(x) - sqrt(x), x<0): {first}");
    println!("  second ((copysign(Y,1) - Y)(x + 1.3)): {second} enclosure {v:?}");
}

/// The two reads: `≤` at the tie, `sign_gated` on the receipt, never
/// `symbolic_zero`; `max(A, B)` is the same read.
#[test]
fn r2_the_reads_label_and_tie() {
    let s = SymRules::shipped();
    let l = row("select(x-2, y, y+1) - y, x in [1,2] (tie at 2: the value door decides, the read?)", how(s, || {
        let x = over("x", 1.0, 2.0);
        let y = over("y", 3.0, 4.0);
        (x - lit(2.0)).select_le_zero(y, y + lit(1.0)) - y
    }));
    println!("    tie at select -> {l}");
    let l = row("select(x-2, y, y+1) - y, x in [1,1.5]", how(s, || {
        let x = over("x", 1.0, 1.5);
        let y = over("y", 3.0, 4.0);
        (x - lit(2.0)).select_le_zero(y, y + lit(1.0)) - y
    }));
    assert_eq!(l, "sign_gated");
    let l = row("select(2-x, y, y+1) - (y+1), x in [1,1.5]", how(s, || {
        let x = over("x", 1.0, 1.5);
        let y = over("y", 3.0, 4.0);
        (lit(2.0) - x).select_le_zero(y, y + lit(1.0)) - (y + lit(1.0))
    }));
    assert_eq!(l, "sign_gated");
    let l = row("max(x, 3) - 3, x in [1,2]", how(s, || {
        let x = over("x", 1.0, 2.0);
        x.max(lit(3.0)) - lit(3.0)
    }));
    assert_eq!(l, "sign_gated");
    let l = row("min(x, 3) - x, x in [1,2]", how(s, || {
        let x = over("x", 1.0, 2.0);
        x.min(lit(3.0)) - x
    }));
    assert_eq!(l, "sign_gated");
    let l = row("max(x, 2) - 2, x in [1,2] (tie at 2)", how(s, || {
        let x = over("x", 1.0, 2.0);
        x.max(lit(2.0)) - lit(2.0)
    }));
    println!("    tie at max -> {l}");
    // Dressed: `sqrt(1+y²)·(x − 3)` ≤ 0 — the stripping and the deep
    // enclosure.
    let l = row("select(sqrt(1+y²)(x-3), a, b) - a, x in [1,2]", how(s, || {
        let x = over("x", 1.0, 2.0);
        let y = over("y", -1.0, 1.0);
        let a = over("a", 3.0, 4.0);
        let d = (lit(1.0) + y.powi(2)).sqrt() * (x - lit(3.0));
        d.select_le_zero(a, a + lit(1.0)) - a
    }));
    assert_eq!(l, "sign_gated");
    // A comparison A0 settles stays a theorem with the read on.
    let l = row("select(-1, a, b) - a", how(s, || {
        let a = over("a", 3.0, 4.0);
        lit(-1.0).select_le_zero(a, a + lit(1.0)) - a
    }));
    assert_eq!(l, "theorem");
    // The read shut: numeric.
    let l = row("[reads off] max(x, 3) - 3", how(SymRules::without_the_reads(), || {
        let x = over("x", 1.0, 2.0);
        x.max(lit(3.0)) - lit(3.0)
    }));
    assert_ne!(l, "sign_gated");
    // A straddling decision is not read.
    let l = row("select(x-1.5, a, a) - a, x in [1,2] (straddles; equal arms)", how(s, || {
        let x = over("x", 1.0, 2.0);
        let a = over("a", 3.0, 4.0);
        (x - lit(1.5)).select_le_zero(a, a) - a
    }));
    println!("    equal arms straddling -> {l}");
}

/// The drive memo's key carries both new dials.
#[test]
fn r2_memo_key_carries_the_new_dials() {
    let m = DriveMemo::new(budget(), SymRules::shipped());
    assert!(m.accepts(budget(), SymRules::shipped()));
    assert!(!m.accepts(budget(), SymRules::without_the_reads()));
    assert!(!m.accepts(budget(), SymRules::without_canonical_root()));
}
