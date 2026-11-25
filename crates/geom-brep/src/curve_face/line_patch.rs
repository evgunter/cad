//! **A line against a spline patch** ([`super::Door::SplineFace`]'s line
//! arm): the line's crossings of the chart, each held in a rectangle of
//! the chart it alone crosses, read by signs of Bernstein coefficients at
//! the scalar.
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
//! # The regions
//!
//! The walk reads **regions**: a rectangle of the chart and the cut's
//! cells over it, each restricted to its part. A region the walk halves
//! is one cell, halved across its longer way; a region grown about a
//! crossing (below) may hold parts of up to four cells, across the cut's
//! knot lines. Each is decided:
//!
//! - **No crossing** where every coefficient of one residual, over every
//!   part, is decided past the band on one side
//!   (`spline_face_past_the_band`): the residual is past the band over
//!   the region, so no point of it is on the line; or every along-line
//!   coefficient is decided past the band behind the span's start or
//!   ahead of its end (`spline_face_past_the_end`).
//! - **At most one crossing** where the residuals' gradients lie in
//!   cones apart (`spline_face_line_cone`): the hulls of the quotient-rule
//!   numerators `w·∂N − N·∂w` (the gradient times `w²`, whose sign it is
//!   since `w > 0`), met over every part, bound each gradient to a box,
//!   and the determinant of every pair of box corners is proved one side
//!   of zero ([`geom_core::k_stats::prove`]): exactly, on the cut's net as
//!   the scalar holds it, the cut's own inequality, which no tolerance
//!   governs. The determinant is multilinear in the corners, so it holds
//!   that sign over the boxes, and over positive multiples of their
//!   points (each part's numerators are its gradients times `w²`, over
//!   its largest weight squared, by its rectangle's sides: all positive).
//!   Two points of the region the map `(r₁, r₂)` sends to one value would
//!   give, on the chord `e` between them (the region is a rectangle, so
//!   it holds the chord), `∫ ∇rᵢ·e = 0` for each residual, continuous
//!   across the parts and smooth on each: the integrals `ḡᵢ = ∫ ∇rᵢ` are
//!   both normal to `e`, so parallel, yet `det(ḡ₁, ḡ₂)` is the double
//!   integral of `det(∇r₁(s), ∇r₂(t))`, every integrand of the boxes' one
//!   sign. The map is one-to-one on the region, and the sign is
//!   `d·(S_u × S_v)` there, the crossing's incidence.
//! - **Exactly one** where, one-to-one, the image of the region's
//!   boundary under `(r₁, r₂)` winds round the origin, read by the parity
//!   of a ray from it ([`boundary_parity`]): the boundary's rows (each
//!   part's rows on the region's sides, each a rational Bézier curve in
//!   the plane) crossing the ray an odd number of times. A row is read off
//!   its coefficients' signs, each decided past the band: clear of the ray
//!   where its offset across it is of one sign or its reach along it
//!   behind, crossed as its ends' sides say where its reach is ahead,
//!   halved otherwise (`spline_face_line_winding`). One-to-one, the
//!   boundary is a Jordan curve and the region its inside, so an odd
//!   count is the one crossing, and an even one none. The boundary is
//!   read past the band, not by exact signs: a crossing on a halving's
//!   cut lies on both halves' boundaries, and an exact sign rounded there
//!   reads it out of both.
//! - **Near**: one-to-one, its boundary within the band of the line. Its
//!   crossing, if any, lies near its side, so it is **grown** by half its
//!   size each way, inside the chart's domain and across the cut's knot
//!   lines, and the grown region is read again. A grown region that holds
//!   a crossing **claims** its rectangle: a cell inside a claim is not
//!   read, and a crossing whose region overlaps a claim is counted only
//!   where the overlap is shown to hold none.
//! - **Within the band**: a region not decided whose every coefficient of
//!   one residual is within the band (the other not of one sign, or it
//!   would be clear) holds a point on one plane within the band of the
//!   other, so within the band of the line: a graze, a crease, or a
//!   crossing no region of this size reads. A stretch, read no further. A
//!   near cell is grown first, and is a stretch only where its growth
//!   reads nothing.
//! - **Anything else** is halved, down to the floor (a cell no wider than
//!   half the band's zero, or [`CELL_DEPTH`] halvings), where it is a
//!   stretch.
//!
//! A crossing found is **narrowed**, part by part (a half, then the other
//! way, then at three eighths; across a grown region's knot lines, a
//! cell's part), until its along-line coefficients' spread is not decided
//! past the band (`spline_face_line_narrow`) or no part can be shown to
//! hold it. A part of a one-to-one region is one-to-one, so its cone is
//! not read again. Its spread is then decided past the band inside the
//! span, or it is a stretch: a crossing within the band of the span's
//! ends is never reported. The line's parameter at its middle is within
//! half the spread of the crossing's, which the reader charges.
//!
//! Nothing in a stretch is counted. A decision in the gap of a residual
//! is the stretch's diagnostics; a cell whose weights are not shown
//! positive is a refusal of the whole reading.
//!
//! The crossings are found where the chart is, with no reading of the
//! face's trim: the trim is the reader's, on each crossing's rectangle.
//! Two crossings within the band of each other are read by the table's
//! placement ([`super::cross::place`]).
//!
//! **A crossing's parameter is the middle of a bracket**, computed at the
//! scalar: at a dual its tangent is the bracket's, not the crossing's. It
//! may be decided on (ordered, placed against the span's ends); it is not
//! a value the body may store.

use geom::NurbsSurface;
use geom_core::k_stats::{decide, prove};
use geom_core::spline::pieces::binomial;
use geom_core::{Band, Decide, Indeterminate, Margin, Point3, Real, Sign, Vec3};

use crate::curve_pair::MeetFault;
use crate::pcurve_cache::spline_incidence::{Cell, Control, Cut, Fraction};
use crate::roots::Unsettled;

/// A residual's coefficient against the band: whether a region's points
/// all stand off one plane through the line. The spline face door's own
/// name for a box's clearance (`spline_face`'s).
const PAST_THE_BAND: &str = "spline_face_past_the_band";
/// An along-line coefficient against the span's ends: whether a region
/// lies wholly past one.
const PAST_THE_END: &str = "spline_face_past_the_end";
/// A corner determinant of the gradients' boxes, proved one sign: whether
/// the region is one-to-one onto the residuals' plane.
const CONE: &str = "spline_face_line_cone";
/// A boundary row's coefficient against the parity ray.
const WINDING: &str = "spline_face_line_winding";
/// A found crossing's along-line coefficients' spread against the band.
const NARROW: &str = "spline_face_line_narrow";

/// How many times a cell is halved at most. The chart's cell walks'
/// knobs differ by what each walk asks: this one halves until a crossing
/// reads or a region is within the band, and narrows a found one to the
/// band's escalation, about forty halvings below a metre-sized cell at
/// ε 1e-9; `spline_incidence`'s (96, with 4096 visits) halves toward one
/// sample's foot, past the band's zero with headroom; `topo`'s spline
/// face placement's (48 quarterings, 8192 visits) quarters, each level
/// two halvings.
pub const CELL_DEPTH: usize = 64;
/// The cells one reading may visit: a refusal's cost on a grazing line,
/// which visits the cells along its touch. Measured on the spout's walls
/// at tilts of 1e-3 rad, a reading that answers visits up to ~12 000.
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
    /// The line's parameters the crossing's region spans along it: its
    /// along-line coefficients' least and greatest, decided past the band
    /// inside the span. Narrowed to the band's escalation where a part
    /// can be shown to hold the crossing; wider where none can.
    pub advance: (T, T),
    /// The chart rectangle `[u0, u1, v0, v1]` of the region that holds it,
    /// and no other crossing.
    pub rect: [f64; 4],
    /// The sign of `d·(S_u × S_v)` over the region: which way the line
    /// passes the chart's orientation.
    pub incidence: Sign,
}

/// **A stretch of the line the arm did not read** ([`read`]): the line's
/// parameters an unread region spans, and why.
#[derive(Clone, Copy, Debug)]
pub struct Unread<T> {
    /// The region's along-line coefficients' least and greatest.
    pub window: (T, T),
    /// Why.
    pub fault: MeetFault,
}

/// **The arm's refusal** ([`read`]).
#[derive(Clone, Debug)]
pub enum PatchRefusal<T> {
    /// The crossings found and the stretches not read. A found crossing's
    /// region overlaps no other's past what it is shown to hold, but may
    /// lie beside or across a stretch: the reader takes the stretches as
    /// windows, which the found meetings in them are taken into.
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
fn points<T: Real>(net: &[Control<T>]) -> impl Iterator<Item = Vec3<T>> + '_ {
    net.iter()
        .map(|(h, _)| Vec3::new(h[0] / h[3], h[1] / h[3], h[2] / h[3]))
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

/// The hull of two hulls.
fn joined<T: Real>(a: Option<(T, T)>, b: (T, T)) -> (T, T) {
    a.map_or(b, |(lo, hi)| (lo.min(b.0), hi.max(b.1)))
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

/// **A region the walk reads** (module docs): a rectangle `[u0, u1, v0,
/// v1]` of the chart and the cut's cells over it, each restricted to its
/// part.
#[derive(Clone)]
struct Region<T> {
    rect: [f64; 4],
    parts: Vec<Cell<T>>,
}

impl<T> Region<T> {
    /// A cell of the cut, or a cell halved from one.
    fn of(cell: Cell<T>) -> Self {
        Self {
            rect: cell.rect,
            parts: vec![cell],
        }
    }

    fn depth(&self) -> usize {
        self.parts.iter().map(|p| p.depth).max().unwrap_or(0)
    }
}

/// What [`Walk::verdict`] makes of one region.
enum Verdict {
    /// No crossing in it.
    Clear,
    /// One crossing, of this incidence.
    One(Sign),
    /// One-to-one, its boundary within the band of the line.
    Near,
    /// Not decided.
    Open,
    /// A weight not shown positive.
    Unhulled,
}

/// What a near cell grown about its crossing reads ([`Walk::grown`]).
enum Grown<T> {
    /// One crossing in the grown region, which claims it.
    One(Region<T>, Sign),
    /// None in the grown region, so none in the near cell.
    Clear,
    /// A weight not shown positive.
    Unhulled,
    /// Nothing read.
    Open,
}

/// The walk over the chart's regions ([`read`]).
struct Walk<T: Real> {
    frame: Frame<T>,
    span: (T, T),
    pu: usize,
    pv: usize,
    band: Band,
    /// The cut's cells, which a grown region is restricted from.
    cut: Vec<Cell<T>>,
    /// The chart's domain, `[u0, u1, v0, v1]`, which a region is grown
    /// inside.
    domain: [f64; 4],
}

impl<T: Decide> Walk<T> {
    /// The region over `rect`: every cell of the cut it meets with area,
    /// restricted to their overlap.
    fn region(&self, rect: [f64; 4]) -> Region<T> {
        let parts = self
            .cut
            .iter()
            .filter_map(|cell| {
                overlap(rect, cell.rect).map(|part| cell.restricted(self.pu, self.pv, part))
            })
            .collect();
        Region { rect, parts }
    }

    /// The region's control points, every part's.
    fn points<'a>(&self, region: &'a Region<T>) -> impl Iterator<Item = Vec3<T>> + 'a {
        region.parts.iter().flat_map(|p| points(&p.net))
    }

    /// **The region's verdict** (module docs), with the first decision in
    /// the gap of a residual.
    ///
    /// `reach` asks the span's ends as well: a region wholly behind the
    /// start or past the end is clear of the span, though the line may
    /// cross it. `known` is the incidence of a region already shown
    /// one-to-one that holds this one, which is one-to-one with it: its
    /// cone test is not read again.
    fn verdict(
        &self,
        region: &Region<T>,
        reach: bool,
        known: Option<Sign>,
        pending: &mut Option<Indeterminate>,
    ) -> Verdict {
        if !region
            .parts
            .iter()
            .flat_map(|p| &p.net)
            .all(|(h, _)| h[3].structure_positive())
        {
            return Verdict::Unhulled;
        }
        let Frame { d, n1, n2 } = self.frame;
        let band = self.band;
        for n in [n1, n2] {
            if past_band(
                PAST_THE_BAND,
                self.points(region).map(|x| x.dot(n)),
                band,
                pending,
            )
            .is_some()
            {
                return Verdict::Clear;
            }
        }
        if reach {
            let (t0, t1) = self.span;
            let (lo, hi) = (t0.min(t1), t0.max(t1));
            let behind = past_band(
                PAST_THE_END,
                self.points(region).map(|x| x.dot(d) - lo),
                band,
                pending,
            );
            let past = past_band(
                PAST_THE_END,
                self.points(region).map(|x| x.dot(d) - hi),
                band,
                pending,
            );
            if behind == Some(Sign::Negative) || past == Some(Sign::Positive) {
                return Verdict::Clear;
            }
        }
        let Some(incidence) = known.or_else(|| self.one_to_one(region)) else {
            return Verdict::Open;
        };
        match boundary_parity(self, region) {
            Some(true) => Verdict::One(incidence),
            Some(false) => Verdict::Clear,
            None => Verdict::Near,
        }
    }

    /// Whether every coefficient of one residual over the region is within
    /// the band (module docs, *Within the band*).
    fn within(&self, region: &Region<T>) -> bool {
        let Frame { n1, n2, .. } = self.frame;
        [n1, n2].into_iter().any(|n| {
            self.points(region).all(|x| {
                !matches!(
                    decide(PAST_THE_BAND, Margin::of(x.dot(n)), self.band),
                    Ok(Sign::Positive | Sign::Negative)
                )
            })
        })
    }

    /// **The cone test** (module docs): the sign of the corner
    /// determinants of the gradient boxes met over every part, where every
    /// one is proved that sign.
    fn one_to_one(&self, region: &Region<T>) -> Option<Sign> {
        let Frame { n1, n2, .. } = self.frame;
        let mut boxes: [Option<(T, T)>; 4] = [None; 4];
        for part in &region.parts {
            for (k, b) in self.gradient_boxes(part, [n1, n2])?.into_iter().enumerate() {
                boxes[k] = Some(joined(boxes[k], b));
            }
        }
        let [Some(a_u), Some(a_v), Some(b_u), Some(b_v)] = boxes else {
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

    /// The boxes of one cell's quotient-rule numerators, `w²·∂rᵢ` along
    /// `u` and `v`, for `n₁` then `n₂`: `[n₁ u, n₁ v, n₂ u, n₂ v]`. A
    /// part's derivatives are over its own unit parameters, so each box is
    /// scaled by the part's rectangle into the chart's parameters, the
    /// one scale every part's gradients are read at.
    fn gradient_boxes(&self, cell: &Cell<T>, normals: [Vec3<T>; 2]) -> Option<[(T, T); 4]> {
        let (pu, pv) = (self.pu, self.pv);
        // Weights scaled to at most one, so each numerator is at most the
        // gradient over the cell.
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
        let [u0, u1, v0, v1] = cell.rect;
        let scale = [
            T::from_computed(1.0 / (u1 - u0)),
            T::from_computed(1.0 / (v1 - v0)),
        ];
        let mut out = Vec::with_capacity(4);
        for n in normals {
            let r = form(&|h| n.x * h[0] + n.y * h[1] + n.z * h[2]);
            for (k, along_u) in [true, false].into_iter().enumerate() {
                let g = w
                    .times(&r.derivative(along_u))
                    .less(&r.times(&w.derivative(along_u)));
                let (lo, hi) = hull(g.c.into_iter())?;
                // A positive scale keeps each hull's order.
                out.push((lo * scale[k], hi * scale[k]));
            }
        }
        out.try_into().ok()
    }

    /// The along-line coefficients' least and greatest, over every part.
    fn advance(&self, region: &Region<T>) -> (T, T) {
        let d = self.frame.d;
        hull(self.points(region).map(|x| x.dot(d))).unwrap_or_else(|| {
            let nan = T::from_f64(f64::NAN);
            (nan, nan)
        })
    }

    /// A cell's extent: the diagonal of its control points' box, on their
    /// brackets' middles (a place, deciding nothing); NaN for the poison.
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

    /// **A near cell grown about its crossing** (module docs): its
    /// rectangle grown by half its size each way, inside the chart's
    /// domain and across the cut's knot lines, clear of every claim, and
    /// read again.
    fn grown(
        &self,
        cell: &Region<T>,
        claims: &[[f64; 4]],
        pending: &mut Option<Indeterminate>,
    ) -> Grown<T> {
        let [u0, u1, v0, v1] = cell.rect;
        let (du, dv) = ((u1 - u0) / 2.0, (v1 - v0) / 2.0);
        let [a0, a1, b0, b1] = self.domain;
        let rect = [
            (u0 - du).max(a0),
            (u1 + du).min(a1),
            (v0 - dv).max(b0),
            (v1 + dv).min(b1),
        ];
        if rect == cell.rect || claims.iter().any(|c| overlap(rect, *c).is_some()) {
            return Grown::Open;
        }
        let grown = self.region(rect);
        match self.verdict(&grown, true, None, pending) {
            Verdict::One(incidence) => Grown::One(grown, incidence),
            Verdict::Clear => Grown::Clear,
            Verdict::Unhulled => Grown::Unhulled,
            Verdict::Near | Verdict::Open => Grown::Open,
        }
    }

    /// Whether the region over `rect` is shown to hold no crossing:
    /// decided clear, or one-to-one with a boundary that does not wind
    /// round the line.
    fn holds_none(&self, rect: [f64; 4], pending: &mut Option<Indeterminate>) -> bool {
        matches!(
            self.verdict(&self.region(rect), false, None, pending),
            Verdict::Clear
        )
    }

    /// **A found crossing narrowed** (module docs): cut until its
    /// along-line coefficients' spread is not decided past the band,
    /// keeping the part that holds it, or until no part can be shown to;
    /// `Err` where two parts read as holding it, or none, which the
    /// region's one crossing contradicts.
    fn narrowed(
        &self,
        mut region: Region<T>,
        incidence: Sign,
        pending: &mut Option<Indeterminate>,
    ) -> Result<Region<T>, Region<T>> {
        loop {
            let (lo, hi) = self.advance(&region);
            if decide(NARROW, Margin::of(hi - lo), self.band) != Ok(Sign::Positive)
                || region.depth() >= CELL_DEPTH
            {
                return Ok(region);
            }
            // A part decided clear of the line (its span's ends not asked:
            // the crossing may lie past them), or shown to hold none,
            // leaves the crossing in the others.
            let held = |part: &Region<T>, pending: &mut Option<Indeterminate>| match self.verdict(
                part,
                false,
                Some(incidence),
                pending,
            ) {
                Verdict::One(_) => Some(true),
                Verdict::Clear => Some(false),
                _ => None,
            };
            let ways: Vec<Vec<Region<T>>> = match region.parts.as_slice() {
                // One cell: halved, the other way, then off the halving's
                // grid, a crossing within the band of a cut leaving both
                // halves undecided.
                [cell] => CUTS
                    .iter()
                    .map(|&(fraction, across)| {
                        let (a, b) = self.halves(cell, fraction, across);
                        vec![Region::of(a), Region::of(b)]
                    })
                    .collect(),
                // A grown region across knot lines: its rectangle cut the
                // same ways, each side the region over it, then its
                // cells' parts.
                parts => {
                    let longer = parts
                        .first()
                        .is_none_or(|p| p.longer_along_u(self.pu, self.pv));
                    let mut ways: Vec<Vec<Region<T>>> = CUTS
                        .iter()
                        .map(|&(fraction, across)| {
                            let (a, b) = cut(region.rect, fraction, longer != across);
                            vec![self.region(a), self.region(b)]
                        })
                        .collect();
                    ways.push(parts.iter().cloned().map(Region::of).collect());
                    ways
                }
            };
            let mut next = None;
            for parts in ways {
                let reads: Vec<Option<bool>> = parts.iter().map(|p| held(p, pending)).collect();
                let holding = reads.iter().filter(|r| **r == Some(true)).count();
                let clear = reads.iter().filter(|r| **r == Some(false)).count();
                if holding > 1 || clear == parts.len() {
                    return Err(region);
                }
                let pick = if holding == 1 {
                    reads.iter().position(|r| *r == Some(true))
                } else if clear + 1 == parts.len() {
                    reads.iter().position(Option::is_none)
                } else {
                    None
                };
                if let Some(k) = pick {
                    next = parts.into_iter().nth(k);
                    break;
                }
            }
            match next {
                Some(part) => region = part,
                None => return Ok(region),
            }
        }
    }
}

/// **The parity of the boundary's image about the origin** (module
/// docs): `Some` where every row of the region's boundary is read clear
/// of the ray or crossing it as its ends say, along one of the four axis
/// rays of the residuals' plane; `None` where none reads so. A part's
/// row lies on the boundary where its side is the region's.
fn boundary_parity<T: Decide>(walk: &Walk<T>, region: &Region<T>) -> Option<bool> {
    let band = walk.band;
    let (pu, pv) = (walk.pu, walk.pv);
    let Frame { n1, n2, .. } = walk.frame;
    let mut rows: Vec<Vec<[T; 3]>> = Vec::new();
    for cell in &region.parts {
        let at = |i: usize, j: usize| {
            let h = cell.net[i * (pv + 1) + j].0;
            [
                n1.x * h[0] + n1.y * h[1] + n1.z * h[2],
                n2.x * h[0] + n2.y * h[1] + n2.z * h[2],
                h[3],
            ]
        };
        // The sides, `[u0, u1, v0, v1]`: the rows at `i = 0`, `i = pu`,
        // `j = 0`, `j = pv`. Parity is a count, so no row's direction
        // matters.
        #[allow(clippy::float_cmp)] // a part's side is the region's own value where it lies on it
        let on = |k: usize| cell.rect[k] == region.rect[k];
        if on(0) {
            rows.push((0..=pv).map(|j| at(0, j)).collect());
        }
        if on(1) {
            rows.push((0..=pv).map(|j| at(pu, j)).collect());
        }
        if on(2) {
            rows.push((0..=pu).map(|i| at(i, 0)).collect());
        }
        if on(3) {
            rows.push((0..=pu).map(|i| at(i, pv)).collect());
        }
    }
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
/// (module docs), in no order: the table's placement orders them.
///
/// # Errors
///
/// [`PatchRefusal`]: the stretches not read, with the crossings found, or
/// the whole reading.
pub fn read<T: Decide>(
    line: &geom::Line<T>,
    span: (T, T),
    chart: &NurbsSurface<T>,
    band: Band,
) -> Result<Vec<PatchCrossing<T>>, PatchRefusal<T>> {
    read_measured(line, span, chart, band, &mut Measure::default())
}

/// **What one reading cost** ([`read_measured`]).
#[derive(Clone, Copy, Debug, Default)]
pub struct Measure {
    /// Cells visited.
    pub visits: usize,
    /// The deepest cell visited or narrowed to.
    pub depth: usize,
    /// Near cells grown.
    pub grown: usize,
    /// Overlaps with a claim read before a crossing was counted.
    pub overlaps: usize,
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
    let domain = cells.iter().fold(
        [
            f64::INFINITY,
            f64::NEG_INFINITY,
            f64::INFINITY,
            f64::NEG_INFINITY,
        ],
        |d, c| {
            [
                d[0].min(c.rect[0]),
                d[1].max(c.rect[1]),
                d[2].min(c.rect[2]),
                d[3].max(c.rect[3]),
            ]
        },
    );
    let walk = Walk {
        frame: Frame::of(line),
        span,
        pu,
        pv,
        band,
        cut: cells.clone(),
        domain,
    };
    let mut stack: Vec<Cell<T>> = cells.into_iter().rev().collect();
    let mut claims: Vec<[f64; 4]> = Vec::new();
    let (mut found, mut unread) = (Vec::new(), Vec::new());
    let mut visits = 0usize;
    while let Some(cell) = stack.pop() {
        visits += 1;
        measure.visits += 1;
        measure.depth = measure.depth.max(cell.depth);
        if visits > CELL_VISITS {
            return Err(PatchRefusal::Whole(MeetFault::Unsettled(
                Unsettled::Exhausted,
            )));
        }
        // A cell inside a claim holds no crossing but its claim's.
        if claims.iter().any(|c| inside(cell.rect, *c)) {
            continue;
        }
        let mut pending = None;
        let region = Region::of(cell);
        let (region, incidence) = match walk.verdict(&region, true, None, &mut pending) {
            Verdict::Clear => continue,
            Verdict::Unhulled => return Err(unhulled()),
            Verdict::One(incidence) => (region, incidence),
            Verdict::Near => {
                measure.grown += 1;
                match walk.grown(&region, &claims, &mut pending) {
                    Grown::One(grown, incidence) => (grown, incidence),
                    Grown::Clear => continue,
                    Grown::Unhulled => return Err(unhulled()),
                    Grown::Open => {
                        undecided(&walk, &mut stack, &mut unread, region, pending);
                        continue;
                    }
                }
            }
            Verdict::Open => {
                undecided(&walk, &mut stack, &mut unread, region, pending);
                continue;
            }
        };
        // The crossing is this region's alone, and no claim's: where the
        // two overlap, the overlap is shown to hold none.
        let overlaps: Vec<[f64; 4]> = claims
            .iter()
            .filter_map(|c| overlap(region.rect, *c))
            .collect();
        measure.overlaps += overlaps.len();
        if !overlaps
            .into_iter()
            .all(|o| walk.holds_none(o, &mut pending))
        {
            unread.push(stretch(&walk, &region, pending, Unsettled::Tangency));
            continue;
        }
        claims.push(region.rect);
        let narrow = match walk.narrowed(region, incidence, &mut pending) {
            Ok(narrow) => narrow,
            Err(region) => {
                unread.push(stretch(&walk, &region, pending, Unsettled::Tangency));
                continue;
            }
        };
        measure.depth = measure.depth.max(narrow.depth());
        let advance = walk.advance(&narrow);
        // Inside the span past the band, or wholly outside it; else in
        // band of an end, a stretch.
        let (t0, t1) = span;
        let (s0, s1) = (t0.min(t1), t0.max(t1));
        let reach = |x: T| decide(PAST_THE_END, Margin::of(x), band);
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

/// An undecided cell (module docs): within the band, a stretch; at the
/// floor (a cell no wider than half the band's zero, or [`CELL_DEPTH`]
/// halvings), a stretch; else halved onto the stack.
fn undecided<T: Decide>(
    walk: &Walk<T>,
    stack: &mut Vec<Cell<T>>,
    unread: &mut Vec<Unread<T>>,
    region: Region<T>,
    pending: Option<Indeterminate>,
) {
    if walk.within(&region) {
        unread.push(stretch(walk, &region, pending, Unsettled::Tangency));
        return;
    }
    let Some(cell) = region.parts.into_iter().next() else {
        return;
    };
    let floor = Walk::<T>::extent(&cell);
    // A NaN extent (the poison) is at the floor.
    let wide = matches!(
        floor.partial_cmp(&(0.5 * walk.band.zero())),
        Some(core::cmp::Ordering::Greater | core::cmp::Ordering::Equal)
    );
    if cell.depth >= CELL_DEPTH || !wide {
        unread.push(stretch(walk, &Region::of(cell), pending, Unsettled::Floor));
        return;
    }
    let (a, b) = walk.halves(&cell, Fraction::Half, false);
    stack.push(b);
    stack.push(a);
}

/// The rectangle cut at `fraction` along `u` (`along_u`) or `v`: the
/// cut's place, deciding nothing.
fn cut(rect: [f64; 4], fraction: Fraction, along_u: bool) -> ([f64; 4], [f64; 4]) {
    let f = match fraction {
        Fraction::Half => 0.5,
        Fraction::ThreeEighths => 0.375,
        Fraction::At(f) => f,
    };
    let k = if along_u { 0 } else { 2 };
    let at = rect[k] + f * (rect[k + 1] - rect[k]);
    let (mut a, mut b) = (rect, rect);
    a[k + 1] = at;
    b[k] = at;
    (a, b)
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

/// An unread region's stretch: its diagnostics where a decision fell in
/// the gap, else the limit `why`.
fn stretch<T: Decide>(
    walk: &Walk<T>,
    region: &Region<T>,
    pending: Option<Indeterminate>,
    why: Unsettled,
) -> Unread<T> {
    Unread {
        window: walk.advance(region),
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

    /// Degree two both ways, an interior knot at a half both ways: four
    /// cells, a ridged net.
    fn four_spans() -> NurbsSurface<f64> {
        let m = |k| NonZeroUsize::new(k).unwrap();
        let k = || Knots::new(vec![(0.0, m(3)), (0.5, m(1)), (1.0, m(3))], 2).unwrap();
        let mut control = Vec::new();
        for i in 0..4 {
            for j in 0..4 {
                let z = 0.3 * f64::from((7 * i + 3 * j) % 5) / 5.0;
                control.push(Point3::new(f64::from(i) / 3.0, f64::from(j) / 3.0, z));
            }
        }
        NurbsSurface::new(k(), k(), control, vec![1.0; 16]).unwrap()
    }

    /// The line through `S(u, v)` along its normal tilted by `tilt`, from
    /// half a metre behind it.
    fn through(s: &NurbsSurface<f64>, u: f64, v: f64, tilt: [f64; 3]) -> geom::Line<f64> {
        let jet = s.ders(u, v);
        let n = jet.du.cross(jet.dv).normalize();
        let d = (n + Vec3::new(tilt[0], tilt[1], tilt[2])).normalize();
        let o = jet.point - d * 0.5;
        line([o.x, o.y, o.z], [d.x, d.y, d.z])
    }

    #[test]
    fn a_crossing_on_a_knot_line_is_read_once() {
        // `u = 0.5` is a cut between cells: the crossing lies on both
        // cells' boundary, where neither reads it, and the near cell is
        // grown across the knot line into its neighbour.
        let s = four_spans();
        let l = through(&s, 0.5, 0.3, [0.3, -0.2, 0.0]);
        let mut m = Measure::default();
        match read_measured(&l, (0.0, 1.0), &s, band(), &mut m) {
            Ok(found) => {
                assert_eq!(found.len(), 1, "{found:?} after {m:?}");
                let [u0, u1, v0, v1] = found[0].rect;
                assert!(
                    u0 <= 0.5 && 0.5 <= u1 && v0 <= 0.3 && 0.3 <= v1,
                    "{:?}",
                    found[0].rect
                );
                let t = found[0].advance;
                assert!(
                    (0.5 - t.0).abs() < 1e-5 && (t.1 - 0.5).abs() < 1e-5,
                    "{t:?}"
                );
            }
            Err(e) => panic!("{e:?} after {m:?}"),
        }
    }

    #[test]
    fn a_crossing_on_a_knot_corner_is_read_once() {
        // `(0.5, 0.5)` is the corner of all four cells.
        let s = four_spans();
        for tilt in [[0.0; 3], [0.2, 0.1, 0.0]] {
            let l = through(&s, 0.5, 0.5, tilt);
            let mut m = Measure::default();
            match read_measured(&l, (0.0, 1.0), &s, band(), &mut m) {
                Ok(found) => assert_eq!(found.len(), 1, "{tilt:?}: {found:?} after {m:?}"),
                Err(e) => panic!("{tilt:?}: {e:?} after {m:?}"),
            }
        }
    }

    /// **An oracle over random patches**: rational degree-2 patches with
    /// an interior knot both ways, random heights and weights, each read
    /// by lines through a known chart point at `t = 3`, at tilts 0.8, 0.1,
    /// 1e-2 and 1e-3 off its tangent plane. Every reading that answers
    /// holds the known crossing; the counts printed are the refusals and
    /// the readings that reached the claimed-rectangle overlap rule.
    /// Measured 2026-11-25: 480 rays, 0 missed, 118 refused, 0 reached.
    /// Run: `cargo nextest run --release -p geom-brep
    /// random_rays_through_known_points --run-ignored only --no-capture`.
    #[test]
    #[ignore = "an oracle over 480 random rays; run command in the docs"]
    #[allow(clippy::cast_precision_loss)]
    fn random_rays_through_known_points_are_read_there() {
        let mut seed = 0x2545_f491_4f6c_dd1du64;
        let mut rnd = move || {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            (seed >> 11) as f64 / (1u64 << 53) as f64
        };
        let (mut refused, mut reached, mut missed) = (0usize, 0usize, Vec::new());
        for _ in 0..120 {
            let m = |k| NonZeroUsize::new(k).unwrap();
            let k = || Knots::new(vec![(0.0, m(3)), (0.5, m(1)), (1.0, m(3))], 2).unwrap();
            let mut control = Vec::new();
            let mut w = Vec::new();
            for i in 0..4 {
                for j in 0..4 {
                    control.push(Point3::new(
                        f64::from(i) / 3.0,
                        f64::from(j) / 3.0,
                        0.4 * rnd(),
                    ));
                    w.push(0.5 + rnd());
                }
            }
            let s = NurbsSurface::new(k(), k(), control, w).unwrap();
            for tilt in [0.8, 0.1, 1e-2, 1e-3] {
                let (u, v) = (rnd(), rnd());
                let jet = s.ders(u, v);
                let n = jet.du.cross(jet.dv).normalize();
                let d = (n * tilt + jet.du.normalize()).normalize();
                let o = jet.point - d * 3.0;
                let l = line([o.x, o.y, o.z], [d.x, d.y, d.z]);
                let mut me = Measure::default();
                match read_measured(&l, (0.0, 6.0), &s, band(), &mut me) {
                    Ok(found)
                        if !found
                            .iter()
                            .any(|c| c.advance.0 <= 3.0 + 1e-5 && 3.0 - 1e-5 <= c.advance.1) =>
                    {
                        missed.push((u, v, tilt, found));
                    }
                    Ok(_) => {}
                    Err(_) => refused += 1,
                }
                reached += usize::from(me.overlaps > 0);
            }
        }
        eprintln!("refused {refused}, reached the overlap rule {reached}");
        assert!(missed.is_empty(), "{missed:?}");
    }
}
