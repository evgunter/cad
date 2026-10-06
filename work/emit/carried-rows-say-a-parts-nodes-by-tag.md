---
id: carried-rows-say-a-parts-nodes-by-tag
kind: issue
title: A carried row from a part says the part's nodes by tag: its route's deeper hops and its mate, group and cause
status: closed
pr: 4114
branch: emit/carried-rows-part-labels
opened: 2026-10-06
closed: 2026-10-06
priority: P3
parent: node-labels-are-document-data
---


Found by `a-frame-that-speaks-a-parts-refusal-holds-no-resolved-part`'s sweep. The rule is DESIGN.md Band 1, "Node labels": a value the evaluation memo reuses holds a label only when its memo key fixes it, and a part's labels are in its pin.

That row made a part's FAULT carry the part's nodes (`PartFault::held`, recorded in `eval/parts.rs` `product_fault`). The rows a part carries up on its SUCCESS path do not ride the fault, so they still say the part's nodes by tag, though the same rule would allow their labels:

- `Route::say` (`crates/editor-core/src/assembly.rs:150`): each `via` hop (`:161`) is a node of a document below, said by tag. The first hop, `through`, is this document's and is said by the frame (PR 3841).
- `CarriedRefusal::say` (`assembly.rs:220`): the `MintRefusal` (`:229`), in `route.of`'s ids, by tag. It reaches `AssemblyError::CarriedMintRefusal` and Python `CarriedRefusal.refusal` (`crates/pncad-py/src/py/assembly.rs:566`, "said by tag").
- `CarriedUnplaced::say` (`assembly.rs:279`): the group (`:288`) and the cause (`:289`), in `route.of`'s ids. It reaches `ExportError::UnplacedBelow` (`crates/pncad/src/export.rs:126`) and Python `export_err` (`crates/pncad-py/src/py/value.rs:1798`).
- `Attribution::Carried` (`assembly.rs:543`): the carried declaration's mate, said through `Speaker::TAG`.

**Why the fault's snapshot does not reach them.** These rows ride `PartValue` (`carried_unminted`, `carried_unplaced`, `carried`; `eval/parts.rs` `evaluate_entered`), built from the part's product, and each instance prepends its hop in `eval/wire.rs:553` (`Route::through_instance`). Each row's ids span several documents: the row body is `route.of`'s, and each `via` hop is the document one level up from the next. A snapshot would have to be taken per row where `route.of` is in hand (the evaluation of that document), and one per hop where the hop's document is in hand (the `evaluate_entered` of each intervening part), then carried on the row. That is a change to `Route` and the three carried row types, beyond a fault.

**Candidate shape.** `evaluate_entered` holds the part's `ProfileDoc` while it builds the `PartValue`. It could record, beside each row it hands up, the part's nodes that row names. That is the row body for rows the part minted itself, and the hop it is about to become (`through`) for rows from below. A frame would then say each id from the snapshot of the document it is numbered in. As with `PartFault`, the rows' equality would then compare those labels too, so rows with equal ids from parts labelled apart (two pins) would compare unequal.
