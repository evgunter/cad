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
//! `Form` is a quotient of polynomials with **no
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
//! fifteen terms and freeze on `SymBudget::max_degree`,
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
//! `N` and in every term of `D` (`content`). Then `N = g·N'` and
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
//! implies `D = 0`; in both steps the only condition is `D ≠ 0` at the
//! point. A denominator has FOUR sources and the argument has to cover
//! all four — it is NOT "only what the value channel divided by", and
//! saying that is the mistake this paragraph replaces.
//!
//! **(i) The numerators of the `Inv` nodes above the form.**
//! `Form::recip` swaps the two halves and `Form::mul`/`Form::add`
//! multiply them, so every factor that enters this way denotes a real
//! the value channel actually DIVIDED by. A point of the box where one
//! vanishes is a point where the scalar divided by zero, and clause 1 —
//! the value channel certified the computation on the WHOLE box — has
//! already refused there.
//!
//! **(ii) Rule D's closed-form denominators.** `trig`'s
//! `build_closed_forms` manufactures `Dx·S` and, per halving,
//! `2·Dx·S·c₂` — factors nothing divided by. They are non-zero for
//! another reason, and it is the same range argument rule D's own
//! soundness rests on: `S = sqrt(1 + X²) ≥ 1` for every real `X`, and
//! each `c₂` is `cos(φ/2ʲ)` for `φ = atan X ∈ (−π/2, π/2)` and
//! `j ≥ 1`, so `|φ/2ʲ| ≤ |φ|/2 < π/4` and `c₂ > cos(π/4) = √2/2`.
//! Both bounds are facts about the FUNCTIONS, not about any value.
//! The bound follows from `j ≥ 1` and `atan`'s range ALONE — a larger
//! `MAX_HALVINGS` only makes `c₂` closer to one, so the cap is not
//! what keeps it true; what a `j = 0` would break is, and
//! `trig::fold`'s halving loop never runs a zeroth halving.
//!
//! **(iii) Rule A's substituted denominator.** `algebra`'s
//! `poly_subst_square` puts `D_repl^h` under the result, where `D_repl`
//! is the denominator of the form an atom's square is replaced by —
//! itself a denominator already in the DAG, so a power of something
//! (i) or (ii) covers.
//!
//! **(iv) Rule C's polynomial square root** (`signed::fold`, dial-off
//! in the shipped set): the new denominator `d` satisfies `d² = D` for
//! the old `D`, and the site refuses a zero `d` outright, so `d = 0`
//! iff `D = 0` — again a power of something already covered.
//!
//! On a box clause 1 admits, then, the two forms denote the same real
//! function at every point, which is exactly what the theorem needs.
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
//! **No step cap, and the reason is structural — in two of the three
//! budgets.** Rules A/B need `EARLY_STEPS` because a substitution can
//! REINTRODUCE reducible atoms and grow the form; this rule cannot. It
//! is one pass over the terms of both halves plus one sort, it
//! allocates no product, and every form it returns has at most as many
//! terms and at most the total degree of the form it was given. So in
//! TERMS and in DEGREE the rule is monotone and a step cap beside it
//! would be a claim the code does not keep.
//!
//! **It is NOT monotone in the third budget, the COEFFICIENT width,
//! and the scale step is why.** Dividing both halves by `1/|s|` puts
//! `s`'s odd part into the denominator of every numerator coefficient,
//! so a form whose coefficients were `c` becomes one whose are `c/s` —
//! up to `s`'s bits wider. Past `rational::COEFF_BITS` the ring
//! refuses and the form FREEZES, which is loud and sound but is a
//! cancellation lost. R2's row
//! `rule_e_can_cost_a_theorem_to_the_coefficient_ring`
//! (`geom-core`'s `sym_rule_e_rows`) is the demonstration:
//! an identity the early walk reaches with the rule OFF and not with
//! it on, at `s` = 0.1's 52 bits against a 200-bit `1/q`. "Nothing
//! lost" is therefore a MEASUREMENT on the documents this unit names
//! and never a structural fact;
//! `work/sym/coefficient-ring-width-is-not-monotone-in-reach` carries
//! the class.
//!
//! **What CAN grow is the walk, and it is not the same thing.** A form
//! the rule brings back under the budget is one that no longer freezes
//! to a one-term indeterminate, so its consumers build what the freeze
//! used to cut off: on the two-hole plate the early walk's frozen
//! count falls 48 → 8 and its largest form grows 90 → 288 terms
//! (`editor-core/tests/m10_sym_profile_interval`'s growth guard, which
//! pins both numbers). On the M10-3 slab, where nothing freezes, the
//! largest form only falls, 10 → 6.
//!
//! **The wall-clock cuts both ways, and the sign is the document's.**
//! Where the rule keeps a form small the walk that used to multiply it
//! gets cheaper; where it un-freezes a form the walk builds what the
//! freeze cut off and gets dearer. Measured (R2, `Pinned`, one
//! evaluation, off → on): two stacked derived frames 220.7 → 1.1 s,
//! the tilt about `u` 0.7 → 1.9 s. The five measured documents' own
//! numbers are in `sym.rs`'s rule-E section.
//!
//! # What it reaches, and what it does not
//!
//! The rule is not a class result and the reach is the DOCUMENT's.
//! **Reached**, by execution on the `Guided` lift at half `1e-3`: a
//! frame tilted about `v` (SYM-5's own document), non-unit stored axes
//! (`u = (2,0,0)`, `v = (0,2,t)` — the datum door normalises them, one
//! refusal becomes none, and no false `Zero` appears), and derived
//! frames STACKED two deep — and there the word "reached" needs its
//! lift: under `Pinned` that document certifies at BOTH dials and what
//! the rule buys is the cost (219.4 s off against 1.1 s on), while
//! under `Guided` the rule takes its refusals from four to ONE and the
//! one left is not the tier's — it is the value channel's clause-1
//! `Invalid` on the boss, the fifth site of
//! `work/props/a-widened-derived-placement-normalises-a-straddling-newell-sum`.
//! **NOT reached by THIS rule**: a frame tilted about `u`
//! (`u = (1,0,t)`), which refuses `carrier_endpoint_end` identically
//! with the dial on and off — the rule turns the DEGREE wall into a
//! TERM wall there (the frozen `Powi` kid goes from 606 terms at
//! degree 60 to 440 terms at degree 28, and `440² > MAX_TERMS`) behind
//! the `abs(1/sqrt(…))` and `copysign` atoms a `FaceFrame`'s `u_ref`
//! derivation mints. Those two atoms are `super::manifest`'s (rule F,
//! SYM-8), which folds them and takes that predicate to 33 theorems
//! and no numeric — on top of this rule, not instead of it. And a
//! `FaceFrame` on a REVOLVED body's cap, which neither dial certifies
//! (the refusal moves to `pcurve_loop_continuity` with the rule on,
//! frozen 3 → 902).
//! **Neither**: an in-plane spin (`u = (1,t,0)`, `v = (−t,1,0)`)
//! certifies with the dial off as well as on, so it says nothing
//! either way. A HALF spin (`u = (1,t,0)`, `v = (0,1,0)`) does: one
//! derived rung refuses `carrier_endpoint_end` `[0, 1.43e-1]` with
//! the dial off and certifies with it on (R1).
//! `editor-core/tests/m10_derived_frame_tilted_interval` carries both
//! as evidence rows, and
//! `work/sym/derived-frame-placement-freezes-on-the-symbolic-lane`
//! the numbers.

use super::form::{Form, Mono, Poly, exp_of};
use super::rational::Rat;

/// **The monomial in both `a` and `b`** — per indeterminate, the
/// smaller exponent, and nothing where either lacks it.
fn shared(a: &Mono, b: &Mono) -> Mono {
    a.iter()
        .filter_map(|&(id, e)| {
            let k = e.min(exp_of(b, id));
            (k > 0).then_some((id, k))
        })
        .collect()
}

/// The monomial every term of `p` is divisible by — the same
/// min-exponent merge as [`shared`], folded over the terms. The empty
/// monomial for the zero polynomial and wherever the terms share no
/// indeterminate.
fn content(p: &Poly) -> Mono {
    let mut it = p.monos();
    let Some(first) = it.next() else {
        return Mono::new();
    };
    let mut g: Mono = first.clone();
    for m in it {
        if g.is_empty() {
            break;
        }
        g = shared(&g, m);
    }
    g
}

/// `p` with every term divided by `g`, or `None` where `g` does not
/// divide one — the caller's invariant is that it does (`g` is the
/// shared content of both halves), and refusing rather than dropping
/// an indeterminate costs one comparison and keeps a future caller
/// from getting a silently wrong polynomial.
///
/// **The result is re-sorted, not re-used in place.** Dividing does NOT
/// preserve the monomial order — `[(1,1),(2,1)] < [(1,2)]`, but after
/// dividing by `[(1,1)]` the first is `[(2,1)]` and the second
/// `[(1,1)]`, and the order has flipped — so the terms are collected
/// and sorted ONCE, which is `n log n` against the `n` binary-searched
/// inserts (each an `O(n)` memmove) the first spelling cost.
fn divide(p: &Poly, g: &Mono) -> Option<Poly> {
    if g.is_empty() {
        return Some(p.clone());
    }
    let mut terms: Vec<(Mono, Rat)> = Vec::with_capacity(p.terms().len());
    for (m, c) in p.terms() {
        let mut rest = Mono::with_capacity(m.len());
        for &(id, e) in m {
            let d = exp_of(g, id);
            if e < d {
                return None;
            }
            if e > d {
                rest.push((id, e - d));
            }
        }
        terms.push((rest, c.clone()));
    }
    terms.sort_by(|(a, _), (b, _)| a.cmp(b));
    Poly::from_sorted_terms(terms)
}

/// `r` where `n = r·d` as polynomials, or `None` where no such rational
/// exists. Both halves are sorted by monomial, so one zip decides it.
pub(super) fn constant_ratio(n: &Poly, d: &Poly) -> Option<Rat> {
    if n.terms().len() != d.terms().len() || d.is_zero() {
        return None;
    }
    let mut ratio: Option<Rat> = None;
    for ((mn, cn), (md, cd)) in n.terms().iter().zip(d.terms().iter()) {
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

/// The denominator's coefficient at its SMALLEST monomial — the pivot
/// the scale step divides both halves by.
fn den_pivot(d: &Poly) -> Option<&Rat> {
    d.terms().first().map(|(_, c)| c)
}

/// **Rule E over one form** (module docs): divide out the monomial the
/// two halves share, then fold what is left to a constant when the
/// numerator is a rational multiple of the denominator. Never fails —
/// a shape the rule does not reach comes back unchanged — and never
/// grows the form.
pub(super) fn cancel(f: &Form) -> Form {
    if f.poisoned || f.num.is_zero() || f.den.is_zero() {
        return f.clone();
    }
    let g = shared(&content(&f.num), &content(&f.den));
    // **The no-op path is free.** Most nodes of a walk have nothing to
    // cancel and a denominator that already leads with ±1 — a plain
    // polynomial over `1`, most of all — and the rule runs at EVERY
    // early node, so cloning both halves to hand them straight back is
    // the whole of what it would cost them. Decide it before any
    // allocation: measured on the leaf instrument (release, shipped
    // set) it is 4–5 % — the pad 15.15 → 14.40 s, the plate 0.373 →
    // 0.358, the link 2.53 → 2.43.
    let pivot_is_unit = den_pivot(&f.den).is_some_and(|s| s.abs() == Rat::one());
    if g.is_empty() && pivot_is_unit && constant_ratio(&f.num, &f.den).is_none() {
        return f.clone();
    }
    let (num, den) = if g.is_empty() {
        (f.num.clone(), f.den.clone())
    } else {
        match (divide(&f.num, &g), divide(&f.den, &g)) {
            (Some(n), Some(d)) => (n, d),
            _ => (f.num.clone(), f.den.clone()),
        }
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
    let (num, den) = match den_pivot(&den) {
        Some(s) => match s.abs().recip().and_then(|k| {
            let n = num.scaled(&k)?;
            let d = den.scaled(&k)?;
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
        for (m, c) in p.terms() {
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
