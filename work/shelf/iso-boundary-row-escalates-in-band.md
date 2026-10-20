---
id: iso-boundary-row-escalates-in-band
kind: issue
title: iso_boundary_row refuses in band and when definite, where the derived section is the general lane
status: open
opened: 2026-10-20
---

Filed by PCERT unit E1 (`pcert/lane-choices`), from its sweep of lane
choices outside `pcurve_cache`.

D3 (`[verdicts.consumers]`) treats a decision that picks between two forms
of the SAME cells (a closed form at Zero, a general one otherwise) as a
lane choice. In band it takes the general lane, as definite does, or it
refuses lever-only "not yet" where no general lane is written. It never
escalates with a tolerance offer and never ends as a defect.

## What

`crates/geom-brep/src/nurbs_iso.rs:357`, `iso_boundary_row`
(`iso_row_at_domain_end`), used by
`crates/topo/src/replace_face.rs:2503`:
- Zero extracts the exact row;
- definite refuses `IsoRowError::Interior` (:366);
- in band refuses `IsoRowError::Escalated` (:358).

The derived section is the general lane over the same row, but it is
taken only when the hold fails, never on this reading.

## Fix

In band and definite take the derived section, recording nothing. Add
a row with an in-band pose, and a mutant that restores the escalation.
