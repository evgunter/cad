---
id: loud-stand-down-announcements-are-discarded-by-the-gate
kind: issue
title: test_utils::vacuity::stood_down prints to a stdout the twelve gating nextest jobs discard: 22 announcements nobody can hear
status: open
opened: 2026-09-15
refs: [D70]
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
