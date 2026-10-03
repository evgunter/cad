---
id: a-declared-seam-subtract-and-intersect-stop-at-the-fallback-extent
kind: issue
title: The sphere-capped tube declared a Seam builds its union, and its subtract and intersect stop at the fallback extent
status: open
opened: 2026-10-02
---


## What

The sphere-capped tube, walls declared `Seam` and discs `Rest`, builds
its union in both orders
(`crates/sweep/tests/pi_seam_and_kiss_through_the_boolean.rs`,
`the_sphere_capped_tube_builds_with_its_walls_declared_a_seam`). Its
`tube ∖ cap`, `cap ∖ tube` and `tube ∩ cap`, under the same
declarations, refuse `FallbackExtentUnsupported`: "the sphere's section
circle runs near the plane face's boundary — whole-circle membership
cannot be certified from the enclosures, and no crossing layer saw an
event". The row pins it.

## Why it is wrong

The answers are closed-form: the tube, the half ball, and empty (the
two share only the disc and the rim). The ops take the no-crossings
fallback, and its containment read of the sphere's section against the
disc lands on the rim itself, which is the disc's boundary. The union
never asks that question, because the declared-REST zip removes the
discs first.
