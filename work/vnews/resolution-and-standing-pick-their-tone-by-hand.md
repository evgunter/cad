---
id: resolution-and-standing-pick-their-tone-by-hand
kind: issue
title: The properties pane picks weak-or-coloured by hand from Resolution and Standing
status: open
opened: 2026-09-19
priority: P1
cost: E
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

## Widened at the tone unit's fix pass (2026-09-20)

The original sweep here was over the **coloured** half —
`colored_label` / `.color(chrome(` and the `ui.weak` sibling each is
chosen against. A review sweep of a different shape found members that
grep structurally cannot see, so the rule is restated and the
population grows.

**The sweep rule**, which is about the PROPERTY and not the call: every
site under `crates/viewer/src` that renders a typed refusal or verdict
and **chooses its salience at the call** — a coloured label, a `weak`
one, or a plain `ui.label` — rather than reading a `frame::Tone` from
the value. Run as the union of `colored_label`, `.color(chrome(`,
`ui.weak(<typed value>.to_string())` and `ui.label(<typed
value>.to_string())`, each read for whether an alternative salience was
available at that site.

**What the coloured-only pattern could not match, and now does:**

- `pane/create.rs`, the store/directory refusal arm (~`:308-310`) —
  `ui.label(refusal.to_string())`, **a tone decision by OMISSION**.
  Neither weak nor coloured, so it takes the body colour, and nothing
  at the site or on the value says whether that was chosen. A grep over
  either salience call is blind to the site that made neither call.
- `pane/properties.rs`, the free-move probe (~`:399`) —
  `ui.weak(fault.to_string())` over a typed `DisplayFault`. (The same
  line is `a-disabled-control-says-why-in-four-shapes`'s on the WORDS
  axis; this row is the salience.)
- `pane/properties.rs` (~`:485`, ~`:508`) — `ui.weak` over a
  dimension's `Display`. These two are the control case: a dimension is
  not a refusal at all, so `weak` is secondary text rather than a tone,
  and they are members of the pattern and **not** of the class. Kept
  here because a sweep that silently drops its non-members cannot be
  re-run.

The decision the row states is unchanged and now covers more: these
values are `pncad`'s and `crate::display`'s, so where a tone for a
non-viewer value lives is still what has to be settled first.
