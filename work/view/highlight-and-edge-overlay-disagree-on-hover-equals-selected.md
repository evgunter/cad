---
id: highlight-and-edge-overlay-disagree-on-hover-equals-selected
kind: issue
title: the two mark halves call themselves twins but only one dedupes a hover that is already the selection
status: closed
branch: view/mark-twins
opened: 2026-09-06
closed: 2026-09-15
refs: [2083]
pr: 2625
---


Found by the style review of #2083 (pre-existing; both members moved
into `crates/viewer/src/marks.rs` together, so the divergence is now on
one screen for the first time).

`crates/viewer/src/marks.rs:187` (at filing) calls `edge_overlay` *"the twin of
[`highlight`]"* and says *"what the two share is the RULE"*. They do
share the narrowing rule. They do not share what happens when the hover
IS the selection:

- `edge_overlay` (`marks.rs:211-214`) filters it out in Rust —
  `hovered_edge = hover.and_then(Hovered::edge).filter(|edge| selected_edge != Some(*edge))`
  — so `EdgeOverlay::hovered` is empty for an edge already selected;
- `highlight` (`marks.rs:105-108`) does not — `Highlight::selected` and
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


## Closed

Closed by documentation and one test row; no behaviour changed. The
three citations above were re-read at `e4dcc5c75a` and all three named
their subject.

**The finding is right and the divergence is not a defect.** The
mechanism check is what settles it. `edge_overlay`'s filter was
justified as *"the precedence the shader's face path already states"*,
which reads as an echo of a ruling made downstream. It is not one: the
edge path has nowhere downstream to make that ruling. An edge vertex
carries exactly one mark word — `EDGE_MARK_SELECTED` is the ABSENCE of
`EDGE_MARK_HOVERED` (`crates/viewer/src/gpu.rs`), so no vertex can mean
both — and `EdgePass::ensure_geometry` writes the selected lane and then
the hovered lane into one buffer for one draw, under a pipeline with
`blend: None`, `depth_write_enabled: false` and
`depth_compare: LessEqual`. Identical geometry drawn second therefore
overwrites. Without the Rust filter the edge path would resolve
HOVER over selection, the opposite of `fs_main`'s `else if`.

So the two halves obey one rule and each applies it at the last place
that can see both answers: for faces that is the fragment, which has
both uniform lanes; for edges it is Rust, because by the time the GPU
sees a vertex the choice has already been made. Making `highlight`
dedupe as well would have removed the face path's only arbitration —
`Highlight`'s fields are public and `crate::marks` is not its only
possible producer — and left the shader's stated precedence with no
reachable input, which is this program's own silently-never-fires
shape.

What was actually wrong was the record. The invariant lived in prose
about a different function, and `EdgeOverlay::hovered`'s own field doc
(*"the hovered edge's segments"*) described the convention the function
does not implement. Both types now state their own convention where a
reader meets it, and `edge_overlay`'s justification names the mechanism
instead of borrowing the face path's.

The sweep for other selected/hovered pairs found no third member:
`theme`'s `selected`/`hovered` are palette entries, always both
present and never an answer about one gesture, and `Mark::over`
composites one mark at a time rather than ordering a pair.
`session::Selection` against `DocSession::hover` is the SOURCE of the
pair and is deliberately un-narrowed — the two answer different
questions. Residue: none.
