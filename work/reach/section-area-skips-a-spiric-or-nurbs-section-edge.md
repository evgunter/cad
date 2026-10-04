---
id: section-area-skips-a-spiric-or-nurbs-section-edge
kind: issue
title: certify_section_area reads a spiric or NURBS section edge as its chord
status: open
opened: 2026-10-03
priority: P3
cost: E
refs: [planar-crossing-lane-reads-a-curved-carrier-as-a-line]
---


Filed from the sweep of `planar-crossing-lane-reads-a-curved-carrier-as-a-line`
(the "curved carrier read as a line" class).

## Finding

`crates/topo/src/splitting/join.rs` `certify_section_area` walks a
section loop and adds each conic edge's segment correction; an edge
whose carrier `loop_winding::ConicFrame::of` does not frame
(`None`, a line, a spiric or a NURBS) is skipped with `continue`, so a
spiric or NURBS section edge is measured as its straight chord in the
shoelace sum. A line's chord is exact; a curved carrier's is not, and
nothing refuses.

**Latent, not live**: a section edge lies in the split plane, and the
split gate (`splitting::classify::gate_operand`) refuses every spiric or
NURBS edge the plane meets. The site does not refuse on its own, so it
turns into a silent wrong area the day the gate narrows.

## Fix

Match the carrier kind: `Line` takes no correction, a conic takes its
term, and a spiric or NURBS edge refuses typed (or gets a term of its
own), with a row that reds on the chord.
