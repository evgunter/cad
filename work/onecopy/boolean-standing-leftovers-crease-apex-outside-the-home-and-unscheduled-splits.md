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
   - **The ten `BooleanError` payloads, by whether a ladder reads them** (`onecopy/touch-unread-names`). `BooleanError::refusal` has six readers, all in the reduction: the sectors' crescent climb, the slit's piece rung, the held pierces' climb (2 sites), the vertex-on-face climb, and the overlap's boundary and corner refusals.
     - **Read, so split by cause:**
       - `ApexSection` reaches the vertex-on-face climb (`vtxfac::pierce_apex_germ`). It stands by its reading (`ApexSectionReading::refusal`). A disagreement is an abort: the table reads the plane missing the apex, as a closed or an open conic (`ClosedOffApex`, `OpenOffApex`), or its rulings miss the walk's sub-sector. The table's other refusals take their own standing.
       - `Join` reaches the vertex-on-face and held-pierce climbs through `join::germ_branch`. It stands by `SplitJoinError::refusal`, exhaustive over its 33 causes, with mixed causes standing as aborts. The mixed set is `UnpairedLooseEnds`, `RingHomingAmbiguous`, `SectionLoopUndecided`, `SectionInvariant`, `RunImageIncidence`, `Section`, and `SectionCrossings`' `NotAlternating`. `DegenerateSection` aborts as a fault: its doc argues the reduction admits no pose that reaches it.
       - `Section` waits on `work/onecopy/the-section-tables-refusal-states-no-standing.md`.
     - **Not read, so left unsplit, each doc naming the stage that raises it after the reduction:** `MixedSeam`, `CoincidentShell`, `ResultInvalid`, `Pcurves`, `CarryRefused`, `Merge`, `Pieces`, and `JoinUndecided`'s chart-image side.
   - **`Unsettled::Crease`: kept `Aborts`. Open question (not built): the split.** Ladders do read it. The spline-face and composite curve-face doors raise it (`curve_face/spline_face.rs`, `curve_face/composite.rs`), and it reaches the census as `MeetFault::Unsettled` (`census/curved.rs`) through `rung_or_abort`. Its doc names two causes: a floor refusal near a crease, and a meeting not decided apart from one. Split, the second would answer by its own standing and a ladder would go on to its next rung. That can move outcomes, as on the census's crease row (`a-solid-touching-itself-at-a-spline-crease-corner-finishes-undetected`), so it waits for that row.
     - **Probed (2026-11-10).** An instrumented build ran the `ci` profile's set of geom-brep, topo, sweep and editor-core (11661 tests; the slow set is not in it) at default ε and at `1e-12`. Both gave the same 8 tests, all geom-brep unit rows:
       - **The spline-face door's floor near a crease:** 4 (`curve_face::spline_face::rows::{a_meeting_on_a_crease_refuses_and_one_beside_it_reads, a_meeting_on_a_flat_crease_refuses, review_r1_a_meeting_within_the_band_of_a_crease_refuses, two_meetings_apart_but_each_within_the_band_of_a_crease_refuse}`).
       - **The composite door's meeting decided at a crease:** 4 (`curve_face::composite::rows::{a_crossing_within_band_of_a_crease_refuses, a_meeting_at_a_crease_refuses_and_one_off_it_crosses, an_in_band_apex_at_a_crease_is_not_read_clear, an_in_band_dip_at_a_crease_refuses_crease}`).
       - **No `ci`-set fixture in topo, sweep or editor-core reaches it.**
       - **Neither door raises `Crease` from a reading in band.** The composite door escalates an in-band meeting-to-crease reading, and its probe arm fired 0 times; it raises `Crease` only on a decided zero. The spline-face door raises it at the floor, within a reach of a crease.
       - So the in-band count is 0 by construction. A split would part a floor refusal from a decided meeting on the crease.

2. **`TouchUnread::Unreadable` is classed by search control.** Its doc (`link.rs`) reads: "an abort: no later face rescues a germ one face makes unreadable, so it decides the outcome now". That is a ladder's reason, not a taxonomy class. It also covers a curved face the clearance does not read (row 2) and a face with no tangent plane at a cone's apex, which is 1(b) and would be in band (`Refusal::of_apex`). The census's `corner_normal` (`census.rs`) also routes a spline crease (1(b), `Refusal::of_crease`) and a surface with no wired sector arm (`Unsupported`, row 2) or a spline half-edge with no chart image to read the vertex off (`Unplaced`) to `Unreadable`, so that the three abort alike, whatever their causes. It wants splitting by cause, the way `BooleanError` was split.
   - **Done on `onecopy/touch-unread-names`: the names split, the control kept.** `corner_normal` now names its causes apart: `TouchUnread::NoTangentPlane` (an apex or a vertex in an axis's band, `VertexFace`), `Crease`, `NoSectorArm` (`Unsupported`) and `Unplaced`. Each answers `Aborts`, as `Unreadable` did. `TouchUnread::curved_unread` is the one predicate that every former `Unreadable` match in the census (`self_touch_verdict`, the edge-in-face chord fallback) and in editor-core `measure.rs` reads. A refusal now says its cause, and no ladder's outcome moves.
   - **Open question (not built): each cause on its own standing.** That would mean `NoTangentPlane` as `Refusal::of_apex`, `Crease` as `Refusal::of_crease`, and `NoSectorArm` as a limit (row 2). A ladder over the faces would then read past these causes rather than stop on them. That moves outcomes: a crease corner refused by its own standing lets a solid touching itself there finish undetected (`a-solid-touching-itself-at-a-spline-crease-corner-finishes-undetected`). It waits for that row, and for a reading that tells a germ the census may skip from one it must not.

3. **Row 2 versus DESIGN's naming rule (N-c).** `BooleanError::refusal()`'s doc says row 2 holds whatever a variant's name is. That records a standing departure from DESIGN's `Unsupported*` naming rule, with no rename and no DESIGN revision scheduled. Ev's call.
