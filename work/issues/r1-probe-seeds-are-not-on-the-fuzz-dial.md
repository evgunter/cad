---
id: r1-probe-seeds-are-not-on-the-fuzz-dial
kind: issue
title: the R1 probe rows seed from the clock under a private R1_SEED that CAD_FUZZ_SEED does not pin
status: open
opened: 2026-09-03
---


**Ratified ground** (`work/tcost/plan.md` §Ratified ground, citing
`memories/test-suite-cost.md`): *a fuzzer that is not gated is a defect
in the fuzzer*. These rows are randomized and are on no dial.

Two rows draw their seed from the clock, under a PRIVATE variable that
the tree's fuzz harness does not know about:

- `crates/editor-core/tests/r1_dual_probes.rs:424`-`:433`
  (`r1_no_value_only_key_collision_search`): `R1_SEED`, else
  `SystemTime::now()`;
- `crates/editor-core/tests/r1_m10_1_probes.rs:39`-`:47` (`Rng::from_env`,
  the file's whole PRNG): `R1_SEED`, else `SystemTime::now()`.

A third site of the same shape is `crates/viewer/tests/review_gui0_r1.rs:96`
(`GUI0_R1_SEED`).

`test_utils::fuzz` is the tree's harness for exactly this — one RNG,
one per-run seed, one EFFORT dial, and `CAD_FUZZ_SEED` to pin a run.
`CAD_FUZZ_SEED` does **not** reach `R1_SEED`, so a run that pins the
fuzz dial still draws these rows fresh.

**What is NOT the evidence for this, said out loud.** TCOST-K3's
digest instrument saw run-to-run differences in its workspace-wide
roster and its unit's PR named these rows as the likely cause. That
diagnosis was then MEASURED and is false: the differences are torn
lines in the instrument's own shared append log (one file, one
`writeln!` per record from every nextest process at once, and
`writeln!` on an unbuffered `File` is not one atomic write) — 0.5-2 %
of records per run arrive glued to a neighbour's tail, and a body
measured exactly once then vanishes from a `sort -u` roster. Two
same-tree runs here differed in WHICH lines tore and still produced
byte-identical rosters. So no measurement in this repo currently
attributes anything to these two rows.

This issue therefore stands on the RULE and not on a cost:
`work/tcost/plan.md` §Ratified ground says a fuzzer that is not gated
is a defect in the fuzzer, and these rows are randomized, ungated, and
unreachable from the one variable that is supposed to pin every
randomized row. What they cost has not been measured, and this issue
does not claim it has.

**The fix** is the harness, not a fixed seed: route both rows through
`test_utils::fuzz` so they draw their seed and their EFFORT from the
same place as every other randomized sweep, and are gated with them.
The keep-out that applies (`work/tcost/plan.md` §Keep-outs) is that no
fixed seed is introduced and no row loses detection power — routing to
the harness keeps both.
