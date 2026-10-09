---
id: hull-sup-limb-reads-its-bound-as-the-miss-at-the-import-door
kind: issue
title: geom-brep: a hull-sup limb refusal's certified upper bound is read as the miss at the import door
status: open
opened: 2026-10-09
priority: P3
cost: M
---



(Filed by the ENCL orchestrator from the full review of PR 4427, `encl/limb-refusal-margin`. Not taken in that PR.)

## What

After PR 4427, both SSI limb refusals carry their reporting margin to the import door: `RefusedArm::SignCertain(Some(margin))`, read by `Unsized::residual_in_file` as `MissReading::Definite`. That is exact for a limb whose reading is a measured miss (`ssi_on_locus`, `ssi_on_locus_foot`).

Three limbs are different: `ssi_hull_sup` and `ssi_hull_sup_chart` in `crates/geom-brep/src/ssi/certify.rs`, and the `net_offset_sup` hull limb in `crates/geom-brep/src/edge_nurbs.rs` (AnalyticRung3). What they refuse on is a certified **upper bound** on the miss, not the miss. The door words that bound as the miss:
- A bound within ε_in reads "lies within the file's declared coincidence distance". That is sound, because the miss is at most the bound.
- A bound past ε_in reads the at-rest ending, "There is no way through: this is a kernel defect or a damaged file". That claims the miss is past ε_in when only the bound is. A loose hull is the certificate's own limit, and the true miss may lie within ε_in.

`ssi_foot_orthogonality` is a related case. Its reading is `Margin::levered_inv(orthogonality residual, speed)`, which measures how far the foot is from being a foot, not a carrier-to-surface distance.

## Repair shape

Give the hull limbs an enclosure reading of [sampled `worst`, `sup`]; `analytic_limbs`/`nurbs_limbs` already compute `worst`. With that reading, `Within::Partly` gives "may lie within" when worst ≤ ε_in < sup.

Building that `MarginDiag` needs a door-sanctioned constructor. `scripts/gates/reporting-margin-door.sh` counts mints, so the shape is a design question: weigh it per the fork protocol before implementing.
