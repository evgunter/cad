---
id: ssi-step-max-is-a-sampling-heuristic
kind: issue
title: ssi: SSI_STEP_MAX (the longest march step, 1/32 of the extent) is a sampling heuristic standing in for the certificate; refine by certification instead
status: closed
opened: 2026-10-03
priority: P2
cost: M
pr: 3998
branch: ssi/step-max-certify
closed: 2026-10-03
---


(SSI orchestrator, from Ev's question on PR 3862, 2026-10-03: "is SSI_STEP_MAX necessary? having a longest step constant sounds like another heuristic". Ev agreed with the direction below: "sounds great!")

## What

`SSI_STEP_MAX` (`crates/geom-brep/src/ssi/march.rs`) caps every realized march step at 1/32 of the caller's feature extent. Soundness does not rest on it: every candidate is certified (C2's three limbs), so the cap only shapes what the march proposes.

The step is set by curvature against ε (`h_fit`, `h_quad`, `h_cub`). On a straight or nearly straight branch those bounds are unbounded, and the cap is what still gives the fit its samples. So the cap stands in for the certificate: it guesses how many samples a carrier will need instead of asking.

## Direction (Ev agreed)

Refine by certification:
- March with the curvature rule alone.
- Fit, then certify.
- Where the certificate refuses a span, subdivide that span and refit.

The carrier then gets as many samples as its certificate needs, and no free constant is involved.

## First, check what else reads the constant

Check whether any part of the march itself, rather than the certificate, relies on the cap. Candidates:
- the step-progress decision;
- the re-march and known-end step `|AB|/SHORT_BRANCH_STEPS`, which is capped by it;
- `SSI_IDEALIZED_STEP`;
- the boundary search on the ℝ³ lane.

Sibling extent fractions to weigh in the same pass, since each is a fraction of the extent:
- the tube ladder's widest rung (`SSI_TUBE_RADIUS_MAX`);
- the seeding floor (`SSI_SEED_FLOOR`).

## Done when

- `SSI_STEP_MAX` is gone, or derived from what the certificate needs.
- The SSI suite's answers hold at every ε.
- Any moved rows are re-pinned with a reason for each.

## Closed (2026-10-03, PR 3998)

`SSI_STEP_MAX` is gone. A realized step is the curvature rungs'
against ε, bounded by the domain's diagonal, and by `|AB|/5` between
known crossings (or `length/5` on the ℝ³ short-branch re-march), the
fit's sample minimum. Where limb 1 or 2 refuses the fitted carrier, the
certificate locates the refused spans, every gap between samples they
meet is halved (the midpoint settled onto the locus), and the carrier
is refitted (`march::refine_by_certificate`). A gap is halved only
while half of it clears the band, and no round overruns the fit budget;
then the certificate's refusal stands. The uncertified door samples the
same way.

Outcomes moved: every curved-dome cut certifies at ε 1e-6 and 1e-9
(cause 4 of `plane-nurbs-ssi-does-not-certify-a-curved-dome`), as do
the weight-9 rational wall's edge cut and the multicell wall; a straight
metre takes 6 samples where it took 33. Branches certified before and
after take 4–7% fewer samples. The cost is the refits: the dense
collocation solve dominates a round, filed as
`work/flux/the-interpolating-fit-solves-a-banded-collocation-system-densely.md`.

The extent's other fractions stay, each for its own reason (PR 3998's
readers table): the idealized step defines the spec stepper, the tube
ladder's widest rung is a separation scale, and the seeding floor only
finds what the ε-tied accounting floor proves. The fixed pcurve schedule
the sweep found is
`work/ssi/the-cylinder-chart-ellipse-pcurve-samples-a-fixed-schedule.md`.
