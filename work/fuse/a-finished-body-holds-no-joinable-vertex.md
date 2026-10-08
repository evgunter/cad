---
id: a-finished-body-holds-no-joinable-vertex
kind: unit
title: Step 3 of the 3881 ruling: no joinable vertex at rest, checked at tier 3 by the join's predicate; every finisher, import included, ends with the join
status: dispatched
opened: 2026-10-07
pr: 4251
branch: fuse/tier2-no-joinable
priority: P1
cost: H
refs: [a-declared-merge-leaves-a-collinear-valence-two-vertex-an-earlier-cut-made, curved-joinable-vertices-are-left-unjoined, 4233]
---


## The question (to Ev)

This is step 3 of PR 3881's build order: the check that no joinable
vertex remains. The ratified clause says the form "is checked at tier
2 on the result alone".

Tier 2 (`validate_closed`) cannot make this check:
- it takes no band and is generic over `Real`;
- it gates construction state, and the merge's own output holds
  joinable vertices by design;
- since PR 4198, the predicate reads margins on its curved arms and
  can come back undecided.

The fork is where the check lives, and whether a finished body that
is not an op's output (an import, a hand-built body) must meet it.

The designer pair (fork-log row 88) ended agreeing. The check is a
tier-3 arm at rest, and every finisher, import included, ends with
the join. A vertex whose reading lands in band is exempt at rest.

## Ruled (Ev, PR 4251, 2026-10-08)

- **Decision 1, as recommended.** "No joinable vertex" is a tier-3 arm
  at rest (`JoinableVertexAtRest`), read by the join's predicate.
  Every finisher, import included, ends with the join. Tier 2 and
  construction state are untouched.
- **Decision 2, against the recommendation.** A vertex whose
  regularity reading lands in the sliver band **refuses** at rest,
  typed, and complains that ε is too high: "if this size is intended,
  tighten the tolerance below m/K".
- **Ev's reasoning.** A STEP file coarser than the offset is ε_in
  healing's to snap onto the pole, where the vertex reads in the zero
  band and is never joinable. So the only bodies that reach rest in
  the sliver band are ones whose file is finer than the run.
  `halfcap_eps6/7` are such files: they declare ε_in = 1e-10 m, and
  their split vertex sits 1e-8 m and 1e-9 m off the pole.
- **What moves.** `halfcap_eps6/7` refuse at the default ε, and the
  halfcap rows re-baseline to that refusal plus a pass at ε = 1e-10 m.

## What the build needs

- **The join door.** `join_stage` becomes a public `Body` door; today
  it is `pub(super)` in `boolean`.
- **One finishing function** for the boolean's four output paths
  (merge, describe, join).
- **The join at every finisher's end:**
  - split;
  - fillet/blend (`composed_die` holds 21 joinable vertices);
  - shell;
  - offset;
  - surgery;
  - the public merge door;
  - import, which reports its joins as `StructureNormalization`.
- **The tier-3 arm.** `JoinableVertexAtRest { vertex }` in
  `validate_geometric` and `_structural`, with in-band readings
  exempt.
- **Classify the 344 bodies first.** These are at-rest bodies on main
  that hold joinable vertices (FUSE log, 2026-10-07). Each is either a
  door output, which the door must fix, or a hand-built body, which
  stays construction state.
- **Settle `composed_die`.** Either the blend surgery leaves split
  rims, or the predicate reads a slit-seam end as joinable. Which one
  must be known before the arm lands.
