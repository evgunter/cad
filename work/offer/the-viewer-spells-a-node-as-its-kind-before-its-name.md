---
id: the-viewer-spells-a-node-as-its-kind-before-its-name
kind: unit
title: viewer: a node is shown as 'Revolve "Revolve 3"', its kind before its name, where the person's name alone is wanted
status: closed
opened: 2026-11-22
priority: P1
cost: H
refs: [tree-rows-lead-with-raw-tags]
pr: 7375
branch: offer/a-node-is-spoken-as-a-mention
closed: 2026-11-23
---

**Ev reported this** (in chat, 2026-11-22): things "should not display like 'Revolve "Revolve 3"', they should just say "Revolve 3" or whatever the user's name for it is, without the additional 'Revolve' at the start".

The spelling comes from `impl Display for SpokenNode` (`crates/editor-core/src/spoken.rs:399-414`, doc at `:275-285`), which writes `{kind} "{label}" ({tag})`. The viewer passes it straight through `tree::node_label` (`crates/viewer/src/tree.rs:713`) and `tree::named` (`tree.rs:722-727`). `tree::headline` (`tree.rs:743-756`) already leads with the label, but only the feature tree uses it (`crates/viewer/src/pane/features.rs:58`). The sites Ev sees include the Properties heading (`crates/viewer/src/pane/properties.rs:513`, `:1389`), the display-section heading (`properties.rs:640`), the View pane's toggles (`crates/viewer/src/pane/view.rs:45`) and the placement editor (`crates/viewer/src/pane/create.rs:645`, `:1023`). There are about 105 `.spoken(` / `named` call sites across the viewer, many inside refusal and notice sentences.

The fix is one person-facing spelling (the label, with the kind only where no label exists or where two nodes share a label), used by every site that names a node to a person. `SpokenNode` is editor-core's, so either it gains that spelling or the viewer owns one beside `tree::headline`. This widens `work/chrome/tree-rows-lead-with-raw-tags.md` (open, P2), which is the same complaint scoped to tree rows.

## The unit (2026-11-22, claimed from VNEWS by OFFER after Ev's decision on #7339)

This row is the kernel half of the decided names design: `## Decided` in `a-node-name-the-viewer-shows-is-not-a-link-to-it`, and DESIGN.md Band 1 "Node labels".

**A sentence is a value.**
- `Say::say` writes into one `Sentence` type: words through `fmt::Write`, mentions through `out.node(..)`, `out.var(..)` and `out.name(..)`.
- A mention carries its id, kind and label, and the document its id is spelled in. The document is stamped from the speaker; `SpokenNode` itself stays 40 bytes.
- The `Speaker` helpers that return a `String` partway through a sentence (`read`, `mate`, `space`, `formula`, `output`) write mentions instead.
- Refusals that capture a `SpokenNode` (`EditError` and others) write the same way.
- `HeldNodes` gains the document it was held from.

**`SpokenNode` and `SpokenVar` lose `Display`.** Only a whole sentence becomes text, and the compiler finds every site that printed a node on its own.

**The spelling.**
- A labelled node is `"base plate" (3fa9c1d2a0b1)`.
- A node with no label is `Extrude 3fa9c1d2a0b1`.
- An id the document does not hold is `node 3fa9c1d2a0b1`.
- One kernel function spells a mention as text. It is used both by the sentence's `Display` and by every flattening.

**The viewer compiles and reads the new spelling.**
- Every viewer site takes the sentence's text for now. The link widget is the next unit, `a-node-name-the-viewer-shows-is-not-a-link-to-it`.
- `tree::named` and `tree::headline` read the kernel's spelling and do not keep their own.

**Re-baselines.**
- `crates/pncad-py/tests/test_document.py`, `test_monte_carlo.py` and `pncad.pyi`.
- Every golden or assert that holds the old `Kind "label" (tag)` form.
- Sweep for the shape, not only for the symbol. State what the sweep could not match.

Moved from `work/vnews/` by `git mv` (OFFER orchestrator, 2026-11-22). Its "kind where two labels collide" suggestion is superseded: the tag is always shown.
