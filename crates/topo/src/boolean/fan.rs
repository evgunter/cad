//! **The vertex–vertex lane's fan arms**: what a sector at a cone's apex
//! is to the pair algebra of [`super::sectors::pair_search`].
//!
//! A sector at a cone's apex is a fan of the cone's rulings between its
//! bounds ([`super::sectors::BoolFace::Apex`]): an arc of a small circle
//! on the sphere of directions, where a planar sector is an arc of a
//! great circle. The pair algebra reads three things of a sector, and
//! each has a fan arm here:
//!
//! - **a direction's side of its face** ([`face_side_code`]): against a
//!   plane, the plane's side; against a fan, the side of the cone's
//!   nappe, read as the point's signed distance from it
//!   ([`nappe_offset`]), exact for a point on a line through the apex;
//! - **whether a direction lies within it** ([`fan_within_signs`]): a
//!   ruling of the face's nappe, between the bounds in the face's sweep
//!   ([`ConeApex::turn`]);
//! - **where two sectors' faces meet at the vertex** ([`meeting`]): two
//!   planes in a line; a plane and a fan in the plane's section rulings,
//!   read through the section table's cone arm, the trace the split and
//!   the pierce read ([`crate::chord_join::cone_apex_plane_section`]);
//!   two fans of one apex in the
//!   rulings both cones hold ([`geom_brep::cone_cone_apex_meeting`]), or
//!   one fan where the cones are one.
//!
//! The pair algebra is bound-coded: a sector's side of a partner face is
//! read off its bounds, which holds where the side is monotone across
//! the sector. [`super::sectors::build_sectors`] cuts every sector at
//! its partner faces' extremes ([`super::sectors::Partners`]) so that
//! it is.

use geom_core::{Band, Decide, Margin, Point3, Sign, UnitVec3, Vec3};

use super::sectors::{BoolFace, BoolSector, Reach};
use super::{BooleanError, Coincide, DeclarationRead, Operand, SideCode};
use crate::body::Body;
use crate::entity::VertexKey;
use crate::sector_face::ConeApex;
use crate::validate::decide;

/// **A point's signed distance from a cone's nappe**, read at the apex:
/// `ρ·cos α − z·sin α` for the point's offset `(z, ρ)` along and off the
/// nappe's axis, which is `|q − apex|·sin(θ − α)`, `θ` its angle off the
/// axis. Positive outside the nappe's cone, negative inside; exact for
/// every point of a line through the apex, so a ruling-shaped bound's
/// far point reads it at its reach.
pub(super) fn nappe_offset<T: Decide>(apex: &ConeApex<T>, like: Vec3<T>, q: Point3<T>) -> T {
    let a = nappe_axis(apex, like);
    let r = q - apex.apex;
    let z = r.dot(a);
    let rho = (r - a * z).norm();
    let (sin, cos) = apex.half_angle.sin_cos();
    rho * cos - z * sin
}

/// The cone's unit axis turned toward the nappe `like` (a ruling the face
/// leaves the apex along) lies on.
pub(super) fn nappe_axis<T: Decide>(apex: &ConeApex<T>, like: Vec3<T>) -> Vec3<T> {
    apex.axis * T::one().copysign(like.dot(apex.axis))
}

/// **A bound's side of a sector's face** at the base vertex: a plane's
/// side ([`super::sectors::plane_side_code`]), or a fan's — the side of
/// the face's nappe its point at its reach lies on ([`nappe_offset`],
/// `bool_fan_side`), In where that is the face's material
/// ([`ConeApex::material_inside`]). A curved bound against a fan is read
/// at its departure tangent, its bend charged as the sagitta it can
/// carry over its extent: a reading within that charge refuses rather
/// than guess.
///
/// # Errors
///
/// The plane reading's; [`BooleanError::coincidence`] on an escalation;
/// [`BooleanError::CurvedBooleanUnsupported`] on the fan's face where a
/// curved bound reads within its bend's charge.
pub(super) fn face_side_code<T: Decide>(
    dir: Vec3<T>,
    reach: Reach<T>,
    (operand, other): (Operand, &BoolSector<T>),
    band: Band,
) -> Result<SideCode, BooleanError> {
    let apex = match &other.at_vertex {
        BoolFace::Tangent(normal) => {
            return super::sectors::plane_side_code(dir, reach, *normal, band);
        }
        BoolFace::Apex(apex) => apex,
    };
    let q = reach.point(apex.apex, dir);
    let offset = nappe_offset(apex, other.start, q);
    // A curved bound's point is its tangent's: the edge lies within its
    // carrier's rise of that tangent over its extent, which a definite
    // reading must clear. A bound with no certified rise reads nothing.
    let charge = match reach {
        Reach::Conic { bend, .. } => Some(match bend.bounds {
            geom_brep::BendBounds::Planar { rise, .. } => Some(rise),
            geom_brep::BendBounds::Unbounded => None,
        }),
        Reach::Chord { .. } | Reach::Extent(_) | Reach::Bisector(_) => None,
    };
    let escalate = |diag| BooleanError::coincidence(Coincide::Sectors, DeclarationRead::Moot, diag);
    let unbuilt = || BooleanError::CurvedBooleanUnsupported {
        operand,
        face: other.face,
        kind: geom::SurfaceKind::Cone,
    };
    let sign = decide("bool_fan_side", Margin::of(offset), band).map_err(escalate)?;
    match charge {
        Some(None) => return Err(unbuilt()),
        Some(Some(rise)) if sign != Sign::Zero => {
            let clear = decide("bool_fan_side_bend", Margin::of(offset.abs() - rise), band)
                .map_err(escalate)?;
            if clear != Sign::Positive {
                return Err(unbuilt());
            }
        }
        Some(Some(_)) | None => {}
    }
    let inside = sign == Sign::Negative;
    Ok(match sign {
        Sign::Zero => SideCode::On,
        _ if inside == apex.material_inside() => SideCode::In,
        _ => SideCode::Out,
    })
}

/// **Where `dir` stands against a fan sector's bounds**, as
/// `super::sectors::within_signs` reads a planar one: the turns from the
/// start bound to `dir` and from `dir` to the end bound in the face's
/// sweep ([`ConeApex::turn`]), each levered at its bound's reach
/// (`bool_sector_within`). A direction off the face's nappe reads
/// outside: its ruling is the other nappe's, which the face does not
/// hold.
///
/// # Errors
///
/// [`BooleanError::coincidence`] on an escalation.
pub(super) fn fan_within_signs<T: Decide>(
    s: &BoolSector<T>,
    apex: &ConeApex<T>,
    (dir, lever): (Vec3<T>, impl Fn(Reach<T>) -> T),
    read: DeclarationRead,
    band: Band,
) -> Result<(Sign, Sign), BooleanError> {
    let escalate = |diag| BooleanError::coincidence(Coincide::Sectors, read, diag);
    // A direction on the cone leaves the apex `cos α` along its nappe's
    // axis, well clear of zero: the nappe it lies on is read at the
    // sector's arm.
    let along = dir.normalize().dot(nappe_axis(apex, s.start));
    if decide("bool_fan_nappe", Margin::levered(along, s.arm), band).map_err(escalate)?
        != Sign::Positive
    {
        return Ok((Sign::Negative, Sign::Negative));
    }
    let ruling = apex.onto_nappe_of(dir, s.start);
    let c1 = Margin::levered(apex.turn(s.start, ruling), lever(s.start_reach));
    let c2 = Margin::levered(apex.turn(ruling, s.end), lever(s.end_reach));
    Ok((
        decide("bool_sector_within", c1, band).map_err(escalate)?,
        decide("bool_sector_within", c2, band).map_err(escalate)?,
    ))
}

/// **Where two sectors' faces meet at their shared vertex**, as
/// [`super::sectors::pair_search`] reads them.
#[derive(Clone, Debug)]
pub(super) enum Meeting<T: geom_core::Real> {
    /// The two faces are one carrier at the vertex: two planes parallel
    /// at the shorter arm, or two fans of one cone's nappe. The pair goes
    /// to the carrier ladder as a coincidence.
    Coincident,
    /// They meet along these directions from the vertex, each a ray
    /// (none: they meet at the vertex alone). Two planes' line is both
    /// its rays.
    Rays(Vec<Vec3<T>>),
}

/// A sector's side of the pair: its operand, its body and the vertex it
/// is read at (`None` where the caller has none, which a fan refuses),
/// and the sector.
pub(super) type Side<'a, T> = (Operand, Option<(&'a Body<T>, VertexKey)>, &'a BoolSector<T>);

/// **Where the faces of `a` and `b` meet at the vertex they share**
/// ([`Meeting`]).
///
/// - Two planes: parallel at the shorter arm (`bool_faces_parallel`)
///   is a coincidence; else their line, both rays.
/// - A plane and a fan: the plane's section of the cone through its
///   apex (`chord_join::cone_apex_plane_section`, the section table's
///   cone arm, which the split's and the pierce's apex germs read), each
///   line on the fan's nappe a ray. A plane tangent to
///   the cone along a ruling is a tangency between distinct carriers,
///   refused as the pierce refuses a graze.
/// - Two fans: axes parallel at the shorter arm (`bool_fans_coaxial`)
///   with one nappe and one half-angle (`bool_fans_one_cone`) are one
///   cone, a coincidence; coaxial otherwise, the fans meet at the apex
///   alone. Else the rulings both cones hold
///   ([`geom_brep::cone_cone_apex_meeting`], `bool_fans_meet`): two, or
///   none; a touching along one ruling refuses as a tangency.
///
/// # Errors
///
/// [`BooleanError::coincidence`] on an escalation; the section table's
/// refusals; [`BooleanError::CurvedBooleanUnsupported`] on a tangency.
pub(super) fn meeting<T: Decide>(
    (a_op, a_site, a): Side<'_, T>,
    (b_op, b_site, b): Side<'_, T>,
    band: Band,
) -> Result<Meeting<T>, BooleanError> {
    let arm = a.arm.min(b.arm);
    let escalate = |diag| BooleanError::coincidence(Coincide::Sectors, DeclarationRead::Moot, diag);
    let zero = |what, m: Margin<T>| -> Result<bool, BooleanError> {
        Ok(decide(what, m, band).map_err(escalate)? == Sign::Zero)
    };
    match (&a.at_vertex, &b.at_vertex) {
        (BoolFace::Tangent(na), BoolFace::Tangent(nb)) => {
            let int = na.vec().cross(nb.vec());
            // A norm levered by a minimum of chord norms: a magnitude.
            match geom_core::k_stats::decide_magnitude(
                "bool_faces_parallel",
                Margin::levered(int.norm(), arm),
                band,
            ) {
                Ok(geom_core::k_stats::Magnitude::Zero) => Ok(Meeting::Coincident),
                Ok(geom_core::k_stats::Magnitude::Positive) => Ok(Meeting::Rays(vec![int, -int])),
                Err(diag) => Err(escalate(diag)),
            }
        }
        (BoolFace::Apex(apex), BoolFace::Tangent(normal)) => {
            plane_fan(((a_op, a_site, a), apex), normal.vec(), band)
        }
        (BoolFace::Tangent(normal), BoolFace::Apex(apex)) => {
            plane_fan(((b_op, b_site, b), apex), normal.vec(), band)
        }
        (BoolFace::Apex(pa), BoolFace::Apex(pb)) => {
            let (axa, axb) = (nappe_axis(pa, a.start), nappe_axis(pb, b.start));
            let tangency = || BooleanError::CurvedBooleanUnsupported {
                operand: a_op,
                face: a.face,
                kind: geom::SurfaceKind::Cone,
            };
            if zero(
                "bool_fans_coaxial",
                Margin::levered(axa.cross(axb).norm(), arm),
            )? {
                // Coaxial axes are along or against each other.
                let one_nappe = decide("bool_fan_nappe", Margin::levered(axa.dot(axb), arm), band)
                    .map_err(escalate)?
                    == Sign::Positive;
                let one_angle = zero(
                    "bool_fans_one_cone",
                    Margin::levered(pa.half_angle - pb.half_angle, arm),
                )?;
                return Ok(if one_nappe && one_angle {
                    Meeting::Coincident
                } else {
                    Meeting::Rays(Vec::new())
                });
            }
            let m = geom_brep::cone_cone_apex_meeting(
                (axa, pa.half_angle.cos()),
                (axb, pb.half_angle.cos()),
            );
            match decide("bool_fans_meet", Margin::levered(m.disc, arm), band).map_err(escalate)? {
                Sign::Negative => Ok(Meeting::Rays(Vec::new())),
                Sign::Zero => Err(tangency()),
                Sign::Positive => Ok(Meeting::Rays(m.rulings().to_vec())),
            }
        }
    }
}

/// [`meeting`]'s plane-and-fan arm: the plane through the apex with unit
/// outward `normal`, against the fan's face, through the section table.
fn plane_fan<T: Decide>(
    ((operand, site, fan), apex): (Side<'_, T>, &ConeApex<T>),
    normal: Vec3<T>,
    band: Band,
) -> Result<Meeting<T>, BooleanError> {
    let tangency = || BooleanError::CurvedBooleanUnsupported {
        operand,
        face: fan.face,
        kind: geom::SurfaceKind::Cone,
    };
    let (fan_body, vertex) = site.ok_or(BooleanError::ClassificationInvariant {
        what: "a fan sector's meeting with a plane is read with no body to read it in",
    })?;
    let unit = UnitVec3::levered(normal, "bool_fan_plane_normal", band, fan.arm).map_err(|_| {
        BooleanError::ClassificationInvariant {
            what: "a plane sector's outward normal is not a unit direction",
        }
    })?;
    let lines = match crate::chord_join::cone_apex_plane_section(
        fan_body,
        band,
        (apex.apex, unit),
        fan.face,
        vertex,
    ) {
        Ok(geom_brep::PlaneConeSection::ApexLinePair { l1, l2 }) => vec![l1, l2],
        Ok(geom_brep::PlaneConeSection::ApexTangentLine(_)) => return Err(tangency()),
        // The plane meets the cone at its apex alone, or (in band of the
        // apex, the section starting past it) a conic off the apex, which
        // the vertex does not meet.
        Ok(
            geom_brep::PlaneConeSection::ApexPoint(_)
            | geom_brep::PlaneConeSection::AxisNormalCircle(_)
            | geom_brep::PlaneConeSection::TiltedEllipse(_)
            | geom_brep::PlaneConeSection::OpenConic(_),
        ) => return Ok(Meeting::Rays(Vec::new())),
        Err(crate::chord_join::SplitJoinError::Escalated { diag, .. }) => {
            return Err(BooleanError::coincidence(
                Coincide::Sectors,
                DeclarationRead::Moot,
                diag,
            ));
        }
        Err(section) => return Err(BooleanError::from(section)),
    };
    let mut rays = Vec::with_capacity(2);
    for line in lines {
        let geom::Curve3::Line { dir, .. } = line else {
            unreachable!("a cone's section ruling through its apex is a line")
        };
        rays.push(apex.onto_nappe_of(dir, fan.start));
    }
    Ok(Meeting::Rays(rays))
}

/// **Whether two fans of one cone overlap**, as
/// `super::sectors::sector_overlap` reads two coplanar sectors: some
/// bound of one lies strictly within the other, or their bounds pair up
/// along one ruling each (`bool_sector_within` at the bounds' reaches),
/// the identical region either way round. Sharing a bound alone is no
/// overlap.
///
/// # Errors
///
/// [`BooleanError::coincidence`] on an escalation.
pub(super) fn fans_overlap<T: Decide>(
    a: &BoolSector<T>,
    b: &BoolSector<T>,
    band: Band,
) -> Result<bool, BooleanError> {
    let (Some(pa), Some(pb)) = (a.apex(), b.apex()) else {
        unreachable!("fans_overlap reads two fans")
    };
    let strictly = |s: &BoolSector<T>, p: &ConeApex<T>, d: Vec3<T>, r: Reach<T>| {
        let own = r.length();
        fan_within_signs(
            s,
            p,
            (d, |b: Reach<T>| b.length().min(own)),
            DeclarationRead::Moot,
            band,
        )
        .map(|(c1, c2)| c1 == Sign::Positive && c2 == Sign::Positive)
    };
    for (s, p, d, r) in [
        (a, pa, b.start, b.start_reach),
        (a, pa, b.end, b.end_reach),
        (b, pb, a.start, a.start_reach),
        (b, pb, a.end, a.end_reach),
    ] {
        if strictly(s, p, d, r)? {
            return Ok(true);
        }
    }
    let escalate = |diag| BooleanError::coincidence(Coincide::Sectors, DeclarationRead::Moot, diag);
    let along = |u: Vec3<T>, v: Vec3<T>, lever: T| -> Result<bool, BooleanError> {
        Ok(decide(
            "bool_fan_bounds_along",
            Margin::levered(pa.turn(u, v), lever),
            band,
        )
        .map_err(escalate)?
            == Sign::Zero)
    };
    let lever = a.arm.min(b.arm);
    let straight = along(a.start, b.start, lever)? && along(a.end, b.end, lever)?;
    let crossed = along(a.start, b.end, lever)? && along(a.end, b.start, lever)?;
    Ok(straight || crossed)
}

/// **A vertex's edges against a partner at a cone's apex**, where the
/// partner is one cone's whole turn round its apex: every sector of the
/// one partner is a fan of that cone's nappe with its material on one
/// side (one face, or a revolve's two meeting along a seam ruling), and
/// no face is touched. The
/// partner's material near the apex is then the inside of the face's
/// nappe, or its outside ([`ConeApex::material_inside`]), so each edge's
/// point (`bounds`, at its reach) is classed by its side of the nappe
/// (`bool_fan_side`, [`nappe_offset`]): In, Out, or On the face.
///
/// # Errors
///
/// [`BooleanError::VertexPairAtConeApex`] on the partner's face where the
/// partner holds more than that one face at the apex (a wedge's apex,
/// fans beside planes), more than one partner reads it, or a face is
/// touched as well: the gathered cone that reads such a link holds no
/// fan sector; [`BooleanError::coincidence`] on an escalation.
pub(super) fn nappe_classes<T: Decide>(
    bounds: &[(crate::entity::HalfEdgeKey, Point3<T>)],
    (operand, partners): (Operand, &[super::vtxfac::Partner<T>]),
    touched: Option<super::vtxfac::Touched<'_, T>>,
    band: Band,
) -> Result<Vec<(crate::entity::HalfEdgeKey, SideCode)>, BooleanError> {
    let fan_face = partners
        .iter()
        .flat_map(|partner| partner.sectors.iter())
        .find(|s| s.apex().is_some())
        .map(|s| s.face);
    let unbuilt = |face| BooleanError::VertexPairAtConeApex { operand, face };
    let [partner] = partners else {
        return Err(unbuilt(fan_face.unwrap_or_default()));
    };
    let Some(first) = partner.sectors.first() else {
        return Err(unbuilt(fan_face.unwrap_or_default()));
    };
    let Some(apex) = first.apex() else {
        return Err(unbuilt(fan_face.unwrap_or_default()));
    };
    if touched.is_some() {
        return Err(unbuilt(first.face));
    }
    let escalate = |diag| BooleanError::coincidence(Coincide::Sectors, DeclarationRead::Moot, diag);
    // Every sector a fan of the one cone, on its nappe, with its material
    // on one side: a revolve's two faces meeting along its seam ruling
    // are one cone's whole turn as a single face's is.
    for s in &partner.sectors {
        let Some(other) = s.apex() else {
            return Err(unbuilt(first.face));
        };
        let arm = first.arm.min(s.arm);
        let (a, b) = (nappe_axis(apex, first.start), nappe_axis(other, s.start));
        let zero = |what, m| -> Result<bool, BooleanError> {
            Ok(decide(what, m, band).map_err(escalate)? == Sign::Zero)
        };
        let one = zero("bool_fans_coaxial", Margin::levered(a.cross(b).norm(), arm))?
            && decide("bool_fan_nappe", Margin::levered(a.dot(b), arm), band).map_err(escalate)?
                == Sign::Positive
            && zero(
                "bool_fans_one_cone",
                Margin::levered(apex.half_angle - other.half_angle, arm),
            )?
            && other.material_inside() == apex.material_inside();
        if !one {
            return Err(unbuilt(s.face));
        }
    }
    bounds
        .iter()
        .map(|&(he, q)| {
            let offset = nappe_offset(apex, first.start, q);
            let class = match decide("bool_fan_side", Margin::of(offset), band).map_err(escalate)? {
                Sign::Zero => SideCode::On,
                Sign::Negative if apex.material_inside() => SideCode::In,
                Sign::Positive if !apex.material_inside() => SideCode::In,
                Sign::Negative | Sign::Positive => SideCode::Out,
            };
            Ok((he, class))
        })
        .collect()
}
