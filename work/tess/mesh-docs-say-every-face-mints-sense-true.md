---
id: mesh-docs-say-every-face-mints-sense-true
kind: issue
title: mesh's pole-band sense read is documented as the identity because 'every face this build mints has sense: true', which no longer holds
status: open
opened: 2026-09-29
priority: P3
cost: E
---

## Finding

`crates/mesh/src/lib.rs` (module docs, ~:131) and
`crates/mesh/src/walk.rs` (module docs, ~:128) say the crate's one
sense read, the pole-to-pole band's azimuth disambiguation, is the
identity today because "every face this build mints has `sense:
true`". That has not held since M5 S11: extrude states `false` on a
concave arc wall, and revolve on its inward walls, including a
concave sphere band (`crates/sweep/tests/m5_s11_concave_sense.rs`).
Since PR 3467 every construction states or derives the bit through
`FaceSurface` (D1), and `false` is common.

**What would close it.** Say what the read does on a `false` face and
which row reaches it, or show no `false` face reaches the pole-band
arm; then drop the "bitwise inert today" claim either way.

Found by PR 3467's fix pass (TOPO), sweeping for present-tense prose
that states a default `sense`.
