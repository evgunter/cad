---
id: carrier-frames-check-1s-frame-half-goes
kind: unit
title: Carrier frames (iii): check 1's frame half, frame_defect, unit_defect and the projected lane's frame check go, with the code docs and the bounds_allowlist entry
status: parked
opened: 2026-11-17
priority: P1
cost: M
parent: carriers-hold-their-frame-as-unitvec3-and-rigidmotion-and-check-1s-frame-half-goes
blocked_on: [carrier-frames-the-types-and-the-lift]
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
