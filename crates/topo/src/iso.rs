//! Structural isomorphism oracle (test support, M1 PR 4): compare two
//! bodies **up to key relabeling and representation-internal choices**.
//!
//! [`canonical_form`] emits a deterministic textual form of a body;
//! [`isomorphic`] compares two forms. The roundtrip property tests use
//! it to assert that an operator followed by its exact inverse restores
//! the body; it remains available to any in-crate fuzzing that needs a
//! structural oracle.
//!
//! # What "isomorphic" means here
//!
//! Two bodies compare equal iff there is a correspondence between their
//! entities preserving: the whole oriented half-edge structure
//! (`next`/mate, hence loops, vertex orbits, and orientation), the loop
//! partition into faces with the outer/ring designation, **each face's
//! orientation sense** (S10 — the other half of orientation, and the
//! half no traversal can reconstruct), empty loops and their lone
//! vertices, the shell/solid spine, and **per-vertex point
//! coordinates** (compared bitwise through their `Debug` form).
//!
//! # How the form is canonical
//!
//! Solids are visited in arena order, shells in solid-list order. A
//! shell's half-edges ("darts") fall into one or more connected
//! components under `{next, mate}` — more than one whenever a face
//! carries a detached ring (`kemr`'s ordinary output), so components
//! are a first-class case, not a corruption. Each component is encoded
//! by a breadth-first traversal labeling darts in first-visit order
//! using only the structure maps (`next`, then mate), emitting per-dart
//! `next`/mate/vertex labels plus a label-free **attachment token**
//! (loop role and face fingerprint — how the dart's loop sits in the
//! face structure), followed by the coordinates of the vertices the
//! component introduces; the encoding is computed **from every dart of
//! the component as the starting root, and the lexicographically
//! minimal encoding wins**. Components are committed in ascending
//! encoding order (greedy, resolved before face emission). Any
//! structure-preserving relabeling of one body produces the same
//! candidate sets, so the result is a true invariant — no dependence on
//! key values, on `Cycle::first` anchors, on `Vertex::emanating`, or on
//! face/ring list order (rings and dartless faces are emitted in sorted
//! order). Ties between candidates require automorphic structure AND
//! identical coordinates AND identical attachment fingerprints — the
//! residual blind spot noted below. Cost is O(darts²) per shell plus
//! the fingerprints — fine at test sizes.
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
//!   curve/surface payloads AND their sharing patterns. Which surface
//!   key a face carries is a construction-history artifact (`mef`
//!   shares the parent's; `mfkrh` takes whichever `FaceSurface` arm the
//!   caller names, and cannot restore the key `kfmrh` reaped), so
//!   including them would make legitimate `kfmrh ∘ mfkrh` roundtrips
//!   compare unequal. Real geometry comparison is a tolerance question
//!   (D4), never bit equality.
//! - **Edge intrinsic direction (`he_plus` vs `he_minus`) is ignored**:
//!   it is a stored representation bit that kill∘make roundtrips
//!   legitimately flip (`mev` always re-mints old → new). Geometry
//!   *does* hang off it — an edge's carrier runs forward from
//!   `start(he_plus)` to `end(he_plus)` (`crate::entity`) — but the
//!   carrier is itself ignored by the bullet above, so a flipped bit
//!   and the re-minted curve that follows it are invisible together.
//!   The oriented manifold structure itself IS compared (via
//!   `next`/mate).
//! - **Provenance is ignored**: it records history, not structure, and
//!   roundtrips legitimately rewrite it (a re-made entity is a new
//!   birth).
//! - **Point coordinates are compared bitwise** (via `Debug`): bodies
//!   differing by a rigid motion — or by `-0.0` vs `0.0` — are NOT
//!   isomorphic to this oracle. Geometric equivalence is out of scope.
//! - **Coordinate-identical automorphic twins**: candidates that tie —
//!   identical dart structure, identical coordinates, identical
//!   attachment fingerprints — are broken by scan order. Issue #60
//!   showed the generator DOES reach such bodies (degenerate self-loop
//!   chains whose darts share two vertices), and the tie let the face
//!   section's cross-component outer/ring pairing leak scan order — a
//!   false negative between isomorphic bodies. The attachment token
//!   now also references the COMMITTED labels of the face's loops, so
//!   once the first component commits, later twin components are
//!   pinned to it and the pairing is invariant. Residual blind spot:
//!   ties WITHIN one commit round between twins whose faces' loops are
//!   all uncommitted are still broken by scan order.
//! - **Tier-1-valid input is assumed**; the traversal panics (test
//!   failure) on bodies whose references do not resolve.

// Test-support code: panicking is a test's failure mechanism (L5).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::fmt::Write as _;
use std::collections::VecDeque;

use slotmap::SecondaryMap;

use crate::body::Body;
use crate::entity::{FaceKey, HalfEdgeKey, LoopBoundary, LoopKey, ShellKey, VertexKey};
use geom_core::Tol;

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

/// The canonical encoding of one shell: connected dart components are
/// labeled greedily by minimal candidate encoding (see the module docs),
/// then faces are emitted against the final labels.
fn canonical_shell(body: &Body<f64>, shell: ShellKey) -> String {
    let darts = shell_darts(body, shell);

    // --- Greedy component labeling. A shell's darts can fall into
    // several connected components under {next, mate}: a kemr-detached
    // ring shares no edges with the rest of its face, so this is an
    // ordinary operator-reachable state, not a corruption. Each
    // component's encoding is minimized over its own roots, and the
    // components are committed in ascending encoding order — both
    // choices are relabeling-invariant.
    let mut label: SecondaryMap<HalfEdgeKey, usize> = SecondaryMap::new();
    let mut order: Vec<HalfEdgeKey> = Vec::new();
    let mut vertex_label: SecondaryMap<VertexKey, usize> = SecondaryMap::new();
    let mut vertex_count = 0_usize;
    // Loop → minimal committed dart label, filled as components commit;
    // later components' attachment tokens reference it, pinning the
    // cross-component pairing (issue #60: without this, coordinate-
    // identical automorphic twin components tie and the FACE section's
    // outer/ring pairing leaked scan order — a false negative between
    // isomorphic bodies).
    let mut committed_loop_min: SecondaryMap<LoopKey, usize> = SecondaryMap::new();
    let mut out = String::new();
    while order.len() < darts.len() {
        let mut best: Option<(String, Vec<HalfEdgeKey>, Vec<VertexKey>)> = None;
        for &root in darts.iter().filter(|d| !label.contains_key(**d)) {
            let candidate = component_encoding(
                body,
                root,
                order.len(),
                vertex_count,
                &vertex_label,
                &committed_loop_min,
            );
            if best.as_ref().is_none_or(|(text, _, _)| candidate.0 < *text) {
                best = Some(candidate);
            }
        }
        let (text, members, new_vertices) =
            best.expect("an unlabeled dart exists while order is short");
        for member in members {
            let parent = body
                .get_half_edge(member)
                .expect("dart resolves")
                .parent_loop;
            if !committed_loop_min.contains_key(parent) {
                committed_loop_min.insert(parent, order.len());
            }
            label.insert(member, order.len());
            order.push(member);
        }
        for vertex in new_vertices {
            vertex_label.insert(vertex, vertex_count);
            vertex_count += 1;
        }
        out.push_str(&text);
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
    out
}

/// Every half-edge of the shell, in a deterministic (but key-dependent)
/// scan order — the candidate-root list for [`canonical_shell`]'s
/// greedy labeling (the scan order never reaches the output; only the
/// minimal encodings do).
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

/// One component's candidate encoding from one root: BFS in first-visit
/// order (successors `next`, then mate), dart lines carrying the
/// next/mate/vertex labels, followed by the coordinates of every vertex
/// the component introduces. Labels are absolute (`dart_offset` /
/// `vertex_offset` continue the shell-wide numbering); vertices already
/// labeled by earlier components keep their committed labels.
fn component_encoding(
    body: &Body<f64>,
    root: HalfEdgeKey,
    dart_offset: usize,
    vertex_offset: usize,
    committed_vertices: &SecondaryMap<VertexKey, usize>,
    committed_loop_min: &SecondaryMap<LoopKey, usize>,
) -> (String, Vec<HalfEdgeKey>, Vec<VertexKey>) {
    // BFS dart labeling within the component.
    let mut local: SecondaryMap<HalfEdgeKey, usize> = SecondaryMap::new();
    let mut members: Vec<HalfEdgeKey> = Vec::new();
    let mut queue = VecDeque::from([root]);
    local.insert(root, dart_offset);
    members.push(root);
    while let Some(dart) = queue.pop_front() {
        let dart_data = body.get_half_edge(dart).expect("dart resolves");
        let mate = body.mate(dart).expect("mate resolves");
        for successor in [dart_data.next, mate] {
            if !local.contains_key(successor) {
                local.insert(successor, dart_offset + members.len());
                members.push(successor);
                queue.push_back(successor);
            }
        }
    }
    // Vertex labels: committed ones first, fresh ones in first-visit
    // order.
    let mut fresh: SecondaryMap<VertexKey, usize> = SecondaryMap::new();
    let mut new_vertices: Vec<VertexKey> = Vec::new();
    let mut vertex_of = |start: VertexKey, new_vertices: &mut Vec<VertexKey>| {
        if let Some(&committed) = committed_vertices.get(start) {
            return committed;
        }
        if let Some(&assigned) = fresh.get(start) {
            return assigned;
        }
        let assigned = vertex_offset + new_vertices.len();
        fresh.insert(start, assigned);
        new_vertices.push(start);
        assigned
    };
    let mut text = String::new();
    for &dart in &members {
        let dart_data = body.get_half_edge(dart).expect("dart resolves");
        let mate = body.mate(dart).expect("mate resolves");
        let vertex = vertex_of(dart_data.start, &mut new_vertices);
        // The attachment token (role + face fingerprint) breaks ties
        // between automorphic roots whose symmetry only the FACE
        // structure distinguishes (e.g. a self-loop edge with one half
        // an outer loop and the other a ring): both label-free and
        // structure-invariant, so it sharpens the candidate order
        // without costing invariance.
        let attachment = dart_attachment(body, dart_data.parent_loop, committed_loop_min);
        let _ = writeln!(
            text,
            "  d{}: n{} m{} v{vertex} {attachment}",
            local[dart], local[dart_data.next], local[mate],
        );
    }
    // The component's own vertex coordinates close the candidate, so
    // two structurally identical components at different coordinates
    // compare (and sort) by their geometry too.
    for &vertex in &new_vertices {
        let _ = writeln!(
            text,
            "  v{}: {}",
            fresh[vertex],
            vertex_coords(body, vertex),
        );
    }
    (text, members, new_vertices)
}

/// A face's emission: orientation sense, outer-loop token, and sorted
/// ring tokens.
///
/// The sense (S10) is part of the face's identity, not of its surface:
/// two bodies with identical topology, identical loop roles and
/// identical coordinates are DIFFERENT solids when a face's material
/// side is reversed (they occupy complementary regions along that
/// face), and without this token they would hash identically — the
/// canonical form would certify a body isomorphic to its own
/// contradiction.
fn face_desc(body: &Body<f64>, face: FaceKey, loop_min: &SecondaryMap<LoopKey, usize>) -> String {
    let face_data = body.get_face(face).expect("face resolves");
    let outer = loop_token(body, face_data.outer, loop_min);
    let mut rings: Vec<String> = face_data
        .rings
        .iter()
        .map(|&ring| loop_token(body, ring, loop_min))
        .collect();
    rings.sort();
    format!(
        "sense={} outer={outer} rings=[{}]",
        sense_token(body, face),
        rings.join(",")
    )
}

/// A face's orientation sense as a single character (`+` agreeing with
/// the chart normal, `-` reversed).
fn sense_token(body: &Body<f64>, face: FaceKey) -> char {
    if body.get_face(face).expect("face resolves").sense {
        '+'
    } else {
        '-'
    }
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

/// A dart's attachment token: its loop's role on its face (`o`uter /
/// `r`ing) plus the face's label-free fingerprint, plus the face's
/// loops' COMMITTED minimal dart labels (`@{o=…;r=[…]}`, `?` for loops
/// not yet committed). Structure-invariant (no keys, no anchors): the
/// fingerprint breaks automorphism ties by face attachment; the
/// committed-label references pin later components to the labeling
/// already committed, so coordinate-identical twin components attached
/// to different faces no longer tie (issue #60 — the tie let the FACE
/// section's outer/ring pairing depend on scan order).
fn dart_attachment(
    body: &Body<f64>,
    parent: LoopKey,
    committed_loop_min: &SecondaryMap<LoopKey, usize>,
) -> String {
    let face = body.get_loop(parent).expect("loop resolves").face;
    let face_data = body.get_face(face).expect("face resolves");
    let role = if face_data.outer == parent { 'o' } else { 'r' };
    let reference = |loop_key: LoopKey| {
        committed_loop_min
            .get(loop_key)
            .map_or_else(|| "?".to_string(), ToString::to_string)
    };
    let mut ring_refs: Vec<String> = face_data.rings.iter().map(|&r| reference(r)).collect();
    ring_refs.sort();
    format!(
        "{role} {} @{{o={};r=[{}]}}",
        face_sig(body, face),
        reference(face_data.outer),
        ring_refs.join(",")
    )
}

/// A face's label-free fingerprint: its orientation sense, the outer
/// loop's signature, and the sorted ring signatures.
///
/// The sense belongs here as well as in [`face_desc`], and for a
/// different reason: this fingerprint is what breaks automorphism ties
/// when the canonical labeling has a choice. A body whose coordinate
/// symmetry maps face A onto face B is genuinely isomorphic to the one
/// with the senses swapped, and the two must therefore CANONICALIZE
/// the same way — which they only do if the tie-break can see the bit
/// that distinguishes A from B. Emitting it in the record alone would
/// leave the labeling free to pick either face and report two
/// isomorphic bodies as different.
fn face_sig(body: &Body<f64>, face: FaceKey) -> String {
    let face_data = body.get_face(face).expect("face resolves");
    let mut rings: Vec<String> = face_data
        .rings
        .iter()
        .map(|&ring| loop_sig(body, ring))
        .collect();
    rings.sort();
    format!(
        "{{s={};o={};r=[{}]}}",
        sense_token(body, face),
        loop_sig(body, face_data.outer),
        rings.join(",")
    )
}

/// A loop's label-free signature: the lone vertex's coordinates for an
/// empty loop, or the rotation-minimal sequence of start-vertex
/// coordinates around the cycle.
fn loop_sig(body: &Body<f64>, loop_key: LoopKey) -> String {
    match body.get_loop(loop_key).expect("loop resolves").boundary {
        LoopBoundary::Empty { vertex } => format!("E({})", vertex_coords(body, vertex)),
        LoopBoundary::Cycle { first } => {
            let coords: Vec<String> = body
                .loop_cycle(first)
                .expect("cycle closes")
                .into_iter()
                .map(|dart| {
                    let start = body.get_half_edge(dart).expect("dart resolves").start;
                    vertex_coords(body, start)
                })
                .collect();
            let len = coords.len();
            let minimal = (0..len)
                .map(|shift| {
                    let mut rotated = Vec::with_capacity(len);
                    for index in 0..len {
                        rotated.push(coords[(shift + index) % len].clone());
                    }
                    rotated.join("→")
                })
                .min()
                .expect("cycle is non-empty");
            format!("C({minimal})")
        }
    }
}

#[cfg(test)]
mod tests {
    use geom_core::Tol;
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
            .mev_line(
                MevSite::Lone {
                    r#loop: seed.r#loop,
                },
                pt(1.0, 0.0, 0.0),
                Tol::witness(),
            )
            .unwrap();
        body.mef_chord(MefSite::Chords {
            he1: seg.he_plus,
            he2: seg.he_minus,
        }, Tol::witness())
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
            .mef_chord(MefSite::Lone {
                r#loop: seed.r#loop,
            }, Tol::witness())
            .unwrap();
        body.mev_line(
            MevSite::Fan {
                he1: circle.he_plus,
                he2: circle.he_minus,
            },
            pt(1.0, 0.0, 0.0),
            Tol::witness(),
        )
        .unwrap();
        body
    }

    #[test]
    fn identical_builds_have_identical_forms() {
        let a = ops_cube(Tol::witness());
        let b = ops_cube(Tol::witness());
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
            .mev_line(
                MevSite::Lone {
                    r#loop: seed.r#loop,
                },
                pt(2.0, 0.0, 0.0),
                Tol::witness(),
            )
            .unwrap();
        body.mef_chord(MefSite::Chords {
            he1: seg.he_plus,
            he2: seg.he_minus,
        }, Tol::witness())
        .unwrap();
        assert!(!isomorphic(&a, &body));
    }

    #[test]
    fn form_is_invariant_under_cycle_first_rotation() {
        // Cycle::first is a representation-internal anchor; rotating it
        // must not change the canonical form (the kill ops re-anchor
        // loops unconditionally, so roundtrips depend on this).
        let t = ops_cube(Tol::witness());
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
        let t = ops_cube(Tol::witness());
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
        let cube = ops_cube(Tol::witness());
        let holed = ops_holed_box(Tol::witness());
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
                .mev_line(
                    MevSite::Lone {
                        r#loop: seed.r#loop,
                    },
                    pt(1.0, 0.0, 0.0),
                    Tol::witness(),
                )
                .unwrap();
            let split_faces = body
                .mef_chord(MefSite::Chords {
                    he1: seg.he_plus,
                    he2: seg.he_minus,
                }, Tol::witness())
                .unwrap();
            // Two hole anchors planted from face A's side (seg.he_plus
            // lives in the new face after mef; its mate in the old).
            let plant = |body: &mut Body<f64>, at, x| {
                let strut = body
                    .mev_line(MevSite::Fan { he1: at, he2: at }, pt(x, 0.0, 0.0), Tol::witness())
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
            .mev_line(
                MevSite::Lone {
                    r#loop: seed.r#loop,
                },
                pt(1.0, 0.0, 0.0),
                Tol::witness(),
            )
            .unwrap();
        let plant = |body: &mut Body<f64>, x| {
            let strut = body
                .mev_line(
                    MevSite::Fan {
                        he1: seg.he_minus,
                        he2: seg.he_minus,
                    },
                    pt(x, 0.0, 0.0),
                    Tol::witness(),
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
