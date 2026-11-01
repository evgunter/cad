---
id: carrier-coincidence-is-decided-by-seven-readings
kind: issue
title: Whether two faces share a carrier is decided by seven readings, one with a runtime datum fallback and one that reads an unread pair as no merge
status: open
opened: 2026-10-14
---

Found by the pair-table audit (`fuse/every-pair-question-has-one-total-table`, #5078). The rule it tests is D3's "One reading per question" (`docs/DESIGN.md`, D3): every decision about carriers is a reading (of one carrier against a point, of a carrier pair's pose, or of a pair outcome that takes the pose); each reading is one exhaustive match with one method per arm and no fallback, and every consumer reads it. C5 (`crates/geom-brep/README.md`) is that rule's instance for surface and curve pairs.

## What

"Are these two faces on one carrier (±)" is answered by:

1. The ladder: `carrier_eq` (`crates/topo/src/boolean/carrier_eq.rs`, ~l.309-411) through `carrier_pair::{face_carrier, carrier_pair_relation}`.
   - Mixed kinds go to `_ => Distinct` (`:494`). That is sound for the analytic kinds it reaches.
   - `face_carrier` has no `CarrierDesc` for cone, Nurbs or Approx, so these come back `PairUnread::OutsideInventory`.
2. `vtxfac::one_carrier` (`crates/topo/src/boolean/vtxfac.rs:1821`) runs the ladder first. When the ladder refuses, it falls back at runtime to a raw datum comparison (`paired_with`, `bool_held_carrier_datum`). So cone×cone is decidable here and refused by the ladder.
3. `merge_faces::faces_continue` (`crates/topo/src/merge_faces.rs:2498`) reads `OutsideInventory` as `Ok(false)` (`:2599`). That is a silent "no merge" rather than a typed refusal.
4. `euler_ring::Body::same_chart` (`crates/topo/src/euler_ring.rs:1771`): key equality plus payload pointer equality.
5. `chart_region::surface_bits_equal` (`crates/topo/src/chart_region.rs:2383`): exact bits.
6. `step-import` `adopt::coincident_surfaces` (`crates/step-import/src/adopt.rs:1216`): raw f64 `<= eps`, planes only, `_ => false`. No margin is decided (EXCH's file).
7. `CoincidentSurfaces` decided inside the C5 arms with their own margins (`sphere_sphere_section`, `cylinder_cylinder_section` in `crates/geom-brep/src/intersect.rs`).

## Owed

Re-aimed to D3 "One reading per question". Coincident ± is one class of the carrier pair's *pose*, decided once by the pose reading. That reading replaces the "one coincidence table": it is total over the kind pairs, its unread pairs refuse typed naming what they lack, and each consumer reads it.

- `vtxfac`'s datum fallback becomes the pose reading's cone arm, rather than a second attempt reached because the ladder refused.
- `merge_faces`' `OutsideInventory` read as `Ok(false)` is a typed refusal read as a verdict, which D4's line "A typed refusal is never read as a verdict" (`docs/DESIGN.md`, D4 item 3) forbids. It refuses, or takes a path that decides coincidence itself. The earlier alternative, a stated "unread is not mergeable" rule, is no longer open.
- The `CoincidentSurfaces` decided inside the C5 arms with their own margins: an outcome takes the pose reading's answer and decides no pose of its own.
- `adopt::coincident_surfaces`: D3 says import is not exempt. It reads the same pose reading, with the file's own coincidence distance as the band.
- Sites 4–5 (`same_chart`, `surface_bits_equal`) are exempt under D3 only if they are identity of stored bits on which no topology decision depends. The fix checks that for each and says which.

## Item 3, the declared half, fixed (2026-10-14, `fuse/pair-gaps-fail-loud`)

**Reproduced.** A frustum's two cone wall faces on two keys, declared one pair, gave `merge_coplanar_faces_declared` → `Ok` with no groups and no skips. The declaration was dropped unsaid.

**Fixed.** `carriers_continue` now refuses a declared pair the ladder cannot read: `MergeCoplanarError::DeclaredPairUnread { f1, f2, kind }`. A declared pair there is always one kind, because the entry gate refuses a pair of two kinds as `InvalidInput` ("declared surfaces are not one kind"). So the refusal covers cone×cone, NURBS×NURBS and `Approx`×`Approx`. Rows: `sweep/tests/curved_mergedoor.rs::a_declared_cone_pair_refuses_typed_rather_than_reading_no_glue`, and `::a_declared_pair_of_two_kinds_refuses_alike_inside_and_outside_the_ladder` for plane×cone and plane×cylinder.

**Still open.** An undeclared pair outside the inventory still reads as no glue. It cannot refuse without breaking every body with two adjacent cones of different keys, such as a revolved two-slope profile. The answer it owes needs a cone, NURBS and `Approx` rung, which is the one coincidence table this row asks for and waits on the principle.

**The glue door's two unread drops, kept and noted at the site.** These are in `crates/topo/src/boolean/glue.rs`.
- `decided_declarations` reads `Err(PairUnread::Extent(_))` as no undeclared glue. This follows the module's rule: an undecided pair is left to the stage that meets it.
- `coaxial_rows` has an `if let Ok`. It was probed on the coaxiality row (`honeone/coaxiality-of-two-revolution-carriers-is-tested-five-ways`).

Both are undeclared detection, so a refusal there is a refusal of a pair nobody asked about. Both belong in the one table this row asks for, as unread arms with a stated reading.

## 2026-10-29: the ladder carries a cone (SPLITDOOR)

`boolean::carrier_eq` has a cone arm (`splitdoor/curved-copy`): apex, axis line, half-angle, the carrier the double cone. A frustum's two wall faces declared one pair now record the `PeriodClosure` the sphere's and the torus's do (`sweep/tests/curved_mergedoor.rs` `sphere_torus_and_cone_pairs_record_their_period_closure`), and `DeclaredPairUnread` is left to NURBS and `Approx`. An undeclared cone pair is read too.

## 2026-10-31: reading 7's cylinder half moved into the pair's pose (SECTARMS)

Two cylinders' `CoincidentSurfaces` is now decided in the section
value's pose (`geom_brep::pair_section::cylinders`, `cc_coaxial` read
across the operation's region), and `cylinder_cylinder_section` reads it
there rather than deciding it itself (SECTARMS stage 1, branch
`sectarms/cylinder-pair-on-the-value`). The carrier ladder's cylinder
arm (`boolean::carrier_eq::cylinder_data`: `carrier_cyl_axis_parallel`,
`carrier_cyl_axis_offset`, `carrier_cyl_radius`) still decides the same
fact at its own lever. The torus pair already shares one decision
between the two (`pair_section::torus_coincidence`); the cylinder pair
could share the pose's the same way.
