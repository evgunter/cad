//! **The span × spline face arm** of the curve × face table
//! ([`super::Door::SplineFace`]): a line, circle, ellipse or NURBS span
//! against a NURBS chart, settled on boxes of the state `(t, u, v)` on
//! `F = C(t) − S(u, v)`. A line or a conic reads `C` and `C′` over a
//! parameter window in closed form; a NURBS span reads them off its
//! ruled sheet (`sheet`), whose enclosures are the span's control hull
//! and its hodograph's quotient bound, cut to the window.
//!
//! `F` is a displacement in metres, so a box is read against the band
//! with no meter: the box is **clear** where `|F|` over it is decided
//! past the band, its enclosure the meet of the componentwise one
//! (`C` over the parameter window minus the chart's first-order box,
//! [`NurbsBoxes::rect_box`]) and the mean-value one `F(m) + J(X)(X − m)`.
//! A box that is not clear is read by the **Krawczyk operator**
//! ([`crate::krawczyk`]), `J = [C′ | −S_u | −S_v]`: where its image lies
//! strictly inside `X`, `X` holds exactly one solution and every
//! Jacobian in it is invertible, so the meeting is transverse; `det J(X)`
//! definitely nonzero is read as well, which the inclusion already
//! implies, as the contract's own spelling of the second half. The
//! meeting's box is narrowed by the operator, then grown into the
//! largest box round it on which the test still holds, its **isolating
//! boxes**: every box inside one is accounted for by its meeting, and
//! only by it. Any other box is halved along its widest side in metres,
//! down to a floor; a box there that no isolating box holds is a pass
//! within the band, a tangency or a meeting on a crease, and the arm
//! refuses: an in-band reading escalates and never prunes, and the
//! operator's "no solution here" is no clearance.
//!
//! The state is `(t, u, v)` and not the chart alone against the span's
//! two implicits: `F` is metric where an implicit residual needs a
//! divisor per implicit to be read as a distance, every analytic kind
//! reads it alike, and the meeting comes out with its chart point, so it
//! is placed on the sheet it was found on.
//!
//! - **No box straddles a crease** (an interior knot run of
//!   multiplicity at least the degree, `Knots::crease_values`), the
//!   span's or the chart's: the state is cut at both before the walk,
//!   and an isolating box is grown inside its own crease-free block. **The crease rule** has
//!   one home ([`Cuts::touch`]): a box touching a crease carries no
//!   Krawczyk proof, so no isolating box holds it, and at the floor it
//!   refuses [`Unsettled::Crease`]. That is what refuses a meeting within
//!   the band of a crease: the carrier there lies within the band of the
//!   crease's point, so every box holding the meeting's state touches
//!   the crease and is never decided past the band, and none is held. A
//!   box at the floor refuses under that label where a crease lies within
//!   the reach its test was read over; the label says where the refusal
//!   stands, not how far.
//! - **The span's ends are the end vertices'.** The walk reads the
//!   carrier a little past each end ([`END_REACH`]), as far as it runs
//!   smoothly (to its domain's end or the nearest crease beyond the end),
//!   so a meeting at an end is isolated like any other; one within the
//!   band of an end, or past it, is not reported. At every end, of every
//!   kind of span, the walk also tries an **end's box**: a box it reads,
//!   or one of the floor growths it tries a proof on, that holds the end's
//!   state (the end's parameter and its foot on the chart) and on which
//!   every solution lies within the band of the end, by the mean value
//!   theorem over the box ([`crate::krawczyk::solution_reach`]). It claims
//!   no solution, so it needs no reading past either domain: it is what
//!   reads an end the walk cannot read past (a NURBS span ending at its
//!   domain's end or on a crease of its own) and an end on the chart's
//!   domain edge. Any other meeting on the chart's domain edge is isolated
//!   by no box inside the domain, so it refuses like any other
//!   unaccounted box, an open chart's edge and a closed chart's seam
//!   alike.
//! - A meeting must read ON the chart, the carrier's point within the
//!   band of the chart's at the meeting's chart point; two within the
//!   band of each other refuse.
//!
//! The walk is `geom_core::bisect::bisect`, with its piece budget.

use core::num::NonZeroUsize;

use geom::{Curve3, NurbsSurface};
use geom_core::bisect::{Step, bisect};
use geom_core::k_stats::decide;
use geom_core::spline::Knots;
use geom_core::{
    Band, Bounds, CertifiedBounds, Decide, Indeterminate, Interval, Margin, Point3, Real, Sign,
    Vec3,
};

use crate::krawczyk::{det3, image, inverse3, meet3, midpoints, norm, solution_reach};
use crate::roots::Unsettled;
use crate::ssi::enclose::{NurbsBoxes, UvWindow};

/// Why [`isolate`] or [`settle`] gave no answer.
#[derive(Clone, Copy, Debug)]
pub enum SplineFaceRefusal {
    /// A decision the arm must read definite fell in the band's gap:
    /// whether a meeting lies ON the chart, or where it stands against
    /// an end of the span, a crease or the meeting before it.
    Escalated(Indeterminate),
    /// The walk could not settle the span: [`Unsettled::Tangency`] (a box
    /// at the floor within the band that no meeting accounts for: a pass
    /// within the band, a tangency, two meetings the band cannot part, or
    /// a meeting on the chart's domain edge), [`Unsettled::Crease`] (such
    /// a box within its floor reach of a crease among its block's sides,
    /// as a meeting within the band of a crease leaves one),
    /// [`Unsettled::SignInBand`] (a meeting does not read on the chart)
    /// or [`Unsettled::Exhausted`].
    Unsettled(Unsettled),
    /// The arm could not read the pair: the carrier is a spiric, or a
    /// scalar, a knot or a speed of the carrier or the chart did not
    /// certify.
    Unread(&'static str),
    /// Two enclosures of one residual share no point: an argument behind
    /// one of them is wrong, a kernel fault.
    Fault(&'static str),
}

impl From<Unsettled> for SplineFaceRefusal {
    fn from(why: Unsettled) -> Self {
        Self::Unsettled(why)
    }
}

/// **One meeting of the span with the chart**: the parameter on the
/// span, the chart point it meets, and the carrier's point there.
#[derive(Clone, Copy, Debug)]
pub struct ChartMeeting<T: Real> {
    /// The parameter on the span.
    pub t: T,
    /// The chart point, `(u, v)`.
    pub uv: (f64, f64),
    /// The carrier's point at `t`.
    pub point: Point3<T>,
}

/// K name: a box of the state read against the band, as definitely
/// apart or not.
const PAST_THE_BAND: &str = "spline_face_past_the_band";

/// K name: how far a solution in an end's box can lie from the end, read
/// against the band.
const AT_THE_END: &str = "spline_face_at_the_end";

/// How far past each end of the span the walk reads the carrier, in
/// escalation lengths at the span's pace (a lower bound on its mean
/// speed). A meeting at an end is a split point decided ON within the
/// band's zero, so its solution lies within `zero / sin θ` of the end, `θ`
/// the crossing's angle: where the carrier runs no faster near the end
/// than its pace, the reach covers every crossing steeper than
/// `zero / (1024·escalate) < 1/1024` rad, and any other is the end's box's
/// to account for or refuses. No answer but a refusal rests on its size.
const END_REACH: f64 = 1024.0;

/// The growth an isolating box is tried at, largest first, in
/// escalation lengths mapped through `Y`: each is re-proved by the test
/// before it is used, so the ladder bounds work and the reach of an
/// isolating box (`4096·|Y|·escalate` at most), never a decision.
const GROWTH: [f64; 5] = [4096.0, 512.0, 64.0, 8.0, 1.0];

/// The most times a meeting's box is narrowed by the operator: each
/// narrowing at least halves a box the operator contracts, so 64 reach
/// `f64` resolution from any box; a narrowing that stops sooner leaves a
/// wider box holding the same one solution.
const NARROWINGS: u32 = 64;

/// How far a box at the floor is grown, in its own sides, for the test
/// to reach a meeting beside it: the one growth (by its sides) every box
/// gets, and at the floor four times them, a work bound with no answer
/// but a refusal resting on it.
const FLOOR_REACHES: [f64; 2] = [1.0, 4.0];

/// A box of the state, `[lo, hi]` per coordinate: `t`, `u`, `v`.
#[derive(Clone, Copy, Debug, PartialEq)]
struct Cell([(f64, f64); 3]);

impl Cell {
    fn mid(self) -> [f64; 3] {
        self.0.map(|(a, b)| a + (b - a) * 0.5)
    }

    fn interval(self, k: usize) -> Interval {
        Interval::from_bounds(self.0[k].0, self.0[k].1)
    }

    fn within(self, o: Self) -> bool {
        (0..3).all(|k| o.0[k].0 <= self.0[k].0 && self.0[k].1 <= o.0[k].1)
    }

    fn strictly_within(self, o: Self) -> bool {
        (0..3).all(|k| o.0[k].0 < self.0[k].0 && self.0[k].1 < o.0[k].1)
    }

    // `!(a <= b)` is deliberate: a NaN side meets nothing.
    #[allow(clippy::neg_cmp_op_on_partial_ord)]
    fn meet(self, o: Self) -> Option<Self> {
        let mut s = self.0;
        for (k, side) in s.iter_mut().enumerate() {
            *side = (side.0.max(o.0[k].0), side.1.min(o.0[k].1));
            if !(side.0 <= side.1) {
                return None;
            }
        }
        Some(Self(s))
    }

    /// The halves along coordinate `k`, lower first.
    fn halves(self, k: usize) -> [Self; 2] {
        let (a, b) = self.0[k];
        let c = a + (b - a) * 0.5;
        let (mut lo, mut hi) = (self, self);
        lo.0[k].1 = c;
        hi.0[k].0 = c;
        [lo, hi]
    }

    /// The box grown by `by` of its own sides each way, inside `block`.
    fn grown(self, by: f64, block: Self) -> Self {
        Self(core::array::from_fn(|k| {
            let w = (self.0[k].1 - self.0[k].0) * by;
            (
                (self.0[k].0 - w).max(block.0[k].0),
                (self.0[k].1 + w).min(block.0[k].1),
            )
        }))
    }
}

/// A chart window, `u` by `v`: the sheet's is `t` by `{0}`.
fn window(u: (f64, f64), v: (f64, f64)) -> Option<UvWindow> {
    UvWindow::new(u.0, u.1, v.0, v.1)
}

/// The sheet's window over the parameter `t`, `None` where `t` does not
/// certify: a NURBS carrier is read off boxes over the window, which
/// carry no decoration of `t`'s, so the refusal is asked here.
fn certified_window(t: Interval) -> Option<UvWindow> {
    if !t.is_certified() {
        return None;
    }
    window((t.lo(), t.hi()), (0.0, 0.0))
}

/// The enclosure of a vector that does not certify, which fails every
/// test.
fn poison() -> [Interval; 3] {
    [Interval::from_bounds(f64::NAN, f64::NAN); 3]
}

/// The point enclosure of `x`.
fn exact(x: f64) -> Interval {
    Interval::from_bounds(x, x)
}

/// The span's carrier crossed into certification arithmetic:
/// `C(t) = o + d·t`, `C(θ) = c + e₁·cos θ + e₂·sin θ`, or a NURBS
/// carrier read off its ruled sheet ([`sheet`]).
enum Path<'a, T: CertifiedBounds> {
    Line {
        o: [Interval; 3],
        d: [Interval; 3],
    },
    Conic {
        c: [Interval; 3],
        e1: [Interval; 3],
        e2: [Interval; 3],
    },
    Spline {
        boxes: Box<NurbsBoxes<'a, T>>,
    },
}

impl<'a, T: CertifiedBounds> Path<'a, T> {
    fn of(carrier: &'a SpanCarrier<'a, T>) -> Option<Self> {
        let (sheet, carrier) = (carrier.sheet.as_ref(), carrier.curve);
        let p = |p: Point3<T>| [p.x, p.y, p.z].map(Interval::from_certified);
        let v = |v: Vec3<T>| [v.x, v.y, v.z].map(Interval::from_certified);
        let scaled = |v: [Interval; 3], s: T| v.map(|c| c * Interval::from_certified(s));
        let path = match *carrier {
            Curve3::Line { origin, dir } => Self::Line {
                o: p(origin),
                d: v(dir),
            },
            Curve3::Circle {
                center,
                axis,
                radius,
                u_ref,
            } => Self::Conic {
                c: p(center),
                e1: scaled(v(u_ref), radius),
                e2: scaled(geom_core::interval::cross(&v(axis), &v(u_ref)), radius),
            },
            Curve3::Ellipse {
                center,
                axis,
                semi_u,
                semi_v,
                u_ref,
            } => Self::Conic {
                c: p(center),
                e1: scaled(v(u_ref), semi_u),
                e2: scaled(geom_core::interval::cross(&v(axis), &v(u_ref)), semi_v),
            },
            Curve3::Nurbs(_) => {
                return sheet.map(|s| Self::Spline {
                    boxes: Box::new(NurbsBoxes::new(s)),
                });
            }
            Curve3::Spiric { .. } => return None,
        };
        let all = match &path {
            Self::Line { o, d } => o.iter().chain(d).all(|c| c.is_certified()),
            Self::Conic { c, e1, e2 } => c.iter().chain(e1).chain(e2).all(|c| c.is_certified()),
            Self::Spline { .. } => true,
        };
        all.then_some(path)
    }

    fn at(&self, t: Interval) -> [Interval; 3] {
        match self {
            Self::Line { o, d } => core::array::from_fn(|k| o[k] + d[k] * t),
            Self::Conic { c, e1, e2 } => {
                let (s, co) = Real::sin_cos(t);
                core::array::from_fn(|k| c[k] + e1[k] * co + e2[k] * s)
            }
            Self::Spline { boxes, .. } => match certified_window(t) {
                Some(w) => {
                    let b = boxes.rect_box(w);
                    [b.x, b.y, b.z]
                }
                None => poison(),
            },
        }
    }

    fn tangent(&self, t: Interval) -> [Interval; 3] {
        match self {
            Self::Line { d, .. } => *d,
            Self::Conic { e1, e2, .. } => {
                let (s, co) = Real::sin_cos(t);
                core::array::from_fn(|k| e2[k] * co - e1[k] * s)
            }
            Self::Spline { boxes, .. } => match certified_window(t) {
                Some(w) => {
                    let b = boxes.deriv_box(w, true);
                    [b.x, b.y, b.z]
                }
                None => poison(),
            },
        }
    }

    /// An upper bound on the carrier's speed over `domain`, and a lower
    /// one, or zero where none certifies; NaN where the upper does not
    /// certify. A NURBS carrier's are its sheet's meter
    /// ([`NurbsSurface::stretch_floor`], [`NurbsBoxes::speed_sup`]): the
    /// quotient bound `geom::NurbsCurve3`'s own speed bounds read, which
    /// this arm's sole `CertifiedBounds` scalar cannot call.
    fn speeds(&self, domain: (f64, f64)) -> (f64, f64) {
        let (a, b) = match self {
            Self::Line { d, .. } => (norm(*d), norm(*d)),
            Self::Conic { e1, e2, .. } => (norm(*e1), norm(*e2)),
            Self::Spline { boxes } => {
                let fast =
                    window(domain, (0.0, 0.0)).map_or(f64::NAN, |w| boxes.speed_sup(w, true));
                let slow = Interval::from_certified(boxes.surface().stretch_floor(true));
                let slow = if slow.is_certified() { slow.lo() } else { 0.0 };
                return if fast.is_finite() {
                    (fast, slow)
                } else {
                    (f64::NAN, f64::NAN)
                };
            }
        };
        if !(a.is_certified() && b.is_certified()) {
            return (f64::NAN, f64::NAN);
        }
        (a.hi().max(b.hi()), a.lo().min(b.lo()))
    }
}

/// **A NURBS carrier's ruled sheet**, `S(t, s) = C(t)`: the carrier's
/// knots along `t`, two copies of its net along `s` at degree one. The
/// sheet's net is the carrier's, so its enclosures over a window
/// `[t₀, t₁] × {0}` ([`NurbsBoxes`]: the control hull, the hodograph's
/// quotient bound, each cut to the window below a knot span) are the
/// carrier's over `[t₀, t₁]`, and its creases along `t` are the
/// carrier's. `None` for any other carrier, or where the net does not
/// form one.
fn sheet<T: Decide>(carrier: &Curve3<T>) -> Option<NurbsSurface<T>> {
    let Curve3::Nurbs(net) = carrier else {
        return None;
    };
    let twice = NonZeroUsize::new(2)?;
    let across = Knots::new(vec![(T::zero(), twice), (T::one(), twice)], 1).ok()?;
    let control = net.control().iter().flat_map(|&p| [p, p]).collect();
    let weights = net.weights().iter().flat_map(|&w| [w, w]).collect();
    NurbsSurface::new(net.knot_values().clone(), across, control, weights).ok()
}

/// **A span's carrier as [`isolate`] reads it**: the carrier, and a NURBS
/// carrier's ruled sheet ([`sheet`]), built from it here and nowhere else.
pub struct SpanCarrier<'a, T: Real> {
    curve: &'a Curve3<T>,
    sheet: Option<NurbsSurface<T>>,
}

impl<'a, T: Decide> SpanCarrier<'a, T> {
    /// `curve`, with its ruled sheet where it is a NURBS.
    #[must_use]
    pub fn of(curve: &'a Curve3<T>) -> Self {
        Self {
            curve,
            sheet: sheet(curve),
        }
    }
}

/// The pair, read over boxes of the state.
struct Pair<'a, T: CertifiedBounds> {
    path: Path<'a, T>,
    boxes: NurbsBoxes<'a, T>,
    /// Metres per unit of each coordinate, upper bounds.
    speeds: [f64; 3],
}

impl<T: CertifiedBounds> Pair<'_, T> {
    /// `C` over `t` minus the chart's first-order box over `w`.
    fn gap(&self, t: Interval, w: UvWindow) -> [Interval; 3] {
        let s = self.boxes.rect_box(w);
        let c = self.path.at(t);
        [c[0] - s.x, c[1] - s.y, c[2] - s.z]
    }

    /// `F` at the state `m`, enclosed.
    fn at(&self, m: [f64; 3]) -> [Interval; 3] {
        match UvWindow::new(m[1], m[1], m[2], m[2]) {
            Some(w) => self.gap(exact(m[0]), w),
            None => poison(),
        }
    }

    /// `J = [C′ | −S_u | −S_v]` over `x`, by rows.
    fn jacobian(&self, x: Cell) -> [[Interval; 3]; 3] {
        let Some(w) = window(x.0[1], x.0[2]) else {
            return [poison(); 3];
        };
        let ct = self.path.tangent(x.interval(0));
        let su = self.boxes.deriv_box(w, true);
        let sv = self.boxes.deriv_box(w, false);
        let (su, sv) = ([su.x, su.y, su.z], [sv.x, sv.y, sv.z]);
        let zero = exact(0.0);
        core::array::from_fn(|r| [ct[r], zero - su[r], zero - sv[r]])
    }

    /// `F` over `x`: the componentwise enclosure met with the mean-value
    /// one. `None` where the two share no point.
    fn residual(
        &self,
        x: Cell,
        jac: &[[Interval; 3]; 3],
        fm: [Interval; 3],
    ) -> Option<[Interval; 3]> {
        let Some(w) = window(x.0[1], x.0[2]) else {
            return Some(poison());
        };
        let plain = self.gap(x.interval(0), w);
        let m = x.mid();
        let d: [Interval; 3] = core::array::from_fn(|k| x.interval(k) - exact(m[k]));
        let mv = core::array::from_fn(|r| (0..3).fold(fm[r], |acc, k| acc + jac[r][k] * d[k]));
        meet3(plain, mv)
    }

    /// The Krawczyk test over `x` (module docs): the image, where it
    /// lies strictly inside `x` and `det J(x)` excludes zero.
    fn krawczyk(&self, x: Cell) -> Option<Cell> {
        self.krawczyk_with(x, &self.jacobian(x), self.at(x.mid()))
    }

    /// [`Self::krawczyk`] with `jac` an enclosure of `J` over `x` and `fm`
    /// of `F` at its middle.
    fn krawczyk_with(&self, x: Cell, jac: &[[Interval; 3]; 3], fm: [Interval; 3]) -> Option<Cell> {
        let det = det3(*jac);
        if !(det.is_certified() && (det.lo() > 0.0 || det.hi() < 0.0)) {
            return None;
        }
        let k = Cell(image(x.0, fm, jac)?);
        k.strictly_within(x).then_some(k)
    }

    /// The meeting in `x`, which the test proved there, narrowed: met with
    /// its image until the meet stops shrinking.
    fn narrow(&self, mut x: Cell) -> Cell {
        for _ in 0..NARROWINGS {
            let Some(next) =
                image(x.0, self.at(x.mid()), &self.jacobian(x)).and_then(|k| Cell(k).meet(x))
            else {
                return x;
            };
            if next == x {
                return x;
            }
            x = next;
        }
        x
    }

    /// The largest box round the meeting `root` inside `block` on which
    /// the test holds, if one does past `proved`, the box it was found
    /// in.
    fn grown(&self, root: Cell, proved: Cell, block: Cell, band: Band) -> Option<Cell> {
        let y = inverse3(midpoints(&self.jacobian(root)))?;
        let c = root.mid();
        for g in GROWTH {
            let mut grown = root;
            for k in 0..3 {
                let r = (0..3).map(|q| y[k][q].abs()).sum::<f64>() * g * band.escalate();
                grown.0[k] = (
                    (c[k] - r).min(root.0[k].0).max(block.0[k].0),
                    (c[k] + r).max(root.0[k].1).min(block.0[k].1),
                );
            }
            if !grown.within(proved) && self.krawczyk(grown).is_some() {
                return Some(grown);
            }
        }
        None
    }

    /// The state of the span's end at the parameter `end`: `end` and the
    /// foot on `chart` of the carrier's point there, with an enclosure of
    /// `F` at that state. The point's enclosure only seeds the foot, so its
    /// middle is taken. `None` where the foot is not found.
    fn end_state(&self, end: f64, chart: &NurbsSurface<T>) -> Option<End> {
        let c = self.path.at(exact(end));
        let [x, y, z] = c.map(|k| T::from_computed(k.lo() + (k.hi() - k.lo()) * 0.5));
        let foot = chart.project(Point3::new(x, y, z)).ok()?;
        let at = [end, foot.u, foot.v];
        Some(End {
            at: Cell(at.map(|c| (c, c))),
            f: self.at(at),
        })
    }

    /// **Whether `x` is an end's box** (module docs): it holds the state
    /// of an end, and every solution in it lies within the band of that
    /// end ([`crate::krawczyk::solution_reach`], `jac` an enclosure of `J`
    /// over `x`).
    fn holds_an_end(&self, x: Cell, jac: &[[Interval; 3]; 3], ends: &[End], band: Band) -> bool {
        ends.iter().any(|e| {
            e.at.within(x)
                && solution_reach(e.f, jac, self.speeds).is_some_and(|r| {
                    matches!(
                        decide(AT_THE_END, Margin::of(Interval::from_bounds(0.0, r)), band),
                        Ok(Sign::Zero)
                    )
                })
        })
    }

    /// The coordinate whose side of `x` is the longest in metres, and
    /// that length.
    fn widest(&self, x: Cell) -> (usize, f64) {
        (0..3)
            .map(|k| (k, (x.0[k].1 - x.0[k].0) * self.speeds[k]))
            .fold((0, f64::NEG_INFINITY), |a, b| if b.1 > a.1 { b } else { a })
    }
}

/// An end of the span as the walk reads it: its state, and an enclosure
/// of `F` there.
struct End {
    at: Cell,
    f: [Interval; 3],
}

/// A meeting the walk isolated: its narrowed box, and every box the
/// test proved it the one solution of.
struct Found {
    root: Cell,
    isolating: Vec<Cell>,
}

impl Found {
    fn holds(&self, x: Cell) -> bool {
        self.isolating.iter().any(|b| x.within(*b))
    }
}

/// **The meetings the walk isolated** ([`isolate`]), read in
/// certification arithmetic and not yet placed against the span's ends
/// or one another ([`settle`]).
#[derive(Clone, Debug)]
pub struct Isolated {
    /// The span, its ends' enclosures' outer ends.
    span: (f64, f64),
    /// Each meeting's narrowed box, `(t, u, v)`, ascending in `t`.
    roots: Vec<[(f64, f64); 3]>,
}

/// **The walk** (module docs): every meeting of `carrier` over `params`
/// with `chart`, each isolated by the Krawczyk test and read clear of the
/// span's and the chart's creases, every decision taken on an enclosure.
/// A NURBS carrier is read off its ruled sheet ([`SpanCarrier`]).
/// [`settle`] places them; the curve × face table's lane composes the
/// three ([`crate::NurbsLane::chart_meetings`]).
///
/// # Errors
///
/// [`SplineFaceRefusal`], as its variants say.
// `!(a < b)` is deliberate: a NaN end refuses.
#[allow(clippy::neg_cmp_op_on_partial_ord)]
pub fn isolate<T: CertifiedBounds>(
    carrier: &SpanCarrier<'_, T>,
    params: (T, T),
    chart: &NurbsSurface<T>,
    band: Band,
) -> Result<Isolated, SplineFaceRefusal> {
    let unread = SplineFaceRefusal::Unread;
    let path = Path::of(carrier).ok_or(unread(
        "the carrier is not a line, a circle, an ellipse or a NURBS, or a scalar of it does not certify",
    ))?;
    let (start, end) = (
        Interval::from_certified(params.0),
        Interval::from_certified(params.1),
    );
    if !(start.is_certified() && end.is_certified()) {
        return Err(unread("an end of the span does not certify"));
    }
    let (a, b) = (start.lo(), end.hi());
    if !(a < b) {
        return Err(unread("the span's ends do not certify forward"));
    }
    // The carrier's own cuts along `t`: its domain's ends and its creases.
    let ts = match &carrier.sheet {
        Some(s) => cuts(s, true)?,
        None => vec![f64::NEG_INFINITY, f64::INFINITY],
    };
    let carried = (ts[0], ts[ts.len() - 1]);
    if !(carried.0 <= a && b <= carried.1) {
        return Err(unread("the span leaves its carrier's domain"));
    }
    let (fast, least) = path.speeds(carried);
    // The span's pace: a lower bound on its mean speed over `[a, b]`, the
    // larger of the carrier's least speed (a NURBS meter is signed, and
    // may read nothing on a bent piece) and its chord over its parameter
    // length. It sizes only the reach and the floor below.
    let (ca, cb) = (path.at(exact(a)), path.at(exact(b)));
    let chord = norm(core::array::from_fn(|k| cb[k] - ca[k]));
    let mean = if chord.is_certified() {
        (exact(chord.lo()) / (exact(b) - exact(a))).lo()
    } else {
        0.0
    };
    let pace = least.max(mean);
    if !(fast.is_finite() && pace > 0.0) {
        return Err(unread("the carrier's speed does not certify"));
    }
    let boxes = NurbsBoxes::new(chart);
    let (us, vs) = (cuts(chart, true)?, cuts(chart, false)?);
    let (du, dv) = ((us[0], us[us.len() - 1]), (vs[0], vs[vs.len() - 1]));
    let domain = UvWindow::new(du.0, du.1, dv.0, dv.1).ok_or(unread("the chart has no domain"))?;
    let speeds = [
        fast,
        boxes.speed_sup(domain, true),
        boxes.speed_sup(domain, false),
    ];
    if !speeds.iter().all(|s| s.is_finite() && *s > 0.0) {
        return Err(unread("the chart's speeds do not certify"));
    }
    // Past each end of the span, as far as the carrier runs smoothly: to
    // its domain's end or its nearest crease beyond the end, whichever
    // comes first. A crease at an end is then a side of the walk, not a
    // line through it, so the end's meeting is the end's.
    let reach = END_REACH * band.escalate() / pace;
    let before = ts
        .iter()
        .copied()
        .filter(|&k| k <= a)
        .fold(carried.0, f64::max);
    let after = ts
        .iter()
        .copied()
        .filter(|&k| k >= b)
        .fold(carried.1, f64::min);
    let t = ((a - reach).max(before), (b + reach).min(after));
    let mut tcuts = vec![t.0];
    tcuts.extend(ts[1..ts.len() - 1].iter().filter(|&&k| t.0 < k && k < t.1));
    tcuts.push(t.1);
    let mut blocks: Vec<Cell> = Vec::new();
    for t in tcuts.windows(2) {
        for u in us.windows(2) {
            for v in vs.windows(2) {
                blocks.push(Cell([(t[0], t[1]), (u[0], u[1]), (v[0], v[1])]));
            }
        }
    }
    let pair = Pair {
        path,
        boxes,
        speeds,
    };
    // The shortest side a box is cut to, in metres: `tangency_floor`'s
    // form over the shorter of the span (its length bounded below by its
    // pace) and the chart. That length is no
    // parting radius, and the floor bounds no decision: a box at it that
    // no meeting holds refuses, so no answer but a refusal rests on it.
    let reach_of = |(lo, hi): (f64, f64), speed: f64| (hi - lo) * speed;
    let rho = reach_of((a, b), pace).min(reach_of(du, speeds[1]) + reach_of(dv, speeds[2]));
    let floor = (rho * band.escalate()).sqrt() / 8.0;
    let cut_lines = Cuts {
        lines: [
            &tcuts[1..tcuts.len() - 1],
            &us[1..us.len() - 1],
            &vs[1..vs.len() - 1],
        ],
    };
    // The span's ends, each at its foot on the chart (module docs).
    let end_states: Vec<End> = [a, b]
        .into_iter()
        .filter_map(|end| pair.end_state(end, chart))
        .collect();
    let mut ends: Vec<Cell> = Vec::new();
    let mut found: Vec<Found> = Vec::new();
    let pieces = blocks.iter().map(|&x| (x, x));
    let walked = bisect(pieces, |(x, block): (Cell, Cell)| {
        if held(&found, &ends, x) {
            return Step::Settled;
        }
        let jac = pair.jacobian(x);
        let fm = pair.at(x.mid());
        let Some(f) = pair.residual(x, &jac, fm) else {
            return Step::Stop(SplineFaceRefusal::Fault(
                "two enclosures of the residual over a box are disjoint",
            ));
        };
        if matches!(
            decide(PAST_THE_BAND, Margin::of(norm(f)), band),
            Ok(Sign::Positive)
        ) {
            return Step::Settled;
        }
        // A box touching a crease carries no proof, so it is cut on to the
        // escalation length, where its carrier is decided past the band of
        // the crease or refuses.
        let creased = cut_lines.touch(x, block);
        let (k, length) = pair.widest(x);
        let at_floor = !(length > if creased { band.escalate() } else { floor });
        // Read on `x`, then on `x` grown inside its block, so a meeting
        // beside `x` is isolated before its neighbours are walked to the
        // floor.
        let reaches: &[f64] = if at_floor {
            &FLOOR_REACHES
        } else {
            &FLOOR_REACHES[..1]
        };
        // No box touching a crease carries a proof (the crease rule): a
        // grown box stays inside the block's crease sides.
        let proof_block = cut_lines.inside(block);
        let grown = reaches.iter().filter_map(|&by| {
            let g = x.grown(by, proof_block);
            (g.within(proof_block) && !cut_lines.touch(g, block)).then_some(g)
        });
        let mut proof = None;
        for g in (!creased).then_some(x).into_iter().chain(grown) {
            let (jg, fg) = if g == x {
                (jac, fm)
            } else {
                (pair.jacobian(g), pair.at(g.mid()))
            };
            // Each reading on `g` stands alone: an end's box accounts for
            // the solutions in `g` whatever the proof below finds there, and
            // a proved solution inside it is the end's (below).
            if pair.holds_an_end(g, &jg, &end_states, band) {
                ends.push(g);
                if held(&found, &ends, x) {
                    return Step::Settled;
                }
            }
            proof = pair.krawczyk_with(g, &jg, fg).map(|k| (g, k));
            if proof.is_some() {
                break;
            }
        }
        // A box the test proves holds one solution; one whose narrowed box
        // lies in a box already proved is that meeting's, the solution
        // there being the only one.
        if let Some((proved, k)) = proof {
            let root = pair.narrow(k);
            match found.iter_mut().find(|f| f.holds(root)) {
                Some(f) => f.isolating.push(proved),
                // The one solution in an end's box is the end's.
                None if ends.iter().any(|e| root.within(*e)) => ends.push(proved),
                None => {
                    let mut isolating = vec![proved];
                    isolating.extend(pair.grown(root, proved, proof_block, band));
                    found.push(Found { root, isolating });
                }
            }
            if held(&found, &ends, x) {
                return Step::Settled;
            }
        }
        if !at_floor {
            let [lo, hi] = x.halves(k);
            return Step::Halve([(lo, block), (hi, block)]);
        }
        // Within the band at the floor, and no meeting holds it: a pass
        // within the band, a tangency, a meeting on the chart's edge, or
        // one on a crease.
        // Labelled by the reach the test was read over: a crease within
        // it is where the refusal stands.
        let near_crease = cut_lines.touch(x.grown(FLOOR_REACHES[1], block), block);
        Step::Stop(if near_crease {
            Unsettled::Crease.into()
        } else {
            Unsettled::Tangency.into()
        })
    });
    match walked {
        Err(geom_core::bisect::Exhausted) => return Err(Unsettled::Exhausted.into()),
        Ok(Some(refusal)) => return Err(refusal),
        Ok(None) => {}
    }
    found.sort_by(|p, q| p.root.0[0].0.total_cmp(&q.root.0[0].0));
    Ok(Isolated {
        span: (a, b),
        roots: found.iter().map(|f| f.root.0).collect(),
    })
}

/// The chart's parameters along `u` (`along_u`) or `v` at which the walk
/// is cut: its domain's ends and its creases strictly inside, each as its
/// enclosure's lower end.
fn cuts<T: CertifiedBounds>(
    chart: &NurbsSurface<T>,
    along_u: bool,
) -> Result<Vec<f64>, SplineFaceRefusal> {
    let knots = if along_u {
        chart.knot_values_u()
    } else {
        chart.knot_values_v()
    };
    let (lo, hi) = knots.domain();
    let (lo, hi) = (Interval::from_certified(lo), Interval::from_certified(hi));
    let creases: Vec<Interval> = knots
        .crease_values()
        .into_iter()
        .map(Interval::from_certified)
        .collect();
    if !(lo.is_certified() && hi.is_certified() && creases.iter().all(|k| k.is_certified())) {
        return Err(SplineFaceRefusal::Unread(
            "a knot of the chart does not certify",
        ));
    }
    let (lo, hi) = (lo.lo(), hi.hi());
    let mut out = vec![lo];
    out.extend(creases.iter().map(|k| k.lo()).filter(|&k| lo < k && k < hi));
    out.push(hi);
    Ok(out)
}

/// The interior crease parameters of the state along each coordinate:
/// the carrier's along `t`, the chart's along `u` and `v`.
#[derive(Clone, Copy)]
struct Cuts<'a> {
    lines: [&'a [f64]; 3],
}

impl Cuts<'_> {
    /// `block` with each side on a crease moved one `f64` step inward: the
    /// largest box inside it that touches no crease, which a proof's box
    /// is grown within.
    fn inside(self, block: Cell) -> Cell {
        let mut out = block;
        for (k, lines) in self.lines.iter().enumerate() {
            let (lo, hi) = block.0[k];
            if lines.contains(&lo) {
                out.0[k].0 = lo.next_up();
            }
            if lines.contains(&hi) {
                out.0[k].1 = hi.next_down();
            }
        }
        out
    }

    /// **The crease rule** (module docs): whether `x` touches a crease
    /// among its block's sides, on the carrier or on the chart. Such a box
    /// carries no proof, and at the floor it refuses [`Unsettled::Crease`].
    fn touch(self, x: Cell, block: Cell) -> bool {
        self.lines.iter().enumerate().any(|(k, lines)| {
            [block.0[k].0, block.0[k].1]
                .into_iter()
                .any(|c| lines.contains(&c) && x.0[k].0 <= c && c <= x.0[k].1)
        })
    }
}

/// Whether a meeting or an end accounts for `x`.
fn held(found: &[Found], ends: &[Cell], x: Cell) -> bool {
    found.iter().any(|f| f.holds(x)) || ends.iter().any(|e| x.within(*e))
}

/// **The meetings strictly inside the span**, ascending, from what
/// [`isolate`] found: each read ON the chart, then placed in metres
/// against the span's ends and the meeting before it. A meeting at an
/// end of the span is not reported: it is the end vertex's. Each is a
/// parameter whose point reads ON the chart, with the chart point it was
/// found at, not an enclosure of the true meeting.
///
/// # Errors
///
/// [`SplineFaceRefusal`], as its variants say.
pub fn settle<T: Decide>(
    carrier: &Curve3<T>,
    chart: &NurbsSurface<T>,
    isolated: &Isolated,
    band: Band,
) -> Result<Vec<ChartMeeting<T>>, SplineFaceRefusal> {
    let (a, b) = isolated.span;
    let gap = |row, p: Point3<T>, q: Point3<T>| {
        decide(row, Margin::of((p - q).norm()), band).map_err(SplineFaceRefusal::Escalated)
    };
    let ends = [a, b].map(|t| carrier.eval(T::from_computed(t)));
    let mut out: Vec<ChartMeeting<T>> = Vec::with_capacity(isolated.roots.len());
    for &root in &isolated.roots {
        let root = Cell(root);
        if root.0[0].1 <= a || root.0[0].0 >= b {
            continue;
        }
        let [tm, um, vm] = root.mid();
        let point = carrier.eval(T::from_computed(tm));
        let foot = chart.eval(T::from_computed(um), T::from_computed(vm));
        if gap("spline_face_root_on", point, foot)? != Sign::Zero {
            return Err(Unsettled::SignInBand.into());
        }
        let mut at_end = false;
        for end in ends {
            at_end |= gap("spline_face_root_at_end", point, end)? == Sign::Zero;
        }
        if at_end {
            continue;
        }
        if !(a < root.0[0].0 && root.0[0].1 < b) {
            return Err(Unsettled::Tangency.into());
        }
        if let Some(before) = out.last()
            && gap("spline_face_roots_apart", point, before.point)? == Sign::Zero
        {
            return Err(Unsettled::Tangency.into());
        }
        out.push(ChartMeeting {
            t: T::from_computed(tm),
            uv: (um, vm),
            point,
        });
    }
    Ok(out)
}

#[cfg(test)]
mod rows;

#[cfg(test)]
mod review_r2_rows;
