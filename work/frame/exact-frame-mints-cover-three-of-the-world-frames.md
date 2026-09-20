---
id: exact-frame-mints-cover-three-of-the-world-frames
kind: issue
title: The frame witness's decision-free mints cover three world frames, so every other exact frame costs a K funnel name
status: open
opened: 2026-09-15
---


## What

Disclosed by FRAME-WITNESS (`docs/FRAME-WITNESS-SPEC.md`), which built
`geom_core::OrthoFrame` and its mints.

`OrthoFrame`'s decision-free mints are `axes_xy`, `axes_yz` and
`axes_zx` — the three CYCLIC world frames, which is what
`profile::SketchPlane`'s three sugar doors needed. Every other frame
whose axes are exact basis vectors has to go through
`OrthoFrame::gram_schmidt`, which is a DECISION and therefore wants a
band and a K funnel name the caller has to invent. On an exactly
orthonormal pair that mint changes no bit (pinned by
`gram_schmidt_keeps_an_exact_pair_bit_for_bit` in
`crates/geom-core/src/linalg/ortho_frame.rs`), so what the caller pays
is not arithmetic — it is a name in the K roster and a refusal path
that cannot fire.

The receipts are the two helpers the unit had to write:

- `demos/tour/src/scalar.rs` `sketch_frame` / `authored_frame`, under
  the invented site `tour_frame_axis` — the tour's `(x̂, ẑ)` planes
  (`klein.rs` ×3, `lily.rs` ×2) are exact and could not use a mint;
- `crates/sweep/src/test_support.rs` `sketch_from_axes`, under
  `fixture_frame_axis`, for the same reason across the fixtures.

`demos/tour` IS in the k-lint sweep's corpus (`scripts/k_probe_sweep.sh`'s
demo-scenes leg), so `tour_frame_axis` is a roster name a demo minted
because the library had no decision-free door for the frame it meant.

## The two shapes a fix could take

1. **Widen the exact mints** to the signed basis frames — an axis pair
   drawn from `±x̂, ±ŷ, ±ẑ`, refusing the parallel pairs at compile
   time by taking an axis-and-sign enum rather than a vector. Twenty-four
   frames, no decision, no name.
2. **Leave it, and say so on the type** — the doc currently says the
   three are "the only frames the type admits without a decision"
   without saying why the other twenty-one are not there.

Either wants PROPS' call, since the type is `geom-core/src/linalg`'s.

## Not the same as a "trust me" mint

Both shapes above stay inside the ruling
(`work/scalar/unit-vector-invariants-carried-as-prose.md` §RATIFIED):
they add EXACT frames whose axes are literal basis vectors, never a
constructor that accepts a pair a caller believes is orthonormal.
