---
id: spiric-rim-carrier
kind: unit
title: The exact spiric rim carrier - Curve3::Spiric, its census and mint_carrier's kind-changing arm (PR-1a), then the pcurve variant and STEP (PR-1b)
status: open
opened: 2026-09-13
branch: curved/spiric-1a
refs: [spiric-carrier-ruling, c5-plane-torus-cone-cylinder-arms, 1858]
priority: P1
cost: H
---


## What

`docs/CURVED-SPIRIC-SPEC.md` (ratified 2026-09-13, executing Ev's
rulings on PR #1858): the klein elbow's partial-revolve rim on a torus
wall is a spiric of Perseus, carried as `Curve3::Spiric` — an
`Ellipse`-style special case in the torus's own minor angle — minted by
`offset_axial.rs`'s `mint_carrier` as the module's one kind-changing
arm. Two PRs: 1a (the kind, the dispatch census, the mint, the suite
flips; the elbow moves from `TogetherAxialEdge` to check 7's props
door), 1b (the exact `Pcurve::Spiric` variant and the export-only STEP
spline). The props quadrature lane is a separate unit after both.

## PR-1a merged (2026-09-19)

PR #2566 merged (ordinal 2204, sample #221; block CURVED-B2 slot 1
concluded): `Curve3::Spiric` with its deciding constructor, the census
and `mint_carrier`'s kind-changing arm; the rims mint. The unit stays
OPEN for PR-1b (`Pcurve::Spiric` + STEP, rows 9–10, branch
`curved/spiric-1b`), which draws its own slot. The STOP-1 finding (the
elbow's equator seams refuse at `reauthor`) is
`equator-seam-reauthor-refuses-the-hollowed-elbow`, after 1b. Record:
MODEL-AB-LOG row SP1A; adjudication comment 5733631165.
