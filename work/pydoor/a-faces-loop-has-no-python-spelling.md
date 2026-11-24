---
id: a-faces-loop-has-no-python-spelling
kind: unit
title: A profile loop that is a face's outline or hole has no Python spelling
status: parked
opened: 2026-11-22
priority: P2
cost: E
parent: a-faces-loop-or-closed-trace-cannot-become-a-profile-loop
blocked_on: [a-profile-on-a-face-holds-that-faces-outline-or-a-hole]
---


Filed by PROFILE's cut of the face-loop ruling (PR 7284), for FACELOOP
(`work/faceloop/plan.md`). Once FACELOOP's U2,
`a-profile-on-a-face-holds-that-faces-outline-or-a-hole`, lands
`LoopProgram::Boundary(Outer | Through(edge))`, a Rust caller can write
a profile loop that is the outline of the face the profile is drawn on,
or the hole through a named edge. A Python caller cannot: the loop
vocabulary in `pncad-py` (`crates/pncad-py/src/py/step.rs` and the
`.pyi`) has no spelling for it. U2 only adds the match arm its new
`ProfileEdgeRef` variant forces in `step.rs`.

Done when a Python profile drawn on a face can hold `Outer` and
`Through(edge)`, with U2's typed refusals reaching Python as tags, and
the elbow (a pipe, its end cap's two loops, a revolve) is one doctest.

P2: library breadth, and the GUI's click (VSEAM's
`a-click-on-an-edge-of-the-sketched-face-writes-its-loop`) is the
everyday door.
