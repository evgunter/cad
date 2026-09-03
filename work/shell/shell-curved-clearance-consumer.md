---
id: shell-curved-clearance-consumer
kind: issue
title: where a curved wall-clearance gate can call the E7 engine from (the shell verb sits below it)
status: open
opened: 2026-09-03
refs: [shell-curved-wall-clearance-window, M10-5, 1055, 1191]
---

## The valve M10-5 left open

M10-5 built the E7 clearance engine and its acceptance fixtures are
issue 1055's own dumbbell: the neck's two facing walls, 0.4 apart, are
reported `Violated` against the 0.6 that two 0.3 walls need, with an
f64-verified witness. The EVALUATOR that issue asked for exists.

What did not land is the consumer, and the blocker is layering, not
effort.

- `topo::shell`'s gate site (`wall_clearance`, `crates/topo/src/shell.rs`)
  is inside `crates/topo`.
- The E7 engine is `editor_core::clearance`, and `editor-core` DEPENDS
  on `topo` (the G1 layering note in `crates/editor-core/Cargo.toml`:
  "editor-core sits ABOVE the kernel … the kernel crates gain NO
  editor-core dependency"). `topo` cannot call it.
- The dependency direction is not incidental to the engine either. Its
  inputs are a `Doc`, a `ParamBox` and a leaf the E6 driver certified —
  document-layer objects that do not exist at `topo`'s altitude. A
  `Body<Interval>`-only engine would be a second subdivision, not a
  call.

So closing 1055 needs a decision about WHERE a curved wall-clearance
gate lives, and that is a VERBS + M10 design question:

1. **A verb-layer gate above editor-core**: `shell` keeps its
   closed-form planar gate, and the curved arm becomes a check the
   document layer runs on the shelled body (the M10-6 reporting lane's
   natural shape — an assertion over a `min_clearance` measure). The
   verb then no longer refuses; a report does.
2. **A `topo`-level engine over `Body<Interval>`**: the same inner
   subdivision without the parameter box, called from `wall_clearance`.
   Correct for the verb's own question, and a duplicate of the cell
   subdivision this unit shipped.

## What it would cost today even with the layering settled

Two measured limits from M10-5's own suite, both worth knowing before
either option is chosen:

- **Box width.** No node's interval replay builds over a parameter box
  wider than a small fraction of ε (issue 1191's class), so a
  parametric curved gate answers over ε-scale boxes only. Option 2 does
  not have this problem — it has no parameter box.
- **Cost, and WHERE it falls.** Measured on the hexagonal prism of
  `crates/editor-core/tests/m10_5_clearance_interval.rs` and on the
  twelve-vertex comb of `m10_5_r2_probes_interval.rs` (R2's fixture):
  a bound the geometry BREAKS costs a handful of cell pairs, because
  the sweep stops at the first verified witness; a bound the tree can
  EXCLUDE costs nothing. What exhausts the shipped
  `DEFAULT_MAX_CELL_PAIRS = 65_536` is the FRONTIER — a bound sitting
  on a pair's own separation, where no cell pair ever resolves either
  way. A shell gate picks its own `c`, so whether it lands in the cheap
  regime or on a frontier is a property of the wall thickness it is
  asked about, not of the engine. It also needs a cheaper pair filter
  than the quadratic adjacency walk the engine does today once the face
  count of a shelled body is in play.

## Home

`crates/topo/src/shell.rs` (the gate site, which cites 1055 by name)
and `crates/editor-core/src/clearance.rs` (the evaluator). Rides with
`work/verbs/shell-curved-wall-clearance-window.md`, which is the
issue-1055 record and stays parked until this is answered.
