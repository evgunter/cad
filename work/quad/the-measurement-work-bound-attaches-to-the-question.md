---
id: the-measurement-work-bound-attaches-to-the-question
kind: ruling
title: what bounds a certified measurement's work, now that the quadrature reads per knot span: per face, per span cell, or per question
status: closed
opened: 2026-10-30
priority: P0
refs: [the-at-rest-gates-cost-grows-with-the-face-count-of-a-split-conic-patch, 6014, 6092]
closed: 2026-10-31
pr: 6102
---


## The question

DESIGN `[tolerance.measure]` names "the per-face work bound (one bound, in one unit, for every lane)". Ev's ruling on PR 6014 (Decision 2, option (i)) asks that the certified readers' work not depend on how many faces a surface is cut into. PR 6092 lays round-zero cells per knot span. That forces the choice:
- **A bound per face:** a whole multi-span patch that cannot meet its target stops short of its tiles. The whole quarter torus stops at `Wall` (pad 8.64e-4), while its 162 tiles meet the default resolution.
- **A bound per span cell:** whole and tiles agree, but a dense smooth face's work grows with its knot count. The 12+12-knot row at ε = 1e-12 went from 8.9 s to about 3000 s in release.

Asked of Ev on the `[ev]` PR that carries this row, after a designer pair weighed it over three rounds (`docs/DESIGN-FORK-LOG.md`, row 220). Until Ev rules, PR 6092 lands with the ratified per-face bound.

Signed: (QUAD orchestrator)

## Closed

Ev ruled on `[ev]` PR 6102 (2026-10-30/31). DESIGN `[tolerance.measure]` now says:
- **Budget:** a measurement refines its surface widest-first until its predicate resolves, refinement stops paying, or the work budget its caller states with the predicate runs out. The budget is a multiple of reading the measured surface once, with no default.
- **The kernel's own questions** (the validity gate, a boolean's decisions, a document's assertions) state a fixed constant per question. Ev: "fixed constant for now because it's easy and we can change it later".
- **Exhaustion:** an exhausted budget refuses (a deciding predicate) or returns its bracket (a resolution); it is never a coarser answer passed off as decided.
- **Resolution:** a resolution is read over the measured surface as a whole. It is only for a caller that wants the number; anything that compares the result passes that comparison as its predicate instead.

Built by `the-measurement-spends-a-question-budget-widest-first`.
