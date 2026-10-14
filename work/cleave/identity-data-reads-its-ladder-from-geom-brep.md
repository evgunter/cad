---
id: identity-data-reads-its-ladder-from-geom-brep
kind: issue
title: carrier_eq::identity_data carries its own copy of the identity-ladder rule geom_brep::ladder::read_identity now owns
status: open
opened: 2026-10-14
priority: P4
cost: E
---


## What

The identity ladder's rule is "any definite datum answers; an in-band
one is kept, and refuses only where no later datum decides". It now has
a home in geom-brep: `geom_brep::ladder::read_identity`
(`crates/geom-brep/src/ladder.rs`). That helper is generic over the
per-datum reader, owns the detached / splice / splice-superseded log
handling, and returns `Apart{at, sign}`, `Zero(Option<MarginDiag>)` or
`Undecided(Kept<E>)` (non-empty, in data order). `pcurve_cache`'s spiric and Villarceau ladders read
it (PCERT,
`work/pcert/an-in-band-gate-masks-a-definite-one-in-the-spiric-and-villarceau-ladders.md`).

`topo::boolean::carrier_eq::identity_data`
(`crates/topo/src/boolean/carrier_eq.rs:1098`, PR 4918) spells the same
rule out by hand, so there are two copies of one rule.

## Fix

Read `identity_data` through `read_identity`:

- the reader is `CoincidenceMeasure::decide`;
- `Apart` maps to `IdentityData::Distinct` and `Zero(m)` to
  `IdentityData::One(m)`;
- `Undecided(kept)` is ranked by `ray_walk::Evidence` (unreadable ahead
  of in band) as today, over `kept.into_vec()`.

One difference to check: the helper splices runs in data order. It
holds the Zero runs read after the first kept one beside the kept ones,
where `identity_data` splices each Zero at once and the kept ones at the
end. The verdict and escalation channels keep their orders either way,
so only the probe sample stream's interleaving can move. Re-baseline any
golden that reads it, and say what moved.

## Two more to retire with it (PR 5089's reviews)

- **Two cause rankers.** `ladder::Kept::refusing_cause` puts the first
  poisoned reading ahead of the first in-band one, and topo's
  `ray_walk::Evidence::ranked` ranks blocked ahead of in-band. They are
  the same ranking twice. The port should keep one: `identity_data`
  ranks its `Kept` through `Evidence`, or `Evidence` reads
  `refusing_cause`'s order.
- **`topo::boolean::rim_wedge::shared_rim`**
  (`crates/topo/src/boolean/rim_wedge.rs:138`) is a near-parallel of
  `read_identity` with the polarity reversed. It takes the first rim
  pair decided one, keeps undecided pairs with their logs held, and
  splices them superseded once a pair answers. It is the same rule and
  the same recording, so it can read the ladder too: `read_identity`
  extended to a pass predicate, as
  `work/pcert/gate-loops-return-on-an-in-band-gate-before-a-definite-one.md`
  asks, covers the reversed polarity.

Ownership: `carrier_eq.rs` is claimed by CLEAVE, HONE and TANG (`work.py
territory`). This stays on CLEAVE, whose slate holds
`rim-identity-decides-centre-radius-and-axis-one-at-a-time`.
