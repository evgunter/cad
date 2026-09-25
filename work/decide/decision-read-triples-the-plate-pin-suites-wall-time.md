---
id: decision-read-triples-the-plate-pin-suites-wall-time
kind: issue
title: the decision read's deep enclosure runs at every select and min/max node, and the plate's pin suite goes 142s to 535s in the dev profile
status: dispatched
opened: 2026-09-21
priority: P2
cost: D
---


## What was measured (DECIDE-3, 2026-09-21)

`SymRules::decision_read` asks `signed::decision` at EVERY `Select`
node and `signed::order` at every `min`/`max` node the early walk
reaches, and each call encloses both halves of the decision form in
the outward-rounded ring, descending through `sqrt`/`abs`/`min`/`max`
atoms to `ENCLOSE_DEPTH = 8`. Where the read SUCCEEDS it pays for
itself; where it DECLINES — the common case, because most decisions
carry an indeterminate no bracket is known for — the enclosure has
already walked the sub-tree before it can say so.

Measured on this box (dev profile, 4 cores, `--test-threads=2`):

| suite | base (`e88987915`) | DECIDE-3 | ratio |
| --- | --- | --- | --- |
| `m10_10_pins_interval` | 142.97 s | 534.93 s | 3.7x |
| `m10_8_pins_interval` | 106.39 s | 144.82 s | 1.36x |
| `m10_9_pins_interval` | 134.27 s | ~81 s (fewer refusals to report) | 0.6x |

On the ceiling-bisection instrument the per-probe costs are unmoved to
the digit at the plain tier and A0 alone (plate 0.18 s / 0.24 s,
bracket 0.61 s), because the read is early-walk only; the shipped
tier's bracket probe is where it shows.

## What would answer it

The cheap tests first, in this order, before any enclosure is built:

1. **The enclosability pre-pass rule C already has.** `signed::fold`
   refuses in one pass over the ids when the form carries an
   indeterminate that is neither a parameter nor π. The decision read
   needs the same test widened by one level — "a parameter, π, or an
   atom whose own arguments pass this test" — run as a cheap
   id-walk before `enclose_deep` allocates a single interval.
2. **Memoize the enclosure per form digest** inside the session. A
   frame's conditioning floor is asked at every component of every
   vector built on it, and the argument forms repeat exactly.
3. **A depth-1 attempt first**, widening only where it straddles.

None of these changes what the read DECIDES, so the pins hold across
them, which is what makes this a follow-up and not a blocker.

## Home

`crates/geom-core/src/sym/signed.rs` (`enclose_indet`, `enclose_deep`,
`decision`), `crates/geom-core/src/sym.rs`'s `combine`. Filed by
DECIDE-3's lane with the measurement above.


## SYM-9 read this row and left it where it is (2026-09-22, re-taken 2026-09-24)

SYM-9's A1 amendment made this row ride as far as that unit's cost
table: if the decision read dominated the retry ladder's cost on any
document, Phase 2 was to take this row's first cheap answer (the
enclosability pre-pass) and re-baseline the suites' wall times.

It does not, and that is now executed rather than argued. On the
affordability line's instrument (one whole-box leaf, release, fastest
of three takes), the shipped rules against `SymRules::without_the_reads`,
each with `SymRetry::kept_atom` and without it:

| document | read on: one attempt / ladder | read shut: one attempt / ladder |
| --- | --- | --- |
| R2's filleted bracket (`1e1·ε`) | 2.86 / 3.88 s | 2.82 / 3.85 s |
| R2's link (`1e1·ε`) | 17.28 / 19.71 s | 17.57 / 19.81 s |
| R2's rounded pad (`1e2·ε`) | 131.3 / 147.7 s | 132.8 / 148.6 s |

On this instrument and these three documents the read costs nothing
measurable, with or
without a ladder. This row's own measurement is a different one — the
pin SUITES' wall time in dev — and nothing here contradicts it; what it
says is that the leaf a drive pays is not where the read's cost lands on
these three documents. The row stays open at P2 with its three cheap
answers in the order it names them.

## Where the read's cost is (DECIDE-6)

**The read is not where the suite's time is.** DECIDE-6 instrumented
the read (`geom_core::sym::profile::ReadProfile`, under
`sym-profile-testing`) and ran it on the plate's pin-suite documents
and the link. On every replay the read is under half a percent of the
wall time, and its enclosure is less than that. The 143 s → 535 s this
row measured came with DECIDE-3, which also added rule G, and rule G is
where the time went: `work/decide/rule-g-is-the-link-and-pads-leaf-cost`.

**Where the read's cost is.** The dev table comes from
`decide_6_read_cost_interval::decide_6_where_the_reads_cost_is` at
ε = 1e-9, the shipped set, no ladder, one profiled replay each. Leaves
are at the certifying end of each pinned bracket. "Asked" counts the
`signed::decision` calls. Every one came through `min`/`max`
(`signed::order`), and no document asked the read at a `Select`. The
pre-pass, memo and depth columns are §What each answer would save.

| replay | wall | asked | settled | declined | read time | enclosing | pre-pass-provable declines (their time) | id-walk, every call | digest repeats (their time) |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| plate, nominal | 2.78 s | 36 | 36 | 0 | 0.62 ms | 0.24 ms | 0 | 0.05 ms | 30 (0.49 ms) |
| plate, leaf at 0.2630 | 2.81 s | 36 | 36 | 0 | 0.63 ms | 0.22 ms | 0 | 0.06 ms | 30 (0.52 ms) |
| annulus, leaf at 0.8416 | 2.96 s | 36 | 36 | 0 | 0.62 ms | 0.25 ms | 0 | 0.05 ms | 30 (0.52 ms) |
| link, leaf at 4.930e2·ε | 37.98 s | 175 | 32 | 143 | 2.75 ms | 0.71 ms | 143 (2.30 ms) | 0.36 ms | 104 (1.51 ms) |
| bracket, leaf at 3.870e2·ε | 13.79 s | 188 | 89 | 99 | 58.85 ms | 44.48 ms | 99 (12.26 ms) | 12.09 ms | 117 (40.20 ms) |

The pad is measured on the release leaf instrument only
(`m10_10_evidence_interval::m10_10_leaf_cost_with_and_without_the_algebra`
with `CAD_M10_10_PROFILE`, 1e2·ε). It asks 440 reads, settles 114 and
declines 326, and all 326 are pre-pass-provable. The read takes 2.48 ms
of a 75.3 s leaf, 0.53 ms of it enclosing.

**Declines by cause** (the enclosure's first failure, in its own order):
- **Link:** all 143 are an indeterminate with no bracket.
  - `tan`: 12 at the top level, 71 below it.
  - `cos`: 54, all at depth 2 or deeper.
  - 6 exhausted the depth cap.
- **Bracket:** all 99 are an opaque id or the depth cap.
  - Opaque id: 1 at the top level, 66 below it.
  - 32 exhausted the depth cap.
- **Pad:** all 326 are unbracketed or the depth cap.
  - `sin`: 156.
  - Opaque id: 110.
  - `cos`: 6.
  - 54 exhausted the depth cap.

No decline anywhere is a straddle or a poisoned enclosure.

**The suite, directly.** The table accounts for the read's time per replay. Two
executions account for it at the suite's scale:
- **Read shut against read on** (the same row, unprofiled, shipped against
  `SymRules::without_the_reads`): 2.52 / 2.35 s on the plate nominal,
  2.45 / 2.63 s on the annulus leaf, 34.72 / 35.01 s on the link leaf and
  12.34 / 12.59 s on the bracket leaf. Shutting the read moves no
  replay beyond run-to-run noise.
- **The read run ten times at every call** (a local patch, not
  committed): `decision` asserted that nine more calls of the read
  returned the same answer. `m10_10_pins_interval` took 340.75 s against
  337.83 s unpatched (dev, `--test-threads 2`). Ten times the read's
  whole cost moves the suite by less than its noise.

## What each answer would save (DECIDE-6)

- **The pre-pass.** It could answer every decline on these documents
  (99 on the bracket, 143 on the link, 326 on the pad) and none on
  the plate or the annulus, which decline nothing. The most it saves
  is the time of those calls: 12.26 ms on the bracket's dev leaf and
  2.30 ms on the link's. Its own id-walk, run beside the read on every
  call, costs 12.09 ms and 0.36 ms. On the bracket it would cost about
  what it saves. It cannot take `m10_10_pins_interval` (337.83 s at
  this head) back within 1.2× of 143 s, because the read-×10 patch
  shows the read's whole cost is inside the suite's noise.
- **The digest memo.** It would hit 117 of 188 calls on the bracket,
  104 of 175 on the link, 30 of 36 on the plate and the annulus, and
  269 of 440 on the pad. Those calls take 40.20 ms of a 13.79 s dev
  leaf on the bracket (0.3 %), and at most 1.51 ms elsewhere.
- **Depth-1 first.** It settles no read on the plate or the annulus,
  where every read needs depth 2. It settles 14 of the link's 32
  settled reads, 18 of the bracket's 89 and 18 of the pad's 114. On
  this enclosure a depth-1 attempt that succeeds does exactly the work
  of the depth-8 one: the cap only refuses, it never coarsens. So it
  saves nothing where it succeeds, and it adds a failed attempt
  wherever the read needs depth 2 or more.

DECIDE-6 therefore took none of the three. The instrument and its
evidence row stay, so the table can be re-taken.
