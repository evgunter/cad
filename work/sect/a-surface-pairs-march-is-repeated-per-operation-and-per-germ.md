---
id: a-surface-pairs-march-is-repeated-per-operation-and-per-germ
kind: issue
title: A surface pair's certified march is run again for every germ mint, operation and operand order: the torus rows march one pair about eight times
status: dispatched
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
