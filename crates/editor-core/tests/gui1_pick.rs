//! GUI-1 Part B: the hit-test service (`resolve::pick`) end-to-end
//! through the public doors — every face of a real tessellated box
//! picks to a distinct resolvable `StableName`; a ray down a shared
//! edge resolves by the documented tie-break; a miss is the typed
//! miss; unusable nodes surface their typed `HitTestError`; two-body
//! occlusion orders by `t` regardless of target order; and the whole
//! thing is deterministic.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod fixture;

use editor_core::{
    CancelToken, EntityKey, EvalOptions, Evaluation, HitTestError, MeshPick, MeshPickError, Node,
    PickTarget, ProfileDoc, Ray, RecipeNodeId, Resolution, RunCtx, ValuePayload, pick_face,
    resolve,
};
use fixture::{desc, insert, len};
use geom_core::{Point3, Tol, Vec3};
use mesh::{FacePatch, Mesh};
use topo::Body;

const DELTA: f64 = 0.1;

fn run(doc: &ProfileDoc) -> Evaluation<f64> {
    editor_core::evaluate::<f64>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

fn ray(origin: [f64; 3], dir: [f64; 3]) -> Ray {
    Ray {
        origin: Point3::new(origin[0], origin[1], origin[2]),
        dir: Vec3::new(dir[0], dir[1], dir[2]),
    }
}

/// A unit cube `[0,1]³` shifted by `dx` along x, as one extrude node.
fn cube_doc_node(doc: ProfileDoc, dx: f64) -> (ProfileDoc, RecipeNodeId) {
    let (doc, profile) = insert(
        doc,
        Node::Profile(desc(
            [0.0; 3],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![vec![(dx, 0.0), (dx + 1.0, 0.0), (dx + 1.0, 1.0), (dx, 1.0)]],
        )),
    );
    insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(1.0),
        },
    )
}

fn body_of(ev: &Evaluation<f64>, node: RecipeNodeId) -> &Body<f64> {
    match &ev.value(node).expect("extrude evaluates").payload {
        ValuePayload::Body(b) => b,
        other => panic!("extrude payload is a body, got {}", other.kind_name()),
    }
}

fn mesh_of(ev: &Evaluation<f64>, node: RecipeNodeId) -> Mesh {
    mesh::tessellate(body_of(ev, node), DELTA, Tol::witness()).expect("box tessellates")
}

/// The patch a pick resolved to, recovered through PUBLIC resolution
/// (name → entity → back-reference match against the mesh) — the
/// round trip that proves the name denotes the face whose patch was
/// hit.
fn resolved_patch<'a>(
    doc: &ProfileDoc,
    ev: &Evaluation<f64>,
    mesh: &'a Mesh,
    name: &editor_core::StableName,
) -> &'a FacePatch {
    let Resolution::Resolved(r) = resolve(RunCtx { doc, eval: ev }, name) else {
        panic!("picked name resolves");
    };
    let EntityKey::Face(fk) = r.entity.key else {
        panic!("picked name denotes a face");
    };
    mesh.patches
        .iter()
        .find(|p| p.face == fk)
        .expect("resolved face has a patch in the mesh")
}

/// Every position index of a patch has the given coordinate equal to
/// `plane` (axis: 0 = x, 1 = y, 2 = z) — the geometric identification
/// of an axis-aligned box face.
fn patch_on_plane(mesh: &Mesh, patch: &FacePatch, axis: usize, plane: f64) -> bool {
    patch.triangles.iter().flatten().all(|&i| {
        let p = mesh.positions[i as usize];
        let c = [p.x, p.y, p.z][axis];
        c == plane
    })
}

/// Picks every face of a box body through the public service: six
/// rays at face centers, six distinct names, each resolving to the
/// patch on the expected plane, at the expected `t` and hit point.
#[test]
fn picks_every_face_of_a_box() {
    let doc = ProfileDoc::empty_derived("gui1_pick", Tol::witness());
    let (doc, ext) = cube_doc_node(doc, 0.0);
    let ev = run(&doc);
    let mesh = mesh_of(&ev, ext);
    let pick = MeshPick::build(&mesh).expect("well-formed mesh");
    let targets = [PickTarget {
        node: ext,
        body: 0,
        pick: &pick,
    }];

    // (ray, axis, plane): each ray shoots at a face center from
    // outside, 2 units out, so every expected t is exactly 2.
    let cases = [
        (ray([0.5, 0.5, -2.0], [0.0, 0.0, 1.0]), 2usize, 0.0),
        (ray([0.5, 0.5, 3.0], [0.0, 0.0, -1.0]), 2, 1.0),
        (ray([-2.0, 0.5, 0.5], [1.0, 0.0, 0.0]), 0, 0.0),
        (ray([3.0, 0.5, 0.5], [-1.0, 0.0, 0.0]), 0, 1.0),
        (ray([0.5, -2.0, 0.5], [0.0, 1.0, 0.0]), 1, 0.0),
        (ray([0.5, 3.0, 0.5], [0.0, -1.0, 0.0]), 1, 1.0),
    ];
    let mut names = Vec::new();
    for (r, axis, plane) in cases {
        let hit = pick_face(&ev, &targets, &r)
            .expect("no hit-test error")
            .expect("face center rays hit");
        assert_eq!(hit.node, ext);
        assert_eq!(hit.body, 0);
        assert_eq!(hit.t, 2.0, "dyadic face-center hit is exact");
        let c = [hit.point.x, hit.point.y, hit.point.z][axis];
        assert_eq!(c, plane, "hit point lies on the face plane");
        let patch = resolved_patch(&doc, &ev, &mesh, &hit.name);
        assert!(
            patch_on_plane(&mesh, patch, axis, plane),
            "resolved patch lies on axis {axis} plane {plane}"
        );
        names.push(hit.name);
    }
    for (i, a) in names.iter().enumerate() {
        for b in names.iter().skip(i + 1) {
            assert_ne!(a, b, "six distinct face names");
        }
    }
}

/// A ray down the shared edge `x = 1 ∧ z = 1` hits the top and +x
/// faces at the same exact `t`; the documented tie-break (earlier
/// target, then earlier flat triangle = earlier patch in face-arena
/// order) picks the LOWER-indexed of the two patches — asserted
/// against the mesh, and repeatable.
#[test]
fn edge_ray_between_two_faces_resolves_deterministically() {
    let doc = ProfileDoc::empty_derived("gui1_pick_edge", Tol::witness());
    let (doc, ext) = cube_doc_node(doc, 0.0);
    let ev = run(&doc);
    let mesh = mesh_of(&ev, ext);
    let pick = MeshPick::build(&mesh).expect("well-formed mesh");
    let targets = [PickTarget {
        node: ext,
        body: 0,
        pick: &pick,
    }];

    // Through (1, 0.5, 1) at t = 1: on the boundary of BOTH the top
    // (z = 1) and the +x (x = 1) faces. All coordinates dyadic, so
    // both faces' exact tests answer t = 1.0 bit-exactly.
    let r = ray([2.0, 0.5, 2.0], [-1.0, 0.0, -1.0]);
    let hit = pick_face(&ev, &targets, &r)
        .expect("no hit-test error")
        .expect("edge ray hits");
    assert_eq!(hit.t, 1.0);

    // Determinism: the same pick answers identically.
    let again = pick_face(&ev, &targets, &r)
        .expect("no hit-test error")
        .expect("edge ray hits");
    assert_eq!(hit.name, again.name);
    assert_eq!(hit.t.to_bits(), again.t.to_bits());

    // The documented tie-break: of the two patches containing the
    // edge, the one earlier in `Mesh::patches` (face-arena) order
    // wins, because flat triangle order is patch-major.
    let top = mesh
        .patches
        .iter()
        .position(|p| patch_on_plane(&mesh, p, 2, 1.0))
        .expect("top patch");
    let px = mesh
        .patches
        .iter()
        .position(|p| patch_on_plane(&mesh, p, 0, 1.0))
        .expect("+x patch");
    let expected = &mesh.patches[top.min(px)];
    let got = resolved_patch(&doc, &ev, &mesh, &hit.name);
    assert_eq!(
        got.face, expected.face,
        "tie resolves to the earlier patch (top {top}, +x {px})"
    );
}

/// A ray that hits nothing is the TYPED miss (`Ok(None)`), not an
/// error and not a panic.
#[test]
fn miss_is_a_typed_miss() {
    let doc = ProfileDoc::empty_derived("gui1_pick_miss", Tol::witness());
    let (doc, ext) = cube_doc_node(doc, 0.0);
    let ev = run(&doc);
    let mesh = mesh_of(&ev, ext);
    let pick = MeshPick::build(&mesh).expect("well-formed mesh");
    let targets = [PickTarget {
        node: ext,
        body: 0,
        pick: &pick,
    }];
    // Points away from the box entirely.
    let r = ray([0.5, 0.5, -2.0], [0.0, 0.0, -1.0]);
    assert!(pick_face(&ev, &targets, &r).expect("no error").is_none());
    // A poison ray is a legal query and a typed miss, never an error.
    let r = ray([f64::NAN, 0.5, -2.0], [0.0, 0.0, 1.0]);
    assert!(pick_face(&ev, &targets, &r).expect("no error").is_none());
}

/// Failed / poisoned / unevaluated target nodes surface their typed
/// `HitTestError` — never flattened into a miss, and checked up front
/// (the ray's own outcome is irrelevant).
#[test]
fn unusable_nodes_surface_typed_errors() {
    let doc = ProfileDoc::empty_derived("gui1_pick_err", Tol::witness());
    let (doc, good) = cube_doc_node(doc, 0.0);
    let (doc, profile) = insert(
        doc,
        Node::Profile(desc(
            [0.0; 3],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
        )),
    );
    let (doc, bad) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(0.0), // degenerate: the extrude fails
        },
    );
    let (doc, poisoned) = insert(
        doc,
        Node::Boolean {
            op: editor_core::BooleanOp::Union,
            a: bad,
            b: good,
            declare: None,
        },
    );
    let ev = run(&doc);
    let mesh = mesh_of(&ev, good);
    let pick = MeshPick::build(&mesh).expect("well-formed mesh");
    // The mesh content is irrelevant: target standing is checked
    // before any geometry.
    let r = ray([0.5, 0.5, -2.0], [0.0, 0.0, 1.0]);

    let t = |node| {
        [PickTarget {
            node,
            body: 0,
            pick: &pick,
        }]
    };
    assert_eq!(
        pick_face(&ev, &t(bad), &r).expect_err("failed node is an error"),
        HitTestError::NodeFailed { node: bad }
    );
    assert_eq!(
        pick_face(&ev, &t(poisoned), &r).expect_err("poisoned node is an error"),
        HitTestError::NodePoisoned {
            node: poisoned,
            through: bad
        }
    );
    let foreign = RecipeNodeId(9999);
    assert_eq!(
        pick_face(&ev, &t(foreign), &r).expect_err("foreign node is an error"),
        HitTestError::NodeNotEvaluated { node: foreign }
    );
    // A good target FIRST does not mask a bad one later in the slice.
    let both = [
        PickTarget {
            node: good,
            body: 0,
            pick: &pick,
        },
        PickTarget {
            node: bad,
            body: 0,
            pick: &pick,
        },
    ];
    assert_eq!(
        pick_face(&ev, &both, &r).expect_err("bad target still surfaces"),
        HitTestError::NodeFailed { node: bad }
    );
}

/// Two bodies along one ray: the nearer wins by `t`, independent of
/// target order; reversing the ray picks the other body.
#[test]
fn occlusion_orders_by_t_across_bodies() {
    let doc = ProfileDoc::empty_derived("gui1_pick_occl", Tol::witness());
    let (doc, near) = cube_doc_node(doc, 0.0); // x ∈ [0, 1]
    let (doc, far) = cube_doc_node(doc, 3.0); // x ∈ [3, 4]
    let ev = run(&doc);
    let mesh_near = mesh_of(&ev, near);
    let mesh_far = mesh_of(&ev, far);
    let pick_near = MeshPick::build(&mesh_near).expect("well-formed mesh");
    let pick_far = MeshPick::build(&mesh_far).expect("well-formed mesh");
    let tn = PickTarget {
        node: near,
        body: 0,
        pick: &pick_near,
    };
    let tf = PickTarget {
        node: far,
        body: 0,
        pick: &pick_far,
    };

    let forward = ray([-1.0, 0.5, 0.5], [1.0, 0.0, 0.0]);
    for targets in [[tn, tf], [tf, tn]] {
        let hit = pick_face(&ev, &targets, &forward)
            .expect("no error")
            .expect("hits the near cube");
        assert_eq!(
            hit.node, near,
            "nearest body wins regardless of target order"
        );
        assert_eq!(hit.t, 1.0);
        assert_eq!((hit.point.x, hit.point.y, hit.point.z), (0.0, 0.5, 0.5));
    }

    let backward = ray([6.0, 0.5, 0.5], [-1.0, 0.0, 0.0]);
    let hit = pick_face(&ev, &[tn, tf], &backward)
        .expect("no error")
        .expect("hits the far cube");
    assert_eq!(hit.node, far);
    assert_eq!(hit.t, 2.0);
    assert_eq!((hit.point.x, hit.point.y, hit.point.z), (4.0, 0.5, 0.5));
}

/// `MeshPick::build` refuses a mesh whose triangle indexes outside
/// its position buffer — typed, with the offending site named in
/// arena-key-free vocabulary.
#[test]
fn corrupt_mesh_is_a_typed_build_error() {
    let doc = ProfileDoc::empty_derived("gui1_pick_corrupt", Tol::witness());
    let (doc, ext) = cube_doc_node(doc, 0.0);
    let ev = run(&doc);
    let mut mesh = mesh_of(&ev, ext);
    // Corrupt one triangle's index past the buffer.
    let bad = mesh.positions.len() as u32;
    mesh.patches[0].triangles[0][1] = bad;
    assert_eq!(
        MeshPick::build(&mesh).expect_err("corrupt index refuses"),
        MeshPickError::PositionOutOfRange {
            patch: 0,
            triangle: 0,
            index: bad
        }
    );
}
