# SHELL-4: shell checks its cavity embedded at certifying scalars

Unit: `work/clear/SHELL-4.md` (P1/H). It closes
`work/clear/shell-curved-wall-clearance-window.md`. Program: CLEAR.
Branch: `clear/shell4`. Review: a single FULL review. The design was
ruled by Ev on `[ev]` #1737. The unit wires an existing, reviewed
certificate into one verb, so its risk is placement, typing and cost,
which one reviewer with claims to falsify covers.

Read the row in full, both its ruling and its *Lane sentence*. Do not
re-open the ruling.

## Context

**What `topo::shell` does today.** It builds a cavity clone of the body
and runs a closed-form PLANAR wall-clearance gate (`wall_clearance` in
`crates/topo/src/shell.rs`). Curved walls closer than `2t` shell
silently into a self-intersecting cavity: tier 3 validates it, and the
volume is wrong. Two measured instances:
- the planar OFF-D dumbbell, which the planar gate refuses;
- a 4×4×4 box with a cylindrical void (axis `(x, z) = (3.2, 2)` along
  `y`, `r = 0.65`, `t = 0.1`), which `shell` returns as two
  interpenetrating solids (the curved-residue row, "Measured instance").

**What it can use now.**
- `topo::injective` proves f : M → ℝ³ injective (`docs/ERROR-DESIGN.md`,
  the *Self-intersection scope* clause).
- It reaches touching pairs across straight, iso, closed curved
  (CURVEDEP, #5991) and convex curved-corner (CORNER, #6045) shared
  edges, and refuses typed elsewhere.
- `AtRestPolicy::clearance_lane()` is the door value: `Some` at
  `Interval` and `Sym<Interval>`, `None` at `f64`, `Probe` and `Dual`.

## What is true when this lands

1. **The gate.** At a lane where `T::clearance_lane()` answers, `shell`
   and `shell_open` run the self-intersection check (E7's injectivity
   question) on the cavity clone before `insert_void`.
   - A verified crossing refuses `ShellError::WallClearance`, typed,
     carrying the witness pair.
   - A refusal of the check (`Uncertified`, budget, frontier) refuses
     `ShellError::Escalated` typed, carrying the engine's refusal.
   - It never answers Ok on a cavity it could not certify.
   - The existing `WallClearance { gap, needed, .. }` and
     `Escalated { source }` arms may not carry a witness pair or an
     engine refusal. Add arms, or extend these, so that the refusal
     states what was found. Say which, and why, in the PR body.
   - Where no lane answers (`f64`, `Probe`, `Dual`), behaviour is
     unchanged, and the module docs keep the window sentence, which is
     true at those scalars.
2. **The planar gate.** Keep it, as the fast first refusal, or retire
   it if the new gate subsumes it. Decide by measuring: report the cost
   both ways on the planar dumbbell and on the tube.
   - Retiring it at `f64` would reopen the planar window at `f64`, so
     at `f64` it stays.
3. **Rows. Each must be able to go red.**
   - **The OFF-D dumbbell's curved twin.** Build it: an extruded
     profile whose neck is arcs (`bulge_loop`), shelled at a `t` whose
     walls cross. At `Interval` it refuses `WallClearance` with a
     witness.
   - **The box with a cylindrical void** (the measured instance above):
     it refuses `WallClearance` at `Interval`.
   - **The tube** (`shell_operands.rs` `tube`) and **the vessel**
     (`vessel`): both shell Ok at `Interval`. Their cavities certify
     embedded.
   - **The planar dumbbell:** it still refuses, through whichever gate
     item 2 keeps.
   - **The certifying `Sym` rebuild:** it runs the same gate and gives
     the same verdict on one of the rows above.
   - **Mutation.** Make each row go red by disabling the gate, and by
     turning a crossing verdict into Ok.
4. **Cost.** Report the gate's time and `CellReceipt` on the tube and
   vessel fixtures, as the ruling asks ("the pair-filter cost is
   measured"). A row that costs ≥ 0.5 s goes in the slow set.
5. **Docs.**
   - The verb's module docs state the gate per scalar.
   - `shell-curved-wall-clearance-window` closes. The orchestrator
     closes it; you leave it.

## Ground

- `crates/topo/src/shell.rs`, which is VERBS' `paths:` territory and
  CLEAR's by this row. Run
  `python3 scripts/work.py territory --base origin/main`.
- `crates/topo/src/clearance.rs` and `injective.rs` only as callers.
  Do not change the certificate. If a row needs it changed, stop and
  report.
- Read `docs/prompts/implementer-discipline.md` in full.
- Never arm a waiter or a background chain for a build or a test.
  Anything that outlives 600 s runs `setsid`-detached and is polled in
  the foreground.
- CI is the verification of record. Allow 25 minutes.

## Report

≤120 lines, covering:
- the gate's placement and its refusal typing;
- the item-2 decision, with its measurements;
- the rows and how each goes red;
- the cost;
- the PR number and its CI result.
