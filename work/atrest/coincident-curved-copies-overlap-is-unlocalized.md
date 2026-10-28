---
id: coincident-curved-copies-overlap-is-unlocalized
kind: issue
title: Coincident curved copies' interference is never bounded: the intersection that localizes it refuses
status: open
opened: 2026-10-27
priority: P2
cost: M
refs: [curved-copies-filling-one-volume-read-as-undecidable]
---


Found by `curved-copies-filling-one-volume-read-as-undecidable` (refs).

## What

Two copies of one revolved cylinder pinned at one pose are one
interference finding at the editor's gate, carrying
`SameSideInterference`. Its overlap is
`Overlap::Unlocalized(Unlocalized::Refused { .. })`, not `Bounded`. The
gate localizes an overlap by intersecting the two copies
(`crates/editor-core/src/checks/at_rest.rs` `localize`), and that
intersection refuses
`BooleanError::FallbackExtentUnsupported` ("the solids' boundaries do
not cross, and the Boolean cannot be sure whether one lies inside the
other … a curved face and a face of the other solid have tangent or
near-tangent carriers"). Two planar copies are bounded
(`asm_r2b_assembly.rs` `copies_filling_one_volume_are_one_interference`).
The curved ones are pinned unlocalized at both poses by
`copies_of_one_cylinder_filling_one_volume_are_one_interference`.

An unbounded finding cannot be quieted by an assertion at its overlap
(`[assertions]`), so an intended overlap of curved copies stays loud.

## What closing it takes

The intersection of two copies whose curved faces coincide on two keys
(the boolean's coincident-curved glue, `A ∩ A` for one surface on two
descriptions). The other route is a localization that reads the overlap
off the census's decided same-side pair rather than off the boolean.

## Closed by `splitdoor/two-rims-coplanar` (2026-10-28)

That branch builds the boolean's coincident case on the no-crossings path. A refusing pair the reduction settled one carrier is deferred to the fallback's shell verdicts, and it is answered where both faces' shells read `On` each other (`boolean::ops` `OnDeferred`). The intersection that `localize` runs now builds the cylinder. The finding is `Overlap::Bounded`: loud, still unquietable (no face of one copy opposes a face of the other for a gap to hold over), and never refused. `copies_of_one_cylinder_filling_one_volume_are_one_interference` is re-baselined to `Bounded` at both poses, at ε = 1e-6, 1e-9 and 1e-12. The row closes when that branch merges.
