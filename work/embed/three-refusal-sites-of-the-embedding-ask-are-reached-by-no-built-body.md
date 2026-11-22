---
id: three-refusal-sites-of-the-embedding-ask-are-reached-by-no-built-body
kind: issue
title: Three refusal sites of the embedding ask are reached by no body the suites build, so their stop is pinned by no row
status: open
opened: 2026-11-21
priority: P2
---


## Finding

From `embed/first-finding` (2026-11-21). The embedding ask ends at its
first refusal through one rule (`Bound::refuse_below`,
`crates/topo/src/clearance.rs`), read at seven sites. A mutant that
made one site keep its refusal and read on was run per site over
topo's `injective::`, `clearance::` and `shell::` unit sets and sweep's
`self_intersection_proves_injectivity` and `shell_cavity_embedded`
files (217 tests). Four went red. Three survived, because no test
reaches the site with the injectivity question:

- **a face its kind does not make injective**, in `Sweep::run`'s
  candidate loop (`injective::by_kind` answering `Err`: a torus not
  decided a ring, a spindle, a trimmed free-form face). The revolve
  door refuses the profile first (`ArcCrossesAxis` for a circle of
  radius 0.5 centred 0.5 and 0.5 + 1e-9 from the axis), and a trimmed
  NURBS face has no window, so the ask refuses before the walk.
- **a witness the verifier declines** (`Bound::settle`): the interval
  pass must classify a non-touching cell pair definitely at zero,
  which needs both cells narrower than ε, and on the one body tried (a
  plate whose void touches its top) a cell beside it runs out of depth
  first on the same level.
- **a terminal sliver or a poison enclosure off a shared cell**
  (`Bound::terminal`): two faces that share nothing, a distance inside
  the band apart. A void whose apex is 3ε or 5ε under the plate's top
  does not build (`Escalated { Coincidence(VertexOnFace) }`,
  `bool_vertex_face_side`), and a brick ε across fails its edges'
  certification (`interval_span_forward`).

The population is those 217 tests and the fixtures named; nothing
says no body reaches them.

## What is owed

A body for each, or the argument that none exists. Each site returns
the rule's `ControlFlow`, which is `#[must_use]`, so a site cannot
drop the stop silently; what is unpinned is that the receipt at each
(one refused, the rest abandoned) is what a body reads. They are also
the three arms that gained the two faces of their cell pair with no
body-level row: `Sliver` and `WitnessUnverified` are pinned as text
only (`crates/editor-core/tests/clearance_refusals_say_what_they_found.rs`).
