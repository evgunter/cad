//! **Which nappe a cone face lies on — decided once, for everyone.**
//!
//! A cone is a double cone, and `geom_brep::offset_surface` moves it by
//! sliding the apex along the axis: the pushforward along the
//! continuous extension of the OPENING nappe's normal field
//! (`geom_brep::ConeOffset`'s ratified contract). `n₊` does not flip
//! across the apex, so a mirror-nappe face's material moves `−d` along
//! its OWN chart normal — and a distance stated the way every offset
//! door states it, along the FACE's outward direction, has to be turned
//! before it reaches that action.
//!
//! Which nappe a FACE is on is a fact only the face has, so the turn is
//! the consumer's. This module is the one place that reads it:
//! [`face_nappe`] decides it from the face's own corner stations, and
//! the axial door, the per-chart door, that door's apex-window gate and
//! `geom_brep::ConeOffset::displacement` all turn by the single answer
//! it returns. A second reading of the same fact — per point, per
//! window, per door — is a sign that can disagree with itself, which is
//! the whole hazard.
//!
//! The predicate is `offset_nappe`, over the SUM of the face's corner
//! stations `Σ (p − apex)·axis`. A face standing at its own apex is on
//! neither nappe and is refused
//! ([`ReplaceFaceError::NappeStraddles`]), never guessed.

use geom::Surface;
use geom_core::k_stats::decide;
use geom_core::{Band, Decide, Margin, Sign};

use crate::body::Body;
use crate::entity::FaceKey;
use crate::replace_face::ReplaceFaceError;

pub use geom_brep::Nappe;

/// Which nappe `face` lies on, from the face's own corner stations.
///
/// The SUM of `(p − apex)·axis` over every corner of every loop. Every
/// corner of a cone face is on one nappe, so the sum carries that
/// nappe's sign; it is a length — no lever, and no comparison to pick a
/// maximum.
///
/// A face carrying any other surface has one sheet, on which the mint's
/// convention and the face-outward one already agree, and answers
/// [`Nappe::Opening`] — whose turn is the identity.
///
/// # Errors
///
/// [`ReplaceFaceError::NappeStraddles`] when the sum is coincident with
/// zero: the face stands at its own apex and is on neither nappe.
/// [`ReplaceFaceError::Escalated`] when the predicate lands in the
/// ambiguity band, [`ReplaceFaceError::Corrupt`] on a key that does not
/// resolve.
pub fn face_nappe<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    band: Band,
) -> Result<Nappe, ReplaceFaceError<T>> {
    let data = body.get_face(face).ok_or(ReplaceFaceError::Corrupt)?;
    let surface = body
        .get_surface(data.surface)
        .ok_or(ReplaceFaceError::Corrupt)?;
    let Surface::Cone { apex, axis, .. } = surface else {
        return Ok(Nappe::Opening);
    };
    let (apex, axis) = (*apex, *axis);
    let mut station = T::zero();
    for lk in core::iter::once(data.outer).chain(data.rings.iter().copied()) {
        let crate::entity::LoopBoundary::Cycle { first } =
            body.get_loop(lk).ok_or(ReplaceFaceError::Corrupt)?.boundary
        else {
            continue;
        };
        for he in body.loop_cycle(first).ok_or(ReplaceFaceError::Corrupt)? {
            let p = body
                .get_half_edge(he)
                .and_then(|h| body.get_vertex(h.start))
                .and_then(|x| body.get_point(x.point).copied())
                .ok_or(ReplaceFaceError::Corrupt)?;
            station = station + (p - apex).dot(axis);
        }
    }
    match decide("offset_nappe", Margin::of(station), band) {
        Ok(Sign::Positive) => Ok(Nappe::Opening),
        Ok(Sign::Negative) => Ok(Nappe::Mirror),
        Ok(Sign::Zero) => Err(ReplaceFaceError::NappeStraddles { face, station }),
        Err(source) => Err(ReplaceFaceError::Escalated { source }),
    }
}
