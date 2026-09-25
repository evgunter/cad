---
id: decision-read-triples-the-plate-pin-suites-wall-time
kind: issue
title: the decision read's deep enclosure runs at every select and min/max node, and the plate's pin suite goes 142s to 535s in the dev profile
status: closed
opened: 2026-09-21
priority: P2
cost: D
closed: 2026-09-25
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

**What the measurement found, and why this row closes.** The read is
not where the suite's time is. DECIDE-6 instrumented it
(`geom_core::sym::profile::ReadProfile`, under `sym-profile-testing`)
on the plate's pin-suite documents: the plate, the annulus, the link,
the bracket and the pad. On every replay the read, `order`'s `a − b`
included, is under half a percent of the wall time. Shutting the read
moves no replay beyond noise, and running it ten times at every call
does not move `m10_10_pins_interval`. The 143 s → 535 s this row
measured came with DECIDE-3, which added rule G, A0's `min`/`max` folds
and the read together. Measured separately, rule G carries the time
and the other two do not: `work/decide/rule-g-is-the-link-and-pads-leaf-cost`.
The title above is the premise as filed; this section is what the
measurement says of it.

**The baselines have moved, and the 143 s was not re-taken.** At
DECIDE-6's head (`019d14841` base, dev, `--test-threads 2`, this box)
the suites are `m10_10_pins_interval` 341.59 s (535 s at DECIDE-3),
`m10_9_pins_interval` 326.35 s (~81 s at DECIDE-3) and
`m10_8_pins_interval` 108.89 s. Rows have been added since, and the
pre-DECIDE-3 tier is not a rule set of today's tree, so no run here
reproduces the 143 s.

**The table.** The rows come from
`decide_6_read_cost_interval::decide_6_where_the_reads_cost_is`: dev,
ε = 1e-9, the shipped set, no ladder, one profiled replay each, at
DECIDE-6's fix-pass head. Leaves are at the certifying end of each
pinned bracket.
- "Asked" counts `signed::decision`'s calls. Every one came through
  `min`/`max` (`signed::order`), and no document asked the read at a
  `Select`.
- "Read" is the read's own time, from entering `decision` to its
  answer; the instrument's work is outside it.
- "`a − b`" is `order` building the difference, before the read; the
  refusals there are budget or ring refusals.

| replay | wall | asked | settled | declined | read (enclosing) | `a − b` (refused) | pre-pass-provable declines: their read (enclosing) | digest repeats (their read) |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| plate, nominal | 3.05 s | 36 | 36 | 0 | 0.34 ms (0.23 ms) | 0.13 ms (13) | 0 | 30 (0.27 ms) |
| plate, leaf at 0.2630 | 2.94 s | 36 | 36 | 0 | 0.39 ms (0.26 ms) | 0.17 ms (13) | 0 | 30 (0.33 ms) |
| annulus, leaf at 0.8416 | 2.94 s | 36 | 36 | 0 | 0.38 ms (0.27 ms) | 0.10 ms (0) | 0 | 30 (0.31 ms) |
| link, leaf at `4.930e2·ε` | 38.12 s | 175 | 32 | 143 | 1.09 ms (0.67 ms) | 0.76 ms (8) | 143: 0.87 ms (0.53 ms) | 104 (0.62 ms) |
| bracket, leaf at `3.870e2·ε` | 13.91 s | 188 | 89 | 99 | 49.98 ms (49.28 ms) | 2.91 ms (12) | 99: 9.01 ms (8.76 ms) | 117 (33.05 ms) |
| pad, leaf at `2.4990e3·ε` | 293.15 s | 440 | 114 | 326 | 4.21 ms (2.60 ms) | 10.38 ms (0) | 326: 2.72 ms (1.52 ms) | 269 (2.64 ms) |

The pad's leaf returns in dev on this box: 277.34 s unprofiled, the
certifying end `m10_10_pins_interval`'s ceiling row replays. On the
release leaf instrument (`m10_10_leaf_cost_with_and_without_the_algebra`
with `CAD_M10_10_PROFILE`, at `1e2·ε`) the same counts come back — 440
asked, 114 settled, 326 declined.

**Declines by cause.** The cause is the enclosure's own refusal, noted
at the arm of `signed::enclose_indet` / `enclose_deep` that made it;
the cap's test comes first, as the enclosure makes it. `@k` is the
atom level the unbracketed id sits at (0: the top of a half).

| cause | link | bracket | pad |
| --- | --- | --- | --- |
| depth exhausted | 8 | 34 | 64 |
| `tan` @0 / @1 / @2 / @3 / @4 / @5 / @6 / @7 | 12 / 37 / 8 / 14 / 4 / 2 / 2 / 2 | — | — |
| `cos` @0 / @1 / @2 / @3 / @4 / @5 / @6 / @7 | 0 / 0 / 18 / 12 / 18 / 2 / 2 / 2 | — | 2 / 4 / 0 / 0 / 0 / 0 / 0 / 0 |
| `sin` @1 / @2 / @3 / @4 / @5 / @6 / @7 | — | — | 45 / 53 / 16 / 18 / 10 / 4 / 4 |
| opaque @0 / @1 / @2 / @3 / @4 / @5 / @6 / @7 | — | 1 / 36 / 16 / 0 / 4 / 4 / 4 / 0 | 0 / 50 / 22 / 16 / 4 / 6 / 4 / 4 |

No decline anywhere is a straddle or a poisoned bracket.

**The suite, directly.** Two executions account for the read at the
suite's scale:
- **Read shut against read on** (the same row, unprofiled, shipped
  against `SymRules::without_the_reads`, at DECIDE-6's first head):
  2.52 / 2.35 s on the plate nominal, 2.45 / 2.63 s on the annulus
  leaf, 34.72 / 35.01 s on the link leaf and 12.34 / 12.59 s on the
  bracket leaf. No replay moves beyond run-to-run noise.
- **The read run ten times at every call** (a local patch, not
  committed): `decision` asserted that nine more calls of the read
  returned the same answer. `m10_10_pins_interval` took 340.75 s
  against 337.83 s unpatched.

## What each answer would save (DECIDE-6)

- **The pre-pass.** It could answer every decline on these documents
  (143 on the link, 99 on the bracket, 326 on the pad) and none on the
  plate or the annulus, which decline nothing. It must strip the
  halves before it walks them, so the most it saves is the ENCLOSING
  time of those calls: 0.53 ms on the link's leaf, 8.76 ms on the
  bracket's, 1.52 ms on the pad's — before its own walk's cost, which
  was not measured because the walk was not built. The largest, the
  bracket's, is 0.06 % of that leaf. It cannot take
  `m10_10_pins_interval` back within 1.2× of 143 s: the read-×10 patch
  puts the read's whole cost inside the suite's noise.
- **The digest memo.** It would hit 117 of 188 calls on the bracket,
  104 of 175 on the link, 30 of 36 on the plate and the annulus, and
  269 of 440 on the pad — floors, because the digest carries the
  `gated` bit, which the read does not look at. Those calls take
  33.05 ms of a 13.91 s leaf on the bracket (0.24 %) and at most
  2.64 ms elsewhere.
- **Depth-1 first.** It settles no read on the plate or the annulus,
  where every read needs depth 2. It settles 14 of the link's 32
  settled reads, 18 of the bracket's 89 and 18 of the pad's 114. On
  this enclosure a depth-1 attempt that succeeds does exactly the work
  of the depth-8 one: the cap only refuses, it never coarsens. So it
  saves nothing where it succeeds, and it adds a failed attempt
  wherever the read needs depth 2 or more.

DECIDE-6 therefore took none of the three. The instrument and its
evidence row stay, so the table can be re-taken.

## Closed (2026-09-25, DECIDE-6, #3229)

The premise is falsified by measurement: the decision read is not where
the pin suites' time is (the two DECIDE-6 sections above). The time
DECIDE-3 added is rule G's, and it is carried by
`rule-g-is-the-link-and-pads-leaf-cost`.
