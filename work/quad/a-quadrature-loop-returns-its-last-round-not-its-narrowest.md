---
id: a-quadrature-loop-returns-its-last-round-not-its-narrowest
kind: issue
title: Past the arithmetic floor a later round can be wider than an earlier one, and the loops return the last round's bracket
status: closed
opened: 2026-10-21
priority: P2
cost: E
closed: 2026-10-23
pr: 5628
---


Measured by a designer weighing `quad2-rational-max-rounds-dial-decision`
(2026-10-21), on the two-span half cylinder with the rational cap raised
locally. Rounds 9–13 quarter (ratio 4.00, 3.99, 3.93, 3.48), round 14
narrows by 1.87× only, and round 15 is WIDER than round 14 (2.30e-10 m
against 1.99e-10). `props::quad`'s `round_exit` returns the current
round's bounds, so a loop that reaches the floor hands back a wider
bracket than one it already held.

It cannot be reached at today's caps (the ratio is 3.98 through round 9),
but it becomes reachable the day a cap or a work bound admits about round
14 on the v-exact arm. Whatever lands the stop rule (`[ev]` PR on the
dial row) returns the narrowest round and pins this half cylinder past
round 14.

## Closed

PR 5589 made every lane answer its narrowest round and `FaceRun::advance` keep the narrower across windows. PR 5628 routes the last remaining replacement, the certificate's continuation (`continue_to_target`, which overwrote a face with its resumed window), through `advance` too. So no walk hands back a wider round than one it held.
