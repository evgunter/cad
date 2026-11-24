---
id: pierce-and-rim-balls-are-built-from-an-arm
kind: issue
title: A pierce's and a planar rim's extent balls are built from an arm, not from the runs' geometry
status: open
opened: 2026-11-23
priority: P1
cost: M
---

Two extent balls are built from an arm through `ExtentBall::unsourced`, a region made from a lever:

- `topo::boolean::vtxfac::held_reads`: `ConsumedExtent::new(ExtentBall::unsourced(p, arm.length()))`, `arm` being the shortest arm the pierce's runs were ordered at (`PlannedRun.order`). The arm is then used as a lever as well (`PierceLever::of(..).within(arm.length())`), which is a margin's business.
- `sweep::blend::open::planar` (near line 961): `ExtentBall::unsourced(p, l1.arm_len.max(l2.arm_len).length())` (band's ground; listed here so the class is in one place).

`PlannedRun.order` holds a lever and no region, so the honest answer (a ball read off the runs' edges, `edge_cover`'s far distance from `p`) needs the plan's record to hold its runs' geometry. The `sizes-are-read-back-out-of-extent-levers` unit left this site for that reason.
