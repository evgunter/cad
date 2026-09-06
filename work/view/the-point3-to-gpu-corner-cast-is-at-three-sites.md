---
id: the-point3-to-gpu-corner-cast-is-at-three-sites
kind: issue
title: three sites cast a Point3 to a GPU corner and the prose reconciling them names two
status: open
opened: 2026-09-06
refs: [2083]
---


Found by the style review of #2083 (pre-existing; the move carried the
reconciling sentence into a new file, where it now understates the
count).

`crates/viewer/src/marks.rs:130-133` — `EdgeOverlay`'s doc — says *"The
buffers are `f32` because that is what a GPU consumes and this is the
display seam — the same cast, at the same boundary, that
[`crate::scene::SceneMesh`] makes."* That names two sites. There are
three:

- `crates/viewer/src/marks.rs:272` —
  `|point: &Point3<f64>| [point.x as f32, point.y as f32, point.z as f32]`
- `crates/viewer/src/scene.rs:492` —
  `positions.push([p.x as f32, p.y as f32, p.z as f32])`
- `crates/viewer/src/pane/viewport.rs:311` —
  `.push([world.x as f32, world.y as f32, world.z as f32])`

The third is not mentioned anywhere. This is
`docs/prompts/reviewer-style-lane.md` Q2's *"comment that exists to
reconcile two spellings of one rule"* shape: the sentence is the only
thing holding the three in correspondence, the code compiles either
way, and the sentence is already wrong about how many there are.

Whether the cast wants a named door (`Point3<f64> -> [f32; 3]` at the
display seam, once) is the question; there is no constant or tolerance
involved, only a repeated three-field cast, so the cost of a door is
small and the cost of the current arrangement is a sentence that has
to be maintained by hand.

## Where else to look

`grep -rn 'as f32' crates/viewer/src` — the same seam is crossed by the
normal and by the `[f32; 4]` colour paths, and neither is covered by
the sentence.

## Confidence

`sure` on the three sites and on the prose naming two. `unsure` whether
a shared door is worth it.
