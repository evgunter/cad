---
id: nightly-all-features-build-hung-once-with-a-test-binary-running
kind: issue
title: The nightly all-features row's build step hung once for two hours with an all-* test binary running under nextest --no-run
status: open
opened: 2026-10-28
priority: P3
cost: E
parent: nightly-jobs-that-do-not-finish-or-read-nothing
---


Filed by `green/gate-trust` (2026-10-28) while it measured the new
`suite under --all-features (default eps)` nightly job through a
temporary push-triggered copy on `green/nightly-selftests-allfeatures`.

## What

Run `40727704860` (job `122955329544`, head `b22a3fac`) is the second
of three runs. Its `build` step printed `Finished … in 31m 06s` at
17:08:51, then printed nothing more until the job's 150-minute timeout
cancelled it at 19:07:21. The runner's cleanup then reported two
orphans: `Terminate orphan process: … (cargo-nextest)` and `… (all-eac264c8dffa2498)`.
So a test binary was executing inside a step that runs
`cargo nextest run … --no-run`, which should build and not execute
anything.

The runs before and after it, on the same command, did not hang:

- Run `40720990736`, with `--all-features`: build 33 min, then the suite.
- Run `40751709384`, with the final feature list and head `48de690a`:
  build 27 min, then 14 008 tests passing in 87 s.

The only change between the hung run and the green one was adding step
timeouts.

## Why it matters, and what is in place

Once is not a pattern, but a silent two-hour hang is the most expensive
way a nightly row can fail. The row now carries `timeout-minutes: 60` on
`build` and `30` on the suite, so a repeat reds within the hour, naming
the step.

## What would settle it

On a repeat, read which binary `all-eac264c8dffa2498` is: `cargo nextest
list --message-format json` maps binary ids to hashes. Then check whether
nextest 0.9.140 executes a test binary under `--no-run`; listing it for
`--list` is the only reason it would. A `--list` that blocks would point
at a static initialiser in that binary.

## Where it stands (2026-11-14, GREEN `green/nightly-jobs`)

**Where: answered.** nextest 0.9.140's `run --no-run` executes every test
binary twice, `--list --format terse` and then with `--ignored`
(`strace -f -e execve`). So the binary running inside the step named
`build` was being listed.

**Why the listing blocked: unknown.** On today's tree nothing blocks:
all 34 test binaries of the all-features build (the tree at `f42cadd6`,
built locally with the nightly's env and feature list) list in 0.2 s
together. The hang has not been seen since 10-28.

**What a repeat now shows.** The `build` step only builds. A step of its
own runs nextest's listing under a 10-minute bound; if that blocks or
fails, `scripts/list-test-binaries.py` lists each binary one at a time,
from its package's directory, and names any that does not finish. It
cannot reproduce nextest's own environment or nextest listing several
binaries at once, and says so when nothing blocks in its own pass.

This row stays open until a repeat is read there, or until enough
nightlies pass without one to call it gone.

The 11-13 overrun of the same step (no log) is
`the-all-features-build-overran-its-hour-on-11-13-for-an-unread-reason`.
