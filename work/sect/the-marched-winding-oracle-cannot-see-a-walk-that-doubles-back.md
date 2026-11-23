---
id: the-marched-winding-oracle-cannot-see-a-walk-that-doubles-back
kind: issue
title: The marched-winding oracle cannot see m = 0, the degree guard over-refuses, winds re-spells Box3's meet, and FittedCarrier::winding holds m times the component's turns
status: open
opened: 2026-10-31
priority: P3
cost: M
---


Filed by SECT's `sect-onesix` lane (PR 6153), from its confirmation review.

## Measured

- **The oracle is blind to `m = 0`.**
  `marched_windings::marched_windings_agree_with_a_projected_oracle`
  (`crates/geom-brep/tests/marched_windings.rs`) Newton-projects the
  carrier's own samples onto the section. So it follows the walk the
  carrier makes, and cannot see a walk that doubles back. An
  independent oracle would trace the section itself, for instance by
  contouring the partner's implicit on the torus chart. It also skips
  most poses (counted sections, refusals and drifted projections all
  count as one "skipped"), and it has no minimum `compared`.
- **The degree guard over-refuses.** `winds::winding` asks for the walk
  once round (`once_round`, `gated`) wherever any channel reads an
  unsettled zero. But a nonzero turn on another channel already proves
  `m ≠ 0`, and then the zero is the component's.
- **Two spellings.** `winds::meet` and `winds::hull` re-implement what
  `Box3::intersection` and a definitely-disjoint test already give in
  `ssi::enclose`.
- **A public field holds the carrier's turns.**
  `FittedCarrier::winding` (`crates/geom-brep/src/pair_section.rs`) is
  a `pub` field that holds `m` times the component's turns. It is the
  component's only where the walk is once round. Only its essentiality
  is read downstream.

## What closing it takes

- Build a tracing oracle, count skips by reason, and floor `compared`.
- Skip the guard where some channel's turns are nonzero.
- Use `Box3`'s intersection and disjointness in `winds`.
- Either store essentiality only, or name the field for what it holds.
