---
id: a-straddling-span-whose-roots-do-not-settle-keeps-its-door-off-the-face
kind: issue
title: A span whose ends straddle a carrier and whose roots do not settle keeps the pierce door though every touch is certified off the face
status: open
opened: 2026-10-06
priority: P3
cost: E
refs: [4128]
---


Found by `reach/pierce-tangent-off-face`, which reads a root set its
door could not settle against the face
(`crates/topo/src/boolean/carrier_touch.rs`, `off_face`) and answers
`SpanVerdict::OffFace` when every touch is certified off it.

The straddle arm of `reduce::curved_face_arm` (the
`(Sign::Positive, Sign::Negative)` arm, `reduce.rs:2389`) still sends
`OffFace` to the frontier with every verdict other than `Pierce` and
`Elsewhere`. Its contradiction posture is for `NoInterior`, a root set
that finds no crossing where the ends promise one. `OffFace` claims no
such absence: it says where the crossing can be, and that is off the
face, so `None` is the answer the localization's argument licenses.

Unreached today, which is why it was not taken with the unit: on a
sphere or a wall a definite straddle forces a definite discriminant,
so the roots settle. Only a line or a conic against a torus can
straddle with an uncertain count (a crossing beside a graze), and no
fixture found builds that pose clear of the face. The fix is one arm;
the row is the work.
