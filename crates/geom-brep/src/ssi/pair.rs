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
//! refuses ([`SsiError::PairBoundaryUnresolved`]), unless the caller
//! handed it in as known:
//!
//! - **A shared row** ([`PairRow`]), a row on a side of both rectangles
//!   with its two certified images, is a [`PairRegion::Side`]. Its
//!   cover is the chain of boxes [`super::four::row_cover`] reads along
//!   it, each at least [`COVER_FLOOR_CELLS`] accounting-floor cells wide
//!   across it, over which the crossing reading is zero-free across the
//!   first chart's coordinate along the row. Each slice of a box at that
//!   coordinate holds a row point and so no other solution: the section
//!   in the cover is the row. Its boxes are contact cells in the
//!   accounting, and the pass walks no piece they hold.
//! - **A shared corner** ([`PairCorner`]), a corner of both rectangles,
//!   is read over a cell as deep along each coordinate (the widest of
//!   the same rungs that reads it). The section's
//!   direction there, the generalized cross product of the Jacobian,
//!   must have every component one-signed over the cell, so every arc
//!   in it is monotone in each coordinate. Where those signs point the
//!   section into the domain along every coordinate, or out along every
//!   one, a branch starts at the corner, a crossing the trace starts
//!   from. Otherwise every arc in the cell leaves the domain along one
//!   coordinate whichever way it runs, so none closes in it: a
//!   [`PairRegion::Corner`], whose cell is a contact cell.
//!
//! A row the caller does not hand in, or a cover or corner the reading
//! does not certify, refuses as an unresolved face does.
//!
//! The selection's one unproven step, Newton's convergence to the
//! component nearest the seed, is `super::handle`'s.

use std::cell::OnceCell;

use geom::NurbsSurface;
use geom_core::{Band, Bounds, Point3, SupSpeed};

use super::boundary::{BoundaryChart, BoundaryPoint, ChartCorner, ChartEnd, ChartSide, Crossing};
use super::certify::SsiTube;
use super::enclose::Box3;
use super::ends::Ends;
use super::exhaust::{self, FloorKind, PairCell, SweepFloor};
use super::four::{
    Box4, PAIR_BOUNDARY_PIECES, PairBoxes, RowCrossing, covered, direction_over,
    face_roots_outside, row_cover,
};
use super::handle::{Banked, foot, passed};
use super::march::{MarchContext, MarchTol, Readout};
use super::system::{Chart, LocalSystem, ParametricPairR4};
use super::{
    ChartAxis, ChartedNurbs, SSI_MAX_STEPS, SSI_NEWTON_ITERS, SSI_SPLINE_NOISE_ULPS, SsiBranch,
    SsiDomain, SsiError, SsiOutcome, finite_net,
};
use crate::Pcurve;
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
    /// The boundary pass, once a reading has run it.
    crossings: OnceCell<Pass>,
    banked: Banked<Box4>,
    /// The rows handed in as known.
    rows: Vec<PairRow>,
    /// The corners handed in as known.
    corners: Vec<PairCorner>,
}

/// What the boundary pass decided: the crossings, the regions of the
/// handed-in rows and corners, and the boxes those regions hold.
#[derive(Clone, Debug, Default)]
struct Pass {
    crossings: Vec<Crossing>,
    regions: Vec<PairRegion>,
    cells: Vec<Box4>,
}

/// How many accounting-floor cells wide a row's cover and a corner's cell
/// are, along each coordinate they reach into the domain (module docs):
/// a floor cell beside the row that its enclosures cannot exclude lies
/// inside them.
pub const COVER_FLOOR_CELLS: f64 = 4.0;

/// **A row of both charts, handed in as a known solution**: each
/// chart's image of one curve over one parameter span, each a row on a
/// side of its chart's rectangle (`description::on_domain_side`). The
/// caller certifies that the two images trace one curve on the two
/// walls; the reading certifies the section beside it.
#[derive(Clone, Debug)]
pub struct PairRow {
    images: [Pcurve<f64>; 2],
    span: (f64, f64),
}

impl PairRow {
    /// The row with `image_a` on `a` and `image_b` on `b` over `span`.
    /// `None` where an image is not a row on a side of its chart, or the
    /// span is not an ordered finite interval.
    #[must_use]
    pub fn new(
        (a, image_a): (&NurbsSurface<f64>, Pcurve<f64>),
        (b, image_b): (&NurbsSurface<f64>, Pcurve<f64>),
        span: (f64, f64),
    ) -> Option<Self> {
        let sided = crate::description::on_domain_side(a, &image_a)
            && crate::description::on_domain_side(b, &image_b);
        (sided && span.0.is_finite() && span.1.is_finite() && span.0 < span.1).then_some(Self {
            images: [image_a, image_b],
            span,
        })
    }

    /// Each image's coordinate along the row and coordinate held, as
    /// state indices, chart `a`'s then `b`'s.
    fn axes(&self) -> [(usize, usize); 2] {
        core::array::from_fn(|i| {
            let along_u = crate::description::row_of(&self.images[i]).is_some_and(|r| r.0);
            let (along, fixed) = if along_u { (0, 1) } else { (1, 0) };
            (2 * i + along, 2 * i + fixed)
        })
    }

    /// The side each image runs along, chart `a`'s then `b`'s.
    fn sides(&self, r: Box4) -> [ChartSide; 2] {
        self.axes().map(|(_, fixed)| {
            let at = self.images[fixed / 2].eval(self.span.0);
            let c = if fixed.is_multiple_of(2) { at.x } else { at.y };
            ChartSide {
                fixed: if fixed.is_multiple_of(2) {
                    ChartAxis::U
                } else {
                    ChartAxis::V
                },
                end: if c == r.0[fixed].0 {
                    ChartEnd::Low
                } else {
                    ChartEnd::High
                },
            }
        })
    }
}

/// **A corner of both charts, handed in as a known solution**: chart
/// `a`'s corner, then chart `b`'s, at which the caller has a shared
/// vertex.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PairCorner(pub [ChartCorner; 2]);

/// A region of the product's boundary the reading certified from what
/// was handed in (module docs).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PairRegion {
    /// A handed-in row: the section in its cover is the row, on these
    /// sides of chart `a` and chart `b`.
    Side {
        /// Chart `a`'s side, then chart `b`'s.
        sides: [ChartSide; 2],
    },
    /// A handed-in corner where the section leaves the domain: every
    /// arc in its cell runs out of the domain along some coordinate.
    Corner {
        /// Chart `a`'s corner, then chart `b`'s.
        corners: [ChartCorner; 2],
    },
}

/// **The whole reading with its regions** ([`Section::read_all`]).
#[derive(Clone, Debug)]
pub struct PairReading {
    /// Every branch and the accounting.
    pub outcome: SsiOutcome,
    /// The regions of the handed-in rows and corners.
    pub regions: Vec<PairRegion>,
}

impl core::fmt::Debug for Section<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let Self {
            lane,
            crossings,
            banked,
            ..
        } = self;
        f.debug_struct("Section")
            .field("domain", &lane.domain)
            .field("band", &lane.band)
            .field("speeds", &lane.speeds)
            .field("crossings", &crossings.get().map(|p| p.crossings.len()))
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
            rows: Vec::new(),
            corners: Vec::new(),
        })
    }

    /// **The section with `rows` and `corners` handed in as known**
    /// (module docs), before any reading runs the boundary pass.
    #[must_use]
    pub fn with_known(mut self, rows: Vec<PairRow>, corners: Vec<PairCorner>) -> Self {
        debug_assert!(
            self.crossings.get().is_none(),
            "known solutions are handed in before the boundary pass"
        );
        self.rows = rows;
        self.corners = corners;
        self
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
    fn boundary_pass(&self, rows: &[PairRow], corners: &[PairCorner]) -> Result<Pass, SsiError> {
        let pair = self.boxes();
        let r = Box4::of_charts(self.a.surface(), self.b.surface());
        let unresolved = || SsiError::PairBoundaryUnresolved {
            pieces: PAIR_BOUNDARY_PIECES,
        };
        let rungs = self.cover_rungs(r)?;
        let mut pass = Pass::default();
        for row in rows {
            let cover = self.row_cover(row, &rungs)?;
            pass.regions.push(PairRegion::Side {
                sides: row.sides(r),
            });
            pass.cells.extend(cover);
        }
        let covers = pass.cells.clone();
        for &PairCorner(corners) in corners {
            let at: [f64; 4] = core::array::from_fn(|k| {
                let end = match (k % 2, corners[k / 2]) {
                    (0, c) => c.u,
                    (_, c) => c.v,
                };
                match end {
                    ChartEnd::Low => r.0[k].0,
                    ChartEnd::High => r.0[k].1,
                }
            });
            // A corner a row's cover holds is that row's end.
            if covered(Box4::at(at), &covers) {
                continue;
            }
            let inward: [bool; 4] = core::array::from_fn(|k| at[k] == r.0[k].0);
            // The widest rung whose cell reads every component of the
            // section's direction one-signed.
            let read = rungs.iter().find_map(|width| {
                let cell = Box4(core::array::from_fn(|k| {
                    let (lo, hi) = r.0[k];
                    if inward[k] {
                        (lo, (lo + width[k]).min(hi))
                    } else {
                        ((hi - width[k]).max(lo), hi)
                    }
                }));
                let signs = direction_over(&pair, cell).map(super::section::sign);
                let signs: Option<Vec<bool>> = signs.into_iter().collect();
                signs.map(|s| (cell, s))
            });
            let Some((cell, signs)) = read else {
                return Err(unresolved());
            };
            let along: Vec<bool> = (0..4).map(|k| signs[k] == inward[k]).collect();
            if along.iter().all(|&x| x) || along.iter().all(|&x| !x) {
                pass.crossings.push(self.corner_crossing(at, corners[0])?);
            } else {
                pass.regions.push(PairRegion::Corner { corners });
            }
            pass.cells.push(cell);
        }
        let roots = face_roots_outside(&pair, r, PAIR_BOUNDARY_PIECES, &pass.cells)
            .ok_or_else(unresolved)?;
        for (q, k, high) in roots {
            pass.crossings.push(self.settle_crossing(q, k, high)?);
        }
        Ok(pass)
    }

    /// **The widths a cover or a corner cell is tried at**, widest
    /// first, in chart units per coordinate: an eighth of the
    /// coordinate's domain, halved down to [`COVER_FLOOR_CELLS`]
    /// accounting-floor cells, which is the last rung (module docs). A
    /// wider cover lets the boundary pass's walk drop its pieces sooner;
    /// the floor's holds every cell beside it the accounting cannot
    /// exclude.
    fn cover_rungs(&self, r: Box4) -> Result<Vec<[f64; 4]>, SsiError> {
        let (root_cell, speed) = self.root_cell();
        let floor = SweepFloor::pair(
            root_cell,
            self.domain.floor(self.band),
            speed,
            FloorKind::Accounting,
        )?;
        let least: [f64; 4] =
            core::array::from_fn(|k| COVER_FLOOR_CELLS * floor.width() / root_cell.scale[k]);
        let mut rungs = Vec::new();
        let mut width: [f64; 4] = core::array::from_fn(|k| (r.0[k].1 - r.0[k].0) / 8.0);
        while (0..4).any(|k| width[k] > least[k]) {
            rungs.push(core::array::from_fn(|k| width[k].max(least[k])));
            width = width.map(|w| 0.5 * w);
        }
        rungs.push(least);
        Ok(rungs)
    }

    /// **A handed-in row's cover** (module docs): the boxes
    /// [`row_cover`] reads along it at the widest of `rungs` it reads
    /// clear (each but the last given up at its first unread piece, the
    /// last read whole, where a turn is looked for), widened across it in chart `a`'s held coordinate, chart
    /// `b`'s held coordinate and chart `b`'s coordinate along it, read
    /// across chart `a`'s coordinate along it.
    ///
    /// # Errors
    ///
    /// [`SsiError::PairRowTangent`] where the walls' tangent planes
    /// coincide at a state of the row, and
    /// [`SsiError::PairBoundaryUnresolved`] where no rung reads every
    /// piece's sign.
    fn row_cover(&self, row: &PairRow, rungs: &[[f64; 4]]) -> Result<Vec<Box4>, SsiError> {
        let [(along_a, held_a), (along_b, held_b)] = row.axes();
        let coordinate =
            |p: geom_core::Point2<f64>, k: usize| if k.is_multiple_of(2) { p.x } else { p.y };
        let read = |lo: f64, hi: f64| {
            let ends = |i: usize| (row.images[i].eval(lo), row.images[i].eval(hi));
            let ((a0, a1), (b0, b1)) = (ends(0), ends(1));
            let hull = |p: f64, q: f64| {
                if p.is_nan() || q.is_nan() {
                    (f64::NAN, f64::NAN)
                } else {
                    (p.min(q), p.max(q))
                }
            };
            let state = Box4([
                hull(a0.x, a1.x),
                hull(a0.y, a1.y),
                hull(b0.x, b1.x),
                hull(b0.y, b1.y),
            ]);
            let mut chord = [0.0; 4];
            chord[along_a] = coordinate(a1, along_a) - coordinate(a0, along_a);
            (state, chord)
        };
        let walls = (self.a.surface(), self.b.surface());
        for (n, width) in rungs.iter().enumerate() {
            let last = n + 1 == rungs.len();
            let mut wide = [0.0; 4];
            for k in [held_a, held_b, along_b] {
                wide[k] = width[k];
            }
            let (crossing, boxes) = row_cover(walls, row.span, wide, !last, read)
                .map_err(|super::four::RowUnreadable(_, refusal)| SsiError::ChartSpeed(refusal))?;
            match crossing {
                RowCrossing::Clear(_) => return Ok(boxes),
                RowCrossing::Turns => return Err(SsiError::PairRowTangent),
                RowCrossing::Unread => {}
            }
        }
        Err(SsiError::PairBoundaryUnresolved {
            pieces: PAIR_BOUNDARY_PIECES,
        })
    }

    /// **The crossing at a handed-in corner** where a branch starts:
    /// the corner state itself, which must lie on the section within the
    /// march's settling tolerance.
    fn corner_crossing(&self, at: [f64; 4], corner: ChartCorner) -> Result<Crossing, SsiError> {
        let side = ChartSide {
            fixed: ChartAxis::U,
            end: corner.u,
        };
        let tol = self.ctx.tol.settling();
        if self
            .sys
            .residual(&at)
            .iter()
            .any(|v| v.is_nan() || v.abs() > tol)
        {
            return Err(SsiError::EndNotOnLocus {
                side,
                bracket: (at[1], at[1]),
            });
        }
        Ok(Crossing {
            at: BoundaryPoint {
                chart: BoundaryChart::First,
                side,
                t: at[1],
            },
            state: at,
        })
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
        let pass = self.pass()?;
        Ok(pass.crossings.iter().map(|c| c.at).collect())
    }

    /// The boundary pass, run once.
    fn pass(&self) -> Result<&Pass, SsiError> {
        passed(&self.crossings, || {
            self.lane.boundary_pass(&self.rows, &self.corners)
        })
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
        let crossings = passed(&self.crossings, || {
            lane.boundary_pass(&self.rows, &self.corners)
        })?;
        let i = self.banked.through(&lane.ends(), x, &crossings.crossings)?;
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
    pub fn all(self) -> Result<SsiOutcome, SsiError> {
        self.read_all().map(|reading| reading.outcome)
    }

    /// **[`Section::all`] with the regions of the rows and corners
    /// handed in** ([`Section::with_known`]): their covers and cells are
    /// contact cells in the accounting and hold no seed.
    ///
    /// # Errors
    ///
    /// As [`Section::all`], and a handed-in row's or corner's reading
    /// ([`SsiError::PairRowTangent`], [`SsiError::PairBoundaryUnresolved`],
    /// [`SsiError::EndNotOnLocus`]).
    pub fn read_all(mut self) -> Result<PairReading, SsiError> {
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
        let pass = passed(&self.crossings, || {
            lane.boundary_pass(&self.rows, &self.corners)
        })?
        .clone();
        for branch in ends.branches(&self.banked.open(&pass.crossings))? {
            self.banked.bank(branch);
        }

        // ---- the closed loops, from the subdivision's seeds ----
        let seeds = exhaust::seed_pair(&pair, seed_floor)?;
        #[allow(clippy::cast_possible_truncation)]
        let seed_count = seeds.len() as u32;
        self.banked.closed_loops(&ends, seeds, &pass.cells)?;

        let exhaustiveness = lane.account(&self.banked.tubes, &pass.cells)?;
        Ok(PairReading {
            outcome: SsiOutcome {
                branches: self.banked.branches,
                boundary: Vec::new(),
                exhaustiveness,
                seeds: seed_count,
            },
            regions: pass.regions,
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
    fn account(
        &self,
        tubes: &[Vec<Box4>],
        regions: &[Box4],
    ) -> Result<super::Exhaustiveness, SsiError> {
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
        let regions: Vec<PairCell> = regions
            .iter()
            .map(|&b| PairCell {
                b,
                scale: root_cell.scale,
            })
            .collect();
        exhaust::account_pair(&self.boxes(), &cells, &regions, floor)
    }
}

/// **An enclosure's centre, and how far every point of it may stand from
/// that centre**, in metres: its half-widths summed, rounded up, which
/// bounds the half-diagonal. `None` for an enclosure with no finite
/// reach.
fn centre_and_reach(s: Box3) -> Option<(Point3<f64>, f64)> {
    let centre = |x: geom_core::Interval| 0.5 * (x.lo() + x.hi());
    let q = Point3::new(centre(s.x), centre(s.y), centre(s.z));
    let reach = |x: geom_core::Interval| (x.hi() - centre(x)).max(centre(x) - x.lo()).next_up();
    let pad = (reach(s.x) + reach(s.y) + reach(s.z)).next_up();
    pad.is_finite().then_some((q, pad))
}

/// The most halvings [`branch_witness`] takes of a slice about the
/// branch's middle before it gives up.
const WITNESS_HALVINGS: u32 = 48;

/// **A point of a certified branch, and how far it may stand from it**,
/// in metres: the centre `q` of the first wall's enclosure over a root
/// the Krawczyk test isolates ([`super::four::slice_root`]) on a slice
/// through the fitted middle state, inside the limb-3 tube box that
/// holds it, and that enclosure's half-widths summed. Limb 3 proves the
/// section in the box one arc, the branch's, so the root is a point of
/// the branch, its image on the first wall in the enclosure. The slice
/// is held at each coordinate in turn and halved about the middle state
/// from the box's own extent until one isolates a root. `None` where the
/// certificate is not a two-chart tube or no slice isolates one.
pub(crate) fn branch_witness(
    (a, b): (&NurbsSurface<f64>, &NurbsSurface<f64>),
    branch: &SsiBranch,
) -> Option<(Point3<f64>, f64)> {
    let (Some(pa), Some(pb), super::SsiTube::TwoChart { pad_a, pad_b, .. }) =
        (&branch.pcurve_a, &branch.pcurve_b, branch.certificate.tube)
    else {
        return None;
    };
    let mid = 0.5 * (branch.params.0 + branch.params.1);
    let (at_a, at_b) = (pa.eval(mid), pb.eval(mid));
    let m = [at_a.x, at_a.y, at_b.x, at_b.y];
    let chain = super::certify::pair_tube_chain((pa, pb), (pad_a, pad_b))?;
    let (held, _) = chain.into_iter().find(|(x, _)| x.holds(&m))?;
    let (ca, cb) = (ChartedNurbs::mint(a).ok()?, ChartedNurbs::mint(b).ok()?);
    let (sa, sb) = (ca.speeds(), cb.speeds());
    let pair = PairBoxes::new(&ca, &cb, [sa.u.get(), sa.v.get(), sb.u.get(), sb.v.get()]);
    let mut half: [f64; 4] = core::array::from_fn(|k| held.0[k].1 - held.0[k].0);
    for _ in 0..WITNESS_HALVINGS {
        for k in 0..4 {
            let slice = Box4(core::array::from_fn(|i| {
                if i == k {
                    (m[i], m[i])
                } else {
                    (m[i] - half[i], m[i] + half[i])
                }
            }))
            .meet(held);
            let Some(root) = slice.and_then(|x| super::four::slice_root(&pair, x, k)) else {
                continue;
            };
            let [(u0, u1), (v0, v1), ..] = root.0;
            let s = super::enclose::NurbsBoxes::new(a)
                .rect_box(super::enclose::UvWindow::new(u0, u1, v0, v1)?);
            return centre_and_reach(s);
        }
        half = half.map(|h| 0.5 * h);
    }
    None
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use geom::NurbsSurface;
    use geom_core::spline::KnotVector;
    use geom_core::{Band, Bounds, Point3};

    use super::Section;
    use crate::ssi::enclose::Box3;
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

    /// The biquadratic standing over `xs × ys` on the unit square, `x`
    /// along `u` and `y` along `v`, its height `z(x, y)` at each control
    /// point, which a function bilinear in `x` and `y` reproduces.
    fn sheet(xs: (f64, f64), ys: (f64, f64), z: &dyn Fn(f64, f64) -> f64) -> NurbsSurface<f64> {
        let k = || KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
        let at = |(lo, hi): (f64, f64), i: usize| lo + 0.5 * (hi - lo) * i as f64;
        let control = (0..9)
            .map(|n| {
                let (x, y) = (at(xs, n / 3), at(ys, n % 3));
                Point3::new(x, y, z(x, y))
            })
            .collect();
        NurbsSurface::new(k(), k(), control, vec![1.0; 9]).unwrap()
    }

    /// **A corner where the section runs into both rectangles is a
    /// crossing, not a region.** The sheet `z = 0.2·x·y` and the plane
    /// `z = x − y/2` over `y ∈ [0, 1.5]` share the corner `(0, 0)` of both
    /// rectangles, and the section leaves it increasing in every
    /// coordinate: handed in, the corner is a crossing of the pass, and
    /// no region. Red against the corner's signs read as leaving.
    #[test]
    fn a_corner_a_branch_starts_at_is_a_crossing() {
        use crate::ssi::{BoundaryChart, ChartCorner, ChartEnd};
        let flat = sheet((0.0, 1.0), (0.0, 1.0), &|x, y| 0.2 * x * y);
        let enters = sheet((0.0, 1.0), (0.0, 1.5), &|x, y| x - 0.5 * y);
        let band = Band::new(1e-9, 1e-8).unwrap();
        let domain = SsiDomain {
            center: Point3::new(0.5, 0.5, 0.0),
            half_extent: 4.0,
            extent: 2.0,
            floor_scale: 1.0,
        };
        let low = ChartCorner {
            u: ChartEnd::Low,
            v: ChartEnd::Low,
        };
        let section = Section::of(&flat, &enters, domain, band)
            .unwrap()
            .with_known(Vec::new(), vec![super::PairCorner([low, low])]);
        let pass = section.pass().expect("the pass reads the corner");
        assert!(pass.regions.is_empty(), "no region: {:?}", pass.regions);
        assert!(
            pass.crossings
                .iter()
                .any(|c| c.state == [0.0; 4] && c.at.chart == BoundaryChart::First),
            "the corner is a crossing: {:?}",
            pass.crossings
        );
    }

    /// **A witness's pad reaches every point of its enclosure.** Each
    /// corner of a lopsided box stands within the pad of its centre, and
    /// the farthest one more than half the pad off, so the pad is no
    /// looser than twice the half-diagonal. Red against the pad dropped
    /// and against the pad halved (the half-diagonal of a box exceeds half
    /// its half-widths' sum).
    #[test]
    fn a_witness_pad_reaches_its_whole_enclosure() {
        use geom_core::Interval;
        let s = Box3 {
            x: Interval::from_bounds(1.0, 1.0 + 3e-9),
            y: Interval::from_bounds(-2.0, -2.0 + 4e-9),
            z: Interval::from_bounds(0.5, 0.5 + 1e-9),
        };
        let (q, pad) = super::centre_and_reach(s).expect("a finite enclosure");
        let corners = [s.x.lo(), s.x.hi()].into_iter().flat_map(|x| {
            [s.y.lo(), s.y.hi()]
                .into_iter()
                .flat_map(move |y| [s.z.lo(), s.z.hi()].map(|z| Point3::new(x, y, z)))
        });
        let far = corners.map(|c| c.distance(q)).fold(0.0, f64::max);
        assert!(
            far <= pad && 0.5 * pad < far,
            "the farthest corner stands {far:e} from the centre, against the pad {pad:e}"
        );
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
            .account(&section.banked.tubes, &[])
            .expect("both tubes close it");
        assert!(whole.accounted > 0, "{whole}");
        for dropped in 0..2 {
            let kept = vec![section.banked.tubes[1 - dropped].clone()];
            let e = section
                .lane
                .account(&kept, &[])
                .expect_err("one branch's cells are left unaccounted");
            assert!(
                matches!(e, SsiError::ExhaustivenessInconclusive(_)),
                "dropping branch {dropped}: {e:?}"
            );
        }
    }
}
