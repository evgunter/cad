//! **Rule E's rows at the scalar** — what the quotient's common factor
//! ([`SymRules::common_factor`](geom_core::SymRules)) folds, what it
//! must never fold, and the one thing it can cost.
//!
//! Every row here drives the tier through the same door a document
//! does (`k_stats::decide` over a `Margin`), and every THEOREM is
//! checked against the margin's own value at the point: a claim of
//! "identically zero" that is not numerically zero is the one failure
//! this rule could have and it would be silent otherwise.
//!
//! **Adopted from SYM-5's two review lanes** — `sym/5-review-r1` at
//! `2dfbb8c46` (`sym5_review_r1_probes.rs`: the zero vector, the
//! vanishing shared factor, the straddling box, the non-unit vector,
//! two stacked normalisations) and `sym/5-review-r2` at `37cc12ea9`
//! (`sym5_r2_probes.rs`: the coefficient-width loss, the two
//! spellings). The probe branches are not merged; these are their rows.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::linalg::Vec3;
use geom_core::predicate::{Band, Margin, Sign};
use geom_core::sym::with_session_rules;
use geom_core::{ParamSymbol, Real, Sym, SymBudget, SymRules};

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

fn label(
    out: Result<Sign, geom_core::predicate::Indeterminate>,
    counts: geom_core::SymCounts,
) -> String {
    match out {
        Ok(Sign::Zero) if counts.symbolic_zero == 1 => "theorem".to_owned(),
        Ok(Sign::Zero) if counts.registered == 1 => "registered".to_owned(),
        Ok(Sign::Zero) if counts.sign_gated == 1 => "sign_gated".to_owned(),
        Ok(s) => format!("numeric {s:?}"),
        Err(e) => format!("refused {:?}", e.margin),
    }
}

/// How the tier answered, and the margin's value at the point.
fn how(rules: SymRules, build: impl FnOnce() -> Sym<f64>) -> (String, f64) {
    let ((out, value), counts) = with_session_rules(budget(), rules, || {
        let m = build();
        (
            geom_core::k_stats::decide("sym_rule_e_row", Margin::of(m), band()),
            m.value,
        )
    });
    (label(out, counts), value)
}

/// **The soundness check every row below shares**: a theorem must be
/// numerically zero at the point it was taken at.
fn sound(what: &str, (l, v): (String, f64)) -> String {
    println!("  {what}: {l} value {v:e}");
    assert!(
        l != "theorem" || v.abs() <= 1.0e-12,
        "{what}: SAYS IDENTICALLY ZERO but the value at the point is {v:e} — UNSOUND"
    );
    l
}

fn unit(x: f64, y: f64, z: f64) -> Vec3<Sym<f64>> {
    Vec3::new(p("x", x), p("y", y), p("z", z)).normalize()
}

/// **The theorem row.** A re-normalised unit vector's own norm is the
/// number one — `sqrt(P/P)` folded — and the tier does NOT reach it
/// without rule E, so the row says which rule took it.
#[test]
fn a_re_normalised_unit_vectors_norm_is_one() {
    println!("=== ‖v̂‖ − 1");
    let resid = || unit(3.0, 4.0, 12.0).norm() - Sym::from_f64(1.0);
    assert_eq!(sound("shipped", how(SymRules::shipped(), resid)), "theorem");
    assert_ne!(
        sound("without_rule_e", how(SymRules::without_rule_e(), resid)),
        "theorem",
        "without the rule the tier does not reach it"
    );
}

/// **The negative rows the acceptance names, at the scalar** (R1).
/// A vector that is unit only NUMERICALLY, a re-normalised vector
/// scaled by a parameter whose value is one, the ZERO vector, and a
/// vector whose norm vanishes at the point — none may decide `Zero`
/// through a fold of a falsehood, and the value check above is what
/// would catch it if one did.
#[test]
fn the_shapes_rule_e_must_not_fold() {
    println!("=== shapes rule E must not fold, shipped set");
    let s = SymRules::shipped();

    // Unit only numerically: ‖(1, e, 0)‖ is sqrt(1 + e²), not 1.
    assert_ne!(
        sound(
            "‖(1,e,0)‖ − 1 at e = 1e-30",
            how(s, || {
                Vec3::new(Sym::from_f64(1.0), p("e", 1.0e-30), Sym::from_f64(0.0)).norm()
                    - Sym::from_f64(1.0)
            })
        ),
        "theorem",
        "a numerically-unit vector is not a unit vector"
    );

    // A scaled unit vector: ‖k·v̂‖ is |k|, and k is a parameter at 1.0.
    assert_ne!(
        sound(
            "‖k·v̂‖ − 1 with k a parameter at 1.0",
            how(s, || {
                (unit(3.0, 4.0, 12.0) * p("k", 1.0)).norm() - Sym::from_f64(1.0)
            })
        ),
        "theorem",
        "‖k·v̂‖ − 1 is not an identity in k"
    );

    // **The zero vector**: normalize is 0/0 in every component. Clause 1
    // is what must refuse, and `Form::recip`'s poison is the form-side
    // half of it.
    let zero = sound(
        "‖0̂‖ − 1 (the zero vector)",
        how(s, || {
            Vec3::new(p("a", 0.0), p("b", 0.0), p("c", 0.0))
                .normalize()
                .norm()
                - Sym::from_f64(1.0)
        }),
    );
    assert!(
        zero.starts_with("refused"),
        "the zero vector has no unit direction: clause 1 refuses, {zero}"
    );

    // The same vector family at the one point where it vanishes, and
    // at a point where the identity is real.
    let along = |x0: f64| {
        how(s, move || {
            Vec3::new(p("x", x0), Sym::from_f64(0.0), Sym::from_f64(0.0))
                .normalize()
                .norm()
                - Sym::from_f64(1.0)
        })
    };
    assert!(
        sound("‖(x,0,0)̂‖ − 1 at x = 0", along(0.0)).starts_with("refused"),
        "at x = 0 the vector is the zero vector"
    );
    assert_eq!(
        sound("‖(x,0,0)̂‖ − 1 at x = 2", along(2.0)),
        "theorem",
        "away from zero it is a real identity and the rule reaches it"
    );
}

/// **The shared monomial divided out where the monomial VANISHES**
/// (R1). `sqrt((x·a)/(x·b)) − sqrt(a/b)`: with the rule the two `sqrt`
/// atoms are ONE and the residual is the zero form — at every `x` the
/// expression has a value. At `x = 0` the left side is `sqrt(0/0)`,
/// which has none, and clause 1 is what must refuse.
#[test]
fn the_shared_factor_folded_where_it_vanishes_is_refused_by_clause_one() {
    println!("=== sqrt((x·a)/(x·b)) − sqrt(a/b)");
    let resid = |x0: f64| {
        move || {
            let x = p("x", x0);
            let a = p("a", 3.0);
            let b = p("b", 5.0);
            ((x * a) / (x * b)).sqrt() - (a / b).sqrt()
        }
    };
    assert_eq!(
        sound("x = 2 shipped", how(SymRules::shipped(), resid(2.0))),
        "theorem",
        "away from zero the shared factor cancels and the two sqrts are one atom"
    );
    assert_ne!(
        sound(
            "x = 2 without_rule_e",
            how(SymRules::without_rule_e(), resid(2.0))
        ),
        "theorem",
        "and without the rule they are two atoms"
    );
    let at_zero = sound("x = 0 shipped", how(SymRules::shipped(), resid(0.0)));
    assert!(
        at_zero.starts_with("refused"),
        "sqrt(0/0) has no value: clause 1 refuses before the identity test, {at_zero}"
    );
}

/// **A genuinely NON-UNIT vector is never normalised** (R1). The rule
/// folds a quotient to the constant its two halves are in ratio, and
/// only that: `‖3·v̂‖ − 1` and `‖½·v̂‖ − 1` must stay numeric, while
/// `‖3·v̂‖ − 3` — the TRUE statement about the same vector — is a
/// theorem, so the negatives fail for the right reason and not because
/// the tier reached nothing at all.
#[test]
fn a_non_unit_vector_never_acquires_a_unit_norm() {
    println!("=== non-unit vectors");
    let s = SymRules::shipped();
    for (name, scale) in [("3", 3.0), ("0.5", 0.5)] {
        assert_ne!(
            sound(
                &format!("‖{name}·v̂‖ − 1"),
                how(s, || {
                    (unit(3.0, 4.0, 12.0) * Sym::from_f64(scale)).norm() - Sym::from_f64(1.0)
                })
            ),
            "theorem",
            "‖{name}·v̂‖ is {name}, not one"
        );
    }
    assert_eq!(
        sound(
            "‖3·v̂‖ − 3",
            how(s, || {
                (unit(3.0, 4.0, 12.0) * Sym::from_f64(3.0)).norm() - Sym::from_f64(3.0)
            })
        ),
        "theorem",
        "the true statement about the same vector IS reached"
    );
}

/// **Two normalisations stacked** (R1) — the shape a derived frame
/// builds: `n̂ = normalize(normalize(v) × w)`. Both `n̂ · n̂ − 1` and
/// `n̂ · v̂` (the cross product is orthogonal to its operands) are real
/// identities, and the row is the value check on both: a fold that
/// went wrong two normalisations deep would show as a theorem whose
/// value is not zero.
#[test]
fn two_normalisations_stacked() {
    println!("=== stacked normalisations");
    let s = SymRules::shipped();
    let n_hat = || {
        let v = Vec3::new(p("vx", 1.0), p("vy", 2.0), p("vz", 3.0)).normalize();
        let w = Vec3::new(p("wx", 0.0), p("wy", 1.0), p("wz", 1.0));
        v.cross(w).normalize()
    };
    assert_eq!(
        sound(
            "‖n̂‖² − 1",
            how(s, || n_hat().dot(n_hat()) - Sym::from_f64(1.0))
        ),
        "theorem",
        "a doubly-normalised vector is still a unit vector"
    );
    assert_eq!(
        sound(
            "n̂ · v̂",
            how(s, || {
                let v = Vec3::new(p("vx", 1.0), p("vy", 2.0), p("vz", 3.0)).normalize();
                n_hat().dot(v) - Sym::from_f64(0.0)
            })
        ),
        "theorem",
        "a cross product is orthogonal to its operands and the tier reaches it \
         through two normalisations"
    );
}

/// **Rule E can cost a theorem to the coefficient ring** (R2's
/// `r2_the_scale_step_loses_a_rule_a_theorem_to_the_ring`, adopted as
/// the row that pins the limit BY NAME). The rule is monotone in terms
/// and in degree; it is NOT monotone in coefficient WIDTH, because the
/// scale step divides every numerator coefficient by the denominator's
/// pivot `s` and so adds `s`'s bits to each. Here `P = (x + |1/q|)/(x + 0.1)`
/// with `q = 3^126` (200 bits): unscaled, rule A substitutes `P` and
/// the product by `h` fits `rational::COEFF_BITS`; scaled by `1/0.1`,
/// `P`'s coefficients are 252 bits, the product needs 259, and both
/// sides freeze to two different indeterminates.
///
/// **This row is expected to be RED-adjacent by design**: it asserts
/// the loss. If it fails because the `on` side has become a theorem,
/// the loss has been closed and the row should be re-baselined, not
/// deleted.
#[test]
fn rule_e_can_cost_a_theorem_to_the_coefficient_ring() {
    let l33 = 5_559_060_566_555_523.0_f64; // 3^33, exact in f64
    let l27 = 7_625_597_484_987.0_f64; // 3^27
    let resid = || {
        let x = p("x", 1.5);
        let y = p("y", 1.5);
        let one = Sym::from_f64(1.0);
        let q = Sym::from_f64(l33) * Sym::from_f64(l33) * Sym::from_f64(l33) * Sym::from_f64(l27);
        let inv_q = one / q;
        let c = Sym::from_f64(0.1);
        let pf = (x + inv_q.abs()) / (x + c);
        let h = y + (one / Sym::from_f64(1000.0)).abs();
        (pf.sqrt().powi(2) * h).sqrt() - (pf * h).sqrt()
    };
    println!("=== the coefficient width the scale step moves");
    let off = sound("without_rule_e", how(SymRules::without_rule_e(), resid));
    let on = sound("shipped", how(SymRules::shipped(), resid));
    assert_eq!(off, "theorem", "without rule E the early walk reaches it");
    assert_ne!(
        on, "theorem",
        "RULE E COSTS THIS THEOREM: the scale step's coefficient width freezes the product. \
         The rule is monotone in terms and degree and NOT in coefficient width, and this row \
         is the standing statement of that. If this row fails the loss has been closed — \
         re-baseline it and say so."
    );
}

/// **The scale step's canonical representative** (R2): two spellings of
/// one quotient key ONE atom with the rule on, which is what makes rule
/// D's hand-built half-angle meet the walk's.
#[test]
fn two_spellings_of_one_quotient_key_one_atom() {
    println!("=== two spellings");
    let resid = || {
        let s = p("s", 1.5);
        let one = Sym::from_f64(1.0);
        let two = Sym::from_f64(2.0);
        let half = Sym::from_f64(0.5);
        ((s + one) / (two * s)).sqrt() - ((half * s + half) / s).sqrt()
    };
    assert_eq!(
        sound("shipped", how(SymRules::shipped(), resid)),
        "theorem",
        "(S+1)/(2S) and (½S+½)/S are one form, so one atom"
    );
}

/// **The scale step's SIGN choice, pinned.** `cancel` scales both
/// halves by `1/|s|` — the MAGNITUDE of the denominator's pivot
/// coefficient — so a quotient whose denominator leads with a negative
/// coefficient keeps that sign. Scaling by `1/s` instead would flip
/// both halves whenever the pivot is negative, and the rules that read
/// a form's SYNTAX would then be reading the pivot's sign rather than
/// the form: `manifest::nonneg` calls a polynomial non-negative
/// when every coefficient is, and `signed::poly_sqrt` looks for an
/// exact square, and a canonicalisation that manufactures either by
/// choosing a scale has made the rule above it an accident.
///
/// The flip is not UNSOUND — `(−N)/(−D)` is the same function — which
/// is why nothing else in the tree reds under it: with `s.recip()`
/// planted in place of `s.abs().recip()`, `geom-core`'s whole suite
/// (342 + 263 + 11 + 11) and `editor-core`'s 377 `m10_*` rows all stay
/// green (measured, SYM-5's fix pass). This row is what pins the
/// convention, and it is the only thing that does.
#[test]
fn the_scale_step_keeps_the_sign_of_both_halves() {
    // `(x + 2) / (−1 − y)`: the denominator's smallest monomial is the
    // constant, coefficient −1, so a signed scale would flip both.
    let resid = || {
        let x = p("x", 1.0);
        let y = p("y", 1.0);
        let q = (x + Sym::from_f64(2.0)) / (Sym::from_f64(-1.0) - y);
        // `q + |q|` is zero iff `q ≤ 0`, and `q` IS negative here; the
        // point of the row is the label, not this identity.
        q - q
    };
    // The residual `q − q` is the zero form under either sign choice,
    // so the sign is read where it is visible: through `atan2`'s
    // syntactic non-negativity, which rule D's A1 fold consults.
    let nonneg = || {
        let x = p("x", 1.0);
        let y = p("y", 1.0);
        // Both halves all-NEGATIVE: the quotient is positive, but the
        // form is not non-negative BY SYNTAX and `atan2(0, ·)` must not
        // fold. A signed scale flips both halves to all-positive and
        // the fold fires — on a true statement, but for the wrong
        // reason.
        //
        // The denominator's pivot is −2 and not −1 ON PURPOSE: at ±1
        // `cancel`'s no-op fast path returns the form untouched and no
        // scale runs at all, so the mutant would be invisible. A row
        // that pins a step has to reach the step.
        let n = (Sym::from_f64(-1.0) - x * x) / (Sym::from_f64(-2.0) - y * y - y * y);
        Sym::from_f64(0.0).atan2(n)
    };
    assert_eq!(sound("q − q", how(SymRules::shipped(), resid)), "theorem");
    let l = sound(
        "atan2(0, (−1−x²)/(−2−2y²))",
        how(SymRules::shipped(), nonneg),
    );
    assert_ne!(
        l, "theorem",
        "neither half is non-negative BY SYNTAX, so rule D's A1 fold must not fire: \
         if this reads `theorem` the scale step has flipped the signs and \
         `manifest::nonneg` is reading the pivot rather than the form"
    );
}

#[cfg(feature = "interval")]
mod gated {
    //! **The clause-3 gate survives the cancellation.** Rule C
    //! (`signed`, dial-off in the shipped set) folds `sqrt(R²)` to `R`
    //! by a certified sign, and marks the form `gated`: a zero reached
    //! through it is `sign_gated`, never `symbolic_zero`. `cancel`
    //! rebuilds the form, so it has to carry that flag — dropping it
    //! would report a weaker claim as a stronger one, the one
    //! direction the receipt may never move in.
    use super::{band, budget};
    use geom_core::predicate::{Margin, Sign};
    use geom_core::sym::with_session_rules;
    use geom_core::{Interval, ParamSymbol, Real, Sym, SymRules};

    #[test]
    fn the_clause_three_gate_survives_rule_e() {
        let (out, counts) = with_session_rules(budget(), SymRules::all(), || {
            let x = Sym::param_over(
                ParamSymbol::of("x"),
                Interval::from_bounds(2.0, 4.0),
                2.0,
                4.0,
            );
            let m = (x * x).sqrt() - x;
            geom_core::k_stats::decide("sym_rule_e_gate", Margin::of(m), band())
        });
        println!("gate row: {out:?} {counts:?}");
        assert!(matches!(out, Ok(Sign::Zero)), "rule C decides it: {out:?}");
        assert_eq!(
            counts.sign_gated, 1,
            "the decision is GATED, not a theorem: {counts:?}"
        );
        assert_eq!(
            counts.symbolic_zero, 0,
            "a dropped `gated` would read here as an unconditional theorem: {counts:?}"
        );
    }
}
