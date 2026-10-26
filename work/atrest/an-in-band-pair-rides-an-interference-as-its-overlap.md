---
id: an-in-band-pair-rides-an-interference-as-its-overlap
kind: issue
title: an in-band face pair between two interfering copies rides their interference as overlap evidence, and keeps a quiet finding loud
status: open
opened: 2026-10-26
priority: P3
cost: E
refs: [an-in-band-face-pair-at-rest-is-not-one-finding]
---


`crates/editor-core/src/checks/at_rest.rs` `partition`'s `undecided_pair`
routes every `CensusUndecidable` naming one entity of each of two copies
the census decided interfere into that pair's interference evidence. The
`covered` closure in `localize` then holds a quiet finding to
"every undecided verdict on the pair names faces of the component it
rides". An in-band face pair (`topo::Undecidable::InBandPair`, the
census's one could-not-look finding for two planar faces whose carriers
read in band) is no evidence of the overlap: any overlap it hides is
thinner than the band (`[assertions]`;
`work/atrest/an-in-band-face-pair-at-rest-is-not-one-finding.md`'s
`## Decided`). So an in-band pair elsewhere on two interfering copies,
outside the overlap's component, keeps an asserted-quiet interference
loud, and when it lies inside the component it is folded into the
interference instead of standing as its own finding.

The fix reads `reason`: an `InBandPair` verdict is not routed into the
pair's evidence, and stands as its own could-not-look finding (stage 5 C's
vessel). It is conservative today (it keeps a finding loud, never quiets
one), which is why it is P3. Found by the in-band unit's sweep. The
file is INTENT D′3's while PR 5636 is open, so the unit left it alone.
