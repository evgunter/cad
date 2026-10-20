---
id: a-plane-piece-the-nesting-rule-cannot-place-reads-as-bordering-the-exterior
kind: issue
title: A plane piece the nesting rule cannot place, or a hole left on a face of its own, reads as bordering the exterior, so its Borders walls can be cited too widely
status: open
opened: 2026-10-20
priority: P3
cost: M
---


Disclosed by WIRE's section-face lane (PR 5486, 2026-10-20). Neither case
is built by any test.

`plane_pieces::plane_walls` (`crates/editor-core/src/names/plane_pieces.rs`)
reads a piece's outline as bordering the exterior unless the kernel
recorded it in a ring (`SplitNaming::islands`, filled by
`record_islands` in `crates/topo/src/splitting/finish.rs`). Two cases
reach the exterior reading wrongly:

- **An outline the nesting rule cannot place.** `section_loops::nest`
  counts an enclosure that it cannot decide (an outline edge on a spiric
  or NURBS carrier, or a reading in the band) as no enclosure. So an
  island in such a ring has no row, and it reads as bordering the
  exterior. The exterior then divides wherever there are two pieces, so
  the island and every outer piece cite their outer walls. That spelling
  is coarser than the rule's, not another face's, but it renames when
  the reading becomes decidable.
- **A hole left on a face of its own.** `nest_hole_sections` leaves a
  clockwise section that it cannot place as its own face, cancelling the
  face round it. That face is a plane piece (a section face), and its
  outline reads as bordering the exterior.

The fix is a typed refusal where the record does not decide, or a
reading that does decide. Either one changes what the emitter is given
by the kernel's record.

Signed (WIRE, section-face lane).

## Moved to WIREREST (2026-10-20)

WIRE measured 69 budget points against 30. This row is P3 or P4, and WIRETAIL already stood at 25 of 30, so it moved by `git mv` (id and body unchanged) to WIRE's second tail. Signed (WIRE orchestrator).
