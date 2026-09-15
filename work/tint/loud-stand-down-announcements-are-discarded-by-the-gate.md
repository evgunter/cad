---
id: loud-stand-down-announcements-are-discarded-by-the-gate
kind: unit
title: test_utils::vacuity::stood_down prints to a stdout the twelve gating nextest jobs discard: 22 announcements nobody can hear
status: closed
opened: 2026-09-15
refs: [D70]
closed: 2026-09-15
pr: 2656
branch: tint/2-stand-down-channel
---


(S-TINT orchestrator) Found while gathering context for `D70`, whose
whole question is whether thirteen silent stand-downs should announce
themselves. They should not announce themselves *through this door*
until this row closes, because on the hosted gate the door is shut.

## The defect

The tree's named loud-skip idiom is `test_utils::vacuity::stood_down`,
and its entire body is a `println!`:

```rust
pub fn stood_down(label: &str, what_is_not_asserted: &str) {
    println!("SKIPPED ({label}): {what_is_not_asserted}");
}
```

Its own rustdoc calls it *"the loud stand-down"* and says the row
*"asserts nothing about it and says so by name"*. **On the gate it says
nothing to anybody.** `cargo-nextest` captures a passing test's stdout
and its `--success-output` default is `never`; the twelve `test (…)`
matrix jobs that gate every PR — `.github/workflows/ci.yml`'s archived
`cargo nextest run` invocations on both the default and the interval
archive — pass no `--success-output`, set no `NEXTEST_SUCCESS_OUTPUT`,
and the repository contains no `.config/nextest.toml` (or any other
`nextest.toml`) to set a profile default. So every announcement is
written to a pipe nextest throws away.

**Measured, not inferred.** Under the pinned `cargo-nextest 0.9.140`, a
one-file crate whose only test prints and passes:

```
        PASS [   0.005s] (1/1) nxprobe t::passes_and_prints
     Summary [   0.008s] 1 test run: 1 passed, 0 skipped
```

The printed line does not appear. It appears under
`--success-output immediate`.

**Exactly one job in the workflow passes that flag** —
`cargo nextest run -p viewer --features app --success-output immediate`
— and its own comment says it is there for the smoke row's adapter, not
for stand-downs. Nothing generalises it: the viewer job is the one place
a `stood_down` line could be read, and no `stood_down` call site is in
`crates/viewer/`.

## The population

**22 call sites in 10 files**, none of them under `viewer/`:

`crates/topo/tests/review_m6_2_probes.rs`,
`crates/topo/tests/m6_2_fitted_at_rest.rs`,
`crates/mesh/tests/fitted_refusals.rs`,
`crates/step-import/tests/cert5_r1_import_probes.rs`,
`crates/sweep/tests/torax_interval.rs`,
`crates/geom-brep/tests/r1_pxn_probes.rs`,
`crates/geom-brep/tests/m5_pr7_ssi.rs`,
`crates/geom-brep/tests/review_m5_pr7_adversarial.rs`,
`crates/geom-brep/tests/r2_cert6_probes.rs`,
`crates/geom-brep/tests/review_m5_pr7b_ssi.rs`.

Count by `grep -rn 'stood_down(' crates/ --include=*.rs` less the
definition and its own doc references; a call site inside a helper
called by several rows announces once per row that reaches it, so 22 is
the number of SITES and the number of announcements per run is at least
that. The sweep keys on the function name, so it cannot see a
hand-rolled `println!("SKIPPED …")` that never adopted the door —
`work/tint/loud-skip-marker-is-a-hand-kept-idiom` names eight
`#[cfg]`-gated marker rows of exactly that hand-rolled shape, and their
bodies are `println!` too, so they are in the same pipe.

## Why this is S-TINT's and what it is not

It is the charter's second shape read one level up: *a guard that cannot
go red is not a guard*, and an announcement nobody can hear is the same
defect with the volume at zero. The rows are not wrong and they are not
vacuous — they genuinely stand down for a stated reason. What is broken
is the **channel**, and a channel that delivers nothing reports the same
green as one that delivers.

**It is not a cost row.** Nothing here is argued on a cpu-second or a
billed minute, and `--success-output` changes no test's runtime. But the
FIX may touch files this program does not own, and the fence says so:
`.github/workflows/*` is CIW's and `scripts/ci-filter.py` is S-TCOST's.
Three repairs are available and they are owned differently —

1. **`--success-output` on the gating jobs** (or `NEXTEST_SUCCESS_OUTPUT`,
   or a committed `.config/nextest.toml` profile). This prints EVERY
   passing test's output, not just stand-downs, so it is a decision about
   log volume as well as a repair. The workflow half is CIW's; a
   `.config/nextest.toml` is not obviously anyone's yet.
2. **Make `stood_down` write somewhere nextest does not capture** — a
   file, or stderr (nextest captures that too, so this does not work as
   stated; recorded so a taker does not re-try it).
3. **Make the stand-down a fact the run can assert on** rather than a
   line it prints: a tallied exposure the suite floors, which is the
   shape `vacuity::Exposure` already has in the same module. This one is
   entirely inside S-TINT's fence and needs nobody's workflow.

The third is the only one that turns an announcement into something that
can go red, which is this program's whole question, so a taker starts
there and treats 1 as the cheap partial. **Nothing is decided here** —
this row states the defect and its population.

## What it blocks

`D70` cannot be answered as *"route the thirteen through the tree's loud
stand-down door"* until this closes: that answer would add 13 more
announcements to the 22 already being discarded and change nothing
observable about the gate. Recorded on `D70` as well.

## Closed — TINT-2, PR #2656 (2026-09-15)

**No guard landed, and that is the honest outcome rather than a
shortfall.** Nothing in this unit can go red. What it delivered is that
the tree stops claiming otherwise, and that the nine marker sites have
one spelling instead of nine.

**The spec's preferred repair did not exist.** It leaned to *"make a
stand-down a fact the suite can floor"* using `vacuity::Exposure`. The
lane refused, and the review verified the reason behaviourally rather
than from prose: a two-row probe under the pinned `cargo-nextest 0.9.140`
reports **pids 12156 and 12157** with a `static AtomicUsize` reading 0 in
both. **nextest is process-per-test**, so nothing a `stood_down` call
records is readable by any other row. Option (A) had no mechanism inside
this fence and was never available.

One refinement the review added, which the item keeps because the PR
body first got it wrong: a row **can** floor a dynamically counted
stand-down condition, and this tree does — `crates/geom-brep/tests/m5_pr7_ssi.rs`'s
fit-budget arms assert the budget is genuinely overrun before they
announce. What a row cannot do is floor a **sibling's** stand-down. So
what is left for the 22 `stood_down` sites is 22 per-row posture
decisions, which `D70` records as **C21's** question — whether a row
should stand down at all comes before any floor — and not this unit's.

**What landed.** `test_utils::loud_skip_marker!` is one home for all nine
in-fence markers. Its shape is the point: the feature literal reaches
both the `#[cfg(not(feature = …))]` and the printed sentence **from the
same token**, so they cannot disagree, and `file!()` supplies the
filename mechanically. Nine hand-kept rustdoc paragraphs and the
hand-typed `mod certified` string that a `git mv` would have desynced in
six files are gone with it.

**Two marker NAMES were hand-kept enumerations too**, and the name is the
half that actually reaches the gate — nextest prints it in the PASS
list. `app_lane_skipped_startup_error_arms_not_checked_here` →
`…_no_error_display_coverage_here` and
`app_lane_skipped_parameter_field_units_not_checked_here` →
`…_no_panel_display_coverage_here`. Verified that every external
consumer keys on the `app_lane_skipped_*` PREFIX
(`.github/workflows/ci.yml`, `local-scripts/ci-local.sh`,
`crates/viewer/GUI-DESIGN.md`) and that
`scripts/check-interval-cfg-additive.py` and
`scripts/interval-only-selection.py` key on the unchanged interval name,
so no other program's filter broke.

**What is still unenforced, stated here because the unit states it at
its own sites too:** a marker whose file gains or loses a gated row still
says nothing; a `stood_down` in a configuration where the mode was
reachable is still green and indistinguishable from one that was not;
all 22 announcements are still discarded on every gating run. The macro
itself can be wrong and pass in two ways — a `feature` naming something
no `#[cfg]` uses, and a bad `absent` argument. Closing any of that needs
a stdout reader on the gating jobs (CIW's, and a log-volume decision) or
a per-row floor (C21's).

## Residues, each with its own file

- `work/ciw/gating-nextest-jobs-discard-every-passing-tests-stdout` —
  the `--success-output` half. Filed with the measurement and no
  hand-kept count.
- `work/ciw/loud-skip-marker-text-is-unchecked-against-its-own-file` —
  the cheap guard that would have caught this unit's own first push.
  **Routing corrected by the lane**: `scripts/check-*.py` is CIW's, not
  S-TCOST's, which holds only `ci-filter.py`, `slowest-tests.py` and
  `base-test-listing.sh`.
- `work/view/viewer-lib-marker-claims-the-log-carries-its-sentence` —
  `crates/viewer/src/lib.rs` is `src/` and out of fence.
- `work/tint/phantom-turn-row-stands-down-outside-the-tree-s-door` —
  this slate; a row announcing through a bare `println!` outside the
  door, the counterexample that forced `vacuity.rs`'s universal to be
  retracted.
