---
id: patch-bound-offset-fit-recentring-origins
kind: issue
title: patch_bound and offset_fit recentre the same homogeneous nets against different origins — unifying the two centres needs a measurement
status: open
opened: 2026-09-01
github: 1532
refs: [1403, 1006]
---

## From GitHub issue 1532

Opened 2026-09-01; 0 comments.

(S-CERT orchestrator) Found by CERT-10's second reviewer (PR 1403): `patch_bound::comp_nets`' doc had assigned "unifying the two centres" to "issue 1006, CERT-10, which owns this seam" — i.e. to the unit that chose to keep the seam, a residue with no owner. The doc now names this issue instead. The lane's own statement of the residue:

CERT-10 (issue 1006) unified the two channel extractions' STORAGE — both build a `geom_core::spline::net::TensorNet`, and the flat/nested bridge is deleted. What it deliberately did NOT unify is the recentring origin. `patch_bound::comp_nets` extracts `w·P` and recentres LATER and per cell, off the cell's own control window (`window_tilde_hull`), because a cell-local hull is what it reads. `offset_fit::channel` extracts `w·(P − c)` against a WHOLE-PATCH origin, because its net feeds polynomial products formed once over the merged break structure, where the ring's rounding scales with the coordinate. Both are correct for their consumer; they are different centres because they are read at different granularities.

Unifying them means deciding whether the composite lane can afford a per-cell centre (it re-forms its products per cell, so the answer is a cost measurement on `offset_fit`'s own numbers) or whether the cell lane must give up its tighter one (it would widen every offset certificate, which is measurable against the micron row). Either way it is a measured decision, not a refactor, and CERT-10 was the wrong unit to make it. Filed so it has an owner. S-CERT-adjacent ground (`geom-brep`); the seam is cross-referenced from both sites; the storage-shape sibling (`spline::compose`'s transpose bookkeeping) is recorded in CERT-10's sweep table as Track R/C consolidation ground. No live claimant.

## Home

`work/cert/` — both sites, `crates/geom-brep/src/patch_bound.rs` and `crates/geom-brep/src/offset_fit.rs`, are named territory globs of S-CERT, and it is CERT-10's own residue.
