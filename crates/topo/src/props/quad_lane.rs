use geom_brep::props::quad::{
    self, Accept, FaceCutBounds, HarmChan, LaneRun, LaneStop, PatchNet, RoundWindow, TrimChord,
    TrimEdgeQ, TrimPiece,
};
use geom_brep::props::{FaceContribution, LoopEdge, PropsError, loop_vector_area};
use geom_brep::props::{rational_rim, spiric};
use geom_brep::{Pcurve, SpiricImage, WallTorus};
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

/// **The trim carrier's kind, as the cylinder lane reads it**: a line, a
/// circle or an ellipse, whose harmonic pcurve the lane integrates; each
/// end's `(cos, sin)` is formed at the limit the lane integrates from
/// ([`quad::trig_at_limit`]), so no vertex is read. The other kinds
/// refuse by name.
fn harmonic_carrier<T: Decide>(carrier: &Curve3<T>) -> Result<(), PropsError> {
    match carrier {
        Curve3::Line { .. } | Curve3::Circle { .. } | Curve3::Ellipse { .. } => Ok(()),
        // The spiric's chart images are not harmonic (its `m`
        // channel is `√((R + r cos v)² − d²)`), so the trig
        // brackets this lane reads do not exist for it. Unreachable
        // by construction: a spiric lies on no cylinder, and a plane or
        // torus chart takes `rim_face` before this lane — the arm
        // names the kind so a gate's removal would meet a typed
        // refusal here rather than a wildcard.
        Curve3::Spiric { .. } => Err(PropsError::QuadratureUnsupported {
            what: "spiric trim carrier on the CYLINDER chart's quadrature lane — a \
                   spiric lies on its plane and its torus only, whose faces take the \
                   spiric lane",
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
/// A face already measured (`quadrature` is its flux and area
/// enclosures and the point `from` they are about) is carried to
/// `centre`: the flux less `(centre − from) · A⃗`, with `A⃗` the face's
/// vector area from its own loops — the same value, at the width it was
/// measured with plus `A⃗`'s times the lever. `rederive` hands a
/// closed-form face here, read about the world origin where it refused
/// about `centre`.
///
/// The second half says whether the face was RECENTRED, its width the
/// body's own: `false` for such a face, and for a closed-form face
/// whose geometry has no translated twin here, whose flux is then the
/// closed form about the world origin less `centre · A⃗`.
pub(super) fn closed_form<T: Decide + geom_core::CertifiedBounds>(
    surface: &Surface<T>,
    loops: &[Vec<LoopEdge<T>>],
    sense: bool,
    band: Band,
    centre: Point3<Interval>,
    quadrature: Option<((Interval, Interval), Point3<Interval>)>,
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
    // A flux about `from` carried to `centre`: less `(centre − from)·A⃗`,
    // which keeps the width of `A⃗` times that lever.
    let about_centre = |flux: Interval, area, from: Point3<Interval>| -> Result<_, PropsError> {
        let va = loops_vector_area(&loops)?;
        Ok((
            FaceContribution {
                flux: flux - (centre - from).dot(va),
                area,
            },
            false,
        ))
    };
    if let Some(((flux, area), from)) = quadrature {
        return about_centre(flux, area, from);
    }
    let surface = surface.map_scalar(Interval::from_certified);
    if let Surface::Plane { normal, .. } = surface {
        return Ok((planar_face_about(&loops, normal, centre)?, true));
    }
    match moved_by(&surface, &loops, centre) {
        Some((surface, moved_loops)) => Ok((
            super::closed_form_of(&surface, &moved_loops, sense, band)?,
            true,
        )),
        None => {
            let about_origin = super::closed_form_of(&surface, &loops, sense, band)?;
            about_centre(about_origin.flux, about_origin.area, Point3::origin())
        }
    }
}

/// A face's loops, each its edges in traversal order.
pub(super) type Loops<U> = Vec<Vec<LoopEdge<U>>>;

/// **A face's geometry carried by `−by`**: its surface and every loop
/// edge's carrier moved, so a closed form read on it is the face's flux
/// about `by`. The one home of reading a closed-form face about a point,
/// at the walk's scalar (`closed_form_about`) and at the interval scalar
/// ([`closed_form`]) alike. `None` for a kind with no analytic datum to
/// move ([`translated_surface`], [`translated_curve`]).
pub(super) fn moved_by<U: Decide>(
    surface: &Surface<U>,
    loops: &[Vec<LoopEdge<U>>],
    by: Point3<U>,
) -> Option<(Surface<U>, Loops<U>)> {
    let loops = loops
        .iter()
        .map(|edges| {
            edges
                .iter()
                .map(|e| {
                    translated_curve(&e.carrier, by).map(|carrier| LoopEdge {
                        carrier,
                        ..e.clone()
                    })
                })
                .collect::<Option<Vec<_>>>()
        })
        .collect::<Option<Vec<_>>>()?;
    Some((translated_surface(surface, by)?, loops))
}

/// A face's vector area `A⃗ = ∫ n dA`, from its loops, each summed about
/// a point of its own (a closed loop's does not depend on it). A
/// spiric-bounded face has no closed form here and refuses; such a face
/// is measured by its own lane, about the centre, and is never carried.
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
pub(super) fn translated_surface<T: Decide>(
    surface: &Surface<T>,
    by: Point3<T>,
) -> Option<Surface<T>> {
    let shift = |p: Point3<T>| Point3::origin() + (p - by);
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
pub(super) fn translated_curve<T: Decide>(curve: &Curve3<T>, by: Point3<T>) -> Option<Curve3<T>> {
    let shift = |p: Point3<T>| Point3::origin() + (p - by);
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
            semi_u,
            semi_v,
            u_ref,
        } => Curve3::Ellipse {
            center: shift(center),
            axis,
            semi_u,
            semi_v,
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
/// scalar: its plane's signed distance from `centre` times its own
/// signed area ([`plane_flux`]), with `A⃗` the face's vector area summed
/// about `anchor`, the first point of its loops, and the distance taken
/// from `anchor` along the stored plane's `normal`.
///
/// `(anchor − centre)·A⃗`, the fan's own flux, reads the same value to
/// first order, but at the width of `A⃗`'s every component times the
/// lever `|anchor − centre|`, the body's whole length: a long, narrow
/// face turned off the axes rounds `A⃗`'s components at its length
/// squared, and that width swamps a thin body's volume
/// (`work/tally/the-corner-fan-volume-of-an-oblique-needle-cancels-the-bodys-extent-cubed.md`).
/// Read along the plane's normal, the distance is as small as the face
/// is near `centre` and the area is the face's own, so the width scales
/// with the face's own flux. The value is the stored geometry's, read
/// off its planes and its faces' polygons: where a loop's points stand
/// off their plane the sum differs from the closed fan's by the in-plane
/// parts the normal drops, which cancel over a closed shell to the
/// order of those offsets times the faces' areas. The carrier's origin
/// is never read, only its normal's direction, and not its sense:
/// distance and area flip together.
fn planar_face_about(
    loops: &[Vec<LoopEdge<Interval>>],
    normal: geom_core::Vec3<Interval>,
    centre: Point3<Interval>,
) -> Result<FaceContribution<Interval>, PropsError> {
    let anchor = loops_anchor(loops)?;
    let mut va = geom_core::Vec3::zero();
    for edges in loops {
        va = va + loop_vector_area(edges, anchor)?;
    }
    Ok(FaceContribution {
        flux: plane_flux(anchor - centre, va, normal) + tilt_slack(anchor - centre, va, normal),
        area: va.norm(),
    })
}

/// **What [`plane_flux`] drops of the loops' own flux**: the loops'
/// flux about the centre is `lever · A⃗` exactly, `lever` the anchor's
/// offset from it, and [`plane_flux`] keeps its part along the plane's
/// normal, `(lever·n̂)(A⃗·n̂)`. The rest is `lever · A⃗⊥`, `A⃗⊥` the vector
/// area's part across the normal — the loops' tilt off the stored plane,
/// their points' stand-off from it. Its size is at most `|lever|·|A⃗⊥|`,
/// returned as the symmetric interval `[−b, b]`, so the face's term
/// encloses the loops' own flux and the shell's sum their volume.
fn tilt_slack(
    lever: geom_core::Vec3<Interval>,
    va: geom_core::Vec3<Interval>,
    normal: geom_core::Vec3<Interval>,
) -> Interval {
    use geom_core::Bounds;
    let across = va - normal * (va.dot(normal) / normal.dot(normal));
    let bound = lever.norm() * across.norm();
    // A refused bracket's ends bound nothing: the slack is then the
    // whole line, and the face's term encloses nothing it can decide.
    if !bound.is_certified() {
        return Interval::from_bounds(f64::NEG_INFINITY, f64::INFINITY);
    }
    let b = bound.hi();
    Interval::from_bounds(-b, b)
}

/// **A plane's flux from a point `lever` off it**: the distance along
/// `normal` times the signed area along it, `(lever·N)(A⃗·N)/(N·N)`,
/// which needs no unit normal and no square root, and is the same for
/// either sense of `N`.
fn plane_flux(
    lever: geom_core::Vec3<Interval>,
    va: geom_core::Vec3<Interval>,
    normal: geom_core::Vec3<Interval>,
) -> Interval {
    lever.dot(normal) * va.dot(normal) / normal.dot(normal)
}

/// A planar face bounded by lines, as the polygons of its vertex points
/// (`rings`, in traversal order) on the plane whose normal is `normal`:
/// its flux about `centre` and its area, at the interval scalar, the
/// plane's distance from `centre` times the rings' signed area
/// ([`plane_flux`], as [`planar_face_about`] reads a face), with `A⃗`
/// the rings fanned from `anchor`, the first point of the first ring.
///
/// Over a shell every face of which is read this way, neighbouring
/// faces' rings meet at the same points. A vertex point is a point
/// interval, so each difference below is one rounding wide, at the
/// difference's own magnitude.
pub(super) fn polygon_face_about(
    rings: &[Vec<Point3<Interval>>],
    normal: geom_core::Vec3<Interval>,
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
        flux: plane_flux(anchor - centre, va, normal) + tilt_slack(anchor - centre, va, normal),
        area: va.norm(),
    })
}

/// **The exact volume of a shell of vertex polygons**, `(1/6) Σ det(r₀,
/// rᵢ, rᵢ₊₁)` over every ring fanned from its own first point: the
/// volume of the closed polyhedron the rings' points span, whatever the
/// planes they were stored on, as an interval a few ulps wide around the
/// exact value (`geom_core::exact::triple_product`, summed without
/// rounding). `None` where a point is not a point interval, or the sum
/// overflows: the stored geometry then has no exact `f64` coordinates
/// to sum.
pub(super) fn exact_polygon_volume(polygons: &[Vec<Vec<Point3<Interval>>>]) -> Option<Interval> {
    use geom_core::Bounds;
    let coords = |p: Point3<Interval>| {
        let c = [p.x, p.y, p.z];
        // A refused bracket is no exact coordinate, whatever its ends.
        c.iter()
            .all(|x| x.is_certified() && x.lo().to_bits() == x.hi().to_bits())
            .then(|| [c[0].lo(), c[1].lo(), c[2].lo()])
    };
    let mut sum = geom_core::exact::Expansion::default();
    for rings in polygons {
        for ring in rings {
            let points: Vec<[f64; 3]> = ring.iter().map(|&p| coords(p)).collect::<Option<_>>()?;
            let Some((&r0, rest)) = points.split_first() else {
                continue;
            };
            for pair in rest.windows(2) {
                sum.add_expansion(&geom_core::exact::triple_product(r0, pair[0], pair[1]));
            }
        }
    }
    if !sum.is_finite() {
        return None;
    }
    let six_volume = sum
        .components()
        .iter()
        .fold(Interval::zero(), |acc, &c| acc + Interval::point(c));
    Some(six_volume / Interval::point(6.0))
}

/// The certified flux/area enclosures of one curved-cut face
/// (module docs of `geom_brep::props::quad`) over a [`RoundWindow`] —
/// the cylinder chart's closed-form lane plus the described-NURBS patch
/// lane (M6-3), entered and left where the window says;
/// [`super::QuadLane`] holds this. Each round is asked one question,
/// `resolution`: a round whose mean boundary displacement
/// ([`quad::mean_boundary_displacement`]) is at most that length ends the
/// call there. It is a sizing, so it is compared, not decided, and
/// nothing is recorded; `None` accepts no round, so the call ends at the
/// window's last round or the lane's own stop. A cone face never arrives:
/// the face walk routes it to its closed form
/// ([`geom_brep::props::cone_face_closed_form`]). A plane or torus face
/// reaches this lane only with a spiric on its boundary
/// ([`rim_face`]); a sphere chart mints stored pcurves but has no
/// flux lane here, and refuses typed.
///
/// The tolerance is the hook's ([`super::QuadLane`]): no lane here reads
/// it, since each trim end's trig is formed at its parameter, not off its
/// vertex within `ε` ([`harmonic_carrier`]).
#[allow(clippy::too_many_arguments)]
pub(super) fn cut_face_rounds<T: Decide + Bounds + CertifiedEnclosure>(
    body: &Body<T>,
    surface: &Surface<T>,
    loops: &[super::FlatLoop<T>],
    band: Band,
    _tol: Tol,
    window: RoundWindow,
    centre: Option<Point3<Interval>>,
    resolution: Option<f64>,
) -> Result<LaneRun, PropsError> {
    let mut accept = |bounds: &FaceCutBounds, _: LaneStop| match resolution {
        None => Ok(false),
        Some(len) => Ok(quad::mean_boundary_displacement(bounds.flux, bounds.area)? <= len),
    };
    cut_face_lane(body, surface, loops, band, window, centre, &mut accept)
}

/// [`cut_face_rounds`]' lanes, asking `accept` of every round.
fn cut_face_lane<T: Decide + Bounds + CertifiedEnclosure>(
    body: &Body<T>,
    surface: &Surface<T>,
    loops: &[super::FlatLoop<T>],
    band: Band,
    window: RoundWindow,
    centre: Option<Point3<Interval>>,
    accept: &mut Accept<'_>,
) -> Result<LaneRun, PropsError> {
    // The NURBS-patch lane (M6-3): a described NURBS face routes
    // to the patch engine over its stored iso-line pcurves.
    // The spline-patch lane (M6-3): a described spline face routes
    // to the patch engine over its stored iso-line pcurves. An
    // approximating face enters on its fit — the certificate's
    // bound is a statement about the DESCRIPTION and does not
    // widen this quadrature (the same deliberate omission the
    // mesh tolerance makes).
    if let Some(payload) = surface.spline_chart() {
        // The patch lanes read one loop; the face walk refuses a ring on
        // a spline face before this (`RingOnCurvedFace`), and this
        // refuses it again rather than drop it.
        let [(outer, hes)] = loops else {
            return Err(PropsError::QuadratureUnsupported {
                what: "a ring on a described spline face — the patch lanes read one loop",
            });
        };
        return nurbs_face(body, payload, outer, hes, band, window, centre, accept);
    }
    // The structural curved-rim arm. A spiric bounds a plane cap or a
    // torus wall and nothing else (`Pcurve::Spiric` certifies on those two
    // chart kinds only); the face walk routes a plane or a cone here only
    // when a spiric or a rational spline rim is on its boundary, and a
    // torus wall comes here when a spiric is on any of its loops, its
    // outer loop or a ring.
    if matches!(surface, Surface::Plane { .. } | Surface::Cone { .. })
        || (matches!(surface, Surface::Torus { .. })
            && loops
                .iter()
                .flat_map(|(edges, _)| edges)
                .any(|e| matches!(e.carrier, Curve3::Spiric { .. })))
    {
        return rim_face(body, surface, loops, band, window, centre, accept);
    }
    let Surface::Cylinder { origin, radius, .. } = surface else {
        return Err(PropsError::QuadratureUnsupported {
            what: "conic trim on a cone/sphere/torus chart — a cone face takes its \
                   closed form before this lane (`cone_face_closed_form`); sphere and \
                   torus charts mint stored pcurves, but this lane's chart-normal flux \
                   algebra is the cylinder's",
        });
    };
    let mut va = geom_core::Vec3::new(T::zero(), T::zero(), T::zero());
    for (edges, _) in loops {
        va = va + loop_vector_area(edges, *origin)?;
    }
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
    let mut trims = Vec::with_capacity(loops.len());
    for (loop_edges, hes) in loops {
        let mut edges = Vec::with_capacity(loop_edges.len());
        let lifted = crate::pcurves::lifted_images(body, hes);
        for ((le, he), lifted) in loop_edges.iter().zip(hes).zip(&lifted) {
            let (Some(cache), Some(image)) = (body.pcurve(*he), lifted) else {
                return Err(PropsError::QuadratureUnsupported {
                    what: "curved-cut face half-edge carries no stored pcurve cache — \
                       caches mint in the split/boolean pipelines",
                });
            };
            // The certified quadrature lane reads a chart image
            // CHANNEL BY CHANNEL out of its harmonic form; a fitted,
            // projected or focal-section image has none on an ANALYTIC
            // chart's Green reduction, and refuses typed. A sphere's general
            // circle mints a projected one at rest (an oblique fillet
            // corner's octant); the props door refuses that face's spherical
            // triangle before this lane is asked.
            let Some((p0, pa, pb, pl)) = image.harmonic_form() else {
                return Err(PropsError::QuadratureUnsupported {
                    what: "curved-cut face half-edge carries a pcurve with no harmonic form on \
                       an analytic chart (a FITTED or PROJECTED image, or a cone section's or \
                       Villarceau circle's focal section) — its Green-form boundary integral \
                       is not wired",
                });
            };
            let (t0, t1) = cache.params();
            let (t0, t1) = (Interval::from_certified(t0), Interval::from_certified(t1));
            harmonic_carrier(&le.carrier)?;
            let trig0 = quad::trig_at_limit(t0);
            let trig1 = Some(quad::trig_at_limit(t1));
            edges.push(TrimEdgeQ {
                u: chan(p0.x, pa.x, pb.x, pl.x)?,
                v: chan(p0.y, pa.y, pb.y, pl.y)?,
                t0,
                t1,
                forward: le.forward,
                trig0,
                trig1,
                env: Interval::from_certified(cache.certificate().envelope),
            });
        }
        trims.push(edges);
    }
    let trims: Vec<&[TrimEdgeQ]> = trims.iter().map(Vec::as_slice).collect();
    quad::cylinder_cut_face_rounds::<T>(
        Interval::from_certified(*radius),
        o_dot_va,
        &trims,
        band,
        window,
        accept,
    )
}

/// An upper bound on an edge's length: a line's speed `|dir|`, an arc's radius (an
/// ellipse's larger semi-axis) times its span, a spiric's piecewise
/// [`spiric::spiric_length_bound`], a B-spline's speed sup times its span.
fn length_bound(e: &LoopEdge<Interval>) -> Result<f64, PropsError> {
    let span = (e.t1 - e.t0).mag();
    Ok(match e.carrier {
        Curve3::Line { dir, .. } => (dir.norm() * Interval::point(span)).mag(),
        Curve3::Circle { radius, .. } => (radius * Interval::point(span)).mag(),
        Curve3::Ellipse { semi_u, semi_v, .. } => {
            (geom::Curve3::semi_axes_by_size(semi_u, semi_v).0 * Interval::point(span)).mag()
        }
        Curve3::Spiric { .. } => spiric_edge_q(e, Point3::origin(), None)
            .and_then(|q| spiric::spiric_length_bound(&q, spiric::LENGTH_PIECES))
            .ok_or(PropsError::QuadratureUnsupported {
                what: "a spiric edge with a piece whose f is not certified positive",
            })?,
        // The carrier's certified speed sup over its whole domain, crossed
        // to metres over the edge's span.
        Curve3::Nurbs(ref c) => {
            let len = c.speed_upper_bound().to_meters(Interval::point(span));
            if !len.is_certified() {
                return Err(PropsError::QuadratureUnsupported {
                    what: "a B-spline rim whose speed bound is not certified",
                });
            }
            len.mag()
        }
    })
}

fn rv(p: geom_core::Vec3<Interval>) -> quad::RVec3 {
    [p.x, p.y, p.z]
}

/// One spiric edge for [`geom_brep::props::spiric`], its centre taken
/// relative to `reference`; `None` for any other carrier.
fn spiric_edge_q(
    e: &LoopEdge<Interval>,
    reference: Point3<Interval>,
    wall: Option<spiric::WallImageQ>,
) -> Option<spiric::SpiricEdgeQ> {
    let Curve3::Spiric {
        center,
        axis,
        u_ref,
        major_radius,
        minor_radius,
        offset,
    } = e.carrier
    else {
        return None;
    };
    Some(spiric::SpiricEdgeQ {
        centre: rv(center - reference),
        axis: rv(axis),
        normal: rv(u_ref),
        major: major_radius,
        minor: minor_radius,
        offset,
        t0: e.t0,
        t1: e.t1,
        forward: e.forward,
        wall,
    })
}

/// One lifted chart image of a torus wall's half-edge, read at the
/// interval scalar: what [`spiric_wall`] classifies.
enum WallImage {
    /// A spiric's `SpiricImage::Wall` and the radii its image carries.
    Spiric {
        v0: Interval,
        sense: Interval,
        torus: WallTorus,
        major: Interval,
        minor: Interval,
    },
    /// A harmonic image's four coefficient pairs `p0, pa, pb, pl`.
    Harmonic([(Interval, Interval); 4]),
    /// Any other image.
    Other,
}

/// A wall half-edge as [`spiric_wall`] reads it: its carrier, its
/// lifted image, where its traversal starts and ends on that image
/// `(u, v)`, its certificate envelope and its carrier span.
struct WallHalf {
    edge: LoopEdge<Interval>,
    image: WallImage,
    ends: [(Interval, Interval); 2],
    env: Interval,
    span: Interval,
}

/// **The curved-rim face lane**: a plane cap with a `Curve3::Spiric` or a
/// rational spline rim on its boundary, or a torus wall with a spiric on
/// its boundary, read at the interval scalar and handed to
/// [`spiric::rim_face_rounds`] (its module docs carry the formulation,
/// [`geom_brep::props::rational_rim`]'s a rational rim's). Reached only
/// from [`cut_face_rounds`]'s structural curved-rim arm, under
/// `QuadLane::certified`, so it is never formed at a
/// dual; the face walk hands it every loop of the face as it read them,
/// the outer loop first.
fn rim_face<T: Decide + geom_core::CertifiedBounds>(
    body: &Body<T>,
    surface: &Surface<T>,
    loops: &[super::FlatLoop<T>],
    band: Band,
    window: RoundWindow,
    centre: Option<Point3<Interval>>,
    accept: &mut Accept<'_>,
) -> Result<LaneRun, PropsError> {
    let centre = centre.unwrap_or_else(Point3::origin);
    let lift = |e: &LoopEdge<T>| LoopEdge {
        carrier: e.carrier.map_scalar(Interval::from_certified),
        carrier_id: e.carrier_id,
        t0: Interval::from_certified(e.t0),
        t1: Interval::from_certified(e.t1),
        forward: e.forward,
        start: e.start,
        end: e.end,
    };
    let iv = Interval::from_certified;
    let face = match surface {
        Surface::Plane { .. } => {
            let loops: Vec<Vec<LoopEdge<Interval>>> = loops
                .iter()
                .map(|(edges, _)| edges.iter().map(lift).collect())
                .collect();
            plane_cap(&loops, centre)?
        }
        Surface::Cone {
            apex,
            axis,
            half_angle,
            ..
        } => {
            let loops: Vec<Vec<LoopEdge<Interval>>> = loops
                .iter()
                .map(|(edges, _)| edges.iter().map(lift).collect())
                .collect();
            let apex = Point3::new(iv(apex.x), iv(apex.y), iv(apex.z));
            let axis = geom_core::Vec3::new(iv(axis.x), iv(axis.y), iv(axis.z));
            let axis = axis / (axis.x.sqr() + axis.y.sqr() + axis.z.sqr()).sqrt();
            // Lane evaluation at `T`, crossed in through its bracket.
            let sin_a = iv(half_angle.sin());
            let face = cap_about(&loops, apex, centre)?;
            return spiric::cone_cap_rounds::<T>(&face, (rv(axis), sin_a), band, window, accept);
        }
        Surface::Torus {
            center,
            major_radius,
            minor_radius,
            ..
        } => {
            // Every loop's half-edges through their lifted images, loop by
            // loop: a ring's are read as the outer loop's are.
            let mut walls = Vec::with_capacity(loops.len());
            for (edges, hes) in loops {
                let lifted = crate::pcurves::lifted_images(body, hes);
                let mut halves = Vec::with_capacity(edges.len());
                for ((le, he), image) in edges.iter().zip(hes).zip(&lifted) {
                    let (Some(cache), Some(image)) = (body.pcurve(*he), image) else {
                        return Err(PropsError::QuadratureUnsupported {
                            what: "a spiric-bounded torus wall's half-edge carries no lifted \
                               pcurve — the offset-axial door mints them",
                        });
                    };
                    let (t0, t1) = cache.params();
                    let pair = |p: geom_core::Point2<T>| (iv(p.x), iv(p.y));
                    let vec = |v: geom_core::Vec2<T>| (iv(v.x), iv(v.y));
                    // The traversal's ends through the image's own evaluator,
                    // on the lane, crossed into certification arithmetic.
                    let (a, b) = (pair(image.eval(t0)), pair(image.eval(t1)));
                    halves.push(WallHalf {
                        edge: lift(le),
                        image: match image {
                            Pcurve::Spiric {
                                major,
                                minor,
                                image:
                                    SpiricImage::Wall {
                                        v0, sense, torus, ..
                                    },
                                ..
                            } => WallImage::Spiric {
                                v0: iv(*v0),
                                sense: iv(*sense),
                                torus: *torus,
                                major: iv(*major),
                                minor: iv(*minor),
                            },
                            Pcurve::Harmonic { u, v } => {
                                let (p0, pa, pb, pl) = geom_brep::harmonic_form_of(*u, *v);
                                WallImage::Harmonic([pair(p0), vec(pa), vec(pb), vec(pl)])
                            }
                            _ => WallImage::Other,
                        },
                        ends: if le.forward { [a, b] } else { [b, a] },
                        env: iv(cache.certificate().envelope),
                        span: iv(t1) - iv(t0),
                    });
                }
                walls.push(halves);
            }
            let c = Point3::new(iv(center.x), iv(center.y), iv(center.z));
            spiric_wall(&walls, c, iv(*major_radius), iv(*minor_radius), centre)?
        }
        _ => {
            return Err(PropsError::QuadratureUnsupported {
                what: "a spiric on a chart other than its plane or its torus",
            });
        }
    };
    spiric::rim_face_rounds::<T>(&face, band, window, accept)
}

/// A loop set's anchor: the first carrier's start point — the convention
/// [`planar_face_about`] and [`plane_cap`] share, so a plane's flux
/// lever is a point of the face whichever lane reads it.
fn loops_anchor(loops: &[Vec<LoopEdge<Interval>>]) -> Result<Point3<Interval>, PropsError> {
    loops
        .first()
        .and_then(|edges| edges.first())
        .map(|e| e.carrier.eval(e.t0))
        .ok_or(PropsError::DegenerateFace)
}

/// A plane cap bounded by a spiric or a rational spline rim: every loop
/// through its carriers, as the closed-form plane reads it — the
/// closed-form edges' vector-area terms about the anchor, each spiric's
/// and each rational rim's through the lane ([`rational_rim`]), the flux
/// the anchor less `centre` dotted with `A⃗`.
fn plane_cap(
    loops: &[Vec<LoopEdge<Interval>>],
    centre: Point3<Interval>,
) -> Result<spiric::RimFaceQ, PropsError> {
    cap_about(loops, loops_anchor(loops)?, centre)
}

/// A cap's loops read about `anchor`: [`plane_cap`]'s reading, taken
/// about a point of a plane, or about a cone's apex for
/// [`spiric::cone_cap_rounds`].
fn cap_about(
    loops: &[Vec<LoopEdge<Interval>>],
    anchor: Point3<Interval>,
    centre: Point3<Interval>,
) -> Result<spiric::RimFaceQ, PropsError> {
    let mut closed_va = [Interval::zero(); 3];
    let mut spirics = Vec::new();
    let mut rationals = Vec::new();
    let mut perimeter = 0.0;
    for e in loops.iter().flatten() {
        perimeter += length_bound(e)?;
        if let Some(q) = spiric_edge_q(e, anchor, None) {
            spirics.push(q);
        } else if let Curve3::Nurbs(c) = &e.carrier
            && geom_brep::props::is_rational(c)
        {
            rationals.push(rational_rim::rational_edge_q(
                c, e.t0, e.t1, e.forward, anchor,
            )?);
        } else {
            let va = rv(loop_vector_area(core::slice::from_ref(e), anchor)?);
            closed_va = [0, 1, 2].map(|k| closed_va[k] + va[k]);
        }
    }
    Ok(spiric::RimFaceQ {
        rationals,
        closed_va,
        spirics,
        lever: rv(anchor - centre),
        perimeter,
        wall: None,
    })
}

/// A harmonic coefficient read as exact `f64` structure (C6): `Some`
/// only for a point bracket.
fn exact(x: Interval) -> Option<f64> {
    (x.is_certified() && x.lo() == x.hi()).then_some(x.lo())
}

/// A spiric-bounded torus wall: each of its loops through the LIFTED
/// chart images, the outer loop and every ring alike, its edges' terms
/// summed over them all and its closure decided loop by loop
/// (`spiric::WallChartQ::ends`) — a spiric's exact `SpiricImage::Wall`, each other edge's
/// harmonic image, which must be iso: a rim (`v` fixed, `u` linear) or a
/// meridian (`u` fixed, `v` linear), read as exact structure. The
/// spiric's own radii, on its carrier and its image, must be the torus's
/// stored brackets end for end, so the wall terms read one `R` and one
/// `r`. Any other image refuses typed.
fn spiric_wall(
    loops: &[Vec<WallHalf>],
    c: Point3<Interval>,
    major: Interval,
    minor: Interval,
    centre: Point3<Interval>,
) -> Result<spiric::RimFaceQ, PropsError> {
    let unsupported = |what| PropsError::QuadratureUnsupported { what };
    let mut chart = spiric::WallChartQ {
        major,
        minor,
        rims: Vec::new(),
        meridians: Vec::new(),
        ends: Vec::with_capacity(loops.len()),
        parts: Vec::with_capacity(loops.len()),
    };
    let mut closed_va = [Interval::zero(); 3];
    let mut spirics = Vec::new();
    let mut perimeter = 0.0;
    for halves in loops {
        let held = (spirics.len(), chart.rims.len(), chart.meridians.len());
        let mut ends = Vec::with_capacity(halves.len());
        for h in halves {
            let e = &h.edge;
            perimeter += length_bound(e)?;
            let signed = |x: Interval| if e.forward { x } else { -x };
            match (&h.image, &e.carrier) {
                (
                    WallImage::Spiric {
                        v0,
                        sense,
                        torus,
                        major: image_major,
                        minor: image_minor,
                    },
                    Curve3::Spiric {
                        major_radius,
                        minor_radius,
                        ..
                    },
                ) => {
                    // The same stored datum: one bracket, end for end.
                    let same = |a: Interval, b: Interval| {
                        a.is_certified() && b.is_certified() && a.lo() == b.lo() && a.hi() == b.hi()
                    };
                    if !(same(*major_radius, major)
                        && same(*minor_radius, minor)
                        && same(*image_major, major)
                        && same(*image_minor, minor))
                    {
                        return Err(unsupported(
                            "a spiric wall edge whose carrier or image radii are not exactly \
                         its torus's",
                        ));
                    }
                    // The wall terms (`spiric::WallImageQ`) read the
                    // carrier's OWN torus; a wall on its mirror through the
                    // cutting plane refuses typed until they read both
                    // (FLUX's `the-spiric-wall-quadrature-lane-reads-only-
                    // the-own-torus`).
                    if *torus == WallTorus::Mirror {
                        return Err(unsupported(
                            "a spiric wall image on the carrier's mirror torus — the wall \
                         terms read the carrier's own torus",
                        ));
                    }
                    let sense = match exact(*sense) {
                        Some(s) if s == 1.0 || s == -1.0 => s,
                        _ => {
                            return Err(unsupported(
                                "a spiric wall image whose sense is not exactly ±1",
                            ));
                        }
                    };
                    let wall = spiric::WallImageQ {
                        sense,
                        v0: *v0,
                        env: h.env,
                    };
                    let q = spiric_edge_q(e, c, Some(wall)).ok_or_else(|| {
                        unsupported("a spiric wall image on an edge whose carrier is no spiric")
                    })?;
                    ends.push(spiric::ChartEndsQ {
                        start: h.ends[0],
                        end: h.ends[1],
                        env: h.env,
                    });
                    spirics.push(q);
                    continue;
                }
                (WallImage::Harmonic([p0, pa, pb, pl]), _) => {
                    let zero = |x: Interval| exact(x) == Some(0.0);
                    if !(zero(pa.0) && zero(pa.1) && zero(pb.0) && zero(pb.1)) {
                        return Err(unsupported(
                            "a torus wall edge beside a spiric whose harmonic image is not a \
                         line of its chart — the spiric lane reads rims and meridians",
                        ));
                    }
                    ends.push(spiric::ChartEndsQ {
                        start: h.ends[0],
                        end: h.ends[1],
                        env: h.env,
                    });
                    if zero(pl.1) {
                        chart.rims.push(spiric::RimQ {
                            v: p0.1,
                            du: signed(pl.0 * h.span),
                            env: h.env,
                        });
                    } else if zero(pl.0) {
                        chart.meridians.push(spiric::MeridianQ {
                            dv: signed(pl.1 * h.span),
                            env: h.env,
                        });
                    } else {
                        return Err(unsupported(
                            "a torus wall edge beside a spiric whose harmonic image is neither \
                         a rim nor a meridian",
                        ));
                    }
                }
                _ => {
                    return Err(unsupported(
                        "a torus wall edge beside a spiric whose lifted image is neither a \
                     spiric wall image nor a harmonic rim or meridian",
                    ));
                }
            }
            let va = rv(loop_vector_area(core::slice::from_ref(e), c)?);
            closed_va = [0, 1, 2].map(|k| closed_va[k] + va[k]);
        }
        chart.ends.push(ends);
        chart.parts.push(spiric::WallLoopParts {
            spirics: spirics.len() - held.0,
            rims: chart.rims.len() - held.1,
            meridians: chart.meridians.len() - held.2,
        });
    }
    Ok(spiric::RimFaceQ {
        rationals: Vec::new(),
        closed_va,
        spirics,
        lever: rv(c - centre),
        perimeter,
        wall: Some(chart),
    })
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
/// cut-loft unit's). The direction the traversal walks its rectangle
/// ([`rectangle_winding`]) IS the S10 orientation input —
/// winding-derived end to end, like the cylinder lane; no sense bit is
/// read.
#[allow(clippy::too_many_arguments)]
fn nurbs_face<T: Decide + Bounds + CertifiedEnclosure>(
    body: &Body<T>,
    payload: &geom::NurbsSurface<T>,
    outer: &[LoopEdge<T>],
    hes: &[HalfEdgeKey],
    band: Band,
    window: RoundWindow,
    centre: Option<Point3<Interval>>,
    accept: &mut Accept<'_>,
) -> Result<LaneRun, PropsError> {
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
        return trimmed_face(body, payload, outer, hes, band, window, centre, accept);
    }
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
    let mut segments: Vec<ChartSegment> = Vec::with_capacity(outer.len());
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
            segments.push(((ax, ay), (bx, by)));
        } else {
            segments.push(((bx, by), (ax, ay)));
        }
        // Metric boundary length bound + the map-residual defect.
        let len = carrier_metric_length(&le.carrier, t0, t1)?;
        perimeter += len;
        boundary_defect += len * Interval::from_certified(cache.certificate().envelope).mag();
    }
    // The rectangle certificate: hull of the segments' starts, every
    // start on its boundary, and the segments walking that boundary
    // once — the direction IS the S10 winding.
    let (mut u0, mut u1) = (f64::INFINITY, f64::NEG_INFINITY);
    let (mut v0, mut v1) = (f64::INFINITY, f64::NEG_INFINITY);
    for &((x, y), _) in &segments {
        u0 = u0.min(x);
        u1 = u1.max(x);
        v0 = v0.min(y);
        v1 = v1.max(y);
    }
    for &((x, y), _) in &segments {
        if (x != u0 && x != u1) && (y != v0 && y != v1) {
            return Err(PropsError::QuadratureUnsupported {
                what: "a NURBS-face boundary vertex sits strictly inside the UV \
                       rectangle — a re-entrant trim is outside the rectangle lane \
                       (the cut-loft unit's)",
            });
        }
    }
    let winding = rectangle_winding(&segments, (u0, u1, v0, v1)).ok_or(
        PropsError::QuadratureUnsupported {
            what: "the NURBS-face boundary does not traverse its UV rectangle exactly \
                   once — a trimmed or multiply-wound region is outside the rectangle \
                   lane (the cut-loft unit's)",
        },
    )?;
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
    let net = PatchNet::new(
        payload.knots_u_f64_interim(),
        payload.knots_v_f64_interim(),
        &control,
        payload.weights_f64_interim(),
    )?;
    let out = quad::nurbs_patch_face_rounds::<T>(
        &net,
        (u0, u1, v0, v1),
        perimeter,
        boundary_defect,
        band,
        window,
        accept,
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

/// One trim edge's chart image as `(start, end)` in traversal order.
type ChartSegment = ((f64, f64), (f64, f64));

/// Whether the trim's iso segments, each `(start, end)` in traversal
/// order, walk the boundary of the rectangle `(u0, u1, v0, v1)` exactly
/// once: `+1.0` counter-clockwise, `−1.0` clockwise, `None` otherwise.
/// Read by comparisons alone, so a rectangle whose sides are not dyadic
/// (a sub-patch at a knot of `1/9`) certifies as exactly as `[0, 1]²`,
/// which a shoelace in `f64` cannot promise.
///
/// A once-around walk is closed and connected, each segment starting
/// where the one before it ended. Each segment is axis-aligned, so it
/// lies on one side of the rectangle (or inside it, which refuses) and
/// runs one way along it. A once-around walk runs every moving segment
/// the same way round and visits the four sides in their order, each in
/// one run.
fn rectangle_winding(
    segments: &[ChartSegment],
    (u0, u1, v0, v1): (f64, f64, f64, f64),
) -> Option<f64> {
    if u0 >= u1 || v0 >= v1 {
        return None;
    }
    // Each moving segment's side (bottom, right, top, left: the
    // counter-clockwise order) and whether it runs counter-clockwise.
    let mut sides = Vec::with_capacity(segments.len());
    let mut ccw = None;
    // Each segment starts where the one before it ended. The two are one
    // vertex's images on two iso pcurves, each the affine `p₀ + l·t` at
    // an end parameter, two roundings of terms no larger than the
    // rectangle's coordinates: they agree within a few ulps of those,
    // and a wider gap is two vertices.
    let slack = |a: f64, b: f64| 4.0 * f64::EPSILON * a.abs().max(b.abs());
    let (su, sv) = (slack(u0, u1), slack(v0, v1));
    let joined = segments
        .iter()
        .zip(segments.iter().cycle().skip(1))
        .all(|(&(_, (ex, ey)), &((sx, sy), _))| (ex - sx).abs() <= su && (ey - sy).abs() <= sv);
    if !joined {
        return None;
    }
    for &((ax, ay), (bx, by)) in segments {
        let (side, forward) = if ay == by && ax != bx {
            match ay {
                y if y == v0 => (0u8, bx > ax),
                y if y == v1 => (2, bx < ax),
                _ => return None,
            }
        } else if ax == bx && ay != by {
            match ax {
                x if x == u1 => (1, by > ay),
                x if x == u0 => (3, by < ay),
                _ => return None,
            }
        } else if ax == bx && ay == by {
            // A segment whose image is one chart point walks no way.
            continue;
        } else {
            return None;
        };
        if *ccw.get_or_insert(forward) != forward {
            return None;
        }
        if sides.last() != Some(&side) {
            sides.push(side);
        }
    }
    if sides.len() > 1 && sides.first() == sides.last() {
        sides.pop();
    }
    let ccw = ccw?;
    let next = |s: u8| if ccw { (s + 1) % 4 } else { (s + 3) % 4 };
    let once = sides.len() == 4 && (0..4).all(|i| sides[(i + 1) % 4] == next(sides[i]));
    once.then_some(if ccw { 1.0 } else { -1.0 })
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
    window: RoundWindow,
    centre: Option<Point3<Interval>>,
    accept: &mut Accept<'_>,
) -> Result<LaneRun, PropsError> {
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
                // Two producers at rest store a sub-range, both filed
                // as
                // `work/quad/a-fitted-cap-cut-by-planes-has-a-sub-range-trim-image.md`:
                // an offset fitted face cut by planes (`replace_face`'s
                // derived plane × fit sections, each spanning the fit's
                // window while its edge spans part of it), pinned by
                // `encl_curved_loft_shell::a_moved_fitted_cap_stands_its_corners_on_the_held_sides`;
                // and `split_edge` on a seam holding a `General` row,
                // which carries the parent's image to each child over
                // the child's part of it, pinned by `sweep`'s
                // `split_edge_fitted_rows::the_mint_rederives_the_carried_rows_and_the_trimmed_lane_refuses_their_sub_range`.
                // The trimmed lane subdivides the STORED image whole,
                // so a sub-range would get a certified number for
                // chart the face does not bound; it refuses instead.
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
                Some(TrimPiece::new(
                    image.knots_f64_interim().clone(),
                    image
                        .control()
                        .iter()
                        .map(|p| (ring(p.x), ring(p.y)))
                        .collect(),
                    image.weights_f64_interim().to_vec(),
                )?)
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
            // the two tori that hold it and on no spline chart at all, so a
            // cache that reached this lane is a corrupt one. Named
            // rather than folded in, because the two refusals name
            // two different classes.
            Pcurve::Spiric { .. } => {
                return Err(PropsError::QuadratureUnsupported {
                    what: "a NURBS-face half-edge carries a SPIRIC pcurve — a spiric's \
                           chart images live on its own cutting plane and the two tori \
                           that hold it, and this chart is a spline patch",
                });
            }
            Pcurve::FocalSection(_) => {
                return Err(PropsError::QuadratureUnsupported {
                    what: "a NURBS-face half-edge carries a FOCAL-SECTION pcurve — that \
                           image certifies on a cone or torus chart only, and this chart \
                           is a spline patch",
                });
            }
            Pcurve::Projected(_) => {
                return Err(PropsError::QuadratureUnsupported {
                    what: "a NURBS-face half-edge carries a PROJECTED pcurve — that image \
                           certifies on an analytic chart only, and this chart is a spline \
                           patch",
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
    let net = PatchNet::new(
        payload.knots_u_f64_interim(),
        payload.knots_v_f64_interim(),
        &control,
        payload.weights_f64_interim(),
    )?;
    quad::trimmed_patch_face_rounds::<T>(&net, &chords, band, window, accept)
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

#[cfg(test)]
mod tests {
    /// The plane cap ([`super::plane_cap`]) over hand-built loops: a rim
    /// whose kinds mix, and the length bound's spline arm.
    #[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
    mod plane_cap_tests {
        use geom::Curve3;
        use geom_brep::props::LoopEdge;
        use geom_brep::props::rational_rim::rational_vector_area_split;
        use geom_core::interval::certification::Certification as _;
        use geom_core::spline::KnotVector;
        use geom_core::{Bounds as _, Interval, Point3, Vec3};

        use super::super::{length_bound, plane_cap};

        fn iv(x: f64) -> Interval {
            Interval::point(x)
        }

        fn ip(x: f64, y: f64) -> Point3<Interval> {
            Point3::new(iv(x), iv(y), iv(0.0))
        }

        fn line(a: Point3<Interval>, b: Point3<Interval>, tags: (u32, u32)) -> LoopEdge<Interval> {
            LoopEdge::hand_built(
                Curve3::Line {
                    origin: a,
                    dir: b - a,
                },
                iv(0.0),
                iv(1.0),
                true,
                tags.0,
                tags.1,
            )
        }

        fn nurbs(
            knots: Vec<f64>,
            degree: usize,
            pts: Vec<Point3<Interval>>,
            w: &[f64],
        ) -> Curve3<Interval> {
            Curve3::Nurbs(std::sync::Arc::new(
                geom::NurbsCurve3::new(
                    geom_core::spline::Knots::from(KnotVector::clamped(knots, degree).unwrap())
                        .lift(geom_core::Real::from_f64),
                    pts,
                    w.iter().map(|&x| iv(x)).collect(),
                )
                .unwrap(),
            ))
        }

        /// **A cap whose rim mixes lines and a rational arc measures**: the
        /// quarter disc of radius 2, bounded by the line from its centre
        /// to `(2, 0)`, the quarter circle to `(0, 2)` as a rational
        /// quadratic, and the line back. The lines' terms enter in closed
        /// form, the arc's through the certified lane, about one anchor,
        /// and the sum holds the sector's vector area `πR²/4 ẑ`.
        #[test]
        fn a_cap_bounded_by_lines_and_a_rational_arc_holds_its_sector() {
            let r = 2.0;
            let (o, a, b) = (ip(0.0, 0.0), ip(r, 0.0), ip(0.0, r));
            let arc = LoopEdge::hand_built(
                nurbs(
                    vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
                    2,
                    vec![a, ip(r, r), b],
                    &[1.0, core::f64::consts::FRAC_1_SQRT_2, 1.0],
                ),
                iv(0.0),
                iv(1.0),
                true,
                1,
                2,
            );
            let loops = vec![vec![line(o, a, (0, 1)), arc, line(b, o, (2, 0))]];
            let face = plane_cap(&loops, Point3::origin()).unwrap();
            assert!(
                face.spirics.is_empty() && face.rationals.len() == 1,
                "two closed-form lines and one rational rim"
            );
            let (arc_va, _) = rational_vector_area_split(&face.rationals[0], 256);
            let va: [Interval; 3] = core::array::from_fn(|k| face.closed_va[k] + arc_va[k]);
            let truth = core::f64::consts::PI * r * r / 4.0;
            assert!(
                va[0].mag() == 0.0 && va[1].mag() == 0.0,
                "the sector lies in z = 0: {va:?}"
            );
            assert!(
                va[2].is_certified() && va[2].lo() <= truth && truth <= va[2].hi(),
                "{:?} ∌ πR²/4 = {truth}",
                va[2]
            );
        }

        /// **The length bound reads a spline edge**: a polynomial spline,
        /// the chord of a spiric cap stated as a degree-1 B-spline, is
        /// bounded by its speed's supremum times its span, where before
        /// this lane read rational rims the bound refused any spline, and
        /// with it every spiric cap a spline edge closed. The cap is the
        /// census's spiric cap (the torus `R = 2, r = 1` in `x = ½`, over
        /// `[−π/2, π/2]`) closed by that spline; it now forms, its spline
        /// read in closed form and its perimeter at least its chord's.
        #[test]
        fn the_length_bound_reads_a_spline_edge_so_a_spiric_cap_it_closes_forms() {
            use core::f64::consts::FRAC_PI_2;
            let spiric = Curve3::Spiric {
                center: Point3::new(iv(0.0), iv(0.0), iv(0.0)),
                axis: Vec3::new(iv(0.0), iv(0.0), iv(1.0)),
                u_ref: Vec3::new(iv(1.0), iv(0.0), iv(0.0)),
                major_radius: iv(2.0),
                minor_radius: iv(1.0),
                offset: iv(0.5),
            };
            let (p0, p1) = (spiric.eval(iv(-FRAC_PI_2)), spiric.eval(iv(FRAC_PI_2)));
            let arc = LoopEdge::hand_built(spiric, iv(-FRAC_PI_2), iv(FRAC_PI_2), true, 0, 1);
            let chord = LoopEdge::hand_built(
                nurbs(vec![0.0, 0.0, 1.0, 1.0], 1, vec![p1, p0], &[1.0, 1.0]),
                iv(0.0),
                iv(1.0),
                true,
                1,
                0,
            );
            let span = (p1 - p0).norm().hi();
            let bound = length_bound(&chord).expect("a spline edge's length is bounded");
            assert!(bound >= span, "{bound} under the chord's {span}");
            let face = plane_cap(&[vec![arc, chord]], Point3::origin())
                .expect("a spiric cap a spline closes forms");
            assert!(
                face.spirics.len() == 1 && face.rationals.is_empty() && face.perimeter >= span,
                "one spiric, the spline in closed form, perimeter {}",
                face.perimeter
            );
        }
    }

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
            FaceRun, QuadLane, References, corner_of, decide_faces_serially, face_flux, face_loops,
            flat_loops, rederive, resolve_face, round_hook, shell_polygons, vertex_rings,
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

        /// An exact fraction of two dyadics, its denominator positive.
        #[derive(Clone, Debug)]
        struct Frac {
            num: Dyadic,
            den: Dyadic,
        }

        impl Frac {
            fn add(&self, o: &Self) -> Self {
                Self {
                    num: self.num.mul(&o.den).add(&o.num.mul(&self.den)),
                    den: self.den.mul(&o.den),
                }
            }
            /// `a ≤ self`.
            fn above(&self, a: &Dyadic) -> bool {
                a.mul(&self.den).le(&self.num)
            }
            /// `self ≤ a`.
            fn below(&self, a: &Dyadic) -> bool {
                self.num.le(&a.mul(&self.den))
            }
        }

        /// Twice the exact flux of `rings` about `c` on the plane whose
        /// normal is `n`: the distance of the first point from `c` along
        /// `n` times the rings' signed area along it, fanned from that
        /// point, `((a − c)·n)(2A⃗·n)/(n·n)`.
        fn exact_flux2(rings: &[Vec<Point3<f64>>], n: &D3, c: &D3) -> Frac {
            let a = d3(rings[0][0]);
            let mut va = [0.0, 0.0, 0.0].map(Dyadic::of);
            for ring in rings {
                for (i, &p) in ring.iter().enumerate() {
                    let q = ring[(i + 1) % ring.len()];
                    let k = cross(&sub(&d3(p), &a), &sub(&d3(q), &a));
                    va = [0, 1, 2].map(|j| va[j].add(&k[j]));
                }
            }
            Frac {
                num: dot(&sub(&a, c), n).mul(&dot(&va, n)),
                den: dot(n, n),
            }
        }

        /// `body`'s faces as `(interval flux, exact doubled flux)` about
        /// its least vertex corner, summed, every face read as its vertex
        /// polygons on its plane.
        fn sums(body: &Body<f64>) -> (Interval, Frac) {
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
            let mut exact = Frac {
                num: Dyadic::of(0.0),
                den: Dyadic::of(1.0),
            };
            for (key, _) in body.faces() {
                let (face, surface) = resolve_face(body, key);
                let loops = flat_loops(body, face).unwrap();
                let rings = vertex_rings(body, surface, &loops, lane)
                    .unwrap()
                    .expect("a planar face bounded by lines");
                let &geom::Surface::Plane { normal, .. } = surface else {
                    panic!("a planar face");
                };
                let lifted = geom_core::Vec3::new(
                    Interval::from_f64(normal.x),
                    Interval::from_f64(normal.y),
                    Interval::from_f64(normal.z),
                );
                let f = super::super::polygon_face_about(&rings, lifted, centre).unwrap();
                flux = flux + f.flux;
                let rings: Vec<Vec<Point3<f64>>> = rings
                    .iter()
                    .map(|r| {
                        r.iter()
                            .map(|p| Point3::new(p.x.lo(), p.y.lo(), p.z.lo()))
                            .collect()
                    })
                    .collect();
                let n = [normal.x, normal.y, normal.z].map(Dyadic::of);
                exact = exact.add(&exact_flux2(&rings, &n, &d3(corner)));
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
                    sliver_box(1e-4, tol).revert().unwrap(),
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
                        exact.above(&six) && exact.below(&six),
                        "{name}: the planes' doubled flux {exact:?} is six times the volume"
                    );
                }
                let two = Dyadic::of(2.0);
                let lo = Dyadic::of(flux.lo()).mul(&two);
                let hi = Dyadic::of(flux.hi()).mul(&two);
                assert!(
                    flux.is_certified() && exact.above(&lo) && exact.below(&hi),
                    "{name}: the enclosure [{:e}, {:e}] holds the exact flux {exact:?}",
                    flux.lo(),
                    flux.hi()
                );
            }
        }

        /// A brick's walk runs, measured as the role read measures them.
        fn runs(body: &Body<f64>, band: Band, tol: Tol) -> Vec<FaceRun<f64>> {
            let faces: Vec<FaceKey> = body.faces().map(|(k, _)| k).collect();
            let hook = round_hook(Some(QuadLane::<f64>::certified()));
            let refs = References::of(
                body,
                faces.iter().copied(),
                band,
                Some(crate::props::QuadLane::certified()),
            );
            decide_faces_serially(&faces, |&f| {
                face_flux(
                    body,
                    f,
                    band,
                    &hook,
                    tol,
                    RoundWindow::SCHEDULE,
                    &refs,
                    Some(crate::test_support::pinned_len(tol)),
                )
            })
            .unwrap()
        }

        /// **All or nothing.** The notch307 prism with a disc planted in
        /// its top has planes bounded by lines and planes bounded by a
        /// circle, so no face takes the polygon route: its re-derivation
        /// is each face's own closed form about the corner, summed in walk
        /// order, bit for bit. The prism's slanted edges end off their
        /// vertex points, so a line-bounded plane read as its polygon
        /// would not match. The prism alone takes the polygon route.
        #[test]
        fn a_shell_with_a_curved_edge_keeps_the_fan_route_bit_for_bit() {
            let tol = Tol::witness();
            let band = Band::linear(tol).unwrap();
            let lane = QuadLane::<f64>::certified();
            let notch = [(0.0, 0.0), (4.0, 0.0), (4.0, 2.0), (2.0, 1.0), (0.0, 2.0)];
            let p = crate::test_support::prism::<f64>(&notch, 1.0, tol);
            let mut body = p.body;
            assert!(
                shell_polygons(&body, lane, &runs(&body, band, tol))
                    .unwrap()
                    .is_some(),
                "the prism alone is read as its polygons"
            );
            let outer = body.get_face(p.top_face).unwrap().outer;
            let LoopBoundary::Cycle { first } = body.get_loop(outer).unwrap().boundary else {
                panic!("the top's outer loop is a cycle");
            };
            crate::test_support::plant_disc_face(
                &mut body,
                first,
                Point3::new(1.0, 0.6, 1.0),
                0.4,
                tol,
            );
            let runs = runs(&body, band, tol);
            assert!(
                shell_polygons(&body, lane, &runs).unwrap().is_none(),
                "a disc in the top keeps every face off the polygon route"
            );
            let (mut flux, mut area) = (Interval::zero(), Interval::zero());
            let mut polygon = Interval::zero();
            for run in &runs {
                let centre = corner_of(lane, run);
                let (face, surface) = resolve_face(&body, run.face);
                let loops = face_loops(&body, face).unwrap();
                let (c, _) =
                    super::super::closed_form(surface, &loops, face.sense, band, centre, None)
                        .unwrap();
                flux = flux + c.flux;
                area = area + c.area;
                polygon = polygon
                    + match vertex_rings(&body, surface, &flat_loops(&body, face).unwrap(), lane)
                        .unwrap()
                    {
                        Some(rings) => {
                            let &geom::Surface::Plane { normal, .. } = surface else {
                                panic!("a polygon face is a plane");
                            };
                            let normal = geom_core::Vec3::new(
                                Interval::from_f64(normal.x),
                                Interval::from_f64(normal.y),
                                Interval::from_f64(normal.z),
                            );
                            super::super::polygon_face_about(&rings, normal, centre)
                                .unwrap()
                                .flux
                        }
                        None => c.flux,
                    };
            }
            let bits = |x: Interval| (x.is_certified(), x.lo().to_bits(), x.hi().to_bits());
            assert_ne!(
                bits(polygon),
                bits(flux),
                "the premise: its line-bounded planes read differently as polygons"
            );
            let volume = flux / Interval::from_f64(3.0);
            for tight in [false, true] {
                let got = rederive(&body, band, tol, lane, &runs, tight).unwrap();
                assert_eq!(
                    (bits(got.volume), bits(got.area), got.recentred),
                    (bits(volume), bits(area), true),
                    "tight {tight}: the re-derivation is every face's closed form"
                );
            }
        }
    }

    /// `rectangle_winding` on hand-built walks of the unit square.
    #[allow(clippy::float_cmp)]
    mod rectangle_winding {
        use super::super::{ChartSegment, rectangle_winding};

        const R: (f64, f64, f64, f64) = (0.0, 1.0, 0.0, 1.0);

        const ONCE: [ChartSegment; 4] = [
            ((0.0, 0.0), (1.0, 0.0)),
            ((1.0, 0.0), (1.0, 1.0)),
            ((1.0, 1.0), (0.0, 1.0)),
            ((0.0, 1.0), (0.0, 0.0)),
        ];

        #[test]
        fn once_around_reads_its_direction_and_twice_around_refuses() {
            assert_eq!(rectangle_winding(&ONCE, R), Some(1.0), "counter-clockwise");
            let cw: Vec<ChartSegment> = ONCE.iter().rev().map(|&(a, b)| (b, a)).collect();
            assert_eq!(rectangle_winding(&cw, R), Some(-1.0), "clockwise");
            let twice: Vec<ChartSegment> = ONCE.iter().chain(&ONCE).copied().collect();
            assert_eq!(rectangle_winding(&twice, R), None, "twice around");
        }

        /// A vertex whose two images round apart by an ulp along their
        /// side, as a degree-1 path sweep's wall closes, is one vertex.
        #[test]
        fn a_vertex_an_ulp_apart_on_its_two_images_is_one_vertex() {
            let below_one = 1.0 - f64::EPSILON / 2.0;
            let mut w = ONCE;
            w[0].1 = (below_one, 0.0);
            w[2].0 = (below_one, 1.0);
            assert_eq!(rectangle_winding(&w, R), Some(1.0));
        }

        /// Walks the segment sides and directions alone would read as
        /// once around, each with a gap between two segments.
        #[test]
        fn a_walk_with_a_gap_refuses() {
            let mut gap = ONCE;
            gap[1].0 = (1.0, 0.5);
            assert_eq!(
                rectangle_winding(&gap, R),
                None,
                "the right side starts half way up"
            );
            let mut short = ONCE;
            short[1].1 = (1.0, 0.5);
            assert_eq!(
                rectangle_winding(&short, R),
                None,
                "the right side stops short"
            );
            let mut hair = ONCE;
            hair[1].0 = (1.0, 1e-9);
            assert_eq!(rectangle_winding(&hair, R), None, "a gap of 1e-9");
            let mut more = ONCE.to_vec();
            more.push(((0.0, 0.0), (0.5, 0.0)));
            assert_eq!(
                rectangle_winding(&more, R),
                None,
                "five quarters round, unclosed"
            );
        }

        /// A spur on the bottom side, out to 1, back to 1/2, out again.
        #[test]
        fn a_spur_on_one_side_refuses() {
            let w = [
                ((0.0, 0.0), (1.0, 0.0)),
                ((1.0, 0.0), (0.5, 0.0)),
                ((0.5, 0.0), (1.0, 0.0)),
                ((1.0, 0.0), (1.0, 1.0)),
                ((1.0, 1.0), (0.0, 1.0)),
                ((0.0, 1.0), (0.0, 0.0)),
            ];
            assert_eq!(rectangle_winding(&w, R), None);
        }
    }
}
