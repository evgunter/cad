---
id: the-gui-has-no-clearance-consumer
kind: issue
title: the GUI has no clearance consumer: no verdict, no witness, no refusal
status: parked
opened: 2026-09-30
priority: P1
cost: M
blocked_on: [a-clearance-assertion-is-decided-by-the-sweep]
---

## What the GUI is missing

A clearance requirement, "these parts clear each other by 2 mm", has
no answer in the viewer. Today it is written as a `min_clearance`
measure plus an assertion over it. That measure has a value only at a
certifying scalar, and the viewer evaluates at `f64`. So its row shows
the kernel's `MeasureUnavailableAt` sentence, the assertion reads
`Unevaluated`, and nothing marks where the parts are too close.

The kernel side is wrong too. `wire_assertion` (`eval/wire.rs`)
compares a bracket that `min_separation` already computed to its full
budget, and it never hands the bound to the search. The predicate
search exists: topo's `Sweep` answers `separation ≥ c`. It discharges
pairs that clear `c`, splits only those that straddle `c`, and stops at
the first verified violation. Yet its only door,
`editor_core::clearance`, spells the question a second time
(`Selection` plus `c`), and nothing outside tests calls it.

## The design (ratified by Ev on PR 6261)

- **The document.** An E10 assertion may bound `clearance(a, b)` with
  length formulas (ERROR-DESIGN E3):
  - from below (`≥ c`);
  - from above (`≤ c`);
  - from both sides (`= c`, or a band `lo ≤ clearance(a, b) ≤ hi`).

  `a` and `b` take the reference seats `min_clearance` has today: a
  body or a face each. No clearance value exists. `min_clearance`
  leaves the measure primitives, so neither a computed minimum nor
  arithmetic over a clearance can be written. Python's constructor
  becomes the assertion's.
- **Evaluation.** At a certifying scalar the assertion resolves its two
  seats and runs `Sweep` at its bounds. The bounds are enclosures
  there, so the sweep's bound changes from an `f64` to an enclosure. Each
  bound is decided with the margin every assertion carries.
  - A lower bound holds when every pair is certified clear of it.
    Otherwise it is `Violated` with a violating pair: two face points
    that the `f64` rebuild (`LaneEnv::nominal_results`, whose build
    condition widens) decides are closer than the bound, together with
    their distance.
  - An upper bound holds when a verified pair lies within it; that
    pair is shown. It is `Violated` when every pair is certified clear
    of it.
  - **Two bounds run as ONE fused sweep** (Ev, PR 6261): each cell pair
    is classified against both bounds in one subdivision, never as two
    searches. `= c` is the band with `lo = hi = c`.

  A typed refusal (sliver, budget, unsupported carrier, …) is the
  verdict's third state, with its recourse. At `f64` the assertion
  reads that third state, naming the certifying lane.
- **What retires.** Over a tolerance box, `drive::assertion_at` is the
  same evaluation per leaf. `editor_core::clearance`'s second spelling
  retires into it. With the measure, these retire too:
  - `MeasureUnavailableAt`
  - `ValuePayload::MeasureUnavailable`
  - `Observed::Unavailable`
  - `UnevaluatedReason::MeasureUnavailable`

  `min_separation` leaves evaluation.
- **The viewer.**
  - An at-rest certify seam with the coincidence door's shape.
    `DocSession::land` submits it when the landed `f64` evaluation
    holds a clearance assertion. It is keyed by generation, cancelled
    by the next submit, and settled into `LandedRun`.
  - The seam returns one verdict per assertion, in plain numbers, from
    a `pncad` door that runs the nominal at `Interval`.
  - The row reads like an equation: `clearance(<a>, <b>) ≥ 2 mm`. It
    then shows `certifying…`, `Holds`, `Violated` (for `≥`: "a
    violating pair 1.84 mm apart"), or the refusal's sentence and
    recourse.
  - A witness pair is drawn as a leg, with its two faces lit, while the
    row is selected or hovered.
  - There is no bracket.
  - The create pane gains "add a clearance requirement": two seats and
    a bound on either side, or both.
- `crates/viewer/README.md`'s clause that a landing is "taken in that
  pair's single `land`" is reworded: a landing may owe answers that
  settle later.

Not part of this row:
- a clearance value door;
- the faces that floor a minimum.

## Seams

- The requirement form, the engine wiring and the retirements:
  editor-core and topo (CLEAR, CLEARTAIL).
- The `pncad` door and `pncad-py`.
- The seam, the rows and the create door: DOORS, shared with VSEAM.
- The pair's mark: `marks.rs` (VGEOM).

CLEARTAIL's
`clearance-violated-verdict-names-its-pair-by-bare-keys` and
`violated-witness-can-sit-off-the-trimmed-face` become the GUI's
witness rows.
