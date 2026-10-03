---
id: ssi-step-max-is-a-sampling-heuristic
kind: issue
title: ssi: SSI_STEP_MAX (the longest march step, 1/32 of the extent) is a sampling heuristic standing in for the certificate; refine by certification instead
status: open
opened: 2026-10-03
priority: P2
cost: M
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
