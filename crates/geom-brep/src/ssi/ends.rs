//! **The plane × NURBS lane's open branches, between known ends.**
//!
//! The boundary pass (`super::boundary`) hands over every crossing of
//! the locus with the wall's knot rectangle, settled onto both surfaces.
//! An open branch runs from one crossing to another, so its two ends are
//! known before any march, at ε, whatever the caller's extent:
//!
//! - From a crossing `A`, the march leaves inward with its step capped
//!   at `|AB|/`[`SHORT_BRANCH_STEPS`], `B` the crossing nearest `A`, so
//!   the branch it reaches is cut into at least that many steps. It
//!   stops at its first state outside the rectangle, and that step is
//!   matched to the one crossing on the side it left, within the step's
//!   reach. A match that is missing or doubled contradicts two passes
//!   that each certified their part, and refuses as the kernel's
//!   ([`SsiError::CrossingUnmatched`]).
//! - Where `|AB|` is below [`SSI_SHORT_CLIP`]`·Kε`, no step of it can
//!   clear the band, so the candidate is not marched: it is the Hermite
//!   cubic through the two certified ends and their tangents, in both
//!   charts and in space. The march and the Hermite are two candidate
//!   generators, each trusted for nothing; C2's three limbs decide
//!   either, and a short candidate they refuse is a sized refusal in its
//!   length ([`SsiError::ShortBranchUncertified`]).

use geom::{Curve3, NurbsCurve2, NurbsCurve3, Surface};
use geom_core::linalg::svd::Svd;
use geom_core::spline::KnotVector;
use geom_core::{Band, Margin, Point2, Point3, Real};

use super::boundary::Crossing;
use super::march::{
    BranchEnd, MarchContext, RectEnd, RectExit, SHORT_BRANCH_STEPS, SSI_STEP_MAX, StepperMode,
    TransversalityData, march,
};
use super::section::BandVerdict;
use super::system::{LocalSystem, ParametricPairR4};
use super::{
    FittedBranch, SsiBranch, SsiError, SsiOperand, TubeScale, certify, fit_branch, seam_tol,
};
use crate::dihedral::decide_reported;
use crate::recourse::Refused;

/// **The short-clip rule** (D9): a branch whose two certified ends are
/// less than this many `Kε` apart takes the Hermite cubic through them
/// as its candidate instead of a march. It is [`SHORT_BRANCH_STEPS`]:
/// below it, a step of `|AB|/SHORT_BRANCH_STEPS` cannot clear the band
/// the march's step progress is decided in.
pub const SSI_SHORT_CLIP: f64 = 5.0;

const _: () = assert!(SSI_SHORT_CLIP == SHORT_BRANCH_STEPS as f64);

/// What the branches between known ends read, minted once per call.
pub(crate) struct Ends<'a> {
    /// The ℝ⁴ system.
    pub sys: &'a ParametricPairR4<'a>,
    /// The march's context.
    pub ctx: MarchContext<4>,
    /// The plane operand.
    pub plane: &'a Surface<f64>,
    /// The wall operand.
    pub wall: &'a SsiOperand<'a, f64>,
    /// The caller's feature extent.
    pub extent: f64,
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
fn close_at(sys: &ParametricPairR4<'_>, states: &mut Vec<[f64; 4]>, end: [f64; 4]) {
    if let [.., before, last] = states.as_slice()
        && states.len() > super::SSI_FIT_DEGREE
        && 2.0 * distance(sys, last, &end) < distance(sys, before, last)
    {
        states.pop();
    }
    states.push(end);
}

/// The 3-D distance between two states.
fn distance<S: LocalSystem<3, 4>>(sys: &S, a: &[f64; 4], b: &[f64; 4]) -> f64 {
    (sys.point(a) - sys.point(b)).norm()
}

/// How far the tangent `d` at state `x` points into the wall's
/// rectangle: the sum of its chart components across every side `x`
/// sits on.
fn inward(x: &[f64; 4], d: &[f64; 4], ctx: &MarchContext<4>) -> f64 {
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
    if (inward(x, &d, ctx) >= 0.0) == into {
        d
    } else {
        d.map(|v| -v)
    }
}

impl Ends<'_> {
    /// **Every open branch**, each from a crossing to its partner, in the
    /// crossings' order (D9).
    ///
    /// # Errors
    ///
    /// [`SsiError::CrossingUnmatched`] for a crossing with no partner,
    /// or a march whose exit matches none or two; any refusal of the
    /// march, the fit or the certificate; and
    /// [`SsiError::ShortBranchUncertified`] for a short candidate the
    /// certificate refuses.
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
                return Err(SsiError::CrossingUnmatched {
                    from: Some(a.at),
                    matches: 0,
                });
            };
            let branch = if near < SSI_SHORT_CLIP * self.band.escalate() {
                used[j] = true;
                self.hermite(a, crossings[j], near)?
            } else {
                let (b, states, min_t) = self.march_from(a, near, crossings, &used)?;
                used[b] = true;
                let end = BranchEnd::Crossings {
                    from: a.at,
                    to: crossings[b].at,
                };
                self.finish(&states, end, min_t)?
            };
            out.push(branch);
        }
        Ok(out)
    }

    /// The march from crossing `a`, inward, to the crossing on the side
    /// it leaves: that crossing's index, the states, and the march's
    /// smallest transversality.
    fn march_from(
        &self,
        a: Crossing,
        near: f64,
        crossings: &[Crossing],
        used: &[bool],
    ) -> Result<(usize, Vec<[f64; 4]>, f64), SsiError> {
        let cap = Real::min(SSI_STEP_MAX * self.extent, near / SHORT_BRANCH_STEPS as f64);
        let d = Svd::<3, 4>::new(self.sys.jacobian(&a.state)).null_direction();
        let direction = if inward(&a.state, &d, &self.ctx) >= 0.0 {
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
            cap,
        )?;
        let RectEnd::Left { inside, outside } = trace.end else {
            // A march from the wall's boundary that comes back to its
            // start without leaving: no crossing ends it.
            return Err(SsiError::CrossingUnmatched {
                from: Some(a.at),
                matches: 0,
            });
        };
        let b = self.match_exit(Some(a), inside, outside, crossings, used)?;
        let mut states = trace.states;
        close_at(self.sys, &mut states, crossings[b].state);
        Ok((b, states, trace.min_transversality))
    }

    /// The one unused crossing on a side the step from `inside` to
    /// `outside` left through, within the step's reach of `inside`.
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
        let found: Vec<usize> = crossings
            .iter()
            .enumerate()
            .filter(|(j, c)| {
                !used[*j]
                    && crossed.iter().any(|&(i, end)| c.state[i] == end)
                    && distance(self.sys, &inside, &c.state) <= reach
            })
            .map(|(j, _)| j)
            .collect();
        match found.as_slice() {
            [b] => Ok(*b),
            _ => Err(SsiError::CrossingUnmatched {
                from: a.map(|c| c.at),
                matches: found.len(),
            }),
        }
    }

    /// **The branch through an interior `seed`**: a loop where the march
    /// closes; otherwise marched both ways, each half ending at the
    /// crossing on the side it leaves. The states, how the branch ends,
    /// and the smallest transversality the marches read.
    ///
    /// # Errors
    ///
    /// As [`Ends::branches`].
    pub(crate) fn through_seed(
        &self,
        seed: [f64; 4],
        crossings: &[Crossing],
        cap: f64,
    ) -> Result<(Vec<[f64; 4]>, BranchEnd, f64), SsiError> {
        let run = |direction| {
            march(
                self.sys,
                &RectExit,
                seed,
                self.ctx,
                StepperMode::Realized,
                direction,
                self.band,
                cap,
            )
        };
        let fwd = run(1.0)?;
        let RectEnd::Left {
            inside: f_in,
            outside: f_out,
        } = fwd.end
        else {
            return Ok((fwd.states, BranchEnd::Closed, fwd.min_transversality));
        };
        let bwd = run(-1.0)?;
        let RectEnd::Left {
            inside: b_in,
            outside: b_out,
        } = bwd.end
        else {
            return Err(SsiError::CrossingUnmatched {
                from: None,
                matches: 0,
            });
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
        let min_t = Real::min(fwd.min_transversality, bwd.min_transversality);
        Ok((states, end, min_t))
    }

    /// Fit, certify and wrap a marched branch.
    pub(crate) fn finish(
        &self,
        states: &[[f64; 4]],
        end: BranchEnd,
        min_transversality: f64,
    ) -> Result<SsiBranch, SsiError> {
        let march_tol = seam_tol(self.ctx.tol, self.band)?;
        let (carrier, pa, pb) = fit_states(self.sys, states)?;
        self.certified(carrier, pa, pb, end, min_transversality, march_tol)
    }

    /// The certificate on a fitted triple, and the branch.
    #[allow(clippy::too_many_arguments)]
    fn certified(
        &self,
        carrier: NurbsCurve3<f64>,
        pa: Option<NurbsCurve2<f64>>,
        pb: Option<NurbsCurve2<f64>>,
        end: BranchEnd,
        min_transversality: f64,
        march_tol: f64,
    ) -> Result<SsiBranch, SsiError> {
        let cert = certify::certify_branch(
            &carrier,
            pb.as_ref(),
            &SsiOperand::Analytic(self.plane),
            self.wall,
            TubeScale::uniform(self.extent),
            self.band,
        )?;
        let params = carrier.domain();
        let carrier = Curve3::Nurbs(std::sync::Arc::new(carrier));
        let witness = carrier.mid_point(params.0, params.1);
        Ok(SsiBranch {
            carrier,
            params,
            end,
            certificate: cert,
            witness,
            pcurve_a: pa,
            pcurve_b: pb,
            min_transversality,
            march_tol,
        })
    }

    /// The transversality `sin θ · arm` at a state, as the march reads
    /// it: reported on a branch that was not marched.
    fn transversality(&self, x: &[f64; 4]) -> f64 {
        let (n1, n2) = self.sys.normals(x);
        let sin_theta = n1.cross(n2).norm() / (n1.norm() * n2.norm());
        let arm = Real::min(self.sys.lever_arm(x), self.extent);
        Margin::levered(sin_theta, arm).value()
    }

    /// **The short clip's candidate**: the Hermite cubic from `a` to `b`
    /// through their tangents, each scaled to the ends' distance, as one
    /// cubic Bézier per chart on a shared parameter. The plane's chart
    /// is affine, so the carrier is that chart's image of its own
    /// Bézier, exactly.
    ///
    /// # Errors
    ///
    /// [`SsiError::ShortBranchUncertified`] when the certificate refuses
    /// the candidate.
    fn hermite(&self, a: Crossing, b: Crossing, length: f64) -> Result<SsiBranch, SsiError> {
        let march_tol = seam_tol(self.ctx.tol, self.band)?;
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
            // decide, and the length is what the user holds.
            return Err(self.short_refusal(
                length,
                SsiError::UnsupportedCertificate {
                    what: "the short clip's Hermite candidate is not a finite cubic",
                },
            ));
        };
        let min_t = Real::min(self.transversality(&a.state), self.transversality(&b.state));
        let end = BranchEnd::Crossings {
            from: a.at,
            to: b.at,
        };
        self.certified(carrier, Some(pa), Some(pb), end, min_t, march_tol)
            .map_err(|e| self.short_refusal(length, e))
    }

    /// The sized refusal of a short candidate of `length`: the length
    /// over [`SSI_SHORT_CLIP`] is the step it would be marched at, and a
    /// tolerance whose band that step clears marches it instead.
    fn short_refusal(&self, length: f64, limb: SsiError) -> SsiError {
        let step = Margin::of(length / SSI_SHORT_CLIP);
        let verdict = match decide_reported("ssi_short_branch", step, self.band) {
            Ok(decided) => match Refused::of(decided, self.band) {
                Some(r) => BandVerdict::Refused(r),
                // A length that clears the band was not short.
                None => return limb,
            },
            Err(cause) => BandVerdict::Undecided(cause),
        };
        SsiError::ShortBranchUncertified {
            length,
            limb: Box::new(limb),
            verdict,
        }
    }
}
