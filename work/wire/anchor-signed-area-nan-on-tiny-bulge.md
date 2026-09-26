---
id: anchor-signed-area-nan-on-tiny-bulge
kind: issue
title: anchor::signed_area returns NaN on a tiny nonzero bulge, which fires the replay/naming debug_assert
status: open
opened: 2026-09-25
---


`crates/editor-core/src/eval/anchor.rs::signed_area` computes each arc's
circular-segment term as `0.5 · r2 · (θ − sin θ)` with
`r2 = chord2 / (4·half²)`, `half = sin(θ/2)`, `θ = 4·atan(b)`. For a tiny
nonzero bulge such as `b = 1e-300`, `half²` underflows to 0, so `r2` is
∞, while `θ − sin θ` is exactly 0: the term is ∞ · 0 = NaN, and so is the
loop's area. (At `b = 1e-150` the term is a finite 0; the NaN needs
`half²` below the subnormal range.)

`replay_naming` then refuses to order the loops (`!largest.is_finite()`)
and returns `None`. Validation classifies the same segment a `Line`
(its sagitta is far inside the band), so the loop validates and
`naming_of` returns `Some`. The `SetProgram` door in
`crates/editor-core/src/edit.rs` asserts the two agree on every program
it admits (the `debug_assert_eq!` beside
`crate::eval::replay_naming(&new_loops)`), so a program with such a
bulge fires that assert in a debug build. In a release build the replay
reading strands every name of the profile where the validated reading
would carry them.

This is pre-existing: the same arithmetic ran on `bulge != 0.0` before
PATHS `canonical-segment-type-in-profile` (#3224), whose review found it.
A repair reads the area the way validation classifies it: a segment too
shallow to be an arc contributes no circular segment, or the term is
written in a form that cannot evaluate ∞ · 0. The
`work/paths/store-constructed-carriers.md` unit deletes this hand copy;
whichever lands first owns the fix.

## EMIT note (2026-09-25)

EMIT's PR 3223 deletes `signed_area`, `replay_naming` and `naming_of`,
along with the anchor code this row cites. A profile's names no longer
depend on its loops' signed areas. Whether this row is closed is WIRE's
call.
