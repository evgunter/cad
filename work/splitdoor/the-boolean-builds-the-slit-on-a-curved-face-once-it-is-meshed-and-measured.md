---
id: the-boolean-builds-the-slit-on-a-curved-face-once-it-is-meshed-and-measured
kind: issue
title: Build the slit form on a curved face, retiring EdgeInFaceGap::CurvedFace, once the walk meshes it and props measure its ring
status: open
opened: 2026-10-30
priority: P2
cost: E
---


`crates/topo/src/boolean/slit.rs`, `part_riders`, refuses
`EdgeInFaceGap::CurvedFace` where a line edge lies in a cylinder or
cone face and its material parts there, or a wall of it lies tangent to
the face (PR 6061). The parting itself builds on a curved face unchanged, and its
result passes tiers 3 and 3′; this was measured on PR 6061's first
commits, where `block ∖ trench`
(`crates/sweep/tests/a_keel_on_a_bores_ruling.rs`) built two keel edges
each bounding the bore. It refuses only because two consumers do not
read the result:

- `work/tesslie/the-walk-panics-on-a-slit-in-a-curved-face.md`: the open
  slit;
- `work/nearpoint/a-zero-area-ring-in-a-cylinder-face-measures-degenerate.md`:
  the closed ring.

Once both land, drop the `CurvedFace` gate and its variant, and re-pin
`a_keel_on_a_bores_ruling.rs`'s typed-refusal row and
`verbs_ga_r2_probes.rs`'s `p ∖ b` at their closed forms (`p ∖ b`:
`4π − 2·√0.91·0.36`).
