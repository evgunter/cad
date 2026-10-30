---
id: SHELL-4
kind: unit
title: shell runs the curved wall-clearance gate at certifying scalars — the cavity clone checked embedded before insertion
status: dispatched
opened: 2026-09-04
refs: [shell-curved-clearance-consumer, shell-curved-wall-clearance-window, 1737]
priority: P1
cost: H
branch: clear/shell4
---


Ruled B on `[ev]` #1737 (Ev, 2026-09-04). `shell` / `shell_open` take
a per-scalar lane in `MinClearanceLane`'s shape (`Interval` answers,
implemented in `topo` beside the moved engine; `f64`, `Probe`, `Dual`
answer `None`); at a lane that answers, the cavity clone is checked
strictly positive over its non-adjacent face pairs before
`insert_void` — E7's self-intersection question asked of the cavity —
and a crossing refuses `ShellError::WallClearance`-typed with the
witness pair, a frontier or budget refuses `Escalated`. At `f64` the
module docs keep the window sentence, true. Closes
`shell-curved-wall-clearance-window` on a row where `shell::<Interval>`
refuses the OFF-D dumbbell's curved twin with a witness and holds on
the tube; the pair-filter cost is measured on the tube and vessel
fixtures in the same PR.

## Lane sentence (CLEAR orchestrator, 2026-10-27)

The decided shape (`symbolic-tier-and-clearance-engine`) changes the lane
sentence above. `shell` reads `T::clearance_lane()`:
- `Interval` and `Sym<T>` answer `Some`, so the certifying `Sym` rebuild
  runs the gate the `Interval` rebuild runs;
- `f64`, `Probe` and `Dual` answer `None`.

The wording "over its non-adjacent face pairs" defers to the
self-intersection row's design. Witness verification without a
document is SHELL-3's spec question, above.

## Parked (CLEAR orchestrator, 2026-10-29)

Parked on `departure-reads-no-curved-corner`. CURVEDEP (#5991)
certifies closed curved edges only. Read at `origin/main`, the cavity
clone keeps the operand's topology: `shell.rs` clones the body and
moves its charts in place. So:
- **Tube** (`shell_operands.rs` `tube`): every curved shared edge is a
  closed rim circle, so the gate would hold.
- **Vessel** (`shell_operands.rs` `vessel`, a full revolve that
  touches the axis): each disc meets the cylinder along open
  semicircles (`sweep/src/revolve/full.rs`'s merged π-bands), which
  is a curved corner. Today the gate would refuse a valid vessel
  `Uncertified { Departure }`, a regression `shell::<Interval>` must
  not ship.
- **The OFF-D dumbbell's curved twin**: no such fixture is built yet.
  The OFF-D dumbbell itself is planar
  (`sweep/tests/offd2_r1_probes.rs`). An extruded twin with arc necks
  would have curved corners.

## Unparked (CLEAR orchestrator, 2026-10-30)

The parking note above was wrong about the vessel. It already held at
base, with `cornered = 0`, because the disc's plane implicit, read as
the carrier arm's ψ, certifies its half-rims; CORNER's lane measured
this and both reviews confirmed it. CORNER (#6045) has landed anyway,
so a curved twin with arc necks would also be read at its convex
corners.
