---
id: the-separation-check-misses-an-overlap-inside-a-nested-or-unread-assemble
kind: issue
title: The advisory Separation check reads only the product's own copies, so an overlap inside a nested ⊔, or inside a ⊔ the product does not read, gets no Separation finding
status: open
opened: 2026-10-26
priority: P2
cost: M
---

Found by both delta reviewers of D′3 (PR 5636, head `220ba3f855`); filed at the INTENT orchestrator's request. It predates the fix pass.

## What

`checks::separation` (`crates/editor-core/src/checks.rs:1825`) pairs the solids of `Subject::Product`, the gathered product, by `gathered.solid_copies`. After D′3 the product is one body the slot reads. When the slot reads a nested `⊔`, `(a ⊔ a′) ⊔ n`, the inner `⊔`'s pieces arrive at the outer gather as one censused arrival. A `⊔` the slot does not read is not gathered here at all (`product::assemble_gathers` carries it beside the product, `Product::assembles`). So an overlap of `a` and `a′` is reported by the census at the inner `⊔`'s door and carried to the at-rest gate, but this advisory check never sees it, as a pair or at all.

## Decide when dispatched

Whether Separation reads every `⊔`'s pieces (each `AssembleGather`, and each censused arrival's own pieces), or states that it holds the product's top-level copies only, with the census answering inside an arrival. Rows owed: an overlap inside a nested `⊔`, and inside a `⊔` the product does not read, each either reported by Separation or named as the census's.

## Slate triage (2026-10-28, INTENT)

Moved from `work/intent/` by `git mv`, id and body unchanged: residue of INTENT's D10 build that belongs to ATREST's charter (at-rest census work; ATREST's stage 5 C makes the census a check at each `⊔`, the same door). INTENT keeps D10's core (G and its riders, the stage-3 umbrellas, the carry, the refactor rows, the roots rename); Ev asked for INTENT's slate to be cut to its charter and its residue handed on (in chat, 2026-10-28).
