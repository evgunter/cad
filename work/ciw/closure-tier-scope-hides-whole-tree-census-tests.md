---
id: closure-tier-scope-hides-whole-tree-census-tests
kind: issue
title: A closure-tier run seeded outside geom-core never executes geom-core's whole-tree census tests, so a door added elsewhere can land with the census red
status: open
opened: 2026-09-05
---


(PROPS orchestrator) A class finding from PROPS-1's blinded review
(R2), filed here because the mechanism is the CI tier's scope.

Instance: FILLET's PR #1895 added `profile::path::arc_fillet::anchor_span`,
a sole-bracket `T: Bounds` door. `crates/geom-core/tests/bounds_census.rs`
walks the WHOLE tree for such doors and asserts each is in its roster —
but it lives in `geom-core`'s test binary. The PR ran `TIER=closure`
with `CARGO_SCOPE` = profile's dependents, which excludes `geom-core`,
so the census never executed and the PR merged green with `main` red on
`every_sole_bracket_bound_door_is_in_the_roster` in BOTH lanes (the
test is ungated). Observed on #1918's merge and on #1920's fix-pass
head; repaired by #1931.

Class: any test that reads files across the tree (a census, a roster, a
doc gate spelled as a test) is only as covered as the crate its binary
lives in. Options the filer sees, for CIW to weigh: (a) the closure tier
always adds the crates that host tree-walking tests (a named list, or a
manifest key); (b) such tests move to a tooling root that every tier
runs (`scripts/gates/` already runs on every tier); (c) `ci-filter.py`
treats a change under any path a census names as seeding the census's
crate. Which tests are tree-walking is the first thing to census.
