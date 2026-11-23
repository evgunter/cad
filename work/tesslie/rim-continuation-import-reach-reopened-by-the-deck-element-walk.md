---
id: rim-continuation-import-reach-reopened-by-the-deck-element-walk
kind: issue
title: the pcurve re-mint no longer refuses a rim row stated a band apart, so RimContinuation's import-door reach is open again and unmeasured
status: closed
opened: 2026-10-03
closed: 2026-11-21
priority: P2
cost: M
---


Found by the PCERT unit that retired the loop's angle-equality
decisions (`pcurve-loop-decisions-state-a-3d-identity-plus-a-branch-margin`).

`CoherenceCondition::RimContinuation` (`crates/topo/src/coherence.rs`)
was documented as dead through the import door: `import_step`'s pcurve
re-mint decided each junction's chart-v jump (`pcurve_loop_continuity`)
at the band the condition reports at, and refused every body the
condition would report on. That decision is gone. The walk
(`topo::pcurves::lift_joint`) now decides only each joint's deck
element; the joint's 3-D coincidence follows from the two rows'
envelopes and the edge certificate's endpoint pinning, a bound of
`4ε` rather than a decision at `ε`.

Measured on `topo/tests/mesh12_rim_row_reach.rs`' two-level rim cap
(the Euler doors): `mint_pcurves` now admits `R·Δv = 1.5ε` and `1.9ε`,
where the examination reports (`the_remint_admits_the_gaps_the_examination_reports`).
So an imported rim row stated as two arcs a band apart can now carry a
rim-continuation finding to a consumer that gives the whole side one
value. The same holds for `MeridianContinuation` by the same argument.

To do: re-measure the import door on the shapes the old note named
(a STEP solid, a two-cap sphere, issue 723's half-cap at 1e-6, 1e-9,
1e-12), and decide whether the consumer that discards the coordinate
needs the condition's verdict or can read the `4ε` joint bound.

## Moved to TESSWALK (what the walk and the lanes decide: structural premises, and readings taken off undecided numbers) (2026-10-28)

Cut out of TESS by its semantic-seam split (`work/README.md`, Track size) when TESS measured 146 budget points against 30. Moved by `git mv`; the id and body are unchanged.

Moved from TESSWALK to TESSLIE on 2026-11-20, by `git mv`, when TESSWALK merged into TESSLIE: TESSWALK held no P0 or P1 row, so no sitting would dispatch it on its own (`work/README.md`, Track size), and its charter is one TESSLIE carries.

## Closed

(2026-11-21, `tesslie/retire-coherence`.) Its subject is retired: the chart-coherence
examination and its `RimContinuation`/`MeridianContinuation` conditions
are removed (Ev's ruling on PR 7224), so there is no import-door reach
left to measure. What the rows it cites still pin about the doors, the
endpoint pin's escalation past the band and the re-mint admitting every
gap the certifying doors build, is kept in
`topo/tests/mesh12_rim_row_reach.rs` as
`the_certifying_doors_build_a_two_level_rim_row_inside_the_endpoint_band`
and `the_remint_admits_every_gap_the_certifying_doors_build`.
