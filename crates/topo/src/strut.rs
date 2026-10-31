//! **Deleting a strut from the face it lies in.** A strut here is an
//! edge whose two halves bound one face and one of whose ends is free
//! (a valence-one vertex). It encloses no area: deleting it with its free
//! end leaves the face's region as it was, whatever the edge's direction.
//! An edge whose other end is free too is a ring holding nothing but
//! itself; the first delete leaves that ring a lone vertex, which a
//! `mekr` to the face's outline and a `kev` delete with the ring.
//!
//! Two doors leave struts and delete them here:
//!
//! - the coplanar merge, whose absorption leaves the dangling legs of a
//!   seam on its survivor (`Body::merge_coplanar_faces`);
//! - the Boolean, whose zip glues a slit's face to itself where the
//!   result keeps that face on both sides of the slit's line
//!   (`boolean::slit::close_struts`).
//!
//! An edge whose two halves bound one face and neither of whose ends is
//! free is no strut: it closes the face across a stretch of a slit the
//! slit runs on past, and deleting it parts the slit's loop, which the
//! Boolean does with `kemr` (`boolean::slit::part_mid_slit`).
//!
//! A delete is planned, then run ([`Body::plan_strut_prune`],
//! [`Body::prune_strut`]): the plan only reads, so a door can re-check
//! what it proved between the two. Only a planar face is pruned: the
//! bridge `mekr` mints is a line chord, which lies in a plane's face and
//! in no curved one's.

use geom_core::Tol;

use crate::body::Body;
use crate::entity::{EdgeKey, EntityId, FaceKey, HalfEdgeKey, LoopBoundary, LoopKey, VertexKey};
use crate::euler::EulerOpError;
use crate::live::proven;

/// A strut delete, as [`Body::plan_strut_prune`] read it.
#[derive(Clone, Copy, Debug)]
pub(crate) struct StrutPrune {
    /// Its half running toward its free end, which `kev` takes.
    pub(crate) toward_free: HalfEdgeKey,
    /// The free end.
    pub(crate) free: VertexKey,
    /// Where the other end is free too: that end, and the loop the
    /// strut alone makes, which the first delete leaves a lone vertex.
    pub(crate) lone: Option<(VertexKey, LoopKey)>,
    /// The face the strut lies in.
    pub(crate) face: FaceKey,
}

/// Why [`Body::plan_strut_prune`] does not delete a strut.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum StrutRefusal {
    /// The face is not a plane, where the lone ring's bridge does not
    /// lie.
    Curved,
    /// Both ends are free and the strut is the face's outline: the face
    /// would be left with a lone vertex for its outline, which bounds no
    /// area.
    LoneOutline,
}

impl<T: crate::props::AtRestPolicy> Body<T> {
    /// **Does `toward` dangle alone at its start vertex**: is that vertex
    /// the free end of a strut?
    ///
    /// # Panics
    ///
    /// Where `toward`'s start vertex's orbit does not close: on a
    /// tier-1-valid body every orbit does.
    #[track_caller]
    pub(crate) fn strut_tip(&self, toward: HalfEdgeKey) -> bool {
        self.orbit_walk(toward).closed("orbit", toward).len() == 1
    }

    /// The half of `edge` running toward a free end of it, the plus
    /// half's end asked first; `None` where neither end is free.
    ///
    /// # Panics
    ///
    /// Where `edge` does not resolve, or an orbit does not close
    /// ([`Body::strut_tip`]).
    #[track_caller]
    pub(crate) fn toward_free_end(&self, edge: EdgeKey) -> Option<HalfEdgeKey> {
        let e = proven(&self.edges, edge, EntityId::Edge);
        [(e.he_plus, e.he_minus), (e.he_minus, e.he_plus)]
            .into_iter()
            .find(|&(_, from_free)| self.strut_tip(from_free))
            .map(|(toward, _)| toward)
    }

    /// **Plans the delete of a strut of `face`** by `toward_free`, its
    /// half running toward a free end ([`Body::toward_free_end`]). Reads
    /// only.
    ///
    /// # Errors
    ///
    /// [`StrutRefusal::Curved`] where `face` is not a plane;
    /// [`StrutRefusal::LoneOutline`] where both ends are free and the
    /// strut is `face`'s outline.
    ///
    /// # Panics
    ///
    /// Where `face`, `toward_free` or its mate does not resolve, or an
    /// orbit does not close.
    #[track_caller]
    pub(crate) fn plan_strut_prune(
        &self,
        face: FaceKey,
        toward_free: HalfEdgeKey,
    ) -> Result<StrutPrune, StrutRefusal> {
        let record = proven(&self.faces, face, EntityId::Face);
        if self.face_surface_linked(face, record).kind() != geom::SurfaceKind::Plane {
            return Err(StrutRefusal::Curved);
        }
        let toward = proven(&self.half_edges, toward_free, EntityId::HalfEdge);
        let (other_end, ring) = (toward.start, toward.parent_loop);
        let free = self.half_edge_end(toward_free).unwrap_or_else(|| {
            unreachable!("strut half {toward_free:?} resolved above, and a live half has an end")
        });
        let lone = self.strut_tip(toward_free).then_some((other_end, ring));
        if lone.is_some() && ring == record.outer {
            return Err(StrutRefusal::LoneOutline);
        }
        Ok(StrutPrune {
            toward_free,
            free,
            lone,
            face,
        })
    }

    /// **Runs a planned strut delete**: `kev` deletes the strut with its
    /// free end, and where the other end was free too, the lone vertex
    /// left goes with its ring ([`Body::delete_lone_ring`]). Returns the
    /// deleted vertices, the free end first.
    ///
    /// # Errors
    ///
    /// The Euler operators' refusals.
    pub(crate) fn prune_strut(
        &mut self,
        plan: &StrutPrune,
        tol: Tol,
    ) -> Result<Vec<VertexKey>, EulerOpError> {
        self.kev(plan.toward_free)?;
        let mut deleted = vec![plan.free];
        if let Some((lone, ring)) = plan.lone {
            self.delete_lone_ring(plan.face, ring, tol)?;
            deleted.push(lone);
        }
        Ok(deleted)
    }

    /// Deletes the lone-vertex ring `ring` of `face` with its vertex:
    /// `mekr` bridges it to `face`'s outline and `kev` kills the bridge
    /// and the vertex.
    ///
    /// # Errors
    ///
    /// The Euler operators' refusals.
    ///
    /// # Panics
    ///
    /// Where `face`'s outline is a lone vertex: every caller stops
    /// before it reduces an outline to one (an outline bounds its face's
    /// area).
    pub(crate) fn delete_lone_ring(
        &mut self,
        face: FaceKey,
        ring: LoopKey,
        tol: Tol,
    ) -> Result<(), EulerOpError> {
        let outline = proven(&self.faces, face, EntityId::Face).outer;
        let target = match crate::live::linked(
            &self.loops,
            outline,
            EntityId::Loop,
            EntityId::Face(face),
            "outer",
        )
        .boundary
        {
            LoopBoundary::Cycle { first } => first,
            LoopBoundary::Empty { .. } => unreachable!(
                "face {face:?}'s outline {outline:?} is a lone vertex: every caller stops before \
                 it reduces an outline to one"
            ),
        };
        let bridge = self.mekr_chord(
            crate::construction::MekrSite::EmptyRing { target, ring },
            tol,
        )?;
        self.kev(bridge.he_plus)?;
        Ok(())
    }
}
