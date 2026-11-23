---
id: a-declared-graze-at-a-cone-apex-is-unmeasured
kind: issue
title: A declared tangency grazing a cone along a ruling through its apex is unmeasured in the Boolean
status: open
opened: 2026-11-01
priority: P3
cost: M
---

Filed by SPLITDOOR on `splitdoor/cone-apex-boolean` (PR 6266), from its
review.

## What

A plane tangent to a cone along a ruling through its apex is pinned only
undeclared: `crates/sweep/tests/a_cone_apex_on_a_plane_through_it.rs`
(`a_plane_grazing_a_tipped_cone_along_a_ruling_refuses_typed`) holds each
op refusing `CurvedBooleanUnsupported { kind: Cone }` at the ruling's rim
end. The same graze declared (`topo::Declared`, the tangency between the
plane face and the cone face) reaches the pierce at the apex with a glue
record for the pair, so `vtxfac`'s `flat` arm and its `TangentRead` would
read the apex sub-sector whose bound is the tangent ruling. No row
measures what it answers: a build at the closed form (the cone lies
wholly on one side of the plane, touching it along the ruling), a typed
refusal, or a wrong body.

## Where to look

Add the declared twin of the graze row, both sides of the plane, at the
wrap edge's ruling and off it, and read what each op answers.
