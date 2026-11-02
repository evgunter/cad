---
id: the-volume-backstops-plus-v-posture-reads-a-sign-the-side-reader-cannot
kind: issue
title: The volume backstop's vol(R) ≥ 0 posture reads tier 3's +V sign off a summed volume, which the side-at-infinity reader cannot answer
status: closed
opened: 2026-10-14
priority: P3
cost: E
pr: 5842
closed: 2026-10-27
---


Found by PR 5119's review (FLUSH `flush/shell-role-reader`), item 9(b)
of its fix pass: every other volume-sign-to-role read now goes through
one home (`props::role_walk`, read by the side-at-infinity reader's
volume fallbacks), and this one does not.

## Measured

`crates/topo/src/boolean/ops.rs` `volume_backstop` decides
`vol(A ∩ B) ≥ 0` (and the other ops' `≥ 0` rows) under
`Posture::PlusV`. `Posture::read` turns that inequality's margin into a
role through `crate::validate::plus_v_read` (`read_role` under check 7's
`PLUS_V` names): `Void` is a violation, `Outer` holds, and an open
reading is exempt.

## Why it is not routed through the reader

The reader (`crate::side_reader::side_at_infinity`) answers which side
of ONE selection of faces lies at infinity: a shell's role, a solid's
orientation. The backstop's margin is not a side. It is the summed
volume of every solid of the result, `Σ vol`, an inequality's
comparand, whose sign says nothing about any one boundary. A result of
two solids, one inside out and one larger and right side out, sums
positive; the reader reads the first `In`. So the reader cannot answer
this row, and `plus_v_read` only borrows check 7's words for the sign of
a number.

## What to find

Whether the `≥ 0` rows still earn their place now that check 7 reads
every solid of the result through the reader at the door
(`finished_body_refusal`). If they do, the posture should read its sign
under its own name (`volume_backstop_*`) and not through
`validate::plus_v_read`, so that check 7's words have one reader.

## Closed (2026-10-27)

The volume backstop left the boolean's operation path (`[ev]` PR 5780: the volume relations are a test oracle the suites run on their own results, `topo::test_support::volume_relations`), so nothing refuses or passes on it. The +V posture went with it, and `validate::plus_v_read` and `props::read_role`, its only readers, are deleted; every volume-sign-to-role read goes through `props::role_walk`.
