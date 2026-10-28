---
id: the-world-still-speaks-of-roots
kind: issue
title: Under the world the kernel still names placements roots: CheckFinding.root, ChecksError::Root, PartRoot*, ProductError::Root*, "part root" wording
status: dispatched
priority: P2
opened: 2026-10-08
refs: [assemble-is-an-operator-and-the-product-is-a-slot]
---


Found by INTENT stage 2 unit C (branch `intent/s2-c-world`), which
retired A10's root list: the product is the copies its placements make.
C reworded the sentences it touched ("placement N output 0" where a
check said "root N output 0") but left the kernel's own names, which
other lanes' bindings and tags read:

- `CheckFinding::root` and `CheckEvidence::NotSeparated { other_root, .. }`
  (`crates/editor-core/src/checks.rs`) hold placement ids;
  `ChecksError::Root` (its text says "checks: placement …").
- `ProductError::RootInvalid`, `RootFailed`, `RootPoisoned`
  (`crates/editor-core/src/product.rs`) are refusals about a placement.
- `PartFault::PartRootFailed` / `PartRootPoisoned` and the "part root"
  wording (`crates/editor-core/src/eval/parts.rs`); the viewer reports
  the part refusal saying "poisoned its root, PlaceInWorld …".
- The Python tags that mirror them (`root_failed`, `root_poisoned`,
  `root_invalid`, `root_without_value`, `part_root_poisoned`).

Also seen by the viewer lane: `resolve` of a vanished name under a
`Placed` wrapper names no node ("a face it derives from vanished
upstream first", `crates/editor-core/src/resolve/mod.rs`), and a check
finding is spoken by the placement's kind noun ("PlaceInWorld <tag>
output 0") rather than by the body it places.

## Re-homed at RECIPE's close (2026-10-11)

Moved to INTENT and parked on stage 3's `a-placement-is-the-bundle-of-mates`, which retires `PlaceInWorld`, roots and the spanning tree. Renaming these now would be churn; the stage 3 B lane renames or deletes them, and closes this row in its PR.

## Unblocked by INTENT stage 3 B (PR 5232, 2026-10-17)

B retired the group roots: `MateRole`, groups and `Unplaced`, as `Space::{World, Loose}`. The product's and checks' `root` vocabulary listed above is untouched; `CheckFinding::root`, `ProductError::Root*` and `PartRoot*` still hold placement ids. The rename is this row's, and nothing blocks it now.

## Skipped by the hygiene batch (2026-10-22)

Checked against D10 as it stands after PR 5494. `[placement.world]`
makes the world one `Space` among others, and `[operations]` gives a
document one product slot reading a body in the world space. Every
name on this row's list (`CheckFinding::root`, `ChecksError::Root`,
`ProductError::Root*`, `PartRoot*`, the Python tags) names a
placement as one entry in the gather of world placements. D′1 keeps
that gather as a named interim (spec §5′.1, "the product is the
gather of the placements whose target is the world"), and
D′3 (`assemble-is-an-operator-and-the-product-is-a-slot`) retires it:
the product becomes a slot and an instance reads its part's product.
What these names become, a body or a piece of `⊔` rather than a
placement, is D′3's answer, so renaming them to "placement" now would
be churn. D′1's spec lists this row among its re-reads; D′3 renames
or deletes these names and closes the row.

## Read by D′3 (2026-10-23)

D′3 (`assemble-is-an-operator-and-the-product-is-a-slot`) retired the gather of world placements. The product is the body its slot reads, and where that is a `⊔` each of its members is one source of the gather (`crates/editor-core/src/product.rs`, `gather`). So the "root" these names hold is now a source of the gather: the slot's body, or a `⊔`'s member read, at the node defining it. That node is a placement only where the member is a placement's copy. `ProductError::Root*` and `CheckFinding::root` still hold that node's id, and a placement's in every corpus document, since each member there is a world copy.

D′3 did not rename them. A rename touches the façade's, the Python tags' and the viewer's vocabulary, and which word replaces "root" (member, source or piece) is a naming choice the spec does not make. The question is open here. The row no longer waits on anything.
