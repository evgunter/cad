---
id: door-scopes-outside-topo-are-unguarded
kind: issue
title: the door-scope guard reads topo/src only: a composite door in sweep, step-import or editor-core can skip D1's tier-1 sweep with nothing red
status: open
opened: 2026-09-10
---


PERF-4 moved D1's whole-body tier-1 sweep from every Euler operator to
every public door. What keeps that honest is
`review_m1_pr5_internal::every_public_mutation_path_preserves_tier1`,
which classifies each door by reading its body: it asserts per call, or
it opens a surgery scope and closes it, or it is allowlisted — and a
scope opened and never closed is a named failure
(`crates/topo/src/source_walk.rs`, `SurgeryPosture`).

**That walk reads `crates/topo/src` and nothing else**
(`source_walk::crate_sources`, and its own docs disclose the limit as
inherited by every guard built on it). Four of the doors this unit
scoped are outside it:

- `crates/sweep/src/extrude.rs:532`, `crates/sweep/src/loft.rs:334`,
  `crates/sweep/src/revolve/full.rs:187,356`,
  `crates/sweep/src/revolve/partial.rs:80`,
  `crates/sweep/src/blend/surgery.rs:616`;
- `crates/step-import/src/assemble.rs:835` (`build_one_solid`).

Nothing reds if one of them loses its close, and the failure is silent
by construction: the body still validates, the suite still passes, and
every operator that runs on that body afterwards has simply stopped
checking. The same hole opens for a NEW composite door written in
`sweep`, `step-import` or `editor-core` — it pays the per-operator
sweep it should not, or opens a scope and forgets the close, and no
guard has an opinion either way.

The runtime half is covered: `Body::open_surgery_scopes` is public and
returns 0 in a release build, so a row in any crate can assert that a
door it drove left no scope open (`crates/topo/src/surgery.rs`,
`every_door_returns_with_its_scopes_closed` does this for topo's).
What is missing is the SOURCE-level census over the other crates.

What would close this: either a shared door walk the other crates'
suites can mount — `source_walk::mutation_doors` is `pub(crate)` and
its floor assertion is `topo/src`-specific, so this is a move into
`test_utils::source` with a per-crate root, not a re-export — or a
per-crate row of the `open_surgery_scopes` shape over each crate's own
door list, which is weaker (it pins the doors a test happens to drive,
not the doors that exist).

Not PERF-4's: the unit's fence is TOPO territory and SEAT/BLEND ground
in `sweep` (`docs/PERF-4-SPEC.md` §5), and a walk that reads four
crates is a `test-utils` change with its own evidence.
