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

#5229 enforced the fast-set time bars (0.5 s for a new test, 1 s as the backstop) and moved 566 tests into the slow set. The slow set now has 1117 entries. The slow-set step runs only the slow tests a diff's added lines spell (`scripts/slow_set.py`'s `is_new`): a seeded crate's other slow tests do not run, and a dependent crate in the diff's closure runs its fast set only. So a moved test in sweep no longer runs on a topo-only PR; the nightly runs it. Topo changes that red only sweep rows have happened (`a-sweep-diff-gates-no-eps-row-of-sweep`, PR 5192).

## The policy (Ev, 2026-11-12, PR 6731)

1. **The nightly attributes each newly red slow test to its merge.** A slow test red on tonight's nightly and green on the previous one is bisected over the first-parent merges between the two nightly heads. It runs alone, at the nightly's build profile, and the bisect names the first red merge. At most five tests are bisected a night; the rest are listed, unattributed, in the run's summary.
2. **What the attribution writes**, as a tracker-only commit to main (as the STATUS render commits):
   - **Where.** If a row under `work/` already names the test's id, it appends a dated `## Nightly attribution` section to that row. Otherwise it writes one row, `work/<program>/main-red-<test-leaf>.md`. `<program>` is the first red merge's head-branch prefix where that directory exists, and `green` otherwise.
   - **The row.** `kind: issue`, `priority: P1`, `cost: E`, `status: open`, titled `main red: <binary> <test> since PR <n>`. Its body states:
     - the test id;
     - the first red merge (PR number, merge sha, head branch) and the last green one;
     - the nightly run and its ε row;
     - the failure's first panic line, verbatim;
     - each probe as `sha: pass | fail`;
     - the line "Found by the nightly attribution; not triaged."
   - It decides nothing about the red's cause, and edits no other row.
3. **A PR does not run its seeded crates' whole slow sets.** That costs about 5.5 min (topo) to 26 min (sweep) of wall time on the gate, on every PR that seeds one of those crates, and Ev ruled that latency out.
4. **Open: a trimmed, much faster per-PR set** that still catches most of these reds. Ev is open to one; it is not designed yet (`trimmed-slow-canary-for-a-seeded-crate`).

This row stays open until the nightly attribution runs.

The measurements behind this are in `log.md`, 2026-11-10.

## Ev's ruling (2026-11-12, PR 6731)

"(3) is good, i think--it doesn't harm latency except if it causes queuing, and even if it does, 40 minutes is a pretty reasonable cost. (4) is rough. i think that level of latency hit on every PR in those areas is probably not acceptable. i'd be quite open to a a trimmed-down, much faster set of tests which would still catch most of these issues, though!"

(3) is the nightly attribution, items 1–2 above; (4) was every seeded crate's slow set on its PR, item 3.

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

Two more on the plate ledger, found while re-pinning PR 6663: PR 6639 (`bc04d87117`) and PR 6631 (`296779102b`), both BOXES cover changes, both undisclosed (6639's body says the plates are unchanged, having read only the largest-form bound). Recorded on `work/boxes/cover-readers-moved-the-plates-sym-ledger-undisclosed.md`.

## Count (2026-11-11, `measuredoor/sliver-bracket-close`)

Two more, on `step-export`'s slow-set `kernel_sidecars::kernel_sidecar_fields_match_live_kernel` (`cut_cylinder.expect`'s `KERNEL_VOLUME_PAD_MM3`). Each is a geom-brep and topo props PR whose closure reached step-export, and whose seeds did not run the row. Both were bisected first-parent over main:
- PR 6478 (`898df16c9c`): the pad moved from `1022.3704888604841` to `1022.3704820511159`, and the midpoint by 9.5e-7 mm³;
- PR 6509 (`e2435632d8`): the pad moved to `1022.370482199146`, and the midpoint moved back.

Both moves are right (`work/measuredoor/main-red-reach-volume-backstop-rows-after-the-exact-sliver-bracket.md`, `## Closed`). The same row was moved by PR 5734 on 2026-10-27, and that move was also first seen by the nightly.
