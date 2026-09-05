//! **The open bands** — the two carves of a link that ENDS: the planar
//! band between trivalent corners ([`planar`]) and the ruled band cut
//! off at transverse caps ([`ruled`]). Each is one link carved into one
//! blend face, its births recorded in the open-band rows of
//! [`BlendNaming`](crate::blend::naming::BlendNaming) (`feet`, `trims`,
//! `arcs`, `blends`); what the two share beyond that is here.
//!
//! The door, the plans' admission, the closed-rim walks, the ring check,
//! the description pass and the one face-destroying door
//! ([`crate::blend::surgery::SourceFaces::kef_minted`]) are
//! [`crate::blend::surgery`]'s, and
//! its header's refusal classes — Row 1 refuses typed, Row 2 names a
//! frontier, Row 4 panics only on a fact THIS call established — bind
//! every site under this directory as they bind there.

pub(super) mod planar;
pub(super) mod ruled;

use geom_core::Decide;
use topo::{Body, EdgeKey, EntityId, FaceKey, HalfEdgeKey, VertexKey};

use crate::blend::BlendError;
use crate::blend::surgery::{flank, not_intact};

/// [`flank`] on a face's OUTER cycle, refusing typed where the cycle
/// does not walk or does not carry the keyed half-edge — the ruled
/// band's and the corner arc's spelling, whose chords always hang in an
/// outer cycle (the cut-off `mef` leaves a cap's rings on the cap; a
/// support with a ring is refused at the plan).
fn chord_site<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    at: impl Fn(&(HalfEdgeKey, VertexKey, EdgeKey)) -> bool,
    back: usize,
    fwd: usize,
) -> Result<(HalfEdgeKey, HalfEdgeKey, VertexKey, VertexKey), BlendError> {
    let outer = body
        .get_face(face)
        .ok_or_else(|| not_intact(EntityId::Face(face), "a face whose cycle a chord spans"))?
        .outer;
    let ((he1, v1), (he2, v2)) = flank(body, outer, at, back, fwd).ok_or_else(|| {
        not_intact(
            EntityId::Face(face),
            "a face's outer cycle does not walk, or does not carry the half-edge the carve \
             keys on",
        )
    })?;
    Ok((he1, he2, v1, v2))
}
