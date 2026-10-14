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

`ssi::compensated::tests::the_blocks_enclose_the_exact_results` (`crates/geom-brep/src/ssi/compensated.rs`) panics with "attempt to shift left with overflow" in its division check's `ai << -e`.

- **Cause:** a draw with `a == 0`. The quotient `x / y` then encloses zero with denormal ends (about `±7.4e-323`), `split` gives their exponent `e ≈ −1074 + …`, around −148 or lower, and `ai << -e` panics once `−e ≥ 128`, though `ai` is zero. The panic is in the test's own check, not in `Dd`.
- **Rate:** about 1e-3 per default run, and about 0.17 per seed at effort 200 (`a` is drawn from 2²¹ integers, so a zero turns up about once in 2·10⁶ draws).
- **Replay:** `CAD_FUZZ_SEED=0x355c599eef34b8c1 CAD_FUZZ_EFFORT=200` (also `0xed3e1fe4b86b0ea4`, `0x2a7987130ebb37fe`, `0x6ffa51f1f1c0e7c7`, `0x46ab3c68487139c8`, `0xa087c637813062a6`).
- **Not one line.** With the shift guarded (`if ai == 0 { 0 } …`), the same draws fail the next assertion: the width check calls `[−7.4e-323, 7.4e-323]` wide for `0/−355163`, since its floor `8·u·|want| + RESOLUTION` is not read for a zero quotient. So both checks need a zero arm, or the draw must exclude `a = 0` and a deterministic row must pin that case.
- **Why it matters:** the fuzzer runs in the default suite with a fresh seed, so it turns a gate touching geom-brep red at random, and the nightly too. Pin the replay seed as a deterministic row beside the fix (discipline §8).
