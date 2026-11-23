---
id: flat-harmlessness-is-unproven-on-the-nurbs-refinement-lane
kind: issue
title: The Flat stop's bound on the count below its rung does not reach the NURBS lane's certificate refinement
status: open
opened: 2026-10-22
priority: P3
cost: M
---

## The finding

`crates/viewer/src/scene.rs`, `fit_delta`'s Flat paragraph, bounds
what a `ProbeStop::Flat` stop does not see. The stop's rung counts
under `TRIANGLE_BUDGET / PROBE_FACTOR`. Across the factor of 8 down
to the request, every step `mesh::sizing` sizes (`curvature_step`,
and `sagitta_step` to within √8·1.0057) shrinks by at most about √8
per direction, so each `ceil_count` grows at most that much plus one.
The count drawn at the request is therefore about the budget at most.

That argument is per-direction and step-sized. The NURBS lane of
`crates/mesh/src/trimmed.rs` has a second arm the argument does not
cover: **certificate-driven refinement** (the module docs, and the
`refine` queue in the emit pass, `refine_lane && bound > tol.delta`).
Any NURBS triangle whose certificate exceeds δ gets its UV centroid
queued as a new candidate. The attempt then rebuilds, for up to
`MAX_GRID_RETRIES` (6) rounds. How many triangles that adds between
the Flat rung and the request is not a function of a step, so the
√8-per-direction bound says nothing about it. A NURBS body flat at
`PROBE_FACTOR` × the request whose refinement fires only below it
could draw more than ~8.1× the rung.

Nothing in the corpus exercises it. Every Flat document in
`crates/viewer/tests/display_budget.rs`'s `ANSWERS` draws a few
hundred triangles, and the module docs call the refinement a
"LAST-RESORT backstop".

## What closing it takes

Either a bound on what one refinement round can add relative to the
grid (each round adds at most one centroid per failing triangle, and
the rounds are capped, so a bound exists; it needs stating against
the grid count), or a fixture: a trimmed NURBS face flat across the
last rung step whose refinement fires at the request, measured
against the fit.

## Where it came from

The FIT lane on `the-budgets-predicted-count-is-not-always-an-over-count`
and `a-flat-rung-pair-is-read-as-flat-below-it` (PR 5600). This row is
the disclosed residue of the second.

## Moved (2026-10-26)

FIT folded back into VGEOM (Ev, in chat, 2026-10-26): FIT was cut from
VGEOM on 2026-09-20 to run in parallel, a reason that is gone at FIT's
size. Id and body unchanged.
