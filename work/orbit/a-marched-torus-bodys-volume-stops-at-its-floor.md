---
id: a-marched-torus-bodys-volume-stops-at-its-floor
kind: issue
title: A boolean body whose torus face a marched plane × torus edge trims can measure its volume only to a bracket (the resolution stops at its floor)
status: open
opened: 2026-11-24
priority: P2
cost: M
refs: [a-torus-near-a-tilted-cut-stops-at-the-extent-scan]
---


Found by `orbit/tilted-torus-rim` (PR 7617).

`crates/sweep/tests/germ_torus_rods.rs`
`random_rods_through_the_tube_never_answer_a_wrong_body` at
`CAD_FUZZ_SEED=0x8187e91f9499a08f CAD_FUZZ_EFFORT=1`: a donut × rod op
builds a body whose torus face a marched (spline) plane × torus edge
trims, and `topo::mass_properties` at `pinned_resolution` comes back
unmet:

```
the measurement did not meet its resolution: it stopped at `floor`, so
its enclosure is a bracket and not a number met
```

The fuzz counts such a body (`Body, volume unmet`) and holds it by any
volume identity whose terms were met; one whose `p` is also unread
(`work/inside/point-in-solid-refuses-a-torus-face-bounded-by-a-spline-chord.md`)
is held by tier 3 alone, under the fuzz's 90% floor.

Settles when the volume lane meets its resolution on a torus face a
marched edge trims (or the row names the floor's reason and pins the
bracket against the identities).
