---
id: a-ring-partner-inside-the-ring-unions-result-invalid
kind: issue
title: The plate with a ring ∪ a flush partner whose edge lies wholly inside the ring, plate first, refuses ResultInvalid
status: closed
closed: 2026-10-27
pr: 5812
opened: 2026-10-25
priority: P1
cost: M
branch: cleave/flush-partners-at-a-slit-build
---


Found by PR 5545's review probe (`cleave/fate-cut-ends-at-a-slit-tip`);
the same on main `42af7c0b48` and on the branch, at ε default, 1e-6
and 1e-12, at rest, turned, and flipped and turned.

## What

`y` is the plate less the pyramid lying under its top, its top carrying
the ray from `MEET` along bearing 60 as a zero-area ring of length 0.5.
`x` is the tetrahedron with apex `MEET + bearing(60, 0.1, 0)` over
`[bearing(60, 0.3, 0), bearing(150, 0.6, 0), bearing(20, 0.6, −0.7)]`
(and its mirror, `bearing(−30, 0.6, 0)`, `bearing(100, 0.6, −0.7)`), so
its edge along the ray, from 0.1 to 0.4, lies wholly inside the ring,
and one of its faces lies flush with the top. `y ∪ x` refuses
`ResultInvalid`; the other five ops build, tier 3 and 3′.
`ResultInvalid` claims a kernel defect.

## Owed

Build it, or refuse it typed before the result gate.

## Cause

The union keeps the top on both sides of the partner's stretch of the
ray and drops the walls there, so the zip leaves that stretch an edge
of the top on both sides, between stretches of the ring the union
keeps. Plate first, the zip's `kill_pair` (`boolean/zip.rs:1027`) reads
it as a seam edge whose pair is no section strut and does not report
it, and the output stage closed only the struts the zip reported
(`ops::finish_output`, `boolean/ops.rs:2946`): the edge stayed, never
described, and the result gate refused `ScaffoldAtRest`. Where the zip
did report it (the stretch from 0.2), `slit::close_struts`
(`boolean/slit.rs:650`) refused `SlitFaceClosesMidSlit`: the strut has
no free end, and parting the ring was not built.

## Closed

2026-10-27, PR 5812. `finish_output` hands the closing the seam edges too, and
`slit::part_mid_slit` deletes the edge left there, a stretch of the
ring closed across with no free end and so no strut, with `kemr`: the
loop parts in two, and the part that is the slit's run becomes a new
ring of the face. The union's top holds two rings on the ray, from
the apex to the partner's stretch and from its end to the tip. Every op
builds, in both orders, as the closed form says, tier 3 and 3′, its
edges on the line the material's creases, the partner's edge from 0.1
to 0.3 or 0.4 and from 0.2 to 0.4, on either side of the ray, at ε
default, 1e-6 and 1e-12, at rest, turned and flipped and turned.
Pinned by
`a_ring_crossed_along_its_line_places_its_germs_apart::a_partner_inside_the_ring_parts_it_in_the_union`.
