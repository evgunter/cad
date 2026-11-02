---
id: an-arc-runs-to-a-crossing-only-about-a-centre
kind: issue
title: A trace's crossing is reached only by a straight leg or an arc about a written centre, and a written coordinate only by a straight leg
status: open
opened: 2026-10-29
priority: P3
cost: M
---


Found by INTENT stage 6 PR C's review (`work/intent/a-sketch-continues-tangent-to-a-surfaces-trace.md`, PR #6043). Departure 4 of #6006's ruling.

`editor_core::trace::resolve_chain` (`crates/editor-core/src/trace.rs:295`)
lowers three forms that run from the pen:
- a straight leg (`line_to`, `continue_to`) to a crossing or to a written `x` or `y` (`:388`);
- an arc about a written centre (`arc_to(Center)`) to a crossing.

Every other verb with a trace target refuses `NotARun` (`:423`): a tangent arc, an arc onto, a fillet's spec, a bulge or via arc. An arc to a written coordinate refuses too.

Each missing form has a closed form against the carriers the door reads, as `topo::boolean::trace::circle_crossings` does for a centre arc. What each one lacks is the arc's circle, written before its far end is known.
- **A tangent arc** has its centre on the incoming normal, at an unknown radius.
- **A bulge or via arc** names its far end.

**Done when** each form either lowers, with a row in
`crates/editor-core/tests/intent_s6_c_trace.rs`, or refuses with a fault
that says why it cannot (`NotARun` says only that the verb does not run).
