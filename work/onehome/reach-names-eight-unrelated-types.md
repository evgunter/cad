---
id: reach-names-eight-unrelated-types
kind: issue
title: "Reach" names eight or more unrelated types across geom-brep, topo and sweep
status: open
opened: 2026-11-02
priority: P4
cost: C
rides_with: section-cert-reach-parallels-extent-ball
---

Found by the margin-scale fork's designer weighing (PR 6329, 2026-11-02).

`Reach` names at least eight unrelated types: `geom_brep::Reach` (the consumed extent), `topo::boolean::section_cert::Reach`, and types in `bound_side`, `carrier_touch`, `join`, `rim_wedge` and `sweep`'s blend. The name collision is part of why a second extent type did not read as a duplicate in review. Once PR 6329 settles the one consumed-extent type, rename the others for what they are, so the name `Reach` means that type alone. A chore: it rides the unit that lands the extent type.
