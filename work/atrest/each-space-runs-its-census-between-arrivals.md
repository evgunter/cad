---
id: each-space-runs-its-census-between-arrivals
kind: unit
title: Each ⊔ decides its operands' meetings: the census at each Assemble's door decides between arrivals and nothing inside one, confirms and matches what an arrival carries, and reports each finding once, at the ⊔ that decided it
status: open
opened: 2026-10-19
priority: P1
cost: M
refs: [a-census-contact-is-a-row-and-a-record, 5903]
blocked_on: []
---

**Sequencing:** `docs/INTENT-STAGE3-SPEC.md` §12 ("Sequencing: the seat and census units"), step 6. Parked on the carry, whose solid-pair match is this unit's confirm-and-match arm; the carry waits on D′3 and E's core. The id keeps its pre-5494 wording ("each space"); ids are stable (`work/README.md`), and `scripts/work.py` has no rename.

D10 `[assertions]` (PR 4666, Ev 2026-10-19; restated by PR 5494, rows 189–191, and PR 5555, row 187): the census is each `⊔`'s door. At rest it examines the meetings between that operator's operands pairwise, and nothing it finds refuses. An arrival is one operand at one value of its indices. Between two arrivals the census decides each pair once, at the highest-dimensional cells that touch; inside one arrival it decides nothing and reads nothing, since the arrival's verdicts are its source's or its door's, carried. A finding belongs to the `⊔` that decided it and is reported once, however many times its body is placed or read, and a `⊔` the product does not read is checked all the same.

What D′3 already builds (`assemble-is-an-operator-and-the-product-is-a-slot`): `Node::Assemble`, whose door grafts its operands as pieces and runs `topo::record_at_rest` over them, one group per arrival, lifted from `product_recorded`'s gather (`crates/editor-core/src/product.rs`, `copy_groups` in `checks/at_rest.rs`). That is a census per `⊔`, decided once per node evaluation, so "a body placed n times is decided once" holds by construction.

What this unit builds on it:
- **Nothing decided inside an arrival.** A transitional check inside each arrival confirms the records it carries and matches the findings it carries (the carry's solid-pair match), and reports anything else as that arrival's door's defect, typed. `the-in-arrival-census-check-retires` removes it.
- **Findings named by reads.** A finding names its cells by the reads they entered the `⊔` through and their names there (a pick where a piece of a copy was read).
- **Reported once.** The unproven-coincidence lint reports each `⊔`'s findings once, at that `⊔`; an instance's findings are its document's, carried.
- **Families.** Before G, D′3 treats each body of a `Pattern` or `PlacedUnion` operand as its own arrival; G makes each member an arrival by construction. This unit keeps that interim and names it.

Re-priced M from H: D′3 moves the census to the node, so what is left is the in-arrival check, the naming and the reporting.

Open measure: `assemble-aggregate-census-is-quadratic-in-solids` for a `⊔` with many arrivals. Also check that `Measure`'s door refuses a `Gap` whose two sites are in different spaces (`crates/editor-core/src/measure.rs`; D′1's `space_faults` is meant to); "an assertion quiets only findings between bodies of its own space" rests on it.

## Slate triage (2026-10-28, INTENT)

Moved from `work/intent/` by `git mv`, id and body unchanged: residue of INTENT's D10 build that belongs to ATREST's charter (at-rest census work; ATREST's stage 5 C makes the census a check at each `⊔`, the same door). INTENT keeps D10's core (G and its riders, the stage-3 umbrellas, the carry, the refactor rows, the roots rename); Ev asked for INTENT's slate to be cut to its charter and its residue handed on (in chat, 2026-10-28).
