---
id: check-mesh-passes-the-empty-mesh
kind: issue
title: validate::check_mesh answers Ok on a mesh with zero triangles
status: open
opened: 2026-09-18
priority: P0
cost: E
parent: TESS-4
---


Measured by the rim-only-cap survey (`tess/rim-only-cap-diag` at
`83833e586`): a two-cap sphere whose both faces emit nothing tessellates
(debug assertions off) to `triangles = 0`, and
`mesh::validate::check_mesh` returns `Ok(())` — watertightness holds
vacuously over no edges, and `signed_volume` is 0. `tessellate_impl`'s
census comment already says so ("`check_mesh` passes the empty patch")
as a reason the census exists; nothing records it as a question about
`check_mesh`.

The call: whether the validator's contract is "closed 2-manifold" (the
empty mesh qualifies, and the doc should say the empty mesh passes and
why) or "the mesh of a solid" (then zero triangles is a refusal with
its own name). TESS-1 removes the one known producer; this row is about
the validator's claim, which outlives the producer.

## Decided (TESS-4, 2026-09-22)

**The contract is "the mesh of a solid", and zero triangles is a
refusal with its own name.** `mesh::validate::MeshError` gains
`NoTriangles` (the whole mesh) and `EmptyPatch { face }` (one face's
patch empty beside filled ones), both decided before the edge census so
the report names the emptiness rather than a neighbour's dangling edge.
**D2 addendum row 1** — reachable by input, invalid against this
contract — not row 4: a validator is handed meshes of unknown
provenance by contract (this crate's own audit suite hand-builds broken
ones), so it answers typed rather than panics. Row 0 is answered no at
the site: a non-emptiness guarantee on `Mesh` means a private triangle
buffer and a fallible constructor, which costs exactly the hand-built
broken mesh the validator exists to catch.

**The survey that decided it.** No caller takes `Ok(())` to mean
anything but "I am holding a solid's boundary": the tour and the wild
corpus (`demos/{tour,wild}/src/main.rs`, `tour/src/bossplate.rs`) panic
on `Err` beside a `signed_volume` check; `crates/stl`'s
`examples/export_acceptance.rs` uses it as the export pre-flight;
`tools/tess-meter`'s probes record `check_mesh(&m).is_ok()` as a
watertightness column over real bodies; `crates/mesh`'s and
`crates/sweep`'s suites assert `Ok` on solids and `Err` on hand-broken
meshes. **Not one caller legitimately validates an empty mesh**, so the
"closed 2-manifold, vacuously" reading had no constituency. `check_mesh`
is deliberately unbound in Python (`crates/pncad-py/tests/test_mesh.py`
says why), and `MeshError` has no exhaustive consumer anywhere — no tag
table takes an arm.

**Producers, measured or cited.** (1) `topo::Body::new()` is public and
`topo::validate`'s own doc says the empty body validates vacuously, so
it is tier-1 and tier-2 VALID; `tessellate_impl` has no face-count
guard and meshes it to zero patches and `Ok`. Pinned by
`survives_the_empty_body_meshes_to_nothing_and_the_validator_says_so`.
(2) A curved face that walks to a degenerate domain with debug
assertions off — zero height (refused typed since TESS-1) or zero width
(`rim-free-loop-on-a-poleless-chart-meshes-as-a-hole`, which measured
`patches = [0, 0]` and `check_mesh = Ok(())` on a two-face torus). (3)
The planar and trimmed lanes' near-zero-area member, read not executed
(`trimmed-and-planar-lanes-answer-ok-on-an-empty-patch`). A boolean
that annihilates its operands is NOT a producer:
`topo::BooleanResult::Empty` is a typed empty success carrying no body.
STEP import refuses `NothingToImport`; there is no STL importer.

**The per-patch question.** `check_mesh` names it; `tessellate` does
not. TESS-1's spec ruled that a REFUSAL is decided on structure and
reads no triangle count — `TessellateError::MeridianFreeCurvedFace`'s
doc says so in the present tense and is decided that way. A VALIDATOR
re-deriving a property from the emitted mesh is the other thing, and
the count is the only evidence it has. The cross-face census re-derives
it too but is `debug_assertions`-only, so in release `EmptyPatch` is
the whole of what sees a hole whose structural fact no guard in front of
the walk has found yet. The state was not unnamed before — an empty
patch leaves a neighbour's chord segment used once, so the edge census
reported `BoundaryEdge` (the rim-only cap beside a disc measured
exactly that) — but that names an edge and blames the wrong side.

**Rows** (`crates/mesh/tests/review_m2_pr6_checkmesh_audit.rs`):
`survives_checkmesh_refuses_the_empty_mesh` (no patch, positions with
no patch, every patch empty), `survives_checkmesh_names_the_face_of_an_empty_patch`
(empty patch beside a closed tetrahedron, and empty patch before an
open fan — the emptiness wins over the boundary edge it causes), and
the empty-body producer row above. The existing `check_mesh` rows stay
green.

Closing with TESS-4's PR.
