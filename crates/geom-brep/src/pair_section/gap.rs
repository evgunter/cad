//! **The critical gap of a torus pair** (module docs of
//! [`super`], the torus poses): whether the two surfaces are tangent,
//! read on their spines by a certified sweep of the torus's core angle.
//! It serves the pairs whose critical values have no closed form, a
//! cylinder's or a torus's partner; a sphere's spine is a point, whose
//! critical values against a core circle are its two meridian distances
//! ([`super::torus`]).
//!
//! It decides whether the pair is tangent and where, and says nothing of
//! whether a given curve on both surfaces runs along the tangency within
//! ε.
//!
//! # Offset duality
//!
//! A cylinder and a torus are each the points at distance `ρ` from a
//! spine: a line, a core circle. Two such surfaces are tangent exactly
//! where a critical value of the distance between a point of one spine
//! and a point of the other equals `ρ₁ + ρ₂` or `|ρ₁ − ρ₂|`: at a common
//! point the normal of each passes through its spine's nearest point, so
//! the two feet and the point are collinear, which is the critical-point
//! condition of the spine distance. The gap is how far the critical
//! values stand from those two levels.
//!
//! The first spine is the torus's core circle, swept by its angle `φ`.
//! At each point `p(φ)` the spine distance is critical along the second
//! spine at its foot (the line's foot, or the circle's nearest and
//! farthest points in `p`'s meridian half-plane), so its critical values
//! are those of the one-variable functions `D(φ) = |p(φ) − q(φ)|`, one
//! per branch of the foot, at their critical points. The circle's feet
//! are unique off its axis; where the core crosses that axis every point
//! of the circle is a foot, `D` has a kink, and a critical value can sit
//! on the kink itself.
//!
//! # The sweep
//!
//! The turn is cut into cells. On a cell of half-width `w` about `m`,
//! `D` moves by at most `R·w` (the spine distance is 1-Lipschitz in
//! space and `p` moves at the core's speed `R`, kinks included). The
//! squared distance `G = D²` is smooth off the kinks (a line's is a
//! trigonometric polynomial, a circle's `|w|² ∓ 2R₂ρ + R₂²` with `ρ` the
//! distance from its axis), so its slope over the cell is bounded by
//! Taylor's theorem: `G′(m)` and `G″(m)` read at the point, and a bound
//! on `|G‴|` over the cell from `|p′| = |p″| = |p‴| = R` (and, for the
//! circle, `ρ` bounded below over the cell, the kink's distance). A cell
//! clears when every branch either
//!
//! - stands past the band from both levels over the whole cell
//!   (`|D(m) − L| − R·w` definitely positive, or, where `G` is smooth,
//!   the range of `D` its Taylor bound gives clear of both levels), or
//! - is smooth on it and monotone, so it holds no critical point:
//!   `G′ = 2D·D′` vanishes at every critical point of the spine
//!   distance, its zeros included.
//!
//! Every cell clearing is a certified [`Gap::Transverse`]: a critical
//! point of the spine distance lies in some cell, where neither test can
//! pass unless its value stands past the band.
//!
//! # The scale of each margin
//!
//! Every margin is a length, and carries the rounding of the reading it
//! is made of ([`Rounding`]):
//!
//! - a value test reads `D` against a level, metres of spine distance,
//!   less `D`'s rounding;
//! - a slope test first certifies the sign of `G′`: `|G′|`, or the
//!   monotone test's Taylor lower bound over the cell, `|G′(m)| −
//!   |G″(m)|·w − |G‴|·w²/2`, is shrunk toward zero by the rounding of
//!   `G′` and `G″`. A `G′` within its own rounding of zero reads no sign
//!   at all, however small `D` is, so no rounding is ever divided by
//!   `D`. What is left is a certified lower bound on `|G′| = 2D·|D′|`,
//!   and over `2D_hi`, `D_hi` the most `D` reaches, a certified lower
//!   bound on `|D′| = R·sin θ`, `θ` the angle between the core's tangent
//!   and the offset `p − q` (`G′ = 2(p − q)·p′`, the foot's own
//!   variation dropping out, the envelope theorem). That sine levered by
//!   the core radius is the band's reading (D4's `θ·r`): how far turning
//!   the core by the angle moves it.
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
//! The margin a `Zero` reports is the bound on the gap that was decided:
//! the window's `|D − L|` bound, or `max(D(m), |ρ₁ − ρ₂|)`, which bounds
//! `|D_min − |ρ₁ − ρ₂||` from above since `0 ≤ D_min ≤ D(m)`. The gap
//! itself is not known closer than that.
//!
//! A critical value on a kink is never read by a turn (no window about
//! it is smooth), and reads `Zero` only where it is the least distance
//! and the level is `|ρ₁ − ρ₂|`: any other tangency on a kink escalates.
//!
//! Any other cell is split, down to a floor where it stands within a
//! quarter band of the core, or until the cell budget is spent: there it
//! escalates on its reading of the gap ([`Sweep::floor_refusal`]). Below
//! the budget a Zero found anywhere outranks an escalation found
//! elsewhere, since a floor cell's escalation is held while the walk
//! goes on; the cell that spends the budget escalates at once, on the
//! held escalation if there is one, without reading the cells left.

use geom_core::k_stats::{decide, decide_reported, gate_rejection, straddle};
use geom_core::{
    Band, Decide, Decided, Indeterminate, Margin, Point3, Real, Sign, SizedPass, Vec3,
    rounding_charge,
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
    /// A cylinder's axis.
    Line { origin: Point3<T>, axis: Vec3<T> },
    /// A torus's core circle.
    Circle {
        center: Point3<T>,
        axis: Vec3<T>,
        radius: T,
    },
}

impl<T: Real> Spine<T> {
    /// The spine's anchor and the most its points stand from it in a
    /// reading: what the reading's coordinates are made of.
    fn anchor(self) -> (Point3<T>, T) {
        match self {
            Self::Line { origin, .. } => (origin, T::zero()),
            Self::Circle { center, radius, .. } => (center, radius),
        }
    }
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
    /// A cell's branch is smooth: off the kinks.
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
        /// The verdict's margin: the bound on the gap that was decided
        /// `Zero` (module docs).
        margin: geom_core::MarginDiag,
        /// The core point the critical value was read at.
        at: Point3<T>,
    },
}

/// **The rounding a reading carries**, a first-order estimate in the
/// `f64` lane's sense ([`rounding_charge`]): bounds on how far `D`, `G′`
/// and `G″` as evaluated stand from their values in real arithmetic at
/// the same angle.
///
/// A reading is made of coordinates of size `s`, the core's centre and
/// radius and the spine's anchor and radius, so the offset `w = p − o`
/// carries a charge of `s` (the sine and cosine, the core point, the
/// difference). `D = |Πw|` carries that and its own root. `G′ = 2Πw·Πp′`
/// carries it levered by `|p′| = R`, and its own product of size
/// `2R·|w|`; `G″ = 2(|Πp′|² + Πw·Πp″)` the same with `|p″| = R`. A
/// circle's branch adds `2R₂` times the rounding of `ρ′` and `ρ″`, whose
/// quotients by `ρ` grow as the core nears the circle's axis: at a kink
/// no slope reads at all.
#[derive(Clone, Copy)]
struct Rounding<T> {
    d: T,
    g1: T,
    g2: T,
}

/// One branch of the spine distance read at one angle: `D`, and the
/// squared distance `G = D²`'s first two derivatives there, in metres
/// squared per radian, with their rounding.
#[derive(Clone, Copy)]
struct Reading<T: Real> {
    d: T,
    g1: T,
    g2: T,
    /// `|p − c|` for the circle, `D` for the line: what `G‴`'s bound
    /// reads.
    reach: T,
    /// The core point's distance from the partner circle's axis, less
    /// its rounding.
    kink: Option<T>,
    err: Rounding<T>,
}

impl<T: Real> Reading<T> {
    /// The rounding of `G = D²` and of its Taylor swing over `w`.
    fn swing_rounding(&self, w: T) -> T {
        let two = T::from_f64(2.0);
        two * self.d * self.err.d + self.err.g1 * w + self.err.g2 * w.powi(2) / two
    }
}

/// `⟨Πw, Πw⟩`'s first two derivatives along the core, `w = p − o` and
/// `Π` the projection square to `axis` (none for the whole space):
/// `(S, S′, S″)`.
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
    let big = core.radius;
    let (two, four) = (T::from_f64(2.0), T::from_f64(4.0));
    let (anchor, spread) = spine.anchor();
    let size =
        (core.center - Point3::origin()).norm() + big + (anchor - Point3::origin()).norm() + spread;
    // The charges of the offset's own products (module docs, `Rounding`).
    let e_w = rounding_charge(size);
    let e1 = rounding_charge(four * big * size);
    let e2 = rounding_charge(two * big * (big + two * size));
    match spine {
        Spine::Line { origin, axis } => {
            let [g, g1, g2] = squared(p - origin, dp, ddp, Some(axis));
            let d = g.sqrt();
            Reading {
                d,
                g1,
                g2,
                reach: d,
                kink: None,
                err: Rounding {
                    d: e_w + rounding_charge(d),
                    g1: e1,
                    g2: e2,
                },
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
            let r1 = s1 / (two * rho);
            let r2 = s2 / (two * rho) - s1.powi(2) / (four * rho.powi(3));
            // `ρ`'s rounding, and its quotients' through `|S′| ≤ 2ρR`.
            let e_rho = e_w + rounding_charge(rho);
            let e_r1 = (e1 / two + big * e_rho) / rho + rounding_charge(r1.abs());
            let e_r2 = e2 / (two * rho)
                + s2.abs() * e_rho / (two * rho.powi(2))
                + (big * e1 + T::from_f64(3.0) * big.powi(2) * e_rho) / rho.powi(2)
                + rounding_charge(r2.abs());
            let sigma = if far { T::one() } else { -T::one() };
            let h = w.dot(axis);
            let across = rho + sigma * radius;
            let d = (across.powi(2) + h.powi(2)).sqrt();
            Reading {
                d,
                g1: n1 + two * sigma * radius * r1,
                g2: n2 + two * sigma * radius * r2,
                reach: n.sqrt(),
                kink: Some(rho - e_rho),
                err: Rounding {
                    d: e_rho + e_w + rounding_charge(d),
                    g1: e1 + two * radius * e_r1,
                    g2: e2 + two * radius * e_r2,
                },
            }
        }
    }
}

/// The least of `|D − L|` over the two levels.
fn nearest<T: Real>(d: T, levels: [T; 2]) -> T {
    (d - levels[0]).abs().min((d - levels[1]).abs())
}

/// `x` shrunk toward zero by `by`, zero where `x ± by` holds it.
fn shrunk<T: Real>(x: T, by: T) -> T {
    (x - by).max(T::zero()) + (x + by).min(T::zero())
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

    /// **A slope of `G` as the crossing it bounds** (module docs, the
    /// scale of each margin): `slope`, a bound on `|G′|` already shrunk
    /// by its rounding, over `2·D_hi·R`, `D_hi` a bound on `D` from above,
    /// is a lower bound on the sine between the core's tangent and the
    /// offset, `D′/R`; levered by the core radius it is the point
    /// deviation that turning the core by that angle moves.
    fn slope(&self, slope: T, d_hi: T) -> Margin<T> {
        let big = self.core.radius;
        Margin::levered(slope / (T::from_f64(2.0) * d_hi * big), big)
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

    /// The sign `D′` reads definitely at `phi`, if it reads one: `G′`'s,
    /// shrunk by its rounding first (a `G′` within its rounding of zero
    /// reads no sign, whatever `D` is), then read as the crossing it
    /// bounds.
    fn turn(&self, far: bool, phi: T) -> Option<Sign> {
        let r = read(self.core, self.spine, far, phi);
        match decide(
            self.rows.turn,
            self.slope(shrunk(r.g1, r.err.g1), r.d + r.err.d),
            self.band,
        ) {
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
            Margin::of(nearest(r.d, self.levels) - reach - r.err.d),
        ) {
            return Cell::Clear;
        }
        if let Ok(Decided {
            sign: Sign::Zero,
            margin,
        }) = decide_reported(
            self.rows.gap,
            Margin::of((r.d + r.err.d).max(self.levels[1])),
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
        let swing = r.g1.abs() * w
            + r.g2.abs() * w.powi(2) / two
            + third * w.powi(3) / T::from_f64(6.0)
            + r.swing_rounding(w);
        let (d_lo, d_hi) = ((g - swing).max(T::zero()).sqrt(), (g + swing).sqrt());
        let apart = |level: T| (level - d_hi).max(d_lo - level);
        if self.positive(
            self.rows.clear,
            Margin::of(apart(self.levels[0]).min(apart(self.levels[1]))),
        ) {
            return Cell::Clear;
        }
        // The least `|G′|` over the cell, less its rounding, as the least
        // crossing it bounds against `D`'s most over the cell.
        let least = r.g1.abs() - r.err.g1 - (r.g2.abs() + r.err.g2) * w - third * w.powi(2) / two;
        if self.positive(self.rows.monotone, self.slope(least, r.d + r.err.d + reach)) {
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
                    let swing = r.g1.abs() * h
                        + r.g2.abs() * h.powi(2) / two
                        + third * h.powi(3) / six
                        + r.swing_rounding(h);
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
    /// cell, `|D(m) − L| ∓ (R·w + its rounding)` for the nearer level.
    /// Bounds that straddle the band escalate as one straddle; bounds both
    /// decided `Zero` stand for a critical value no reading certified, a
    /// `Zero` the gap cannot pass.
    fn floor_refusal(&self, r: Reading<T>, reach: T) -> Indeterminate {
        let near = nearest(r.d, self.levels);
        let reach = reach + r.err.d;
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
        Spine::Line { .. } => &[false],
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
