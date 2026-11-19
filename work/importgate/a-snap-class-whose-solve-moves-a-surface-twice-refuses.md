---
id: a-snap-class-whose-solve-moves-a-surface-twice-refuses
kind: issue
title: step-import: a snap class whose solve moves a surface twice refuses; a joint solve anchored at the class's earliest record would make it
status: open
opened: 2026-11-19
---


Found by the dual review of `importgate/snap` (PR #7135).

## What

`crates/step-import/src/snap.rs` solves each class of relations the file
band reads `Zero` with every move computed from surfaces no construction
moves, each relation's mover its own later record (`solve`, `later`). A
class where that rule would move a surface twice, or would read a
surface another construction moves, refuses not-yet-supported naming
this row (`ANCHOR_LANE`):

- a surface two constructions would move: a corner sphere of a filleted
  box off its corner, the later record of the three fillets' coaxial
  relations (`a_relation_the_file_band_reads_zero_is_snapped.rs`,
  `a_surface_two_constructions_would_move_refuses`);
- a relation whose kept surface another construction moves: a chain
  A–B–C of coaxial records, a plate later in the file than two bosses it
  is normal to, a capsule whose cap and base are both off its cylinder
  (`a_construction_reading_a_moved_surface_refuses`).

Whether such a file imports turns on which record it states first: the
same solid with its records in another order may snap.

## What would close it

A joint solve anchored at the class's earliest record: the earliest
record of the class stays, and each other surface is constructed from
the surfaces already fixed, walking the class outward, the order the
graph fixes rather than the file.
