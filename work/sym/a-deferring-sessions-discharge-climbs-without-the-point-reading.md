---
id: a-deferring-sessions-discharge-climbs-without-the-point-reading
kind: issue
title: A deferring session's discharge climbs the whole ladder with no point reading, so a margin an eager session refutes is climbed there and a registration false at the point is proven
status: open
opened: 2026-11-28
---


Found by the refute-first lane's sweep (PR 7712; BOXFINISH).

`Sym::decide_sign` (`crates/geom-core/src/sym.rs`) reads an undecided
margin at one point of the box before it asks the tier
(`crates/geom-core/src/sym/point.rs`, `refutes`): a reading that
excludes 0 proves the margin no identity, and the tier is asked
nothing. A deferring session (`sym::deferred::with_deferred_session`)
returns from `decide_sign` before that reading: `deferred::defer`
keeps an in-band `Zero`'s node, and `deferred::discharge` later climbs
it through `discharge_checked` with no reading at all
(`sym/deferred.rs`, `discharge`). The shipped caller is
`editor_core::coincide::replay` (`discharge`, `discharge_or_why`).

Two consequences, neither measured:

- **Cost.** A discharge climbs every rung and retry on a margin an
  eager session would have refuted for one arithmetic pass. Only the
  margins a reader names are discharged, and they are in-band numeric
  `Zero`s, whose reading at a point is near 0 too, so the refutable
  share is expected small; no corpus counts it.
- **One margin, two answers.** A registration false at the point (a
  registrant stating an identity that does not hold over the box) is
  refuted in an eager session, so the door is never asked and the
  decision is the numeric one, while a deferring session discharges
  the same margin through the door and proves it `Zero`. The eager
  answer is the right one (the margin is not zero at a point of the
  box); the deferring one is the false axiom believed. Shown in an
  eager session by `geom-core`'s
  `sym_root_rows::the_doors_zero_beside_the_read_is_proven`, whose
  registration had to be made true for that row to keep proving.

What would close it: the same reading ahead of the climb in
`deferred::discharge` (the node and the leaf's brackets are both at
hand there), or a statement in the deferred module of why a
discharge believes the door where a decision does not.
(boxfinish/refute-first)
