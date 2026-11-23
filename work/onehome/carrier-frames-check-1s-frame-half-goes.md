---
id: carrier-frames-check-1s-frame-half-goes
kind: unit
title: Carrier frames (iii): check 1's frame half, frame_defect, unit_defect and the projected lane's frame check go, with the code docs and the bounds_allowlist entry
status: closed
opened: 2026-11-17
priority: P1
cost: M
parent: carriers-hold-their-frame-as-unitvec3-and-rigidmotion-and-check-1s-frame-half-goes
blocked_on: [carrier-frames-the-types-and-the-lift]
closed: 2026-11-22
pr: 7323
branch: onehome/check1-frame-half
---


Rides on `carrier-frames-the-types-and-the-lift`. These go, as the
parent row lists them:

- `geom::convention::frame_margins` and the `representability_margins`
  frame terms built on it;
- `pcurve_cache::frame_defect` and its `EnvelopeTerm::Frame` charge;
- the projected lane's frame check;
- `implicit.rs`'s `unit_defect` reads of `u_ref` and `axis`.

These code docs change with it:

- `geom`'s at-rest paragraph;
- the "What is NOT this type" paragraphs of `UnitVec3` and
  `RigidMotion`;
- the `bounds_allowlist` entry that admits the frame read, checked by
  `scripts/gates/bounds-allowlist.sh`.

## Closed (2026-11-22, PR 7323)

What went:

- `geom::convention::frame_margins` and the frame terms of both
  `representability_margins` doors. They take no band now, and
  `ConventionMeasure` went with its last two variants.
- `implicit.rs`'s `unit_defect` reads. Its frame error is built from
  `Frame::rounding` (`RigidMotion::rounding`). The door itself stays in
  `geom_core::running`, as a public charge door other rows count.
- The code docs and the `bounds_allowlist` entry the row lists.

What stays, re-founded under ruling (b′):

- `pcurve_cache::frame_defect` and `EnvelopeTerm::Frame`.
- The projected lane's frame check, which is the same function
  (`pcurve_cache/projected.rs`).

Both now read `Frame::rounding`. That is zero at an enclosure scalar,
whose frame encloses the rotation it holds. At a point scalar it is the
stored axes' measured distance from that rotation, with its rounding
charged. Deleting them would leave the f64 lane's frame rounding
uncharged at a large radius
(`pcurve_frame_premise_rows::a_composed_frames_rounding_is_charged_at_a_large_radius`
goes red with the rounding zeroed).
