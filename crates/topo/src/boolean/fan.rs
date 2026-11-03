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
    geom_brep::implicit::cone_offset(rho, z, sin, cos)
}

/// The cone's unit axis turned toward the nappe `like` (a ruling the face
/// leaves the apex along) lies on.
pub(super) fn nappe_axis<T: Decide>(apex: &ConeApex<T>, like: Vec3<T>) -> Vec3<T> {
    apex.axis * T::one().copysign(like.dot(apex.axis))
}

/// **A bound's side of a sector's face** at the base vertex: a plane's
/// side ([`super::sectors::sector_code`]), or a fan's — the side of
/// the face's nappe its departure ray lies on, read at the bound's reach
/// ([`nappe_offset`], `bool_fan_side`), In where that is the face's
/// material ([`ConeApex::material_inside`]).
///
/// The verdict is the bound's side NEAR THE VERTEX, as
/// [`crate::bound_side::read`]'s is: a curved bound is read at its
/// departure tangent, whatever its own bend. Against a fan that ray needs
/// no face charge either: the cone is homogeneous about its apex, so the
/// ray's offset from the nappe is linear along it, `l·offset(d̂)`, and a
/// definite sign at the reach is the sign at every distance. It has no
/// descent: a bound on the nappe to first order reads On, and nothing
/// reads it one order down.
///
/// # Errors
///
/// The plane reading's; [`BooleanError::coincidence`] on an escalation.
pub(super) fn face_side_code<T: Decide>(
    dir: Vec3<T>,
    reach: Reach<T>,
    other: &BoolSector<T>,
    band: Band,
) -> Result<SideCode, BooleanError> {
    let apex = match &other.at_vertex {
        BoolFace::Tangent(_) => {
            return super::sectors::sector_code(dir, reach, other, band);
        }
        BoolFace::Apex(apex) => apex,
    };
    let q = reach.point(apex.apex, dir);
    let offset = nappe_offset(apex, other.start, q);
    let escalate = |diag| BooleanError::coincidence(Coincide::Sectors, DeclarationRead::Moot, diag);
    let sign = decide("bool_fan_side", Margin::of(offset), band).map_err(escalate)?;
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
            let apart = match one_cone((pa, axa), (pb, axb), arm, band)? {
                Coaxial::OneCone => return Ok(Meeting::Coincident),
                Coaxial::Other => return Ok(Meeting::Rays(Vec::new())),
                Coaxial::Apart(s) => s,
            };
            let m = geom_brep::cone_cone_apex_meeting(
                (axa, pa.half_angle.cos()),
                (axb, pb.half_angle.cos()),
            );
            // `disc` levered by the arm reads the two rulings' half-angle
            // apart, `√disc`, against `√(ε/arm)`: rulings closer than that
            // read as a tangency and refuse, more than the band itself
            // would ask (`ε/arm`), which is conservative.
            match decide("bool_fans_meet", Margin::levered(m.disc, arm), band).map_err(escalate)? {
                Sign::Negative => Ok(Meeting::Rays(Vec::new())),
                Sign::Zero => Err(tangency()),
                // The rulings are read only where the axes place them to
                // the band ([`conditioned`]); a meeting at the apex alone
                // reads no ruling and needs no such gate.
                Sign::Positive => match conditioned(apart, arm, band).map_err(escalate)? {
                    Conditioned::Read => Ok(Meeting::Rays(m.rulings().to_vec())),
                    Conditioned::Unread => Err(BooleanError::VertexPairAtConeApex {
                        operand: a_op,
                        face: a.face,
                    }),
                },
            }
        }
    }
}

/// The row [`cones_share_apex`] decides under.
pub(super) const CONES_SHARE_APEX: &str = "bool_cones_share_apex";

/// **Whether two cones share their apex**: the apexes' gap
/// (`bool_cones_share_apex`, a length) decided in the band. The one home
/// of the question, which the section certificate's cone pair and both
/// of the join's cone-pair readings ask. It lives beside the fan arms
/// because a shared apex is what makes two cone faces' sectors fans of
/// one vertex: the certificate and the join ask it of the pair the
/// vertex lane reads, so the lane's module is the one all three import.
///
/// # Errors
///
/// The decision's diagnostic where it escalates.
pub(super) fn cones_share_apex<T: Decide>(
    one: Point3<T>,
    two: Point3<T>,
    band: Band,
) -> Result<bool, geom_core::Indeterminate> {
    Ok(decide(CONES_SHARE_APEX, Margin::of((one - two).norm()), band)? == Sign::Zero)
}

/// What [`one_cone`] reads of two fans at one apex.
pub(super) enum Coaxial<T> {
    /// Their axes are definitely apart, `‖a₁ × a₂‖` the given.
    Apart(T),
    /// One cone, one nappe: the same fan of rulings.
    OneCone,
    /// Coaxial, but on opposite nappes or of other apertures.
    Other,
}

/// **The one-cone ladder** for two fans at one apex, each read by its
/// nappe's axis ([`nappe_axis`], the caller's): coaxial
/// (`bool_fans_coaxial`, `‖a₁ × a₂‖` levered by `arm`), then on one nappe
/// (`bool_fan_nappe`, `a₁·a₂`), then of one aperture (`bool_fans_one_cone`).
/// The one home of the reading, which [`meeting`] and [`nappe_classes`]
/// both ask.
///
/// # Errors
///
/// [`BooleanError::coincidence`] on an escalation.
pub(super) fn one_cone<T: Decide>(
    (pa, a): (&ConeApex<T>, Vec3<T>),
    (pb, b): (&ConeApex<T>, Vec3<T>),
    arm: T,
    band: Band,
) -> Result<Coaxial<T>, BooleanError> {
    let escalate = |diag| BooleanError::coincidence(Coincide::Sectors, DeclarationRead::Moot, diag);
    let apart = a.cross(b).norm();
    if decide("bool_fans_coaxial", Margin::levered(apart, arm), band).map_err(escalate)?
        != Sign::Zero
    {
        return Ok(Coaxial::Apart(apart));
    }
    let one_nappe = decide("bool_fan_nappe", Margin::levered(a.dot(b), arm), band)
        .map_err(escalate)?
        == Sign::Positive;
    let one_angle = decide(
        "bool_fans_one_cone",
        Margin::levered(pa.half_angle - pb.half_angle, arm),
        band,
    )
    .map_err(escalate)?
        == Sign::Zero;
    Ok(if one_nappe && one_angle {
        Coaxial::OneCone
    } else {
        Coaxial::Other
    })
}

/// What [`conditioned`] reads of two fans' common rulings.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Conditioned {
    /// The axes place the rulings to the band.
    Read,
    /// Definitely not: the fans are nearly coaxial past what the
    /// formula's arithmetic can place a ruling to.
    Unread,
}

/// **Whether two fans' common rulings are read to the band**
/// (`bool_fans_meet_conditioning`). The inputs are exact; what the
/// closed form ([`geom_brep::cone_cone_apex_meeting`]) loses is its own
/// arithmetic, and the rulings' turn about the axes' plane is read off
/// `a₁ × a₂`, whose components each round by about `2u` (`u` the unit
/// roundoff) on a length `s = ‖a₁ × a₂‖`: a direction placed to `~4u/s`,
/// so a ruling at `arm` to `arm·4u/s`. Asked only where the fans cross
/// along rulings, which is all it gates.
///
/// # Errors
///
/// The decision's diagnostic where it escalates: neither a refusal of
/// the reading nor a tangency.
pub(super) fn conditioned<T: Decide>(
    s: T,
    arm: T,
    band: Band,
) -> Result<Conditioned, geom_core::Indeterminate> {
    let spread = arm * T::from_f64(4.0 * geom_core::UNIT_ROUNDOFF) / s;
    Ok(
        match decide("bool_fans_meet_conditioning", Margin::of(spread), band)? {
            Sign::Zero => Conditioned::Read,
            Sign::Positive | Sign::Negative => Conditioned::Unread,
        },
    )
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
        let geom::Curve3::Line(geom::Line { dir, .. }) = line else {
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
        let axes = (nappe_axis(apex, first.start), nappe_axis(other, s.start));
        let one = matches!(
            one_cone((apex, axes.0), (other, axes.1), arm, band)?,
            Coaxial::OneCone
        ) && other.material_inside() == apex.material_inside();
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

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod rows {
    use super::*;
    use crate::boolean::sectors::Reach;
    use crate::entity::{FaceKey, HalfEdgeKey, VertexKey};

    fn band() -> Band {
        Band::new(1e-9, 1e-8).unwrap()
    }

    /// **A whole-turn cone partner reads each bound by its nappe**
    /// ([`nappe_classes`]): against the 45° cone's fan at `(0, 1, 0)`, its
    /// nappe below, a bound down the axis is inside the nappe, one
    /// across it outside, and one along a ruling on it. With the cone's
    /// material inside, they read In, Out and On; with it outside (a
    /// conical hole), Out, In and On.
    #[test]
    fn a_bound_reads_in_where_the_partner_cones_material_is() {
        let key = |n: u64| slotmap::KeyData::from_ffi((1 << 32) | n);
        let h = core::f64::consts::FRAC_PI_4;
        let o = Point3::new(0.0, 1.0, 0.0);
        let start = Vec3::new(h.sin(), -h.cos(), 0.0);
        for sense in [true, false] {
            let apex = ConeApex {
                apex: o,
                axis: Vec3::new(0.0, 1.0, 0.0),
                half_angle: h,
                u_ref: Vec3::new(1.0, 0.0, 0.0),
                sense,
            };
            let sector = BoolSector {
                he: HalfEdgeKey::from(key(1)),
                start,
                end: apex.ruling_from(start, core::f64::consts::FRAC_PI_2),
                start_reach: Reach::straight_bisector(1.0),
                end_reach: Reach::straight_bisector(1.0),
                face: FaceKey::from(key(2)),
                at_vertex: BoolFace::Apex(apex),
                bend: crate::bound_side::FaceBend::Plane,
                arm: 1.0,
            };
            let partner = super::super::vtxfac::Partner {
                vertex: VertexKey::default(),
                at: o,
                sectors: vec![sector],
            };
            let (down, across, along) = (
                HalfEdgeKey::from(key(3)),
                HalfEdgeKey::from(key(4)),
                HalfEdgeKey::from(key(5)),
            );
            let bounds = [
                (down, o + Vec3::new(0.0, -1.0, 0.0)),
                (across, o + Vec3::new(0.0, 0.0, 1.0)),
                (along, o + start),
            ];
            let got = nappe_classes(&bounds, (Operand::A, &[partner]), None, band()).unwrap();
            let (inside, outside) = if sense {
                (SideCode::In, SideCode::Out)
            } else {
                (SideCode::Out, SideCode::In)
            };
            assert_eq!(
                got,
                vec![(down, inside), (across, outside), (along, SideCode::On)],
                "material inside {sense}"
            );
        }
    }
    /// A fan sector of the cone at `(0, 1, 0)` whose nappe axis is `down`
    /// turned, of half-angle `half`, a quarter turn wide.
    fn fan_about(down: Vec3<f64>, half: f64, face: u64) -> BoolSector<f64> {
        fan_at(down, half, face, 1.0)
    }
    /// [`fan_about`] with its bounds' reaches and arm `arm`.
    fn fan_at(down: Vec3<f64>, half: f64, face: u64, arm: f64) -> BoolSector<f64> {
        let key = |n: u64| slotmap::KeyData::from_ffi((1 << 32) | n);
        let side = down.cross(Vec3::new(0.0, 0.0, 1.0)).normalize();
        let start = down * half.cos() + side * half.sin();
        let apex = ConeApex {
            apex: Point3::new(0.0, 1.0, 0.0),
            axis: down * -1.0,
            half_angle: half,
            u_ref: side,
            sense: true,
        };
        BoolSector {
            he: HalfEdgeKey::from(key(face)),
            start,
            end: apex.ruling_from(start, core::f64::consts::FRAC_PI_2),
            start_reach: Reach::straight_bisector(arm),
            end_reach: Reach::straight_bisector(arm),
            face: FaceKey::from(key(face)),
            at_vertex: BoolFace::Apex(apex),
            bend: crate::bound_side::FaceBend::Plane,
            arm,
        }
    }

    /// **Two fans' common rulings are read only where the axes place them**
    /// ([`conditioned`], [`meeting`]): the 0.5 rad cone against itself
    /// tilted `1e-3` meets along two rulings; tilted `2e-8` (its axes
    /// definitely apart at the row's fixed `1e-9` band, `arm·4u/s` definitely past
    /// it), its rulings refuse by name, neither read nor called a
    /// tangency; and fans of apertures `0.01` apart, tilted as little,
    /// meet at the apex alone, which no ruling is read for, so the gate
    /// does not ask.
    #[test]
    fn a_fans_rulings_are_read_only_where_their_axes_place_them() {
        let down = Vec3::new(0.0, -1.0, 0.0);
        let tilted = |t: f64| Vec3::new(t.sin(), -t.cos(), 0.0);
        let meet = |a: &BoolSector<f64>, b: &BoolSector<f64>| {
            meeting((Operand::A, None, a), (Operand::B, None, b), band())
        };
        let one = fan_about(down, 0.5, 1);
        match meet(&one, &fan_about(tilted(1e-3), 0.5, 2)) {
            Ok(Meeting::Rays(rays)) => assert_eq!(rays.len(), 2),
            other => panic!("tilted 1e-3: {other:?}"),
        }
        assert!(
            matches!(
                meet(&one, &fan_about(tilted(2e-8), 0.5, 3)),
                Err(BooleanError::VertexPairAtConeApex { .. })
            ),
            "tilted 2e-8"
        );
        match meet(&one, &fan_about(tilted(2e-8), 0.51, 4)) {
            Ok(Meeting::Rays(rays)) => assert!(rays.is_empty()),
            other => panic!("apart in aperture: {other:?}"),
        }
    }

    /// **Near-coaxial fans across the ε × arm × tilt grid** ([`meeting`],
    /// [`one_cone`], [`conditioned`]): at bands `ε` = 1e-6, 1e-9 and
    /// 1e-12 (escalating at `10ε`), arms 1e-3, 1 and 1e2, tilts 1e-3 down
    /// to 1e-12. Fans of one aperture (sharing rulings) whose axes are
    /// definitely apart (`arm·sin t` past `10ε`), and whose rulings the
    /// formula places (`arm·4u/sin t` under `ε`), **build**: two rulings,
    /// no refusal. Decided coaxial (`arm·sin t` under `ε`), they are one
    /// cone's fans. Between, they read, are one cone or refuse by name
    /// (`VertexPairAtConeApex`, or a decision's escalation), never a
    /// tangency. With the axes apart, where the rulings are placed past
    /// `20ε` they refuse `VertexPairAtConeApex`, and where the spread lies
    /// in the gap (`2ε`–`5ε`) they escalate at the gate
    /// (`bool_fans_meet_conditioning`): the row pins where the gate is.
    /// Fans of apertures `0.1` apart meet at the apex alone
    /// wherever the axes are decided, and escalate (`bool_fans_coaxial`)
    /// only with `arm·sin t` in the gap.
    #[test]
    fn near_coaxial_fans_meet_or_refuse_by_name_across_the_grid() {
        let down = Vec3::new(0.0, -1.0, 0.0);
        let tilted = |t: f64| Vec3::new(t.sin(), -t.cos(), 0.0);
        let (mut built, mut refused, mut escalated) = (0, 0, 0);
        for eps in [1e-6, 1e-9, 1e-12] {
            let band = Band::new(eps, 10.0 * eps).unwrap();
            for arm in [1e-3, 1.0, 1e2] {
                let a = fan_at(down, 0.5, 1, arm);
                for k in 3..=12 {
                    let tilt = 10f64.powi(-k);
                    let what = format!("ε {eps:e}, arm {arm:e}, tilt {tilt:e}");
                    let meet = |b: &BoolSector<f64>| {
                        meeting((Operand::A, None, &a), (Operand::B, None, b), band)
                    };
                    let lever = arm * tilt.sin();
                    let spread = arm * 4.0 * geom_core::UNIT_ROUNDOFF / tilt.sin();
                    let apart = lever > 20.0 * eps;
                    let coaxial = lever < 0.5 * eps;
                    match meet(&fan_at(tilted(tilt), 0.6, 2, arm)) {
                        Ok(Meeting::Rays(rays)) => {
                            assert!(rays.is_empty(), "apertures apart, {what}")
                        }
                        Err(BooleanError::Escalated { .. }) if !apart && !coaxial => {}
                        other => panic!("apertures apart, {what}: {other:?}"),
                    }
                    let got = meet(&fan_at(tilted(tilt), 0.5, 3, arm));
                    match &got {
                        Ok(Meeting::Rays(rays)) => assert_eq!(rays.len(), 2, "shared, {what}"),
                        Ok(Meeting::Coincident) => assert!(!apart, "shared, {what}: {got:?}"),
                        Err(
                            BooleanError::VertexPairAtConeApex { .. }
                            | BooleanError::Escalated { .. },
                        ) => {}
                        other => panic!("shared, {what}: {other:?}"),
                    }
                    if apart && spread < 0.5 * eps {
                        assert!(
                            matches!(&got, Ok(Meeting::Rays(r)) if r.len() == 2),
                            "axes apart and placed, {what}: {got:?}"
                        );
                        built += 1;
                    }
                    // The gate itself: axes apart, rulings past what the
                    // formula places them to, refused by name; in the
                    // band's gap, its own escalation.
                    if apart && spread > 20.0 * eps {
                        assert!(
                            matches!(&got, Err(BooleanError::VertexPairAtConeApex { .. })),
                            "axes apart, rulings unplaced, {what}: {got:?}"
                        );
                        refused += 1;
                    }
                    if apart && spread > 2.0 * eps && spread < 5.0 * eps {
                        assert!(
                            matches!(&got, Err(BooleanError::Escalated { diag, .. })
                                if diag.predicate == Some("bool_fans_meet_conditioning")),
                            "axes apart, spread in the gap, {what}: {got:?}"
                        );
                        escalated += 1;
                    }
                    if coaxial {
                        assert!(
                            matches!(&got, Ok(Meeting::Coincident)),
                            "coaxial, {what}: {got:?}"
                        );
                    }
                }
            }
        }
        assert!(built >= 19, "{built} apart pairs built");
        assert!(refused > 0, "no grid point reaches the gate's refusal");
        assert!(escalated > 0, "no grid point reaches the gate's escalation");
    }
}
