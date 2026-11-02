---
id: two-bisector-node-rows-break-the-fast-set-one-second-bar
kind: issue
title: Two sweep bisector_nodes rows take over 1 s in the fast set, so the one-second bar (#5229) reds every PR that seeds sweep
status: closed
opened: 2026-10-16
priority: P1
cost: E
closed: 2026-10-28
---

#5229 (`ciw/fast-set-one-second-limit`, `01048df235`, closing `work/ciw/fast-set-tests-over-one-second-are-not-enforced.md`) made `scripts/check-fast-set-times.py` fail the per-PR default-eps step on any fast-set test at `BAR = 1.0` s or more. Two `sweep` rows in `crates/sweep/tests/bisector_nodes.rs` are in the fast set, are not in `.config/nextest.toml`'s `ci` default-filter, and take longer than that, so every PR that seeds `sweep` goes red. #5233 is the first PR seen hitting it.

Measured on main (`9ae7e2e959`) under `--profile ci` with CI's `CARGO_PROFILE_{DEV,TEST}_OPT_LEVEL=1`, alone (`-j1`) on a 4-vCPU box:

| test | time |
|---|---|
| `bisector_nodes::the_near_node_poses_at_the_witness_band` (`:441`) | 5.09 s |
| `bisector_nodes::an_oblique_seam_near_a_node_builds` (`:716`) | 1.07 s (1.34 s in parallel) |

The file's other four fast-set rows take 0.05 to 0.18 s. Its seven heavy rows are already in the slow set. Both new rows came with #5203 (`tang/pinch-u2p-pair-reading`, `f4ebb63a56`), which merged after #5229's survey table was taken, so neither is in that item's table.

**Fix:** add both to the `binary_id(sweep::all)` slow-set filter in `.config/nextest.toml`, or make them faster. `the_near_node_poses_at_the_witness_band` runs only at the witness band (it stands down elsewhere), and `an_oblique_seam_near_a_node_builds` runs at every row. Neither has a recorded catch that the fast set would otherwise miss.

**Also seen on PR 5225** (PCERT, `pcert/azimuth-placement`), CI run
38996794002: `the_near_node_poses_at_the_witness_band` took 7.276 s and
`an_oblique_seam_near_a_node_builds` 1.620 s on its test job. Release
builds (`--build-jobs 2`) of main at `52b4501d12` and of PR 5225's head
time them alike: 2.42 s against 2.45 s, and 0.74 s against 0.80 s. So
the bar fails on that PR from main's rows, not from the PR.

## Closed (2026-10-28, GREEN's cut)

Already fixed: both rows (`bisector_nodes::the_near_node_poses_at_the_witness_band`
and `bisector_nodes::an_oblique_seam_near_a_node_builds`) sit in
`.config/nextest.toml`'s `binary_id(sweep::all)` slow-set filter on main
`9809965b`, which is the fix this row asked for.
