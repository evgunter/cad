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
    // A reading that refuses (a margin of the family in band) is folded
    // into `None`: the pair keeps the quadric's route, whose own reads
    // then decide or refuse it, as the node pass skips such a pair.
    .ok()
    .flatten()?;
    Some(section.planes)
}

/// **A cylinder wall's sense at a point of it**: whether its outward
/// normal `n` there points along `radial`, the point less its foot on
/// the axis (`true`, the wall bounds a rod) or against it (a bore). The
/// one reading of the fact for the node pass and the pierce.
///
/// # Errors
///
/// [`BooleanError::ClassificationInvariant`] where the two read square:
/// a wall of radius `r` read against its own radius is `±r`, which the
/// section table decided definite on this band before it served the
/// pair (`geom`'s `ellipse_minor_positive`, the minor semi-axis `r`). An
/// undecided margin escalates [`super::Coincide::Sectors`].
pub(super) fn wall_sense<T: Decide>(
    n: geom_core::Vec3<T>,
    radial: geom_core::Vec3<T>,
    band: Band,
) -> Result<bool, BooleanError> {
    use geom_core::{Margin, Sign};
    match crate::validate::decide("bool_node_wall_sense", Margin::of(n.dot(radial)), band) {
        Ok(Sign::Positive) => Ok(true),
        Ok(Sign::Negative) => Ok(false),
        Ok(Sign::Zero) => Err(BooleanError::ClassificationInvariant {
            what: "a wall reads its outward normal square to its own radius",
        }),
        Err(diag) => Err(BooleanError::coincidence(
            super::Coincide::Sectors,
            super::DeclarationRead::Moot,
            diag,
        )),
    }
}

/// **The pierced wall's datum at a pierce point `p` on the bisector plane
/// `branch` names**, for what lies on the piercing wall: that plane's unit
/// normal, signed to point the way the pierced face's outward normal
/// does. On the piercing wall the pierced wall's residual is `±c·d₀·d₁`
/// (`c > 0`, `+` where A pierces), so at `p`, where the branch's
/// distance `dᵢ` is zero, it grows along `±sign(dⱼ)·n̂ᵢ` across the
/// plane: a product of decided signs. `faces` are A's and B's.
///
/// `Ok(None)` keeps the tangent plane: a branch naming no plane, or a
/// pair whose named plane `p` lies definitely off. The second is a sector
/// on another wall of the piercing body than the one the sweep read the
/// pierce against (three equal walls meeting at a vertex): `p`, on both
/// of that pair's walls, lies on one of its planes, and a branch naming
/// the other reads it off that one, so the record says nothing of that
/// pair.
///
/// # Errors
///
/// - [`BooleanError::NodeUnbuilt`] (`GermOnNode`) where `p` lies on both
///   planes, a node of the pair;
/// - either plane distance in band escalates
///   [`super::Coincide::VertexOnCurvedFace`], the sweep's own decision of it;
/// - the pierced wall's sense, as [`wall_sense`] refuses it;
/// - [`BooleanError::ClassificationInvariant`] for a pierced surface
///   that is no cylinder, which [`bisectors`] never serves.
#[allow(clippy::too_many_arguments)]
pub(super) fn bisector_datum<T: Decide>(
    planes: [Bisector<T>; 2],
    branch: super::Branch,
    (piercing, faces): (Operand, (crate::entity::FaceKey, crate::entity::FaceKey)),
    p: geom_core::Point3<T>,
    (pierced, n_pierced): (&geom::Surface<T>, geom_brep::OutwardNormal<T>),
    band: Band,
) -> Result<Option<geom_brep::OutwardNormal<T>>, BooleanError> {
    use geom_core::{Margin, Sign};
    let (on, other) = match branch {
        super::Branch::Positive => (planes[0], planes[1]),
        super::Branch::Negative => (planes[1], planes[0]),
        super::Branch::Only | super::Branch::Component(_) | super::Branch::Line(_) => {
            return Ok(None);
        }
    };
    let side = |(origin, normal): Bisector<T>| {
        crate::validate::decide(
            super::reduce::factored_crossing::FACTORED_PLANE_SIDE,
            Margin::of((p - origin).dot(normal)),
            band,
        )
        .map_err(|diag| {
            BooleanError::coincidence(
                super::Coincide::VertexOnCurvedFace,
                super::DeclarationRead::Moot,
                diag,
            )
        })
    };
    if side(on)? != Sign::Zero {
        return Ok(None);
    }
    let other = match side(other)? {
        sign @ (Sign::Positive | Sign::Negative) => sign,
        Sign::Zero => {
            return Err(BooleanError::NodeUnbuilt {
                a_face: faces.0,
                b_face: faces.1,
                why: super::NodeWhy::GermOnNode,
            });
        }
    };
    let &geom::Surface::Cylinder { origin, axis, .. } = pierced else {
        return Err(BooleanError::ClassificationInvariant {
            what: "an equal cylinder pair's pierced face is no cylinder",
        });
    };
    // The pierced wall's residual grows radially, away from its axis.
    let w = p - origin;
    let radial = w - axis * (w.dot(axis) / axis.norm_squared());
    let outward = wall_sense(n_pierced.vec(), radial, band)?;
    let grows = (other == Sign::Positive) == (piercing == Operand::A);
    Ok(Some(geom_brep::OutwardNormal::from_chart(
        on.1,
        grows == outward,
    )))
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    use geom_core::{Band, Point3, Tol, Vec3};

    use super::{Operand, bisector_datum};
    use crate::boolean::Branch;
    use crate::entity::FaceKey;

    /// Two unit walls, A's about `ẑ` and B's about `ŷ`, meeting at the
    /// origin: on A's wall B's residual is `z² − y² = 2·d₀·d₁`, `d₀` the
    /// distance from the `ẑ − ŷ` plane and `d₁` from the `ẑ + ŷ` one.
    fn pair() -> ([super::Bisector<f64>; 2], [geom::Surface<f64>; 2]) {
        let unit = |v: Vec3<f64>| v / v.norm();
        let (a1, a2) = (Vec3::new(0.0, 0.0, 1.0), Vec3::new(0.0, 1.0, 0.0));
        let o = Point3::new(0.0, 0.0, 0.0);
        let wall = |axis| geom::Surface::Cylinder {
            origin: o,
            axis,
            radius: 1.0,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        (
            [(o, unit(a1 - a2)), (o, unit(a1 + a2))],
            [wall(a1), wall(a2)],
        )
    }

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    /// The datum at `p`, a point of both walls, for `piercing`'s pierce
    /// of the other operand's wall, that wall bounding a rod or a bore.
    fn datum(
        branch: Branch,
        piercing: Operand,
        rod: bool,
        p: Point3<f64>,
    ) -> Result<Option<Vec3<f64>>, crate::boolean::BooleanError> {
        let (planes, walls) = pair();
        let pierced = match piercing {
            Operand::A => &walls[1],
            Operand::B => &walls[0],
        };
        let &geom::Surface::Cylinder { axis, .. } = pierced else {
            unreachable!()
        };
        let w = p - Point3::new(0.0, 0.0, 0.0);
        let radial = w - axis * w.dot(axis);
        let out = geom_brep::OutwardNormal::from_chart(radial, rod);
        let faces = (FaceKey::default(), FaceKey::default());
        bisector_datum(planes, branch, (piercing, faces), p, (pierced, out), band())
            .map(|d| d.map(|d| d.vec()))
    }

    /// **The sign product.** At `p = (√0.99, 0.1, 0.1)`, on both walls and
    /// on the `ẑ − ŷ` plane with `d₁ > 0`, B's residual on A's wall grows
    /// along `+n̂₀ = (0, −1, 1)/√2` (`z² − y²` rises as `z` passes `y`),
    /// and A's on B's wall along `−n̂₀`. The datum points the way each
    /// pierced face's outward normal does: with the growth for a rod,
    /// against it for a bore.
    #[test]
    fn the_datum_points_as_the_pierced_residual_grows() {
        let p = Point3::new(0.99_f64.sqrt(), 0.1, 0.1);
        let n0 = Vec3::new(0.0, -1.0, 1.0) / 2.0_f64.sqrt();
        for (piercing, grows) in [(Operand::A, 1.0), (Operand::B, -1.0)] {
            for (rod, sense) in [(true, 1.0), (false, -1.0)] {
                let got = datum(Branch::Positive, piercing, rod, p).unwrap().unwrap();
                assert!(
                    (got - n0 * (grows * sense)).norm() < 1e-12,
                    "{piercing:?} piercing a {}: got {got:?}",
                    if rod { "rod" } else { "bore" }
                );
            }
        }
    }

    /// **A pierce no plane of its pair names keeps the tangent plane**: a
    /// branch naming no plane, and a branch naming the plane `p` lies off
    /// (the record read against another pair) read nothing; at a node,
    /// on both planes, the datum refuses as a node, never a guessed plane.
    #[test]
    fn a_pierce_no_plane_names_keeps_the_tangent_plane() {
        let p = Point3::new(0.99_f64.sqrt(), 0.1, 0.1);
        assert!(
            datum(Branch::Only, Operand::A, true, p).unwrap().is_none(),
            "a branch naming no plane"
        );
        assert!(
            datum(Branch::Negative, Operand::A, true, p)
                .unwrap()
                .is_none(),
            "a branch naming the plane the pierce lies off"
        );
        let node = Point3::new(1.0, 0.0, 0.0);
        assert!(
            matches!(
                datum(Branch::Positive, Operand::A, true, node),
                Err(crate::boolean::BooleanError::NodeUnbuilt {
                    why: crate::boolean::NodeWhy::GermOnNode,
                    ..
                })
            ),
            "a pierce at a node, on both planes"
        );
    }
}
