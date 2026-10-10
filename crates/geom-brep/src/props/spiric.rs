//! **Certified quadrature for spiric-bounded faces** — the plane cap
//! and the torus wall a `Curve3::Spiric` bounds
//! (`docs/CURVED-SPIRIC-DESIGN.md` §4.7, Q5).
//!
//! With `P(t) = c + n·d + m·f(t) + a·r·sin t`, `m = a × n`,
//! `ρ = R + r·cos t`, `f = √(ρ² − d²)` (the carrier's own closed form,
//! `t` the torus's minor angle), both faces reduce to one-dimensional
//! integrals in `t` of algebraic functions of `(cos t, sin t)` — elliptic
//! integrals, with no elementary closed form. Everything else stays
//! closed.
//!
//! # The cap: the vector area
//!
//! A plane's flux is `anchor·A⃗` and its area `‖A⃗‖`, so the cap needs
//! only the spiric arc's vector-area term `(1/2)∫(P − ref) × P′ dt`.
//! Splitting `P − ref = (c + n·d − ref) + q`, `q = m·f + a·r·sin t`,
//! and using `m × a = n`:
//!
//! ```text
//! (1/2)·[(c + n·d − ref) × (P(t₁) − P(t₀)) + n·r·(2·J − [f·sin t]_{t₀}^{t₁})],
//! J = ∫ f(t)·cos t dt
//! ```
//!
//! (`q × q′ = n·r·(f cos t − f′ sin t)`, and the `f′` half integrates
//! by parts). `J` is the only quadrature.
//!
//! # The wall: the torus chart's Green form
//!
//! The torus face's flux about its centre and its chart area are
//! `−∮ F(v) du` and `−∮ G(v) du` over its lifted boundary, with
//! `F′ = r·ρ(v)·(R cos v + r)` and `G′ = r·ρ(v)` (the closed-form
//! lane's `torus_chart`, same antiderivatives). A rim at lifted level
//! `V` turning `Δu` gives `−F(V)·Δu` exactly; a meridian gives
//! nothing. A spiric's lifted image is exact
//! (`SpiricImage::Wall`: `u = u₀ + σ·atan2(f, d)`, `v = v₀ + σ·t`,
//! `v₀ = k·τ`), so `sin v = σ sin t`, `cos v = cos t` and
//! `u′ = −σ·d·r·sin t/(ρ·f)`, and its terms are
//!
//! ```text
//! −∫ F(v₀ + σt)·u′ dt,   −∫ G(v₀ + σt)·u′ dt,   ∫ u′ dt  (its Δu, for the closure check)
//! ```
//!
//! # The rule and its remainder
//!
//! Per piece `[p, q]` with its midpoint `x` as the node (Taylor with
//! the Lagrange remainder; `gₖ = g⁽ᵏ⁾/k!`):
//!
//! ```text
//! ∫_p^q g  ∈  Σ_{k=0..3} gₖ(x)·∫_p^q (t − x)ᵏ dt  +  g₄([p, q])·∫_p^q (t − x)⁴ dt
//! ```
//!
//! — the weight `(t − x)⁴` is nonnegative, so the remainder's mean
//! value lies in the hull of `g₄` over the piece. The coefficients at
//! the node and over the piece come from one Taylor jet ([`Jet`]) in
//! certification arithmetic, whose only non-ring operation is `√`
//! (admitted by C9); a piece's width is `g₄`'s width over it times
//! `h⁵/80`, so the composite width falls as `h⁵`. `cos` and `sin`
//! enter as enclosures by [`super::quad`]'s transcendental-free
//! rotation (`trig_at` from the exact `(1, 0)` at `t = 0`, widened
//! over the piece by the interval rotation). The bracket is sound at
//! every resolution, so the enclosure only tightens as the schedule
//! doubles the pieces. Every piece boundary is an exact `f64`, the
//! last one the stored end itself, so the pieces tile the carrier
//! interval exactly.
//!
//! # Honesty pads
//!
//! - **Endpoint brackets**: a non-thin `t₀`/`t₁` bracket adds `sup|g|`
//!   (over the whole edge) times its half-width, and its closed-form
//!   endpoint terms are evaluated over the bracket.
//! - **Map residual** (the wall only): each lifted image tracks its
//!   carrier within its certificate envelope `e`. The chart metric is
//!   `ds² = ρ²du² + r²dv²` with `ρ ≥ R − r`, so the chart region moves
//!   by at most `(chart length)·e/min(R − r, r)` of area, which costs
//!   at most `r(R + r)²` of flux and `r(R + r)` of area per unit.

use geom_core::interval::Interval;
use geom_core::interval::certification::Certification;
use geom_core::{Band, Bounds, Decide, Margin, Sign};

use super::quad::{
    AbsEnclosure, FaceCutBounds, QUAD_INIT_PIECES, QUAD_MAX_ROUNDS, QUAD_TARGET_LEN_FACTOR, RVec3,
    RoundOutcome, RoundWindow, classify_len, lo_or_refuse, mean_boundary_displacement, mid, rotate,
    round_exit, rv_cross, rv_dot, trig_at,
};
use super::{PropsCheck, PropsError};
use crate::offset_meters::mig;

fn pt(x: f64) -> Interval {
    Interval::point(x)
}

/// The Taylor coefficients `g⁽ᵏ⁾/k!`, `k = 0..=4`, of a function of `t`,
/// each an enclosure: at a node they are thin; over a piece (inputs
/// enclosing the piece) each encloses its coefficient over the whole
/// piece, which is what the remainder reads.
#[derive(Clone, Copy)]
struct Jet([Interval; ORDER]);

/// Coefficients carried: the rule is exact through degree 3 and its
/// remainder reads the degree-4 coefficient.
const ORDER: usize = 5;

impl Jet {
    fn cst(v: Interval) -> Self {
        let mut c = [Interval::zero(); ORDER];
        c[0] = v;
        Self(c)
    }

    fn add(self, o: Self) -> Self {
        Self(core::array::from_fn(|k| self.0[k] + o.0[k]))
    }

    fn sub(self, o: Self) -> Self {
        Self(core::array::from_fn(|k| self.0[k] - o.0[k]))
    }

    fn scale(self, x: Interval) -> Self {
        Self(self.0.map(|c| c * x))
    }

    /// The Cauchy product.
    fn mul(self, o: Self) -> Self {
        Self(core::array::from_fn(|k| {
            (0..=k).fold(Interval::zero(), |acc, i| acc + self.0[i] * o.0[k - i])
        }))
    }

    /// `1/x`: `b₀ = 1/a₀`, `bₖ = −(Σ_{i=1..k} aᵢ·bₖ₋ᵢ)/a₀`.
    fn recip(self) -> Self {
        let inv = Interval::one() / self.0[0];
        let mut b = [Interval::zero(); ORDER];
        b[0] = inv;
        for k in 1..ORDER {
            let sum = (1..=k).fold(Interval::zero(), |acc, i| acc + self.0[i] * b[k - i]);
            b[k] = -(sum * inv);
        }
        Self(b)
    }

    /// `√x`: `s₀ = √a₀`, `sₖ = (aₖ − Σ_{i=1..k−1} sᵢ·sₖ₋ᵢ)/(2·s₀)`.
    fn sqrt(self) -> Self {
        let mut r = [Interval::zero(); ORDER];
        r[0] = self.0[0].sqrt();
        let half_inv = Interval::one() / (pt(2.0) * r[0]);
        for k in 1..ORDER {
            let sum = (1..k).fold(Interval::zero(), |acc, i| acc + r[i] * r[k - i]);
            r[k] = (self.0[k] - sum) * half_inv;
        }
        Self(r)
    }
}

/// The jets of `t`, `cos t` and `sin t` from enclosures of their values.
fn trig_jets(t: Interval, c: Interval, s: Interval) -> (Jet, Jet, Jet) {
    let z = Interval::zero();
    (
        Jet([t, Interval::one(), z, z, z]),
        Jet([c, -s, -c / pt(2.0), s / pt(6.0), c / pt(24.0)]),
        Jet([s, c, -s / pt(2.0), -c / pt(6.0), s / pt(24.0)]),
    )
}

/// `(cos x, sin x)` over a parameter bracket: rotated from `(1, 0)` to
/// its midpoint, then widened over its half-width by the interval
/// rotation (exact for a thin bracket).
fn trig_of(x: Interval) -> (Interval, Interval) {
    let m = mid(x);
    if !x.is_certified() || !m.is_finite() {
        return (Interval::refused(), Interval::refused());
    }
    let at = trig_at((Interval::one(), Interval::zero()), m);
    let hw = (pt(x.hi()) - pt(m)).hi().max((pt(m) - pt(x.lo())).hi());
    rotate(at.0, at.1, spread_cos(hw), spread_sin(hw))
}

/// `cos δ` over `|δ| ≤ h`: `[1 − h²/2, 1]` (the whole circle past it).
fn spread_cos(h: f64) -> Interval {
    if h <= 1.5 {
        Interval::from_bounds((pt(1.0) - pt(h).sqr() / pt(2.0)).lo().max(-1.0), 1.0)
    } else {
        Interval::from_bounds(-1.0, 1.0)
    }
}

/// `sin δ` over `|δ| ≤ h`: `[−h, h]`, clipped to the unit interval.
fn spread_sin(h: f64) -> Interval {
    let b = h.min(1.0);
    Interval::from_bounds(-b, b)
}

/// `∫_a^b g` for `N` integrands at once, `pieces` equal pieces (the rule
/// in the module docs), plus the endpoint-bracket pad: `a`, `b` are the
/// brackets' midpoints, and `widths` their summed half-widths.
fn composite<const N: usize>(
    t0: Interval,
    t1: Interval,
    pieces: usize,
    g: impl Fn(Jet, Jet, Jet) -> [Jet; N],
) -> [Interval; N] {
    let (a, b) = (mid(t0), mid(t1));
    let span = b - a;
    if !(t0.is_certified() && t1.is_certified() && span.is_finite() && span >= 0.0) || pieces == 0 {
        return [Interval::refused(); N];
    }
    let mut total = [Interval::zero(); N];
    let one_zero = (Interval::one(), Interval::zero());
    #[allow(clippy::cast_precision_loss)]
    let node = |i: usize| {
        if i == pieces {
            b
        } else {
            a + span * (i as f64 / pieces as f64)
        }
    };
    for i in 0..pieces {
        let (p, q) = (node(i), node(i + 1));
        let x = p + (q - p) * 0.5;
        let (ri, le) = (pt(q) - pt(x), pt(p) - pt(x));
        // `∫_p^q (t − x)ᵏ dt`, `k = 0..=4`.
        let weight: [Interval; ORDER] = core::array::from_fn(|k| {
            #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
            let e = k as i32 + 1;
            (ri.powi(e) - le.powi(e)) / pt(f64::from(e))
        });
        let (cx, sx) = trig_at(one_zero, x);
        let (tj, cj, sj) = trig_jets(pt(x), cx, sx);
        let at = g(tj, cj, sj);
        let hw = ri.hi().max(-le.lo());
        let (cb, sb) = rotate(cx, sx, spread_cos(hw), spread_sin(hw));
        let (tb, cbj, sbj) = trig_jets(Interval::from_bounds(p, q), cb, sb);
        let over = g(tb, cbj, sbj);
        for k in 0..N {
            let exact =
                (0..ORDER - 1).fold(Interval::zero(), |acc, j| acc + at[k].0[j] * weight[j]);
            total[k] = total[k] + exact + over[k].0[ORDER - 1] * weight[ORDER - 1];
        }
    }
    let widths = (pt(t0.hi()) - pt(a)).hi().max((pt(a) - pt(t0.lo())).hi())
        + (pt(t1.hi()) - pt(b)).hi().max((pt(b) - pt(t1.lo())).hi());
    if widths > 0.0 {
        let full = Interval::from_bounds(-1.0, 1.0);
        let (tj, cj, sj) = trig_jets(Interval::hull(t0, t1), full, full);
        let sup = g(tj, cj, sj);
        for k in 0..N {
            let pad = (sup[k].0[0] * pt(widths)).mag();
            total[k] = total[k] + Interval::from_bounds(-pad, pad);
        }
    }
    total
}

/// `ρ = R + r·cos t` and `f = √(ρ² − d²)` as jets.
fn rho_f(major: Interval, minor: Interval, offset: Interval, c: Jet) -> (Jet, Jet) {
    let rho = Jet::cst(major).add(c.scale(minor));
    let f = rho.mul(rho).sub(Jet::cst(offset.sqr())).sqrt();
    (rho, f)
}

/// One spiric boundary edge of a cap or a wall, bracketed by the
/// caller (key-free, like [`super::LoopEdge`]).
#[derive(Clone, Copy, Debug)]
pub struct SpiricEdgeQ {
    /// The carrier's centre less the face's reference point.
    pub centre: RVec3,
    /// The carrier's unit axis `a`.
    pub axis: RVec3,
    /// The cutting plane's unit normal `n` (the carrier's `u_ref`).
    pub normal: RVec3,
    /// `R`.
    pub major: Interval,
    /// `r`.
    pub minor: Interval,
    /// `d`.
    pub offset: Interval,
    /// The carrier interval's start.
    pub t0: Interval,
    /// The carrier interval's end.
    pub t1: Interval,
    /// Loop traversal direction.
    pub forward: bool,
    /// On a torus wall, the edge's lifted chart image.
    pub wall: Option<WallImageQ>,
}

/// A spiric's lifted image on its torus wall
/// (`SpiricImage::Wall`): `u = u₀ + σ·atan2(f, d)`, `v = v₀ + σ·t`.
#[derive(Clone, Copy, Debug)]
pub struct WallImageQ {
    /// `σ = ±1`.
    pub sense: f64,
    /// The lifted minor-angle constant `v₀`.
    pub v0: Interval,
    /// The image's certificate envelope (metres).
    pub env: Interval,
}

/// A rim of a torus wall on its lift: the level `V` and its signed `Δu`
/// (traversal folded in).
#[derive(Clone, Copy, Debug)]
pub struct RimQ {
    /// The lifted level.
    pub v: Interval,
    /// The signed azimuth turn.
    pub du: Interval,
    /// The image's certificate envelope (metres).
    pub env: Interval,
}

/// A meridian of a torus wall on its lift: its signed `Δv`.
#[derive(Clone, Copy, Debug)]
pub struct MeridianQ {
    /// The signed minor-angle turn.
    pub dv: Interval,
    /// The image's certificate envelope (metres).
    pub env: Interval,
}

/// The torus chart a wall's Green form runs on.
#[derive(Clone, Debug)]
pub struct WallChartQ {
    /// The torus's `R`.
    pub major: Interval,
    /// The torus's `r`.
    pub minor: Interval,
    /// Its rims.
    pub rims: Vec<RimQ>,
    /// Its meridians.
    pub meridians: Vec<MeridianQ>,
}

/// One spiric-bounded face, bracketed by the caller.
///
/// The face's vector area about a reference point is `closed_va` (the
/// closed-form edges, already summed) plus each spiric's term; its flux
/// is `lever·A⃗` (a plane: `lever` is a point of the face less the
/// centre; a torus: the torus centre less the centre) plus, on a wall,
/// the chart Green form. The area is `‖A⃗‖` on a cap and the chart
/// area's magnitude on a wall.
#[derive(Clone, Debug)]
pub struct SpiricFaceQ {
    /// The closed-form edges' vector-area terms, about the reference.
    pub closed_va: RVec3,
    /// The spiric edges, centres relative to the reference.
    pub spirics: Vec<SpiricEdgeQ>,
    /// The flux lever.
    pub lever: RVec3,
    /// An upper bound on the boundary's length (metres) — the extent
    /// gate's lever.
    pub perimeter: f64,
    /// The torus chart, for a wall; `None` for a cap.
    pub wall: Option<WallChartQ>,
}

/// `J = ∫ f(t)·cos t dt` over the edge's carrier interval.
fn cos_moment(e: &SpiricEdgeQ, pieces: usize) -> Interval {
    let (big, r, d) = (e.major, e.minor, e.offset);
    let [j] = composite(e.t0, e.t1, pieces, |_, c, _| {
        let (_, f) = rho_f(big, r, d, c);
        [f.mul(c)]
    });
    j
}

/// One spiric edge's vector-area term about the reference (module
/// docs), at `pieces` resolution.
#[must_use]
pub fn spiric_vector_area(e: &SpiricEdgeQ, pieces: usize) -> RVec3 {
    let (big, r, d) = (e.major, e.minor, e.offset);
    let j = cos_moment(e, pieces);
    let end = |t: Interval| {
        let (c, s) = trig_of(t);
        let f = ((big + r * c).sqr() - d.sqr()).sqrt();
        (f, s)
    };
    let ((f0, s0), (f1, s1)) = (end(e.t0), end(e.t1));
    let m = rv_cross(e.axis, e.normal);
    let base = [0, 1, 2].map(|k| e.centre[k] + e.normal[k] * d);
    let chord = [0, 1, 2].map(|k| m[k] * (f1 - f0) + e.axis[k] * (r * (s1 - s0)));
    let sector = r * (pt(2.0) * j - (f1 * s1 - f0 * s0));
    let cross = rv_cross(base, chord);
    let half = pt(0.5);
    let va = [0, 1, 2].map(|k| (cross[k] + e.normal[k] * sector) * half);
    if e.forward { va } else { va.map(|x| -x) }
}

/// One spiric wall edge's Green-form terms `(−∫F du, −∫G du, Δu, Δv)`
/// on its lift (module docs), traversal folded in.
fn spiric_wall_terms(
    e: &SpiricEdgeQ,
    w: &WallImageQ,
    chart: &WallChartQ,
    pieces: usize,
) -> [Interval; 4] {
    let (big, r) = (chart.major, chart.minor);
    let d = e.offset;
    let sigma = pt(w.sense);
    let k = sigma * d * r;
    let rr = big.sqr() + r.sqr();
    let three_half = pt(1.5) * big * r;
    let half_br = pt(0.5) * big * r;
    let v0 = w.v0;
    let [flux, area, du] = composite(e.t0, e.t1, pieces, |t, c, s| {
        let (rho, f) = rho_f(big, r, d, c);
        let wgt = s.mul(rho.mul(f).recip()).scale(k);
        let v = Jet::cst(v0).add(t.scale(sigma));
        let ff = s
            .scale(sigma * rr)
            .add(v.scale(three_half))
            .add(s.mul(c).scale(sigma * half_br))
            .scale(r);
        let gg = v.scale(big).add(s.scale(sigma * r)).scale(r);
        [ff.mul(wgt), gg.mul(wgt), wgt.scale(-Interval::one())]
    });
    let dv = sigma * (pt(mid(e.t1)) - pt(mid(e.t0)));
    let terms = [flux, area, du, dv];
    if e.forward { terms } else { terms.map(|x| -x) }
}

/// `F(V)` and `G(V)` at a lifted level (the closed-form lane's
/// antiderivatives).
fn rim_fg(big: Interval, r: Interval, v: Interval) -> (Interval, Interval) {
    let (c, s) = trig_of(v);
    let f = r * ((big.sqr() + r.sqr()) * s + pt(1.5) * big * r * v + pt(0.5) * big * r * s * c);
    let g = r * (big * v + r * s);
    (f, g)
}

/// The wall's chart terms at `pieces`: `(flux, signed area, Σ Δu, Σ Δv)`
/// with the map-residual pad folded into the first two.
fn wall_chart(
    face: &SpiricFaceQ,
    chart: &WallChartQ,
    pieces: usize,
) -> Result<[Interval; 4], PropsError> {
    let (big, r) = (chart.major, chart.minor);
    let mut acc = [Interval::zero(); 4];
    // Σ (chart length · envelope), for the map-residual pad.
    let mut defect = Interval::zero();
    for e in &face.spirics {
        let Some(w) = e.wall.as_ref() else {
            return Err(PropsError::QuadratureUnsupported {
                what: "a spiric edge of a torus wall carries no lifted wall image",
            });
        };
        let t = spiric_wall_terms(e, w, chart, pieces);
        for k in 0..4 {
            acc[k] = acc[k] + t[k];
        }
        // |u′| ≤ d·r/((R − r)·f_min) and |v′| = 1 over the whole edge.
        let f_min = ((big - r).sqr() - e.offset.sqr()).sqrt();
        let du_max = pt(e.offset.mag()) * r / ((big - r) * f_min);
        let len = (du_max + Interval::one()) * (pt(mid(e.t1)) - pt(mid(e.t0)));
        defect = defect + len * w.env;
    }
    for rim in &chart.rims {
        let (f, g) = rim_fg(big, r, rim.v);
        acc[0] = acc[0] - f * rim.du;
        acc[1] = acc[1] - g * rim.du;
        acc[2] = acc[2] + rim.du;
        defect = defect + pt(rim.du.mag()) * rim.env;
    }
    for mer in &chart.meridians {
        acc[3] = acc[3] + mer.dv;
        defect = defect + pt(mer.dv.mag()) * mer.env;
    }
    // The chart's smallest metric scale, `min(R − r, r)`, from below — a
    // refused radius has ordinary endpoints, so it is asked by name.
    let floor = (big - r).lo().min(r.lo());
    if !(big.is_certified() && r.is_certified()) || floor.is_nan() || floor <= 0.0 {
        return Err(PropsError::QuadratureUnsupported {
            what: "a spiric wall on a torus whose metric floor min(R − r, r) is not \
                   certified positive, or whose radii are refused brackets",
        });
    }
    let uv = (defect / pt(floor)).mag();
    let outer = big + r;
    let pad_flux = (pt(uv) * r * outer.sqr()).mag();
    let pad_area = (pt(uv) * r * outer).mag();
    acc[0] = acc[0] + Interval::from_bounds(-pad_flux, pad_flux);
    acc[1] = acc[1] + Interval::from_bounds(-pad_area, pad_area);
    Ok(acc)
}

/// The face's enclosures at `pieces` resolution.
fn face_bounds(
    face: &SpiricFaceQ,
    pieces: usize,
) -> Result<(FaceCutBounds, [Interval; 2]), PropsError> {
    let mut va = face.closed_va;
    for e in &face.spirics {
        let t = spiric_vector_area(e, pieces);
        va = [0, 1, 2].map(|k| va[k] + t[k]);
    }
    let position = rv_dot(face.lever, va);
    Ok(match &face.wall {
        None => (
            FaceCutBounds {
                flux: position,
                area: (va[0].sqr() + va[1].sqr() + va[2].sqr()).sqrt(),
            },
            [Interval::zero(); 2],
        ),
        Some(chart) => {
            let [flux, area, du, dv] = wall_chart(face, chart, pieces)?;
            (
                FaceCutBounds {
                    flux: flux + position,
                    area: area.abs_enclosure(),
                },
                [du, dv],
            )
        }
    })
}

/// **The certified flux and area of a spiric-bounded face** over a
/// [`RoundWindow`] — [`super::quad`]'s schedule, meter and exits
/// (pieces doubling from `QUAD_INIT_PIECES`, convergence at
/// `QUAD_TARGET_LEN_FACTOR·ε` of mean boundary displacement, the
/// face-extent gate, the budget refusal).
///
/// On a wall, the lift must close (`props_spiric_lift_closed`): the
/// Green form bounds a region of the lifted chart only for a loop
/// whose `Σ Δu` and `Σ Δv` vanish, decided as lengths at `R` and `r`:
/// the distance of the returned enclosures from zero, so a round too
/// coarse to tell is not read as an open loop.
///
/// # Errors
///
/// [`PropsError`] — an escalated funnel decision, a degenerate face,
/// a wall loop whose lift does not close, or the typed
/// [`PropsError::QuadratureBudget`] refusal.
pub fn spiric_face_rounds<T: Decide>(
    face: &SpiricFaceQ,
    eps: f64,
    band: Band,
    window: RoundWindow,
) -> Result<RoundOutcome, PropsError> {
    let target_len = QUAD_TARGET_LEN_FACTOR * eps;
    let first = window.first.min(QUAD_MAX_ROUNDS);
    let mut pieces = QUAD_INIT_PIECES << first;
    for round in first..=QUAD_MAX_ROUNDS {
        let (bounds, [du, dv]) = face_bounds(face, pieces)?;
        let width_len = mean_boundary_displacement(bounds.flux, bounds.area)?;
        let converged = classify_len::<T>(
            "props_quad_converged",
            Margin::of(target_len - width_len),
            band,
            PropsCheck::Converged,
        )? == Sign::Positive;
        let out = if converged {
            match classify_len::<T>(
                "props_quad_face_extent",
                Margin::over_lever(lo_or_refuse(bounds.area), face.perimeter),
                band,
                PropsCheck::Extent,
            )? {
                Sign::Positive => {}
                Sign::Zero | Sign::Negative => return Err(PropsError::DegenerateFace),
            }
            Some(RoundOutcome::Converged(bounds))
        } else {
            round_exit(
                round,
                QUAD_MAX_ROUNDS,
                window,
                bounds,
                (width_len, target_len),
            )
        };
        if let Some(out) = out {
            if let Some(chart) = &face.wall {
                if !(du.is_certified() && dv.is_certified()) {
                    return Err(PropsError::QuadratureUnsupported {
                        what: "a spiric wall whose lift-closure enclosure is refused",
                    });
                }
                // The enclosure's distance from zero: a wide bracket about
                // zero is an open round, not an open loop.
                let gap = mig(du * chart.major).max(mig(dv * chart.minor));
                if classify_len::<T>(
                    "props_spiric_lift_closed",
                    Margin::of(gap),
                    band,
                    PropsCheck::Inventory,
                )? != Sign::Zero
                {
                    return Err(PropsError::NotIsoRectangle {
                        what: "props_spiric_lift_closed",
                    });
                }
            }
            return Ok(out);
        }
        pieces *= 2;
    }
    unreachable!("the round window is non-empty: `first` is clamped to the last round")
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;

    /// One reference case: `(R, r, d, t₀, t₁, σ, v₀)` and the four
    /// integrals' exact values as f64 round-down/round-up pairs
    /// (`J`, `Δu`, `−∫F du`, `−∫G du`), from a 50-digit adaptive
    /// quadrature of the DEFINITIONS (`u` differentiated numerically
    /// from `σ·atan2(f, d)`, not from this file's `u′`).
    type Case = ((f64, f64, f64, f64, f64, f64, f64), [(f64, f64); 4]);

    const PI: f64 = core::f64::consts::PI;

    fn cases() -> Vec<(&'static str, Case)> {
        vec![
            (
                "the vessel's arc, d < 0",
                (
                    (
                        0.09375,
                        0.0703125,
                        -0.0078125,
                        -0.8911225078866529,
                        0.8911225078866528,
                        -1.0,
                        0.0,
                    ),
                    [
                        (0.24255864761540002, 0.24255864761540005),
                        (2.4967740135765447e-18, 2.496774013576545e-18),
                        (-1.9156748891069987e-05, -1.9156748891069984e-05),
                        (-0.0001220663324153128, -0.00012206633241531278),
                    ],
                ),
            ),
            (
                "the vessel's arc, d > 0",
                (
                    (
                        0.09375,
                        0.0703125,
                        0.0078125,
                        -0.8911225078866529,
                        0.8911225078866528,
                        -1.0,
                        0.0,
                    ),
                    [
                        (0.24255864761540002, 0.24255864761540005),
                        (-2.496774013576545e-18, -2.4967740135765447e-18),
                        (1.9156748891069984e-05, 1.9156748891069987e-05),
                        (0.00012206633241531278, 0.0001220663324153128),
                    ],
                ),
            ),
            (
                "a span across π on the lift's second turn",
                (
                    (1.2, 0.225, 0.05, PI / 2.0, 3.0 * PI / 2.0, 1.0, 2.0 * PI),
                    [
                        (-2.0441206394570477, -2.0441206394570473),
                        (-1.915173823133119e-18, -1.9151738231331187e-18),
                        (0.0030300413224422515, 0.003030041322442252),
                        (-0.0042231023902593095, -0.004223102390259309),
                    ],
                ),
            ),
            (
                "a whole period, d < 0, σ = −1, the lift one turn down",
                (
                    (1.0, 0.3, -0.25, 0.0, 2.0 * PI, -1.0, -2.0 * PI),
                    [
                        (0.9759549009659082, 0.9759549009659083),
                        (-1.356466621510341e-33, -1.3564666215103408e-33),
                        (-0.022829979179362032, -0.02282997917936203),
                        (0.11236141214982714, 0.11236141214982716),
                    ],
                ),
            ),
            (
                "an asymmetric span, where Δu is not zero",
                (
                    (1.2, 0.225, 0.05, 0.3, 2.1, -1.0, 0.0),
                    [
                        (0.802442501578793, 0.8024425015787932),
                        (0.010695140649616026, 0.010695140649616028),
                        (0.004534513409502406, 0.004534513409502407),
                        (0.004469279612030439, 0.00446927961203044),
                    ],
                ),
            ),
            (
                "a near-tangent cut, f_min ≈ 0.01",
                (
                    (1.0, 0.5, 0.4999, -PI, PI, 1.0, 0.0),
                    [
                        (1.9990851916383723, 1.9990851916383725),
                        (0.0, 0.0),
                        (2.8963604554843574, 2.896360455484358),
                        (3.126972087751443, 3.1269720877514433),
                    ],
                ),
            ),
        ]
    }

    fn edge(c: (f64, f64, f64, f64, f64, f64, f64)) -> (SpiricEdgeQ, WallImageQ, WallChartQ) {
        let (big, r, d, t0, t1, sense, v0) = c;
        let wall = WallImageQ {
            sense,
            v0: pt(v0),
            env: Interval::zero(),
        };
        let e = SpiricEdgeQ {
            centre: [pt(0.25), pt(-0.5), pt(1.0)],
            axis: [pt(0.0), pt(0.0), pt(1.0)],
            normal: [pt(1.0), pt(0.0), pt(0.0)],
            major: pt(big),
            minor: pt(r),
            offset: pt(d),
            t0: pt(t0),
            t1: pt(t1),
            forward: true,
            wall: Some(wall),
        };
        let chart = WallChartQ {
            major: pt(big),
            minor: pt(r),
            rims: Vec::new(),
            meridians: Vec::new(),
        };
        (e, wall, chart)
    }

    /// **Containment against exact values**, at every resolution the
    /// schedule visits (the bracket is sound at each, not only once
    /// converged), and the enclosure is useful by 1024 pieces: narrower
    /// than `10⁻³` of the case's largest integral, which a rule
    /// returning a sound but `sup|g|·span`-wide bracket would not be.
    #[test]
    fn every_integral_encloses_its_exact_value_at_every_resolution() {
        for (name, (c, refs)) in cases() {
            let (e, w, chart) = edge(c);
            let scale = refs.iter().fold(0.0_f64, |m, &(lo, _)| m.max(lo.abs()));
            for pieces in [16, 128, 1024] {
                let [flux, area, du, _] = spiric_wall_terms(&e, &w, &chart, pieces);
                let got = [cos_moment(&e, pieces), du, flux, area];
                for (k, (g, (lo, hi))) in got.iter().zip(refs).enumerate() {
                    assert!(
                        g.is_certified() && g.lo() <= lo && hi <= g.hi(),
                        "{name}: integral {k} at {pieces} pieces is [{}, {}], \
                         the exact value is in [{lo}, {hi}]",
                        g.lo(),
                        g.hi()
                    );
                    if pieces == 1024 {
                        assert!(
                            g.hi() - g.lo() <= 1e-3 * scale,
                            "{name}: integral {k} at 1024 pieces is [{}, {}], wider than \
                             1e-3 of the case's scale {scale}",
                            g.lo(),
                            g.hi()
                        );
                    }
                }
            }
        }
    }

    /// A reversed edge negates every term, and `Δv` is `σ·(t₁ − t₀)`.
    #[test]
    fn reversal_negates_and_dv_is_the_span() {
        let (_, (c, _)) = cases().swap_remove(4);
        let (e, w, chart) = edge(c);
        let f = spiric_wall_terms(&e, &w, &chart, 64);
        let r = spiric_wall_terms(
            &SpiricEdgeQ {
                forward: false,
                ..e
            },
            &w,
            &chart,
            64,
        );
        for (a, b) in f.iter().zip(r) {
            assert_eq!((a.lo(), a.hi()), (-b.hi(), -b.lo()));
        }
        assert!(f[3].contains(-(2.1 - 0.3)), "dv = σ·span, got {:?}", f[3]);
    }

    /// Over a whole period the chord and endpoint terms vanish and the
    /// vector area is `n·r·J` — the oval's area along its plane normal.
    #[test]
    fn a_whole_oval_has_vector_area_n_r_j() {
        let (_, (c, refs)) = cases().swap_remove(5);
        let (e, ..) = edge(c);
        let va = spiric_vector_area(&e, 1024);
        let (lo, hi) = refs[0];
        let area = va[0];
        assert!(
            area.lo() <= 0.5 * lo && 0.5 * hi <= area.hi(),
            "r·J with r = 0.5, got {area:?}"
        );
        assert!(va[1].contains(0.0) && va[2].contains(0.0), "{va:?}");
    }
}
