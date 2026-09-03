---
id: mirror-parity-never-compares-flags
kind: issue
title: check-ci-mirror-parity compares which checks each half names, never their flags, so the two halves can drift on what a red run reports
status: open
opened: 2026-08-30
github: 1295
refs: [1128, 1232]
---

## From GitHub issue 1295

opened 2026-08-30, 0 comments.

## What

`scripts/check-ci-mirror-parity.py` is the guard that keeps
`.github/workflows/ci.yml` and `local-scripts/ci-local.sh` running the same
checks. Its subject is the **roster**: which checks each half names, which
gate modes it runs, that every hosted job is either cited by the local half or
says why it has no local half, that every tier-blind check sits where nothing
can skip it.

It never reads the **flags on the commands**. Two rows can be paired,
identically named, both green — and be running the same test binary under
different reporting semantics, with the parity check reporting OK.

## Measured, on this repo, during S-QA unit QA-2

Adding `--no-fail-fast` to hosted CI's two sharded nextest rows (issue 1128)
made the halves disagree about what a red run says: hosted would report a
shard's whole failure surface while `ci-local.sh` — the half that runs every
point of the matrix on one tree, the one reached for before a merge that would
be expensive to get wrong — still reported one failure per row.

`python3 scripts/check-ci-mirror-parity.py` returned OK across that state. The
divergence was found by hand, and only because the unit happened to be about
fail-fast.

(Both halves now pass the flag, so the specific divergence is closed. The
blind spot that let it exist silently is not.)

## Why it is worth more than a comment

The parity checker exists because two hand-maintained enumerations drifted and
nothing noticed — that is the failure it was built for, and this is the same
failure one level down. A roster check answers "does the local half run this
check?"; it cannot answer "does it run it the same way?". The flags that
change what a run *means* rather than what it *executes* are exactly the ones
a reader will not diff by eye:

- `--no-fail-fast` — whether a red row reports its surface or its first row.
- `--test-threads` / `-j` — process-parallelism, which also moves how much of
  a red run survives.
- `--partition` — hosted shards, local does not; a declared asymmetry today,
  but declared in prose only.
- `--features`, `--all-targets`, `--profile`, `-E` filtersets — a paired row
  whose selection quietly narrowed on one side runs fewer tests under the same
  name.

It is also an **absence detector**, which the file's own reasoning says must
not be sampled and must be sited where it can fire: a flag that drops off one
half leaves no future red for anything to catch. It merges once, silently.

## Sketch of a fix

Not obviously a full argv comparison — the two halves legitimately differ
(archives and `--workspace-remap` hosted, a shared `target/` locally;
`--partition` hosted only), and the file already has a vocabulary for declared
asymmetries (`MIRROR_EXEMPT`, and the per-key sentences that say why a job has
no local half).

The cheap version is an **allowlist over a small set of semantics-bearing
flags**: for each mirrored pair, extract that set from both sides and require
them equal unless the pair carries a declared exemption naming the flag and
the reason. That keeps the existing "declare your asymmetry in a sentence"
shape rather than adding a second mechanism.

## Provenance

Found by S-QA unit QA-2 (PR 1232) while adding `--no-fail-fast` to both CI
halves for issue 1128; filed at the orchestrator's request rather than left as
a comment in `ci-local.sh`.

## Home

`work/issues/` — `scripts/check-ci-mirror-parity.py` and `local-scripts/ci-local.sh` are S-QA territory and S-QA is closed.
