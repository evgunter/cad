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
use crate::entity::{EntityId, Face, FaceKey};
use crate::live::{BoundaryMember, proven};
use crate::replace_face::ReplaceFaceError;

pub use geom_brep::Nappe;

/// Which nappe `face` lies on, from the face's own corner stations.
///
/// A face carrying any other surface has one sheet, on which the mint's
/// convention and the face-outward one already agree, and answers
/// [`Nappe::Opening`] — whose turn is the identity. The answer is TOTAL
/// deliberately: a caller holding a face of unknown kind gets a turn it
/// can apply, rather than an `Option` every call site would unwrap the
/// same way. A door that already knows it holds a cone is free to ask
/// only there, and both of them do.
///
/// # Errors
///
/// [`ReplaceFaceError::NappeStraddles`] when the face's corners do not
/// all stand strictly on one side of its apex (module docs).
/// [`ReplaceFaceError::Escalated`] when either extreme lands in the
/// ambiguity band, [`ReplaceFaceError::StaleFace`] when `face` does not
/// resolve.
///
/// # Panics
///
/// On a torn body — the face's surface, a loop, a walk or a corner's
/// point that does not resolve — naming the record (D2 row 4).
pub fn face_nappe<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    band: Band,
) -> Result<Nappe, ReplaceFaceError<T>> {
    let data = body
        .get_face(face)
        .ok_or(ReplaceFaceError::StaleFace { face })?;
    let Surface::Cone { apex, axis, .. } = body.face_surface_linked(face, data) else {
        return Ok(Nappe::Opening);
    };
    let (station_min, station_max) = corner_stations(body, face, data, *apex, *axis);
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
///
/// # Panics
///
/// [`face_nappe`]'s.
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
                // `face_nappe` resolved this face and answered `Mirror`
                // or `Opening` against another member's other answer,
                // and only a cone has two.
                let data = proven(&body.faces, face, EntityId::Face);
                let Surface::Cone { apex, axis, .. } = body.face_surface_linked(face, data) else {
                    unreachable!("{face:?} disagreed on a nappe, which only a cone face has")
                };
                let (station_min, station_max) = corner_stations(body, face, data, *apex, *axis);
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
/// `(apex, axis)`: every half-edge's start, and an empty loop's lone
/// vertex, so a face (whose outer loop is a cycle or holds a vertex)
/// always has one.
///
/// The comparison picks WHICH station is metered and decides nothing:
/// the extremes bound every corner, so their two verdicts carry the
/// whole set's.
///
/// # Panics
///
/// Where a boundary hop past `face` (a loop, a member's edge, a lone vertex's
/// point) does not resolve, or a loop walk does not close
/// ([`crate::live::NAMES_ONLY_LIVE`] / [`crate::body::WALKS_CLOSE`];
/// [`crate::live::OPERATORS_KEEP_LINKS`]).
#[track_caller]
fn corner_stations<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    data: &Face,
    apex: Point3<T>,
    axis: Vec3<T>,
) -> (T, T) {
    let station = |p: Point3<T>| (p - apex).dot(axis);
    let stations: Vec<T> = body
        .face_boundary_linked(face, data)
        .map(|member| {
            station(match member {
                BoundaryMember::Isolated { point, .. } => point,
                BoundaryMember::Edge { he, half, .. } => {
                    body.linked_vertex_point(half.start, EntityId::HalfEdge(he), "start")
                }
            })
        })
        .collect();
    let Some(&first) = stations.first() else {
        unreachable!("{face:?}'s outer loop holds a vertex or a cycle, so it has a corner")
    };
    stations
        .iter()
        .fold((first, first), |(a, b), &h| (a.min(h), b.max(h)))
}

/// **`corner_stations` panics on a ring link that does not resolve**,
/// where it stepped over it.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod torn_hop_rows {
    use super::*;
    use crate::live::OPERATORS_KEEP_LINKS;
    use crate::review_d18::{ROW_FOUR, assert_torn_op_panics};
    use geom_core::Tol;

    #[test]
    fn the_corner_stations_panic_on_a_torn_ring_link() {
        let mut body = crate::test_support_fixtures::geometric_cube::<f64>(Tol::witness()).body;
        let (face, data) = body.faces().next().map(|(k, d)| (k, d.clone())).unwrap();
        let (apex, axis) = (Point3::new(0.0, 0.0, -1.0), Vec3::unit_z());
        let (lo, hi) = corner_stations(&body, face, &data, apex, axis);
        assert!(lo <= hi, "ordered stations");
        let named = crate::review_d18::tear_ring(&mut body, face);
        assert_torn_op_panics(
            "corner_stations",
            &mut body,
            &[&named, ROW_FOUR, OPERATORS_KEEP_LINKS],
            |b| {
                let data = b.get_face(face).unwrap().clone();
                corner_stations(b, face, &data, apex, axis)
            },
        );
    }
}
