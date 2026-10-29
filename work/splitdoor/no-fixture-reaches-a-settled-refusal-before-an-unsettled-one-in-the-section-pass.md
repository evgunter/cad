---
id: no-fixture-reaches-a-settled-refusal-before-an-unsettled-one-in-the-section-pass
kind: issue
title: No fixture reaches the section pass with a settled pair's refusal before an unsettled pair's, so walking every pair after a hold is unpinned
status: open
opened: 2026-10-29
priority: P3
cost: M
---


Found by the review of PR 6029 (`splitdoor/curved-copy`), its M2.

## What

`boolean::ops::section_extent_pass` holds a refusal on a pair the reduction settled one carrier (`OnDeferred::hold`). Since the hold landed (PR 5969), the walk under it stopped at the first refusing pair (`walk_pairs`' `stop`). A first pair that was held therefore left every later pair uncertified, so a later pair's real refusal could go unraised. PR 6029 walks every pair (`fallback_section_pairs`, `stop: false`), and no row pins that.

## Measured

On PR 6029's head, I mutated `fallback_section_pairs` back to `stop: true`. Every test still passed:
- `cargo nextest run -p sweep -p topo -p editor-core --profile ci`: 8953 of 8953;
- the slow and ignored curved-copy, two-rims, torus-rest and ruling rows: 50 of 50.

No fixture in the corpus has a held refusal ahead of a refusal that is not held.

## Why one is hard to build

The two refusals have to land on one walk: a settled pair (a face of A on one carrier with a face of B), then, later in arena order, a pair that is not settled and that the section certificate refuses with no crossing anywhere. Every attempt so far fails for one of two reasons:
- **The extra geometry overlaps the copy.** A part of B that meets A also meets B's own copy of A's face, so B is not a valid operand. This is B as a copy plus a disjoint lump.
- **The meeting brings a crossing.** The extra geometry's edges then lie on, or cross, A's face, so the sweep's touches clear the pair by W4, or the path is the crossings one. This is B as the copy with a feature attached (a ring about a rod copy).

A pose that might reach it: B's settled face partly coincident with A's, plus a curved face of B that meets A's face in a loop interior to both, with no edge of either on the other.

## Owed

A fixture that reaches the hole and a row that goes red under `stop: true`, or a proof that the no-crossings path cannot reach it.
