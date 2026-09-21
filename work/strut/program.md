---
id: strut
kind: program
title: STRUT — one sweep rule with several homes: the strut rule, the chain model and the walks beside them
status: ready
opened: 2026-09-20
area: kernel
prefix: strut/
tag: (STRUT orchestrator)
ab_band: 7900-7999
paths: [crates/sweep/src/test_support.rs, crates/sweep/src/extrude.rs, crates/sweep/src/swept.rs]
keep_out: [opened by CARVE's 2026-09-20 priority-seam cut (Ev, in chat) per work/README.md Track size - CARVE measured 73 budget points and was cut into three tracks meant to run in PARALLEL (Ev, in chat: for these high priority tracks it is ideal to have several components that can be worked on in parallel), the siblings are CARVE BAND STRUT and they share crates/sweep by design - shared ground is legitimate by the README's 2026-09-20 rule and what is owed is awareness while a lane is LIVE, so run scripts/work.py territory on your branch and announce the seam in the PR rather than drawing a fence, TANG owns the declared-tangency lane and ISO the rational carriers, topo/src is TOPO's and its cut siblings']
priority: P1
---
The entrenching class on the sweep crate's ground: **one rule, several
homes.** `extrude`'s strut rule is spelled in three places and three
functions carry the same fixed context-argument list; `sweep/src`
spells the half-edge-to-face walk five times where `topo` now exports
one; `battery.rs` holds the chain data model, the walk AND a recourse
audit beside the predicates; `test_support` has become four modules in
one file with six `f64`/`_at` twin pairs.

Two rows are the special-case shape rather than the duplication one:
`attach_contact` picks seam/transverse/smooth from the carrier KIND
where the dihedral is what decides, and the homed seed finder reads
`center.y`, so every z-poled fixture rolls its own.

Charter and order: `work/strut/plan.md`; narrative in `work/strut/log.md`.
