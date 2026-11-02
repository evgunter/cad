//! Fixtures shared by this crate's suites, **derived from the scene
//! they test** rather than restated beside it.
//!
//! Why this file exists: the plate's dimensions were hand-copied into
//! three suites, so changing `scene::plate_with_hole` would have left
//! two of them testing a box the scene no longer has — green, and
//! measuring nothing. `viewer::scene` exports the plate's identity
//! (`PLATE_EXTENT`, `PLATE_HOLE_RADIUS`) and the plate fixtures here
//! are functions of it, so they cannot drift from the subject.
//!
//! **Whether a helper here carries an oracle is a question about that
//! helper, not about what kind of file reads it**, and it is asked one
//! helper at a time: a blanket sentence over this module is false for
//! whatever is added to it next. Some of what lives here plainly does
//! carry one — the plate helpers are functions of `PLATE_EXTENT`, so a
//! row reading one measures the scene against its own constants;
//! `framed` IS a call to `Camera::framing`; `near` fixes the tolerance
//! a comparison passes at. Others are spelling and nothing more. Three
//! whose signatures do not show it say so in their own docs instead —
//! `near`'s chosen bound, `body_volume`'s choice of WHICH document it
//! reads, and `gallery_ring_at`'s note of the row that checks its work.
//! A suite that keeps its own code instead of sharing says why in its
//! own header.

#![allow(dead_code)] // one instance per binary; no single consumer uses all of it
#![allow(unreachable_pub)]
// why: root Cargo.toml, the `unreachable_pub` stanza
// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

/// The GUI-4 assembly fixture (a gallery-shaped workspace on disk).
pub mod asm;

/// The corpus pick suites' walk: their landings, their single-level
/// reference and their two aims.
pub mod corpus_pick;

/// Where a drawn copy stands, and what it draws.
pub mod drawn;

use bvh::Aabb;
use pncad::document::{SolvedPoses, solve_document};
use pncad::geom_core::Point3;
use viewer::camera::Camera;

/// **The placements of `doc` solved under `session`'s own seam** —
/// each mated part resolved through the session's resolver the way its
/// landed evaluation resolved it (`DocSession::eval_options`), through
/// the kernel's public door. `doc` is the session's landed document, or
/// one derived from it that resolves against the same directory.
pub fn solve(session: &DocSession, doc: &Doc<ProfileProgram>, tol: Tol) -> SolvedPoses {
    solve_document(doc, &session.eval_options(), tol)
}
use viewer::scene::{PLATE_EXTENT, PLATE_HOLE_RADIUS};

/// The spike plate's bounding box, from the scene's own dimensions.
pub fn plate_bounds() -> Aabb {
    let [width, depth, thickness] = PLATE_EXTENT;
    Aabb {
        min_x: 0.0,
        min_y: 0.0,
        min_z: 0.0,
        max_x: width,
        max_y: depth,
        max_z: thickness,
    }
}

/// The plate's nominal solid volume: the block, less the through hole.
pub fn plate_volume() -> f64 {
    let [width, depth, thickness] = PLATE_EXTENT;
    width * depth * thickness
        - std::f64::consts::PI * PLATE_HOLE_RADIUS * PLATE_HOLE_RADIUS * thickness
}

/// The default framing on the plate at `aspect`.
///
/// This IS a call to `Camera::framing`, so a row whose subject is that
/// door cannot take its camera from here and still be checking it.
pub fn framed(aspect: f64) -> Camera {
    Camera::framing(Some(plate_bounds()), aspect).expect("the plate frames")
}

/// The eight corners of a box.
pub fn corners(b: &Aabb) -> Vec<Point3<f64>> {
    let mut out = Vec::new();
    for x in [b.min_x, b.max_x] {
        for y in [b.min_y, b.max_y] {
            for z in [b.min_z, b.max_z] {
                out.push(Point3::new(x, y, z));
            }
        }
    }
    out
}

// --- document fixtures for the panel suites ------------------------
//
// Authored through the ordinary document doors, in the order a user
// would: variables before the expressions that read them, nodes
// before the nodes that consume them. A fixture that reached past
// `apply` would be testing a document the edit vocabulary cannot
// produce.

use pncad::document::{
    Dimension, Doc, DocEdit, Formula, FreeVar, LoopProgram, Node, ProfileProgram, RecipeNodeId,
    VarName,
};
use pncad::geom_core::Tol;
use viewer::props::Notation;
use viewer::sketch::ProfileShape;

// The literal, edit, frame, rectangle and δ doors are `viewer`'s own
// `test_support` (its `test-support` feature, on for these suites through
// the crate's self dev-dependency), so the crate's unit-test modules and
// these suites read ONE definition. A suite says `common::len` as before.
pub use viewer::test_support::*;

/// **The witnessed band a placement axis is decided under** — what
/// `Frame::rotate_then_translate` asks the direction door with. Rows
/// whose axis is a literal pass this and unwrap; a row whose SUBJECT
/// is the axis decision reads the refusal instead.
pub fn band() -> pncad::geom_core::Band {
    pncad::geom_core::Band::linear(Tol::witness()).expect("the witnessed band")
}

/// One form template lowered CANONICALLY — what a suite means when it
/// authors a shape without a word about notation.
///
/// The suites that DO care which unit a literal remembers say so by
/// naming the notation (`sketch::loop_program` with one of its own),
/// which is the point of the units riding the lowering rather than
/// the op.
pub fn shape(template: &ProfileShape) -> LoopProgram<Formula> {
    viewer::sketch::loop_program(template, Notation::CANONICAL).expect("a finite template")
}

/// The name of the parametric fixture's driving variable.
pub fn thickness_param() -> VarName {
    VarName::from_static("thickness")
}

/// The variable `doc` names `name` — the id every variable-keyed op
/// and row addresses it by.
pub fn var_of(doc: &Doc<ProfileProgram>, name: &str) -> pncad::document::VarId {
    doc.var_named(name)
        .unwrap_or_else(|| panic!("the document names a variable {name}"))
}

/// The parametric fixture's driving variable, by its id in `doc`.
pub fn thickness_var(doc: &Doc<ProfileProgram>) -> pncad::document::VarId {
    var_of(doc, thickness_param().as_str())
}

/// A document whose extrude distance is DRIVEN by a document
/// variable — the expression-driven-dimension fixture.
///
/// The extrude is placed. Answers the document, the profile node and
/// the extrude node.
pub fn parametric_plate(tol: Tol) -> (Doc<ProfileProgram>, RecipeNodeId, RecipeNodeId) {
    let doc: Doc<ProfileProgram> = Doc::empty_derived("gui3-parametric", tol);
    let (doc, _) = edited(
        &doc,
        DocEdit::DeclareVar {
            name: thickness_param(),
            def: pncad::document::VarDecl::Free(FreeVar::continuous(Dimension::Length, 0.008)),
        },
        tol,
    );
    let (doc, profile) = square_profile(&doc, 0.04, tol);
    let (doc, extrude) = inserted(
        &doc,
        Node::Extrude {
            profile: profile.into(),
            // `thickness / 2` — a composed expression over a
            // variable, which is the shape the refusal affordance
            // exists for.
            distance: Formula::div(
                Formula::named(thickness_param(), Dimension::Length),
                scl(2.0),
            )
            .expect("length / scalar is a length"),
        },
        tol,
    );
    let (doc, _) = placed(&doc, extrude, tol);
    (doc, profile, extrude)
}

/// A document that FAILS at one node and poisons its descendant.
///
/// The failure is a division by a zero literal in the extrude's
/// distance — an expression that is well-dimensioned at the edit door
/// and non-finite at evaluation, which is exactly the shape GQ2's
/// per-node result exists to report. A placement of it is pinned in the
/// world. Answers the document, the failing extrude and the poisoned
/// placement.
pub fn broken_document(tol: Tol) -> (Doc<ProfileProgram>, RecipeNodeId, RecipeNodeId) {
    let doc: Doc<ProfileProgram> = Doc::empty_derived("gui3-broken", tol);
    let (doc, profile) = square_profile(&doc, 0.04, tol);
    let (doc, extrude) = inserted(
        &doc,
        Node::Extrude {
            profile: profile.into(),
            distance: Formula::div(len(0.008), scl(0.0)).expect("length / scalar is a length"),
        },
        tol,
    );
    let (doc, moved) = inserted(
        &doc,
        Node::place(
            pncad::document::BodyRead::plain(pncad::document::Operand::from(extrude)),
            pncad::document::Operand::Var(pncad::document::VarId::WORLD),
            pncad::document::stand().bundle(),
        ),
        tol,
    );
    let at = pncad::document::Frame {
        translation: [0.01, 0.0, 0.0],
        ..pncad::document::Frame::IDENTITY
    };
    let doc = pin(
        &doc,
        moved,
        &at,
        &pncad::document::EvalOptions::default(),
        tol,
    );
    (crate::fixture::assemble_world_copies(doc), extrude, moved)
}

/// **`doc` with `body` placed in the world at `at`** — a placement of
/// it, pinned there over its geometry ([`pin`]) — answering the
/// document and the placement: a copy of `body` standing translated by
/// `at` from where its body stands.
pub fn placed_at(
    doc: &Doc<ProfileProgram>,
    body: RecipeNodeId,
    at: [f64; 3],
    tol: Tol,
) -> (Doc<ProfileProgram>, RecipeNodeId) {
    let (doc, placement) = inserted(
        doc,
        Node::place(
            pncad::document::BodyRead::plain(pncad::document::Operand::from(body)),
            pncad::document::Operand::Var(pncad::document::VarId::WORLD),
            pncad::document::stand().bundle(),
        ),
        tol,
    );
    let pose = pncad::document::Frame {
        translation: at,
        ..pncad::document::Frame::IDENTITY
    };
    let doc = pin(
        &doc,
        placement,
        &pose,
        &pncad::document::EvalOptions::default(),
        tol,
    );
    (crate::fixture::assemble_world_copies(doc), placement)
}

/// The gallery ring document as `demo-tour gallery` saved it (the
/// exporter round trip with the dialog and the window taken out).
///
/// It is version-stamped in its name, as `pncad`'s own fixture is: a
/// schema break makes this file unreadable, and the fix is to
/// regenerate it from `demo-tour gallery` and rename, never to teach
/// the loader about an old shape.
///
/// Regenerated for the identity bundle (FORK-REST): the tour stands its
/// ring by `stand`, which writes no mate and the constant `0` on the
/// three slides and three spins, where it wrote an `Axis` mate to the
/// world's x axis turned by the revolve's heading and slid by its axis
/// numbers. The ring's placement holds six values and no mate, so its
/// ids from the placement on moved; the profile and the revolve did
/// not.
///
/// Regenerated for values on the target space's axes (D′2): a spin
/// value names its axis (`spin`, where it was `spin_about`), and the
/// tour's variables now record the display units it authors them in.
/// The placement's id, its constraints' and the copy's moved with the
/// content key; no number moved.
pub const GALLERY_RING: &str = include_str!("../gallery_ring.pncad");

/// **ε is a run parameter, and a saved document records the one it was
/// decided at** — "one process, one ε", which `load` enforces by
/// refusing a file whose recorded ε is not the process's
/// (`PersistError::ToleranceConflict`). The CI matrix sweeps ε, so a
/// committed document fixture is loadable at exactly one of its
/// points and refuses at the others.
///
/// So the fixture is re-stamped with THIS run's ε before it is opened.
/// The new ε line comes from `save` itself, via a throwaway document
/// at the process tolerance: spelling a float the way the serializer
/// spells it is the serializer's job, not this file's.
///
/// **What this function does NOT do is check its own work.** The real
/// claim (a re-stamped fixture is byte-for-byte what the exporter
/// writes at this ε) is measured by `doc_io`'s
/// `the_restamped_fixture_is_what_the_serializer_writes_at_this_eps`,
/// which puts the bytes back through `save` rather than through this
/// function's own arithmetic.
pub fn gallery_ring_at(tol: Tol) -> String {
    let probe: Doc<ProfileProgram> = Doc::empty_derived("gui3-epsilon-probe", tol);
    let probe_text = pncad::document::save(&probe, &[], tol).expect("an empty document saves");
    let is_epsilon = |line: &str| line.trim_start().starts_with("\"epsilon\":");
    let wanted = probe_text
        .lines()
        .find(|line| is_epsilon(line))
        .expect("a saved document records its ε");
    assert_eq!(
        GALLERY_RING.lines().filter(|l| is_epsilon(l)).count(),
        1,
        "the fixture must carry exactly one ε line"
    );
    let mut text: String = GALLERY_RING
        .lines()
        .map(|line| if is_epsilon(line) { wanted } else { line })
        .collect::<Vec<&str>>()
        .join("\n");
    text.push('\n');
    text
}

// --- session helpers for the op-vocabulary suites -------------------
//
// One home for the helpers every `DocSession`-driving suite wants:
// each is a statement about the session contract (one op, one
// committed insert; a node's value is one body), not about any one
// suite's geometry, so a per-suite copy could only drift.

use viewer::session::{DocSession, FaceSelection, SessionOp};

/// **Perform one authoring op and answer the node it made**
/// (`OpOutcome::made`), holding the session's contract on the rest of
/// the action: every other edit inserts what the node is made of or
/// read through (a `new_space()`, the operands a combine places against
/// its first operand's root, a mirror's image, a `⊔` over a family),
/// takes over what read its input (a source, a `⊔` operand, the product
/// slot) or withdraws a copy, writes a bundle, or declares the index a
/// pattern's placement reads.
///
/// This is the op vocabulary's door, and it holds the session's
/// contract. [`inserted`] and [`insert_into`] are the document's door
/// instead: they call `apply` with no session, and check only that the
/// edit applies and mints an id — there is no op, so no outcome to hold
/// to that contract. A fixture that means to exercise the chrome's ops
/// reaches for this one.
pub fn session_insert(session: &mut DocSession, op: SessionOp) -> RecipeNodeId {
    let outcome = session.perform(op);
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    for edit in &outcome.committed {
        assert!(
            matches!(
                edit,
                DocEdit::InsertNode { .. }
                    | DocEdit::SetParam {
                        slot: pncad::document::SlotId::Operand(
                            pncad::document::OperandSlot::Source
                                | pncad::document::OperandSlot::Space
                                | pncad::document::OperandSlot::Member(_)
                                | pncad::document::OperandSlot::Members
                        ),
                        ..
                    }
                    | DocEdit::SetProduct { .. }
                    | DocEdit::DeleteNode { .. }
                    | DocEdit::DeclareVar { .. }
                    | DocEdit::SetConstraints { .. }
                    | DocEdit::AddConstraint { .. }
                    | DocEdit::RemoveConstraint { .. }
                    | DocEdit::RemoveConstraints { .. }
            ),
            "every other edit makes, reads or takes over: {edit:?}"
        );
    }
    outcome.made.expect("an authoring op made a node")
}

/// **Add an instance of the part `id` through the session**, pump the
/// seam, and answer the instance node — [`session_insert`] over
/// `SessionOp::AddInstance`, the step every assembly-authoring suite
/// takes before it has anything to mate or pick.
pub fn instance_in(session: &mut DocSession, id: pncad::document::DocumentId) -> RecipeNodeId {
    let node = session_insert(session, SessionOp::AddInstance { id });
    session.pump();
    node
}

/// **Every mate `doc` holds between two copies**: each mate, by its
/// address, in placement order and bundle order within one — the
/// recipe's own mates, without the values.
pub fn mates_in(doc: &Doc<ProfileProgram>) -> Vec<pncad::document::MateRef> {
    doc.placements()
        .into_iter()
        .flat_map(|placement| {
            crate::fixture::bundle(doc, placement)
                .into_iter()
                .filter(|constraint| constraint.mate().is_some())
                .map(move |constraint| pncad::document::MateRef::new(placement, constraint.id))
        })
        .collect()
}

/// **A mate and its values written into `doc`** through the edit door:
/// `placement`'s bundle written whole as `mate` and `values` naming it
/// (`viewer::test_support::mated`) — what the mate tool's op commits,
/// for a row that authors its document before a session opens it.
/// Answers the mate.
pub fn mate_into(
    doc: &mut Doc<ProfileProgram>,
    placement: RecipeNodeId,
    mate: pncad::document::Mate<Formula>,
    values: Vec<(pncad::document::Coord, Formula)>,
    tol: Tol,
) -> pncad::document::MateRef {
    let (next, mate) = viewer::test_support::mated(doc, placement, mate, values, tol);
    *doc = next;
    mate
}

/// **Commit a mate through the session**, pump, and answer the mate it
/// added — the op's last committed edit checked to BE an
/// `AddConstraint` (or the bundle written whole over it with the
/// panel's values), and the mate read back by its address, because
/// that address is what the solve keys a fault by (its placement's).
///
/// A row that authors two mates into ONE evaluation does not pump
/// between them, and so takes [`add_mate`] instead.
///
/// # Panics
///
/// If the op refuses, or commits anything but one new mate.
pub fn commit_mate(session: &mut DocSession, op: SessionOp) -> pncad::document::MateRef {
    let mate = add_mate(session, op);
    session.pump();
    mate
}

/// [`commit_mate`] without the pump.
pub fn add_mate(session: &mut DocSession, op: SessionOp) -> pncad::document::MateRef {
    // A placement's editor writes its bundle whole; a mate is added to it.
    let (SessionOp::AddMate { placement, .. } | SessionOp::SetBundle { placement, .. }) = &op
    else {
        panic!("a mate is added to a bundle by `AddMate` or the editor's `SetBundle`: {op:?}")
    };
    let placement = *placement;
    let mates = |doc: &Doc<ProfileProgram>| -> Vec<_> {
        crate::fixture::bundle(doc, placement)
            .into_iter()
            .filter(|constraint| constraint.mate().is_some())
            .map(|constraint| constraint.id)
            .collect()
    };
    let before = mates(session.committed_doc());
    let outcome = session.perform(op);
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    // The mate added, its bundle then written whole with the panel's
    // values where it has any, after a re-target where the mate reads
    // another space.
    assert!(
        matches!(
            outcome.committed.last(),
            Some(DocEdit::AddConstraint { .. } | DocEdit::SetConstraints { .. })
        ),
        "the op writes one bundle: {:?}",
        outcome.committed
    );
    let added: Vec<_> = mates(session.committed_doc())
        .into_iter()
        .filter(|id| !before.contains(id))
        .collect();
    let [id] = added[..] else {
        panic!("the op wrote one mate on {placement:?}: {added:?}")
    };
    pncad::document::MateRef::new(placement, id)
}

/// **A rectangle profile through the session**: the chrome's rectangle
/// template, `width` by `height` and centred on the profile's own
/// origin, drawn by `SessionOp::AddProfile` — answering the profile.
pub fn rectangle_in(session: &mut DocSession, width: f64, height: f64) -> RecipeNodeId {
    session_insert(
        session,
        SessionOp::AddProfile {
            loops: vec![shape(&ProfileShape::Rectangle { width, height })],
        },
    )
}

/// **A box through the session**: [`rectangle_in`], then
/// `SessionOp::AddExtrude` by `depth` — answering the profile and the
/// extrude.
pub fn box_in(
    session: &mut DocSession,
    [width, height, depth]: [f64; 3],
) -> (RecipeNodeId, RecipeNodeId) {
    let profile = rectangle_in(session, width, height);
    let extrude = session_insert(
        session,
        SessionOp::AddExtrude {
            profile,
            distance: len(depth),
        },
    );
    (profile, extrude)
}

/// [`box_in`], answering the extrude — the body a row goes on to
/// combine, blend or measure.
pub fn xy_box_in(session: &mut DocSession, size: [f64; 3]) -> RecipeNodeId {
    box_in(session, size).1
}

/// **One node's row out of a tree render.**
///
/// Panics rather than answering `None`: every id these rows pass is
/// one the document holds, so a missing row is the failure, not a
/// case to handle.
pub fn row_of(rows: &[viewer::tree::TreeRow], id: RecipeNodeId) -> &viewer::tree::TreeRow {
    rows.iter()
        .find(|row| row.id == id)
        .unwrap_or_else(|| panic!("node {id:?} has a row"))
}

/// One node's row status out of a tree render ([`row_of`]).
pub fn status_of(rows: &[viewer::tree::TreeRow], id: RecipeNodeId) -> viewer::tree::RowStatus {
    row_of(rows, id).status.clone()
}

/// `got` and `want` agree to one part in 10⁹, relatively.
///
/// The 1e-9 is a chosen bound, not a derived one: the closed-form
/// volume rows in the suites that share this helper hold it with
/// margin, and it is kept tight so real drift cannot hide inside it.
pub fn near(got: f64, want: f64) -> bool {
    ((got - want) / want).abs() < 1e-9
}

/// [`evaluated_volume`] of `node` with the seam pumped.
///
/// The evaluation read is the SHOWN document's, so mid-gesture this
/// measures the scratch preview exactly as the viewport does.
pub fn body_volume(session: &mut DocSession, node: RecipeNodeId, tol: Tol) -> f64 {
    session.pump();
    evaluated_volume(
        session.evaluation().expect("the inline seam landed"),
        node,
        tol,
    )
}

/// The story-gallery door: the directory named by
/// `PNCAD_STORY_GALLERY`, when the invoker of the test run set one.
///
/// The contract: when the variable is set, each story suite saves its
/// finished document(s) into the named directory through the session's
/// own save door, so the screenshot recipe in
/// `docs/gui-shots/2026-09-01/README.md` can open them in the live app
/// — `PNCAD_STORY_GALLERY=<dir> cargo test -p viewer --test all
/// story_` is that README's production command. When unset, the suites
/// skip the save; no assertion depends on the variable either way.
pub fn story_gallery_dir() -> Option<std::path::PathBuf> {
    std::env::var_os("PNCAD_STORY_GALLERY").map(std::path::PathBuf::from)
}

/// A fresh directory under the OS temp root, named for the caller.
///
/// One home, and it stays one: a temp-directory name carries no oracle
/// — no row can assert anything about it — so there is nothing here
/// for a copy to derive independently, whoever wrote the suite.
pub fn tempdir(label: &str) -> std::path::PathBuf {
    let unique = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_nanos());
    let dir = std::env::temp_dir().join(format!("{label}-{unique}"));
    std::fs::create_dir_all(&dir).expect("the fixture directory is creatable");
    dir
}

// --- the pick seam: the index, the aimed rays, the displayed pick ----
//
// A suite picks against an index built from four values the session
// already holds — the landed document and evaluation, the generation
// that names the run, and the session's ε — plus a δ. The δ is a
// per-suite choice and stays an argument; the other four are not, so
// the call lives here.
//
// This is the PLAIN door, `PickIndex::build`. It is not a second
// spelling of `DocSession::index_inputs`, which packages the same four
// for the MEMOISED path (`PickCache` -> `evalseam::build_index` ->
// `PickIndex::build_with`) and cannot be handed to `build`: the two
// are what `index_memo` exists to compare, so a suite that reached the
// plain door through the memo's packaging would have nothing left to
// check.
//
// The rays are axis-aligned — vertical (`down_at`, `up_at`) or level
// (`along_x`, `along_y`) — and the pick below reads the session's
// display view, which is what makes it the viewport's pick.

use bvh::test_support::ray;
use pncad::select::Ray;
use viewer::pickindex::{PickIndex, PickIndexError, PictureKey};
use viewer::scene::DisplayTolerance;

/// The pick index for `session`'s landed evaluation at `delta`, or the
/// refusal — a failed or poisoned placement is an ordinary editing state,
/// and a suite whose subject is that refusal reads it here.
pub fn index_at(
    session: &DocSession,
    delta: DisplayTolerance,
) -> Result<PickIndex, PickIndexError> {
    let (doc, eval) = session
        .landed_pair()
        .expect("the inline seam lands its first evaluation");
    let generation = session
        .landed_generation()
        .expect("a landed evaluation has a generation");
    PickIndex::build(
        doc,
        eval,
        PictureKey::of(generation, delta, session.display().shown()),
        session.tol(),
    )
}

/// [`index_at`] for a fixture that indexes by construction: the
/// refusal is the failure.
pub fn index_of(session: &DocSession, delta: DisplayTolerance) -> PickIndex {
    index_at(session, delta).expect("the fixture indexes")
}

/// [`index_of`] at [`plate_delta`] — what a plate-scale suite wants.
pub fn plate_index(session: &DocSession) -> PickIndex {
    index_of(session, plate_delta())
}

/// [`index_of`] at [`corpus_delta`] — what a corpus suite wants.
pub fn corpus_index(session: &DocSession) -> PickIndex {
    index_of(session, corpus_delta())
}

/// A ray straight down through `(x, y)` from height `z` —
/// `editor_core::test_support`'s, re-exported rather than re-written, as
/// the mate heads below are: `editor-core`'s pick suites aim the same
/// ray.
pub use editor_core::test_support::down_from;

/// [`down_from`] at one metre up — above anything the plate- and
/// assembly-scale fixtures build. A suite whose fixture reaches higher,
/// or which wants the origin closer, passes its own height.
pub fn down_at(x: f64, y: f64) -> Ray {
    down_from(x, y, 1.0)
}

/// A ray straight up through `(x, y)` from one metre below — under
/// anything those fixtures build, for the underside faces a downward
/// ray never reaches.
pub fn up_at(x: f64, y: f64) -> Ray {
    ray([x, y, -1.0], [0.0, 0.0, 1.0])
}

/// **The face `ray` meets, picked the way the viewport picks it** —
/// through `index` against `session`'s landed evaluation and its
/// display view, so a hidden instance is not picked and a probed one is
/// picked where it is drawn. (`PickIndex::face_at` is the same pick
/// under no display view; the name says which one a row reads.)
///
/// # Panics
///
/// If the session has no landed evaluation, the pick refuses, or the
/// ray meets no face: a row aims its ray at a face it means to pick.
pub fn displayed_face_at(session: &DocSession, index: &PickIndex, ray: &Ray) -> FaceSelection {
    let (_, eval) = session.landed_pair().expect("landed");
    index
        .face_at_for(eval, ray, &session.display_view())
        .expect("the pick answers")
        .expect("the ray hits")
}

/// A level ray along x through `(y, z)`, travelling toward `sense`'s
/// sign (`1.0` or `-1.0`) from one metre back on the far side of
/// `x = 0` — for the walls a vertical ray never reaches, on the same
/// plate- and assembly-scale fixtures as [`down_at`].
///
/// # Panics
///
/// Unless `sense` is `1.0` or `-1.0`.
pub fn along_x(sense: f64, y: f64, z: f64) -> Ray {
    level([1.0, 0.0], sense, [-sense, y, z])
}

/// [`along_x`] one axis over: a level ray along y through `(x, z)`.
///
/// # Panics
///
/// Unless `sense` is `1.0` or `-1.0`.
pub fn along_y(sense: f64, x: f64, z: f64) -> Ray {
    level([0.0, 1.0], sense, [x, -sense, z])
}

/// The one body both level rays share: `axis` scaled by `sense`, from
/// `origin`.
fn level(axis: [f64; 2], sense: f64, origin: [f64; 3]) -> Ray {
    assert!(
        sense == 1.0 || sense == -1.0,
        "a level ray's sense is 1 or -1: {sense}"
    );
    ray(origin, [sense * axis[0], sense * axis[1], 0.0])
}

// The mate-head helpers are `crate::fixture`'s, re-exported rather than
// re-written: `tests/fixture/` is editor-core's tree, symlinked into
// this one and mounted by this binary's root, so a second body here
// would be a second definition of one fixture claim — and
// `pncad::document::SitedFace` IS `editor_core::SitedFace`, the façade
// re-exporting the kernel's type rather than wrapping it. A suite says
// `common::head` as before.
pub use crate::fixture::{head, head_at};

/// **The drawn copies**, in document order: the placements whose copy
/// the picture draws ([`viewer::world::drawn_nodes`]).
pub fn drawn_copies(doc: &Doc<ProfileProgram>) -> Vec<RecipeNodeId> {
    let placements = doc.placements();
    viewer::world::drawn_nodes(doc)
        .into_iter()
        .filter(|node| placements.contains(node))
        .collect()
}

/// **What the world places** (A10): for each drawn copy, in document
/// order, the node defining the body it places.
pub fn world(doc: &Doc<ProfileProgram>) -> Vec<RecipeNodeId> {
    drawn_copies(doc)
        .into_iter()
        .map(|placement| viewer::world::seat_of(doc, placement))
        .collect()
}

/// **The one world placement of `body`**: the node its drawn copy is
/// picked on (A10) — an instance itself, which is a placement. A new
/// body lands unplaced, so a row that wants a copy adds it to the
/// product ([`world_copy_in`]), or, building a document, places one by
/// hand ([`world_copy`]); a row that wants whatever the body is drawn
/// under asks [`drawn_node`].
///
/// # Panics
///
/// Unless exactly one placement places `body`.
pub fn copy_of(doc: &Doc<ProfileProgram>, body: RecipeNodeId) -> RecipeNodeId {
    // An instance is a placement: its copies are drawn on it.
    if matches!(doc.node(body), Some(Node::InstantiatePart { .. })) {
        return body;
    }
    let [copy] = viewer::world::placements_of(doc, body)[..] else {
        panic!("{body:?} is placed once")
    };
    copy
}

/// **`body`'s one world copy, placed by hand where nothing copies it**:
/// a `Place` into the world by the identity bundle, the product left as
/// it was, unless `doc` holds one already — a document a row builds,
/// not a person's path, which is the product door's ([`world_copy_in`]).
/// Answers the document and the copy.
pub fn world_copy(
    doc: &Doc<ProfileProgram>,
    body: RecipeNodeId,
    tol: Tol,
) -> (Doc<ProfileProgram>, RecipeNodeId) {
    match viewer::world::placements_of(doc, body)[..] {
        [] if !matches!(doc.node(body), Some(Node::InstantiatePart { .. })) => {
            inserted(doc, world_place(pncad::document::Operand::Node(body)), tol)
        }
        _ => (doc.clone(), copy_of(doc, body)),
    }
}

/// **A `Place` of `read` into the world by the identity bundle** — the
/// placement a person writes by hand, which no gesture writes for them.
pub fn world_place(read: pncad::document::Operand) -> pncad::document::AuthoredNode {
    Node::place(
        pncad::document::BodyRead::plain(read),
        pncad::document::Operand::Var(pncad::document::VarId::WORLD),
        pncad::document::stand().bundle(),
    )
}

/// **`body` added to the product in a session**, as a person adds it:
/// a landed session over the session's document, `body` taken through
/// the one door ([`SessionOp::AddToProduct`]) — which places it into
/// the world by the identity bundle and adds that copy, or moves a
/// placement nothing reads into the world — unless the world holds a
/// copy of it already. Answers the session and the world copy.
///
/// # Panics
///
/// If the door refuses.
pub fn world_copy_in(
    session: &DocSession,
    body: RecipeNodeId,
    tol: Tol,
) -> (DocSession, RecipeNodeId) {
    let mut session = inline_without_door(session.committed_doc().clone(), tol);
    session.pump();
    let doc = session.committed_doc();
    let placed = matches!(doc.node(body), Some(Node::InstantiatePart { .. }))
        || !viewer::world::placements_of(doc, body).is_empty();
    if !placed {
        let outcome = session.perform(SessionOp::AddToProduct {
            node: body,
            body: None,
        });
        assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
        session.pump();
    }
    let doc = session.committed_doc();
    // A placement nothing read is moved into the world, not copied.
    let copy = match viewer::world::world_copy(doc, body) {
        Some(_) => body,
        None => copy_of(doc, body),
    };
    (session, copy)
}

/// [`world_copy`] in a session: a landed session over the document
/// holding `body`'s world copy placed by hand, the product left as it
/// was, and the copy — for a row about a document holding that shape,
/// which the product door would not write (it moves a placement nothing
/// reads rather than copying it).
pub fn world_copy_by_hand_in(
    session: &DocSession,
    body: RecipeNodeId,
    tol: Tol,
) -> (DocSession, RecipeNodeId) {
    let (doc, copy) = world_copy(session.committed_doc(), body, tol);
    let mut session = inline_without_door(doc, tol);
    session.pump();
    (session, copy)
}

/// **The node `node`'s body is drawn under**: its one world placement
/// when the world places it once ([`copy_of`]), `node` itself
/// otherwise — so a row can aim at a profile or an unplaced node the way
/// it aims at a placed body.
///
/// # Panics
///
/// Where several placements copy `node`: the row has to say which.
pub fn drawn_node(doc: &Doc<ProfileProgram>, node: RecipeNodeId) -> RecipeNodeId {
    match viewer::world::placements_of(doc, node)[..] {
        [] => node,
        [copy] => copy,
        _ => panic!("{node:?} is copied more than once: say which copy"),
    }
}

/// **A door seam that answers nothing**: it drops every job, so a
/// landing's checks report reads the coincidence door pending for
/// good. For the rows that never read the coincidence door's findings
/// (some read the other residents', which do not pend), which would
/// otherwise pay the door's replay at every landing.
pub struct NoDoor;

impl viewer::evalseam::DoorService for NoDoor {
    fn submit(&mut self, _request: viewer::evalseam::DoorRequest) {}

    fn cancel(&mut self) {}

    fn poll(&mut self) -> Option<viewer::evalseam::DoorDone> {
        None
    }

    fn busy(&self) -> bool {
        false
    }
}

/// [`DocSession::inline`] with [`NoDoor`].
pub fn inline_without_door(
    doc: pncad::document::Doc<pncad::document::ProfileProgram>,
    tol: pncad::geom_core::Tol,
) -> viewer::session::DocSession {
    viewer::session::DocSession::inline(doc, tol).with_door(Box::new(NoDoor))
}

/// **A row of `count` copies of `body`** (D10 `[repetition]`): a fresh
/// index `k = index(count)` and one placement of `body` in its own space
/// slid `scalar(k)·pitch` along `axis` ([`pncad::pattern::linear_pattern`]).
/// Answers the document and the family's placement.
pub fn row_of_copies(
    doc: &Doc<ProfileProgram>,
    body: impl Into<pncad::document::Operand>,
    count: i64,
    (axis, pitch): (pncad::document::TargetAxis, f64),
    tol: Tol,
) -> (Doc<ProfileProgram>, RecipeNodeId) {
    let index = viewer::combine::fresh_index(doc);
    let declared = pncad::document::apply(
        doc,
        &pncad::pattern::index_decl(index.clone(), Formula::count(count), None),
        tol,
    )
    .expect("an index declares")
    .doc;
    let body = body.into();
    let space = declared
        .operand_space(&body)
        .unwrap_or(pncad::document::VarId::WORLD);
    let row = pncad::pattern::linear_pattern(
        &index,
        pncad::document::BodyRead::plain(body),
        pncad::document::Operand::Var(space),
        &[(axis, len(pitch))],
    )
    .expect("a length pitch");
    inserted(&declared, row, tol)
}

/// **One member of the family `family`, read alone** — the part door's
/// node ([`viewer::combine::part_node`]).
pub fn member_read(
    doc: &Doc<ProfileProgram>,
    family: RecipeNodeId,
    at: i64,
    tol: Tol,
) -> (Doc<ProfileProgram>, RecipeNodeId) {
    let node =
        viewer::combine::part_node(doc, family, viewer::session::PartSelectSpec::Instance(at))
            .expect("the family is in a space");
    inserted(doc, node, tol)
}

/// **The index the family `family` is evaluated once per value of.**
pub fn family_index(doc: &Doc<ProfileProgram>, family: RecipeNodeId) -> pncad::document::VarId {
    let reach = doc.node_reach(family);
    let &[index] = reach.chain().expect("a family") else {
        panic!("a family of one index");
    };
    index
}

/// **The family `family`'s count, as written.**
pub fn family_count(doc: &Doc<ProfileProgram>, family: RecipeNodeId) -> Formula {
    let index = family_index(doc, family);
    let count = &doc
        .var(index)
        .and_then(|var| var.def().index())
        .expect("an index")
        .count;
    doc.written(count)
}

/// **The op that gives the family `family` `count` members**: the
/// index's count, written through the variable door.
pub fn recount_op(session: &DocSession, family: RecipeNodeId, count: i64) -> SessionOp {
    SessionOp::SetVariable {
        var: family_index(session.committed_doc(), family),
        value: viewer::props::SlotValue::Count(count),
    }
}

/// **The placement editor's Apply for `tool`**, its active row chosen as
/// `choice` says, derived against `session`'s landed pair.
///
/// # Panics
///
/// With nothing landed.
pub fn proposed(
    session: &DocSession,
    tool: &viewer::matetool::MateTool,
    choice: viewer::matetool::MateChoice,
) -> Result<viewer::matetool::MateProposal, viewer::matetool::MateToolError> {
    let (doc, ev) = session.landed_pair().expect("landed");
    tool.clone()
        .chosen(choice)?
        .proposal(doc, ev, session.tol())
}
