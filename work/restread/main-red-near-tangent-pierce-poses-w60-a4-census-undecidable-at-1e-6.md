---
id: main-red-near-tangent-pierce-poses-w60-a4-census-undecidable-at-1e-6
kind: issue
title: Main red at ε 1e-6: near_tangent_pierce_poses' w60 nt e1 a4 d1e-7 prism ∪ cube fails tier 3′ CensusUndecidable
status: open
opened: 2026-11-01
---

Found by the TANGRIM inflection-ruling lane's outcome diff, on
`origin/main` e669a2f891 (the merge of PR 6217) and identically on its
branch, which touches no planar-pair path.

**The red.** `sweep::all near_tangent_pierce_poses::near_tangent_pierce_poses_build_their_closed_form_or_refuse_typed`
(slow set) fails at ε = 1e-6 (`near_tangent_pierce_poses.rs`, its
final `assert!`); it passes at the default ε. It reads "1 of the poses'
runs are wrong: w60 nt e1 a4 d1e-7, prism ∪ cube: fails tier 3′
definitely", with the one finding

```
CensusUndecidable { a: Solid(1v1), b: Solid(2v1),
  what: "another finding left their boundaries unchecked, and this one cannot be decided until that is resolved. Recourse: resolve that finding first",
  reason: MayOverlap }
```

This is not the closed
`work/restread/main-red-near-tangent-pierce-poses-pinned-finding-moved-at-1e-6.md`
(pose w345 a3.8, a pinned finding's numbers). Not bisected; the first
thing owed is the commit that turned it red, then which finding left
the boundaries unchecked.

Moved from `work/issues/` to `work/restread/` on 2026-11-03, when the `work/issues/` bucket was retired (Ev's direction, in chat); id and body unchanged.

## Dated and pinned (GREEN, 2026-11-14)

Still red on main `b363f5fa1` with the same finding. In the nightly it passed at ε 1e-6 on 11-01 (main `fbfebd92`) and has failed since 11-02 (main `8a440cfa`).

Bisected, first-parent, at ε 1e-6: green at `082c42d27`; red at `367ff259a`, the merge of PR 6212 (`splitdoor/prism-tilt-reach`, which changed `crates/topo/src/boolean/sectors.rs`, `insert.rs` and `vtxfac.rs`), whose first parent is `082c42d27`; red at `d05cc76c9`, `8a611050d`, `286c28f0a`, `e669a2f89` and `cda6d86c4`. So the boolean changed what it builds for this pose, and the census then cannot decide the body. Which finding leaves the boundaries unchecked is still owed, and whether the body PR 6212 builds here is right is SPLITDOOR's half of the question.

The row is under `test_utils::pinned_red::row` at ε 1e-6 on that message (`green/eps-1e-6`): the one wrong run, `w60 nt e1 a4 d1e-7, prism ∪ cube`, `CensusUndecidable`, `MayOverlap`. Take the pin off with the fix.
