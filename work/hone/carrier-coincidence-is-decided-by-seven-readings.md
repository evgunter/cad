---
id: carrier-coincidence-is-decided-by-seven-readings
kind: issue
title: Whether two faces share a carrier is decided by seven readings, one with a runtime datum fallback and one that reads an unread pair as no merge
status: open
opened: 2026-10-14
---

Found by the pair-table audit (`fuse/every-pair-question-has-one-total-table`), which tests every geometric pair question against C5's rule: one total kind-pair table, one method per pair, no runtime fallback (`crates/geom-brep/README.md`, C5).

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

- One coincidence table over every kind pair. Its unread pairs refuse typed, and each consumer reads it.
- `vtxfac`'s datum fallback becomes the table's cone arm, rather than a second attempt.
- `merge_faces`' silent `false` becomes a typed refusal or a stated "unread is not mergeable" rule.

Sites 4–5 are bit-identity rather than geometric questions. Whether they stay outside the table is the fix's call.

## Item 3, the declared half, fixed (2026-10-14, `fuse/pair-gaps-fail-loud`)

**Reproduced.** A frustum's two cone wall faces on two keys, declared one pair, gave `merge_coplanar_faces_declared` → `Ok` with no groups and no skips. The declaration was dropped unsaid.

**Fixed.** `carriers_continue` now refuses a declared pair the ladder cannot read: `MergeCoplanarError::DeclaredPairUnread { f1, f2, kind }`. A declared pair there is always one kind, because the entry gate refuses a pair of two kinds as `InvalidInput` ("declared surfaces are not one kind"). So the refusal covers cone×cone, NURBS×NURBS and `Approx`×`Approx`. Rows: `sweep/tests/curved_mergedoor.rs::a_declared_cone_pair_refuses_typed_rather_than_reading_no_glue`, and `::a_declared_pair_of_two_kinds_refuses_alike_inside_and_outside_the_ladder` for plane×cone and plane×cylinder.

**Still open.** An undeclared pair outside the inventory still reads as no glue. It cannot refuse without breaking every body with two adjacent cones of different keys, such as a revolved two-slope profile. The answer it owes needs a cone, NURBS and `Approx` rung, which is the one coincidence table this row asks for and waits on the principle.

**The glue door's two unread drops, kept and noted at the site.** These are in `crates/topo/src/boolean/glue.rs`.
- `decided_declarations` reads `Err(PairUnread::Extent(_))` as no undeclared glue. This follows the module's rule: an undecided pair is left to the stage that meets it.
- `coaxial_rows` has an `if let Ok`. It was probed on the coaxiality row (`hone/coaxiality-of-two-revolution-carriers-is-tested-five-ways`).

Both are undeclared detection, so a refusal there is a refusal of a pair nobody asked about. Both belong in the one table this row asks for, as unread arms with a stated reading.
