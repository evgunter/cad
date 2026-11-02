---
id: a-cone-against-a-parallel-axis-torus-has-no-section-arm
kind: issue
title: A cone whose axis is parallel to a torus's and apart refuses on reach: the parallel-tori arm's mirror construction has no cone sibling
status: open
opened: 2026-10-27
priority: P3
cost: M
---


Found by SECT's lily lane (`sect/lily-parallel-tori-certificate`) in its
sweep of the section certificate's `Intractable` arms, 2026-10-27.

## Measured

`boolean/section_cert.rs` `torus_pair`'s cone arm reads
`Pose::Parallel { .. } | Pose::Other => Section::Intractable`: a cone
whose axis is parallel to the torus's and apart has no arm, so a pair
whose boxes overlap refuses R-reach.

## What the taker owes

`torus_parallel_torus` reads two tori on parallel axes by the reflection
through the plane `Π` of the axes: the section meets `Π` at the
crossings of the two meridian traces there, and the curve matching the
two meridian traces' heights carries the components. A cone's meridian
trace is a pair of lines, so the same construction holds with the
cone's two nappe sheets spanning every height (the heights' span is the
torus's alone). Build that arm, pinned against a traced section.

Moved from SECTTAIL to SECT on 2026-11-02, by `git mv`, when SECTTAIL merged back into SECT: the track budget now counts only the P0, P1 and unbanded rows a sitting dispatches (Ev, in chat, 2026-11-02), so SECT and the three tracks cut from it (SECTTAIL, SECTRUNG, SECTSITE) came in under one budget.
