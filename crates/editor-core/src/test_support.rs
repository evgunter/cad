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
        |p, f| ProfileProgram::lower(p, f),
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

/// **`doc` with `placed`'s mate and values added to `placement`**.
///
/// # Panics
///
/// If a door refuses one.
fn constrained(doc: ProfileDoc, placement: RecipeNodeId, placed: crate::Placed) -> ProfileDoc {
    let tol = geom_core::Tol::witness();
    let step = |doc: &ProfileDoc, edit: DocEdit<ProfileProgram>| {
        crate::apply(doc, &edit, tol).expect("a placement's constraint applies")
    };
    let mated = step(
        &doc,
        DocEdit::AddConstraint {
            placement,
            mate: Box::new(placed.mate),
            fresh: Vec::new(),
        },
    );
    let mate = *mated
        .record
        .minted_constraints
        .first()
        .expect("a mate mints its id");
    placed
        .values
        .into_iter()
        .fold(mated.doc, |doc, (coord, by)| {
            step(
                &doc,
                DocEdit::AddValue {
                    placement,
                    value: Box::new(crate::mate::Value { mate, coord, by }),
                    fresh: Vec::new(),
                },
            )
            .doc
        })
}

/// **A placement of `body`**, inserted with no constraint yet.
fn place_of(doc: ProfileDoc, body: crate::Operand) -> (ProfileDoc, RecipeNodeId) {
    let tol = geom_core::Tol::witness();
    let applied = crate::apply(
        &doc,
        &DocEdit::InsertNode {
            node: Box::new(Node::place(vec![crate::BodyRead::plain(body)])),
            fresh: Vec::new(),
        },
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

/// **A copy of `body` standing in the world at `at`** (D10: a body has
/// no location, so where it stands is its placement's): where it
/// stands at the identity ([`crate::stand`]), elsewhere by the "place
/// part" gesture ([`crate::place_at`]). Returns the document and the
/// placement; its copy is the placement's output.
///
/// # Panics
///
/// If the copy offers no geometry to place it by, or a door refuses.
pub fn in_world(
    doc: ProfileDoc,
    body: impl Into<crate::Operand>,
    at: &crate::Frame,
) -> (ProfileDoc, RecipeNodeId) {
    let tol = geom_core::Tol::witness();
    let body = body.into();
    let opts = crate::EvalOptions::default();
    let placed = if *at == crate::Frame::IDENTITY {
        crate::stand(&doc, &body, &opts, tol)
    } else {
        crate::place_at(&doc, &body, at, &opts, tol)
    }
    .expect("the copy places over its geometry");
    let (doc, placement) = place_of(doc, body);
    (constrained(doc, placement, placed), placement)
}

/// **A copy of `body` placed against `target`'s geometry at `at`** —
/// `at` the copy's own frame in `target`'s own coordinates
/// ([`crate::place_against`]): how a construction combined with another
/// body joins that body's space. Returns the document and the
/// placement; its copy is the placement's output.
///
/// # Panics
///
/// If either offers no geometry to place by, or a door refuses.
pub fn against(
    doc: ProfileDoc,
    target: impl Into<crate::Operand>,
    body: impl Into<crate::Operand>,
    at: &crate::Frame,
) -> (ProfileDoc, RecipeNodeId) {
    let tol = geom_core::Tol::witness();
    let (target, body) = (target.into(), body.into());
    let placed = crate::place_against(
        &doc,
        &target,
        &body,
        at,
        &crate::EvalOptions::default(),
        tol,
    )
    .expect("the copy places against its target's geometry");
    let (doc, placement) = place_of(doc, body);
    (constrained(doc, placement, placed), placement)
}

/// **The frame at `origin` with axes `u` and `v`** (`v` made orthogonal
/// to `u`, both normalised), the normal `u × v` — a construction's own
/// frame where a test places it.
///
/// # Panics
///
/// If `u` and `v` span no plane.
#[must_use]
pub fn frame_at(origin: [f64; 3], u: [f64; 3], v: [f64; 3]) -> crate::Frame {
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

/// **The plane through `origin` with normal `normal`, read off `body`'s
/// own geometry** ([`crate::frame_at`] through its `xy`): a split's tool
/// or a plane target where a test writes one in the body's own
/// coordinates.
///
/// # Panics
///
/// If `normal` is zero or `body` offers no frame to read.
#[must_use]
pub fn plane_on(
    doc: &ProfileDoc,
    body: impl Into<crate::Operand>,
    origin: [f64; 3],
    normal: [f64; 3],
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
    let at = frame_at(origin, [u.x, u.y, u.z], [v.x, v.y, v.z]);
    let frame = crate::frame_at(
        doc,
        &body.into(),
        &at,
        &crate::EvalOptions::default(),
        geom_core::Tol::witness(),
    )
    .expect("the body offers a frame to read the plane off");
    crate::pose::RoleRead::through(frame, crate::pose::PoseRole::Xy)
}

/// **The axis through `origin` along `dir`, read off `body`'s own
/// geometry** ([`crate::frame_at`] through its `z`): a pattern's axis or
/// an axis target where a test writes one in the body's own
/// coordinates.
///
/// # Panics
///
/// If `dir` is zero or `body` offers no frame to read.
#[must_use]
pub fn axis_on(
    doc: &ProfileDoc,
    body: impl Into<crate::Operand>,
    origin: [f64; 3],
    dir: [f64; 3],
) -> crate::pose::RoleRead<crate::Operand> {
    let plane = plane_on(doc, body, origin, dir);
    crate::pose::RoleRead::through(plane.read, crate::pose::PoseRole::Z)
}

// --- recipes ---------------------------------------------------------

/// **A cylinder split through a rim arc twice**: `circle(0, 0, 0.5)`
/// extruded 1.0, split by the plane through `(0, 0.2, 0)` with normal
/// `(0, 1, -1)`, read off the cylinder's own geometry
/// ([`crate::frame_at`]). The plane crosses the start rim arc twice and
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
        }),
    );
    let (doc, ext) = ins(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(1.0),
        },
    );
    let tool = plane_on(&doc, ext, [0.0, 0.2, 0.0], [0.0, 1.0, -1.0]);
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
    /// `VerbatimEdge::Selected`: one body of the input, projected.
    Selected,
    /// `VerbatimEdge::Intact`: the entities a split leaves intact.
    Intact,
}

/// **The name-carrying edge the recipe walks read `node` as, if any**:
/// `names::verbatim_edge`, the one statement of N1's pass-through set
/// that the product's two-roots check and the mate member walk follow,
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
        VerbatimEdge::Selected { .. } => VerbatimKind::Selected,
        VerbatimEdge::Intact => VerbatimKind::Intact,
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
/// the constraint lowered and admitted as [`DocEdit::AddConstraint`]
/// admits it into an empty bundle, then the bundle's other
/// constraints put back beside it. The state a doctored save leaves, which
/// no edit door admits and the fold refuses (`MateFault::Overconstrained`); for
/// rows that hold the solve's half of the rule. Answers the document
/// and the constraint's id.
///
/// # Panics
///
/// If `placement` is no placement, or the door refuses `mate` on its
/// own.
pub fn beside_past_the_bundle_rule(
    doc: &ProfileDoc,
    placement: RecipeNodeId,
    mate: crate::Mate<Formula>,
    tol: geom_core::Tol,
) -> (ProfileDoc, crate::node::ConstraintId) {
    let bundle = |doc: &mut ProfileDoc| match doc.nodes.get_mut(&placement) {
        Some(Node::Place { constraints, .. } | Node::InstantiatePart { constraints, .. }) => {
            std::mem::take(constraints)
        }
        _ => panic!("{placement} is no placement"),
    };
    let mut emptied = doc.clone();
    let held = bundle(&mut emptied);
    let applied = crate::apply(
        &emptied,
        &DocEdit::AddConstraint {
            placement,
            mate: Box::new(mate),
            fresh: Vec::new(),
        },
        tol,
    )
    .unwrap_or_else(|e| panic!("the door admits the mate alone: {e}"));
    let mut out = applied.doc;
    let id = applied.record.minted_constraints[0];
    // The door dropped the variables only the taken-out constraints
    // read; they go back with them.
    for (var, def) in &doc.vars {
        out.vars.entry(*var).or_insert_with(|| def.clone());
    }
    let added = bundle(&mut out);
    match out.nodes.get_mut(&placement) {
        Some(Node::Place { constraints, .. } | Node::InstantiatePart { constraints, .. }) => {
            *constraints = held.into_iter().chain(added).collect();
        }
        _ => unreachable!("the placement was found above"),
    }
    (out, id)
}

// --- a union's fold rows, as their steps named them ---------------

/// **One fold-step row as the step named it**: its two cells' names in
/// the union's own space, collapsed ([`crate::names::collapse_table`]),
/// the relation the step decided, and the row the union spelled from
/// them in member space. A suite resolves the names to member cells by
/// its own reading and holds the spelled row to it.
#[derive(Clone, Debug)]
pub struct FoldRowNames {
    /// The two cells' collapsed names, in the order the step read them.
    pub cells: [crate::names::StableName; 2],
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
    let [a, b] = named.cells.as_slice() else {
        panic!("a fold step's row names two cells: {named:?}");
    };
    rows.push(FoldRowNames {
        cells: [name(a), name(b)],
        relation: named.relation,
        spelled: spelled.clone(),
    });
}
