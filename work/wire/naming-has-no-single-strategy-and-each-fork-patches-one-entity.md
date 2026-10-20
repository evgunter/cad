---
id: naming-has-no-single-strategy-and-each-fork-patches-one-entity
kind: issue
title: Names have no single strategy: each naming fork patches one entity kind, and the same leak (build order, slot, index, frame) keeps recurring
status: dispatched
opened: 2026-10-20
priority: P1
cost: H
design: true
---


Ev on PR 5426 (2026-10-20): "it seems like we are still in general
missing a good consistent strategy for names in a way that keeps causing
problems".

## The recurrence (WIRE's naming forks, 2026-10-17 to 20)

| fork-log row | entity | what leaked into the name |
|---|---|---|
| 158 FORK-CIRCLEPHASE | a split circle's pieces | the circle's phase, a frame choice |
| 160 FORK-SPANCHORD | a chord across merged constituents | no name: the rule covered one constituent |
| 163 FORK-INPLANE | an in-plane operand edge | its own name died: build classified it as a chord |
| 169 FORK-ROWPRESENCE | an n-ary union's coincidence rows | fold (member) order |
| 178 FORK-PAIRSEAM | a pair boolean's seams and crossings | operand slot; bare names collided |
| 179 FORK-SECTIONID | a split's section faces | completion (sweep) order, a plane frame |

Each fork was settled for its own entity kind. The shared shape: a name
read something the author did not write and the operands' names do not
fix (an order, a slot, an index, a frame) or failed to read something
they do.

## What is owed

1. A survey of every naming defect in `work/` (open and closed), grouped
   by what leaked, to test whether the shape above covers them.
2. A strategy stated once, as a principle the N-clauses instantiate rather
   than a per-entity rule, if the survey supports one.
3. A mechanical check that finds the class before an issue does. For
   example, a property test that perturbs what a name must not read (member
   order, operand order for symmetric ops, the plane frame, a tilt within
   one topology, a Transform copy of an operand) and asserts the name
   table is invariant up to declared ties.

The strategy is Ev's to ratify (an `[ev]` PR); the survey and the check
are not.
