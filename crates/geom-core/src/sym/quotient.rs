//! **Rule E — the quotient's common factor** behind
//! [`SymRules::common_factor`](super::SymRules::common_factor): in the
//! early walk, a form `N / D` is replaced by `N' / D'` with the factor
//! both halves share divided out, and by the CONSTANT `r` when what is
//! left is `r · D' / D'`. Nothing here reads a value: both steps are
//! equalities of rational functions, and the fold is an equality of
//! forms.
//!
//! # Why a normalisation needs it
//!
//! [`Form`](super::form::Form) is a quotient of polynomials with **no
//! common-factor cancellation** — deliberately, because a polynomial
//! GCD is not linear in the expression and a missed cancellation is
//! only a numeric decision (`form`'s own docs). The price is paid by
//! one shape in particular: a NORMALISATION. `Vec3::normalize` is
//! `self / self.norm()`, so a unit vector reaches the DAG as three
//! quotients over one `sqrt(v·v)` atom `A`, and anything built from it
//! carries `A` in both halves — `(a/A)·(b/A) + (c/A)·(d/A)` is
//! `(ab + cd)/A²`, and the next normalisation divides by
//! `sqrt((ab + cd)²/A⁴ + …)`, whose argument is a quotient with `A` to
//! a growing power above and below.
//!
//! Two things go wrong, and this rule is one fix for both.
//!
//! **The degree grows without the expression growing.** Every
//! normalisation multiplies the shared power of `A` in both halves,
//! and every square doubles it. A derived frame's axes are the
//! kernel's already-normalised stored vectors, so a boss placed on one
//! and certified walks that chain three times: measured on the tilted
//! derived frame, the early forms reach total degree 128 in two to
//! fifteen terms and freeze on [`SymBudget::max_degree`](super::SymBudget),
//! and a frozen subtree cancels nothing.
//!
//! **The number one is carried as an opaque atom.** `‖v̂‖` for an
//! already-unit `v̂` is `sqrt(P/P)` for the polynomial `P = v·v` — the
//! literal number 1, spelled as a `sqrt` of a quotient that A0's
//! constant fold cannot read, because neither half is a constant. On
//! the tilted document `P` is a degree-8 polynomial in the frame's
//! parameter, and the atom over it multiplies into every form the
//! certification builds.
//!
//! # The two steps, and why each is unconditional
//!
//! Let the form be `N / D`, `D` not the zero polynomial (a form whose
//! denominator is zero is poison, and poison never reaches this rule).
//!
//! **1 — the shared monomial.** Let `g` be the monomial whose exponent
//! for each indeterminate is the smallest that appears in EVERY term of
//! `N` and in every term of `D` ([`content`]). Then `N = g·N'` and
//! `D = g·D'` exactly, as polynomials, and
//!
//! ```text
//! N / D = (g·N') / (g·D') = N' / D'   wherever g ≠ 0 and D' ≠ 0.
//! ```
//!
//! **2 — the constant ratio.** If `N'` and `D'` have the same monomials
//! and `N' = r·D'` for a rational `r`, then
//!
//! ```text
//! N' / D' = r·D' / D' = r            wherever D' ≠ 0.
//! ```
//!
//! **Why the side conditions are free.** `g` divides `D`, so `g = 0`
//! implies `D = 0`, and `D = r⁻¹·N'`… in both steps the only condition
//! is `D ≠ 0` at the point. A form's denominator is built from
//! nothing but the numerators of the `Inv` nodes above it
//! ([`Form::recip`](super::form::Form::recip) swaps the two halves,
//! [`Form::mul`](super::form::Form::mul) and
//! [`Form::add`](super::form::Form::add) multiply them), and each of
//! those denotes a real the value channel actually DIVIDED by. So a
//! point of the box where `D` vanishes is a point where the scalar
//! divided by zero, and clause 1 of the theorem — the value channel
//! certified the computation on the WHOLE box — has already refused
//! there. On a box clause 1 admits, the two forms denote the same real
//! function at every point, which is exactly what the theorem needs.
//!
//! This is the same posture rule A takes (`sqrt(X)² = X` needs
//! `X ≥ 0`, which holds wherever the atom has a real value) and the
//! same one the quotient normal form itself rests on.
//!
//! # What it cannot do, and what it costs
//!
//! It is NOT a polynomial GCD. A shared factor that is not a monomial
//! and not a constant multiple — `(x + 1)` dividing both halves —
//! stays, which is the conservative direction: a missed cancellation
//! is a numeric decision. `(2x + y)/(x + y)` is not folded, and
//! `x²/x` becomes `x/1` rather than anything smaller.
//!
//! **No step cap, and the reason is structural.** Rules A/B need
//! [`EARLY_STEPS`](super::EARLY_STEPS) because a substitution can
//! REINTRODUCE reducible atoms and grow the form; this rule cannot. It
//! is one pass over the terms of both halves, it allocates no product,
//! and every form it returns has at most as many terms and at most the
//! total degree of the form it was given. Its cost is therefore
//! bounded by the size of a form the budget already bounds, and a step
//! cap beside it would be a claim the code does not keep. Measured on
//! the tilted derived frame it makes a Guided replay CHEAPER, not
//! dearer (the forms it shrinks are the ones the walk then multiplies).

use super::form::{Form, Mono, Poly};
use super::rational::Rat;

/// The monomial every term of `p` is divisible by — per indeterminate,
/// the smallest exponent that appears in all of them. The empty
/// monomial for the zero polynomial and whenever the terms share no
/// indeterminate.
fn content(p: &Poly) -> Mono {
    let mut it = p.terms.keys();
    let Some(first) = it.next() else {
        return Mono::new();
    };
    let mut g: Mono = first.clone();
    for m in it {
        if g.is_empty() {
            break;
        }
        g = g
            .iter()
            .filter_map(|&(id, e)| {
                let other = m.iter().find(|(i, _)| *i == id).map_or(0, |(_, e)| *e);
                let k = e.min(other);
                (k > 0).then_some((id, k))
            })
            .collect();
    }
    g
}

/// The monomial in both `a` and `b`: per indeterminate, the smaller
/// exponent.
fn shared(a: &Mono, b: &Mono) -> Mono {
    a.iter()
        .filter_map(|&(id, e)| {
            let other = b.iter().find(|(i, _)| *i == id).map_or(0, |(_, e)| *e);
            let k = e.min(other);
            (k > 0).then_some((id, k))
        })
        .collect()
}

/// `p` with every term divided by `g` — exact, because `g` divides
/// every term by construction. Monomials stay sorted by id and carry
/// no zero exponent, so the key ordering is preserved and no two terms
/// can collide.
fn divide(p: &Poly, g: &Mono) -> Poly {
    if g.is_empty() {
        return p.clone();
    }
    let mut out = Poly::zero();
    for (m, c) in &p.terms {
        let rest: Mono = m
            .iter()
            .filter_map(|&(id, e)| {
                let d = g.iter().find(|(i, _)| *i == id).map_or(0, |(_, e)| *e);
                (e > d).then_some((id, e - d))
            })
            .collect();
        out.terms.insert(rest, c.clone());
    }
    out
}

/// `p` with every coefficient multiplied by `k`.
fn scale(p: &Poly, k: &Rat) -> Option<Poly> {
    let mut out = Poly::zero();
    for (m, c) in &p.terms {
        out.terms.insert(m.clone(), c.mul(k)?);
    }
    Some(out)
}

/// `r` where `n = r·d` as polynomials, or `None` where no such rational
/// exists. Both maps are sorted by monomial, so one zip decides it.
fn constant_ratio(n: &Poly, d: &Poly) -> Option<Rat> {
    if n.terms.len() != d.terms.len() || d.terms.is_empty() {
        return None;
    }
    let mut ratio: Option<Rat> = None;
    for ((mn, cn), (md, cd)) in n.terms.iter().zip(d.terms.iter()) {
        if mn != md {
            return None;
        }
        let r = cn.mul(&cd.recip()?)?;
        match &ratio {
            None => ratio = Some(r),
            Some(prev) if *prev == r => {}
            Some(_) => return None,
        }
    }
    ratio
}

/// **Rule E over one form** (module docs): divide out the monomial the
/// two halves share, then fold what is left to a constant when the
/// numerator is a rational multiple of the denominator. Never fails —
/// a shape the rule does not reach comes back unchanged — and never
/// grows the form.
pub(super) fn cancel(f: &Form) -> Form {
    if f.poisoned || f.num.is_zero() || f.den.terms.is_empty() {
        return f.clone();
    }
    let g = shared(&content(&f.num), &content(&f.den));
    let (num, den) = if g.is_empty() {
        (f.num.clone(), f.den.clone())
    } else {
        (divide(&f.num, &g), divide(&f.den, &g))
    };
    // The SCALE, canonicalised: both halves multiplied by `1/|s|`,
    // where `s` is the denominator's coefficient at its smallest
    // monomial. Multiplying a quotient's two halves by one non-zero
    // rational is an identity, and it gives the function one
    // representative instead of one per spelling — `(S + 1)/(2·S)` and
    // `(½S + ½)/S` are the same real function and now the same FORM,
    // which is what lets an atom keyed on one meet an atom keyed on
    // the other. The magnitude, not the signed value: flipping the
    // signs would cost `trig::manifestly_nonneg` the syntactic
    // non-negativity rule D's A1 fold reads.
    let (num, den) = match den.terms.iter().next() {
        Some((_, s)) => match s.abs().recip().and_then(|k| {
            let n = scale(&num, &k)?;
            let d = scale(&den, &k)?;
            Some((n, d))
        }) {
            Some(pair) => pair,
            None => (num, den),
        },
        None => (num, den),
    };
    if let Some(r) = constant_ratio(&num, &den) {
        return Form {
            num: Poly::constant(r),
            den: Poly::one(),
            poisoned: false,
            gated: f.gated,
        };
    }
    Form {
        num,
        den,
        poisoned: false,
        gated: f.gated,
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::sym::SymBudget;

    fn budget() -> SymBudget {
        SymBudget {
            max_terms: 4096,
            max_degree: 128,
        }
    }

    fn mono(p: &[(u128, u32)]) -> Mono {
        p.to_vec()
    }

    fn poly(terms: &[(&[(u128, u32)], i128)]) -> Poly {
        let mut p = Poly::zero();
        for (m, c) in terms {
            p.insert(mono(m), Rat::new(*c, 1, 0).unwrap()).unwrap();
        }
        p
    }

    /// `A·x / (A·y)` loses the shared `A`: the degree falls by two and
    /// the function is unchanged.
    #[test]
    fn the_shared_monomial_is_divided_out() {
        let (a, x, y) = (7_u128, 8_u128, 9_u128);
        let f = Form::quotient(
            poly(&[(&[(a, 3), (x, 1)], 1)]),
            poly(&[(&[(a, 2), (y, 1)], 1)]),
        );
        let out = cancel(&f);
        assert_eq!(out.num, poly(&[(&[(a, 1), (x, 1)], 1)]));
        assert_eq!(out.den, poly(&[(&[(y, 1)], 1)]));
    }

    /// `P / P` is the number one, however many terms `P` has — the
    /// shape a re-normalised unit vector's own norm arrives in.
    #[test]
    fn a_polynomial_over_itself_is_one() {
        let p = poly(&[(&[(3, 8)], 5), (&[(3, 1), (4, 2)], -7), (&[], 2)]);
        let f = Form::quotient(p.clone(), p);
        let out = cancel(&f);
        assert!(
            out.num.as_constant().unwrap() == Rat::one()
                && out.den.as_constant().unwrap() == Rat::one(),
            "P/P folds to 1"
        );
        // And the residual against the literal one is then the zero form.
        let resid = out.add(&Form::poly(Poly::one()).neg().unwrap(), budget());
        assert!(resid.unwrap().is_zero(), "‖v̂‖ − 1 is the zero form");
    }

    /// A constant multiple folds to that constant: `3P / P = 3`.
    #[test]
    fn a_constant_multiple_folds_to_the_constant() {
        let p = poly(&[(&[(3, 2)], 1), (&[(4, 1)], 1)]);
        let mut three_p = Poly::zero();
        for (m, c) in &p.terms {
            three_p
                .insert(m.clone(), c.mul(&Rat::new(3, 1, 0).unwrap()).unwrap())
                .unwrap();
        }
        let out = cancel(&Form::quotient(three_p, p));
        assert_eq!(out.num.as_constant().unwrap(), Rat::new(3, 1, 0).unwrap());
    }

    /// **The negative row.** A quotient whose halves are not a rational
    /// multiple of one another is NOT folded — `(2x + y)/(x + y)` is
    /// not the constant 2 and must not become one — and a shared factor
    /// that is a polynomial rather than a monomial stays, which is the
    /// conservative direction.
    #[test]
    fn a_non_constant_ratio_never_folds() {
        let (x, y) = (3_u128, 4_u128);
        let f = Form::quotient(
            poly(&[(&[(x, 1)], 2), (&[(y, 1)], 1)]),
            poly(&[(&[(x, 1)], 1), (&[(y, 1)], 1)]),
        );
        let out = cancel(&f);
        assert_eq!(out.num, f.num, "the numerator is untouched");
        assert_eq!(out.den, f.den, "the denominator is untouched");
        let resid = out
            .add(
                &Form::poly(Poly::constant(Rat::new(2, 1, 0).unwrap()))
                    .neg()
                    .unwrap(),
                budget(),
            )
            .unwrap();
        assert!(
            !resid.is_zero(),
            "(2x + y)/(x + y) − 2 is not the zero form"
        );
        // (x + 1) divides both halves of this one, and the rule leaves
        // it alone: it is not a polynomial GCD.
        let shared_factor = Form::quotient(
            poly(&[(&[(x, 2)], 1), (&[(x, 1)], 1)]),
            poly(&[(&[(x, 1), (y, 1)], 1), (&[(y, 1)], 1)]),
        );
        let out = cancel(&shared_factor);
        assert_eq!(out.num, shared_factor.num, "no polynomial GCD is taken");
    }

    /// **The negative row, second half.** Poison never folds: an
    /// expression with no real value cannot be normalised into one that
    /// has.
    #[test]
    fn poison_is_never_cancelled() {
        let out = cancel(&Form::poison());
        assert!(out.poisoned, "poison survives the rule");
        assert!(!out.is_zero(), "and is never read as a zero");
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod scalar_rows {
    //! The rule at the SCALAR, through the door a document uses: one
    //! theorem row and one negative row.
    use crate::linalg::Vec3;
    use crate::predicate::{Band, Margin, Sign};
    use crate::sym::{SymBudget, SymRules, with_session_rules};
    use crate::{ParamSymbol, Real, Sym};

    fn budget() -> SymBudget {
        SymBudget {
            max_terms: 4096,
            max_degree: 128,
        }
    }

    fn band() -> Band {
        Band::new(1.0e-9, 1.0e-8).unwrap()
    }

    /// Decides `margin` under `rules` and says whether the tier proved
    /// it a theorem.
    fn theorem(rules: SymRules, build: impl FnOnce() -> Sym<f64>) -> bool {
        let (out, counts) = with_session_rules(budget(), rules, || {
            crate::k_stats::decide("sym5_rule_e_row", Margin::of(build()), band())
        });
        matches!(out, Ok(Sign::Zero)) && counts.symbolic_zero == 1
    }

    fn p(name: &str, v: f64) -> Sym<f64> {
        Sym::param(ParamSymbol::of(name), v)
    }

    /// **The theorem row.** An already-normalised vector's own norm is
    /// the number one: `‖v̂‖ − 1` is the ZERO form under rule E, and is
    /// not without it — which is `sqrt(P/P)` folded, the shape a
    /// derived frame's re-normalised axis arrives in.
    #[test]
    fn a_re_normalised_unit_vectors_norm_is_one() {
        let unit = || {
            let v = Vec3::new(p("x", 3.0), p("y", 4.0), p("z", 12.0));
            v.normalize().norm() - Sym::from_f64(1.0)
        };
        assert!(
            theorem(SymRules::shipped(), unit),
            "with rule E, ‖v̂‖ − 1 is a theorem"
        );
        assert!(
            !theorem(SymRules::without_rule_e(), unit),
            "and without it the tier does not reach it — the rule is what took it"
        );
    }

    /// **The negative row.** The rule folds a quotient to the constant
    /// its two halves are in ratio, never to ONE: a vector that is not
    /// a unit vector does not acquire a unit norm, and neither does a
    /// scaled one.
    #[test]
    fn a_non_unit_vector_never_acquires_a_unit_norm() {
        for (name, scale) in [("three", 3.0), ("half", 0.5)] {
            let margin = || {
                let v = Vec3::new(p("x", 3.0), p("y", 4.0), p("z", 12.0));
                // `scale · v̂` has norm `|scale|`, never 1.
                (v.normalize() * Sym::from_f64(scale)).norm() - Sym::from_f64(1.0)
            };
            assert!(
                !theorem(SymRules::shipped(), margin),
                "{name}: ‖{scale}·v̂‖ − 1 is not the zero form and must not decide Zero"
            );
        }
        // And the true statement about the same vector IS reached, so
        // the row above fails for the right reason.
        assert!(
            theorem(SymRules::shipped(), || {
                let v = Vec3::new(p("x", 3.0), p("y", 4.0), p("z", 12.0));
                (v.normalize() * Sym::from_f64(3.0)).norm() - Sym::from_f64(3.0)
            }),
            "‖3·v̂‖ − 3 is a theorem"
        );
    }
}
