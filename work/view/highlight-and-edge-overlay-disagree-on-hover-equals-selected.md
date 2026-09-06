---
id: highlight-and-edge-overlay-disagree-on-hover-equals-selected
kind: issue
title: the two mark halves call themselves twins but only one dedupes a hover that is already the selection
status: open
opened: 2026-09-06
refs: [2083]
---


Found by the style review of #2083 (pre-existing; both members moved
into `crates/viewer/src/marks.rs` together, so the divergence is now on
one screen for the first time).

`crates/viewer/src/marks.rs:200` calls `edge_overlay` *"the twin of
[`highlight`]"* and says *"what the two share is the RULE"*. They do
share the narrowing rule. They do not share what happens when the hover
IS the selection:

- `edge_overlay` (`marks.rs:224-227`) filters it out in Rust —
  `hovered_edge = hover.and_then(Hovered::edge).filter(|edge| selected_edge != Some(*edge))`
  — so `EdgeOverlay::hovered` is empty for an edge already selected;
- `highlight` (`marks.rs:118-121`) does not — `Highlight::selected` and
  `Highlight::hovered` carry the same patch id when a hover lands on the
  selection, and the precedence is resolved downstream in the shader.

`edge_overlay`'s doc states the divergence and justifies it (*"the
precedence the shader's face path already states"*), which is honest.
What is left is that two values a consumer reads together answer *"is
this hovered?"* differently for the same gesture: a test asserting
`highlight.hovered == IdMap::NOTHING` for a hover on the selection
would be wrong, and one asserting the equivalent of `EdgeOverlay` would
be right, with nothing in the types saying which convention applies.

`crates/viewer/src/gpu.rs` consumes both. If the face path's shader
precedence ever changes, `Highlight`'s two fields become a pair that
nothing resolves, and the only record of the intended invariant is a
sentence in `edge_overlay`'s doc about a different function.

## Where else to look

Any other selected/hovered pair in the crate:
`crate::session::Hovered` against `Selection`, and `crate::theme`'s
`selected`/`hovered` marks, which are composited by
`Mark::over` in an order that assumes one answer.

## Confidence

`likely`. Nothing is broken today and the asymmetry is documented; the
finding is that a "twin" relation stated in prose is carrying an
invariant the types do not.
