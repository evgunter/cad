---
id: mispaired-ids-exempts-the-empty-window
kind: issue
title: MispairedIds exempts the zero case, which is the window shape most worth checking
status: open
opened: 2026-09-04
refs: [1768, 1098]
priority: P3
cost: E
---

Found by CHROME's style lane on PR 1768, against that PR's own claim.

`crates/viewer/src/scene.rs:445` refuses `SceneError::MispairedIds`
when a drawn part's id count disagrees with its mesh's patch count —
but it guards with

```rust
!part.ids.is_empty() && part.ids.len() != part.mesh.patches.len()
```

so a part whose `patch_names(eval)` yields **zero** while its mesh has
patches passes silently and is drawn with no ids at all.

**Why this is worth a file rather than a shrug.** PR 1768 made that
check meaningful for the first time: before it, the ids were sliced to
a length taken from `mesh.patches.len()` in the first place, so the
comparison was tautological. After it, the id count comes from the name
list and the check really compares two sources. The PR body said it was
"live on exactly the invariant" — true only for NONZERO mismatches.

And the exempted shape is the one that unit singled out as
interesting: an empty run still OWNS a window, which is why 1768 covers
it synthetically at the structure (`pick.rs`'s unit tests) rather than
through the tessellator, which cannot produce it. The one case the
refactor went out of its way to reason about is the case this guard
skips.

Not fixed on 1768: narrowing the guard is a behaviour change in
`scene.rs` with its own evidence to write, and that unit's subject was
the window index. Whoever takes it should establish first whether a
zero-name part with a nonempty mesh is reachable at all — if it is not,
the guard's exemption is harmless and the finding is the comment that
does not say so.

Signed: (CHROME orchestrator)

## Parked on VIEW's index-buffer row, 2026-09-15

`scene.rs` is ceded to VIEW under the carve-out, and VIEW's
`scene-mesh-carries-an-identity-index-buffer` rewrites the tail of
`SceneMesh::build_parts_focused` — the very function this guard opens.
Editing the guard from CHROME's side while that change is pending would
put two branches in one function.

Note for whoever takes it: the row makes a reachability question a
precondition, and answering it needs `NodePick::patch_names` in
`crates/editor-core/src/resolve/pick.rs` — a claim about another
crate's emission, outside both programs' viewer ground.

## The trigger fired and left this row untouched (2026-09-15)

VIEW closed `scene-mesh-carries-an-identity-index-buffer`, so `parked`
became false and lint said so. Re-opened.

The park note expected that change to rewrite the tail of
`SceneMesh::build_parts_focused` — the function this guard opens. It
did, and **the guard is unchanged**:

```rust
if !part.ids.is_empty() && part.ids.len() != part.mesh.patches.len() {
```

So the finding is live exactly as filed, and the reason for parking is
spent. The reachability question the row makes a precondition is also
untouched: answering it still needs `NodePick::patch_names` in
`crates/editor-core/src/resolve/pick.rs`, outside both programs' viewer
ground.

Ground is ceded to VIEW under the carve-out, so CHROME does not work it.
