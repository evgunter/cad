---
id: a-disabled-control-says-why-in-four-shapes
kind: issue
title: A control a reader cannot use owes the sentence a click would have been answered with — the rule, and the three sites that owe it
status: open
opened: 2026-09-11
refs: [environmental-facts-answer-usable-as-a-bool-with-the-reason-elsewhere, the-new-document-button-states-its-refusal-twice, the-range-button-re-mints-the-ratified-affordance, undo-and-redo-are-disabled-in-silence-over-a-refusal-that-has-words, two-pickers-spell-one-not-well-typed-sentence-twice, a-disabled-controls-reason-has-one-home]
---

Filed 2026-09-11 by the review of the cancel-door unit (#2320) as a
census of four spellings. **Censused 2026-09-19 at merge base
`2654cc111417da806d9786c40136106469096fec`**, and rewritten as the
answer. The filing's counts and every one of its citations are
superseded: the tree moved, `pane/profile.rs` and `widgets.rs` grew
members the filing does not name, and **neither of the two sites the
filing predicted would be the only genuine hits is one.**

## The rule

**A control a reader cannot use owes the sentence a click would have
been answered with — when there is such a sentence.** The test is not
"is there a refusal type in scope" and not "does this call
`on_disabled_hover_text`". It is one question asked at the control:

> If the reader got past this gate and the operation ran, what sentence
> would come back?

- **A sentence would come back.** Then the pre-click sentence and the
  post-click sentence are *the same sentence*, and they get **one
  composition** — read off the refusal value, or off a wording helper
  the refusal's own `Display` also calls. Minting a second is the
  defect; saying nothing is the same defect with the second copy empty.
- **Nothing would come back**, because no operation can be formed yet —
  the control gates a DRAFT, not a door. Then there is no sentence to
  be the same as, a literal at the control is correct, and the only
  obligation is that it be true.

### This rule is already ratified, three times, in `refuse.rs`

It is not minted here. `Refusal::self_instance` exists so that *"the
predicate has one home and the chrome's disabled reason is the same
value the click would have been answered with"*; `Refusal::affordance`
because *"two independently-built copies is how the wording drifts from
the decision"*; `Refusal::exists_wording` so that *"the pre-click notice
and the refusal cannot drift apart"*. Three helpers, one argument,
written by three different units. The census's contribution is to state
the rule the helpers already obey and to find the controls that do not.

### What the rule does NOT say

It does not say every disabled control owes prose. `add_enabled(free &&
index > 0, …)` on a move-up arrow (`pane/profile.rs`) gates a row
reorder that cannot be formed at index 0; there is no op, no refusal,
and no sentence is owed. And it does not say a reason must ride on
`on_disabled_hover_text`: `pane/properties.rs`'s free-move probe draws
`fault.to_string()` **where the control would be** and is the tree's
best-obeying member.

## Population, and the rule that produces it

The property is **a control drawn as unusable, together with whatever
the reader is told about why**. Three passes over `crates/viewer/src`,
unioned, each read at the line:

- **P1 — `add_enabled` / `add_enabled_ui`: 30 call sites.** (32 grep
  lines minus the two that are prose inside `pane/profile.rs`'s
  `add_enabled`-vs-`add_enabled_ui` comment.) By file: `widgets.rs` 8,
  `pane/profile.rs` 8, `app.rs` 6, `pane/create.rs` 6,
  `pane/properties.rs` 2.
- **P2 — `on_disabled_hover_text`: 11 call sites** (14 grep lines minus
  three doc comments, in `platform.rs`, `session/op.rs` and
  `pane/profile.rs`). **Every one of them attaches to a P1 site, so P2
  contributes no member P1 did not already have.** That is the proxy
  table's eighth row, measured: the call-shaped rule is a strict
  subset, and it is the subset that silently drops the silent controls.
- **P3 — a branch that draws a sentence in the place a control would
  occupy: 5 stand-alone sites.** Derived by reading all 61 `.weak(` /
  `colored_label(` sites and keeping those that stand where a control
  would be rather than beside one. Five more `.weak` sites are the
  reason-half of a P1 member (`pane/create.rs`'s `blocked`, its datum
  notice, `forms::SHAPE_LOCKED`, `pane/profile.rs::preview_verdict`)
  and are classified with their control, not counted twice.

**35 members** (30 + 5). There is no fourth pass: `ui.set_enabled`,
`ui.disable()` and a `.enabled(` builder appear nowhere in the crate,
and the four `Sense::` hits are viewport allocations and an `AxisSense`
enum.

## What this pattern could not match

Stated as the instrument requires, before the hit list rather than
after it.

- **A control that is simply not drawn.** A tool panel that returns
  early, a row omitted from a list, a `CollapsingHeader` never opened —
  a reader cannot use it and is told nothing, and no pass above can see
  an absence. `session/op.rs`'s seat tables are the likely population
  and a different census.
- **A control drawn as usable that refuses on click.** The rule's
  mirror image. `pane/create.rs`'s "Add profile" reaches an
  unreachable-in-practice arm that pushes tool news instead, which is a
  post-click sentence with no pre-click one.
- **egui's own greying.** A widget inside a `Ui` whose ancestor was
  disabled by a caller several frames of code away is greyed without
  any P1 call at the widget; P1 finds the `add_enabled_ui` but not what
  it wraps, so a nested control's silence reads as its parent's.
- **A sentence in a tooltip that is not `on_disabled_hover_text`.**
  `on_hover_text` on a control that is sometimes disabled shows nothing
  when it is — `pane/create.rs`'s "Clear picks" is exactly this — so a
  reader sees a greyed control whose only words are for the enabled
  case. P1 caught these only because P1 does not range over text.
- **The non-viewer crates.** The rule is about chrome; a refusal's
  wording in `editor-core` is `EditError`'s business.

## The classified population

**Genuine hits (3 controls, 2 rows newly filed, 1 already filed).**

1. `app.rs`, the New-document **Create** button (`:1442-1443`) — gated
   on `!typed.is_empty()`; a blank name is what `Refusal::EmptyName`
   refuses. Second composition, and the two sentences already differ.
   Already filed as `the-new-document-button-states-its-refusal-twice`.
2. `app.rs`, **Undo** and **Redo** (`:1496`, `:1502`) — gated on
   `history().can_undo()` / `can_redo()`, which is exactly the
   condition `Session::step` refuses with `Refusal::NothingToDo`
   (*"nothing to undo or redo"*). The sentence exists, has one home,
   and the buttons show nothing. Filed:
   `undo-and-redo-are-disabled-in-silence-over-a-refusal-that-has-words`.
3. `pane/properties.rs`, the slot **range button** (`:767-773`) — gated
   on `!row.driver.is_driven() && row.value.is_ok()`, and
   `Session::probe_bounds` refuses a driven slot through `guard_driven`
   with the ratified affordance. The button mints *"a computed slot has
   no range of its own to probe"* instead — a third spelling in a panel
   that already calls `Refusal::affordance` at `slot_notes_ui`. Filed:
   `the-range-button-re-mints-the-ratified-affordance`.

**Obeying the rule already (7).** Not defects; they are the evidence
the rule is the tree's.

- `app.rs`'s cancel doors (`:1523-1526`) — `door.blocked`'s `Refusal`,
  rendered by the control.
- `pane/create.rs`'s catalogue entries (`:291-296`) — `entry.refusal()`,
  *"read off the entry, not minted here"*.
- `pane/properties.rs`'s free-move probe (`:394-399`) —
  `free_move_check`, the same function `display.rs`'s op path calls,
  its fault drawn where the control would be.
- `pane/properties.rs`'s add-parameter form (`:241`) —
  `Refusal::exists_wording`, the shared home.
- `pane/profile.rs`'s **Apply** (`:99`) and `preview_verdict`
  (`:136-190`) — the reason drawn from the typed preview above the
  button the same `bool` holds.
- `pane/profile.rs`'s editor refusal (`:53`) — the typed refusal,
  rendered in place of the whole editor.
- `widgets.rs::offer` (`:494-503`) — the lattice's own `admits`
  answer, worded from `sketch::tip_state_words`.

**Not this class — a draft gate, no operation to refuse (16 controls).**

- `pane/create.rs:508-601`, `blocked: Option<&'static str>` in four arms
  — *"pick a frame to draw on"*, *"choose a shape to add"*, *"add a step
  to the chain"* and the bore/radius guard. **The filing predicted this
  was one of the only two genuine hits; it is not.** No `SessionOp` can
  be formed with the plane or the loops missing — the commit arm at
  `:635` says so and pushes tool news rather than refusing — and the
  bore guard's own sentence says the door would *not* refuse a larger
  bore, it would swap the roles. Four true form sentences with nothing
  to be a second copy of.
- `pane/create.rs:622` "Add profile" — `blocked.is_none() && !refused`;
  the `blocked` half above, the `refused` half already drawn.
- `pane/create.rs:427` "Add datum" — `ui.weak` at `:424` above it.
- `pane/create.rs:684-685` "Extrude" — no profile selected.
- `pane/create.rs:987-988` "Select all edges" — nothing picked or
  evaluated.
- `pane/create.rs:1059` "Clear picks" — `count > 0`; nothing to clear.
  Its `on_hover_text` is invisible when it is disabled, which is the
  blind spot above, not this class.
- `pane/create.rs:52`, `:265-277` — a sentence where a picker or an
  entry list would be, over an empty document and an empty directory.
- `pane/properties.rs:249-253` "Create" parameter — `ready` is
  `!name.is_empty() && dimension.is_some()`; `create_param` refuses only
  `ParamExists`, which `:241` already shows before the click. Neither
  arm is a refusal. (The name arm is silent — a gap, not this class.)
- `widgets.rs` and `pane/profile.rs`'s eight `shape.free()` sites
  (`widgets.rs:453`, `:588`, `:614`, `:628`, `:633`, `:638`, `:770`;
  `pane/profile.rs:320`) — a locked editor. `forms::SHAPE_LOCKED` says
  why once, at the top, and no op is attempted.

**Not this class — structural, no sentence owed (5).**
`pane/profile.rs:278`, `:285`, `:292`, `:313` (remove / move up / move
down / insert, gated on `free` and on the index) and `:114` (Revert,
gated on `moved`). Nothing to move up at index 0 is not a refusal; it
is an operation that does not exist.

**Not this class — no operation at all (2 controls).** `app.rs:1464`
and `:1483`, Open… and Save As…, gated on `chooser.usable()` and
carrying `platform::NO_CHOOSER_BACKEND`. **The filing predicted this was
the other genuine hit; it is not.** No `SessionOp` is pushed when the
backend is absent — the button never reaches a door — so there is no
post-click sentence, and `NO_CHOOSER_BACKEND` is a first composition
rather than a second. `platform.rs` says as much at the const: *"the
disabled control carrying this as its `on_disabled_hover_text` is the
read and there is no status-line route beside it."*

**A separate defect found in passing (1).** `pane/profile.rs:333-348`
and `widgets.rs:494-503` compose the same sentence — *"X is not
well-typed here — the tip is {}"* over `sketch::tip_state_words` — in
two places, with `widgets.rs::offer` sitting right there as the helper
the second could call. Both READ the lattice, so both obey this rule;
what they do not have is one home for the words. Filed:
`two-pickers-spell-one-not-well-typed-sentence-twice`.

## The dispositions this rule hands down

- **`the-new-document-button-states-its-refusal-twice`: answer 1, read
  the refusal.** The button IS gated on the condition `NewDocument`
  refuses, so the two sentences are one sentence. The row's cost
  objection — that the refusal's wording is written for a status line —
  is answered by `Refusal::exists_wording`, which exists precisely so
  one composition serves a pre-click notice and a post-click refusal.
  If the button needs shorter words, the words move in `refuse.rs` and
  both surfaces move together. Answer 2 is refused: *"the comment
  should say the button is gated on the same CONDITION"* describes the
  drift rather than repairing it.
- **`environmental-facts-answer-usable-as-a-bool-with-the-reason-
  elsewhere`: this census does not decide it, and the filing's claim
  that `NO_CHOOSER_BACKEND` is a member of this class is wrong.** The
  two classes are disjoint at that site. That row is about a *value*
  that knows a fact and carries none of its words; the complaint stands
  on its own evidence and is untouched by the rule here, which only
  asks whether a second sentence exists to converge with. It does not.

## The filing's citations, re-derived

Recorded because the register's rule is re-derive, never shift. Shape
1: `pane/create.rs:249-260` → `:291-296`; `app.rs:1255-1262` →
`:1523-1526`. Shape 2: `app.rs:1175` → `:1442-1443`;
`pane/properties.rs:213` → `:249-253`; `:735` → `:767-773`;
`pane/create.rs:590-593` → `:622`; `:831` and `:1132` → `:684-685` and
`:987-988`. Shape 3: `app.rs:1197`, `:1227` → `:1464-1465`,
`:1483-1484`. Shape 4: `pane/create.rs:741-751` →
**`pane/profile.rs:333-348`**, with a sibling at `widgets.rs:494-503`
the filing does not name. The `blocked` field:
`:445`/`:449-520`/`:524-526`/`:591` →
`:508`/`:512-596`/`:600-601`/`:622`. The free-move probe:
`pane/properties.rs:352-357` → `:394-399`.

## Home

The rule's durable home is a clause in `crates/viewer/README.md`, which
is VDOC's. Filed there as `a-disabled-controls-reason-has-one-home`,
citing this item. Nothing under `crates/` was touched by this census.
