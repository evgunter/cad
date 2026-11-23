(join-split implementer) A2: check_volume on the head's trimmed-cap cells. Default ε, with the body's volume and area from topo::mass_properties. 435 cells, of which 406 mesh and 29 refuse.

**No certificate defect.**
- Every meshed cell PASSES check_volume, all 27 FLAT and all 28 under-half among them.
- Every flat or under-half cell sits at δ ≥ 3h. Its δ·A/V is 12.0 (δ = 3h) or 38 to 40 (δ = 10h), so the bound is vacuous there, as the designer said.
- No cell at δ ≤ h is flat or under half. At δ = h, 0.3h and 0.1h the head's caps are positive and within half of main's.

Flat cells, as (h, δ = k·h):
- h 1e-3: 3h ×3 (lat −75, az 30/120/210); 10h ×8 (lat −60 az 30/210; lat −75 az 30/120/210; lat 45 az 120; lat 60 az 120; lat 75 az 210)
- h 1e-2: 3h ×3 (lat −75, az 30/120/210); 10h ×9 (lat −30 az 210; lat −60 az 210; lat −75 az 30/120/210; lat 0 az 120/210; lat 60 az 120)
- h 3e-2: 10h ×5 (lat 0 az 120/210; lat 45 az 120; lat 75 az 120/300)

Under half of main, as (h, δ, head ratio / main ratio):
- h 1e-3:
  - δ 3h: lat 75 az 120, 0.395/0.802
  - δ 10h: lat −30 az 120/210, 0.300/0.763; lat 0 az 120/210, 0.128/0.717; lat 45 az 210, 0.075/0.728; lat 75 az 30/120, 0.224/0.748
- h 1e-2:
  - δ 3h: lat 75 az 120, 0.015/0.839
  - δ 10h: lat −30 az 120, 0.148; lat −45 az 120, 0.073; lat −45 az 210, 0.300; lat 30 az 120, 0.074; lat 45 az 120, 0.073; lat 75 az 30/120/210, 0.258/0.271/0.084 (main 0.729 to 0.765)
- h 3e-2:
  - δ 10h: lat ±45 az 300, 0.238/0.656 (three faces); lat −30 az 120/210, 0.071/0.296; lat −45 az 120/210, 0.137/0.299; lat −60 az 120/210, 0.057; lat 30 az 120/210, 0.071; lat 60 az 120, 0.250 (main 0.727 to 0.763)

The 29 head REFUSALS (FoldedTriangle from the join's piece check, where main meshes) are not bounded the same way. They run down to δ 9e-3 at h 3e-2, which is 0.3h: lat ±75, az 30/120/210/300, at δ 0.3, 0.09, 0.03 and 0.009. A lost body is not a δ ≥ volume/area case.

A3 (the foot table) is building now.
