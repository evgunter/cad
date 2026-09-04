---
id: point-in-solid-refusal-names-faces-zero
kind: issue
title: point_in_solid's body-scoped refusals carry faces[0] under a payload doc that says 'the face being tested'
status: open
opened: 2026-09-04
---


## The finding

Found by the class sweep of the FIX unit that made
`ValidationError::CensusUnsupported` carry the face PAIR its census arm
was examining, instead of one half of it chosen by arena order. The
sweep's shape — *a refusal whose subject is a set, carrying one member
the arena's ordering picked* — has one other hit in the kernel, and it
is a payload-doc falsehood rather than an attribution defect.

Three sites in `crates/topo/src/boolean/solid_contain.rs` build a
`PointInSolidError` naming `faces[0]`, where the refusal's subject is
the whole body:

- `solid_contain.rs:3464` — `MassPropsError::Corrupt { .. } |
  MassPropsError::NullScaffoldEdge { .. }` becomes
  `PointInSolidError::CorruptFace { face: faces[0] }`. Neither source
  variant is about `faces[0]`: `Corrupt` is a body-arena claim and
  `NullScaffoldEdge` names an EDGE. The face is not the corrupt thing;
  it is the first face of the walk.
- `solid_contain.rs:3480` — the at-infinity orientation probe's
  escalation becomes `PointInSolidError::Escalated { face: faces[0],
  diag }`. That margin is `Margin::over_lever(props.volume,
  props.surface_area)` — a WHOLE-BODY quantity. No face escalated.

The payload docs (`solid_contain.rs:139` *"The face being tested"*,
`:160` *"The face."*) then state something the two sites do not
establish, which is the same defect shape S190 named at the census: a
reader (or a GUI highlighting the offending entity) is sent to a face
that had nothing to do with the refusal.

## Not fixed here, and why

`crates/topo/src/boolean/` is S-BOOL's glob and this is a different
door from the FIX unit's two; the fix is also a choice rather than a
mechanical widening — either a body-scoped variant (no face), or the
payload doc narrowed to *"a face of the body being tested, as a
locator; not the site of the refusal"*. That choice wants the owning
program.

## Home

`crates/topo/src/boolean/solid_contain.rs` — S-BOOL territory.
