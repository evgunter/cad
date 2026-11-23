---
id: a-cut-in-beside-a-pole-sliver-keeps-period-closure-so-the-op-refuses
kind: issue
title: A sphere cut-in beside a pole leaves a sliver the merge reads as a period closure, so the op refuses where main builds
status: open
opened: 2026-11-23
priority: P1
refs: [a-full-revolves-curved-wall-carries-two-seam-meridians-nobody-wrote]
cost: M
---


Found by the LATHE interior-loop lane (2026-11-23, PR 7440), on the
LATHE orchestrator's routing of the re-run of the base-vs-head mesh rows.

## What

`sweep/tests/spheres_crossing_off_every_edge.rs`'s
`a_trimmed_face_cut_beside_its_pole_builds` runs the unit ball less its
cap `x > 0.9` (a trimmed one-band face) against a ball of radius `0.01`
whose circle on the unit sphere passes `δ` from its pole. On the one-band
stack (PR 7440, head `36d2e703e4`), `b ∪ a` refuses at every `δ`
(`10⁻³`, `10⁻⁵`, `10⁻⁷`) with the no-crossings path's surviving-cut
refusal (`boolean::ops`, the cut-in's `survives`):

```
FallbackExtentUnsupported { operand: B, face: FaceKey(4v1), what: "the meridian the sphere face was cut in along survives the output merge, which glues it away only where the face's pieces merge back into one face" }
```

The row pins that cell as the refusal (`cut_survives`); the other five ops
at each `δ` keep their wants.

## Diagnosis (δ 10⁻³, `b ∪ a`)

From an instrumented run on the stack's head (not committed):

- the output merge skipped the cut ball's two pieces, faces `5v1` and
  `6v1`, with `PeriodClosure { edge: 4v3 }`;
- the two pieces share four edges, `1v3`, `3v5`, `4v3` and `5v3`: the cut
  meridian's pieces;
- `4v3` is a sliver about `10⁻³` long, from `(0, 0.9999995, 0.001)` to
  the pole `(0, 1, 0)`. It runs where the small ball's face (`2v1`, with
  its own wrap edge `2v3`) meets the cut beside the pole. The merge's
  bridge glue (`merge_faces::merge_group`, its duplicate pass and
  `curved_outline_ring`) reads it as a period closure rather than a
  bridge, so the pieces never merge back.

## Against main

On `origin/main` (`43295779ee`), the row is green: `b ∪ a` builds at
`δ 10⁻³`, and its volume, certificate and mesh hold. **Its result is
not maximal, though**, measured with a throwaway probe: 6 faces, 7
edges dividing two faces on one surface record, and the merge skipped
two groups as `PeriodClosure` (`[2v1, 5v1]` on edge `4v1`, and
`[3v3, 8v1, 7v1]` on edge `2v3`); `a ∪ b` reads the same. So main's
green is a sound but non-maximal body. On the stack, the op refuses
rather than answer that body. Either way, the regression against main is
"builds" turning into "refuses".

## Fix direction

Teach the bridge glue the sliver at the pole: a same-face duplicate whose
one end is the chart's pole is still a bridge on a simply connected
carrier. Then `b ∪ a` builds maximal, and the row's pin flips back to its
mesh wants (`Draws`, `Folds`, `Budget` by `δ`).
