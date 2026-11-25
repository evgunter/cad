//! **An interior loop on the crossings path, cut in.**
//!
//! The interior-loop guard ([`super::guard_refusal`]) refuses a
//! pair whose section has a component inside both faces with no event on
//! it (the section certificate's R-loop): the join cuts only along what
//! the crossing layer saw, so the loop would go unbuilt. Where the pair
//! is a sphere face against a plane or another sphere, that component is
//! the carriers' one circle, and it is the no-crossings path's trimmed
//! escape in another place: each sphere face holding it is cut along the
//! meridian of its own chart through the circle ([`super::SphereCutIn`],
//! [`super::apply_cut_ins`]), and the pipeline re-enters once. The
//! re-entered crossing layer meets the circle where it crosses the cut,
//! and the output stage glues the cut away with the two pieces it
//! divided. Where it leaves one, the op refuses with the guard's
//! refusal rather than answer a face that is not maximal
//! ([`super::reenter_recut`]).
//!
//! A refusing pair of any other kind, or of any other refusal, keeps the
//! guard's refusal for the whole op, and so does a cut-in that cannot
//! be made: the pipeline goes on, and the guard raises its refusal on
//! the built body as it would have. A decision of the cut-in's own that
//! its band cannot read escalates as itself.

use geom_core::{Band, Decide};

use super::super::section_cert::Refusal as SectionRefusal;
use super::{PairVerdict, SphereCutIn};
use crate::body::Body;
use crate::entity::FaceKey;
use crate::{BooleanError, Operand};

/// **The cut-ins the guard's refusing pairs ask for**, or `None` where
/// one of them is not an R-loop of a sphere face against a plane or a
/// sphere, or its circle does not decide: the guard's refusal then
/// stands. Two spheres cut both faces, so the circle lands as chords on
/// both, as the extent scan cuts a crossing pair.
///
/// # Errors
///
/// None of its own; a sphere pair's circle refuses as the extent scan's
/// does ([`super::sphere_pair_cut`]).
pub(super) fn cut_ins<T: Decide>(
    a: &Body<T>,
    b: &Body<T>,
    pairs: &[PairVerdict],
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
            let Some(cut) = cut_in((operand, face), x, y, band)? else {
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

/// The cut-in of `operand`'s `face` on the sphere `x` through its section
/// circle with `y`; `None` for a partner neither a plane nor a sphere, or
/// a plane the ball is not read crossing.
fn cut_in<T: Decide>(
    (operand, face): (Operand, FaceKey),
    x: &geom::Surface<T>,
    y: &geom::Surface<T>,
    band: Band,
) -> Result<Option<SphereCutIn<T>>, BooleanError> {
    let &geom::Surface::Sphere { frame, radius } = x else {
        return Ok(None);
    };
    let center = frame.origin();
    Ok(match y {
        &geom::Surface::Plane { frame: plane } => {
            super::plane_cut_of_sphere((operand, face), center, radius, plane, band)
        }
        geom::Surface::Sphere { .. } => Some(super::sphere_pair_cut((operand, face), x, y, band)?),
        _ => None,
    })
}
