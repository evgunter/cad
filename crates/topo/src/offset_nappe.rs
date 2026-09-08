//! **Which nappe a cone face lies on — decided once for the offset
//! lane.**
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
//! the consumer's. This module is the one place the OFFSET LANE reads
//! it: [`face_nappe`] decides it from the face's own corner stations,
//! [`group_nappe`] agrees it across a chart's faces, and the axial
//! door, the per-chart door, that door's apex-window gate and
//! `geom_brep::ConeOffset::displacement` all turn by the single answer
//! those two return. A second reading of the same fact — per point, per
//! window, per door — is a sign that can disagree with itself, which is
//! the whole hazard.
//!
//! **The tree at large reads the nappe in four more places**, none of
//! them this lane's and none of them this module's to unify:
//! `geom-brep/src/pcurve_cache.rs` (`pcurve_cone_chart_nappe`, per
//! point), `geom-brep/src/props/curved.rs` (`props_cone_nappe`, over a
//! window) and `topo/src/boolean/solid_contain.rs` (three predicates
//! over a slant window). That class is filed as
//! `work/issues/cone-nappe-is-decided-in-five-places.md`.
//!
//! # The premise, ENFORCED
//!
//! The predicate is `offset_nappe`, over the face's extreme corner
//! stations `(p − apex)·axis`. Both extremes are decided, not their
//! sum: a sum is a lever, and it answers `Opening` for a face with
//! corners on both nappes as readily as for one with none there. The
//! extremes bound every other corner, so deciding the two decides the
//! set — and a face whose corners do not all stand strictly on one side
//! of its apex has no nappe and is refused
//! ([`ReplaceFaceError::NappeStraddles`]), never guessed. That includes
//! a face merely TOUCHING its apex: the action's displacement is
//! undetermined there (every azimuth maps to the apex), so the honest
//! answer is the refusal rather than a nappe the touching corner does
//! not have.

use geom::Surface;
use geom_core::k_stats::decide;
use geom_core::{Band, Decide, Margin, Point3, Sign, Vec3};

use crate::body::Body;
use crate::entity::{Face, FaceKey};
use crate::replace_face::ReplaceFaceError;

pub use geom_brep::Nappe;

/// Which nappe `face` lies on, from the face's own corner stations.
///
/// A face carrying any other surface has one sheet, on which the mint's
/// convention and the face-outward one already agree, and answers
/// [`Nappe::Opening`] — whose turn is the identity. That keeps the
/// answer total, so a door turns its `d` the same way for every kind
/// and no caller re-matches on the surface to find out whether the
/// question applies.
///
/// # Errors
///
/// [`ReplaceFaceError::NappeStraddles`] when the face's corners do not
/// all stand strictly on one side of its apex (module docs).
/// [`ReplaceFaceError::Escalated`] when either extreme lands in the
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
    let (station_min, station_max) = corner_stations(body, data, *apex, *axis)?;
    let esc = |source| ReplaceFaceError::Escalated { source };
    let lo = decide("offset_nappe", Margin::of(station_min), band).map_err(esc)?;
    let hi = decide("offset_nappe", Margin::of(station_max), band).map_err(esc)?;
    match (lo, hi) {
        (Sign::Positive, Sign::Positive) => Ok(Nappe::Opening),
        (Sign::Negative, Sign::Negative) => Ok(Nappe::Mirror),
        _ => Err(ReplaceFaceError::NappeStraddles {
            face,
            station_min,
            station_max,
            what: "a cone face whose own corners reach its apex, so it stands on \
                   neither nappe alone",
        }),
    }
}

/// The one nappe a chart GROUP lies on: every face decided, and the
/// agreement enforced.
///
/// A surface can be worn by more than one face, and nothing about
/// sharing a cone makes two faces share a nappe — a chart with a band
/// above the apex and a band below it wears one surface and has two
/// answers. The offset doors move a chart, not a face, so the number
/// they turn is the group's; deciding one member and minting for all of
/// them is how a face's sign comes to stand for its neighbour's.
///
/// # Errors
///
/// [`face_nappe`]'s, plus [`ReplaceFaceError::NappeStraddles`] naming
/// the first member that disagrees with the group's answer.
/// [`ReplaceFaceError::EmptyGroup`] for an empty group.
pub fn group_nappe<T: Decide>(
    body: &Body<T>,
    faces: &[FaceKey],
    band: Band,
) -> Result<Nappe, ReplaceFaceError<T>> {
    let mut agreed: Option<Nappe> = None;
    for &face in faces {
        let here = face_nappe(body, face, band)?;
        match agreed {
            None => agreed = Some(here),
            Some(first) if first == here => {}
            Some(_) => {
                let data = body.get_face(face).ok_or(ReplaceFaceError::Corrupt)?;
                let surface = body
                    .get_surface(data.surface)
                    .ok_or(ReplaceFaceError::Corrupt)?;
                let Surface::Cone { apex, axis, .. } = surface else {
                    return Err(ReplaceFaceError::Corrupt);
                };
                let (station_min, station_max) = corner_stations(body, data, *apex, *axis)?;
                return Err(ReplaceFaceError::NappeStraddles {
                    face,
                    station_min,
                    station_max,
                    what: "a chart whose faces do not all lie on one nappe, so one \
                           offset distance cannot be turned for all of them",
                });
            }
        }
    }
    agreed.ok_or(ReplaceFaceError::EmptyGroup)
}

/// `face`'s extreme corner stations `(min, max)` on the cone
/// `(apex, axis)`.
///
/// The comparison picks WHICH station is metered and decides nothing:
/// the extremes bound every corner, so their two verdicts carry the
/// whole set's.
fn corner_stations<T: Decide>(
    body: &Body<T>,
    data: &Face,
    apex: Point3<T>,
    axis: Vec3<T>,
) -> Result<(T, T), ReplaceFaceError<T>> {
    let mut window: Option<(T, T)> = None;
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
            let h = (p - apex).dot(axis);
            window = Some(match window {
                None => (h, h),
                Some((a, b)) => (a.min(h), b.max(h)),
            });
        }
    }
    window.ok_or(ReplaceFaceError::Corrupt)
}
