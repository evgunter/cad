---
id: compensated-blocks-check-shifts-past-i128
kind: issue
title: ssi::compensated's the_blocks_enclose_the_exact_results panics shifting an i128 past its width in its own division check
status: open
opened: 2026-10-14
priority: P2
cost: E
---

Seen by the FUSE lane on stage (c) of `work/fuse/the-curve-pair-reader.md`, whose diff does not touch `ssi`. It failed once in a local `ci`-profile run of geom-brep, at the default effort.

`ssi::compensated::tests::the_blocks_enclose_the_exact_results` (`crates/geom-brep/src/ssi/compensated.rs`) panics with "attempt to shift left with overflow" in its division check's `ai << -e`. That check compares an enclosure end `m·2^e` with `a/b` in `i128`. Where the end's exponent is far enough below zero, `-e` passes the width that `a` can be shifted by. The panic is in the test's own check, not in `Dd`.

- **Replay:** `CAD_FUZZ_SEED=0x355c599eef34b8c1 CAD_FUZZ_EFFORT=200` (also `0xed3e1fe4b86b0ea4`, `0x2a7987130ebb37fe`, `0x6ffa51f1f1c0e7c7`, `0x46ab3c68487139c8`, `0xa087c637813062a6`). About 1 seed in 4 fails at effort 200.
- **Why it matters:** the fuzzer runs in the default suite with a fresh seed, so it turns any PR gate touching geom-brep red at random, and the nightly too.
- **A fix:** compare in a wider integer or in exact rationals, or bound `e` first (a quotient end that small sits within resolution of zero, which a sign test against `a/b` can read directly). Pin the replay seed as a deterministic row beside the fix (discipline §8).
