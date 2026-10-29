---
id: a-surface-pairs-march-is-repeated-per-operation-and-per-germ
kind: issue
title: A surface pair's certified march is run again for every germ mint, operation and operand order: the torus rows march one pair about eight times
status: open
opened: 2026-10-29
priority: P1
cost: M
---


Filed by SECT's orchestrator, 2026-10-29, from PR 6011's review and its
cost row (`work/sectrung/torus-germ-rows-march-for-an-hour-at-a-fine-band.md`).

## Measured

PR 6011 made the germ mint carry each pair the section table holds, so a
germ minted on a general torus pose marches the pair (`PairSection::carry`,
`implicit_pair_r3`). The section table is built per boolean operation, so
nothing shares a march across operations: a row running ∪, ∩ and ∖ in both
operand orders marches one pair about eight times, and within one operation
the mint and the join may each carry. At opt-level 2 and ε 1e-12 the
tilted-rod spins row takes 1015–1770 s, drum × donut 400–700 s and two
donuts 520–900 s, inside the nightly `full-suite` job's 180-minute limit
(cancelled once on 10-29, by another row).

## What a fix has to supply

One march per surface pair and band where its answer is the same: measure
how many marches each op and each row runs today, share within an operation
first (mint, join, certificate), then across operations and operand orders
if the answer is a function of the pair and band alone. Where the sharing
crosses the public API (a cache the caller holds, or one the kernel keeps),
that is a design choice to put to the designers, not to make in the build.

## Measured (2026-10-29, `sect/march-once`)

Marches counted by `geom_brep::implicit_pair_marches` (a test-only
count of `ssi::implicit_pair_r3` calls, the one march every door and
`PairSection::carry` reach), timed at opt-level 2 on a shared 4-core
box; "march" and "seat" are the time inside `PairSection::carry`'s march
and `PairSection::seat`, read by a probe that did not land.

| row | ops | marches | ε 1e-9: op / march / seat | ε 1e-12: op / march / seat |
|---|---|---|---|---|
| tilted rod, spins (`other_spins_…`) | 40 | 40 | 195 s / 31 s / 9 s | 1 170 s / 156 s / 49 s |
| drum × donut (`a_drum_…`) | 12 | 12 | 80 s / 6 s / 3 s | 460 s / 30 s / 15 s |
| two donuts (`two_donuts_…`) | 6 | 6 | 109 s / 10 s / 4 s | 609 s / 30 s / 19 s |
| ball × donut (`a_ball_…`) | 2 | 2 | 26 s | not run |
| radial hole (4 reports, 4 ∖) | 8 | 4 | 26 s / 3 s / 1 s | 170 s / 15 s / 4 s |

**Within one operation a pair is marched once already**: every op of
every row marched exactly once (each row has one marched pair), the
germ mint, the join's seats and the certificate all reading the section
table's one carry (`SectionTable::carried`, `ops::pair_verdict`); the
radial hole's `no_crossings_section_report` marches nothing (its square
wall is counted). So there was nothing to share inside an op; the rows
now pin it (`germ_torus_components` `row`: one march per run;
`germ_tilted_rod` spins: five per pose).

**The march is not the rows' cost**: 3–16 % of each row at either ε.
A gdb sample of drum placement 0's ∪ at 1e-12 (43 s, 32 samples, no
sample in the march): the pcurve mint after the build
(`pcurves::mint_pcurves` → `pcurve_cache::certify_projected`) 11, the
result gate's geometric certificate (`validate::validate_geometric_certificate`)
7, the join's chord edges (`chord_join::join` → `euler::mef` →
`certify_edge_spec` → `edge_nurbs::analytic_rung3`,
`pcurve_cache::limb2_offset_sup`) 10; inclusive, `twin_span_hulls` 16
and `ssi::certify::certify_branch` (limb 3's tube) 10. The cost is the
certificates of the fitted chord edges, read per op, at the mint, the
pcurves and the gate. Recorded on SECTRUNG's cost row.

## What remains: sharing across operations and operand orders

A design choice, not made here (the orchestrator puts it to the
designers):

- **Is the carried section a function of the pair and band alone?** In
  one operand order, yes: `PairSection::carry` marches
  `PairPose::domain()` (a ball the surfaces alone decide) at the band,
  with no face or body read, so two ops in one order march the same
  bits. Across orders, no: `A ∪ B` and `B ∪ A` list the same loops
  (drum 1, two donuts 1, rod spin 0: as many components, each
  component's start read on exactly one of the other's, the windings
  swapped with the operands) but no carrier is bit-identical, and the
  torus × torus loops start elsewhere on the loop. Sharing across
  orders needs the section carried in a canonical operand order first
  (`work/sectrung/a-surface-pairs-section-is-carried-in-operand-order.md`),
  which `[intrinsic.contingent]` asks of the bits anyway.
- **Where the shared section would live** touches the public API and
  `[pure-model]` (no hidden state; caching is "free" because models are
  values): a section value the caller holds and passes to each op, a
  memo the kernel keeps keyed by the pair's surfaces and the band, or
  the document's evaluation sharing it across the ops it evaluates.
- **What is at stake** is small: with every march shared across ops and
  orders the rows march 8, 6, 3 and 4 times instead of 40, 12, 6 and 4,
  saving at most 125 s of 1 170 s, 15 s of 460 s, 15 s of 609 s and
  nothing at 1e-12 (seats shared too would add 39 s, 10 s and 13 s).
  The cost the nightly feels is the chord edges' certificates above.
