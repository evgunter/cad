//! **The plane × NURBS lane's open branches, between known ends.**
//!
//! The boundary pass (`super::boundary`) hands over every crossing of
//! the locus with the wall's knot rectangle, settled onto both surfaces.
//! An open branch runs from one crossing to another, so its two ends are
//! known before any march, at ε, whatever the caller's extent:
//!
//! - From a crossing `A`, the simplest candidate is tried first: the
//!   Hermite cubic from `A` to `B`, the nearest crossing not yet used,
//!   through their tangents, in both charts and in space. It is one span
//!   and exact at both ends, and its two states are the ones the march's
//!   transversality decision reads (`ssi_transversality`); whether `B`
//!   is the branch's other end is the certificate's to decide, on the
//!   chart lane, whose limb 3 proves the tube holds one arc.
//! - Where the certificate refuses it, the march leaves `A` inward, its
//!   step the curvature's against ε, kept only where its predicted state
//!   lies on the locus (`super::march`). It stops at its first state
//!   outside the rectangle, and that step is matched to the unused
//!   crossing on the side it left within the step's reach, the one
//!   nearest where the step's chord meets the side where two are
//!   (branches converging on a side). A march that leaves where no
//!   crossing matches refuses as the march's limit
//!   ([`SsiError::CrossingUnmatched`]).
//!
//! The Hermite and the march are two candidate generators, each trusted
//! for nothing; C2's three limbs decide either. The march's states reach
//! the fit with at least the cubic's four samples, by halving
//! (`super::refine::fit_minimum`). Where neither candidate certifies,
//! the march's refusal stands, except where its states were too short to
//! halve, a sized refusal in the length along them
//! ([`SsiError::ShortBranchUncertified`]), or it refused for want of
//! step, the step's refusal ([`SsiError::MarchStepInBand`]); both carry
//! the Hermite's ([`neither`]).

use geom::{Curve3, NurbsCurve2, NurbsCurve3, Surface};
use geom_core::linalg::svd::Svd;
use geom_core::spline::KnotVector;
use geom_core::{Band, Margin, Point2, Point3};

use super::boundary::Crossing;
use super::march::{
    BranchEnd, MarchContext, RectEnd, RectExit, StepperMode, decide_transversality, march,
};
use super::refine::refine_by_certificate;
use super::section::{BandVerdict, band_verdict};
use super::system::{LocalSystem, ParametricPairR4};
use super::{
    FittedBranch, SsiBranch, SsiError, SsiOperand, TubeScale, certify, fit_branch, seam_tol,
};

/// What the branches between known ends read, minted once per call.
pub(crate) struct Ends<'a> {
    /// The ℝ⁴ system.
    pub sys: &'a ParametricPairR4<'a>,
    /// The march's context, the caller's feature extent among it.
    pub ctx: MarchContext<4>,
    /// The plane operand.
    pub plane: &'a Surface<f64>,
    /// The wall operand.
    pub wall: &'a SsiOperand<'a, f64>,
    /// The run band.
    pub band: Band,
}

/// The fitted triple of a state polyline, on one shared parameter.
///
/// # Errors
///
/// As [`fit_branch`].
pub(crate) fn fit_states(
    sys: &ParametricPairR4<'_>,
    states: &[[f64; 4]],
) -> Result<FittedBranch, SsiError> {
    let points: Vec<Point3<f64>> = states.iter().map(|s| sys.point(s)).collect();
    let chart_a: Vec<Point2<f64>> = states.iter().map(|s| Point2::new(s[0], s[1])).collect();
    let chart_b: Vec<Point2<f64>> = states.iter().map(|s| Point2::new(s[2], s[3])).collect();
    fit_branch(&points, Some((&chart_a, &chart_b)))
}

/// Ends a marched half-branch `states` (its last state the last one
/// inside the wall) at the crossing `end`. A last state nearer `end` than
/// half its own step gives way to it: a final chord far shorter than the
/// steps before it is one the cubic fit amplifies the states' settling
/// residual across, and the march is a candidate generator, so which of
/// its samples the fit reads is a selection, not a decision.
fn close_at<const M: usize, const N: usize, S: LocalSystem<M, N>>(
    sys: &S,
    states: &mut Vec<[f64; N]>,
    end: [f64; N],
) {
    if let [.., before, last] = states.as_slice()
        && states.len() > super::SSI_FIT_DEGREE
        && 2.0 * distance(sys, last, &end) < distance(sys, before, last)
    {
        states.pop();
    }
    states.push(end);
}

/// The refusal of a branch neither candidate certified.
///
/// The march's refusal stands, but for a march too short for the fit.
/// Where the march's samples were too short to halve, their sized
/// refusal in the branch's length carries the Hermite's. Where the march
/// refused for want of step, its step falling in the band (decided, or
/// undecided on a valid margin), the branch may be long and the step is
/// the short quantity: the refusal is the step's
/// ([`SsiError::MarchStepInBand`]), carrying the Hermite's.
///
/// Beside a march that could not step, the Hermite's transversality
/// decision at an end stands instead. A march refuses at its own first
/// state as the Hermite does at that end, so the Hermite refused at the
/// far end, which a march too short to step never reaches.
fn neither(hermite: SsiError, march: SsiError, band: Band) -> SsiError {
    let verdict = match march {
        SsiError::ShortBranchUncertified {
            length,
            limb: None,
            verdict,
            bounded_by,
        } => {
            return SsiError::ShortBranchUncertified {
                length,
                limb: Some(Box::new(hermite)),
                verdict,
                bounded_by,
            };
        }
        // The step guard's own decision on the step it refused, so the
        // same verdict.
        SsiError::StepCollapsed { step_meters, .. } => {
            match band_verdict("ssi_step_progress", Margin::of(step_meters), band) {
                Some(verdict) => verdict,
                None => return march,
            }
        }
        SsiError::Escalated {
            decision: super::TraceDecision::StepProgress,
            ref cause,
        } if !cause.margin.is_invalid() => BandVerdict::Undecided(*cause),
        _ => return march,
    };
    if matches!(
        hermite,
        SsiError::TransversalityBand { .. }
            | SsiError::Escalated {
                decision: super::TraceDecision::Transversality
                    | super::TraceDecision::TransversalityArm,
                ..
            }
    ) {
        return hermite;
    }
    SsiError::MarchStepInBand {
        verdict,
        limb: Box::new(hermite),
    }
}

/// The 3-D distance between two states.
fn distance<const M: usize, const N: usize, S: LocalSystem<M, N>>(
    sys: &S,
    a: &[f64; N],
    b: &[f64; N],
) -> f64 {
    (sys.point(a) - sys.point(b)).norm()
}

/// How far the tangent `d` at state `x` points into the wall's
/// rectangle: the sum of its chart components across every side `x`
/// sits on.
fn inwardness(x: &[f64; 4], d: &[f64; 4], ctx: &MarchContext<4>) -> f64 {
    (2..4)
        .map(|i| {
            if x[i] == ctx.domain[i][0] {
                d[i]
            } else if x[i] == ctx.domain[i][1] {
                -d[i]
            } else {
                0.0
            }
        })
        .sum()
}

/// The unit tangent of the locus at a crossing, pointing into the wall
/// when `into`, out of it otherwise.
fn tangent(
    sys: &ParametricPairR4<'_>,
    x: &[f64; 4],
    ctx: &MarchContext<4>,
    into: bool,
) -> [f64; 4] {
    let d = Svd::<3, 4>::new(sys.jacobian(x)).null_direction();
    if (inwardness(x, &d, ctx) >= 0.0) == into {
        d
    } else {
        d.map(|v| -v)
    }
}

impl<'a> Ends<'a> {
    /// The ends' reader for one plane × NURBS call.
    pub(crate) fn of(
        sys: &'a ParametricPairR4<'a>,
        ctx: MarchContext<4>,
        plane: &'a Surface<f64>,
        wall: &'a SsiOperand<'a, f64>,
        band: Band,
    ) -> Self {
        Self {
            sys,
            ctx,
            plane,
            wall,
            band,
        }
    }

    /// **Every open branch**, each from a crossing to its partner, in the
    /// crossings' order (D9).
    ///
    /// # Errors
    ///
    /// [`SsiError::CrossingUnmatched`] for a crossing with no partner,
    /// or a march whose exit matches none; any refusal of the
    /// march, the fit or the certificate, where the Hermite candidate
    /// was refused too; and [`SsiError::ShortBranchUncertified`] where
    /// neither candidate certifies a branch too short to march or to
    /// halve to the fit's samples ([`neither`]).
    pub(crate) fn branches(&self, crossings: &[Crossing]) -> Result<Vec<SsiBranch>, SsiError> {
        let n = crossings.len();
        let mut used = vec![false; n];
        let mut out = Vec::new();
        for i in 0..n {
            if used[i] {
                continue;
            }
            used[i] = true;
            let a = crossings[i];
            let nearest = (0..n)
                .filter(|&j| !used[j])
                .map(|j| (j, distance(self.sys, &a.state, &crossings[j].state)))
                .min_by(|x, y| x.1.total_cmp(&y.1));
            let Some((j, near)) = nearest else {
                return Err(SsiError::CrossingUnmatched { from: Some(a.at) });
            };
            let branch = match self.hermite(a, crossings[j], near) {
                Ok(branch) => {
                    used[j] = true;
                    branch
                }
                Err(hermite) => match self.marched(a, crossings, &used) {
                    Ok((b, branch)) => {
                        used[b] = true;
                        branch
                    }
                    Err(march) => return Err(neither(hermite, march, self.band)),
                },
            };
            out.push(branch);
        }
        Ok(out)
    }

    /// The marched candidate from crossing `a`: its far crossing's
    /// index, and the branch fitted and certified.
    fn marched(
        &self,
        a: Crossing,
        crossings: &[Crossing],
        used: &[bool],
    ) -> Result<(usize, SsiBranch), SsiError> {
        let (b, states) = self.march_from(a, crossings, used)?;
        let end = BranchEnd::Crossings {
            from: a.at,
            to: crossings[b].at,
        };
        Ok((b, self.finish(&states, end)?))
    }

    /// The march from crossing `a`, inward, to the crossing on the side
    /// it leaves: that crossing's index, and the states.
    fn march_from(
        &self,
        a: Crossing,
        crossings: &[Crossing],
        used: &[bool],
    ) -> Result<(usize, Vec<[f64; 4]>), SsiError> {
        let d = Svd::<3, 4>::new(self.sys.jacobian(&a.state)).null_direction();
        let direction = if inwardness(&a.state, &d, &self.ctx) >= 0.0 {
            1.0
        } else {
            -1.0
        };
        let trace = march(
            self.sys,
            &RectExit,
            a.state,
            self.ctx,
            StepperMode::Realized,
            direction,
            self.band,
        )?;
        let RectEnd::Left { inside, outside } = trace.end else {
            // A march from the wall's boundary that comes back to its
            // start without leaving: no crossing ends it.
            return Err(SsiError::CrossingUnmatched { from: Some(a.at) });
        };
        let b = self.match_exit(Some(a), inside, outside, crossings, used)?;
        let mut states = trace.states;
        close_at(self.sys, &mut states, crossings[b].state);
        Ok((b, states))
    }

    /// The unused crossing on a side the step from `inside` to
    /// `outside` left through, within the step's reach of `inside`: the
    /// one nearest where the step's chord meets that side.
    pub(crate) fn match_exit(
        &self,
        a: Option<Crossing>,
        inside: [f64; 4],
        outside: [f64; 4],
        crossings: &[Crossing],
        used: &[bool],
    ) -> Result<usize, SsiError> {
        let dom = &self.ctx.domain;
        // The sides the step crossed, as (coordinate, end value).
        let mut crossed: Vec<(usize, f64)> = Vec::new();
        for i in 2..4 {
            if outside[i] < dom[i][0] {
                crossed.push((i, dom[i][0]));
            } else if outside[i] > dom[i][1] {
                crossed.push((i, dom[i][1]));
            }
        }
        // The exit lies on the locus between the two states, so within
        // the arc of the step: its chord, with the band for the ends'
        // settling.
        let reach = 1.1 * distance(self.sys, &inside, &outside) + self.band.escalate();
        // The step's chord where it meets the side it crossed: the exit
        // point's estimate, from which the nearest candidate is taken.
        // Two branches converging on a side can both have a crossing in
        // the window; picking the nearer is a candidate's choice, trusted
        // for nothing, and the certificate decides it.
        let exit = |i: usize, end: f64| {
            let t = (end - inside[i]) / (outside[i] - inside[i]);
            core::array::from_fn::<f64, 4, _>(|k| inside[k] + (outside[k] - inside[k]) * t)
        };
        let found = crossings
            .iter()
            .enumerate()
            .filter_map(|(j, c)| {
                if used[j] || distance(self.sys, &inside, &c.state) > reach {
                    return None;
                }
                crossed
                    .iter()
                    .find(|&&(i, end)| c.state[i] == end)
                    .map(|&(i, end)| (j, distance(self.sys, &exit(i, end), &c.state)))
            })
            .min_by(|x, y| x.1.total_cmp(&y.1).then(x.0.cmp(&y.0)));
        match found {
            Some((b, _)) => Ok(b),
            None => Err(SsiError::CrossingUnmatched {
                from: a.map(|c| c.at),
            }),
        }
    }

    /// **The branch through an interior `seed`**: a loop where the march
    /// closes; otherwise marched both ways, each half ending at the
    /// crossing on the side it leaves. The states, and how the branch
    /// ends.
    ///
    /// # Errors
    ///
    /// As [`Ends::branches`].
    pub(crate) fn through_seed(
        &self,
        seed: [f64; 4],
        crossings: &[Crossing],
    ) -> Result<(Vec<[f64; 4]>, BranchEnd), SsiError> {
        let run = |direction, ctx| {
            march(
                self.sys,
                &RectExit,
                seed,
                ctx,
                StepperMode::Realized,
                direction,
                self.band,
            )
        };
        let fwd = run(1.0, self.ctx)?;
        let RectEnd::Left {
            inside: f_in,
            outside: f_out,
        } = fwd.end
        else {
            return Ok((fwd.states, BranchEnd::Closed));
        };
        let bwd = run(-1.0, self.ctx.rest(&fwd))?;
        let RectEnd::Left {
            inside: b_in,
            outside: b_out,
        } = bwd.end
        else {
            return Err(SsiError::CrossingUnmatched { from: None });
        };
        let mut used = vec![false; crossings.len()];
        let to = self.match_exit(None, f_in, f_out, crossings, &used)?;
        used[to] = true;
        let from = self.match_exit(None, b_in, b_out, crossings, &used)?;
        let mut back = bwd.states;
        close_at(self.sys, &mut back, crossings[from].state);
        back.reverse();
        let mut forth = fwd.states;
        close_at(self.sys, &mut forth, crossings[to].state);
        let mut states = back;
        states.extend_from_slice(&forth[1..]);
        let end = BranchEnd::Crossings {
            from: crossings[from].at,
            to: crossings[to].at,
        };
        Ok((states, end))
    }

    /// Fit, certify and wrap a marched branch, refined where the
    /// certificate refuses it ([`refine_by_certificate`]).
    pub(crate) fn finish(
        &self,
        states: &[[f64; 4]],
        end: BranchEnd,
    ) -> Result<SsiBranch, SsiError> {
        let march_tol = seam_tol(self.ctx.tol, self.band)?;
        let (carrier, pa, pb, cert) = refine_by_certificate(
            self.sys,
            states.to_vec(),
            &self.ctx,
            self.band,
            |states, limbs| {
                let (carrier, pa, pb) = fit_states(self.sys, states)?;
                let (SsiOperand::Nurbs(wall), Some(pcurve)) = (self.wall, pb.as_ref()) else {
                    return Err(SsiError::UnsupportedCertificate {
                        what: certify::NURBS_LIMBS_NEED_PCURVE,
                    }
                    .into());
                };
                let cert = certify::certify_located(
                    &carrier,
                    certify::Lane::Chart {
                        plane: self.plane,
                        wall: *wall,
                        pcurve,
                    },
                    TubeScale::uniform(self.ctx.extent),
                    self.band,
                    limbs,
                )?;
                Ok((carrier, pa, pb, cert))
            },
        )?;
        Ok(self.branch(carrier, pa, pb, cert, end, march_tol))
    }

    /// The certificate on a fitted triple, and the branch.
    fn certified(
        &self,
        carrier: NurbsCurve3<f64>,
        pa: Option<NurbsCurve2<f64>>,
        pb: Option<NurbsCurve2<f64>>,
        end: BranchEnd,
        march_tol: f64,
    ) -> Result<SsiBranch, SsiError> {
        let (SsiOperand::Nurbs(wall), Some(pcurve)) = (self.wall, pb.as_ref()) else {
            return Err(SsiError::UnsupportedCertificate {
                what: certify::NURBS_LIMBS_NEED_PCURVE,
            });
        };
        let cert = certify::certify_branch(
            &carrier,
            certify::Lane::Chart {
                plane: self.plane,
                wall: *wall,
                pcurve,
            },
            TubeScale::uniform(self.ctx.extent),
            self.band,
            certify::Limbs::All,
            &mut Vec::new(),
        )?;
        Ok(self.branch(carrier, pa, pb, cert, end, march_tol))
    }

    /// The branch a certified triple is.
    fn branch(
        &self,
        carrier: NurbsCurve3<f64>,
        pa: Option<NurbsCurve2<f64>>,
        pb: Option<NurbsCurve2<f64>>,
        cert: certify::SsiCertificate<f64>,
        end: BranchEnd,
        march_tol: f64,
    ) -> SsiBranch {
        let params = carrier.domain();
        let carrier = Curve3::Nurbs(std::sync::Arc::new(carrier));
        let witness = carrier.mid_point(params.0, params.1);
        SsiBranch {
            carrier,
            params,
            end,
            certificate: cert,
            witness,
            pcurve_a: pa,
            pcurve_b: pb,
            march_tol,
        }
    }

    /// **The Hermite candidate**: the cubic from `a` to `b`
    /// through their tangents, each scaled to the ends' distance, as one
    /// cubic Bézier per chart on a shared parameter. The plane's chart
    /// is affine, so the carrier is that chart's image of its own
    /// Bézier, exactly.
    ///
    /// # Errors
    ///
    /// The march tolerance's refusal ([`seam_tol`]), the transversality
    /// decision's at either end, as the march's at a state
    /// ([`decide_transversality`]), and the certificate's refusal of the
    /// candidate, after any of which the caller marches.
    fn hermite(&self, a: Crossing, b: Crossing, length: f64) -> Result<SsiBranch, SsiError> {
        let march_tol = seam_tol(self.ctx.tol, self.band)?;
        for x in [&a.state, &b.state] {
            let sigma = Svd::<3, 4>::new(self.sys.jacobian(x)).sigma_min();
            decide_transversality(self.sys, x, sigma, self.ctx.extent, self.band)?;
        }
        let ta = tangent(self.sys, &a.state, &self.ctx, true);
        let tb = tangent(self.sys, &b.state, &self.ctx, false);
        let va = ta.map(|v| v * length / self.sys.tangent_speed(&a.state, &ta));
        let vb = tb.map(|v| v * length / self.sys.tangent_speed(&b.state, &tb));
        let ctrl: [[f64; 4]; 4] = [
            a.state,
            core::array::from_fn(|i| a.state[i] + va[i] / 3.0),
            core::array::from_fn(|i| b.state[i] - vb[i] / 3.0),
            b.state,
        ];
        let bezier = || KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], 3);
        let built = (|| {
            let carrier = NurbsCurve3::new(
                bezier()?,
                ctrl.iter().map(|s| self.sys.point(s)).collect(),
                vec![1.0; 4],
            )?;
            let pa = NurbsCurve2::new(
                bezier()?,
                ctrl.iter().map(|s| Point2::new(s[0], s[1])).collect(),
                vec![1.0; 4],
            )?;
            let pb = NurbsCurve2::new(
                bezier()?,
                ctrl.iter().map(|s| Point2::new(s[2], s[3])).collect(),
                vec![1.0; 4],
            )?;
            Ok::<_, geom_core::spline::SplineError>((carrier, pa, pb))
        })();
        let Ok((carrier, pa, pb)) = built else {
            // A non-finite tangent scale: the certificate has nothing to
            // decide.
            return Err(SsiError::UnsupportedCertificate {
                what: "the Hermite candidate is not a finite cubic",
            });
        };
        let end = BranchEnd::Crossings {
            from: a.at,
            to: b.at,
        };
        self.certified(carrier, Some(pa), Some(pb), end, march_tol)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use geom_core::{Band, Indeterminate, MarginDiag};

    use super::{close_at, neither};
    use crate::recourse::{Classified, Refused};
    use crate::ssi::march::tests::FixedSpeedR3;
    use crate::ssi::section::BandVerdict;
    use crate::ssi::{BranchBound, SsiError, SsiLimb, TraceDecision};

    /// **A branch neither candidate certifies is short only where the
    /// march's own states were.** The march's sized refusal carries the
    /// Hermite's. Beside a march whose step fell in the band, decided or
    /// undecided, the refusal is the step's, carrying the Hermite's and
    /// the step's verdict, never a branch length. The
    /// Hermite's transversality decision at an end stands over a march
    /// that could not step. Beside a tangency, an undecided
    /// transversality, an undecided step on a margin that is no number,
    /// or a certificate's refusal, the march's refusal is returned.
    #[test]
    fn a_branch_is_sized_short_only_where_its_march_states_were() {
        let band = Band::new(1e-9, 1e-8).unwrap();
        let cause = |margin| Indeterminate {
            margin,
            band,
            predicate: None,
            terminal_sliver: false,
        };
        let limb = || SsiError::CertificateLimb {
            limb: SsiLimb::OnLocus,
            value: 3e-9,
        };
        let step = |margin| SsiError::Escalated {
            decision: TraceDecision::StepProgress,
            cause: cause(margin),
        };
        let collapsed = || SsiError::StepCollapsed {
            mode: "realized",
            step_meters: 6e-10,
            speed: 1.0,
        };
        for (what, march, refused) in [
            ("the step collapsed", collapsed(), true),
            ("the step undecided", step(MarginDiag::value(6e-9)), false),
        ] {
            let e = neither(limb(), march, band);
            let SsiError::MarchStepInBand {
                limb: ref hermite,
                ref verdict,
            } = e
            else {
                panic!("{what}: the step's refusal, got {e:?}");
            };
            assert!(
                matches!(**hermite, SsiError::CertificateLimb { .. }),
                "{what}: the Hermite's refusal in it: {hermite:?}"
            );
            assert_eq!(
                matches!(verdict, BandVerdict::Refused(_)),
                refused,
                "{what}: the step's verdict {verdict:?}"
            );
        }
        let marched_short = SsiError::ShortBranchUncertified {
            length: 1.5e-8,
            limb: None,
            verdict: BandVerdict::Undecided(cause(MarginDiag::value(7e-9))),
            bounded_by: BranchBound::Wall,
        };
        let e = neither(limb(), marched_short, band);
        assert!(
            matches!(
                &e,
                SsiError::ShortBranchUncertified {
                    length,
                    limb: Some(_),
                    ..
                } if *length == 1.5e-8
            ),
            "the march's states too short to halve: theirs, the Hermite's in it: {e}"
        );
        let tangency = SsiError::TransversalityBand {
            sin_theta: 5e-10,
            arm: 1.0,
            sigma_min: 5e-10,
            verdict: Refused::Zero(Classified {
                margin: MarginDiag::value(5e-10),
                band,
            }),
        };
        for (what, march) in [
            ("a tangency", tangency),
            (
                "an undecided transversality",
                SsiError::Escalated {
                    decision: TraceDecision::Transversality,
                    cause: cause(MarginDiag::value(5e-9)),
                },
            ),
            ("an invalid step margin", step(MarginDiag::INVALID)),
            ("a certificate's refusal", limb()),
        ] {
            let e = neither(limb(), march, band);
            assert!(
                !matches!(
                    e,
                    SsiError::ShortBranchUncertified { .. } | SsiError::MarchStepInBand { .. }
                ),
                "{what}: the march's refusal, got {e}"
            );
        }
        let far_end = SsiError::Escalated {
            decision: TraceDecision::Transversality,
            cause: cause(MarginDiag::value(5e-9)),
        };
        let e = neither(far_end, collapsed(), band);
        assert!(
            matches!(
                e,
                SsiError::Escalated {
                    decision: TraceDecision::Transversality,
                    ..
                }
            ),
            "the Hermite's far end undecided beside a collapsed step: the far end's, got {e}"
        );
    }

    /// **A last state short of its end gives way to it.** Five states
    /// 0.2 apart up the `x` axis, the last `0.8`, end at `0.8 + δ`: a last
    /// state nearer the end than half its own step (δ from 1e-11 to
    /// 0.099) is dropped, so the fit never reads a final chord far
    /// shorter than the steps before it; one half a step or more short
    /// (δ 0.1, 0.15) is kept. A march of fewer states than the cubic
    /// needs keeps every one.
    #[test]
    fn a_last_state_short_of_its_end_gives_way_to_it() {
        let sys = FixedSpeedR3::at_speed(1.0);
        let marched =
            |n: usize| -> Vec<[f64; 3]> { (0..n).map(|i| [0.2 * i as f64, 0.0, 0.0]).collect() };
        for (delta, kept) in [
            (1e-11, false),
            (1e-6, false),
            (1e-3, false),
            (0.099, false),
            (0.1, true),
            (0.15, true),
        ] {
            let mut states = marched(5);
            let end = [0.8 + delta, 0.0, 0.0];
            close_at(&sys, &mut states, end);
            let want = if kept { 6 } else { 5 };
            assert_eq!(states.len(), want, "δ = {delta:e}: {states:?}");
            assert_eq!(states.last(), Some(&end), "δ = {delta:e}: ends at its end");
            assert_eq!(
                states[states.len() - 2][0] == 0.8,
                kept,
                "δ = {delta:e}: the last state kept"
            );
        }
        let mut short = marched(3);
        close_at(&sys, &mut short, [0.4 + 1e-9, 0.0, 0.0]);
        assert_eq!(short.len(), 4, "three states and the end: {short:?}");
    }
}
