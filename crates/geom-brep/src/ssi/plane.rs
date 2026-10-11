//! **Plane × NURBS: one complete section, read at a seed or whole.**
//!
//! The section of a plane and a NURBS wall is the full solution set of
//! `P(s, t) = S(u, v)` over the wall's knot rectangle, the plane's chart
//! on the caller's window, traced in ℝ⁴ on the state `(s, t, u, v)`, its
//! limb 3 the chart tube in the wall's rectangle. [`PlaneSection::of`]
//! checks the operands, mints the floors and the march's tolerance, and
//! refuses a window that does not hold the wall; the boundary pass
//! (`super::boundary`) runs once, at the first reading that needs its
//! crossings or regions. Its two readings are one contract, as on the
//! NURBS × NURBS lane ([`super::Section`]):
//!
//! - [`PlaneSection::branch_at`] answers the branch through a seed: a
//!   seed that settles into a tube already certified is that branch; one
//!   that settles into a `Side` or `Corner` region the pass reported is
//!   no branch and refuses ([`SsiError::SeedInRegion`]); any other is
//!   traced to the crossings it reaches, fitted, refined and certified,
//!   and the answer is the branch whose certified chart tube holds the
//!   settled seed. What is proved is that the answer is the branch
//!   through the point Newton settled on, not that this is the branch
//!   nearest the seed.
//! - [`PlaneSection::all`] answers every branch: each open branch
//!   between crossings, each closed loop from the chart lane's seeds, the
//!   pass's regions, and the accounting that every cell of the wall's
//!   rectangle is excluded, in a tube, or in a region. Exhaustiveness is
//!   `all`'s obligation.
//!
//! **One unproven step**, as on the NURBS × NURBS lane: Newton's
//! convergence from a seed to the component nearest it is not proved,
//! the branch is traced through the settled state so its tube holds it
//! ([`SsiError::SeedOutsideTube`] guards the trace and the certificate,
//! not the selection), and no distance gate refuses a far seed. The
//! backstops sit with the caller.

use geom::{NurbsSurface, Surface};
use geom_core::{Band, Point3, Vec3};

use super::boundary::{self, BoundaryPass};
use super::ends::Ends;
use super::exhaust::{self, FloorKind, SweepFloor, UvRect};
use super::handle::{Banked, TubeCell, foot};
use super::march::{MarchContext, MarchTol};
use super::system::{Chart, ParametricPairR4};
use super::{
    ChartedNurbs, SSI_MAX_STEPS, SsiBranch, SsiDomain, SsiError, SsiOperand, SsiOutcome,
    boundary_pass, branch_chart_tubes, finite_net, finite_operand, spline_readout, window_reach,
};

/// **The section of a plane and a NURBS wall** (module docs): the
/// operands checked, the boundary pass run once, and every branch
/// certified so far.
pub struct PlaneSection<'a> {
    plane: &'a Surface<f64>,
    /// The plane's origin, its unit normal, and its chart's two axes.
    frame: (Point3<f64>, Vec3<f64>, Vec3<f64>, Vec3<f64>),
    wall: ChartedNurbs<'a, f64>,
    sys: ParametricPairR4<'a>,
    ctx: MarchContext<4>,
    domain: SsiDomain,
    band: Band,
    seed_floor: SweepFloor<UvRect>,
    account_floor: SweepFloor<UvRect>,
    /// The boundary pass, once it has run.
    pass: Option<BoundaryPass>,
    banked: Banked<UvRect>,
}

impl core::fmt::Debug for PlaneSection<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let Self {
            plane,
            frame: _,
            wall: _,
            sys: _,
            ctx: _,
            domain,
            band,
            seed_floor: _,
            account_floor: _,
            pass,
            banked,
        } = self;
        f.debug_struct("PlaneSection")
            .field("plane", plane)
            .field("domain", domain)
            .field("band", band)
            .field("crossings", &pass.as_ref().map(|p| p.crossings.len()))
            .field("regions", &pass.as_ref().map(|p| p.contacts.len()))
            .field("branches", &banked.branches.len())
            .finish_non_exhaustive()
    }
}

/// The lane's refusal of a plane operand that is not a plane chart.
fn wrong_lane() -> SsiError {
    SsiError::WrongLane {
        expected: "a plane and a NURBS surface traced in ℝ⁴ on their charts",
    }
}

impl<'a> PlaneSection<'a> {
    /// **The section of `plane` and `wall`**: the operands checked, the
    /// wall's chart speeds and both floors minted, the march's tolerance
    /// derived, and the plane's window on `domain` (centred on the
    /// plane's origin, `±half_extent` along its chart) checked to hold
    /// the wall, so a march ends only at the knot rectangle. A plane is
    /// unbounded, so the window is the caller's obligation, never a
    /// guess. The boundary pass runs at the first reading that needs it.
    ///
    /// # Errors
    ///
    /// [`SsiError::WrongLane`] when `plane` is not a plane,
    /// [`SsiError::OperandNotFinite`] for a plane or wall that is not
    /// finite, [`SsiError::DomainUnusable`], [`SsiError::ChartSpeed`] for
    /// a wall whose chart speed along an axis is zero or has no finite
    /// bound, [`SsiError::FloorUnresolvable`] for a floor the wall's
    /// rectangle cannot resolve, the march tolerance's refusals, and
    /// [`SsiError::WindowShortOfWall`].
    pub fn of(
        plane: &'a Surface<f64>,
        wall: &'a NurbsSurface<f64>,
        domain: SsiDomain,
        band: Band,
    ) -> Result<Self, SsiError> {
        Self::within(plane, wall, domain, band, SSI_MAX_STEPS)
    }

    /// [`PlaneSection::of`] under a step budget of `max_steps` a branch.
    pub(super) fn within(
        plane: &'a Surface<f64>,
        wall: &'a NurbsSurface<f64>,
        domain: SsiDomain,
        band: Band,
        max_steps: usize,
    ) -> Result<Self, SsiError> {
        let Surface::Plane {
            origin: p0,
            normal,
            u_ref,
        } = *plane
        else {
            return Err(wrong_lane());
        };
        // Before any sweep: the chart sweep lifts the plane's origin and
        // normal into intervals, which refuse a non-finite point, and its
        // refusal arm can only name the wall's net.
        finite_operand("plane", plane)?;
        finite_net("NURBS wall", wall)?;
        domain.check()?;
        let half = domain.half_extent;
        let chart_a =
            Chart::plane_of(plane, (-half, half), (-half, half)).ok_or_else(wrong_lane)?;
        let window = window_reach(&chart_a, half);
        let chart_b = Chart::Nurbs(wall);
        let ((pu, pv), (ud, vd)) = (chart_a.domain(), chart_b.domain());
        let sys = ParametricPairR4 {
            a: chart_a,
            b: chart_b,
        };
        let root = UvRect { u: ud, v: vd };

        // The wall's chart speeds, minted once per axis; the mint refuses
        // a zero or non-finite axis by name. The chart floors are metres
        // ÷ the larger of the two, so a floor stated in metres means the
        // same thing in both lanes, and dividing by a sup UNDER-states
        // the parameter reach — the safe side of a floor. Each floor is
        // minted once over the wall's domain, which refuses one the
        // domain cannot resolve, the proof obligation's first.
        let charted = ChartedNurbs::mint(wall)?;
        let speed = charted.speeds().max();
        let account_floor =
            SweepFloor::chart(root, domain.floor(band), speed, FloorKind::Accounting)?;
        let seed_floor = SweepFloor::chart(root, domain.seed_floor(), speed, FloorKind::Seeding)?;
        let tol = MarchTol::from_band(band, spline_readout(window, wall, root, speed))?;
        boundary::window_holds_wall((p0, u_ref, normal.cross(u_ref)), wall, half)?;

        let ctx = MarchContext::<4> {
            domain: [[pu.0, pu.1], [pv.0, pv.1], [ud.0, ud.1], [vd.0, vd.1]],
            extent: domain.extent,
            tol,
            max_steps,
            spent: Default::default(),
        };
        Ok(Self {
            plane,
            frame: (p0, normal, u_ref, normal.cross(u_ref)),
            wall: charted,
            sys,
            ctx,
            domain,
            band,
            seed_floor,
            account_floor,
            pass: None,
            banked: Banked::new(branch_chart_tubes),
        })
    }

    /// The boundary pass (C3), the pass run here if no reading has run
    /// it yet.
    fn pass(&mut self) -> Result<BoundaryPass, SsiError> {
        if let Some(p) = &self.pass {
            return Ok(p.clone());
        }
        let p = boundary_pass(
            self.wall.surface(),
            &self.wall,
            self.plane,
            &self.sys,
            self.ctx.tol,
            &self.domain,
            self.band,
        )?;
        self.pass = Some(p.clone());
        Ok(p)
    }

    /// **The branch through `seed`** (module docs). The seed is a point
    /// in space; its feet on the plane's chart and the wall's, settled
    /// onto the section by Newton, are the state the answer is selected
    /// by. The answer is the branch through that settled state, wherever
    /// Newton took it: that it is the component nearest the seed is not
    /// proved, and no distance from the seed is refused.
    ///
    /// # Errors
    ///
    /// [`SsiError::FootPointInconclusive`] when the wall's foot will not
    /// converge, [`SsiError::SeedRefinementFailed`] when the state will
    /// not settle onto the section, [`SsiError::SeedOffDomain`] when it
    /// settles outside the wall's rectangle, the transversality decision
    /// at the settled state ([`SsiError::TransversalityBand`] where the
    /// plane is tangent to the wall there), the boundary pass's,
    /// [`SsiError::SeedInRegion`] where a region the pass reported holds
    /// the settled state, any refusal of the trace, the fit or the
    /// certificate, and [`SsiError::SeedOutsideTube`] where the certified
    /// branch's tube does not hold the settled state (a trace or
    /// certificate fault; it does not detect a seed settled onto another
    /// component).
    pub fn branch_at(&mut self, seed: Point3<f64>) -> Result<&SsiBranch, SsiError> {
        let (origin, _, u_ref, v_ref) = self.frame;
        let q = seed - origin;
        let (u, v) = foot(self.wall.surface(), seed)?;
        let start = [q.dot(u_ref), q.dot(v_ref), u, v];
        let wall_op = SsiOperand::Nurbs(self.wall);
        let ends = Ends::of(&self.sys, self.ctx, self.plane, &wall_op, self.band);
        let x = ends.settle(start)?;
        if let Some(i) = self.banked.holding(&x) {
            return Ok(&self.banked.branches[i]);
        }
        let pass = self.pass()?;
        if pass.regions.iter().any(|r| r.holds(&x)) {
            return Err(SsiError::SeedInRegion { state: x });
        }
        let ends = Ends::of(&self.sys, self.ctx, self.plane, &wall_op, self.band);
        let i = self.banked.through(&ends, x, &pass.crossings)?;
        Ok(&self.banked.branches[i])
    }

    /// **Every branch, and the proof they are all of them** (module
    /// docs). Branches [`PlaneSection::branch_at`] already certified are
    /// kept, and their crossings are not traced again.
    ///
    /// # Errors
    ///
    /// The boundary pass's, the seeding sweep's, any refusal of the
    /// trace, the fit or the certificate on a branch between crossings
    /// (notably [`SsiError::CertificateEscalated`] naming
    /// [`super::SsiLimb::HullSup`] when the fitted pair's real
    /// between-samples deviation lands in the band, which an inflected
    /// wall can genuinely earn, and [`SsiError::RefinementExhausted`]
    /// where refinement meets the band or the branch's step budget), of a
    /// closed loop's march where no tube holds its seed, and the
    /// accounting's ([`SsiError::ExhaustivenessInconclusive`],
    /// [`SsiError::CellBudget`]).
    pub fn all(mut self) -> Result<SsiOutcome, SsiError> {
        let pass = self.pass()?;
        let (origin, normal, u_ref, v_ref) = self.frame;
        let wall = self.wall.surface();
        let seeds = exhaust::seed_chart_plane(wall, origin, normal, self.seed_floor)?;
        #[allow(clippy::cast_possible_truncation)]
        let seed_count = seeds.len() as u32;

        let wall_op = SsiOperand::Nurbs(self.wall);
        let ends = Ends::of(&self.sys, self.ctx, self.plane, &wall_op, self.band);
        // ---- the open branches, between the crossings not yet ended ----
        let open = self.banked.open(pass.crossings);
        for branch in ends.branches(&open)? {
            self.banked.bank(branch);
        }

        // ---- the closed loops, from the subdivision's seeds ----
        let states = seeds.iter().map(|&(u, v)| {
            let q = wall.eval(u, v) - origin;
            [q.dot(u_ref), q.dot(v_ref), u, v]
        });
        self.banked.closed_loops(&ends, states, &pass.regions)?;

        let tubes: Vec<UvRect> = self.banked.tubes.iter().flatten().copied().collect();
        let exhaustiveness = exhaust::account_chart_plane(
            wall,
            origin,
            normal,
            &tubes,
            &pass.regions,
            &pass.clear,
            self.account_floor,
        )?;
        Ok(SsiOutcome {
            branches: self.banked.branches,
            boundary: pass.contacts,
            exhaustiveness,
            seeds: seed_count,
        })
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use geom::{NurbsSurface, Surface};
    use geom_core::spline::KnotVector;
    use geom_core::{Band, Point3, Vec3};

    use super::PlaneSection;
    use crate::ssi::handle::TubeCell;
    use crate::ssi::{SsiDomain, SsiError};

    /// A biquadratic over `[0, 1]²` whose height is `z(i, j)` at control
    /// point `(i, j)`.
    fn net(z: &dyn Fn(usize, usize) -> f64) -> NurbsSurface<f64> {
        let k = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
        let control = (0..9)
            .map(|n| {
                let (i, j) = (n / 3, n % 3);
                Point3::new(0.5 * i as f64, 0.5 * j as f64, z(i, j))
            })
            .collect();
        NurbsSurface::new(k.clone(), k, control, vec![1.0; 9]).unwrap()
    }

    /// The horizontal plane at height `h`.
    fn level(h: f64) -> Surface<f64> {
        Surface::Plane {
            origin: Point3::new(0.5, 0.5, h),
            normal: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        }
    }

    fn domain() -> SsiDomain {
        SsiDomain {
            center: Point3::new(0.5, 0.5, 0.0),
            half_extent: 2.0,
            extent: 1.0,
            floor_scale: 1.0,
        }
    }

    fn band() -> Band {
        Band::new(1e-9, 1e-8).unwrap()
    }

    /// **Each seed is answered by the branch through it.** The trough
    /// `z = (x − ½)²` cut at `z = 0.1` has two branches, at
    /// `x = ½ ± √0.1`. A seed beside each is answered by a branch whose
    /// certified tube holds the state it settled to, the two answers are
    /// different branches, a second seed on a branch already certified
    /// traces nothing, and `all` keeps both and accounts the rectangle.
    #[test]
    fn each_seed_is_answered_by_the_branch_through_it() {
        let (plane, wall) = (level(0.1), net(&|i, _| [0.25, -0.25, 0.25][i]));
        let mut section = PlaneSection::of(&plane, &wall, domain(), band()).unwrap();
        let r = 0.1_f64.sqrt();
        for x in [0.5 - r, 0.5 + r] {
            let mid = section
                .branch_at(Point3::new(x + 1e-3, 0.4, 0.1))
                .unwrap()
                .witness;
            assert!(
                (mid.x - x).abs() < 1e-6,
                "the branch at x = {x} passes {mid:?}"
            );
        }
        assert_eq!(section.banked.branches.len(), 2, "two branches certified");
        for (i, x) in [0.5 - r, 0.5 + r].into_iter().enumerate() {
            let tube = &section.banked.tubes[i];
            let state = [x - 0.5, 0.2, x, 0.7];
            assert!(
                tube.iter().any(|c| c.holds(&state)),
                "branch {i}'s tube holds the section at x = {x}"
            );
        }
        section.branch_at(Point3::new(0.5 + r, 0.7, 0.1)).unwrap();
        assert_eq!(
            section.banked.branches.len(),
            2,
            "a seed on a certified branch is that branch"
        );
        let out = section.all().unwrap();
        assert_eq!(out.branches.len(), 2, "{out:?}");
        assert!(out.exhaustiveness.accounted > 0, "{}", out.exhaustiveness);
    }

    /// **A seed a side region holds is no branch.** The plane `z = 0`
    /// holds the bottom row of a wall rising from it, which the boundary
    /// pass reports as a `Side` region; a seed on that row settles in the
    /// region and refuses by it.
    #[test]
    fn a_seed_in_a_side_region_refuses() {
        let (plane, wall) = (level(0.0), net(&|_, j| 0.5 * j as f64));
        let mut section = PlaneSection::of(&plane, &wall, domain(), band()).unwrap();
        let e = section
            .branch_at(Point3::new(0.5, 0.0, 0.0))
            .expect_err("the bottom row is a region");
        assert!(matches!(e, SsiError::SeedInRegion { .. }), "{e:?}");
    }
}
