---
id: messages-in-the-creation-and-properties-panes-still-draw-past-their-row
kind: issue
title: viewer: fifty refusal, fault and prompt sentences in the creation and properties panes are still drawn by the layout's rule, not the message's
status: open
opened: 2026-09-22
priority: P0
cost: D
refs: [error-and-check-text-overflows-its-region]
---


`crates/viewer/src/widgets.rs`'s `message`, `message_link` and
`message_toned` are the one home for *a sentence drawn inside the
region it is drawn in*, added by the layout half of
`error-and-check-text-overflows-its-region`. The sites below were NOT
routed through it, for one reason only: `pane/create.rs` and
`pane/properties.rs` were live in three other lanes' open PRs (3052,
2960, 2961) in the wave that added it. Nothing about them is harder
than the sites that were converted.

## The test this census was taken with

The first version of this row said *eleven* and listed thirteen, and
thirteen was itself an undercount, because no test separated a
"sentence" from a "name" — each site was classified by hand. The test,
written down so the list below can be argued with site by site:

> **A text is a NAME when its drawn width is bounded by something this
> crate controls**: a fixed literal short enough to read at a glance, a
> value from a closed set this crate enumerates (`SlotKind::label`,
> `BlendKindChoice::ALL`, an enum's `symbol()`), or a number
> `readout::MAX_CHARS` already bounds. **Everything else is a
> SENTENCE**: prose long enough that a narrow pane cannot be assumed to
> hold it, or any text with a part interpolated from a value this crate
> does not bound — a user-authored parameter name, a filesystem path, a
> kernel `Display`, a joined list.

Two consequences worth stating, because both change the answer:

- **The test is about WIDTH, not about provenance or about whether the
  widget is a control.** `ui.link` and `ui.selectable_label` lay a
  galley out exactly as `ui.label` does. So the three `ui.link` sites
  the first census classified as names — `pane/properties.rs`'s
  document-parameter list (`:136`) and the two `edit {name}` links
  (`:287`, `:792`) — are sentences under it: a parameter name is
  authored by the user and nothing bounds it. They are listed below.
- **The soft edge is "short enough to read at a glance"**, and it is
  soft. `"no picks yet"`, `"no directory"`, `"none picked"`,
  `"structural"`, `"select a feature"` and `"free-move probe (mm,
  display only):"` are all called names here — fixed literals, none of
  which a narrow pane cannot hold. A reader who disagrees can move any
  one of them; what the test buys is that the disagreement is about a
  named site rather than about a count.

Fifty sites, split by what the layout does with them today.

## Drawn in a non-wrapping horizontal row — these run past the edge

egui gives a label in `ui.horizontal` `TextWrapMode::Extend`, which
lays the text out at infinite width, so a sentence longer than the pane
is drawn past its right-hand edge rather than wrapped or shortened.
Eight sites:

- `pane/create.rs`, `picker`'s empty arm (`:64`) — *"none in this
  document — add a frame datum first"*, in the row that holds the combo.
- `pane/create.rs`, `add_part_ui`'s per-entry id (`:368`) — a document's
  own part id beside its pick button.
- `pane/create.rs`, `blend_tool_ui`'s picked-face wording (`:564`) —
  `BlendTarget::of_face(face).to_string()`.
- `pane/properties.rs`, `add_param_ui`'s existing-name arm (`:286`) —
  `Refusal::exists_wording`, beside its `edit <name>` link.
- `pane/properties.rs`, `add_param_ui`'s `edit {name}` link (`:287`).
- `pane/properties.rs`, `slot_notes_ui`'s `Refusal::affordance`
  (`:787`). The longest of the eight: its wording grows with the
  expression's parameter list.
- `pane/properties.rs`, `slot_notes_ui`'s per-parameter `edit {name}`
  links (`:792`), one per parameter in that list.
- `pane/properties.rs`, the bounds row (`:865`) —
  `BoundsReading::wording`, beside the `range?` button.

## Drawn top-down in a pane function — correct today, by the caller

These sit directly in a `*_ui` body (or a window body), which its pane
calls from a top-down layout, where egui's default IS a wrap at the
pane's width. They are not drawn wrongly; what they lack is any reason
they could not be. The rule is the caller's layout rather than the
message's kind.

`pane/create.rs`, twenty-seven:

- `mate_tool_ui`: the pick prompt `ToolKind::Mate.says(…)` (`:191`),
  the one-pick and two-pick state lines (`:197`, `:200`), and the
  admission verdict `format!("admission: {}", …no_record_reason())`
  (`:221`).
- `add_part_ui`: the directory line `format!("parts in {}",
  dir.display())` (`:332`), the empty-directory sentence (`:344`, two
  clauses long and the longest fixed literal in the file), and the
  store's own `refusal.to_string()` (`:376`).
- `add_datum_ui`: the two standing notes (`:444`, `:485`),
  `kind.unmet_seat()` (`:509`), and `fault.to_string()` (`:516`).
- `blend_tool_ui`: the sketch-frame note (`:581`), the pick prompt
  (`:1126`), `FREEZE_NOTE` (`:1127`), and the picked-count line
  (`:1128`).
- `add_profile_ui`: `reason` (`:784`).
- The five remaining tool forms, each a pick prompt plus a
  `seat_line(…)` whose length grows with the seat list — revolve
  (`:893`, `:894`), boolean (`:926`, `:927`, plus the subtract note at
  `:940`), split (`:956`, `:957`), transform (`:973`, `:974`) and
  pattern (`:1025`, `:1026`).

`pane/properties.rs`, fifteen:

- `properties_ui`: *"this feature carries no parameters"* twice
  (`:41`, `:54`), the parameter header `format!("parameter {} ({})", …)`
  (`:72`), *"that parameter is gone"* (`:129`), and the
  document-parameter links (`:136`).
- The `arguments` collapsing header's blurb (`:162`) — three clauses,
  the longest sentence in either file.
- `add_param_ui`'s `Refusal::offer_wording` (`:205`).
- `standing_ui`'s *"parameter {} is no longer declared"* (`:371`).
- `entity_standing_ui`'s four verdicts: the no-evaluation caveat
  (`:427`), *"this {noun} is gone: {error}"* (`:431`), the rebind-offer
  count (`:436`), and `indeterminate_wording` (`:443`).
- `instance_ui`'s `fault.to_string()` (`:490`).
- `slot_value_ui`'s `{error}` (`:628`).
- `slot_notes_ui`'s SECOND sentence, `format!("{}: {}",
  row.slot.label(), reading.wording())` (`:811`) — which falls between
  the two `slot_notes_ui` sites the first census listed and was missed
  by both.

`pane/profile.rs`'s `preview_verdict` was in exactly this class and was
converted, because it is a free function with two callers in two
different forms — the argument applies to the rest unchanged.

## What the sweep could not see

The pattern was `ui.{label,weak,colored_label,small,link,monospace,
strong,selectable_label}(` over the two files, with the enclosing
layout derived by brace-depth tracking. Three blind spots:

- **Text drawn through something other than those calls** — a
  `ui.add(egui::Label::new(…))`, a hover text, a button's own label. A
  second pass for `egui::Label::new` and `on_hover_text` over both
  files found no sentence in a row (hover text is its own window and
  wraps there), but a widget helper that draws text internally would
  not be visible to either pass.
- **Layout derived statically.** A `*_ui` body called from inside a
  caller's `ui.horizontal` would be misclassified as top-down. The
  brace tracker sees the callee's own text, not its caller's — which is
  exactly the reason `preview_verdict` was converted rather than
  classified.
- **The other files.** This row is scoped to the two panes; the sweep
  was not run over `pane/viewport.rs` or `pane/view.rs`. One site
  outside both is already filed:
  `feature-tree-row-labels-draw-an-unbounded-pose-in-an-extend-row`.

## What a fix owes

`widgets::message_tests` measures the rule (a message's lines stay
inside the region, every line begins under the first, and the widest
line grows with the region rather than sitting at a constant) against a
region laid out in a real `egui::Context`; `app`'s
`the_toolbars_status_line_wraps_under_itself_rather_than_at_the_windows_edge`
measures it in the real toolbar. Nothing measures it at these call
sites, because `create_ui` and the properties rows hang off
`ViewerBehavior` and `crate::pane::headless` cannot drive a pane method
(`headless-egui-harness-spelled-five-times` carries that limit). The
eight sites in the first list are free functions or could be made ones,
which is the cheap route to a row that can go red.

**Read `messages-wrapped-at-a-region-and-numbers-bounded-by-characters-are-two-answers`
first.** It asks whether wrapping at the region is the right answer for
these sites at all, or whether some of them want the character bound
`readout::number_text` already applies — which is also the concision
half of `error-and-check-text-overflows-its-region`. Converting fifty
sites to the width answer before that question is settled is fifty
sites to revisit.
