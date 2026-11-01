//! **The certified real roots of a depressed monic quartic**: the sign
//! ladder [`depressed_quartic_roots`] (the half-angle ladder,
//! [`crate::half_angle`], is one caller), the cancellation-free
//! [`quadratic_roots`] its factors (and every quadratic arm) are solved by,
//! and the `sqrt`-chain cube root its Ferrari resolvent is built on.

use geom_core::k_stats::decide;
use geom_core::{Band, Decide, Margin, Sign};

use crate::roots::{RootRefusal, Roots, Unsettled};

/// **The two roots of `a·t² + 2b·t + c`**, `[(−b − √disc)/a,
/// (−b + √disc)/a]` with `disc = b² − a·c` decided positive by the
/// caller, each spelled so that no sum in it cancels.
///
/// Of the two numerators `−b ∓ √disc`, one adds magnitudes and the
/// other cancels when `a·c` is small beside `b²`; divided by a lead `a`
/// that is only decided nonzero, the cancelled one is wrong by
/// `≈ ulp·|b|/a`, far above the band on a line all but parallel to a
/// wall's axis. Since `(−b − √disc)(−b + √disc) = a·c`, that root is
/// also `c` over the other numerator, which adds magnitudes, and that is
/// the spelling taken for it. Which numerator adds magnitudes is the
/// sign of `b`, read by [`geom_core::Real::select_le_zero`] as a value
/// selection, not a decision: both spellings name the same root, so
/// where an enclosure of `b` straddles zero the hull of the two is as
/// narrow as either, and neither denominator nears zero there (it is
/// `√disc` plus a sliver).
pub fn quadratic_roots<T: Decide>(a: T, b: T, c: T, disc: T) -> [T; 2] {
    let root = disc.max(T::zero()).sqrt();
    // `−b − √disc` adds magnitudes for `b > 0`, `√disc − b` for `b ≤ 0`.
    let (minus, plus) = (T::zero() - b - root, root - b);
    [
        b.select_le_zero(c / plus, minus / a),
        b.select_le_zero(plus / a, c / minus),
    ]
}

/// What [`depressed_quartic_roots`] answers: a certified count of zero,
/// or a certified count (2 or 4) and that many roots, unordered, in the
/// `y` variable. A monic quartic is never a zero constant, so it has no
/// [`Roots::On`] arm.
pub type QuarticRoots<T> = Roots<T, 4>;

/// A real cube root, built from `sqrt` alone — and a TRUNCATED one, with
/// the truncation covered by a magnitude argument rather than by
/// containment.
///
/// The scalar trait has no `cbrt`, and this arm needs one for exactly
/// one branch (below). Rather than reach for an iteration the `Interval`
/// scalar cannot follow — a Newton step seeded from a float returns a
/// tight interval that need not contain the answer — take the identity
/// `1/3 = Σ_{k≥1} 4^{-k}`: `x^{1/3} = Π_{k≥1} x^{4^{-k}}`, and each
/// factor is two more square roots of the last one.
///
/// # What the enclosure covers, and what it does not
///
/// Truncating at `n` terms computes `x^{(1−4^{-n})/3}`. The
/// construction is a FIXED composition of `sqrt` — monotone, total —
/// so the interval instantiation encloses **that** power by composition
/// of containments. It does not enclose `x^{1/3}`: the gap between the
/// two is a systematic BIAS, identical on every run, and no interval
/// arithmetic will widen to cover it.
///
/// So the gap is bounded by magnitude instead. At `n = 27` the exponent
/// is short of a third by `4^{-27}/3 ≈ 1.8e-17`, and the relative gap is
/// `|x^{-4^{-27}/3} − 1| ≈ 1.8e-17·|ln x|`. The resolvent's argument is
/// a length⁶, so `|ln x| ≤ 83` even at the extreme `x = 1e36` a
/// kilometre-scale torus could reach — a relative gap under `1.5e-15`,
/// which `cbrt_rows::the_truncation_is_a_bias_not_a_containment`
/// reports directly (a printed census, not a gate: the bound above is
/// held by `cbrt_rows::the_chain_tracks_the_true_cube_root`). Against the ε-scale margins every
/// decision here is metered at, that is fifteen orders of headroom at
/// the coarsest shipped row and six at the finest.
///
/// **And the certified COUNT never touches this function.** The count is
/// read off `Δ`, `p` and `D` — exact sign algebra on the coefficients —
/// before any root is constructed, so the truncation can perturb a root
/// VALUE and can never move the count the arm answers on. What a
/// perturbed root value can do is shift a hit by that relative gap,
/// which the trim and incidence predicates then decide with their own
/// margins.
fn cbrt<T: geom_core::Real>(x: T) -> T {
    let mut u = x.abs();
    let mut acc = T::one();
    for _ in 0..27 {
        u = u.sqrt().sqrt();
        acc = acc * u;
    }
    acc.copysign(x)
}

/// The LARGEST real root of `z³ + c2·z² + c1·z + c0`, given whether the
/// cubic has three real roots.
///
/// Which branch to take is not decided here: the caller has already
/// decided the QUARTIC's discriminant, and the Ferrari resolvent shares
/// it, so `three_real` is that sign rather than a second decision on the
/// same quantity.
///
/// * three real roots — Viète's trigonometric form, whose `−P/3` is
///   positive exactly when three real roots exist, so the square root is
///   real by the branch's own premise, and whose `k = 0` member is the
///   largest. The caller reaches it only with four real quartic roots,
///   where the resolvent's roots are squares of real sums, so none is
///   negative and the largest is at least a third of their sum `−c2`:
///   removing the shift `c2/3` then adds magnitudes rather than
///   cancelling them;
/// * one real root — Cardano's, on [`cbrt`], assembled so that no
///   cancellation reaches the root's RELATIVE accuracy (below).
///
/// # The one-real-root branch keeps the root's relative accuracy
///
/// With `x³ + P x + Q` the depressed cubic and `s = c2/3` its shift, the
/// real root is `z = x − s` with `x = a + b`, where `a³`, `b³` are
/// Cardano's two radicands `−Q/2 ± √(Q²/4 + P³/27)` and `ab = −P/3`. Two
/// subtractions in that form can lose every digit, and the resolvent
/// reaches both:
///
/// * **Inside a radicand.** One of the two cancels whenever `P³/27` is
///   small beside `Q²/4`. Only `|Q|/2 + √(…)`, whose terms are both
///   non-negative, goes through [`cbrt`], as `A`; its partner's
///   magnitude comes from `|ab| = |P|/3`, as `B = P/(3A)`, and
///   `x = −sgn Q·(A − B)`.
/// * **Against the shift.** When the real root is small beside the other
///   two, `x` and `s` agree in their leading digits and `x − s` keeps
///   only their rounding. That is the resolvent of every ray whose odd
///   coefficient `q̂` is small but decided nonzero (a ray all but
///   perpendicular to the axis, or passing near the midplane): its real
///   root is `≈ q̂²/c1`, and `x − s` would multiply the relative error of
///   `x` by `|s|/z` — a factor the [`cbrt`] magnitude argument does not
///   cover.
///
/// # Where `Q`'s sign goes, and why its zero is harmless
///
/// `Q = 0` is not a degenerate pose: it is a codimension-one surface of
/// GENERIC rays (there the real root is simply `z = −s`), and at the
/// `Interval` scalar its neighbourhood is where an enclosure of `Q`
/// straddles zero and `copysign` answers with the hull of both signs.
/// So the sign is transferred only onto `A − B`, which VANISHES at
/// `Q = 0` — there `A³ = √(P³/27)`, so `A = √(P/3) = B` — making
/// `x = (A − B)·sgn(−Q)` continuous across the surface, and its
/// straddling enclosure `[−|A − B|, |A − B|]` as narrow as `x` itself
/// (`≈ |Q|/P`). The textbook stable form transfers it onto `√(…)`
/// instead, which does not vanish there: the radicand straddles zero,
/// `P/(3A)` becomes the whole line, and the arm escalates a ray it has
/// every digit for (`r1_the_q_zero_surface_certifies_on_both_sides`). The
/// quadratic arms escape the same hazard by selecting, per root, between
/// two spellings of that one root ([`quadratic_roots`]).
///
/// `A − B` itself cancels when `x` is small beside `A`, which costs an
/// ABSOLUTE error `≈ ulp·A`, and only for `P > 0`: for `P < 0`, `B < 0`
/// and `A − B` is a sum of magnitudes. Where `x − s` carries weight
/// below (`|z| ≥ |w|`), that absolute error is a relative one of the
/// root: `B > 0` makes `|w|² ≥ ¾(A + B)² ≥ ¾A²`, so `|z| ≥ (√3/2)·A`.
///
/// # The shift, by a fraction rather than a blend
///
/// The product of the three roots is `−c0`, and the complex pair `w, w̄`
/// has `|w|² = (x/2 + s)² + ¾(A + B)²` — a sum of squares, whose
/// `x/2 + s` cancels only when the real root is LARGE (`z ≈ −3s`). So
/// `−c0/|w|²` is accurate exactly where `d = x − s` is not, and
/// `z + 2·Re w = −3s` makes at least one of `|z|`, `|w|` as large as
/// `|s|`. The two are weighted by `τ = d²/(d² + |w|²)`, small precisely
/// when `z` is, and since `(−c0/|w|²)·|w|² = −c0`,
///
/// ```text
/// τ·d + (1 − τ)·(−c0/|w|²) = (d³ − c0) / (d² + |w|²)
/// ```
///
/// which takes the well-conditioned form without branching on a value
/// (this scalar cannot) and without an interval weight: spelled as a
/// blend, `τ`'s enclosure is as wide as `d`'s relative width even at
/// `τ ≈ 1`, and multiplies `|z|` twice. The denominator is positive,
/// `|w|² ≥ ¾(A + B)²`.
///
/// # What the `Interval` instantiation encloses
///
/// Not the root. Every step after [`cbrt`] is a fixed composition, so
/// the enclosure holds the value this function takes with `cbrt`'s
/// truncated power `x^{(1−4^{-27})/3}` in place of the cube root — a
/// bias of relative size `δ ≤ 1.5e-15` in `A` (`cbrt`'s own doc) that no
/// interval widens to cover. The construction's job is that `δ` reaches
/// `z` as a RELATIVE error of its own order rather than multiplied by
/// `|s|/z`: through `d` it moves `x` by `δ·(A + B)`, which the weight
/// `d²/(d² + |w|²)` scales to at most `δ·|z|/√3`; through `|w|²` it
/// moves a sum of squares by `≤ 2δ` relative. That is what
/// `r1_the_near_perpendicular_ray_keeps_its_roots` checks against the
/// pose's exact roots — a biased enclosure that missed them would fail
/// it — and at 1e-15 against the 1e-9 width that row allows, the bias is
/// not what an enclosure's width is made of.
///
/// # `P = Q = 0`
///
/// There the radicand `|Q|/2 + √(…)` is exactly zero, `A = 0`, and
/// `B = P/(3A)` is `0/0`: the f64 result is NaN. It is not reached: the
/// cubic then has a TRIPLE root, i.e. a zero discriminant, which this
/// branch's caller has decided definitely negative. And if a rounding
/// ever did produce it, the NaN is loud where it lands: the caller's
/// `bool_ray_torus_split_lead` reads a NaN margin as `Invalid` and
/// escalates, rather than returning a plausible root.
fn cubic_largest_real_root<T: geom_core::Real>(c2: T, c1: T, c0: T, three_real: bool) -> T {
    let three = T::from_f64(3.0);
    let two = T::from_f64(2.0);
    let shift = c2 / three;
    let p = c1 - c2.powi(2) / three;
    let q = c2.powi(3) * two / T::from_f64(27.0) - c2 * c1 / three + c0;
    if three_real {
        let scale = (T::zero() - p / three).max(T::zero()).sqrt();
        let arg = (three * q) / (two * p) * (T::zero() - three / p).max(T::zero()).sqrt();
        // The argument is a cosine by construction; clamping is the
        // rounding guard, not a decision (an out-of-range value here
        // would be a rounding artefact of the branch's own premise).
        let arg = arg.max(T::zero() - T::one()).min(T::one());
        return two * scale * (arg.acos() / three).cos() - shift;
    }
    let inner = (q.powi(2) / T::from_f64(4.0) + p.powi(3) / T::from_f64(27.0))
        .max(T::zero())
        .sqrt();
    // Both terms non-negative: the sum does not cancel, and needs no sign.
    let big_a = cbrt(q.abs() / two + inner);
    let big_b = p / (three * big_a);
    // `A − B` vanishes at `Q = 0`, so its sign transfer is continuous
    // there (the doc above).
    let x = (big_a - big_b).copysign(T::zero() - q);
    let d = x - shift;
    let pair = (x / two + shift).powi(2) + T::from_f64(0.75) * (big_a + big_b).powi(2);
    (d.powi(3) - c0) / (d.powi(2) + pair)
}

/// The predicate names one caller of [`depressed_quartic_roots`] meters
/// its ladder under, so each lane's rows stay its own in the verdict
/// log.
pub struct QuarticRows {
    /// The discriminant `Δ`'s sign.
    pub disc: &'static str,
    /// `p`'s sign on the positive branch.
    pub shape: &'static str,
    /// `D = 64s − 16p²`'s sign on the positive branch.
    pub depth: &'static str,
    /// Whether `q̂` is in band (the biquadratic arm).
    pub odd: &'static str,
    /// A quadratic factor's discriminant.
    pub split: &'static str,
    /// The resolvent's largest root.
    pub split_lead: &'static str,
}

/// **The certified real roots of the depressed monic quartic**
/// `y⁴ + p y² + q̂ y + s`: the count read off exact sign algebra (`Δ`,
/// then `p` and `D = 64s − 16p²` on the positive branch), the
/// biquadratic arm when `q̂` is in band, Ferrari's factorization
/// otherwise, and the constructed count checked against the certified
/// one.
///
/// `y` is a LENGTH and `lever` the length scale its roots spread over:
/// every margin is metered as `coefficient / lever^k` so that it is a
/// length (`Δ` over `lever¹¹`, `p` over `lever`, `D` over `lever³`,
/// `q̂` over `lever²`, the factors' discriminants over `lever`), which
/// is what makes the ladder's band a statement in metres. The roots
/// come back unordered in `ts[..count]`, in the `y` variable.
///
/// # The certified count, and what it costs
///
/// The count is read off exact sign algebra, never off the roots: with
/// `Δ` the quartic's discriminant,
///
/// * `Δ` Negative ⇒ exactly **2** real roots;
/// * `Δ` Positive ⇒ **4** real when `p` and `D` are both Negative, and
///   **0** otherwise;
/// * `Δ` Zero, or a Zero on `p`/`D` inside the positive branch ⇒ the
///   count is NOT certain, and the ladder answers only on a certain one.
///
/// `Δ` is metered over `lever¹¹` because for a monic quartic
/// `Δ = Π_{i<j}(y_i − y_j)²`: two roots a distance `δ` apart, with the
/// rest spread over the lever, put `Δ ≈ δ²·lever¹⁰` and the margin at
/// `δ²/lever`. **So the ladder refuses over a BAND around a repeated
/// root, not at one**, and the band is as wide as `√(K·ε·lever)` in root
/// separation.
///
/// # The ladder, rung by rung
///
/// The leading coefficient is the literal `1`: the degree cannot drop,
/// and nothing here divides by a lead. What a degenerate quartic
/// degrades is its ROOT STRUCTURE, and each rung of that is typed, by
/// its row in [`QuarticRows`]:
///
/// 1. **`disc` Zero**: the quartic has a repeated real root.
///    [`Unsettled::Tangency`].
/// 2. **`odd` Zero**: `q̂` is in band and the quartic is BIQUADRATIC.
///    Not a degeneracy but a different closed form: `y²` solves a
///    quadratic and the roots come out of two square roots, which is also
///    the only form the resolvent below cannot be used for (its `α = √z`
///    would be zero and `q̂/α` poison).
///    **This arm has a band, and it costs root accuracy rather than a
///    count.** It is taken whenever `q̂/lever²` is in band, i.e. for a
///    `q̂` that is small rather than zero, and it then solves the quartic
///    with `q̂` SET to zero: the nearby quartic, not the posed one. The
///    root error that buys is first order in the dropped term,
///    `≈ q̂/lever²`, so it is bounded by the band itself. The count is
///    unaffected: it was certified from `Δ` before this branch was
///    chosen.
/// 3. **`split_lead` non-Positive**: the resolvent's largest real root
///    is not positive, which contradicts its own constant term `−q̂² < 0`
///    on the branch that computed it. [`Unsettled::Resolvent`]. It is
///    NOT only a rounding-scale contradiction: a `q̂` small enough that
///    the root `≈ q̂²/c1` falls inside the band, but large enough to clear
///    rung 2, reaches it too, a legitimate root refused on its size
///    (`work/inside/torus-split-lead-escalates-a-legitimately-small-resolvent-root.md`).
/// 4. **`split` Zero**: one of the two quadratic factors has a zero
///    discriminant, i.e. a double root, which contradicts the definite
///    discriminant of rung 1. [`Unsettled::Tangency`].
/// 5. **[`Unsettled::CountDisagrees`]**: the roots CONSTRUCTED
///    disagree in number with the count CERTIFIED by rung 1. Where
///    measured exactly, the construction is right and rung 1 is wrong:
///    `Δ`'s `f64` evaluation carries rounding far past the band that no
///    decision charges, so a definite-looking sign can be the noise's
///    (`work/inside/ray-torus-count-certifies-on-an-uncharged-discriminant.md`;
///    measured by `topo`'s
///    `torus_predicate_rows::the_ladder_disagrees_with_itself_on_a_lanes_coefficients`
///    and `a_far_ray_reaches_the_count_disagreement`). This rung catches
///    the defect only where the two counts differ; where the noise flips
///    the count to [`Roots::Miss`], the answer is wrong and
///    nothing here sees it.
///
/// # Errors
///
/// [`RootRefusal::Escalated`], an in-band classifying sign, and
/// [`RootRefusal::Unsettled`], the rungs above that certify no count:
/// each is a reading legal input reaches.
#[allow(clippy::too_many_lines)] // one closed form and its ladder
pub fn depressed_quartic_roots<T: Decide>(
    p: T,
    q_hat: T,
    s: T,
    lever: T,
    rows: &QuarticRows,
    band: Band,
) -> Result<QuarticRoots<T>, RootRefusal> {
    let two = T::from_f64(2.0);
    let four = T::from_f64(4.0);
    let disc = T::from_f64(256.0) * s.powi(3) - T::from_f64(128.0) * p.powi(2) * s.powi(2)
        + T::from_f64(144.0) * p * q_hat.powi(2) * s
        - T::from_f64(27.0) * q_hat.powi(4)
        + T::from_f64(16.0) * p.powi(4) * s
        - four * p.powi(3) * q_hat.powi(2);
    let disc_sign = decide(rows.disc, Margin::over_lever(disc, lever.powi(11)), band)?;
    let count = match disc_sign {
        Sign::Negative => 2usize,
        Sign::Zero => return Err(Unsettled::Tangency.into()),
        Sign::Positive => {
            let shape = decide(rows.shape, Margin::over_lever(p, lever), band)?;
            let depth = decide(
                rows.depth,
                Margin::over_lever(
                    T::from_f64(64.0) * s - T::from_f64(16.0) * p.powi(2),
                    lever.powi(3),
                ),
                band,
            )?;
            match (shape, depth) {
                (Sign::Negative, Sign::Negative) => 4,
                (Sign::Positive, _) | (_, Sign::Positive) => 0,
                // One of the two is on its own zero while the other does
                // not settle the question: the count is not certain.
                _ => return Err(Unsettled::Tangency.into()),
            }
        }
    };
    if count == 0 {
        return Ok(Roots::Miss);
    }
    // The two quadratic factors of the depressed quartic, as
    // `(y² + α y + β)(y² − α y + γ)`. The biquadratic arm is the
    // `α = 0` one and is taken on its own closed form.
    let (f0, f1) =
        if decide(rows.odd, Margin::over_lever(q_hat, lever.powi(2)), band)? == Sign::Zero {
            // `y⁴ + p y² + s`: with `α = 0` the factorization's own
            // relations collapse to `β + γ = p` and `βγ = s`, so the two
            // factors are `y² + β` and `y² + γ` with `β`, `γ` the roots of
            // `X² − pX + s`. Those are the NEGATIVES of the roots of the
            // quadratic in `y²`, which is the sign this arm is easy to get
            // backwards and which no in-band case would have caught.
            let inner = p.powi(2) - four * s;
            match decide(rows.split, Margin::over_lever(inner, lever.powi(3)), band)? {
                Sign::Positive => {}
                // A repeated `y²`, or none at all with a count that says
                // otherwise: neither is a certified pair of factors.
                Sign::Zero | Sign::Negative => return Err(Unsettled::Tangency.into()),
            }
            let [lo, hi] = quadratic_roots(T::one(), T::zero() - p / two, s, inner / four);
            ((T::zero(), hi), (T::zero(), lo))
        } else {
            // Ferrari: `z = α²` is a root of `z³ + 2p z² + (p² − 4s) z − q̂²`,
            // whose constant term is negative, so its LARGEST real root is
            // positive — the one root whose square root splits the quartic
            // over the reals. On four real quartic roots the resolvent has
            // three and the largest is at least a third of their sum; on two
            // it has exactly ONE, and that one can be as small as `≈ q̂²/c1`
            // (a ray all but perpendicular to the axis), which is where
            // `cubic_largest_real_root`'s conditioning has to come from its
            // own construction rather than from the choice. The resolvent
            // shares the quartic's discriminant, so the branch is the sign
            // already decided above rather than a second decision.
            let z = cubic_largest_real_root(
                two * p,
                p.powi(2) - four * s,
                T::zero() - q_hat.powi(2),
                disc_sign == Sign::Positive,
            );
            match decide(rows.split_lead, Margin::over_lever(z, lever), band)? {
                Sign::Positive => {}
                Sign::Zero | Sign::Negative => return Err(Unsettled::Resolvent.into()),
            }
            let alpha = z.max(T::zero()).sqrt();
            let half_gap = q_hat / alpha;
            (
                (alpha, (p + z - half_gap) / two),
                (T::zero() - alpha, (p + z + half_gap) / two),
            )
        };
    // Each factor `y² + a y + c` contributes its own two roots when its
    // discriminant is definitely positive, none when definitely
    // negative. A ZERO discriminant is a double root, which contradicts
    // the definite quartic discriminant above — uncertain, not answered.
    let mut ts = [T::zero(); 4];
    let mut found = 0usize;
    for (a, c) in [f0, f1] {
        let inner = a.powi(2) - four * c;
        match decide(rows.split, Margin::over_lever(inner, lever), band)? {
            Sign::Negative => continue,
            Sign::Zero => return Err(Unsettled::Tangency.into()),
            Sign::Positive => {}
        }
        for y in quadratic_roots(T::one(), a / two, c, inner / four) {
            ts[found] = y;
            found += 1;
        }
    }
    // **Rung 5.** Two factors contribute two roots each, so `found` is
    // 0, 2 or 4 by construction and the array cannot overflow; what this
    // compares is the CONSTRUCTED count against the CERTIFIED one, the
    // one place the sign ladder and the factorization are made to agree.
    if found != count {
        return Err(Unsettled::CountDisagrees {
            certified: count,
            constructed: found,
        }
        .into());
    }
    Ok(Roots::of(&ts[..found])
        .unwrap_or_else(|| unreachable!("two factors make at most four roots")))
}

#[cfg(test)]
mod cbrt_rows {
    //! The `sqrt`-chain cube root against `f64::cbrt`, and its truncation
    //! registered as a systematic bias rather than an enclosure — the
    //! magnitude argument [`super::cbrt`]'s docs rest on.

    use super::cbrt;

    /// Compared against `f64::cbrt` over 12 decades either side of 1 and
    /// both signs.
    #[test]
    fn the_chain_tracks_the_true_cube_root() {
        let mut worst = 0.0f64;
        let mut worst_x = 0.0f64;
        for e in -30..=30 {
            for mant in [1.0, 1.7, 3.3, 6.1, 9.4] {
                for sgn in [1.0, -1.0] {
                    let x = sgn * mant * 10f64.powi(e);
                    let got = cbrt(x);
                    let want = x.cbrt();
                    let rel = ((got - want) / want).abs();
                    if rel > worst {
                        worst = rel;
                        worst_x = x;
                    }
                }
            }
        }
        println!("cbrt: worst relative error {worst:e} at x = {worst_x:e}");
        assert_eq!(cbrt(0.0), 0.0, "cbrt(0) must be 0");
        assert!(
            worst < 1e-12,
            "the sqrt-chain cube root drifts from the true one by {worst:e} at {worst_x:e}"
        );
    }

    /// The truncation is a SYSTEMATIC offset, not an enclosure: the chain
    /// computes `x^((1−4^-27)/3)`, and this measures how far that is from
    /// `x^(1/3)` at the magnitudes the resolvent actually feeds it.
    #[test]
    #[ignore = "a printed census, asserting nothing: cargo nextest run -p geom-brep --run-ignored only the_truncation_is_a_bias"]
    fn the_truncation_is_a_bias_not_a_containment() {
        for x in [1e-18f64, 1e-6, 1.0, 1e6, 1e18, 1e36] {
            let exact = x.powf((1.0 - 4f64.powi(-27)) / 3.0);
            let truth = x.cbrt();
            println!(
                "cbrt truncation: x={x:e} chain-exponent value={exact:e} true={truth:e} rel={:e}",
                ((exact - truth) / truth).abs()
            );
        }
    }
}
