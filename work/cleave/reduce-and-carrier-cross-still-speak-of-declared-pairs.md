---
id: reduce-and-carrier-cross-still-speak-of-declared-pairs
kind: issue
title: reduce.rs and carrier_cross still speak of declared and undeclared pairs, and on_declared_shared_carrier's doc claims C2/C4 forbid a value-decided coincidence
status: open
opened: 2026-10-12
priority: P3
cost: E
---



Found by `cleave/one-carrier-reads-ellipses` while removing
`carrier_cross::BoundaryReads`, whose `LinesAndCircles` reading was
justified by "the D10 hold". That branch rewrote the lines it touched
and left the rest, which this row holds.

## What

Booleans glue on Zero (PR 4496), and stage 4 F (PR 4641) retired the
declared surface, so a boolean no longer reads declared pairs. The
reduction still describes its one-carrier arms in that vocabulary:

- `crates/topo/src/boolean/reduce.rs` has about a hundred lines that
  say "declared" or "undeclared": the `Interior` enum ("What the
  declared arms know"), `Placement::declared` and
  `Placement::undeclared_no_interior`, and the comments in
  `curved_face_arm` that tell "the declared arms" from "the undeclared
  cosurface question".
- `reduce::on_declared_shared_carrier`'s doc says the rung reads "a
  verified one-carrier declaration", that `Rest` and a continuation
  are the classes that license it, and that "an undeclared pair is
  never read as coincident by value (CONTACT-DESIGN C2/C4)". Under D10
  a coincidence decided Zero by value is exactly what glues, so the
  C2/C4 claim is false. The function reads
  `GluedPairs::verified_one_carrier`, which the glue door records for
  value-decided pairs.
- `crates/topo/src/boolean/carrier_cross.rs`'s module doc opens with
  "The declared-cover arms of `reduce::curved_face_arm`".

## Direction

Rename by what each thing decides now (a pair the glue door decided
one carrier, a pair decided distinct), and restate
`on_declared_shared_carrier`'s doc from what `verified_one_carrier`
records. The rename has no design content.

**For the README's owner:** `crates/topo/README.md`'s C2 and C4
(CONTACT-DESIGN: "Declared contact as data", the `ContactClass`
verification lists) still describe declared contact as live. D10
retires the declared-contact seats ("CONTACT-DESIGN C4's
`BooleanCoincidence` on booleans"), so those clauses are stale. They
are a ratified companion page, so their rewording is the owner's
(the program retiring the declared-contact seats under D10), not this
row's.
