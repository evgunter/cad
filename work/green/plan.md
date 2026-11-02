# GREEN — the plan

Main green and the per-PR gate fast. Opened 2026-10-28 by a
priority-seam cut across the infra group (`work/green/log.md`).

## State

No orchestrator holds the track (`status: ready`). Its P0 and P1 rows
are closed (2026-11-02):

- **Latency.** Over 170 PR runs on 11-01/02 the median was 12.8 min
  and p90 19.7, with queue time negligible (median 3 s). Since then
  three levers landed, none adding a job: the gates run one per CPU and
  script readers are derived (#6324, about 13% of runner-minutes); the
  integration-test binaries build at opt-level 0 (#6361, kernel shard
  build 814 → 537 s paired, projected p90 13.9 → about 9.2 min). The
  next long pole is topo's lib unit-test harness, on TOPO's slate
  (`topos-unit-tests-recompile-the-whole-crate-on-every-topo-closure-pr`).
- **Nightly.** The full suite builds once and runs each ε row as its
  own job (#6323). Nightly 11-03 is its first run: check that each
  leg's `full-suite-junit-<eps>` artifact is populated and the build
  runner's disk. The suite's cost is the kernel, not `debug_assert`
  (#6339's census); the speed rows sit on SECTRUNG.
- **Main reds.** Every red named on this slate is greened, through
  #6343 and #6355. Nightly 11-02 (run 41393708363) is red and had no
  triage PR at hand-back; nor had 10-31 or 11-01.

## The slate

The board has it (`python3 scripts/work.py status --program green`):
P3 and P4 rows only. `test-shards-rebuild-dependencies-their-cache-holds`
pays little wall clock (the rebuilds overlap the first workspace libs),
and its priming fix adds runner-minutes on every main push, so weigh it
against queueing before taking it.

## How the track runs when picked up

Each morning, read that night's `triage/nightly-<date>` PR and green
what it left red, under `program.md`'s two rules: a red is greened,
not fixed, here; and GREEN acts on the triage, not beside it. Wait for
the triage table before touching any red.
