---
id: one-home-for-where-a-shell-stands
kind: issue
title: Where a shell stands against another is read three ways: check 10 and the result sort by one vertex, the boolean by a vertex-edge-face ladder; and a shell's role by three readers
status: open
opened: 2026-10-03
priority: P2
cost: M
---

Filed by PR 3891 (the piece rule's sort), from its dual review's
MINOR-7, NOTE-1 and the three-role-readers note.

**Where a shell stands.** Three sites ask whether a point of one
closed shell lies inside another:

- tier 3's check 10, `crates/topo/src/validate.rs`,
  `shell_winding_errors` over `witness_insides`;
- the result sort, `crates/topo/src/pieces.rs`, over the same
  `witness_insides`;
- the boolean's uncut-shell verdict,
  `crates/topo/src/boolean/shell_witness.rs`, `complex_side`.

The first two read vertices only: a shell every vertex of which lies on
another shell refuses the sort (`PieceSortError::WitnessTouching`) and
leaves check 10 silent, and a walk refusal at the first vertex refuses
the sort (`PieceSortError::Probe`) where the boolean's ladder would read
on. `complex_side` walks vertex → edge midpoint → planar face interior
and skips inconclusive readings, but it is typed to a whole other body
(`point_in_solid`) and to `BooleanError`. One ladder, generic over the
probe, would serve all three.

**A shell's role.** Three readers decide `Outer` or `Void`:
`validate::shell_role` (check 10's sign walk, used by the sort),
`props::classify_shells_of` (the classifier `shell` and the editor's
Connectedness check use), and `boolean::solid_contain`'s at-infinity
read (closed form only). They can part in band, which is how
`ShellError::OperandOuterShells` can still fire with two outers after
the sort (its doc says so).

**NOTE-1, folded here.** A solid with one decided `Outer` and an
undecided shell that is really a second piece stays under one solid:
the sort is silent there by design, as check 10 is, so nothing reads
the second piece. One reader that decides more often narrows it; it
cannot close it.
