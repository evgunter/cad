---
id: a-held-box-keys-an-authored-negative-zero-apart
kind: issue
title: A held (Fixed) box binds an authored -0.0 as +0.0, so a slot reading it keys differently from the unboxed build
status: open
opened: 2026-10-18
---


## Finding

Found by the stage 3 C review (PR #5267, R2's m4). A held axis of a
parameter box binds `nominal + 0.0`
(`crates/editor-core/src/analysis.rs`, `var_env_over`, the `Fixed`
arm: "`-0.0` lands on `+0.0` there too"), while the unboxed build binds
the nominal's own bits (`Doc::var_env`). A content key feeds each slot's
value by its bits (`crates/editor-core/src/eval/mod.rs`, the slot loop
in `content_key`), so a slot reading a variable authored at `-0.0` keys
differently under an all-`Fixed` box than without one — though the box
varies nothing, which `m10_4_seed::an_unseeded_evaluation_is_the_build`
holds is not an analysis.

Measured: `box_slid_by_a_parameter(-0.0)` (a Length declared at `-0.0`
read by a slide value) keys its placement apart under an all-`Fixed`
box; at `+0.0` the two keys agree. PR #5267 removed the case its own
writers produced (`mate::pin`'s `written` emits no signed zero), so the
corpus row passes; the class is any authored `-0.0`, mates or not.

## What would close it

One spelling of a held zero: the box door binds a held axis's nominal
bits as the unboxed build does, or the declaration doors write no
signed zero (as `mate::pin::written` does), or the key feeds a slot's
zero unsigned. Whichever is chosen, `m10_4_seed` gains a row over a
document holding an authored `-0.0`.
