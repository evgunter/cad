---
id: the-crescents-second-order-gap-is-spelled-three-times
kind: issue
title: The second-order gap Δκ·l²/2 between two sheets tangent along a line is spelled three times (sectors runs_into, sectors crescent, slit walls_inside)
status: open
opened: 2026-10-31
priority: P2
cost: E
refs: [5793, 5852]
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
