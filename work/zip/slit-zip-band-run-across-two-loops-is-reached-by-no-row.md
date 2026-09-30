---
id: slit-zip-band-run-across-two-loops-is-reached-by-no-row
kind: issue
title: slit_zip's band-closure run that spans two loops of the folded face (mfkrh-then-kef) is reached by no row, so its transient promotion's sense is unexercised
status: open
opened: 2026-09-29
priority: P3
cost: M
---

## Finding

`crates/topo/src/boolean/rest.rs`, `slit_zip`'s band-closure loop
(~:1860–1885): a later run edge whose two halves lie in two loops of
the one folded face is killed by promoting the ring (`mfkrh(ring,
FaceSurface::Inherit)`) and then `kef` from the promoted side. PR
3467's review found no row in the topo or sweep suites that reaches
that arm: its mutants forcing the bit of every transient
`mfkrh(Inherit)` promotion in `boolean/finish.rs` and
`boolean/rest.rs` to `true`, to `false` or to the parent's each
survived both suites, and this arm is not entered at all.

Since PR 3467 the promotion takes the parent's bit negated on the
parent's chart (D1). The face dies in the `kef` that follows, so no
reader sees the bit today, but the arm itself (the `kef` from the
promoted side, the remnant merging into the other loop) has no row.

**What would close it.** A band-closure fixture whose second shared
run lands across two loops of the folded face, asserting the arm is
taken (the op sequence or a count) and that the body validates at
rest. If no reachable input produces the shape, say so in `slit_zip`'s
doc and make the arm a typed refusal.

Filed from PR 3467's fix pass (TOPO).
