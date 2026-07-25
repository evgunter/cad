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
    Dimension, DocEdit, DocParam, Expr, Node, ParamName, ProfileDesc, ProfileDoc, RecipeNodeId,
};
use geom_core::{Point2, Point3, Vec3};
use profile::{Profile, ProfileLoop, SketchPlane};

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
pub fn step(doc: ProfileDoc, edit: DocEdit<ProfileDesc>) -> (ProfileDoc, Option<RecipeNodeId>) {
    let applied = doc.apply(&edit).unwrap();
    (applied.doc, applied.record.minted)
}

pub fn insert(doc: ProfileDoc, node: Node<ProfileDesc>) -> (ProfileDoc, RecipeNodeId) {
    let (doc, minted) = step(doc, DocEdit::InsertNode { node });
    (doc, minted.unwrap())
}

/// A profile description: `loops` on the plane with the given frame.
pub fn desc(
    origin: [f64; 3],
    u: [f64; 3],
    v: [f64; 3],
    loops: Vec<Vec<(f64, f64)>>,
) -> ProfileDesc {
    let plane = SketchPlane::from_frame(
        Point3::new(origin[0], origin[1], origin[2]),
        Vec3::new(u[0], u[1], u[2]),
        Vec3::new(v[0], v[1], v[2]),
    );
    let loops = loops
        .into_iter()
        .map(|pts| ProfileLoop::polygon(pts.into_iter().map(|(x, y)| Point2::new(x, y))))
        .collect();
    ProfileDesc(Profile::new(plane, loops))
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

/// The authored die and the ids the tests address.
pub struct Die {
    pub doc: ProfileDoc,
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
    let mut doc = ProfileDoc::empty();
    // pip_depth: the mid-DAG continuous parameter.
    (doc, _) = step(
        doc,
        DocEdit::SetDocParam {
            name: ParamName::new("pip_depth"),
            value: DocParam::Continuous {
                dim: Dimension::Length,
                value: DEPTH,
            },
        },
    );
    // The cube: profile on the xy plane, extruded +2.
    let (d, cube_profile) = insert(
        doc,
        Node::Profile(desc(
            [0.0; 3],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![vec![(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)]],
        )),
    );
    let (d, cube) = insert(
        d,
        Node::Extrude {
            profile: cube_profile,
            distance: len(2.0),
        },
    );
    doc = d;

    // Per-face masters: pip profile centered at the plane origin,
    // extruded INWARD by pip_depth (normal points out ⇒ negative
    // distance).
    let mut masters = Vec::new(); // (extrude id, u, v, pips)
    for (o, u, v, pips) in faces() {
        let (d, prof) = insert(
            doc,
            Node::Profile(desc(o, u, v, vec![square(0.0, 0.0, 0.125)])),
        );
        let (d, ext) = insert(
            d,
            Node::Extrude {
                profile: prof,
                distance: Expr::neg(Expr::param(ParamName::new("pip_depth"), Dimension::Length)),
            },
        );
        doc = d;
        masters.push((ext, u, v, pips));
    }

    // Interleaved Transform + Subtract pairs (the PR 1 die shape).
    let mut acc = cube;
    let mut pz_transform = acc; // overwritten below
    for &(ext, u, v, pips) in &masters {
        for &(a, b) in pips {
            let t = [
                a * u[0] + b * v[0],
                a * u[1] + b * v[1],
                a * u[2] + b * v[2],
            ];
            let (d, tr) = insert(
                doc,
                Node::Transform {
                    input: ext,
                    translation: [len(t[0]), len(t[1]), len(t[2])],
                    rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
                    rotation_angle: ang(0.0),
                },
            );
            let (d, sub) = insert(
                d,
                Node::Boolean {
                    op: editor_core::BooleanOp::Subtract,
                    a: acc,
                    b: tr,
                    declare: None,
                },
            );
            doc = d;
            acc = sub;
            pz_transform = tr;
        }
    }

    let n_nodes = doc.len();
    Die {
        doc,
        final_node: acc,
        pz_extrude: masters[5].0,
        pz_transform,
        n_nodes,
    }
}

pub mod pr4;
