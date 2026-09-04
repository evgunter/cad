---
id: opt-level-selftest-runs-nowhere
kind: issue
title: opt-level-calibrate.py --selftest is invoked by nothing in the tree - a guard that has never been shown to fire
status: open
opened: 2026-09-04
---


Found by CIW unit 5 (PR 1722), which extended that selftest and had to
check where it runs in order to claim the extension was verified.

## The finding

`scripts/opt-level-calibrate.py --selftest` exists, is substantial, and
**is invoked by nothing**. Every reference to the script in the tree is
one of its three real modes:

* `.github/workflows/nightly.yml:1023` — `read-free-arm`
* `.github/workflows/nightly.yml:1036` — `decide`
* `.github/workflows/nightly.yml:1243` — `record`

plus prose mentions in `local-scripts/test-fast.sh:39`,
`.github/workflows/nightly.yml:852`/`:1795` and
`scripts/check-ci-mirror-parity.py:155`. No workflow, no gate, no
`local-scripts/` row and no `test-fast.sh` row runs `--selftest`.

## Why nothing catches it

`scripts/gates/gate-roster.sh` is the check that would: it refuses a
hosted half that runs a gate "without running its `--selftest` first — a
guard that has never been shown to fire is not a guard" (`:165`, `:221`,
`:283`). Its scope is `scripts/gates/*` plus a named outlier list, and
`scripts/opt-level-calibrate.py` is in neither. So the rule exists, the
sentence that names this exact defect exists, and the file it applies to
sits outside its reach.

The sibling sets the precedent in the other direction. The criterion lane
runs `scripts/criterion-emit.py --selftest` immediately before the real
invocation (`.github/workflows/nightly.yml:1804`), and the comment above
it cites *this very script* as the precedent for siting such rows in the
nightly rather than the merge gate — while this script does not have the
row it is being cited for.

## What is at stake, and it is not hypothetical

The selftest is the only thing that exercises the parts of this script a
malformed sample would come from: shard summing, the docs-tier skip, a
cancelled shard, a renamed step, the schema-1/2/3 readers, the cadence
triggers, the argmin over three arms, the one-arm refusal, and (as of PR
1722) the environment block's host-identity degradation. The `record`
mode it guards **appends to an append-only history** — `docs/perf-data/
opt-level/` — where a malformed entry cannot be overwritten out. That is
the same failure the criterion comment names as its own reason for
running its selftest first.

## Same class as

`work/ciw/nightly-demotions-have-never-run` — a check that exists and
runs nowhere, so its greenness is a statement about nothing. That item is
about rows that were demoted and never observed; this one is about a
guard that was written and never invoked. Both are "the tree believes it
is covered here and is not".

## Not fixed in unit 5, deliberately

Wiring it is an edit to `.github/workflows/nightly.yml`'s `opt-level`
job, and unit 5's brief fenced that file off — sibling lanes are in it.
Which row it belongs on is also a real question rather than an obvious
one: before `read-free-arm` (earliest, cheapest, fails before any minutes
are spent) or immediately before `record` (closest to the append it
protects, matching the criterion lane's spelling). Whoever takes it
should also decide whether `gate-roster.sh`'s outlier list is the right
home for the general rule, so the next script in this position is caught
by a check rather than by a lane that happened to look.
