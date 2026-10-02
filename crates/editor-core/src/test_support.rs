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
    Datum, Dimension, DocEdit, Expr, HitTestError, LoopProgram, Node, PickHit, ProfileDoc,
    ProfileProgram, RecipeNodeId, RefusingReach,
};

// --- literals -------------------------------------------------------

/// A length literal, in canonical metres.
///
/// # Panics
///
/// If `metres` is not finite — the refusal is `Expr::literal`'s, and a
/// row about that refusal spells the call rather than reaching here.
pub fn len(metres: f64) -> Expr {
    Expr::literal(metres, Dimension::Length).expect("a finite length")
}

/// An angle literal, in radians.
///
/// # Panics
///
/// If `radians` is not finite.
pub fn ang(radians: f64) -> Expr {
    Expr::literal(radians, Dimension::Angle).expect("a finite angle")
}

/// A dimensionless literal — a direction component, a bulge, a ratio.
///
/// # Panics
///
/// If `value` is not finite.
pub fn scl(value: f64) -> Expr {
    Expr::literal(value, Dimension::Scalar).expect("a finite scalar")
}

/// Two length literals — a point in a sketch frame's own coordinates.
///
/// # Panics
///
/// If either coordinate is not finite.
pub fn len2(v: [f64; 2]) -> [Expr; 2] {
    [len(v[0]), len(v[1])]
}

// --- the frame a sketch is drawn on ---------------------------------

/// The frame datum a profile is drawn on, as a node to insert: an
/// origin, and the two directions sketch +x and +y point.
///
/// # Panics
///
/// If a component is not finite.
pub fn frame(origin: [f64; 3], u: [f64; 3], v: [f64; 3]) -> Node<ProfileProgram> {
    Node::Datum(Datum::Frame {
        origin: origin.map(len),
        u: u.map(scl),
        v: v.map(scl),
    })
}

/// The world xy frame as a node — origin at the world origin, sketch
/// +x along world +x, sketch +y along world +y.
pub fn xy_frame() -> Node<ProfileProgram> {
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
    let ins = |doc: ProfileDoc, node: Node<ProfileProgram>| {
        let a = crate::apply(
            &doc,
            &DocEdit::InsertNode {
                node: Box::new(node),
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
            plane,
            loops: vec![LoopProgram::circle(0.0, 0.0, 0.5).expect("finite")],
            ids: Vec::new(),
        }),
    );
    let (doc, ext) = ins(
        doc,
        Node::Extrude {
            profile,
            distance: len(1.0),
        },
    );
    let (doc, tool) = ins(
        doc,
        Node::Datum(Datum::Plane {
            origin: [len(0.0), len(0.2), len(0.0)],
            normal: [scl(0.0), scl(1.0), scl(-1.0)],
        }),
    );
    let (doc, split) = ins(doc, Node::Split { target: ext, tool });
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

// --- the mint's preimage --------------------------------------------

/// **The node id an insert of `node` draws from an empty document's
/// mint**: `Mint::insert`, lifted out of the crate so a row can pin the
/// preimage node shape by node shape
/// (`tests/switch_slots.rs`, `every_node_shapes_mint_is_pinned`).
///
/// Carries no oracle: it IS the mint's draw, with no document around
/// it, so a shape whose inputs name no live node still draws.
pub fn first_node_id(node: &Node<ProfileProgram>) -> RecipeNodeId {
    crate::Mint::empty()
        .insert(node)
        .expect("an empty log holds no id")
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

/// **A node as another document spells it** — the remapping split and
/// inline carry nodes with (`refactor::remap_node`), with gauge
/// references rewritten through `map` too and the world kept the world.
/// A round-trip comparator reads two documents through it.
///
/// # Errors
///
/// The id the maps lack, as a sentence.
pub fn remap_node(
    node: &Node<ProfileProgram>,
    map: &crate::refactor::NodeMap,
    steps: &crate::refactor::StepMap,
) -> Result<Node<ProfileProgram>, String> {
    use crate::refactor::RemapMiss;
    let regauge = |g: Option<RecipeNodeId>| match g {
        None => Ok(None),
        Some(g) => map.get(&g).copied().map(Some).ok_or(RemapMiss::Input(g)),
    };
    crate::refactor::remap_node(node, map, steps, &regauge).map_err(|miss| match miss {
        RemapMiss::Input(id) => format!("input {id} is unmapped"),
        RemapMiss::Name { name, missing } => {
            format!("name {name:?} reaches {missing:?}, which is unmapped")
        }
    })
}
