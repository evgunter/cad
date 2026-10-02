---
id: viewer-panes-speak-the-kernel-refusals-they-draw
kind: unit
title: The viewer's panes and frame speak the kernel refusals they draw (Standing's verdict, the indeterminate line, a part's carried level and fault)
status: open
opened: 2026-10-02
priority: P2
cost: M
parent: node-labels-are-document-data
refs: [viewer-refusals-speak-the-node]
---


Split from `viewer-refusals-speak-the-node`. The kernel types that gained `spoken(doc)`/`Say` in PRs 3760 and 3782 are still drawn by their `Display` (tags) in these viewer reads. Each speaks from the document its ids are spelled in, fixed by the site.

- **The selection's verdict** (`pane/properties.rs`, `standing_verdict`): `Resolution::Failed`'s `ResolveError` ("this {noun} is gone: {}") and `Resolution::Indeterminate` through `app::indeterminate_wording` (`ResolveIndeterminate`). `Standing` is recomputed every frame (`DocSession::standing`) against the landed evaluation, so it speaks live from the landed document; `standing_verdict` and `indeterminate_wording` take it. `Standing` itself prints no id.
- **A part's carried level and fault** (`tree.rs`): the tree speaks a carried level with `CarriedLevel::line_in(doc)`; a level and a `PartFault` from inside a part have `line_in_part(part, tol)` and `PartFault::spoken(doc_ref, part, tol)`, which no frame calls yet. The tree holds the part files (`PartFiles`); where it holds the resolved part at the pinned version, speak through them, else the tag stands (PR 3782's pinned-part rule).
- **The rest of the reads** the parent row listed in `frame.rs`, `pane/create.rs`, `pane/viewport.rs` and `app.rs`: sweep each format of a kernel refusal value there (`{error}`, `{fault}`, `.to_string()`) for a type that is `Say`, and speak it from the site's document. Not every read prints a node.

`viewer-pick-path-refusals-speak-the-node` takes the pick index's types; `viewer-product-badge-speaks-the-node` takes the gather's refusal.
