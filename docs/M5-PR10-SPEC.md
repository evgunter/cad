# M5 PR 10 — sweeps/lofts as definitional feature nodes; schema v2 (binding spec)

Branch `ev/m5-pr10-sweeps-lofts` from current main. Plan line 10
(docs/M5-PLAN.md:275-282). Ratified contract: C11 sweeps/lofts
(CURVED-DESIGN :620-626), Q8 definitional posture (DESIGN.md
§Q8), R3 + Evan's rider (M5-PLAN :116-129). Depends on PR 3
(NURBS substrate) + PR 4 (fit/linalg); does NOT depend on PR 9 or
PR 7b — coordination with the boolean layer is §5's honest-refusal
clause. Binding; deviations numbered and reported.

## 1. Feature nodes (recipe vocabulary)

- `Node::Loft { profiles: Vec<RecipeNodeId>, .. }` — skinned
  surface through ≥2 section profiles (Book §10.3). `Node::Sweep {
  profile, path, .. }` — profile swept along a path (Book §10.4,
  scope-boxed: rigid profile, translational or path-following;
  no variable sections, no scaling laws). Both join the M4 node
  vocabulary as ORDINARY ops: slots per the D5 named-slot rule,
  structural/continuous divide preserved, input refs resolve to
  existing nodes only.
- Q8 posture, stated in rustdoc at the node and the surface: the
  produced NURBS **is** the definition; the recipe is provenance;
  NO residual obligation, NO approximating-surface machinery.
  Only DERIVED items (intersections with these walls, pcurves of
  non-iso edges) carry certificates.

## 2. Geometry (Book ch. 10, under the existing substrate rules)

- Loft: make sections compatible — degree-elevate to the common
  degree (§5.5), merge knot vectors (§5.3) — then interpolate
  v-directionally (§10.3's global interpolation; the linear solves
  go through geom-core::linalg's fixed-shape rules, D9; banded
  solver only if PR 4's stack already provides it — otherwise
  dense small systems, a numbered note on size limits).
- Sweep: §10.4's translational and path-sweep constructions only.
- All evaluation generic over `Real`, de Boor fixed recursion
  order, w > 0 enforced at construction (the convex-hull invariant
  every C9 bound stands on).
- Incompatible inputs refuse TYPED at the node door (profile loop
  counts differ, open vs closed mixed, path/profile plane
  degeneracies) — closed error enum, two-tolerance message shape
  INCLUDING definite arms (S9 lesson).

## 3. Topology of the produced solid

- Caps from the end profiles; NURBS walls between; wall-wall
  seams where the profile has corners (one wall strip per profile
  segment — the segment structure is the u-direction).
- Cap-wall and wall-wall edges are **iso-parameter curves of the
  produced surface by construction** — their pcurves are exact
  straight lines in UV (the definitional payoff). Store them
  through PR 6's doors as an exact lane; no fitted pcurve, no
  UnsupportedCarrier hit. If the existing Pcurve enum needs a
  Line-in-UV variant, that is IN scope here (it is exact — not
  PR 9's deferred Fitted variant; coordinate by rebase if 9c
  lands it first).
- Tier gates: the produced body validates at tier 3 (transverse
  edges at caps; wall-wall seams are the smooth-join conventional
  class — G2/G1 per profile tangency declarations, consistent
  with #101's declared-tangency discipline and PR 9's mark).

## 4. Schema v2 (R3 — minted HERE)

- **The call: MIGRATE, identity-shaped** (recorded per the rider;
  consultation posted 2026-07-31, #148 comment 5147668504).
  Grounds: v1 = `ProfileDoc` snapshot + `DocEdit<ProfileDesc>`
  log; the v2 delta is purely additive (new Node variants + slot
  ids); every v1 file parses under v2 types. `migrate(1→2)` =
  parse v1 body with v2 types + bump header, through the existing
  chain (persist/mod.rs:191). Write one v1 GOLDEN file into the
  test tree and pin: loads under v2, replays identically, re-saves
  as v2. **Flip condition**: if drafting-to-code finds the loft
  input model forces a ProfileDoc/DescToken restructure, STOP and
  report — the call flips to clean-break only via the orchestrator
  and Evan (#148 thread), never silently.
- `SCHEMA_VERSION = 2`; UnknownSchema stays the too-new refusal;
  round-trip + replay rows at the post-S7 battery convention
  (default + 1e-6 + 1e-12 + Interval).

## 5. Acceptance

- **Shape (iii)'s loft body**: a definitional loft (≥3 sections,
  at least one non-affine pair so the wall is genuinely curved)
  builds, validates tier-3, persists v2, round-trips + replays
  bit-identically, joins the Band 4 corpus with persistence/
  latency rows.
- The plane-CUT of that body is the exit criterion's NURBS-wall
  boolean: run it end-to-end IF the boolean layer accepts it at
  this PR's merge time (PR 9 + 7b + the edge×NURBS-face layer);
  otherwise pin the TYPED refusal naming the missing layer and
  hand the green row to the PR that closes it. Never a silent
  skip.
- Sweep acceptance: a path-swept profile body, same rows.
- v1 golden migration row (§4). Demos: loft + sweep demo stops
  authored (render rides tessellation's current planar-cap
  ability; full curved-wall render lands with PR 11 — say so in
  the demo text).
- Multi-ε honesty: placements scale from the resolved band;
  derived sample counts; skip-with-reason only where scaling is
  dishonest.

## 6. Out of scope

Offsets/shelling (Q8, M5+); scattered-data fitting (M7); variable
sections/scaling sweeps; degree reduction; ruled-surface special
cases beyond what §10.3 gives free; curved tessellation (PR 11);
fillets on these bodies (PR 12); any marcher/SSI change. Frontier
errors name the missing front door.

## 7. Process

One implementer + one blinded adversarial reviewer + one fix
pass. Review charter must include: independent §10.3
interpolation verification (evaluate the loft at section
parameters — must pass through the sections exactly at ring
tolerance); compatibility-refusal probes; a v1-corpus migration
sweep (every existing golden loads under v2); persistence
symmetry attack (save/load/save bit-compare); the CODE QUALITY
REPORT with the fixed rubric. Touched-crate battery (geom-curves,
geom-surfaces, sweep, editor-core + consumers) at default ε +
Interval; hosted CI is the gate. Push per unit. OUTPUT DISCIPLINE
and foreground-verification clauses per standing process.
