---
id: the-section-value-claims-a-cone-pairs-winding-on-the-partner
kind: unit
title: The section value claims a cone pair's winding on the partner: cone_ruled reads each loop's turning about the cylinder or second cone, so topo's classes survive the move onto the value
status: closed
opened: 2026-11-22
priority: P1
cost: M
parent: cone-germ-pairs-against-a-curved-face-have-no-section-frame
branch: sectcone/cone-partner-winding
closed: 2026-11-23
pr: 7455
---

## Closed (2026-11-23, PR 7455)

`cone_ruled` claims a loop's winding about the partner (a cylinder or second cone). It reads each loop's turning about the partner's axis from the cone sweep's own cells: the loop is homotopic to the polygon through its pieces' joints, each piece's hull is decided clear of the axis, and each chord turn is decided within `TRUSTED_TURN`. A read that doesn't settle claims nothing and records why (`ConeRuledPose::unclaimed_windings`). A missing claim is weaker, never wrong, so it never refuses the section (orchestrator's ruling, log 2026-11-22). Rows assert topo's classes both ways round. Dual Opus review: both APPROVE-WITH-FIXES, no MAJOR. Follow-ups are filed as `the-partner-winding-lever-is-spelled-twice`.


**Review tier: dual** (two Opus reviewers). New certified code in geom-brep. Difficulty letter: M.

Found by U2's lane (`the-section-certificate-reads-the-cone-pairs-on-the-value`), which stopped before building.

## What

`pair_section/cones.rs` `cone_ruled` sets every component's winding with `pose.order([arc.encircles, false])`. It sweeps only the ruled cone, so it never claims a winding about the cylinder or partner cone. Topo's section certificate reads a chart per carrier (`charted`) and claims a class on both. Moving topo onto the value as it stands would weaken the oracle rows in `section_cert_cone_pair_rows.rs`:
- `a_rod_through_the_cone…`: (F,T)×2 → (F,F)×2;
- `a_fat_steep_wall…`: (T,T) → (T,F);
- `a_narrow_cone_through_the_side…`: (F,T) → (F,F);
- `a_parallel_axis_cone…`: (F,T) → (F,F);
- `a_twin_cone_with_its_apex_inside…`: (T,T) → (T,F);
- the coaxial and parallel cone × cylinder poses.

W2 then clears less, and a component essential only on a `Sheet::One` partner face refuses where it used to clear. The orchestrator ruled that this unit goes first (log, 2026-11-22). Weakening the oracle evidence was not taken.

## Done when

- **The value claims both windings.** `cone_ruled` claims `winds` on the partner (cylinder or second cone) as a certified decision.
  - Prefer reading each loop's turning about the partner's axis from the sweep's own cells, with `pair_section/winds.rs`'s lever, so the component carries one claim from one sweep.
  - A second sweep ported from topo's wall chart is the fallback. If you take it, say why the first could not certify.
  - An undecidable winding escalates; it never guesses.
- **The value's rows assert the classes.** Its own rows assert the partner winding at least where topo's oracle rows do: the rod through the cone, the fat steep wall, the narrow cone through the side, the parallel-axis cone, the twin cone with its apex inside, and coaxial and parallel cone × cylinder. Each row goes red if the claim is dropped.
