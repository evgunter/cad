---
id: messages-in-the-creation-and-properties-panes-still-draw-past-their-row
kind: issue
title: viewer: fifty refusal, fault and prompt sentences in the creation and properties panes are still drawn by the layout's rule, not the message's
status: closed
opened: 2026-09-22
closed: 2026-09-24
pr: 3139
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

## Two things a conversion here now owes (`chrome/message-floor`)

- **`add_part_ui`'s window needs a `default_width`.** The "Add part"
  window (`pane/create.rs`, `add_part_ui`) is `.collapsible(false)
  .resizable(false)` with no default width, and in an auto-sized
  container `egui::Ui::available_width` is last frame's content — the
  third case `crate::widgets::message`'s doc now lists. So its three
  sentences above (the directory line, the empty-directory sentence and
  the store's refusal) converted there would ask for exactly the width
  they already had and never wrap. The conversion gives the window a
  width, as `crate::app`'s `checks_window` does. (Carried here from
  `an-auto-sized-window-makes-available-width-last-frames-content`,
  closed with the doc half.)
- **The two lists above now wrap by two rules.** A converted site is
  laid out down to `crate::widgets::message_floor` and never inside a
  word; an unconverted one in a top-down layout wraps with no floor, so
  in a narrow pane it becomes a few characters a line and breaks a
  path or a number where no space fits. "Correct today, by the caller"
  above is about where a line STARTS; it is no longer the whole of how
  the sentence reads.

## properties.rs half, done (2026-09-23)

`chrome/properties-messages` routed every sentence in
`pane/properties.rs` through `crate::widgets::message`,
`message_link` or `message_toned`, re-taking the census with the test
above rather than the list.

- **In a row: six, not five.** The five listed above, plus
  `slot_value_ui`'s `{error}`, which the list put under *top-down*. Its
  own body is top-down; both of its callers in `slot_group_ui` draw it
  inside a `ui.horizontal` — the second blind spot above. Per Ev's
  ruling (floor, then scroll, and a sentence that reaches the floor is
  a finding about the layout), none of the six is converted in place.
  Each sentence now has a line of its own under the control, in three
  free functions a headless row can drive: `exists_notice` (the
  already-declared wording, then its `edit` door under it),
  `slot_notes` (the slot's fault — moved out of the field's row and
  named by its slot, as the range reading already was — then the
  affordance, named by its slot too, then its `edit` doors in a wrapping row, then the
  reading) and `bounds_notes` (a parameter's reading, then its
  `range?` button under it). `properties.rs`'s `layout_tests` holds
  one row per function, each red against the old layout.
- **Top-down: thirteen**, the fifteen above less `slot_value_ui`'s
  and less `entity_standing_ui`'s `"{n} rebind candidate(s) offered"`.
  All thirteen converted.
- **Names, left as labels**: `"{n} rebind candidate(s) offered"` (a
  count and a fixed literal, the shape `pane/profile.rs`'s
  `preview_verdict` already argues is a name for `"{n} loop(s), drawn
  in the viewport"`), `"select a feature"`, `"document
  parameters"`, `"add"`, the node-number and `"{noun} of {node}"`
  headers, `"deleted"`, `"instance {id}"`, `"free-move probe (mm,
  display only):"`, and every slot, family, axis, dimension and unit
  label the rows draw.

**Still owed here: `pane/create.rs`'s twenty-seven top-down sites and
its three in-row sites**, which this wave left alone because
`pane/create.rs` is live in AUTHOR's PR 3052. `add_part_ui`'s window
still needs the `default_width` the bullet above asks for, landed
with its three sentences.

The sweep that found `slot_value_ui` also found a field that is not a
message: a driven slot's value field shows its expression's source,
unbounded, in the same row. Filed as
`a-driven-slots-field-draws-its-expression-source-at-any-width`.

## create.rs half, done (2026-09-24)

`chrome/create-messages` routed every sentence in `pane/create.rs`
through `crate::widgets::message`, `message_toned` (every `ui.weak`
became `Tone::Advisory`) or plain `message` (every `ui.label`),
re-taking the census by subject with the test above. The line numbers
above had drifted and three of the subjects were misattributed; the
list below is by function.

- **In a row: two, not three.** `frame_picker`'s empty arm (now
  `NO_FRAMES`, said on a line of its own under the picker's label) and
  `add_part_ui`'s per-entry id (now `part_entry`, a free function: the
  pick button, then the 32-digit id under it). The third,
  `BlendTarget::of_face(face)` in the face row, is in
  `datum_face_frame_rows`, not `blend_tool_ui`, and is a NAME:
  `"feature {n} body {m}"` is a fixed literal and two integers. It
  stays beside its `face` label. `create.rs`'s `layout_tests` holds a
  row per in-row site, each red against the old layout.
- **Top-down: twenty-five**, the twenty-seven above less two that are
  NAMES by the same count rule the properties half used for
  `"{n} rebind candidate(s) offered"`: `mate_tool_ui`'s one-pick line
  (`"pick a: face of node {n}"`) and `blend_tool_ui`'s picked-count
  line (`"{count} edges picked on {target}"`, whose target is the
  two-integer name above). The mate tool's two-pick line stays a
  sentence, because it is a joined pair, as `seat_line` is. The
  "sketch-frame note" listed under `blend_tool_ui` is
  `datum_face_frame_rows`'s. All twenty-five are converted.
- **Names, left as labels**: `"no picks yet"`, `"no directory"`,
  `"none picked"`, `"no edges picked yet"`, the two above, every form
  label, every radio and checkbox label from a closed set, and every
  fixed button label including `"Extrude {node}"`.

**The window width.** `add_part_ui`'s window is now built by
`part_window`, which gives it the width of the pane that opened it
(`egui::Ui::available_width`, never under `message_floor`). The premise
that carried this bullet here did not hold as stated. Measured on
egui 0.36.1: `egui::Window::new` begins at egui's own 340-point default
size, so with no `default_width` the chooser's sentences wrapped at 326
points from the first painted frame. `egui::Resize::begin` ratchets UP
to last frame's content and never back. So the wrap did fire, at a
width egui chose, and a wider row (a long file name on a pick button)
widened the window instead. `widgets::message`'s doc now says that.
`the_part_choosers_sentences_wrap_at_the_opening_panes_width` holds the
opener's width, red against the window with no width (its sentence
ran 62 points past a 260-point opener).

Filed from the sweep: `a-part-choosers-pick-button-draws-a-file-name-at-any-width`
and `a-frame-pickers-closed-combo-draws-a-frames-pose-in-an-extend-row`.
The sentences AUTHOR's PR 3052 adds to `create.rs` (`part_selector_rows`'
two notes, `part_tool_ui` and `duplicate_tool_ui`'s prompts, seat lines
and `duplicate_note`) were not on this branch and were not chased. They
are that PR's to route through `message`.
