---
id: the-crescents-second-order-gap-is-spelled-three-times
kind: issue
title: The second-order gap Δκ·l²/2 between two sheets tangent along a line is spelled three times (sectors runs_into, sectors crescent, slit walls_inside)
status: closed
opened: 2026-10-31
priority: P2
cost: E
refs: [5793, 5852]
closed: 2026-11-01
pr: 6217
---

Found by the re-review of PR 5793 (CLEAVE).

Where two sheets are tangent along a line, the second-order gap between
them at arc length `l` off the line is `(κ_b − κ_a)·n̂·l²/2`. It is
written out three times:

- `crates/topo/src/boolean/sectors.rs` `runs_into` (`sectors.rs:2005`,
  `entered`);
- `crates/topo/src/boolean/sectors.rs` `crescent` (`sectors.rs:2248`,
  `opened`);
- `crates/topo/src/boolean/slit.rs` `walls_inside` (the tangent arm,
  `apart`, with `bend_across` supplying the bends).

Three spellings of one reading can be decided differently in the band.
A route is one function of the two bend vectors, the normal and the
lever, which the three call. `sectors.rs` is under review in PR 5852
(the crescent), so this waits for it to land.

## Slate triage

2026-11-01: moved from `work/cleave/` by CLEAVE's split on its semantic seams (Ev, in chat: CLEAVE measured 114 budget points against 30). TANGRIM holds the tangent curved rims and crescents; id and body unchanged. (CLEAVE orchestrator)

## Closed

2026-11-01, PR 6217: `geom_brep::parting` is the one spelling of `(κ₁ − κ₂)·û·l²/2`; `runs_into`, `crescent`, `slit::walls_inside`, `fate::corner_holds` (a fourth this row did not count), `tangent_relative_side`, `tied_germ_turn` and `enters_material_order2` call it.
