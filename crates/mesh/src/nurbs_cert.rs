//! **The trimmed-NURBS deviation certificate** (M7, the montage
//! skin-scenes unit): a certified per-face Hessian sup bound from the
//! control net, and the per-triangle interpolation bound it feeds —
//! the torus certificate's derivation generalized from a closed-form
//! Hessian to a **hull-derived** one.
//!
//! # The Hessian bound is a convexity fact, not an estimate
//!
//! For a NON-RATIONAL tensor-product B-spline surface
//! `S(u,v) = Σᵢⱼ Nᵢ(u)·Nⱼ(v)·Pᵢⱼ`, each second partial is itself a
//! tensor-product B-spline whose coefficient net comes from knot
//! differencing (The NURBS Book eq. 3.24, applied per direction —
//! exactly [`geom_core::spline::hull::derivative_coeffs`], iterated):
//! `S_uu` differences twice along `u`, `S_vv` twice along `v`, `S_uv`
//! once along each. The B-spline bases in both directions are
//! nonnegative partitions of unity (degree ≥ 0), so every value of
//! `S_uu` (etc.) is a convex combination of its derived coefficients
//! and lies in their hull — the C2.2 mechanism `chords` already uses
//! for edge carriers, lifted to the surface. Componentwise hulls give
//! `sup‖S_uu‖ ≤ √(Σ_c sup²)` = [`NurbsFaceBound::muu`], and likewise
//! `muv`, `mvv`. Rounding: interval (ring) arithmetic end to end, with
//! a final `next_up` on the square root.
//!
//! # The per-triangle certificate
//!
//! For a mesh triangle with UV corners spanning an axis-aligned box of
//! extents `(a_u, a_v)`, expand `S` to first order at the box center
//! `w₀` (the surface is defined on the whole chart rectangle, so the
//! box is in-domain). Every triangle point `w` has
//! `|w_u − w₀_u| ≤ a_u/2`, `|w_v − w₀_v| ≤ a_v/2`, and the integral
//! Taylor remainder (valid for C¹ surfaces with an a.e. second
//! derivative bound — hence the C¹ gate below) gives
//!
//! ```text
//! |S(w) − T₁(w)| ≤ ½·(muu·(a_u/2)² + 2·muv·(a_u/2)(a_v/2) + mvv·(a_v/2)²)
//!               = Q/8,   Q := muu·a_u² + 2·muv·a_u·a_v + mvv·a_v²
//! ```
//!
//! The affine interpolant Π agrees with `S` at the three vertices, so
//! `|Π − T₁| ≤ Q/8` at the vertices; `Π − T₁` is affine over the
//! triangle, hence `≤ Q/8` everywhere on it. Total: `‖S − Π‖ ≤ Q/4` —
//! [`NurbsFaceBound::cert`]. (Sanity: with `muu = R+r`, `muv = r`,
//! `mvv = r` and `a_u, a_v ≤ L` this is at most `(R+2r)·L²/2`,
//! strictly inside the torus certificate's `(3/4)(R+2r)·L²` — same
//! derivation, anisotropic accounting.)
//!
//! As everywhere in this crate the two documented additive slacks (≤ ε
//! boundary-carrier residual, f64 evaluation rounding) sit OUTSIDE the
//! bound: the honest promise is δ + ε (crate docs).
//!
//! # The rational arm (M8-5)
//!
//! A RATIONAL face is `S = A/w` with `A = ΣΣ Nᵢ Nⱼ wᵢⱼ Pᵢⱼ` and
//! `w = ΣΣ Nᵢ Nⱼ wᵢⱼ` — both POLYNOMIAL tensor-product B-splines, so
//! every ingredient below is the same control-hull convexity fact as
//! the integral arm, taken on the homogeneous nets. The quotient rule
//! (exactly `NurbsSurface::ders_in_span`'s corrections) gives, for any
//! constant `c` (write `Ã = A − c·w`, so `S − c = Ã/w` and the
//! derivatives of `S` are unchanged):
//!
//! ```text
//! S_u  = (Ã_u  − (S − c)·w_u) / w                        (v symmetric)
//! S_uu = (Ã_uu − 2·S_u·w_u − (S − c)·w_uu) / w           (v symmetric)
//! S_uv = (Ã_uv − S_u·w_v − S_v·w_u − (S − c)·w_uv) / w
//! ```
//!
//! Taking sups componentwise over one knot-span cell:
//!
//! - `sup|Ã_kl|` — hull of the recentred homogeneous derivative
//!   coefficients active on the cell ([`hull::derivative_coeffs`]
//!   iterated, exactly as the integral arm; recentring commutes with
//!   knot differencing, `d(A − c·w) = dA − c·dw`);
//! - `sup|S^c − c^c| ≤ max_active |P^c − c^c|` — the rational value
//!   hull: strictly positive weights make the rational basis a
//!   nonnegative partition of unity over the active net (the licence,
//!   checked; refused typed otherwise);
//! - `sup|S_u|` — the same recurrence one order down;
//! - `sup|w_kl|` — the weight spline's own derivative hulls.
//!
//! **The divisor is `w_min`, argued not assumed.** On the cell,
//! `w ∈ [w_min, w_max]` of the active weights (convex combination).
//! Every numerator bound above is a NONNEGATIVE magnitude sup, and for
//! a nonnegative numerator the conservative (sup-side) division is by
//! the SMALLEST denominator: `|X|/w ≤ sup|X|/w_min`. This is the
//! mirror image of the speed meter's lower-bound choice
//! (`geom_curves::rational_speed_lower_bound`: a nonnegative numerator
//! divides by `w_max` for an INF bound) — same lattice, opposite side.
//! The interval division by the cell's weight hull `[w_lo, w_hi]`
//! computes exactly `sup/w_lo`, outward-rounded, and poisons if
//! positivity was never proven.
//!
//! **Recentring keeps the cross terms cell-sized**: with the cell's
//! control centroid as `c`, `sup|S − c|` is a cell-of-control-net
//! fact, not a whole-patch one, so `(S − c)·w_dd` and `S_d·w_d` do not
//! inflate with the patch's distance from the origin (the M8-2
//! template's trick, lifted to two parameters). The whole-domain bound
//! is the max over cells of the per-cell sups, after the FIXED
//! [`RATIONAL_CERT_SPLITS`] refinement (schedule docs there).
//!
//! A degree-1 direction's `Ã_dd`, `w_dd` are exactly zero, but its
//! CROSS terms survive — a rational degree-1 direction genuinely
//! curves in parameter (a Möbius-reparameterized ruling), so its
//! second partials are NOT the integral arm's exact zero. The
//! recurrences above carry that automatically.
//!
//! The certificate consumer is unchanged: the Q/4 Taylor bound only
//! needs C¹ with an a.e. second-derivative bound, and the C¹ gate
//! (interior multiplicities ≤ p − 1, on the homogeneous nets) plus
//! `w > 0` gives exactly that for `S = A/w`.
//!
//! # Grid sizing (heuristic; the certificate is the guarantee)
//!
//! Budgeting a triangle's box at two grid cells per axis
//! (`a_u ≤ 2·h_u`, `a_v ≤ 2·h_v`) and splitting δ_s across the `u`/`v`
//! groups via `2·a_u·a_v ≤ a_u² + a_v²`:
//!
//! ```text
//! cert ≤ (muu + muv)·h_u² + (mvv + muv)·h_v²  ⇒
//! h_u = √(δ_s / (2(muu + muv))),   h_v = √(δ_s / (2(mvv + muv)))
//! ```
//!
//! A zero group (e.g. a degree-1 direction of a ruled wall with
//! `muv = 0`) leaves that direction unconstrained — step ∞, one cell.
//! The same steps bound the BOUNDARY chord schedule of every adjacent
//! edge (`chords`: the adjacent-torus tightening pattern), so boundary
//! triangles obey the same budget.
//!
//! # Covered vs refused (partial coverage, stated)
//!
//! Covered: described, per direction either degree ≥ 2 with interior
//! multiplicities ≤ p − 1 (C¹) or degree 1 single-span — integral
//! faces through the direct hull arm, rational faces (any weight not
//! bitwise 1, all strictly positive) through the quotient-rule arm
//! above (M8-5; the loft/sweep wall class plus the arc-walled bodies
//! M8-2 made buildable). Refused typed
//! ([`TessellateError::UnsupportedNurbsFace`]): illegal rational
//! descriptions (a non-positive/non-finite weight voids the
//! convex-combination licence), C⁰ creases (the Taylor remainder
//! needs C¹ — for the standard multi-arc rational quadratic that
//! means split at the double knots), degree-0 directions, and
//! poisoned/non-finite hulls. The placeholder refuses
//! [`TessellateError::UnsupportedSurface`] upstream in `trimmed`.

use geom_core::ring_interval::RingInterval;
use geom_core::spline::KnotVector;
use geom_core::spline::hull::derivative_coeffs;
use geom_surfaces::NurbsSurface;
use topo::FaceKey;

use crate::types::TessellateError;

/// Certified sup bounds on the three second partials of one described
/// non-rational NURBS face, over its whole chart rectangle (module
/// docs). Computed once per face; consumed by the trimmed lane's grid
/// sizing, its per-triangle certificate, and the chord pass's
/// adjacent-face tightening.
#[derive(Clone, Copy, Debug)]
pub(crate) struct NurbsFaceBound {
    /// `sup ‖S_uu‖` (outward-rounding dust for a degree-1 single-span
    /// u direction — the exact-zero term still passes through the
    /// ring's conservative arithmetic).
    pub muu: f64,
    /// `sup ‖S_uv‖`.
    pub muv: f64,
    /// `sup ‖S_vv‖` (dust for a degree-1 single-span v direction).
    pub mvv: f64,
}

impl NurbsFaceBound {
    /// The per-triangle deviation certificate `Q/4` (module docs) for
    /// a triangle whose UV corners are `uv`.
    pub fn cert(&self, uv: [[f64; 2]; 3]) -> f64 {
        let au = max3(uv[0][0], uv[1][0], uv[2][0]) - min3(uv[0][0], uv[1][0], uv[2][0]);
        let av = max3(uv[0][1], uv[1][1], uv[2][1]) - min3(uv[0][1], uv[1][1], uv[2][1]);
        0.25 * (self.muu * au.powi(2) + 2.0 * self.muv * au * av + self.mvv * av.powi(2))
    }

    /// The `(h_u, h_v)` UV grid steps for sizing target `delta_s`
    /// (module docs) — `f64::INFINITY` for an unconstrained direction
    /// ([`crate::chords::ceil_count`] turns that into one cell).
    pub fn grid_steps(&self, delta_s: f64) -> (f64, f64) {
        let step = |group: f64| {
            if group > 0.0 {
                (delta_s / (2.0 * group)).sqrt()
            } else {
                f64::INFINITY
            }
        };
        (step(self.muu + self.muv), step(self.mvv + self.muv))
    }
}

fn max3(a: f64, b: f64, c: f64) -> f64 {
    a.max(b).max(c)
}

fn min3(a: f64, b: f64, c: f64) -> f64 {
    a.min(b).min(c)
}

/// The certified Hessian sup bounds of a described NURBS face, or the
/// typed refusal naming its class (module docs).
///
/// Two arms share the gates and the finite check: the INTEGRAL arm
/// (all weights bitwise `1.0` — the kernel's definition of
/// non-rational) is the original hull assembly, bit-identical; the
/// RATIONAL arm (M8-5) is the quotient-rule assembly over the
/// homogeneous nets (module docs, "The rational arm").
///
/// # Errors
///
/// [`TessellateError::UnsupportedNurbsFace`] — C⁰-creased, degree-0,
/// an illegal rational description (non-positive/non-finite weight),
/// or a poisoned/unbounded hull. The PLACEHOLDER is the caller's check
/// (it refuses `UnsupportedSurface`, the mvfs state's historical
/// variant).
pub(crate) fn nurbs_face_bound(
    n: &NurbsSurface<f64>,
    fk: FaceKey,
) -> Result<NurbsFaceBound, TessellateError> {
    check_direction(n.knots_u(), fk)?;
    check_direction(n.knots_v(), fk)?;
    let bound = if n.weights().iter().any(|w| *w != 1.0) {
        rational_face_bound(n, fk)?
    } else {
        integral_face_bound(n, fk)?
    };
    if !(bound.muu.is_finite() && bound.muv.is_finite() && bound.mvv.is_finite()) {
        return Err(TessellateError::UnsupportedNurbsFace {
            face: fk,
            note: "NURBS face second-derivative hull is unbounded/poisoned — \
                   outside the certified inventory",
        });
    }
    Ok(bound)
}

/// The integral (all-unit-weight) arm of [`nurbs_face_bound`]: the
/// direct control-hull convexity assembly (module docs). Bit-identical
/// to the pre-M8-5 path.
fn integral_face_bound(
    n: &NurbsSurface<f64>,
    fk: FaceKey,
) -> Result<NurbsFaceBound, TessellateError> {
    let (nu, nv) = n.control_counts();
    let comp = |c: usize| -> Vec<RingInterval> {
        n.control()
            .iter()
            .map(|p| {
                RingInterval::point(match c {
                    0 => p.x,
                    1 => p.y,
                    _ => p.z,
                })
            })
            .collect()
    };
    let mut sq_uu = RingInterval::zero();
    let mut sq_uv = RingInterval::zero();
    let mut sq_vv = RingInterval::zero();
    for c in 0..3 {
        // Row-major layout: control[iu·nv + iv] (NurbsSurface docs).
        let grid = comp(c);
        let u_rows: Vec<Vec<RingInterval>> = (0..nv)
            .map(|j| (0..nu).map(|i| grid[i * nv + j]).collect())
            .collect();
        let v_rows: Vec<Vec<RingInterval>> = (0..nu)
            .map(|i| (0..nv).map(|j| grid[i * nv + j]).collect())
            .collect();
        sq_uu = sq_uu + second_derivative_hull(n.knots_u(), &u_rows, fk)?.sqr();
        sq_vv = sq_vv + second_derivative_hull(n.knots_v(), &v_rows, fk)?.sqr();
        sq_uv = sq_uv + mixed_derivative_hull(n.knots_u(), n.knots_v(), &u_rows)?.sqr();
    }
    let m = |sq: RingInterval| sq.hi().sqrt().next_up();
    Ok(NurbsFaceBound {
        muu: m(sq_uu),
        muv: m(sq_uv),
        mvv: m(sq_vv),
    })
}

/// The fixed refinement schedule of the RATIONAL arms (this face bound
/// and `chords`' rational carrier bound): every nonempty span of every
/// direction splits into this many equal pieces before the per-cell
/// assembly. A CONSTANT (D9: structure, never a data-dependent
/// iteration) — the `RATIONAL_METER_SPLITS = 16` precedent of
/// `geom_curves`' rational speed meter, mirrored. Knot insertion is
/// evaluation-invariant in ℝ, so it changes no geometry; it only
/// shrinks every hull the bound is assembled from, which is what keeps
/// the `sup‖S − c‖·sup|w_dd|` cross terms cell-sized. (The f64
/// insertion arithmetic rounds; that residue sits inside the crate's
/// documented f64-evaluation slack — the ε side of δ + ε — exactly
/// where every `surface.eval` already spends it.)
pub(crate) const RATIONAL_CERT_SPLITS: usize = 16;

/// The interior split points of the fixed rational refinement schedule
/// for one knot vector (module-constant docs): `RATIONAL_CERT_SPLITS`
/// equal pieces per nonempty span, skipping any split point floating
/// point collapses onto a span end — refinement is a tightening, never
/// a correctness condition (the speed meter's rule, verbatim).
pub(crate) fn rational_split_points(kv: &KnotVector) -> Vec<f64> {
    let mut add = Vec::new();
    for span in kv.first_span()..=kv.last_span() {
        if !kv.span_is_nonempty(span) {
            continue;
        }
        let (Some(&lo), Some(&hi)) = (kv.knots().get(span), kv.knots().get(span + 1)) else {
            continue;
        };
        for k in 1..RATIONAL_CERT_SPLITS {
            #[allow(clippy::cast_precision_loss)]
            let f = k as f64 / RATIONAL_CERT_SPLITS as f64;
            let u = lo + (hi - lo) * f;
            if u > lo && u < hi {
                add.push(u);
            }
        }
    }
    add
}

/// A coefficient net as ring enclosures, u-major: `net[i][j]` is the
/// coefficient at u-index `i`, v-index `j`.
type Net = Vec<Vec<RingInterval>>;

/// Differences a net once along u against `kv_u` (per fixed v index):
/// `(nu − 1) × nv` from `nu × nv`.
fn net_d_u(kv_u: &KnotVector, net: &Net) -> Net {
    let nu = net.len();
    let nv = net.first().map_or(0, Vec::len);
    let cols: Vec<Vec<RingInterval>> = (0..nv)
        .map(|j| {
            let col: Vec<RingInterval> = (0..nu).map(|i| net[i][j]).collect();
            derivative_coeffs(kv_u, &col)
        })
        .collect();
    let nu1 = nu.saturating_sub(1);
    (0..nu1)
        .map(|i| {
            (0..nv)
                .map(|j| cols[j].get(i).copied().unwrap_or_else(RingInterval::poison))
                .collect()
        })
        .collect()
}

/// Differences a net once along v against `kv_v` (per fixed u index):
/// `nu × (nv − 1)`.
fn net_d_v(kv_v: &KnotVector, net: &Net) -> Net {
    net.iter().map(|row| derivative_coeffs(kv_v, row)).collect()
}

/// The signed hull of `a[i][j] − c·w[i][j]` over the window
/// `[i0, i1] × [j0, j1]` — the recentred homogeneous net `Ã = A − c·w`
/// read through the linearity of knot differencing (`d(A − c·w) =
/// dA − c·dw`, entrywise, same knots). Out-of-range indices poison.
fn window_tilde_hull(
    a: &Net,
    w: &Net,
    c: RingInterval,
    (i0, i1): (usize, usize),
    (j0, j1): (usize, usize),
) -> RingInterval {
    let mut acc: Option<RingInterval> = None;
    for i in i0..=i1 {
        for j in j0..=j1 {
            let (av, wv) = match (
                a.get(i).and_then(|r| r.get(j)),
                w.get(i).and_then(|r| r.get(j)),
            ) {
                (Some(&av), Some(&wv)) => (av, wv),
                _ => (RingInterval::poison(), RingInterval::poison()),
            };
            let e = av - c * wv;
            acc = Some(match acc {
                None => e,
                Some(h) => RingInterval::hull(h, e),
            });
        }
    }
    acc.unwrap_or_else(RingInterval::poison)
}

/// The `[0, sup]` magnitude enclosure of a signed hull (poison flows).
fn mag_iv(h: RingInterval) -> RingInterval {
    RingInterval::from_bounds(0.0, h.mag())
}

/// The rational (non-unit-weight) arm of [`nurbs_face_bound`] — the
/// M8-5 quotient-rule Hessian assembly. Derivation: module docs, "The
/// rational arm". The C¹ gates have already run (homogeneous C¹ plus
/// `w > 0` gives `S = A/w` C¹, which is what the Taylor certificate
/// needs).
fn rational_face_bound(
    n: &NurbsSurface<f64>,
    fk: FaceKey,
) -> Result<NurbsFaceBound, TessellateError> {
    // The convex-combination licence, on f64 STRUCTURE: every hull
    // fact below (the rational value hull, the weight-range divisor)
    // requires strictly positive finite weights. `!(w > 0.0)` catches
    // NaN. (`NurbsSurface::new` refuses these at the door; re-checked
    // here so THIS bound never divides by an unproven denominator.)
    #[allow(clippy::neg_cmp_op_on_partial_ord)]
    if n.weights().iter().any(|w| !(*w > 0.0) || !w.is_finite()) {
        return Err(TessellateError::UnsupportedNurbsFace {
            face: fk,
            note: "rational NURBS face with a non-positive or non-finite weight — an \
                   illegal rational description: the convex-combination licence every \
                   hull fact rests on requires strictly positive weights",
        });
    }
    // Fixed refinement (RATIONAL_CERT_SPLITS docs), both directions.
    let refined = n
        .refine_knots_u(&rational_split_points(n.knots_u()))
        .and_then(|r| r.refine_knots_v(&rational_split_points(r.knots_v())))
        .map_err(|_| TessellateError::UnsupportedNurbsFace {
            face: fk,
            note: "rational NURBS face whose refinement fails to materialise — \
                   outside the certified inventory",
        })?;
    let r = &refined;
    // Positivity survives insertion in ℝ (convex combinations); this
    // code may not assume floating point did (the speed meter's rule).
    #[allow(clippy::neg_cmp_op_on_partial_ord)]
    if r.weights().iter().any(|w| !(*w > 0.0) || !w.is_finite()) {
        return Err(TessellateError::UnsupportedNurbsFace {
            face: fk,
            note: "rational NURBS face whose refined weights lost positivity — \
                   outside the certified inventory",
        });
    }
    let (kv_u, kv_v) = (r.knots_u(), r.knots_v());
    let (pu, pv) = (kv_u.degree(), kv_v.degree());
    let (nu, nv) = r.control_counts();
    // Homogeneous nets: w and A^c = w·P^c per component, as ring
    // products of the exact f64 inputs.
    let w_grid: Net = (0..nu)
        .map(|i| {
            (0..nv)
                .map(|j| RingInterval::point(r.weights()[i * nv + j]))
                .collect()
        })
        .collect();
    let comp_grid = |c: usize| -> Net {
        (0..nu)
            .map(|i| {
                (0..nv)
                    .map(|j| {
                        let p = r.control()[i * nv + j];
                        RingInterval::point(r.weights()[i * nv + j])
                            * RingInterval::point(match c {
                                0 => p.x,
                                1 => p.y,
                                _ => p.z,
                            })
                    })
                    .collect()
            })
            .collect()
    };
    // Derivative nets. Second derivatives along a degree-1 direction
    // are EXACTLY zero in ℝ for the polynomial nets A and w (the
    // direction is a single linear span pre-refinement — the C¹ gate —
    // and refinement's inserted knots are removable), so those nets
    // are `None` and their terms exact zeros; the CROSS terms
    // (`S_d·w_d`, and `w_dd` of the other direction) stay, which is
    // where a rational degree-1 direction genuinely curves in
    // parameter.
    let kv_u1 = (pu >= 2).then(|| derived_knots(kv_u, fk)).transpose()?;
    let kv_v1 = (pv >= 2).then(|| derived_knots(kv_v, fk)).transpose()?;
    struct DNets {
        d10: Net,
        d01: Net,
        d11: Net,
        d20: Option<Net>,
        d02: Option<Net>,
    }
    let build = |base: &Net| -> DNets {
        let d10 = net_d_u(kv_u, base);
        let d01 = net_d_v(kv_v, base);
        let d11 = net_d_v(kv_v, &d10);
        let d20 = kv_u1.as_ref().map(|k1| net_d_u(k1, &d10));
        let d02 = kv_v1.as_ref().map(|k1| net_d_v(k1, &d01));
        DNets {
            d10,
            d01,
            d11,
            d20,
            d02,
        }
    };
    let w_nets = build(&w_grid);
    let a_nets: Vec<(Net, DNets)> = (0..3)
        .map(|c| {
            let g = comp_grid(c);
            let d = build(&g);
            (g, d)
        })
        .collect();
    // Per-cell assembly, hull-accumulated across cells (max of sups ==
    // hull of the squared enclosures; poison flows into the shared
    // finite check).
    let mut sq_uu: Option<RingInterval> = None;
    let mut sq_uv: Option<RingInterval> = None;
    let mut sq_vv: Option<RingInterval> = None;
    let acc = |slot: &mut Option<RingInterval>, v: RingInterval| {
        *slot = Some(match *slot {
            None => v,
            Some(h) => RingInterval::hull(h, v),
        });
    };
    let two = RingInterval::point(2.0);
    for su in kv_u.first_span()..=kv_u.last_span() {
        if !kv_u.span_is_nonempty(su) {
            continue;
        }
        for sv in kv_v.first_span()..=kv_v.last_span() {
            if !kv_v.span_is_nonempty(sv) {
                continue;
            }
            let (Some(iu0), Some(jv0)) = (su.checked_sub(pu), sv.checked_sub(pv)) else {
                return Err(TessellateError::MissingEntity {
                    what: "NURBS span below its degree",
                });
            };
            // Active windows on span (su, sv): value indices
            // [su−p, su]; each u/v differencing drops the top index.
            let wu_val = (iu0, su);
            let wv_val = (jv0, sv);
            let wu_d1 = (iu0, su - 1);
            let wv_d1 = (jv0, sv - 1);
            // The cell centroid — a translation CHOICE (any finite c
            // is sound), computed on f64 structure, fixed order.
            let mut csum = [0.0f64; 3];
            let mut count = 0.0f64;
            for i in iu0..=su {
                for j in jv0..=sv {
                    let p = r.control()[i * nv + j];
                    csum[0] += p.x;
                    csum[1] += p.y;
                    csum[2] += p.z;
                    count += 1.0;
                }
            }
            let c = [csum[0] / count, csum[1] / count, csum[2] / count];
            // The cell's weight range — the divisor (module docs: for
            // a SUP bound the divisor is w_min; the interval division
            // by [w_lo, w_hi] takes exactly num_sup/w_lo for the
            // nonnegative numerators below, outward-rounded, and
            // poisons if positivity was never proven).
            let mut w_cell: Option<RingInterval> = None;
            for i in iu0..=su {
                for j in jv0..=sv {
                    let wv = w_grid[i][j];
                    w_cell = Some(match w_cell {
                        None => wv,
                        Some(h) => RingInterval::hull(h, wv),
                    });
                }
            }
            let w_cell = w_cell.unwrap_or_else(RingInterval::poison);
            let zero = RingInterval::zero();
            // Weight-net magnitude sups on the cell.
            let w10 = mag_iv(window_tilde_hull(
                &w_nets.d10,
                &w_nets.d10,
                zero,
                wu_d1,
                wv_val,
            ));
            let w01 = mag_iv(window_tilde_hull(
                &w_nets.d01,
                &w_nets.d01,
                zero,
                wu_val,
                wv_d1,
            ));
            let w11 = mag_iv(window_tilde_hull(
                &w_nets.d11,
                &w_nets.d11,
                zero,
                wu_d1,
                wv_d1,
            ));
            let w20 = w_nets.d20.as_ref().map_or_else(RingInterval::zero, |d| {
                mag_iv(window_tilde_hull(d, d, zero, (iu0, su - 2), wv_val))
            });
            let w02 = w_nets.d02.as_ref().map_or_else(RingInterval::zero, |d| {
                mag_iv(window_tilde_hull(d, d, zero, wu_val, (jv0, sv - 2)))
            });
            let (mut cuu, mut cuv, mut cvv) = (
                RingInterval::zero(),
                RingInterval::zero(),
                RingInterval::zero(),
            );
            for (comp, (_, a)) in a_nets.iter().enumerate() {
                let cc = RingInterval::point(c[comp]);
                // sup|S^c − c^c| on the cell: the rational value hull
                // (positive weights ⇒ nonnegative partition of unity
                // over the ACTIVE control points).
                let mut v0h: Option<RingInterval> = None;
                for i in iu0..=su {
                    for j in jv0..=sv {
                        let p = r.control()[i * nv + j];
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
                let v0 = mag_iv(v0h.unwrap_or_else(RingInterval::poison));
                // Recentred homogeneous derivative sups Ã_kl = A_kl −
                // c·w_kl on the cell.
                let a10 = mag_iv(window_tilde_hull(&a.d10, &w_nets.d10, cc, wu_d1, wv_val));
                let a01 = mag_iv(window_tilde_hull(&a.d01, &w_nets.d01, cc, wu_val, wv_d1));
                let a11 = mag_iv(window_tilde_hull(&a.d11, &w_nets.d11, cc, wu_d1, wv_d1));
                let a20 = match (a.d20.as_ref(), w_nets.d20.as_ref()) {
                    (Some(ad), Some(wd)) => {
                        mag_iv(window_tilde_hull(ad, wd, cc, (iu0, su - 2), wv_val))
                    }
                    _ => RingInterval::zero(),
                };
                let a02 = match (a.d02.as_ref(), w_nets.d02.as_ref()) {
                    (Some(ad), Some(wd)) => {
                        mag_iv(window_tilde_hull(ad, wd, cc, wu_val, (jv0, sv - 2)))
                    }
                    _ => RingInterval::zero(),
                };
                // The quotient-rule recurrences (module docs), each
                // division by the cell weight range.
                let s1u = (a10 + v0 * w10) / w_cell;
                let s1v = (a01 + v0 * w01) / w_cell;
                let suu = (a20 + two * s1u * w10 + v0 * w20) / w_cell;
                let svv = (a02 + two * s1v * w01 + v0 * w02) / w_cell;
                let suv = (a11 + s1u * w01 + s1v * w10 + v0 * w11) / w_cell;
                cuu = cuu + suu.sqr();
                cuv = cuv + suv.sqr();
                cvv = cvv + svv.sqr();
            }
            acc(&mut sq_uu, cuu);
            acc(&mut sq_uv, cuv);
            acc(&mut sq_vv, cvv);
        }
    }
    let m = |sq: Option<RingInterval>| sq.map_or(f64::NAN, |s| s.hi().sqrt().next_up());
    Ok(NurbsFaceBound {
        muu: m(sq_uu),
        muv: m(sq_uv),
        mvv: m(sq_vv),
    })
}

/// The C¹ gate per direction (module docs): degree 0 refuses; degree 1
/// must be single-span (an interior knot is a C⁰ crease); degree ≥ 2
/// needs interior multiplicities ≤ p − 1 — the chord pass's carrier
/// gate, verbatim on the surface's knot vectors.
fn check_direction(kv: &KnotVector, fk: FaceKey) -> Result<(), TessellateError> {
    let p = kv.degree();
    if p == 0 {
        return Err(TessellateError::UnsupportedNurbsFace {
            face: fk,
            note: "degree-0 NURBS direction (a degenerate face description)",
        });
    }
    let interior = &kv.knots()[p + 1..kv.knots().len() - p - 1];
    if p == 1 {
        if interior.is_empty() {
            return Ok(());
        }
        return Err(TessellateError::UnsupportedNurbsFace {
            face: fk,
            note: "degree-1 NURBS direction with interior knots (a C⁰ crease) — \
                   the interpolation Taylor bound needs C¹; split the face at \
                   the crease",
        });
    }
    let mut i = 0;
    while i < interior.len() {
        let mut j = i + 1;
        while j < interior.len() && interior[j] == interior[i] {
            j += 1;
        }
        if j - i > p - 1 {
            return Err(TessellateError::UnsupportedNurbsFace {
                face: fk,
                note: "NURBS direction with a C⁰ crease (interior multiplicity = \
                       degree) — the interpolation Taylor bound needs C¹; split \
                       the face at the crease",
            });
        }
        i = j;
    }
    Ok(())
}

/// Hull of ALL second-derivative coefficients along one direction:
/// each `rows[k]` is the coefficient row of one fixed cross-direction
/// index, differenced twice against `kv` (module docs). A degree-1
/// single-span direction answers the exact zero.
fn second_derivative_hull(
    kv: &KnotVector,
    rows: &[Vec<RingInterval>],
    fk: FaceKey,
) -> Result<RingInterval, TessellateError> {
    if kv.degree() < 2 {
        return Ok(RingInterval::zero());
    }
    let kv1 = derived_knots(kv, fk)?;
    let mut acc: Option<RingInterval> = None;
    for row in rows {
        let q1 = derivative_coeffs(kv, row);
        let q2 = derivative_coeffs(&kv1, &q1);
        for q in q2 {
            acc = Some(match acc {
                None => q,
                Some(a) => RingInterval::hull(a, q),
            });
        }
    }
    acc.ok_or(TessellateError::MissingEntity {
        what: "empty NURBS control net",
    })
}

/// Hull of ALL mixed-derivative coefficients: difference once along
/// `kv_u` (per `u_rows` row, i.e. per fixed v index), then once along
/// `kv_v` across the resulting net (module docs — one application per
/// direction, so only degree ≥ 1 is needed on each, which
/// [`check_direction`] guarantees).
fn mixed_derivative_hull(
    kv_u: &KnotVector,
    kv_v: &KnotVector,
    u_rows: &[Vec<RingInterval>],
) -> Result<RingInterval, TessellateError> {
    // q_u[j][i'] : derivative-in-u coefficients at v index j.
    let q_u: Vec<Vec<RingInterval>> = u_rows
        .iter()
        .map(|row| derivative_coeffs(kv_u, row))
        .collect();
    let nu1 = q_u.first().map_or(0, Vec::len);
    let nv = q_u.len();
    if nu1 == 0 || nv == 0 {
        return Err(TessellateError::MissingEntity {
            what: "empty NURBS control net",
        });
    }
    let mut acc: Option<RingInterval> = None;
    // Walked column-wise on purpose (transposing the row-major
    // intermediate net), so `i` indexes across the OUTER Vec.
    #[allow(clippy::needless_range_loop)]
    for i in 0..nu1 {
        let v_row: Vec<RingInterval> = (0..nv).map(|j| q_u[j][i]).collect();
        for q in derivative_coeffs(kv_v, &v_row) {
            acc = Some(match acc {
                None => q,
                Some(a) => RingInterval::hull(a, q),
            });
        }
    }
    acc.ok_or(TessellateError::MissingEntity {
        what: "empty NURBS control net",
    })
}

/// The once-differenced knot vector (drop the outer knot pair, degree
/// − 1) — the chord pass's construction, shared refusal shape.
fn derived_knots(kv: &KnotVector, fk: FaceKey) -> Result<KnotVector, TessellateError> {
    let inner = kv.knots()[1..kv.knots().len() - 1].to_vec();
    KnotVector::clamped(inner, kv.degree() - 1).map_err(|_| TessellateError::UnsupportedNurbsFace {
        face: fk,
        note: "NURBS direction whose derivative knot vector fails to materialise — \
               outside the certified inventory",
    })
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use geom_core::Point3;

    /// A wavy degree-2×3 integral net on [0,1]² (nothing symmetric, so
    /// every second partial is genuinely nonzero).
    fn wavy() -> NurbsSurface<f64> {
        let kv_u = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.4, 1.0, 1.0, 1.0], 2).unwrap();
        let kv_v =
            KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 0.6, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
        let (nu, nv) = (kv_u.control_count(), kv_v.control_count());
        let mut control = Vec::new();
        for i in 0..nu {
            for j in 0..nv {
                let (x, y) = (i as f64 * 0.7, j as f64 * 0.5);
                control.push(Point3::new(x, y, (1.3 * x + 0.9 * y).sin() + 0.3 * x * y));
            }
        }
        let w = vec![1.0; control.len()];
        NurbsSurface::new(kv_u, kv_v, control, w).unwrap()
    }

    /// The hull bound DOMINATES every sampled second partial — the
    /// convexity claim, measured (never the other way round).
    #[test]
    fn hessian_hull_dominates_sampled_second_partials() {
        let s = wavy();
        let b = nurbs_face_bound(&s, FaceKey::default()).expect("covered class");
        assert!(b.muu > 0.0 && b.muv > 0.0 && b.mvv > 0.0);
        let n = 41;
        let (mut wuu, mut wuv, mut wvv) = (0.0f64, 0.0f64, 0.0f64);
        for i in 0..=n {
            for j in 0..=n {
                let jet = s.ders(f64::from(i) / f64::from(n), f64::from(j) / f64::from(n));
                wuu = wuu.max(jet.duu.norm());
                wuv = wuv.max(jet.duv.norm());
                wvv = wvv.max(jet.dvv.norm());
            }
        }
        assert!(
            wuu > 0.0 && wuu <= b.muu,
            "sup|S_uu| {wuu} vs hull {}",
            b.muu
        );
        assert!(
            wuv > 0.0 && wuv <= b.muv,
            "sup|S_uv| {wuv} vs hull {}",
            b.muv
        );
        assert!(
            wvv > 0.0 && wvv <= b.mvv,
            "sup|S_vv| {wvv} vs hull {}",
            b.mvv
        );
    }

    /// The certificate arithmetic: Q/4 with the box extents.
    ///
    /// DEMOTED to a formula FREEZE (MIN-1, the #218 review): this
    /// mirrors the implementation and can only catch accidental
    /// edits, tautologically — the review's planted 0.25 → 0.05 cert
    /// weakening sailed past the aggregate δ+ε pin and died only
    /// here. The GUARD against under-certification is the empirical
    /// per-triangle falsifier (`probe_review::z1`, armed through
    /// `probe_stats::arm`), which kills the same plant on measured
    /// deviations. Kept as a cheap freeze; never cite it as evidence
    /// the bound is honest.
    #[test]
    fn cert_is_the_documented_quarter_q() {
        let b = NurbsFaceBound {
            muu: 2.0,
            muv: 3.0,
            mvv: 5.0,
        };
        let uv = [[0.0, 0.0], [0.1, 0.0], [0.0, 0.2]];
        let q = 2.0 * 0.01 + 2.0 * 3.0 * 0.1 * 0.2 + 5.0 * 0.04;
        assert!((b.cert(uv) - 0.25 * q).abs() < 1e-15);
    }

    /// A planar bilinear quad is flat: every bound collapses to the
    /// ring's outward-rounding dust (subnormal / few-ulp scale, never
    /// claimed as exact zero — conservative is the promised
    /// direction), and both grid steps come out effectively
    /// unconstrained.
    #[test]
    fn planar_bilinear_bounds_collapse() {
        let kv = KnotVector::unit_segment(1);
        let control = vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(1.0, 1.0, 0.0),
        ];
        let s = NurbsSurface::new(kv.clone(), kv, control, vec![1.0; 4]).unwrap();
        let b = nurbs_face_bound(&s, FaceKey::default()).unwrap();
        assert!(b.muu < 1e-100 && b.muv < 1e-12 && b.mvv < 1e-100);
        let (hu, hv) = b.grid_steps(1e-3);
        // Effectively unconstrained: one cell across any real chart
        // rectangle (spans are O(1)).
        assert!(hu > 1e3 && hv > 1e3);
    }

    /// A TWISTED bilinear quad has S_uv = ΔΔP exactly; the hull pins
    /// that constant to outward rounding (degree-1 directions
    /// contribute only rounding dust to uu/vv).
    #[test]
    fn twisted_bilinear_mixed_term_is_tight() {
        let kv = KnotVector::unit_segment(1);
        let control = vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(1.0, 1.0, 0.5),
        ];
        let s = NurbsSurface::new(kv.clone(), kv, control, vec![1.0; 4]).unwrap();
        let b = nurbs_face_bound(&s, FaceKey::default()).unwrap();
        assert!(b.muu < 1e-100 && b.mvv < 1e-100);
        // S_uv = P11 − P10 − P01 + P00 = (0, 0, 0.5); the hull only
        // ever widens.
        assert!(b.muv >= 0.5 && b.muv < 0.5 + 1e-12);
    }

    /// REVIEW Z2: degree-0 direction refuses typed (if constructible).
    #[test]
    fn probe_degree_zero_refuses_typed() {
        let Ok(kv_u) = KnotVector::clamped(vec![0.0, 1.0], 0) else {
            // Degree 0 cannot even be described — the gate is upstream.
            return;
        };
        let kv_v = KnotVector::unit_segment(1);
        let n = kv_u.control_count() * kv_v.control_count();
        let control = vec![Point3::new(0.0, 0.0, 0.0); n];
        let s = NurbsSurface::new(kv_u, kv_v, control, vec![1.0; n]).unwrap();
        match nurbs_face_bound(&s, FaceKey::default()) {
            Err(TessellateError::UnsupportedNurbsFace { note, .. }) => {
                assert!(note.contains("degree-0"));
            }
            other => panic!("expected typed refusal, got {other:?}"),
        }
    }

    /// REVIEW Z2 boundary: interior multiplicity EXACTLY p-1 is C^1 —
    /// the covered side of the gate — and the hull still dominates a
    /// dense sample straddling the knot (S_uu jumps there).
    #[test]
    fn probe_multiplicity_p_minus_one_is_covered_and_dominated() {
        for (p_deg, mult) in [(2usize, 1usize), (3, 2)] {
            let mut knots = vec![0.0; p_deg + 1];
            knots.extend(std::iter::repeat_n(0.5, mult));
            knots.extend(vec![1.0; p_deg + 1]);
            let kv_u = KnotVector::clamped(knots, p_deg).unwrap();
            let kv_v = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
            let (nu, nv) = (kv_u.control_count(), kv_v.control_count());
            let mut control = Vec::new();
            for i in 0..nu {
                for j in 0..nv {
                    let (x, y) = (i as f64 * 0.6, j as f64 * 0.8);
                    control.push(Point3::new(x, y, (2.1 * x - 1.3 * y).cos() + 0.4 * x * y));
                }
            }
            let s = NurbsSurface::new(kv_u, kv_v, control, vec![1.0; nu * nv]).unwrap();
            let b = nurbs_face_bound(&s, FaceKey::default())
                .expect("multiplicity p-1 is the covered side");
            let n = 160;
            let (mut wuu, mut wuv, mut wvv) = (0.0f64, 0.0f64, 0.0f64);
            for i in 0..=n {
                for j in 0..=n {
                    let jet = s.ders(f64::from(i) / f64::from(n), f64::from(j) / f64::from(n));
                    wuu = wuu.max(jet.duu.norm());
                    wuv = wuv.max(jet.duv.norm());
                    wvv = wvv.max(jet.dvv.norm());
                }
            }
            assert!(
                wuu <= b.muu && wuv <= b.muv && wvv <= b.mvv,
                "deg {p_deg} mult {mult}: sampled ({wuu},{wuv},{wvv}) vs hull ({},{},{})",
                b.muu,
                b.muv,
                b.mvv
            );
        }
    }

    /// CONSCIOUS FLIP (M8-5): `rational_face_refuses_typed` re-derived
    /// as the positive row. The refusal's premise — "a rational second
    /// derivative is not a control-hull convexity fact" — was answered
    /// by the quotient-rule assembly over the HOMOGENEOUS nets (module
    /// docs), so the pin is now the bound's honesty: on adversarial
    /// rational patches, dense-sampled true second partials are
    /// DOMINATED by the certified sups, and the sups are real (> 0),
    /// never a fabricated zero.
    #[test]
    fn rational_face_bound_dominates_sampled_hessian() {
        // (a) The exact quarter cylinder: rational quadratic arc × line
        // (the arc-walled loft class M8-2 made buildable) — muu real,
        // mvv parameter-flat.
        let arc = quarter_cylinder();
        // (b) A wavy rational 2×3 patch with steep alternating weights
        // (nothing symmetric; every second partial nonzero, and the
        // weight ramps are what the cross terms have to survive).
        let wild = wavy_rational();
        for (name, s, all_positive) in [("quarter_cylinder", arc, false), ("wavy", wild, true)] {
            let b = nurbs_face_bound(&s, FaceKey::default()).expect("rational is covered now");
            let n = 80;
            let (mut wuu, mut wuv, mut wvv) = (0.0f64, 0.0f64, 0.0f64);
            for i in 0..=n {
                for j in 0..=n {
                    let jet = s.ders(f64::from(i) / f64::from(n), f64::from(j) / f64::from(n));
                    wuu = wuu.max(jet.duu.norm());
                    wuv = wuv.max(jet.duv.norm());
                    wvv = wvv.max(jet.dvv.norm());
                }
            }
            assert!(
                wuu <= b.muu && wuv <= b.muv && wvv <= b.mvv,
                "{name}: sampled ({wuu},{wuv},{wvv}) escapes the certified \
                 ({},{},{})",
                b.muu,
                b.muv,
                b.mvv
            );
            assert!(
                b.muu > 0.0,
                "{name}: muu must be real, not a fabricated zero"
            );
            if all_positive {
                assert!(b.muv > 0.0 && b.mvv > 0.0, "{name}: all partials real");
            }
        }
    }

    /// The exact unit quarter cylinder: rational quadratic arc
    /// (weights `[1, √2/2, 1]`) × unit line in z.
    fn quarter_cylinder() -> NurbsSurface<f64> {
        let kv_u = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
        let kv_v = KnotVector::unit_segment(1);
        let w = core::f64::consts::FRAC_1_SQRT_2;
        let arc = [(1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
        let mut control = Vec::new();
        for (x, y) in arc {
            for z in [0.0, 1.0] {
                control.push(Point3::new(x, y, z));
            }
        }
        let weights = vec![1.0, 1.0, w, w, 1.0, 1.0];
        NurbsSurface::new(kv_u, kv_v, control, weights).unwrap()
    }

    /// The `wavy` net with steep alternating weights.
    fn wavy_rational() -> NurbsSurface<f64> {
        let kv_u = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.4, 1.0, 1.0, 1.0], 2).unwrap();
        let kv_v =
            KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 0.6, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
        let (nu, nv) = (kv_u.control_count(), kv_v.control_count());
        let mut control = Vec::new();
        let mut weights = Vec::new();
        for i in 0..nu {
            for j in 0..nv {
                let (x, y) = (i as f64 * 0.7, j as f64 * 0.5);
                control.push(Point3::new(x, y, (1.3 * x + 0.9 * y).sin() + 0.3 * x * y));
                weights.push(match (i + 2 * j) % 4 {
                    0 => 0.4,
                    1 => 2.5,
                    2 => 1.0,
                    _ => 3.0,
                });
            }
        }
        NurbsSurface::new(kv_u, kv_v, control, weights).unwrap()
    }

    /// The rational wall of the REAL construction: the M8-5 pie loft
    /// (single-span bulge-0.4 arc profile, straight loft — the class
    /// M8-2's rational span meter made buildable), extracted from the
    /// assembled body.
    fn pie_wall() -> NurbsSurface<f64> {
        use geom_core::{Affine3, Point2, Vec3};
        let v = |x: f64, y: f64, bulge: f64| sweep::ProfileVertex {
            pos: Point2::new(x, y),
            bulge,
        };
        let lp =
            sweep::ProfileLoop::new(vec![v(1.0, 0.0, 0.4), v(0.0, 1.0, 0.0), v(0.0, 0.0, 0.0)]);
        let sections = vec![vec![lp.clone()], vec![lp]];
        let places: Vec<Affine3<f64>> = [0.0, 1.0]
            .iter()
            .map(|z| Affine3::translation(Vec3::new(0.0, 0.0, *z)))
            .collect();
        let body = sweep::loft_body::<f64>(&sections, &places, 1)
            .expect("the rational pie lofts")
            .body;
        for (_, face) in body.faces() {
            if let Some(geom_surfaces::Surface::Nurbs(p)) = body.get_surface(face.surface) {
                if p.weights().iter().any(|w| *w != 1.0) {
                    return (**p).clone();
                }
            }
        }
        panic!("the pie loft minted no rational wall — the fixture stopped exercising M8-5");
    }

    /// The #218 per-triangle falsifier, pointed at the RATIONAL arm
    /// (M8-5): the z1 driver's 12-deep barycentric lattice (91
    /// samples/triangle), run on the real pie wall and the two
    /// adversarial rational patches, over the certificate's own
    /// `grid_steps` triangulation at two deltas. Every triangle's
    /// sampled `|S − Π|` must be dominated by `cert(uv)` (plus f64
    /// evaluation dust — the crate's documented ε-side slack, taken
    /// here as 1e-12 on O(1) fixtures), and the worst ratio is
    /// printed as the PR's evidence. Full-body z1 coverage of this
    /// lane awaits the topo pcurve frontier
    /// (`z1r_rational_wall_full_body_frontier_pin`).
    #[test]
    fn rational_z1_lattice_falsification() {
        // Per-fixture deltas: the steep-weight `wavy_rational`'s bound
        // is conservative (documented — a bound, not an estimate), so
        // its fine-δ grids run to millions of triangles; the lattice's
        // falsification power is PER TRIANGLE, not δ-dependent, so it
        // takes coarser deltas and the two real-construction fixtures
        // take the z1 driver's.
        for (name, s, deltas) in [
            ("pie_wall", pie_wall(), [3e-2, 6e-3]),
            ("quarter_cylinder", quarter_cylinder(), [3e-2, 6e-3]),
            ("wavy_rational", wavy_rational(), [1.0, 2e-1]),
        ] {
            let b = nurbs_face_bound(&s, FaceKey::default()).expect("covered");
            let (u0, u1) = s.knots_u().domain();
            let (v0, v1) = s.knots_v().domain();
            for delta in deltas {
                let delta_s = delta / 2.0;
                let (hu, hv) = b.grid_steps(delta_s);
                let cells = |span: f64, h: f64| -> usize {
                    let raw = (span / h).ceil();
                    if raw.is_finite() && raw >= 1.0 {
                        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                        {
                            raw as usize
                        }
                    } else {
                        1
                    }
                };
                let (nu, nv) = (cells(u1 - u0, hu), cells(v1 - v0, hv));
                let at = |i: usize, j: usize| -> [f64; 2] {
                    #[allow(clippy::cast_precision_loss)]
                    [
                        u0 + (u1 - u0) * (i as f64 / nu as f64),
                        v0 + (v1 - v0) * (j as f64 / nv as f64),
                    ]
                };
                let (mut worst_ratio, mut tris) = (0.0f64, 0usize);
                for i in 0..nu {
                    for j in 0..nv {
                        let (a, bb, c, d) =
                            (at(i, j), at(i + 1, j), at(i + 1, j + 1), at(i, j + 1));
                        for uv in [[a, bb, c], [a, c, d]] {
                            tris += 1;
                            let cert = b.cert(uv);
                            let p: Vec<geom_core::Point3<f64>> =
                                uv.iter().map(|w| s.eval(w[0], w[1])).collect();
                            let m = 12usize;
                            for ba in 0..=m {
                                for bbn in 0..=(m - ba) {
                                    #[allow(clippy::cast_precision_loss)]
                                    let (b0, b1) = (ba as f64 / m as f64, bbn as f64 / m as f64);
                                    let b2 = 1.0 - b0 - b1;
                                    let (u, v) = (
                                        b0 * uv[0][0] + b1 * uv[1][0] + b2 * uv[2][0],
                                        b0 * uv[0][1] + b1 * uv[1][1] + b2 * uv[2][1],
                                    );
                                    let sv = s.eval(u, v);
                                    let pi = Point3::new(
                                        b0 * p[0].x + b1 * p[1].x + b2 * p[2].x,
                                        b0 * p[0].y + b1 * p[1].y + b2 * p[2].y,
                                        b0 * p[0].z + b1 * p[1].z + b2 * p[2].z,
                                    );
                                    let dev = ((sv.x - pi.x).powi(2)
                                        + (sv.y - pi.y).powi(2)
                                        + (sv.z - pi.z).powi(2))
                                    .sqrt();
                                    assert!(
                                        dev <= cert + 1e-12,
                                        "{name}: per-triangle violation |S-Pi| {dev} > cert \
                                         {cert} at uv=({u},{v}) tri {uv:?}"
                                    );
                                    if cert > 0.0 {
                                        worst_ratio = worst_ratio.max(dev / cert);
                                    }
                                }
                            }
                        }
                    }
                }
                println!(
                    "{name} delta={delta:.0e}: grid {nu}x{nv} tris={tris} max d/cert={worst_ratio:.4}"
                );
                assert!(
                    worst_ratio <= 1.0,
                    "{name}: a triangle's samples exceeded its certificate"
                );
            }
        }
    }

    /// The POISON row the flip keeps: an ILLEGAL rational (non-positive
    /// or non-finite weight) cannot even be described —
    /// `NurbsSurface::new` refuses at the door, which is why
    /// `rational_face_bound`'s own licence check is a defensive
    /// backstop rather than a reachable lane.
    #[test]
    fn illegal_rational_weight_refuses_at_the_door() {
        let kv = KnotVector::unit_segment(1);
        let control = vec![Point3::new(0.0, 0.0, 0.0); 4];
        for bad in [0.0, -1.0, f64::NAN] {
            assert!(
                NurbsSurface::new(
                    kv.clone(),
                    kv.clone(),
                    control.clone(),
                    vec![1.0, bad, 1.0, 1.0]
                )
                .is_err(),
                "weight {bad} must refuse at construction"
            );
        }
    }

    /// The rational dust re-derivation (the
    /// `planar_bilinear_bounds_collapse` class, rational arm): a flat
    /// bilinear patch with UNIFORM weight `1/2` is bitwise rational but
    /// geometrically the same flat quad (constant weights cancel in
    /// ℝ), so every true second partial is zero. What the rational
    /// assembly answers is DUST scaled by the divisor `w_min = 1/2` —
    /// re-derived, not blind-reused from the integral thresholds:
    ///
    /// - `muu`/`mvv`: the `Ã_dd`/`w_dd` terms are exact zeros (degree
    ///   1) and the `w_d` hulls of a CONSTANT weight column stay
    ///   outward-rounded zeros through refinement (convex combinations
    ///   of `0.5` are exact), so the cross terms are subnormal dust
    ///   over `w_min` — the integral 1e-100 class survives (measured
    ///   ~5e-166).
    /// - `muv`: the refined homogeneous net carries knot-INSERTION
    ///   rounding (~ulp of O(1) coefficients), and the mixed
    ///   differencing scales it by `p/Δu` at the 16-fold-refined span
    ///   width — measured ~2e-13, so the integral arm's 1e-12 pin
    ///   would be blind reuse; pinned an order above at 1e-11.
    #[test]
    fn rational_uniform_weight_bilinear_dust() {
        let kv = KnotVector::unit_segment(1);
        let control = vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(1.0, 1.0, 0.0),
        ];
        let s = NurbsSurface::new(kv.clone(), kv, control, vec![0.5; 4]).unwrap();
        let b = nurbs_face_bound(&s, FaceKey::default()).unwrap();
        assert!(
            b.muu < 1e-100 && b.muv < 1e-11 && b.mvv < 1e-100,
            "rational dust escaped its derivation: ({}, {}, {})",
            b.muu,
            b.muv,
            b.mvv
        );
        let (hu, hv) = b.grid_steps(1e-3);
        assert!(hu > 1e3 && hv > 1e3, "flat stays effectively unconstrained");
    }

    /// A C⁰ crease (interior multiplicity = degree) refuses typed —
    /// the Taylor remainder needs C¹.
    #[test]
    fn c0_crease_refuses_typed() {
        let kv_u = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0], 2).unwrap();
        let kv_v = KnotVector::unit_segment(1);
        let n = kv_u.control_count() * kv_v.control_count();
        let control = vec![Point3::new(0.0, 0.0, 0.0); n];
        let s = NurbsSurface::new(kv_u, kv_v, control, vec![1.0; n]).unwrap();
        match nurbs_face_bound(&s, FaceKey::default()) {
            Err(TessellateError::UnsupportedNurbsFace { note, .. }) => {
                assert!(note.contains("crease"));
            }
            other => panic!("expected UnsupportedNurbsFace, got {other:?}"),
        }
    }

    /// A degree-1 direction with interior knots is a crease too.
    #[test]
    fn degree_one_interior_knot_refuses_typed() {
        let kv_u = KnotVector::clamped(vec![0.0, 0.0, 0.5, 1.0, 1.0], 1).unwrap();
        let kv_v = KnotVector::unit_segment(1);
        let n = kv_u.control_count() * kv_v.control_count();
        let control = vec![Point3::new(0.0, 0.0, 0.0); n];
        let s = NurbsSurface::new(kv_u, kv_v, control, vec![1.0; n]).unwrap();
        match nurbs_face_bound(&s, FaceKey::default()) {
            Err(TessellateError::UnsupportedNurbsFace { note, .. }) => {
                assert!(note.contains("crease"));
            }
            other => panic!("expected UnsupportedNurbsFace, got {other:?}"),
        }
    }
}
