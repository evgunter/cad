//! Body-level mass properties over the **exact** B-rep (M2 PR 7):
//! volume and surface area assembled from `geom-brep`'s closed-form
//! per-face contributions ([`geom_brep::props`] — divergence-theorem
//! flux split against per-surface anchors; Mäntylä §13.3 generalized
//! off the polyhedral case).
//!
//! This is the exact-geometry counterpart of the mesh oracle
//! (`mesh::validate::signed_volume`): no tessellation, no sampling, no
//! quadrature — every face's contribution is a closed form over the
//! stored analytic data, scalar-generic over [`Decide`] so the same
//! formulas instantiate at `f64` (a value) and at the certified
//! interval scalar (an enclosure that **is** the certified bound, Q1).
//! The coned-polyhedron fan over boundary vertices (Mäntylä's
//! `svolume`) is deliberately absent: on curved faces its magnitude is
//! wrong (it measures the cone over the boundary, not the face — the
//! M2 PR 5 review's finding); the divergence formulation here is exact
//! for the whole M2 face inventory.
//!
//! Layering: `geom-brep` owns the key-free per-face math; this module
//! walks the body's arenas (face → loops → half-edge cycles), flattens
//! each loop into [`geom_brep::LoopEdge`]s, and sums. The tier-3
//! validator consumes [`mass_properties_with`] for the +V orientation
//! invariant without any new inter-crate dependency.

use core::fmt;

use geom_brep::props::quad::FaceCutBounds;
use geom_brep::props::{FaceContribution, LoopEdge, PropsError, curved_face, planar_face};
use geom_core::{Band, BandError, Decide, Real};
use geom_surfaces::Surface;

use crate::body::Body;
use crate::entity::{FaceKey, HalfEdgeKey, LoopBoundary, LoopKey, VertexKey};

/// Exact-B-rep integral properties of a body.
///
/// Closed-form faces contribute exact values; curved-CUT faces (M5
/// PR 11) contribute **certified quadrature enclosures**, carried as
/// the enclosure midpoint in `volume`/`surface_area` plus the summed
/// half-widths in the `*_pad` fields — so the certified brackets are
/// `volume ± volume_pad` and `surface_area ± area_pad`. Both pads are
/// exactly `0.0` for bodies whose every face has a closed form (the
/// pre-PR-11 behaviour, bit-identical).
#[derive(Clone, Copy, Debug)]
pub struct MassProperties<T: Real> {
    /// The **signed** enclosed volume by the divergence theorem —
    /// positive for a correctly oriented (outward-normal) closed body;
    /// a definitely negative value is orientation corruption (the
    /// tier-3 +V invariant). For quadrature faces this is the
    /// certified enclosure's midpoint; the bracket is `± volume_pad`.
    pub volume: T,
    /// The total surface area (a sum of unsigned face areas); for
    /// quadrature faces the enclosure midpoint (bracket `± area_pad`).
    pub surface_area: T,
    /// Certified half-width of the volume bracket (m³) — the summed
    /// quadrature enclosure half-widths; `0.0` when every face is
    /// closed-form. The tier-3 +V backstop consumes this.
    pub volume_pad: f64,
    /// Certified half-width of the area bracket (m²).
    pub area_pad: f64,
}

/// Typed failure of [`mass_properties`] (closed enum, D4 ¶3).
#[derive(Clone, Debug, PartialEq)]
pub enum MassPropsError {
    /// The run's tolerance cannot form a band.
    Band {
        /// The band construction failure.
        error: BandError,
    },
    /// A face's closed form failed: boundary outside the M2
    /// iso-rectangle inventory, a definite consistency failure, or an
    /// escalated classification.
    Face {
        /// The offending face.
        face: FaceKey,
        /// The per-face failure.
        source: PropsError,
    },
    /// A curved face carries interior rings — no M2 construction
    /// produces one (curved patches are swept UV rectangles).
    RingOnCurvedFace {
        /// The offending face.
        face: FaceKey,
    },
    /// A loop is empty or a referenced key fails to resolve —
    /// tier-1/tier-2 scaffolding or corruption; the structural
    /// validators own the diagnosis, this is the fail-loud surface.
    Corrupt {
        /// What failed to resolve (static description).
        what: &'static str,
    },
    /// An edge is M3 null-edge scaffolding (no carrier by type — see
    /// `crate::null`): the body is mid-surgery, and mass properties are
    /// defined on at-rest bodies only (tier 2 refuses null entities).
    NullScaffoldEdge {
        /// The scaffolding edge.
        edge: crate::entity::EdgeKey,
    },
}

impl fmt::Display for MassPropsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Band { error } => write!(f, "mass properties: {error}"),
            Self::Face { face, source } => {
                write!(f, "mass properties: face {face:?}: {source}")
            }
            Self::RingOnCurvedFace { face } => {
                write!(
                    f,
                    "mass properties: curved face {face:?} carries interior rings"
                )
            }
            Self::Corrupt { what } => {
                write!(f, "mass properties: corrupt body ({what})")
            }
            Self::NullScaffoldEdge { edge } => {
                write!(
                    f,
                    "mass properties: edge {edge:?} is null-edge scaffolding \
                     (mid-surgery body; tier 2 refuses null entities at rest)"
                )
            }
        }
    }
}

impl std::error::Error for MassPropsError {}

/// Volume and surface area of `body` over the exact B-rep (module
/// docs). Faces are visited in face-arena order; the accumulation
/// order is fixed (D9).
///
/// # Errors
///
/// [`MassPropsError`] — a misconfigured band, an out-of-inventory
/// face, rings on a curved face, or unresolvable structure. Bodies
/// that pass the structural tiers and were built by M2's public
/// operations always compute.
pub fn mass_properties<T: PropsQuadLane>(
    body: &Body<T>,
) -> Result<MassProperties<T>, MassPropsError> {
    let band = Band::linear().map_err(|error| MassPropsError::Band { error })?;
    mass_properties_with(body, band)
}

/// [`mass_properties`] against a caller-built band (the tier-3
/// validator's entry — it builds its band once at operation entry).
pub(crate) fn mass_properties_with<T: PropsQuadLane>(
    body: &Body<T>,
    band: Band,
) -> Result<MassProperties<T>, MassPropsError> {
    mass_properties_impl(body, band, T::quad_cut_face)
}

/// The closed-form-only variant for the boolean engine's INTERNAL
/// backstops (`volume_backstop`, `at_infinity_side`): plain
/// `T: Decide`, no quadrature dispatch — on a conic-trimmed face the
/// closed form refuses typed exactly as it did pre-PR-11, which is
/// those callers' historical (fail-loud) posture. The at-rest
/// measurement door is [`mass_properties`], which carries the
/// certified lane.
pub(crate) fn mass_properties_closed_form<T: Decide>(
    body: &Body<T>,
    band: Band,
) -> Result<MassProperties<T>, MassPropsError> {
    mass_properties_impl(body, band, |_, _, _, _, _| Ok(None))
}

/// The shared face walk; `quad` is the per-scalar certified-quadrature
/// hook (`Ok(None)` = no lane / not attempted — the closed form then
/// answers, refusing typed on trimmed faces).
#[allow(clippy::type_complexity)]
fn mass_properties_impl<T: Decide>(
    body: &Body<T>,
    band: Band,
    quad: impl Fn(
        &Body<T>,
        &Surface<T>,
        &[LoopEdge<T>],
        &[HalfEdgeKey],
        Band,
    ) -> Result<Option<FaceCutBounds>, PropsError>,
) -> Result<MassProperties<T>, MassPropsError> {
    let mut flux = T::zero();
    let mut area = T::zero();
    let mut flux_pad = 0.0f64;
    let mut area_pad = 0.0f64;
    for (face_key, face) in body.faces.iter() {
        let Some(surface) = body.surfaces.get(face.surface) else {
            return Err(MassPropsError::Corrupt {
                what: "face surface key does not resolve",
            });
        };
        let wrap = |source| MassPropsError::Face {
            face: face_key,
            source,
        };
        let contribution: FaceContribution<T> = match *surface {
            Surface::Plane { origin, .. } => {
                let mut loops = Vec::with_capacity(1 + face.rings.len());
                for &lk in core::iter::once(&face.outer).chain(&face.rings) {
                    loops.push(loop_edges(body, lk)?.0);
                }
                planar_face(origin, &loops).map_err(wrap)?
            }
            _ => {
                if !face.rings.is_empty() {
                    return Err(MassPropsError::RingOnCurvedFace { face: face_key });
                }
                let (outer, hes) = loop_edges(body, face.outer)?;
                // Structural dispatch (C5: on the carrier KIND, never a
                // runtime fallback): a conic/NURBS trim carrier routes
                // the face to the PR 11 certified-quadrature lane; an
                // iso boundary keeps its closed form. The S10 sense bit
                // does NOT enter the quadrature lane: its Green form is
                // winding-derived end to end (the signed UV area IS
                // s_f·|Ω| through the stored loop traversal), exactly
                // the class the S10 module docs keep bit-free.
                let is_trimmed = outer.iter().any(|e| {
                    matches!(
                        e.carrier,
                        geom_curves::Curve3::Ellipse { .. } | geom_curves::Curve3::Nurbs(_)
                    )
                });
                let quad_out = if is_trimmed {
                    quad(body, surface, &outer, &hes, band).map_err(wrap)?
                } else {
                    None
                };
                match quad_out {
                    Some(bounds) => {
                        let (fc, fp) = quad_lane::mid_pad(bounds.flux);
                        let (ac, ap) = quad_lane::mid_pad(bounds.area);
                        flux_pad += fp;
                        area_pad += ap;
                        FaceContribution {
                            flux: T::from_f64(fc),
                            area: T::from_f64(ac),
                        }
                    }
                    // Either an iso boundary (the closed forms — the
                    // face's S10 sense entering at `curved_face`'s one
                    // sanctioned site, the rimless sphere band) or a
                    // scalar with NO certified lane (the dual arm of
                    // [`PropsQuadLane`]) — whose honest outcome on a
                    // trimmed face is the closed form's typed refusal.
                    None => curved_face(surface, &outer, face.sense_sign(), band).map_err(wrap)?,
                }
            }
        };
        flux = flux + contribution.flux;
        area = area + contribution.area;
    }
    Ok(MassProperties {
        volume: flux / T::from_f64(3.0),
        surface_area: area,
        volume_pad: flux_pad / 3.0,
        area_pad,
    })
}

/// Flatten one loop's half-edge cycle into key-free [`LoopEdge`]s
/// (traversal order; vertex tags are loop-local first-seen indices),
/// alongside the half-edge keys walked (the PR 11 quadrature lane
/// reads stored pcurve caches through them).
#[allow(clippy::type_complexity)]
fn loop_edges<T: Decide>(
    body: &Body<T>,
    lk: LoopKey,
) -> Result<(Vec<LoopEdge<T>>, Vec<crate::entity::HalfEdgeKey>), MassPropsError> {
    let corrupt = |what| MassPropsError::Corrupt { what };
    let Some(loop_) = body.loops.get(lk) else {
        return Err(corrupt("loop key does not resolve"));
    };
    let LoopBoundary::Cycle { first } = loop_.boundary else {
        return Err(corrupt("empty loop (construction scaffolding at rest)"));
    };
    let Some(cycle) = body.loop_cycle(first) else {
        return Err(corrupt("broken half-edge cycle"));
    };
    let mut tags: Vec<VertexKey> = Vec::new();
    let mut tag_of = |v: VertexKey| -> u32 {
        if let Some(i) = tags.iter().position(|&t| t == v) {
            i as u32
        } else {
            tags.push(v);
            (tags.len() - 1) as u32
        }
    };
    let mut edges = Vec::with_capacity(cycle.len());
    let mut hes = Vec::with_capacity(cycle.len());
    for &he_key in &cycle {
        hes.push(he_key);
        let Some(he) = body.half_edges.get(he_key) else {
            return Err(corrupt("half-edge key does not resolve"));
        };
        let Some(edge) = body.edges.get(he.edge) else {
            return Err(corrupt("edge key does not resolve"));
        };
        let Some(entry) = body.curves.get(edge.curve) else {
            return Err(corrupt("curve key does not resolve"));
        };
        let Some(curve) = entry.certified() else {
            return Err(MassPropsError::NullScaffoldEdge { edge: he.edge });
        };
        let Some(end) = body.half_edge_end(he_key) else {
            return Err(corrupt("half-edge mate does not resolve"));
        };
        let (t0, t1) = curve.params();
        edges.push(LoopEdge {
            carrier: curve.carrier().clone(),
            t0,
            t1,
            forward: he_key == edge.he_plus,
            start: tag_of(he.start),
            end: tag_of(end),
        });
    }
    Ok((edges, hes))
}

/// The certified-quadrature **lane split** (M5 PR 11; Evan's ruling at
/// this PR, superseding a runtime-`Option` bracket seam): certification
/// is the f64 / Probe / Interval lanes' business; derivative transport
/// is the dual lane's — and that split lives in the TYPES. Each
/// certified impl routes through the `T: Decide + Bounds` plumbing in
/// [`quad_lane`] (a ratified compound-`Bounds` seam — see the
/// discipline allowlist), so the quadrature machinery only ever
/// instantiates for bracket-carrying scalars; the `Dual` impl contains
/// **no quadrature code at all** — it answers "no lane", the
/// closed-form pass's typed refusal stands, and tier 3's check 7
/// reports `VolumeUncomputable` there. The dual lane validates what is
/// its business; volume certification is proven by the certified
/// lanes.
///
/// # Why the pcurve lane rides along (M6-2)
///
/// The supertrait is [`geom_brep::PcurveFittedLane`], not bare
/// [`Decide`], and the bundling is deliberate rather than incidental:
/// it is **the same split, over the same four scalars, for the same
/// reason**. A fitted (rung-3) pcurve's between-samples obligation is a
/// C9-ring hull bound reached through a scalar's bracket, exactly as
/// the quadrature's flux enclosures are; `f64`, the telemetry probe and
/// the interval scalar can derive both, and the dual scalar — which has
/// no bracket to offer, only a derivative — can derive neither and says
/// so in a refusing impl on each side.
///
/// So `T: PropsQuadLane` reads, at every consumer that already writes
/// it, as **"this scalar can certify a body at rest"**, which is what
/// every one of them meant. The alternative was to thread a second,
/// pointwise-identical lane bound through every tier-3 signature and
/// every generic body helper in the workspace, which would have bought
/// no additional honesty — the refusing side is the same scalar.
pub trait PropsQuadLane: Decide + geom_brep::PcurveFittedLane {
    /// The certified flux/area enclosures of a conic-trimmed cylinder
    /// face, or `None` when this scalar has no certified lane.
    ///
    /// # Errors
    ///
    /// [`PropsError`] from the quadrature lane (budget, unsupported
    /// inventory, escalations) — never from the "no lane" arm.
    fn quad_cut_face(
        body: &Body<Self>,
        surface: &Surface<Self>,
        outer: &[LoopEdge<Self>],
        hes: &[HalfEdgeKey],
        band: Band,
    ) -> Result<Option<FaceCutBounds>, PropsError>;
}

impl PropsQuadLane for f64 {
    fn quad_cut_face(
        body: &Body<Self>,
        surface: &Surface<Self>,
        outer: &[LoopEdge<Self>],
        hes: &[HalfEdgeKey],
        band: Band,
    ) -> Result<Option<FaceCutBounds>, PropsError> {
        quad_lane::cut_face(body, surface, outer, hes, band).map(Some)
    }
}

impl PropsQuadLane for geom_core::Probe {
    fn quad_cut_face(
        body: &Body<Self>,
        surface: &Surface<Self>,
        outer: &[LoopEdge<Self>],
        hes: &[HalfEdgeKey],
        band: Band,
    ) -> Result<Option<FaceCutBounds>, PropsError> {
        quad_lane::cut_face(body, surface, outer, hes, band).map(Some)
    }
}

#[cfg(feature = "interval")]
impl PropsQuadLane for geom_core::interval::Interval {
    fn quad_cut_face(
        body: &Body<Self>,
        surface: &Surface<Self>,
        outer: &[LoopEdge<Self>],
        hes: &[HalfEdgeKey],
        band: Band,
    ) -> Result<Option<FaceCutBounds>, PropsError> {
        quad_lane::cut_face(body, surface, outer, hes, band).map(Some)
    }
}

/// The dual lane: STATICALLY no quadrature — this impl instantiates
/// none of the certified machinery (trait docs).
impl<T> PropsQuadLane for geom_core::Dual<T>
where
    geom_core::Dual<T>: Decide,
{
    fn quad_cut_face(
        _body: &Body<Self>,
        _surface: &Surface<Self>,
        _outer: &[LoopEdge<Self>],
        _hes: &[HalfEdgeKey],
        _band: Band,
    ) -> Result<Option<FaceCutBounds>, PropsError> {
        Ok(None)
    }
}

/// The PR 11 certified-quadrature lane's body-side plumbing: stored
/// pcurve caches → ring-bracketed [`quad::TrimEdgeQ`]s (C4's first hot
/// consumer). Key-free math stays in `geom_brep::props::quad`; this
/// module owns everything that needs half-edges and vertex points.
mod quad_lane {
    use geom_brep::Pcurve;
    use geom_brep::props::quad::{self, FaceCutBounds, HarmChan, TrimEdgeQ};
    use geom_brep::props::{LoopEdge, PropsError, loop_vector_area};
    use geom_core::ring_interval::RingInterval;
    // The compound `Decide + Bounds` bound below is a RATIFIED seam
    // (M5 PR 11, Evan's lane-split ruling; discipline allowlist row):
    // this module is the certified lanes' plumbing and never
    // instantiates for duals — the split is enforced by
    // [`super::PropsQuadLane`]'s explicit impls.
    use geom_core::{Band, Bounds, Decide, Point3, Tolerance};
    use geom_curves::Curve3;
    use geom_surfaces::Surface;

    use crate::body::Body;
    use crate::entity::HalfEdgeKey;

    /// Enclosure midpoint and half-width (the [`super::MassProperties`]
    /// pad decomposition).
    pub(super) fn mid_pad(x: RingInterval) -> (f64, f64) {
        ((x.lo() + x.hi()) * 0.5, (x.hi() - x.lo()) * 0.5)
    }

    /// Bracket a scalar through its [`Bounds`] accessors (infallible:
    /// only bracket-carrying scalars reach this module — the static
    /// lane split above).
    fn br<T: Bounds>(x: T) -> RingInterval {
        RingInterval::from_bounds(x.lo(), x.hi())
    }

    /// `(cos t₀, sin t₀)` enclosure at the carrier-interval start,
    /// recovered algebraically from the carrier frame and the interval
    /// start's VERTEX point (within the run's ε of the carrier, D4 ¶2
    /// — the ε rides into the bracket as an explicit pad).
    fn trig_at_start<T: Decide + Bounds>(
        carrier: &Curve3<T>,
        p: Point3<T>,
        eps: f64,
    ) -> Result<(RingInterval, RingInterval), PropsError> {
        let full = RingInterval::from_bounds(-1.0, 1.0);
        let clamp = |x: RingInterval, pad: f64| {
            RingInterval::from_bounds((x.lo() - pad).max(-1.0), (x.hi() + pad).min(1.0))
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
                let c = br(w.dot(*u_ref)) / br(*radius);
                let s = br(w.dot(v_ref)) / br(*radius);
                let pad = (RingInterval::point(eps) / br(*radius)).mag();
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
                let c = br(w.dot(*u_ref)) / br(*major);
                let s = br(w.dot(v_ref)) / br(*minor);
                let pad_c = (RingInterval::point(eps) / br(*major)).mag();
                let pad_s = (RingInterval::point(eps) / br(*minor)).mag();
                Ok((clamp(c, pad_c), clamp(s, pad_s)))
            }
            Curve3::Nurbs(_) => Err(PropsError::QuadratureUnsupported {
                what: "B-spline trim carrier on the quadrature lane — its stored pcurve \
                       variant arrives with the loft assembly unit",
            }),
        }
    }

    /// One channel of a stored harmonic pcurve, bracketed.
    fn chan<T: Decide + Bounds>(c0: T, ca: T, cb: T, cl: T) -> Result<HarmChan, PropsError> {
        Ok(HarmChan {
            c0: br(c0),
            ca: br(ca),
            cb: br(cb),
            cl: br(cl),
        })
    }

    /// The certified flux/area enclosures of one curved-cut face
    /// (module docs of `geom_brep::props::quad`): cylinder charts only
    /// — the one chart whose pcurves mint today; other charts refuse
    /// typed naming that frontier.
    pub(super) fn cut_face<T: Decide + Bounds>(
        body: &Body<T>,
        surface: &Surface<T>,
        outer: &[LoopEdge<T>],
        hes: &[HalfEdgeKey],
        band: Band,
    ) -> Result<FaceCutBounds, PropsError> {
        let Surface::Cylinder { origin, radius, .. } = surface else {
            return Err(PropsError::QuadratureUnsupported {
                what: "conic trim on a non-cylinder chart — cone/sphere/torus pcurves do \
                       not mint yet (they arrive with their consumers); the cylinder lane \
                       is M5 PR 11's",
            });
        };
        let eps = Tolerance::get().eps;
        let va = loop_vector_area(outer, *origin)?;
        let o_dot_va = br((*origin - Point3::origin()).dot(va));
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
            // (rung-3) image has no such form, and inventing one would
            // be a quadrature over a curve nobody bounded. Typed
            // refusal instead — the NURBS-patch flux door is the
            // loft/sweep assembly unit's, and this is where it lands.
            let Pcurve::Harmonic { p0, pa, pb, pl } = *cache.pcurve() else {
                return Err(PropsError::QuadratureUnsupported {
                    what: "curved-cut face half-edge carries a FITTED (rung-3) pcurve —                            the certified quadrature reads a closed-form chart image's                            four channels, and a spline image has none; the NURBS-patch                            flux door is the loft/sweep assembly unit's",
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
                t0: br(t0),
                t1: br(t1),
                forward: le.forward,
                trig0,
                env: br(cache.certificate().envelope),
            });
        }
        quad::cylinder_cut_face::<T>(br(*radius), o_dot_va, &edges, eps, band)
    }

    /// The vertex POINT at a half-edge's carrier-interval start (its
    /// edge's `he_plus` start vertex).
    fn start_point<T: Decide + Bounds>(
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
}
