---
id: canonical-segment-type-in-profile
kind: unit
title: Lower to the canonical segment form (verbatim vertices + Line | Arc{centre, radius, Δθ}) inside profile; byte-identical
status: open
opened: 2026-09-25
priority: P1
cost: H
parent: lower-profiles-to-carrier-and-interval-not-vertex-and-bulge
---


Unit 1 of the #3218 lowering (A2, ratified by Ev). Every profile-side reader of the bulge form moves to the canonical segment, except geom-brep (unit 2). Carriers are derived exactly as `seg::arc_carrier` derives them today, and Δθ = 4·atan b, so output is byte-identical. The `RawLoop` fixture door takes canonical segments. `ProfileVertex` retires. Fixtures migrate through a test-support helper that forwards to the algebra's `arc_to(Bulge)` lowering (Ev, #3218 q4). Survey: parent row, "Survey" §1a–1c, §3(a)(b)(d).
