---
id: the-tours-pattern-scenes-write-their-yields-by-hand
kind: issue
title: The tour's heatsink, impeller and assembly write SetYields and the splice by hand
status: open
opened: 2026-11-23
---

After PR 7506, the tour's pattern scenes write the binder's yields and
the splice by hand:

- `demos/tour/src/heatsink.rs` and `demos/tour/src/impeller.rs`: a
  `pncad::pattern::yields_edit` after the pattern insert, and
  `Spell::All` in the union.
- `demos/tour/src/assembly.rs`: a `yields_edit` before the world reads
  the row's members.

That is the plumbing a demo should not show (`memories/demo-purpose.md`).
Each site carries a gap comment pointing here. They move to the pncad
preset writers once those write the binder whole: the Rust side of
`python-writes-repetition-through-doc-each-and-binder-presets`.
