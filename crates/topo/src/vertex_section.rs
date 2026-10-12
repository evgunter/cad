//! **A vertex read against the sections a plane ends at it**: the one
//! rule both doors that put a vertex on a plane decide it by — the
//! split's vertex sweep (`splitting::classify`) and the Boolean's
//! vertex against a planar face of the other solid
//! (`boolean::reduce`).
//!
//! A vertex `δ` off the plane is on the plane to the band when `|δ|`
//! reads Zero. The join then ends at it each section the plane cuts in
//! a face it bounds, and certifies each end against it. Where that
//! section is a conic the table constructs from the plane and the
//! face's surface, it passes `δ / sin ψ` from the vertex, `ψ` the
//! dihedral between the plane and the face at the vertex: at a grazing
//! plane the section's end lies far further from the vertex than the
//! plane does. So a vertex is on the plane only where it is also on
//! each such section, and that second reading is decided here, by its
//! own name, before anything is built on it.
//!
//! A planar face's section, and a ruling the table reads a curved face
//! in, is minted straight, through the vertices it ends at or along the
//! ruling they lie on, so it reads nothing past the plane offset. A
//! vertex with no conic section through it reads the plane offset
//! alone.

use geom_core::{Band, Decide, Decided, Indeterminate, Margin, Point3, Sign, UnitVec3};

use crate::body::Body;
use crate::chord_join::{SectionCase, WallSection, wall_section};
use crate::entity::VertexKey;
use crate::validate::decide_reported;

/// K name: a vertex ON a plane read against the conic sections the
/// plane cuts in the faces it bounds — its offset over the sine of the
/// steepest dihedral between the plane and such a face (metres).
pub(crate) const VERTEX_SECTION_SIDE: &str = "vertex_section_side";

/// Decides `vertex`, at point `at` and signed `offset` from the plane
/// through `origin`, ON the plane, above it or below it. `normal` reads
/// the plane's unit normal into the section table, and is called only
/// for a vertex whose offset reads Zero; `None` there leaves the offset
/// as the reading.
///
/// The offset is decided first, under the door's own `name`, as it
/// always was: an in-band offset escalates and a definite one is the
/// verdict. A Zero offset is then read against the sections the plane
/// ends at the vertex ([`section_offset`]), under
/// [`VERTEX_SECTION_SIDE`]: Zero is the verdict ON, a definite reading
/// puts the vertex on the side its offset's sign names, and an in-band
/// one escalates — the vertex lies in the band of a section it would
/// end. `offset` is the door's own signed residual (a split plane's or
/// a face's outward normal), and the section reading keeps its sign.
///
/// # Errors
///
/// The escalation of either reading.
pub(crate) fn decide_vertex_on_plane<T: Decide>(
    name: &'static str,
    body: &Body<T>,
    (vertex, at): (VertexKey, Point3<T>),
    (origin, normal): (Point3<T>, impl FnOnce() -> Option<UnitVec3<T>>),
    offset: T,
    band: Band,
) -> Result<Decided, Indeterminate> {
    let on_plane = decide_reported(name, Margin::of(offset), band)?;
    if on_plane.sign != Sign::Zero {
        return Ok(on_plane);
    }
    let Some(normal) = normal() else {
        return Ok(on_plane);
    };
    match section_offset(body, (vertex, at), (origin, normal), offset, band) {
        Some(reach) => decide_reported(VERTEX_SECTION_SIDE, Margin::of(reach), band),
        None => Ok(on_plane),
    }
}

/// The vertex's distance from the conic sections the plane cuts in the
/// faces it bounds, signed as `offset`: `offset / sin ψ` at the face
/// whose dihedral `ψ` with the plane at the vertex is shallowest, the
/// first-order distance along that face from the vertex to where the
/// plane meets it. `None` where no face at the vertex has a conic
/// section: a planar face, a ruling, a face the table refuses (that
/// refusal is the join's to report, where it reads the face), and a
/// vertex whose orbit does not read.
fn section_offset<T: Decide>(
    body: &Body<T>,
    (vertex, at): (VertexKey, Point3<T>),
    (origin, normal): (Point3<T>, UnitVec3<T>),
    offset: T,
    band: Band,
) -> Option<T> {
    let n = normal.get();
    body.faces_of_vertex(vertex)?
        .into_iter()
        .filter_map(
            |face| match wall_section(body, band, origin, normal, face, vertex) {
                Ok(Some(WallSection {
                    wall,
                    case: SectionCase::Conic(_),
                })) => Some(
                    n.cross(geom_brep::implicit_gradient(&wall, at).normalize())
                        .norm(),
                ),
                _ => None,
            },
        )
        .reduce(|a, b| a.min(b))
        .map(|sine| offset / sine)
}
