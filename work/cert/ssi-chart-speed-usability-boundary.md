---
id: ssi-chart-speed-usability-boundary
kind: issue
title: ssi: wrong diagnoses survive at finite-but-unusable speeds — the usability boundary is ~5.6e-312, not 0, and both guards test only the class
status: open
opened: 2026-08-29
github: 1238
refs: [762, 1221]
---

## From GitHub issue 1238

opened 2026-08-29, 0 comments.

**Filed from CERT-2's fix pass (S-CERT). Class issue — deliberately not fixed in PR 1221**, whose fence covers the non-positive-finite *class* guards (issue 762's residue) and not the design question below. Measurements are the blinded reviews' probes; reproduction sources are the two filed reviewer branches **`cert/2r1-probes`** and **`cert/2r2-probes`**.

## The class

Both chart-speed guards (`ssi.rs`'s seeding guard, `ssi/march.rs`'s stepper guard) refuse exactly `!speed.is_finite() || speed <= 0.0`. That is the right *class* test for issue 762's defect, but it draws the usable/unusable boundary at 0 and ∞ when the arithmetic downstream draws it orders of magnitude inside the positive-finite range: a speed can pass the guard and still make `h = (SSI_IDEALIZED_STEP · extent)/speed` overflow, or drive the translated floors below every representable cell.

## Measured (reviewer probes, reproducible from the branches above)

- **march, speed `1e-320` and `5e-324`** (positive finite): the step `h` overflows to `+∞`, the marcher cannot place a sample, and the caller is told `SeedRefinementFailed` — a wrong diagnosis; the speed was never usable.
- **march, speed `1e-300`**: silent `Ok` with a step of **~2e297 m** — no refusal at all, and a number no consumer can mean.
- **the real march usability boundary is ~5.6e-312**, not 0: below it `h` leaves the finite range at this extent; the guard's `> 0.0` admits ~5.6e-312 worth of unusable window.
- **seeding lane, net magnitude `1e150` through the public door**: chart speed ≈ `1e150` is finite, the guard passes it, the translated floors land at ~`1e-152`, and the sweep runs to `CellBudget` — the budget answering in the guard's place, the same wrong-diagnosis substitution issue 762 recorded for `+∞`.

## The design question (this issue's, not PR 1221's)

Two fix shapes, with different reach:

1. **A usability-class predicate at the guards** — refuse when the speed cannot translate this context's floors/steps into the finite range (a function of speed, extent, and the floor constants, not of speed alone).
2. **Guard the derived quantities instead** — `h`, `h_meters`, and the translated floors each check finiteness where they are minted, so the boundary is wherever the arithmetic actually is.

(1) keeps the refusal at one named door but hard-codes the downstream arithmetic's shape into the guard; (2) is local and exact but multiplies refusal sites. Either way the diagnosis must name the speed, not the budget or the seed refinement.

## Where the guards point here

`ssi/march.rs`'s guard comment and PR 1221's body both state the guard's obligation as the non-positive-finite class and cite this issue for the finite-but-unusable window it leaves open.

Refs: issue 762 (the class the guards do close), PR 1221 (CERT-2), reviewer branches `cert/2r1-probes`, `cert/2r2-probes`.

## Home

`work/cert/` — S-CERT's charter names interval-mode honesty and the chart-speed guards explicitly, and the issue was filed from CERT-2's fix pass.
