---
id: ssi-limb-three-takes-the-widest-one-arc-rungs-band-verdict
kind: issue
title: ssi: limb 3 takes the widest one-arc rung's band verdict without descending to a narrower rung
status: open
opened: 2026-10-06
priority: P2
---


(Filed by the transversality-lever lane, 2026-10-06; found by lever A
in the design fork on
`work/ssi/ssi-transversality-at-a-point-is-spelled-three-ways.md`.)

## What

`limb_three` (`crates/geom-brep/src/ssi/certify.rs` ~1118) walks the
ladder widest first and returns at the first rung whose chain is a
graph and is proved one arc, deciding that rung's margin
(`tube_transversality`) and returning its verdict, in band or not. It
never tries a narrower rung after an in-band verdict, though a
narrower rung's enclosure is tighter and can clear where the wider
one's slack did not.

README C2 says the refusal is for an enclosure "that does not clear
the band at any rung" (`crates/geom-brep/README.md`, C2, "Refusal is
typed, never a retry loop").

## Witness

The near-degenerate hyperbola (`m5_pr7_ssi::a_hyperbola_along_its_asymptote_pairs_its_branches_right`,
its `c = +1e-4` near-degenerate chart, the case
`m5_pr7_ssi::the_near_degenerate_hyperbola_answers_at_a_coarse_eps` runs) at ε 1e-6, with the tube levered by the wall's
curvature radius (2.2 mm) instead of the extent: the widest one-arc
rung (7.8 mm) has a clearance of 8.8e-5, enclosure slack (the true
sine on the locus is about 0.4), and it refuses `TubeStraddles`; the
next rung clears (lever A, round 1). Under the extent lever this row
answers, so the defect is latent on main.

## Repair shape

Descend past an in-band rung as past a straddling one, and refuse with
the narrowest rung's verdict.
