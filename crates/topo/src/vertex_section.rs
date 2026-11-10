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
//! star's reading of the vertex's bounds and sectors ([`crate::star`],
//! each bound's side as [`crate::splitting::bound_side`] reads it): a
//! face's corner holds the plane's trace where its two edges leave on
//! opposite sides of the plane, or where a divider inside it leaves on
//! the side its edges do not. A sector with a bound
//! in the plane holds the trace along that edge, which ends at the
//! vertex. Of the faces a section enters, a planar one is cut along a
//! line the join mints through its ends, and a cylinder or cone the
//! table (`chord_join::wall_section`) reads in rulings is cut straight
//! too; a ruling's own distance from the vertex is read where its
//! crossing is placed, not here. Every other entered face is read.
//!
//! The reading divides by `sin ψ`, so its own rounding is charged
//! through the division ([`geom_core::running::Rounded`]). It is the
//! input's reading, with its lever, only where that charge is within
//! `ε/K`. Past it, a verdict the charge cannot move still stands; the
//! rest keeps the offset's own verdict, so the join builds or ends at its
//! own gate (Ev's decision on PR 4812: a reading within the kernel's
//! own resolution refuses nothing that builds).

use geom_brep::ReferenceNormal;
use geom_core::running::Rounded;
use geom_core::{Band, Decide, Decided, Indeterminate, Margin, Point3, Sign, UnitVec3, Vec3};

use crate::body::Body;
use crate::chord_join::{SectionCase, WallSection, wall_section};
use crate::entity::{FaceKey, VertexKey};
use crate::sector_face::SectorFaceError;
use crate::sector_shape::SlitTips;
use crate::splitting::{EntryFault, PlaneSide, SectorReading, SplitReduceError, bound_side};
use crate::star::{Datum, Star, StarFault};
use crate::validate::decide_reported;

/// K name: a vertex ON a plane read against the sections the plane cuts
/// into the faces it bounds — its offset over the sine of the shallowest
/// dihedral between the plane and such a face (metres).
pub(crate) const VERTEX_SECTION_SIDE: &str = "vertex_section_side";

/// K name: that reading's own rounding, `K` times over, against the
/// zero band (metres): Zero admits the reading as the input's.
pub(crate) const VERTEX_SECTION_RESOLUTION: &str = "vertex_section_resolution";

/// K name: that reading moved toward zero by its own rounding, where the
/// rounding is past `ε/K` (metres): only a definite side stands.
pub(crate) const VERTEX_SECTION_CLEAR: &str = "vertex_section_clear";

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

/// A door's signed offset of the vertex from the plane, as it evaluated
/// it ([`geom::plane_offset_rounded`]), and the normal it
/// evaluated it with.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Offset<T: geom_core::Real> {
    pub value: Rounded<T>,
    pub normal: Vec3<T>,
}

/// Why a vertex has no verdict on a plane, by the decision that
/// refused it.
#[derive(Debug)]
pub(crate) enum VertexRefusal {
    /// Its offset from the plane, or a far vertex's, lies in the band,
    /// under the door's own name: the input's.
    Offset(Indeterminate),
    /// Its admitted distance from a section it would end lies in the
    /// band ([`VERTEX_SECTION_SIDE`]): the input's.
    Section(Indeterminate),
    /// The walk refused the vertex's neighbourhood: a sector reading in
    /// the band, or a degenerate one.
    Walk(SplitReduceError),
    /// The walk could not resolve a sector of the vertex: a cone face's
    /// apex undecided.
    Sector(SectorFaceError),
    /// The door could not read its plane into the section table; the
    /// clause says why.
    Plane(&'static str),
}

/// Decides `vertex` of `body`, whose slit tips at rest are `tips`, at
/// point `at` and `offset` from `plane`, ON the plane, above it or
/// below it.
///
/// The offset is decided first, under the door's own `name`, as it
/// always was: an in-band offset escalates and a definite one is the
/// verdict. A Zero offset is then read against the sections the plane
/// ends at the vertex ([`read_sections`]). `plane` is read only for a
/// vertex whose offset reads Zero and that bounds a read face; `far` is
/// a far vertex's side, for the walk's line edges.
///
/// # Errors
///
/// [`VertexRefusal`].
pub(crate) fn decide_vertex_on_plane<T: Decide>(
    name: &'static str,
    (body, tips): (&Body<T>, &SlitTips),
    (vertex, at): (VertexKey, Point3<T>),
    plane: impl FnOnce() -> Result<SectionPlane<T>, VertexRefusal>,
    (offset, far): (
        Offset<T>,
        impl Fn(VertexKey) -> Result<PlaneSide, VertexRefusal>,
    ),
    band: Band,
) -> Result<Decided, VertexRefusal> {
    let on_plane = decide_reported(name, Margin::of(offset.value.value), band)
        .map_err(VertexRefusal::Offset)?;
    if on_plane.sign != Sign::Zero {
        return Ok(on_plane);
    }
    read_sections(
        (body, tips),
        (vertex, at),
        plane,
        (offset, on_plane, far),
        band,
    )
}

/// [`decide_vertex_on_plane`] past the offset: `on_plane` is the
/// offset's Zero verdict.
///
/// The reading is decided under [`VERTEX_SECTION_SIDE`] where its own
/// rounding admits it: Zero is ON, a definite reading puts the vertex
/// on the side its offset's sign names, and an in-band one escalates
/// with the door's lever. Where its rounding is past `ε/K`, a definite
/// side the rounding cannot move stands, and otherwise `on_plane` does;
/// a sine within its own rounding of zero leaves the rounding unbounded,
/// so `on_plane` stands.
///
/// # Errors
///
/// [`VertexRefusal`].
pub(crate) fn read_sections<T: Decide>(
    (body, tips): (&Body<T>, &SlitTips),
    (vertex, at): (VertexKey, Point3<T>),
    plane: impl FnOnce() -> Result<SectionPlane<T>, VertexRefusal>,
    (offset, on_plane, far): (
        Offset<T>,
        Decided,
        impl Fn(VertexKey) -> Result<PlaneSide, VertexRefusal>,
    ),
    band: Band,
) -> Result<Decided, VertexRefusal> {
    let orbit = body.vertex_orbit_linked(vertex);
    let faces: Vec<FaceKey> = orbit
        .iter()
        .map(|&he| crate::sector_face::face_of(body, vertex, he))
        .collect();
    if !faces.iter().any(|&face| read_kind(body, face, at, band)) {
        return Ok(on_plane);
    }
    let plane = plane()?;
    let entered = entered_faces((body, tips), (vertex, at), plane, &far, band)?;
    let Some(sine) = entered
        .into_iter()
        .filter_map(|face| section_sine(body, (vertex, at), face, plane, band))
        .reduce(|a, b| Rounded::select_le_zero(b.value - a.value, b, a))
    else {
        return Ok(on_plane);
    };
    // A sine within its own rounding of zero bounds no reading: the
    // plane is tangent to the face at the vertex as far as the rounding
    // tells, and a reading past any bound is past `ε/K`.
    let Some(sine) = sine.divisor("vertex_section_sine") else {
        return Ok(on_plane);
    };
    let reading = Rounded {
        value: offset.value.value,
        error: offset.value.error + skew(offset.normal, plane, at),
    }
    .quotient(sine);
    if crate::validate::resolved(VERTEX_SECTION_RESOLUTION, reading.error, band).is_ok() {
        return decide_reported(VERTEX_SECTION_SIDE, Margin::of(reading.value), band)
            .map_err(VertexRefusal::Section);
    }
    // Moved toward zero by its rounding: past the band there, it is past
    // it wherever the rounding put it.
    match decide_reported(
        VERTEX_SECTION_CLEAR,
        Margin::of(reading.toward_zero()),
        band,
    ) {
        Ok(off) if off.sign != Sign::Zero => Ok(off),
        _ => Ok(on_plane),
    }
}

/// The offset the door evaluated with `normal`, read against the table's
/// unit normal: their difference, levered at the vertex's reach from the
/// plane's origin. The door's normal is the table's, or its negation,
/// to the rounding of the unit normal's construction.
fn skew<T: Decide>(normal: Vec3<T>, plane: SectionPlane<T>, at: Point3<T>) -> T {
    let unit = plane.normal.get();
    let (same, opposite) = ((normal - unit).norm(), (normal + unit).norm());
    same.min(opposite) * (at - plane.origin).norm()
}

/// Whether `face` is read at `at`: a curved face with an implicit form
/// and a tangent plane there. A planar face's section is a line through
/// its ends; a fitted face has no implicit normal to read `ψ` from, and
/// the section the join fits in it is certified at the fit's own
/// resolution. At a cone's apex (decided as the sector walk decides it,
/// [`crate::face_normal::cone_point`]) the wall has no tangent plane,
/// and a plane through the apex cuts the cone along rulings, which are
/// minted straight, or meets it at the apex alone. A vertex near the
/// axis but not at the apex is read; one whose apex reading escalates
/// is read, and its sector's resolution refuses it.
///
/// The split's walk refuses a sphere's or a torus's sector
/// (`neighborhood::sector_face`) because the split's gate refuses those
/// kinds wherever the plane may meet them; the Boolean serves them, so
/// this reading resolves sectors through [`crate::sector_face::resolve`],
/// which has an arm for every kind read here.
fn read_kind<T: Decide>(body: &Body<T>, face: FaceKey, at: Point3<T>, band: Band) -> bool {
    let f = body.proven_face(face);
    match *f.surface() {
        geom::Surface::Cone { apex, axis, .. } => !matches!(
            crate::face_normal::cone_point(
                geom::Line {
                    origin: apex,
                    dir: axis
                },
                at,
                band
            ),
            Ok(crate::face_normal::ConePoint::Apex)
        ),
        geom::Surface::Cylinder { .. }
        | geom::Surface::Sphere { .. }
        | geom::Surface::Torus { .. } => true,
        geom::Surface::Plane { .. }
        | geom::Surface::Spindle { .. }
        | geom::Surface::Nurbs(_)
        | geom::Surface::Approx(_) => false,
    }
}

/// The read faces ([`read_kind`]) whose corner at the vertex holds the
/// plane's trace (module docs), read off the vertex's star refined
/// against the plane ([`crate::star`]); a corner at one of `tips` is the
/// face's full turn round the slit's tip.
fn entered_faces<T: Decide>(
    (body, tips): (&Body<T>, &SlitTips),
    (vertex, at): (VertexKey, Point3<T>),
    plane: SectionPlane<T>,
    far: &impl Fn(VertexKey) -> Result<PlaneSide, VertexRefusal>,
    band: Band,
) -> Result<Vec<FaceKey>, VertexRefusal> {
    let star = Star::walk(body, vertex, tips, band)
        .and_then(|mut star| {
            star.refine(&[Datum::Plane(plane.normal.get())], band)?;
            Ok(star)
        })
        .map_err(|fault| match fault {
            StarFault::Face(e) => VertexRefusal::Sector(e),
            fault => VertexRefusal::Walk(fault.split_refusal(vertex, band)),
        })?;
    let sides = star
        .bounds
        .iter()
        .zip(&star.sectors)
        .map(|(bound, sector)| {
            bound_side(body, bound, plane.reference, far, band).map_err(|fault| match fault {
                EntryFault::Far(e) => e,
                EntryFault::Undecided(diag) => {
                    VertexRefusal::Walk(SplitReduceError::SliverSector {
                        vertex,
                        face: sector.face,
                        reading: SectorReading::Escalated(diag),
                    })
                }
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    let m = star.bounds.len();
    let mut entered = Vec::new();
    for k in (0..m).filter(|&k| star.bounds[k].real) {
        let face = star.sectors[k].face;
        if !read_kind(body, face, at, band) || entered.contains(&face) {
            continue;
        }
        // The corner runs from its own edge across its dividers to the
        // next real edge.
        let mut next = (k + 1) % m;
        let mut inner = Vec::new();
        while !star.bounds[next].real {
            inner.push(sides[next]);
            next = (next + 1) % m;
        }
        let (a, b) = (sides[k], sides[next]);
        let holds = match (a, b) {
            (PlaneSide::Above, PlaneSide::Below) | (PlaneSide::Below, PlaneSide::Above) => true,
            (PlaneSide::Above, PlaneSide::Above) | (PlaneSide::Below, PlaneSide::Below) => {
                inner.iter().any(|&side| opposes(side, a))
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
    let f = body.proven_face(face);
    let wall = f.surface().clone();
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
    // sine is a few ulps, where the offset's has long been past `ε/K`.
    let g = geom_brep::implicit_gradient(&wall, at).normalize();
    let charge = geom_core::running::direction_rounding(T::one());
    let g = [g.x, g.y, g.z].map(|c| Rounded {
        value: c,
        error: charge,
    });
    let [x, y, z] = geom_core::running::cross(geom_core::running::exact_vec(plane.normal.get()), g);
    Some(x.hypot(y).hypot(z))
}
