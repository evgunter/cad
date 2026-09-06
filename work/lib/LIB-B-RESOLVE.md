---
id: LIB-B-RESOLVE
kind: unit
title: binding census family B-RESOLVE
status: closed
pr: 1664
opened: 2026-09-03
closed: 2026-09-03
branch: lib/b-resolve
---

Queued mechanical census family (the B-READBACK/B-CHECKS shape): sweep the
family's bindings against the census contract, construct the previously
unconstructible pins where the surface now allows, and re-cut the census
rows honestly. Families share the census/tags/test files, so at most two
run concurrently, staggered.

## Derived scope (stated before any code changed)

`crates/pncad-py/tests/test_binding_census.py` charters `B-RESOLVE` in
`FAMILIES` and exactly three `NOT_BOUND` entries cite it, all under one
`gap:` prose ("names across runs"): `resolve`, `Resolution`, `RunCtx`.
That roster IS the family; nothing else in the census cites the id, and
the `NOT_BOUND` docstring's `B-RESOLVE` bullet names the same three.
All three were unbound at the start, so this is not a no-op.

The scope has an unusually crisp upper bound, and it is not this
unit's ruling: `crates/pncad/src/select.rs` carries exactly
`{Resolution, RunCtx, resolve}` from the resolution module and says at
length what it is leaving behind and why, and
`crates/pncad/tests/all.rs`'s `NOT_CARRIED` records the rest —
`Resolved`, `ResolveError`, `ResolutionFailure`,
`ResolveIndeterminate`, `Diagnosis`, `Tombstone`, `TieWitness`,
`RecipeEditRef`, `resolve_with_prior` — as "Naming interior", with the
GUI-2 history that put them back after carrying them briefly. Since
`pncad-py` depends on `pncad` and `quantity` alone, the three census
names are also the three names this crate CAN reach. That decision is
honored, not relitigated; its one consequence is banked as a finding
rather than acted on.

## Outcome

All three bound. `Resolution` crosses name-for-name; `resolve` becomes
`Evaluation.resolve` beside the read-back and picking doors; `RunCtx`
maps to `Evaluation`, which now captures the document at `evaluate`
beside the `ParamEnv` it already captured — a run is a (document,
evaluation) pair and Python's `Evaluation` became that pair, so a
caller cannot ask an evaluation about a document it is not of.

20 Python tests in `crates/pncad-py/tests/test_resolve.py`, one Rust
tag pin, both ty fixtures, census re-cut with the charter deleted and
the closure recorded. Two findings banked:
`work/lib/resolution-failure-arms-are-unmatchable-under-resolution.md`
(the carrier-projection rung, third instance) and
`work/lib/pncad-py-python-feature-clippy-lane-is-red.md` (measured en
route, inherited from main, not this unit's).
