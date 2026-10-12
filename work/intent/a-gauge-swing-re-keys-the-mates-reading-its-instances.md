---
id: a-gauge-swing-re-keys-the-mates-reading-its-instances
kind: issue
title: A gauge swing, a value edit, re-keys the mates reading the instances on the gauge
status: parked
priority: P3
cost: E
blocked_on: [a-placement-is-the-bundle-of-mates]
opened: 2026-10-10
---

Found by INTENT stage 2 F (`a-mate-reads-face-variables`, PR 4566), and kept as a named interim at the orchestrator's ruling.

## What

A mate's content key is its reads' upstream keys (spec §7, "Content key"). Its sides read faces of instances, and an instance's key carries its whole gauge chain. So a value edit to a gauge — the tour's `swing`, a `DocEdit::SetVarValue` on the turntable's Angle parameter — re-keys every mate reading an instance on that gauge, though the mate's answer does not move. `demos/tour/src/assembly.rs` (the swing loop's `moved` set) now lists `mate_1`, `mate_2` and `crate_mate` among the nodes a swing re-keys.

That is a key moving under an edit of a value, against the rule that keys survive value edits (N2's spirit; D9: bits are a function of the reads). The memo re-runs the mates and answers the same, so nothing wrong is served; the cost is recomputation.

## Why parked

Stage 3 B retires gauges into placements (`a-placement-is-the-bundle-of-mates`). Then a mate reads bodies whose keys carry no pose, and the swing re-keys only the placement. When it lands, re-check the tour's `moved` set: the three mates should drop out of it. If they do not, this row is the defect.
