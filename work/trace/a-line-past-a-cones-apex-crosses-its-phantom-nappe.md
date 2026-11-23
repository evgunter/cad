---
id: a-line-past-a-cones-apex-crosses-its-phantom-nappe
kind: issue
title: A piece past a cone's apex crosses the carrier's other nappe, which the face does not hold
status: open
opened: 2026-10-29
priority: P1
cost: M
---


Found by INTENT stage 6 PR C's review (`work/intent/a-sketch-continues-tangent-to-a-surfaces-trace.md`, PR #6043). The orchestrator's note asked for one probe of the cone arm. The far generator's crossing is certified: a line across a revolved cone above its apex enters at `x = -1/2` and leaves at `x = 1/2`, exactly.

A cone's carrier is a double cone: its implicit residual vanishes on both nappes (`plane_misses`, `crates/topo/src/boolean/trace.rs:505`, "every cone, whose locus is double"). The trace door reads the carrier, not the face (its module docs). So a line drawn past the apex, across the nappe the face does not hold, also enters at `x = -1/2` and leaves at `x = 1/2`.

The probe: a triangle `(0,0)-(1,1)-(0,1)` revolved about `y`, traced on the frame through its axis; lines from `(-2, ±1/2)` along `+x`. The door's answer agrees with its documentation, but a sketch running onto a crossing there runs onto no point of the face.

**Done when** a cone's crossings are only those on the face's nappe, the apex's side read from the face's own parameter range, and a row shows a line past the apex finding none.
