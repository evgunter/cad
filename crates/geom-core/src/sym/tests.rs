use super::*;
use crate::predicate::Margin;
use crate::tolerance::Tol;

/// **The SHIPPED budget**, so these rows exercise the dials a drive
/// actually runs at. They used to use `max_degree: 16` while the
/// shipped value was 128 — a unit test that never touched the
/// configuration it was defending.
fn budget() -> SymBudget {
    SymBudget {
        max_terms: 4096,
        max_degree: 128,
    }
}

fn band() -> Band {
    Band::linear(Tol::witness()).expect("the witness tolerance has a linear band")
}

/// The parameter, at `f64`: a point value with a symbol on it.
fn p(name: &str, v: f64) -> Sym<f64> {
    Sym::param(ParamSymbol::of(name), v)
}

fn decides_zero(m: Sym<f64>) -> bool {
    crate::k_stats::decide("sym_test", Margin::of(m), band()) == Ok(Sign::Zero)
}

#[test]
fn a_literal_difference_is_the_zero_form() {
    let (out, counts) = with_session(budget(), || {
        let x = p("w", 0.37);
        let a = x + Sym::from_f64(2.0) * x;
        let b = Sym::from_f64(3.0) * x;
        decides_zero(a - b)
    });
    assert!(out, "3x written two ways is the same polynomial");
    assert_eq!(counts.symbolic_zero, 1);
    assert_eq!(counts.numeric, 0);
}

/// The shape most identity margins arrive in: a NORM of a vector
/// that is componentwise zero. `sqrt` is an opaque atom, so this
/// only works because an atom over a zero form folds to `f(0)`.
#[test]
fn a_norm_of_a_zero_vector_decides_symbolically() {
    let (out, _) = with_session(budget(), || {
        let t = p("depth", 0.5);
        let one = Sym::from_f64(1.0);
        // (P + t·d) − (P + t·d), componentwise, then the norm.
        let comp = |k: f64| {
            let base = Sym::from_f64(k);
            let a = base + t * one;
            let b = base + one * t;
            a - b
        };
        let (x, y, z) = (comp(3.0), comp(-1.5), comp(0.0));
        let n = (x * x + y * y + z * z).sqrt();
        decides_zero(n)
    });
    assert!(out, "the norm of a componentwise-zero vector is zero");
}

/// A COINCIDENCE at the nominal is not an identity: two segments
/// collinear at `p = 0` only. It never decides symbolically, at any
/// parameter value.
#[test]
fn a_coincidence_at_the_nominal_never_decides_symbolically() {
    for v in [0.0, 1e-12, 0.5] {
        let (_, counts) = with_session(budget(), || {
            let x = p("w", v);
            decides_zero(x * x)
        });
        // At v = 0 the NUMERIC channel answers `Zero`, and that is
        // the point: a coincidence is decided by the enclosure, at
        // the width the enclosure has, and widens with the box. The
        // tier never claims it.
        assert_eq!(counts.symbolic_zero, 0, "at {v}");
        assert_eq!(counts.numeric, 1, "at {v}");
    }
}

/// The cross product of a direction with itself: zero in every
/// component, by different routes through the same symbols.
#[test]
fn a_self_cross_product_is_identically_zero() {
    let (out, _) = with_session(budget(), || {
        let d = [p("a", 0.3), p("b", -0.7), p("c", 0.1)];
        let cross = [
            d[1] * d[2] - d[2] * d[1],
            d[2] * d[0] - d[0] * d[2],
            d[0] * d[1] - d[1] * d[0],
        ];
        cross.into_iter().all(decides_zero)
    });
    assert!(out, "d x d is the zero vector, symbolically");
}

/// π's own identity: `τ − 2π` is zero as a real, and the form says
/// so without reading either constant's value.
#[test]
fn tau_is_two_pi_in_the_form() {
    let (out, _) = with_session(budget(), || {
        decides_zero(<Sym<f64> as Real>::tau() - Sym::from_f64(2.0) * <Sym<f64> as Real>::pi())
    });
    assert!(out);
}

/// The documented limits, pinned as limits: no factoring past the
/// quotient, and no trigonometric identity beyond rule B. `sin(2θ)
/// − 2·sinθ·cosθ` is not the zero form — `sin(2θ)` is an atom of
/// another argument and nothing relates it to the pair — and with
/// the rules OFF the Pythagorean pair is not either, which is what
/// makes `SymRules::none` the pre-algebra tier.
#[test]
fn the_opaque_atoms_are_opaque() {
    let (_, counts) = with_session(budget(), || {
        let x = p("w", 3.0);
        let (s, c) = x.sin_cos();
        let (s2, _) = (x + x).sin_cos();
        decides_zero(s2 - Sym::from_f64(2.0) * s * c)
    });
    // Numerically zero at this point — the numeric channel answers
    // it, as it always did. What is pinned is that the TIER claims
    // nothing: the double-angle identity is outside every rule.
    assert_eq!(counts.symbolic_zero, 0, "no double-angle identity");
    assert_eq!(counts.sign_gated, 0);
    assert_eq!(counts.numeric, 1);
    let (_, counts) = with_session_rules(budget(), SymRules::none(), || {
        let x = p("w", 3.0);
        let (s, c) = x.sin_cos();
        decides_zero(s * s + c * c - Sym::from_f64(1.0))
    });
    assert_eq!(counts.symbolic_zero, 0, "rule B off: both atoms opaque");
    assert_eq!(counts.numeric, 1);
}

/// **Rule B**: the Pythagorean pair of ONE argument form is the zero
/// form, whatever the argument; of two different arguments it is
/// not.
#[test]
fn the_pythagorean_pair_of_one_argument_is_a_theorem() {
    let (out, counts) = with_session_rules(budget(), SymRules::all(), || {
        let x = p("w", 3.0);
        let y = p("h", 0.25);
        let (s, c) = (x * y + Sym::from_f64(2.0)).sin_cos();
        let same = decides_zero(s * s + c * c - Sym::from_f64(1.0));
        let (s2, _) = y.sin_cos();
        let mixed = decides_zero(s2 * s2 + c * c - Sym::from_f64(1.0));
        (same, mixed)
    });
    assert!(out.0, "sin²θ + cos²θ − 1 is the zero form");
    assert_eq!(counts.symbolic_zero, 1, "{counts:?}");
    assert_eq!(counts.sign_gated, 0);
    // The mixed pair decides NUMERICALLY (it is not a theorem, and
    // at this point it is not zero either).
    assert_eq!(counts.numeric, 1, "{counts:?}");
}

/// **Rule A**: an even power of a `sqrt` atom is its argument, so
/// `sqrt(X)·sqrt(X) − X` and `sqrt(X)³ − X·sqrt(X)` are theorems.
#[test]
fn a_square_root_squared_is_its_argument() {
    let (out, counts) = with_session_rules(budget(), SymRules::all(), || {
        let (x, y) = (p("w", 3.0), p("h", 0.25));
        let arg = x * x + y * y + Sym::from_f64(1.0);
        let s = arg.sqrt();
        let a = decides_zero(s * s - arg);
        let b = decides_zero(s * s * s - arg * s);
        let c = decides_zero(s.powi(2) - arg);
        (a, b, c)
    });
    assert_eq!(out, (true, true, true));
    assert_eq!(counts.symbolic_zero, 3, "{counts:?}");
    assert_eq!(counts.sign_gated, 0, "rule A reads no value");
}

/// A parameter with its bracket recorded, at `f64` — the door rule C
/// reads through ([`Sym::param_over`]).
fn p_over(name: &str, v: f64, lo: f64, hi: f64) -> Sym<f64> {
    Sym::param_over(ParamSymbol::of(name), v, lo, hi)
}

/// **Rule C, clause 3: `sqrt(r²) − r` is a theorem CONDITIONAL on
/// `r`'s sign**, and is counted as one. With `r`'s bracket strictly
/// positive the fold takes `sqrt(r²) → r` and the decision is
/// `sign_gated` — never `symbolic_zero`, because it holds on the box
/// and not identically. `abs(r) − r` folds the same way. The
/// residual sits one power below rule A (`sqrt` to the FIRST
/// power), which is why an unconditional rule cannot reach it.
#[test]
fn rule_c_discharges_a_signed_root_as_sign_gated() {
    let (out, counts) = with_session_rules(budget(), SymRules::all(), || {
        let r = p_over("r", 1.25e-3, 1.0e-3, 2.0e-3);
        let sq = decides_zero((r * r).sqrt() - r);
        let abs = decides_zero(r.abs() - r);
        (sq, abs)
    });
    assert_eq!(out, (true, true));
    assert_eq!(
        counts.sign_gated, 2,
        "both are clause-3 theorems: {counts:?}"
    );
    assert_eq!(
        counts.symbolic_zero, 0,
        "and neither is an unconditional one"
    );
    assert_eq!(counts.numeric, 0);
}

/// **Rule C's negative sign**: `sqrt(r²) + r` folds when `r` is
/// DEFINITELY negative (`sqrt(r²) = −r` there), and not otherwise.
#[test]
fn rule_c_folds_the_negated_root_under_a_negative_sign() {
    let (out, counts) = with_session_rules(budget(), SymRules::all(), || {
        let r = p_over("r", -1.25e-3, -2.0e-3, -1.0e-3);
        (
            decides_zero((r * r).sqrt() + r),
            decides_zero(r.abs() + r),
            // The same residual with the WRONG sign is not zero, and
            // the fold does not make it one: it decides numerically.
            decides_zero((r * r).sqrt() - r),
        )
    });
    assert_eq!(out, (true, true, false));
    assert_eq!(counts.sign_gated, 2, "{counts:?}");
    assert_eq!(counts.numeric, 1);
}

/// **Rule C's refusals**: a bracket that STRADDLES zero never folds
/// (the sign is not certified), a parameter with no bracket
/// recorded never folds, and with the rule off the atom stays
/// opaque — in every case the decision is the numeric channel's
/// own and `sign_gated` stays zero.
#[test]
fn rule_c_never_folds_without_a_certified_sign() {
    // Straddling.
    let (out, counts) = with_session_rules(budget(), SymRules::all(), || {
        let r = p_over("r", 1.25e-3, -1.0e-3, 2.0e-3);
        (decides_zero((r * r).sqrt() - r), decides_zero(r.abs() - r))
    });
    assert_eq!(out, (true, true), "numerically zero at the point");
    assert_eq!(
        counts.sign_gated, 0,
        "a straddling bracket folds nothing: {counts:?}"
    );
    assert_eq!(counts.numeric, 2);
    // A zero endpoint is not strictly signed.
    let (_, counts) = with_session_rules(budget(), SymRules::all(), || {
        let r = p_over("r", 1.25e-3, 0.0, 2.0e-3);
        decides_zero((r * r).sqrt() - r)
    });
    assert_eq!(counts.sign_gated, 0, "{counts:?}");
    // No bracket at all (`Sym::param`).
    let (_, counts) = with_session_rules(budget(), SymRules::all(), || {
        let r = p("r", 1.25e-3);
        decides_zero((r * r).sqrt() - r)
    });
    assert_eq!(counts.sign_gated, 0, "no bracket, no read: {counts:?}");
    assert_eq!(counts.numeric, 1);
    // The rule off (the shipped set is measured, not assumed:
    // `SymRules::shipped`'s docs).
    let (_, counts) = with_session_rules(
        budget(),
        SymRules {
            signed_root: false,
            ..SymRules::all()
        },
        || {
            let r = p_over("r", 1.25e-3, 1.0e-3, 2.0e-3);
            decides_zero((r * r).sqrt() - r)
        },
    );
    assert_eq!(counts.sign_gated, 0, "{counts:?}");
    assert_eq!(counts.numeric, 1);
}

/// **A plain theorem is never re-labelled by rule C.** The early
/// walk runs ALONGSIDE the plain form, and a decision the plain form
/// answers is `symbolic_zero` even when a gated fold would also have
/// reached it.
#[test]
fn a_plain_theorem_stays_unconditional_beside_rule_c() {
    let (_, counts) = with_session_rules(budget(), SymRules::all(), || {
        let r = p_over("r", 1.25e-3, 1.0e-3, 2.0e-3);
        // `sqrt(r²)·sqrt(r²) − r²`: rule A reaches it in the early
        // walk too, and the plain form does not — but with r's sign
        // certified the early walk's FIRST fold is C's, so this is
        // gated; the plain-zero row below is the one that must not
        // be.
        decides_zero((r * r).sqrt() - (r * r).sqrt());
    });
    assert_eq!(
        counts.symbolic_zero, 1,
        "x − x is the zero form: {counts:?}"
    );
    assert_eq!(counts.sign_gated, 0);
}

/// **The candidate shape the plate's ceiling has**: `sqrt(X) − R`
/// with `X = R²` as forms where `X` is NOT a syntactic square —
/// `(a + 2r)²` expanded to `a² + 4ar + 4r²` under the root — folds
/// under rule C when `a + 2r` has a certified sign. This is
/// `‖q − c‖ = r` with the endpoint at `c + r·(1, 0)` scaled by 2.
#[test]
fn rule_c_recovers_the_root_of_an_expanded_square() {
    let (out, counts) = with_session_rules(budget(), SymRules::all(), || {
        let a = p_over("a", 0.5, 0.25, 0.75);
        let r = p_over("r", 1.25e-3, 1.0e-3, 2.0e-3);
        let x = a * a + Sym::from_f64(4.0) * a * r + Sym::from_f64(4.0) * r * r;
        decides_zero(x.sqrt() - (a + Sym::from_f64(2.0) * r))
    });
    assert!(out);
    assert_eq!(counts.sign_gated, 1, "{counts:?}");
    assert_eq!(counts.symbolic_zero, 0);
}

/// **Division is IN the normal form** (the quotient of polynomials,
/// not an opaque reciprocal): `(x/y)·y − x` is the zero form.
///
/// This is the shape the kernel's own endpoint-pinning identity
/// arrives in — an extruded strut's carrier is metered in metres and
/// its direction normalized, so the residual is
/// `w·(‖w‖ · ‖w‖⁻¹ − 1)` — and holding the reciprocal opaque leaves
/// exactly that identity undischarged.
#[test]
fn a_reciprocal_cancels_because_the_form_is_a_quotient() {
    let (_, counts) = with_session(budget(), || {
        let x = p("w", 3.0);
        let y = p("h", 2.0);
        decides_zero((x / y) * y - x);
        // And the shape the kernel actually builds: a direction
        // normalized by a norm, re-metered by the same norm.
        let w = [p("a", 0.0), p("b", 0.0), p("d", 1.0)];
        let n = (w[0] * w[0] + w[1] * w[1] + w[2] * w[2]).sqrt();
        let residual = w[2] / n * n - w[2];
        decides_zero(residual)
    });
    assert_eq!(counts.symbolic_zero, 2, "{counts:?}");
}

/// A poisoned expression never decides `Zero` however zero its form
/// is: clause 1 of the theorem, on the value channel.
#[test]
fn a_domain_violation_never_certifies_symbolically() {
    let (out, _) = with_session(budget(), || {
        let neg = Sym::from_f64(-1.0);
        let r = neg.sqrt();
        decides_zero(r - r)
    });
    assert!(!out, "sqrt(-1) - sqrt(-1) is not a certified zero");
}

/// A budget of zero terms is the tier switched off inside the
/// scalar: nothing is asked of the DAG at all.
#[test]
fn a_zero_budget_decides_everything_numerically() {
    let (_, counts) = with_session(SymBudget::none(), || {
        let x = p("w", 0.37);
        decides_zero(x - x)
    });
    assert_eq!(counts.symbolic_zero, 0);
    assert_eq!(counts.numeric, 1);
    assert_eq!(counts.frozen, 0, "nothing is even computed");
}

/// Freezing is SOUND, not silent: a form driven past the term
/// budget decides numerically and the freeze is counted.
#[test]
fn an_over_budget_form_freezes_and_is_counted() {
    let tight = SymBudget {
        max_terms: 2,
        max_degree: 16,
    };
    let (out, counts) = with_session(tight, || {
        let (x, y, z) = (p("a", 1.0), p("b", 2.0), p("c", 3.0));
        let wide = x + y + z;
        // The sum has three terms: over budget, so it freezes into
        // an atom. The DIFFERENCE of two identical frozen atoms is
        // still zero, which is sound — same node, same real.
        let out = decides_zero(wide - wide);
        (out, session_counts())
    });
    assert!(out.0, "identical frozen nodes still cancel");
    assert!(counts.frozen >= 1, "the freeze is counted: {counts:?}");
    assert_eq!(
        out.1.map(|c| c.frozen),
        Some(counts.frozen),
        "the mid-replay door reports the same freezes the session ends with"
    );
}

/// D9: the node ids are content hashes, so two sessions building the
/// same expression in different orders agree bit for bit, and no
/// table is shared between them.
#[test]
fn node_ids_are_bit_identical_across_sessions_and_orders() {
    let build_forward = || {
        let x = p("w", 1.0);
        let y = p("h", 2.0);
        (x * y + x).node().bits()
    };
    let build_backward = || {
        let y = p("h", 2.0);
        let x = p("w", 1.0);
        let m = x * y;
        (m + x).node().bits()
    };
    let (a, _) = with_session(budget(), build_forward);
    let (b, _) = with_session(budget(), build_backward);
    let (c, _) = with_session(SymBudget::none(), build_forward);
    assert_eq!(a, b);
    assert_eq!(a, c, "the id does not depend on the budget");
    // And outside any session at all.
    assert_eq!(a, build_forward());
}

// ------------------------------------------- the registered door

/// The arc-rim SHAPE the door exists for, in miniature: a "radius"
/// `r`, a "rim vector" `v` whose norm the construction guarantees
/// is `r`, and the residual a carrier's endpoint pinning produces —
/// `v·(r/‖v‖ − 1)`, componentwise zero exactly when `‖v‖ = r`.
///
/// `v` is built from parameters so its norm is a genuine `sqrt`
/// atom, and `r` an `abs` atom, so the two are unrelated
/// indeterminates to every rule the tier ships — which is the whole
/// point: this residual is no theorem the tier can reach, and it is
/// the plate's ceiling.
fn rim(vx: f64, vy: f64, rv: f64) -> (Sym<f64>, Sym<f64>, [Sym<f64>; 2]) {
    let (x, y) = (p("vx", vx), p("vy", vy));
    let r = p("r", rv).abs();
    let n = (x * x + y * y).sqrt();
    let one = Sym::from_f64(1.0);
    let scale = r / n - one;
    (n, r, [x * scale, y * scale])
}

/// **How one margin was answered, read from the RECEIPT** — because
/// the answer cannot say. At a point scalar the residual of a true
/// identity is numerically zero as well, so every row below decides
/// `Zero` whatever the tier does; what separates a theorem, an
/// axiom and a band decision is which column the decision landed
/// in, and that is what these rows read.
fn how(m: Sym<f64>) -> &'static str {
    let before = session_counts().expect("inside a session");
    let _ = decides_zero(m);
    let after = session_counts().expect("inside a session");
    if after.registered > before.registered {
        "registered"
    } else if after.sign_gated > before.sign_gated {
        "sign_gated"
    } else if after.symbolic_zero > before.symbolic_zero {
        "theorem"
    } else {
        "numeric"
    }
}

/// **The door discharges the rim residual, and counts it apart.**
/// Without the registration every component is a numeric decision;
/// with it every one is `registered`, and `symbolic_zero` does not
/// move by one.
#[test]
fn a_registered_identity_decides_zero_and_is_counted_apart() {
    let run = |register: bool| {
        with_session(budget(), || {
            let (n, r, resid) = rim(3.0, 4.0, 5.0);
            if register {
                assert_eq!(n.register_equal(r), SymRegistration::Recorded);
            }
            resid.map(how)
        })
    };
    let (how_off, off) = run(false);
    assert_eq!(how_off, ["numeric", "numeric"], "{off:?}");
    assert_eq!(off.registered, 0);
    let (how_on, on) = run(true);
    assert_eq!(how_on, ["registered", "registered"], "{on:?}");
    assert_eq!(on.registered, 2);
    assert_eq!(
        on.symbolic_zero, off.symbolic_zero,
        "the door moves decisions out of `numeric` and out of nothing else"
    );
    assert_eq!(on.numeric + 2, off.numeric);
}

/// **A GATED door form does not discharge**, and this is its pin.
///
/// The door is asked LAST — plain form, early walk, the A/B
/// reduction of the top residual, then the registry. But the walk
/// the door runs is the EARLY one, and with rule C on
/// (`SymRules::signed_root`) that walk can reach zero through a
/// clause-3 SIGN READ, which is a conditional claim rather than an
/// identity. A zero resting BOTH on a constructor's axiom and on a
/// box-wise sign read is two weakenings at once, and the receipt
/// has a column for each and none for the pair — so it falls to the
/// numeric channel, which is the conservative direction.
///
/// The pair below is one shape, twice. `z` and `y` are independent
/// parameters of equal value. Registering `z = sqrt(y·y)` makes
/// `z − y` reach zero only under rule C's fold, and the decision
/// must stay NUMERIC; registering `z = y` makes the same margin the
/// zero form outright, and it is `registered`. Same door, same
/// registry, same margin — the gate is the only difference.
#[test]
fn a_gated_door_form_does_not_discharge() {
    let rules = SymRules {
        signed_root: true,
        ..SymRules::shipped()
    };
    let run = |gated: bool| {
        with_session_rules(budget(), rules, || {
            let y = p("y", 2.0);
            let z = p("z", 2.0);
            let to = if gated { (y * y).sqrt() } else { y };
            assert_eq!(
                z.register_equal(to),
                SymRegistration::Recorded,
                "both registrations are witnessed at the point"
            );
            how(z - y)
        })
    };
    let (gated, gc) = run(true);
    assert_eq!(
        gated, "numeric",
        "a door form that is zero only under a clause-3 sign read is not a \
         discharge: {gc:?}"
    );
    assert_eq!(gc.registered, 0, "{gc:?}");
    let (plain, pc) = run(false);
    assert_eq!(
        plain, "registered",
        "and the same margin through an UNGATED registry is: {pc:?}"
    );
    assert_eq!(pc.registered, 1, "{pc:?}");
}

/// **The value channel is untouched**: every value in the residual
/// is bit-identical with the registration and without it, so
/// `u_ref` is still `v / ‖v‖`. The rejected cheaper spelling is the
/// third row, and it differs in the bits — which is why it is
/// rejected.
#[test]
fn a_registration_changes_no_value_and_the_cheap_spelling_would() {
    let bits = |register: bool| {
        with_session(budget(), || {
            let (n, r, resid) = rim(0.3, 0.4, 0.5000000001);
            if register {
                let _ = n.register_equal(r);
            }
            [resid[0].value.to_bits(), resid[1].value.to_bits()]
        })
        .0
    };
    assert_eq!(bits(false), bits(true), "no value moves");
    // The rejected spelling, at the same numbers: normalizing by the
    // DECLARED radius instead of by the computed norm.
    let cheap = with_session(budget(), || {
        let (x, y) = (p("vx", 0.3), p("vy", 0.4));
        let r = p("r", 0.5000000001).abs();
        let one = Sym::from_f64(1.0);
        let scale = r / r - one;
        [(x * scale).value.to_bits(), (y * scale).value.to_bits()]
    })
    .0;
    assert_ne!(
        cheap,
        bits(true),
        "the `(q - c) / r` spelling changes the numeric channel's bits, which is the \
         whole reason the door exists instead of it"
    );
}

/// **A lying registration is refused, typed, and the decisions stay
/// numeric** — the planted `‖q − c‖ ≡ 2r`.
#[test]
fn a_lying_registration_is_refused_typed() {
    let (how_, counts) = with_session(budget(), || {
        let (n, r, resid) = rim(3.0, 4.0, 5.0);
        let two_r = Sym::from_f64(2.0) * r;
        assert_eq!(
            n.register_equal(two_r),
            SymRegistration::Contradicted,
            "5 is not 10, and the witness says so at the point"
        );
        resid.map(how)
    });
    assert_eq!(how_, ["numeric", "numeric"]);
    assert_eq!(counts.registered, 0, "nothing was recorded: {counts:?}");
    assert_eq!(counts.numeric, 2);
}

/// **A registration made AFTER a consumer decided still discharges
/// the next one** — the door invalidates the memo rather than
/// silently missing (the order clause of `Sym::register_equal`).
#[test]
fn a_registration_after_a_decision_discharges_the_next_one() {
    let (rows, counts) = with_session(budget(), || {
        let (n, r, resid) = rim(3.0, 4.0, 5.0);
        let first = how(resid[0]);
        assert_eq!(n.register_equal(r), SymRegistration::Recorded);
        let second = how(resid[0]);
        // Idempotent, and a repeat invalidates nothing.
        assert_eq!(n.register_equal(r), SymRegistration::Already);
        [first, second]
    });
    assert_eq!(
        rows,
        ["numeric", "registered"],
        "asked before the registration and asked again after it: {counts:?}"
    );
    assert_eq!((counts.registered, counts.numeric), (1, 1));
}

/// **A registration that would close a cycle is refused, typed** —
/// the property `form_in`'s termination argument rests on. `x·x`
/// at `x = 1` has the same VALUE as `x`, so the witness passes and
/// the cycle test is what refuses.
#[test]
fn a_cyclic_registration_is_refused_typed() {
    with_session(budget(), || {
        let x = p("w", 1.0);
        let bigger = x * x;
        assert_eq!(x.register_equal(bigger), SymRegistration::Cyclic);
        // The other direction is not a cycle: `bigger` contains
        // `x`, `x` does not contain `bigger`.
        assert_eq!(bigger.register_equal(x), SymRegistration::Recorded);
    });
}

/// **The door OFF is M10-8's tier**: the same decisions in the same
/// columns, and a registration that records nothing and says so.
#[test]
fn the_door_off_records_nothing_and_reproduces_the_tier() {
    let (rows, counts) = with_session_rules(budget(), SymRules::shipped_without_the_door(), || {
        let (n, r, resid) = rim(3.0, 4.0, 5.0);
        assert_eq!(n.register_equal(r), SymRegistration::Witnessed);
        resid.map(how)
    });
    assert_eq!(rows, ["numeric", "numeric"]);
    assert_eq!((counts.registered, counts.numeric), (0, 2));
}

/// **Outside a session the claim is witnessed and nothing is
/// recorded**, and at a bare scalar the door is a no-op that still
/// answers — the `Real`-level hook's default and its overrides.
#[test]
fn the_hook_is_a_no_op_off_the_symbolic_scalar() {
    assert_eq!(
        <f64 as Real>::register_equal(1.0, 1.0 + 1e-15),
        SymRegistration::Witnessed
    );
    assert_eq!(
        <f64 as Real>::register_equal(1.0, 2.0),
        SymRegistration::Contradicted
    );
    assert_eq!(
        <f64 as Real>::register_equal(f64::NAN, 1.0),
        SymRegistration::Unwitnessed
    );
    // Outside `with_session` there is no table to record in.
    let a = Sym::<f64>::from_f64(2.0);
    let b = Sym::<f64>::from_f64(2.0);
    assert_eq!(a.register_equal(b), SymRegistration::Witnessed);
}

/// **Claim 9 — the axiom agrees with the tier where the tier can
/// reach it.** Written as polynomials the squared identity
/// `‖v‖² − r²` IS a plain-form theorem, with no registration
/// anywhere; written through the root — `sqrt(X)·sqrt(X) − X`, the
/// shape the construction actually produces — it is a theorem of
/// rule A per node, and with the algebra off it is not, because the
/// atom is then opaque. That gap is exactly what the door states
/// for the UNSQUARED identity, and the rows here are what make the
/// axiom consistent with the tier rather than merely asserted.
#[test]
fn the_squared_identity_is_a_plain_form_theorem() {
    let (row, _) = with_session(budget(), || {
        let (x, y) = (p("vx", 3.0), p("vy", 4.0));
        how((x * x + y * y) - (x * x + y * y))
    });
    assert_eq!(row, "theorem", "the squared identity, as polynomials");
    let (row, _) = with_session(budget(), || {
        let (x, y) = (p("vx", 3.0), p("vy", 4.0));
        let n2 = x * x + y * y;
        let root = n2.sqrt();
        how(n2 - root * root)
    });
    assert_eq!(
        row, "theorem",
        "through the root, rule A per node reaches it"
    );
    let (row, _) = with_session_rules(budget(), SymRules::without_the_algebra(), || {
        let (x, y) = (p("vx", 3.0), p("vy", 4.0));
        let n2 = x * x + y * y;
        let root = n2.sqrt();
        how(n2 - root * root)
    });
    assert_eq!(
        row, "numeric",
        "with the algebra off the atom is opaque and the root form stays numeric"
    );
}

/// The arc carrier's SECOND same-object identity, in miniature —
/// the SPAN identity `carrier.eval(θ) = q_to` (M10-9 amendment A1;
/// `sweep::swept::register_span_identity`). The far endpoint is
/// reached by rotating the rim vector through the span, so the
/// residual carries `cos`/`sin` atoms of `4·atan|b|` that no rule
/// relates to the polynomial `q_to − c` is: it is registered per
/// COMPONENT, because the consumer asks
/// `carrier.eval(t1).distance(end)`.
///
/// Answers the three pairs `[eval(θ), q_to, eval(θ) − q_to]`.
fn span(theta: f64, off: f64) -> [[Sym<f64>; 2]; 3] {
    let (vx, vy) = (p("vx", 3.0), p("vy", 4.0));
    let b = p("b", theta);
    let (sn, cs) = (Sym::from_f64(4.0) * b.abs().atan()).sin_cos();
    // The rotated rim vector, as a circle carrier's `eval` builds
    // it, plus the centre.
    let (cx, cy) = (p("cx", 1.0), p("cy", -2.0));
    let (px, py) = (cx + (vx * cs - vy * sn), cy + (vx * sn + vy * cs));
    // The far vertex the construction actually holds, and it is
    // built INDEPENDENTLY — a lamina vertex, not a function of the
    // carrier — which is what makes the registration an axiom
    // rather than a tautology, and what keeps it out of the cyclic
    // arm. Its value is the same real, computed the same way;
    // `off` displaces it into a claim that is FALSE.
    let (s0, c0) = (4.0 * theta.abs().atan()).sin_cos();
    let (qx, qy) = (
        p("qx", 1.0 + (3.0 * c0 - 4.0 * s0) + off),
        p("qy", -2.0 + (3.0 * s0 + 4.0 * c0) + off),
    );
    [[px, py], [qx, qy], [px - qx, py - qy]]
}

/// **The span identity discharges the far endpoint's residual, per
/// component, and a planted lie about it is refused typed.**
#[test]
fn the_span_identity_discharges_and_its_planted_lie_is_refused() {
    let (rows, counts) = with_session(budget(), || {
        let [p_end, q_to, resid] = span(0.4, 0.0);
        for (a, b) in p_end.into_iter().zip(q_to) {
            assert_eq!(a.register_equal(b), SymRegistration::Recorded);
        }
        resid.map(how)
    });
    assert_eq!(rows, ["registered", "registered"], "{counts:?}");
    assert_eq!(counts.symbolic_zero, 0, "no rule reaches this one");
    // The planted lie: the same registration against a far vertex
    // displaced by a geometric amount.
    let (rows, counts) = with_session(budget(), || {
        let [p_end, q_to, resid] = span(0.4, 1.0e-3);
        for (a, b) in p_end.into_iter().zip(q_to) {
            assert_eq!(
                a.register_equal(b),
                SymRegistration::Contradicted,
                "the witness separates a displaced far vertex"
            );
        }
        resid.map(how)
    });
    assert_eq!(rows, ["numeric", "numeric"]);
    assert_eq!(counts.registered, 0, "nothing recorded: {counts:?}");
}

/// **D9**: the registry is content-hash keyed, so two runs of the
/// same leaf register the same ids and count the same — and the
/// node the registrant registers IS the node the consumer built
/// (the same-object condition, testable because ids are content
/// hashes).
#[test]
fn the_registration_is_deterministic_and_the_ids_are_the_consumers() {
    let run = || {
        with_session(budget(), || {
            let (n, r, resid) = rim(3.0, 4.0, 5.0);
            let _ = n.register_equal(r);
            // The registrant's node, recomputed: `Vec3::norm` is
            // `norm_squared().sqrt()` and ids are content hashes, so
            // the consumer's divisor is the very node registered.
            let (x, y) = (p("vx", 3.0), p("vy", 4.0));
            let again = (x * x + y * y).sqrt();
            assert_eq!(again.node().bits(), n.node().bits());
            (resid.map(how), n.node().bits())
        })
    };
    let a = run();
    let b = run();
    assert_eq!(a, b, "identical across repeats");
    assert_eq!(a.0.0, ["registered", "registered"]);
}

// ------------------------------------------------ rule D (M10-10)

/// `atan X` at a point `x`, with `X` a parameter so the atom is a
/// function of a symbol and nothing folds on a constant.
fn atan_of(x: f64) -> (Sym<f64>, Sym<f64>) {
    let x = p("bulge", x);
    (x, x.atan())
}

/// **Rule D decides the closed forms of `sin`/`cos` at `k·atan X`
/// for `k ∈ {1, 2, 3, 4}`** — each against the closed form spelled
/// by hand through the scalar's own ops, at a positive, a negative
/// and a zero-valued `X` — and each is counted a THEOREM: no value
/// read, so `symbolic_zero` and nothing else.
#[test]
fn rule_d_decides_the_multiples_against_their_closed_forms() {
    for xv in [1.0, -0.6, 2.75, 0.0] {
        let (rows, counts) = with_session(budget(), || {
            let (x, phi) = atan_of(xv);
            let one = Sym::from_f64(1.0);
            let s = (one + x * x).sqrt();
            let q = one + x * x;
            let k = |n: f64| Sym::from_f64(n) * phi;
            let mul = |n: f64| Sym::from_f64(n);
            [
                // sin φ = X/S, cos φ = 1/S.
                how(k(1.0).sin_cos().0 - x / s),
                how(k(1.0).sin_cos().1 - one / s),
                // sin 2φ = 2X/(1+X²), cos 2φ = (1−X²)/(1+X²).
                how(k(2.0).sin_cos().0 - mul(2.0) * x / q),
                how(k(2.0).sin_cos().1 - (one - x * x) / q),
                // sin 3φ = (3X − X³)/((1+X²)·S), cos 3φ = (1 − 3X²)/((1+X²)·S).
                how(k(3.0).sin_cos().0 - (mul(3.0) * x - x * x * x) / (q * s)),
                how(k(3.0).sin_cos().1 - (one - mul(3.0) * x * x) / (q * s)),
                // sin 4φ = 4X(1−X²)/(1+X²)², cos 4φ = (1 − 6X² + X⁴)/(1+X²)².
                how(k(4.0).sin_cos().0 - mul(4.0) * x * (one - x * x) / (q * q)),
                how(k(4.0).sin_cos().1 - (one - mul(6.0) * x * x + x * x * x * x) / (q * q)),
            ]
        });
        assert_eq!(rows, ["theorem"; 8], "at X = {xv}: {counts:?}");
        assert_eq!(
            (counts.sign_gated, counts.registered),
            (0, 0),
            "a rule-D zero reads no value and rests on no axiom: {counts:?}"
        );
    }
}

/// **Halves and quarters, on the positive branch**: the identities
/// a half-angle satisfies against the whole angle — `sin φ =
/// 2·sin(φ/2)·cos(φ/2)`, `cos φ = 2·cos²(φ/2) − 1`, `cos(φ/2) =
/// 2·cos²(φ/4) − 1` — and the pushforward's own spelling `cos(sθ)
/// − 1 = −2·sin²(sθ/2)` at `θ = 4·atan X`, `s = i/8`, for every
/// sample of the certifier's schedule, decide `Zero` at a positive
/// and a NEGATIVE `X`. The sign of the half-angle's sine rides in
/// `X` as a form; nothing here reads it.
#[test]
fn rule_d_decides_the_halves_on_the_positive_branch() {
    for xv in [0.8, -1.0, 3.5] {
        let (rows, counts) = with_session(budget(), || {
            let (_, phi) = atan_of(xv);
            let one = Sym::from_f64(1.0);
            let two = Sym::from_f64(2.0);
            let half = phi * Sym::from_f64(0.5);
            let quarter = phi * Sym::from_f64(0.25);
            let mut rows = vec![
                how(phi.sin_cos().0 - two * half.sin_cos().0 * half.sin_cos().1),
                how(phi.sin_cos().1 - (two * half.sin_cos().1 * half.sin_cos().1 - one)),
                how(half.sin_cos().1 - (two * quarter.sin_cos().1 * quarter.sin_cos().1 - one)),
            ];
            // The pushforward's spelling at every schedule sample.
            let theta = Sym::from_f64(4.0) * phi;
            for i in 0..=8 {
                let s = Sym::from_f64(f64::from(i) / 8.0);
                let cos_m1 = -(two * (s * theta * Sym::from_f64(0.5)).sin_cos().0.powi(2));
                rows.push(how(((s * theta).sin_cos().1 - one) - cos_m1));
            }
            rows
        });
        assert!(
            rows.iter().all(|r| *r == "theorem"),
            "at X = {xv}: {rows:?} {counts:?}"
        );
        assert_eq!((counts.sign_gated, counts.registered), (0, 0));
    }
}

/// **The two spellings of one arc meet.** The certifier's carrier
/// sample `cos t`, `sin t` at `t = 4·atan|b|·(i/8)` against the
/// pushforward's `sin(s·θ)` and `1 − 2·sin²(s·θ/2)` at `θ =
/// 4·atan b`, `s = i/8`, for a bulge that is a LITERAL (the circle
/// kernel's `1`, so `|b|` folds under A0) and for a parameter
/// bulge — where `atan|b|` and `atan b` are two atoms and the
/// residual stays numeric, which is the honest limit this rule
/// draws: the turn sign the carrier's axis carries is a `Sign`,
/// not a form.
#[test]
fn rule_d_meets_the_carrier_and_the_pushforward_at_every_sample() {
    let (rows, _) = with_session(budget(), || {
        let b = Sym::from_f64(1.0);
        let theta = Sym::from_f64(4.0) * b.atan();
        let span = Sym::from_f64(4.0) * b.abs().atan();
        let one = Sym::from_f64(1.0);
        let two = Sym::from_f64(2.0);
        (0..=8)
            .map(|i| {
                let s = Sym::from_f64(f64::from(i) / 8.0);
                let t = Sym::zero() + (span - Sym::zero()) * s;
                let (st, ct) = t.sin_cos();
                let sin = (s * theta).sin_cos().0;
                let cos_m1 = -(two * (s * theta * Sym::from_f64(0.5)).sin_cos().0.powi(2));
                (how(st - sin), how(ct - (cos_m1 + one)))
            })
            .collect::<Vec<_>>()
    });
    assert!(
        rows.iter().all(|r| *r == ("theorem", "theorem")),
        "literal bulge: {rows:?}"
    );
    let (rows, _) = with_session(budget(), || {
        let b = p("bulge", 0.7);
        let theta = Sym::from_f64(4.0) * b.atan();
        let span = Sym::from_f64(4.0) * b.abs().atan();
        let s = Sym::from_f64(3.0 / 8.0);
        let (st, _) = (span * s).sin_cos();
        how(st - (s * theta).sin_cos().0)
    });
    assert_eq!(
        rows, "numeric",
        "a parameter bulge: `atan|b|` and `atan b` are two atoms, and no rule here \
         reads the sign that would relate them"
    );
}

/// **Nothing folds at an argument that is not `q·atan(X)`**: an
/// `atan2`, an `atan` plus a constant, a non-dyadic multiple, a
/// product of two `atan`s — each a TRUE identity of the reals that
/// rule D must leave to the numeric channel, because the closed
/// form it states is not the one that holds there.
#[test]
fn rule_d_folds_nothing_at_any_other_argument() {
    let (rows, counts) = with_session(budget(), || {
        let (x, phi) = atan_of(0.9);
        let one = Sym::from_f64(1.0);
        let s = (one + x * x).sqrt();
        let c = Sym::from_f64(0.3);
        [
            // atan2(X, 1) = atan X, but the op is `Atan2`.
            how(x.atan2(one).sin_cos().0 - x / s),
            // sin(φ + c) = sin φ·cos c + cos φ·sin c: `c` is opaque.
            how((phi + c).sin_cos().0 - (x / s * c.sin_cos().1 + one / s * c.sin_cos().0)),
            // A non-dyadic multiple has no closed form here.
            how((phi / Sym::from_f64(3.0)).sin_cos().1 - (phi / Sym::from_f64(3.0)).sin_cos().1),
            // A product of two atans is degree two in the atom.
            how((phi * phi).sin_cos().0 - (phi * phi).sin_cos().0),
        ]
    });
    assert_eq!(rows[0], "numeric", "atan2 never folds: {counts:?}");
    assert_eq!(
        rows[1], "numeric",
        "atan plus a constant never folds: {counts:?}"
    );
    // The last two are `a − a`: the zero form by node identity,
    // which is a theorem whether or not the atom folds — they are
    // here so the shapes are exercised, and what they pin is that
    // no fold PANICS or POISONS on them.
    assert_eq!(&rows[2..], ["theorem", "theorem"]);
}

/// **Rule D's second fold: `atan2(0, N) = 0` for an `N` non-negative
/// BY SYNTAX** — a `sqrt` atom, an even power, and the chart phase's
/// own `r²/sqrt(r²)` with `r = nominal + δ` (a perfect square over a
/// `sqrt` atom) decide `Zero` as theorems; and it NEVER folds at
/// `atan2(0, X)` for a plain parameter, at `atan2(Y, N)` with `Y` a
/// numeric zero that is not the zero form (a coincidence of two
/// parameters at one nominal), or at `atan2(0, 0)` as a form.
#[test]
fn rule_d_folds_atan2_of_the_zero_form_over_a_manifestly_nonnegative_form_and_nothing_else() {
    let (rows, counts) = with_session(budget(), || {
        let x = p("x", 0.37);
        let d = p("delta", 1.0e-5);
        let zero = Sym::zero();
        let r = Sym::from_f64(1.25e-3) + d;
        let r2 = r * r;
        [
            how(zero.atan2(x.sqrt())),
            how(zero.atan2(x * x)),
            how(zero.atan2(r2 / r2.sqrt())),
            how(zero.atan2(x.abs() * x.sqrt() + x * x)),
        ]
    });
    assert_eq!(rows, ["theorem"; 4], "{counts:?}");
    assert_eq!((counts.sign_gated, counts.registered), (0, 0));
    let (rows, _) = with_session(budget(), || {
        let x = p("x", 0.37);
        let y = p("y", 0.0);
        let x2 = p("x2", 0.37);
        let zero = Sym::zero();
        [
            // A plain parameter has no sign the form knows.
            how(zero.atan2(x)),
            // A numeric zero in the first slot is not the zero form.
            how(y.atan2(x.sqrt())),
            // Two parameters equal at the nominal: a coincidence.
            how((x - x2).atan2(x.sqrt())),
            // `atan2(0, 0)` as a form claims nothing.
            how(zero.atan2(zero)),
            // An odd power in a positive-coefficient sum.
            how(zero.atan2(x * x + x)),
        ]
    });
    assert_eq!(rows, ["numeric"; 5]);
}

/// **Rule D's third fold: `sin`/`cos` at an exact half-multiple of
/// π** — `cos π = −1`, `sin π = 0`, `cos(π/2) = 0`, `sin(3π/2) =
/// −1`, `cos(2π) = 1` decide as theorems; `cos(π/3)` and
/// `cos(π + atan X)` never fold.
#[test]
fn rule_d_folds_trig_at_half_multiples_of_pi_and_nothing_else() {
    let (rows, counts) = with_session(budget(), || {
        let pi = Sym::<f64>::pi();
        let one = Sym::from_f64(1.0);
        let half = Sym::from_f64(0.5);
        [
            how(pi.sin_cos().1 + one),
            how(pi.sin_cos().0),
            how((pi * half).sin_cos().1),
            how((pi * Sym::from_f64(1.5)).sin_cos().0 + one),
            how(Sym::tau().sin_cos().1 - one),
        ]
    });
    assert_eq!(rows, ["theorem"; 5], "{counts:?}");
    assert_eq!((counts.sign_gated, counts.registered), (0, 0));
    let (rows, _) = with_session(budget(), || {
        let pi = Sym::<f64>::pi();
        let x = p("x", 0.3);
        // A value-equal spelling on the other side keeps each row
        // a genuine identity that must stay NUMERIC.
        [
            how((pi / Sym::from_f64(3.0)).sin_cos().1 - Sym::from_f64(0.5)),
            how((pi + x.atan()).sin_cos().1 + x.atan().sin_cos().1),
        ]
    });
    assert_eq!(rows, ["numeric"; 2]);
}
