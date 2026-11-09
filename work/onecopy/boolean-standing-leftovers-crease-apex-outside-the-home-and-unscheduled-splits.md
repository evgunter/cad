---
id: boolean-standing-leftovers-crease-apex-outside-the-home-and-unscheduled-splits
kind: issue
title: PR 6576's leftovers: twelve abort-until-split payloads to split, TouchUnread::Unreadable classed by search control, and row 2 against DESIGN's naming rule
status: open
opened: 2026-11-09
priority: P3
cost: M
---

Found by the delta review of PR 6576 (`onecopy/boolean-standing`). That PR merged with these open.

**Narrowed by `onecopy/boolean-standing-2`**, which closed four items:
- **The split's crease.** It is argued at `SectorFaceError::split_refusal`: the split cuts no spline face, so its refusal is the face's kind (row 2), whatever the crease reads.
- **The census's corner.** `corner_normal` now reads `Refusal::of_apex` and `Refusal::of_crease` through `TouchUnread::kept`.
- **The join's torus premise.** It is argued at `seam_germ_side` (`section_refusal` refuses the torus at the pair's frame before any chord), and `seam_germ_side`'s `# Errors` doc now names it.
- **The torus class.** It has one home, `geom_brep::TorusConvention::decided_refusal`, and the normal door and the Boolean both read it.

## What remains

1. **Split the abort-until-split payloads.** Each states its class as "an abort until it is split", and none is scheduled elsewhere.
   - `BooleanError`: `ApexSection`, `NonFiniteSectorChord`, `MixedSeam`, `CoincidentShell`, `ResultInvalid`, `Pcurves`, `Join`, `CarryRefused`, `Merge`, `Pieces`, and `JoinUndecided`'s chart-image side.
   - `geom_brep::roots::Unsettled::Crease`.

   `NonFiniteSectorChord` is not the cheap split the review expected. `ConicDeparture::of` (`conic_bend.rs`) serves conics only, and a conic's derivative never stalls. The stalled spline end comes from a spline bound built on another path. Splitting it means finding that path and refusing the stall there, before the chord.

2. **`TouchUnread::Unreadable` is classed by search control.** Its doc (`link.rs`) reads: "an abort: no later face rescues a germ one face makes unreadable, so it decides the outcome now". That is a ladder's reason, not a taxonomy class. It also covers a curved face the clearance does not read (row 2) and a face with no tangent plane at a cone's apex, which is 1(b) and would be in band (`Refusal::of_apex`). It wants splitting by cause, the way `BooleanError` was split.

3. **Row 2 versus DESIGN's naming rule (N-c).** `BooleanError::refusal()`'s doc says row 2 holds whatever a variant's name is. That records a standing departure from DESIGN's `Unsupported*` naming rule, with no rename and no DESIGN revision scheduled. Ev's call.
