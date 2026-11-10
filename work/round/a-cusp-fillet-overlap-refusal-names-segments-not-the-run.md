---
id: a-cusp-fillet-overlap-refusal-names-segments-not-the-run
kind: issue
title: line(L).cusp().fillet(r) refuses at validation as an overlap between segment 0 and segment 1, naming indices rather than the cusp or the fillet's run-in
status: open
opened: 2026-11-06
priority: P3
cost: E
rides_with: anchor-fit-refusal-reports-a-setback-excess-not-a-radius-reduction
---


**Found by PR 6545's review (ROUND, 2026-11-06).** After the fix, a
`.cusp()` before a fillet on a straight leg emits its reversed run as
its own segment, which always overlaps the leg, so validation refuses
"profile is not simple: overlap between loop 0 segment 0 and loop 0
segment 1". The editor can map segment 1 to the fillet's `RunIn`
locator; a bare-API user reads indices. `.cusp().line(..)` back over a
leg reads the same way, so the question is whether the path door should
refuse the reversed ray-extension run typed before validation (a new
`PathError` variant naming the cusp and the run), or whether the
validator's overlap text should name the steps that wrote each segment.
