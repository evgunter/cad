---
id: boolean-standing-leftovers-crease-apex-outside-the-home-and-unscheduled-splits
kind: issue
title: PR 6576's leftovers: eleven abort-until-split payloads to split, TouchUnread::Unreadable classed by search control, and row 2 against DESIGN's naming rule
status: open
opened: 2026-11-09
priority: P3
cost: M
---

Found by the delta review of PR 6576 (`onecopy/boolean-standing`). That PR merged with these open.

**Narrowed by `onecopy/boolean-standing-2`**, which closed five items:
- **The split's crease.** It is argued at `SectorFaceError::split_refusal`: the carrier gate (`classify::carrier_gate`) refuses a spline face the plane may reach before any sector is read, so the split's refusal is the face's kind (row 2), whatever the crease reads.
- **The census's apex.** `corner_normal` reads `Refusal::of_apex` through `TouchUnread::kept`. Its crease stays `Unreadable` (below) and is filed as `a-solid-touching-itself-at-a-spline-crease-corner-finishes-undetected`, since keeping it would let a self-touch finish undetected.
- **The join's torus premise.** It is argued at `seam_germ_side`: tier 3 refuses a finished operand's torus decided outside the convention (`ValidationError::DegenerateTorus`), and the germ pair's frame reads it again (`section_refusal`). `seam_germ_side`'s `# Errors` doc names it.
- **The torus class.** It has one home, `geom_brep::TorusRefusal::refusal`, and the normal door, the Boolean and `replace_face` all read it.
- **`NonFiniteSectorChord`.** It is one class, a limit (row 1(b)). The stalled spline end it was thought to carry never reaches the chord, because certification decides every carrier's speed floor positive (`CertifyError::SpanMeterCollapsed` for a spline). That leaves the format's scale as the only cause.

## What remains

1. **Split the abort-until-split payloads.** Each states its class as "an abort until it is split", and none is scheduled elsewhere.
   - `BooleanError`: `ApexSection`, `MixedSeam`, `CoincidentShell`, `ResultInvalid`, `Pcurves`, `Join`, `CarryRefused`, `Merge`, `Pieces`, and `JoinUndecided`'s chart-image side.
   - `geom_brep::roots::Unsettled::Crease`.

2. **`TouchUnread::Unreadable` is classed by search control.** Its doc (`link.rs`) reads: "an abort: no later face rescues a germ one face makes unreadable, so it decides the outcome now". That is a ladder's reason, not a taxonomy class. It also covers a curved face the clearance does not read (row 2) and a face with no tangent plane at a cone's apex, which is 1(b) and would be in band (`Refusal::of_apex`). The census's `corner_normal` (`census.rs`) also routes a spline crease (1(b), `Refusal::of_crease`) and a surface with no wired sector arm (`Unsupported`, row 2) or a spline half-edge with no chart image to read the vertex off (`Unplaced`) to `Unreadable`, so that the three abort alike, whatever their causes. It wants splitting by cause, the way `BooleanError` was split.
   - **Done on `onecopy/touch-unread-names`: the names split, the control kept.** `corner_normal` now names its causes apart: `TouchUnread::NoTangentPlane` (an apex or a vertex in an axis's band, `VertexFace`), `Crease`, `NoSectorArm` (`Unsupported`) and `Unplaced`. Each answers `Aborts`, as `Unreadable` did. `TouchUnread::curved_unread` is the one predicate that every former `Unreadable` match in the census (`self_touch_verdict`, the edge-in-face chord fallback) and in editor-core `measure.rs` reads. A refusal now says its cause, and no ladder's outcome moves.
   - **Open question (not built): each cause on its own standing.** That would mean `NoTangentPlane` as `Refusal::of_apex`, `Crease` as `Refusal::of_crease`, and `NoSectorArm` as a limit (row 2). A ladder over the faces would then read past these causes rather than stop on them. That moves outcomes: a crease corner refused by its own standing lets a solid touching itself there finish undetected (`a-solid-touching-itself-at-a-spline-crease-corner-finishes-undetected`). It waits for that row, and for a reading that tells a germ the census may skip from one it must not.

3. **Row 2 versus DESIGN's naming rule (N-c).** `BooleanError::refusal()`'s doc says row 2 holds whatever a variant's name is. That records a standing departure from DESIGN's `Unsupported*` naming rule, with no rename and no DESIGN revision scheduled. Ev's call.
