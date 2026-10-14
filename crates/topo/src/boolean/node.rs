//! **The nodes of an equal cylinder pair's section** (D2: a branch may
//! end at a node). Two cylinder walls of one radius whose axes meet cut
//! each other in the two bisector-plane ellipses, which cross at two
//! points where the walls are tangent. Where such a node lies inside a
//! face of each operand, the section crosses itself there with no
//! operand edge to mark it: this pass reads it from the section table
//! (`geom_brep::cylinder_cylinder_section`'s `nodes`) and mints it, a
//! ring in each wall ([`super::vtxfac::mint_node`]), before the join
//! matches any germ. The four arcs leaving it then join like any
//! others, each on its own ellipse's bisector plane.

use geom_core::{Band, Bounds, Decide, Tol};

use super::contain::{CurvedPlacement, FaceContainment, curved_face_placement};
use super::{BooleanError, Operand};
use crate::body::Body;

/// Where a node lies in one face, as the containment door places it.
enum Placed {
    /// Definitely outside the face: no point of the pair's section.
    Out,
    /// Strictly inside it.
    In,
    /// On the face's boundary, or where its trim gives no verdict.
    Unbuilt { on_boundary: bool },
}

/// [`Placed`] of `node` in `face` of the unmutated operand `body`, the
/// containment door's margined read ([`curved_face_placement`]).
fn placed<T: Decide>(
    (operand, body): (Operand, &Body<T>),
    face: crate::entity::FaceKey,
    node: geom_core::Point3<T>,
    band: Band,
) -> Result<Placed, BooleanError> {
    match curved_face_placement(body, face, node, band)
        .map_err(|e| super::reduce::esc(e, operand, face))?
    {
        CurvedPlacement::Trim(Some(FaceContainment::Out)) => Ok(Placed::Out),
        CurvedPlacement::Trim(Some(FaceContainment::In)) => Ok(Placed::In),
        CurvedPlacement::Trim(Some(FaceContainment::OnEdge(_) | FaceContainment::OnVertex(_))) => {
            Ok(Placed::Unbuilt { on_boundary: true })
        }
        CurvedPlacement::Trim(None) => Ok(Placed::Unbuilt { on_boundary: false }),
        CurvedPlacement::OffCarrier => Err(BooleanError::JoinDesync {
            what: "a node of an equal cylinder pair reads off a wall it lies on",
        }),
    }
}

/// **Every node of every equal cylinder pair of the two operands,
/// built**: each face pair of A's and B's cylinder walls whose boxes
/// overlap and whose section is the two bisector-plane ellipses
/// ([`super::join::crossing_cylinder_section`]) has its two nodes placed
/// in its two faces of the unmutated operands. A node definitely outside
/// either face is no point of the pair's section there. A node inside
/// both is minted into the working bodies `a` and `b` (one ring in each
/// wall, [`super::vtxfac::mint_node`]); its arcs' tangents are the
/// ellipses' at the node, and the germs' order is metered at the walls'
/// radius. Each ring comes back as its records.
///
/// # Errors
///
/// [`BooleanError::NodeUnbuilt`] for a node on the boundary of a face
/// that holds it, or that a face's trim cannot place, unless the other
/// face holds it definitely outside; the containment door's refusals;
/// [`super::vtxfac::mint_node`]'s.
pub(super) fn mint_nodes<T: Decide + Bounds + crate::props::AtRestPolicy>(
    (a, b): (&mut Body<T>, &mut Body<T>),
    (a_operand, b_operand): (&Body<T>, &Body<T>),
    band: Band,
    tol: Tol,
) -> Result<Vec<super::vtxfac::RingOut<T>>, BooleanError> {
    let wall = |s: &geom::Surface<T>| matches!(s, geom::Surface::Cylinder { .. });
    let mut out = Vec::new();
    for (fa, fb) in super::glue::overlapping_pairs(a_operand, b_operand, wall, band)? {
        let Some(section) =
            super::join::crossing_cylinder_section((a_operand, fa), (b_operand, fb), band)
        else {
            continue;
        };
        let radius = match a_operand
            .get_face(fa)
            .and_then(|f| a_operand.get_surface(f.surface))
        {
            Some(&geom::Surface::Cylinder { radius, .. }) => radius.abs(),
            _ => {
                return Err(BooleanError::JoinDesync {
                    what: "a crossing cylinder pair's A face is no cylinder",
                });
            }
        };
        // The first node is e₁ at π/2 and e₂ at −π/2, the second the
        // other way round (the table's `nodes`).
        let half = T::from_f64(core::f64::consts::FRAC_PI_2);
        let params = [(half, -half), (-half, half)];
        for (node, (t1, t2)) in section.nodes.into_iter().zip(params) {
            let here = (
                placed((Operand::A, a_operand), fa, node, band)?,
                placed((Operand::B, b_operand), fb, node, band)?,
            );
            let unbuilt = |operand, face, on_boundary| BooleanError::NodeUnbuilt {
                operand,
                face,
                on_boundary,
            };
            match here {
                (Placed::Out, _) | (_, Placed::Out) => continue,
                (Placed::In, Placed::In) => {}
                (Placed::Unbuilt { on_boundary }, _) => {
                    return Err(unbuilt(Operand::A, fa, on_boundary));
                }
                (_, Placed::Unbuilt { on_boundary }) => {
                    return Err(unbuilt(Operand::B, fb, on_boundary));
                }
            }
            let unit = |v: geom_core::Vec3<T>| v / v.norm();
            let tangents = [unit(section.e1.deriv(t1)), unit(section.e2.deriv(t2))];
            out.push(super::vtxfac::mint_node(
                (&mut *a, &mut *b),
                (fa, fb),
                node,
                tangents,
                section.axes,
                radius,
                band,
                tol,
            )?);
        }
    }
    Ok(out)
}
