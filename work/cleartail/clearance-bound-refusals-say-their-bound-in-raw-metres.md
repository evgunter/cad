---
id: clearance-bound-refusals-say-their-bound-in-raw-metres
kind: issue
title: A clearance bound's refusals (NotADistance, EmptyBand) say the bound as a raw metre count, not in the reader's notation
status: open
opened: 2026-11-02
priority: P3
cost: E
---

Found by DOORS' `the-gui-has-no-clearance-consumer`, whose clearance
row draws these sentences under the requirement's equation.

`ClearanceRefusal::say` (`crates/editor-core/src/clearance.rs`, the
`NotADistance` and `EmptyBand` arms) writes the offending bound as the
bare `f64` it evaluated to: a requirement written `≥ -1 mm` reads
"the bound -0.001 is not a distance", and a band written `20 mm … 5 mm`
reads "the band's lower end 0.02 is above its upper end 0.005" — in
metres with no unit, beside an equation the viewer spells in the
reader's notation (`viewer::tree::Cleared::equation`). The number is
the right one; the reader is left to recognise `0.02` as their
`20 mm`.

## Repair shape

Say the bound as a length (a unit), or carry it typed in the refusal
so a consumer spells it in its own notation, as the viewer spells
every other computed value (`viewer::props::Computed::spelled`).
