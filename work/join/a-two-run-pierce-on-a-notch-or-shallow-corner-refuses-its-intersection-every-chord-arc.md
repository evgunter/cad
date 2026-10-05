---
id: a-two-run-pierce-on-a-notch-or-shallow-corner-refuses-its-intersection-every-chord-arc
kind: issue
title: A two-run pierce on a 307° notch or a 200° shallow corner refuses its intersection 'every chord arc separates a loose scaffolding pair'
status: closed
opened: 2026-10-04
priority: P0
cost: H
refs: [a-pierce-whose-wide-run-pinches-its-intersection-refuses]
closed: 2026-10-05
---


## What

Found by PR 4026's review r1 (m3), on `crates/sweep/examples/r1_pierce_probes.rs`
(branch `join/pierce-two-out-runs-review-r1`). With two Out runs, the
intersection of the prism with a cube refuses
`JoinDesync { "every chord arc separates a loose scaffolding pair" }`:

- on `notch307`, a V-notch corner of 307°, in 17 of 35 two-run poses;
- on `shallow200`, a reflex corner of about 200°, in 7 of 15.

None of the filed pierce rows names this message.
`a-pierce-whose-wide-run-pinches-its-intersection-refuses` is the L
corner's ∩ residue under another message ("derived ring role order").
Whether the two are one cause is the first thing to measure.

## The shape to give

Run `r1_pierce_probes cube notch307` and `… shallow200`. Take one pose
of each, and read where the join's chord arcs separate the loose pair:
the ring's struts, or the piercing side's In face passing `v` twice.

## Built

PR 4038 (2026-10-05). The measured ∩ start rule is gone, and the ring struts face by the walk in every op. That is right once `zip::cross_pinches` crosses the pinch the rule had been dodging. Both dual reviewers built every two-run face ∩ SOUND: r2 350/350 over 8 corners, r1 340 per order on the original five corners plus its new ones, notch307 and shallow200 included. The rule's mutant turns 2 rows red.
