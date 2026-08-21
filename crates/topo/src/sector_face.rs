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
use crate::entity::{EntityId, FaceKey, HalfEdgeKey, VertexKey};
use crate::face_normal::face_outward_normal;
use geom_core::Tol;

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
#[derive(Debug)]
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
    /// A traversal or arena lookup returned nothing — the neighborhood
    /// does not walk, the body is corrupt. The payload is the entity
    /// that did not resolve, or (when the missing key was the geometry
    /// hanging off one) the entity that carried it: the walk's own
    /// half-edge, the mate, the parent loop, the face, or the base
    /// vertex. Each lane maps it onto its own corruption arm, and the
    /// payload is what makes that arm say WHERE.
    Corrupt(EntityId),
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
/// Named `resolve` rather than `sector_face`: the module is the noun,
/// and each lane's thin wrapper keeps that name for its own callers,
/// so `sector_face` in prose means one thing per scope instead of
/// three.
///
/// # Errors
///
/// [`SectorFaceError`] — a corrupt traversal, or a carrier with no arm.
pub(crate) fn resolve<T: Decide>(
    body: &Body<T>,
    vertex: VertexKey,
    he: HalfEdgeKey,
) -> Result<SectorFace<T>, SectorFaceError> {
    let mate = body
        .mate(he)
        .ok_or(SectorFaceError::Corrupt(EntityId::HalfEdge(he)))?;
    let parent = body
        .get_half_edge(mate)
        .ok_or(SectorFaceError::Corrupt(EntityId::HalfEdge(mate)))?
        .parent_loop;
    let face = body
        .get_loop(parent)
        .ok_or(SectorFaceError::Corrupt(EntityId::Loop(parent)))?
        .face;
    // The planar arm goes through the crate's ONE sense-flip door
    // ([`crate::face_normal`]) rather than re-deriving the flip, and
    // reads nothing else: a plane's outward normal is a property of
    // the face alone.
    if let Some(normal) = face_outward_normal(body, face) {
        return Ok(SectorFace {
            face,
            normal,
            carrier: SectorCarrier::Plane,
        });
    }
    // The charted arms need the face's own chart and the base vertex.
    let face_data = body
        .get_face(face)
        .ok_or(SectorFaceError::Corrupt(EntityId::Face(face)))?;
    let sense = face_data.sense;
    let point = || {
        body.get_vertex(vertex)
            .ok_or(SectorFaceError::Corrupt(EntityId::Vertex(vertex)))
            .and_then(|v| {
                body.get_point(v.point)
                    .copied()
                    .ok_or(SectorFaceError::Corrupt(EntityId::Vertex(vertex)))
            })
    };
    let charted = |chart, carrier| {
        Ok(SectorFace {
            face,
            normal: OutwardNormal::from_chart(chart, sense),
            carrier,
        })
    };
    match body
        .get_surface(face_data.surface)
        .ok_or(SectorFaceError::Corrupt(EntityId::Face(face)))?
    {
        geom::Surface::Cylinder { origin, axis, .. } => {
            let w = point()? - *origin;
            charted(
                (w - *axis * w.dot(*axis)).normalize(),
                SectorCarrier::Cylinder,
            )
        }
        geom::Surface::Sphere { center, .. } => {
            charted((point()? - *center).normalize(), SectorCarrier::Sphere)
        }
        // The planar arm returned above; anything else has no arm.
        s => Err(SectorFaceError::Unsupported {
            face,
            kind: SurfaceKind::of(s),
        }),
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use geom_core::Tol;
    use super::*;
    use crate::fixtures::prism;

    /// A sphere face, installed on a prism's side face so the orbit
    /// around one of its vertices walks a sphere-carried sector.
    fn sphere_sided_prism() -> (crate::Body<f64>, crate::entity::FaceKey) {
        let p = prism(3, Tol::witness());
        let face = p.face_side[0];
        let mut body = p.body;
        body.set_face_surface(
            face,
            crate::FaceSurface::New(geom::Surface::Sphere {
                center: geom_core::Point3::new(0.0, 0.0, 0.0),
                radius: 2.0,
                axis: geom_core::Vec3::new(0.0, 0.0, 1.0),
                u_ref: geom_core::Vec3::new(1.0, 0.0, 0.0),
            }),
        )
        .unwrap();
        (body, face)
    }

    /// The corruption payload names WHAT did not resolve, which is the
    /// whole point of it carrying one: a dangling half-edge reports
    /// that half-edge, not the base vertex the lanes' public arms are
    /// limited to.
    #[test]
    fn corrupt_names_the_entity_that_did_not_resolve() {
        let p = prism(3, Tol::witness());
        let bogus = crate::entity::HalfEdgeKey::default();
        match resolve(&p.body, p.t[0], bogus) {
            Err(SectorFaceError::Corrupt(EntityId::HalfEdge(k))) => assert_eq!(k, bogus),
            other => panic!("expected the dangling half-edge named, got {other:?}"),
        }
    }

    /// The charted arms read the base vertex, so a dangling VERTEX is
    /// reported as the vertex — the same payload, a different entity.
    #[test]
    fn a_charted_arm_names_the_vertex_it_could_not_read() {
        let (body, face) = sphere_sided_prism();
        let he = body
            .get_loop(body.get_face(face).unwrap().outer)
            .and_then(|l| match l.boundary {
                crate::entity::LoopBoundary::Cycle { first } => Some(first),
                crate::entity::LoopBoundary::Empty { .. } => None,
            })
            .unwrap();
        // The sector CW-after `mate(he)` is this face's corner.
        let orbit_he = body.mate(he).unwrap();
        let bogus = crate::entity::VertexKey::default();
        match resolve(&body, bogus, orbit_he) {
            Err(SectorFaceError::Corrupt(EntityId::Vertex(k))) => assert_eq!(k, bogus),
            other => panic!("expected the dangling vertex named, got {other:?}"),
        }
    }

    /// The sphere arm is WIRED here (the boolean lane executes it) and
    /// reports its carrier, so the lane without a sphere arm can refuse
    /// by name rather than by kind-matching a surface itself.
    #[test]
    fn the_sphere_arm_is_wired_and_names_its_carrier() {
        let (body, face) = sphere_sided_prism();
        let he = body
            .get_loop(body.get_face(face).unwrap().outer)
            .and_then(|l| match l.boundary {
                crate::entity::LoopBoundary::Cycle { first } => Some(first),
                crate::entity::LoopBoundary::Empty { .. } => None,
            })
            .unwrap();
        let orbit_he = body.mate(he).unwrap();
        let vertex = body.get_half_edge(orbit_he).unwrap().start;
        let resolved = resolve(&body, vertex, orbit_he).expect("the sphere arm resolves");
        assert_eq!(resolved.carrier, SectorCarrier::Sphere);
        assert_eq!(resolved.face, face);
    }
}
