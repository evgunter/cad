---
id: topo-tests-mapped-cube-call-sites-that-are-brick-with-ceremony
kind: issue
title: Nine mapped_cube call sites pass a diagonal affine map, which is brick spelled the long way
status: open
opened: 2026-09-16
priority: P4
cost: E
---


## Finding

- **Where**: `crates/topo/tests/` — `census_g2_carrier.rs:71`,
  `h14_census_deferrals.rs:38`, `m9_2_census_door.rs:40` and `:402`,
  `m9_2b_r2_probes.rs:22`, `review_m3_pr6.rs:259`, `:289`, `:331`,
  `m3_pr6_tier3prime.rs:335`.
- **Importance**: low-medium — vocabulary, not correctness
- **Confidence**: sure the maps are diagonal affines; the equality they
  rely on is **proved**, not assumed
- **Raised by**: the full review of PR #2727 (S-DUP), 2026-09-16

Each of these passes `common::mapped_cube` a map of the form
`|x, y, z| Point3::new(sx*x + dx, sy*y + dy, sz*z + dz)` — a diagonal
affine, which carries the unit cube onto an axis-aligned box and
nothing else. `common::brick((dx, dx+sx), (dy, dy+sy), (dz, dz+sz))`
is the same body: PR #2727 proved it by execution at two boxes, and
`crates/topo/tests/cube_doors_agree.rs` now holds it as a standing row
at four. So these sites spell a `brick` the long way, in a suite that
already has `brick` in scope at most of them.

This only became a duplication finding once the equality was measured.
Before that, a reader could reasonably believe the two doors differed
and that naming `mapped_cube` was a choice about which body you wanted.

## The instrument for extending this census

**Every `mapped_cube` argument that is a diagonal affine.** Not a grep
for `mapped_cube` — that returns the tilts too, which are exactly the
sites that must keep it. Parse the closure body and ask whether each
component mentions only its own coordinate. The nine above came from
reading all `mapped_cube` call sites in the tree by hand; the same
question applies to `cube_into`'s four (three of which are diagonal:
`review_m3_pr6.rs:260`, `:290`, `m3_pr6_tier3prime.rs:336`), and there
is no `brick_into` for them to move to yet.

## The standing question at every site

`work/dup/plan.md` method item 6: does the helper still make that
suite's intent readable? A census suite that says
`mapped_cube(|x,y,z| Point3::new(x + dx, y + dy, z + dz))` may be
saying *"this body is a translate of that one"* on purpose, and
`brick` at computed extents would say it worse. Answering **no** at a
site is a result, not a failure to finish.
