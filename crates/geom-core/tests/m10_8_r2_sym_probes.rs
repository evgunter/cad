//! **R2's independent probes of M10-8's atom algebra** — derived from
//! the claims, not from the unit's rows.
//!
//! Two jobs. The gating half attacks the SOUNDNESS of rules A and B
//! where they do fire (M10-8-SPEC's review claims 2 and 3): a `sqrt`
//! atom over a negative or straddling argument must never reach the
//! identity test, an odd power must not be folded, and two `sin`/`cos`
//! atoms of DIFFERENT arguments must never cancel. The evidence half
//! (`#[ignore]`d, prints) measures the unit's central NEGATIVE claim —
//! that a per-node reduction is a runaway — against a BOUNDED early
//! reduction tried ALONGSIDE the plain form (`CAD_M10_8_R2_EARLY`, the
//! term cap under which a node's form is reduced during the walk).
//!
//! Everything here is a deterministic fixture; nothing samples, so
//! nothing needs a seed ([[test-suite-cost]]).
//!
//! NO TEST IN THIS FILE IS EXECUTED BY CI — the whole file is behind
//! `#![cfg(all(feature = "interval", feature = "probe"))]` and nothing
//! in `scripts/k_probe_sweep.sh` rosters it. That is the disposition
//! this reviewer's suite asks for: the rows are a review artifact,
//! quoted in the report, and the branch is not proposed for merge as
//! it carries an experimental patch to `geom-core::sym`.

#![cfg(all(feature = "interval", feature = "probe"))]
#![allow(clippy::unwrap_used, clippy::panic, clippy::float_cmp)]

use geom_core::interval::Interval;
use geom_core::predicate::{Band, Sign};
use geom_core::real::Real;
use geom_core::sym::{with_session, with_session_rules};
use geom_core::{Decide, ParamSymbol, Sym, SymBudget, SymCounts, SymRules, Tol};

fn budget() -> SymBudget {
    SymBudget {
        max_terms: 4096,
        max_degree: 128,
    }
}

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// A parameter over `[lo, hi]`.
fn p(name: &str, lo: f64, hi: f64) -> Sym<Interval> {
    Sym::param(ParamSymbol::of(name), Interval::from_bounds(lo, hi))
}

fn lit(x: f64) -> Sym<Interval> {
    <Sym<Interval> as Real>::from_f64(x)
}

/// The decision the funnel would make about `m`.
fn sign_of(m: Sym<Interval>) -> Result<Sign, ()> {
    m.sign_within(band()).map_err(|_| ())
}

/// Runs `f` with every buildable rule ON — the configuration the unit
/// filed, and the one whose soundness the claims are about.
fn ruled<R>(f: impl FnOnce() -> R) -> (R, SymCounts) {
    with_session_rules(budget(), SymRules::all(), f)
}

// ------------------------------------------ claim 2: rule A's domain

/// **A `sqrt` of a NEGATIVE or STRADDLING argument never reaches the
/// identity test** (clause 1), so rule A cannot launder a domain
/// violation into a theorem. Three shapes, each with the atom squared
/// so that the rule WOULD fire if the domain gate let it: `sqrt(−y²)`,
/// a `sqrt` over a box that straddles zero, and `sqrt(x − x)` whose
/// argument is the zero form.
///
/// The first two must not answer `Zero` symbolically. The third is the
/// interesting one and is asserted separately: `sqrt(0) = 0` is a fold
/// the PLAIN form already takes (M10-7), so it stays a theorem here
/// whatever rule A does — what is pinned is that the answer did not
/// change when the rule was switched on.
#[test]
fn r2_rule_a_never_folds_a_sqrt_that_has_no_real_value() {
    // `sqrt(−(y²)) ² − (−(y²))`: the argument is negative for every
    // y ≠ 0, so no real value exists and clause 1 must refuse.
    let (out, counts) = ruled(|| {
        let y = p("y", 1.0, 2.0);
        let arg = -(y * y);
        let s = arg.sqrt();
        sign_of(s * s - arg)
    });
    assert_ne!(
        out,
        Ok(Sign::Zero),
        "a sqrt with no real value must not decide Zero"
    );
    assert_eq!(
        counts.symbolic_zero, 0,
        "clause 1 refuses before the identity test: {counts:?}"
    );

    // A box that STRADDLES: `x ∈ [−1, 1]`, `sqrt(x)² − x`. The identity
    // is true only on the half where the atom has a value; the tier
    // must not claim it over the whole box.
    let (out, counts) = ruled(|| {
        let x = p("x", -1.0, 1.0);
        let s = x.sqrt();
        sign_of(s * s - x)
    });
    assert_ne!(out, Ok(Sign::Zero), "a straddling sqrt must not fold");
    assert_eq!(counts.symbolic_zero, 0, "{counts:?}");
}

/// **`sqrt(x − x)` — the argument is the ZERO FORM** — folds to 0 by
/// the plain form's at-zero rule, with or without rule A, and the
/// answer is the same both ways. Pinned so a rule that changed it
/// would be visible.
#[test]
fn r2_a_sqrt_of_the_zero_form_answers_the_same_with_the_rules_on() {
    let case = || {
        let x = p("x", -1.0, 1.0);
        let s = (x - x).sqrt();
        sign_of(s * s)
    };
    let (on, c_on) = ruled(case);
    let (off, c_off) = with_session_rules(budget(), SymRules::none(), case);
    assert_eq!(on, off, "rule A moved a decision the plain form made");
    assert_eq!(
        c_on.symbolic_zero, c_off.symbolic_zero,
        "{c_on:?} vs {c_off:?}"
    );
}

/// **An ODD power of a `sqrt` atom is not its argument.** `sqrt(X)³ −
/// X` is NOT an identity (it is `X^{3/2} − X`), and the rule's
/// even-power rewrite must leave one factor of the atom behind rather
/// than collapsing it. Over a box where the two sides genuinely differ
/// the decision must not be `Zero`.
#[test]
fn r2_rule_a_leaves_the_odd_factor_alone() {
    let (out, counts) = ruled(|| {
        let x = p("x", 4.0, 9.0);
        let s = x.sqrt();
        sign_of(s * s * s - x)
    });
    assert_ne!(out, Ok(Sign::Zero), "X^{{3/2}} − X is not an identity");
    assert_eq!(counts.symbolic_zero, 0, "{counts:?}");
    // The true identity one power up IS reached, so the row above is
    // not passing because the rule is inert.
    let (ok, counts) = ruled(|| {
        let x = p("x", 4.0, 9.0);
        let s = x.sqrt();
        sign_of(s * s * s - x * s) == Ok(Sign::Zero)
    });
    assert!(ok, "sqrt(X)³ − X·sqrt(X) is an identity rule A reaches");
    assert_eq!(counts.symbolic_zero, 1, "{counts:?}");
}

// ------------------------------------------ claim 3: rule B's argument

/// **Rule B never crosses two arguments.** `sin²θ + cos²φ − 1` with
/// `θ ≠ φ` must stay numeric at every width, including one where the
/// two arguments coincide NUMERICALLY at the nominal — a coincidence,
/// not an identity, and the tier's whole job is to tell them apart.
#[test]
fn r2_rule_b_never_pairs_two_different_arguments() {
    for (lo, hi) in [(0.3_f64, 0.3_f64), (0.3, 0.9)] {
        let (out, counts) = ruled(|| {
            let a = p("theta", lo, hi);
            let b = p("phi", lo, hi);
            let (sa, _) = a.sin_cos();
            let (_, cb) = b.sin_cos();
            sign_of(sa * sa + cb * cb - lit(1.0))
        });
        assert_eq!(
            counts.symbolic_zero, 0,
            "two arguments were paired over [{lo}, {hi}]: {counts:?} ({out:?})"
        );
    }
    // The SAME argument reached through two different expressions that
    // share a normal form IS one argument, and does pair — the atom is
    // keyed by the argument's form, not by its node.
    let (ok, counts) = ruled(|| {
        let a = p("theta", 0.3, 0.9);
        let (s, _) = (a + a).sin_cos();
        let (_, c) = (lit(2.0) * a).sin_cos();
        sign_of(s * s + c * c - lit(1.0)) == Ok(Sign::Zero)
    });
    assert!(ok, "θ+θ and 2·θ are one argument form");
    assert_eq!(counts.symbolic_zero, 1, "{counts:?}");
}

/// **The Pythagorean pair is a theorem at EVERY width**, including a
/// macroscopic box — the property no enclosure has, and the reason a
/// rule is worth anything at all.
#[test]
fn r2_rule_b_holds_at_every_width() {
    for half in [1e-12_f64, 1e-3, 0.5, 3.0] {
        let (out, counts) = ruled(|| {
            let a = p("theta", 1.0 - half, 1.0 + half);
            let (s, c) = a.sin_cos();
            sign_of(s * s + c * c - lit(1.0))
        });
        assert_eq!(out, Ok(Sign::Zero), "half-width {half}");
        assert_eq!(counts.symbolic_zero, 1, "half-width {half}: {counts:?}");
    }
}

/// **`with_session` is not the shipped configuration.** The tier ships
/// with every rule OFF (`SymRules::shipped()` is empty and
/// `SymbolicDials::default()` carries it), but the un-suffixed session
/// door installs `SymRules::all()` — so a caller who writes
/// `with_session` gets the FILED algebra, not the shipped one. Pinned
/// as a fact about the API rather than as an approval of it.
#[test]
fn r2_the_unsuffixed_session_door_is_not_the_shipped_rule_set() {
    let case = || {
        let a = p("theta", 0.3, 0.9);
        let (s, c) = a.sin_cos();
        sign_of(s * s + c * c - lit(1.0))
    };
    let (_, plain) = with_session_rules(budget(), SymRules::shipped(), case);
    let (_, doored) = with_session(budget(), case);
    assert_eq!(plain.symbolic_zero, 0, "the SHIPPED tier proves nothing here");
    assert_eq!(
        doored.symbolic_zero, 1,
        "but `with_session` runs the filed algebra: {doored:?}"
    );
}

// ------------------------- evidence: the bounded early reduction (claim 2)

/// The arc family's blocking shape, at the scalar: `u·u − 1` for a
/// normalized vector `u = v/‖v‖`. `‖v‖ = sqrt(v·v)` is an atom, and
/// `u·u = (v·v)/sqrt(v·v)²` — the atom SQUARED in a denominator.
fn normalized_self_dot(n: usize) -> Sym<Interval> {
    let mut comps = Vec::new();
    for i in 0..n {
        comps.push(p(&format!("v{i}"), 0.5 + i as f64, 1.5 + i as f64));
    }
    let mut d = lit(0.0);
    for c in &comps {
        d = d + *c * *c;
    }
    let norm = d.sqrt();
    let mut uu = lit(0.0);
    for c in &comps {
        let u = *c / norm;
        uu = uu + u * u;
    }
    uu - lit(1.0)
}

/// **The top-residual reduction DOES reach the arc family's shape when
/// the form is small** — so the unit's "inert" result is about the
/// forms the documents build, not about the rule. Evidence-only.
#[test]
#[ignore = "evidence-only: prints where the top-residual fold reaches the normalize shape"]
fn r2_the_normalize_shape_under_each_rule_set() {
    for n in 1..=6 {
        let (out, counts) = ruled(|| sign_of(normalized_self_dot(n)));
        let (_, plain) = with_session_rules(budget(), SymRules::none(), || {
            sign_of(normalized_self_dot(n))
        });
        println!(
            "   n={n}: A+B {out:?} symbolic={} frozen={} | none symbolic={} frozen={}",
            counts.symbolic_zero, counts.frozen, plain.symbolic_zero, plain.frozen
        );
    }
}

/// **The bounded EARLY reduction, alongside the plain form.** Prints
/// what it discharges that the top-residual fold does not, at each cap.
/// Set `CAD_M10_8_R2_EARLY` to the cap before running.
#[test]
#[ignore = "evidence-only: needs CAD_M10_8_R2_EARLY set; prints the early pass's extra discharges"]
fn r2_the_bounded_early_reduction_on_the_normalize_shape() {
    let cap = std::env::var("CAD_M10_8_R2_EARLY").unwrap_or_default();
    for n in 1..=8 {
        let start = std::time::Instant::now();
        let (out, counts) = with_session_rules(budget(), SymRules::none(), || {
            sign_of(normalized_self_dot(n))
        });
        println!(
            "   cap={cap} n={n}: {out:?} symbolic={} early={} frozen={} in {:?}",
            counts.symbolic_zero,
            counts.sign_gated,
            counts.frozen,
            start.elapsed()
        );
    }
}

/// **The positive control for R2's DEEP variant** (`CAD_M10_8_R2_DEEP`):
/// `sqrt(sqrt(X)^2 - X)` is a residual whose whole body is ONE atom
/// whose ARGUMENT is an identity — the shape M10-8's own shape report
/// prints for `carrier_endpoint_start`. The shipped top-residual fold
/// cannot see it (the even power is inside the outer atom's argument);
/// the deep fold reduces the argument to the zero form and takes the
/// at-zero fold. Evidence-only: it prints.
#[test]
#[ignore = "evidence-only: prints whether the deep fold reaches a nested identity"]
fn r2_the_deep_fold_reaches_an_identity_inside_an_atoms_argument() {
    let depth = std::env::var("CAD_M10_8_R2_DEEP").unwrap_or_else(|_| "0".into());
    let (out, counts) = with_session_rules(budget(), SymRules::none(), || {
        let (x, y) = (p("w", 3.0, 3.0), p("h", 0.25, 0.25));
        let arg = x * x + y * y + lit(1.0);
        let s = arg.sqrt();
        let a = sign_of((s * s - arg).abs());
        let b = sign_of((s * s - arg + lit(4.0)).sqrt() - lit(2.0));
        (a, b)
    });
    println!("   DEEP={depth}: nested identity abs/re-mint -> {out:?} {counts:?}");
}
