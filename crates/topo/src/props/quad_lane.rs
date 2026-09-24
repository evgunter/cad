use geom_brep::Pcurve;
use geom_brep::props::quad::{
    self, FaceCutBounds, HarmChan, RoundOutcome, RoundWindow, TrimChord, TrimEdgeQ, TrimPiece,
};
use geom_brep::props::{LoopEdge, PropsError, loop_vector_area};
use geom_core::Tol;
use geom_core::interval::Interval;
use geom_core::interval::certification::Certification;
// The compound `Decide + Bounds` bound below is a RATIFIED seam
// (M5 PR 11, Ev's lane-split ruling; discipline allowlist row):
// this module is the certified lanes' plumbing and never
// instantiates for duals. Every signature below is
// `Decide + Bounds + CertifiedEnclosure`, which no `Dual`
// implements — a dual carries a bracket since D1 (2026-08-19) and
// still may not certify — and [`super::QuadLane::certified`], the
// one door from the reporting walks into `cut_face`, carries the
// same bound. So the module stays uninstantiable at a dual.
use geom::Curve3;
use geom::Surface;
use geom_core::{Band, Bounds, CertifiedEnclosure, Decide, Point3};

use crate::body::Body;
use crate::entity::HalfEdgeKey;

/// Enclosure midpoint and half-width (the [`super::MassProperties`]
/// pad decomposition).
///
/// A refused enclosure has no midpoint and no width, and answers
/// `NaN` for both — which is what every consumer of this pair
/// already carries through `T::from_f64`. The refusal is asked by
/// name: interval arithmetic keeps it in the decoration, so a refused
/// enclosure's two endpoints are ordinary numbers and their
/// average would be a plausible mass property with nothing behind
/// it.
pub(super) fn mid_pad(x: Interval) -> (f64, f64) {
    if !x.is_certified() {
        return (f64::NAN, f64::NAN);
    }
    ((x.lo() + x.hi()) * 0.5, (x.hi() - x.lo()) * 0.5)
}

/// `(cos t₀, sin t₀)` enclosure at the carrier-interval start,
/// recovered algebraically from the carrier frame and the interval
/// start's VERTEX point (within the run's ε of the carrier, D4 ¶2
/// — the ε rides into the bracket as an explicit pad).
fn trig_at_start<T: Decide + Bounds + CertifiedEnclosure>(
    carrier: &Curve3<T>,
    p: Point3<T>,
    eps: f64,
) -> Result<(Interval, Interval), PropsError> {
    let full = Interval::from_bounds(-1.0, 1.0);
    // `from_bounds` mints a fresh bracket out of whatever
    // endpoints it is handed, so a refused operand would come back
    // clean. The refusal is carried across by hand.
    let clamp = |x: Interval, pad: f64| {
        if !x.is_certified() {
            return Interval::refused();
        }
        Interval::from_bounds(x.lo() - pad, x.hi() + pad).clamped_to(-1.0, 1.0)
    };
    match carrier {
        // A line's harmonic pcurve has zero trig amplitudes; the
        // whole-circle bracket is sound and multiplies away.
        Curve3::Line { .. } => Ok((full, full)),
        Curve3::Circle {
            center,
            axis,
            radius,
            u_ref,
        } => {
            let v_ref = axis.cross(*u_ref);
            let w = p - *center;
            let c = Interval::from_certified(w.dot(*u_ref)) / Interval::from_certified(*radius);
            let s = Interval::from_certified(w.dot(v_ref)) / Interval::from_certified(*radius);
            let pad = (Interval::point(eps) / Interval::from_certified(*radius)).mag();
            Ok((clamp(c, pad), clamp(s, pad)))
        }
        Curve3::Ellipse {
            center,
            axis,
            major,
            minor,
            u_ref,
        } => {
            let v_ref = axis.cross(*u_ref);
            let w = p - *center;
            let c = Interval::from_certified(w.dot(*u_ref)) / Interval::from_certified(*major);
            let s = Interval::from_certified(w.dot(v_ref)) / Interval::from_certified(*minor);
            let pad_c = (Interval::point(eps) / Interval::from_certified(*major)).mag();
            let pad_s = (Interval::point(eps) / Interval::from_certified(*minor)).mag();
            Ok((clamp(c, pad_c), clamp(s, pad_s)))
        }
        // The spiric's chart images are not harmonic (its `m`
        // channel is `√((R + r cos v)² − d²)`), so the trig
        // brackets this lane reads do not exist for it. Unreachable
        // by construction: this lane is entered only for a CYLINDER
        // chart (`cut_face_rounds`'s chart gate), and a spiric lies
        // on no cylinder — the arm names the kind so the gate's
        // removal would meet a typed refusal here rather than a
        // wildcard. The props quadrature lane for a spiric-bounded
        // face is the spiric unit's props PR.
        Curve3::Spiric { .. } => Err(PropsError::QuadratureUnsupported {
            what: "spiric trim carrier on an ANALYTIC chart's quadrature lane — the \
                   hollowed partial revolve's torus wall and plane cap; the spiric \
                   quadrature lane is not yet written",
        }),
        Curve3::Nurbs(_) => Err(PropsError::QuadratureUnsupported {
            what: "B-spline trim carrier on an ANALYTIC chart's quadrature lane — \
                   the cut-loft class (a loft wall cut by a plane/cylinder), which \
                   needs the edge×NURBS-face boolean layer that is not \
                   written; described-NURBS faces with iso-line pcurves route to \
                   the patch engine instead",
        }),
    }
}

/// One channel of a stored harmonic pcurve, bracketed.
fn chan<T: Decide + Bounds + CertifiedEnclosure>(
    c0: T,
    ca: T,
    cb: T,
    cl: T,
) -> Result<HarmChan, PropsError> {
    Ok(HarmChan {
        c0: Interval::from_certified(c0),
        ca: Interval::from_certified(ca),
        cb: Interval::from_certified(cb),
        cl: Interval::from_certified(cl),
    })
}

/// The certified flux/area enclosures of one curved-cut face
/// (module docs of `geom_brep::props::quad`): the cylinder chart's
/// closed-form lane plus the described-NURBS patch lane (M6-3);
/// cone/sphere/torus charts MINT stored pcurves since M6-3 (walk
/// row 4) but their chart-normal flux algebra is not written —
/// they refuse typed naming that true blocker.
pub(super) fn cut_face<T: Decide + Bounds + CertifiedEnclosure>(
    body: &Body<T>,
    surface: &Surface<T>,
    outer: &[LoopEdge<T>],
    hes: &[HalfEdgeKey],
    band: Band,
    tol: Tol,
) -> Result<FaceCutBounds, PropsError> {
    cut_face_rounds(body, surface, outer, hes, band, tol, RoundWindow::SCHEDULE)?.into_target()
}

/// [`cut_face`] over a [`RoundWindow`] — the same lanes, entered
/// and left where the window says (the quadrature module's two
/// levels).
#[allow(clippy::too_many_arguments)]
pub(super) fn cut_face_rounds<T: Decide + Bounds + CertifiedEnclosure>(
    body: &Body<T>,
    surface: &Surface<T>,
    outer: &[LoopEdge<T>],
    hes: &[HalfEdgeKey],
    band: Band,
    tol: Tol,
    window: RoundWindow,
) -> Result<RoundOutcome, PropsError> {
    // The NURBS-patch lane (M6-3): a described NURBS face routes
    // to the patch engine over its stored iso-line pcurves.
    // The spline-patch lane (M6-3): a described spline face routes
    // to the patch engine over its stored iso-line pcurves. An
    // approximating face enters on its fit — the certificate's
    // bound is a statement about the DESCRIPTION and does not
    // widen this quadrature (the same deliberate omission the
    // mesh tolerance makes).
    if let Some(payload) = surface.spline_chart() {
        return nurbs_face(body, payload, outer, hes, band, tol, window);
    }
    let Surface::Cylinder { origin, radius, .. } = surface else {
        return Err(PropsError::QuadratureUnsupported {
            what: "conic trim on a cone/sphere/torus chart — those charts mint stored \
                   pcurves, but this lane's chart-normal flux algebra is the \
                   cylinder chart's; the other analytic charts' closed-form flux \
                   has no lane",
        });
    };
    let eps = tol.eps();
    let va = loop_vector_area(outer, *origin)?;
    let o_dot_va = Interval::from_certified((*origin - Point3::origin()).dot(va));
    let mut edges = Vec::with_capacity(outer.len());
    for (le, he) in outer.iter().zip(hes) {
        let Some(cache) = body.pcurve(*he) else {
            return Err(PropsError::QuadratureUnsupported {
                what: "curved-cut face half-edge carries no stored pcurve cache — \
                       caches mint in the split/boolean pipelines",
            });
        };
        // The certified quadrature lane reads a chart image
        // CHANNEL BY CHANNEL out of its closed form; a fitted
        // (rung-3) image has no such form on an ANALYTIC chart's
        // Green reduction. Typed refusal — the TRUE remaining
        // blocker (M6-3 stale-claims sweep): no at-rest body mints
        // a fitted pcurve on a cylinder chart today (the marched
        // join windows and the edge×NURBS-face boolean layer are
        // both banked past M6), and the fitted-boundary Green lane
        // (`quad::bspline_green_integral`'s remaining consumer)
        // lands WITH whichever of those first produces one.
        let Pcurve::Harmonic { p0, pa, pb, pl } = *cache.pcurve() else {
            return Err(PropsError::QuadratureUnsupported {
                what: "curved-cut face half-edge carries a FITTED (rung-3) pcurve on an \
                       analytic chart — its Green-form boundary integral \
                       (bspline_green_integral) wires up with the construction that \
                       first mints one at rest (the banked join-window/edge×NURBS-face \
                       boolean layers); nothing does today",
            });
        };
        let (t0, t1) = cache.params();
        // The interval-start vertex: traversal start when forward,
        // traversal end when reversed (`he_plus` start either way).
        let p_start = start_point(body, *he, le.forward)?;
        let trig0 = trig_at_start(&le.carrier, p_start, eps)?;
        edges.push(TrimEdgeQ {
            u: chan(p0.x, pa.x, pb.x, pl.x)?,
            v: chan(p0.y, pa.y, pb.y, pl.y)?,
            t0: Interval::from_certified(t0),
            t1: Interval::from_certified(t1),
            forward: le.forward,
            trig0,
            env: Interval::from_certified(cache.certificate().envelope),
        });
    }
    quad::cylinder_cut_face_rounds::<T>(
        Interval::from_certified(*radius),
        o_dot_va,
        &edges,
        eps,
        band,
        window,
    )
}

/// **The NURBS-patch flux lane** (M6-3 Leg C; RATIONAL since
/// M8-3): certified volume flux + area of a described NURBS face whose
/// stored pcurves pin its trim region to an exact axis-aligned UV
/// rectangle (every loft/sweep wall — their boundaries are iso
/// lines with exact-structure `0`/`1` chart values).
///
/// Structure checks are EXACT `f64` (C6: the minted chart values
/// are exact by construction; a non-exact or non-rectangular
/// boundary refuses typed, naming the trimmed-NURBS lane as the
/// cut-loft unit's). The traversal's shoelace sign IS the S10
/// orientation input — winding-derived end to end, like the
/// cylinder lane; no sense bit is read.
#[allow(clippy::too_many_arguments)]
fn nurbs_face<T: Decide + Bounds + CertifiedEnclosure>(
    body: &Body<T>,
    payload: &geom::NurbsSurface<T>,
    outer: &[LoopEdge<T>],
    hes: &[HalfEdgeKey],
    band: Band,
    tol: Tol,
    window: RoundWindow,
) -> Result<RoundOutcome, PropsError> {
    if payload.is_placeholder() {
        return Err(PropsError::QuadratureUnsupported {
            what: "the mvfs Nurbs placeholder reached the quadrature lane — a \
                   mid-surgery body has no mass properties (tier 2 refuses it at rest)",
        });
    }
    // **The dispatch is by pcurve KIND** (TRIM-2 §8.1): a loop
    // whose every image is an iso class pins the trim region to an
    // axis-aligned rectangle and keeps the rectangle certificate
    // below, bit for bit; a loop carrying a `General` image bounds
    // a region that is not a rectangle of its chart at all, and
    // takes the trimmed lane. `Fitted` keeps its own refusal in
    // both — no shipped construction mints one here.
    if hes
        .iter()
        .filter_map(|he| body.pcurve(*he))
        .any(|c| matches!(c.pcurve(), Pcurve::General(_)))
    {
        return trimmed_face(body, payload, outer, hes, band, tol, window);
    }
    let eps = tol.eps();
    // Exact-structure read of a T scalar (point bracket required).
    let exact = |x: Interval| -> Result<f64, PropsError> {
        // The refusal first: a refused crossing carries the
        // scalar's own endpoints, so a point bracket that may not
        // certify passes both tests below.
        if x.is_certified() && x.lo() == x.hi() && x.lo().is_finite() {
            Ok(x.lo())
        } else {
            Err(PropsError::QuadratureUnsupported {
                what: "a NURBS-face pcurve endpoint is not exact structure — the \
                       rectangle-trim certificate needs the minted exact 0/1 chart \
                       values (trimmed-NURBS regions are the cut-loft unit's)",
            })
        }
    };
    let mut polygon: Vec<(f64, f64)> = Vec::with_capacity(outer.len());
    let mut boundary_defect = 0.0f64;
    let mut perimeter = 0.0f64;
    for (le, he) in outer.iter().zip(hes) {
        let Some(cache) = body.pcurve(*he) else {
            return Err(PropsError::QuadratureUnsupported {
                what: "NURBS face half-edge carries no stored pcurve cache — the \
                       loft assembly mints them; a body that lost its caches must \
                       re-mint before mass properties",
            });
        };
        // Both iso classes pin the trim region to the rectangle:
        // `IsoLine` for seams and line rims, `IsoArc` for a
        // rational wall's arc rims (M8-3) — an arc rim's chart
        // image is the SAME boundary line, only its
        // parameterization differs, and this lane reads endpoints.
        if !matches!(
            cache.pcurve(),
            Pcurve::IsoLine { .. } | Pcurve::IsoArc { .. }
        ) {
            return Err(PropsError::QuadratureUnsupported {
                what: "a NURBS-face half-edge carries a non-iso pcurve — a trimmed \
                       NURBS region's quadrature is the cut-loft unit's (the \
                       edge×NURBS-face boolean layer mints those trims)",
            });
        }
        let (t0, t1) = cache.params();
        let a = cache.pcurve().eval(t0);
        let b = cache.pcurve().eval(t1);
        let (ax, ay) = (
            exact(Interval::from_certified(a.x))?,
            exact(Interval::from_certified(a.y))?,
        );
        let (bx, by) = (
            exact(Interval::from_certified(b.x))?,
            exact(Interval::from_certified(b.y))?,
        );
        if ax != bx && ay != by {
            return Err(PropsError::QuadratureUnsupported {
                what: "a NURBS-face pcurve is not axis-aligned — a diagonal trim is \
                       outside the rectangle lane (the cut-loft unit's)",
            });
        }
        // Traversal order: the loop walks he_plus-forward edges
        // start→end and reversed ones end→start.
        if le.forward {
            polygon.push((ax, ay));
        } else {
            polygon.push((bx, by));
        }
        // Metric boundary length bound + the map-residual defect.
        let len = carrier_metric_length(&le.carrier, t0, t1)?;
        perimeter += len;
        boundary_defect += len * Interval::from_certified(cache.certificate().envelope).mag();
    }
    // The rectangle certificate: hull of the traversal polygon,
    // every vertex on a corner, and the shoelace equal to ±the
    // rectangle area — the sign IS the S10 winding.
    let (mut u0, mut u1) = (f64::INFINITY, f64::NEG_INFINITY);
    let (mut v0, mut v1) = (f64::INFINITY, f64::NEG_INFINITY);
    for &(x, y) in &polygon {
        u0 = u0.min(x);
        u1 = u1.max(x);
        v0 = v0.min(y);
        v1 = v1.max(y);
    }
    let mut shoelace = 0.0f64;
    for i in 0..polygon.len() {
        let (xa, ya) = polygon[i];
        let (xb, yb) = polygon[(i + 1) % polygon.len()];
        shoelace += xa * yb - xb * ya;
        if (xa != u0 && xa != u1) && (ya != v0 && ya != v1) {
            return Err(PropsError::QuadratureUnsupported {
                what: "a NURBS-face boundary vertex sits strictly inside the UV \
                       rectangle — a re-entrant trim is outside the rectangle lane \
                       (the cut-loft unit's)",
            });
        }
    }
    shoelace *= 0.5;
    let rect_area = (u1 - u0) * (v1 - v0);
    let winding = if shoelace == rect_area {
        1.0
    } else if shoelace == -rect_area {
        -1.0
    } else {
        return Err(PropsError::QuadratureUnsupported {
            what: "the NURBS-face boundary does not traverse its UV rectangle exactly \
                   once (shoelace ≠ ±rectangle area) — a trimmed or multiply-wound \
                   region is outside the rectangle lane (the cut-loft unit's)",
        });
    };
    let control: Vec<quad::RVec3> = payload
        .control()
        .iter()
        .map(|p| {
            [
                Interval::from_certified(p.x),
                Interval::from_certified(p.y),
                Interval::from_certified(p.z),
            ]
        })
        .collect();
    let out = quad::nurbs_patch_face_rounds::<T>(
        payload.knots_u(),
        payload.knots_v(),
        &control,
        payload.weights(),
        (u0, u1, v0, v1),
        perimeter,
        boundary_defect,
        eps,
        band,
        window,
    )?;
    // The winding sign carries the S10 orientation into the flux;
    // the area is unsigned. It applies at whichever round the
    // window ended: the sign is a property of the traversal, not
    // of the refinement.
    Ok(out.map_bounds(|b| FaceCutBounds {
        flux: if winding < 0.0 { -b.flux } else { b.flux },
        area: b.area,
    }))
}

/// A certified UPPER bound on a trim carrier's METRIC length, in
/// metres — the lever of both honesty pads (`Σ L·envelope` widens
/// the area, and `× p_bound` the flux) and of the extent gate's
/// perimeter. ONE home: the rectangle certificate and the trimmed
/// lane bound the same quantity the same way, and two spellings of
/// it would be two things to keep equal.
fn carrier_metric_length<T: Decide + Bounds + CertifiedEnclosure>(
    carrier: &Curve3<T>,
    t0: T,
    t1: T,
) -> Result<f64, PropsError> {
    Ok(match carrier {
        Curve3::Line { dir, .. } => {
            (Interval::from_certified(dir.norm()) * Interval::from_certified(t1 - t0)).mag()
        }
        // The control polygon bounds the spline's arc length
        // (the convex-hull/variation-diminishing fact).
        Curve3::Nurbs(c) => {
            let mut l = Interval::zero();
            for w in c.control().windows(2) {
                l = l + Interval::from_certified(w[0].distance(w[1]));
            }
            l.mag()
        }
        // An ARC cap rim on a rational wall (M8-3): the metric
        // length is exactly `r·Δθ` — the carrier's own parameter
        // IS the angle, so no bound is needed.
        Curve3::Circle { radius, .. } => {
            (Interval::from_certified(*radius) * Interval::from_certified(t1 - t0)).mag()
        }
        _ => {
            return Err(PropsError::QuadratureUnsupported {
                what: "a NURBS-face boundary carrier outside the loft inventory \
                       (line, spline and circle rims are the minted classes)",
            });
        }
    })
}

/// **The TRIMMED-region flux lane** (TRIM-2): a described NURBS
/// face whose loop carries a `General` chart image, so its trim
/// region is what the image bounds rather than a rectangle of the
/// chart.
///
/// This function assembles; the certification is
/// [`quad::trimmed_patch_face_rounds`]'s. The traversal's own
/// direction is carried per chord and the S10 winding is the chord
/// polygon's shoelace sign, read inside the engine — winding-derived
/// end to end, exactly as the rectangle certificate and the cylinder
/// lane are.
#[allow(clippy::too_many_arguments)]
fn trimmed_face<T: Decide + Bounds + CertifiedEnclosure>(
    body: &Body<T>,
    payload: &geom::NurbsSurface<T>,
    outer: &[LoopEdge<T>],
    hes: &[HalfEdgeKey],
    band: Band,
    tol: Tol,
    window: RoundWindow,
) -> Result<RoundOutcome, PropsError> {
    let ring = |x: T| Interval::from_certified(x);
    let mut chords: Vec<TrimChord> = Vec::with_capacity(outer.len());
    for (le, he) in outer.iter().zip(hes) {
        let Some(cache) = body.pcurve(*he) else {
            return Err(PropsError::QuadratureUnsupported {
                what: "NURBS face half-edge carries no stored pcurve cache — the \
                       loft assembly mints them; a body that lost its caches must \
                       re-mint before mass properties",
            });
        };
        let (t0, t1) = cache.params();
        let (pa, pb) = (cache.pcurve().eval(t0), cache.pcurve().eval(t1));
        let (a, b) = if le.forward {
            ((ring(pa.x), ring(pa.y)), (ring(pb.x), ring(pb.y)))
        } else {
            ((ring(pb.x), ring(pb.y)), (ring(pa.x), ring(pa.y)))
        };
        let piece = match cache.pcurve() {
            // An iso image is one exact chord: its endpoints are
            // structure, so there is no arc to bound.
            Pcurve::IsoLine { .. } | Pcurve::IsoArc { .. } => None,
            Pcurve::General(image) => {
                // The engine subdivides the WHOLE stored image, so
                // a cache whose carrier interval is a sub-range of
                // its image's domain would have the lane integrate
                // along chart the face does not bound. Exact
                // structure, like every other read on this path.
                let (d0, d1) = image.domain();
                let (r0, r1) = (ring(t0), ring(t1));
                // NO ROW AND NO KNOWN PRODUCER, stated so a reader
                // does not take the guard for evidence of the case:
                // `derive_general_image` mints an image over the
                // carrier's whole interval, so nothing at rest
                // stores a sub-range, and nothing in the suites
                // hand-builds one. It is here because the trimmed
                // lane subdivides the STORED image whole, and a
                // future producer that stored a sub-range would get
                // a certified number for chart the face does not
                // bound rather than a refusal.
                // The refusal first, for the reason the
                // `exact` closure above gives.
                if !r0.is_certified()
                    || !r1.is_certified()
                    || !(r0.lo() == r0.hi() && r1.lo() == r1.hi() && r0.lo() == d0 && r1.hi() == d1)
                {
                    return Err(PropsError::QuadratureUnsupported {
                        what: "a General trim image whose carrier interval is not its \
                               own knot domain — the trimmed lane subdivides the \
                               stored image whole, and a sub-range would integrate \
                               along chart the face does not bound",
                    });
                }
                Some(TrimPiece {
                    knots: image.knots().clone(),
                    control: image
                        .control()
                        .iter()
                        .map(|p| (ring(p.x), ring(p.y)))
                        .collect(),
                    weights: image.weights().to_vec(),
                })
            }
            Pcurve::Fitted(_) => {
                return Err(PropsError::QuadratureUnsupported {
                    what: "a NURBS-face half-edge carries a FITTED (rung-3) pcurve — \
                           the trimmed lane certifies the General class, whose \
                           agreement with its carrier is a measurement; nothing \
                           ships that mints a Fitted image on a spline chart",
                });
            }
            Pcurve::Harmonic { .. } => {
                return Err(PropsError::QuadratureUnsupported {
                    what: "a NURBS-face half-edge carries a HARMONIC pcurve — that is \
                           an analytic chart's closed form, and this chart is a \
                           spline patch",
                });
            }
        };
        chords.push(TrimChord {
            a,
            b,
            piece,
            forward: le.forward,
            env: ring(cache.certificate().envelope),
        });
    }
    // **One bracket per shared vertex.** Two consecutive half-edges
    // meet at a vertex, and each reads it through its OWN pcurve —
    // an `IsoLine`'s `eval(t)` against a `General`'s clamped end —
    // so at `f64` the two reads can differ by the certification's
    // own size (2.2e-16 on the P-2 fixture, where the image's
    // control box is `u ∈ [2 − 2.2e-16, 2]` against the rim's exact
    // `u = 2`). Hulling them makes the walk close by construction
    // and hands the door the honest bracket for the vertex; the
    // door's own closure check then guards a CALLER, not this
    // assembler's rounding.
    for i in 0..chords.len() {
        let j = (i + 1) % chords.len();
        let merged = (
            Interval::hull(chords[i].b.0, chords[j].a.0),
            Interval::hull(chords[i].b.1, chords[j].a.1),
        );
        chords[i].b = merged;
        chords[j].a = merged;
    }
    let control: Vec<quad::RVec3> = payload
        .control()
        .iter()
        .map(|p| [ring(p.x), ring(p.y), ring(p.z)])
        .collect();
    quad::trimmed_patch_face_rounds::<T>(
        payload.knots_u(),
        payload.knots_v(),
        &control,
        payload.weights(),
        &chords,
        tol.eps(),
        band,
        window,
    )
}

/// The vertex POINT at a half-edge's carrier-interval start (its
/// edge's `he_plus` start vertex).
fn start_point<T: Decide + Bounds + CertifiedEnclosure>(
    body: &Body<T>,
    he: HalfEdgeKey,
    forward: bool,
) -> Result<Point3<T>, PropsError> {
    let corrupt = PropsError::QuadratureUnsupported {
        what: "corrupt body reaching the quadrature lane (a key did not resolve)",
    };
    let vk = if forward {
        body.half_edges.get(he).ok_or(corrupt.clone())?.start
    } else {
        body.half_edge_end(he).ok_or(corrupt.clone())?
    };
    let v = body.vertices.get(vk).ok_or(corrupt.clone())?;
    body.points.get(v.point).copied().ok_or(corrupt)
}

#[cfg(test)]
mod tests {
    /// The scalar bracket seam, at the `Interval` scalar.
    ///
    /// A bracket can be sound and still inadmissible:
    /// `sqrt([−1, 4]) + 1` is `[1, 3]` with decoration `Trv`.
    /// The crossing into certification arithmetic reads the verdict
    /// here and caps the decoration at `Trv`, so the quadrature
    /// lane's scalars are refused HERE rather than a certified flux
    /// enclosure being built from a quantity that was clamped out of
    /// its own domain.
    #[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
    mod bracket_seam_tests {
        use geom_core::{Bounds, CertifiedEnclosure, Interval, Real};

        use super::super::chan;

        /// Finite, strictly positive, and unable to certify — the case
        /// where the laundered answer is a *usable* number.
        fn trv_pos() -> Interval {
            Interval::from_bounds(-1.0, 4.0).sqrt() + Interval::from_f64(1.0)
        }

        #[test]
        fn the_fixture_is_a_finite_bracket_that_cannot_certify() {
            let x = trv_pos();
            assert_eq!((Bounds::lo(x), Bounds::hi(x)), (1.0, 3.0));
            assert!(x.certified_bracket().is_none());
        }

        #[test]
        fn the_certified_door_refuses_a_violated_scalar() {
            let r = Interval::from_certified(trv_pos());
            assert!(
                !r.is_certified(),
                "a domain-violated scalar crossed into certification arithmetic as {r:?} — \
                 the bracket door does not read decorations, so the \
                 quadrature lane certifies a flux built from it"
            );
            // Non-vacuity: a certified scalar crosses with its endpoints.
            let ok = Interval::from_certified(Interval::from_bounds(1.0, 4.0).sqrt());
            assert_eq!((ok.lo(), ok.hi()), (1.0, 2.0));
        }

        /// Where a violated scalar would have to come FROM. Every
        /// scalar this lane hands to [`Interval::from_certified`]
        /// is either read straight off
        /// the stored body or built from it by `dot`, `norm`,
        /// `distance` and arithmetic — and none of those can
        /// manufacture a domain violation: a norm is the square root of
        /// a sum of squares, which is never partly negative, so it
        /// certifies even where it is zero and the vector degenerate.
        /// A `Trv` reaching the door therefore has to have been STORED in
        /// the body, not produced here. That is a property of the
        /// arithmetic, not of any guard, so it is pinned rather than
        /// assumed.
        #[test]
        fn the_lanes_own_arithmetic_cannot_manufacture_a_violation() {
            use geom_core::Vec3;
            let iv = geom_core::Interval::from_f64;
            for v in [
                Vec3::new(iv(0.0), iv(0.0), iv(0.0)),
                Vec3::new(iv(-3.0), iv(4.0), iv(0.0)),
                Vec3::new(
                    geom_core::Interval::from_bounds(-1.0, 1.0),
                    iv(0.0),
                    iv(0.0),
                ),
            ] {
                assert!(
                    v.norm().certified_bracket().is_some(),
                    "a norm certified nothing for {v:?}"
                );
                assert!(Interval::from_certified(v.norm()).is_certified());
            }
        }

        /// The seam is per scalar, not per channel: one violated
        /// coefficient poisons its own slot and leaves the rest intact,
        /// so the poison reaches the flux algebra where it is visible.
        #[test]
        fn chan_poisons_only_the_violated_coefficient() {
            let one = Interval::from_f64(1.0);
            let c = chan(one, trv_pos(), one, one).expect("channel builds");
            assert!(!c.ca.is_certified(), "the violated coefficient survived");
            for (tag, r) in [("c0", c.c0), ("cb", c.cb), ("cl", c.cl)] {
                assert!(r.is_certified(), "{tag} poisoned a certified coefficient");
            }
        }
    }
}
