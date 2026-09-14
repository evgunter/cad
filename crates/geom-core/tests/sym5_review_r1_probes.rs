//! **R1's independent probes of SYM-5 PR-2's rule E** (the quotient's
//! common factor), attacked through the PUBLIC scalar rather than the
//! module's own tests. Every row that expects a THEOREM also checks the
//! residual's `f64` (or interval) VALUE at the point, because a fold
//! that decides `Zero` on a residual that is not numerically zero is
//! the one defect these rows exist to catch.
//!
//! Evidence-only: these rows print and assert soundness, never a
//! particular count.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::linalg::Vec3;
use geom_core::predicate::{Band, Margin, Sign};
use geom_core::sym::with_session_rules;
use geom_core::{Interval, ParamSymbol, Real, Sym, SymBudget, SymRules, Tol};

fn budget() -> SymBudget {
    SymBudget {
        max_terms: 4096,
        max_degree: 128,
    }
}

fn band() -> Band {
    Band::linear(Tol::witness()).expect("the witness tolerance has a linear band")
}

fn p(name: &str, v: f64) -> Sym<f64> {
    Sym::param(ParamSymbol::of(name), v)
}

fn pi(name: &str, lo: f64, hi: f64) -> Sym<Interval> {
    Sym::param_over(ParamSymbol::of(name), Interval::from_bounds(lo, hi), lo, hi)
}

/// How the tier answered the margin, and the margin's value at the
/// point.
fn how_f64(rules: SymRules, build: impl FnOnce() -> Sym<f64>) -> (String, f64) {
    let ((out, value), counts) = with_session_rules(budget(), rules, || {
        let m = build();
        (
            geom_core::k_stats::decide("sym5_r1", Margin::of(m), band()),
            m.value,
        )
    });
    (label(out, counts), value)
}

fn how_iv(rules: SymRules, build: impl FnOnce() -> Sym<Interval>) -> (String, String) {
    let ((out, value), counts) = with_session_rules(budget(), rules, || {
        let m = build();
        (
            geom_core::k_stats::decide("sym5_r1", Margin::of(m), band()),
            format!("{:?}", m.value),
        )
    });
    (label(out, counts), value)
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

/// A theorem must be numerically zero at the point.
fn sound(what: &str, (l, v): (String, f64)) {
    println!("  {what}: {l} value {v:e}");
    if l == "theorem" {
        assert!(
            v.abs() <= 1.0e-12,
            "{what}: SAYS IDENTICALLY ZERO but the value at the point is {v:e} — UNSOUND"
        );
    }
}

fn unit(x: f64, y: f64, z: f64) -> Vec3<Sym<f64>> {
    Vec3::new(p("x", x), p("y", y), p("z", z)).normalize()
}

/// **The shapes rule E newly folds, each checked against the value.**
/// A nearly-unit vector, a scaled one, a vector whose norm is a
/// parameter that can be zero, and a zero vector.
#[test]
fn r1_the_newly_folded_shapes_are_checked_against_the_value() {
    let s = SymRules::shipped();
    println!("=== rule E's newly folded shapes, shipped set");
    sound(
        "|v_hat| - 1",
        how_f64(s, || unit(3.0, 4.0, 12.0).norm() - Sym::from_f64(1.0)),
    );
    // A vector that is unit only NUMERICALLY: (1, eps, 0) with eps a
    // parameter whose value is tiny. Its norm is sqrt(1 + eps^2), not 1.
    sound(
        "|(1,eps,0)| - 1 with eps = 1e-30 (numerically one, not one)",
        how_f64(s, || {
            Vec3::new(Sym::from_f64(1.0), p("e", 1.0e-30), Sym::from_f64(0.0)).norm()
                - Sym::from_f64(1.0)
        }),
    );
    // A re-normalised vector scaled by a parameter whose VALUE is one.
    sound(
        "|k * v_hat| - 1 with k a parameter at 1.0",
        how_f64(s, || {
            (unit(3.0, 4.0, 12.0) * p("k", 1.0)).norm() - Sym::from_f64(1.0)
        }),
    );
    // The zero vector: normalize is 0/0 in every component.
    sound(
        "|zero_hat| - 1",
        how_f64(s, || {
            Vec3::new(p("a", 0.0), p("b", 0.0), p("c", 0.0))
                .normalize()
                .norm()
                - Sym::from_f64(1.0)
        }),
    );
    // A vector whose norm VANISHES at the nominal through a parameter
    // that is also the shared factor: v = (x, 0, 0) at x = 0.
    sound(
        "|(x,0,0)_hat| - 1 at x = 0",
        how_f64(s, || {
            Vec3::new(p("x", 0.0), Sym::from_f64(0.0), Sym::from_f64(0.0))
                .normalize()
                .norm()
                - Sym::from_f64(1.0)
        }),
    );
    // ... and at x = 2, where it IS a unit vector and the identity is real.
    sound(
        "|(x,0,0)_hat| - 1 at x = 2",
        how_f64(s, || {
            Vec3::new(p("x", 2.0), Sym::from_f64(0.0), Sym::from_f64(0.0))
                .normalize()
                .norm()
                - Sym::from_f64(1.0)
        }),
    );
}

/// **The shared monomial divided out where the monomial VANISHES.**
/// `sqrt((x·a)/(x·b)) − sqrt(a/b)`: with rule E off the two `sqrt`s are
/// two atoms and the residual is numeric; with it on they are ONE atom
/// and the residual is the zero form. At `x = 0` the left side is
/// `sqrt(0/0)`, which has no value — so the row asks whether clause 1
/// catches it.
#[test]
fn r1_the_shared_monomial_divided_out_where_it_vanishes() {
    println!("=== sqrt((x a)/(x b)) - sqrt(a/b)");
    for x0 in [2.0_f64, 0.0] {
        for (name, rules) in [
            ("shipped", SymRules::shipped()),
            ("no_e", SymRules::without_rule_e()),
        ] {
            let r = how_f64(rules, || {
                let x = p("x", x0);
                let a = p("a", 3.0);
                let b = p("b", 5.0);
                ((x.clone() * a.clone()) / (x * b.clone())).sqrt() - (a / b).sqrt()
            });
            sound(&format!("x = {x0} {name}"), r);
        }
    }
    // The same, over an interval box that STRADDLES zero: clause 1
    // should refuse before any identity test is asked.
    for (name, rules) in [
        ("shipped", SymRules::shipped()),
        ("no_e", SymRules::without_rule_e()),
    ] {
        let (l, v) = how_iv(rules, || {
            let x = pi("x", -1.0e-3, 1.0e-3);
            let a = pi("a", 3.0, 3.0);
            let b = pi("b", 5.0, 5.0);
            ((x.clone() * a.clone()) / (x * b.clone())).sqrt() - (a / b).sqrt()
        });
        println!("  straddling box {name}: {l} value {v}");
        assert_ne!(
            l, "theorem",
            "a box where the shared factor vanishes must not be a theorem"
        );
    }
}

/// **An interval box whose parameter straddles zero through a
/// re-normalisation.** `‖v̂‖ − 1` on `v = (x, 0, 0)` over a box
/// containing `x = 0`: the identity is false at the one point where the
/// vector is the zero vector, so the tier must not answer `Zero`
/// symbolically over that box.
#[test]
fn r1_a_re_normalisation_over_a_straddling_box() {
    println!("=== |(x,0,0)_hat| - 1 over boxes");
    for (lo, hi) in [(1.0, 3.0), (-1.0e-3, 1.0e-3), (0.0, 2.0)] {
        for (name, rules) in [
            ("shipped", SymRules::shipped()),
            ("no_e", SymRules::without_rule_e()),
        ] {
            let (l, v) = how_iv(rules, || {
                let x = pi("x", lo, hi);
                Vec3::new(x, Sym::from_f64(0.0), Sym::from_f64(0.0))
                    .normalize()
                    .norm()
                    - Sym::from_f64(1.0)
            });
            println!("  box [{lo},{hi}] {name}: {l} value {v}");
            if (lo..=hi).contains(&0.0) {
                assert_ne!(
                    l, "theorem",
                    "box [{lo},{hi}] contains the zero vector: ‖v̂‖ − 1 is not an identity there"
                );
            }
        }
    }
}

/// **A genuinely NON-UNIT vector driven through the same door.** A
/// careless rule would normalise it; rule E must fold only the ratio
/// its two halves actually are.
#[test]
fn r1_a_genuinely_non_unit_vector_is_never_normalised() {
    println!("=== non-unit vectors");
    let s = SymRules::shipped();
    let v = || Vec3::new(p("x", 3.0), p("y", 4.0), p("z", 12.0));
    // ‖v‖ − 1 for a vector of norm 13.
    sound("|v| - 1 (|v| = 13)", how_f64(s, || v().norm() - Sym::from_f64(1.0)));
    // ‖v‖ − 13 is TRUE at the point but not an identity in the parameters.
    sound("|v| - 13", how_f64(s, || v().norm() - Sym::from_f64(13.0)));
    // v · v̂ − ‖v‖ IS an identity.
    sound(
        "v . v_hat - |v|",
        how_f64(s, || v().dot(v().normalize()) - v().norm()),
    );
    // v̂ · v̂ − 1 is an identity.
    sound(
        "v_hat . v_hat - 1",
        how_f64(s, || {
            v().normalize().dot(v().normalize()) - Sym::from_f64(1.0)
        }),
    );
    // (v̂ × w) · v̂ − 0 is an identity; the cross of a normalised axis
    // with another vector is orthogonal to it.
    sound(
        "(v_hat x w) . v_hat",
        how_f64(s, || {
            let w = Vec3::new(p("u", 1.0), p("q", -2.0), p("r", 0.5));
            v().normalize().cross(w).dot(v().normalize())
        }),
    );
    // ‖v̂ × ŵ‖ − 1 is NOT an identity (only for orthogonal v, w).
    sound(
        "|v_hat x w_hat| - 1",
        how_f64(s, || {
            let w = Vec3::new(p("u", 1.0), p("q", -2.0), p("r", 0.5));
            v().normalize().cross(w.normalize()).norm() - Sym::from_f64(1.0)
        }),
    );
}

/// **Two normalisations stacked** — the shape a derived frame builds:
/// `n̂ = normalize(normalize(v) × w)`, then `n̂ · n̂ − 1` and
/// `n̂ · v̂` (not an identity).
#[test]
fn r1_two_normalisations_stacked() {
    println!("=== stacked normalisations");
    for (name, rules) in [
        ("shipped", SymRules::shipped()),
        ("no_e", SymRules::without_rule_e()),
    ] {
        let mk = || {
            let v = Vec3::new(p("x", 3.0), p("y", 4.0), p("z", 12.0)).normalize();
            let w = Vec3::new(p("u", 1.0), p("q", -2.0), p("r", 0.5));
            v.cross(w).normalize()
        };
        sound(
            &format!("{name}: n_hat . n_hat - 1"),
            how_f64(rules, || mk().dot(mk()) - Sym::from_f64(1.0)),
        );
        sound(
            &format!("{name}: n_hat . v_hat (orthogonal: an identity)"),
            how_f64(rules, || {
                let v = Vec3::new(p("x", 3.0), p("y", 4.0), p("z", 12.0)).normalize();
                mk().dot(v)
            }),
        );
        sound(
            &format!("{name}: n_hat . w (NOT an identity)"),
            how_f64(rules, || {
                let w = Vec3::new(p("u", 1.0), p("q", -2.0), p("r", 0.5));
                mk().dot(w) - Sym::from_f64(0.0)
            }),
        );
    }
}

/// **Rule E beside rule D**: a `cos`/`sin` of a quarter `atan` whose
/// argument is spelled two ways, one of which carries a common factor
/// the rule divides out. The residual must never be a false theorem.
#[test]
fn r1_rule_e_beside_rule_d() {
    println!("=== rule E beside rule D");
    for (name, rules) in [
        ("shipped", SymRules::shipped()),
        ("no_e", SymRules::without_rule_e()),
    ] {
        // cos(atan(X)) - 1/sqrt(1 + X^2) with X spelled (k·a)/(k·b).
        sound(
            &format!("{name}: cos(atan((k a)/(k b))) - 1/sqrt(1 + (a/b)^2)"),
            how_f64(rules, || {
                let k = p("k", 2.0);
                let a = p("a", 3.0);
                let b = p("b", 5.0);
                let x = (k.clone() * a.clone()) / (k * b.clone());
                let y = a / b;
                x.atan().cos() - Sym::from_f64(1.0) / (Sym::from_f64(1.0) + y.clone() * y).sqrt()
            }),
        );
        // sin(atan(X)/4)^2 + cos(atan(X)/4)^2 - 1: a quarter angle, the
        // half-angle atoms, and the Pythagorean identity.
        sound(
            &format!("{name}: sin^2 + cos^2 - 1 at a quarter atan"),
            how_f64(rules, || {
                let x = p("x", 0.7);
                let t = x.atan() * Sym::from_f64(0.25);
                let (s, c) = t.sin_cos();
                s.clone() * s + c.clone() * c - Sym::from_f64(1.0)
            }),
        );
    }
}
