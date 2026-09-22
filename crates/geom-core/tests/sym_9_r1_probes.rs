//! **Reviewer r1's probes for SYM-9's retry ladder** — decisions built
//! at the scalar door that the FIRST attempt refuses, driven through
//! `sym::with_session_retry` at each shape, with the receipt read.
//!
//! Every margin here is identically zero as a real, so a `theorem` is
//! a true claim and a `numeric` is the tier declining; the rows read
//! `SymCounts`, not the answer, because the numeric channel answers
//! `Zero` inside the band either way.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::predicate::{Band, Margin, Sign};
use geom_core::sym::{with_session_retry, with_session_rules};
use geom_core::{ParamSymbol, Real, Sym, SymBudget, SymCounts, SymRetry, SymRules};

fn budget() -> SymBudget {
    SymBudget {
        max_terms: 4096,
        max_degree: 128,
    }
}

fn band() -> Band {
    Band::new(1.0e-9, 1.0e-8).unwrap()
}

fn p(name: &str, v: f64) -> Sym<f64> {
    Sym::param(ParamSymbol::of(name), v)
}

fn label(out: Result<Sign, geom_core::predicate::Indeterminate>, c: &SymCounts) -> String {
    match out {
        Ok(Sign::Zero) if c.symbolic_zero == 1 => "theorem".to_owned(),
        Ok(Sign::Zero) if c.registered == 1 => "registered".to_owned(),
        Ok(Sign::Zero) if c.sign_gated == 1 => "sign_gated".to_owned(),
        Ok(s) => format!("numeric {s:?}"),
        Err(e) => format!("refused {:?}", e.margin),
    }
}

/// One decision through the ladder `retry`, answering `(label, counts)`.
fn through(retry: SymRetry, build: impl FnOnce() -> Sym<f64>) -> (String, SymCounts) {
    let (out, counts) = with_session_retry(budget(), SymRules::shipped(), retry, || {
        geom_core::k_stats::decide("sym_9_r1", Margin::of(build()), band())
    });
    (label(out, &counts), counts)
}

/// `c^n` as a left-associated chain, and as `(c^a)·(c^(n-a))` — two
/// spellings of ONE constant whose partial products differ, so the
/// ring refuses them at different nodes and the two freeze into
/// different indeterminates.
fn two_spellings(n: usize, a: usize) -> Sym<f64> {
    let c = || Sym::<f64>::from_f64(1.0 / 3.0);
    let chain = |k: usize| (1..k).fold(c(), |acc, _| acc * c());
    chain(n) - chain(a) * chain(n - a)
}

/// **The ring retry, end to end**: `(1/3)^5` is a 263-bit odd
/// mantissa, so the shipped ring (256) refuses the products and both
/// spellings freeze; a 512-bit retry builds them and the difference is
/// the zero polynomial. The identity of reals is the same at either
/// width — the integers are exact — so the retry's zero is a theorem.
#[test]
fn r1_a_product_past_256_that_fits_at_512_is_closed_by_the_ring_retry() {
    let (none, c0) = through(SymRetry::none(), || two_spellings(5, 2));
    println!("no ladder      : {none} {c0:?}");
    assert_eq!(c0.numeric, 1, "the shipped ring refuses 263 bits");
    assert_eq!(c0.retried, 0);

    let r512 = SymRetry {
        bits: Some(512),
        ..SymRetry::none()
    };
    let (l512, c512) = through(r512, || two_spellings(5, 2));
    println!("ring 512       : {l512} {c512:?}");
    assert_eq!(l512, "theorem", "512 bits builds both products");
    assert_eq!(c512.symbolic_zero, 1);
    assert_eq!(c512.retried, 1, "`retried` says the ladder carried it");
    assert_eq!(c512.numeric, 0);

    // The SHIPPED ladder has no ring attempt at all.
    let (shipped, cs) = through(SymRetry::kept_atom(), || two_spellings(5, 2));
    println!("shipped ladder : {shipped} {cs:?}");
    assert_eq!(cs.numeric, 1, "`kept_atom` offers no wider ring");
    assert_eq!(cs.retried, 0);
}

/// **And one that fits at neither.** `(1/3)^11` is 577 bits: past 256
/// and past 512, so the ring retry is offered and declines and the
/// decision stays numeric with `retried` at zero — the spec's negative
/// row, built independently.
#[test]
fn r1_a_product_past_512_stays_numeric_under_a_512_bit_ladder() {
    let r512 = SymRetry {
        bits: Some(512),
        without: SymRetry::kept_atom().without,
    };
    let (l, c) = through(r512, || two_spellings(11, 5));
    println!("ring 512 + kept atom on 577 bits: {l} {c:?}");
    assert_eq!(c.numeric, 1, "577 bits is past 512 too");
    assert_eq!(c.symbolic_zero, 0);
    assert_eq!(c.retried, 0, "a refused ladder carries nothing");

    let r1024 = SymRetry {
        bits: Some(1024),
        ..SymRetry::none()
    };
    let (l, c) = through(r1024, || two_spellings(11, 5));
    println!("ring 1024 on 577 bits          : {l} {c:?}");
    assert_eq!(l, "theorem", "1024 bits reaches it");
    assert_eq!(c.retried, 1);
}

/// **The widened bound does not leak to the NEXT decision.** Two
/// decisions in one session, each past the shipped ring and inside
/// 512: if `with_coeff_bound`'s guard left the ring wide, the second
/// decision's FIRST attempt would close it and `retried` would read 1
/// instead of 2.
#[test]
fn r1_a_retrys_wider_ring_does_not_leak_into_the_next_decision() {
    let retry = SymRetry {
        bits: Some(512),
        ..SymRetry::none()
    };
    let (_, counts) = with_session_retry(budget(), SymRules::shipped(), retry, || {
        let a = geom_core::k_stats::decide("sym_9_r1", Margin::of(two_spellings(5, 2)), band());
        let b = geom_core::k_stats::decide("sym_9_r1", Margin::of(two_spellings(6, 2)), band());
        (a, b)
    });
    println!("two decisions under a 512 ladder: {counts:?}");
    assert_eq!(counts.symbolic_zero, 2, "both close on their retry");
    assert_eq!(
        counts.retried, 2,
        "each paid its own retry — a bound left wide would have closed the second \
         on its FIRST attempt and left `retried` at 1"
    );
}

/// **And not into the next SESSION**: the same margin, decided in a
/// session with no ladder after one with a 1024-bit ladder, is numeric
/// again.
#[test]
fn r1_a_retrys_wider_ring_does_not_leak_into_the_next_session() {
    let r1024 = SymRetry {
        bits: Some(1024),
        ..SymRetry::none()
    };
    let (_, wide) = through(r1024, || two_spellings(11, 5));
    assert_eq!(wide.symbolic_zero, 1);
    let (after, c) = through(SymRetry::none(), || two_spellings(11, 5));
    println!("after a 1024 session, with no ladder: {after} {c:?}");
    assert_eq!(c.numeric, 1, "the session's ring is COEFF_BITS again");
}

/// **A retry's form never serves the FIRST attempt** — the separation
/// `Session::retries` is for. The SAME margin is decided twice in one
/// session under a 512-bit ladder: the node ids are hash-consed, so
/// the second decision asks the same nodes the first one's retry
/// built. If a retry's forms went into `forms_early`/`forms_door`, the
/// second decision would close on its FIRST attempt and `retried`
/// would read 1 while `symbolic_zero` read 2.
#[test]
fn r1_a_retrys_form_never_serves_the_first_attempt() {
    let retry = SymRetry {
        bits: Some(512),
        ..SymRetry::none()
    };
    let (_, counts) = with_session_retry(budget(), SymRules::shipped(), retry, || {
        let a = geom_core::k_stats::decide("sym_9_r1", Margin::of(two_spellings(5, 2)), band());
        let b = geom_core::k_stats::decide("sym_9_r1", Margin::of(two_spellings(5, 2)), band());
        (a, b)
    });
    println!("the same margin twice under a 512 ladder: {counts:?}");
    assert_eq!(counts.symbolic_zero, 2, "both close");
    assert_eq!(
        counts.retried, 2,
        "each closed on its OWN retry — a retry form served to the first attempt would \
         have closed the second there and left `retried` at 1"
    );
}

/// **A fewer-rules retry closes a MEASURED rule loss.** The fixture is
/// `sym_rule_e_rows::rule_e_can_cost_a_theorem_to_the_coefficient_ring`
/// — a residual `without_rule_e` reaches and `shipped` does not,
/// because rule E's scale step pushes the product past the ring. As a
/// RETRY it is recovered: the first attempt is the shipped tier and
/// nothing moves it, and the second attempt runs the same algebra with
/// one rule taken away.
fn rule_e_residual() -> Sym<f64> {
    let l33 = 5_559_060_566_555_523.0_f64; // 3^33, exact in f64
    let l27 = 7_625_597_484_987.0_f64; // 3^27
    let x = p("x", 1.5);
    let y = p("y", 1.5);
    let one = Sym::from_f64(1.0);
    let q = Sym::from_f64(l33) * Sym::from_f64(l33) * Sym::from_f64(l33) * Sym::from_f64(l27);
    let inv_q = one / q;
    let c = Sym::from_f64(0.1);
    let pf = (x + inv_q.abs()) / (x + c);
    let h = y + (one / Sym::from_f64(1000.0)).abs();
    (pf.sqrt().powi(2) * h).sqrt() - (pf * h).sqrt()
}

#[test]
fn r1_a_fewer_rules_retry_closes_the_rule_e_loss() {
    let (base, c0) = through(SymRetry::none(), rule_e_residual);
    println!("no ladder          : {base} {c0:?}");
    assert_ne!(
        base, "theorem",
        "rule E costs this theorem (SYM-5 R2's row)"
    );

    let mask = SymRules {
        common_factor: false,
        ..SymRules::all()
    };
    let retry = SymRetry {
        bits: None,
        without: [Some(mask), None],
    };
    let (l, c) = through(retry, rule_e_residual);
    println!("rule E shut, retry : {l} {c:?}");
    assert_eq!(l, "theorem", "the fewer-rules attempt reaches it");
    assert_eq!(c.symbolic_zero, 1);
    assert_eq!(c.retried, 1);

    // **The SHIPPED ladder does not reach it.** `SymRetry::kept_atom`
    // carries rule G and rule A only, so a loss demonstrated at the
    // scalar and not paid by any measured document stays numeric.
    let (l, c) = through(SymRetry::kept_atom(), rule_e_residual);
    println!("shipped ladder     : {l} {c:?}");
    assert_ne!(l, "theorem");
    assert_eq!(c.retried, 0);
}

/// **A plain theorem is never re-asked, at any ladder.** The receipt
/// is bit-identical at `SymRetry::none()` and at every shape, and
/// `retried` is zero throughout — the "first attempt is identical with
/// the ladder and without it" claim, read off the counts.
#[test]
fn r1_a_plain_theorem_is_the_same_receipt_at_every_shape() {
    let margin = || {
        let x = p("w", 0.37);
        x + Sym::from_f64(2.0) * x - Sym::from_f64(3.0) * x
    };
    let (_, base) = with_session_rules(budget(), SymRules::shipped(), || {
        geom_core::k_stats::decide("sym_9_r1", Margin::of(margin()), band())
    });
    for (name, retry) in [
        ("none", SymRetry::none()),
        ("kept_atom", SymRetry::kept_atom()),
        (
            "ring_512",
            SymRetry {
                bits: Some(512),
                ..SymRetry::kept_atom()
            },
        ),
    ] {
        let (l, c) = through(retry, margin);
        println!("{name:<10}: {l} {c:?}");
        assert_eq!(l, "theorem");
        assert_eq!(
            (c.symbolic_zero, c.sign_gated, c.registered, c.numeric),
            (
                base.symbolic_zero,
                base.sign_gated,
                base.registered,
                base.numeric
            ),
            "{name}: the ladder moved the first attempt's receipt"
        );
        assert_eq!(c.retried, 0, "{name}: the ladder was never entered");
        assert_eq!(c.frozen, base.frozen, "{name}: the plain walk's work moved");
    }
}

/// **A mask can only take rules away**, and the shipped ladder's own
/// masks: rule G shut, then rule A shut, applied to the SHIPPED set.
#[test]
fn r1_the_shipped_ladders_masks_are_the_shipped_set_minus_one_rule() {
    let s = SymRules::shipped();
    let [g, a] = SymRetry::kept_atom().without;
    let g = s.masked_by(g.unwrap());
    let a = s.masked_by(a.unwrap());
    println!("shipped   : {s:?}");
    println!("G shut    : {g:?}");
    println!("A shut    : {a:?}");
    assert!(!g.canonical_root, "the first attempt shuts rule G");
    assert!(!a.sqrt_square, "the second shuts rule A");
    // Every OTHER dial is the session's.
    assert_eq!(
        SymRules {
            canonical_root: true,
            ..g
        },
        s,
        "the rule-G mask took away more than rule G"
    );
    assert_eq!(
        SymRules {
            sqrt_square: true,
            ..a
        },
        s,
        "the rule-A mask took away more than rule A"
    );
}
