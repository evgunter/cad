---
id: a-reflex-vertex-and-its-partner-read-the-same-b-sense-along-an-edge-through-the-corner
kind: issue
title: Along an edge of b through a's reflex corner, both B halves of a matched pair read up (JoinDesync B senses agree)
status: open
opened: 2026-10-02
priority: P1
cost: H
refs: [reflex-corner-vertex-vertex-sites-refuse-under-a-tilted-cap, locus-matching-moves-frontier-refusals-to-join-desync]
---


(Filed by the `join/pole-strut-binding` lane, the residue of
`locus-matching-moves-frontier-refusals-to-join-desync`. Evidence for
the reflex-corner P0 row, which owns the site class; absorb it there
if that row's diagnosis covers it.)

## What

`crates/sweep/tests/join1_r1_probes.rs` `join1_r1_reflex_battery`,
release: `eLeft` (`(0,−0.5) (1,−0.5) (1,0.5) (0,0.5)`, so its edge
`x = 0` runs through `a`'s reflex corner) at `(sx, sy) = (−0.5, 0)` and
`(−0.25, 0)`, `b ∖ a`, refuses `JoinDesync { "every chord arc
separates a loose scaffolding pair" }` on main `3ee0e4b6` and
`JoinDesync { "B senses agree at a matched pair" }` on JOIN-1's head
`0b6e39ca`. A kernel-bug refusal on both; 19 other poses of the battery
refuse the head's words on main already. The same two shears' `a ∩ b`
refuse `"every chord arc separates …"` on the head.

## Measured: the matched pair

Raising site `join.rs`, the matched-pair check after the mutual-facing
test (`B senses agree at a matched pair`). In `b ∖ a`, operand A is
`b` and operand B is `a`. At `(−0.5, 0)`:

- the pair's segment runs along `b`'s cap edge `x = 0` (both germs
  `a_locus = OnEdge`, `b_locus = InFace` of `a`'s top face), from
  `p_c = (0, 0.5, 1)` (a vertex of `b` on `a`'s top face) to
  `p_e = (0, 0, 1)` (`a`'s reflex corner, on that edge of `b`);
- A senses: `false` at `p_c`, `true` at `p_e` — opposed, as they must
  be;
- B senses: `true` at both. The anti-correlation theorem makes this a
  kernel bug.

The B half at `p_e` is minted at `a`'s reflex vertex, where `b`'s edge
passes through it: a vertex-on-edge site at a 315° corner, the site
class of the reflex-corner P0 row. Which of the two B halves is bound
wrong was not measured; the strut facing rule
(`insert::strut_facing`) reads only germs along the piercing solid's
own edges, and here the edge is `b`'s, the other operand's.

## The bar

`join1_r1_reflex_battery` reports no `JoinDesync`.

## Measured beside it (reflex-corner lane, PR 3900)

This row's poses are in the class of
`four-germ-vertex-pairs-run-b-in-a-order`. With the strut order fixed,
every `eLeft` pose at `sx < 0` (∩, ∪, `a ∖ b`) still refuses. The
first step that goes wrong is at the corner's vertex pair: it keeps
four germs, and `insert` runs one or both of B's null edges the long
way round in A's germ order. The `b ∖ a` op was not instrumented. A B
half bound to the wrong run reads the wrong sense, which would give
this row's symptom.
