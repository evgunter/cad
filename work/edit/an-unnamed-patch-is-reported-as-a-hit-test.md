---
id: an-unnamed-patch-is-reported-as-a-hit-test
kind: issue
title: patch_names reports a patch with no name through HitTestError, so the sentence names a hit test that did not happen
status: review
branch: edit/unnamed-patch-is-a-lookup
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

## Ruled and spec'd (2026-09-29, EDIT orchestrator) — E-class (green CI and the orchestrator's read), branch `edit/unnamed-patch-is-a-lookup`

The row's remedy is the ruling. The per-entity lane of
`NodePick::patch_names` and `boundary_names` carries a naming error of
its own. It holds the `Unnamed` fields, its `Display` names a table
lookup, and it names no hit test. The lane's type is exactly the state
it can hold, so no hit-test arm is admissible there. The viewer
readers (`idpass.rs`'s `IdAnswer` and `pickindex.rs`'s
`EdgeNameFault`) forward it unchanged: an announced crossing onto
VGEOM's ground. One `Display` pin per new sentence goes in the F6
census.

## Built (2026-09-29)

`UnnamedEntity` (`crates/editor-core/src/resolve/hit.rs`) is the name
lookup's one refusal: the `Unnamed` fields, a `Display` that opens
*"name lookup:"* and names no hit test. `NodePick::patch_names` and
`boundary_names` answer `Vec<Result<StableName, UnnamedEntity>>`; the
node's standing is settled once for the call (`standing`, the one
ladder every door of `hit.rs` and `pick.rs` now climbs), so the
per-entity lane holds exactly that. `HitTestError::Unnamed` carries
the same value and forwards its sentence under *"hit test:"*, the one
door where that prefix is true. The viewer readers (`IdAnswer::Unnamed`,
`WindowFault::Unnamed`, `EdgeNameFault::Unnamed`, `PickIndex::name_of`)
carry `UnnamedEntity` and forward it unchanged; the edge pick's unnamed
edge is `PickError::EdgeName`, not a `HitTestError`. The façade carries
the type; the Python slot value crosses as the `unnamed` arm of
`HitTestError` (`work/lib/the-unnamed-slot-crosses-as-a-hit-test-error`).
Premise corrected: the lane held the standing arms too, for a later
evaluation in which the node failed; that refusal is now of the call.
Sibling left: `the-standing-ladder-speaks-as-a-hit-test-at-every-door`.

