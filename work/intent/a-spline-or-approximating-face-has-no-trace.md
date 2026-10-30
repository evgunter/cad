---
id: a-spline-or-approximating-face-has-no-trace
kind: issue
title: A face on a spline or approximating carrier has no trace: a step reading one refuses unsupported_carrier
status: open
opened: 2026-10-29
priority: P3
---


Found by INTENT stage 6 PR C's review (`work/intent/a-sketch-continues-tangent-to-a-surfaces-trace.md`, PR #6043).

The trace door reads only closed-form carriers. A spline or approximating carrier refuses `TraceFault::Unsupported` (`crates/topo/src/boolean/trace.rs:193`, `:276`, `:448`, `:506`). A step reading such a face therefore refuses `unsupported_carrier`: a twisted loft's wall, in `intent_s6_c_trace::the_pen_the_carrier_and_the_band_refuse_typed`.

The door's piece is a line or a circle. Its crossing with a spline face is a root of the spline's residual along the piece, which the boolean's spline arms certify for an edge. The trace door does not reach them.

**Done when** a straight piece and a centre arc read a spline face's crossings through the boolean's certified spline doors, or the refusal names that door as its recourse.
