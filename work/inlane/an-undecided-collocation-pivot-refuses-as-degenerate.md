---
id: an-undecided-collocation-pivot-refuses-as-degenerate
kind: issue
title: Past the fit's reach a wide loft refuses LsqDegenerate { pivot: NaN }: misnamed, no margin, no recourse (a regression from CarrierKnotNotAPoint)
status: open
opened: 2026-10-19
priority: P2
---


Found by both reviewers of C6 S5c (PR 5332): reviewer A's S1 and reviewer B's M3. The INLANE orchestrator ruled it a regression in the refusal, not only a reach limit, and so its own P2 item.

## Finding

Before S5c, a loft over a section wide enough to widen an interior averaged knot refused in the fit, as `SkinError::Fit(FitError::Structure(CarrierKnotNotAPoint { index }))`, naming the knot. S5c lets the fit solve over the wide knot, and inside the solve's reach the wall then refuses at its carrier's door, still naming the knot.

Past the reach (`work/inlane/the-fits-enclosure-solve-reaches-only-narrow-parameters.md`, from a half-width of about 0.01–0.05 at degrees 2–3), the same loft now refuses `SkinError::Fit(FitError::Lsq(LsqError::LsqDegenerate { pivot_index, pivot: NaN }))`.
- **The refusal is minted** in `crates/geom/src/curves/banded.rs`, `factor`'s pivot check (`structure_nonzero(pivot)`, then `pivot: pivot.value_channel()`).
- **It reaches the fit** through `crates/geom/src/curves/fit.rs`, `interpolate_columns_in` (`banded::factor(..).map_err(FitError::Lsq)`).

That refusal is wrong three ways:
- **Misnamed.** Every member's collocation matrix is regular: it is totally positive, as the parameters satisfy Schoenberg–Whitney. The pivot is undecided over the enclosure, not degenerate.
- **No margin.** `pivot` is the value channel of a wide pivot, which is NaN. C6 asks that a refusal on which members may disagree carry its margin, so a caller such as E6 can bisect.
- **No recourse.** It names no parameter to narrow.

## Owed

A refusal of its own where the pivot is wider than a point, kept apart from a true `LsqDegenerate` at a point pivot. For example:
- `banded::factor` reports the undecided pivot as its interval, through a crate-local error;
- the fit maps that to `FitError::CollocationUndecided { pivot_index, margin }`, with a recourse ("narrow the parameters the sections' enclosures give, or supply them as points").

Its cost:
- `FitError` is public and matched across geom-brep, sweep, topo and pncad;
- the pncad-py tag table (`crates/pncad-py/src/tags.rs`) needs a row;
- the reporting-margin-door gate needs the new mint;
- the refusal-recourse rows need its message.

Rows owed:
- a wide loft past the reach refuses the new variant with a margin straddling zero and one recourse;
- a point-singular system still refuses `LsqDegenerate`.
