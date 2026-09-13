---
id: guide-has-no-chamfer-or-tube-step
kind: issue
title: docs/GUIDE.md has no chamfer or tube step for the recipe-door ladder
status: closed
opened: 2026-09-08
closed: 2026-09-08
parent: LIB-SMALL
---

`docs/LIB-G17-SPEC.md` §7 placed the shell's guide step "beside chamfer
and tube", and `docs/GUIDE.md` has neither: the recipe-door ladder
there runs fillet → selection → (now) shell. `Node.chamfer`,
`Node.tube` and `Node.hollow_tube` are bound and executed by
`tests/test_north_star.py`, but no guide block spells them — a reader
of the guide does not meet the chamfer's twin-of-fillet rule, the
tube's intent parameters, or the hollow tube's required wall. Two
guide steps, each a self-contained ```python block `test_guide.py`
executes; the shell step (`### Hollowing a body`) is the shape.
Measured at LIB-G17, which put the shell step after the selection
section and recorded the gap here rather than writing the neighbours
it was told to sit beside.

## Closed

Closed by LIB-SMALL. `docs/GUIDE.md` gains two recipe-door steps
before `### Hollowing a body`, which is where LIB-G17's spec said the
shell step sits "beside" them:

- `### Chamfering the same edges: fillet's twin` — the twin rule (the
  same opaque, frozen edge selection) and the two things that differ:
  `distance` is a setback along each support rather than a radius, and
  both supports must be planes. The block meters the derived closed
  form, shows the chamfer taking more than the fillet of the same
  size, counts the named faces the result carries, and catches
  `chamfer_selection_empty` in the chamfer's own word.
- `### Tubes: a ring from its intent, and the same ring with a wall` —
  the five intent parameters (spine datum axis, `u_ref`, major radius,
  window, minor radius), then `Node.hollow_tube` as a different node
  kind whose `wall` is required and whose `minor_radius` is the outer
  radius. One block covers both doors so the pair reads as one story,
  ending on the solid-minus-hollow bore differential.

`tests/test_guide.py` executes both: 36 blocks before, 38 after.
