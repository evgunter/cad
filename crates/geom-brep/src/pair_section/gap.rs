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
//! space and `p` moves at the core's speed `R`, kinks included), and
//! off the spine and off the kinks it is smooth with `|D″| ≤ R + R²/κ`,
//! `κ` the least of `D` and the distance from the circle's axis over the
//! cell (each bounds a principal curvature of the distance's level set).
//! A cell clears when every branch either
//!
//! - stands past the band from both levels over the whole cell
//!   (`|D(m) − L| − R·w` definitely positive), or
//! - is smooth on it and monotone (`|D′(m)| − |D″|·w` definitely
//!   positive), so it holds no critical point.
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
//!   critical point lies inside, and the window's bound on `|D − L|` is
//!   decided `Zero`. The window is centred where `D′` changes sign, found
//!   by bisection, and sized so its second-order bound fits the band;
//! - **the least distance**: the spine distance's global minimum is
//!   always a critical value, and no larger than `D` anywhere, so `D(m)`
//!   and the level `|ρ₁ − ρ₂|` both within the zero band decide it. This
//!   is the tube-circle tangency (equal minor radii, cores touching),
//!   whose critical point sits where `D` is zero and not smooth.
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
const GAP_WINDOWS: u32 = 4;

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

/// One branch of the spine distance read at one angle.
#[derive(Clone, Copy)]
struct Reading<T: Real> {
    /// `D`.
    d: T,
    /// `D′`, metres per radian.
    slope: T,
    /// The core point's distance from the partner circle's axis.
    kink: Option<T>,
}

fn read<T: Real>(core: Core<T>, spine: Spine<T>, far: bool, phi: T) -> Reading<T> {
    let (p, dp) = (core.point(phi), core.velocity(phi));
    match spine {
        Spine::Point(c) => {
            let off = p - c;
            let d = off.norm();
            Reading {
                d,
                slope: off.dot(dp) / d,
                kink: None,
            }
        }
        Spine::Line { origin, axis } => {
            let (off, d) = super::square_part(p - origin, axis);
            Reading {
                d,
                slope: off.dot(dp) / d,
                kink: None,
            }
        }
        Spine::Circle {
            center,
            axis,
            radius,
        } => {
            let w = p - center;
            let h = w.dot(axis);
            let (perp, rho) = super::square_part(w, axis);
            let across = if far { rho + radius } else { rho - radius };
            let d = (across.powi(2) + h.powi(2)).sqrt();
            // `∇D` is `(across·ê + h·a) / D`, `ê` the unit part of `w`
            // square to the axis.
            let grad = (perp / rho * across + axis * h) / d;
            Reading {
                d,
                slope: grad.dot(dp),
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

    /// `D″`'s bound over a window `reach` metres of core about a reading,
    /// where the window is smooth.
    fn bend(&self, r: Reading<T>, reach: T) -> Option<T> {
        let big = self.core.radius;
        if !self.positive(self.rows.smooth, Margin::of(r.d - reach)) {
            return None;
        }
        let mut least = r.d - reach;
        if let Some(rho) = r.kink {
            if !self.positive(self.rows.smooth, Margin::of(rho - reach)) {
                return None;
            }
            least = least.min(rho - reach);
        }
        Some(big + big.powi(2) / least)
    }

    /// The sign `D′` reads definitely at `phi`, if it reads one.
    fn turn(&self, far: bool, phi: T) -> Option<Sign> {
        let r = read(self.core, self.spine, far, phi);
        let big = self.core.radius;
        match decide(
            self.rows.turn,
            Margin::levered(r.slope / big, big),
            self.band,
        ) {
            Ok(s @ (Sign::Positive | Sign::Negative)) => Some(s),
            Ok(Sign::Zero) | Err(_) => None,
        }
    }

    fn cell(&self, far: bool, (lo, m, hi): (T, T, T), w: T) -> Cell<T> {
        let big = self.core.radius;
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
        let Some(bend) = self.bend(r, reach) else {
            return Cell::Open(r);
        };
        if self.positive(
            self.rows.monotone,
            Margin::levered((r.slope.abs() - bend * w) / big, big),
        ) {
            return Cell::Clear;
        }
        match (self.turn(far, lo), self.turn(far, hi)) {
            (Some(a), Some(b)) if a != b => self
                .turn_window(far, (lo, hi))
                .map_or(Cell::Open(r), Cell::Tangent),
            _ => Cell::Open(r),
        }
    }

    /// **A turn of `D` inside `[lo, hi]`**, whose ends read `D′` of
    /// opposite signs: the sign change bisected, and windows about it
    /// read until one decides the gap `Zero` with its own ends turning.
    fn turn_window(&self, far: bool, (lo, hi): (T, T)) -> Option<Gap<T>> {
        let big = self.core.radius;
        let two = T::from_f64(2.0);
        let slope = |phi: T| read(self.core, self.spine, far, phi).slope;
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
        let guess = self.bend(r, T::zero())?;
        let mut h = (T::from_f64(self.band.zero()) / (two * guess)).sqrt() / big;
        for _ in 0..GAP_WINDOWS {
            if let Some(bend) = self.bend(r, big * h) {
                let (left, right) = (self.turn(far, mid - h), self.turn(far, mid + h));
                if matches!((left, right), (Some(x), Some(y)) if x != y) {
                    for level in self.levels {
                        let bound =
                            (r.d - level).abs() + r.slope.abs() * h + bend * h.powi(2) / two;
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
