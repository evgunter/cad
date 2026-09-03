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
#![allow(dead_code)] // loaded once per consumer; each uses a subset
#![allow(unreachable_pub)] // why: root Cargo.toml, the `unreachable_pub` stanza

use editor_core::{
    CapEnd, Datum, Dimension, DocEdit, DocParam, EntityKind, Expr, LoopProgram, Node, ParamName,
    ProfileDoc, ProfileEdgeRef, ProfileProgram, ProfileVertexRef, RecipeNodeId, RoleSeg,
    StableName,
};
use geom_core::Tol;

/// The pip depth the document's `pip_depth` parameter starts at.
pub const DEPTH: f64 = 0.125;
/// The exact die volume oracle at `DEPTH` (M3).
pub const DIE_VOLUME: f64 = 7.8359375;

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
pub fn step(doc: ProfileDoc, edit: DocEdit<ProfileProgram>) -> (ProfileDoc, Option<RecipeNodeId>) {
    let applied = doc.apply(&edit, Tol::witness()).unwrap();
    (applied.doc, applied.record.minted)
}

pub fn insert(doc: ProfileDoc, node: Node<ProfileProgram>) -> (ProfileDoc, RecipeNodeId) {
    let (doc, minted) = step(doc, DocEdit::InsertNode { node });
    (doc, minted.unwrap())
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
/// frame's authored literals and hands them to the same
/// `SketchPlane::from_frame` the evaluator's own read uses.
///
/// **Orthonormality is the caller's, as it is at every other
/// `from_frame`** — this asserts rather than orthogonalizes, so a
/// fixture whose frame is not already orthonormal fails here instead of
/// silently getting a different plane from the one the evaluator would
/// build. Every fixture frame in this tree is authored orthonormal.
///
/// # Panics
///
/// If `plane` is not a `Datum::Frame`, if its components are not
/// literals, or if `u` and `v` are not an orthonormal pair.
pub fn plane_of(doc: &ProfileDoc, plane: RecipeNodeId) -> profile::SketchPlane<f64> {
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
    for (name, w) in [("u", u), ("v", v)] {
        assert!(
            (w.norm() - 1.0).abs() < 1e-12,
            "fixture frame {}'s {name} is not unit",
            plane.0
        );
    }
    assert!(
        u.dot(v).abs() < 1e-12,
        "fixture frame {}'s u and v are not perpendicular",
        plane.0
    );
    profile::SketchPlane::from_frame(geom_core::Point3::new(o.x, o.y, o.z), u, v)
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
    ProfileProgram { plane, loops }
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
    pub edits: Vec<DocEdit<ProfileProgram>>,
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
        let applied =
            editor_core::apply(&self.doc, &edit, Tol::witness()).expect("recorded edit must apply");
        self.doc = applied.doc;
        self.edits.push(edit);
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
    pub edits: Vec<DocEdit<ProfileProgram>>,
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
    let wall = |seg: u32| {
        RoleSeg::Lateral(ProfileEdgeRef {
            loop_index: 0,
            segment: seg,
        })
    };
    let mut cube_face_names: [StableName; 6] = [
        face_name(cube, RoleSeg::Cap(CapEnd::Bottom)),
        face_name(cube, wall(1)),
        face_name(cube, wall(3)),
        face_name(cube, wall(2)),
        face_name(cube, wall(0)),
        face_name(cube, RoleSeg::Cap(CapEnd::Top)),
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
            let pip_cap = face_name(ext, RoleSeg::Cap(CapEnd::Bottom));
            let decl = r.insert(Node::declare_rest(vec![(
                cube_face_names[face_idx].clone(),
                pip_cap,
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
                *name = face_name(sub, RoleSeg::FromA(Box::new(name.clone())));
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

/// One face name at a node (authoring shorthand).
pub fn fname(node: RecipeNodeId, seg: RoleSeg) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node,
        path: vec![seg],
    }
}

/// One edge name at a node (authoring shorthand).
pub fn ename(node: RecipeNodeId, seg: RoleSeg) -> StableName {
    StableName {
        kind: EntityKind::Edge,
        node,
        path: vec![seg],
    }
}

/// **The twelve edges of an extruded `n`-gon prism, by name** — the
/// authoring form of "every edge" for a `Node::Fillet` selection
/// (M6-5). `n` is the outer loop's segment count; the names are the
/// extrude emitter's own: a cap–wall rim per (cap end, segment) and a
/// strut per profile vertex.
///
/// Authored, not queried: a selection FREEZES, so a corpus document
/// states the set it means rather than asking an evaluation.
pub fn prism_edges(node: RecipeNodeId, n: u32) -> Vec<StableName> {
    let mut out = Vec::new();
    for seg in 0..n {
        let e = ProfileEdgeRef {
            loop_index: 0,
            segment: seg,
        };
        out.push(ename(node, RoleSeg::RimEdge(CapEnd::Bottom, e)));
        out.push(ename(node, RoleSeg::RimEdge(CapEnd::Top, e)));
        out.push(ename(
            node,
            RoleSeg::LateralEdge(ProfileVertexRef {
                loop_index: 0,
                vertex: seg,
            }),
        ));
    }
    out
}

/// A wall (lateral) role for outer-loop segment `seg`.
pub fn wall(seg: u32) -> RoleSeg {
    RoleSeg::Lateral(ProfileEdgeRef {
        loop_index: 0,
        segment: seg,
    })
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
    let pairs = vec![
        (fname(a_ext, wall(0)), fname(b_ext, wall(0))),
        (fname(a_ext, wall(2)), fname(b_ext, wall(2))),
        (
            fname(a_ext, RoleSeg::Cap(CapEnd::Bottom)),
            fname(b_ext, RoleSeg::Cap(CapEnd::Bottom)),
        ),
        (
            fname(a_ext, RoleSeg::Cap(CapEnd::Top)),
            fname(b_ext, RoleSeg::Cap(CapEnd::Top)),
        ),
    ];
    insert(doc, Node::declare_rest(pairs))
}
