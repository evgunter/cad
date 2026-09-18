---
id: culling-is-load-bearing-with-no-pixel-test
kind: issue
title: Which faces exist is now a rendering decision, and nothing in CI ever looks at a pixel
status: open
opened: 2026-09-04
---


## Finding

Issue #1097's hardware run settled `FrontFace::Ccw` and the scene and
id passes now set `cull_mode: Some(Face::Back)`. **That makes "which
faces exist" a rendering decision for the first time**, and no test in
this repo has ever looked at a rendered pixel.

**What changed, precisely.** With `cull_mode: None` an inverted patch
still drew — shaded oddly, because the normal comes from the vertex
order, but present and pickable. With culling on it is **absent**: the
scene pass drops it and the id pass drops it too, so the face is
neither visible nor selectable. The two passes stay consistent with
each other, which is what §2 of #1097 warned about and is correctly
handled; the new exposure is different. Absent geometry in a CAD
viewer reads as a MODELLING error, so a rendering fault now presents
as a kernel fault.

**Nothing would catch it.** `gpu::tests::every_pass_builds_on_a_real_device`
(PR 1755) creates a device on the software adapter and constructs every
pass — deliberately no surface, no frame, no pixels asserted. So the
whole family downstream of a frame is uncovered, which Ev's own run
notes: it exercised buffer and texture allocation, render-pass
encoding and the id pass's readback, *"the family §4 said still escaped
CI after #1755, since each needs a frame rather than a constructor."*

## Why this is buildable rather than a lament

CI already has the lavapipe stack that smoke row runs on. A row that
renders ONE frame of a known body and reads back either a pixel or the
id buffer would close it — the id buffer is the easier target, being
`R32Uint` and already copied to a mappable buffer by the existing
readback path. A body whose front and back faces both cover the cursor
distinguishes culled from not-culled in one sample.

Scope note for a taker: this is the first row in the crate that would
assert something about a rendered image, so it also decides the
convention for that kind of row.

## Sibling

`work/chrome/chrome-weight-is-outside-the-palette` is the same class
one layer up — a semantic distinction carried in badge WEIGHT that no
test sees, because no test sees any badge's colour either. Two visual
properties are now load-bearing with nothing watching; a taker of
either should read both.

## Home

`work/chrome/` — `crates/viewer/src/gpu.rs` and
`crates/viewer/tests/` are this program's ground.

Opened from issue #1097's hardware run (Ev, 2026-09-04).
