---
id: names-emit-keeps-an-unguarded-two-refusal-walk
kind: issue
title: names/emit keeps a two-refusal half-edge to face walk that no test distinguishes
status: dispatched
opened: 2026-09-19
priority: P3
cost: E
branch: emit/emit-rs-drivebys
---


## Finding

- **Where**: `crates/editor-core/src/names/emit.rs` — `rim_between`
  (~:766 and ~:769), the constants `DANGLING_MATE` and `DANGLING_LOOP`
  (~:717, ~:720), and `face_half_edges` (~:799).
- **Importance**: medium
- **Confidence**: sure — read, and the absence of a distinguishing test
  was checked by grepping both constant names over the whole tree.
- **Raised by**: PR #2857's fix pass, 2026-09-19, announced by seam from
  `work/dup/half-edge-to-face-walk-is-spelled-once-per-suite.md`.

`rim_between` walks half-edge → `parent_loop` → `.face` by hand and
refuses with a DIFFERENT constant per hop: `DANGLING_MATE` when the mate
half-edge does not resolve, `DANGLING_LOOP` when its loop does not. That
is the one thing `pncad::topo::Body::face_of_half_edge` (PR #2857,
`pub`, total, `Option`) cannot express, so the site is right to keep its
own walk.

**But nothing holds it there.** Two findings:

1. **No test anywhere distinguishes the two.** Each constant is used
   exactly once, at the site above, and neither name appears in any test
   in the tree. PR #2857's central result is that folding this posture
   onto the `Option` door leaves every other row green — it measured
   that at `topo`'s `splitting/join.rs`, where the whole 727-test lib
   suite passed the fold. `emit.rs` has no guard at all.
2. **What blocks the fold today is a data dependency, not a decision.**
   `mate_he` is reused at ~:772 for `.edge`, so the obvious fold does
   not typecheck. Move or duplicate that read and the fold becomes
   available and silent again. Safe by accident is not safe.

The repair is a guard that names both refusals — the shape of
`topo`'s `splitting::join::tests::he_face_names_the_key_that_went_stale`
and `body::tests::the_walk_consumers_keep_their_own_refusal`.

**A third refusal in the same file has never been censused.**
`face_half_edges` (~:799) raises `bug("face walk: dangling loop")`. It
is `face → loop` — the opposite direction, so NOT a member of the
half-edge → loop → face class — but it is the same unguarded
hand-written-walk shape and belongs in the same repair.

## What the instrument could not see

Found by a prose census (the walk described in words, every tracked
file, no path argument) plus a grep of the two constant names. Neither
reaches a refusal spelled inline without a named constant.
