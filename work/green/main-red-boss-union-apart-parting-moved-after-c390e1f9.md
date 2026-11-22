---
id: main-red-boss-union-apart-parting-moved-after-c390e1f9
kind: issue
title: Main red: revalue_boss_union_apart_to_its_parting's pinned (decisions before the parting, checks) moved after c390e1f9, in the default set
status: open
opened: 2026-11-22
priority: P0
cost: E
---


Found by BOXFINISH's doors-read lane (PR 7384) on 2026-11-22, re-merging
`origin/main` at `69f31a4e`. `editor-core`
`revalue_corpus::revalue_boss_union_apart_to_its_parting` is red on
`69f31a4e` itself (run in a clean checkout of it, default filter,
default ε). It passed on `c390e1f9` in that lane's slow-set runs at the
default ε, 1e-6 and 1e-12, so a first-parent merge in
`c390e1f9..69f31a4e` (about forty) moved it. Not bisected.

`crates/editor-core/tests/revalue_corpus.rs:520`:

```
boss_union: (decisions in step before the parting, checks paired) are not the pinned ones, which hold at every ε row
  left: (2390, 3856)
 right: (2360, 3857)
```

It parts at `(2390, 7417, "pcurve_cylinder_chart_incident")`, with
`moved: 1474, drifted: 0, escalated: 0`, and `laundered` empty. `seen`
includes `session_box_reach: 1` and `mate_spin_cut_side: 1`. Two
first-parent candidates touch the profile-reading paths the row walks:
the reach reader (`crates/editor-core/src/mate/reach.rs`, `1f053f3b`
and `342f9117`) and `profile/*` merges (#7403, #7260, #7263, #7289,
#7290, #7284). The test file itself is untouched in the range.

The nearest open row, `inlane/under-the-guided-lift-boss-unions-bool-pair-gap-decisions-swap-order-under-the-apart-move`,
holds the instrument's order pairing, not this pin.

**Owed:** bisect the range on the first parent; re-pin with the moving
PR's reason, or fix what it moved.
