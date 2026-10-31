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
//! - [`PlaneSection::branch_at`] answers what holds a seed: a seed that
//!   settles into a tube already certified is that branch; one that
//!   settles into a `Side` or `Corner` region the pass reported is that
//!   region; any other is traced to the crossings it reaches, fitted,
//!   refined and certified, and the answer is the branch whose certified
//!   chart tube holds the settled seed. What is proved is that the answer
//!   holds the point Newton settled on, not that it is the piece of the
//!   section nearest the seed (`super::handle`'s one unproven step).
//! - [`PlaneSection::all`] answers every branch: each open branch
//!   between crossings, each closed loop from the chart lane's seeds, the
//!   pass's regions, and the accounting that every cell of the wall's
//!   rectangle is excluded, in a tube, or in a region. Exhaustiveness is
//!   `all`'s obligation.

use std::cell::OnceCell;

use geom::{NurbsSurface, Surface};
use geom_core::{Band, Point3, Vec3};

use super::boundary::{self, BoundaryPass, SsiBoundaryContact};
use super::ends::Ends;
use super::exhaust::{self, FloorKind, SweepFloor, UvRect};
use super::handle::{Banked, TubeCell, foot, passed};
use super::march::{MarchContext, MarchTol};
use super::system::{Chart, ParametricPairR4};
use super::{
    ChartedNurbs, SSI_MAX_STEPS, SsiBranch, SsiDomain, SsiError, SsiOperand, SsiOutcome,
    boundary_pass, branch_chart_tubes, finite_net, finite_operand, spline_readout, window_reach,
};

/// **What holds a seed** ([`PlaneSection::branch_at`]): the branch
/// through it, or the boundary region it settled in.
#[derive(Clone, Copy, Debug)]
pub enum SectionReading<'s> {
    /// The certified branch whose tube holds the settled seed.
    Branch(&'s SsiBranch),
    /// The `Side` or `Corner` region the boundary pass reported that
    /// holds the settled seed: the section there runs along a side of
    /// the wall's rectangle, or leaves it at a corner, within the band.
    Region(&'s SsiBoundaryContact),
}

/// What the section is read over: the operands, their charts and the
/// march, fixed by [`PlaneSection::of`].
struct Lane<'a> {
    plane: &'a Surface<f64>,
    /// The plane's origin, its unit normal, and its chart's two axes.
    frame: (Point3<f64>, Vec3<f64>, Vec3<f64>, Vec3<f64>),
    wall: ChartedNurbs<'a, f64>,
    wall_op: SsiOperand<'a, f64>,
    sys: ParametricPairR4<'a>,
    ctx: MarchContext<4>,
    domain: SsiDomain,
    band: Band,
    seed_floor: SweepFloor<UvRect>,
    account_floor: SweepFloor<UvRect>,
}

impl Lane<'_> {
    /// The ends' reader over this lane.
    fn ends(&self) -> Ends<'_> {
        Ends::of(&self.sys, self.ctx, self.plane, &self.wall_op, self.band)
    }

    /// The state of the wall's point `p` at `(u, v)`: `p`'s coordinates
    /// on the plane's chart, then the wall's.
    fn state(&self, p: Point3<f64>, (u, v): (f64, f64)) -> [f64; 4] {
        let (origin, _, u_ref, v_ref) = self.frame;
        let q = p - origin;
        [q.dot(u_ref), q.dot(v_ref), u, v]
    }

    /// The boundary pass (C3).
    fn pass(&self) -> Result<BoundaryPass, SsiError> {
        boundary_pass(
            self.wall.surface(),
            &self.wall,
            self.plane,
            &self.sys,
            self.ctx.tol,
            &self.domain,
            self.band,
        )
    }
}

/// **The section of a plane and a NURBS wall** (module docs): the
/// operands checked, the boundary pass run once, and every branch
/// certified so far.
pub struct PlaneSection<'a> {
    lane: Lane<'a>,
    /// The boundary pass, once a reading has run it.
    pass: OnceCell<BoundaryPass>,
    banked: Banked<UvRect>,
}

impl core::fmt::Debug for PlaneSection<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let Self { lane, pass, banked } = self;
        f.debug_struct("PlaneSection")
            .field("plane", lane.plane)
            .field("domain", &lane.domain)
            .field("band", &lane.band)
            .field("crossings", &pass.get().map(|p| p.crossings.len()))
            .field("regions", &pass.get().map(|p| p.contacts.len()))
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
        Self::within(
            plane,
            (wall, ChartedNurbs::mint),
            domain,
            band,
            SSI_MAX_STEPS,
        )
    }

    /// [`PlaneSection::of`] with the wall's chart had from `chart`,
    /// called where [`PlaneSection::of`] mints it (after the operands and
    /// the domain are checked, so the refusals come in the same order):
    /// [`ChartedNurbs::mint`] itself, or a chart a caller minted once for
    /// the operation and hands to every section of that wall. **The
    /// chart must be of `wall`**, as a mint of it is: the door checks
    /// that it is the very surface, in every build, and refuses
    /// otherwise.
    ///
    /// # Errors
    ///
    /// As [`PlaneSection::of`], with the chart's own refusals where it
    /// mints, and [`SsiError::WrongLane`] where the chart is of another
    /// surface.
    pub fn of_charted(
        plane: &'a Surface<f64>,
        (wall, chart): (
            &'a NurbsSurface<f64>,
            impl FnOnce(&'a NurbsSurface<f64>) -> Result<ChartedNurbs<'a, f64>, SsiError>,
        ),
        domain: SsiDomain,
        band: Band,
    ) -> Result<Self, SsiError> {
        Self::within(plane, (wall, chart), domain, band, SSI_MAX_STEPS)
    }

    /// [`PlaneSection::of_charted`] under a step budget of `max_steps` a
    /// branch.
    pub(super) fn within(
        plane: &'a Surface<f64>,
        (wall, chart): (
            &'a NurbsSurface<f64>,
            impl FnOnce(&'a NurbsSurface<f64>) -> Result<ChartedNurbs<'a, f64>, SsiError>,
        ),
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
        let charted = chart(wall)?;
        if !core::ptr::eq(charted.surface(), wall) {
            return Err(SsiError::WrongLane {
                expected: "the wall paired with a chart minted from it (`PlaneSection::of_charted`)",
            });
        }
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
            lane: Lane {
                plane,
                frame: (p0, normal, u_ref, normal.cross(u_ref)),
                wall: charted.clone(),
                wall_op: SsiOperand::Nurbs(charted),
                sys,
                ctx,
                domain,
                band,
                seed_floor,
                account_floor,
            },
            pass: OnceCell::new(),
            banked: Banked::new(branch_chart_tubes),
        })
    }

    /// **The boundary pass (C3)**, run at its first asking and kept: the
    /// pair's classification reads it before its regularity sweep, and
    /// [`PlaneSection::all`] reads the same pass.
    ///
    /// # Errors
    ///
    /// The pass's refusal.
    pub(crate) fn pass(&self) -> Result<&BoundaryPass, SsiError> {
        passed(&self.pass, || self.lane.pass())
    }

    /// The section with its boundary pass already run, over the same
    /// plane, wall, domain and band ([`PlaneSection::pass`]'s answer
    /// carried from an earlier reading of this pair), so no reading runs
    /// it again.
    pub(crate) fn with_pass(self, pass: BoundaryPass) -> Self {
        Self {
            pass: OnceCell::from(pass),
            ..self
        }
    }

    /// The wall's chart enclosures, minted once for the section.
    pub(crate) fn boxes(&self) -> &super::enclose::NurbsBoxes<'a, f64> {
        self.lane.wall.own_boxes()
    }

    /// The accounting floor over the wall's rectangle: the width at which
    /// a cell no certificate holds is refused, or declined.
    pub(crate) fn account_floor(&self) -> SweepFloor<UvRect> {
        self.lane.account_floor
    }

    /// **What holds `seed`** (module docs). The seed is a point in space;
    /// its feet on the plane's chart and the wall's, settled onto the
    /// section by Newton, are the state the answer is selected by. The
    /// answer holds that settled state, wherever Newton took it: that it
    /// is the piece of the section nearest the seed is not proved, and no
    /// distance from the seed is refused.
    ///
    /// # Errors
    ///
    /// [`SsiError::FootPointInconclusive`] when the wall's foot will not
    /// converge, [`SsiError::SeedRefinementFailed`] when the state will
    /// not settle onto the section, [`SsiError::SeedOffDomain`] when it
    /// settles outside the wall's rectangle, the transversality decision
    /// at the settled state ([`SsiError::TransversalityBand`] where the
    /// plane is tangent to the wall there), the boundary pass's, any
    /// refusal of the trace, the fit or the certificate, and
    /// [`SsiError::SeedOutsideTube`] where the certified branch's tube
    /// does not hold the settled state (a trace or certificate fault; it
    /// does not detect a seed settled onto another component).
    pub fn branch_at(&mut self, seed: Point3<f64>) -> Result<SectionReading<'_>, SsiError> {
        let start = self.lane.state(seed, foot(self.lane.wall.surface(), seed)?);
        let x = self.lane.ends().settle(start)?;
        if let Some(i) = self.banked.holding(&x) {
            return Ok(SectionReading::Branch(&self.banked.branches[i]));
        }
        let pass = passed(&self.pass, || self.lane.pass())?;
        if let Some(i) = pass.regions.iter().position(|r| r.holds_state(&x)) {
            return Ok(SectionReading::Region(&pass.contacts[i]));
        }
        let i = self.banked.through(&self.lane.ends(), x, &pass.crossings)?;
        Ok(SectionReading::Branch(&self.banked.branches[i]))
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
    /// [`super::SsiLimb::HullSupChart`] when the fitted pair's real
    /// between-samples deviation lands in the band, which an inflected
    /// wall can genuinely earn, and [`SsiError::RefinementExhausted`]
    /// where refinement meets the band or the branch's step budget), of a
    /// closed loop's march where no tube holds its seed, and the
    /// accounting's ([`SsiError::ExhaustivenessInconclusive`],
    /// [`SsiError::CellBudget`]).
    pub fn all(mut self) -> Result<SsiOutcome, SsiError> {
        let lane = &self.lane;
        let pass = passed(&self.pass, || lane.pass())?;
        let (origin, normal, ..) = lane.frame;
        let wall = lane.wall.surface();
        let seeds = exhaust::seed_chart_plane(wall, origin, normal, lane.seed_floor)?;
        #[allow(clippy::cast_possible_truncation)]
        let seed_count = seeds.len() as u32;

        let ends = lane.ends();
        // ---- the open branches, between the crossings not yet ended ----
        for branch in ends.branches(&self.banked.open(&pass.crossings))? {
            self.banked.bank(branch);
        }

        // ---- the closed loops, from the subdivision's seeds ----
        let states = seeds
            .iter()
            .map(|&uv| lane.state(wall.eval(uv.0, uv.1), uv));
        self.banked.closed_loops(&ends, states, &pass.regions)?;

        let tubes: Vec<UvRect> = self.banked.tubes.iter().flatten().copied().collect();
        let exhaustiveness = exhaust::account_chart_plane(
            wall,
            origin,
            normal,
            &tubes,
            &pass.regions,
            &pass.clear,
            lane.account_floor,
        )?;
        Ok(SsiOutcome {
            branches: self.banked.branches,
            boundary: pass.contacts.clone(),
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

    use super::{PlaneSection, SectionReading};
    use crate::ssi::handle::TubeCell;
    use crate::ssi::{ChartAxis, ChartEnd, SsiBoundaryContact, SsiDomain};

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

    /// **A plane section of a chart minted once reads as of a fresh
    /// mint.** [`PlaneSection::of_charted`] handed the wall's chart,
    /// minted once and shared by two sections in turn (the second reads
    /// every cell the first formed), answers both seeds with the same
    /// branches and tubes as [`PlaneSection::of`], bit for bit.
    #[test]
    fn a_plane_section_of_a_chart_minted_once_reads_as_a_fresh_mint() {
        let (plane, wall) = (level(0.1), net(&|i, _| [0.25, -0.25, 0.25][i]));
        let r = 0.1_f64.sqrt();
        let read = |mut section: PlaneSection<'_>| {
            for x in [0.5 - r, 0.5 + r] {
                section.branch_at(Point3::new(x + 1e-3, 0.4, 0.1)).unwrap();
            }
            format!("{:?} {:?}", section.banked.branches, section.banked.tubes)
        };
        let fresh = read(PlaneSection::of(&plane, &wall, domain(), band()).unwrap());
        let chart = super::ChartedNurbs::mint(&wall).unwrap();
        for turn in 0..2 {
            let charted =
                PlaneSection::of_charted(&plane, (&wall, |_| Ok(chart.clone())), domain(), band())
                    .unwrap();
            assert!(
                read(charted) == fresh,
                "turn {turn}: the charted section reads otherwise"
            );
        }
    }

    /// **A chart of another wall is refused.**
    /// [`PlaneSection::of_charted`] handed another wall's chart refuses
    /// with [`SsiError::WrongLane`] in every build, before any reading.
    #[test]
    fn a_chart_of_another_wall_is_refused() {
        let plane = level(0.1);
        let (wall, other) = (
            net(&|i, _| [0.25, -0.25, 0.25][i]),
            net(&|i, _| [0.3, -0.2, 0.3][i]),
        );
        let chart = super::ChartedNurbs::mint(&other).unwrap();
        let got =
            PlaneSection::of_charted(&plane, (&wall, |_| Ok(chart.clone())), domain(), band());
        assert!(
            matches!(got, Err(super::SsiError::WrongLane { .. })),
            "a mispaired chart came back {:?}",
            got.err()
        );
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
            let SectionReading::Branch(branch) =
                section.branch_at(Point3::new(x + 1e-3, 0.4, 0.1)).unwrap()
            else {
                panic!("the seed beside x = {x} settles on a branch");
            };
            let mid = branch.witness;
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
                tube.iter().any(|c| c.holds_state(&state)),
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

    /// **A seed a side region holds is answered by the region.** The
    /// plane `z = 0` holds the bottom row of a wall rising from it, which
    /// the boundary pass reports as a `Side` region; a seed on that row
    /// settles in the region, and the region is the answer.
    #[test]
    fn a_seed_in_a_side_region_is_answered_by_it() {
        let (plane, wall) = (level(0.0), net(&|_, j| 0.5 * j as f64));
        let mut section = PlaneSection::of(&plane, &wall, domain(), band()).unwrap();
        let reading = section.branch_at(Point3::new(0.5, 0.0, 0.0)).unwrap();
        let SectionReading::Region(SsiBoundaryContact::Side { side, .. }) = reading else {
            panic!("the bottom row is a side region, got {reading:?}");
        };
        assert_eq!(
            (side.fixed, side.end),
            (ChartAxis::V, ChartEnd::Low),
            "the region is the bottom row"
        );
    }

    /// **A seed a corner region holds is answered by the corner.** The
    /// plane `z = 0` meets the tilted wall `z = x + y` only at its corner
    /// `(0, 0)`; a seed there settles in the corner region.
    #[test]
    fn a_seed_in_a_corner_region_is_answered_by_it() {
        let (plane, wall) = (level(0.0), net(&|i, j| 0.5 * (i + j) as f64));
        let mut section = PlaneSection::of(&plane, &wall, domain(), band()).unwrap();
        let reading = section.branch_at(Point3::new(0.0, 0.0, 0.0)).unwrap();
        assert!(
            matches!(
                reading,
                SectionReading::Region(SsiBoundaryContact::Corner { .. })
            ),
            "the wall meets the plane at its corner, got {reading:?}"
        );
    }
}
