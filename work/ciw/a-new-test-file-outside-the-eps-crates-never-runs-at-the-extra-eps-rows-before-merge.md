---
id: a-new-test-file-outside-the-eps-crates-never-runs-at-the-extra-eps-rows-before-merge
kind: issue
title: A test file a PR adds or changes outside the four eps-sensitive crates never runs at 1e-6 or 1e-12 before merge, so a non-portable new test reaches main and reds the nightly
status: open
opened: 2026-10-01
---


## What

`.github/workflows/ci.yml`'s `change filter` job (the step that prints
`eps_extra=`) selects the 1e-6 and 1e-12 rows only for seeded
`step-import`, `geom-brep`, `profile` and `topo`, plus any crate with a
changed file whose path matches `^crates/[^/]+/.*(probe|golden)`. That is
the policy `work/ciw/latency-cut.md` ("What the gate runs") states. A
test file added to any other crate under a name without `probe` or
`golden` in it therefore runs at the default ε only before it merges.

Measured (2026-10-01):
- PR 3611 added `crates/sweep/tests/reach_volume_backstop.rs`. Its run
  36816755227 printed `SEEDS=geom-brep,pncad-py,sweep,topo` and
  `eps_extra=package(geom-brep) | package(topo)`. Sweep was seeded but
  got no ε rows. On main the suite failed 6 rows at 1e-6 and 4 at 1e-12
  (PR 3636 restates the rows in units of ε).
- PR 3627 touched sweep files named `*_probes.rs`, so the probe/golden
  clause selected `package(sweep)`, and that run went red on the same
  suite.
- Pushes to main run only the cache primer, so main's own CI never ran
  the suite at any ε. The first run to see it is the nightly.

`latency-cut.md` counts about 50 of 72 ε-only reds as "tests that were
not eps-portable": an ε literal, an absolute slack, a golden that embeds
ε. A test file that is new or edited is the likeliest place for one of
those, and today the gate never runs such a file at the extra ε rows
unless its crate is one of the four.

## The shape of a fix

Run the rows of the test files the diff adds or modifies at 1e-6 and
1e-12, as a `test(...)`/binary filter rather than a whole package. The
cost is those files' own rows, not the crate's suite. This widens the
gate's cost policy, so it is ciw's call and not a drive-by.

## Another instance (2026-10-06, SHELL PR 4111)

PR 4074 added `crates/sweep/tests/pinch_faces_tessellate.rs`. It
touched sweep, but no path in it matched `(probe|golden)`, so the file
never ran at 1e-6 before merge. Its `notch307 fib117` row escalates the
boolean at 1e-6 on main (`work/join/pinch-tessellate-row-escalates-at-eps-1e-6`).
The first gate run to see it was PR 4111's (run 37421965598). That PR
reached sweep's ε rows only because it edits two `*_probes.rs` files.
The gap therefore covers most of sweep's rows and every
non-probe/golden row of any crate outside the hard-coded four: they
first meet 1e-6 and 1e-12 at the nightly, and a PR that later touches
a probe file inherits the red.
