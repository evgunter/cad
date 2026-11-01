---
id: dependents-slow-sets-skip-a-kernel-pr
kind: issue
title: a PR runs only its own crates' slow sets, so the 566 tests #5229 moved skip a PR that reaches their crate only as a dependent
status: open
opened: 2026-10-16
priority: P3
cost: M
---


## What

#5229 enforced the fast-set time bars (0.5 s for a new test, 1 s as the backstop) and moved 566 tests into the slow set. The slow set now has 1117 entries. The slow-set step runs only the slow tests of the crates a diff SEEDS. A dependent crate in the diff's closure runs its fast set only. So a moved test in sweep no longer runs on a topo-only PR; the nightly runs it. Topo changes that red only sweep rows have happened (`a-sweep-diff-gates-no-eps-row-of-sweep`, PR 5192).

## Ev's ruling (2026-10-16, chat)

Option (a) was taken: merge as is. "we can revisit this if too many bugs are getting through."

## The revisit, if it comes

Option (b): the slow-set step also runs the slow sets of the closure's dependent crates, or of a chosen subset, e.g. sweep for a topo seed. Measure its cost on a kernel PR first. The trigger is nightly reds traced to a moved test that a PR's closure reached and its seeds did not. Count them when triaging the nightly.
