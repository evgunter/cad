//! **The assembly stop** (ASSEMBLY-DESIGN A2–A13, the ASM program's
//! v1 scope): two part documents, a workspace on disk, and two
//! assembly documents built out of them — one a flat-pack layout that
//! patterns a part, one a mated bench whose placements are solved
//! constructively from its mates.
//!
//! Written the way a user writes an assembly, through the public
//! doors: `Workspace` to hold the documents, `InstantiatePart` +
//! `SetPlacement` to reference and place them, `Node::Pattern` to
//! replicate one, `Node::Mate` to seat one part on another,
//! `assemble` for the at-rest gate, `split`/`inline` to refactor,
//! `update_to_store` to accept a new version of a part, and `save` /
//! `load` to round-trip. What this stop reports it MEASURED on this
//! run; what refuses, refuses typed and is printed with the recourse
//! the library gave.
//!
//! # The two documents, and why there are two
//!
//! `bench-stand` is the assembled object: two posts and a shelf, the
//! shelf seated on the posts by mates. `bench-layout` is the same two
//! parts laid out flat for shipping — two posts on their side (one
//! instance, patterned) and the shelf beside them, nothing touching.
//! Both are real things a user models, and between them they cover
//! the two halves of A5's validity story: the layout is DISJOINT and
//! its at-rest gate passes outright; the stand TOUCHES, and its gate
//! CERTIFIES, its two flush seats included (see [`stand_scene`]).
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
//! - **#943 / #1063 — CLOSED, and the accommodation retired.** A mate
//!   declares a FACE PAIR; the census backs the vertex-on-edge and
//!   edge-edge events a flush seat induces from that one declaration,
//!   and the declared pair itself now certifies on the two
//!   descriptions' shared world carrier. The stand's posts are seated
//!   FLUSH with the shelf's ends — the obvious way to draw it — where
//!   they had to be inset while the chart-identity door declined every
//!   cross-instance pair (`SEAT_A`).
//! - **#944** — nothing mints a mate's alignment frame from a
//!   selected face, so the frame and the geometry drift apart
//!   silently (`stops`, `update_door`).
//! - **#945** — mates and patterns do not compose at all, which is
//!   why this file has two assembly documents rather than one; it
//!   also records the A11 rule-4 drift, and wants Evan's ruling.
//! - **#946** — a sub-assembly's mate declarations do not cross the
//!   instantiation seam.
//! - **#947** — the pin-mismatch recourse is emitted twice
//!   (ASSERTED here, so it goes red when fixed), and two refusals
//!   carry no recourse sentence at all (`refusals`).
//! - **#948** — no parametric loop constructor (`rect`).
//!
//! The declared direction's frontier — a mated assembly's gate can
//! neither certify nor refute — is not new here; it is the census
//! Door-2 gap steered to M9 on #591, and `at_rest` states what this
//! scene can and cannot say about it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::{BTreeMap, BTreeSet};
use std::f64::consts::PI;
use std::path::Path;
use std::sync::Arc;

use pncad::document::{
    Alignment, Assembly, AssemblyError, Attribution, AxisSense, CancelToken, Dimension, DocEdit,
    DocParam, DocParamValue, DocRef, DocumentId, EvalOptions, Evaluation, Expr, Frame, InlineError,
    LoopProgram, MateFault, MateFrame, MatePrimitive, Node, ParamName, PatternKind, ProfileDoc,
    ProfileProgram, ProgramStep, ProgramTarget, RecipeNodeId, apply, assemble, content_pin,
    evaluate, inline, load, mixed_pins, parse_expr, product_named, save, solve_document, split,
};
use pncad::geom_core::Tol;
use pncad::prelude::StableName;
use pncad::profile::SketchPlane;
use pncad::select::{
    CapEnd, ContactClass, EntityKind, NamePat, NameTable, RoleSeg, SegPat, SegTag, Selector,
};
use pncad::topo::Body;
use pncad::workspace::{PIN_MISMATCH_RECOURSE, Workspace, WorkspaceError, update_to_store};

use crate::{SceneBody, Stop, View};

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

/// The post's own seating point, in POST coordinates: the centre of
/// its top cap. Every mate that seats something on a post is authored
/// against this one value — a second spelling of it is a second place
/// for the model and the mates to disagree.
const POST_SEAT: [f64; 3] = [POST_SECTION / 2.0, POST_SECTION / 2.0, POST_HEIGHT];

/// One post's volume, and the shelf's — the arithmetic every census
/// below is checked against.
const POST_VOLUME: f64 = POST_SECTION * POST_SECTION * POST_HEIGHT;
const SHELF_VOLUME: f64 = SHELF_LENGTH * SHELF_DEPTH * SHELF_THICKNESS;

// ---- Small authoring helpers ----

/// The text expression door, with the document's parameters in scope
/// — the way a user types a dimension (`"section"`, `"120 mm"`).
fn pe(src: &str, params: &BTreeMap<ParamName, Dimension>) -> Expr {
    parse_expr(src, params).unwrap_or_else(|e| panic!("expression `{src}`: {e:?}"))
}

/// Inserts a node and returns its minted id.
fn insert(doc: &mut ProfileDoc, node: Node<ProfileProgram>, tol: Tol) -> RecipeNodeId {
    let applied = apply(doc, &DocEdit::InsertNode { node }, tol).expect("the insert applies");
    *doc = applied.doc;
    applied.record.minted.expect("an insert mints an id")
}

/// Applies an edit that mints nothing.
fn edit(doc: &mut ProfileDoc, e: &DocEdit<ProfileProgram>, tol: Tol) {
    let applied = apply(doc, e, tol).unwrap_or_else(|err| panic!("edit refused: {err:?}"));
    *doc = applied.doc;
}

/// A rectangle in the sketch plane from two Expr corners — the
/// parametric spelling of `LoopProgram::polygon`, which only takes
/// literals.
///
/// GAP (#948): a parametric author writes the five steps by hand. The
/// document's own doc comment says so ("parametric authors write the
/// steps with their own Exprs"), and this function is what every
/// parametric consumer will write until the loop vocabulary grows an
/// Expr-bearing rectangle.
fn rect(w: &Expr, h: &Expr, zero: &Expr) -> LoopProgram {
    let pt = |x: &Expr, y: &Expr| ProgramTarget::Point([x.clone(), y.clone()]);
    LoopProgram::Chain(vec![
        ProgramStep::At([zero.clone(), zero.clone()]),
        ProgramStep::LineTo(pt(w, zero)),
        ProgramStep::LineTo(pt(w, h)),
        ProgramStep::LineTo(pt(zero, h)),
        ProgramStep::LineTo(ProgramTarget::Start),
    ])
}

/// A mate frame: origin, primary axis, clocking reference.
fn mate_frame(origin: [f64; 3]) -> MateFrame {
    MateFrame {
        origin,
        axis: [0.0, 0.0, 1.0],
        reference: [1.0, 0.0, 0.0],
    }
}

/// A part-local name, wrapped at the instance that placed it — the
/// instance-qualified form every cross-document reference takes (the
/// GQ4 wrapper × N1–N7).
fn in_part(instance: RecipeNodeId, local: &StableName) -> StableName {
    StableName {
        kind: local.kind,
        node: instance,
        path: vec![RoleSeg::InPart {
            of: Box::new(local.clone()),
        }],
    }
}

/// The evaluation options that carry the store: an evaluation with no
/// resolver refuses every instantiate node rather than pretending a
/// part is empty, so the workspace IS what makes an assembly
/// evaluable.
fn with_store(ws: &Workspace) -> EvalOptions {
    EvalOptions {
        resolver: Some(Arc::new(ws.clone())),
        ..EvalOptions::default()
    }
}

fn run(doc: &ProfileDoc, opts: &EvalOptions, tol: Tol) -> Evaluation<f64> {
    evaluate::<f64>(doc, None, &CancelToken::new(), opts, tol)
}

/// The structural census the A4 acceptance identity compares: solids,
/// faces, edges, vertices of a whole product.
fn census(body: &Body<f64>) -> (usize, usize, usize, usize) {
    (
        body.shells().count(),
        body.faces().count(),
        body.edges().count(),
        body.vertices().count(),
    )
}

/// A product's volume, by bits — the other half of the identity.
fn volume_bits(body: &Body<f64>, tol: Tol) -> u64 {
    pncad::topo::mass_properties(body, tol)
        .expect("mass properties")
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
/// The extrusion runs +z from the sketch plane at z = 0, so the part's
/// SEATING face is its top cap and its datum face is the origin plane.
fn prism_part(
    label: &str,
    params: &[(&str, f64)],
    plan: (&str, &str),
    length: &str,
    tol: Tol,
) -> ProfileDoc {
    let mut doc = ProfileDoc::empty(DocumentId::derive(label), tol);
    let mut scope: BTreeMap<ParamName, Dimension> = BTreeMap::new();
    for &(name, value) in params {
        let name = ParamName::new(name);
        edit(
            &mut doc,
            &DocEdit::SetDocParam {
                name: name.clone(),
                value: DocParam::continuous(Dimension::Length, value),
            },
            tol,
        );
        scope.insert(name, Dimension::Length);
    }
    let zero = pe("0 mm", &scope);
    let profile = insert(
        &mut doc,
        Node::Profile(ProfileProgram {
            plane: SketchPlane::xy(),
            loops: vec![rect(&pe(plan.0, &scope), &pe(plan.1, &scope), &zero)],
        }),
        tol,
    );
    insert(
        &mut doc,
        Node::Extrude {
            profile,
            distance: pe(length, &scope),
        },
        tol,
    );
    doc
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

/// A part's own cap-face name at `end`, as its PRODUCT answers to it —
/// the name a mate on that face refers to, before the instance
/// qualifier wraps it.
///
/// Selected structurally (`Cap`, side `end`) rather than hand-built:
/// the naming vocabulary is what a user reaches for, and a selector
/// that stops matching is the library telling you the vocabulary
/// moved.
fn cap_of(doc: &ProfileDoc, end: CapEnd, tol: Tol) -> StableName {
    let ev = run(doc, &EvalOptions::default(), tol);
    let tip = *doc.roots().first().expect("the part has a product root");
    let sel =
        Selector::of(NamePat::of_kind(EntityKind::Face).seg(SegPat::tag(SegTag::Cap).side(end)));
    let found = pncad::select::select(&ev, tip, &sel);
    assert_eq!(
        found.len(),
        1,
        "an extruded prism has exactly one {end:?} cap; got {found:?}"
    );
    found.into_iter().next().expect("checked non-empty")
}

// ---- The assembly documents ----

/// The flat-pack: two posts on their side (ONE instance, patterned)
/// and the shelf laid beside them. Nothing touches.
///
/// Every placement carries [`FLAT_PACK_GAP`] along +x, which is how the
/// flat-pack sits BESIDE the assembled bench in their shared montage
/// cell. It is AUTHORED into the frames rather than applied to the
/// gathered body, and that is not a preference: both furniture bodies
/// carry declared contacts, and `transform_rigid` re-mints face keys,
/// so a moved product would carry `ContactRecords` naming faces that
/// no longer exist. A common offset on every placement moves the
/// product and changes nothing else about it.
fn layout_doc(post: DocRef, shelf: DocRef, tol: Tol) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let mut doc = ProfileDoc::empty(DocumentId::derive("pncad-demo-layout"), tol);
    let scope = BTreeMap::new();
    let post_i = insert(&mut doc, Node::instantiate_part(post), tol);
    // Explicit frame (A3): the post is laid on its side — a rotation
    // that a translation-only registry could not express, which is
    // why the frame stores a general linear part.
    edit(
        &mut doc,
        &DocEdit::SetPlacement {
            node: post_i,
            frame: Frame::rotate_then_translate(
                [0.0, 1.0, 0.0],
                -PI / 2.0,
                [FLAT_PACK_GAP + POST_HEIGHT, 0.0, 0.0],
            ),
        },
        tol,
    );
    let pattern = insert(
        &mut doc,
        Node::Pattern {
            input: post_i,
            count: pe("2", &scope),
            kind: PatternKind::Linear {
                direction: [pe("0.0", &scope), pe("1.0", &scope), pe("0.0", &scope)],
                spacing: pe("200 mm", &scope),
            },
        },
        tol,
    );
    let shelf_i = insert(&mut doc, Node::instantiate_part(shelf), tol);
    edit(
        &mut doc,
        &DocEdit::SetPlacement {
            node: shelf_i,
            frame: Frame::translation([FLAT_PACK_GAP, 0.9, 0.0]),
        },
        tol,
    );
    (doc, pattern, shelf_i)
}

/// The stand: a post at each end of the shelf, the shelf SEATED on
/// them by mates. Only the gauge post carries an authored frame —
/// A11 puts placement on the cluster, and the mates place the rest.
struct Stand {
    doc: ProfileDoc,
    post_a: RecipeNodeId,
    shelf_i: RecipeNodeId,
    post_b: RecipeNodeId,
    mate_1: RecipeNodeId,
    mate_2: RecipeNodeId,
}

fn stand_doc(
    post: DocRef,
    shelf: DocRef,
    post_top: &StableName,
    shelf_bottom: &StableName,
    primitive: MatePrimitive,
    tol: Tol,
) -> Stand {
    let mut doc = ProfileDoc::empty(DocumentId::derive("pncad-demo-stand"), tol);
    let post_a = insert(&mut doc, Node::instantiate_part(post), tol);
    edit(
        &mut doc,
        &DocEdit::SetPlacement {
            node: post_a,
            frame: Frame::translation([0.0, (SHELF_DEPTH - POST_SECTION) / 2.0, 0.0]),
        },
        tol,
    );
    let shelf_i = insert(&mut doc, Node::instantiate_part(shelf), tol);
    let post_b = insert(&mut doc, Node::instantiate_part(post), tol);

    let mate_1 = insert(
        &mut doc,
        Node::Mate {
            a: in_part(post_a, post_top),
            b: in_part(shelf_i, shelf_bottom),
            class: ContactClass::Rest,
            alignment: Alignment {
                a: mate_frame(POST_SEAT),
                b: mate_frame(SEAT_A),
                primitive,
                sense: AxisSense::Aligned,
                clocking: None,
            },
        },
        tol,
    );
    let mate_2 = insert(
        &mut doc,
        Node::Mate {
            a: in_part(shelf_i, shelf_bottom),
            b: in_part(post_b, post_top),
            class: ContactClass::Rest,
            alignment: Alignment {
                a: mate_frame(SEAT_B),
                b: mate_frame(POST_SEAT),
                primitive,
                sense: AxisSense::Aligned,
                clocking: None,
            },
        },
        tol,
    );
    Stand {
        doc,
        post_a,
        shelf_i,
        post_b,
        mate_1,
        mate_2,
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
    /// The post's top cap, in the post's own names.
    post_top: StableName,
    /// The shelf's underside, in the shelf's own names.
    shelf_bottom: StableName,
}

/// Writes the two part documents into a fresh workspace directory and
/// opens it.
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
    ws.create(&post, tol).expect("write the post document");
    ws.create(&shelf, tol).expect("write the shelf document");
    let reference = |d: &ProfileDoc| DocRef {
        id: d.id(),
        pin: content_pin(d, tol).expect("the pin computes"),
    };
    let parts = Parts {
        post: reference(&post),
        shelf: reference(&shelf),
        post_top: cap_of(&post, CapEnd::Top, tol),
        shelf_bottom: cap_of(&shelf, CapEnd::Bottom, tol),
    };
    (ws, parts)
}

// ---- The scenes ----

/// The flat-pack layout: three disjoint solids, and A5's disjoint half
/// — the at-rest gate passes outright.
///
/// TWO posts, the same count the bench assembles, so "the same parts,
/// flat-packed" is true of the PARTS and not only of the documents. Two
/// is also what the name lookup below needs: it asks for instance 1,
/// the first NON-IDENTITY instance — `Node::Pattern` may hand back the
/// prototype verbatim for index 0, so i = 0 need not exercise a
/// placement at all, and i = 1 is the first that must.
fn layout_scene(ws: &Workspace, doc: &ProfileDoc, pattern: RecipeNodeId, tol: Tol) -> SceneBody {
    let ev = run(doc, &with_store(ws), tol);
    let (body, names) = product_of(doc, &ev, tol);

    assert_eq!(
        body.shells().count(),
        3,
        "one instance, patterned twice, plus the shelf"
    );
    let want = 2.0 * POST_VOLUME + SHELF_VOLUME;
    let props = pncad::topo::mass_properties(&body, tol).expect("mass properties");
    assert!(
        (props.volume - want).abs() < 1e-12,
        "the layout gathers {want} m^3 of material, measured {}",
        props.volume
    );
    println!(
        "   [layout] {} instantiated solid(s) from 2 part documents; V = {:.6} m^3 \
         (2 x post + shelf, exact); {} product names",
        body.shells().count(),
        props.volume,
        names.iter().count()
    );

    // A name lookup a user actually does: "where is the second
    // patterned post's end face?" The answer is one instance-qualified
    // name away — the pattern's `Instance(i)` segment wrapping the
    // part's own cap name (N1 x the GQ4 wrapper).
    // The name NESTS rather than concatenating: `Instance(i)` carries
    // the instance's own name as its argument, which carries the
    // part's name as ITS argument. So the pattern reads as three
    // wrappers deep — pattern index, then instance, then the part's
    // own cap.
    let cap_of_part =
        NamePat::of_kind(EntityKind::Face).seg(SegPat::tag(SegTag::Cap).side(CapEnd::Top));
    let caps = pncad::select::select(
        &ev,
        pattern,
        &Selector::of(
            NamePat::of_kind(EntityKind::Face).seg(
                SegPat::tag(SegTag::Instance).of([NamePat::of_kind(EntityKind::Face)
                    .seg(SegPat::tag(SegTag::InPart).of([cap_of_part]))]),
            ),
        ),
    );
    let indexed: Vec<&StableName> = caps
        .iter()
        .filter(|n| matches!(n.path.first(), Some(RoleSeg::Instance { i: 1, .. })))
        .collect();
    assert_eq!(
        indexed.len(),
        1,
        "instance 1 has exactly one post top cap: {caps:?}"
    );
    let pose = pncad::select::face_frame(&ev, pattern, indexed[0])
        .expect("the named face answers with its frame");
    println!(
        "   [layout] name lookup: pattern instance 1's post cap sits at \
         ({:.3}, {:.3}, {:.3}) m",
        pose.origin.x, pose.origin.y, pose.origin.z
    );
    // One 200 mm pattern step along +y puts instance 1's post between
    // y = 0.2 and y = 0.2 + section; its cap frame's origin lies on
    // that face.
    assert!(
        (0.2..=0.2 + POST_SECTION).contains(&pose.origin.y),
        "the second post is one 200 mm step along +y, measured {}",
        pose.origin.y
    );

    // A5's disjoint half: the at-rest gate over the gathered product.
    // No mate declares anything here and nothing touches, so the
    // kernel's tier-3' door passes outright — which is exactly what
    // "disjoint assemblies validate today" means.
    let assembly = assemble(doc, &ev, tol)
        .unwrap_or_else(|e| panic!("the disjoint layout must pass the at-rest gate: {e}"));
    assert!(
        assembly.minted.is_empty(),
        "a mate-less assembly mints no declarations"
    );
    println!("   [layout] A5 at-rest gate: PASSED (disjoint, per-solid checks, 0 declarations)");

    SceneBody::seamed(
        "benchlayout",
        [0.62, 0.51, 0.36],
        assembly.body,
        assembly.contacts,
    )
}

/// The assembled stand: the mates place the clusters, mint their
/// declarations, and the gate reports what it could decide.
fn stand_scene(ws: &Workspace, stand: &Stand, tol: Tol) -> SceneBody {
    let ev = run(&stand.doc, &with_store(ws), tol);

    // The solve, read the way an author reads it: which instance is
    // the cluster's gauge, and what role each mate took (A11 rules
    // 3-4 — tree mates DETERMINE, the rest DECLARE).
    let poses = solve_document(&stand.doc, tol);
    let gauge = poses.gauge(stand.shelf_i).expect("the shelf is placed");
    assert_eq!(
        gauge, stand.post_a,
        "the cluster's gauge is its earliest instance in document order"
    );
    for mate in [stand.mate_1, stand.mate_2] {
        assert!(
            poses.fault(mate).is_none(),
            "a determined mate records no fault"
        );
    }
    println!(
        "   [stand] one placement cluster of 3 instances, gauge = node {}; \
         2 mates, roles {:?}/{:?} — the shelf and the far post carry NO authored frame",
        gauge.0,
        poses.role(stand.mate_1).expect("mate 1 is live"),
        poses.role(stand.mate_2).expect("mate 2 is live"),
    );

    // Where the mates put the far post: SOLVED, composed outward from
    // the gauge along the mate tree, never stored. The registry holds
    // one frame for the whole cluster, and it is the gauge's.
    let solved = poses
        .placement(&stand.doc, stand.post_b)
        .expect("the far post is placed");
    let want = [
        SEAT_B[0] - SEAT_A[0],
        (SHELF_DEPTH - POST_SECTION) / 2.0,
        0.0,
    ];
    assert!(
        solved
            .translation
            .iter()
            .zip(want)
            .all(|(got, want)| (got - want).abs() < 1e-12),
        "the far post's solved translation is {:?}, expected {want:?}",
        solved.translation
    );
    // And the ROTATION, which is the half a translation check cannot
    // see: both mates align +z with +z at zero clocking, so composing
    // out from the gauge must leave the post's own axes unturned. A
    // solve that rotated the post and still landed its seating point
    // would pass the translation check and put the part in sideways.
    assert_eq!(
        solved.columns,
        Frame::IDENTITY.columns,
        "aligned frame-coincidence mates compose to no net rotation"
    );
    assert!(
        !stand.doc.placements().contains_key(&stand.post_b),
        "a mated instance carries no frame of its own (A11 rule 2)"
    );

    let (gathered, _) = product_of(&stand.doc, &ev, tol);
    assert_eq!(gathered.shells().count(), 3, "two posts and a shelf");
    let want = 2.0 * POST_VOLUME + SHELF_VOLUME;
    let props = pncad::topo::mass_properties(&gathered, tol).expect("mass properties");
    assert!(
        (props.volume - want).abs() < 1e-12,
        "the stand gathers {want} m^3, measured {}",
        props.volume
    );

    let gate = at_rest(&stand.doc, &ev, tol);
    println!(
        "   [stand] the mates minted {} declaration(s) into the product's contact record \
         set, at FACE granularity; A5 at-rest gate: {}",
        gate.minted(),
        gate.verdict.describe()
    );
    assert_eq!(
        gate.minted(),
        2,
        "one record per solved mate (A3's minting)"
    );
    // ASSERTED, not merely printed: this is #1063's visible acceptance.
    // The stand is the natural drawing of a bench — two posts seated
    // FLUSH with the shelf's ends — and until the census could answer a
    // declared cross-instance pair it reached the frontier and no
    // further. A scene that only PRINTED its verdict would keep saying
    // so with the sentence and the geometry drifting apart, which is
    // the shape of the demo bug this file exists to avoid.
    assert!(
        matches!(gate.verdict, AtRestVerdict::Certified),
        "the flush-seated stand CERTIFIES at the A5 gate: {}",
        gate.verdict.describe()
    );

    SceneBody::at_rest("bench", [0.55, 0.44, 0.30], gate.body, gate.contacts)
}

/// What the A5 gate decided — the two arms a caller must tell apart,
/// as a value the scenes match on.
///
/// A verdict is NOT a count. The gate's two non-refusing arms carry
/// different things (`Ok` carries the minted declarations; the
/// frontier carries findings and no minted list), so a single number
/// standing for "how many" would mean something different per arm —
/// and the number a scene wants for MINTING is neither: it is the
/// record set's own size, which both arms carry.
enum AtRestVerdict {
    /// Every declaration certified.
    Certified,
    /// Nothing refuted and nothing certified: the census declined
    /// every declared face. Carries how many it declined.
    Uncertified { declined: usize },
}

impl AtRestVerdict {
    /// The sentence a scene prints.
    fn describe(&self) -> String {
        match self {
            Self::Certified => "PASSED (every declaration certified)".to_string(),
            Self::Uncertified { declined } => format!(
                "UNCERTIFIED — {declined} declared face(s) neither certified nor refuted \
                 (the declared direction's frontier: nothing was decided about this \
                 geometry either way)"
            ),
        }
    }
}

/// The gate's product, its record set, and its verdict.
struct AtRest {
    body: Body<f64>,
    contacts: pncad::topo::ContactRecords,
    verdict: AtRestVerdict,
}

impl AtRest {
    /// **How many declarations were minted** — the size of the record
    /// set the gate was handed, which is what a mate's minting
    /// produces (A3) and the one quantity both arms carry.
    fn minted(&self) -> usize {
        self.contacts.patches.len()
    }
}

/// Runs the A5 gate.
///
/// `Ok` and `Uncertified` are BOTH accepted, and the difference is
/// reported rather than asserted. Since #1063 a declared PLANAR
/// cross-instance pair certifies on its shared world carrier, so this
/// stand returns `Ok`; the `Uncertified` arm stays because it is still
/// the honest answer for everything the carrier arm does not reach — a
/// declared CURVED cross-instance pair, or a planar pair whose two
/// descriptions disagree over the pair's own extent. What is NOT
/// accepted is `AtRest` — a finding AGAINST the document — which is
/// the arm that means the declarations do not hold, and which the
/// update walk deliberately provokes.
///
/// # What the frontier's observable does and does not say
///
/// The finding that arm carries is `CensusUnsupported` naming a face.
/// The declared-patch loop emits that from MORE THAN ONE door — the
/// carrier-identity check, the chart-region inventory and the carrier
/// tilt row all decline through it — so the observable a caller sees
/// does NOT say which of them declined, and this demo does not claim
/// to know. What holds either way is the whole content of the
/// frontier: nothing was decided about the geometry, in either
/// direction. Separating the doors would take a probe against the
/// census, which is the census's unit to write, not this scene's.
fn at_rest(doc: &ProfileDoc, ev: &Evaluation<f64>, tol: Tol) -> AtRest {
    match assemble(doc, ev, tol) {
        Ok(Assembly { body, contacts, .. }) => AtRest {
            body,
            contacts,
            verdict: AtRestVerdict::Certified,
        },
        // Every finding here is a `Declined` by the arm's own
        // construction (`assembly.rs` routes a refusal to `AtRest` the
        // moment one finding is not), so re-checking that would be
        // asserting the constructor rather than the geometry. The arm
        // SPLIT is the check.
        Err(AssemblyError::Uncertified { contacts, findings }) => {
            let (body, _) = product_of(doc, ev, tol);
            AtRest {
                body,
                contacts: *contacts,
                verdict: AtRestVerdict::Uncertified {
                    declined: findings.len(),
                },
            }
        }
        Err(other) => panic!("the stand's declarations must not be refuted: {other}"),
    }
}

// ---- The refusal walk ----

/// Everything v1 does NOT do, met the way an author meets it and
/// printed with the recourse the library itself gave.
///
/// Fail-loud is the design, so each of these is EVIDENCE: a refusal
/// that stopped being typed, or stopped naming its subject, breaks
/// this walk.
fn refusals(ws: &Workspace, parts: &Parts, tol: Tol) {
    let (post, shelf) = (parts.post, parts.shelf);
    let (post_top, shelf_bottom) = (&parts.post_top, &parts.shelf_bottom);
    println!("\n-- the v1 boundary, walked: four refusals an author actually hits --");

    // (1) UNDER-DETERMINED. One planar rest between two parts fixes
    // the seating plane and nothing else — the pair may still slide
    // and spin in it. A11 rule 4 requires every TREE mate to
    // determine, so the solve refuses and names the residual in class
    // vocabulary rather than picking a pose.
    let under = stand_doc(
        post,
        shelf,
        post_top,
        shelf_bottom,
        MatePrimitive::PlanarRest { offset: 0.0 },
        tol,
    );
    let poses = solve_document(&under.doc, tol);
    let fault = poses
        .fault(under.mate_1)
        .expect("a planar rest alone does not determine the pair");
    assert!(
        matches!(fault, MateFault::Under { .. }),
        "an under-determined tree mate is the UNDER refusal, got {fault:?}"
    );
    println!("   (1) under-determined: {fault}");

    // (2) CONTRADICTORY. Two mates on ONE pair intersect their cosets
    // exactly; two rests at different standoffs meet nowhere, and the
    // refusal names both mates, the predicate that decided, and the
    // measured clash.
    let mut contra = stand_doc(
        post,
        shelf,
        post_top,
        shelf_bottom,
        MatePrimitive::FrameCoincidence,
        tol,
    );
    let clash = insert(
        &mut contra.doc,
        Node::Mate {
            a: in_part(contra.post_a, post_top),
            b: in_part(contra.shelf_i, shelf_bottom),
            class: ContactClass::Rest,
            alignment: Alignment {
                a: mate_frame([POST_SECTION / 2.0, POST_SECTION / 2.0, POST_HEIGHT]),
                // The same pair, seated 10 mm higher: the author has
                // said two things that cannot both be true.
                b: mate_frame([SEAT_A[0], SEAT_A[1], SEAT_A[2] - 0.01]),
                primitive: MatePrimitive::FrameCoincidence,
                sense: AxisSense::Aligned,
                clocking: None,
            },
        },
        tol,
    );
    let poses = solve_document(&contra.doc, tol);
    let fault = poses
        .fault(clash)
        .or_else(|| poses.fault(contra.mate_1))
        .expect("two rests at different heights cannot both hold");
    assert!(
        matches!(fault, MateFault::Contradictory { .. }),
        "an empty coset intersection is the CONTRADICTORY refusal, got {fault:?}"
    );
    println!("   (2) contradictory: {fault}");

    // (3) OUTSIDE v1's VOCABULARY. `Tangent` solves — a tangency pins
    // a coset like any other primitive — and then has nowhere to go
    // at rest: its kernel record is a `CurveContact` keyed by the
    // witness EDGE along which the two surfaces touch, and an assembly
    // at rest has no such edge, because nothing zipped the instances
    // together. The mint door refuses typed and says so in the class's
    // own terms.
    let mut tangent = stand_doc(
        post,
        shelf,
        post_top,
        shelf_bottom,
        MatePrimitive::FrameCoincidence,
        tol,
    );
    edit(
        &mut tangent.doc,
        &DocEdit::DeleteNode { id: tangent.mate_2 },
        tol,
    );
    let mut swapped = tangent.doc.clone();
    if let Some(Node::Mate {
        a, b, alignment, ..
    }) = tangent.doc.node(tangent.mate_1).cloned()
    {
        edit(
            &mut swapped,
            &DocEdit::DeleteNode { id: tangent.mate_1 },
            tol,
        );
        insert(
            &mut swapped,
            Node::Mate {
                a,
                b,
                class: ContactClass::Tangent,
                alignment,
            },
            tol,
        );
    }
    let ev = run(&swapped, &with_store(ws), tol);
    let err = assemble(&swapped, &ev, tol).expect_err("a Tangent mate has no at-rest record");
    assert!(
        matches!(err, AssemblyError::NoAtRestRecord { .. }),
        "the class table's mint half is what refuses, got {err}"
    );
    println!("   (3) outside v1's at-rest vocabulary: {err}");

    // (4) A REFERENCE PINNING THE WRONG VERSION. An assembly is a
    // self-contained reproducible value: a part that changed on disk
    // does not retarget it. Asking the store for a version it no
    // longer holds is the typed pin refusal, and its recourse names
    // the edit that legitimately moves a pin.
    //
    // THE TRIGGER HERE IS HAND-BUILT, and no user builds one: pairing
    // one document's id with another's pin is a `DocRef` nobody would
    // author. It is here because it reaches the refusal in one line
    // with no store mutation, so the walk can state the refusal's
    // shape beside the other three. The USER-REACHABLE instance of the
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
    println!("   (4) wrong pin: {err}");
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

    let part_id = DocumentId::derive("pncad-demo-shelf-cell");
    let out = split(layout, &BTreeSet::from([shelf_i]), part_id, tol)
        .expect("cutting one whole cluster out is legal");
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
    // sample: kept names verbatim, cut names re-anchored under the
    // remainder's new instance.
    let mapped = out.node_map[&shelf_i];
    let mut crossed = 0usize;
    for (name, _) in before_names.iter() {
        let expected = if name.node == shelf_i {
            crossed += 1;
            let RoleSeg::InPart { of } = &name.path[0] else {
                panic!("an instance-minted product name wraps a part-local one");
            };
            in_part(
                out.instance,
                &StableName {
                    kind: name.kind,
                    node: mapped,
                    path: vec![RoleSeg::InPart { of: of.clone() }],
                },
            )
        } else {
            name.clone()
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
    // splice mints FRESH host ids, so the cut instance comes back as a
    // different node. A4 asks for name-resolution identity, not
    // arena-key identity, so the correspondence is the composition of
    // the two recorded maps — split's, then inline's — and every
    // pre-split name must resolve through it.
    let back = inline(&out.remainder, out.instance, ws, tol).expect("the instance inlines back");
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
    let restored = back.node_map[&mapped];
    let mut returned = 0usize;
    for (name, _) in before_names.iter() {
        let expected = if name.node == shelf_i {
            returned += 1;
            StableName {
                kind: name.kind,
                node: restored,
                path: name.path.clone(),
            }
        } else {
            name.clone()
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
    // The cluster frame the split hoisted onto the instance is put
    // back on the restored node, bit for bit — placement is document
    // data, and a round trip that dropped it would still pass every
    // name check above while moving the part.
    assert!(
        back.doc
            .placement(restored)
            .bit_eq(&layout.placement(shelf_i)),
        "the round trip restores the cluster frame exactly"
    );
    println!(
        "   inline: {} node(s) spliced back, {} recorded edit(s); all {} product names \
         resolve through the two recorded node maps, the table is the same size, and the \
         hoisted cluster frame comes back bit-exact",
        back.node_map.len(),
        back.edits.len(),
        before_names.iter().count()
    );

    // THE SECOND CUT, probed rather than assumed: the patterned posts.
    //
    // The invariant under test is that split and inline are INVERSES
    // for every legal cut, and the shape most likely to break it is
    // this one — the cut hoists the post cluster's authored frame onto
    // the remainder's instance, and `inline` refuses a non-identity
    // frame whose part's roots are not themselves instances
    // (`UnplaceableFrame`), which a Pattern root is not. It does NOT
    // refuse here, because the hoist leaves the pattern's own recipe
    // able to express the placement; the arms below say which answer
    // this tree gave rather than asserting one, so a change in either
    // direction is reported at the scene instead of passing silently.
    let posts_id = DocumentId::derive("pncad-demo-posts-cell");
    let post_i = *layout
        .order()
        .iter()
        .find(|&&id| matches!(layout.node(id), Some(Node::InstantiatePart { .. })))
        .expect("the layout has a post instance");
    let pattern = *layout
        .order()
        .iter()
        .find(|&&id| matches!(layout.node(id), Some(Node::Pattern { .. })))
        .expect("the layout has a pattern");
    match split(layout, &BTreeSet::from([post_i, pattern]), posts_id, tol) {
        Ok(posts) => {
            ws.create(&posts.part, tol)
                .expect("the posts cell is stored");
            match inline(&posts.remainder, posts.instance, ws, tol) {
                Ok(_) => println!(
                    "   second cut: the patterned-post cell splits out AND inlines back \
                     (the hoisted cluster frame is expressible in the part's own recipe)"
                ),
                Err(e @ InlineError::UnplaceableFrame { .. }) => println!(
                    "   second cut (gap): the patterned-post cell splits out but does NOT \
                     inline back — a subtree a user can cut is then not one they can put \
                     back: {e:?}"
                ),
                Err(other) => panic!("unexpected inline refusal: {other:?}"),
            }
        }
        Err(e) => {
            println!("   second cut (gap): the patterned-post cell does not split at all — {e:?}")
        }
    }
}

// ---- The update door (A13) ----

/// Accepts a new version of a part, the way A13 says an author does:
/// the per-reference primitive, the whole-document elaboration over
/// it, the mixed-pin LINT in between, and re-verification at every
/// evaluation.
fn update_door(ws: &mut Workspace, stand: &Stand, shelf: DocRef, tol: Tol) {
    println!("\n-- the update door: moving a pin is a recorded edit --");
    let before = run(&stand.doc, &with_store(ws), tol);
    let (before_body, _) = product_of(&stand.doc, &before, tol);
    let before_volume = pncad::topo::mass_properties(&before_body, tol)
        .expect("mass properties")
        .volume;

    // The part changes on disk: a thicker board. The shelf is modelled
    // from its UNDERSIDE up, so the face the mates seat on does not
    // move — which is what keeps the assembly fitting (see the gap
    // note in `stops`).
    let mut thicker = ws.resolve(&shelf, tol).expect("the shelf resolves");
    edit(
        &mut thicker,
        &DocEdit::SetDocParamValue {
            name: ParamName::new("thickness"),
            value: DocParamValue::Continuous(SHELF_THICKNESS * 1.5),
        },
        tol,
    );
    ws.resave(&thicker, tol).expect("the new version is stored");

    // Until the pin moves, the assembly still means what it meant: the
    // reference names a version the store no longer holds, and the
    // evaluation says so instead of silently taking the new one.
    let stale = run(&stand.doc, &with_store(ws), tol);
    let refused = stale
        .node_error(stand.shelf_i)
        .expect("an out-of-date pin is surfaced, never silently retargeted");
    println!(
        "   before the edit: the instance refuses — {}",
        pin_fault(refused)
    );

    // GAP (#947), in the message a user reads: the recourse paragraph
    // arrives TWICE. `WorkspaceError::PinMismatch`'s own Display
    // already ends on `PIN_MISMATCH_RECOURSE`, and the `PartResolver`
    // impl appends it again when it classifies the failure for the
    // kernel.
    assert_eq!(
        refused
            .kind
            .to_string()
            .matches(PIN_MISMATCH_RECOURSE)
            .count(),
        2,
        "the doubled recourse is what this line records (#947); ONE copy means it was \
         fixed, and this count must be flipped to 1 in that same change"
    );
    println!(
        "   note (gap): that message carries its recourse paragraph twice — the store's \
         Display ends on it and the seam classifier appends it again"
    );

    // The elaboration: "update this document everywhere", one recorded
    // per-reference edit per site, applied as a group.
    let edits = update_to_store(&stand.doc, shelf.id, ws, tol).expect("the store has a new pin");
    assert_eq!(edits.len(), 1, "the shelf is referenced once");
    let mut updated = stand.doc.clone();
    for e in &edits {
        assert!(
            matches!(e, DocEdit::UpdateReference { .. }),
            "the elaboration is per-reference primitives and nothing else"
        );
        edit(&mut updated, e, tol);
    }

    let after = run(&updated, &with_store(ws), tol);
    // A13 clause 4: the pin move triggers ordinary re-evaluation, and
    // the crossing declarations go back through the gate against the
    // NEW geometry. What the gate then decides is its own business —
    // saying "re-verified" and reporting the frontier in one breath
    // would claim a verdict the frontier explicitly does not give.
    let gate = at_rest(&updated, &after, tol);
    let (after_body, _) = product_of(&updated, &after, tol);
    let after_volume = pncad::topo::mass_properties(&after_body, tol)
        .expect("mass properties")
        .volume;
    assert!(
        after_volume > before_volume,
        "the new version carries more material"
    );
    println!(
        "   after {} recorded UpdateReference edit(s): V {:.6} -> {:.6} m^3; the {} \
         declaration(s) were re-minted against the new geometry and put back through \
         the gate — {}",
        edits.len(),
        before_volume,
        after_volume,
        gate.minted(),
        gate.verdict.describe()
    );

    // A13 clause 3: two pins of one document id in one assembly is
    // LEGAL and sometimes intended (a staged migration), so it is a
    // lint, not a refusal. The post is referenced twice; move ONE with
    // the primitive and the lint reports the multiplicity with the
    // nodes holding each pin.
    let mut shorter = ws
        .resolve(
            &DocRef {
                id: post_ref_of(&stand.doc, stand.post_a),
                pin: post_pin_of(&stand.doc, stand.post_a),
            },
            tol,
        )
        .expect("the post resolves");
    edit(
        &mut shorter,
        &DocEdit::SetDocParamValue {
            name: ParamName::new("height"),
            value: DocParamValue::Continuous(POST_HEIGHT - 0.04),
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
        stranded.node.0,
        pin_fault(stranded)
    );

    // So finish the migration the way A13 says: the whole-document
    // ELABORATION, one recorded per-reference edit per site.
    let post_id = post_ref_of(&stand.doc, stand.post_a);
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

    // And now the fit gate does its job. Both posts are 40 mm short,
    // the mates still seat the shelf where the AUTHOR said (an
    // alignment frame is authored data, not the seating face read
    // back), so the declared rest between each post's cap and the
    // shelf's underside is REFUTED — named, with its mate.
    let ev = run(&migrated, &with_store(ws), tol);
    match assemble(&migrated, &ev, tol) {
        Err(AssemblyError::AtRest { findings }) => {
            let refuted: Vec<_> = findings
                .iter()
                .filter_map(|f| match &f.attribution {
                    Attribution::Refuted(m) => Some(m.mate),
                    _ => None,
                })
                .collect();
            assert!(
                refuted.contains(&stand.mate_1) || refuted.contains(&stand.mate_2),
                "the gate names the mate whose declaration stopped holding: {findings:?}"
            );
            println!(
                "   \"does it actually fit\": after the migration the shortened posts leave \
                 a 40 mm gap, and {} of {} finding(s) REFUTE their mate by name — the swap \
                 is verified, never assumed",
                refuted.len(),
                findings.len()
            );
        }
        other => panic!("a 40 mm gap under a declared rest must refuse: {other:?}"),
    }

    // Undo is keeping the prior value: the migrated document is one
    // the author can simply not adopt. Put both parts back, so the
    // workspace a reader opens is the one the saved assemblies pin.
    edit(
        &mut shorter,
        &DocEdit::SetDocParamValue {
            name: ParamName::new("height"),
            value: DocParamValue::Continuous(POST_HEIGHT),
        },
        tol,
    );
    ws.resave(&shorter, tol).expect("the post is restored");
    edit(
        &mut thicker,
        &DocEdit::SetDocParamValue {
            name: ParamName::new("thickness"),
            value: DocParamValue::Continuous(SHELF_THICKNESS),
        },
        tol,
    );
    ws.resave(&thicker, tol).expect("the shelf is restored");
    let restored = run(&stand.doc, &with_store(ws), tol);
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
                    err.node.0, doc_ref.id
                )
            }
            other => format!("node {}: {other:?}", err.node.0),
        },
        other => format!("node {}: {other}", err.node.0),
    }
}

/// The document id an instance references.
fn post_ref_of(doc: &ProfileDoc, node: RecipeNodeId) -> DocumentId {
    match doc.node(node) {
        Some(Node::InstantiatePart { doc_ref, .. }) => doc_ref.id,
        other => panic!("node {} is not an instance: {other:?}", node.0),
    }
}

/// The pin an instance's reference carries.
fn post_pin_of(doc: &ProfileDoc, node: RecipeNodeId) -> pncad::document::ContentPin {
    match doc.node(node) {
        Some(Node::InstantiatePart { doc_ref, .. }) => doc_ref.pin,
        other => panic!("node {} is not an instance: {other:?}", node.0),
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
/// two rendered scenes.
///
/// `work` is the store: an assembly needs a DIRECTORY of documents,
/// which is the one thing a tour scene had never needed before — see
/// the friction note in `walk_tour`.
///
/// # Gap: a mate's alignment frame is authored data
///
/// A11 makes the solve structural on purpose — no geometry
/// inspection, no numerics beyond decided predicates — so a mate's
/// two frames are numbers the AUTHOR wrote, not the seating face read
/// back. The consequence a user meets is here in plain sight: the
/// stand's mates carry the post's cap height (`POST_HEIGHT`) and the
/// shelf's seating points as literals, and an edit to the part that
/// moves that face does not move them. The mitigation this file uses
/// is the one a CAD user learns — model each part from the datum it
/// mates on, so the mated face sits at the part origin and a size
/// change never moves it — and the update walk shows what happens
/// when it is violated: the fit gate refutes the declaration and
/// names its mate. There is no door today that derives an alignment
/// frame FROM a selected face.
pub fn stops(work: &Path, tol: Tol) -> Vec<Stop> {
    let (mut ws, parts) = workspace(work, tol);
    println!(
        "   workspace: {} document(s) in {}",
        ws.documents().len(),
        ws.root().display()
    );

    let (layout, pattern, shelf_i) = layout_doc(parts.post, parts.shelf, tol);
    ws.create(&layout, tol).expect("the layout is stored");
    let stand = stand_doc(
        parts.post,
        parts.shelf,
        &parts.post_top,
        &parts.shelf_bottom,
        MatePrimitive::FrameCoincidence,
        tol,
    );
    ws.create(&stand.doc, tol).expect("the stand is stored");

    let layout_body = layout_scene(&ws, &layout, pattern, tol);
    let stand_body = stand_scene(&ws, &stand, tol);

    refusals(&ws, &parts, tol);

    println!("\n-- the round trip: a document is a value on disk --");
    round_trip(&ws, &layout, "layout", tol);
    round_trip(&ws, &stand.doc, "stand", tol);

    refactorings(&mut ws, &layout, shelf_i, tol);
    // The refactoring walk rewrote the layout's file; the store's job
    // for the rest of this stop is to answer for the PARTS, so put the
    // authored layout back where a reader will look for it.
    ws.resave(&layout, tol).expect("the layout is restored");

    update_door(&mut ws, &stand, parts.shelf, tol);

    // ONE cell for both framings. The assembled bench and the flat-pack
    // are the same two part documents answering two questions — what
    // the mates solve, and what ships — and two independently-scaled
    // panels make them look like two subjects. The flat-pack's offset
    // is AUTHORED into its placements (see `layout_doc`), where a
    // layout's placements are its subject.
    vec![Stop {
        name: "bench",
        caption: "the bench — assembled, and flat-packed".to_string(),
        montage: true,
        story: "an ASSEMBLY document: two instances of a post document and one of a \
                shelf document, the shelf SEATED on both by mates — only the gauge post \
                carries an authored frame, the other two poses are solved. Beside it \
                the same two part documents laid out for shipping: ONE post instance \
                patterned TWICE plus the shelf, nothing touching, which is A5's \
                disjoint half where the at-rest gate passes outright — the SAME two \
                parts the bench assembles, so 'the same parts, flat-packed' is true of \
                the parts and not only of the documents",
        ops: "post.pncad + shelf.pncad -> InstantiatePart x3 (pinned) -> Mate x2 \
              (Rest, frame-coincidence) -> constructive solve -> A10 product gather; \
              and InstantiatePart (explicit rotated frame) -> LinearPattern(2) + \
              InstantiatePart (explicit frame) -> A10 product gather -> assemble",
        delta: 4e-3,
        note: Some(format!(
            "ASSEMBLED: 3 solids, V = {:.6} m^3; the mates mint their Rest \
             declarations into the product's contact record set, and the A5 at-rest \
             gate CERTIFIES them — each post is seated flush with a shelf end, and a \
             declared planar pair with no shared chart is answered on the two \
             descriptions' world carrier. FLAT-PACKED: 4 solids, V = {:.6} m^3; every \
             product entity answers to an instance-qualified name (the pattern's \
             Instance(i) over the part's own)",
            2.0 * POST_VOLUME + SHELF_VOLUME,
            2.0 * POST_VOLUME + SHELF_VOLUME
        )),
        view: View {
            elev: 22.0,
            azim: -60.0,
            up: 'z',
        },
        bodies: vec![stand_body, layout_body],
    }]
}

/// The four AUTHORED documents of this scene, written into `dir` as a
/// workspace and nothing more: the two parts, the flat-pack layout,
/// and the mated stand — each pinning the version of the parts the
/// store holds when this returns.
///
/// [`stops`] is the walk, and a walk MOVES the store: its update door
/// resaves the shelf as a thicker board on purpose, so the state it
/// leaves behind is a store whose assemblies pin a version that is no
/// longer there. That is the right end state for a demo about the pin
/// gate and the wrong one for a corpus, which is why this door exists
/// beside it rather than inside it. Same authoring functions, so
/// there is still exactly one place these documents are written.
pub fn corpus(dir: &Path, tol: Tol) {
    let (mut ws, parts) = workspace(dir, tol);
    let (layout, _, _) = layout_doc(parts.post, parts.shelf, tol);
    ws.create(&layout, tol).expect("the layout is stored");
    let stand = stand_doc(
        parts.post,
        parts.shelf,
        &parts.post_top,
        &parts.shelf_bottom,
        MatePrimitive::FrameCoincidence,
        tol,
    );
    ws.create(&stand.doc, tol).expect("the stand is stored");

    // A store names its files by IDENTITY, which is a hash — so a
    // consumer that wants "the layout" needs the one thing the scan
    // cannot tell it. The manifest is that and nothing else: the label
    // each identity was derived from, beside the documents it names.
    // Not a `.pncad`, so the scan ignores it.
    let mut manifest = String::new();
    for (label, id) in [
        ("post", parts.post.id),
        ("shelf", parts.shelf.id),
        ("layout", layout.id()),
        ("stand", stand.doc.id()),
    ] {
        manifest.push_str(&format!("{label} {}\n", id.hex()));
    }
    std::fs::write(dir.join("MANIFEST"), &manifest).expect("the manifest writes");

    println!(
        "assembly corpus → {} ({} document(s))",
        dir.display(),
        ws.documents().len()
    );
    print!("{manifest}");
}
