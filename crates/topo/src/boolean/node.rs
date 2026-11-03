//! **The nodes of a pair's section** (C5: a branch may end at a node).
//! Two cylinder walls of one radius whose axes meet cut each other in
//! the two bisector-plane ellipses, which cross at two points where the
//! walls are tangent. Where such a node lies inside a face of each
//! operand, the section crosses itself there with no operand edge to
//! mark it: this pass reads it from the section value, as the value
//! lists it (`geom_brep::PairSection::nodes`), and mints it, a ring in
//! each wall ([`super::vtxfac::mint_node`]), before the join matches any
//! germ. Its germs are minted from the arc ends the value lists there,
//! one per end, each riding its arc's component; the arcs then join like
//! any others, on the carried lane.

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
/// walls whose boxes overlap ([`is_wall`]), has each node its section's
/// value lists placed in its two faces of the unmutated operands. A node
/// definitely outside either face is no point of the pair's section
/// there. A node inside both is minted into the working bodies `a` and
/// `b` (one ring in each wall, [`super::vtxfac::mint_node`]), with one
/// germ per arc end the value lists there, in the rays' turn, the
/// germs' order metered at the section's scale. Each ring comes back as
/// its records.
///
/// # Errors
///
/// [`BooleanError::NodeUnbuilt`] for a node on the boundary of a face
/// that holds it, or that a face's trim cannot place, unless the other
/// face holds it definitely outside; a desync for a node whose arc ends
/// are other than one along each of its four rays; the containment
/// door's refusals; [`super::vtxfac::mint_node`]'s. A pair whose reading
/// refuses is skipped here, not silently: a germ riding it refuses the
/// reading's refusal at the join's guard.
pub(super) fn mint_nodes<T: Decide + crate::props::AtRestPolicy>(
    sections: &super::section_table::SectionTable<T>,
    (a, b): (&mut Body<T>, &mut Body<T>),
    (a_operand, b_operand): (&Body<T>, &Body<T>),
    walls: &[(crate::entity::FaceKey, crate::entity::FaceKey)],
    band: Band,
    tol: Tol,
) -> Result<Vec<super::vtxfac::RingOut<T>>, BooleanError> {
    let desync = |what| BooleanError::JoinDesync { what };
    let mut out = Vec::new();
    for &(fa, fb) in walls {
        // A refused section mints no node here and pre-empts nothing: the
        // pair keeps the route its germs take, and a germ that rides it
        // reads the same refusal at the join (`join::cylinder_read`), so
        // no node of a pair the boolean builds goes unminted. The table
        // holds every pair of served faces whose boxes overlap at the
        // sweep's pad (`glue::section_door`), the boxes `walls` was read
        // at, so a pair a glue door's table does not hold is a desync; a
        // sweep-only door's table holds none and mints no node.
        let held = match sections.held((a_operand, fa), (b_operand, fb)) {
            Some(Ok(held)) => held,
            Some(Err(_)) => continue,
            None if !sections.unheld_is_desync() => continue,
            None => {
                return Err(desync(
                    "two overlapping cylinder walls the section table does not hold",
                ));
            }
        };
        if held.nodes().is_empty() {
            continue;
        }
        let listed = held
            .counted()
            .ok_or(desync("a section listing nodes counts no components"))?;
        for (n, node) in held.nodes().iter().enumerate() {
            let arcs = node_arcs(listed, n)?;
            let here = (
                placed((Operand::A, a_operand), fa, node.point, band)?,
                placed((Operand::B, b_operand), fb, node.point, band)?,
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
                node,
                arcs,
                held.lever(),
                band,
                tol,
            )?);
        }
    }
    Ok(out)
}

/// **The component leaving the node `n` along each of its rays**, in
/// their turn ([`geom_brep::SectionNode::ray_index`]): the arc ends the
/// value lists there, one per ray.
///
/// # Errors
///
/// A desync where a ray has no arc end, or two.
fn node_arcs<T: geom_core::Real>(
    listed: &[geom_brep::SectionComponent<T>],
    n: usize,
) -> Result<[usize; 4], BooleanError> {
    let desync = |what| BooleanError::JoinDesync { what };
    let mut rays = [None; 4];
    for (k, c) in listed.iter().enumerate() {
        let geom_brep::ComponentShape::Open { ends } = c.shape else {
            continue;
        };
        for end in ends.nodes.both().into_iter().flatten() {
            if end.node != n {
                continue;
            }
            let ray = geom_brep::SectionNode::<T>::ray_index(end.line, end.sense);
            if rays[ray].replace(k).is_some() {
                return Err(desync("two arcs leave a node along one ray"));
            }
        }
    }
    match rays {
        [Some(r0), Some(r1), Some(r2), Some(r3)] => Ok([r0, r1, r2, r3]),
        _ => Err(desync("a node is listed with other than four arc ends")),
    }
}

/// One bisector plane of an equal cylinder pair with meeting axes:
/// through the axes' meeting point, with a unit normal.
pub(super) type Bisector<T> = (geom_core::Point3<T>, geom_core::Vec3<T>);

/// **The bisector planes of the pair `wall`, a cylinder wall of `x`,
/// makes with `face` of `y`**, normal to `â₁ − â₂` first, then
/// `â₁ + â₂`: on either wall the other's residual is the product of the
/// two planes' distances (`reduce::factored_crossing`). `None` where the
/// pair's section, held by the operation's table, is not the bisector
/// pose, or refused.
pub(super) fn bisectors<T: Decide>(
    sections: &super::section_table::SectionTable<T>,
    (x, x_is, wall): (&Body<T>, Operand, crate::entity::FaceKey),
    (y, face): (&Body<T>, crate::entity::FaceKey),
) -> Result<Option<[Bisector<T>; 2]>, BooleanError> {
    let is_wall = |b: &Body<T>, f| is_wall(b.proven_face_surface(f));
    if !is_wall(x, wall) || !is_wall(y, face) {
        return Ok(None);
    }
    let (on_x, on_y) = ((x, wall), (y, face));
    let held = match x_is {
        Operand::A => sections.held(on_x, on_y),
        Operand::B => sections.held(on_y, on_x),
    };
    // A refused section is no bisector pair here: the pair keeps the
    // quadric's route, whose own reads then decide or refuse it, as the
    // node pass skips such a pair. Two walls an edge of one crosses the
    // other at overlap, so a glue door's table holds them: one it does
    // not is a desync. A sweep-only door's table holds none, and its
    // crossing keeps the quadric's route.
    match held {
        Some(Ok(held)) => Ok(match held.pose() {
            geom_brep::PairPose::CylinderCylinder(pose) => pose
                .bisector_planes()
                .map(|(centre, [n1, n2])| [(centre, n1), (centre, n2)]),
            _ => None,
        }),
        Some(Err(_)) => Ok(None),
        None if !sections.unheld_is_desync() => Ok(None),
        None => Err(BooleanError::JoinDesync {
            what: "two cylinder walls a crossing meets the section table does not hold",
        }),
    }
}

/// **A cylinder wall's sense at a point of it**: whether its outward
/// normal `n` there points along `radial`, the point less its foot on
/// the axis (`true`, the wall bounds a rod) or against it (a bore): the
/// pierce's reading of the fact.
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

/// **A face's sense at a node** (C5): whether its outward normal `n`
/// there points along the node's common normal `normal` (`true`) or
/// against it, the one read per face the node pass takes, levered at the
/// section's scale `arm`. The face is tangent to its partner at the node,
/// so `n` is `±normal`, and the margin `±arm`.
///
/// # Errors
///
/// [`BooleanError::ClassificationInvariant`] where the two read square,
/// which a face tangent at the node never does; an undecided margin
/// escalates [`super::Coincide::Sectors`].
pub(super) fn face_sense<T: Decide>(
    n: geom_core::Vec3<T>,
    normal: geom_core::Vec3<T>,
    arm: geom_core::Lever<T>,
    band: Band,
) -> Result<bool, BooleanError> {
    use geom_core::{Margin, Sign};
    match crate::validate::decide(
        "bool_node_face_sense",
        Margin::levered(n.dot(normal), arm),
        band,
    ) {
        Ok(Sign::Positive) => Ok(true),
        Ok(Sign::Negative) => Ok(false),
        Ok(Sign::Zero) => Err(BooleanError::ClassificationInvariant {
            what: "a face reads its outward normal square to a node's common normal",
        }),
        Err(diag) => Err(BooleanError::coincidence(
            super::Coincide::Sectors,
            super::DeclarationRead::Moot,
            diag,
        )),
    }
}

/// **The pierced wall's datum at a pierce point `p` on the bisector plane
/// `plane`** (its index in [`bisectors`]' order, the plane whose factor
/// the sweep found the pierce a root of), for what lies on the piercing
/// wall: that plane's unit
/// normal, signed to point the way the pierced face's outward normal
/// does. On the piercing wall the pierced wall's residual is `±c·d₀·d₁`
/// (`c > 0`, `+` where A pierces), so at `p`, where the branch's
/// distance `dᵢ` is zero, it grows along `±sign(dⱼ)·n̂ᵢ` across the
/// plane: a product of decided signs. `faces` are A's and B's.
///
/// `Ok(None)` keeps the tangent plane: a pair whose named plane `p` lies
/// definitely off, a sector on another wall of the piercing body than
/// the one the sweep read the pierce against (three equal walls meeting
/// at a vertex): `p`, on both of that pair's walls, lies on one of its
/// planes, and a record naming the other reads it off that one, so the
/// record says nothing of that pair.
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
    plane: usize,
    (piercing, faces): (Operand, (crate::entity::FaceKey, crate::entity::FaceKey)),
    p: geom_core::Point3<T>,
    (pierced, n_pierced): (&geom::Surface<T>, geom_brep::OutwardNormal<T>),
    band: Band,
) -> Result<Option<geom_brep::OutwardNormal<T>>, BooleanError> {
    use geom_core::{Margin, Sign};
    let (on, other) = match plane {
        0 => (planes[0], planes[1]),
        1 => (planes[1], planes[0]),
        _ => {
            return Err(BooleanError::ClassificationInvariant {
                what: "a pierce's recorded bisector plane is neither of its pair's two",
            });
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
        plane: usize,
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
        bisector_datum(planes, plane, (piercing, faces), p, (pierced, out), band())
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
                let got = datum(0, piercing, rod, p).unwrap().unwrap();
                assert!(
                    (got - n0 * (grows * sense)).norm() < 1e-12,
                    "{piercing:?} piercing a {}: got {got:?}",
                    if rod { "rod" } else { "bore" }
                );
            }
        }
    }

    /// **A pierce no plane of its pair names keeps the tangent plane**: a
    /// record naming the plane `p` lies off (the record read against
    /// another pair) reads nothing; at a node, on both planes, the datum
    /// refuses as a node, never a guessed plane.
    #[test]
    fn a_pierce_no_plane_names_keeps_the_tangent_plane() {
        let p = Point3::new(0.99_f64.sqrt(), 0.1, 0.1);
        assert!(
            datum(1, Operand::A, true, p).unwrap().is_none(),
            "a record naming the plane the pierce lies off"
        );
        let node = Point3::new(1.0, 0.0, 0.0);
        assert!(
            matches!(
                datum(0, Operand::A, true, node),
                Err(crate::boolean::BooleanError::NodeUnbuilt {
                    why: crate::boolean::NodeWhy::GermOnNode,
                    ..
                })
            ),
            "a pierce at a node, on both planes"
        );
    }

    /// **A node's germs leave along its arc ends, and its runs are B's
    /// sectors inside A's material** (C5). Two unit walls whose axes meet
    /// at three angles, in both orders, read on the value: at each node
    /// each ray has exactly one arc end, the component a germ along it
    /// rides, and the ray is the way that component's carrier leaves the
    /// node (`+C′` at its span's first end, `−C′` at its last, the
    /// derivative as the oracle). For each pair of face senses the two
    /// runs are opposite sectors whose middle, stepped `h` along B's wall,
    /// lies inside A's material (inside A's cylinder for a rod, outside it
    /// for a bore), each starting at the bound counterclockwise of its end
    /// about B's outward normal.
    #[test]
    fn a_node_mints_a_germ_at_each_arc_end_and_runs_through_bs_sectors_inside_a() {
        let unit = |v: Vec3<f64>| v / v.norm();
        let o = Point3::new(0.0, 0.0, 0.0);
        let wall = |axis: Vec3<f64>| geom::Surface::Cylinder {
            origin: o,
            axis,
            radius: 1.0,
            u_ref: unit(axis.orthonormal_basis().0),
        };
        let axis_of = |s: &geom::Surface<f64>| match s {
            geom::Surface::Cylinder { axis, .. } => unit(*axis),
            _ => unreachable!(),
        };
        for angle in [0.5_f64, 1.2, 2.3] {
            let (wa, wb) = (
                wall(Vec3::new(0.0, 0.0, 1.0)),
                wall(Vec3::new(angle.sin(), 0.0, angle.cos())),
            );
            for (label, x, y) in [("a, b", &wa, &wb), ("b, a", &wb, &wa)] {
                let label = format!("{angle} rad, {label}");
                let region = geom_brep::ExtentBall::of_sphere(o, 3.0);
                let section = match geom_brep::pair_section(x, y, Some(region), None, band()) {
                    Ok(geom_brep::SectionRead::Built(section)) => section,
                    other => panic!("{label}: {other:?}"),
                };
                let listed = section.counted().unwrap();
                assert_eq!(section.nodes().len(), 2, "{label}: two nodes");
                for (n, node) in section.nodes().iter().enumerate() {
                    let arcs = super::node_arcs(listed, n).unwrap();
                    let rays = [node.lines[0], node.lines[1], -node.lines[0], -node.lines[1]];
                    for (i, &k) in arcs.iter().enumerate() {
                        let geom_brep::ClassCarrier::Span { curve, span } = &listed[k].carrier
                        else {
                            panic!("{label}: an arc's carrier");
                        };
                        let leaves = [(span.0, 1.0), (span.1, -1.0)]
                            .into_iter()
                            .filter(|&(t, _)| (curve.eval(t) - node.point).norm() < 1e-9)
                            .map(|(t, sign)| curve.deriv(t) * sign)
                            .map(unit)
                            .collect::<Vec<_>>();
                        assert!(
                            leaves.iter().any(|d| (*d - rays[i]).norm() < 1e-9),
                            "{label}: node {n}, ray {i} leaves along arc {k}'s carrier"
                        );
                    }
                    let (ax, ay) = (axis_of(x), axis_of(y));
                    for (a_along, b_along) in
                        [(true, true), (true, false), (false, true), (false, false)]
                    {
                        let runs = super::super::vtxfac::node_runs(node, a_along, b_along);
                        assert_eq!(
                            (runs[1].0, runs[1].1),
                            ((runs[0].0 + 2) % 4, (runs[0].1 + 2) % 4),
                            "{label}: opposite sectors"
                        );
                        let n_b = if b_along { node.normal } else { -node.normal };
                        for (s0, e0) in runs {
                            let m = unit(rays[s0] + rays[e0]);
                            let h = 1e-2;
                            let q = node.point + m * h
                                - node.normal * ((1.0 - m.dot(ay).powi(2)) * h * h / 2.0);
                            let w = q - o;
                            let off_a = (w - ax * w.dot(ax)).norm() - 1.0;
                            assert!(
                                (off_a < 0.0) == a_along,
                                "{label}: node {n}, run ({s0}, {e0}), senses \
                                 ({a_along}, {b_along}): B's sector inside A's material, \
                                 {off_a:e} off A's wall"
                            );
                            assert!(
                                rays[e0].cross(rays[s0]).dot(n_b) > 0.1,
                                "{label}: node {n}, run ({s0}, {e0}): it starts \
                                 counterclockwise of its end about B's outward normal"
                            );
                        }
                    }
                }
            }
        }
    }
}
