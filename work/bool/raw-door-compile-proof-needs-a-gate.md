---
id: raw-door-compile-proof-needs-a-gate
kind: issue
title: BOOL-9's shut-build proof (a downstream crate cannot mint a ProfileLoop without the test-support edge) is a measurement, not a row — it needs a gate with a toolchain
status: open
opened: 2026-09-08
refs: [BOOL-9, 2134]
---

BOOL-9 (PR 2134 §11) measured the enforcement by hand — a witness crate
depending on `profile` with no features reds `E0603: trait RawLoop is
private`; the same crate with `features = ["test-support"]` compiles —
and could not make it a row: the hosted test jobs run from a nextest
archive on a runner with no toolchain and no registry, so a nested
`cargo check` row reported "cargo not available" rather than the fact
it claimed (that row was removed from the branch). What stands
permanently are three census rows (the pncad root-export census, the
production-writer census, the gate-at-the-door row), which pin the
tree's text rather than the compiler's verdict.

The permanent home is a `scripts/gates/` entry with its selftest — the
witness crate checked shut and open — wired into a CI job that has a
toolchain (the `discipline` job, or a new one). That is CI surface
(CIW's territory; `ci.yml` had three lanes editing it the week this was
filed), so it is filed rather than done inside BOOL-9. Difficulty S.
Announce to CIW before wiring.
