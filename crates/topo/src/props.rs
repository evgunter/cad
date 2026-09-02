//! Mass properties over the **exact** B-rep (M2 PR 7): volume and
//! surface area assembled from `geom-brep`'s closed-form per-face
//! contributions ([`geom_brep::props`] — divergence-theorem flux split
//! against per-surface anchors; Mäntylä §13.3 generalized off the
//! polyhedral case) — at two granularities over ONE per-face walk:
//! whole-body ([`mass_properties`]) and per-shell
//! ([`classify_shells`], which also decides each shell's outer/void
//! role from its volume's sign).
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

use geom::Surface;
use geom_brep::props::quad::FaceCutBounds;
use geom_brep::props::{FaceContribution, LoopEdge, PropsError, curved_face, planar_face};
use geom_core::{Band, BandError, Decide, Indeterminate, Margin, Real, Sign, Tol};

use crate::body::Body;
use crate::boolean::ContactRecords;
use crate::entity::{FaceKey, HalfEdgeKey, LoopBoundary, LoopKey, ShellKey, SolidKey, VertexKey};
use crate::validate::ValidationError;

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
/// face, rings on a curved face, or unresolvable structure.
///
/// **Not every valid body computes**, and the sentence that used to
/// stand here (*"bodies that pass the structural tiers and were built
/// by M2's public operations always compute"*) is falsified by #649's
/// own fixture: `merge_coplanar_faces` is a public operation that
/// moves no geometry and conserves χ, and running it on a body whose
/// cylindrical walls are authored as rectangular sub-faces produces a
/// structurally valid body carrying a plus-shaped iso domain, which
/// the closed forms refuse (`props_rim_level`, S58). A refusal here is
/// D2-addendum row 2 for that arm — valid input, lane not built — not
/// a claim about the body's integrity. See
/// [`ValidationError::VolumeUncomputable`](crate::ValidationError)'s
/// per-source breakdown.
pub fn mass_properties<T: PropsQuadLane>(
    body: &Body<T>,
    tol: Tol,
) -> Result<MassProperties<T>, MassPropsError> {
    let band = Band::linear(tol).map_err(|error| MassPropsError::Band { error })?;
    mass_properties_with(body, band, tol)
}

/// [`mass_properties`] against a caller-built band (the tier-3
/// validator's entry — it builds its band once at operation entry).
pub(crate) fn mass_properties_with<T: PropsQuadLane>(
    body: &Body<T>,
    band: Band,
    tol: Tol,
) -> Result<MassProperties<T>, MassPropsError> {
    mass_properties_impl(body, band, T::quad_cut_face, tol)
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
    tol: Tol,
) -> Result<MassProperties<T>, MassPropsError> {
    mass_properties_impl(body, band, |_, _, _, _, _, _| Ok(None), tol)
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
        Tol,
    ) -> Result<Option<FaceCutBounds>, PropsError>,
    tol: Tol,
) -> Result<MassProperties<T>, MassPropsError> {
    let mut flux = T::zero();
    let mut area = T::zero();
    let mut flux_pad = 0.0f64;
    let mut area_pad = 0.0f64;
    for (face_key, _) in body.faces.iter() {
        let contribution = face_flux(body, face_key, band, &quad, tol)?;
        flux = flux + contribution.flux;
        area = area + contribution.area;
        flux_pad += contribution.flux_pad;
        area_pad += contribution.area_pad;
    }
    Ok(MassProperties {
        volume: flux / T::from_f64(3.0),
        surface_area: area,
        volume_pad: flux_pad / 3.0,
        area_pad,
    })
}

/// One face's divergence-theorem contribution, with the certified
/// quadrature half-widths it carries (`0.0` for closed-form faces).
/// The unit shared by the whole-body walk ([`mass_properties_impl`])
/// and the per-shell walk ([`classify_shells`]) — one flux
/// implementation, restricted by choosing which faces to visit, never
/// re-derived.
struct FaceFlux<T> {
    flux: T,
    area: T,
    flux_pad: f64,
    area_pad: f64,
}

/// The per-face body of the flux walk (module docs): resolve the
/// surface, flatten the loops, dispatch closed form vs certified
/// quadrature. Every refusal is a typed [`MassPropsError`].
#[allow(clippy::type_complexity)]
fn face_flux<T: Decide>(
    body: &Body<T>,
    face_key: FaceKey,
    band: Band,
    quad: &impl Fn(
        &Body<T>,
        &Surface<T>,
        &[LoopEdge<T>],
        &[HalfEdgeKey],
        Band,
        Tol,
    ) -> Result<Option<FaceCutBounds>, PropsError>,
    tol: Tol,
) -> Result<FaceFlux<T>, MassPropsError> {
    let Some(face) = body.faces.get(face_key) else {
        return Err(MassPropsError::Corrupt {
            what: "face key does not resolve",
        });
    };
    let Some(surface) = body.surfaces.get(face.surface) else {
        return Err(MassPropsError::Corrupt {
            what: "face surface key does not resolve",
        });
    };
    let wrap = |source| MassPropsError::Face {
        face: face_key,
        source,
    };
    let mut flux_pad = 0.0f64;
    let mut area_pad = 0.0f64;
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
                    geom::Curve3::Ellipse { .. } | geom::Curve3::Nurbs(_)
                )
            });
            // A described NURBS face ALWAYS takes the quadrature
            // lane (M6-3): its flux has no closed form regardless
            // of what bounds it, and the patch engine reads the
            // stored iso pcurves rather than the carriers.
            // A described SPLINE face always takes the quadrature lane
            // (M6-3): its flux has no closed form regardless of what
            // bounds it. An approximating face is one — the flux of
            // its fit, which is the geometry the face actually carries.
            let quad_out = if is_trimmed || surface.spline_chart().is_some() {
                quad(body, surface, &outer, &hes, band, tol).map_err(wrap)?
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
    Ok(FaceFlux {
        flux: contribution.flux,
        area: contribution.area,
        flux_pad,
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

/// The derived outer/void designation of one shell. The shell list
/// stores no such designation ([`crate::entity::Solid`]'s documented
/// invariant): the role IS the sign of the shell's signed volume —
/// a shell's loops wind about the outward normal, so a boundary that
/// bounds material from outside integrates positive and a cavity wall
/// integrates negative.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShellRole {
    /// Definitely-positive signed volume: the shell is the outer
    /// boundary of one connected component of material.
    Outer,
    /// Definitely-negative signed volume: the shell bounds an internal
    /// cavity.
    Void,
}

/// One shell's flux-derived properties and its decided role.
///
/// `volume` is the shell's SIGNED enclosed volume (divergence theorem
/// over exactly this shell's faces — the same per-face closed
/// forms/quadrature the whole-body [`mass_properties`] sums, so the
/// per-shell volumes of a body sum to its body volume and the pads to
/// its pad). For quadrature faces the value is the certified
/// enclosure's midpoint and the bracket is `± volume_pad`
/// ([`MassProperties`]' convention).
#[derive(Clone, Copy, Debug)]
pub struct ShellClassification<T: Real> {
    /// The classified shell.
    pub shell: ShellKey,
    /// The solid owning it ([`crate::entity::Shell::solid`]).
    pub solid: SolidKey,
    /// The shell's signed enclosed volume (midpoint; bracket
    /// `± volume_pad`).
    pub volume: T,
    /// Certified half-width of the volume bracket (m³); `0.0` when
    /// every face of the shell is closed-form.
    pub volume_pad: f64,
    /// The shell's surface area (midpoint; bracket `± area_pad`).
    pub surface_area: T,
    /// Certified half-width of the area bracket (m²).
    pub area_pad: f64,
    /// The decided role.
    pub role: ShellRole,
}

/// Typed refusal of [`classify_shells`] (closed enum, D4 ¶3). Never a
/// silent skip: a shell this door cannot classify refuses with the
/// shell named, exactly as tier 3's check 7 refuses a body whose flux
/// the props inventory cannot compute.
#[derive(Clone, Debug, PartialEq)]
pub enum ShellClassifyError {
    /// The run's tolerance cannot form a band.
    Band {
        /// The band construction failure.
        error: BandError,
    },
    /// A face of this shell refused in the flux inventory — the
    /// [`MassPropsError`] posture inherited unaltered (rational walls,
    /// out-of-inventory boundaries, corrupt structure).
    Props {
        /// The shell whose face refused.
        shell: ShellKey,
        /// The per-face/per-body failure.
        source: MassPropsError,
    },
    /// The sign read escalated: the shell's `V/A` margin sits inside
    /// the ambiguity band. F6: an in-band orientation is never
    /// guessed to a side.
    Escalated {
        /// The unclassifiable shell.
        shell: ShellKey,
        /// The named escalation from the funnel.
        source: Indeterminate,
    },
    /// The signed volume is definitely zero, or its certified bracket
    /// definitely straddles zero — there is no side to classify to.
    ZeroVolume {
        /// The unclassifiable shell.
        shell: ShellKey,
    },
}

impl fmt::Display for ShellClassifyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Band { error } => write!(f, "shell classification: {error}"),
            Self::Props { shell, source } => {
                write!(f, "shell classification: shell {shell:?}: {source}")
            }
            Self::Escalated { shell, source } => {
                write!(f, "shell classification: shell {shell:?}: {source}")
            }
            Self::ZeroVolume { shell } => write!(
                f,
                "shell classification: shell {shell:?}'s signed volume is \
                 definitely zero (or its certified bracket straddles zero) — \
                 no outer/void side exists to classify to"
            ),
        }
    }
}

impl std::error::Error for ShellClassifyError {}

/// Per-shell signed volume and outer/void role for every shell of
/// `body`, in shell-arena slot order (deterministic per D9).
///
/// The flux machinery is tier-3 check 7's, SHARED (engineering
/// convention 2): each shell sums the same per-face contributions the
/// whole-body [`mass_properties`] sums, restricted to that shell's
/// face list — never a second flux implementation. The sign is a
/// decided predicate through the crate funnel (`chk_shell_volume_sign`)
/// with check 7's margin convention: the comparand is `V/A`, the mean
/// boundary displacement the volume corresponds to — a length. For
/// quadrature faces the read is bracket-honest: `Outer` requires the
/// bracket's LOW end definitely positive, `Void` its HIGH end
/// definitely negative; anything else refuses typed
/// ([`ShellClassifyError::Escalated`] / [`ShellClassifyError::ZeroVolume`]),
/// never a guess.
///
/// # Errors
///
/// [`ShellClassifyError`] — the first shell that cannot be classified
/// refuses the call: an inherited flux refusal, an in-band sign, or a
/// definite zero. (A component count over a partial classification
/// would be a guess; the caller gets the refusal instead.)
pub fn classify_shells<T: PropsQuadLane>(
    body: &Body<T>,
    tol: Tol,
) -> Result<Vec<ShellClassification<T>>, ShellClassifyError> {
    let band = Band::linear(tol).map_err(|error| ShellClassifyError::Band { error })?;
    let mut out = Vec::new();
    for (shell_key, shell) in body.shells.iter() {
        let mut flux = T::zero();
        let mut area = T::zero();
        let mut flux_pad = 0.0f64;
        let mut area_pad = 0.0f64;
        for &face_key in &shell.faces {
            let contribution =
                face_flux(body, face_key, band, &T::quad_cut_face, tol).map_err(|source| {
                    ShellClassifyError::Props {
                        shell: shell_key,
                        source,
                    }
                })?;
            flux = flux + contribution.flux;
            area = area + contribution.area;
            flux_pad += contribution.flux_pad;
            area_pad += contribution.area_pad;
        }
        let volume = flux / T::from_f64(3.0);
        let volume_pad = flux_pad / 3.0;
        // The named sign read — ONE funnel site, evaluated at a
        // bracket end. `V/A` is a length (check 7's margin
        // convention): the mean displacement of this shell's boundary
        // that the volume defect corresponds to.
        let sign_at = |end: T| {
            crate::validate::decide("chk_shell_volume_sign", Margin::over_lever(end, area), band)
        };
        let lo = sign_at(volume - T::from_f64(volume_pad));
        let role = if matches!(lo, Ok(Sign::Positive)) {
            // Even the bracket's low end is definitely positive.
            ShellRole::Outer
        } else {
            // Closed-form shells (pad = 0) reuse the one verdict; a
            // padded bracket reads its own high end.
            let hi = if volume_pad == 0.0 {
                lo
            } else {
                sign_at(volume + T::from_f64(volume_pad))
            };
            match hi {
                Ok(Sign::Negative) => ShellRole::Void,
                Err(source) => {
                    return Err(ShellClassifyError::Escalated {
                        shell: shell_key,
                        source,
                    });
                }
                Ok(Sign::Zero) => {
                    return Err(ShellClassifyError::ZeroVolume { shell: shell_key });
                }
                // High end definitely positive while the low end was
                // not: either the low end escalated (surface that
                // escalation) or the certified bracket definitely
                // straddles zero.
                Ok(Sign::Positive) => match lo {
                    Err(source) => {
                        return Err(ShellClassifyError::Escalated {
                            shell: shell_key,
                            source,
                        });
                    }
                    _ => return Err(ShellClassifyError::ZeroVolume { shell: shell_key }),
                },
            }
        };
        out.push(ShellClassification {
            shell: shell_key,
            solid: shell.solid,
            volume,
            volume_pad,
            surface_area: area,
            area_pad,
            role,
        });
    }
    Ok(out)
}

/// The certified-quadrature **lane split** (M5 PR 11; Evan's ruling at
/// this PR, superseding a runtime-`Option` bracket seam): certification
/// is the f64 / Probe / Interval lanes' business; derivative transport
/// is the dual lane's — and that split lives in the TYPES. Each
/// certified impl routes through the `T: Decide + Bounds +
/// CertifiedEnclosure` plumbing in [`quad_lane`] (a ratified
/// compound-`Bounds` seam — see the discipline allowlist), so the
/// quadrature machinery only ever instantiates for CERTIFYING scalars
/// (bracket-carrying is no longer the distinguishing property — a dual
/// carries a bracket since D1, 2026-08-19); the `Dual` impl contains
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
/// the interval scalar can derive both, and the dual scalar can derive
/// neither and says so in a refusing impl on each side. (Until the D1
/// ruling of 2026-08-19 the dual's refusal was read off its lack of a
/// bracket. It has one now — the value channel's — and the refusal
/// stands on the ruling itself: a dual may not certify, which is
/// `geom_core::CertifiedEnclosure`'s absence, not `Bounds`'.)
///
/// So `T: PropsQuadLane` reads, at every consumer that already writes
/// it, as **"this scalar can certify a body at rest"**, which is what
/// every one of them meant. The alternative was to thread a second,
/// pointwise-identical lane bound through every tier-3 signature and
/// every generic body helper in the workspace, which would have bought
/// no additional honesty — the refusing side is the same scalar.
pub trait PropsQuadLane:
    Decide
    + geom_brep::PcurveFittedLane
    + geom_brep::EdgeNurbsLane
    + crate::chart_region::ChartRegionLane
{
    /// The certified flux/area enclosures of a conic-trimmed cylinder
    /// face, or `None` when this scalar has no certified lane.
    ///
    /// **The low end of a STORED DATUM's bracket** — not a margin, and
    /// deliberately not an `Enclosure` bound on this trait.
    ///
    /// Tier 3 sometimes has to answer whether a number the body carries
    /// is a number at all: a torus tube radius that is zero, negative
    /// or poison does not describe a small torus, it fails to describe
    /// one, and there is no band to meter that against (the chamfer's
    /// `NonpositiveSize` precedent — a fact about the DATUM takes no
    /// `k_stats` name). This is the one accessor that answers it.
    ///
    /// **Why a method and not `PropsQuadLane: Enclosure`.**
    /// `PropsQuadLane` is exactly "this scalar can certify a body at
    /// rest", and a supertrait `Enclosure` would hand every certifying
    /// signature silent bracket extraction through the blanket
    /// `impl<T: Bounds> Enclosure for T`. `scripts/gates/`
    /// `bounds-allowlist.sh` greps `Enclosure` exactly as `Bounds`
    /// (DUAL-DESIGN DL4), but this file is on its allowlist, so the
    /// gate would not refuse the supertrait here — the named accessor
    /// is what keeps the bracket read explicit and single-doored. Its
    /// four implementations live in this file, the seam already
    /// ratified for the compound `Bounds` bound. The distinct name
    /// also keeps `lo`/`hi` unshadowed at every concrete call site.
    fn datum_lo(self) -> f64;

    /// Re-derives an approximating surface's certificate against its
    /// own stored description and fit, classified against
    /// `tolerance` — the tier-3 never-trust posture (O5), one
    /// dimension up from `EdgeCurve::recertify`.
    ///
    /// **The tolerance is the RUN's, not the surface's.** The edge
    /// machinery re-certifies every carrier against the run's band and
    /// never against a stored bound, and the surface claim is the same
    /// shape: O3 ratifies `sup ‖S_fit − (S + d·n)‖ ≤ ε_precision`, so
    /// verifying it means measuring against the ε this validation call
    /// runs at. A surface minted at a loose tolerance validating
    /// forever afterwards would be the stored bound quietly replacing
    /// the ratified one. The stored tolerance stays what it always
    /// was: the MINT's parameter, and the fit door's own gate.
    ///
    /// `None` = this scalar has no re-derivation lane. That is a
    /// statement about the DERIVATION, never about which values can
    /// arrive: [`geom::ApproxSurface::certify`] is generic in the
    /// scalar and takes its certifier as an argument, so an
    /// `ApproxSurface<Self>` is representable at every scalar and such
    /// a face reaches this arm. `None` is not a pass — tier 3 reports
    /// it as [`ValidationError::ApproxLaneUnsupported`](crate::ValidationError),
    /// because a surface certificate is the one claim this kernel
    /// refuses to leave unchecked.
    ///
    /// # Errors
    ///
    /// The fit door's typed refusal, when the re-derivation fails.
    fn recertify_approx(
        approx: &geom::ApproxSurface<Self>,
        tolerance: f64,
        band: Band,
    ) -> Option<Result<geom::OffsetCertificate, geom_brep::OffsetFitError>>;

    /// Mints the certified approximating surface for a NURBS operand's
    /// offset — the fit door, reached through the lane so the doors
    /// above it stay scalar-generic.
    ///
    /// `None` = this scalar has no fit lane. That is not a pass: a
    /// caller that cannot mint the offset refuses, exactly as tier 3
    /// refuses a certificate it cannot re-derive. The fit itself is
    /// derived at `f64` only, so `None` is every other scalar's honest
    /// answer — the absence of a derivation, not of a representable
    /// operand.
    ///
    /// # Errors
    ///
    /// The fit door's typed refusal (the meters, a rational operand,
    /// the refinement budget, a certificate limb).
    fn approx_offset_surface(
        base: std::sync::Arc<geom::NurbsSurface<Self>>,
        d: Self,
        tolerance: f64,
        band: Band,
    ) -> Option<Result<Surface<Self>, geom_brep::OffsetFitError>>;

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
        tol: Tol,
    ) -> Result<Option<FaceCutBounds>, PropsError>;
}

impl PropsQuadLane for f64 {
    fn datum_lo(self) -> f64 {
        geom_core::Bounds::lo(self)
    }

    fn recertify_approx(
        approx: &geom::ApproxSurface<Self>,
        tolerance: f64,
        band: Band,
    ) -> Option<Result<geom::OffsetCertificate, geom_brep::OffsetFitError>> {
        Some(geom_brep::recertify_approx(approx, tolerance, band))
    }

    fn approx_offset_surface(
        base: std::sync::Arc<geom::NurbsSurface<Self>>,
        d: Self,
        tolerance: f64,
        band: Band,
    ) -> Option<Result<Surface<Self>, geom_brep::OffsetFitError>> {
        Some(geom_brep::approx_offset_surface(base, d, tolerance, band))
    }

    fn quad_cut_face(
        body: &Body<Self>,
        surface: &Surface<Self>,
        outer: &[LoopEdge<Self>],
        hes: &[HalfEdgeKey],
        band: Band,
        tol: Tol,
    ) -> Result<Option<FaceCutBounds>, PropsError> {
        quad_lane::cut_face(body, surface, outer, hes, band, tol).map(Some)
    }
}

#[cfg(feature = "probe")]
impl PropsQuadLane for geom_core::Probe {
    // The offset fit is derived at `f64` only, so this scalar has no
    // re-derivation lane. That is the whole reason, and it is about the
    // derivation: an `ApproxSurface<Self>` is representable here —
    // `ApproxSurface::certify` is scalar-generic and takes its
    // certifier as an argument — so such a face does reach this arm and
    // tier 3 reports `ApproxLaneUnsupported` rather than passing.
    fn recertify_approx(
        _approx: &geom::ApproxSurface<Self>,
        _tolerance: f64,
        _band: Band,
    ) -> Option<Result<geom::OffsetCertificate, geom_brep::OffsetFitError>> {
        None
    }

    fn approx_offset_surface(
        _base: std::sync::Arc<geom::NurbsSurface<Self>>,
        _d: Self,
        _tolerance: f64,
        _band: Band,
    ) -> Option<Result<Surface<Self>, geom_brep::OffsetFitError>> {
        None
    }

    fn datum_lo(self) -> f64 {
        geom_core::Bounds::lo(self)
    }

    fn quad_cut_face(
        body: &Body<Self>,
        surface: &Surface<Self>,
        outer: &[LoopEdge<Self>],
        hes: &[HalfEdgeKey],
        band: Band,
        tol: Tol,
    ) -> Result<Option<FaceCutBounds>, PropsError> {
        quad_lane::cut_face(body, surface, outer, hes, band, tol).map(Some)
    }
}

#[cfg(feature = "interval")]
impl PropsQuadLane for geom_core::interval::Interval {
    // The offset fit is derived at `f64` only, so this scalar has no
    // re-derivation lane. That is the whole reason, and it is about the
    // derivation: an `ApproxSurface<Self>` is representable here —
    // `ApproxSurface::certify` is scalar-generic and takes its
    // certifier as an argument — so such a face does reach this arm and
    // tier 3 reports `ApproxLaneUnsupported` rather than passing.
    fn recertify_approx(
        _approx: &geom::ApproxSurface<Self>,
        _tolerance: f64,
        _band: Band,
    ) -> Option<Result<geom::OffsetCertificate, geom_brep::OffsetFitError>> {
        None
    }

    fn approx_offset_surface(
        _base: std::sync::Arc<geom::NurbsSurface<Self>>,
        _d: Self,
        _tolerance: f64,
        _band: Band,
    ) -> Option<Result<Surface<Self>, geom_brep::OffsetFitError>> {
        None
    }

    fn datum_lo(self) -> f64 {
        geom_core::Bounds::lo(self)
    }

    fn quad_cut_face(
        body: &Body<Self>,
        surface: &Surface<Self>,
        outer: &[LoopEdge<Self>],
        hes: &[HalfEdgeKey],
        band: Band,
        tol: Tol,
    ) -> Result<Option<FaceCutBounds>, PropsError> {
        quad_lane::cut_face(body, surface, outer, hes, band, tol).map(Some)
    }
}

/// The dual lane: STATICALLY no quadrature — this impl instantiates
/// none of the certified machinery (trait docs).
impl<T> PropsQuadLane for geom_core::Dual<T>
where
    geom_core::Dual<T>: Decide + geom_core::Bounds,
{
    // The offset fit is derived at `f64` only, so this scalar has no
    // re-derivation lane. That is the whole reason, and it is about the
    // derivation: an `ApproxSurface<Self>` is representable here —
    // `ApproxSurface::certify` is scalar-generic and takes its
    // certifier as an argument — so such a face does reach this arm and
    // tier 3 reports `ApproxLaneUnsupported` rather than passing.
    fn recertify_approx(
        _approx: &geom::ApproxSurface<Self>,
        _tolerance: f64,
        _band: Band,
    ) -> Option<Result<geom::OffsetCertificate, geom_brep::OffsetFitError>> {
        None
    }

    fn approx_offset_surface(
        _base: std::sync::Arc<geom::NurbsSurface<Self>>,
        _d: Self,
        _tolerance: f64,
        _band: Band,
    ) -> Option<Result<Surface<Self>, geom_brep::OffsetFitError>> {
        None
    }

    fn datum_lo(self) -> f64 {
        geom_core::Bounds::lo(self)
    }

    fn quad_cut_face(
        _body: &Body<Self>,
        _surface: &Surface<Self>,
        _outer: &[LoopEdge<Self>],
        _hes: &[HalfEdgeKey],
        _band: Band,
        _tol: Tol,
    ) -> Result<Option<FaceCutBounds>, PropsError> {
        Ok(None)
    }
}

/// The **scalar policy for the certified at-rest gates**
/// (`docs/DUAL-DESIGN.md` DL3): whether an evaluation-service
/// consumer of [`crate::validate_geometric`] /
/// [`crate::validate_pseudomanifold`] runs them at this scalar.
///
/// Certified validation is an act of certification — its tier-3
/// battery re-derives surface certificates
/// ([`PropsQuadLane::recertify_approx`]), encloses volume flux
/// through the quadrature lane, and certifies the contact census —
/// so it belongs to the scalars with certification rights (`f64`,
/// the telemetry probe, the interval scalar), whose impls here
/// delegate to the validation doors verbatim. At a
/// [`Dual`](geom_core::Dual) the gate is **structurally absent**:
/// the impl calls nothing, and its success arm SAYS so — the outcome
/// type separates [`AtRestOutcome::Validated`] from
/// [`AtRestOutcome::NotRunAtThisScalar`], so a dual gate's `Ok` can
/// never be read as a certification at any call site. What makes the
/// absence sound is the PAIRING OBLIGATION: a dual evaluation rides
/// BESIDE a base-scalar evaluation of the same recipe, whose value
/// channel is bit-identical (the dual contract) and which these same
/// gates validate. Nothing in the type system enforces that pairing
/// at the public doors today; the E4 driver's content-key equality
/// assertion (DL3's soundness hook) is a NAMED banked obligation of
/// M10-4's driver. That is sound because
/// a dual evaluation's value channel is bit-identical to the base
/// scalar's run of the same recipe (the dual contract), which the
/// base-scalar evaluation it rides beside already validates;
/// re-validating the same bits through the dual's refusing
/// certification arms adds no information. Asking a dual to
/// validate IS asking it to certify, and a dual never certifies
/// (DL1).
///
/// This is a compile-time, per-scalar policy — never a runtime flag,
/// and never a swallowed per-face error: the dual arm does not run
/// validation and discard refusals, it runs nothing. The validation
/// doors themselves keep their meaning at every `PropsQuadLane`
/// scalar; this trait only decides which scalars' evaluation-service
/// gates consult them.
pub trait AtRestPolicy: PropsQuadLane {
    /// The at-rest gate over a body ([`crate::validate_geometric`] at
    /// certifying scalars; absent at duals, and the outcome says
    /// which).
    ///
    /// # Errors
    ///
    /// The validator's own findings, verbatim, where the scalar runs
    /// it.
    fn gate_at_rest(body: &Body<Self>, tol: Tol) -> Result<AtRestOutcome, Vec<ValidationError>>;

    /// The at-rest gate over a body with declared contacts — the
    /// tier-3′ census door ([`crate::validate_pseudomanifold`] at
    /// certifying scalars; absent at duals).
    ///
    /// # Errors
    ///
    /// The validator's own findings, verbatim, where the scalar runs
    /// it.
    fn gate_at_rest_declared(
        body: &Body<Self>,
        contacts: &ContactRecords,
        tol: Tol,
    ) -> Result<AtRestOutcome, Vec<ValidationError>>;
}

/// What an [`AtRestPolicy`] gate's success MEANS — the word that keeps
/// a non-certifying scalar's `Ok` from reading as a certification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AtRestOutcome {
    /// The certified at-rest validator ran on these bits and passed.
    Validated,
    /// This scalar's policy runs no validator (a dual): nothing was
    /// checked here and nothing is granted. The base-scalar evaluation
    /// beside this one is the validation of record — the DL3 pairing
    /// obligation, stated at the doors that return this.
    NotRunAtThisScalar,
}

impl AtRestPolicy for f64 {
    fn gate_at_rest(body: &Body<Self>, tol: Tol) -> Result<AtRestOutcome, Vec<ValidationError>> {
        crate::validate::validate_geometric(body, tol).map(|()| AtRestOutcome::Validated)
    }

    fn gate_at_rest_declared(
        body: &Body<Self>,
        contacts: &ContactRecords,
        tol: Tol,
    ) -> Result<AtRestOutcome, Vec<ValidationError>> {
        crate::validate::validate_pseudomanifold(body, contacts, tol)
            .map(|()| AtRestOutcome::Validated)
    }
}

#[cfg(feature = "probe")]
impl AtRestPolicy for geom_core::Probe {
    fn gate_at_rest(body: &Body<Self>, tol: Tol) -> Result<AtRestOutcome, Vec<ValidationError>> {
        crate::validate::validate_geometric(body, tol).map(|()| AtRestOutcome::Validated)
    }

    fn gate_at_rest_declared(
        body: &Body<Self>,
        contacts: &ContactRecords,
        tol: Tol,
    ) -> Result<AtRestOutcome, Vec<ValidationError>> {
        crate::validate::validate_pseudomanifold(body, contacts, tol)
            .map(|()| AtRestOutcome::Validated)
    }
}

#[cfg(feature = "interval")]
impl AtRestPolicy for geom_core::interval::Interval {
    fn gate_at_rest(body: &Body<Self>, tol: Tol) -> Result<AtRestOutcome, Vec<ValidationError>> {
        crate::validate::validate_geometric(body, tol).map(|()| AtRestOutcome::Validated)
    }

    fn gate_at_rest_declared(
        body: &Body<Self>,
        contacts: &ContactRecords,
        tol: Tol,
    ) -> Result<AtRestOutcome, Vec<ValidationError>> {
        crate::validate::validate_pseudomanifold(body, contacts, tol)
            .map(|()| AtRestOutcome::Validated)
    }
}

/// The dual arm: STRUCTURALLY ABSENT — no validation door is named,
/// so nothing runs, nothing refuses, and no error exists to swallow;
/// the success arm is [`AtRestOutcome::NotRunAtThisScalar`], never a
/// claim about the geometry (trait docs). The base-scalar evaluation
/// of the same recipe is where these bits are validated — the DL3
/// pairing obligation.
impl<T> AtRestPolicy for geom_core::Dual<T>
where
    geom_core::Dual<T>: PropsQuadLane,
{
    fn gate_at_rest(_body: &Body<Self>, _tol: Tol) -> Result<AtRestOutcome, Vec<ValidationError>> {
        Ok(AtRestOutcome::NotRunAtThisScalar)
    }

    fn gate_at_rest_declared(
        _body: &Body<Self>,
        _contacts: &ContactRecords,
        _tol: Tol,
    ) -> Result<AtRestOutcome, Vec<ValidationError>> {
        Ok(AtRestOutcome::NotRunAtThisScalar)
    }
}

#[cfg(test)]
mod at_rest_policy_tests {
    #![allow(clippy::expect_used)]
    //! The certifying policy arms ARE the validation doors — pinned on
    //! a REFUSING subject, because the passing direction is pinned all
    //! day by every green corpus gather while an arm gutted into a
    //! grant stays invisible to it. Each certifying (scalar, method)
    //! pair is asserted equal to its door on a body the door refuses,
    //! so `Ok(Validated)`-without-validating cannot survive these rows.

    use super::{AtRestOutcome, AtRestPolicy};
    use crate::body::Body;
    use crate::boolean::ContactRecords;
    use geom_core::{Decide, Point3, Tol};

    /// The `mvfs` seed body: its face surface is the placeholder, which
    /// a body at rest may not carry (`Body::mvfs` docs) — the cheapest
    /// scalar-generic refusing subject.
    fn refusing_body<T: Decide>() -> Body<T> {
        let mut b = Body::new();
        b.mvfs(Point3::new(T::zero(), T::zero(), T::zero()))
            .expect("mvfs has no preconditions");
        b
    }

    fn certifying_arms_are_the_doors<T: AtRestPolicy>() {
        let tol = Tol::witness();
        let b = refusing_body::<T>();
        let door = crate::validate::validate_geometric(&b, tol);
        assert!(door.is_err(), "the seed body must refuse validation");
        assert_eq!(
            T::gate_at_rest(&b, tol),
            door.map(|()| AtRestOutcome::Validated),
            "gate_at_rest must be validate_geometric verbatim at a certifying scalar"
        );
        let contacts = ContactRecords::default();
        let door = crate::validate::validate_pseudomanifold(&b, &contacts, tol);
        assert!(door.is_err(), "the seed body must refuse the census door");
        assert_eq!(
            T::gate_at_rest_declared(&b, &contacts, tol),
            door.map(|()| AtRestOutcome::Validated),
            "gate_at_rest_declared must be validate_pseudomanifold verbatim at a certifying scalar"
        );
    }

    #[test]
    fn f64_gates_run_the_doors() {
        certifying_arms_are_the_doors::<f64>();
    }

    #[cfg(feature = "probe")]
    #[test]
    fn probe_gates_run_the_doors() {
        certifying_arms_are_the_doors::<geom_core::Probe>();
    }

    #[cfg(feature = "interval")]
    #[test]
    fn interval_gates_run_the_doors() {
        certifying_arms_are_the_doors::<geom_core::interval::Interval>();
    }

    /// The dual arm on the SAME refusing subject: the gate does not run
    /// and says so — [`AtRestOutcome::NotRunAtThisScalar`], never a
    /// verdict about geometry the door itself refuses.
    #[test]
    fn dual_gate_is_absent_not_a_verdict() {
        let tol = Tol::witness();
        let b = refusing_body::<geom_core::Dual64>();
        assert!(
            crate::validate::validate_geometric(&b, tol).is_err(),
            "the direct door still refuses at a dual"
        );
        assert_eq!(
            <geom_core::Dual64 as AtRestPolicy>::gate_at_rest(&b, tol),
            Ok(AtRestOutcome::NotRunAtThisScalar)
        );
        assert_eq!(
            <geom_core::Dual64 as AtRestPolicy>::gate_at_rest_declared(
                &b,
                &ContactRecords::default(),
                tol
            ),
            Ok(AtRestOutcome::NotRunAtThisScalar)
        );
    }
}

/// The PR 11 certified-quadrature lane's body-side plumbing: stored
/// pcurve caches → ring-bracketed [`geom_brep::props::quad::TrimEdgeQ`]s (C4's first hot
/// consumer). Key-free math stays in `geom_brep::props::quad`; this
/// module owns everything that needs half-edges and vertex points.
mod quad_lane {
    use geom_brep::Pcurve;
    use geom_brep::props::quad::{self, FaceCutBounds, HarmChan, TrimEdgeQ};
    use geom_brep::props::{LoopEdge, PropsError, loop_vector_area};
    use geom_core::Tol;
    use geom_core::ring_interval::RingInterval;
    // The compound `Decide + Bounds` bound below is a RATIFIED seam
    // (M5 PR 11, Evan's lane-split ruling; discipline allowlist row):
    // this module is the certified lanes' plumbing and never
    // instantiates for duals. TWO things enforce that, and since #643
    // the second is the load-bearing one: [`super::PropsQuadLane`]'s
    // explicit impls split the API, and every signature below is
    // `Decide + Bounds + CertifiedEnclosure`, which no `Dual`
    // implements. So the module stays uninstantiable at a dual with or
    // without the lane — a dual carries a bracket since D1
    // (2026-08-19) and still may not certify.
    use geom::Curve3;
    use geom::Surface;
    use geom_core::{Band, Bounds, CertifiedEnclosure, Decide, Point3};

    use crate::body::Body;
    use crate::entity::HalfEdgeKey;

    /// Enclosure midpoint and half-width (the [`super::MassProperties`]
    /// pad decomposition).
    pub(super) fn mid_pad(x: RingInterval) -> (f64, f64) {
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
    ) -> Result<(RingInterval, RingInterval), PropsError> {
        let full = RingInterval::from_bounds(-1.0, 1.0);
        let clamp = |x: RingInterval, pad: f64| {
            RingInterval::from_bounds(x.lo() - pad, x.hi() + pad).clamped_to(-1.0, 1.0)
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
                let c = RingInterval::from_certified(w.dot(*u_ref))
                    / RingInterval::from_certified(*radius);
                let s = RingInterval::from_certified(w.dot(v_ref))
                    / RingInterval::from_certified(*radius);
                let pad = (RingInterval::point(eps) / RingInterval::from_certified(*radius)).mag();
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
                let c = RingInterval::from_certified(w.dot(*u_ref))
                    / RingInterval::from_certified(*major);
                let s = RingInterval::from_certified(w.dot(v_ref))
                    / RingInterval::from_certified(*minor);
                let pad_c = (RingInterval::point(eps) / RingInterval::from_certified(*major)).mag();
                let pad_s = (RingInterval::point(eps) / RingInterval::from_certified(*minor)).mag();
                Ok((clamp(c, pad_c), clamp(s, pad_s)))
            }
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
            c0: RingInterval::from_certified(c0),
            ca: RingInterval::from_certified(ca),
            cb: RingInterval::from_certified(cb),
            cl: RingInterval::from_certified(cl),
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
        // The NURBS-patch lane (M6-3): a described NURBS face routes
        // to the patch engine over its stored iso-line pcurves.
        // The spline-patch lane (M6-3): a described spline face routes
        // to the patch engine over its stored iso-line pcurves. An
        // approximating face enters on its fit — the certificate's
        // bound is a statement about the DESCRIPTION and does not
        // widen this quadrature (the same deliberate omission the
        // mesh tolerance makes).
        if let Some(payload) = surface.spline_chart() {
            return nurbs_face(body, payload, outer, hes, band, tol);
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
        let o_dot_va = RingInterval::from_certified((*origin - Point3::origin()).dot(va));
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
                t0: RingInterval::from_certified(t0),
                t1: RingInterval::from_certified(t1),
                forward: le.forward,
                trig0,
                env: RingInterval::from_certified(cache.certificate().envelope),
            });
        }
        quad::cylinder_cut_face::<T>(
            RingInterval::from_certified(*radius),
            o_dot_va,
            &edges,
            eps,
            band,
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
    fn nurbs_face<T: Decide + Bounds + CertifiedEnclosure>(
        body: &Body<T>,
        payload: &geom::NurbsSurface<T>,
        outer: &[LoopEdge<T>],
        hes: &[HalfEdgeKey],
        band: Band,
        tol: Tol,
    ) -> Result<FaceCutBounds, PropsError> {
        if payload.is_placeholder() {
            return Err(PropsError::QuadratureUnsupported {
                what: "the mvfs Nurbs placeholder reached the quadrature lane — a \
                       mid-surgery body has no mass properties (tier 2 refuses it at rest)",
            });
        }
        let eps = tol.eps();
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
                exact(RingInterval::from_certified(a.x))?,
                exact(RingInterval::from_certified(a.y))?,
            );
            let (bx, by) = (
                exact(RingInterval::from_certified(b.x))?,
                exact(RingInterval::from_certified(b.y))?,
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
            let len = match &le.carrier {
                geom::Curve3::Line { dir, .. } => (RingInterval::from_certified(dir.norm())
                    * RingInterval::from_certified(t1 - t0))
                .mag(),
                geom::Curve3::Nurbs(c) => {
                    let mut l = RingInterval::zero();
                    for w in c.control().windows(2) {
                        l = l + RingInterval::from_certified(w[0].distance(w[1]));
                    }
                    l.mag()
                }
                // An ARC cap rim on a rational wall (M8-3): the metric
                // length is exactly `r·Δθ` — the carrier's own
                // parameter IS the angle, so no bound is needed.
                geom::Curve3::Circle { radius, .. } => (RingInterval::from_certified(*radius)
                    * RingInterval::from_certified(t1 - t0))
                .mag(),
                _ => {
                    return Err(PropsError::QuadratureUnsupported {
                        what: "a NURBS-face boundary carrier outside the loft inventory \
                               (line, spline and circle rims are the minted classes)",
                    });
                }
            };
            perimeter += len;
            boundary_defect +=
                len * RingInterval::from_certified(cache.certificate().envelope).mag();
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
                    RingInterval::from_certified(p.x),
                    RingInterval::from_certified(p.y),
                    RingInterval::from_certified(p.z),
                ]
            })
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
        /// `RingInterval` has no decoration channel, so the quadrature
        /// lane's scalars have to be refused HERE or a certified flux
        /// enclosure gets built from a quantity that was clamped out of
        /// its own domain.
        #[cfg(feature = "interval")]
        #[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
        mod bracket_seam_tests {
            use geom_core::ring_interval::RingInterval;
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
                let r = RingInterval::from_certified(trv_pos());
                assert!(
                    r.is_poison(),
                    "a domain-violated scalar crossed into the ring as {r:?} —                  the bracket door does not read decorations, so the                  quadrature lane certifies a flux built from it"
                );
                // Non-vacuity: a certified scalar crosses with its endpoints.
                let ok = RingInterval::from_certified(Interval::from_bounds(1.0, 4.0).sqrt());
                assert_eq!((ok.lo(), ok.hi()), (1.0, 2.0));
            }

            /// Where a violated scalar would have to come FROM. Every
            /// scalar this lane hands to [`RingInterval::from_certified`]
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
                    assert!(!RingInterval::from_certified(v.norm()).is_poison());
                }
            }

            /// The seam is per scalar, not per channel: one violated
            /// coefficient poisons its own slot and leaves the rest intact,
            /// so the poison reaches the flux algebra where it is visible.
            #[test]
            fn chan_poisons_only_the_violated_coefficient() {
                let one = Interval::from_f64(1.0);
                let c = chan(one, trv_pos(), one, one).expect("channel builds");
                assert!(c.ca.is_poison(), "the violated coefficient survived");
                for (tag, r) in [("c0", c.c0), ("cb", c.cb), ("cl", c.cl)] {
                    assert!(!r.is_poison(), "{tag} poisoned a certified coefficient");
                }
            }
        }
    }
}
