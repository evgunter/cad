//! **The assembly stop** (ASSEMBLY-DESIGN A2–A13, the ASM program's
//! v1 scope): three part documents, a workspace on disk, and two
//! assembly documents built out of them — one a flat-pack layout that
//! patterns a part, one a mated bench standing on a turntable, whose
//! placements are solved constructively from their mates onto the one
//! copy pinned to the world through a turn a document parameter drives.
//!
//! Written the way a user writes an assembly, through the public
//! doors: `Workspace` to hold the documents, `InstantiatePart` to
//! reference a part (an instance IS a placement, and its bundle of
//! constraints says where it sits), values on the world's axes to
//! place one in the world at a pose, a placement over an index to replicate one, `SetConstraints` to seat
//! one part on another by a mate in the mover's bundle with the values
//! that set the freedoms the mate leaves, `SetVarValue` to turn the
//! turntable, `assemble` for the at-rest
//! gate, `split`/`inline` to refactor, `update_to_store` to accept a
//! new version of a part, and `save` / `load` to round-trip. What
//! this stop reports it MEASURED on this run; what refuses, refuses
//! typed and is printed with the recourse the library gave.
//!
//! # The two documents, and why there are two
//!
//! `bench-stand` is the assembled object: two posts and a shelf, the
//! shelf seated on the posts by mates, the near post pinned to the world
//! through a turntable whose swing is the document's `swing` parameter,
//! and a crate resting
//! on the shelf's top face. `bench-layout` is the post
//! and the shelf laid out flat for shipping — two posts on their side
//! (one instance, patterned) and the shelf beside them, nothing
//! touching. Both are real things a user models, and between them they
//! cover the two halves of A5's validity story: the layout is DISJOINT
//! and its at-rest check finds nothing; the stand TOUCHES, and its
//! check finds nothing either, its two flush seats and the crate's rest included
//! (see [`stand_scene`]). Three `SetVarValue`
//! edits on `swing` render three poses, each re-running only the
//! instances the turntable places (see [`poses`]).
//!
//! Every door this file uses is `pncad::…`, the tour's standing
//! invariant: the demos are the façade's acceptance corpus, so a scene
//! that had to reach past it would be evidence about the reach rather
//! than about the library.
//!
//! # The library findings this scene met, and where they live
//!
//! Writing an assembly the way a user would is what turns up what
//! using the library is actually like. Each of these is commented at
//! the site that meets it and filed where it can be fixed:
//!
//! - **#943 / #1063 — CLOSED, and the accommodation retired.** A seat
//!   is a FACE PAIR the census records; it backs the vertex-on-edge and
//!   edge-edge events a flush seat induces, and the pair itself
//!   certifies on the two descriptions' shared world carrier. The stand's posts are seated
//!   FLUSH with the shelf's ends — the obvious way to draw it — where
//!   they had to be inset while the chart-identity door declined every
//!   cross-instance pair (`SEAT_A`).
//! - **#944 — CLOSED.** A mate's pose is read off a FACE, which the
//!   evaluation resolves from the part's own geometry; every seat here
//!   is a face's plane, and `update_door` shows the mate FOLLOWING the
//!   post's cap when the post is shortened — the shelf comes down with
//!   it and the seat still certifies.
//! - **#945** — mates and patterns do not compose at all, which is
//!   why this file has two assembly documents rather than one.
//! - **#947 — the doubled recourse is CLOSED.** The pin-mismatch
//!   recourse now reaches the author exactly once, from the store's
//!   own `Display`; `update_door` is what holds that count down.
//!   All three refusals the walk prints end on a recourse sentence,
//!   each asserted here against the library's own constant
//!   (`refusals`).
//! - **#948** — no parametric loop constructor (`rect`).
//! - **The turntable is one turn about one axis.** Every value charts
//!   on the world's axes, so each copy's values carry the turn: a spin
//!   about the world's vertical by the swing, and slides that are the
//!   copy's spot turned about the pivot — formulas of the swing, so the
//!   swing is a document parameter every bundle reads (`swung`).
//! - **A part resting on another follows a part edit.** The crate's
//!   mate rests its bottom face on the shelf's top FACE, and its values
//!   say where on the top it stands, so a thicker shelf lifts it and
//!   shorter posts lower it, and the at-rest check finds nothing in either
//!   (`update_door`).
//! - **A seat's spin turns about the face it rests on.** A value is
//!   measured on the world's axes, carried onto the mated direction by
//!   the least turn from where the mate's orientation puts the copy's,
//!   and the spin is the turn left about the target face's own outward
//!   normal, whichever side the rest's flip is written on. A seat on an
//!   upward face writes a turn about the world's vertical as it is, and
//!   a seat under a downward face writes it negated (`stand_doc`).
//!
//! The declared direction's frontier — a mated assembly's gate can
//! neither certify nor refute — is not new here; it is the census
//! Door-2 gap steered to M9 on #591, and `at_rest` states what this
//! scene can and cannot say about it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use pncad::prelude::AuthoredNode;
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use std::sync::Arc;

use pncad::document::coincidence::Relation;
use pncad::document::pose::{PoseDef, PoseRole, RoleRead};
use pncad::document::{
    Advisory, BodyRead, CancelToken, CheckEvidence, CheckId, ChecksConfig, Constraint, Coord,
    Dimension, DocEdit, DocRef, DocumentId, EvalOptions, Evaluation, Formula, Frame, FreeValue,
    FreeVar, LoopProgram, Mate, MateFault, MateRef, Node, OVERCONSTRAINED_RECOURSE, Operand,
    Orientation, PartResolver, Placed, ProfileDoc, ProfileProgram, RecipeNodeId, Severity,
    SitedFace, Subject, TargetAxis, UNPINNED_RECOURSE, ValuePayload, VarId, VarName, apply,
    content_pin, evaluate, flatten, inline, load, mixed_pins, parse_formula, product_named,
    product_recorded, run_checks, run_checks_on, save, solve_document, split, stand,
};
use pncad::geom_core::Tol;
use pncad::prelude::StableName;
use pncad::select::{CapEnd, EntityKind, NamePat, NameTable, RoleSeg, SegPat, SegTag, Selector};
use pncad::topo::Body;
use pncad::topo::EulerCounts;
use pncad::topo::readback::euler_counts;
use pncad::workspace::{PIN_MISMATCH_RECOURSE, Workspace, WorkspaceError, update_to_store};

use crate::{SceneBody, TourStop, View};

// ---- The model's dimensions, in metres ----
//
// The kernel's own seat fixtures restate these numbers rather than
// importing them (`topo/tests/m9_c1_rest_face_rung.rs` and
// `editor-core/tests/asm_r2b_assembly.rs`, both `flush_seat`): a demo
// is evidence about the library, so nothing in the library may depend
// on what this scene happens to measure.

/// How far along +x the flat-pack sits from the assembled bench, so
/// the two share ONE montage cell at one camera and one scale. The
/// bench is `SHELF_LENGTH` = 0.9 long, so 1.4 leaves half a metre of
/// clear air.
const FLAT_PACK_GAP: f64 = 1.4;
/// The post's square section and its length.
const POST_SECTION: f64 = 0.12;
const POST_HEIGHT: f64 = 0.5;
/// The shelf's plan size and thickness.
const SHELF_LENGTH: f64 = 0.9;
const SHELF_DEPTH: f64 = 0.30;
const SHELF_THICKNESS: f64 = 0.04;
/// The crate standing on the shelf: its plan size and height.
const CRATE_WIDTH: f64 = 0.24;
const CRATE_DEPTH: f64 = 0.18;
const CRATE_HEIGHT: f64 = 0.14;
/// How far along the shelf, from its centre towards the far post, the
/// crate is set.
const CRATE_SLIDE: f64 = 0.2;

/// The point the stand swings about, in the turntable's own
/// coordinates: the centre of the shelf's plan, on the floor.
const PIVOT: [f64; 3] = [SHELF_LENGTH / 2.0, SHELF_DEPTH / 2.0, 0.0];
/// The three swings the scene edits the turntable to, in degrees.
const SWINGS: [f64; 3] = [30.0, 60.0, 90.0];

/// Where the shelf's underside meets each post, in SHELF coordinates
/// — the two seating points the stand's mates are authored against.
///
/// **FLUSH with the shelf's two ends**, which is the obvious way to
/// draw a bench: each post's outer face is in the plane of the shelf
/// end above it, so the post's cap shares a boundary LINE with the
/// shelf's underside. They stay inset in y, because a bench top
/// overhangs front and back and a post is not the depth of the shelf.
///
/// "Flush" here is flush AS THE MODEL COMPUTES IT, and the two ends are
/// not identical about it: `SEAT_A`'s near face lands on `x = 0`
/// exactly (`POST_SECTION / 2 - POST_SECTION / 2`), while `SEAT_B`'s far
/// face lands 1.11e-16 m past `SHELF_LENGTH`, because
/// `0.9 - 0.06 + 0.06` is not `0.9` in binary floating point. That is
/// one ulp of the model's own coordinates and four orders below the
/// TIGHTEST ε the hosted matrix runs (1e-12), so it is a residue the
/// seat's own predicate certifies rather than a gap in the drawing —
/// which is the whole "certified everywhere within ε, never exact"
/// posture, met by the demo instead of asserted about it. The
/// dimensions are NOT adjusted to make the arithmetic close: moving
/// geometry so a number reads round is the one thing this file may not
/// do.
///
/// This was authored INSET until #1063 landed, with a comment saying
/// flush and inset reached the same verdict — true then, and the
/// reason it was a gap: a declared cross-instance pair was declined at
/// the census's chart-identity door whatever its geometry, so the
/// natural drawing was the one that did not certify. The pair now
/// answers on its shared world carrier, and the shared boundary is
/// carried by the interior-witness rung, so the flush seat certifies
/// and the accommodation is retired.
const SEAT_A: [f64; 3] = [POST_SECTION / 2.0, SHELF_DEPTH / 2.0, 0.0];
const SEAT_B: [f64; 3] = [SHELF_LENGTH - POST_SECTION / 2.0, SHELF_DEPTH / 2.0, 0.0];

/// One post's volume, and the shelf's — the arithmetic every census
/// below is checked against.
const POST_VOLUME: f64 = POST_SECTION * POST_SECTION * POST_HEIGHT;
const SHELF_VOLUME: f64 = SHELF_LENGTH * SHELF_DEPTH * SHELF_THICKNESS;
const CRATE_VOLUME: f64 = CRATE_WIDTH * CRATE_DEPTH * CRATE_HEIGHT;

// ---- Small authoring helpers ----

/// The text expression door, with the document's parameters in scope
/// — the way a user types a dimension (`"section"`, `"120 mm"`).
fn pe(src: &str, params: &BTreeMap<VarName, Dimension>) -> Formula {
    parse_formula(src, params).unwrap_or_else(|e| panic!("expression `{src}`: {e:?}"))
}

/// A mate head, read at the instance the name is qualified by.
///
/// A mate rests two FACES, and the kernel says so in the type: a mate's
/// face side is read through a `SitedFace`, whose
/// names are
/// `pncad::document::FaceName`s. A caller holding a `StableName` from
/// a selection door asks for one and handles the refusal — which for
/// this tour is the same loud panic every other authoring helper here
/// uses, because a demo that named an edge would be a demo with a bug
/// in it.
///
/// **Spelled by full path**, because this binary also uses
/// `tess_meter::FaceName` (`lib.rs`'s face-name tokens): two
/// unrelated types with one short name, and a bare `FaceName` here
/// would be read as either. `tess-meter`'s is a validated text token
/// for a mesh report; this one is a `StableName` whose kind is
/// `Face`.
///
/// The head is the part's own face, read at the instance whose copy
/// carries it: a copy carries its source's names as they are.
fn head(instance: RecipeNodeId, local: &StableName) -> SitedFace {
    let name = pncad::document::FaceName::new(local.clone())
        .unwrap_or_else(|err| panic!("a mate head names a face: {err}"));
    SitedFace::new(instance, name)
}

/// The plane of the face `local` of `instance`'s part, outward normal
/// and all — the pose a mate reads off that face.
fn plane(instance: RecipeNodeId, local: &StableName) -> Operand {
    Operand::Pose(Box::new(PoseDef::Plane {
        face: head(instance, local).into(),
    }))
}

/// `pose`, its sense reversed.
fn flipped(pose: Operand) -> Operand {
    Operand::Pose(Box::new(PoseDef::Flip {
        pose: RoleRead::plain(pose),
    }))
}

/// `plane` slid along its own normal by `by` metres — a standoff.
fn standoff(plane: Operand, by: f64) -> Operand {
    Operand::Pose(Box::new(PoseDef::Slide {
        pose: RoleRead::plain(plane.clone()),
        along: RoleRead::through(plane, PoseRole::Normal),
        by: length(by),
    }))
}

/// A face of the copy resting against a face of what it joins: the
/// copy's plane equated with the other's FLIPPED, so the two outward
/// normals face each other. `on` reads the copy the mate places.
fn rest(on: Operand, against: Operand) -> Mate<Formula> {
    Mate {
        on,
        to: flipped(against),
        orient: Orientation::IDENTITY,
    }
}

/// A length literal, metres.
fn length(metres: f64) -> Formula {
    Formula::literal(metres, Dimension::Length).expect("a finite length")
}

/// Inserts a node and returns its minted id.
///
/// Every node the scenes insert through here — an instance (standing
/// where its part's world stands until its bundle is written), a
/// pattern, a sketch — is admitted on its own datum and reads no part's
/// faces. A mate goes through [`constrain`].
fn insert(doc: &mut ProfileDoc, node: AuthoredNode, tol: Tol) -> RecipeNodeId {
    let applied = apply(
        doc,
        &DocEdit::InsertNode {
            node: Box::new(node),
            fresh: Vec::new(),
        },
        tol,
    )
    .unwrap_or_else(|err| panic!("the insert applies: {err:?}"));
    *doc = applied.doc;
    applied.record.minted.expect("an insert mints an id")
}

/// Writes `mate` and the `values` naming it as `placement`'s bundle,
/// the whole of it in one edit (every bundle pins its copy), and returns
/// the mate's address.
fn constrain(
    doc: &mut ProfileDoc,
    placement: RecipeNodeId,
    mate: Mate<Formula>,
    values: impl IntoIterator<Item = (Coord, Formula)>,
    tol: Tol,
) -> MateRef {
    let placed = Placed {
        mates: vec![mate],
        values: values.into_iter().collect(),
    };
    let applied = apply(doc, &placed.write(placement), tol)
        .unwrap_or_else(|err| panic!("the bundle is admitted: {err:?}"));
    *doc = applied.doc;
    let Some(&id) = applied.record.minted_constraints.first() else {
        panic!("writing a bundle mints its ids")
    };
    MateRef::new(placement, id)
}

/// The edit adding `mate` to `placement`'s bundle beside what it holds.
fn add_constraint(placement: RecipeNodeId, mate: Mate<Formula>) -> DocEdit<ProfileProgram> {
    DocEdit::AddConstraint {
        placement,
        mate: Box::new(mate),
        fresh: Vec::new(),
    }
}

/// A plane mate's three freedoms set: the slides along its chart's `x`
/// and `y` and the spin about its normal.
fn seated_at(x: Formula, y: Formula, spin: Formula) -> [(Coord, Formula); 3] {
    [
        (Coord::Slide(TargetAxis::X), x),
        (Coord::Slide(TargetAxis::Y), y),
        (Coord::SPIN, spin),
    ]
}

/// `placement`'s copy placed in the world by `values` on the world's
/// axes, as written, which read nothing of the copy.
fn place_in_world(
    doc: &mut ProfileDoc,
    placement: RecipeNodeId,
    values: Vec<(Coord, Formula)>,
    tol: Tol,
) {
    edit(doc, &Placed::unmated(values).write(placement), tol);
}

/// Places one copy of `body` in the world where it stands
/// ([`stand`]: the identity bundle, no mate and the constant `0` on
/// every freedom) and returns the placement's id.
fn place(doc: &mut ProfileDoc, body: RecipeNodeId, tol: Tol) -> RecipeNodeId {
    insert(
        doc,
        Node::place(
            BodyRead::plain(body),
            Operand::Var(VarId::WORLD),
            stand().bundle(),
        ),
        tol,
    )
}

/// An instance of `part` standing where its part's world stands, in
/// `space`: the identity bundle, which a gesture then writes over.
fn instance(doc: &mut ProfileDoc, part: DocRef, space: Operand, tol: Tol) -> RecipeNodeId {
    insert(
        doc,
        Node::instantiate_part(part, space, stand().bundle()),
        tol,
    )
}

/// Applies an edit that mints nothing.
fn edit(doc: &mut ProfileDoc, e: &DocEdit<ProfileProgram>, tol: Tol) {
    let applied = apply(doc, e, tol).unwrap_or_else(|err| panic!("edit refused: {err:?}"));
    *doc = applied.doc;
}

/// The evaluation options that carry the store: an evaluation with no
/// resolver refuses every instantiate node rather than pretending a
/// part is empty, so the workspace IS what makes an assembly
/// evaluable.
fn with_store(ws: &Workspace) -> EvalOptions {
    EvalOptions {
        resolver: Some(store(ws)),
        ..EvalOptions::default()
    }
}

/// The workspace as the document seam an evaluation resolves parts
/// through.
fn store(ws: &Workspace) -> Arc<dyn PartResolver> {
    Arc::new(ws.clone())
}

fn run(doc: &ProfileDoc, opts: &EvalOptions, tol: Tol) -> Evaluation<f64> {
    evaluate::<f64>(doc, None, &CancelToken::new(), opts, tol)
}

/// The structural census the A4 acceptance identity compares: shells,
/// faces, edges, vertices of a whole product.
fn census(body: &Body<f64>) -> (i64, i64, i64, i64) {
    let EulerCounts { v, e, f, s, .. } = euler_counts(body);
    (s, f, e, v)
}

/// A product's volume, by bits — the other half of the identity.
fn volume_bits(body: &Body<f64>, tol: Tol) -> u64 {
    pncad::topo::mass_properties(&crate::booleans::at_rest(body, tol), tol, crate::printed())
        .expect("mass properties")
        .enclosure
        .volume
        .to_bits()
}

/// Gathers a document's product with its names — the whole-document
/// A10 gather, which is what "what this document means" is.
fn product_of(doc: &ProfileDoc, ev: &Evaluation<f64>, tol: Tol) -> (Body<f64>, NameTable) {
    product_named(doc, ev, tol).expect("the product gathers")
}

// ---- The part documents ----

/// One extruded prism, parametric in its plan size and its length —
/// the shape both parts are.
///
/// `params` declares the document's named dimensions; `plan` names the
/// two that span the sketch rectangle and `length` the one the extrude
/// consumes. Naming them rather than taking them positionally is what
/// lets the post declare TWO parameters and spend one of them twice —
/// a square section is one dimension, not two.
///
/// The extrusion runs up its own +z from z = 0, so the part's SEATING
/// face is its top cap and its base cap lies on the origin plane.
fn prism_part(
    label: &str,
    params: &[(&'static str, f64)],
    plan: (&str, &str),
    length: &str,
    tol: Tol,
) -> ProfileDoc {
    let mut doc = ProfileDoc::empty(DocumentId::derive(label), tol);
    let mut scope: BTreeMap<VarName, Dimension> = BTreeMap::new();
    for &(name, value) in params {
        let name = VarName::from_static(name);
        edit(
            &mut doc,
            &DocEdit::DeclareVar {
                name: name.clone(),
                def: pncad::document::VarDecl::Free(FreeVar::continuous(Dimension::Length, value)),
            },
            tol,
        );
        scope.insert(name, Dimension::Length);
    }
    let zero = Formula::zero(Dimension::Length);
    // The section, counter-clockwise from the origin corner: the two
    // extents are the document's own named parameters, so the corners
    // are expressions and the loop is built from them directly.
    let (width, height) = (pe(plan.0, &scope), pe(plan.1, &scope));
    let profile = insert(
        &mut doc,
        Node::Profile(ProfileProgram {
            loops: vec![LoopProgram::polygon_expr([
                [zero.clone(), zero.clone()],
                [width.clone(), zero.clone()],
                [width, height.clone()],
                [zero.clone(), height],
            ])],
            ids: Vec::new(),
        }),
        tol,
    );
    let prism = insert(
        &mut doc,
        Node::Extrude {
            profile: profile.into(),
            distance: pe(length, &scope),
        },
        tol,
    );
    let copy = place(&mut doc, prism, tol);
    // What the part makes: its one copy.
    crate::world::set_product(&doc, &[Operand::Node(copy)], tol).doc
}

/// The post: a square section, standing.
fn post_part(tol: Tol) -> ProfileDoc {
    prism_part(
        "pncad-demo-post",
        &[("section", POST_SECTION), ("height", POST_HEIGHT)],
        ("section", "section"),
        "height",
        tol,
    )
}

/// The shelf: a board.
fn shelf_part(tol: Tol) -> ProfileDoc {
    prism_part(
        "pncad-demo-shelf",
        &[
            ("length", SHELF_LENGTH),
            ("depth", SHELF_DEPTH),
            ("thickness", SHELF_THICKNESS),
        ],
        ("length", "depth"),
        "thickness",
        tol,
    )
}

/// The crate: a box set on the shelf.
fn crate_part(tol: Tol) -> ProfileDoc {
    prism_part(
        "pncad-demo-crate",
        &[
            ("width", CRATE_WIDTH),
            ("depth", CRATE_DEPTH),
            ("height", CRATE_HEIGHT),
        ],
        ("width", "depth"),
        "height",
        tol,
    )
}

/// A part's own cap-face name at `end`, as its PRODUCT answers to it —
/// the name a mate on that face refers to, read at the instance: the
/// cap selected on the placed body, which the part's one placement
/// carries as it is.
///
/// Selected structurally (`Cap`, side `end`) rather than hand-built:
/// the naming vocabulary is what a user reaches for, and a selector
/// that stops matching is the library telling you the vocabulary
/// moved.
fn cap_of(doc: &ProfileDoc, end: CapEnd, tol: Tol) -> StableName {
    let ev = run(doc, &EvalOptions::default(), tol);
    let (_, body) = placed_body(doc);
    let sel =
        Selector::of(NamePat::of_kind(EntityKind::Face).seg(SegPat::tag(SegTag::Cap).side(end)));
    let found = pncad::select::select(&ev, body, &sel);
    assert_eq!(
        found.len(),
        1,
        "an extruded prism has exactly one {end:?} cap; got {found:?}"
    );
    found.into_iter().next().expect("checked non-empty")
}

/// The node a `Place` over one shape copies.
fn placed_shape(doc: &ProfileDoc, placement: RecipeNodeId) -> Option<RecipeNodeId> {
    match doc.node(placement) {
        Some(Node::Place { source, .. }) => doc.operation_of(source.read),
        _ => None,
    }
}

/// The one `Place` copying `body`'s output.
fn placement_reading(doc: &ProfileDoc, body: RecipeNodeId) -> RecipeNodeId {
    let reading: Vec<RecipeNodeId> = doc
        .placements()
        .into_iter()
        .filter(|&p| placed_shape(doc, p) == Some(body))
        .collect();
    let [one] = reading[..] else {
        panic!("node {body} has one placement, got {reading:?}")
    };
    one
}

/// A part document's one placement, and the node whose body it places.
fn placed_body(doc: &ProfileDoc) -> (RecipeNodeId, RecipeNodeId) {
    let [placement] = doc.placements()[..] else {
        panic!("a part places one body, got {:?}", doc.placements())
    };
    let node = placed_shape(doc, placement).expect("the placement copies one live body");
    (placement, node)
}

/// Whether `placement`'s bundle places its copy in the world by values
/// alone, no mate reading another copy.
fn on_the_world(doc: &ProfileDoc, placement: RecipeNodeId) -> bool {
    match doc.node(placement) {
        Some(Node::InstantiatePart { constraints, .. } | Node::Place { constraints, .. }) => {
            constraints.iter().all(|c| c.mate().is_none())
        }
        _ => false,
    }
}

/// `placement`'s bundle, in bundle order.
fn bundle(doc: &ProfileDoc, placement: RecipeNodeId) -> Vec<Constraint> {
    match doc.node(placement) {
        Some(Node::InstantiatePart { constraints, .. } | Node::Place { constraints, .. }) => {
            constraints.clone()
        }
        _ => Vec::new(),
    }
}

/// The one body an instance defines: a copy of its part's product.
fn instance_body(ev: &Evaluation<f64>, instance: RecipeNodeId, what: &str, tol: Tol) -> Body<f64> {
    match &ev.value(instance).expect("the instance evaluates").payload {
        ValuePayload::Body(body) => crate::world::in_its_space(body, tol),
        other => panic!("{what} is a {}, not an instance's copy", other.kind_name()),
    }
}

// ---- The assembly documents ----

/// The flat-pack: two posts on their side (ONE instance, patterned)
/// and the shelf laid beside them. Nothing touches.
///
/// The post instance is placed into a `new_space()` of its own: its
/// part's own coordinates, and no part of the product, which is what a
/// pattern master is here. Each patterned copy is placed in the world by its
/// own `Place`, at the post laid on its side; the shelf instance is
/// placed in the world itself. Each is placed by values on the world's
/// axes that put the copy at its pose.
///
/// Every pose carries [`FLAT_PACK_GAP`] along +x, which is how the
/// flat-pack sits BESIDE the assembled bench in their shared montage
/// cell. It is AUTHORED into the placements rather than applied to the
/// gathered body, and that is not a preference: a layout document's
/// placements ARE its subject, so where the parts sit has to be
/// something this document SAYS. What the montage ships is the body
/// `assemble` returned from this document (`layout_scene`); moving
/// that afterwards would hand the renderer a body no gate had seen,
/// standing at coordinates no document records. A common translation
/// in every pose moves the product and changes nothing else about it.
fn layout_doc(post: DocRef, shelf: DocRef, tol: Tol) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let mut doc = ProfileDoc::empty(DocumentId::derive("pncad-demo-layout"), tol);
    let scope = BTreeMap::new();
    let master = insert(&mut doc, Node::Space, tol);
    let post_i = instance(&mut doc, post, Operand::Node(master), tol);
    // The post laid on its side. The pattern steps along +y, which this
    // turn about +y leaves where it was, so each copy is placed at the
    // same pose.
    let lying = || {
        let mut values = vec![(
            Coord::Spin(Some(TargetAxis::Y)),
            crate::world::turn(4, true),
        )];
        values.extend(crate::world::slides([
            FLAT_PACK_GAP + POST_HEIGHT,
            0.0,
            0.0,
        ]));
        values
    };
    // The row: `k = index(2)`, post `k` slid `scalar(k)·200 mm` along
    // the master space's +y.
    let k = VarName::new("k").expect("an index name");
    edit(
        &mut doc,
        &pncad::pattern::index_decl(k.clone(), pe("2", &scope), None),
        tol,
    );
    let master_space = doc
        .node_space(post_i)
        .expect("the post is in its master space");
    let pattern = insert(
        &mut doc,
        pncad::pattern::linear_pattern(
            &k,
            BodyRead::plain(Operand::Node(post_i)),
            Operand::Var(master_space),
            &[(TargetAxis::Y, pe("200 mm", &scope))],
        )
        .expect("the pitch is a length"),
        tol,
    );
    // The world: each member of the row, laid down — a copy of the one
    // indexed read `pattern[i]`.
    let mut posts = Vec::with_capacity(2);
    for i in 0..2 {
        let placement = insert(
            &mut doc,
            Node::place(
                BodyRead::indexed(pattern, vec![Formula::count(i)]),
                Operand::Var(VarId::WORLD),
                stand().bundle(),
            ),
            tol,
        );
        place_in_world(&mut doc, placement, lying(), tol);
        posts.push(placement);
    }
    // The shelf, placed in the world beside them: the last placement,
    // as the instance a split of it leaves behind is.
    let shelf_i = instance(&mut doc, shelf, Operand::Var(VarId::WORLD), tol);
    place_in_world(
        &mut doc,
        shelf_i,
        crate::world::slides([FLAT_PACK_GAP, 0.9, 0.0]),
        tol,
    );
    // What the flat-pack is: its two posts and its shelf.
    let made: Vec<Operand> = posts
        .into_iter()
        .chain([shelf_i])
        .map(Operand::Node)
        .collect();
    let doc = crate::world::set_product(&doc, &made, tol).doc;
    (doc, pattern, shelf_i)
}

/// The stand: a post at each end of the shelf, the shelf SEATED on
/// them by mates, the three turning on a turntable whose swing is the
/// document's `swing` parameter. Only the near post is pinned to the
/// world — its pin's world frame is its spot on the turntable, spun
/// about the turntable's axis by the swing — and each other copy's
/// bundle is the one mate that places it and the values that set the
/// freedoms the mate leaves: the shelf on the near post, the far post
/// under the shelf.
struct Stand {
    doc: ProfileDoc,
    post_a: RecipeNodeId,
    shelf_i: RecipeNodeId,
    post_b: RecipeNodeId,
    /// The shelf's mate onto the near post, in the shelf's bundle.
    mate_1: MateRef,
    /// The far post's mate under the shelf, in the far post's bundle.
    mate_2: MateRef,
    /// Where the shelf's origin stands at swing zero, on the world's
    /// `x` and `y`.
    shelf_at: [f64; 2],
}

/// The document parameter the turntable's swing reads.
fn swing() -> VarName {
    VarName::from_static("swing")
}

/// **`spot`, on the turntable, where the swing turns it**: the point
/// turned about [`PIVOT`] by the document's `swing`, as two formulas on
/// the world's `x` and `y` — `p + R(swing)·(spot − p)`.
fn swung(spot: [f64; 2], scope: &BTreeMap<VarName, Dimension>) -> [Formula; 2] {
    let [px, py, _] = PIVOT;
    let (dx, dy) = (spot[0] - px, spot[1] - py);
    let trig = |f: fn(Formula) -> Result<Formula, pncad::document::DimensionError>| {
        f(pe("swing", scope)).expect("an angle's ratio")
    };
    let times = |metres: f64, ratio: Formula| {
        Formula::mul(length(metres), ratio).expect("a length times a ratio")
    };
    let add = |a: Formula, b: Formula| Formula::add(a, b).expect("lengths add");
    let sub = |a: Formula, b: Formula| Formula::sub(a, b).expect("lengths subtract");
    [
        add(
            length(px),
            sub(times(dx, trig(Formula::cos)), times(dy, trig(Formula::sin))),
        ),
        add(
            length(py),
            add(times(dx, trig(Formula::sin)), times(dy, trig(Formula::cos))),
        ),
    ]
}

/// **The near post on the turntable**: values on the world's axes, its
/// own frame at `spot` turned about the turntable's axis by `swing` —
/// the spin about the world's vertical, and the slides that carry its
/// origin to `spot` turned about [`PIVOT`] ([`swung`]). The swing is a
/// formula inside the values, so it stays a variable inside the pin.
fn on_the_turntable(spot: [f64; 3], scope: &BTreeMap<VarName, Dimension>) -> Placed {
    let [x, y] = swung([spot[0], spot[1]], scope);
    Placed::unmated(vec![
        (Coord::Spin(Some(TargetAxis::Z)), pe("swing", scope)),
        (Coord::Slide(TargetAxis::X), x),
        (Coord::Slide(TargetAxis::Y), y),
        (Coord::Slide(TargetAxis::Z), length(spot[2])),
    ])
}

/// Builds the stand.
///
/// The seats' values are where the posts meet the shelf, [`SEAT_A`]
/// and [`SEAT_B`]: each copy's origin at swing zero, turned about the
/// pivot by the swing ([`swung`]), on the world's axes, and its spin the
/// swing about the target face's own normal. The shelf's seat reads the
/// near post's top, which faces up, so its spin is the swing; the far
/// post's reads the shelf's underside, which faces down, so a turn by
/// the swing about the world's `+z` is a spin of the negated swing.
fn stand_doc(parts: &Parts, tol: Tol) -> Stand {
    let (post, shelf) = (parts.post, parts.shelf);
    let (post_top, shelf_bottom) = (&parts.post_top, &parts.shelf_bottom);
    let mut doc = ProfileDoc::empty(DocumentId::derive("pncad-demo-stand"), tol);
    edit(
        &mut doc,
        &DocEdit::DeclareVar {
            name: swing(),
            def: pncad::document::VarDecl::Free(FreeVar::continuous(Dimension::Angle, 0.0)),
        },
        tol,
    );
    let scope = BTreeMap::from([(swing(), Dimension::Angle)]);
    let spot = [0.0, (SHELF_DEPTH - POST_SECTION) / 2.0, 0.0];
    let world = || Operand::Var(VarId::WORLD);
    let post_a = instance(&mut doc, post, world(), tol);
    edit(&mut doc, &on_the_turntable(spot, &scope).write(post_a), tol);
    let shelf_i = instance(&mut doc, shelf, world(), tol);
    let post_b = instance(&mut doc, post, world(), tol);

    // Each mate names the part it moves first and goes in that part's
    // bundle: "mate the shelf to the post" places the shelf on the
    // pinned post, and the far post under the placed shelf. Each side
    // is its head face's plane, read off the part's own evaluation, so
    // no number here can disagree with the model.
    let half = POST_SECTION / 2.0;
    // Each copy's origin at swing zero, in the world.
    let shelf_at = [spot[0] + half - SEAT_A[0], spot[1] + half - SEAT_A[1]];
    let post_b_at = [
        shelf_at[0] + SEAT_B[0] - half,
        shelf_at[1] + SEAT_B[1] - half,
    ];
    let on_top = |at: [f64; 2]| {
        let [x, y] = swung(at, &scope);
        seated_at(x, y, pe("swing", &scope))
    };
    // A seat under a downward face spins about its normal, the world's
    // `-z`: a turn about the world's `+z` is a spin of the negated swing.
    let under = |at: [f64; 2]| {
        let [x, y] = swung(at, &scope);
        seated_at(x, y, pe("-swing", &scope))
    };
    let mate_1 = constrain(
        &mut doc,
        shelf_i,
        rest(plane(shelf_i, shelf_bottom), plane(post_a, post_top)),
        on_top(shelf_at),
        tol,
    );
    let mate_2 = constrain(
        &mut doc,
        post_b,
        rest(plane(post_b, post_top), plane(shelf_i, shelf_bottom)),
        under(post_b_at),
        tol,
    );
    // What the stand is: its two posts and its shelf.
    let made = [post_a, shelf_i, post_b].map(Operand::Node);
    let doc = crate::world::set_product(&doc, &made, tol).doc;
    Stand {
        doc,
        post_a,
        shelf_i,
        post_b,
        mate_1,
        mate_2,
        shelf_at,
    }
}

/// The bench as it ships: the stand, and a crate set on its shelf.
struct Bench {
    doc: ProfileDoc,
    crate_i: RecipeNodeId,
    crate_mate: MateRef,
}

/// Sets a crate on the stand's shelf.
///
/// The crate rests on the shelf's top face, by a placing `Rest` mate —
/// the crate's bottom face against the shelf's top — and the values
/// that set where on the top it stands: [`CRATE_SLIDE`] along the
/// shelf from its centre, squared to it — its origin at swing zero
/// turned about the pivot by the swing, on the world's axes. The crate is
/// an instance whose bundle is that mate and its values, so it is
/// placed on the shelf: it swings with the stand and follows any
/// edit of the shelf or the posts under it.
fn bench(stand: &Stand, parts: &Parts, tol: Tol) -> Bench {
    let mut doc = stand.doc.clone();
    let crate_i = instance(&mut doc, parts.crate_ref, Operand::Var(VarId::WORLD), tol);
    let scope = BTreeMap::from([(swing(), Dimension::Angle)]);
    let [x, y] = swung(
        [
            stand.shelf_at[0] + (SHELF_LENGTH - CRATE_WIDTH) / 2.0 + CRATE_SLIDE,
            stand.shelf_at[1] + (SHELF_DEPTH - CRATE_DEPTH) / 2.0,
        ],
        &scope,
    );
    let crate_mate = constrain(
        &mut doc,
        crate_i,
        rest(
            plane(crate_i, &parts.crate_bottom),
            plane(stand.shelf_i, &parts.shelf_top),
        ),
        seated_at(x, y, pe("swing", &scope)),
        tol,
    );
    // The stand's product, the crate with it.
    let made = [stand.post_a, stand.shelf_i, stand.post_b, crate_i].map(Operand::Node);
    let doc = crate::world::replace_product(&doc, &made, tol).doc;
    Bench {
        doc,
        crate_i,
        crate_mate,
    }
}

// ---- The workspace ----

/// The store, and everything about the parts an assembly author needs
/// after writing them: each part's reference — the `(id, pin)` pair an
/// assembly carries (A4: the id answers "which part", the pin "which
/// version of it") — and the cap-face names its mates refer to.
///
/// The names are taken HERE, from the documents this function just
/// built, because that is the only place they exist already: naming a
/// face means evaluating the part, and re-deriving the part document
/// somewhere else to do it would be authoring it twice.
struct Parts {
    post: DocRef,
    shelf: DocRef,
    crate_ref: DocRef,
    /// The post's top cap, in the post's own names.
    post_top: StableName,
    /// The shelf's underside, in the shelf's own names.
    shelf_bottom: StableName,
    /// The shelf's top face, which the crate is set on.
    shelf_top: StableName,
    /// The crate's underside.
    crate_bottom: StableName,
}

/// Writes the three part documents into a fresh workspace directory
/// and opens it.
fn workspace(dir: &Path, tol: Tol) -> (Workspace, Parts) {
    // A demo re-runs; a store that accumulated yesterday's documents
    // would resolve a pin nobody wrote today.
    if dir.exists() {
        std::fs::remove_dir_all(dir).expect("clear the demo workspace");
    }
    std::fs::create_dir_all(dir).expect("create the demo workspace");
    let mut ws = Workspace::open(dir).expect("the empty workspace opens");
    let post = post_part(tol);
    let shelf = shelf_part(tol);
    let crate_doc = crate_part(tol);
    ws.create(&post, tol).expect("write the post document");
    ws.create(&shelf, tol).expect("write the shelf document");
    ws.create(&crate_doc, tol)
        .expect("write the crate document");
    let reference = |d: &ProfileDoc| DocRef {
        id: d.id(),
        pin: content_pin(d, tol).expect("the pin computes"),
    };
    let parts = Parts {
        post: reference(&post),
        shelf: reference(&shelf),
        crate_ref: reference(&crate_doc),
        post_top: cap_of(&post, CapEnd::End, tol),
        shelf_bottom: cap_of(&shelf, CapEnd::Start, tol),
        shelf_top: cap_of(&shelf, CapEnd::End, tol),
        crate_bottom: cap_of(&crate_doc, CapEnd::Start, tol),
    };
    (ws, parts)
}

// ---- The scenes ----

/// The flat-pack layout: three disjoint solids, and A5's disjoint half
/// — the at-rest check finds nothing.
///
/// TWO posts, the same count the bench assembles, so "the same parts,
/// flat-packed" is true of the PARTS and not only of the documents. Two
/// is also what the name lookup below needs: it asks for member 1, the
/// first member slid off the master — member 0 stands where its source
/// does, so k = 0 need not exercise a slide at all, and k = 1 is the
/// first that must.
fn layout_scene(ws: &Workspace, doc: &ProfileDoc, pattern: RecipeNodeId, tol: Tol) -> SceneBody {
    let ev = run(doc, &with_store(ws), tol);
    let (body, names) = product_of(doc, &ev, tol);

    assert_eq!(
        body.shells().count(),
        3,
        "one instance, patterned twice, plus the shelf"
    );
    let want = 2.0 * POST_VOLUME + SHELF_VOLUME;
    let props =
        pncad::topo::mass_properties(&crate::booleans::at_rest(&body, tol), tol, crate::printed())
            .expect("mass properties");
    assert!(
        (props.enclosure.volume - want).abs() < 1e-12,
        "the layout gathers {want} m^3 of material, measured {}",
        props.enclosure.volume
    );
    println!(
        "   [layout] {} instantiated solid(s) from 2 part documents; V = {:.6} m^3 \
         (2 x post + shelf, exact); {} product names",
        body.shells().count(),
        props.enclosure.volume,
        names.iter().count()
    );

    // A name lookup a user actually does: "where is the second
    // patterned post's end face?" The answer is one member-qualified
    // name away — the family's `Member(k = 1)` segment wrapping the
    // part's own cap name (D10 `[repetition]`).
    // The name NESTS rather than concatenating: `Member` carries the
    // member's own name as its argument, and a copy carries its
    // source's names as they are, so the member's name is the part's
    // product name, which is the part's cap in the one copy its
    // placement puts in the part's world. So the family reads as one
    // wrapper deep — the index's value, then the part's own cap.
    let face = || NamePat::of_kind(EntityKind::Face);
    let cap_of_part = face().seg(SegPat::tag(SegTag::Cap).side(CapEnd::End));
    let caps = pncad::select::select(
        &ev,
        pattern,
        &Selector::of(face().seg(SegPat::tag(SegTag::Member).of([cap_of_part]))),
    );
    let indexed: Vec<&StableName> = caps
        .iter()
        .filter(|n| {
            matches!(n.path.first(), Some(RoleSeg::Member { at, .. }) if at.iter().map(|&(_, v)| v).eq([1]))
        })
        .collect();
    assert_eq!(
        indexed.len(),
        1,
        "member 1 has exactly one post top cap: {caps:?}"
    );
    let pose = pncad::select::face_frame(&ev, pattern, indexed[0])
        .expect("the named face answers with its frame");
    println!(
        "   [layout] name lookup: member 1's post cap sits at \
         ({:.3}, {:.3}, {:.3}) m",
        pose.origin.x, pose.origin.y, pose.origin.z
    );
    // One 200 mm pattern step along +y puts member 1's post between
    // y = 0.2 and y = 0.2 + section; its cap frame's origin lies on
    // that face.
    assert!(
        (0.2..=0.2 + POST_SECTION).contains(&pose.origin.y),
        "the second post is one 200 mm step along +y, measured {}",
        pose.origin.y
    );

    // A5's disjoint half: the at-rest check over the gathered product.
    // Nothing touches, so the census finds nothing and records nothing.
    let assembly = at_rest(doc, &ev, tol);
    assert!(
        assembly.contacts.is_empty(),
        "a disjoint assembly records no contact"
    );
    println!("   [layout] A5 at-rest check: nothing found (disjoint, 0 contacts)");

    SceneBody::seamed(
        "benchlayout",
        [0.62, 0.51, 0.36],
        assembly.body,
        assembly.contacts,
    )
}

/// The assembled bench at its authored swing: the near post's world
/// mate places it through the turntable, the mates place the rest onto
/// it, the census records the seats, and the at-rest check finds nothing.
fn stand_scene(ws: &Workspace, stand: &Stand, bench: &Bench, tol: Tol) -> Evaluation<f64> {
    let doc = &bench.doc;
    let ev = run(doc, &with_store(ws), tol);

    // The solve, read the way an author reads it: every copy is in the
    // world's space — the near post by its pin, the rest because each
    // bundle's one mate pins it onto a copy that is — and no bundle
    // records a fault.
    let poses = solve_document(doc, &with_store(ws), tol);
    for (copy, what) in [
        (stand.post_a, "the near post"),
        (stand.shelf_i, "the shelf"),
        (stand.post_b, "the far post"),
        (bench.crate_i, "the crate"),
    ] {
        assert!(
            poses.fault(copy).is_none(),
            "{what}'s bundle records no fault: {:?}",
            poses.fault(copy)
        );
        assert_eq!(
            poses.space(copy),
            Some(VarId::WORLD),
            "{what} is in the world's space"
        );
    }
    for (mate, what) in [
        (stand.mate_1, "the shelf"),
        (stand.mate_2, "the far post"),
        (bench.crate_mate, "the crate"),
    ] {
        let mates: Vec<_> = bundle(doc, mate.placement)
            .into_iter()
            .filter(|c| c.mate().is_some())
            .map(|c| c.id)
            .collect();
        assert_eq!(
            mates,
            vec![mate.id],
            "{what}'s bundle holds the one mate that places it"
        );
        assert_eq!(
            bundle(doc, mate.placement)
                .iter()
                .filter(|c| c.value().is_some())
                .count(),
            3,
            "{what}'s values each set a freedom that mate leaves"
        );
        assert!(
            !on_the_world(doc, mate.placement),
            "{what} is placed by its mate, not pinned to the world"
        );
    }
    assert!(
        on_the_world(doc, stand.post_a),
        "the near post is pinned to the world on the turntable, by values alone"
    );
    println!(
        "   [stand] 4 instances in the world: node {} pinned to it on the turntable, and 3 \
         mates each in the bundle of the copy it places, its freedoms set by values — the \
         shelf on the near post, the far post under the shelf, the crate on the shelf's top \
         face",
        stand.post_a,
    );
    placed_as_composed(ws, stand, bench, &ev, 0.0, tol);

    let (gathered, _) = product_of(doc, &ev, tol);
    assert_eq!(
        gathered.shells().count(),
        4,
        "two posts, a shelf and a crate"
    );
    let props = pncad::topo::mass_properties(
        &crate::booleans::at_rest(&gathered, tol),
        tol,
        crate::printed(),
    )
    .expect("mass properties");
    assert!(
        (props.enclosure.volume - BENCH_VOLUME).abs() < 1e-12,
        "the bench gathers {BENCH_VOLUME} m^3, measured {}",
        props.enclosure.volume
    );

    // ASSERTED, not merely printed: the stand is the natural drawing of
    // a bench — two posts seated FLUSH with the shelf's ends — with a
    // crate resting on its shelf; it certifies.
    let gate = at_rest(doc, &ev, tol);
    let seats = gate.contacts.face_face().count();
    println!(
        "   [stand] the census recorded {seats} seat(s) between the copies, at FACE \
         granularity; A5 at-rest check: nothing found"
    );
    assert_eq!(seats, 3, "one record per seat the mates make");
    // Each seat is the at-rest census's row of its mate's two faces,
    // which the mate states are one construction: the coincidence door
    // proves all three. Each post's outer face is in the plane of the
    // shelf's end, facing the same way: two flush pairs, each a row of its
    // own, which no mate states, so the lint reports those two and only
    // those (flush by number is reported; a flag mate is what states one).
    let report = run_checks(doc, &ev, &ChecksConfig::default(), tol).expect("the checks run");
    let unproven: Vec<Relation> = report
        .findings
        .iter()
        .filter_map(|finding| match &finding.evidence {
            CheckEvidence::UnprovenCoincidence { row, .. }
                if finding.check == CheckId::UnprovenCoincidence =>
            {
                Some(row.relation)
            }
            _ => None,
        })
        .collect();
    println!(
        "   [stand] unproven-coincidence: {} finding(s), the posts' ends flush with the \
         shelf's; each seat is its mate's statement",
        unproven.len()
    );
    assert_eq!(
        unproven,
        [Relation::SameOriented; 2],
        "every mated seat is proven, and the two flush ends are reported: {report}"
    );
    ev
}

/// The bench's material: two posts, a shelf and a crate.
const BENCH_VOLUME: f64 = 2.0 * POST_VOLUME + SHELF_VOLUME + CRATE_VOLUME;

/// **The three poses**: one `SetVarValue` on `swing` each, every
/// evaluation fed the one before it as its memo.
///
/// A swing re-runs the four instances the turntable places, and
/// nothing else. Each pose is checked against the chain composed here,
/// its volume
/// against the bench's material, and its at-rest check is run again.
fn poses(
    ws: &Workspace,
    stand: &Stand,
    bench: &Bench,
    authored: Evaluation<f64>,
    tol: Tol,
) -> Vec<(f64, AtRest, String)> {
    let mut doc = bench.doc.clone();
    let mut prior = authored;
    let mut out = Vec::new();
    for degrees in SWINGS {
        edit(
            &mut doc,
            &DocEdit::SetVarValue {
                var: swing().into(),
                value: FreeValue::Continuous(degrees.to_radians()),
            },
            tol,
        );
        let ev = evaluate::<f64>(
            &doc,
            Some(&prior),
            &CancelToken::new(),
            &with_store(ws),
            tol,
        );
        let counters = format!(
            "swing -> {degrees} deg: recomputed {} node(s), reused {}",
            ev.recomputed, ev.reused
        );
        println!("   [pose] {counters}");
        // WHICH nodes re-ran, not only how many: the ones whose content
        // key the edit moved. The near post's pin reads `swing`,
        // and a placement's key carries the copies its bundle's mates
        // read (D10), so the shelf, the far post and the crate re-key
        // with the post they stand on, and the product's `⊔` with the
        // four it reads.
        let product = doc
            .product()
            .and_then(|var| doc.operation_of(var))
            .expect("the bench sets its product");
        let moved: BTreeSet<RecipeNodeId> = doc
            .ids()
            .iter()
            .copied()
            .filter(|&id| {
                ev.value(id).map(|v| v.content_key) != prior.value(id).map(|v| v.content_key)
            })
            .collect();
        assert_eq!(
            moved,
            BTreeSet::from([
                stand.post_a,
                stand.shelf_i,
                stand.post_b,
                bench.crate_i,
                product
            ]),
            "a swing re-keys the four instances on the turntable and the product, and nothing else"
        );
        assert_eq!(
            (ev.recomputed, ev.reused),
            (moved.len(), doc.ids().len() - moved.len()),
            "what re-keyed re-ran, and everything else was reused"
        );
        placed_as_composed(ws, stand, bench, &ev, degrees.to_radians(), tol);
        far_post_solved(&doc, stand, &with_store(ws), degrees.to_radians(), tol);
        shelf_stays_on_the_pivot(&ev, stand, degrees, tol);
        let (gathered, _) = product_of(&doc, &ev, tol);
        let volume = pncad::topo::mass_properties(
            &crate::booleans::at_rest(&gathered, tol),
            tol,
            crate::printed(),
        )
        .expect("mass properties")
        .enclosure
        .volume;
        assert!(
            (volume - BENCH_VOLUME).abs() < 1e-12,
            "a swing moves material and makes none: {BENCH_VOLUME} m^3, measured {volume}"
        );
        let gate = at_rest(&doc, &ev, tol);
        out.push((degrees, gate, counters));
        prior = ev;
    }
    out
}

/// **The oracle**: every vertex of every placed body is its part's own
/// vertex carried by the chain composed HERE — the turntable's turn by
/// `swing` about [`PIVOT`], after the instance's pose on the turntable
/// — never read back from the kernel.
///
/// Each pose on the turntable is a translation stated by the model's
/// dimensions: the near post's authored spot, the shelf `POST_HEIGHT`
/// up on it, the far post one shelf length minus a section along, and
/// the crate on the shelf's top, [`CRATE_SLIDE`] along it from its
/// centre.
fn placed_as_composed(
    ws: &Workspace,
    stand: &Stand,
    bench: &Bench,
    ev: &Evaluation<f64>,
    swing: f64,
    tol: Tol,
) {
    let inset = (SHELF_DEPTH - POST_SECTION) / 2.0;
    let (sin, cos) = swing.sin_cos();
    let turn = |p: [f64; 3]| {
        let (x, y) = (p[0] - PIVOT[0], p[1] - PIVOT[1]);
        [
            PIVOT[0] + cos * x - sin * y,
            PIVOT[1] + sin * x + cos * y,
            p[2],
        ]
    };
    for (instance, what, at) in [
        (stand.post_a, "the near post", [0.0, inset, 0.0]),
        (stand.shelf_i, "the shelf", [0.0, 0.0, POST_HEIGHT]),
        (
            stand.post_b,
            "the far post",
            [SHELF_LENGTH - POST_SECTION, inset, 0.0],
        ),
        (
            bench.crate_i,
            "the crate",
            [
                (SHELF_LENGTH - CRATE_WIDTH) / 2.0 + CRATE_SLIDE,
                (SHELF_DEPTH - CRATE_DEPTH) / 2.0,
                POST_HEIGHT + SHELF_THICKNESS,
            ],
        ),
    ] {
        let Some(Node::InstantiatePart { doc_ref, .. }) = bench.doc.node(instance) else {
            panic!("{what} is an instance");
        };
        let part = ws.resolve(doc_ref, tol).expect("the part resolves");
        let own = points(&part_product_body(
            &run(&part, &EvalOptions::default(), tol),
            &part,
            tol,
        ));
        let placed = points(&instance_body(ev, instance, what, tol));
        assert_eq!(
            own.len(),
            placed.len(),
            "{what}: one placed vertex per part vertex"
        );
        for (p, q) in own.iter().zip(&placed) {
            let want = turn([p[0] + at[0], p[1] + at[1], p[2] + at[2]]);
            assert!(
                want.iter().zip(q).all(|(w, q)| (w - q).abs() < 1e-12),
                "{what} at a swing of {swing} rad: the chain puts {p:?} at {want:?}, \
                 evaluated {q:?}"
            );
        }
    }
}

/// Where the mates put the far post, read through the solve's own
/// world-pose door: SOLVED, composed outward from the near post's pin
/// along the mates, never stored.
///
/// The translation is the far post's pose in the turntable, one shelf
/// length minus a section along, turned about [`PIVOT`]. The rotation
/// is the half a translation check cannot see: each seat's spin is the
/// swing about its face's own normal, so the post turns by the swing and
/// no more — a solve that
/// rotated it further and still landed its seating point would pass
/// the translation check and put the part in sideways.
fn far_post_solved(doc: &ProfileDoc, stand: &Stand, opts: &EvalOptions, swing: f64, tol: Tol) {
    let solved = solve_document(doc, opts, tol)
        .placement(doc, stand.post_b)
        .expect("the far post is placed");
    let (sin, cos) = swing.sin_cos();
    let (x, y) = (
        SEAT_B[0] - SEAT_A[0] - PIVOT[0],
        (SHELF_DEPTH - POST_SECTION) / 2.0 - PIVOT[1],
    );
    let translation = [
        PIVOT[0] + cos * x - sin * y,
        PIVOT[1] + sin * x + cos * y,
        0.0,
    ];
    let columns = [[cos, sin, 0.0], [-sin, cos, 0.0], [0.0, 0.0, 1.0]];
    let close = |a: &[f64; 3], b: &[f64; 3]| a.iter().zip(b).all(|(a, b)| (a - b).abs() < 1e-12);
    assert!(
        close(&solved.translation, &translation),
        "the far post's solved translation at a swing of {swing} rad is {:?}, expected \
         {translation:?}",
        solved.translation
    );
    assert!(
        solved
            .columns
            .iter()
            .zip(&columns)
            .all(|(a, b)| close(a, b)),
        "the far post turns by the swing and no more: {:?}, expected {columns:?}",
        solved.columns
    );
}

/// The turntable turns the bench about the shelf's centre, which is
/// the point of the pivot: the mean of the shelf's eight corners — its
/// centroid in plan — stays on [`PIVOT`] at every swing.
fn shelf_stays_on_the_pivot(ev: &Evaluation<f64>, stand: &Stand, degrees: f64, tol: Tol) {
    let corners = points(&instance_body(ev, stand.shelf_i, "the shelf", tol));
    let n = corners.len() as f64;
    let centre = [0, 1].map(|k| corners.iter().map(|p| p[k]).sum::<f64>() / n);
    assert!(
        (centre[0] - PIVOT[0]).abs() < 1e-12 && (centre[1] - PIVOT[1]).abs() < 1e-12,
        "the shelf's centre stays on the pivot at {degrees} deg: {centre:?} vs {PIVOT:?}"
    );
}

/// A part document's product body: its one placement's value.
fn part_product_body(ev: &Evaluation<f64>, part: &ProfileDoc, tol: Tol) -> Body<f64> {
    let (placement, _) = placed_body(part);
    match &ev.value(placement).expect("the part evaluates").payload {
        ValuePayload::Body(body) => crate::world::in_its_space(body, tol),
        other => panic!("a part's placement is a {}, not a body", other.kind_name()),
    }
}

/// A body's points, in arena order.
fn points(body: &Body<f64>) -> Vec<[f64; 3]> {
    body.points().map(|(_, p)| [p.x, p.y, p.z]).collect()
}

/// The product at rest and its record set.
struct AtRest {
    body: Body<f64>,
    contacts: pncad::topo::ContactRecords,
}

/// Gathers a stand the scene means to be at rest and runs the at-rest
/// check over it, and panics on any finding: every caller (the stand,
/// the instanced stand, and the updated bench the update walk re-seats)
/// asserts it is at rest.
fn at_rest(doc: &ProfileDoc, ev: &Evaluation<f64>, tol: Tol) -> AtRest {
    let product = product_recorded(doc, ev, tol)
        .unwrap_or_else(|e| panic!("the stand gathers: {}", e.spoken(doc)));
    let at_rest_only = ChecksConfig {
        connectedness: Severity::Off,
        unproven_coincidence: Advisory::Off,
        ..ChecksConfig::default()
    };
    let report = run_checks_on(doc, ev, Subject::Product(&product), &at_rest_only, tol)
        .unwrap_or_else(|e| panic!("the at-rest check runs: {}", e.spoken(doc)));
    assert!(
        report.findings.is_empty(),
        "the stand must be at rest: {}",
        report.spoken(doc)
    );
    AtRest {
        body: product.body.into_body(),
        contacts: product.contacts,
    }
}

// ---- The refusal walk ----

/// Everything v1 does NOT do, met the way an author meets it and
/// printed with the recourse the library itself gave.
///
/// Fail-loud is the design, so each of these is EVIDENCE: a refusal
/// that stopped being typed, stopped naming its subject, or stopped
/// ending on its recourse breaks this walk.
fn refusals(ws: &Workspace, parts: &Parts, tol: Tol) {
    let (post, shelf) = (parts.post, parts.shelf);
    println!("\n-- the v1 boundary, walked: three refusals an author actually hits --");

    // (1) UNDER-DETERMINED. One planar rest between two parts fixes
    // the seating plane and nothing else — the shelf could still slide
    // and spin in it. Every bundle pins its copy, so a bundle that pins
    // less is refused at the door, naming the freedoms it leaves, and
    // the recourse is the values that set them.
    let under = stand_doc(parts, tol);
    let alone = Placed {
        mates: vec![rest(
            plane(under.shelf_i, &parts.shelf_bottom),
            plane(under.post_a, &parts.post_top),
        )],
        values: Vec::new(),
    };
    let fault = match apply(&under.doc, &alone.write(under.shelf_i), tol) {
        Err(pncad::document::EditError::MateRefused { fault, .. }) => *fault,
        Err(other) => panic!("a lone rest refuses as the bundle's own fault, got {other:?}"),
        Ok(_) => panic!("the door admitted a rest that leaves the shelf free in its plane"),
    };
    assert!(
        matches!(fault, MateFault::Unpinned { free: 3, .. }),
        "a planar rest alone is the UNPINNED refusal, got {fault:?}"
    );
    assert!(
        fault.to_string().contains(UNPINNED_RECOURSE),
        "the refusal carries its recourse verbatim"
    );
    println!("   (1) under-determined, at the door: {fault}");

    // (2) OVERCONSTRAINED. A second rest in ONE bundle fixes the
    // seating plane the first already fixes, whatever its standoff: the
    // door refuses it, measuring nothing, naming both mates and the
    // freedoms the two would leave. Where two parts meet beyond what
    // their mates fix, the recourse is an assertion, never a mate that
    // checks.
    let again = rest(
        plane(under.shelf_i, &parts.shelf_bottom),
        // The same pair, seated 10 mm apart.
        standoff(plane(under.post_a, &parts.post_top), 0.01),
    );
    let fault = match apply(&under.doc, &add_constraint(under.shelf_i, again), tol) {
        Err(pncad::document::EditError::MateRefused { fault, .. }) => *fault,
        Err(other) => panic!("a second rest refuses as the mate's own fault, got {other:?}"),
        Ok(_) => panic!("the door admitted a second rest beside the first"),
    };
    assert!(
        matches!(fault, MateFault::Overconstrained { .. }),
        "a rest beside a rest is the OVERCONSTRAINED refusal, got {fault:?}"
    );
    assert!(
        fault.to_string().contains(OVERCONSTRAINED_RECOURSE),
        "the refusal carries its recourse verbatim"
    );
    println!("   (2) overconstrained, at the door: {fault}");

    // (3) A REFERENCE PINNING THE WRONG VERSION. An assembly is a
    // self-contained reproducible value: a part that changed on disk
    // does not retarget it. Asking the store for a version it no
    // longer holds is the typed pin refusal, and its recourse names
    // the edit that legitimately moves a pin.
    //
    // THE TRIGGER HERE IS HAND-BUILT, and no user builds one: pairing
    // one document's id with another's pin is a `DocRef` nobody would
    // author. It is here because it reaches the refusal in one line
    // with no store mutation, so the walk can state the refusal's
    // shape beside the other two. The USER-REACHABLE instance of the
    // same refusal is in `update_door`, where a part legitimately
    // changes on disk and the un-updated assembly stops resolving —
    // that is the one that says what this costs an author.
    let stale = DocRef {
        id: post.id,
        pin: shelf.pin,
    };
    let err = ws
        .resolve(&stale, tol)
        .expect_err("a pin the store does not hold cannot resolve");
    assert!(
        matches!(err, WorkspaceError::PinMismatch { .. }),
        "a moved pin is PinMismatch, got {err}"
    );
    assert!(
        err.to_string().contains(PIN_MISMATCH_RECOURSE),
        "the refusal carries its recourse verbatim"
    );
    println!("   (3) wrong pin: {err}");
}

// ---- The recorded refactorings (A4) ----

/// Splits the shelf's cell out of the layout into its own document and
/// inlines it back, checking the ratified acceptance property at each
/// step: **structural + name-resolution identity** — same census, same
/// volume bits, every stable name still resolving (cut names through
/// the instance qualifier).
fn refactorings(ws: &mut Workspace, layout: &ProfileDoc, shelf_i: RecipeNodeId, tol: Tol) {
    println!("\n-- split and inline: the recorded refactorings, and their acceptance --");
    let before_ev = run(layout, &with_store(ws), tol);
    let (before, before_names) = product_of(layout, &before_ev, tol);

    // The cut is the shelf's instance, which is its own placement in
    // the world: a part delivers only its world.
    let part_id = DocumentId::derive("pncad-demo-shelf-cell");
    let store: Arc<dyn PartResolver> = Arc::new(ws.clone());
    let out = split(
        layout,
        &BTreeSet::from([shelf_i]),
        part_id,
        tol,
        Some(&store),
    )
    .expect("cutting one pinned instance out is legal");
    ws.create(&out.part, tol).expect("the new part is stored");
    ws.resave(&out.remainder, tol)
        .expect("the remainder is stored");

    let after_ev = run(&out.remainder, &with_store(ws), tol);
    let (after, after_names) = product_of(&out.remainder, &after_ev, tol);
    assert_eq!(census(&before), census(&after), "the census is preserved");
    assert_eq!(
        volume_bits(&before, tol),
        volume_bits(&after, tol),
        "and the volume, bit for bit"
    );

    // Name-resolution identity, over the WHOLE table rather than a
    // sample: kept names verbatim, cut names re-read through the
    // remainder's new instance. The product is a `⊔` of the layout's
    // copies, naming each copy's entities by the read it takes them
    // through; a copy carries its source's names, so the shelf's are
    // the shelf part's own on either side of the seam, and what moves
    // is the read: the shelf instance's copy before, the remainder
    // instance's after.
    let (was, now) = (
        layout
            .output(shelf_i, 0)
            .expect("the shelf instance's copy"),
        out.remainder
            .output(out.instance, 0)
            .expect("the remainder instance's copy"),
    );
    let mut crossed = 0usize;
    for (name, _) in before_names.iter() {
        let expected = match name.path.as_slice() {
            [RoleSeg::From { read, of }] if *read == was => {
                crossed += 1;
                StableName {
                    kind: name.kind,
                    node: name.node,
                    path: vec![RoleSeg::From {
                        read: now,
                        of: of.clone(),
                    }],
                }
            }
            _ => name.clone(),
        };
        assert!(
            after_names.lookup(&expected).is_some(),
            "{name:?} must still resolve after the split (as {expected:?})"
        );
    }
    // The shelf cell is one instance, and every face, edge and vertex
    // of the shelf's product crossed with it. Pinned as a NUMBER, not
    // as "> 0": a cut that moved fewer names would still satisfy a
    // positivity check while quietly having re-anchored less than the
    // whole seam.
    assert_eq!(
        crossed, 26,
        "every name the shelf instance minted crosses the seam"
    );
    // Cardinality BOTH ways. The loop above proves the split's table is
    // a superset of the original's; this proves it is not a proper one,
    // so a refactoring that ADDED names fails here rather than passing
    // a per-name check that never looks for extras.
    assert_eq!(
        before_names.iter().count(),
        after_names.iter().count(),
        "the split neither loses nor invents a name"
    );
    println!(
        "   split: {} node(s) moved into a new document; {} recorded edit(s) on the \
         remainder, {} on the part; {crossed} of {} product names re-anchored, all resolve",
        out.node_map.len(),
        out.remainder_edits.len(),
        out.part_edits.len(),
        before_names.iter().count()
    );

    // Inline is the inverse, and the SAME identity is what says so —
    // per name, not per count.
    //
    // NOT verbatim, and that is the acceptance property rather than a
    // shortfall: a `StableName` carries the node that minted it, the
    // splice mints FRESH host ids, and the part's world comes back in a
    // `new_space()` of its own, placed into the world under the
    // instance's bundle by one placement (`back.placement`), so a cut
    // name comes back as the part's name with its nodes mapped,
    // wrapped in that placement's copy. A4 asks for name-resolution
    // identity, not arena-key identity, so the correspondence is the
    // composition of the two recorded maps — split's, then inline's —
    // and every pre-split name must resolve through it.
    // The store as it stands NOW — the new part was written above — is
    // what the inline resolves the instance through.
    let store: Arc<dyn PartResolver> = Arc::new(ws.clone());
    let back =
        inline(&out.remainder, out.instance, &store, tol).expect("the instance inlines back");
    let back_ev = run(&back.doc, &with_store(ws), tol);
    let (back_body, back_names) = product_of(&back.doc, &back_ev, tol);
    assert_eq!(
        census(&before),
        census(&back_body),
        "inline restores the census"
    );
    assert_eq!(
        volume_bits(&before, tol),
        volume_bits(&back_body, tol),
        "and the volume bits"
    );
    // The shelf instance comes back as the node the part's own maps to,
    // and the `⊔` reads, in the shelf's seat, the copy the inline lands.
    let restored = back.node_map[&out.node_map[&shelf_i]];
    let members = |doc: &ProfileDoc| match doc
        .product()
        .and_then(|var| doc.operation_of(var))
        .and_then(|at| doc.node(at))
    {
        Some(Node::Assemble {
            members: pncad::document::Bodies::Spelled(members),
        }) => members.iter().map(|member| member.read).collect::<Vec<_>>(),
        other => panic!("the layout's product is a `⊔`, got {other:?}"),
    };
    let seat = members(layout)
        .iter()
        .position(|&read| read == was)
        .expect("the `⊔` reads the shelf's copy");
    let again = members(&back.doc)[seat];
    let mut returned = 0usize;
    for (name, _) in before_names.iter() {
        let expected = match name.path.as_slice() {
            [RoleSeg::From { read, of }] if *read == was => {
                returned += 1;
                StableName {
                    kind: name.kind,
                    node: name.node,
                    path: vec![RoleSeg::From {
                        read: again,
                        of: of.clone(),
                    }],
                }
            }
            _ => name.clone(),
        };
        assert!(
            back_names.lookup(&expected).is_some(),
            "{name:?} must resolve again after the round trip (as {expected:?})"
        );
    }
    assert_eq!(returned, crossed, "the same names came back that went out");
    assert_eq!(
        before_names.iter().count(),
        back_names.iter().count(),
        "and the inline neither loses nor invents a name either"
    );
    // The shelf's placement in the world crosses into the part with it
    // and comes back on the restored node, solving to the same pose bit
    // for bit — where an instance sits is document data, and a round
    // trip that dropped it would still pass every name check above
    // while moving the part.
    let solved = |doc: &ProfileDoc, placement| {
        solve_document(doc, &with_store(ws), tol)
            .placement(doc, placement)
            .expect("the shelf is placed in the world")
    };
    // The restored shelf stands in the part's world, a space of its
    // own, at the pose its bundle gives it; that space is placed into
    // the world at the identity, the remainder instance's bundle.
    assert!(
        on_the_world(&back.doc, restored),
        "the restored shelf is placed by its own bundle"
    );
    assert!(
        on_the_world(&back.doc, back.placement),
        "the part's world is placed into the world by values alone"
    );
    let poses = solve_document(&back.doc, &with_store(ws), tol);
    assert_eq!(
        poses.relative(back.placement),
        Some(Frame::IDENTITY),
        "the part's world stands at the world's identity"
    );
    assert_eq!(
        poses.relative(restored),
        Some(solved(layout, shelf_i)),
        "the round trip puts the shelf back where it stood, exactly"
    );
    println!(
        "   inline: {} node(s) spliced back, {} recorded edit(s); all {} product names \
         resolve through the two recorded node maps, the table is the same size, and the \
         shelf solves to its pose bit-exact",
        back.node_map.len(),
        back.edits.len(),
        before_names.iter().count()
    );

    // THE SECOND CUT: the patterned posts. Split and inline are
    // INVERSES for every legal cut, and the shape most likely to break
    // that is this one: these place a family's members of an instance,
    // and the post instance stands in a `new_space()` the cell does not
    // name, which goes with its readers. The cut leaves the instance
    // pinned at the identity, so inline then flatten gives the layout
    // back up to minted ids and order (A4).
    //
    // The cell is the post instance, the row that copies it, and the
    // placement that puts each member in the world.
    let posts_id = DocumentId::derive("pncad-demo-posts-cell");
    let post_i = layout
        .ids()
        .into_iter()
        .find(|&id| matches!(layout.node(id), Some(Node::InstantiatePart { .. })))
        .expect("the layout has a post instance");
    let pattern = placement_reading(layout, post_i);
    let mut cell = BTreeSet::from([post_i, pattern]);
    cell.extend(
        layout
            .placements()
            .into_iter()
            .filter(|&p| placed_shape(layout, p) == Some(pattern)),
    );
    let store: Arc<dyn PartResolver> = Arc::new(ws.clone());
    let posts = split(layout, &cell, posts_id, tol, Some(&store))
        .unwrap_or_else(|e| panic!("the patterned-post cell splits out: {e}"));
    ws.create(&posts.part, tol)
        .expect("the posts cell is stored");
    let store: Arc<dyn PartResolver> = Arc::new(ws.clone());
    let back = inline(&posts.remainder, posts.instance, &store, tol)
        .unwrap_or_else(|e| panic!("the patterned-post cell inlines back: {e}"));
    let flat = flatten(&back.doc, back.placement, tol)
        .unwrap_or_else(|e| panic!("the inlined cell's identity placement flattens: {e}"))
        .doc;
    let kinds = |doc: &ProfileDoc| {
        let mut kinds: Vec<&'static str> = doc
            .ids()
            .into_iter()
            .filter(|&id| doc.node(id).is_some())
            .filter_map(|id| doc.spoken(id).kind())
            .collect();
        kinds.sort_unstable();
        kinds
    };
    assert_eq!(
        kinds(&flat),
        kinds(layout),
        "the round trip holds the layout's nodes, kind for kind"
    );
    let (flat_body, _) = product_of(&flat, &run(&flat, &with_store(ws), tol), tol);
    assert_eq!(
        census(&flat_body),
        census(&before),
        "the round trip keeps the census"
    );
    // The same pieces, gathered in another order: the volumes agree to
    // the rounding of their sum, not bit for bit.
    let (round_tripped, original) = (
        f64::from_bits(volume_bits(&flat_body, tol)),
        f64::from_bits(volume_bits(&before, tol)),
    );
    let drift = (round_tripped - original).abs() / original.abs();
    assert!(
        drift <= 64.0 * f64::EPSILON,
        "and the volume, to its sum's rounding: {round_tripped:e} against {original:e}"
    );
    println!(
        "   second cut: the patterned-post cell, its space with it, splits out and inlines \
         back; flattened, it is the layout up to minted ids and order ({} nodes, the same \
         census, the volume within {drift:.1e} relative)",
        kinds(&flat).len()
    );
}

// ---- The update door (A13) ----

/// Accepts a new version of a part, the way A13 says an author does:
/// the per-reference primitive, the whole-document elaboration over
/// it, the mixed-pin LINT in between, and re-verification at every
/// evaluation.
fn update_door(ws: &mut Workspace, stand: &Stand, bench: &Bench, shelf: DocRef, tol: Tol) {
    println!("\n-- the update door: moving a pin is a recorded edit --");
    let before = run(&bench.doc, &with_store(ws), tol);
    let crate_before = solve_document(&bench.doc, &with_store(ws), tol)
        .placement(&bench.doc, bench.crate_i)
        .expect("the crate is placed on the shelf");
    let (before_body, _) = product_of(&bench.doc, &before, tol);
    let before_volume = pncad::topo::mass_properties(
        &crate::booleans::at_rest(&before_body, tol),
        tol,
        crate::printed(),
    )
    .expect("mass properties")
    .enclosure
    .volume;

    // The part changes on disk: a thicker board. The shelf is modelled
    // from its UNDERSIDE up, so the face the posts' mates seat on does
    // not move — which is what keeps the stand fitting. Its TOP moves,
    // and that is what the crate meets (wall 1).
    let mut thicker = ws.resolve(&shelf, tol).expect("the shelf resolves");
    edit(
        &mut thicker,
        &DocEdit::SetVarValue {
            var: VarName::from_static("thickness").into(),
            value: FreeValue::Continuous(SHELF_THICKNESS * 1.5),
        },
        tol,
    );
    ws.resave(&thicker, tol).expect("the new version is stored");

    // Until the pin moves, the assembly still means what it meant: the
    // reference names a version the store no longer holds, and the
    // evaluation says so instead of silently taking the new one.
    let stale = run(&bench.doc, &with_store(ws), tol);
    let refused = stale
        .node_error(stand.shelf_i)
        .expect("an out-of-date pin is surfaced, never silently retargeted");
    println!(
        "   before the edit: the instance refuses — {}",
        pin_fault(refused)
    );

    // The message a user reads carries the recourse EXACTLY ONCE, and
    // this line is what holds that number down: the store's sentence
    // for a moved pin ends on `PIN_MISMATCH_RECOURSE`, and the resolve
    // door appends nothing to that arm. Zero would mean the sentence
    // dropped it and the kernel-side message no longer tells an author
    // what to do; two would mean the door started appending it. The
    // ZERO case is also held by `crates/viewer/tests/instance_authoring.rs`,
    // which asserts the recourse on the badge; what only this line and
    // the Python author suite hold is the COUNT, which is what a
    // `contains` assertion cannot see.
    assert_eq!(
        refused
            .kind
            .to_string()
            .matches(PIN_MISMATCH_RECOURSE)
            .count(),
        1,
        "the kernel-side message carries the recourse once, from the store's sentence"
    );
    println!("   in full: {}", refused.kind);

    // The elaboration: "update this document everywhere", one recorded
    // per-reference edit per site, applied as a group.
    let edits = update_to_store(&bench.doc, shelf.id, ws, tol).expect("the store has a new pin");
    assert_eq!(edits.len(), 1, "the shelf is referenced once");
    let mut updated = bench.doc.clone();
    for e in &edits {
        assert!(
            matches!(e, DocEdit::UpdateReference { .. }),
            "the elaboration is per-reference primitives and nothing else"
        );
        edit(&mut updated, e, tol);
    }

    let after = run(&updated, &with_store(ws), tol);
    // A13 clause 4: the pin move triggers ordinary re-evaluation, and
    // the census records the seats again against the NEW geometry.
    // The posts' seats name the shelf's UNDERSIDE, which a thicker
    // board leaves where it was, so they hold. The crate rests on the
    // shelf's TOP face, so it rides up with the board by exactly the
    // thickness it gained, and the at-rest check passes the bench as it
    // stands.
    let crate_after = solve_document(&updated, &with_store(ws), tol)
        .placement(&updated, bench.crate_i)
        .expect("the crate is placed on the thicker shelf");
    let risen = crate_after.translation[2] - crate_before.translation[2];
    assert!(
        (risen - SHELF_THICKNESS * 0.5).abs() < 1e-12,
        "the crate rides up with the shelf's top by the thickness it gained: {risen} m"
    );
    assert_eq!(
        crate_before.columns, crate_after.columns,
        "the crate's orientation is untouched by a thickness edit"
    );
    // The at-rest check passes the thicker bench, the crate on its top.
    at_rest(&updated, &after, tol);
    let (after_body, _) = product_of(&updated, &after, tol);
    let after_volume = pncad::topo::mass_properties(
        &crate::booleans::at_rest(&after_body, tol),
        tol,
        crate::printed(),
    )
    .expect("mass properties")
    .enclosure
    .volume;
    assert!(
        after_volume > before_volume,
        "the new version carries more material"
    );
    println!(
        "   after {} recorded UpdateReference edit(s): V {:.6} -> {:.6} m^3; the \
         seats were recorded again against the new geometry and put back through the \
         at-rest check, which finds nothing: the crate rode up {risen:.3} m with the shelf's top",
        edits.len(),
        before_volume,
        after_volume,
    );

    // A13 clause 3: two pins of one document id in one assembly is
    // LEGAL and sometimes intended (a staged migration), so it is a
    // lint, not a refusal. The post is referenced twice; move ONE with
    // the primitive and the lint reports the multiplicity with the
    // nodes holding each pin.
    // Where the shelf sits on the posts as they are — read now, while
    // the store still holds the version both posts pin: a seat's pose
    // is read off the post's own cap face, so a solve of a document
    // whose pin the store has moved past refuses that side rather than
    // remember an old pose.
    let shelf_before = solve_document(&updated, &with_store(ws), tol)
        .placement(&updated, stand.shelf_i)
        .expect("the shelf is solved before the migration");
    let mut shorter = ws
        .resolve(
            &DocRef {
                id: post_ref_of(&bench.doc, stand.post_a),
                pin: post_pin_of(&bench.doc, stand.post_a),
            },
            tol,
        )
        .expect("the post resolves");
    edit(
        &mut shorter,
        &DocEdit::SetVarValue {
            var: VarName::from_static("height").into(),
            value: FreeValue::Continuous(POST_HEIGHT - 0.04),
        },
        tol,
    );
    ws.resave(&shorter, tol).expect("the short post is stored");
    let short_pin = content_pin(&shorter, tol).expect("the pin computes");

    assert!(
        mixed_pins(&updated).is_empty(),
        "before the staged edit every reference agrees"
    );
    let mut staged = updated.clone();
    edit(
        &mut staged,
        &DocEdit::UpdateReference {
            node: stand.post_a,
            new_pin: short_pin,
        },
        tol,
    );
    let lint = mixed_pins(&staged);
    assert_eq!(lint.len(), 1, "one id at two pins");
    println!(
        "   mixed-pin lint (A13 clause 3, a REPORT not a gate): document {} held at {} pins \
         by {} referencing node(s)",
        lint[0].id,
        lint[0].pins.len(),
        lint[0].pins.iter().map(|s| s.nodes.len()).sum::<usize>()
    );

    // GAP, met here and not worked around: the staged state the lint
    // exists FOR cannot be evaluated. (AQ1, the open document-store
    // question, is where "which version lives where" is decided.) A13 calls two pins of one id
    // "legal and sometimes intended", but a workspace is one file per
    // document id, so exactly one of the two pins can ever resolve —
    // the site still holding the other refuses typed, naming its node.
    // The lint is structural and answers; the evaluation cannot honor
    // what the lint permits. Which version lives where is AQ1, the
    // open document-store question, arriving at user scale.
    let staged_ev = run(&staged, &with_store(ws), tol);
    let stranded = staged_ev
        .node_error(stand.post_b)
        .expect("the site still holding the old pin cannot resolve");
    println!(
        "   staged state (gap): node {} still pins the previous version, and the store \
         holds one file per id — {}",
        stranded.node,
        pin_fault(stranded)
    );

    // So finish the migration the way A13 says: the whole-document
    // ELABORATION, one recorded per-reference edit per site.
    let post_id = post_ref_of(&bench.doc, stand.post_a);
    let all = update_to_store(&staged, post_id, ws, tol).expect("the store has the new pin");
    assert_eq!(all.len(), 1, "one site is already on the new pin");
    let mut migrated = staged.clone();
    for e in &all {
        edit(&mut migrated, e, tol);
    }
    assert!(
        mixed_pins(&migrated).is_empty(),
        "the elaboration leaves every site on one pin"
    );

    // And now the mates do their job. Both posts are 40 mm short, and
    // each seat's pose is the plane of the post's top cap FACE, read
    // off the post's own evaluation at every solve — so the shelf
    // comes down 40 mm with the posts, the rest between each cap and
    // the shelf's underside still holds, and the at-rest check passes both
    // seats. The face name is the state; the pose is derived.
    let ev = run(&migrated, &with_store(ws), tol);
    let shelf_after = solve_document(&migrated, &with_store(ws), tol)
        .placement(&migrated, stand.shelf_i)
        .expect("the shelf is solved after the migration");
    let dropped = shelf_before.translation[2] - shelf_after.translation[2];
    assert!(
        (dropped - 0.04).abs() < 1e-12,
        "the shelf follows the shortened posts by the height change: {dropped} m"
    );
    assert_eq!(
        shelf_before.columns, shelf_after.columns,
        "the shelf's orientation is untouched by a height edit"
    );
    // The posts' seats follow their caps, so the stand fits, and the
    // crate rests on the shelf's top face, so it comes down with the
    // shelf by the same height; the at-rest check finds nothing on the
    // migrated bench.
    let crate_migrated = solve_document(&migrated, &with_store(ws), tol)
        .placement(&migrated, bench.crate_i)
        .expect("the crate is solved after the migration");
    let crate_dropped = crate_after.translation[2] - crate_migrated.translation[2];
    assert!(
        (crate_dropped - 0.04).abs() < 1e-12,
        "the crate comes down with the shelf by the height change: {crate_dropped} m"
    );
    // The at-rest check finds nothing on the migrated bench, the crate on
    // its shelf.
    at_rest(&migrated, &ev, tol);
    println!(
        "   \"does it actually fit\": after the migration the shelf came down {dropped:.3} m with \
         the shortened posts and the crate {crate_dropped:.3} m with the shelf — each mate names \
         a FACE and the solve reads its pose off the part — and the at-rest check finds \
         nothing"
    );

    // Undo is keeping the prior value: the migrated document is one
    // the author can simply not adopt. Put both parts back, so the
    // workspace a reader opens is the one the saved assemblies pin.
    edit(
        &mut shorter,
        &DocEdit::SetVarValue {
            var: VarName::from_static("height").into(),
            value: FreeValue::Continuous(POST_HEIGHT),
        },
        tol,
    );
    ws.resave(&shorter, tol).expect("the post is restored");
    edit(
        &mut thicker,
        &DocEdit::SetVarValue {
            var: VarName::from_static("thickness").into(),
            value: FreeValue::Continuous(SHELF_THICKNESS),
        },
        tol,
    );
    ws.resave(&thicker, tol).expect("the shelf is restored");
    let restored = run(&bench.doc, &with_store(ws), tol);
    assert!(
        restored.node_error(stand.shelf_i).is_none(),
        "the authored stand resolves again against the restored store"
    );
}

/// A failing instantiate node's seam fault, read the way a caller
/// reads it — the CLASSIFICATION, not the store's paragraph. The three
/// seam rules A4 singles out stay separately observable here without
/// anyone parsing a rendered message.
fn pin_fault(err: &pncad::document::NodeError) -> String {
    match &err.kind {
        pncad::document::NodeErrorKind::Part { doc_ref, fault } => match fault {
            pncad::document::PartFault::Unresolved { fault, .. } => {
                format!(
                    "node {} reaches document {}: {fault:?}",
                    err.node, doc_ref.id
                )
            }
            other => format!("node {}: {other:?}", err.node),
        },
        other => format!("node {}: {other}", err.node),
    }
}

/// The document id an instance references.
fn post_ref_of(doc: &ProfileDoc, node: RecipeNodeId) -> DocumentId {
    match doc.node(node) {
        Some(Node::InstantiatePart { doc_ref, .. }) => doc_ref.id,
        other => panic!("node {} is not an instance: {other:?}", node),
    }
}

/// The pin an instance's reference carries.
fn post_pin_of(doc: &ProfileDoc, node: RecipeNodeId) -> pncad::document::ContentPin {
    match doc.node(node) {
        Some(Node::InstantiatePart { doc_ref, .. }) => doc_ref.pin,
        other => panic!("node {} is not an instance: {other:?}", node),
    }
}

// ---- The round trip ----

/// Saves each assembly through the persistence door, loads it back, and
/// evaluates the loaded document: same census, same volume bits, same
/// gate verdict. A document is a value on disk, or it is not a
/// document.
fn round_trip(ws: &Workspace, doc: &ProfileDoc, label: &str, tol: Tol) {
    let ev = run(doc, &with_store(ws), tol);
    let (body, names) = product_of(doc, &ev, tol);

    let text = save(doc, &[], tol).expect("the document saves");
    let loaded = load(&text, tol).expect("and loads");
    assert_eq!(
        loaded.doc.id(),
        doc.id(),
        "identity survives the round trip"
    );

    let ev2 = run(&loaded.doc, &with_store(ws), tol);
    let (body2, names2) = product_of(&loaded.doc, &ev2, tol);
    assert_eq!(census(&body), census(&body2), "same census after reload");
    assert_eq!(
        volume_bits(&body, tol),
        volume_bits(&body2, tol),
        "same volume, bit for bit"
    );
    assert_eq!(
        names.iter().count(),
        names2.iter().count(),
        "same name table"
    );
    for (name, _) in names.iter() {
        assert!(
            names2.lookup(name).is_some(),
            "{name:?} still resolves after the round trip"
        );
    }
    println!(
        "   [{label}] {} bytes through the persistence door: reloaded and re-evaluated \
         identically ({} names, V bit-equal)",
        text.len(),
        names.iter().count()
    );
}

// ---- The stop ----
/// Authors the workspace, walks every assembly door, and returns the
/// rendered scenes: the bench at three swings of its turntable, the
/// first beside the flat-pack in the montage cell.
///
/// `work` is the store: an assembly needs a DIRECTORY of documents,
/// which is the one thing a tour scene had never needed before — see
/// the friction note in `walk_tour`.
///
/// # A mate's poses are read off faces, and the solve reads them
///
/// A11 keeps the solve's ALGORITHM structural — coset intersection
/// over decided predicates, no numeric fitting — while its inputs are
/// the document plus its mated parts' evaluations: each seat is a
/// face's plane, read off the part's own evaluation every time, and
/// the values setting the freedoms the seat leaves are numbers in the
/// bodies' own coordinates. A post whose height changes moves the seat
/// with it, which the update walk shows — the shelf comes down with
/// the shortened posts and the at-rest check still finds nothing; the crate rests
/// on the shelf's top face, so it follows the shelf.
///
/// # A pin places the stand, and a parameter drives the pin
///
/// The near post's pin to the world spins its world frame about the
/// turntable's axis by the swing, so every copy the mates place onto
/// it is turned with it, and the swing is a document parameter: one
/// value edit moves every part on it, the crate included, and re-runs
/// those four instances alone.
pub fn stops(work: &Path, tol: Tol) -> Vec<TourStop> {
    let (mut ws, parts) = workspace(work, tol);
    println!(
        "   workspace: {} document(s) in {}",
        ws.documents().len(),
        ws.root().display()
    );

    let (layout, pattern, shelf_i) = layout_doc(parts.post, parts.shelf, tol);
    ws.create(&layout, tol).expect("the layout is stored");
    let stand = stand_doc(&parts, tol);
    let bench = bench(&stand, &parts, tol);
    ws.create(&bench.doc, tol).expect("the bench is stored");

    let layout_body = layout_scene(&ws, &layout, pattern, tol);
    let authored = stand_scene(&ws, &stand, &bench, tol);
    let posed = poses(&ws, &stand, &bench, authored, tol);

    refusals(&ws, &parts, tol);

    println!("\n-- the round trip: a document is a value on disk --");
    round_trip(&ws, &layout, "layout", tol);
    round_trip(&ws, &bench.doc, "bench", tol);

    refactorings(&mut ws, &layout, shelf_i, tol);
    // The refactoring walk rewrote the layout's file; the store's job
    // for the rest of this stop is to answer for the PARTS, so put the
    // authored layout back where a reader will look for it.
    ws.resave(&layout, tol).expect("the layout is restored");

    update_door(&mut ws, &stand, &bench, parts.shelf, tol);

    // ONE cell for both framings, at the first swing. The assembled
    // bench and the flat-pack are the same part documents answering
    // two questions — what the mates and the pin place, and
    // what ships — and two independently-scaled panels make them look
    // like two subjects. The flat-pack's translation is AUTHORED into
    // its placements (see `layout_doc`), where a layout's placements
    // are its subject. The other two swings render on their own.
    let bench_ops = "post.pncad + shelf.pncad + crate.pncad -> \
                     InstantiatePart (values on the world's axes, turned about the turntable's \
                     axis by swing) + InstantiatePart x2 -> SetConstraints x2 (a Rest, plane on \
                     flipped plane, and its slide x, slide y and spin, the whole bundle of the \
                     copy it places) -> constructive solve; InstantiatePart (crate) -> \
                     SetConstraints (a Rest, crate's bottom on the shelf's top face, and its \
                     three values) -> A10 product gather -> \
                     SetVarValue(swing)";
    let bench_story = "an ASSEMBLY document: two instances of a post document and one of a \
                       shelf document, the shelf SEATED on both by mates — only the near post \
                       is pinned to the world, the other two poses are solved — and the near \
                       post's pin turns on a TURNTABLE whose swing is a document parameter. \
                       A crate rests on the shelf's top face, set where it stands by values, \
                       by a mate that places it on the shelf, so it swings with the stand and \
                       follows any edit of the shelf. One SetVarValue on the swing moves all four \
                       parts";
    let assembled = |counters: &str| {
        format!(
            "{counters}. ASSEMBLED: 4 solids, V = {BENCH_VOLUME:.6} m^3 at every swing; every \
             vertex is where the turntable's chain composed by hand puts it, the shelf's centroid \
             stays on the pivot, and the A5 at-rest check finds nothing at the posts' flush \
             seats or the crate's rest"
        )
    };
    let mut layout_body = Some(layout_body);
    posed
        .into_iter()
        .enumerate()
        .map(|(i, (degrees, gate, counters))| {
            let name: &'static str = match i {
                0 => "bench",
                1 => "bench60",
                _ => "bench90",
            };
            let mut bodies = vec![SceneBody::at_rest(
                name,
                [0.55, 0.44, 0.30],
                gate.body,
                gate.contacts,
            )];
            let montage = i == 0;
            let (caption, story, ops, note) = if montage {
                bodies.extend(layout_body.take());
                (
                    format!("the bench — on a turntable at {degrees} deg, and flat-packed"),
                    "the bench on its turntable at the first swing, and beside it the post and \
                     shelf documents laid out for shipping: ONE post instance patterned TWICE \
                     plus the shelf, nothing touching, which is A5's disjoint half where the \
                     at-rest check finds nothing at all. The bench: an ASSEMBLY document — two \
                     instances of a post document and one of a shelf document, the shelf \
                     SEATED on both by mates, only the near post pinned to the world — its \
                     pin turning on a TURNTABLE whose swing is a document parameter — and a \
                     crate resting on the shelf's top face, set where it stands by values \
                     and placed on the shelf",
                    "post.pncad + shelf.pncad + crate.pncad -> \
                     InstantiatePart (values on the world's axes, turned about the turntable's \
                     axis by swing) + InstantiatePart x2 -> SetConstraints x2 (a Rest, plane on \
                     flipped plane, and its slide x, slide y and spin, the whole bundle of the \
                     copy it places) -> constructive solve; InstantiatePart (crate) -> \
                     SetConstraints (a Rest, crate's bottom on the shelf's top face, and its \
                     three values) -> A10 product gather -> \
                     SetVarValue(swing); and \
                     InstantiatePart (into a new space) -> k = index(2) -> Place (a row over k) -> \
                     Place x2 of its members \
                     (values, a rotated pose) + InstantiatePart (values, a translated \
                     pose) -> A10 product gather -> assemble",
                    format!(
                        "{}. FLAT-PACKED: 3 solids, V = {:.6} m^3; every product entity \
                         answers to a member-qualified name (the family's Member(k) over \
                         the part's own)",
                        assembled(&counters),
                        2.0 * POST_VOLUME + SHELF_VOLUME
                    ),
                )
            } else {
                (
                    format!("the bench — on a turntable at {degrees} deg"),
                    bench_story,
                    bench_ops,
                    assembled(&counters),
                )
            };
            TourStop {
                name,
                caption,
                montage,
                story,
                ops,
                delta: 4e-3,
                note: Some(note),
                view: View {
                    elev: 22.0,
                    azim: -60.0,
                    up: 'z',
                },
                bodies,
            }
        })
        .collect()
}
