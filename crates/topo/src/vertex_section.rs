//! **A vertex read against the sections a plane ends at it**: the one
//! rule both doors that put a vertex on a plane decide it by — the
//! split's vertex sweep (`splitting::classify`) and the Boolean's
//! vertex against a planar face of the other solid
//! (`boolean::reduce`).
//!
//! A vertex `δ` off the plane is on the plane to the band when `|δ|`
//! reads Zero. The join then ends at it each section the plane cuts
//! into a face it bounds, and certifies each end against it. Where that
//! section is a curve the plane and the face's surface make, it passes
//! `δ / sin ψ` from the vertex, `ψ` the dihedral between the plane and
//! the face there: at a grazing plane the section's end lies far
//! further from the vertex than the plane does. So a vertex is on the
//! plane only where it is also on each such section, and that second
//! reading is decided here, by its own name, before anything is built
//! on it.
//!
//! Which faces a section enters from the vertex is the neighbourhood
//! walk's reading of the vertex's edges
//! ([`crate::splitting::entry_side`]): a face's sector holds the plane's
//! trace where its two bounds leave on opposite sides of the plane, or
//! where it is wide and its subdivision direction leaves on the side
//! its bounds do not. A sector with a bound in the plane holds the
//! trace along that edge, which ends at the vertex. Of the faces a
//! section enters, a planar one is cut along a line the join mints
//! through its ends, and a cylinder or cone the table
//! (`chord_join::wall_section`) reads in rulings is cut straight too;
//! a ruling's own distance from the vertex is read where its crossing
//! is placed, not here. Every other entered face is read.
//!
//! The reading divides by `sin ψ`, so its own rounding is charged
//! through the division ([`geom_core::running::Rounded`]). It is the
//! input's reading only where that charge is within `ε/K`
//! ([`UnsupportedConditioning::admit`]). Past it, only a verdict the
//! charge cannot move stands; the rest is the kernel's, refused with no
//! lever.

use geom_brep::ReferenceNormal;
use geom_brep::recourse::UnsupportedConditioning;
use geom_core::running::{self, Rounded};
use geom_core::{Band, Decide, Decided, Indeterminate, Margin, Point3, Sign, UnitVec3, Vec3};

use crate::body::Body;
use crate::chord_join::{SectionCase, WallSection, wall_section};
use crate::entity::{EntityId, FaceKey, HalfEdgeKey, VertexKey};
use crate::sector_shape::{SectorFault, SectorShape, sector_shape};
use crate::splitting::{EntryFault, PlaneSide, SplitReduceError, bisector_side, entry_side};
use crate::validate::decide_reported;

/// K name: a vertex ON a plane read against the sections the plane cuts
/// into the faces it bounds — its offset over the sine of the shallowest
/// dihedral between the plane and such a face (metres).
pub(crate) const VERTEX_SECTION_SIDE: &str = "vertex_section_side";

/// K name: that reading's own rounding, `K` times over, against the
/// zero band (metres).
pub(crate) const VERTEX_SECTION_RESOLUTION: &str = "vertex_section_resolution";

/// K name: that reading moved toward zero by its own rounding, where the
/// rounding is past `ε/K` (metres): only a definite side stands.
pub(crate) const VERTEX_SECTION_CLEAR: &str = "vertex_section_clear";

/// The question a refusal of the section reading states.
const SUBJECT: &str = "whether a vertex lies on the section the plane cuts in a face at it";

/// The plane a vertex is read against, as the section table and the
/// walk read it: a point on it, its unit normal, and the reference side
/// a conic's bend is read against (a split plane's own normal, or a
/// face's outward one).
#[derive(Clone, Copy, Debug)]
pub(crate) struct SectionPlane<T: geom_core::Real> {
    pub origin: Point3<T>,
    pub normal: UnitVec3<T>,
    pub reference: ReferenceNormal<T>,
}

/// Why a vertex has no verdict on a plane.
#[derive(Debug)]
pub(crate) enum VertexRefusal {
    /// Its offset, or its distance from a section it would end, lies in
    /// the band: the input's.
    Undecided(Indeterminate),
    /// Which side of the plane a bound of `face`'s sector leaves the
    /// vertex on lies in the band.
    Sector {
        /// The face whose sector was read.
        face: FaceKey,
        /// The escalated reading.
        diag: Indeterminate,
    },
    /// The walk refused the vertex's neighbourhood.
    Walk(SplitReduceError),
    /// The door could not read its plane into the section table; the
    /// clause says why.
    Plane(&'static str),
    /// Its distance from a section carries rounding past `ε/K`: the
    /// kernel's.
    Unresolved(UnsupportedConditioning),
}

/// Decides `vertex`, at point `at` and signed `offset` from `plane`, ON
/// the plane, above it or below it.
///
/// The offset is decided first, under the door's own `name`, as it
/// always was: an in-band offset escalates and a definite one is the
/// verdict. A Zero offset is then read against the sections the plane
/// ends at the vertex (module docs), under [`VERTEX_SECTION_SIDE`]:
/// Zero is the verdict ON, a definite reading puts the vertex on the
/// side its offset's sign names, and an in-band one escalates. `offset`
/// is the door's own signed residual, and the section reading keeps its
/// sign. `plane` is read only for a vertex whose offset reads Zero and
/// that bounds a read face; `far` is a far vertex's side, for the walk's
/// line edges.
///
/// # Errors
///
/// [`VertexRefusal`].
pub(crate) fn decide_vertex_on_plane<T: Decide>(
    name: &'static str,
    body: &Body<T>,
    (vertex, at): (VertexKey, Point3<T>),
    plane: impl FnOnce() -> Result<SectionPlane<T>, VertexRefusal>,
    (offset, far): (T, impl Fn(VertexKey) -> Result<PlaneSide, VertexRefusal>),
    band: Band,
) -> Result<Decided, VertexRefusal> {
    let on_plane =
        decide_reported(name, Margin::of(offset), band).map_err(VertexRefusal::Undecided)?;
    if on_plane.sign != Sign::Zero {
        return Ok(on_plane);
    }
    read_sections(body, (vertex, at), plane, (offset, on_plane, far), band)
}

/// [`decide_vertex_on_plane`] past the offset: `on_plane` is the offset's
/// Zero verdict, which stands where no section is read.
///
/// # Errors
///
/// [`VertexRefusal`].
pub(crate) fn read_sections<T: Decide>(
    body: &Body<T>,
    (vertex, at): (VertexKey, Point3<T>),
    plane: impl FnOnce() -> Result<SectionPlane<T>, VertexRefusal>,
    (offset, on_plane, far): (
        T,
        Decided,
        impl Fn(VertexKey) -> Result<PlaneSide, VertexRefusal>,
    ),
    band: Band,
) -> Result<Decided, VertexRefusal> {
    let orbit = body.vertex_orbit_linked(vertex);
    let faces: Vec<FaceKey> = orbit
        .iter()
        .map(|&he| sector_face(body, vertex, he))
        .collect();
    if !faces.iter().any(|&face| read_kind(body, face, at)) {
        return Ok(on_plane);
    }
    let plane = plane()?;
    let entered = entered_faces(body, (vertex, at), (&orbit, &faces), plane, &far, band)?;
    let Some(sine) = entered
        .into_iter()
        .filter_map(|face| section_sine(body, (vertex, at), face, plane, band))
        .reduce(|a, b| Rounded::select_le_zero(b.value - a.value, b, a))
    else {
        return Ok(on_plane);
    };
    let reading = Rounded {
        value: offset,
        error: offset_rounding(at, plane),
    }
    .quotient(sine);
    match UnsupportedConditioning::admit(VERTEX_SECTION_RESOLUTION, SUBJECT, reading.error, band) {
        Ok(()) => decide_reported(VERTEX_SECTION_SIDE, Margin::of(reading.value), band)
            .map_err(VertexRefusal::Undecided),
        Err(unresolved) => {
            let (m, e, zero) = (reading.value, reading.error, T::zero());
            // Moved away from zero by its rounding: within the zero band
            // there, it is within it wherever the rounding put it.
            let outward = m.select_le_zero(m - e, m + e);
            if let Ok(on) = decide_reported(VERTEX_SECTION_SIDE, Margin::of(outward), band)
                && on.sign == Sign::Zero
            {
                return Ok(on);
            }
            // Moved toward zero, and no further: past the band there, it
            // is past it wherever the rounding put it.
            let inward = m.select_le_zero((m + e).min(zero), (m - e).max(zero));
            match decide_reported(VERTEX_SECTION_CLEAR, Margin::of(inward), band) {
                Ok(off) if off.sign != Sign::Zero => Ok(off),
                _ => Err(VertexRefusal::Unresolved(unresolved)),
            }
        }
    }
}

/// The face `he`'s sector is a corner of: the face of its mate's loop,
/// as [`crate::sector_face::resolve`] reads it.
fn sector_face<T: Decide>(body: &Body<T>, vertex: VertexKey, he: HalfEdgeKey) -> FaceKey {
    let mate = body.proven_mate(he, crate::live::link(EntityId::Vertex(vertex), "orbit"));
    body.face_of_linked(mate.mate)
}

/// Whether `face` is read at `at`: a curved face with an implicit form
/// and a tangent plane there. A planar face's section is a line through
/// its ends; a fitted face has no implicit normal to read `ψ` from, and
/// the section the join fits in it is certified at the fit's own
/// resolution; at a cone's apex the wall has no normal, and a plane
/// through it cuts the cone along rulings or at the apex alone, which
/// the apex's own refusals answer.
fn read_kind<T: Decide>(body: &Body<T>, face: FaceKey, at: Point3<T>) -> bool {
    let f = crate::live::proven(&body.faces, face, EntityId::Face);
    let wall = body.face_surface_linked(face, f);
    matches!(
        wall,
        geom::Surface::Cylinder { .. }
            | geom::Surface::Cone { .. }
            | geom::Surface::Sphere { .. }
            | geom::Surface::Torus { .. }
    ) && {
        let g = geom_brep::implicit_gradient(wall, at);
        !(g.x.is_poison() || g.y.is_poison() || g.z.is_poison())
    }
}

/// The read faces (`read_kind`) whose sector at the vertex holds the
/// plane's trace (module docs). `orbit` and `faces` are the vertex's
/// orbit and each orbit half-edge's sector face.
fn entered_faces<T: Decide>(
    body: &Body<T>,
    (vertex, at): (VertexKey, Point3<T>),
    (orbit, faces): (&[HalfEdgeKey], &[FaceKey]),
    plane: SectionPlane<T>,
    far: &impl Fn(VertexKey) -> Result<PlaneSide, VertexRefusal>,
    band: Band,
) -> Result<Vec<FaceKey>, VertexRefusal> {
    let n = plane.normal.get();
    let entries = orbit
        .iter()
        .zip(faces)
        .map(|(&he, &face)| {
            entry_side(body, vertex, he, (n, plane.reference), far, band).map_err(|fault| {
                match fault {
                    EntryFault::Walk(e) => VertexRefusal::Walk(e),
                    EntryFault::Far(e) => e,
                    EntryFault::Undecided(diag) => VertexRefusal::Sector { face, diag },
                }
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    let mut entered = Vec::new();
    for (i, (&he, &face)) in orbit.iter().zip(faces).enumerate() {
        if !read_kind(body, face, at) || entered.contains(&face) {
            continue;
        }
        let next = (i + 1) % orbit.len();
        let ((dir_a, a), (dir_b, b)) = (entries[i], entries[next]);
        let holds = match (a, b) {
            (PlaneSide::Above, PlaneSide::Below) | (PlaneSide::Below, PlaneSide::Above) => true,
            (PlaneSide::Above, PlaneSide::Above) | (PlaneSide::Below, PlaneSide::Below) => {
                match wide_bisector(body, vertex, (he, orbit[next]), (dir_a, dir_b), band)? {
                    Some((bisector, arm)) => opposes(
                        bisector_side(bisector, arm, n, band)
                            .map_err(|diag| VertexRefusal::Sector { face, diag })?,
                        a,
                    ),
                    None => false,
                }
            }
            _ => false,
        };
        if holds {
            entered.push(face);
        }
    }
    Ok(entered)
}

/// Whether `a` is the strict side opposite `b`.
fn opposes(a: PlaneSide, b: PlaneSide) -> bool {
    matches!(
        (a, b),
        (PlaneSide::Above, PlaneSide::Below) | (PlaneSide::Below, PlaneSide::Above)
    )
}

/// The subdivision direction and arm of the sector after `he` when it
/// is wide ([`sector_shape`], as the walk reads it); `None` for a
/// convex one.
fn wide_bisector<T: Decide>(
    body: &Body<T>,
    vertex: VertexKey,
    (he, next): (HalfEdgeKey, HalfEdgeKey),
    (dir_a, dir_b): (Vec3<T>, Vec3<T>),
    band: Band,
) -> Result<Option<(Vec3<T>, T)>, VertexRefusal> {
    let resolved = crate::sector_face::resolve(body, vertex, he).map_err(|e| match e {
        crate::sector_face::SectorFaceError::Unsupported { face, kind } => {
            VertexRefusal::Walk(SplitReduceError::CurvedBooleanUnsupported { face, kind })
        }
    })?;
    let face = resolved.face;
    let SectorShape { arm, bisector, .. } =
        sector_shape(dir_a, dir_b, resolved.normal, he == next, band).map_err(
            |fault| match fault {
                SectorFault::NonFiniteChord => {
                    VertexRefusal::Walk(SplitReduceError::NonFiniteSectorChord { vertex, face })
                }
                SectorFault::UnderflowedChord => {
                    VertexRefusal::Walk(SplitReduceError::UnderflowedSectorChord { vertex, face })
                }
                SectorFault::Rung { diag, .. } => VertexRefusal::Sector { face, diag },
            },
        )?;
    Ok(bisector.map(|b| (b, arm)))
}

/// `sin ψ` between the plane and entered `face` at `at`, with its
/// rounding; `None` where the table reads the face's section in rulings,
/// which are minted straight (module docs). A face the table cannot
/// type is read as a curve, which its section is unless it is a ruling.
fn section_sine<T: Decide>(
    body: &Body<T>,
    (vertex, at): (VertexKey, Point3<T>),
    face: FaceKey,
    plane: SectionPlane<T>,
    band: Band,
) -> Option<Rounded<T>> {
    let f = crate::live::proven(&body.faces, face, EntityId::Face);
    let wall = body.face_surface_linked(face, f).clone();
    if let Ok(Some(WallSection {
        case: SectionCase::Straight(_) | SectionCase::Tangent(_),
        ..
    })) = wall_section(body, band, plane.origin, plane.normal, face, vertex)
    {
        return None;
    }
    // The wall's unit normal, charged its gradient's evaluation and
    // normalization at a few `u` a component. Beside the offset's
    // charge over the same sine, this share decides nothing until the
    // sine is a few ulps, where the offset's has long refused.
    let g = geom_brep::implicit_gradient(&wall, at).normalize();
    let charge = T::from_f64(4.0 * geom_core::UNIT_ROUNDOFF);
    let g = [g.x, g.y, g.z].map(|c| Rounded {
        value: c,
        error: charge,
    });
    let [x, y, z] = running::cross(running::exact_vec(plane.normal.get()), g);
    Some(x.hypot(y).hypot(z))
}

/// The rounding of the offset `(at − origin)·n̂`, as the doors evaluate
/// it, its inputs exact.
fn offset_rounding<T: Decide>(at: Point3<T>, plane: SectionPlane<T>) -> T {
    let o = plane.origin;
    let d =
        [(at.x, o.x), (at.y, o.y), (at.z, o.z)].map(|(p, q)| Rounded::exact(p) - Rounded::exact(q));
    running::dot(d, running::exact_vec(plane.normal.get())).error
}
