---
id: reach-names-eight-unrelated-types
kind: issue
title: "Reach" names eight or more unrelated types across geom-brep, topo and sweep
status: dispatched
opened: 2026-11-02
priority: P4
cost: C
rides_with: section-cert-reach-parallels-extent-ball
branch: onehome/one-extent
---

Found by the margin-scale fork's designer weighing (PR 6329, 2026-11-02).

`Reach` names at least eight unrelated types: `geom_brep::Reach` (the consumed extent), `topo::boolean::section_cert::Reach`, and types in `bound_side`, `carrier_touch`, `join`, `rim_wedge` and `sweep`'s blend. The name collision is part of why a second extent type did not read as a duplicate in review. Once PR 6329 settles the one consumed-extent type, rename the others for what they are, so the name `Reach` means that type alone. A chore: it rides the unit that lands the extent type.

## Done by `onehome/one-extent` (2026-11-02)

Renamed for what each is: `bound_side::BoundSupport` (what stands
behind a sector bound), `carrier_touch::MedialDistance` (a carrier's
distance to its medial axis), `join::PartnerDistance`,
`rim_wedge::FaceOnLocus`, `census`'s local `ProximityFace`, and
sweep's `blend::reach::Stretch`. `section_cert::Reach` is deleted. The
name `Reach` in the kernel now means `geom_brep::Reach` alone.
