---
id: tangent-joints-are-derived
kind: issue
title: D10 stage 4 PR G: ProfileLoop.tangent_joints is no longer stored; the set is derived at lowering, a value-decided junction is recorded, UndeclaredTangency retires
status: open
opened: 2026-10-08
priority: P0
cost: M
---

INTENT stage 4, PR G. Spec: `docs/INTENT-STAGE4-SPEC.md` §8.

`ProfileLoop.tangent_joints` (`crates/profile/src/lib.rs:474`) goes; the set is derived at lowering from the constructors and the junction verdicts (D1, already ratified), a junction decided Zero that no constructor made is recorded at the door, `UndeclaredTangency` retires and `TangencyContradicted` stays for constructor-made joints. Profile digests are bit-equal. Releases 2 rows (spec §12).
