---
id: curved-escalations-offer-a-declaration-the-door-cannot-take
kind: issue
title: curved: an ellipse carrier's escalation offers 'declare the coincidence' and 'construct the Circle carrier' at a split, which takes neither
status: review
branch: curved/equator-seam
opened: 2026-09-29
priority: P2
cost: E
---


(CHROME, from the triage in
`work/chrome/escalations-forwarded-whole-are-untriaged-for-a-declarations-object.md`.)

## What

`EllipseInvalid::Escalated` (`crates/geom/src/curves.rs`, the variant
near :265, its `Display` near :287) renders
"ellipse construction escalated: {payload} — … construct the Circle
carrier, or {COINCIDENCE_RECOURSE} (D4)". It is written for a caller
of `Curve3::ellipse`. The only callers are the plane × cylinder
section arms (`crates/geom-brep/src/intersect.rs` near :902 and
:1425), which wrap it as `SectionError::Carrier`. The chord join nests
that whole in `SplitJoinError::Section`
(`crates/topo/src/chord_join.rs`, `table` near :750, `Display` near
:450), and so it reaches:

- the split (`SplitError::Join`), which takes no declaration
  (`geom_core::SPLIT_PLANE_RECOURSE`'s doc), and where "construct the
  Circle carrier" names nothing the user can build;
- the Boolean (`BooleanError::Join`), which takes declarations.

(`SpiricInvalid::Escalated`, next door near :322, has the same shape,
but its one product caller re-wraps it: `crates/topo/src/offset_axial.rs`
near :2113 turns it into `ReplaceFaceError::Escalated`, which the SHELL
row covers.)

## Repair shape

Give the escalation a subject in plain words ("whether the section is
a circle or an ellipse") and a recourse routed by door. A split cannot
declare, and no door the user reaches builds a carrier. The routing can
live here or at `SplitJoinError::Section` (REACH/TANG ground, which
already routes `NO_DECLARATION_RECOURSE` for its own escalations). See
`sweep::blend::BlendError::Escalated`'s `Display` for the shape.
