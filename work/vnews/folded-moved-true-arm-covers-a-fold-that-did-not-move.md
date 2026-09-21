---
id: folded-moved-true-arm-covers-a-fold-that-did-not-move
kind: issue
title: frame::folded_moved answers true for a fold that refused and did not move, under a name and a doc line that say only moved
status: open
opened: 2026-09-19
priority: P1
cost: E
---

Found by the sweep `is-instance-collapses-absent-and-wrong-kind`
carried, which had `frame::folded_moved` on its list of four
unadjudicated siblings. It is **not** that row's class — the type is
not hiding news a reader wants — so it is filed rather than fixed
there, and `frame.rs` was off limits to that lane besides.

## The mismatch

```rust
/// Whether a folded event stream actually moved the camera.
pub fn folded_moved(folded: &Folded) -> bool {
    !folded.applied.is_empty() || folded.refused.is_some()
}
```

`crates/viewer/src/frame.rs`, `folded_moved` (~`:2096`). The `true` arm
covers two states: a fold that applied camera operations, and a fold
that applied none and carries a refusal. **The second did not move the
camera.**

**The collapse is deliberate and asserted**, which is why this is a
prose row and not a behaviour one.
`a_refused_fold_is_news_about_the_camera_and_apply_overwrites_with_it`
(`frame.rs`, the module's own tests) asserts `folded_moved` of a
refused fold with the reason inline — *"a refusal is a camera event
too"* — and `frame_policy.rs`'s `folded_moved` row pins both arms the
same way. The module doc states the question correctly too
(`frame.rs:25`, *"What a folded event stream amounts to"*). So the
value is right and two of the three places that describe it are right.

**What is wrong is the name and the function's own first line**, which
both say *moved* over an arm that did not move. The one production
caller (`pane::viewport`, the `land` guard) wants both states, because
`land` both applies the move and delivers the refusal, and the doc
comment says the guard exists to keep *"`land` is documented as the one
place a camera MOVE becomes application state"* true. A predicate whose
`true` arm includes a fold that did not move is a weaker guarantee than
that sentence claims, so the next reader building on it gets the weaker
thing without being told.

## The fix

Re-word the first doc line to the question the value actually answers —
*whether a folded stream is a camera event at all* — and rename if the
lane taking it judges the rename worth the churn (three production
sites plus two test files). VNEWS's own rule is that a rename nobody
reads is not news, so the doc line is the part that has to move and the
name is the part that is arguable.

## Home

VNEWS's: `crates/viewer/src/frame.rs`. It takes with whichever
`frame.rs` lane is next in the serialized cluster — this is a doc
comment and possibly a name, not a build of its own.
