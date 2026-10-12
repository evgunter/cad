---
id: w1-and-c6-read-a-fold-and-a-certified-refinement
kind: ruling
title: "W1's 'number of restrictions' and C6's certified refinement: two clarifications fork3's designers found"
status: closed
opened: 2026-10-10
closed: 2026-10-10
pr: 4539
---


Fork3 decided the spline layer's certified combine. Its decisions are built without a ruling, because no ratified text has to move for them:
- the monotone two-point step met with the sources' hull, as a door on `Certification`;
- the projective knot algebra made unrepresentable at `Interval`;
- one blossom primitive on `CoeffWindow`.

Both designers found two places where ratified text reads less clearly than the decision needs. Neither changes what is decided, so they come to Ev here as wording, and the PR edits the text in place:

1. **W1** (`crates/geom-core/README.md`). "Width proportional to … the number of restrictions a value has been through, is a defect." Read literally, it condemns the rounding every form adds once per step. Both designers read it as "inherited width multiplied per step": the lerp form's `(1 + t)` per insertion. The edit says that.
2. **C6** (`crates/geom-brep/README.md`). The clause says the f64 lane chooses every weight and combination coefficient, and the interval lane proves that choice. A refinement made inside a certificate chooses nothing: its refined weights are generally not `f64`. The edit says that such a refinement is held as homogeneous enclosures `(w·P, w)`, and that the projective knot algebra has no meaning at the certification scalar.
   - Designer B holds that this sentence is the principle behind fork3's Q2 decision.
   - Designer A holds that the decision follows from C6 as written and the sentence only states it.

Fork record: `docs/DESIGN-FORK-LOG.md` row 107.

## Closed

Ev adopted both edits as written ("sounds good", PR 4539, 2026-10-10). W1's scale bullet now names inherited width multiplied per step as the defect, with rounding added once per step as the floor. C6 says a refinement inside a certificate is held only as homogeneous enclosures, and that the projective knot algebra has no meaning at the certification scalar.
