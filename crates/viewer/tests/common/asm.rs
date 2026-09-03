//! The assembly fixture the GUI-4 suites share: a workspace directory
//! shaped exactly like the gallery's `assembly/` store — part
//! documents beside the assembly document that pins them — authored
//! through the public doors and saved through the workspace's own
//! write side, so a session that opens the assembly exercises the
//! same open → resolve path the gallery does.
//!
//! Geometry is deliberately tiny (three small prisms, all disjoint)
//! so every suite's evaluations stay cheap; what matters is the
//! STRUCTURE: two pinned part documents, three instances, no mates —
//! post_b completely unconstrained, which is what the free-move and
//! mate-tool rows need to start from.

#![allow(dead_code)] // loaded once per consumer; each uses a subset
#![allow(unreachable_pub)]
// why: root Cargo.toml, the `unreachable_pub` stanza
#![allow(clippy::expect_used)]

use std::collections::BTreeMap;
use std::path::PathBuf;

use pncad::document::{
    CancelToken, Dimension, DocEdit, DocRef, DocumentId, EvalOptions, Evaluation, Expr, Frame,
    LoopProgram, Node, ProfileDoc, ProfileProgram, RecipeNodeId, apply, content_pin, evaluate,
};
use pncad::geom_core::{Point3, Tol, Vec3};
use pncad::prelude::StableName;
use pncad::select::{CapEnd, EntityKind, NamePat, Ray, SegPat, SegTag, Selector};
use pncad::workspace::Workspace;
use viewer::session::{DocSession, SessionOp};

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
    /// The gauge post instance (identity placement).
    pub post_a: RecipeNodeId,
    /// The shelf instance (explicit frame).
    pub shelf_i: RecipeNodeId,
    /// The completely-unconstrained post instance.
    pub post_b: RecipeNodeId,
    /// The post document's reference.
    pub post: DocRef,
    /// The shelf document's reference.
    pub shelf: DocRef,
    /// The post's top-cap name, in the post's own names.
    pub post_top: StableName,
    /// The shelf's bottom-cap name, in the shelf's own names.
    pub shelf_bottom: StableName,
}

fn len(metres: f64) -> Expr {
    Expr::literal(metres, Dimension::Length).expect("a length literal")
}

fn insert(doc: &mut ProfileDoc, node: Node<ProfileProgram>, tol: Tol) -> RecipeNodeId {
    let applied = apply(doc, &DocEdit::InsertNode { node }, tol).expect("the insert applies");
    *doc = applied.doc;
    applied.record.minted.expect("an insert mints an id")
}

fn edit(doc: &mut ProfileDoc, e: &DocEdit<ProfileProgram>, tol: Tol) {
    let applied = apply(doc, e, tol).expect("the edit applies");
    *doc = applied.doc;
}

/// One extruded box, authored through the ordinary doors.
fn box_part(label: &str, width: f64, depth: f64, height: f64, tol: Tol) -> ProfileDoc {
    let mut doc = ProfileDoc::empty(DocumentId::derive(label), tol);
    let outline = LoopProgram::polygon([(0.0, 0.0), (width, 0.0), (width, depth), (0.0, depth)])
        .expect("a literal rectangle");
    let plane = insert(&mut doc, super::xy_frame(), tol);
    let profile = insert(
        &mut doc,
        Node::Profile(ProfileProgram {
            plane,
            loops: vec![outline],
        }),
        tol,
    );
    insert(
        &mut doc,
        Node::Extrude {
            profile,
            distance: len(height),
        },
        tol,
    );
    doc
}

/// A part's own cap-face name at `end`, selected structurally from
/// its evaluated product (the demo's `cap_of` shape).
fn cap_of(doc: &ProfileDoc, end: CapEnd, tol: Tol) -> StableName {
    let ev: Evaluation<f64> =
        evaluate(doc, None, &CancelToken::new(), &EvalOptions::default(), tol);
    let tip = *doc.roots().first().expect("the part has a product root");
    let sel =
        Selector::of(NamePat::of_kind(EntityKind::Face).seg(SegPat::tag(SegTag::Cap).side(end)));
    let found = pncad::select::select(&ev, tip, &sel);
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
    let reference = |d: &ProfileDoc| DocRef {
        id: d.id(),
        pin: content_pin(d, tol).expect("the pin computes"),
    };
    let post_ref = reference(&post);
    let shelf_ref = reference(&shelf);

    let mut asm = ProfileDoc::empty(DocumentId::derive("gui4-bench"), tol);
    let post_a = insert(&mut asm, Node::instantiate_part(post_ref), tol);
    let shelf_i = insert(&mut asm, Node::instantiate_part(shelf_ref), tol);
    edit(
        &mut asm,
        &DocEdit::SetPlacement {
            node: shelf_i,
            frame: Frame::translation(SHELF_AT),
        },
        tol,
    );
    let post_b = insert(&mut asm, Node::instantiate_part(post_ref), tol);
    edit(
        &mut asm,
        &DocEdit::SetPlacement {
            node: post_b,
            frame: Frame::translation(POST_B_AT),
        },
        tol,
    );
    let asm_path = ws.create(&asm, tol).expect("the assembly stores");

    Bench {
        dir,
        asm_path,
        post_a,
        shelf_i,
        post_b,
        post: post_ref,
        shelf: shelf_ref,
        post_top: cap_of(&post, CapEnd::Top, tol),
        shelf_bottom: cap_of(&shelf, CapEnd::Bottom, tol),
    }
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

/// The instance-qualified spelling of a part-local name (the GQ4
/// wrapper), for rows that author a mate directly.
pub fn in_part(instance: RecipeNodeId, local: &StableName) -> StableName {
    StableName {
        kind: local.kind,
        node: instance,
        path: vec![pncad::select::RoleSeg::InPart {
            of: Box::new(local.clone()),
        }],
    }
}

/// A ray straight down onto the assembly at `(x, y)`.
pub fn down_at(x: f64, y: f64) -> Ray {
    Ray {
        origin: Point3::new(x, y, 1.0),
        dir: Vec3::new(0.0, 0.0, -1.0),
    }
}

/// A ray straight up from under the assembly at `(x, y)` — how a
/// part's underside is picked.
pub fn up_at(x: f64, y: f64) -> Ray {
    Ray {
        origin: Point3::new(x, y, -1.0),
        dir: Vec3::new(0.0, 0.0, 1.0),
    }
}

/// The seat choice the mate rows commit: Rest at frame coincidence,
/// axes opposed, no clocking rider — on a frame coincidence the coset
/// table decides any nonzero rider contradictory.
pub fn seat() -> viewer::matetool::MateChoice {
    viewer::matetool::MateChoice {
        class: pncad::select::ContactClass::Rest,
        primitive: pncad::document::MatePrimitive::FrameCoincidence,
        sense: pncad::document::AxisSense::Opposed,
        clocking: None,
    }
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

/// The pick index for a session's landed evaluation.
pub fn index_of(session: &DocSession) -> viewer::pick::PickIndex {
    let (doc, eval) = session.landed_pair().expect("an evaluation has landed");
    let generation = session
        .landed_generation()
        .expect("a landed evaluation has a generation");
    viewer::pick::PickIndex::build(doc, eval, generation, delta(), session.tol())
        .expect("the assembly indexes")
}
