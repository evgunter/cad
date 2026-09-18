---
id: comb
kind: program
title: COMB — the roll-ups and the sweeps that go last
status: open
opened: 2026-09-11
area: infra
prefix: comb/
tag: (COMB orchestrator)
ab_band: 4300-4399
paths: []
keep_out: [this program claims NO paths by construction - every row here is workspace-wide and collides with every fence, which is the reason all five L rows were written as not-takeable-while-a-track-is-open in the first place, no row here may be dispatched onto files another program has live units on - the ordering rule is decide-before-you-delete and delete-before-you-polish and this program is the polish, the roll-ups (S35 S11 S19 S43) are PARTITIONED not landed - a member that belongs to a live program is filed there as its own row and struck here, never fixed from this program, L4 and L5 audit other programs' closed records and produce findings and rides-along files - they never edit another program's live item, docs/DESIGN.md and memories/* are Ev's - a sweep that wants a standing sentence relocated proposes it on an [ev] PR]
---

Code-quality's *Last, deliberately* group given a program of its own,
plus the four roll-ups no single track can hold. `L1`–`L5`, `S35`'s 62
live sub-rows, `S11`'s dead machinery, `S19`'s ~260 refusal sites,
`S43`'s remaining idiom, the two comment sweeps and the milestone-naming
pass. Nothing here is `E`, eleven of thirteen rows are `H`, and **none
of it is takeable while a track is open on the files it would touch** —
which is what the group has always said about itself. Charter and unit
order: `work/comb/plan.md`.
