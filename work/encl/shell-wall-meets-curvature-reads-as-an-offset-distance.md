---
id: shell-wall-meets-curvature-reads-as-an-offset-distance
kind: issue
title: offset fit: CurvatureHeadroom's recourse speaks of an offset distance and its side, which a shell user sets as a wall thickness
status: review
branch: encl/shell-wall-curvature-recourse
pr: 4377
opened: 2026-10-06
priority: P3
cost: E
---


(SHELL refusal-text lane, PR 4163, from the style review's S2 sweep.)

## What

The shell op moves each face by the wall thickness, signed by the
face's sense (`crates/topo/src/shell.rs`, `inward`). PR 4163 renders
the replace-face arms that speak of the move's length in wall terms
under the shell (`AsShelled`, same file). Those arms are
`ReanchorCollapse`, `ApexWindow`, `RadiusFloor` and `TorusRing`.

The fit meter's `CurvatureHeadroom` reaches the shell user through
`ReplaceFaceError::Fit`, and it still ends "Recourse: use an offset
distance of smaller magnitude, or offset to the other side"
(`geom-brep/src/offset_fit.rs`, the meter's recourse;
`refusal_concision_chains.rs`, `meter_verdicts`' `DISTANCE`). A shell
user sets a thickness and cannot pick a side.

## Why it is filed rather than fixed there

The text is ENCL's, and the transform's re-fit reaches it too. The
chain guard pins that ending on both routes
(`every_offset_fit_refusal_ends_exactly_once`). A shell-only rendering
therefore has to split that pin by route.

## Repair shape

Give the meter's recourse the wall wording when it is read under the
shell, or render the arm in `AsShelled` with the pin split per route.
