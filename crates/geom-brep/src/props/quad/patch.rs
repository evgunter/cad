//! **The NURBS-patch lane** over a whole UV rectangle
//! ([`nurbs_patch_face_rounds`]): the polynomial patch's exact per-span
//! Newton–Cotes rule ([`patch_flux_exact`]) with the composite rounds
//! behind it. A rational patch routes to [`super::rational`].

use geom_core::interval::certification::Certification;
use geom_core::interval::{Interval, cross, dot};
use geom_core::spline::KnotVector;
use geom_core::{Band, Decide};

use super::area::{
    AREA_CELLS_PER_SPAN, AreaCell, PERIMETER_CHORDS_PER_SPAN, area_midpoint_taylor,
    boundary_chord_perimeter_lo, cut_window, debug_assert_area_gauge,
};
use super::cells::{
    HULL_BLOCKS_PER_SPAN, advance_block, cell_count, cell_midpoint, knot_aligned_cuts,
    remainder_cell_u, remainder_cell_v, round_cuts, span_cell_count,
};
use super::net::{
    Collapse, PatchGrid, PatchNet, PatchRect, clipped_spans, grid_vec, newton_cotes_weights,
    norm_hi, normal_partials, widen,
};
use super::rational::rational_patch_face;
use super::run::{
    Accept, FaceCutBounds, LaneEnd, LaneRun, LaneStop, RoundWindow, Run, face_extent,
    span_work_wall,
};
use crate::props::PropsError;
use crate::props::bracket::pt;

/// Weight of one cell of the integral patch lane's composite.
const W_INTEGRAL_COMPOSITE: usize = 32;

/// **The exact per-span lane** (the primary flux path): on each
/// knot-span rectangle the integrand `f = S·(S_u×S_v)` is one
/// polynomial of degree ≤ 3p−1 per direction, so the tensor closed
/// Newton–Cotes rule of order `3p` integrates it EXACTLY — the whole
/// enclosure width is the nodes' and weights' outward rounding. `None`
/// when the rule order leaves the supported window (degree > 4 per
/// direction), in which case the caller runs the composite rounds.
pub(super) fn patch_flux_exact(
    s: &PatchGrid,
    su: Option<&PatchGrid>,
    sv: Option<&PatchGrid>,
    kv_u: &KnotVector,
    kv_v: &KnotVector,
    rect: (f64, f64, f64, f64),
) -> Option<Interval> {
    let (u0, u1, v0, v1) = rect;
    let (mu, mv) = (3 * kv_u.degree(), 3 * kv_v.degree());
    let (wu, wv) = (newton_cotes_weights(mu)?, newton_cotes_weights(mv)?);
    let mut flux = Interval::zero();
    for &(au, bu, mid_u) in &clipped_spans(kv_u, u0, u1) {
        let su_scale = pt(bu) - pt(au);
        // Node enclosures for this span, u direction.
        #[allow(clippy::cast_precision_loss)]
        let nodes_u: Vec<Interval> = (0..=mu)
            .map(|j| pt(au) + su_scale * (pt(j as f64) / pt(mu as f64)))
            .collect();
        for &(av, bv, mid_v) in &clipped_spans(kv_v, v0, v1) {
            let sv_scale = pt(bv) - pt(av);
            #[allow(clippy::cast_precision_loss)]
            let nodes_v: Vec<Interval> = (0..=mv)
                .map(|j| pt(av) + sv_scale * (pt(j as f64) / pt(mv as f64)))
                .collect();
            let scale = su_scale * sv_scale;
            let mut acc = Interval::zero();
            for (nu_i, wu_i) in nodes_u.iter().zip(&wu) {
                let cu = Collapse::AtSpan {
                    mid: mid_u,
                    t: nu_i,
                };
                let mut row = Interval::zero();
                for (nv_j, wv_j) in nodes_v.iter().zip(&wv) {
                    let cv = Collapse::AtSpan {
                        mid: mid_v,
                        t: nv_j,
                    };
                    let fm = dot(
                        &s.vec(cu, cv),
                        &cross(&grid_vec(su, cu, cv), &grid_vec(sv, cu, cv)),
                    );
                    row = row + *wv_j * fm;
                }
                acc = acc + *wu_i * row;
            }
            flux = flux + scale * acc;
        }
    }
    Some(flux)
}

/// The work of the composite arm's round `round` on `rect`: its cells
/// ([`round_cuts`]) at [`W_INTEGRAL_COMPOSITE`] each.
pub(super) fn composite_round_cost(
    rect: PatchRect,
    knots: (&[f64], &[f64]),
    round: usize,
) -> usize {
    W_INTEGRAL_COMPOSITE * cell_count(&round_cuts(rect, knots, false, round))
}

/// **Certified NURBS-patch contributions** (M6-3 Leg C): the
/// chart-normal volume flux `∫∫ S·(S_u×S_v) du dv` and the patch area
/// `∫∫ |S_u×S_v| du dv` of a patch over the exact UV
/// rectangle `rect = (u0, u1, v0, v1)` — the loft/sweep wall case,
/// where the face's stored iso-line pcurves pin the trim region to a
/// rectangle exactly.
///
/// The flux rides the 2-D midpoint composite with hull-bounded
/// second-derivative remainders per axis (module docs' rule, iterated:
/// `∫∫_cell f ∈ A·f(m) + hull(f_uu)·h_u³h_v/24 + hull(f_vv)·h_u h_v³/24`,
/// no cross term — the two 1-D remainders compose), with
///
/// ```text
/// f_uu = S_u·(S_uu×S_v) + S·(S_uuu×S_v) + 2·S·(S_uu×S_uv) + S·(S_u×S_uuv)
/// f_vv = S_v·(S_u×S_vv) + S·(S_uvv×S_v) + 2·S·(S_uv×S_vv) + S·(S_u×S_vvv)
/// ```
///
/// (the `S_u·(S_u×S_uv)`-shaped triples vanish identically). Cells
/// are cut ON the interior knots ([`knot_aligned_cuts`](super::cells::knot_aligned_cuts)), so each one
/// lies inside a single smooth piece and the Taylor remainder is the
/// whole error there. The area rides the
/// shared [`area_midpoint_taylor`] rule (midpoint + Lipschitz pad on
/// the same knot-aligned cells) — sound at every
/// resolution and O(h), because the +V gate meters `V/A` and needs an
/// honest denominator. A patch whose weights are not one constant
/// ([`constant_weight`](geom_core::spline::constant_weight)) routes to [`rational_patch_face`] (M8-3), which certifies the same
/// two numbers through the quotient statement.
///
/// Honesty pads (both directions accounted): `boundary_defect` is the
/// caller's `Σ (metric edge length)·(pcurve envelope)` bound on the
/// metric area between the true trim boundary and the rectangle; it
/// widens the area directly and the flux through the patch's position
/// magnitude bound. Exactly zero for minted (exact-in-family) iso
/// pcurves.
///
/// The flux is signed by the CHART normal `S_u×S_v`; the caller owns
/// the loop-winding sign (S10: winding-derived end to end).
///
/// # Errors
///
/// [`PropsError`] — `accept`'s own refusal, an escalated funnel
/// decision, a degenerate face, or the rational lane's own refusals.
pub fn nurbs_patch_face<T: Decide>(
    net: &PatchNet<'_>,
    rect: (f64, f64, f64, f64),
    perimeter: f64,
    boundary_defect: f64,
    band: Band,
    accept: &mut Accept<'_>,
) -> Result<LaneRun, PropsError> {
    nurbs_patch_face_rounds::<T>(
        net,
        rect,
        perimeter,
        boundary_defect,
        band,
        RoundWindow::SCHEDULE,
        accept,
    )
}

/// [`nurbs_patch_face`] over a [`RoundWindow`] — the same schedule (and
/// the same rational lane behind it), entered and left where the window
/// says.
///
/// The exact per-span arm has no rounds at all: it answers once, at
/// its floor ([`LaneStop::Floor`]), and a window can neither start
/// after it nor ask it for a tighter enclosure.
///
/// `pub` for the same reason [`cylinder_cut_face_rounds`](super::cylinder::cylinder_cut_face_rounds) is: its
/// consumer is `topo`'s `props::quad_lane`, across the crate
/// boundary.
///
/// # Errors
///
/// As [`nurbs_patch_face`].
#[allow(clippy::too_many_lines)] // one engine, kept whole like the cylinder lane
pub fn nurbs_patch_face_rounds<T: Decide>(
    net: &PatchNet<'_>,
    rect: (f64, f64, f64, f64),
    perimeter: f64,
    boundary_defect: f64,
    band: Band,
    window: RoundWindow,
    accept: &mut Accept<'_>,
) -> Result<LaneRun, PropsError> {
    let rect = PatchRect::new(rect)?;
    let (u0, u1, v0, v1) = rect.corners();
    // M8-3: the rational patch is the SAME integrand over a polynomial
    // cube (`f = N/w³`), so it takes its own lane rather than refusing.
    if net.is_rational() {
        return rational_patch_face::<T>(
            net,
            rect,
            perimeter,
            boundary_defect,
            band,
            window,
            accept,
        );
    }
    // The derivative grids, built once (mixed partials commute on
    // spline nets exactly).
    let (kv_u, kv_v) = (net.knots_u(), net.knots_v());
    let s = PatchGrid::base(&net.positions());
    let su = s.deriv_u();
    let sv = s.deriv_v();
    let suu = su.as_ref().and_then(PatchGrid::deriv_u);
    let suv = su.as_ref().and_then(PatchGrid::deriv_v);
    let svv = sv.as_ref().and_then(PatchGrid::deriv_v);
    let suuu = suu.as_ref().and_then(PatchGrid::deriv_u);
    let suuv = suu.as_ref().and_then(PatchGrid::deriv_v);
    let suvv = suv.as_ref().and_then(PatchGrid::deriv_v);
    let svvv = svv.as_ref().and_then(PatchGrid::deriv_v);
    let (knots_u, knots_v) = (kv_u.knots(), kv_v.knots());
    // Position magnitude bound over the rectangle (the flux pad's
    // lever): a 1-norm over-bound of the 2-norm suffices — only an
    // upper bound is consumed.
    let s_hull = s.vec(Collapse::Over(rect.u), Collapse::Over(rect.v));
    let p_bound = s_hull[0].mag() + s_hull[1].mag() + s_hull[2].mag();

    // The second-derivative hulls, one per hull block ([`knot_aligned_cuts`]
    // at [`HULL_BLOCKS_PER_SPAN`]), computed once: a block's hull
    // contains every cell's inside it, so the per-cell remainder
    // `hull(f_uu)·h³/24` may use it soundly, and a cell reads three thin
    // evals rather than ten grid hulls. It bounds the remainder on every
    // cell because the cells are cut on the interior knots, so each one
    // lies inside a single smooth piece and the Taylor remainder is the
    // whole error there.
    let edges_u = knot_aligned_cuts(u0, u1, HULL_BLOCKS_PER_SPAN, knots_u);
    let edges_v = knot_aligned_cuts(v0, v1, HULL_BLOCKS_PER_SPAN, knots_v);
    let nbv = edges_v.len() - 1;
    let block = |bu: usize, bv: usize| -> (Interval, Interval) {
        let over = (
            Collapse::Over(cut_window(edges_u[bu], edges_u[bu + 1])),
            Collapse::Over(cut_window(edges_v[bv], edges_v[bv + 1])),
        );
        let g = |grid: Option<&PatchGrid>| grid_vec(grid, over.0, over.1);
        let g_s = s.vec(over.0, over.1);
        let (g_su, g_sv) = (g(su.as_ref()), g(sv.as_ref()));
        let (g_suu, g_suv, g_svv) = (g(suu.as_ref()), g(suv.as_ref()), g(svv.as_ref()));
        let f_uu = dot(&g_su, &cross(&g_suu, &g_sv))
            + dot(&g_s, &cross(&g(suuu.as_ref()), &g_sv))
            + pt(2.0) * dot(&g_s, &cross(&g_suu, &g_suv))
            + dot(&g_s, &cross(&g_su, &g(suuv.as_ref())));
        let f_vv = dot(&g_sv, &cross(&g_su, &g_svv))
            + dot(&g_s, &cross(&g(suvv.as_ref()), &g_sv))
            + pt(2.0) * dot(&g_s, &cross(&g_suv, &g_svv))
            + dot(&g_s, &cross(&g_su, &g(svvv.as_ref())));
        (f_uu, f_vv)
    };

    // AREA: the shared [`area_midpoint_taylor`] rule (#313's integral
    // half) with this lane's polynomial integrand `g = |S_u×S_v|` —
    // the rational lane's rule at `w ≡ 1`, where the quotient pad
    // collapses to `|∂_d (S_u×S_v)|` exactly. It replaces the plain
    // per-cell hull rule, which was span-granular and therefore did
    // not tighten at all: on the standard multi-arc form it returned
    // `area.lo() == 0` and the face was refused DegenerateFace.
    let area = area_midpoint_taylor(
        rect,
        AREA_CELLS_PER_SPAN,
        boundary_defect,
        (knots_u, knots_v),
        |b| -> Result<AreaCell, PropsError> {
            let over = (Collapse::Over(b.u), Collapse::Over(b.v));
            let m = (Collapse::At(b.umid), Collapse::At(b.vmid));
            let cm = cross(
                &grid_vec(su.as_ref(), m.0, m.1),
                &grid_vec(sv.as_ref(), m.0, m.1),
            );
            let (h_su, h_sv) = (
                grid_vec(su.as_ref(), over.0, over.1),
                grid_vec(sv.as_ref(), over.0, over.1),
            );
            let (d_u, d_v) = normal_partials(
                &h_su,
                &h_sv,
                &grid_vec(suu.as_ref(), over.0, over.1),
                &grid_vec(suv.as_ref(), over.0, over.1),
                &grid_vec(svv.as_ref(), over.0, over.1),
            );
            Ok(AreaCell {
                g_mid: (cm[0].sqr() + cm[1].sqr() + cm[2].sqr()).sqrt(),
                g_u: norm_hi(d_u),
                g_v: norm_hi(d_v),
                g_hull: {
                    let c = cross(&h_su, &h_sv);
                    (c[0].sqr() + c[1].sqr() + c[2].sqr()).sqrt()
                },
            })
        },
    )?;

    let perimeter_lo = || {
        boundary_chord_perimeter_lo(
            rect.corners(),
            PERIMETER_CHORDS_PER_SPAN,
            (knots_u, knots_v),
            |cu, cv| s.vec(Collapse::At(cu), Collapse::At(cv)),
        )
    };

    // ---- The exact per-span lane first (fn docs): one tensor
    // Newton–Cotes pass whose enclosure width is outward rounding only.
    // The composite rounds below are the fallback for degrees outside
    // the rule window (> 4 per direction). ----
    if let Some(exact) =
        patch_flux_exact(&s, su.as_ref(), sv.as_ref(), kv_u, kv_v, (u0, u1, v0, v1))
    {
        let flux = widen(exact, boundary_defect * p_bound);
        let bounds = FaceCutBounds { flux, area };
        // The exact rule leaves no remainder for a round to remove: its
        // one pass is at its floor.
        let accepted = accept(&bounds, LaneStop::Floor)?;
        if accepted {
            face_extent::<T>(area, perimeter, band)?;
            debug_assert_area_gauge(area, perimeter, &perimeter_lo);
        }
        return Ok(LaneRun {
            bounds,
            round: 0,
            rounds: 0,
            end: if accepted {
                LaneEnd::Accepted(LaneStop::Floor)
            } else {
                LaneEnd::Floor
            },
        });
    }

    // Knot-aligned cells, for the reason [`knot_aligned_cuts`] gives:
    // this lane reaches the composite only where the exact rule cannot
    // run, and a cell holding an interior knot has no rule but the
    // smoothness-free hull — span-granular, and therefore a floor
    // rather than a remainder. One home for the loop and its cost.
    let cuts = |round: usize| round_cuts(rect, (knots_u, knots_v), false, round);
    let cost = |round: usize| composite_round_cost(rect, (knots_u, knots_v), round);
    let mut run = Run::enter_walled(
        window,
        span_work_wall(span_cell_count(rect, (knots_u, knots_v))),
        &cost,
    );
    let blocks: Vec<(Interval, Interval)> = (0..edges_u.len() - 1)
        .flat_map(|bu| (0..nbv).map(move |bv| (bu, bv)))
        .map(|(bu, bv)| block(bu, bv))
        .collect();
    let mut round = run.first();
    loop {
        let mut flux = Interval::zero();
        let mut removable = 0.0_f64;
        let (cuts_u, cuts_v) = cuts(round);
        let mut bu = 0usize;
        for iu in 0..cuts_u.len() - 1 {
            let (c_ulo, c_uhi) = (cuts_u[iu], cuts_u[iu + 1]);
            advance_block(&mut bu, c_ulo, &edges_u);
            let hu = pt(c_uhi) - pt(c_ulo);
            let mut bv = 0usize;
            for iv in 0..cuts_v.len() - 1 {
                let (c_vlo, c_vhi) = (cuts_v[iv], cuts_v[iv + 1]);
                advance_block(&mut bv, c_vlo, &edges_v);
                let (g_f_uu, g_f_vv) = blocks[bu * nbv + bv];
                let hv = pt(c_vhi) - pt(c_vlo);
                let (um, vm) = (cell_midpoint(c_ulo, c_uhi), cell_midpoint(c_vlo, c_vhi));
                let m = (
                    Collapse::AtSpan {
                        mid: um.0,
                        t: &um.1,
                    },
                    Collapse::AtSpan {
                        mid: vm.0,
                        t: &vm.1,
                    },
                );
                let fm = dot(
                    &s.vec(m.0, m.1),
                    &cross(
                        &grid_vec(su.as_ref(), m.0, m.1),
                        &grid_vec(sv.as_ref(), m.0, m.1),
                    ),
                );
                let r_uu = g_f_uu * remainder_cell_u(hu, hv);
                let r_vv = g_f_vv * remainder_cell_v(hu, hv);
                removable += r_uu.width() + r_vv.width();
                flux = flux + hu * hv * fm + r_uu + r_vv;
            }
        }
        let flux = widen(flux, boundary_defect * p_bound);
        let bounds = FaceCutBounds { flux, area };
        let (stop, accepted) = run.ask(round, bounds, removable, accept, || {
            face_extent::<T>(area, perimeter, band)?;
            debug_assert_area_gauge(area, perimeter, &perimeter_lo);
            Ok(())
        })?;
        if let Some(out) = run.exit(round, stop, accepted, bounds) {
            return Ok(out);
        }
        round += 1;
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::super::net::RVec3;
    use super::*;
    use crate::props::quad::test_target::{met, patch, to_target};
    use geom_core::Bounds;
    use geom_core::Tol;

    /// The patch lane on the flat unit square at z = 1: `S = (u, v, 1)`
    /// has `f = S·(S_u×S_v) = 1` and `|S_u×S_v| = 1`, so flux = area
    /// = 1 exactly; then a genuinely warped bilinear patch is checked
    /// against a dense midpoint oracle (containment + tightness).
    #[test]
    fn nurbs_patch_flux_matches_flat_and_warped_oracles() {
        let band = Band::linear(Tol::witness()).unwrap();
        let eps = Tol::witness().get().eps;
        let kv = KnotVector::unit_segment(core::num::NonZeroUsize::MIN);
        let p = |x: f64, y: f64, z: f64| [pt(x), pt(y), pt(z)];
        // Row-major iu·nv+iv, nu = nv = 2: [(u0v0), (u0v1), (u1v0), (u1v1)].
        let flat = [
            p(0.0, 0.0, 1.0),
            p(0.0, 1.0, 1.0),
            p(1.0, 0.0, 1.0),
            p(1.0, 1.0, 1.0),
        ];
        let out = met(nurbs_patch_face::<f64>(
            &patch(&kv, &kv, &flat, &[1.0; 4]),
            (0.0, 1.0, 0.0, 1.0),
            4.0,
            0.0,
            band,
            &mut to_target(eps),
        ))
        .unwrap();
        assert!(out.flux.contains(1.0), "flat flux {:?}", out.flux);
        assert!(out.area.contains(1.0), "flat area {:?}", out.area);
        assert!(out.flux.width() < 1e-9, "flux width {}", out.flux.width());

        // Warped: corners lifted unevenly — a ruled quadric patch.
        let warped = [
            p(0.0, 0.0, 0.5),
            p(-0.2, 1.1, 1.0),
            p(1.3, 0.0, 0.8),
            p(1.0, 1.0, 2.0),
        ];
        let out = met(nurbs_patch_face::<f64>(
            &patch(&kv, &kv, &warped, &[1.0; 4]),
            (0.0, 1.0, 0.0, 1.0),
            6.0,
            0.0,
            band,
            &mut to_target(eps),
        ))
        .unwrap();
        // Dense midpoint oracle on the bilinear closed form.
        let sv = |u: f64, v: f64| -> [f64; 3] {
            core::array::from_fn(|k| {
                let c = [
                    warped[0][k].lo(),
                    warped[1][k].lo(),
                    warped[2][k].lo(),
                    warped[3][k].lo(),
                ];
                (1.0 - u) * ((1.0 - v) * c[0] + v * c[1]) + u * ((1.0 - v) * c[2] + v * c[3])
            })
        };
        let n = 400usize;
        let (mut oracle_flux, mut oracle_area) = (0.0f64, 0.0f64);
        let h = 1.0 / n as f64;
        for i in 0..n {
            for j in 0..n {
                let (u, v) = ((i as f64 + 0.5) * h, (j as f64 + 0.5) * h);
                let s = sv(u, v);
                let du = core::array::from_fn::<f64, 3, _>(|k| {
                    (sv(u + 1e-6, v)[k] - sv(u - 1e-6, v)[k]) / 2e-6
                });
                let dv = core::array::from_fn::<f64, 3, _>(|k| {
                    (sv(u, v + 1e-6)[k] - sv(u, v - 1e-6)[k]) / 2e-6
                });
                let cr = [
                    du[1] * dv[2] - du[2] * dv[1],
                    du[2] * dv[0] - du[0] * dv[2],
                    du[0] * dv[1] - du[1] * dv[0],
                ];
                oracle_flux += (s[0] * cr[0] + s[1] * cr[1] + s[2] * cr[2]) * h.powi(2);
                oracle_area += (cr[0].powi(2) + cr[1].powi(2) + cr[2].powi(2)).sqrt() * h.powi(2);
            }
        }
        assert!(
            out.flux.lo() - 1e-4 <= oracle_flux && oracle_flux <= out.flux.hi() + 1e-4,
            "flux {:?} vs oracle {oracle_flux}",
            out.flux
        );
        assert!(
            out.area.lo() - 1e-2 <= oracle_area && oracle_area <= out.area.hi() + 1e-2,
            "area {:?} vs oracle {oracle_area}",
            out.area
        );
    }

    /// The quarter cylinder `x² + y² = 1`, `0 ≤ z ≤ 2` as ONE rational
    /// Bézier span by one linear span: the exact-v arm, one v span.
    fn one_span_quarter_cylinder() -> (KnotVector, KnotVector, Vec<RVec3>, Vec<f64>) {
        let kv_u = KnotVector::unit_segment(const { core::num::NonZeroUsize::new(2).unwrap() });
        let kv_v = KnotVector::unit_segment(core::num::NonZeroUsize::MIN);
        let w = core::f64::consts::FRAC_1_SQRT_2;
        let mut net = Vec::new();
        let mut weights = Vec::new();
        for ([x, y], h) in [([1.0, 0.0], 1.0), ([1.0, 1.0], w), ([0.0, 1.0], 1.0)] {
            for z in [0.0, 2.0] {
                net.push([pt(x), pt(y), pt(z)]);
                weights.push(h);
            }
        }
        (kv_u, kv_v, net, weights)
    }

    /// **The wall is cumulative, at the lane.** Never accepted, the
    /// one-span quarter cylinder's exact-v arm runs to its wall: rounds
    /// 0 to 11 spend 7.6e6 of the 1.425e7, and round 12 alone (7.6e6)
    /// would take it past, so it stops after 12 rounds. A wall read
    /// per round would admit round 12 and stop after 13.
    #[test]
    fn a_never_accepted_face_stops_at_its_cumulative_wall() {
        let (ku, kv, net, w) = one_span_quarter_cylinder();
        let run = nurbs_patch_face_rounds::<f64>(
            &patch(&ku, &kv, &net, &w),
            (0.0, 1.0, 0.0, 1.0),
            2.0f64.mul_add(2.0, core::f64::consts::PI),
            0.0,
            Band::linear(Tol::witness()).unwrap(),
            RoundWindow::SCHEDULE,
            &mut |_, _| Ok(false),
        )
        .expect("the quarter cylinder has an enclosure");
        assert_eq!(
            (run.end, run.round, run.rounds),
            (LaneEnd::Wall, 11, 12),
            "{run:?}"
        );
    }
}
