use geom_brep::Pcurve;
use geom_brep::props::quad::{
    self, FaceCutBounds, HarmChan, RoundOutcome, RoundWindow, TrimChord, TrimEdgeQ, TrimPiece,
};
use geom_brep::props::{FaceContribution, LoopEdge, PropsError, loop_vector_area};
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
// one door from the face walks into `cut_face_rounds`, carries the
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

/// One face's flux about `centre` and its area, at the interval scalar.
///
/// A closed-form face (`quadrature` is `None`): its surface and loops
/// lifted point for point (`map_scalar`, which does no arithmetic),
/// carried by `−centre` ([`translated_surface`], [`translated_curve`]),
/// and handed to the same closed form the face walk runs
/// (`super::closed_form_of`), whose flux about the moved origin is the
/// face's flux about `centre`. A plane is taken about `centre` directly
/// ([`planar_face_about`]).
///
/// A quadrature face (`quadrature` is its lane's flux and area
/// enclosures, about the world origin) arrives here only where its lane
/// refused to measure it about `centre` (`rederive` asks
/// [`cut_face_rounds`] first): the flux less `centre · A⃗`, with `A⃗` the
/// face's vector area from its own loops — the same value, at the width
/// the quadrature returned it with.
///
/// The second half says whether the face was RECENTRED, its width the
/// body's own: `false` for such a quadrature face, and for a closed-form face
/// whose geometry has no translated twin here, whose flux is then the
/// closed form about the world origin less `centre · A⃗`.
pub(super) fn closed_form<T: Decide + geom_core::CertifiedBounds>(
    surface: &Surface<T>,
    loops: &[Vec<LoopEdge<T>>],
    sense: bool,
    band: Band,
    centre: Point3<Interval>,
    quadrature: Option<(Interval, Interval)>,
) -> Result<(FaceContribution<Interval>, bool), PropsError> {
    let loops: Vec<Vec<LoopEdge<Interval>>> = loops
        .iter()
        .map(|edges| {
            edges
                .iter()
                .map(|e| LoopEdge {
                    carrier: e.carrier.map_scalar(Interval::from_certified),
                    carrier_id: e.carrier_id,
                    t0: Interval::from_certified(e.t0),
                    t1: Interval::from_certified(e.t1),
                    forward: e.forward,
                    start: e.start,
                    end: e.end,
                })
                .collect()
        })
        .collect();
    let about_centre = |flux: Interval, area| -> Result<_, PropsError> {
        let va = loops_vector_area(&loops)?;
        Ok((
            FaceContribution {
                flux: flux - (centre - Point3::origin()).dot(va),
                area,
            },
            false,
        ))
    };
    if let Some((flux, area)) = quadrature {
        return about_centre(flux, area);
    }
    let surface = surface.map_scalar(Interval::from_certified);
    if let Surface::Plane { .. } = surface {
        return Ok((planar_face_about(&loops, centre)?, true));
    }
    let moved_loops = loops
        .iter()
        .map(|edges| {
            edges
                .iter()
                .map(|e| {
                    translated_curve(&e.carrier, centre).map(|carrier| LoopEdge {
                        carrier,
                        ..e.clone()
                    })
                })
                .collect::<Option<Vec<_>>>()
        })
        .collect::<Option<Vec<_>>>();
    match translated_surface(&surface, centre).zip(moved_loops) {
        Some((surface, moved_loops)) => Ok((
            super::closed_form_of(&surface, &moved_loops, sense, band)?,
            true,
        )),
        None => {
            let about_origin = super::closed_form_of(&surface, &loops, sense, band)?;
            about_centre(about_origin.flux, about_origin.area)
        }
    }
}

/// A face's vector area `A⃗ = ∫ n dA`, from its loops, each summed about
/// a point of its own (a closed loop's does not depend on it).
fn loops_vector_area(
    loops: &[Vec<LoopEdge<Interval>>],
) -> Result<geom_core::Vec3<Interval>, PropsError> {
    let mut va = geom_core::Vec3::zero();
    for edges in loops {
        let Some(anchor) = edges.first().map(|e| e.carrier.eval(e.t0)) else {
            return Err(PropsError::DegenerateFace);
        };
        va = va + loop_vector_area(edges, anchor)?;
    }
    Ok(va)
}

/// `surface` carried by `−by`: every point-valued datum moved, every
/// direction and length kept. `None` for a kind with no analytic datum
/// to move (a spline or fitted surface).
fn translated_surface(
    surface: &Surface<Interval>,
    by: Point3<Interval>,
) -> Option<Surface<Interval>> {
    let shift = |p: Point3<Interval>| Point3::origin() + (p - by);
    Some(match surface.clone() {
        Surface::Plane {
            origin,
            normal,
            u_ref,
        } => Surface::Plane {
            origin: shift(origin),
            normal,
            u_ref,
        },
        Surface::Cylinder {
            origin,
            axis,
            radius,
            u_ref,
        } => Surface::Cylinder {
            origin: shift(origin),
            axis,
            radius,
            u_ref,
        },
        Surface::Cone {
            apex,
            axis,
            half_angle,
            u_ref,
        } => Surface::Cone {
            apex: shift(apex),
            axis,
            half_angle,
            u_ref,
        },
        Surface::Sphere {
            center,
            radius,
            axis,
            u_ref,
        } => Surface::Sphere {
            center: shift(center),
            radius,
            axis,
            u_ref,
        },
        Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            u_ref,
        } => Surface::Torus {
            center: shift(center),
            axis,
            major_radius,
            minor_radius,
            u_ref,
        },
        _ => return None,
    })
}

/// `curve` carried by `−by`: the analytic kinds' centre or origin moved,
/// a spline's control net moved point for point (its net is stored
/// Euclidean, so a translation is the curve's image).
fn translated_curve(curve: &Curve3<Interval>, by: Point3<Interval>) -> Option<Curve3<Interval>> {
    let shift = |p: Point3<Interval>| Point3::origin() + (p - by);
    Some(match curve.clone() {
        Curve3::Line { origin, dir } => Curve3::Line {
            origin: shift(origin),
            dir,
        },
        Curve3::Circle {
            center,
            axis,
            radius,
            u_ref,
        } => Curve3::Circle {
            center: shift(center),
            axis,
            radius,
            u_ref,
        },
        Curve3::Ellipse {
            center,
            axis,
            major,
            minor,
            u_ref,
        } => Curve3::Ellipse {
            center: shift(center),
            axis,
            major,
            minor,
            u_ref,
        },
        Curve3::Spiric {
            center,
            axis,
            u_ref,
            major_radius,
            minor_radius,
            offset,
        } => Curve3::Spiric {
            center: shift(center),
            axis,
            u_ref,
            major_radius,
            minor_radius,
            offset,
        },
        Curve3::Nurbs(n) => Curve3::Nurbs(std::sync::Arc::new(n.map_points(shift))),
    })
}

/// A planar face's flux about `centre` and its area, at the interval
/// scalar: `(anchor − centre)·A⃗`, with `A⃗` the face's vector area summed
/// about `anchor`, the first point of its loops.
///
/// It is the exact flux, about `centre`, of each loop fanned from
/// `anchor` (a point `x` of the fan has `(x − anchor)` in its tangent
/// plane, so `x·n` integrates to `anchor·A⃗`). The fans of neighbouring
/// faces meet on their shared edges, but a loop closes only to the
/// rounding of its carriers' ends, and a fan from a far anchor reads that
/// gap at the face's length times its lever
/// (`work/tally/a-fan-over-carrier-ends-reads-an-ulp-gap-at-the-faces-length-times-its-lever`);
/// a shell of line-bounded planes is read off its vertex polygons
/// instead ([`polygon_face_about`]), which close exactly. A flux taken off the
/// stored plane instead (`((origin − centre)·n)(n·A⃗)/(n·n)`) is not a
/// closed surface's: a glued face whose points stand `δ` off its carrier
/// misses `δ` times its area, which crossed the oracle on the door's
/// settled-residue fixture (`tests/door_backstop_settled_residue.rs`).
/// `anchor` is a point of the face, so the width is the body's own size
/// times `A⃗`'s; the product is as small as `centre` is near the face,
/// which `rederive` arranges by taking a corner of the body's own loop
/// points. The carrier's origin is never read.
fn planar_face_about(
    loops: &[Vec<LoopEdge<Interval>>],
    centre: Point3<Interval>,
) -> Result<FaceContribution<Interval>, PropsError> {
    let Some(anchor) = loops
        .first()
        .and_then(|edges| edges.first())
        .map(|e| e.carrier.eval(e.t0))
    else {
        return Err(PropsError::DegenerateFace);
    };
    let mut va = geom_core::Vec3::zero();
    for edges in loops {
        va = va + loop_vector_area(edges, anchor)?;
    }
    Ok(FaceContribution {
        flux: (anchor - centre).dot(va),
        area: va.norm(),
    })
}

/// A planar face bounded by lines, as the polygons of its vertex points
/// (`rings`, in traversal order): its flux about `centre` and its area,
/// at the interval scalar, `(anchor − centre)·A⃗` with `A⃗` the rings
/// fanned from `anchor`, the first point of the first ring.
///
/// Over a shell every face of which is read this way, neighbouring
/// faces' rings meet at the same points, so the sum is the volume of the
/// closed polyhedron of the vertex points. A vertex point is a point
/// interval, so each difference below is one rounding wide, at the
/// difference's own magnitude.
pub(super) fn polygon_face_about(
    rings: &[Vec<Point3<Interval>>],
    centre: Point3<Interval>,
) -> Result<FaceContribution<Interval>, PropsError> {
    let Some(&anchor) = rings.first().and_then(|ring| ring.first()) else {
        return Err(PropsError::DegenerateFace);
    };
    let mut va = geom_core::Vec3::zero();
    for ring in rings {
        for (i, &p) in ring.iter().enumerate() {
            let q = ring[(i + 1) % ring.len()];
            va = va + (p - anchor).cross(q - anchor) * Interval::point(0.5);
        }
    }
    Ok(FaceContribution {
        flux: (anchor - centre).dot(va),
        area: va.norm(),
    })
}

/// The certified flux/area enclosures of one curved-cut face
/// (module docs of `geom_brep::props::quad`) over a [`RoundWindow`]
/// — the cylinder chart's closed-form lane plus the described-NURBS
/// patch lane (M6-3), entered and left where the window says (the
/// quadrature module's two levels); [`super::QuadLane`] holds this
/// and reads it at either level. A cone face never arrives: the face
/// walk routes it to its closed form
/// ([`geom_brep::props::cone_face_closed_form`]). Sphere and torus
/// charts mint stored pcurves but have no flux lane here, and refuse
/// typed.
#[allow(clippy::too_many_arguments)]
pub(super) fn cut_face_rounds<T: Decide + Bounds + CertifiedEnclosure>(
    body: &Body<T>,
    surface: &Surface<T>,
    outer: &[LoopEdge<T>],
    hes: &[HalfEdgeKey],
    band: Band,
    tol: Tol,
    window: RoundWindow,
    centre: Option<Point3<Interval>>,
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
        return nurbs_face(body, payload, outer, hes, band, tol, window, centre);
    }
    let Surface::Cylinder { origin, radius, .. } = surface else {
        return Err(PropsError::QuadratureUnsupported {
            what: "conic trim on a cone/sphere/torus chart — a cone face takes its \
                   closed form before this lane (`cone_face_closed_form`); sphere and \
                   torus charts mint stored pcurves, but this lane's chart-normal flux \
                   algebra is the cylinder's",
        });
    };
    let eps = tol.eps();
    let va = loop_vector_area(outer, *origin)?;
    // The flux about the chart's own origin, `r²·A_s`, is where the
    // face is; only this term carries its position. Taken about a centre
    // it is `(origin − centre)·A⃗` in interval arithmetic, a lever the
    // size of the body rather than of its distance from the world origin.
    let o_dot_va = match centre {
        None => Interval::from_certified((*origin - Point3::origin()).dot(va)),
        Some(c) => {
            let lift = |p: Point3<T>| {
                Point3::new(
                    Interval::from_certified(p.x),
                    Interval::from_certified(p.y),
                    Interval::from_certified(p.z),
                )
            };
            let va = geom_core::Vec3::new(
                Interval::from_certified(va.x),
                Interval::from_certified(va.y),
                Interval::from_certified(va.z),
            );
            (lift(*origin) - c).dot(va)
        }
    };
    let mut edges = Vec::with_capacity(outer.len());
    let lifted = crate::pcurves::lifted_images(body, hes);
    for ((le, he), lifted) in outer.iter().zip(hes).zip(&lifted) {
        let (Some(cache), Some(image)) = (body.pcurve(*he), lifted) else {
            return Err(PropsError::QuadratureUnsupported {
                what: "curved-cut face half-edge carries no stored pcurve cache — \
                       caches mint in the split/boolean pipelines",
            });
        };
        // The certified quadrature lane reads a chart image
        // CHANNEL BY CHANNEL out of its harmonic form; a fitted or
        // focal-section image has none on an ANALYTIC chart's Green
        // reduction.
        // Typed refusal: the fitted-boundary Green lane
        // (`quad::bspline_green_integral`'s remaining consumer) is not
        // wired. A sphere's general circle mints one at rest (an
        // oblique fillet corner's octant); the props door refuses that
        // face's spherical triangle before this lane is asked.
        let Pcurve::Harmonic { p0, pa, pb, pl } = *image else {
            return Err(PropsError::QuadratureUnsupported {
                what: "curved-cut face half-edge carries a pcurve with no harmonic form on \
                       an analytic chart (a FITTED image, or a cone section's or Villarceau \
                       circle's focal section) — its Green-form boundary integral is not \
                       wired",
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
    centre: Option<Point3<Interval>>,
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
        return trimmed_face(body, payload, outer, hes, band, tol, window, centre);
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
    let lifted = crate::pcurves::lifted_images(body, hes);
    for ((le, he), lifted) in outer.iter().zip(hes).zip(&lifted) {
        let (Some(cache), Some(image)) = (body.pcurve(*he), lifted) else {
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
        let a = image.eval(t0);
        let b = image.eval(t1);
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
    let control = about(
        payload
            .control()
            .iter()
            .map(|p| {
                [
                    Interval::from_certified(p.x),
                    Interval::from_certified(p.y),
                    Interval::from_certified(p.z),
                ]
            })
            .collect(),
        centre,
    );
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
    centre: Option<Point3<Interval>>,
) -> Result<RoundOutcome, PropsError> {
    let ring = |x: T| Interval::from_certified(x);
    let mut chords: Vec<TrimChord> = Vec::with_capacity(outer.len());
    let lifted = crate::pcurves::lifted_images(body, hes);
    for ((le, he), lifted) in outer.iter().zip(hes).zip(&lifted) {
        let (Some(cache), Some(image)) = (body.pcurve(*he), lifted) else {
            return Err(PropsError::QuadratureUnsupported {
                what: "NURBS face half-edge carries no stored pcurve cache — the \
                       loft assembly mints them; a body that lost its caches must \
                       re-mint before mass properties",
            });
        };
        let (t0, t1) = cache.params();
        let (pa, pb) = (image.eval(t0), image.eval(t1));
        let (a, b) = if le.forward {
            ((ring(pa.x), ring(pa.y)), (ring(pb.x), ring(pb.y)))
        } else {
            ((ring(pb.x), ring(pb.y)), (ring(pa.x), ring(pa.y)))
        };
        let piece = match image {
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
                    what: "a NURBS-face half-edge carries a FITTED pcurve — \
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
            // Same ground as the harmonic arm, one kind over: a
            // spiric image certifies on its own cutting plane and
            // its own torus and on no spline chart at all, so a
            // cache that reached this lane is a corrupt one. Named
            // rather than folded in, because the two refusals name
            // two different classes.
            Pcurve::Spiric { .. } => {
                return Err(PropsError::QuadratureUnsupported {
                    what: "a NURBS-face half-edge carries a SPIRIC pcurve — a spiric's \
                           chart images live on its own cutting plane and its own \
                           torus, and this chart is a spline patch",
                });
            }
            Pcurve::FocalSection(_) => {
                return Err(PropsError::QuadratureUnsupported {
                    what: "a NURBS-face half-edge carries a FOCAL-SECTION pcurve — that \
                           image certifies on a cone or torus chart only, and this chart \
                           is a spline patch",
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
    let control = about(
        payload
            .control()
            .iter()
            .map(|p| [ring(p.x), ring(p.y), ring(p.z)])
            .collect(),
        centre,
    );
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

/// A patch's lifted control net carried by `−centre`, when the flux is
/// taken about a centre. The net is stored Euclidean, weights apart, so
/// the translated net is the translated patch, rational or not; and the
/// flux pad's position bound, read off the net's hull, becomes the bound
/// on `|x − centre|` it should be.
fn about(control: Vec<quad::RVec3>, centre: Option<Point3<Interval>>) -> Vec<quad::RVec3> {
    match centre {
        None => control,
        Some(c) => control
            .into_iter()
            .map(|x| [x[0] - c.x, x[1] - c.y, x[2] - c.z])
            .collect(),
    }
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
        /// coefficient is refused in its own slot and leaves the rest
        /// intact, so the refusal reaches the flux algebra where it is
        /// visible.
        #[test]
        fn chan_refuses_only_the_violated_coefficient() {
            let one = Interval::from_f64(1.0);
            let c = chan(one, trv_pos(), one, one).expect("channel builds");
            assert!(!c.ca.is_certified(), "the violated coefficient survived");
            for (tag, r) in [("c0", c.c0), ("cb", c.cb), ("cl", c.cl)] {
                assert!(r.is_certified(), "{tag} refused a certified coefficient");
            }
        }
    }

    /// The polygon route of the role read's re-derivation
    /// (`props::shell_polygons`), against exact values: the dyadic sum of
    /// a shell's vertex polygons against its known volume, the interval
    /// sum against that dyadic sum, and a shell with a curved edge kept on
    /// the fan route bit for bit.
    #[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
    mod polygon_tests {
        use geom_brep::props::quad::RoundWindow;
        use geom_core::{Band, Bounds, Interval, Point3, Real, Tol};
        use num_bigint::BigInt;

        use crate::body::Body;
        use crate::entity::{FaceKey, LoopBoundary};
        use crate::props::{
            FaceRun, QuadLane, corner_of, decide_faces_serially, face_flux, face_loops, rederive,
            rederive_about, reporting_hook, resolve_face, shell_polygons, vertex_rings,
        };

        /// `m · 2^e`, exactly.
        #[derive(Clone, Debug)]
        struct Dyadic {
            m: BigInt,
            e: i64,
        }

        impl Dyadic {
            fn of(x: f64) -> Self {
                assert!(x.is_finite(), "a finite coordinate");
                let bits = x.to_bits();
                let sign = if bits >> 63 == 1 { -1 } else { 1 };
                let exp = ((bits >> 52) & 0x7ff) as i64;
                let frac = bits & ((1 << 52) - 1);
                let (mant, e) = if exp == 0 {
                    (frac, -1074)
                } else {
                    (frac | (1 << 52), exp - 1075)
                };
                Self {
                    m: BigInt::from(mant) * sign,
                    e,
                }
            }
            fn add(&self, o: &Self) -> Self {
                let e = self.e.min(o.e);
                let shift = |d: &Self| d.m.clone() << (d.e - e) as usize;
                Self {
                    m: shift(self) + shift(o),
                    e,
                }
            }
            fn neg(&self) -> Self {
                Self {
                    m: -self.m.clone(),
                    e: self.e,
                }
            }
            fn mul(&self, o: &Self) -> Self {
                Self {
                    m: &self.m * &o.m,
                    e: self.e + o.e,
                }
            }
            /// `self ≤ o`.
            fn le(&self, o: &Self) -> bool {
                o.add(&self.neg()).m >= BigInt::from(0)
            }
        }

        type D3 = [Dyadic; 3];

        fn d3(p: Point3<f64>) -> D3 {
            [Dyadic::of(p.x), Dyadic::of(p.y), Dyadic::of(p.z)]
        }
        fn sub(a: &D3, b: &D3) -> D3 {
            [0, 1, 2].map(|i| a[i].add(&b[i].neg()))
        }
        fn cross(a: &D3, b: &D3) -> D3 {
            let c = |i: usize, j: usize| a[i].mul(&b[j]).add(&a[j].mul(&b[i]).neg());
            [c(1, 2), c(2, 0), c(0, 1)]
        }
        fn dot(a: &D3, b: &D3) -> Dyadic {
            a[0].mul(&b[0]).add(&a[1].mul(&b[1])).add(&a[2].mul(&b[2]))
        }

        /// Twice the exact flux of `rings` about `c`, fanned from the
        /// first point.
        fn exact_flux2(rings: &[Vec<Point3<f64>>], c: &D3) -> Dyadic {
            let a = d3(rings[0][0]);
            let mut va = [0.0, 0.0, 0.0].map(Dyadic::of);
            for ring in rings {
                for (i, &p) in ring.iter().enumerate() {
                    let q = ring[(i + 1) % ring.len()];
                    let k = cross(&sub(&d3(p), &a), &sub(&d3(q), &a));
                    va = [0, 1, 2].map(|j| va[j].add(&k[j]));
                }
            }
            dot(&sub(&a, c), &va)
        }

        /// `body`'s faces as `(interval flux, exact doubled flux)` about
        /// its least vertex corner, summed, every face read as its vertex
        /// polygons.
        fn sums(body: &Body<f64>) -> (Interval, Dyadic) {
            let lane = QuadLane::<f64>::certified();
            let points: Vec<_> = body.vertex_points().map(|(_, p)| p).collect();
            let corner = points.iter().fold(points[0], |c, p| {
                Point3::new(c.x.min(p.x), c.y.min(p.y), c.z.min(p.z))
            });
            let centre = Point3::new(
                Interval::from_f64(corner.x),
                Interval::from_f64(corner.y),
                Interval::from_f64(corner.z),
            );
            let mut flux = Interval::zero();
            let mut exact = Dyadic::of(0.0);
            for (key, _) in body.faces() {
                let (face, surface) = resolve_face(body, key);
                let loops = face_loops(body, face).unwrap();
                let rings = vertex_rings(body, face, surface, &loops, lane)
                    .unwrap()
                    .expect("a planar face bounded by lines");
                let f = super::super::polygon_face_about(&rings, centre).unwrap();
                flux = flux + f.flux;
                let rings: Vec<Vec<Point3<f64>>> = rings
                    .iter()
                    .map(|r| {
                        r.iter()
                            .map(|p| Point3::new(p.x.lo(), p.y.lo(), p.z.lo()))
                            .collect()
                    })
                    .collect();
                exact = exact.add(&exact_flux2(&rings, &d3(corner)));
            }
            (flux, exact)
        }

        /// A `2 m × w × w` box along a generic direction from `(2, 1, 1)`.
        fn sliver_box(w: f64, tol: Tol) -> Body<f64> {
            let long = [1.788_854_382, 0.894_427_191, 0.031_25];
            let side = [-0.447_213_595, 0.894_427_191, 0.0];
            let up = [-0.0156, -0.03125, 0.9993];
            crate::test_support::mapped_cube::<f64>(
                move |x, y, z| {
                    let at = |i: usize| {
                        [2.0, 1.0, 1.0][i] + long[i] * x + side[i] * w * y + up[i] * w * z
                    };
                    Point3::new(at(0), at(1), at(2))
                },
                tol,
            )
        }

        /// The exact doubled flux of a shell's vertex polygons is six
        /// times its volume where that is known (a wrong ring order, a
        /// dropped ring or a carrier end read for a vertex breaks it),
        /// and the interval sum holds it.
        #[test]
        fn a_planar_shells_polygon_enclosure_holds_its_exact_volume() {
            let tol = Tol::witness();
            let notch = [(0.0, 0.0), (4.0, 0.0), (4.0, 2.0), (2.0, 1.0), (0.0, 2.0)];
            let tiny = f64::from_bits((1023 - 60) << 52);
            let shells: [(&str, Body<f64>, Option<Dyadic>); 6] = [
                (
                    "a unit brick 5 km out",
                    crate::test_support::brick((5e3, 5e3 + 1.0), (0.0, 1.0), (0.0, 1.0), tol),
                    Some(Dyadic::of(1.0)),
                ),
                (
                    "a brick from −2⁻⁶⁰, whose differences round",
                    crate::test_support::brick((-tiny, 1.0), (0.0, 1.0), (0.0, 1.0), tol),
                    Some(Dyadic::of(1.0).add(&Dyadic::of(tiny))),
                ),
                (
                    "the notch307 prism",
                    crate::test_support::prism::<f64>(&notch, 1.0, tol).body,
                    Some(Dyadic::of(6.0)),
                ),
                (
                    "a 3 × 2 × 2 block with a unit square hole",
                    crate::test_support::holed_block::<f64>(3.0, &[1.5], tol),
                    Some(Dyadic::of(10.0)),
                ),
                ("a 2 m sliver box 1e-4 wide", sliver_box(1e-4, tol), None),
                (
                    "a 2 m sliver box 1e-4 wide, inside out",
                    sliver_box(1e-4, tol).revert(),
                    None,
                ),
            ];
            let holed = shells[3].1.faces().any(|(_, f)| !f.rings.is_empty());
            assert!(holed, "the holed block has a face with an inner ring");
            for (name, body, volume) in &shells {
                let (flux, exact) = sums(body);
                if let Some(volume) = volume {
                    let six = volume.mul(&Dyadic::of(6.0));
                    assert!(
                        exact.le(&six) && six.le(&exact),
                        "{name}: the vertex polygons' doubled flux {exact:?} is six times the volume"
                    );
                }
                let two = Dyadic::of(2.0);
                let lo = Dyadic::of(flux.lo()).mul(&two);
                let hi = Dyadic::of(flux.hi()).mul(&two);
                assert!(
                    flux.is_certified() && lo.le(&exact) && exact.le(&hi),
                    "{name}: the enclosure [{:e}, {:e}] holds the exact flux {exact:?}",
                    flux.lo(),
                    flux.hi()
                );
            }
        }

        /// A brick's walk runs, measured as the role read measures them.
        fn runs(body: &Body<f64>, band: Band, tol: Tol) -> Vec<FaceRun<f64>> {
            let faces: Vec<FaceKey> = body.faces().map(|(k, _)| k).collect();
            let hook = reporting_hook(Some(QuadLane::<f64>::certified()));
            decide_faces_serially(&faces, |&f| {
                face_flux(body, f, band, &hook, tol, RoundWindow::SCHEDULE)
            })
            .unwrap()
        }

        /// **All or nothing.** A brick with a disc planted in its top has
        /// planes bounded by lines and planes bounded by a circle: none of
        /// its faces takes the polygon route, and its re-derivation is the
        /// fan route's, bit for bit. The brick alone takes it.
        #[test]
        fn a_shell_with_a_curved_edge_keeps_the_fan_route_bit_for_bit() {
            let tol = Tol::witness();
            let band = Band::linear(tol).unwrap();
            let lane = QuadLane::<f64>::certified();
            let p = crate::test_support::prism_z::<f64>(
                &[(0.0, 0.0), (4.0, 0.0), (4.0, 4.0), (0.0, 4.0)],
                0.0,
                1.0,
                tol,
            );
            let mut body = p.body;
            assert!(
                shell_polygons(&body, lane, &runs(&body, band, tol))
                    .unwrap()
                    .is_some(),
                "the brick alone is read as its polygons"
            );
            let outer = body.get_face(p.top_face).unwrap().outer;
            let LoopBoundary::Cycle { first } = body.get_loop(outer).unwrap().boundary else {
                panic!("the top's outer loop is a cycle");
            };
            crate::test_support::plant_disc_face(
                &mut body,
                first,
                Point3::new(2.0, 2.0, 1.0),
                1.0,
                tol,
            );
            let runs = runs(&body, band, tol);
            assert!(
                shell_polygons(&body, lane, &runs).unwrap().is_none(),
                "a disc in the top keeps every face off the polygon route"
            );
            let centre = corner_of(&body, lane, &runs).unwrap();
            let bits = |x: Interval| (x.is_certified(), x.lo().to_bits(), x.hi().to_bits());
            for tight in [false, true] {
                let got = rederive(&body, band, tol, lane, &runs, tight).unwrap();
                let fan = rederive_about(&body, band, tol, lane, &runs, Some(centre), tight)
                    .unwrap()
                    .unwrap();
                assert_eq!(
                    (bits(got.volume), bits(got.area), got.recentred),
                    (bits(fan.volume), bits(fan.area), fan.recentred),
                    "tight {tight}: the re-derivation is the fan route's"
                );
            }
        }
    }
}
