---
id: an-unnamed-patch-is-reported-as-a-hit-test
kind: issue
title: patch_names reports a patch with no name through HitTestError, so the sentence names a hit test that did not happen
status: open
opened: 2026-09-25
priority: P4
cost: D
---

Filed by `vnews/an-unnamed-id-is-not-nothing` (PR #3249).

## The finding

`crates/editor-core/src/resolve/pick.rs`'s `NodePick::patch_names`
names every patch of a drawn mesh by a table lookup (`entity_name`),
and reports a patch with no name as `HitTestError::Unnamed`. The
error's `Display` (`crates/editor-core/src/resolve/hit.rs`) starts
every arm with *"hit test: "*. So when the viewer forwards that
refusal in the naming layer's own words, which is this crate's rule
(`PickError`'s and `EdgeNameFault`'s `Display` docs), the status line
reads *"id buffer id 9, a drawn patch: hit test: node 2's face in output
body 0 evaluated but has no name in its table — …"*. No hit test ran;
the index was built by a lookup.

`boundary_names` has the same shape for edges, and it surfaces the same
way through `EdgeNameFault::Unnamed`.

The type is also wider than the state that holds it. The per-patch
lane only ever holds `Unnamed`, but `Result<StableName, HitTestError>`
admits every hit-test arm, so a reader has to handle refusals that
cannot occur there.

## What a fix would be

The per-entity lane of `patch_names` / `boundary_names` carries a
naming error of its own (the `Unnamed` fields), whose `Display` names a
lookup and not a hit test. Readers in `crates/viewer` forward it
unchanged.

## Fence

`crates/editor-core/src/resolve/pick.rs` and `resolve/hit.rs` (EDIT).
The viewer readers are `crates/viewer/src/idpass.rs` (`IdAnswer`'s
`Display`) and `pickindex.rs` (`EdgeNameFault`).
