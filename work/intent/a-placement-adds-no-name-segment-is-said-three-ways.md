---
id: a-placement-adds-no-name-segment-is-said-three-ways
kind: issue
title: N1's scope clause, N4 and the open placement-copy row say three different things about RoleSeg::Placed and whether a placement adds a name segment
status: open
opened: 2026-10-20
priority: P3
cost: E
---


Found by WIRE's naming survey (`wire-namesurvey`, 2026-10-20), off its
question, against main's text at `3b687e1652`:
- `crates/editor-core/src/names/README.md` **N4**: a world placement
  qualifies names by `RoleSeg::Placed`.
- **N1**'s scope clause: `Place [P1,P2]` would be refused `DuplicateName`.
- N1, DM4 and N6 elsewhere: "a placement adds no name segment".
- `work/intent/a-placement-copy-is-its-own-table-in-the-product.md`
  (open) retires `Placed`.

Likely a live inconsistency. It is D10 ground, so it is INTENT's to settle;
WIRE's naming-strategy row
(`work/wire/naming-has-no-single-strategy-and-each-fork-patches-one-entity.md`)
will cite whatever INTENT rules.
