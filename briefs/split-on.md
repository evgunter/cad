# Lane `heldrecords/split-on` — P1, step 3 of the plan

Row: `work/heldrecords/a-curved-slit-fails-tier-3-prime-on-its-two-coincident-arcs.md` (P1, M), read in full, especially `## Decided (2026-10-28, Ev on PR 5967)`: the slit's two coincident arcs are two cells (one curve entry is NOT an option); **the split records them as an edge–edge pair, citing a new `SplitOn` row (the operand edge ON the plane); the census confirms the record as a curve touch; `CurvedSlitTrace` (`Sweep::section_kind`) and `EdgeInFaceGap::Curved` (`topo::boolean::slit::part_riders`) come out once the record lands.** Also read `work/heldrecords/plan.md` steps 2–3 and the new row `work/heldrecords/a-cut-across-two-curved-or-partly-overlapping-edges-on-one-locus-mints-two-point-keys.md` on `origin/heldrecords/split-keys` (step 2's lane filed it; it is the curved crossing-key instance and is NOT yours unless it falls out).

**Stacked**: branch `heldrecords/split-on` off `origin/heldrecords/split-keys` (PR 7421, step 2, in review; itself stacked on `embed/records` / PR 7124, where `AtRestBody` holds records and the split hands `finish` the records it decided). PR base `heldrecords/split-keys`. Merge it in when it moves. Do not push to either base.

Owed:
1. The `SplitOn` lineage/provenance row (where the split's lineage rows live; read how other rows are cited by records) and the split's own edge–edge contact record for every coincident pair of edges it mints, straight or curved (Ev: "coincident edges an op mints, straight or curved, are recorded"). Straight ones pass today only through `census.rs` `ee_collinear_lane`'s shared-bounds exemption.
2. The census confirms that record as a curve touch (`sweep_curved_edge_edge`, pass 5′); a record the census cannot confirm refuses at the split's gate (that is PR 7124's gate).
3. Lift `CurvedSlitTrace` and `EdgeInFaceGap::Curved`; the ring form and the one-end slit build and pass tier 3′ against their records, both normals, the V's tip at 0 and ±0.6ε, at three ε; the Boolean's twin (`a_curved_keel_on_the_floor_refuses_its_slit_form.rs`) builds both forms at the closed-form volume with 3′.
4. Do NOT retire the shared-bounds exemption in this PR. Measure instead: with the exemption off, which tests refuse (the split's straight pinches should now be recorded; the Boolean's pinch and slit lanes may not be). Put the list on `work/heldrecords/plan.md`'s step 3 owner row as a dated section of the curved-slit row, and file a row for the Boolean lanes' records if they are the remainder.

Review tier: single FULL review.

Standing rules: `briefs/common.md` on this branch.
