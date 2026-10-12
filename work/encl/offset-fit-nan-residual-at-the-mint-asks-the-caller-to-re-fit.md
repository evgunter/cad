---
id: offset-fit-nan-residual-at-the-mint-asks-the-caller-to-re-fit
kind: issue
title: offset fit: a NaN sampled residual inside the fit loop reads as a stored surface to re-fit, which the minting op's user never stored
status: closed
closed: 2026-10-09
branch: encl/offset-fit-mint-nan-limb
pr: 4395
opened: 2026-10-09
priority: P3
cost: E
---


(ENCL, from the sweep of the shell-wall curvature-recourse unit.)

## What

`fit_offset_at` (`crates/geom-brep/src/offset_fit.rs`) returns
`OffsetFitError::Limb` at `OffsetLimb::OnLocus` when a round's sampled
residual is NaN, inside the fit loop. That arm's `Display` is written
for the re-derivation door (`certify_offset`): "a stored offset surface
does not hold to the tolerance ... Recourse: re-fit the offset at this
tolerance".

At the mint nothing is stored yet, and the caller's re-fit is the call
that just refused. The shell op (`crates/topo/src/shell.rs`,
`AsShelled`, through `ReplaceFaceError::Fit`) and the transform's
re-fit both reach it this way; the chain roster carries the row
(`refusal_concision_chains.rs`, `offset_fit_routes`, `Limb`).

`Shell/Face/Fit/Limb` at `OffsetLimb::HullSup` renders the same
"stored … re-fit" text on the shell route, and is likely unreachable
at the mint: `fit_offset_at` raises `Limb` only at `OnLocus`, and a
hull bound over the tolerance there continues the loop instead.

## Repair shape

A NaN residual on a face the door's meters accepted is the kernel's,
so the mint's arm likely wants the defect ending, told apart from the
at-rest limb by its route rather than by the same variant.

## Closed

2026-10-09. PR 4395 merged at `67c756d147` after a review and a fix pass; hosted CI was green.
- `OffsetFitError::MintLimb` is raised only at the mint (`nan_residual_at`, and `mint`'s own certification of the fresh fit) and ends in `KERNEL_DEFECT_ENDING`. Only `OnLocus` reaches it; `HullSup` is the same measure on the same fit.
- The at-rest `Limb` keeps `LIMB_REFIT_RECOURSE`.
- No arm prints "NaN m".
- The PCTAIL row `pcurve-image-mismatch-at-the-body-mint-asks-to-re-mint` was filed alongside.
