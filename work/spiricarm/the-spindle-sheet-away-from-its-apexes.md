---
id: the-spindle-sheet-away-from-its-apexes
kind: unit
title: Surface::Spindle stage 1: the sheet away from its apexes (datums, shared core, revolve's arc classification, window-form readers), landing the football with conical ends
status: dispatched
opened: 2026-11-03
priority: P0
cost: H
parent: horn-and-spindle-tori-have-no-representation
branch: spiricarm/spindle-sheet
---


Stage 1 of `horn-and-spindle-tori-have-no-representation`, which Ev ruled on PR 6464 (2026-11-03). What is decided is in that row's `## Ruled` section, in `crates/topo/README.md`'s structural conventions (the arc rule), and in `docs/DESIGN.md` [surfaces] (the kinds list and the locus sentence). The designers' full reports are PR 6464's body. Read them: they name the readers.

## Phase 0: a census and a staging proposal, reported before code

1. **Every exhaustive `match` on `Surface`** that the new variant reaches. Group the sites by what a `Spindle` arm needs:
   - agnostic: datum moves, transforms;
   - the shared revolved-circle core;
   - a window form, `inf |R + r cos v|` over the face's `v` window;
   - a typed "no lane yet" refusal (the ring-only closed forms: the spiric, the plane×torus poses, `torus_region`'s path of circles, `ring_path`, `sector_face`, `injective`'s `RING_TORUS`);
   - an apex arm (stage 2's, refused typed in stage 1).
2. **Every non-exhaustive kind test** (`matches!(… Surface::…)`, `if let Surface::Torus`, `== SurfaceKind::…`). The compiler will not list these. `pcurves.rs`'s `has_singular_points` is one. Record the grep shapes you used and what they cannot match (implementer discipline §5).
3. **The sites the designers named as assuming a ring undecided**:
   - `topo::boolean::carrier_touch::reach` (`min(r, R − r)`);
   - `props/curved.rs`'s dormant arm;
   - `implicit.rs`'s `(R − r).max(0)` curvature lever.
4. **A unit cut.** If stage 1 is more than one PR's work, propose the cut, each piece a PR that leaves main sound. A natural seam is the variant plus every arm refusing typed, then the readers that make the conical-ended football build. Then proceed with the first piece, unless the cut changes the design.

## What stage 1 makes true

- `Surface::Spindle { center, axis, major_radius (signed), minor_radius, u_ref }`, one sheet: `ρ = R + r cos v ≥ 0`, `|v| ≤ v*`, `cos v* = −R/r`. Datums are decided at rest: `r > 0`, `R ≠ 0`, `−r < R ≤ r`.
- **One revolved-circle core** (jet, residual, inverse `v = atan2(h, ρ − R)`, normal), shared with `Torus`, not copied. With signed R, the residual and inverse are exact on the sheet.
- **The revolve reads the arc**: one half-plane check on the arc's interior for both classes. Then the signed centre distance picks the kind: zero gives a sphere, more than `r` gives a ring torus, anything else gives a `Spindle` with `R = rc`. The near-horn band refuses as a sliver, as `ArcCenter` and `LineRadial` already do. `UnsupportedToroid`, `ArcClearance` and their recourse text retire.
- **A face whose window reaches an apex** refuses typed in stage 1, as an apex is stage 2's work.
- **The football with conical ends builds through the public revolve door**: a band of the lemon between two cones. Its oracles:
  - the exact volume of revolution, by Pappus or by direct integration of the lemon band in meridian coordinates;
  - its area;
  - its tessellation (watertight, `admesh`);
  - tier 3.

  Also add an apple band, and a lemon band off the origin translated by 1e3.
- **A demo**: add the conical-ended football to the tour if a natural spot exists (`docs/prompts/implementer-discipline.md` §3, `memories/demo-purpose.md`). Write it as a user would.

## Not in stage 1

- The apex (stage 2).
- Offset's `TorusRing`, blend's `SpineIrregular`, and STEP's `DEGENERATE_TOROIDAL_SURFACE` (each a row once stage 1 lands; file them).
- Pair sections against a sheet beyond what the football needs (typed "no lane yet").

Review tier: **dual**. A new surface kind is an architectural decision with broad impact, and hard to change later.
