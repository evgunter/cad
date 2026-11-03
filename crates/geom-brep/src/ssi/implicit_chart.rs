//! **A residual side × NURBS: one complete section, read at a seed or
//! whole**, on the 1×2 trace (C3).
//!
//! The section of a cylinder, sphere or ring torus and a NURBS wall is
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
//! are the plane lane's contract ([`super::PlaneSection`]):
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
//! partner's form over the wall's box there, read as a floor in metres
//! ([`Curved::floor_metres`]), is clear of zero: the cell is certified
//! apart from the partner. A floor's sign is the form's, so the reading
//! is `f`'s own zero test, stated in metres.

use std::cell::OnceCell;

use geom::{NurbsSurface, Surface};
use geom_core::{Band, Point3};

use super::boundary::BoundaryPass;
use super::ends::Ends;
use super::exhaust::{self, FloorKind, SweepFloor, UvRect};
use super::handle::{Banked, TubeCell, foot, passed};
use super::march::MarchContext;
use super::residual::{Curved, ImplicitWall};
use super::system::ImplicitChartR2;
use super::{
    SSI_MAX_STEPS, SectionReading, SsiDomain, SsiError, SsiOperand, SsiOutcome, branch_chart_tubes,
    finite_operand,
};

/// What the section is read over, fixed by [`ImplicitSection::of`].
struct Lane<'a> {
    curved: Curved,
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

    /// Whether the partner's form over the cell's box is clear of zero,
    /// read as a floor in metres (module docs).
    fn excluded(&self, cell: UvRect) -> Result<bool, SsiError> {
        let b = cell.window().map_or_else(super::enclose::refused_box, |w| {
            self.wall.boxes().rect_box(w)
        });
        let q = self.curved.framed(b);
        let metres = self.curved.floor_metres(self.curved.value(q), q);
        if !metres.is_certified() {
            return Err(SsiError::UnsupportedCertificate { what: FORM_REFUSED });
        }
        Ok(exhaust::excludes_zero(metres))
    }
}

/// The exclusion's refusal of a cell whose form does not certify.
const FORM_REFUSED: &str = "the curved partner's form over a cell of the NURBS wall does not \
     certify — the wall's enclosure refused there, or the form's divisor has no positive finite \
     bound";

/// **The section of a cylinder, sphere or ring torus and a NURBS wall**
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
    /// [`SsiError::WrongLane`] for a partner that is not a cylinder, a
    /// sphere or a ring torus, [`SsiError::OperandNotFinite`] for an
    /// operand that is not finite, and [`ImplicitWall::of`]'s and the
    /// seeding floor's refusals.
    pub fn of(
        partner: &'a Surface<f64>,
        wall: &'a NurbsSurface<f64>,
        domain: SsiDomain,
        band: Band,
    ) -> Result<Self, SsiError> {
        Self::within(partner, wall, domain, band, SSI_MAX_STEPS)
    }

    /// [`ImplicitSection::of`] under a step budget of `max_steps` a
    /// branch.
    pub(super) fn within(
        partner: &'a Surface<f64>,
        wall: &'a NurbsSurface<f64>,
        domain: SsiDomain,
        band: Band,
        max_steps: usize,
    ) -> Result<Self, SsiError> {
        let curved = Curved::of(partner).map_err(|_| SsiError::WrongLane {
            expected: "a cylinder, sphere or ring torus and a NURBS surface traced on the wall's \
                       chart",
        })?;
        finite_operand("curved partner", partner)?;
        let read = ImplicitWall::of(wall, domain, band)?;
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

    /// The wall's chart enclosures, minted once for the section.
    pub(crate) fn boxes(&self) -> &super::enclose::NurbsBoxes<'a, f64> {
        self.lane.wall.boxes()
    }

    /// The accounting floor over the wall's rectangle.
    pub(crate) fn account_floor(&self) -> SweepFloor<UvRect> {
        self.lane.wall.account_floor()
    }

    /// The partner's form, as the section reads it.
    pub(crate) fn curved(&self) -> &Curved {
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
    /// The boundary pass's, the seeding sweep's, any refusal of the
    /// trace, the fit or the certificate on a branch between crossings
    /// or of a closed loop's march where no tube holds its seed, and the
    /// accounting's ([`SsiError::ExhaustivenessInconclusive`],
    /// [`SsiError::CellBudget`]).
    pub fn all(mut self) -> Result<SsiOutcome, SsiError> {
        let lane = &self.lane;
        let pass = passed(&self.pass, || lane.wall.pass(&lane.curved))?;
        let seeds = exhaust::seed_chart_partner(lane.seed_floor, |cell| lane.excluded(cell))?;
        #[allow(clippy::cast_possible_truncation)]
        let seed_count = seeds.len() as u32;

        let ends = lane.ends()?;
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
