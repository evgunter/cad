---
id: resolution-and-standing-pick-their-tone-by-hand
kind: issue
title: The properties pane picks weak-or-coloured by hand from Resolution and Standing
status: open
opened: 2026-09-19
---



Found by the sweep of `tone-is-a-value-in-frame-and-a-comment-in-two-
panes` (the class: *the actionable-or-not rule spelled somewhere other
than as a `frame::Tone`*). Not taken there, because its answer is a
different decision — see below.

**The sites**, all in `crates/viewer/src/pane/properties.rs`:

- `entity_standing_ui`'s resolution arms (~`:349-373`): `None` draws
  `ui.weak("no evaluation yet to resolve this against")`,
  `Resolution::Failed` and `Resolution::Indeterminate` draw
  `ui.colored_label(chrome(self.theme.unresolved), …)`, and the
  rebind-candidate count under a failure draws weak again. That is the
  actionable-or-not rule exactly — *a report nobody can act on yet*
  against *a verdict a reader must rebind* — picked per arm.
- `standing_ui` (~`:291`, `:297`): a deleted feature and an undeclared
  parameter each take the colour, hand-picked at the call.

**Why it is a different decision from the tree row.** `RowStatus` is
the viewer's own type, so `tone()` lives on it. `Resolution`,
`ResolutionFailure` and the cause type are `pncad::select`'s — the
kernel's — and the kernel has no reason to know how loud a viewer
draws a verdict. So this row has to decide where a tone for a kernel
value lives: a free function in the viewer keyed on the kernel type, a
viewer-side wrapper the pane builds first, or `Standing` (which IS the
viewer's, `crates/viewer/src/props.rs`) growing the tone and the
resolution arms folding into it.

**Not the same row as `a-disabled-control-says-why-in-four-shapes`**,
which reaches `properties.rs:352-357` on the adjacent axis: that row is
about which WORDS a reader gets and where they come from, this one is
about how loudly they are drawn. A fix to either leaves the other
standing.

`properties.rs` is VNEWS's, VGEOM's, CHROME's and VIEW's.
