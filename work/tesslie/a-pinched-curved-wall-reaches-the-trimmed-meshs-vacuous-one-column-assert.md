---
id: a-pinched-curved-wall-reaches-the-trimmed-meshs-vacuous-one-column-assert
kind: issue
title: A curved wall pinched at one vertex reaches trimmed.rs's 'vacuous' one-column repeated-id debug_assert at delta 0.02
status: open
opened: 2026-10-12
priority: P3
cost: M
---


## What

Found by FLUSH's pinch-residue lane (branch `flush/pinch-residue`).
`crates/mesh/src/trimmed.rs` (the uniform-candidates arm of the
trimmed-cylinder sampling) carries a `debug_assert!` that a trim polygon
never repeats a mesh id at two UV locations with a single interior
column (`nu == 2`), the shape #678's non-manifold fan needs
(`curved::pole_columns`). Its comment records the case as "VACUOUS
TODAY": `nu == 2` needs `u1 - u0 <= π/2`, and a repeat-apart was taken
to need the full 2π seam.

A body reaches it. `sweep` `curved_notch_pinch::curved_notch_pinch_one`
with `CURVED_POSE="3 1 0 ab S"` (the curved notch less the corner
pinch, plain booleans) builds on the lane's branch, where it refused
`JoinDesync` on main: exact volume, tier 2, the certificate, one
vertex per cone at `v`, reuse, and it meshes at δ 0.05. At δ 0.02 the
mesher panics there:

```
face FaceKey(6v1): a trim polygon repeats a mesh id at two UV locations
with a single interior column (nu = 2, nv = 2)
```

Not read: which mesh id repeats. The likely one is the pinch vertex
`v = (1, 1, 1)`, which a face of a pinched body may pass twice; if so,
one id sits at two UV locations on a narrow (`nu == 2`) span with no
full seam involved, which is the premise the comment rules out. The release build of `sweep`'s tests has debug
assertions on, so this is a panic there; a build without them reaches
whatever #678's fan does with that polygon.

## Open

Whether a face through one vertex twice on a narrow span needs the
`pole_columns` treatment, or the assert's premise is just wrong for
pinched faces (and what the mesh then is). Ruling S65 (`work/predtail/S65.md`)
is about whether the #678 backstop stays debug-only; this row is a
witness that the case it guards is reachable.

## Moved to TESSLIE (the mesh answers Ok, or panics, where it should refuse or mesh right) (2026-10-28)

Cut out of TESS by its semantic-seam split (`work/README.md`, Track size) when TESS measured 146 budget points against 30. Moved by `git mv`; the id and body are unchanged.
