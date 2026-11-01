---
id: patch-bound-offset-fit-recentring-origins
kind: issue
title: patch_bound and offset_fit recentre the same homogeneous nets against different origins — unifying the two centres needs a measurement
status: open
opened: 2026-09-01
refs: [1403, 1006]
priority: P1
cost: M
design: true
---

## From GitHub issue 1532

Opened 2026-09-01; 0 comments.

(S-CERT orchestrator) Found by CERT-10's second reviewer (PR 1403): `patch_bound::comp_nets`' doc had assigned "unifying the two centres" to "issue 1006, CERT-10, which owns this seam" — i.e. to the unit that chose to keep the seam, a residue with no owner. The doc now names this issue instead. The lane's own statement of the residue:

CERT-10 (issue 1006) unified the two channel extractions' STORAGE — both build a `geom_core::spline::net::TensorNet`, and the flat/nested bridge is deleted. What it deliberately did NOT unify is the recentring origin. `patch_bound::comp_nets` extracts `w·P` and recentres LATER and per cell, off the cell's own control window (`window_tilde_hull`), because a cell-local hull is what it reads. `offset_fit::channel` extracts `w·(P − c)` against a WHOLE-PATCH origin, because its net feeds polynomial products formed once over the merged break structure, where the ring's rounding scales with the coordinate. Both are correct for their consumer; they are different centres because they are read at different granularities.

Unifying them means deciding whether the composite lane can afford a per-cell centre (it re-forms its products per cell, so the answer is a cost measurement on `offset_fit`'s own numbers) or whether the cell lane must give up its tighter one (it would widen every offset certificate, which is measurable against the micron row). Either way it is a measured decision, not a refactor, and CERT-10 was the wrong unit to make it. Filed so it has an owner. S-CERT-adjacent ground (`geom-brep`); the seam is cross-referenced from both sites; the storage-shape sibling (`spline::compose`'s transpose bookkeeping) is recorded in CERT-10's sweep table as Track R/C consolidation ground. No live claimant.

## Home

`work/cert/` — both sites, `crates/geom-brep/src/patch_bound.rs` and `crates/geom-brep/src/offset_fit.rs`, are named territory globs of S-CERT, and it is CERT-10's own residue.

## Re-homed (2026-09-06)

Moved from `work/cert/` to `work/props/` on S-CERT's exit walk PR
(#1924, its handoffs ledger; merged by Ev 2026-09-06 = ratified), before
`work/cert/` was deleted at sweep 7 of `docs/DOC-LEDGER.md`. Id, body
and header are unchanged; the directory is the claim (`work/README.md`).
The `## Home` section above naming `work/cert/` is superseded by this
line and is kept as the record of why the file was filed there.

## Un-parked (band-rule triage, 2026-10-25)

Parked 2026-09-26 on `f64-refinement-inside-an-enclosure-has-five-more-sites` because the lerp-form width in the Bézier decomposition swamped any per-cell centre, so the measurement had to wait ("Measure after", `work/encl/log.md`). That was the row's site 5, and PR 3524 closed it. The one site still open there, `crates/mesh/src/chords.rs`, is not on either recentring path, so the measurement can be taken now. Re-priced from the legacy `cost: D`: `M`, with `design: true` for the open choice between the two centres.

## The home (JETFRAME, CLEAR, 2026-11-01)

A rational net's first derivative over a cell now has one home in
`geom`: `net::rational_slope`, reading the cell's homogeneous net about
a point of the cell (`net::cell_origin`, `net::homogeneous_about`).
`B = A − c·w` is the homogeneous form of `S − c`, whose derivative is
`S`'s for any constant `c`, so `c` is never certified and costs only
width. `NurbsSurface::rect_jet` and `NurbsCurve3::span_deriv` read
through it, and their widths no longer move with the net. This row was
not in reach of that unit, and was not fixed in it. The home is
`pub(crate)` in `geom`; a caller in another crate needs it exported, or
a door in `geom` that reads through it.
