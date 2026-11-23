---
id: a-traced-profile-is-tangent-at-a-point-where-its-plane-holds-a-symmetry-axis
kind: issue
title: A traced profile is tangent at the crossing where its frame's plane contains an axis of the face's symmetry: the point tier of the trace statement is ratified (#6006) and unbuilt
status: open
opened: 2026-10-29
priority: P3
cost: M
---


`[coincidence.structural]`'s rung one, as ratified on #6006, includes the point tier. A piece drawn along a face's trace, whose frame's plane contains an axis of the face's symmetry, is tangent to the face at the crossing. Ev: "B is probably closer to what we want eventually? but it's fine to defer actually building it".

Stage 6 D builds the curve tier only. This row builds the point tier once the kernel records a single-point touch as a coincidence row. Today it records none: stage 6 A's scenes have no point-contact row, so there is nothing for the door to prove.

Done when a scene's point-contact row (e.g. a great circle's tangent line extruded normal to the sketch plane, touching a sphere) is proven at rung one by this statement, and a row whose plane misses every symmetry axis stays unproven.
