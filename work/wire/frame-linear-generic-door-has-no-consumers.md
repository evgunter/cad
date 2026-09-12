---
id: frame-linear-generic-door-has-no-consumers
kind: issue
title: Frame::linear<T> is pub with zero call sites workspace-wide after PR 2375, kept alive only by its own test module
status: open
opened: 2026-09-11
refs: [2375]
---


## Finding

From the full review of PR 2375 (S4, confidence `sure`). Accurate at
that PR's head.

`crates/editor-core/src/placement.rs:230-232`'s `pub fn linear<T: Real>`
has **no consumers anywhere**: PR 2375 moved `determinant` onto
`linear_f64` and `affine` onto `affine_f64`, and repo-wide the only
`.linear::<` call sites left are the three inside that file's own test
module. The reviewer proved it mechanically — dropping `pub` yields
`warning: method 'linear' is never used` from
`cargo check --workspace --all-targets`.

The reviewer's own sentence is the finding: *"a public generic door kept
alive by its own test is exactly what the item was retiring at the other
end."*

## Why it was not done in PR 2375's fix pass

Deliberate, orchestrator's call. It is a **public API removal**, the
review classed it as a style finding (non-gating), and a fix pass exists
to repair what its review found wrong — not to widen the PR into a
surface decision nobody asked for. It gets its own row so the decision
is made on its merits.

## What the decision actually is

Three answers and none is obviously right:

1. **Delete it.** Nothing uses it, the workspace proves it, and this
   project is pre-release with no external consumers to break. Fail-loud
   says an unused public door is a claim the library does not keep.
2. **Keep it as API.** `Frame::affine<T>` is the kernel's placement
   door; `linear<T>` is its other half, and a library that offers the
   affine map at the backend scalar but not the linear part is oddly
   shaped from outside. If that is the argument, it belongs at the site,
   because right now nothing says it.
3. **Demote it to `fn`** beside `linear_f64` and `affine_f64`, keeping
   it for whatever reads it next. Cheapest, and the one that leaves the
   question open rather than answering it.

Whoever takes it should check first whether `Frame` is re-exported on a
public path at all and whether the Python surface reaches it — the
review measured the *workspace*, which is the right instrument for (1)
but not for a claim about external users.
