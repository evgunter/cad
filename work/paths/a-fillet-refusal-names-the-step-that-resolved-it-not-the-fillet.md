---
id: a-fillet-refusal-names-the-step-that-resolved-it-not-the-fillet
kind: issue
title: A fillet that cannot be resolved is reported at the step that resolved it, not at the fillet's own step, though the chain already records the fillet's step
status: open
opened: 2026-09-24
---


Reported by Ev from the running viewer, 2026-09-24. A profile whose
**only fillet is in step 3** was refused with:

> loop 0 step 11: a radius-0.03 m fillet through a turn of 1.5707963268
> rad needs a tangency the scene cannot resolve: lengths of about 0.21 m
> resolve only to about 4.6629367e-17 m ('carrier_circles_internal'
> classifies it at 0.0467501796 m). Recourse: use a smaller radius (a
> LARGER one makes this worse), place the geometry nearer the origin, or
> drop the fillet and leave the corner sharp

The refusal is about the fillet, and it names a step eight away from it.
Every recourse it offers — a smaller radius, dropping the fillet — is
an edit to step 3, which the sentence never mentions.

## Why it says step 11

A fillet is not resolved at its own step. `fillet(r)` opens a
**pending** side (`Core::pending`, set only by `fillet_kernel`), and the
arc is only computed when the *arriving* carrier is known — in
`crates/profile/src/path.rs`'s fillet resolution, which takes the
pending side (`take_pending`) and calls `line_line_fillet_trims`, whose
error goes through `map_fillet_err`. That can be many steps later:
every non-carrier step in between (`At`, `Angle`, `Turn`, …) leaves the
fillet pending.

`crates/profile/src/path/program.rs`'s replay loop then stamps whatever
the current step returns with the **current** index —
`apply(tip, *step, tol, &mut guide).map_err(|kind| ReplayError { step:
i, kind })` — so a fillet that fails at resolution is charged to the
arrival step. The viewer passes that through untouched: `sketch.rs`'s
`refusal()` copies `error.step` into `PreviewError::Geometry`, whose
`Display` writes `loop {loop_} step {step}: …`.

## The chain already knows the right answer

`PendingMeta` in `crates/profile/src/path.rs`, held beside the pending
side, carries `bound_at`, documented as:

> **The authored step this fillet was BOUND at**, in program order. The
> radius is that step's argument wherever the arc is finally emitted: a
> `fillet(r)` binds on one step and its arc is emitted by the arrival
> step, and the emission record names the binder, because that is where
> the radius a reader would edit lives.

So the success path already follows the rule this bug needs — the
record names the binder, because that is the step a reader edits — and
the failure path does not. `meta.bound_at` is in scope at the very site
that raises the refusal (it is taken with the pending side, one line
above the `map_fillet_err` call), and is dropped.

## What a fix has to decide

- **Which step a fillet refusal names.** The module's own argument says
  the binder. The arrival step is not wrong information — it is where
  the fillet met the carrier it could not reach — so the fix may want
  both: *the fillet bound at step 3 cannot reach the carrier at step
  11*. But a sentence naming only step 11 while every recourse edits
  step 3 is the case to end.
- **Where the index is carried.** `ReplayError` has one `step: usize`,
  stamped by the replay loop. Either the fillet's `PathError` carries
  its binder and the loop does not overwrite it, or `ReplayError` grows
  a second index. That is this program's call; the viewer only renders
  what arrives.
- **Whether the class is wider than fillets.** Anything resolved at a
  later step than it was authored — a pending anything, a closer — will
  be charged to the resolving step by the same `step: i` stamp. The
  close already reports `step: steps.len()`, one past the last step,
  which is honest for a close; check that nothing else rides it.

## Checked before filing

- **Tracker pass:** no row on PATHS or elsewhere covers fillet refusal
  step attribution (grepped `work/` for the shape — resolving step,
  bound step, wrong step — not only for this message).
- The step Ev counts as 3 and the `step 11` in the message may differ
  by an indexing base as well as by the defect; that the gap is eight
  steps, not one, says the defect is the attribution. Worth confirming
  against the reproducing profile.

## Noticed, not filed separately

The same sentence names an internal function to a reader —
`'carrier_circles_internal' classifies it at 0.0467501796 m` — and
gives the turn as `1.5707963268 rad` rather than as the 90° a reader
authored. Both are the wording of `map_fillet_err`'s refusal and belong
with whoever next edits it; neither is this row's defect.

## Home

`crates/profile` is PATHS's. The viewer side (`sketch.rs`'s `refusal()`)
needs no change if the index arriving from `ReplayError` is right.
