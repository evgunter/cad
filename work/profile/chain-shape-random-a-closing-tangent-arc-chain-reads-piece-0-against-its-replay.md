---
id: chain-shape-random-a-closing-tangent-arc-chain-reads-piece-0-against-its-replay
kind: issue
title: chain_shape_random: seed 0xcf882d40c459f94b finds chain_shape and replay disagreeing on whether piece 0 continues, on a chain closed by a tangent arc and LineTo(StartArriving)
status: open
opened: 2026-11-23
priority: P1
cost: M
---


Found by GREEN's U17 on probe run 45310731023 (`test (kernel)`, default ε), at `origin/main` `82273ca89b` plus a diff that touches no profile code. `profile` `chain_shape_random::a_chains_shape_walks_what_its_replay_walks` (`crates/profile/tests/chain_shape_random.rs`, the panic near :583) is a counterexample search. This seed fails it:

```
[fuzz] chain-shape-random: seed=0xcf882d40c459f94b effort=1
the shape says piece 0 continues the one before it (false), against the replay's construction: [false, false, false]:
[At(Point2 { x: -4.0, y: -2.0 }), Toward { dx: -1.0, dy: 0.5 }, Line(0.8736741749754569), Angle(0.7853981633974483), TangentArcTo(Point(Point2 { x: 2.0, y: -5.0 })), LineTo(StartArriving)]
shape Ok(ChainShape { pieces: [Piece { step: 2, role: Leg }, Piece { step: 4, role: Leg }, Piece { step: 5, role: Leg }], continues: [false, false, false], entered: [Entry, PlainPoint, DirectedPlain, DirectedPoint, DirectedIncoming, DirectedPoint] })
replay Ok(())
reproduce with CAD_FUZZ_SEED=0xcf882d40c459f94b CAD_FUZZ_EFFORT=1
```

PR 7670's own gate drew another seed and passed. So this is a defect the search found, not a red every run meets. The closing `LineTo(StartArriving)` arrives along the first leg's direction, so a reading where the closing piece and piece 0 continue is plausible, and one of `chain_shape` and the replay disagrees with it. Which is right was not judged. The last changes near here are DOCTAIL's (`5c28add626`, "a circle's stations and a lone arc's tangent closer continue"). Owed: replay the seed, decide which side is wrong, fix it, and pin the chain as a deterministic row beside the fix.
