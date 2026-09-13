---
id: ladder-rim-phase-may-retire-a-new-split-key
kind: unit
title: blend: the ladder rim phase can push a fresh split key as a retirement
status: dispatched
opened: 2026-09-05
branch: blend/8-ladder-split-key
---

## Finding

In `crates/sweep/src/blend/surgery.rs`, `rim_phase` step (2) splits each
rim vertex's meridian and names the piece still touching the rim vertex
the UPPER remnant:

```rust
let upper = if touches_v(body, m) { m } else { created.new_edge };
rec.meridian_splits.push((created.vertex, m));
let lower = if upper == m { created.new_edge } else { m };
rec.meridian_remnants.push((lower, m));
remnants.push((v, upper, m));
```

and step (6) retires the upper remnant with `rec.dead.edges.push(mr)`
where `mr` is that `upper`. When `split_edge` hands the source key `m` to
the LOWER piece (the parent keeps the half whose `he_plus` starts at the
far end), `upper == created.new_edge`, and a FRESH key is pushed to
`dead.edges`. `Retired` is documented as source keys; the totality walk's
direction (b) — "every retirement names a SOURCE key" —
(`test_support::assert_naming_totality`) would fail on it, and the
document layer's `emit_blend` would build a retired-set entry that no
row can ever match.

## Status

MEASURED, and the branch is LIVE. The census over every ladder carve the
tree runs (25 carves across 25 rows, instrumented at the split site)
finds both orientations shipped: a REVOLVE-minted cap seam runs
pole-to-rim, so `upper == created.new_edge` and the phase pushes a fresh
key, while a BOOLEAN-minted pip seam runs rim-to-pole and pushes the
source key. Fourteen shipped rows reach the fresh-key push — among them
`ring_clearance_forms::the_bosss_dome_rim_carves_inside_its_hosts_circular_boundary`
and `fillet_h4_concave_rim::the_boss_carves_a_concave_ladder_band_and_adds_the_cap_fill`
— and none of them runs `assert_naming_totality`, which is why no row
went red. The item's earlier reading ("no shipped ladder fixture reaches
the orientation") was the unmeasured guess it announced itself to be.

The ruled band's `split_rim` in the same crate guards the same shape with
`if near == source` and a fragment-provenance read, which is the fix
shape; the ladder path was deliberately NOT changed in FILLET-H7 (PR
1897) because every existing carve's dump is held bit-identical there.
The dump IS bit-identical under the fix — the records the fix moves are
naming rows, which no geometry reads.

## Re-homed (2026-09-06)

Moved from `work/issues/` to `work/blend/` in the tracker-wide cut of 2026-09-06 (Ev's direction, in-chat), which read every open `work/issues/` file and every open code-quality row against every live program's `paths` and opened four programs for the ground none covered. Id, body and header are unchanged except as noted; the directory is the claim (`work/README.md`). `blend/surgery.rs`'s ladder rim phase; owes a fixture before the branch is touched.
