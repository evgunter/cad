---
id: main-red-the-prose-census-cannot-decide-contain-and-replace-face-sites
kind: issue
title: main red: the prose census cannot decide three ContainError sites and a ReplaceFaceError distance site, so every PR that selects it fails test (app)
status: closed
opened: 2026-11-28
priority: P1
cost: E
branch: lathe/prose-census-red
closed: 2026-11-28
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

## Cause

Bisected 2026-11-28 by running one census binary (built at `91f1ee7873`;
`prose_census.rs` is unchanged since `cf97ce423ed`, 11-23) over main's
first-parent source trees.

- **`ContainError`'s three sites: PR 7748** (`inside/contain-small-rows`,
  merged as `753c0ff11df`, 2026-11-25). It named the loop and edge in
  `RayExhausted`, `Uncrossable` and `ArcEndClearOfCorners`, rendering
  `u.r#loop`, `u.edge` and `gaps.edge` positionally. The resolver typed
  a bare binding, `self.field` and `ident.0`, not a named field. The tree
  just before it (`c8a4c30c1e6`) passes.
- **`ReplaceFaceError.distance`: PR 7658** (`germ/conic-constructions-audit`,
  merged as `cbd68a06410`, 2026-11-26). It added
  `MovedCircleCollapses { distance: T }`, rendered `{distance:?}`. The
  tree just before it (`8b52692b9e0`) has only the contain sites.
- **Not the one-band stack.** The row fails on every tree from
  `753c0ff11df` on, the nightly's `71e8c546fb` included. The 2026-11-27
  nightly did not run it: its "full suite, build once" job failed, so the
  full-suite shards that carry the slow set were skipped.

## Fix

- The census's resolver (`prose_census::expression_type`) now types one
  named field of a binding as the struct declares it, reading through a
  `Box`, as field access does. That decides the three `ContainError`
  sites, and `OffsetFitError`'s two `window.u`, `window.v` sites, whose
  `UNDECIDED` line is removed. No site reads as braced.
- `ReplaceFaceError.distance` is named in `UNDECIDED` with the reason its
  siblings `gap`, `offset` and `shift` carry: the `Real` scalar
  parameter, a brace at `Interval` and prose at `f64`.

Closed by PR (this branch, `lathe/prose-census-red`).
