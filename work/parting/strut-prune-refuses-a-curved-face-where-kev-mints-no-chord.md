---
id: strut-prune-refuses-a-curved-face-where-kev-mints-no-chord
kind: issue
title: plan_strut_prune refuses every strut on a curved face, though a strut with one free end is deleted by kev, which mints no chord
status: open
opened: 2026-10-27
priority: P3
cost: E
refs: [a-ring-partner-inside-the-ring-unions-result-invalid]
---

Found by PR 5812's narrow re-review (`cleave/flush-partners-at-a-slit-build`).

## What

`Body::plan_strut_prune` (`crates/topo/src/strut.rs`) refuses
`StrutRefusal::Curved` for every strut on a face that is not a plane.
The Boolean's closing reports this as `CurvedFaceClosesAcrossSlit`. The
reason the refusal gives holds only for a strut with both ends free:
deleting the lone ring left behind mints a `mekr` bridge, which is a
line chord and lies in no curved face. A strut with one free end is
deleted by `kev` alone, which mints nothing, so on a curved face it is
refused for no reason. PR 5812 dropped the same unreasoned gate from
`slit::part_mid_slit`, whose `kemr` also mints no chord.

It is unreachable today, as that gate was: PR 5812's re-review
instrumented the sweep suite and saw no curved closing, and its curved
collar witness refuses typed before the closing.

## Owed

Refuse `Curved` only where the lone ring's bridge is minted, with a
witness that reaches a one-free-end strut on a curved face. Otherwise
state the reason the refusal covers the `kev` case.


## Slate triage

2026-11-01: moved from `work/cleave/` by CLEAVE's split on its semantic seams (Ev, in chat: CLEAVE measured 114 budget points against 30). PARTING holds the parting's reading of edges lying in faces and the cuts on spline faces; id and body unchanged. (CLEAVE orchestrator)
