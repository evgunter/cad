---
id: the-cone-apex-refusal-zone-is-a-frontier
kind: issue
title: The cone root lanes refuse a wide zone about the apex, growing with scale: a frontier VERBS-CONE consumers will meet
status: open
opened: 2026-10-06
priority: P3
cost: H
refs: [4135]
---


Left by the last fix pass of PR 4135 (the cone root lane), from both
reviews (r1 NOTE-1, r2 n-2). Sound — no answer near the apex was found
wrong — but wide, and growing with scale.

## What

Measured nearest answers to the apex:

- **Conics** (r1): circles passing `d` from the apex are never answered
  at `d ≤ 1e4·ε` at any scale; at scale 1 the first certified answers
  appear at 1e5–1e6 ε (ε 1e-9) and 1e9 ε (ε 1e-12). r2: nearest answer
  at 8e-5…5e-3·scale, some poses none up to 5e-3·scale; at ε 1e-12,
  14 % of clear random conics refuse.
- **Lines** (r2, ε 1e-9): 4e-6 m at scale 1e-3, 2.5e-4 m at 1, 8–60 m at
  1e3. r1: the line arm answers `AtApex` on about a third of apex-class
  draws and never certifies a root within ε of the apex.

Two levers set the conic zone (the last fix pass's measurements):

- The cone's root-slack meter (`bool_conic_cone_root_slack`,
  `crates/topo/src/boolean/conic_quadric/mod.rs:371`) reads the
  root's slope as `F′` through the ceiling `1`
  (`crates/topo/src/boolean/circle_roots.rs:757`), and near the apex
  `|F′| ≤ 2|q|·speed/R`, far below the residual's own slope on a
  transversal crossing. A circle of radius 1 crossing a right cone
  transversally 1 µm from its apex is charged 8.9e-10 m at ε 1e-12
  while its true root error is about 1e-16 m
  (`cone_rows::a_root_the_slack_meter_cannot_place_refuses` pins that
  pose's refusal).
- The subdivision cannot read a definite side within the escalation
  band of the apex, so a root closer than about ten bands is never
  isolated at ε 1e-6; at 40 000 circles with roots 1–10 ε from the apex,
  none was certified at any ε.

The line zone is set by the depth rung's reach lever
(`the-line-cone-depth-rung-is-levered-by-the-carriers-reach`).

## The fix owed

A decision on how close to the apex `VERBS-CONE` must answer, then the
levers above re-read against it (the slack meter on the residual's own
slope, where it is known to bound `F′/R`), each with a row that
answers nearer the apex and stays right against an oracle.
