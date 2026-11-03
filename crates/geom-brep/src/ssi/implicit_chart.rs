//! **A residual side × NURBS: one complete section, read at a seed or
//! whole**, on the 1×2 trace (C3).
//!
//! The section of a plane, cylinder, sphere or ring torus and a NURBS wall is
//! the zero set of `φ = f∘S` over the wall's knot rectangle, `f` the
//! partner's implicit residual in metres and `S` the wall's chart,
//! traced on the wall's `(u, v)` alone ([`ImplicitChartR2`]): the point
//! is eliminated, the carrier is the wall's image of the traced chart
//! curve, and the branch carries the wall's pcurve and none on the
//! partner, which has no chart. Its limb 3 is the chart tube in the
//! wall's rectangle, read against the partner's form, the plane lane's
//! one tube ([`super::enclose::ChartPhi`]).
//!
//! [`ImplicitSection::of`] checks the operands, mints the wall's floors
//! and the march's tolerance ([`ImplicitWall`]); the boundary pass
//! (`super::boundary`, against the partner's form) runs once, at the
//! first reading that needs its crossings or regions. Its two readings
//! are one contract, as on the NURBS × NURBS lane ([`super::Section`]):
//!
//! - [`ImplicitSection::branch_at`] answers what holds a seed: a tube
//!   already certified, a `Side` or `Corner` region the pass reported, or
//!   the branch traced through the settled seed and certified, whose
//!   chart tube holds it (`super::handle`'s one unproven step stands).
//! - [`ImplicitSection::all`] answers every branch: each open branch
//!   between crossings, each closed loop from the seeds, the pass's
//!   regions, and the accounting that every cell of the wall's rectangle
//!   is excluded, in a tube, or in a region.
//!
//! **Exclusion.** Seeding and accounting exclude a cell where the
//! partner's form over the wall's box there is clear of zero: the cell
//! is certified apart from the partner. The test is a zero test, not a
//! band decision, so it reads the form itself; a distance in metres
//! would carry its sign and no more.

use std::cell::OnceCell;

use geom::{NurbsSurface, Surface};
use geom_core::{Band, Point3};

use super::boundary::BoundaryPass;
use super::enclose::ChartPhi;
use super::ends::Ends;
use super::exhaust::{self, FloorKind, SweepFloor, UvRect};
use super::handle::{Banked, TubeCell, foot, passed};
use super::march::MarchContext;
use super::residual::{ImplicitForm, ImplicitWall};
use super::system::ImplicitChartR2;
use super::{
    ChartedNurbs, SSI_MAX_STEPS, SsiBoundaryContact, SsiBranch, SsiDomain, SsiError, SsiOperand,
    SsiOutcome, branch_chart_tubes, finite_operand,
};

/// **What holds a seed** ([`ImplicitSection::branch_at`]): the branch
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

/// What the section is read over, fixed by [`ImplicitSection::of`].
struct Lane<'a> {
    curved: ImplicitForm,
    wall: ImplicitWall<'a>,
    wall_op: SsiOperand<'a, f64>,
    sys: ImplicitChartR2<'a>,
    /// The march's context, or the refusal of its tolerance, which only
    /// a reading that marches meets.
    ctx: Result<MarchContext<2>, SsiError>,
    seed_floor: SweepFloor<UvRect>,
}

impl Lane<'_> {
    /// The ends' reader over this lane.
    ///
    /// # Errors
    ///
    /// The march tolerance's refusal.
    fn ends(&self) -> Result<Ends<'_, ImplicitChartR2<'_>, 1, 2>, SsiError> {
        let ctx = self.ctx.clone()?;
        Ok(Ends::implicit(
            &self.sys,
            ctx,
            &self.wall_op,
            self.wall.band(),
        ))
    }

    /// Whether the partner's form over the cell's box is clear of zero
    /// (module docs).
    ///
    /// # Errors
    ///
    /// [`SsiError::UnsupportedCertificate`] where the form over the cell
    /// does not certify, as the plane lane refuses a height that does not
    /// ([`exhaust::chart_plane_height`]).
    fn excluded(&self, cell: UvRect) -> Result<bool, SsiError> {
        let f = ChartPhi::Curved(&self.curved).over(self.wall.boxes(), cell.window());
        if !f.is_certified() {
            return Err(SsiError::UnsupportedCertificate { what: FORM_REFUSED });
        }
        Ok(exhaust::excludes_zero(f))
    }
}

/// The exclusion's refusal of a cell whose form does not certify.
const FORM_REFUSED: &str = "the curved partner's form over a cell of the NURBS wall does not \
     certify — the wall's enclosure refused there";

/// **The section of a plane, cylinder, sphere or ring torus and a NURBS wall**
/// (module docs): the operands checked, the boundary pass run once, and
/// every branch certified so far.
pub struct ImplicitSection<'a> {
    lane: Lane<'a>,
    /// The boundary pass, once a reading has run it.
    pass: OnceCell<BoundaryPass<[f64; 2]>>,
    banked: Banked<UvRect>,
}

impl core::fmt::Debug for ImplicitSection<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let Self { lane, pass, banked } = self;
        f.debug_struct("ImplicitSection")
            .field("partner", lane.sys.partner)
            .field("domain", &lane.wall.domain())
            .field("band", &lane.wall.band())
            .field("crossings", &pass.get().map(|p| p.crossings.len()))
            .field("regions", &pass.get().map(|p| p.contacts.len()))
            .field("branches", &banked.branches.len())
            .finish_non_exhaustive()
    }
}

impl<'a> ImplicitSection<'a> {
    /// **The section of `partner` and `wall`** over `domain` at `band`:
    /// the partner's form read, the operands checked, the wall's chart
    /// speeds and both floors minted, and the march's tolerance derived.
    /// The boundary pass runs at the first reading that needs it.
    ///
    /// # Errors
    ///
    /// [`SsiError::WrongLane`] for a partner that is not a plane, a
    /// cylinder, a sphere or a ring torus, [`SsiError::OperandNotFinite`]
    /// for an operand that is not finite, and [`ImplicitWall::of`]'s and
    /// the seeding floor's refusals.
    pub fn of(
        partner: &'a Surface<f64>,
        wall: &'a NurbsSurface<f64>,
        domain: SsiDomain,
        band: Band,
    ) -> Result<Self, SsiError> {
        Self::within(
            partner,
            (wall, ChartedNurbs::mint),
            domain,
            band,
            SSI_MAX_STEPS,
        )
    }

    /// [`ImplicitSection::of`] with the wall's chart had from `chart`,
    /// called where [`ImplicitSection::of`] mints it: [`ChartedNurbs::mint`]
    /// itself, or a chart a caller minted once for the operation and hands
    /// to every section of that wall. **The chart must be of `wall`**, as
    /// a mint of it is: the door checks that it is the very surface, in
    /// every build, and refuses otherwise.
    ///
    /// # Errors
    ///
    /// As [`ImplicitSection::of`], with the chart's own refusals where it
    /// mints, and [`SsiError::WrongLane`] where the chart is of another
    /// surface.
    pub fn of_charted(
        partner: &'a Surface<f64>,
        (wall, chart): (
            &'a NurbsSurface<f64>,
            impl FnOnce(&'a NurbsSurface<f64>) -> Result<ChartedNurbs<'a, f64>, SsiError>,
        ),
        domain: SsiDomain,
        band: Band,
    ) -> Result<Self, SsiError> {
        Self::within(partner, (wall, chart), domain, band, SSI_MAX_STEPS)
    }

    /// [`ImplicitSection::of_charted`] under a step budget of `max_steps`
    /// a branch.
    pub(super) fn within(
        partner: &'a Surface<f64>,
        (wall, chart): (
            &'a NurbsSurface<f64>,
            impl FnOnce(&'a NurbsSurface<f64>) -> Result<ChartedNurbs<'a, f64>, SsiError>,
        ),
        domain: SsiDomain,
        band: Band,
        max_steps: usize,
    ) -> Result<Self, SsiError> {
        // A partner that is not finite is refused by name before its form
        // is read, whose frame would refuse it as no partner at all.
        let operand = match partner {
            Surface::Plane { .. } => "plane",
            _ => "curved partner",
        };
        if matches!(
            partner,
            Surface::Plane { .. }
                | Surface::Cylinder { .. }
                | Surface::Sphere { .. }
                | Surface::Torus { .. }
        ) {
            finite_operand(operand, partner)?;
        }
        let curved = ImplicitForm::of(partner).map_err(|_| SsiError::WrongLane {
            expected: "a plane, cylinder, sphere or ring torus and a NURBS surface traced on the \
                       wall's chart",
        })?;
        let read = ImplicitWall::of_charted((wall, chart), domain, band)?;
        let (ud, vd) = (wall.knot_values_u().domain(), wall.knot_values_v().domain());
        let root = UvRect { u: ud, v: vd };
        let speed = read.charted().speeds().max();
        let seed_floor = SweepFloor::chart(root, domain.seed_floor(), speed, FloorKind::Seeding)?;
        let ctx = read.march_tol(&curved).map(|tol| MarchContext::<2> {
            domain: [[ud.0, ud.1], [vd.0, vd.1]],
            extent: domain.extent,
            tol,
            max_steps,
            spent: Default::default(),
        });
        let wall_op = SsiOperand::Nurbs(read.charted().clone());
        Ok(Self {
            lane: Lane {
                curved,
                wall: read,
                wall_op,
                sys: ImplicitChartR2 { partner, wall },
                ctx,
                seed_floor,
            },
            pass: OnceCell::new(),
            banked: Banked::new(branch_chart_tubes),
        })
    }

    /// **The boundary pass (C3)**, run at its first asking and kept: the
    /// pair's classification reads it before its regularity sweep, and
    /// [`ImplicitSection::all`] reads the same pass.
    ///
    /// # Errors
    ///
    /// The pass's refusal.
    pub(crate) fn pass(&self) -> Result<&BoundaryPass<[f64; 2]>, SsiError> {
        passed(&self.pass, || self.lane.wall.pass(&self.lane.curved))
    }

    /// The section with its boundary pass already run, over the same
    /// partner, wall, domain and band ([`ImplicitSection::pass`]'s answer
    /// carried from an earlier reading of this pair), so no reading runs
    /// it again.
    pub(crate) fn with_pass(self, pass: BoundaryPass<[f64; 2]>) -> Self {
        Self {
            pass: OnceCell::from(pass),
            ..self
        }
    }

    /// The wall's chart enclosures, minted once for the section.
    pub(crate) fn boxes(&self) -> &super::enclose::NurbsBoxes<'a, f64> {
        self.lane.wall.boxes()
    }

    /// The accounting floor over the wall's rectangle.
    pub(crate) fn account_floor(&self) -> SweepFloor<UvRect> {
        self.lane.wall.account_floor()
    }

    /// The partner's form, as the section reads it.
    pub(crate) fn curved(&self) -> &ImplicitForm {
        &self.lane.curved
    }

    /// **What holds `seed`** (module docs): the seed's foot on the wall,
    /// settled onto the section by Newton, is the state the answer is
    /// selected by.
    ///
    /// # Errors
    ///
    /// [`SsiError::FootPointInconclusive`] when the wall's foot will not
    /// converge, [`SsiError::SeedRefinementFailed`] when the state will
    /// not settle onto the section, [`SsiError::SeedOffDomain`] when it
    /// settles outside the wall's rectangle, the transversality decision
    /// at the settled state, the boundary pass's, any refusal of the
    /// trace, the fit or the certificate, and
    /// [`SsiError::SeedOutsideChartTube`] where the certified branch's
    /// tube does not hold the settled state.
    pub fn branch_at(&mut self, seed: Point3<f64>) -> Result<SectionReading<'_>, SsiError> {
        let (u, v) = foot(self.lane.wall.charted().surface(), seed)?;
        let x = self.lane.ends()?.settle([u, v])?;
        if let Some(i) = self.banked.holding(&x) {
            return Ok(SectionReading::Branch(&self.banked.branches[i]));
        }
        let pass = passed(&self.pass, || self.lane.wall.pass(&self.lane.curved))?;
        if let Some(i) = pass.regions.iter().position(|r| r.holds_state(&x)) {
            return Ok(SectionReading::Region(&pass.contacts[i]));
        }
        let i = self
            .banked
            .through(&self.lane.ends()?, x, &pass.crossings)?;
        Ok(SectionReading::Branch(&self.banked.branches[i]))
    }

    /// **Every branch, and the proof they are all of them** (module
    /// docs). Branches [`ImplicitSection::branch_at`] already certified
    /// are kept, and their crossings are not traced again.
    ///
    /// # Errors
    ///
    /// The march tolerance's ([`SsiError::SettlingUnresolvable`], naming
    /// the partner's reach where that binds), the boundary pass's, the
    /// seeding sweep's, any refusal of the
    /// trace, the fit or the certificate on a branch between crossings
    /// or of a closed loop's march where no tube holds its seed, and the
    /// accounting's ([`SsiError::ExhaustivenessInconclusive`],
    /// [`SsiError::CellBudget`]).
    pub fn all(mut self) -> Result<SsiOutcome, SsiError> {
        let lane = &self.lane;
        // Every whole reading marches, so the march's tolerance is met
        // first, ahead of the pass whose crossings it would settle.
        let ends = lane.ends()?;
        let pass = passed(&self.pass, || lane.wall.pass(&lane.curved))?;
        let seeds = exhaust::seed_chart_partner(lane.seed_floor, |cell| lane.excluded(cell))?;
        #[allow(clippy::cast_possible_truncation)]
        let seed_count = seeds.len() as u32;

        for branch in ends.branches(&self.banked.open(&pass.crossings))? {
            self.banked.bank(branch);
        }
        let states = seeds.iter().map(|&(u, v)| [u, v]);
        self.banked.closed_loops(&ends, states, &pass.regions)?;

        let tubes: Vec<UvRect> = self.banked.tubes.iter().flatten().copied().collect();
        let exhaustiveness = exhaust::account_chart_partner(
            (&tubes, &pass.regions, &pass.clear),
            lane.wall.account_floor(),
            |cell| lane.excluded(cell),
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

    use super::{ImplicitSection, SectionReading};
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
    /// mint.** [`ImplicitSection::of_charted`] handed the wall's chart,
    /// minted once and shared by two sections in turn (the second reads
    /// every cell the first formed), answers both seeds with the same
    /// branches and tubes as [`ImplicitSection::of`], bit for bit.
    #[test]
    fn a_plane_section_of_a_chart_minted_once_reads_as_a_fresh_mint() {
        let (plane, wall) = (level(0.1), net(&|i, _| [0.25, -0.25, 0.25][i]));
        let r = 0.1_f64.sqrt();
        let read = |mut section: ImplicitSection<'_>| {
            for x in [0.5 - r, 0.5 + r] {
                section.branch_at(Point3::new(x + 1e-3, 0.4, 0.1)).unwrap();
            }
            format!("{:?} {:?}", section.banked.branches, section.banked.tubes)
        };
        let fresh = read(ImplicitSection::of(&plane, &wall, domain(), band()).unwrap());
        let chart = super::ChartedNurbs::mint(&wall).unwrap();
        for turn in 0..2 {
            let charted = ImplicitSection::of_charted(
                &plane,
                (&wall, |_| Ok(chart.clone())),
                domain(),
                band(),
            )
            .unwrap();
            assert!(
                read(charted) == fresh,
                "turn {turn}: the charted section reads otherwise"
            );
        }
    }

    /// **A chart of another wall is refused.**
    /// [`ImplicitSection::of_charted`] handed another wall's chart refuses
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
            ImplicitSection::of_charted(&plane, (&wall, |_| Ok(chart.clone())), domain(), band());
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
        let mut section = ImplicitSection::of(&plane, &wall, domain(), band()).unwrap();
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
            let state = [x, 0.7];
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
        let mut section = ImplicitSection::of(&plane, &wall, domain(), band()).unwrap();
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
        let mut section = ImplicitSection::of(&plane, &wall, domain(), band()).unwrap();
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
