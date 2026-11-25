use geom_brep::props::quad::{
    self, Accept, FaceCutBounds, HarmChan, LaneRun, LaneStop, PatchNet, RoundWindow, TrimChord,
    TrimEdgeQ, TrimPiece,
};
use geom_brep::props::{FaceContribution, LoopEdge, PropsError, exact_point, loop_vector_area};
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
use geom_core::{Band, Bounds, CertifiedEnclosure, Decide, Point2, Point3};

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
    (geom::mid_param(x.lo(), x.hi()), (x.hi() - x.lo()) * 0.5)
}

/// **The carrier of a harmonic image, as the cylinder lane reads it**: a
/// line, a circle or an ellipse, whose harmonic pcurve the lane
/// integrates; each end's `(cos, sin)` is formed at the limit the lane
/// integrates from ([`quad::trig_at_limit`]), so no vertex is read. The
/// other kinds refuse by name.
fn harmonic_carrier<T: Decide>(carrier: &Curve3<T>) -> Result<(), PropsError> {
    match carrier {
        Curve3::Line(_) | Curve3::Circle(..) | Curve3::Ellipse(..) => Ok(()),
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
        Curve3::Nurbs(_) | Curve3::Periodic(_) => Err(PropsError::QuadratureUnsupported {
            what: "B-spline trim carrier with a harmonic image on the cylinder chart — a \
                   spline carrier's image there is projected, which the lane reads \
                   through the carrier itself; a harmonic one is never minted for it",
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
    held: &[Point3<T>],
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
    let held: Vec<Point3<Interval>> = held
        .iter()
        .map(|p| {
            Point3::new(
                Interval::from_certified(p.x),
                Interval::from_certified(p.y),
                Interval::from_certified(p.z),
            )
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
    if let Surface::Plane { frame, .. } = surface {
        let normal = frame.axis();
        return Ok((planar_face_about(&loops, normal, centre)?, true));
    }
    match moved_by(&surface, &loops, centre) {
        Some((surface, moved_loops)) => {
            let moved_held: Vec<Point3<Interval>> =
                held.iter().map(|&p| moved_point(p, centre)).collect();
            Ok((
                super::closed_form_of(&surface, &moved_loops, &moved_held, sense, band)?,
                true,
            ))
        }
        None => {
            let about_origin = super::closed_form_of(&surface, &loops, &held, sense, band)?;
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

/// A face's vector area `A⃗ = ∫ n dA`, from its loops, summed about the
/// face's lever point ([`loops_anchor`]; a closed loop's does not depend
/// on it). A
/// spiric-bounded face has no closed form here and refuses; such a face
/// is measured by its own lane, about the centre, and is never carried.
fn loops_vector_area(
    loops: &[Vec<LoopEdge<Interval>>],
) -> Result<geom_core::Vec3<Interval>, PropsError> {
    let anchor = loops_anchor(loops)?;
    let mut va = geom_core::Vec3::zero();
    for edges in loops {
        va = va + loop_vector_area(edges, anchor.lever)?;
    }
    Ok(va)
}

/// A point carried by `−by`, as [`moved_by`] carries a face's geometry.
pub(super) fn moved_point<U: Decide>(p: Point3<U>, by: Point3<U>) -> Point3<U> {
    Point3::origin() + (p - by)
}

/// `surface` carried by `−by`: every point-valued datum moved, every
/// direction and length kept. `None` for a kind with no analytic datum
/// to move (a spline or fitted surface).
pub(super) fn translated_surface<T: Decide>(
    surface: &Surface<T>,
    by: Point3<T>,
) -> Option<Surface<T>> {
    let shift = |p: Point3<T>| moved_point(p, by);
    Some(match surface.clone() {
        Surface::Plane {
            frame: carrier_frame,
        } => Surface::Plane {
            frame: carrier_frame.with_origin(shift(carrier_frame.origin())),
        },
        Surface::Cylinder {
            frame: carrier_frame,
            radius,
        } => Surface::Cylinder {
            frame: carrier_frame.with_origin(shift(carrier_frame.origin())),
            radius,
        },
        Surface::Cone {
            frame: carrier_frame,
            half_angle,
        } => Surface::Cone {
            frame: carrier_frame.with_origin(shift(carrier_frame.origin())),
            half_angle,
        },
        Surface::Sphere {
            frame: carrier_frame,
            radius,
        } => Surface::Sphere {
            frame: carrier_frame.with_origin(shift(carrier_frame.origin())),
            radius,
        },
        Surface::Torus {
            frame: carrier_frame,
            major_radius,
            minor_radius,
        } => Surface::Torus {
            frame: carrier_frame.with_origin(shift(carrier_frame.origin())),
            major_radius,
            minor_radius,
        },
        Surface::Spindle {
            frame: carrier_frame,
            major_radius,
            minor_radius,
        } => Surface::Spindle {
            frame: carrier_frame.with_origin(shift(carrier_frame.origin())),
            major_radius,
            minor_radius,
        },
        _ => return None,
    })
}

/// `curve` carried by `−by`: the analytic kinds' centre or origin moved,
/// a spline's control net moved point for point (its net is stored
/// Euclidean, so a translation is the curve's image).
pub(super) fn translated_curve<T: Decide>(curve: &Curve3<T>, by: Point3<T>) -> Option<Curve3<T>> {
    let shift = |p: Point3<T>| Point3::origin() + (p - by);
    let back = -(by - Point3::origin());
    Some(match curve.clone() {
        Curve3::Line(geom::Line { origin, dir }) => Curve3::line(shift(origin), dir),
        Curve3::Circle(c) => Curve3::Circle(c.placed(|f| f.translated(back))),
        Curve3::Ellipse(e) => Curve3::Ellipse(e.translated(back)),
        Curve3::Spiric {
            frame: carrier_frame,
            major_radius,
            minor_radius,
            offset,
        } => Curve3::Spiric {
            frame: carrier_frame.with_origin(shift(carrier_frame.origin())),
            major_radius,
            minor_radius,
            offset,
        },
        Curve3::Nurbs(n) => Curve3::Nurbs(std::sync::Arc::new(n.map_points(shift))),
        Curve3::Periodic(n) => Curve3::Periodic(std::sync::Arc::new(n.map_points(shift))),
    })
}

/// A planar face's flux about `centre` and its area, at the interval
/// scalar: its plane's signed distance from `centre` times its own
/// signed area ([`plane_flux`]), at the face's anchor ([`loops_anchor`]):
/// `A⃗` the face's vector area summed about its lever point, and the
/// distance taken from its mean vertex along the stored plane's
/// `normal`.
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
        va = va + loop_vector_area(edges, anchor.lever)?;
    }
    let lever = anchor.from(centre);
    Ok(FaceContribution {
        flux: plane_flux(lever, va, normal) + tilt_slack(lever, va, normal),
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
/// the rings fanned from their points' anchor
/// ([`geom_brep::props::face_anchor`], the one [`planar_face_about`]
/// takes of the same face's loops).
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
    let Some(anchor) = geom_brep::props::face_anchor(rings.iter().flatten().copied()) else {
        return Err(PropsError::DegenerateFace);
    };
    let l = anchor.lever;
    let mut va = geom_core::Vec3::zero();
    for ring in rings {
        for (i, &p) in ring.iter().enumerate() {
            let q = ring[(i + 1) % ring.len()];
            va = va + (p - l).cross(q - l) * Interval::point(0.5);
        }
    }
    let lever = anchor.from(centre);
    Ok(FaceContribution {
        flux: plane_flux(lever, va, normal) + tilt_slack(lever, va, normal),
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
/// ([`rim_face`]); a cylinder or sphere face takes the rotational lane
/// (`geom_brep::props::quad::rotational_cut_face_rounds`).
///
/// The tolerance is the hook's ([`super::QuadLane`]): no lane here reads
/// it, since each trim end's trig is formed at its parameter, not off its
/// vertex within `ε` ([`harmonic_carrier`]).
#[allow(clippy::too_many_arguments)]
pub(super) fn cut_face_rounds<T: Decide + Bounds + CertifiedEnclosure>(
    body: &Body<T>,
    surface: &Surface<T>,
    sense: bool,
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
    cut_face_lane(
        body,
        surface,
        sense,
        loops,
        band,
        window,
        centre,
        &mut accept,
    )
}

/// [`cut_face_rounds`]' lanes, asking `accept` of every round.
#[allow(clippy::too_many_arguments)]
fn cut_face_lane<T: Decide + Bounds + CertifiedEnclosure>(
    body: &Body<T>,
    surface: &Surface<T>,
    sense: bool,
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
    // The structural curved-rim arm: a plane comes here only when a
    // spiric or a rational spline rim is on its boundary, a cone when a
    // trim its closed form does not read is ([`super::takes_quadrature`]).
    // A torus wall a spiric bounds is the rotational lane's, below.
    if matches!(surface, Surface::Plane { .. } | Surface::Cone { .. }) {
        return rim_face(surface, loops, band, window, centre, accept);
    }
    let iv = Interval::from_certified;
    let (origin, chart) = match surface {
        Surface::Cylinder { frame, radius, .. } => {
            let origin = frame.origin();
            (
                origin,
                quad::RotationalChartQ::Cylinder {
                    radius: iv(*radius),
                },
            )
        }
        Surface::Sphere { frame, radius, .. } => {
            let center = frame.origin();
            (
                center,
                quad::RotationalChartQ::Sphere {
                    radius: iv(*radius),
                    outward: sense,
                },
            )
        }
        Surface::Torus {
            frame,
            major_radius,
            minor_radius,
            ..
        } => {
            let center = frame.origin();
            (
                center,
                quad::RotationalChartQ::Torus {
                    major: iv(*major_radius),
                    minor: iv(*minor_radius),
                    outward: sense,
                },
            )
        }
        _ => {
            return Err(PropsError::QuadratureUnsupported {
                what: "a trimmed face with no volume lane on its chart — a cone face takes \
                       its closed form before this lane (`cone_face_closed_form`), and the \
                       rotational lane's chart table holds the cylinder, the sphere and the \
                       torus",
            });
        }
    };
    let lift = |p: Point3<T>| Point3::new(iv(p.x), iv(p.y), iv(p.z));
    let lifted_edge = |e: &LoopEdge<T>| LoopEdge {
        carrier: e.carrier.map_scalar(iv),
        carrier_id: e.carrier_id,
        t0: iv(e.t0),
        t1: iv(e.t1),
        forward: e.forward,
        start: e.start,
        end: e.end,
    };
    // The chart term `o·A⃗`: each closed-form edge's vector area about the
    // chart's own origin at the caller's scalar, and each rational or
    // spiric carrier's through its rule, read per round.
    let mut va = geom_core::Vec3::new(T::zero(), T::zero(), T::zero());
    let mut rationals = Vec::new();
    let mut spirics = Vec::new();
    for (edges, _) in loops {
        for e in edges {
            match &e.carrier {
                Curve3::Nurbs(c) if geom_brep::props::is_rational(c) => {
                    rationals.push(rational_rim::rational_edge_q(
                        &c.map_scalar(iv),
                        iv(e.t0),
                        iv(e.t1),
                        e.forward,
                        lift(origin),
                    )?);
                }
                Curve3::Spiric { .. } => {
                    spirics.push(spiric_edge_q(&lifted_edge(e), lift(origin)).ok_or(
                        PropsError::QuadratureUnsupported {
                            what: "a spiric carrier that is no spiric",
                        },
                    )?)
                }
                _ => va = va + loop_vector_area(core::slice::from_ref(e), origin)?,
            }
        }
    }
    // The flux about the chart's own origin, `−∮F du`, is where the
    // face is; only the chart term carries its position. Taken about a
    // centre it is `(origin − centre)·A⃗` in interval arithmetic, a lever
    // the size of the body rather than of its distance from the world
    // origin.
    let (closed, lever) = match centre {
        None => (
            Interval::from_certified((origin - Point3::origin()).dot(va)),
            lift(origin) - Point3::origin(),
        ),
        Some(c) => {
            let va = geom_core::Vec3::new(iv(va.x), iv(va.y), iv(va.z));
            ((lift(origin) - c).dot(va), lift(origin) - c)
        }
    };
    let mut terms = Vec::with_capacity(loops.len());
    // The carriers' half of the ribbon between them and their images, the
    // chart term's pad (`quad::ChartTermQ::ribbon`): each edge's envelope
    // times its carrier's length bound.
    let mut ribbon = Interval::zero();
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
            let (t0, t1) = cache.params();
            let env = iv(cache.certificate().envelope);
            let le = lifted_edge(le);
            let carrier_length = length_bound(&le)?;
            ribbon = ribbon + Interval::point(env.mag()) * Interval::point(carrier_length);
            // The lane reads a harmonic image channel by channel, and a
            // projected one through its carrier in the chart's frame. A
            // fitted or focal-section image has neither on an analytic
            // chart's Green reduction, and refuses typed.
            if let Some((p0, pa, pb, pl)) = image.harmonic_form() {
                harmonic_carrier(&le.carrier)?;
                let (t0, t1) = (iv(t0), iv(t1));
                edges.push(quad::EdgeTermQ::Harmonic(TrimEdgeQ {
                    u: chan(p0.x, pa.x, pb.x, pl.x)?,
                    v: chan(p0.y, pa.y, pb.y, pl.y)?,
                    t0,
                    t1,
                    forward: le.forward,
                    trig0: quad::trig_at_limit(t0),
                    trig1: Some(quad::trig_at_limit(t1)),
                    env,
                }));
                continue;
            }
            let pair = |p: geom_core::Point2<T>| (iv(p.x), iv(p.y));
            // The ends through the image's own evaluator, on the lane,
            // crossed into certification arithmetic.
            let ends = [pair(image.eval(t0)), pair(image.eval(t1))];
            if let Pcurve::Spiric {
                major,
                minor,
                image:
                    SpiricImage::Wall {
                        v0, sense, torus, ..
                    },
                ..
            } = image
            {
                edges.push(quad::EdgeTermQ::Spiric(spiric_wall_edge(
                    &le,
                    chart,
                    (iv(*major), iv(*minor)),
                    (iv(*v0), iv(*sense), *torus),
                    ends,
                    env,
                    lift(origin),
                )?));
                continue;
            }
            let Some(projected) = image.projected() else {
                return Err(PropsError::QuadratureUnsupported {
                    what: "curved-cut face half-edge carries a pcurve that is neither harmonic \
                       nor projected on an analytic chart (a FITTED image, or a cone \
                       section's or Villarceau circle's focal section) — its Green-form \
                       boundary integral is not wired",
                });
            };
            edges.push(quad::EdgeTermQ::Projected(quad::ProjectedEdgeQ::of_image(
                projected,
                chart,
                (iv(t0), iv(t1)),
                le.forward,
                ends,
                env,
                iv,
            )?));
        }
        terms.push(edges);
    }
    let face = quad::RotationalFaceQ {
        chart,
        chart_term: quad::ChartTermQ {
            closed,
            lever: rv(lever),
            rationals,
            spirics,
            ribbon: if ribbon.is_certified() {
                ribbon.hi()
            } else {
                f64::INFINITY
            },
        },
        loops: terms,
    };
    quad::rotational_cut_face_rounds::<T>(&face, band, window, accept)
}

/// An upper bound on an edge's length: a line's speed `|dir|`, an arc's radius (an
/// ellipse's larger semi-axis) times its span, a spiric's piecewise
/// [`spiric::spiric_length_bound`], a B-spline's speed sup times its span.
fn length_bound(e: &LoopEdge<Interval>) -> Result<f64, PropsError> {
    let span = (e.t1 - e.t0).mag();
    Ok(match e.carrier {
        Curve3::Line(geom::Line { dir, .. }) => (dir.get().norm() * Interval::point(span)).mag(),
        Curve3::Circle(_) | Curve3::Ellipse(_) => match e.carrier.conic() {
            Some(k) => (k.speed_hi() * Interval::point(span)).mag(),
            None => unreachable!("a circle or an ellipse is a conic"),
        },
        Curve3::Spiric { .. } => spiric_edge_q(e, Point3::origin())
            .and_then(|q| spiric::spiric_length_bound(&q, spiric::LENGTH_PIECES))
            .ok_or(PropsError::QuadratureUnsupported {
                what: "a spiric edge with a piece whose f is not certified positive",
            })?,
        // The carrier's certified speed sup over its whole domain, crossed
        // to metres over the edge's span.
        Curve3::Nurbs(_) | Curve3::Periodic(_) => {
            let Some(c) = e.carrier.whole_net() else {
                unreachable!("a spline has a net");
            };
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
) -> Option<spiric::SpiricEdgeQ> {
    let Curve3::Spiric {
        frame,
        major_radius,
        minor_radius,
        offset,
    } = e.carrier
    else {
        return None;
    };
    let (center, axis, u_ref) = (frame.origin(), frame.axis(), frame.u_ref());
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
    })
}

/// **A spiric edge of a torus wall as the rotational lane reads it**
/// ([`quad::SpiricWallQ`]): its carrier about the torus centre `origin`,
/// its exact `SpiricImage::Wall` (`v0`, its sense `σ` and which torus it
/// lies on), and its ends, read by the image's evaluator. The carrier's
/// radii and the image's must be the chart's brackets end for end, so
/// the arm's terms read one `R` and one `r`; an image on the carrier's
/// mirror torus through the cutting plane refuses typed until the arm
/// reads it (`the-spiric-wall-quadrature-lane-reads-only-the-own-torus`).
fn spiric_wall_edge(
    le: &LoopEdge<Interval>,
    chart: quad::RotationalChartQ,
    (image_major, image_minor): (Interval, Interval),
    (v0, sense, torus): (Interval, Interval, WallTorus),
    ends: [(Interval, Interval); 2],
    env: Interval,
    origin: Point3<Interval>,
) -> Result<quad::SpiricWallQ, PropsError> {
    let unsupported = |what| PropsError::QuadratureUnsupported { what };
    let quad::RotationalChartQ::Torus { major, minor, .. } = chart else {
        return Err(unsupported(
            "a spiric wall image on a chart other than a torus",
        ));
    };
    let edge = spiric_edge_q(le, origin)
        .ok_or_else(|| unsupported("a spiric wall image on an edge whose carrier is no spiric"))?;
    // The same stored datum: one bracket, end for end.
    let same = |a: Interval, b: Interval| {
        a.is_certified() && b.is_certified() && a.lo() == b.lo() && a.hi() == b.hi()
    };
    if !(same(edge.major, major)
        && same(edge.minor, minor)
        && same(image_major, major)
        && same(image_minor, minor))
    {
        return Err(unsupported(
            "a spiric wall edge whose carrier or image radii are not exactly its torus's",
        ));
    }
    if torus == WallTorus::Mirror {
        return Err(unsupported(
            "a spiric wall image on the carrier's mirror torus — the wall terms read the \
             carrier's own torus",
        ));
    }
    let sense = match exact_point(sense) {
        Some(s) if s == 1.0 || s == -1.0 => s,
        _ => {
            return Err(unsupported(
                "a spiric wall image whose sense is not exactly ±1",
            ));
        }
    };
    Ok(quad::SpiricWallQ {
        edge,
        image: spiric::WallImageQ { sense, v0, env },
        ends,
    })
}

/// **The curved-rim face lane**: a plane cap with a `Curve3::Spiric` or
/// a rational spline rim on its boundary, or a cone cap with a spiric or
/// a spline rim, past `geom_brep::props::require_cone_cap`'s gates, read
/// at the
/// interval scalar and handed to
/// [`spiric::rim_face_rounds`] (its module docs carry the formulation,
/// [`geom_brep::props::rational_rim`]'s a rational rim's). Reached only
/// from [`cut_face_rounds`]'s structural curved-rim arm, under
/// `QuadLane::certified`, so it is never formed at a
/// dual; the face walk hands it every loop of the face as it read them,
/// the outer loop first.
fn rim_face<T: Decide + geom_core::CertifiedBounds>(
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
            frame, half_angle, ..
        } => {
            let edges: Vec<&[LoopEdge<T>]> = loops.iter().map(|(e, _)| e.as_slice()).collect();
            geom_brep::props::require_cone_cap(
                (frame.origin(), frame.axis_unit(), *half_angle),
                &edges,
                band,
            )?;
            let (apex, axis) = (frame.origin(), frame.axis());
            let loops: Vec<Vec<LoopEdge<Interval>>> = loops
                .iter()
                .map(|(edges, _)| edges.iter().map(lift).collect())
                .collect();
            let apex = Point3::new(iv(apex.x), iv(apex.y), iv(apex.z));
            let axis = geom_core::Vec3::new(iv(axis.x), iv(axis.y), iv(axis.z));
            let axis = axis / (axis.x.sqr() + axis.y.sqr() + axis.z.sqr()).sqrt();
            let (sin_a, _) =
                geom_brep::implicit::cone_sin_cos(*half_angle, Interval::from_certified);
            let face = cap_about(&loops, apex, apex - centre)?;
            return spiric::cone_cap_rounds::<T>(&face, (rv(axis), sin_a), band, window, accept);
        }
        _ => {
            return Err(PropsError::QuadratureUnsupported {
                what: "a curved rim on a chart other than a plane's or a cone's",
            });
        }
    };
    spiric::rim_face_rounds::<T>(&face, band, window, accept)
}

/// A loop set's anchor ([`geom_brep::props::loops_anchor`]), the point
/// every lane reads a plane face's flux about.
fn loops_anchor(
    loops: &[Vec<LoopEdge<Interval>>],
) -> Result<geom_brep::props::FaceAnchor<Interval>, PropsError> {
    geom_brep::props::loops_anchor(loops).ok_or(PropsError::DegenerateFace)
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
    let anchor = loops_anchor(loops)?;
    cap_about(loops, anchor.lever, anchor.from(centre))
}

/// A cap's loops read about `anchor`, their vector area summed about it
/// and the flux levered by `lever`, the offset of the point it is read
/// about from the centre. [`plane_cap`]'s reading takes the face's anchor
/// ([`loops_anchor`]); [`spiric::cone_cap_rounds`] reads both about a
/// cone's apex.
fn cap_about(
    loops: &[Vec<LoopEdge<Interval>>],
    anchor: Point3<Interval>,
    lever: geom_core::Vec3<Interval>,
) -> Result<spiric::RimFaceQ, PropsError> {
    let mut closed_va = [Interval::zero(); 3];
    let mut spirics = Vec::new();
    let mut rationals = Vec::new();
    let mut perimeter = 0.0;
    for e in loops.iter().flatten() {
        perimeter += length_bound(e)?;
        if let Some(q) = spiric_edge_q(e, anchor) {
            spirics.push(q);
        } else if let Curve3::Nurbs(c) = &e.carrier {
            // Every spline, a polynomial one too: at this scalar the
            // closed form's Gauss rule is rounded, not an enclosure
            // (`geom_brep::props::closed_segment_term`).
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
        lever: rv(lever),
        perimeter,
    })
}

/// **The NURBS-patch flux lane** (M6-3 Leg C; RATIONAL since
/// M8-3): certified volume flux + area of a described NURBS face whose
/// trim is a rectangle of its chart: four sides, each a run of iso
/// images on one chart line placed at an exact value (every loft/sweep
/// wall: its seams and rims are the chart's boundary columns and rows).
///
/// The rectangle is the lines the loop's images hold and their meets
/// ([`trim_rectangle`]), never an image's end, so a split's cut and an
/// end a re-mint reads through a measured map are never asked to be
/// exact; how far an end stands off its joint's point is padded
/// ([`joint_defect`]). A loop that is not that shape refuses typed,
/// naming the trimmed-NURBS lane as the cut-loft unit's. The direction
/// the loop walks its rectangle IS the S10 orientation input —
/// winding-derived end to end, like the cylinder lane; no sense bit is
/// read.
///
/// # Why the rectangle's flux is the face's, to within the pad
///
/// The certified flux is that of the patch over the rectangle `R`, widened
/// by `D · p_bound` (the area by `D`), with `D = Σ L·e + joint_defect`:
/// each image's carrier length bound `L` times its envelope `e`, plus the
/// joint term. The argument, with what it assumes:
///
/// 1. **Each image is within its envelope of its edge**: checks 3 and 4,
///    stored with the row. Tier 3's check 8 replays them at rest, after
///    check 7 in the one-pass battery and before it in
///    `validate_geometric`; at check 7's point in the one-pass battery
///    the stored certificate is taken as stated, as it always was.
/// 2. **Each image lies on its side's line**: its held channel's slope is
///    an exact zero, so every point of it holds that one value
///    ([`geom_brep::Pcurve::held_line`]); each side's pieces state one
///    exact value and run one decided way from the side before it to the
///    side after it ([`TrimRectangle::walked`]). So the images and `∂R`
///    differ only near the joints, where an image may stop short of its
///    joint's point or run past it.
/// 3. **Along the images** the strip between each image's surface curve
///    and its edge has area at most `L·e`, so `Σ L·e`, as before.
/// 4. **At each joint** ([`joint_defect`]) the stretch of `∂R` between an
///    image's end and the joint's point has length at most `a`, metred by
///    the net's stretch sups, and starts within `e` of its edge's end; the
///    fan from that edge end over it has area at most `a·(a + 2e)`.
///
/// What it assumes, and what the joint term buys:
///
/// - **(a) The strip and fan bounds are bounds on areas in `R³`**, the
///   ribbon between two curves and the fan over a stretch of line; that
///   they bound the area *on the surface* between the chart region and
///   the rectangle needs the surface to be regular there (its metric
///   comparable on the two sides of the bound), which the envelope pad
///   `Σ L·e` assumes too. It is the same assumption, not a new one.
/// - **(b) On a body whose edges are continuous**, the vertex lies within
///   `e` of both images' ends, so the strips already reach it and the
///   joint term is not what carries the bound; a measurement over 200,000
///   flat-chart trials held without it. What it buys is a bound that
///   does not rest on continuity having been checked first: a loop whose
///   joints gape (refused by tier 3, `PcurveMintError::LoopDiscontinuity`,
///   which runs after this check in the one-pass battery) is padded by
///   its gaps rather than certified past them.
/// - **(c) The stretch sups are sups over the knot domain**, so they do not
///   cover the stretch of an image that runs past the domain's edge. Check
///   4's domain gate (`pcurve_iso_domain`, in `run_iso_checks`) decides an
///   iso image's escape past the domain `Zero` at the band and folds its
///   metred excursion into the envelope, so such an overshoot is at most
///   `ε` metric and inside `e`.
/// - **(d) `p_bound` is taken over the rectangle**, while the strips and
///   fans lie up to `a + e` outside it; the flux pad's lever there differs
///   by second-order terms, as on main.
/// - **(e) "Computed upward"**: the pad is summed in interval arithmetic,
///   outward at both scalars. Its inputs are read as their scalar states
///   them: at `Interval` the gaps and sups are certified enclosures and
///   their magnitudes are upper bounds; at `f64` they are the
///   round-to-nearest values the run reads as exact
///   (`[simple.conditioned]`).
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
    // **The dispatch is by pcurve KIND** (TRIM-2 §8.1): a loop whose
    // every image is an iso class may be a rectangle of its chart, and
    // takes the rectangle certificate below; a loop carrying a `General`
    // image bounds a region that is not a rectangle of its chart at all,
    // and takes the trimmed lane. `Fitted` keeps its own refusal in
    // both — no shipped construction mints one here.
    if hes
        .iter()
        .filter_map(|he| body.pcurve(*he))
        .any(|c| matches!(c.pcurve(), Pcurve::General(_)))
    {
        return trimmed_face(body, payload, outer, hes, band, window, centre, accept);
    }
    let mut envelopes = std::collections::HashMap::with_capacity(hes.len());
    let mut boundary_defect = 0.0f64;
    let mut perimeter = 0.0f64;
    for (le, he) in outer.iter().zip(hes) {
        let cache = body.pcurve(*he).ok_or(PropsError::QuadratureUnsupported {
            what: "NURBS face half-edge carries no stored pcurve cache — the \
                       loft assembly mints them; a body that lost its caches must \
                       re-mint before mass properties",
        })?;
        // Both iso classes lie on a row or a column of the chart:
        // `IsoLine` for seams and line rims, `IsoArc` for a rational
        // wall's arc rims (M8-3), the same line on another
        // parameterization of it.
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
        // Metric boundary length bound + the map-residual defect.
        let (t0, t1) = cache.params();
        let len = carrier_metric_length(&le.carrier, t0, t1)?;
        let envelope = Interval::from_certified(cache.certificate().envelope).mag();
        perimeter += len;
        boundary_defect += len * envelope;
        envelopes.insert(*he, envelope);
    }
    let Some(r#loop) = hes
        .first()
        .and_then(|&first| body.get_half_edge(first))
        .map(|h| h.parent_loop)
    else {
        return Err(PropsError::QuadratureUnsupported {
            what: "a NURBS face with an empty loop reached the rectangle lane — an \
                   empty loop bounds no region",
        });
    };
    let lift = crate::pcurves::Lift::read(body, r#loop).map_err(|_| {
        PropsError::QuadratureUnsupported {
            what: "the NURBS face's loop has no lift to read — a half-edge stores no \
                   joint element on the joint into it, or its image is one its lift \
                   does not apply to",
        }
    })?;
    if lift.rows().len() != hes.len()
        || !lift
            .iter()
            .all(|row| envelopes.contains_key(&row.half_edge))
    {
        return Err(PropsError::QuadratureUnsupported {
            what: "the NURBS face's loop lift holds other half-edges than the face \
                   walk read — the loop's cycle and its flattened edges disagree",
        });
    }
    let collapsed: Vec<(geom_brep::ChartSide, f64)> =
        geom_brep::chart_singularity::collapsed_sides(payload)
            .into_iter()
            .filter_map(|(side, _)| {
                let domain = match side.fixed {
                    geom_brep::ChartAxis::U => payload.knot_values_u().domain(),
                    geom_brep::ChartAxis::V => payload.knot_values_v().domain(),
                };
                let end = match side.end {
                    geom_brep::ChartEnd::Low => domain.0,
                    geom_brep::ChartEnd::High => domain.1,
                };
                exact_value(Interval::from_certified(end), false)
                    .ok()
                    .map(|value| (side, value))
            })
            .collect();
    let TrimRectangle { rect, turn, jumps } = trim_rectangle(&lift, &collapsed)?;
    let envelopes: Vec<f64> = lift.iter().map(|row| envelopes[&row.half_edge]).collect();
    let (sup_u, sup_v) = payload.stretch_sup();
    let mag = |x: T| Interval::from_certified(x).mag();
    boundary_defect += joint_defect(
        &lift,
        (mag(sup_u.get()), mag(sup_v.get())),
        &envelopes,
        &jumps,
    );
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
        rect.corners(),
        perimeter,
        boundary_defect,
        band,
        window,
        accept,
    )?;
    // The winding carries the S10 orientation into the flux; the area
    // is unsigned. It applies at whichever round the window ended: the
    // turn is a property of the traversal, not of the refinement.
    Ok(out.map_bounds(|b| FaceCutBounds {
        flux: match turn {
            Turn::CounterClockwise => b.flux,
            Turn::Clockwise => -b.flux,
        },
        area: b.area,
    }))
}

/// A rectangle of a chart, `u.0 < u.1` and `v.0 < v.1`.
#[derive(Clone, Copy, Debug, PartialEq)]
struct ChartRect {
    /// The columns, `(u0, u1)`.
    u: (f64, f64),
    /// The rows, `(v0, v1)`.
    v: (f64, f64),
}

impl ChartRect {
    /// `(u0, u1, v0, v1)`, the patch lane's spelling.
    fn corners(self) -> (f64, f64, f64, f64) {
        (self.u.0, self.u.1, self.v.0, self.v.1)
    }
}

/// The way a loop walks its rectangle.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Turn {
    CounterClockwise,
    Clockwise,
}

/// The way a piece runs along its line: with its moving channel or
/// against it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Way {
    WithChannel,
    AgainstChannel,
}

/// One side of a trim: the axis its line holds, the line's exact value,
/// and the way the side runs along it.
#[derive(Clone, Copy, Debug, PartialEq)]
struct Side {
    axis: geom_brep::ChartAxis,
    value: f64,
    way: Way,
}

/// The rectangle a NURBS face's trim states, and the way its loop walks
/// it. Formed only by [`TrimRectangle::walked`].
#[derive(Debug, PartialEq)]
struct TrimRectangle {
    rect: ChartRect,
    turn: Turn,
    /// The joints that walk a collapsed side: the row each enters, and
    /// the side.
    jumps: Vec<(usize, Side)>,
}

impl TrimRectangle {
    /// **The rectangle four consecutive sides walk**, or `None` where they
    /// walk none. Side `k` runs from the line of side `k − 1` to that of
    /// side `k + 1` (both hold the other axis): it must run from the one
    /// value strictly towards the other, the way it states, so two equal
    /// values refuse here too. Then the four meets are the
    /// rectangle's corners, the sides visit them once round, and the first
    /// row's way says which way round: counter-clockwise walks the bottom
    /// row with `u` and the top against it.
    fn walked(sides: [Side; 4]) -> Option<Self> {
        for k in 0..4 {
            let (from, to) = (sides[(k + 3) % 4], sides[(k + 1) % 4]);
            let runs_with = match sides[k].way {
                Way::WithChannel => to.value > from.value,
                Way::AgainstChannel => to.value < from.value,
            };
            if !runs_with {
                return None;
            }
        }
        let span = |a: f64, b: f64| (a.min(b), a.max(b));
        let (row, column) = match sides[0].axis {
            geom_brep::ChartAxis::V => (0, 1),
            geom_brep::ChartAxis::U => (1, 0),
        };
        let rect = ChartRect {
            u: span(sides[column].value, sides[column + 2].value),
            v: span(sides[row].value, sides[row + 2].value),
        };
        let bottom = sides[row].value == rect.v.0;
        let turn = if bottom == (sides[row].way == Way::WithChannel) {
            Turn::CounterClockwise
        } else {
            Turn::Clockwise
        };
        Some(Self {
            rect,
            turn,
            jumps: Vec::new(),
        })
    }
}

/// **The rectangle a NURBS face's trim states**, read off its loop's
/// lift ([`crate::pcurves::Lift::runs`]): the loop is a whole loop that
/// closes in its chart; it is images that lie on a row or a column of the
/// chart and nothing else; grouped into runs on one held axis there are
/// exactly four; every piece of a run holds one value, a single number,
/// and runs the same way along it, a decided sign; and the four sides walk
/// a rectangle ([`TrimRectangle::walked`]). The rectangle is the four held
/// values, its corners the meets of consecutive sides
/// ([`crate::pcurves::Lift::joint_point`]), read by comparisons alone, so
/// a rectangle whose sides are not dyadic (a sub-patch at a knot of
/// `1/9`) is read as exactly as `[0, 1]²`.
///
/// No end of any image is read. A cut, a joint between two images on
/// one line, is never asked to be exact, and a corner is the meet of two
/// lines, never an image's end; how far an end lies from its joint's
/// point is the pad's ([`joint_defect`]).
///
/// A step between two lines of one axis is a side of the rectangle the
/// boundary walks with no edge on it: a collapsed side of the net
/// (`collapsed`, each side held at its domain end's exact value), which
/// the step's joint walks along and which [`joint_defect`] reads as that
/// side's two corners.
fn trim_rectangle<T: Decide + Bounds + CertifiedEnclosure>(
    lift: &crate::pcurves::Lift<T>,
    collapsed: &[(geom_brep::ChartSide, f64)],
) -> Result<TrimRectangle, PropsError> {
    let refuse = |what| PropsError::QuadratureUnsupported { what };
    match lift.winding() {
        None => {
            return Err(refuse(
                "a NURBS-face boundary read as an open run, not a whole loop — the \
                 rectangle certificate reads a loop's lift",
            ));
        }
        Some(winding) if !winding.is_zero() => {
            return Err(refuse(
                "a NURBS-face boundary winds a period of its chart — a loop round a \
                 closed chart bounds no rectangle of it",
            ));
        }
        Some(_) => {}
    }
    let runs = lift.runs().map_err(|_| {
        refuse(
            "a NURBS-face pcurve is not an iso image — a trimmed NURBS region's \
             quadrature is the cut-loft unit's",
        )
    })?;
    // Each piece's held value and rate, crossed into certification
    // arithmetic, and whether the lift moved it by a period.
    let piece = |i: usize, line: &geom_brep::HeldLine<T>| Piece {
        value: Interval::from_certified(line.line.value),
        rate: Interval::from_certified(line.rate),
        moved: lift.rows()[i].lift != crate::joint::Deck::IDENTITY,
    };
    // A run splits where its held value steps: two lines on one axis,
    // joined across a collapsed side of the net (a pole the boundary
    // walks along with no edge on it). That side is the rectangle's too,
    // held at the domain's end the run before it reaches, and it runs
    // from that run's line to the next.
    let mut held: Vec<(geom_brep::ChartAxis, Vec<Piece>, usize)> = Vec::new();
    for run in &runs {
        for &(i, line) in &run.members {
            let p = piece(i, &line);
            let value = exact_value(p.value, p.moved)?;
            match held.last_mut() {
                Some((axis, pieces, _))
                    if *axis == run.axis
                        && pieces
                            .last()
                            .is_some_and(|q| exact_value(q.value, q.moved).ok() == Some(value)) =>
                {
                    pieces.push(p);
                }
                _ => held.push((run.axis, vec![p], i)),
            }
        }
    }
    let mut sides: Vec<Side> = Vec::with_capacity(4);
    let mut jumps: Vec<(usize, Side)> = Vec::new();
    let m = held.len();
    for k in 0..m {
        let (axis, pieces, _) = &held[k];
        let this = side(*axis, pieces)?;
        sides.push(this);
        let (next_axis, next_pieces, next_row) = &held[(k + 1) % m];
        if m > 1 && next_axis == axis {
            let next = side(*next_axis, next_pieces)?;
            let across = axis.other();
            let end = match this.way {
                Way::WithChannel => geom_brep::ChartEnd::High,
                Way::AgainstChannel => geom_brep::ChartEnd::Low,
            };
            let Some(&(_, value)) = collapsed
                .iter()
                .find(|(c, _)| c.fixed == across && c.end == end)
            else {
                return Err(refuse(
                    "a NURBS-face side's pieces lie on different chart lines, with no \
                     collapsed side of its chart between them — a stepped trim is outside \
                     the rectangle lane (the cut-loft unit's)",
                ));
            };
            let pole = Side {
                axis: across,
                value,
                way: if next.value > this.value {
                    Way::WithChannel
                } else {
                    Way::AgainstChannel
                },
            };
            sides.push(pole);
            jumps.push((*next_row, pole));
        }
    }
    let Ok(sides) = <[Side; 4]>::try_from(sides) else {
        return Err(refuse(
            "the NURBS-face boundary is not four sides of its chart — a trimmed \
             or multiply-wound region is outside the rectangle lane (the \
             cut-loft unit's)",
        ));
    };
    let TrimRectangle { rect, turn, .. } = TrimRectangle::walked(sides).ok_or(refuse(
        "the NURBS-face boundary does not traverse its UV rectangle exactly once — \
         a trimmed or multiply-wound region is outside the rectangle lane (the \
         cut-loft unit's)",
    ))?;
    Ok(TrimRectangle { rect, turn, jumps })
}

/// One piece of a run as [`side`] reads it: its line's held value and its
/// rate in chain order, crossed into certification arithmetic, and
/// whether the lift moved its image by a period.
struct Piece {
    value: Interval,
    rate: Interval,
    moved: bool,
}

/// **One run's side**: the one exact value every piece of a run holds,
/// and the one way every piece runs along it.
fn side(axis: geom_brep::ChartAxis, pieces: &[Piece]) -> Result<Side, PropsError> {
    let refuse = |what| PropsError::QuadratureUnsupported { what };
    let mut held: Option<(f64, Way)> = None;
    for piece in pieces {
        let value = exact_value(piece.value, piece.moved)?;
        let way = way_along(piece.rate)?;
        let &mut (v, w) = held.get_or_insert((value, way));
        if v != value {
            return Err(refuse(
                "a NURBS-face side's pieces lie on different chart lines — a \
                 stepped trim is outside the rectangle lane (the cut-loft unit's)",
            ));
        }
        if w != way {
            return Err(refuse(
                "a NURBS-face side runs back on itself — a trim that doubles a \
                 side is outside the rectangle lane (the cut-loft unit's)",
            ));
        }
    }
    let (value, way) = held.ok_or(refuse("a NURBS-face run holds no piece"))?;
    Ok(Side { axis, value, way })
}

/// A held value as the exact number it is: a certified point bracket.
/// `moved` says whether the lift moved the image by a period, which is
/// the cause where the value is no point there.
fn exact_value(x: Interval, moved: bool) -> Result<f64, PropsError> {
    // The refusal first: a refused crossing carries the scalar's own
    // endpoints, so a point bracket that may not certify passes both
    // tests below.
    if x.is_certified() && x.lo() == x.hi() && x.lo().is_finite() {
        Ok(x.lo())
    } else if moved {
        Err(PropsError::QuadratureUnsupported {
            what: "a NURBS-face line the lift moved by periods of a closed chart is held \
                   at a computed value, which is no single number — only a line at an end \
                   of the chart's domain, moved one period, is the other end exactly",
        })
    } else {
        Err(PropsError::QuadratureUnsupported {
            what: "a NURBS-face boundary line's held value is not exact structure — \
                   the rectangle-trim certificate needs each side's chart value as one \
                   number (a line placed at a measured foot is the cut-loft unit's)",
        })
    }
}

/// The way a piece runs along its line ([`geom_brep::HeldLine::rate`] in
/// chain order), decided by the rate's certified bracket lying wholly on
/// one side of zero.
fn way_along(x: Interval) -> Result<Way, PropsError> {
    if x.is_certified() && x.lo() > 0.0 {
        Ok(Way::WithChannel)
    } else if x.is_certified() && x.hi() < 0.0 {
        Ok(Way::AgainstChannel)
    } else {
        Err(PropsError::QuadratureUnsupported {
            what: "a NURBS-face boundary piece's way along its chart line is not \
                   decided — the rectangle-trim certificate reads each side's direction",
        })
    }
}

/// **How far the trim's images stand off the rectangle at its joints**,
/// as a metric area: step 4 of [`nurbs_face`]'s argument, the term added
/// to the envelope pad `Σ L·e`.
///
/// At the joint into row `i` the rectangle's boundary passes through the
/// joint's point `M` ([`crate::pcurves::Lift::joint_point`]), the row
/// before it leaves its line at `P` and row `i` enters at `Q`. The stretch
/// of line from `P` to `M` has metric length at most
/// `a = sup|S_u|·|ΔP_u| + sup|S_v|·|ΔP_v|`, and `b` from `M` to `Q`
/// likewise; the joint adds `a·(a + 2e₋) + b·(b + 2e₊)`, `e₋` and `e₊` the
/// arriving and leaving images' envelopes. A corner whose ends read as
/// its meet adds nothing; a joint tier 3 decides closed at the band
/// (`crate::pcurves::spline_gap_closes`, through the same sups) adds at
/// most `ε·(ε + 2e₋) + ε·(ε + 2e₊)`.
fn joint_defect<T: Decide + Bounds + CertifiedEnclosure>(
    lift: &crate::pcurves::Lift<T>,
    (sup_u, sup_v): (f64, f64),
    envelopes: &[f64],
    jumps: &[(usize, Side)],
) -> f64 {
    let iv = Interval::from_certified;
    let (su, sv) = (Interval::point(sup_u), Interval::point(sup_v));
    let n = lift.rows().len();
    let mut defect = Interval::zero();
    for i in 0..n {
        let before = (i + n - 1) % n;
        let (exit, entry) = (
            lift.rows()[before].entry_exit_points().1,
            lift.rows()[i].entry_exit_points().0,
        );
        // Off the collapsed side a joint walks, each end against its own
        // corner of that side: the meet of the side with the end's line,
        // which differs from the end only in the side's channel.
        let pole = jumps
            .iter()
            .find(|(row, _)| *row == i)
            .map(|(_, side)| *side);
        let off = |x: Interval| Interval::point(x.mag());
        let reach = |p: Point2<T>, m: Point2<Interval>| {
            Interval::point((su * off(iv(p.x) - m.x) + sv * off(iv(p.y) - m.y)).mag())
        };
        let at = |p: Point2<T>| match pole {
            Some(Side {
                axis: geom_brep::ChartAxis::U,
                value,
                ..
            }) => Point2::new(Interval::point(value), iv(p.y)),
            Some(Side {
                axis: geom_brep::ChartAxis::V,
                value,
                ..
            }) => Point2::new(iv(p.x), Interval::point(value)),
            None => {
                let m = lift.joint_point(i);
                Point2::new(iv(m.x), iv(m.y))
            }
        };
        let (a, b) = (reach(exit, at(exit)), reach(entry, at(entry)));
        let (e0, e1) = (
            Interval::point(envelopes[before]),
            Interval::point(envelopes[i]),
        );
        let two = Interval::point(2.0);
        defect = defect + a * (a + two * e0) + b * (b + two * e1);
    }
    defect.mag()
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
        Curve3::Line(geom::Line { dir, .. }) => {
            (Interval::from_certified(dir.get().norm()) * Interval::from_certified(t1 - t0)).mag()
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
        Curve3::Circle(circle) => {
            let radius = circle.radius();
            (Interval::from_certified(radius) * Interval::from_certified(t1 - t0)).mag()
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
    let unminted = || PropsError::QuadratureUnsupported {
        what: "NURBS face half-edge carries no stored pcurve cache — the \
               loft assembly mints them; a body that lost its caches must \
               re-mint before mass properties",
    };
    let (lift, rows) = crate::pcurves::lift_of_halves(body, hes).ok_or_else(unminted)?;
    for ((le, he), &row) in outer.iter().zip(hes).zip(&rows) {
        let row = row.ok_or_else(unminted)?;
        let cache = body.pcurve(*he).ok_or_else(unminted)?;
        let image = &lift.rows()[row].pcurve;
        let (t0, t1) = cache.params();
        // The chord runs between the row's two joints in the lift: a
        // corner's meet where an iso row meets an iso column, the
        // image's own value everywhere else.
        let (pa, pb) = (lift.joint_point(row), lift.exit_point(row));
        let (a, b) = ((ring(pa.x), ring(pa.y)), (ring(pb.x), ring(pb.y)));
        let piece = match image {
            // An iso image is its chord: a straight segment of the
            // chart, so there is no arc to bound.
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
                // held-value read gives ([`exact_value`]).
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
    /// **The spiric arm's builder refuses what its terms cannot read**
    /// ([`super::spiric_wall_edge`]), each guard alone: a chart that is
    /// not a torus, a carrier that is no spiric, a carrier's or an image's
    /// radii not exactly the torus's, an image on the carrier's mirror
    /// torus, and an image whose sense is not exactly `±1`. The unrefused
    /// reading beside them builds.
    #[allow(clippy::unwrap_used, clippy::panic)]
    mod spiric_wall_edge_rows {
        use geom::Curve3;
        use geom_brep::WallTorus;
        use geom_brep::props::quad::RotationalChartQ;
        use geom_brep::props::{LoopEdge, PropsError};
        use geom_core::interval::certification::Certification as _;
        use geom_core::{Interval, Point3, Vec3};

        use super::super::spiric_wall_edge;

        fn iv(x: f64) -> Interval {
            Interval::point(x)
        }

        fn spiric(major: f64, minor: f64) -> LoopEdge<Interval> {
            LoopEdge::hand_built(
                Curve3::Spiric {
                    frame: geom::Frame::new(
                        Point3::new(iv(0.0), iv(0.0), iv(0.0)),
                        Vec3::new(iv(0.0), iv(0.0), iv(1.0)),
                        Vec3::new(iv(1.0), iv(0.0), iv(0.0)),
                    ),
                    major_radius: iv(major),
                    minor_radius: iv(minor),
                    offset: iv(0.05),
                },
                iv(0.3),
                iv(2.1),
                true,
                0,
                1,
            )
        }

        fn torus() -> RotationalChartQ {
            RotationalChartQ::Torus {
                major: iv(1.2),
                minor: iv(0.225),
                outward: true,
            }
        }

        fn read(
            le: &LoopEdge<Interval>,
            chart: RotationalChartQ,
            image: (f64, f64),
            (sense, torus): (f64, WallTorus),
        ) -> Result<(), PropsError> {
            spiric_wall_edge(
                le,
                chart,
                (iv(image.0), iv(image.1)),
                (iv(0.0), iv(sense), torus),
                [(iv(0.0), iv(0.3)), (iv(0.0), iv(2.1))],
                Interval::zero(),
                Point3::new(iv(0.0), iv(0.0), iv(0.0)),
            )
            .map(|_| ())
        }

        #[test]
        fn the_spiric_arms_builder_refuses_each_unread_image() {
            let own = (1.0, WallTorus::Own);
            let good = spiric(1.2, 0.225);
            assert!(
                read(&good, torus(), (1.2, 0.225), own).is_ok(),
                "the premise: an own-torus spiric builds"
            );
            let line = LoopEdge::hand_built(
                Curve3::line(
                    Point3::new(iv(0.0), iv(0.0), iv(0.0)),
                    geom_core::UnitVec3::exact_x(),
                ),
                iv(0.0),
                iv(1.0),
                true,
                0,
                1,
            );
            for (what, got) in [
                (
                    "a chart that is not a torus",
                    read(
                        &good,
                        RotationalChartQ::Cylinder { radius: iv(1.0) },
                        (1.2, 0.225),
                        own,
                    ),
                ),
                (
                    "a carrier that is no spiric",
                    read(&line, torus(), (1.2, 0.225), own),
                ),
                (
                    "a carrier's radius off the torus's",
                    read(&spiric(1.25, 0.225), torus(), (1.2, 0.225), own),
                ),
                (
                    "an image's radius off the torus's",
                    read(&good, torus(), (1.2, 0.2), own),
                ),
                (
                    "an image on the mirror torus",
                    read(&good, torus(), (1.2, 0.225), (1.0, WallTorus::Mirror)),
                ),
                (
                    "a sense not ±1",
                    read(&good, torus(), (1.2, 0.225), (0.5, WallTorus::Own)),
                ),
            ] {
                assert!(
                    matches!(got, Err(PropsError::QuadratureUnsupported { .. })),
                    "{what}: {got:?}"
                );
            }
        }
    }

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

        use super::super::{cap_about, length_bound, plane_cap};

        fn iv(x: f64) -> Interval {
            Interval::point(x)
        }

        fn ip(x: f64, y: f64) -> Point3<Interval> {
            Point3::new(iv(x), iv(y), iv(0.0))
        }

        fn line(a: Point3<Interval>, b: Point3<Interval>, tags: (u32, u32)) -> LoopEdge<Interval> {
            // The side is its chord, its span in metres.
            let (dir, len) = geom_core::UnitVec3::chord(b - a);
            LoopEdge::hand_built(Curve3::line(a, dir), iv(0.0), len, true, tags.0, tags.1)
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
        /// read through the certified lane, as every spline rim is, and
        /// its perimeter at least its chord's.
        #[test]
        fn the_length_bound_reads_a_spline_edge_so_a_spiric_cap_it_closes_forms() {
            use core::f64::consts::FRAC_PI_2;
            let spiric = Curve3::Spiric {
                frame: geom::Frame::new(
                    Point3::new(iv(0.0), iv(0.0), iv(0.0)),
                    Vec3::new(iv(0.0), iv(0.0), iv(1.0)),
                    Vec3::new(iv(1.0), iv(0.0), iv(0.0)),
                ),
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
                face.spirics.len() == 1 && face.rationals.len() == 1 && face.perimeter >= span,
                "one spiric, the spline enclosed, perimeter {}",
                face.perimeter
            );
        }

        /// The cone `x² + y² = z²` (apex at the origin, `α = π/4`) and a
        /// cap on it: the parabola the plane `z = x + 2` cuts, `P(t) =
        /// ((t² − 4)/4, t, (t² + 4)/4)` over `t ∈ [0, 2]` as a polynomial
        /// quadratic with middle control point `mid`, the rim `z = 2` from
        /// `(0, 2, 2)` to `(−2, 0, 2)`, and the generator back to
        /// `(−1, 0, 1)`.
        fn parabola_cap(
            mid: Point3<f64>,
        ) -> (
            (Point3<f64>, geom_core::UnitVec3<f64>, f64),
            Vec<LoopEdge<f64>>,
        ) {
            use core::f64::consts::{FRAC_PI_2, FRAC_PI_4, PI};
            let frame = geom::Frame::new(Point3::origin(), Vec3::unit_z(), Vec3::unit_x());
            let cone = (frame.origin(), frame.axis_unit(), FRAC_PI_4);
            let (a, b, c) = (
                Point3::new(-1.0, 0.0, 1.0),
                Point3::new(0.0, 2.0, 2.0),
                Point3::new(-2.0, 0.0, 2.0),
            );
            let parabola = Curve3::Nurbs(std::sync::Arc::new(
                geom::NurbsCurve3::new(
                    geom_core::spline::Knots::from(
                        KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap(),
                    ),
                    vec![a, mid, b],
                    vec![1.0; 3],
                )
                .unwrap(),
            ));
            let rim = Curve3::circle(
                Point3::new(0.0, 0.0, 2.0),
                Vec3::unit_z(),
                2.0,
                Vec3::unit_x(),
            );
            let (dir, len) = geom_core::UnitVec3::chord(a - c);
            (
                cone,
                vec![
                    LoopEdge::hand_built(parabola, 0.0, 1.0, true, 0, 1),
                    LoopEdge::hand_built(rim, FRAC_PI_2, PI, true, 1, 2),
                    LoopEdge::hand_built(Curve3::line(c, dir), 0.0, len, true, 2, 0),
                ],
            )
        }

        /// **A cone cap a polynomial spline bounds is enclosed, and holds
        /// its closed form.** [`parabola_cap`]'s loop passes the cone cap
        /// gates, its parabola is read through the certified lane, and
        /// the axial vector area about the apex, the cap's area times
        /// `sin α`, holds the projected region's `π − 4/3` (the quarter
        /// disc of radius 2 less the parabola's lobe).
        #[test]
        fn a_cone_cap_a_polynomial_spline_bounds_holds_its_closed_form() {
            let band = geom_core::Band::linear(geom_core::Tol::witness()).unwrap();
            let (cone, edges) = parabola_cap(Point3::new(-1.0, 1.0, 1.0));
            geom_brep::props::require_cone_cap(cone, &[edges.as_slice()], band)
                .expect("the cap lies on one nappe");
            let lift = |e: &LoopEdge<f64>| {
                LoopEdge::hand_built(
                    e.carrier.map_scalar(Interval::from_certified),
                    iv(e.t0),
                    iv(e.t1),
                    e.forward,
                    e.start,
                    e.end,
                )
            };
            let loops = vec![edges.iter().map(lift).collect::<Vec<_>>()];
            let face = cap_about(
                &loops,
                Point3::origin(),
                Vec3::new(iv(0.0), iv(0.0), iv(1.0)),
            )
            .unwrap();
            assert!(
                face.spirics.is_empty() && face.rationals.len() == 1,
                "the parabola is enclosed, the rim and the generator in closed form"
            );
            let (arc_va, _) = rational_vector_area_split(&face.rationals[0], 256);
            let axial = face.closed_va[2] + arc_va[2];
            let truth = core::f64::consts::PI - 4.0 / 3.0;
            assert!(
                axial.is_certified() && axial.lo() <= truth && truth <= axial.hi(),
                "{axial:?} ∌ π − 4/3 = {truth}"
            );
        }

        /// **A cone cap whose spline's hull crosses the apex refuses**:
        /// [`parabola_cap`] with the parabola's middle control point moved
        /// to `z = −3`, where the curve itself dips to `z = −7/9` at
        /// `s = 4/9`, is no longer held on one nappe by its cover, and the
        /// gate refuses it before the quadrature.
        #[test]
        fn a_cone_cap_whose_spline_hull_crosses_the_apex_refuses() {
            let band = geom_core::Band::linear(geom_core::Tol::witness()).unwrap();
            let (cone, edges) = parabola_cap(Point3::new(-1.0, 1.0, -3.0));
            assert_eq!(
                geom_brep::props::require_cone_cap(cone, &[edges.as_slice()], band),
                Err(geom_brep::props::PropsError::NappeSpanning)
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
            flat_loops, rederive, round_hook, shell_polygons, vertex_rings,
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
                let face = body.proven_face(key);
                let surface = face.surface();
                let loops = flat_loops(face).unwrap();
                let rings = vertex_rings(body, surface, &loops, lane)
                    .unwrap()
                    .expect("a planar face bounded by lines");
                let &geom::Surface::Plane { frame, .. } = surface else {
                    panic!("a planar face");
                };
                let normal = frame.axis();
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
                let face = body.proven_face(run.face);
                let surface = face.surface();
                let loops = face_loops(face).unwrap();
                let (c, _) = super::super::closed_form(
                    surface,
                    &loops,
                    &crate::props::face_tips(face).unwrap(),
                    face.sense(),
                    band,
                    centre,
                    None,
                )
                .unwrap();
                flux = flux + c.flux;
                area = area + c.area;
                polygon = polygon
                    + match vertex_rings(&body, surface, &flat_loops(face).unwrap(), lane).unwrap()
                    {
                        Some(rings) => {
                            let &geom::Surface::Plane { frame, .. } = surface else {
                                panic!("a polygon face is a plane");
                            };
                            let normal = frame.axis();
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

    /// **The rectangle certificate reads a loop's held lines**
    /// ([`super::super::trim_rectangle`]): loops of the unit square and
    /// their misshapen kin, stated row by row at `f64` and at `Interval`,
    /// each read as the rectangle its lines state or refused.
    ///
    /// The loops are stated, not built ([`LiftedRow::stated_for_tests`],
    /// [`Lift::stated_for_tests`]): a row on no half-edge, a winding no
    /// joint decided, consecutive images that need not meet, and envelopes
    /// the rows do not carry. Each row says where its loop is one no body
    /// could carry.
    #[allow(clippy::float_cmp, clippy::unwrap_used, clippy::panic)]
    mod trim_rectangle_rows {
        use super::super::{ChartRect, TrimRectangle, Turn, joint_defect, trim_rectangle};
        use crate::joint::{Deck, Winding};
        use crate::pcurves::{Lift, LiftedRow};
        use geom_brep::props::PropsError;
        use geom_brep::{ChartLine, Pcurve};
        use geom_core::{Bounds as _, Interval, Point2, Real, Vec2};

        /// One side piece `p0 + pl·t` over `t ∈ [0, 1]`, traversed forward:
        /// the row through `p0` where the fixture's `pl` has no `v` part,
        /// the column through it otherwise (so `pl = (0, 0)` is a column
        /// that does not move).
        fn piece<T: Real>(p0: (T, T), pl: (f64, f64)) -> LiftedRow<T> {
            assert!(
                pl.0 == 0.0 || pl.1 == 0.0,
                "a side piece is a row or a column"
            );
            let pcurve = if pl.1 == 0.0 && pl.0 != 0.0 {
                Pcurve::IsoLine {
                    line: ChartLine::row(p0.1),
                    at0: p0.0,
                    rate: T::from_f64(pl.0),
                }
            } else {
                Pcurve::IsoLine {
                    line: ChartLine::column(p0.0),
                    at0: p0.1,
                    rate: T::from_f64(pl.1),
                }
            };
            LiftedRow::stated_for_tests(pcurve, (T::zero(), T::one()), true)
        }

        /// The piece from `a` to `b`, along one chart line.
        fn from_to<T: Real>(a: (f64, f64), b: (f64, f64)) -> LiftedRow<T> {
            let f = T::from_f64;
            piece((f(a.0), f(a.1)), (b.0 - a.0, b.1 - a.1))
        }

        /// The loop through `corners`, one piece per side.
        fn polygon<T: Real>(corners: &[(f64, f64)]) -> Vec<LiftedRow<T>> {
            (0..corners.len())
                .map(|i| from_to(corners[i], corners[(i + 1) % corners.len()]))
                .collect()
        }

        const SQUARE: [(f64, f64); 4] = [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];

        /// The winding of a loop that closes in its chart.
        fn closed() -> Option<Winding> {
            Some(Winding::of(core::iter::empty()))
        }

        /// The rectangle `(u0, u1, v0, v1)`.
        fn rect(u0: f64, u1: f64, v0: f64, v1: f64) -> ChartRect {
            ChartRect {
                u: (u0, u1),
                v: (v0, v1),
            }
        }

        /// The two scalars a loop is read at, each reading its loop.
        trait Reads: Real {
            fn read(rows: Vec<LiftedRow<Self>>) -> Result<TrimRectangle, PropsError> {
                Self::read_collapsed(rows, &[])
            }
            fn read_collapsed(
                rows: Vec<LiftedRow<Self>>,
                collapsed: &[(geom_brep::ChartSide, f64)],
            ) -> Result<TrimRectangle, PropsError>;
        }

        impl Reads for f64 {
            fn read_collapsed(
                rows: Vec<LiftedRow<Self>>,
                collapsed: &[(geom_brep::ChartSide, f64)],
            ) -> Result<TrimRectangle, PropsError> {
                trim_rectangle(&Lift::stated_for_tests(rows, closed()), collapsed)
            }
        }

        impl Reads for Interval {
            fn read_collapsed(
                rows: Vec<LiftedRow<Self>>,
                collapsed: &[(geom_brep::ChartSide, f64)],
            ) -> Result<TrimRectangle, PropsError> {
                trim_rectangle(&Lift::stated_for_tests(rows, closed()), collapsed)
            }
        }

        fn read<T: Reads>(rows: Vec<LiftedRow<T>>) -> Result<TrimRectangle, PropsError> {
            T::read(rows)
        }

        fn refused<T: Reads>(rows: Vec<LiftedRow<T>>, says: &str) {
            match read(rows) {
                Err(PropsError::QuadratureUnsupported { what }) if what.contains(says) => {}
                other => panic!("{}: refused, saying {says:?}: {other:?}", T::NAME),
            }
        }

        fn both(check: impl Fn(&dyn Fn(Vec<(f64, f64)>) -> Result<TrimRectangle, PropsError>)) {
            check(&|c| read::<f64>(polygon(&c)));
            check(&|c| read::<Interval>(polygon(&c)));
        }

        /// The square once round reads `[0, 1]²` and its way: forward
        /// counter-clockwise, and reversed, or traversed backward, clockwise.
        #[test]
        fn the_square_once_round_reads_its_rectangle_and_its_way() {
            both(|read| {
                let ccw = TrimRectangle {
                    rect: rect(0.0, 1.0, 0.0, 1.0),
                    turn: Turn::CounterClockwise,
                    jumps: Vec::new(),
                };
                assert_eq!(read(SQUARE.to_vec()).unwrap(), ccw);
                let cw: Vec<_> = SQUARE.iter().rev().copied().collect();
                assert_eq!(read(cw).unwrap().turn, Turn::Clockwise, "clockwise");
                let mut started = SQUARE.to_vec();
                started.rotate_left(2);
                assert_eq!(read(started).unwrap(), ccw, "the start is gauge");
            });
            let mut backward = polygon::<f64>(&SQUARE);
            for row in &mut backward {
                let (entry, exit) = row.entry_exit_points();
                *row = from_to((exit.x, exit.y), (entry.x, entry.y));
                row.plus = false;
            }
            assert_eq!(
                read(backward).unwrap().turn,
                Turn::CounterClockwise,
                "each traversed backward"
            );
        }

        /// A sub-rectangle bounded by lines at values no chart edge has,
        /// `1/9` among them, reads as those lines: the certificate takes
        /// the rectangle the lines state, never the chart's domain.
        #[test]
        fn a_sub_rectangle_reads_its_own_lines() {
            let (a, b) = (1.0 / 9.0, 0.7);
            both(|read| {
                let r = read(vec![(a, 0.25), (b, 0.25), (b, 1.0 - a), (a, 1.0 - a)]).unwrap();
                assert_eq!(r.rect, rect(a, b, 0.25, 1.0 - a));
            });
        }

        /// **A cut is never asked to be exact.** The bottom side in two
        /// pieces meeting at a cut whose chart value is an enclosure: the
        /// square reads as before, and the joint's gap is padded.
        #[test]
        fn a_side_cut_at_an_inexact_value_reads_its_rectangle() {
            let cut = Interval::from_bounds(1.0 / 3.0 - 1e-16, 1.0 / 3.0 + 1e-16);
            let mut rows = polygon::<Interval>(&SQUARE);
            let (zero, one) = (Interval::zero(), Interval::one());
            let first = piece((zero, zero), (1.0, 0.0));
            let first_short = first.clone().piece_for_tests((zero, cut));
            let second = first.piece_for_tests((cut, one));
            rows.splice(0..1, [first_short, second]);
            let lift = Lift::stated_for_tests(rows, closed());
            assert_eq!(
                trim_rectangle(&lift, &[]).unwrap(),
                TrimRectangle {
                    rect: rect(0.0, 1.0, 0.0, 1.0),
                    turn: Turn::CounterClockwise,
                    jumps: Vec::new(),
                }
            );
            let pad = joint_defect(&lift, (2.0, 2.0), &[0.0; 5], &[]);
            assert!(
                pad > 0.0 && pad < 1e-29,
                "the cut's gap is its enclosure's width, squared: {pad:e}"
            );
        }

        /// A corner whose image ends read as its meet adds nothing to the
        /// pad, and one an end stops short of adds its gap's reach. A gap
        /// of a quarter beside an envelope of `1e-3` is no body's (tier 3
        /// would refuse the loop); the row pins the term's arithmetic.
        #[test]
        fn a_corner_adds_its_gap_to_the_pad() {
            let lift = Lift::stated_for_tests(polygon::<f64>(&SQUARE), closed());
            assert_eq!(joint_defect(&lift, (2.0, 3.0), &[1e-3; 4], &[]), 0.0);
            let mut short = polygon::<f64>(&SQUARE);
            short[0] = from_to((0.0, 0.0), (0.75, 0.0));
            let lift = Lift::stated_for_tests(short, closed());
            assert_eq!(
                trim_rectangle(&lift, &[]).unwrap().rect,
                rect(0.0, 1.0, 0.0, 1.0)
            );
            // The bottom's end stands 0.25 short of `(1, 0)`: a reach of
            // 0.5 at `sup |S_u| = 2`, swept against its envelope.
            let pad = joint_defect(&lift, (2.0, 3.0), &[1e-3; 4], &[]);
            assert!((pad - 0.5 * (0.5 + 2e-3)).abs() < 1e-12, "{pad}");
        }

        /// The joint term's other half: the right column ENTERS a quarter
        /// above the bottom-right corner, so the joint into it measures
        /// `b = 0.25·sup|S_v| = 0.75` from the meet `(1, 0)`, against the
        /// column's own envelope, while the bottom's exit adds nothing.
        /// (A quarter's gap is no body's; the row pins the arithmetic.)
        #[test]
        fn an_entry_short_of_its_corner_adds_its_gap() {
            let mut rows = polygon::<f64>(&SQUARE);
            rows[1] = from_to((1.0, 0.25), (1.0, 1.0));
            let lift = Lift::stated_for_tests(rows, closed());
            assert_eq!(
                trim_rectangle(&lift, &[]).unwrap().rect,
                rect(0.0, 1.0, 0.0, 1.0)
            );
            let e = [1e-3, 2e-3, 1e-3, 1e-3];
            let pad = joint_defect(&lift, (2.0, 3.0), &e, &[]);
            let want = 0.75 * (0.75 + 2.0 * 2e-3);
            assert!((pad - want).abs() < 1e-12, "{pad} against {want}");
        }

        /// A side that runs out and back along its line refuses.
        #[test]
        fn a_back_and_forth_side_refuses() {
            fn each<T: Reads>() {
                let mut rows = polygon::<T>(&SQUARE);
                rows.splice(
                    0..1,
                    [
                        from_to((0.0, 0.0), (1.0, 0.0)),
                        from_to((1.0, 0.0), (0.5, 0.0)),
                        from_to((0.5, 0.0), (1.0, 0.0)),
                    ],
                );
                refused::<T>(rows, "runs back");
            }
            each::<f64>();
            each::<Interval>();
        }

        /// **A side that runs against its neighbours refuses**: the square
        /// with its right column traversed downward, so the bottom arrives
        /// at `(1, 0)` and the column leaves from `(1, 1)`. Four alternating
        /// runs, one value and one way each, so only the walk's own test
        /// ([`TrimRectangle::walked`]) stands between this loop and a
        /// rectangle read with the bottom's way as its winding.
        #[test]
        fn a_side_against_its_neighbours_refuses() {
            fn each<T: Reads>() {
                let mut rows = polygon::<T>(&SQUARE);
                rows[1] = from_to((1.0, 1.0), (1.0, 0.0));
                refused::<T>(rows, "exactly once");
            }
            each::<f64>();
            each::<Interval>();
        }

        /// Three sides, five, and a notch refuse: none is four sides on
        /// four lines.
        #[test]
        fn three_five_and_notched_sides_refuse() {
            fn each<T: Reads>() {
                // Three: the bottom, the right side and the top, the top's
                // row joined to the bottom's across a side no net
                // collapses (`a_collapsed_side_is_a_side_of_the_rectangle`).
                let three = vec![
                    from_to((0.0, 0.0), (1.0, 0.0)),
                    from_to((1.0, 0.0), (1.0, 1.0)),
                    from_to((1.0, 1.0), (0.0, 1.0)),
                ];
                refused::<T>(three, "different chart lines");
                // Five: the square with its left side stopped half way
                // and a row at `v = 1/2` back to the start's column.
                let five = vec![
                    from_to((0.0, 0.0), (1.0, 0.0)),
                    from_to((1.0, 0.0), (1.0, 1.0)),
                    from_to((1.0, 1.0), (0.0, 1.0)),
                    from_to((0.0, 1.0), (0.0, 0.5)),
                    from_to((0.0, 0.5), (0.5, 0.5)),
                ];
                refused::<T>(five, "different chart lines");
                // A notch: the bottom steps up a tenth half way along.
                let notch = polygon::<T>(&[
                    (0.0, 0.0),
                    (0.5, 0.0),
                    (0.5, 0.1),
                    (1.0, 0.1),
                    (1.0, 1.0),
                    (0.0, 1.0),
                ]);
                refused::<T>(notch, "not four sides");
                // A step with no riser: the bottom's two pieces on two rows.
                let mut stepped = polygon::<T>(&SQUARE);
                stepped.splice(
                    0..1,
                    [
                        from_to((0.0, 0.0), (0.5, 0.0)),
                        from_to((0.5, 0.1), (1.0, 0.1)),
                    ],
                );
                refused::<T>(stepped, "different chart lines");
            }
            each::<f64>();
            each::<Interval>();
        }

        /// **A collapsed side is a side of the rectangle**: the three
        /// sides above, the top's row joined to the bottom's across the
        /// net's collapsed `u = 0` column, read the whole square, with
        /// the joint into the bottom row walking that column; across the
        /// collapsed `u = 1` column instead, which the top row does not
        /// reach, they refuse.
        #[test]
        fn a_collapsed_side_is_a_side_of_the_rectangle() {
            use geom_brep::{ChartAxis, ChartEnd, ChartSide};
            fn each<T: Reads>() {
                let three = || {
                    vec![
                        from_to::<T>((0.0, 0.0), (1.0, 0.0)),
                        from_to((1.0, 0.0), (1.0, 1.0)),
                        from_to((1.0, 1.0), (0.0, 1.0)),
                    ]
                };
                let side = |end| ChartSide {
                    fixed: ChartAxis::U,
                    end,
                };
                let read = |collapsed: &[(ChartSide, f64)]| T::read_collapsed(three(), collapsed);
                let got = read(&[(side(ChartEnd::Low), 0.0)]).unwrap();
                assert_eq!(
                    (got.rect, got.turn),
                    (rect(0.0, 1.0, 0.0, 1.0), Turn::CounterClockwise),
                    "{}: the square",
                    T::NAME
                );
                assert_eq!(
                    got.jumps
                        .iter()
                        .map(|(row, s)| (*row, s.axis, s.value))
                        .collect::<Vec<_>>(),
                    vec![(0, ChartAxis::U, 0.0)],
                    "{}: the joint into the bottom row walks the column",
                    T::NAME
                );
                assert!(
                    matches!(
                        read(&[(side(ChartEnd::High), 1.0)]),
                        Err(PropsError::QuadratureUnsupported { what })
                            if what.contains("different chart lines")
                    ),
                    "{}: the other column is not the one the top row reaches",
                    T::NAME
                );
            }
            each::<f64>();
            each::<Interval>();
        }

        /// A side that is no line the certificate reads refuses, naming
        /// why: a slanted line (a curve in UV: no iso image holds one), a
        /// harmonic image (no iso image at all), and an iso image that
        /// does not move, whose way along its line is undecided.
        #[test]
        fn a_side_on_no_held_line_refuses_and_says_why() {
            fn each<T: Reads>() {
                // A slanted line is no iso image: it is stated as a curve
                // in UV, which holds no line however straight its net.
                let f = T::from_f64;
                let mut diagonal = polygon::<T>(&SQUARE);
                let slanted = geom::NurbsCurve2::new(
                    geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
                    vec![Point2::new(1.0, 0.0), Point2::new(1.0 + 1e-3, 1.0)],
                    vec![1.0, 1.0],
                )
                .unwrap();
                diagonal[1].pcurve = Pcurve::General(std::sync::Arc::new(slanted.map_scalar(f)));
                refused::<T>(diagonal, "not an iso image");
                let mut harmonic = polygon::<T>(&SQUARE);
                harmonic[1].pcurve = Pcurve::free_harmonic(
                    Point2::new(f(1.0), f(0.0)),
                    Vec2::new(f(0.0), f(0.0)),
                    Vec2::new(f(1e-3), f(0.0)),
                    Vec2::new(f(0.0), f(1.0)),
                );
                refused::<T>(harmonic, "not an iso image");
                let mut point = polygon::<T>(&SQUARE);
                point.insert(1, piece((T::one(), T::zero()), (0.0, 0.0)));
                refused::<T>(point, "not decided");
            }
            each::<f64>();
            each::<Interval>();
        }

        /// **A held value that is not a point refuses**: a column placed
        /// at a measured foot, its value an enclosure some ulps wide.
        #[test]
        fn a_held_value_that_is_not_a_point_refuses() {
            let foot = Interval::from_bounds(0.75 - 1e-16, 0.75 + 1e-16);
            let zero = Interval::zero();
            let mut rows = polygon::<Interval>(&[(0.0, 0.0), (0.75, 0.0), (0.75, 1.0), (0.0, 1.0)]);
            rows[1] = piece((foot, zero), (0.0, 1.0));
            refused::<Interval>(rows.clone(), "measured foot");
            // The same column at its point reads.
            rows[1] = piece((Interval::from_f64(0.75), zero), (0.0, 1.0));
            assert_eq!(read(rows).unwrap().rect, rect(0.0, 0.75, 0.0, 1.0));
        }

        /// A side whose way along its line is undecided refuses: its rate
        /// an enclosure of zero.
        #[test]
        fn a_side_whose_way_is_undecided_refuses() {
            let mut rows = polygon::<Interval>(&SQUARE);
            let LiftedRow { pcurve, .. } = &mut rows[0];
            *pcurve = Pcurve::IsoLine {
                line: geom_brep::ChartLine::row(Interval::zero()),
                at0: Interval::zero(),
                rate: Interval::from_bounds(-1e-300, 1.0),
            };
            refused::<Interval>(rows, "not decided");
        }

        /// A loop that winds a period of its chart bounds no rectangle of
        /// it, and an open run is no loop: each refuses, saying which. The
        /// winding is stated, as the joints of a loop round a closed chart
        /// compose to one period.
        #[test]
        fn a_loop_that_does_not_close_in_its_chart_refuses_saying_how() {
            let says = |lift: Lift<f64>, text: &str| match trim_rectangle(&lift, &[]) {
                Err(PropsError::QuadratureUnsupported { what }) if what.contains(text) => {}
                other => panic!("refused saying {text:?}: {other:?}"),
            };
            let once_round = Winding {
                deck: Deck {
                    u: 1,
                    v: 0,
                    twin: false,
                },
            };
            says(
                Lift::stated_for_tests(polygon(&SQUARE), Some(once_round)),
                "winds a period",
            );
            says(Lift::stated_for_tests(polygon(&SQUARE), None), "open run");
        }

        /// **A wall trimmed short of its chart's edge certifies the lines
        /// it holds**: a flat patch at `z = 1` over `[0, 2] × [0, 3]`, its
        /// trim's left column one ulp, and then a tenth of the band, in
        /// from the chart's `u = 0`. The rectangle is the column's value,
        /// never the chart's domain, and the flux the patch lane certifies
        /// over it holds the trimmed wall's own, `z·area = 6·(1 − δ)`; over
        /// the domain it would be `6`, which the tenth of the band puts
        /// outside the bracket. The pad is the certificate's own, `Σ L·e`
        /// for images at an envelope `e = 1e-12` plus the joint term, and it
        /// widens the flux by at least `2·D` (the position bound is at
        /// least the plane's height, 1).
        #[test]
        fn a_wall_trimmed_short_of_its_chart_certifies_the_lines_it_holds() {
            use geom_brep::props::quad::{self, PatchNet};
            use geom_core::spline::KnotVector;
            use geom_core::{Band, Tol};
            fn trim<T: Reads>(delta: f64) -> Vec<LiftedRow<T>> {
                polygon(&[(delta, 0.0), (1.0, 0.0), (1.0, 1.0), (delta, 1.0)])
            }
            let tol = Tol::witness();
            let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
            let p = |x: f64, y: f64| [x, y, 1.0].map(Interval::from_f64);
            let control = vec![p(0.0, 0.0), p(0.0, 3.0), p(2.0, 0.0), p(2.0, 3.0)];
            let net = PatchNet::new(&kv, &kv, &control, &[1.0; 4]).unwrap();
            let e = 1e-12;
            // `δ` in chart units: the chart's `u` runs 2 m.
            for delta in [f64::EPSILON / 2.0, tol.eps() / 20.0] {
                let perimeter = 2.0 * 2.0 * (1.0 - delta) + 2.0 * 3.0;
                let lift = Lift::stated_for_tests(trim::<f64>(delta), closed());
                let pad = perimeter * e + joint_defect(&lift, (2.0, 3.0), &[e; 4], &[]);
                for (scalar, r) in [
                    ("f64", read::<f64>(trim(delta)).unwrap()),
                    ("Interval", read::<Interval>(trim(delta)).unwrap()),
                ] {
                    assert_eq!(
                        r.rect,
                        rect(delta, 1.0, 0.0, 1.0),
                        "{scalar}: the column's own value"
                    );
                    let run = quad::nurbs_patch_face::<f64>(
                        &net,
                        r.rect.corners(),
                        perimeter,
                        pad,
                        Band::linear(tol).unwrap(),
                        &mut |_, _| Ok(true),
                    )
                    .unwrap();
                    let flux = run.bounds.flux;
                    let trimmed =
                        Interval::from_f64(6.0) * (Interval::one() - Interval::from_f64(delta));
                    assert!(
                        flux.lo() <= trimmed.lo() && trimmed.hi() <= flux.hi(),
                        "{scalar}, δ = {delta:e}: the certified flux {flux:?} holds the trimmed \
                         wall's {trimmed:?}"
                    );
                    assert!(
                        flux.hi() - flux.lo() >= 2.0 * pad,
                        "{scalar}: the pad {pad:e} widens the flux {flux:?}"
                    );
                    if delta > 1e-12 {
                        assert!(
                            flux.hi() < 6.0,
                            "{scalar}: the domain's flux 6 is outside {flux:?}"
                        );
                    }
                }
            }
        }

        /// **A seam column moved one period of a closed chart is its
        /// domain's other end**, on domains that do not start at 0, end
        /// to end: the column stored at `lo` is moved by the lift's own
        /// door ([`crate::joint::Deck::apply`] on a chart whose `u`
        /// domain is `[lo, hi]`, which moves an iso image's held value
        /// through [`geom_brep::Period::moved`]), and the moved image is
        /// held at `hi` itself, so the certificate reads the rectangle
        /// `[lo, hi]` at `f64` and at `Interval` alike. The sum
        /// `lo + (hi − lo)` it replaces has width at `Interval` on every
        /// one of these domains and at `f64` misses `hi` by an ulp on
        /// three of them. A column that is NOT at an end lands its offset
        /// off the other end, a computed value that at `Interval` is no
        /// point here, and the certificate refuses it, naming the move.
        ///
        /// Red under `Period::moved` adding `k·(hi − lo)` to every value:
        /// the `Interval` rectangles refuse and three `f64` ones miss `hi`.
        #[test]
        fn a_seam_column_moved_one_period_is_the_domains_other_end() {
            const ONCE: Deck = Deck {
                u: 1,
                v: 0,
                twin: false,
            };
            /// The bilinear chart over `[lo, hi] × [0, 1]`.
            fn chart<T: geom_core::Decide>(lo: f64, hi: f64) -> geom::Surface<T> {
                use geom_core::spline::KnotVector;
                let ku = KnotVector::clamped(vec![lo, lo, hi, hi], 1).unwrap();
                let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
                let control = [(0.0, 0.0), (0.0, 1.0), (1.0, 0.0), (1.0, 1.0)]
                    .map(|(x, y)| geom_core::Point3::new(x, y, 0.0))
                    .to_vec();
                let net = geom::NurbsSurface::new(ku, kv, control, vec![1.0; 4]).unwrap();
                geom::Surface::Nurbs(std::sync::Arc::new(net.map_scalar(T::from_f64)))
            }
            /// The square on `[lo, hi] × [0, 1]` whose far column is the
            /// column stored at `at`, as the lift moves it one period of
            /// the chart's `u` channel.
            fn moved<T: Reads + geom_core::Decide>(
                (lo, hi): (f64, f64),
                at: f64,
            ) -> Result<TrimRectangle, PropsError> {
                let f = T::from_f64;
                let mut rows = polygon::<T>(&[(lo, 0.0), (hi, 0.0), (hi, 1.0), (lo, 1.0)]);
                let surface = chart::<T>(lo, hi);
                let periods = crate::pcurves::ChartPeriods::stated(&surface);
                let stored = piece((f(at), T::zero()), (0.0, 1.0));
                // The chart states a `u` period, so the move is made.
                let image = ONCE.apply(&stored.pcurve, &surface, periods).unwrap();
                let mut lifted = stored.moved_for_tests(ONCE);
                lifted.pcurve = image;
                rows[1] = lifted;
                read(rows)
            }
            fn each<T: Reads + geom_core::Decide>() {
                for (lo, hi) in [(0.3, 1.7), (0.4, 1.7), (0.6, 1.7), (0.7, 3.1)] {
                    assert_eq!(
                        moved::<T>((lo, hi), lo).unwrap().rect,
                        rect(lo, hi, 0.0, 1.0),
                        "the seam at {lo} lifted one period of [{lo}, {hi}] is the column {hi}"
                    );
                }
            }
            each::<f64>();
            each::<Interval>();
            // A column an ulp inside the domain is no end: it lands that
            // ulp past the other end, which at `Interval` is no point.
            let (lo, hi) = (0.4_f64, 1.7_f64);
            match moved::<Interval>((lo, hi), f64::from_bits(lo.to_bits() + 1)) {
                Err(PropsError::QuadratureUnsupported { what })
                    if what.contains("computed value") => {}
                other => panic!("Interval: a moved interior column is no point: {other:?}"),
            }
        }

        /// **A ring is never offered as a side.** The certificate reads one
        /// loop; a spline face with a ring refuses at the lane, before any
        /// loop is read (the face walk refuses it earlier still,
        /// `MassPropsError::RingOnCurvedFace`).
        #[test]
        fn a_ring_on_a_spline_face_refuses_at_the_lane() {
            use geom::{NurbsSurface, Surface};
            use geom_core::spline::{KnotVector, Knots};
            use geom_core::{Band, Point3, Tol};
            let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
            let p = |x: f64, y: f64| Point3::new(x, y, 0.0);
            let surface = Surface::Nurbs(std::sync::Arc::new(
                NurbsSurface::new(
                    Knots::from(kv.clone()).lift(Real::from_f64),
                    Knots::from(kv).lift(Real::from_f64),
                    vec![p(0.0, 0.0), p(0.0, 1.0), p(1.0, 0.0), p(1.0, 1.0)],
                    vec![1.0; 4],
                )
                .unwrap(),
            ));
            let tol = Tol::witness();
            let rows = super::super::cut_face_rounds(
                &crate::Body::<f64>::new(),
                &surface,
                true,
                &[(Vec::new(), Vec::new()), (Vec::new(), Vec::new())],
                Band::linear(tol).unwrap(),
                tol,
                geom_brep::props::quad::RoundWindow::SCHEDULE,
                None,
                None,
            );
            match rows {
                Err(PropsError::QuadratureUnsupported { what }) if what.contains("a ring") => {}
                other => panic!("a ring refuses: {other:?}"),
            }
        }
    }
}
