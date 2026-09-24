---
id: kernel-direct-origin-does-not-separate-hand-built-from-derived
kind: issue
title: GeomOrigin::KernelDirect holds two of the four origins: nothing inside the kernel can tell a hand-built description from a derived one
status: open
opened: 2026-09-14
priority: P1
cost: D
---



## Finding

`geom-source-absence-conflates-four-origins` asked for four origins
told apart: imported, hand-built, kernel-derived, and
cleared-and-not-re-stamped. `crates/topo/src/source.rs`'s `GeomOrigin`
ships three of them plus the recipe arm, and puts hand-built and
kernel-derived on ONE arm, `KernelDirect`. This row is the fourth
separation, disclosed at the moment it was left out.

**Why it was left out.** Every description enters the arenas through
`Body::add_surface`, `Body::add_curve`, `Body::add_null_curve` and
`Body::add_point` (`crates/topo/src/body.rs`), which are `pub(crate)`
and carry no caller identity. Those doors DO write a mark —
`GeomOrigin::KernelDirect`, unconditionally, which is what makes the
origin map total over live keys — but the mark they can write is the
fused one: seventeen files in the crate reach `add_surface`, and they
are exactly the mixture: `euler.rs` (the make-operators a hand-built
body is assembled from), `euler_ring.rs`, `euler_kill.rs` and
`split.rs` on one side; `boolean/boxes.rs`, `chord_join.rs`,
`chart_region.rs` and `transform.rs` on the other. What separates a
hand-built description from a derived one is **which public door the
caller opened**, and nothing carries that fact down to the mint.

So the separation is a per-public-door decision — every `pub` door in
`crates/topo` that mints geometry says which of the two it is — and
that is a sweep of its own with a real design question inside it:
`mev` on an existing body, `split_edge`, `merge_faces` and
`transform_rigid` are not obviously either. That question is why this
is a row and not a line in the unit that found it.

**The sweep's size, measured** (review of PR 2576). 45 functions under
`crates/topo/src` (review and fixture files excluded) call an `add_*`
mint door; 5 of them are `pub` (`mvfs`, `split_edge`,
`offset_charts_together`, `offset_planes_together`,
`replace_faces_offset`) and the rest are reached through `pub(crate)`
helpers (`mev_*_execute`, `mint_face_surface`, `mekr_mint`,
`set_edge_curve_via`) behind the euler doors. The public doors a
per-door stamp would have to speak for are order 10–15 — a bounded
sweep, doable in a unit. The blocker is the design question above, not
the reach.

## What a taker owes

Decide whether the distinction is worth a per-door stamp at all. The
downstream consumer named by the sequence
(`work/exch/step-import-discards-the-entity-ids-that-are-its-identity-channel`,
step 2) needs `Imported` told from everything else, which ships; no
reader in the tree today asks for hand-built vs derived. If nothing
does, the honest close is to say so on `GeomOrigin::KernelDirect`'s
doc and retire the four-way framing, rather than to build a channel
with no consumer.

If a taker does build it, the shape is already there:
`GeomOrigin::KernelDirect` (`crates/topo/src/source.rs`) splits into
`Constructed` and `Derived`, the mint doors take which one to write
from their caller, and the public doors decide it. Nothing about the
`Recipe`, `Imported` or `Cleared` arms moves, and N6 is untouched
either way — the origin channel decides nothing the coincidence ladder
reads.

## refs

Found by `geom-source-absence-conflates-four-origins` (TOPO block
TOPO-B4 slot 0), whose PR states the deviation and this row's number.
