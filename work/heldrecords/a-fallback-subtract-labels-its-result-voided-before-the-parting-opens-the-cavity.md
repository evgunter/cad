---
id: a-fallback-subtract-labels-its-result-voided-before-the-parting-opens-the-cavity
kind: issue
title: The containment fallback's subtract fixes BooleanResultKind::Voided before the parting re-sorts its shells, so a parted result with no void is labelled Voided
status: open
opened: 2026-11-25
priority: P3
cost: M
refs: [a-tube-touching-a-ball-from-inside-along-its-rim-refuses-the-extent-scan]
---

Filed by INSIDE's orchestrator from the two-designer weighing of
`work/inside/a-tube-touching-a-ball-from-inside-along-its-rim-refuses-the-extent-scan.md`
(2026-11-25); both designers raised it (likely; read, not run).

`boolean/ops.rs`'s containment fallback fixes its result kind before
the output stage (`BooleanOp::Subtract => BooleanResultKind::Voided`,
in `fallback`), and only then does `finish_output`'s parting
(`slit::part_riders`) re-sort the shells. Where the parting opens the
cavity, the result has no void: the keel-cavity ring
(`a_cavity_resting_its_keel_on_the_floor_opens_through_it_as_a_ring`) is
one shell, and `ball ∖ tube` with its rims on the sphere would be two or
three solids. Both are still labelled `Voided`, whose doc says the result
holds legitimate voids. The kind should be derived from the finished
result, not stored ahead of the step that changes it. Neighbour of
`boolean-result-kind-assembly-names-a-type-the-kernel-does-not-have`.
