---
id: d2-addendum-has-no-row-for-a-state-only-the-entry-gate-excludes
kind: issue
title: design: the D2 addendum has no row for a state no input reaches because only the door's whole-body entry gate excludes it
status: open
opened: 2026-09-30
priority: P3
cost: M
design: true
refs: [D262]
---

Filed by TOPO's fix pass on PR 3532. **Design text, so Ev's call**:
the D2 addendum (`docs/DESIGN.md`, "The bug-vs-invalid-state
taxonomy") is ratified, and this row states the gap rather than
editing it. Filed on VERDICT's slate beside
`d2-addendum-has-no-row-for-an-admitted-input-answered-wrongly`, the
other row that asks the addendum for a class it lacks.

## The gap

Row 1 is "reachable by input, invalid", a typed error. Row 4 is a
kernel bug observable in a branch, `unreachable!`, licensed only by a
proof in the same call (a key minted there, or proven live there) and
**never** by the body's tier-1 validity. Neither fits a lookup that no
input reaches because the door's own whole-body entry gate refuses
every body that could fail it:

- it is not reachable by input at the public door, so row 1's name is
  wrong for it;
- its only proof is a whole-body check (tier 1, or tier 2's
  `validate_closed`), which is exactly what row 4 bars.

Such arms are typed refusals today by house precedent, not by a row:
`merge_faces`' `merge_kind` (face and surface lookups), and since PR
3532 `planes_declared_equal`'s face lookup and `edge_chord_len`'s
vertex and point lookups (`crates/topo/src/merge_faces.rs`). The door's
tier-2 entry gate refuses every tear they read, measured by
`merge_faces::tests::the_entry_gate_refuses_every_tear_the_adjacency_scan_reads`.
That PR says so rather than filing them under row 1.

## What the addendum could say

Candidates, for the designers and then Ev:

1. A row between 1 and 4: "excluded at the door by a whole-body gate
   the same call ran": a typed error, with the gate's test named beside
   the arm. The error stays recoverable if a later caller reaches the
   helper without the gate.
2. Widen row 4's licence to a gate the same public call ran on the
   same body, unmutated since: `unreachable!` with the gate named in
   the message. The cost is that a helper called from a second door
   without that gate becomes a panic on input.
3. Leave the rows as they are and say in the row-1 bullet that row 1
   also holds states a whole-body gate excludes (typed, never
   `unreachable!`), which is what the code does now.

Row 0 answers first in each case: the arms above that could take a
type change have (PR 3532 moved the adjacency test's surface lookup
into the kind census's resolved record), and what is left reads a key
from a back-pointer no type proves live.
