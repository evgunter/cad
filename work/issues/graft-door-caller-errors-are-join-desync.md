---
id: graft-door-caller-errors-are-join-desync
kind: issue
title: the graft doors refuse a caller's wrong source as JoinDesync, which offer_key reads as a kernel defect
status: open
opened: 2026-10-08
priority: P3
cost: M
---


## What

`BooleanError::JoinDesync` is the join's invariant break: the two
solids' surgery diverged, or a key the join read stopped resolving.
The public graft doors raise it for a caller's wrong argument, which is
neither:

- `topo::instance::graft_disjoint` (about `:104`): `"graft source does
  not hold exactly one solid"`;
- `topo::instance::graft_disjoint_all_keyed` (about `:261`), and so
  `graft_disjoint_all`: `"graft source holds no solid to graft"`.

The mislabel has a consumer: `topo::lib`'s `offer_key` (about `:556`)
marks every `BooleanErrorKind::JoinDesync` a defect, so a caller who
passes an empty body is told the kernel is broken. The Display text
says so too ("A/B lockstep invariant violated: … (kernel bug or
corrupt reduction)").

## The shape to give

A typed caller-error variant (the source's solid count, against what
the door takes), with its `BooleanErrorKind`, its pncad-py tag
(`crates/pncad-py/src/tags.rs`, and the tag table in
`crates/pncad-py/src/tests.rs`) and its offer key. The rows that pin
these refusals today pin them by `what` (`instance.rs`'s
`a_source_that_is_not_a_single_solid_refuses_typed`,
`crates/topo/tests/graft_disjoint.rs`'s
`the_n_solid_door_refuses_an_empty_source_and_the_single_door_still_refuses_n`),
so a variant turns them red and they move with it.

Found by the JOIN lane closing `rows-pin-join-desync-without-its-what`,
which pinned these rows by `what` rather than adding the variant (a
public-API addition reaching the bindings, outside that row's fence).
