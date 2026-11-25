---
id: a-slow-rows-eps-pins-run-nowhere-on-the-pr-that-adds-them
kind: issue
title: The slow rows a PR selects run at the default ε only; run them at 1e-6 and 1e-12 too, under a per-row time cap from the nightly's junit
status: open
opened: 2026-11-14
priority: P2
cost: E
refs: [6897, 7057, 7078]
---

**The gap.** The slow-set step (`.github/workflows/ci.yml:457-475`) runs
the slow rows a PR selects at the default ε. Since PR 7359 that means the
rows it names, plus every slow row in a file it edits. The ε step
(`ci.yml:432-447`) runs under `--profile ci`, which leaves the slow set
out. So a slow row's 1e-6 and 1e-12 assertions first run on the nightly,
on the PR that adds them as on every later one.

**Instances since 2026-10-22.**
- **Merged red:**
  - PR 5872 (10-28): four `pair_section` and `cylinder_sphere_frame` rows
    red at 1e-6;
  - PR 6897 (11-17): `node_end_rows` red at 1e-6.
- **Caught by review or by hand:**
  - PR 6946 (11-14): `far_walls_of_many_arcs`, whose pins are all at
    1e-12;
  - PR 6972 (11-15): `m10_5_r1` at 1e-6 and 1e-12, and
    `coincidence_door` at 1e-12;
  - PR 6985 (11-15): three of its six re-stated rows red at 1e-12;
  - PR 6934.
- **Worked around:** PR 7078 (11-18) added a fast row so its 1e-12
  branch would be gated at all.

**The cost.** Times come from the nightly's junit (run 44870651337,
opt-2), scaled by about 2.5 for the PR gate's opt-1. A file's slow rows
cost a median of 2 s at the default ε and 3 s at 1e-12, and a p90 of 34
s and 52 s. So adding 1e-6 and 1e-12 costs about 10 s for a median
selecting PR and about 3 to 4 min at the p90, and nothing on a PR that
selects no slow row. That last case is the typical PR.

The tail at 1e-12 is the risk: 113 slow rows take more than 30 s there
and 10 take more than 300 s. A cap keeps it bounded. Each row runs at
1e-12 only when its nightly 1e-12 time is under 30 s, read from the last
nightly's junit or from a list kept beside the slow set. The rows over
the cap are named in the step summary as not run.

**What it subsumes.**
- `the-slow-tests-a-diff-adds-run-at-the-default-eps-only`, the same gap
  with the cost of the PR 5872 rows.
- Most of the cases `an-extra-eps-step-that-selects-no-test-reads-green`
  describes.

What would close it: the slow-set step's selection also runs at 1e-6
and, under the cap, at 1e-12, and its summary names the rows the cap
left out.

- 2026-11-22 — **Reformulated by CIW triage** (the earlier body is in git history): - Since 10-22:
- merged red: PR 5872 (10-28, four rows at 1e-6) and PR 6897 (11-17,
`node_end_rows` at 1e-6);
- caught by review or by hand: PRs 6946, 6972, 6985 and 6934;
- worked around: PR 7078.
- Cost, from nightly junit scaled ×2.5: a median edited file's slow rows
add about 10 s at 1e-6 plus 1e-12, and the p90 adds about 3 to 4 min.
A PR that selects no slow row pays 0, and that is the typical PR.
- The 1e-12 tail needs a cap: 113 slow rows take more than 30 s at
1e-12, and 10 take more than 300 s.
- Reform at `reform/a-slow-rows-eps-pins-run-nowhere-on-the-pr-that-adds-them.md`:
run at 1e-6 always, at 1e-12 under a 30 s per-row cap, and name the
rows the cap skipped.

## Seen again (2026-11-25, SSIARITH, PR 7781)

PR 7781 edits `crates/sweep/tests/verbs_cylcyl_r1_review_probes.rs`'s
slow row `every_reachable_crossing_pose_refuses_typed_or_answers_correctly`:
its 1e-12 pin removed, its 1e-6 pin re-pinned on new digits. The gate
(run 45594701363) ran the row once, in the slow-set step at the default
ε (75 s). The `eps 1e-6, 1e-12` step's `--profile ci` dropped it, so
neither changed pin had a hosted reading; the lane ran both locally
instead. — (SSIARITH orchestrator)
