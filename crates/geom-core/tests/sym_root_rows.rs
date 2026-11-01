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
    Sym::param_over(
        ParamSymbol::new(test_utils::symbol_id(name)),
        Interval::from_bounds(lo, hi),
        lo,
        hi,
    )
}

/// How the tier answered, as one word: `proven` where it discharged,
/// the sign where the numeric channel answered, and the refusal
/// otherwise.
fn label(out: Result<Sign, geom_core::predicate::Indeterminate>, c: SymCounts) -> String {
    match out {
        Ok(Sign::Zero) if c.symbolic > 0 => "proven".to_owned(),
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
    if l == "proven" {
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
        planted, "proven",
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
        assert_eq!(l, "proven", "k = {k}");
        let l = row(
            &format!("sqrt(1/{k}) * sqrt({k}) - 1"),
            how(SymRules::shipped(), || {
                (lit(1.0) / lit(k)).sqrt() * lit(k).sqrt() - lit(1.0)
            }),
        );
        assert_eq!(l, "proven", "k = {k}");
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
    assert_eq!(l, "proven");
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
        assert_eq!(l, "proven", "{name}");
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
        l, "proven",
        "if this reads `proven` the limitation is gone — delete the row"
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
    assert_eq!(l, "proven");
    let l = row(
        "sin(atan x)·sqrt(1 + x·x) - x, x in [0.2, 0.8]",
        how(SymRules::shipped(), || {
            let x = over("x", 0.2, 0.8);
            x.atan().sin() * (lit(1.0) + x * x).sqrt() - x
        }),
    );
    assert_eq!(l, "proven");
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
        l, "proven",
        "the two spellings meet in the WALK; the registration is not needed"
    );
    let (shut, _) = how(
        SymRules {
            registered: false,
            ..SymRules::shipped()
        },
        || {
            let x = over("x", 1.0, 2.0);
            (lit(4.0) * x).sqrt() - lit(2.0) * x.sqrt()
        },
    );
    assert_eq!(shut, "proven", "and with the door shut it still meets");
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
            l, "proven",
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
        assert_eq!(l, "proven", "order {order}");
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
        l, "proven",
        "the definite-quadratic source is what carries a candidate norm's denominator"
    );
}

/// **A denominator whose sign no FORM settles is not split**, and that
/// is the whole of what dropping the atom-table source cost: `1/x` over
/// a box where `x > 0` is true but not manifest. A sign of the box is
/// not a decision the construction took, so no dial reaches it.
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
        shipped, "proven",
        "`x > 0` is a fact about the BOX here, and a theorem may not rest on one"
    );
    let all = row(
        "[every rule on] sqrt(1/x) * sqrt(x) - 1, x in [1,2]",
        how(SymRules::all(), || {
            let x = over("x", 1.0, 2.0);
            (lit(1.0) / x).sqrt() * x.sqrt() - lit(1.0)
        }),
    );
    assert_ne!(all, "proven", "nor under every rule the tier has");
}

// ------------------------------------------------------------------
// 4. The reads: what they answer
// ------------------------------------------------------------------

/// A certified decision is proven through the arm it takes; a decision
/// the FORM settles is proven without the read; with the dial shut the
/// certified one is not reached.
#[test]
fn a_certified_decision_is_proven_and_the_read_is_its_dial() {
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
        assert_eq!(l, "proven", "{what}");
    }
    let l = row(
        "select(-1, a, b) - a — A0 settles it",
        how(s, || {
            let a = over("a", 3.0, 4.0);
            lit(-1.0).select_le_zero(a, a + lit(1.0)) - a
        }),
    );
    assert_eq!(l, "proven");
    let l = row(
        "[reads off] max(x, 3) - 3, x in [1,2]",
        how(SymRules::without_the_reads(), || {
            let x = over("x", 1.0, 2.0);
            x.max(lit(3.0)) - lit(3.0)
        }),
    );
    assert_ne!(
        l, "proven",
        "the read has a dial, and shut it reaches nothing"
    );
}

/// **The order read picks the arm its DIFFERENCE signs, and a
/// difference the budget refuses picks none.** `min(x, y)` with `x`
/// below `y` over the box is `x` there, and `max(x, y)` is `y`: a read
/// that signed `b − a` in place of `a − b` would take the other arm and
/// leave each residual below numeric. Under a four-term budget the
/// difference of `(x+y+z)/(u+1)` and `(v+w)/(t+1)` is refused (its
/// cross product has six terms) while both kids fit; the node is then
/// an opaque atom and the residual `min(P, Q) − P` stays numeric — a
/// read that fell back to either argument would call it zero, on a box
/// where `Q < P` at some points.
#[test]
fn the_order_read_signs_a_minus_b_and_a_refused_difference_picks_no_arm() {
    let s = SymRules::shipped();
    for (what, build) in [
        (
            "min(x, y) - x, x in [1,2], y in [3,4]",
            (|| {
                let (x, y) = (over("x", 1.0, 2.0), over("y", 3.0, 4.0));
                x.min(y) - x
            }) as fn() -> Sym<Interval>,
        ),
        ("max(x, y) - y, x in [1,2], y in [3,4]", || {
            let (x, y) = (over("x", 1.0, 2.0), over("y", 3.0, 4.0));
            x.max(y) - y
        }),
    ] {
        assert_eq!(row(what, how(s, build)), "proven", "{what}");
    }
    let tiny = SymBudget {
        max_terms: 4,
        max_degree: 128,
    };
    fn kids() -> (Sym<Interval>, Sym<Interval>) {
        let p = (over("x", 1.0, 2.0) + over("y", 1.0, 2.0) + over("z", 1.0, 2.0))
            / (over("u", 0.0, 1.0) + lit(1.0));
        let q = (over("v", 1.0, 2.0) + over("w", 1.0, 2.0)) / (over("t", 0.0, 1.0) + lit(1.0));
        (p, q)
    }
    for (what, build) in [
        (
            "[4 terms] min(P, Q) - P",
            (|| {
                let (p, q) = kids();
                p.min(q) - p
            }) as fn() -> Sym<Interval>,
        ),
        ("[4 terms] min(P, Q) - Q", || {
            let (p, q) = kids();
            p.min(q) - q
        }),
    ] {
        let ((out, value), counts) = with_session_rules(tiny, s, || {
            let m = build();
            (
                geom_core::k_stats::decide("sym_root_rows", Margin::of(m), band()),
                m.value,
            )
        });
        let l = row(what, (label(out, counts), value.enclosure_probe()));
        assert!(
            l.starts_with("numeric") || l.starts_with("refused"),
            "{what}: the difference is refused, so the node is an atom and nothing discharges: {l}"
        );
    }
}

/// **A comparison of two CONSTANTS is A0's.** `max(1, 1/4)` is
/// arithmetic on the coefficient ring: the tier's ability to answer it
/// may not depend on the leaf's box. A0 folds it; with A0 and the read
/// both shut nothing claims it.
#[test]
fn a_comparison_of_two_constants_is_a0s() {
    let l = row(
        "max(1, 1/4)·y - y, y in [3,4]",
        how(SymRules::shipped(), || {
            let y = over("y", 3.0, 4.0);
            lit(1.0).max(lit(0.25)) * y - y
        }),
    );
    assert_eq!(l, "proven", "an exact rational comparison");
    let l = row(
        "min(1, 1/4) - 1/4",
        how(SymRules::shipped(), || lit(1.0).min(lit(0.25)) - lit(0.25)),
    );
    assert_eq!(l, "proven");
    let l = row(
        "[A0 and the read off] max(1, 1/4)·y - y, y in [3,4]",
        how(
            SymRules {
                const_fold: false,
                decision_read: false,
                ..SymRules::shipped()
            },
            || {
                let y = over("y", 3.0, 4.0);
                lit(1.0).max(lit(0.25)) * y - y
            },
        ),
    );
    assert_ne!(l, "proven", "A0 and the read are the dials that decide it");
}

/// `Z = sqrt(x)² − x`: zero under rule A in the early walk, and only
/// there — the plain form keeps the root.
fn z(x: Sym<Interval>) -> Sym<Interval> {
    x.sqrt().powi(2) - x
}

/// **A zero through the read's arm is a zero**, in a product and under
/// a `copysign` as at the node itself. `dihedral_wedge`'s margin is
/// `sin θ · arm`; at a tangent join `sin θ` is the zero form under the
/// early walk's rules, and the lever arm is a `min` the read settles.
/// With the read shut a zero that rests on the arm is not reached.
#[test]
fn a_zero_factor_times_a_read_factor() {
    /// `(shape, its build, proven with the read on, with it shut)`.
    type Shape = (&'static str, fn() -> Sym<Interval>, bool, bool);
    let shapes: [Shape; 6] = [
        (
            "Z · min(x, 3)",
            || {
                let x = over("x", 1.0, 2.0);
                z(x) * x.min(lit(3.0))
            },
            true,
            true,
        ),
        (
            "copysign(Z, min(x, 3) - 3/2)",
            || {
                let x = over("x", 1.0, 2.0);
                z(x).copysign(x.min(lit(3.0)) - lit(1.5))
            },
            true,
            true,
        ),
        (
            "Z · (min(x, 3) - x)",
            || {
                let x = over("x", 1.0, 2.0);
                z(x) * (x.min(lit(3.0)) - x)
            },
            true,
            true,
        ),
        (
            "(min(x, 3) - x) · y",
            || {
                let x = over("x", 1.0, 2.0);
                let y = over("y", 3.0, 4.0);
                (x.min(lit(3.0)) - x) * y
            },
            true,
            false,
        ),
        (
            "min(x, 3) - x",
            || {
                let x = over("x", 1.0, 2.0);
                x.min(lit(3.0)) - x
            },
            true,
            false,
        ),
        (
            "copysign(min(x, 3) - x, y)",
            || {
                let x = over("x", 1.0, 2.0);
                let y = over("y", 3.0, 4.0);
                (x.min(lit(3.0)) - x).copysign(y)
            },
            true,
            false,
        ),
    ];
    for (what, build, on, off) in shapes {
        let got = (
            row(what, how(SymRules::shipped(), build)) == "proven",
            row(what, how(SymRules::without_the_reads(), build)) == "proven",
        );
        assert_eq!(got, (on, off), "{what}: (read on, read shut) proven");
    }
}

/// The drive memo's key carries both dials by construction.
#[test]
fn the_drive_memo_key_carries_both_new_dials() {
    let m = DriveMemo::new(budget(), SymRules::shipped());
    assert!(m.accepts(budget(), SymRules::shipped()));
    assert!(!m.accepts(budget(), SymRules::without_the_reads()));
    assert!(!m.accepts(budget(), SymRules::without_canonical_root()));
}

// ------------------------------------------------------------------
// 5. The read at its node, under a cancellation above it
// ------------------------------------------------------------------

/// `(read on, read shut)`: whether the tier proves the shape zero with
/// the decision read on and with it shut.
fn proven_both(what: &str, build: fn() -> Sym<Interval>) -> (bool, bool) {
    (
        row(what, how(SymRules::shipped(), build)) == "proven",
        row(
            &format!("[read shut] {what}"),
            how(SymRules::without_the_reads(), build),
        ) == "proven",
    )
}

/// **The read answers at its node, and a zero is a zero.** `x + Z` and
/// `x` are two nodes with one early form. With the read shut a `min`,
/// `max` or `Select` over each is ONE atom and their difference
/// cancels; with it on each node is read at itself and the same zero
/// arrives through the two arms. Either way the decision is proven. One
/// shape per placement of the cancellation (S1–S8, S10, S11 of
/// `work/decide/the-read-at-its-node-relabels-a-cancellation-above-it`):
/// at the node's parent, through a `Select`, one level up, under an
/// atom, nested, and through a product. S8 is the atom whose argument
/// the read moved: an atom is keyed by its argument's polynomial, so
/// `atan` of the read-on `x` is `atan(x)`.
#[test]
fn a_cancellation_above_the_read_is_proven_with_the_read_on_and_shut() {
    type Shape = (&'static str, fn() -> Sym<Interval>);
    let shapes: [Shape; 10] = [
        ("S1 max(x + Z, 3) - max(x, 3)", || {
            let x = over("x", 1.0, 2.0);
            (x + z(x)).max(lit(3.0)) - x.max(lit(3.0))
        }),
        ("S2 min(x + Z, 3) - min(x, 3)", || {
            let x = over("x", 1.0, 2.0);
            (x + z(x)).min(lit(3.0)) - x.min(lit(3.0))
        }),
        ("S3 max(x + Z, y) - max(x, y)", || {
            let (x, y) = (over("x", 1.0, 2.0), over("y", 3.0, 4.0));
            (x + z(x)).max(y) - x.max(y)
        }),
        (
            "S4 select(x - 3 + Z, y, y + 1) - select(x - 3, y, y + 1)",
            || {
                let (x, y) = (over("x", 1.0, 2.0), over("y", 3.0, 4.0));
                (x - lit(3.0) + z(x)).select_le_zero(y, y + lit(1.0))
                    - (x - lit(3.0)).select_le_zero(y, y + lit(1.0))
            },
        ),
        ("S5 (max(x + Z, 3) + 1) - (max(x, 3) + 1)", || {
            let x = over("x", 1.0, 2.0);
            ((x + z(x)).max(lit(3.0)) + lit(1.0)) - (x.max(lit(3.0)) + lit(1.0))
        }),
        ("S6 atan(max(x + Z, 3)) - atan(max(x, 3))", || {
            let x = over("x", 1.0, 2.0);
            (x + z(x)).max(lit(3.0)).atan() - x.max(lit(3.0)).atan()
        }),
        ("S7 max(max(x + Z, 3), y) - max(max(x, 3), y)", || {
            let (x, y) = (over("x", 1.0, 2.0), over("y", 3.0, 4.0));
            (x + z(x)).max(lit(3.0)).max(y) - x.max(lit(3.0)).max(y)
        }),
        ("S8 atan(max(x + Z, 3) - max(x, 3) + x) - atan(x)", || {
            let x = over("x", 1.0, 2.0);
            ((x + z(x)).max(lit(3.0)) - x.max(lit(3.0)) + x).atan() - x.atan()
        }),
        (
            "S10 atan(select(x - 3 + Z, y, y + 1)) - atan(select(x - 3, y, y + 1))",
            || {
                let (x, y) = (over("x", 1.0, 2.0), over("y", 3.0, 4.0));
                (x - lit(3.0) + z(x)).select_le_zero(y, y + lit(1.0)).atan()
                    - (x - lit(3.0)).select_le_zero(y, y + lit(1.0)).atan()
            },
        ),
        ("S11 (max(x + Z, 3) - max(x, 3)) · y", || {
            let (x, y) = (over("x", 1.0, 2.0), over("y", 3.0, 4.0));
            ((x + z(x)).max(lit(3.0)) - x.max(lit(3.0))) * y
        }),
    ];
    for (what, build) in shapes {
        assert_eq!(
            proven_both(what, build),
            (true, true),
            "{what}: (read on, read shut) proven"
        );
    }
}

/// **What only the read answers** (C1–C7): comparisons no form settles.
/// Each is proven with the read on and not with it shut, which is what
/// the read buys. C6 is the atom over an argument the read settled: its
/// key is the argument's polynomial, the constant `3`, so it is the
/// atom `atan(3)` the other side mints.
#[test]
fn what_only_the_read_answers() {
    type Shape = (&'static str, fn() -> Sym<Interval>);
    let shapes: [Shape; 7] = [
        ("C1 max(x, 3) - 3", || {
            let x = over("x", 1.0, 2.0);
            x.max(lit(3.0)) - lit(3.0)
        }),
        ("C2 max(x + Z, 3) - 3", || {
            let x = over("x", 1.0, 2.0);
            (x + z(x)).max(lit(3.0)) - lit(3.0)
        }),
        ("C3 max(x + Z, 3) - max(x, 3) + max(x, 3) - 3", || {
            let x = over("x", 1.0, 2.0);
            (x + z(x)).max(lit(3.0)) - x.max(lit(3.0)) + x.max(lit(3.0)) - lit(3.0)
        }),
        ("C4 min(x, y) - x", || {
            let (x, y) = (over("x", 1.0, 2.0), over("y", 3.0, 4.0));
            x.min(y) - x
        }),
        ("C5 max(select(x - 3, y, y + 1), 5) - 5", || {
            let (x, y) = (over("x", 1.0, 2.0), over("y", 3.0, 4.0));
            (x - lit(3.0)).select_le_zero(y, y + lit(1.0)).max(lit(5.0)) - lit(5.0)
        }),
        ("C6 atan(max(x, 3)) - atan(3)", || {
            let x = over("x", 1.0, 2.0);
            x.max(lit(3.0)).atan() - lit(3.0).atan()
        }),
        ("C7 max(x, 3)·y - 3·y", || {
            let (x, y) = (over("x", 1.0, 2.0), over("y", 3.0, 4.0));
            x.max(lit(3.0)) * y - lit(3.0) * y
        }),
    ];
    for (what, build) in shapes {
        assert_eq!(
            proven_both(what, build),
            (true, false),
            "{what}: (read on, read shut) proven"
        );
    }
}

/// **A `max` is a decision and a magnitude is not** (R1's S4). Over
/// `r ∈ [1e-3, 2e-3]`, `|r| − r`, `sqrt(r²) − r` and `max(r, −r) − r`
/// are one real function, zero everywhere on the box. `max(r, −r)` is a
/// comparison the construction TAKES: the read certifies `−r − r < 0`
/// and the arm it takes is the arm the construction takes at every
/// point, so the zero is E12's claim and is proven. `abs` and `sqrt`
/// take no decision: `r ≥ 0` is a sign of the box, and no rule folds on
/// one (M10-8's rule C did, and is retired), so those two are the
/// numeric channel's. The line between them is the construction's own
/// spelling, which is the claim's own line: "where the construction
/// takes the decisions it took". The lib row
/// `a_root_and_a_magnitude_over_a_signed_bracket_are_not_folded` is the
/// same two at a point.
#[test]
fn a_max_is_a_decision_and_a_magnitude_is_not() {
    type Shape = (&'static str, fn() -> Sym<Interval>);
    let shapes: [(Shape, (bool, bool)); 4] = [
        (
            ("max(r, -r) - r", || {
                let r = over("r", 1.0e-3, 2.0e-3);
                r.max(-r) - r
            }),
            (true, false),
        ),
        (
            ("sqrt(r²) - max(r, -r)", || {
                let r = over("r", 1.0e-3, 2.0e-3);
                (r * r).sqrt() - r.max(-r)
            }),
            (false, false),
        ),
        (
            ("|r| - r", || {
                let r = over("r", 1.0e-3, 2.0e-3);
                r.abs() - r
            }),
            (false, false),
        ),
        (
            ("sqrt(r²) - r", || {
                let r = over("r", 1.0e-3, 2.0e-3);
                (r * r).sqrt() - r
            }),
            (false, false),
        ),
    ];
    for ((what, build), want) in shapes {
        assert_eq!(
            proven_both(what, build),
            want,
            "{what}: (read on, read shut) proven"
        );
    }
}

/// **A freeze keyed by its node (S13).** At a four-term budget the
/// read's arm `P = x + y + w` makes `P · Q` six terms, so both products
/// freeze, each to its own node's indeterminate, and the difference is
/// not the zero form. With the read shut both products are `M · Q` over
/// one atom `M` and cancel. The read-on refusal is filed
/// (`work/decide/a-freeze-keyed-by-its-node-hides-a-cancellation-above-the-read`).
#[test]
fn a_cancellation_above_a_frozen_read_is_refused_with_the_read_on() {
    let tiny = SymBudget {
        max_terms: 4,
        max_degree: 128,
    };
    let mut got = Vec::new();
    for (dial, rules) in [
        ("shipped", SymRules::shipped()),
        ("read shut", SymRules::without_the_reads()),
    ] {
        let ((out, value), counts) = with_session_rules(tiny, rules, || {
            let x = over("x", 1.0, 2.0);
            let p = x + over("y", 3.0, 4.0) + over("w", 3.0, 4.0);
            let q = over("u", 1.0, 2.0) + over("v", 1.0, 2.0);
            let m = (x + z(x)).max(p) * q - x.max(p) * q;
            (
                geom_core::k_stats::decide("sym_root_rows", Margin::of(m), band()),
                m.value,
            )
        });
        let what = format!("S13 [4 terms, {dial}] max(x + Z, P)·Q - max(x, P)·Q");
        got.push(row(&what, (label(out, counts), value.enclosure_probe())) == "proven");
    }
    assert_eq!(got, [false, true], "(read on, read shut) proven");
}

/// **The door's instances** (S9, S12): `x·x` registered equal to `x`
/// over `[0.9, 1.1]`.
/// - S9 `max(x·x, 3) − max(x, 3)`: the read settles it at the early
///   rung (`3 − 3` over the box); with the read shut the door does.
/// - S12 `(max(x·x, 3) − max(x, 3)) + (x·x − x)`: zero in the door walk
///   alone, with the read on as with it shut.
/// - `(x·x − x) · min(x, 3)`: the door's zero times a factor the read
///   settles.
#[test]
fn the_doors_zero_beside_the_read_is_proven() {
    type Build = fn(Sym<Interval>, Sym<Interval>) -> Sym<Interval>;
    let cases: [(&str, Build); 3] = [
        ("S9 max(x·x, 3) - max(x, 3)", |sq, x| {
            sq.max(lit(3.0)) - x.max(lit(3.0))
        }),
        ("S12 (max(x·x, 3) - max(x, 3)) + (x·x - x)", |sq, x| {
            (sq.max(lit(3.0)) - x.max(lit(3.0))) + (sq - x)
        }),
        ("(x·x - x) · min(x, 3)", |sq, x| {
            (sq - x) * x.min(lit(3.0))
        }),
    ];
    for (what, build) in cases {
        for (dial, rules) in [
            ("shipped", SymRules::shipped()),
            ("read shut", SymRules::without_the_reads()),
        ] {
            let ((out, value), counts) = with_session_rules(budget(), rules, || {
                let x = over("x", 0.9, 1.1);
                let sq = x * x;
                let reg = sq.register_equal(x, Tol::witness());
                assert!(
                    matches!(reg, geom_core::sym::SymRegistration::Recorded),
                    "the registration is recorded: {reg:?}"
                );
                let m = build(sq, x);
                (
                    geom_core::k_stats::decide("sym_root_rows", Margin::of(m), band()),
                    m.value,
                )
            });
            let what = format!("[x·x = x, {dial}] {what}");
            let l = row(&what, (label(out, counts), value.enclosure_probe()));
            assert_eq!(l, "proven", "{what}: {counts:?}");
        }
    }
}
