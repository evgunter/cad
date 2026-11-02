//! **Test fixtures**, behind the `test-support` feature (on only
//! through dev-dependency edges): a literal of each dimension, a point
//! of two lengths, the world xy frame a sketch is drawn on, the rays a
//! pick row aims, the near-tangent candidate the certified test is
//! probed with, the door's answer read as a list, the uncertified
//! determinant the review rows read the certified one against, the
//! recipe walks' pass-through classification a row holds against the
//! evaluator, and the load door's nesting limit a row holds against
//! what a save writes, the recipes more than one suite builds, and the
//! spelling a content key writes for a slot.
//!
//! One home for every reader in this crate and the crates that test
//! against it: the unit-test modules reach it as `crate::test_support`,
//! `tests/fixture` re-exports the authoring doors so a suite reads ONE
//! definition, and `viewer`'s own `test_support` re-exports them in
//! turn. The ray from arrays is `bvh::test_support::ray` — the ray IS
//! `bvh`'s — and not restated.
//!
//! Whether a door here carries an oracle is asked door by door, in each
//! door's own docs.

// Panicking is a fixture's failure mechanism (workspace lint note).
#![allow(clippy::panic, clippy::expect_used)]

use bvh::Ray;
use bvh::test_support::ray;
use geom_core::{Point3, Vec3};

use crate::{
    Dimension, DocEdit, Expr, Formula, HitTestError, LoopProgram, Node, PickHit, ProfileDoc,
    ProfileProgram, RecipeNodeId,
};

// --- literals -------------------------------------------------------

/// A length literal, in canonical metres.
///
/// # Panics
///
/// If `metres` is not finite — the refusal is `Formula::literal`'s, and a
/// row about that refusal spells the call rather than reaching here.
pub fn len(metres: f64) -> Formula {
    Formula::literal(metres, Dimension::Length).expect("a finite length")
}

/// An angle literal, in radians.
///
/// # Panics
///
/// If `radians` is not finite.
pub fn ang(radians: f64) -> Formula {
    Formula::literal(radians, Dimension::Angle).expect("a finite angle")
}

/// A written dimensionless value ([`Formula::scalar`]) — a direction
/// component, a bulge, a ratio: a variable the edit door mints, at a
/// slot's root and inside a formula alike. The exact constant is
/// [`Formula::ratio`].
///
/// # Panics
///
/// If `value` is not finite.
pub fn scl(value: f64) -> Formula {
    Formula::scalar(value).expect("a finite scalar")
}

/// Two length literals — a point in a sketch frame's own coordinates.
///
/// # Panics
///
/// If either coordinate is not finite.
pub fn len2(v: [f64; 2]) -> [Formula; 2] {
    [len(v[0]), len(v[1])]
}

// --- the stored form ------------------------------------------------

/// **A document to lower into and throw away**, for a row that asks a
/// stored form something its variables' values do not answer.
pub fn scratch(tol: geom_core::Tol) -> ProfileDoc {
    ProfileDoc::empty_derived("scratch", tol)
}

/// The stored node `node` lowers to in `doc`: what the edit door would
/// write for it, every variable it mints minted into `doc`, for a row
/// that places a node in a document by hand.
///
/// # Panics
///
/// If `node` does not lower in `doc`.
pub fn stored(doc: &mut ProfileDoc, node: &crate::AuthoredNode) -> Node<ProfileProgram> {
    crate::edit::lower_node_into(doc, node).expect("a node the document can answer lowers")
}

/// **A stored node read as given**: every slot lowered into `doc` as
/// [`stored`] lowers it, and every operand stored as the read it names
/// with no door around it — a variable by id as itself, a node or a
/// port as the id `read` hands for it — for a row about a node's own
/// shape, whose operands need name no live output.
///
/// # Panics
///
/// If a slot does not lower in `doc`, or an operand reads by name.
pub fn stored_reading(
    doc: &mut ProfileDoc,
    node: &crate::AuthoredNode,
    read: impl Fn(RecipeNodeId, u8) -> crate::VarId,
) -> Node<ProfileProgram> {
    use crate::ProfilePayload;
    fn given(
        doc: &core::cell::RefCell<&mut ProfileDoc>,
        slot: crate::OperandSlot,
        operand: &crate::Operand,
        read: &impl Fn(RecipeNodeId, u8) -> crate::VarId,
    ) -> crate::VarId {
        match operand {
            crate::Operand::Node(id) => read(*id, 0),
            crate::Operand::Output { node, port } => read(*node, *port),
            crate::Operand::Var(var) => *var,
            crate::Operand::Name(name) => {
                panic!("an operand read as given has no name: {name}")
            }
            crate::Operand::Select { body, names } => {
                let body = given(doc, slot, body, read);
                crate::edit::selection_into(&mut doc.borrow_mut(), slot, body, names)
            }
            crate::Operand::Pick { placement, of } => {
                let placement = given(doc, slot, placement, read);
                crate::edit::pick_into(&mut doc.borrow_mut(), placement, of)
            }
            crate::Operand::Pose(def) => {
                let lowered = def
                    .try_map(
                        &mut |pose_read, operand| {
                            // A selection read off geometry is the
                            // singleton of its names' entity kind, as a
                            // distance reference mints one.
                            let at = match pose_read {
                                crate::pose::PoseRead::Face | crate::pose::PoseRead::Of => {
                                    crate::OperandSlot::Measured(crate::MeasureVerb::Distance, 0)
                                }
                                _ => slot,
                            };
                            Ok::<_, core::convert::Infallible>(given(doc, at, operand, read))
                        },
                        &mut |_, f| {
                            Ok(crate::edit::lower_slot_into(&mut doc.borrow_mut(), f)
                                .expect("a pose's formula the document can answer lowers"))
                        },
                    )
                    .unwrap_or_else(|never| match never {});
                crate::edit::pose_into(&mut doc.borrow_mut(), lowered)
            }
        }
    }
    let doc = core::cell::RefCell::new(doc);
    node.try_map_slots(
        |p, f, r| ProfileProgram::lower(p, f, r),
        &mut |f| crate::edit::lower_slot_into(&mut doc.borrow_mut(), f),
        &mut |slot, operand| Ok(given(&doc, slot, operand, &read)),
    )
    .expect("a node the document can answer lowers")
}

/// **A live node as it was written**: each slot the formula its
/// variable was written as ([`crate::Doc::written`]) — an anonymous
/// variable's value or definition, a named one's reader — for a row
/// that rebuilds a document by re-inserting its nodes, minting their
/// anonymous variables afresh, as the original inserts did.
///
/// # Panics
///
/// If `node` reads an anonymous variable a written re-insert would not
/// reproduce ([`crate::Doc::written_would_not_reproduce`]): the rebuilt
/// document would not be this one. A value edited after its insert is
/// the caller's to rule out, by comparing the rebuilt ids.
pub fn as_written(doc: &ProfileDoc, node: &Node<ProfileProgram>) -> crate::AuthoredNode {
    let lost = doc.written_would_not_reproduce();
    // Written expands each anonymous definition, so a lost variable is
    // read through the definitions the node's slots reach as well.
    let mut reached: Vec<crate::VarId> = node.exprs().into_iter().copied().collect();
    let mut at = 0;
    while let Some(&var) = reached.get(at) {
        if doc.var_name(var).is_none() {
            reached.extend(doc.definition_reads(var));
        }
        at += 1;
    }
    let read: Vec<crate::VarId> = reached
        .into_iter()
        .filter(|var| lost.contains(var))
        .collect();
    assert!(
        read.is_empty(),
        "a written re-insert would not reproduce {read:?}: read twice, or toleranced"
    );
    node.written(doc)
}

/// The stored expression `formula` lowers to where no name is held.
///
/// # Panics
///
/// If `formula` reads a variable by name.
pub fn stored_expr(formula: &Formula) -> Expr {
    Expr::try_from(formula).expect("a formula with no name leaf lowers in any scope")
}

/// The stored program `program` lowers to in `doc`, its plane read as
/// given ([`stored_reading`]): a plane naming node `n` is stored as the
/// read with `n`'s own id, for a row about a program's own shape,
/// whose plane need name no live frame.
///
/// # Panics
///
/// If `program` does not lower in `doc`, or its plane reads by name.
pub fn stored_program(doc: &mut ProfileDoc, program: &ProfileProgram<Formula>) -> ProfileProgram {
    match stored_reading(doc, &Node::Profile(program.clone()), |id, _| {
        crate::VarId(id.0)
    }) {
        Node::Profile(stored) => stored,
        _ => unreachable!("a profile lowers to a profile"),
    }
}

/// The stored loop `program` lowers to in `doc` ([`stored`]).
///
/// # Panics
///
/// If `program` does not lower in `doc`.
pub fn stored_loop(doc: &mut ProfileDoc, program: &LoopProgram<Formula>) -> LoopProgram {
    program
        .try_map_slots(&mut |f| crate::edit::lower_slot_into(doc, f))
        .expect("a loop the document can answer lowers")
}

// --- placing a construction ------------------------------------------

/// **A placement of `body` into `target` under `placed`**, its whole
/// bundle written by the insert.
///
/// # Panics
///
/// If the insert door refuses it.
fn place_of(
    doc: ProfileDoc,
    body: crate::Operand,
    target: crate::Operand,
    placed: crate::Placed,
    tol: geom_core::Tol,
) -> (ProfileDoc, RecipeNodeId) {
    let applied = crate::apply(
        &doc,
        &DocEdit::place(crate::BodyRead::plain(body), target, placed.bundle()),
        tol,
    )
    .expect("a placement inserts");
    let placement = applied.record.minted.expect("an insert mints a node");
    (applied.doc, placement)
}

/// **The body a node defines, as a placement reads it**: a revolve's
/// `body` port, any other node itself.
#[must_use]
pub fn body_of(doc: &ProfileDoc, node: RecipeNodeId) -> crate::Operand {
    match doc.node(node) {
        Some(Node::Revolve { .. }) => crate::Operand::output(node, 0),
        _ => crate::Operand::Node(node),
    }
}

/// **The moves a frame states** (the helpers ruling): the slides along
/// the target space's axes to its origin, and a chain of spins about
/// `x`, then `y`, then `z` — by quarter turns written `turn/4` where its
/// axes are the target space's up to sign and order, so the copy stands
/// exactly where the frame says, else by the angles `atan2` reads off
/// its columns (`linear = Rz(γ)·Ry(β)·Rx(α)`), to their rounding. The
/// constant `0` is left to [`crate::Placed::of`].
pub fn moves_at(at: &crate::Frame) -> Vec<(crate::Coord, Formula)> {
    use crate::{Coord, TargetAxis};
    let quarter = |k: i32| -> [[i32; 3]; 3] {
        // The turn by `k` quarters about z, in columns.
        let (c, s) = match k.rem_euclid(4) {
            0 => (1, 0),
            1 => (0, 1),
            2 => (-1, 0),
            _ => (0, -1),
        };
        [[c, s, 0], [-s, c, 0], [0, 0, 1]]
    };
    // A turn about z carried onto x or y by cycling the coordinates.
    let about = |axis: usize, k: i32| -> [[i32; 3]; 3] {
        let q = quarter(k);
        let cyc = |i: usize| (i + 1 + axis) % 3;
        let mut m = [[0; 3]; 3];
        for (col, column) in q.iter().enumerate() {
            for (row, &v) in column.iter().enumerate() {
                m[cyc(col)][cyc(row)] = v;
            }
        }
        m
    };
    let mul = |a: [[i32; 3]; 3], b: [[i32; 3]; 3]| -> [[i32; 3]; 3] {
        let mut m = [[0; 3]; 3];
        for (col, out) in m.iter_mut().enumerate() {
            for (row, cell) in out.iter_mut().enumerate() {
                *cell = (0..3).map(|k| a[k][row] * b[col][k]).sum();
            }
        }
        m
    };
    let target: Vec<[f64; 3]> = at.columns.to_vec();
    let chain = (0..64).find_map(|n| {
        let (x, y, z) = (n % 4, (n / 4) % 4, n / 16);
        let r = mul(about(2, z), mul(about(1, y), about(0, x)));
        let exact = r
            .iter()
            .zip(&target)
            .all(|(col, want)| col.iter().zip(want).all(|(&v, &w)| f64::from(v) == w));
        exact.then_some([(TargetAxis::X, x), (TargetAxis::Y, y), (TargetAxis::Z, z)])
    });
    let turn = |k: i32| match k {
        1 => Formula::turn_over(4).expect("a quarter turn"),
        2 => Formula::turn_over(2).expect("a half turn"),
        _ => Formula::neg(Formula::turn_over(4).expect("a quarter turn")).expect("an angle"),
    };
    let slides = TargetAxis::ALL
        .into_iter()
        .zip(at.translation)
        .filter(|&(_, by)| by != 0.0)
        .map(|(axis, by)| (Coord::Slide(axis), len(by)));
    let spins: Vec<(Coord, Formula)> = match chain {
        Some(chain) => chain
            .into_iter()
            .filter(|&(_, k)| k != 0)
            .map(|(axis, k)| (Coord::Spin(Some(axis)), turn(k)))
            .collect(),
        None => euler_turns(&at.columns)
            .into_iter()
            .filter(|&(_, angle)| angle != 0.0)
            .map(|(axis, angle)| (Coord::Spin(Some(axis)), ang(angle)))
            .collect(),
    };
    slides.chain(spins).collect()
}

/// **The turns about `x`, then `y`, then `z` that the rotation
/// `columns` makes** (`linear = Rz(γ)·Ry(β)·Rx(α)`), read by the
/// kernel's one chain read ([`crate::mate::chain_turns`]) — for a test
/// that states a pose as a matrix.
#[must_use]
pub fn euler_turns(columns: &[[f64; 3]; 3]) -> [(crate::TargetAxis, f64); 3] {
    use crate::TargetAxis;
    let col = |j: usize| geom_core::linalg::Vec3::new(columns[j][0], columns[j][1], columns[j][2]);
    let [alpha, beta, gamma] =
        crate::mate::chain_turns(&geom_core::linalg::Mat3::from_cols(col(0), col(1), col(2)));
    [
        (TargetAxis::X, alpha),
        (TargetAxis::Y, beta),
        (TargetAxis::Z, gamma),
    ]
}

/// **A copy of `body` standing in the world at `at`** (D10: a body has
/// no location, so where it stands is its placement's): the bundle
/// with no mate whose values are the moves `at` states
/// ([`moves_at`]), the identity bundle ([`crate::stand`]) at the
/// identity. Returns the document and the placement; its copy is the
/// placement's output.
///
/// # Panics
///
/// If a door refuses.
pub fn in_world(
    doc: ProfileDoc,
    body: impl Into<crate::Operand>,
    at: &crate::Frame,
    tol: geom_core::Tol,
) -> (ProfileDoc, RecipeNodeId) {
    let body = body.into();
    let placed = crate::Placed::of(&doc, Vec::new(), moves_at(at));
    place_of(
        doc,
        body,
        crate::Operand::Var(crate::VarId::WORLD),
        placed,
        tol,
    )
}

/// **A copy of `body` placed against `target`'s geometry at `at`** —
/// `at` the copy's own frame in `target`'s own coordinates: how a construction combined with another body joins that
/// body's space. A plane mate equates the copy's first planar face by id
/// lying on its own `xy` with the target's lying on its own `xy`, slid
/// along its normal to where the copy's face stands at `at` (flipped
/// where the two face one way), and the three values put the copy at
/// `at` on the chart of that mate, each the number `at` states (a turn
/// by quarter turns written `turn/4`); where either body has no planar
/// face on its own `xy`, or `at` tilts the copy's face off the
/// target's, no mate, and the moves `at` states ([`moves_at`]). Returns
/// the document and the placement; its copy is the placement's output.
///
/// # Panics
///
/// If a door refuses.
pub fn against(
    doc: ProfileDoc,
    target: impl Into<crate::Operand>,
    body: impl Into<crate::Operand>,
    at: &crate::Frame,
    tol: geom_core::Tol,
) -> (ProfileDoc, RecipeNodeId) {
    let (target, body) = (target.into(), body.into());
    let placed = placed_against(&doc, &target, &body, at, tol);
    let space = doc
        .operand_space(&target)
        .expect("the target is a body in a space");
    place_of(doc, body, crate::Operand::Var(space), placed, tol)
}

/// **[`against`]'s bundle**, not yet written: the mate and its three
/// values, into the space `target` is in.
///
/// # Panics
///
/// As [`against`]'s.
#[must_use]
pub fn placed_against(
    doc: &ProfileDoc,
    target: &crate::Operand,
    body: &crate::Operand,
    at: &crate::Frame,
    tol: geom_core::Tol,
) -> crate::Placed {
    placed_against_in(doc, &crate::EvalOptions::default(), target, body, at, tol)
}

/// [`placed_against`] under `opts` — which reaches the parts an
/// instance copies.
///
/// # Panics
///
/// As [`against`]'s.
#[must_use]
pub fn placed_against_in(
    doc: &ProfileDoc,
    opts: &crate::EvalOptions,
    target: &crate::Operand,
    body: &crate::Operand,
    at: &crate::Frame,
    tol: geom_core::Tol,
) -> crate::Placed {
    use crate::pose::{PoseDef, PoseRole, RoleRead};
    use crate::{Coord, TargetAxis};
    let ev: crate::Evaluation<f64> =
        crate::evaluate(doc, None, &crate::CancelToken::new(), opts, tol);
    // The first planar face by id on the body's own xy: its name, the
    // sign of its outward normal's z there, and its outward normal and a
    // point of it where the body stands in its space, which is where a
    // mate reads it.
    let own_plane = |shape: &crate::Operand| {
        // A copy's body is its source's, in the source's coordinates.
        let (_, framed, table, _) =
            crate::mate::corner::read_of(doc, &ev, shape).expect("the body has a value");
        let solid = framed.body();
        solid.faces().find_map(|(face, _)| {
            let kind = topo::readback::face_carrier_kind(solid, face).ok()?;
            (kind == geom::SurfaceKind::Plane).then_some(())?;
            let pose = topo::readback::face_pose(solid, face).ok()?;
            let n = geom_brep::OutwardNormal::from_chart(pose.axis, pose.sense).vec();
            let o = pose.origin - geom_core::linalg::Point3::origin();
            // The body is in its own coordinates; where it stands in its
            // space is its frame's (D10 `[placement.frame]`).
            let (own_o, own_n) = (o, n);
            let (o, n) = (
                framed.point_in_space(geom_core::linalg::Point3::origin() + o)
                    - geom_core::linalg::Point3::origin(),
                framed.vec_in_space(n),
            );
            let name = table.name_of(&crate::names::EntityRef {
                body: 0,
                key: crate::names::EntityKey::Face(face),
            })?;
            [own_o.z, own_n.x, own_n.y]
                .into_iter()
                .all(|c| c == 0.0)
                .then(|| (name.clone(), own_n.z.signum(), n, o))
        })
    };
    // A body with no planar face on its own xy (a full revolve, an arc
    // tube) is placed by the moves `at` states, with no mate.
    let (Some((copy_face, copy_z, _, _)), Some((target_face, _, d, face_at))) =
        (own_plane(body), own_plane(target))
    else {
        return crate::Placed::of(doc, Vec::new(), moves_at(at));
    };
    let placed = at.affine::<f64>();
    // The copy's face's normal where `at` puts it.
    let m = placed.linear * geom_core::linalg::Vec3::new(0.0, 0.0, copy_z);
    // The target's axis its face's normal lies along, where it stands.
    let normal_axis = [d.x, d.y, d.z]
        .iter()
        .position(|c| c.abs() == 1.0)
        .filter(|&k| [d.x, d.y, d.z].iter().filter(|c| **c == 0.0).count() == 2 && k < 3);
    // An `at` tilting the copy's face off the target's face is no face
    // rest: the copy is placed by the moves `at` states, with no mate.
    let Some(k) = normal_axis.filter(|_| m.cross(d).norm() == 0.0) else {
        return crate::Placed::of(doc, Vec::new(), moves_at(at));
    };
    let plane = |shape: &crate::Operand, face: crate::StableName| {
        crate::Operand::Pose(Box::new(PoseDef::Plane {
            face: crate::Operand::select(shape.clone(), vec![face]),
        }))
    };
    // The standoff: how far along the target's normal the copy's face
    // stands from the target's.
    let along = d.dot(placed.translation - face_at);
    let to_plane = plane(target, target_face);
    let to = if along == 0.0 {
        to_plane
    } else {
        crate::Operand::Pose(Box::new(PoseDef::Slide {
            pose: RoleRead::plain(to_plane.clone()),
            along: RoleRead::through(to_plane, PoseRole::Normal),
            by: len(along),
        }))
    };
    let flipped = m.dot(d) < 0.0;
    let on = plane(body, copy_face);
    let on = if flipped {
        crate::Operand::Pose(Box::new(PoseDef::Flip {
            pose: RoleRead::plain(on),
        }))
    } else {
        on
    };
    // The orientation nearest where `at` turns the copy, which puts its
    // face's normal on the target's: the least turn is the identity, the
    // slides are the target's own coordinates across the normal, and the
    // spin is what is left about it, in quarter turns where it is one.
    let orient = crate::Orientation::nearest(&placed.linear);
    let left = placed.linear * orient.matrix::<f64>().transpose();
    let axes = [
        geom_core::linalg::Vec3::new(1.0, 0.0, 0.0),
        geom_core::linalg::Vec3::new(0.0, 1.0, 0.0),
        geom_core::linalg::Vec3::new(0.0, 0.0, 1.0),
    ];
    let u = axes[(k + 1) % 3];
    let turned = left * u;
    let (c, s) = (u.dot(turned), u.cross(turned).dot(d));
    let spin = match (c, s) {
        (1.0, 0.0) => Formula::zero(Dimension::Angle),
        (0.0, 1.0) => Formula::turn_over(4).expect("a quarter turn"),
        (-1.0, 0.0) => Formula::turn_over(2).expect("a half turn"),
        (0.0, -1.0) => {
            Formula::neg(Formula::turn_over(4).expect("a quarter turn")).expect("an angle")
        }
        (c, s) => ang(s.atan2(c)),
    };
    let across = [TargetAxis::of_index(k + 1), TargetAxis::of_index(k + 2)];
    let across = if across[0] < across[1] {
        across
    } else {
        [across[1], across[0]]
    };
    let coordinate = |a: TargetAxis| axes[a.index()].dot(placed.translation);
    let slide = |by: f64| {
        if by == 0.0 {
            Formula::zero(Dimension::Length)
        } else {
            len(by)
        }
    };
    crate::Placed {
        mates: vec![crate::Mate { on, to, orient }],
        values: vec![
            (Coord::Slide(across[0]), slide(coordinate(across[0]))),
            (Coord::Slide(across[1]), slide(coordinate(across[1]))),
            (Coord::SPIN, spin),
        ],
    }
}

/// **The frame at `origin` with axes `u` and `v`** (`v` made orthogonal
/// to `u`, both normalised), the normal `u × v` — a construction's own
/// frame where a test places it.
///
/// # Panics
///
/// If `u` and `v` span no plane.
#[must_use]
pub fn axes_at(origin: [f64; 3], u: [f64; 3], v: [f64; 3]) -> crate::Frame {
    use geom_core::linalg::Vec3 as V;
    let u = V::new(u[0], u[1], u[2]);
    let v = V::new(v[0], v[1], v[2]);
    let x = u * (1.0 / u.norm());
    let y = v - x * v.dot(x);
    assert!(y.norm() > 0.0, "the frame's axes span a plane");
    let y = y * (1.0 / y.norm());
    let z = x.cross(y);
    crate::Frame {
        columns: [[x.x, x.y, x.z], [y.x, y.y, y.z], [z.x, z.y, z.z]],
        translation: origin,
    }
}

/// **The plane through `origin` with normal `normal`, read off a face of
/// `body` that has that normal** (both in `body`'s own coordinates): the
/// plane of the body's planar face whose outward normal is `normal`'s
/// direction, or its opposite flipped, slid along it onto `origin` — a
/// split's tool or a plane target parallel to a face of the body,
/// written with no turn, so it stands exactly where it is meant. A face
/// whose plane passes through `origin`, within `tol`'s ε, is that
/// plane, read with no slide; of several, the first by name, so the
/// face read is the document's choice and never the arena's. A placed
/// copy is read where its placement put it, `origin` and `normal`
/// carried there with it.
///
/// # Panics
///
/// If `body` has no planar face with that normal, either way, or no
/// value; a plane no face of the body is parallel to is
/// [`plane_on_frame`]'s.
#[must_use]
pub fn plane_on(
    doc: &ProfileDoc,
    body: impl Into<crate::Operand>,
    origin: [f64; 3],
    normal: [f64; 3],
    tol: geom_core::Tol,
) -> crate::pose::RoleRead<crate::Operand> {
    use crate::pose::{PoseDef, PoseRole, RoleRead};
    use geom_core::linalg::{Point3, Vec3 as V};
    let body = body.into();
    let ev: crate::Evaluation<f64> = crate::evaluate(
        doc,
        None,
        &crate::CancelToken::new(),
        &crate::EvalOptions::default(),
        tol,
    );
    // A copy's body is its source's, in the source's coordinates.
    let (node, framed, table, _) =
        crate::mate::corner::read_of(doc, &ev, &body).expect("the body has a value");
    let solid = framed.body();
    let n = V::new(normal[0], normal[1], normal[2]);
    let n = n * (1.0 / n.norm());
    let o = Point3::new(origin[0], origin[1], origin[2]);
    let parallel: Vec<_> = solid
        .faces()
        .filter_map(|(face, _)| {
            let kind = topo::readback::face_carrier_kind(solid, face).ok()?;
            (kind == geom::SurfaceKind::Plane).then_some(())?;
            let pose = topo::readback::face_pose(solid, face).ok()?;
            let out = geom_brep::OutwardNormal::from_chart(pose.axis, pose.sense).vec();
            let sense = if out.to_array() == n.to_array() {
                1.0
            } else if (-out).to_array() == n.to_array() {
                -1.0
            } else {
                return None;
            };
            Some((face, sense, pose.origin))
        })
        .collect();
    let named = |face| {
        table
            .name_of(&crate::names::EntityRef {
                body: 0,
                key: crate::names::EntityKey::Face(face),
            })
            .expect("a face of the body is named")
            .clone()
    };
    let through = parallel
        .iter()
        .filter(|(_, _, at)| (o - *at).dot(n).abs() <= tol.eps())
        .map(|&(face, sense, at)| (named(face), sense, at))
        .min_by(|a, b| a.0.cmp(&b.0));
    let (name, sense, at) = match through.clone() {
        Some(on) => on,
        None => parallel
            .first()
            .map(|&(face, sense, at)| (named(face), sense, at))
            .unwrap_or_else(|| panic!("{node} has a planar face with normal {normal:?}")),
    };
    let read = |def| RoleRead::plain(crate::Operand::Pose(Box::new(def)));
    let plane = read(PoseDef::Plane {
        face: crate::Operand::select(body, vec![name]),
    });
    let plane = if sense > 0.0 {
        plane
    } else {
        read(PoseDef::Flip { pose: plane })
    };
    if through.is_some() {
        return plane;
    }
    let by = (o - at).dot(n);
    let along = RoleRead::through(plane.read.clone(), PoseRole::Normal);
    read(PoseDef::Slide {
        pose: plane,
        along,
        by: crate::Formula::literal(by, Dimension::Length).expect("a finite slide"),
    })
}

/// **The plane through `origin` with normal `normal`, read off a frame
/// on `body`'s own geometry** ([`frame_on`] through its `xy`): a
/// plane no face of the body is parallel to. The frame's turns are read
/// once off the geometry by `atan2` and written as numbers, so a plane
/// meant on the body's axes stands there to the rounding of those
/// turns; [`plane_on`] reads one a face is parallel to exactly. A
/// test's tool, which no placement reads.
///
/// # Panics
///
/// If `normal` is zero or `body` offers no frame to read.
#[must_use]
pub fn plane_on_frame(
    doc: &ProfileDoc,
    body: impl Into<crate::Operand>,
    origin: [f64; 3],
    normal: [f64; 3],
    tol: geom_core::Tol,
) -> crate::pose::RoleRead<crate::Operand> {
    use geom_core::linalg::Vec3 as V;
    let n = V::new(normal[0], normal[1], normal[2]);
    // Any axis off the normal starts the plane's own `x`.
    let seed = if n.x.abs() <= n.y.abs().max(n.z.abs()) {
        V::new(1.0, 0.0, 0.0)
    } else {
        V::new(0.0, 1.0, 0.0)
    };
    let u = seed - n * (seed.dot(n) / n.dot(n));
    let v = n.cross(u);
    let at = axes_at(origin, [u.x, u.y, u.z], [v.x, v.y, v.z]);
    let frame = frame_on(doc, &body.into(), &at, tol);
    crate::pose::RoleRead::through(frame, crate::pose::PoseRole::Xy)
}

/// **A frame read off `body`'s own geometry, standing at `at` in its
/// own coordinates**: `Through` the first edge by id with an axis and
/// the first vertex by id off its line, spun about that frame's own x,
/// y and z (`linear = Rz(γ)·Ry(β)·Rx(α)`, the angles taken once by
/// `atan2`) and slid along them to `at`.
///
/// # Panics
///
/// If the body has no value, or no such edge and vertex.
pub fn frame_on(
    doc: &ProfileDoc,
    body: &crate::Operand,
    at: &crate::Frame,
    tol: geom_core::Tol,
) -> crate::Operand {
    frame_on_in(doc, &crate::EvalOptions::default(), body, at, tol)
}

/// [`frame_on`] under `opts` — which reaches the parts an instance
/// copies.
///
/// # Panics
///
/// As [`frame_on`]'s.
#[must_use]
pub fn frame_on_in(
    doc: &ProfileDoc,
    opts: &crate::EvalOptions,
    body: &crate::Operand,
    at: &crate::Frame,
    tol: geom_core::Tol,
) -> crate::Operand {
    use crate::names::{EntityKey, EntityRef};
    use crate::pose::{PoseDef, RoleRead};
    use crate::{FrameMove, TargetAxis};
    use geom_core::UnitVec3;
    let band = geom_core::predicate::Band::linear(tol).expect("a band");
    let ev: crate::Evaluation<f64> =
        crate::evaluate(doc, None, &crate::CancelToken::new(), opts, tol);
    // A copy's body is its source's, in the source's coordinates.
    let (node, framed, table, _) =
        crate::mate::corner::read_of(doc, &ev, body).expect("the body has a value");
    let solid = framed.body();
    let name = |key| table.name_of(&EntityRef { body: 0, key }).cloned();
    let picked = solid.edges().find_map(|(edge, _)| {
        let kind = topo::readback::edge_carrier_kind(solid, edge).ok()?;
        matches!(
            kind,
            geom::CurveKind::Line | geom::CurveKind::Circle | geom::CurveKind::Ellipse
        )
        .then_some(())?;
        let line = topo::readback::edge_pose(solid, edge).ok()?;
        let dir = UnitVec3::new(line.axis, topo::query::POSE_UNIT_NORM, band).ok()?;
        let edge_name = name(EntityKey::Edge(edge))?;
        solid.vertices().find_map(|(vertex, _)| {
            let point = topo::readback::vertex_point(solid, vertex).ok()?;
            let off = point - line.origin;
            let across = off - dir.get() * off.dot(dir.get());
            UnitVec3::new(across, "frame_vertex_off_the_line", band).ok()?;
            let vertex_name = name(EntityKey::Vertex(vertex))?;
            let foot = line.origin + dir.get() * off.dot(dir.get());
            let frame = geom_core::RigidMotion::from_aim_and_reference(
                foot,
                dir,
                off,
                topo::query::POSE_UNIT_NORM,
                band,
            )
            .ok()?;
            Some((edge_name.clone(), vertex_name, frame))
        })
    });
    let (edge_name, vertex_name, frame) =
        picked.unwrap_or_else(|| panic!("{node} offers a frame to read"));
    let at = at.affine();
    let read = |def: PoseDef<crate::Operand, Formula>| {
        RoleRead::plain(crate::Operand::Pose(Box::new(def)))
    };
    let base = crate::Operand::Pose(Box::new(PoseDef::Through {
        axis: read(PoseDef::Axis {
            of: crate::Operand::select(body.clone(), vec![edge_name]),
        }),
        point: read(PoseDef::Point {
            of: crate::Operand::select(body.clone(), vec![vertex_name]),
        }),
    }));
    let rel = frame.to_affine().inverse() * at;
    let columns = [rel.linear.c0, rel.linear.c1, rel.linear.c2].map(|c| [c.x, c.y, c.z]);
    let spins = euler_turns(&columns)
        .into_iter()
        .filter(|(_, angle)| *angle != 0.0)
        .map(|(axis, angle)| FrameMove::Spin(axis, ang(angle)));
    let t = rel.translation;
    let slides = [
        (TargetAxis::X, t.x),
        (TargetAxis::Y, t.y),
        (TargetAxis::Z, t.z),
    ]
    .into_iter()
    .filter(|(_, length)| *length != 0.0)
    .map(|(axis, length)| FrameMove::Slide(axis, len(length)));
    crate::moved_frame(base, &spins.chain(slides).collect::<Vec<_>>())
}

/// **The axis through `origin` along `dir`, read off two faces of
/// `body`** (both in `body`'s own coordinates): the meet of the planes
/// through `origin` across `dir` along the two coordinate axes it is
/// not ([`plane_on`]), so a coordinate axis of the body is written
/// with no turn. A pattern's axis or an axis target where a test
/// writes one in the body's own coordinates.
///
/// # Panics
///
/// If `dir` is not a coordinate axis, either way, or `body` lacks a
/// planar face across either of the other two.
#[must_use]
pub fn axis_on(
    doc: &ProfileDoc,
    body: impl Into<crate::Operand>,
    origin: [f64; 3],
    dir: [f64; 3],
    tol: geom_core::Tol,
) -> crate::pose::RoleRead<crate::Operand> {
    use crate::pose::{PoseDef, RoleRead};
    let body = body.into();
    // The two coordinate axes across `dir`, in the order whose cross
    // product is `dir`.
    let (a, b) = match dir {
        [0.0, 0.0, z] if z != 0.0 => ([1.0, 0.0, 0.0], [0.0, z.signum(), 0.0]),
        [x, 0.0, 0.0] if x != 0.0 => ([0.0, 1.0, 0.0], [0.0, 0.0, x.signum()]),
        [0.0, y, 0.0] if y != 0.0 => ([0.0, 0.0, 1.0], [y.signum(), 0.0, 0.0]),
        _ => panic!("{dir:?} is a coordinate axis"),
    };
    let a = plane_on(doc, body.clone(), origin, a, tol);
    let b = plane_on(doc, body, origin, b, tol);
    RoleRead::plain(crate::Operand::Pose(Box::new(PoseDef::Meet { a, b })))
}

// --- recipes ---------------------------------------------------------

/// **A cylinder split through a rim arc twice**: `circle(0, 0, 0.5)`
/// extruded 1.0, split by the plane through `(0, 0.2, 0)` with normal
/// `(0, 1, -1)`, read off the cylinder's own geometry
/// ([`plane_on_frame`]). The plane crosses the start rim arc twice and
/// leaves the wall and the start cap one piece per side, under `tol`.
/// Returns the document and the ids of the extrude and the split. No
/// oracle: the rows that read it state what it must evaluate to.
///
/// # Panics
///
/// If a door refuses an insert.
pub fn clipped_cylinder(tol: geom_core::Tol) -> (ProfileDoc, [RecipeNodeId; 2]) {
    let ins = |doc: ProfileDoc, node: crate::AuthoredNode| {
        let a = crate::apply(
            &doc,
            &DocEdit::InsertNode {
                node: Box::new(node),
                fresh: Vec::new(),
            },
            tol,
        )
        .expect("the clipped cylinder's inserts apply");
        (a.doc, a.record.minted.expect("an insert mints a node"))
    };
    let doc = ProfileDoc::empty_derived("clipped_cylinder", tol);
    let (doc, profile) = ins(
        doc,
        Node::Profile(ProfileProgram {
            loops: vec![LoopProgram::circle(0.0, 0.0, 0.5).expect("finite")],
            ids: Vec::new(),
            on: None,
        }),
    );
    let (doc, ext) = ins(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(1.0),
        },
    );
    let tool = plane_on_frame(&doc, ext, [0.0, 0.2, 0.0], [0.0, 1.0, -1.0], tol);
    let (doc, split) = ins(
        doc,
        Node::Split {
            target: ext.into(),
            tool,
        },
    );
    (doc, [ext, split])
}

// --- the pick door --------------------------------------------------

/// **The six axis directions**, `+x, -x, +y, -y, +z, -z` in that order —
/// what every aim that walks a mesh from outside it fires along. No
/// oracle: which rays are fired decides nothing about what one answers.
///
/// The order is part of the value: a sweep that caps its printed
/// examples keeps the first few it meets, so a reordering changes what a
/// failure message shows and nothing a row counts.
pub const AXES: [Vec3<f64>; 6] = [
    Vec3::new(1.0, 0.0, 0.0),
    Vec3::new(-1.0, 0.0, 0.0),
    Vec3::new(0.0, 1.0, 0.0),
    Vec3::new(0.0, -1.0, 0.0),
    Vec3::new(0.0, 0.0, 1.0),
    Vec3::new(0.0, 0.0, -1.0),
];

/// A ray straight down through `(x, y)` from height `z`. No oracle: a
/// wrong ray misses, and the row that aimed it reds on its own premise.
pub fn down_from(x: f64, y: f64, z: f64) -> Ray {
    ray([x, y, z], [0.0, 0.0, -1.0])
}

/// **A ray aimed at `target`**: along `dir`, from `reach` directions
/// back, so the target is at parameter `reach` — and at distance `reach`
/// when `dir` is a unit vector.
///
/// The origin is `target - dir * reach` computed once, here; a row that
/// compares a hit's `t` against `reach` bit for bit is comparing against
/// the rounding of exactly this expression. No oracle, as [`down_from`].
pub fn aimed(target: Point3<f64>, dir: Vec3<f64>, reach: f64) -> Ray {
    Ray {
        origin: target - dir * reach,
        dir,
    }
}

/// **The near-tangent candidate**: a crossing whose determinant is
/// certified at `k / 6` of its own bound, over a triangle of area `0.5`
/// that is not degenerate: `ζ = 2⁻²⁰` and `ξ = k` ULP of it,
/// `e1 = (1, 0, ζ + ξ)`, `e2 = (0, 1, 0)`, `d = (1, 1, ζ)`, so
/// `p = (−ζ, 0, 1)` and `det = ξ` exactly. The origin is placed a unit
/// away and one unit off-axis, which makes `u = v = 0.5` and `u + v = 1`
/// exactly at every `k` — all three INSIDE the closed range, so what
/// happens to the candidate is INFORM's doing alone, at `t = 1.5`. The
/// bounds are `9/(k − 6)`, `15/(k − 6)` and their sum, so `k` is the
/// dial that moves the intervals without moving the values.
pub fn near_tangent(k: f64) -> (Ray, [Point3<f64>; 3]) {
    let zeta = 2f64.powi(-20);
    let xi = k * zeta * f64::EPSILON;
    let tri = [
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(1.0, 0.0, zeta + xi),
        Point3::new(0.0, 1.0, 0.0),
    ];
    (ray([-1.0, -1.0, 0.5 * xi - zeta], [1.0, 1.0, zeta]), tri)
}

/// **A pick door's whole answer as a list**: the one hit, the empty
/// miss, or the tied faces of a refusal. No oracle: it reads the door
/// and decides nothing about what it answered.
///
/// A certified tie between faces is an ANSWER about the ray — every hit
/// in it is true — so a row that reads the door reads it this way rather
/// than unwrapping past it.
///
/// # Panics
///
/// On any other refusal: that one is about the targets, not the
/// geometry, and a row reading a list aimed at targets it built.
pub fn listed(answer: Result<Option<PickHit>, HitTestError>) -> Vec<PickHit> {
    match answer {
        Ok(hit) => hit.into_iter().collect(),
        Err(HitTestError::Ambiguous { hits }) => hits,
        Err(other) => panic!("the pick answers or refuses on a tie: {other:?}"),
    }
}

/// **Möller–Trumbore's determinant, uncertified, and its
/// conditioning** `|det| / (|e1|·|e2|·|d|)` — up to a constant the sine
/// of the angle between the ray and the plane.
///
/// This one DOES carry an oracle: it is the reference a row reads
/// `crossing`'s refusals against (a refused candidate whose conditioning
/// is far above the certification's own bound is a genuine crossing the
/// door failed to certify), and a row that picks its well-conditioned
/// cases by it. It is computed here in plain `f64`, sharing no code with
/// the door, so a row reading it compares two computations rather than
/// one with itself.
pub fn det_and_conditioning(ray: &Ray, tri: &[Point3<f64>; 3]) -> (f64, f64) {
    let e1: Vec3<f64> = tri[1] - tri[0];
    let e2: Vec3<f64> = tri[2] - tri[0];
    let det = e1.dot(ray.dir.cross(e2));
    (det, det.abs() / (e1.norm() * e2.norm() * ray.dir.norm()))
}

// --- the recipe walks' pass-through set -----------------------------

/// **Which name-carrying edge the recipe walks read a node as**: the
/// variant of `names::VerbatimEdge`, without the operands it carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum VerbatimKind {
    /// `VerbatimEdge::Intact`: the entities a split leaves intact.
    Intact,
    /// `VerbatimEdge::Reflected`: a mirror's image.
    Reflected,
    /// `VerbatimEdge::Copied`: a placement's or an instance's copy.
    Copied,
}

/// **The name-carrying edge the recipe walks read `node` as, if any**:
/// `names::verbatim_edge`, the one statement of N1's pass-through set
/// that the recipe walks (refactor, coincidence, the mate member walk) follow,
/// reduced to its variant.
///
/// Carries no oracle: it IS the classification, lifted out of the
/// crate so a row can hold it against what evaluation publishes
/// (`tests/names_verbatim_edge_evaluator.rs`). The match has no
/// wildcard, so an edge kind added to `VerbatimEdge` does not compile
/// until it is mirrored here.
pub fn verbatim_kind<P>(node: &Node<P>) -> Option<VerbatimKind> {
    use crate::names::VerbatimEdge;
    Some(match crate::names::verbatim_edge(node)? {
        VerbatimEdge::Intact { .. } => VerbatimKind::Intact,
        VerbatimEdge::Reflected { .. } => VerbatimKind::Reflected,
        VerbatimEdge::Copied { .. } => VerbatimKind::Copied,
    })
}

// --- the load door's nesting limit ----------------------------------

/// **The deepest a body the load door reads may nest**, in JSON
/// brackets: `persist::nesting`'s limit, lifted out of the crate so a
/// row can hold it against the deepest body a save writes
/// (`tests/expr_nesting_bound.rs`).
pub const BODY_NESTING: usize = crate::persist::nesting::BODY_NESTING;

/// **The deepest a body nests around a metadata value at its bound**,
/// which [`BODY_NESTING`] covers (`tests/meta_nesting_bound.rs`).
pub const META_BODY_NESTING: usize = crate::persist::nesting::META_BODY_NESTING;

/// **How deep a saved `text` nests**, in JSON brackets outside strings
/// (its header line holds none, so this is its body's depth):
/// the load door's own scan, so a row measures a save as the door does.
#[must_use]
pub fn bracket_depth(text: &str) -> usize {
    crate::persist::nesting::deepest(text)
}

// --- the mint's preimage --------------------------------------------

/// **The node id an insert of `node` draws in an empty document**:
/// the node lowered as the door lowers it, its variables minted first,
/// then `Mint::insert`, lifted out of the crate so a row can pin the
/// preimage node shape by node shape
/// (`tests/switch_slots.rs`, `every_node_shapes_mint_is_pinned`).
///
/// Carries no oracle: it IS the mint's draw, with no door around it, so
/// a shape whose operands read no live output still draws: an operand
/// naming node `n` is stored as the read with `n`'s own id
/// ([`stored_reading`]).
pub fn first_node_id(node: &crate::AuthoredNode, tol: geom_core::Tol) -> RecipeNodeId {
    let mut doc = ProfileDoc::empty_derived("first_node_id", tol);
    let node = stored_reading(&mut doc, node, |id, _| crate::VarId(id.0));
    doc.mint.insert(&node)
}

/// **A spoken node built by hand**: what a document holding `id` as a
/// `kind` with no label would say (`None`: a document that does not
/// hold it), for a fixture that builds a row by hand rather than
/// through a document.
#[must_use]
pub fn spoken(id: RecipeNodeId, kind: Option<&'static str>) -> crate::SpokenNode {
    crate::SpokenNode::forged(id, kind, None)
}

/// [`spoken`] for a node the document holds as a `kind` labelled
/// `label`.
#[must_use]
pub fn spoken_labelled(
    id: RecipeNodeId,
    kind: &'static str,
    label: crate::Label,
) -> crate::SpokenNode {
    crate::SpokenNode::forged(id, Some(kind), Some(label))
}

/// `name` as a sentence speaks it with its minting node spoken as
/// `minter` says, which must name the same node.
#[must_use]
pub fn spoken_name(name: crate::StableName, minter: crate::SpokenNode) -> crate::SpokenName {
    assert_eq!(name.node, minter.id(), "the minter is the name's own node");
    crate::SpokenName::forged(name, minter)
}

// --- the content key's spelling --------------------------------------

/// **The bytes the content key writes for `node`'s `slot`**: the slot's
/// variable through the document's definitions, each free variable by
/// its id (`param_source::feed_content_key`'s encoding) — so two slots
/// spell equal exactly where they read one expression. `None` where
/// the node or the slot is absent.
pub fn slot_spelling(doc: &ProfileDoc, node: RecipeNodeId, slot: crate::SlotId) -> Option<Vec<u8>> {
    let var = doc.slot(node, slot)?;
    Some(crate::param_source::var_spelling(
        &crate::param_source::definitions_of(doc),
        var,
    ))
}

// --- a state no door admits -------------------------------------------

/// **`mate` written into `placement`'s bundle past the bundle rule**:
/// the constraint lowered as the door lowers it, its id minted, and
/// admitted on its own datum as the door admits it, then appended
/// beside the bundle's other constraints with no bundle check. The
/// state a doctored save leaves, which no edit door admits and the fold
/// refuses (`MateFault::Overconstrained`); for rows that hold the
/// solve's half of the rule. Answers the document and the constraint's
/// id.
///
/// # Panics
///
/// If `placement` is no placement, or the door refuses `mate` on its
/// own.
pub fn beside_past_the_bundle_rule(
    doc: &ProfileDoc,
    placement: RecipeNodeId,
    mate: crate::Mate<Formula>,
    _tol: geom_core::Tol,
) -> (ProfileDoc, crate::node::ConstraintId) {
    let mut out = doc.clone();
    // The mate lowered in a carrier bundle of its own, as the door
    // lowers a bundle's constraints.
    let carrier: Node<
        <crate::program::ProfileProgram as crate::ProfilePayload>::Authored,
        Formula,
    > = Node::Place {
        source: crate::BodyRead::plain(crate::Operand::Var(
            out.output(placement, 0)
                .unwrap_or_else(|| panic!("{placement:?} defines its copy")),
        )),
        target: crate::Operand::Var(crate::VarId::WORLD),
        constraints: vec![crate::Constraint::of_mate(
            crate::node::ConstraintId::placeholder(1),
            mate,
        )],
    };
    let Node::Place {
        constraints: mut lowered,
        ..
    } = crate::edit::lower_node_into(&mut out, &carrier)
        .unwrap_or_else(|e| panic!("the mate lowers: {e}"))
    else {
        unreachable!("a placement lowers to a placement")
    };
    let mut written = lowered.remove(0);
    let id = out.mint.add_constraint(placement, &written.kind);
    written.id = id;
    let crate::mate::ConstraintKind::Mate(data) = &written.kind else {
        unreachable!("the carrier holds a mate")
    };
    crate::mate::solve::admit_mate(&out, crate::MateRef::new(placement, id), data)
        .unwrap_or_else(|e| panic!("the door admits the mate on its own: {e}"));
    match out.nodes.get_mut(&placement) {
        Some(Node::Place { constraints, .. } | Node::InstantiatePart { constraints, .. }) => {
            constraints.push(written);
        }
        _ => panic!("{placement} is no placement"),
    }
    (out, id)
}

/// **A save's snapshot, read past the load door's validator**: the
/// state a doctored save leaves, which [`crate::persist::load`]
/// refuses; for rows that hold the save door's half of the shared
/// validator. The edit log is parsed and dropped.
///
/// # Panics
///
/// If `text` is not a save's spelling the reader accepts.
pub fn past_the_load_door(text: &str) -> ProfileDoc {
    crate::persist::snapshot_unchecked(text)
        .unwrap_or_else(|e| panic!("the text reads as a save: {e}"))
}

// --- a union's fold rows, as their steps named them ---------------

/// **One fold-step row as the step named it**: its cells' names in
/// the union's own space, collapsed ([`crate::names::collapse_table`]),
/// the relation the step decided, and the row the union spelled from
/// them in member space. A suite resolves the names to member cells by
/// its own reading and holds the spelled row to it.
#[derive(Clone, Debug)]
pub struct FoldRowNames {
    /// The cells' collapsed names, in the order the step read them: two,
    /// or a meeting's more (`topo::Coincidence::more`).
    pub cells: Vec<crate::names::StableName>,
    /// What the step decided between them.
    pub relation: topo::Relation,
    /// The row the union spelled.
    pub spelled: crate::coincide::NamedCoincidence,
}

/// Where [`fold_rows_of`] collects, while it runs.
static FOLD_ROWS: std::sync::Mutex<Option<Vec<FoldRowNames>>> = std::sync::Mutex::new(None);

/// **`f`, with every fold-step row a union spells while it runs**, in
/// spelling order. One collector at a time: a nested call panics.
///
/// # Panics
///
/// Where another call is collecting, or the collector was poisoned.
pub fn fold_rows_of<R>(f: impl FnOnce() -> R) -> (R, Vec<FoldRowNames>) {
    {
        let mut slot = FOLD_ROWS.lock().expect("the fold-row collector");
        assert!(slot.is_none(), "one fold-row collector at a time");
        *slot = Some(Vec::new());
    }
    let out = f();
    let rows = FOLD_ROWS
        .lock()
        .expect("the fold-row collector")
        .take()
        .unwrap_or_default();
    (out, rows)
}

/// Records one fold-step row where [`fold_rows_of`] is collecting.
pub(crate) fn record_fold_row(
    named: &crate::coincide::NamedCoincidence,
    spelled: &crate::coincide::NamedCoincidence,
) {
    let mut slot = FOLD_ROWS.lock().expect("the fold-row collector");
    let Some(rows) = slot.as_mut() else { return };
    let name = |cell: &crate::coincide::NamedCell| match cell {
        crate::coincide::NamedCell::Entity { name, .. } => name.clone(),
        other => panic!("a fold row's cell is an entity of the union: {other:?}"),
    };
    rows.push(FoldRowNames {
        cells: named.cells.iter().map(name).collect(),
        relation: named.relation,
        spelled: spelled.clone(),
    });
}

/// **The product said as the world's copies** (D10 `[operations]`): the
/// slot set to the one world copy, or to a `⊔` of the world's copies in
/// document order, written through the doors — what a fixture that
/// places bodies into the world and makes all of them means. A `⊔` this
/// wrote before is replaced, so a fixture that places one more body
/// calls it again. A document with no world placement is left with an
/// empty product. The one home of this fixture: a demo or a row about
/// what a user authors names its product's copies instead.
///
/// # Panics
///
/// If a door refuses an edit it writes.
#[must_use]
pub fn assemble_world_copies(doc: ProfileDoc, tol: geom_core::Tol) -> ProfileDoc {
    assemble_world_copies_edits(doc, tol).0
}

/// **[`assemble_world_copies`] with the edits it wrote**, in the order it
/// wrote them: what a recorded log replays to reach the same product.
///
/// # Panics
///
/// If a door refuses an edit it writes.
#[must_use]
#[allow(clippy::expect_used)]
pub fn assemble_world_copies_edits(
    doc: ProfileDoc,
    tol: geom_core::Tol,
) -> (ProfileDoc, Vec<crate::DocEdit<ProfileProgram>>) {
    let mut log = Vec::new();
    let apply = |log: &mut Vec<_>, doc: &ProfileDoc, edit: crate::DocEdit<ProfileProgram>| {
        let out = crate::apply(doc, &edit, tol)
            .unwrap_or_else(|e| panic!("the product's edit applies: {e}"))
            .doc;
        log.push(edit);
        out
    };
    let mut doc = doc;
    // A `⊔` the slot reads, read by nothing else, is this helper's own.
    let held = doc
        .product()
        .and_then(|var| doc.operation_of(var))
        .filter(|&at| matches!(doc.node(at), Some(Node::Assemble { .. })))
        .filter(|&at| {
            let var = doc.output(at, 0);
            !doc.ids().into_iter().any(|id| {
                doc.node(id).is_some_and(|node| {
                    node.operand_rows()
                        .into_iter()
                        .any(|(_, read)| Some(read) == var)
                })
            })
        });
    // A product already the world's copies is kept as it is, so a
    // document written twice is the one document.
    let world: Vec<crate::VarId> = doc
        .placements()
        .into_iter()
        .filter(|&p| doc.node_space(p) == Some(crate::VarId::WORLD))
        .filter_map(|p| doc.output(p, 0))
        .filter(|&copy| {
            doc.var(copy)
                .is_some_and(|v| v.kind() == crate::VarKind::Body)
        })
        .collect();
    let spelled = |copies: &[crate::VarId]| {
        crate::Bodies::Spelled(
            copies
                .iter()
                .map(|&copy| crate::BodyRead::plain(copy))
                .collect(),
        )
    };
    let kept = match world.as_slice() {
        [] => doc.product().is_none(),
        [one] => doc.product() == Some(*one),
        several => held.is_some_and(|at| {
            matches!(doc.node(at), Some(Node::Assemble { members }) if *members == spelled(several))
        }),
    };
    if kept {
        return (doc, log);
    }
    doc = apply(&mut log, &doc, crate::DocEdit::SetProduct { product: None });
    if let Some(at) = held {
        doc = apply(&mut log, &doc, crate::DocEdit::DeleteNode { id: at });
    }
    let product = match world.as_slice() {
        [] => return (doc, log),
        [one] => crate::Operand::Var(*one),
        several => {
            let insert = crate::DocEdit::InsertNode {
                node: Box::new(Node::Assemble {
                    members: crate::Bodies::Spelled(
                        several
                            .iter()
                            .map(|&copy| crate::BodyRead::plain(crate::Operand::Var(copy)))
                            .collect(),
                    ),
                }),
                fresh: Vec::new(),
            };
            let outcome = crate::apply(&doc, &insert, tol)
                .unwrap_or_else(|e| panic!("the world's copies assemble: {e}"));
            log.push(insert);
            doc = outcome.doc;
            crate::Operand::Node(outcome.record.minted.expect("an insert mints its node"))
        }
    };
    let doc = apply(
        &mut log,
        &doc,
        crate::DocEdit::SetProduct {
            product: Some(product),
        },
    );
    (doc, log)
}

/// **What the at-rest check could not look at in one gathered
/// `product`**, read as the `AtRest` resident reads it: for a row at a
/// scalar the checks registry cannot run at (a dual, which carries no
/// chart-coherence lane), whose census is structurally absent
/// ([`crate::AtRestEvidence::NotRun`]).
pub fn at_rest_could_not_look<T: crate::checks::at_rest::LocalizeLane>(
    product: &crate::Product<T>,
    tol: geom_core::Tol,
) -> Vec<crate::AtRestEvidence> {
    crate::checks::at_rest::reading(product, tol).could_not_look
}

/// The judgement whose naming [`with_judgement_naming_fault`] refuses,
/// while it runs.
static JUDGEMENT_FAULT: std::sync::Mutex<Option<usize>> = std::sync::Mutex::new(None);

/// What [`with_judgement_naming_fault`]'s refused naming says.
pub const JUDGEMENT_NAMING_FAULT: &str = "a test refused the naming of a union's judgement";

/// **`f`, with the naming of every union's judgement `k` refused**
/// ([`JUDGEMENT_NAMING_FAULT`]): a seam for the order in which a union
/// announces what goes wrong, which no geometry reaches on demand. One
/// at a time: a nested call panics.
///
/// The seam is disarmed when `f` returns or unwinds.
///
/// # Panics
///
/// Where another call is running.
pub fn with_judgement_naming_fault<R>(k: usize, f: impl FnOnce() -> R) -> R {
    /// Disarms the seam on every way out of `f`.
    struct Disarm;
    impl Drop for Disarm {
        fn drop(&mut self) {
            *JUDGEMENT_FAULT
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner) = None;
        }
    }
    {
        let mut slot = JUDGEMENT_FAULT
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        assert!(slot.is_none(), "one judgement-naming fault at a time");
        *slot = Some(k);
    }
    let _disarm = Disarm;
    f()
}

/// The refusal [`with_judgement_naming_fault`] makes of judgement `k`'s
/// naming, where it is running for `k`.
pub(crate) fn judgement_naming_fault(k: usize) -> Option<crate::names::NamingError> {
    (*JUDGEMENT_FAULT
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        == Some(k))
    .then_some(crate::names::NamingError::Emission {
        what: JUDGEMENT_NAMING_FAULT,
    })
}

// --- the quieting rule ---------------------------------------------

/// **The row half of the quieting rule, asked of any row**
/// (`checks::at_rest::quiet_row`): the assertion that quiets `row` in
/// `evaluation`, or `None`. For a row of a kind no document reaches
/// yet, pinned over cells of a real evaluation.
#[must_use]
pub fn quiet_row(
    doc: &ProfileDoc,
    evaluation: &crate::Evaluation<f64>,
    row: &crate::coincide::NamedCoincidence,
    tol: geom_core::Tol,
) -> Option<RecipeNodeId> {
    crate::checks::at_rest::quiet_row(doc, evaluation, row, tol)
}

/// Whether an assertion could quiet `row` in `evaluation`
/// (`checks::at_rest::row_quietable`), asked as [`quiet_row`] asks.
#[must_use]
pub fn row_quietable(
    doc: &ProfileDoc,
    evaluation: &crate::Evaluation<f64>,
    row: &crate::coincide::NamedCoincidence,
    tol: geom_core::Tol,
) -> bool {
    crate::checks::at_rest::row_quietable(doc, evaluation, row, tol)
}

#[cfg(test)]
mod tests {
    use super::{axes_at, moves_at};
    use crate::{Coord, Formula, TargetAxis};

    /// **A frame's quarter turns are read about the axes they turn
    /// about**: a frame turned a quarter about each of the target
    /// space's axes, and one turned about `x` then `z`, read back as
    /// those spins, each `turn/4`, in the chain's order.
    ///
    /// *Breaks if* a turn about one axis is labelled with another.
    #[test]
    fn a_quarter_turn_is_read_about_its_own_axis() {
        let quarter = Formula::turn_over(4).expect("a quarter turn");
        let spins = |at| -> Vec<(Coord, Formula)> {
            moves_at(&at)
                .into_iter()
                .filter(|(c, _)| matches!(c, Coord::Spin(_)))
                .collect()
        };
        for (axis, x, y) in [
            (TargetAxis::X, [1.0, 0.0, 0.0], [0.0, 0.0, 1.0]),
            (TargetAxis::Y, [0.0, 0.0, -1.0], [0.0, 1.0, 0.0]),
            (TargetAxis::Z, [0.0, 1.0, 0.0], [-1.0, 0.0, 0.0]),
        ] {
            assert_eq!(
                spins(axes_at([0.0; 3], x, y)),
                vec![(Coord::Spin(Some(axis)), quarter.clone())],
                "a quarter turn about {axis:?}"
            );
        }
        // About x, then z: x ↦ y, y ↦ z, z ↦ x.
        assert_eq!(
            spins(axes_at([0.0; 3], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0])),
            vec![
                (Coord::Spin(Some(TargetAxis::X)), quarter.clone()),
                (Coord::Spin(Some(TargetAxis::Z)), quarter),
            ],
        );
    }
}
