//! **The analytic span × spline face arm** of the curve × face table
//! ([`super::Door::SplineFace`]): a line, circle or ellipse span against
//! a NURBS chart, settled on boxes of the state `(t, u, v)` on
//! `F = C(t) − S(u, v)`.
//!
//! `F` is a displacement in metres, so a box is read against the band
//! with no meter: the box is **clear** where `|F|` over it is decided
//! past the band, its enclosure the meet of the componentwise one
//! (`C` over the parameter window minus the chart's first-order box,
//! [`NurbsBoxes::rect_box`]) and the mean-value one `F(m) + J(X)(X − m)`.
//! A box that is not clear is read by the **Krawczyk operator**
//! `K = m − Y·F(m) + (I − Y·J(X))(X − m)`, `Y` the inverse of `J`'s
//! midpoint, `J = [C′ | −S_u | −S_v]`: where `K` lies strictly inside
//! `X` and `det J(X)` is definitely nonzero, `X` holds exactly one
//! solution and every Jacobian in it is invertible, so the meeting is
//! transverse. Its box is narrowed by the operator, then grown into the
//! largest box round it on which the test still holds, its **isolating
//! box**: every box inside one is accounted for by its meeting, and only
//! by it. Any other box is halved along its widest side in metres, down
//! to the band's escalation length; a box there that no isolating box
//! holds is a pass within the band or a tangency, and the arm refuses
//! ([`Unsettled::Tangency`]): an in-band reading escalates and never
//! prunes, and the Krawczyk operator's "no solution here" is no
//! clearance.
//!
//! The state is `(t, u, v)` and not the chart alone against the span's
//! two implicits: `F` is metric where an implicit residual needs a
//! divisor per implicit to be read as a distance, every analytic kind
//! reads it alike, and the meeting comes out with its chart point, so it
//! is placed on the sheet it was found on.
//!
//! - **No box straddles a crease** (an interior knot run of
//!   multiplicity at least the degree, `KnotStructure::crease_runs`):
//!   the chart is cut at its creases before the walk, and an isolating
//!   box is grown inside its own crease-free block. A meeting within the
//!   band of a crease's iso-curve refuses ([`Unsettled::Crease`]).
//! - **The span's ends are the end vertices'.** The walk reads the
//!   carrier a little past each end ([`END_REACH`]), so a meeting at an
//!   end is isolated like any other; one within the band of an end, or
//!   past it, is not reported.
//! - A meeting must read ON the chart, the carrier's point within the
//!   band of the chart's at the meeting's chart point; two within the
//!   band of each other refuse.
//!
//! The walk is `geom_core::bisect::bisect`, with its piece budget.

use geom::{Curve3, NurbsSurface};
use geom_core::bisect::{Step, bisect};
use geom_core::k_stats::decide;
use geom_core::{
    Band, Bounds, CertifiedBounds, Decide, Indeterminate, Interval, Margin, Point3, Real, Sign,
    Vec3,
};

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
    /// within the band, a tangency, or two meetings the band cannot
    /// part), [`Unsettled::Crease`], [`Unsettled::SignInBand`] (a
    /// meeting does not read on the chart) or [`Unsettled::Exhausted`].
    Unsettled(Unsettled),
    /// The arm could not read the pair: the carrier is not a line, circle
    /// or ellipse, or a scalar, a knot or the chart's speeds did not
    /// certify.
    Unread(&'static str),
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

/// How far past each end of the span the walk reads the carrier, in
/// escalation lengths: a meeting at an end then lies inside the walk's
/// domain, where the Krawczyk test can isolate it.
const END_REACH: f64 = 1024.0;

/// The growth an isolating box is tried at, largest first, in
/// escalation lengths mapped through `Y`.
const GROWTH: [f64; 5] = [4096.0, 512.0, 64.0, 8.0, 1.0];

/// The most times a meeting's box is narrowed by the operator.
const NARROWINGS: u32 = 64;

/// `|f|`, enclosed.
fn norm(f: [Interval; 3]) -> Interval {
    (f[0].powi(2) + f[1].powi(2) + f[2].powi(2)).sqrt()
}

/// The point enclosure of `x`.
fn point(x: f64) -> Interval {
    Interval::from_bounds(x, x)
}

/// The refused enclosure.
fn refused() -> Interval {
    Interval::from_bounds(f64::NAN, f64::NAN)
}

/// The meet of two certified enclosures of one value, or `a` where
/// either is refused or they share no point.
fn meet(a: Interval, b: Interval) -> Interval {
    if !(a.is_certified() && b.is_certified()) {
        return a;
    }
    let (lo, hi) = (a.lo().max(b.lo()), a.hi().min(b.hi()));
    if lo <= hi {
        Interval::from_bounds(lo, hi)
    } else {
        a
    }
}

/// A pair of the chart's knot values, as the `f64` ends of their
/// enclosures; `None` where one does not certify.
fn ends_of<T: CertifiedBounds>((lo, hi): (T, T)) -> Option<(f64, f64)> {
    let (lo, hi) = (Interval::from_certified(lo), Interval::from_certified(hi));
    (lo.is_certified() && hi.is_certified()).then(|| (lo.lo(), hi.hi()))
}

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
}

/// The span's carrier crossed into certification arithmetic:
/// `C(t) = o + d·t` or `C(θ) = c + e₁·cos θ + e₂·sin θ`.
enum Path {
    Line {
        o: [Interval; 3],
        d: [Interval; 3],
    },
    Conic {
        c: [Interval; 3],
        e1: [Interval; 3],
        e2: [Interval; 3],
    },
}

impl Path {
    fn of<T: CertifiedBounds>(carrier: &Curve3<T>) -> Option<Self> {
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
            Curve3::Spiric { .. } | Curve3::Nurbs(_) => return None,
        };
        let all = match &path {
            Self::Line { o, d } => o.iter().chain(d).all(|c| c.is_certified()),
            Self::Conic { c, e1, e2 } => c.iter().chain(e1).chain(e2).all(|c| c.is_certified()),
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
        }
    }

    fn tangent(&self, t: Interval) -> [Interval; 3] {
        match self {
            Self::Line { d, .. } => *d,
            Self::Conic { e1, e2, .. } => {
                let (s, co) = Real::sin_cos(t);
                core::array::from_fn(|k| e2[k] * co - e1[k] * s)
            }
        }
    }

    /// An upper bound on the carrier's speed, and a lower one.
    fn speeds(&self) -> (f64, f64) {
        match self {
            Self::Line { d, .. } => {
                let n = norm(*d);
                (n.hi(), n.lo())
            }
            Self::Conic { e1, e2, .. } => {
                let (a, b) = (norm(*e1), norm(*e2));
                (a.hi().max(b.hi()), a.lo().min(b.lo()))
            }
        }
    }
}

/// The pair, read over boxes of the state.
struct Pair<'a, T: CertifiedBounds> {
    path: Path,
    boxes: NurbsBoxes<'a, T>,
    /// Metres per unit of each coordinate, upper bounds.
    speeds: [f64; 3],
}

impl<T: CertifiedBounds> Pair<'_, T> {
    fn window(x: Cell) -> Option<UvWindow> {
        UvWindow::new(x.0[1].0, x.0[1].1, x.0[2].0, x.0[2].1)
    }

    /// `F` at the state `m`, enclosed.
    fn at(&self, m: [f64; 3]) -> [Interval; 3] {
        let Some(w) = UvWindow::new(m[1], m[1], m[2], m[2]) else {
            return [refused(); 3];
        };
        let s = self.boxes.rect_box(w);
        let c = self.path.at(point(m[0]));
        [c[0] - s.x, c[1] - s.y, c[2] - s.z]
    }

    /// `J = [C′ | −S_u | −S_v]` over `x`, by rows.
    fn jacobian(&self, x: Cell) -> [[Interval; 3]; 3] {
        let Some(w) = Self::window(x) else {
            return [[refused(); 3]; 3];
        };
        let ct = self.path.tangent(x.interval(0));
        let su = self.boxes.deriv_box(w, true);
        let sv = self.boxes.deriv_box(w, false);
        let (su, sv) = ([su.x, su.y, su.z], [sv.x, sv.y, sv.z]);
        let zero = point(0.0);
        core::array::from_fn(|r| [ct[r], zero - su[r], zero - sv[r]])
    }

    /// `F` over `x`: the componentwise enclosure met with the mean-value
    /// one.
    fn residual(&self, x: Cell, jac: &[[Interval; 3]; 3], fm: [Interval; 3]) -> [Interval; 3] {
        let Some(w) = Self::window(x) else {
            return [refused(); 3];
        };
        let s = self.boxes.rect_box(w);
        let c = self.path.at(x.interval(0));
        let plain = [c[0] - s.x, c[1] - s.y, c[2] - s.z];
        let m = x.mid();
        let d: [Interval; 3] = core::array::from_fn(|k| x.interval(k) - point(m[k]));
        core::array::from_fn(|r| {
            let mv = (0..3).fold(fm[r], |acc, k| acc + jac[r][k] * d[k]);
            meet(plain[r], mv)
        })
    }

    /// The Krawczyk test over `x` (module docs): the image, where it
    /// lies strictly inside `x` and `det J(x)` excludes zero.
    fn krawczyk(&self, x: Cell) -> Option<Cell> {
        self.krawczyk_with(x, self.jacobian(x), self.at(x.mid()))
    }

    /// [`Self::krawczyk`] with `jac` an enclosure of `J` over `x` and `fm`
    /// of `F` at its middle.
    fn krawczyk_with(&self, x: Cell, jac: [[Interval; 3]; 3], fm: [Interval; 3]) -> Option<Cell> {
        let det = crate::ssi::four::det3(jac);
        if !(det.is_certified() && (det.lo() > 0.0 || det.hi() < 0.0)) {
            return None;
        }
        let k = self.image(x, &jac, fm)?;
        k.strictly_within(x).then_some(k)
    }

    /// The Krawczyk operator's image of `x`: every solution in `x` lies
    /// in it. `None` where `J`'s midpoint is singular or the image does
    /// not certify.
    fn image(&self, x: Cell, jac: &[[Interval; 3]; 3], f: [Interval; 3]) -> Option<Cell> {
        let m = x.mid();
        let jm = jac.map(|row| row.map(|v| v.lo() + (v.hi() - v.lo()) * 0.5));
        let y = crate::ssi::four::inverse3(jm)?;
        let d: [Interval; 3] = core::array::from_fn(|k| x.interval(k) - point(m[k]));
        let mut out = x;
        for r in 0..3 {
            let mut v = point(m[r]);
            for c in 0..3 {
                v = v - point(y[r][c]) * f[c];
            }
            for c in 0..3 {
                let mut yj = point(if r == c { 1.0 } else { 0.0 });
                for q in 0..3 {
                    yj = yj - point(y[r][q]) * jac[q][c];
                }
                v = v + yj * d[c];
            }
            if !v.is_certified() {
                return None;
            }
            out.0[r] = (v.lo(), v.hi());
        }
        Some(out)
    }

    /// The meeting in `x`, which the test proved there, narrowed: met with
    /// its image until the meet stops shrinking.
    fn narrow(&self, mut x: Cell) -> Cell {
        for _ in 0..NARROWINGS {
            let Some(next) = self
                .image(x, &self.jacobian(x), self.at(x.mid()))
                .and_then(|k| k.meet(x))
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
        let jm = self
            .jacobian(root)
            .map(|row| row.map(|v| v.lo() + (v.hi() - v.lo()) * 0.5));
        let y = crate::ssi::four::inverse3(jm)?;
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

    /// The coordinate whose side of `x` is the longest in metres, and
    /// that length.
    fn widest(&self, x: Cell) -> (usize, f64) {
        (0..3)
            .map(|k| (k, (x.0[k].1 - x.0[k].0) * self.speeds[k]))
            .fold((0, f64::NEG_INFINITY), |a, b| if b.1 > a.1 { b } else { a })
    }
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
/// chart's creases, every decision taken on an enclosure. [`settle`]
/// places them; the curve × face table's lane composes the two
/// ([`crate::NurbsLane::chart_meetings`]).
///
/// # Errors
///
/// [`SplineFaceRefusal`], as its variants say.
// `!(a < b)` is deliberate: a NaN end refuses.
#[allow(clippy::neg_cmp_op_on_partial_ord)]
pub fn isolate<T: CertifiedBounds>(
    carrier: &Curve3<T>,
    params: (T, T),
    chart: &NurbsSurface<T>,
    band: Band,
) -> Result<Isolated, SplineFaceRefusal> {
    let unread = SplineFaceRefusal::Unread;
    let path = Path::of(carrier).ok_or(unread(
        "the carrier is not a line, a circle or an ellipse, or a scalar of it does not certify",
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
    let (fast, slow) = path.speeds();
    if !(fast.is_finite() && slow > 0.0) {
        return Err(unread("the carrier's speed does not certify"));
    }
    let boxes = NurbsBoxes::new(chart);
    let unknotted = || unread("a knot of the chart does not certify");
    let du = ends_of(chart.knot_values_u().domain()).ok_or_else(unknotted)?;
    let dv = ends_of(chart.knot_values_v().domain()).ok_or_else(unknotted)?;
    let domain = UvWindow::new(du.0, du.1, dv.0, dv.1).ok_or(unread("the chart has no domain"))?;
    let speeds = [
        fast,
        boxes.speed_sup(domain, true),
        boxes.speed_sup(domain, false),
    ];
    if !speeds.iter().all(|s| s.is_finite() && *s > 0.0) {
        return Err(unread("the chart's speeds do not certify"));
    }
    let reach = END_REACH * band.escalate() / slow;
    let t = (a - reach, b + reach);
    let (us, vs) = (cuts(chart, true, du)?, cuts(chart, false, dv)?);
    let blocks: Vec<Cell> = us
        .windows(2)
        .flat_map(|u| {
            vs.windows(2)
                .map(move |v| Cell([t, (u[0], u[1]), (v[0], v[1])]))
        })
        .collect();
    let pair = Pair {
        path,
        boxes,
        speeds,
    };
    // A box not cleared nor isolated is cut no finer than a tangency
    // keeps open, read over the shorter of the span and the chart.
    let reach_of = |(lo, hi): (f64, f64), speed: f64| (hi - lo) * speed;
    let rho = reach_of((a, b), slow).min(reach_of(du, speeds[1]) + reach_of(dv, speeds[2]));
    let floor = geom_core::bisect::tangency_floor(rho, band.escalate());
    let mut found: Vec<Found> = Vec::new();
    let ends = [a, b].map(|t| pair.path.at(point(t)));
    let edges = [du, dv];
    let (interior_u, interior_v) = (&us[1..us.len() - 1], &vs[1..vs.len() - 1]);
    let pieces = blocks.iter().map(|&x| (x, x));
    let walked = bisect(pieces, |(x, block): (Cell, Cell)| {
        if found.iter().any(|f| f.holds(x)) {
            return Step::Settled;
        }
        let jac = pair.jacobian(x);
        let fm = pair.at(x.mid());
        if matches!(
            decide(
                PAST_THE_BAND,
                Margin::of(norm(pair.residual(x, &jac, fm))),
                band
            ),
            Ok(Sign::Positive)
        ) {
            return Step::Settled;
        }
        let (k, length) = pair.widest(x);
        let at_floor = !(length > floor);
        // Read on `x`, then on `x` grown inside its block by its own sides
        // (and, at the floor, by four times them), so a meeting beside `x`
        // is isolated before its neighbours are walked to the floor.
        let grown = |by: f64| {
            Cell(core::array::from_fn(|k| {
                let w = (x.0[k].1 - x.0[k].0) * by;
                (
                    (x.0[k].0 - w).max(block.0[k].0),
                    (x.0[k].1 + w).min(block.0[k].1),
                )
            }))
        };
        let reaches: &[f64] = if at_floor { &[1.0, 4.0] } else { &[1.0] };
        let mut proof = pair.krawczyk_with(x, jac, fm).map(|k| (x, k));
        for &by in reaches {
            if proof.is_some() {
                break;
            }
            let g = grown(by);
            proof = pair.krawczyk(g).map(|k| (g, k));
        }
        // A box the test proves holds one solution; one whose narrowed box
        // lies in a box already proved is that meeting's, the solution
        // there being the only one.
        if let Some((proved, k)) = proof {
            let root = pair.narrow(k);
            match found.iter_mut().find(|f| f.holds(root)) {
                Some(f) => f.isolating.push(proved),
                None => {
                    let mut isolating = vec![proved];
                    isolating.extend(pair.grown(root, proved, block, band));
                    found.push(Found { root, isolating });
                }
            }
            if found.iter().any(|f| f.holds(x)) {
                return Step::Settled;
            }
        }
        if !at_floor {
            let [lo, hi] = x.halves(k);
            return Step::Halve([(lo, block), (hi, block)]);
        }
        // A box on the chart's edge within reach of an end of the span is
        // that end's: the end lies within the band of the chart's edge,
        // which is the boundary's to read.
        let on_edge = (1..3).any(|k| x.0[k].0 <= edges[k - 1].0 || x.0[k].1 >= edges[k - 1].1);
        let c = pair.path.at(x.interval(0));
        let near_end = ends.iter().any(|e| {
            norm(core::array::from_fn(|k| c[k] - e[k])).hi() <= END_REACH * band.escalate()
        });
        if on_edge && near_end {
            return Step::Settled;
        }
        // Within the band at the floor, and no meeting there: a pass
        // within the band, a tangency, or a meeting on a crease.
        let touches =
            |side: (f64, f64), cuts: &[f64]| cuts.iter().any(|&k| side.0 <= k && k <= side.1);
        Step::Stop(
            if touches(x.0[1], interior_u) || touches(x.0[2], interior_v) {
                Unsettled::Crease
            } else {
                Unsettled::Tangency
            },
        )
    });
    match walked {
        Err(geom_core::bisect::Exhausted) => return Err(Unsettled::Exhausted.into()),
        Ok(Some(why)) => return Err(why.into()),
        Ok(None) => {}
    }
    found.sort_by(|p, q| p.root.0[0].0.total_cmp(&q.root.0[0].0));
    let creases: Vec<(bool, f64)> = interior_u
        .iter()
        .map(|&k| (true, k))
        .chain(interior_v.iter().map(|&k| (false, k)))
        .collect();
    let mut roots = Vec::with_capacity(found.len());
    for f in &found {
        // A meeting past the span, or at its end, is the end's, and is
        // no meeting to read against a crease.
        if f.root.0[0].1 > a && f.root.0[0].0 < b {
            for &(along_u, k) in &creases {
                crease_apart(&pair, f.root, (along_u, k), (du, dv), band)?;
            }
        }
        roots.push(f.root.0);
    }
    Ok(Isolated {
        span: (a, b),
        roots,
    })
}

/// The chart's parameters along `u` (`along_u`) or `v` at which the walk
/// is cut: its domain's ends and its creases strictly inside.
fn cuts<T: CertifiedBounds>(
    chart: &NurbsSurface<T>,
    along_u: bool,
    (lo, hi): (f64, f64),
) -> Result<Vec<f64>, SplineFaceRefusal> {
    let knots = if along_u {
        chart.knot_values_u()
    } else {
        chart.knot_values_v()
    };
    let mut values = Vec::new();
    for (k, _) in knots.runs() {
        let k = Interval::from_certified(k);
        if !k.is_certified() {
            return Err(SplineFaceRefusal::Unread(
                "a knot of the chart does not certify",
            ));
        }
        values.push(k.lo());
    }
    let mut out = vec![lo];
    out.extend(
        knots
            .structure()
            .crease_runs()
            .map(|r| values[r])
            .filter(|&k| lo < k && k < hi),
    );
    out.push(hi);
    Ok(out)
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

/// Whether the meeting `root` stands past the band of the crease's
/// iso-curve at `k` along `u` (`along_u`) or `v`: the iso-curve read in
/// windows of the chart, each a first-order box, halved until every one
/// is decided past the band of the carrier's box over `root`'s
/// parameters.
///
/// # Errors
///
/// [`Unsettled::Crease`] where a window is decided within the band of
/// it, [`SplineFaceRefusal::Escalated`] where a window at the floor is
/// neither, and [`Unsettled::Exhausted`] past the budget.
fn crease_apart<T: CertifiedBounds>(
    pair: &Pair<'_, T>,
    root: Cell,
    (along_u, k): (bool, f64),
    (du, dv): ((f64, f64), (f64, f64)),
    band: Band,
) -> Result<(), SplineFaceRefusal> {
    let c = pair.path.at(root.interval(0));
    let (lo, hi) = if along_u { dv } else { du };
    let speed = pair.speeds[if along_u { 2 } else { 1 }];
    let walked = bisect([(lo, hi)], |(s0, s1): (f64, f64)| {
        let w = if along_u {
            UvWindow::new(k, k, s0, s1)
        } else {
            UvWindow::new(s0, s1, k, k)
        };
        let Some(w) = w else {
            return Step::Stop(Err(SplineFaceRefusal::Unread("a crease window is empty")));
        };
        let s = pair.boxes.rect_box(w);
        let f = [c[0] - s.x, c[1] - s.y, c[2] - s.z];
        let d = norm(f);
        match decide("spline_face_root_at_crease", Margin::of(d), band) {
            Ok(Sign::Positive) => Step::Settled,
            Ok(_) => Step::Stop(Err(Unsettled::Crease.into())),
            Err(_) if (s1 - s0) * speed <= band.escalate() && d.lo() <= band.zero() => {
                Step::Stop(Err(Unsettled::Crease.into()))
            }
            Err(diag) if (s1 - s0) * speed <= band.escalate() => {
                Step::Stop(Err(SplineFaceRefusal::Escalated(diag)))
            }
            Err(_) => {
                let m = s0 + (s1 - s0) * 0.5;
                Step::Halve([(s0, m), (m, s1)])
            }
        }
    });
    match walked {
        Err(geom_core::bisect::Exhausted) => Err(Unsettled::Exhausted.into()),
        Ok(Some(refusal)) => refusal,
        Ok(None) => Ok(()),
    }
}

#[cfg(test)]
mod rows;
