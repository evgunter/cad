//! **A line against a spline patch** ([`super::Door::SplineFace`]'s line
//! arm): the line's crossings of the chart, each held in a cell of the
//! chart it alone crosses, read by signs of Bernstein coefficients at the
//! scalar.
//!
//! # The residuals
//!
//! The line `q + t·d` is the meet of two planes through it, with unit
//! normals `n₁`, `n₂` and `n₁ × n₂ = d`. Each plane's residual composed
//! with the chart, `rᵢ = nᵢ·(S − q)`, and the along-line offset
//! `a = d·(S − q)`, are linear in the chart's points: on a cell of the
//! chart's Bézier cut (`crate::pcurve_cache::spline_incidence::Cut`,
//! held about `q`), each is a rational Bernstein form whose coefficients
//! are `nᵢ·Pⱼₖ`, `d·Pⱼₖ` (`Pⱼₖ` the cell's control points less `q`), in
//! metres. Every weight shown positive, each lies in the hull of its
//! coefficients over the cell (the convex-hull property), which is what
//! makes every verdict below sound. The coefficients are computed at the
//! scalar: reals at `f64` with no pad or charge, enclosures at an
//! interval scalar ([`geom_core::Decide`], DESIGN `[simple.conditioned]`).
//!
//! # The cells
//!
//! The cut's cells are halved across their longer way, and one whose
//! boundary passes too near the line to be read is cut at three eighths
//! instead, off the halving's grid (an aimed ray aims at a chart's
//! middle). Each is decided:
//!
//! - **No crossing** where every coefficient of one residual is decided
//!   past the band on one side (`spline_face_line_residual`): the
//!   residual is past the band over the cell, so no point of it is on
//!   the line; or every along-line coefficient is decided past the band
//!   behind the span's start or ahead of its end
//!   (`spline_face_line_reach`).
//! - **At most one crossing** where the residuals' gradients lie in
//!   cones decided apart (`spline_face_line_cone`): the hulls of the
//!   quotient-rule numerators `w·∂N − N·∂w` (the gradient times `w²`,
//!   whose sign it is since `w > 0`) bound each gradient to a box, and
//!   the determinant of every pair of box corners is proved one side of
//!   zero, exactly: the cut's own inequality, which no tolerance
//!   governs ([`geom_core::k_stats::prove`]). The determinant is
//!   multilinear in the corners, so it holds that sign over the boxes,
//!   and two points of the cell the map `(r₁, r₂)` sends to one value
//!   would need a gradient of each normal to their chord, parallel, so
//!   a determinant of zero: the map is one-to-one on the cell. Its sign
//!   is the sign of `d·(S_u × S_v)` there, the crossing's incidence.
//! - **Exactly one** where, one-to-one, the image of the cell's boundary
//!   under `(r₁, r₂)` winds round the origin, read by the parity of a
//!   ray from it ([`boundary_parity`]): the boundary's four rows, each a
//!   rational Bézier curve in the plane, crossing the ray an odd number
//!   of times. A row is read off its coefficients' signs alone, each
//!   proved exactly (a cell's side is the cut's, not the geometry's): clear
//!   of the ray where its offset across it is of one sign or its reach
//!   along it negative, crossed as its ends' sides say where its reach is
//!   positive, halved otherwise (`spline_face_line_winding`). One-to-one, the
//!   boundary is a Jordan curve and the cell its inside, so an odd count
//!   is the one crossing, and an even one none.
//! - A crossing found is narrowed, half by half, until the along-line
//!   coefficients' spread is not decided past the band
//!   (`spline_face_line_narrow`): at most the band's escalation, so the
//!   line's parameter at the middle is within half of it of the
//!   crossing's, and the cell's rectangle holds it.
//! - **Within the band**: a cell not shown one-to-one whose every
//!   residual coefficient is within the band of the line is a graze or a
//!   crease no halving reads, a stretch at once.
//! - **Anything else** at the floor (a cell no wider than half the band's
//!   zero, or [`CELL_DEPTH`] halvings) is a stretch of the line the arm
//!   does not read: a graze, a crease the residuals turn on, two
//!   crossings within the band of each other, a crossing within the band
//!   of a cell's side, or a reading on the band's gap. It is never
//!   counted. A decision in the gap is the stretch's diagnostics; a cell
//!   whose weights are not shown positive is a refusal of the whole
//!   reading.
//!
//! The crossings are found where the chart is, with no reading of the
//! face's trim: the trim is the reader's, on each crossing's rectangle.
//! A crossing within the band of the span's ends, or of another, is read
//! by the table's placement ([`super::cross::place`]).

use geom::NurbsSurface;
use geom_core::k_stats::{decide, prove};
use geom_core::{Band, Decide, Indeterminate, Margin, Point3, Real, Sign, Vec3};

use crate::curve_pair::MeetFault;
use crate::pcurve_cache::spline_incidence::{Cell, Control, Cut, Fraction};
use crate::roots::Unsettled;

/// A residual's coefficient against the band: whether a cell's points
/// all stand off one plane through the line.
const RESIDUAL: &str = "spline_face_line_residual";
/// An along-line coefficient against the span's ends.
const REACH: &str = "spline_face_line_reach";
/// A corner determinant of the gradients' boxes, levered by the cell's
/// extent: whether the cell is one-to-one onto the residuals' plane.
const CONE: &str = "spline_face_line_cone";
/// A boundary row's coefficient against the parity ray.
const WINDING: &str = "spline_face_line_winding";
/// A found crossing's along-line coefficients' spread against the band.
const NARROW: &str = "spline_face_line_narrow";

/// How many times a cell is halved at most.
pub const CELL_DEPTH: usize = 64;
/// The cells one reading may visit.
pub const CELL_VISITS: usize = 1 << 14;
/// The cuts a found crossing's cell is narrowed by, in the order tried:
/// across its longer way at the middle, across its shorter way, then the
/// same at three eighths.
const CUTS: [(Fraction, bool); 4] = [
    (Fraction::Half, false),
    (Fraction::Half, true),
    (Fraction::ThreeEighths, false),
    (Fraction::ThreeEighths, true),
];

/// How many times a boundary row is halved at most.
const ROW_DEPTH: usize = 48;

/// **One crossing of the line with the chart** ([`read`]).
#[derive(Clone, Copy, Debug)]
pub struct PatchCrossing<T> {
    /// The line's parameters the crossing's cell spans along it: its
    /// along-line coefficients' least and greatest, within the band's
    /// escalation of each other.
    pub advance: (T, T),
    /// The chart rectangle `[u0, u1, v0, v1]` of the cell that holds it,
    /// and no other crossing.
    pub rect: [f64; 4],
    /// The sign of `d·(S_u × S_v)` over the cell: which way the line
    /// passes the chart's orientation.
    pub incidence: Sign,
}

/// **A stretch of the line the arm did not read** ([`read`]): the line's
/// parameters an unread cell spans, and why.
#[derive(Clone, Copy, Debug)]
pub struct Unread<T> {
    /// The cell's along-line coefficients' least and greatest.
    pub window: (T, T),
    /// Why.
    pub fault: MeetFault,
}

/// **The arm's refusal** ([`read`]).
#[derive(Clone, Debug)]
pub enum PatchRefusal<T> {
    /// The crossings found outside every unread stretch, and the
    /// stretches.
    Stretches {
        /// The crossings.
        found: Vec<PatchCrossing<T>>,
        /// The unread stretches.
        unread: Vec<Unread<T>>,
    },
    /// The chart does not cut to Bézier cells, a weight is not shown
    /// positive, or the walk ran out of visits: nothing is read.
    Whole(MeetFault),
}

/// The line's frame: its origin, its unit direction and the two unit
/// normals with `n₁ × n₂ = d`.
#[derive(Clone, Copy)]
struct Frame<T: Real> {
    d: Vec3<T>,
    n1: Vec3<T>,
    n2: Vec3<T>,
}

impl<T: Decide> Frame<T> {
    fn of(line: &geom::Line<T>) -> Self {
        let d = line.dir.get();
        // The world axis the line is least along: a choice of plane,
        // which decides nothing.
        let mag = [d.x, d.y, d.z].map(|c| c.abs().value_channel());
        let k = (0..3)
            .min_by(|&a, &b| mag[a].total_cmp(&mag[b]))
            .unwrap_or(0);
        let axis = match k {
            0 => Vec3::new(T::one(), T::zero(), T::zero()),
            1 => Vec3::new(T::zero(), T::one(), T::zero()),
            _ => Vec3::new(T::zero(), T::zero(), T::one()),
        };
        let n1 = d.cross(axis).normalize();
        Self {
            d,
            n1,
            n2: d.cross(n1),
        }
    }
}

/// A cell's control points out of homogeneous form, about the line's
/// origin.
fn points<T: Real>(net: &[Control<T>]) -> Vec<Vec3<T>> {
    net.iter()
        .map(|(h, _)| Vec3::new(h[0] / h[3], h[1] / h[3], h[2] / h[3]))
        .collect()
}

/// Every value decided past the band on one side under `name`: that
/// side, or `None`, with the first decision in the gap.
fn past_band<T: Decide>(
    name: &'static str,
    values: impl Iterator<Item = T>,
    band: Band,
    pending: &mut Option<Indeterminate>,
) -> Option<Sign> {
    let mut side = None;
    for x in values {
        match decide(name, Margin::of(x), band) {
            Ok(s @ (Sign::Positive | Sign::Negative)) if side.is_none_or(|k| k == s) => {
                side = Some(s);
            }
            Ok(_) => return None,
            Err(diag) => {
                pending.get_or_insert(diag);
                return None;
            }
        }
    }
    side
}

/// The least and greatest of `values`.
fn hull<T: Real>(values: impl Iterator<Item = T>) -> Option<(T, T)> {
    values.fold(None, |acc, x| match acc {
        None => Some((x, x)),
        Some((lo, hi)) => Some((lo.min(x), hi.max(x))),
    })
}

/// A tensor-product Bernstein form's coefficients, row-major in `u`, of
/// degree `(du, dv)`.
#[derive(Clone)]
struct Form<T> {
    du: usize,
    dv: usize,
    c: Vec<T>,
}

impl<T: Real> Form<T> {
    fn at(&self, i: usize, j: usize) -> T {
        self.c[i * (self.dv + 1) + j]
    }

    /// The derivative along `u` (`along_u`) or `v`, over the cell's own
    /// unit parameter: degree one lower that way.
    fn derivative(&self, along_u: bool) -> Self {
        let (du, dv) = if along_u {
            (self.du - 1, self.dv)
        } else {
            (self.du, self.dv - 1)
        };
        let degree = if along_u { self.du } else { self.dv };
        let n = T::from_f64(degree as f64);
        let mut c = Vec::with_capacity((du + 1) * (dv + 1));
        for i in 0..=du {
            for j in 0..=dv {
                let (hi, lo) = if along_u {
                    (self.at(i + 1, j), self.at(i, j))
                } else {
                    (self.at(i, j + 1), self.at(i, j))
                };
                c.push(n * (hi - lo));
            }
        }
        Self { du, dv, c }
    }

    /// The product's coefficients, of the degrees' sums: each the sum of
    /// the factors' products weighted by binomials, over the product's.
    fn times(&self, other: &Self) -> Self {
        let (du, dv) = (self.du + other.du, self.dv + other.dv);
        let mut c = vec![T::zero(); (du + 1) * (dv + 1)];
        for i in 0..=self.du {
            for j in 0..=self.dv {
                for k in 0..=other.du {
                    for l in 0..=other.dv {
                        let w = binomial(self.du, i)
                            * binomial(other.du, k)
                            * binomial(self.dv, j)
                            * binomial(other.dv, l);
                        let at = (i + k) * (dv + 1) + (j + l);
                        c[at] = c[at] + T::from_f64(w) * self.at(i, j) * other.at(k, l);
                    }
                }
            }
        }
        for i in 0..=du {
            for j in 0..=dv {
                let at = i * (dv + 1) + j;
                let over = binomial(du, i) * binomial(dv, j);
                c[at] = c[at] / T::from_f64(over);
            }
        }
        Self { du, dv, c }
    }

    fn less(&self, other: &Self) -> Self {
        Self {
            du: self.du,
            dv: self.dv,
            c: self.c.iter().zip(&other.c).map(|(&a, &b)| a - b).collect(),
        }
    }
}

/// `n` choose `k`, exact in `f64` at every degree a cell reads.
fn binomial(n: usize, k: usize) -> f64 {
    (0..k).fold(1.0, |acc, i| acc * (n - i) as f64 / (i + 1) as f64)
}

/// What [`read`] makes of one cell.
enum Verdict {
    /// No crossing in it.
    Clear,
    /// One crossing, of this incidence.
    One(Sign),
    /// One-to-one, its boundary within the band of the line.
    Near,
    /// Undecided, and every point of it within the band of the line.
    Within,
    /// Not decided.
    Open,
    /// A weight not shown positive.
    Unhulled,
}

/// The walk over the chart's cells ([`read`]).
struct Walk<T: Real> {
    frame: Frame<T>,
    span: (T, T),
    pu: usize,
    pv: usize,
    band: Band,
}

impl<T: Decide> Walk<T> {
    /// The cell's verdict (module docs), with the first decision in the
    /// gap.
    /// `reach` asks the span's ends as well: a cell wholly behind the
    /// start or past the end is clear of the span, though the line may
    /// cross it.
    ///
    /// `known` is the incidence of a cell already shown one-to-one that
    /// holds this one, which is one-to-one with it: its cone test is not
    /// read again.
    fn verdict(
        &self,
        cell: &Cell<T>,
        reach: bool,
        known: Option<Sign>,
        pending: &mut Option<Indeterminate>,
    ) -> Verdict {
        if !cell.net.iter().all(|(h, _)| h[3].structure_positive()) {
            return Verdict::Unhulled;
        }
        let p = points(&cell.net);
        let Frame { d, n1, n2 } = self.frame;
        let band = self.band;
        for n in [n1, n2] {
            if past_band(RESIDUAL, p.iter().map(|x| x.dot(n)), band, pending).is_some() {
                return Verdict::Clear;
            }
        }
        let (t0, t1) = self.span;
        let lo = t0.min(t1);
        let hi = t0.max(t1);
        if reach
            && past_band(REACH, p.iter().map(|x| x.dot(d) - lo), band, pending)
                == Some(Sign::Negative)
            || reach
                && past_band(REACH, p.iter().map(|x| x.dot(d) - hi), band, pending)
                    == Some(Sign::Positive)
        {
            return Verdict::Clear;
        }
        // Not clear, so each residual changes sign over the cell's net, and
        // one of them within the band over all of it: a point of the
        // chart on one plane within the band of the other, so within the
        // band of the line. A graze, a crease, or a crossing no cut of
        // this size reads.
        let within = || {
            [n1, n2].into_iter().any(|n| {
                p.iter().all(|x| {
                    !matches!(
                        decide(RESIDUAL, Margin::of(x.dot(n)), band),
                        Ok(Sign::Positive | Sign::Negative)
                    )
                })
            })
        };
        let undecided = |near: Verdict| if within() { Verdict::Within } else { near };
        let Some(incidence) = known.or_else(|| self.one_to_one(cell)) else {
            return undecided(Verdict::Open);
        };
        match boundary_parity(self, cell) {
            Some(true) => Verdict::One(incidence),
            Some(false) => Verdict::Clear,
            None => undecided(Verdict::Near),
        }
    }

    /// **The cone test** (module docs): the sign of the corner
    /// determinants where every one is decided past the band on it.
    fn one_to_one(&self, cell: &Cell<T>) -> Option<Sign> {
        let (pu, pv) = (self.pu, self.pv);
        let Frame { n1, n2, .. } = self.frame;
        // Weights scaled to at most one, so each numerator is at most the
        // gradient in metres over the cell.
        let top = cell
            .net
            .iter()
            .map(|(h, _)| h[3])
            .fold(T::zero(), |m, w| m.max(w));
        let form = |f: &dyn Fn(&[T; 4]) -> T| Form {
            du: pu,
            dv: pv,
            c: cell.net.iter().map(|(h, _)| f(h) / top).collect(),
        };
        let w = form(&|h| h[3]);
        let numerators = |n: Vec3<T>| {
            let r = form(&|h| n.x * h[0] + n.y * h[1] + n.z * h[2]);
            [true, false].map(|along_u| {
                w.times(&r.derivative(along_u))
                    .less(&r.times(&w.derivative(along_u)))
            })
        };
        let boxes = |n: Vec3<T>| numerators(n).map(|g| hull(g.c.into_iter()));
        let ([Some(a_u), Some(a_v)], [Some(b_u), Some(b_v)]) = (boxes(n1), boxes(n2)) else {
            return None;
        };
        let mut sign = None;
        for au in [a_u.0, a_u.1] {
            for av in [a_v.0, a_v.1] {
                for bu in [b_u.0, b_u.1] {
                    for bv in [b_v.0, b_v.1] {
                        let det = au * bv - av * bu;
                        match prove(CONE, Margin::sign_of(det)) {
                            Ok(s @ (Sign::Positive | Sign::Negative))
                                if sign.is_none_or(|k| k == s) =>
                            {
                                sign = Some(s);
                            }
                            _ => return None,
                        }
                    }
                }
            }
        }
        sign
    }

    /// **A near cell grown about its crossing** (module docs): one-to-one
    /// and its boundary within the band of the line, its crossing, if any,
    /// lies near its side, so the cell is grown by half its size each way
    /// (inside the cut's cell `top` it was cut from, and clear of every
    /// claimed cell) and read again.
    fn grown(
        &self,
        top: &Cell<T>,
        cell: &Cell<T>,
        claims: &[[f64; 4]],
        pending: &mut Option<Indeterminate>,
    ) -> Grown<T> {
        let [u0, u1, v0, v1] = cell.rect;
        let (du, dv) = ((u1 - u0) / 2.0, (v1 - v0) / 2.0);
        let [a0, a1, b0, b1] = top.rect;
        let rect = [
            (u0 - du).max(a0),
            (u1 + du).min(a1),
            (v0 - dv).max(b0),
            (v1 + dv).min(b1),
        ];
        if rect == cell.rect || claims.iter().any(|c| overlap(rect, *c).is_some()) {
            return Grown::Open;
        }
        let grown = top.restricted(self.pu, self.pv, rect);
        match self.verdict(&grown, true, None, pending) {
            Verdict::One(incidence) => Grown::One(grown, incidence),
            Verdict::Clear => Grown::Clear,
            Verdict::Unhulled => Grown::Unhulled,
            Verdict::Near | Verdict::Within | Verdict::Open => Grown::Open,
        }
    }

    /// Whether `cell` is shown to hold no crossing: decided clear, or
    /// one-to-one with a boundary that does not wind round the line.
    fn holds_none(&self, cell: &Cell<T>, pending: &mut Option<Indeterminate>) -> bool {
        matches!(self.verdict(cell, false, None, pending), Verdict::Clear)
    }

    /// The along-line coefficients' least and greatest.
    fn advance(&self, cell: &Cell<T>) -> (T, T) {
        let d = self.frame.d;
        hull(points(&cell.net).iter().map(|x| x.dot(d))).unwrap_or_else(|| {
            let nan = T::from_f64(f64::NAN);
            (nan, nan)
        })
    }

    /// The cell's extent: the diagonal of its control points' box, on
    /// their brackets' middles (a place, deciding nothing); NaN for the
    /// poison.
    fn extent(cell: &Cell<T>) -> f64 {
        let (mut lo, mut hi) = ([f64::INFINITY; 3], [f64::NEG_INFINITY; 3]);
        for x in points(&cell.net) {
            for (k, c) in [x.x, x.y, x.z].into_iter().enumerate() {
                let c = c.structure_bracket().map_or(f64::NAN, geom::bracket_mid);
                lo[k] = lo[k].min(c);
                hi[k] = hi[k].max(c);
            }
        }
        (0..3).map(|k| (hi[k] - lo[k]).powi(2)).sum::<f64>().sqrt()
    }

    /// The cell cut at `fraction` across its longer way, or its shorter
    /// (`across`).
    fn halves(&self, cell: &Cell<T>, fraction: Fraction, across: bool) -> (Cell<T>, Cell<T>) {
        let longer = cell.longer_along_u(self.pu, self.pv);
        cell.split_at(self.pu, self.pv, longer != across, fraction)
    }

    /// **A found crossing narrowed** (module docs): cut until its
    /// along-line coefficients' spread is not decided past the band,
    /// keeping the part that holds it, or until no part can be shown to;
    /// `Err` where two parts read as holding it, or neither, which the
    /// cell's one crossing contradicts.
    fn narrowed(
        &self,
        mut cell: Cell<T>,
        incidence: Sign,
        pending: &mut Option<Indeterminate>,
    ) -> Result<Cell<T>, Cell<T>> {
        loop {
            let (lo, hi) = self.advance(&cell);
            if decide(NARROW, Margin::of(hi - lo), self.band) != Ok(Sign::Positive)
                || cell.depth >= CELL_DEPTH
            {
                return Ok(cell);
            }
            // The crossing is in one part; a part decided clear of the
            // line (its span's ends not asked: the crossing may lie past
            // them), or shown to hold none, leaves it in the other. A crossing within the
            // band of the cut leaves both undecided, and the cell is cut
            // elsewhere: the other way, then off the halving's grid.
            let held = |c: &Cell<T>, pending: &mut Option<Indeterminate>| match self.verdict(
                c,
                false,
                Some(incidence),
                pending,
            ) {
                Verdict::One(_) => Some(true),
                Verdict::Clear => Some(false),
                _ => None,
            };
            let mut next = None;
            for (fraction, across) in CUTS {
                let (a, b) = self.halves(&cell, fraction, across);
                next = match (held(&a, pending), held(&b, pending)) {
                    (Some(true) | None, Some(false)) | (Some(true), None) => Some(a),
                    (Some(false), Some(true) | None) | (None, Some(true)) => Some(b),
                    (None, None) => continue,
                    (Some(true), Some(true)) | (Some(false), Some(false)) => return Err(cell),
                };
                break;
            }
            match next {
                Some(part) => cell = part,
                None => return Ok(cell),
            }
        }
    }
}

/// **The parity of the boundary's image about the origin** (module
/// docs): `Some` where every row is read clear of the ray or crossing it
/// as its ends say, along one of the four axis rays of the residuals'
/// plane; `None` where none reads so.
fn boundary_parity<T: Decide>(walk: &Walk<T>, cell: &Cell<T>) -> Option<bool> {
    let band = walk.band;
    let (pu, pv) = (walk.pu, walk.pv);
    let Frame { n1, n2, .. } = walk.frame;
    let at = |i: usize, j: usize| {
        let h = cell.net[i * (pv + 1) + j].0;
        [
            n1.x * h[0] + n1.y * h[1] + n1.z * h[2],
            n2.x * h[0] + n2.y * h[1] + n2.z * h[2],
            h[3],
        ]
    };
    // The boundary in order: v = v0 forward in u, u = u1 forward in v,
    // v = v1 back in u, u = u0 back in v.
    let rows: [Vec<[T; 3]>; 4] = [
        (0..=pu).map(|i| at(i, 0)).collect(),
        (0..=pv).map(|j| at(pu, j)).collect(),
        (0..=pu).rev().map(|i| at(i, pv)).collect(),
        (0..=pv).rev().map(|j| at(0, j)).collect(),
    ];
    // The ray along +x in each frame `(x, y)` of the plane.
    let frames: [fn([T; 3]) -> [T; 3]; 4] = [
        |[a, b, w]| [a, b, w],
        |[a, b, w]| [b, T::zero() - a, w],
        |[a, b, w]| [T::zero() - a, T::zero() - b, w],
        |[a, b, w]| [T::zero() - b, a, w],
    ];
    'frame: for turn in frames {
        let mut parity = false;
        for row in &rows {
            let row: Vec<[T; 3]> = row.iter().map(|&h| turn(h)).collect();
            match row_parity(&row, band, 0) {
                Some(odd) => parity ^= odd,
                None => continue 'frame,
            }
        }
        return Some(parity);
    }
    None
}

/// How often a rational row `(x, y, w)` crosses the ray `y = 0, x > 0`,
/// mod 2; `None` where it is not read.
fn row_parity<T: Decide>(row: &[[T; 3]], band: Band, depth: usize) -> Option<bool> {
    let x = || row.iter().map(|h| h[0] / h[2]);
    let y = || row.iter().map(|h| h[1] / h[2]);
    // A decision in the gap is the cut's, not the line's: it reads
    // nothing, and is not reported.
    let side = |values: &mut dyn Iterator<Item = T>| past_band(WINDING, values, band, &mut None);
    if side(&mut y()).is_some() || side(&mut x()) == Some(Sign::Negative) {
        return Some(false);
    }
    if side(&mut x()) == Some(Sign::Positive) {
        let end = |h: &[T; 3]| decide(WINDING, Margin::of(h[1] / h[2]), band);
        if let (Some(first), Some(last)) = (row.first(), row.last()) {
            match (end(first), end(last)) {
                (
                    Ok(a @ (Sign::Positive | Sign::Negative)),
                    Ok(b @ (Sign::Positive | Sign::Negative)),
                ) => {
                    return Some(a != b);
                }
                _ => return None,
            }
        }
    }
    if depth >= ROW_DEPTH {
        return None;
    }
    // De Casteljau at the half, in homogeneous form.
    let n = row.len() - 1;
    let half = T::from_f64(0.5);
    let mut work = row.to_vec();
    let (mut left, mut right) = (Vec::with_capacity(n + 1), Vec::with_capacity(n + 1));
    for level in 0..=n {
        left.push(work[0]);
        right.push(work[n - level]);
        for i in 0..n - level {
            work[i] = core::array::from_fn(|c| work[i][c] + (work[i + 1][c] - work[i][c]) * half);
        }
    }
    right.reverse();
    let a = row_parity(&left, band, depth + 1)?;
    let b = row_parity(&right, band, depth + 1)?;
    Some(a ^ b)
}

/// **The line's crossings of `chart` over the line's parameters `span`**
/// (module docs), ascending by no order: the table's placement orders
/// them.
///
/// # Errors
///
/// [`PatchRefusal`]: the stretches not read, with the crossings outside
/// them, or the whole reading.
pub fn read<T: Decide>(
    line: &geom::Line<T>,
    span: (T, T),
    chart: &NurbsSurface<T>,
    band: Band,
) -> Result<Vec<PatchCrossing<T>>, PatchRefusal<T>> {
    read_measured(line, span, chart, band, &mut Measure::default())
}

/// **What one reading cost** ([`read_measured`]): the cells it visited,
/// the deepest it halved one, the deepest it narrowed a crossing to.
#[derive(Clone, Copy, Debug, Default)]
pub struct Measure {
    /// Cells visited.
    pub visits: usize,
    /// The deepest cell visited.
    pub depth: usize,
}

/// [`read`], counting what it cost into `measure`.
///
/// # Errors
///
/// As [`read`]'s.
pub fn read_measured<T: Decide>(
    line: &geom::Line<T>,
    span: (T, T),
    chart: &NurbsSurface<T>,
    band: Band,
    measure: &mut Measure,
) -> Result<Vec<PatchCrossing<T>>, PatchRefusal<T>> {
    let o = line.origin;
    let about = Vec3::new(o.x, o.y, o.z);
    let Some(Cut { cells, pu, pv }) = Cut::of(chart, about, false) else {
        return Err(PatchRefusal::Whole(MeetFault::Unsupported {
            pose: "a spline chart that does not cut to Bézier cells",
        }));
    };
    let walk = Walk {
        frame: Frame::of(line),
        span,
        pu,
        pv,
        band,
    };
    // Each cell with the index of the cut's cell it was cut from, which a
    // grown cell is restricted from.
    let mut stack: Vec<(Cell<T>, usize)> =
        cells.iter().cloned().zip(0..cells.len()).rev().collect();
    let mut claims: Vec<[f64; 4]> = Vec::new();
    let (mut found, mut unread) = (Vec::new(), Vec::new());
    let mut visits = 0usize;
    while let Some((cell, top)) = stack.pop() {
        visits += 1;
        measure.visits += 1;
        measure.depth = measure.depth.max(cell.depth);
        if visits > CELL_VISITS {
            return Err(PatchRefusal::Whole(MeetFault::Unsettled(
                Unsettled::Exhausted,
            )));
        }
        // A cell inside a claimed one holds no crossing but its claim's.
        if claims.iter().any(|c| inside(cell.rect, *c)) {
            continue;
        }
        let mut pending = None;
        let (cell, incidence) = match walk.verdict(&cell, true, None, &mut pending) {
            Verdict::Clear => continue,
            Verdict::Unhulled => return Err(unhulled()),
            Verdict::One(incidence) => (cell, incidence),
            Verdict::Within => {
                unread.push(stretch(&walk, &cell, pending, Unsettled::Tangency));
                continue;
            }
            Verdict::Near => match walk.grown(&cells[top], &cell, &claims, &mut pending) {
                Grown::One(grown, incidence) => (grown, incidence),
                Grown::Clear => continue,
                Grown::Unhulled => return Err(unhulled()),
                Grown::Open => {
                    push_halves(&walk, &mut stack, &mut unread, cell, top, pending);
                    continue;
                }
            },
            Verdict::Open => {
                push_halves(&walk, &mut stack, &mut unread, cell, top, pending);
                continue;
            }
        };
        // The crossing is this cell's alone, and no claimed cell's: where
        // the two overlap, the overlap is shown to hold none.
        if !claims
            .iter()
            .filter_map(|c| overlap(cell.rect, *c))
            .all(|o| walk.holds_none(&cells[top].restricted(pu, pv, o), &mut pending))
        {
            unread.push(stretch(&walk, &cell, pending, Unsettled::Tangency));
            continue;
        }
        claims.push(cell.rect);
        let Ok(narrow) = walk.narrowed(cell.clone(), incidence, &mut pending) else {
            unread.push(stretch(&walk, &cell, pending, Unsettled::Tangency));
            continue;
        };
        measure.depth = measure.depth.max(narrow.depth);
        let advance = walk.advance(&narrow);
        // Inside the span past the band, or wholly outside it; else in
        // band of an end.
        let (t0, t1) = span;
        let (s0, s1) = (t0.min(t1), t0.max(t1));
        let reach = |x: T| decide(REACH, Margin::of(x), band);
        match (reach(advance.0 - s0), reach(advance.1 - s1)) {
            (Ok(Sign::Positive), Ok(Sign::Negative)) => found.push(PatchCrossing {
                advance,
                rect: narrow.rect,
                incidence,
            }),
            _ if matches!(reach(advance.1 - s0), Ok(Sign::Negative))
                || matches!(reach(advance.0 - s1), Ok(Sign::Positive)) => {}
            (a, b) => {
                let gap = a.err().or(b.err());
                unread.push(stretch(
                    &walk,
                    &narrow,
                    gap.or(pending),
                    Unsettled::Tangency,
                ));
            }
        }
    }
    if unread.is_empty() {
        Ok(found)
    } else {
        Err(PatchRefusal::Stretches { found, unread })
    }
}

/// The refusal of a cell whose weights are not shown positive.
fn unhulled<T>() -> PatchRefusal<T> {
    PatchRefusal::Whole(MeetFault::Unsupported {
        pose: "a spline chart cell whose weights are not shown positive",
    })
}

/// `cell` halved onto the stack, or, at the floor (a cell no wider than
/// half the band's zero, or [`CELL_DEPTH`] halvings), a stretch.
fn push_halves<T: Decide>(
    walk: &Walk<T>,
    stack: &mut Vec<(Cell<T>, usize)>,
    unread: &mut Vec<Unread<T>>,
    cell: Cell<T>,
    top: usize,
    pending: Option<Indeterminate>,
) {
    let floor = Walk::<T>::extent(&cell);
    // A NaN extent (the poison) is at the floor.
    let wide = matches!(
        floor.partial_cmp(&(0.5 * walk.band.zero())),
        Some(core::cmp::Ordering::Greater | core::cmp::Ordering::Equal)
    );
    if cell.depth >= CELL_DEPTH || !wide {
        unread.push(stretch(walk, &cell, pending, Unsettled::Floor));
        return;
    }
    let (a, b) = walk.halves(&cell, Fraction::Half, false);
    stack.push((b, top));
    stack.push((a, top));
}

/// Whether rectangle `a` lies inside `b`.
fn inside(a: [f64; 4], b: [f64; 4]) -> bool {
    b[0] <= a[0] && a[1] <= b[1] && b[2] <= a[2] && a[3] <= b[3]
}

/// The overlap of two rectangles, where it has area.
fn overlap(a: [f64; 4], b: [f64; 4]) -> Option<[f64; 4]> {
    let o = [
        a[0].max(b[0]),
        a[1].min(b[1]),
        a[2].max(b[2]),
        a[3].min(b[3]),
    ];
    (o[0] < o[1] && o[2] < o[3]).then_some(o)
}

/// What a near cell grown about its crossing reads ([`Walk::grown`]).
enum Grown<T> {
    /// One crossing in the grown cell, which claims it.
    One(Cell<T>, Sign),
    /// None in the grown cell, so none in the near one.
    Clear,
    /// A weight not shown positive.
    Unhulled,
    /// Nothing read: the near cell is halved.
    Open,
}

/// An unread cell's stretch: its diagnostics where a decision fell in
/// the gap, else the limit `why`.
fn stretch<T: Decide>(
    walk: &Walk<T>,
    cell: &Cell<T>,
    pending: Option<Indeterminate>,
    why: Unsettled,
) -> Unread<T> {
    Unread {
        window: walk.advance(cell),
        fault: pending.map_or(MeetFault::Unsettled(why), MeetFault::Escalated),
    }
}

/// The point of the line at parameter `t`.
#[must_use]
pub fn point_at<T: Real>(line: &geom::Line<T>, t: T) -> Point3<T> {
    line.origin + line.dir.get() * t
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod rows {
    use super::*;
    use core::num::NonZeroUsize;
    use geom_core::UnitVec3;
    use geom_core::spline::Knots;

    fn band() -> Band {
        Band::new(1e-9, 1e-6).unwrap()
    }

    fn knots(p: usize) -> Knots<f64> {
        let m = NonZeroUsize::new(p + 1).unwrap();
        Knots::new(vec![(0.0, m), (1.0, m)], p).unwrap()
    }

    /// The loft prism's flared wall, one cell: degree one across, two up.
    fn wall() -> NurbsSurface<f64> {
        let mut control = Vec::new();
        for y in [-1.0, 1.0] {
            for (x, z) in [(1.0, 0.0), (1.75, 1.0), (1.0, 2.0)] {
                control.push(Point3::new(x, y, z));
            }
        }
        NurbsSurface::new(knots(1), knots(2), control, vec![1.0; 6]).unwrap()
    }

    fn line(o: [f64; 3], d: [f64; 3]) -> geom::Line<f64> {
        geom::Line {
            origin: Point3::new(o[0], o[1], o[2]),
            dir: UnitVec3::new(Vec3::new(d[0], d[1], d[2]), "row", band()).unwrap(),
        }
    }

    #[test]
    fn a_line_across_the_wall_crosses_it_once() {
        let l = line([0.0, 0.1, 1.1], [1.0, 0.0, 0.0]);
        let mut m = Measure::default();
        let got = read_measured(&l, (0.0, 5.0), &wall(), band(), &mut m);
        match &got {
            Ok(found) => {
                assert_eq!(found.len(), 1, "{found:?}");
                // `S_u × S_v` is `+x` on this wall.
                assert_eq!(found[0].incidence, Sign::Positive);
                let [u0, u1, v0, v1] = found[0].rect;
                let (u, v) = (0.55, 0.55);
                assert!(
                    u0 <= u && u <= u1 && v0 <= v && v <= v1,
                    "{:?}",
                    found[0].rect
                );
            }
            Err(e) => panic!("{e:?} after {m:?}"),
        }
    }

    #[test]
    fn a_line_up_the_flare_crosses_it_twice_and_one_past_it_not_at_all() {
        let up = |x: f64| {
            read(
                &line([x, 0.3, -1.0], [0.0, 0.0, 1.0]),
                (0.0, 4.0),
                &wall(),
                band(),
            )
        };
        let found = up(1.2).expect("two transverse crossings");
        let mut signs: Vec<Sign> = found.iter().map(|c| c.incidence).collect();
        signs.sort_by_key(|s| *s == Sign::Positive);
        assert_eq!(signs, [Sign::Negative, Sign::Positive], "{found:?}");
        assert!(up(2.0).expect("a line past the net").is_empty());
        // Behind the span's start: none.
        let behind = read(
            &line([1.2, 0.3, 3.0], [0.0, 0.0, 1.0]),
            (0.0, 4.0),
            &wall(),
            band(),
        );
        assert!(behind.expect("a line leaving the wall").is_empty());
    }

    #[test]
    fn a_line_through_the_cells_middle_crosses_it_once() {
        // The cell's first halvings cut at the middle of both ways, where
        // this line meets the wall: its crossing is read off them.
        let mid = wall().eval(0.5, 0.5);
        let l = line([mid.x - 1.0, mid.y + 0.2, mid.z + 0.1], [1.0, -0.2, -0.1]);
        let found = read(&l, (0.0, 5.0), &wall(), band()).expect("a transverse crossing");
        assert_eq!(found.len(), 1, "{found:?}");
    }

    /// A quarter of the unit cylinder about `z`, rational in `u`, from
    /// `+x` to `+y`, `z` from 0 to 1.
    fn quarter() -> NurbsSurface<f64> {
        let mut control = Vec::new();
        for (x, y) in [(1.0, 0.0), (1.0, 1.0), (0.0, 1.0)] {
            for z in [0.0, 1.0] {
                control.push(Point3::new(x, y, z));
            }
        }
        let h = core::f64::consts::FRAC_1_SQRT_2;
        let weights = vec![1.0, 1.0, h, h, 1.0, 1.0];
        NurbsSurface::new(knots(2), knots(1), control, weights).unwrap()
    }

    #[test]
    fn a_line_into_a_rational_quarter_crosses_it_once_where_the_circle_is() {
        // Along `+x` at y = 0.5: the circle is at x = √0.75.
        let l = line([0.0, 0.5, 0.4], [1.0, 0.0, 0.0]);
        let found = read(&l, (0.0, 3.0), &quarter(), band()).expect("a transverse crossing");
        assert_eq!(found.len(), 1, "{found:?}");
        let (lo, hi) = found[0].advance;
        let x = 0.75f64.sqrt();
        assert!(lo - 1e-6 <= x && x <= hi + 1e-6, "{found:?} against {x}");
        // `S_u × S_v` points out of the cylinder: the line leaves it.
        assert_eq!(found[0].incidence, Sign::Positive);
    }

    #[test]
    fn an_enclosure_reads_the_crossings_a_real_does() {
        use geom_core::Interval;
        let on = |x: f64| <Interval as Real>::from_f64(x);
        let l = geom::Line {
            origin: Point3::new(on(0.0), on(0.5), on(0.4)),
            dir: UnitVec3::new(Vec3::new(on(1.0), on(0.0), on(0.0)), "row", band()).unwrap(),
        };
        let chart = quarter().map_scalar(on);
        let found = read(&l, (on(0.0), on(3.0)), &chart, band()).expect("a transverse crossing");
        assert_eq!(found.len(), 1, "{found:?}");
        assert_eq!(found[0].incidence, Sign::Positive);
        // And the twisted wall's two, one on a halving's cut.
        let (o, d) = (
            [0.9500167984823057, -0.5871610932500297, 2.65580438601809],
            [
                0.14807717914130855,
                0.40131646106520646,
                -0.9038906167759746,
            ],
        );
        let l = geom::Line {
            origin: Point3::new(on(o[0]), on(o[1]), on(o[2])),
            dir: UnitVec3::new(Vec3::new(on(d[0]), on(d[1]), on(d[2])), "row", band()).unwrap(),
        };
        let chart = twisted().map_scalar(on);
        let found = read(&l, (on(0.0), on(5.85)), &chart, band()).expect("two crossings");
        assert_eq!(found.len(), 2, "{found:?}");
    }

    /// The loft prism's twisted wall: flared along its `y = −1` row,
    /// straight along its `y = 1` row.
    fn twisted() -> NurbsSurface<f64> {
        let control = [
            [1.0, -1.0, 0.0],
            [1.75, -1.0, 1.0],
            [1.0, -1.0, 2.0],
            [1.0, 1.0, 0.0],
            [1.0, 1.0, 1.0],
            [1.0, 1.0, 2.0],
        ]
        .map(|[x, y, z]| Point3::new(x, y, z));
        NurbsSurface::new(knots(1), knots(2), control.to_vec(), vec![1.0; 6]).unwrap()
    }

    /// A line through the twisted wall twice, once at `u = 1/2` exactly:
    /// on the first halving's cut, where both halves' boundaries pass
    /// through it. Neither drops it: the near half is grown about it.
    #[test]
    fn a_crossing_on_a_halvings_cut_is_read_once() {
        let l = line(
            [0.9500167984823057, -0.5871610932500297, 2.65580438601809],
            [
                0.14807717914130855,
                0.40131646106520646,
                -0.9038906167759746,
            ],
        );
        let found = read(&l, (0.0, 5.852349955359813), &twisted(), band()).expect("two crossings");
        let mut at: Vec<f64> = found
            .iter()
            .map(|c| (c.advance.0 + c.advance.1) / 2.0)
            .collect();
        at.sort_by(f64::total_cmp);
        // A scan of the wall's `x` against the line's at its `(y, z)`.
        let want = [1.012_368_757, 1.463_087_489];
        assert_eq!(at.len(), 2, "{found:?}");
        for (got, want) in at.iter().zip(want) {
            assert!((got - want).abs() < 1e-4, "{found:?}");
        }
    }

    #[test]
    fn a_line_touching_the_flare_is_not_read() {
        // The flare's crest is at x = 1.375, z = 1.
        let band = band();
        let l = geom::Line {
            origin: Point3::new(1.375, 0.3, -1.0),
            dir: UnitVec3::exact_z(),
        };
        let mut m = Measure::default();
        let got = read_measured(&l, (0.0, 4.0), &wall(), band, &mut m);
        assert!(m.visits < 1000, "the cells along the touch are few: {m:?}");
        assert!(
            matches!(got, Err(PatchRefusal::Stretches { ref found, .. }) if found.is_empty()),
            "{got:?}"
        );
    }
}
