//! The die document fixture (spec D7): the M3 exact-oracle die —
//! 2×2×2 cube, 21 pips 0.25×0.25×`pip_depth` — authored as a recipe
//! through `apply`, evaluated by the M4 PR 2 service.
//!
//! REPORTED deviation from the PR 1 die authoring (spec D8): PR 1
//! placed all 21 pips from ONE master via rotational Transforms
//! (angles ±π/2, π). `sin`/`cos` of those angles are not exact in
//! f64, so a rotational placement cannot hit the dyadic volume oracle
//! bit-exactly. This document keeps the 21 interleaved
//! Transform+Subtract pairs but derives each face's pips from a
//! per-face master profile (6 masters), so every Transform is a
//! translation-only rigid map (rotation angle exactly 0 — an exact
//! identity in IEEE arithmetic) and the oracle stays exact. The
//! rotational Transform path is exercised separately (non-dyadic
//! assertions) in the wire tests.
//!
//! **This file is the ONE home for what a name-reading suite works an
//! evaluation with**: the die document above, the [`Recorder`] and the
//! `insert`/`step` authoring shorthands a suite builds a document
//! with, the name-authoring shorthands ([`minted`], [`fname`],
//! [`ename`], [`vname`], [`rim_edge`], [`cap_vertex`], [`pole`],
//! [`in_copy`]) and, below the banner, the reader doors over a
//! published table and body ([`table`], [`key_of`], [`face_of`],
//! [`edge_of`], [`vertex_of`], [`count`], [`point`], [`ends`],
//! [`face_vertices`], [`face_edges`]). A suite **imports a door; it
//! never copies one** — a copy diverges silently, and the divergence
//! is discovered by the row it breaks rather than by the reader of
//! either file. Where the door does not fit, the suite either widens
//! the door here or writes an adapter that DELEGATES to it; an
//! adapter never reuses a door's name, because a door's name in a
//! suite means the door.
#![allow(dead_code)]
// one instance per binary; no single consumer uses all of it
// WHY A HELPER TREE ALLOWS THESE — the one statement of it, cited by every
// other tree that carries an allow of this shape. A helper tree AUTHORS test
// documents, and a document that will not build is a test failure, not a
// value to hand back: its builders panic on a malformed fixture rather than
// thread a `Result` out to a caller whose only recourse is to unwrap it.
// Each tree names exactly the lints its own code raises, here rather than in
// the crate-root allow of whatever module loads it.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![allow(unreachable_pub)] // why: root Cargo.toml, the `unreachable_pub` stanza

/// The provenance-extended evaluation digest the verb-migration suites
/// pin their documents with — one feed, per-suite constants.
pub mod digest;

/// The part store an assembly suite instantiates through, and the
/// names an instantiated part's faces are spelled with.
pub mod resolver;

/// The comparator a split-then-inline round trip is held to: one
/// document up to node ids.
pub mod round_trip;

/// The whole-frame product oracle a mate suite measures a seat with.
pub mod seat;

/// The value-channel digest a cross-scalar differential reads — the one
/// feed behind every "bit-identical to the `f64` run" claim in this tree.
pub mod value_channel;

use editor_core::AuthoredNode;
use editor_core::{
    AssemblyError, CancelToken, CapEnd, Dimension, DocEdit, EntityKey, EntityKind, Entry,
    EvalOptions, Evaluation, Formula, FreeVar, LoopProgram, NameTable, Node, ProfileDoc,
    ProfileEdgeRef, ProfilePieces, ProfileProgram, ProfileVertexRef, RecipeNodeId, RoleSeg,
    SitedRef, SolvedPoses, StableName, VarId, VarName, assemble, evaluate, solve_document,
};
use geom_core::{Point3, Tol};
use std::collections::HashSet;
use topo::{Body, EdgeKey, FaceKey, LoopBoundary, VertexKey};

/// **The evaluation, through the ordinary door** — `evaluate` at
/// `f64` with a fresh cancel token and the witness tolerance, which
/// is what every suite here wants and what none of them should spell
/// for itself.
///
/// A document with a mate between copies is also checked for where the
/// mate solve's decisions land: the fold runs inside its placement's
/// op, so the evaluation records none outside its nodes. What it
/// recorded there still reaches the caller's frame.
pub fn run(doc: &editor_core::ProfileDoc, o: &EvalOptions) -> Evaluation<f64> {
    let (ev, outside) = geom_core::k_stats::detached(|| {
        evaluate::<f64>(doc, None, &CancelToken::new(), o, Tol::witness())
    });
    if !constrained(doc).is_empty() {
        let recorded = outside.recorded();
        assert!(
            recorded.escalations.is_empty() && recorded.verdicts.is_empty(),
            "document {}: the evaluation recorded outside its nodes: {recorded:?}",
            doc.id()
        );
    }
    geom_core::k_stats::splice(outside);
    ev
}

/// **Every placement whose bundle holds a mate to a pose of its
/// space**, in document order: the placements mated to other copies.
pub fn constrained(doc: &ProfileDoc) -> Vec<RecipeNodeId> {
    doc.placements()
        .into_iter()
        .filter(|&p| {
            bundle(doc, p).iter().any(|c| {
                c.mate()
                    .is_some_and(|m| matches!(m.to, editor_core::Target::Pose(_)))
            })
        })
        .collect()
}

/// **A placement's bundle**, empty for a node that is none.
pub fn bundle(doc: &ProfileDoc, placement: RecipeNodeId) -> Vec<editor_core::Constraint> {
    match doc.node(placement) {
        Some(Node::Place { constraints, .. } | Node::InstantiatePart { constraints, .. }) => {
            constraints.clone()
        }
        _ => Vec::new(),
    }
}

/// **The constraint `mate` names**, as its placement holds it.
pub fn constraint(doc: &ProfileDoc, mate: editor_core::MateRef) -> editor_core::Mate {
    bundle(doc, mate.placement)
        .into_iter()
        .find(|c| c.id == mate.id)
        .and_then(|c| c.mate().cloned())
        .unwrap_or_else(|| panic!("{mate:?} is no mate of its placement"))
}

/// **A face read as its plane** (DM1a: its carrier's plane, with the
/// face's outward normal), as a pose a mate equates.
pub fn plane(face: impl Into<editor_core::Operand>) -> editor_core::Operand {
    editor_core::Operand::Pose(Box::new(editor_core::pose::PoseDef::Plane {
        face: face.into(),
    }))
}

/// **The plane `by` up an extrude's own `z`**: its start cap's plane,
/// flipped to face up its `z`, slid along that normal by `by` — a
/// split's tool at a height the document can drive (`by` a named
/// variable) where a datum plane stood at that height.
pub fn start_cap_at(
    extrude: RecipeNodeId,
    by: Formula,
) -> editor_core::pose::RoleRead<editor_core::Operand> {
    use editor_core::pose::{PoseDef, PoseRole, RoleRead};
    let up = || {
        editor_core::Operand::Pose(Box::new(PoseDef::Flip {
            pose: RoleRead::plain(plane(editor_core::Operand::select(
                extrude,
                vec![fname(extrude, RoleSeg::Cap(CapEnd::Start))],
            ))),
        }))
    };
    RoleRead::plain(editor_core::Operand::Pose(Box::new(PoseDef::Slide {
        pose: RoleRead::plain(up()),
        along: RoleRead::through(up(), PoseRole::Normal),
        by,
    })))
}

/// **An axis read off a face or an edge** (a carrier's axis, an edge's
/// line).
pub fn axis(of: impl Into<editor_core::Operand>) -> editor_core::Operand {
    editor_core::Operand::Pose(Box::new(editor_core::pose::PoseDef::Axis { of: of.into() }))
}

/// **A point read off a face, an edge or a vertex** (a carrier's centre,
/// a vertex's point).
pub fn point_of(of: impl Into<editor_core::Operand>) -> editor_core::Operand {
    editor_core::Operand::Pose(Box::new(editor_core::pose::PoseDef::Point {
        of: of.into(),
    }))
}

/// **A pose's opposite sense** ([`editor_core::pose::PoseDef::Flip`]).
pub fn flip(pose: editor_core::Operand) -> editor_core::Operand {
    editor_core::Operand::Pose(Box::new(editor_core::pose::PoseDef::Flip {
        pose: editor_core::pose::RoleRead::plain(pose),
    }))
}

/// **A rest of face `on` against face `to`** (A5, FORK-S3M): `on`'s
/// plane equated with `to`'s flipped, so the two faces face each other
/// — a plain plane mate equates outward normals, and stacking carries a
/// flip. `on` reads the copy the mate places.
pub fn rest(
    on: impl Into<editor_core::Operand>,
    to: impl Into<editor_core::Operand>,
    class: editor_core::ContactClass,
) -> editor_core::Mate<Formula> {
    editor_core::Mate {
        on: plane(on),
        to: editor_core::Target::Pose(flip(plane(to))),
        class,
    }
}

/// **A mate equating two poses** as written, `on` reading the copy.
pub fn mate(
    on: editor_core::Operand,
    to: editor_core::Operand,
    class: editor_core::ContactClass,
) -> editor_core::Mate<Formula> {
    editor_core::Mate {
        on,
        to: editor_core::Target::Pose(to),
        class,
    }
}

/// **The placement whose copy an authored face side reads**: the node
/// its selection's body names, where that is a placement.
pub fn side_placement(doc: &ProfileDoc, side: &editor_core::Operand) -> Option<RecipeNodeId> {
    let editor_core::Operand::Select { body, .. } = side else {
        return None;
    };
    let at = match body.as_ref() {
        editor_core::Operand::Node(at) | editor_core::Operand::Output { node: at, .. } => *at,
        editor_core::Operand::Var(var) => doc.operation_of(*var)?,
        editor_core::Operand::Name(name) => doc.operation_of(doc.var_named(name.as_str())?)?,
        _ => return None,
    };
    matches!(
        doc.node(at),
        Some(Node::Place { .. } | Node::InstantiatePart { .. })
    )
    .then_some(at)
}

/// Whether `placement`'s bundle reads the world.
pub fn on_world(doc: &ProfileDoc, placement: RecipeNodeId) -> bool {
    bundle(doc, placement).iter().any(|c| {
        c.mate()
            .is_some_and(|m| matches!(m.to, editor_core::Target::World(_)))
    })
}

/// **The selections an authored pose reads**, in read order: each
/// `Select` inside it, through every pose construction.
pub fn selections_in(pose: &editor_core::Operand) -> Vec<&editor_core::Operand> {
    fn walk<'a>(op: &'a editor_core::Operand, out: &mut Vec<&'a editor_core::Operand>) {
        match op {
            editor_core::Operand::Select { .. } => out.push(op),
            editor_core::Operand::Pose(def) => {
                for (_, read, _, _) in def.reads() {
                    walk(read, out);
                }
            }
            _ => {}
        }
    }
    let mut out = Vec::new();
    walk(pose, &mut out);
    out
}

/// **The placement a mate places**: the one whose copy its `on` reads —
/// an instance whose output a selection in it reads, else the one
/// placement copying the body it reads.
pub fn placed_by(doc: &ProfileDoc, mate: &editor_core::Mate<Formula>) -> RecipeNodeId {
    for select in selections_in(&mate.on) {
        if let Some(at) = side_placement(doc, select) {
            return at;
        }
        let editor_core::Operand::Select { body, .. } = select else {
            continue;
        };
        let read = match body.as_ref() {
            editor_core::Operand::Node(at) | editor_core::Operand::Output { node: at, .. } => {
                Some(*at)
            }
            editor_core::Operand::Var(var) => doc.operation_of(*var),
            _ => None,
        };
        let copying: Vec<RecipeNodeId> = doc
            .placements()
            .into_iter()
            .filter(|&p| {
                matches!(doc.node(p), Some(Node::Place { shapes, .. })
                    if shapes.iter().any(|s| doc.operation_of(s.read) == read))
            })
            .collect();
        if let [one] = copying.as_slice() {
            return *one;
        }
    }
    panic!("name the placement a mate places (`constrain_at`): {mate:?}")
}

/// **`mate` added to the bundle of the copy it places** ([`placed_by`]),
/// the constraint by its address.
pub fn constrain(
    doc: ProfileDoc,
    mate: editor_core::Mate<Formula>,
) -> (ProfileDoc, editor_core::MateRef) {
    let placement = placed_by(&doc, &mate);
    constrain_at(doc, placement, mate)
}

/// [`constrain`] onto `placement` as named.
pub fn constrain_at(
    doc: ProfileDoc,
    placement: RecipeNodeId,
    mate: editor_core::Mate<Formula>,
) -> (ProfileDoc, editor_core::MateRef) {
    let applied = editor_core::apply(
        &doc,
        &DocEdit::AddConstraint {
            placement,
            mate: Box::new(mate),
            fresh: Vec::new(),
        },
        Tol::witness(),
    )
    .unwrap_or_else(|e| panic!("the constraint is admitted: {e}"));
    let id = applied.record.minted_constraints[0];
    (applied.doc, editor_core::MateRef::new(placement, id))
}

/// **The mate solve, through the ordinary door**, over the parts `o`'s
/// resolver reaches. A row that solves with a store it does not hand
/// here levers nothing: every mate on a part faults in the resolver's
/// voice, which is the kernel's answer and not a fixture default.
pub fn solve(doc: &editor_core::ProfileDoc, o: &EvalOptions, tol: Tol) -> SolvedPoses {
    solve_document(doc, o, tol)
}

/// **Every mate-placed copy pinned to the world where its mates solve
/// it** — the edits a user makes to keep a part where it is shown
/// before removing what places it: each placement whose bundle does not
/// read the world gets its bundle replaced by one world pin at its
/// solved pose.
pub fn pinned_where_solved(doc: ProfileDoc, o: &EvalOptions) -> ProfileDoc {
    let poses = solve(&doc, o, Tol::witness());
    let placed: Vec<(RecipeNodeId, editor_core::Frame)> = doc
        .placements()
        .into_iter()
        .filter(|&p| !bundle(&doc, p).is_empty() && !on_world(&doc, p))
        .filter_map(|p| Some((p, poses.placement(&doc, p).ok()?)))
        .collect();
    let mut doc = doc;
    for (placement, frame) in placed {
        // Each mate with the values naming it, as the façades remove one.
        for c in bundle(&doc, placement) {
            if c.value().is_some() {
                continue;
            }
            for edit in DocEdit::removing(&doc, editor_core::MateRef::new(placement, c.id)) {
                doc = step(doc, edit).0;
            }
        }
        doc = pin(doc, placement, &frame, o);
    }
    doc
}

/// **The copy `placement` defines, pinned to the world at `pose`**:
/// at the identity standing where its body stands
/// ([`editor_core::stand`]: a plane mate on a face no variable reaches
/// and the constant `0` on its freedoms); elsewhere through the one
/// migration helper ([`editor_core::pin_at`]), a frame read off the
/// copy's geometry equated with the world's frame moved to where that
/// frame sits at `pose`. A `Place` is read through its first shape, an
/// instance through its own copies; `o` evaluates them.
pub fn pin(
    doc: ProfileDoc,
    placement: RecipeNodeId,
    pose: &editor_core::Frame,
    o: &EvalOptions,
) -> ProfileDoc {
    if *pose == editor_core::Frame::IDENTITY {
        let copy = copy_read(&doc, placement);
        let stood = editor_core::stand(&doc, &copy, o, Tol::witness())
            .unwrap_or_else(|refusal| panic!("the copy stands: {refusal}"));
        return written(doc, placement, stood);
    }
    let mate = pin_mate(&doc, placement, pose, o);
    constrain_at(doc, placement, mate).0
}

/// **`placement`'s copy as its constraints read it**: a `Place` through
/// its first shape, an instance through its own copies.
fn copy_read(doc: &ProfileDoc, placement: RecipeNodeId) -> editor_core::Operand {
    match doc.node(placement) {
        Some(Node::Place { shapes, .. }) => {
            editor_core::Operand::Var(shapes.first().expect("a placement copies a shape").read)
        }
        Some(Node::InstantiatePart { .. }) => editor_core::Operand::Var(
            doc.output(placement, 0)
                .expect("an instance defines its copies"),
        ),
        other => panic!("node {} is a placement, got {other:?}", placement.0),
    }
}

/// **A gesture's constraints written onto `placement`**
/// ([`editor_core::Placed::write`]): the mate and the values naming it,
/// or the values of a bundle with no mate as one write.
pub fn written(
    doc: ProfileDoc,
    placement: RecipeNodeId,
    placed: editor_core::Placed,
) -> ProfileDoc {
    let mut doc = doc;
    placed
        .write(placement, |edit| {
            let applied = doc.apply(&edit, Tol::witness()).unwrap();
            doc = applied.doc;
            Ok::<_, std::convert::Infallible>(applied.record.minted_constraints)
        })
        .unwrap_or_else(|never| match never {});
    doc
}

/// **`mate` and the `values` naming it, added to `placement`**.
pub fn with_values(
    doc: ProfileDoc,
    placement: RecipeNodeId,
    mate: editor_core::Mate<Formula>,
    values: Vec<(editor_core::Coord, Formula)>,
) -> ProfileDoc {
    let (doc, id) = constrain_at(doc, placement, mate);
    values.into_iter().fold(doc, |doc, (coord, by)| {
        step(
            doc,
            DocEdit::AddValue {
                placement,
                value: Box::new(editor_core::Value {
                    mate: Some(id.id),
                    coord,
                    by,
                }),
                fresh: Vec::new(),
            },
        )
        .0
    })
}

/// **The mate [`pin`] adds**, authored and not yet added.
pub fn pin_mate(
    doc: &ProfileDoc,
    placement: RecipeNodeId,
    pose: &editor_core::Frame,
    o: &EvalOptions,
) -> editor_core::Mate<Formula> {
    let copy = copy_read(doc, placement);
    match editor_core::pin_at(doc, &copy, pose, o, Tol::witness()) {
        Ok(mate) => mate,
        // A copy that does not evaluate has no geometry to read a frame
        // off: it is still placed in the world, by a rest of the cap its
        // body would mint on the world's floor, which its placement
        // never reaches (the placement is poisoned through its shape).
        Err(
            editor_core::PinRefusal::Unevaluated { node }
            | editor_core::PinRefusal::NotABody { node },
        ) => editor_core::Mate {
            on: plane(editor_core::Operand::select(
                copy.clone(),
                vec![fname(node, RoleSeg::Cap(editor_core::CapEnd::Start))],
            )),
            to: editor_core::Target::World(
                editor_core::WorldRead::through(editor_core::pose::PoseRole::Xy).flip(),
            ),
            class: editor_core::ContactClass::Rest,
        },
        Err(refusal) => panic!("the copy pins over its geometry: {refusal}"),
    }
}

/// [`pin`] at the identity.
pub fn pinned(doc: ProfileDoc, placement: RecipeNodeId, o: &EvalOptions) -> ProfileDoc {
    pin(doc, placement, &editor_core::Frame::IDENTITY, o)
}

/// **`node` inserted and pinned to the world at the identity**
/// ([`pinned`]): an instance or a placement that stands where its
/// shapes do, in the product.
pub fn insert_pinned(
    doc: ProfileDoc,
    node: AuthoredNode,
    o: &EvalOptions,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, id) = insert(doc, node);
    (pinned(doc, id, o), id)
}

/// **The at-rest gate's verdict**, as a mate row wants to read it:
/// whether the assembly mints, with the minted records dropped.
///
/// # Errors
///
/// The gate's own refusal, unaltered.
pub fn gate(doc: &editor_core::ProfileDoc, ev: &Evaluation<f64>) -> Result<(), AssemblyError> {
    assemble(doc, ev, Tol::witness()).map(|_| ())
}

/// **What the gather minted for `doc`'s own mates**: the declarations,
/// and the rows it could not mint (`Product::minted`, `unminted`).
pub fn mate_mints(
    doc: &ProfileDoc,
    ev: &Evaluation<f64>,
) -> (
    Vec<editor_core::MintedDeclaration>,
    Vec<editor_core::MintRefusal>,
) {
    let p = editor_core::product_recorded(doc, ev, Tol::witness()).expect("the product gathers");
    (p.minted, p.unminted)
}

/// **Asserts the gather minted nothing for `mate`** — no declaration and
/// no refusal: a mate side whose body no world placement reads has no
/// world copy, so the mate states nothing about the product.
pub fn assert_mints_nothing(doc: &ProfileDoc, ev: &Evaluation<f64>, mate: editor_core::MateRef) {
    let (minted, unminted) = mate_mints(doc, ev);
    assert!(
        minted.iter().all(|d| d.mate != mate) && unminted.iter().all(|r| r.mate() != mate),
        "a mate whose side's body no placement reads mints nothing: {minted:?} {unminted:?}"
    );
}

/// A mate side over a name this fixture built as a face, read at its
/// own mint (`editor_core::SitedFace`, which lowers to the `Face`
/// selection of the name in its minting node's body).
///
/// The `expect` in [`face`] is the fixture's own claim that the name it
/// just made is a face — if it is not, the fixture is wrong and says so
/// where it is built.
pub fn head(name: StableName) -> editor_core::SitedFace {
    editor_core::SitedFace::at_mint(face(name))
}

/// The same side read at `at` rather than at its mint.
pub fn head_at(at: RecipeNodeId, name: StableName) -> editor_core::SitedFace {
    editor_core::SitedFace::new(at, face(name))
}

/// **`name` as the union `union` carries it from its member `member`**
/// — the name a read at the union says for that member's entity: one
/// `From` segment keyed by the member's read in `doc`.
pub fn member_name(
    doc: &ProfileDoc,
    union: RecipeNodeId,
    member: RecipeNodeId,
    name: StableName,
) -> StableName {
    StableName {
        kind: name.kind,
        node: union,
        path: vec![editor_core::RoleSeg::From {
            read: out(doc, member),
            of: name.into(),
        }],
    }
}

/// **The face name a document's mate side selects** — the one name of
/// the side's `Face` selection.
pub fn side_name(doc: &ProfileDoc, side: editor_core::VarId) -> StableName {
    doc.selection(side)
        .expect("a mate side is a selection")
        .names[0]
        .clone()
}

/// **The head an authored mate side spells** — the node its selection
/// is read in and its one face name, for a side authored as a
/// selection at a node (`SitedFace`'s lowering).
pub fn authored_head(side: &editor_core::Operand) -> editor_core::SitedFace {
    let editor_core::Operand::Select { body, names } = side else {
        panic!("an authored side is a selection, got {side:?}")
    };
    let editor_core::Operand::Node(at) = **body else {
        panic!("an authored side is read at a node, got {body:?}")
    };
    let [name] = names.as_slice() else {
        panic!("a side selects one face")
    };
    editor_core::SitedFace::new(at, face(name.clone()))
}

/// **The head a document's mate side reads** — its selection's body
/// operation and its face name (`editor_core::head_of`).
pub fn side_head(doc: &ProfileDoc, side: editor_core::VarId) -> editor_core::SitedFace {
    editor_core::head_of(doc, side).expect("the side reads a face of a live body")
}

/// **Every face side that resolves to a member, as `(mate, instance)`**:
/// each constraint in placement and bundle order, `a` before `b`,
/// landing on the member's instance. Which instance each side's member
/// walk lands on — the solve's question, not a dependency: a constraint
/// depends on what its sides read, as every reader does.
pub fn mate_members(doc: &ProfileDoc) -> Vec<(editor_core::MateRef, RecipeNodeId)> {
    let mut out = Vec::new();
    for placement in doc.placements() {
        for c in bundle(doc, placement) {
            let Some(mate) = c.mate() else { continue };
            let sides = std::iter::once(mate.on).chain(mate.to.pose().copied());
            for side in sides.flat_map(|side| face_reads(doc, side)) {
                if let Some(m) = editor_core::head_of(doc, side)
                    .and_then(|head| editor_core::member_of(doc, &head))
                {
                    out.push((editor_core::MateRef::new(placement, c.id), m.instance));
                }
            }
        }
    }
    out
}

/// **The selections a pose variable's definition reads**, through every
/// construction, in read order.
pub fn face_reads(doc: &ProfileDoc, pose: VarId) -> Vec<VarId> {
    fn walk(doc: &ProfileDoc, var: VarId, out: &mut Vec<VarId>) {
        if doc.selection(var).is_some() {
            out.push(var);
            return;
        }
        if let Some(editor_core::VarDef::Pose(def)) = doc.var(var).map(editor_core::Var::def) {
            for (_, &read, _, _) in def.reads() {
                walk(doc, read, out);
            }
        }
    }
    let mut out = Vec::new();
    walk(doc, pose, &mut out);
    out
}

/// A fixture's name as an `editor_core::FaceName`.
pub fn face(name: StableName) -> editor_core::FaceName {
    editor_core::FaceName::new(name).expect("the fixture names a face")
}

/// **A name worn as copy `i` of `pattern`** — one `Instance(i)`
/// wrapper, the segment a pattern's table puts round every master
/// name it emits. Nest the calls for a nested copy.
pub fn in_copy(pattern: RecipeNodeId, i: u32, of: StableName) -> StableName {
    StableName {
        kind: of.kind,
        node: pattern,
        path: vec![RoleSeg::Instance { i, of: of.into() }],
    }
}

/// **A copy of `input` moved within its own space**: rotated by
/// `angle` about `axis` through its own origin, then translated — a
/// placement of `input` against its own geometry
/// ([`editor_core::test_support::against`]), since nothing moves a
/// body. Returns the document and the placement, whose output is the
/// copy.
///
/// # Panics
///
/// If the copy offers no geometry to place by.
pub fn xform(
    doc: ProfileDoc,
    input: RecipeNodeId,
    translation: [f64; 3],
    axis: [f64; 3],
    angle: f64,
) -> (ProfileDoc, RecipeNodeId) {
    let at = editor_core::Frame::rotate_then_translate(axis, angle, translation, band())
        .expect("a rotation axis with a direction");
    against(doc, input, input, &at)
}

/// The pip depth the document's `pip_depth` parameter starts at.
pub const DEPTH: f64 = 0.125;
/// The exact die volume oracle at `DEPTH` (M3).
pub const DIE_VOLUME: f64 = 7.8359375;

/// **The witnessed band a placement axis is decided under** — what
/// `Frame::rotate_then_translate` asks the direction door with. Rows
/// whose axis is a literal pass this and unwrap; a row whose SUBJECT
/// is the axis decision reads the refusal instead.
/// The K funnel name a fixture decides a mate-frame axis or normal
/// under when a row builds a witness by hand — one name the fixtures
/// own, rostered in `docs/K-REPORT.md` beside the other fixture
/// mints, so a suite never names a production funnel for a decision
/// no door made.
pub const FIXTURE_MATE_AXIS: &str = "fixture_mate_axis";

pub fn band() -> geom_core::Band {
    geom_core::Band::linear(Tol::witness()).expect("the witnessed band")
}

/// The literal of each dimension, a point of two lengths, and the
/// frame a sketch is drawn on — `editor_core::test_support`'s, which the
/// crate's own unit-test modules read too, re-exported so a suite
/// imports them from here beside the rest of its authoring doors.
// Not every suite that mounts the fixture reads each one.
#[allow(unused_imports)]
pub use editor_core::test_support::{
    against, ang, axis_on, frame_at, in_world, len, len2, plane_on, scl,
};

/// Whether two placements are pinned to the world by the same world
/// read, bit for bit as written (or neither is one world pin).
pub fn same_offset(a: &ProfileDoc, ai: RecipeNodeId, b: &ProfileDoc, bi: RecipeNodeId) -> bool {
    offset_of(a, ai) == offset_of(b, bi)
}

/// **An instance that sits where its mates put it**: its bundle empty,
/// a loose copy until the constraints added to it pin it.
pub fn mated_instance(doc_ref: editor_core::DocRef) -> AuthoredNode {
    Node::instantiate_part_with(doc_ref, editor_core::InterfaceRecord::default())
}

/// **The world read a placement is pinned at**, as written, where its
/// bundle's one mate is to the world ([`pin`]); `None` for any other
/// bundle. Panics on a node that is no placement.
pub fn offset_of(doc: &ProfileDoc, id: RecipeNodeId) -> Option<editor_core::WorldRead<Formula>> {
    let constraints = match doc.node(id) {
        Some(Node::InstantiatePart { constraints, .. } | Node::Place { constraints, .. }) => {
            constraints
        }
        other => panic!("node {} is a placement, got {other:?}", id.0),
    };
    let mut mates = constraints.iter().filter_map(editor_core::Constraint::mate);
    let (Some(only), None) = (mates.next(), mates.next()) else {
        return None;
    };
    let editor_core::Target::World(world) = &only.to else {
        return None;
    };
    let dims: Vec<(bool, Dimension)> = world
        .scalars()
        .into_iter()
        .map(|(spin, _)| {
            (
                spin,
                if spin {
                    Dimension::Angle
                } else {
                    Dimension::Length
                },
            )
        })
        .collect();
    let mut k = 0;
    world
        .try_map(&mut |&var| {
            let dim = dims[k].1;
            k += 1;
            Ok::<_, core::convert::Infallible>(doc.written(&editor_core::Expr::var(var, dim)))
        })
        .ok()
}

/// Applies an edit, returning the new doc and any minted id.
pub fn step(doc: ProfileDoc, edit: DocEdit<ProfileProgram>) -> (ProfileDoc, Option<RecipeNodeId>) {
    let applied = doc.apply(&edit, Tol::witness()).unwrap();
    (applied.doc, applied.record.minted)
}

/// `doc` with every node labelled `text`. Labels are outside every
/// content key, so the relabelled document builds the same bits and
/// differs only in what it speaks: a test that must tell two documents
/// apart by their sentences relabels one of them.
pub fn label_every_node(doc: ProfileDoc, text: &str) -> ProfileDoc {
    let label = editor_core::Label::new(text).expect("a valid label");
    let order = doc.ids().to_vec();
    order.into_iter().fold(doc, |doc, node| {
        step(
            doc,
            DocEdit::SetLabel {
                node,
                label: Some(label.clone()),
            },
        )
        .0
    })
}

/// **A copy of `body` slid by `by` in `target`'s own coordinates**
/// ([`against`] at a translation): how a construction that sat on a
/// datum off its target's origin joins the target's space. Returns the
/// document and the placement, whose output is the copy.
pub fn slid(
    doc: ProfileDoc,
    target: impl Into<editor_core::Operand>,
    body: impl Into<editor_core::Operand>,
    by: [f64; 3],
) -> (ProfileDoc, RecipeNodeId) {
    against(doc, target, body, &editor_core::Frame::translation(by))
}

/// **The slot of `placement`'s value on `coord`**: the address a row
/// revalues a placed copy's slide or spin through.
///
/// # Panics
///
/// If `placement` is no placement or holds no value on `coord`.
pub fn value_slot(
    doc: &ProfileDoc,
    placement: RecipeNodeId,
    coord: editor_core::Coord,
) -> editor_core::SlotId {
    let Some(Node::Place { constraints, .. }) = doc.node(placement) else {
        panic!("{placement} is a placement");
    };
    let c = constraints
        .iter()
        .find(|c| c.value().is_some_and(|v| v.coord == coord))
        .unwrap_or_else(|| panic!("{placement} holds a value on {coord:?}"));
    match coord {
        editor_core::Coord::Slide(_) => editor_core::SlotId::ValueSlide { constraint: c.id },
        editor_core::Coord::Spin | editor_core::Coord::SpinAbout(_) => {
            editor_core::SlotId::ValueSpin { constraint: c.id }
        }
    }
}

/// **An edit moving `placement`'s value on `coord` by `delta`**: the
/// value is a typed number, read back from its variable, and written
/// again `delta` further — a revaluation a row bumps a placed copy by.
///
/// # Panics
///
/// If the value is no typed number ([`value_slot`]'s panics besides).
pub fn value_bump(
    doc: &ProfileDoc,
    placement: RecipeNodeId,
    coord: editor_core::Coord,
    delta: f64,
) -> DocEdit<ProfileProgram> {
    let slot = value_slot(doc, placement, coord);
    let Some(Node::Place { constraints, .. }) = doc.node(placement) else {
        unreachable!("value_slot found a placement")
    };
    let by = constraints
        .iter()
        .find_map(|c| c.value().filter(|v| v.coord == coord).map(|v| v.by))
        .unwrap_or_else(|| unreachable!("value_slot found the value"));
    let held = match doc.free(by) {
        Some(editor_core::FreeVar::Continuous { value, .. }) => *value,
        other => panic!("{placement}'s value on {coord:?} is a typed number, not {other:?}"),
    };
    let value = match coord {
        editor_core::Coord::Slide(_) => len(held + delta),
        editor_core::Coord::Spin | editor_core::Coord::SpinAbout(_) => ang(held + delta),
    };
    DocEdit::SetParam {
        node: placement,
        slot,
        value: value.into(),
        fresh: Vec::new(),
    }
}

/// **The id `doc`'s next insert would mint**, read by making that
/// insert on a copy: an id no node of `doc` holds, whatever the mint.
pub fn next_mint(doc: &ProfileDoc) -> RecipeNodeId {
    insert(
        doc.clone(),
        Node::Profile(desc(vec![square(0.0, 0.0, 1.0)])),
    )
    .1
}

/// **The last node in `doc.order()`** — the one a just-applied
/// `InsertNode` minted, for a row that pushes an insert through a door
/// that hands back only the document.
///
/// # Panics
///
/// If `doc` holds no live node.
pub fn newest(doc: &ProfileDoc) -> RecipeNodeId {
    *doc.ids().last().expect("the document holds a node")
}

pub fn insert(doc: ProfileDoc, node: AuthoredNode) -> (ProfileDoc, RecipeNodeId) {
    let (doc, minted) = step(
        doc,
        DocEdit::InsertNode {
            node: Box::new(node),
            fresh: Vec::new(),
        },
    );
    (doc, minted.unwrap())
}

/// **A primitive over indices into `refs`, sited**: the rows' spelling
/// of a measure over references they list once.
///
/// # Panics
///
/// When an index is past the end of `refs`.
pub fn sited<R: Clone>(
    p: editor_core::MeasurePrimitive<u32>,
    refs: &[R],
) -> editor_core::MeasurePrimitive<R> {
    p.try_map(|&i| refs.get(i as usize).cloned().ok_or(i))
        .unwrap_or_else(|i| panic!("reference {i} of {}", refs.len()))
}

/// **Measures, inserted** ([`editor_core::measure`]): one per primitive,
/// each [`sited`] over `refs`; the document after them, and the
/// measures with their outputs. A refusal is a loud test failure.
pub fn measure<R: Clone + Into<editor_core::Operand>>(
    doc: ProfileDoc,
    primitives: &[editor_core::MeasurePrimitive<u32>],
    refs: &[R],
) -> (ProfileDoc, editor_core::Measured) {
    let sited: Vec<_> = primitives.iter().map(|&p| sited(p, refs)).collect();
    let out = editor_core::measure(&doc, &sited, Tol::witness()).expect("the measures insert");
    (out.doc, out.measured)
}

/// **One measure inserted**: the document after it, and the measure
/// node.
pub fn measure_node<R: Clone + Into<editor_core::Operand>>(
    doc: &ProfileDoc,
    primitive: editor_core::MeasurePrimitive<u32>,
    refs: Vec<R>,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, measured) = measure(doc.clone(), &[primitive], &refs);
    (doc, measured.measures[0])
}

/// **A reader of `measure`'s value**, authored: the formula an
/// assertion's `value` holds to bound one measure's output.
///
/// # Panics
///
/// When `measure` defines no scalar in `doc`.
pub fn value_of(doc: &ProfileDoc, measure: RecipeNodeId) -> Formula {
    let var = doc.output(measure, 0).expect("a measure defines its value");
    let dim = doc
        .var(var)
        .and_then(|v| v.kind().dimension())
        .expect("a measure's value is a scalar");
    Formula::var(var, dim)
}

/// **A reader of `var`**, authored at its kind's dimension: the formula
/// an assertion's `value` holds to bound a measured value.
///
/// # Panics
///
/// When `var` is not a scalar of `doc`.
pub fn read_var(doc: &ProfileDoc, var: editor_core::VarId) -> Formula {
    let dim = doc
        .var(var)
        .and_then(|v| v.kind().dimension())
        .expect("a scalar variable");
    Formula::var(var, dim)
}

/// **`var`'s value in `ev`** ([`editor_core::Evaluation::reading`]):
/// `None` when it has none at this scalar or refuses.
pub fn reading<T: geom_core::Decide>(
    doc: &ProfileDoc,
    ev: &editor_core::Evaluation<T>,
    var: editor_core::VarId,
) -> Option<T> {
    match ev.reading(doc, var) {
        Ok(editor_core::Observed::Value(v)) => Some(v),
        _ => None,
    }
}

/// **`measure`'s value**: the variable its one port defines.
///
/// # Panics
///
/// When `measure` defines nothing in `doc`.
pub fn output(doc: &ProfileDoc, measure: RecipeNodeId) -> editor_core::VarId {
    doc.output(measure, 0).expect("a measure defines its value")
}

/// **The variable an assertion reads**: its measured value, for a row
/// that asks a stackup or a reading of it.
///
/// # Panics
///
/// When `assertion` is not a live assertion of `doc`.
pub fn assertion_value(doc: &ProfileDoc, assertion: RecipeNodeId) -> editor_core::VarId {
    match doc.node(assertion) {
        Some(Node::Assertion { value, .. }) => *value,
        other => panic!("{assertion} is not an assertion: {other:?}"),
    }
}

/// **One copy of `body` in the world** (A10), at the identity: the
/// document's product is the copies its placements define.
pub fn place(doc: ProfileDoc, body: RecipeNodeId) -> (ProfileDoc, RecipeNodeId) {
    let read = body_read(&doc, body.into());
    let (doc, placement) = insert(doc, Node::place(vec![editor_core::BodyRead::plain(read)]));
    (pinned(doc, placement, &EvalOptions::default()), placement)
}

/// **A read of the copy of `body` an instance delivers** — `instance`'s
/// copies, keyed by the copy's identity in its part
/// ([`resolver::copy_key`]): what a seat reading one body of an instance
/// reads.
pub fn copy(instance: RecipeNodeId, body: RecipeNodeId) -> editor_core::BodyRead<Formula> {
    editor_core::BodyRead::keyed(
        editor_core::Operand::Node(instance),
        resolver::copy_key(body),
    )
}

/// **The body read a placement of `body` names**: a revolve named
/// alone reads its `body` port (Q5 refuses a several-output node named
/// alone, and a revolve's other port is its axis); any other operand as
/// it is, so a split named alone still refuses.
pub fn body_read(doc: &ProfileDoc, body: editor_core::Operand) -> editor_core::Operand {
    match body {
        editor_core::Operand::Node(id) if matches!(doc.node(id), Some(Node::Revolve { .. })) => {
            editor_core::Operand::output(id, 0)
        }
        other => other,
    }
}

/// [`place`] for each of `bodies`, in order: the document keeps only
/// the placed document.
pub fn place_all(doc: ProfileDoc, bodies: &[RecipeNodeId]) -> ProfileDoc {
    bodies.iter().fold(doc, |doc, &body| place(doc, body).0)
}

/// **`cut` and the world placements of what it moves**: every
/// placement whose body a cut node defines, added to the cut, as a
/// split moves a body with its placements (a part delivers only its
/// world).
pub fn with_placements(
    doc: &ProfileDoc,
    cut: &std::collections::BTreeSet<RecipeNodeId>,
) -> std::collections::BTreeSet<RecipeNodeId> {
    let mut out = cut.clone();
    for placement in doc.placements() {
        if let Some(Node::Place { shapes, .. }) = doc.node(placement)
            && shapes.iter().any(|shape| {
                doc.operation_of(shape.read)
                    .is_some_and(|at| cut.contains(&at))
            })
        {
            out.insert(placement);
        }
    }
    out
}

/// [`editor_core::split`] of `cut` with the placements of what it
/// moves ([`with_placements`]).
///
/// # Errors
///
/// The split's own refusal.
pub fn split_world(
    doc: &ProfileDoc,
    cut: &std::collections::BTreeSet<RecipeNodeId>,
    part_id: editor_core::DocumentId,
    tol: Tol,
    resolver: Option<&std::sync::Arc<dyn editor_core::PartResolver>>,
) -> Result<editor_core::SplitOutcome, editor_core::SplitError> {
    editor_core::split(doc, &with_placements(doc, cut), part_id, tol, resolver)
}

/// **A world the gather refuses while every copy evaluates**: one
/// instance of a placed block, its bundle empty, so a loose copy. Its
/// copy lives in its own space, so the world holds no copy and the
/// gather refuses [`editor_core::ProductError::EmptyProduct`] naming it
/// loose, while the instance itself evaluates. Answers the document and
/// the options that resolve its part.
pub fn unplaced_world(id: &str) -> (ProfileDoc, EvalOptions) {
    let mut store = resolver::PartStore::default();
    let part = ProfileDoc::empty_derived(&format!("{id}-part"), Tol::witness());
    let (part, profile) = on_frame(
        part,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(0.0, 0.0, 0.5)],
    );
    let (part, body) = insert(
        part,
        Node::Extrude {
            profile: profile.into(),
            distance: len(1.0),
        },
    );
    let (part_ref, _) = store.insert_part((part, body), Tol::witness());
    let doc = ProfileDoc::empty_derived(id, Tol::witness());
    let (doc, _) = insert(doc, mated_instance(part_ref));
    (doc, resolver::with_resolver(store))
}

/// **The refusal inserting `node` into `doc` meets**, for a row about
/// what the insert door refuses.
///
/// # Panics
///
/// If the insert is accepted.
pub fn insert_refused(doc: &ProfileDoc, node: AuthoredNode) -> editor_core::EditError {
    match editor_core::apply(
        doc,
        &DocEdit::InsertNode {
            node: Box::new(node),
            fresh: Vec::new(),
        },
        Tol::witness(),
    ) {
        Ok(_) => panic!("the insert door accepted a node this row expects it to refuse"),
        Err(refusal) => refusal,
    }
}

/// `name` as the node `to` would mint it where `from` did: the same
/// role path at another minting node. Two inserts that differ only in
/// what a row varies are two nodes under two ids (N1), and a row that
/// compares their tables compares them up to that id.
pub fn renoded(name: &StableName, from: RecipeNodeId, to: RecipeNodeId) -> StableName {
    let mut name = name.clone();
    if name.node == from {
        name.node = to;
    }
    name
}

/// **A union whose members stand in `members`' order**, reached the way
/// one union is reordered rather than by inserting another: the union
/// is inserted over the members in document order, and `SetMembers`
/// then puts them in the order asked. Every order of one member set is
/// so one node under one id — an insert mints its id from its node,
/// the members' order included (N1) — which is what a row comparing
/// member orders compares.
pub fn union_over(doc: ProfileDoc, members: &[RecipeNodeId]) -> (ProfileDoc, RecipeNodeId) {
    let mut inserted = members.to_vec();
    inserted.sort();
    let (doc, union) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(
                inserted.clone().into_iter().map(Into::into).collect(),
            ),
        },
    );
    if inserted == members {
        return (doc, union);
    }
    let (doc, _) = step(
        doc,
        DocEdit::SetMembers {
            node: union,
            members: editor_core::Bodies::Spelled(
                members.iter().copied().map(Into::into).collect(),
            ),
        },
    );
    (doc, union)
}

/// The `Debug` of the one body `id` built — its bits, for setting two
/// runs' bodies side by side.
pub fn built_bits(ev: &Evaluation<f64>, id: RecipeNodeId) -> String {
    match ev.value(id).map(|v| &v.payload) {
        Some(editor_core::ValuePayload::Body(b)) => format!("{b:?}"),
        Some(editor_core::ValuePayload::Boolean(editor_core::BooleanValue::Body {
            body, ..
        })) => format!("{body:?}"),
        other => panic!("{id:?} built no body: {other:?} / {:?}", ev.nodes.get(&id)),
    }
}

/// **The constraint door's verdict on a mate**: the door asks the solve's own per-constraint admission — a frame with no
/// definite direction, the table's gaps, a face-based side resolved from
/// the part's own face, a rider on a coincidence decided over the mated
/// parts' extent, a mate beyond the pin — so a mate the solve refuses on
/// its own datum comes out of the door as its fault. The mate is added
/// to the bundle of the copy it places ([`placed_by`]). `Ok` is the
/// document with the constraint and its address; `Err` the placement
/// the door named and the solve's fault.
// The pair is matched by value at every call site; a test's refusal
// path is no hot `Err`.
#[allow(clippy::result_large_err)]
pub fn at_the_door(
    doc: &ProfileDoc,
    mate: editor_core::Mate<Formula>,
) -> Result<(ProfileDoc, editor_core::MateRef), (RecipeNodeId, editor_core::MateFault)> {
    let placement = placed_by(doc, &mate);
    at_the_door_of(doc, placement, mate)
}

/// [`at_the_door`] onto `placement` as named.
#[allow(clippy::result_large_err)]
pub fn at_the_door_of(
    doc: &ProfileDoc,
    placement: RecipeNodeId,
    mate: editor_core::Mate<Formula>,
) -> Result<(ProfileDoc, editor_core::MateRef), (RecipeNodeId, editor_core::MateFault)> {
    match doc.apply(
        &DocEdit::AddConstraint {
            placement,
            mate: Box::new(mate),
            fresh: Vec::new(),
        },
        Tol::witness(),
    ) {
        Ok(applied) => {
            let id = applied.record.minted_constraints[0];
            Ok((applied.doc, editor_core::MateRef::new(placement, id)))
        }
        Err(editor_core::EditError::MateRefused { node, fault, .. }) => Err((node.id(), *fault)),
        Err(other) => panic!("the door refused otherwise: {other:?}"),
    }
}

/// [`at_the_door`] for a mate the door refuses on the datum alone: the
/// fault it carries.
pub fn door_refusal(
    doc: &editor_core::ProfileDoc,
    mate: editor_core::Mate<Formula>,
) -> editor_core::MateFault {
    match at_the_door(doc, mate) {
        Err((_, fault)) => fault,
        Ok(_) => panic!("the door admitted a mate it refuses on its own datum"),
    }
}

/// **A profile program on the xy frame, extruded a unit, evaluated**
/// — the preamble a row that measures a profile door's answer against
/// the solid the profile swept opens with, written once.
///
/// Seven statements, and the only thing that varies between the rows
/// that write them is the loop list: mint the document, insert the
/// frame, insert the profile on it, extrude it, evaluate, then reach
/// back for the program node and the evaluated profile value. Holding
/// the document and the evaluation together is what lets the last two
/// be borrows rather than a fourth and fifth thing to unpack.
///
/// It wears no door's name: a row asks it for the DOCUMENT it
/// measures, and asks the kernel for the answer it is measuring.
pub struct Swept {
    /// The document, with the profile and the extrude on it.
    pub doc: ProfileDoc,
    /// The profile node.
    pub profile: RecipeNodeId,
    /// The extrude over it.
    pub ext: RecipeNodeId,
    /// The evaluation of the whole document.
    pub ev: Evaluation<f64>,
}

/// [`Swept`]'s constructor: `loops` on a fresh document named `id`.
///
/// # Panics
///
/// If the document does not build — a fixture that will not author is
/// a test failure, not a value to hand back.
pub fn wall_row(id: &str, loops: Vec<LoopProgram<Formula>>) -> Swept {
    let doc = ProfileDoc::empty_derived(id, Tol::witness());
    let (doc, profile) = insert(
        doc,
        Node::Profile(ProfileProgram {
            loops,
            ids: Vec::new(),
        }),
    );
    let (doc, ext) = insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(1.0),
        },
    );
    let ev = run(&doc, &EvalOptions::default());
    Swept {
        doc,
        profile,
        ext,
        ev,
    }
}

impl Swept {
    /// The program the profile node holds.
    ///
    /// # Panics
    ///
    /// If the node this built is not a profile.
    pub fn program(&self) -> &ProfileProgram {
        match self.doc.node(self.profile) {
            Some(Node::Profile(p)) => p,
            _ => panic!("the profile node this fixture inserted is a program"),
        }
    }

    /// The evaluated profile value — its validated loops, its naming
    /// anchor and its per-edge radius table.
    ///
    /// # Panics
    ///
    /// If the profile did not evaluate, or evaluated to something else.
    pub fn profile_value(&self) -> &editor_core::eval::ProfileValue<f64> {
        match &self
            .ev
            .value(self.profile)
            .expect("the fixture profile evaluates")
            .payload
        {
            editor_core::ValuePayload::Profile(pv) => pv,
            _ => panic!("the profile node's value carries a profile"),
        }
    }
}

/// A profile program from polygon corner lists (LIB-SWITCH §4i: the
/// corpus's polygon choke point — under v4 each loop is a chain
/// program, `At(p0), LineTo(p1), …, LineTo(Start)`, the VQ5 expansion
/// at literal points), in the profile's own 2-D coordinates.
pub fn desc(loops: Vec<Vec<(f64, f64)>>) -> ProfileProgram<Formula> {
    let loops = loops
        .into_iter()
        .map(|pts| LoopProgram::polygon(pts).expect("finite corners"))
        .collect();
    ProfileProgram {
        loops,
        ids: Vec::new(),
    }
}

/// **A profile, inserted** — what a call site that wants "a square in
/// its own plane" writes in one line. The frame is the profile's own
/// `xy` and nothing else: a profile reads no plane (D10), so one that
/// stood elsewhere is a construction PLACED there ([`against`],
/// [`in_world`]).
///
/// # Panics
///
/// If the frame is not the identity: such a site is restated as a
/// placement.
pub fn on_frame(
    doc: ProfileDoc,
    origin: [f64; 3],
    u: [f64; 3],
    v: [f64; 3],
    loops: Vec<Vec<(f64, f64)>>,
) -> (ProfileDoc, RecipeNodeId) {
    own_frame(origin, u, v);
    insert(doc, Node::Profile(desc(loops)))
}

/// **That `(origin, u, v)` is a profile's own `xy`** — the one frame a
/// profile is drawn on.
///
/// # Panics
///
/// If it is any other: the construction is placed there instead.
pub fn own_frame(origin: [f64; 3], u: [f64; 3], v: [f64; 3]) {
    assert!(
        frame_at(origin, u, v) == editor_core::Frame::IDENTITY,
        "a profile reads no plane: a construction drawn on {origin:?}, {u:?}, {v:?} is \
         placed there"
    );
}

/// A revolve of `profile` by `angle` about the axis line through
/// `origin` at `heading` from the profile's x axis, both in the
/// profile's own 2-D coordinates.
pub fn revolve_about(
    profile: impl Into<editor_core::Operand>,
    origin: (f64, f64),
    heading: Formula,
    angle: Formula,
) -> AuthoredNode {
    Node::Revolve {
        profile: profile.into(),
        axis_through: [origin.0, origin.1].map(|at| {
            if at == 0.0 {
                Formula::zero(Dimension::Length)
            } else {
                len(at)
            }
        }),
        axis_heading: heading,
        angle,
    }
}

/// The heading along the profile's +x: the constant `0`.
pub fn along_x() -> Formula {
    Formula::zero(Dimension::Angle)
}

/// The heading along the profile's +y: the constant `turn/4`.
pub fn along_y() -> Formula {
    Formula::turn_over(4).expect("a quarter turn")
}

/// An axis-aligned square of half-width `h` centered at (cx, cy).
pub fn square(cx: f64, cy: f64, h: f64) -> Vec<(f64, f64)> {
    vec![
        (cx - h, cy - h),
        (cx + h, cy - h),
        (cx + h, cy + h),
        (cx - h, cy + h),
    ]
}

/// Applies-and-RECORDS: the edit-log author shared by the die
/// fixture and the M4 PR 8a Band 4 corpus. The saved snapshot is the
/// EMPTY document and the log is everything, so a load replays the
/// whole document through `apply`'s doors.
pub struct Recorder {
    /// The document as edited so far.
    pub doc: ProfileDoc,
    /// The recorded log.
    pub edits: Vec<editor_core::DocEdit<ProfileProgram>>,
    /// The last evaluation a placement read, the next one's prior: the
    /// nodes it already holds are reused, so placing a body costs its
    /// new nodes alone.
    pub last: Option<Evaluation<f64>>,
}

impl Default for Recorder {
    fn default() -> Self {
        Self::new()
    }
}

impl Recorder {
    /// A recorder over the empty document.
    pub fn new() -> Self {
        Self {
            doc: ProfileDoc::empty_derived("mod", Tol::witness()),
            edits: Vec::new(),
            last: None,
        }
    }

    /// **The document as edited so far, evaluated**, the last such
    /// evaluation its prior: what a placement reads its bodies off.
    pub fn evaluated(&mut self) -> &Evaluation<f64> {
        let ev = editor_core::evaluate::<f64>(
            &self.doc,
            self.last.as_ref(),
            &editor_core::CancelToken::new(),
            &EvalOptions::default(),
            Tol::witness(),
        );
        self.last.insert(ev)
    }

    /// Applies an edit (the doors refusing is a loud test failure)
    /// and records it; returns any minted id.
    pub fn push(&mut self, edit: DocEdit<ProfileProgram>) -> Option<RecipeNodeId> {
        let applied =
            editor_core::apply(&self.doc, &edit, Tol::witness()).expect("recorded edit must apply");
        self.edits.push(edit);
        self.doc = applied.doc;
        applied.record.minted
    }

    /// Inserts measures ([`editor_core::measure`]), one per primitive
    /// [`sited`] over `refs`, recording their edits; returns the measures
    /// and their outputs.
    pub fn measure<R: Clone + Into<editor_core::Operand>>(
        &mut self,
        primitives: &[editor_core::MeasurePrimitive<u32>],
        refs: &[R],
    ) -> editor_core::Measured {
        let sited: Vec<_> = primitives.iter().map(|&p| sited(p, refs)).collect();
        let out =
            editor_core::measure(&self.doc, &sited, Tol::witness()).expect("the measures insert");
        self.edits.extend(out.edits);
        self.doc = out.doc;
        out.measured
    }

    /// **`formula` declared as the defined variable `name`**, returning
    /// its id: a measurement's arithmetic over measure outputs, named.
    pub fn define(&mut self, name: &str, formula: Formula) -> editor_core::VarId {
        let name = editor_core::VarName::new(name).expect("a valid name");
        self.push(DocEdit::DeclareVar {
            name: name.clone(),
            def: editor_core::VarDecl::Defined(formula),
        });
        self.doc.resolve_var(&name.into()).expect("declared")
    }

    /// The length an output of this recorder's document reads.
    pub fn len_of(&self, var: editor_core::VarId) -> Formula {
        read_var(&self.doc, var)
    }

    /// **A measure whose value is the length variable `param`**: a unit
    /// cube and its copy translated by `param` along x, measured from the
    /// cube's vertex at the origin to the copy's, which is `|param − 0|`
    /// with no geometry standing between the variable and the reading.
    /// Returns the measure.
    pub fn measure_of_translation(&mut self, param: &str) -> RecipeNodeId {
        let name = editor_core::VarName::new(param).expect("a valid name");
        let profile = self.profile(
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
        );
        let cube = self.insert(Node::Extrude {
            profile: profile.into(),
            distance: len(1.0),
        });
        // The copy slid `param` along the cube's own x: a plane mate of
        // its start cap to the cube's, the slide a value.
        let cap = || {
            plane(editor_core::Operand::select(
                cube,
                vec![fname(cube, RoleSeg::Cap(CapEnd::Start))],
            ))
        };
        let copy = self.insert(Node::place(vec![editor_core::BodyRead::plain(
            editor_core::Operand::from(cube),
        )]));
        self.constrain(
            copy,
            editor_core::Mate {
                on: cap(),
                to: editor_core::Target::Pose(cap()),
                class: editor_core::ContactClass::Rest,
            },
            vec![
                (
                    editor_core::Coord::Slide(editor_core::TargetAxis::X),
                    Formula::named(name, editor_core::Dimension::Length),
                ),
                (
                    editor_core::Coord::Slide(editor_core::TargetAxis::Y),
                    Formula::zero(Dimension::Length),
                ),
                (editor_core::Coord::Spin, Formula::zero(Dimension::Angle)),
            ],
        );
        let ev = editor_core::evaluate::<f64>(
            &self.doc,
            None,
            &editor_core::CancelToken::new(),
            &editor_core::EvalOptions::default(),
            Tol::witness(),
        );
        let vertex = editor_core::all_vertices(&ev, cube)
            .into_iter()
            .find(|v| {
                let p = editor_core::vertex_position(&ev, cube, v).expect("a vertex");
                (p.x, p.y, p.z) == (0.0, 0.0, 0.0)
            })
            .expect("the cube has a vertex at the origin");
        // The copy's vertex is the cube's, qualified by the copy.
        let copied = StableName {
            kind: vertex.kind,
            node: copy,
            path: vec![RoleSeg::Placed {
                of: editor_core::NameRef::new(vertex.clone()),
            }],
        };
        self.insert(Node::Measure {
            primitive: editor_core::MeasurePrimitive::Distance {
                a: editor_core::SitedRef::new(cube, vertex).into(),
                b: editor_core::SitedRef::new(copy, copied).into(),
            },
        })
    }

    /// Inserts a node, returning its minted id.
    pub fn insert(&mut self, node: AuthoredNode) -> RecipeNodeId {
        self.push(DocEdit::InsertNode {
            node: Box::new(node),
            fresh: Vec::new(),
        })
        .expect("minted id")
    }

    /// **`mate` and the `values` naming it, added to `placement`**,
    /// recorded.
    pub fn constrain(
        &mut self,
        placement: RecipeNodeId,
        mate: editor_core::Mate<Formula>,
        values: Vec<(editor_core::Coord, Formula)>,
    ) {
        let edit = DocEdit::AddConstraint {
            placement,
            mate: Box::new(mate),
            fresh: Vec::new(),
        };
        let applied =
            editor_core::apply(&self.doc, &edit, Tol::witness()).expect("recorded edit must apply");
        let mate = applied.record.minted_constraints[0];
        self.edits.push(edit);
        self.doc = applied.doc;
        for (coord, by) in values {
            self.push(DocEdit::AddValue {
                placement,
                value: Box::new(editor_core::Value { mate: Some(mate), coord, by }),
                fresh: Vec::new(),
            });
        }
    }

    /// **A gesture's constraints written onto `placement`**, recorded
    /// ([`editor_core::Placed::write`]).
    pub fn write(&mut self, placement: RecipeNodeId, placed: editor_core::Placed) {
        placed
            .write(placement, |edit| {
                let applied = editor_core::apply(&self.doc, &edit, Tol::witness())
                    .expect("recorded edit must apply");
                self.edits.push(edit);
                self.doc = applied.doc;
                Ok::<_, std::convert::Infallible>(applied.record.minted_constraints)
            })
            .unwrap_or_else(|never| match never {});
    }

    /// **One copy of `body` in the world** (A10), standing where its
    /// body stands ([`editor_core::stand`]), returning the placement.
    pub fn place(&mut self, body: impl Into<editor_core::Operand>) -> RecipeNodeId {
        let read = body_read(&self.doc, body.into());
        self.evaluated();
        let ev = self.last.as_ref().expect("just evaluated");
        let stood = editor_core::stand_in(&self.doc, ev, &read)
            .unwrap_or_else(|refusal| panic!("the copy stands over its geometry: {refusal}"));
        let placement = self.insert(Node::place(vec![editor_core::BodyRead::plain(read)]));
        self.write(placement, stood);
        placement
    }

    /// **One copy of `body` in the world at `at`**
    /// ([`editor_core::place_at`], [`Self::place`] at the identity):
    /// how a construction that sat on a literal datum frame stands where
    /// that frame put it. Returns the placement.
    pub fn in_world(
        &mut self,
        body: impl Into<editor_core::Operand>,
        at: &editor_core::Frame,
    ) -> RecipeNodeId {
        if *at == editor_core::Frame::IDENTITY {
            return self.place(body);
        }
        let read = body_read(&self.doc, body.into());
        self.evaluated();
        let ev = self.last.as_ref().expect("just evaluated");
        let placed = editor_core::place_at_in(&self.doc, ev, &read, at, Tol::witness())
            .unwrap_or_else(|refusal| panic!("the copy places at its frame: {refusal}"));
        let placement = self.insert(Node::place(vec![editor_core::BodyRead::plain(read)]));
        self.write(placement, placed);
        placement
    }

    /// **A copy of `body` placed against `target`'s geometry at `at`**
    /// (`at` the copy's own frame in `target`'s own coordinates,
    /// [`editor_core::place_against`]), returning the placement, whose
    /// output is the copy.
    pub fn against(
        &mut self,
        target: impl Into<editor_core::Operand>,
        body: impl Into<editor_core::Operand>,
        at: &editor_core::Frame,
    ) -> RecipeNodeId {
        let target = body_read(&self.doc, target.into());
        let read = body_read(&self.doc, body.into());
        self.evaluated();
        let ev = self.last.as_ref().expect("just evaluated");
        let placed =
            editor_core::place_against_in(&self.doc, ev, &target, &read, at, Tol::witness())
                .unwrap_or_else(|refusal| panic!("the copy places against its target: {refusal}"));
        let placement = self.insert(Node::place(vec![editor_core::BodyRead::plain(read)]));
        self.write(placement, placed);
        placement
    }

    /// **A profile**, returning its id — [`on_frame`]'s shape for a
    /// recorder: the frame is the profile's own `xy`.
    ///
    /// # Panics
    ///
    /// If the frame is not the identity ([`own_frame`]).
    pub fn profile(
        &mut self,
        origin: [f64; 3],
        u: [f64; 3],
        v: [f64; 3],
        loops: Vec<Vec<(f64, f64)>>,
    ) -> RecipeNodeId {
        own_frame(origin, u, v);
        self.insert(Node::Profile(desc(loops)))
    }
}

/// The authored die and the ids the tests address.
pub struct Die {
    pub doc: ProfileDoc,
    /// The document's full edit log (snapshot = the empty document).
    pub edits: Vec<editor_core::DocEdit<ProfileProgram>>,
    /// The final Subtract (the die body).
    pub final_node: RecipeNodeId,
    /// The +z face's pip-master Extrude (the poisoning target: its
    /// descendants are exactly the LAST placement + subtract).
    pub pz_extrude: RecipeNodeId,
    /// The +z pip's placement (the incremental-edit target).
    pub pz_placement: RecipeNodeId,
    /// Total node count.
    pub n_nodes: usize,
}

/// Face frames (origin, u, v; normal = u×v points OUT of the cube
/// `[0,2]³`) and pip layouts in face coordinates, +z LAST so the
/// incremental edit's cone is minimal.
type Face = ([f64; 3], [f64; 3], [f64; 3], &'static [(f64, f64)]);
pub fn faces() -> [Face; 6] {
    // Pip grid: {0.5, 1.0, 1.5} per face axis.
    [
        // -z: 6
        (
            [0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [1.0, 0.0, 0.0],
            &[
                (0.5, 0.5),
                (1.0, 1.0),
                (1.5, 1.5),
                (0.5, 1.5),
                (1.5, 0.5),
                (0.5, 1.0),
            ][..],
        ),
        // +x: 2
        (
            [2.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
            &[(0.5, 0.5), (1.5, 1.5)][..],
        ),
        // -x: 5
        (
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            [0.0, 1.0, 0.0],
            &[(0.5, 0.5), (1.5, 1.5), (0.5, 1.5), (1.5, 0.5), (1.0, 1.0)][..],
        ),
        // +y: 3
        (
            [0.0, 2.0, 0.0],
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 0.0],
            &[(0.5, 0.5), (1.0, 1.0), (1.5, 1.5)][..],
        ),
        // -y: 4
        (
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            &[(0.5, 0.5), (0.5, 1.5), (1.5, 0.5), (1.5, 1.5)][..],
        ),
        // +z: 1 (LAST)
        (
            [0.0, 0.0, 2.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            &[(1.0, 1.0)][..],
        ),
    ]
}

/// Authors the die document (module docs).
pub fn die() -> Die {
    let mut r = Recorder::new();
    // pip_depth: the mid-DAG continuous parameter.
    r.push(DocEdit::DeclareVar {
        name: VarName::from_static("pip_depth"),
        def: editor_core::VarDecl::Free(FreeVar::continuous(Dimension::Length, DEPTH)),
    });
    // The cube: profile on the xy plane, extruded +2.
    let cube_profile = r.profile(
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)]],
    );
    let cube = r.insert(Node::Extrude {
        profile: cube_profile.into(),
        distance: len(2.0),
    });

    // Per-face masters: a pip profile centered at its own origin,
    // extruded along its own +z by pip_depth.
    let mut masters = Vec::new(); // (extrude id, origin, u, v, pips)
    for (o, u, v, pips) in faces() {
        let prof = r.profile(
            [0.0; 3],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![square(0.0, 0.0, 0.125)],
        );
        let ext = r.insert(Node::Extrude {
            profile: prof.into(),
            distance: Formula::named(VarName::from_static("pip_depth"), Dimension::Length),
        });
        masters.push((ext, o, u, v, pips));
    }

    // Interleaved placement + Subtract pairs: each pip placed against
    // the cube with its start cap ON its cube face and its own +z
    // INTO the cube (the frame's `v` reversed), a coincidence the
    // subtract's margins decide and glue.
    let mut acc = cube;
    let mut pz_placement = acc; // overwritten below
    for &(ext, o, u, v, pips) in &masters {
        for &(a, b) in pips {
            let at = core::array::from_fn(|i| o[i] + a * u[i] + b * v[i]);
            let placed = r.against(cube, ext, &frame_at(at, u, v.map(|c| -c)));
            let sub = r.insert(Node::Subtract {
                from: acc.into(),
                tool: placed.into(),
            });
            acc = sub;
            pz_placement = placed;
        }
    }

    let n_nodes = r.doc.len();
    Die {
        doc: r.doc,
        edits: r.edits,
        final_node: acc,
        pz_extrude: masters[5].0,
        pz_placement,
        n_nodes,
    }
}

pub mod pr4;

/// **The witness tolerance a suite decides under** — `Tol::witness()`
/// under the name the suites reach for it by.
pub fn tol() -> Tol {
    Tol::witness()
}

/// **A one-segment name at a node** — the whole of what "the name
/// `node` mints for `seg`" is, for any entity kind. [`fname`],
/// [`ename`] and [`vname`] are this with the kind spelled in the
/// call.
pub fn minted(kind: EntityKind, node: RecipeNodeId, seg: RoleSeg) -> StableName {
    StableName {
        kind,
        node,
        path: vec![seg],
    }
}

/// One face name at a node (authoring shorthand).
pub fn fname(node: RecipeNodeId, seg: RoleSeg) -> StableName {
    minted(EntityKind::Face, node, seg)
}

/// An extrude's cap, read at the extrude (authoring shorthand).
pub fn cap_ref(node: RecipeNodeId, end: editor_core::CapEnd) -> editor_core::SitedRef {
    editor_core::SitedRef::new(node, fname(node, editor_core::RoleSeg::Cap(end)))
}

/// One vertex name at a node (authoring shorthand).
pub fn vname(node: RecipeNodeId, seg: RoleSeg) -> StableName {
    minted(EntityKind::Vertex, node, seg)
}

/// **A cap RIM edge of an extrude**, by name — the arc cap `end`
/// shares with the wall over outer- or hole-loop segment `edge`.
pub fn rim_edge(node: RecipeNodeId, end: CapEnd, edge: ProfileEdgeRef) -> StableName {
    ename(node, RoleSeg::RimEdge(end, edge.into()))
}

/// **A cap VERTEX of an extrude**, by name — the corner cap `end`
/// carries at profile vertex `vertex`.
pub fn cap_vertex(node: RecipeNodeId, end: CapEnd, vertex: ProfileVertexRef) -> StableName {
    vname(node, RoleSeg::CapVertex(end, vertex))
}

/// **A POLE vertex of a revolve**, by name — the vertex the axis pins,
/// minted for profile vertex `vertex`.
pub fn pole(node: RecipeNodeId, vertex: ProfileVertexRef) -> StableName {
    vname(node, RoleSeg::Pole(vertex))
}

/// **The symmetric U cutter, whose subtract table holds an N2 tie** —
/// a 4x4x4 block with a U-shaped prism cut through it, the U's two
/// arms congruent so nothing covariant discriminates the faces they
/// mint and the table records one `Entry::Tied` row instead of two
/// names. Returns the subtract node.
///
/// One home for a document a row needs when it needs a REAL tie
/// rather than a hand-planted one. The same shape is
/// hand-copied across this tree; `work/wire` carries the row for
/// re-pointing those copies here.
pub fn u_cutter_tie(doc: ProfileDoc) -> (ProfileDoc, RecipeNodeId) {
    let (doc, block_profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (4.0, 0.0), (4.0, 4.0), (0.0, 4.0)]],
    );
    let (doc, target) = insert(
        doc,
        Node::Extrude {
            profile: block_profile.into(),
            distance: len(4.0),
        },
    );
    let (doc, u_profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![
            (2.0, 1.0),
            (6.0, 1.0),
            (6.0, 3.0),
            (2.0, 3.0),
            (2.0, 2.5),
            (5.0, 2.5),
            (5.0, 1.5),
            (2.0, 1.5),
        ]],
    );
    let (doc, cutter) = insert(
        doc,
        Node::Extrude {
            profile: u_profile.into(),
            distance: len(2.0),
        },
    );
    // The cutter, a construction of its own, placed against the block
    // one up its height.
    let (doc, cutter) = against(
        doc,
        target,
        cutter,
        &frame_at([0.0, 0.0, 1.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]),
    );
    insert(
        doc,
        Node::Subtract {
            from: target.into(),
            tool: cutter.into(),
        },
    )
}

/// One edge name at a node (authoring shorthand).
pub fn ename(node: RecipeNodeId, seg: RoleSeg) -> StableName {
    minted(EntityKind::Edge, node, seg)
}

// ---------------------------------------------------------------- //
// Reading an evaluation, its tables and its bodies
// ---------------------------------------------------------------- //
//
// ONE home for the readers every name-reading suite spells for itself.
// Each is the loud form: a missing node, a name that resolves to
// nothing or to the wrong KIND of thing, or a dead key, is a panic
// naming the subject rather than a `None` a row can drop on the floor.

/// The name table `id` published, or a panic naming what it did
/// instead.
pub fn table(ev: &Evaluation<f64>, id: RecipeNodeId) -> &NameTable {
    ev.table_at(id)
        .unwrap_or_else(|| panic!("node {id:?} has no one table: {:?}", ev.nodes.get(&id)))
}

/// **The read `name` is read in where it was minted**: the one named
/// output of its minting node as the evaluation reads it, else as the
/// document defines it (a node with no value in the run), NAMES N1: a
/// name is resolved in the read that holds its body.
///
/// # Panics
///
/// If the minting node is the document's and neither names one read
/// for it (a split, whose halves are two): the row names its read.
/// A node the document no longer holds has no read left, and reads in
/// a variable no document defines.
pub fn at_mint<T: geom_core::Decide>(
    ctx: editor_core::RunCtx<'_, T>,
    name: &StableName,
) -> editor_core::VarId {
    ctx.eval
        .at_var(name.node)
        .or_else(|| ctx.doc.read_of_node(name.node))
        .unwrap_or_else(|| {
            assert!(
                ctx.doc.node(name.node).is_none(),
                "{:?} has no one read: the row names the read {name:?} is read in",
                name.node
            );
            editor_core::VarId::new(0, 0)
        })
}

/// [`editor_core::resolve`] in the read `name` was minted in
/// ([`at_mint`]): the row's site when it is about a name where its own
/// node published it. A name whose minting node the document no longer
/// holds is stranded with it ([`editor_core::stranded_at`]: `NodeGone`).
///
/// # Panics
///
/// If the door refuses the read: [`at_mint`] names a body output.
pub fn resolve_at_mint<T: geom_core::Decide>(
    ctx: editor_core::RunCtx<'_, T>,
    name: &StableName,
) -> editor_core::Resolution {
    if let Some(gone) = editor_core::stranded_at(ctx.doc, name.node, name) {
        return gone.expect("a name minted at a gone node is NodeGone");
    }
    editor_core::resolve(ctx, at_mint(ctx, name), name).expect("the mint's read holds a body")
}

/// [`editor_core::resolve_with_prior`] in the read `name` was minted in
/// ([`at_mint`], in the new document); [`resolve_at_mint`]'s stranded
/// case alike.
///
/// # Panics
///
/// [`resolve_at_mint`]'s.
pub fn resolve_with_prior_at_mint<T: geom_core::Decide, U: geom_core::Decide>(
    new: editor_core::RunCtx<'_, T>,
    prior: editor_core::RunCtx<'_, U>,
    name: &StableName,
) -> editor_core::Resolution {
    if let Some(gone) = editor_core::stranded_at(new.doc, name.node, name) {
        return gone.expect("a name minted at a gone node is NodeGone");
    }
    editor_core::resolve_with_prior(new, prior, at_mint(new, name), name)
        .expect("the mint's read holds a body")
}

/// **The body read an appearance entry on `name` is read in**, for a
/// row that paints a name where it was minted (NAMES N1: an entry is
/// keyed by its body read and its name): the read an entry the store
/// already holds on `name` is in, else the one output of the node that
/// minted it. A name with neither — its node gone or with several
/// outputs, and nothing painted on it — reads in a variable no document
/// defines: a row about a refusal the door raises before it reads the
/// body.
///
/// # Panics
///
/// If the store holds `name` under two reads: which one a row means is
/// the row's to say, by reading `doc.appearance_of(read, name)` itself.
pub fn painted_in(doc: &ProfileDoc, name: &StableName) -> editor_core::VarId {
    let mut held = doc.appearance().keys().filter(|key| key.name == *name);
    let first = held.next().map(|key| key.at);
    if let Some(second) = held.next() {
        panic!(
            "{name:?} is painted in two reads ({:?} and {:?}): the row names its read",
            first, second.at
        );
    }
    first
        .or_else(|| doc.read_of_node(name.node))
        .unwrap_or(editor_core::VarId::new(0, 0))
}

/// **The appearance record on `name`, read where it was minted** —
/// [`painted_in`]'s entry.
///
/// # Panics
///
/// If the record is held on a read the document no longer defines: a
/// key whose read went is a stranded key whatever its name says, and a
/// row asking for a live paint must not find one there. A row about a
/// stranded record reads it with `doc.appearance_of` directly.
pub fn painted<'a>(
    doc: &'a ProfileDoc,
    name: &StableName,
) -> Option<&'a editor_core::AppearanceRecord> {
    let read = painted_in(doc, name);
    let record = doc.appearance_of(read, name);
    assert!(
        record.is_none() || doc.operation_of(read).is_some(),
        "{name:?} is painted in {read:?}, which no node of the document defines"
    );
    record
}

/// **The name table of `split`'s half `half`** — the read of its port
/// (NAMES N1: each half its own table, its body that table's body 0).
pub fn half(ev: &Evaluation<f64>, split: RecipeNodeId, half: editor_core::SplitHalf) -> &NameTable {
    ev.output_var(split, half.port())
        .and_then(|var| ev.table(var))
        .unwrap_or_else(|| {
            panic!(
                "split {split:?} has no {half:?} table: {:?}",
                ev.nodes.get(&split)
            )
        })
}

/// **Every name table `id`'s outputs carry**, in port order (NAMES N1:
/// one per output variable, a split's two halves two) — empty for a
/// node with no value.
pub fn tables<T: geom_core::Decide>(ev: &Evaluation<T>, id: RecipeNodeId) -> Vec<&NameTable> {
    ev.named_outputs(id)
        .into_iter()
        .filter_map(|var| ev.table(var))
        .collect()
}

/// The one entity a name answers to — the row's loud end when a mint
/// is missing, misspelled or aliased. `what` is the caller's word for
/// the subject, so a failure says which row's name did not resolve.
pub fn key_of(t: &NameTable, what: &str, n: &StableName) -> EntityKey {
    match t.lookup(n) {
        Some(Entry::Unique(r)) => r.key,
        other => panic!("{what}: {n:?} is not uniquely named: {other:?}"),
    }
}

/// [`key_of`], refusing anything that is not an edge.
pub fn edge_of(t: &NameTable, what: &str, n: &StableName) -> EdgeKey {
    match key_of(t, what, n) {
        EntityKey::Edge(k) => k,
        other => panic!("{what}: {n:?} names {other:?}, not an edge"),
    }
}

/// [`key_of`], refusing anything that is not a vertex.
pub fn vertex_of(t: &NameTable, what: &str, n: &StableName) -> VertexKey {
    match key_of(t, what, n) {
        EntityKey::Vertex(k) => k,
        other => panic!("{what}: {n:?} names {other:?}, not a vertex"),
    }
}

/// [`key_of`], refusing anything that is not a face.
pub fn face_of(t: &NameTable, what: &str, n: &StableName) -> FaceKey {
    match key_of(t, what, n) {
        EntityKey::Face(k) => k,
        other => panic!("{what}: {n:?} names {other:?}, not a face"),
    }
}

/// How many names in `t` take `seg`'s role.
pub fn count(t: &NameTable, seg: fn(&RoleSeg) -> bool) -> usize {
    t.iter().filter(|(n, _)| seg(&n.path[0])).count()
}

/// **`names`, faces of `node`'s body, ordered left to right** by the
/// least x any of the face's vertices stands at: a geometric order for
/// rows that mean "the left fragment", which name order does not give
/// (a name's order follows the ids it spells).
pub fn left_to_right(
    ev: &Evaluation<f64>,
    node: RecipeNodeId,
    mut names: Vec<StableName>,
) -> Vec<StableName> {
    let body = crate::corpus::body_of(ev, node);
    let t = table(ev, node);
    let least_x = |n: &StableName| {
        face_vertices(body, face_of(t, "a fragment", n))
            .into_iter()
            .map(|v| point(body, v).x)
            .fold(f64::INFINITY, f64::min)
    };
    names.sort_by(|a, b| least_x(a).total_cmp(&least_x(b)));
    names
}

/// Where a vertex stands.
pub fn point(body: &Body<f64>, v: VertexKey) -> Point3<f64> {
    topo::readback::vertex_point(body, v).expect("a live vertex")
}

/// An edge's two end vertices.
pub fn ends(body: &Body<f64>, e: EdgeKey) -> [VertexKey; 2] {
    let edge = body.get_edge(e).expect("a live edge");
    let h = body.get_half_edge(edge.he_plus).expect("a live half-edge");
    let far = body.half_edge_end(edge.he_plus).expect("a forward half");
    [h.start, far]
}

/// Every vertex on `f`'s boundary — the face's own EXTENT, read out of
/// the body rather than inferred from the surface it is a region of.
/// An empty ring contributes its lone vertex: a pole is a vertex of
/// the extent that bounds none of the face's edges.
pub fn face_vertices(body: &Body<f64>, f: FaceKey) -> HashSet<VertexKey> {
    let face = body.get_face(f).expect("a live face");
    let mut out = HashSet::new();
    for lk in core::iter::once(face.outer).chain(face.rings.iter().copied()) {
        match body.get_loop(lk).expect("a live loop").boundary {
            LoopBoundary::Empty { vertex } => {
                out.insert(vertex);
            }
            LoopBoundary::Cycle { first } => {
                for he in body.loop_cycle(first).expect("a closed cycle") {
                    out.insert(body.get_half_edge(he).expect("a live half-edge").start);
                }
            }
        }
    }
    out
}

/// Every edge on `f`'s boundary — [`face_vertices`]'s twin over the
/// same walk. An empty ring bounds no edge, so it contributes nothing
/// here.
pub fn face_edges(body: &Body<f64>, f: FaceKey) -> HashSet<EdgeKey> {
    let face = body.get_face(f).expect("a live face");
    let mut out = HashSet::new();
    for lk in core::iter::once(face.outer).chain(face.rings.iter().copied()) {
        if let LoopBoundary::Cycle { first } = body.get_loop(lk).expect("a live loop").boundary {
            for he in body.loop_cycle(first).expect("a closed cycle") {
                out.insert(body.get_half_edge(he).expect("a live half-edge").edge);
            }
        }
    }
    out
}

/// **The twelve edges of an extruded `n`-gon prism, by name** — the
/// authoring form of "every edge" for a `Node::Fillet` selection
/// (M6-5). `n` is the outer loop's segment count; the names are the
/// extrude emitter's own: a cap–wall rim per (cap end, segment) and a
/// strut per profile vertex.
///
/// Authored, not queried: a selection FREEZES, so a corpus document
/// states the set it means rather than asking an evaluation.
pub fn prism_edges(doc: &editor_core::ProfileDoc, node: RecipeNodeId, n: u32) -> Vec<StableName> {
    let mut out = Vec::new();
    for seg in 0..n as usize {
        let e = piece(doc, node, 0, seg);
        out.push(rim_edge(node, CapEnd::Start, e));
        out.push(rim_edge(node, CapEnd::End, e));
        out.push(ename(node, RoleSeg::LateralEdge(vpiece(doc, node, 0, seg))));
    }
    out
}

/// **The piece every canonical position of the profile at `profile`
/// is**, under the document's current values — what a sweep over it
/// names each wall, rim and vertex by.
///
/// # Panics
///
/// If `profile` is not a profile node, or its program does not replay
/// and validate.
pub fn pieces(doc: &editor_core::ProfileDoc, profile: RecipeNodeId) -> ProfilePieces {
    match doc.node(profile) {
        Some(Node::Profile(p)) => p
            .pieces(&doc.var_env::<f64>(), Tol::witness())
            .expect("the profile's program replays and validates"),
        other => panic!("node {} is not a profile: {other:?}", profile.0),
    }
}

/// **The profile a sweep node sweeps** — an extrude's or a revolve's
/// operand, or the node itself where it IS a profile; any other node
/// is read through its first input, so a row spelling a wall-shaped
/// name in a downstream node's space spells it with the pieces of the
/// sweep underneath.
///
/// # Panics
///
/// If `node` is not live, or no sweep is upstream of it along first
/// inputs.
pub fn swept(doc: &ProfileDoc, node: RecipeNodeId) -> RecipeNodeId {
    match doc.node(node) {
        Some(Node::Extrude { profile, .. } | Node::Revolve { profile, .. }) => doc
            .operation_of(*profile)
            .expect("the profile read is live"),
        Some(Node::Profile(_)) => node,
        Some(_) => match doc.upstream(node).first() {
            Some(&input) => swept(doc, input),
            None => panic!("node {} sweeps no profile", node.0),
        },
        None => panic!("node {} is not live", node.0),
    }
}

/// **The step that drew `piece`**, whose tag a name's words say where
/// no document is at hand.
///
/// # Panics
///
/// Where `piece` is a kernel-built section's, which no step drew.
pub fn step_of(piece: &ProfileEdgeRef) -> editor_core::StepId {
    match piece {
        ProfileEdgeRef::Piece { step, .. } => *step,
        ProfileEdgeRef::Section { .. } => panic!("a section's piece has no step"),
    }
}

/// **The piece canonical segment `k` of canonical loop `l` is**, on the
/// profile `sweep` sweeps (or `sweep` itself, a profile).
///
/// # Panics
///
/// Where [`pieces`] does, or where the position is past the profile.
pub fn piece(
    doc: &editor_core::ProfileDoc,
    sweep: RecipeNodeId,
    l: usize,
    k: usize,
) -> ProfileEdgeRef {
    pieces(doc, swept(doc, sweep))
        .edge(l, k)
        .expect("the canonical position is the profile's")
}

/// **The piece starting at canonical vertex `v` of canonical loop
/// `l`**, on the profile `sweep` sweeps.
///
/// # Panics
///
/// Where [`piece`] does.
pub fn vpiece(
    doc: &editor_core::ProfileDoc,
    sweep: RecipeNodeId,
    l: usize,
    v: usize,
) -> ProfileVertexRef {
    pieces(doc, swept(doc, sweep))
        .vertex(l, v)
        .expect("the canonical position is the profile's")
}

/// **The leg step `step` draws**, spelled without a document — for a
/// row whose names are compared, sorted or carried and never resolved.
pub fn leg(step: u64) -> ProfileEdgeRef {
    ProfileEdgeRef::Piece {
        step: editor_core::StepId::new(0, step),
        role: editor_core::PieceRole::Leg,
    }
}

/// **A live node as an author inserts it**: a profile program enters
/// the document without step ids — the insert door mints them — so a
/// row that rebuilds a document by re-inserting its nodes clears them;
/// re-inserted in the same order, they are minted the same.
pub fn as_authored(doc: &ProfileDoc, node: &Node<editor_core::ProfileProgram>) -> AuthoredNode {
    let mut node = editor_core::test_support::as_written(doc, node);
    if let Node::Profile(program) = &mut node {
        program.ids = Vec::new();
    }
    node
}

/// **A piece no profile draws, spelled without a document**: step id
/// 0 in a role no verb gives it — a well-formed locator that denotes
/// nothing, since `Piece(7)` is no verb's role. Whether a document's
/// mint log holds id 0 is a matter of what its chain drew, so a row
/// whose name passes a door that checks the log uses [`no_piece_of`],
/// which spells a step the document minted.
pub fn no_piece() -> ProfileEdgeRef {
    ProfileEdgeRef::Piece {
        step: editor_core::StepId::new(0, 0),
        role: editor_core::PieceRole::Piece(7),
    }
}

/// **A piece no profile of `doc` draws**: the least step id `doc` has
/// minted, in a role no verb gives it — a locator that is well formed,
/// spells a minted step, and denotes nothing.
pub fn no_piece_of(doc: &editor_core::ProfileDoc) -> ProfileEdgeRef {
    ProfileEdgeRef::Piece {
        step: doc
            .mint()
            .steps()
            .next()
            .expect("the document has minted a step"),
        role: editor_core::PieceRole::Piece(7),
    }
}

/// A wall (lateral) role for outer-loop canonical segment `seg` of the
/// profile the extrude `ext` sweeps, spelled by the piece it is.
pub fn wall(doc: &editor_core::ProfileDoc, ext: RecipeNodeId, seg: u32) -> RoleSeg {
    RoleSeg::Lateral(piece(doc, ext, 0, seg as usize).into())
}

/// The END-cap rim edge over outer-loop canonical segment `seg` of the
/// profile the extrude `ext` sweeps: the edge [`wall`]'s face meets the
/// end cap along, named for a reader of edges.
pub fn end_rim(doc: &editor_core::ProfileDoc, ext: RecipeNodeId, seg: u32) -> StableName {
    rim_edge(ext, CapEnd::End, piece(doc, ext, 0, seg as usize))
}

/// **The four flush families two x-offset blocks share** — the walls
/// y0/y1 (segments 0/2, the `square`/`desc` corner order) and both
/// caps — in ONE place, so every suite that names them agrees about
/// which four they are.
pub fn flush_segs(doc: &editor_core::ProfileDoc, ext: RecipeNodeId) -> [RoleSeg; 4] {
    [
        wall(doc, ext, 0),
        wall(doc, ext, 2),
        RoleSeg::Cap(CapEnd::Start),
        RoleSeg::Cap(CapEnd::End),
    ]
}

/// **Those four families as name pairs**, each side SITED: `at` is
/// the operand (or member) the entity is read at, and the name is the
/// one the extrude `ext` minted, which a pass-through op carries
/// verbatim (N1).
pub fn flush_pairs(
    doc: &ProfileDoc,
    (a_at, a_ext): (RecipeNodeId, RecipeNodeId),
    (b_at, b_ext): (RecipeNodeId, RecipeNodeId),
) -> Vec<(SitedRef<VarId>, SitedRef<VarId>)> {
    let (a_read, b_read) = (out(doc, a_at), out(doc, b_at));
    flush_segs(doc, a_ext)
        .into_iter()
        .zip(flush_segs(doc, b_ext))
        .map(|(a, b)| {
            (
                SitedRef::new(a_read, fname(a_ext, a)),
                SitedRef::new(b_read, fname(b_ext, b)),
            )
        })
        .collect()
}

/// **The read of `node`'s one output** — what an operand reading the
/// node holds, and so what a sited name is read at and a carried name
/// is keyed by (`RoleSeg::From`).
pub fn out(doc: &ProfileDoc, node: RecipeNodeId) -> VarId {
    doc.output(node, 0)
        .unwrap_or_else(|| panic!("{node} defines an output"))
}

/// The member a union-minted name came from, or `None` for a name that
/// is not a carried (`From`) row. Read off the member's READ, which is
/// the only thing that answers: the inner name's minting node is the
/// PROTOTYPE's where a member is a placement of one.
pub fn member_of(doc: &ProfileDoc, name: &StableName) -> Option<RecipeNodeId> {
    match name.path.first() {
        Some(RoleSeg::From { read, .. }) => doc.operation_of(*read),
        _ => None,
    }
}

/// **One ENTITY of one member, in the UNION's own name space** — the
/// row `member_view` puts into that member's operand table, and the
/// shape a union's published table carries.
///
/// The test-side spelling of the crate's `names::member_name`, which
/// is crate-private. One home, so a suite that reads a union's table
/// and a suite that writes an expected row spell the rule once.
pub fn member_entity(
    union: RecipeNodeId,
    member: VarId,
    of: StableName,
    kind: EntityKind,
) -> StableName {
    StableName {
        kind,
        node: union,
        path: vec![RoleSeg::From {
            read: member,
            of: of.into(),
        }],
    }
}

/// The same, for the FACE case every row but a carried-contact one
/// wants.
pub fn member_face(union: RecipeNodeId, member: VarId, of: StableName) -> StableName {
    member_entity(union, member, of, EntityKind::Face)
}

/// **What every at-rest finding says about a declaration, in one
/// vocabulary** — the mate it names and the relation it bears, for a
/// row that wants to compare a whole finding list at once.
///
/// One definition for every suite that asks the question. A CARRIED
/// row's mate is a node of ANOTHER document, so it reports under its
/// own words rather than joining the own-minted ones and reading as
/// this document's.
pub fn relations(
    findings: &[editor_core::AtRestFinding],
) -> Vec<(editor_core::MateRef, &'static str)> {
    findings
        .iter()
        .map(|f| match &f.attribution {
            Some(editor_core::Attribution::Refuted(m)) => (m.mate, "refuted"),
            Some(editor_core::Attribution::Declined(m)) => (m.mate, "declined"),
            Some(editor_core::Attribution::Carried {
                declaration,
                relation,
                ..
            }) => (
                declaration.mate,
                match relation {
                    editor_core::Relation::Refuted => "carried_refuted",
                    editor_core::Relation::Declined => "carried_declined",
                },
            ),
            None => (
                editor_core::MateRef::new(
                    RecipeNodeId::new(0, u64::MAX),
                    editor_core::ConstraintId(editor_core::MintId::new(0, u64::MAX)),
                ),
                "no declaration",
            ),
        })
        .collect()
}

/// **The product's at-rest rows are all contacts between copies**, and
/// there is at least one: a seat no mate speaks for is rows of the two
/// copies it joins (D10), never a mate's.
pub fn assert_copies_rows_alone(doc: &ProfileDoc, ev: &editor_core::Evaluation<f64>) {
    let product =
        editor_core::product_recorded(doc, ev, geom_core::Tol::witness()).expect("it gathers");
    assert!(
        !product.coincidences.is_empty()
            && product
                .coincidences
                .iter()
                .all(|r| matches!(r.by, editor_core::AtRestBy::Copies(..))),
        "the seats are the copies' rows: {:?}",
        product.coincidences
    );
}

/// **No published merged face has a merged face among its
/// constituents** — the N3 flatness rule, asserted over every name of
/// every table an evaluation produced.
///
/// One walker for every suite that evaluates a document, so the rule
/// is checked wherever a `Merged` can be minted — the pair boolean's
/// own tables, the n-ary union's, and whatever wraps either — and not
/// only in the rows written to look for it. The walk is over every
/// name a segment embeds ([`embedded_names`]), so a merged face that
/// reaches a table inside a blend's or a pattern's name is held to
/// the same rule as one at a row's head; and a constituent is read
/// through its descent wrappers ([`is_merged_face`]), so a merged
/// face carried through untouched booleans before being merged again
/// is nesting exactly as a bare one is.
pub fn assert_no_nested_merged<T: geom_core::Decide>(ev: &editor_core::Evaluation<T>) {
    for id in ev.nodes.keys() {
        for (name, _) in tables(ev, *id).into_iter().flat_map(NameTable::iter) {
            let nested = merged_sets(name)
                .into_iter()
                .flat_map(|set| set.iter())
                .find(|c| is_merged_face(c));
            assert!(
                nested.is_none(),
                "node {id:?} published a merged face with a merged constituent {nested:?}: {name:?}"
            );
        }
    }
}

/// True iff `name`, read through its `From` descent chain,
/// is a bare merged face — the shape a flat constituent set never
/// holds. A FRAGMENT of a merged face (`Merged` head with a
/// `Fragment` tail at the foot) is a fragment, not a merge, and is a
/// legitimate constituent.
fn is_merged_face(name: &StableName) -> bool {
    match name.path.as_slice() {
        [RoleSeg::Merged(_)] => true,
        [RoleSeg::From { of: inner, .. }] => is_merged_face(inner),
        _ => false,
    }
}

/// Every `Merged` constituent set reachable from `name`, its own
/// segments included.
fn merged_sets(name: &StableName) -> Vec<&[StableName]> {
    let mut out = Vec::new();
    for seg in &name.path {
        if let RoleSeg::Merged(set) = seg {
            out.push(set.as_slice());
        }
        for inner in embedded_names(seg) {
            out.extend(merged_sets(inner));
        }
    }
    out
}

/// The names one role segment embeds — a derivation argument or a
/// discrimination partner alike, since a merged face is held to the
/// flatness rule wherever it is written.
///
/// The match is EXHAUSTIVE on purpose: a segment added to the
/// vocabulary must be classified here before the suite compiles, so a
/// new name-carrying segment cannot hide a merged face from the walk.
fn embedded_names(seg: &RoleSeg) -> Vec<&StableName> {
    use editor_core::Qualifier;
    match seg {
        RoleSeg::From { of: x, .. }
        | RoleSeg::SectionEdge { face: x, .. }
        | RoleSeg::SplitFragment { parent: x, .. }
        | RoleSeg::CrossingVertex { edge: x, .. }
        | RoleSeg::OnToolVertex { of: x, .. }
        | RoleSeg::Instance { of: x, .. }
        | RoleSeg::InPart { of: x }
        | RoleSeg::Placed { of: x }
        | RoleSeg::BlendFace(x)
        | RoleSeg::CornerFace(x)
        | RoleSeg::Mitre { vertex: x }
        | RoleSeg::TurnFoot { vertex: x }
        | RoleSeg::BandTrim { edge: x, .. }
        | RoleSeg::BandFoot(x)
        | RoleSeg::BandCut(x)
        | RoleSeg::Inner(x)
        | RoleSeg::Rim(x)
        | RoleSeg::HoleRim { of: x, .. } => vec![x.as_ref()],
        RoleSeg::Seam { a: x, b: y }
        | RoleSeg::Crossing {
            edge: x, face: y, ..
        }
        | RoleSeg::TrimEdge {
            edge: x,
            support: y,
        }
        | RoleSeg::FootVertex {
            vertex: x,
            support: y,
        }
        | RoleSeg::EndArc { vertex: x, edge: y } => vec![x.as_ref(), y.as_ref()],
        RoleSeg::EdgeCrossing(run) => run.iter().map(|l| l.line.as_ref()).collect(),
        RoleSeg::Merged(v) | RoleSeg::BandFace(v) => v.iter().collect(),
        RoleSeg::BandCross { edge, band } | RoleSeg::BandSlit { edge, band } => {
            std::iter::once(edge.as_ref()).chain(band).collect()
        }
        RoleSeg::Fragment(Qualifier::Borders(v) | Qualifier::Keeps(v) | Qualifier::Ends(v)) => {
            v.iter().collect()
        }
        RoleSeg::Fragment(Qualifier::OrderAlong { .. })
        | RoleSeg::OutputBody
        | RoleSeg::Cap(_)
        | RoleSeg::Lateral(_)
        | RoleSeg::RimEdge(..)
        | RoleSeg::LateralEdge(_)
        | RoleSeg::CapVertex(..)
        | RoleSeg::Band(_)
        | RoleSeg::BandRim(_)
        | RoleSeg::BandRimPi(_)
        | RoleSeg::BandPi(_)
        | RoleSeg::Meridian(..)
        | RoleSeg::MeridianVertex(..)
        | RoleSeg::RevolveCap(_)
        | RoleSeg::Pole(_)
        | RoleSeg::AxisEdge(_)
        | RoleSeg::SplitBody
        | RoleSeg::SectionFace { .. }
        | RoleSeg::LoftWall(_)
        | RoleSeg::LoftSeam(_) => Vec::new(),
    }
}

/// **A run on a rayon pool of exactly `threads` workers** — `install`
/// binds the pool for the closure, so every `par_iter` under it (the
/// evaluator's level map, the driver's leaf map) runs on that pool.
/// A width-blind row cannot tell an ordered fold from a lucky
/// schedule; a row that names the width can.
pub fn on_pool<R: Send>(threads: usize, run: impl Fn() -> R + Send + Sync) -> R {
    rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .build()
        .expect("the pool builds")
        .install(run)
}

/// **Two overlapping unit blocks on two frames, and their union** — the
/// smallest document whose level schedule and serial order DISAGREE,
/// which is what a row about the parallel node map needs.
///
/// The blocks overlap in all three axes and share no plane, so the union
/// is a generic one and decides at every scalar the evaluator runs.
/// Inserted as profile, extrude, profile, extrude, the second block's
/// placement against the first, union, so the serial order (Kahn, least
/// id first) finishes the first block before starting the second, while
/// the levels are the two profiles, the two extrudes, the placement,
/// then the union: the first two levels hold two independent nodes, and
/// a fold that recorded level by level would interleave the blocks.
/// Returns the document and the union's id.
pub fn two_blocks_and_their_union(label: &str) -> (ProfileDoc, RecipeNodeId) {
    let mut r = Recorder {
        doc: ProfileDoc::empty_derived(label, Tol::witness()),
        ..Recorder::new()
    };
    let block = |r: &mut Recorder, [x, y]: [f64; 2]| {
        let p = r.profile(
            [0.0; 3],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![square(x, y, 0.5)],
        );
        r.insert(Node::Extrude {
            profile: p.into(),
            distance: len(1.0),
        })
    };
    let a = block(&mut r, [0.0, 0.0]);
    let b = block(&mut r, [0.5, 0.25]);
    // The second block, a construction of its own, placed against the
    // first half way up it: one space, so the union reads both.
    let b = r.against(
        a,
        b,
        &frame_at([0.0, 0.0, 0.5], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]),
    );
    let union = r.insert(Node::Union {
        members: editor_core::Bodies::Spelled(vec![a.into(), b.into()]),
    });
    (r.doc, union)
}

/// **The names an authored selection names** ([`editor_core::Operand::Select`]).
///
/// # Panics
///
/// If `operand` authors no selection.
pub fn authored_names(operand: &editor_core::Operand) -> Vec<StableName> {
    match operand {
        editor_core::Operand::Select { names, .. } => names.clone(),
        other => panic!("{other} authors no selection"),
    }
}

/// **The names the selection `var` holds** in `doc`.
///
/// # Panics
///
/// If `var` is not a selection `doc` holds.
pub fn selected(doc: &ProfileDoc, var: editor_core::VarId) -> Vec<StableName> {
    doc.selection(var)
        .unwrap_or_else(|| panic!("{var:?} is a selection"))
        .names
        .clone()
}

/// **The body of the one selection of `doc` naming `name`**: what a
/// `Rebind` of that name is addressed by.
///
/// # Panics
///
/// If no selection, or more than one, names it.
pub fn body_selecting(doc: &ProfileDoc, name: &StableName) -> Option<editor_core::VarId> {
    let bodies: Vec<editor_core::VarId> = doc
        .vars()
        .values()
        .filter_map(|var| var.def().select())
        .filter(|select| select.names.contains(name))
        .map(|select| select.body)
        .collect();
    let [one] = bodies.as_slice() else {
        panic!("one selection names {name:?}, not {bodies:?}");
    };
    Some(*one)
}

/// **The one selection `node` reads** in `doc`.
///
/// # Panics
///
/// If `node` reads no selection, or several.
pub fn selection_read(doc: &ProfileDoc, node: RecipeNodeId) -> editor_core::VarId {
    let reads: Vec<editor_core::VarId> = doc
        .node(node)
        .map(|n| n.operand_rows())
        .unwrap_or_default()
        .into_iter()
        .map(|(_, var)| var)
        .filter(|var| doc.selection(*var).is_some())
        .collect();
    let [one] = reads.as_slice() else {
        panic!("node {node:?} reads one selection, not {reads:?}");
    };
    *one
}

/// **A selection's strand row** as the delete or reshaping reports it:
/// the selection `carrier` reads, said by `carrier`, as `doc` speaks it.
pub fn selection_strand(
    doc: &ProfileDoc,
    carrier: RecipeNodeId,
    name: editor_core::SpokenName,
    took: editor_core::Took,
) -> editor_core::Maintenance {
    editor_core::Maintenance::StrandedSelection {
        var: doc.spoken_var(selection_read(doc, carrier)),
        readers: vec![doc.spoken(carrier)],
        name,
        took,
    }
}

/// **An edit's maintenance with its anonymous-variable removals left
/// out**: a slot or program rewrite retires the variables its old
/// values were written in (VR7), which a row about strands does not
/// ask about. A retirement that took a tolerance is kept
/// (`Maintenance::is_silent_retirement`).
pub fn without_anonymous(
    maintenance: &[editor_core::Maintenance],
) -> Vec<editor_core::Maintenance> {
    maintenance
        .iter()
        .filter(|m| !m.is_silent_retirement())
        .cloned()
        .collect()
}

/// **Two documents that say the same thing**: one node order, each
/// node the same as written ([`Node::written`]: an anonymous variable
/// its value or definition, a named one its reader), and one named
/// variable table. What two edits that write the same values through
/// different doors land: the anonymous variables they mint are their
/// own, so the documents are not [`ProfileDoc::bit_eq`].
pub fn same_as_written(a: &ProfileDoc, b: &ProfileDoc) -> bool {
    let named = |doc: &ProfileDoc| -> Vec<_> {
        doc.var_names()
            .iter()
            .map(|(id, name)| (*id, name.clone(), doc.var(*id).cloned()))
            .collect()
    };
    a.ids() == b.ids()
        && a.ids()
            .iter()
            .all(|&id| a.node(id).map(|n| n.written(a)) == b.node(id).map(|n| n.written(b)))
        && named(a) == named(b)
}

/// How many continuous free variables `doc` holds — every parameter
/// axis, named or written in a slot (VR8).
pub fn continuous_vars(doc: &ProfileDoc) -> usize {
    doc.free_vars()
        .filter(|(_, free)| matches!(free, editor_core::FreeVar::Continuous { .. }))
        .count()
}

/// `a`, `b` standing on `a`'s top raised by `lift` (0 glues `b`'s bottom
/// to `a`'s top, −0.1 sinks it, 0.1 lifts it clear), and `c` rising
/// through `a`'s top across `b`'s far end, its x = 1.2 wall cutting
/// `b`'s y = 0.25 bottom rim.
pub fn glued_rim(lift: f64) -> impl Fn(ProfileDoc) -> (ProfileDoc, Vec<RecipeNodeId>) {
    move |doc| {
        let (doc, a) = glued_block(doc, (0.0, 2.0), (0.0, 1.0), 1.0);
        let (doc, b) = glued_block(doc, (0.5, 1.5), (0.25, 0.75), 0.5);
        let (doc, b) = against(
            doc,
            a,
            b,
            &editor_core::Frame::translation([0.0, 0.0, 1.0 + lift]),
        );
        let (doc, c) = glued_block(doc, (1.2, 2.5), (-0.2, 1.2), 1.3);
        let (doc, c) = against(doc, a, c, &editor_core::Frame::translation([0.0, 0.0, 0.5]));
        (doc, vec![a, b, c])
    }
}

/// An axis-aligned block on its own xy plane, extruded `dz`:
/// [`glued_rim`]'s members, each after the first placed against it.
fn glued_block(
    doc: ProfileDoc,
    (x0, x1): (f64, f64),
    (y0, y1): (f64, f64),
    dz: f64,
) -> (ProfileDoc, RecipeNodeId) {
    let p_node = Node::Profile(desc(vec![vec![(x0, y0), (x1, y0), (x1, y1), (x0, y1)]]));
    let (doc, p) = insert(doc, p_node);
    insert(
        doc,
        Node::Extrude {
            profile: p.into(),
            distance: len(dz),
        },
    )
}

/// **`name` read at `at`**, of any entity kind: a one-name selection of
/// `at`'s body.
pub fn read_at(at: RecipeNodeId, name: StableName) -> editor_core::Operand {
    editor_core::Operand::select(at, vec![name])
}

/// [`read_at`] at the node that minted `name`.
pub fn read_minted(name: StableName) -> editor_core::Operand {
    let at = name.node;
    read_at(at, name)
}

/// **A block's corner frame** (`PoseDef::Through`): `z` along the
/// lateral edge `edge`, its origin on the edge level with the cap
/// vertex `corner`, `x` towards it.
pub fn corner_frame(
    edge: impl Into<editor_core::Operand>,
    corner: impl Into<editor_core::Operand>,
) -> editor_core::Operand {
    let r = editor_core::pose::RoleRead::plain;
    editor_core::Operand::Pose(Box::new(editor_core::pose::PoseDef::Through {
        axis: r(axis(edge)),
        point: r(point_of(corner)),
    }))
}

/// **A frame slid `x` along its own `x`, then `y` along its own `y`.**
pub fn slid_across(frame: editor_core::Operand, x: f64, y: f64) -> editor_core::Operand {
    use editor_core::pose::{PoseDef, PoseRole, RoleRead};
    let slide = |pose: editor_core::Operand, role, by: f64| {
        if by == 0.0 {
            return pose;
        }
        editor_core::Operand::Pose(Box::new(PoseDef::Slide {
            pose: RoleRead::plain(pose.clone()),
            along: RoleRead::through(pose, role),
            by: len(by),
        }))
    };
    slide(slide(frame, PoseRole::X, x), PoseRole::Y, y)
}

/// **A block seated on another by one pin** (FORK-S3O): `top`'s corner
/// frame equated with `base`'s, slid `(x, y)` across it. The seat three
/// plane mates — a rest and two walls — used to state, which fix one
/// rotation twice and which the bundle rule refuses.
pub fn frame_seat(
    top: editor_core::Operand,
    base: editor_core::Operand,
    x: f64,
    y: f64,
) -> editor_core::Mate<Formula> {
    mate(
        top,
        slid_across(base, x, y),
        editor_core::ContactClass::Rest,
    )
}
