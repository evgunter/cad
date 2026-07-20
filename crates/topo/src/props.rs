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

use geom_brep::props::{FaceContribution, LoopEdge, PropsError, curved_face, planar_face};
use geom_core::{Band, BandError, Decide, Real};
use geom_surfaces::Surface;

use crate::body::Body;
use crate::entity::{FaceKey, LoopBoundary, LoopKey, VertexKey};

/// Exact-B-rep integral properties of a body.
#[derive(Clone, Copy, Debug)]
pub struct MassProperties<T: Real> {
    /// The **signed** enclosed volume by the divergence theorem —
    /// positive for a correctly oriented (outward-normal) closed body;
    /// a definitely negative value is orientation corruption (the
    /// tier-3 +V invariant).
    pub volume: T,
    /// The total surface area (a sum of unsigned face areas).
    pub surface_area: T,
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
pub fn mass_properties<T: Decide>(body: &Body<T>) -> Result<MassProperties<T>, MassPropsError> {
    let band = Band::linear().map_err(|error| MassPropsError::Band { error })?;
    mass_properties_with(body, band)
}

/// [`mass_properties`] against a caller-built band (the tier-3
/// validator's entry — it builds its band once at operation entry).
pub(crate) fn mass_properties_with<T: Decide>(
    body: &Body<T>,
    band: Band,
) -> Result<MassProperties<T>, MassPropsError> {
    let mut flux = T::zero();
    let mut area = T::zero();
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
                    loops.push(loop_edges(body, lk)?);
                }
                planar_face(origin, &loops).map_err(wrap)?
            }
            _ => {
                if !face.rings.is_empty() {
                    return Err(MassPropsError::RingOnCurvedFace { face: face_key });
                }
                let outer = loop_edges(body, face.outer)?;
                curved_face(surface, &outer, band).map_err(wrap)?
            }
        };
        flux = flux + contribution.flux;
        area = area + contribution.area;
    }
    Ok(MassProperties {
        volume: flux / T::from_f64(3.0),
        surface_area: area,
    })
}

/// Flatten one loop's half-edge cycle into key-free [`LoopEdge`]s
/// (traversal order; vertex tags are loop-local first-seen indices).
fn loop_edges<T: Decide>(body: &Body<T>, lk: LoopKey) -> Result<Vec<LoopEdge<T>>, MassPropsError> {
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
    for &he_key in &cycle {
        let Some(he) = body.half_edges.get(he_key) else {
            return Err(corrupt("half-edge key does not resolve"));
        };
        let Some(edge) = body.edges.get(he.edge) else {
            return Err(corrupt("edge key does not resolve"));
        };
        let Some(curve) = body.curves.get(edge.curve) else {
            return Err(corrupt("curve key does not resolve"));
        };
        let Some(end) = body.half_edge_end(he_key) else {
            return Err(corrupt("half-edge mate does not resolve"));
        };
        let (t0, t1) = curve.params();
        edges.push(LoopEdge {
            carrier: *curve.carrier(),
            t0,
            t1,
            forward: he_key == edge.he_plus,
            start: tag_of(he.start),
            end: tag_of(end),
        });
    }
    Ok(edges)
}
