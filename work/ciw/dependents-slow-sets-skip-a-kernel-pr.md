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

## Count (2026-11-06, `onecopy/main-reds-k-and-m10`, PR 6560)

Nightly reds traced to a moved test that a PR's closure reached and its seeds did not, bisected first-parent over main:
- PR 6504 (`143d99ec29`) and PR 6527 (`3fef8d7ade`): geom-brep pcurve-incidence changes that moved `editor-core`'s slow-set `m10_sym_profile_interval` plate ledger (two rows).
- PR 6562 (`5ff0801009`, `onecopy/feet-and-params`): moved the same plate ledger again (+2 forms on three lines), the day PR 6560 re-pinned it.

Two more counts beside them that are not this trigger's shape, because the moved rows are `probe`-gated and no PR runs them at all, fast or slow: `sweep`'s `ray_wall_margin_twins` (PRs 5781, 6349, 6482, 6491, 6559) and `editor-core`'s `m4_pr8_k_probe` corpus dump (PRs 6341, 6376).

Three in this trigger's shape over 2026-11-04 to 11-06 is a count, not a revisit; it is recorded here for whoever triages next.

## Count (2026-11-09, PR 6663)

Four more on the same plate ledger (`m10_sym_profile_interval::the_forms_the_plates_walks_build_are_pinned`), each a PR whose closure reached editor-core and whose seeds did not run the row: PR 6568 (`324300a2a1`), PR 6594 (`38099d0e88`), PR 6588 (`6bbebd7e31`), PR 6628 (`eae91f8825`). 6628 disclosed its move; the other three did not, and 6588's body said the row passed.

One more from PR 6663's review: PR 6594 (`38099d0e88`) also moved `m10_9_pins_interval::m10_9_no_registrant_lies_on_any_measured_document` (`r2_rounded_pad` 1660 → 1684 symbolic), undisclosed. Filed as `work/onecopy/main-red-the-rounded-pad-reads-24-more-symbolic-decisions-since-planar-foot.md`.
