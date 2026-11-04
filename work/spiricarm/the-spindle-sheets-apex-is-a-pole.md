---
id: the-spindle-sheets-apex-is-a-pole
kind: unit
title: Surface::Spindle stage 2: the sheet's apex as a pole, landing the football (an arc revolved about its own chord)
status: dispatched
opened: 2026-11-04
priority: P0
cost: H
parent: horn-and-spindle-tori-have-no-representation
branch: spiricarm/spindle-apex
---


Stage 2 of `horn-and-spindle-tori-have-no-representation`, the design Ev ruled on in PR 6464. Stage 1, `the-spindle-sheet-away-from-its-apexes` (PRs 6473 and 6486), builds a band of a sheet clear of its apexes. Today an arc that ends on the axis on a sheet's circle refuses with `RevolveError::SheetApex`.

## What stage 2 makes true

**The football builds through the public revolve door.** The football is a shallow arc closed by its chord on the axis and revolved about that chord: a lemon whose two apexes are vertices. The apple builds as well, with two dimple apexes. The horn (H1, Ev) is the sheet's closed end at `v* = π`, a single cusp apex. A face that reaches the horn's cusp from one side is a needle tip. A face that reaches it from both sides pinches, and the existing `NonManifoldAxisContact` refuses it.

The apex is a true conical point, like a cone's apex and unlike a sphere's regular pole. The kernel's precedents:
- the cone's apex handling (`Singularity::Apex`, the revolve's wire tips that become apexes);
- the sphere's pole form (`geom::pole_form`, the two half-turn meridians, `revolve::full::build_wire`).

Wire in each of these:
- `RevolveError::SheetApex` retires, apart from any case the design still refuses.
- `chart_singularity` / `Chart::poles` list the sheet's apexes; piece 1 already gives their positions, now through one home in `geom::sheet_apex`.
- Tier 3: the apex is a vertex in the pole form. `SheetApexReached` keeps its role for an apex that is neither a vertex nor shown clear.
- The mesh's pole fans or pole columns at an apex.
- Pcurves and the walk at an apex vertex.
- Props, if any closed form changes at an apex.

## Oracles (containment, never narrowness)

- **Volume and area** of the football, the apple and a needle-tip horn, each against an exact closed form of revolution that is independent of the code.
- **Watertight tessellation** (`admesh`).
- **Tier 3** at every ε row.
- **A wrong-apex row:** a sheet whose apex is swapped with the other sheet's must go red.

## Rules this unit carries (from the stage 1 reviews)

- **Every new certificate or guard ships with a row that goes red when its load-bearing check is removed:** a mutant row or a seeded fuzz. Both stage 1 duals found a soundness argument written in a doc that no test executed.
- **A census of kind tests sweeps every family the new code joins**, not only the variant being extended. Here that means "has a pole", "has an apex", and `Sphere | Cone` groupings.

## Also on this row's ground

- The tour stop, `the-conical-ended-football-has-no-tour-stop` (P3), is natural once the football builds. Take it if it is cheap.
- The wide apple, `a-wide-apples-tube-angle-passes-the-latitude-windows-cut`: an apple near its apexes is wide whenever `R > r·cos 1`. Fix it if stage 2's apple needs it.

Review tier: **dual**. This is the apex machinery for a new surface kind.
