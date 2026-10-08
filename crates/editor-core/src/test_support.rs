//! **Test fixtures**, behind the `test-support` feature (on only
//! through dev-dependency edges): a literal of each dimension, a point
//! of two lengths, the world xy frame a sketch is drawn on, the rays a
//! pick row aims, the near-tangent candidate the certified test is
//! probed with, the door's answer read as a list, the uncertified
//! determinant the review rows read the certified one against, the
//! recipe walks' pass-through classification a row holds against the
//! evaluator, and the load door's nesting limit a row holds against
//! what a save writes, and the recipes more than one suite builds.
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
    Datum, Dimension, DocEdit, Expr, Formula, HitTestError, LoopProgram, Node, PickHit, ProfileDoc,
    ProfileProgram, RecipeNodeId, RefusingReach,
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
    node.try_map_slots(
        |p, f, r| ProfileProgram::lower(p, f, r),
        &mut |f| crate::edit::lower_slot_into(doc, f),
        &mut |_, operand| {
            Ok(match operand {
                crate::Operand::Node(id) => read(*id, 0),
                crate::Operand::Output { node, port } => read(*node, *port),
                crate::Operand::Var(var) => *var,
                crate::Operand::Name(name) => {
                    panic!("an operand read as given has no name: {name}")
                }
            })
        },
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
    let read: Vec<crate::VarId> = node
        .exprs()
        .into_iter()
        .copied()
        .filter(|var| lost.contains(var))
        .collect();
    assert!(
        read.is_empty(),
        "a written re-insert would not reproduce {read:?}: shared, or toleranced"
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

/// The stored placement `placement` lowers to in `doc` ([`stored`]).
///
/// # Panics
///
/// If `placement` does not lower in `doc`.
pub fn stored_placement(
    doc: &mut ProfileDoc,
    placement: &crate::Placement<Formula>,
) -> crate::Placement {
    placement
        .try_map_slots(&mut |f| crate::edit::lower_slot_into(doc, f))
        .expect("a placement the document can answer lowers")
}

// --- the frame a sketch is drawn on ---------------------------------

/// The frame datum a profile is drawn on, as a node to insert: an
/// origin, and the two directions sketch +x and +y point.
///
/// # Panics
///
/// If a component is not finite.
pub fn frame(origin: [f64; 3], u: [f64; 3], v: [f64; 3]) -> crate::AuthoredNode {
    Node::Datum(Datum::Frame {
        origin: origin.map(len),
        u: u.map(scl),
        v: v.map(scl),
    })
}

/// The world xy frame as a node — origin at the world origin, sketch
/// +x along world +x, sketch +y along world +y.
pub fn xy_frame() -> crate::AuthoredNode {
    frame([0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0])
}

// --- recipes ---------------------------------------------------------

/// **A cylinder split through a rim arc twice**: `circle(0, 0, 0.5)`
/// extruded 1.0, split by the plane through `(0, 0.2, 0)` with normal
/// `(0, 1, -1)`. The plane crosses the start rim arc twice and leaves
/// the wall and the start cap one piece per side, under `tol`. Returns the document
/// and the ids of the extrude, the plane datum and the split. No
/// oracle: the rows that read it state what it must evaluate to.
///
/// # Panics
///
/// If a door refuses an insert.
pub fn clipped_cylinder(tol: geom_core::Tol) -> (ProfileDoc, [RecipeNodeId; 3]) {
    let ins = |doc: ProfileDoc, node: crate::AuthoredNode| {
        let a = crate::apply(
            &doc,
            &DocEdit::InsertNode {
                node: Box::new(node),
                fresh: Vec::new(),
            },
            tol,
            &RefusingReach,
        )
        .expect("the clipped cylinder's inserts apply");
        (a.doc, a.record.minted.expect("an insert mints a node"))
    };
    let doc = ProfileDoc::empty_derived("clipped_cylinder", tol);
    let (doc, plane) = ins(doc, xy_frame());
    let (doc, profile) = ins(
        doc,
        Node::Profile(ProfileProgram {
            frame: plane.into(),
            loops: vec![LoopProgram::circle(0.0, 0.0, 0.5).expect("finite")],
            ids: Vec::new(),
        }),
    );
    let (doc, ext) = ins(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(1.0),
            side: crate::ExtrudeSide::Along,
        },
    );
    let (doc, tool) = ins(
        doc,
        Node::Datum(Datum::Plane {
            origin: [len(0.0), len(0.2), len(0.0)],
            normal: [scl(0.0), scl(1.0), scl(-1.0)],
        }),
    );
    let (doc, split) = ins(
        doc,
        Node::Split {
            target: ext.into(),
            tool: tool.into(),
        },
    );
    (doc, [ext, tool, split])
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
    /// `VerbatimEdge::Whole`: every body of the input, moved.
    Whole,
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
        VerbatimEdge::Whole { .. } => VerbatimKind::Whole,
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
