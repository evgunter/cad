---
id: an-instance-carries-its-parts-coincidences
kind: issue
title: An instance carries its part's coincidence rows, so the lint on a product document reports its parts' unproven glue
status: closed
opened: 2026-10-08
priority: P1
cost: M
closed: 2026-10-27
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
  every node's `NodeValue::coincidences` and prove them against the
  part's document there (`coincide::prove(doc, &[(node, rows)],
  resolver, tol, cancel)`, which replays that document once at `Sym`,
  its own parts resolved through `resolver`, each document's variables
  its own symbols), and carry the
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

## Re-cut (2026-10-26, ATREST orchestrator)

The row's first blocker (`coincidences-are-recorded-at-one-door`, PR 4354) is closed. It is re-parked on D′3 (`assemble-is-an-operator-and-the-product-is-a-slot`, PR 5636), which rewrites `wire_instantiate_part` and what an instance defines (one body, the copy of its part's product, with its census decisions carried).

**The premise, restated against D10.** `[assertions]` says a finding belongs to the `⊔` whose census decided it and is reported once there. `[placement.world]` says an instance's decisions are reported once, as that document's. So a part's unproven rows are the part's findings, and the host has nothing to re-decide or re-report. What still stands is the reading of the report: the host's lint is silent about its parts, and under `CheckKind::Certified` silence reads as "proven".

**What closing it takes.** The host's report carries each of a part's unproven rows **as the part's**: once per part row, with its `Route`, judged by the part's own door and quieted only by the part's assertions. No assertion of the host quiets it. This is the shape J (PR 5325) and stage 5 C keep for carried interference (`CarriedInterference`, `once_per_part_row`). The design sketch above stands, except that the finding is the part's, reached through the instance, and is not the instance's own finding. Read the sketch's `FindingSubject::Node(instance)` as the route's first hop, not as the owner.

## Released (INTENT, 2026-10-26)

D′3 merged (PR 5636). An instance now defines one body, a copy of its part's product, and `PartValue` holds `coincidences` (`eval/parts.rs:118`, carried at `:145`/`:827`). Re-check against main whether the lint on a product document now reports its parts' unproven glue. ATREST's call: close it, or narrow it to what remains.

## Closed

2026-10-27 (ATREST lane). Re-checked against main after D′3 and stage 5 C: the row was still red (the part reported its five unproven rows, the host none). Now the part's door rows (its own, then those it carried up) ride `PartValue::door_rows` and `CarriedFindings::coincidences`, each once per part row with its `Route` and its pinned document. The host's `DoorJob` judges them with the part's own door over that document, at rest and cancelable, and reports each as `CheckEvidence::CarriedCoincidence` at the route's first instance, quiet only as the part's assertions quiet it. Python reads it as `CheckEvidence.carried_coincidence`; the viewer shows it at the instance, said by the part's labels. Rows: `crates/editor-core/tests/atrest_carried_coincidence.rs`. The sweep's sibling, a boolean over an instance dropping the part's census findings, is filed as `a-boolean-over-an-instance-drops-its-parts-census-findings`.
