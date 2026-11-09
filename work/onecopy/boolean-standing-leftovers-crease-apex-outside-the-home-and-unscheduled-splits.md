---
id: boolean-standing-leftovers-crease-apex-outside-the-home-and-unscheduled-splits
kind: issue
title: PR 6576's leftovers: a crease and an apex class still stated by hand outside their home, an unpinned join premise, and abort-until-split payloads no row schedules
status: open
opened: 2026-11-09
---


Found by the delta review of PR 6576 (`onecopy/boolean-standing`, 3355634d/0e0ecd45). All are FIX-grade but not gating; that PR merged with them open.

1. **A crease's reading is dropped in the split.** `splitting/neighborhood.rs:~72` maps `SectorFaceError::Crease { face, kind, .. }` to `SplitReduceError::CurvedBooleanUnsupported`. The `..` drops the `reading` that 6576 added, so an in-band crease is called a row-2 "unsupported" in the split, while the boolean side carries the reading. Delegate to `Refusal::of_crease`.
2. **The apex class is stated by hand.** `census.rs:~8743` `corner_normal`: `ApexUndecided => TouchUnread::InBand` spells the apex class without its home (`Refusal::of_apex`), and `Err(_) => Unreadable` covers Crease whatever its reading.
3. **The join's degenerate-torus premise is unpinned.** `seam_germ_side` now aborts on a torus region's refusal, where it used to be a kept Limit in `climb_agreeing`. Its stated premise, "the germ pair's frame refuses that torus before any chord is joined", is asserted, not shown. If it is false, an invalid input (1(a), with a lever) surfaces as ClassificationInvariant. Pin it with a row, or argue it at the site. `seam_germ_side`'s `# Errors` doc (join.rs ~5609-5616) still lists only "off the surface".
4. **Nothing schedules the abort-until-split payloads.** There are 13 or more, including NonFiniteSectorChord (F4) and Unsettled::Crease. NonFiniteSectorChord's split is local and cheap: `ConicDeparture::of` (conic_bend.rs:80-91) holds `tangent` before `.normalize()`, so a stalled end is visible there.
5. **The torus class rides on the face-normal door (N-b).** `BooleanError::DegenerateTorus` delegates by building a `NormalAtError::DegenerateTorus`, and join's FrameError path borrows the normal door's class for a frame fact. The fact's natural home is the torus convention verdict (`TorusConvention`/`Refused`), which both doors read.
6. **Row 2 versus DESIGN's naming rule (N-c).** `refusal()`'s doc says row 2 is "whatever its name". That records a standing departure from DESIGN's `Unsupported*` naming rule, with no rename and no DESIGN revision scheduled. Ev's call if it should change.
