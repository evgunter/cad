# Design fork: a-stored-arcs-end-is-never-checked-against-its-authored-vertex (PR #5449 amends D1's point-scalar clause)

Dispatched 2026-10-20 by the PATHS orchestrator. Blinding byte 114 (parity 0): Opus is A, Fable is B.
Protocol at dispatch: 26db1af89e
Deviations: the implementation (PR #5449) was built before the fork was recognised; designers are told not to read its branch.

## Round 1
- A: guard at the path door, keyed on WITNESS: point scalars read every arc back through validate's table checks (start on carrier, landing, sweep range) plus Via's q; exact scalars unchanged. Likely. Framing rejected: partly (at f64 the facts are not checked even once). Ratified text: y (README mechanism sentences).
- B: same: decide at the door at inexact witnesses, per mode's residual (arc_landing; Via arc_through_point), scene-resolution sibling; exact unchanged. Likely. Framing rejected: partly (register_equal compares with the wrong slack and drops Disputed). Ratified text: y (same sentences; DESIGN [topology] three words).
- Agree on the final state; differ on which readings (full table checks vs per-mode residual). No reconciliation round.
