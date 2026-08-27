//! Fixtures shared by this crate's suites, **derived from the scene
//! they test** rather than restated beside it.
//!
//! Why this file exists: the plate's dimensions were hand-copied into
//! three suites, so changing `scene::plate_with_hole` would have left
//! two of them testing a box the scene no longer has — green, and
//! measuring nothing. `viewer::scene` now exports the plate's identity
//! (`PLATE_EXTENT`, `PLATE_HOLE_RADIUS`) and everything here is a
//! function of those constants, so the fixtures cannot drift from the
//! subject.
//!
//! The two review suites (`review_gui0_r1`, `review_gui0_r2`) keep
//! their own fixtures on purpose: a promoted review suite's value is
//! that it is an INDEPENDENT derivation of what the unit claims
//! (`memories/review-and-dependency-policy.md`), and pointing it at
//! the implementation's own constants would spend exactly that.

#![allow(dead_code)] // loaded once per consumer; each uses a subset
#![allow(unreachable_pub)]
// why: root Cargo.toml, the `unreachable_pub` stanza
// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]

use bvh::Aabb;
use pncad::geom_core::Point3;
use viewer::camera::Camera;
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
use pncad::profile::SketchPlane;

/// Apply one edit, answering the new document and any minted id.
pub fn edited(
    doc: &Doc<ProfileProgram>,
    edit: DocEdit<ProfileProgram>,
    tol: Tol,
) -> (Doc<ProfileProgram>, Option<RecipeNodeId>) {
    let applied = apply(doc, &edit, tol).expect("the fixture's edit applies");
    (applied.doc, applied.record.minted)
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

/// A square profile node's payload, `side` metres on a side.
pub fn square(side: f64) -> Node<ProfileProgram> {
    Node::Profile(ProfileProgram {
        plane: SketchPlane::xy(),
        loops: vec![
            LoopProgram::polygon([(0.0, 0.0), (side, 0.0), (side, side), (0.0, side)])
                .expect("finite corners"),
        ],
    })
}

/// A length literal.
pub fn len(metres: f64) -> Expr {
    Expr::literal(metres, Dimension::Length).expect("a finite length")
}

/// A dimensionless literal.
pub fn scl(value: f64) -> Expr {
    Expr::literal(value, Dimension::Scalar).expect("a finite scalar")
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
            value: DocParam::Continuous {
                dim: Dimension::Length,
                value: 0.008,
            },
        },
        tol,
    );
    let (doc, profile) = inserted(&doc, square(0.04), tol);
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
    let (doc, profile) = inserted(&doc, square(0.04), tol);
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
            rotation_angle: Expr::literal(0.0, Dimension::Angle).expect("finite"),
        },
        tol,
    );
    (doc, extrude, moved)
}

/// A fresh directory under the OS temp root, named for the caller.
///
/// One home: two suites wanted the same six lines and had copied them
/// verbatim, which is exactly the drift this module's header exists to
/// prevent. (A review suite keeping its own copy is the one case that
/// argument does not cover — independence is the point there.)
pub fn tempdir(label: &str) -> std::path::PathBuf {
    let unique = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_nanos());
    let dir = std::env::temp_dir().join(format!("{label}-{unique}"));
    std::fs::create_dir_all(&dir).expect("the fixture directory is creatable");
    dir
}
