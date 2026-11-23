//! **An interior loop, cut in.**
//!
//! The section certificate refuses a pair whose section has a component
//! inside both faces with no event on it (R-loop): the join cuts only
//! along what the crossing layer saw, so the loop would go unbuilt. On
//! the crossings path the interior-loop guard ([`super::guard_refusal`])
//! raises it, and on the no-crossings path the section pass
//! ([`super::section_extent_pass`]). Where a face of the pair is on a
//! sphere, the loop is the no-crossings path's trimmed escape in another
//! place: the sphere face is cut along a meridian of its own chart the
//! loop crosses ([`super::SphereCutIn`], [`super::apply_cut_ins`]), and
//! the pipeline re-enters once. The re-entered crossing layer meets the
//! loop where it crosses the cut, and the output stage glues the cut
//! away with the two pieces it divided. Where it leaves one, the op
//! refuses rather than answer a face that is not maximal
//! ([`super::reenter_recut`]).
//!
//! Against a plane or a sphere the loop is the carriers' one circle.
//! Against a torus, a cylinder or a cone it is the march's one loop, and
//! the cut runs along the meridian through its net's mean, which its
//! tube is certified to cross ([`super::CutSection::Loop`]). A pair with
//! no sphere face (two tori, a torus and a cylinder) takes no cut: a
//! torus, cylinder or cone face is not cut through a chart meridian here,
//! so the refusal stands.
//!
//! A refusing pair of any other kind, or of any other refusal, keeps its
//! refusal for the whole op, and so does a cut-in that cannot be made:
//! the pipeline goes on, and the refusal is raised as it would have
//! been. A decision of the cut-in's own that its band cannot read
//! escalates as itself.

use geom_core::{Band, Decide, NonzeroSign};

use super::super::section_cert::Refusal as SectionRefusal;
use super::super::section_table::SectionTable;
use super::{CutSection, PairVerdict, SphereCutIn};
use crate::body::Body;
use crate::entity::FaceKey;
use crate::props::AtRestPolicy;
use crate::{BooleanError, Operand};

/// **The cut-ins the refusing pairs ask for**, or `None` where one of
/// them is not an R-loop with a sphere face, or its loop is not read:
/// the refusal then stands. Two spheres cut both faces, so the circle
/// lands as chords on both, as the extent scan cuts a crossing pair.
///
/// # Errors
///
/// None of its own; a sphere pair's circle refuses as the extent scan's
/// does ([`super::sphere_pair_cut`]).
pub(super) fn cut_ins<T: Decide + AtRestPolicy>(
    a: &Body<T>,
    b: &Body<T>,
    pairs: &[PairVerdict],
    sections: &SectionTable<T>,
    band: Band,
) -> Result<Option<Vec<SphereCutIn<T>>>, BooleanError> {
    let mut out = Vec::new();
    for p in pairs {
        match p.verdict {
            Ok(_) => continue,
            Err(SectionRefusal::Loop) => {}
            Err(_) => return Ok(None),
        }
        let sa = a.proven_face(p.a_face).surface();
        let sb = b.proven_face(p.b_face).surface();
        let mut any = false;
        for (operand, face, x, y) in [
            (Operand::A, p.a_face, sa, sb),
            (Operand::B, p.b_face, sb, sa),
        ] {
            if !matches!(x, geom::Surface::Sphere { .. }) {
                continue;
            }
            let Some(cut) = cut_in((operand, face), x, y, sections, band)? else {
                return Ok(None);
            };
            out.push(cut);
            any = true;
        }
        if !any {
            return Ok(None);
        }
    }
    Ok((!out.is_empty()).then_some(out))
}

/// The cut-in of `operand`'s `face` on the sphere `x` across its section
/// with `y`; `None` for a plane the ball is not read crossing, or a loop
/// [`loop_cut_in`] does not read.
fn cut_in<T: Decide + AtRestPolicy>(
    (operand, face): (Operand, FaceKey),
    x: &geom::Surface<T>,
    y: &geom::Surface<T>,
    sections: &SectionTable<T>,
    band: Band,
) -> Result<Option<SphereCutIn<T>>, BooleanError> {
    let &geom::Surface::Sphere { frame, radius } = x else {
        return Ok(None);
    };
    let center = frame.origin();
    Ok(match y {
        &geom::Surface::Plane { frame: plane } => {
            let normal = plane.axis();
            match super::ball_against_plane(center, radius, plane.origin(), normal, band) {
                Ok((NonzeroSign::Positive, s)) => {
                    let (foot, rho) = super::plane_section(center, radius, normal, s);
                    Some(SphereCutIn {
                        operand,
                        face,
                        center,
                        radius,
                        section: CutSection::Circle {
                            foot,
                            rho,
                            normal,
                            u_ref: plane.u_ref(),
                        },
                    })
                }
                Ok((NonzeroSign::Negative, _)) | Err(_) => None,
            }
        }
        geom::Surface::Sphere { .. } => Some(super::sphere_pair_cut((operand, face), x, y, band)?),
        geom::Surface::Torus { .. }
        | geom::Surface::Cylinder { .. }
        | geom::Surface::Cone { .. } => {
            loop_cut_in((operand, face), (center, radius), x, y, sections, band)
        }
        _ => None,
    })
}

/// **The cut-in across the march's one loop** of the sphere `x` (centre
/// and radius) with the curved `y`: its meridian through the sum of
/// the loop's carrier's net's points about the centre, and what [`super::apply_cut_ins`] reads the
/// loop by. `None` where the section is not one marched loop with a net
/// and a tube, or its reading refuses: the certificate read the same
/// section, so the refusal it raised stands.
fn loop_cut_in<T: Decide + AtRestPolicy>(
    (operand, face): (Operand, FaceKey),
    (center, radius): (geom_core::Point3<T>, T),
    x: &geom::Surface<T>,
    y: &geom::Surface<T>,
    sections: &SectionTable<T>,
    band: Band,
) -> Option<SphereCutIn<T>> {
    let geom_brep::SectionRead::Built(section) = geom_brep::pair_section(
        x,
        y,
        sections.region_for(x, y),
        None::<geom_core::NoLane>,
        band,
    )
    .ok()?
    else {
        return None;
    };
    let carried = section.carry(T::ssi_lane(), band).ok()?;
    let [component] = carried.components() else {
        return None;
    };
    let net = component.carrier.whole_net()?.control().to_vec();
    if component.runs.is_empty() || net.is_empty() || !component.closed() {
        return None;
    }
    let toward = net
        .iter()
        .fold(geom_core::Vec3::zero(), |acc, &q| acc + (q - center));
    Some(SphereCutIn {
        operand,
        face,
        center,
        radius,
        section: CutSection::Loop {
            toward,
            net,
            pad: component.pad,
            tube: component.runs.iter().map(|r| (r.lo, r.hi)).collect(),
        },
    })
}
