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

**The pane.** Every arm's words are drawn by one free function,
`pane::properties::standing_verdict`, exhaustive over `Standing` and
reading its tone once off `standing.tone()`: the deleted node's one
word, the undeclared parameter's sentence, and a picked entity's
resolution verdict with its rebind count. The entity's noun is read off
its own arm, so no caller hands it one. `standing_ui` draws the header
line (number, delete button, the "face of" label) and calls it.

**A fact is said once in this pane** — the principle `failure_lines`
keeps for a failed tree row (loud at the badge, the words under it
quiet) and `instance_ui` keeps for a vanished instance (silent under
the header that said so). The parameter panel's own `"that parameter
is gone"` line was one fact in one frame in two tones, `Actionable`
above and `Advisory` below; it is **gone**, not re-toned, since the
header already says it loud. For the same reason a **deleted** node no
longer draws *"this feature carries no parameters"* under its loud
`deleted` — that sentence was false (the node carries nothing because
it is not there) and is now gated on `standing.live()`, as the pick arm
already was.

**The widened population, disposed.** The rule
(`a-tree-rows-message-line-picks-its-affordance-by-hand`) is that the
value owes the tone wherever the tone varies across its arms; where a
site admits one arm only, the literal stays, **and the literal is still
checked**.

- `pane/create.rs`, the part chooser: **the value carries it**,
  `parts::PartChooser::tone`. An empty listing (the open document's own
  file has gone from its directory) and a refusal are `Actionable`, a
  listing is `Advisory`; the chooser's body is the free
  `pane::create::part_listing`. The header draws nothing for a chooser
  with no directory, since the refusal under it says exactly that — the
  old quiet `"no directory"` was the same fact in the other voice.
- `pane/create.rs`, the datum form's face-frame fault: **the value
  carries it**, `session::FaceFrameFault::tone`. A fault about the pick
  (`NotOneBody`, `NotPlanar`, `Unresolved` — the last the same stale
  pick `Standing::Face` draws loud in the header) is `Actionable`; a
  seat not yet answerable (`NoFace`, `NotLanded`) is `Advisory`.
- `pane/create.rs`, the add-profile form's held reason: **typed**,
  `pane::create::Held`. It was one `&'static str` for three prompts
  (`Advisory`) and one refused input — a bore at least as wide as the
  radius — which is `Actionable`.
- `pane/create.rs`, the mate tool's admission line: **left**. It reports
  the admission table's verdict on a class, which asks nothing of the
  reader.
- `pane/properties.rs`, the free-move probe's fault and `hide_toggle`'s:
  **left, with the reason at each site, and the literal re-checked**.
  Past the section's kind gate the only faults that arrive are a mate
  placing the instance and geometry fused with another's — the document
  as written, which asks nothing of the reader.
- `pane/properties.rs`, `slot_group_ui`'s two `ui.weak` over a
  dimension: secondary text, not a tone.

**Receipts.** Every row reads the colour off the paint
(`pane::headless::Landed::ink`) and holds it against fixed colours
(`pane::headless::Voices`: `Theme::DEFAULT.unresolved` and egui's own
weak text), through the free function the pane calls — so a literal put
back at a draw site goes red as well as a wrong value:
`pane::properties::verdict_tests` (a vanished face and its offer count,
an indeterminate edge, no evaluation, a deleted node, an undeclared
parameter, and silence for a standing that still denotes) and
`pane::create::tone_tests` (the chooser's three answers, the face-frame
faults, the held reason). `session::select::tests` holds
`Standing::tone`'s node and parameter arms against fixed `Tone`s. The
mutation runs are named in PR #3230.
