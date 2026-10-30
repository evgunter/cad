//! **The patch substrate** every patch lane evaluates on: the checked
//! net ([`PatchNet`]), its derivative grids ([`PatchGrid`]) with the
//! loose knot structures of the derivatives no clamped vector spells,
//! how a direction collapses ([`Collapse`]), span-local de Boor, and the
//! exact Newton–Cotes weights.

use geom_core::Bounds;
use geom_core::interval::certification::Certification;
use geom_core::interval::{Interval, certified_hi, cross};
use geom_core::spline::net::TensorNet;
use geom_core::spline::{
    CoeffWindow, Derived, KnotVector, LooseKnots, Param, ParamRange, SplineCoeffs, TensorChannels,
    constant_weight, diagonal_in_span,
};

use crate::props::PropsError;
use crate::props::bracket::pt;

/// Interval de Boor on one span: the span's polynomial evaluated at an
/// enclosed parameter from its `p + 1` active ring-bracketed
/// coefficients — the thin `f(m)` the composite rule needs on spline
/// channels. Sound for any `t`; outside the span's closure it is the
/// polynomial extension, which the composite rule reads where a vertex
/// pad puts a node past the domain.
///
/// A `t` inside the span's closure is the piece's blossom on the
/// diagonal; any other `t` is the extension ([`de_boor_on`]).
fn de_boor(win: CoeffWindow<'_, Interval>, t: Interval) -> Interval {
    let span = win.span();
    let kv = span.knots_f64_interim();
    de_boor_on(
        kv.knots(),
        kv.degree(),
        span.first_control(),
        win.coeffs(),
        t,
    )
}

/// The evaluation [`de_boor`] and the [`Loose::Raw`] evaluators share:
/// `active` the `p + 1` coefficients of the span whose first control is
/// `first`, over the knot list `u` they are a span of — a
/// [`CoeffWindow`]'s, or a raw direction's line windowed by
/// [`LooseKnots::span_of`]. Both callers form `active` from the span that `first`
/// names, so every index here is in range.
///
/// A `t` lying whole in the span's closure is the piece's blossom on
/// the diagonal ([`diagonal_in_span`], every level
/// [`Certification::between`]). Any other `t` — a vertex pad's node past
/// the domain, or one straddling a span end — is the polynomial
/// extension: de Boor's recurrence with the affine step
/// `(1 − α)·x + α·y`, sound for any `α`, since a ratio past `[0, 1]` is
/// no combination between its sources.
fn de_boor_on(u: &[f64], p: usize, first: usize, active: &[Interval], t: Interval) -> Interval {
    if let Some(value) = diagonal_in_span(u, p, first, active, t) {
        return value;
    }
    let mut d = active.to_vec();
    for r in 1..=p {
        for j in (r..=p).rev() {
            let i = first + j;
            let denom = pt(u[i + p + 1 - r]) - pt(u[i]);
            let alpha = (t - pt(u[i])) / denom;
            d[j] = (pt(1.0) - alpha) * d[j - 1] + alpha * d[j];
        }
    }
    d[p]
}

/// The pair's value at an exact parameter: [`de_boor`] in the span
/// containing `t`, refused at a non-finite `t`.
pub(super) fn eval_at(pair: SplineCoeffs<'_, Interval>, t: f64) -> Interval {
    match pair.span_at(t).filter(|_| t.is_finite()) {
        Some(win) => de_boor(win, pt(t)),
        None => Interval::refused(),
    }
}

/// Hull of a scalar B-spline over `range`: the hull of the active
/// spans' coefficient hulls (conservative to span granularity), the
/// last span read by [`closed_range_last_span`].
pub(super) fn range_hull(pair: SplineCoeffs<'_, Interval>, range: ParamRange) -> Interval {
    let knots = pair.knots();
    let (s0, s1) = knots.span_range(range);
    let (_, end) = range.ends();
    let last = closed_range_last_span(s0.index(), s1.index(), knots.knots(), end.get());
    let mut acc = Interval::refused();
    let mut seeded = false;
    for index in s0.index()..=last {
        // Emptiness check and window construction are one step.
        let Some(win) = pair.span(index) else {
            continue;
        };
        let h = win.hull();
        acc = if seeded { Interval::hull(acc, h) } else { h };
        seeded = true;
    }
    acc
}

/// A ring-bracketed 3-vector (a control point, an enclosure).
pub type RVec3 = [Interval; 3];

/// Widens both ends by a nonnegative pad (the honesty-pad fold).
pub(super) fn widen(x: Interval, pad: f64) -> Interval {
    if !x.is_certified() {
        return x;
    }
    Interval::from_bounds(x.lo() - pad, x.hi() + pad)
}

/// How a tensor direction collapses: a thin evaluation, a thin
/// evaluation of THIS SPAN'S polynomial at an enclosed parameter (the
/// exact Newton–Cotes lane — the span is located by a strictly
/// interior point, and the span polynomial is evaluated even at the
/// span's closed endpoints, which is exactly what integrating that
/// polynomial over the span wants), or a range hull.
#[derive(Clone, Copy)]
pub(super) enum Collapse<'a> {
    At(f64),
    AtSpan {
        /// A point strictly inside the target span (locates it in
        /// every derivative knot vector at once — they share
        /// breakpoints).
        mid: f64,
        /// The enclosed evaluation parameter.
        t: &'a Interval,
    },
    Over(ParamRange),
}

/// One direction's knot structure: a real knot vector, or the
/// [`Loose`] structure of a derivative no clamped vector spells.
#[derive(Clone)]
pub(super) enum Dir {
    Kv(KnotVector),
    Loose(Loose),
}

/// A derivative direction whose knot vector is **not representable** as
/// a [`KnotVector`] — shared by the patch lanes' [`Dir`] and the 1-D
/// [`Level`](super::green::Level). Its knots are a [`LooseKnots`], which locates spans as a
/// clamped vector does.
#[derive(Clone)]
pub(super) enum Loose {
    /// A derivative of degree ≥ 1: an interior knot
    /// of multiplicity equal to the parent's degree (the standard
    /// rational-arc structure) makes the derivative genuinely
    /// DISCONTINUOUS there, and the clamped invariant refuses interior
    /// multiplicity above the degree. Evaluated span-locally
    /// on the loose knots; the discontinuity is honest, and no
    /// patch cell spans it — the composite cells are cut on the
    /// interior knots ([`knot_aligned_cuts`](super::cells::knot_aligned_cuts)).
    Raw(LooseKnots),
    /// The degree-0 remnant a derivative ladder bottoms out at:
    /// coefficients are per-span constants over `knots[i]..knots[i+1]`
    /// — the parent's interior knot list, kept so span-LOCAL collapse
    /// stays exact; the 1-D ladder's whole-domain hull would lose the
    /// per-span constancy the Newton–Cotes lane depends on.
    Const(LooseKnots),
}

impl Loose {
    /// A [`Derived`]'s clamped payload, or the structure no clamped
    /// vector spells.
    pub(super) fn from_derived<V>(derived: Derived<V>) -> Result<V, Self> {
        match derived {
            Derived::Vector(v) => Ok(v),
            Derived::SpanConstant(knots) => Err(Self::Const(knots)),
            Derived::Discontinuous(knots) => Err(Self::Raw(knots)),
        }
    }

    /// [`Loose::Raw`] at degree ≥ 1, [`Loose::Const`] at 0.
    fn of(knots: LooseKnots) -> Self {
        if knots.degree() >= 1 {
            Self::Raw(knots)
        } else {
            Self::Const(knots)
        }
    }

    /// The structure one derivative down and the line step onto it, or
    /// `None` at per-span constants (the ladder's outer-None).
    pub(super) fn step(&self) -> Option<(Self, DerivTake)> {
        match self {
            Self::Raw(knots) => {
                let below = knots.derivative()?;
                let knots = knots.clone();
                Some((
                    Self::of(below),
                    Box::new(move |c: &[Interval]| raw_deriv(&knots, c)),
                ))
            }
            Self::Const(_) => None,
        }
    }
}

/// `line` as the coefficients of `knots`: every line a [`Loose`]
/// direction reads is built at its [`LooseKnots::control_count`], so a
/// line of another length is a broken grid, not an input.
fn loose_line<'l>(knots: &LooseKnots, line: &'l [Interval]) -> &'l [Interval] {
    if line.len() != knots.control_count() {
        unreachable!(
            "a loose direction's line is its count long: {} coefficients against {}",
            line.len(),
            knots.control_count()
        );
    }
    line
}

/// In-span de Boor on loose knots (the [`Loose::Raw`] evaluator).
///
/// `coeffs` is the direction's line, built at [`Dir::count`] by
/// `collapse_1d`.
pub(super) fn raw_eval(knots: &LooseKnots, coeffs: &[Interval], t: f64) -> Interval {
    match finite_param(t) {
        Some(at) => raw_de_boor(knots, coeffs, at, pt(t)),
        None => Interval::refused(),
    }
}

/// In-span de Boor at an ENCLOSED parameter on loose knots (the
/// [`Loose::Raw`] counterpart of [`de_boor`]): the node lies in the
/// closure of `mid`'s span, and the span polynomial is what the rule
/// integrates.
fn raw_eval_in_span(knots: &LooseKnots, coeffs: &[Interval], mid: f64, t: &Interval) -> Interval {
    match Param::new(mid) {
        Some(mid) => raw_de_boor(knots, coeffs, mid, *t),
        None => Interval::refused(),
    }
}

/// [`de_boor_on`] in the span that `at` locates, which is in
/// `degree ..= count − 1` ([`LooseKnots::span_of`]), so the window is
/// in range.
fn raw_de_boor(knots: &LooseKnots, coeffs: &[Interval], at: Param, t: Interval) -> Interval {
    let coeffs = loose_line(knots, coeffs);
    let (span, degree) = (knots.span_of(at), knots.degree());
    let first = span - degree;
    de_boor_on(knots.knots(), degree, first, &coeffs[first..=span], t)
}

/// Hull of a [`Loose::Raw`] spline over `range`: the local control
/// blocks of every touched span (the same convexity fact
/// [`range_hull`] uses), over a line built as [`raw_eval`]'s is.
pub(super) fn raw_range_hull(
    knots: &LooseKnots,
    coeffs: &[Interval],
    range: ParamRange,
) -> Interval {
    let coeffs = loose_line(knots, coeffs);
    let degree = knots.degree();
    let mut acc = Interval::refused();
    let mut seeded = false;
    for span in loose_spans_over(knots, range) {
        for c in &coeffs[span - degree..=span] {
            acc = if seeded { Interval::hull(acc, *c) } else { *c };
            seeded = true;
        }
    }
    acc
}

/// **The last span a closed range reads**: the one holding its end,
/// unless the range ENDS on a knot, where it is the span before. The
/// locator assigns a knot to the span starting at it, but the cells the
/// lanes hull are closed cells `[x, k]` lying in ONE smooth piece, and
/// what a cell's rule reads at `k` is the left piece's one-sided value.
/// That holds for a derivative too, which may jump at a C⁰ knot: the
/// cell's own piece's coefficients hull its one-sided values there, and
/// the next span's describe a function the cell does not contain.
/// Reading them hulls a neighbour the cell only touches (on a sliver
/// span, a derivative 1e9 times the cell's own), and reads a cell
/// ending at a C⁰ knot differently from the same cell of the one-span
/// tile ending there. One rule for both knot holders: [`range_hull`]
/// and [`loose_spans_over`].
fn closed_range_last_span(first: usize, last: usize, knots: &[f64], end: f64) -> usize {
    if last > first && knots[last] == end {
        last - 1
    } else {
        last
    }
}

/// **The nonempty spans of loose knots a closed range reads**: from the
/// span holding its start to [`closed_range_last_span`].
fn loose_spans_over(knots: &LooseKnots, range: ParamRange) -> impl Iterator<Item = usize> + '_ {
    let (start, end) = range.ends();
    let k = knots.knots();
    let first = knots.span_of(start);
    let last = closed_range_last_span(first, knots.span_of(end), k, end.get());
    (first..=last).filter(move |&s| k[s] < k[s + 1])
}

/// Derivative coefficients on loose knots. `degree ≥ 1` and the
/// line holds the direction's `knots.len() − degree − 1 ≥ 2`
/// coefficients, by the construction of [`Loose::Raw`] and [`Level::Loose`](super::green::Level::Loose),
/// so every index below is in range.
fn raw_deriv(loose: &LooseKnots, coeffs: &[Interval]) -> Vec<Interval> {
    let coeffs = loose_line(loose, coeffs);
    let (knots, degree) = (loose.knots(), loose.degree());
    #[allow(clippy::cast_precision_loss)]
    let p = pt(degree as f64);
    (0..coeffs.len() - 1)
        .map(|i| {
            let (a, b) = (knots[i + degree + 1], knots[i + 1]);
            // `knots[i+1] == knots[i+degree+1]` marks a DEGENERATE
            // (empty) span — the derivative has no coefficient there
            // because the function has no value there.
            // `LooseKnots::span_of` never selects an empty span, so this slot is only ever
            // hulled; zero is the safe filler (enlarging a hull can
            // never make a containment claim false).
            if a == b {
                return Interval::zero();
            }
            p * (coeffs[i + 1] - coeffs[i]) / (pt(a) - pt(b))
        })
        .collect()
}

/// One direction's coefficient-differentiation step: the map from a
/// line of the grid to the same line of its derivative grid.
type DerivTake = Box<dyn Fn(&[Interval]) -> Vec<Interval>>;

impl Dir {
    /// The number of coefficients a line along this direction holds —
    /// every net of a [`PatchGrid`] is built at its directions' counts,
    /// and every line a collapse reads is built at this one.
    fn count(&self) -> usize {
        match self {
            Self::Kv(kv) => kv.control_count(),
            Self::Loose(Loose::Raw(knots) | Loose::Const(knots)) => knots.control_count(),
        }
    }
}

/// `t` as a [`Param`] when it is finite: the point arms refuse an
/// infinite point as well as a NaN one.
fn finite_param(t: f64) -> Option<Param> {
    Param::new(t).filter(|p| p.get().is_finite())
}

/// A patch lane's UV rectangle: finite and of positive extent on each
/// side, held as the two ranges the lanes read over. [`PatchRect::new`]
/// is the lanes' one rectangle refusal.
#[derive(Clone, Copy)]
pub(super) struct PatchRect {
    pub(super) u: ParamRange,
    pub(super) v: ParamRange,
}

impl PatchRect {
    /// `(u0, u1, v0, v1)` as a rectangle, or the refusal for one that is
    /// empty, inverted, NaN or not finite.
    pub(super) fn new((u0, u1, v0, v1): (f64, f64, f64, f64)) -> Result<Self, PropsError> {
        // A finite difference of a strictly ordered pair: both ends are
        // finite and the side has positive extent.
        let side = |a: f64, b: f64| ParamRange::new(a, b).filter(|_| (b - a).is_finite() && a < b);
        match (side(u0, u1), side(v0, v1)) {
            (Some(u), Some(v)) => Ok(Self { u, v }),
            _ => Err(PropsError::QuadratureUnsupported {
                what: "empty or non-finite UV rectangle",
            }),
        }
    }

    /// The corners as `(u0, u1, v0, v1)`.
    pub(super) fn corners(self) -> (f64, f64, f64, f64) {
        (self.u.start(), self.u.end(), self.v.start(), self.v.end())
    }
}

/// **A patch's bracketed control net over its two knot vectors**, with
/// its weights: one [`TensorChannels`] of four channels — the position
/// `(x, y, z)` and the weight as a point bracket — over the caller's
/// vectors, borrowed. [`PatchNet::new`] checks the counts once, so no
/// door reads a net beside vectors it is related to by count alone.
#[derive(Clone, Debug)]
pub struct PatchNet<'a> {
    net: TensorChannels<'a, 4>,
}

impl<'a> PatchNet<'a> {
    /// The net `control` (row-major, `v` fastest) with `weights` over
    /// `kv_u × kv_v`.
    ///
    /// # Errors
    ///
    /// [`PropsError::QuadratureUnsupported`] when `control` is not
    /// `kv_u`'s count by `kv_v`'s, `weights` not one per point, or a
    /// weight not finite.
    pub fn new(
        kv_u: &'a KnotVector,
        kv_v: &'a KnotVector,
        control: &[RVec3],
        weights: &[f64],
    ) -> Result<Self, PropsError> {
        let nv = kv_v.control_count();
        let extent = kv_u.control_count().checked_mul(nv);
        if extent != Some(control.len()) || weights.len() != control.len() {
            return Err(PropsError::QuadratureUnsupported {
                what: "a patch net that does not fill its knot vectors' extent, or whose \
                       weights are not one per control point",
            });
        }
        // A point bracket holds a finite weight exactly; a non-finite
        // one has no point to hold.
        if weights.iter().any(|w| !w.is_finite()) {
            return Err(PropsError::QuadratureUnsupported {
                what: "a patch net with a non-finite weight",
            });
        }
        Ok(Self {
            net: TensorChannels::from_fn(kv_u, kv_v, |i, j| {
                let k = i * nv + j;
                let c = control[k];
                [c[0], c[1], c[2], pt(weights[k])]
            }),
        })
    }
}

impl PatchNet<'_> {
    /// The `u` direction's knot vector.
    #[must_use]
    pub fn knots_u(&self) -> &KnotVector {
        self.net.knots_u()
    }

    /// The `v` direction's knot vector.
    #[must_use]
    pub fn knots_v(&self) -> &KnotVector {
        self.net.knots_v()
    }

    /// Control point `(i, j)` and its weight.
    pub(super) fn at(&self, i: usize, j: usize) -> (RVec3, f64) {
        let [x, y, z, w] = self.net.channels().map(|c| c.net().get(i, j));
        ([x, y, z], w.lo())
    }

    /// `f` of the weights, row-major, as the point brackets that hold
    /// them exactly: `lo()` is the weight.
    pub(super) fn with_weights<R>(&self, f: impl FnOnce(&[Interval]) -> R) -> R {
        let [.., w] = self.net.channels();
        f(w.net().as_flat())
    }

    /// Whether the weights are not one constant ([`constant_weight`],
    /// on exact `f64` structure, C6).
    pub(super) fn is_rational(&self) -> bool {
        self.with_weights(|ws| {
            let values: Vec<f64> = ws.iter().map(|w| w.lo()).collect();
            constant_weight(&values).is_none()
        })
    }

    /// The three channels of `f` over the net's vectors, `f` read at
    /// each control point and its weight.
    pub(super) fn map_points(&self, f: impl Fn(RVec3, f64) -> RVec3) -> TensorChannels<'_, 3> {
        TensorChannels::from_fn(self.knots_u(), self.knots_v(), |i, j| {
            let (c, w) = self.at(i, j);
            f(c, w)
        })
    }

    /// The position channels — the net itself, weights apart.
    pub(super) fn positions(&self) -> TensorChannels<'_, 3> {
        self.map_points(|c, _| c)
    }
}

/// One (possibly derivative-exhausted) tensor grid of a vector
/// channel triple, three [`TensorNet`]s over one shared extent — the
/// 2-D counterpart of
/// [`DerivLadder`](super::green::DerivLadder)'s levels: a [`Loose::Const`] direction holds
/// per-span constants, and a grid that differentiates to nothing at
/// all is `Option<PatchGrid> = None`, read as identically zero — sound
/// on a KNOT-FREE cell, where the piece really is one polynomial of
/// exhausted degree, and that is what [`knot_aligned_cuts`](super::cells::knot_aligned_cuts) makes
/// every cell of both patch composites.
///
/// Each net is `du.count() × dv.count()`: the base is built from a
/// [`TensorChannels`] at its two vectors' extents and each derivative
/// shrinks the direction it differences by one, as its `Dir` does. It
/// is not itself a [`TensorChannels`]: a derivative direction may be
/// [`Loose::Raw`] or [`Loose::Const`], which no clamped vector can spell,
/// and the grid carries those beside the nets as the tensor pair
/// carries its vectors.
pub(super) struct PatchGrid {
    du: Dir,
    dv: Dir,
    ch: [TensorNet; 3],
}
impl PatchGrid {
    /// The base grid: the three channels of one bracketed control net
    /// over its one pair of vectors.
    pub(super) fn base(net: &TensorChannels<'_, 3>) -> Self {
        Self {
            du: Dir::Kv(net.knots_u().clone()),
            dv: Dir::Kv(net.knots_v().clone()),
            ch: net.channels().map(|c| c.net().clone()),
        }
    }

    /// This direction's derivative structure and its line step, or
    /// `None` at a per-span constant (the ladder's outer-None).
    fn step(dir: &Dir) -> Option<(Dir, DerivTake)> {
        match dir {
            Dir::Kv(kv) => {
                let below = Loose::from_derived(kv.derived()).map_or_else(Dir::Loose, Dir::Kv);
                let kv = kv.clone();
                // The one knot-vector step (`TensorCoeffs`'s too).
                Some((
                    below,
                    Box::new(move |c: &[Interval]| kv.difference_coeffs(c)),
                ))
            }
            Dir::Loose(loose) => {
                let (below, take) = loose.step()?;
                Some((Dir::Loose(below), take))
            }
        }
    }

    /// The u-partial-derivative grid (`None` = identically zero away
    /// from knots — the ladder's outer-None).
    ///
    /// A step that does not answer `count() - 1` coefficients refuses
    /// its line (`TensorNet::diff_u`). Unreachable from here and kept
    /// as a guard: both steps answer `n - 1`, and `raw_deriv` fills a
    /// DEGENERATE (empty) span with an explicit zero itself rather
    /// than by returning fewer coefficients.
    pub(super) fn deriv_u(&self) -> Option<Self> {
        let (du, take) = Self::step(&self.du)?;
        Some(Self {
            du,
            dv: self.dv.clone(),
            ch: core::array::from_fn(|k| self.ch[k].diff_u(&take)),
        })
    }

    /// The v-partial-derivative grid, per [`PatchGrid::deriv_u`].
    pub(super) fn deriv_v(&self) -> Option<Self> {
        let (dv, take) = Self::step(&self.dv)?;
        Some(Self {
            du: self.du.clone(),
            dv,
            ch: core::array::from_fn(|k| self.ch[k].diff_v(&take)),
        })
    }

    /// One direction's collapse of a line along it. Every line this
    /// grid hands it is [`Dir::count`] long — a row or column of a net
    /// built at its directions' counts, or the collapse of one — so a
    /// knot-vector direction's mint of it cannot refuse. The line is
    /// minted where it lies rather than rebuilt at the vector's count:
    /// these collapses run per line per cell, and the copy cost the
    /// rational patch lane 27% of its time.
    fn collapse_1d(dir: &Dir, line: &[Interval], op: Collapse<'_>) -> Interval {
        match dir {
            Dir::Kv(kv) => {
                let Some(pair) = kv.with_coeffs(line) else {
                    unreachable!("a grid line is its direction's count long");
                };
                match op {
                    Collapse::At(t) => eval_at(pair, t),
                    Collapse::AtSpan { mid, t } => match pair.span_at(mid) {
                        Some(win) => de_boor(win, *t),
                        None => Interval::refused(),
                    },
                    Collapse::Over(range) => range_hull(pair, range),
                }
            }
            Dir::Loose(Loose::Raw(knots)) => match op {
                Collapse::At(t) => raw_eval(knots, line, t),
                Collapse::AtSpan { mid, t } => raw_eval_in_span(knots, line, mid, t),
                Collapse::Over(range) => raw_range_hull(knots, line, range),
            },
            // Degree 0: the span index is the coefficient index.
            Dir::Loose(Loose::Const(knots)) => {
                let line = loose_line(knots, line);
                match op {
                    // A non-finite point refuses, as the `Kv` and `Raw`
                    // point arms beside it do.
                    Collapse::At(t) => match finite_param(t) {
                        Some(t) => line[knots.span_of(t)],
                        None => Interval::refused(),
                    },
                    Collapse::AtSpan { mid, .. } => match Param::new(mid) {
                        Some(mid) => line[knots.span_of(mid)],
                        None => Interval::refused(),
                    },
                    Collapse::Over(range) => loose_spans_over(knots, range)
                        .map(|s| line[s])
                        .reduce(Interval::hull)
                        .unwrap_or_else(Interval::refused),
                }
            }
        }
    }

    /// Collapses one channel: v first (per u-row), then u.
    pub(super) fn channel(&self, k: usize, u: Collapse<'_>, v: Collapse<'_>) -> Interval {
        let rows: Vec<Interval> = (0..self.du.count())
            .map(|i| Self::collapse_1d(&self.dv, self.ch[k].row(i), v))
            .collect();
        Self::collapse_1d(&self.du, &rows, u)
    }

    /// **The u-first collapse**: this grid's three channels reduced in
    /// the u direction only, leaving one v-coefficient vector each.
    ///
    /// Mathematically identical to [`PatchGrid::channel`]'s v-then-u
    /// order (a tensor collapse commutes); the association differs, so
    /// the outward rounding does — which is why only the rational lane,
    /// whose numbers are its own, uses it. Its point is that every
    /// cell in one column of the composite grid shares the SAME u
    /// collapse, so hoisting it out of the inner loop turns a
    /// per-cell `nu`-fold de Boor into a per-column one.
    pub(super) fn slice_u(&self, u: Collapse<'_>) -> [Vec<Interval>; 3] {
        core::array::from_fn(|k| {
            (0..self.dv.count())
                .map(|j| Self::collapse_1d(&self.du, &self.ch[k].column(j), u))
                .collect()
        })
    }

    /// The v collapse of a [`PatchGrid::slice_u`] result.
    pub(super) fn at_v(&self, sl: &[Vec<Interval>; 3], v: Collapse<'_>) -> RVec3 {
        core::array::from_fn(|k| Self::collapse_1d(&self.dv, &sl[k], v))
    }

    /// The vector value/hull of the whole triple.
    pub(super) fn vec(&self, u: Collapse<'_>, v: Collapse<'_>) -> RVec3 {
        [
            self.channel(0, u, v),
            self.channel(1, u, v),
            self.channel(2, u, v),
        ]
    }
}

/// The windows over a bracket box `bu × bv`, or `None` when either side
/// does not certify: a refused bracket names no region, so a hull over
/// it is refused rather than read off a window it does not bound.
pub(super) fn certified_box(
    bu: Interval,
    bv: Interval,
) -> Option<(Collapse<'static>, Collapse<'static>)> {
    Some((
        Collapse::Over(ParamRange::certified(bu)?),
        Collapse::Over(ParamRange::certified(bv)?),
    ))
}

/// The hull a refused window answers, per component.
pub(super) fn refused_vec() -> RVec3 {
    [
        Interval::refused(),
        Interval::refused(),
        Interval::refused(),
    ]
}

/// The zero-or-collapse read of an optional (derivative) grid.
/// A grid's three channels collapsed in u, `None` where the derivative
/// ladder has run out ([`grid_slice`]).
pub(super) type GridSlice = Option<[Vec<Interval>; 3]>;

/// **A grid's u-collapsed slice** ([`PatchGrid::slice_u`]), `None`
/// where the derivative ladder has run out. A loop reading many cells
/// of one column collapses u once here and each cell in v alone
/// ([`grid_slice_at`]), so its cost is linear in the cells rather than
/// in cells times rows.
pub(super) fn grid_slice(g: Option<&PatchGrid>, u: Collapse<'_>) -> GridSlice {
    g.map(|g| g.slice_u(u))
}

/// A [`grid_slice`] at one v collapse: [`grid_vec`]'s reading, the
/// identically-zero derivative where the ladder has run out.
pub(super) fn grid_slice_at(
    g: Option<&PatchGrid>,
    sl: Option<&[Vec<Interval>; 3]>,
    v: Collapse<'_>,
) -> RVec3 {
    match (g, sl) {
        (Some(g), Some(sl)) => g.at_v(sl, v),
        _ => [Interval::zero(), Interval::zero(), Interval::zero()],
    }
}

pub(super) fn grid_vec(g: Option<&PatchGrid>, u: Collapse<'_>, v: Collapse<'_>) -> RVec3 {
    match g {
        Some(g) => g.vec(u, v),
        None => [Interval::zero(), Interval::zero(), Interval::zero()],
    }
}

/// Closed Newton–Cotes weights on `[0, 1]` with `m + 1` equally
/// spaced nodes `j/m`, as EXACT `i128` fractions reduced and then
/// bracketed outward — no Gaussian trust anywhere: the rule's
/// algebraic exactness for polynomials of degree ≤ m (m odd; m + 1
/// for even m) is a theorem, the nodes are rational, and the
/// enclosure carries only interval arithmetic divisions' rounding. `None` when
/// `m` is outside the supported window (the fraction arithmetic's
/// `i128` headroom, m ≤ 12 — callers fall back to the composite
/// rule).
pub(super) fn newton_cotes_weights(m: usize) -> Option<Vec<Interval>> {
    if m == 0 || m > 12 {
        return None;
    }
    fn gcd(a: i128, b: i128) -> i128 {
        let (mut a, mut b) = (a.abs(), b.abs());
        while b != 0 {
            let t = a % b;
            a = b;
            b = t;
        }
        a.max(1)
    }
    let mi = m as i128;
    let mut out = Vec::with_capacity(m + 1);
    for j in 0..=m {
        // L_j(x) = Π_{k≠j} (m·x − k) / (j − k): numerator coefficients
        // by convolution, denominator the exact product.
        let mut num: Vec<i128> = vec![1];
        let mut den: i128 = 1;
        for k in 0..=m {
            if k == j {
                continue;
            }
            let ki = k as i128;
            let ji = j as i128;
            den = den.checked_mul(ji - ki)?;
            let mut next = vec![0i128; num.len() + 1];
            for (i, c) in num.iter().enumerate() {
                next[i + 1] = next[i + 1].checked_add(c.checked_mul(mi)?)?;
                next[i] = next[i].checked_add(c.checked_mul(-ki)?)?;
            }
            num = next;
        }
        // ∫₀¹ Σ cᵢ xⁱ dx = Σ cᵢ/(i+1), accumulated as one fraction.
        let (mut acc_n, mut acc_d): (i128, i128) = (0, 1);
        for (i, c) in num.iter().enumerate() {
            let d = (i as i128) + 1;
            acc_n = acc_n.checked_mul(d)?.checked_add(c.checked_mul(acc_d)?)?;
            acc_d = acc_d.checked_mul(d)?;
            let g = gcd(acc_n, acc_d);
            acc_n /= g;
            acc_d /= g;
        }
        // w_j = acc / den, sign-normalised, bracketed via outward ring
        // division of exactly-representable integers.
        let mut w_n = acc_n;
        let mut w_d = acc_d.checked_mul(den)?;
        if w_d < 0 {
            w_n = -w_n;
            w_d = -w_d;
        }
        let g = gcd(w_n, w_d);
        w_n /= g;
        w_d /= g;
        let exact53 = |x: i128| x.abs() < (1i128 << 53);
        if !exact53(w_n) || !exact53(w_d) {
            return None;
        }
        #[allow(clippy::cast_precision_loss)]
        out.push(pt(w_n as f64) / pt(w_d as f64));
    }
    Some(out)
}

/// The knots strictly inside `(lo, hi)`, in order. A range filter, not
/// `interior_knots`: `(lo, hi)` is a trim rectangle's side or a chord's
/// span, which is the vector's domain only sometimes.
pub(super) fn knots_inside(knots: &[f64], lo: f64, hi: f64) -> impl Iterator<Item = f64> + '_ {
    knots.iter().copied().filter(move |k| *k > lo && *k < hi)
}

/// The knot-span intervals of a vector clipped to `[lo, hi]`, each
/// with a strictly interior locator point.
pub(super) fn clipped_spans(kv: &KnotVector, lo: f64, hi: f64) -> Vec<(f64, f64, f64)> {
    range_spans(kv.knots(), lo, hi)
        .into_iter()
        .map(|(a, b)| (a, b, a.midpoint(b)))
        .collect()
}

/// **The knot spans a range meets**, clipped to it: the nonempty
/// intervals between `lo`, each distinct knot of the sorted slice
/// `knots` strictly inside `(lo, hi)`, and `hi`, in order. `hi` is the
/// last end as ITSELF, so the spans tile `[lo, hi]` exactly. The one
/// home of the span walk the cut lists, the exact rules and the
/// quotient statement's cells all read.
pub(super) fn range_spans(knots: &[f64], lo: f64, hi: f64) -> Vec<(f64, f64)> {
    let mut ends: Vec<f64> = vec![lo];
    for k in knots_inside(knots, lo, hi) {
        if ends.last().is_none_or(|e| *e != k) {
            ends.push(k);
        }
    }
    ends.push(hi);
    ends.windows(2)
        .filter(|w| w[1] > w[0])
        .map(|w| (w[0], w[1]))
        .collect()
}

/// `∂_u(S_u×S_v) = S_uu×S_v + S_u×S_uv` and `∂_v(S_u×S_v) =
/// S_uv×S_v + S_u×S_vv`, from the partials' readings.
pub(super) fn normal_partials(
    h_su: &RVec3,
    h_sv: &RVec3,
    h_suu: &RVec3,
    h_suv: &RVec3,
    h_svv: &RVec3,
) -> (RVec3, RVec3) {
    (
        rv_add(cross(h_suu, h_sv), cross(h_su, h_suv)),
        rv_add(cross(h_suv, h_sv), cross(h_su, h_svv)),
    )
}

/// An upper bound on `|v|` (2-norm) of a bracketed 3-vector.
pub(super) fn norm_hi(v: RVec3) -> f64 {
    certified_hi((v[0].sqr() + v[1].sqr() + v[2].sqr()).sqrt())
}

/// Componentwise sum of two bracketed 3-vectors.
pub(super) fn rv_add(a: RVec3, b: RVec3) -> RVec3 {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use geom_core::Bounds;

    /// **A node straddling its span's end, whose true value lies past
    /// it, is enclosed by the extension.** On `t − 1` over `[0, 1]` (the
    /// degree-1 piece with coefficients `−1, 0`), the node
    /// `[1⁻, 1⁺]` may be `1 + 2⁻⁵²`, where the piece reads `2⁻⁵²`. A node
    /// not whole in the span's closure takes the affine extension, which
    /// encloses it; the blossom would meet it with the sources' hull
    /// `[−1, 0]` and lose it. Red when the closure test is loosened to
    /// an overlap.
    #[test]
    fn a_node_straddling_its_span_end_is_enclosed_by_the_extension() {
        let past = 1.0f64.next_up();
        let t = Interval::from_bounds(1.0f64.next_down(), past);
        let got = de_boor_on(&[0.0, 0.0, 1.0, 1.0], 1, 0, &[pt(-1.0), pt(0.0)], t);
        assert!(
            got.contains(past - 1.0),
            "{got:?} misses the extension's value {:e} at {past}",
            past - 1.0
        );
        let inside = de_boor_on(&[0.0, 0.0, 1.0, 1.0], 1, 0, &[pt(-1.0), pt(0.0)], pt(0.5));
        assert!(inside.contains(-0.5), "{inside:?}");
    }

    /// **A refused bracket mints no window, so no hull is read over
    /// one.** The designers' worked example: knots `[0,0,0,1,2,3,3,3]`,
    /// degree 2, coefficients `c_i = i`. A refused bracket's ends were
    /// NaN, and the locator landed NaN on the first span, so the hull
    /// came back as that span's, `[0, 2]`, certified, over a region
    /// nobody asked about. Now the producer has no window to build, and
    /// the reading it would have fed answers refused.
    #[test]
    fn a_refused_bracket_mints_no_window_and_its_reading_refuses() {
        let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 3.0, 3.0], 2).unwrap();
        let coeffs: Vec<Interval> = (0..5).map(|i| pt(f64::from(i))).collect();
        let first = ParamRange::certified(Interval::from_bounds(0.25, 0.5)).unwrap();
        let pair = kv.with_coeffs(&coeffs).unwrap();
        let h = range_hull(pair, first);
        assert_eq!(
            (h.lo(), h.hi()),
            (0.0, 2.0),
            "CONTROL: the first span's hull"
        );
        let whole = ParamRange::certified(Interval::from_bounds(0.0, 3.0)).unwrap();
        let h = range_hull(pair, whole);
        assert_eq!(
            (h.lo(), h.hi()),
            (0.0, 4.0),
            "CONTROL: the whole domain's hull"
        );
        let good = Interval::from_bounds(0.0, 3.0);
        for (name, bad) in [
            ("refused", Interval::refused()),
            ("Trv", Interval::from_bounds(-1.0, 4.0).sqrt()),
        ] {
            assert!(
                ParamRange::certified(bad).is_none(),
                "{name}: minted a window"
            );
            assert!(certified_box(bad, good).is_none(), "{name} u: minted a box");
            assert!(certified_box(good, bad).is_none(), "{name} v: minted a box");
        }
    }

    /// **A NaN point locates no span on any `Dir`**: the `Const` point
    /// arm used to answer `coeffs[0]` for a NaN `t` and now refuses a
    /// non-finite one, as the `Kv` and `Raw` arms do; the span-located
    /// arms refuse a NaN `mid`.
    #[test]
    fn a_nan_point_collapses_to_a_refusal_on_every_dir() {
        let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 3.0, 3.0], 2).unwrap();
        let coeffs: Vec<Interval> = (0..5).map(|i| pt(f64::from(i))).collect();
        let raw = LooseKnots::new(kv.knots().to_vec(), 2).unwrap();
        let constant = LooseKnots::new(vec![0.0, 1.0, 2.0, 3.0], 0).unwrap();
        let dirs = [
            ("Kv", Dir::Kv(kv)),
            ("Raw", Dir::Loose(Loose::Raw(raw))),
            ("Const", Dir::Loose(Loose::Const(constant))),
        ];
        let t = pt(0.5);
        for (name, dir) in &dirs {
            let line: Vec<Interval> = (0..dir.count())
                .map(|i| coeffs.get(i).copied().unwrap_or_else(Interval::refused))
                .collect();
            let at = |op| PatchGrid::collapse_1d(dir, &line, op);
            assert!(
                at(Collapse::At(0.5)).is_certified(),
                "CONTROL {name}: a point certifies"
            );
            for bad in [f64::NAN, f64::INFINITY] {
                assert!(
                    !at(Collapse::At(bad)).is_certified(),
                    "{name}: At({bad}) certified"
                );
            }
            assert!(
                !at(Collapse::AtSpan {
                    mid: f64::NAN,
                    t: &t
                })
                .is_certified(),
                "{name}: AtSpan at a NaN mid certified"
            );
        }
    }
}
