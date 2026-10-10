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

## Weighed (design fork, 2026-10-10)

Two designers weighed this one, under fork-log row 103. They converged on the core:
- A certified bound is a kind of quantity, so it is recorded on the check (`Unsized { Defect, Fit, Bound }`), not patched at the door.
- A bound refusal ends in the kernel-limit last resort at every reading, at rest included: a loose hull contradicts nothing.
- At the import door, a bound within ε_in gets a sentence that names the bound (`MissReading::Bound`).
- `TangentHull` is in the class, and the AnalyticRung3 hull limb gets its own check.
- The filed [worst, sup] repair is wrong: limb 2 runs only after limb 1 passed every sample, so `worst ≤ ε` always, and the stopgap it offers fails.

That core touches no ratified text and is implemented as its own unit.

For Ev (the `[ev]` PR): retire `ssi_foot_orthogonality` and reword C2 limb 1. Its margin `|S_u·r|/|S_u| ≤ |r|` uses the same `r` and the same point as the distance limb, so it can refuse only on interval slop. The clause traces to the CURVED-DESIGN draft and PR 7's binding spec, so it may be ratified.

**Ev, 2026-10-10 (PR 4498):** retire `ssi_foot_orthogonality` and reword C2 limb 1 ("nice catch, sounds good!"). The C2 text lands with that PR. Removing the limb from code (its check, `SsiLimb::FootOrthogonality`, the `projection.rs` module doc's three-residual story and `RoundMargin::Over`'s reading of it) is the row `foot-orthogonality-limb-is-retired-in-the-code`.
