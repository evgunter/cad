---
id: a-face-base-puts-its-reference-on-local-y
kind: issue
title: A face base's in-plane axes put the carrier's reference on local +Y, so an offset along the face's reference is written along y
status: closed
opened: 2026-10-03
priority: P3
cost: M
design: true
closed: 2026-10-17
---


Found by PLACE's mate-frame-offset unit (branch
`place/mate-frame-offset`), writing the tour's crate the way a user
would.

## What

A face base (`MateFrame::on_face`, `crates/editor-core/src/mate.rs`)
is built through the witness ladder's `point_at_frame`
(`crates/editor-core/src/mate/solve.rs`, `face_base`): local +Z is the
carrier's chart axis and the roll reference — the carrier's `u_ref` —
lands on local **+Y**, with local +X their cross product
(`reference × axis`). So "slide the crate 0.2 m along the shelf" (the
shelf top cap's reference is +x) is written
`Frame::translation([0.0, 0.2, 0.0])` (`demos/tour/src/assembly.rs`,
`bench`). The same holds for authored vectors
(`MateFrame::authored` with axis +z, reference +x builds columns
`[[0,-1,0],[1,0,0],[0,0,1]]`).

Two more things an author must know and nothing names short of
interrogating the face (`names::interrogate::face_frame`):

- the frame's origin is the carrier's chart origin — for an extruded
  cap, the cap's centre, not the profile's first corner;
- a `Start` cap's chart runs axis and reference reversed (`-z`, `-x`
  on a prism extruded along +z), so the two caps of one prism meet
  square under `AxisSense::Opposed` with no turn.

## What it wants

Either a frame convention where the reference is local +X (a change to
what every authored frame means, so a design question and a re-baseline
of every mate frame's bits), or a statement of the convention where a
user meets it: `MateFrame`'s docs now say it, and the Python stub; the
guide states it once. Priority low: it is learnable, and it surprised
the first author who wrote an in-face offset.

## Re-pointed from the D10 hold (2026-10-08)

Waits on `intent-stage3-is-built`, not on the whole program: what a frame read off a face means (a Frame against a Plane pose that forgets in-plane motion) is stage 3's pose-kind work, which re-specifies face_base and point_at_frame. (INTENT's re-homing of the parked rows, `work/intent/log.md`.)

## Closed

Stage 3 C retires `FrameBase`: a value's reference direction is each body's own frame swung onto the mated direction (`reference_about`, ruling C-1), not a face base's local axis.
