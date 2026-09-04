//! **R2's independent probes of the symbolic identity tier** (M10-7,
//! ERROR-DESIGN E12) — derived from the claims rather than from the
//! unit's own rows, so a row here that agrees with `sym::tests` agrees
//! by construction of the tier and not by construction of the test.
//!
//! What each block attacks is named at the block. Everything is a
//! deterministic fixture (no sampling), so nothing here needs a seed.

#![cfg(all(feature = "interval", feature = "probe"))]
#![allow(clippy::unwrap_used, clippy::panic, clippy::float_cmp)]

use geom_core::interval::Interval;
use geom_core::k_stats::decide;
use geom_core::k_stats::{Probe, SampleOutcome, start_recording, take_samples};
use geom_core::predicate::{Band, Margin, Sign};
use geom_core::real::Real;
use geom_core::sym::with_session;
use geom_core::{Decide, ParamSymbol, Sym, SymBudget, Tol};

fn budget() -> SymBudget {
    SymBudget {
        max_terms: 4096,
        max_degree: 128,
    }
}

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// A parameter bound over `[lo, hi]` — the shape `param_env_over` binds.
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

// ------------------------------------------------- claim 2: identities

/// **Identities by four different routes decide `Zero` at every width**,
/// including a MACROSCOPIC one — the property no enclosure has.
#[test]
fn r2_identities_by_different_routes_decide_zero_at_every_width() {
    // 7.0 is deliberately absent: at that half-width `dx` and `t` both
    // reach zero, `‖w‖` reaches zero with them, and the quotient row's
    // division is then a DOMAIN VIOLATION — clause 1 refuses it, which
    // is the honest answer and is exactly the slab's measured ceiling.
    for half in [1e-12_f64, 1e-3, 0.5, 1.5] {
        let (out, counts) = with_session(budget(), || {
            let t = p("t", 1.0 - half, 1.0 + half);
            let a = p("a", 2.0 - half, 2.0 + half);
            let b = p("b", -half, half);
            let dx = p("dx", 3.0 - half, 3.0 + half);

            // (1) P + t·d − P against t·d.
            let pt = a + t * dx;
            let r1 = (pt - a) - t * dx;
            // (2) (a+b) − b − a.
            let r2 = ((a + b) - b) - a;
            // (3) a cross-product component d × d (the z component).
            let r3 = dx * t - t * dx;
            // (4) the quotient identity D1 exists for: w·(‖w‖·‖w‖⁻¹ − 1).
            let norm = (dx * dx + t * t).sqrt();
            let r4 = dx * (norm / norm - lit(1.0));
            // (5) sqrt(x²+y²) − sqrt(y²+x²) — atoms keyed by form.
            let r5 = (dx * dx + t * t).sqrt() - (t * t + dx * dx).sqrt();
            vec![
                ("P+t·d−P", sign_of(r1)),
                ("(a+b)−b−a", sign_of(r2)),
                ("d×d", sign_of(r3)),
                ("w(‖w‖/‖w‖−1)", sign_of(r4)),
                ("sqrt sym", sign_of(r5)),
            ]
        });
        for (name, s) in &out {
            assert_eq!(
                *s,
                Ok(Sign::Zero),
                "half-width {half}: the identity {name} did not decide Zero"
            );
        }
        assert_eq!(
            counts.symbolic_zero,
            out.len() as u64,
            "half-width {half}: every one of these must be a SYMBOLIC zero, \
             not a numeric one that happened to be tight"
        );
    }
}

/// **A COINCIDENCE at the nominal never decides symbolically, and its
/// enclosure widens with the box** — the distinction E12 claims the
/// tier can make and no enclosure can.
#[test]
fn r2_a_nominal_coincidence_never_decides_symbolically_and_widens() {
    let mut widths = Vec::new();
    for half in [1e-9_f64, 1e-6, 1e-3] {
        let (w, counts) = with_session(budget(), || {
            // A radius equal to a distance AT THE NOMINAL only.
            let r = p("r", 1.0 - half, 1.0 + half);
            let d = p("d", 1.0 - half, 1.0 + half);
            let m = r - d;
            let s = sign_of(m);
            assert!(
                s != Ok(Sign::Zero) || half < 1e-12,
                "half-width {half}: a coincidence decided Zero"
            );
            geom_core::Bounds::hi(m) - geom_core::Bounds::lo(m)
        });
        assert_eq!(
            counts.symbolic_zero, 0,
            "half-width {half}: a coincidence produced a symbolic zero"
        );
        widths.push(w);
    }
    assert!(
        widths[0] < widths[1] && widths[1] < widths[2],
        "the coincidence's enclosure must WIDEN with the box: {widths:?}"
    );
}

/// **Two segments collinear at p₀ only.** The perp-dot of the two
/// directions is zero at the nominal and not identically zero; the tier
/// must leave it to the numeric channel at every width.
#[test]
fn r2_collinear_only_at_the_nominal_is_never_a_theorem() {
    for half in [0.0_f64, 1e-9, 1e-3] {
        let (s, counts) = with_session(budget(), || {
            let k = p("k", 1.0 - half, 1.0 + half);
            // dir A = (1, 1); dir B = (k, 1). perp-dot = 1·1 − 1·k.
            let perp = lit(1.0) * lit(1.0) - lit(1.0) * k;
            sign_of(perp)
        });
        assert_eq!(
            counts.symbolic_zero, 0,
            "half-width {half}: a nominal collinearity decided symbolically ({s:?})"
        );
    }
}

// ------------------------------- claim 2: clause 1 and the D1 quotient

/// **A domain violation never certifies symbolically** — `sqrt(-1) −
/// sqrt(-1)` is the zero form and has no real value.
#[test]
fn r2_a_domain_violation_never_certifies_symbolically() {
    let (s, counts) = with_session(budget(), || {
        let x = p("x", -4.0, -1.0);
        let r = x.sqrt() - x.sqrt();
        sign_of(r)
    });
    assert!(s.is_err(), "sqrt of a negative box decided {s:?}");
    assert_eq!(counts.symbolic_zero, 0);
}

/// **`0 · (1/y)` where `y` reaches zero.** The numerator's form is the
/// zero polynomial, so the quotient form is zero; the value channel's
/// `Trv` is the only thing standing between that and a false theorem.
#[test]
fn r2_a_zero_numerator_over_a_zero_reaching_denominator_is_refused() {
    let (s, counts) = with_session(budget(), || {
        let x = p("x", 1.0, 2.0);
        let y = p("y", -1.0, 1.0);
        let r = (x - x) / y;
        sign_of(r)
    });
    assert!(
        s.is_err(),
        "a division whose denominator reaches zero decided {s:?} — clause 1 did not hold"
    );
    assert_eq!(counts.symbolic_zero, 0);
}

/// The same shape with a denominator that does NOT reach zero: the
/// quotient is a theorem and decides Zero at a macroscopic width.
#[test]
fn r2_a_zero_numerator_over_a_safe_denominator_is_a_theorem() {
    let (s, counts) = with_session(budget(), || {
        let x = p("x", 1.0, 9.0);
        let y = p("y", 2.0, 8.0);
        sign_of((x - x) / y)
    });
    assert_eq!(s, Ok(Sign::Zero));
    assert_eq!(counts.symbolic_zero, 1);
}

/// **`(a/b)·b − a` is NOT folded** (the documented limit) — and where
/// `b` reaches zero it must not decide either.
#[test]
fn r2_the_reciprocal_is_opaque_in_both_directions() {
    let (safe, _) = with_session(budget(), || {
        let a = p("a", 1.0, 2.0);
        let b = p("b", 3.0, 4.0);
        sign_of((a / b) * b - a)
    });
    // MEASURED: with D1's quotient form this DOES decide Zero — the
    // field of fractions gives `(a/b)·b = a` for `b ≠ 0`, and clause 1
    // supplies the side condition. Sound, and it CONTRADICTS
    // `geom_core::sym`'s module docs, which still list
    // "`(x/y)·y − x` does not decide symbolically" among the tier's
    // documented limits (`sym.rs`'s "The documented limits" paragraph).
    assert_eq!(
        safe,
        Ok(Sign::Zero),
        "EXPECTED-DOC-DEFECT ROW: if this is no longer Zero, the module \
         docs' documented-limits paragraph has become true again"
    );
    let (unsafe_, counts) = with_session(budget(), || {
        let a = p("a", 1.0, 2.0);
        let b = p("b", -1.0, 1.0);
        sign_of((a / b) * b - a)
    });
    assert_ne!(unsafe_, Ok(Sign::Zero));
    assert_eq!(counts.symbolic_zero, 0);
}

/// **The atom-at-zero folds are the function's true value at zero.**
/// `sqrt 0 = 0`, `cos 0 = 1`, `acos 0 = π/2` — each checked as the
/// IDENTITY it licenses, and each checked not to fold the wrong way.
#[test]
fn r2_the_atom_at_zero_folds_are_the_functions_own_values() {
    let (out, _) = with_session(budget(), || {
        let x = p("x", 0.25, 0.5);
        // A zero form whose ENCLOSURE stays inside every domain below:
        // `powi(2)` is tight at interval, so this is `[0, w²]`, which is
        // the shape a squared-distance residual actually arrives in.
        let z = (x - x).powi(2);
        let pi = <Sym<Interval> as Real>::pi();
        vec![
            ("sqrt 0", sign_of(z.sqrt())),
            ("cos 0 − 1", sign_of(z.sin_cos().1 - lit(1.0))),
            ("acos 0 − π/2", sign_of(z.acos() - pi / lit(2.0))),
            ("sin 0", sign_of(z.sin_cos().0)),
            ("tan 0", sign_of(z.tan())),
            ("atan 0", sign_of(z.atan())),
            ("asin 0", sign_of(z.asin())),
            ("abs 0", sign_of(z.abs())),
            ("floor 0", sign_of(z.floor())),
            // τ − 2π: the value channel's τ against the node's 2·π.
            (
                "τ − 2π",
                sign_of(<Sym<Interval> as Real>::tau() - lit(2.0) * pi),
            ),
        ]
    });
    for (name, s) in &out {
        assert_eq!(*s, Ok(Sign::Zero), "the fold {name} did not decide Zero");
    }
    // And the WRONG folds are absent: cos 0 is not 0, acos 0 is not 0.
    let (wrong, _) = with_session(budget(), || {
        let x = p("x", 0.25, 0.5);
        let z = (x - x).powi(2);
        vec![
            ("cos 0", sign_of(z.sin_cos().1)),
            ("acos 0", sign_of(z.acos())),
        ]
    });
    for (name, s) in &wrong {
        assert_ne!(*s, Ok(Sign::Zero), "{name} folded to zero, which is false");
    }
}

/// **`min`/`max`/`abs`/`floor`/`copysign` as atoms keyed by their
/// arguments.** None of these may claim a cancellation the tier does
/// not have: `min(x,x) − x`, `abs(x) − abs(−x)` and `floor` at an
/// integer are all conservative misses, never theorems.
#[test]
fn r2_the_kink_atoms_never_claim_a_cancellation() {
    let (out, counts) = with_session(budget(), || {
        let x = p("x", 1.0, 2.0);
        let n = p("n", 3.0, 3.0); // an exact integer box
        vec![
            ("min(x,x)−x", sign_of(x.min(x) - x)),
            ("max(x,x)−x", sign_of(x.max(x) - x)),
            ("abs(x)−abs(−x)", sign_of(x.abs() - (-x).abs())),
            ("floor(n)−n", sign_of(n.floor() - n)),
            ("copysign(x,x)−x", sign_of(x.copysign(x) - x)),
        ]
    });
    // The claim is about the SYMBOLIC channel: `floor(n) − n` on a
    // degenerate box is a legitimate NUMERIC zero, so the sign alone
    // cannot carry the row.
    assert_eq!(
        counts.symbolic_zero, 0,
        "an atom keyed by its arguments claimed an identity it has no \
         argument for, among {out:?}"
    );
    // The one fold these DO license: min/max/copysign of zero forms.
    let (folds, _) = with_session(budget(), || {
        let x = p("x", 1.0, 2.0);
        let z = x - x;
        vec![
            ("min(0,0)", sign_of(z.min(z))),
            ("max(0,0)", sign_of(z.max(z))),
            ("copysign(0,x)", sign_of(z.copysign(x))),
        ]
    });
    for (name, s) in &folds {
        assert_eq!(*s, Ok(Sign::Zero), "{name} should be a theorem");
    }
}

// ---------------------------------------------- claim 3: freezing

/// **An i128 coefficient overflow FREEZES**, and a frozen form never
/// decides Zero falsely: the same expression that is an identity still
/// decides Zero (frozen nodes with equal ids cancel), while a
/// non-identity built past the overflow decides numerically.
#[test]
fn r2_a_coefficient_overflow_freezes_and_never_decides_falsely() {
    // A literal with a huge dyadic exponent, squared repeatedly: the
    // exponent field is i32 and the odd part is 1, so the freeze this
    // reaches is the DEGREE budget rather than the coefficient. Force
    // the coefficient path instead with a product of large odd
    // literals, whose numerators multiply.
    let (frozen_seen, counts) = with_session(budget(), || {
        let mut acc = lit(1.0);
        // 3^k with odd numerators: 3, 9, 27 … the product's numerator
        // overflows i128 after ~80 factors.
        for _ in 0..96 {
            acc = acc * lit(3.0);
        }
        let r = acc - acc; // still an identity: the frozen node cancels
        (sign_of(r), sign_of(acc))
    });
    assert!(counts.frozen > 0, "no form froze: {counts:?}");
    assert_eq!(
        frozen_seen.0,
        Ok(Sign::Zero),
        "a frozen node must still cancel against itself"
    );
    assert_ne!(
        frozen_seen.1,
        Ok(Sign::Zero),
        "a frozen non-identity decided Zero"
    );
}

/// **A degree overflow freezes**, and the identity through it survives
/// while the non-identity does not.
#[test]
fn r2_a_degree_overflow_freezes() {
    let tight = SymBudget {
        max_terms: 4096,
        max_degree: 4,
    };
    let (out, counts) = with_session(tight, || {
        let x = p("x", 1.0, 2.0);
        let deep = x * x * x * x * x * x * x * x; // degree 8 > 4
        (sign_of(deep - deep), sign_of(deep))
    });
    assert!(
        counts.frozen > 0,
        "the degree budget did not freeze: {counts:?}"
    );
    assert_eq!(out.0, Ok(Sign::Zero));
    assert_ne!(out.1, Ok(Sign::Zero));
}

/// **A budget of zero terms is the tier switched off**: nothing is
/// asked of the DAG, so even a plain identity decides numerically.
#[test]
fn r2_a_zero_budget_reproduces_the_numeric_only_lane() {
    let (s, counts) = with_session(SymBudget::none(), || {
        let x = p("x", 0.0, 1.0);
        sign_of(x - x)
    });
    assert_eq!(
        counts.symbolic_zero, 0,
        "a zero budget answered symbolically"
    );
    assert_eq!(
        counts.frozen, 0,
        "a zero budget should not even build a form"
    );
    // The numeric answer over [0,1] − [0,1] = [−1,1] is indeterminate.
    assert!(s.is_err(), "the numeric lane decided {s:?} on [−1,1]");
}

// ------------------------------------------------ claim 5: determinism

/// **Node ids are content hashes**: the same expression built in two
/// sessions, in different insertion orders and on different threads,
/// carries the same id.
#[test]
fn r2_node_ids_are_stable_across_sessions_orders_and_threads() {
    let build = || {
        with_session(budget(), || {
            let a = p("a", 0.0, 1.0);
            let b = p("b", 2.0, 3.0);
            ((a + b) * a - b).node().bits()
        })
        .0
    };
    let one = build();
    let two = build();
    assert_eq!(one, two, "two sessions minted different ids");
    // A different insertion order for the same expression.
    let reordered = with_session(budget(), || {
        let b = p("b", 2.0, 3.0);
        let _noise = b * b * b;
        let a = p("a", 0.0, 1.0);
        ((a + b) * a - b).node().bits()
    })
    .0;
    assert_eq!(one, reordered, "insertion order changed an id");
    let on_thread = std::thread::spawn(build).join().unwrap();
    assert_eq!(one, on_thread, "another thread minted a different id");
}

/// **No session, no tier**: a `Sym` built outside `with_session` still
/// carries a deterministic id, and every decision on it is numeric —
/// there is no partial-on state.
#[test]
fn r2_outside_a_session_the_tier_is_wholly_off() {
    let x = p("x", 0.0, 1.0);
    let id_out = (x - x).node().bits();
    assert!(geom_core::sym::session_counts().is_none());
    assert!(
        sign_of(x - x).is_err(),
        "an identity decided Zero with no session installed"
    );
    // The id is the SAME one a session would have minted.
    let id_in = with_session(budget(), || {
        let y = p("x", 0.0, 1.0);
        (y - y).node().bits()
    })
    .0;
    assert_eq!(
        id_out, id_in,
        "the id depended on whether a session existed"
    );
    // And a value minted OUTSIDE a session, decided INSIDE one: the
    // node is missing from the table and FREEZES — but two frozen nodes
    // with the same id still cancel, so the decision is SYMBOLIC.
    //
    // MEASURED, and it falsifies the PR body's "the lookup misses, the
    // form freezes, every decision is numeric — the tier is never
    // partially on": here the lookup misses, the form freezes, and the
    // decision is still the tier's. Sound (x − x is zero whatever x is),
    // but not what the sentence says.
    let (s, counts) = with_session(budget(), || sign_of(x - x));
    assert_eq!(
        s,
        Ok(Sign::Zero),
        "EXPECTED-DOC-DEFECT ROW: a node minted before the session was \
         installed decided {s:?}"
    );
    assert_eq!(counts.symbolic_zero, 1);
    assert!(
        counts.frozen > 0,
        "the missing node did not freeze: {counts:?}"
    );
}

// ------------------------------------------------- claim 7: the K row

/// **A symbolic zero RE-TAGS the base scalar's sample** — one sample per
/// decision, with its real margin kept.
#[test]
fn r2_a_symbolic_zero_retags_rather_than_double_counting() {
    start_recording();
    let (_, counts) = with_session(budget(), || {
        let x = Sym::param(ParamSymbol::of("x"), Probe(1.5));
        let _ = decide("r2_identity", Margin::of(x - x), band());
        let _ = decide("r2_real", Margin::of(x), band());
    });
    let s = take_samples();
    assert_eq!(s.len(), 2, "expected exactly two samples, got {s:?}");
    assert_eq!(s[0].outcome, SampleOutcome::SymbolicZero);
    assert_eq!(s[0].margin, 0.0, "the re-tagged sample lost its margin");
    assert_eq!(s[1].outcome, SampleOutcome::Definite(Sign::Positive));
    assert_eq!(counts.symbolic_zero, 1);
    assert_eq!(counts.numeric, 1);
}

/// **THE RE-TAG IS NOT ADDRESSED TO ITS OWN SAMPLE.** `retag_symbolic_zero`
/// rewrites whatever sits LAST in the thread-local sink, and the base
/// scalar of the driver's own replay lane (`Sym<Interval>`) records
/// nothing at all. A sample pushed by an earlier `Probe` decision on the
/// same thread is therefore re-labelled by a decision that has nothing
/// to do with it — including an `Indeterminate` sample, which is exactly
/// the population k-lint's rule 1 gates on.
///
/// Evidence-only: this row DOCUMENTS the behaviour rather than asserting
/// the fix, so it goes red the day the re-tag is addressed.
#[test]
fn r2_the_retag_rewrites_a_foreign_sample_from_another_scalar() {
    start_recording();
    // An in-band Probe decision: the rule-1 population.
    let tiny = Tol::witness().eps() * 2.0;
    let _ = decide("r2_in_band", Margin::of(Probe(tiny)), band());
    {
        let s = geom_core::k_stats::take_samples();
        assert_eq!(s.len(), 1);
        assert_eq!(
            s[0].outcome,
            SampleOutcome::Indeterminate,
            "the fixture must plant an in-band sample"
        );
    }
    // Now the same sequence, but with a `Sym<Interval>` identity decided
    // afterwards — the driver's own lane, which records nothing.
    start_recording();
    let _ = decide("r2_in_band", Margin::of(Probe(tiny)), band());
    let _ = with_session(budget(), || {
        let x = p("x", 1.0, 2.0);
        let _ = decide("r2_sym_identity", Margin::of(x - x), band());
    });
    let s = take_samples();
    assert_eq!(s.len(), 1, "the interval lane recorded a sample of its own");
    assert_eq!(
        s[0].predicate, "r2_in_band",
        "the surviving sample is the Probe one"
    );
    assert_eq!(
        s[0].outcome,
        SampleOutcome::SymbolicZero,
        "EXPECTED-DEFECT ROW: if this is now `Indeterminate`, the re-tag has \
         been addressed to its own sample and this row should be deleted"
    );
}

// ------------------------------------- the `opaque` door's shared id

/// **`Sym::opaque` gives EVERY untracked value the same node id**
/// (`SymId::UNRECORDED`), so two DIFFERENT untracked values cancel in
/// the normal form and the tier answers `Zero` on a margin that is not
/// zero.
///
/// `Sym::opaque` is the public door for "a lane that legitimately has no
/// expression to track — a bracket handed back by an engine that ran at
/// another scalar" (its own docs), and `AxisScalar::axis for Sym<T>`
/// already calls it. The reserved id is documented as "the mixer never
/// produces it, so it can never collide with a node" — which is true of
/// nodes and false of the OTHER opaque values, all of which are that one
/// id, hence one indeterminate.
///
/// Evidence-only: this row asserts the DEFECT, so it goes red the day
/// the door mints a distinct id per untracked value.
#[test]
fn r2_two_distinct_opaque_values_cancel_to_a_false_theorem() {
    let (s, counts) = with_session(budget(), || {
        let a = Sym::opaque(Interval::from_bounds(1.0, 1.0));
        let b = Sym::opaque(Interval::from_bounds(2.0, 2.0));
        let m = a - b; // the real margin is exactly −1
        (
            sign_of(m),
            geom_core::Bounds::lo(m),
            geom_core::Bounds::hi(m),
        )
    });
    assert_eq!(
        (s.1, s.2),
        (-1.0, -1.0),
        "the numeric channel must see the true margin"
    );
    assert_eq!(
        s.0,
        Ok(Sign::Zero),
        "EXPECTED-DEFECT ROW: if this is now `Negative`, `Sym::opaque` \
         mints distinct ids and the hole is closed"
    );
    assert_eq!(
        counts.symbolic_zero, 1,
        "and the false answer was the SYMBOLIC tier's, not the numeric one's"
    );
}

/// The same hole one level up: an opaque value is indistinguishable from
/// ANY other opaque value, so a comparison between two of them is a
/// theorem the tier has no argument for.
#[test]
fn r2_an_opaque_value_equals_every_other_opaque_value() {
    let (out, _) = with_session(budget(), || {
        let a = Sym::opaque(Interval::from_bounds(-5.0, -5.0));
        let b = Sym::opaque(Interval::from_bounds(7.0, 7.0));
        let c = Sym::opaque(Interval::from_bounds(0.5, 0.5));
        vec![
            ("a−b", sign_of(a - b)),
            ("b−c", sign_of(b - c)),
            ("a·b−b·a", sign_of(a * b - b * a)),
        ]
    });
    for (name, s) in &out {
        assert_eq!(
            *s,
            Ok(Sign::Zero),
            "EXPECTED-DEFECT ROW ({name}): opaque values are no longer one \
             indeterminate"
        );
    }
}
