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
//! Covered: described, non-rational (all weights bitwise 1), per
//! direction either degree ≥ 2 with interior multiplicities ≤ p − 1
//! (C¹) or degree 1 single-span (exactly the loft/sweep wall class:
//! degree 1×2 and 1×3). Refused typed
//! ([`TessellateError::UnsupportedNurbsFace`]): rational surfaces (a
//! rational second derivative is not a hull convexity fact — the same
//! deliberate absence as `hull`'s missing rational derivative path),
//! C⁰ creases (the Taylor remainder needs C¹), degree-0 directions,
//! and poisoned/non-finite hulls. The placeholder refuses
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
/// # Errors
///
/// [`TessellateError::UnsupportedNurbsFace`] — rational, C⁰-creased,
/// degree-0, or a poisoned/unbounded hull. The PLACEHOLDER is the
/// caller's check (it refuses `UnsupportedSurface`, the mvfs state's
/// historical variant).
pub(crate) fn nurbs_face_bound(
    n: &NurbsSurface<f64>,
    fk: FaceKey,
) -> Result<NurbsFaceBound, TessellateError> {
    if n.weights().iter().any(|w| *w != 1.0) {
        return Err(TessellateError::UnsupportedNurbsFace {
            face: fk,
            note: "rational NURBS face — a rational second derivative is not a \
                   control-hull convexity fact (the hull module's deliberate \
                   absence), so no certified Hessian bound exists here; rational \
                   walls already refuse tier 3 at the volume door",
        });
    }
    check_direction(n.knots_u(), fk)?;
    check_direction(n.knots_v(), fk)?;
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
    let bound = NurbsFaceBound {
        muu: m(sq_uu),
        muv: m(sq_uv),
        mvv: m(sq_vv),
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

    /// Rational faces refuse typed — a rational second derivative is
    /// not a hull convexity fact.
    #[test]
    fn rational_face_refuses_typed() {
        let kv = KnotVector::unit_segment(1);
        let control = vec![Point3::new(0.0, 0.0, 0.0); 4];
        let s = NurbsSurface::new(kv.clone(), kv, control, vec![1.0, 2.0, 1.0, 1.0]).unwrap();
        match nurbs_face_bound(&s, FaceKey::default()) {
            Err(TessellateError::UnsupportedNurbsFace { note, .. }) => {
                assert!(note.contains("rational"));
            }
            other => panic!("expected UnsupportedNurbsFace, got {other:?}"),
        }
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
