---
id: a-new-test-file-outside-the-eps-crates-never-runs-at-the-extra-eps-rows-before-merge
kind: issue
title: A test file a PR adds or changes outside the four eps-sensitive crates never runs at 1e-6 or 1e-12 before merge, so a non-portable new test reaches main and reds the nightly
status: closed
opened: 2026-10-01
priority: P2
cost: E
branch: ciw/eps-rows-changed-tests
closed: 2026-10-15
pr: 5211
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
boolean at 1e-6 on main (`pinch-tessellate-row-escalates-at-eps-1e-6`, JOIN, closed by PR 4083).
The first gate run to see it was PR 4111's (run 37421965598). That PR
reached sweep's ε rows only because it edits two `*_probes.rs` files.
The gap therefore covers most of sweep's rows and every
non-probe/golden row of any crate outside the hard-coded four: they
first meet 1e-6 and 1e-12 at the nightly, and a PR that later touches
a probe file inherits the red.

## 2026-10-13: a crate's own source edits miss the rows too (ENCL, PR 4925)

The gap is wider than new test files. `ci.yml`'s `change filter` hard-codes the ε crates (`for c in step-import geom-brep profile topo`) beside the `probe|golden` path match. A PR that changes geom-core's or editor-core's *source*, and re-baselines their existing tests, never runs those crates at 1e-6 or 1e-12.

PR 4925 (the ε floor) is the case:
- It changed `tolerance.rs` and `predicate.rs` and re-baselined `geom-core/tests/k_stats_doors.rs`. Both of that file's rows fail at 1e-12 without the re-baseline.
- It changed four editor-core tests.

The gate ran none of these at either extra row. They were exercised only by the lane's local `CAD_TOLERANCE_EPS=1e-12` and `1e-6` runs. A fix keyed to the seeded crates, instead of a fixed list, would cover both cases.

## Closed

`scripts/ci-filter.py` now computes the extra rows as `EPS_EXTRA`
(`eps_rows_filter`), and `ci.yml`'s `change filter` prints it as
`eps_extra=`. Beside the whole-crate terms it names, in any crate, the
rows of each test file the diff adds or modifies:

- a `tests/` suite is its `#[path]` module of the crate's `all` binary,
  read from `tests/all.rs` (`_all_rs_modules`), e.g.
  `(binary_id(sweep::all) & test(/^reach_volume_backstop::/))`;
- a changed `src/` file that defines a test (`#[test]`, `proptest!`) is
  its module of the lib-test binary, less every child module it mounts
  from another file, e.g.
  `(binary_id(topo) & test(/^chord_join::/) & not test(/^chord_join::(chart_ring_rows|…)::/))`;
- a changed `tests/all.rs` with tests of its own is the `all` binary
  less every module it mounts.

PR 3611's and PR 4074's sweep files, and PR 4925's
`geom-core/tests/k_stats_doors.rs`, `tolerance.rs`, `predicate.rs` and
editor-core tests, each now run at 1e-6 and 1e-12 before merge.

A notice in the filter log names what the file half cannot map: a
`tests/` helper no `#[path]` line includes, a `#[path]`-mounted `src/`
file (any visibility), a crate root, and a `src/bin/` file. Their rows
run at the extra ε nightly. A notice also names a changed file whose
rows are in ci's slow set, since this step filters those out.

One part of the 2026-10-13 note is not fixed here: a `src/` edit
outside the four crates that reds a test the diff leaves untouched.
That residue is carried by
`a-sweep-diff-gates-no-eps-row-of-sweep`.

