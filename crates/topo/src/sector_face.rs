//! The vertex-neighborhood **sector face**: which face a sector belongs
//! to, and that face's outward normal at the base vertex — one
//! implementation, shared by the two lanes that ask this question.
//!
//! # What the question is
//!
//! Both the splitting lane (`splitting::neighborhood`) and the boolean
//! lane ([`crate::boolean::sectors`]) walk a vertex orbit and, for the
//! sector CW-after orbit half-edge `he`, must resolve two things before
//! any predicate can run:
//!
//! 1. **Whose corner is this?** The sector after `he` is the corner of
//!    `face(loop(mate(he)))` — the traversal is the same on both sides,
//!    and its derivation lives in `splitting::neighborhood`'s module
//!    docs, which own it.
//! 2. **Which way is out, HERE?** Sector predicates meter against the
//!    face's outward normal **at the base vertex**, which for a charted
//!    face is a local quantity: a plane hands back its stored normal, a
//!    cylinder its chart-outward radial at the vertex point, a sphere
//!    the radial from its centre.
//!
//! **Every arm folds in the face's `sense` bit** (S10) by minting an
//! [`OutwardNormal`]: each reads a CHART normal and returns it as the
//! face's OUTWARD normal, and the chart is the only encoding of
//! orientation the face has. This function is therefore the chokepoint
//! for both vertex-neighborhood lanes: everything downstream —
//! `splitting::rules::apply_rule_a`'s `enters_material` call, the
//! boolean's `within` / `side_code` / `sector_overlap` / `germ_dir` /
//! `pierce_germ_dir`, and the sector-shape rungs on both sides — is
//! sense-invariant GIVEN this value and must NOT multiply again. Those
//! sites pair the normal with the STORED orbit order, which `revert`
//! reverses in the same breath as the sense bit, so a second
//! `sense_sign` factor would cancel this one.
//!
//! # Why the code is here and not in either lane
//!
//! Until S5's second fix this walk existed **twice**, once per lane,
//! and the splitting copy's own doc called itself *"the twin of"* the
//! boolean's. They differed in exactly three things, none of them
//! geometric: the error type they refuse into, whether the caller is
//! told the face was planar, and whether a `Sphere` face has a wired
//! arm. The first two are adaptation the callers do; the third is
//! reported here as a [`SectorCarrier`] and refused by the caller that
//! has no arm for it. What is left — the traversal and all three
//! normals — is one body.
//!
//! Like [`crate::sector_shape`], this module is a **top-level sibling**
//! of `boolean/` and `splitting/` precisely so neither half hosts the
//! other's core; both lanes already depend on this scope
//! (`crate::body`, `crate::entity`), so sharing here adds no dependency
//! edge between the halves and no public API.
//!
//! `sector_shape`'s docs name this unification as the trigger to
//! re-open whether that module belongs in `geom-brep` instead. **It
//! fires and resolves the other way**: this body takes a [`Body`] and
//! three arena keys, so `geom-brep` — which has no `Body` and no arena
//! — cannot host it at any price. The crate root is the deepest scope
//! that can, and the two shared sector modules belong together.

use geom_brep::{OutwardNormal, SurfaceKind};
use geom_core::Decide;

use crate::body::Body;
use crate::entity::{FaceKey, HalfEdgeKey, VertexKey};

/// Which charted carrier a sector's face sits on — the arms with a
/// wired outward-normal-at-the-vertex construction.
///
/// This is a **capability** report, not a surface taxonomy: the kinds
/// with no arm never reach it (they refuse as
/// [`SectorFaceError::Unsupported`]), and a caller whose own pipeline
/// does not execute one of these arms refuses on it by name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SectorCarrier {
    /// A plane: the normal is the stored chart normal.
    Plane,
    /// A cylinder: the normal is the chart-outward radial at the vertex.
    Cylinder,
    /// A sphere: the normal is the radial from the centre.
    Sphere,
}

/// The resolved sector face.
pub(crate) struct SectorFace<T: geom_core::Real> {
    /// The face the sector is a corner of.
    pub face: FaceKey,
    /// That face's OUTWARD unit normal at the base vertex, minted here
    /// from the chart normal and the face's `sense` bit — the type
    /// carries the obligation not to re-apply the sense downstream.
    pub normal: OutwardNormal<T>,
    /// Which arm produced the normal.
    pub carrier: SectorCarrier,
}

/// Why a sector face could not be resolved. Each lane maps this onto
/// its own error type — the boolean's twins additionally carry the
/// `Operand`, which is why the adaptation stays with the callers.
#[derive(Clone, Copy, Debug)]
pub(crate) enum SectorFaceError {
    /// A traversal or arena lookup returned nothing: mate, half-edge,
    /// loop, face, vertex, point or surface. The neighborhood does not
    /// walk — the body is corrupt.
    Corrupt,
    /// The face's surface has no wired sector arm.
    Unsupported {
        /// The face whose carrier has no arm.
        face: FaceKey,
        /// The kind that has none.
        kind: SurfaceKind,
    },
}

/// Resolves the sector CW-after `he` (`face(loop(mate(he)))`) to its
/// face, that face's outward normal at `vertex`, and the carrier arm
/// that produced the normal.
///
/// # Errors
///
/// [`SectorFaceError`] — a corrupt traversal, or a carrier with no arm.
pub(crate) fn sector_face<T: Decide>(
    body: &Body<T>,
    vertex: VertexKey,
    he: HalfEdgeKey,
) -> Result<SectorFace<T>, SectorFaceError> {
    let corrupt = || SectorFaceError::Corrupt;
    let mate = body.mate(he).ok_or_else(corrupt)?;
    let parent = body.get_half_edge(mate).ok_or_else(corrupt)?.parent_loop;
    let face = body.get_loop(parent).ok_or_else(corrupt)?.face;
    let face_data = body.get_face(face).ok_or_else(corrupt)?;
    let sense = face_data.sense;
    // The base vertex's point — read by the charted arms only; the
    // planar arm's normal is a property of the face alone.
    let point = || {
        body.get_vertex(vertex)
            .and_then(|v| body.get_point(v.point))
            .copied()
            .ok_or_else(corrupt)
    };
    let charted = |chart, carrier| {
        Ok(SectorFace {
            face,
            normal: OutwardNormal::from_chart(chart, sense),
            carrier,
        })
    };
    match body.get_surface(face_data.surface).ok_or_else(corrupt)? {
        geom_surfaces::Surface::Plane { normal, .. } => charted(*normal, SectorCarrier::Plane),
        geom_surfaces::Surface::Cylinder { origin, axis, .. } => {
            let w = point()? - *origin;
            charted((w - *axis * w.dot(*axis)).normalize(), SectorCarrier::Cylinder)
        }
        geom_surfaces::Surface::Sphere { center, .. } => {
            charted((point()? - *center).normalize(), SectorCarrier::Sphere)
        }
        s => Err(SectorFaceError::Unsupported {
            face,
            kind: SurfaceKind::of(s),
        }),
    }
}
