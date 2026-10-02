---
id: split-halves-have-no-contact-records-so-no-pseudomanifold-self-check
kind: issue
title: split returns pinch halves with no ContactRecords for their touching vertex copies, so no pseudomanifold self-check of its outputs can pass a designed pinch half
status: spec
opened: 2026-10-02
priority: P3
cost: M
refs: [validate-passes-a-body-with-a-zero-width-slit-face, 3797]
---


## What

Measured by `tquery/split-self-validate` (PR 3797) on main, 2026-10-02.
Each of the 573 halves the topo + sweep suites split was run through
the pseudomanifold door (`gate_at_rest_kept` + `gate_at_rest_declared`)
with no contacts declared. 45 halves refuse even though their operands
pass:

- **37 are the designed pinch halves**: `notched_block_end_to_end`,
  `review_m3_pr3_bob`, `review_m3_pr6`, and the BOOL1 notch rows.
  - Their pieces touch along a tip line through distinct vertex copies.
  - The door reads those copies as undeclared `VertexVertex` contacts,
    and split returns no `ContactRecords`.
  - So that door can never pass a pinch half, and PR 3797's self-check
    stops at tier 2.
- **8 are CLEAVE's wrong-arc halves**, refused as `EdgeFaceOverlap`
  (`work/cleave/split-pairs-curved-face-crossings-across-the-wrong-arc.md`).

## The question

Should split declare the contacts its pinch halves carry, as the
boolean's outputs do through `ContactRecords`? A pseudomanifold
self-check of split's outputs, and the at-rest census of a stored pinch
half, both depend on the answer. Which layer declares them, and in what
vocabulary, is a design choice, so weigh it before building.

## The fork (2026-10-02)

Weighed by a designer pair and then a further designer
(`docs/DESIGN-FORK-LOG.md` row 45). Both answers keep a pinch half as
one body. They differ on how the touch is held:

- **Shared point.** The copies an op cuts from one vertex keep that
  vertex's `PointKey`. The census counts two vertices on one point as
  structural sharing, the ladder's first rung, as it already counts
  faces on one `SurfaceKey`. No record is needed.
- **Records.** Each split side comes back as a body plus
  `ContactRecords`, the boolean's shape. The records are projected at
  mint time from `SplitReduction::null_edges`.

The deciding question: when an op mints two vertices from one, is their
coincidence an identity inside the body, or a placement the op asserts
and the census verifies? The `[ev]` PR carries both, and DESIGN.md
tier 3′ is edited to the recommended answer.

## Ruled (2026-10-02, PR 3813)

Ev, after asking whether `Transform` works either way. It does: a
split's halves are separate carved bodies, and the pinch sits inside
one half. Ev's words: "i think a shared vertex, rather than two
identically located ones, is good then".

This is the shared-point answer: one `PointKey` referenced by each
prong's own vertex, not one vertex shared by both prongs. The
orchestrator confirmed the vocabulary on the PR.

DESIGN.md D1 tier 3′ carries the rule:
- "structural sharing (same surface or point key)";
- (ii) "or from an op's copies of one vertex, which share its point".

**The build:**
- `mev_null` hands each copy the original's `PointKey`, in both lanes.
- The census's vertex–vertex pass and the bound rungs
  (`ee_bound_backed`) back a pair of distinct vertices on one point as
  rung-1 structural. They still find such a pair and clear it, never
  skip it. Distinct points at one position stay `UndeclaredContact`.
- One crate-internal "move a vertex" door always mints a fresh point,
  so no write goes through a shared point in place.
- Re-baseline what moves: the iso point counts, and the euler
  null-edge rows that become key-identical.
