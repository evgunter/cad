---
id: a-name-through-a-parts-assemble-moves-with-every-earlier-part-edit
kind: issue
title: A host's name through a part's product ⊔ moves with every edit the part made before the ⊔
status: open
priority: P2
opened: 2026-10-23
---


Found by D′3 (`assemble-is-an-operator-and-the-product-is-a-slot`).

A part whose product is a `⊔` of several copies names each copy's
entities `From { read, of }` under the `⊔`'s node. A host reads those
names verbatim through its instance (spec §5′.3). The `⊔` node and the
copy reads it names are minted when the part writes its product,
usually its last edit, and a mint id carries the digest of every edit
before it (`crates/editor-core/src/mint.rs`). So any earlier edit of
the part — a pin value, a profile number — gives the `⊔` and its reads
new ids, and a host name through them vanishes at the next version
update (`DocEdit::UpdateReference`), although the face it names is
the same face of the same body.

Before D′3 the host spelled the part's own name under `InPart`, and a
part name minted early (an extrude's cap) survived later edits.
`crates/editor-core/tests/p2_split.rs`
`i5_a_mate_placed_instance_over_any_part_inlines` re-seats its mate
after the version update for this reason.

The question for the design: whether a `⊔`'s names should not carry
the `⊔`'s own id and read ids (keyed by member position, say), or
whether a version update should carry host names across a re-minted
`⊔` (as `update_references` carries step ids), or whether this is the
expected cost of a product the author re-states.

## Slate triage (2026-10-28, INTENT)

Moved from `work/intent/` by `git mv`, id and body unchanged: residue of INTENT's D10 build that belongs to REFS's charter (names scoped by their read). INTENT keeps D10's core (G and its riders, the stage-3 umbrellas, the carry, the refactor rows, the roots rename); Ev asked for INTENT's slate to be cut to its charter and its residue handed on (in chat, 2026-10-28).
