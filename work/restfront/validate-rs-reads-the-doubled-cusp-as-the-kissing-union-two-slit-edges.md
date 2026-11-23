---
id: validate-rs-reads-the-doubled-cusp-as-the-kissing-union-two-slit-edges
kind: issue
title: validate.rs's deferred list still reads the kissing union as the doubled cusp's two coincident edges, which PRs 4914 and 4795 superseded
status: open
opened: 2026-10-19
priority: P4
cost: E
---


Found by both designers of the pinch-crescent fork (`[ev]` branch
`contacthold/ev-pinch-crescent`), off its question.

## What

`validate_geometric`'s deferred list (`crates/topo/src/validate.rs:5032`)
says:

> **The doubled cusp** (two material wedges on one tangent line — the
> kissing union, a slit interior to material) is not one 4-face edge
> but F2's coincident-distinct-edges class …

That is #131's wording (2026-08-23). Since then PR 4914 (2026-10-12)
ruled that a line where no edge lies is a touch between faces,
recorded like any touch, with no op cutting edges to make a junction
of it, so the kissing union (a rod resting on a plate) is two solids
and a face–face record (C7), not two coincident slit edges. PR 4795
(2026-10-19) restated the paragraph as a principle (each creased
material sector between consecutive sheets is one edge) and kept that
clause. The bullet still names the kissing union as the doubled cusp's
example.

The same superseded reading is in CLEAVE's closed
`work/cleave/tier-3-passes-a-curved-wall-touching-a-plane-face-interior-along-a-line.md`
("Designed (2026-10-03)"), which now carries a note saying so.

## Owed

The bullet rewritten from D1 tier 3's current text: what tier 3 defers
for coincident distinct edges, with no kissing-union example. If the
pinch-crescent fork rules that a material pinch is two cusp edges, that
is the bullet's example. It did (Ev, `[ev]` PR 5395, 2026-10-20): plate
∖ a bore tangent to its face from inside is the example.
