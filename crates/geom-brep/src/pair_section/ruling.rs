//! **The ruling sweep** (C3, C5): a pair with a ruled side read along that
//! side's rulings. Each ruling is a line, and it meets the partner where a
//! quadratic in the line's parameter vanishes, in closed form: nowhere
//! where the discriminant `Δ` is negative, twice where it is positive (two
//! sheets of meetings), once where it vanishes. A simple zero of `Δ` is a
//! branch point, where the two sheets join; a double one is a tangency,
//! which a cylinder's pose decides before any sweep and a cone's sweep
//! decides itself ([`Touch`]).
//!
//! The ruling angle `φ` is cut into cells, each read on enclosures of `Δ`,
//! of the two meetings and of their velocity over the cell ([`Ruled`]).
//! The sweep then:
//!
//! - **counts the components**: an arc of `φ` where `Δ > 0`, bounded by
//!   two branch points, is one loop (its two sheets joined at both), and
//!   `Δ > 0` over the whole turn is two loops, one per sheet; a crossing
//!   tangency is a node every walk through it passes, the loops cut into
//!   arcs that end there;
//! - **cuts them by the domain**: a cut end is a certified root where a
//!   sheet crosses the boundary of the ball the march ends at, in a cell
//!   where that sheet is monotone against the ball and its two ends lie
//!   on opposite sides; the stretches between cut ends that lie inside
//!   the ball are the open components, and a loop no root cuts is one
//!   component inside the ball or none;
//! - **witnesses** each component it lists at a point decided inside the
//!   ball, the march's seed. A loop the ball holds whole is witnessed
//!   at its middle, halfway along its arc from either branch point, and
//!   a loop of the whole turn at the first cut, `φ = 0`, where the
//!   cylinder × sphere frame points at the centre and `Δ` peaks. The
//!   march starts and ends its trace at the witness, so the fit's two
//!   ends stand clear of the branch points and the far ruling, where a
//!   loop near a node turns sharpest.
//!
//! A branch point is read sheet-wise: its cell encloses both meetings
//! through the discriminant's root over `[0, √Δ_max]`, which decides the
//! ball's side for both sheets at once. Where it cannot, the cell is
//! halved toward the branch point, the half clear of it joining the arc;
//! a branch cell at the floor escalates, a boundary within band of a
//! branch point.
//!
//! # The partner's winding
//!
//! A sweep handed the partner's frame ([`sweep_about`], [`sweep_divided`])
//! also reads whether each loop the ball holds whole winds about the
//! partner's axis: a cylinder's, or a cone's through its apex. The loop is
//! walked as pieces in its own order (one sheet's cells over the turn;
//! or an arc's rise, the plus sheet to its fall and the minus sheet back,
//! a branch cell read on both sheets at once), each piece's meetings in
//! one box ([`Ruled::meeting`]). Neighbouring pieces' boxes both hold the
//! loop's point where they meet, so a joint point is read in their meet;
//! a branch cell's box holds both sheets either side of the branch point.
//!
//! The lever is `winds.rs`'s (its module docs, "The lever"), read here
//! on the sweep's own cells rather than a march's tube boxes: each
//! piece's box, with its two joints, is decided clear of the axis in the
//! frame's plane. Its rectangle there is convex and misses the axis, so
//! the stretch of the loop it holds and the chord between its joints
//! turn about the axis alike, by less than half a turn: the chord's turn
//! is the principal difference of its ends' azimuths, trusted within
//! `winds.rs`'s `TRUSTED_TURN`. The loop is homotopic, off the axis, to
//! the polygon through its joints, so it winds as that polygon's turns
//! sum, a whole number of turns decided past half a turn or short of it.
//! A piece that does not settle is split, a branch cell where `Δ`
//! decides. On a cylinder the loop stands its radius off the axis; on a
//! cone off the apex, so the pieces settle once they are small beside
//! that. **A read that still does not settle** — a piece at the floor or
//! past the budget, a branch cell `Δ` decides at no cut, a sum in band of
//! half a turn — claims nothing: the loop's partner winding is unclaimed,
//! which every reader takes as weaker, never wrong, and the sweep records
//! the margin that failed (`Sweep::unclaimed`). It never refuses the
//! section.
//!
//! Every cell is decided at the band through `k_stats` (D4). A cell no
//! test settles is split, down to a floor; a cell at the floor, or the
//! cell that spends the budget, escalates on its own reading.

use geom_core::bisect::{Budget, Exhausted, Step, bisect_within};
use geom_core::k_stats::{decide, decide_reported, gate_rejection, straddle};
use geom_core::{
    Band, Decide, Indeterminate, Margin, Point3, Real, Sign, SizedPass, Vec3, rounding_charge,
};

use crate::ExtentBall;

/// The cells the first cut makes of the turn: an eighth of a radian
/// each, fine enough that a branch point's cell is cut in a few halvings
/// on the poses measured, coarse enough to cost nothing beside them.
const START: usize = 32;
/// The most cells one sweep reads before it escalates: the deepest
/// sweep of the cylinder rows read under a thousand, so the budget is
/// thirty-fold headroom and bounds an in-band pose's cost.
const BUDGET: usize = 1 << 15;
/// The narrowest cell the sweep splits, in radians: `2π / 2⁴⁰`, where a
/// cell's ends still differ by thousands of ulps of the angle, so its
/// cosine and sine are read on two distinct points.
#[allow(clippy::cast_precision_loss)]
const FLOOR: f64 = core::f64::consts::TAU / (1u64 << 40) as f64;
/// The bisections that place a cut end in its isolating cell: 2⁻⁴⁸ of
/// the cell, below the band at any lever the sweep reads before the
/// midpoint's side stops deciding, which ends it sooner.
const SETTLE: u32 = 48;
/// The hair a turning point of the cosine or sine is held within a cell
/// by, in radians: past the rounding of `π` and of a cell's ends, so a
/// turning point on a cut is held by both its cells.
const TURN_HAIR: f64 = 1e-12;

/// **The cuts tried near `at` in a cell of `width`**, in order: `at`, then
/// an eighth and a quarter of the width either side. A cut whose reading
/// stands within the band of a zero holds the cells it ends undecided at
/// any width; one of these stands off it wherever the zero is simple.
/// The one ladder every cut the sweep places takes.
fn off_a_zero(at: f64, width: f64) -> impl Iterator<Item = f64> {
    [0.0, 0.125, -0.125, 0.25, -0.25]
        .into_iter()
        .map(move |f| at + f * width)
}

/// **The rows the sweep decides on.**
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Row {
    /// `Δ`'s sign, as a margin in metres.
    Discriminant,
    /// `Δ`'s slope's sign.
    Slope,
    /// A meeting's side of the ball the march ends at.
    Cut,
    /// A sheet's monotonicity against that ball over a cell.
    Monotone,
    /// Both meetings' side of the ball over a branch point's cell.
    Branch,
    /// A piece of a loop clear of the partner's axis, and its turn about
    /// it trusted or the loop's whole turns decided (`Sweeper::winds_about`).
    Winds,
}

impl Row {
    const fn name(self) -> &'static str {
        match self {
            Self::Discriminant => "ruling_discriminant",
            Self::Slope => "ruling_discriminant_slope",
            Self::Cut => "ruling_cut",
            Self::Monotone => "ruling_cut_monotone",
            Self::Branch => "ruling_branch_cut",
            Self::Winds => "ruling_partner_winding",
        }
    }

    /// The row's straddle row: a floor cell's two bounds straddling the
    /// band.
    const fn straddle(self) -> &'static str {
        match self {
            Self::Discriminant => "ruling_discriminant_straddle",
            Self::Slope => "ruling_discriminant_slope_straddle",
            Self::Cut => "ruling_cut_straddle",
            Self::Monotone => "ruling_cut_monotone_straddle",
            Self::Branch => "ruling_branch_cut_straddle",
            // A partner winding that does not settle claims nothing, so
            // it never escalates (`Sweeper::read_windings`).
            Self::Winds => unreachable!(),
        }
    }
}

/// **Which side of zero a reading is decided on**, its whole enclosure
/// past the band.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Reads {
    /// Its lower end decided positive.
    Above,
    /// Its upper end decided negative.
    Below,
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
            lo: a.lo.powi(2),
            hi: a.hi.powi(2),
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

    /// `−self`.
    fn neg(self) -> Self {
        self.from(T::zero())
    }

    /// The part both enclose, each an enclosure of one value; their hull
    /// where rounding parts them.
    fn meet(self, o: Self) -> Self {
        let met = Self {
            lo: self.lo.max(o.lo),
            hi: self.hi.min(o.hi),
        };
        let gap = met.lo - met.hi;
        Self {
            lo: gap.select_le_zero(met.lo, self.lo.min(o.lo)),
            hi: gap.select_le_zero(met.hi, self.hi.max(o.hi)),
        }
    }

    /// `self / o` where `o` stands clear of zero on either side, and
    /// every value ([`WIDE`]) where it does not: a selection on the
    /// enclosure's ends, not a decision.
    fn quotient(self, o: Self) -> Self {
        let wide = T::from_f64(WIDE);
        let above = self.over(o);
        let below = self.neg().over(o.neg());
        // `o.lo > 0`: above; `o.hi < 0`: below; else neither.
        let pick = |a: T, b: T, w: T| {
            o.lo.select_le_zero((T::zero() - o.hi).select_le_zero(w, b), a)
        };
        Self {
            lo: pick(above.lo, below.lo, T::zero() - wide),
            hi: pick(above.hi, below.hi, wide),
        }
    }

    /// The larger of the two ends' sizes: a ceiling on `|x|` over the
    /// enclosure.
    fn size(self) -> T {
        self.abs().hi
    }
}

/// The size [`Iv::quotient`] stands a quotient at whose divisor reaches
/// zero: every finite value lies inside it.
const WIDE: f64 = f64::MAX;

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

    /// The constant vector `v`.
    fn of(v: Vec3<T>) -> Self {
        Self {
            x: Iv::point(v.x),
            y: Iv::point(v.y),
            z: Iv::point(v.z),
        }
    }

    /// `self · k`, `k` an enclosure.
    fn times(self, k: Iv<T>) -> Self {
        Self {
            x: self.x.times(k),
            y: self.y.times(k),
            z: self.z.times(k),
        }
    }

    /// `self · v`, `v` constant.
    fn toward(self, v: Vec3<T>) -> Iv<T> {
        self.x
            .scale(v.x)
            .plus(self.y.scale(v.y))
            .plus(self.z.scale(v.z))
    }

    /// `self × v`, `v` constant.
    fn cross(self, v: Vec3<T>) -> Self {
        Self {
            x: self.y.scale(v.z).plus(self.z.scale(T::zero() - v.y)),
            y: self.z.scale(v.x).plus(self.x.scale(T::zero() - v.z)),
            z: self.x.scale(v.y).plus(self.y.scale(T::zero() - v.x)),
        }
    }

    /// The box holding both.
    fn hull(self, o: Self) -> Self {
        Self {
            x: self.x.hull(o.x),
            y: self.y.hull(o.y),
            z: self.z.hull(o.z),
        }
    }

    /// The part both enclose, axis by axis ([`Iv::meet`]).
    fn meet(self, o: Self) -> Self {
        Self {
            x: self.x.meet(o.x),
            y: self.y.meet(o.y),
            z: self.z.meet(o.z),
        }
    }

    /// The enclosure's centre: a selection, not a bound.
    pub(crate) fn mid(self) -> Point3<T> {
        let half = T::from_f64(0.5);
        let mid = |i: Iv<T>| (i.lo + i.hi) * half;
        Point3::new(mid(self.x), mid(self.y), mid(self.z))
    }
}

/// **A cell of the ruling angle**, `[lo, hi]` radians, with enclosures
/// of its cosine and sine. The sweep's turn starts at a first cut a
/// quarter of a first cell or less off zero ([`off_a_zero`]), so its
/// cells lie within `[−π/64, 2π + π/64]`, each turning point of the
/// cosine and sine in it one of `0, π/2, π, 3π/2, 2π`.
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
        let (sa, ca) = T::from_computed(lo).sin_cos();
        let (sb, cb) = T::from_computed(hi).sin_cos();
        let (mut cos, mut sin) = (Iv::of(ca, cb), Iv::of(sa, sb));
        let holds = |e: f64| lo - e <= TURN_HAIR && hi - e >= -TURN_HAIR;
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
        geom::mid_param(self.lo, self.hi)
    }

    fn width(&self) -> f64 {
        self.hi - self.lo
    }
}

/// **The two sheets of meetings**: the quadratic's roots by the sign `σ`
/// of `√Δ` in them (`Plus` the `+`), joined at a branch point. For a
/// cylinder's rulings, whose leading coefficient is positive, `Plus` is
/// the larger; a cone's projective roots keep `σ` and not the order.
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
/// angle: the shape a ruled pair hands the sweep, built for a cylinder
/// against a cylinder or a sphere ([`RuledCylinders`],
/// [`RuledCylinderSphere`]) and a cone against a cylinder or a cone
/// ([`RuledCone`], [`DividedCone`]).
pub(crate) trait Ruled<T: Real> {
    /// The ruled wall's radius of curvature: the lever a cosine read
    /// against the ruling's turn is metered at.
    fn lever(&self) -> geom_core::Lever<T>;
    /// `Δ` over the cell as a margin in metres, of `Δ`'s sign.
    fn discriminant(&self, cell: &Cell<T>) -> Iv<T>;
    /// `Δ′` over the cell, of its sign, in metres per radian.
    fn discriminant_slope(&self, cell: &Cell<T>) -> Iv<T>;
    /// The meetings of the cell's rulings on `sheet`, the discriminant's
    /// root enclosed through zero where `Δ` straddles it.
    fn meeting(&self, cell: &Cell<T>, sheet: Sheet) -> V3<T>;
    /// Their velocity along the angle, over a cell where [`Self::smooth`]
    /// is decided positive.
    fn velocity(&self, cell: &Cell<T>, sheet: Sheet) -> V3<T>;
    /// **A floor on the distance of the cell's meetings on `sheet` from
    /// `center`**, for a side whose rulings may run along an asymptotic
    /// direction of the partner, where a meeting runs past every bound
    /// and [`Self::meeting`] encloses it by every value: `None` where the
    /// meetings are always bounded.
    fn far(&self, _cell: &Cell<T>, _sheet: Sheet, _center: Point3<T>) -> Option<T> {
        None
    }
    /// The margin whose positive reading makes both sheets smooth over
    /// the cell, so that [`Self::velocity`] reads: `Δ` itself, unless the
    /// side says otherwise.
    fn smooth(&self, cell: &Cell<T>) -> Iv<T> {
        self.discriminant(cell)
    }
    /// **A ceiling on the size of the rulings' clearance over the cell**
    /// (`Δ` is a margin of its sign, and may be only a floor on its
    /// size), where the side reads one: `None` for a side whose pose
    /// decides its tangencies before any sweep, so that the sweep never
    /// decides a double root of `Δ` itself.
    fn ceiling(&self, _cell: &Cell<T>) -> Option<T> {
        None
    }
}

/// **The thinner of two cylinders swept against the other.** The ruled
/// wall is `foot + ρ(cos φ·n + sin φ·v) + t·axis` in the pose's `frame`
/// (`foot` its origin, `n` its `u_ref`, square to both axes, and `v` its
/// `v_ref`, `axis × n`); the partner's axis runs through `foot + gap·n`
/// along `axis′`, with `cos = axis·axis′` and `beta = axis′·v`
/// (`cos² + beta² = 1`, `beta ≠ 0` for crossing axes). A ruling meets the
/// partner where `β²t² − 2β·ρ sin φ cos·t + C = 0`, whose discriminant is
/// `β²(r² − x²)`, `x = ρ cos φ − gap`, and whose roots are
/// `t = (ρ sin φ cos ± √(r² − x²))/β`. Its extremes over the turn are the
/// pose's margins: `r + ρ − e` (the gap level, where `e ≥ ρ`) and
/// `r − ρ − e` (the nest level).
#[derive(Clone, Copy, Debug)]
pub(crate) struct RuledCylinders<T: Real> {
    /// The ruled wall's axis through the common perpendicular's foot on
    /// it, `u_ref` that perpendicular's unit direction.
    pub(crate) frame: geom::Frame<T>,
    /// The ruled wall's radius.
    pub(crate) rho: T,
    /// The partner's.
    pub(crate) r: T,
    pub(crate) gap: T,
    pub(crate) cos: T,
    pub(crate) beta: T,
    /// The ruled wall's curvature arm: the thinner wall's, the lesser
    /// of the two.
    pub(crate) lever: geom_core::Lever<T>,
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
            .from(self.r.powi(2))
            .widen(rounding_charge(self.r.powi(2) + reach.powi(2)))
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
        let size =
            (self.frame.origin() - Point3::origin()).norm() + self.rho + self.r + self.gap.abs();
        let t = rounding_charge(size + self.rho + self.r) * self.tilt().abs();
        (rounding_charge(size), t)
    }
}

impl<T: Real> Ruled<T> for RuledCylinders<T> {
    fn lever(&self) -> geom_core::Lever<T> {
        self.lever
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
        let f = &self.frame;
        V3::at(f.origin())
            .plus(V3::along(f.u_ref(), cell.cos.scale(self.rho)))
            .plus(V3::along(f.v_ref(), cell.sin.scale(self.rho)))
            .plus(V3::along(f.axis(), t))
            .widen(point)
    }

    fn velocity(&self, cell: &Cell<T>, sheet: Sheet) -> V3<T> {
        // `x′ = −ρ sin φ`, so `(√(r² − x²))′ = ρ·x·sin φ / √(r² − x²)`.
        let root = self.root(cell);
        let ds = self.x(cell).times(cell.sin).scale(self.rho).over(root);
        // `t′`'s rounding: the root's charge `along`, and `ds`'s quotient by
        // the root, which levers the numerator's rounding by `ρ/√(r² − x²)`
        // (`|x·sin φ| ≤ ρ + |gap|`, already in `along`'s size).
        let (_, along) = self.charge();
        let dt = cell
            .cos
            .scale(self.rho * self.cos)
            .plus(ds.scale(sheet.sign()))
            .scale(self.tilt())
            .widen(along * (T::one() + self.rho / root.lo));
        let f = &self.frame;
        V3::along(f.u_ref(), cell.sin.scale(T::zero() - self.rho))
            .plus(V3::along(f.v_ref(), cell.cos.scale(self.rho)))
            .plus(V3::along(f.axis(), dt))
    }
}

/// **A cylinder's rulings against a sphere.** The ruled wall is
/// `foot + r(cos φ·n + sin φ·v) + t·axis`, `foot` the sphere's centre's
/// foot on the axis, `n` the unit direction from the axis toward the
/// centre and `v = axis × n`. With `w` from the centre, `w·axis = t`, so a
/// ruling meets the sphere where `t² − (R² − ρ²) = 0`, `ρ` the ruling's
/// distance from the centre, `ρ² = (r − d)² + 2rd(1 − cos φ)` at offset
/// `d`: `Δ` is `R − ρ` in metres, and the roots are `t = ±√(R² − ρ²)`.
/// Its extremes over the turn are the pose's margins: `R − |d − r|` at
/// `φ = 0` (the near reach) and `R − (d + r)` at `φ = π` (the far
/// reach).
#[derive(Clone, Copy, Debug)]
pub(crate) struct RuledCylinderSphere<T: Real> {
    pub(crate) foot: Point3<T>,
    pub(crate) n: Vec3<T>,
    pub(crate) v: Vec3<T>,
    pub(crate) axis: Vec3<T>,
    /// The cylinder's radius.
    pub(crate) r: T,
    /// The centre's distance from the axis.
    pub(crate) d: T,
    /// The sphere's radius.
    pub(crate) big_r: T,
    /// The cylinder's curvature arm.
    pub(crate) lever: geom_core::Lever<T>,
}

impl<T: Real> RuledCylinderSphere<T> {
    /// `1 − cos φ` over the cell, read where it keeps its relative
    /// precision: as `sin²φ / (1 + cos φ)` on a cell inside the half turn
    /// about `φ = 0`, where `1 − cos φ` would cancel, else directly. A
    /// selection on the cell's angles, not a decision.
    fn versine(cell: &Cell<T>) -> Iv<T> {
        use core::f64::consts::{FRAC_PI_2, PI};
        if cell.hi < FRAC_PI_2 || cell.lo > PI + FRAC_PI_2 {
            cell.sin.square().over(cell.cos.shift(T::one()))
        } else {
            cell.cos.from(T::one())
        }
    }

    /// `ρ² = (r − d)² + 2rd(1 − cos φ)` over the cell, both terms posed
    /// to their own relative precision, so `ρ` stays well-conditioned
    /// where it is least.
    fn reach_sq(&self, cell: &Cell<T>) -> Iv<T> {
        let (r, d) = (self.r, self.d);
        Self::versine(cell)
            .scale(T::from_f64(2.0) * r * d)
            .shift((r - d).powi(2))
    }

    /// `√(R² − ρ²)` over the cell, through zero where it straddles it.
    fn root(&self, cell: &Cell<T>) -> Iv<T> {
        self.reach_sq(cell).from(self.big_r.powi(2)).sqrt()
    }
}

impl<T: Real> Ruled<T> for RuledCylinderSphere<T> {
    fn lever(&self) -> geom_core::Lever<T> {
        self.lever
    }

    fn discriminant(&self, cell: &Cell<T>) -> Iv<T> {
        self.reach_sq(cell).sqrt().from(self.big_r)
    }

    /// `−rd·sin φ / R`: `Δ′ = −rd·sin φ / ρ` scaled by `ρ/R > 0`, so of
    /// its sign, and equal to it at a branch point, where `ρ = R`. It
    /// does not divide by `ρ`, which vanishes on a centre on the wall.
    fn discriminant_slope(&self, cell: &Cell<T>) -> Iv<T> {
        cell.sin.scale(T::zero() - self.r * self.d / self.big_r)
    }

    fn meeting(&self, cell: &Cell<T>, sheet: Sheet) -> V3<T> {
        V3::at(self.foot)
            .plus(V3::along(self.n, cell.cos.scale(self.r)))
            .plus(V3::along(self.v, cell.sin.scale(self.r)))
            .plus(V3::along(self.axis, self.root(cell).scale(sheet.sign())))
    }

    fn velocity(&self, cell: &Cell<T>, sheet: Sheet) -> V3<T> {
        // `t′ = ∓ρρ′/√(R² − ρ²)`, `ρρ′ = rd·sin φ`, read only where `Δ` is
        // decided positive, so the root is away from zero.
        let (r, d) = (self.r, self.d);
        let dt = cell
            .sin
            .scale(T::zero() - r * d * sheet.sign::<T>())
            .over(self.root(cell));
        V3::along(self.n, cell.sin.scale(T::zero() - r))
            .plus(V3::along(self.v, cell.cos.scale(r)))
            .plus(V3::along(self.axis, dt))
    }
}

/// **The partner of a cone's rulings as a quadric form**
/// `q(X) = κ|w|² + β(d·w)² + γ`, `w = X − P`, `d` unit: a cylinder of
/// radius `r` about the axis through `P` (`κ = 1`, `β = −1`, `γ = −r²`),
/// or the double cone of half-angle `α` from the apex `P` (`κ = −cos²α`,
/// `β = 1`, `γ = 0`). The form's matrix `κI + βddᵀ` has norm at most one
/// in both, which the clearance ceiling reads.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Form<T: Real> {
    pub(crate) at: Point3<T>,
    pub(crate) d: Vec3<T>,
    kappa: T,
    beta: T,
    gamma: T,
    /// The cylinder's radius, whose clearance is read exactly.
    radius: Option<T>,
    /// The partner cone's `cos α`.
    cos: Option<T>,
    /// How far `P` may stand off the point it names, in metres: its own
    /// rounding where it was computed (a foot on the axis).
    slack: T,
}

impl<T: Real> Form<T> {
    /// The wall of radius `r` about `axis`, its direction unit.
    pub(crate) fn cylinder(axis: geom::Line<T>, r: T) -> Self {
        Self {
            at: axis.origin,
            d: axis.dir.get(),
            kappa: T::one(),
            beta: T::zero() - T::one(),
            gamma: T::zero() - r.powi(2),
            radius: Some(r),
            cos: None,
            slack: T::zero(),
        }
    }

    /// The double cone from the apex `axis.origin` about its unit
    /// direction, of half-angle cosine `cos`.
    pub(crate) fn cone(axis: geom::Line<T>, cos: T) -> Self {
        Self {
            at: axis.origin,
            d: axis.dir.get(),
            kappa: T::zero() - cos.powi(2),
            beta: T::one(),
            gamma: T::zero(),
            radius: None,
            cos: Some(cos),
            slack: T::zero(),
        }
    }

    /// The form with `P` read within `slack` of the point it names.
    pub(crate) fn with_slack(self, slack: T) -> Self {
        Self { slack, ..self }
    }

    /// The cylinder's radius, `None` for a cone.
    pub(crate) fn radius(&self) -> Option<T> {
        self.radius
    }

    /// The cone's `cos α`, `None` for a cylinder.
    pub(crate) fn cone_cos(&self) -> Option<T> {
        self.cos
    }
}

/// **A cone's rulings against a cylinder or another cone.** A ruling is
/// the whole line `apex + t·v(φ)`, `v = cos α·â + sin α(cos φ·n + sin φ·m)`
/// (`n`, `m` unit, square to the axis and each other): `t > 0` is the
/// nappe `v` opens into at azimuth `φ`, `t < 0` the mirror nappe at
/// `φ + π`, so the turn of `φ` reads both nappes, each point of the
/// double cone but the apex on one ruling. With `u = apex − P` constant,
/// the partner's form along the ruling is `p₂t² + p₁t + p₀`:
/// `p₂ = κ + β(d·v)²`, `p₁ = 2(κ u·v + β(d·u)(d·v))` and the apex's own
/// `p₀ = q(apex)`, constant. Its discriminant `E = p₁² − 4p₂p₀` is a
/// trigonometric polynomial of degree two in `φ`.
///
/// `Δ` is in metres and shares `E`'s sign: against a cylinder the
/// ruling's clearance itself, `r − δ`, `δ` the ruling's distance from
/// the axis; against a cone the floor `E/16|u|` on it, `|u|` at least
/// the ruling's distance from the partner's apex (`topo`'s
/// `section_cert::ruling` docs derive `E/16d`; `E` does not depend on
/// where along the ruling `t` is measured).
///
/// Each sheet is a projective root of the quadratic,
/// `t_σ = (−p₁ + σ√E)/2p₂ = 2p₀/(−p₁ − σ√E)`, both forms read and met, so
/// a sheet stays enclosed where `p₂` vanishes (a ruling along an
/// asymptotic direction of the partner): there one form stays finite and
/// the other sheet leaves every bound, which [`Ruled::far`] reads as a
/// floor on its distance.
#[derive(Clone, Copy, Debug)]
pub(crate) struct RuledCone<T: Real> {
    pub(crate) apex: Point3<T>,
    /// The cone's unit axis.
    pub(crate) axis: Vec3<T>,
    pub(crate) n: Vec3<T>,
    pub(crate) m: Vec3<T>,
    /// `(sin α, cos α)`.
    pub(crate) sc: (T, T),
    pub(crate) partner: Form<T>,
    /// The lever an angle along the rulings is read at: the region's
    /// farthest reach from the apex.
    pub(crate) lever: geom_core::Lever<T>,
}

/// A ruling's quadratic over a cell: `p₂`, `p₁` and their slopes along
/// the angle, the constant `p₀`, `E` and its slope.
struct Quadratic<T: Real> {
    p2: Iv<T>,
    p1: Iv<T>,
    p0: Iv<T>,
    dp2: Iv<T>,
    dp1: Iv<T>,
    e: Iv<T>,
    de: Iv<T>,
}

impl<T: Real> RuledCone<T> {
    /// `v` over the cell.
    pub(crate) fn dir(&self, cell: &Cell<T>) -> V3<T> {
        let (s, c) = self.sc;
        V3::of(self.axis * c)
            .plus(V3::along(self.n, cell.cos.scale(s)))
            .plus(V3::along(self.m, cell.sin.scale(s)))
    }

    /// `v′` over the cell.
    fn turn(&self, cell: &Cell<T>) -> V3<T> {
        let (s, _) = self.sc;
        V3::along(self.n, cell.sin.scale(T::zero() - s)).plus(V3::along(self.m, cell.cos.scale(s)))
    }

    /// `u = apex − P`.
    fn u(&self) -> Vec3<T> {
        self.apex - self.partner.at
    }

    /// `|u|`, at least the ruling's distance from `P`.
    fn reach(&self) -> T {
        self.u().norm()
    }

    /// **The rounding each coefficient carries** at the scale `|u|`
    /// ([`rounding_charge`]): `p₂` of size two, `p₁` of `4|u|`, `p₀` of
    /// `2|u|² + |γ|`, `E` of their products.
    fn charges(&self) -> (T, T, T, T) {
        let big_u = self.reach();
        let two = T::from_f64(2.0);
        let four = T::from_f64(4.0);
        let s1 = four * big_u;
        let s0 = two * big_u.powi(2) + self.partner.gamma.abs();
        // `P`'s slack moves `u` by as much: `p₁` by at most four times it
        // (the form's matrix has norm at most one, `|v| = 1`), `p₀` by
        // `4|u|` times it, and `E` by `2|p₁|` and `4|p₂|` times those.
        let slack = self.partner.slack;
        let (m1, m0) = (four * slack, four * big_u * slack);
        (
            rounding_charge(two),
            rounding_charge(s1) + m1,
            rounding_charge(s0) + m0,
            rounding_charge(s1.powi(2) + T::from_f64(8.0) * s0)
                + two * s1 * m1
                + T::from_f64(8.0) * m0,
        )
    }

    fn quadratic(&self, cell: &Cell<T>) -> Quadratic<T> {
        let Form {
            d,
            kappa,
            beta,
            gamma,
            ..
        } = self.partner;
        let u = self.u();
        let two = T::from_f64(2.0);
        let (v, w) = (self.dir(cell), self.turn(cell));
        let (dv, uv) = (v.toward(d), v.toward(u));
        let (ddv, duv) = (w.toward(d), w.toward(u));
        let du = d.dot(u);
        let (c2, c1, c0, ce) = self.charges();
        let p2 = dv.square().scale(beta).shift(kappa).widen(c2);
        let p1 = uv
            .scale(two * kappa)
            .plus(dv.scale(two * beta * du))
            .widen(c1);
        let p0 = Iv::point(kappa * u.norm_squared() + beta * du.powi(2) + gamma).widen(c0);
        let dp2 = dv.times(ddv).scale(two * beta).widen(c2);
        let dp1 = duv
            .scale(two * kappa)
            .plus(ddv.scale(two * beta * du))
            .widen(c1);
        let minus_four = T::from_f64(-4.0);
        let e = p1.square().plus(p2.times(p0).scale(minus_four)).widen(ce);
        let de = p1
            .times(dp1)
            .scale(two)
            .plus(dp2.times(p0).scale(minus_four))
            .widen(ce);
        Quadratic {
            p2,
            p1,
            p0,
            dp2,
            dp1,
            e,
            de,
        }
    }

    /// `δ = |u·(v × d)| / |v × d|`, the ruling's distance from a
    /// cylinder's axis, held inside `[0, |u|]` (the distance between two
    /// of their points) where `|v × d|` reaches zero over the cell, and
    /// its slope along the angle, every value where `u·(v × d)` reaches
    /// zero (`δ` is not smooth there, and stands far from the radius).
    fn axis_distance(&self, cell: &Cell<T>) -> (Iv<T>, Iv<T>) {
        let (d, u) = (self.partner.d, self.u());
        let wide = T::from_f64(WIDE);
        let w = self.dir(cell).cross(d);
        let dw = self.turn(cell).cross(d);
        let (num, dnum) = (w.toward(u), dw.toward(u));
        let den = w.norm();
        let reach = self.reach();
        // The quotient's rounding: the numerator's, of size `|u|`, and the
        // divisor's relative one, both levered by `1/|v × d|`, which a
        // ruling near the axis's direction makes large.
        let charge = den
            .lo
            .select_le_zero(T::zero(), rounding_charge(reach + reach) / den.lo);
        let q = num.abs().quotient(den).widen(charge + self.partner.slack);
        let delta = Iv {
            lo: q.lo.max(T::zero()).min(reach),
            hi: q.hi.min(reach).max(T::zero()),
        };
        // `δ′ = sign(N)(N′D − N D′)/D²`, `D′ = (w·w′)/D`.
        let dden = w.dot(dw).quotient(den);
        // Its rounding: the numerator's, of size `|u|`, through the
        // quotient by `|v × d|²`, which a ruling near the axis's direction
        // levers as it levers `δ`'s.
        let slope_charge = den.lo.select_le_zero(
            T::zero(),
            rounding_charge(reach + reach) * (T::one() + reach / den.lo) / den.lo.powi(2),
        );
        let g = dnum
            .times(den)
            .plus(num.times(dden).neg())
            .quotient(den.square())
            .widen(slope_charge);
        let slope = Iv {
            lo: num.lo.select_le_zero(
                num.hi.select_le_zero(T::zero() - g.hi, T::zero() - wide),
                g.lo,
            ),
            hi: num
                .lo
                .select_le_zero(num.hi.select_le_zero(T::zero() - g.lo, wide), g.hi),
        };
        (delta, slope)
    }

    /// The sheet's `t` over the cell, both projective forms met.
    fn root(&self, q: &Quadratic<T>, sheet: Sheet) -> Iv<T> {
        let two = T::from_f64(2.0);
        let s = q.e.sqrt().scale(sheet.sign());
        let minus_p1 = q.p1.neg();
        let wide = minus_p1.plus(s).quotient(q.p2.scale(two));
        let narrow = q.p0.scale(two).quotient(minus_p1.plus(s.neg()));
        wide.meet(narrow)
    }

    /// The point `apex + t·v` over the cell, `t` the enclosure given.
    fn point(&self, cell: &Cell<T>, t: Iv<T>) -> V3<T> {
        let size = (self.apex - Point3::origin()).norm() + t.size();
        V3::at(self.apex)
            .plus(self.dir(cell).times(t))
            .widen(rounding_charge(size))
    }
}

impl<T: Real> Ruled<T> for RuledCone<T> {
    fn lever(&self) -> geom_core::Lever<T> {
        self.lever
    }

    fn discriminant(&self, cell: &Cell<T>) -> Iv<T> {
        match self.partner.radius {
            Some(r) => {
                let (delta, _) = self.axis_distance(cell);
                delta.from(r).widen(rounding_charge(r + self.reach()))
            }
            None => {
                let q = self.quadratic(cell);
                q.e.scale(T::one() / (T::from_f64(16.0) * self.reach()))
            }
        }
    }

    fn discriminant_slope(&self, cell: &Cell<T>) -> Iv<T> {
        match self.partner.radius {
            Some(r) => {
                let (_, slope) = self.axis_distance(cell);
                slope.neg().widen(rounding_charge(r + self.reach()))
            }
            None => {
                let q = self.quadratic(cell);
                q.de.scale(T::one() / (T::from_f64(16.0) * self.reach()))
            }
        }
    }

    fn meeting(&self, cell: &Cell<T>, sheet: Sheet) -> V3<T> {
        let q = self.quadratic(cell);
        self.point(cell, self.root(&q, sheet))
    }

    fn velocity(&self, cell: &Cell<T>, sheet: Sheet) -> V3<T> {
        // `t′ = −(p₂′t² + p₁′t) / (2p₂t + p₁)`, `2p₂t + p₁ = σ√E`.
        let q = self.quadratic(cell);
        let t = self.root(&q, sheet);
        let dt = q
            .dp2
            .times(t.square())
            .plus(q.dp1.times(t))
            .quotient(q.e.sqrt().scale(sheet.sign()))
            .neg();
        self.dir(cell).times(dt).plus(self.turn(cell).times(t))
    }

    /// `|t| ≥ |−p₁ + σ√E| / |2p₂|` and `≥ |2p₀| / |−p₁ − σ√E|`, each a
    /// floor whatever the divisor's sign, and `|X − c| ≥ |t| − |apex − c|`.
    fn far(&self, cell: &Cell<T>, sheet: Sheet, center: Point3<T>) -> Option<T> {
        let q = self.quadratic(cell);
        let two = T::from_f64(2.0);
        let s = q.e.sqrt().scale(sheet.sign());
        let minus_p1 = q.p1.neg();
        let floor = |num: Iv<T>, den: Iv<T>| {
            let (n, d) = (num.abs().lo, den.size());
            d.select_le_zero(T::zero(), n / d)
        };
        let t = floor(minus_p1.plus(s), q.p2.scale(two))
            .max(floor(q.p0.scale(two), minus_p1.plus(s.neg())));
        Some(t - (self.apex - center).norm())
    }

    fn ceiling(&self, cell: &Cell<T>) -> Option<T> {
        Some(self.form_ceiling(cell))
    }
}

impl<T: Real> RuledCone<T> {
    /// **The clearance ceiling over a cell**: at the ruling's point `X*`
    /// where the form is extreme along it (`t* = −p₁/2p₂`, the gradient
    /// `g` there square to the ruling), translating the ruling by `s`
    /// along `ĝ` moves the extreme to `q* + s|g| + s²k′`,
    /// `|k′| ≤ 1 + 1/|p₂|` (the form's matrix has norm at most one, for
    /// a cylinder and for a cone). Where `|g|² ≥ 4(1 + 1/|p₂|)|q*|` that
    /// has a root within `2|q*|/|g|`, where the ruling turns tangent, so
    /// its clearance is at most that; elsewhere it is no ceiling, and
    /// reads as every value ([`WIDE`]). `q* = −E/4p₂` is read from `E`
    /// by its mean value about the cell's middle,
    /// `|E| ≤ |E(φ₀)| + |E′|·w/2`, which is second order in the width
    /// where `E′` vanishes, as at a double root; the point `X*` itself
    /// moves to first order, so `q` read there directly would not be.
    pub(crate) fn form_ceiling(&self, cell: &Cell<T>) -> T {
        let Form {
            at, d, kappa, beta, ..
        } = self.partner;
        let wide = T::from_f64(WIDE);
        let two = T::from_f64(2.0);
        let q = self.quadratic(cell);
        let mid = self.quadratic(&Cell::at(cell.mid()));
        let half = T::from_computed(0.5 * cell.width());
        let e = mid.e.size() + q.de.size() * half;
        let value = e / (T::from_f64(4.0) * q.p2.abs().lo);
        let value = value + rounding_charge(value);
        let t = q.p1.quotient(q.p2.scale(two)).neg();
        let w = self.point(cell, t).less(at);
        let dw = w.toward(d);
        let gv = w
            .times(Iv::point(two * kappa))
            .plus(V3::along(d, dw.scale(two * beta)));
        let g = gv.norm().lo - rounding_charge(two * w.norm().hi);
        let k = T::one() + T::one() / q.p2.abs().lo;
        let room = T::from_f64(4.0) * k * value - g.powi(2);
        let ceiling = room.select_le_zero(two * value / g, wide);
        if ceiling.is_poison() { wide } else { ceiling }
    }

    /// **The chart of a crossing tangency at the ruling angle `phi`**
    /// ([`Crossing`]), read at the angle's own scalars.
    pub(crate) fn crossing(&self, phi: f64) -> Crossing<T> {
        let Form {
            at, d, kappa, beta, ..
        } = self.partner;
        let cell = Cell::at(phi);
        let mid = |v: V3<T>| v.mid() - Point3::origin();
        let (v, dv) = (mid(self.dir(&cell)), mid(self.turn(&cell)));
        let ddv = self.axis * self.sc.1 - v;
        let u = self.u();
        let two = T::from_f64(2.0);
        let (du, dvv, dvd, dvdd) = (d.dot(u), d.dot(v), d.dot(dv), d.dot(ddv));
        // The quadratic and its slopes as the sweep reads them, at the
        // angle; its curvatures, which nothing else reads, here.
        let q = self.quadratic(&cell);
        let at_angle = |i: Iv<T>| geom::mid_param(i.lo, i.hi);
        let (p2, p1, dp2, dp1) = (
            at_angle(q.p2),
            at_angle(q.p1),
            at_angle(q.dp2),
            at_angle(q.dp1),
        );
        let ddp2 = two * beta * (dvd.powi(2) + dvv * dvdd);
        let ddp1 = two * (kappa * u.dot(ddv) + beta * du * dvdd);
        let t = T::zero() - p1 / (two * p2);
        let (a, b, c) = (ddp2 * t.powi(2) + ddp1 * t, two * dp2 * t + dp1, two * p2);
        let root = (b.powi(2) - a * c).max(T::zero()).sqrt();
        let slopes = [(root - b) / c, (T::zero() - root - b) / c];
        let point = self.apex + v * t;
        let w = point - at;
        let gradient = (w * kappa + d * (beta * d.dot(w))) * two;
        Crossing {
            point,
            normal: (gradient * kappa.unit_sign()).normalize(),
            lines: slopes.map(|m| (v * m + dv * t).normalize()),
            slopes,
            form: [a, b, c],
            outward: kappa.unit_sign(),
        }
    }
}

/// **A crossing tangency's chart** ([`RuledCone::crossing`]): the partner's
/// form along the cone's rulings, `F(φ, t) = p₂t² + p₁t + p₀`, has a
/// critical point of value zero there, and its Hessian
/// `[[A, B], [B, C]]` (`A = F_φφ`, `B = F_φt`, `C = F_tt = 2p₂`) is
/// indefinite: its null directions `(1, m±)`, `m± = (−B ± √(B² − AC))/C`,
/// are the two branches, and their images `m±·v + t·v′` the node's lines.
/// Sheet `σ` leaves the node toward the side `e = ±1` of the angle
/// along `e·(1, m)`, `m` the root of sign `σe` (its `t′` is the mean
/// slope plus `σe·√(E″/2)/2p₂`, and `C = 2p₂`).
#[derive(Clone, Copy, Debug)]
pub(crate) struct Crossing<T: Real> {
    pub(crate) point: Point3<T>,
    /// The partner's unit normal there, outward.
    pub(crate) normal: Vec3<T>,
    /// The images of `(1, m+)` and `(1, m−)`, unit.
    pub(crate) lines: [Vec3<T>; 2],
    pub(crate) slopes: [T; 2],
    /// `[A, B, C]`.
    pub(crate) form: [T; 3],
    /// `+1` where the partner's form grows outward (a cylinder), `−1`
    /// where it shrinks (a cone).
    pub(crate) outward: T,
}

/// **A cone whose apex lies on its partner, its rulings read with the
/// apex root divided out** (C5): `p₀` is decided `Zero`, so each ruling
/// meets the partner at its apex and once more, at `t = −p₁/p₂`. The one
/// sheet that leaves runs the whole turn, through the apex where `p₁`
/// vanishes (a ruling in the partner's tangent plane there: the node's
/// lines) and past every bound where `p₂` does. Read as a ruled side of
/// one sheet: `Δ` is `p₁/2`, in metres, whose simple zeros
/// ([`Disc::Rise`], [`Disc::Fall`]) are the sheet's passes through the
/// apex, and the sheet is smooth where `p₂` stands off zero.
pub(crate) struct DividedCone<'a, T: Real>(pub(crate) &'a RuledCone<T>);

impl<T: Real> DividedCone<'_, T> {
    fn t(&self, q: &Quadratic<T>) -> Iv<T> {
        q.p1.quotient(q.p2).neg()
    }
}

impl<T: Real> Ruled<T> for DividedCone<'_, T> {
    fn lever(&self) -> geom_core::Lever<T> {
        self.0.lever
    }

    fn discriminant(&self, cell: &Cell<T>) -> Iv<T> {
        self.0.quadratic(cell).p1.scale(T::from_f64(0.5))
    }

    fn discriminant_slope(&self, cell: &Cell<T>) -> Iv<T> {
        self.0.quadratic(cell).dp1.scale(T::from_f64(0.5))
    }

    fn smooth(&self, cell: &Cell<T>) -> Iv<T> {
        self.0.quadratic(cell).p2.abs().scale(self.0.reach())
    }

    fn meeting(&self, cell: &Cell<T>, _: Sheet) -> V3<T> {
        let q = self.0.quadratic(cell);
        self.0.point(cell, self.t(&q))
    }

    fn velocity(&self, cell: &Cell<T>, _: Sheet) -> V3<T> {
        // `t′ = −(p₁′p₂ − p₁p₂′)/p₂²`.
        let q = self.0.quadratic(cell);
        let t = self.t(&q);
        let dt = q
            .dp1
            .times(q.p2)
            .plus(q.p1.times(q.dp2).neg())
            .quotient(q.p2.square())
            .neg();
        self.0.dir(cell).times(dt).plus(self.0.turn(cell).times(t))
    }

    fn far(&self, cell: &Cell<T>, _: Sheet, center: Point3<T>) -> Option<T> {
        let q = self.0.quadratic(cell);
        let d = q.p2.size();
        let t = d.select_le_zero(T::zero(), q.p1.abs().lo / d);
        Some(t - (self.0.apex - center).norm())
    }
}

/// **A component the sweep lists**: open between two ends, or a loop
/// the ball holds whole.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Swept<T: Real> {
    /// Its two ends in the order the sweep walks it, where it is open:
    /// each a certified root where it crosses the ball's boundary, or a
    /// node it ends at.
    pub(crate) ends: Option<[End<T>; 2]>,
    /// A point of it decided inside the ball (module docs: where).
    pub(crate) witness: Point3<T>,
    /// A loop of one sheet over the whole turn, which winds about the
    /// ruled side.
    pub(crate) encircles: bool,
    /// A loop the sweep read winding about the partner's axis (`Sweeper::winds_about`):
    /// read only where the sweep was handed that axis, and only for a
    /// loop the ball holds whole.
    pub(crate) winds_partner: bool,
}

impl<T: Real> Swept<T> {
    /// Its two cut ends, where both its ends are on the ball's boundary.
    pub(crate) fn cut_ends(&self) -> Option<[Point3<T>; 2]> {
        match self.ends {
            Some([End::Cut(a), End::Cut(b)]) => Some([a, b]),
            _ => None,
        }
    }
}

/// **Where a sheet passes through a node**: the ruling angle, the sheet,
/// and whether the walk runs it against the angle there. Which of the
/// node's rays an arc leaves along follows from the three.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Pass {
    pub(crate) phi: f64,
    pub(crate) sheet: Sheet,
    pub(crate) backward: bool,
}

/// **A double root of `Δ` the sweep decided a tangency** (C5): `Δ` turns
/// once over the cell, monotone either side of a core where its slope
/// is in band, and the side's clearance ceiling over that core
/// ([`Ruled::ceiling`]) is decided `Zero` on [`TANGENCY`]. Below zero
/// either side (a maximum), the rulings touch the partner at one point
/// of the section; above it (a minimum), two arcs of the section cross
/// there, a node of the section the arcs either side end at (C5: a
/// `Zero` tangency whose section still has carriers).
#[derive(Clone, Copy, Debug)]
pub(crate) struct Touch<T: Real> {
    /// The ruling's double meeting at the core's middle.
    pub(crate) at: Point3<T>,
    /// A minimum: the section's arcs cross there.
    pub(crate) crossing: bool,
    /// The ceiling's `Zero` margin.
    pub(crate) margin: geom_core::MarginDiag,
    /// The core's middle, the ruling angle the double meeting is read at.
    pub(crate) phi: f64,
}

/// The row a double root of `Δ` is decided a tangency on.
pub(crate) const TANGENCY: &str = "ruling_tangency";

/// **What one sweep read**: the components inside the ball, and how
/// many loops the discriminant's arcs make before the ball cuts them or
/// a crossing cuts them into arcs between nodes (one per arc of `Δ > 0`
/// bounded by branch points, two over the whole turn: the count the
/// cylinder pairs' poses check theirs against).
#[derive(Clone, Debug)]
pub(crate) struct Sweep<T: Real> {
    pub(crate) components: Vec<Swept<T>>,
    /// The loops `Δ`'s arcs make, uncut.
    pub(crate) loops: usize,
    /// The double roots of `Δ` decided tangencies.
    pub(crate) touches: Vec<Touch<T>>,
    /// Each whole loop whose winding about the partner's axis did not
    /// settle, by the margin that failed: it claims none
    /// (`Sweeper::read_windings`).
    pub(crate) unclaimed: Vec<&'static str>,
}

/// **Why a sweep refused.**
#[derive(Clone, Debug)]
pub(crate) enum SweepRefusal {
    /// A cell at the floor, or the cell that spent the budget, on its
    /// own reading.
    Escalated(Indeterminate),
    /// Two of the sweep's readings disagree about the side of the ball
    /// one stretch of a sheet lies on. A winding about the partner's axis
    /// never refuses: one that does not settle claims nothing
    /// ([`Sweep::unclaimed`]).
    Inconsistent(&'static str),
}

/// **A sweep's refusal as the section value's**, for every ruled pair: an
/// escalation is the value's escalation, and readings that disagree are
/// an unaccounted section, carrying their cause.
impl From<SweepRefusal> for crate::intersect::SectionError {
    fn from(e: SweepRefusal) -> Self {
        match e {
            SweepRefusal::Escalated(diag) => Self::Escalated(diag),
            SweepRefusal::Inconsistent(cause) => Self::Unaccounted {
                cause: cause.to_owned(),
            },
        }
    }
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
    /// A double root decided a tangency ([`Touch`]): `Δ` positive about
    /// it where `crossing`, negative otherwise.
    Touch {
        crossing: bool,
    },
}

impl Disc {
    /// The side of zero a cell's arcs read it on.
    fn positive(self) -> Option<bool> {
        match self {
            Self::Positive | Self::Touch { crossing: true } => Some(true),
            Self::Negative | Self::Touch { crossing: false } => Some(false),
            Self::Rise | Self::Fall => None,
        }
    }
}

/// One arc of `Δ > 0` between its two branch cells.
struct Arc<T: Real> {
    rise: Cell<T>,
    fall: Cell<T>,
    smooth: Vec<Cell<T>>,
}

/// Why a partner winding read claims nothing: a piece's box, with its
/// joints, at the floor or past the budget and not decided clear of the
/// partner's axis.
const UNCLEARED: &str = "a piece of the loop's walk stood undecided or short of clear of the partner's axis at the floor";
/// A chord's turn about the axis not decided within
/// [`super::winds::TRUSTED_TURN`] at the floor or past the budget.
const UNTRUSTED: &str =
    "a chord of the loop's walk turned past the trusted turn about the partner's axis at the floor";
/// A branch cell whose `Δ` decides at no cut the split offers.
const UNCUT: &str = "a branch cell of the loop's walk read `Δ` undecided at every cut";
/// The walk's summed turns in band of half a turn.
const UNTURNED: &str =
    "the loop's whole turns about the partner's axis read in band of half a turn";

/// **One piece of a loop's walk** (module docs, "The partner's winding"):
/// a cell read on one sheet, run against the angle where `backward`, or a
/// branch cell read on both sheets at once, `rising` where `Δ` rises
/// through its zero as the angle grows, so that the walk enters it on the
/// minus sheet and leaves it on the plus.
#[derive(Clone, Copy, Debug)]
enum Piece<T: Real> {
    Sheet {
        cell: Cell<T>,
        sheet: Sheet,
        backward: bool,
    },
    Branch {
        cell: Cell<T>,
        rising: bool,
    },
}

impl<T: Real> Piece<T> {
    fn sheet(cell: Cell<T>, sheet: Sheet, backward: bool) -> Self {
        Self::Sheet {
            cell,
            sheet,
            backward,
        }
    }

    fn cell(&self) -> Cell<T> {
        match *self {
            Self::Sheet { cell, .. } | Self::Branch { cell, .. } => cell,
        }
    }

    /// **An arc's loop as a walk**: its rise, the plus sheet on to its
    /// fall, then the minus sheet back.
    fn arc(arc: &Arc<T>) -> Vec<Self> {
        let mut walk = vec![Self::Branch {
            cell: arc.rise,
            rising: true,
        }];
        walk.extend(
            arc.smooth
                .iter()
                .map(|&c| Self::sheet(c, Sheet::Plus, false)),
        );
        walk.push(Self::Branch {
            cell: arc.fall,
            rising: false,
        });
        walk.extend(
            arc.smooth
                .iter()
                .rev()
                .map(|&c| Self::sheet(c, Sheet::Minus, true)),
        );
        walk
    }
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

impl<T: Real> Leaf<T> {
    fn cell(&self) -> &Cell<T> {
        match self {
            Self::Inside(cell) | Self::Outside(cell) | Self::Root { cell, .. } => cell,
        }
    }
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
    /// A pass through a node of the section: a crossing tangency
    /// ([`Touch`]), or the apex of a cone whose apex root is divided out
    /// ([`DividedCone`]).
    Node { pass: Pass },
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
    Sweeper::new(ruled, ball, band, None).run()
}

/// [`sweep`], each loop the ball holds whole also read for its winding
/// about the axis of `about`, the partner's frame, where the partner has
/// an essential channel (`Sweeper::winds_about`): a cylinder or a cone,
/// not a sphere.
///
/// # Errors
///
/// [`SweepRefusal`]; a winding no piece of the loop's walk settles
/// escalates on its own reading.
pub(crate) fn sweep_about<T: Decide, R: Ruled<T>>(
    ruled: &R,
    ball: ExtentBall<T>,
    band: Band,
    about: Option<geom::Frame<T>>,
) -> Result<Sweep<T>, SweepRefusal> {
    Sweeper::new(ruled, ball, band, about).run()
}

/// **An end of an arc of a sheet** ([`Swept::ends`]): a cut end on the
/// ball's boundary, or a node it ends at, where it passes through it.
#[derive(Clone, Copy, Debug)]
pub(crate) enum End<T: Real> {
    Cut(Point3<T>),
    Node(Pass),
}

/// **What a divided sweep read**: the arcs inside the ball, and the
/// ruling angles of the sheet's passes through the apex the ball holds.
#[derive(Clone, Debug)]
pub(crate) struct Divided<T: Real> {
    pub(crate) arcs: Vec<Swept<T>>,
    pub(crate) nodes: Vec<f64>,
    /// As [`Sweep::unclaimed`].
    pub(crate) unclaimed: Vec<&'static str>,
}

/// **The sweep of a cone whose apex lies on its partner** ([`DividedCone`]):
/// the one sheet read round the turn, its passes through the apex found
/// as the simple zeros of `p₁` and read against the ball as a branch
/// point is (both sides of the zero at once, the cell halved toward it),
/// and the turn cut into arcs at those passes and at the cut ends.
///
/// # Errors
///
/// [`SweepRefusal`].
pub(crate) fn sweep_divided<T: Decide>(
    cone: &RuledCone<T>,
    ball: ExtentBall<T>,
    band: Band,
    about: Option<geom::Frame<T>>,
) -> Result<Divided<T>, SweepRefusal> {
    let side = DividedCone(cone);
    let mut sweeper = Sweeper::new(&side, ball, band, about);
    let cells = sweeper.tiled()?;
    let mut events = Vec::new();
    for &(cell, disc) in &cells {
        match disc {
            Disc::Rise | Disc::Fall => events.extend(sweeper.passes(cell, disc == Disc::Rise)?),
            Disc::Positive | Disc::Negative | Disc::Touch { .. } => {
                for leaf in sweeper.leaves(&[cell], Sheet::Plus)? {
                    events.push(sweeper.event(leaf, Sheet::Plus, false));
                }
            }
        }
    }
    let nodes = events
        .iter()
        .filter_map(|e| match *e {
            Event::Node { pass } => Some(pass.phi),
            Event::Side(..) | Event::Cut { .. } => None,
        })
        .collect();
    let mut arcs = cut_noded(&events, true, events.len() / 2)?;
    // A loop the ball holds whole passes no node, so the sheet passes
    // through the apex nowhere on it: every cell is one of the sheet's.
    let turn: Vec<Piece<T>> = cells
        .iter()
        .map(|&(cell, _)| Piece::sheet(cell, Sheet::Plus, false))
        .collect();
    sweeper.read_windings(&mut arcs, || turn);
    Ok(Divided {
        arcs,
        nodes,
        unclaimed: sweeper.unclaimed,
    })
}

/// **A walk's readings cut into its arcs**: each stretch inside the ball
/// from a node or an entering cut end to the next node or leaving cut
/// end is an arc; a walk with neither is [`cut_loop`]'s, one loop where it
/// lies inside (witnessed at `events[middle]`), none where outside.
fn cut_noded<T: Real>(
    events: &[Event<T>],
    encircles: bool,
    middle: usize,
) -> Result<Vec<Swept<T>>, SweepRefusal> {
    let inconsistent =
        SweepRefusal::Inconsistent("the ruling sweep reads one stretch of a sheet on both sides");
    let boundary = |e: &Event<T>| {
        matches!(
            e,
            Event::Node { .. }
                | Event::Cut {
                    entering: false,
                    ..
                }
        )
    };
    if !events.iter().any(|e| matches!(e, Event::Node { .. })) {
        return cut_loop(events, encircles, middle);
    }
    let Some(start) = events.iter().position(boundary) else {
        return Err(inconsistent);
    };
    let n = events.len();
    let mut arcs = Vec::new();
    // The open arc: where it starts, the point beside an entering cut
    // end, and the points read inside along it.
    type Open<T> = (End<T>, Option<Point3<T>>, Vec<Point3<T>>);
    let opened = |e: Event<T>| -> Option<Open<T>> {
        match e {
            Event::Node { pass } => Some((End::Node(pass), None, Vec::new())),
            Event::Side(..) | Event::Cut { .. } => None,
        }
    };
    let closed =
        |(from, beside, points): Open<T>, to: End<T>| -> Result<Swept<T>, SweepRefusal> {
            let witness = points.get(points.len() / 2).copied().or(beside).ok_or(
                SweepRefusal::Inconsistent(
                    "the ruling sweep read an arc of a sheet with no point inside the ball",
                ),
            )?;
            Ok(Swept {
                ends: Some([from, to]),
                witness,
                encircles: false,
                winds_partner: false,
            })
        };
    let mut open = opened(events[start]);
    for k in 1..=n {
        open = match (open, events[(start + k) % n]) {
            (None, Event::Side(false, _)) => None,
            (
                None,
                Event::Cut {
                    entering: true,
                    at,
                    inside,
                },
            ) => Some((End::Cut(at), Some(inside), Vec::new())),
            (Some((from, beside, mut points)), Event::Side(true, at)) => {
                points.extend(at);
                Some((from, beside, points))
            }
            (
                Some(run),
                Event::Cut {
                    entering: false,
                    at,
                    ..
                },
            ) => {
                arcs.push(closed(run, End::Cut(at))?);
                None
            }
            (Some(run), Event::Node { pass }) => {
                arcs.push(closed(run, End::Node(pass))?);
                Some((End::Node(pass), None, Vec::new()))
            }
            _ => return Err(inconsistent),
        };
    }
    // Round the walk, it stands where it started.
    if open.is_some() != opened(events[start]).is_some() {
        return Err(inconsistent);
    }
    Ok(arcs)
}

struct Sweeper<'a, T: Real, R> {
    ruled: &'a R,
    ball: ExtentBall<T>,
    band: Band,
    spent: usize,
    touches: Vec<Touch<T>>,
    /// The angles of the crossing tangencies the ball holds, each a node
    /// every walk through it passes.
    crossings: Vec<f64>,
    /// The partner's frame, its origin on its axis, where the sweep reads
    /// each whole loop's winding about that axis (`Sweeper::winds_about`).
    about: Option<geom::Frame<T>>,
    /// The windings read and not settled ([`Sweep::unclaimed`]).
    unclaimed: Vec<&'static str>,
}

impl<'a, T: Decide, R: Ruled<T>> Sweeper<'a, T, R> {
    fn new(ruled: &'a R, ball: ExtentBall<T>, band: Band, about: Option<geom::Frame<T>>) -> Self {
        Self {
            ruled,
            ball,
            band,
            spent: 0,
            touches: Vec::new(),
            crossings: Vec::new(),
            about,
            unclaimed: Vec::new(),
        }
    }

    fn run(&mut self) -> Result<Sweep<T>, SweepRefusal> {
        let cells = self.tiled()?;
        // A crossing the ball holds is a node every walk through it
        // passes; one within band of its boundary is a node the walks may
        // or may not reach, and escalates.
        let mut crossings = Vec::new();
        for t in self.touches.iter().filter(|t| t.crossing) {
            let past = self.past(V3::at(t.at));
            match self.reads(Row::Cut, past) {
                Some(Reads::Below) => crossings.push(t.phi),
                Some(Reads::Above) => {}
                None => return Err(self.escalate(Row::Cut, past)),
            }
        }
        self.crossings = crossings;
        if cells.iter().all(|(_, d)| d.positive() == Some(true)) {
            let turn: Vec<Cell<T>> = cells.into_iter().map(|(c, _)| c).collect();
            let mut components = Vec::new();
            for sheet in [Sheet::Plus, Sheet::Minus] {
                let leaves = self.leaves(&turn, sheet)?;
                let events = self.walk(leaves, sheet, false);
                let mut sheets = cut_noded(&events, true, 0)?;
                self.read_windings(&mut sheets, || {
                    turn.iter()
                        .map(|&cell| Piece::sheet(cell, sheet, false))
                        .collect()
                });
                components.extend(sheets);
            }
            return Ok(Sweep {
                components,
                loops: 2,
                touches: core::mem::take(&mut self.touches),
                unclaimed: core::mem::take(&mut self.unclaimed),
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
            touches: core::mem::take(&mut self.touches),
            unclaimed: core::mem::take(&mut self.unclaimed),
        })
    }

    /// **The turn tiled into cells of `Δ`** ([`Disc`]), in the turn's
    /// order.
    fn tiled(&mut self) -> Result<Vec<(Cell<T>, Disc)>, SweepRefusal> {
        let mut cells = Vec::new();
        #[allow(clippy::cast_precision_loss)]
        let width = core::f64::consts::TAU / START as f64;
        // Each first cut where `Δ`'s side decides ([`Self::decided_at`]),
        // the last the first's turn on, so the cells tile the turn. A cut
        // after the first where `Δ` decides at none of the five angles
        // [`off_a_zero`] offers about it is dropped: a branch cell must
        // reach past the stretch in the band it stands in, so the cells
        // either side of it are read as one. The first cut is kept
        // wherever it stands, so every cell lies in the turn [`Cell`]
        // reads, and the turn is never read over no cell.
        let cuts: Vec<f64> = (0..START)
            .filter_map(|k| {
                #[allow(clippy::cast_precision_loss)]
                let base = width * k as f64;
                let at = self.decided_at(base, width, |phi| self.discriminant_reads(phi));
                (k == 0 || self.discriminant_reads(at)).then_some(at)
            })
            .collect();
        let Some(&first) = cuts.first() else {
            return Err(SweepRefusal::Inconsistent(
                "the ruling sweep kept no first cut, so its turn holds no cell",
            ));
        };
        for (k, &lo) in cuts.iter().enumerate() {
            let hi = cuts
                .get(k + 1)
                .copied()
                .unwrap_or(first + core::f64::consts::TAU);
            self.discriminant_cell(Cell::new(lo, hi), &mut cells)?;
        }
        Ok(cells)
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

    /// **Which side of zero `reading` is decided on**: its lower end
    /// past the band above, or its upper end past it below. A reading
    /// that straddles zero, or reaches into the band, decides neither.
    fn reads(&self, row: Row, reading: Iv<T>) -> Option<Reads> {
        if self.positive(row, Margin::of(reading.lo)) {
            Some(Reads::Above)
        } else if self.positive(row, Margin::of(T::zero() - reading.hi)) {
            Some(Reads::Below)
        } else {
            None
        }
    }

    fn positive(&self, row: Row, m: Margin<T>) -> bool {
        matches!(decide(row.name(), m, self.band), Ok(Sign::Positive))
    }

    /// The bounds of `reading` straddle the band: its lower end decided
    /// negative and its upper end positive, past both of the band's
    /// edges.
    fn straddles(&self, row: Row, reading: Iv<T>) -> bool {
        self.positive(row, Margin::of(T::zero() - reading.lo))
            && self.positive(row, Margin::of(reading.hi))
    }

    /// **The escalation of a cell no test settled**, on the enclosure
    /// `reading`: bounds that straddle the band escalate as one straddle
    /// on the row's own straddle row, bounds both `Zero` as a `Zero` no
    /// side passes.
    fn escalate(&self, row: Row, reading: Iv<T>) -> SweepRefusal {
        let name = row.name();
        let bound_as = |m: T| decide_reported(name, Margin::of(m), self.band);
        SweepRefusal::Escalated(match (bound_as(reading.lo), bound_as(reading.hi)) {
            (Err(e), _) | (_, Err(e)) => e,
            (Ok(lower), Ok(upper)) if lower.sign < upper.sign => {
                straddle(row.straddle(), lower, upper, self.band)
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

    /// Whether `Δ`'s side decides at the single angle `phi`.
    fn discriminant_reads(&self, phi: f64) -> bool {
        self.reads(Row::Discriminant, self.ruled.discriminant(&Cell::at(phi)))
            .is_some()
    }

    /// **A cut near `at`, within a cell of `width`, where `decides`**: the
    /// first of [`off_a_zero`]'s where the reading decides, else `at`. A cell's end is a
    /// reading every test of the cell shares, so an end within the band
    /// of a zero would hold the cell undecided to the floor at any
    /// width; off the zero, it decides.
    fn decided_at(&self, at: f64, width: f64, decides: impl Fn(f64) -> bool) -> f64 {
        off_a_zero(at, width)
            .find(|&phi| decides(phi))
            .unwrap_or(at)
    }

    /// **`cell` cut in two** near its middle, where the reading `decides`
    /// ([`Self::decided_at`]).
    fn split(&self, cell: &Cell<T>, decides: impl Fn(f64) -> bool) -> (Cell<T>, Cell<T>) {
        let at = self.decided_at(cell.mid(), cell.width(), decides);
        (Cell::new(cell.lo, at), Cell::new(at, cell.hi))
    }

    /// The signed distance of the meetings `x` past the ball's boundary.
    fn past(&self, x: V3<T>) -> Iv<T> {
        x.less(self.ball.center())
            .norm()
            .shift(T::zero() - self.ball.radius())
    }

    /// **The signed distance of the cell's meetings on `sheet` past the
    /// ball's boundary**: [`Self::past`] of their enclosure, raised to
    /// the side's floor on their distance where it has one
    /// ([`Ruled::far`]).
    fn side(&self, cell: &Cell<T>, sheet: Sheet) -> Iv<T> {
        let h = self.past(self.ruled.meeting(cell, sheet));
        match self.ruled.far(cell, sheet, self.ball.center()) {
            Some(floor) => Iv {
                lo: h.lo.max(floor - self.ball.radius()),
                hi: h.hi,
            },
            None => h,
        }
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
        let kind = if self.positive(Row::Discriminant, Margin::of(m.lo)) {
            Some(Disc::Positive)
        } else if self.positive(Row::Discriminant, Margin::of(T::zero() - m.hi)) {
            Some(Disc::Negative)
        } else {
            self.branch(&cell)
        };
        let kind = match kind {
            Some(kind) => Some(kind),
            None => self.touch(&cell)?.map(|t| {
                let crossing = t.crossing;
                self.touches.push(t);
                Disc::Touch { crossing }
            }),
        };
        if let Some(kind) = kind {
            out.push((cell, kind));
            return Ok(());
        }
        if self.spent_on(&cell) {
            return Err(self.escalate(Row::Discriminant, m));
        }
        let (l, r) = self.split(&cell, |phi| self.discriminant_reads(phi));
        self.discriminant_cell(l, out)?;
        self.discriminant_cell(r, out)
    }

    /// `Δ` monotone over `cell` with its ends decided on opposite sides:
    /// one simple zero, a branch point.
    fn branch(&self, cell: &Cell<T>) -> Option<Disc> {
        let slope = self.ruled.discriminant_slope(cell);
        let monotone = self.positive(Row::Slope, Margin::of(slope.lo))
            || self.positive(Row::Slope, Margin::of(T::zero() - slope.hi));
        if !monotone {
            return None;
        }
        let end = |phi: f64| self.reads(Row::Discriminant, self.ruled.discriminant(&Cell::at(phi)));
        match (end(cell.lo), end(cell.hi)) {
            (Some(Reads::Below), Some(Reads::Above)) => Some(Disc::Rise),
            (Some(Reads::Above), Some(Reads::Below)) => Some(Disc::Fall),
            _ => None,
        }
    }

    /// **A double root of `Δ` in `cell`, decided a tangency** ([`Touch`]),
    /// for a side that reads a clearance ceiling ([`Ruled::ceiling`]):
    /// `Δ` decided on one side at both ends and turning between them (a
    /// maximum below zero, or a minimum above it), its slope decided at
    /// both ends; the core where the slope is in band found by bisection
    /// and widened by half its width either way; `Δ` monotone from each
    /// end to the core, every piece's slope decided; and the ceiling over
    /// the core decided on [`TANGENCY`]. `Zero` is the tangency; past it,
    /// or a cell of another shape, is none, and the cell is split; in
    /// band, it escalates.
    fn touch(&mut self, cell: &Cell<T>) -> Result<Option<Touch<T>>, SweepRefusal> {
        if self.ruled.ceiling(&Cell::at(cell.mid())).is_none() {
            return Ok(None);
        }
        let disc =
            |phi: f64| self.reads(Row::Discriminant, self.ruled.discriminant(&Cell::at(phi)));
        let slope =
            |phi: f64| self.reads(Row::Slope, self.ruled.discriminant_slope(&Cell::at(phi)));
        let (left, right, crossing) =
            match (disc(cell.lo), disc(cell.hi), slope(cell.lo), slope(cell.hi)) {
                (
                    Some(Reads::Below),
                    Some(Reads::Below),
                    Some(Reads::Above),
                    Some(Reads::Below),
                ) => (Reads::Above, Reads::Below, false),
                (
                    Some(Reads::Above),
                    Some(Reads::Above),
                    Some(Reads::Below),
                    Some(Reads::Above),
                ) => (Reads::Below, Reads::Above, true),
                _ => return Ok(None),
            };
        // The core: from the last angle whose slope reads `left` to the
        // first that reads `right`.
        let (mut l, mut r) = (cell.lo, cell.hi);
        let mut flat = None;
        for _ in 0..SETTLE {
            let mid = geom::mid_param(l, r);
            match slope(mid) {
                Some(x) if x == left => l = mid,
                Some(_) => r = mid,
                None => {
                    flat = Some(mid);
                    break;
                }
            }
        }
        if let Some(flat) = flat {
            let (mut a, mut b) = (flat, flat);
            for _ in 0..SETTLE {
                let mid = geom::mid_param(l, a);
                if slope(mid) == Some(left) {
                    l = mid
                } else {
                    a = mid
                }
                let mid = geom::mid_param(b, r);
                if slope(mid) == Some(right) {
                    r = mid
                } else {
                    b = mid
                }
            }
        }
        let width = 0.5 * (r - l);
        let (a, b) = ((l - width).max(cell.lo), (r + width).min(cell.hi));
        if disc(geom::mid_param(a, b)).is_some() {
            return Ok(None);
        }
        if !self.monotone_over(cell.lo, a, left) || !self.monotone_over(b, cell.hi, right) {
            return Ok(None);
        }
        let core = Cell::new(a, b);
        let Some(ceiling) = self.ruled.ceiling(&core) else {
            return Ok(None);
        };
        match decide_reported(TANGENCY, Margin::of(ceiling), self.band) {
            Ok(geom_core::Decided {
                sign: Sign::Zero,
                margin,
            }) => Ok(Some(Touch {
                at: self
                    .ruled
                    .meeting(&Cell::at(geom::mid_param(a, b)), Sheet::Plus)
                    .mid(),
                crossing,
                margin,
                phi: geom::mid_param(a, b),
            })),
            Ok(_) => Ok(None),
            Err(diag) => Err(SweepRefusal::Escalated(diag)),
        }
    }

    /// Whether `[lo, hi]` tiles into pieces each with `Δ`'s slope read
    /// `want`, within the floor and the budget.
    fn monotone_over(&mut self, lo: f64, hi: f64, want: Reads) -> bool {
        let mut open = vec![(lo, hi)];
        while let Some((lo, hi)) = open.pop() {
            if hi <= lo {
                continue;
            }
            let cell = Cell::new(lo, hi);
            self.spend();
            if self.reads(Row::Slope, self.ruled.discriminant_slope(&cell)) == Some(want) {
                continue;
            }
            if self.spent_on(&cell) {
                return false;
            }
            let mid = cell.mid();
            open.extend([(lo, mid), (mid, hi)]);
        }
        true
    }

    /// **One arc's components**: its branch cells read sheet-wise, its
    /// two sheets' leaves, and the loop they make cut by the ball.
    fn arc_components(&mut self, arc: &Arc<T>) -> Result<Vec<Swept<T>>, SweepRefusal> {
        let mut smooth = arc.smooth.clone();
        let rise = self.branch_side(arc.rise, true, &mut smooth)?;
        let fall = self.branch_side(arc.fall, false, &mut smooth)?;
        let plus = self.leaves(&smooth, Sheet::Plus)?;
        // The leaf at the arc's middle, by the angle the arc spans: the
        // leaves tile `smooth`, which runs from the rise to the fall.
        let half = smooth.iter().map(Cell::width).sum::<f64>() / 2.0;
        let middle = plus
            .iter()
            .scan(0.0, |run, l| {
                *run += l.cell().width();
                Some(*run)
            })
            .position(|run| run >= half)
            .unwrap_or(plus.len().saturating_sub(1));
        // Along the loop: the first sheet from the rise to the fall, then
        // the second back, each through the crossings on the arc.
        let mut events = vec![Event::Side(rise, None)];
        events.extend(self.walk(plus, Sheet::Plus, false));
        events.push(Event::Side(fall, None));
        let mut back = self.leaves(&smooth, Sheet::Minus)?;
        back.reverse();
        events.extend(self.walk(back, Sheet::Minus, true));
        let mut components = cut_noded(&events, false, 1 + middle)?;
        self.read_windings(&mut components, || Piece::arc(arc));
        Ok(components)
    }

    /// **A walk along `sheet` over `leaves`**, in the walk's order
    /// (`backward`: against the angle), a pass through each crossing the
    /// leaves reach placed before the first leaf past its angle.
    fn walk(&self, leaves: Vec<Leaf<T>>, sheet: Sheet, backward: bool) -> Vec<Event<T>> {
        // The ruling angle's period, and half of it.
        let turn = geom_core::period::Period::<f64>::angle().length();
        let half = 0.5 * turn;
        // The leaves' angles unwrapped in the walk's own order: an arc read
        // across the turn's seam ([`arcs`]) steps back a turn there, and
        // the walk reads its angles on from its start.
        let mut offset = 0.0;
        let mut last: Option<f64> = None;
        let span: Vec<(f64, f64)> = leaves
            .iter()
            .map(|l| {
                let c = l.cell();
                if let Some(last) = last {
                    if !backward && c.lo + offset < last - half {
                        offset += turn;
                    } else if backward && c.hi + offset > last + half {
                        offset -= turn;
                    }
                }
                last = Some(if backward {
                    c.lo + offset
                } else {
                    c.hi + offset
                });
                (c.lo + offset, c.hi + offset)
            })
            .collect();
        let (lo, hi) = span
            .iter()
            .fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), &(a, b)| {
                (lo.min(a), hi.max(b))
            });
        // Each crossing at its angle in that frame, a turn either way.
        let mut pending: Vec<(f64, f64)> = self
            .crossings
            .iter()
            .flat_map(|&phi| {
                [-2.0, -1.0, 0.0, 1.0, 2.0]
                    .into_iter()
                    .map(move |k| (phi, phi + k * turn))
            })
            .filter(|&(_, at)| lo < at && at < hi)
            .collect();
        let node = |phi: f64| Event::Node {
            pass: Pass {
                phi,
                sheet,
                backward,
            },
        };
        let mut out = Vec::new();
        for (leaf, (a, b)) in leaves.into_iter().zip(span) {
            pending.retain(|&(phi, at)| {
                let past = if backward { b <= at } else { a >= at };
                if past {
                    out.push(node(phi));
                }
                !past
            });
            out.push(self.event(leaf, sheet, backward));
        }
        out.extend(pending.into_iter().map(|(phi, _)| node(phi)));
        out
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
                .side(&cell, Sheet::Plus)
                .hull(self.side(&cell, Sheet::Minus));
            self.spend();
            // An arc with no cell clear of its branch points has no stretch
            // to witness it on: split until it has one.
            let side = if self.positive(Row::Branch, Margin::of(T::zero() - both.hi)) {
                Some(true)
            } else if self.positive(Row::Branch, Margin::of(both.lo)) {
                Some(false)
            } else {
                None
            };
            if let (Some(inside), false) = (side, smooth.is_empty()) {
                return Ok(inside);
            }
            if self.spent_on(&cell) {
                return Err(match side {
                    None => self.escalate(Row::Branch, both),
                    Some(_) => self.escalate(Row::Discriminant, self.ruled.discriminant(&cell)),
                });
            }
            // Cut where `Δ`'s side decides, off the middle where the zero
            // stands within the band of it.
            let cut = off_a_zero(cell.mid(), cell.width()).find_map(|at| {
                self.reads(Row::Discriminant, self.ruled.discriminant(&Cell::at(at)))
                    .map(|reads| (at, reads == Reads::Above))
            });
            let Some((at, positive)) = cut else {
                // The branch point placed to within the band of the zero:
                // where its side of the ball is still undecided, a
                // boundary within band of it.
                return Err(match side {
                    None => self.escalate(Row::Branch, both),
                    Some(_) => self.escalate(Row::Discriminant, self.ruled.discriminant(&cell)),
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

    /// **A pass of a divided sheet through the apex in `cell`**, read
    /// against the ball as [`Self::branch_side`] reads a branch point:
    /// the cell's meetings decided inside (a node) or outside, else the
    /// cell cut where `Δ` decides and the half clear of the zero read
    /// leaf by leaf, in the turn's order about the pass. `rising`: `Δ`
    /// reads below zero at the cell's start.
    fn passes(&mut self, mut cell: Cell<T>, rising: bool) -> Result<Vec<Event<T>>, SweepRefusal> {
        let start = if rising { Reads::Below } else { Reads::Above };
        let (mut before, mut after) = (Vec::new(), Vec::new());
        let at_pass = loop {
            let h = self.side(&cell, Sheet::Plus);
            self.spend();
            if self.positive(Row::Branch, Margin::of(T::zero() - h.hi)) {
                // The cell's ends are points of the arcs either side,
                // decided inside with it: an arc between two passes reads
                // a point even where no leaf lies between them.
                let at = |phi: f64| Some(self.ruled.meeting(&Cell::at(phi), Sheet::Plus).mid());
                before.push(Event::Side(true, at(cell.lo)));
                after.insert(0, Event::Side(true, at(cell.hi)));
                break Event::Node {
                    pass: Pass {
                        phi: cell.mid(),
                        sheet: Sheet::Plus,
                        backward: false,
                    },
                };
            }
            if self.positive(Row::Branch, Margin::of(h.lo)) {
                break Event::Side(false, None);
            }
            if self.spent_on(&cell) {
                return Err(self.escalate(Row::Branch, h));
            }
            let cut = off_a_zero(cell.mid(), cell.width()).find_map(|at| {
                self.reads(Row::Discriminant, self.ruled.discriminant(&Cell::at(at)))
                    .map(|reads| (at, reads))
            });
            let Some((at, reads)) = cut else {
                return Err(self.escalate(Row::Branch, h));
            };
            let (l, r) = (Cell::new(cell.lo, at), Cell::new(at, cell.hi));
            if reads == start {
                // The zero lies past the cut.
                for leaf in self.leaves(&[l], Sheet::Plus)? {
                    before.push(self.event(leaf, Sheet::Plus, false));
                }
                cell = r;
            } else {
                let mut right = Vec::new();
                for leaf in self.leaves(&[r], Sheet::Plus)? {
                    right.push(self.event(leaf, Sheet::Plus, false));
                }
                right.extend(after);
                after = right;
                cell = l;
            }
        };
        before.push(at_pass);
        before.extend(after);
        Ok(before)
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
        let h = self.side(&cell, sheet);
        self.spend();
        if self.positive(Row::Cut, Margin::of(T::zero() - h.hi)) {
            out.push(Leaf::Inside(cell));
            return Ok(());
        }
        if self.positive(Row::Cut, Margin::of(h.lo)) {
            out.push(Leaf::Outside(cell));
            return Ok(());
        }
        if let Some(leaf) = self.monotone(&cell, sheet, x)? {
            out.push(leaf);
            return Ok(());
        }
        if self.spent_on(&cell) {
            return Err(self.escalate(Row::Cut, h));
        }
        let (l, r) = self.split(&cell, |phi| {
            self.reads(Row::Cut, self.side(&Cell::at(phi), sheet))
                .is_some()
        });
        self.leaf(l, sheet, out)?;
        self.leaf(r, sheet, out)
    }

    /// **The sheet monotone against the ball over `cell`**: the distance
    /// from the ball's centre moves one way across it, so its two ends
    /// decide it, inside, outside, or one cut end between them. Read
    /// only where the side reads its sheets smooth ([`Ruled::smooth`]).
    fn monotone(
        &self,
        cell: &Cell<T>,
        sheet: Sheet,
        x: V3<T>,
    ) -> Result<Option<Leaf<T>>, SweepRefusal> {
        if !self.positive(Row::Discriminant, Margin::of(self.ruled.smooth(cell).lo)) {
            return Ok(None);
        }
        let dx = self.ruled.velocity(cell, sheet);
        let w = x.less(self.ball.center());
        let along = w.dot(dx);
        let scale = w.norm().hi * dx.norm().hi;
        // `(x − c)·x′` over `|x − c|·|x′|`, the cosine between the sheet's
        // tangent and the ray from the centre, levered by the ruling's.
        let lever = self.ruled.lever();
        let monotone = self.positive(Row::Monotone, Margin::levered(along.lo / scale, lever))
            || self.positive(
                Row::Monotone,
                Margin::levered((T::zero() - along.hi) / scale, lever),
            );
        if !monotone {
            return Ok(None);
        }
        let end = |phi: f64| self.reads(Row::Cut, self.side(&Cell::at(phi), sheet));
        Ok(match (end(cell.lo), end(cell.hi)) {
            (Some(Reads::Below), Some(Reads::Below)) => Some(Leaf::Inside(*cell)),
            (Some(Reads::Above), Some(Reads::Above)) => Some(Leaf::Outside(*cell)),
            (Some(first), Some(_)) => {
                let entering = first == Reads::Above;
                Some(Leaf::Root {
                    cell: *cell,
                    entering,
                    at: self.settle(cell, sheet, entering)?,
                })
            }
            _ => None,
        })
    }

    /// **A cut end placed in its cell** by bisection on the sides of the
    /// meetings, stopping where the midpoint's reading lies within the
    /// band of the boundary: a selection within the isolating cell, whose
    /// root it narrows to the band. A midpoint whose reading straddles the
    /// band, its enclosure wider than the band about the boundary, cannot
    /// place the root there, and escalates on the cut's row.
    fn settle(
        &self,
        cell: &Cell<T>,
        sheet: Sheet,
        entering: bool,
    ) -> Result<Point3<T>, SweepRefusal> {
        let (mut lo, mut hi) = (cell.lo, cell.hi);
        let at = |phi: f64| self.ruled.meeting(&Cell::at(phi), sheet);
        for _ in 0..SETTLE {
            let mid = geom::mid_param(lo, hi);
            let past = self.side(&Cell::at(mid), sheet);
            match (self.reads(Row::Cut, past), entering) {
                // Outside at `mid`: the root lies on the inside's side.
                (Some(Reads::Above), true) | (Some(Reads::Below), false) => lo = mid,
                (Some(Reads::Above), false) | (Some(Reads::Below), true) => hi = mid,
                (None, _) if self.straddles(Row::Cut, past) => {
                    return Err(self.escalate(Row::Cut, past));
                }
                (None, _) => return Ok(at(mid).mid()),
            }
        }
        Ok(at(geom::mid_param(lo, hi)).mid())
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

    /// **Each loop of `components` the ball holds whole read for its
    /// winding about the partner's axis** ([`Self::winds_about`]), over
    /// the walk `walk` gives, where the sweep was handed that axis. A
    /// walk is one sheet's turn or one arc's loop, so it makes one loop
    /// or none. A read that does not settle claims nothing (a weaker
    /// claim, never a wrong one), and is recorded with the margin that
    /// failed ([`Sweep::unclaimed`]).
    fn read_windings(&mut self, components: &mut [Swept<T>], walk: impl FnOnce() -> Vec<Piece<T>>) {
        let Some(frame) = self.about else {
            return;
        };
        if let Some(whole) = components.iter_mut().find(|c| c.ends.is_none()) {
            match self.winds_about(&frame, walk()) {
                Ok(winds) => whole.winds_partner = winds,
                Err(why) => self.unclaimed.push(why),
            }
        }
    }

    /// **Whether the loop `walk` runs round winds about `frame`'s axis**
    /// (module docs, "The partner's winding"): each piece's box, with the
    /// joints it shares with its neighbours, decided clear of the axis in
    /// the frame's plane and its turn there decided within
    /// [`super::winds::TRUSTED_TURN`]; a piece that is not is split. The
    /// turns' sum is then decided past half a turn or short of it.
    ///
    /// # Errors
    ///
    /// The margin that did not settle, named: a piece at the floor or
    /// past the budget, a branch cell `Δ` decides at no cut, or a sum in
    /// band of half a turn. The caller claims nothing for it.
    fn winds_about(
        &mut self,
        frame: &geom::Frame<T>,
        mut walk: Vec<Piece<T>>,
    ) -> Result<bool, &'static str> {
        let trusted = T::from_f64(super::winds::TRUSTED_TURN);
        let lever = self.ruled.lever();
        loop {
            let boxes: Vec<V3<T>> = walk.iter().map(|p| self.piece_box(p)).collect();
            let n = boxes.len();
            let joint = |i: usize| boxes[(i + n - 1) % n].meet(boxes[i % n]);
            let start = frame.azimuth(joint(0).mid()).near(T::zero());
            let mut from = start;
            let mut unsettled = None;
            for (i, piece) in boxes.iter().enumerate() {
                let (into, out) = (joint(i), joint(i + 1));
                let clear = Margin::of(self.clearance(frame, piece.hull(into).hull(out)));
                if !self.positive(Row::Winds, clear) {
                    unsettled = Some((i, UNCLEARED));
                    break;
                }
                let to = frame.azimuth(out.mid()).near(from);
                let short = Margin::levered(trusted - (to - from).abs(), lever);
                if !self.positive(Row::Winds, short) {
                    unsettled = Some((i, UNTRUSTED));
                    break;
                }
                from = to;
            }
            match unsettled {
                None => return self.whole_turns(from - start, lever),
                Some((i, why)) => self.split_piece(&mut walk, i, why)?,
            }
        }
    }

    /// **The box a piece's meetings lie in**: its sheet's, or both sheets'
    /// over a branch cell.
    fn piece_box(&self, piece: &Piece<T>) -> V3<T> {
        match *piece {
            Piece::Sheet { cell, sheet, .. } => self.ruled.meeting(&cell, sheet),
            Piece::Branch { cell, .. } => self
                .ruled
                .meeting(&cell, Sheet::Plus)
                .hull(self.ruled.meeting(&cell, Sheet::Minus)),
        }
    }

    /// **How far the box `b` stands clear of `frame`'s axis**, read in
    /// the frame's plane: the largest of its rectangle's four distances
    /// past the axis, positive where the rectangle misses it.
    fn clearance(&self, frame: &geom::Frame<T>, b: V3<T>) -> T {
        let w = b.less(frame.origin());
        let (x, y) = (w.toward(frame.u_ref()), w.toward(frame.v_ref()));
        x.lo.max(T::zero() - x.hi).max(y.lo.max(T::zero() - y.hi))
    }

    /// **The walk's whole turns read from its turning `total`**: decided
    /// past half a turn (it winds) or short of it (it does not). The sum
    /// telescopes: each chord's turn ends at the azimuth the next one
    /// starts from, so `total` is the last joint's azimuth less the
    /// first's, the same joint read twice, a whole number of turns to
    /// rounding, and no closing check is read.
    fn whole_turns(&self, total: T, lever: geom_core::Lever<T>) -> Result<bool, &'static str> {
        match decide(
            Row::Winds.name(),
            Margin::levered(total.abs() - T::pi(), lever),
            self.band,
        ) {
            Ok(Sign::Positive) => Ok(true),
            Ok(Sign::Negative) => Ok(false),
            Ok(Sign::Zero) | Err(_) => Err(UNTURNED),
        }
    }

    /// **Piece `i` of `walk` split** where its `reading` did not settle:
    /// a sheet's cell in two, in the walk's order; a branch cell where
    /// `Δ` decides, the zero's half staying the branch, the positive half
    /// a cell of both sheets either side of it, the negative half dropped
    /// (no meeting lies over it). At the floor or past the budget, the
    /// margin `why` names did not settle.
    fn split_piece(
        &mut self,
        walk: &mut Vec<Piece<T>>,
        i: usize,
        why: &'static str,
    ) -> Result<(), &'static str> {
        let piece = walk[i];
        let cell = piece.cell();
        self.spend();
        if self.spent_on(&cell) {
            return Err(why);
        }
        match piece {
            Piece::Sheet {
                sheet, backward, ..
            } => {
                let (l, r) = self.split(&cell, |_| true);
                let (a, b) = if backward { (r, l) } else { (l, r) };
                walk[i] = Piece::sheet(a, sheet, backward);
                walk.insert(i + 1, Piece::sheet(b, sheet, backward));
            }
            Piece::Branch { rising, .. } => {
                let cut = off_a_zero(cell.mid(), cell.width()).find_map(|at| {
                    self.reads(Row::Discriminant, self.ruled.discriminant(&Cell::at(at)))
                        .map(|reads| (at, reads == Reads::Above))
                });
                let Some((at, positive)) = cut else {
                    return Err(UNCUT);
                };
                let (l, r) = (Cell::new(cell.lo, at), Cell::new(at, cell.hi));
                let (zero, clear) = match (positive, rising) {
                    (true, true) => (l, Some(r)),
                    (false, true) => (r, None),
                    (true, false) => (r, Some(l)),
                    (false, false) => (l, None),
                };
                walk[i] = Piece::Branch { cell: zero, rising };
                if let Some(c) = clear {
                    // A rise is entered on the minus sheet and left on the
                    // plus; a fall the other way round.
                    let (before, after) = if rising {
                        (
                            Piece::sheet(c, Sheet::Minus, true),
                            Piece::sheet(c, Sheet::Plus, false),
                        )
                    } else {
                        (
                            Piece::sheet(c, Sheet::Plus, false),
                            Piece::sheet(c, Sheet::Minus, true),
                        )
                    };
                    walk.insert(i + 1, after);
                    if i == 0 {
                        walk.push(before);
                    } else {
                        walk.insert(i, before);
                    }
                }
            }
        }
        Ok(())
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
        return if cells.iter().all(|(_, d)| d.positive() == Some(false)) {
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
        open = match (
            open,
            kind.positive()
                .map_or(kind, |p| if p { Disc::Positive } else { Disc::Negative }),
        ) {
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
/// lies inside, none where outside, witnessed at `events[middle]`.
/// `encircles` is the loop's winding about the ruled side.
fn cut_loop<T: Real>(
    events: &[Event<T>],
    encircles: bool,
    middle: usize,
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
        if events.iter().any(|e| !matches!(e, Event::Side(..))) {
            return Err(inconsistent);
        }
        let sides: Vec<bool> = events
            .iter()
            .filter_map(|e| match *e {
                Event::Side(inside, _) => Some(inside),
                Event::Cut { .. } | Event::Node { .. } => None,
            })
            .collect();
        let inside = sides.first().copied().unwrap_or(false);
        if sides.iter().any(|&s| s != inside) {
            return Err(inconsistent);
        }
        if !inside {
            return Ok(Vec::new());
        }
        let Some(Event::Side(true, Some(witness))) = events.get(middle).copied() else {
            return Err(SweepRefusal::Inconsistent(
                "the ruling sweep read a loop inside the ball but no point at its middle",
            ));
        };
        return Ok(vec![Swept {
            ends: None,
            witness,
            encircles,
            winds_partner: false,
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
                    ends: Some([End::Cut(from), End::Cut(at)]),
                    witness: points.get(points.len() / 2).copied().unwrap_or(inside),
                    encircles: false,
                    winds_partner: false,
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

/// **The ruling angle's coverage of a pair with a ruled cylinder or cone**, a
/// second reading of the count the sweep closed exhaustiveness on (C3):
/// every cell of `ruled`'s ruling angle has its rulings' meetings with
/// `partner` outside `ball` (the ball the march ends at), or inside one of
/// `tubes` (the found branches' tube boxes). A component the count missed
/// has meetings in no tube. It is read on certified enclosures at
/// `Interval` and from the operands alone, by the general quadratic
/// `At² + 2Bt + C` of a ruling `p + t·a`, roots `(−B ± √(B² − AC))/A`:
/// against a cylinder's axis `(o′, a′)` of radius `r`, with `w = p − o′`,
/// `A = |a × a′|²`, `B = (w × a′)·(a × a′)`, `C = |w × a′|² − r²`; against
/// a sphere about `c` of radius `R`, with `w = p − c`, `A = a·a`,
/// `B = w·a`, `C = w·w − R²`; against a cone from `k` about `d` of
/// half-angle `α`, with `w = p − k`, `A = (d·a)² − cos²α|a|²`,
/// `B = (d·w)(d·a) − cos²α w·a`, `C = (d·w)² − cos²α|w|²`. A cone's
/// rulings run from its apex along its generators, `t` over both nappes,
/// and a root whose divisor reaches zero is read by the other form, or
/// past the ball by its floor. Neither the sweep's frame nor its closed
/// form enters it, so a defect in either does not hide itself here: it
/// is independent of the sweep in its algebra. It is not independent in
/// its enclosure arithmetic, which it shares with the sweep
/// ([`Cell::new`], [`Iv`], [`V3`]). Either of two cylinders may be ruled:
/// every section point lies on exactly one of its rulings. The cone side
/// has no caller yet: the join's reading of the cone pairs gives it its
/// first (`work/sectcone/the-join-reads-the-cone-pairs-on-the-value.md`).
///
/// # Errors
///
/// The first cell it could not cover within [`COVERAGE_DEPTH`] halvings
/// and [`COVERAGE_CELLS`] cells, `[lo, hi]`; the whole turn for a pair it
/// does not read.
pub(crate) fn coverage(
    ruled: &geom::Surface<f64>,
    partner: &geom::Surface<f64>,
    ball: ExtentBall<f64>,
    tubes: &[crate::ssi::enclose::Box3],
) -> Result<Coverage, [f64; 2]> {
    use crate::ssi::enclose::Box3;
    use geom_core::{Bounds as _, Interval, SpanLocate as _};
    let unread = [0.0, core::f64::consts::TAU];
    let lift = Interval::from_computed;
    let p3 = |p: Point3<f64>| Point3::new(lift(p.x), lift(p.y), lift(p.z));
    let v3 = |v: Vec3<f64>| Vec3::new(lift(v.x), lift(v.y), lift(v.z));
    let along = |k: Vec3<Interval>| V3::along(k, Iv::point(Interval::one()));
    // The rulings over a cell, `p + t·a`: a cylinder's round its wall
    // along its axis, from its own frame squared from its seam
    // reference; a cone's from its apex along its generators, which turn.
    type Rulings = Box<dyn Fn(&Cell<Interval>) -> (V3<Interval>, V3<Interval>)>;
    let rulings: Rulings = match *ruled {
        geom::Surface::Cylinder {
            frame: carrier_frame,
            radius: rho,
        } => {
            let Some(frame) = ruled.frame().map(|f| f.square()) else {
                return Err(unread);
            };
            let (u, v) = (frame.u_ref, frame.axis.cross(frame.u_ref));
            let (o, a, u, v) = (
                p3(carrier_frame.origin()),
                v3(carrier_frame.axis()),
                v3(u),
                v3(v),
            );
            let rho = lift(rho);
            Box::new(move |cell| {
                let p = V3::at(o)
                    .plus(V3::along(u, cell.cos.scale(rho)))
                    .plus(V3::along(v, cell.sin.scale(rho)));
                (p, along(a))
            })
        }
        geom::Surface::Cone { frame, half_angle } => {
            let Some(square) = ruled.frame().map(|f| f.square()) else {
                return Err(unread);
            };
            let (axis, n) = (square.axis, square.u_ref);
            let (apex, axis, n, m) = (p3(frame.origin()), v3(axis), v3(n), v3(axis.cross(n)));
            let (s, c) = lift(half_angle).sin_cos();
            Box::new(move |cell| {
                let a = along(axis * c)
                    .plus(V3::along(n, cell.cos.scale(s)))
                    .plus(V3::along(m, cell.sin.scale(s)));
                (V3::at(apex), a)
            })
        }
        _ => return Err(unread),
    };
    // `(A, B, C)` of the rulings through the enclosure `p` along `a`.
    type Quadratic = Box<dyn Fn(V3<Interval>, V3<Interval>) -> [Iv<Interval>; 3]>;
    let quadratic: Quadratic = match *partner {
        geom::Surface::Cylinder {
            frame, radius: r, ..
        } => {
            let (q, b, r) = (p3(frame.origin()), v3(frame.axis()), lift(r));
            Box::new(move |p, a| {
                let (wb, ab) = (p.less(q).cross(b), a.cross(b));
                let qc = wb.dot(wb).shift(Interval::zero() - r.powi(2));
                [ab.dot(ab), wb.dot(ab), qc]
            })
        }
        geom::Surface::Sphere {
            frame,
            radius: big_r,
            ..
        } => {
            let (c, big_r) = (p3(frame.origin()), lift(big_r));
            Box::new(move |p, a| {
                let w = p.less(c);
                let qc = w.dot(w).shift(Interval::zero() - big_r.powi(2));
                [a.dot(a), w.dot(a), qc]
            })
        }
        // `q = (d·w)² − cos²α|w|²`.
        geom::Surface::Cone { frame, half_angle } => {
            let (k, d) = (p3(frame.origin()), v3(frame.axis()));
            let c2 = lift(half_angle).cos().powi(2);
            Box::new(move |p, a| {
                let w = p.less(k);
                let (dw, da) = (w.toward(d), a.toward(d));
                let form = |x: Iv<Interval>, y: Iv<Interval>, xy: Iv<Interval>| {
                    x.times(y).plus(xy.scale(Interval::zero() - c2))
                };
                [
                    form(da, da, a.dot(a)),
                    form(dw, da, w.dot(a)),
                    form(dw, dw, w.dot(w)),
                ]
            })
        }
        _ => return Err(unread),
    };
    let hull = |i: Iv<Interval>| i.lo.enclosure_hull(i.hi);
    let (centre, radius) = (ball.center(), ball.radius());
    // `distance > reach` on `f64` readings each held a relative 1e-12 to
    // the safe side, far past the few roundings between them.
    let past = |distance: f64, reach: f64| distance * (1.0 - 1e-12) > reach * (1.0 + 1e-12);
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
        past((gx.powi(2) + gy.powi(2) + gz.powi(2)).sqrt(), radius)
    };
    let mut hint = 0usize;
    let mut read = Coverage { cells: 0, depth: 0 };
    #[allow(clippy::cast_precision_loss)]
    let cut = |k: i32| core::f64::consts::TAU * f64::from(k) / f64::from(COVERAGE_START);
    // The last cut first, as the walk has always read them.
    let roots = (0..COVERAGE_START)
        .rev()
        .map(|k| (cut(k), cut(k + 1), 0u32));
    // `COVERAGE_CELLS` is a wall in cells read, but it stays in the visit
    // (`read.cells`) rather than being the walk's `Budget`: its refusal
    // names the cell it stopped in (`[lo, hi]`), which an `Exhausted`
    // cannot. The walk halves only while it has cells left, so the reads
    // that implies never bind first.
    let mut reads = Budget::implied_by_halvings(COVERAGE_START as usize, COVERAGE_CELLS);
    let walked = bisect_within(roots, &mut reads, |(lo, hi, depth)| {
        read.cells += 1;
        read.depth = read.depth.max(depth);
        let cell = Cell::<Interval>::new(lo, hi);
        let (p, a) = rulings(&cell);
        let [big_a, qb, qc] = quadratic(p, a);
        let disc = qb.square().plus(qc.times(big_a).neg());
        // Off zero: the enclosure is certified and excludes it.
        let apart = |i: Iv<Interval>| {
            let h = hull(i);
            h.is_certified() && (h.lo() > 0.0 || h.hi() < 0.0)
        };
        let covered = hull(disc).is_certified() && hull(disc).hi() < 0.0
            || [Interval::one(), Interval::zero() - Interval::one()]
                .into_iter()
                .all(|sign| {
                    // Each root by the form whose divisor stands off zero:
                    // `(−B + σ√D)/A`, or `C/(−B − σ√D)` where `A` reaches
                    // zero (a ruling along an asymptotic direction). Where
                    // neither does, the root stands at least
                    // `|−B + σ√D|/|A|` along the ruling, and is clear where
                    // that puts it past the ball.
                    let root = disc.sqrt().scale(sign);
                    let (wide, narrow) = (qb.neg().plus(root), qb.neg().plus(root.neg()));
                    let t = if apart(big_a) {
                        wide.quotient(big_a)
                    } else if apart(narrow) {
                        qc.quotient(narrow)
                    } else {
                        let (n, d) = (hull(wide.abs()).lo(), hull(big_a.abs()).hi());
                        let reach = hull(p.less(ball.center().map(lift)).norm()).hi();
                        let speed = hull(a.norm()).lo();
                        return past(n / d * speed, reach + radius);
                    };
                    let x = p.plus(a.times(t));
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
            return Step::Settled;
        }
        if depth >= COVERAGE_DEPTH || read.cells >= COVERAGE_CELLS {
            return Step::Stop([lo, hi]);
        }
        let mid = geom::mid_param(lo, hi);
        Step::Halve([(lo, mid, depth + 1), (mid, hi, depth + 1)])
    });
    match walked {
        Ok(None) => Ok(read),
        Ok(Some(cell)) => Err(cell),
        Err(Exhausted) => unreachable!("a coverage walk spends its cells before its reads"),
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::{Cell, Iv, Ruled, RuledCylinders, SweepRefusal, V3, sweep};
    use crate::pair_section::{CcStance, CylinderCylinderPose, CylinderSpherePose};
    use crate::{ExtentBall, Reach};
    use geom::Surface;
    use geom_core::{Band, Point3, Tol, Vec3};

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    fn wall(origin: (f64, f64, f64), axis: Vec3<f64>, radius: f64) -> Surface<f64> {
        let axis = axis.normalize();
        Surface::Cylinder {
            frame: geom::Frame::new(
                Point3::new(origin.0, origin.1, origin.2),
                axis,
                axis.orthonormal_basis().0,
            ),
            radius,
        }
    }

    fn ruled(a: &Surface<f64>, b: &Surface<f64>) -> (RuledCylinders<f64>, f64) {
        let region = ExtentBall::fixture(Point3::origin(), 3.0);
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

    /// **A discriminant in the band over the whole turn escalates**: no
    /// first cut decides, and the first is kept anyway, so the turn is read
    /// as one cell, split to the budget. A sweep that dropped every first
    /// cut would read the turn over no cell.
    #[test]
    fn a_discriminant_in_the_band_over_the_whole_turn_escalates() {
        struct Flat;
        impl Ruled<f64> for Flat {
            fn lever(&self) -> geom_core::Lever<f64> {
                geom_core::Lever::curvature_radius(1.0)
            }
            fn discriminant(&self, _: &Cell<f64>) -> Iv<f64> {
                Iv::point(0.0)
            }
            fn discriminant_slope(&self, _: &Cell<f64>) -> Iv<f64> {
                Iv::point(0.0)
            }
            fn meeting(&self, _: &Cell<f64>, _: super::Sheet) -> V3<f64> {
                V3::at(Point3::origin())
            }
            fn velocity(&self, _: &Cell<f64>, _: super::Sheet) -> V3<f64> {
                V3::at(Point3::origin())
            }
        }
        let got = sweep(&Flat, ExtentBall::fixture(Point3::origin(), 1.0), band());
        assert!(
            matches!(got, Err(SweepRefusal::Escalated(_))),
            "a turn in the band escalates: {got:?}"
        );
    }

    /// **The coverage reads a sphere's meetings**: two rings of the wall
    /// `r = 1/2` about `z` against a sphere at `(0.1, 0, 0.2)` of radius
    /// `0.7`, each enclosed by boxes about points of its closed form
    /// (`h² = R² − r² − d² + 2rd·cos θ` over the centre's height), are
    /// covered, and with either ring's boxes dropped a cell is not. The
    /// sweep's march is not read, so the default set holds the coverage's
    /// sphere arm to its quadratic without the slow set's carries.
    #[test]
    fn the_coverage_reads_a_spheres_meetings() {
        use crate::ssi::enclose::Box3;
        let (r, d, big) = (0.5f64, 0.1f64, 0.7f64);
        let c = wall((0.0, 0.0, 0.0), Vec3::unit_z(), r);
        let s = Surface::Sphere {
            frame: geom::Frame::new(Point3::new(d, 0.0, 0.2), Vec3::unit_z(), Vec3::unit_x()),
            radius: big,
        };
        let ring = |sheet: f64| -> Vec<Box3> {
            (0..2048)
                .map(|k| {
                    let theta = core::f64::consts::TAU * f64::from(k) / 2048.0;
                    let h = (big * big - r * r - d * d + 2.0 * r * d * theta.cos()).sqrt();
                    let (sn, cs) = theta.sin_cos();
                    Box3::around(Point3::new(r * cs, r * sn, 0.2 + sheet * h), 0.01)
                })
                .collect()
        };
        let (upper, lower) = (ring(1.0), ring(-1.0));
        let both: Vec<Box3> = upper.iter().chain(&lower).copied().collect();
        let ball = ExtentBall::fixture(Point3::new(d, 0.0, 0.2), big);
        assert!(
            super::coverage(&c, &s, ball, &both).is_ok(),
            "both rings' boxes cover every cell"
        );
        for (which, kept) in [("upper", &lower), ("lower", &upper)] {
            assert!(
                super::coverage(&c, &s, ball, kept).is_err(),
                "the {which} ring's boxes dropped: a cell uncovered"
            );
        }
    }

    /// **A sphere's `Δ` reaches the pose's margins at its extremes**: the
    /// margin read over a fine turn reaches the near reach `R − |d − r|`
    /// at its most and the far reach `R − (d + r)` at its least, the two
    /// margins the pose decides the count on, for a sphere crossing the
    /// wall in one loop, in two, centred on the wall, missing it, inside
    /// it, and on a turned axis, in both orders; and the slope is of the
    /// sign of `Δ`'s change across each cell of the turn.
    #[test]
    fn a_spheres_discriminant_extremes_are_the_poses_reaches() {
        let sphere = |c: (f64, f64, f64), radius: f64| Surface::Sphere {
            frame: geom::Frame::new(Point3::new(c.0, c.1, c.2), Vec3::unit_z(), Vec3::unit_x()),
            radius,
        };
        let turned = Vec3::new(0.3, -0.2, 1.0);
        let poses = [
            (
                "one loop",
                wall((0.0, 0.0, 0.0), Vec3::unit_z(), 0.5),
                sphere((0.6, 0.0, 0.2), 0.3),
            ),
            (
                "two loops",
                wall((0.0, 0.0, 0.0), Vec3::unit_z(), 0.5),
                sphere((0.1, 0.0, 0.2), 0.7),
            ),
            (
                "centred on the wall",
                wall((0.0, 0.0, 0.0), Vec3::unit_z(), 0.5),
                sphere((0.0, 0.5, -0.1), 0.4),
            ),
            (
                "missing it",
                wall((0.0, 0.0, 0.0), Vec3::unit_z(), 0.5),
                sphere((1.2, 0.3, 0.0), 0.4),
            ),
            (
                "inside it",
                wall((0.0, 0.0, 0.0), Vec3::unit_z(), 0.5),
                sphere((0.1, 0.1, 0.0), 0.2),
            ),
            (
                "turned, one loop",
                wall((0.1, -0.2, 0.3), turned, 0.4),
                sphere((0.6, 0.1, 0.5), 0.35),
            ),
            (
                "turned, two loops",
                wall((0.1, -0.2, 0.3), turned, 0.4),
                sphere((0.2, -0.1, 0.0), 0.9),
            ),
        ];
        for (what, c, s) in poses {
            for (order, x, y) in [("cylinder, sphere", &c, &s), ("sphere, cylinder", &s, &c)] {
                let label = format!("{what} ({order})");
                let pose = CylinderSpherePose::decide(x, y, band())
                    .expect("a cylinder and a sphere")
                    .unwrap_or_else(|e| panic!("{label}: a decided pose: {e:?}"));
                let ruled = pose.ruled().expect("an off-axis centre");
                let ((r, big_r), d) = (pose.radii(), pose.offset());
                let (mut most, mut least) = (f64::NEG_INFINITY, f64::INFINITY);
                let n = 4096;
                let at = |k: i32| core::f64::consts::TAU * f64::from(k) / f64::from(n);
                for k in 0..=n {
                    let m = ruled.discriminant(&Cell::at(at(k)));
                    most = most.max(m.hi);
                    least = least.min(m.lo);
                }
                for k in 0..n {
                    let rise = ruled.discriminant(&Cell::at(at(k + 1))).lo
                        - ruled.discriminant(&Cell::at(at(k))).lo;
                    let slope = ruled.discriminant_slope(&Cell::new(at(k), at(k + 1)));
                    assert!(
                        (rise <= 0.0 || slope.hi > 0.0) && (rise >= 0.0 || slope.lo < 0.0),
                        "{label}: cell {k}: `Δ` moves {rise} across a slope of {slope:?}"
                    );
                }
                assert!(
                    (most - (big_r - (d - r).abs())).abs() <= 1e-12,
                    "{label}: the most is the near reach, {most} against {}",
                    big_r - (d - r).abs()
                );
                assert!(
                    (least - (big_r - (d + r))).abs() <= 1e-12,
                    "{label}: the least is the far reach, {least} against {}",
                    big_r - (d + r)
                );
            }
        }
    }

    /// **A loop is witnessed where its sheets stand farthest apart**: the
    /// march seeds at the witness and its fit ends there, so a witness at
    /// the far ruling puts both ends of the fit on the loop's sharpest
    /// turn when the loop runs close to the node. For one loop and two,
    /// near the node and clear of it, on a turned wall, in both orders,
    /// every witness's ruling angle lies within a first cell of the near
    /// ruling, `φ = 0`.
    #[test]
    fn a_loop_is_witnessed_at_the_near_ruling() {
        let sphere = |c: (f64, f64, f64), radius: f64| Surface::Sphere {
            frame: geom::Frame::new(Point3::new(c.0, c.1, c.2), Vec3::unit_z(), Vec3::unit_x()),
            radius,
        };
        let upright = wall((0.0, 0.0, 0.0), Vec3::unit_z(), 0.5);
        let turned = wall((0.1, -0.2, 0.3), Vec3::new(0.3, -0.2, 1.0), 0.4);
        let poses = [
            ("one loop", &upright, sphere((0.6, 0.0, 0.2), 0.3), 1),
            (
                "one loop near the node",
                &upright,
                sphere((0.2, 0.0, 0.0), 0.699),
                1,
            ),
            ("two loops", &upright, sphere((0.1, 0.0, 0.2), 0.7), 2),
            (
                "two loops near the node",
                &upright,
                sphere((0.2, 0.0, 0.0), 0.701),
                2,
            ),
            (
                "turned, one loop",
                &turned,
                sphere((0.6, 0.1, 0.5), 0.35),
                1,
            ),
            (
                "turned, two loops",
                &turned,
                sphere((0.2, -0.1, 0.0), 0.9),
                2,
            ),
        ];
        #[allow(clippy::cast_precision_loss)]
        let first_cell = core::f64::consts::TAU / super::START as f64;
        for (what, c, s, loops) in poses {
            for (order, x, y) in [("cylinder, sphere", c, &s), ("sphere, cylinder", &s, c)] {
                let label = format!("{what} ({order})");
                let pose = CylinderSpherePose::decide(x, y, band())
                    .expect("a cylinder and a sphere")
                    .unwrap_or_else(|e| panic!("{label}: a decided pose: {e:?}"));
                let ruled = pose.ruled().expect("an off-axis centre");
                let swept = sweep(&ruled, pose.sweep_ball(band()), band())
                    .unwrap_or_else(|e| panic!("{label}: the sweep: {e:?}"));
                assert_eq!(swept.components.len(), loops, "{label}: its loops");
                for c in &swept.components {
                    let w = c.witness - ruled.foot;
                    let phi = w.dot(ruled.v).atan2(w.dot(ruled.n));
                    assert!(
                        phi.abs() <= first_cell,
                        "{label}: witnessed at φ {phi}, past {first_cell} from the near ruling"
                    );
                }
            }
        }
    }

    /// **The clearance ceiling holds only where its translation meets
    /// the partner**: the unit cone's ruling along `(0, 1, 1)/√2` against
    /// three rods along `x` about `y = 0, z = 1` and one far off.
    /// Grazing (radius `1/√2`, the ruling tangent at `(0, 1/2, 1/2)`) the
    /// ceiling is the touch's own, within rounding; clear by `≈ 0.107`
    /// (radius `0.6`, `|g|² = 2 ≥ 4k|q*| = 1.12`) it is a ceiling, at or
    /// above that clearance (`2|q*|/|g| ≈ 0.198`); a thin rod `5` off
    /// (`|g|²` short of `4k|q*|`) reads no ceiling at all. A ceiling read
    /// past the condition, or below the clearance: red.
    #[test]
    fn the_clearance_ceiling_holds_only_where_its_condition_does() {
        use super::{Form, RuledCone, WIDE};
        use core::f64::consts::{FRAC_1_SQRT_2, FRAC_PI_2, FRAC_PI_4};
        let cone = |origin: (f64, f64, f64), r: f64| RuledCone {
            apex: Point3::origin(),
            axis: Vec3::unit_z(),
            n: Vec3::unit_x(),
            m: Vec3::unit_y(),
            sc: FRAC_PI_4.sin_cos(),
            partner: Form::cylinder(
                geom::Line {
                    origin: Point3::new(origin.0, origin.1, origin.2),
                    dir: geom_core::UnitVec3::exact_x(),
                },
                r,
            ),
            lever: ExtentBall::fixture(Point3::origin(), 10.0).lever(),
        };
        let at = Cell::at(FRAC_PI_2);
        let graze = cone((0.0, 0.0, 1.0), FRAC_1_SQRT_2).form_ceiling(&at);
        assert!(
            graze < 1e-12,
            "the graze's ceiling is its rounding: {graze:e}"
        );
        let clear = cone((0.0, 0.0, 1.0), 0.6).form_ceiling(&at);
        let clearance = FRAC_1_SQRT_2 - 0.6;
        assert!(
            clear >= clearance && clear < 0.2,
            "a ceiling at or above the clearance {clearance}: {clear}"
        );
        let far = cone((0.0, 5.0, 1.0), 0.1).form_ceiling(&Cell::at(-FRAC_PI_2));
        assert!(far >= WIDE, "no ceiling past the condition: {far:e}");
        // Over a cell about the graze, the ceiling bounds every ruling's
        // clearance in it, the edges' too: `r − δ`, `δ` the ruling's
        // distance from the rod's axis. The mean value's `|E′|·w/2` is what
        // reaches them; dropped, the ceiling is the middle's rounding.
        let eps = 0.02;
        let wide = cone((0.0, 0.0, 1.0), FRAC_1_SQRT_2)
            .form_ceiling(&Cell::new(FRAC_PI_2 - eps, FRAC_PI_2 + eps));
        let (s, c) = FRAC_PI_4.sin_cos();
        let (vy, vz) = (s * eps.cos(), c);
        let edge = FRAC_1_SQRT_2 - vy / (vy * vy + vz * vz).sqrt();
        assert!(
            edge > 0.0 && wide >= edge,
            "a wide cell's ceiling {wide:e} reaches its edge's clearance {edge:e}"
        );
    }

    /// **A branch cell split in the walk keeps the loop's order**: the rod
    /// through the cone (about `y = 0, z = 1` along `x`, radius `0.2`),
    /// whose arc loops wind about the rod. Each branch piece is split until
    /// it sheds a positive half: the rise's at the walk's start, so its
    /// minus half goes to the walk's end, and the fall's in the middle.
    /// The plus half must follow a rise and precede a fall, the minus half
    /// the other way round, and the split walk must read the winding the
    /// unsplit one does. Swapping the halves either side of the branch, or
    /// inserting the rise's minus half at the front: red.
    #[test]
    fn a_split_branch_cell_keeps_the_walks_order() {
        use super::{Arc, Form, Piece, RuledCone, Sheet, Sweeper, arcs};
        use core::f64::consts::FRAC_PI_4;
        let cone = RuledCone {
            apex: Point3::origin(),
            axis: Vec3::unit_z(),
            n: Vec3::unit_x(),
            m: Vec3::unit_y(),
            sc: FRAC_PI_4.sin_cos(),
            partner: Form::cylinder(
                geom::Line {
                    origin: Point3::new(0.0, 0.0, 1.0),
                    dir: geom_core::UnitVec3::exact_x(),
                },
                0.2,
            ),
            lever: ExtentBall::fixture(Point3::origin(), 5.0).lever(),
        };
        let about = geom::Frame::new(Point3::new(0.0, 0.0, 1.0), Vec3::unit_x(), Vec3::unit_y());
        let ball = ExtentBall::fixture(Point3::origin(), 5.0);
        let mut sweeper = Sweeper::new(&cone, ball, band(), Some(about));
        let cells = sweeper.tiled().unwrap();
        let loops: Vec<Arc<f64>> = arcs(cells).unwrap();
        assert_eq!(loops.len(), 2, "two arcs of the discriminant");
        let is_sheet = |p: &Piece<f64>, want: Sheet, back: bool| matches!(*p, Piece::Sheet { sheet, backward, .. } if sheet == want && backward == back);
        for arc in &loops {
            let whole = Piece::arc(arc);
            assert_eq!(
                sweeper.winds_about(&about, whole.clone()),
                Ok(true),
                "round the rod"
            );
            // The rise, at the walk's start.
            let mut walk = whole.clone();
            let n = walk.len();
            while walk.len() == n {
                sweeper
                    .split_piece(&mut walk, 0, "the test's split")
                    .unwrap();
            }
            assert!(matches!(walk[0], Piece::Branch { rising: true, .. }));
            assert!(
                is_sheet(&walk[1], Sheet::Plus, false),
                "the rise's plus half after it"
            );
            assert!(
                is_sheet(walk.last().unwrap(), Sheet::Minus, true),
                "the rise's minus half at the walk's end, before the rise"
            );
            assert_eq!(
                walk[1].cell().lo,
                walk.last().unwrap().cell().lo,
                "one half, both sheets"
            );
            assert_eq!(
                sweeper.winds_about(&about, walk.clone()),
                Ok(true),
                "the split rise"
            );
            // The fall, in the middle.
            let fall = walk
                .iter()
                .position(|p| matches!(p, Piece::Branch { rising: false, .. }))
                .unwrap();
            let n = walk.len();
            let mut k = fall;
            while walk.len() == n {
                sweeper
                    .split_piece(&mut walk, k, "the test's split")
                    .unwrap();
                k = walk
                    .iter()
                    .position(|p| matches!(p, Piece::Branch { rising: false, .. }))
                    .unwrap();
            }
            assert!(
                is_sheet(&walk[k - 1], Sheet::Plus, false),
                "the fall's plus half before it"
            );
            assert!(
                is_sheet(&walk[k + 1], Sheet::Minus, true),
                "the fall's minus half after it"
            );
            assert_eq!(
                sweeper.winds_about(&about, walk),
                Ok(true),
                "the split fall"
            );
        }
    }
}
