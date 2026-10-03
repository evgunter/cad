//! **Refinement by certification** (C3): a fitted carrier gets the
//! samples its certificate asks for.
//!
//! The march spaces its samples by the curvature against ε, a design
//! target and not a bound (`SSI_STEP_DEVIATION`), and on a straight
//! stretch places only what the fit needs. Where limbs 1 and 2 refuse the
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
//! The rule is fixed (D9), and it stops on its own terms, which the
//! refusal then names ([`RefineStop`]): where no selected gap can be
//! halved, half of each falling in the band (`ssi_refine_halving`) or
//! its midpoint not settling inside the domain, and where the next round
//! would hand the fit more than [`super::SSI_MAX_FIT_SAMPLES`]. Every
//! round that does not stop adds a sample, so refinement takes at most
//! `SSI_MAX_FIT_SAMPLES − n₀ + 1` rounds from `n₀` marched samples, and
//! at most half that where every selected midpoint settles on a branch
//! of two gaps or more, since a refused gap is then halved with at least
//! one neighbour.

use geom::NurbsCurve3;
use geom_core::{Band, Margin, Point3, Sign};

use crate::dihedral::decide;

use super::certify::Located;
use super::march::{MarchContext, newton_refine, within};
use super::system::LocalSystem;
use super::{SSI_MAX_FIT_SAMPLES, SsiError};

/// Where refinement stopped short of a certificate
/// ([`SsiError::RefinementExhausted`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RefineStop {
    /// No gap the refusal selected could be halved.
    NothingToHalve {
        /// Gaps half of which falls in the band.
        in_band: usize,
        /// Gaps whose midpoint did not settle onto the locus inside the
        /// domain.
        unsettled: usize,
    },
    /// The next round would have handed the fit more samples than it
    /// takes.
    FitBudget {
        /// The samples the next round asked for.
        asked: usize,
        /// The fit's budget, [`super::SSI_MAX_FIT_SAMPLES`].
        budget: usize,
    },
}

/// Fit and certify `states`, refining them where the certificate
/// refuses (module docs) until it certifies.
///
/// # Errors
///
/// A refusal the certificate does not locate (limb 3, a foot point, a
/// limb whose margin is no number, the fit), as it is; a located one
/// that refinement cannot answer as [`SsiError::RefinementExhausted`],
/// naming where it stopped and on how many samples.
pub(crate) fn refine_by_certificate<const M: usize, const N: usize, S, C>(
    sys: &S,
    mut states: Vec<[f64; N]>,
    ctx: &MarchContext<N>,
    band: Band,
    mut certify: impl FnMut(&[[f64; N]]) -> Result<C, Located>,
) -> Result<C, SsiError>
where
    S: LocalSystem<M, N>,
{
    loop {
        let refused = match certify(&states) {
            Ok(certified) => return Ok(certified),
            Err(refused) => refused,
        };
        if refused.at.is_empty() {
            return Err(refused.error);
        }
        let points: Vec<Point3<f64>> = states.iter().map(|s| sys.point(s)).collect();
        // The fit just read these same points, so this does not refuse.
        let Ok(params) = NurbsCurve3::<f64>::chord_parameters(&points) else {
            return Err(refused.error);
        };
        let hit: Vec<bool> = params
            .windows(2)
            .map(|t| refused.at.iter().any(|r| r.lo <= t[1] && r.hi >= t[0]))
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
                Halving::InBand => in_band += 1,
                Halving::Unsettled => unsettled += 1,
            }
        }
        finer.extend(states.last().copied());
        let stop = if finer.len() == states.len() {
            Some(RefineStop::NothingToHalve { in_band, unsettled })
        } else if finer.len() > SSI_MAX_FIT_SAMPLES {
            Some(RefineStop::FitBudget {
                asked: finer.len(),
                budget: SSI_MAX_FIT_SAMPLES,
            })
        } else {
            None
        };
        if let Some(stop) = stop {
            return Err(SsiError::RefinementExhausted {
                stop,
                samples: states.len(),
                refusal: Box::new(refused.error),
            });
        }
        states = finer;
    }
}

/// What halving one gap gave.
enum Halving<const N: usize> {
    /// The midpoint, settled onto the locus inside the domain.
    Settled([f64; N]),
    /// Half the gap's chord does not clear the band.
    InBand,
    /// The midpoint did not settle onto the locus inside the domain, or
    /// settled onto an end.
    Unsettled,
}

/// The midpoint of the gap from `a` to `b`, settled onto the locus,
/// where half the gap's chord clears the band (`ssi_refine_halving`, a
/// length that passes positive).
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
    let half = 0.5 * (sys.point(b) - sys.point(a)).norm();
    if !matches!(
        decide("ssi_refine_halving", Margin::of(half), band),
        Ok(Sign::Positive)
    ) {
        return Halving::InBand;
    }
    let mid: [f64; N] = core::array::from_fn(|i| 0.5 * (a[i] + b[i]));
    match newton_refine(sys, mid, ctx.tol) {
        Some(settled) if within(&settled, &ctx.domain) && settled != *a && settled != *b => {
            Halving::Settled(settled)
        }
        _ => Halving::Unsettled,
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use geom_core::Band;

    use super::{Located, RefineStop, SSI_MAX_FIT_SAMPLES, SsiError, refine_by_certificate};
    use crate::ssi::certify::RefusedSpan;
    use crate::ssi::march::tests::{FixedSpeedR3, unit_ctx};

    /// The x coordinates of states on the `x` axis.
    fn xs(states: &[[f64; 3]]) -> Vec<f64> {
        states.iter().map(|s| s[0]).collect()
    }

    /// The chord parameter of `x` on the `x` axis from −0.8 to 0.8.
    fn t(x: f64) -> f64 {
        (x + 0.8) / 1.6
    }

    /// A refusal of `[lo, hi]` in `x`, as the stand-ins below raise it.
    fn refusal(lo: f64, hi: f64) -> Located {
        Located {
            error: SsiError::CellBudget { budget: 0 },
            at: vec![RefusedSpan {
                lo: t(lo),
                hi: t(hi),
            }],
        }
    }

    /// A certificate stand-in over the `x` axis from −0.8 to 0.8: it
    /// refuses `[lo, hi]` in `x` while any gap meeting it is longer than
    /// `longest`, and otherwise passes the states through.
    fn refusing(
        lo: f64,
        hi: f64,
        longest: f64,
    ) -> impl FnMut(&[[f64; 3]]) -> Result<Vec<[f64; 3]>, Located> {
        move |states| {
            let refused = states
                .windows(2)
                .any(|g| g[1][0] >= lo && g[0][0] <= hi && g[1][0] - g[0][0] > longest);
            if refused {
                Err(refusal(lo, hi))
            } else {
                Ok(states.to_vec())
            }
        }
    }

    /// Five states 0.4 apart along the `x` axis.
    fn axis_states() -> Vec<[f64; 3]> {
        [-0.8, -0.4, 0.0, 0.4, 0.8].map(|x| [x, 0.0, 0.0]).to_vec()
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
        let ctx = unit_ctx(band);
        let mut rounds = 0;
        let mut certify = refusing(-0.05, 0.05, 0.1);
        let out = refine_by_certificate(&sys, axis_states(), &ctx, band, |s| {
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

    /// **Refinement stops where it can no longer help, and says so**: a
    /// refusal at `x = 0` that no density answers is halved until half a
    /// gap would fall in the band, and stops naming the band; one over
    /// the whole branch stops short of the round that would overrun the
    /// fit's budget, naming the budget.
    #[test]
    fn refinement_stops_at_the_band_and_at_the_fit_budget() {
        let band = Band::new(1.0e-9, 1.0e-8).unwrap();
        let sys = FixedSpeedR3::at_speed(1.0);
        let ctx = unit_ctx(band);
        let mut finest = f64::INFINITY;
        let mut certify = refusing(0.0, 0.0, 0.0);
        let r = refine_by_certificate(&sys, axis_states(), &ctx, band, |s| {
            for g in s.windows(2) {
                finest = finest.min(g[1][0] - g[0][0]);
            }
            certify(s)
        });
        match r {
            Err(SsiError::RefinementExhausted {
                stop: RefineStop::NothingToHalve { in_band, unsettled },
                refusal,
                ..
            }) => {
                assert!(
                    in_band > 0 && unsettled == 0,
                    "{in_band} in band, {unsettled}"
                );
                assert!(
                    matches!(*refusal, SsiError::CellBudget { budget: 0 }),
                    "the stand-in's refusal stands: {refusal:?}"
                );
            }
            other => panic!("expected refinement exhausted at the band, got {other:?}"),
        }
        assert!(
            finest > band.escalate() && finest <= 4.0 * band.escalate(),
            "the last cut is the band's: {finest:e}"
        );

        let mut seen = 0;
        let mut certify = refusing(-0.8, 0.8, 0.0);
        let r = refine_by_certificate(&sys, axis_states(), &ctx, band, |s| {
            seen = seen.max(s.len());
            certify(s)
        });
        match r {
            Err(SsiError::RefinementExhausted {
                stop: RefineStop::FitBudget { asked, budget },
                samples,
                ..
            }) => {
                assert_eq!(budget, SSI_MAX_FIT_SAMPLES);
                assert!(asked > budget && samples <= budget, "{samples} → {asked}");
                assert_eq!(samples, seen, "the payload names the samples refused");
            }
            other => panic!("expected refinement exhausted at the budget, got {other:?}"),
        }
    }

    /// **A refusal that creeps one gap a round is bounded.** The
    /// stand-in refuses only the leftmost gap longer than 1e-3, so the
    /// branch would certify on 1601 samples, more than the fit takes:
    /// refusal by refusal, the leftmost coarse gap moves right. Without
    /// the neighbours, every round added one sample and the stand-in ran
    /// 1196 rounds to the budget. With them, a refused gap is halved
    /// with at least one neighbour, so a round adds at least two samples
    /// and the rounds are at most `(SSI_MAX_FIT_SAMPLES − 5)/2 + 1`;
    /// the refusal names the budget.
    #[test]
    fn a_creeping_refusal_is_bounded_by_the_neighbours_it_halves() {
        let band = Band::new(1.0e-9, 1.0e-8).unwrap();
        let sys = FixedSpeedR3::at_speed(1.0);
        let ctx = unit_ctx(band);
        let tau = 1.0e-3;
        let mut rounds = 0usize;
        let r = refine_by_certificate(&sys, axis_states(), &ctx, band, |s: &[[f64; 3]]| {
            rounds += 1;
            match s.windows(2).find(|g| g[1][0] - g[0][0] > tau) {
                Some(g) => Err(refusal(g[0][0] + 1e-12, g[1][0] - 1e-12)),
                None => Ok(()),
            }
        });
        assert!(
            matches!(
                r,
                Err(SsiError::RefinementExhausted {
                    stop: RefineStop::FitBudget { .. },
                    ..
                })
            ),
            "{r:?}"
        );
        let bound = (SSI_MAX_FIT_SAMPLES - 5) / 2 + 1;
        assert!(rounds <= bound, "{rounds} rounds, bound {bound}");
    }
}
