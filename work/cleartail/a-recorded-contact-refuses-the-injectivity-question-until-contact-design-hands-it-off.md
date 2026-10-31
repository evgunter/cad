---
id: a-recorded-contact-refuses-the-injectivity-question-until-contact-design-hands-it-off
kind: issue
title: a pair with a recorded contact refuses the injectivity question typed until CONTACT-DESIGN's records hand off to it
status: open
opened: 2026-10-28
priority: P2
cost: M
---


## Finding

SELFX (`work/clear/self-intersection-drops-every-vertex-sharing-face-pair-globally.md`,
item 4 of `docs/SELFX-SPEC.md`) makes the injectivity question refuse a
body that carries a contact record between two cells of the scope:
`crates/editor-core/src/clearance.rs:997`, through `recorded_contact`
(`:1394`), returns `ClearanceRefusal::RecordedContact { a, b }`
(`:481`), naming the record's two cells. A record says the two cells'
interiors meet, so the boundary is not injective there by declaration,
and nothing yet says what the question should answer about it.

## The handoff to design

What the injectivity question does with a recorded contact is
CONTACT-DESIGN's to say. The candidates:

- the record is the answer: report it as the violation, with its two
  cells as the witness;
- the record is an exemption: the pair it names is decided by the
  record, and the rest of the body is still examined, so a contact
  elsewhere still reads;
- the record is lifted into M: the glued surface identifies the two
  cells, and the walk then treats them as one cell of M (touching).

Until one is chosen the refusal stands, typed. The analysis drive
(`box_independent_measure_class`, `crates/editor-core/src/drive.rs:2245`)
classes it with the refusals that bisect, because the records are read
at one leaf's replay and can differ over a smaller box.
