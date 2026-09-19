---
id: validity-refuses-an-interior-chart-singularity
kind: issue
title: Validity refuses a revolution-chart face whose loop is rims only (a pole or apex interior to the face)
status: open
opened: 2026-09-18
refs: [rim-only-sphere-cap-panics-at-census, import-normalizes-the-rim-only-cap]
---


Filed by the TESS orchestrator from Ev's ruling on `[ev]` PR 2850
(2026-09-18): DESIGN.md, "A chart singularity inside a face is a vertex
of it".

Tiers 1–3 accept a sphere face bounded by one latitude circle with the
pole interior (measured: `validate`, `validate_closed`,
`validate_geometric` all `Ok`). DESIGN now says that statement is not a
face, so validity refuses it — by the structural fact (the face's loop
on a revolution chart classifies with no meridian), in whichever tier
owns rim/meridian classification; the unit says which and why. The
Euler door is the one native way to state it
(`crates/topo/tests/mesh12_rim_row_reach.rs::two_level_rim_cap`).

Lands AFTER `work/exch/import-normalizes-the-rim-only-cap.md`. With it,
props' rim-only arm (PR 2741: `sphere_rim_only_pole_level`,
`require_rim_only_closed`, and the rim-only half of
`rim_interior_side`'s callers) is unreachable through a valid body and
retires in the same unit or a PROPS one; the arm is whole at
`6a1f6d60d`, which `work/tess/consider-emitting-the-rim-only-cap-instead-of-normalizing-it.md`
records for the tabled alternative — repoint that row at the retiring
commit's parent.

Signed: (TESS orchestrator)
