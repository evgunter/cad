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
#![allow(dead_code)] // shared across test binaries; not all use all of it

use editor_core::{
    CapEnd, Dimension, DocEdit, DocParam, EntityKind, Expr, LoopProgram, Node, ParamName,
    ProfileDoc, ProfileEdgeRef, ProfileProgram, ProfileVertexRef, RecipeNodeId, RoleSeg,
    StableName,
};
use geom_core::{Point3, Vec3};
use profile::SketchPlane;
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

/// A profile PROGRAM: polygon `loops` on the plane with the given
/// frame (LIB-SWITCH §4i: the corpus's polygon choke point — under v4
/// each loop is a chain program, `At(p0), LineTo(p1), …,
/// LineTo(Start)`, the VQ5 expansion at literal points).
pub fn desc(
    origin: [f64; 3],
    u: [f64; 3],
    v: [f64; 3],
    loops: Vec<Vec<(f64, f64)>>,
) -> ProfileProgram {
    let plane = SketchPlane::from_frame(
        Point3::new(origin[0], origin[1], origin[2]),
        Vec3::new(u[0], u[1], u[2]),
        Vec3::new(v[0], v[1], v[2]),
    );
    let loops = loops
        .into_iter()
        .map(|pts| LoopProgram::polygon(pts).expect("finite corners"))
        .collect();
    ProfileProgram { plane, loops }
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
        let applied = editor_core::apply(&self.doc, &edit, Tol::witness()).expect("recorded edit must apply");
        self.doc = applied.doc;
        self.edits.push(edit);
        applied.record.minted
    }

    /// Inserts a node, returning its minted id.
    pub fn insert(&mut self, node: Node<ProfileProgram>) -> RecipeNodeId {
        self.push(DocEdit::InsertNode { node }).expect("minted id")
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
        value: DocParam::Continuous {
            dim: Dimension::Length,
            value: DEPTH,
        },
    });
    // The cube: profile on the xy plane, extruded +2.
    let cube_profile = r.insert(Node::Profile(desc(
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)]],
    )));
    let cube = r.insert(Node::Extrude {
        profile: cube_profile,
        distance: len(2.0),
    });

    // Per-face masters: pip profile centered at the plane origin,
    // extruded INWARD by pip_depth (normal points out ⇒ negative
    // distance).
    let mut masters = Vec::new(); // (extrude id, u, v, pips)
    for (o, u, v, pips) in faces() {
        let prof = r.insert(Node::Profile(desc(o, u, v, vec![square(0.0, 0.0, 0.125)])));
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
