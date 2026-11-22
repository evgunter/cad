---
id: the-viewer-spells-a-node-as-its-kind-before-its-name
kind: issue
title: viewer: a node is shown as 'Revolve "Revolve 3"', its kind before its name, where the person's name alone is wanted
status: open
opened: 2026-11-22
priority: P1
cost: M
refs: [tree-rows-lead-with-raw-tags]
---

**Ev reported this** (in chat, 2026-11-22): things "should not display like 'Revolve "Revolve 3"', they should just say "Revolve 3" or whatever the user's name for it is, without the additional 'Revolve' at the start".

The spelling comes from `impl Display for SpokenNode` (`crates/editor-core/src/spoken.rs:399-414`, doc at `:275-285`), which writes `{kind} "{label}" ({tag})`. The viewer passes it straight through `tree::node_label` (`crates/viewer/src/tree.rs:713`) and `tree::named` (`tree.rs:722-727`). `tree::headline` (`tree.rs:743-756`) already leads with the label, but only the feature tree uses it (`crates/viewer/src/pane/features.rs:58`). The sites Ev sees include the Properties heading (`crates/viewer/src/pane/properties.rs:513`, `:1389`), the display-section heading (`properties.rs:640`), the View pane's toggles (`crates/viewer/src/pane/view.rs:45`) and the placement editor (`crates/viewer/src/pane/create.rs:645`, `:1023`). There are about 105 `.spoken(` / `named` call sites across the viewer, many inside refusal and notice sentences.

The fix is one person-facing spelling (the label, with the kind only where no label exists or where two nodes share a label), used by every site that names a node to a person. `SpokenNode` is editor-core's, so either it gains that spelling or the viewer owns one beside `tree::headline`. This widens `work/chrome/tree-rows-lead-with-raw-tags.md` (open, P2), which is the same complaint scoped to tree rows.
