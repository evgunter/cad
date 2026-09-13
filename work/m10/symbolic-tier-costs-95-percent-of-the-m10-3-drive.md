---
id: symbolic-tier-costs-95-percent-of-the-m10-3-drive
kind: issue
title: The E12 symbolic tier is 95% of the M10-3 interval drive: 20.8x measured, and nothing has profiled inside the normal form
status: open
opened: 2026-09-11
---


Filed 2026-09-11 by S-TCOST, on Ev's direction in chat, out of the
diagnosis of `work/tcost/m10-3-chamber-row-reads-ten-times-its-recorded-cost`.
**Nothing here says the tier is wrong or should be off.** It says the
tier is expensive, nobody has measured where inside it the time goes, and
the first work is that measurement.

## The measurement

One box (4 vCPU / 15 GB, shared), `test` profile (opt-0), same command
each time — `cargo nextest run -p editor-core --features interval
--test all -E 'test(/m10_3_r1_probes_interval/)'`. LOCAL readings, an
iteration tool and not a result of record
(`memories/perf-measurement-lane.md`); a unit would need a hosted pair.

| configuration | suite wall | note |
|---|--:|---|
| `SymbolicDials::default()` with `enabled: false` | **15.364 s** | all 9 rows still pass |
| shipped (`enabled: true`, degree 128) | **319.373 s** | `main` at `486557f5` |
| shipped, degree forced back to 16 | 467.731 s | *slower* — see below |
| the same suite before the tier (`a4439fbef`, 2026-09-03) | 21.041 s | pre-M10-7 |

**The tier is 304 of 319 seconds — 95.2% of the suite, a 20.8x
multiplier.** And the rest of the kernel did not regress: with the tier
off the suite is *faster today* (15.4 s) than the whole suite was before
the tier existed (21.0 s). The entire delta is E12.

Hosted, applying the 3.8x local:hosted ratio measured on this same row:
~84 s with the tier (which matches the nightly's 83.3 s reading) against
~4 s without.

**The degree dial is not a lever and the design doc already said so.**
Forcing `DEFAULT_SYM_MAX_DEGREE` back to 16 makes it *slower*, exactly as
`drive.rs`'s own note predicts — *"the endpoint identity the tier's
headline row depends on needs degree >= 32 to cancel; at 16 it freezes
and the row does not move."* A frozen form sends the work back to
subdivision. The dial is correctly set; this row is not about the dials.

## What is known about the mechanism, from the design's own words

`drive.rs`'s `SymbolicDials` note is unusually explicit and is the best
starting evidence:

> the form the tier actually builds is not the written one. It is a
> QUOTIENT of polynomials reached by repeated cross-multiplication, and
> a metered extrusion contributes `‖w‖` and its reciprocal to every term
> it touches, so the degree that matters is the degree AFTER those
> denominators have been carried up the DAG.

And `sym.rs`'s own header records that the coefficient ring moved off
`i128` to arbitrary precision — `Rat` over `num-bigint`, bounded at
`COEFF_BITS` — *"They were an in-tree `i128` through M10-7"* — with a
`Small(i128)`-inline / `BigInt`-on-overflow hybrid already in place
because *"measured, a `BigInt` for every one of them"* was worse.

So the plausible cost centres, in no order and **none of them measured**:

1. **Degree growth from carried denominators.** Cross-multiplication up
   the DAG is the mechanism the doc names; a form that reaches degree 32+
   in several symbols has a term count that grows fast, and every
   normalisation walks it.
2. **The coefficient ring.** `Rat` over `num-bigint` past the `i128`
   inline path — how often the promotion actually fires on this fixture
   is a counter nobody has read.
3. **Term storage and allocation.** `BTreeMap`/`HashMap` per form, rebuilt
   per normalisation.

## What this row asks for, in order

1. **Profile inside `geom_core::sym` on the M10-3 slab.** Which of the
   three above dominates, with numbers. Everything else is guesswork
   until this exists — including any patch written from reading the code,
   which is exactly how the degree-16 hypothesis above got refuted.
2. **Read the freeze and promotion counters that already exist.** The
   design says the evidence for the budget is *"the FROZEN COUNT on the
   verdict"*; whatever that reports on this fixture is free evidence and
   nobody has looked.
3. **Only then, a change** — and it is a kernel unit under S-TCOST's
   charter clause for exactly this shape (*where a test is slow because
   the CODE it exercises is slow in a way the real program pays too, the
   fix is a kernel change*), so it runs under whatever review posture
   M10 sets, with a hosted before/after.

## One observation, recorded without an argument attached

**All nine rows pass with `enabled: false`.** That is a fact, not a
case for turning the tier off: `drive.rs` says the tier is on because
*"a certifier that can only certify boxes narrower than its own ε is the
state M10-3 pinned and E12 exists to leave"*, and these rows may simply
not be the ones that exercise what it buys. But if the M10-3 suite is
green either way, then on today's tree **nothing in it would red if the
tier regressed to its pre-E12 answers** — which is a coverage question
for M10 and is why it is written down here rather than left in a
terminal.

## Territory

Filed on M10's slate: M10 designed the tier (M10-7, PR #1725), owns
`crates/editor-core/src/drive.rs` where `SymbolicDials` and both budget
constants live, and owns the M10-3 rows that pay the cost.
**`crates/geom-core/src/sym.rs` and `sym/` are PROPS's** by its
`crates/geom-core/src/*` glob, so a fix that lands inside the normal form
is an announced cross-fence change to PROPS rather than M10's to take
silently. Named here so the first lane does not discover it at
`work.py territory`.

## Provenance

The regression that surfaced this is
`work/tcost/m10-3-chamber-row-reads-ten-times-its-recorded-cost`: the
M10-3 interval suite went 21.04 s -> 319.37 s between 2026-09-03 and
2026-09-11, bisected to PR #1725, and the gate that was supposed to
watch it skipped the suite on that very PR (the marker names
editor-core paths; #1725 changed `geom-core`). The cost was never
recorded anywhere — `work/m10/` records the tier's API costs and the
`COEFF_BITS` trade, and no runtime figure at all.
