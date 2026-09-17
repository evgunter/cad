//! The pick door's early-out and its certified tie, pinned THROUGH
//! [`editor_core::pick_face`] — over real targets, real tables and the
//! real traversal, never a restatement of it.
//!
//! A row that restates the walk pins the restatement: the traversal's
//! bound is the door's own, and a mutant in `pick.rs` has to be able
//! to red these. Each row here therefore builds a mesh, indexes it and
//! asks the public door.
//!
//! The two early-out fixtures were authored by the EDIT-PICK3 review
//! lanes `pick3-r1` and `pick3-r2`, where they were red: the break
//! fired on `t_hi(best) < t_enter(cand)` while membership of the
//! certified tie is `t_lo(cand) ≤ t_hi(best)`, so a candidate whose
//! interval reached back below its own box entry was pruned — and, in
//! `pick3-r2`'s fixture, the answer then depended on the order the
//! targets were offered in. The fix pass derived the margin
//! (`early_out_margin`) that closes the gap; the fixtures stay as the
//! rows that hold it closed.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::float_cmp
)]

test_utils::gated_to![
    "crates/editor-core/src/resolve/",
    "crates/bvh/src/",
    "crates/editor-core/tests/fixture/",
];

use crate::fixture;

use bvh::{Aabb, Ray};
use editor_core::resolve::{TSpan, crossing, ray_triangle};
use editor_core::{
    CancelToken, EvalOptions, Evaluation, MeshPick, Node, PickHit, PickTarget, ProfileDoc,
    RecipeNodeId, ValuePayload, pick_face,
};
use fixture::{insert, len, on_frame};
use geom_core::{Point3, Tol, Vec3};
use mesh::Mesh;
use topo::Body;

// ---------------------------------------------------------------
// The fixture: a cube's evaluation, whose patches lend their face
// keys to the triangles a row wants indexed.
// ---------------------------------------------------------------

fn run(doc: &ProfileDoc) -> Evaluation<f64> {
    editor_core::evaluate::<f64>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

fn cube(doc: ProfileDoc) -> (ProfileDoc, RecipeNodeId) {
    let (doc, profile) = on_frame(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
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
    mesh::tessellate(body, 0.1, Tol::witness()).expect("box tessellates")
}

/// A mesh carrying `tris` as one triangle per patch, under the first
/// patches' face keys of `base`, so [`pick_face`] names the winner
/// through the real tables. `MeshPick` copies geometry out, so the
/// index is exactly these triangles.
fn mesh_over(base: &Mesh, tris: &[[Point3<f64>; 3]], first_patch: usize) -> Mesh {
    let mut positions = Vec::new();
    let mut patches = Vec::new();
    for (i, tri) in tris.iter().enumerate() {
        let k = positions.len() as u32;
        positions.extend_from_slice(tri);
        let mut patch = base.patches[first_patch + i].clone();
        patch.triangles = vec![[k, k + 1, k + 2]];
        patches.push(patch);
    }
    Mesh {
        positions,
        patches,
        boundaries: Vec::new(),
    }
}

/// The door over a set of triangles, one target per group.
struct Door {
    ev: Evaluation<f64>,
    node: RecipeNodeId,
    base: Mesh,
}

impl Door {
    fn new(name: &str) -> Self {
        let doc = ProfileDoc::empty_derived(name, Tol::witness());
        let (doc, node) = cube(doc);
        let ev = run(&doc);
        let base = mesh_of(&ev, node);
        Self { ev, node, base }
    }

    /// `pick_face` over one target per group of triangles, in the
    /// order given.
    fn ask(&self, groups: &[&[[Point3<f64>; 3]]], ray: &Ray) -> PickHit {
        self.ask_from(groups, &vec![0usize; groups.len()], ray)
    }

    /// [`Door::ask`] with each target's triangles carried under the
    /// face keys starting at `first_patch[i]`, so two targets can hold
    /// the SAME triangle under DIFFERENT faces.
    fn ask_from(
        &self,
        groups: &[&[[Point3<f64>; 3]]],
        first_patch: &[usize],
        ray: &Ray,
    ) -> PickHit {
        let meshes: Vec<Mesh> = groups
            .iter()
            .zip(first_patch)
            .map(|(g, &p)| mesh_over(&self.base, g, p))
            .collect();
        let picks: Vec<MeshPick> = meshes
            .iter()
            .map(|m| MeshPick::build(m).expect("the triangles index"))
            .collect();
        let targets: Vec<PickTarget<'_>> = picks
            .iter()
            .map(|p| PickTarget::new(&self.ev, self.node, 0, p))
            .collect();
        pick_face(&self.ev, &targets, ray)
            .expect("no error")
            .expect("a hit")
    }
}

fn entry(ray: &Ray, tri: &[Point3<f64>; 3]) -> f64 {
    ray.slab_enter(&Aabb::from_points(*tri).expect("three points box"))
        .expect("the ray enters the box")
}

fn span_of(ray: &Ray, tri: &[Point3<f64>; 3], what: &str) -> TSpan {
    ray_triangle(ray, tri).unwrap_or_else(|| panic!("{what} is admitted: {:?}", crossing(ray, tri)))
}

// ---------------------------------------------------------------
// `pick3-r1`'s fixture: a candidate whose whole interval reaches back
// before its own box.
// ---------------------------------------------------------------

/// The family both early-out rows use: a triangle whose plane the ray
/// all but contains, entered EXACTLY at the corner the ray crosses, so
/// the box entry and the hit coincide and the whole of the candidate's
/// interval reaches back before its own box.
///
/// `zeta = 2^-20`, `xi = k*zeta*EPSILON`; `a = corner`,
/// `e1 = scale*(1, 0, zeta + xi)`, `e2 = scale*(0, 1, 0)`,
/// `d = (1, 1, zeta)`. Then `u = v = 0` exactly and the hit is the
/// corner itself. Authored by review lane `pick3-r1`.
fn corner_tangent(k: f64, scale: f64, at: f64) -> [Point3<f64>; 3] {
    let zeta = 2f64.powi(-20);
    let xi = k * zeta * f64::EPSILON;
    let corner = Point3::new(at, at, at * zeta);
    [
        corner,
        corner + Vec3::new(scale, 0.0, scale * (zeta + xi)),
        corner + Vec3::new(0.0, scale, 0.0),
    ]
}

/// **The early-out keeps a candidate of the certified tie whose
/// interval reaches below its own box entry.**
///
/// A wide candidate at `t = 4` and a narrower one whose box the ray
/// does not enter until `t = 4.5` — past the wide one's upper end, so
/// the un-margined break fires — but not past the narrow one's own
/// lower end, so the narrow one is IN the tie by the door's own rule
/// and, being narrower, is what the tie-break takes. The two answers
/// are different faces, not different ULPs.
///
/// Authored by review lane `pick3-r1`, where it was red; re-expressed
/// through `pick_face` rather than through a hand-run traversal, so a
/// margin that goes missing from the door reds it.
#[test]
fn the_early_out_keeps_a_candidate_whose_interval_reaches_below_its_box() {
    let zeta = 2f64.powi(-20);
    let ray = Ray {
        origin: Point3::new(0.0, 0.0, 0.0),
        dir: Vec3::new(1.0, 1.0, zeta),
    };
    let tris = [
        corner_tangent(154.0, 1.0, 4.0),
        corner_tangent(238.0, 1.0, 4.5),
    ];
    let wide = span_of(&ray, &tris[0], "the wide candidate");
    let narrow = span_of(&ray, &tris[1], "the narrow candidate");
    let entry_narrow = entry(&ray, &tris[1]);
    // The premises, so the row cannot pass on a fixture that drifted.
    assert!(
        wide.t_hi < entry_narrow,
        "the un-margined break fires: {} < {entry_narrow}",
        wide.t_hi
    );
    assert!(
        narrow.t_lo <= wide.t_hi,
        "and yet the narrow candidate is IN the certified tie ({} <= {})",
        narrow.t_lo,
        wide.t_hi
    );
    assert!(
        narrow.width() < wide.width(),
        "and it is the narrower of the two, so the tie-break takes it"
    );
    assert_eq!(
        TSpan::best_of(&[wide, narrow]),
        Some(1),
        "the certified order over both candidates answers the narrow one"
    );

    let door = Door::new("pick3_early_out_r1");
    assert_eq!(
        door.ask(&[&tris], &ray).t,
        narrow.t,
        "and so does the door, with both triangles in one target"
    );
}

// ---------------------------------------------------------------
// `pick3-r2`'s fixture: the same gap, and the order of the targets.
// ---------------------------------------------------------------

/// `pick.rs`'s `near_tangent` fixture: a determinant certified at
/// `k / 6` of its own bound, `u = v = 0.5` exactly, `t = 1.5`.
/// Authored by review lane `pick3-r2`.
fn near_tangent(k: f64) -> (Ray, [Point3<f64>; 3]) {
    let zeta = 2f64.powi(-20);
    let xi = k * zeta * f64::EPSILON;
    let tri = [
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(1.0, 0.0, zeta + xi),
        Point3::new(0.0, 1.0, 0.0),
    ];
    let ray = Ray {
        origin: Point3::new(-1.0, -1.0, 0.5 * xi - zeta),
        dir: Vec3::new(1.0, 1.0, zeta),
    };
    (ray, tri)
}

/// The same shape at `k`, scaled by `lambda` and placed so `ray` meets
/// it at `u = v = 0.5` at parameter `t_a`.
fn near_tangent_copy(k: f64, lambda: f64, t_a: f64, ray: &Ray) -> [Point3<f64>; 3] {
    let zeta = 2f64.powi(-20);
    let xi = k * zeta * f64::EPSILON;
    let e1 = Vec3::new(lambda, 0.0, lambda * (zeta + xi));
    let e2 = Vec3::new(0.0, lambda, 0.0);
    let p = ray.origin + ray.dir * t_a;
    let a = p - (e1 + e2) * 0.5;
    [a, a + e1, a + e2]
}

/// **The certified tie is decided by the candidates, not by the order
/// the targets were offered in.**
///
/// Two `near_tangent` triangles on one ray: B, whose interval reaches
/// below its own box entry, and A, a scaled copy nearer the origin
/// whose `t_hi` lands in `[t_lo(B), t_enter(B))`. Neither precedes the
/// other, so they are one certified tie, and B is the narrower claim.
/// The rule says B in all three arrangements — both in one target, B's
/// target first, A's target first — and an early-out that prunes B
/// answers A in two of the three, which is the order-dependence the
/// determinism contract forbids.
///
/// The search over `(k_a, k_b, λ, t_a)` is deterministic and prints
/// what it found. Authored by review lane `pick3-r2`, where it was
/// red.
#[test]
fn the_certified_tie_is_decided_by_the_candidates_and_not_the_targets_order() {
    let mut found = None;
    'search: for k_b in 37..=60u32 {
        let (ray, b) = near_tangent(f64::from(k_b));
        let Some(span_b) = ray_triangle(&ray, &b) else {
            continue;
        };
        let enter_b = entry(&ray, &b);
        if span_b.t_lo >= enter_b {
            continue;
        }
        for k_a in 37..=60u32 {
            for lambda in [1.5, 2.0, 3.0, 4.0] {
                for j in 1..64u32 {
                    let t_a = f64::from(j) / 64.0;
                    let a = near_tangent_copy(f64::from(k_a), lambda, t_a, &ray);
                    let Some(span_a) = ray_triangle(&ray, &a) else {
                        continue;
                    };
                    let enter_a = entry(&ray, &a);
                    if enter_a < enter_b
                        && span_a.t_hi < enter_b
                        && span_b.t_lo <= span_a.t_hi
                        && span_b.width() < span_a.width()
                    {
                        found = Some((ray, a, b, span_a, span_b, enter_b));
                        break 'search;
                    }
                }
            }
        }
    }
    let (ray, a, b, span_a, span_b, enter_b) = found
        .expect("the search finds a wide A whose t_hi lands between B's t_lo and B's box entry");
    println!(
        "# A span {span_a:?} width {}\n# B span {span_b:?} width {} entry {enter_b}",
        span_a.width(),
        span_b.width()
    );
    assert!(
        span_a.t_hi < enter_b,
        "A's upper end is below B's box entry: the un-margined break fires"
    );
    assert!(
        span_b.t_lo <= span_a.t_hi && !span_a.precedes(&span_b) && !span_b.precedes(&span_a),
        "neither precedes the other: they are one certified tie"
    );
    assert!(
        span_b.width() < span_a.width(),
        "and B is the narrower claim"
    );

    let door = Door::new("pick3_early_out_r2");
    let both = [a, b];
    let one_target = door.ask(&[&both], &ray).t;
    let b_then_a = door.ask(&[&[b], &[a]], &ray).t;
    let a_then_b = door.ask(&[&[a], &[b]], &ray).t;
    println!("# one target {one_target} ; [B, A] {b_then_a} ; [A, B] {a_then_b}");
    assert_eq!(
        b_then_a, span_b.t,
        "with B tested first, both survive and the narrower claim B wins"
    );
    assert_eq!(
        a_then_b, b_then_a,
        "and the answer does not depend on which target was offered first"
    );
    assert_eq!(
        one_target, span_b.t,
        "nor on whether the two triangles share a target: the early-out prunes only \
         candidates the set rule already drops"
    );
}

// ---------------------------------------------------------------
// The bound the early-out compares, and the last tie-break key.
// ---------------------------------------------------------------

/// **The early-out's bound is the interval's UPPER end.** Two
/// triangles on one ray: [`near_tangent`] at `k = 64`, admitted with
/// `u = v = 0.5` exactly and an interval half a unit wide around
/// `t = 1.5`; and a small transversal triangle crossing the same ray
/// at `t = 1.7`, whose box the ray does not enter until after the
/// first candidate's ROUNDED answer but before its upper end.
///
/// The door reaches the second candidate — the geometry has not yet
/// said which is in front — and the certified tie falls to the narrow
/// one at `1.7`. A door whose bound were the rounded `t` would stop
/// first and answer `1.5`.
#[test]
fn the_early_outs_bound_is_the_intervals_upper_end() {
    let (ray, wide) = near_tangent(64.0);
    // Perpendicular to the ray's `x` advance, around the point the ray
    // reaches at t = 1.7, and comfortably inside the triangle.
    let narrow = [
        Point3::new(0.7, 0.5, -0.2),
        Point3::new(0.7, 0.9, -0.2),
        Point3::new(0.7, 0.7, 0.3),
    ];
    let tris = [wide, narrow];
    let spans = [
        span_of(&ray, &tris[0], "the near-tangent candidate"),
        span_of(&ray, &tris[1], "the transversal candidate"),
    ];
    assert_eq!(spans[0].t, 1.5, "the near-tangent candidate answers 1.5");
    assert!(
        (spans[1].t - 1.7).abs() < 1e-12,
        "the transversal candidate answers 1.7: {:?}",
        spans[1]
    );
    assert!(
        spans[1].width() < 1e-12 && spans[0].width() > 0.5,
        "one claim is certified to the bit and the other to half a unit: {spans:?}"
    );
    let entry_narrow = entry(&ray, &tris[1]);
    assert!(
        spans[0].t < entry_narrow && entry_narrow <= spans[0].t_hi,
        "the row's premise: the second box is entered after the first candidate's rounded t \
         ({}) and before its upper end ({}) — entry {entry_narrow}",
        spans[0].t,
        spans[0].t_hi
    );
    let door = Door::new("pick3_early_out_upper_end");
    assert_eq!(
        door.ask(&[&tris], &ray).t,
        spans[1].t,
        "the door reaches the transversal candidate and the tie falls to it"
    );
    assert_eq!(
        door.ask(&[&[tris[1]], &[tris[0]]], &ray).t,
        spans[1].t,
        "and again with the candidates offered the other way round"
    );
}

/// **At equal width, the target's position decides** — the tie-break's
/// last key, pinned through the door.
///
/// The SAME triangle at the same magnitudes, offered as two targets
/// under two DIFFERENT faces of the cube. Every number either
/// candidate computes is the same number, so neither precedes the
/// other and the widths are equal to the bit: nothing but position is
/// left. The door answers the earlier target's face, and swapping the
/// targets swaps the answer — while both place the hit at the same
/// point, which is what makes the tie a tie. The door is choosing
/// which face to NAME, not where the ray met the mesh.
///
/// Reversing `target_pos` in `pick_face`'s order reds this row.
#[test]
fn equal_widths_fall_to_the_earlier_target() {
    let tri = [
        Point3::new(1.0, 1.0, 1.0),
        Point3::new(5.0, 1.0, 1.0),
        Point3::new(1.0, 5.0, 1.0),
    ];
    let ray = Ray {
        origin: Point3::new(2.0, 2.0, 3.0),
        dir: Vec3::new(0.0, 0.0, -1.0),
    };
    let span = span_of(&ray, &tri, "the interior hit");
    let door = Door::new("pick3_equal_widths");
    let one = door.ask_from(&[&[tri]], &[0], &ray);
    let two = door.ask_from(&[&[tri]], &[1], &ray);
    assert_ne!(
        one.name, two.name,
        "the fixture: the two targets carry the triangle under different faces"
    );
    let first = door.ask_from(&[&[tri], &[tri]], &[0, 1], &ray);
    let second = door.ask_from(&[&[tri], &[tri]], &[1, 0], &ray);
    assert_eq!(
        first.name, one.name,
        "the earlier target wins the equal-width tie"
    );
    assert_eq!(
        second.name, two.name,
        "and it is position that decided, not the face: swapping the targets swaps the answer"
    );
    assert_eq!(
        (first.t, second.t),
        (span.t, span.t),
        "both answers are the same hit"
    );
    assert_eq!(
        [first.point.x, first.point.y, first.point.z],
        [second.point.x, second.point.y, second.point.z],
        "at the same point: the door picks a name, not a place"
    );
    assert_eq!(
        first.t_hi - first.t_lo,
        second.t_hi - second.t_lo,
        "on intervals of equal width, which is why position had to decide"
    );
}
