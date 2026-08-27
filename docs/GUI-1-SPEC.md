# GUI-1 spec — `Bvh::ray` + the hit-test service

Unit 1 of `docs/GUI-PLAN.md` (RATIFIED 2026-08-27). Headless — no
rendering dependency anywhere in this unit; CI-tested without
pixels. Independent of GUI-0 and dispatched concurrently with it:
**do not touch the `viewer` crate or workspace rows GUI-0 adds**
(crate-disjoint lanes). Read GUI-PLAN's GUI-1 entry,
`docs/GUI-DESIGN.md` G1, and `docs/GQ6-RESURVEY.md` §3 first.
Standing lane obligations: `docs/prompts/implementer-discipline.md`.

## Part A — `Bvh::ray` (crates/bvh)

Extend the existing deterministic AABB BVH (`crates/bvh` —
`Bvh::build`/`overlapping` in `tree.rs`, `Aabb` in `aabb.rs`) with
a ray query: a ray-slab test and a traversal. The re-survey's §3
recommendation, which the plan adopts: **extend our own BVH, do not
adopt `parry3d`** (that import buys nalgebra and a second geometry
vocabulary for a query we can write).

Contract requirements:

- **Conservative superset**, matching the crate's documented
  posture: the query must never miss a leaf whose box the ray truly
  intersects; returning extra candidates is legal. Get the IEEE
  slab-test corners right rather than fast: zero direction
  components (the `0 × ∞ → NaN` trap), rays originating inside a
  box, boxes with zero extent on an axis, `Aabb::poison`. State in
  doc-comments which side each boundary comparison falls on and
  why it is conservative.
- **Deterministic candidate order** (the crate is deterministic;
  keep it so — repeatable hit ordering is called out in §3's note).
  An ordering useful to the consumer (e.g. by entry-`t`, ties
  broken by index) is worth having; whatever you choose, document
  and test it.
- Ray parameterization: origin + direction over `t ∈ [0, ∞)` (or a
  documented finite range); no normalization requirement the API
  silently depends on.
- Picking is a UI concern with **no D9 obligation** (§3's note) —
  do not spend interval machinery here; plain f64 with conservative
  comparisons is the design.

## Part B — the `editor-core` hit-test service (`ray → stable ref`)

The G1 service: hit-testing is an editor-core service on the mesh
back-references and the shipped arena-key→stable-name inversion
(`editor-core::resolve::hit::entity_name`, which is total per its
header — an unnamed entity is a loud typed bug, never a swallow).

Shape (exact signatures are yours; the boundary rules are not):

- Input: an evaluated document (`editor-core::eval::Evaluation`),
  the tessellated mesh(es) for the node/bodies being displayed
  (`mesh::Mesh` with its `FacePatch::face` /
  `BoundaryPolyline::edge`/`start_vertex`/`end_vertex`
  back-references), and a ray.
- Work: BVH-accelerated candidate triangles → exact ray/triangle
  tests in f64 → nearest hit by `t` with a deterministic
  tie-break → triangle → `FaceKey` via the patch back-reference →
  `entity_name` → **`StableName`**.
- Output: a typed hit (name + node + `t` + hit point at minimum)
  or a typed miss; `HitTestError` from the inversion propagates
  typed — never flattened into "no hit". **No arena key crosses
  the layer 2/3 boundary** (G1): the service's public answer is
  names and typed errors only.
- Face picks are the v1 requirement (selection feeds the edit
  doors; GUI-2's ID-buffer consumes this service for its ray
  path). Edge/vertex proximity picking is welcome if it falls out
  of the same traversal but is NOT required — do not grow the unit
  for it; if you skip it, say so (that is a scoped exclusion, not
  a deviation).

The BVH build over triangles is per-mesh state the service consumer
holds or the service memoizes — design it so a static scene does
not rebuild per query; say in the PR where the build lives and
when it is invalidated (an evaluation epoch changing the mesh is
the obvious invalidator).

## Testing

- **bvh**: unit rows for the slab-test corners above; a
  randomized-by-effort-dial sweep (see `memories/test-suite-cost.md`
  — shape first, seeds never fixed) comparing `ray` candidates
  against brute-force box tests for the conservative-superset
  property; determinism row (same query twice, identical output).
- **editor-core**: pick every face of a box body through the public
  service; a ray down an edge between two faces resolves
  deterministically and documented; a miss is a typed miss; a
  poisoned/failed node surfaces its typed `HitTestError`; hit `t`
  ordering with two bodies/occlusion.
- Everything runs headless in ordinary hosted CI.

## Out of scope

Any rendering or egui/wgpu dependency; GPU ID-buffer (GUI-2);
selection values/highlighting (GUI-2); snapping toolboxes (the
`parry3d` fallback stays banked in §3 for if that ever grows).

## Acceptance

`Bvh::ray` shipped with the contract above; the editor-core service
answers `ray → StableName` on real tessellated bodies with typed
errors end-to-end; hosted CI green (the verification of record).

Branch `gui/gui-1-ray`; merge `origin/main` immediately before
opening the PR and re-merge whenever main moves (CONFLICTING = a
silent CI outage). NO Co-Authored-By trailer in lane commits
(A/B blinding; `memories/model-ab-experiment.md`).
