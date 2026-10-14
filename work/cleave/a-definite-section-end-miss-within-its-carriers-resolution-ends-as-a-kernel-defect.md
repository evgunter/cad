---
id: a-definite-section-end-miss-within-its-carriers-resolution-ends-as-a-kernel-defect
kind: issue
title: A section chord's definite end miss within its carrier's own resolution still ends as a kernel defect, where Ev's PR 4812 interim reads a kernel limitation
status: open
opened: 2026-10-14
priority: P2
cost: M
---


## What

Found by PR 4810's census (`sweep/examples/section_end_census_probe.rs`,
family `axial`, ε = 1e-12, on main `eb6f9c0f80`). PR 4810 reads an
*escalated* attachment pin on a section chord against its carrier's own
resolution (`geom::Curve3::resolution`, admitted by
`vertex_section::resolved`): past `ε/K` it ends as
`SplitJoinError::SectionUnresolved`, D4 ¶1 (i)'s last resort. A
*definite* pin miss stays the gate's, as its row's design says, so it
still ends "There is no way through: this is a kernel defect; report
it".

On the unit cylinder (`0 ≤ z ≤ 1`) cut by planes `θ` off its axis, 119
runs refuse `Join(Euler(Certification { ResidualExceeded {
EndpointStart | EndpointEnd } }))`:

| θ | runs | margin |
|---|---|---|
| 1e-6 | 45 | 5.8e-11 … 3.5e-10 |
| 3e-6 | 41 | 1.5e-11 … 8.7e-11 |
| 1e-5 | 33 | 1.1e-11 … 2.9e-11 |

Each is within the carrier's bound (about `64·u·r/θ`: 7e-9 at 1e-6,
7e-10 at 1e-5), at 4–5% of it. The escalated neighbours, 83 runs at
θ = 1e-5 … 3e-4, measure 0.2–6.7% of theirs. These are the same
far-centre rounding, and Ev's interim on PR 4812 reads "a miss within a
carrier's own resolution ends as a kernel limitation … not 'kernel
defect'".

## Owed

Decide whether a definite pin miss on a carrier whose resolution
exceeds the miss ends in the last resort too, in
`chord_join::SectionEnds::refusal` (the one home). The anchored conic
(`the-section-table-admits-a-curve-on-its-resolution`) removes the
far-centre term and should empty this population; re-measure there.
