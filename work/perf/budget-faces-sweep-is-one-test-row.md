---
id: budget-faces-sweep-is-one-test-row
kind: issue
title: budget_faces.rs runs a 70-cell fit_offset_at sweep in one #[test] and is the whole geom-brep binary's wall
status: open
opened: 2026-09-10
---

## The finding

Measured by the PERF developer lane (`perf/explore-dev`; CI's dev/test
profile, 4 vCPU under the build-slot mutex).
`crates/geom-brep/tests/budget_faces.rs:22`
`every_budget_face_keeps_its_own_payload_invariants` is **46.4 cpu-s in
one `#[test]`**, and that row IS the geom-brep binary's wall (46.4 s;
the other 556 tests fit under it). It is a 2 bases × 7 deltas × 5
tolerances = 70-cell `fit_offset_at` sweep (`:23-35`) in a single row,
so three of four cores idle for the whole binary. Added 2026-09-08 —
after S-TCOST's timing census — with no `ci-filter` gate marker, so it
runs on every code-tier run, hosted and local.

## What a fix is

Split the sweep into rows nextest can spread (per base, or per δ), or
demote the fine cells per S-TCOST's shape rules
(`memories/test-suite-cost.md`: ask which SHAPE a test is before giving
it a seed). Saves ~30 s of every geom-brep run. Test-only: no A/B row.
S-TCOST territory (`crates/*/tests/*`); filed here because it was found
here and S-TCOST's census predates the file — handed over by
announcement if S-TCOST is live, dispatched from here otherwise.
