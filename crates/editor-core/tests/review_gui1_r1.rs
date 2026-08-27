//! Reviewer consumer suite for GUI-1 Part B (the hit-test service),
//! R1 lane — an INDEPENDENT derivation of the PR's claims through the
//! PUBLIC doors only, checked against an exact-rational Möller–Trumbore
//! oracle of this reviewer's own construction (i128 arithmetic over the
//! integer-valued cube geometry; no floating point in any acceptance
//! decision).
//!
//! Rows:
//!
//! 1. `dyadic_battery_...` (static fixture, shape 2 of
//!    `memories/test-suite-cost.md` — no seed): rays at all 6 face
//!    centers, down all 12 edges, into all 8 corners, and from the
//!    interior, on dyadic geometry where every winning computation is
//!    exact — asserting hit `t`, the resolved face (via public
//!    `resolve`), and the documented tie-break (earliest flat
//!    patch-major triangle among the exactly-tied) on every case.
//! 2. `coplanar_cross_target_tie_...` (static): two touching bodies
//!    whose faces meet the ray at the SAME exact `t` — the winner is
//!    the earlier TARGET POSITION (not node id), so reversing the
//!    slice flips the winner. The PR's own occlusion row has distinct
//!    `t`s and cannot see this clause.
//! 3. `random_integer_rays_match_the_exact_oracle` (counterexample
//!    search — varying seed, effort dial): random integer rays against
//!    the cube; the exact oracle computes every triangle hit as a
//!    rational, picks the documented winner, and the service must
//!    agree (face and `t`); an oracle miss must be the typed miss.
//!    Distinct rational `t`s on this geometry differ by ≥ 1/64, so
//!    f64 noise cannot blur the comparison; exact ties assert
//!    membership in the tied minimal set (the strict flat-order pin
//!    for ties lives in row 1, where every computation is exact).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod fixture;

use editor_core::{
    CancelToken, EntityKey, EvalOptions, Evaluation, MeshPick, Node, PickTarget, ProfileDoc, Ray,
    RecipeNodeId, Resolution, RunCtx, ValuePayload, pick_face, resolve,
};
use fixture::{desc, insert, len};
use geom_core::{Point3, Tol, Vec3};
use mesh::Mesh;
use test_utils::fuzz;
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

/// A unit cube `[dx, dx+1] × [0,1] × [0,1]` as one extrude node.
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

fn mesh_of(ev: &Evaluation<f64>, node: RecipeNodeId) -> Mesh {
    let body: &Body<f64> = match &ev.value(node).expect("extrude evaluates").payload {
        ValuePayload::Body(b) => b,
        other => panic!("extrude payload is a body, got {}", other.kind_name()),
    };
    mesh::tessellate(body, DELTA, Tol::witness()).expect("box tessellates")
}

/// The patch index a pick's name resolves to, recovered through the
/// PUBLIC resolution door — the arena-key-free round trip.
fn resolved_patch_index(
    doc: &ProfileDoc,
    ev: &Evaluation<f64>,
    mesh: &Mesh,
    name: &editor_core::StableName,
) -> usize {
    let Resolution::Resolved(r) = resolve(RunCtx { doc, eval: ev }, name) else {
        panic!("picked name resolves");
    };
    let EntityKey::Face(fk) = r.entity.key else {
        panic!("picked name denotes a face");
    };
    mesh.patches
        .iter()
        .position(|p| p.face == fk)
        .expect("resolved face has a patch in the mesh")
}

/// Exact integer coordinates of a mesh position (this suite's meshes
/// are dyadic-integer by construction; a fractional coordinate is a
/// fixture bug, surfaced loudly).
fn int_point(p: Point3<f64>) -> [i128; 3] {
    let c = [p.x, p.y, p.z];
    let mut out = [0i128; 3];
    for a in 0..3 {
        assert!(
            c[a].fract() == 0.0 && c[a].abs() <= 1e15,
            "fixture premise: integer mesh coordinates, got {}",
            c[a]
        );
        out[a] = c[a] as i128;
    }
    out
}

fn sub(a: [i128; 3], b: [i128; 3]) -> [i128; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
fn cross(a: [i128; 3], b: [i128; 3]) -> [i128; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}
fn dot(a: [i128; 3], b: [i128; 3]) -> i128 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

/// One exact oracle hit: `t = t_num / t_den` (den > 0), at flat
/// triangle position `flat` of target `target_pos`.
#[derive(Clone, Copy, Debug)]
struct OracleHit {
    t_num: i128,
    t_den: i128,
    target_pos: usize,
    flat: usize,
    patch: usize,
}

impl OracleHit {
    /// Exact `(t, target_pos, flat) <= other` — the documented order.
    fn le(&self, o: &OracleHit) -> bool {
        let l = self.t_num * o.t_den;
        let r = o.t_num * self.t_den;
        l < r || (l == r && (self.target_pos, self.flat) <= (o.target_pos, o.flat))
    }
    fn t_ties(&self, o: &OracleHit) -> bool {
        self.t_num * o.t_den == o.t_num * self.t_den
    }
    fn t_f64(&self) -> f64 {
        self.t_num as f64 / self.t_den as f64
    }
}

/// The exact both-sided closed Möller–Trumbore oracle, derived from
/// the documented contract (`u ∈ [0,1]`, `v ≥ 0`, `u + v ≤ 1`,
/// `t ≥ 0`, all closed; zero determinant is a miss), in exact i128
/// rational arithmetic on integer geometry.
fn oracle_hits(
    meshes: &[(usize, &Mesh)], // (target_pos, mesh) in slice order
    o: [i128; 3],
    d: [i128; 3],
) -> Vec<OracleHit> {
    let mut hits = Vec::new();
    for &(target_pos, mesh) in meshes {
        let mut flat = 0usize;
        for (pi, patch) in mesh.patches.iter().enumerate() {
            for tri in &patch.triangles {
                let a = int_point(mesh.positions[tri[0] as usize]);
                let b = int_point(mesh.positions[tri[1] as usize]);
                let c = int_point(mesh.positions[tri[2] as usize]);
                let e1 = sub(b, a);
                let e2 = sub(c, a);
                let p = cross(d, e2);
                let det = dot(e1, p);
                if det != 0 {
                    let s = sub(o, a);
                    let q = cross(s, e1);
                    let sd = det.signum();
                    let ad = det.abs();
                    let u = dot(s, p) * sd;
                    let v = dot(d, q) * sd;
                    let t = dot(e2, q) * sd;
                    if u >= 0 && u <= ad && v >= 0 && u + v <= ad && t >= 0 {
                        hits.push(OracleHit {
                            t_num: t,
                            t_den: ad,
                            target_pos,
                            flat,
                            patch: pi,
                        });
                    }
                }
                flat += 1;
            }
        }
    }
    hits
}

fn oracle_winner(hits: &[OracleHit]) -> Option<OracleHit> {
    let mut best: Option<OracleHit> = None;
    for h in hits {
        if best.as_ref().is_none_or(|b| h.le(b)) {
            best = Some(*h);
        }
    }
    best
}

/// Row 1 — the dyadic battery: face centers, all 12 edges, all 8
/// corners, and an interior (both-sided) origin, each checked against
/// the exact oracle including the documented flat-order tie-break.
/// Every ray here has integer origin/direction and hits at integer
/// `t` through triangles whose determinants make the service's f64
/// arithmetic exact, so the assertions are equality, not tolerance.
#[test]
fn dyadic_battery_pins_faces_edges_corners_and_tiebreak() {
    let doc = ProfileDoc::empty_derived("r1_battery", Tol::witness());
    let (doc, ext) = cube_doc_node(doc, 0.0);
    let ev = run(&doc);
    let mesh = mesh_of(&ev, ext);

    // (origin, dir): 6 face centers × axis rays; 12 edge rays (down
    // the edge's direction? no — ACROSS each edge: diagonal rays whose
    // first contact is exactly the edge midpoint); 8 corner rays
    // (space-diagonal, first contact exactly the corner); 1 interior.
    let mut cases: Vec<([f64; 3], [f64; 3])> = vec![
        // Face centers (from 2 out, in): axis rays. (Non-integer
        // origin components stay dyadic; directions integer.)
        ([0.5, 0.5, -2.0], [0.0, 0.0, 1.0]),
        ([0.5, 0.5, 3.0], [0.0, 0.0, -1.0]),
        ([-2.0, 0.5, 0.5], [1.0, 0.0, 0.0]),
        ([3.0, 0.5, 0.5], [-1.0, 0.0, 0.0]),
        ([0.5, -2.0, 0.5], [0.0, 1.0, 0.0]),
        ([0.5, 3.0, 0.5], [0.0, -1.0, 0.0]),
        // Interior origin: both-sided test, exits through x = 1.
        ([0.5, 0.5, 0.5], [1.0, 0.0, 0.0]),
    ];
    // The 12 edges: for each pair of axes (a, b) and each corner
    // combination on those axes, aim a face-diagonal ray at the edge
    // midpoint from 1 unit out on both axes.
    for (a, b) in [(0usize, 1usize), (0, 2), (1, 2)] {
        for (sa, sb) in [(0.0, 0.0), (0.0, 1.0), (1.0, 0.0), (1.0, 1.0)] {
            let mut origin = [0.5; 3]; // the free axis stays at the middle
            let mut dir = [0.0; 3];
            origin[a] = if sa == 0.0 { -1.0 } else { 2.0 };
            origin[b] = if sb == 0.0 { -1.0 } else { 2.0 };
            dir[a] = if sa == 0.0 { 1.0 } else { -1.0 };
            dir[b] = if sb == 0.0 { 1.0 } else { -1.0 };
            cases.push((origin, dir));
        }
    }
    // The 8 corners: space-diagonal rays whose first contact is the
    // corner itself.
    for cx in [0.0, 1.0] {
        for cy in [0.0, 1.0] {
            for cz in [0.0, 1.0] {
                let origin = [
                    if cx == 0.0 { -1.0 } else { 2.0 },
                    if cy == 0.0 { -1.0 } else { 2.0 },
                    if cz == 0.0 { -1.0 } else { 2.0 },
                ];
                let dir = [
                    if cx == 0.0 { 1.0 } else { -1.0 },
                    if cy == 0.0 { 1.0 } else { -1.0 },
                    if cz == 0.0 { 1.0 } else { -1.0 },
                ];
                cases.push((origin, dir));
            }
        }
    }

    // The oracle wants integer coordinates and some origins sit at
    // dyadic halves, so BOTH sides of the comparison run on the
    // doubled lattice: mesh positions, origins, and directions all
    // ×2. Doubling positions and origin doubles every hit's world
    // coordinates; doubling the direction as well leaves each hit's
    // parameter `t` exactly what it was on the unit lattice — and the
    // service side picks against a `MeshPick` built from the doubled
    // mesh (legal precisely because `MeshPick` copies geometry out:
    // the claim-6 self-containment in action), inverting to the same
    // face names through the same evaluation.
    let mut scaled = mesh.clone();
    for p in &mut scaled.positions {
        *p = Point3::new(p.x * 2.0, p.y * 2.0, p.z * 2.0);
    }
    let scaled_meshes = [(0usize, &scaled)];
    let scaled_pick = MeshPick::build(&scaled).expect("scaled mesh builds");
    let scaled_targets = [PickTarget {
        node: ext,
        body: 0,
        pick: &scaled_pick,
    }];
    for (ci, (o, d)) in cases.iter().enumerate() {
        let o2 = [
            (o[0] * 2.0) as i128,
            (o[1] * 2.0) as i128,
            (o[2] * 2.0) as i128,
        ];
        let d2 = [(d[0] * 2.0) as i128, (d[1] * 2.0) as i128, (d[2] * 2.0) as i128];
        let hits = oracle_hits(&scaled_meshes, o2, d2);
        let win = oracle_winner(&hits).unwrap_or_else(|| panic!("case {ci}: oracle hit expected"));
        let r2 = ray(
            [o[0] * 2.0, o[1] * 2.0, o[2] * 2.0],
            [d[0] * 2.0, d[1] * 2.0, d[2] * 2.0],
        );
        let hit = pick_face(&ev, &scaled_targets, &r2)
            .expect("no hit-test error")
            .unwrap_or_else(|| panic!("case {ci}: service hit expected"));
        assert_eq!(
            hit.t,
            win.t_f64(),
            "case {ci}: exact dyadic t (o={o:?} d={d:?})"
        );
        let got_patch = resolved_patch_index(&doc, &ev, &mesh, &hit.name);
        assert_eq!(
            got_patch, win.patch,
            "case {ci}: documented winner patch (o={o:?} d={d:?}; oracle hits {hits:?})"
        );
    }
}

/// Row 2 — the coplanar cross-target tie: cubes `[0,1]` and `[1,2]`
/// share the plane `x = 1`; a ray running INSIDE that plane first
/// touches both bodies at the same exact `t` (their `y = 0` faces'
/// shared edge point). The documented winner is the earlier TARGET
/// POSITION, so reversing the slice must flip the winning node — and
/// the parallel `x = 1` faces themselves are misses (zero
/// determinant), which this row also witnesses through the resolved
/// patch being a `y = 0` patch, not an `x = 1` patch.
#[test]
fn coplanar_cross_target_tie_resolves_by_target_position() {
    let doc = ProfileDoc::empty_derived("r1_xtie", Tol::witness());
    let (doc, a) = cube_doc_node(doc, 0.0); // x ∈ [0, 1]
    let (doc, b) = cube_doc_node(doc, 1.0); // x ∈ [1, 2] — touching
    let ev = run(&doc);
    let mesh_a = mesh_of(&ev, a);
    let mesh_b = mesh_of(&ev, b);
    let pick_a = MeshPick::build(&mesh_a).expect("mesh a");
    let pick_b = MeshPick::build(&mesh_b).expect("mesh b");
    let ta = PickTarget {
        node: a,
        body: 0,
        pick: &pick_a,
    };
    let tb = PickTarget {
        node: b,
        body: 0,
        pick: &pick_b,
    };
    // In the x = 1 plane, aimed at the shared edge point (1, 0, 1/2),
    // reaching it at t = 1 for both bodies' y = 0 faces.
    let r = ray([1.0, -1.0, 0.5], [0.0, 1.0, 0.0]);

    let hit = pick_face(&ev, &[ta, tb], &r)
        .expect("no error")
        .expect("tie ray hits");
    assert_eq!(hit.t, 1.0);
    assert_eq!(hit.node, a, "earlier target wins the exact tie");

    let flipped = pick_face(&ev, &[tb, ta], &r)
        .expect("no error")
        .expect("tie ray hits");
    assert_eq!(flipped.t, 1.0);
    assert_eq!(
        flipped.node, b,
        "the tie-break is target POSITION, not node identity"
    );

    // The winning face is a y = 0 patch (the in-plane x = 1 faces are
    // parallel-miss by the documented zero-determinant rule).
    let pi = resolved_patch_index(&doc, &ev, &mesh_a, &hit.name);
    let patch = &mesh_a.patches[pi];
    assert!(
        patch
            .triangles
            .iter()
            .flatten()
            .all(|&i| mesh_a.positions[i as usize].y == 0.0),
        "winner lies on the y = 0 face"
    );
}

/// Row 3 — random integer rays vs the exact oracle (counterexample
/// search; varying seed, effort dial). Unique oracle winner ⇒ the
/// service must agree exactly on the face and to ≤ 4 ulps on `t`
/// (distinct rational `t`s here differ by ≥ 1/64, so agreement on the
/// face is never blurred by f64 noise); exactly-tied oracle winners ⇒
/// the service's face must be among the tied minimal set; oracle miss
/// ⇒ the typed miss.
#[test]
fn random_integer_rays_match_the_exact_oracle() {
    let doc = ProfileDoc::empty_derived("r1_oracle", Tol::witness());
    let (doc, ext) = cube_doc_node(doc, 0.0);
    let ev = run(&doc);
    let mesh = mesh_of(&ev, ext);
    let pick = MeshPick::build(&mesh).expect("well-formed mesh");
    let targets = [PickTarget {
        node: ext,
        body: 0,
        pick: &pick,
    }];
    let meshes = [(0usize, &mesh)];

    let mut rng = fuzz::start("review r1: pick_face exact-oracle sweep");
    let mut hits_seen = 0usize;
    let total = fuzz::scaled(120);
    for case in 0..total {
        let mut o = [0i128; 3];
        let mut d = [0i128; 3];
        for a in 0..3 {
            o[a] = rng.below(7) as i128 - 3; // [-3, 3]
            d[a] = rng.below(5) as i128 - 2; // [-2, 2], zeros welcome
        }
        let r = ray(
            [o[0] as f64, o[1] as f64, o[2] as f64],
            [d[0] as f64, d[1] as f64, d[2] as f64],
        );
        let hits = oracle_hits(&meshes, o, d);
        let win = oracle_winner(&hits);
        let got = pick_face(&ev, &targets, &r).expect("no hit-test error");
        match (win, got) {
            (None, None) => {}
            (None, Some(h)) => panic!(
                "case {case}: service hit {h:?} where the exact oracle misses \
                 (o={o:?} d={d:?}); {}",
                fuzz::replay()
            ),
            (Some(w), None) => panic!(
                "case {case}: service missed; oracle wins with {w:?} \
                 (o={o:?} d={d:?}); {}",
                fuzz::replay()
            ),
            (Some(w), Some(h)) => {
                hits_seen += 1;
                assert!(
                    (h.t - w.t_f64()).abs() <= 4.0 * f64::EPSILON * w.t_f64().abs().max(1.0),
                    "case {case}: t {} vs exact {} (o={o:?} d={d:?}); {}",
                    h.t,
                    w.t_f64(),
                    fuzz::replay()
                );
                let got_patch = resolved_patch_index(&doc, &ev, &mesh, &h.name);
                let tied: Vec<usize> = hits
                    .iter()
                    .filter(|x| x.t_ties(&w))
                    .map(|x| x.patch)
                    .collect();
                if tied.len() == 1 {
                    assert_eq!(
                        got_patch, w.patch,
                        "case {case}: unique oracle winner (o={o:?} d={d:?}); {}",
                        fuzz::replay()
                    );
                } else {
                    assert!(
                        tied.contains(&got_patch),
                        "case {case}: winner outside the exactly-tied set \
                         {tied:?} (o={o:?} d={d:?}); {}",
                        fuzz::replay()
                    );
                }
            }
        }
    }
    // Anti-vacuity is structural, not searched (the battery row covers
    // guaranteed hits); still, a sweep where nothing ever hit would be
    // a broken generator worth hearing about.
    assert!(
        hits_seen > 0,
        "no draw hit the cube — generator shape broke; {}",
        fuzz::replay()
    );
}
