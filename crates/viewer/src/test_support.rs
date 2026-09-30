//! **The fixture doors a test in this crate authors a document with** —
//! literals of each dimension, the world xy frame, an axis-aligned
//! rectangle, one edit or insert through the document's own `apply`, and
//! the display tolerances the suites index at.
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
    BooleanValue, CancelToken, Datum, Dimension, Doc, DocEdit, DocParam, EditError, EvalOptions,
    Evaluation, Expr, LoopProgram, Node, ParamName, ProfileProgram, RecipeNodeId, RefusingReach,
    ValuePayload, apply, evaluate,
};
use pncad::geom_core::{Point2, Tol};
use pncad::prelude::{CapEnd, EntityKind, RoleSeg, StableName};
use pncad::profile::{ArcData, ArcSide, Step, Target, TipState, Verb};

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
pub fn len_mm(metres: f64) -> Expr {
    Expr::literal_with_unit(metres, Dimension::Length, pncad::prelude::MM.def())
        .expect("a finite length")
}

/// Three length literals — a datum origin, a translation.
pub fn len3(v: [f64; 3]) -> [Expr; 3] {
    [len(v[0]), len(v[1]), len(v[2])]
}

/// Three dimensionless literals — a normal, a direction, an axis.
pub fn scl3(v: [f64; 3]) -> [Expr; 3] {
    [scl(v[0]), scl(v[1]), scl(v[2])]
}

/// Two dimensionless literals — a direction in a sketch frame.
pub fn scl2(v: [f64; 2]) -> [Expr; 2] {
    [scl(v[0]), scl(v[1])]
}

// --- the document's own edit door -----------------------------------
//
// Authored through `apply`, in the order a user would: a fixture that
// reached past it would be testing a document the edit vocabulary
// cannot produce. The fixtures are part-less — no mate, no cluster — so
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
    let applied = apply(doc, &edit, tol, &RefusingReach)?;
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
    node: Node<ProfileProgram>,
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
    node: Node<ProfileProgram>,
    tol: Tol,
) -> Result<(Doc<ProfileProgram>, RecipeNodeId), EditError> {
    let (doc, minted) = try_edited(doc, DocEdit::InsertNode { node }, tol)?;
    Ok((doc, minted.expect("an insert mints an id")))
}

// --- the nodes a fixture sketches with ------------------------------

/// A sketch frame node, and the world xy frame these fixtures sketch
/// on — `editor_core::test_support`'s, as the literals above.
pub use editor_core::test_support::{frame, xy_frame};

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
pub fn rectangle_loop(origin: [f64; 2], w: f64, h: f64) -> LoopProgram {
    assert!(
        w > 0.0 && h > 0.0,
        "a rectangle has positive sides: {w} x {h}"
    );
    let [x0, y0] = origin;
    LoopProgram::polygon([(x0, y0), (x0 + w, y0), (x0 + w, y0 + h), (x0, y0 + h)])
        .expect("finite corners")
}

/// An axis-aligned rectangular profile node's payload on `plane`:
/// [`rectangle_loop`] drawn on it. `square` is this with two equal
/// sides at the plane origin, and a fixture whose block sits elsewhere
/// moves `origin`.
pub fn rectangle(plane: RecipeNodeId, origin: [f64; 2], w: f64, h: f64) -> Node<ProfileProgram> {
    Node::Profile(ProfileProgram {
        plane,
        loops: vec![rectangle_loop(origin, w, h)],
        ids: Vec::new(),
    })
}

/// A square profile node's payload on `plane`, `side` metres on a side,
/// at the plane origin.
pub fn square(plane: RecipeNodeId, side: f64) -> Node<ProfileProgram> {
    rectangle(plane, [0.0, 0.0], side, side)
}

// --- documents built from the doors above ---------------------------

/// **A document holding one declared parameter and nothing else** —
/// the fixture both panel suites build their parameter rows on.
///
/// `label` is the document's derived name, so two fixtures in one
/// binary cannot share an identity. No oracle: it is the spelling of
/// `Doc::empty_derived` plus one `SetDocParam`, and what each row
/// asserts is about the `value` it handed in.
pub fn declared(label: &str, name: &ParamName, value: DocParam, tol: Tol) -> Doc<ProfileProgram> {
    let doc: Doc<ProfileProgram> = Doc::empty_derived(label, tol);
    edited(
        &doc,
        DocEdit::SetDocParam {
            name: name.clone(),
            value,
        },
        tol,
    )
    .0
}

/// The `&mut` spelling of `inserted`: insert a node in place and
/// answer the minted id, for a fixture that threads one document
/// through a sequence of edits rather than rebinding at each one.
/// Same call and same refusal behaviour — only the caller differs.
pub fn insert_into(
    doc: &mut Doc<ProfileProgram>,
    node: Node<ProfileProgram>,
    tol: Tol,
) -> RecipeNodeId {
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

/// **A frame and a square drawn on it**, answering the document and the
/// PROFILE's id — two nodes where a fixture used to insert one, because
/// a profile names the plane it is drawn on.
pub fn framed_square(
    doc: &Doc<ProfileProgram>,
    side: f64,
    tol: Tol,
) -> (Doc<ProfileProgram>, RecipeNodeId) {
    let (doc, plane) = inserted(doc, xy_frame(), tol);
    inserted(&doc, square(plane, side), tol)
}

/// The boss-on-a-face scene's block: 40 × 20 × 10 mm, centred on the
/// world xy frame and extruded up ([`boss_on_block`]).
pub const BOSS_BLOCK: [f64; 3] = [0.04, 0.02, 0.01];
/// The scene's boss: a disc of this radius on the block's top cap…
pub const BOSS_RADIUS: f64 = 0.005;
/// …extruded this far up.
pub const BOSS_HEIGHT: f64 = 0.004;

/// **A block and a boss drawn on its top cap**, answering the document,
/// the block and the boss — FLUSH with each other at that cap by
/// construction, which is the whole point of the scene: their union
/// refuses until the contact is declared.
///
/// The boss's frame is read off the block's top cap
/// (`Datum::FaceFrame`, zero spin), the node the add-datum form mints
/// from a face pick. `tests/creation_ops.rs`'s boss row authors the same
/// scene through the op vocabulary, because the gesture is what that
/// row is about; this is the scene alone. Its union's volume is
/// [`boss_on_block_union_volume`].
pub fn boss_on_block(label: &str, tol: Tol) -> (Doc<ProfileProgram>, RecipeNodeId, RecipeNodeId) {
    let [width, height, depth] = BOSS_BLOCK;
    let (doc, plane) = inserted(&Doc::empty_derived(label, tol), xy_frame(), tol);
    let (doc, section) = inserted(
        &doc,
        rectangle(plane, [-width / 2.0, -height / 2.0], width, height),
        tol,
    );
    let (doc, block) = inserted(
        &doc,
        Node::Extrude {
            profile: section,
            distance: len(depth),
        },
        tol,
    );
    let (doc, frame) = inserted(
        &doc,
        Node::Datum(Datum::FaceFrame {
            at: block,
            face: StableName {
                kind: EntityKind::Face,
                node: block,
                path: vec![RoleSeg::Cap(CapEnd::End)],
            },
            spin: ang(0.0),
        }),
        tol,
    );
    let (doc, disc) = inserted(
        &doc,
        Node::Profile(ProfileProgram {
            plane: frame,
            loops: vec![LoopProgram::circle(0.0, 0.0, BOSS_RADIUS).expect("a finite circle")],
            ids: Vec::new(),
        }),
        tol,
    );
    let (doc, boss) = inserted(
        &doc,
        Node::Extrude {
            profile: disc,
            distance: len(BOSS_HEIGHT),
        },
        tol,
    );
    (doc, block, boss)
}

/// The closed-form volume of [`boss_on_block`]'s union: the block's
/// box and the boss's cylinder, which meet only at the cap.
pub fn boss_on_block_union_volume() -> f64 {
    let [width, height, depth] = BOSS_BLOCK;
    width * height * depth + core::f64::consts::PI * BOSS_RADIUS * BOSS_RADIUS * BOSS_HEIGHT
}

/// **`doc` with `node` inserted, evaluated from scratch** — what the
/// kernel answers for a node, for a row comparing a session door's
/// verdict with the kernel's own. Answers the evaluation and the id.
pub fn evaluated_insert(
    doc: &Doc<ProfileProgram>,
    node: Node<ProfileProgram>,
    tol: Tol,
) -> (Evaluation<f64>, RecipeNodeId) {
    let (doc, id) = inserted(doc, node, tol);
    let eval = evaluate(
        &doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        tol,
    );
    (eval, id)
}

/// **The volume of `node`'s single body in `eval`** — an extrude's, a
/// blend's or a boolean's. A node with no value panics with its own
/// recorded error, not just the absence of a value.
pub fn evaluated_volume(eval: &Evaluation<f64>, node: RecipeNodeId, tol: Tol) -> f64 {
    let value = eval
        .value(node)
        .unwrap_or_else(|| panic!("the node evaluated: {:?}", eval.node_error(node)));
    let body = match &value.payload {
        ValuePayload::Body(body) | ValuePayload::Boolean(BooleanValue::Body { body, .. }) => body,
        other => panic!("expected a body, got {other:?}"),
    };
    pncad::topo::mass_properties(body, tol)
        .expect("mass properties")
        .volume
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

/// **Every tip state no `line_to` leaves**, read off the lattice table
/// (`sketch::admits_at` over each state some verb has a row at), so a
/// state the table gains is in the census without an edit here.
pub fn tips_no_line_to_leaves() -> Vec<TipState> {
    let mut states: Vec<TipState> = Vec::new();
    for &verb in Verb::ALL {
        for &state in verb.states() {
            if !states.contains(&state)
                && crate::sketch::admits_at(Some(state), Verb::LineTo).is_err()
            {
                states.push(state);
            }
        }
    }
    states
}

/// **[`two_legs`] from the origin, then steps that leave its tip in
/// `state`**, one of [`tips_no_line_to_leaves`] — exhaustive, so a
/// state the lattice gains has to be given a way in (or ruled out)
/// before this compiles. `None` for the entry, which no step precedes,
/// and for the states a `line_to` leaves.
pub fn two_legs_awaiting(state: TipState) -> Option<Vec<Step<f64>>> {
    use core::f64::consts::PI;
    let fillet = Step::Fillet { radius: 0.002 };
    let arrival = |spec| Step::FilletArc {
        radius: 0.002,
        spec,
    };
    let radius = ArcData::Radius {
        r: 0.02,
        side: ArcSide::Left,
    };
    let via = |target| ArcData::Via {
        q: Point2::new(0.005, 0.02),
        target,
    };
    let tail = match state {
        TipState::Entry | TipState::PlainPoint | TipState::DirectedPoint | TipState::Closed => {
            return None;
        }
        TipState::Open => vec![fillet],
        TipState::Angle => vec![fillet, Step::Angle(PI)],
        TipState::DirectedPlain => {
            vec![fillet, Step::At(Point2::new(-0.005, 0.02)), Step::Angle(PI)]
        }
        TipState::DirectedIncoming => vec![Step::Turn(0.5)],
        TipState::RadiusArrival => vec![arrival(radius)],
        TipState::RadiusArrivalAt => vec![arrival(radius), Step::At(Point2::new(0.0, 0.02))],
        TipState::RadiusArrivalDir => vec![arrival(radius), Step::Angle(PI)],
        TipState::ViaArrival => vec![arrival(via(Target::Point(Point2::new(0.0, 0.015))))],
        TipState::ViaArrivalStart => vec![arrival(via(Target::Start))],
    };
    let mut steps = two_legs(0.0, 0.0);
    steps.extend(tail);
    Some(steps)
}
