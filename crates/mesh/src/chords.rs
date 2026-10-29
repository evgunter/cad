//! Per-edge chord points: computed **once** from each edge's certified
//! carrier and consumed by both adjacent faces (the watertightness
//! half of the pure-function invariant, crate docs).
//!
//! Chord counts are deterministic ceil arithmetic from δ: a chord's
//! worst point lands at δ from its carrier (the edge-chord theorem at
//! `sizing::sagitta_step`), and each adjacent face whose lane needs
//! more raises the count through one of the tightenings below.
//!
//! - Line carriers: 1 chord (a segment of the exact locus).
//! - Circle carriers (radius ρ, forward span Δt): per-chord step
//!   φ = 2·acos(1 − δ/ρ) (the closed-form sagitta bound
//!   ρ(1 − cos(φ/2)) ≤ δ), capped at π/4 (`sizing::MAX_ANGULAR_STEP`);
//!   n = ceil(Δt/φ).
//! - Adjacent-grid tightening: a torus, sphere or cone face's
//!   worst-triangle theorem is about its grid's cells, so a rim chord
//!   point the grid did not place would make triangles the theorem
//!   does not cover. A circle edge's carrier parameter *is* the chart
//!   coordinate along it (azimuth for rims, minor angle or latitude for
//!   meridians), so each such adjacent face adds n ≥ ceil(Δt/h) with
//!   `h` the face's grid step in the edge's own direction —
//!   `sizing::grid_boundary_step` names each surface's `h` and says
//!   what refuses.
//! - Unproved headroom: an edge of a trimmed cylinder or trimmed sphere
//!   face (the two arms without their theorem,
//!   `trimmed::UNPROVED_HEADROOM`) is chorded at that headroom's share
//!   of δ, so the arm's trim polygon is the one its lattice was sized
//!   beside.
//! - Adjacent-sphere-arm tightening: a trimmed sphere face's lattice
//!   runs in the chart, which its tilted circles cross, so each such
//!   face adds `n ≥ ⌈s_u·Δt/h_u⌉` and `⌈s_v·Δt/h_v⌉` from closed-form
//!   speed bounds of the half-edge's image and the arm's lattice steps
//!   ([`sphere_arm_tighten`]).
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
//! - Crease joints: an arc rim of `m ≥ 2` sub-arcs meets its face's C⁰
//!   creases at the sub-arc joints, and the trimmed lane cuts its
//!   triangles along those creases, so the count is rounded up to a
//!   multiple of `m`: point `q·k` of `q·m` then sits at the joint's own
//!   parameter, `t₀ + Δt·geom_brep::sub_arc_joint(k, m)`, the expression
//!   `sweep::skin::segment_curve` places the joint knot with.
//!
//! An adjacent face reaches a chord count only through the
//! tightenings above (not the crease joints, which round a count
//! already tightened): the `Circle` arm's
//! `sizing::grid_boundary_step`, the headroom target ([`edge_target`]),
//! [`nurbs_tighten`] and [`sphere_arm_tighten`]. Each reads the face
//! through `tessellate::face_surface`; the `Circle` arm and the
//! headroom target read the dispatch itself (`tessellate::lane_of`),
//! `sphere_arm_tighten` reads `trimmed::takes_sphere_arm`, the predicate
//! `lane_of` routes a sphere face by, and `nurbs_tighten` reads the
//! surface's spline chart, so a face tightens an edge only for the lane
//! that will mesh it. **This list is not enforced**: nothing in the
//! tree checks that these are the only four, and a fifth caller
//! compiles green as a fifth way for a neighbour to enter the count.
//!
//! With that door held, chord points are a pure function of (carrier +
//! interval, endpoint points, adjacent surface parameters, an adjacent
//! cone face's largest rim radius, whether an adjacent cylinder or
//! sphere face is trimmed, an adjacent trimmed sphere face's latitude
//! window and the half-edge's image on it, δ).
//!
//! Polyline endpoints are the topology vertices' points **bitwise**
//! (never `carrier(t₀)`, which is only within ε of them) so every
//! polyline meeting at a vertex shares its mesh vertex id; interior
//! points are `carrier(t₀ + (t₁−t₀)·i/n)` in `he_plus`-forward order.

use std::collections::HashMap;

use geom::Curve3;
use geom_brep::Pcurve;
use geom_core::Bounds;
use geom_core::interval::Interval;
use geom_core::interval::certification::Certification;
use geom_core::spline::SplineCoeffs;
use topo::{Body, EdgeKey};

use crate::nurbs_cert::{FaceBounds, face_bound};
use crate::sizing::{
    ceil_count, curvature_step, ellipse_step, grid_boundary_step, sagitta_step, spiric_step,
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
    delta: f64,
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
        let target = edge_target(body, ek, delta)?;
        let n = match *curve.carrier() {
            Curve3::Line { .. } => 1,
            Curve3::Circle { .. } => {
                let mut n = ceil_count(span, sagitta_step(target, circle_radius(curve.carrier())))?;
                for fk in adjacent_faces(body, ek)? {
                    if let Some(h) = grid_boundary_step(body, fk, curve, ek, delta)? {
                        n = n.max(ceil_count(span, h)?);
                    }
                }
                n
            }
            // Ellipse arcs (curved-cut boundaries, M5 PR 5): the
            // certified-conservative curvature-bound step; no torus
            // tightening applies (an ellipse never lies on a torus
            // chart of this kernel's constructions).
            Curve3::Ellipse { semi_u, semi_v, .. } => {
                ceil_count(span, ellipse_step(target, semi_u, semi_v))?
            }
            // Spiric arcs (the hollowed partial revolve's torus rim):
            // the curvature-bound step from the closed-form `sup|C″|`
            // (`spiric_step`). No torus tightening: `grid_boundary_step`
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
                spiric_step(target, major_radius, minor_radius, offset),
            )?,
            // B-spline carriers (rung-3 edges at rest, M5 PR 9):
            // the hull-bounded sagitta generalization (C9/PR 11) —
            // secant deviation on a parameter step h is ≤ h²·sup|C″|/8
            // (Taylor with integral remainder; needs C¹, i.e. interior
            // multiplicities ≤ p − 1), and sup|C″| is a control-
            // coefficient convexity fact via iterated derivative
            // hulls — on the CONTROL NET directly for an integral
            // carrier, through the second-order quotient statement
            // for a RATIONAL one (M8-5; a
            // rational-walled body's seam edges read back rational,
            // so this gate is what makes the face bound reachable).
            Curve3::Nurbs(ref n) => nurbs_chord_count(n, span, target, ek)?,
        };
        // Adjacent-NURBS tightening (module docs), on every carrier
        // kind — a straight wall edge is one 3-D chord but many UV
        // steps of the wall's certificate budget.
        let n = nurbs_tighten(
            body,
            ek,
            (t0, t1),
            crate::nurbs_cert::lattice_budget(delta),
            bounds,
            n,
        )?;
        let n = sphere_arm_tighten(body, ek, (t0, t1), delta, n)?;
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
            let t = t0 + span * geom_brep::sub_arc_joint(i, n);
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
/// `|C″| ≤ √(Σ sup²)`, and the per-step bound `h²·M/8 ≤ δ` sizes
/// `h`. Integral carriers take the direct control-hull arm; RATIONAL
/// carriers (M8-5) take [`rational_carrier_m_bound`] — the surface
/// bound's second-order quotient statement one dimension down. (The former
/// blanket rational refusal claimed "arc-bearing profiles refuse at
/// the rational-wall gate" — false since M8-2's rational span meter:
/// arc-walled bodies BUILD now, and their seam edges read back
/// rational, which is exactly why this arm exists.)
fn nurbs_chord_count(
    n: &geom::NurbsCurve3<f64>,
    span: f64,
    target: f64,
    ek: EdgeKey,
) -> Result<usize, TessellateError> {
    // Every weight is strictly positive and finite: the door's own
    // condition, and the convex-combination licence each hull fact below
    // reads. Which arm bounds `C″` is the second-derivative door's.
    let kv = n.knots_f64_interim();
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
    // The non-rational hull is `geom`'s, so the export lane's node-count
    // schedule and this chord schedule read one spelling of the iterated
    // difference-coefficient bound; the door names which structure
    // denied a bound, and a rational carrier goes to the second-order
    // quotient statement below.
    let refuse = |note| TessellateError::UnsupportedCurve { edge: ek, note };
    let m_bound = match geom::nonrational_second_derivative_sup(n) {
        Ok(bound) => bound,
        Err(geom::SecondDerivativeUnbounded::Rational) => rational_carrier_m_bound(n, ek)?,
        Err(geom::SecondDerivativeUnbounded::DegreeBelowTwo) => {
            return Err(refuse(
                "B-spline carrier of degree below 2 — the hull sagitta bound \
                 needs a second derivative to hull",
            ));
        }
        Err(geom::SecondDerivativeUnbounded::DerivativeKnotVector) => {
            return Err(refuse(
                "B-spline carrier whose derivative knot vector fails to \
                 materialise — outside the certified chord inventory",
            ));
        }
        Err(geom::SecondDerivativeUnbounded::PoisonedHull) => {
            return Err(refuse(
                "B-spline carrier second-derivative hull is \
                 unbounded/refused — outside the certified chord inventory",
            ));
        }
    };
    if m_bound == 0.0 {
        return Ok(1);
    }
    ceil_count(span, curvature_step(target, m_bound))
}

/// Certified `sup‖C″‖` for a RATIONAL B-spline carrier (M8-5): the
/// second-order quotient statement
/// ([`geom::NurbsCurve3::accel_upper_bound`]; the derivation is
/// `geom_core::spline::derivative`'s "The second order"), read in
/// certification arithmetic off the described carrier, every value a
/// point, so the bound's upper end is certified. `C″ = M/W³` on each
/// dyadic piece of each span (halved from `M` and `W³` formed once over
/// the span), and the bound is the largest
/// `‖m_k‖/b_k` over them. A refusal (a degree above the statement's
/// ceiling, a poisoned piece) refuses typed.
///
/// # Errors
///
/// [`TessellateError::UnsupportedCurve`] — a degree above 18 (`C″`'s
/// quotient sits at `3p`, past [`geom_core::spline::QUOTIENT_MAX_DEGREE`]),
/// or a bound that does not certify: a weight spread wide enough at a
/// high enough degree lets the weight power's coefficient enclosures
/// reach zero (`geom_core::spline::derivative`'s module docs, "The
/// practical ceiling").
fn rational_carrier_m_bound(
    n: &geom::NurbsCurve3<f64>,
    ek: EdgeKey,
) -> Result<f64, TessellateError> {
    if 3 * n.knots_f64_interim().degree() > geom_core::spline::QUOTIENT_MAX_DEGREE {
        return Err(TessellateError::UnsupportedCurve {
            edge: ek,
            note: "rational B-spline carrier of degree above 18, past the degree its \
                   second-derivative bound reaches (three times the degree, at most 56); \
                   describe the edge at degree 18 or below",
        });
    }
    let bound = n.map_scalar(Interval::point).accel_upper_bound();
    if bound.is_certified() && bound.hi().is_finite() {
        Ok(bound.hi())
    } else {
        Err(TessellateError::UnsupportedCurve {
            edge: ek,
            note: "rational B-spline carrier whose second-derivative bound does not certify: \
                   at high degree a wide weight spread lets the weight power reach zero \
                   within its rounding — outside the certified chord inventory",
        })
    }
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
    budget: f64,
    bounds: &mut FaceBounds,
    n: usize,
) -> Result<usize, TessellateError> {
    let (t0, t1) = params;
    let span = t1 - t0;
    // The sub-arc count every adjacent arc rim needs the schedule to
    // be a multiple of (the `IsoArc` arm below).
    let mut joints = 1;
    let n = tighten_by_sides(body, ek, span, n, |hek, fk, surface| {
        // The UV step schedule is a statement about the chart, so both
        // spline kinds take it — an approximating surface's chart is
        // its fit's.
        let Some(payload) = surface.spline_chart() else {
            return Ok(None);
        };
        if payload.is_placeholder() {
            return Err(TessellateError::UnsupportedSurface { face: fk });
        }
        let (hu, hv) = face_bound(bounds, payload, fk)?.grid_steps(budget);
        let Some(cache) = body.pcurve(hek) else {
            return Err(TessellateError::UnsupportedCurve {
                edge: ek,
                note: "NURBS-face half-edge carries no stored pcurve cache — caches \
                       mint at loft/sweep assembly and STEP adoption; without one \
                       the chord schedule has no certified UV step bound",
            });
        };
        // Per-axis UV speed bounds (module docs): exact for the iso
        // line, the supremum `√(pa² + pb²) + |pl|` per axis for the
        // harmonic form ([`harmonic_uv_speeds`]), the derivative net's
        // hull for the general spline image.
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
                pd,
                t0: arc_t0,
                angle,
                breaks,
                ..
            } => {
                let spans = geom_brep::iso_arc_spans(breaks);
                if spans == 0 {
                    return Err(TessellateError::UnsupportedCurve {
                        edge: ek,
                        note: "an arc-rim pcurve with no sub-arc structure — a malformed \
                               cache, not a chord-schedule question",
                    });
                }
                // THE CREASE CHORD POINT. Two sub-arcs meet at a knot
                // of multiplicity two, a C⁰ crease of the chart in `u`,
                // and the trimmed lane needs a boundary point exactly
                // there to cut its triangles at it. A schedule of
                // `q·spans` points puts point `q·k` at
                // `t0 + span·sub_arc_joint(k, spans)`, the joint's own
                // parameter, so the count is rounded up to a multiple
                // below. That needs the rim's turn to be this edge's
                // interval, bit for bit.
                if spans > 1 {
                    if *arc_t0 != t0 || *angle != span {
                        return Err(TessellateError::UnsupportedCurve {
                            edge: ek,
                            note: "an arc-rim pcurve whose turn is not its edge's parameter \
                                   interval, so no chord point lands on its sub-arc joints \
                                   (the C⁰ creases of its NURBS face)",
                        });
                    }
                    joints = lcm(joints, spans);
                }
                #[allow(clippy::cast_precision_loss)]
                let m = spans as f64;
                let rate = 1.0 / (2.0 * m * (angle / (2.0 * m)).sin());
                (pd.x.abs() * rate, pd.y.abs() * rate)
            }
            Pcurve::Harmonic { u, v } => harmonic_uv_speeds(*u, *v),
            Pcurve::Fitted(_) => {
                return Err(TessellateError::UnsupportedCurve {
                    edge: ek,
                    note: "NURBS-face half-edge carries a FITTED pcurve — no \
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
            // Unreachable by construction, and refused rather than
            // given a bound: this loop runs only on SPLINE charts, and
            // a spiric image certifies on a plane or a torus chart and
            // nowhere else (`geom_brep::SpiricImage`). A cache that
            // reached here would be a corrupt one, not a frontier.
            Pcurve::Spiric { .. } => {
                return Err(TessellateError::UnsupportedCurve {
                    edge: ek,
                    note: "NURBS-face half-edge carries a SPIRIC pcurve — a spiric's \
                           chart images live on its own cutting plane and the two tori \
                           that hold it, so no spline chart mints one",
                });
            }
            // As the spiric: a focal-section image certifies on a cone
            // or torus chart only, so no spline chart mints one.
            Pcurve::FocalSection(_) => {
                return Err(TessellateError::UnsupportedCurve {
                    edge: ek,
                    note: "NURBS-face half-edge carries a FOCAL-SECTION pcurve — that \
                           image lives on a cone or torus chart, so no spline chart mints \
                           one",
                });
            }
            // As the focal section: a projected image is an analytic
            // chart's inverse of its carrier.
            Pcurve::Projected(_) => {
                return Err(TessellateError::UnsupportedCurve {
                    edge: ek,
                    note: "NURBS-face half-edge carries a PROJECTED pcurve — that image \
                           lives on an analytic chart, so no spline chart mints one",
                });
            }
        };
        Ok(Some(((hu, hv), (su, sv))))
    })?;
    Ok(n.div_ceil(joints) * joints)
}

/// The least common multiple of two sub-arc counts.
fn lcm(a: usize, b: usize) -> usize {
    let (mut x, mut y) = (a, b);
    while y != 0 {
        (x, y) = (y, x % y);
    }
    a / x * b
}

/// The per-axis UV speed sups `(s_u, s_v)` of a **general
/// curve-in-UV** chart image — what [`nurbs_tighten`] reads where the
/// `IsoLine` arm reads its exact `|pl|` components and the `Harmonic`
/// arm its supremum `√(pa² + pb²) + |pl|`. Those two are closed
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
/// typed ([`geom::NurbsCurve2::constant_weight`]).
///
/// **What this function does NOT check, because the type already
/// does.** A degree-0 image and an interior multiplicity above the
/// degree are both refused by [`KnotVector::clamped`](geom_core::KnotVector::clamped) at construction,
/// so no image carrying either can reach here. Multiplicity `= p` — a
/// C⁰ kink — IS constructible and IS admitted: the bound is a
/// Lipschitz statement, which C⁰ suffices for.
fn general_uv_speeds(
    image: &geom::NurbsCurve2<f64>,
    params: (f64, f64),
    ek: EdgeKey,
) -> Result<(f64, f64), TessellateError> {
    if image.constant_weight().is_none() {
        return Err(TessellateError::UnsupportedCurve {
            edge: ek,
            note: "NURBS-face half-edge carries a RATIONAL general curve-in-UV pcurve — \
                   the certified UV speed here is the polynomial net's difference hull, \
                   and a varying weight vector's derivative is not that net; no mint \
                   produces a rational chart image of this class",
        });
    }
    let kv = image.knots_f64_interim();
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

/// The deviation target an edge's own carrier is chorded at: δ, or
/// [`crate::trimmed::UNPROVED_HEADROOM`]·δ when either adjacent face is
/// a trimmed cylinder or a trimmed sphere — the arms whose theorem is
/// not yet written size their lattice at that headroom, and their trim
/// polygon is these chords.
fn edge_target(body: &Body<f64>, ek: EdgeKey, delta: f64) -> Result<f64, TessellateError> {
    for fk in adjacent_faces(body, ek)? {
        if crate::trimmed::sizes_at_headroom(body, fk)? {
            return Ok(delta * crate::trimmed::UNPROVED_HEADROOM);
        }
    }
    Ok(delta)
}

/// **Adjacent-sphere-arm tightening**: for each side whose face the
/// trimmed lane's sphere arm meshes, raise `n` until every chord's UV
/// step fits inside that face's lattice steps,
/// `n ≥ ⌈s_u·Δt/h_u⌉` and `⌈s_v·Δt/h_v⌉`, with `(h_u, h_v)` the arm's
/// own ([`crate::trimmed::sphere_lattice_steps`]) and `(s_u, s_v)` the
/// half-edge image's per-axis speed bounds ([`sphere_uv_speeds`]). A
/// chord's UV step is at most its speed bound times its parameter
/// step, so no boundary chord spans more than one lattice step in
/// either direction — the curved lanes' rule that rims are chorded at
/// the face's grid step, on a lattice the boundary does not run along.
fn sphere_arm_tighten(
    body: &Body<f64>,
    ek: EdgeKey,
    params: (f64, f64),
    delta: f64,
    n: usize,
) -> Result<usize, TessellateError> {
    let span = params.1 - params.0;
    tighten_by_sides(body, ek, span, n, |hek, fk, surface| {
        if !crate::trimmed::takes_sphere_arm(body, fk, surface)? {
            return Ok(None);
        }
        let (hu, hv) = crate::trimmed::sphere_lattice_steps(body, fk, surface, delta)?;
        let Some(cache) = body.pcurve(hek) else {
            return Err(TessellateError::UnsupportedCurve {
                edge: ek,
                note: "trimmed sphere face half-edge carries no stored pcurve cache — \
                       without one the chord schedule has no UV step bound",
            });
        };
        let (su, sv) = sphere_uv_speeds(cache.pcurve(), params, ek, fk)?;
        // An arc passing near a pole asks about `ρ_c·Δt/(r·d·h_u)`
        // chords at polar distance `d` (the schedule is uniform, so as
        // fine everywhere as at the closest approach). Past the arm's
        // per-edge work budget (`trimmed::MAX_SPHERE_EDGE_CHORDS`, whose
        // doc states it) it refuses typed, read on the raw ratios so that
        // an arc through or near the pole refuses by its own cause before
        // `ceil_count`'s `MAX_COUNT` (whose sentence is about δ) would.
        #[allow(clippy::cast_precision_loss)]
        let budget = crate::trimmed::MAX_SPHERE_EDGE_CHORDS as f64;
        if !(su * span / hu <= budget && sv * span / hv <= budget) {
            return Err(TessellateError::SphereTrimShape {
                face: fk,
                edge: ek,
                cause: crate::types::SphereTrimCause::ChordBudget,
            });
        }
        Ok(Some(((hu, hv), (su, sv))))
    })
}

/// **The side walk both UV tightenings share** ([`nurbs_tighten`],
/// [`sphere_arm_tighten`]): for each side of the edge, `side` reads the
/// face's lattice steps `(h_u, h_v)` and the half-edge image's per-axis
/// speed bounds `(s_u, s_v)` — `None` for a face the tightening is not
/// about — and the count rises to `⌈s·Δt/h⌉` on each axis.
fn tighten_by_sides(
    body: &Body<f64>,
    ek: EdgeKey,
    span: f64,
    mut n: usize,
    mut side: impl FnMut(
        topo::HalfEdgeKey,
        topo::FaceKey,
        &geom::Surface<f64>,
    ) -> Result<Option<((f64, f64), (f64, f64))>, TessellateError>,
) -> Result<usize, TessellateError> {
    let sides = sides_of(body, ek)?;
    for s in [sides.plus, sides.minus] {
        let surface = crate::tessellate::face_surface(body, s.face)?;
        if let Some(((hu, hv), (su, sv))) = side(s.half_edge, s.face, surface)? {
            n = n
                .max(ceil_count(su * span, hu)?)
                .max(ceil_count(sv * span, hv)?);
        }
    }
    Ok(n)
}

/// A harmonic chart image's per-axis speed bounds: with
/// `P′(t) = −pa·sin t + pb·cos t + pl`, each axis' supremum
/// `√(pa² + pb²) + |pl|`. The one spelling the NURBS and sphere
/// tightenings read.
fn harmonic_uv_speeds(u: geom_brep::Channel<f64>, v: geom_brep::Channel<f64>) -> (f64, f64) {
    let (_, pa, pb, pl) = geom_brep::harmonic_form_of(u, v);
    (pa.x.hypot(pb.x) + pl.x.abs(), pa.y.hypot(pb.y) + pl.y.abs())
}

/// Per-axis speed bounds `(sup|u′|, sup|v′|)` of a sphere chart image
/// over `[t₀, t₁]`, for the two forms a sphere face's circles carry.
///
/// - **Harmonic** (a parallel or a meridian):
///   `|P′| = |−pa·sin t + pb·cos t + pl|`, so per axis
///   `√(pa² + pb²) + |pl|`.
/// - **Projected circle** `p(t) = c + a·cos t + b·sin t` in the chart
///   frame (`z` on the axis), with `u = atan2(y, x)` and
///   `v = atan2(z, ρ)`, `ρ = √(x² + y²)`: `u′ = (x·y′ − y·x′)/ρ²` and
///   `v′ = p′·ê_v/|p|`, so `|u′| ≤ |p′|/ρ` and `|v′| ≤ |p′|/|p|`, on any
///   point, on the sphere or off it. `sup|p′|²` is the largest
///   eigenvalue of the Gram matrix of `(a, b)`; `|p|²` is
///   `K + 2(c·a)·cos t + 2(c·b)·sin t + ½(|a|² − |b|²)·cos 2t + (a·b)·sin 2t`
///   with `K = |c|² + ½(|a|² + |b|²)`, bounded below by `K` less its two
///   harmonics' amplitudes; and `ρ² = |p|² − z²` with `z`'s range over
///   the interval closed form ([`crate::trimmed::harmonic_range`]). On
///   an exact circle of the sphere `|p|` is the radius `r` and `ρ` is
///   `r·cos v`, so the bounds read `ρ_c/r` and `ρ_c/(r·cos v)`, and
///   the second is attained where the arc turns in latitude.
///
/// **How near a pole an arc may pass is decided by what its schedule
/// asks.** The azimuth bound grows as `1/ρ` toward a pole, and the
/// uniform schedule [`sphere_arm_tighten`] sizes from it is as fine
/// everywhere as at the arc's closest approach: `⌈s_u·Δt/h_u⌉` chords,
/// which [`ceil_count`] refuses typed (`ResolutionOverflow`) past
/// `sizing::MAX_COUNT`, the allocation cap every count in the crate is
/// held to. Only where the bound itself fails — `ρ² ≤ 0` on the arc's
/// closed-form lower bound, an arc through or numerically on the pole —
/// is there no count to ask, and that refuses
/// [`SphereTrimCause::ThroughPole`](crate::types::SphereTrimCause::ThroughPole).
///
/// The deck map moves an image by a translation and a sign, which no
/// speed sees. A spline carrier's projected image, or any other form,
/// refuses typed: the arm meshes circle-bounded faces.
pub(crate) fn sphere_uv_speeds(
    pcurve: &Pcurve<f64>,
    params: (f64, f64),
    ek: EdgeKey,
    fk: topo::FaceKey,
) -> Result<(f64, f64), TessellateError> {
    let refuse = |note| TessellateError::UnsupportedCurve { edge: ek, note };
    match pcurve {
        Pcurve::Harmonic { u, v } => Ok(harmonic_uv_speeds(*u, *v)),
        Pcurve::Projected(image) => {
            let geom_brep::FramedCarrier::Circle { centre, a, b, .. } = image.carrier() else {
                return Err(refuse(
                    "trimmed sphere face bounded by the projected image of a spline carrier — \
                     the sphere arm bounds the UV speed of a circle's image only",
                ));
            };
            projected_circle_speeds((*centre, *a, *b), params).ok_or(
                TessellateError::SphereTrimShape {
                    face: fk,
                    edge: ek,
                    cause: crate::types::SphereTrimCause::ThroughPole,
                },
            )
        }
        _ => Err(refuse(
            "trimmed sphere face half-edge carries an image that is neither harmonic nor a \
             projected circle — no sphere mint produces one",
        )),
    }
}

/// The per-axis speed bounds of a projected circle
/// `p(t) = c + a·cos t + b·sin t` in its chart's frame over `params`
/// ([`sphere_uv_speeds`]' doc), or `None` where the azimuth bound fails:
/// `ρ² ≤ 0` on its closed-form lower bound, an arc through or
/// numerically on the chart's pole.
pub(crate) fn projected_circle_speeds(
    (c, a, b): (
        geom_core::Vec3<f64>,
        geom_core::Vec3<f64>,
        geom_core::Vec3<f64>,
    ),
    params: (f64, f64),
) -> Option<(f64, f64)> {
    let (aa, bb, ab) = (a.dot(a), b.dot(b), a.dot(b));
    let speed_sq = 0.5 * (aa + bb) + (0.5 * (aa - bb)).hypot(ab);
    let p_sq_min = 0.5f64.mul_add(aa + bb, c.dot(c))
        - 2.0 * c.dot(a).hypot(c.dot(b))
        - (0.5 * (aa - bb)).hypot(ab);
    let (z_lo, z_hi) = crate::trimmed::harmonic_range(c.z, a.z, b.z, params);
    let rho_sq_min = p_sq_min - z_lo.abs().max(z_hi.abs()).powi(2);
    if !(rho_sq_min > 0.0 && speed_sq.is_finite()) {
        return None;
    }
    let speed = speed_sq.sqrt();
    Some((speed / rho_sq_min.sqrt(), speed / p_sq_min.sqrt()))
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

/// An edge's two sides, the edge's miss named as the tessellator names
/// it.
fn sides_of(body: &Body<f64>, ek: EdgeKey) -> Result<topo::EdgeSides, TessellateError> {
    topo::readback::edge_sides(body, ek)
        .map_err(|_| TessellateError::MissingEntity { what: "edge" })
}

/// The (≤ 2 distinct) faces adjacent to an edge.
fn adjacent_faces(body: &Body<f64>, ek: EdgeKey) -> Result<Vec<topo::FaceKey>, TessellateError> {
    let (plus, minus) = sides_of(body, ek)?.faces();
    Ok(if plus == minus {
        vec![plus]
    } else {
        vec![plus, minus]
    })
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::nurbs_cert::tests::Domination;
    use geom::NurbsCurve3;
    use geom_core::spline::KnotVector;
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
        let (d0, d1) = c.knots_f64_interim().domain();
        let span = d1 - d0;
        let mut ts: Vec<f64> = Vec::new();
        for i in 0..=samples {
            #[allow(clippy::cast_precision_loss)]
            let lam = i as f64 / samples as f64;
            ts.push(d0 + span * lam);
        }
        for (k, _) in c.knots_f64_interim().interior_knots() {
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
            let (d0, d1) = image.knots_f64_interim().domain();
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
    #[test]
    fn a_constant_weight_image_answers_and_a_varying_one_refuses() {
        let base = interior_max_cubic();
        let (d0, d1) = base.knots_f64_interim().domain();
        let plain = general_uv_speeds(&base, (d0, d1), EdgeKey::default()).unwrap();

        let two = geom::NurbsCurve2::new(
            base.knots_f64_interim().clone(),
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
            geom::NurbsCurve2::new(base.knots_f64_interim().clone(), base.control().to_vec(), w)
                .unwrap();
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
        let (d0, d1) = image.knots_f64_interim().domain();
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

    /// **A constant weight is the integral arm's.** The wiggle's net
    /// with every weight one constant (`2`, `3`, `0.1`) is the same
    /// polynomial curve, so its chord
    /// count is the unit-weight curve's at every target (the control
    /// hull reads the net alone); the rational arm's quotient bound on
    /// the same curve is its own, and looser.
    #[test]
    fn a_constant_weight_carrier_counts_as_its_polynomial() {
        let unit = wiggle();
        let (d0, d1) = unit.domain();
        for c in [2.0, 3.0, 0.1] {
            let constant = NurbsCurve3::new(
                unit.knot_values().clone(),
                unit.control().to_vec(),
                vec![c; unit.control().len()],
            )
            .unwrap();
            for target in [1e-2, 1e-4, 1e-6] {
                let count = |n: &NurbsCurve3<f64>| {
                    nurbs_chord_count(n, d1 - d0, target, EdgeKey::default()).expect("in inventory")
                };
                assert_eq!(
                    count(&constant),
                    count(&unit),
                    "weights {c}, δ = {target:e}"
                );
            }
        }
    }

    /// The hull-bounded sagitta generalization: the chord count sizes
    /// the secant deviation under δ — verified against a dense
    /// per-segment sampling oracle.
    #[test]
    fn nurbs_chords_bound_the_secant_deviation() {
        let n = wiggle();
        let (d0, d1) = n.knots_f64_interim().domain();
        for target in [1e-2, 1e-3, 1e-4] {
            let count =
                nurbs_chord_count(&n, d1 - d0, target, EdgeKey::default()).expect("in inventory");
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
                worst <= target * 1.0000001,
                "secant deviation {worst} exceeds the certified budget {target}"
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
        let (d0, d1) = n.knots_f64_interim().domain();
        for target in [1e-2, 1e-3, 1e-4] {
            let count =
                nurbs_chord_count(&n, d1 - d0, target, EdgeKey::default()).expect("in inventory");
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
                worst <= target * 1.0000001,
                "spike deviation {worst} exceeds the certified budget {target}"
            );
            assert!(
                worst >= target * 0.5,
                "spike deviation {worst} fell below half the budget {target} — the \
                 adversarial fixture went slack (review measured 0.990-0.999)"
            );
        }
    }

    /// CONSCIOUS FLIP (M8-5): `rational_nurbs_carrier_refuses_typed`
    /// re-derived as the positive row. Rational carriers are metered
    /// now (the quotient `sup|C″|` of
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
        let wiggle_r = NurbsCurve3::new(
            base.knots_f64_interim().clone(),
            base.control().to_vec(),
            steep,
        )
        .unwrap();
        for (name, n) in [("quarter_arc", arc), ("steep_wiggle", wiggle_r)] {
            let (d0, d1) = n.knots_f64_interim().domain();
            for target in [1e-2, 1e-3, 1e-4] {
                let count = nurbs_chord_count(&n, d1 - d0, target, EdgeKey::default())
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
                    worst <= target * 1.0000001,
                    "{name}: secant deviation {worst} exceeds the certified budget \
                     {target} over {count} chords"
                );
            }
        }
    }

    /// The refusal row the flip keeps: an ILLEGAL rational carrier
    /// (non-positive weight) cannot even be described —
    /// `NurbsCurve3::new` refuses at the door, which is why
    /// [`nurbs_chord_count`] carries no licence check of its own.
    #[test]
    fn illegal_rational_carrier_weight_refuses_at_the_door() {
        let n = wiggle();
        for bad in [0.0, -0.5, f64::NAN] {
            let mut weights = vec![1.0; n.control().len()];
            weights[2] = bad;
            assert!(
                NurbsCurve3::new(n.knots_f64_interim().clone(), n.control().to_vec(), weights)
                    .is_err(),
                "weight {bad} must refuse at construction"
            );
        }
    }

    // ------------------------------------------------------------------
    // R1 REVIEW PROBES (M8-5, PR #322): adversarial rational carriers
    // beyond the PR's — extreme/near-zero weights and the C¹
    // multiplicity edge — checked BOTH ways: the m-bound dominates the
    // dense-sampled true sup‖C″‖ (via deriv2, plus the chord counts
    // keep the measured secant deviation inside δ).
    // ------------------------------------------------------------------

    fn r1_secant_worst(n: &NurbsCurve3<f64>, count: usize) -> f64 {
        let (d0, d1) = n.knots_f64_interim().domain();
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
        let (d0, d1) = n.knots_f64_interim().domain();
        let mut truth = 0.0f64;
        for k in 0..=4000 {
            let t = d0 + (d1 - d0) * f64::from(k) / 4000.0;
            truth = truth.max(n.deriv2(t).norm());
        }
        let d = Domination::sampled_under_certified(&[("sup|C''|", truth, m)]);
        assert!(d.holds(), "{name}: {d}");
        println!("{name}: truth/bound = {:.4}", truth / m);
        // (b) chord counts keep the secant inside target.
        for &target in deltas {
            let count =
                nurbs_chord_count(n, d1 - d0, target, EdgeKey::default()).expect("in inventory");
            let worst = r1_secant_worst(n, count);
            assert!(
                worst <= target * 1.0000001,
                "{name}: secant {worst} exceeds {target} over {count} chords"
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
    /// `worst <= target` on every one of `count` segments — so a finer
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
        let n =
            NurbsCurve3::new(base.knots_f64_interim().clone(), base.control().to_vec(), w).unwrap();
        r1_check("extreme_weight_carrier", &n, &R1_DELTAS[..2]);
    }

    /// Near-zero-touching legal weight (1e-5) amid O(1).
    #[test]
    fn r1_near_zero_weight_carrier() {
        let base = wiggle();
        let mut w = vec![1.0; base.control().len()];
        w[3] = 1e-5;
        let n =
            NurbsCurve3::new(base.knots_f64_interim().clone(), base.control().to_vec(), w).unwrap();
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

    /// **A rational carrier past the second-order ceiling refuses
    /// typed.** A rational Bézier of degree 19 reads `C″` at
    /// `3 · 19 = 57`, past the statement's `56`: the chord pass names the
    /// ceiling rather than sizing a count from a `NaN` bound.
    #[test]
    fn a_rational_carrier_past_the_ceiling_refuses_typed() {
        let kv = KnotVector::clamped([vec![0.0; 20], vec![1.0; 20]].concat(), 19).unwrap();
        let pts = (0..20)
            .map(|i| {
                let t = f64::from(i) / 19.0;
                Point3::new(t, t * t, 0.0)
            })
            .collect();
        let mut w = vec![1.0; 20];
        w[3] = 2.0;
        let n = NurbsCurve3::new(kv, pts, w).unwrap();
        match nurbs_chord_count(&n, 1.0, 1e-3, EdgeKey::default()) {
            Err(TessellateError::UnsupportedCurve { note, .. }) => assert!(
                note.contains("degree above 18") && note.contains("56"),
                "{note}"
            ),
            other => panic!("a degree-19 rational carrier was not refused typed: {other:?}"),
        }
    }

    /// **A rational carrier with an empty span is bounded.** The
    /// quotient's span loop ([`geom_core::spline::accel_pieces`]) draws
    /// each window from a `Span`, so a window past the net or below the
    /// degree is unrepresentable and the loop's only exit is the
    /// empty-span skip — which an interior knot of multiplicity ≥ 2
    /// produces (`knots[s] == knots[s+1]` for an `s` inside
    /// `[first_span, last_span]`). The fixture's bound and chords are
    /// checked as every carrier's are.
    ///
    /// The `any(span(s).is_none())` assertion is the anti-slack guard:
    /// it fails loudly if the fixture ever stops presenting an empty
    /// span, which would silently make this row cover nothing.
    ///
    /// Multiplicity `p` itself is checked too, and it is a REFUSAL, not
    /// a bound: a C⁰ kink leaves the certified inventory at the
    /// degree/kink gate, before any span is read.
    #[test]
    fn a_rational_carrier_with_an_empty_span_is_bounded() {
        // p − 1 = 2: inside the inventory, and it presents an empty span.
        let n = mult_cubic(2);
        let kv = n.knots_f64_interim();
        assert!(
            (kv.first_span()..=kv.last_span()).any(|s| kv.span(s).is_none()),
            "the multiplicity-2 fixture must present an empty span — \
             otherwise this row stops covering the skip"
        );
        // The full δ schedule, as the other cheap carriers take: this
        // fixture is `rational_mult_p1_carrier`'s sibling in cost, not
        // `extreme_weight`'s, so there is nothing here to trim.
        r1_check("rational_mult_2_carrier", &n, &R1_DELTAS);

        // p = 3: refused, typed, and NOT by either deleted message.
        let n = mult_cubic(3);
        let (d0, d1) = n.knots_f64_interim().domain();
        match nurbs_chord_count(&n, d1 - d0, 1e-3, EdgeKey::default()) {
            Err(TessellateError::UnsupportedCurve { note, .. }) => assert!(
                note.contains("C⁰ kink"),
                "multiplicity-p refusal should name the C⁰ kink, got {note:?}"
            ),
            other => panic!("multiplicity p must leave the inventory, got {other:?}"),
        }
    }
}
