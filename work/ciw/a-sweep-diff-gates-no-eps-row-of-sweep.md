---
id: a-sweep-diff-gates-no-eps-row-of-sweep
kind: issue
title: a kernel diff's rows that it does not edit — sweep's under a topo change (PR 5192), or a seeded crate's outside the four — meet 1e-6/1e-12 only at the nightly
status: open
opened: 2026-10-01
priority: P2
cost: M
refs: [5211]
---

Filed by the reach-eps lane (REACH, PR 3636).

**What.** `.github/workflows/ci.yml`'s change filter (the "EXTRA EPS
ROWS" block) adds a crate's 1e-6 and 1e-12 rows only when the diff seeds
one of `step-import geom-brep profile topo`, or touches a path matching
`crates/<c>/.*(probe|golden)`. `sweep` is in neither. PR 3636 changes
`crates/sweep/tests/reach_volume_backstop.rs`, and its run 36823485092
logs `eps_extra=package(topo)`, which comes from its doc edit in topo.
So none of the rows it fixes run at 1e-6 or 1e-12 at the gate.

Those rows' eps-only red reached main through PR 3611 the same way
(`reach-volume-backstop-fails-off-the-default-eps` (REACH, closed by PR 3636)). It
surfaced only on a diff that ran `all()` (EDIT's PR 3625), and then it
blocked unrelated PRs (#2468). The filter's own comment names the rule
for joining the list ("the eps-only reds of 2026-07..09 sat in these
crates"), and sweep now qualifies.

A `workflow_dispatch` with `scope=all` would cover it by hand. From an
agent session's integration token that dispatch is refused with
`403 Resource not accessible by integration`.

**The question for CIW** is the cost of adding `sweep` to the list
against the latency cut, or a narrower route such as the eps rows of
the changed test files only.

## 2026-10-15 — a kernel diff in topo, red only in sweep at 1e-12 (TANG, PR 5192)

PR 5192 changes `crates/topo/src/boolean/` and adds sweep rows. Its
row `verbs_germarms2_interval::the_meeting_axes_pose_builds_at_the_certified_scalar`
refused at ε 1e-12 only (`carrier_endpoint_end`, an interval split
vertex [0, 1.76e-12] wide). Local runs of topo, sweep and editor-core
at the default ε passed. CI caught it only because the same diff edited
`.config/nextest.toml`, which forced `TIER=all` and `eps_extra=all()`.
A topo-only diff makes `eps_extra=package(topo)`. That runs none of the
sweep rows where topo's booleans are exercised, so the same defect
would have merged green and reddened the nightly.

## 2026-10-15 — the changed files are covered; the rest is priced (CIW)

The extra rows now include every test file the diff adds or modifies,
in any crate (`scripts/ci-filter.py`'s `eps_rows_filter`, closing
`a-new-test-file-outside-the-eps-crates-never-runs-at-the-extra-eps-rows-before-merge`).
That covers the cases above: PR 3636's
`crates/sweep/tests/reach_volume_backstop.rs` would now print
`(binary_id(sweep::all) & test(/^reach_volume_backstop::/))`.

What this row still holds is a red in a row the diff does not edit.

**PR 5192.** Run locally on its crate files, without the
`.config/nextest.toml` edit that forced `TIER=all`, the filter now prints
`package(topo) | (binary_id(sweep::all) & test(/^bisector_nodes::/))`.
The red row sits in `crates/sweep/tests/verbs_germarms2_interval.rs`,
which the diff does not edit, so it would still merge green.

**What covering it costs.** These are step times from `get_workflow_job`
on 2026-10-15:

| run | `eps_extra` | ε 1e-6 + 1e-12 step |
|---|---|---|
| 38804490307 (PR 5198) | `package(geom-brep) \| package(topo)` | 4m05s |
| 38806115057 (PR 5204) | `package(geom-brep) \| package(topo) \| package(sweep)` | 8m58s |

The 1e-12 pass of 38806115057 ran 7193 tests in 320 s. So sweep's own
rows cost about 4.9 min a PR across the two passes, on a job whose
build already exceeds the 15-minute bar
(`a-sweep-seeded-diff-runs-the-test-job-past-the-fifteen-minute-bar`).

- Adding `package(sweep)` for a seeded sweep would cost about 5 min
  on every sweep PR.
- Adding it for every topo-seeded PR, which is PR 5192's case, would
  cost the same on every topo PR.

Both exceed the "a few minutes" this gate can afford, so neither is
done.

**Seeded crates outside the four** (geom-core, editor-core, mesh, and
the rest). A `src/` edit there runs only its own changed file's rows.
Their whole-suite row cost is unmeasured. A local measurement was
abandoned because the box ran out of disk. The per-test logs that
would give it are past the 5000-line tail that `get_job_logs` returns.

## Owed

- The per-crate cost of one ε row for each member outside the four.
  A run whose `eps_extra` names them would give it, for example a
  `workflow_dispatch` with `scope=all` read per binary from a log
  artefact, or a nextest JUnit upload.
- With those costs in hand, decide `EPS_ROWS_PACKAGES`
  (`scripts/ci-filter.py`). One option is "every seed except those
  whose rows cost more than N s".
- PR 5192's case, a dependent crate's untouched rows: either accept
  it as the nightly's, or find a cheaper selector than
  `package(sweep)`. One candidate is the sweep suites that call the
  changed topo module, if a reverse index from module to suites is
  cheap to keep.

