---
id: step-import-adopts-frame-directions-on-eps-in-that-check-1-refuses-at-eps
kind: issue
title: step-import adopts a DIRECTION and a ref_direction verbatim within the file's eps_in, dimensionless and unlevered; a cylinder so imported can refuse at check 1 at the ambient eps
status: open
opened: 2026-09-25
priority: P3
cost: D
refs: [ATREST-13, step-import-restates-the-cone-half-angle-convention]
---


## What

`step-import`'s resolver adopts a stated `DIRECTION` verbatim when
`|‖d‖ − 1| ≤ ε_in` (`crates/step-import/src/entities.rs`,
`Resolver::direction`), and a stated `ref_direction` verbatim when
`|axis · u_ref| ≤ ε_in` (`Resolver::placement`; the projection that
would re-derive it runs only past that window). ε_in is the file's own
declared uncertainty, and both windows are dimensionless and unlevered.
The body is then certified at the AMBIENT ε.

ATREST-13 (PR 3238) made tier 3's check 1 read every analytic frame's
unit-ness and orthogonality to within ε of locus movement at the kind's
radius (`geom::Surface::representability_margins`, the frame margins
in `crates/geom/src/convention.rs`). So a file with ε_in = 1e-6 can
import a cylinder whose `u_ref` is 5e-7 off unit, and check 1 refuses
it at the ambient ε = 1e-9 once `r·5e-7 > 1e-9` — any `r > 2 mm` — as
`UnrepresentableSurfaceDatum`, naming a face rather than the file's
direction record.

Found by reading. The measured corpus does not do it: ATREST-13's
instrument over the wild STEP corpus saw a worst `u_ref` 4.2e-13 off
unit (1.7e-14 m of movement), because the committed files print their
directions to full precision.

## The shape

The same kind as `step-import-restates-the-cone-half-angle-convention`:
a mint-time window stated in `step-import` beside a convention `geom`
now computes. Two ways it could close, not chosen here: renormalize
(and re-project) whenever the window is not exact — dropping the
"verbatim" adoption for frames, as the reader already does past ε_in;
or state the window in metres by the placed surface's radius, which
the placement does not yet know.

## Fence

`crates/step-import/src/entities.rs` (exch's ground).
