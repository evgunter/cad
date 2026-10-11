//! **What the two ℝ⁴ section handles share.** The plane × NURBS handle
//! ([`super::plane::PlaneSection`]) and the NURBS × NURBS one
//! ([`super::pair::Section`]) read one section two ways under one
//! contract, and differ in the charts, the boundary pass and the cells a
//! tube is proved over. The rules that are the same on both live here:
//! settling a seed, the branch through a settled seed and the check that
//! its tube holds it, the crossings a certified branch already ends at,
//! and the closed loops from the subdivision's seeds.

use geom::NurbsSurface;
use geom_core::Point3;

use super::boundary::Crossing;
use super::ends::Ends;
use super::exhaust::UvRect;
use super::four::Box4;
use super::march::{
    BranchEnd, RectEnd, StepperMode, decide_transversality, domain_margin, march, newton_refine,
};
use super::system::LocalSystem;
use super::{SsiBranch, SsiError};
use geom_core::linalg::svd::Svd;

/// A cell of a certified tube, read against an ℝ⁴ state.
pub(crate) trait TubeCell: Copy {
    /// Whether the cell holds the state `x`.
    fn holds(&self, x: &[f64; 4]) -> bool;
}

/// The two-chart tube's cell: the state's four coordinates.
impl TubeCell for Box4 {
    fn holds(&self, x: &[f64; 4]) -> bool {
        Box4::holds(*self, x)
    }
}

/// The chart tube's cell: the wall's coordinates, the state's last two.
impl TubeCell for UvRect {
    fn holds(&self, x: &[f64; 4]) -> bool {
        x[2] >= self.u.0 && x[2] <= self.u.1 && x[3] >= self.v.0 && x[3] <= self.v.1
    }
}

/// The foot of `seed` on a NURBS chart, `(u, v)`.
///
/// # Errors
///
/// [`SsiError::FootPointInconclusive`] when the projection will not
/// converge.
pub(crate) fn foot(s: &NurbsSurface<f64>, seed: Point3<f64>) -> Result<(f64, f64), SsiError> {
    s.project(seed)
        .map(|f| (f.u, f.v))
        .map_err(|e| SsiError::FootPointInconclusive {
            t: f64::NAN,
            last_distance: e.last_distance,
        })
}

impl Ends<'_> {
    /// **A seed settled onto the section**: Newton from `start`, the
    /// state inside the march's rectangles, and the two surfaces
    /// transverse there.
    ///
    /// # Errors
    ///
    /// [`SsiError::SeedRefinementFailed`] when the state will not settle,
    /// [`SsiError::SeedOffDomain`] when it settles outside a rectangle,
    /// and the transversality decision at the settled state
    /// ([`SsiError::TransversalityBand`] where the surfaces are tangent).
    pub(crate) fn settle(&self, start: [f64; 4]) -> Result<[f64; 4], SsiError> {
        let mode = StepperMode::Realized;
        let x = newton_refine(self.sys, start, self.ctx.tol)
            .ok_or(SsiError::SeedRefinementFailed { mode: mode.name() })?;
        if !self.exit.inside(&x, &self.ctx) {
            return Err(SsiError::SeedOffDomain {
                mode: mode.name(),
                margin: domain_margin(&x, &self.ctx, self.sys, &x),
            });
        }
        let sigma = Svd::<3, 4>::new(self.sys.jacobian(&x)).sigma_min();
        decide_transversality(self.sys, &x, sigma, self.ctx.extent, self.band)?;
        Ok(x)
    }
}

/// The branches a handle has certified, and the cells each one's tube is
/// proved over.
pub(crate) struct Banked<T> {
    /// The certified branches, in the order they were found.
    pub branches: Vec<SsiBranch>,
    /// Each branch's tube, by the branch's index.
    pub tubes: Vec<Vec<T>>,
    /// The cells a certified branch's tube is proved over.
    tube_of: fn(&SsiBranch) -> Vec<T>,
}

impl<T: TubeCell> Banked<T> {
    /// Nothing certified yet, each tube to be read by `tube_of`.
    pub(crate) fn new(tube_of: fn(&SsiBranch) -> Vec<T>) -> Self {
        Self {
            branches: Vec::new(),
            tubes: Vec::new(),
            tube_of,
        }
    }

    /// The certified branch whose tube holds `x`, by index.
    pub(crate) fn holding(&self, x: &[f64; 4]) -> Option<usize> {
        self.tubes.iter().position(|t| t.iter().any(|c| c.holds(x)))
    }

    /// Bank a certified branch and its tube; its index.
    pub(crate) fn bank(&mut self, branch: SsiBranch) -> usize {
        self.tubes.push((self.tube_of)(&branch));
        self.branches.push(branch);
        self.branches.len() - 1
    }

    /// **The branch through the settled state `x`**: traced to the
    /// crossings it reaches, fitted, refined and certified, and banked
    /// where its certified tube holds `x`; its index.
    ///
    /// # Errors
    ///
    /// Any refusal of the trace, the fit or the certificate, and
    /// [`SsiError::SeedOutsideTube`] where the certified tube does not
    /// hold `x` (a trace or certificate fault: the branch is traced
    /// through `x`, so this does not detect a seed settled onto another
    /// component).
    pub(crate) fn through(
        &mut self,
        ends: &Ends<'_>,
        x: [f64; 4],
        crossings: &[Crossing],
    ) -> Result<usize, SsiError> {
        let (states, end) = ends.through_seed(x, crossings)?;
        let branch = ends.finish(&states, end)?;
        let tube = (self.tube_of)(&branch);
        if !tube.iter().any(|c| c.holds(&x)) {
            return Err(SsiError::SeedOutsideTube { state: x });
        }
        self.tubes.push(tube);
        self.branches.push(branch);
        Ok(self.branches.len() - 1)
    }

    /// The `crossings` no certified branch already ends at.
    pub(crate) fn open(&self, crossings: Vec<Crossing>) -> Vec<Crossing> {
        let ended: Vec<_> = self
            .branches
            .iter()
            .filter_map(|b| match b.end {
                BranchEnd::Crossings { from, to } => Some([from, to]),
                _ => None,
            })
            .flatten()
            .collect();
        crossings
            .into_iter()
            .filter(|c| !ended.contains(&c.at))
            .collect()
    }

    /// **The closed loops** from the subdivision's `seeds`: a seed whose
    /// settled state a tube or one of `also` already holds is skipped,
    /// any other is marched, and a march that closes is fitted,
    /// certified and banked. A seed that will not settle inside the
    /// rectangles is no branch; one that settles on an open branch
    /// reaches the boundary, where the branch was traced from its
    /// crossings.
    ///
    /// # Errors
    ///
    /// A march's refusal from a seed no tube holds by the end (a march
    /// that refuses from a seed some later branch's tube turns out to
    /// hold was a wasted march, not the operation's refusal), and any
    /// refusal of a closed loop's fit or certificate.
    pub(crate) fn closed_loops(
        &mut self,
        ends: &Ends<'_>,
        seeds: impl IntoIterator<Item = [f64; 4]>,
        also: &[T],
    ) -> Result<(), SsiError> {
        let held = |bank: &Self, x: &[f64; 4]| {
            bank.holding(x).is_some() || also.iter().any(|c| c.holds(x))
        };
        let mut refused: Vec<([f64; 4], SsiError)> = Vec::new();
        for state in seeds {
            let landed = newton_refine(ends.sys, state, ends.ctx.tol).unwrap_or(state);
            if held(self, &landed) {
                continue;
            }
            let trace = match march(
                ends.sys,
                &ends.exit,
                state,
                ends.ctx,
                StepperMode::Realized,
                1.0,
                ends.band,
            ) {
                Ok(t) => t,
                Err(SsiError::SeedRefinementFailed { .. } | SsiError::SeedOffDomain { .. }) => {
                    continue;
                }
                Err(e) => {
                    refused.push((landed, e));
                    continue;
                }
            };
            let RectEnd::Closed = trace.end else {
                continue;
            };
            let branch = ends.finish(&trace.states, BranchEnd::Closed)?;
            self.bank(branch);
        }
        match refused.into_iter().find(|(x, _)| !held(self, x)) {
            Some((_, e)) => Err(e),
            None => Ok(()),
        }
    }
}
