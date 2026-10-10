---
id: an-instance-carries-its-parts-coincidences
kind: issue
title: An instance carries its part's coincidence rows, so the lint on a product document reports its parts' unproven glue
status: open
opened: 2026-10-08
priority: P1
cost: M
blocked_on: [coincidences-are-recorded-at-one-door]
---


## What

`crates/editor-core/src/eval/wire.rs` `wire_instantiate_part` sets
`OpOut::coincidences` empty ("The part's own rows are its document's,
read there"), while it carries the part's `contacts` and
`CarriedDeclarations` through. So `run_checks` on a product document
reports none of its parts' unproven coincidences: the lint is silent
about a part's declared glue, and `CheckKind::Certified` reads that
silence as "proven". D10's "recorded" holds (the rows sit on the part's
own evaluation); the product's checks pane does not see them.
Orchestrator ruling on S4-B (2026-10-08): its own row, not in B.

## Why it is not a carry like the contacts

A part's rows are not on its product. They sit on every deciding node of
the part's evaluation (a union, a split, a blend inside the part), their
cells named in those nodes' input tables, in the part document's node
ids. Contacts cross because the product owns them in its arena keys and
`transform_rigid` keeps keys. So a row cannot be renamed into the
instance's table: its cells generally have no name in the part's
product (a declared `Rest`'s two caps are merged away), a part node id
means nothing in the instancing document, and the door's walk runs over
the part's document.

## Design sketch (the `unplaced` precedent)

- In `crates/editor-core/src/eval/parts.rs`, where the nested
  evaluation is read (`unplaced` gathers
  `evaluation.unplaced_groups()` and `all_unplaced_below()`), gather
  every node's `NodeValue::coincidences` and prove each against the
  part's document there (`coincide::prove(doc, row)`), and carry the
  part's own and those its parts carried up as `PartRow<…>`
  (`assembly.rs`: `of`, the route `via`, and the held labels), on a
  new `PartValue` field.
- `wire_instantiate_part` adds this instance as each row's first hop
  (`PartRow::through`), as it does for `minted`/`unminted`/`unplaced`,
  onto a new `CarriedDeclarations` field.
- `checks.rs` `unproven_coincidence` reads every value's carried rows
  too and reports each unproven one attributed to the instance
  (`FindingSubject::Node(instance)`), with the route and the part's
  verdict; the story says the cells with the part's held labels (a
  `Say` impl for the row), and the evidence carries the route.
- Python (`Coincidence`, `CheckEvidence`) and the viewer's Checks
  window project the route.
- The route type is shared with B2's instantiate seam
  (`contact-records-cite-their-decision`: a carried record cites the
  part's decision through the instance's read); build whichever lands
  first so the other reuses it.

The row that pins it: a product instancing a part whose union declares
a `Rest` reports one unproven-coincidence finding about the instance,
naming the part's two caps by the part's labels; one whose part's row
is proven reports none.
