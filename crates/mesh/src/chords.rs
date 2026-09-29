//! Per-edge chord points: computed **once** from each edge's certified
//! carrier and consumed by both adjacent faces (the watertightness
//! half of the pure-function invariant, crate docs).
//!
//! Chord counts are deterministic ceil arithmetic from δ_s = δ/2 (the
//! documented sizing safety factor):
//!
//! - Line carriers: 1 chord (a segment of the exact locus).
//! - Circle carriers (radius ρ, forward span Δt): per-chord step
//!   φ = 2·acos(1 − δ_s/ρ) (the closed-form sagitta bound
//!   ρ(1 − cos(φ/2)) ≤ δ_s), capped at π/4 (`sizing::MAX_ANGULAR_STEP`);
//!   n = ceil(Δt/φ).
//! - Adjacent-torus tightening: a face on a torus certifies through
//!   the UV interpolation bound (crate docs), which needs boundary UV
//!   steps within its grid steps `(h_u, h_v)` =
//!   `sizing::torus_grid_steps`; a circle edge's carrier parameter
//!   *is* the torus chart coordinate along it (azimuth for rims, minor
//!   angle for meridians), so each adjacent torus face adds
//!   n ≥ ceil(Δt/h) with `h` the step of the edge's OWN direction —
//!   `sizing::torus_boundary_step` classifies it with the walk's own
//!   rim/meridian rule and says what refuses.
//! - Adjacent-NURBS tightening (M7, the trimmed-NURBS lane): the same
//!   shape with a hull-derived Hessian — a described NURBS face
//!   certifies through `crate::nurbs_cert`'s anisotropic bound, which
//!   needs boundary UV steps within the face's own (h_u, h_v); the
//!   half-edge's stored pcurve gives per-axis UV speed bounds
//!   (s_u, s_v) — exact `|pl|` components for the iso line, the
//!   closed-form sub-arc rate for the arc rim, the amplitude sum for
//!   the harmonic form, and for the `General` spline image the hull of
//!   its own differenced control net (`general_uv_speeds`, a convexity
//!   fact rather than a closed form, and a SUP the image need not
//!   attain) — so each adjacent NURBS face adds n ≥ ⌈s_u·Δt/h_u⌉ and
//!   ⌈s_v·Δt/h_v⌉, on EVERY carrier kind (a straight wall edge is one
//!   3-D chord but many UV steps). A `Fitted` image has no certified
//!   speed bound here and refuses typed (the trimmed lane's module
//!   docs name its consumer).
//!
//! An adjacent surface reaches a chord count only through
//! [`adjacent_surface`], and the two tightenings above are its two
//! call sites — the `Circle` arm's torus boundary step and
//! [`nurbs_tighten`].
//! The claim is therefore about one function's callers, which a reader
//! settles by grepping this file for the name. **Nothing in the tree
//! checks it**: a third caller compiles green, and it would be a third
//! way for a neighbour to enter the count.
//!
//! With that door held, chord points are a pure function of (carrier +
//! interval, endpoint points, adjacent surface parameters, δ).
//!
//! Polyline endpoints are the topology vertices' points **bitwise**
//! (never `carrier(t₀)`, which is only within ε of them) so every
//! polyline meeting at a vertex shares its mesh vertex id; interior
//! points are `carrier(t₀ + (t₁−t₀)·i/n)` in `he_plus`-forward order.

use geom_core::Bounds;
use std::collections::HashMap;

use geom::Curve3;
use geom_brep::Pcurve;
use geom_core::interval::Interval;
use geom_core::interval::certification::Certification;
use geom_core::spline::{KnotVector, SplineCoeffs};
use topo::{Body, EdgeKey};

use crate::nurbs_cert::{FaceBounds, face_bound};
use crate::sizing::{
    ceil_count, curvature_step, ellipse_step, sagitta_step, spiric_step, torus_boundary_step,
};
use crate::types::TessellateError;

/// The chord pass's output: every edge's chord-point ids and the
/// parameter schedule they were sampled at.
///
/// One value rather than two, because they are ONE derivation and
/// every consumer that reads the ids at a parameter reads both — the
/// trimmed lane evaluates pcurves at exactly the schedule the 3-D
/// chords were minted on, which is the property that keeps a face's
/// boundary and its neighbour's the same points.
pub(crate) struct ChordPass {
    /// Per-edge chord-point mesh ids, `he_plus`-forward.
    pub ids: HashMap<EdgeKey, Vec<u32>>,
    /// The matching per-edge chord parameters (endpoints included).
    pub params: HashMap<EdgeKey, Vec<f64>>,
}

/// Computes every edge's chord-point ids (minting interior points into
/// `positions`), in edge-arena order. `vids` maps topology vertices to
/// their already-minted mesh ids.
pub(crate) fn compute_chords(
    body: &Body<f64>,
    delta_s: f64,
    vids: &HashMap<topo::VertexKey, u32>,
    positions: &mut Vec<geom_core::Point3<f64>>,
    bounds: &mut FaceBounds,
) -> Result<ChordPass, TessellateError> {
    let mut chords = HashMap::new();
    // Chord PARAMETERS per edge (`he_plus`-forward, endpoints
    // included) — the trimmed-face lane evaluates pcurves at exactly
    // the chord schedule, so both stay one derivation (M5 PR 11).
    let mut params = HashMap::new();
    for (ek, edge) in body.edges() {
        let curve = body
            .get_curve_geom(edge.curve)
            .ok_or(TessellateError::MissingEntity { what: "edge curve" })?
            .certified()
            .ok_or(TessellateError::NullScaffoldEdge { edge: ek })?;
        let (t0, t1) = curve.params();
        let span = t1 - t0;
        let n = match *curve.carrier() {
            Curve3::Line { .. } => 1,
            Curve3::Circle { .. } => {
                let mut n =
                    ceil_count(span, sagitta_step(delta_s, circle_radius(curve.carrier())))?;
                for fk in adjacent_faces(body, ek)? {
                    if let Some(h) =
                        torus_boundary_step(adjacent_surface(body, fk)?, curve, ek, delta_s)?
                    {
                        n = n.max(ceil_count(span, h)?);
                    }
                }
                n
            }
            // Ellipse arcs (curved-cut boundaries, M5 PR 5): the
            // certified-conservative curvature-bound step; no torus
            // tightening applies (an ellipse never lies on a torus
            // chart of this kernel's constructions).
            Curve3::Ellipse { major, minor, .. } => {
                ceil_count(span, ellipse_step(delta_s, major, minor))?
            }
            // Spiric arcs (the hollowed partial revolve's torus rim):
            // the curvature-bound step from the closed-form `sup|C″|`
            // (`spiric_step`). No torus tightening: `torus_boundary_step`
            // is the circle arm's (a spiric is neither a rim nor a
            // meridian traversal of its chart), and the step here is
            // already a chord bound on the carrier itself.
            Curve3::Spiric {
                major_radius,
                minor_radius,
                offset,
                ..
            } => ceil_count(
                span,
                spiric_step(delta_s, major_radius, minor_radius, offset),
            )?,
            // B-spline carriers (rung-3 edges at rest, M5 PR 9):
            // the hull-bounded sagitta generalization (C9/PR 11) —
            // secant deviation on a parameter step h is ≤ h²·sup|C″|/8
            // (Taylor with integral remainder; needs C¹, i.e. interior
            // multiplicities ≤ p − 1), and sup|C″| is a control-
            // coefficient convexity fact via iterated derivative
            // hulls — on the CONTROL NET directly for an integral
            // carrier, through the quotient-rule assembly over the
            // homogeneous net for a RATIONAL one (M8-5; a
            // rational-walled body's seam edges read back rational,
            // so this gate is what makes the face bound reachable).
            Curve3::Nurbs(ref n) => nurbs_chord_count(n, span, delta_s, ek)?,
        };
        // Adjacent-NURBS tightening (module docs), on every carrier
        // kind — a straight wall edge is one 3-D chord but many UV
        // steps of the wall's certificate budget.
        let n = nurbs_tighten(body, ek, (t0, t1), delta_s, bounds, n)?;
        let (vs, ve) = edge_vertices(body, ek)?;
        let start_id = *vids.get(&vs).ok_or(TessellateError::MissingEntity {
            what: "start vertex",
        })?;
        let end_id = *vids
            .get(&ve)
            .ok_or(TessellateError::MissingEntity { what: "end vertex" })?;
        let mut ids = Vec::with_capacity(n + 1);
        let mut ts = Vec::with_capacity(n + 1);
        ids.push(start_id);
        ts.push(t0);
        for i in 1..n {
            #[allow(clippy::cast_precision_loss)]
            let t = t0 + span * (i as f64 / n as f64);
            #[allow(clippy::cast_possible_truncation)]
            let id = positions.len() as u32;
            positions.push(curve.carrier().eval(t));
            ids.push(id);
            ts.push(t);
        }
        ids.push(end_id);
        ts.push(t1);
        chords.insert(ek, ids);
        params.insert(ek, ts);
    }
    Ok(ChordPass {
        ids: chords,
        params,
    })
}

/// Chord count for a B-spline carrier from the hull-bounded sagitta
/// (module note at the call site): componentwise `sup|C″|` bounds give
/// `|C″| ≤ √(Σ sup²)`, and the per-step bound `h²·M/8 ≤ δ_s` sizes
/// `h`. Integral carriers take the direct control-hull arm; RATIONAL
/// carriers (M8-5) take [`rational_carrier_m_bound`] — the surface
/// bound's quotient-rule assembly one dimension down. (The former
/// blanket rational refusal claimed "arc-bearing profiles refuse at
/// the rational-wall gate" — false since M8-2's rational span meter:
/// arc-walled bodies BUILD now, and their seam edges read back
/// rational, which is exactly why this arm exists.)
fn nurbs_chord_count(
    n: &geom::NurbsCurve3<f64>,
    span: f64,
    delta_s: f64,
    ek: EdgeKey,
) -> Result<usize, TessellateError> {
    let rational = n.weights().iter().any(|w| *w != 1.0);
    // The convex-combination licence (the surface arm's rule): every
    // hull fact below needs strictly positive finite weights.
    // `!(w > 0.0)` catches NaN.
    #[allow(clippy::neg_cmp_op_on_partial_ord)]
    if rational && n.weights().iter().any(|w| !(*w > 0.0) || !w.is_finite()) {
        return Err(TessellateError::UnsupportedCurve {
            edge: ek,
            note: "rational B-spline carrier with a non-positive or non-finite weight — \
                   an illegal rational description: the convex-combination licence \
                   every hull fact rests on requires strictly positive weights",
        });
    }
    let kv = n.knots();
    let p = kv.degree();
    if p < 2 {
        // A single-segment degree-1 carrier is an exact chord — for a
        // RATIONAL one too: the image is still the segment between its
        // two control points (a Möbius reparameterization moves the
        // parameter, not the locus, and the single chord's deviation
        // is a locus fact). One with interior knots is a C⁰ polyline
        // whose kinks a uniform parameter schedule would miss —
        // refused, not guessed at.
        return if kv.interior_knots().next().is_none() {
            Ok(1)
        } else {
            Err(TessellateError::UnsupportedCurve {
                edge: ek,
                note: "degree-1 B-spline carrier with interior knots (a C⁰ polyline) — \
                       the uniform chord schedule cannot pin its kinks; split the edge",
            })
        };
    }
    // C¹ needed for the secant bound: interior multiplicities ≤ p − 1
    // (p ≥ 2 here — degree 1 returned above).
    if kv.interior_knots().any(|(_, m)| m > p - 1) {
        return Err(TessellateError::UnsupportedCurve {
            edge: ek,
            note: "B-spline carrier with a C⁰ kink (interior multiplicity = degree) — \
                   the hull sagitta bound needs C¹; split the edge at the kink",
        });
    }
    let m_bound = if rational {
        rational_carrier_m_bound(n, ek)?
    } else {
        let mut sum_sq = Interval::zero();
        for comp in 0..3 {
            let coeffs: Vec<Interval> = n
                .control()
                .iter()
                .map(|pt| {
                    Interval::point(match comp {
                        0 => pt.x,
                        1 => pt.y,
                        _ => pt.z,
                    })
                })
                .collect();
            let q1 = kv.difference_coeffs(&coeffs);
            let inner = kv.derivative_knot_slice().to_vec();
            let Ok(kv1) = KnotVector::clamped(inner, p - 1) else {
                return Err(TessellateError::UnsupportedCurve {
                    edge: ek,
                    note: "B-spline carrier whose derivative knot vector fails to \
                           materialise — outside the certified chord inventory",
                });
            };
            // The hull of the SECOND-difference net, through the
            // geom-core door rather than a fold spelled here: the
            // second difference is the first difference of `q1`
            // against the derivative vector `kv1`, which is exactly
            // what `derivative_domain_hull` answers. A length the
            // mint refuses arrives refused, as `difference_coeffs`
            // would have delivered it.
            let hull = kv1
                .with_coeffs(&q1)
                .map_or_else(Interval::refused, SplineCoeffs::derivative_domain_hull);
            sum_sq = sum_sq + hull.sqr();
        }
        // A refused hull has no bound to report: `NaN` is what the
        // `is_finite` test below reads as "unbounded/refused", and
        // the refusal is asked by name because interval arithmetic carries it in
        // the decoration rather than in the endpoints.
        if !sum_sq.is_certified() {
            f64::NAN
        } else {
            sum_sq.hi().sqrt().next_up()
        }
    };
    if !m_bound.is_finite() {
        return Err(TessellateError::UnsupportedCurve {
            edge: ek,
            note: "B-spline carrier second-derivative hull is unbounded/refused — \
                   outside the certified chord inventory",
        });
    }
    if m_bound == 0.0 {
        return Ok(1);
    }
    ceil_count(span, curvature_step(delta_s, m_bound))
}

/// Certified `sup‖C″‖` for a RATIONAL B-spline carrier (M8-5): the
/// face bound's quotient-rule assembly one dimension down. Write
/// `C = A/w` with `A = Σ Nᵢ wᵢ Pᵢ`, `w = Σ Nᵢ wᵢ` (both polynomial);
/// for any constant `c` (`Ã = A − c·w`):
///
/// ```text
/// C′ = (Ã′ − (C − c)·w′) / w
/// C″ = (Ã″ − 2·C′·w′ − (C − c)·w″) / w
/// ```
///
/// Per span (after the fixed
/// [`geom_brep::patch_bound::RATIONAL_CERT_SPLITS`] refinement), each
/// ingredient is an active-window hull on the homogeneous nets:
/// `sup|C − c| ≤ max_active |P − c|` (positive weights — the licence
/// the caller checked — make the rational basis a nonnegative
/// partition of unity), `sup|Ã'|`/`sup|Ã″|`/`sup|w′|`/`sup|w″|` are
/// iterated [`geom_core::spline::SplineCoeffs::derivative_coeffs`] hulls, and the divisor is the span's
/// weight range: for a SUP bound with a nonnegative numerator the
/// conservative division is by `w_min` (the mirror image of the speed
/// meter's lower-bound `w_max` choice — the interval division by
/// `[w_lo, w_hi]` computes exactly that, outward-rounded, and refuses
/// if positivity was never proven). Recentring at the span's control
/// centroid keeps the cross terms span-sized. The domain bound is the
/// max over spans (hull of the squared enclosures), `next_up` after
/// the final square root — a refusal flows to the caller's finite check.
fn rational_carrier_m_bound(
    n: &geom::NurbsCurve3<f64>,
    ek: EdgeKey,
) -> Result<f64, TessellateError> {
    let refined = n
        .refine_knots(&geom_brep::patch_bound::rational_split_points(n.knots()))
        .map_err(|_| TessellateError::UnsupportedCurve {
            edge: ek,
            note: "rational B-spline carrier whose refinement fails to materialise — \
                   outside the certified chord inventory",
        })?;
    // Positivity survives insertion in ℝ; re-checked on the refined
    // weights because this code may not assume floating point did.
    #[allow(clippy::neg_cmp_op_on_partial_ord)]
    if refined
        .weights()
        .iter()
        .any(|w| !(*w > 0.0) || !w.is_finite())
    {
        return Err(TessellateError::UnsupportedCurve {
            edge: ek,
            note: "rational B-spline carrier whose refined weights lost positivity — \
                   outside the certified chord inventory",
        });
    }
    let kv = refined.knots();
    let p = kv.degree(); // ≥ 2: the caller's degree gate ran first
    let inner = kv.derivative_knot_slice().to_vec();
    let Ok(kv1) = KnotVector::clamped(inner, p - 1) else {
        return Err(TessellateError::UnsupportedCurve {
            edge: ek,
            note: "B-spline carrier whose derivative knot vector fails to \
                   materialise — outside the certified chord inventory",
        });
    };
    // Homogeneous coefficient nets and their derivative enclosures.
    let w_pts: Vec<Interval> = refined
        .weights()
        .iter()
        .map(|w| Interval::point(*w))
        .collect();
    let dw = kv.difference_coeffs(&w_pts);
    let ddw = kv1.difference_coeffs(&dw);
    let comp = |c: usize| -> Vec<Interval> {
        refined
            .control()
            .iter()
            .zip(refined.weights())
            .map(|(pt, w)| {
                Interval::point(*w)
                    * Interval::point(match c {
                        0 => pt.x,
                        1 => pt.y,
                        _ => pt.z,
                    })
            })
            .collect()
    };
    let a_nets: Vec<(Vec<Interval>, Vec<Interval>)> = (0..3)
        .map(|c| {
            let a = comp(c);
            let da = kv.difference_coeffs(&a);
            let dda = kv1.difference_coeffs(&da);
            (da, dda)
        })
        .collect();
    // The signed hull of `net[i] − c·wnet[i]` over `[i0, i1]`
    // (out-of-range is refused; recentring commutes with differencing).
    let window = |net: &[Interval],
                  wnet: &[Interval],
                  c: Interval,
                  active: core::ops::RangeInclusive<usize>|
     -> Interval {
        let mut acc: Option<Interval> = None;
        for i in active {
            let e = match (net.get(i), wnet.get(i)) {
                (Some(&a), Some(&w)) => a - c * w,
                _ => Interval::refused(),
            };
            acc = Some(match acc {
                None => e,
                Some(h) => Interval::hull(h, e),
            });
        }
        acc.unwrap_or_else(Interval::refused)
    };
    let mag = |h: Interval| Interval::from_bounds(0.0, h.mag());
    let two = Interval::point(2.0);
    let mut sq_acc: Option<Interval> = None;
    for s in kv.first_span()..=kv.last_span() {
        // Emptiness check and window validation in one step: `span`
        // yields `None` exactly for the empty spans this loop skipped,
        // and its `first_control` is `s − p` computed once, in range.
        // The two runtime refusals this replaces ("span below its
        // degree", "span beyond its control net") are now
        // unrepresentable: `Span` exists only for `p ≤ s ≤ last_span`,
        // and `last_span = knots.len() − p − 2 = control_count − 1`,
        // which `validate_counts` pins to `refined.control().len()`.
        let Some(span) = kv.span(s) else { continue };
        // The span centroid — a translation CHOICE (any finite c is
        // sound), f64 structure, fixed order.
        let mut csum = [0.0f64; 3];
        let mut count = 0.0f64;
        for pt in &refined.control()[span.window()] {
            csum[0] += pt.x;
            csum[1] += pt.y;
            csum[2] += pt.z;
            count += 1.0;
        }
        let cen = [csum[0] / count, csum[1] / count, csum[2] / count];
        // The span's weight range — the divisor (doc comment).
        let mut w_span: Option<Interval> = None;
        for w in &w_pts[span.window()] {
            w_span = Some(match w_span {
                None => *w,
                Some(h) => Interval::hull(h, *w),
            });
        }
        let w_span = w_span.unwrap_or_else(Interval::refused);
        let zero = Interval::zero();
        // Active windows: value [s−p, s]; each differencing drops the
        // top index, which is what `derived_window` names — so `s − 1`
        // and `s − 2` are not subtractions at the use site either. The
        // caller's degree gate makes p ≥ 2, so the order-2 window is
        // always `Some`; if that ever stopped holding the bound
        // is REFUSED (and the caller's finite check reads it) rather than
        // underflowing.
        // `p ≥ 2` (the caller's degree gate), so the order-2 window is
        // `Some` on every reachable path. It is asserted rather than
        // merely commented: `debug_assert` is the tree's fail-loud form
        // for a state that cannot occur — the panic family is denied in
        // kernel code (workspace lints), so the release build still
        // takes the total route below and REFUSES the
        // bound instead of quietly under-reporting it.
        let d2 = span.derived_window(2);
        debug_assert!(
            d2.is_some(),
            "the degree gate promised p ≥ 2 for this carrier, got {p}"
        );
        let w1 = mag(window(&dw, &dw, zero, span.first_derived_window()));
        let w2 = mag(d2
            .clone()
            .map_or_else(Interval::refused, |a| window(&ddw, &ddw, zero, a)));
        let mut sq = Interval::zero();
        for (c, (da, dda)) in a_nets.iter().enumerate() {
            let cc = Interval::point(cen[c]);
            let mut v0h: Option<Interval> = None;
            for pt in &refined.control()[span.window()] {
                let e = Interval::point(match c {
                    0 => pt.x,
                    1 => pt.y,
                    _ => pt.z,
                }) - cc;
                v0h = Some(match v0h {
                    None => e,
                    Some(h) => Interval::hull(h, e),
                });
            }
            let v0 = mag(v0h.unwrap_or_else(Interval::refused));
            let a1 = mag(window(da, &dw, cc, span.first_derived_window()));
            let a2 = mag(d2
                .clone()
                .map_or_else(Interval::refused, |a| window(dda, &ddw, cc, a)));
            let s1 = (a1 + v0 * w1) / w_span;
            let s2 = (a2 + two * s1 * w1 + v0 * w2) / w_span;
            sq = sq + s2.sqr();
        }
        sq_acc = Some(match sq_acc {
            None => sq,
            Some(h) => Interval::hull(h, sq),
        });
    }
    // Same contract, same reason: a refused hull answers `NaN`.
    Ok(sq_acc.map_or(f64::NAN, |s| {
        if !s.is_certified() {
            f64::NAN
        } else {
            s.hi().sqrt().next_up()
        }
    }))
}

/// The adjacent-NURBS chord tightening (module docs): for each
/// adjacent described NURBS face, raise `n` until the edge's UV image
/// steps fit inside the face's certificate-budget grid steps. The
/// bound comes from the tessellation's shared [`FaceBounds`] memo, so
/// the Hessian hull is assembled once per face for the whole run.
fn nurbs_tighten(
    body: &Body<f64>,
    ek: EdgeKey,
    params: (f64, f64),
    delta_s: f64,
    bounds: &mut FaceBounds,
    mut n: usize,
) -> Result<usize, TessellateError> {
    let (t0, t1) = params;
    let span = t1 - t0;
    let edge = body
        .get_edge(ek)
        .ok_or(TessellateError::MissingEntity { what: "edge" })?;
    for hek in [edge.he_plus, edge.he_minus] {
        let he = body
            .get_half_edge(hek)
            .ok_or(TessellateError::MissingEntity { what: "half-edge" })?;
        let lp = body
            .get_loop(he.parent_loop)
            .ok_or(TessellateError::MissingEntity {
                what: "parent loop",
            })?;
        let fk = lp.face;
        let surface = adjacent_surface(body, fk)?;
        // The UV step schedule is a statement about the chart, so both
        // spline kinds take it — an approximating surface's chart is
        // its fit's.
        let Some(payload) = surface.spline_chart() else {
            continue;
        };
        if payload.is_placeholder() {
            return Err(TessellateError::UnsupportedSurface { face: fk });
        }
        let (hu, hv) = face_bound(bounds, payload, fk)?.grid_steps(delta_s);
        let Some(cache) = body.pcurve(hek) else {
            return Err(TessellateError::UnsupportedCurve {
                edge: ek,
                note: "NURBS-face half-edge carries no stored pcurve cache — caches \
                       mint at loft/sweep assembly and STEP adoption; without one \
                       the chord schedule has no certified UV step bound",
            });
        };
        // Per-axis UV speed bounds (module docs): exact for the iso
        // line, the amplitude sum |pa|+|pb|+|pl| componentwise for the
        // harmonic form (|P′| = |−pa·sin t + pb·cos t + pl|), the
        // derivative net's hull for the general spline image.
        let (su, sv) = match cache.pcurve() {
            Pcurve::IsoLine { pl, .. } => (pl.x.abs(), pl.y.abs()),
            // The ARC rim (M8-3). With `s = ½ + tan(φ/2)/(2·tan(h/4))`
            // and `g = (k + s)/m`,
            //   `dg/dt = sec²(φ/2) / (4·m·tan(h/4))`,
            // maximal at the sub-arc ends (`|φ| = h/2`), where
            // `sec²(h/4)/(4·m·tan(h/4)) = 1/(2·m·sin(h/2))`.
            // A closed-form sup over the whole span, like the other
            // two arms — no sampling.
            Pcurve::IsoArc {
                pd, angle, breaks, ..
            } => {
                let spans = breaks.control_count().saturating_sub(1);
                if spans == 0 {
                    return Err(TessellateError::UnsupportedCurve {
                        edge: ek,
                        note: "an arc-rim pcurve with no sub-arc structure — a malformed \
                               cache, not a chord-schedule question",
                    });
                }
                #[allow(clippy::cast_precision_loss)]
                let m = spans as f64;
                let rate = 1.0 / (2.0 * m * (angle / (2.0 * m)).sin());
                (pd.x.abs() * rate, pd.y.abs() * rate)
            }
            Pcurve::Harmonic { pa, pb, pl, .. } => (
                pa.x.abs() + pb.x.abs() + pl.x.abs(),
                pa.y.abs() + pb.y.abs() + pl.y.abs(),
            ),
            Pcurve::Fitted(_) => {
                return Err(TessellateError::UnsupportedCurve {
                    edge: ek,
                    note: "NURBS-face half-edge carries a FITTED (rung-3) pcurve — no \
                           certified UV speed bound is wired for a fitted image's \
                           chord schedule; its first tessellation consumer is the \
                           edge×NURBS-face boolean layer (the cut-loft unit)",
                });
            }
            // The general curve-in-UV arm (U2): no closed form, but
            // the same KIND of fact as its two siblings — a hull over
            // the image's own differenced control net
            // ([`general_uv_speeds`]).
            Pcurve::General(image) => general_uv_speeds(image, params, ek)?,
        };
        n = n
            .max(ceil_count(su * span, hu)?)
            .max(ceil_count(sv * span, hv)?);
    }
    Ok(n)
}

/// The per-axis UV speed sups `(s_u, s_v)` of a **general
/// curve-in-UV** chart image — what [`nurbs_tighten`] reads where the
/// `IsoLine` arm reads its exact `|pl|` components and the `Harmonic`
/// arm its amplitude sum `|pa| + |pb| + |pl|`. Those two are closed
/// forms of the image; this one is a CONVEXITY fact about its control
/// net, which is the only shape a spline image admits.
///
/// **The fact.** For `P(t) = Σ_j N_{j,p}(t)·P_j` on a clamped vector,
/// the derivative is again a spline — `P′(t) = Σ_j N_{j,p−1}(t)·Q_j`
/// with `Q_j = p·(P_{j+1} − P_j)/(u_{j+p+1} − u_{j+1})` — and the
/// `N_{j,p−1}` are a nonnegative partition of unity, so every value of
/// `u′` is a convex combination of the `Q^u_j` and lies in their hull.
/// `max_j |Q^u_j|` is therefore a certified `sup|u′|`, with no
/// evaluation and no sampling, and likewise in `v`. Both the
/// differencing and the hull are
/// [`SplineCoeffs::derivative_domain_hull`]'s, not respelled here.
///
/// **The bound is a SUP, not an attained maximum.** It is attained
/// exactly when the winning coefficient's basis function reaches 1
/// somewhere — at a clamped end, or at an interior knot of a
/// degree-`p` image whose derivative basis is degree `p − 1`. On an
/// interior coefficient a higher-degree basis never fully activates,
/// and the bound is then a genuine over-estimate (the row
/// `general_uv_speeds_dominate_the_sampled_image_speeds` carries one
/// leg of each kind, with the measured slack).
///
/// **The parameter range.** The hull is over the image's WHOLE knot
/// domain, and the caller's `[t₀, t₁]` must lie inside it — which is
/// what this function checks, because `eval`/`deriv` extrapolate the
/// end span's polynomial past the domain and no hull of the net bounds
/// that. At every mint on this head the two coincide (the image is
/// built on the carrier's own parameter), so the check is a premise
/// made explicit rather than a live refusal; a proper sub-range is
/// admitted and strictly over-bounded, which is sound in the only
/// direction that matters. A per-span reading for a genuine sub-range
/// producer is the sibling `rational_carrier_m_bound`'s shape.
///
/// **Constant weights only.** The hull licence above is the
/// POLYNOMIAL one. A constant weight vector cancels out of the
/// rational basis (`R_j = N_j·c / Σ_k N_k·c = N_j`), so such an image
/// IS its polynomial wrap and the differenced control net is exactly
/// its derivative net; a genuinely varying weight vector's derivative
/// is a quotient whose coefficients are not that net, and refuses
/// typed. Asking "constant" rather than "every weight is 1" is
/// `work/trim/rational-gates-test-unit-weights-not-constancy.md`'s fix
/// shape, which names this file.
///
/// **What this function does NOT check, because the type already
/// does.** A degree-0 image and an interior multiplicity above the
/// degree are both refused by [`KnotVector::clamped`] at construction,
/// so no image carrying either can reach here. Multiplicity `= p` — a
/// C⁰ kink — IS constructible and IS admitted: the bound is a
/// Lipschitz statement, which C⁰ suffices for.
fn general_uv_speeds(
    image: &geom::NurbsCurve2<f64>,
    params: (f64, f64),
    ek: EdgeKey,
) -> Result<(f64, f64), TessellateError> {
    let weights = image.weights();
    let w0 = weights.first().copied().unwrap_or(f64::NAN);
    // `!(w0 > 0.0)` catches NaN; the positivity is the convex-
    // combination licence the constant has to carry with it.
    #[allow(clippy::neg_cmp_op_on_partial_ord)]
    if !(w0 > 0.0) || !w0.is_finite() || weights.iter().any(|w| *w != w0) {
        return Err(TessellateError::UnsupportedCurve {
            edge: ek,
            note: "NURBS-face half-edge carries a RATIONAL general curve-in-UV pcurve — \
                   the certified UV speed here is the polynomial net's difference hull, \
                   and a varying weight vector's derivative is not that net; no mint \
                   produces a rational chart image of this class",
        });
    }
    let kv = image.knots();
    let (d0, d1) = kv.domain();
    let (t0, t1) = params;
    if !(t0 >= d0 && t1 <= d1) {
        return Err(TessellateError::UnsupportedCurve {
            edge: ek,
            note: "a general curve-in-UV pcurve read outside its own knot domain — the \
                   control-net hull bounds the image's speed on its domain and says \
                   nothing about the end span's polynomial extended past it",
        });
    }
    let mut speeds = [f64::NAN; 2];
    for (axis, s) in speeds.iter_mut().enumerate() {
        let coeffs: Vec<Interval> = image
            .control()
            .iter()
            .map(|pt| Interval::point(if axis == 0 { pt.x } else { pt.y }))
            .collect();
        // `mag` is NaN on a refusal — a coefficient array the mint
        // refuses arrives refused and leaves as the refusal below,
        // never as a finite bound.
        *s = kv
            .with_coeffs(&coeffs)
            .map_or_else(Interval::refused, SplineCoeffs::derivative_domain_hull)
            .mag()
            .next_up();
    }
    let [su, sv] = speeds;
    if !su.is_finite() || !sv.is_finite() {
        return Err(TessellateError::UnsupportedCurve {
            edge: ek,
            note: "general curve-in-UV pcurve whose derivative control hull is \
                   unbounded/refused — outside the certified chord inventory",
        });
    }
    Ok((su, sv))
}

/// The radius of a circle carrier (caller guarantees the variant).
fn circle_radius(carrier: &Curve3<f64>) -> f64 {
    match *carrier {
        Curve3::Circle { radius, .. } => radius,
        _ => f64::NAN,
    }
}

/// The (start, end) vertices of the edge's `he_plus`.
pub(crate) fn edge_vertices(
    body: &Body<f64>,
    ek: EdgeKey,
) -> Result<(topo::VertexKey, topo::VertexKey), TessellateError> {
    let edge = body
        .get_edge(ek)
        .ok_or(TessellateError::MissingEntity { what: "edge" })?;
    let he = body
        .get_half_edge(edge.he_plus)
        .ok_or(TessellateError::MissingEntity { what: "he_plus" })?;
    let end = body
        .half_edge_end(edge.he_plus)
        .ok_or(TessellateError::MissingEntity {
            what: "he_plus end",
        })?;
    Ok((he.start, end))
}

/// The door an adjacent face's surface reaches a chord count through.
/// The module header's claim is a claim about this function's call
/// sites, and it names them: the `Circle` arm's torus tightening and
/// [`nurbs_tighten`].
///
/// **Nothing counts those call sites.** What the name buys is a token
/// to grep for, which is what the claim lacked; a third caller still
/// compiles green.
fn adjacent_surface(
    body: &Body<f64>,
    fk: topo::FaceKey,
) -> Result<&geom::Surface<f64>, TessellateError> {
    let face = body
        .get_face(fk)
        .ok_or(TessellateError::MissingEntity { what: "face" })?;
    body.get_surface(face.surface)
        .ok_or(TessellateError::MissingEntity {
            what: "face surface",
        })
}

/// The (≤ 2 distinct) faces adjacent to an edge.
fn adjacent_faces(body: &Body<f64>, ek: EdgeKey) -> Result<Vec<topo::FaceKey>, TessellateError> {
    let edge = body
        .get_edge(ek)
        .ok_or(TessellateError::MissingEntity { what: "edge" })?;
    let mut out = Vec::with_capacity(2);
    for hek in [edge.he_plus, edge.he_minus] {
        let he = body
            .get_half_edge(hek)
            .ok_or(TessellateError::MissingEntity { what: "half-edge" })?;
        let lp = body
            .get_loop(he.parent_loop)
            .ok_or(TessellateError::MissingEntity {
                what: "parent loop",
            })?;
        if !out.contains(&lp.face) {
            out.push(lp.face);
        }
    }
    Ok(out)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::nurbs_cert::tests::Domination;
    use geom::NurbsCurve3;
    use geom_core::{Point2, Point3};
    use topo::EdgeKey;

    /// A degree-1 chart image, the shape this head's producer mints
    /// (`edge_nurbs::PXN_IMAGE_DEGREE = 1`, 33 interpolated feet): a
    /// polyline whose per-span speed IS a difference coefficient, so
    /// the hull bound is the sup exactly and any slack is a defect.
    fn polyline_image() -> geom::NurbsCurve2<f64> {
        let n = 9;
        let pts: Vec<Point2<f64>> = (0..n)
            .map(|i| {
                let t = f64::from(i) / f64::from(n - 1);
                Point2::new(1.0 + t + 0.15 * (6.0 * t).sin(), 0.4 * t * t - 0.2 * t)
            })
            .collect();
        let mut knots = vec![0.0];
        knots.extend((0..n).map(|i| f64::from(i) / f64::from(n - 1)));
        knots.push(1.0);
        let kv = KnotVector::clamped(knots, 1).unwrap();
        geom::NurbsCurve2::new(kv, pts, vec![1.0; n as usize]).unwrap()
    }

    /// A degree-3 chart image whose per-axis derivative-net maximum
    /// sits on an INTERIOR coefficient (`Q_1^u = 3.0` against
    /// `Q_0 = Q_2 = Q_3 = 1.2`): a degree-2 derivative basis peaks at
    /// ½ and never reaches 1, so the hull bound is a genuine
    /// over-estimate — `sup|u′|` sampled ≈ 2.4 against a certified 3.0.
    ///
    /// This is the leg that exercises what convexity is FOR. A bound
    /// taken over only the end coefficients is wrong here and right on
    /// the other two legs, which is exactly the degradation a
    /// domination row has to be able to see.
    ///
    /// Adopted from the review lane's interior-maximum probe.
    fn interior_max_cubic() -> geom::NurbsCurve2<f64> {
        let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
        let pts = vec![
            Point2::new(0.0, 0.0),
            Point2::new(0.2, 0.0),
            Point2::new(1.2, 0.5),
            Point2::new(1.6, 1.0),
            Point2::new(1.8, 1.0),
        ];
        geom::NurbsCurve2::new(kv, pts, vec![1.0; 5]).unwrap()
    }

    /// A degree-2 image whose maximum is attained ONLY at its interior
    /// knot: the derivative basis is degree 1, whose Greville
    /// abscissae ARE the knots, so `Q_1` is reached exactly at
    /// `t = ½` and nowhere else. `Q_1^u = 2·(0.9 − 0.1)/(1 − 0) = 1.6`
    /// against `Q_0^u = Q_2^u = 0.4`.
    ///
    /// Two things ride on this leg: the bound is ATTAINED (so the
    /// two-sided pin applies), and it is attained at a point a uniform
    /// sampling grid can miss — which is what makes
    /// [`sampled_uv_speeds`]' both-sides-of-every-interior-knot
    /// augmentation load-bearing rather than decorative.
    ///
    /// Adopted from the review lane's interior-knot probe.
    fn interior_knot_quadratic() -> geom::NurbsCurve2<f64> {
        let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0], 2).unwrap();
        let pts = vec![
            Point2::new(0.0, 0.0),
            Point2::new(0.1, 0.3),
            Point2::new(0.9, 0.4),
            Point2::new(1.0, 0.7),
        ];
        geom::NurbsCurve2::new(kv, pts, vec![1.0; 4]).unwrap()
    }

    /// The densest honest reading of the image's per-axis UV speeds:
    /// `|u′|` and `|v′|` at `samples + 1` parameters across the
    /// domain, plus both sides of every interior knot (a degree-1
    /// image's speed is piecewise constant and its extremes sit on the
    /// spans, which a grid that lands ON a break would read as the
    /// one-sided value the evaluator happens to pick;
    /// [`interior_knot_quadratic`] is the leg that makes this an
    /// assertion rather than a precaution).
    fn sampled_uv_speeds(c: &geom::NurbsCurve2<f64>, samples: usize) -> (f64, f64) {
        let (d0, d1) = c.knots().domain();
        let span = d1 - d0;
        let mut ts: Vec<f64> = Vec::new();
        for i in 0..=samples {
            #[allow(clippy::cast_precision_loss)]
            let lam = i as f64 / samples as f64;
            ts.push(d0 + span * lam);
        }
        for (k, _) in c.knots().interior_knots() {
            ts.push(k - span * 1e-9);
            ts.push(k + span * 1e-9);
        }
        let (mut su, mut sv) = (0.0f64, 0.0f64);
        for t in ts {
            let d = c.deriv(t.clamp(d0, d1));
            su = su.max(d.x.abs());
            sv = sv.max(d.y.abs());
        }
        (su, sv)
    }

    /// **DOMINATION — the `General` arm's certified sup is a sup, and
    /// on the legs where it is attained it is not padded.**
    ///
    /// [`general_uv_speeds`] answers a convexity fact about the
    /// differenced control net; this row answers the same question by
    /// DENSE SAMPLING of `P′` and requires the certificate to dominate
    /// it on both axes. The vertex-count row downstream (E2 in
    /// `sweep/tests/m8_4_intersection_iso.rs`) cannot see a sup that
    /// is too SMALL — it would simply schedule fewer chords and still
    /// produce a mesh — so this is the row that can, and it prints
    /// both sides at full precision so a near miss is readable.
    ///
    /// **Three legs, and which is which** (the reason a single fixture
    /// will not do):
    ///
    /// | leg | where the max sits | attained? |
    /// | --- | --- | --- |
    /// | [`polyline_image`] — degree 1, the head's minted shape | an END coefficient | yes, exactly |
    /// | [`interior_knot_quadratic`] — degree 2 | an INTERIOR coefficient, at the interior knot | yes, exactly, at one point |
    /// | [`interior_max_cubic`] — degree 3 | an INTERIOR coefficient, mid-span | no — `sup` ≈ 2.4 against a certified 3.0 |
    ///
    /// The two-sided pin runs on the first two. A bound folded over
    /// only the end coefficients passes the first leg unchanged and
    /// fails the other two, which is what the middle column is for; a
    /// bound scaled down anywhere fails all three.
    #[test]
    fn general_uv_speeds_dominate_the_sampled_image_speeds() {
        for (name, image, attained) in [
            (
                "degree-1 polyline, max at an end coefficient (the head's minted shape)",
                polyline_image(),
                true,
            ),
            (
                "degree-2, max at an interior coefficient attained at the interior knot",
                interior_knot_quadratic(),
                true,
            ),
            (
                "degree-3, max at an interior coefficient the basis never fully activates",
                interior_max_cubic(),
                false,
            ),
        ] {
            let (d0, d1) = image.knots().domain();
            let (su, sv) = general_uv_speeds(&image, (d0, d1), EdgeKey::default())
                .expect("a constant-weight polynomial image is in the certified inventory");
            let (wu, wv) = sampled_uv_speeds(&image, 4096);
            println!(
                "DOMINATION {name}: certified s_u {su:.17e} vs sampled {wu:.17e}; \
                 certified s_v {sv:.17e} vs sampled {wv:.17e}"
            );
            assert!(
                wu > 0.0 && wv > 0.0,
                "{name}: the fixture must MOVE on both axes, else the row is vacuous — \
                 sampled sup|u'| {wu:.17e}, sup|v'| {wv:.17e}"
            );
            assert!(
                wu <= su,
                "{name}: the certified sup|u'| {su:.17e} is DOMINATED BY a sampled \
                 speed {wu:.17e} — the chord schedule would under-count"
            );
            assert!(
                wv <= sv,
                "{name}: the certified sup|v'| {sv:.17e} is DOMINATED BY a sampled \
                 speed {wv:.17e} — the chord schedule would under-count"
            );
            if attained {
                // Where the winning coefficient's basis function
                // reaches 1, the hull max IS the sup and the two sides
                // may differ only by each side's outward rounding —
                // interval arithmetic's difference quotient and its `next_up` on
                // one side, the evaluator's basis pass on the other.
                // `1e-14` relative is ~45 ulps at these magnitudes,
                // measured at ~6e-16; a rewrite that pads the bound by
                // so much as 1e-13 of it reds here.
                assert!(
                    su <= wu * (1.0 + 1e-14) && sv <= wv * (1.0 + 1e-14),
                    "{name}: this bound is ATTAINED, so a certified \
                     s = ({su:.17e}, {sv:.17e}) this far above the sampled \
                     ({wu:.17e}, {wv:.17e}) is slack this arm must not carry"
                );
            } else {
                // The converse claim, so the table above is a
                // measurement and not a belief: on this leg the bound
                // is STRICTLY loose, by more than 20%. An arm that
                // became exact here would be a different mechanism and
                // owes a new argument.
                assert!(
                    su > wu * 1.2,
                    "{name}: the u bound is meant to be convexity-LOOSE here — \
                     certified {su:.17e} against sampled {wu:.17e}"
                );
            }
        }
    }

    /// **The rational refusal is live and load-bearing.** A constant
    /// weight vector cancels out of the rational basis, so a
    /// constant-`2.0` image IS its polynomial wrap and answers
    /// identically to the unit-weight one; a VARYING weight vector's
    /// derivative is a quotient whose coefficients are not the
    /// differenced net, and refuses typed.
    ///
    /// The constancy half is
    /// `work/trim/rational-gates-test-unit-weights-not-constancy.md`'s
    /// own fix shape, which names this file.
    #[test]
    fn a_constant_weight_image_answers_and_a_varying_one_refuses() {
        let base = interior_max_cubic();
        let (d0, d1) = base.knots().domain();
        let plain = general_uv_speeds(&base, (d0, d1), EdgeKey::default()).unwrap();

        let two = geom::NurbsCurve2::new(
            base.knots().clone(),
            base.control().to_vec(),
            vec![2.0; base.control().len()],
        )
        .unwrap();
        let constant = general_uv_speeds(&two, (d0, d1), EdgeKey::default())
            .expect("a constant weight vector is the polynomial wrap");
        assert_eq!(
            plain, constant,
            "constant weights cancel: the same curve must answer the same sup"
        );

        let mut w = vec![1.0; base.control().len()];
        w[2] = 0.5;
        let varying =
            geom::NurbsCurve2::new(base.knots().clone(), base.control().to_vec(), w).unwrap();
        let err = general_uv_speeds(&varying, (d0, d1), EdgeKey::default())
            .expect_err("a varying weight vector has no certified net here");
        assert!(
            matches!(&err, TessellateError::UnsupportedCurve { note, .. }
                     if note.contains("RATIONAL")),
            "the refusal names its own class: {err:?}"
        );
    }

    /// **A C⁰ kink is admitted; the type refuses what this arm does
    /// not.** Interior multiplicity `= p` is constructible and the
    /// Lipschitz bound holds across it. Degree 0 and multiplicity
    /// `p + 1` are refused by [`KnotVector::clamped`] itself, so the
    /// arm carries no guard for either — this row is why those two
    /// sentences in its doc are a measurement.
    #[test]
    fn the_knot_vector_type_refuses_what_the_speed_arm_does_not_guard() {
        assert!(KnotVector::clamped(vec![0.0, 1.0], 0).is_err(), "degree 0");
        assert!(
            KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 0.5, 0.5, 1.0, 1.0, 1.0], 2).is_err(),
            "interior multiplicity p + 1"
        );
        let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0], 2).unwrap();
        let pts = vec![
            Point2::new(0.0, 0.0),
            Point2::new(0.3, 0.1),
            Point2::new(0.5, 0.9),
            Point2::new(0.7, 0.2),
            Point2::new(1.0, 1.0),
        ];
        let image = geom::NurbsCurve2::new(kv, pts, vec![1.0; 5]).unwrap();
        let (su, sv) = general_uv_speeds(&image, (0.0, 1.0), EdgeKey::default())
            .expect("multiplicity = p is a C0 kink, which a Lipschitz bound survives");
        let (wu, wv) = sampled_uv_speeds(&image, 4096);
        println!("C0 KINK: certified ({su:.17e}, {sv:.17e}) vs sampled ({wu:.17e}, {wv:.17e})");
        let d = Domination::sampled_under_certified(&[("u'", wu, su), ("v'", wv, sv)]);
        assert!(
            wu > 0.0 && wv > 0.0 && d.holds(),
            "C0 kink: sampled speeds must be positive and under the certified sups: {d}"
        );
    }

    /// **The domain premise is checked, not assumed.** The hull bounds
    /// the image on its own knot domain; past the domain `deriv`
    /// extends the end span's polynomial and the hull says nothing.
    /// A half-edge interval reaching outside refuses typed; a proper
    /// sub-range is admitted and strictly over-bounded.
    #[test]
    fn a_read_outside_the_images_domain_refuses_and_a_subrange_is_over_bounded() {
        let image = interior_max_cubic();
        let (d0, d1) = image.knots().domain();
        let err = general_uv_speeds(&image, (d0, d1 + 0.25), EdgeKey::default())
            .expect_err("past the domain there is no certified bound");
        assert!(
            matches!(&err, TessellateError::UnsupportedCurve { note, .. }
                     if note.contains("outside its own knot domain")),
            "{err:?}"
        );
        let whole = general_uv_speeds(&image, (d0, d1), EdgeKey::default()).unwrap();
        let part = general_uv_speeds(&image, (d0, d1 * 0.5), EdgeKey::default()).unwrap();
        assert_eq!(
            whole, part,
            "the bound is the whole domain's whatever sub-range is asked for — \
             conservative, and the premise the doc states"
        );
    }

    /// **The count formula, from a hand-built sup.** The arm's answer
    /// feeds `n ≥ ⌈s·Δt/h⌉` and nothing else rows that step, so a
    /// `ceil` quietly become a `floor` (or an off-by-one in the step
    /// division) would be caught only by another suite's goldens.
    ///
    /// The numbers are chosen so `ceil` and `floor` DISAGREE: `s·Δt/h`
    /// is `2.5` and `3.0` respectively, never an integer by accident.
    #[test]
    fn the_chord_count_is_the_ceiling_of_the_speed_times_the_span_over_the_step() {
        // s = 5, Δt = 0.5, h = 1 → 2.5 → 3 chords.
        assert_eq!(ceil_count(5.0 * 0.5, 1.0), Ok(3));
        // exactly on the boundary: 3.0 → 3, not 4.
        assert_eq!(ceil_count(6.0 * 0.5, 1.0), Ok(3));
        // a sup below one step still buys one chord, never zero.
        assert_eq!(ceil_count(1e-9 * 0.5, 1.0), Ok(1));
    }

    fn wiggle() -> NurbsCurve3<f64> {
        let pts: Vec<Point3<f64>> = (0..7)
            .map(|i| {
                let t = f64::from(i) / 6.0;
                Point3::new(t, (3.0 * t * (1.0 - t)).powi(2), 0.3 * t.powi(2))
            })
            .collect();
        NurbsCurve3::interpolate(&pts, 3).unwrap()
    }

    /// The hull-bounded sagitta generalization: the chord count sizes
    /// the secant deviation under δ_s — verified against a dense
    /// per-segment sampling oracle.
    #[test]
    fn nurbs_chords_bound_the_secant_deviation() {
        let n = wiggle();
        let (d0, d1) = n.knots().domain();
        for delta_s in [1e-2, 1e-3, 1e-4] {
            let count =
                nurbs_chord_count(&n, d1 - d0, delta_s, EdgeKey::default()).expect("in inventory");
            assert!(count >= 1);
            #[allow(clippy::cast_precision_loss)]
            let h = (d1 - d0) / count as f64;
            let mut worst = 0.0f64;
            for seg in 0..count {
                #[allow(clippy::cast_precision_loss)]
                let a = d0 + h * seg as f64;
                let b = a + h;
                let (pa, pb) = (n.eval(a), n.eval(b));
                for k in 1..32 {
                    let t = a + h * f64::from(k) / 32.0;
                    let p = n.eval(t);
                    let lam = f64::from(k) / 32.0;
                    let chord = Point3::new(
                        pa.x + (pb.x - pa.x) * lam,
                        pa.y + (pb.y - pa.y) * lam,
                        pa.z + (pb.z - pa.z) * lam,
                    );
                    worst = worst.max((p - chord).norm());
                }
            }
            assert!(
                worst <= delta_s * 1.0000001,
                "secant deviation {worst} exceeds the certified budget {delta_s}"
            );
        }
    }

    /// Review probe (adopted): the SPIKE carrier — an interpolated
    /// cubic with one control point far off the line concentrates
    /// |C″| in one span, driving the measured secant deviation to
    /// 0.990–0.999 of the certified budget (the review's
    /// falsification sweep). The pin is two-sided: never OVER budget
    /// (soundness), and at least half of it (the fixture stays
    /// adversarial — a slack rewrite of the bound fails here too).
    #[test]
    fn adversarial_spike_stays_inside_but_near_the_budget() {
        let pts: Vec<Point3<f64>> = [
            (0.0, 0.0),
            (0.2, 0.01),
            (0.4, 0.02),
            (0.5, 0.9),
            (0.6, 0.02),
            (0.8, 0.01),
            (1.0, 0.0),
        ]
        .iter()
        .map(|&(x, y)| Point3::new(x, y, 0.0))
        .collect();
        let n = NurbsCurve3::interpolate(&pts, 3).unwrap();
        let (d0, d1) = n.knots().domain();
        for delta_s in [1e-2, 1e-3, 1e-4] {
            let count =
                nurbs_chord_count(&n, d1 - d0, delta_s, EdgeKey::default()).expect("in inventory");
            #[allow(clippy::cast_precision_loss)]
            let h = (d1 - d0) / count as f64;
            let mut worst = 0.0f64;
            for seg in 0..count {
                #[allow(clippy::cast_precision_loss)]
                let a = d0 + h * seg as f64;
                let b = a + h;
                let (pa, pb) = (n.eval(a), n.eval(b));
                for k in 1..64 {
                    let t = a + h * f64::from(k) / 64.0;
                    let p = n.eval(t);
                    let lam = f64::from(k) / 64.0;
                    let chord = Point3::new(
                        pa.x + (pb.x - pa.x) * lam,
                        pa.y + (pb.y - pa.y) * lam,
                        pa.z + (pb.z - pa.z) * lam,
                    );
                    worst = worst.max((p - chord).norm());
                }
            }
            assert!(
                worst <= delta_s * 1.0000001,
                "spike deviation {worst} exceeds the certified budget {delta_s}"
            );
            assert!(
                worst >= delta_s * 0.5,
                "spike deviation {worst} fell below half the budget {delta_s} — the \
                 adversarial fixture went slack (review measured 0.990-0.999)"
            );
        }
    }

    /// CONSCIOUS FLIP (M8-5): `rational_nurbs_carrier_refuses_typed`
    /// re-derived as the positive row. Rational carriers are metered
    /// now (the quotient-rule `sup|C″|` of
    /// [`rational_carrier_m_bound`]), and the pin is the bound's
    /// honesty against the TRUTH: on adversarial rational carriers the
    /// dense-sampled secant deviation stays inside the certified
    /// budget on every segment, and the sampled sup of `‖C″‖` itself
    /// is dominated by a real (> 0) bound.
    #[test]
    fn rational_carrier_chords_bound_the_secant_deviation() {
        // The exact unit quarter circle (the arc-walled seam class)
        // and a steep-weight wiggle (weights alternating 0.4–3.0 on
        // the interpolated cubic's control).
        let w = core::f64::consts::FRAC_1_SQRT_2;
        let arc = NurbsCurve3::new(
            KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap(),
            vec![
                Point3::new(1.0, 0.0, 0.0),
                Point3::new(1.0, 1.0, 0.0),
                Point3::new(0.0, 1.0, 0.0),
            ],
            vec![1.0, w, 1.0],
        )
        .unwrap();
        let base = wiggle();
        let steep: Vec<f64> = (0..base.control().len())
            .map(|i| match i % 4 {
                0 => 0.4,
                1 => 3.0,
                2 => 1.0,
                _ => 0.6,
            })
            .collect();
        let wiggle_r =
            NurbsCurve3::new(base.knots().clone(), base.control().to_vec(), steep).unwrap();
        for (name, n) in [("quarter_arc", arc), ("steep_wiggle", wiggle_r)] {
            let (d0, d1) = n.knots().domain();
            for delta_s in [1e-2, 1e-3, 1e-4] {
                let count = nurbs_chord_count(&n, d1 - d0, delta_s, EdgeKey::default())
                    .expect("rational carriers are in the chord inventory now");
                assert!(count >= 1, "{name}: a real count, not a fabricated zero");
                #[allow(clippy::cast_precision_loss)]
                let h = (d1 - d0) / count as f64;
                let mut worst = 0.0f64;
                for seg in 0..count {
                    #[allow(clippy::cast_precision_loss)]
                    let a = d0 + h * seg as f64;
                    let b = a + h;
                    let (pa, pb) = (n.eval(a), n.eval(b));
                    for k in 1..64 {
                        let t = a + h * f64::from(k) / 64.0;
                        let p = n.eval(t);
                        let lam = f64::from(k) / 64.0;
                        let chord = Point3::new(
                            pa.x + (pb.x - pa.x) * lam,
                            pa.y + (pb.y - pa.y) * lam,
                            pa.z + (pb.z - pa.z) * lam,
                        );
                        worst = worst.max((p - chord).norm());
                    }
                }
                assert!(
                    worst <= delta_s * 1.0000001,
                    "{name}: secant deviation {worst} exceeds the certified budget \
                     {delta_s} over {count} chords"
                );
            }
        }
    }

    /// The refusal row the flip keeps: an ILLEGAL rational carrier
    /// (non-positive weight) cannot even be described —
    /// `NurbsCurve3::new` refuses at the door, so
    /// [`nurbs_chord_count`]'s own licence check is a defensive
    /// backstop rather than a reachable lane.
    #[test]
    fn illegal_rational_carrier_weight_refuses_at_the_door() {
        let n = wiggle();
        for bad in [0.0, -0.5, f64::NAN] {
            let mut weights = vec![1.0; n.control().len()];
            weights[2] = bad;
            assert!(
                NurbsCurve3::new(n.knots().clone(), n.control().to_vec(), weights).is_err(),
                "weight {bad} must refuse at construction"
            );
        }
    }

    // ------------------------------------------------------------------
    // R1 REVIEW PROBES (M8-5, PR #322): adversarial rational carriers
    // beyond the PR's — extreme/near-zero weights and the C¹
    // multiplicity edge — checked BOTH ways: the m-bound dominates the
    // dense-sampled true sup‖C″‖ (via deriv2, plus the chord counts
    // keep the measured secant deviation inside δ_s).
    // ------------------------------------------------------------------

    fn r1_secant_worst(n: &NurbsCurve3<f64>, count: usize) -> f64 {
        let (d0, d1) = n.knots().domain();
        #[allow(clippy::cast_precision_loss)]
        let h = (d1 - d0) / count as f64;
        let mut worst = 0.0f64;
        for seg in 0..count {
            #[allow(clippy::cast_precision_loss)]
            let a = d0 + h * seg as f64;
            let b = a + h;
            let (pa, pb) = (n.eval(a), n.eval(b));
            for k in 1..128 {
                let t = a + h * f64::from(k) / 128.0;
                let p = n.eval(t);
                let lam = f64::from(k) / 128.0;
                let chord = Point3::new(
                    pa.x + (pb.x - pa.x) * lam,
                    pa.y + (pb.y - pa.y) * lam,
                    pa.z + (pb.z - pa.z) * lam,
                );
                worst = worst.max((p - chord).norm());
            }
        }
        worst
    }

    /// The δ schedule the R1 carriers sweep. Per-fixture, because the
    /// count scales as δ^(−½) and one fixture's bound is conservative
    /// enough to make the finest row dominate the crate — see
    /// [`r1_extreme_weight_carrier`].
    const R1_DELTAS: [f64; 3] = [1e-2, 1e-3, 1e-4];

    fn r1_check(name: &str, n: &NurbsCurve3<f64>, deltas: &[f64]) {
        // (a) m-bound vs dense-sampled true sup|C''|. δ-free: every
        // carrier gets this arm in full.
        let m = rational_carrier_m_bound(n, EdgeKey::default()).expect("in inventory");
        let (d0, d1) = n.knots().domain();
        let mut truth = 0.0f64;
        for k in 0..=4000 {
            let t = d0 + (d1 - d0) * f64::from(k) / 4000.0;
            truth = truth.max(n.deriv2(t).norm());
        }
        let d = Domination::sampled_under_certified(&[("sup|C''|", truth, m)]);
        assert!(d.holds(), "{name}: {d}");
        println!("{name}: truth/bound = {:.4}", truth / m);
        // (b) chord counts keep the secant inside delta_s.
        for &delta_s in deltas {
            let count =
                nurbs_chord_count(n, d1 - d0, delta_s, EdgeKey::default()).expect("in inventory");
            let worst = r1_secant_worst(n, count);
            assert!(
                worst <= delta_s * 1.0000001,
                "{name}: secant {worst} exceeds {delta_s} over {count} chords"
            );
        }
    }

    /// Extreme weights (1e-2 .. 1e2) on a multi-span cubic.
    ///
    /// SHORT δ SCHEDULE (no 1e-4), and the reasoning is the one this
    /// tree has already ratified twice on the sibling SURFACE arm:
    /// `nurbs_cert.rs:1106-1111` ("the lattice's falsification power is
    /// PER TRIANGLE, not δ-dependent, so it takes coarser deltas") and
    /// `nurbs_cert.rs:1513-1516` ("the per-triangle claim d ≤ cert(uv)
    /// is grid-independent, so a coarser grid still falsifies"). The
    /// claim asserted in `r1_check`'s arm (b) is likewise PER SEGMENT —
    /// `worst <= delta_s` on every one of `count` segments — so a finer
    /// δ only multiplies how many segments are checked; it adds no
    /// falsification power per segment. What it does add is cost: the
    /// count scales as δ^(−½), and THIS fixture's bound is the extreme
    /// case (measured `truth/bound = 0.0023`, i.e. ~435x conservative,
    /// exactly the conservatism `nurbs_cert` capped its grid for), so
    /// its 1e-4 row alone was ~70% of a test that was in turn 98% of
    /// the whole `chords` module (12.55 s of 12.77 s, measured).
    ///
    /// WHAT IS LOST: the δ = 1e-4 schedule on THIS fixture only. The
    /// finest row stays exercised, on every other carrier in the
    /// module: `r1_near_zero_weight_carrier` and
    /// `r1_rational_mult_p_minus_one_carrier` (both still pass the full
    /// `R1_DELTAS` through this same helper, and cost 0.05 s / 0.08 s
    /// doing it), plus `rational_carrier_chords_bound_the_secant_
    /// deviation`'s two rational fixtures and the polynomial
    /// `nurbs_chords_bound_the_secant_deviation` /
    /// `adversarial_spike_stays_inside_but_near_the_budget`. Arm (a),
    /// the bound-honesty claim that is this fixture's actual point, is
    /// δ-free and untouched.
    #[test]
    fn r1_extreme_weight_carrier() {
        let base = wiggle();
        let w: Vec<f64> = (0..base.control().len())
            .map(|i| [1e-2, 1.0, 1e2, 0.3, 7.0][i % 5])
            .collect();
        let n = NurbsCurve3::new(base.knots().clone(), base.control().to_vec(), w).unwrap();
        r1_check("extreme_weight_carrier", &n, &R1_DELTAS[..2]);
    }

    /// Near-zero-touching legal weight (1e-5) amid O(1).
    #[test]
    fn r1_near_zero_weight_carrier() {
        let base = wiggle();
        let mut w = vec![1.0; base.control().len()];
        w[3] = 1e-5;
        let n = NurbsCurve3::new(base.knots().clone(), base.control().to_vec(), w).unwrap();
        r1_check("near_zero_weight_carrier", &n, &R1_DELTAS);
    }

    /// Interior multiplicity EXACTLY p−1 (the C¹ edge) on a RATIONAL
    /// cubic — |C''| jumps at the double knot.
    #[test]
    fn r1_rational_mult_p_minus_one_carrier() {
        let kv =
            KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
        let pts: Vec<Point3<f64>> = (0..kv.control_count())
            .map(|i| {
                let t = i as f64 / 5.0;
                Point3::new(t, (2.5 * t).sin(), 0.4 * t.powi(2))
            })
            .collect();
        let w: Vec<f64> = (0..pts.len()).map(|i| [0.3, 2.0, 0.9][i % 3]).collect();
        let n = NurbsCurve3::new(kv, pts, w).unwrap();
        r1_check("rational_mult_p1_carrier", &n, &R1_DELTAS);
    }

    /// Builds a rational cubic whose single interior knot has the given
    /// multiplicity.
    fn mult_cubic(multiplicity: usize) -> NurbsCurve3<f64> {
        let mut knots = vec![0.0; 4];
        knots.extend(core::iter::repeat_n(0.5, multiplicity));
        knots.extend(core::iter::repeat_n(1.0, 4));
        let kv = KnotVector::clamped(knots, 3).expect("clamped cubic");
        #[allow(clippy::cast_precision_loss)]
        let pts: Vec<Point3<f64>> = (0..kv.control_count())
            .map(|i| {
                let t = i as f64 / 6.0;
                Point3::new(t, (3.0 * t).cos(), 0.7 * t)
            })
            .collect();
        let w: Vec<f64> = (0..pts.len()).map(|i| [1.4, 0.5, 2.2][i % 3]).collect();
        NurbsCurve3::new(kv, pts, w).expect("legal rational cubic")
    }

    /// The row that pins the deletion of the two runtime refusals
    /// [`rational_carrier_m_bound`]'s span loop used to carry ("NURBS
    /// span below its degree", "NURBS span beyond its control net").
    /// Drawing the window from a `Span` makes both unrepresentable, so
    /// the loop head's ONLY remaining exit is the empty-span skip —
    /// and an interior knot of multiplicity ≥ 2 is exactly what
    /// produces one (`knots[s] == knots[s+1]` for an `s` inside
    /// `[first_span, last_span]`).
    ///
    /// The `any(span(s).is_none())` assertion is the anti-slack guard:
    /// it fails loudly if refinement ever stops presenting an empty
    /// span, which would silently make this row cover nothing.
    ///
    /// Multiplicity `p` itself is checked too, and it is a REFUSAL, not
    /// a bound: a C⁰ kink leaves the certified inventory at the
    /// degree/kink gate, well before the span loop. That exit is
    /// `UnsupportedCurve` and always was — the deleted `MissingEntity`
    /// refusals never guarded it.
    #[test]
    fn empty_spans_survive_the_deleted_window_guards() {
        // p − 1 = 2: inside the inventory, and it presents an empty span.
        let n = mult_cubic(2);
        let refined = n
            .refine_knots(&geom_brep::patch_bound::rational_split_points(n.knots()))
            .expect("refinement materialises");
        let rkv = refined.knots();
        assert!(
            (rkv.first_span()..=rkv.last_span()).any(|s| rkv.span(s).is_none()),
            "the multiplicity-2 fixture must still present an empty span after \
             refinement — otherwise this row stops covering the skip"
        );
        // The full δ schedule, as the other cheap carriers take: this
        // fixture is `rational_mult_p1_carrier`'s sibling in cost, not
        // `extreme_weight`'s, so there is nothing here to trim.
        r1_check("rational_mult_2_carrier", &n, &R1_DELTAS);

        // p = 3: refused, typed, and NOT by either deleted message.
        let n = mult_cubic(3);
        let (d0, d1) = n.knots().domain();
        match nurbs_chord_count(&n, d1 - d0, 1e-3, EdgeKey::default()) {
            Err(TessellateError::UnsupportedCurve { note, .. }) => assert!(
                note.contains("C⁰ kink"),
                "multiplicity-p refusal should name the C⁰ kink, got {note:?}"
            ),
            other => panic!("multiplicity p must leave the inventory, got {other:?}"),
        }
    }
}
