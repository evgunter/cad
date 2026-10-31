---
id: a-tube-whose-graph-margin-straddles-zero-refuses-as-a-decided-zero
kind: issue
title: limb 3 reads a graph margin whose enclosure holds zero (a lower bound of 0, not proved) as a decided Zero verdict, so an unproved tube refuses TubeStraddles definitely instead of escalating
status: open
opened: 2026-10-28
priority: P1
cost: M
refs: [ssi-limb-three-takes-the-widest-one-arc-rungs-band-verdict, a-two-chart-tube-cannot-prove-one-arc-along-a-chart-boundary]
---

Found by PR 5907's r2 review (its N7) and filed by SPLICE's
rim-exact-image lane. It predates that PR, which does not touch it.

## What

`ssi::certify::limb_three`, the `(false, _)` arm of the shortfall
match, sits after the ladder: no rung was taken, and the narrowest
rung's chain was not a graph. That arm hands the narrowest rung's margin
to `tube_transversality`. That margin is `zero_free_lower_bound` of
the graph margin's enclosure (`ssi::enclose`), and it is exactly `0.0`
whenever the enclosure holds zero. A lower bound of zero says only
"not proved". `tube_transversality` levers it and passes it to
`band_verdict`, which classifies `0.0` as a DECIDED `Zero`, the
refused arm. The lane then refuses `TubeStraddles { verdict:
Zero(Classified { margin: 0.0 }) }`, a definite verdict that the
crossing is a sliver.

An enclosure that holds zero cannot tell a genuine sliver from slack
in the enclosure. So this is an undecided reading reported as decided,
the fail-loud class. `PlaneNurbsRefusal::TubeStraddles`' own doc says
the margin is `0.0` on every straddling refusal and should be read as
"the bound the certificate could prove". The verdict arm still claims
a decision.

## Witness

demo-tour lily wall 8 at default ε, on PR 5907's head:
`Certification { error: PlaneNurbs(TubeStraddles { verdict:
Zero(Classified { margin: MarginDiag(Value(0.0, ..)), band: [1e-9, 1e-8)
}), boxes: 16 }) }`. That wall 8 was the leaf's sheath grafted on its blade; since 2026-10-29 the scene builds the leaf as one loft, and the sweep row `crates/sweep/tests/a_sheath_grafted_on_its_blade.rs` builds the same graft.

## What is open

Whether a zero lower bound should escalate (`Undecided`, its cause
the enclosure) rather than refuse, and which consumers read
`TubeStraddles` as a definite sliver. The analytic lane's
`AnalyticRung3Refusal::TubeStraddles` shares `tube_transversality`, so
the same reading reaches it.

