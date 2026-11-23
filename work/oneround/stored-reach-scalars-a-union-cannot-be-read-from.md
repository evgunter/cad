---
id: stored-reach-scalars-a-union-cannot-be-read-from
kind: issue
title: Leg::arm and editor-core's Carrier reach are stored scalars a union can be bounded from above but not read from below
status: open
opened: 2026-11-22
priority: P2
---

Found by `oneround/unsourced-unions` (PR 7294). Both sites are pinned `// Unsourced lever:`, and both belong to the records unit (`work/oneround/plan.md`; Ev's condition on records, PR 7141): converting them changes what a record stores.

- **editor-core `eval::measure`** (`parallel`, `face_pose`; 2 pins). `arm` sums `reach(a) + reach(b) + |ref(b) − ref(a)|`, each `reach` the stored `Carrier::{Plane, Cylinder, Sphere}.reach: T` (and `Carrier::Pose.reach: Option<T>`): a face's cover far distance read once, by `reach_of`, from that carrier's own reference point. From above, the union can be bounded from what is stored: from `ref(a)`, face `b` stands no farther than `|ref(b) − ref(a)| + reach(b)`, so `max(reach(a), |ref(b) − ref(a)| + reach(b))` bounds the union's reach from that pivot. That is tighter than the sum. Two things are missing. One is a door to take the bound from: `ExtentBall`'s reads take a cover or points, not a stored centre and radius, and a new wrapper that takes caller numbers is held by `lever-doors.sh`. The other is the bound from below. A stored centre and far distance enclose a face, but do not hold any of its points. The `Carrier` keeps neither the cover nor a face key, so the face could be far smaller than its ball. Nothing stored bounds the union's reach from below, and a definite verdict has nothing to read. To read it, `Carrier` would hold its region, e.g. `ExtentBall::of_cover_from(cover, origin)` moved with the seat, and `arm` would read both balls from one pivot. What flows: the same data as now (a centre and a far distance), typed as a region. `mate::reach` reads `reach_of` too.
- **profile `sugar::arc_fillet_trims`** (1 pin). It levers `Leg::arm`, a length `Leg::at_corner` stores beside the leg's `len` and carrier, folded from them. The leg's own geometry is what the lever should be read from; the 2D read has no door either (`profile-has-no-lever-door-a-2d-crate-can-call`).
