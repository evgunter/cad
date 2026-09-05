---
id: ladder-rim-phase-may-retire-a-new-split-key
kind: issue
title: blend: the ladder rim phase can push a fresh split key as a retirement
status: open
opened: 2026-09-05
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

UNMEASURED. No shipped ladder fixture reaches the orientation (every
revolve-minted meridian's `he_plus` starts at the rim vertex, so `upper`
is the source key on every row that runs). The ruled band's `split_rim`
in the same file guards the same shape with `if near == source` and a
fragment-provenance read, which is the fix shape; the ladder path was
deliberately NOT changed in FILLET-H7 (PR 1897) because every existing
carve's dump is held bit-identical there and the orientation has no
witness. The fix owes a fixture that reaches it (a meridian whose stored
direction runs from the pole to the rim) before the branch is touched.
