//! **One home for the wrap scan**: does a set of faces on one curved
//! surface key cover a whole chart coordinate — or the whole chart — so
//! that a trim has nothing left to do in it?
//!
//! Every curved containment arm asks it, and they ask the SAME question
//! with one rule. Walk every boundary edge of every member: an edge
//! mated to a member is a seam, interior to the union's closure, and
//! bounds nothing. What is left — the UNMATED boundary — decides:
//!
//! - **the closed group** ([`surface_group`]): nothing may be left. The
//!   union then has no boundary against any other surface, and a closed
//!   subsurface without boundary of a connected compact surface (a
//!   sphere, a ring torus) IS that surface;
//! - **the wrapped coordinate** ([`wrap_rims_within`], [`wrap_rims`] for
//!   one face): what is left must be circles that are iso-lines of the
//!   OTHER coordinate ([`WrapRims`]) — coaxial rims for the azimuth of a
//!   cylinder, cone or sphere and the major angle of a torus, meridians
//!   for a torus's minor angle. They bound the union there and never in
//!   the wrapped coordinate, so the union attains every value of it over
//!   the range they bound. An unmated edge of any other kind bounds the
//!   union in the wrapped coordinate, and then it does not wrap however
//!   wide the window its walk reads: a band merged with half of the
//!   band beside it reads a whole turn and holds only half of it there.
//!
//! The members are the CALLER's choice — its scope's wearers of the key
//! ([`ChartGroups`]), or one face — never the body's: a chart is
//! body-wide, and a wearer outside the scope bounds material the caller
//! is not asking about.
//!
//! # Structure first, margins last
//!
//! The walk ([`unmated_boundary`]) is exact-`f64` structure selection
//! (C6): rings, arena keys, mate adjacency and curve VARIANTS only. It
//! has no in-band twin and does not move with ε, and a union it rules
//! out answers `None` without deciding anything. Only the wrapped
//! coordinate's rim test is a margin (`bool_wrap_rim`), and it is asked
//! only of a union whose every unmated edge is already a circle — so a
//! windowed face, bounded by a meridian or section mated to a
//! neighbour, never escalates here.
//!
//! Rings take a face out of every class: a ringed face is a trimmed one,
//! and a ring is a boundary the scan cannot see.

use crate::body::Body;
use crate::chart_groups::ChartGroups;
use crate::entity::{FaceKey, LoopBoundary};
use crate::validate::decide;
use geom_core::{Band, Decide, Indeterminate, Margin, Point3, Sign, Vec3};

use super::rim_wedge::Rim;
use super::solid_contain::PointInSolidError;

/// The distance of `c` from the axis line `(origin, axis)`.
pub(super) fn off_axis<T: Decide>(origin: Point3<T>, axis: Vec3<T>, c: Point3<T>) -> T {
    let e = c - origin;
    (e - axis * e.dot(axis)).norm()
}

/// **The coaxial test, in one home**: the two margins that are zero
/// exactly when the circle `rim` is coaxial with the axis line
/// `(origin, axis)` — the sine between the axes, levered by the circle's
/// own radius (the distance the circle moves per radian of tilt), and
/// the centre's distance off the axis in metres.
///
/// Every site that asks it decides the margins under its own predicate
/// name, because the names are its door's rows: [`WrapRims::Coaxial`]
/// (`bool_wrap_rim`), the cylinder wall's rim class
/// (`bool_wall_iso_rim`) and the sphere trim's latitude-rim class
/// (`bool_sphere_iso_rim`).
pub(super) fn coaxial_margins<T: Decide>(
    origin: Point3<T>,
    axis: Vec3<T>,
    rim: &Rim<T>,
) -> [Margin<T>; 2] {
    [
        Margin::levered(rim.axis.cross(axis).norm(), rim.radius),
        Margin::of(off_axis(origin, axis, rim.center)),
    ]
}

/// Which unmated boundary circles bound a union in the coordinate it
/// does NOT wrap, and so may stay unmated when it wraps the other one.
#[derive(Clone, Copy, Debug)]
pub(super) enum WrapRims<T: geom_core::Real> {
    /// Circles COAXIAL with the carrier's axis `(origin, axis)`
    /// ([`coaxial_margins`]). On a sphere a meridian is a circle too,
    /// centred on the axis but not coaxial, so the test is both halves.
    Coaxial { origin: Point3<T>, axis: Vec3<T> },
    /// Circles whose plane CONTAINS the axis direction: a torus's
    /// meridians, the iso-lines of its major angle.
    Meridians { axis: Vec3<T> },
}

impl<T: Decide> WrapRims<T> {
    fn holds(self, rim: &Rim<T>, band: Band) -> Result<bool, Indeterminate> {
        let margins = match self {
            Self::Coaxial { origin, axis } => coaxial_margins(origin, axis, rim).map(Some),
            Self::Meridians { axis } => {
                [Some(Margin::levered(rim.axis.dot(axis), rim.radius)), None]
            }
        };
        for margin in margins.into_iter().flatten() {
            if decide("bool_wrap_rim", margin, band)? != Sign::Zero {
                return Ok(false);
            }
        }
        Ok(true)
    }
}

/// Each unmated boundary circle, with the member it bounds.
type Unmated<T> = Vec<(FaceKey, Rim<T>)>;

/// The walk: every member's unmated boundary edges, each of which must
/// be a certified circle. `None` when a member has a ring, an outline
/// that is not a cycle, or an unmated edge of any other kind.
///
/// # Errors
///
/// The member whose walk lost an arena entity.
fn unmated_boundary<T: Decide>(
    body: &Body<T>,
    members: &[FaceKey],
) -> Result<Option<Unmated<T>>, FaceKey> {
    let mut found = Vec::new();
    for &member in members {
        let f = body.get_face(member).ok_or(member)?;
        if !f.rings.is_empty() {
            return Ok(None);
        }
        let Some(LoopBoundary::Cycle { first }) = body.get_loop(f.outer).map(|l| l.boundary) else {
            return Ok(None);
        };
        for he in body.loop_cycle(first).ok_or(member)? {
            let neighbour = body
                .mate(he)
                .and_then(|m| body.face_of_half_edge(m))
                .ok_or(member)?;
            if members.contains(&neighbour) {
                continue;
            }
            let edge = body.get_half_edge(he).ok_or(member)?.edge;
            let Some(geom::Curve3::Circle {
                center,
                axis,
                radius,
                u_ref,
            }) = body
                .get_edge(edge)
                .and_then(|e| body.get_curve_geom(e.curve))
                .and_then(crate::null::CurveGeom::certified)
                .map(|c| c.carrier().clone())
            else {
                return Ok(None);
            };
            found.push((
                member,
                Rim {
                    center,
                    axis,
                    radius,
                    u_ref,
                },
            ));
        }
    }
    Ok(Some(found))
}

/// **The closed group**: is the set of `charts`' wearers of `face`'s
/// surface key closed against the rest of the body? `Some(representative)`
/// — the first member in the scope's order, the member the arms act for
/// — when no member's boundary is left unmated.
///
/// Acting for one member is not an optimization. The group's members
/// share the trim their closure certifies, so a per-face arm would fold
/// the same root once per member and tie the closest-hit rule into a
/// permanent graze.
///
/// * `Ok(None)` — definitely NOT closed. The caller falls through to its
///   per-face class.
/// * `Err(face)` — an arena claim about a BROKEN body, naming the face
///   the walk lost an entity on. A caller that reports corruption raises
///   it; one whose class simply does not apply to a body it cannot walk
///   maps it to `None`.
///
/// # Errors
///
/// The face key whose walk could not be completed, or `face` itself
/// when the scope does not hold it.
pub(super) fn surface_group<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    charts: &ChartGroups,
) -> Result<Option<FaceKey>, FaceKey> {
    let members = scope_members(body, face, charts)?;
    Ok(unmated_boundary(body, members)?
        .filter(Vec::is_empty)
        .map(|_| members[0]))
}

/// `charts`' wearers of `face`'s surface key, `face` among them.
///
/// # Errors
///
/// `face` when it does not resolve or the scope does not hold it.
pub(super) fn scope_members<'c, T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    charts: &'c ChartGroups,
) -> Result<&'c [FaceKey], FaceKey> {
    let surface = body.get_face(face).ok_or(face)?.surface;
    let members = charts.of(surface);
    if !members.contains(&face) {
        return Err(face);
    }
    Ok(members)
}

/// **Do `members` together wrap one coordinate of their chart?** The
/// unmated boundary is all circles `rims` admits (see the module
/// header). `Some(rims)`, those circles, or `None`.
///
/// # Errors
///
/// [`PointInSolidError::CorruptFace`] for an unwalkable member;
/// [`PointInSolidError::Escalated`] for an in-band rim margin.
pub(super) fn wrap_rims_within<T: Decide>(
    body: &Body<T>,
    members: &[FaceKey],
    rims: WrapRims<T>,
    band: Band,
) -> Result<Option<Vec<Rim<T>>>, PointInSolidError> {
    let Some(found) =
        unmated_boundary(body, members).map_err(|face| PointInSolidError::CorruptFace { face })?
    else {
        return Ok(None);
    };
    for (member, rim) in &found {
        if !rims
            .holds(rim, band)
            .map_err(|diag| PointInSolidError::Escalated {
                face: *member,
                diag,
            })?
        {
            return Ok(None);
        }
    }
    Ok(Some(found.into_iter().map(|(_, rim)| rim).collect()))
}

/// **Does `face` ALONE wrap one coordinate of its chart?** The one
/// notion of "this face's window is a full period":
/// [`wrap_rims_within`] for the face alone.
///
/// # Errors
///
/// As [`wrap_rims_within`].
pub(super) fn wrap_rims<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    rims: WrapRims<T>,
    band: Band,
) -> Result<Option<Vec<Rim<T>>>, PointInSolidError> {
    wrap_rims_within(body, &[face], rims, band)
}
