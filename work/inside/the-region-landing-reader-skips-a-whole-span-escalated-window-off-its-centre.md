---
id: the-region-landing-reader-skips-a-whole-span-escalated-window-off-its-centre
kind: issue
title: region::landings skips an Escalated window whose centre lies off the leg, reading it as a root in band of an end, whatever the window's width
status: open
opened: 2026-11-25
---


Found by `inside/face-door-spline-arm` (the face door's spline arm).

## What

`crates/topo/src/region.rs`, `landings`: a `MeetRefusal::Windows` of
`Read::EdgeOverLeg` is passed to `set_aside_unless`, whose skip rule
reads every `CrossWhy::Method { fault: MeetFault::Escalated(_), .. }`
window as "a root in band of the edge's end, the window's centre" and
drops it when `edge.at(mid(window))` lies off the leg's section or past
its end. The rule holds for the placement's windows (`cross::place`'s
`PlaceWhy::End`, a band's reach about a root). It does not hold for a
window that is a whole span or a stretch: its centre says nothing about
where the meeting lies.

The ring composite's whole-span band refusal was `Escalated` on that
branch's first spelling, and the region read a tangent leg as no meeting:
`torus_region::tests::a_spline_patch_reaching_a_crest_reads_no_wrong_side`
answered a wrong side at `(0.021, π/2 + 10⁻⁹)`. The composite now maps it
to `MeetFault::Touch`, which the rule does not skip.

## Not checked

Whether another arm of `curve_face::cross` answers a whole-span or
wide `Escalated` window that reaches this rule: a method with nothing
local (`Walked::whole`, e.g. a line × torus ladder's escalation through
`RootRefusal`), or a pose rung's window (`CrossWhy::Pose`, not skipped).
The skip should read the window's kind (`PlaceWhy::End`), not its
`MeetFault`.
