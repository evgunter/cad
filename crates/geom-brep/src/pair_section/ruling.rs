//! **The ruling sweep** (C3, C5): a pair with a ruled side read along that
//! side's rulings. Each ruling is a line, and it meets the partner where a
//! quadratic in the line's parameter vanishes, in closed form: nowhere
//! where the discriminant `Δ` is negative, twice where it is positive (two
//! sheets of meetings), once where it vanishes. A simple zero of `Δ` is a
//! branch point, where the two sheets join; a double one is a tangency,
//! which the pose decides before any sweep.
//!
//! The ruling angle `φ` is cut into cells, each read on enclosures of `Δ`,
//! of the two meetings and of their velocity over the cell ([`Ruled`]).
//! The sweep then:
//!
//! - **counts the components**: an arc of `φ` where `Δ > 0`, bounded by
//!   two branch points, is one loop (its two sheets joined at both), and
//!   `Δ > 0` over the whole turn is two loops, one per sheet;
//! - **cuts them by the domain**: a cut end is a certified root where a
//!   sheet crosses the boundary of the ball the march ends at, in a cell
//!   where that sheet is monotone against the ball and its two ends lie
//!   on opposite sides; the stretches between cut ends that lie inside
//!   the ball are the open components, and a loop no root cuts is one
//!   component inside the ball or none;
//! - **witnesses** each component it lists at a point decided inside the
//!   ball, the march's seed.
//!
//! A branch point is read sheet-wise: its cell encloses both meetings
//! through the discriminant's root over `[0, √Δ_max]`, which decides the
//! ball's side for both sheets at once. Where it cannot, the cell is
//! halved toward the branch point, the half clear of it joining the arc;
//! a branch cell at the floor escalates, a boundary within band of a
//! branch point.
//!
//! Every cell is decided at the band through `k_stats` (D4). A cell no
//! test settles is split, down to a floor; a cell at the floor, or the
//! cell that spends the budget, escalates on its own reading.

use geom_core::k_stats::{decide, decide_reported, gate_rejection, straddle};
use geom_core::{
    Band, Decide, Indeterminate, Margin, Point3, Real, Sign, SizedPass, Vec3, rounding_charge,
};

use crate::ExtentBall;

/// The cells the first cut makes of the turn.
const START: usize = 32;
/// The most cells one sweep reads before it escalates.
const BUDGET: usize = 1 << 15;
/// The narrowest cell the sweep splits, in radians: `2π / 2⁴⁰`.
#[allow(clippy::cast_precision_loss)]
const FLOOR: f64 = core::f64::consts::TAU / (1u64 << 40) as f64;
/// The bisections that place a cut end in its isolating cell.
const SETTLE: u32 = 48;

/// `Δ`'s sign over a cell, as a margin in metres.
const ROW_DISCRIMINANT: &str = "ruling_discriminant";
/// `Δ`'s slope's sign over a cell.
const ROW_SLOPE: &str = "ruling_discriminant_slope";
/// A meeting's side of the ball the march ends at.
const ROW_CUT: &str = "ruling_cut";
/// A sheet's monotonicity against that ball over a cell.
const ROW_MONOTONE: &str = "ruling_cut_monotone";
/// Both meetings' side of the ball over a branch point's cell.
const ROW_BRANCH: &str = "ruling_branch_cut";

/// A row's straddle row: a floor cell's two bounds straddling the band.
fn straddle_row(name: &'static str) -> &'static str {
    match name {
        ROW_DISCRIMINANT => "ruling_discriminant_straddle",
        ROW_BRANCH => "ruling_branch_cut_straddle",
        _ => "ruling_cut_straddle",
    }
}

/// **A closed interval of a scalar**, its two ends read at the scalar:
/// the sweep's enclosure of a quantity over a cell of the ruling angle.
/// The ends are combined as an interval's are, so at `Interval` each end
/// is itself an enclosure and the bound is certified; at `f64` each
/// reading is widened by its rounding in the `f64` lane's sense
/// ([`rounding_charge`]), the precedent of `gap`'s sweep.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Iv<T> {
    pub(crate) lo: T,
    pub(crate) hi: T,
}

impl<T: Real> Iv<T> {
    pub(crate) fn point(x: T) -> Self {
        Self { lo: x, hi: x }
    }

    fn of(a: T, b: T) -> Self {
        Self {
            lo: a.min(b),
            hi: a.max(b),
        }
    }

    fn hull(self, o: Self) -> Self {
        Self {
            lo: self.lo.min(o.lo),
            hi: self.hi.max(o.hi),
        }
    }

    /// Widened by `e` each way.
    fn widen(self, e: T) -> Self {
        Self {
            lo: self.lo - e,
            hi: self.hi + e,
        }
    }

    fn plus(self, o: Self) -> Self {
        Self {
            lo: self.lo + o.lo,
            hi: self.hi + o.hi,
        }
    }

    fn shift(self, k: T) -> Self {
        Self {
            lo: self.lo + k,
            hi: self.hi + k,
        }
    }

    fn scale(self, k: T) -> Self {
        Self::of(self.lo * k, self.hi * k)
    }

    fn times(self, o: Self) -> Self {
        let (a, b, c, d) = (
            self.lo * o.lo,
            self.lo * o.hi,
            self.hi * o.lo,
            self.hi * o.hi,
        );
        Self {
            lo: a.min(b).min(c.min(d)),
            hi: a.max(b).max(c.max(d)),
        }
    }

    /// `k − self`.
    fn from(self, k: T) -> Self {
        Self {
            lo: k - self.hi,
            hi: k - self.lo,
        }
    }

    fn abs(self) -> Self {
        Self {
            lo: self.lo.max(T::zero() - self.hi).max(T::zero()),
            hi: (T::zero() - self.lo).max(self.hi),
        }
    }

    fn square(self) -> Self {
        let a = self.abs();
        Self {
            lo: a.lo * a.lo,
            hi: a.hi * a.hi,
        }
    }

    /// The root of the part of `self` at or above zero.
    fn sqrt(self) -> Self {
        Self {
            lo: self.lo.max(T::zero()).sqrt(),
            hi: self.hi.max(T::zero()).sqrt(),
        }
    }

    /// `self / o`, where `o` lies above zero.
    fn over(self, o: Self) -> Self {
        Self::of(self.lo / o.lo, self.lo / o.hi).hull(Self::of(self.hi / o.lo, self.hi / o.hi))
    }
}

/// **An enclosure of a point**, axis by axis.
#[derive(Clone, Copy, Debug)]
pub(crate) struct V3<T> {
    pub(crate) x: Iv<T>,
    pub(crate) y: Iv<T>,
    pub(crate) z: Iv<T>,
}

impl<T: Real> V3<T> {
    fn at(p: Point3<T>) -> Self {
        Self {
            x: Iv::point(p.x),
            y: Iv::point(p.y),
            z: Iv::point(p.z),
        }
    }

    /// `v · k`.
    fn along(v: Vec3<T>, k: Iv<T>) -> Self {
        Self {
            x: k.scale(v.x),
            y: k.scale(v.y),
            z: k.scale(v.z),
        }
    }

    fn plus(self, o: Self) -> Self {
        Self {
            x: self.x.plus(o.x),
            y: self.y.plus(o.y),
            z: self.z.plus(o.z),
        }
    }

    fn widen(self, e: T) -> Self {
        Self {
            x: self.x.widen(e),
            y: self.y.widen(e),
            z: self.z.widen(e),
        }
    }

    /// `self − p`.
    fn less(self, p: Point3<T>) -> Self {
        Self {
            x: self.x.shift(T::zero() - p.x),
            y: self.y.shift(T::zero() - p.y),
            z: self.z.shift(T::zero() - p.z),
        }
    }

    fn dot(self, o: Self) -> Iv<T> {
        self.x
            .times(o.x)
            .plus(self.y.times(o.y))
            .plus(self.z.times(o.z))
    }

    fn norm(self) -> Iv<T> {
        self.x
            .square()
            .plus(self.y.square())
            .plus(self.z.square())
            .sqrt()
    }

    /// The enclosure's centre: a selection, not a bound.
    pub(crate) fn mid(self) -> Point3<T> {
        let half = T::from_f64(0.5);
        let mid = |i: Iv<T>| (i.lo + i.hi) * half;
        Point3::new(mid(self.x), mid(self.y), mid(self.z))
    }
}

/// **A cell of the ruling angle**, `[lo, hi]` radians within `[0, 2π]`,
/// with enclosures of its cosine and sine.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Cell<T> {
    pub(crate) lo: f64,
    pub(crate) hi: f64,
    pub(crate) cos: Iv<T>,
    pub(crate) sin: Iv<T>,
}

impl<T: Real> Cell<T> {
    /// The cell `[lo, hi]`: the cosine and sine read at its ends, widened
    /// to `±1` where the cell holds a turning point of either (within a
    /// hair of it, so a turning point on a cut is held by both cells).
    pub(crate) fn new(lo: f64, hi: f64) -> Self {
        use core::f64::consts::{FRAC_PI_2, PI, TAU};
        let (sa, ca) = T::from_f64(lo).sin_cos();
        let (sb, cb) = T::from_f64(hi).sin_cos();
        let (mut cos, mut sin) = (Iv::of(ca, cb), Iv::of(sa, sb));
        let holds = |e: f64| lo - e <= 1e-12 && hi - e >= -1e-12;
        if holds(0.0) || holds(TAU) {
            cos.hi = T::one();
        }
        if holds(PI) {
            cos.lo = T::zero() - T::one();
        }
        if holds(FRAC_PI_2) {
            sin.hi = T::one();
        }
        if holds(PI + FRAC_PI_2) {
            sin.lo = T::zero() - T::one();
        }
        Self { lo, hi, cos, sin }
    }

    /// The single angle `phi`.
    pub(crate) fn at(phi: f64) -> Self {
        Self::new(phi, phi)
    }

    fn mid(&self) -> f64 {
        0.5 * (self.lo + self.hi)
    }

    fn halves(&self) -> (Self, Self) {
        let m = self.mid();
        (Self::new(self.lo, m), Self::new(m, self.hi))
    }

    fn width(&self) -> f64 {
        self.hi - self.lo
    }
}

/// **The two sheets of meetings**: the quadratic's larger root and its
/// smaller, joined at a branch point.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Sheet {
    Plus,
    Minus,
}

impl Sheet {
    fn sign<T: Real>(self) -> T {
        match self {
            Self::Plus => T::one(),
            Self::Minus => T::zero() - T::one(),
        }
    }
}

/// **A ruled side against its partner**, read over cells of the ruling
/// angle: the shape every ruled pair (a cylinder or a cone against a
/// plane, a cylinder, a cone or a sphere) hands the sweep.
pub(crate) trait Ruled<T: Real> {
    /// How far a ruling moves per radian of its angle: the lever a turn
    /// of the angle is read at.
    fn lever(&self) -> T;
    /// `Δ` over the cell as a margin in metres, of `Δ`'s sign.
    fn discriminant(&self, cell: &Cell<T>) -> Iv<T>;
    /// `Δ′` over the cell, of its sign, in metres per radian.
    fn discriminant_slope(&self, cell: &Cell<T>) -> Iv<T>;
    /// The meetings of the cell's rulings on `sheet`, the discriminant's
    /// root enclosed through zero where `Δ` straddles it.
    fn meeting(&self, cell: &Cell<T>, sheet: Sheet) -> V3<T>;
    /// Their velocity along the angle, over a cell where `Δ` is decided
    /// positive.
    fn velocity(&self, cell: &Cell<T>, sheet: Sheet) -> V3<T>;
}

/// **The thinner of two cylinders swept against the other.** The ruled
/// wall is `foot + ρ(cos φ·n + sin φ·v) + t·axis`, `n` square to both axes
/// and `v = axis × n`; the partner's axis runs through `foot + gap·n`
/// along `axis′`, with `cos = axis·axis′` and `beta = axis′·v`
/// (`cos² + beta² = 1`, `beta ≠ 0` for crossing axes). A ruling meets the
/// partner where `β²t² − 2β·ρ sin φ cos·t + C = 0`, whose discriminant is
/// `β²(r² − x²)`, `x = ρ cos φ − gap`, and whose roots are
/// `t = (ρ sin φ cos ± √(r² − x²))/β`. Its extremes over the turn are the
/// pose's margins: `r + ρ − e` (the gap level, where `e ≥ ρ`) and
/// `r − ρ − e` (the nest level).
#[derive(Clone, Copy, Debug)]
pub(crate) struct RuledCylinders<T: Real> {
    pub(crate) foot: Point3<T>,
    pub(crate) n: Vec3<T>,
    pub(crate) v: Vec3<T>,
    pub(crate) axis: Vec3<T>,
    /// The ruled wall's radius.
    pub(crate) rho: T,
    /// The partner's.
    pub(crate) r: T,
    pub(crate) gap: T,
    pub(crate) cos: T,
    pub(crate) beta: T,
}

impl<T: Real> RuledCylinders<T> {
    /// `x = ρ cos φ − gap` over the cell.
    fn x(&self, cell: &Cell<T>) -> Iv<T> {
        cell.cos.scale(self.rho).shift(T::zero() - self.gap)
    }

    /// `√(r² − x²)` over the cell, through zero where it straddles it,
    /// the radicand widened by its rounding.
    fn root(&self, cell: &Cell<T>) -> Iv<T> {
        let x = self.x(cell);
        let reach = self.rho + self.gap.abs();
        x.square()
            .from(self.r * self.r)
            .widen(rounding_charge(self.r * self.r + reach * reach))
            .sqrt()
    }

    /// The `1/β` the roots are divided by.
    fn tilt(&self) -> T {
        T::one() / self.beta
    }

    /// **The rounding the meetings carry** ([`rounding_charge`]): the
    /// frame's coordinates are of size `S`, `|foot| + ρ + r + |gap|`, and
    /// a root's numerator of `ρ + r`, divided by `β`; a ruling nearly
    /// parallel to the partner's axis levers both by `1/|β|`.
    fn charge(&self) -> (T, T) {
        let size = (self.foot - Point3::origin()).norm() + self.rho + self.r + self.gap.abs();
        let t = rounding_charge(size + self.rho + self.r) * self.tilt().abs();
        (rounding_charge(size), t)
    }
}

impl<T: Real> Ruled<T> for RuledCylinders<T> {
    fn lever(&self) -> T {
        self.rho
    }

    fn discriminant(&self, cell: &Cell<T>) -> Iv<T> {
        self.x(cell)
            .abs()
            .from(self.r)
            .widen(rounding_charge(self.rho + self.gap.abs() + self.r))
    }

    fn discriminant_slope(&self, cell: &Cell<T>) -> Iv<T> {
        self.x(cell)
            .times(cell.sin)
            .widen(rounding_charge(self.rho + self.gap.abs()))
    }

    fn meeting(&self, cell: &Cell<T>, sheet: Sheet) -> V3<T> {
        let (point, along) = self.charge();
        let t = cell
            .sin
            .scale(self.rho * self.cos)
            .plus(self.root(cell).scale(sheet.sign()))
            .scale(self.tilt())
            .widen(along);
        V3::at(self.foot)
            .plus(V3::along(self.n, cell.cos.scale(self.rho)))
            .plus(V3::along(self.v, cell.sin.scale(self.rho)))
            .plus(V3::along(self.axis, t))
            .widen(point)
    }

    fn velocity(&self, cell: &Cell<T>, sheet: Sheet) -> V3<T> {
        // `x′ = −ρ sin φ`, so `(√(r² − x²))′ = ρ·x·sin φ / √(r² − x²)`.
        let root = self.root(cell);
        let ds = self.x(cell).times(cell.sin).scale(self.rho).over(root);
        let (_, along) = self.charge();
        let dt = cell
            .cos
            .scale(self.rho * self.cos)
            .plus(ds.scale(sheet.sign()))
            .scale(self.tilt())
            .widen(along * (T::one() + self.rho / root.lo));
        V3::along(self.n, cell.sin.scale(T::zero() - self.rho))
            .plus(V3::along(self.v, cell.cos.scale(self.rho)))
            .plus(V3::along(self.axis, dt))
    }
}

/// **A component the sweep lists**: open between two cut ends, or a loop
/// the ball holds whole.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Swept<T: Real> {
    /// Its two cut ends, where it is open: the certified roots where it
    /// crosses the ball's boundary, the one it enters by first.
    pub(crate) cut_ends: Option<[Point3<T>; 2]>,
    /// A point of it decided inside the ball.
    pub(crate) witness: Point3<T>,
    /// A loop of one sheet over the whole turn, which winds about the
    /// ruled side.
    pub(crate) encircles: bool,
}

/// **What one sweep read**: the components inside the ball, and the
/// loops the section has before the ball cuts them.
#[derive(Clone, Debug)]
pub(crate) struct Sweep<T: Real> {
    pub(crate) components: Vec<Swept<T>>,
    pub(crate) loops: usize,
}

/// **Why a sweep refused.**
#[derive(Clone, Debug)]
pub(crate) enum SweepRefusal {
    /// A cell at the floor, or the cell that spent the budget, on its
    /// own reading.
    Escalated(Indeterminate),
    /// Two of the sweep's readings disagree about the side of the ball
    /// one stretch of a sheet lies on.
    Inconsistent(&'static str),
}

/// `Δ`'s reading of one cell of the turn.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Disc {
    Positive,
    Negative,
    /// One simple zero, `Δ` rising through it as the angle grows.
    Rise,
    /// One simple zero, `Δ` falling.
    Fall,
}

/// One arc of `Δ > 0` between its two branch cells.
struct Arc<T: Real> {
    rise: Cell<T>,
    fall: Cell<T>,
    smooth: Vec<Cell<T>>,
}

/// A sheet's reading of one cell against the ball.
#[derive(Clone, Copy, Debug)]
enum Leaf<T: Real> {
    Inside(Cell<T>),
    Outside(Cell<T>),
    /// One cut end in the cell: `entering` where the sheet enters the
    /// ball as the angle grows.
    Root {
        cell: Cell<T>,
        entering: bool,
        at: Point3<T>,
    },
}

/// One reading along a loop, in the loop's order.
#[derive(Clone, Copy, Debug)]
enum Event<T: Real> {
    /// The stretch here lies on this side, `true` inside; with a point
    /// of it decided inside, where the side is inside and one was read.
    Side(bool, Option<Point3<T>>),
    /// A cut end, `entering` along the loop's order, with a point decided
    /// inside beside it.
    Cut {
        entering: bool,
        at: Point3<T>,
        inside: Point3<T>,
    },
}

/// **The sweep of `ruled` against `ball`**, the ball the march ends at
/// (module docs).
///
/// # Errors
///
/// [`SweepRefusal`].
pub(crate) fn sweep<T: Decide, R: Ruled<T>>(
    ruled: &R,
    ball: ExtentBall<T>,
    band: Band,
) -> Result<Sweep<T>, SweepRefusal> {
    Sweeper {
        ruled,
        ball,
        band,
        spent: 0,
    }
    .run()
}

struct Sweeper<'a, T: Real, R> {
    ruled: &'a R,
    ball: ExtentBall<T>,
    band: Band,
    spent: usize,
}

impl<T: Decide, R: Ruled<T>> Sweeper<'_, T, R> {
    fn run(&mut self) -> Result<Sweep<T>, SweepRefusal> {
        let mut cells = Vec::new();
        #[allow(clippy::cast_precision_loss)]
        let cut = |k: usize| core::f64::consts::TAU * k as f64 / START as f64;
        for k in 0..START {
            self.discriminant_cell(Cell::new(cut(k), cut(k + 1)), &mut cells)?;
        }
        if cells.iter().all(|(_, d)| *d == Disc::Positive) {
            let turn: Vec<Cell<T>> = cells.into_iter().map(|(c, _)| c).collect();
            let mut components = Vec::new();
            for sheet in [Sheet::Plus, Sheet::Minus] {
                let events = self
                    .leaves(&turn, sheet)?
                    .into_iter()
                    .map(|l| self.event(l, sheet, false))
                    .collect();
                components.extend(cut_loop(events, true)?);
            }
            return Ok(Sweep {
                components,
                loops: 2,
            });
        }
        let arcs = arcs(cells)?;
        let mut components = Vec::new();
        for arc in &arcs {
            components.extend(self.arc_components(arc)?);
        }
        Ok(Sweep {
            components,
            loops: arcs.len(),
        })
    }

    /// Counts a cell against the budget.
    fn spend(&mut self) {
        self.spent += 1;
    }

    /// Whether an unsettled `cell` may not be split again: at the floor,
    /// or past the budget.
    fn spent_on(&self, cell: &Cell<T>) -> bool {
        cell.width() <= FLOOR || self.spent > BUDGET
    }

    /// `Some(true)` where `x` is decided positive, `Some(false)` where it
    /// is decided negative.
    fn side(&self, name: &'static str, x: T) -> Option<bool> {
        match decide(name, Margin::of(x), self.band) {
            Ok(Sign::Positive) => Some(true),
            Ok(Sign::Negative) => Some(false),
            Ok(Sign::Zero) | Err(_) => None,
        }
    }

    fn positive(&self, name: &'static str, m: Margin<T>) -> bool {
        matches!(decide(name, m, self.band), Ok(Sign::Positive))
    }

    /// **The escalation of a cell no test settled**, on the enclosure
    /// `reading`: bounds that straddle the band escalate as one straddle
    /// on the row's own straddle row, bounds both `Zero` as a `Zero` no
    /// side passes.
    fn escalate(&self, name: &'static str, reading: Iv<T>) -> SweepRefusal {
        let bound = |m: T| decide_reported(name, Margin::of(m), self.band);
        SweepRefusal::Escalated(match (bound(reading.lo), bound(reading.hi)) {
            (Err(e), _) | (_, Err(e)) => e,
            (Ok(lower), Ok(upper)) if lower.sign < upper.sign => {
                straddle(straddle_row(name), lower, upper, self.band)
            }
            (Ok(_), Ok(upper)) if upper.sign == Sign::Zero => {
                gate_rejection(name, upper, self.band, SizedPass::NonZero)
            }
            (Ok(_), Ok(upper)) => unreachable!(
                "`{name}` escalated an enclosure decided {:?} at both ends: the sweep \
                 escalates only a cell no test settled",
                upper.sign
            ),
        })
    }

    /// The signed distance of the meetings `x` past the ball's boundary.
    fn past(&self, x: V3<T>) -> Iv<T> {
        x.less(self.ball.center())
            .norm()
            .shift(T::zero() - self.ball.radius())
    }

    /// `Δ`'s reading of `cell`, split until each piece is one of
    /// [`Disc`], the pieces in the turn's order.
    fn discriminant_cell(
        &mut self,
        cell: Cell<T>,
        out: &mut Vec<(Cell<T>, Disc)>,
    ) -> Result<(), SweepRefusal> {
        let m = self.ruled.discriminant(&cell);
        self.spend();
        let kind = if self.positive(ROW_DISCRIMINANT, Margin::of(m.lo)) {
            Some(Disc::Positive)
        } else if self.positive(ROW_DISCRIMINANT, Margin::of(T::zero() - m.hi)) {
            Some(Disc::Negative)
        } else {
            self.branch(&cell)
        };
        if let Some(kind) = kind {
            out.push((cell, kind));
            return Ok(());
        }
        if self.spent_on(&cell) {
            return Err(self.escalate(ROW_DISCRIMINANT, m));
        }
        let (l, r) = cell.halves();
        self.discriminant_cell(l, out)?;
        self.discriminant_cell(r, out)
    }

    /// `Δ` monotone over `cell` with its ends decided on opposite sides:
    /// one simple zero, a branch point.
    fn branch(&self, cell: &Cell<T>) -> Option<Disc> {
        let slope = self.ruled.discriminant_slope(cell);
        let monotone = self.positive(ROW_SLOPE, Margin::of(slope.lo))
            || self.positive(ROW_SLOPE, Margin::of(T::zero() - slope.hi));
        if !monotone {
            return None;
        }
        let end =
            |phi: f64| self.side(ROW_DISCRIMINANT, self.ruled.discriminant(&Cell::at(phi)).lo);
        match (end(cell.lo), end(cell.hi)) {
            (Some(false), Some(true)) => Some(Disc::Rise),
            (Some(true), Some(false)) => Some(Disc::Fall),
            _ => None,
        }
    }

    /// **One arc's components**: its branch cells read sheet-wise, its
    /// two sheets' leaves, and the loop they make cut by the ball.
    fn arc_components(&mut self, arc: &Arc<T>) -> Result<Vec<Swept<T>>, SweepRefusal> {
        let mut smooth = arc.smooth.clone();
        let rise = self.branch_side(arc.rise, true, &mut smooth)?;
        let fall = self.branch_side(arc.fall, false, &mut smooth)?;
        // Along the loop: the first sheet from the rise to the fall, then
        // the second back.
        let mut events = vec![Event::Side(rise, None)];
        events.extend(
            self.leaves(&smooth, Sheet::Plus)?
                .into_iter()
                .map(|l| self.event(l, Sheet::Plus, false)),
        );
        events.push(Event::Side(fall, None));
        let back: Vec<Event<T>> = self
            .leaves(&smooth, Sheet::Minus)?
            .into_iter()
            .rev()
            .map(|l| self.event(l, Sheet::Minus, true))
            .collect();
        events.extend(back);
        cut_loop(events, false)
    }

    /// **A branch point's side of the ball**, read sheet-wise: both
    /// meetings over its cell decided on one side. Otherwise the cell is
    /// halved toward the zero, the half clear of it joining `smooth` at
    /// the arc's `rising` end.
    fn branch_side(
        &mut self,
        mut cell: Cell<T>,
        rising: bool,
        smooth: &mut Vec<Cell<T>>,
    ) -> Result<bool, SweepRefusal> {
        loop {
            let both = self
                .past(self.ruled.meeting(&cell, Sheet::Plus))
                .hull(self.past(self.ruled.meeting(&cell, Sheet::Minus)));
            self.spend();
            // An arc with no cell clear of its branch points has no stretch
            // to witness it on: split until it has one.
            let side = if self.positive(ROW_BRANCH, Margin::of(T::zero() - both.hi)) {
                Some(true)
            } else if self.positive(ROW_BRANCH, Margin::of(both.lo)) {
                Some(false)
            } else {
                None
            };
            if let (Some(inside), false) = (side, smooth.is_empty()) {
                return Ok(inside);
            }
            if self.spent_on(&cell) {
                return Err(match side {
                    None => self.escalate(ROW_BRANCH, both),
                    Some(_) => self.escalate(ROW_DISCRIMINANT, self.ruled.discriminant(&cell)),
                });
            }
            // Cut where `Δ`'s side decides, off the middle where the zero
            // stands within the band of it.
            let cut = [0.5, 0.375, 0.625].into_iter().find_map(|f| {
                let at = cell.lo + f * cell.width();
                let m = self.ruled.discriminant(&Cell::at(at)).lo;
                self.side(ROW_DISCRIMINANT, m)
                    .map(|positive| (at, positive))
            });
            let Some((at, positive)) = cut else {
                // The branch point placed to within the band of the zero:
                // where its side of the ball is still undecided, a
                // boundary within band of it.
                return Err(match side {
                    None => self.escalate(ROW_BRANCH, both),
                    Some(_) => self.escalate(ROW_DISCRIMINANT, self.ruled.discriminant(&cell)),
                });
            };
            let (l, r) = (Cell::new(cell.lo, at), Cell::new(at, cell.hi));
            match (positive, rising) {
                // The zero in the left part: a rise's right part is
                // positive and joins the arc's start.
                (true, true) => {
                    smooth.insert(0, r);
                    cell = l;
                }
                (false, true) => cell = r,
                // A fall's left part is positive and joins its end.
                (true, false) => {
                    smooth.push(l);
                    cell = r;
                }
                (false, false) => cell = l,
            }
        }
    }

    /// **A sheet's leaves over `cells`**, each cell split until each
    /// piece is decided inside, outside, or one cut end.
    fn leaves(&mut self, cells: &[Cell<T>], sheet: Sheet) -> Result<Vec<Leaf<T>>, SweepRefusal> {
        let mut out = Vec::new();
        for &cell in cells {
            self.leaf(cell, sheet, &mut out)?;
        }
        Ok(out)
    }

    fn leaf(
        &mut self,
        cell: Cell<T>,
        sheet: Sheet,
        out: &mut Vec<Leaf<T>>,
    ) -> Result<(), SweepRefusal> {
        let x = self.ruled.meeting(&cell, sheet);
        let h = self.past(x);
        self.spend();
        if self.positive(ROW_CUT, Margin::of(T::zero() - h.hi)) {
            out.push(Leaf::Inside(cell));
            return Ok(());
        }
        if self.positive(ROW_CUT, Margin::of(h.lo)) {
            out.push(Leaf::Outside(cell));
            return Ok(());
        }
        if let Some(leaf) = self.monotone(&cell, sheet, x) {
            out.push(leaf);
            return Ok(());
        }
        if self.spent_on(&cell) {
            return Err(self.escalate(ROW_CUT, h));
        }
        let (l, r) = cell.halves();
        self.leaf(l, sheet, out)?;
        self.leaf(r, sheet, out)
    }

    /// **The sheet monotone against the ball over `cell`**: the distance
    /// from the ball's centre moves one way across it, so its two ends
    /// decide it, inside, outside, or one cut end between them. Read
    /// only where `Δ` is decided positive, so the sheet is smooth.
    fn monotone(&self, cell: &Cell<T>, sheet: Sheet, x: V3<T>) -> Option<Leaf<T>> {
        if !self.positive(
            ROW_DISCRIMINANT,
            Margin::of(self.ruled.discriminant(cell).lo),
        ) {
            return None;
        }
        let dx = self.ruled.velocity(cell, sheet);
        let w = x.less(self.ball.center());
        let along = w.dot(dx);
        let scale = w.norm().hi * dx.norm().hi;
        // `(x − c)·x′` over `|x − c|·|x′|`, the cosine between the sheet's
        // tangent and the ray from the centre, levered by the ruling's.
        let lever = self.ruled.lever();
        let monotone = self.positive(ROW_MONOTONE, Margin::levered(along.lo / scale, lever))
            || self.positive(
                ROW_MONOTONE,
                Margin::levered((T::zero() - along.hi) / scale, lever),
            );
        if !monotone {
            return None;
        }
        let end = |phi: f64| {
            self.side(
                ROW_CUT,
                self.past(self.ruled.meeting(&Cell::at(phi), sheet)).lo,
            )
        };
        match (end(cell.lo), end(cell.hi)) {
            (Some(false), Some(false)) => Some(Leaf::Inside(*cell)),
            (Some(true), Some(true)) => Some(Leaf::Outside(*cell)),
            (Some(outside_first), Some(_)) => Some(Leaf::Root {
                cell: *cell,
                entering: outside_first,
                at: self.settle(cell, sheet, outside_first),
            }),
            _ => None,
        }
    }

    /// **A cut end placed in its cell** by bisection on the ends' sides,
    /// stopping where the midpoint's side is in the band: a selection
    /// within the isolating cell, whose root it narrows to the band.
    fn settle(&self, cell: &Cell<T>, sheet: Sheet, entering: bool) -> Point3<T> {
        let (mut lo, mut hi) = (cell.lo, cell.hi);
        let at = |phi: f64| self.ruled.meeting(&Cell::at(phi), sheet);
        for _ in 0..SETTLE {
            let mid = 0.5 * (lo + hi);
            match self.side(ROW_CUT, self.past(at(mid)).lo) {
                // Outside at `mid`: the root lies on the inside's side.
                Some(true) if entering => lo = mid,
                Some(true) => hi = mid,
                Some(false) if entering => hi = mid,
                Some(false) => lo = mid,
                None => return at(mid).mid(),
            }
        }
        at(0.5 * (lo + hi)).mid()
    }

    /// **A leaf as one reading along its loop**, `backward` where the
    /// loop runs it against the angle. A cut end carries the meeting at
    /// its cell's inside end, decided inside by the leaf.
    fn event(&self, leaf: Leaf<T>, sheet: Sheet, backward: bool) -> Event<T> {
        let point = |phi: f64| self.ruled.meeting(&Cell::at(phi), sheet).mid();
        match leaf {
            Leaf::Inside(cell) => Event::Side(true, Some(point(cell.mid()))),
            Leaf::Outside(_) => Event::Side(false, None),
            Leaf::Root { cell, entering, at } => Event::Cut {
                entering: entering != backward,
                at,
                inside: point(if entering { cell.hi } else { cell.lo }),
            },
        }
    }
}

/// **`Δ`'s cells grouped into its arcs**: a rise, the positive cells
/// after it, a fall. The turn is read from just past a fall, so an arc
/// across the angle's seam is read whole.
fn arcs<T: Real>(cells: Vec<(Cell<T>, Disc)>) -> Result<Vec<Arc<T>>, SweepRefusal> {
    let inconsistent = SweepRefusal::Inconsistent(
        "the ruling sweep's cells of the discriminant do not alternate rise and fall",
    );
    let Some(after) = cells.iter().position(|(_, d)| *d == Disc::Fall) else {
        return if cells.iter().all(|(_, d)| *d == Disc::Negative) {
            Ok(Vec::new())
        } else {
            Err(inconsistent)
        };
    };
    let n = cells.len();
    let mut arcs = Vec::new();
    let mut open: Option<(Cell<T>, Vec<Cell<T>>)> = None;
    for k in 1..=n {
        let (cell, kind) = cells[(after + k) % n];
        open = match (open, kind) {
            (None, Disc::Negative) => None,
            (None, Disc::Rise) => Some((cell, Vec::new())),
            (Some((rise, mut smooth)), Disc::Positive) => {
                smooth.push(cell);
                Some((rise, smooth))
            }
            (Some((rise, smooth)), Disc::Fall) => {
                arcs.push(Arc {
                    rise,
                    fall: cell,
                    smooth,
                });
                None
            }
            _ => return Err(inconsistent),
        };
    }
    if open.is_some() {
        return Err(inconsistent);
    }
    Ok(arcs)
}

/// An open stretch inside the ball, as [`cut_loop`] walks it.
type Stretch<T> = (Point3<T>, Point3<T>, Vec<Point3<T>>);

/// **One loop's readings cut into its components**: each stretch inside
/// the ball between an entering cut end and the next leaving one is an
/// open component; a loop no cut end crosses is one component where it
/// lies inside, none where outside. `encircles` is the loop's winding
/// about the ruled side.
fn cut_loop<T: Real>(
    events: Vec<Event<T>>,
    encircles: bool,
) -> Result<Vec<Swept<T>>, SweepRefusal> {
    let inconsistent =
        SweepRefusal::Inconsistent("the ruling sweep reads one stretch of a sheet on both sides");
    let Some(start) = events.iter().position(|e| {
        matches!(
            e,
            Event::Cut {
                entering: false,
                ..
            }
        )
    }) else {
        if events.iter().any(|e| matches!(e, Event::Cut { .. })) {
            return Err(inconsistent);
        }
        let sides: Vec<(bool, Option<Point3<T>>)> = events
            .iter()
            .filter_map(|e| match *e {
                Event::Side(inside, at) => Some((inside, at)),
                Event::Cut { .. } => None,
            })
            .collect();
        let inside = sides.first().is_some_and(|s| s.0);
        if sides.iter().any(|s| s.0 != inside) {
            return Err(inconsistent);
        }
        if !inside {
            return Ok(Vec::new());
        }
        let points: Vec<Point3<T>> = sides.iter().filter_map(|s| s.1).collect();
        let witness = *points
            .get(points.len() / 2)
            .ok_or(SweepRefusal::Inconsistent(
                "the ruling sweep read a loop inside the ball at no point",
            ))?;
        return Ok(vec![Swept {
            cut_ends: None,
            witness,
            encircles,
        }]);
    };
    // From just past a leaving cut end: outside, until the next entering.
    let n = events.len();
    let mut out = Vec::new();
    // The open stretch: its entering cut end, the point beside it, and
    // the points read inside along it.
    let mut run: Option<Stretch<T>> = None;
    for k in 1..=n {
        run = match (run, events[(start + k) % n]) {
            (None, Event::Side(false, _)) => None,
            (
                None,
                Event::Cut {
                    entering: true,
                    at,
                    inside,
                },
            ) => Some((at, inside, Vec::new())),
            (Some((from, inside, mut points)), Event::Side(true, at)) => {
                points.extend(at);
                Some((from, inside, points))
            }
            (
                Some((from, inside, points)),
                Event::Cut {
                    entering: false,
                    at,
                    ..
                },
            ) => {
                out.push(Swept {
                    cut_ends: Some([from, at]),
                    witness: points.get(points.len() / 2).copied().unwrap_or(inside),
                    encircles: false,
                });
                None
            }
            _ => return Err(inconsistent),
        };
    }
    if run.is_some() {
        return Err(inconsistent);
    }
    Ok(out)
}

/// **What a coverage read**: the cells, and the deepest halving of the
/// first cut's.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Coverage {
    pub(crate) cells: usize,
    pub(crate) depth: u32,
}

/// The first cut's cells of the turn, for [`coverage`].
const COVERAGE_START: i32 = 64;
/// The halvings of the first cut's cells [`coverage`] takes before it
/// gives up on a cell.
pub(crate) const COVERAGE_DEPTH: u32 = 30;
/// The cells [`coverage`] reads before it gives up.
pub(crate) const COVERAGE_CELLS: usize = 1 << 18;

/// **The ruling angle's coverage of a cylinder pair**, a second reading
/// of the count the sweep closed exhaustiveness on (C3): every cell of
/// `ruled`'s ruling angle has its rulings' meetings with `partner` outside
/// `ball` (the ball the march ends at), or inside one of `tubes` (the
/// found branches' tube boxes). A component the count missed has
/// meetings in no tube. It is read on certified enclosures at `Interval`
/// and from the operands alone, by the general quadratic `At² + 2Bt + C`
/// of a ruling `p + t·a` against the partner's axis `(o′, a′)`, with
/// `w = p − o′`: `A = |a × a′|²`, `B = (w × a′)·(a × a′)`,
/// `C = |w × a′|² − r²`, roots `(−B ± √(B² − AC))/A`. Neither the
/// sweep's frame nor its closed form enters it, so a defect in either
/// does not hide itself here. Either wall may be ruled: every section
/// point lies on exactly one of its rulings.
///
/// # Errors
///
/// The first cell it could not cover within [`COVERAGE_DEPTH`] halvings
/// and [`COVERAGE_CELLS`] cells, `[lo, hi]`.
pub(crate) fn coverage(
    ruled: &geom::Surface<f64>,
    partner: &geom::Surface<f64>,
    ball: ExtentBall<f64>,
    tubes: &[crate::ssi::enclose::Box3],
) -> Result<Coverage, [f64; 2]> {
    use crate::ssi::enclose::Box3;
    use geom_core::{Bounds as _, Interval, SpanLocate as _};
    let (
        &geom::Surface::Cylinder {
            origin: o,
            axis: a,
            radius: rho,
            ..
        },
        &geom::Surface::Cylinder {
            origin: q,
            axis: b,
            radius: r,
            ..
        },
    ) = (ruled, partner)
    else {
        return Err([0.0, core::f64::consts::TAU]);
    };
    let lift = Interval::from_f64;
    let p3 = |p: Point3<f64>| Point3::new(lift(p.x), lift(p.y), lift(p.z));
    let v3 = |v: Vec3<f64>| Vec3::new(lift(v.x), lift(v.y), lift(v.z));
    let (u, v) = a.orthonormal_basis();
    let (o, q, a, b, u, v) = (p3(o), p3(q), v3(a), v3(b), v3(u), v3(v));
    let (rho, r) = (lift(rho), lift(r));
    let ab = a.cross(b);
    let big_a = ab.dot(ab);
    // `w × a′` for the enclosure `w`.
    let cross = |w: V3<Interval>| V3 {
        x: w.y.scale(b.z).plus(w.z.scale(Interval::zero() - b.y)),
        y: w.z.scale(b.x).plus(w.x.scale(Interval::zero() - b.z)),
        z: w.x.scale(b.y).plus(w.y.scale(Interval::zero() - b.x)),
    };
    let along = |k: Vec3<Interval>| V3::along(k, Iv::point(Interval::one()));
    let hull = |i: Iv<Interval>| i.lo.enclosure_hull(i.hi);
    let (centre, radius) = (ball.center(), ball.radius());
    let clear = |bx: Box3| {
        let gap = |i: Interval, c: f64| {
            if !i.is_certified() {
                return f64::NAN;
            }
            (i.lo() - c).max(c - i.hi()).max(0.0)
        };
        let (gx, gy, gz) = (
            gap(bx.x, centre.x),
            gap(bx.y, centre.y),
            gap(bx.z, centre.z),
        );
        (gx * gx + gy * gy + gz * gz).sqrt() * (1.0 - 1e-12) > radius * (1.0 + 1e-12)
    };
    let mut hint = 0usize;
    let mut read = Coverage { cells: 0, depth: 0 };
    #[allow(clippy::cast_precision_loss)]
    let cut = |k: i32| core::f64::consts::TAU * f64::from(k) / f64::from(COVERAGE_START);
    let mut stack: Vec<(f64, f64, u32)> = (0..COVERAGE_START)
        .map(|k| (cut(k), cut(k + 1), 0))
        .collect();
    while let Some((lo, hi, depth)) = stack.pop() {
        read.cells += 1;
        read.depth = read.depth.max(depth);
        let cell = Cell::<Interval>::new(lo, hi);
        let p = V3::at(o)
            .plus(V3::along(u, cell.cos.scale(rho)))
            .plus(V3::along(v, cell.sin.scale(rho)));
        let wb = cross(p.less(q));
        let qb = wb.dot(along(ab));
        let qc = wb.dot(wb).shift(Interval::zero() - r * r);
        let disc = qb.square().plus(qc.scale(Interval::zero() - big_a));
        let covered = hull(disc).is_certified() && hull(disc).hi() < 0.0
            || [Interval::one(), Interval::zero() - Interval::one()]
                .into_iter()
                .all(|sign| {
                    let t = disc
                        .sqrt()
                        .scale(sign)
                        .plus(qb.scale(Interval::zero() - Interval::one()))
                        .scale(Interval::one() / big_a);
                    let x = p.plus(V3::along(a, t));
                    let bx = Box3 {
                        x: hull(x.x),
                        y: hull(x.y),
                        z: hull(x.z),
                    };
                    clear(bx)
                        || tubes.get(hint).is_some_and(|t| bx.contained_in(*t))
                        || tubes
                            .iter()
                            .position(|t| bx.contained_in(*t))
                            .is_some_and(|k| {
                                hint = k;
                                true
                            })
                });
        if covered {
            continue;
        }
        if depth >= COVERAGE_DEPTH || read.cells >= COVERAGE_CELLS {
            return Err([lo, hi]);
        }
        let mid = 0.5 * (lo + hi);
        stack.push((mid, hi, depth + 1));
        stack.push((lo, mid, depth + 1));
    }
    Ok(read)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::{Cell, Ruled, RuledCylinders};
    use crate::pair_section::{CcStance, CylinderCylinderPose};
    use crate::{ExtentBall, Reach};
    use geom::Surface;
    use geom_core::{Band, Point3, Tol, Vec3};

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    fn wall(origin: (f64, f64, f64), axis: Vec3<f64>, radius: f64) -> Surface<f64> {
        let axis = axis.normalize();
        Surface::Cylinder {
            origin: Point3::new(origin.0, origin.1, origin.2),
            axis,
            radius,
            u_ref: axis.orthonormal_basis().0,
        }
    }

    fn ruled(a: &Surface<f64>, b: &Surface<f64>) -> (RuledCylinders<f64>, f64) {
        let region = ExtentBall::new(Point3::origin(), 3.0);
        let pose = CylinderCylinderPose::decide(a, b, &Reach::Ball(region), band())
            .expect("two cylinders")
            .expect("a decided pose");
        let CcStance::Crossing { gap, .. } = pose.stance() else {
            panic!("crossing axes");
        };
        (pose.ruled().expect("a ruled side"), gap.abs())
    }

    /// **`Δ`'s extremes over the turn are the pose's margins**: the
    /// discriminant's margin read at every angle of a fine turn (its two
    /// extremes, at `0` and `π`, among them) reaches `r₁ + r₂ − e` at its
    /// most (the gap level, `e` at least the ruled radius) and
    /// `|r₁ − r₂| − e` at its least (the nest level), each the pose's
    /// closed-form margin negated, for walls that cross in one loop, in
    /// two, and clear each other, at a right angle and skew, in both
    /// orders.
    #[test]
    fn the_discriminants_extremes_are_the_poses_margins() {
        let z = Vec3::unit_z();
        let skew = Vec3::new(1.0, 0.4, 0.3);
        let poses = [
            (
                "one loop",
                wall((0.0, 0.4, 0.0), Vec3::unit_x(), 0.2),
                0.5,
                0.2,
            ),
            (
                "two loops",
                wall((0.0, 0.1, 0.0), Vec3::unit_x(), 0.2),
                0.5,
                0.2,
            ),
            (
                "clear",
                wall((0.0, 0.9, 0.0), Vec3::unit_x(), 0.2),
                0.5,
                0.2,
            ),
            (
                "skew, one loop",
                wall((0.1, 0.45, -0.2), skew, 0.3),
                0.5,
                0.3,
            ),
            (
                "skew, two loops",
                wall((0.0, 0.05, 0.1), skew, 0.25),
                0.5,
                0.25,
            ),
        ];
        for (what, b, r1, r2) in poses {
            let a = wall((0.0, 0.0, 0.0), z, r1);
            for (order, x, y) in [("a, b", &a, &b), ("b, a", &b, &a)] {
                let (ruled, e) = ruled(x, y);
                let (mut most, mut least) = (f64::NEG_INFINITY, f64::INFINITY);
                for k in 0..=4096 {
                    let m = ruled
                        .discriminant(&Cell::at(core::f64::consts::TAU * f64::from(k) / 4096.0));
                    most = most.max(m.hi);
                    least = least.min(m.lo);
                }
                let label = format!("{what} ({order})");
                assert!(
                    (least - ((r1 - r2).abs() - e)).abs() <= 1e-12,
                    "{label}: the least is the nest level, {least} against {}",
                    (r1 - r2).abs() - e
                );
                if e >= r1.min(r2) {
                    assert!(
                        (most - (r1 + r2 - e)).abs() <= 1e-12,
                        "{label}: the most is the gap level, {most} against {}",
                        r1 + r2 - e
                    );
                } else {
                    assert!(most > 0.0, "{label}: the walls cross");
                }
            }
        }
    }
}
