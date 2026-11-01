---
id: circle-torus-root-slack-divides-by-a-spread-sine-read-off-its-cosine
kind: issue
title: The circle x torus root slack divides by sin_spread = sqrt(1 - c^2), whose own rounding floors at sqrt(2u) near a tangent root, where the acos linearisation it serves has already failed
status: open
opened: 2026-10-13
priority: P2
cost: E
---

Found by the review of PR 5004 (`pcert/cone-section-interval-beta`),
which fixed a `√(1 − x²)` read near `x → 1` in `chart_pcurve`'s cone
arm. Not measured here: this is the arithmetic, and whether a verdict
moves is the owner's to say.

## What happens

`parallel_axes_roots` (`topo/src/boolean/circle_torus.rs`) places the
roots at `spread = acos c` and meters their slack as
`radius·(charge/slope + rounding/sin_spread)`, with
`sin_spread = √(1 − c²)` (`circle_torus.rs:609`) and the same
`sin_spread` in `slope`.

- Near a tangent root (`c → ±1`), `sin_spread` read off `c` carries
  about `√(2u)` (2e-8) from `c`'s rounding. A truly tangent pose can
  read `sin_spread ≈ 2e-8` rather than 0, so the slack comes out
  finite where it is unbounded.
- `rounding/sin_spread` is the linearised error of `acos`. That holds
  only while `rounding ≪ sin_spread²`, which fails at the same
  `sin_spread ~ √rounding` where the root's floor sets in. Past it, the
  error in `spread` is about `√(2·rounding)`, not `rounding/sin_spread`.

## What closes it

Bound the root placement's own error by `√(2·rounding)` once
`sin_spread²` is not well above `rounding`, or read `sin_spread` from
the geometry rather than from `c`. Add a row at a pose within `√u` of
a tangent root showing the slack's verdict there.

## Slate triage (2026-10-26, CLEAVE)

- Re-banded P3 → P2: a root slack linearised past where it holds: interval and error propagation, P2 by `work/README.md` Priority.
- Moved to HONEROOT (`work/honeroot/`) by `git mv`, id and body unchanged: whose charter is the boolean's root doors and sector readers and their margins. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).
