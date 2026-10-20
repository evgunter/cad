//! **NURBS × NURBS: one complete section, read at a seed or whole.**
//!
//! The section of two NURBS charts is the full solution set of
//! `S₁(u₁, v₁) = S₂(u₂, v₂)` over the product of the two knot
//! rectangles, traced in ℝ⁴ on the state `(u₁, v₁, u₂, v₂)` by the same
//! marcher, fit and C2 certificate the plane × NURBS lane uses, its
//! limb 3 the two-chart tube ([`super::four`]). [`Section::of`] returns
//! a handle that runs the boundary pass once, at the first reading that
//! needs its crossings, so a seed refused before any trace (off both
//! charts, or where they are tangent) costs no pass. Its two readings are
//! one contract:
//!
//! - [`Section::branch_at`] answers the branch through a seed: a seed
//!   that settles into a tube already certified is that branch; any
//!   other is traced to the crossings it reaches, fitted, refined and
//!   certified, and the answer is the branch whose certified tube holds
//!   the settled seed. What is proved is that the answer is the branch
//!   through the point Newton settled on, not that this is the branch
//!   nearest the seed.
//! - [`Section::all`] answers every branch: each open branch between
//!   crossings, each closed loop from the subdivision's seeds, and the
//!   accounting that every cell of the product is excluded or lies in a
//!   tube. Exhaustiveness is `all`'s obligation.
//!
//! # The boundary pass
//!
//! A side of either rectangle is a face of the product: one coordinate
//! held, three equations in three unknowns. The pass walks all eight
//! faces ([`super::four::face_roots`]) and keeps each solution the
//! Krawczyk test isolates, settled onto both surfaces with its face's
//! coordinate held: those are the crossings, the only ends an open
//! branch has. A face the walk cannot resolve — the section touching or
//! running along a side, or through a corner, of either rectangle —
//! refuses ([`SsiError::PairBoundaryUnresolved`]): this lane reports no
//! side or corner region, where the plane × NURBS pass reports one.
//!
//! The selection's one unproven step, Newton's convergence to the
//! component nearest the seed, is `super::handle`'s.

use std::cell::OnceCell;

use geom::NurbsSurface;
use geom_core::{Band, Point3, SupSpeed};

use super::boundary::{BoundaryChart, BoundaryPoint, ChartEnd, ChartSide, Crossing};
use super::certify::SsiTube;
use super::enclose::Box3;
use super::ends::Ends;
use super::exhaust::{self, FloorKind, PairCell, SweepFloor};
use super::four::{Box4, PAIR_BOUNDARY_PIECES, PairBoxes, face_roots};
use super::handle::{Banked, foot, passed};
use super::march::{MarchContext, MarchTol, Readout};
use super::system::{Chart, LocalSystem, ParametricPairR4};
use super::{
    ChartAxis, ChartedNurbs, SSI_MAX_STEPS, SSI_NEWTON_ITERS, SSI_SPLINE_NOISE_ULPS, SsiBranch,
    SsiDomain, SsiError, SsiOutcome, finite_net,
};
use geom_core::linalg::svd::Svd;

/// What the section is read over: the two charts and the march, fixed
/// by [`Section::of`].
struct PairLane<'a> {
    a: ChartedNurbs<'a, f64>,
    b: ChartedNurbs<'a, f64>,
    sys: ParametricPairR4<'a>,
    ctx: MarchContext<4>,
    domain: SsiDomain,
    band: Band,
    speeds: [f64; 4],
}

/// **The section of two NURBS charts** (module docs): the boundary pass
/// run once, and every branch certified so far.
pub struct Section<'a> {
    lane: PairLane<'a>,
    /// The boundary pass's crossings, once a reading has run it.
    crossings: OnceCell<Vec<Crossing>>,
    banked: Banked<Box4>,
}

impl core::fmt::Debug for Section<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let Self {
            lane,
            crossings,
            banked,
        } = self;
        f.debug_struct("Section")
            .field("domain", &lane.domain)
            .field("band", &lane.band)
            .field("speeds", &lane.speeds)
            .field("crossings", &crossings.get().map(Vec::len))
            .field("branches", &banked.branches.len())
            .finish_non_exhaustive()
    }
}

/// The control hull of a net, as a box.
fn net_box(s: &NurbsSurface<f64>) -> Option<Box3> {
    s.control()
        .iter()
        .map(|&p| Box3::between(p, p))
        .reduce(Box3::hull)
}

impl<'a> Section<'a> {
    /// **The section of `s1` and `s2`**: the operands checked and both
    /// charts' speeds minted; the boundary pass over the product of their
    /// knot rectangles runs once, at the first reading that needs it.
    /// `domain` names the feature extent and
    /// the accounting floor's scale; its centre and half-extent, a
    /// window a plane needs, bound nothing here: the rectangles do.
    ///
    /// # Errors
    ///
    /// [`SsiError::OperandNotFinite`], [`SsiError::DomainUnusable`],
    /// [`SsiError::ChartSpeed`] and [`SsiError::SettlingUnresolvable`].
    pub fn of(
        s1: &'a NurbsSurface<f64>,
        s2: &'a NurbsSurface<f64>,
        domain: SsiDomain,
        band: Band,
    ) -> Result<Self, SsiError> {
        Self::of_charted(
            (s1, ChartedNurbs::mint),
            (s2, ChartedNurbs::mint),
            domain,
            band,
        )
    }

    /// [`Section::of`] with each surface's chart had from `chart_1` and
    /// `chart_2`, called where [`Section::of`] mints them (after the
    /// operands and the domain are checked, so the refusals come in the
    /// same order): [`ChartedNurbs::mint`] itself, or a chart a caller
    /// minted once for the operation and hands to every section of that
    /// surface. **Each chart must be of the surface it is paired with**,
    /// as a mint of it is: the door checks that it is the very surface,
    /// in every build, and refuses otherwise.
    ///
    /// # Errors
    ///
    /// As [`Section::of`], with the charts' own refusals where they mint,
    /// and [`SsiError::WrongLane`] where a chart is of another surface.
    pub fn of_charted(
        (s1, chart_1): (
            &'a NurbsSurface<f64>,
            impl FnOnce(&'a NurbsSurface<f64>) -> Result<ChartedNurbs<'a, f64>, SsiError>,
        ),
        (s2, chart_2): (
            &'a NurbsSurface<f64>,
            impl FnOnce(&'a NurbsSurface<f64>) -> Result<ChartedNurbs<'a, f64>, SsiError>,
        ),
        domain: SsiDomain,
        band: Band,
    ) -> Result<Self, SsiError> {
        finite_net("first NURBS surface", s1)?;
        finite_net("second NURBS surface", s2)?;
        domain.check()?;
        let (a, b) = (chart_1(s1)?, chart_2(s2)?);
        if !(core::ptr::eq(a.surface(), s1) && core::ptr::eq(b.surface(), s2)) {
            return Err(SsiError::WrongLane {
                expected: "each surface paired with a chart minted from it (`Section::of_charted`)",
            });
        }
        let (sa, sb) = (a.speeds(), b.speeds());
        let speeds = [sa.u.get(), sa.v.get(), sb.u.get(), sb.v.get()];
        let r = Box4::of_charts(s1, s2);
        let rect = |k: usize| super::exhaust::UvRect {
            u: r.0[k],
            v: r.0[k + 1],
        };
        let reach = match (net_box(s1), net_box(s2)) {
            (Some(p), Some(q)) => p.hull(q),
            _ => {
                return Err(SsiError::WrongLane {
                    expected: "two NURBS surfaces with control points",
                });
            }
        };
        let readout = Readout::spatial(reach, None, SSI_SPLINE_NOISE_ULPS)
            .or_chart(rect(0), sa.max(), SSI_SPLINE_NOISE_ULPS)
            .or_chart(rect(2), sb.max(), SSI_SPLINE_NOISE_ULPS);
        let tol = MarchTol::from_band(band, readout)?;
        let ctx = MarchContext::<4> {
            domain: r.0.map(|(lo, hi)| [lo, hi]),
            extent: domain.extent,
            tol,
            max_steps: SSI_MAX_STEPS,
            spent: Default::default(),
        };
        let sys = ParametricPairR4 {
            a: Chart::Nurbs(s1),
            b: Chart::Nurbs(s2),
        };
        Ok(Self {
            lane: PairLane {
                a,
                b,
                sys,
                ctx,
                domain,
                band,
                speeds,
            },
            crossings: OnceCell::new(),
            banked: Banked::new(Self::tube_of),
        })
    }
}

impl<'a> PairLane<'a> {
    /// The pair's boxes.
    fn boxes(&self) -> PairBoxes<'a, f64> {
        PairBoxes::new(&self.a, &self.b, self.speeds)
    }

    /// The ends' reader over this pair.
    fn ends(&self) -> Ends<'_> {
        Ends::pair(
            &self.sys,
            self.ctx,
            (self.a.clone(), self.b.clone()),
            self.band,
        )
    }

    /// The boundary pass (module docs): every crossing, in face order
    /// and as each face's walk found them.
    fn boundary_pass(&self) -> Result<Vec<Crossing>, SsiError> {
        let pair = self.boxes();
        let r = Box4::of_charts(self.a.surface(), self.b.surface());
        let roots =
            face_roots(&pair, r, PAIR_BOUNDARY_PIECES).ok_or(SsiError::PairBoundaryUnresolved {
                pieces: PAIR_BOUNDARY_PIECES,
            })?;
        roots
            .into_iter()
            .map(|(q, k, high)| self.settle_crossing(q, k, high))
            .collect()
    }

    /// The crossing isolated in `q` on the face holding coordinate `k`
    /// at its `high` or low end: Newton on the three free coordinates,
    /// from `q`'s middle, settled within the march's tolerance and
    /// inside `q`, which holds exactly one solution.
    fn settle_crossing(&self, q: Box4, k: usize, high: bool) -> Result<Crossing, SsiError> {
        let free: Vec<usize> = (0..4).filter(|&i| i != k).collect();
        let mut x = q.0.map(|(lo, hi)| 0.5 * (lo + hi));
        let tol = self.ctx.tol.settling();
        let chart = if k < 2 {
            BoundaryChart::First
        } else {
            BoundaryChart::Second
        };
        let side = ChartSide {
            fixed: if k.is_multiple_of(2) {
                ChartAxis::U
            } else {
                ChartAxis::V
            },
            end: if high { ChartEnd::High } else { ChartEnd::Low },
        };
        let along = k ^ 1;
        let refuse = || SsiError::EndNotOnLocus {
            side,
            bracket: q.0[along],
        };
        let mut settled = false;
        for _ in 0..SSI_NEWTON_ITERS {
            let f = self.sys.residual(&x);
            if f.iter().any(|v| v.is_nan()) {
                return Err(refuse());
            }
            if f.iter().all(|v| v.abs() <= tol) {
                settled = true;
                break;
            }
            let j = self.sys.jacobian(&x);
            let m: [[f64; 3]; 3] =
                core::array::from_fn(|r| core::array::from_fn(|c| j[r][free[c]]));
            let svd = Svd::<3, 3>::new(m);
            let dx = svd.solve_min_norm(&f.map(|v| -v));
            for (c, &i) in free.iter().enumerate() {
                x[i] += dx[c];
            }
        }
        // `q` holds the one solution, and its sides can be as thin as a
        // coordinate the two charts share exactly: the settled state is
        // read in it, and must still settle there.
        for (c, (lo, hi)) in x.iter_mut().zip(q.0) {
            *c = c.clamp(lo, hi);
        }
        if !settled
            || self
                .sys
                .residual(&x)
                .iter()
                .any(|v| v.abs().is_nan() || v.abs() > tol)
        {
            return Err(refuse());
        }
        Ok(Crossing {
            at: BoundaryPoint {
                chart,
                side,
                t: x[along],
            },
            state: x,
        })
    }
}

impl Section<'_> {
    /// The crossings the boundary pass found.
    ///
    /// # Errors
    ///
    /// The boundary pass's refusals: [`SsiError::PairBoundaryUnresolved`],
    /// and [`SsiError::EndNotOnLocus`] for a crossing that will not
    /// settle.
    pub fn crossings(&self) -> Result<Vec<BoundaryPoint>, SsiError> {
        let pass = passed(&self.crossings, || self.lane.boundary_pass())?;
        Ok(pass.iter().map(|c| c.at).collect())
    }

    /// The tube a certified branch proved, as ℝ⁴ boxes: the product of
    /// its two pcurves' padded span windows, as limb 3 read them.
    fn tube_of(branch: &SsiBranch) -> Vec<Box4> {
        let (Some(pa), Some(pb), SsiTube::TwoChart { pad_a, pad_b, .. }) =
            (&branch.pcurve_a, &branch.pcurve_b, branch.certificate.tube)
        else {
            return Vec::new();
        };
        super::certify::pair_tube_chain((pa, pb), (pad_a, pad_b))
            .map(|chain| chain.into_iter().map(|(b, _)| b).collect())
            .unwrap_or_default()
    }

    /// **The branch through `seed`** (module docs). The seed is a point
    /// in space; each chart's foot of it, settled onto the section by
    /// Newton, is the state the answer is selected by. The answer is the
    /// branch through that settled state, wherever Newton took it: that
    /// it is the component nearest the seed is not proved, and no
    /// distance from the seed is refused.
    ///
    /// # Errors
    ///
    /// [`SsiError::FootPointInconclusive`] when a foot will not
    /// converge, [`SsiError::SeedRefinementFailed`] when the state will
    /// not settle onto the section, [`SsiError::SeedOffDomain`] when it
    /// settles outside either rectangle (the section leaves the charts'
    /// windows there), the transversality decision at the settled state
    /// ([`SsiError::TransversalityBand`] where the two surfaces are
    /// tangent there), the boundary pass's ([`Section::crossings`]), any
    /// refusal of the trace, the fit or the
    /// certificate, and [`SsiError::SeedOutsideTube`] where the certified
    /// branch's tube does not hold the settled state (a trace or
    /// certificate fault; it does not detect a seed settled onto another
    /// component).
    pub fn branch_at(&mut self, seed: Point3<f64>) -> Result<&SsiBranch, SsiError> {
        let lane = &self.lane;
        let (fa, fb) = (foot(lane.a.surface(), seed)?, foot(lane.b.surface(), seed)?);
        let x = lane.ends().settle([fa.0, fa.1, fb.0, fb.1])?;
        if let Some(i) = self.banked.holding(&x) {
            return Ok(&self.banked.branches[i]);
        }
        let crossings = passed(&self.crossings, || lane.boundary_pass())?;
        let i = self.banked.through(&lane.ends(), x, crossings)?;
        Ok(&self.banked.branches[i])
    }

    /// **Every branch, and the proof they are all of them** (module
    /// docs). Branches [`Section::branch_at`] already certified are kept,
    /// and their crossings are not traced again.
    ///
    /// # Errors
    ///
    /// The boundary pass's ([`Section::crossings`]), any refusal of the
    /// trace, the fit or the certificate on a branch
    /// between crossings, of a closed loop's march where no tube holds
    /// its seed, and the accounting's ([`SsiError::ExhaustivenessInconclusive`],
    /// [`SsiError::CellBudget`], [`SsiError::FloorUnresolvable`]).
    pub fn all(mut self) -> Result<SsiOutcome, SsiError> {
        let lane = &self.lane;
        let pair = lane.boxes();
        let (root_cell, speed) = lane.root_cell();
        let seed_floor = SweepFloor::pair(
            root_cell,
            lane.domain.seed_floor(),
            speed,
            FloorKind::Seeding,
        )?;
        let ends = lane.ends();

        // ---- the open branches, between the crossings not yet ended ----
        let crossings = passed(&self.crossings, || lane.boundary_pass())?;
        for branch in ends.branches(&self.banked.open(crossings))? {
            self.banked.bank(branch);
        }

        // ---- the closed loops, from the subdivision's seeds ----
        let seeds = exhaust::seed_pair(&pair, seed_floor)?;
        #[allow(clippy::cast_possible_truncation)]
        let seed_count = seeds.len() as u32;
        self.banked.closed_loops(&ends, seeds, &[])?;

        let exhaustiveness = lane.account(&self.banked.tubes)?;
        Ok(SsiOutcome {
            branches: self.banked.branches,
            boundary: Vec::new(),
            exhaustiveness,
            seeds: seed_count,
        })
    }
}

impl PairLane<'_> {
    /// The product of the two knot rectangles as the pair lane's root
    /// cell, and the fastest of the four chart speeds its floors are
    /// stated at.
    fn root_cell(&self) -> (PairCell, SupSpeed<f64>) {
        let fastest = self.speeds.iter().copied().fold(0.0, f64::max);
        let cell = PairCell {
            b: Box4::of_charts(self.a.surface(), self.b.surface()),
            scale: self.speeds.map(|s| s / fastest),
        };
        (cell, SupSpeed::new(fastest))
    }

    /// **The accounting**: every cell of the product excluded or inside
    /// one of `tubes`, at the domain's floor.
    fn account(&self, tubes: &[Vec<Box4>]) -> Result<super::Exhaustiveness, SsiError> {
        let (root_cell, speed) = self.root_cell();
        let floor = SweepFloor::pair(
            root_cell,
            self.domain.floor(self.band),
            speed,
            FloorKind::Accounting,
        )?;
        let cells: Vec<PairCell> = tubes
            .iter()
            .flatten()
            .map(|&b| PairCell {
                b,
                scale: root_cell.scale,
            })
            .collect();
        exhaust::account_pair(&self.boxes(), &cells, floor)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use geom::NurbsSurface;
    use geom_core::spline::KnotVector;
    use geom_core::{Band, Point3};

    use super::Section;
    use crate::ssi::{SsiDomain, SsiError};

    /// `z = (x − ½)²` over the unit square, and the plane `z = 0.04 +
    /// 0.05·y` over `[−0.2, 1.2]²` as a biquadratic: two branches.
    fn pair() -> (NurbsSurface<f64>, NurbsSurface<f64>) {
        let k = |(a, b): (f64, f64)| KnotVector::clamped(vec![a, a, a, b, b, b], 2).unwrap();
        let net = |(x0, x1): (f64, f64), z: &dyn Fn(usize, usize) -> f64| {
            let control = (0..9)
                .map(|n| {
                    let (i, j) = (n / 3, n % 3);
                    let at = |t: usize| x0 + 0.5 * (x1 - x0) * t as f64;
                    Point3::new(at(i), at(j), z(i, j))
                })
                .collect();
            NurbsSurface::new(k((x0, x1)), k((x0, x1)), control, vec![1.0; 9]).unwrap()
        };
        let trough = net((0.0, 1.0), &|i, _| [0.25, -0.25, 0.25][i]);
        let plane = net((-0.2, 1.2), &|_, j| 0.04 + 0.05 * (-0.2 + 0.7 * j as f64));
        (trough, plane)
    }

    /// **A section of charts minted once reads as one of fresh mints.**
    /// [`Section::of_charted`] handed one chart per surface, minted once
    /// and shared by two sections in turn (the second reads every cell
    /// the first formed), answers a seed with the same branch and tube as
    /// [`Section::of`], bit for bit (`Debug` prints each `f64` exactly).
    #[test]
    fn a_section_of_charts_minted_once_reads_as_fresh_mints() {
        let (s1, s2) = pair();
        let band = Band::new(1e-9, 1e-8).unwrap();
        let domain = SsiDomain {
            center: Point3::new(0.5, 0.5, 0.5),
            half_extent: 2.0,
            extent: 1.0,
            floor_scale: 1.0,
        };
        let y = 0.4;
        let (z, r) = (0.04 + 0.05 * y, (0.04_f64 + 0.05 * y).sqrt());
        // One seed a section keeps the row inside the new-row time bar;
        // the second charted turn still reads every cell the first formed.
        let read = |mut section: Section<'_>| {
            section.branch_at(Point3::new(0.5 - r, y, z)).unwrap();
            format!("{:?} {:?}", section.banked.branches, section.banked.tubes)
        };
        let fresh = read(Section::of(&s1, &s2, domain, band).unwrap());
        let (a, b) = (
            super::ChartedNurbs::mint(&s1).unwrap(),
            super::ChartedNurbs::mint(&s2).unwrap(),
        );
        for turn in 0..2 {
            let charted = Section::of_charted(
                (&s1, |_| Ok(a.clone())),
                (&s2, |_| Ok(b.clone())),
                domain,
                band,
            )
            .unwrap();
            assert!(
                read(charted) == fresh,
                "turn {turn}: the charted section reads otherwise"
            );
        }
    }

    /// **A chart of another surface is refused.** [`Section::of_charted`]
    /// handed `s2`'s chart for `s1` (or `s1`'s for `s2`) refuses with
    /// [`SsiError::WrongLane`] in every build, before any reading:
    /// computing on with the other surface's chart would answer for the
    /// wrong wall silently.
    #[test]
    fn a_chart_of_another_surface_is_refused() {
        let (s1, s2) = pair();
        let band = Band::new(1e-9, 1e-8).unwrap();
        let domain = SsiDomain {
            center: Point3::new(0.5, 0.5, 0.5),
            half_extent: 2.0,
            extent: 1.0,
            floor_scale: 1.0,
        };
        let (a, b) = (
            super::ChartedNurbs::mint(&s1).unwrap(),
            super::ChartedNurbs::mint(&s2).unwrap(),
        );
        for (first, second) in [(&b, &b), (&a, &a), (&b, &a)] {
            let got = Section::of_charted(
                (&s1, |_| Ok(first.clone())),
                (&s2, |_| Ok(second.clone())),
                domain,
                band,
            );
            assert!(
                matches!(got, Err(SsiError::WrongLane { .. })),
                "a mispaired chart came back {:?}",
                got.err()
            );
        }
    }

    /// **The exhaustiveness receipt is red if a branch is dropped.** The
    /// accounting `all` runs closes over both branches' tubes, and the
    /// same accounting over one of them refuses at the floor: the other
    /// branch's cells are neither excluded nor accounted.
    #[test]
    fn dropping_a_branch_reddens_the_accounting() {
        let (s1, s2) = pair();
        let band = Band::new(1e-9, 1e-8).unwrap();
        let domain = SsiDomain {
            center: Point3::new(0.5, 0.5, 0.5),
            half_extent: 2.0,
            extent: 1.0,
            floor_scale: 1.0,
        };
        let mut section = Section::of(&s1, &s2, domain, band).unwrap();
        let y = 0.4;
        let (z, r) = (0.04 + 0.05 * y, (0.04_f64 + 0.05 * y).sqrt());
        for x in [0.5 - r, 0.5 + r] {
            section.branch_at(Point3::new(x, y, z)).unwrap();
        }
        assert_eq!(section.banked.tubes.len(), 2, "both branches certified");
        let whole = section
            .lane
            .account(&section.banked.tubes)
            .expect("both tubes close it");
        assert!(whole.accounted > 0, "{whole}");
        for dropped in 0..2 {
            let kept = vec![section.banked.tubes[1 - dropped].clone()];
            let e = section
                .lane
                .account(&kept)
                .expect_err("one branch's cells are left unaccounted");
            assert!(
                matches!(e, SsiError::ExhaustivenessInconclusive(_)),
                "dropping branch {dropped}: {e:?}"
            );
        }
    }
}
