//! The GUI-1 hit-test service driven as a CONSUMER would drive it,
//! and held against an oracle written independently of it (review
//! lane R2).
//!
//! `gui1_pick.rs` checks the service against hand-computed answers on
//! one axis-aligned unit cube. This file instead:
//!
//! 1. authors, evaluates and tessellates real documents through the
//!    public doors, then picks through `pick_face` — no private item
//!    is touched anywhere below;
//! 2. answers every pick a SECOND time with [`brute_nearest`], a
//!    ray/triangle test written to a different formula (plane
//!    intersection, then barycentric coordinates from edge cross
//!    products) over every triangle of every target, with no BVH and
//!    no early-out — so a wrong traversal, a wrong prune or a wrong
//!    early-out shows up as a disagreement rather than as two copies
//!    of the same mistake;
//! 3. drives the adversarial shapes a viewport actually produces:
//!    rays that graze an edge, rays that lie in a face plane, rays
//!    aimed at a shared vertex, three-body occlusion, and a mesh
//!    carrying a degenerate (zero-area) triangle.
//!
//! Sweep shape (per `memories/test-suite-cost.md`): counterexample
//! search, varying seed, counts on the effort dial, with an
//! anti-vacuity floor under the number of pick answers actually
//! compared.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

test_utils::gated_to![
    "crates/editor-core/src/resolve/",
    "crates/bvh/src/",
    "crates/mesh/src/",
    "crates/geom-core/src/linalg/",
];

use crate::fixture;

use editor_core::{
    CancelToken, EvalOptions, Evaluation, MeshPick, Node, PickTarget, ProfileDoc, Ray,
    RecipeNodeId, ValuePayload, pick_face,
};
use fixture::{insert, len, on_frame};
use geom_core::{Point3, Tol, Vec3};
use mesh::Mesh;
use test_utils::{fuzz, vacuity::Exposure};
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

/// A `w × w × h` box with its lower corner at `(ox, oy, 0)`, as one
/// extrude node appended to `doc`.
fn box_node(doc: ProfileDoc, ox: f64, oy: f64, w: f64, h: f64) -> (ProfileDoc, RecipeNodeId) {
    let (doc, profile) = on_frame(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(ox, oy), (ox + w, oy), (ox + w, oy + w), (ox, oy + w)]],
    );
    insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(h),
        },
    )
}

fn mesh_of(ev: &Evaluation<f64>, node: RecipeNodeId) -> Mesh {
    let body: &Body<f64> = match &ev.value(node).expect("node evaluates").payload {
        ValuePayload::Body(b) => b,
        other => panic!("expected a body, got {}", other.kind_name()),
    };
    mesh::tessellate(body, DELTA, Tol::witness()).expect("body tessellates")
}

/// The independent ray/triangle test: intersect the ray with the
/// triangle's PLANE, then decide containment from the three edge
/// cross products against the plane normal. Different algebra from
/// Möller–Trumbore (no shared subexpression, no `1/det` scaling of
/// the barycentrics), same closed-boundary semantics.
///
/// `Some(t)` for a hit at `t ≥ 0`; `None` for a miss, a parallel ray,
/// a degenerate triangle, or any NaN.
fn brute_ray_triangle(r: &Ray, a: Point3<f64>, b: Point3<f64>, c: Point3<f64>) -> Option<f64> {
    let n = (b - a).cross(c - a);
    let denom = n.dot(r.dir);
    if denom == 0.0 {
        return None;
    }
    let t = n.dot(a - r.origin) / denom;
    if t.is_nan() || t < 0.0 {
        return None;
    }
    let p = r.origin + r.dir * t;
    // Closed containment: p is inside iff every edge cross product
    // agrees in sign with the face normal (zero counts as inside).
    let nn = n.dot(n);
    if nn.is_nan() || nn <= 0.0 {
        return None; // degenerate triangle
    }
    let w0 = (b - a).cross(p - a).dot(n);
    let w1 = (c - b).cross(p - b).dot(n);
    let w2 = (a - c).cross(p - c).dot(n);
    // Scale-free tolerance: the cross products carry |n| · |edge| ·
    // |offset|, so compare against a relative floor rather than 0.
    let eps = nn.sqrt() * 1e-9;
    if w0 >= -eps && w1 >= -eps && w2 >= -eps {
        Some(t)
    } else {
        None
    }
}

/// The nearest hit over EVERY triangle of EVERY target, no BVH and no
/// early-out: `(t, target position, flat triangle position)`.
fn brute_nearest(meshes: &[&Mesh], r: &Ray) -> Option<(f64, usize, usize, topo::FaceKey)> {
    let mut best: Option<(f64, usize, usize, topo::FaceKey)> = None;
    for (tp, m) in meshes.iter().enumerate() {
        let mut flat = 0usize;
        for patch in &m.patches {
            for tri in &patch.triangles {
                let [a, b, c] = [
                    m.positions[tri[0] as usize],
                    m.positions[tri[1] as usize],
                    m.positions[tri[2] as usize],
                ];
                if let Some(t) = brute_ray_triangle(r, a, b, c) {
                    let cand = (t, tp, flat, patch.face);
                    best = Some(match best {
                        None => cand,
                        Some(b0) => {
                            if (cand.0, cand.1, cand.2) < (b0.0, b0.1, b0.2) {
                                cand
                            } else {
                                b0
                            }
                        }
                    });
                }
                flat += 1;
            }
        }
    }
    best
}

/// **The differential: `pick_face` against an independent brute-force
/// nearest hit, over randomized viewport-shaped rays.**
///
/// Two bodies side by side; rays aimed from random points on a
/// surrounding sphere at random points in the scene box, so a healthy
/// fraction hit. Per draw: the service and the oracle must agree on
/// whether there is a hit at all, on `t` to a tight relative
/// tolerance, and — when the oracle's winner is unambiguous (no other
/// face within a relative margin of the winning `t`) — on WHICH face
/// was hit.
#[test]
fn pick_face_agrees_with_an_independent_brute_force_nearest_hit() {
    let doc = ProfileDoc::empty_derived("gui1_r2_diff", Tol::witness());
    let (doc, a) = box_node(doc, 0.0, 0.0, 1.0, 1.0);
    let (doc, b) = box_node(doc, 2.0, 0.5, 1.5, 2.0);
    let ev = run(&doc);
    let (ma, mb) = (mesh_of(&ev, a), mesh_of(&ev, b));
    let (pa, pb) = (
        MeshPick::build(&ma).expect("mesh a indexes"),
        MeshPick::build(&mb).expect("mesh b indexes"),
    );
    let targets = [
        PickTarget {
            node: a,
            body: 0,
            pick: &pa,
        },
        PickTarget {
            node: b,
            body: 0,
            pick: &pb,
        },
    ];
    let meshes = [&ma, &mb];

    let mut rng = fuzz::start("pick_face vs independent brute force (R2)");
    let mut seen = Exposure::new("pick_face vs independent brute force (R2)");
    for case in 0..fuzz::scaled(150) {
        // Origin on a sphere of radius 8 around the scene, aimed at a
        // uniform point in the scene's bounding box — the shape of a
        // click ray, and one that hits often enough to matter.
        let (u, v) = (rng.range(0.0, 1.0), rng.range(0.0, 1.0));
        let phi = 2.0 * std::f64::consts::PI * u;
        let cz = 2.0f64.mul_add(v, -1.0);
        let s = (1.0 - cz * cz).max(0.0).sqrt();
        let o = [
            8.0 * s * phi.cos() + 1.5,
            8.0 * s * phi.sin() + 1.0,
            8.0 * cz + 1.0,
        ];
        let aim = [
            rng.range(-0.5, 4.0),
            rng.range(-0.5, 2.5),
            rng.range(-0.5, 2.5),
        ];
        let r = ray(o, [aim[0] - o[0], aim[1] - o[1], aim[2] - o[2]]);

        let got = pick_face(&ev, &targets, &r).expect("evaluated targets never error");
        let want = brute_nearest(&meshes, &r);

        match (&got, &want) {
            (None, None) => seen.note("agreed misses"),
            (Some(h), Some((t, _, _, face))) => {
                seen.note("agreed hits");
                let rel = (h.t - t).abs() / t.abs().max(1.0);
                assert!(
                    rel < 1e-9,
                    "case {case}: t disagrees — service {} vs oracle {t}; {}",
                    h.t,
                    fuzz::replay()
                );
                // Which face: only asserted where the oracle's winner
                // is not in a near-tie with another face, since the
                // documented tie-break is lexicographic on positions
                // the two implementations enumerate identically but
                // whose `t`s are separately rounded.
                let mut rivals = 0;
                for m in meshes {
                    let mut flat = 0usize;
                    for patch in &m.patches {
                        for tri in &patch.triangles {
                            let [x, y, z] = [
                                m.positions[tri[0] as usize],
                                m.positions[tri[1] as usize],
                                m.positions[tri[2] as usize],
                            ];
                            if let Some(tt) = brute_ray_triangle(&r, x, y, z)
                                && patch.face != *face
                                && (tt - t).abs() <= t.abs().mul_add(1e-6, 1e-9)
                            {
                                rivals += 1;
                            }
                            flat += 1;
                        }
                    }
                    let _ = flat;
                }
                if rivals == 0 {
                    seen.note("unambiguous winners, face identity compared");
                    let resolved = editor_core::resolve(
                        editor_core::RunCtx {
                            doc: &doc,
                            eval: &ev,
                        },
                        &h.name,
                    );
                    let editor_core::Resolution::Resolved(res) = resolved else {
                        panic!(
                            "case {case}: picked name does not resolve; {}",
                            fuzz::replay()
                        );
                    };
                    let editor_core::EntityKey::Face(fk) = res.entity.key else {
                        panic!("case {case}: picked name is not a face; {}", fuzz::replay());
                    };
                    assert_eq!(
                        fk,
                        *face,
                        "case {case}: service and oracle picked different faces; {}",
                        fuzz::replay()
                    );
                } else {
                    seen.note("near-tie draws (face identity not compared)");
                }
            }
            (Some(h), None) => panic!(
                "case {case}: service reports a hit at t = {} the oracle does not see; {}",
                h.t,
                fuzz::replay()
            ),
            (None, Some((t, _, _, _))) => panic!(
                "case {case}: service MISSED a hit the oracle finds at t = {t}; {}",
                fuzz::replay()
            ),
        }
    }
    seen.report();
    // Floors stated at CAD_FUZZ_EFFORT=1 (measured: ~55 agreed hits,
    // ~90 agreed misses of 150 draws).
    seen.require(
        "agreed hits",
        20,
        &format!(
            "the sweep never actually hit the geometry, so nothing about \
             the nearest-hit chain was exercised; {}",
            fuzz::replay()
        ),
    );
    seen.require(
        "unambiguous winners, face identity compared",
        10,
        &format!(
            "every hit was a near-tie, so face identity was never \
             differentially compared; {}",
            fuzz::replay()
        ),
    );
    seen.require(
        "agreed misses",
        10,
        &format!(
            "the sweep never missed, so the typed-miss path was never \
             differentially compared; {}",
            fuzz::replay()
        ),
    );
}

/// **The seam question**: a ray aimed exactly at a box CORNER — the
/// point three faces share — must hit, not fall between them. Closed
/// triangle boundaries are what makes that true; the answer is one of
/// the three incident faces, and it is the same one every time.
#[test]
fn a_ray_through_a_shared_corner_hits_and_is_repeatable() {
    let doc = ProfileDoc::empty_derived("gui1_r2_corner", Tol::witness());
    let (doc, n) = box_node(doc, 0.0, 0.0, 1.0, 1.0);
    let ev = run(&doc);
    let m = mesh_of(&ev, n);
    let p = MeshPick::build(&m).expect("mesh indexes");
    let targets = [PickTarget {
        node: n,
        body: 0,
        pick: &p,
    }];
    // Straight at the (1,1,1) corner along the body diagonal.
    let r = ray([3.0, 3.0, 3.0], [-1.0, -1.0, -1.0]);
    let hit = pick_face(&ev, &targets, &r)
        .expect("no error")
        .expect("a corner ray hits — closed triangle boundaries");
    assert_eq!(hit.t, 2.0, "dyadic corner hit is exact");
    assert_eq!(
        (hit.point.x, hit.point.y, hit.point.z),
        (1.0, 1.0, 1.0),
        "the hit point is the corner"
    );
    for _ in 0..8 {
        let again = pick_face(&ev, &targets, &r)
            .expect("no error")
            .expect("hits");
        assert_eq!(again.name, hit.name, "corner pick is repeatable");
        assert_eq!(again.t.to_bits(), hit.t.to_bits());
    }
    // The oracle sees the same corner hit.
    let want = brute_nearest(&[&m], &r).expect("oracle sees the corner hit");
    assert!((want.0 - hit.t).abs() < 1e-12);
}

/// A ray lying exactly IN a face plane. Möller–Trumbore's determinant
/// is zero for every triangle of that face, so the coplanar face is
/// never hit; the ray still meets the box's other faces, and the
/// service must answer one of THOSE rather than a miss or a panic.
#[test]
fn a_ray_in_a_face_plane_answers_from_the_transverse_faces() {
    let doc = ProfileDoc::empty_derived("gui1_r2_coplanar", Tol::witness());
    let (doc, n) = box_node(doc, 0.0, 0.0, 1.0, 1.0);
    let ev = run(&doc);
    let m = mesh_of(&ev, n);
    let p = MeshPick::build(&m).expect("mesh indexes");
    let targets = [PickTarget {
        node: n,
        body: 0,
        pick: &p,
    }];
    // In the z = 1 (top) plane, travelling +x through the middle.
    let r = ray([-2.0, 0.5, 1.0], [1.0, 0.0, 0.0]);
    let hit = pick_face(&ev, &targets, &r)
        .expect("no error")
        .expect("the -x face is met edge-on in its own plane's boundary");
    assert_eq!(hit.t, 2.0);
    assert_eq!(hit.point.x, 0.0, "enters at the -x face");
    let want = brute_nearest(&[&m], &r).expect("oracle agrees there is a hit");
    assert!(
        (want.0 - hit.t).abs() < 1e-12,
        "service {} vs oracle {}",
        hit.t,
        want.0
    );
}

/// Three bodies stacked along one ray: the service answers the
/// NEAREST, and answers each of the others once the nearer ones are
/// dropped from the target slice — the occlusion ordering a viewport
/// depends on, and the same answer regardless of target order.
#[test]
fn three_body_occlusion_peels_in_t_order() {
    let doc = ProfileDoc::empty_derived("gui1_r2_occl3", Tol::witness());
    let (doc, n0) = box_node(doc, 0.0, 0.0, 1.0, 1.0);
    let (doc, n1) = box_node(doc, 2.0, 0.0, 1.0, 1.0);
    let (doc, n2) = box_node(doc, 4.0, 0.0, 1.0, 1.0);
    let ev = run(&doc);
    let ms = [mesh_of(&ev, n0), mesh_of(&ev, n1), mesh_of(&ev, n2)];
    let ps: Vec<MeshPick> = ms
        .iter()
        .map(|m| MeshPick::build(m).expect("mesh indexes"))
        .collect();
    let mk = |i: usize, node| PickTarget {
        node,
        body: 0,
        pick: &ps[i],
    };
    let nodes = [n0, n1, n2];
    let r = ray([-1.0, 0.5, 0.5], [1.0, 0.0, 0.0]);
    let expect_t = [1.0, 3.0, 5.0];

    for drop in 0..3 {
        // Present the remaining targets in REVERSE order, so the
        // answer cannot come from slice position.
        let mut targets: Vec<PickTarget<'_>> = (drop..3).map(|i| mk(i, nodes[i])).collect();
        targets.reverse();
        let hit = pick_face(&ev, &targets, &r)
            .expect("no error")
            .expect("hits the nearest remaining body");
        assert_eq!(
            hit.node, nodes[drop],
            "the nearest remaining body wins, not the first in the slice"
        );
        assert_eq!(hit.t, expect_t[drop]);
    }
}

/// A mesh carrying a DEGENERATE (zero-area) triangle: the index still
/// builds, the degenerate triangle is never the answer (zero
/// determinant is a miss), and the surviving geometry still picks.
#[test]
fn a_degenerate_triangle_is_unhittable_and_harmless() {
    let doc = ProfileDoc::empty_derived("gui1_r2_degen", Tol::witness());
    let (doc, n) = box_node(doc, 0.0, 0.0, 1.0, 1.0);
    let ev = run(&doc);
    let mut m = mesh_of(&ev, n);
    // Collapse one triangle of the first patch onto a single vertex.
    let v = m.patches[0].triangles[0][0];
    m.patches[0].triangles[0] = [v, v, v];
    // And add a sliver: a triangle whose three corners are collinear.
    let base = m.positions.len() as u32;
    m.positions.push(Point3::new(-5.0, 0.5, 0.5));
    m.positions.push(Point3::new(-4.0, 0.5, 0.5));
    m.positions.push(Point3::new(-3.0, 0.5, 0.5));
    m.patches[0].triangles.push([base, base + 1, base + 2]);

    let p = MeshPick::build(&m).expect("degenerate geometry still indexes");
    let targets = [PickTarget {
        node: n,
        body: 0,
        pick: &p,
    }];
    // A ray straight through the collinear sliver: never a hit.
    let along = ray([-6.0, 0.5, 0.5], [1.0, 0.0, 0.0]);
    let hit = pick_face(&ev, &targets, &along)
        .expect("no error")
        .expect("the box behind the sliver still answers");
    assert_eq!(hit.t, 6.0, "the answer is the box face, not the sliver");
    // And the oracle, over the same corrupted mesh, agrees.
    let want = brute_nearest(&[&m], &along).expect("oracle hits the box");
    assert!((want.0 - hit.t).abs() < 1e-12);
}

/// **Provenance is held by convention, and a mismatch answers a
/// CONFIDENT WRONG NAME** — a review finding, recorded as a probe.
///
/// `PickTarget`'s doc says the `(node, body)` pair "must be the one
/// the mesh was tessellated from". Nothing checks it. Two sibling
/// extrudes mint face keys in their own arenas, so the keys collide
/// numerically: pairing body A's `MeshPick` with node B answers a
/// name belonging to B — no error, no miss, a plausible wrong answer
/// that a selection consumer cannot distinguish from a right one.
/// The service's typed-error posture covers node STANDING and does
/// not reach node/mesh PROVENANCE.
///
/// The row below asserts what a checked service would do — refuse, or
/// at least not answer a name for a face the mesh never carried. It
/// stays IGNORED after the fix pass, deliberately: the fix closes the
/// lane by CONSTRUCTION rather than by check — `NodePick` is the door
/// whose `(node, body)` ↔ mesh pairing cannot be mis-asserted (it
/// fetches the body from the evaluation payload itself), and
/// `PickTarget` now carries the loud contract naming this exact
/// failure mode. Raw target assembly remains verification-free — the
/// keys carry no node identity to check against — so this row is the
/// standing witness of what mis-assembling raw targets costs, kept as
/// documentation of the residual class (issue #1098).
///
/// **Revisited when GUI-2's cache landed, and kept ignored.** That
/// cache holds `NodePick`s and offers targets only through
/// `NodePick::target`, so the viewer never assembles a raw
/// `PickTarget` and never enters this lane — which is the outcome
/// #1098 asked for, and is why the row still cannot be gated: gating
/// it would mean checking a pairing the keys do not carry, and
/// deleting it would erase the record of what raw assembly still
/// costs a consumer who reaches for it.
#[test]
#[ignore = "R2 review finding: raw PickTarget provenance is by construction unverifiable; NodePick is the checked door — this row documents the residual raw-assembly class"]
fn a_mesh_paired_with_the_wrong_node_does_not_answer_a_name() {
    let doc = ProfileDoc::empty_derived("gui1_r2_provenance", Tol::witness());
    let (doc, a) = box_node(doc, 0.0, 0.0, 1.0, 1.0);
    let (doc, b) = box_node(doc, 5.0, 0.0, 1.0, 1.0);
    let ev = run(&doc);
    let ma = mesh_of(&ev, a);
    let pa = MeshPick::build(&ma).expect("mesh a indexes");
    // Body A's index, presented as node B's target — the mistake a
    // consumer holding a cache keyed by the wrong node id makes.
    let wrong = [PickTarget {
        node: b,
        body: 0,
        pick: &pa,
    }];
    let right = [PickTarget {
        node: a,
        body: 0,
        pick: &pa,
    }];
    let r = ray([0.5, 0.5, -2.0], [0.0, 0.0, 1.0]);
    let truth = pick_face(&ev, &right, &r)
        .expect("no error")
        .expect("hits")
        .name;
    let got = pick_face(&ev, &wrong, &r).expect("no error");
    assert!(
        got.is_none_or(|h| h.name == truth),
        "a mesh paired with the wrong node answered a name from that \
         other node — provenance is unchecked"
    );
}

/// A ray with an INFINITE direction component and a ray with a zero
/// direction: legal input, answered as the typed miss, never a panic
/// and never an error.
///
/// EVIDENCE ROW: it records that the degenerate rays a viewport can
/// produce (a zero-length pick direction from a collapsed camera, a
/// direction that overflowed) stay on the typed-miss path.
#[test]
fn degenerate_rays_are_typed_misses() {
    let doc = ProfileDoc::empty_derived("gui1_r2_degenray", Tol::witness());
    let (doc, n) = box_node(doc, 0.0, 0.0, 1.0, 1.0);
    let ev = run(&doc);
    let m = mesh_of(&ev, n);
    let p = MeshPick::build(&m).expect("mesh indexes");
    let targets = [PickTarget {
        node: n,
        body: 0,
        pick: &p,
    }];
    for r in [
        ray([0.5, 0.5, -2.0], [0.0, 0.0, 0.0]),
        ray([0.5, 0.5, -2.0], [0.0, 0.0, f64::INFINITY]),
        ray([f64::INFINITY, 0.5, -2.0], [0.0, 0.0, 1.0]),
    ] {
        let out = pick_face(&ev, &targets, &r).expect("degenerate rays are not errors");
        assert!(
            out.is_none(),
            "a degenerate ray answers the typed miss, got {out:?}"
        );
    }
}

/// **The G1 boundary, mechanically.** The hit-test service's own
/// source must not name an arena key type on any `pub` line: the
/// public answer is names, node ids, body indices and geometry.
///
/// A source-text guard rather than a type-level one because Rust has
/// no way to assert the absence of a field type; the same shape as
/// `pncad`'s façade census.
#[test]
fn no_arena_key_type_appears_on_a_public_line_of_the_service() {
    let src_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/resolve/pick.rs");
    let src = std::fs::read_to_string(&src_path)
        .unwrap_or_else(|e| panic!("reading {}: {e}", src_path.display()));
    // The public surface: every line of a `pub struct` / `pub enum`
    // body (their fields ARE the surface), and the SIGNATURE of every
    // `pub fn` up to its opening brace. A private helper nested inside
    // a public function's body is not public surface and is skipped —
    // which is exactly where `pick_face` legitimately holds a
    // `FaceKey`.
    //
    // Comments and literal bodies blanked once, for the whole file:
    // the needles are TYPE NAMES, so a key named in prose is not a
    // field, and every brace below is a real brace — the precondition
    // `test_utils::source::balanced_end` carves the item body on. The
    // view keeps byte offsets, so a position is still a line number.
    let code = test_utils::source::code_only(&src);
    let lines: Vec<&str> = code.lines().collect();
    let starts: Vec<usize> = std::iter::once(0)
        .chain(code.match_indices('\n').map(|(at, _)| at + 1))
        .collect();
    let line_of = |off: usize| starts.partition_point(|&s| s <= off) - 1;
    let mut surface: Vec<(usize, &str)> = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        let t = line.trim_start();
        let item = t.starts_with("pub struct") || t.starts_with("pub enum");
        if !item && !t.starts_with("pub fn") {
            continue;
        }
        // Where the head ends: the `{` opening a body, or the `;` of a
        // unit struct or a bodiless declaration, whichever comes first.
        let from = starts[i];
        let brace = code[from..].find('{').map(|o| from + o);
        let semi = code[from..].find(';').map(|o| from + o);
        let head_end = match (brace, semi) {
            (Some(b), Some(sc)) if sc < b => sc,
            (Some(b), _) => b,
            (None, Some(sc)) => sc,
            (None, None) => code.len() - 1,
        };
        // A `pub fn`'s surface is its signature; an item's surface is
        // its whole body, because the fields ARE the surface.
        let last = if item && Some(head_end) == brace {
            line_of(
                test_utils::source::balanced_end(&code, head_end).unwrap_or_else(|| {
                    panic!(
                        "{}:{}: a public item that never closes",
                        src_path.display(),
                        i + 1
                    )
                }),
            )
        } else {
            line_of(head_end)
        };
        surface.extend((i..=last).map(|n| (n + 1, lines[n])));
    }
    assert!(
        surface.len() > 20,
        "the scanner collected only {} public-surface lines — the file's \
         shape changed and this guard was about to pass vacuously",
        surface.len()
    );
    let mut offenders = Vec::new();
    for (n, line) in &surface {
        for key in ["FaceKey", "EdgeKey", "VertexKey", "ShellKey", "LoopKey"] {
            if line.contains(key) {
                offenders.push(format!("{}:{n}: {}", src_path.display(), line.trim()));
            }
        }
    }
    assert!(
        offenders.is_empty(),
        "an arena key type appears inside a public item of the hit-test \
         service — G1 says the layer-2/3 answer is names only:\n  {}",
        offenders.join("\n  ")
    );
    // Anti-vacuity: the scan must have seen the public items at all.
    assert!(
        code.contains("pub struct PickHit") && code.contains("pub fn pick_face"),
        "the scanner found neither PickHit nor pick_face — the file's \
         shape changed and this guard was about to pass vacuously"
    );
}
