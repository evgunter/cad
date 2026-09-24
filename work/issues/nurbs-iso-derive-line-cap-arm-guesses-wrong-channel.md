---
id: nurbs-iso-derive-line-cap-arm-guesses-wrong-channel
kind: issue
title: nurbs_iso_derive's LINE cap-rim arm mints a v-row image for a u-column ruling
status: open
opened: 2026-09-04
priority: P0
cost: H
---


TRIM territory (no live TRIM orchestrator until CURVED's exit; filed
in `work/issues/` per the cross-program rule).

`topo/src/pcurves.rs`'s LINE cap-rim arm (at :683 as of EXCH-H1's
merge base) mints iso images for line rims by picking the fixed
channel with `side_pick`. Handed a straight U-COLUMN ruling seam that
reached it as a `PlacedSegment`/`Chart(plane)` description, the arm
minted a V-ROW image — the wrong channel — and the loop walk failed
`LoopDiscontinuity { half_edge: HalfEdgeKey(20v1) }` instead of a
typed wrong-channel refusal. Measured on the EXCH-H1 lane while the
wall-column candidate was stripped (the seven red 1e-6 rows of run
33843873270, e.g. `recognize_pins::the_mixed_arc_prism_imports_first_
class_over_the_intersection_pcurve_arm`); at the unit's head the
candidate exists and the path is UNREACHABLE in every measured
configuration — the mis-mint is latent behind a candidate gap, not
live. The defect shape: the arm's channel guess is not validated
against the ruling's actual iso direction before the walk consumes
the image; a `decide`-doored channel check (or a typed refusal when
the mapped endpoints disagree with the guessed channel) closes it.

Found by the EXCH-H1 lane (PR #1798); routed here by its fix pass.
