---
id: a-cone-boss-over-a-hole-tangent-at-its-rim-refuses-in-every-pose
kind: issue
title: A cone boss over a hole whose rim is tangent to the hole's refuses CurvedPierceUnsupported in every op, whatever the vertices' azimuths
status: open
opened: 2026-10-26
priority: P2
cost: M
---

Found sweeping `one-segment-tangent-rims-refuse-where-their-two-arc-forms-build`
for its class: a rim touching another rim where the crossing layer
cannot settle the tangency.

## The refusal

A frustum boss (`sweep::test_support::revolved_about_y` of
`(0, 0), (0.3, 0), (0.2, 1), (0, 1)`, turned so its axis is `z`; its
base rim one closed edge with its vertex at `(0.3, 0, 0)`) over the
plate `[−1, 1]² × [−0.5, 0]` holed by a circle `r = 0.1` about
`(±0.4, 0)`, tangent outside the boss's base rim. Every op in both
orders refuses `CurvedPierceUnsupported`, in the hole's one-segment and
two-arc forms alike, with a vertex of either rim at the tangent point or
of neither (measured at the default ε; probe not committed).

## Why

The frustum's wall is a cone, which is not tangent to the hole's
cylinder along a line, so neither rim is covered against the other's
wall (`boolean/reduce.rs` `curved_face_arm`'s one-sided cover). Each
rim's residual against the other wall touches zero at the tangent point,
the certified roots answer a tangency (`wall_crossing`'s
`Some(Err(_)) | None` arm), `carrier_touch::off_face` cannot place the
touch off the face (it lies on the face's boundary), and the pair reads
`SpanVerdict::Unsettled`, the frontier. The covered case — a cylinder
boss, the walls tangent along a ruling — is read through
`reduce.rs` `interior` and `sheets_part`, which split both rims at the
touch; the uncovered arm asks no boundary question. A vertex at the
point does not help: the endpoint arms meet the same unsettled tangency
on the fragments.

## What it needs

A tangency the roots cannot settle, whose touch lies on the face's
boundary (`carrier_cross::boundary_crossing` reads the rims' meeting as
a curve × curve touch), is a meeting both edges can be split at, as
`interior` does for a covered edge. What stays to show is that the
fragments, each ending on the touch, then read their endpoint arms
without the tangency refusing again, and that the walls part there
(`sheets_part`'s reading), so the split does not reach a crescent of
walls the sweep cannot read (`face_face_touch.rs`
`a_nested_disc_with_its_vertices_off_the_line_refuses_its_mid_arc_touch`).

## Slate triage

2026-11-01: moved from `work/cleave/` by CLEAVE's split on its semantic seams (Ev, in chat: CLEAVE measured 114 budget points against 30). TANGRIM holds the tangent curved rims and crescents; id and body unchanged. (CLEAVE orchestrator)
