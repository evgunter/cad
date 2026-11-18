---
id: an-imported-under-determined-smooth-join-refuses-not-yet-supported
kind: issue
title: An imported smooth join the must-carry rule answers with the pair refuses not-yet-supported
status: open
opened: 2026-11-18
---



Found by `importgate/build` (the build that makes every imported edge's
carrier by the kernel's own constructions).

## What

A native body whose smooth join the must-carry rule reads
under-determined stores the join as its conventional pair (D2, C7;
`geom_brep::reask_pair` answering `OnPair`). Before the build, the import
adopted that join by certifying the file's printed curve against the
pair. The build constructs no carrier from the file's curve, and the
kernel has no construction of a tangency its two surfaces under-determine
(the relative normal curvature across it is not certified apart from
zero), so `crates/step-import/src/adopt.rs` (`must_carry_rung`'s
`Some(Ok)` arm in Phase C) refuses the edge as
`StepImportError::NotYetSupported`, naming the lane "the construction of
an under-determined smooth join (D2's conventional pair, C7)".

`crates/step-import/tests/a_smooth_join_imports_by_the_must_carry_rule.rs`
(`a_bore_join_round_trips_as_the_rule_describes_it`) pins it: the torus
ring with a bore at margin ε/2 exports and refuses on re-import, where
the jet-determinate margin round-trips as a `TangentIntersection`. No
file of the wild or own corpus states such a join (the census at that
head: 130 tangent circles and 101 tangent lines, all built and
certified).

## What would close it

A construction of the join's carrier from the pair alone (what the
native door stores), which the build would call where the rule answers
`OnPair`, and the test's under-determined cell re-pinned as a round trip.
