---
id: guide-has-no-chamfer-or-tube-step
kind: issue
title: docs/GUIDE.md has no chamfer or tube step for the recipe-door ladder
status: open
opened: 2026-09-08
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
