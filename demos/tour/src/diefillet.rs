//! **The die** (M5 PR 12, acceptance shape (v)): constant-radius
//! rolling-ball fillets, made visible.
//!
//! Three stops, and since LIB-SEL1 **ONE recipe document** behind all
//! three — the die is a `Doc`, its stops are three of its nodes, and
//! every edge set it blends is a `select_where` call rather than a
//! hand-rolled loop over a kernel body. That is the whole point of the
//! scene now: the die is the tour's fillet showcase AND its
//! geometric-selector showcase, because the two are the same model.
//!
//! - **the blank** — a unit cube with all twelve edges filleted. Six
//!   shrunk planar faces, twelve quarter-cylinder blends, eight
//!   sphere-octant corner patches: 26 faces, 48 edges, 24 vertices.
//!   Every open chain terminates in a three-convex-edge corner (OQ6's
//!   one in-scope configuration), every trimline is stored
//!   `TangentIntersection` from birth, and the corner trimlines are
//!   CIRCLES — a jet-certificate class this unit had to retire into
//!   the lane before it could store them honestly. The selection is
//!   `all_edges`, the every-edge materializer.
//! - **the pips** — 21 spherical dimples on the six faces of a sharp
//!   cube, cut in ONE certified group operation (S13's closed-group
//!   arm), each ball charted with its pole along the face it is cut
//!   by.
//! - **the composed die** (M6 unit 1) — the pipped cube's twelve box
//!   edges blended IN PLACE (the pip rims carried through as rings)
//!   and all 21 rims replaced by torus bands: one body, tier-3
//!   certified, closed-form volume, watertight. The M5 frontier rows
//!   (`deviation_1_*`) are flipped in
//!   `crates/sweep/tests/m5_pr12_die.rs`; the full ladder lives in
//!   `crates/sweep/tests/m6_surgery.rs`. Its two selections are the
//!   geometric vocabulary's EXACT atoms, said as calls.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

// **This document is written in MILLIMETRES and DEGREES**, and it is
// the MIXED half of the units exhibit: `ring` authors one unit
// throughout, `heatsink` and `checks` author canonically, and this one
// carries two units and a dimensionless axis in the same recipe.
//
// It also shows the OTHER authoring door. `ring` types its numbers in
// the unit it means (`Formula::length_in(300.0, MM)`); every length
// here is DERIVED from the die's geometry, already in canonical metres,
// and only its notation is being chosen — which is `canonical_in`, and
// is exactly the shape a GUI form has, where the draft is canonical
// whatever the picker shows.

use pncad::document::{
    Bodies, BooleanValue, FrameMove, Operand, TargetAxis, corner_frame, moved_frame, save,
};
use pncad::prelude::AuthoredNode;
use pncad::prelude::{
    CancelToken, CurveKind, CurveKindSet, DEG, Dimension, Doc, DocEdit, EntityKind, EvalOptions,
    Evaluation, Formula, GeomPred, LoopProgram, MM, NamePat, Node, ProfileProgram, ProgramArcData,
    ProgramStep, ProgramTarget, RecipeNodeId, Selector, SurfaceKind, SurfaceKindSet, ValuePayload,
    WrittenLength, all_edges, apply, evaluate, select_where,
};
use pncad::topo::Body;

use crate::scalar::Scalar;
use crate::{SceneBody, TourStop, View};
use pncad::document::Orientation;
use pncad::geom_core::{Tol, Vec3};

/// The die's side, shared by both dice (`diechamfer` closes its volume
/// form over it).
pub const L: f64 = 1.0;
/// The blend size, shared by both dice: `diechamfer` sets its SETBACK
/// to this same number, which is what makes the two montage panels a
/// comparison of verbs rather than of parameters.
pub const R: f64 = 0.12;
/// The pip-rim blend radius (the composed stop's second call).
const RIM_R: f64 = 0.02;
const PIP_R: f64 = 0.09;
const PIP_H: f64 = 0.05;
const PIP_D: f64 = 0.22;

/// A length, written in the millimetres this document is authored in.
fn len(v: f64) -> Formula {
    Formula::written_length(WrittenLength::canonical_in(v, MM)).expect("a length")
}
/// An angle the face table states IN DEGREES — a quarter turn is `90`,
/// and the recipe says so.
fn ang(degrees: f64) -> Formula {
    Formula::angle_in(degrees, DEG).expect("an angle")
}
fn scl(v: f64) -> Formula {
    Formula::literal(v, Dimension::Scalar).expect("a scalar")
}

fn layout(n: u32) -> Vec<(f64, f64)> {
    let c = vec![(0.0, 0.0)];
    let diag = vec![(-1.0, -1.0), (1.0, 1.0)];
    let anti = vec![(-1.0, 1.0), (1.0, -1.0)];
    let sides = vec![(-1.0, 0.0), (1.0, 0.0)];
    match n {
        1 => c,
        2 => diag,
        3 => [diag.clone(), c].concat(),
        4 => [diag.clone(), anti.clone()].concat(),
        5 => [diag.clone(), anti.clone(), c].concat(),
        _ => [diag, anti, sides].concat(),
    }
}

/// One pip's placement: where its centre goes, and the EXACT turn
/// carrying the cube's `+Z` onto the face normal it is cut by, the
/// ball's pole stood along it (the chart discipline the plane×sphere
/// section needs).
///
/// The six turns are AUTHORED, one per face, rather than derived by a
/// runtime `cross`/`acos` branch, so the placement is data. Every turn
/// here is a whole number of quarter turns about a coordinate axis,
/// written as the fraction of a turn it is, so it turns exactly.
struct Placement {
    centre: [f64; 3],
    /// The face's outward normal, a coordinate axis either way.
    normal: [f64; 3],
    axis: TargetAxis,
    /// The turn carrying +Z onto the face's normal, as `(num, den)` of
    /// a turn.
    turn: (i64, u64),
}

impl Placement {
    /// The slide along the pole: the cube's axis the normal lies along,
    /// and the centre's coordinate on it.
    fn slide(&self) -> (TargetAxis, Formula) {
        let k = (0..3)
            .find(|&i| self.normal[i] != 0.0)
            .expect("a normal lies along an axis");
        (TargetAxis::ALL[k], len(self.centre[k]))
    }

    /// The ball's orientation: its axis, the meridian's `+y`, along the
    /// face's normal, its other axes as near the cube's own as that
    /// allows.
    fn orient(&self) -> Orientation {
        let [x, y, z] = self.normal;
        Orientation::IDENTITY.toward(Vec3::new(0.0, 1.0, 0.0), Vec3::new(x, y, z))
    }

    /// The pole's line in the cube's own frame: turned about `axis`,
    /// then slid to `centre`.
    fn moves(&self) -> Vec<FrameMove> {
        let (num, den) = self.turn;
        let spin = (num != 0).then(|| {
            let by = Formula::mul(
                Formula::ratio(num, den).expect("a fraction of a turn"),
                Formula::turn(),
            )
            .expect("a fraction times a turn is an angle");
            FrameMove::Spin(self.axis, by)
        });
        let slides = [TargetAxis::X, TargetAxis::Y, TargetAxis::Z]
            .into_iter()
            .zip(self.centre)
            .map(|(axis, at)| FrameMove::Slide(axis, len(at)));
        spin.into_iter().chain(slides).collect()
    }
}

const X: [f64; 3] = [1.0, 0.0, 0.0];
const Y: [f64; 3] = [0.0, 1.0, 0.0];
const Z: [f64; 3] = [0.0, 0.0, 1.0];
const NEG_X: [f64; 3] = [-1.0, 0.0, 0.0];
const NEG_Y: [f64; 3] = [0.0, -1.0, 0.0];
const NEG_Z: [f64; 3] = [0.0, 0.0, -1.0];

fn placements() -> Vec<Placement> {
    let h = L / 2.0;
    // pip count, outward normal, the two in-plane axes the layout
    // offsets ride, then the axis and the turn carrying +Z onto that
    // normal, as a fraction of a turn: quarter and half turns, the
    // table a reader can check against the die in their hand.
    let faces = [
        (1u32, Z, X, Y, TargetAxis::Z, (0, 1)),
        (6, NEG_Z, X, Y, TargetAxis::X, (1, 2)),
        (2, X, Y, Z, TargetAxis::Y, (1, 4)),
        (5, NEG_X, Y, Z, TargetAxis::Y, (-1, 4)),
        (3, Y, Z, X, TargetAxis::X, (-1, 4)),
        (4, NEG_Y, Z, X, TargetAxis::X, (1, 4)),
    ];
    let mut out = Vec::new();
    for (n, normal, ex, ey, axis, turn) in faces {
        // The centre stands `PIP_R - PIP_H` OUTSIDE the face plane, so
        // the cavity it cuts is exactly a cap of height `PIP_H`.
        let d = h + (PIP_R - PIP_H);
        for (u, w) in layout(n) {
            let centre = core::array::from_fn(|i| {
                h + normal[i] * d + ex[i] * (u * PIP_D) + ey[i] * (w * PIP_D)
            });
            out.push(Placement {
                centre,
                normal,
                axis,
                turn,
            });
        }
    }
    out
}

/// The pip ball's meridian: one bulge-1 semicircle from pole to pole,
/// closed by the on-axis diameter. Every vertex is ON the revolve
/// axis, which is what a sphere's meridian IS — the revolve names its
/// poles from the sweep's construction record (M9-D1), so the natural
/// three-step program is what the document authors.
fn half_disc() -> LoopProgram<Formula> {
    let p = |x: f64, y: f64| [len(x), len(y)];
    LoopProgram::Chain(vec![
        ProgramStep::At(p(0.0, -PIP_R)),
        ProgramStep::ArcTo(ProgramArcData::Bulge {
            target: ProgramTarget::Point(p(0.0, PIP_R)),
            b: scl(1.0),
        }),
        ProgramStep::LineTo(ProgramTarget::Start),
    ])
}

/// The die's recipe, and the three nodes the three stops read.
struct Die {
    doc: Doc<ProfileProgram>,
    /// Every edit `build` applied, in order.
    log: Vec<DocEdit<ProfileProgram>>,
    blank: RecipeNodeId,
    pipped: RecipeNodeId,
    composed: RecipeNodeId,
    /// What the two geometric selections actually materialized, kept
    /// for the captions: the twelve box edges and the 42 rim arcs.
    box_edges: usize,
    rims: usize,
}

fn eval(doc: &Doc<ProfileProgram>, tol: Tol) -> Evaluation<f64> {
    evaluate::<f64>(doc, None, &CancelToken::new(), &EvalOptions::default(), tol)
}

/// A document being authored, and every edit applied to it in order:
/// the log [`corpus_text`] exports.
struct Author {
    doc: Doc<ProfileProgram>,
    log: Vec<DocEdit<ProfileProgram>>,
}

impl Author {
    fn new(label: &str, tol: Tol) -> Self {
        Self {
            doc: Doc::empty_derived(label, tol),
            log: Vec::new(),
        }
    }

    fn insert(&mut self, node: AuthoredNode, tol: Tol) -> RecipeNodeId {
        let edit = DocEdit::InsertNode {
            node: Box::new(node),
            fresh: Vec::new(),
        };
        let applied = apply(&self.doc, &edit, tol).expect("the edit applies");
        self.doc = applied.doc;
        self.log.push(edit);
        applied.record.minted.expect("insert mints an id")
    }

    /// A placement's edits, applied by [`crate::world`], taken into
    /// the log; returns the placement.
    fn placed(&mut self, placed: crate::world::Placed) -> RecipeNodeId {
        self.doc = placed.doc;
        self.log.extend(placed.edits);
        placed.placement
    }
}

/// **The die's SOURCE SOLID**, as recipe nodes: the sharp cube, and
/// the cube with its 21 pips cut. Everything above the blends —
/// which is the stage BOTH dice start from, this scene's filleted one
/// and `diechamfer`'s chamfered one — so the two panels differ in the
/// verb and in nothing else.
fn cube_node(a: &mut Author, tol: Tol) -> RecipeNodeId {
    // ---- the sharp cube, [0, L]³ ----
    let cube_p = a.insert(
        Node::Profile(ProfileProgram {
            loops: vec![LoopProgram::polygon([(0.0, 0.0), (L, 0.0), (L, L), (0.0, L)]).unwrap()],
            ids: Vec::new(),
            on: None,
        }),
        tol,
    );
    a.insert(
        Node::Extrude {
            profile: cube_p.into(),
            distance: len(L),
        },
        tol,
    )
}

/// The 21 pips, cut from `cube` in one certified group operation —
/// the second half of the shared source solid.
fn pipped_node(a: &mut Author, cube: RecipeNodeId, tol: Tol) -> RecipeNodeId {
    // ---- the master ball, poled along its own +y ----
    let ball_p = a.insert(
        Node::Profile(ProfileProgram {
            loops: vec![half_disc()],
            ids: Vec::new(),
            on: None,
        }),
        tol,
    );
    let ball = a.insert(
        Node::Revolve {
            profile: ball_p.into(),
            // The pole axis: the meridian's own +y through (0, 0).
            axis_through: [
                Formula::zero(Dimension::Length),
                Formula::zero(Dimension::Length),
            ],
            axis_heading: Formula::turn_over(4).expect("a quarter turn"),
            // A full turn, in the degrees this document is written in.
            angle: ang(360.0),
        },
        tol,
    );

    // ---- 21 placements, unioned into ONE cutting tool ----
    // The 21 placements are the MEMBERS of ONE `Node::Union`, which is
    // how the recipe layer says "these bodies, fused". A union names
    // each pip by its own member EDGE, so a pip's rim name records
    // nothing about the other twenty and dropping one pip leaves their
    // names — and the rim fillet's frozen selection — standing. That
    // is the acceptance row this scene exists to carry
    // (`docm3_union::removing_any_pip_leaves_both_die_fillets_resolving`).
    //
    // Fusing them as a group is also what keeps the cut off the extent
    // scan's refusal path: cutting the pips one at a time presents a
    // body already carrying a TRIMMED sphere face as the next operand.
    // That is a CHOICE rather than a requirement — a trimmed sphere
    // face is served through the sphere chart's own [azimuth] ×
    // [latitude] window, and the closed-group certificate is asked only
    // where it is used.
    // Each pip's pole stands on a line stated in the cube's own frame,
    // read off its corner at the origin: the edge up its `z` and the
    // corner along its `x`.
    let own = corner_frame(
        &a.doc,
        &Operand::from(cube),
        [0.0; 3],
        [0.0, 0.0, L],
        [L, 0.0, 0.0],
        &EvalOptions::default(),
        tol,
    )
    .expect("the cube has its corner at its own origin");
    let members: Vec<RecipeNodeId> = placements()
        .into_iter()
        .map(|p| {
            let frame = moved_frame(own.clone(), &p.moves());
            let placed = crate::world::on_axis(&a.doc, ball, frame, p.slide(), p.orient(), tol);
            a.placed(placed)
        })
        .collect();
    let tool = a.insert(
        Node::Union {
            members: Bodies::Spelled(members.into_iter().map(Into::into).collect()),
        },
        tol,
    );
    a.insert(
        Node::Subtract {
            from: cube.into(),
            tool: tool.into(),
        },
        tol,
    )
}

/// **The source solid as BODIES** — `(sharp cube, pipped cube)` off
/// one evaluation of the shared recipe above.
///
/// `diechamfer` starts here: it exercises `chamfer_edges` from the
/// PLAIN-body seat, so it takes the source out as geometry rather than
/// continuing this document. (`Node::Chamfer` exists since LIB-G16 —
/// that scene's choice is about which seat it is evidence for, not
/// about an absent node.) What it can share is the model above the
/// blends, and this is that door.
pub fn source_bodies(tol: Tol) -> (Body<f64>, Body<f64>) {
    let mut a = Author::new("die-source", tol);
    let cube = cube_node(&mut a, tol);
    let pipped = pipped_node(&mut a, cube, tol);
    let ev = eval(&a.doc, tol);
    (body_at(&ev, cube, tol), body_at(&ev, pipped, tol))
}

/// Builds the die document, materializing each blend's edge set
/// against the evaluation of the recipe SO FAR — evaluate, select,
/// store, which is the whole authoring loop the document layer has.
///
/// The selections are frozen at authoring time on purpose: a stored
/// "every plane–sphere rim" that re-ran on every edit would silently
/// grow under an upstream change, which is the staleness the freeze
/// exists to prevent (`pncad::select` module docs).
fn build(tol: Tol) -> Die {
    let mut a = Author::new(DOC_LABEL, tol);
    // Every geometric selection in this file runs over EDGES.
    let edges = Selector::of(NamePat::of_kind(EntityKind::Edge));

    let cube = cube_node(&mut a, tol);

    // ---- the blank: EVERY edge of the cube, at one radius ----
    // No predicate needed — "all of them" has its own door.
    let all_twelve = all_edges(&eval(&a.doc, tol), cube);
    let blank = a.insert(Node::fillet(cube, len(R), all_twelve), tol);

    let pipped = pipped_node(&mut a, cube, tol);

    // ---- the surgery, both blends selected GEOMETRICALLY ----
    //
    // P10's alphabet, said in the ratified vocabulary as `select_where`
    // CALLS (SELECT-DESIGN §§1-2). Both atoms are EXACT — they read the
    // carrier's enum tag — so they are total: no funnel, no margin, and
    // `expect` here is a statement about the atoms, not optimism.
    let params = a.doc.var_env::<f64>();

    // The twelve box edges: the only LINES in the pipped cube. Every
    // pip cavity contributes circles (two rim arcs, two meridian
    // seams), so the carrier tag alone separates them.
    let box_edges = select_where(
        &eval(&a.doc, tol),
        pipped,
        &edges,
        &[GeomPred::CurveKind(CurveKindSet::just(CurveKind::Line))],
        &params,
        tol,
    )
    .expect("EXACT atoms are total");
    let blanked = a.insert(Node::fillet(pipped, len(R), box_edges.clone()), tol);

    // The 21 pip rims: plane against sphere, UNORDERED across the edge
    // (the atom takes two SETS, not a left and a right). What this
    // EXCLUDES is the point — the two cavity meridians of each pip read
    // `(Sphere, Sphere)` and match neither set, and `fillet_edges`
    // would refuse them `TangentialEdge` at a margin of exactly zero,
    // correctly, at any radius: two half-cap faces sharing ONE sphere
    // surface have no dihedral wedge to roll a ball into. The
    // exclusion is by DESCRIPTION, not by a hand-maintained omission.
    let rims = select_where(
        &eval(&a.doc, tol),
        blanked,
        &edges,
        &[GeomPred::AdjacentKinds(
            SurfaceKindSet::just(SurfaceKind::Plane),
            SurfaceKindSet::just(SurfaceKind::Sphere),
        )],
        &params,
        tol,
    )
    .expect("EXACT atoms are total");
    let composed = a.insert(Node::fillet(blanked, len(RIM_R), rims.clone()), tol);

    Die {
        doc: a.doc,
        log: a.log,
        blank,
        pipped,
        composed,
        box_edges: box_edges.len(),
        rims: rims.len(),
    }
}

/// The recipe's bodies at any tour scalar — the scene's geometry is
/// the EVALUATION now, so this is where the generic lives (the
/// `heatsink` precedent).
fn body_at<S: Scalar>(ev: &Evaluation<S>, id: RecipeNodeId, tol: Tol) -> Body<S> {
    match &ev.value(id).expect("the node evaluated").payload {
        ValuePayload::Body(b) => crate::world::in_its_space(b, tol),
        ValuePayload::Boolean(BooleanValue::Body { body, .. }) => {
            crate::world::in_its_space(body, tol)
        }
        other => panic!("expected a body, got {other:?}"),
    }
}

/// The document label [`build`] authors under, and therefore the
/// identity the exported save file replays from.
const DOC_LABEL: &str = "die";

/// **The composed die as a SAVE FILE**, so this scene stays the one
/// place the model is written down.
///
/// `crates/editor-core`'s Band 4 corpus registers this document. That
/// crate cannot call [`build`] — `demos/tour` is a detached cargo
/// workspace and the kernel must never depend on demo tooling — so the
/// document crosses as BYTES: written here, committed there,
/// regenerated by this door.
///
/// The file is an ordinary `.pncad`, written through the same
/// `document::save` the gallery uses, but with the EMPTY document as
/// snapshot and the whole model as an edit LOG. That split is what
/// makes the bytes usable at a tolerance other than the one they were
/// written at: a snapshot records its ε and refuses to load against a
/// different process ε, while an edit log records none — so a consumer
/// that replays the log into a document of its own gets the same
/// recipe at every ε row.
///
/// The exported document is [`gallery_document`]'s — the scene as a
/// DOCUMENT, the composed die placed and the blank left unplaced — for
/// that function's own reason, which does not stop being true when the
/// consumer is a test corpus instead of a viewer: a corpus evaluates,
/// gathers and round-trips its documents, and its product is the die.
///
/// The log is the one [`build`] RECORDED as it applied each edit
/// (every insert, and each pip placement's mate and values), so
/// replaying it into the empty document mints the same ids in the same
/// order; that the replay plus the world placement IS the published
/// document is asserted here rather than assumed.
pub fn corpus_text(tol: Tol) -> String {
    let die = build(tol);
    let empty: Doc<ProfileProgram> = Doc::empty_derived(DOC_LABEL, tol);
    let mut edits = die.log;
    let mut replay = empty.clone();
    for edit in &edits {
        replay = apply(&replay, edit, tol)
            .expect("the recorded log replays")
            .doc;
    }
    // The world placement is the "place part" gesture: its edits are
    // applied against the replayed document, because its values name
    // the mate by the id that mate's own edit mints.
    let placed = crate::world::place_in_world(&replay, die.composed, tol);
    edits.extend(placed.edits);
    let product = crate::world::set_product(&placed.doc, &[placed.placement.into()], tol);
    edits.extend(product.edits);
    replay = product.doc;
    assert_eq!(
        replay,
        gallery_document(tol),
        "the recorded log must reproduce the document this scene publishes"
    );
    save(&empty, &edits, tol).expect("the die document saves")
}

/// This scene's recipe, as a document the GUI can open — **the
/// composed die, placed in the world**.
///
/// The same `build` the stops walk, geometric selections and all: what
/// a reader opens in the viewer is the document this scene renders,
/// not a re-authoring of it, plus the one edit that says which of its
/// bodies the document is about.
///
/// # Why the blank is not in the product
///
/// The scene has three stops and `build` authors all three, because a
/// stop renders ONE named body and three stops need three of them.
/// [`Die::blank`] is a second fillet of the same cube, consumed by
/// nothing — a narration body.
///
/// A document's product is what it sets its product to: here the
/// one copy of the die (A10). The file places the scene's
/// SUBJECT — "the composed die, which remains the sheet's die" (Ev's
/// montage curation, #218 follow-up, which ruled the two partial dice
/// out of the sheet for reading as near-duplicates) — and nothing
/// places the blank, so the blank stays a value in the file, outside
/// the product. It is the composed die's own outer shape with no pips
/// cut; placed beside the die it would plug every pip cavity and
/// count the same material twice (the #1162 defect, which the
/// at-rest check reports).
///
/// The product is one copy of the die: 89 faces, V = 0.952915, no
/// at-rest finding. The 21 pips are the members of ONE
/// `Node::Union`, so the node count is the pips, the blank, the two
/// blends and the placement — not a chain length — and the product
/// slot reads the placement's copy.
pub fn gallery_document(tol: Tol) -> Doc<ProfileProgram> {
    let die = build(tol);
    let placed = crate::world::place_in_world(&die.doc, die.composed, tol);
    crate::world::set_product(&placed.doc, &[placed.placement.into()], tol).doc
}

pub fn stops(tol: Tol) -> Vec<TourStop> {
    let die = build(tol);
    let ev = evaluate::<f64>(
        &die.doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        tol,
    );

    // The two geometric selections ARE the composed stop's claim, so
    // they are pinned as counts: twelve lines, and 21 rims × two arcs
    // (a full revolve's band is two half-faces split at the seam
    // meridians, so each rim is two arcs rather than one circle).
    assert_eq!(
        die.box_edges, 12,
        "CurveKind(Line) is exactly the box edges"
    );
    assert_eq!(
        die.rims, 42,
        "AdjacentKinds(Plane, Sphere) is exactly the 21 pip rims, two arcs each"
    );

    let blank = body_at(&ev, die.blank, tol);
    let vol = pncad::topo::mass_properties(
        &crate::booleans::at_rest(&blank, tol),
        tol,
        crate::printed(),
    )
    .unwrap()
    .enclosure
    .volume;
    let want = crate::oracles::rounded_box_volume([L; 3], R);
    assert!(
        (vol - want).abs() < 1e-9 * want,
        "the blank's volume is a closed form: {vol} vs {want}"
    );
    let (f, e, v) = (
        blank.faces().count(),
        blank.edges().count(),
        blank.vertices().count(),
    );
    assert_eq!((f, e, v), (26, 48, 24));
    let pipped = body_at(&ev, die.pipped, tol);
    let pip_vol = pncad::topo::mass_properties(
        &crate::booleans::at_rest(&pipped, tol),
        tol,
        crate::printed(),
    )
    .unwrap()
    .enclosure
    .volume;
    let composed = body_at(&ev, die.composed, tol);
    let comp = pncad::topo::mass_properties(
        &crate::booleans::at_rest(&composed, tol),
        tol,
        crate::printed(),
    )
    .unwrap();
    assert_eq!(
        pncad::topo::validate_geometric(&composed, tol),
        Ok(()),
        "the composed die is tier-3 valid"
    );
    let (cf, ce, cv) = (
        composed.faces().count(),
        composed.edges().count(),
        composed.vertices().count(),
    );
    // On each pip's planar trimline, the foot its band's slit does not
    // reach has valence two and is joined away: one vertex and one edge.
    assert_eq!((cf, ce, cv), (26 + 21 * 3, 48 + 21 * 6, 24 + 21 * 4));

    vec![
        TourStop {
            name: "diefillet",
            caption: "the die blank (rolling-ball fillets)".to_string(),
            // Standalone since the montage-v2 curation (Ev, #218
            // follow-up): the two PARTIAL dice — the blank and the
            // pipped cube — are interesting for how they work (the
            // battery, the closed-group cut), but without that context
            // they read as near-duplicates of the composed die, which
            // remains the sheet's die. Scenes and narration stay alive.
            montage: false,
            story: "every edge of a cube blended at one radius — twelve quarter-cylinders \
                    and eight sphere-octant corners",
            ops: "Node::Fillet(cube, all_edges, r = 0.12): battery first, then \
                  plane–plane → cylinder blends with sphere-octant corner patches",
            delta: 5e-3,
            note: Some(format!(
                "the validity battery runs over the INPUTS before a surface exists — six \
                 named Q1 trileans (radius headroom, face consumption, spine regularity, \
                 chain G1, convexity sign, corner configuration); the result is {f} faces, \
                 {e} edges, {v} vertices, every trimline stored TangentIntersection from \
                 birth (the corner ones on CIRCLE carriers, a jet-certificate class this \
                 unit retired into the lane by equivariance), and V = {vol:.6} m³ on the \
                 closed form"
            )),
            view: View {
                elev: 26.0,
                azim: -50.0,
                up: 'z',
            },
            bodies: vec![
                SceneBody::plain("diefillet", [0.80, 0.72, 0.55], blank).named(&ev, die.blank),
            ],
        },
        TourStop {
            name: "diepips",
            caption: "the die's pips (one group cut)".to_string(),
            // Standalone with `diefillet` (see the note there).
            montage: false,
            story: "21 spherical dimples on six faces, subtracted as a single 21-shell \
                    operand",
            ops: "cube ∖ (21 disjoint balls): S13's closed-group extent arm, each ball \
                  charted with its pole along the face it is cut by",
            delta: 5e-3,
            note: Some(format!(
                "cutting the pips one at a time used to present a TRIMMED sphere face as \
                 the next operand, which the extent scan refused typed; the sphere chart's \
                 [azimuth] × [latitude] window serves that face now, so the group cut here \
                 is a CHOICE the recipe layer's pairwise-union shape makes convenient, not \
                 a requirement. Charting a ball with a tilted pole still makes the \
                 plane×sphere section non-polar, which the split-join refuses typed. One \
                 certified operation either way: V = {pip_vol:.6} m³. At M5 this and the \
                 blank were the die's two halves; the next stop is M6's composition \
                 surgery joining them"
            )),
            view: View {
                elev: 26.0,
                azim: -50.0,
                up: 'z',
            },
            bodies: vec![
                SceneBody::plain("diepips", [0.62, 0.66, 0.78], pipped).named(&ev, die.pipped),
            ],
        },
        TourStop {
            name: "diecomposed",
            caption: "THE COMPOSED DIE (in-place surgery)".to_string(),
            montage: true,
            story: "the filleted blank, the 21 pips and the filleted pip rims in ONE body \
                    — M6's in-place edge-blend composition surgery, both blends selected \
                    by GEOMETRY",
            ops: "cube ∖ pips, then Node::Fillet twice IN PLACE: select_where(CurveKind = \
                  Line) → the twelve box edges (faces split along stored trimlines, rings \
                  carried through, octant corners grafted), then \
                  select_where(AdjacentKinds = {Plane} × {Sphere}) → all 21 rims as closed \
                  chains → torus bands (donut-style slit-seamed annuli)",
            delta: 5e-3,
            note: Some(format!(
                "M5 exited with the die as two bodies pinned at two frontiers; both are \
                 retired — door B by the surgery, door A by the circle-carrier \
                 definite-miss rider. The composed body: {cf} faces, {ce} edges, {cv} \
                 vertices, tier-3 certified, V = {:.6} m³ on the closed form \
                 (Steiner blank − 21·(cap + rim-torus term)) at zero enclosure pad. \
                 Neither blend names an edge by hand: {} lines and {} plane–sphere arcs \
                 come back from `select_where`, and the 42 co-surface meridians it does \
                 NOT return are the ones `fillet_edges` would refuse at margin zero",
                comp.enclosure.volume, die.box_edges, die.rims
            )),
            view: View {
                elev: 26.0,
                azim: -50.0,
                up: 'z',
            },
            bodies: vec![
                SceneBody::plain("diecomposed", [0.85, 0.63, 0.46], composed)
                    .named(&ev, die.composed),
            ],
        },
    ]
}
