---
id: movefac-row-skips-three-component-shells
kind: issue
title: the movefac catalog row offers only two-component shells, so a three-component shell is never partitioned
status: open
opened: 2026-09-06
track: P
refs: [S69, 2014]
---

## What

`crates/topo/src/seqgen.rs`'s `movefac_candidates` offers only the
shells whose incidence complex has fallen into **exactly two**
connected components. `movefac` on a `c`-component shell mints
`c − 1` shells, and a catalog row's Euler vector is a per-variant
constant (`OpChoice::ep_vector` takes `&self` and no body), so
offering `c > 2` would need the count carried in the choice — the
denormalization `OpChoice::SplitEdge` deliberately avoids for its
split parameter.

Measured on the corpus while S69 was being written: over 3072 steps of
the standard walk, 4131 shell observations held 3251 one-component,
760 two-component, 111 three-component and 9 four-component shells. So
the excluded class is real, at roughly 3% of shell observations before
the row existed — less afterwards, since the row fires on the
two-component shells a third component would have grown from.

The coverage this costs is stated in-file rather than hidden: **a
shell that reaches three components is never partitioned by this
walk**, and neither is the `c − 1 > 1` arm of `movefac`'s own minting
loop (`crates/topo/src/movefac.rs:148-171`) — which is exercised only
by `movefac.rs`'s own deterministic rows, never by the randomised
lane.

## What closing it looks like

Two shapes, and the choice between them is the interesting part:

- Carry the component count in the variant
  (`OpChoice::Movefac(ShellKey, i64)`) and let `ep_vector` read it.
  Cheapest; costs the "a site is an ADDRESS, not a derived fact"
  property the rest of the catalog holds.
- Give `ep_vector` the body it would need to derive the count, which
  changes its signature for every row.

Either way the row's candidate filter drops to `>= 2` and the
`shell_components` walk it already does is reused.
