---
id: resolution-and-standing-pick-their-tone-by-hand
kind: issue
title: The properties pane picks weak-or-coloured by hand from Resolution and Standing
status: closed
opened: 2026-09-19
branch: vnews/salience-read-from-the-value
pr: 3230
closed: 2026-09-25
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

## Decided (2026-09-25): the tone lives on `Standing`

**`Standing::tone()`** (`crates/viewer/src/session/select.rs` — the
type has moved there from `props.rs`), total like `RowStatus::tone`:
`Actionable` for a deleted node, an undeclared parameter, a name that
failed to resolve and one the evaluation could not answer for;
`Advisory` for nothing selected, a live selection, and a pick with no
evaluation behind it yet. The match is exhaustive over the kernel's
`Resolution`, so a verdict the kernel grows has to answer it here.

**Why this home and not the other two.**

- *A free function keyed on `Resolution`* reaches the resolution arms
  only. The deleted-node and undeclared-parameter arms are not
  `Resolution`s, so they would need a second home for the same rule,
  and the "no evaluation yet" arm is not a `Resolution` either — it is
  `Standing`'s own `Option`.
- *A wrapper the pane builds first* already exists: it is `Standing`.
  The session builds it from the kernel's verdict, and `live()` and
  `unresolved()` are two readings of that verdict for the chrome. A
  second wrapper would be a second copy of the same interpretation.
- Nothing enters `pncad`: the kernel's verdict stays the kernel's, and
  how loud a viewer draws it is decided by the viewer's own value.

**The pane.** `standing_ui`'s node arm draws `toned("deleted", …,
standing.tone())`; its parameter arm and the parameter panel's own
`"that parameter is gone"` both read `standing.tone()` — those two were
one fact in one frame drawn in two tones, `Actionable` above and
`Advisory` below (the words half stays
`three-spellings-say-a-parameter-is-not-declared`'s). The resolution
arms moved into `pane::properties::entity_verdict`, a free function the
headless harness reaches: it composes the words per arm from the
verdict's payload and draws them once, in `standing.tone()` — the rule
`a-tree-rows-message-line-picks-its-affordance-by-hand` states.

**The widened population, disposed.**

- `pane/create.rs`, the part chooser's scan refusal: **fixed**, by a
  tone stated at the site. It was body colour by omission; it is now
  `Tone::Actionable`, once, with the reason at the site — every refusal
  that reaches the arm (`Refusal::NoDocumentDirectory`,
  `Refusal::Workspace`, `DocSession::part_catalogue`'s two) leaves the
  chooser with nothing to offer until the reader saves or repairs the
  directory and rescans. A literal and not a value, because this site
  varies no tone across arms.
- `pane/properties.rs`, the free-move probe's fault, and `hide_toggle`'s:
  **left, with the reason at each site**. Both are single-tone:
  past the section's kind gate the only faults that arrive are a mate
  placing the instance and geometry fused with another's, which are the
  document as written.
- `pane/properties.rs`, `slot_group_ui`'s two `ui.weak` over a
  dimension: the control case, as before — secondary text, not a tone.
- `entity_verdict`'s rebind count under a failure: secondary text under
  a verdict that carries the tone; the site says so.

**Receipts.** `pane::properties::verdict_tests` reads the colour off
the paint (`pane::headless::Landed::ink`, added here) and holds it
against fixed colours — `Theme::DEFAULT.unresolved` and egui's own weak
text — for a vanished face (and its offer count), an indeterminate
edge, and a pick with no evaluation. Planting `Failed(_) =>
Tone::Advisory` in `Standing::tone` reds the first.
`session::select::tests` holds the node and parameter arms, which no
headless drive reaches, against fixed `Tone`s; planting `Advisory` for
a vanished node or parameter reds it. (The mutation run is named in
PR #3230.)
