---
id: five-director-doors-skip-the-underflow-question
kind: issue
title: the five doors that learned the overflow question still miss the underflow twin, and one names a recourse that cannot work
status: open
opened: 2026-09-11
---


(FIX orchestrator) From the `direction-underflow-reports-zero-length`
lane (PR 2359), which was asked whether the five doors PR 2356 fixed
need the underflow gate too, and answered **yes, with evidence per
door**. Filed as its own unit because it is five doors and a signature
change, not one line each.

PR 2356 put the finiteness question in front of five decide-then-
normalize doors, closing the **overflow** end. PR 2359 closed the
**underflow** end at `decide_unit_direction`, which those five do not
go through. So the documented symmetry in `Vec3::normalize`'s own note
is now closed at one door and open at five.

| door | measured | what it says today |
| --- | --- | --- |
| `geom-core::frame::definitely_positive` | **executed**: `mirror_across_plane(origin,(1e-180,0,0),witness)` → `FrameError::Degenerate` | *"declare the coincidence, move the geometry, or **lower the tolerance**"* — **a recourse that cannot work** for an underflowed normal |
| `sweep::AxisFrame::build` | read | `DegenerateAxis`, *"no definite length (zero or sliver)"*, with `COINCIDENCE_RECOURSE` rather than scale |
| `topo::sector_shape` | read | the arm is `min(norm_own, norm_next)`, so as at the overflow end a `min` hides an underflowed chord behind a good one — the question is **per chord** |
| `profile::unit_from_components` | **executed** | `ZeroDirection`, whose sentence already carries the true cause and the right recourse — but see the formatter row below |
| `profile::arc_fillet::carrier_tangent` | **executed** | `DegenerateArcCenter { radius: 0.0 }` — a different arm entirely, reached because the **radius** underflowed first |

**The first row is the one that makes this a defect and not a
tidiness.** A user told to lower the tolerance for an underflowed
normal will lower it, fail again, and have been sent the wrong way —
`memories/refusal-text-is-not-cause.md` at the recourse rather than the
cause, which is the same defect PR 2359 fixed one door over.

## What makes it more than a repeat of PR 2356

`geom_core::is_underflowed_length(len, witness)` needs the **witness**
— the largest `|component|` — because the distinction dies inside
`norm_squared`'s multiplication and cannot be recovered from the norm.
`definitely_positive` takes `length: T` only, so the witness must be
plumbed through its signature from its four call sites. That is a
public signature change on a `geom-core` door, which is why this is a
unit rather than a thread.

The `sector_shape` row wants the question asked **per chord** rather
than on the `min`, exactly as PR 2356's fix pass established for the
overflow end — where `Real::min` propagating `NaN` but not infinity
generated the whole surprising table. Establish the underflow
equivalent before assuming symmetry.

## Fences

`crates/geom-core/src/*` and `crates/geom-brep/*` are **PROPS's**;
`crates/sweep/src/revolve/*` is **BLEND's**; `crates/topo/src/sector_shape.rs`
is unowned with its fence drawn in PR 2356; `crates/profile/*` is
**BOOL's**.
