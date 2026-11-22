---
id: stored-reach-scalars-a-union-cannot-be-read-from
kind: issue
title: Leg::arm and editor-core's Carrier reach are stored scalars a union cannot be read from
status: open
opened: 2026-11-22
priority: P2
---

Found by `oneround/unsourced-unions` (PR 7294). Both sites are pinned `// Unsourced lever:`, and both belong to the records unit (`work/oneround/plan.md`; Ev's condition on records, PR 7141): converting them changes what a record stores.

- **editor-core `eval::measure`** (`parallel`, `face_pose`; 2 pins). `arm` sums `reach(a) + reach(b) + |ref(b) − ref(a)|`, each `reach` the stored `Carrier::{Plane, Cylinder, Sphere}.reach: T` (and `Carrier::Pose.reach: Option<T>`): a face's cover far distance read once, by `reach_of`, from that carrier's own reference point. The `Carrier` holds neither the cover nor a face key, so the union of the two faces cannot be read again from one pivot, and a `max` of the two reaches is not a union read (they are taken from two points). To read it, `Carrier` would hold its region, e.g. `ExtentBall::of_cover_from(cover, origin)` moved with the seat, and `arm` would read both balls from one pivot. What flows: the same data as now (a centre and a far distance), typed as a region. `mate::reach` reads `reach_of` too.
- **profile `sugar::arc_fillet_trims`** (1 pin). It levers `Leg::arm`, a length `Leg::at_corner` stores beside the leg's `len` and carrier, folded from them. The leg's own geometry is what the lever should be read from; the 2D read has no door either (`profile-has-no-lever-door-a-2d-crate-can-call`).
