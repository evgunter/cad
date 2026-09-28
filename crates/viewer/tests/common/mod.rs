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

use bvh::Aabb;
use pncad::document::{SolvedPoses, mate_reach, solve_document};
use pncad::geom_core::Point3;
use viewer::camera::Camera;

/// **The mate solve of `doc` under `session`'s own seam** — the lever
/// is each mated part's extent, resolved through the session's
/// resolver the way its landed evaluation resolved it
/// (`DocSession::eval_options`), built through the kernel's public
/// door. `doc` is the session's landed document, or one derived from
/// it that resolves against the same directory.
pub fn solve(session: &DocSession, doc: &Doc<ProfileProgram>, tol: Tol) -> SolvedPoses {
    let opts = session.eval_options();
    let reach = mate_reach::<f64>(&opts, tol);
    solve_document(doc, &reach, tol)
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
    Camera::framing(&plate_bounds(), aspect).expect("the plate frames")
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
// would: parameters before the expressions that read them, nodes
// before the nodes that consume them. A fixture that reached past
// `apply` would be testing a document the edit vocabulary cannot
// produce.

use pncad::document::{
    Dimension, Doc, DocEdit, DocParam, Expr, LoopProgram, Node, ParamName, ProfileProgram,
    RecipeNodeId,
};
use pncad::geom_core::Tol;
use viewer::sketch::{Notation, ProfileShape};

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
pub fn shape(template: &ProfileShape) -> LoopProgram {
    viewer::sketch::loop_program(template, Notation::CANONICAL).expect("a finite template")
}

/// The name of the parametric fixture's driving parameter.
pub fn thickness_param() -> ParamName {
    ParamName::new("thickness")
}

/// A document whose extrude distance is DRIVEN by a document
/// parameter — the expression-driven-dimension fixture.
///
/// Answers the document, the profile node and the extrude node.
pub fn parametric_plate(tol: Tol) -> (Doc<ProfileProgram>, RecipeNodeId, RecipeNodeId) {
    let doc: Doc<ProfileProgram> = Doc::empty_derived("gui3-parametric", tol);
    let (doc, _) = edited(
        &doc,
        DocEdit::SetDocParam {
            name: thickness_param(),
            value: DocParam::continuous(Dimension::Length, 0.008),
        },
        tol,
    );
    let (doc, profile) = framed_square(&doc, 0.04, tol);
    let (doc, extrude) = inserted(
        &doc,
        Node::Extrude {
            profile,
            // `thickness / 2` — a composed expression over a
            // parameter, which is the shape the refusal affordance
            // exists for.
            distance: Expr::div(Expr::param(thickness_param(), Dimension::Length), scl(2.0))
                .expect("length / scalar is a length"),
        },
        tol,
    );
    (doc, profile, extrude)
}

/// A document that FAILS at one node and poisons its descendant.
///
/// The failure is a division by a zero literal in the extrude's
/// distance — an expression that is well-dimensioned at the edit door
/// and non-finite at evaluation, which is exactly the shape GQ2's
/// per-node result exists to report. Answers the document, the failing
/// extrude and the poisoned transform.
pub fn broken_document(tol: Tol) -> (Doc<ProfileProgram>, RecipeNodeId, RecipeNodeId) {
    let doc: Doc<ProfileProgram> = Doc::empty_derived("gui3-broken", tol);
    let (doc, profile) = framed_square(&doc, 0.04, tol);
    let (doc, extrude) = inserted(
        &doc,
        Node::Extrude {
            profile,
            distance: Expr::div(len(0.008), scl(0.0)).expect("length / scalar is a length"),
        },
        tol,
    );
    let (doc, moved) = inserted(
        &doc,
        Node::Transform {
            input: extrude,
            translation: [len(0.01), len(0.0), len(0.0)],
            rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
            rotation_angle: ang(0.0),
        },
        tol,
    );
    (doc, extrude, moved)
}

/// The gallery ring document as `demo-tour gallery` saved it (the
/// exporter round trip with the dialog and the window taken out).
///
/// It is version-stamped in its name, as `pncad`'s own fixture is: a
/// schema break makes this file unreadable, and the fix is to
/// regenerate it from `demo-tour gallery` and rename, never to teach
/// the loader about an old shape.
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

use pncad::document::{BooleanValue, NodeResult};
use pncad::prelude::ValuePayload;
use viewer::session::{DocSession, FaceSelection, SessionOp};

/// Add the world xy frame through the session, answering its id — the
/// pick every `SessionOp::AddProfile` below hands over.
///
/// Through the vocabulary's own numbers (`ProfilePlane::world_xy`)
/// rather than a second spelling of them here: the add-profile form's
/// `NewXy` choice mints that frame, so a suite that hand-wrote the
/// components would stop testing the frame the chrome authors the
/// moment either moved.
pub fn xy_frame_in(session: &mut DocSession) -> RecipeNodeId {
    session_insert(
        session,
        SessionOp::AddDatum {
            datum: viewer::session::ProfilePlane::world_xy().expect("the world xy frame lowers"),
        },
    )
}

/// **Insert through the session**: perform one op that must commit
/// exactly one insert, answering the id of the node it minted.
///
/// This is the op vocabulary's door, and it holds the session's
/// contract — no refusal, one committed edit, and that edit an
/// `InsertNode`. [`inserted`] and [`insert_into`] are the document's
/// door instead: they call `apply` with no session, and check only
/// that the edit applies and mints an id — there is no op, so no
/// outcome to hold to that contract. A fixture that means to exercise
/// the chrome's ops reaches for this one.
pub fn session_insert(session: &mut DocSession, op: SessionOp) -> RecipeNodeId {
    let outcome = session.perform(op);
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert_eq!(outcome.committed.len(), 1, "exactly one committed edit");
    assert!(matches!(
        outcome.committed.first(),
        Some(DocEdit::InsertNode { .. })
    ));
    *session
        .committed_doc()
        .order()
        .last()
        .expect("the insert landed")
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

/// **Commit a mate through the session's insert door**, pump, and
/// answer the node it minted — checked to BE a mate, because that id
/// is what the solve keys a fault by.
///
/// A pattern node, a `Part` node or an instance is NOT such a key:
/// `SolvedPoses::fault` maps refusing MATES and the instances of a
/// cluster that consequently has no pose, so `fault(pattern)` answers
/// `None` for every document ever written and asserts nothing. The
/// kind check is what keeps a row's `fault(mate).is_none()` from
/// passing on an id it could never fail on.
///
/// A row that authors two mates into ONE evaluation does not pump
/// between them, and so takes [`session_insert`] instead.
///
/// # Panics
///
/// If the op refuses, commits anything but one insert, or inserts a
/// node that is not a `Node::Mate`.
pub fn commit_mate(session: &mut DocSession, op: SessionOp) -> RecipeNodeId {
    let mate = session_insert(session, op);
    assert!(
        matches!(session.committed_doc().node(mate), Some(Node::Mate { .. })),
        "the op inserts a mate: {:?}",
        session.committed_doc().node(mate)
    );
    session.pump();
    mate
}

/// **A rectangle profile through the session**: the chrome's rectangle
/// template, `width` by `height` and centred on `plane`'s origin,
/// drawn by `SessionOp::AddProfile` — answering the profile.
pub fn rectangle_in(
    session: &mut DocSession,
    plane: RecipeNodeId,
    width: f64,
    height: f64,
) -> RecipeNodeId {
    session_insert(
        session,
        SessionOp::AddProfile {
            plane: viewer::session::ProfilePlane::Existing(plane),
            loops: vec![shape(&ProfileShape::Rectangle { width, height })],
        },
    )
}

/// **A box through the session**: [`rectangle_in`] on `plane`, then
/// `SessionOp::AddExtrude` by `depth` — answering the profile and the
/// extrude.
pub fn box_in(
    session: &mut DocSession,
    plane: RecipeNodeId,
    [width, height, depth]: [f64; 3],
) -> (RecipeNodeId, RecipeNodeId) {
    let profile = rectangle_in(session, plane, width, height);
    let extrude = session_insert(
        session,
        SessionOp::AddExtrude {
            profile,
            distance: len(depth),
        },
    );
    (profile, extrude)
}

/// [`box_in`] on a fresh world xy frame ([`xy_frame_in`]) — answering
/// the extrude, the body a row goes on to combine, blend or measure.
pub fn xy_box_in(session: &mut DocSession, size: [f64; 3]) -> RecipeNodeId {
    let plane = xy_frame_in(session);
    box_in(session, plane, size).1
}

/// A closed polygon through `points`, in order, as the step chain a
/// `ProfileShape::Path` carries: an `At` on the first point, a line to
/// each of the rest, and a line back to the start.
pub fn polygon_steps(points: &[(f64, f64)]) -> Vec<pncad::profile::Step<f64>> {
    use pncad::geom_core::Point2;
    use pncad::profile::{Step, Target};
    let mut steps = vec![Step::At(Point2::new(points[0].0, points[0].1))];
    for &(x, y) in &points[1..] {
        steps.push(Step::LineTo(Target::Point(Point2::new(x, y))));
    }
    steps.push(Step::LineTo(Target::Start));
    steps
}

/// One node's row status out of a tree render — the lookup five
/// suites had written out by hand.
///
/// Panics rather than answering `None`: every id these rows pass is
/// one the document holds, so a missing row is the failure, not a
/// case to handle.
pub fn status_of(rows: &[viewer::tree::TreeRow], id: RecipeNodeId) -> viewer::tree::RowStatus {
    rows.iter()
        .find(|row| row.id == id)
        .map(|row| row.status.clone())
        .unwrap_or_else(|| panic!("node {id:?} has a row"))
}

/// `got` and `want` agree to one part in 10⁹, relatively.
///
/// The 1e-9 is a chosen bound, not a derived one: the closed-form
/// volume rows in the suites that share this helper hold it with
/// margin, and it is kept tight so real drift cannot hide inside it.
pub fn near(got: f64, want: f64) -> bool {
    ((got - want) / want).abs() < 1e-9
}

/// The evaluated volume of `node`'s single body — an extrude's, a
/// blend's, or a boolean's — with the seam pumped.
///
/// The evaluation read is the SHOWN document's, so mid-gesture this
/// measures the scratch preview exactly as the viewport does. A node
/// that failed to evaluate panics with the node's own recorded error,
/// not just the absence of a value.
pub fn body_volume(session: &mut DocSession, node: RecipeNodeId, tol: Tol) -> f64 {
    session.pump();
    let eval = session.evaluation().expect("the inline seam landed");
    let value = eval.value(node).unwrap_or_else(|| {
        panic!(
            "the node evaluated: {:?}",
            eval.result(node).and_then(NodeResult::error)
        )
    });
    let body = match &value.payload {
        ValuePayload::Body(body) => body.clone(),
        ValuePayload::Boolean(BooleanValue::Body { body, .. }) => body.clone(),
        other => panic!("expected a body, got {other:?}"),
    };
    pncad::topo::mass_properties(&body, tol)
        .expect("mass properties")
        .volume
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
/// refusal — a failed or poisoned root is an ordinary editing state,
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
    PickIndex::build(doc, eval, PictureKey::of(generation, delta), session.tol())
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
