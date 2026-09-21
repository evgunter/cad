//! **R1's independent probes of DECIDE-3's canonical root (rule G) and
//! the two certified reads**, at the scalar door. Forms the unit did
//! not measure, driven through `Sym<Interval>`.

#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::panic, clippy::float_cmp)]

use geom_core::interval::Interval;
use geom_core::predicate::{Band, Sign};
use geom_core::real::Real;
use geom_core::sym::{SymCounts, with_session_rules};
use geom_core::{Decide, ParamSymbol, Sym, SymBudget, SymRules, Tol};

fn budget() -> SymBudget {
    SymBudget {
        max_terms: 4096,
        max_degree: 128,
    }
}

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

fn pi_(name: &str, lo: f64, hi: f64) -> Sym<Interval> {
    Sym::param(ParamSymbol::of(name), Interval::from_bounds(lo, hi))
}

fn k(v: f64) -> Sym<Interval> {
    Sym::from_f64(v)
}

/// Runs `f` under `rules` and reports the sign of the residual it
/// returns with the counts the session took.
fn probe(rules: SymRules, f: impl FnOnce() -> Sym<Interval>) -> (Result<Sign, String>, SymCounts) {
    let (s, c) = with_session_rules(budget(), rules, || {
        let m = f();
        m.sign_within(band()).map_err(|e| format!("{e:?}"))
    });
    (s, c)
}

fn line(name: &str, r: &(Result<Sign, String>, SymCounts)) {
    println!(
        "  {name:<46} {:?}  sym0={} gated={} reg={} numeric={}",
        r.0, r.1.symbolic_zero, r.1.sign_gated, r.1.registered, r.1.numeric
    );
}

/// **The forms rule G is asked about, and what they key.** Each row is
/// a residual that is ZERO as a real; `Zero` means the two spellings
/// met as one atom, anything else means they did not.
#[test]
fn r1_what_the_door_keys() {
    let rules = SymRules::shipped();
    let rows: Vec<(&str, fn() -> Sym<Interval>)> = vec![
        // sqrt(8) = 2·sqrt(2): the constant content split.
        ("sqrt(8) - 2*sqrt(2)", || {
            k(8.0).sqrt() - k(2.0) * k(2.0).sqrt()
        }),
        // sqrt(12·X) = 2·sqrt(3·X): non-square content over a primitive.
        ("sqrt(12X) - 2*sqrt(3X)", || {
            let x = pi_("x", 1.0, 2.0);
            (k(12.0) * x).sqrt() - k(2.0) * (k(3.0) * x).sqrt()
        }),
        // sqrt(2·X²) = sqrt(2)·|X|.
        ("sqrt(2X^2) - sqrt(2)*|X|", || {
            let x = pi_("x", 1.0, 2.0);
            (k(2.0) * x * x).sqrt() - k(2.0).sqrt() * x.abs()
        }),
        // sqrt(X²) must be |X| and NOT X.
        ("sqrt(X^2) - |X|", || {
            let x = pi_("x", 1.0, 2.0);
            (x * x).sqrt() - x.abs()
        }),
        ("sqrt(X^2) - X        [must NOT be Zero]", || {
            let x = pi_("x", 1.0, 2.0);
            (x * x).sqrt() - x
        }),
        // sqrt(|X|²) must be |X| and not a second abs.
        ("sqrt(|X|^2) - |X|", || {
            let x = pi_("x", -2.0, 2.0);
            (x.abs() * x.abs()).sqrt() - x.abs()
        }),
        // A square inside a NON-square primitive.
        ("sqrt(X^2*Y) - |X|*sqrt(Y)", || {
            let x = pi_("x", 1.0, 2.0);
            let y = pi_("y", 1.0, 2.0);
            (x * x * y).sqrt() - x.abs() * y.sqrt()
        }),
        // Nested root: rule A must still reduce sqrt(sqrt(X))².
        ("sqrt(sqrt(X))^2 - sqrt(X)", || {
            let x = pi_("x", 1.0, 2.0);
            let r = x.sqrt().sqrt();
            r * r - x.sqrt()
        }),
        // Negative content: the sign stays in the primitive part.
        ("sqrt(-2X) - sqrt(2)*sqrt(-X)  [X<0]", || {
            let x = pi_("x", -2.0, -1.0);
            (k(-2.0) * x).sqrt() - k(2.0).sqrt() * (k(-1.0) * x).sqrt()
        }),
        // The tilted document's own shape, at the scalar door.
        ("sqrt(1/X)*sqrt(X) - 1", || {
            let x = pi_("x", 1.0, 2.0);
            let s = x.sqrt();
            (k(1.0) / x).sqrt() * s - k(1.0)
        }),
    ];
    println!("rule G at the scalar door (shipped):");
    for (name, f) in rows {
        line(name, &probe(rules, f));
    }
}

/// **`sqrt(P²/X)·sqrt(X) = |P|`, planted in BOTH minting orders.** The
/// module header claims order can change only WHETHER a root splits,
/// never which atom it splits into. This is the measurement of that.
#[test]
fn r1_the_split_depends_on_minting_order() {
    let root_first = probe(SymRules::shipped(), || {
        let x = pi_("x", 1.0, 2.0);
        let p = pi_("p", 1.0, 2.0);
        let s = x.sqrt(); // source 3 in hand BEFORE the quotient root
        let q = ((p * p) / x).sqrt();
        q * s - p.abs()
    });
    let quotient_first = probe(SymRules::shipped(), || {
        let x = pi_("x", 1.0, 2.0);
        let p = pi_("p", 1.0, 2.0);
        let q = ((p * p) / x).sqrt(); // no sqrt(X) in the session yet
        let s = x.sqrt();
        q * s - p.abs()
    });
    line("sqrt(P^2/X)*sqrt(X)-|P|  root minted first", &root_first);
    line(
        "sqrt(P^2/X)*sqrt(X)-|P|  quotient first   ",
        &quotient_first,
    );
    println!("ORDER-DEPENDENT: {}", root_first.0 != quotient_first.0);
}

/// **The adversary against the side condition's source 3**: the root
/// of `D` is in the session, but `D` takes BOTH SIGNS on the box, so
/// the `sqrt(D)` node has no value on half of it. Does the split
/// happen, and is the answer still an identity of reals there?
#[test]
fn r1_source_three_on_a_straddling_denominator() {
    let straddle = probe(SymRules::shipped(), || {
        let x = pi_("x", -1.0, 1.0);
        let p = pi_("p", 1.0, 2.0);
        let s = x.sqrt(); // mints sqrt(X) though X straddles zero
        let q = ((p * p) / x).sqrt();
        q * s - p.abs()
    });
    line("straddling D, root minted first          ", &straddle);
    // What the two sides are as reals at an interior point of the box
    // where D < 0: the left side is not a real number at all.
    let at = |xv: f64, pv: f64| ((pv * pv / xv).sqrt() * xv.sqrt(), pv.abs());
    println!(
        "  at x=-0.5, p=1.5: lhs={:?} rhs={:?}",
        at(-0.5, 1.5).0,
        at(-0.5, 1.5).1
    );
    println!(
        "  at x= 0.5, p=1.5: lhs={:?} rhs={:?}",
        at(0.5, 1.5).0,
        at(0.5, 1.5).1
    );
}

/// **The two reads.** What `max` of two CONSTANTS is counted as, and
/// whether a read is ever counted `symbolic_zero`.
#[test]
fn r1_the_reads_at_min_max_and_select() {
    let consts = probe(SymRules::shipped(), || {
        // max(1, 1/4) is an exact rational comparison, no value read.
        let m = k(1.0).max(k(0.25));
        m - k(1.0)
    });
    let consts_no_read = probe(SymRules::without_the_reads(), || {
        let m = k(1.0).max(k(0.25));
        m - k(1.0)
    });
    let certified = probe(SymRules::shipped(), || {
        let x = pi_("x", 3.0, 4.0);
        let m = x.max(k(1.0));
        m - x
    });
    let sel = probe(SymRules::shipped(), || {
        let x = pi_("x", 3.0, 4.0);
        let s = Real::select_le_zero(x, k(7.0), k(9.0));
        s - k(9.0)
    });
    let tie = probe(SymRules::shipped(), || {
        // d is certified `<= 0` with equality attainable at an endpoint.
        let x = pi_("x", -1.0, 0.0);
        let s = Real::select_le_zero(x, k(7.0), k(9.0));
        s - k(7.0)
    });
    line("max(1, 1/4) - 1          shipped         ", &consts);
    line(
        "max(1, 1/4) - 1          without_the_reads",
        &consts_no_read,
    );
    line("max(X,1) - X   X in [3,4]                ", &certified);
    line("select(X,7,9) - 9   X in [3,4]           ", &sel);
    line("select(X,7,9) - 7   X in [-1,0] (tie)    ", &tie);
}
