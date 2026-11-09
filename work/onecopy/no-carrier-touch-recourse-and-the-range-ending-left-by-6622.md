---
id: no-carrier-touch-recourse-and-the-range-ending-left-by-6622
kind: issue
title: PR 6622's leftovers: NoCarrier::Touch's recourse offers a crossing that never carries, Seat drops the SplineError fact, and the range ending has a third spelling
status: open
opened: 2026-11-09
---


Found by the delta review of PR 6622 (`onecopy/unbuilt-recourse`, 6af34753). All of these are NOTE-grade; the PR merged with them open.

1. **`NoCarrier::Touch`'s recourse ("clearly cross or clearly part") may be half false.** A spline × implicit pair that clearly crosses is never carried either: its carry refuses `SplineImplicitWhole` (pair_section.rs ~1608, ~2062). Only "clearly part" is a way through. The fix is to drop the crossing half.
   - Reachable today only by an API caller of `carry`. The glue door's `pair_section::serves` (~1336) keeps Nurbs, Plane and Cone out.
2. **`KnotAlgebraError::fact()` on `Structure(e)` drops `e`** ("the knot edit refused its inputs"). The Seat sentence loses the SplineError fact, which is exactly what a defect report wants.
3. **The range ending has a third spelling.** `concat!("Recourse: ", geom_core::range_recourse!())` appears twice (winds.rs ~176, topo boolean/mod.rs ~2894). The other 15 range sites format `RANGE_RECOURSE`. Use one spelling.
4. **`write` still decides recourses inline** for NoLane, TubeUnread, PadShort and Seat. TubeUnread's cause belongs to geom-brep's `CarryError`, but its recourse is decided in topo's renderer. Move each recourse onto its payload.
5. **TubeUnread's sentence says "not finite"**, but the arm also fires on a decoration below `Def`. No path to that is known.
6. **`NoCarrier::Tangent`'s sentence** ("tangent along it or at it") has no antecedent for "it" inside the Boolean's sentence.
