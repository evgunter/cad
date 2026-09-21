//! **SYM-12 review R2 — the negative arm driven through the scalar
//! door on forms the unit did not measure.** Every row prints what the
//! tier decides and asserts only what the predicate's own contract
//! says; the soundness check (`sound`) is the shared one.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Real, Sym, SymRules};

use crate::sym_rule_f_rows::{how, one, p, sound};

fn t() -> Sym<f64> {
    p("t", 0.25)
}
fn s() -> Sym<f64> {
    p("s", 0.5)
}

/// The brief's list, each as `abs(X) − (what the identity says)` or
/// `copysign(Y, X) − (what it says)`, with the expected label.
#[test]
fn r2_the_negative_predicates_strictness_at_the_scalar_door() {
    type Case = (&'static str, fn() -> Sym<f64>, bool);
    let cases: [Case; 12] = [
        (
            "abs(−(1 + t²)) − (1 + t²)",
            || {
                let x = -(one() + t().powi(2));
                x.abs() + x
            },
            true,
        ),
        (
            "abs(−t²) − t²  [negated square: must decline]",
            || {
                let x = -t().powi(2);
                x.abs() + x
            },
            false,
        ),
        (
            "abs(−(t² + s²)) − (t² + s²)  [negated sum of squares: must decline]",
            || {
                let x = -(t().powi(2) + s().powi(2));
                x.abs() + x
            },
            false,
        ),
        (
            "abs(−|t|) − |t|  [no strictly negative term of positive indets]",
            || {
                let x = -t().abs();
                x.abs() + x
            },
            false,
        ),
        (
            "abs(−(1 + t²)/(1 + s²)) − (1 + t²)/(1 + s²)",
            || {
                let x = -((one() + t().powi(2)) / (one() + s().powi(2)));
                x.abs() + x
            },
            true,
        ),
        (
            "abs(−(1 + t²)/s²) − (1 + t²)/s²  [D can be zero: clause 1's]",
            || {
                let x = -((one() + t().powi(2)) / s().powi(2));
                x.abs() + x
            },
            true,
        ),
        (
            "abs(−2 + t²) − |−2 + t²|  [mixed signs: must decline]",
            || {
                let x = t().powi(2) - Sym::from_f64(2.0);
                x.abs() + x
            },
            false,
        ),
        (
            "copysign(1, −(1 + t²)) + 1",
            || one().copysign(-(one() + t().powi(2))) + one(),
            true,
        ),
        (
            "copysign(y, −sqrt(1 + t²)) + |y|",
            || {
                let y = s() - t();
                y.copysign(-(one() + t().powi(2)).sqrt()) + y.abs()
            },
            true,
        ),
        (
            "copysign(0, −(1 + t²))",
            || Sym::from_f64(0.0).copysign(-(one() + t().powi(2))),
            true,
        ),
        (
            "abs(−(1 + t²)) − (1 + t²) [the theorem, F shut must NOT reach]",
            || {
                let x = -(one() + t().powi(2));
                x.abs() + x
            },
            true,
        ),
        (
            "abs(−(1 + t²) + 1) + (1 + t²) − 1 = abs(−t²) − t² spelled through a sum",
            || {
                let x = -(one() + t().powi(2)) + one();
                x.abs() + x
            },
            false,
        ),
    ];
    for (what, build, folds) in cases {
        let on = sound(what, how(SymRules::shipped(), build));
        let off = sound(
            &format!("{what} [F shut]"),
            how(SymRules::without_rule_f(), build),
        );
        println!("  => shipped {on} | F shut {off}");
        assert_eq!(on == "theorem", folds, "{what}: shipped");
        if what.starts_with("copysign(0") {
            // A zero FIRST argument is zero whatever the sign argument
            // does (`sym.rs`: "copysign carries `a`'s MAGNITUDE"), at
            // both dials — not rule F's.
            assert_eq!(off, "theorem", "{what}: the at-zero fold, not rule F");
        } else {
            assert_ne!(
                off, "theorem",
                "{what}: rule F is the only rule that could take it"
            );
        }
    }
}

/// **`magnitude` does not know the negative arm.** `copysign(Y, X)`
/// with `X` manifestly positive and `Y` manifestly NEGATIVE mints the
/// `Abs` atom over `Y` (`manifest::magnitude`'s third branch), while
/// an `abs(Y)` node beside it now folds to `−Y` under the same dial —
/// so the two spellings of `|Y|` no longer meet. On `main` both were
/// the same atom and `copysign(Y, X) − abs(Y)` cancelled in the early
/// walk; here the PLAIN walk still cancels it (both opaque), so the
/// bare pair is a theorem either way, and the loss shows on a residual
/// the plain walk cannot close — one that needs rule E's `Q/Q → 1`.
#[test]
fn r2_the_minted_magnitude_of_a_manifestly_negative_y_is_not_the_folded_abs() {
    let y = || -(one() + t().powi(2));
    let x = || one() + s().powi(2);
    let bare = || y().copysign(x()) - y().abs();
    let (on, _) = how(SymRules::shipped(), bare);
    let (off, _) = how(SymRules::without_rule_f(), bare);
    println!("  copysign(Y, X) − abs(Y), Y = −(1 + t²): shipped {on} | F shut {off}");
    // the identity `copysign(Y, X) = |Y| = −Y` for negative Y, positive X:
    let direct = || y().copysign(x()) + y();
    let (on_d, _) = how(SymRules::shipped(), direct);
    println!("  copysign(Y, X) + Y: shipped {on_d}");
    // the same pair through a factor the plain walk cannot clear:
    let q = || one() + t().powi(2);
    let through_e = || {
        let unit = (q() / q()).sqrt();
        y().copysign(x()) * unit - y().abs()
    };
    let (on_e, _) = how(SymRules::shipped(), through_e);
    let (off_e, _) = how(SymRules::without_rule_f(), through_e);
    println!("  copysign(Y, X)·sqrt(Q/Q) − abs(Y): shipped {on_e} | F shut {off_e}");
    // and with the roles the positive arm already handles (Y positive):
    let yp = || one() + t().powi(2);
    let pos_e = || {
        let unit = (q() / q()).sqrt();
        yp().copysign(x()) * unit - yp().abs()
    };
    let (on_p, _) = how(SymRules::shipped(), pos_e);
    println!("  Y positive, same shape: shipped {on_p}");
    assert_eq!(
        on_p, "theorem",
        "the positive Y closes through magnitude's nonneg branch"
    );
    assert_eq!(
        on_d, "theorem",
        "copysign(Y, X) + Y, Y manifestly negative, X positive — the arm's own identity"
    );
    assert_eq!(
        on_e, "theorem",
        "copysign(Y, X)·sqrt(Q/Q) − abs(Y) for a manifestly negative Y: main closed this in the early walk (both spellings one atom); here abs(Y) folds to −Y and copysign mints the atom"
    );
}
