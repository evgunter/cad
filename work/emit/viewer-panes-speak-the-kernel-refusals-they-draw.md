---
id: viewer-panes-speak-the-kernel-refusals-they-draw
kind: unit
title: The viewer's panes and frame speak the kernel refusals they draw (Standing's verdict, the indeterminate line, the sweep of the rest)
status: closed
pr: 3827
opened: 2026-10-02
closed: 2026-10-02
priority: P2
cost: M
parent: node-labels-are-document-data
refs: [viewer-refusals-speak-the-node]
---


Split from `viewer-refusals-speak-the-node`. The kernel types that gained `spoken(doc)`/`Say` in PRs 3760 and 3782 are still drawn by their `Display` (tags) in these viewer reads. Each speaks from the document its ids are spelled in, fixed by the site.

- **The selection's verdict** (`pane/properties.rs`, `standing_verdict`): `Resolution::Failed`'s `ResolveError` ("this {noun} is gone: {}") and `Resolution::Indeterminate` through `app::indeterminate_wording` (`ResolveIndeterminate`). `Standing` is recomputed every frame (`DocSession::standing`) against the landed evaluation, so it speaks live from the landed document; `standing_verdict` and `indeterminate_wording` take it. `Standing` itself prints no id.
- **The rest of the reads** the parent row listed in `frame.rs`, `pane/create.rs`, `pane/viewport.rs` and `app.rs`: sweep each format of a kernel refusal value there (`{error}`, `{fault}`, `.to_string()`) for a type that is `Say`, and speak it from the site's document. Not every read prints a node.

`viewer-pick-path-refusals-speak-the-node` takes the pick index's types; `viewer-product-badge-speaks-the-node` takes the gather's refusal. A part's carried level and its `PartFault` (`line_in_part`, `PartFault::spoken`) are `a-frame-that-speaks-a-parts-refusal-holds-no-resolved-part`: no frame holds a resolved part, so they keep their tags, and how one comes to hold it is a design question.

**Built (PR 3827).** `standing_verdict` and `indeterminate_wording` speak from the landed document. The rest of the sweep found no other kernel `Say` value drawn by tag in `frame.rs`, the panes or `app.rs`; the PR body carries the hit list.
