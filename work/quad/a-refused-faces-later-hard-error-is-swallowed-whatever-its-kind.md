---
id: a-refused-faces-later-hard-error-is-swallowed-whatever-its-kind
kind: issue
title: A face's later-round hard error is held as FaceStop::Refused whatever its kind, so a defect kind would be hidden by check 7
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


## Update 2026-10-24 (measure door unit B, PR 5628)

The arm moved and widened by ruling. `walk_set` holds ANY face that a
later round refuses outright at the enclosure it held
(`FaceStop::Refused`), not only one already carrying a budget refusal.
The walk decides on what it holds, and the refusal surfaces only where
the walk runs out undecided (`Walked::Exhausted`). The continuation and
`measure` keep the held face too. This restores main's
`PastTarget::refine` semantics (the fix pass's ruling 2).

The row's question stands, and is now wider. A defect kind
(`classify_mass_props(..).defect`) met at round ≥ 1 is held like any
other refusal, so check 7 would decide on the enclosure before it,
where the round's refusal says the stored data is wrong. The fix's
shape is unchanged: in `FaceRun::advance`'s `Err` arm, or at
`walk_set`'s call of it, surface a defect kind at once.

## Re-checked on main `9267d3ae` (QUAD tracker sweep, 2026-10-27)

Still holds after PRs 5741 and 5815. There is no budget refusal any more
(5741). The arm is now `FaceRun::advance`'s `Err` branch
(`crates/topo/src/props.rs:4123`). It sets `FaceStop::Refused(refusal)`
(`:4104`) for any refusal, and nothing reads
`classify_mass_props(..).defect` first. `walk_set` (`props.rs:826`) and
`role_walk_set` (`:6237`) decide on the held enclosure. The title
dropped "budget-refused", because every face is held now. The fix's
shape is unchanged.
