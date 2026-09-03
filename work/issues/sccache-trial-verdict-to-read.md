---
id: sccache-trial-verdict-to-read
kind: issue
title: sccache on trial - check in a few days whether it actually helped
status: open
opened: 2026-08-21
github: 853
refs: [852]
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
