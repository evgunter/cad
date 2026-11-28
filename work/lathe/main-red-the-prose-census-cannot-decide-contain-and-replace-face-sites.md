---
id: main-red-the-prose-census-cannot-decide-contain-and-replace-face-sites
kind: issue
title: main red: the prose census cannot decide three ContainError sites and a ReplaceFaceError distance site, so every PR that selects it fails test (app)
status: open
opened: 2026-11-28
priority: P1
cost: E
---


## What

`pncad-py prose_census::tests::every_site_this_census_cannot_decide_is_named_with_its_reason`
(`crates/pncad-py/src/prose_census.rs:2436`) fails on main. It finds four
sites the census cannot decide and its roster (`UNDECIDED`) does not name:

- `crates/topo/src/contain.rs`, `ContainError`, `<positional>`, 3;
- `crates/topo/src/replace_face.rs`, `ReplaceFaceError`, `distance`, 1
  (`MovedCircleCollapses`, rendered `{distance:?}` at
  `crates/topo/src/replace_face.rs:1064`).

The test is in the slow set, so a push to main does not run it, and a PR
runs it only where its diff selects it. PR 7831 selects it, and its
`test (app)` job fails on it at every head.

## Measured

- Main at `91f1ee7873`: with `crates/` byte-identical to main's (PR
  7831's crate files checked out from `origin/main`), the row fails with
  those four sites. So it is main's, not 7831's.
- The nightly at `71e8c546fb` (2026-11-27) passed it. The one-band stack
  (PR 7414 onward, merged as `8b164f8e9c`, then 7826 and 7827) landed
  after that run. `contain.rs` last changed in `d459bb8b48` (LATHE, in
  that stack), though that commit edits no `Display` arm. Which change
  moved the census is not traced here.

## Done when

The four sites are rewritten so the census decides them, or named in
`UNDECIDED` with the reason each cannot be decided, and the row passes.
