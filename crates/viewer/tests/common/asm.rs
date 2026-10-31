//! The assembly fixture the GUI-4 suites share: a workspace directory
//! shaped exactly like the gallery's `assembly/` store — part
//! documents beside the assembly document that pins them — authored
//! through the public doors and saved through the workspace's own
//! write side, so a session that opens the assembly exercises the
//! same open → resolve path the gallery does.
//!
//! Geometry is deliberately tiny (three small prisms, all disjoint)
//! so every suite's evaluations stay cheap; what matters is the
//! STRUCTURE: two pinned part documents, three instances, each pinned
//! to the world by values on the world's axes (`test_support::pin`)
//! and related by no mate. A copy
//! placed into the world is in the product's space, which the free-move
//! probe does not move (DI5), so a free-move row gives post_b a space of
//! its own first ([`unpin`]) and probes it there with [`where_b_stood`].

#![allow(dead_code)] // loaded once per consumer; each uses a subset
#![allow(unreachable_pub)]
// why: root Cargo.toml, the `unreachable_pub` stanza
#![allow(clippy::expect_used)]

use pncad::document::Dimension;
use pncad::document::Formula;
use std::collections::BTreeMap;
use std::path::PathBuf;

use pncad::document::{
    CancelToken, Coord, DocRef, DocumentId, EvalOptions, Evaluation, Frame, Node, ProfileDoc,
    RecipeNodeId, TargetAxis, evaluate,
};
use pncad::geom_core::Tol;
use pncad::prelude::StableName;
use pncad::select::{CapEnd, EntityKind, NamePat, SegPat, SegTag, Selector};
use pncad::workspace::Workspace;
use viewer::session::{DocSession, FaceSelection, SessionOp};

use super::{insert_into, len};

/// The post's square section and height, metres.
pub const POST_SECTION: f64 = 0.02;
pub const POST_HEIGHT: f64 = 0.05;
/// The shelf's plan and thickness, metres.
pub const SHELF_LENGTH: f64 = 0.10;
pub const SHELF_DEPTH: f64 = 0.03;
pub const SHELF_THICKNESS: f64 = 0.01;
/// Where the shelf instance sits (translation), well clear of both
/// posts.
pub const SHELF_AT: [f64; 3] = [0.0, 0.08, 0.0];
/// Where the unconstrained post (post_b) sits.
pub const POST_B_AT: [f64; 3] = [0.06, 0.0, 0.0];

/// The authored workspace and everything a suite needs to drive it.
pub struct Bench {
    /// The workspace directory (the "gallery assembly" directory).
    pub dir: PathBuf,
    /// The assembly document's save file inside it.
    pub asm_path: PathBuf,
    /// The first post instance, pinned to the world at the identity.
    /// An instance is a placement, so its drawn copy is picked on it.
    pub post_a: RecipeNodeId,
    /// The shelf instance, pinned to the world at [`SHELF_AT`].
    pub shelf_i: RecipeNodeId,
    /// The second post instance, pinned to the world at [`POST_B_AT`]
    /// and named by no mate.
    pub post_b: RecipeNodeId,
    /// The post document's reference.
    pub post: DocRef,
    /// The shelf document's reference.
    pub shelf: DocRef,
    /// The two part documents, held as a resolver: what a row that pins
    /// an instance of its own reads the copy's geometry through
    /// ([`Bench::opts`]).
    pub store: crate::fixture::resolver::PartStore,
    /// The post's top-cap name, in the post's own names.
    pub post_top: StableName,
    /// The shelf's bottom-cap name, in the shelf's own names.
    pub shelf_bottom: StableName,
}

/// One extruded box, placed, authored through the ordinary doors.
fn box_part(label: &str, width: f64, depth: f64, height: f64, tol: Tol) -> ProfileDoc {
    let mut doc = ProfileDoc::empty(DocumentId::derive(label), tol);
    let profile = insert_into(&mut doc, super::rectangle([0.0, 0.0], width, depth), tol);
    let extrude = insert_into(
        &mut doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(height),
        },
        tol,
    );
    // The part's world: its one body, placed (A10).
    super::placed(&doc, extrude, tol).0
}

/// A part's own cap-face name at `end`, selected structurally from
/// its evaluated body (the demo's `cap_of` shape), as its placement's
/// copy carries it — the part's product name.
fn cap_of(doc: &ProfileDoc, end: CapEnd, tol: Tol) -> StableName {
    let ev: Evaluation<f64> =
        evaluate(doc, None, &CancelToken::new(), &EvalOptions::default(), tol);
    let placement = *doc.placements().first().expect("the part places its body");
    let body = viewer::world::seat_of(doc, placement);
    let sel =
        Selector::of(NamePat::of_kind(EntityKind::Face).seg(SegPat::tag(SegTag::Cap).side(end)));
    let found = pncad::select::select(&ev, body, &sel);
    assert_eq!(found.len(), 1, "one {end:?} cap: {found:?}");
    found.into_iter().next().expect("checked non-empty")
}

/// Author the workspace into a fresh per-test directory and return
/// its handles. `tag` keeps concurrent tests out of each other's
/// stores.
pub fn bench(tag: &str, tol: Tol) -> Bench {
    let dir = std::env::temp_dir().join(format!("gui4-bench-{}-{tag}", std::process::id()));
    if dir.exists() {
        std::fs::remove_dir_all(&dir).expect("clear the fixture directory");
    }
    std::fs::create_dir_all(&dir).expect("create the fixture directory");
    let mut ws = Workspace::open(&dir).expect("the empty workspace opens");

    let post = box_part("gui4-post", POST_SECTION, POST_SECTION, POST_HEIGHT, tol);
    let shelf = box_part(
        "gui4-shelf",
        SHELF_LENGTH,
        SHELF_DEPTH,
        SHELF_THICKNESS,
        tol,
    );
    ws.create(&post, tol).expect("the post document stores");
    ws.create(&shelf, tol).expect("the shelf document stores");
    let mut store = crate::fixture::resolver::PartStore::new();
    let post_ref = store.insert(post.clone(), tol);
    let shelf_ref = store.insert(shelf.clone(), tol);
    let opts = crate::fixture::resolver::with_resolver(store.clone());

    let mut asm = ProfileDoc::empty(DocumentId::derive("gui4-bench"), tol);
    let post_a = instance_at(&mut asm, post_ref, [0.0; 3], &opts, tol);
    let shelf_i = instance_at(&mut asm, shelf_ref, SHELF_AT, &opts, tol);
    let post_b = instance_at(&mut asm, post_ref, POST_B_AT, &opts, tol);
    // What the assembly makes: its three copies.
    let asm = crate::fixture::assemble_world_copies(asm);
    let asm_path = ws.create(&asm, tol).expect("the assembly stores");

    Bench {
        dir,
        asm_path,
        post_a,
        shelf_i,
        post_b,
        post: post_ref,
        shelf: shelf_ref,
        store,
        post_top: cap_of(&post, CapEnd::End, tol),
        shelf_bottom: cap_of(&shelf, CapEnd::Start, tol),
    }
}

impl Bench {
    /// Evaluation options that reach the bench's parts.
    pub fn opts(&self) -> EvalOptions {
        crate::fixture::resolver::with_resolver(self.store.clone())
    }
}

/// **An instance of `part` pinned to the world at the translation
/// `at`**: an instance whose bundle is empty, then one world mate.
pub fn instance_at(
    asm: &mut ProfileDoc,
    part: DocRef,
    at: [f64; 3],
    opts: &EvalOptions,
    tol: Tol,
) -> RecipeNodeId {
    instance_posed(asm, part, &Frame::translation(at), opts, tol)
}

/// [`instance_at`] at any pose: the pin is values on the world's axes
/// (`test_support::pin`).
pub fn instance_posed(
    asm: &mut ProfileDoc,
    part: DocRef,
    pose: &Frame,
    opts: &EvalOptions,
    tol: Tol,
) -> RecipeNodeId {
    let instance = insert_into(asm, crate::fixture::mated_instance(part), tol);
    *asm = super::pin(asm, instance, pose, opts, tol);
    instance
}

/// **Give `instance`'s copy a space of its own through the session**,
/// and pump (`SessionOp::IntoOwnSpace`): a `new_space()`, the instance
/// re-targeted into it at the identity. It leaves the world, and the
/// free-move probe moves its space. Answers the space.
pub fn unpin(session: &mut DocSession, instance: RecipeNodeId) -> pncad::document::VarId {
    let outcome = session.perform(SessionOp::IntoOwnSpace {
        placement: instance,
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
    session
        .committed_doc()
        .node_space(instance)
        .expect("the copy is placed into its own space")
}

/// **A free-move frame for post_b in its own space, drawn where its pin
/// stood it**, composed with `frame`: `frame ∘ T(POST_B_AT)`.
///
/// A free-move moves the picture of a shown space (DI5), and a copy at
/// the identity in its own space is drawn at its body's own
/// coordinates. In its own space ([`unpin`]), post_b is drawn at the
/// part's origin, so a probe that should show it
/// where the pinned post stood, displaced by `frame` as the display
/// frame displaced the pinned copy, composes the pin's translation in
/// first. A row asserting a drawn position keeps its numbers.
pub fn where_b_stood(frame: Frame) -> Frame {
    frame.compose(&Frame::translation(POST_B_AT))
}

/// **The read of `instance`'s copy**, as a seat that reads one body
/// takes it: an instance defines one body, a copy of its part's
/// product. `instance` is one of the bench's posts or its shelf, or
/// another instance of the same parts.
pub fn copy(
    _bench: &Bench,
    doc: &ProfileDoc,
    instance: RecipeNodeId,
) -> pncad::document::BodyRead<Formula> {
    assert!(
        matches!(doc.node(instance), Some(Node::InstantiatePart { .. })),
        "{instance:?} is an instance"
    );
    pncad::document::BodyRead::plain(pncad::document::Operand::Node(instance))
}

/// **A union of `members`' copies written into the bench's stored
/// assembly**, answering the union. The viewer's boolean form reads
/// whole bodies, and an instance's output is its copies, so a union of
/// instance copies is authored into the file before a session opens it.
pub fn store_union(bench: &Bench, members: &[RecipeNodeId], tol: Tol) -> RecipeNodeId {
    let text = std::fs::read_to_string(&bench.asm_path).expect("the assembly reads");
    let mut doc = pncad::document::load(&text, tol)
        .expect("the assembly loads")
        .doc;
    let reads = members
        .iter()
        .map(|&member| copy(bench, &doc, member))
        .collect();
    let union = insert_into(
        &mut doc,
        Node::Union {
            members: pncad::document::Bodies::Spelled(reads),
        },
        tol,
    );
    std::fs::write(
        &bench.asm_path,
        pncad::document::save(&doc, &[], tol).expect("the assembly saves"),
    )
    .expect("the assembly writes");
    union
}

/// A session opened on the bench's assembly through the typed `Open`
/// door (which is what wires the resolver), evaluated and landed.
pub fn open_bench(bench: &Bench, tol: Tol) -> DocSession {
    let mut session =
        DocSession::inline(pncad::document::Doc::empty_derived("gui4-boot", tol), tol);
    let outcome = session.perform(SessionOp::Open(bench.asm_path.clone()));
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
    session
}

/// **A part-local face, read at `instance`**: a copy carries its
/// part's names as they are, so the name is the part's and the site
/// is the instance whose copy carries it — for rows that author a mate
/// directly.
pub fn in_part(instance: RecipeNodeId, local: &StableName) -> crate::fixture::resolver::PartName {
    crate::fixture::resolver::PartName {
        at: instance,
        name: local.clone(),
    }
}

/// **The part-local face a mate side reads**: the side walked to its
/// instance and the instance's wrapper taken off (`head_face`, the
/// kernel's one strip) — what a tool row compares a proposal's side
/// against. `None` for a side outside the member vocabulary.
pub fn side_face(
    doc: &pncad::document::Doc<pncad::document::ProfileProgram>,
    side: &pncad::document::SitedFace,
) -> Option<StableName> {
    pncad::document::head_face(doc, side).map(pncad::document::FaceName::into_name)
}

// The assembly suites say `asm::down_at` / `asm::up_at`; both name the
// shared rays, re-exported rather than restated.
pub use super::{down_at, up_at};

/// **The seat's values, at `b_x` along the shelf**: a post's top cap
/// rests on the shelf's underside (a plane mate, flipped), which leaves
/// the post free to slide and spin in the shelf's plane; the values put
/// its top cap's middle under the shelf at `b_x` along it and midway
/// across it, unturned. Each value is measured on the axes of the space
/// the shelf stands in — the world, the shelf at [`SHELF_AT`]
/// ([`seat_values_at`]).
pub fn seat_values(b_x: f64) -> Vec<(Coord, pncad::document::Formula)> {
    seat_values_at(b_x, SHELF_AT)
}

/// [`seat_values`] under a shelf standing unturned at `shelf_at` in its
/// space: the post's origin from the space's, on the space's own axes —
/// the mated direction is the underside's flipped normal, the space's
/// `+z`, where the identity orientation puts the post's top, so the least
/// turn is the identity.
pub fn seat_values_at(b_x: f64, shelf_at: [f64; 3]) -> Vec<(Coord, pncad::document::Formula)> {
    vec![
        (
            Coord::Slide(TargetAxis::X),
            len(shelf_at[0] + b_x - POST_SECTION / 2.0),
        ),
        (
            Coord::Slide(TargetAxis::Y),
            len(shelf_at[1] + SHELF_DEPTH / 2.0 - POST_SECTION / 2.0),
        ),
        (Coord::SPIN, Formula::zero(Dimension::Angle)),
    ]
}

/// [`seat_values`] under the shelf's middle.
pub fn middle_seat_values() -> Vec<(Coord, pncad::document::Formula)> {
    seat_values(SHELF_LENGTH / 2.0)
}

/// [`seat_values_at`] under the shelf's middle, the shelf standing at
/// its space's identity — a shelf given a space of its own ([`unpin`]),
/// or pinned at the world's origin.
pub fn own_space_seat_values() -> Vec<(Coord, pncad::document::Formula)> {
    seat_values_at(SHELF_LENGTH / 2.0, [0.0; 3])
}

/// **Where [`seat_values`] puts a post under the bench shelf**: the
/// post unturned, its top cap on the shelf's underside, its middle at
/// `b_x` along the shelf and midway across it.
pub fn seated_at(b_x: f64) -> Frame {
    Frame::translation([
        SHELF_AT[0] + b_x - POST_SECTION / 2.0,
        SHELF_AT[1] + SHELF_DEPTH / 2.0 - POST_SECTION / 2.0,
        SHELF_AT[2] - POST_HEIGHT,
    ])
}

/// **Asserts `placement` solves to `want`** in the session's landed
/// document, each entry of the rigid motion within `1e-9`.
pub fn assert_solved_at(session: &DocSession, placement: RecipeNodeId, want: &Frame, what: &str) {
    let (doc, _) = session.landed_pair().expect("landed");
    let got = super::solve(session, doc, Tol::witness())
        .placement(doc, placement)
        .unwrap_or_else(|refusal| panic!("{what}: {placement:?} has no pose: {refusal}"));
    let close = got
        .columns
        .iter()
        .flatten()
        .chain(&got.translation)
        .zip(want.columns.iter().flatten().chain(&want.translation))
        .all(|(g, w)| (g - w).abs() < 1e-9);
    assert!(close, "{what}: {got:?} vs {want:?}");
}

/// The mate tool's choice for the seat: a mate of two planes, the
/// target flipped (the faces rest against each other), at
/// [`middle_seat_values`].
pub fn seat_choice() -> viewer::matetool::MateChoice {
    viewer::matetool::MateChoice {
        kind: pncad::document::VarKind::Plane,
        flip: true,
        values: middle_seat_values(),
    }
}

/// **The seat mate as the op the session takes**: `post`'s top cap
/// resting on the bench shelf's underside, with `values`,
/// in `post`'s bundle.
///
/// One home for the op because the two faces it names are one fact
/// about this fixture; what a row varies is the post and the values. A row hands it to [`super::commit_mate`] or
/// [`super::session_insert`], or performs it itself when its subject
/// is the outcome.
pub fn seat_op(
    bench: &Bench,
    post: RecipeNodeId,
    values: Vec<(Coord, pncad::document::Formula)>,
) -> SessionOp {
    seat_op_under(bench, post, bench.shelf_i, values)
}

/// [`seat_op`] under a shelf instance other than the bench's — one a
/// row authored itself.
pub fn seat_op_under(
    bench: &Bench,
    post: RecipeNodeId,
    shelf: RecipeNodeId,
    values: Vec<(Coord, pncad::document::Formula)>,
) -> SessionOp {
    let (post_top, shelf_bottom) = seat_faces(bench, post, shelf);
    SessionOp::AddMate {
        placement: post,
        mate: Box::new(crate::fixture::rest(post_top, shelf_bottom)),
        values,
        into_target_space: true,
    }
}

/// **The shelf resting on `post`'s top, `offset` above it**, as the op
/// the session takes: the shelf's underside on the post's top cap,
/// flipped and slid `offset` along its own normal (a standoff), in the
/// the shelf's bundle written whole: the rest and the constant `0` on
/// each freedom it leaves. A second rest beside it fixes the plane
/// again, which the edit door refuses.
pub fn shelf_rest_op(bench: &Bench, post: RecipeNodeId, offset: f64) -> SessionOp {
    use pncad::document::pose::{PoseDef, PoseRole, RoleRead};
    let (post_top, shelf_bottom) = seat_faces(bench, post, bench.shelf_i);
    let seat = crate::fixture::flip(crate::fixture::plane(post_top));
    let to = pncad::document::Operand::Pose(Box::new(PoseDef::Slide {
        pose: RoleRead::plain(seat.clone()),
        along: RoleRead::through(seat, PoseRole::Normal),
        by: len(offset),
    }));
    SessionOp::AddMate {
        placement: bench.shelf_i,
        mate: Box::new(crate::fixture::mate(
            crate::fixture::plane(shelf_bottom),
            to,
        )),
        values: Vec::new(),
        into_target_space: true,
    }
}

/// **A mate the shelf's bundle refuses where it evaluates**: a coaxial
/// mate on the axis of the shelf's underside, which a plane has none
/// of, against the axis of `post`'s top, its slide and spin the
/// constant `0`. The door admits it (its kinds agree, its `on` reads the
/// shelf and the bundle writes every freedom); the placement refuses it
/// where it evaluates, its `on` pose unevaluated.
pub fn no_centre_op(bench: &Bench, post: RecipeNodeId) -> SessionOp {
    let (post_top, shelf_bottom) = seat_faces(bench, post, bench.shelf_i);
    SessionOp::AddMate {
        placement: bench.shelf_i,
        mate: Box::new(crate::fixture::mate(
            crate::fixture::axis(shelf_bottom),
            crate::fixture::axis(post_top),
        )),
        values: Vec::new(),
        into_target_space: true,
    }
}

/// **The shelf's bundle refused, and post_b chained to it**: post_b
/// seated under the shelf's middle (`sound`, post_b's bundle), then the
/// shelf's bundle one mate the evaluation refuses ([`no_centre_op`],
/// the `offender`, against post_a), all in the world; each bundle's
/// mates are taken off first ([`unpin`]). The shelf has no pose, and post_b, placed against
/// the shelf, has none either; post_a, which only the shelf's
/// constraints read, keeps its own. Committed, each pumped.
pub fn refuse_shelf(session: &mut DocSession, bench: &Bench) -> Refused {
    refuse_shelf_chaining(session, bench, bench.post_b, bench.post_a)
}

/// [`refuse_shelf`] with `chained` seated under the shelf and the
/// shelf's refused mate against `rest_on`.
pub fn refuse_shelf_chaining(
    session: &mut DocSession,
    bench: &Bench,
    chained: RecipeNodeId,
    rest_on: RecipeNodeId,
) -> Refused {
    // Its pin off first, by a space of its own: a seat beside a mate the
    // bundle holds is refused.
    unpin(session, chained);
    let sound = super::commit_mate(session, seat_op(bench, chained, middle_seat_values()));
    // The shelf's mates off, by a space of its own: a mate beside one
    // the bundle holds is refused. The refused mate places it back.
    unpin(session, bench.shelf_i);
    let offender = super::commit_mate(session, no_centre_op(bench, rest_on));
    Refused { sound, offender }
}

/// The two constraints [`refuse_shelf`] commits.
#[derive(Debug, Clone, Copy)]
pub struct Refused {
    /// post_b's seat under the shelf, in post_b's bundle.
    pub sound: pncad::document::MateRef,
    /// The shelf's one mate, which its evaluation refuses.
    pub offender: pncad::document::MateRef,
}

/// The two faces a seat names: `post`'s top cap and `shelf`'s
/// underside, each as its instance holds it.
fn seat_faces(
    bench: &Bench,
    post: RecipeNodeId,
    shelf: RecipeNodeId,
) -> (pncad::document::SitedFace, pncad::document::SitedFace) {
    (
        super::head(in_part(post, &bench.post_top)),
        super::head(in_part(shelf, &bench.shelf_bottom)),
    )
}

/// A `BTreeMap` from a small list — the shape a few rows want for
/// expected-per-instance assertions.
pub fn map_of<K: Ord, V>(entries: impl IntoIterator<Item = (K, V)>) -> BTreeMap<K, V> {
    entries.into_iter().collect()
}

/// The display δ every assembly suite uses: coarse (the fixture is
/// all planes) and cheap.
pub fn delta() -> viewer::scene::DisplayTolerance {
    viewer::scene::DisplayTolerance::new(1.0e-3).expect("a positive delta")
}

/// The pick index for a session's landed evaluation, at this
/// fixture's δ.
pub fn index_of(session: &DocSession) -> viewer::pickindex::PickIndex {
    super::index_of(session, delta())
}

/// [`super::displayed_face_at`] through a fresh [`index_of`] — one
/// pick, when a row has no index of its own to reuse.
pub fn pick_face(session: &DocSession, ray: &pncad::select::Ray) -> FaceSelection {
    super::displayed_face_at(session, &index_of(session), ray)
}

/// The ray straight up through the middle of the shelf's footprint:
/// from below, the first face it meets is the shelf's underside unless
/// a row has put something between.
pub fn under_shelf() -> pncad::select::Ray {
    up_at(
        SHELF_AT[0] + SHELF_LENGTH / 2.0,
        SHELF_AT[1] + SHELF_DEPTH / 2.0,
    )
}

/// The ray straight down through the middle of post_b's AUTHORED
/// footprint: its top cap while post_b stands where the bench put it,
/// and a miss once a probe or a mate has moved it.
pub fn over_post_b() -> pncad::select::Ray {
    down_at(
        POST_B_AT[0] + POST_SECTION / 2.0,
        POST_B_AT[1] + POST_SECTION / 2.0,
    )
}

/// The face [`under_shelf`] picks — the shelf's underside, or whatever
/// instance a row has placed there.
pub fn shelf_underside(session: &DocSession) -> FaceSelection {
    pick_face(session, &under_shelf())
}

/// **The two picks the seat mate starts from**: post_b's top cap from
/// above, then the shelf's underside from below, each at its face's
/// middle and each checked to land on the instance it aims at.
pub fn seat_picks(session: &DocSession, bench: &Bench) -> (FaceSelection, FaceSelection) {
    let post_top = pick_face(session, &over_post_b());
    assert_eq!(post_top.node, bench.post_b, "the first pick is post_b's");
    let shelf_bottom = shelf_underside(session);
    assert_eq!(
        shelf_bottom.node, bench.shelf_i,
        "the second pick is the shelf's"
    );
    (post_top, shelf_bottom)
}

/// **The feature tree's instance rows**: the rows of `InstantiatePart`
/// nodes, without the rows of the bodies' own placements (A10) — what
/// a row about resolving instances reads.
pub fn instance_rows(session: &DocSession) -> Vec<viewer::tree::TreeRow> {
    session
        .tree_rows()
        .into_iter()
        .filter(|row| row.spoken.kind() == Some("InstantiatePart"))
        .collect()
}

/// **The committed free-move frame over the space `instance`'s copy is
/// placed into now**, if any: frames are keyed by space (DI5), and a
/// row asks after the instance it probed.
pub fn frame_of(session: &DocSession, instance: RecipeNodeId) -> Option<&Frame> {
    let space = session.committed_doc().node_space(instance)?;
    session.display().free_move_of(space)
}
