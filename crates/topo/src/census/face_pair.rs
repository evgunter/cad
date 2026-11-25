//! **The one reading of a face pair behind a box overlap**
//! ([`faces_apart`]): whether two faces, each of its own body and both in
//! one frame, are certainly apart. Arm 1 of the census's cross-solid
//! backstop, the separation doors ([`crate::Separation`],
//! [`crate::SolidSeparation`]) and the boolean's pair doors (the operand
//! gate, the NURBS re-gate) read it; it reads the census's depth and
//! in-plane gaps ([`super::depth_gap`], [`super::in_plane`]) and the
//! narrow phase's directions ([`crate::separating`]).

use geom_core::{Band, Decide};

use super::{depth_gap, in_plane};
use crate::body::Body;
use crate::entity::FaceKey;

/// **Whether two faces, each of its own body and both in one frame, are
/// certainly apart** behind a box overlap: the one reading of a face
/// pair that arm 1 of [`super::sweep_cross_solid_backstop`], the
/// separation doors ([`crate::Separation`], [`crate::SolidSeparation`])
/// and the boolean's pair doors (the operand gate, the NURBS re-gate)
/// share. It proves two faces' LOCI disjoint, and nothing about the
/// material either bounds: a door that grants two solids disjoint reads
/// their containment besides. Each step is read only where the one
/// before it leaves the pair:
///
/// - a gap along either face's own normal where it is planar, along one
///   of the caller's `axes`, or along the axis between the two faces'
///   anchors, each face read by its plain cover
///   ([`crate::separating::apart`]);
/// - a gap along every direction the pair's own geometry aims
///   ([`depth_gap::proposed`]: the offset from an axis or centre either
///   face proposes to the other's nearest boundary segment point, either
///   face's arc chords' normals, the line between the two carriers' axes
///   or centres), each face's cover read along the direction with a
///   sphere face's pole-side cut ([`crate::separating::apart_by`],
///   [`crate::cover::face_cover_toward`]). The aimed cover differs from
///   the plain one only on a sphere face, so the first step's normals and
///   axes are read again only where a face is a sphere; the anchor axis,
///   which [`crate::separating::apart_by`] always reads, is read again on
///   every pair;
/// - a gap in depth about an axis, a centre or a circle either face
///   proposes ([`depth_gap::apart_in_depth`]), which clears a nested
///   pair no direction separates;
/// - for two planar faces, a gap where one meets the other's plane
///   ([`in_plane::apart_in_plane`]).
///
/// The direction gaps are less `pad` on each face; the depth and
/// in-plane gaps less each face's [`crate::boxes::sweep_pad`]. Each
/// clears only where it is decided definitely positive at this lane's
/// scalar, and each cover holds its face's locus, so any one of them
/// proves the loci disjoint. Every reading is of the two faces alone, so
/// the verdict does not depend on how the pair is turned. A face with no
/// cover is never apart.
///
/// The pole-side cut places a point in a face through its region
/// ([`crate::contain::curved_face_placement`]), and narrows only on a
/// definite `Out`. That is the door the boolean's curved sweep arm
/// places its pierce and end points by on the same bodies mid-operation
/// (`boolean::reduce::curved_face_arm`), so a body the boolean's doors
/// read here is one the operation already trusts that door on.
///
/// # Panics
///
/// As [`crate::cover::face_cover`]: each face is one the caller read
/// out of its body.
pub(crate) fn faces_apart<T: Decide>(
    (x, fa): (&Body<T>, FaceKey),
    (y, fb): (&Body<T>, FaceKey),
    axes: &[geom_core::UnitVec3<T>],
    pad: f64,
    band: Band,
) -> bool {
    use crate::separating::{Item, apart, apart_by, planar_axis};
    let (xi, yi) = ((x, Item::Face(fa)), (y, Item::Face(fb)));
    let mut dirs: Vec<_> = [(x, fa), (y, fb)]
        .into_iter()
        .filter_map(|(body, f)| planar_axis(body, f, band))
        .collect();
    dirs.extend_from_slice(axes);
    if apart(xi, yi, &dirs, pad, band) {
        return true;
    }
    // Both faces are read once for their depths, which aim the offset
    // directions and bound the depth gap.
    let reads = depth_gap::face_read(x, fa, band).zip(depth_gap::face_read(y, fb, band));
    let proposed = reads
        .as_ref()
        .map_or_else(Vec::new, |(rx, ry)| depth_gap::proposed(rx, ry, band));
    let kind = |body: &Body<T>, f| body.proven_face(f).surface().kind();
    // The aimed cover differs from the plain one only on a sphere face
    // (its pole-side cut), so the directions the first read tried are
    // read again only where one face is a sphere.
    let sphere = [(x, fa), (y, fb)]
        .into_iter()
        .any(|(body, f)| kind(body, f) == geom::SurfaceKind::Sphere);
    if !sphere {
        dirs.clear();
    }
    dirs.extend(proposed.iter().copied());
    let planar = |body: &Body<T>, f| kind(body, f) == geom::SurfaceKind::Plane;
    apart_by(xi, yi, &dirs, pad, band, &|body, f, aim| {
        crate::cover::face_cover_toward(body, f, band, aim).ok()
    }) || reads
        .as_ref()
        .is_some_and(|(rx, ry)| depth_gap::apart_in_depth((x, fa, rx), (y, fb, ry), band))
        || (planar(x, fa)
            && planar(y, fb)
            && in_plane::apart_in_plane((x, fa), (y, fb), &proposed, band))
}
