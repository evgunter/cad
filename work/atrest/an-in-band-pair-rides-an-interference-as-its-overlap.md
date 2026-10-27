---
id: an-in-band-pair-rides-an-interference-as-its-overlap
kind: issue
title: an in-band face pair between two interfering copies rides their interference as overlap evidence, and keeps a quiet finding loud
status: closed
opened: 2026-10-26
priority: P3
cost: E
refs: [an-in-band-face-pair-at-rest-is-not-one-finding]
closed: 2026-10-26
---


`crates/editor-core/src/checks/at_rest.rs` `partition`'s `undecided_pair`
routes every `CensusUndecidable` naming one entity of each of two copies
the census decided interfere into that pair's interference evidence. The
`covered` closure in `localize` then holds a quiet finding to
"every undecided verdict on the pair names faces of the component it
rides". An `ThinInBand` verdict (`topo::Undecidable::ThinInBand`) is the
census's one could-not-look finding for an opposed planar pair whose
carriers read in band, or an aligned one sharing no area thicker than
the band. It is no evidence of the overlap: any overlap it hides is
thinner than the band (`[assertions]`;
`work/atrest/an-in-band-face-pair-at-rest-is-not-one-finding.md`'s
`## Decided`, as corrected on PR 5782's review). A `MayOverlap` unread
pair, such as an aligned pair sharing area or one whose Door 1
escalated, is rightly evidence and rides the interference as today. So an in-band pair elsewhere on two interfering copies,
outside the overlap's component, keeps an asserted-quiet interference
loud, and when it lies inside the component it is folded into the
interference instead of standing as its own finding.

The fix reads `reason`: an `ThinInBand` verdict, and only that one, is not
routed into the pair's evidence, and stands as its own could-not-look finding (stage 5 C's
vessel). It is conservative today (it keeps a finding loud, never quiets
one), which is why it is P3. Found by the in-band unit's sweep. The
file is INTENT D′3's while PR 5636 is open, so the unit left it alone.

## Closed

2026-10-26, by stage 5 C (`the-at-rest-census-is-a-check`): `partition`'s
`undecided_pair` reads `reason`, and a `ThinInBand` verdict is routed into
no pair's evidence; it stands as its own could-not-look finding
(`AtRestEvidence::NoLane`). Pinned by
`intent_s5_c_at_rest_check::an_in_band_pair_is_no_overlaps_evidence`
(red with the arm removed). The quiet half the row named does not arise
there: the two copies' intersection meets the same in-band arm and
refuses, so the overlap is `Unlocalized`, loud either way.
