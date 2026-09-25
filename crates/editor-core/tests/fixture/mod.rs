//! The die document fixture (spec D7): the M3 exact-oracle die —
//! 2×2×2 cube, 21 pips 0.25×0.25×`pip_depth` — authored as a recipe
//! through `apply`, evaluated by the M4 PR 2 service.
//!
//! REPORTED deviation from the PR 1 die authoring (spec D8): PR 1
//! placed all 21 pips from ONE master via rotational Transforms
//! (angles ±π/2, π). `sin`/`cos` of those angles are not exact in
//! f64, so a rotational placement cannot hit the dyadic volume oracle
//! bit-exactly. This document keeps the 21 interleaved
//! Transform+Subtract pairs but derives each face's pips from a
//! per-face master profile (6 masters), so every Transform is a
//! translation-only rigid map (rotation angle exactly 0 — an exact
//! identity in IEEE arithmetic) and the oracle stays exact. The
//! rotational Transform path is exercised separately (non-dyadic
//! assertions) in the wire tests.
//!
//! **This file is the ONE home for what a name-reading suite works an
//! evaluation with**: the die document above, the [`Recorder`] and the
//! `insert`/`step` authoring shorthands a suite builds a document
//! with, the name-authoring shorthands ([`minted`], [`fname`],
//! [`ename`], [`vname`], [`rim_edge`], [`cap_vertex`], [`pole`],
//! [`in_copy`]) and, below the banner, the reader doors over a
//! published table and body ([`table`], [`key_of`], [`face_of`],
//! [`edge_of`], [`vertex_of`], [`count`], [`point`], [`ends`],
//! [`face_vertices`], [`face_edges`]). A suite **imports a door; it
//! never copies one** — a copy diverges silently, and the divergence
//! is discovered by the row it breaks rather than by the reader of
//! either file. Where the door does not fit, the suite either widens
//! the door here or writes an adapter that DELEGATES to it; an
//! adapter never reuses a door's name, because a door's name in a
//! suite means the door.
#![allow(dead_code)]
// one instance per binary; no single consumer uses all of it
// WHY A HELPER TREE ALLOWS THESE — the one statement of it, cited by every
// other tree that carries an allow of this shape. A helper tree AUTHORS test
// documents, and a document that will not build is a test failure, not a
// value to hand back: its builders panic on a malformed fixture rather than
// thread a `Result` out to a caller whose only recourse is to unwrap it.
// Each tree names exactly the lints its own code raises, here rather than in
// the crate-root allow of whatever module loads it.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![allow(unreachable_pub)] // why: root Cargo.toml, the `unreachable_pub` stanza

/// The provenance-extended evaluation digest the verb-migration suites
/// pin their documents with — one feed, per-suite constants.
pub mod digest;

/// The part store an assembly suite instantiates through, and the
/// names an instantiated part's faces are spelled with.
pub mod resolver;

/// The whole-frame product oracle a mate suite measures a seat with.
pub mod seat;

/// The value-channel digest a cross-scalar differential reads — the one
/// feed behind every "bit-identical to the `f64` run" claim in this tree.
pub mod value_channel;

use editor_core::{
    AssemblyError, CancelToken, CapEnd, Datum, Dimension, DocEdit, DocParam, EntityKey, EntityKind,
    Entry, EvalOptions, Evaluation, Expr, LoggedEdit, LoopProgram, MateReach, NameTable, Node,
    ParamName, ProfileDoc, ProfileEdgeRef, ProfilePieces, ProfileProgram, ProfileVertexRef,
    RecipeNodeId, RefusingReach, RoleSeg, SitedRef, SolvedPoses, StableName, assemble, evaluate,
    mate_reach, solve_document,
};
use geom_core::{Point3, Tol};
use std::collections::HashSet;
use topo::{Body, EdgeKey, FaceKey, LoopBoundary, VertexKey};

/// **The evaluation, through the ordinary door** — `evaluate` at
/// `f64` with a fresh cancel token and the witness tolerance, which
/// is what every suite here wants and what none of them should spell
/// for itself.
pub fn run(doc: &editor_core::ProfileDoc, o: &EvalOptions) -> Evaluation<f64> {
    evaluate::<f64>(doc, None, &CancelToken::new(), o, Tol::witness())
}

/// **The mate solve, through the ordinary door** — levered by the
/// parts `o`'s resolver reaches, built through the evaluation's own
/// public door (`mate_reach`) at `f64` and the witness tolerance. A
/// row that solves with a store it does not hand here levers nothing:
/// every mate on a part faults in the resolver's voice, which is the
/// kernel's answer and not a fixture default.
pub fn solve(doc: &editor_core::ProfileDoc, o: &EvalOptions, tol: Tol) -> SolvedPoses {
    let reach = mate_reach::<f64>(o, tol);
    solve_document(doc, &reach, tol)
}

/// **The at-rest gate's verdict**, as a mate row wants to read it:
/// whether the assembly mints, with the minted records dropped.
///
/// # Errors
///
/// The gate's own refusal, unaltered.
pub fn gate(doc: &editor_core::ProfileDoc, ev: &Evaluation<f64>) -> Result<(), AssemblyError> {
    assemble(doc, ev, Tol::witness()).map(|_| ())
}

/// A mate head over a name this fixture built as a face, read at its
/// own mint.
///
/// A head is an `editor_core::SitedFace`, so the kind is the type's
/// and a fixture that names an edge does not compile. The `expect` in
/// [`face`] is the fixture's own claim that the name it just made is a
/// face — if it is not, the fixture is wrong and says so where it is
/// built.
pub fn head(name: StableName) -> editor_core::SitedFace {
    editor_core::SitedFace::at_mint(face(name))
}

/// The same head read at `at` rather than at its mint.
pub fn head_at(at: RecipeNodeId, name: StableName) -> editor_core::SitedFace {
    editor_core::SitedFace::new(at, face(name))
}

/// A fixture's name as an `editor_core::FaceName`.
pub fn face(name: StableName) -> editor_core::FaceName {
    editor_core::FaceName::new(name).expect("the fixture names a face")
}

/// **A name worn as copy `i` of `pattern`** — one `Instance(i)`
/// wrapper, the segment a pattern's table puts round every master
/// name it emits. Nest the calls for a nested copy.
pub fn in_copy(pattern: RecipeNodeId, i: u32, of: StableName) -> StableName {
    StableName {
        kind: of.kind,
        node: pattern,
        path: vec![RoleSeg::Instance { i, of: of.into() }],
    }
}

/// A `Transform` over `input`: a translation, and `angle` about
/// `axis`.
///
/// # Panics
///
/// If `angle` is not a finite angle literal.
pub fn xform(
    input: RecipeNodeId,
    translation: [f64; 3],
    axis: [f64; 3],
    angle: f64,
) -> Node<ProfileProgram> {
    Node::Transform {
        input,
        translation: translation.map(len),
        rotation_axis: axis.map(scl),
        rotation_angle: Expr::literal(angle, Dimension::Angle).expect("an angle literal"),
    }
}

/// The pip depth the document's `pip_depth` parameter starts at.
pub const DEPTH: f64 = 0.125;
/// The exact die volume oracle at `DEPTH` (M3).
pub const DIE_VOLUME: f64 = 7.8359375;

/// **The witnessed band a placement axis is decided under** — what
/// `Frame::rotate_then_translate` asks the direction door with. Rows
/// whose axis is a literal pass this and unwrap; a row whose SUBJECT
/// is the axis decision reads the refusal instead.
/// The K funnel name a fixture decides a mate-frame axis or normal
/// under when a row builds a witness by hand — one name the fixtures
/// own, rostered in `docs/K-REPORT.md` beside the other fixture
/// mints, so a suite never names a production funnel for a decision
/// no door made.
pub const FIXTURE_MATE_AXIS: &str = "fixture_mate_axis";

pub fn band() -> geom_core::Band {
    geom_core::Band::linear(Tol::witness()).expect("the witnessed band")
}

pub fn len(v: f64) -> Expr {
    Expr::literal(v, Dimension::Length).unwrap()
}
pub fn ang(v: f64) -> Expr {
    Expr::literal(v, Dimension::Angle).unwrap()
}
pub fn scl(v: f64) -> Expr {
    Expr::literal(v, Dimension::Scalar).unwrap()
}

/// Applies an edit, returning the new doc and any minted id.
///
/// Through the REFUSING reach: an edit that moves a cluster's gauge
/// on a mated document mints a frame from the parts' extent and
/// refuses here — a row that deletes a mate or an instance of a mated
/// document steps through [`step_with`] and the store's own reach.
pub fn step(doc: ProfileDoc, edit: DocEdit<ProfileProgram>) -> (ProfileDoc, Option<RecipeNodeId>) {
    step_with(doc, edit, &RefusingReach)
}

/// [`step`] through `reach` — the store's, for an edit whose
/// maintenance mints a frame from a solve.
pub fn step_with(
    doc: ProfileDoc,
    edit: DocEdit<ProfileProgram>,
    reach: &dyn MateReach,
) -> (ProfileDoc, Option<RecipeNodeId>) {
    let applied = doc.apply(&edit, Tol::witness(), reach).unwrap();
    (applied.doc, applied.record.minted)
}

pub fn insert(doc: ProfileDoc, node: Node<ProfileProgram>) -> (ProfileDoc, RecipeNodeId) {
    let (doc, minted) = step(doc, DocEdit::InsertNode { node });
    (doc, minted.unwrap())
}

/// **The insert door's verdict on a mate**, through `reach`: the door
/// asks the solve's own per-mate admission — a frame with no definite
/// direction, the table's gaps, a rider on a coincidence decided over
/// the mated parts' extent — so a mate the solve refuses on its own
/// datum comes out of the door as its fault. `Ok` is the document
/// with the mate and its id; `Err` the id the door named and the
/// solve's fault. A rider needs the store's reach; everything else
/// decides on the datum alone, so [`RefusingReach`] serves.
pub fn at_the_door(
    doc: &ProfileDoc,
    reach: &dyn MateReach,
    node: Node<ProfileProgram>,
) -> Result<(ProfileDoc, RecipeNodeId), (RecipeNodeId, editor_core::MateFault)> {
    match doc.apply(&DocEdit::InsertNode { node }, Tol::witness(), reach) {
        Ok(applied) => {
            let id = applied.record.minted.expect("an insert mints an id");
            Ok((applied.doc, id))
        }
        Err(editor_core::EditError::MateRefused { node, fault }) => Err((node, *fault)),
        Err(other) => panic!("the door refused otherwise: {other:?}"),
    }
}

/// [`at_the_door`] for a mate the door refuses on the datum alone,
/// through the refusing reach: the fault it carries.
pub fn door_refusal(
    doc: &editor_core::ProfileDoc,
    node: Node<ProfileProgram>,
) -> editor_core::MateFault {
    match at_the_door(doc, &RefusingReach, node) {
        Err((_, fault)) => fault,
        Ok(_) => panic!("the door admitted a mate it refuses on its own datum"),
    }
}

/// **A mate one of whose heads resolves to NO member**, authored the
/// way such a head arises. The insert door asks the solve's own
/// per-mate admission, so a head that resolves to no member at insert
/// is refused there (`EditError::MateRefused`); a head can stop
/// resolving only through a LATER edit (N5) — a rebind, a shrunk
/// pattern (`SetStructuralParam` on its count), a re-pointed `Part`,
/// a deleted operand — and this is the shortest road to a head on
/// LIVE geometry. The mate enters with that head on copy 1 of a scratch
/// pattern over `anchor`, the instance its OTHER head stands on — two
/// members over one instance, so it welds nothing and no cluster
/// moves — then `DocEdit::Rebind` moves the head onto the name `node`
/// spells for it (the name-repair door checks that its target is
/// live, not that a member stands there), and the scratch pattern is
/// deleted. Nothing solves, so the refusing reach suffices, and the
/// document differs from one that inserted `node` as spelled only in
/// the id the scratch pattern consumed.
///
/// `side` is the head that resolves to nothing, spelled in `node` as
/// it is meant to read — at its own mint, which is where the rebind
/// leaves it.
pub fn insert_mate_with_stranded_head(
    doc: ProfileDoc,
    node: Node<ProfileProgram>,
    side: editor_core::MateSide,
    anchor: RecipeNodeId,
) -> (ProfileDoc, RecipeNodeId) {
    let Node::Mate {
        a,
        b,
        class,
        alignment,
    } = node
    else {
        panic!("a mate");
    };
    let (doc, scratch) = insert(
        doc,
        Node::Pattern {
            input: anchor,
            count: Expr::count(2),
            kind: editor_core::PatternKind::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: len(1.0),
            },
        },
    );
    let stand_in = in_copy(scratch, 1, resolver::in_part(anchor, CapEnd::End));
    let (stranded, a, b) = match side {
        editor_core::MateSide::A => (a, head(stand_in.clone()), b),
        editor_core::MateSide::B => (b, a, head(stand_in.clone())),
    };
    let (doc, mate) = insert(
        doc,
        Node::Mate {
            a,
            b,
            class,
            alignment,
        },
    );
    let (doc, _) = step(
        doc,
        DocEdit::Rebind {
            from: stand_in,
            to: (*stranded.name).clone(),
        },
    );
    let (doc, _) = step(doc, DocEdit::DeleteNode { id: scratch });
    let Some(Node::Mate { a, b, .. }) = doc.node(mate) else {
        panic!("the mate is live");
    };
    let now = match side {
        editor_core::MateSide::A => a,
        editor_core::MateSide::B => b,
    };
    assert_eq!(
        (now.at, &now.name),
        (stranded.at, &stranded.name),
        "the rebind left the head read where `node` spelled it"
    );
    (doc, mate)
}

/// The frame datum a profile is drawn on, as a node to insert.
///
/// The components `desc` used to bake into a `SketchPlane` are the
/// frame's own slots now, spelled the same way round: an origin and
/// the two directions sketch +x and +y point.
pub fn frame(origin: [f64; 3], u: [f64; 3], v: [f64; 3]) -> Node<ProfileProgram> {
    Node::Datum(editor_core::Datum::Frame {
        origin: origin.map(len),
        u: u.map(scl),
        v: v.map(scl),
    })
}

/// **The `SketchPlane` a frame NODE denotes**, read out of a document.
///
/// A test that builds a `profile::Profile` by hand needs the plane the
/// profile's `plane` id names, and the id alone is not it. Reads the
/// frame's authored literals and mints the SAME frame witness the
/// evaluator mints from them — Gram–Schmidt under the datum
/// boundary's own funnel name — so the plane here is the evaluator's
/// bit for bit whether or not the fixture authored an orthonormal
/// pair.
///
/// # Panics
///
/// If `plane` is not a `Datum::Frame`, if its components are not
/// literals, or if `u` and `v` span no plane.
pub fn plane_of(doc: &editor_core::ProfileDoc, plane: RecipeNodeId) -> profile::SketchPlane<f64> {
    let Some(Node::Datum(editor_core::Datum::Frame { origin, u, v })) = doc.node(plane) else {
        panic!("node {} is not a Datum::Frame", plane.0)
    };
    let read = |xs: &[Expr; 3]| {
        let c = |e: &Expr| {
            e.literal_value()
                .expect("a fixture frame's components are literals")
        };
        geom_core::Vec3::new(c(&xs[0]), c(&xs[1]), c(&xs[2]))
    };
    let (o, u, v) = (read(origin), read(u), read(v));
    profile::SketchPlane::from_frame(
        geom_core::OrthoFrame::gram_schmidt(
            geom_core::Point3::new(o.x, o.y, o.z),
            u,
            v,
            topo::DATUM_UNIT_NORM,
            geom_core::Band::linear(geom_core::Tol::witness()).expect("the witness band"),
        )
        .expect("a fixture frame's two axes span a plane"),
    )
}

/// **A profile program on the xy frame, extruded a unit, evaluated**
/// — the preamble a row that measures a profile door's answer against
/// the solid the profile swept opens with, written once.
///
/// Seven statements, and the only thing that varies between the rows
/// that write them is the loop list: mint the document, insert the
/// frame, insert the profile on it, extrude it, evaluate, then reach
/// back for the program node and the evaluated profile value. Holding
/// the document and the evaluation together is what lets the last two
/// be borrows rather than a fourth and fifth thing to unpack.
///
/// It wears no door's name: a row asks it for the DOCUMENT it
/// measures, and asks the kernel for the answer it is measuring.
pub struct Swept {
    /// The document, with the frame, the profile and the extrude on it.
    pub doc: ProfileDoc,
    /// The frame datum the profile is drawn on.
    pub plane: RecipeNodeId,
    /// The profile node.
    pub profile: RecipeNodeId,
    /// The extrude over it.
    pub ext: RecipeNodeId,
    /// The evaluation of the whole document.
    pub ev: Evaluation<f64>,
}

/// [`Swept`]'s constructor: `loops` on a fresh document named `id`.
///
/// # Panics
///
/// If the document does not build — a fixture that will not author is
/// a test failure, not a value to hand back.
pub fn wall_row(id: &str, loops: Vec<LoopProgram>) -> Swept {
    let doc = ProfileDoc::empty_derived(id, Tol::witness());
    let (doc, plane) = insert(doc, xy_frame());
    let (doc, profile) = insert(
        doc,
        Node::Profile(ProfileProgram {
            plane,
            loops,
            ids: Vec::new(),
        }),
    );
    let (doc, ext) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(1.0),
        },
    );
    let ev = run(&doc, &EvalOptions::default());
    Swept {
        doc,
        plane,
        profile,
        ext,
        ev,
    }
}

impl Swept {
    /// The program the profile node holds.
    ///
    /// # Panics
    ///
    /// If the node this built is not a profile.
    pub fn program(&self) -> &ProfileProgram {
        match self.doc.node(self.profile) {
            Some(Node::Profile(p)) => p,
            _ => panic!("the profile node this fixture inserted is a program"),
        }
    }

    /// The evaluated profile value — its validated loops, its naming
    /// anchor and its per-edge radius table.
    ///
    /// # Panics
    ///
    /// If the profile did not evaluate, or evaluated to something else.
    pub fn profile_value(&self) -> &editor_core::eval::ProfileValue<f64> {
        match &self
            .ev
            .value(self.profile)
            .expect("the fixture profile evaluates")
            .payload
        {
            editor_core::ValuePayload::Profile(pv) => pv,
            _ => panic!("the profile node's value carries a profile"),
        }
    }
}

/// The world xy frame as a node — origin at the world origin, sketch
/// +x along world +x, sketch +y along world +y.
///
/// The `SketchPlane::xy()` constant most of these suites used, spelled
/// as the node a profile now names. One per document, shared by every
/// sketch on it: that is what "the same plane" is once the plane is a
/// node, where the constant left each profile holding its own copy of
/// identical floats.
pub fn xy_frame() -> Node<ProfileProgram> {
    frame([0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0])
}

/// A profile program on `plane`, from polygon corner lists
/// (LIB-SWITCH §4i: the corpus's polygon choke point — under v4 each
/// loop is a chain program, `At(p0), LineTo(p1), …, LineTo(Start)`,
/// the VQ5 expansion at literal points).
///
/// It takes the frame's NODE rather than an origin and two vectors: a
/// profile's plane is a document node, so the caller inserts the frame
/// (with [`frame`]) and hands this the id. Two nodes where there was
/// one, which is the shape of the document now — a sketch names the
/// frame it is drawn on.
pub fn desc(plane: RecipeNodeId, loops: Vec<Vec<(f64, f64)>>) -> ProfileProgram {
    let loops = loops
        .into_iter()
        .map(|pts| LoopProgram::polygon(pts).expect("finite corners"))
        .collect();
    ProfileProgram {
        plane,
        loops,
        ids: Vec::new(),
    }
}

/// **A frame and a profile on it, inserted in that order** — the whole
/// of what a `desc(origin, u, v, loops)` call used to be, so a call
/// site that only wants "a square on the xy plane" stays one line.
///
/// Returns the doc and the PROFILE's id: the frame is scaffolding at
/// almost every call site, and one that needs its id has both nodes'
/// doors ([`frame`] and [`desc`]) to reach for instead.
pub fn on_frame(
    doc: ProfileDoc,
    origin: [f64; 3],
    u: [f64; 3],
    v: [f64; 3],
    loops: Vec<Vec<(f64, f64)>>,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, plane) = insert(doc, frame(origin, u, v));
    insert(doc, Node::Profile(desc(plane, loops)))
}

/// [`on_frame`], keeping the FRAME's id too — what a revolve needs,
/// because its axis has to be written in the same frame the profile
/// is drawn on and the axis's door names that frame.
pub fn on_frame_keeping(
    doc: ProfileDoc,
    origin: [f64; 3],
    u: [f64; 3],
    v: [f64; 3],
    loops: Vec<Vec<(f64, f64)>>,
) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let (doc, plane) = insert(doc, frame(origin, u, v));
    let (doc, profile) = insert(doc, Node::Profile(desc(plane, loops)));
    (doc, plane, profile)
}

/// An axis written in `plane`'s own 2-D coordinates — a revolve's axis
/// of revolution.
pub fn axis_in_plane(
    plane: RecipeNodeId,
    origin: (f64, f64),
    dir: (f64, f64),
) -> Node<ProfileProgram> {
    Node::Datum(Datum::AxisInPlane {
        plane,
        origin: [len(origin.0), len(origin.1)],
        direction: [scl(dir.0), scl(dir.1)],
    })
}

/// An axis-aligned square of half-width `h` centered at (cx, cy).
pub fn square(cx: f64, cy: f64, h: f64) -> Vec<(f64, f64)> {
    vec![
        (cx - h, cy - h),
        (cx + h, cy - h),
        (cx + h, cy + h),
        (cx - h, cy + h),
    ]
}

/// Applies-and-RECORDS: the edit-log author shared by the die
/// fixture and the M4 PR 8a Band 4 corpus. The saved snapshot is the
/// EMPTY document and the log is everything, so a load replays the
/// whole document through `apply`'s doors.
pub struct Recorder {
    /// The document as edited so far.
    pub doc: ProfileDoc,
    /// The recorded log.
    pub edits: Vec<editor_core::LoggedEdit<ProfileProgram>>,
}

impl Default for Recorder {
    fn default() -> Self {
        Self::new()
    }
}

impl Recorder {
    /// A recorder over the empty document.
    pub fn new() -> Self {
        Self {
            doc: ProfileDoc::empty_derived("mod", Tol::witness()),
            edits: Vec::new(),
        }
    }

    /// Applies an edit (the doors refusing is a loud test failure)
    /// and records it; returns any minted id.
    pub fn push(&mut self, edit: DocEdit<ProfileProgram>) -> Option<RecipeNodeId> {
        let applied = editor_core::apply(&self.doc, &edit, Tol::witness(), &RefusingReach)
            .expect("recorded edit must apply");
        self.edits.push(LoggedEdit {
            edit,
            maintenance: applied.cluster_rows(),
        });
        self.doc = applied.doc;
        applied.record.minted
    }

    /// Inserts a node, returning its minted id.
    pub fn insert(&mut self, node: Node<ProfileProgram>) -> RecipeNodeId {
        self.push(DocEdit::InsertNode { node }).expect("minted id")
    }

    /// **A frame and a profile drawn on it**, returning the PROFILE's
    /// id — [`on_frame`]'s shape for a recorder.
    ///
    /// It exists so a call site that wants "a square on this plane"
    /// stays one line now that saying so takes two nodes. A site that
    /// needs the frame's own id inserts the two itself.
    pub fn profile(
        &mut self,
        origin: [f64; 3],
        u: [f64; 3],
        v: [f64; 3],
        loops: Vec<Vec<(f64, f64)>>,
    ) -> RecipeNodeId {
        self.profile_keeping(origin, u, v, loops).1
    }

    /// [`Self::profile`], keeping the FRAME's id — what a revolve
    /// needs, because its axis is written in that frame.
    pub fn profile_keeping(
        &mut self,
        origin: [f64; 3],
        u: [f64; 3],
        v: [f64; 3],
        loops: Vec<Vec<(f64, f64)>>,
    ) -> (RecipeNodeId, RecipeNodeId) {
        let plane = self.insert(frame(origin, u, v));
        (plane, self.insert(Node::Profile(desc(plane, loops))))
    }
}

/// The authored die and the ids the tests address.
pub struct Die {
    pub doc: ProfileDoc,
    /// The document's full edit log (snapshot = the empty document).
    pub edits: Vec<editor_core::LoggedEdit<ProfileProgram>>,
    /// The final Subtract (the die body).
    pub final_node: RecipeNodeId,
    /// The +z face's pip-master Extrude (the poisoning target: its
    /// descendants are exactly the LAST transform + subtract).
    pub pz_extrude: RecipeNodeId,
    /// The +z pip's Transform (the incremental-edit target).
    pub pz_transform: RecipeNodeId,
    /// Total node count.
    pub n_nodes: usize,
}

/// Face frames (origin, u, v; normal = u×v points OUT of the cube
/// `[0,2]³`) and pip layouts in face coordinates, +z LAST so the
/// incremental edit's cone is minimal.
type Face = ([f64; 3], [f64; 3], [f64; 3], &'static [(f64, f64)]);
pub fn faces() -> [Face; 6] {
    // Pip grid: {0.5, 1.0, 1.5} per face axis.
    [
        // -z: 6
        (
            [0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [1.0, 0.0, 0.0],
            &[
                (0.5, 0.5),
                (1.0, 1.0),
                (1.5, 1.5),
                (0.5, 1.5),
                (1.5, 0.5),
                (0.5, 1.0),
            ][..],
        ),
        // +x: 2
        (
            [2.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
            &[(0.5, 0.5), (1.5, 1.5)][..],
        ),
        // -x: 5
        (
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            [0.0, 1.0, 0.0],
            &[(0.5, 0.5), (1.5, 1.5), (0.5, 1.5), (1.5, 0.5), (1.0, 1.0)][..],
        ),
        // +y: 3
        (
            [0.0, 2.0, 0.0],
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 0.0],
            &[(0.5, 0.5), (1.0, 1.0), (1.5, 1.5)][..],
        ),
        // -y: 4
        (
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            &[(0.5, 0.5), (0.5, 1.5), (1.5, 0.5), (1.5, 1.5)][..],
        ),
        // +z: 1 (LAST)
        (
            [0.0, 0.0, 2.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            &[(1.0, 1.0)][..],
        ),
    ]
}

/// Authors the die document (module docs).
pub fn die() -> Die {
    let mut r = Recorder::new();
    // pip_depth: the mid-DAG continuous parameter.
    r.push(DocEdit::SetDocParam {
        name: ParamName::new("pip_depth"),
        value: DocParam::continuous(Dimension::Length, DEPTH),
    });
    // The cube: profile on the xy plane, extruded +2.
    let cube_profile = r.profile(
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)]],
    );
    let cube = r.insert(Node::Extrude {
        profile: cube_profile,
        distance: len(2.0),
    });

    // Per-face masters: pip profile centered at the plane origin,
    // extruded INWARD by pip_depth (normal points out ⇒ negative
    // distance).
    let mut masters = Vec::new(); // (extrude id, u, v, pips)
    for (o, u, v, pips) in faces() {
        let prof = r.profile(o, u, v, vec![square(0.0, 0.0, 0.125)]);
        let ext = r.insert(Node::Extrude {
            profile: prof,
            distance: Expr::neg(Expr::param(ParamName::new("pip_depth"), Dimension::Length)),
        });
        masters.push((ext, u, v, pips));
    }

    // Interleaved Declare + Transform + Subtract triples (M4 PR 5,
    // F5): every pip's outer cap lies exactly ON its cube face — a
    // coincidence the recipe DECLARES per subtract (name pairs
    // resolved through the operands' tables at evaluation; the
    // retired bit rung no longer infers it from values). The A-side
    // face name wraps in `FromA` per boolean, tracked here.
    let face_name = |node: RecipeNodeId, seg: RoleSeg| StableName {
        kind: EntityKind::Face,
        node,
        path: vec![seg],
    };
    // faces() order: -z, +x, -x, +y, -y, +z against the cube extrude's
    // roles (profile (0,0)->(2,0)->(2,2)->(0,2): wall seg 0 = -y,
    // 1 = +x, 2 = +y, 3 = -x; caps: Bottom = -z, Top = +z).
    let wall = |seg: u32| wall(&r.doc, cube, seg);
    let mut cube_face_names: [StableName; 6] = [
        face_name(cube, RoleSeg::Cap(CapEnd::Start)),
        face_name(cube, wall(1)),
        face_name(cube, wall(3)),
        face_name(cube, wall(2)),
        face_name(cube, wall(0)),
        face_name(cube, RoleSeg::Cap(CapEnd::End)),
    ];
    let mut acc = cube;
    let mut pz_transform = acc; // overwritten below
    for (face_idx, &(ext, u, v, pips)) in masters.iter().enumerate() {
        for &(a, b) in pips {
            let t = [
                a * u[0] + b * v[0],
                a * u[1] + b * v[1],
                a * u[2] + b * v[2],
            ];
            let tr = r.insert(Node::Transform {
                input: ext,
                translation: [len(t[0]), len(t[1]), len(t[2])],
                rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
                rotation_angle: ang(0.0),
            });
            // The pip master extrudes INWARD (negative distance), so
            // its OUTER cap — the flush one — is Bottom (on the
            // sketch plane, which IS the cube face's plane).
            let pip_cap = face_name(ext, RoleSeg::Cap(CapEnd::Start));
            let decl = r.insert(Node::declare_rest(vec![(
                SitedRef::new(acc, cube_face_names[face_idx].clone()),
                SitedRef::new(tr, pip_cap),
            )]));
            let sub = r.insert(Node::Boolean {
                op: editor_core::BooleanOp::Subtract,
                a: acc,
                b: tr,
                declare: Some(decl),
            });
            acc = sub;
            pz_transform = tr;
            // Every A-side face name wraps once per boolean (N1
            // derivation paths through the new subtract node).
            for name in &mut cube_face_names {
                *name = face_name(sub, RoleSeg::FromA(name.clone().into()));
            }
        }
    }

    let n_nodes = r.doc.len();
    Die {
        doc: r.doc,
        edits: r.edits,
        final_node: acc,
        pz_extrude: masters[5].0,
        pz_transform,
        n_nodes,
    }
}

pub mod pr4;

/// **The witness tolerance a suite decides under** — `Tol::witness()`
/// under the name the suites reach for it by.
pub fn tol() -> Tol {
    Tol::witness()
}

/// **A one-segment name at a node** — the whole of what "the name
/// `node` mints for `seg`" is, for any entity kind. [`fname`],
/// [`ename`] and [`vname`] are this with the kind spelled in the
/// call.
pub fn minted(kind: EntityKind, node: RecipeNodeId, seg: RoleSeg) -> StableName {
    StableName {
        kind,
        node,
        path: vec![seg],
    }
}

/// One face name at a node (authoring shorthand).
pub fn fname(node: RecipeNodeId, seg: RoleSeg) -> StableName {
    minted(EntityKind::Face, node, seg)
}

/// One vertex name at a node (authoring shorthand).
pub fn vname(node: RecipeNodeId, seg: RoleSeg) -> StableName {
    minted(EntityKind::Vertex, node, seg)
}

/// **A cap RIM edge of an extrude**, by name — the arc cap `end`
/// shares with the wall over outer- or hole-loop segment `edge`.
pub fn rim_edge(node: RecipeNodeId, end: CapEnd, edge: ProfileEdgeRef) -> StableName {
    ename(node, RoleSeg::RimEdge(end, edge))
}

/// **A cap VERTEX of an extrude**, by name — the corner cap `end`
/// carries at profile vertex `vertex`.
pub fn cap_vertex(node: RecipeNodeId, end: CapEnd, vertex: ProfileVertexRef) -> StableName {
    vname(node, RoleSeg::CapVertex(end, vertex))
}

/// **A POLE vertex of a revolve**, by name — the vertex the axis pins,
/// minted for profile vertex `vertex`.
pub fn pole(node: RecipeNodeId, vertex: ProfileVertexRef) -> StableName {
    vname(node, RoleSeg::Pole(vertex))
}

/// **The symmetric U cutter, whose subtract table holds an N2 tie** —
/// a 4x4x4 block with a U-shaped prism cut through it, the U's two
/// arms congruent so nothing covariant discriminates the faces they
/// mint and the table records one `Entry::Tied` row instead of two
/// names. Returns the subtract node.
///
/// One home for a document a row needs when it needs a REAL tie
/// rather than a hand-planted one. The same four-node shape is
/// hand-copied across this tree; `work/wire` carries the row for
/// re-pointing those copies here.
pub fn u_cutter_tie(doc: ProfileDoc) -> (ProfileDoc, RecipeNodeId) {
    let (doc, block_profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (4.0, 0.0), (4.0, 4.0), (0.0, 4.0)]],
    );
    let (doc, target) = insert(
        doc,
        Node::Extrude {
            profile: block_profile,
            distance: len(4.0),
        },
    );
    let (doc, u_profile) = on_frame(
        doc,
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![
            (2.0, 1.0),
            (6.0, 1.0),
            (6.0, 3.0),
            (2.0, 3.0),
            (2.0, 2.5),
            (5.0, 2.5),
            (5.0, 1.5),
            (2.0, 1.5),
        ]],
    );
    let (doc, cutter) = insert(
        doc,
        Node::Extrude {
            profile: u_profile,
            distance: len(2.0),
        },
    );
    insert(
        doc,
        Node::Boolean {
            op: editor_core::BooleanOp::Subtract,
            a: target,
            b: cutter,
            declare: None,
        },
    )
}

/// One edge name at a node (authoring shorthand).
pub fn ename(node: RecipeNodeId, seg: RoleSeg) -> StableName {
    minted(EntityKind::Edge, node, seg)
}

// ---------------------------------------------------------------- //
// Reading an evaluation, its tables and its bodies
// ---------------------------------------------------------------- //
//
// ONE home for the readers every name-reading suite spells for itself.
// Each is the loud form: a missing node, a name that resolves to
// nothing or to the wrong KIND of thing, or a dead key, is a panic
// naming the subject rather than a `None` a row can drop on the floor.

/// The name table `id` published, or a panic naming what it did
/// instead.
pub fn table(ev: &Evaluation<f64>, id: RecipeNodeId) -> &NameTable {
    &ev.value(id)
        .unwrap_or_else(|| panic!("node {id:?} has no value: {:?}", ev.nodes.get(&id)))
        .name_table
}

/// The one entity a name answers to — the row's loud end when a mint
/// is missing, misspelled or aliased. `what` is the caller's word for
/// the subject, so a failure says which row's name did not resolve.
pub fn key_of(t: &NameTable, what: &str, n: &StableName) -> EntityKey {
    match t.lookup(n) {
        Some(Entry::Unique(r)) => r.key,
        other => panic!("{what}: {n:?} is not uniquely named: {other:?}"),
    }
}

/// [`key_of`], refusing anything that is not an edge.
pub fn edge_of(t: &NameTable, what: &str, n: &StableName) -> EdgeKey {
    match key_of(t, what, n) {
        EntityKey::Edge(k) => k,
        other => panic!("{what}: {n:?} names {other:?}, not an edge"),
    }
}

/// [`key_of`], refusing anything that is not a vertex.
pub fn vertex_of(t: &NameTable, what: &str, n: &StableName) -> VertexKey {
    match key_of(t, what, n) {
        EntityKey::Vertex(k) => k,
        other => panic!("{what}: {n:?} names {other:?}, not a vertex"),
    }
}

/// [`key_of`], refusing anything that is not a face.
pub fn face_of(t: &NameTable, what: &str, n: &StableName) -> FaceKey {
    match key_of(t, what, n) {
        EntityKey::Face(k) => k,
        other => panic!("{what}: {n:?} names {other:?}, not a face"),
    }
}

/// How many names in `t` take `seg`'s role.
pub fn count(t: &NameTable, seg: fn(&RoleSeg) -> bool) -> usize {
    t.iter().filter(|(n, _)| seg(&n.path[0])).count()
}

/// Where a vertex stands.
pub fn point(body: &Body<f64>, v: VertexKey) -> Point3<f64> {
    topo::readback::vertex_point(body, v).expect("a live vertex")
}

/// An edge's two end vertices.
pub fn ends(body: &Body<f64>, e: EdgeKey) -> [VertexKey; 2] {
    let edge = body.get_edge(e).expect("a live edge");
    let h = body.get_half_edge(edge.he_plus).expect("a live half-edge");
    let far = body.half_edge_end(edge.he_plus).expect("a forward half");
    [h.start, far]
}

/// Every vertex on `f`'s boundary — the face's own EXTENT, read out of
/// the body rather than inferred from the surface it is a region of.
/// An empty ring contributes its lone vertex: a pole is a vertex of
/// the extent that bounds none of the face's edges.
pub fn face_vertices(body: &Body<f64>, f: FaceKey) -> HashSet<VertexKey> {
    let face = body.get_face(f).expect("a live face");
    let mut out = HashSet::new();
    for lk in core::iter::once(face.outer).chain(face.rings.iter().copied()) {
        match body.get_loop(lk).expect("a live loop").boundary {
            LoopBoundary::Empty { vertex } => {
                out.insert(vertex);
            }
            LoopBoundary::Cycle { first } => {
                for he in body.loop_cycle(first).expect("a closed cycle") {
                    out.insert(body.get_half_edge(he).expect("a live half-edge").start);
                }
            }
        }
    }
    out
}

/// Every edge on `f`'s boundary — [`face_vertices`]'s twin over the
/// same walk. An empty ring bounds no edge, so it contributes nothing
/// here.
pub fn face_edges(body: &Body<f64>, f: FaceKey) -> HashSet<EdgeKey> {
    let face = body.get_face(f).expect("a live face");
    let mut out = HashSet::new();
    for lk in core::iter::once(face.outer).chain(face.rings.iter().copied()) {
        if let LoopBoundary::Cycle { first } = body.get_loop(lk).expect("a live loop").boundary {
            for he in body.loop_cycle(first).expect("a closed cycle") {
                out.insert(body.get_half_edge(he).expect("a live half-edge").edge);
            }
        }
    }
    out
}

/// **The twelve edges of an extruded `n`-gon prism, by name** — the
/// authoring form of "every edge" for a `Node::Fillet` selection
/// (M6-5). `n` is the outer loop's segment count; the names are the
/// extrude emitter's own: a cap–wall rim per (cap end, segment) and a
/// strut per profile vertex.
///
/// Authored, not queried: a selection FREEZES, so a corpus document
/// states the set it means rather than asking an evaluation.
pub fn prism_edges(doc: &editor_core::ProfileDoc, node: RecipeNodeId, n: u32) -> Vec<StableName> {
    let mut out = Vec::new();
    for seg in 0..n as usize {
        let e = piece(doc, node, 0, seg);
        out.push(rim_edge(node, CapEnd::Start, e));
        out.push(rim_edge(node, CapEnd::End, e));
        out.push(ename(node, RoleSeg::LateralEdge(vpiece(doc, node, 0, seg))));
    }
    out
}

/// **The piece every canonical position of the profile at `profile`
/// is**, under the document's current values — what a sweep over it
/// names each wall, rim and vertex by.
///
/// # Panics
///
/// If `profile` is not a profile node, or its program does not replay
/// and validate.
pub fn pieces(doc: &editor_core::ProfileDoc, profile: RecipeNodeId) -> ProfilePieces {
    match doc.node(profile) {
        Some(Node::Profile(p)) => p
            .pieces(&doc.param_env::<f64>(), Tol::witness())
            .expect("the profile's program replays and validates"),
        other => panic!("node {} is not a profile: {other:?}", profile.0),
    }
}

/// **The profile a sweep node sweeps** — an extrude's or a revolve's
/// operand, or the node itself where it IS a profile; any other node
/// is read through its first input, so a row spelling a wall-shaped
/// name in a downstream node's space spells it with the pieces of the
/// sweep underneath.
///
/// # Panics
///
/// If `node` is not live, or no sweep is upstream of it along first
/// inputs.
pub fn swept(doc: &ProfileDoc, node: RecipeNodeId) -> RecipeNodeId {
    match doc.node(node) {
        Some(Node::Extrude { profile, .. } | Node::Revolve { profile, .. }) => *profile,
        Some(Node::Profile(_)) => node,
        Some(other) => match other.inputs().first() {
            Some(&input) => swept(doc, input),
            None => panic!("node {} sweeps no profile: {other:?}", node.0),
        },
        None => panic!("node {} is not live", node.0),
    }
}

/// **The piece canonical segment `k` of canonical loop `l` is**, on the
/// profile `sweep` sweeps (or `sweep` itself, a profile).
///
/// # Panics
///
/// Where [`pieces`] does, or where the position is past the profile.
pub fn piece(
    doc: &editor_core::ProfileDoc,
    sweep: RecipeNodeId,
    l: usize,
    k: usize,
) -> ProfileEdgeRef {
    pieces(doc, swept(doc, sweep))
        .edge(l, k)
        .expect("the canonical position is the profile's")
}

/// **The piece starting at canonical vertex `v` of canonical loop
/// `l`**, on the profile `sweep` sweeps.
///
/// # Panics
///
/// Where [`piece`] does.
pub fn vpiece(
    doc: &editor_core::ProfileDoc,
    sweep: RecipeNodeId,
    l: usize,
    v: usize,
) -> ProfileVertexRef {
    pieces(doc, swept(doc, sweep))
        .vertex(l, v)
        .expect("the canonical position is the profile's")
}

/// **The leg step `step` draws**, spelled without a document — for a
/// row whose names are compared, sorted or carried and never resolved.
pub fn leg(step: u64) -> ProfileEdgeRef {
    ProfileEdgeRef::Piece {
        step: editor_core::StepId(step),
        role: editor_core::PieceRole::Leg,
    }
}

/// **A live node as an author inserts it**: a profile program enters
/// the document without step ids — the insert door mints them — so a
/// row that rebuilds a document by re-inserting its nodes clears them;
/// re-inserted in the same order, they are minted the same.
pub fn as_authored(node: &Node<ProfileProgram>) -> Node<ProfileProgram> {
    let mut node = node.clone();
    if let Node::Profile(program) = &mut node {
        program.ids = Vec::new();
    }
    node
}

/// **A piece no profile draws**: the first step any document mints,
/// in a role no verb gives it — a locator that is well formed and
/// within every document's step counter, and denotes nothing.
pub fn no_piece() -> ProfileEdgeRef {
    ProfileEdgeRef::Piece {
        step: editor_core::StepId(0),
        role: editor_core::PieceRole::Piece(7),
    }
}

/// A wall (lateral) role for outer-loop canonical segment `seg` of the
/// profile the extrude `ext` sweeps, spelled by the piece it is.
pub fn wall(doc: &editor_core::ProfileDoc, ext: RecipeNodeId, seg: u32) -> RoleSeg {
    RoleSeg::Lateral(piece(doc, ext, 0, seg as usize))
}

/// **The four flush families two x-offset blocks share** — the walls
/// y0/y1 (segments 0/2, the `square`/`desc` corner order) and both
/// caps — in ONE place, so a suite that names them and a suite that
/// declares them cannot disagree about which four they are.
pub fn flush_segs(doc: &editor_core::ProfileDoc, ext: RecipeNodeId) -> [RoleSeg; 4] {
    [
        wall(doc, ext, 0),
        wall(doc, ext, 2),
        RoleSeg::Cap(CapEnd::Start),
        RoleSeg::Cap(CapEnd::End),
    ]
}

/// **Those four families as a declared pair list**, each side SITED:
/// `at` is the operand (or member) the entity is read at, and the
/// name is the one the extrude `ext` minted, which a pass-through op
/// carries verbatim (N1).
pub fn flush_pairs(
    doc: &ProfileDoc,
    (a_at, a_ext): (RecipeNodeId, RecipeNodeId),
    (b_at, b_ext): (RecipeNodeId, RecipeNodeId),
) -> Vec<(SitedRef, SitedRef)> {
    flush_segs(doc, a_ext)
        .into_iter()
        .zip(flush_segs(doc, b_ext))
        .map(|(a, b)| {
            (
                SitedRef::new(a_at, fname(a_ext, a)),
                SitedRef::new(b_at, fname(b_ext, b)),
            )
        })
        .collect()
}

/// **One ENTITY of one member, in the UNION's own name space** — the
/// row `member_view` puts into that member's operand table, and the
/// shape a union's published table carries.
///
/// The test-side spelling of the crate's `names::member_name`, which
/// is crate-private. One home, so a suite that reads a union's table
/// and a suite that writes an expected row spell the rule once.
pub fn member_entity(
    union: RecipeNodeId,
    member: RecipeNodeId,
    of: StableName,
    kind: EntityKind,
) -> StableName {
    StableName {
        kind,
        node: union,
        path: vec![RoleSeg::FromMember {
            member,
            of: of.into(),
        }],
    }
}

/// The same, for the FACE case every row but a carried-contact one
/// wants.
pub fn member_face(union: RecipeNodeId, member: RecipeNodeId, of: StableName) -> StableName {
    member_entity(union, member, of, EntityKind::Face)
}

/// A `Declare` node pairing the flush planes of two axis-aligned
/// extruded blocks that share their y-range and z-range and differ
/// along x only (the corpus's standard sliding-overlap shape): walls
/// y0/y1 (segments 0/2, the `square`/`desc` corner order) plus both
/// caps (M4 PR 5 — the recipe states the coincidence intent the
/// retired bit rung used to infer from values).
pub fn declare_x_offset_flush(
    doc: ProfileDoc,
    a_ext: RecipeNodeId,
    b_ext: RecipeNodeId,
) -> (ProfileDoc, RecipeNodeId) {
    declare_x_offset_flush_at(doc, (a_ext, a_ext), (b_ext, b_ext))
}

/// The same, when the consuming boolean's OPERAND is not the extrude
/// that minted the names — a transform of it, which contributes no
/// role segment (N1) and so carries the extrude's names verbatim.
///
/// The site is the operand, always: it is what says which side of the
/// boolean the name is read on.
pub fn declare_x_offset_flush_at(
    doc: ProfileDoc,
    (a_at, a_ext): (RecipeNodeId, RecipeNodeId),
    (b_at, b_ext): (RecipeNodeId, RecipeNodeId),
) -> (ProfileDoc, RecipeNodeId) {
    // Each name is sited at the OPERAND whose table holds it, which
    // is what says which side of the boolean it is read on.
    let pairs = flush_pairs(&doc, (a_at, a_ext), (b_at, b_ext));
    insert(doc, Node::declare_rest(pairs))
}

/// **What every at-rest finding says about a declaration, in one
/// vocabulary** — the mate it names and the relation it bears, for a
/// row that wants to compare a whole finding list at once.
///
/// One definition for every suite that asks the question. A CARRIED
/// row's mate is a node of ANOTHER document, so it reports under its
/// own words rather than joining the own-minted ones and reading as
/// this document's.
pub fn relations(findings: &[editor_core::AtRestFinding]) -> Vec<(RecipeNodeId, &'static str)> {
    findings
        .iter()
        .map(|f| match &f.attribution {
            editor_core::Attribution::Refuted(m) => (m.mate, "refuted"),
            editor_core::Attribution::Declined(m) => (m.mate, "declined"),
            editor_core::Attribution::Carried {
                declaration,
                relation,
                ..
            } => (
                declaration.mate,
                match relation {
                    editor_core::Relation::Refuted => "carried_refuted",
                    editor_core::Relation::Declined => "carried_declined",
                },
            ),
            editor_core::Attribution::Unattributed => (RecipeNodeId(u64::MAX), "unattributed"),
        })
        .collect()
}

/// **No published merged face has a merged face among its
/// constituents** — the N3 flatness rule, asserted over every name of
/// every table an evaluation produced.
///
/// One walker for every suite that evaluates a document, so the rule
/// is checked wherever a `Merged` can be minted — the pair boolean's
/// own tables, the n-ary union's, and whatever wraps either — and not
/// only in the rows written to look for it. The walk is over every
/// name a segment embeds ([`embedded_names`]), so a merged face that
/// reaches a table inside a blend's or a pattern's name is held to
/// the same rule as one at a row's head; and a constituent is read
/// through its descent wrappers ([`is_merged_face`]), so a merged
/// face carried through untouched booleans before being merged again
/// is nesting exactly as a bare one is.
pub fn assert_no_nested_merged<T: geom_core::Decide>(ev: &editor_core::Evaluation<T>) {
    for (id, result) in &ev.nodes {
        let editor_core::NodeResult::Ok(value) = result else {
            continue;
        };
        for (name, _) in value.name_table.iter() {
            let nested = merged_sets(name)
                .into_iter()
                .flat_map(|set| set.iter())
                .find(|c| is_merged_face(c));
            assert!(
                nested.is_none(),
                "node {id:?} published a merged face with a merged constituent {nested:?}: {name:?}"
            );
        }
    }
}

/// True iff `name`, read through its `FromA`/`FromB` descent chain,
/// is a bare merged face — the shape a flat constituent set never
/// holds. A FRAGMENT of a merged face (`Merged` head with a
/// `Fragment` tail at the foot) is a fragment, not a merge, and is a
/// legitimate constituent.
fn is_merged_face(name: &StableName) -> bool {
    match name.path.as_slice() {
        [RoleSeg::Merged(_)] => true,
        [RoleSeg::FromA(inner) | RoleSeg::FromB(inner)] => is_merged_face(inner),
        _ => false,
    }
}

/// Every `Merged` constituent set reachable from `name`, its own
/// segments included.
fn merged_sets(name: &StableName) -> Vec<&[StableName]> {
    let mut out = Vec::new();
    for seg in &name.path {
        if let RoleSeg::Merged(set) = seg {
            out.push(set.as_slice());
        }
        for inner in embedded_names(seg) {
            out.extend(merged_sets(inner));
        }
    }
    out
}

/// The names one role segment embeds — a derivation argument or a
/// discrimination partner alike, since a merged face is held to the
/// flatness rule wherever it is written.
///
/// The match is EXHAUSTIVE on purpose: a segment added to the
/// vocabulary must be classified here before the suite compiles, so a
/// new name-carrying segment cannot hide a merged face from the walk.
fn embedded_names(seg: &RoleSeg) -> Vec<&StableName> {
    use editor_core::Qualifier;
    match seg {
        RoleSeg::FromA(x)
        | RoleSeg::FromB(x)
        | RoleSeg::FromMember { of: x, .. }
        | RoleSeg::SectionEdge { face: x, .. }
        | RoleSeg::SplitFragment { parent: x, .. }
        | RoleSeg::CrossingVertex { edge: x, .. }
        | RoleSeg::OnToolVertex { of: x, .. }
        | RoleSeg::Instance { of: x, .. }
        | RoleSeg::InPart { of: x }
        | RoleSeg::FromTarget(x)
        | RoleSeg::BlendFace(x)
        | RoleSeg::CornerFace(x)
        | RoleSeg::BandTrim { edge: x, .. }
        | RoleSeg::BandFoot(x)
        | RoleSeg::BandCross(x)
        | RoleSeg::BandCut(x)
        | RoleSeg::BandSlit(x)
        | RoleSeg::Inner(x)
        | RoleSeg::Rim(x)
        | RoleSeg::HoleRim { of: x, .. } => vec![x.as_ref()],
        RoleSeg::Seam { a: x, b: y }
        | RoleSeg::TrimEdge {
            edge: x,
            support: y,
        }
        | RoleSeg::FootVertex {
            vertex: x,
            support: y,
        }
        | RoleSeg::EndArc { vertex: x, edge: y } => vec![x.as_ref(), y.as_ref()],
        RoleSeg::Merged(v) | RoleSeg::BandFace(v) => v.iter().collect(),
        RoleSeg::Fragment(Qualifier::SideOf(v)) => v.iter().map(|(p, _)| p).collect(),
        RoleSeg::Fragment(Qualifier::OrderAlong { .. })
        | RoleSeg::OutputBody
        | RoleSeg::Cap(_)
        | RoleSeg::Lateral(_)
        | RoleSeg::RimEdge(..)
        | RoleSeg::LateralEdge(_)
        | RoleSeg::CapVertex(..)
        | RoleSeg::Band(_)
        | RoleSeg::BandRim(_)
        | RoleSeg::BandRimPi(_)
        | RoleSeg::BandPi(_)
        | RoleSeg::Meridian(..)
        | RoleSeg::MeridianVertex(..)
        | RoleSeg::RevolveCap(_)
        | RoleSeg::Pole(_)
        | RoleSeg::AxisEdge(_)
        | RoleSeg::SplitBody(_)
        | RoleSeg::SectionFace { .. }
        | RoleSeg::LoftWall(_)
        | RoleSeg::LoftSeam(_) => Vec::new(),
    }
}
