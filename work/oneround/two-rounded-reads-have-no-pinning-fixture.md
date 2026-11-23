---
id: two-rounded-reads-have-no-pinning-fixture
kind: issue
title: Two Rounded reads have no fixture that places a reading inside its own rounding: vertex_section's skew and implicit's frame middle
status: open
opened: 2026-11-21
priority: P2
cost: M
---


Found by the mutant pass of PR 7221 (`oneround/rounded-pairs`) and its
review. Each read below goes through a door of
`geom_core::running::Rounded`. A mutant at each survives every test in
geom-brep and topo, because no fixture puts a reading inside its own
rounding where the read decides.

- `topo::vertex_section::read_sections` adds the skew between the
  door's normal and the table's
  (`offset.value.with_charge(skew(…))`) before the quotient by the
  section sine. The mutant `skew → 0` survives. The fixture this needs is
  a vertex whose offset is decided by a door whose normal differs from
  the table's unit normal by more than its own rounding. That is the
  same plane built twice, by two constructions, at a vertex far from
  the plane's origin (the skew grows with that reach), with the offset
  near `ε/K`, so the resolution row reads Zero without the skew and in
  the gap with it.
- `geom_brep::implicit::circle_sphere_harmonic`'s frame defect reads
  `middle = (a + b + a·b − c²).magnitude()`. The mutant `→ value.abs()`
  survives, because in every fixture the max with
  `max(|a|, |b|) + |c|` (whose entries carry the same bounds) covers it.
  The fixture this needs is a frame whose two defects are both
  inexact and of one sign: `û` and `n̂` each a few ulps long, with
  `n̂·û` exactly zero. Then `middle ≈ |a| + |b|` exceeds the other entry.
  An exact dyadic evaluation of `‖GᵀG − I‖` should then fall between
  the mutant's bound and the read's.

P2: both reads are charges that only widen a bound, and no answer moves
under the mutants today.
