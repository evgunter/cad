---
id: an-empty-outline-on-a-face-refuses-with-a-recourse-about-mating-a-part
kind: issue
title: A profile drawn on a face with no loops refuses face_chart_unleverable, with a recourse to mate a part, where a profile with no loops elsewhere evaluates as the empty region
status: open
opened: 2026-11-22
priority: P3
cost: E
---

Found by VSEAM's review of PR 7465 (2026-11-22). Filed by the VSEAM orchestrator. The old fused binding behaved the same way.

`Doc.draw_on_face(body, face, [])` is accepted at the door and refuses at `evaluate` as `face_chart_unleverable`: the chart "has no faces… Recourse: mate a part that has a body". `Node.profile([])` evaluates, as the empty region.

The viewer README says "a profile with no loops is the empty region: it evaluates and draws nothing", and the viewer's New profile on a face inserts exactly such a profile. Check whether the GUI's empty on-face profile meets this refusal too.

Two things are wrong:
- the recourse sentence does not describe the cause;
- the chart's leverability depends on the loops, which a chart should not.
