//! **Certified control-hull bounds on a NURBS patch's partials**, per
//! knot-span cell — the surface-side companion of
//! [`geom_core::spline::hull`]'s scalar span hulls.
//!
//! # The convexity fact, one dimension up
//!
//! For a NON-RATIONAL tensor-product B-spline
//! `S(u,v) = Σᵢⱼ Nᵢ(u)·Nⱼ(v)·Pᵢⱼ`, every partial is itself a
//! tensor-product B-spline whose coefficient net comes from knot
//! differencing per direction (The NURBS Book Eq. 3.24 — exactly
//! [`geom_core::spline::net::TensorNet::diff_u_knots`], which is
//! [`geom_core::spline::hull::derivative_coeffs`] iterated across the
//! net's lines; that iteration is the ONE spelling, here and in every
//! other tensor consumer). Both
//! bases are nonnegative partitions of unity, so on a knot-span cell
//! every value of a partial is a convex combination of the derived
//! coefficients active there and lies in their **signed** hull.
//!
//! # One reading
//!
//! Every cell reports **signed componentwise enclosures**
//! ([`PatchCell::s_u`] … [`PatchCell::s_vv`]) — of the patch this
//! module actually assembled, which on the rational arm is the
//! REFINED net, not the described one ([`PatchCell`], "What the
//! enclosure encloses"). An inf-side consumer
//! needs them as such — a magnitude sup cannot bound `‖S_u × S_v‖`
//! from below, because the cross product's sign structure is exactly
//! the information a magnitude throws away — and a sup-side consumer
//! reads a vector magnitude off them with [`sq_norm`], whose
//! `√hi` is a sup bound on the norm.
//!
//! There used to be a second, MAGNITUDE reading alongside: the
//! rational arm applied the triangle inequality to the quotient rule
//! (all `+`, divide by the smallest weight) where the signed one
//! evaluates the quotient rule itself in the ring (the true `−` signs,
//! divide by the whole weight hull). Both were sound and the signed
//! one is strictly tighter, so the magnitude one is gone. What it cost
//! to keep is what it now saves: per rational cell, five ring
//! recurrences and five extra hull passes on the SHIPPED tessellation
//! sizing path. What its removal buys the consumer is a tighter grid —
//! the cancellation the triangle inequality could not see is real, and
//! on a quarter cylinder `sup‖S_uv‖` falls by an order of magnitude.
//!
//! # The rational arm
//!
//! A RATIONAL patch is `S = A/w` with `A = ΣΣ Nᵢ Nⱼ wᵢⱼ Pᵢⱼ` and
//! `w = ΣΣ Nᵢ Nⱼ wᵢⱼ` — both POLYNOMIAL tensor-product B-splines, so
//! every ingredient is the same control-hull fact taken on the
//! homogeneous nets. With `Ã = A − c·w` for a cell-local centre `c`
//! (so `S − c = Ã/w`, and knot differencing is linear:
//! `d(A − c·w) = dA − c·dw`), the quotient rule — exactly
//! `NurbsSurface::ders_in_span`'s corrections — reads
//!
//! ```text
//! S_u  = (Ã_u  − (S − c)·w_u) / w                        (v symmetric)
//! S_uu = (Ã_uu − 2·S_u·w_u − (S − c)·w_uu) / w           (v symmetric)
//! S_uv = (Ã_uv − S_u·w_v − S_v·w_u − (S − c)·w_uv) / w
//! ```
//!
//! **The divisor is the cell's weight hull, argued not assumed.** On
//! the cell `w` is a convex combination of the active weights, so
//! `w ∈ [w_min, w_max]`; the ring's division refuses a zero-touching
//! divisor, so a net whose positivity was never proven poisons rather
//! than answering.
//!
//! **Recentring keeps the cross terms cell-sized**: with the cell's
//! control centroid as `c`, `sup|S − c|` is a cell-of-control-net
//! fact rather than a whole-patch one, so `(S − c)·w_dd` does not
//! inflate with the patch's distance from the origin.
//!
//! A degree-1 direction's `Ã_dd` and `w_dd` are exactly zero, but its
//! CROSS terms survive — a rational degree-1 direction genuinely
//! curves in parameter — and the recurrences carry that.
//!
//! # Conservatism
//!
//! The answer is a bound, not an estimate. Ordinary walls measure
//! within a small factor of the true sup; extreme weight ratios can
//! leave it orders above, because the product terms lose the sign
//! correlation a steep ramp lives in — the residue of that loss, not
//! the whole of it: what the retired magnitude reading additionally
//! threw away was the quotient rule's OWN signs, and recovering those
//! is worth an order of magnitude on `sup‖S_uv‖` for an arc-walled
//! patch. The cost is only how finely a
//! consumer must subdivide; the bound is never wrong.
//!
//! # Poison (fail-loud, D4 ¶2)
//!
//! Structural refusals are typed ([`PatchBoundError`]); arithmetic
//! failures are ring poison, and a poisoned hull fails every `≤ ε`
//! comparison it reaches.

use std::ops::RangeInclusive;

use geom::surfaces::NurbsSurface;
use geom_core::ring_interval::RingInterval;
use geom_core::spline::KnotVector;
use geom_core::spline::net::TensorNet;

/// The fixed refinement schedule of the RATIONAL arm: every nonempty
/// span of every direction splits into this many equal pieces before
/// the per-cell assembly. A CONSTANT (D9: structure, never a
/// data-dependent iteration) — the `RATIONAL_METER_SPLITS = 16`
/// precedent of `geom::curves`' rational speed meter, mirrored. Knot
/// insertion is evaluation-invariant in ℝ, so it changes no geometry;
/// it only shrinks every hull the bound is assembled from, which is
/// what keeps the `sup‖S − c‖·sup|w_dd|` cross terms cell-sized.
pub const RATIONAL_CERT_SPLITS: usize = 16;

/// A typed refusal of the patch-bound assembly (fail-loud). Each
/// carries the prose its consumers print, so a lifted consumer's
/// message is this module's message.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PatchBoundError {
    /// A degree-0 direction — a degenerate patch description.
    DegreeZero,
    /// A degree-1 direction carrying interior knots: a C⁰ crease.
    Degree1Crease,
    /// A direction whose interior multiplicity equals its degree: a
    /// C⁰ crease.
    Crease,
    /// A rational description with a non-positive or non-finite
    /// weight — the convex-combination licence never held.
    NonPositiveWeight,
    /// The same, discovered after the fixed rational refinement.
    RefinedWeightLostPositivity,
    /// The fixed rational refinement failed to materialise.
    RefinementFailed,
    /// A direction whose once-differenced knot vector failed to
    /// materialise.
    DerivedKnots,
}

impl PatchBoundError {
    /// The refusal's prose — one spelling, shared by every consumer
    /// that reports a patch-bound refusal in its own error type.
    pub fn note(self) -> &'static str {
        match self {
            Self::DegreeZero => "degree-0 NURBS direction (a degenerate face description)",
            Self::Degree1Crease => {
                "degree-1 NURBS direction with interior knots (a C⁰ crease) — \
                 the interpolation Taylor bound needs C¹; split the face at \
                 the crease"
            }
            Self::Crease => {
                "NURBS direction with a C⁰ crease (interior multiplicity = \
                 degree) — the interpolation Taylor bound needs C¹; split \
                 the face at the crease"
            }
            Self::NonPositiveWeight => {
                "rational NURBS face with a non-positive or non-finite weight — an \
                 illegal rational description: the convex-combination licence every \
                 hull fact rests on requires strictly positive weights"
            }
            Self::RefinedWeightLostPositivity => {
                "rational NURBS face whose refined weights lost positivity — \
                 outside the certified inventory"
            }
            Self::RefinementFailed => {
                "rational NURBS face whose refinement fails to materialise — \
                 outside the certified inventory"
            }
            Self::DerivedKnots => {
                "NURBS direction whose derivative knot vector fails to materialise — \
                 outside the certified inventory"
            }
        }
    }
}

impl core::fmt::Display for PatchBoundError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.note())
    }
}

impl core::error::Error for PatchBoundError {}

/// One knot-span cell's certified bounds on the patch's partials, with
/// the UV rectangle they hold on.
///
/// # What the enclosure encloses (the rational arm's caveat, stated)
///
/// The INTEGRAL arm assembles on the described control net, so its
/// enclosures are enclosures of the described patch, full stop.
///
/// The RATIONAL arm first inserts [`RATIONAL_CERT_SPLITS`] knots per
/// span. Knot insertion is evaluation-invariant **in ℝ**, but the
/// refined control net is materialised in `f64` and therefore rounded:
/// what these cells enclose is the refined-`f64` patch, which differs
/// from the described one by insertion rounding. The gap is dust — at
/// the scale of an ulp of the coordinate — and it is NOT bounded here.
///
/// **It is visible, and it matters at exactly one place: a component
/// whose true value is structurally zero.** On a ruled (degree-1 in
/// `v`) rational face the true `S_vv` is identically zero, and 6 of
/// the quarter cylinder's 256 cells report a `z` enclosure of
/// `[-4.1e-15, -3.4e-15]` — sound about the refined patch, and it
/// EXCLUDES the described patch's zero. A consumer that reads these
/// as "the described surface's partial lies in here" is right to
/// within insertion dust and wrong beyond it, and a consumer testing a
/// STRUCTURAL predicate (`contains(0)`, an exact sign) must not use
/// them on the rational arm.
///
/// This is not new with the signed reading — the rational arm has
/// always refined — but the retired magnitude reading could never
/// exhibit it: `[0, m]` contains zero by construction, so the sup
/// spelling hid the gap rather than being free of it. Making the
/// signed reading the only one is what puts the caveat in the
/// contract, which is where it belongs.
///
/// Closing the gap needs an enclosure of the insertion rounding
/// itself, carried through the refinement — a real piece of work with
/// its own cost, deliberately not done here.
#[derive(Clone, Copy, Debug)]
pub struct PatchCell {
    /// The cell's `u` extent, `[lo, hi]`.
    pub u: (f64, f64),
    /// The cell's `v` extent, `[lo, hi]`.
    pub v: (f64, f64),
    /// Signed componentwise enclosure of `S_u` on the cell — of the
    /// assembled patch, refined-`f64` on the rational arm (see the
    /// type's docs, "What the enclosure encloses").
    pub s_u: [RingInterval; 3],
    /// Signed componentwise enclosure of `S_v` on the cell (the
    /// provenance caveat on [`PatchCell::s_u`] applies to every field).
    pub s_v: [RingInterval; 3],
    /// Signed componentwise enclosure of `S_uu` on the cell (the
    /// provenance caveat on [`PatchCell::s_u`] applies to every field).
    pub s_uu: [RingInterval; 3],
    /// Signed componentwise enclosure of `S_uv` on the cell (the
    /// provenance caveat on [`PatchCell::s_u`] applies to every field).
    pub s_uv: [RingInterval; 3],
    /// Signed componentwise enclosure of `S_vv` on the cell (the
    /// provenance caveat on [`PatchCell::s_u`] applies to every field).
    pub s_vv: [RingInterval; 3],
}

/// Whether a patch is rational under the kernel's definition (any
/// weight not bitwise `1.0`) — the arm selection, on f64 STRUCTURE.
pub fn is_rational(n: &NurbsSurface<f64>) -> bool {
    n.weights().iter().any(|w| *w != 1.0)
}

/// **The per-cell certified bounds** on a described NURBS patch's
/// first and second partials (module docs).
///
/// Cell granularity is the arm's own: the integral arm reports the
/// raw knot-span cells, the rational arm the cells of the fixed
/// [`RATIONAL_CERT_SPLITS`] refinement.
///
/// # Errors
///
/// [`PatchBoundError`] — a C⁰-creased or degree-0 direction, an
/// illegal rational description, or a refinement/derived-knot
/// construction that fails to materialise.
pub fn patch_cells(n: &NurbsSurface<f64>) -> Result<Vec<PatchCell>, PatchBoundError> {
    check_direction(n.knots_u())?;
    check_direction(n.knots_v())?;
    if is_rational(n) {
        rational_cells(n, RATIONAL_CERT_SPLITS)
    } else {
        integral_cells(n)
    }
}

/// [`patch_cells`] on an EXPLICIT refinement schedule: every nonempty
/// span of every direction is cut into `splits` equal pieces first,
/// in **both** arms.
///
/// Why a consumer would ask for more than [`patch_cells`] gives: a
/// B-spline coefficient hull over one knot span covers the whole
/// `(p + 1)`-span support of the basis functions active there, so its
/// width shrinks only LINEARLY in the span size, with a constant
/// `p + 1` times the cell. A sup-side consumer barely notices; an
/// inf-side one does, because the width is subtracted from the
/// quantity it is trying to prove positive. The measured case is the
/// exactly-umbilic sphere band: at 16 splits the first fundamental
/// form's own `F` — identically zero on an orthogonal chart —
/// encloses as `[−0.82, 0.81]`, and the certified curvature range
/// comes out twenty times wider than the surface's single value.
/// Refinement is exact in ℝ, so this buys tightness and nothing else.
///
/// # Errors
///
/// As [`patch_cells`], plus [`PatchBoundError::RefinementFailed`].
pub fn patch_cells_refined(
    n: &NurbsSurface<f64>,
    splits: usize,
) -> Result<Vec<PatchCell>, PatchBoundError> {
    check_direction(n.knots_u())?;
    check_direction(n.knots_v())?;
    if is_rational(n) {
        rational_cells(n, splits)
    } else {
        let refined = n
            .refine_knots_u(&split_points(n.knots_u(), splits))
            .and_then(|r| r.refine_knots_v(&split_points(r.knots_v(), splits)))
            .map_err(|_| PatchBoundError::RefinementFailed)?;
        integral_cells(&refined)
    }
}

/// The C¹ gate per direction: degree 0 refuses; degree 1 must be
/// single-span (an interior knot is a C⁰ crease); degree ≥ 2 needs
/// interior multiplicities ≤ p − 1.
///
/// # Errors
///
/// [`PatchBoundError::DegreeZero`], [`PatchBoundError::Degree1Crease`],
/// [`PatchBoundError::Crease`].
pub fn check_direction(kv: &KnotVector) -> Result<(), PatchBoundError> {
    let p = kv.degree();
    if p == 0 {
        return Err(PatchBoundError::DegreeZero);
    }
    if p == 1 {
        return if kv.interior_knots().next().is_none() {
            Ok(())
        } else {
            Err(PatchBoundError::Degree1Crease)
        };
    }
    if kv.interior_knots().any(|(_, m)| m > p - 1) {
        return Err(PatchBoundError::Crease);
    }
    Ok(())
}

/// The once-differenced knot vector (drop the outer knot pair, degree
/// − 1).
///
/// # Errors
///
/// [`PatchBoundError::DerivedKnots`] when the result is not a valid
/// clamped vector.
pub fn derived_knots(kv: &KnotVector) -> Result<KnotVector, PatchBoundError> {
    let inner = kv.knots()[1..kv.knots().len() - 1].to_vec();
    KnotVector::clamped(inner, kv.degree() - 1).map_err(|_| PatchBoundError::DerivedKnots)
}

/// The interior split points of the fixed rational refinement
/// schedule for one knot vector ([`RATIONAL_CERT_SPLITS`] equal
/// pieces per nonempty span), skipping any split point floating point
/// collapses onto a span end — refinement is a tightening, never a
/// correctness condition.
pub fn rational_split_points(kv: &KnotVector) -> Vec<f64> {
    split_points(kv, RATIONAL_CERT_SPLITS)
}

/// **Near-twin, recorded and deliberately not unified**:
/// `geom_brep::props::quad`'s `knot_aligned_cuts` builds the same
/// concept for the rational patch-flux composite — a knot-aligned
/// subdivision of a parameter range, with its own sliver guard — and
/// arrived at the same sliver lesson independently. Unifying the two
/// is Track R's consolidation ground (C-m/D30, gated behind #723),
/// not either caller's.
///
/// The interior split points that cut every nonempty span of `kv`
/// into `splits` equal pieces, skipping any point floating point
/// collapses onto a span end — refinement is a tightening, never a
/// correctness condition (the speed meter's rule, verbatim).
pub fn split_points(kv: &KnotVector, splits: usize) -> Vec<f64> {
    let mut add = Vec::new();
    for span in kv.first_span()..=kv.last_span() {
        if !kv.span_is_nonempty(span) {
            continue;
        }
        let (Some(&lo), Some(&hi)) = (kv.knots().get(span), kv.knots().get(span + 1)) else {
            continue;
        };
        for k in 1..splits {
            #[allow(clippy::cast_precision_loss)]
            let f = k as f64 / splits as f64;
            let u = lo + (hi - lo) * f;
            if u > lo && u < hi {
                add.push(u);
            }
        }
    }
    add
}

/// A coefficient net as ring enclosures — the shared tensor assembly,
/// homed in [`geom_core::spline::net`] (issue 1006). The alias is kept
/// so this module's own prose and its consumers keep naming the thing
/// they read; the differencing is not this module's any more.
pub type Net = TensorNet;

/// The signed hull of `a[i][j] − c·w[i][j]` over the window
/// `wu × wv` — the recentred homogeneous net `Ã = A − c·w` read
/// through the linearity of knot differencing (`d(A − c·w) = dA −
/// c·dw`, entrywise, same knots). Out-of-range indices poison.
///
/// This module's own READING, not the shared assembly: no other
/// consumer of a tensor net recentres at the hull read, because no
/// other one has a cell-local centre to recentre against.
///
/// Fixed association (D9): `u`-major, `i` outer and `j` inner, hulled
/// left to right — [`TensorNet::window_hull`]'s order, so the two
/// readings of one net agree on which coefficient came first.
pub fn window_tilde_hull(
    a: &Net,
    w: &Net,
    c: RingInterval,
    wu: &RangeInclusive<usize>,
    wv: &RangeInclusive<usize>,
) -> RingInterval {
    let mut acc: Option<RingInterval> = None;
    for i in wu.clone() {
        for j in wv.clone() {
            let e = a.get(i, j) - c * w.get(i, j);
            acc = Some(match acc {
                None => e,
                Some(h) => RingInterval::hull(h, e),
            });
        }
    }
    acc.unwrap_or_else(RingInterval::poison)
}

/// The signed hull of `net[i][j]` over the window `wu × wv` —
/// [`TensorNet::window_hull`], re-exported under the name this
/// module's consumers already use.
///
/// Distinct from [`window_tilde_hull`] with a zero centre on purpose:
/// that spelling computes `a − 0·w`, and the ring's outward rounding
/// makes the subtraction widen the answer by an ulp — enough to put a
/// CELL's bound above the whole-patch hull it is a subset of.
pub fn window_hull(
    net: &Net,
    wu: &RangeInclusive<usize>,
    wv: &RangeInclusive<usize>,
) -> RingInterval {
    net.window_hull(wu, wv)
}

/// **The squared-sum collapse of one signed componentwise enclosure**:
/// `sum over c of sup squared`, whose `sqrt(hi)` is a sup bound on the
/// vector's norm. One spelling, consumed wherever a vector partial's
/// magnitude is read off its signed enclosure.
///
/// Fixed association (D9): channel order `x, y, z`, accumulated left
/// to right from the ring zero. Poison in one channel poisons the sum.
#[must_use]
pub fn sq_norm(v: [RingInterval; 3]) -> RingInterval {
    v.iter().fold(RingInterval::zero(), |acc, c| acc + c.sqr())
}

/// A span's `[knot, next knot]` extent (the caller has already
/// established the span is nonempty, so both knots exist).
fn span_extent(kv: &KnotVector, span: usize) -> (f64, f64) {
    let k = kv.knots();
    (
        k.get(span).copied().unwrap_or(f64::NAN),
        k.get(span + 1).copied().unwrap_or(f64::NAN),
    )
}

/// The three spatial channels of a control net, as ring points.
///
/// (`offset_fit::channel` is the same extraction in the row-major
/// slice shape `PatchSpans::decompose` consumes; the two shapes have
/// different consumers and are bridged rather than unified — see that
/// function for the argument.)
///
/// **The two no longer share their arithmetic, and the divergence is
/// deliberate.** This one extracts `w·P`; `offset_fit::channel`
/// extracts `w·(P − c)` against a whole-patch recentring origin,
/// because its net feeds polynomial products formed once over the
/// merged break structure, where the ring's rounding scales with the
/// coordinate. This site recentres too, but LATER and per cell
/// ([`window_tilde_hull`]), off the cell's own control window — the
/// tighter centre, available here because a cell-local hull is what
/// is being read. So a change to one is no longer automatically a
/// change to both. What they still share is the ORDER
/// (`weight · coordinate`), and a change to THAT is a change to both.
/// Unifying the two centres and the shape they are computed in is the
/// patch-hull consolidation's (issue 1006, CERT-10), which owns this
/// seam.
fn comp_nets(n: &NurbsSurface<f64>, weighted: bool) -> Vec<Net> {
    let (nu, nv) = n.control_counts();
    (0..3)
        .map(|c| {
            Net::from_fn(nu, nv, |i, j| {
                // Row-major layout: control[iu·nv + iv] — the net's own.
                let p = n.control()[i * nv + j];
                let x = RingInterval::point(match c {
                    0 => p.x,
                    1 => p.y,
                    _ => p.z,
                });
                if weighted {
                    RingInterval::point(n.weights()[i * nv + j]) * x
                } else {
                    x
                }
            })
        })
        .collect()
}

/// The five per-direction derivative nets one channel needs.
struct DNets {
    d10: Net,
    d01: Net,
    d11: Net,
    d20: Option<Net>,
    d02: Option<Net>,
}

impl DNets {
    fn build(
        base: &Net,
        kv_u: &KnotVector,
        kv_v: &KnotVector,
        kv_u1: Option<&KnotVector>,
        kv_v1: Option<&KnotVector>,
    ) -> Self {
        let d10 = base.diff_u_knots(kv_u);
        let d01 = base.diff_v_knots(kv_v);
        let d11 = d10.diff_v_knots(kv_v);
        let d20 = kv_u1.map(|k1| d10.diff_u_knots(k1));
        let d02 = kv_v1.map(|k1| d01.diff_v_knots(k1));
        Self {
            d10,
            d01,
            d11,
            d20,
            d02,
        }
    }
}

/// The active windows of one cell, both directions, all three orders.
struct CellWindows {
    u_val: RangeInclusive<usize>,
    v_val: RangeInclusive<usize>,
    u_d1: RangeInclusive<usize>,
    v_d1: RangeInclusive<usize>,
    u_d2: Option<RangeInclusive<usize>>,
    v_d2: Option<RangeInclusive<usize>>,
}

/// Assembles a cell from the five signed componentwise enclosures
/// (`S_u, S_v, S_uu, S_uv, S_vv`, in that order).
fn cell_from(uv: ((f64, f64), (f64, f64)), signed: [[RingInterval; 3]; 5]) -> PatchCell {
    PatchCell {
        u: uv.0,
        v: uv.1,
        s_u: signed[0],
        s_v: signed[1],
        s_uu: signed[2],
        s_uv: signed[3],
        s_vv: signed[4],
    }
}

/// The INTEGRAL arm (all weights bitwise `1.0`): the plain hull
/// assembly on the spatial nets — no quotient rule intervenes, so the
/// enclosure IS the coefficient hull.
fn integral_cells(n: &NurbsSurface<f64>) -> Result<Vec<PatchCell>, PatchBoundError> {
    let (kv_u, kv_v) = (n.knots_u(), n.knots_v());
    let kv_u1 = (kv_u.degree() >= 2)
        .then(|| derived_knots(kv_u))
        .transpose()?;
    let kv_v1 = (kv_v.degree() >= 2)
        .then(|| derived_knots(kv_v))
        .transpose()?;
    let nets: Vec<DNets> = comp_nets(n, false)
        .iter()
        .map(|base| DNets::build(base, kv_u, kv_v, kv_u1.as_ref(), kv_v1.as_ref()))
        .collect();
    let zero = RingInterval::zero();
    let mut cells = Vec::new();
    for su in kv_u.first_span()..=kv_u.last_span() {
        let Some(span_u) = kv_u.span(su) else {
            continue;
        };
        for sv in kv_v.first_span()..=kv_v.last_span() {
            let Some(span_v) = kv_v.span(sv) else {
                continue;
            };
            let w = CellWindows {
                u_val: span_u.window(),
                v_val: span_v.window(),
                u_d1: span_u.first_derived_window(),
                v_d1: span_v.first_derived_window(),
                u_d2: span_u.derived_window(2),
                v_d2: span_v.derived_window(2),
            };
            let mut s_u = [zero; 3];
            let mut s_v = [zero; 3];
            let mut s_uu = [zero; 3];
            let mut s_uv = [zero; 3];
            let mut s_vv = [zero; 3];
            for (c, d) in nets.iter().enumerate() {
                let g20 = d
                    .d20
                    .as_ref()
                    .zip(w.u_d2.as_ref())
                    .map_or(zero, |(net, wu2)| window_hull(net, wu2, &w.v_val));
                let g02 = d
                    .d02
                    .as_ref()
                    .zip(w.v_d2.as_ref())
                    .map_or(zero, |(net, wv2)| window_hull(net, &w.u_val, wv2));
                let g11 = window_hull(&d.d11, &w.u_d1, &w.v_d1);
                let g10 = window_hull(&d.d10, &w.u_d1, &w.v_val);
                let g01 = window_hull(&d.d01, &w.u_val, &w.v_d1);
                s_uu[c] = g20;
                s_vv[c] = g02;
                s_uv[c] = g11;
                s_u[c] = g10;
                s_v[c] = g01;
            }
            cells.push(cell_from(
                (span_extent(kv_u, su), span_extent(kv_v, sv)),
                [s_u, s_v, s_uu, s_uv, s_vv],
            ));
        }
    }
    Ok(cells)
}

/// The RATIONAL arm: the quotient-rule assembly over the homogeneous
/// nets, on the cells of the fixed [`RATIONAL_CERT_SPLITS`]
/// refinement (module docs).
#[allow(clippy::too_many_lines)]
fn rational_cells(n: &NurbsSurface<f64>, splits: usize) -> Result<Vec<PatchCell>, PatchBoundError> {
    // The convex-combination licence, on f64 STRUCTURE. `!(w > 0.0)`
    // catches NaN. (`NurbsSurface::new` refuses these at the door;
    // re-checked here so THIS bound never divides by an unproven
    // denominator.)
    #[allow(clippy::neg_cmp_op_on_partial_ord)]
    if n.weights().iter().any(|w| !(*w > 0.0) || !w.is_finite()) {
        return Err(PatchBoundError::NonPositiveWeight);
    }
    let refined = n
        .refine_knots_u(&split_points(n.knots_u(), splits))
        .and_then(|r| r.refine_knots_v(&split_points(r.knots_v(), splits)))
        .map_err(|_| PatchBoundError::RefinementFailed)?;
    let r = &refined;
    // Positivity survives insertion in ℝ (convex combinations); this
    // code may not assume floating point did (the speed meter's rule).
    #[allow(clippy::neg_cmp_op_on_partial_ord)]
    if r.weights().iter().any(|w| !(*w > 0.0) || !w.is_finite()) {
        return Err(PatchBoundError::RefinedWeightLostPositivity);
    }
    let (kv_u, kv_v) = (r.knots_u(), r.knots_v());
    let (pu, pv) = (kv_u.degree(), kv_v.degree());
    let (nu, nv) = r.control_counts();
    let w_grid = Net::from_fn(nu, nv, |i, j| RingInterval::point(r.weights()[i * nv + j]));
    // Second derivatives along a degree-1 direction are EXACTLY zero
    // in ℝ for the polynomial nets A and w (the direction is a single
    // linear span pre-refinement — the C¹ gate — and refinement's
    // inserted knots are removable), so those nets are `None` and
    // their terms exact zeros; the CROSS terms stay.
    let kv_u1 = (pu >= 2).then(|| derived_knots(kv_u)).transpose()?;
    let kv_v1 = (pv >= 2).then(|| derived_knots(kv_v)).transpose()?;
    let w_nets = DNets::build(&w_grid, kv_u, kv_v, kv_u1.as_ref(), kv_v1.as_ref());
    let a_nets: Vec<DNets> = comp_nets(r, true)
        .iter()
        .map(|g| DNets::build(g, kv_u, kv_v, kv_u1.as_ref(), kv_v1.as_ref()))
        .collect();
    let zero = RingInterval::zero();
    let two = RingInterval::point(2.0);
    let mut cells: Vec<PatchCell> = Vec::new();
    for su in kv_u.first_span()..=kv_u.last_span() {
        for sv in kv_v.first_span()..=kv_v.last_span() {
            // Emptiness skip, span validation and window construction
            // in one operation, both directions.
            let Some(win) = r.window(su, sv) else {
                continue;
            };
            let (span_u, span_v) = (win.span_u(), win.span_v());
            let w = CellWindows {
                u_val: span_u.window(),
                v_val: span_v.window(),
                u_d1: span_u.first_derived_window(),
                v_d1: span_v.first_derived_window(),
                u_d2: span_u.derived_window(2),
                v_d2: span_v.derived_window(2),
            };
            // The cell centroid — a translation CHOICE (any finite c
            // is sound), computed on f64 structure, fixed order.
            let mut csum = [0.0f64; 3];
            let mut count = 0.0f64;
            for i in 0..=span_u.degree() {
                let row = win.row(i);
                for j in 0..=span_v.degree() {
                    let p = r.control()[row + j];
                    csum[0] += p.x;
                    csum[1] += p.y;
                    csum[2] += p.z;
                    count += 1.0;
                }
            }
            let c = [csum[0] / count, csum[1] / count, csum[2] / count];
            // The cell's weight hull — the divisor (module docs).
            let w_cell = window_hull(&w_grid, &w.u_val, &w.v_val);
            // Weight-net hulls on the cell.
            let w10s = window_hull(&w_nets.d10, &w.u_d1, &w.v_val);
            let w01s = window_hull(&w_nets.d01, &w.u_val, &w.v_d1);
            let w11s = window_hull(&w_nets.d11, &w.u_d1, &w.v_d1);
            let w20s = w_nets
                .d20
                .as_ref()
                .zip(w.u_d2.as_ref())
                .map_or(zero, |(net, wu2)| window_hull(net, wu2, &w.v_val));
            let w02s = w_nets
                .d02
                .as_ref()
                .zip(w.v_d2.as_ref())
                .map_or(zero, |(net, wv2)| window_hull(net, &w.u_val, wv2));
            let mut s_u = [zero; 3];
            let mut s_v = [zero; 3];
            let mut s_uu = [zero; 3];
            let mut s_uv = [zero; 3];
            let mut s_vv = [zero; 3];
            for (comp, a) in a_nets.iter().enumerate() {
                let cc = RingInterval::point(c[comp]);
                // The rational VALUE hull on the cell: positive
                // weights make the rational basis a nonnegative
                // partition of unity over the ACTIVE control points,
                // so `S − c` lies in the hull of `P − c`.
                let mut v0h: Option<RingInterval> = None;
                for i in 0..=span_u.degree() {
                    let row = win.row(i);
                    for j in 0..=span_v.degree() {
                        let p = r.control()[row + j];
                        let e = RingInterval::point(match comp {
                            0 => p.x,
                            1 => p.y,
                            _ => p.z,
                        }) - cc;
                        v0h = Some(match v0h {
                            None => e,
                            Some(h) => RingInterval::hull(h, e),
                        });
                    }
                }
                let v0s = v0h.unwrap_or_else(RingInterval::poison);
                // Recentred homogeneous derivative hulls
                // `Ã_kl = A_kl − c·w_kl` on the cell.
                let at =
                    |an: &Net, wn: &Net, wu: &RangeInclusive<usize>, wv: &RangeInclusive<usize>| {
                        window_tilde_hull(an, wn, cc, wu, wv)
                    };
                let a10s = at(&a.d10, &w_nets.d10, &w.u_d1, &w.v_val);
                let a01s = at(&a.d01, &w_nets.d01, &w.u_val, &w.v_d1);
                let a11s = at(&a.d11, &w_nets.d11, &w.u_d1, &w.v_d1);
                let a20s = match (a.d20.as_ref(), w_nets.d20.as_ref(), w.u_d2.as_ref()) {
                    (Some(an), Some(wn), Some(wu2)) => at(an, wn, wu2, &w.v_val),
                    _ => zero,
                };
                let a02s = match (a.d02.as_ref(), w_nets.d02.as_ref(), w.v_d2.as_ref()) {
                    (Some(an), Some(wn), Some(wv2)) => at(an, wn, &w.u_val, wv2),
                    _ => zero,
                };
                // The quotient rule itself, in the ring, divided by
                // the whole weight hull.
                let s1u = (a10s - v0s * w10s) / w_cell;
                let s1v = (a01s - v0s * w01s) / w_cell;
                s_u[comp] = s1u;
                s_v[comp] = s1v;
                s_uu[comp] = (a20s - two * s1u * w10s - v0s * w20s) / w_cell;
                s_vv[comp] = (a02s - two * s1v * w01s - v0s * w02s) / w_cell;
                s_uv[comp] = (a11s - s1u * w01s - s1v * w10s - v0s * w11s) / w_cell;
            }
            cells.push(cell_from(
                (span_extent(kv_u, su), span_extent(kv_v, sv)),
                [s_u, s_v, s_uu, s_uv, s_vv],
            ));
        }
    }
    Ok(cells)
}
