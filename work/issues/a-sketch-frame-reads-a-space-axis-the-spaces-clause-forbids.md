---
id: a-sketch-frame-reads-a-space-axis-the-spaces-clause-forbids
kind: issue
title: The bare-disc sketch frame (#6298) reads a space's x, which [variables.kinds] says only placement values do
status: open
opened: 2026-11-03
---


Found by FORMS' fork row 247, round 3: designers A, B and C each found it independently.

Two texts in `docs/DESIGN.md` disagree:
- `[variables.kinds]` (Spaces) says a space holds "no origin, axis or plane that a reader takes as a pose … the values of placements into a space are the only readers of its axes". `[placement.values]` says "A space's axes are read only by values". Both were written by the D′ commit `679b4abbf7` on `[ev]` 5494, as designer text that Ev approved in passing.
- The later `[placement]` sketch ruling on `[ev]` 6298 (Ev, "ok sure", 2026-11-02) gives a sketch on a bare disc an x that is "the space's x carried onto the face by the least turn". That is a pose reading a space's axis, outside any placement.

The spaces clause was not updated. On `[ev]` 6406 (2026-11-03), Ev declined to add a space-frame pose form ("we should only add this if we need it"), so 6406 leaves the clause alone. The contradiction stands: one of the two texts needs another spelling.
(FORMS orchestrator)
