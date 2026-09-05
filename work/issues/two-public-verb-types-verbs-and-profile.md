---
id: two-public-verb-types-verbs-and-profile
kind: issue
title: Two public types named Verb in one workspace (verbs::Verb, profile::Verb) — the collision is known at one site and reconciled nowhere
status: open
opened: 2026-09-05
refs: [SEAT-7, 1910, 1547]
---


(SEAT orchestrator) Class note from SEAT-7's dual review (PR 1910),
filed so it has a home; unowned — it straddles SEAT (`crates/verbs`)
and the profile layer.

`verbs::Verb<T>` (the kernel verb vocabulary, SEAT-4) and
`profile::Verb` (the sketch program's verb) are both public. SEAT-7's
`verbs/src/verb.rs` cites `profile::Verb::ALL` as the PRECEDENT for its
own census in the same file — the collision is known there — and the
same file goes to some length to distinguish `sweep::blend::BlendKind`
while saying nothing about this harder one. Readers of `profile::Verb`
outside the profile crate: `viewer/src/pane/create.rs` and
`switch_slots.rs`. No behavior is at stake; the cost is every future
reader's `use` line and every doc sentence that says "the verb" without
a crate. A rename on one side (the sketch program's is the older and
the more local — `profile::Op`/`SketchVerb`-shaped) or a stated
convention in both crates' module docs would settle it; the choice is
a naming ruling, small enough to ride whichever unit next opens
`profile`'s public surface.
