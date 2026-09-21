//! **Rule G's rows** — the canonical square root and the two certified
//! reads, driven through `Sym<Interval>` at the scalar door, where a
//! form is written out and the tier's answer is read off one decision.
//!
//! Three things are held here, each because a measurement found it:
//!
//! 1. **The side condition is PROVED, never inferred from the session's
//!    atom table.** The dead-arm adversary below is what closed the
//!    first cut's source 3 (`sym/root.rs`'s header carries the
//!    argument): a `Sqrt` atom on an arm the value channel never reads
//!    licensed a split over a NEGATIVE denominator, and rule F then
//!    called the product of two `Sqrt` atoms non-negative — a false
//!    theorem, on a construction made of such arms.
//! 2. **Equal reals key ONE atom, over every normalisation the key is a
//!    function of**: the content's odd part, the denominator's sign,
//!    the root's sign. One row per normalisation, and one row for the
//!    limitation that is declared rather than fixed (a square FACTOR
//!    inside a non-square primitive).
//! 3. **Uniformity per mint site**: the walk, rule D's hand-built roots
//!    and the registered-identity door key the same argument the same
//!    way, which is what "one door" means when it is a claim and not a
//!    wish.

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

/// How the tier answered, as one word: the discharge kind where it
/// discharged, the sign where the numeric channel answered, and the
/// refusal otherwise.
fn label(out: Result<Sign, geom_core::predicate::Indeterminate>, c: SymCounts) -> String {
    match out {
        Ok(Sign::Zero) if c.symbolic_zero > 0 => "theorem".to_owned(),
        Ok(Sign::Zero) if c.registered > 0 => "registered".to_owned(),
        Ok(Sign::Zero) if c.sign_gated > 0 => "sign_gated".to_owned(),
        Ok(s) => format!("numeric {s:?}"),
        Err(e) => format!("refused {:?}", e.margin),
    }
}

/// One decision under `rules`: the label, and the value channel's
/// enclosure of the same residual.
fn how(rules: SymRules, build: impl FnOnce() -> Sym<Interval>) -> (String, Option<(f64, f64)>) {
    let ((out, value), counts) = with_session_rules(budget(), rules, || {
        let m = build();
        (
            geom_core::k_stats::decide("sym_root_rows", Margin::of(m), band()),
            m.value,
        )
    });
    (label(out, counts), value.enclosure_probe())
}

/// **Every row is a soundness row first.** A form the tier calls zero
/// must be one the value channel's enclosure admits as zero; a row that
/// says otherwise is the tier lying, whatever else the row is about.
fn row(what: &str, (l, v): (String, Option<(f64, f64)>)) -> String {
    println!("  {what}: {l} enclosure {v:?}");
    if l == "theorem" || l == "sign_gated" || l == "registered" {
        let (lo, hi) = v.expect("a certified enclosure");
        assert!(
            lo <= 1e-9 && hi >= -1e-9,
            "{what}: the tier SAYS ZERO and the value channel encloses [{lo:e}, {hi:e}] — UNSOUND"
        );
    }
    l
}

// ------------------------------------------------------------------
// 1. The side condition is proved, not inferred
// ------------------------------------------------------------------

/// `sqrt(N/x)·sqrt(M/x)·x` over a box where `x < 0` throughout. Its
/// real value is `−sqrt(NM)`, so `copysign(Y, 1) − Y` is `2·sqrt(NM)`
/// and zero nowhere on the box — but spelled as a PRODUCT OF TWO `Sqrt`
/// ATOMS it is what `manifest::nonneg` reads as non-negative, and
/// `copysign` then folds to `Y` itself.
fn adversary(x: Sym<Interval>) -> Sym<Interval> {
    let n = lit(1.0) + x.powi(3);
    let m = lit(1.0) + x.powi(5);
    let y = (n / x).sqrt() * (m / x).sqrt() * x;
    y.copysign(lit(1.0)) - y
}

/// **The dead-arm adversary: the split may not be licensed by an atom
/// the session happens to hold.** The decision door records BOTH arms
/// of every `Select`, so a `sqrt(x)` on the arm the door does not take
/// is in the session's atom table while contributing nothing to the
/// residual's value. If that atom licensed `sqrt(N/x) → sqrt(N)/sqrt(x)`
/// the tier would prove a residual zero that is bounded away from zero
/// over the whole box.
///
/// The row is GATING: the planted and unplanted forms must answer the
/// same way, and neither may be a theorem.
#[test]
fn a_dead_arm_root_licenses_no_split() {
    let s = SymRules::shipped();
    let control = row(
        "control (no sqrt(x) node anywhere): copysign(Y,1) - Y, x in [-1.5,-1.1]",
        how(s, || adversary(over("x", -1.5, -1.1))),
    );
    // Times a factor that straddles zero, so the numeric channel cannot
    // answer and the tier is the only thing standing between the form
    // and a wrong verdict.
    let straddle = |plant: bool| {
        how(s, || {
            let x = over("x", -1.5, -1.1);
            let r = adversary(x) * (x + lit(1.3));
            if plant {
                lit(-1.0).select_le_zero(r, x.sqrt())
            } else {
                r
            }
        })
    };
    let bare = row("(copysign(Y,1) - Y)(x + 1.3), no plant", straddle(false));
    let planted = row(
        "select(-1, (copysign(Y,1) - Y)(x + 1.3), sqrt(x)) — the dead arm carries sqrt(x)",
        straddle(true),
    );
    assert_ne!(
        planted, "theorem",
        "a root on the arm the door never takes may not license a split: {planted}"
    );
    assert_eq!(
        bare, planted,
        "the dead arm changes nothing: {bare} against {planted}"
    );
    println!("  control {control} | bare {bare} | planted {planted}");
}

/// The same adversary with the root minted by an EARLIER decision of
/// the same session rather than by a dead arm — the session's atom
/// table outlives one decision, so this is the same defect with a
/// different carrier. The definite form is asked here (no straddling
/// factor), so a false theorem would trip the two-channel debug
/// assertion in `Sym::sign_within` rather than come back quietly; the
/// row asserts that no panic and no zero is reached.
#[test]
fn an_earlier_decisions_root_licenses_no_split() {
    let caught = std::panic::catch_unwind(|| {
        with_session_rules(budget(), SymRules::shipped(), || {
            let x = over("x", -1.5, -1.1);
            // Refused by clause 1 — `sqrt(x)` has no real value here —
            // exactly as a document that goes on to be Invalid would.
            let first = geom_core::k_stats::decide(
                "sym_root_rows_first",
                Margin::of(x.sqrt() - x.sqrt()),
                band(),
            );
            let second = geom_core::k_stats::decide(
                "sym_root_rows_second",
                Margin::of(adversary(x)),
                band(),
            );
            (format!("{first:?}"), format!("{second:?}"))
        })
    });
    match caught {
        Ok(((first, second), counts)) => {
            println!("  first (sqrt(x) - sqrt(x), x < 0): {first}");
            println!("  second (copysign(Y,1) - Y): {second} {counts:?}");
            assert!(
                !second.starts_with("Ok(Zero)"),
                "the residual is 2*sqrt(NM), zero nowhere on this box: {second}"
            );
        }
        Err(_) => panic!("the two channels contradicted each other"),
    }
}

// ------------------------------------------------------------------
// 2. Equal reals key ONE atom
// ------------------------------------------------------------------

/// **The content's odd part.** `sqrt(x/k)` and `sqrt(k·x)/k` are the
/// same real for every positive rational `k`; the key is the primitive
/// polynomial times a constant `sqrt(f)` atom, so `f` must be a
/// function of the VALUE class and not of which side of the fraction
/// the odd part arrived on.
#[test]
fn the_content_is_canonical_over_its_odd_part() {
    for k in [2.0, 3.0, 8.0, 12.0, 17.0] {
        let l = row(
            &format!("sqrt(x/{k}) - sqrt({k}x)/{k}, x in [1,2]"),
            how(SymRules::shipped(), || {
                let x = over("x", 1.0, 2.0);
                (x / lit(k)).sqrt() - (lit(k) * x).sqrt() / lit(k)
            }),
        );
        assert_eq!(l, "theorem", "k = {k}");
        let l = row(
            &format!("sqrt(1/{k}) * sqrt({k}) - 1"),
            how(SymRules::shipped(), || {
                (lit(1.0) / lit(k)).sqrt() * lit(k).sqrt() - lit(1.0)
            }),
        );
        assert_eq!(l, "theorem", "k = {k}");
    }
}

/// **The denominator's sign.** Rule E leaves the sign on the
/// denominator, so `x/(−1−y²)` and `(−x)/(1+y²)` are two spellings of
/// one real; the side condition's non-positive source is what makes
/// them one atom.
#[test]
fn a_negative_denominator_keys_the_same_atom() {
    let l = row(
        "sqrt(x/(-1-y²)) - sqrt(-x/(1+y²)), x in [-2,-1], y in [-1,1]",
        how(SymRules::shipped(), || {
            let x = over("x", -2.0, -1.0);
            let y = over("y", -1.0, 1.0);
            (x / (lit(-1.0) - y.powi(2))).sqrt() - ((-x) / (lit(1.0) + y.powi(2))).sqrt()
        }),
    );
    assert_eq!(l, "theorem");
}

/// **The root's sign.** `sqrt(R²) = |R|` is minted on a SIGN-NORMALISED
/// `R`, so `|1 − 2x|` and `|2x − 1|` are one `Abs` atom and the root
/// meets either spelling of the node.
#[test]
fn the_root_of_a_square_is_sign_normalised() {
    for (name, build) in [("sqrt((1-2x)²) - |1-2x|", 0), ("sqrt((1-2x)²) - |2x-1|", 1)] {
        let l = row(
            &format!("{name}, x in [0,1]"),
            how(SymRules::shipped(), || {
                let x = over("x", 0.0, 1.0);
                let r = lit(1.0) - lit(2.0) * x;
                let s = r.powi(2).sqrt();
                if build == 0 {
                    s - r.abs()
                } else {
                    s - (-r).abs()
                }
            }),
        );
        assert_eq!(l, "theorem", "{name}");
    }
}

/// **The declared limitation**: a square FACTOR inside a primitive that
/// is not itself a square is left under the root. `sqrt(x²·y)` keys on
/// `x²·y`, not on `|x|·sqrt(y)`, so the two do not meet. Splitting the
/// polynomial's square factors would need a multivariate squarefree
/// decomposition at every mint; the rule declines instead, and this row
/// is what says so out loud rather than leaving a reader to find it.
#[test]
fn a_square_factor_under_a_non_square_root_is_not_split() {
    let l = row(
        "sqrt(x²y) - |x| sqrt(y), x in [-1,1], y in [1,2] — DECLARED LIMITATION",
        how(SymRules::shipped(), || {
            let x = over("x", -1.0, 1.0);
            let y = over("y", 1.0, 2.0);
            (x.powi(2) * y).sqrt() - x.abs() * y.sqrt()
        }),
    );
    assert_ne!(
        l, "theorem",
        "if this reads `theorem` the limitation is gone — delete the row"
    );
}

// ------------------------------------------------------------------
// 3. One door: the mint sites agree
// ------------------------------------------------------------------

/// **The walk's root and rule D's hand-built root are one atom.** Rule
/// D builds `S = sqrt(1 + X²)` by hand for `sin`/`cos` of an `atan`;
/// a `sqrt(1 + x·x)` the walk mints over the same `x` has to be the
/// same indeterminate, or the two spellings of one arc never cancel.
#[test]
fn the_walk_and_rule_ds_roots_are_one_atom() {
    let l = row(
        "cos(atan x)·sqrt(1 + x·x) - 1, x in [0.2, 0.8]",
        how(SymRules::shipped(), || {
            let x = over("x", 0.2, 0.8);
            x.atan().cos() * (lit(1.0) + x * x).sqrt() - lit(1.0)
        }),
    );
    assert_eq!(l, "theorem");
    let l = row(
        "sin(atan x)·sqrt(1 + x·x) - x, x in [0.2, 0.8]",
        how(SymRules::shipped(), || {
            let x = over("x", 0.2, 0.8);
            x.atan().sin() * (lit(1.0) + x * x).sqrt() - x
        }),
    );
    assert_eq!(l, "theorem");
}

/// **The registry's forms and the walk's are one atom.** A registrant
/// states `sqrt(4x) = 2·sqrt(x)`; the walk's own root over the same
/// argument has to meet it, and under rule G the residual is a THEOREM
/// before the axiom is needed at all — which is the strongest form of
/// "they meet".
#[test]
fn the_registrys_forms_and_the_walks_are_one_atom() {
    let ((out, value), counts) = with_session_rules(budget(), SymRules::shipped(), || {
        let x = over("x", 1.0, 2.0);
        let left = (lit(4.0) * x).sqrt();
        let right = lit(2.0) * x.sqrt();
        let _ = left.register_equal(right, Tol::witness());
        let m = left - right;
        (
            geom_core::k_stats::decide("sym_root_rows", Margin::of(m), band()),
            m.value,
        )
    });
    let l = label(out, counts);
    println!(
        "  sqrt(4x) - 2 sqrt(x) with a registrant: {l} {:?}",
        value.enclosure_probe()
    );
    assert_eq!(
        l, "theorem",
        "the two spellings meet in the WALK; the registration is not needed"
    );
    assert_eq!(counts.registered, 0, "and the door is not what closed it");
}

/// **The identity the tilted document's candidate norms rest on**: a
/// root over a quotient meeting the root of its own denominator, with
/// a perfect square above and either order of the product. The
/// denominator here is manifestly positive — `1 + x²` — which is what
/// the side condition needs and all it needs.
#[test]
fn a_root_over_a_quotient_meets_the_root_of_its_denominator() {
    let s = SymRules::shipped();
    for order in [0, 1] {
        let l = row(
            &format!("sqrt(1/(1+x²)) * sqrt(1+x²) - 1, x in [-1,1], order {order}"),
            how(s, || {
                let x = over("x", -1.0, 1.0);
                let d = lit(1.0) + x.powi(2);
                let q = (lit(1.0) / d).sqrt();
                let r = d.sqrt();
                (if order == 0 { q * r } else { r * q }) - lit(1.0)
            }),
        );
        assert_eq!(
            l, "theorem",
            "the key is a function of the argument, not of the order (order {order})"
        );
        let l = row(
            &format!("sqrt(P²/(1+x²)) * sqrt(1+x²) - |P|, P = 1+2x, order {order}"),
            how(s, || {
                let x = over("x", -1.0, 1.0);
                let p = lit(1.0) + lit(2.0) * x;
                let d = lit(1.0) + x.powi(2);
                let q = (p.powi(2) / d).sqrt();
                let r = d.sqrt();
                (if order == 0 { q * r } else { r * q }) - p.abs()
            }),
        );
        assert_eq!(l, "theorem", "order {order}");
    }
    // The candidate norm's shape on the tilted frame: a denominator
    // that is neither term-wise non-negative (the odd power of `t`)
    // nor a perfect square, but which completes the square with room
    // to spare — `16t²/17 + 8t/17 + 1 = (16/17)((t + 1/4)² + 1)`.
    let l = row(
        "sqrt(1/(16t²/17 + 8t/17 + 1)) * sqrt(16t²/17 + 8t/17 + 1) - 1, t in [-1,1]",
        how(s, || {
            let t = over("t", -1.0, 1.0);
            let d = lit(16.0 / 17.0) * t.powi(2) + lit(8.0 / 17.0) * t + lit(1.0);
            (lit(1.0) / d).sqrt() * d.sqrt() - lit(1.0)
        }),
    );
    assert_eq!(
        l, "theorem",
        "the definite-quadratic source is what carries a candidate norm's denominator"
    );
}

/// **A denominator whose sign no FORM settles is not split**, and that
/// is the whole of what dropping the atom-table source cost: `1/x` over
/// a box where `x > 0` is true but not manifest. The honest answers
/// are "refused" under the shipped set and a GATED read under rule C's
/// dial, which is where a value read belongs.
#[test]
fn a_denominator_the_form_does_not_sign_is_not_split() {
    let shipped = row(
        "sqrt(1/x) * sqrt(x) - 1, x in [1,2] — no value-free source",
        how(SymRules::shipped(), || {
            let x = over("x", 1.0, 2.0);
            (lit(1.0) / x).sqrt() * x.sqrt() - lit(1.0)
        }),
    );
    assert_ne!(
        shipped, "theorem",
        "`x > 0` is a fact about the BOX here, and a theorem may not rest on one"
    );
    let read = row(
        "[rule C on] sqrt(1/x) * sqrt(x) - 1, x in [1,2]",
        how(SymRules::all(), || {
            let x = over("x", 1.0, 2.0);
            (lit(1.0) / x).sqrt() * x.sqrt() - lit(1.0)
        }),
    );
    assert_eq!(
        read, "sign_gated",
        "source 4 reads the box, so what it reaches is gated and never a theorem"
    );
}

// ------------------------------------------------------------------
// 4. The reads: what they answer and how they are counted
// ------------------------------------------------------------------

/// A certified decision is `sign_gated`, never a theorem; a decision
/// the FORM settles stays a theorem; with the dial shut neither is
/// reached.
#[test]
fn a_certified_decision_is_gated_and_a_form_settled_one_is_not() {
    let s = SymRules::shipped();
    for (what, l) in [
        (
            "select(x-2, y, y+1) - y, x in [1,1.5]",
            row(
                "select(x-2, y, y+1) - y, x in [1,1.5]",
                how(s, || {
                    let x = over("x", 1.0, 1.5);
                    let y = over("y", 3.0, 4.0);
                    (x - lit(2.0)).select_le_zero(y, y + lit(1.0)) - y
                }),
            ),
        ),
        (
            "max(x, 3) - 3, x in [1,2]",
            row(
                "max(x, 3) - 3, x in [1,2]",
                how(s, || {
                    let x = over("x", 1.0, 2.0);
                    x.max(lit(3.0)) - lit(3.0)
                }),
            ),
        ),
        (
            "select(sqrt(1+y²)(x-3), a, b) - a, x in [1,2]",
            row(
                "select(sqrt(1+y²)(x-3), a, b) - a, x in [1,2]",
                how(s, || {
                    let x = over("x", 1.0, 2.0);
                    let y = over("y", -1.0, 1.0);
                    let a = over("a", 3.0, 4.0);
                    let d = (lit(1.0) + y.powi(2)).sqrt() * (x - lit(3.0));
                    d.select_le_zero(a, a + lit(1.0)) - a
                }),
            ),
        ),
    ] {
        assert_eq!(l, "sign_gated", "{what}");
    }
    let l = row(
        "select(-1, a, b) - a — A0 settles it, so it stays a THEOREM",
        how(s, || {
            let a = over("a", 3.0, 4.0);
            lit(-1.0).select_le_zero(a, a + lit(1.0)) - a
        }),
    );
    assert_eq!(l, "theorem");
    let l = row(
        "[reads off] max(x, 3) - 3, x in [1,2]",
        how(SymRules::without_the_reads(), || {
            let x = over("x", 1.0, 2.0);
            x.max(lit(3.0)) - lit(3.0)
        }),
    );
    assert_ne!(
        l, "sign_gated",
        "the read has a dial and it is the only gate"
    );
}

/// **A comparison of two CONSTANTS is not a read.** `max(1, 1/4)` is an
/// exact rational comparison; whether the tier can answer it may not
/// depend on the box, and answering it as a gated READ would report a
/// theorem-shaped fact as one conditional on a bracket.
/// `work/decide/a0-leaves-max-and-min-of-constants-opaque` is what has
/// to fold it; until it does, the honest answer is `numeric`.
#[test]
fn a_comparison_of_two_constants_is_not_read() {
    let l = row(
        "max(1, 1/4)·y - y, y in [3,4]",
        how(SymRules::shipped(), || {
            let y = over("y", 3.0, 4.0);
            lit(1.0).max(lit(0.25)) * y - y
        }),
    );
    assert_ne!(
        l, "sign_gated",
        "an exact rational comparison reads no value: it is A0's to fold, not the read's to gate"
    );
}

/// The drive memo's key carries both dials by construction.
#[test]
fn the_drive_memo_key_carries_both_new_dials() {
    let m = DriveMemo::new(budget(), SymRules::shipped());
    assert!(m.accepts(budget(), SymRules::shipped()));
    assert!(!m.accepts(budget(), SymRules::without_the_reads()));
    assert!(!m.accepts(budget(), SymRules::without_canonical_root()));
}
