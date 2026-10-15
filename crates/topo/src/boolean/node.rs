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

use geom_core::{Band, Decide, Tol};

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
    Unbuilt(super::NodeWhy),
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
            Ok(Placed::Unbuilt(super::NodeWhy::OnBoundary(operand)))
        }
        CurvedPlacement::Trim(None) => Ok(Placed::Unbuilt(super::NodeWhy::Unplaced(operand))),
        CurvedPlacement::OffCarrier => Err(BooleanError::JoinDesync {
            what: "a node of an equal cylinder pair reads off a wall it lies on",
        }),
    }
}

/// Whether a face's surface is a cylinder wall: the faces whose
/// overlapping pairs [`mint_nodes`] reads.
pub(super) fn is_wall<T: Decide>(s: &geom::Surface<T>) -> bool {
    matches!(s, geom::Surface::Cylinder { .. })
}

/// **Every node of every equal cylinder pair of the two operands,
/// built**: each of `walls`, the face pairs of A's and B's cylinder
/// walls whose boxes overlap ([`is_wall`]), whose section is the two
/// bisector-plane ellipses ([`super::join::crossing_cylinder_reading`])
/// has its two nodes placed in its two faces of the unmutated operands.
/// A node definitely outside either face is no point of the pair's
/// section there. A node inside both is minted into the working bodies
/// `a` and `b` (one ring in each wall, [`super::vtxfac::mint_node`]); its
/// arcs leave it along `â₁ + â₂` (`e₁`) and `â₁ − â₂` (`e₂`), where the
/// ellipses cross square, and the germs' order is metered at the walls'
/// radius. Each ring comes back as its records.
///
/// # Errors
///
/// [`BooleanError::NodeUnbuilt`] for a node on the boundary of a face
/// that holds it, or that a face's trim cannot place, unless the other
/// face holds it definitely outside; the containment door's refusals;
/// [`super::vtxfac::mint_node`]'s. A pair whose reading refuses is
/// skipped here, not silently: a germ riding one of its ellipses refuses
/// the reading's refusal at the join's guard.
pub(super) fn mint_nodes<T: Decide + crate::props::AtRestPolicy>(
    (a, b): (&mut Body<T>, &mut Body<T>),
    (a_operand, b_operand): (&Body<T>, &Body<T>),
    walls: &[(crate::entity::FaceKey, crate::entity::FaceKey)],
    band: Band,
    tol: Tol,
) -> Result<Vec<super::vtxfac::RingOut<T>>, BooleanError> {
    let mut out = Vec::new();
    for &(fa, fb) in walls {
        // A refused reading mints no node here and pre-empts nothing: the
        // pair keeps the route its germs take, and a germ that rides an
        // ellipse of it reads it again before the join and refuses there
        // with this refusal (`join::crossing_germs_have_their_reading`),
        // so no node of a pair the boolean builds goes unminted.
        let Ok(Some(section)) =
            super::join::crossing_cylinder_reading((a_operand, fa), (b_operand, fb), band)
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
        let [a1, a2] = section.axes;
        let unit = |v: geom_core::Vec3<T>| v / v.norm();
        let tangents = [unit(a1 + a2), unit(a1 - a2)];
        // The axes' meeting point, midway between the nodes.
        let [n0, n1] = section.nodes;
        let centre = n0 + (n1 - n0) * T::from_f64(0.5);
        for node in section.nodes {
            let here = (
                placed((Operand::A, a_operand), fa, node, band)?,
                placed((Operand::B, b_operand), fb, node, band)?,
            );
            match here {
                (Placed::Out, _) | (_, Placed::Out) => continue,
                (Placed::In, Placed::In) => {}
                (Placed::Unbuilt(why), _) | (_, Placed::Unbuilt(why)) => {
                    return Err(BooleanError::NodeUnbuilt {
                        a_face: fa,
                        b_face: fb,
                        why,
                    });
                }
            }
            out.push(super::vtxfac::mint_node(
                (&mut *a, &mut *b),
                (fa, fb),
                (node, node - centre),
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

/// One bisector plane of an equal cylinder pair with meeting axes:
/// through the axes' meeting point, with a unit normal.
pub(super) type Bisector<T> = (geom_core::Point3<T>, geom_core::Vec3<T>);

/// **The bisector planes of the pair `wall`, a cylinder wall of `x`,
/// makes with `face` of `y`**, normal to `â₁ − â₂` first, then
/// `â₁ + â₂`: on either wall the other's residual is the product of the
/// two planes' distances (`reduce::factored_crossing`). `None` where the
/// pair is not an equal cylinder pair with meeting axes
/// ([`super::join::crossing_cylinder_reading`]), or its reading refuses.
pub(super) fn bisectors<T: Decide>(
    (x, x_is, wall): (&Body<T>, Operand, crate::entity::FaceKey),
    (y, face): (&Body<T>, crate::entity::FaceKey),
    band: Band,
) -> Option<[Bisector<T>; 2]> {
    let is_wall = |b: &Body<T>, f| {
        b.get_face(f)
            .and_then(|f| b.get_surface(f.surface))
            .is_some_and(is_wall)
    };
    if !is_wall(x, wall) || !is_wall(y, face) {
        return None;
    }
    let (on_x, on_y) = ((x, wall), (y, face));
    let section = match x_is {
        Operand::A => super::join::crossing_cylinder_reading(on_x, on_y, band),
        Operand::B => super::join::crossing_cylinder_reading(on_y, on_x, band),
    }
    .ok()
    .flatten()?;
    let [a1, a2] = section.axes;
    let [n0, n1] = section.nodes;
    let centre = n0 + (n1 - n0) * T::from_f64(0.5);
    let unit = |v: geom_core::Vec3<T>| v / v.norm();
    Some([(centre, unit(a1 - a2)), (centre, unit(a1 + a2))])
}

/// **The pierced wall's datum at a pierce point `p` that lies on one
/// bisector plane**, for what lies on the piercing wall: that plane's
/// normal, signed so that it points the way the pierced face's outward
/// normal does. On the piercing wall the pierced wall's residual is
/// `±c·d₀·d₁` (`c > 0`, `+` where A pierces), so at `p`, where `dᵢ` is
/// zero and `dⱼ` is not, it grows along `±sign(dⱼ)·n̂ᵢ`, to first order
/// in what departs from `p` along the wall. Near a node the two walls'
/// normals there stand only about the node's distance apart, and this
/// datum, read against a plane the wall crosses at a definite angle,
/// stands in for the pierced face's tangent plane.
///
/// `None` keeps the tangent plane: `p` on both planes or on neither (a
/// node, or a pierce the planes did not find), either distance in band,
/// or the pierced face's outward side undecided against its axis.
pub(super) fn bisector_datum<T: Decide>(
    planes: [Bisector<T>; 2],
    piercing: Operand,
    p: geom_core::Point3<T>,
    (pierced, n_pierced): (&geom::Surface<T>, geom_brep::OutwardNormal<T>),
    band: Band,
) -> Option<geom_brep::OutwardNormal<T>> {
    use geom_core::{Margin, Sign};
    let off = |(centre, normal): Bisector<T>| {
        crate::validate::decide("bool_vertex_face_side", Margin::of((p - centre).dot(normal)), band)
    };
    let (normal, other) = match planes.map(off) {
        [Ok(Sign::Zero), Ok(other @ (Sign::Positive | Sign::Negative))] => (planes[0].1, other),
        [Ok(other @ (Sign::Positive | Sign::Negative)), Ok(Sign::Zero)] => (planes[1].1, other),
        _ => return None,
    };
    let &geom::Surface::Cylinder { origin, axis, .. } = pierced else {
        return None;
    };
    // The pierced wall's residual grows radially, away from its axis.
    let w = p - origin;
    let radial = w - axis * (w.dot(axis) / axis.norm_squared());
    let outward = match crate::validate::decide(
        "bool_vertex_face_side",
        Margin::of(n_pierced.vec().dot(radial)),
        band,
    ) {
        Ok(Sign::Positive) => true,
        Ok(Sign::Negative) => false,
        Ok(Sign::Zero) | Err(_) => return None,
    };
    let grows = (other == Sign::Positive) == (piercing == Operand::A);
    Some(geom_brep::OutwardNormal::from_chart(normal, grows == outward))
}
