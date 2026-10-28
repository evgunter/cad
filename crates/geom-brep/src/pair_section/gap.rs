//! **The critical gap of a torus pair** (module docs of
//! [`super`], the torus poses): whether the two surfaces are tangent,
//! read on their spines by a certified sweep of the torus's core angle.
//!
//! # Offset duality
//!
//! A sphere, a cylinder and a torus are each the points at distance `ρ`
//! from a spine: a point, a line, a core circle. Two such surfaces are
//! tangent exactly where a critical value of the distance between a
//! point of one spine and a point of the other equals `ρ₁ + ρ₂` or
//! `|ρ₁ − ρ₂|`: at a common point the normal of each passes through its
//! spine's nearest point, so the two feet and the point are collinear,
//! which is the critical-point condition of the spine distance. The gap
//! is how far the critical values stand from those two levels.
//!
//! The first spine is the torus's core circle, swept by its angle `φ`.
//! At each point `p(φ)` the spine distance is critical along the second
//! spine at its foot (a point, the line's foot, or the circle's nearest
//! and farthest points in `p`'s meridian half-plane), so its critical
//! values are those of the one-variable functions `D(φ) = |p(φ) − q(φ)|`,
//! one per branch of the foot, at their critical points. The circle's
//! feet are unique off its axis; where the core crosses that axis every
//! point of the circle is a foot, `D` has a kink, and a critical value can
//! sit on the kink itself.
//!
//! # The sweep
//!
//! The turn is cut into cells. On a cell of half-width `w` about `m`,
//! `D` moves by at most `R·w` (the spine distance is 1-Lipschitz in
//! space and `p` moves at the core's speed `R`, kinks included). The
//! squared distance `G = D²` is smooth off the kinks (a point's and a
//! line's are trigonometric polynomials, a circle's `|w|² ∓ 2R₂ρ + R₂²`
//! with `ρ` the distance from its axis), so its slope over the cell is
//! bounded by Taylor's theorem: `G′(m)` and `G″(m)` read at the point,
//! and a bound on `|G‴|` over the cell from `|p′| = |p″| = |p‴| = R`
//! (and, for the circle, `ρ` bounded below over the cell, the kink's
//! distance). A cell clears when every branch either
//!
//! - stands past the band from both levels over the whole cell
//!   (`|D(m) − L| − R·w` definitely positive, or, where `G` is smooth,
//!   the range of `D` its Taylor bound gives clear of both levels), or
//! - is smooth on it and monotone (`|G′(m)| − |G″(m)|·w − |G‴|·w²/2`
//!   positive, read as the least `|D′|` it bounds), so it holds no
//!   critical point: `G′ = 2D·D′` vanishes at every critical point of the
//!   spine distance, its zeros included.
//!
//! Every cell clearing is a certified [`Gap::Transverse`]: a critical
//! point of the spine distance lies in some cell, where neither test can
//! pass unless its value stands past the band.
//!
//! [`Gap::Tangent`] is decided two ways, each a critical value within
//! the zero band of a level:
//!
//! - **a turn**: on a smooth window `D′` reads definitely negative at one
//!   end and definitely positive at the other (or the reverse), so a
//!   critical point lies inside, and the window's bound on `|D − L|`,
//!   `G`'s Taylor bound about the turn read back through `√`, is decided
//!   `Zero`. The window is centred where `G′` changes sign, found by
//!   bisection across the open cell padded by half its width each side
//!   (a turn on a cut lies inside both neighbours' reach), and shrunk by
//!   halves until its bound fits the band;
//! - **the least distance**: the spine distance's global minimum is
//!   always a critical value, and no larger than `D` anywhere, so `D(m)`
//!   and the level `|ρ₁ − ρ₂|` both within the zero band decide it. This
//!   is the tube-circle tangency (equal minor radii, cores touching),
//!   where `D′` reads no sign.
//!
//! Any other cell is split, down to a floor where it stands within a
//! quarter band of the core, or until the cell budget is spent: there it
//! escalates on its reading of the gap ([`floor_refusal`]). A Zero found
//! anywhere outranks an escalation found elsewhere; the walk is finished
//! before it escalates.

use geom_core::k_stats::{decide, decide_reported, gate_rejection, straddle};
use geom_core::{
    Band, Decide, Decided, Indeterminate, Margin, Point3, Real, Sign, SizedPass, Vec3,
};

/// The cells the first cut makes of the turn.
const GAP_START: usize = 32;
/// The most cells one sweep reads before it escalates.
const GAP_CELLS: usize = 1 << 14;
/// The bisection steps that place a sign change of `D′` in its cell.
const GAP_BISECTIONS: u32 = 64;
/// The window sizes a turn is tried at, each half the last.
const GAP_WINDOWS: u32 = 32;

/// **A torus's core circle**, swept by its angle: `center + R·(cos φ·u +
/// sin φ·v)`, `u` and `v` unit and square to each other and to the axis.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Core<T: Real> {
    pub(crate) center: Point3<T>,
    pub(crate) u: Vec3<T>,
    pub(crate) v: Vec3<T>,
    pub(crate) radius: T,
}

impl<T: Real> Core<T> {
    /// The core of the torus `(center, axis, R)` whose seam is `u_ref`.
    pub(crate) fn of(center: Point3<T>, axis: Vec3<T>, u_ref: Vec3<T>, radius: T) -> Self {
        let frame = geom::square_frame(axis, u_ref);
        Self {
            center,
            u: frame.u_ref,
            v: frame.axis.cross(frame.u_ref),
            radius,
        }
    }

    pub(crate) fn point(self, phi: T) -> Point3<T> {
        let (s, c) = phi.sin_cos();
        self.center + (self.u * c + self.v * s) * self.radius
    }

    fn velocity(self, phi: T) -> Vec3<T> {
        let (s, c) = phi.sin_cos();
        (self.v * c - self.u * s) * self.radius
    }
}

/// **The partner's spine**: the set the partner stands at a constant
/// distance from. Axes are unit.
#[derive(Clone, Copy, Debug)]
pub(crate) enum Spine<T: Real> {
    /// A sphere's centre.
    Point(Point3<T>),
    /// A cylinder's axis.
    Line { origin: Point3<T>, axis: Vec3<T> },
    /// A torus's core circle.
    Circle {
        center: Point3<T>,
        axis: Vec3<T>,
        radius: T,
    },
}

/// The rows a sweep decides on, each a pair's own name.
#[derive(Clone, Copy, Debug)]
pub(crate) struct GapRows {
    /// The gap itself: a Zero verdict, and the floor's escalation.
    pub(crate) gap: &'static str,
    /// A cell's branch stands past the band from both levels.
    pub(crate) clear: &'static str,
    /// A cell's branch is monotone.
    pub(crate) monotone: &'static str,
    /// A cell's branch is smooth: off the spine and off the kinks.
    pub(crate) smooth: &'static str,
    /// The sign of `D′` at a cell's or a window's end.
    pub(crate) turn: &'static str,
    /// A cell at the floor.
    pub(crate) floor: &'static str,
    /// The floor's two bounds on the gap straddling the band.
    pub(crate) straddle: &'static str,
}

/// **What the gap decided.**
#[derive(Clone, Copy, Debug)]
pub enum Gap<T: Real> {
    /// Every critical value stands past the band from both levels.
    Transverse,
    /// A critical value stands within the zero band of a level: the
    /// surfaces are tangent.
    Tangent {
        /// The verdict's margin.
        margin: geom_core::MarginDiag,
        /// The core point the critical value was read at.
        at: Point3<T>,
    },
}

/// One branch of the spine distance read at one angle: `D`, and the
/// squared distance `G = D²`'s first two derivatives there, in metres
/// squared per radian.
#[derive(Clone, Copy)]
struct Reading<T: Real> {
    d: T,
    g1: T,
    g2: T,
    /// `|p − c|` for the point and the circle, `D` for the line: what
    /// `G‴`'s bound reads.
    reach: T,
    /// The core point's distance from the partner circle's axis.
    kink: Option<T>,
}

/// `⟨Πw, Πw⟩`'s first two derivatives along the core, `w = p − o` and
/// `Π` the projection square to `axis` (none for a point): `(S, S′, S″)`.
fn squared<T: Real>(w: Vec3<T>, dp: Vec3<T>, ddp: Vec3<T>, axis: Option<Vec3<T>>) -> [T; 3] {
    let flat = |v: Vec3<T>| match axis {
        Some(a) => a.cross(v.cross(a)),
        None => v,
    };
    let (w, dp, ddp) = (flat(w), flat(dp), flat(ddp));
    let two = T::from_f64(2.0);
    [w.dot(w), two * w.dot(dp), two * (dp.dot(dp) + w.dot(ddp))]
}

fn read<T: Real>(core: Core<T>, spine: Spine<T>, far: bool, phi: T) -> Reading<T> {
    let (p, dp) = (core.point(phi), core.velocity(phi));
    let ddp = core.center - p;
    match spine {
        Spine::Point(c) => {
            let [g, g1, g2] = squared(p - c, dp, ddp, None);
            let d = g.sqrt();
            Reading {
                d,
                g1,
                g2,
                reach: d,
                kink: None,
            }
        }
        Spine::Line { origin, axis } => {
            let [g, g1, g2] = squared(p - origin, dp, ddp, Some(axis));
            let d = g.sqrt();
            Reading {
                d,
                g1,
                g2,
                reach: d,
                kink: None,
            }
        }
        Spine::Circle {
            center,
            axis,
            radius,
        } => {
            // `G = |w|² ∓ 2R₂ρ + R₂²`, `ρ = √S` the distance from the
            // circle's axis: `ρ′ = S′/2ρ`, `ρ″ = S″/2ρ − S′²/4ρ³`.
            let w = p - center;
            let [n, n1, n2] = squared(w, dp, ddp, None);
            let [s0, s1, s2] = squared(w, dp, ddp, Some(axis));
            let rho = s0.sqrt();
            let two = T::from_f64(2.0);
            let r1 = s1 / (two * rho);
            let r2 = s2 / (two * rho) - s1.powi(2) / (T::from_f64(4.0) * rho.powi(3));
            let sigma = if far { T::one() } else { -T::one() };
            let h = w.dot(axis);
            let across = rho + sigma * radius;
            Reading {
                d: (across.powi(2) + h.powi(2)).sqrt(),
                g1: n1 + two * sigma * radius * r1,
                g2: n2 + two * sigma * radius * r2,
                reach: n.sqrt(),
                kink: Some(rho),
            }
        }
    }
}

/// The least of `|D − L|` over the two levels.
fn nearest<T: Real>(d: T, levels: [T; 2]) -> T {
    (d - levels[0]).abs().min((d - levels[1]).abs())
}

/// One sweep's frame: what it reads and at which band.
struct Sweep<'a, T: Real> {
    core: Core<T>,
    spine: Spine<T>,
    levels: [T; 2],
    rows: &'a GapRows,
    band: Band,
}

/// What one cell's branch reads.
enum Cell<T: Real> {
    Clear,
    Tangent(Gap<T>),
    Open(Reading<T>),
}

impl<T: Decide> Sweep<'_, T> {
    fn positive(&self, name: &'static str, m: Margin<T>) -> bool {
        matches!(decide(name, m, self.band), Ok(Sign::Positive))
    }

    /// **`G‴`'s bound over a window `reach` metres of core about a
    /// reading**, where `G` is smooth there: `None` where the window may
    /// hold a kink. With `|p′| = |p″| = |p‴| = R` and `p′·p″ = 0`,
    /// `⟨Πw, Πw⟩‴ = 2(3⟨Πp′, Πp″⟩ + ⟨Πw, Πp‴⟩)` is at most
    /// `2R(3R + |Πw|)`, and for the circle `ρ‴`, from `ρ = √S`, at most
    /// `(3R² + ρR)/ρ + 3R(R² + ρR)/ρ² + 3R³/ρ²` over `ρ ≥ ρ_lo`.
    fn third(&self, r: Reading<T>, reach: T) -> Option<T> {
        let big = self.core.radius;
        let (two, three) = (T::from_f64(2.0), T::from_f64(3.0));
        let far = r.reach + reach;
        match (self.spine, r.kink) {
            (Spine::Circle { radius, .. }, Some(rho)) => {
                if !self.positive(self.rows.smooth, Margin::of(rho - reach)) {
                    return None;
                }
                let lo = rho - reach;
                let rho3 = (three * big.powi(2) + lo * big) / lo
                    + three * big * (big.powi(2) + lo * big) / lo.powi(2)
                    + three * big.powi(3) / lo.powi(2);
                // `|w|²‴ = 2w·p‴`, the other terms vanishing on a circle.
                Some(two * far * big + two * radius * rho3)
            }
            _ => Some(two * big * (three * big + far)),
        }
    }

    /// The sign `D′` reads definitely at `phi`, if it reads one: `G′/2D`,
    /// the cosine between the core's tangent and the spine's normal
    /// levered by the core radius.
    fn turn(&self, far: bool, phi: T) -> Option<Sign> {
        let r = read(self.core, self.spine, far, phi);
        let big = self.core.radius;
        let cos = r.g1 / (T::from_f64(2.0) * r.d * big);
        match decide(self.rows.turn, Margin::levered(cos, big), self.band) {
            Ok(s @ (Sign::Positive | Sign::Negative)) => Some(s),
            Ok(Sign::Zero) | Err(_) => None,
        }
    }

    fn cell(&self, far: bool, (lo, m, hi): (T, T, T), w: T) -> Cell<T> {
        let big = self.core.radius;
        let two = T::from_f64(2.0);
        let reach = big * w;
        let r = read(self.core, self.spine, far, m);
        if self.positive(
            self.rows.clear,
            Margin::of(nearest(r.d, self.levels) - reach),
        ) {
            return Cell::Clear;
        }
        if let Ok(Decided {
            sign: Sign::Zero,
            margin,
        }) = decide_reported(
            self.rows.gap,
            Margin::of(r.d.max(self.levels[1])),
            self.band,
        ) {
            return Cell::Tangent(Gap::Tangent {
                margin,
                at: self.core.point(m),
            });
        }
        let Some(third) = self.third(r, reach) else {
            return Cell::Open(r);
        };
        // `D` over the cell from `G`'s Taylor bound, which stays tight
        // where `D` is small and `R·w` is not.
        let g = r.d.powi(2);
        let swing =
            r.g1.abs() * w + r.g2.abs() * w.powi(2) / two + third * w.powi(3) / T::from_f64(6.0);
        let (d_lo, d_hi) = ((g - swing).max(T::zero()).sqrt(), (g + swing).sqrt());
        let apart = |level: T| (level - d_hi).max(d_lo - level);
        if self.positive(
            self.rows.clear,
            Margin::of(apart(self.levels[0]).min(apart(self.levels[1]))),
        ) {
            return Cell::Clear;
        }
        // The least `|G′|` over the cell, as the least `|D′|` it bounds.
        let least = r.g1.abs() - r.g2.abs() * w - third * w.powi(2) / two;
        if self.positive(
            self.rows.monotone,
            Margin::levered(least / (two * (r.d + reach) * big), big),
        ) {
            return Cell::Clear;
        }
        // Read across the cell padded by half its width each side, so a
        // turn on a cut between two cells is inside the window of each.
        let (lo, hi) = (lo - w, hi + w);
        match (self.turn(far, lo), self.turn(far, hi)) {
            (Some(a), Some(b)) if a != b => self
                .turn_window(far, (lo, hi), w)
                .map_or(Cell::Open(r), Cell::Tangent),
            _ => Cell::Open(r),
        }
    }

    /// **A turn of `D` inside `[lo, hi]`**, whose ends read `D′` of
    /// opposite signs: the sign change bisected, and windows about it, of
    /// half-width `w` and each half the last, read until one decides the
    /// gap `Zero` with its own ends turning. `G` over a window is its
    /// Taylor bound about the turn, its value read back through `√`.
    fn turn_window(&self, far: bool, (lo, hi): (T, T), w: T) -> Option<Gap<T>> {
        let big = self.core.radius;
        let (two, six) = (T::from_f64(2.0), T::from_f64(6.0));
        let slope = |phi: T| read(self.core, self.spine, far, phi).g1;
        let (mut a, mut b) = (lo, hi);
        let at_lo = slope(lo);
        for _ in 0..GAP_BISECTIONS {
            let c = (a + b) / two;
            let product = slope(c) * at_lo;
            a = product.select_le_zero(a, c);
            b = product.select_le_zero(c, b);
        }
        let mid = (a + b) / two;
        let r = read(self.core, self.spine, far, mid);
        let g = r.d.powi(2);
        let mut h = w;
        for _ in 0..GAP_WINDOWS {
            if let Some(third) = self.third(r, big * h) {
                let (left, right) = (self.turn(far, mid - h), self.turn(far, mid + h));
                if matches!((left, right), (Some(x), Some(y)) if x != y) {
                    let swing =
                        r.g1.abs() * h + r.g2.abs() * h.powi(2) / two + third * h.powi(3) / six;
                    let (d_lo, d_hi) = ((g - swing).max(T::zero()).sqrt(), (g + swing).sqrt());
                    for level in self.levels {
                        let bound = (d_lo - level).abs().max((d_hi - level).abs());
                        if let Ok(Decided {
                            sign: Sign::Zero,
                            margin,
                        }) = decide_reported(self.rows.gap, Margin::of(bound), self.band)
                        {
                            return Some(Gap::Tangent {
                                margin,
                                at: self.core.point(mid),
                            });
                        }
                    }
                }
            }
            h = h / two;
        }
        None
    }

    /// **The escalation of a cell no test settled**: the gap read over the
    /// cell, `|D(m) − L| ∓ R·w` for the nearer level. Bounds that straddle
    /// the band escalate as one straddle; bounds both decided `Zero` stand
    /// for a critical value no reading certified, a `Zero` the gap cannot
    /// pass.
    fn floor_refusal(&self, r: Reading<T>, reach: T) -> Indeterminate {
        let near = nearest(r.d, self.levels);
        let bound = |m: T| decide_reported(self.rows.gap, Margin::of(m), self.band);
        match (bound(near - reach), bound(near + reach)) {
            (Err(e), _) | (_, Err(e)) => e,
            (Ok(lower), Ok(upper)) if lower.sign < upper.sign => {
                straddle(self.rows.straddle, lower, upper, self.band)
            }
            (Ok(_), Ok(upper)) => {
                gate_rejection(self.rows.gap, upper, self.band, SizedPass::Positive)
            }
        }
    }
}

/// **The critical gap of the core `core` against `spine`**, at the
/// levels `ρ₁ + ρ₂` and `|ρ₁ − ρ₂|` of the two offsets `radii` (module
/// docs).
///
/// # Errors
///
/// The [`Indeterminate`] of the first cell the sweep could not settle
/// at its floor or its budget, where no cell decided a tangency.
pub(crate) fn critical_gap<T: Decide>(
    core: Core<T>,
    spine: Spine<T>,
    [ra, rb]: [T; 2],
    rows: &GapRows,
    band: Band,
) -> Result<Gap<T>, Indeterminate> {
    let sweep = Sweep {
        core,
        spine,
        levels: [ra + rb, (ra - rb).abs()],
        rows,
        band,
    };
    let branches: &[bool] = match spine {
        Spine::Circle { .. } => &[false, true],
        Spine::Point(_) | Spine::Line { .. } => &[false],
    };
    let two = T::from_f64(2.0);
    let piece = T::tau() / T::from_f64(GAP_START as f64);
    let mut cells: Vec<(T, T)> = (0..GAP_START)
        .rev()
        .map(|k| {
            let lo = piece * T::from_f64(k as f64);
            (lo, lo + piece)
        })
        .collect();
    let mut spent = 0;
    let mut held: Option<Indeterminate> = None;
    while let Some((lo, hi)) = cells.pop() {
        spent += 1;
        let w = (hi - lo) / two;
        let m = lo + w;
        let mut open = None;
        for &far in branches {
            match sweep.cell(far, (lo, m, hi), w) {
                Cell::Clear => {}
                Cell::Tangent(gap) => return Ok(gap),
                Cell::Open(r) => {
                    open.get_or_insert(r);
                }
            }
        }
        let Some(r) = open else {
            continue;
        };
        let reach = core.radius * w;
        let floored = matches!(
            decide(rows.floor, Margin::of(reach * T::from_f64(4.0)), band),
            Ok(Sign::Zero)
        );
        if spent >= GAP_CELLS {
            return Err(held.unwrap_or_else(|| sweep.floor_refusal(r, reach)));
        }
        if floored {
            held.get_or_insert_with(|| sweep.floor_refusal(r, reach));
            continue;
        }
        cells.push((m, hi));
        cells.push((lo, m));
    }
    held.map_or(Ok(Gap::Transverse), Err)
}
