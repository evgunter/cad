---
id: the-gui-has-no-clearance-consumer
kind: issue
title: the GUI has no clearance consumer: no verdict, no witness, no refusal
status: parked
opened: 2026-09-30
priority: P1
cost: H
design: true
parent: the-gui-shows-no-measure-value-and-no-clearance
blocked_on: [clearance-refusal-names-one-face-twice-across-bodies]
---


Tier 3 of `the-gui-shows-no-measure-value-and-no-clearance`, split out
by AUTH-7, which shipped tiers 1 and 2. A measure's row now shows its
value, or for `min_clearance` at `f64` the kernel's
`MeasureUnavailableAt` sentence naming `clearance::min_separation` as
the door that answers. Nothing in `crates/viewer` calls that door.

What a consumer would show, and what it would have to decide first:

* **The verdict and its witness.** The engine answers with a certified
  enclosure (`crates/editor-core/src/clearance.rs`) and a `Holds`
  verdict that carries its certificate. It also has eleven typed
  refusal arms (`ClearanceRefusal`), and none of them has a `Display`
  yet. CLEAR's `clearance-refusal-names-one-face-twice-across-bodies`
  covers that, and a consumer that shows the refusals waits on it.
* **Which scalar the app evaluates at.** The viewer is an `f64` build
  (`Evaluation<f64>` throughout `session.rs`). An enclosure needs the
  interval lane over a parameter box, so this is a second evaluation
  on a second seam, not a panel change. It raises the staleness
  question `DocSession::land` answers for the first seam, and raises
  it again for the second.
* **Where it appears.** The measure row's `Reading` is the obvious
  anchor. A witness point belongs in the viewport, which is
  `pane/viewport.rs` ground (shared with VGEOM and VSEAM).

Seams: CLEAR owns the engine (`crates/editor-core/src/clearance.rs`)
and its refusals' words. The session seam is shared with CHROME,
VSEAM and VGEOM (`work/author/program.md` keep_out). Weigh it with
designers before a lane builds it.
