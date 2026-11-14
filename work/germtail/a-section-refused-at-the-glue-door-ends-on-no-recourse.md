---
id: a-section-refused-at-the-glue-door-ends-on-no-recourse
kind: issue
title: A germ section refused at the glue door renders SectionError arms with no recourse
status: open
opened: 2026-11-09
---


Filed by the ONECOPY `onecopy/unbuilt-recourse` lane, whose gate
`topo::boolean::section_unbuilt_rows::every_section_unbuilt_arm_ends_on_one_recourse`
admits this by name in its `FILED` list.

## What

`topo::boolean::join::section_unbuilt` maps every non-escalated
`Uncarried::Section(e)` to `SectionUnbuilt::Refused(e)`. That arm renders
`e` through `geom_brep::recourse::read_at`, so its ending is
`SectionError::write`'s (`crates/geom-brep/src/intersect.rs`). Several of
those arms end on a description and no recourse:

- `DegenerateOperand`: "a section operand is degenerate: {what}". This is the
  case `SectionUnbuilt::Refused`'s doc names: an operand whose radius is not
  definitely positive.
- `RoutesToGeneralRung`: "the {pair} section: {why}".
- `Carrier` / `Spiric`: "the section's curve refused: {e}" and "the section's
  spiric refused: {e}". These end however the nested refusal ends.
- `Unaccounted`: "the section of a plane and a spline surface was not read
  whole: {cause}".

So a Boolean refused at its glue door on one of these reaches the user as
`the Boolean cannot build the section where …: a section operand is
degenerate: …`, with nothing to do.

## What closing it takes

Decide each arm's recourse on `SectionError` itself: a geometry lever, the
kernel limit, not yet, or the defect. Then delete the `Refused` entry from
`section_unbuilt_rows`' `FILED` list. That list goes red once the sample
states a recourse.
