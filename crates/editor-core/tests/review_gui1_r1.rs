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
//!    `resolve`), and the door's set rule over the exactly-tied — one
//!    face is one answer, several are the refusal, and no second key
//!    separates them — on every case.
//! 2. `coplanar_cross_target_tie_...` (static): two touching bodies
//!    whose faces meet the ray at the SAME exact `t` — a certified tie
//!    the door breaks by the narrower interval, so the answer is a
//!    function of the candidates and NOT of the slice order, and
//!    reversing the slice does not flip it. The PR's own occlusion row
//!    has distinct `t`s and cannot see this clause.
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

// `tests/fixture/` is named because the bodies these rows pick against are built
// there. A marker's own file is implicit; a sibling helper module is not.
test_utils::gated_to![
    "crates/editor-core/src/resolve/",
    "crates/bvh/src/",
    "crates/mesh/src/",
    "crates/geom-core/src/linalg/",
    "crates/editor-core/tests/fixture/",
];

use crate::fixture;

use editor_core::resolve::{TSpan, ray_triangle};
use editor_core::{
    CancelToken, EntityKey, EvalOptions, Evaluation, HitTestError, MeshPick, Node, PickTarget,
    ProfileDoc, Ray, RecipeNodeId, Resolution, RunCtx, ValuePayload, pick_face, resolve,
};
use fixture::{insert, len, on_frame};
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
    let (doc, profile) = on_frame(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(dx, 0.0), (dx + 1.0, 0.0), (dx + 1.0, 1.0), (dx, 1.0)]],
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
    resolved_patch(doc, ev, mesh, name).expect("resolved face has a patch in the mesh")
}

/// [`resolved_patch_index`] where the name may belong to another
/// mesh — the cross-target rows, where each answer is resolved
/// against whichever body drew it.
fn resolved_patch(
    doc: &ProfileDoc,
    ev: &Evaluation<f64>,
    mesh: &Mesh,
    name: &editor_core::StableName,
) -> Option<usize> {
    let Resolution::Resolved(r) = resolve(RunCtx { doc, eval: ev }, name) else {
        panic!("picked name resolves");
    };
    let EntityKey::Face(fk) = r.entity.key else {
        panic!("picked name denotes a face");
    };
    mesh.patches.iter().position(|p| p.face == fk)
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

/// The corners of flat triangle `flat` of target `target_pos`, in the
/// patch-major order the oracle and the service both count in.
fn flat_triangle(meshes: &[(usize, &Mesh)], target_pos: usize, flat: usize) -> [Point3<f64>; 3] {
    for &(pos, mesh) in meshes {
        if pos != target_pos {
            continue;
        }
        let mut i = 0usize;
        for patch in &mesh.patches {
            for tri in &patch.triangles {
                if i == flat {
                    return tri.map(|k| mesh.positions[k as usize]);
                }
                i += 1;
            }
        }
    }
    panic!("no flat triangle {flat} on target {target_pos}")
}

/// **The certified tie over the exactly-tied hits**, through
/// [`TSpan::survivors`] rather than a second spelling of it. The
/// oracle's exact arithmetic says WHICH hits tie — that half stays
/// deliberately independent of the door, which is the point of this
/// file; which of them the certified order keeps is the door's own
/// answer and is read from it.
fn tie_survivors(tied: &[OracleHit], meshes: &[(usize, &Mesh)], ray: &Ray) -> Vec<OracleHit> {
    let mut rows: Vec<(OracleHit, TSpan)> = tied
        .iter()
        .map(|h| {
            let tri = flat_triangle(meshes, h.target_pos, h.flat);
            let span = ray_triangle(ray, &tri).expect("an oracle hit is admitted by the door");
            (*h, span)
        })
        .collect();
    rows.sort_by_key(|(h, _)| (h.target_pos, h.flat));
    let spans: Vec<TSpan> = rows.iter().map(|&(_, s)| s).collect();
    TSpan::survivors(&spans)
        .into_iter()
        .map(|i| rows[i].0)
        .collect()
}

/// **What the door said**, as a list of hits: the one answer, the
/// empty miss, or the tied faces of a refusal. Every row here reads
/// the door this way, because the refusal is not an error to swallow
/// — it is the door naming more than one face, and each hit in it is
/// true.
fn answered(
    ev: &Evaluation<f64>,
    targets: &[PickTarget<'_>],
    ray: &Ray,
) -> Vec<editor_core::PickHit> {
    match pick_face(ev, targets, ray) {
        Ok(None) => Vec::new(),
        Ok(Some(hit)) => vec![hit],
        Err(HitTestError::Ambiguous { hits }) => hits,
        Err(other) => panic!("no hit-test error: {other:?}"),
    }
}

/// The distinct patches a set of answers resolves to, sorted.
fn patches_of(
    doc: &ProfileDoc,
    ev: &Evaluation<f64>,
    meshes: &[&Mesh],
    hits: &[editor_core::PickHit],
) -> Vec<usize> {
    let mut out: Vec<usize> = hits
        .iter()
        .map(|h| {
            meshes
                .iter()
                .find_map(|m| resolved_patch(doc, ev, m, &h.name))
                .unwrap_or_else(|| panic!("the answered name resolves to a drawn patch"))
        })
        .collect();
    out.sort_unstable();
    out.dedup();
    out
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
/// the exact oracle including what the door does with a certified tie
/// — answer the one face, or refuse with every tied face.
/// Every ray here has integer origin/direction and hits at integer
/// `t` through triangles whose determinants make the service's f64
/// arithmetic exact, so the assertions are equality, not tolerance.
#[test]
fn dyadic_battery_pins_faces_edges_corners_and_the_certified_tie() {
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
    let scaled_targets = [PickTarget::new(&ev, ext, 0, &scaled_pick)];
    for (ci, (o, d)) in cases.iter().enumerate() {
        let o2 = [
            (o[0] * 2.0) as i128,
            (o[1] * 2.0) as i128,
            (o[2] * 2.0) as i128,
        ];
        let d2 = [
            (d[0] * 2.0) as i128,
            (d[1] * 2.0) as i128,
            (d[2] * 2.0) as i128,
        ];
        let hits = oracle_hits(&scaled_meshes, o2, d2);
        let win = oracle_winner(&hits).unwrap_or_else(|| panic!("case {ci}: oracle hit expected"));
        let r2 = ray(
            [o[0] * 2.0, o[1] * 2.0, o[2] * 2.0],
            [d[0] * 2.0, d[1] * 2.0, d[2] * 2.0],
        );
        let said = answered(&ev, &scaled_targets, &r2);
        assert!(
            !said.is_empty(),
            "case {ci}: service hit expected (o={o:?} d={d:?})"
        );
        for hit in &said {
            assert_eq!(
                hit.t,
                win.t_f64(),
                "case {ci}: exact dyadic t (o={o:?} d={d:?})"
            );
        }
        let tied: Vec<OracleHit> = hits.iter().copied().filter(|h| h.t_ties(&win)).collect();
        let survivors = tie_survivors(&tied, &scaled_meshes, &r2);
        let mut expected: Vec<usize> = survivors.iter().map(|h| h.patch).collect();
        expected.sort_unstable();
        expected.dedup();
        assert_eq!(
            patches_of(&doc, &ev, &[&mesh], &said),
            expected,
            "case {ci}: the door names exactly the faces the certified tie keeps — one of them \
             when the tie is one face, all of them as a refusal when it is several (o={o:?} \
             d={d:?}; oracle hits {hits:?}; tied {tied:?})"
        );
    }
}

/// Row 2 — the coplanar cross-target tie: cubes `[0,1]` and `[1,2]`
/// share the plane `x = 1`; a ray running INSIDE that plane first
/// touches both bodies at the same exact `t` (their `y = 0` faces'
/// shared edge point). Their `t` intervals overlap, so this is a
/// CERTIFIED TIE between two FACES on two bodies, and the door
/// refuses with both — one hit each, both true, at the same `t`.
/// **The refusal is a function of the candidates and not of the slice
/// order**: reversing the slice reverses the LIST and nothing else,
/// which is the determinism property this row pins. The parallel
/// `x = 1` faces themselves are misses (zero determinant), which this
/// row also witnesses through both resolved patches being `y = 0`
/// patches.
#[test]
fn a_coplanar_cross_target_tie_refuses_with_both_bodies() {
    let doc = ProfileDoc::empty_derived("r1_xtie", Tol::witness());
    let (doc, a) = cube_doc_node(doc, 0.0); // x ∈ [0, 1]
    let (doc, b) = cube_doc_node(doc, 1.0); // x ∈ [1, 2] — touching
    let ev = run(&doc);
    let mesh_a = mesh_of(&ev, a);
    let mesh_b = mesh_of(&ev, b);
    let pick_a = MeshPick::build(&mesh_a).expect("mesh a");
    let pick_b = MeshPick::build(&mesh_b).expect("mesh b");
    let ta = PickTarget::new(&ev, a, 0, &pick_a);
    let tb = PickTarget::new(&ev, b, 0, &pick_b);
    // In the x = 1 plane, aimed at the shared edge point (1, 0, 1/2),
    // reaching it at t = 1 for both bodies' y = 0 faces.
    let r = ray([1.0, -1.0, 0.5], [0.0, 1.0, 0.0]);

    let said = answered(&ev, &[ta, tb], &r);
    assert_eq!(
        said.iter().map(|h| h.node).collect::<Vec<_>>(),
        vec![a, b],
        "both bodies are tied and neither is named alone: {said:?}"
    );
    for hit in &said {
        assert_eq!(hit.t, 1.0);
        assert!(
            hit.t_lo < hit.t && hit.t < hit.t_hi,
            "each tied hit is an interval around the exact t: {hit:?}"
        );
    }
    // The widths differ, and the refusal is deaf to that: what used to
    // be the second key is now only a measurement.
    let alone_a = pick_face(&ev, &[ta], &r)
        .expect("no error")
        .expect("a hits");
    let alone_b = pick_face(&ev, &[tb], &r)
        .expect("no error")
        .expect("b hits");
    assert_eq!((alone_a.t, alone_b.t), (1.0, 1.0));
    let (wa, wb) = (alone_a.t_hi - alone_a.t_lo, alone_b.t_hi - alone_b.t_lo);
    assert!(
        wb < wa,
        "the second body's claim is the better-certified one ({wb} against {wa}), and the \
         door refuses all the same"
    );

    let flipped = answered(&ev, &[tb, ta], &r);
    assert_eq!(
        flipped.iter().map(|h| h.node).collect::<Vec<_>>(),
        vec![b, a],
        "reversing the slice reverses the LIST — the certified tie is decided by the \
         candidates, not by the order they were offered in"
    );
    assert_eq!(
        flipped
            .iter()
            .map(|h| (h.t_lo.to_bits(), h.t_hi.to_bits()))
            .rev()
            .collect::<Vec<_>>(),
        said.iter()
            .map(|h| (h.t_lo.to_bits(), h.t_hi.to_bits()))
            .collect::<Vec<_>>(),
        "and carries the same intervals, to the bit"
    );

    // Both tied faces are y = 0 patches (the in-plane x = 1 faces are
    // parallel-miss by the documented zero-determinant rule).
    for (hit, mesh) in said.iter().zip([&mesh_a, &mesh_b]) {
        let pi = resolved_patch_index(&doc, &ev, mesh, &hit.name);
        assert!(
            mesh.patches[pi]
                .triangles
                .iter()
                .flatten()
                .all(|&i| mesh.positions[i as usize].y == 0.0),
            "each tied face lies on the y = 0 face"
        );
    }
}

/// Row 3 — random integer rays vs the exact oracle (counterexample
/// search; varying seed, effort dial). Unique oracle winner ⇒ the
/// service must answer that one face and agree to ≤ 4 ulps on `t`
/// (distinct rational `t`s here differ by ≥ 1/64, so agreement on the
/// face is never blurred by f64 noise); exactly-tied oracle winners ⇒
/// every face the service names is in the tied minimal set; oracle
/// miss ⇒ the typed miss.
#[test]
fn random_integer_rays_match_the_exact_oracle() {
    let doc = ProfileDoc::empty_derived("r1_oracle", Tol::witness());
    let (doc, ext) = cube_doc_node(doc, 0.0);
    let ev = run(&doc);
    let mesh = mesh_of(&ev, ext);
    let pick = MeshPick::build(&mesh).expect("well-formed mesh");
    let targets = [PickTarget::new(&ev, ext, 0, &pick)];
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
        let said = answered(&ev, &targets, &r);
        match (win, said.first()) {
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
            (Some(w), Some(_)) => {
                hits_seen += 1;
                for h in &said {
                    assert!(
                        (h.t - w.t_f64()).abs() <= 4.0 * f64::EPSILON * w.t_f64().abs().max(1.0),
                        "case {case}: t {} vs exact {} (o={o:?} d={d:?}); {}",
                        h.t,
                        w.t_f64(),
                        fuzz::replay()
                    );
                }
                let got = patches_of(&doc, &ev, &[&mesh], &said);
                let tied: Vec<usize> = hits
                    .iter()
                    .filter(|x| x.t_ties(&w))
                    .map(|x| x.patch)
                    .collect();
                if tied.len() == 1 {
                    assert_eq!(
                        got,
                        vec![w.patch],
                        "case {case}: unique oracle winner, answered and not refused \
                         (o={o:?} d={d:?}); {}",
                        fuzz::replay()
                    );
                } else {
                    assert!(
                        got.iter().all(|p| tied.contains(p)),
                        "case {case}: a named face outside the exactly-tied set \
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
