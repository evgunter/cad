//! **Refinement by certification** (C3): a fitted carrier gets the
//! samples its certificate asks for.
//!
//! The march spaces its samples by the curvature against ε, a design
//! target and not a bound (`SSI_STEP_DEVIATION`), and on a straight
//! stretch can place fewer than the cubic fit needs; those it is given
//! by the same halving, before any certificate ([`fit_minimum`]).
//! Where limbs 1 and 2 refuse the
//! fitted carrier, the certificate names the spans it refused
//! ([`Located`]); the gaps between samples those spans meet are halved,
//! with one gap on each side of them, and the carrier is refitted and
//! certified again. A new sample is the gap's midpoint settled onto the
//! locus, kept only inside the domain, so it is a candidate like any
//! marched state, and the certificate decides it.
//!
//! The neighbours are halved with the refused gap because the
//! interpolating cubic's value over a gap moves with the samples beside
//! it: halving the refused gap alone leaves an abrupt change of spacing
//! next to it, where the refusal then moves, one gap a round.
//!
//! A midpoint settles within half its gap and the settling residual of
//! the gap's chord midpoint where the gap holds one arc ([`halve`]); one
//! that settles farther, or not at all, refuses the polyline as not one
//! arc ([`SsiError::PolylineNotOneArc`]), so the fit never reads a
//! polyline short of its samples.
//!
//! The rule is fixed (D9), and it stops on its own terms, which the
//! refusal then names ([`RefineStop`]): where no selected gap can be
//! halved, half of each falling in the band (`ssi_refine_halving`) or
//! its midpoint settling outside the domain, and where the next round
//! would give the branch more steps than its budget
//! ([`super::SSI_MAX_STEPS`], the march's own). The band stop is the
//! termination: a gap is halved only while half of it clears the band,
//! so a branch of length `L` holds at most about `2L` over the band's
//! escalate bound in samples. That bound is formal, so the step budget
//! is the resource wall. Every round that does not stop adds a sample,
//! so refinement takes at most `budget − n₀ + 2` rounds from `n₀`
//! marched samples, and at most half that where every selected
//! midpoint settles on a branch of two gaps or more, since a refused
//! gap is then halved with at least one neighbour.
//!
//! The refusal carries the earlier rounds' refused limbs and margins
//! ([`RefusedRound`]) beside its own, so a margin that stays flat while
//! the samples double shows as the floor it is, and the step budget's
//! ending reads the last two ([`stopped_falling`]). Each round's time is
//! linear in the samples, and only the wall bounds the rounds
//! (`work/ssi/ssi-refinement-can-spend-hours-on-one-branch-before-the-step-wall.md`).

use geom::NurbsCurve3;
use geom_core::{Band, Margin, MarginDiag, Point3};

use crate::dihedral::decide_reported;
use crate::recourse::Refused;

use super::certify::{Located, SsiLimb};
use super::march::{MarchContext, newton_refine, within};
use super::section::BandVerdict;
use super::system::LocalSystem;
use super::{ArcMiss, BranchBound, SSI_FIT_DEGREE, SsiError};

/// Where refinement stopped short of a certificate
/// ([`SsiError::RefinementExhausted`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RefineStop {
    /// No gap the refusal selected could be halved.
    NothingToHalve {
        /// Gaps half of which falls in the band.
        in_band: usize,
        /// Gaps whose midpoint settled outside the domain, or onto an
        /// end.
        unsettled: usize,
    },
    /// The next round would have given the branch more steps, the gaps
    /// between its samples, than the step budget the march spends
    /// ([`super::SSI_MAX_STEPS`]): a resource wall, not a verdict on
    /// the carrier.
    StepBudget {
        /// The budget.
        budget: usize,
    },
}

/// One refinement round's refusal, as the certificate reported it. The
/// certificate stops at the first limb that refuses, so a round names
/// that limb alone, and a later round can name another.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RefusedRound {
    /// The samples the refused carrier was fitted through.
    pub samples: usize,
    /// The limb that refused.
    pub limb: SsiLimb,
    /// What it read.
    pub margin: RoundMargin,
}

/// What a refusing limb read in one round.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RoundMargin {
    /// A definite refusal, in metres: limb 2's bound over the whole
    /// carrier, or limb 1's residual at the first sample that refused.
    Over(f64),
    /// Undecided: the margin the classifier saw, inside the band, for
    /// the refusal's report only.
    InBand(MarginDiag),
}

/// The limb that refused and what it read, for a refusal of limb 1 or 2:
/// the refusals the certificate locates. `None` for every other.
pub(crate) fn limb_reading(error: &SsiError) -> Option<(SsiLimb, RoundMargin)> {
    match error {
        SsiError::CertificateLimb { limb, value } => Some((*limb, RoundMargin::Over(*value))),
        SsiError::CertificateEscalated { limb, cause } => {
            Some((*limb, RoundMargin::InBand(cause.margin)))
        }
        _ => None,
    }
}

/// Whether the refused margin stopped falling over the last two rounds
/// of a refinement the step budget stopped: both in the band, or both
/// definite with the later no smaller. A reading of the numbers the
/// rounds recorded, for the refusal's ending; it decides nothing.
pub(crate) fn stopped_falling(before: Option<RoundMargin>, last: Option<RoundMargin>) -> bool {
    match (before, last) {
        (Some(RoundMargin::InBand(_)), Some(RoundMargin::InBand(_))) => true,
        (Some(RoundMargin::Over(a)), Some(RoundMargin::Over(b))) => b >= a,
        _ => false,
    }
}

/// Fit and certify `states`, refining them where the certificate
/// refuses (module docs) until it certifies.
///
/// # Errors
///
/// [`fit_minimum`]'s refusals, the sized one bounded by `bounded_by`;
/// [`SsiError::PolylineNotOneArc`] where a refined midpoint shows its
/// gap does not hold one arc; a refusal the certificate does not locate
/// (limb 3, a foot point, a limb whose margin is no number, the fit), as
/// it is; and a located one that refinement cannot answer as
/// [`SsiError::RefinementExhausted`], naming where it stopped, on how
/// many samples, and every round's refusal.
pub(crate) fn refine_by_certificate<const M: usize, const N: usize, S, C>(
    sys: &S,
    states: Vec<[f64; N]>,
    ctx: &MarchContext<N>,
    band: Band,
    bounded_by: BranchBound,
    mut certify: impl FnMut(&[[f64; N]]) -> Result<C, Located>,
) -> Result<C, SsiError>
where
    S: LocalSystem<M, N>,
{
    let mut states = fit_minimum(sys, states, ctx, band, bounded_by)?;
    let mut earlier = Vec::new();
    loop {
        let (error, at) = match certify(&states) {
            Ok(certified) => return Ok(certified),
            Err(Located {
                error,
                at: Some(at),
            }) => (error, at),
            Err(Located { error, at: None }) => return Err(error),
        };
        let points: Vec<Point3<f64>> = states.iter().map(|s| sys.point(s)).collect();
        // The fit just read these same points, so this does not refuse.
        let Ok(params) = NurbsCurve3::<f64>::chord_parameters(&points) else {
            return Err(error);
        };
        let hit: Vec<bool> = params
            .windows(2)
            .map(|t| at.spans.iter().any(|r| r.lo <= t[1] && r.hi >= t[0]))
            .collect();
        let (mut in_band, mut unsettled) = (0usize, 0usize);
        let mut finer = Vec::with_capacity(2 * states.len());
        for (i, pair) in states.windows(2).enumerate() {
            finer.push(pair[0]);
            let selected =
                hit[i] || (i > 0 && hit[i - 1]) || hit.get(i + 1).copied().unwrap_or(false);
            if !selected {
                continue;
            }
            match halve(sys, &pair[0], &pair[1], ctx, band) {
                Halving::Settled(mid) => finer.push(mid),
                Halving::InBand(_) => in_band += 1,
                Halving::OffDomain => unsettled += 1,
                Halving::NotOneArc(miss) => return Err(not_one_arc(sys, pair, miss)),
            }
        }
        finer.extend(states.last().copied());
        let stop = if finer.len() == states.len() {
            Some(RefineStop::NothingToHalve { in_band, unsettled })
        } else if finer.len() - 1 > ctx.max_steps {
            Some(RefineStop::StepBudget {
                budget: ctx.max_steps,
            })
        } else {
            None
        };
        if let Some(stop) = stop {
            return Err(SsiError::RefinementExhausted {
                stop,
                samples: states.len(),
                refusal: Box::new(error),
                earlier,
            });
        }
        earlier.push(RefusedRound {
            samples: states.len(),
            limb: at.limb,
            margin: at.margin,
        });
        states = finer;
    }
}

/// **The fit's minimum** (C3): `states` given the cubic's
/// `SSI_FIT_DEGREE + 1` samples where it has fewer. Each round halves the
/// longest gap whose midpoint settles ([`halve`]), so a polyline reaches
/// the minimum exactly and nothing but the curvature and the
/// certificate sets a count above it.
///
/// # Errors
///
/// [`SsiError::PolylineNotOneArc`] for the first gap whose midpoint
/// shows it does not hold one arc, or, where nothing halves and no half
/// gap falls in the band, the longest gap, its midpoint settled outside
/// the domain. Otherwise, where nothing halves,
/// [`SsiError::ShortBranchUncertified`] in the polyline's length, on
/// the first half gap that falls in the band, bounded by `bounded_by`.
pub(crate) fn fit_minimum<const M: usize, const N: usize, S>(
    sys: &S,
    mut states: Vec<[f64; N]>,
    ctx: &MarchContext<N>,
    band: Band,
    bounded_by: BranchBound,
) -> Result<Vec<[f64; N]>, SsiError>
where
    S: LocalSystem<M, N>,
{
    let chord =
        |states: &[[f64; N]], i: usize| (sys.point(&states[i + 1]) - sys.point(&states[i])).norm();
    'round: while states.len() < SSI_FIT_DEGREE + 1 {
        let mut gaps: Vec<usize> = (0..states.len().saturating_sub(1)).collect();
        gaps.sort_by(|&i, &j| {
            chord(&states, j)
                .total_cmp(&chord(&states, i))
                .then(i.cmp(&j))
        });
        let mut in_band = None;
        let mut off_domain = None;
        for &i in &gaps {
            match halve(sys, &states[i], &states[i + 1], ctx, band) {
                Halving::Settled(mid) => {
                    states.insert(i + 1, mid);
                    continue 'round;
                }
                Halving::InBand(verdict) => {
                    in_band.get_or_insert(verdict);
                }
                Halving::OffDomain => {
                    off_domain.get_or_insert(i);
                }
                Halving::NotOneArc(miss) => {
                    return Err(not_one_arc(sys, &states[i..=i + 1], miss));
                }
            }
        }
        if let (None, Some(i)) = (&in_band, off_domain) {
            return Err(not_one_arc(sys, &states[i..=i + 1], ArcMiss::OffDomain));
        }
        // A polyline of one state has no gap, and no length to halve.
        let verdict = match in_band {
            Some(verdict) => verdict,
            None => match halving_verdict(0.0, band) {
                Some(verdict) => verdict,
                None => unreachable!("a zero length clears no band"),
            },
        };
        return Err(SsiError::ShortBranchUncertified {
            length: (0..states.len().saturating_sub(1))
                .map(|i| chord(&states, i))
                .sum(),
            limb: None,
            verdict,
            bounded_by,
        });
    }
    Ok(states)
}

/// The refusal of a polyline whose gap from `pair[0]` to `pair[1]` does
/// not hold one arc.
fn not_one_arc<const M: usize, const N: usize, S: LocalSystem<M, N>>(
    sys: &S,
    pair: &[[f64; N]],
    miss: ArcMiss,
) -> SsiError {
    SsiError::PolylineNotOneArc {
        gap: (sys.point(&pair[1]) - sys.point(&pair[0])).norm(),
        miss,
    }
}

/// What halving one gap gave.
enum Halving<const N: usize> {
    /// The midpoint, settled onto the locus inside the domain, within
    /// one arc's reach of the gap's chord.
    Settled([f64; N]),
    /// Half the gap's chord does not clear the band: the verdict on it.
    InBand(BandVerdict),
    /// The midpoint settled outside the domain, or onto an end.
    OffDomain,
    /// The midpoint shows the gap does not hold one arc.
    NotOneArc(ArcMiss),
}

/// The verdict on half a gap `half` metres long where it does not clear
/// the band (`ssi_refine_halving`, a length that passes positive).
fn halving_verdict(half: f64, band: Band) -> Option<BandVerdict> {
    match decide_reported("ssi_refine_halving", Margin::of(half), band) {
        Ok(decided) => Refused::of(decided, band).map(BandVerdict::Refused),
        Err(cause) => Some(BandVerdict::Undecided(cause)),
    }
}

/// The midpoint of the gap from `a` to `b`, settled onto the locus,
/// where half the gap's chord clears the band.
///
/// **The one-arc stop.** An arc turning by at most π lies within `g/2`
/// of its chord's midpoint, `g` the chord's length, and the curvature
/// rungs keep each step's turn far below π. So a gap holding one arc has
/// its midpoint settle within `g/2` and the settling residual of the
/// chord's midpoint; one that settles farther, or not at all, does not.
fn halve<const M: usize, const N: usize, S>(
    sys: &S,
    a: &[f64; N],
    b: &[f64; N],
    ctx: &MarchContext<N>,
    band: Band,
) -> Halving<N>
where
    S: LocalSystem<M, N>,
{
    let (pa, pb) = (sys.point(a), sys.point(b));
    let half = 0.5 * (pb - pa).norm();
    if let Some(verdict) = halving_verdict(half, band) {
        return Halving::InBand(verdict);
    }
    let mid: [f64; N] = core::array::from_fn(|i| 0.5 * (a[i] + b[i]));
    let Some(settled) = newton_refine(sys, mid, ctx.tol) else {
        return Halving::NotOneArc(ArcMiss::Unsettled);
    };
    let off = (sys.point(&settled) - pa.lerp(pb, 0.5)).norm();
    if off.is_nan() || off > half + ctx.tol.settling() {
        return Halving::NotOneArc(ArcMiss::Far { off });
    }
    if within(&settled, &ctx.domain) && settled != *a && settled != *b {
        Halving::Settled(settled)
    } else {
        Halving::OffDomain
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use geom_core::Band;

    use super::{
        BranchBound, Located, RefineStop, RefusedRound, RoundMargin, SsiError, SsiLimb,
        refine_by_certificate,
    };
    use crate::ssi::SSI_MAX_STEPS;
    use crate::ssi::certify::RefusedSpan;
    use crate::ssi::certify::Spans;
    use crate::ssi::march::MarchContext;
    use crate::ssi::march::tests::{FixedSpeedR3, unit_ctx};

    /// The x coordinates of states on the `x` axis.
    fn xs(states: &[[f64; 3]]) -> Vec<f64> {
        states.iter().map(|s| s[0]).collect()
    }

    /// The chord parameter of `x` on the `x` axis from −0.8 to 0.8.
    fn t(x: f64) -> f64 {
        (x + 0.8) / 1.6
    }

    /// The hull limb's refusal of `[lo, hi]` in `x`, reading `value`, as
    /// the stand-ins below raise it.
    fn refusal(lo: f64, hi: f64, value: f64) -> Located {
        Located {
            error: SsiError::CertificateLimb {
                limb: SsiLimb::HullSup,
                value,
            },
            at: Some(Box::new(Spans {
                limb: SsiLimb::HullSup,
                margin: RoundMargin::Over(value),
                spans: vec![RefusedSpan {
                    lo: t(lo),
                    hi: t(hi),
                }],
            })),
        }
    }

    /// A certificate stand-in over the `x` axis from −0.8 to 0.8: it
    /// refuses `[lo, hi]` in `x` while any gap meeting it is longer than
    /// `longest`, reading the longest such gap, and otherwise passes the
    /// states through.
    fn refusing(
        lo: f64,
        hi: f64,
        longest: f64,
    ) -> impl FnMut(&[[f64; 3]]) -> Result<Vec<[f64; 3]>, Located> {
        move |states| {
            let widest = states
                .windows(2)
                .filter(|g| g[1][0] >= lo && g[0][0] <= hi)
                .map(|g| g[1][0] - g[0][0])
                .fold(0.0, f64::max);
            if widest > longest {
                Err(refusal(lo, hi, widest))
            } else {
                Ok(states.to_vec())
            }
        }
    }

    /// Five states 0.4 apart along the `x` axis.
    fn axis_states() -> Vec<[f64; 3]> {
        [-0.8, -0.4, 0.0, 0.4, 0.8].map(|x| [x, 0.0, 0.0]).to_vec()
    }

    /// The unit context with the kernel's step budget.
    fn kernel_ctx(band: Band) -> MarchContext<3> {
        MarchContext {
            max_steps: SSI_MAX_STEPS,
            ..unit_ctx(band)
        }
    }

    /// **Refinement halves the gaps the certificate refused and their
    /// neighbours, and no others.** The stand-in refuses `|x| ≤ 0.05`
    /// until the gaps meeting it are at most 0.1 long. The first round
    /// halves the two gaps about 0 and their neighbours, the outer gaps;
    /// the second halves the two gaps about 0 again and their neighbours
    /// out to ±0.4, so the outer gaps, never refused, are halved once.
    #[test]
    fn refinement_halves_the_refused_gaps_and_their_neighbours() {
        let band = Band::new(1.0e-9, 1.0e-8).unwrap();
        let sys = FixedSpeedR3::at_speed(1.0);
        let ctx = kernel_ctx(band);
        let mut rounds = 0;
        let mut certify = refusing(-0.05, 0.05, 0.1);
        let out = refine_by_certificate(&sys, axis_states(), &ctx, band, BranchBound::Slab, |s| {
            rounds += 1;
            certify(s)
        })
        .unwrap();
        let want = [
            -0.8, -0.6, -0.4, -0.3, -0.2, -0.1, 0.0, 0.1, 0.2, 0.3, 0.4, 0.6, 0.8,
        ];
        let got = xs(&out);
        assert!(
            got.len() == want.len() && got.iter().zip(want).all(|(g, w)| (g - w).abs() < 1e-12),
            "the refused gaps and their neighbours: {got:?}"
        );
        assert_eq!(rounds, 3, "two refusals and the pass");
    }

    /// **Refinement stops at the band, carrying every round's refusal**:
    /// a refusal at `x = 0` that no density answers is halved until half
    /// a gap would fall in the band, and stops naming the band, with one
    /// round recorded per refused carrier, each on the samples it had and
    /// the margin it read, and the certificate's own refusal standing.
    #[test]
    fn refinement_stops_at_the_band_with_its_history() {
        let band = Band::new(1.0e-9, 1.0e-8).unwrap();
        let sys = FixedSpeedR3::at_speed(1.0);
        let ctx = kernel_ctx(band);
        let mut finest = f64::INFINITY;
        let mut seen = Vec::new();
        let mut certify = refusing(0.0, 0.0, 0.0);
        let r = refine_by_certificate(&sys, axis_states(), &ctx, band, BranchBound::Slab, |s| {
            for g in s.windows(2) {
                finest = finest.min(g[1][0] - g[0][0]);
            }
            let verdict = certify(s);
            if let Err(Located {
                error: SsiError::CertificateLimb { value, .. },
                ..
            }) = &verdict
            {
                seen.push((s.len(), *value));
            }
            verdict
        });
        match r {
            Err(SsiError::RefinementExhausted {
                stop: RefineStop::NothingToHalve { in_band, unsettled },
                refusal,
                samples,
                earlier,
            }) => {
                assert!(
                    in_band > 0 && unsettled == 0,
                    "{in_band} in band, {unsettled}"
                );
                assert!(
                    matches!(
                        *refusal,
                        SsiError::CertificateLimb {
                            limb: SsiLimb::HullSup,
                            ..
                        }
                    ),
                    "the stand-in's refusal stands: {refusal:?}"
                );
                assert_eq!(
                    earlier.len() + 1,
                    seen.len(),
                    "a round per refused carrier, the last as the refusal"
                );
                for (round, (n, value)) in earlier.iter().zip(&seen) {
                    assert_eq!(
                        *round,
                        RefusedRound {
                            samples: *n,
                            limb: SsiLimb::HullSup,
                            margin: RoundMargin::Over(*value),
                        },
                        "each round as it was refused"
                    );
                }
                assert_eq!(
                    samples,
                    seen.last().unwrap().0,
                    "the last carrier's samples"
                );
            }
            other => panic!("expected refinement exhausted at the band, got {other:?}"),
        }
        assert!(
            finest > band.escalate() && finest <= 4.0 * band.escalate(),
            "the last cut is the band's: {finest:e}"
        );
    }

    /// **A floor refinement cannot pass meets the step budget, typed,
    /// with the floor in its history.** The stand-in refuses the whole
    /// branch at a margin that does not move, as an enclosure's floor
    /// would, so every round doubles the samples; the round that would
    /// overrun the context's 64 steps is not taken, and every round reads
    /// the same margin.
    #[test]
    fn a_flat_margin_stops_at_the_step_budget_with_its_history() {
        let band = Band::new(1.0e-9, 1.0e-8).unwrap();
        let sys = FixedSpeedR3::at_speed(1.0);
        let ctx = unit_ctx(band);
        let floor = 3.0e-9;
        let r = refine_by_certificate(&sys, axis_states(), &ctx, band, BranchBound::Slab, |_| {
            Err::<(), _>(refusal(-0.8, 0.8, floor))
        });
        match r {
            Err(SsiError::RefinementExhausted {
                stop: RefineStop::StepBudget { budget },
                samples,
                earlier,
                ..
            }) => {
                assert_eq!(budget, ctx.max_steps);
                assert_eq!(samples, 65, "the doubling the wall stopped: 128 steps next");
                let counts: Vec<usize> = earlier.iter().map(|r| r.samples).collect();
                assert_eq!(counts, [5, 9, 17, 33], "the samples doubled each round");
                assert!(
                    earlier.iter().all(|r| r.margin == RoundMargin::Over(floor)),
                    "the flat margin, round by round: {earlier:?}"
                );
            }
            other => panic!("expected refinement exhausted at the step budget, got {other:?}"),
        }
    }

    /// **A refusal that creeps one gap a round is bounded.** The
    /// stand-in refuses only the leftmost gap longer than 1e-3, so the
    /// branch would certify on 1601 samples, more than the context's 64
    /// steps: refusal by refusal, the leftmost coarse gap moves right. A
    /// refused gap is halved with at least one neighbour, so a round adds
    /// at least two samples and the rounds are at most `(64 − 4)/2 + 1`;
    /// the refusal names the step budget.
    #[test]
    fn a_creeping_refusal_is_bounded_by_the_neighbours_it_halves() {
        let band = Band::new(1.0e-9, 1.0e-8).unwrap();
        let sys = FixedSpeedR3::at_speed(1.0);
        let ctx = unit_ctx(band);
        let tau = 1.0e-3;
        let mut calls = 0usize;
        let r = refine_by_certificate(&sys, axis_states(), &ctx, band, BranchBound::Slab, |s: &[[f64; 3]]| {
            calls += 1;
            match s.windows(2).find(|g| g[1][0] - g[0][0] > tau) {
                Some(g) => Err(refusal(g[0][0] + 1e-12, g[1][0] - 1e-12, g[1][0] - g[0][0])),
                None => Ok(()),
            }
        });
        let rounds = match r {
            Err(SsiError::RefinementExhausted {
                stop: RefineStop::StepBudget { .. },
                earlier,
                ..
            }) => earlier.len() + 1,
            other => panic!("expected refinement exhausted at the step budget, got {other:?}"),
        };
        assert_eq!(rounds, calls, "a round per refused carrier");
        let bound = (ctx.max_steps - 4) / 2 + 1;
        assert!(calls <= bound, "{calls} rounds, bound {bound}");
    }
}
