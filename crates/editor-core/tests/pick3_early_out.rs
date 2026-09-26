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
    "crates/editor-core/src/test_support.rs",
    "crates/bvh/src/",
    "crates/editor-core/tests/fixture/",
];

use crate::fixture;

use bvh::test_support::ray;
use bvh::{Aabb, Ray};
use editor_core::resolve::{TSpan, crossing, ray_triangle};
use editor_core::test_support::{down_from, near_tangent};
use editor_core::{
    CancelToken, EvalOptions, Evaluation, HitTestError, MeshPick, Node, PickHit, PickTarget,
    ProfileDoc, RecipeNodeId, ValuePayload, pick_face,
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
    /// order given, where the door ANSWERS. Each target's triangles
    /// are carried under the face keys starting at `first_patch[i]`,
    /// so two targets can hold the SAME triangle under DIFFERENT
    /// faces — and one target's triangles can be spread over several.
    fn ask_from(
        &self,
        groups: &[&[[Point3<f64>; 3]]],
        first_patch: &[usize],
        ray: &Ray,
    ) -> PickHit {
        self.raw(groups, first_patch, ray)
            .expect("no error")
            .expect("a hit")
    }

    /// [`Door::ask_from`] where the door REFUSES: the tied faces'
    /// hits, in the order the refusal lists them.
    fn tied(
        &self,
        groups: &[&[[Point3<f64>; 3]]],
        first_patch: &[usize],
        ray: &Ray,
    ) -> Vec<PickHit> {
        match self.raw(groups, first_patch, ray) {
            Err(HitTestError::Ambiguous { hits }) => hits,
            other => panic!("the certified tie between faces refuses: {other:?}"),
        }
    }

    /// The door's answer, whatever it is.
    fn raw(
        &self,
        groups: &[&[[Point3<f64>; 3]]],
        first_patch: &[usize],
        ray: &Ray,
    ) -> Result<Option<PickHit>, HitTestError> {
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
    }
}

/// The names the refusal lists, in its own order.
fn named(hits: &[PickHit]) -> Vec<String> {
    hits.iter().map(|h| h.name.to_string()).collect()
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
/// lower end, so the narrow one is IN the tie by the door's own rule.
/// The two are different faces, so the door refuses with both, and
/// the kept candidate is what the refusal's second entry IS:
/// `Pruned == Every` over the LIST. A door that pruned it would
/// answer the wide face alone, and answering is not refusing.
///
/// Authored by review lane `pick3-r1`, where it was red; re-expressed
/// through `pick_face` rather than through a hand-run traversal, so a
/// margin that goes missing from the door reds it.
#[test]
fn the_early_out_keeps_a_candidate_whose_interval_reaches_below_its_box() {
    let zeta = 2f64.powi(-20);
    let skew = ray([0.0, 0.0, 0.0], [1.0, 1.0, zeta]);
    let tris = [
        corner_tangent(154.0, 1.0, 4.0),
        corner_tangent(238.0, 1.0, 4.5),
    ];
    let wide = span_of(&skew, &tris[0], "the wide candidate");
    let narrow = span_of(&skew, &tris[1], "the narrow candidate");
    let entry_narrow = entry(&skew, &tris[1]);
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
    assert_eq!(
        TSpan::survivors(&[wide, narrow]),
        vec![0, 1],
        "the certified order over both candidates keeps both"
    );

    let door = Door::new("pick3_early_out_r1");
    let hits = door.tied(&[&tris], &[0], &skew);
    assert_eq!(
        hits.iter().map(|h| h.t).collect::<Vec<_>>(),
        vec![wide.t, narrow.t],
        "and so does the door, with both triangles in one target: the refusal carries both, \
         the kept candidate included"
    );
    assert_eq!(
        named(&hits).len(),
        2,
        "two faces, not two triangles of one: {:?}",
        named(&hits)
    );
}

// ---------------------------------------------------------------
// `pick3-r2`'s fixture: the same gap, and the order of the targets.
// ---------------------------------------------------------------

/// [`near_tangent`]'s shape at `k`, scaled by `lambda` and placed so `ray` meets
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
/// other, so they are one certified tie; carried under two different
/// faces, they are the refusal. The rule says the SAME two faces in
/// all three arrangements — both in one target, B's target first, A's
/// target first — and an early-out that prunes B answers A alone in
/// two of the three, which is the order-dependence the determinism
/// contract forbids.
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
        "and B is the narrower claim, which decides nothing"
    );

    let door = Door::new("pick3_early_out_r2");
    let both = [a, b];
    // A under the cube's first face, B under its second, in all three
    // arrangements: the same two faces are tied every time.
    let one_target = door.tied(&[&both], &[0], &ray);
    let b_then_a = door.tied(&[&[b], &[a]], &[1, 0], &ray);
    let a_then_b = door.tied(&[&[a], &[b]], &[0, 1], &ray);
    let set = |hits: &[PickHit]| {
        let mut names = named(hits);
        names.sort();
        names
    };
    println!(
        "# one target {:?} ; [B, A] {:?} ; [A, B] {:?}",
        named(&one_target),
        named(&b_then_a),
        named(&a_then_b)
    );
    assert_eq!(
        named(&b_then_a),
        named(&one_target).into_iter().rev().collect::<Vec<_>>(),
        "with B's target first the LIST is B then A — target order, and nothing else"
    );
    assert_eq!(
        set(&a_then_b),
        set(&b_then_a),
        "and the refusal's SET does not depend on which target was offered first"
    );
    assert_eq!(
        set(&one_target),
        set(&a_then_b),
        "nor on whether the two triangles share a target: the early-out prunes only \
         candidates the set rule already drops"
    );
    assert_eq!(
        [a_then_b[0].t, a_then_b[1].t],
        [span_a.t, span_b.t],
        "and each tied face carries its own true parameter"
    );
}

// ---------------------------------------------------------------
// The bound the early-out compares.
// ---------------------------------------------------------------

/// **The early-out's bound is the interval's UPPER end.** Two
/// triangles on one ray: [`near_tangent`] at `k = 64`, admitted with
/// `u = v = 0.5` exactly and an interval half a unit wide around
/// `t = 1.5`; and a small transversal triangle crossing the same ray
/// at `t = 1.7`, whose box the ray does not enter until after the
/// first candidate's ROUNDED answer but before its upper end.
///
/// The door reaches the second candidate — the geometry has not yet
/// said which is in front — so the two are one certified tie on two
/// faces and the door refuses with both. A door whose bound were the
/// rounded `t` would stop first and ANSWER the near-tangent face at
/// `1.5`.
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
    let one_target = door.tied(&[&tris], &[0], &ray);
    assert_eq!(
        one_target.iter().map(|h| h.t).collect::<Vec<_>>(),
        vec![spans[0].t, spans[1].t],
        "the door reaches the transversal candidate, and both faces are in the tie"
    );
    let reversed = door.tied(&[&[tris[1]], &[tris[0]]], &[1, 0], &ray);
    assert_eq!(
        reversed.iter().map(|h| h.t).collect::<Vec<_>>(),
        vec![spans[1].t, spans[0].t],
        "and again with the candidates offered the other way round: the same two, listed in \
         the new target order"
    );
}

/// **At equal width, the door refuses** — there is no last key.
///
/// The SAME triangle at the same magnitudes, offered as two targets
/// under two DIFFERENT faces of the cube. Every number either
/// candidate computes is the same number, so neither precedes the
/// other and the widths are equal to the bit: nothing is left to
/// choose with, and choosing anyway would be choosing which face to
/// NAME on a rule the user never asked for. The door names both, at
/// the one point they agree on, and swapping the targets swaps the
/// LIST's order and nothing else.
///
/// A door that kept the position key answers one face here and reds
/// this row.
#[test]
fn equal_widths_refuse_with_both_faces() {
    let tri = [
        Point3::new(1.0, 1.0, 1.0),
        Point3::new(5.0, 1.0, 1.0),
        Point3::new(1.0, 5.0, 1.0),
    ];
    let ray = down_from(2.0, 2.0, 3.0);
    let span = span_of(&ray, &tri, "the interior hit");
    let door = Door::new("pick3_equal_widths");
    let one = door.ask_from(&[&[tri]], &[0], &ray);
    let two = door.ask_from(&[&[tri]], &[1], &ray);
    assert_ne!(
        one.name, two.name,
        "the fixture: the two targets carry the triangle under different faces"
    );
    let first = door.tied(&[&[tri], &[tri]], &[0, 1], &ray);
    let second = door.tied(&[&[tri], &[tri]], &[1, 0], &ray);
    assert_eq!(
        named(&first),
        vec![one.name.to_string(), two.name.to_string()],
        "both faces are refused, listed in target order"
    );
    assert_eq!(
        named(&second),
        vec![two.name.to_string(), one.name.to_string()],
        "and swapping the targets swaps the LIST and nothing else"
    );
    assert_eq!(
        first.iter().map(|h| h.t).collect::<Vec<_>>(),
        vec![span.t, span.t],
        "both entries are the same hit"
    );
    for hit in first.iter().chain(&second) {
        assert_eq!(
            [hit.point.x, hit.point.y, hit.point.z],
            [first[0].point.x, first[0].point.y, first[0].point.z],
            "at the same point: the refusal names faces, not places"
        );
        assert_eq!(
            hit.t_hi - hit.t_lo,
            span.t_hi - span.t_lo,
            "on intervals of equal width, which is why nothing could have decided"
        );
    }
}

// ---------------------------------------------------------------
// The certified tie between faces, through the door.
// ---------------------------------------------------------------

/// **A ray down a shared edge refuses with BOTH faces.**
///
/// Two triangles mirrored across the segment from `(0, 0, 0)` to
/// `(1, 0, 0)`, carried under two different faces of the cube, and a
/// `−z` ray through the segment's midpoint. Both triangles compute
/// `u = 0.5`, `v = 0`, `t = 2` without a rounding, so neither
/// precedes the other and both hits are TRUE: each places the hit at
/// the midpoint, to the bit. The geometry names no face, so the door
/// names none either — it refuses with both, each carrying its own
/// hit, listed in the caller's target order.
///
/// **The refusal is a function of the set.** Offering the targets the
/// other way round reverses the list and changes nothing else: the
/// same two faces, the same two parameters, the same point.
///
/// The arithmetic underneath is `pick.rs`'s
/// `a_ray_down_a_shared_edge_ties_two_narrow_intervals_and_nothing_breaks_it`;
/// this row is what the door does with it.
#[test]
fn a_ray_down_a_shared_edge_refuses_with_both_faces() {
    let shared = [Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 0.0, 0.0)];
    let above = [shared[0], shared[1], Point3::new(0.5, 1.0, 0.0)];
    let below = [shared[0], shared[1], Point3::new(0.5, -1.0, 0.0)];
    let midpoint = Point3::new(0.5, 0.0, 0.0);
    let ray = down_from(midpoint.x, midpoint.y, 2.0);
    let spans = [
        span_of(&ray, &above, "the triangle above the edge"),
        span_of(&ray, &below, "the triangle below it"),
    ];
    assert!(
        !spans[0].precedes(&spans[1]) && !spans[1].precedes(&spans[0]),
        "the row's premise: neither interval lies below the other, so they are one \
         certified tie ({spans:?})"
    );

    let door = Door::new("pick3_shared_edge_refuses");
    // Each triangle under its own face of the cube: the tie is
    // BETWEEN faces, which is what refuses.
    let hits = door.tied(&[&[above], &[below]], &[0, 1], &ray);
    assert_eq!(hits.len(), 2, "one hit per tied face: {:?}", named(&hits));
    for (i, hit) in hits.iter().enumerate() {
        assert_eq!(hit.t, 2.0, "tied face {i} answers the midpoint at t = 2");
        assert_eq!(
            [hit.point.x, hit.point.y, hit.point.z].map(f64::to_bits),
            [midpoint.x, midpoint.y, midpoint.z].map(f64::to_bits),
            "and places it at the shared edge's midpoint: every tied hit is TRUE"
        );
    }
    assert_ne!(
        hits[0].name, hits[1].name,
        "the two entries are two faces, not one face met twice"
    );

    let swapped = door.tied(&[&[below], &[above]], &[1, 0], &ray);
    assert_eq!(
        named(&swapped),
        named(&hits).into_iter().rev().collect::<Vec<_>>(),
        "offering the targets the other way round reverses the LIST"
    );
    let mut one = named(&hits);
    let mut other = named(&swapped);
    one.sort();
    other.sort();
    assert_eq!(one, other, "and refuses with the same SET of faces");
}

/// **Several triangles of ONE face are one answer, not a tie.**
///
/// A face split along a diagonal, both halves carried under the SAME
/// face of the cube, and a ray down that diagonal: both triangles are
/// hit, neither precedes the other, and the survivors name one face —
/// so the door answers that face, with the HULL of the two intervals
/// and the smaller of the two rounded `t`s.
///
/// This is the case a refusal must not reach: it is every ray across
/// a triangle diagonal or an in-face shared edge, which is most
/// picks. A door that refused on the number of surviving TRIANGLES
/// rather than on the number of faces reds here.
///
/// **The two halves do not meet the ray at the same depth, on
/// purpose.** A face's triangles are chorded one by one, so two of
/// them meet along a seam only to within a rounding — and the second
/// half here sits a few ulps along the ray, far enough that the two
/// members answer DIFFERENT rounded parameters and far inside either
/// interval, so neither is certified in front of the other. Both
/// halves in one exact plane, struck at a dyadic midpoint, answer one
/// `t` between them and a door reporting the LARGER member, or the
/// hull's midpoint, passes the row. The far half is offered FIRST, so
/// a door answering its first member rather than its smallest reds
/// too. All of it is asserted rather than assumed.
#[test]
fn several_triangles_of_one_face_answer_that_face() {
    // The unit square in z = 0, split along the diagonal from
    // (0, 0) to (1, 1); the ray runs down that diagonal's midpoint.
    // The second half is chorded a few ulps further along the ray,
    // which is the seam a tessellation actually leaves.
    let sag = 2e-15;
    let corners = [
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(1.0, 0.0, 0.0),
        Point3::new(1.0, 1.0, 0.0),
        Point3::new(0.0, 1.0, 0.0),
    ];
    let dropped = |p: Point3<f64>| Point3::new(p.x, p.y, p.z - sag);
    // The FIRST half offered is the far one, so "the smallest rounded
    // `t`" and "the first member" are different answers here.
    let halves = [
        [
            dropped(corners[0]),
            dropped(corners[1]),
            dropped(corners[2]),
        ],
        [corners[0], corners[2], corners[3]],
    ];
    let midpoint = Point3::new(0.5, 0.5, 0.0);
    let ray = down_from(midpoint.x, midpoint.y, 3.0);
    let spans = [
        span_of(&ray, &halves[0], "the first half"),
        span_of(&ray, &halves[1], "the second half"),
    ];
    assert!(
        !spans[0].precedes(&spans[1]) && !spans[1].precedes(&spans[0]),
        "the row's premise: the diagonal is a hit for both halves and neither is in \
         front ({spans:?})"
    );
    assert_ne!(
        spans[0].t, spans[1].t,
        "the row's premise: the two members answer DIFFERENT rounded parameters, so \
         the smallest-`t` rule has something to decide ({spans:?})"
    );

    let door = Door::new("pick3_one_face_diagonal");
    // ONE face: both halves are offered as two targets carrying the
    // cube's first patch, so the two candidates share a face key.
    let hit = door.ask_from(&[&[halves[0]], &[halves[1]]], &[0, 0], &ray);
    let one = door.ask_from(&[&[halves[0]]], &[0], &ray);
    assert_eq!(
        hit.name, one.name,
        "the door answers that face rather than refusing"
    );
    assert_eq!(
        hit.t,
        spans[0].t.min(spans[1].t),
        "with the smaller of the members' rounded parameters"
    );
    assert_eq!(
        (hit.t_lo, hit.t_hi),
        (
            spans[0].t_lo.min(spans[1].t_lo),
            spans[0].t_hi.max(spans[1].t_hi)
        ),
        "and the HULL of their intervals, which encloses every crossing the tie holds"
    );
}

/// **One face's members answer the SMALLEST rounded `t`**, at any
/// separation.
///
/// `several_triangles_of_one_face_answer_that_face` puts its two
/// members a rounding apart, which is where the rule is REACHED — a
/// diagonal, an in-face shared edge. This row puts two members of one
/// face at `t = 1.5` and `t = 1.7` (the `the_early_outs_bound_…`
/// pair, carried under one face instead of two): their intervals
/// overlap, so both survive, and the parameters they answer are far
/// enough apart that no rounding could confuse the two.
///
/// Reds a door that answers the larger member, or the hull's midpoint.
#[test]
fn one_faces_members_answer_the_smallest_rounded_t() {
    let (ray, wide) = near_tangent(64.0);
    let narrow = [
        Point3::new(0.7, 0.5, -0.2),
        Point3::new(0.7, 0.9, -0.2),
        Point3::new(0.7, 0.7, 0.3),
    ];
    let spans = [
        span_of(&ray, &wide, "the near-tangent member"),
        span_of(&ray, &narrow, "the transversal member"),
    ];
    assert!(
        !spans[0].precedes(&spans[1]) && !spans[1].precedes(&spans[0]),
        "the row's premise: both members survive ({spans:?})"
    );
    assert!(
        spans[0].t < spans[1].t,
        "and they answer DIFFERENT parameters: {spans:?}"
    );
    let door = Door::new("pick3_one_face_two_parameters");
    // Both members under the cube's FIRST face: one face, one answer.
    let hit = door.ask_from(&[&[wide], &[narrow]], &[0, 0], &ray);
    assert_eq!(
        hit.t, spans[0].t,
        "the smallest rounded t of the face's members, not the largest"
    );
    assert_eq!(
        (hit.t_lo, hit.t_hi),
        (
            spans[0].t_lo.min(spans[1].t_lo),
            spans[0].t_hi.max(spans[1].t_hi)
        ),
        "on the hull of both members' intervals"
    );
    assert_ne!(
        hit.t, spans[1].t,
        "a door answering the LARGEST member's parameter reds here"
    );
}
