---
id: sccache-trial-verdict-to-read
kind: issue
title: sccache on trial - check in a few days whether it actually helped
status: closed
opened: 2026-08-21
github: 853
refs: [852, 1648]
pr: 1648
branch: tcost/c4-sccache-reread
closed: 2026-09-04
---

## From GitHub issue 853

Opened 2026-08-21; 0 comments.

sccache went on trial in #852 (audit finding F4, `docs/CI-MINUTES-2026-08.md`). It is **on** — the kill switch is the repo variable `SCCACHE` set to `"0"`; unset means enabled. This issue is the reminder to go back and read the result, in a few days, once enough warm runs have accumulated.

## Why it was worth trying at all

`docs/LOCAL-BUILD-PERF.md` reverted sccache locally on a real measurement — cold build 156 s → 96 s at a 99.4% hit rate, given up because sccache and incremental compilation are mutually exclusive and going non-incremental cost 5–7× on the edit-rebuild loop.

**That objection does not reach the runner.** `Swatinem/rust-cache` sets `CARGO_INCREMENTAL=0` itself, on every job that uses it, so hosted CI has already paid sccache's one documented cost and currently gets nothing back for it.

## What to actually check

**Discard the first run after #852 merged.** `RUSTC_WRAPPER` is `RUST*`-prefixed, so rust-cache hashes it into its key: the flip buys exactly one cold rebuild. That run is not the verdict — the same trap the `OPT LEVEL` note on the build job warns about.

Then, on a warm run:

1. **Step duration.** Compare `build test binaries + archive` against the pre-sccache baseline recorded in the audit: **11.90 min (interval)** and **11.03 min (default)**, from run `32425890937`.
2. **`sccache --show-stats`** (a step on both build jobs). This is the part that decides it:
   - **Hits on dependency crates prove nothing.** rust-cache already serves those ~225 crates; a run that only hits there has measured its own redundancy.
   - **Hits on workspace crates are the whole hypothesis.** rust-cache deliberately evicts them (the note on the build job, and render.yml's), and at opt-2 they dominate these two jobs. That is the number worth having.
3. Sanity-check the object cache is persisting at all — a permanently cold `sccache-obj-<lane>-` restore means the trial measured nothing.

## Outcomes

- **Helped** — drop the `vars.SCCACHE` check and make it unconditional; record the measurement in `docs/CI-MINUTES-2026-08.md` under F4 and in `docs/LOCAL-BUILD-PERF.md` next to the local revert, so the two verdicts are legible as the different questions they are.
- **Did not help** — set `SCCACHE=0`, or remove the rig. Either way write the number down under F4: "we tried it and it did not pay" is worth as much here as the reverse, and prevents a third pass at the same idea.

Note the build jobs are ~24 of a code-tier PR's ~87 billed minutes and `build + archive (interval)` alone is ~88% of the run's 13.75 min critical path, so this is the one knob left that could move both cost and latency.

## Home

CI-minutes ground belonged to S-QA, which is closed and may hold only closed items, so this open reminder lands under `work/issues/`.

## The verdict (TCOST-C4, PR 1648)

Read on 2026-09-03, after the trial window on `tcost/c4-sccache-reread`.
The rig had been inert since it landed (`vars.SCCACHE` = `"0"`), so the
first job was to make it run at all.

**It does not pay, and the reason is structural.** sccache 0.16.0 does not
cache `--crate-type bin`; every test binary in the nextest archive is one,
and test targets are 82 % of the build job's compile time. The warm run
(33726782739) shows exactly that: 18 hits, 0 misses, 50 non-cacheable calls
of which 47 are `crate-type`. The 18 are the workspace libs.

Item 3 of this issue also fired: the ~205 MB per-lane object cache restored
at a 9-minute gap and missed at 38 and 60 minutes.

Disposition: rig kept, condition inverted to `vars.SCCACHE == '1'` so it is
off with no variable set. Numbers in `docs/CI-MINUTES-2026-08.md` F4 and
`docs/perf-data/sccache-trial/`.

## Closed (2026-09-04)

PR 1648 merged 2026-09-03 11:31 UTC (`eb7a78a2` into main). The verdict
above is the deliverable and it is on main; F4 of
`docs/CI-MINUTES-2026-08.md` and `docs/perf-data/sccache-trial/` carry
the numbers. Nothing is owed here.

The one finding the trial turned up that this item does NOT close is
finding (d) — `Swatinem/rust-cache` restored nothing on five of seven
build jobs and on the control. That is filed as
`work/tcost/rust-cache-never-restores-across-branches`.
