---
id: a-vertex-sited-blend-refusal-names-no-site-in-the-editor
kind: issue
title: A blend refusal sited at a vertex names no site in the editor: BlendError::edge() is None for it
status: open
opened: 2026-11-12
priority: P2
cost: E
---


Found by the UnbuiltEnd designers (2026-11-12).

The editor reads one thing from a blend refusal: `BlendError::edge()`, which becomes `NodeErrorKind::Blend.edge` (`crates/editor-core/src/eval/wire.rs`). It is `None` for every vertex-sited refusal, so seven of the eight `UnbuiltEnd` tags reach the user with no site named. That is every corner refusal today. The viewer only matches the variant (`crates/viewer/src/tree.rs`).

The fix is a vertex name on `NodeErrorKind::Blend`, or a site enum (edge | vertex), so the editor can say "refused at <vertex>". It lands after the vocabulary cut, whose payloads carry the vertex.
