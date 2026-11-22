---
id: mesh-pole-junction-unwrap-picks-a-whole-turn-off-numbers
kind: issue
title: the mesh walk's pole-junction unwrap picks a branch off numbers, unswept against the same-edge rule
status: parked
opened: 2026-10-14
priority: P1
cost: M
refs: [the-period-jump-has-no-ruling]
blocked_on: [a-horns-cusp-reads-as-its-upper-apex-so-a-face-below-it-turns-the-wrong-way]
---


Found by PR 5080's review (both reviewers). It was outside PCERT's sweep
of strict-branch picks at singular points.

**The site.** `mesh::walk::unwrap_tie` (`crates/mesh/src/walk.rs`, about
:378-391; called from the loop walk's pole-junction arm, about :1247).
It is the f64 mesh twin of `topo::chord_join`'s pole pick:
- It reads `k = (prev − raw)/τ`.
- It takes `k.round()` when `k` is within ¼ of an integer.
- Otherwise it takes the floor/ceil candidate nearer an `anchor`.

So at a pole junction it chooses between `prev` and `prev ± τ` off the
numbers alone.

**The ruling.** Ev's ruling on `the-period-jump-has-no-ruling`
(PR 4975) says a whole turn is never read off numbers. At a pole:
- the walk entering and leaving along one edge is a whole turn by
  orientation, a literal;
- distinct edges are decided by sign decisions at whole-period marks,
  metered by the parallelism lever.

`topo::chord_join::pole_branch` implements this (PR 5080).

**What is unchecked.** Whether `unwrap_tie` agrees with that structure:
- on a slit, the same edge in and out at the pole, where `k` is an
  integer and `round` keeps the previous branch rather than turning a
  whole turn;
- on two distinct edges within ε of one direction.

A row on the slit dome of `crates/sweep/tests/pole_slit_window.rs`,
meshed, would show it.

## Moved to TESSWALK (what the walk and the lanes decide: structural premises, and readings taken off undecided numbers) (2026-10-28)

Cut out of TESS by its semantic-seam split (`work/README.md`, Track size) when TESS measured 146 budget points against 30. Moved by `git mv`; the id and body are unchanged.

## The stored lift now holds the structure (`onedoor/three-walks`, 2026-11-12)

The lift's joint at a pole stores the turn on the face's side
(`topo::pcurves`' `singular_turn`, `crates/topo/src/joint.rs` "The
singular set"): a whole turn where the loop leaves along the edge it
arrived on, and the face-side branch between two distinct edges. The
trimmed lane reads it (`mesh::trimmed::trim_polygon` over
`Body::loop_lift`), and its own pole search, `choose_pole_azimuths`,
is gone. `mesh::walk`'s `unwrap_tie` is the pin left standing: it
places a pole junction's azimuth from `prev` and `anchor` on the
curved lane's own walk, beside the stored element. Reading the
junction off `Body::loop_lift` there would retire it.

Moved from TESSWALK to TESSLIE on 2026-11-20, by `git mv`, when TESSWALK merged into TESSLIE: TESSWALK held no P0 or P1 row, so no sitting would dispatch it on its own (`work/README.md`, Track size), and its charter is one TESSLIE carries.

## Re-banded P1 (2026-11-20, PR 7202 review)

A second derivation of a whole-turn branch the stored lift already holds, the reason `singular-turn-spells-the-chart-winding-rule-by-hand` stayed P1, and against the ruling on `the-period-jump-has-no-ruling` (PR 4975) that a whole turn is never read off numbers.

## Measured against the lift; parked (TESSLIE lift-turns, 2026-11-21)

Every `unwrap_tie` call over the mesh and sweep suites was compared with
the loop's stored lift: the column's offset from traversal 0, walk
against lift (`LiftedRow::entry_exit_points`), agreeing when the two
differ by less than half a turn. 21527 agree, 2 disagree, 32 read a loop
with no lift (the d9 golden corpus and four probe rows, all Euler-built).

- **No call is at a pole junction.** Every call opens a meridian column
  after a RIM junction, where `prev_u` is the rim's end azimuth and the
  copy about it is within a quarter turn: continuity, never the tie
  zone. In a rim-anchored loop the meridian-to-meridian pole junction is
  the closing side's column (`anchor`), which `unwrap_tie` does not read.
  The arm this row is about has no witness
  (`no-suite-body-reaches-a-pole-junction-off-the-closing-side`).
- **Both disagreements are the lift's**, not the walk's: the lower needle
  of `crates/sweep/tests/the_football_revolves.rs` (two rows, both
  volume-checked and passing) stores a lift that winds the chart once at
  the horn's cusp. Reading the lift there would mesh the complement.

So the walk is not converted while the stored lift is wrong on the only
pole-cusp faces the suites hold. Parked on CHARTTAIL's cusp row.
