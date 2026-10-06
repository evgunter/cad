---
id: a-verdicts-other-nodes-are-said-by-tag-once-deleted
kind: issue
title: A selection verdict's diagnosis names nodes the selection does not keep, and says them by tag once deleted
status: open
opened: 2026-10-06
---


Found in review of `a-selected-node-deleted-is-said-by-tag-where-the-tools-say-its-label` (PR 4086).

**The case.** The selection keeps only the nodes `Selection::nodes`
names (`crates/viewer/src/session/select.rs:264`): the node itself, or
for a pick its minter, its feature and the hit body. `standing_verdict`
(`crates/viewer/src/pane/properties.rs`) says a `ResolveError` by
`Speaker::of(landed).or_held(said)`, and a `Vanished` verdict's
`Diagnosis` (`crates/editor-core/src/resolve/mod.rs:292`) can name other
nodes:
- `Diagnosis::StructuralParam`'s `node` (`resolve/mod.rs:340`);
- `Diagnosis::Upstream`'s `node` (`resolve/mod.rs:367`);
- `Diagnosis::Cascade`'s `through` (`resolve/mod.rs:353`);
- `Diagnosis::BorderDelta`'s `gone` and `new` walls (`resolve/mod.rs:307`).

If the landed document no longer holds one of these, the selection kept
nothing for it, so it is said `node <tag>`. Its minter in the same
sentence is said by its label, so one sentence mixes the two.

**The fix.** Keep the nodes the verdict names as well as the
selection's. Either respeak the verdict's own nodes with `held_by` when
the standing is computed, or widen what the session keeps to the nodes
the diagnosis names.
