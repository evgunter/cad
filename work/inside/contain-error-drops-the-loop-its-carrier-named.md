---
id: contain-error-drops-the-loop-its-carrier-named
kind: issue
title: ContainError's From<PointInLoopError> discards the loop key two of its arms were given, so a refusal that names its mechanism cannot name its subject
status: open
opened: 2026-09-12
refs: [2420]
priority: P1
cost: E
---



(FIX orchestrator, 2026-09-12) Found by the
`census-containment-flatten-fabricates-its-diagnostic` lane (PR 2420)
and **reported rather than filed by it** — `crates/topo/src/boolean/*`
is claimed by BOTH `bool` and `curved`, so the owner is disputed rather
than clear, which is the case `work/README.md` reserves
`work/issues/` for. Either program may claim it by moving this file.
Verified by the orchestrator before filing.

## The defect

`crates/topo/src/contain.rs:64-72`:

```rust
impl From<PointInLoopError> for ContainError {
    fn from(e: PointInLoopError) -> Self {
        match e {
            PointInLoopError::Escalated { diag, .. } => Self::Escalated(diag),
            PointInLoopError::RayExhausted { .. } => Self::RayExhausted,
            PointInLoopError::CorruptLoop { .. } => Self::Corrupt,
        }
    }
}
```

`PointInLoopError::RayExhausted { r#loop }` and `::CorruptLoop { r#loop }`
**each carry the loop they are about**, and both conversions discard it
with `{ .. }`.

**What makes it a defect rather than a narrowing is the comment
immediately below it** (`contain.rs:74-78`): *"Each arm names WHAT
STOPPED and the repair that moves it, because a consumer that carries
this refusal renders it verbatim and adds no sentence of its own."*
Two of those three arms cannot name **which** loop — while
`ArcLoopUnsupported`, sitting beside them in the same enum, does. A
reader told to *"re-model the loop"* is not told which one.

## Why now

PR 2420 gave `ContainError` a `Display` and routed it through the tier-3′
census as a typed cause, so these arms now reach a **user** rather than
stopping at a `Debug` in a developer's log. The payload loss was
survivable while nothing rendered it; it is not now.

## Not this item

The census-side flatten is fixed (PR 2420). This is one layer down, in
the conversion that feeds it.

Moved from `work/issues/` to `work/inside/` on 2026-11-03, when the `work/issues/` bucket was retired (Ev's direction, in chat); id and body unchanged.

## Fixed (2026-11-25, lane `inside-contain-small`)

The premise had half rotted: the conversion now lives in
`crates/topo/src/contain_loop.rs` (`From<PointInLoopError> for
ContainError`), and `CorruptLoop { r#loop }` already reached the door as
`ContainError::LoopUnreadable(r#loop)`, rendered with its key.
`RayExhausted { .. }` still dropped its loop, there and in `contain.rs`'s
`region_placement` (`RegionRefusal::RayExhausted { outer }`).
`ContainError::RayExhausted` now carries the loop (`RayExhausted(LoopKey)`)
and `ContainError::write` names it; `Uncrossable`, which carried its loop
but did not render it — the arm whose repair is "re-model the loop" — names
it too. The sweep's hit list is in the PR body; its out-of-unit hits are
filed as `arc-end-clear-of-corners-names-no-edge`,
`solid-door-reach-drops-the-faces-uncovered-edges` (INSIDE),
`chart-region-face-area-drops-the-torn-loop` (CHARTTAIL) and
`census-face-boundary-reads-drop-the-torn-loop` (ATREST).
