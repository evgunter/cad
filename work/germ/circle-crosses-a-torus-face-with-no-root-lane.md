---
id: circle-crosses-a-torus-face-with-no-root-lane
kind: issue
title: A circle edge against a torus face has no root lane: the lily's stem seam refuses CurvedPierceUnsupported at the arch's torus wall
status: dispatched
opened: 2026-09-26
priority: P1
cost: H
refs: [torus-operand-gate-admission]
---


## What

Sub-item (b) of `torus-operand-gate-admission`, left open when the rest of
that row landed (PR 3265). The arch's outer-equator seam crosses the stem's
carrier at 29° along the arch, outside the stem face's window. With the
torus admitted, the lily's wall 1 now stops there as a circle×torus
`CurvedPierceUnsupported`. That is an undecided enclosure: there is no root
lane for a circle against a torus. The item's honest negative certificate
maps the arc into the stem's chart.

## Home

GERM: the torus operand lane. `wall_crossing` answers line×torus since
PR 3265, and this is the circle×torus crossing beside it.
