---
id: sphere-operand-box-is-the-whole-ball
kind: issue
title: A sphere face's operand box is the whole ball - the same per-kind box class the cone, cylinder and torus arms have left
status: open
opened: 2026-09-06
refs: [torus-operand-boxes-span-whole-ring, 1907]
---


## What

`FaceBoxRule::WholeBall` boxes every sphere face as the whole ball
(`boxes.rs`), reading nothing from the boundary — the class the
VERBS-GATE cone clip, the cylinder's `clip_to_boundary` and the torus
window (PR #1907) each retired for their kind. Named-and-deferred in
#1907's sweep with no file; this is the file. A sphere face's chart
window from its stored `Harmonic` images is the torus construction
with one channel (the sphere has no interior critical point in `v`
beyond the poles, which the walk's pole joint already handles).

## Home

CURVED — the operand boxes are the operand-reach lane's.
