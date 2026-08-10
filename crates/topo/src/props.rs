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
                // A described NURBS face ALWAYS takes the quadrature
                // lane (M6-3): its flux has no closed form regardless
                // of what bounds it, and the patch engine reads the
                // stored iso pcurves rather than the carriers.
                let quad_out = if is_trimmed || matches!(surface, Surface::Nurbs(_)) {
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
pub(crate) fn loop_edges<T: Decide>(
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
pub trait PropsQuadLane: Decide + geom_brep::PcurveFittedLane + geom_brep::EdgeNurbsLane {
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
                what: "B-spline trim carrier on an ANALYTIC chart's quadrature lane — \
                       the cut-loft class (a loft wall cut by a plane/cylinder), which \
                       needs the edge×NURBS-face boolean layer (M5 PR 9c item 5, \
                       banked); described-NURBS faces with iso-line pcurves route to \
                       the patch engine instead",
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
    /// (module docs of `geom_brep::props::quad`): the cylinder chart's
    /// closed-form lane plus the described-NURBS patch lane (M6-3);
    /// cone/sphere/torus charts MINT stored pcurves since M6-3 (walk
    /// row 4) but their chart-normal flux algebra is not written —
    /// they refuse typed naming that true blocker.
    pub(super) fn cut_face<T: Decide + Bounds>(
        body: &Body<T>,
        surface: &Surface<T>,
        outer: &[LoopEdge<T>],
        hes: &[HalfEdgeKey],
        band: Band,
    ) -> Result<FaceCutBounds, PropsError> {
        // The NURBS-patch lane (M6-3): a described NURBS face routes
        // to the patch engine over its stored iso-line pcurves.
        if let Surface::Nurbs(payload) = surface {
            return nurbs_face(body, payload, outer, hes, band);
        }
        let Surface::Cylinder { origin, radius, .. } = surface else {
            return Err(PropsError::QuadratureUnsupported {
                what: "conic trim on a cone/sphere/torus chart — those charts mint stored \
                       pcurves since M6-3 (walk row 4), but this lane's chart-normal \
                       flux algebra is the cylinder chart's (M5 PR 11); the other \
                       analytic charts' closed-form flux is its own banked lane",
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
                t0: br(t0),
                t1: br(t1),
                forward: le.forward,
                trig0,
                env: br(cache.certificate().envelope),
            });
        }
        quad::cylinder_cut_face::<T>(br(*radius), o_dot_va, &edges, eps, band)
    }

    /// **The NURBS-patch flux lane** (M6-3 Leg C): certified volume
    /// flux + area of a described, NON-RATIONAL NURBS face whose
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
    fn nurbs_face<T: Decide + Bounds>(
        body: &Body<T>,
        payload: &std::sync::Arc<geom_surfaces::NurbsSurface<T>>,
        outer: &[LoopEdge<T>],
        hes: &[HalfEdgeKey],
        band: Band,
    ) -> Result<FaceCutBounds, PropsError> {
        if payload.is_placeholder() {
            return Err(PropsError::QuadratureUnsupported {
                what: "the mvfs Nurbs placeholder reached the quadrature lane — a \
                       mid-surgery body has no mass properties (tier 2 refuses it at rest)",
            });
        }
        // The rational gate fires here too (before any geometry), so
        // the recourse text reaches the +V caller verbatim.
        //
        // M8-3 half 2 retired the gate one level DOWN — `quad`'s
        // `nurbs_patch_face` now routes weights ≠ 1 into a certified
        // rational lane — so the REASON this one still stands has
        // changed, and the text below states the new reason rather
        // than the retired one. (R1's n1: the plan put the
        // quad.rs-level retirement in PR-2; it landed in PR-1 because
        // that is where the engine was written. No reachability
        // changed — THIS gate is the binding one, and it does not move
        // until the arc-rim chart map, M8-3 half 1.)
        if payload.weights().iter().any(|w| *w != 1.0) {
            return Err(PropsError::QuadratureUnsupported {
                what: "RATIONAL patch flux (weights != 1): the patch engine certifies these \
                       since M8-3, but every rational wall comes from an arc profile segment \
                       and such a wall's cap rims are arcs on a NURBS chart, which no pcurve \
                       variant can map — so no such body assembles, and the body-level lane \
                       stays BANKED until the arc-rim chart map lands; loft with a polyline \
                       profile, or wait for the rational-wall unit",
            });
        }
        let eps = Tolerance::get().eps;
        // Exact-structure read of a T scalar (point bracket required).
        let exact = |x: RingInterval| -> Result<f64, PropsError> {
            if x.lo() == x.hi() && x.lo().is_finite() {
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
            let Pcurve::IsoLine { .. } = cache.pcurve() else {
                return Err(PropsError::QuadratureUnsupported {
                    what: "a NURBS-face half-edge carries a non-iso pcurve — a trimmed \
                           NURBS region's quadrature is the cut-loft unit's (the \
                           edge×NURBS-face boolean layer mints those trims)",
                });
            };
            let (t0, t1) = cache.params();
            let a = cache.pcurve().eval(t0);
            let b = cache.pcurve().eval(t1);
            let (ax, ay) = (exact(br(a.x))?, exact(br(a.y))?);
            let (bx, by) = (exact(br(b.x))?, exact(br(b.y))?);
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
            let len = match &le.carrier {
                geom_curves::Curve3::Line { dir, .. } => (br(dir.norm()) * br(t1 - t0)).mag(),
                geom_curves::Curve3::Nurbs(c) => {
                    let mut l = RingInterval::zero();
                    for w in c.control().windows(2) {
                        l = l + br(w[0].distance(w[1]));
                    }
                    l.mag()
                }
                // Circle/Ellipse boundaries only arise on rational
                // walls, refused above; kept total via the hull bound
                // 2πr-ish is unavailable without the kind — refuse.
                _ => {
                    return Err(PropsError::QuadratureUnsupported {
                        what: "a NURBS-face boundary carrier outside the polyline-loft \
                               inventory (conic boundary ⇒ rational wall, refused above)",
                    });
                }
            };
            perimeter += len;
            boundary_defect += len * br(cache.certificate().envelope).mag();
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
            .map(|p| [br(p.x), br(p.y), br(p.z)])
            .collect();
        let out = quad::nurbs_patch_face::<T>(
            payload.knots_u(),
            payload.knots_v(),
            &control,
            payload.weights(),
            (u0, u1, v0, v1),
            perimeter,
            boundary_defect,
            eps,
            band,
        )?;
        // The winding sign carries the S10 orientation into the flux;
        // the area is unsigned.
        let flux = if winding < 0.0 { -out.flux } else { out.flux };
        Ok(FaceCutBounds {
            flux,
            area: out.area,
        })
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
