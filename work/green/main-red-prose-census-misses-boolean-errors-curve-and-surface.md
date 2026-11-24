---
id: main-red-prose-census-misses-boolean-errors-curve-and-surface
kind: issue
title: Main red: pncad-py's prose census finds two BooleanError fields (curve, surface in topo/src/boolean/mod.rs) it neither decides nor names
status: open
opened: 2026-11-24
priority: P1
cost: E
---


Found by ONEHOME's `onehome/mints-sweep` lane (PR 7625, CI run
45289734218, job `test (app)`, step "the slow-set tests this diff adds
or edits"), and reproduced locally on that branch, which has no diff in
`crates/topo`:
`pncad-py` `prose_census::tests::every_site_this_census_cannot_decide_is_named_with_its_reason`
(`crates/pncad-py/src/prose_census.rs`, the assertion near :2422) reds
with two sites the census cannot decide and `UNDECIDED` does not name:

```
("crates/topo/src/boolean/mod.rs", "BooleanError", "curve"), 1
("crates/topo/src/boolean/mod.rs", "BooleanError", "surface"), 1
```

**Likely cause:** main's latest change to `topo/src/boolean/mod.rs`,
`68ba6e030` (heldrecords/elbow-ends: `junction_end_corner`, the
`ReflexCorner` and `OpposedStretches` refusals). Not bisected.

**Why it hides:** the row is in the slow set, and the PR gate runs it
only on a PR that edits pncad-py's tests (`crates/pncad-py/src/tests.rs`
here), so PRs that touch topo alone do not see it; the nightly does.

It resembles the closed
`work/bind/main-red-prose-census-misses-two-boolean-error-fields.md`
(the `edge` and `face` fields, 2026-10-27), and its fix is the census's
own rule again: render each field so the census can decide it, or give
each its line in `UNDECIDED` with the reason it cannot be decided.

Reproduce: `cargo nextest run -p pncad-py --run-ignored all -E
'test(/every_site_this_census_cannot_decide/)'` on main.
