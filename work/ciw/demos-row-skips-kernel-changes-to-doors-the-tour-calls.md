---
id: demos-row-skips-kernel-changes-to-doors-the-tour-calls
kind: issue
title: The PR gate runs the demos row only when demos/ changes, so a kernel change to a door the tour calls reaches the tour only nightly
status: open
opened: 2026-10-02
priority: P3
cost: E
---


## What

`.github/workflows/ci.yml`'s change filter keys the demos row on
`flag demos "touched '^demos/'"` (the classify step, near line 112),
so the PR gate runs `demos/tour` and `demos/wild` only when the diff
touches `demos/`. A kernel change to a door the tour calls is gated
only by the nightly. Instance: PR 3773 rewrote `topo::query::rim_of`,
which `demos/tour/src/bodies.rs` calls (the bud's mouth rim), and
`RimError`'s arms; its gate skipped the demos row, and its reviewer
ran the tour suite by hand (86/86).

The implementer discipline already asks a lane changing a public
signature to run the two demos' clippy by hand; nothing asks for
their tests, and a behaviour change with no signature change gets no
demos run before merge at all.

## The shape to give

Seed the demos row from the crates the demos depend on (the way the
`topo` row is `seeded topo`), or at least from the crates whose public
doors the tour calls; or state in the discipline that a kernel door
change runs the tour suite before push. Weigh against the row's cost.
