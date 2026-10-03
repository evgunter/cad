---
id: phantom-turn-row-stands-down-outside-the-tree-s-door
kind: issue
title: m10_5_r1 phantom-turn row asserts nothing in three of four arms and announces through a bare println!, outside the stand-down door
status: open
opened: 2026-09-15
priority: P3
cost: E
---



Found by TINT-2's sweep for the stand-down population. Not taken there:
converting it would route one more announcement into a pipe the gate
discards and change nothing observable, which is the shape
`loud-stand-down-announcements-are-discarded-by-the-gate` exists to
name; and whether the row should stand down at all is the C21 question
`D70` settled for its own population and not for this one.

## The finding

`crates/editor-core/tests/m10_5_r1_probes_interval.rs`,
`fn a_partial_revolve_band_reports_its_phantom_turn`. Its own rustdoc
says *"Loud skip if the revolve does not build at the interval scalar
over an ε-box."* The row asserts a receipt holds, then matches on the
verdict:

```rust
match report.verdict() {
    ClearanceVerdict::Refused(ClearanceRefusal::Selection(s)) => {
        println!("[r1] interval_lane_skipped_revolve_did_not_build: {s}");
    }
    ClearanceVerdict::Refused(ClearanceRefusal::Unsupported { carrier, face }) => {
        println!("[r1] interval_lane_skipped_unsupported: {carrier} at {face:?}");
    }
    ClearanceVerdict::Violated(v) => { /* the row's actual claim */ }
    other => println!("[r1] quarter annulus answered {other:?}"),
}
```

**Three of the four arms assert nothing about the row's subject** — the
phantom turn — and announce through a bare `println!`. Only the
`Violated` arm makes the claim the row is named for. So a run in which
the revolve stops building, or the carrier stops being supported, or the
verdict becomes anything else at all, is the same green as a run that
proved the phantom turn, and the only difference is a line the gate
throws away.

Two separate defects, and the second is the one that matters:

1. **It is outside the tree's door.** `test_utils::vacuity`'s module docs
   claimed every in-row stand-down in `crates/` goes through
   `stood_down`. This is the counterexample; the claim is retracted in
   `vacuity.rs` by the same unit that found it. The sweep behind that
   claim matched `SKIPPED (` / `SKIPPED:`, and these three lines say
   `interval_lane_skipped_…` instead — the blind spot the docs warned
   about, occupied.
2. **The `other =>` arm is a catch-all stand-down with no stated
   reason.** The two named arms at least say which refusal happened. The
   fourth prints the verdict and returns green over a row whose whole
   subject is unexamined, and no sentence anywhere says what that green
   means. That is the `stood_down` rustdoc's own complaint — *"a
   stand-down that says only 'skipped' leaves the reader to work out
   what the green meant"* — one level worse, since it does not even say
   skipped.

## What a taker decides

Not "route it through `stood_down`": that is a channel with no gate
reader (`work/ciw/gating-nextest-jobs-discard-every-passing-tests-stdout`).
The question is C21's — whether the row is entitled to stand down on
these three verdicts at all, or whether some of them are outcomes it
should red on. `ClearanceRefusal::Unsupported` for a carrier this
fixture builds on purpose looks like a red, not a skip; `Selection` at
an ε-box may be a genuine stand-down. `D70`'s finding is the precedent
for how that gets argued.
