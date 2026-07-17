//! Structural isomorphism oracle (test support, M1 PR 4): compare two
//! bodies **up to key relabeling and representation-internal choices**.
//!
//! [`canonical_form`] emits a deterministic textual form of a body;
//! [`isomorphic`] compares two forms. The roundtrip property tests use
//! it to assert that an operator followed by its exact inverse restores
//! the body, and M1 PR 5's validator fuzzing can reuse it.
//!
//! # What "isomorphic" means here
//!
//! Two bodies compare equal iff there is a correspondence between their
//! entities preserving: the whole oriented half-edge structure
//! (`next`/mate, hence loops, vertex orbits, and orientation), the loop
//! partition into faces with the outer/ring designation, empty loops and
//! their lone vertices, the shell/solid spine, and **per-vertex point
//! coordinates** (compared bitwise through their `Debug` form).
//!
//! # How the form is canonical
//!
//! Solids are visited in arena order, shells in solid-list order. Each
//! shell's half-edges ("darts") are encoded by a breadth-first traversal
//! that labels darts in first-visit order using only the structure maps
//! (`next`, then mate); the encoding is computed **from every dart of
//! the shell as the starting root, and the lexicographically minimal
//! encoding wins**. Any structure-preserving relabeling of one body
//! produces the same set of candidate encodings, so the minimum is a
//! true invariant — no dependence on key values, on `Cycle::first`
//! anchors, on `Vertex::emanating`, or on face/ring list order (rings
//! and dartless faces are emitted in sorted order; ties can only occur
//! between identically-emitted, i.e. interchangeable, entities). Cost is
//! O(darts²) per shell — fine at test sizes.
//!
//! # Honest limits
//!
//! - **Solid/shell order is positional**: solids compare in arena order,
//!   shells in list order. For bodies built by one history (or a history
//!   plus balanced roundtrips — slot recycling keeps arena positions)
//!   this is exact; for bodies built by *unrelated* histories whose
//!   solids happen to sit in different arena order, a false negative is
//!   possible. Per-shell the encoding is genuinely canonical.
//! - **Geometry payloads other than vertex points are ignored** —
//!   curve/surface payloads AND their sharing patterns. At M1 they are
//!   `Placeholder` ballast whose anchors are construction-history
//!   artifacts (`mef` shares the parent's surface, `mfkrh` must mint a
//!   fresh one), so including them would make legitimate
//!   `kfmrh ∘ mfkrh` roundtrips compare unequal. Real geometry
//!   comparison is an M2+ tolerance question (D4), never bit equality.
//! - **Edge intrinsic direction (`he_plus` vs `he_minus`) is ignored**:
//!   it is a stored representation bit that kill∘make roundtrips
//!   legitimately flip (`mev` always re-mints old → new), and at M1 no
//!   geometry hangs off it yet. The oriented manifold structure itself
//!   IS compared (via `next`/mate).
//! - **Provenance is ignored**: it records history, not structure, and
//!   roundtrips legitimately rewrite it (a re-made entity is a new
//!   birth).
//! - **Point coordinates are compared bitwise** (via `Debug`): bodies
//!   differing by a rigid motion — or by `-0.0` vs `0.0` — are NOT
//!   isomorphic to this oracle. Geometric equivalence is out of scope.
//! - **Tier-1-valid input is assumed**; the traversal panics (test
//!   failure) on bodies whose references do not resolve. If a shell's
//!   dart graph is disconnected (unreachable through the operators),
//!   the encoding falls back to restarting from the first unvisited
//!   dart in scan order, which is deterministic but not
//!   relabeling-invariant — documented heuristic, never hit by
//!   operator-built bodies.

// Test-support code: panicking is a test's failure mechanism (L5).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::fmt::Write as _;
use std::collections::VecDeque;

use slotmap::SecondaryMap;

use crate::body::Body;
use crate::entity::{FaceKey, HalfEdgeKey, LoopBoundary, LoopKey, ShellKey, VertexKey};

/// `true` iff the two bodies are structurally isomorphic (see the
/// [module docs](self) for the exact relation and its limits).
pub(crate) fn isomorphic(a: &Body<f64>, b: &Body<f64>) -> bool {
    canonical_form(a) == canonical_form(b)
}

/// The canonical textual form of a body (module docs). Equal strings ⇔
/// isomorphic bodies, within the documented scope.
pub(crate) fn canonical_form(body: &Body<f64>) -> String {
    let mut out = String::new();
    // Count header: topology arenas + points. Curves/surfaces are
    // deliberately absent (module docs: geometry payloads and sharing
    // are not compared).
    let _ = writeln!(
        out,
        "solids={} shells={} faces={} loops={} halves={} edges={} \
         vertices={} points={}",
        body.solids().count(),
        body.shells().count(),
        body.faces().count(),
        body.loops().count(),
        body.half_edges().count(),
        body.edges().count(),
        body.vertices().count(),
        body.points().count(),
    );
    for (solid_index, (_, solid)) in body.solids().enumerate() {
        let _ = writeln!(out, "solid {solid_index}");
        for (shell_index, &shell) in solid.shells.iter().enumerate() {
            let _ = writeln!(out, " shell {shell_index}");
            out.push_str(&canonical_shell(body, shell));
        }
    }
    out
}

/// The lexicographically minimal encoding of one shell over all root
/// darts (or the dartless encoding for shells with no half-edges).
fn canonical_shell(body: &Body<f64>, shell: ShellKey) -> String {
    let darts = shell_darts(body, shell);
    if darts.is_empty() {
        return encode_shell(body, shell, &darts, None);
    }
    darts
        .iter()
        .map(|&root| encode_shell(body, shell, &darts, Some(root)))
        .min()
        .expect("darts is non-empty")
}

/// Every half-edge of the shell, in a deterministic (but key-dependent)
/// scan order — used as the candidate-root list and the BFS-restart
/// fallback order.
fn shell_darts(body: &Body<f64>, shell: ShellKey) -> Vec<HalfEdgeKey> {
    let shell_data = body.get_shell(shell).expect("shell resolves");
    let mut darts = Vec::new();
    for &face in &shell_data.faces {
        let face_data = body.get_face(face).expect("face resolves");
        for loop_key in core::iter::once(face_data.outer).chain(face_data.rings.iter().copied()) {
            let loop_data = body.get_loop(loop_key).expect("loop resolves");
            if let LoopBoundary::Cycle { first } = loop_data.boundary {
                darts.extend(body.loop_cycle(first).expect("cycle closes"));
            }
        }
    }
    darts
}

/// One shell encoding from one root (or dartless). See the module docs
/// for the labeling scheme.
fn encode_shell(
    body: &Body<f64>,
    shell: ShellKey,
    darts: &[HalfEdgeKey],
    root: Option<HalfEdgeKey>,
) -> String {
    // --- Dart labels: BFS in first-visit order, successors next, mate.
    let mut label: SecondaryMap<HalfEdgeKey, usize> = SecondaryMap::new();
    let mut order: Vec<HalfEdgeKey> = Vec::new();
    let visit_from = |start: HalfEdgeKey,
                      label: &mut SecondaryMap<HalfEdgeKey, usize>,
                      order: &mut Vec<HalfEdgeKey>| {
        let mut queue = VecDeque::from([start]);
        label.insert(start, order.len());
        order.push(start);
        while let Some(dart) = queue.pop_front() {
            let dart_data = body.get_half_edge(dart).expect("dart resolves");
            let mate = body.mate(dart).expect("mate resolves");
            for successor in [dart_data.next, mate] {
                if !label.contains_key(successor) {
                    label.insert(successor, order.len());
                    order.push(successor);
                    queue.push_back(successor);
                }
            }
        }
    };
    if let Some(root) = root {
        visit_from(root, &mut label, &mut order);
        // Disconnected-dart fallback (module docs): deterministic, not
        // relabeling-invariant; unreachable through operator-built
        // bodies.
        while order.len() < darts.len() {
            let next_unvisited = darts
                .iter()
                .copied()
                .find(|d| !label.contains_key(*d))
                .expect("count mismatch implies an unvisited dart");
            visit_from(next_unvisited, &mut label, &mut order);
        }
    }

    // --- Vertex labels by first visit over dart order.
    let mut vertex_label: SecondaryMap<VertexKey, usize> = SecondaryMap::new();
    let mut vertex_order: Vec<VertexKey> = Vec::new();
    for &dart in &order {
        let start = body.get_half_edge(dart).expect("dart resolves").start;
        if !vertex_label.contains_key(start) {
            vertex_label.insert(start, vertex_order.len());
            vertex_order.push(start);
        }
    }

    // --- Loop first-visit order and per-loop minimal dart label.
    let mut loop_min: SecondaryMap<LoopKey, usize> = SecondaryMap::new();
    let mut loop_order: Vec<LoopKey> = Vec::new();
    for (index, &dart) in order.iter().enumerate() {
        let parent = body.get_half_edge(dart).expect("dart resolves").parent_loop;
        if !loop_min.contains_key(parent) {
            loop_min.insert(parent, index);
            loop_order.push(parent);
        }
    }

    // --- Face order: first visit through the loop order.
    let mut face_seen: SecondaryMap<FaceKey, ()> = SecondaryMap::new();
    let mut face_order: Vec<FaceKey> = Vec::new();
    for &loop_key in &loop_order {
        let face = body.get_loop(loop_key).expect("loop resolves").face;
        if !face_seen.contains_key(face) {
            face_seen.insert(face, ());
            face_order.push(face);
        }
    }

    // --- Emission.
    let mut out = String::new();
    for (index, &dart) in order.iter().enumerate() {
        let dart_data = body.get_half_edge(dart).expect("dart resolves");
        let mate = body.mate(dart).expect("mate resolves");
        let _ = writeln!(
            out,
            "  d{index}: n{} m{} v{}",
            label[dart_data.next], label[mate], vertex_label[dart_data.start],
        );
    }
    for &face in &face_order {
        let _ = writeln!(out, "  face: {}", face_desc(body, face, &loop_min));
    }
    // Dartless faces (empty outer, no cycle rings — the mvfs face and
    // mfkrh's empty-ring promotions): sorted by emission; ties emit
    // identically and are interchangeable.
    let shell_data = body.get_shell(shell).expect("shell resolves");
    let mut dartless: Vec<String> = shell_data
        .faces
        .iter()
        .filter(|face| !face_seen.contains_key(**face))
        .map(|&face| face_desc(body, face, &loop_min))
        .collect();
    dartless.sort();
    for desc in dartless {
        let _ = writeln!(out, "  face*: {desc}");
    }
    for (index, &vertex) in vertex_order.iter().enumerate() {
        let _ = writeln!(out, "  v{index}: {}", vertex_coords(body, vertex));
    }
    out
}

/// A face's emission: outer-loop token plus sorted ring tokens.
fn face_desc(body: &Body<f64>, face: FaceKey, loop_min: &SecondaryMap<LoopKey, usize>) -> String {
    let face_data = body.get_face(face).expect("face resolves");
    let outer = loop_token(body, face_data.outer, loop_min);
    let mut rings: Vec<String> = face_data
        .rings
        .iter()
        .map(|&ring| loop_token(body, ring, loop_min))
        .collect();
    rings.sort();
    format!("outer={outer} rings=[{}]", rings.join(","))
}

/// A loop's token: the minimal dart label of its cycle, or the lone
/// vertex's coordinates for an empty loop.
fn loop_token(
    body: &Body<f64>,
    loop_key: LoopKey,
    loop_min: &SecondaryMap<LoopKey, usize>,
) -> String {
    match body.get_loop(loop_key).expect("loop resolves").boundary {
        LoopBoundary::Empty { vertex } => format!("empty({})", vertex_coords(body, vertex)),
        LoopBoundary::Cycle { .. } => format!(
            "cycle@{}",
            loop_min
                .get(loop_key)
                .expect("cycle loop of this shell was labeled"),
        ),
    }
}

/// A vertex's point coordinates, bitwise via `Debug` (module docs).
fn vertex_coords(body: &Body<f64>, vertex: VertexKey) -> String {
    let point = body.get_vertex(vertex).expect("vertex resolves").point;
    format!("{:?}", body.get_point(point).expect("point resolves"))
}

#[cfg(test)]
mod tests {
    use geom_core::Point3;

    use super::*;
    use crate::euler::{MefSite, MevSite};
    use crate::fixtures::{ops_cube, ops_holed_box};

    fn pt(x: f64, y: f64, z: f64) -> Point3<f64> {
        Point3::new(x, y, z)
    }

    /// The digon pillow built the canonical way: mvfs, mev(Lone),
    /// mef(Chords) — two vertices at x = 0 and x = 1.
    fn pillow_via_segment() -> Body<f64> {
        let mut body = Body::<f64>::new();
        let seed = body.mvfs(pt(0.0, 0.0, 0.0)).unwrap();
        let seg = body
            .mev(
                MevSite::Lone {
                    r#loop: seed.r#loop,
                },
                pt(1.0, 0.0, 0.0),
            )
            .unwrap();
        body.mef(MefSite::Chords {
            he1: seg.he_plus,
            he2: seg.he_minus,
        })
        .unwrap();
        body
    }

    /// The same digon pillow built through the OTHER degenerate route:
    /// mvfs, mef(Lone) — the circular self-loop edge — then a fan mev
    /// splitting the self-loop vertex into two.
    fn pillow_via_circle() -> Body<f64> {
        let mut body = Body::<f64>::new();
        let seed = body.mvfs(pt(0.0, 0.0, 0.0)).unwrap();
        let circle = body
            .mef(MefSite::Lone {
                r#loop: seed.r#loop,
            })
            .unwrap();
        body.mev(
            MevSite::Fan {
                he1: circle.he_plus,
                he2: circle.he_minus,
            },
            pt(1.0, 0.0, 0.0),
        )
        .unwrap();
        body
    }

    #[test]
    fn identical_builds_have_identical_forms() {
        let a = ops_cube();
        let b = ops_cube();
        assert_eq!(canonical_form(&a.body), canonical_form(&b.body));
        assert!(isomorphic(&a.body, &b.body));
    }

    #[test]
    fn pillow_is_isomorphic_across_different_op_orders() {
        // Same structure (v2 e2 f2 digon pillow, same coordinates)
        // reached through two entirely different operator sequences —
        // different key histories, different loop anchors.
        let a = pillow_via_segment();
        let b = pillow_via_circle();
        assert_eq!(crate::validate::validate(&a), Ok(()));
        assert_eq!(crate::validate::validate(&b), Ok(()));
        assert!(isomorphic(&a, &b));
    }

    #[test]
    fn pillows_at_different_coordinates_differ() {
        // Bitwise coordinate comparison is part of the relation.
        let a = pillow_via_segment();
        let mut body = Body::<f64>::new();
        let seed = body.mvfs(pt(0.0, 0.0, 0.0)).unwrap();
        let seg = body
            .mev(
                MevSite::Lone {
                    r#loop: seed.r#loop,
                },
                pt(2.0, 0.0, 0.0), // elsewhere
            )
            .unwrap();
        body.mef(MefSite::Chords {
            he1: seg.he_plus,
            he2: seg.he_minus,
        })
        .unwrap();
        assert!(!isomorphic(&a, &body));
    }

    #[test]
    fn form_is_invariant_under_cycle_first_rotation() {
        // Cycle::first is a representation-internal anchor; rotating it
        // must not change the canonical form (the kill ops re-anchor
        // loops unconditionally, so roundtrips depend on this).
        let t = ops_cube();
        let before = canonical_form(&t.body);
        let mut rotated = t.body.clone();
        let loops: Vec<_> = rotated.loops().map(|(k, _)| k).collect();
        for loop_key in loops {
            let LoopBoundary::Cycle { first } = rotated.get_loop(loop_key).unwrap().boundary else {
                continue;
            };
            let next = rotated.get_half_edge(first).unwrap().next;
            rotated.get_loop_mut(loop_key).unwrap().boundary = LoopBoundary::Cycle { first: next };
        }
        assert_eq!(crate::validate::validate(&rotated), Ok(()));
        assert_eq!(canonical_form(&rotated), before);
    }

    #[test]
    fn form_is_invariant_under_emanating_choice() {
        // Vertex::emanating names an arbitrary orbit member; re-anchoring
        // it must not change the form.
        let t = ops_cube();
        let before = canonical_form(&t.body);
        let mut reanchored = t.body.clone();
        let vertices: Vec<_> = reanchored.vertices().map(|(k, _)| k).collect();
        for vertex in vertices {
            let Some(emanating) = reanchored.get_vertex(vertex).unwrap().emanating else {
                continue;
            };
            let orbit = reanchored.vertex_orbit(emanating).unwrap();
            reanchored.get_vertex_mut(vertex).unwrap().emanating = Some(orbit[1 % orbit.len()]);
        }
        assert_eq!(crate::validate::validate(&reanchored), Ok(()));
        assert_eq!(canonical_form(&reanchored), before);
    }

    #[test]
    fn cube_is_not_the_holed_box() {
        let cube = ops_cube();
        let holed = ops_holed_box();
        assert!(!isomorphic(&cube.body, &holed.body));
    }

    #[test]
    fn ring_distribution_near_miss_is_distinguished() {
        // Two bodies with IDENTICAL arena counts (same v, e, f, loops,
        // rings) differing only in which face carries which empty ring:
        // both rings on face A versus one ring on each face. The count
        // header cannot tell them apart; the structural emission must.
        let build = |split: bool| {
            let mut body = Body::<f64>::new();
            let seed = body.mvfs(pt(0.0, 0.0, 0.0)).unwrap();
            let seg = body
                .mev(
                    MevSite::Lone {
                        r#loop: seed.r#loop,
                    },
                    pt(1.0, 0.0, 0.0),
                )
                .unwrap();
            let split_faces = body
                .mef(MefSite::Chords {
                    he1: seg.he_plus,
                    he2: seg.he_minus,
                })
                .unwrap();
            // Two hole anchors planted from face A's side (seg.he_plus
            // lives in the new face after mef; its mate in the old).
            let plant = |body: &mut Body<f64>, at, x| {
                let strut = body
                    .mev(MevSite::Fan { he1: at, he2: at }, pt(x, 0.0, 0.0))
                    .unwrap();
                body.kemr(strut.he_plus, strut.he_minus).unwrap()
            };
            let r1 = plant(&mut body, seg.he_plus, 2.0);
            let _r2 = plant(&mut body, seg.he_plus, 3.0);
            if split {
                // Move ONE ring to the other face. ring_move is not an
                // Euler op; counts are unchanged.
                let other_face = body.get_half_edge(seg.he_minus).unwrap().parent_loop;
                let other_face = body.get_loop(other_face).unwrap().face;
                assert_ne!(other_face, split_faces.face);
                body.ring_move(r1.ring, other_face).unwrap();
            }
            assert_eq!(crate::validate::validate(&body), Ok(()));
            body
        };
        let together = build(false);
        let split = build(true);
        // Identical counts...
        assert_eq!(together.loops().count(), split.loops().count());
        assert_eq!(together.faces().count(), split.faces().count());
        assert_eq!(together.vertices().count(), split.vertices().count());
        // ...different structure.
        assert!(!isomorphic(&together, &split));
    }

    #[test]
    fn ring_list_order_does_not_matter() {
        // Face::rings is semantically a set; kfmrh/mfkrh roundtrips
        // permute the list (retain + push). Permuting it by hand must
        // not change the form.
        let mut body = Body::<f64>::new();
        let seed = body.mvfs(pt(0.0, 0.0, 0.0)).unwrap();
        let seg = body
            .mev(
                MevSite::Lone {
                    r#loop: seed.r#loop,
                },
                pt(1.0, 0.0, 0.0),
            )
            .unwrap();
        let plant = |body: &mut Body<f64>, x| {
            let strut = body
                .mev(
                    MevSite::Fan {
                        he1: seg.he_minus,
                        he2: seg.he_minus,
                    },
                    pt(x, 0.0, 0.0),
                )
                .unwrap();
            body.kemr(strut.he_plus, strut.he_minus).unwrap()
        };
        plant(&mut body, 2.0);
        plant(&mut body, 3.0);
        let before = canonical_form(&body);
        let face = body.get_loop(seed.r#loop).unwrap().face;
        body.get_face_mut(face).unwrap().rings.reverse();
        assert_eq!(crate::validate::validate(&body), Ok(()));
        assert_eq!(canonical_form(&body), before);
    }
}
