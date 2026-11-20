---
id: the-nightlys-shared-build-job-dies-to-a-runner-shutdown-most-nights
kind: issue
title: The nightly's shared build job is killed by a runner shutdown on three nights of four, so no full-suite leg runs and main's slow set goes unread
status: open
opened: 2026-11-19
priority: P1
cost: M
---


Filed by the SCALAR orchestrator, 2026-11-19, while reading the
nightlies to close two stale `main-red` rows.

## What

`nightly.yml`'s `full suite, build once (archive; doc-tests, slow-set
names, topo scalpel, wasm32)` job ends `##[error]The runner has
received a shutdown signal` / `exit code 143` partway through its
compile, and every `full suite (eps …)` leg that needs its archive is
then skipped. Of the last four finished nightlies:

| nightly | main | build job | full-suite legs |
|---|---|---|---|
| 11-16, run 43810139969 | `1ca97d1d30` | killed (GREEN's log: at 39 min, compiling pncad-py) | none ran |
| 11-17, run 44023697569 | `cdee746161` | ran | ran at all three ε |
| 11-18, run 44243918484 | `db86602a55` | killed, exit 143 | none ran |
| 11-19, run 44464826302 | `8c12a73427` | killed about 27 min in, 13 min after `Compiling viewer` | none ran |

So main's slow set and its 1e-6 and 1e-12 rows were read once in four
nights. A red that lands on one of those rows is found a few nights
late, and a `main-red` row cannot be closed against a night that did
not run.

## What is not known

Why the runner is shut down. The job dies inside the build, not at its
time limit, and the log's last lines are compile lines for the largest
test binaries (viewer, pncad-py), which is what running out of memory
or disk on a hosted runner looks like; neither was measured. The job's
runner log and the step's memory and disk at the kill are the
instrument.

P1 because the nightly is the only reader of most of the suite
(`docs/prompts/implementer-discipline.md`, Verification): while it
dies, a green PR gate is the only evidence main has.
