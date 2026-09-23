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
    RecipeNodeId, apply,
};
use pncad::geom_core::Tol;
use viewer::sketch::{Notation, ProfileShape};

/// Apply one edit, answering the new document and any minted id.
pub fn edited(
    doc: &Doc<ProfileProgram>,
    edit: DocEdit<ProfileProgram>,
    tol: Tol,
) -> (Doc<ProfileProgram>, Option<RecipeNodeId>) {
    let applied = apply(doc, &edit, tol, &pncad::document::RefusingReach)
        .expect("the fixture's edit applies");
    (applied.doc, applied.record.minted)
}

/// **A document holding one declared parameter and nothing else** —
/// the fixture both panel suites build their parameter rows on.
///
/// `label` is the document's derived name, so two fixtures in one
/// binary cannot share an identity. No oracle: it is the spelling of
/// `Doc::empty_derived` plus one `SetDocParam`, and what each row
/// asserts is about the `value` it handed in.
pub fn declared(label: &str, name: &ParamName, value: DocParam) -> Doc<ProfileProgram> {
    let tol = Tol::witness();
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

/// Insert a node, answering the new document and the minted id.
pub fn inserted(
    doc: &Doc<ProfileProgram>,
    node: Node<ProfileProgram>,
    tol: Tol,
) -> (Doc<ProfileProgram>, RecipeNodeId) {
    let (doc, minted) = edited(doc, DocEdit::InsertNode { node }, tol);
    (doc, minted.expect("an insert mints an id"))
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

/// A sketch frame node's payload.
pub fn frame(origin: [f64; 3], u: [f64; 3], v: [f64; 3]) -> Node<ProfileProgram> {
    Node::Datum(pncad::document::Datum::Frame {
        origin: len3(origin),
        u: scl3(u),
        v: scl3(v),
    })
}

/// The world xy frame's payload — the plane these fixtures sketch on.
pub fn xy_frame() -> Node<ProfileProgram> {
    frame([0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0])
}

/// An axis-aligned rectangular profile node's payload on `plane`:
/// `w` by `h`, its lower-left corner at `origin` in the plane's own
/// coordinates. `square` is this with two equal sides at the plane
/// origin, and a fixture whose block sits elsewhere moves `origin`.
pub fn rectangle(plane: RecipeNodeId, origin: [f64; 2], w: f64, h: f64) -> Node<ProfileProgram> {
    let [x0, y0] = origin;
    Node::Profile(ProfileProgram {
        plane,
        loops: vec![
            LoopProgram::polygon([(x0, y0), (x0 + w, y0), (x0 + w, y0 + h), (x0, y0 + h)])
                .expect("finite corners"),
        ],
    })
}

/// A square profile node's payload on `plane`, `side` metres on a side,
/// at the plane origin.
pub fn square(plane: RecipeNodeId, side: f64) -> Node<ProfileProgram> {
    rectangle(plane, [0.0, 0.0], side, side)
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

/// **The witnessed band a placement axis is decided under** — what
/// `Frame::rotate_then_translate` asks the direction door with. Rows
/// whose axis is a literal pass this and unwrap; a row whose SUBJECT
/// is the axis decision reads the refusal instead.
pub fn band() -> pncad::geom_core::Band {
    pncad::geom_core::Band::linear(Tol::witness()).expect("the witnessed band")
}

/// A length literal.
pub fn len(metres: f64) -> Expr {
    Expr::literal(metres, Dimension::Length).expect("a finite length")
}

/// A dimensionless literal.
pub fn scl(value: f64) -> Expr {
    Expr::literal(value, Dimension::Scalar).expect("a finite scalar")
}

/// An angle literal.
pub fn ang(radians: f64) -> Expr {
    Expr::literal(radians, Dimension::Angle).expect("a finite angle")
}

/// Three length literals — a datum origin, a translation.
pub fn len3(v: [f64; 3]) -> [Expr; 3] {
    [len(v[0]), len(v[1]), len(v[2])]
}

/// Three dimensionless literals — a normal, a direction, an axis.
pub fn scl3(v: [f64; 3]) -> [Expr; 3] {
    [scl(v[0]), scl(v[1]), scl(v[2])]
}

/// Two length literals — a point in a sketch frame's own coordinates.
pub fn len2(v: [f64; 2]) -> [Expr; 2] {
    [len(v[0]), len(v[1])]
}

/// Two dimensionless literals — a direction in a sketch frame.
pub fn scl2(v: [f64; 2]) -> [Expr; 2] {
    [scl(v[0]), scl(v[1])]
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
use viewer::session::{DocSession, SessionOp};

/// Add the world xy frame through the session, answering its id — the
/// pick every `SessionOp::AddProfile` below hands over.
///
/// Through the vocabulary's own numbers (`ProfilePlane::world_xy`)
/// rather than a second spelling of them here: the add-profile form's
/// `NewXy` choice mints that frame, so a suite that hand-wrote the
/// components would stop testing the frame the chrome authors the
/// moment either moved.
pub fn xy_frame_in(session: &mut DocSession) -> RecipeNodeId {
    insert(
        session,
        SessionOp::AddDatum {
            datum: viewer::session::ProfilePlane::world_xy().expect("the world xy frame lowers"),
        },
    )
}

/// Perform one op that must commit exactly one insert, answering the
/// id of the node it minted.
pub fn insert(session: &mut DocSession, op: SessionOp) -> RecipeNodeId {
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

// The mate-head helpers are `crate::fixture`'s, re-exported rather than
// re-written: `tests/fixture/` is editor-core's tree, symlinked into
// this one and mounted by this binary's root, so a second body here
// would be a second definition of one fixture claim — and
// `pncad::document::SitedFace` IS `editor_core::SitedFace`, the façade
// re-exporting the kernel's type rather than wrapping it. A suite says
// `common::head` as before.
pub use crate::fixture::{head, head_at};
