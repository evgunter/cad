//! **R2's independent probes of M10-10's rule D** (PR #2100, frozen head
//! `e904d9691`): the trig-of-`atan` fold, the A1 `atan2` fold, the
//! half-π fold, the zero normalization and the per-node A/B walk, each
//! attacked through the PUBLIC scalar rather than the module's own
//! tests. Every row that expects a THEOREM also checks the residual's
//! `f64` VALUE at a point where the plain-`f64` evaluation is exact
//! enough to see a wrong fold — a fold that decides `Zero` on a
//! residual that is not numerically zero is the one defect these rows
//! exist to catch, and a `theorem` label alone would not see it.
//!
//! Rows marked EVIDENCE print and assert only soundness (never a false
//! theorem); the rest gate.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::predicate::{Band, Margin, Sign};
use geom_core::sym::with_session;
use geom_core::{ParamSymbol, Real, Sym, SymBudget, Tol};

fn budget() -> SymBudget {
    SymBudget {
        max_terms: 4096,
        max_degree: 128,
    }
}

fn band() -> Band {
    Band::linear(Tol::witness()).expect("the witness tolerance has a linear band")
}

/// A parameter at `f64`.
fn p(name: &str, v: f64) -> Sym<f64> {
    Sym::param(ParamSymbol::of(name), v)
}

/// How the tier answered the margin `build` makes inside a FRESH
/// session, and the margin's `f64` value: `theorem` / `registered` /
/// `sign_gated` / `numeric <sign>` / `refused`.
fn how_f64(build: impl FnOnce() -> Sym<f64>) -> (String, f64) {
    let ((out, value), counts) = with_session(budget(), || {
        let m = build();
        (
            geom_core::k_stats::decide("r2_probe", Margin::of(m), band()),
            m.value,
        )
    });
    let label = match out {
        Ok(Sign::Zero) if counts.symbolic_zero == 1 => "theorem".to_owned(),
        Ok(Sign::Zero) if counts.registered == 1 => "registered".to_owned(),
        Ok(Sign::Zero) if counts.sign_gated == 1 => "sign_gated".to_owned(),
        Ok(s) => format!("numeric {s:?}"),
        Err(e) => format!("refused {:?}", e.margin),
    };
    (label, value)
}

/// A theorem whose value at the point is zero to `f64` accuracy — the
/// soundness check a label cannot make.
fn assert_sound_theorem(what: &str, (label, value): (String, f64), tol: f64) {
    assert_eq!(label, "theorem", "{what}: expected a theorem, got {label}");
    assert!(
        value.abs() <= tol,
        "{what}: the form says identically zero but the value at the point is {value:e} \
         (> {tol:e}) — an UNSOUND fold"
    );
}

/// SOUND whatever the label: the value at the point is zero, and the
/// label is a theorem or a numeric zero (the by-hand quotient form can
/// outgrow the budget where the fold's own does not — a missed
/// cancellation, never a wrong one). Answers whether it was a theorem.
fn assert_sound(what: &str, (label, value): (String, f64), tol: f64) -> bool {
    assert!(
        value.abs() <= tol,
        "{what}: value at the point is {value:e} (> {tol:e}) under label {label}"
    );
    assert!(
        label == "theorem" || label == "numeric Zero",
        "{what}: a true identity answered {label}"
    );
    label == "theorem"
}

/// `cos(φ/2ʲ)`, `sin(φ/2ʲ)` spelled by hand through `sqrt` and
/// division only (no trig atom anywhere), then `k` multiples by angle
/// addition — the closed form rule D must agree with, built as a
/// DIFFERENT expression than the fold's own.
fn by_hand(x: Sym<f64>, k: u32, halvings: u32) -> (Sym<f64>, Sym<f64>) {
    let one = Sym::from_f64(1.0);
    let two = Sym::from_f64(2.0);
    let s = (one + x * x).sqrt();
    let (mut c, mut sn) = (one / s, x / s);
    for _ in 0..halvings {
        let ch = ((one + c) / two).sqrt();
        sn = sn / (two * ch);
        c = ch;
    }
    let (mut ck, mut sk) = (one, Sym::from_f64(0.0));
    for _ in 0..k {
        let nc = ck * c - sk * sn;
        let ns = sk * c + ck * sn;
        ck = nc;
        sk = ns;
    }
    (ck, sk)
}

/// **Rule D at every `q = k/2ᵐ` the reader accepts, against a by-hand
/// closed form, at points where `q·atan X` leaves `(−π/4, π/4)` and
/// even `(−π/2, π/2)`** — `k ∈ {1, 2, 3, 5, 7, 31, 32}`, `m ∈ {0, 1,
/// 2, 3}`, `X ∈ {−7, −0.3, 0, 0.9, 10, 1e3}`. Each residual is a
/// theorem AND numerically zero at the point.
#[test]
fn r2_rule_d_agrees_with_a_by_hand_closed_form_at_every_accepted_q() {
    for xv in [-7.0, -0.3, 0.0, 0.9, 10.0, 1.0e3] {
        for m in 0..=3u32 {
            for k in [1u32, 2, 3, 5, 7, 31, 32] {
                let q = f64::from(k) / f64::from(1u32 << m);
                if q > 32.0 {
                    continue;
                }
                let what = format!("sin/cos({k}/{}·atan {xv})", 1u32 << m);
                let sin_row = how_f64(|| {
                    let x = p("x", xv);
                    let (_, sk) = by_hand(x, k, m);
                    (Sym::from_f64(q) * x.atan()).sin_cos().0 - sk
                });
                let t1 = assert_sound(&format!("{what} sin"), sin_row, 1.0e-9);
                let cos_row = how_f64(|| {
                    let x = p("x", xv);
                    let (ck, _) = by_hand(x, k, m);
                    (Sym::from_f64(q) * x.atan()).sin_cos().1 - ck
                });
                let t2 = assert_sound(&format!("{what} cos"), cos_row, 1.0e-9);
                // The NEGATIVE multiple too: sin is odd, cos is even.
                let neg_row = how_f64(|| {
                    let x = p("x", xv);
                    let (ck, sk) = by_hand(x, k, m);
                    let (s, c) = (Sym::from_f64(-q) * x.atan()).sin_cos();
                    (s + sk) + (c - ck)
                });
                let t3 = assert_sound(&format!("{what} negated"), neg_row, 1.0e-9);
                if !(t1 && t2 && t3) {
                    println!("   {what}: sound, but the by-hand spelling froze (numeric zero)");
                }
                // The schedule's own set (`k ≤ 8`, `m ≤ 2`) against the
                // fold's own spelling of the SAME angle by addition —
                // both sides fold, so the residual is small, and it
                // must be a theorem.
                if k <= 8 && m <= 2 {
                    let row = how_f64(|| {
                        let x = p("x", xv);
                        let phi = x.atan();
                        let half = f64::from(1u32 << m);
                        let (sa, ca) = (Sym::from_f64(f64::from(k - 1) / half) * phi).sin_cos();
                        let (sb, cb) = (Sym::from_f64(1.0 / half) * phi).sin_cos();
                        let (s, c) = (Sym::from_f64(q) * phi).sin_cos();
                        (s - (sa * cb + ca * sb)) + (c - (ca * cb - sa * sb))
                    });
                    assert_sound_theorem(&format!("{what} by addition"), row, 1.0e-9);
                }
            }
        }
    }
}

/// **`X` a form with atoms of its own** — `X = sqrt(a) − b·sqrt(c)`,
/// `X = a/b`, `X = |a|·c` — and `X` NEGATIVE: rule D's closed forms
/// are theorems and numerically zero.
#[test]
fn r2_rule_d_with_x_a_form_carrying_atoms() {
    let xs: [(&str, fn() -> Sym<f64>); 3] = [
        ("sqrt(a) - b·sqrt(c)", || {
            p("a", 2.0).sqrt() - p("b", 3.0) * p("c", 5.0).sqrt()
        }),
        ("a/b", || p("a", -2.5) / p("b", 0.7)),
        ("|a|·c", || p("a", -0.8).abs() * p("c", -4.0)),
    ];
    for (name, mk) in xs {
        for (k, m) in [(1u32, 0u32), (2, 0), (4, 0), (1, 1), (3, 2), (7, 3)] {
            let q = f64::from(k) / f64::from(1u32 << m);
            let row = how_f64(|| {
                let x = mk();
                let (ck, sk) = by_hand(x, k, m);
                let (s, c) = (Sym::from_f64(q) * x.atan()).sin_cos();
                (s - sk) * Sym::from_f64(3.0) + (c - ck)
            });
            let what = format!("X = {name}, q = {k}/{}", 1u32 << m);
            // FINDING (R2): for an `X` carrying a `sqrt` atom the fold
            // keys its `sqrt(1 + X²)` atom on the UN-REDUCED product
            // `X·X` (`trig::fold` multiplies the early form of `X` by
            // itself without the per-node A/B reduction), while the
            // by-hand `sqrt(1 + x·x)` node's argument IS reduced per
            // node — two atoms, no cancellation, numeric. Sound; a
            // missed cancellation. The other two `X`s are theorems.
            // Likewise for `X = a/b`: the fold's half-angle atom is
            // `sqrt((b·S + b)/(2·b·S))`, the by-hand one `sqrt((S + 1)/
            // (2·S))` — the quotient form cancels no common factor, so
            // the two are different indeterminates. Sound, a missed
            // cancellation; printed per row.
            let theorem = assert_sound(&what, row, 1.0e-9);
            println!(
                "   {what}: {}",
                if theorem {
                    "theorem"
                } else {
                    "numeric zero (the fold's atoms do not meet the by-hand ones)"
                }
            );
        }
    }
}

/// **Nothing folds at any other shape, and no false theorem is
/// claimed there**: `q` past the caps (`33`, `1/16`), `atan X + atan Y`,
/// `atan(X)/3`, an `atan2`. Each is a TRUE identity of the reals
/// spelled against a folding spelling, so a wrong fold would show as
/// a theorem; the honest answer is numeric.
#[test]
fn r2_rule_d_folds_nothing_past_its_caps_or_at_other_shapes() {
    let rows = [
        (
            "33·atan X vs 32φ + φ",
            how_f64(|| {
                let x = p("x", 0.9);
                let phi = x.atan();
                let (s32, c32) = (Sym::from_f64(32.0) * phi).sin_cos();
                let (s1, c1) = phi.sin_cos();
                (Sym::from_f64(33.0) * phi).sin_cos().0 - (s32 * c1 + c32 * s1)
            }),
        ),
        (
            "atan(X)/16 vs half of atan(X)/8",
            how_f64(|| {
                let x = p("x", 0.9);
                let phi = x.atan();
                let c8 = (phi * Sym::from_f64(0.125)).sin_cos().1;
                let one = Sym::from_f64(1.0);
                (phi * Sym::from_f64(1.0 / 16.0)).sin_cos().1
                    - ((one + c8) / Sym::from_f64(2.0)).sqrt()
            }),
        ),
        (
            "atan X + atan Y",
            how_f64(|| {
                let (x, y) = (p("x", 0.9), p("y", -0.4));
                let (sx, cx) = x.atan().sin_cos();
                let (sy, cy) = y.atan().sin_cos();
                (x.atan() + y.atan()).sin_cos().0 - (sx * cy + cx * sy)
            }),
        ),
        (
            "atan(X)/3",
            how_f64(|| {
                let x = p("x", 0.9);
                let t = x.atan() / Sym::from_f64(3.0);
                let (s, _) = t.sin_cos();
                // sin 3t = 3 sin t − 4 sin³ t, and sin(3t) = sin(atan X) folds.
                x.atan().sin_cos().0 - (Sym::from_f64(3.0) * s - Sym::from_f64(4.0) * s * s * s)
            }),
        ),
        (
            "atan2(X, 1)",
            how_f64(|| {
                let x = p("x", 0.9);
                let one = Sym::from_f64(1.0);
                x.atan2(one).sin_cos().0 - x / (one + x * x).sqrt()
            }),
        ),
    ];
    for (name, (label, value)) in rows {
        assert!(
            label.starts_with("numeric"),
            "{name}: must stay numeric (no closed form is stated there), got {label}"
        );
        assert!(
            value.abs() < 1.0e-12,
            "{name}: the identity is true, value {value:e}"
        );
    }
}

/// **The half-π fold: exact forms only.** Theorems at `k·π/2`,
/// numerically zero; NEVER at `π·x` with `x` a parameter at 1.0, at a
/// literal one ulp off π's coefficient, at `π/3`, or at `π + atan X`.
#[test]
fn r2_half_pi_folds_only_the_exact_form() {
    fn one() -> Sym<f64> {
        Sym::from_f64(1.0)
    }
    fn pi() -> Sym<f64> {
        Sym::<f64>::pi()
    }
    let theorems: [(&str, fn() -> Sym<f64>); 6] = [
        ("cos π + 1", || pi().sin_cos().1 + one()),
        ("sin(3π/2) + 1", || {
            (pi() * Sym::from_f64(1.5)).sin_cos().0 + one()
        }),
        ("cos(−π/2)", || {
            (-(pi() * Sym::from_f64(0.5))).sin_cos().1
        }),
        ("sin(2π)", || (pi() * Sym::from_f64(2.0)).sin_cos().0),
        ("cos(4π) − 1", || {
            (pi() * Sym::from_f64(4.0)).sin_cos().1 - one()
        }),
        (
            "cos(π + 0·x) + 1 (the zero normalization feeds the fold)",
            || (pi() + Sym::from_f64(0.0) * p("x", 0.3)).sin_cos().1 + one(),
        ),
    ];
    for (name, mk) in theorems {
        assert_sound_theorem(name, how_f64(mk), 1.0e-15);
    }
    let never: [(&str, fn() -> Sym<f64>); 4] = [
        ("cos(π·x) + 1 at x = 1.0", || {
            (pi() * p("x", 1.0)).sin_cos().1 + one()
        }),
        ("cos(π·(1 + ulp)) + 1", || {
            (pi() * Sym::from_f64(1.0 + f64::EPSILON)).sin_cos().1 + one()
        }),
        ("cos(π/3) − ½", || {
            (pi() / Sym::from_f64(3.0)).sin_cos().1 - Sym::from_f64(0.5)
        }),
        ("cos(π + atan x) + cos(atan x)", || {
            let x = p("x", 0.3);
            (pi() + x.atan()).sin_cos().1 + x.atan().sin_cos().1
        }),
    ];
    for (name, mk) in never {
        let (label, _) = how_f64(mk);
        assert!(
            label.starts_with("numeric"),
            "{name}: must not fold, got {label}"
        );
    }
}

/// **The zero normalization, at `f64`**: `0/d + x = x` is a theorem
/// where `d` has a value; through the ZERO polynomial it is poison and
/// never a theorem.
#[test]
fn r2_zero_normalization_is_poisoned_through_the_zero_polynomial() {
    let (label, value) = how_f64(|| {
        let (x, y, z) = (p("x", 0.3), p("y", 0.7), p("z", 1.1));
        (x - x) / y + z - z
    });
    assert_sound_theorem("(x−x)/y + z − z", (label, value), 0.0);
    let (label, _) = how_f64(|| {
        let (x, y) = (p("x", 0.3), p("y", 0.7));
        let q = (x - x) / (y - y);
        q - q
    });
    assert!(
        !label.starts_with("theorem"),
        "0/0 − 0/0 has no value and must not be a theorem: {label}"
    );
}

/// **Rules A/B per node on the shapes the linear substitution has to
/// get right**: an ODD power of a `sqrt` atom, a `sqrt` atom in the
/// DENOMINATOR, an argument that is itself a quotient, two atoms at
/// once, and rule B at the fourth power. Theorems, numerically zero;
/// and the non-identities beside them stay numeric.
#[test]
fn r2_ab_per_node_on_odd_powers_quotient_arguments_and_denominators() {
    let rows: [(&str, fn() -> Sym<f64>); 6] = [
        ("sqrt(a/b)³ − (a/b)·sqrt(a/b)", || {
            let (a, b) = (p("a", 1.7), p("b", 0.6));
            let r = (a / b).sqrt();
            r * r * r - (a / b) * r
        }),
        ("1/sqrt(a/b)² − b/a", || {
            let (a, b) = (p("a", 1.7), p("b", 0.6));
            let r = (a / b).sqrt();
            Sym::from_f64(1.0) / (r * r) - b / a
        }),
        ("sqrt(a)²·sqrt(b)² − a·b", || {
            let (a, b) = (p("a", 1.7), p("b", 0.6));
            a.sqrt().powi(2) * b.sqrt().powi(2) - a * b
        }),
        ("sqrt(a/b)⁵/sqrt(a/b)³ − a/b", || {
            let (a, b) = (p("a", 1.7), p("b", 0.6));
            let r = (a / b).sqrt();
            r.powi(5) / r.powi(3) - a / b
        }),
        ("sin⁴ t − (1 − cos² t)²", || {
            let t = p("t", 0.9);
            let (s, c) = t.sin_cos();
            s.powi(4) - (Sym::from_f64(1.0) - c * c).powi(2)
        }),
        ("sin² t/cos² t − (1 − cos² t)/cos² t", || {
            let t = p("t", 0.9);
            let (s, c) = t.sin_cos();
            s * s / (c * c) - (Sym::from_f64(1.0) - c * c) / (c * c)
        }),
    ];
    for (name, mk) in rows {
        let row = how_f64(mk);
        // A/B may answer over the TOP residual (`algebra::reduce`) or
        // per node; either is a theorem and both must be sound.
        assert_sound_theorem(name, row, 1.0e-12);
    }
    let never: [(&str, fn() -> Sym<f64>); 3] = [
        ("sqrt(a²) − a (no even power of the atom)", || {
            let a = p("a", 1.7);
            (a * a).sqrt() - a
        }),
        ("sqrt(a)·sqrt(b) − sqrt(ab)", || {
            let (a, b) = (p("a", 1.7), p("b", 0.6));
            a.sqrt() * b.sqrt() - (a * b).sqrt()
        }),
        ("sqrt(a)² − |a| (the abs atom stands)", || {
            let a = p("a", 1.7);
            a.sqrt().powi(2) - a.abs()
        }),
    ];
    for (name, mk) in never {
        let (label, _) = how_f64(mk);
        assert!(label.starts_with("numeric"), "{name}: {label}");
    }
}

/// EVIDENCE — **the caps skip, never mis-fold**: a residual carrying
/// more distinct `sqrt` atoms than `EARLY_STEPS` (64) and a true
/// identity in every one of them. Whatever the tier answers (a theorem
/// through the top-residual reduction, or numeric), it is sound.
#[test]
fn r2_evidence_a_residual_past_the_step_cap_is_answered_soundly() {
    for n in [8usize, 60, 70, 130] {
        let (label, value) = how_f64(|| {
            let mut acc = Sym::from_f64(0.0);
            for i in 0..n {
                let a = p(&format!("a{i}"), 1.0 + i as f64 * 0.01);
                acc = acc + a.sqrt().powi(2) - a;
            }
            acc
        });
        println!("   {n} atoms: {label}, value {value:e}");
        assert!(value.abs() < 1.0e-9);
        assert!(
            label == "theorem" || label.starts_with("numeric"),
            "{label}"
        );
    }
}

#[cfg(feature = "interval")]
mod over_boxes {
    //! The same folds over BOXES at `Sym<Interval>`: straddling and
    //! negative `X`, and the degenerate boxes A1's argument hands to
    //! clause 1.
    use super::{band, budget};
    use geom_core::predicate::{Margin, Sign};
    use geom_core::sym::with_session;
    use geom_core::{Interval, ParamSymbol, Real, Sym};

    fn over(name: &str, lo: f64, hi: f64) -> Sym<Interval> {
        Sym::param_over(ParamSymbol::of(name), Interval::from_bounds(lo, hi), lo, hi)
    }

    fn how(build: impl FnOnce() -> Sym<Interval>) -> String {
        let (out, counts) = with_session(budget(), || {
            geom_core::k_stats::decide("r2_probe_interval", Margin::of(build()), band())
        });
        match out {
            Ok(Sign::Zero) if counts.symbolic_zero == 1 => "theorem".to_owned(),
            Ok(Sign::Zero) if counts.registered == 1 => "registered".to_owned(),
            Ok(s) => format!("numeric {s:?}"),
            Err(e) => format!("refused {:?}", e.margin),
        }
    }

    /// The by-hand closed form at `Interval`.
    fn by_hand(x: Sym<Interval>, k: u32, halvings: u32) -> (Sym<Interval>, Sym<Interval>) {
        let one = Sym::from_f64(1.0);
        let two = Sym::from_f64(2.0);
        let s = (one + x * x).sqrt();
        let (mut c, mut sn) = (one / s, x / s);
        for _ in 0..halvings {
            let ch = ((one + c) / two).sqrt();
            sn = sn / (two * ch);
            c = ch;
        }
        let (mut ck, mut sk) = (one, Sym::from_f64(0.0));
        for _ in 0..k {
            let nc = ck * c - sk * sn;
            let ns = sk * c + ck * sn;
            ck = nc;
            sk = ns;
        }
        (ck, sk)
    }

    /// **Rule D over boxes**: negative, straddling and wide `X`, every
    /// schedule `q`, `X` a plain box and a form with a `sqrt` atom.
    #[test]
    fn r2_rule_d_decides_over_negative_straddling_and_wide_boxes() {
        for (lo, hi) in [
            (-2.0, -0.5),
            (-1.0, 1.0),
            (0.3, 0.31),
            (5.0, 20.0),
            (-1.0e3, 1.0e3),
        ] {
            for (k, m) in [(1u32, 0u32), (2, 0), (3, 0), (4, 0), (1, 1), (7, 2), (3, 3)] {
                let q = f64::from(k) / f64::from(1u32 << m);
                let plain = how(|| {
                    let x = over("x", lo, hi);
                    let (ck, sk) = by_hand(x, k, m);
                    let (s, c) = (Sym::from_f64(q) * x.atan()).sin_cos();
                    (s - sk) + (c - ck) * Sym::from_f64(2.0)
                });
                // The by-hand quotient form outgrows the budget past the
                // schedule's set (`k ≤ 4`, `m ≤ 2`); printed there.
                if k <= 4 && m <= 2 {
                    assert_eq!(
                        plain,
                        "theorem",
                        "X over [{lo}, {hi}], q = {k}/{}",
                        1u32 << m
                    );
                } else {
                    println!(
                        "   X over [{lo}, {hi}], q = {k}/{}: by hand {plain}",
                        1u32 << m
                    );
                }
                // The same angle by addition, both sides folded.
                let added = how(|| {
                    let x = over("x", lo, hi);
                    let phi = x.atan();
                    let half = f64::from(1u32 << m);
                    let (sa, ca) = (Sym::from_f64(f64::from(k - 1) / half) * phi).sin_cos();
                    let (sb, cb) = (Sym::from_f64(1.0 / half) * phi).sin_cos();
                    let (s, c) = (Sym::from_f64(q) * phi).sin_cos();
                    (s - (sa * cb + ca * sb)) + (c - (ca * cb - sa * sb))
                });
                // The schedule's set is `m ≤ 2`; at three halvings the
                // reduction does not close the ring (printed, and it
                // is never a wrong answer: the numeric channel refuses).
                if m <= 2 {
                    assert_eq!(
                        added,
                        "theorem",
                        "by addition, X over [{lo}, {hi}], q = {k}/{}",
                        1u32 << m
                    );
                } else {
                    println!(
                        "   X over [{lo}, {hi}], q = {k}/{} by addition: {added}",
                        1u32 << m
                    );
                    assert!(
                        !added.starts_with("numeric Positive")
                            && !added.starts_with("numeric Negative")
                    );
                }
                let with_atoms = how(|| {
                    let x = over("x", lo, hi).sqrt() - over("y", 0.1, 3.0);
                    let (ck, sk) = by_hand(x, k, m);
                    let (s, c) = (Sym::from_f64(q) * x.atan()).sin_cos();
                    (s - sk) + (c - ck) * Sym::from_f64(2.0)
                });
                if lo > 0.0 {
                    // See the `f64` row: two `sqrt(1 + X²)` atoms when
                    // `X` carries a `sqrt` atom — never a wrong answer.
                    assert!(
                        with_atoms == "theorem"
                            || with_atoms.starts_with("numeric Zero")
                            || with_atoms.starts_with("refused"),
                        "X = sqrt(x) − y, x over [{lo}, {hi}], q = {k}/{}: {with_atoms}",
                        1u32 << m
                    );
                    if k == 1 && m == 0 {
                        println!("   X = sqrt(x) − y over [{lo}, {hi}], q = 1: {with_atoms}");
                    }
                } else {
                    assert!(
                        with_atoms.starts_with("refused"),
                        "sqrt over a box reaching zero is clause 1's: {with_atoms}"
                    );
                }
            }
        }
    }

    /// **A1 over the degenerate boxes the argument hands to clause 1**:
    /// wherever `N`'s box reaches zero the value channel refuses BEFORE
    /// the fold, for every syntactic class the fold accepts — an even
    /// power, an `abs` atom, a `sqrt` atom squared, a perfect square,
    /// a positive-coefficient sum.
    #[test]
    fn r2_a1_the_degenerate_boxes_refuse_through_clause_one_for_every_class() {
        let rows: [(&str, fn() -> Sym<Interval>); 6] = [
            ("atan2(0, x²), x ∋ 0", || {
                let x = over("x", -1.0, 1.0);
                Sym::zero().atan2(x * x)
            }),
            ("atan2(0, |x|), x ∋ 0", || {
                Sym::zero().atan2(over("x", -1.0, 1.0).abs())
            }),
            ("atan2(0, sqrt(x)²), x ∋ 0", || {
                let x = over("x", 0.0, 1.0);
                Sym::zero().atan2(x.sqrt() * x.sqrt())
            }),
            ("atan2(0, (x−1)²), x ∋ 1", || {
                let d = over("x", 0.5, 1.5) - Sym::from_f64(1.0);
                Sym::zero().atan2(d * d)
            }),
            ("atan2(0, x² + y²), x, y ∋ 0", || {
                let (x, y) = (over("x", -1.0, 1.0), over("y", -1.0, 1.0));
                Sym::zero().atan2(x * x + y * y)
            }),
            ("atan2(x − x, y²), y ∋ 0", || {
                let (x, y) = (over("x", -1.0, 1.0), over("y", -1.0, 1.0));
                (x - x).atan2(y * y)
            }),
        ];
        for (name, mk) in rows {
            let label = how(mk);
            assert!(
                label.starts_with("refused"),
                "{name}: N's box reaches zero; clause 1 must refuse before the form: {label}"
            );
        }
        // And where N's box is POSITIVE every class folds.
        let folds: [(&str, fn() -> Sym<Interval>); 4] = [
            ("atan2(0, x²)", || {
                let x = over("x", 0.5, 1.0);
                Sym::zero().atan2(x * x)
            }),
            ("atan2(x − x, sqrt(y))", || {
                let (x, y) = (over("x", -1.0, 1.0), over("y", 0.5, 1.0));
                (x - x).atan2(y.sqrt())
            }),
            ("atan2(0, (x−1)²)", || {
                let d = over("x", 1.5, 2.0) - Sym::from_f64(1.0);
                Sym::zero().atan2(d * d)
            }),
            ("atan2(0, |x|·sqrt(y) + z²)", || {
                let (x, y, z) = (
                    over("x", -2.0, -1.0),
                    over("y", 0.5, 1.0),
                    over("z", 1.0, 2.0),
                );
                Sym::zero().atan2(x.abs() * y.sqrt() + z * z)
            }),
        ];
        for (name, mk) in folds {
            assert_eq!(how(mk), "theorem", "{name}");
        }
        // EVIDENCE: `1 + x²` over a straddling `x` — true value ≥ 1, but
        // the enclosure of `x·x` over `[−1, 1]` decides whether clause 1
        // sees the origin. Printed, never a theorem-or-else assertion.
        println!(
            "   atan2(0, 1 + x²), x ∈ [−1, 1]: {}",
            how(|| {
                let x = over("x", -1.0, 1.0);
                Sym::zero().atan2(Sym::from_f64(1.0) + x * x)
            })
        );
    }

    /// **A numerical coincidence in `Z`'s place never folds, over a
    /// box**: two parameters equal at the nominal, and a plain `X`.
    #[test]
    fn r2_a1_a_coincidence_or_a_plain_parameter_never_folds_over_a_box() {
        let coincidence = how(|| {
            let (x, y) = (over("x", 0.3, 0.4), over("y", 0.3, 0.4));
            (x - y).atan2(over("z", 0.5, 1.0).sqrt())
        });
        assert_ne!(
            coincidence, "theorem",
            "x − y at equal boxes is a coincidence"
        );
        let plain = how(|| Sym::zero().atan2(over("x", 0.5, 1.0)));
        assert_ne!(
            plain, "theorem",
            "a plain parameter has no sign the form knows"
        );
    }

    /// **The zero normalization where `d` can vanish on the box**:
    /// `(x − x)/d + z − z` is refused where `d ∋ 0` (clause 1 — the
    /// division has no certified value) and a theorem where it does
    /// not; `(x − x)/d · w` likewise.
    #[test]
    fn r2_zero_normalization_over_a_denominator_that_can_vanish() {
        let refused = how(|| {
            let (x, d, z) = (
                over("x", 0.0, 1.0),
                over("d", -1.0, 1.0),
                over("z", 2.0, 3.0),
            );
            (x - x) / d + z - z
        });
        assert!(
            refused.starts_with("refused"),
            "d ∋ 0 is clause 1's: {refused}"
        );
        let refused = how(|| {
            let (x, d, w) = (
                over("x", 0.0, 1.0),
                over("d", -1.0, 1.0),
                over("w", 2.0, 3.0),
            );
            (x - x) / d * w
        });
        assert!(
            refused.starts_with("refused"),
            "d ∋ 0 is clause 1's: {refused}"
        );
        let ok = how(|| {
            let (x, d, z) = (
                over("x", 0.0, 1.0),
                over("d", 0.5, 1.0),
                over("z", 2.0, 3.0),
            );
            (x - x) / d + z - z
        });
        assert_eq!(ok, "theorem");
    }

    /// **A/B per node where the shared denominator vanishes on part of
    /// the box**: `sqrt(a/b)² − a/b` with `b ∋ 0` is refused, never a
    /// theorem; with `b` positive it is one.
    #[test]
    fn r2_ab_per_node_over_a_vanishing_shared_denominator() {
        let refused = how(|| {
            let (a, b) = (over("a", 1.0, 2.0), over("b", -1.0, 1.0));
            let r = (a / b).sqrt();
            r * r - a / b
        });
        assert!(refused.starts_with("refused"), "{refused}");
        let ok = how(|| {
            let (a, b) = (over("a", 1.0, 2.0), over("b", 0.5, 1.0));
            let r = (a / b).sqrt();
            r * r * r - (a / b) * r
        });
        assert_eq!(ok, "theorem");
    }
}
