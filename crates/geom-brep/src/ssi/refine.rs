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
//! A midpoint that does not settle is read by the march's transversality
//! decision at the gap's chord midpoint ([`halve`]): in the band, or
//! undecided, the surfaces are near tangent there, and that is the
//! refusal; clear of it, the gap is one refinement cannot halve.
//!
//! The rule is fixed (D9), and it stops on its own terms, which the
//! refusal then names ([`RefineStop`]): where no selected gap can be
//! halved, half of each falling in the band (`ssi_refine_halving`), its
//! midpoint not settling, or settling outside the domain, and where the
//! next round
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
use geom_core::linalg::svd::Svd;
use geom_core::{Band, Margin, MarginDiag, Point3};

use super::certify::{Limbs, Located, SsiLimb};
use super::march::{
    MarchContext, TransversalityData, decide_transversality, newton_refine, within,
};
use super::section::{BandVerdict, band_verdict};
use super::system::LocalSystem;
use super::{BranchBound, SSI_FIT_DEGREE, SsiError};

/// Where refinement stopped short of a certificate
/// ([`SsiError::RefinementExhausted`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RefineStop {
    /// No gap the refusal selected could be halved.
    NothingToHalve {
        /// Gaps half of which falls in the band.
        in_band: usize,
        /// Gaps whose midpoint did not settle onto the locus, where the
        /// surfaces cross clear of the band.
        unsettled: usize,
        /// Gaps whose midpoint settled outside the domain, or onto an
        /// end.
        off_domain: usize,
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

/// Whether the refused margin stopped falling over two consecutive
/// rounds: both in the band, whatever their values, or both definite
/// with the later no smaller. It decides two things: when refinement
/// asks limb 3 of the carrier ([`refine_by_certificate`]), and how the
/// step budget's refusal ends, the arithmetic's floor or the curvature.
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
    mut certify: impl FnMut(&[[f64; N]], Limbs) -> Result<C, Located>,
) -> Result<C, SsiError>
where
    S: LocalSystem<M, N> + TransversalityData<N>,
{
    let mut states = fit_minimum(sys, states, ctx, band)?;
    let mut earlier: Vec<RefusedRound> = Vec::new();
    let mut tube_asked = false;
    loop {
        let (error, at) = match certify(&states, Limbs::All) {
            Ok(certified) => return Ok(certified),
            Err(Located {
                error,
                at: Some(at),
            }) => (error, at),
            Err(Located { error, at: None }) => return Err(error),
        };
        // A refused margin that stopped falling may be a carrier across
        // two arcs, which no halving answers: limb 3 is asked once.
        if !tube_asked && stopped_falling(earlier.last().map(|r| r.margin), Some(at.margin)) {
            tube_asked = true;
            if let Err(Located { error, .. }) = certify(&states, Limbs::Tube) {
                return Err(error);
            }
        }
        let points: Vec<Point3<f64>> = states.iter().map(|s| sys.point(s)).collect();
        // The fit just read these same points, so this does not refuse.
        let Ok(params) = NurbsCurve3::<f64>::chord_parameters(&points) else {
            return Err(error);
        };
        let hit: Vec<bool> = params
            .windows(2)
            .map(|t| at.spans.iter().any(|r| r.lo <= t[1] && r.hi >= t[0]))
            .collect();
        let (mut in_band, mut unsettled, mut off_domain) = (0usize, 0usize, 0usize);
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
                Halving::Unsettled => unsettled += 1,
                Halving::OffDomain => off_domain += 1,
                Halving::Tangent(tangent) => return Err(tangent),
            }
        }
        finer.extend(states.last().copied());
        let stop = if finer.len() == states.len() {
            Some(RefineStop::NothingToHalve {
                in_band,
                unsettled,
                off_domain,
            })
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
/// the minimum exactly.
///
/// # Errors
///
/// The transversality decision's refusal at a gap's chord midpoint where
/// the midpoint does not settle and the surfaces are near tangent there.
/// Where nothing halves: [`SsiError::ShortBranchUncertified`] in the
/// polyline's length where every gap's half falls in the band, bounded
/// by the lane ([`BranchBound::of_lane`]). Otherwise, on the ℝ³ lane,
/// [`SsiError::RefinementExhausted`] stopped with nothing to halve, the
/// slab's limit as its refusal ([`SsiError::TraceUnresolved`]); on the
/// plane × NURBS lane, [`SsiError::MarchShortOfFit`], counting what each
/// gap's midpoint did.
pub(crate) fn fit_minimum<const M: usize, const N: usize, S>(
    sys: &S,
    mut states: Vec<[f64; N]>,
    ctx: &MarchContext<N>,
    band: Band,
) -> Result<Vec<[f64; N]>, SsiError>
where
    S: LocalSystem<M, N> + TransversalityData<N>,
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
        let (mut halves_in_band, mut unsettled, mut off_domain) = (0usize, 0usize, 0usize);
        for &i in &gaps {
            match halve(sys, &states[i], &states[i + 1], ctx, band) {
                Halving::Settled(mid) => {
                    states.insert(i + 1, mid);
                    continue 'round;
                }
                Halving::InBand(verdict) => {
                    halves_in_band += 1;
                    in_band.get_or_insert(verdict);
                }
                Halving::Unsettled => unsettled += 1,
                Halving::OffDomain => off_domain += 1,
                Halving::Tangent(tangent) => return Err(tangent),
            }
        }
        // Every gap's half in the band: the branch is short.
        if let (Some(verdict), 0, 0) = (in_band, unsettled, off_domain) {
            return Err(SsiError::ShortBranchUncertified {
                length: gaps.iter().map(|&i| chord(&states, i)).sum(),
                limb: None,
                verdict,
                bounded_by: BranchBound::of_lane::<N>(),
            });
        }
        return Err(match BranchBound::of_lane::<N>() {
            BranchBound::Slab => SsiError::RefinementExhausted {
                stop: RefineStop::NothingToHalve {
                    in_band: halves_in_band,
                    unsettled,
                    off_domain,
                },
                samples: states.len(),
                refusal: Box::new(SsiError::TraceUnresolved {
                    samples: states.len(),
                    step: gaps.first().map_or(0.0, |&i| chord(&states, i)),
                }),
                earlier: Vec::new(),
            },
            BranchBound::Wall => SsiError::MarchShortOfFit {
                samples: states.len(),
                in_band: halves_in_band,
                unsettled,
                off_domain,
            },
        });
    }
    Ok(states)
}

/// What halving one gap gave.
enum Halving<const N: usize> {
    /// The midpoint, settled onto the locus inside the domain.
    Settled([f64; N]),
    /// Half the gap's chord does not clear the band: the verdict on it.
    InBand(BandVerdict),
    /// The midpoint did not settle, the surfaces crossing clear of the
    /// band at the gap's chord midpoint.
    Unsettled,
    /// The midpoint did not settle, and the transversality decision at
    /// at the gap's chord midpoint refused: the surfaces are near tangent
    /// there.
    Tangent(SsiError),
    /// The midpoint settled outside the domain, or onto an end.
    OffDomain,
}

/// The midpoint of the gap from `a` to `b`, settled onto the locus,
/// where half the gap's chord clears the band.
///
/// Where it does not settle, the march's transversality decision
/// (`ssi_transversality`) is read at the gap's chord midpoint, the state
/// Newton started from. Newton's last iterate is not read: where it
/// stops is wherever its fixed iterations ran out, off the chart or far
/// from the gap, where the decision's lever arm is not the gap's. A
/// midpoint between two branches in the band of each other lies where
/// the surfaces' normals align, and that is the refusal; clear of the
/// band, the gap is one Newton could not settle.
fn halve<const M: usize, const N: usize, S>(
    sys: &S,
    a: &[f64; N],
    b: &[f64; N],
    ctx: &MarchContext<N>,
    band: Band,
) -> Halving<N>
where
    S: LocalSystem<M, N> + TransversalityData<N>,
{
    let half = 0.5 * (sys.point(b) - sys.point(a)).norm();
    if let Some(verdict) = band_verdict("ssi_refine_halving", Margin::of(half), band) {
        return Halving::InBand(verdict);
    }
    let mid: [f64; N] = core::array::from_fn(|i| 0.5 * (a[i] + b[i]));
    match newton_refine(sys, mid, ctx.tol) {
        Some(settled) if within(&settled, &ctx.domain) && settled != *a && settled != *b => {
            Halving::Settled(settled)
        }
        Some(_) => Halving::OffDomain,
        None => {
            let sigma = Svd::<M, N>::new(sys.jacobian(&mid)).sigma_min();
            match decide_transversality(sys, &mid, sigma, ctx.extent, band) {
                Ok(()) => Halving::Unsettled,
                Err(tangent) => Halving::Tangent(tangent),
            }
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use geom_core::Band;

    use super::{
        BranchBound, Limbs, Located, RefineStop, RefusedRound, RoundMargin, SsiError, SsiLimb,
        refine_by_certificate,
    };
    use crate::ssi::SSI_MAX_STEPS;
    use crate::ssi::certify::RefusedSpan;
    use crate::ssi::certify::Spans;
    use crate::ssi::march::MarchContext;
    use crate::ssi::march::tests::{FixedSpeedR3, unit_ctx};
    use crate::ssi::march::{NormalPair, TransversalityData};
    use crate::ssi::system::LocalSystem;
    use geom_core::{Point3, Vec3};

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
        let out = refine_by_certificate(&sys, axis_states(), &ctx, band, |s, limbs| {
            if limbs == Limbs::Tube {
                return Ok(s.to_vec());
            }
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
        let r = refine_by_certificate(&sys, axis_states(), &ctx, band, |s, limbs| {
            if limbs == Limbs::Tube {
                return Ok(s.to_vec());
            }
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
                stop:
                    RefineStop::NothingToHalve {
                        in_band,
                        unsettled,
                        off_domain,
                    },
                refusal,
                samples,
                earlier,
            }) => {
                assert!(
                    in_band > 0 && unsettled == 0 && off_domain == 0,
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
        let r = refine_by_certificate(&sys, axis_states(), &ctx, band, |_, limbs| match limbs {
            Limbs::Tube => Ok(()),
            Limbs::All => Err(refusal(-0.8, 0.8, floor)),
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
        let r = refine_by_certificate(&sys, axis_states(), &ctx, band, |s: &[[f64; 3]], limbs| {
            if limbs == Limbs::Tube {
                return Ok(());
            }
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

    /// The parabola `y = 4x²`: a gap across its vertex from `x = −½` to
    /// `½` is 1 long, and its midpoint settles onto the vertex, 1 from
    /// the chord's midpoint.
    fn hairpin(x: f64) -> [f64; 3] {
        [4.0 * x * x, 8.0 * x, 8.0]
    }

    /// States on the hairpin at `xs`.
    fn on_hairpin(xs: &[f64]) -> Vec<[f64; 3]> {
        xs.iter().map(|&x| [x, hairpin(x)[0], 0.0]).collect()
    }

    /// A locus with a hole: `y = 0` but for `|x| < 0.1`, where the
    /// residual is no number, and the surfaces cross at a right angle
    /// everywhere. A midpoint in the hole does not settle.
    fn holed(x: f64) -> [f64; 3] {
        if x.abs() < 0.1 {
            [f64::NAN, 0.0, 0.0]
        } else {
            [0.0; 3]
        }
    }

    /// The plane `z = 0` and the wall `z = y(y − g)` in ℝ³: two branches,
    /// the lines `y = 0` and `y = g`, and between them a sliver where the
    /// wall's normal turns onto the plane's, parallel at `y = g/2`.
    struct SliverR3 {
        g: f64,
    }

    impl LocalSystem<2, 3> for SliverR3 {
        fn residual(&self, x: &[f64; 3]) -> [f64; 2] {
            [x[2], x[2] - x[1] * (x[1] - self.g)]
        }

        fn jacobian(&self, x: &[f64; 3]) -> [[f64; 3]; 2] {
            [[0.0, 0.0, 1.0], [0.0, self.g - 2.0 * x[1], 1.0]]
        }

        fn rhs2(&self, _x: &[f64; 3], d1: &[f64; 3]) -> [f64; 2] {
            [0.0, 2.0 * d1[1] * d1[1]]
        }

        fn rhs3(&self, _x: &[f64; 3], d1: &[f64; 3], d2: &[f64; 3]) -> [f64; 2] {
            [0.0, 6.0 * d1[1] * d2[1]]
        }

        fn point(&self, x: &[f64; 3]) -> Point3<f64> {
            Point3::from_array(*x)
        }

        fn coordinate_scale(&self, _x: &[f64; 3]) -> [f64; 3] {
            [1.0; 3]
        }

        fn tangent_speed(&self, _x: &[f64; 3], d: &[f64; 3]) -> f64 {
            Vec3::from_array(*d).norm()
        }

        fn carrier_jet(
            &self,
            _x: &[f64; 3],
            d1: &[f64; 3],
            d2: &[f64; 3],
            d3: &[f64; 3],
        ) -> [Vec3<f64>; 3] {
            [*d1, *d2, *d3].map(Vec3::from_array)
        }
    }

    impl TransversalityData<3> for SliverR3 {
        fn normals(&self, x: &[f64; 3]) -> NormalPair {
            (
                Vec3::new(0.0, 0.0, 1.0),
                Vec3::new(0.0, self.g - 2.0 * x[1], 1.0),
            )
        }

        fn lever_arm(&self, _x: &[f64; 3]) -> f64 {
            1.0
        }
    }

    /// **The fit's minimum halves the longest gap to the cubic's four
    /// samples, keeping a midpoint wherever it settles.** On the
    /// hairpin, two states 0.2 apart across the vertex take four samples,
    /// each settled onto the locus; two states 1 apart across it keep
    /// the vertex, 1 from the chord's midpoint, as a sample, which the
    /// fit and the certificate decide. Two states the band apart are too
    /// short to halve: the sized refusal in their length, bounded by the
    /// lane (ℝ³, the slab).
    #[test]
    fn the_fits_minimum_halves_to_four_keeping_every_settled_midpoint() {
        use super::fit_minimum;
        use crate::ssi::march::tests::GraphR3;
        let band = Band::new(1.0e-9, 1.0e-8).unwrap();
        let ctx = kernel_ctx(band);
        let sys = GraphR3(hairpin);
        for xs in [[-0.1, 0.1], [-0.5, 0.5]] {
            let four = fit_minimum(&sys, on_hairpin(&xs), &ctx, band)
                .unwrap_or_else(|e| panic!("{xs:?}: {e}"));
            assert_eq!(four.len(), 4, "{xs:?}: the cubic's four: {four:?}");
            assert!(
                four.iter()
                    .all(|s| (s[1] - hairpin(s[0])[0]).abs() <= ctx.tol.settling()),
                "{xs:?}: every sample on the locus: {four:?}"
            );
            assert!(
                four.iter().any(|s| s[0].abs() < 1e-9),
                "{xs:?}: the vertex kept: {four:?}"
            );
        }
        let sys = FixedSpeedR3::at_speed(1.0);
        match fit_minimum(&sys, vec![[0.0; 3], [2e-9, 0.0, 0.0]], &ctx, band) {
            Err(SsiError::ShortBranchUncertified {
                length,
                limb: None,
                bounded_by: BranchBound::Slab,
                ..
            }) => assert!((length - 2e-9).abs() < 1e-18, "its length {length:e}"),
            other => panic!("two states the band apart: expected the sized refusal, got {other:?}"),
        }
    }

    /// **A midpoint that does not settle is read by the transversality
    /// decision at the gap's chord midpoint.** Across the hole, where the
    /// surfaces cross at a right angle, the gap is one nothing halves:
    /// the fit's minimum stops with nothing to halve, one gap unsettled,
    /// the slab's limit its refusal; refinement counts it and halves the
    /// rest. Across the sliver between the two lines `0.1` apart, the
    /// surfaces are tangent at the chord's midpoint, and both refuse
    /// there by the transversality decision, whose ending is the clearer
    /// angle.
    #[test]
    fn a_midpoint_that_does_not_settle_is_read_by_the_transversality_at_its_chord() {
        use super::fit_minimum;
        use crate::recourse::Reading;
        use crate::ssi::march::tests::GraphR3;
        let band = Band::new(1.0e-9, 1.0e-8).unwrap();
        let ctx = kernel_ctx(band);
        let sys = GraphR3(holed);
        match fit_minimum(&sys, vec![[-0.5, 0.0, 0.0], [0.5, 0.0, 0.0]], &ctx, band) {
            Err(SsiError::RefinementExhausted {
                stop:
                    RefineStop::NothingToHalve {
                        in_band: 0,
                        unsettled: 1,
                        off_domain: 0,
                    },
                refusal,
                ..
            }) => assert!(
                matches!(*refusal, SsiError::TraceUnresolved { .. }),
                "the slab's limit: {refusal:?}"
            ),
            other => panic!("across the hole: expected nothing to halve, got {other:?}"),
        }
        let states: Vec<[f64; 3]> = [-0.8, -0.4, 0.4, 0.8].map(|x| [x, 0.0, 0.0]).to_vec();
        let r = refine_by_certificate(&sys, states, &ctx, band, |s: &[[f64; 3]], limbs| {
            if limbs == Limbs::Tube {
                return Ok(());
            }
            match s.windows(2).find(|g| g[0][0] < 0.0 && g[1][0] > 0.0) {
                Some(g) => Err::<(), _>(refusal(g[0][0] + 1e-12, g[1][0] - 1e-12, 1.0)),
                None => Ok(()),
            }
        });
        assert!(
            matches!(
                r,
                Err(SsiError::RefinementExhausted {
                    stop: RefineStop::NothingToHalve {
                        unsettled: 1,
                        off_domain: 0,
                        ..
                    },
                    ..
                })
            ),
            "refinement across the hole: {r:?}"
        );
        let sys = SliverR3 { g: 0.1 };
        let across = vec![[0.0, 0.0, 0.0], [0.1, 0.1, 0.0]];
        let e = fit_minimum(&sys, across, &ctx, band).unwrap_err();
        assert!(
            matches!(e, SsiError::TransversalityBand { .. }),
            "the fit's minimum across the sliver: {e:?}"
        );
        assert!(
            e.ending(Reading::Build)
                .contains("move the geometry so the surfaces cross at a clearer angle"),
            "the clearer angle: {}",
            e.ending(Reading::Build)
        );
        let states = vec![
            [-0.2, 0.0, 0.0],
            [-0.1, 0.0, 0.0],
            [0.0, 0.0, 0.0],
            [0.1, 0.1, 0.0],
            [0.2, 0.1, 0.0],
        ];
        let r = refine_by_certificate(&sys, states, &ctx, band, |_: &[[f64; 3]], _| {
            Err::<(), _>(refusal(-1.0, 1.0, 1.0))
        });
        assert!(
            matches!(r, Err(SsiError::TransversalityBand { .. })),
            "refinement across the sliver: {r:?}"
        );
    }

    /// The plane × NURBS lane's shape of [`holed`]: the line `a = b = c =
    /// 0` in a state `(s, a, b, c)`, with a hole for `|s| < 0.1` where the
    /// residual is no number, and the surfaces crossing at a right angle.
    struct HoledR4;

    impl LocalSystem<3, 4> for HoledR4 {
        fn residual(&self, x: &[f64; 4]) -> [f64; 3] {
            let r = [x[1], x[2], x[3]];
            if x[0].abs() < 0.1 { [f64::NAN; 3] } else { r }
        }

        fn jacobian(&self, _x: &[f64; 4]) -> [[f64; 4]; 3] {
            [
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ]
        }

        fn rhs2(&self, _x: &[f64; 4], _d1: &[f64; 4]) -> [f64; 3] {
            [0.0; 3]
        }

        fn rhs3(&self, _x: &[f64; 4], _d1: &[f64; 4], _d2: &[f64; 4]) -> [f64; 3] {
            [0.0; 3]
        }

        fn point(&self, x: &[f64; 4]) -> Point3<f64> {
            Point3::new(x[0], x[1], x[2])
        }

        fn coordinate_scale(&self, _x: &[f64; 4]) -> [f64; 4] {
            [1.0; 4]
        }

        fn tangent_speed(&self, _x: &[f64; 4], d: &[f64; 4]) -> f64 {
            d[0].abs()
        }

        fn carrier_jet(
            &self,
            _x: &[f64; 4],
            d1: &[f64; 4],
            d2: &[f64; 4],
            d3: &[f64; 4],
        ) -> [Vec3<f64>; 3] {
            [*d1, *d2, *d3].map(|d| Vec3::new(d[0], d[1], d[2]))
        }
    }

    impl TransversalityData<4> for HoledR4 {
        fn normals(&self, _x: &[f64; 4]) -> NormalPair {
            (Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0, 0.0, 1.0))
        }

        fn lever_arm(&self, _x: &[f64; 4]) -> f64 {
            1.0
        }
    }

    /// **On the plane × NURBS lane, a polyline short of the fit's samples
    /// that nothing halves says what its midpoints did.** Two states
    /// across the hole of [`HoledR4`], 1 apart: the midpoint does not
    /// settle, the surfaces crossing at a right angle there, so the
    /// refusal is the march's samples short of the fit, one gap
    /// unsettled; with a third state the band past the second, one gap in
    /// the band does not make the branch short while the other does not
    /// settle. Two states the band apart are short: the sized refusal,
    /// the wall's.
    #[test]
    fn a_wall_polyline_short_of_the_fit_says_what_its_midpoints_did() {
        use super::fit_minimum;
        let band = Band::new(1.0e-9, 1.0e-8).unwrap();
        let ctx = MarchContext::<4> {
            domain: [[-1.0, 1.0]; 4],
            extent: 1.0,
            tol: kernel_ctx(band).tol,
            max_steps: SSI_MAX_STEPS,
            spent: Default::default(),
        };
        let across = vec![[-0.5, 0.0, 0.0, 0.0], [0.5, 0.0, 0.0, 0.0]];
        match fit_minimum(&HoledR4, across, &ctx, band) {
            Err(SsiError::MarchShortOfFit {
                samples: 2,
                in_band: 0,
                unsettled: 1,
                off_domain: 0,
            }) => {}
            other => panic!("across the hole: expected the march short of the fit, got {other:?}"),
        }
        let mixed = vec![
            [-0.5, 0.0, 0.0, 0.0],
            [0.5, 0.0, 0.0, 0.0],
            [0.5 + 2e-9, 0.0, 0.0, 0.0],
        ];
        match fit_minimum(&HoledR4, mixed, &ctx, band) {
            Err(SsiError::MarchShortOfFit {
                samples: 3,
                in_band: 1,
                unsettled: 1,
                off_domain: 0,
            }) => {}
            other => panic!("one gap in the band, one across the hole: not short, got {other:?}"),
        }
        let short = vec![[0.5, 0.0, 0.0, 0.0], [0.5 + 2e-9, 0.0, 0.0, 0.0]];
        assert!(
            matches!(
                fit_minimum(&HoledR4, short, &ctx, band),
                Err(SsiError::ShortBranchUncertified {
                    bounded_by: BranchBound::Wall,
                    ..
                })
            ),
            "two states the band apart on the wall: the sized refusal"
        );
    }
}
