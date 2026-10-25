---
id: a-refused-faces-later-hard-error-is-swallowed-whatever-its-kind
kind: issue
title: A budget-refused face's later-round hard error is mapped to stopped() whatever its kind, so a defect kind would be hidden by check 7
status: open
opened: 2026-10-21
priority: P3
cost: E
---


From PR 5568's fix-pass re-review (F2), 2026-10-21. `topo::props`'s `walk`
maps any hard error at round ≥ 1 on a face that already carries a budget
refusal to `FaceRun::stopped()`: the face keeps its earlier sound enclosure
and its refusal. Every kind that can reach the arm today is "cannot refine",
not a defect. The reviewer classified each one: `QuadratureUnsupported`
from a non-finite sum, the trimmed lane's `Escalated{Inventory}` and
`TRIM_OPEN_WALK`. `classify_mass_props` marks none of them `defect: true`.

The arm keys on "carries a refusal", not on the error's kind, so a lane
that ever raises a defect kind per round would be hidden by check 7.
`TRIM_OPEN_WALK` first seen at round r is already borderline: it says the
pcurves disagree beyond the slack at that resolution, which is information
about stored data. Fix: keep `Err(hard)` when
`classify_mass_props(&hard).defect` holds, and say at the arm why the
remaining kinds are safe to swallow.
