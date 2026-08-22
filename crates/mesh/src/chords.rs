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
//!   steps ≤ its grid step h = √(δ_s/(3(R+2r))); a circle edge's
//!   carrier parameter *is* the torus chart coordinate along it
//!   (azimuth for rims, minor angle for meridians), so each adjacent
//!   torus face adds n ≥ ceil(Δt/h).
//! - Adjacent-NURBS tightening (M7, the trimmed-NURBS lane): the same
//!   shape with a hull-derived Hessian — a described NURBS face
//!   certifies through `crate::nurbs_cert`'s anisotropic bound, which
//!   needs boundary UV steps within the face's own (h_u, h_v); the
//!   half-edge's stored CLOSED-FORM pcurve (`IsoLine`/`Harmonic`)
//!   gives per-axis UV speed bounds (s_u, s_v) — exact `|pl|`
//!   components for the iso line, the amplitude sum for the harmonic
//!   form — so each adjacent NURBS face adds n ≥ ⌈s_u·Δt/h_u⌉ and
//!   ⌈s_v·Δt/h_v⌉, on EVERY carrier kind (a straight wall edge is one
//!   3-D chord but many UV steps). A `Fitted` image has no certified
//!   speed bound here and refuses typed (the trimmed lane's module
//!   docs name its consumer).
//!
//! These tightenings are the only places adjacent surfaces enter
//! chord counts — chord points remain a pure function of (carrier +
//! interval, endpoint points, adjacent surface parameters, δ).
//!
//! Polyline endpoints are the topology vertices' points **bitwise**
//! (never `carrier(t₀)`, which is only within ε of them) so every
//! polyline meeting at a vertex shares its mesh vertex id; interior
//! points are `carrier(t₀ + (t₁−t₀)·i/n)` in `he_plus`-forward order.

use std::collections::HashMap;

use geom::Curve3;
use geom::Surface;
use geom_brep::Pcurve;
use geom_core::ring_interval::RingInterval;
use geom_core::spline::KnotVector;
use geom_core::spline::hull::derivative_coeffs;
use topo::{Body, EdgeKey};

use crate::nurbs_cert::{FaceBounds, face_bound};
use crate::sizing::{ceil_count, curvature_step, ellipse_step, sagitta_step, torus_step};
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
                    let face = body
                        .get_face(fk)
                        .ok_or(TessellateError::MissingEntity { what: "face" })?;
                    let surface =
                        body.get_surface(face.surface)
                            .ok_or(TessellateError::MissingEntity {
                                what: "face surface",
                            })?;
                    if let Some(h) = torus_step(surface, delta_s) {
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
        let n = nurbs_tighten(body, ek, span, delta_s, bounds, n)?;
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
        let mut sum_sq = RingInterval::zero();
        for comp in 0..3 {
            let coeffs: Vec<RingInterval> = n
                .control()
                .iter()
                .map(|pt| {
                    RingInterval::point(match comp {
                        0 => pt.x,
                        1 => pt.y,
                        _ => pt.z,
                    })
                })
                .collect();
            let q1 = derivative_coeffs(kv, &coeffs);
            let inner = kv.knots()[1..kv.knots().len() - 1].to_vec();
            let Ok(kv1) = KnotVector::clamped(inner, p - 1) else {
                return Err(TessellateError::UnsupportedCurve {
                    edge: ek,
                    note: "B-spline carrier whose derivative knot vector fails to \
                           materialise — outside the certified chord inventory",
                });
            };
            let q2 = derivative_coeffs(&kv1, &q1);
            let mut hull = RingInterval::poison();
            for (k, q) in q2.iter().enumerate() {
                hull = if k == 0 {
                    *q
                } else {
                    RingInterval::hull(hull, *q)
                };
            }
            sum_sq = sum_sq + hull.sqr();
        }
        sum_sq.hi().sqrt().next_up()
    };
    if !m_bound.is_finite() {
        return Err(TessellateError::UnsupportedCurve {
            edge: ek,
            note: "B-spline carrier second-derivative hull is unbounded/poisoned — \
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
/// [`crate::nurbs_cert::RATIONAL_CERT_SPLITS`] refinement), each
/// ingredient is an active-window hull on the homogeneous nets:
/// `sup|C − c| ≤ max_active |P − c|` (positive weights — the licence
/// the caller checked — make the rational basis a nonnegative
/// partition of unity), `sup|Ã'|`/`sup|Ã″|`/`sup|w′|`/`sup|w″|` are
/// iterated [`derivative_coeffs`] hulls, and the divisor is the span's
/// weight range: for a SUP bound with a nonnegative numerator the
/// conservative division is by `w_min` (the mirror image of the speed
/// meter's lower-bound `w_max` choice — the interval division by
/// `[w_lo, w_hi]` computes exactly that, outward-rounded, and poisons
/// if positivity was never proven). Recentring at the span's control
/// centroid keeps the cross terms span-sized. The domain bound is the
/// max over spans (hull of the squared enclosures), `next_up` after
/// the final square root — poison flows to the caller's finite check.
fn rational_carrier_m_bound(
    n: &geom::NurbsCurve3<f64>,
    ek: EdgeKey,
) -> Result<f64, TessellateError> {
    let refined = n
        .refine_knots(&crate::nurbs_cert::rational_split_points(n.knots()))
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
    let inner = kv.knots()[1..kv.knots().len() - 1].to_vec();
    let Ok(kv1) = KnotVector::clamped(inner, p - 1) else {
        return Err(TessellateError::UnsupportedCurve {
            edge: ek,
            note: "B-spline carrier whose derivative knot vector fails to \
                   materialise — outside the certified chord inventory",
        });
    };
    // Homogeneous coefficient nets and their derivative enclosures.
    let w_pts: Vec<RingInterval> = refined
        .weights()
        .iter()
        .map(|w| RingInterval::point(*w))
        .collect();
    let dw = derivative_coeffs(kv, &w_pts);
    let ddw = derivative_coeffs(&kv1, &dw);
    let comp = |c: usize| -> Vec<RingInterval> {
        refined
            .control()
            .iter()
            .zip(refined.weights())
            .map(|(pt, w)| {
                RingInterval::point(*w)
                    * RingInterval::point(match c {
                        0 => pt.x,
                        1 => pt.y,
                        _ => pt.z,
                    })
            })
            .collect()
    };
    let a_nets: Vec<(Vec<RingInterval>, Vec<RingInterval>)> = (0..3)
        .map(|c| {
            let a = comp(c);
            let da = derivative_coeffs(kv, &a);
            let dda = derivative_coeffs(&kv1, &da);
            (da, dda)
        })
        .collect();
    // The signed hull of `net[i] − c·wnet[i]` over `[i0, i1]`
    // (out-of-range poisons; recentring commutes with differencing).
    let window = |net: &[RingInterval],
                  wnet: &[RingInterval],
                  c: RingInterval,
                  active: core::ops::RangeInclusive<usize>|
     -> RingInterval {
        let mut acc: Option<RingInterval> = None;
        for i in active {
            let e = match (net.get(i), wnet.get(i)) {
                (Some(&a), Some(&w)) => a - c * w,
                _ => RingInterval::poison(),
            };
            acc = Some(match acc {
                None => e,
                Some(h) => RingInterval::hull(h, e),
            });
        }
        acc.unwrap_or_else(RingInterval::poison)
    };
    let mag = |h: RingInterval| RingInterval::from_bounds(0.0, h.mag());
    let two = RingInterval::point(2.0);
    let mut sq_acc: Option<RingInterval> = None;
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
        let mut w_span: Option<RingInterval> = None;
        for w in &w_pts[span.window()] {
            w_span = Some(match w_span {
                None => *w,
                Some(h) => RingInterval::hull(h, *w),
            });
        }
        let w_span = w_span.unwrap_or_else(RingInterval::poison);
        let zero = RingInterval::zero();
        // Active windows: value [s−p, s]; each differencing drops the
        // top index, which is what `derived_window` names — so `s − 1`
        // and `s − 2` are not subtractions at the use site either. The
        // caller's degree gate makes p ≥ 2, so the order-2 window is
        // always `Some`; if that ever stopped holding the bound
        // POISONS (and the caller's finite check refuses) rather than
        // underflowing.
        // `p ≥ 2` (the caller's degree gate), so the order-2 window is
        // `Some` on every reachable path. It is asserted rather than
        // merely commented: `debug_assert` is the tree's fail-loud form
        // for a state that cannot occur — the panic family is denied in
        // kernel code (workspace lints), so the release build still
        // takes the total route below and POISONS, which refuses the
        // bound instead of quietly under-reporting it.
        let d2 = span.derived_window(2);
        debug_assert!(
            d2.is_some(),
            "the degree gate promised p ≥ 2 for this carrier, got {p}"
        );
        let w1 = mag(window(&dw, &dw, zero, span.first_derived_window()));
        let w2 = mag(d2
            .clone()
            .map_or_else(RingInterval::poison, |a| window(&ddw, &ddw, zero, a)));
        let mut sq = RingInterval::zero();
        for (c, (da, dda)) in a_nets.iter().enumerate() {
            let cc = RingInterval::point(cen[c]);
            let mut v0h: Option<RingInterval> = None;
            for pt in &refined.control()[span.window()] {
                let e = RingInterval::point(match c {
                    0 => pt.x,
                    1 => pt.y,
                    _ => pt.z,
                }) - cc;
                v0h = Some(match v0h {
                    None => e,
                    Some(h) => RingInterval::hull(h, e),
                });
            }
            let v0 = mag(v0h.unwrap_or_else(RingInterval::poison));
            let a1 = mag(window(da, &dw, cc, span.first_derived_window()));
            let a2 = mag(d2
                .clone()
                .map_or_else(RingInterval::poison, |a| window(dda, &ddw, cc, a)));
            let s1 = (a1 + v0 * w1) / w_span;
            let s2 = (a2 + two * s1 * w1 + v0 * w2) / w_span;
            sq = sq + s2.sqr();
        }
        sq_acc = Some(match sq_acc {
            None => sq,
            Some(h) => RingInterval::hull(h, sq),
        });
    }
    Ok(sq_acc.map_or(f64::NAN, |s| s.hi().sqrt().next_up()))
}

/// The adjacent-NURBS chord tightening (module docs): for each
/// adjacent described NURBS face, raise `n` until the edge's UV image
/// steps fit inside the face's certificate-budget grid steps. The
/// bound comes from the tessellation's shared [`FaceBounds`] memo, so
/// the Hessian hull is assembled once per face for the whole run.
fn nurbs_tighten(
    body: &Body<f64>,
    ek: EdgeKey,
    span: f64,
    delta_s: f64,
    bounds: &mut FaceBounds,
    mut n: usize,
) -> Result<usize, TessellateError> {
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
        let face = body
            .get_face(fk)
            .ok_or(TessellateError::MissingEntity { what: "face" })?;
        let surface = body
            .get_surface(face.surface)
            .ok_or(TessellateError::MissingEntity {
                what: "face surface",
            })?;
        let Surface::Nurbs(ref payload) = *surface else {
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
        // harmonic form (|P′| = |−pa·sin t + pb·cos t + pl|).
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
        };
        n = n
            .max(ceil_count(su * span, hu)?)
            .max(ceil_count(sv * span, hv)?);
    }
    Ok(n)
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
    use geom::NurbsCurve3;
    use geom_core::Point3;
    use topo::EdgeKey;

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

    /// The POISON row the flip keeps: an ILLEGAL rational carrier
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
        assert!(
            truth <= m,
            "{name}: sampled sup|C''| {truth:.6e} escapes the certified {m:.6e}"
        );
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
    #[cfg_attr(
        not(nightly_suite),
        ignore = "nightly-only: 3.2 s -- an extreme-weight carrier through the chord pass; never red in \
     the repository's entire life. mesh builds at opt-level 2 in both CI lanes (the \
     [profile.dev.package.mesh] stanza), so this row is expensive in wall clock at every \
     opt level, not only at opt-2."
    )]
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
            .refine_knots(&crate::nurbs_cert::rational_split_points(n.knots()))
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
