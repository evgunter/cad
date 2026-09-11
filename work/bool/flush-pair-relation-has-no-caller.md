---
id: flush-pair-relation-has-no-caller
kind: issue
title: The planar flush-pair door has no in-tree caller since the detector widened
status: open
opened: 2026-09-05
---


Disclosed by SEAT-FW, which is what removed the caller.

`topo::boolean::flush_pair_relation` (`crates/topo/src/boolean/rest.rs:521`)
is the planar projection of `carrier_pair_relation`: same descriptions,
same identity record, and its verdict function is the one
`carrier_eq`'s `(Plane, Plane)` arm delegates to. Verify-at-use stopped
calling it at M9-1; the flush detector was its last consumer and
`topo::flush::pair_finding` now asks `carrier_pair_relation` instead
(`crates/topo/src/flush.rs:250`). So the door is exported from
`topo`'s root (`crates/topo/src/lib.rs:284`) and from
`topo::boolean` (`crates/topo/src/boolean/mod.rs:125`) with **no
caller anywhere in the tree** — tests, demos and probes included.

Two dispositions, and this unit took neither because both are wider
than its fence (the door lives in the boolean's own module):

- **Keep it** as a published planar door for a caller who holds two
  faces and wants the plane question asked as a plane question. Then
  it wants a row that exercises it, because nothing does today: an
  unused pub door is one nobody would notice breaking.
- **Delete it**, and with it the last spelling of "the planar
  projection", leaving `carrier_pair_relation` the only door. Cheap —
  the deletion is the function, two re-export lines and the doc
  paragraph — and it is an API removal, so it is a decision rather
  than a tidy.

SEAT-FW updated the door's prose to say it has no in-tree consumer
(the sentence that claimed "exactly one caller" was made false by this
unit and could not be left standing) and stopped there.
