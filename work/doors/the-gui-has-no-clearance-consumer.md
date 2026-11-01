---
id: the-gui-has-no-clearance-consumer
kind: issue
title: the GUI has no clearance consumer: no verdict, no witness, no refusal
status: open
opened: 2026-09-30
priority: P1
cost: H
design: true
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

## The design

- **The document.** A clearance is a requirement, `Clears { a, b, by
  }` (ERROR-DESIGN E3), and not a measure.
  - `a` and `b` take the reference seats `min_clearance` has today, a
    body or a face each; `by` is a length formula.
  - It has no relation field and defines no variable, so a computed
    minimum, arithmetic over a clearance, and `=` or `≤` cannot be
    written.
  - `min_clearance` leaves the measure primitives. Python's
    constructor becomes the requirement's.
- **Evaluation.** At a certifying scalar the node resolves its two seats
  and runs `Sweep` at `by`. `by` is an enclosure there, so the sweep's
  bound changes from `f64` to an enclosure: `Holds` is decided against
  its upper end and a violation against its lower end. Each verdict
  is one of:
  - `Holds`;
  - `Violated` with a violating pair: two face points that the `f64`
    rebuild (`LaneEnv::nominal_results`, whose build condition widens)
    decides are closer than `by`, with their distance;
  - a typed refusal, which is the verdict's third state (sliver, budget,
    unsupported carrier, …) and carries its recourse.

  At `f64` the node reads its third state, naming the certifying
  lane, and never poisons.
- **Retiring.** Over a tolerance box, `drive::assertion_at` is the same
  evaluation per leaf, and `editor_core::clearance`'s second spelling
  retires into it. With the measure, these retire too:
  - `MeasureUnavailableAt`;
  - `ValuePayload::MeasureUnavailable`;
  - `Observed::Unavailable`;
  - `UnevaluatedReason::MeasureUnavailable`.

  `min_separation` leaves evaluation.
- **The viewer.** An at-rest certify seam has the coincidence door's
  shape. `DocSession::land` submits it when the landed `f64` evaluation
  holds a `Clears` node. It is keyed by generation, cancelled by the
  next submit, and settled into `LandedRun`. It returns one verdict
  per node, in plain numbers, from a `pncad` door that runs the nominal
  at `Interval`. The requirement's row reads:
  - `certifying…` while the seam runs;
  - `Holds: clears 2 mm`, with no other number, because none is
    computed;
  - `Violated: a violating pair 1.84 mm apart (needs ≥ 2 mm)`, spelled
    so rounding never reaches the bound. While the row is selected or
    hovered, the viewport draws the pair as a leg with its two faces
    lit.
  - the refusal's sentence and recourse.

  There is no bare clearance row and no bracket. The create pane gains
  "add a clearance requirement": two seats and a bound.
- `crates/viewer/README.md`'s clause that a landing is "taken in that
  pair's single `land`" is reworded: a landing may owe answers that
  settle later.

Not part of this row:
- a "within" requirement (`≤`), which is the same sweep read inverted
  and is additive;
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
