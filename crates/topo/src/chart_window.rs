//! **A face's azimuth window on its chart**: the hull of its boundary
//! edges' chart images, walked in loop order with each edge's branch
//! pinned to the previous edge's exit. The pin is nearest-branch
//! continuity ([`nearest_branch`]), except where the chart carries no
//! azimuth continuity: at a sphere pole the face's orientation picks the
//! branch ([`pole_branch`]), and at a cone apex, where no bit recovers
//! the lift, the walk refuses ([`SplitJoinError::ApexUnlifted`]).
//!
//! [`face_azimuth_images`] is the walk, one image per charted half-edge;
//! [`face_azimuth_window`] and [`face_loop_azimuth_windows`] hull it
//! ([`azimuth_hull`]). The chord join's island winding reads the images
//! themselves ([`run_azimuth_images`]).
//!
//! Its home follows `[topo.homes]` (`crates/topo/README.md`).

use geom_brep::{Pcurve, chart_pcurve_over};
use geom_core::{Band, Decide, InfSpeed, Margin, Real, Sign};

use crate::body::Body;
use crate::chord_join::{SplitJoinError, corrupt_face};
use crate::entity::{EdgeKey, EntityId, FaceKey, HalfEdgeKey, LoopBoundary};
use crate::live::{Proven, linked, proven};
use crate::null::CurveGeom;
use crate::validate::decide;

/// The azimuth window of `face`'s outer loop on `surface`'s chart: the
/// hull of [`face_azimuth_images`]. `None` for a loop that is not a
/// cycle, or a walk with no charted edge.
///
/// # Errors
///
/// As [`face_azimuth_images`].
///
/// # Panics
///
/// As [`face_azimuth_images`].
pub(crate) fn face_azimuth_window<T: Decide>(
    body: &Body<T>,
    surface: &geom::Surface<T>,
    face: FaceKey,
    band: Band,
) -> Result<Option<(T, T)>, SplitJoinError> {
    Ok(face_azimuth_images(body, surface, face, band)?.and_then(|images| azimuth_hull(&images)))
}

/// The azimuth window of each cycle loop of `face` on `surface`'s chart,
/// the outer loop and its rings alike: the hull of the loop's images,
/// `None` for a walk with no charted edge. A lone-vertex loop winds
/// nothing and is not listed.
///
/// # Errors
///
/// As [`face_azimuth_images`], for any loop.
///
/// # Panics
///
/// As [`face_azimuth_images`].
pub(crate) fn face_loop_azimuth_windows<T: Decide>(
    body: &Body<T>,
    surface: &geom::Surface<T>,
    face: FaceKey,
    band: Band,
) -> Result<Vec<Option<(T, T)>>, SplitJoinError> {
    let data = body.passed_face(face).ok_or_else(|| corrupt_face(face))?;
    data.loops()
        .filter_map(|(_, l, _)| match l.boundary {
            LoopBoundary::Cycle { first } => Some(first),
            _ => None,
        })
        .map(|first| {
            let halves = body.loop_walk(first).closed("loop", first);
            run_azimuth_images(body, surface, face, &halves, band)
                .map(|images| azimuth_hull(&images))
        })
        .collect()
}

/// [`face_azimuth_window`] on `face`'s own surface: the window every
/// reader folds, read from outside the crate so a suite can hold the
/// interval lane's window against the `f64` replay's. `sweep-testing`
/// only, never production surface.
///
/// # Errors
///
/// As [`face_azimuth_window`]: [`SplitJoinError::Corrupt`] for a face
/// key that does not resolve.
///
/// # Panics
///
/// As [`face_azimuth_window`], and where the face's surface does not
/// resolve: a torn link past the caller's key (D2 row 4).
#[cfg(feature = "sweep-testing")]
pub fn face_azimuth_window_traces<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    band: Band,
) -> Result<Option<(T, T)>, SplitJoinError> {
    let data = body.passed_face(face).ok_or_else(|| corrupt_face(face))?;
    face_azimuth_window(body, data.surface(), face, band)
}

/// The azimuth hull of a walk's images: the window every caller folds.
pub(crate) fn azimuth_hull<T: Real>(images: &[AzimuthImage<T>]) -> Option<(T, T)> {
    images
        .iter()
        .map(|image| image.range)
        .reduce(|(a, b), (lo, hi)| (a.min(lo), b.max(hi)))
}

/// The **exact** chart-azimuth extent of a pcurve over `[t₀, t₁]`.
///
/// On a cylinder chart [`geom_brep::chart_pcurve`] produces an azimuth
/// channel that is exactly `α + β·t` (it writes `pa.x = pb.x = 0` and
/// `pl.x = β ∈ {−1, 0, +1}` in both of its arms), so the two endpoint
/// evaluations ARE the range — this is closed-form structure, not a
/// sampled bound. A focal section's azimuth is strictly
/// monotone, so its endpoints are its range too. It is read off
/// [`Pcurve::closed_form_span_box`], whose trigonometric widening is
/// exactly zero for every pcurve this lane derives and keeps the
/// statement true if the family ever widens.
///
/// **The closed-form lane only, and it says so with `None`.** The
/// window walk reads a chart image's azimuth through its closed form; a
/// fitted (rung-3) image has none, so it gets no answer rather than a
/// sentinel. A sentinel would be actively wrong here: the caller hulls
/// with `(a.min(lo), b.max(hi))`, which ABSORBS an inverted range
/// silently instead of propagating it, so "the empty range makes the
/// window refuse" would have been false. The caller reads `None` by the
/// image: a projected image (a circle's lane choice on a sphere, a
/// spline's only image) refuses `RunImageNotYet`, and a spiric image
/// `SectionInvariant`. A fitted image never reaches this function:
/// `chart_pcurve` derives none.
fn chart_azimuth_range<T: Real>(p: &Pcurve<T>, t0: T, t1: T) -> Option<(T, T)> {
    p.closed_form_span_box(t0, t1).map(|b| (b.u_min, b.u_max))
}

/// One boundary half-edge's chart azimuth image, on the branch the
/// walk pinned: the azimuth where the walk ENTERS it and where it
/// EXITS it (the half-edge's own start and end), and the hull
/// [`face_azimuth_window`] folds.
#[derive(Clone, Debug)]
pub(crate) struct AzimuthImage<T: geom_core::Real> {
    /// The azimuth at its start vertex.
    pub(crate) entry: T,
    /// The azimuth at its end vertex.
    pub(crate) exit: T,
    /// Its azimuth extent over its edge's span ([`chart_azimuth_range`]).
    pub(crate) range: (T, T),
    /// The chart's second coordinate at its start and end vertices.
    pub(crate) v: (T, T),
    /// The carrier parameter at its start and end vertices.
    pub(crate) t: (T, T),
    /// Its edge's certified carrier, as the walk read it.
    pub(crate) carrier: geom::Curve3<T>,
    /// The half-edge's chart image when it is harmonic (a closed form,
    /// read by the chord join's island winding and evaluated directly);
    /// `None` for a fitted image.
    pub(crate) harmonic: Option<Pcurve<T>>,
}

/// Every charted half-edge of `face`'s outer loop, in loop order, with
/// its azimuth image on the branch the walk pins — the same walk
/// [`face_azimuth_window`] hulls, so each image's entry and exit sit on
/// the window's own branch. `None` for a loop that is not a cycle.
///
/// # Errors
///
/// [`SplitJoinError::Corrupt`] where `face`, the caller's key, does not
/// resolve; otherwise the walk's geometric refusals: a boundary edge with no
/// closed-form chart image, a fitted image, a vertex off the carrier, an
/// apex junction ([`SplitJoinError::ApexUnlifted`]), or an escalation.
///
/// # Panics
///
/// On a torn hop past `face` ([`crate::live::FaceView::outer_cycle`], [`run_azimuth_images`]).
pub(crate) fn face_azimuth_images<T: Decide>(
    body: &Body<T>,
    surface: &geom::Surface<T>,
    face: FaceKey,
    band: Band,
) -> Result<Option<Vec<AzimuthImage<T>>>, SplitJoinError> {
    let data = body.passed_face(face).ok_or_else(|| corrupt_face(face))?;
    let Some(halves) = data.outer_cycle() else {
        return Ok(None);
    };
    run_azimuth_images(body, surface, face, &halves, band).map(Some)
}

/// **How a boundary edge's chart-image refusal ends the walk** (D3): a
/// class reading or a lane not built "not yet" ([`SplitJoinError::RunImageNotYet`]);
/// an incidence refusal by its read, through the one map
/// ([`SplitJoinError::RunImageIncidence`]); another escalation as the
/// walk's; anything else an invariant the shipped carriers keep.
fn run_image_refusal(
    face: FaceKey,
    kind: geom::SurfaceKind,
    e: geom_brep::PcurveCertifyError,
) -> SplitJoinError {
    match e {
        // The class reading is a lane choice (D3): in band, and
        // with no general lane written, it ends "not yet".
        geom_brep::PcurveCertifyError::Escalated {
            check: geom_brep::PcurveCheck::ChartClass | geom_brep::PcurveCheck::GeneralLane,
            ..
        }
        | geom_brep::PcurveCertifyError::NoGeneralLane { .. } => {
            SplitJoinError::RunImageNotYet { face, kind }
        }
        // An incidence refusal ends by its read, through the one
        // map, never as an invariant break or the walk's own lever.
        refusal if geom_brep::IncidenceRead::of(&refusal).is_some() => {
            SplitJoinError::RunImageIncidence {
                face,
                kind,
                refusal,
            }
        }
        geom_brep::PcurveCertifyError::Escalated { cause, .. } => {
            SplitJoinError::Escalated { face, diag: cause }
        }
        _ => SplitJoinError::SectionInvariant {
            face,
            what: "a run edge has no closed-form chart image on the divided face's chart",
        },
    }
}

/// The walk behind [`face_azimuth_images`]:
/// each charted half-edge's image, its branch pinned to the previous
/// edge's exit.
///
/// `halves` are members of a loop walk the caller closed, so each is
/// proven, and its edge and that edge's curve are links: a miss panics
/// (tier 1 at rest, [`crate::live::OPERATORS_KEEP_LINKS`]
/// mid-operation), and only null scaffolding is skipped.
pub(crate) fn run_azimuth_images<T: Decide>(
    body: &Body<T>,
    surface: &geom::Surface<T>,
    face: FaceKey,
    halves: &[HalfEdgeKey],
    band: Band,
) -> Result<Vec<AzimuthImage<T>>, SplitJoinError> {
    let tau = T::tau();
    let mut images = Vec::new();
    let mut prev_exit: Option<T> = None;
    let mut prev_edge: Option<EdgeKey> = None;
    let mut prev_k = 0;
    for &he in halves {
        let he_data = proven(&body.half_edges, he, EntityId::HalfEdge);
        let edge = linked(
            &body.edges,
            he_data.edge,
            EntityId::Edge,
            EntityId::HalfEdge(he),
            "edge",
        );
        let CurveGeom::Certified(curve) = body.edge_curve_linked(he_data.edge, edge) else {
            continue; // null scaffolding: zero-length, no azimuth extent
        };
        let (t0, t1) = curve.params();
        let base = chart_pcurve_over(curve.carrier(), t0, t1, surface, band)
            .map_err(|e| run_image_refusal(face, surface.kind(), e))?;
        let plus = crate::pcurves::is_plus(body, he);
        let (entry_t, exit_t) = if plus { (t0, t1) } else { (t1, t0) };
        let pcurve = match prev_exit {
            None => base,
            Some(prev) => {
                let raw = base.eval(entry_t).x;
                // The branch pin. Default: nearest-branch continuity —
                // the PR 6 loop-walk rule, exact wherever the previous
                // edge's exit and this edge's entry share a chart
                // point, decided as structure ([`nearest_branch`]).
                //
                // A sphere POLE carries no azimuth continuity (the
                // chart is singular there, and two meridians half a
                // period apart make the nearest pin a tie), so the
                // face's orientation picks the branch ([`pole_branch`]).
                let mut pole_k = None;
                // A cone APEX junction carries no azimuth at all, and
                // unlike a sphere pole no orientation bit recovers it:
                // the jump there is the whole lift's, which no pin
                // crosses. A walk that would is refused rather than
                // pinned.
                if let geom::Surface::Cone { apex, .. } = surface {
                    let entry_v = he_data.start;
                    let p = body.resolve_vertex_point(entry_v, Proven);
                    if decide(
                        "chart_window_cone_apex",
                        Margin::of((p - *apex).norm()),
                        band,
                    )
                    .map_err(|diag| SplitJoinError::Escalated { face, diag })?
                        == Sign::Zero
                    {
                        return Err(SplitJoinError::ApexUnlifted { face });
                    }
                }
                if let geom::Surface::Sphere {
                    center,
                    radius,
                    axis,
                    ..
                } = surface
                {
                    let entry_v = he_data.start;
                    let p = body.resolve_vertex_point(entry_v, Proven);
                    let d = (p - *center).dot(*axis);
                    match decide(
                        "chart_window_sphere_pole",
                        Margin::of(*radius - d.abs()),
                        band,
                    )
                    .map_err(|diag| SplitJoinError::Escalated { face, diag })?
                    {
                        Sign::Positive => {}
                        Sign::Negative => {
                            return Err(SplitJoinError::SectionInvariant {
                                face,
                                what: "a run vertex lies off its sphere face's carrier (its \
                                       axial offset exceeds the radius)",
                            });
                        }
                        Sign::Zero => {
                            let south =
                                match decide("chart_window_sphere_pole_side", Margin::of(d), band)
                                    .map_err(|diag| SplitJoinError::Escalated { face, diag })?
                                {
                                    Sign::Negative => true,
                                    Sign::Positive => false,
                                    Sign::Zero => {
                                        return Err(SplitJoinError::SectionInvariant {
                                            face,
                                            what: "a run vertex reads as both a pole and an \
                                                   equator point — a zero-radius sphere face",
                                        });
                                    }
                                };
                            let sense =
                                body.get_face(face).ok_or_else(|| corrupt_face(face))?.sense;
                            let advancing = south == sense;
                            let prior: &AzimuthImage<T> =
                                images.last().ok_or(SplitJoinError::SectionInvariant {
                                    face,
                                    what: "a pole joint has no previous image",
                                })?;
                            pole_k = Some(pole_branch(
                                PoleJoint {
                                    face,
                                    surface: (*center, *radius, *axis),
                                    pole_height: d,
                                    advancing,
                                    gap: prev - raw,
                                    prev_k,
                                    same_edge: prev_edge == Some(he_data.edge),
                                },
                                (&prior.carrier, prior.t.1, prior.t.0),
                                (curve.carrier(), entry_t, exit_t),
                                band,
                            )?);
                        }
                    }
                }
                let k = match pole_k {
                    Some(k) => k,
                    None => nearest_branch(
                        surface,
                        face,
                        body.resolve_vertex_point(he_data.start, Proven),
                        prev - raw,
                        band,
                    )?,
                };
                prev_k = k;
                base.shift_branch(k, geom_brep::Period::new(tau))
            }
        };
        // A general-lane image (a circle's class read in band on a sphere,
        // or a spline's only image: the projected lane) has no closed-form
        // azimuth box: the walk's lane for it is not written, which is no
        // invariant break. The other image the derivation above returns
        // without a box is a spiric carrier's exact `Pcurve::Spiric` (a
        // torus wall's plane section); it ends `SectionInvariant` today,
        // though it is one of the chart's own curves and the walk's lane
        // for it is simply not written
        // (`work/pcert/a-spiric-run-edge-ends-the-window-walk-as-an-invariant.md`).
        // A fitted image never comes out of that derivation.
        let (lo, hi) = chart_azimuth_range(&pcurve, t0, t1).ok_or(match &pcurve {
            Pcurve::Projected(_) | Pcurve::General(_) => SplitJoinError::RunImageNotYet {
                face,
                kind: surface.kind(),
            },
            _ => SplitJoinError::SectionInvariant {
                face,
                what: "a boundary edge's chart image has no closed-form azimuth box (a spiric \
                       image) — the window walk reads a closed-form azimuth",
            },
        })?;
        let exit = pcurve.eval(exit_t).x;
        images.push(AzimuthImage {
            entry: pcurve.eval(entry_t).x,
            exit,
            range: (lo, hi),
            v: (pcurve.eval(entry_t).y, pcurve.eval(exit_t).y),
            t: (entry_t, exit_t),
            carrier: curve.carrier().clone(),
            harmonic: matches!(pcurve, Pcurve::Harmonic { .. }).then(|| pcurve.clone()),
        });
        prev_exit = Some(exit);
        prev_edge = Some(he_data.edge);
    }
    Ok(images)
}

/// **The nearest branch, as structure**: the whole number of periods
/// `k` that lands an image's entry within half a period of the previous
/// exit, `gap` apart, at the joint's `vertex`. Decided by sign decisions
/// at the half-period marks, metered at the vertex's own lever
/// (`pcurves::joint_arm`), with the loop walk's near-pole read where the
/// marks miss (`pcurves::lever_periods`), so `k` is a literal at every
/// scalar. The walk answers a sphere pole by orientation and refuses a
/// cone apex before it gets here; a vertex decided clear of the apex can
/// still have a lever under the band (`d·sin α` on a narrow cone), which
/// the near-pole read serves.
///
/// # Errors
///
/// [`SplitJoinError::Escalated`] where a mark is in band, or the gap
/// sits on a mark with a branch the near-pole read tries in band;
/// [`SplitJoinError::SectionInvariant`] where the gap sits on a mark off
/// every branch that read tries, or the chart
/// is the placeholder.
fn nearest_branch<T: Decide>(
    surface: &geom::Surface<T>,
    face: FaceKey,
    vertex: geom_core::Point3<T>,
    gap: T,
    band: Band,
) -> Result<i32, SplitJoinError> {
    let chart = crate::pcurves::DescribedChart::of(surface).ok_or(SplitJoinError::SectionInvariant {
        face,
        what: "a divided face's chart is the placeholder — it has no lever to decide a branch by",
    })?;
    let arm = crate::pcurves::joint_arm(chart, vertex);
    crate::pcurves::lever_periods(
        "chart_window_branch",
        arm,
        gap,
        geom_brep::Period::new(T::tau()),
        crate::pcurves::Orbit::Periods,
        band,
    )
    .map_err(|miss| match miss {
        geom_brep::BranchMiss::Undecided(diag) => SplitJoinError::Escalated { face, diag },
        geom_brep::BranchMiss::OnMark => SplitJoinError::SectionInvariant {
            face,
            what: "a run edge's chart entry sits half a period from the previous edge's exit — \
                   the two images name no common branch",
        },
    })
}

/// A boundary vertex on a sphere pole, as [`pole_branch`] reads it.
struct PoleJoint<T: Decide> {
    face: FaceKey,
    /// The sphere's centre, radius and axis.
    surface: (geom_core::Point3<T>, T, geom_core::Vec3<T>),
    /// The pole's axial offset from the centre: `±radius`.
    pole_height: T,
    /// Whether azimuth advances through this pole on the face's walk.
    advancing: bool,
    /// The previous exit less this entry's unshifted azimuth.
    gap: T,
    /// The previous image's branch.
    prev_k: i32,
    /// Whether the walk leaves the pole along the edge it arrived on.
    same_edge: bool,
}

/// **The strict branch at a sphere pole**: the chart carries no azimuth
/// continuity there, so the face's orientation carries the bit. Walking
/// an outward face's cycle with the face on the left, azimuth ADVANCES
/// through the south pole (the entry is the unique branch in
/// `(prev, prev + τ)`) and RETURNS through the north pole
/// (`(prev − τ, prev)`); a reversed face swaps the poles' roles.
///
/// The interval is open, and its ends are where the two edges leave the
/// pole along one direction. That is structure, never a number:
/// - the walk leaves along the edge it arrived on, so the face wraps the
///   pole: the previous branch one whole turn on, `prev_k ± 1`, a
///   literal at every scalar;
/// - two distinct edges: [`geom_brep::Period::decide`] of
///   `gap ± τ/2`, its marks at whole periods, metered by D4 ¶1's
///   parallelism lever ([`pole_lever`]). A mark decided Zero is two
///   distinct edges leaving along one direction, an overlap or a cusp,
///   and refuses ([`SplitJoinError::PoleEdgesCoincide`]); one in band
///   escalates.
///
/// `arrived` and `leaving` are the two edges' carriers with the
/// parameters of each one's pole end and far end.
///
/// Where the lever is so short that both marks either side of the gap
/// lie under the escalation band, which is read first decides between
/// the refusal and the escalation; both fail loud.
fn pole_branch<T: Decide>(
    joint: PoleJoint<T>,
    arrived: (&geom::Curve3<T>, T, T),
    leaving: (&geom::Curve3<T>, T, T),
    band: Band,
) -> Result<i32, SplitJoinError> {
    let PoleJoint {
        face,
        surface,
        pole_height,
        advancing,
        gap,
        prev_k,
        same_edge,
    } = joint;
    let step = if advancing { 1 } else { -1 };
    if same_edge {
        return Ok(prev_k + step);
    }
    let lever = match (
        pole_lever(arrived, surface, pole_height),
        pole_lever(leaving, surface, pole_height),
    ) {
        (Some(a), Some(b)) => a.min(b),
        _ => {
            return Err(SplitJoinError::SectionInvariant {
                face,
                what: "an edge at a sphere pole is not a circle — the pole's branch has no \
                       lever to decide it by",
            });
        }
    };
    let half = T::tau() * T::from_f64(0.5);
    let shifted = if advancing { gap + half } else { gap - half };
    // **What the lever is.** Not a rate either edge has throughout: a
    // point's azimuth rate is its distance from the axis, zero at the pole
    // and at most the edge's reach, so over either edge the lever is that
    // rate's sup, and its inf is zero. What the marks decide is the two
    // edges' largest separation, which on the meridians that reach here is
    // the chord `2ρ·sin(g/2)` at the reach `ρ` both share: `g·lever`
    // overstates it by a relative `g²/24` at most, and where `g·ρ` is
    // within a few bands of a mark that is a few bands squared over `ρ²`,
    // beneath any band's resolution. So the lever is
    // the decision's inf to that order, and the decision is the side of
    // the mark the pair's separation lies on. It is no inf on a curved
    // edge bending back (the lens, [`pole_lever`]'s four times), which
    // never gets here: a non-meridian circle's chart image is projected,
    // and the walk refuses it as FITTED.
    geom_brep::Period::new(T::tau())
        .decide(
            "chart_window_pole_branch",
            shifted,
            |g| Margin::metered(g, InfSpeed::new(lever)),
            band,
        )
        .map_err(|stop| match stop.miss {
            geom_brep::BranchMiss::Undecided(diag) => SplitJoinError::Escalated { face, diag },
            geom_brep::BranchMiss::OnMark => SplitJoinError::PoleEdgesCoincide { face },
        })
}

/// **The length over which an angle at a sphere pole displaces an
/// edge**: the edge's furthest distance from the axis, capped at its
/// curvature radius. The ruling's "relative curvature radius" is read as
/// the edge's own `1/κ_g`, its geodesic curvature against the sphere,
/// which a meridian lacks.
///
/// The pole is where the edge's circle peaks in axial height, so the
/// height falls monotonically for half the circle from it. The arc's
/// furthest point from the axis is therefore where it has swept half its
/// circle from the pole, `pole_t ± π`, or its far end `far_t` if that
/// comes first; and it is the sphere's radius where that point lies
/// across the equator. An edge whose far end is back at the pole (a
/// closed circle, a near-full sweep) reads its antipode, not its end.
///
/// **What it bounds.** Two meridians `θ` apart separate by at most
/// `2ρ·sin(θ/2) ≤ θ·ρ` at reach `ρ`, so for them the meter `θ·lever`
/// is exact to first order. Against a curved edge bending back toward
/// its partner (the lens between a meridian and a small circle) it
/// overstates the pair's largest separation about four times
/// (`work/pcert/the-pole-lever-overstates-a-lens-about-four-times.md`),
/// which a band with `K` under 4 would decide apart. No answer rides on
/// that today: a non-meridian circle's chart image is projected, and the
/// walk refuses it as FITTED.
///
/// `None` for a carrier other than a circle, the only one a sphere
/// section draws through a pole.
fn pole_lever<T: Decide>(
    (carrier, pole_t, far_t): (&geom::Curve3<T>, T, T),
    (center, radius, axis): (geom_core::Point3<T>, T, geom_core::Vec3<T>),
    pole_height: T,
) -> Option<T> {
    let geom::Curve3::Circle(geom::Circle { radius: r, .. }) = *carrier else {
        return None;
    };
    let swept = (far_t - pole_t).max(T::zero() - T::pi()).min(T::pi());
    let w = carrier.eval(pole_t + swept) - center;
    let height = w.dot(axis);
    let far = (w - axis * height).norm();
    let reach = (height * pole_height).select_le_zero(radius, far);
    let bend = (radius.powi(2) - r.powi(2)).max(T::zero()).sqrt();
    Some((reach * bend - r * radius).select_le_zero(reach, r * radius / bend))
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod first_image_rows;

/// **A walk's joint branch is a literal.** Over a box, an entry `k`
/// periods from the previous exit as polynomials that are not constants,
/// on a cylinder, at `k` either side of zero: the branch less `k` is the
/// zero polynomial. And on a narrow cone (`sin α ≈ 0.01`), a continuous
/// joint at a vertex decided clear of the apex but with its lever
/// `d·sin α` under the band takes a lift of its own point by the
/// near-pole read, where every mark reads Zero or in band: the image
/// itself at a zero gap, the image a period over at a whole turn's.
#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod nearest_branch_rows {
    use super::*;
    use geom_core::{Interval, Point3, Real, Sym, Tol, Vec3};

    #[test]
    fn the_nearest_branch_is_a_literal_over_a_box() {
        let band = Band::linear(Tol::witness()).unwrap();
        for k in [-7.0, -1.0, 0.0, 1.0, 2.0, 5.0] {
            let (out, counts) =
                crate::test_support_fixtures::whole_period_gap_over_a_box(k, |a, b| {
                    let f = Sym::<Interval>::from_f64;
                    let cylinder = geom::Surface::Cylinder {
                        origin: Point3::new(f(0.0), f(0.0), f(0.0)),
                        axis: Vec3::new(f(0.0), f(0.0), f(1.0)),
                        radius: f(2.0),
                        u_ref: Vec3::new(f(1.0), f(0.0), f(0.0)),
                    };
                    let vertex = Point3::new(f(2.0), f(0.0), f(0.0));
                    let got =
                        nearest_branch(&cylinder, FaceKey::default(), vertex, a - b, band).unwrap();
                    decide(
                        "nearest_branch_row",
                        Margin::of(f(f64::from(got)) - f(k)),
                        band,
                    )
                });
            assert!(
                matches!(out, Ok(Sign::Zero)),
                "k = {k}: the branch is {k} periods: {out:?}"
            );
            assert_eq!(
                counts.symbolic, 1,
                "k = {k}: the branch less k is the zero polynomial"
            );
        }
    }

    /// **A gap on a mark answers by the band at the lever, however many
    /// turns out**, as the loop walk's joint does: the branches astride
    /// the walk's mark sit `π·lever` off, so in band the split escalates
    /// and past it the gap names no common branch.
    #[test]
    fn a_half_turn_gap_answers_by_the_band_at_every_turn_count() {
        let band = Band::linear(Tol::witness()).unwrap();
        let alpha = 0.01f64;
        let cone = geom::Surface::Cone {
            apex: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            half_angle: alpha,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let (pi, tau) = (core::f64::consts::PI, core::f64::consts::TAU);
        let esc = band.escalate();
        let at = |lever: f64, gap: f64| {
            let d = lever / alpha.sin();
            let vertex = Point3::new(lever, 0.0, d * alpha.cos());
            nearest_branch(&cone, FaceKey::default(), vertex, gap, band)
        };
        for f in [0.55, 0.7, 0.75, 0.85, 0.95, 0.99] {
            let lever = f * esc / pi;
            for turns in [0.5, -0.5, 1.5, -1.5, 2.5, -2.5, 3.5, -3.5] {
                let got = at(lever, turns * tau);
                assert!(
                    matches!(got, Err(SplitJoinError::Escalated { .. })),
                    "π·lever {f}·escalate, gap {turns}τ: in band: {got:?}"
                );
            }
        }
        let past = 1.01 * esc / pi;
        for turns in [0.5, -2.5, 3.5] {
            let got = at(past, turns * tau);
            assert!(
                matches!(got, Err(SplitJoinError::SectionInvariant { .. })),
                "π·lever 1.01·escalate, gap {turns}τ: decided off: {got:?}"
            );
        }
        for (turns, k) in [(2.0, 2), (-2.0, -2)] {
            let got = at(0.75 * esc / pi, turns * tau);
            assert!(
                matches!(got, Ok(b) if b == k),
                "gap {turns}τ: the branch is {k}: {got:?}"
            );
        }
    }

    #[test]
    fn a_narrow_cone_joint_clear_of_the_apex_takes_a_lift_of_its_point() {
        let band = Band::linear(Tol::witness()).unwrap();
        let alpha = 0.01f64;
        let cone = geom::Surface::Cone {
            apex: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            half_angle: alpha,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        // From the band's escalate edge, where the incidence first decides
        // the vertex clear of the apex and its lever is a hundredth of
        // that, to a thousand times it, where every mark decides.
        for d in [1.0, 5.0, 10.0, 100.0, 1000.0].map(|m| m * band.escalate()) {
            let vertex = Point3::new(d * alpha.sin(), 0.0, d * alpha.cos());
            assert_eq!(
                decide("cone_row_apex", Margin::of(d), band),
                Ok(Sign::Positive),
                "d = {d:e}: the vertex is decided clear of the apex"
            );
            let lever = d * alpha.sin();
            let got = nearest_branch(&cone, FaceKey::default(), vertex, 0.0, band);
            assert!(
                matches!(got, Ok(0)),
                "d = {d:e} (lever {lever:e}): the continuous joint is its own branch: {got:?}"
            );
            // A whole turn's gap meets the image a period over; where a
            // whole turn at the lever is itself within ε, every branch
            // tried is, and the image itself is taken first.
            let tau = core::f64::consts::TAU;
            for turns in [1.0, -1.0] {
                let want = if tau * lever < band.zero() {
                    0.0
                } else {
                    turns
                };
                let got = nearest_branch(&cone, FaceKey::default(), vertex, turns * tau, band);
                assert!(
                    matches!(got, Ok(k) if f64::from(k) == want),
                    "d = {d:e} (lever {lever:e}), gap {turns}·τ: the branch is {want}: {got:?}"
                );
            }
            // Half a turn round: within ε at the lever the image is taken;
            // in band the two ends may be one point, an escalation; past
            // the band the gap names no common branch.
            let pi_lever = core::f64::consts::PI * lever;
            let got = nearest_branch(&cone, FaceKey::default(), vertex, 0.5 * tau, band);
            let read = if pi_lever < band.zero() {
                matches!(got, Ok(0))
            } else if pi_lever < band.escalate() {
                matches!(got, Err(SplitJoinError::Escalated { .. }))
            } else {
                matches!(got, Err(SplitJoinError::SectionInvariant { .. }))
            };
            assert!(read, "d = {d:e} (lever {lever:e}), gap π: {got:?}");
        }
    }
}

/// The strict branch at a sphere pole ([`pole_branch`]): its lever, the
/// same-edge whole turn, and the refusal on a mark.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod pole_branch_rows {
    use super::*;
    use geom_core::{Interval, Point3, Real, Tol, Vec3};

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    /// The unit sphere about `z`, its north pole at `z = 1`.
    fn sphere<T: Real>() -> (Point3<T>, T, Vec3<T>) {
        let f = T::from_f64;
        (
            Point3::new(f(0.0), f(0.0), f(0.0)),
            f(1.0),
            Vec3::new(f(0.0), f(0.0), f(1.0)),
        )
    }

    /// The unit sphere's meridian at azimuth `phi`, as a circle whose
    /// parameter `t` is the polar angle from the north pole.
    fn meridian<T: Real>(phi: f64) -> geom::Curve3<T> {
        let f = T::from_f64;
        let (s, c) = phi.sin_cos();
        geom::Curve3::circle(
            Point3::new(f(0.0), f(0.0), f(0.0)),
            Vec3::new(f(-s), f(c), f(0.0)),
            f(1.0),
            Vec3::new(f(0.0), f(0.0), f(1.0)),
        )
    }

    /// The circle through the north pole cut by the plane whose normal
    /// leans `beta` from the axis toward `+x`: radius `sin β`, geodesic
    /// curvature radius `tan β`, leaving the pole along `−y` (azimuth
    /// `−π/2`). `t = 0` is the pole, `t = π` the point across the circle
    /// from it.
    fn small_circle(beta: f64) -> geom::Curve3<f64> {
        let (s, c) = beta.sin_cos();
        geom::Curve3::circle(
            Point3::new(s * c, 0.0, c * c),
            Vec3::new(s, 0.0, c),
            s,
            Vec3::new(-c, 0.0, s),
        )
    }

    fn joint<T: geom_core::Decide>(gap: T, same_edge: bool) -> PoleJoint<T> {
        PoleJoint {
            face: FaceKey::default(),
            surface: sphere(),
            pole_height: T::one(),
            advancing: true,
            gap,
            prev_k: 0,
            same_edge,
        }
    }

    /// **The lever is the stub's reach, not the sphere's radius**: two
    /// meridian stubs leaving the pole `θ` apart, each `s` long, with
    /// `θ·s` a thousandth of ε while `θ·R` is a hundred escalation
    /// bands. Everywhere along them the two edges are within ε, so the
    /// pole reads them as leaving along one direction and refuses; a
    /// lever at the sphere's radius would decide them apart and hand
    /// back a branch.
    #[test]
    fn a_short_stub_within_eps_everywhere_refuses_at_its_mark() {
        let band = band();
        let theta = 100.0 * band.escalate();
        let s = band.zero() / (1000.0 * theta);
        let (a, b) = (meridian::<f64>(0.0), meridian::<f64>(theta));
        let lever = pole_lever((&a, 0.0, s), sphere(), 1.0).unwrap();
        assert!(
            (lever - s.sin()).abs() <= 1e-15 * s,
            "θ = {theta:e}, s = {s:e}: the stub's lever is its reach sin s, not R = 1: {lever:e}"
        );
        let got = pole_branch(joint(-theta, false), (&a, 0.0, s), (&b, 0.0, s), band);
        assert!(
            matches!(got, Err(SplitJoinError::PoleEdgesCoincide { .. })),
            "θ = {theta:e}, s = {s:e}: the stubs are within ε everywhere: {got:?}"
        );
        // The same angle on full meridians, pole to pole, is a wedge.
        let got = pole_branch(joint(-theta, false), (&a, 0.0, PI), (&b, 0.0, PI), band);
        assert_eq!(
            got.ok(),
            Some(0),
            "θ = {theta:e}: full meridians leave the pole definitely apart, the entry on \
             its own branch just past the exit"
        );
    }

    /// **A curved edge's lever is capped at its geodesic curvature
    /// radius**: a small circle through the pole sweeping most of its
    /// circle reaches nearly its diameter from the axis, but bends away
    /// from its pole tangent over `tan β`. A meridian leaving `θ` off
    /// that tangent, `θ·tan β` under ε, is within ε of it over the bend,
    /// and the pole refuses; levered at the reach the same wedge would
    /// read in band.
    #[test]
    fn a_curved_edge_through_the_pole_is_levered_at_its_curvature_radius() {
        let band = band();
        let beta: f64 = 0.3;
        let far = 0.9 * PI;
        let curved = small_circle(beta);
        let end = curved.eval(far);
        let reach = end.x.hypot(end.y);
        let lever = pole_lever((&curved, 0.0, far), sphere(), 1.0).unwrap();
        assert!(
            (lever - beta.tan()).abs() <= 1e-12 && lever < 0.75 * reach,
            "β = {beta}: the lever is tan β = {:e}, under the reach {reach:e}: {lever:e}",
            beta.tan()
        );
        let theta = 0.9 * band.zero() / beta.tan();
        let m = meridian::<f64>(-PI / 2.0 + theta);
        let got = pole_branch(
            joint(-theta, false),
            (&m, 0.0, PI),
            (&curved, 0.0, far),
            band,
        );
        assert!(
            matches!(got, Err(SplitJoinError::PoleEdgesCoincide { .. })),
            "θ = {theta:e}: a meridian within ε of the curved edge over its bend: {got:?}"
        );
        // A meridian half a turn off is clear at the same lever.
        let m = meridian::<f64>(PI / 2.0);
        let got = pole_branch(joint(-PI, false), (&m, 0.0, PI), (&curved, 0.0, far), band);
        assert_eq!(
            got.ok(),
            Some(0),
            "a half-turn wedge against the curved edge"
        );
    }

    /// **The same edge in and out is a whole turn, as a literal**: the
    /// walk enters the pole along an edge and leaves along it, so the
    /// face wraps the pole and the entry is the previous branch one turn
    /// on (advancing) or back (returning), at `f64` and over an interval
    /// gap straddling the jump alike.
    #[test]
    fn the_same_edge_in_and_out_is_a_whole_turn_at_every_scalar() {
        let band = band();
        let a = meridian::<f64>(0.0);
        for (advancing, want) in [(true, 4), (false, 2)] {
            let at_f64 = pole_branch(
                PoleJoint {
                    advancing,
                    prev_k: 3,
                    ..joint(3.0 * TAU, true)
                },
                (&a, 0.0, PI),
                (&a, 0.0, PI),
                band,
            );
            assert_eq!(at_f64.ok(), Some(want), "f64, advancing {advancing}");
            let ai = meridian::<Interval>(0.0);
            let straddle = Interval::from_f64(3.0 * TAU) + Interval::from_bounds(-1e-15, 1e-15);
            let at_interval = pole_branch(
                PoleJoint {
                    advancing,
                    prev_k: 3,
                    ..joint(straddle, true)
                },
                (&ai, Interval::zero(), Interval::pi()),
                (&ai, Interval::zero(), Interval::pi()),
                band,
            );
            assert_eq!(
                at_interval.ok(),
                Some(want),
                "Interval over a gap straddling the jump, advancing {advancing}"
            );
        }
    }

    /// **Two distinct edges on one direction refuse, typed, naming both
    /// readings**: two whole meridians at one azimuth (an overlap),
    /// whatever the scalar.
    #[test]
    fn distinct_edges_on_a_mark_refuse_as_an_overlap_or_a_cusp() {
        let band = band();
        let a = meridian::<f64>(0.5);
        let got = pole_branch(joint(0.0, false), (&a, 0.0, PI), (&a, 0.0, PI), band);
        let Err(e @ SplitJoinError::PoleEdgesCoincide { .. }) = got else {
            panic!("an exact overlap at the pole: {got:?}");
        };
        let said = e.render(geom_brep::recourse::Reading::Build);
        assert!(
            said.contains("overlap") && said.contains("cusp"),
            "the refusal names both readings: {said}"
        );
        let ai = meridian::<Interval>(0.5);
        let got = pole_branch(
            joint(Interval::from_f64(TAU), false),
            (&ai, Interval::zero(), Interval::pi()),
            (&ai, Interval::zero(), Interval::pi()),
            band,
        );
        assert!(
            matches!(got, Err(SplitJoinError::PoleEdgesCoincide { .. })),
            "Interval, a whole turn's gap: {got:?}"
        );
        // In band: the wedge's displacement at the lever sits between
        // the zero and escalation bands.
        let theta = 0.5 * (band.zero() + band.escalate());
        let b = meridian::<f64>(0.5 + theta);
        let got = pole_branch(joint(-theta, false), (&a, 0.0, PI), (&b, 0.0, PI), band);
        assert!(
            matches!(got, Err(SplitJoinError::Escalated { .. })),
            "θ = {theta:e} at a unit lever is in band: {got:?}"
        );
    }

    /// **An edge whose far end is back at the pole is levered at its
    /// furthest point, not its end**: a closed meridian and a closed
    /// small circle through the pole (and one a hair short of closing)
    /// reach their antipodes, so a quarter-turn wedge against a meridian
    /// decides its branch, the entry a turn on.
    #[test]
    fn an_edge_closing_back_at_the_pole_is_levered_at_its_antipode() {
        let band = band();
        let beta: f64 = 0.3;
        let curved = small_circle(beta);
        let m = meridian::<f64>(0.0);
        for far in [TAU, TAU - 1e-9] {
            let lever = pole_lever((&curved, 0.0, far), sphere(), 1.0).unwrap();
            assert!(
                (lever - beta.tan()).abs() <= 1e-12,
                "far = {far}: the closed small circle reaches sin 2β past its cap tan β: \
                 {lever:e}"
            );
            let got = pole_branch(
                joint(PI / 2.0, false),
                (&curved, 0.0, far),
                (&m, 0.0, PI),
                band,
            );
            assert_eq!(got.ok(), Some(1), "far = {far}: a quarter-turn wedge");
        }
        let closed = meridian::<f64>(0.0);
        assert_eq!(
            pole_lever((&closed, 0.0, TAU), sphere(), 1.0),
            Some(1.0),
            "a closed meridian crosses the equator"
        );
        let got = pole_branch(
            joint(PI / 2.0, false),
            (&closed, 0.0, TAU),
            (&meridian::<f64>(-PI / 2.0), 0.0, PI),
            band,
        );
        assert_eq!(
            got.ok(),
            Some(1),
            "a quarter-turn wedge against a closed meridian"
        );
    }

    /// **In band escalates at every scalar**: a wedge whose displacement
    /// at a unit lever sits between the zero and escalation bands.
    #[test]
    fn a_wedge_in_band_escalates_at_interval_and_at_a_dual() {
        fn escalates<T: geom_core::Decide>(lane: &str) {
            let band = band();
            let theta = 0.5 * (band.zero() + band.escalate());
            let (a, b) = (meridian::<T>(0.5), meridian::<T>(0.5 + theta));
            let got = pole_branch(
                joint(T::from_f64(-theta), false),
                (&a, T::zero(), T::pi()),
                (&b, T::zero(), T::pi()),
                band,
            );
            assert!(
                matches!(got, Err(SplitJoinError::Escalated { .. })),
                "[{lane}] θ = {theta:e} at a unit lever is in band: {got:?}"
            );
        }
        escalates::<Interval>("Interval");
        escalates::<geom_core::Dual64>("Dual64");
    }

    use core::f64::consts::{PI, TAU};
}

/// The walk's hops past the caller's key are links: a torn one panics
/// naming it (`OPERATORS_KEEP_LINKS`: the walk runs mid-operation too).
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod torn_hop_rows {
    use geom_core::{Band, Tol};

    use super::SplitJoinError;
    use crate::body::Body;
    use crate::body::WALKS_CLOSE;
    use crate::entity::{EntityId, FaceKey, GeomRef, HalfEdgeKey, LoopBoundary};
    use crate::live::OPERATORS_KEEP_LINKS;
    use crate::review_d18::{ROW_FOUR, assert_torn_op_panics};

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    fn cyl_sheet() -> (Body<f64>, FaceKey) {
        let mut body = Body::<f64>::new();
        let face = crate::test_support_fixtures::cyl_wall_sheet(
            &mut body,
            crate::test_support_fixtures::CylFrame::canonical(1.0),
            (0.2, 1.4),
            (0.0, 1.0),
            Tol::witness(),
        );
        (body, face)
    }

    fn first_member(body: &Body<f64>, face: FaceKey) -> HalfEdgeKey {
        let outer = body.get_face(face).unwrap().outer;
        let LoopBoundary::Cycle { first } = body.get_loop(outer).unwrap().boundary else {
            panic!("the fixture's outer loop is a cycle");
        };
        first
    }

    /// Drops the curve of `he`'s edge, and names the link that dangles.
    fn drop_curve(body: &mut Body<f64>, he: HalfEdgeKey) -> String {
        let edge = body.get_half_edge(he).unwrap().edge;
        let curve = body.get_edge(edge).unwrap().curve;
        body.curves.remove(curve);
        format!(
            "{}'s curve names {}",
            EntityId::Edge(edge),
            GeomRef::Curve(curve)
        )
    }

    /// `face_azimuth_images` (`FaceView::outer_cycle`, `run_azimuth_images`): a
    /// torn outer loop, loop walk, member edge and curve panic, where the
    /// loop and walk answered `Corrupt` and the edge and curve were
    /// stepped over; a stale face keeps `Corrupt`.
    #[test]
    fn the_azimuth_walk_panics_on_a_torn_loop_and_a_torn_curve() {
        let (body, face) = cyl_sheet();
        let surface = body.face_surface(face).unwrap().clone();
        let images = |b: &Body<f64>| {
            super::face_azimuth_images(b, &surface, face, band()).map(|i| i.map(|i| i.len()))
        };
        assert_eq!(
            images(&body).unwrap(),
            Some(4),
            "every edge of the sound sheet has an image"
        );

        let mut stale = body.clone();
        let data = stale.get_face(face).unwrap().clone();
        let gone = stale.faces.insert(data);
        stale.faces.remove(gone);
        assert!(
            matches!(
                super::face_azimuth_images(&stale, &surface, gone, band()),
                Err(SplitJoinError::Corrupt {
                    entity: EntityId::Face(f)
                }) if f == gone
            ),
            "a stale face is the caller's key, refused typed"
        );

        let mut torn = body.clone();
        let outer = torn.get_face(face).unwrap().outer;
        torn.loops.remove(outer);
        let named = format!(
            "{}'s outer names {}",
            EntityId::Face(face),
            EntityId::Loop(outer)
        );
        assert_torn_op_panics(
            "face_azimuth_images (loop)",
            &mut torn,
            &[&named, ROW_FOUR, OPERATORS_KEEP_LINKS],
            |b| images(b),
        );

        let mut torn = body.clone();
        let he = first_member(&torn, face);
        let named = drop_curve(&mut torn, he);
        assert_torn_op_panics(
            "face_azimuth_images (curve)",
            &mut torn,
            &[&named, ROW_FOUR, OPERATORS_KEEP_LINKS],
            |b| images(b),
        );

        // A member's edge: the walk steps by `next` and closes past it.
        let mut torn = body.clone();
        let he = first_member(&torn, face);
        let edge = torn.get_half_edge(he).unwrap().edge;
        torn.edges.remove(edge);
        let named = format!(
            "{}'s edge names {}",
            EntityId::HalfEdge(he),
            EntityId::Edge(edge)
        );
        assert_torn_op_panics(
            "face_azimuth_images (edge)",
            &mut torn,
            &[&named, ROW_FOUR, OPERATORS_KEEP_LINKS],
            |b| images(b),
        );

        // A member past the first: the outer loop's walk breaks.
        let mut torn = body.clone();
        let first = first_member(&torn, face);
        let second = torn.get_half_edge(first).unwrap().next;
        torn.half_edges.remove(second);
        let named = format!("the loop walk from {first:?} breaks at {first:?}");
        assert_torn_op_panics(
            "face_azimuth_images (walk)",
            &mut torn,
            &[&named, WALKS_CLOSE, OPERATORS_KEEP_LINKS],
            |b| images(b),
        );
    }
}

#[cfg(test)]
mod run_image_incidence {
    #![allow(clippy::unwrap_used)]
    use super::SplitJoinError;
    use geom_brep::recourse::{INCIDENCE_AT_POSE, INCIDENCE_UNSHOWN};
    use geom_brep::{OffMargin, PcurveCertifyError, PcurveCheck};
    use geom_core::{Band, Indeterminate, KERNEL_DEFECT_ENDING, MarginDiag};

    /// **A boundary edge's incidence refusal ends by the one map**,
    /// through the walk's own mapping (`run_image_refusal`): decided off,
    /// the kernel's defect; at a pose, the pose lever; undecided, its
    /// lever; a poisoned margin, the defect. Never an invariant break or
    /// a coincidence lever.
    #[test]
    fn a_run_edges_incidence_refusal_ends_by_its_read() {
        let off = |margin| PcurveCertifyError::CarrierOffChart {
            chart: geom::SurfaceKind::Cone,
            carrier: geom::CurveKind::Circle,
            why: "WHY",
            margin,
        };
        let undecided = |margin| PcurveCertifyError::Escalated {
            check: PcurveCheck::ChartIncidence,
            sample: 0,
            cause: Indeterminate {
                margin,
                band: Band::new(1e-9, 1e-8).unwrap(),
                predicate: Some("pcurve_cone_chart_incident"),
                terminal_sliver: false,
            },
        };
        let m = MarginDiag::value(4e-10);
        for (refusal, ending) in [
            (off(None), KERNEL_DEFECT_ENDING),
            (off(Some(OffMargin::InBand(m))), INCIDENCE_AT_POSE.lever),
            (undecided(MarginDiag::value(5e-9)), INCIDENCE_UNSHOWN.lever),
            (undecided(MarginDiag::INVALID), KERNEL_DEFECT_ENDING),
        ] {
            let mapped = super::run_image_refusal(
                crate::FaceKey::default(),
                geom::SurfaceKind::Cone,
                refusal.clone(),
            );
            assert!(
                matches!(mapped, SplitJoinError::RunImageIncidence { .. }),
                "{refusal:?}: {mapped:?}"
            );
            let text = mapped.render(geom_brep::recourse::Reading::Build);
            assert!(text.contains(ending), "{refusal:?}: {text}");
            assert_eq!(
                text.matches("Recourse").count() + text.matches("defect").count(),
                1,
                "{text}"
            );
            assert!(
                !text.contains("invariant") && !text.contains("coincide"),
                "{text}"
            );
        }
    }
}
