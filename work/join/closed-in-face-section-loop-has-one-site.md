---
id: closed-in-face-section-loop-has-one-site
kind: issue
title: A closed operand conic lying in the partner's face makes a single-site closed section loop, which nothing in the join or the REST lane represents
status: open
opened: 2026-10-02
priority: P1
cost: H
refs: [an-edge-lying-in-a-cutter-face-past-its-end-wall-leaves-loose-ends-unpaired]
parent: JOIN-1
---


Found by JOIN's in-face measurement (2026-10-02, probe branch
`join/inface-probe`).

## What

The tube of `an-edge-lying-in-a-cutter-face-past-its-end-wall-leaves-loose-ends-unpaired`'s
"closed in-face conic" measurement (`crates/sweep/tests/germ_coplanar_conic.rs`
`every_op_refuses_or_answers_its_closed_form`) produces two section
loops with ONE site each:

- the lone vertex `(1, 0, 0)` of the circle `ρ = 1` that lies in B's
  top face (A's face 5 on both slots of the record);
- `(0.5, 0, 0)` on the inner wall's transverse section circle (A's face
  3 on both slots).

The labels agree, but each germ's only partner is the other slot of
its own record. `find_match` skips `entry == cand`, and the REST lane
skips germs of the same pair, so both remain loose:
`Join(UnpairedLooseEnds { count: 4 })` for ∪, ∖ and ∩. The inner circle
is not an in-face edge at all, so this is not the flank class.

## What the taker owes

A representation of a closed section loop with one site, in the join
(and in the REST lane where one is declared), or a typed refusal
naming it before the loose ends are counted. Weigh it with the
designers' answer on the flank fork, which may cover it.

## Built (JOIN-1, branch `join/1-germ-locus`)

The typed-refusal half: both loops now refuse
`Join(SingleSiteSectionLoop { count: 2 })` under ∪, ∖ and ∩, before
the loose ends are counted; pinned by
`crates/sweep/tests/germ_coplanar_conic.rs`
`a_closed_section_loop_with_one_site_refuses_typed`. The self-loop
arm (a record matching itself) is not built.
