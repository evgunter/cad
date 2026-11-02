//! **The fixture doors a test in this crate authors a document with** —
//! literals of each dimension, the world xy frame, an axis-aligned
//! rectangle, one edit or insert through the document's own `apply`, the
//! display tolerances the suites index at, and the plate indexed at one.
//!
//! Module kind: **vocabulary** — it names no driver type and no toolkit
//! type; every door is a pure function over document and display
//! values.
//!
//! Behind this crate's `test-support` feature, which only its own self
//! dev-dependency turns on: the unit-test modules reach it as
//! `crate::test_support` and `tests/` as `viewer::test_support` (through
//! `tests/common`'s re-export), so the two read ONE definition.
//!
//! Whether a door carries an oracle is a question about that door, asked
//! in its own docs where the answer is not "no": the δ doors fix what a
//! pick or a draw is measured at.

// Panicking is a fixture's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use pncad::document::{
    AuthoredNode, BooleanValue, CancelToken, ContentPin, Dimension, Doc, DocEdit, DocRef,
    DocumentId, EditError, EvalOptions, Evaluation, Formula, FreeVar, LoopProgram, Node,
    NodeErrorKind, PartFault, ProfileProgram, RecipeNodeId, ValuePayload, VarName, apply, evaluate,
};
use pncad::geom_core::{Point2, Tol};
#[cfg(test)]
use pncad::profile::{PathErrorKind, TipState, Verb};
use pncad::profile::{Step, Target};

use crate::generation::Generation;
use crate::parts::PartFiles;
use crate::pickindex::{PickIndex, PictureKey};
use crate::scene::DisplayTolerance;

// --- literals -------------------------------------------------------

/// A literal of each dimension, and a point of two lengths —
/// `editor_core::test_support`'s, over the same `Expr` the façade
/// re-exports, so this crate reads the kernel's fixtures rather than a
/// second spelling of them.
pub use editor_core::test_support::{ang, len, len2, scl};

/// A length literal that remembers it was WRITTEN in millimetres —
/// `len` lowers canonically and carries no notation, which is what a
/// row about the unit a literal keeps cannot use.
pub fn len_mm(metres: f64) -> Formula {
    Formula::literal_with_unit(metres, Dimension::Length, pncad::prelude::MM.def())
        .expect("a finite length")
}

/// Three length literals — a point, a translation.
pub fn len3(v: [f64; 3]) -> [Formula; 3] {
    [len(v[0]), len(v[1]), len(v[2])]
}

/// The constant zero translation: three lengths no one wrote, so they
/// mint no variables (`[variables.literals]`).
pub fn no_translation() -> [Formula; 3] {
    std::array::from_fn(|_| Formula::zero(Dimension::Length))
}

/// A translation from its three components: each zero is the constant
/// `0` and each other component a literal, so only the components
/// written mint variables (`[variables.literals]`).
pub fn translation(v: [f64; 3]) -> [Formula; 3] {
    v.map(|c| {
        if c == 0.0 {
            Formula::zero(Dimension::Length)
        } else {
            len(c)
        }
    })
}

/// Three dimensionless literals — a normal, a direction, an axis.
pub fn scl3(v: [f64; 3]) -> [Formula; 3] {
    [scl(v[0]), scl(v[1]), scl(v[2])]
}

/// Two dimensionless literals — a direction in a sketch frame.
pub fn scl2(v: [f64; 2]) -> [Formula; 2] {
    [scl(v[0]), scl(v[1])]
}

// --- the document's own edit door -----------------------------------
//
// Authored through `apply`, in the order a user would: a fixture that
// reached past it would be testing a document the edit vocabulary
// cannot produce. The fixtures are part-less — no mate, no group — so
// the reach is the refusing one and is never asked.

/// Apply one edit, answering the new document and any minted id.
///
/// # Panics
///
/// If the document refuses the edit — the fixture is wrong. A row whose
/// PREMISE is that the edit applies calls [`try_edited`] and names that
/// premise in its own `.expect(..)`.
pub fn edited(
    doc: &Doc<ProfileProgram>,
    edit: DocEdit<ProfileProgram>,
    tol: Tol,
) -> (Doc<ProfileProgram>, Option<RecipeNodeId>) {
    try_edited(doc, edit, tol).expect("the fixture's edit applies")
}

/// [`edited`] with the refusal handed back, for a row whose premise is
/// that the edit applies: the row's `.expect(..)` says which premise
/// failed.
///
/// # Errors
///
/// The document's own refusal of the edit, unaltered.
pub fn try_edited(
    doc: &Doc<ProfileProgram>,
    edit: DocEdit<ProfileProgram>,
    tol: Tol,
) -> Result<(Doc<ProfileProgram>, Option<RecipeNodeId>), EditError> {
    let applied = apply(doc, &edit, tol)?;
    Ok((applied.doc, applied.record.minted))
}

/// Insert a node through the document's own door, answering the new
/// document and the minted id.
///
/// # Panics
///
/// If the document refuses the node — the fixture is wrong. A row whose
/// PREMISE is that the document admits the node calls [`try_inserted`]
/// and names that premise in its own `.expect(..)`.
pub fn inserted(
    doc: &Doc<ProfileProgram>,
    node: AuthoredNode,
    tol: Tol,
) -> (Doc<ProfileProgram>, RecipeNodeId) {
    try_inserted(doc, node, tol).expect("the fixture's edit applies")
}

/// [`inserted`] with the refusal handed back, for a row whose premise is
/// that the document admits the node: the row's `.expect(..)` says which
/// premise failed.
///
/// # Errors
///
/// The document's own refusal of the insert, unaltered.
pub fn try_inserted(
    doc: &Doc<ProfileProgram>,
    node: AuthoredNode,
    tol: Tol,
) -> Result<(Doc<ProfileProgram>, RecipeNodeId), EditError> {
    let (doc, minted) = try_edited(
        doc,
        DocEdit::InsertNode {
            node: Box::new(node),
            fresh: Vec::new(),
        },
        tol,
    )?;
    Ok((doc, minted.expect("an insert mints an id")))
}

// --- the nodes a fixture sketches with ------------------------------

/// A spoken node built by hand, for a row a test builds without a
/// document — `editor_core::test_support`'s.
pub use editor_core::test_support::spoken;

/// An axis-aligned rectangular loop, `w` by `h`, its lower-left
/// corner at `origin` in the plane's own coordinates, counter-clockwise
/// from that corner — the loop [`rectangle`] draws, for a
/// `SessionOp::AddProfile` that takes loops rather than a node.
///
/// # Panics
///
/// Unless `w` and `h` are both positive: a non-positive side would
/// turn the winding or collapse the loop, and "lower-left,
/// counter-clockwise" would stop being true of what it returns.
pub fn rectangle_loop(origin: [f64; 2], w: f64, h: f64) -> LoopProgram<Formula> {
    assert!(
        w > 0.0 && h > 0.0,
        "a rectangle has positive sides: {w} x {h}"
    );
    let [x0, y0] = origin;
    LoopProgram::polygon([(x0, y0), (x0 + w, y0), (x0 + w, y0 + h), (x0, y0 + h)])
        .expect("finite corners")
}

/// An axis-aligned rectangular profile node's payload:
/// [`rectangle_loop`] drawn on its own plane. `square` is this with two
/// equal sides at the plane origin, and a fixture whose block sits
/// elsewhere moves `origin`.
pub fn rectangle(origin: [f64; 2], w: f64, h: f64) -> AuthoredNode {
    Node::Profile(ProfileProgram {
        loops: vec![rectangle_loop(origin, w, h)],
        ids: Vec::new(),
        on: None,
    })
}

/// A square profile node's payload, `side` metres on a side, at the
/// plane origin.
pub fn square(side: f64) -> AuthoredNode {
    rectangle([0.0, 0.0], side, side)
}

// --- documents built from the doors above ---------------------------

/// **A document holding one declared variable and nothing else** —
/// the fixture both panel suites build their parameter rows on.
///
/// `label` is the document's derived name, so two fixtures in one
/// binary cannot share an identity. No oracle: it is the spelling of
/// `Doc::empty_derived` plus one `DeclareVar`, and what each row
/// asserts is about the `value` it handed in.
pub fn declared(label: &str, name: &VarName, value: FreeVar, tol: Tol) -> Doc<ProfileProgram> {
    let doc: Doc<ProfileProgram> = Doc::empty_derived(label, tol);
    edited(
        &doc,
        DocEdit::DeclareVar {
            name: name.clone(),
            def: pncad::document::VarDecl::Free(value),
        },
        tol,
    )
    .0
}

/// **`doc`'s node ids without the world** ([`Doc::ids`] less
/// `RecipeNodeId::WORLD`): the nodes an author made, in id order.
pub fn authored_ids(doc: &Doc<ProfileProgram>) -> Vec<RecipeNodeId> {
    doc.ids()
        .into_iter()
        .filter(|&id| id != RecipeNodeId::WORLD)
        .collect()
}

/// **`doc` with `body` placed in the world** at the identity (A10): a
/// placement of it, standing where its body stands ([`pinned`]),
/// answering the document and the placement.
pub fn placed(
    doc: &Doc<ProfileProgram>,
    body: RecipeNodeId,
    tol: Tol,
) -> (Doc<ProfileProgram>, RecipeNodeId) {
    placed_read(doc, pncad::document::Operand::from(body), tol)
}

/// [`placed`] of any body read — an operation's port, an instance's
/// copy. The product is then the world's copies
/// ([`editor_core::test_support::assemble_world_copies`]).
pub fn placed_read(
    doc: &Doc<ProfileProgram>,
    read: pncad::document::Operand,
    tol: Tol,
) -> (Doc<ProfileProgram>, RecipeNodeId) {
    let (doc, placement) = inserted(
        doc,
        Node::place(
            pncad::document::BodyRead::plain(read),
            pncad::document::Operand::Var(pncad::document::VarId::WORLD),
            pncad::document::stand().bundle(),
        ),
        tol,
    );
    (
        editor_core::test_support::assemble_world_copies(doc, tol),
        placement,
    )
}

/// The copy `placement` defines, standing where its body stands in the
/// space it is placed into ([`pncad::document::stand`]): the identity
/// bundle, no mate and the constant `0` on every freedom, which reads
/// nothing of the copy.
pub fn pinned(doc: &Doc<ProfileProgram>, placement: RecipeNodeId, tol: Tol) -> Doc<ProfileProgram> {
    constrained(doc, placement, pncad::document::stand(), tol)
}

/// **`body`'s copy placed against `target`'s own geometry at `at`**
/// ([`editor_core::test_support::placed_against_in`]), its mate and
/// values written, evaluated under `opts` — a document whose bodies read a store
/// (an instance's part) is placed under the options that reach it.
/// Answers the document and the placement.
///
/// # Panics
///
/// If the door refuses: a fixture's bug.
pub fn against_in(
    doc: &Doc<ProfileProgram>,
    target: pncad::document::Operand,
    body: pncad::document::Operand,
    at: &pncad::document::Frame,
    opts: &EvalOptions,
    tol: Tol,
) -> (Doc<ProfileProgram>, RecipeNodeId) {
    let placed = editor_core::test_support::placed_against_in(doc, opts, &target, &body, at, tol);
    let space = doc
        .operand_space(&target)
        .expect("the target is in a space");
    inserted(
        doc,
        Node::place(
            pncad::document::BodyRead::plain(body),
            pncad::document::Operand::Var(space),
            placed.bundle(),
        ),
        tol,
    )
}

/// **`mate` added to `placement`'s bundle and `values` written over
/// it**, as the mate tool's op writes them ([`DocEdit::AddConstraint`],
/// which drops the values whose freedoms the mate fixes and writes `0`
/// on each it leaves unset, then [`crate::session::valued`]), the
/// placement re-targeted into the space the mate's `to` reads first.
/// Answers the document and the mate's address.
///
/// # Panics
///
/// If the door refuses: a fixture's bug.
pub fn mated(
    doc: &Doc<ProfileProgram>,
    placement: RecipeNodeId,
    mate: pncad::document::Mate<Formula>,
    values: Vec<(pncad::document::Coord, Formula)>,
    tol: Tol,
) -> (Doc<ProfileProgram>, pncad::document::MateRef) {
    // A mate into another space re-targets the copy here, an explicit
    // fixture move, never the gesture's.
    let doc = retargeted(doc, placement, doc.operand_space(&mate.to), tol);
    let applied = apply(
        &doc,
        &DocEdit::AddConstraint {
            placement,
            mate: Box::new(mate),
            fresh: Vec::new(),
        },
        tol,
    )
    .unwrap_or_else(|refusal: EditError| panic!("the mate is admitted: {refusal}"));
    let id = applied.record.minted_constraints[0];
    let doc = if values.is_empty() {
        applied.doc
    } else {
        let held = crate::session::held_bundle(&applied.doc, placement);
        written(&applied.doc, placement, valued(held, values), tol)
    };
    (doc, pncad::document::MateRef::new(placement, id))
}

/// What a placing door answers, written onto `placement` whole
/// ([`editor_core::Placed::write`]), the placement first re-targeted
/// into the space the mate's `to` reads where that is not its target.
fn constrained(
    doc: &Doc<ProfileProgram>,
    placement: RecipeNodeId,
    placed: editor_core::Placed,
    tol: Tol,
) -> Doc<ProfileProgram> {
    written(doc, placement, placed.bundle(), tol)
}

/// `placement` re-targeted into `space` where that is not its target:
/// the identity first, which reads nothing, so no mate of the bundle it
/// held strands the re-target.
fn retargeted(
    doc: &Doc<ProfileProgram>,
    placement: RecipeNodeId,
    space: Option<pncad::document::VarId>,
    tol: Tol,
) -> Doc<ProfileProgram> {
    let Some(space) = space.filter(|&space| doc.node_space(placement) != Some(space)) else {
        return doc.clone();
    };
    let edits = [
        pncad::document::stand().write(placement),
        DocEdit::SetParam {
            node: placement,
            slot: pncad::document::SlotId::Operand(pncad::document::OperandSlot::Space),
            value: pncad::document::SlotValue::Read(pncad::document::Operand::Var(space)),
            fresh: Vec::new(),
        },
    ];
    edits.into_iter().fold(doc.clone(), |doc, edit| {
        apply(&doc, &edit, tol)
            .unwrap_or_else(|refusal: EditError| panic!("the placement re-targets: {refusal}"))
            .doc
    })
}

/// `constraints` written onto `placement` whole, re-targeted first into
/// the space the bundle's first mate's `to` reads ([`retargeted`]).
fn written(
    doc: &Doc<ProfileProgram>,
    placement: RecipeNodeId,
    constraints: Vec<pncad::document::Constraint<Formula>>,
    tol: Tol,
) -> Doc<ProfileProgram> {
    let space = constraints
        .iter()
        .find_map(|c| c.mate())
        .and_then(|mate| doc.operand_space(&mate.to));
    let doc = retargeted(doc, placement, space, tol);
    let edits = [DocEdit::SetConstraints {
        placement,
        constraints,
        fresh: Vec::new(),
    }];
    edits.into_iter().fold(doc, |doc, edit| {
        apply(&doc, &edit, tol)
            .unwrap_or_else(|refusal: EditError| {
                panic!("the placement's constraints are admitted: {refusal}")
            })
            .doc
    })
}

/// **The copy `placement` defines, pinned to the world at `pose`**:
/// its own coordinates moved to `pose` by values on the world's axes
/// ([`editor_core::test_support::moves_at`], a quarter turn written
/// `turn/4`), which read nothing of the copy, so a copy whose body
/// does not evaluate is pinned all the same and poisoned through its
/// body, as the product reads it. `opts` is unread: the pin evaluates
/// nothing.
pub fn pin(
    doc: &Doc<ProfileProgram>,
    placement: RecipeNodeId,
    pose: &pncad::document::Frame,
    _opts: &EvalOptions,
    tol: Tol,
) -> Doc<ProfileProgram> {
    let placed = editor_core::Placed::unmated(editor_core::test_support::moves_at(pose));
    constrained(doc, placement, placed, tol)
}

/// The `&mut` spelling of `inserted`: insert a node in place and
/// answer the minted id, for a fixture that threads one document
/// through a sequence of edits rather than rebinding at each one.
/// Same call and same refusal behaviour — only the caller differs.
pub fn insert_into(doc: &mut Doc<ProfileProgram>, node: AuthoredNode, tol: Tol) -> RecipeNodeId {
    let (applied, id) = inserted(doc, node, tol);
    *doc = applied;
    id
}

/// The `&mut` spelling of `edited`, for an edit whose minted id (if
/// any) the caller does not want.
pub fn edit_into(doc: &mut Doc<ProfileProgram>, edit: DocEdit<ProfileProgram>, tol: Tol) {
    let (applied, _) = edited(doc, edit, tol);
    *doc = applied;
}

/// **A square profile inserted**, answering the document and the
/// profile's id.
pub fn square_profile(
    doc: &Doc<ProfileProgram>,
    side: f64,
    tol: Tol,
) -> (Doc<ProfileProgram>, RecipeNodeId) {
    inserted(doc, square(side), tol)
}

/// The boss-on-a-face scene's block: 40 × 20 × 10 mm, centred on the
/// world xy frame and extruded up ([`boss_on_block`]).
pub const BOSS_BLOCK: [f64; 3] = [0.04, 0.02, 0.01];
/// The scene's boss: a disc of this radius on the block's top cap…
pub const BOSS_RADIUS: f64 = 0.005;
/// …extruded this far up.
pub const BOSS_HEIGHT: f64 = 0.004;

/// **A block and a boss standing on its top cap**, answering the
/// document, the block and the boss — FLUSH with each other at that cap
/// by construction, which is the whole point of the scene.
///
/// Each body is a construction in its own coordinates, placed in the
/// world: the block where it stands, the boss at the block's depth up
/// the world's `z`, slid there by its values. `tests/creation_ops.rs`'s
/// boss row authors the scene through the op vocabulary
/// ([`crate::session::SessionOp::SketchOnFace`]), because the gesture
/// is what that row is about; this is the scene alone, each body
/// placed. Its union's volume is `boss_on_block_union_volume`.
pub fn boss_on_block(label: &str, tol: Tol) -> (Doc<ProfileProgram>, RecipeNodeId, RecipeNodeId) {
    let [width, height, depth] = BOSS_BLOCK;
    let (doc, section) = inserted(
        &Doc::empty_derived(label, tol),
        rectangle([-width / 2.0, -height / 2.0], width, height),
        tol,
    );
    let (doc, block) = inserted(
        &doc,
        Node::Extrude {
            profile: section.into(),
            distance: len(depth),
        },
        tol,
    );
    let (doc, disc) = inserted(
        &doc,
        Node::Profile(ProfileProgram {
            loops: vec![LoopProgram::circle(0.0, 0.0, BOSS_RADIUS).expect("a finite circle")],
            ids: Vec::new(),
            on: None,
        }),
        tol,
    );
    let (doc, boss) = inserted(
        &doc,
        Node::Extrude {
            profile: disc.into(),
            distance: len(BOSS_HEIGHT),
        },
        tol,
    );
    // Both bodies are the world (A10), block first.
    let (doc, _) = placed(&doc, block, tol);
    let on_top = pncad::document::Frame {
        translation: [0.0, 0.0, depth],
        ..pncad::document::Frame::IDENTITY
    };
    let boss_read = pncad::document::Operand::from(boss);
    let at = editor_core::Placed::unmated(editor_core::test_support::moves_at(&on_top));
    let (doc, _) = inserted(
        &doc,
        Node::place(
            pncad::document::BodyRead::plain(boss_read),
            pncad::document::Operand::Var(pncad::document::VarId::WORLD),
            at.bundle(),
        ),
        tol,
    );
    (doc, block, boss)
}

/// **`doc` with `node` inserted, evaluated from scratch** — what the
/// kernel answers for a node, for a row comparing a session door's
/// verdict with the kernel's own. Answers the evaluation and the id.
pub fn evaluated_insert(
    doc: &Doc<ProfileProgram>,
    node: AuthoredNode,
    tol: Tol,
) -> (Evaluation<f64>, RecipeNodeId) {
    let (_, eval, id) = inserted_and_evaluated(doc, node, tol);
    (eval, id)
}

/// [`evaluated_insert`] with the document the evaluation was taken of,
/// for a row that speaks the evaluation's ids.
pub fn inserted_and_evaluated(
    doc: &Doc<ProfileProgram>,
    node: AuthoredNode,
    tol: Tol,
) -> (Doc<ProfileProgram>, Evaluation<f64>, RecipeNodeId) {
    let (doc, id) = inserted(doc, node, tol);
    let eval = evaluate(
        &doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        tol,
    );
    (doc, eval, id)
}

/// **The volume of `node`'s single body in `eval`** — an extrude's, a
/// blend's or a boolean's. A node with no value panics with its own
/// recorded error, not just the absence of a value. It is read at
/// `topo`'s pinned resolution (1024·ε of mean boundary displacement),
/// the one the viewer suites' volume pins are stated at.
pub fn evaluated_volume(eval: &Evaluation<f64>, node: RecipeNodeId, tol: Tol) -> f64 {
    let value = eval
        .value(node)
        .unwrap_or_else(|| panic!("the node evaluated: {:?}", eval.node_error(node)));
    let body = match &value.payload {
        ValuePayload::Body(body) | ValuePayload::Boolean(BooleanValue::Body { body, .. }) => body,
        other => panic!("expected a body, got {other:?}"),
    };
    pncad::topo::mass_properties(
        &pncad::topo::AtRestBody::validate(body.body().clone(), tol)
            .expect("the measured body passes tier 3"),
        tol,
        pncad::topo::test_support::pinned_resolution(tol),
    )
    .expect("mass properties")
    .met()
    .expect("the pinned resolution is met")
    .volume
}

// --- a part reference the evaluation refused ------------------------

/// The file a unit-test fixture names a part by: an instance row's,
/// and [`part_refused`]'s scan.
pub const PART_FILE: &str = "post.pncad";

/// **An instance whose reference refused with `fault`**, as the
/// evaluation carries it, and one scan of the directory that names the
/// part [`PART_FILE`].
pub fn part_refused(fault: PartFault) -> (NodeErrorKind, PartFiles) {
    let doc_ref = DocRef {
        id: DocumentId::derive("refused-part"),
        pin: ContentPin::of_bytes(b"refused-part v1"),
    };
    let files = PartFiles::Scanned([(doc_ref.id, PART_FILE.to_owned())].into());
    (NodeErrorKind::Part { doc_ref, fault }, files)
}

// --- the display tolerances the suites index at ---------------------

/// The display tolerance the plate-scale suites run the display
/// pipeline at, 2*10^-4 m — whatever they hand it to: an index build,
/// a pick cache sync, a fit request.
///
/// Two bounds pull opposite ways on it — coarser is cheaper to run,
/// finer resolves more of a curved face — and this value is where the
/// suites that share it settled. A suite whose fixture is a different
/// size chooses its own; `tests/common`'s `asm::delta` is the assembly
/// fixture's.
pub fn plate_delta() -> DisplayTolerance {
    DisplayTolerance::new(2.0e-4).expect("a positive delta")
}

/// **The spike plate, evaluated and indexed at [`plate_delta`]**, with
/// the world placement whose copy it draws — the picture a unit row
/// marks, loads or names edges in, keyed by that placement.
///
/// Evaluated through the kernel door rather than through a session: the
/// rows that read it are vocabularies and a session is a driver
/// (`crates/viewer/README.md`, Module boundaries), and the index only
/// ever wanted the evaluation.
pub fn plate_indexed(tol: Tol) -> (Evaluation<f64>, PickIndex, RecipeNodeId) {
    let (doc, _) = crate::scene::plate_with_hole(tol).expect("the plate authors");
    let eval = evaluate(
        &doc,
        None,
        &CancelToken::default(),
        &EvalOptions::default(),
        tol,
    );
    let index = PickIndex::build(
        &doc,
        &eval,
        PictureKey::of(
            Generation::FIRST,
            plate_delta(),
            crate::world::Shown::none(),
        ),
        tol,
    )
    .expect("the plate indexes");
    let [placement] = doc.placements()[..] else {
        panic!("the plate is placed once")
    };
    (eval, index, placement)
}

/// **An index seam that refuses the whole index on demand**: it answers
/// what `inner` answers, except that while its switch is on every build
/// is [`crate::pickindex::PickIndexError::DrawnTwice`]. A refusal of the
/// index as a whole is a fault of the index itself, which no document
/// produces, so a row about one injects it here.
pub struct RefusingIndexer<S> {
    inner: S,
    refusing: std::sync::Arc<std::sync::atomic::AtomicBool>,
}

impl<S> RefusingIndexer<S> {
    /// A seam over `inner` that refuses from the start, and the switch.
    pub fn new(inner: S) -> (Self, std::sync::Arc<std::sync::atomic::AtomicBool>) {
        let refusing = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));
        (
            Self {
                inner,
                refusing: std::sync::Arc::clone(&refusing),
            },
            refusing,
        )
    }
}

impl<S: crate::evalseam::IndexService> crate::evalseam::IndexService for RefusingIndexer<S> {
    fn submit(&mut self, request: crate::evalseam::IndexRequest) {
        self.inner.submit(request);
    }

    fn poll(&mut self) -> Option<crate::evalseam::IndexDone> {
        let mut done = self.inner.poll()?;
        if self.refusing.load(std::sync::atomic::Ordering::Relaxed) {
            done.index = Err(crate::pickindex::PickIndexError::DrawnTwice {
                node: RecipeNodeId::new(0, 3),
                body: 0,
            });
        }
        Some(done)
    }

    fn busy(&self) -> bool {
        self.inner.busy()
    }
}

/// **The naming layer's refusal for a drawn edge of `node`'s output
/// `body`** — what a part whose table could not name that edge holds in
/// its slot. The edge's own key is not the index's to know, so it is
/// the default one.
pub fn unnamed_edge(node: RecipeNodeId, body: u32) -> pncad::select::UnnamedEntity {
    pncad::select::UnnamedEntity {
        node,
        entity: editor_core::EntityRef {
            body,
            key: editor_core::EntityKey::Edge(pncad::topo::EdgeKey::default()),
        },
    }
}

/// The display tolerance the corpus suites hand the pick seam,
/// 2*10^-3 m.
///
/// An order coarser than [`plate_delta`], for a reason those suites do
/// not share: the corpus holds documents whose tessellation at the
/// application's own δ is large enough that a suite walking all of
/// them pays for every facet.
pub fn corpus_delta() -> DisplayTolerance {
    DisplayTolerance::new(2.0e-3).expect("a positive delta")
}

/// The display tolerance the GUI-2 suites index the gallery ring at,
/// 2*10^-3 m — a cost choice: those rows are about the selection walk,
/// not the facet count.
pub fn ring_delta() -> DisplayTolerance {
    DisplayTolerance::new(2.0e-3).expect("a positive delta")
}

/// **Two legs of a path from `(x, y)`**: `at`, a leg along `+x`, then
/// one along `+y`, ending at `(x + 0.01, y + 0.01)` on a leg end that
/// arrives heading `+y` — the chain the path-preview rows leave
/// unfinished or cut short.
pub fn two_legs(x: f64, y: f64) -> Vec<Step<f64>> {
    vec![
        Step::At(Point2::new(x, y)),
        Step::LineTo(Target::Point(Point2::new(x + 0.01, y))),
        Step::LineTo(Target::Point(Point2::new(x + 0.01, y + 0.01))),
    ]
}

/// **A straight leg to a point**, `line_to (x, y)`.
#[cfg(test)]
pub fn line_to(x: f64, y: f64) -> Step<f64> {
    Step::LineTo(Target::Point(Point2::new(x, y)))
}

/// **A step no leg end takes**: an `at`, which is ill-typed after a
/// leg — the refused step the walk-back rows put after a chain.
#[cfg(test)]
pub fn ill_typed() -> Step<f64> {
    Step::At(Point2::new(0.1, 0.1))
}

/// **A last leg onto the start of a square**, from `(0, 0)`: every
/// leg of it by `line_to`, the last targeting `(0, 0)` by value.
#[cfg(test)]
pub fn square_onto_the_start() -> Vec<Step<f64>> {
    let mut steps = two_legs(0.0, 0.0);
    steps.extend([line_to(0.0, 0.01), line_to(0.0, 0.0)]);
    steps
}

/// The vertices of [`square_onto_the_start`], and of every square the
/// rows below draw from `(0, 0)`.
#[cfg(test)]
pub fn square_vertices() -> Vec<[f64; 2]> {
    vec![[0.0, 0.0], [0.01, 0.0], [0.01, 0.01], [0.0, 0.01]]
}

/// **An unfinished chain whose provisional close is refused on its
/// geometry**, and what the preview draws of it and says.
#[cfg(test)]
pub struct RefusedClose {
    /// The chain, every step of which replays.
    pub steps: Vec<Step<f64>>,
    /// The drawn loop's own vertices.
    pub vertices: Vec<[f64; 2]>,
    /// Whether the author's own last leg closes the drawn loop.
    pub closes: bool,
    /// The kind of the close's refusal the chain says; `None` where it
    /// says the end-of-program refusal, its last leg being the close in
    /// all but spelling.
    pub said: Option<PathErrorKind>,
}

/// **Every shape of close refused on its geometry**, from `(0, 0)`: a
/// last leg onto the start point, whose close would have no length,
/// after an entry that opens with `at` (a triangle and a square) and
/// after one that opens with a direction; and a pending `fillet` and
/// `arc_fillet` whose arrival the close would bind, finding no corner,
/// drawn as the legs before them.
#[cfg(test)]
pub fn geometry_refused_closes() -> Vec<RefusedClose> {
    let legs_then = |tail: Step<f64>| {
        let mut steps = two_legs(0.0, 0.0);
        steps.extend([tail, Step::At(Point2::new(-0.005, 0.02))]);
        steps
    };
    let two = || vec![[0.0, 0.0], [0.01, 0.0], [0.01, 0.01]];
    vec![
        RefusedClose {
            steps: [two_legs(0.0, 0.0), vec![line_to(0.0, 0.0)]].concat(),
            vertices: two(),
            closes: true,
            said: None,
        },
        RefusedClose {
            steps: square_onto_the_start(),
            vertices: square_vertices(),
            closes: true,
            said: None,
        },
        RefusedClose {
            steps: vec![
                Step::Angle(0.0),
                Step::At(Point2::new(0.0, 0.0)),
                Step::Line(0.01),
                line_to(0.01, 0.01),
                line_to(0.0, 0.01),
                line_to(0.0, 0.0),
            ],
            vertices: square_vertices(),
            closes: true,
            said: None,
        },
        RefusedClose {
            steps: legs_then(Step::Fillet { radius: 0.002 }),
            vertices: two(),
            closes: false,
            said: Some(PathErrorKind::NoCornerOfPair),
        },
        RefusedClose {
            steps: legs_then(crate::sketch::fresh_step_at(
                Verb::ArcFillet,
                Some(TipState::DirectedPoint),
            )),
            vertices: two(),
            closes: false,
            said: Some(PathErrorKind::NoCornerForFillet),
        },
    ]
}

/// **Chains whose `line_to Start` is refused as tangent and whose
/// close the lattice spells another way**, from `(0, 0)`, each with
/// the vertex a refused step after it is crossed at:
///
/// - a close that would run straight on from the last leg
///   (`continue_to Start`): back along a leg to `(0.01, 0)`, and
///   half way back toward the start from [`two_legs`];
/// - a close that would arrive continuing the entry's first side
///   (`line_to` the start declared tangent): [`two_legs`], then a leg
///   to `(-0.01, 0)`;
/// - a close that would do both (`continue_to` the start declared
///   tangent): round a rectangle back to `(-0.01, 0)` on the line of
///   the first side, heading along it.
#[cfg(test)]
pub fn tangent_closes() -> Vec<(Vec<Step<f64>>, [f64; 2])> {
    let from_legs = |x, y| {
        let mut steps = two_legs(0.0, 0.0);
        steps.push(line_to(x, y));
        (steps, [x, y])
    };
    vec![
        (
            vec![
                Step::At(Point2::new(0.0, 0.0)),
                line_to(0.01, 0.01),
                line_to(0.02, 0.0),
                line_to(0.01, 0.0),
            ],
            [0.01, 0.0],
        ),
        from_legs(0.005, 0.005),
        from_legs(-0.01, 0.0),
        (
            vec![
                Step::At(Point2::new(0.0, 0.0)),
                line_to(0.01, 0.0),
                line_to(0.01, 0.01),
                line_to(-0.02, 0.01),
                line_to(-0.02, 0.0),
                line_to(-0.01, 0.0),
            ],
            [-0.01, 0.0],
        ),
    ]
}

/// **A fused entry and a chain after it**: `arc_fillet` about
/// `(0, -1)` from the start `(0, 0)`, its line arrival anchored at
/// `(0.01, 0.003)` by an `at` — which binds no start — then legs up and
/// across, and `last`.
#[cfg(test)]
pub fn fused_entry_then(last: Step<f64>) -> Vec<Step<f64>> {
    use pncad::profile::{ArcData, ArcSweep};
    vec![
        Step::ArcFillet {
            spec: ArcData::Center {
                c: Point2::new(0.0, -1.0),
                winding: ArcSweep::Cw,
                target: Target::Point(Point2::new(0.0, 0.0)),
            },
            radius: 0.001,
        },
        Step::At(Point2::new(0.01, 0.003)),
        line_to(0.01, 0.01),
        line_to(0.005, 0.01),
        last,
    ]
}

/// **Every unclosable tip state** — one an unfinished chain can end on
/// that no `line_to` leaves, so the provisional close is ill-typed
/// there — read off the lattice table over the kernel's census of
/// states (`profile::test_support`). `Closed` refuses `line_to` too,
/// and is finished.
#[cfg(test)]
pub fn unclosable_tips() -> Vec<TipState> {
    ::profile::test_support::every_state()
        .into_iter()
        .filter(|&state| {
            state != TipState::Closed
                && crate::sketch::admits_at(Some(state), Verb::LineTo).is_err()
        })
        .collect()
}

/// **`held` with `values` written over it**: each
/// value of `held` on a freedom `values` sets takes that number, keeping
/// its id; a freedom `values` sets that `held` holds no value on is
/// appended, for the door to refuse as the bundle's own
/// ([`pncad::document::MateFault::Overconstrained`]).
fn valued(
    held: Vec<pncad::document::Constraint<Formula>>,
    values: Vec<(pncad::document::Coord, Formula)>,
) -> Vec<pncad::document::Constraint<Formula>> {
    use pncad::document::{Constraint, ConstraintId, Value};
    let mut values = values;
    let mut bundle: Vec<Constraint<Formula>> = held
        .into_iter()
        .map(|c| {
            let set = c
                .value()
                .and_then(|v| values.iter().position(|(coord, _)| *coord == v.coord));
            match set {
                Some(at) => {
                    let (coord, by) = values.remove(at);
                    Constraint::of_value(c.id, Value { coord, by })
                }
                None => c,
            }
        })
        .collect();
    bundle.extend((1..).zip(values).map(|(i, (coord, by))| {
        Constraint::of_value(ConstraintId::placeholder(i), Value { coord, by })
    }));
    bundle
}
