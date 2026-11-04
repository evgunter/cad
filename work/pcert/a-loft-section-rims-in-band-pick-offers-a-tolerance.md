---
id: a-loft-section-rims-in-band-pick-offers-a-tolerance
kind: issue
title: A loft's interior section rim read in band by the arc-rim pick refuses with a tolerance offer, where the decided-off reading refuses with its lever (D3)
status: open
opened: 2026-11-03
priority: P2
cost: E
---

Filed by PCERT unit E5 (`pcert/mint-picks-general-lane`), from its
sweep of the callers of `geom_brep::pick_arc_rim`. E5 made the pick's
lane-choice consumers (the plane and pair lanes' `exact_arc_rim`, the
mint's `arc_rim` closure and `iso_schedule`) take the lane a definite
reading takes when the pick reads in band. The loft is the one caller
that still ends the in-band reading by itself.

## What

`crates/sweep/src/loft.rs` `describe_section_rim` (near :1117)
re-describes an interior section's rim as its row's `IsoArc` image,
picked by `pick_arc_rim`. The two non-Zero readings end two ways:

- decided off (`Ok(None)`): `LoftError::SectionRim { source: None }`,
  whose text names the lever — "loft at v-degree 2 or more, whose walls
  span every section" (`LoftError`'s `Display`, near :504);
- in band (`Err(cause)`): `LoftError::SectionRim { source: Some(cause) }`,
  which renders `source.undecided(…, source.ending(COINCIDENCE_RECOURSE))`
  (near :514): an escalation with the coincidence recourse, a tolerance
  offer.

The rim's description form is the edge's rest locus, a lane choice
under D3 (`docs/DESIGN.md` `[verdicts.consumers]`): in band takes the
lane definite takes, and refuses only with the closed form's pose as its
lever, never a tolerance offer. The two readings should end alike.

The rows that pin today's shape: `crates/sweep/tests/degree1_loft_slabs.rs`
(the `source: None` arm) and
`crates/editor-core/tests/refusal_concision_chains.rs`
(`"SectionRim(Escalated)"`, near :3488).

## Fix

End the in-band pick as the decided-off one: `SectionRim` carries no
`source`, and the in-band row joins the degree-1 refusal's lever. A row
with an in-band section rim (sections whose arcs part by a bulge inside
the band), and a mutant that restores the `Some(cause)` arm, which reds
it.
