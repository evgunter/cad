---
id: a-disabled-control-says-why-in-four-shapes
kind: issue
title: A control a reader cannot use owes the sentence a click would have been answered with — the rule, and the controls that owe it
status: review
opened: 2026-09-11
refs: [environmental-facts-answer-usable-as-a-bool-with-the-reason-elsewhere, the-new-document-button-states-its-refusal-twice, the-range-button-re-mints-the-ratified-affordance, undo-and-redo-are-disabled-in-silence-over-a-refusal-that-has-words, two-pickers-spell-one-not-well-typed-sentence-twice, three-spellings-say-a-parameter-is-not-declared, gesture-drags-have-no-cancel-door, a-disabled-controls-reason-has-one-home]
priority: P3
cost: E
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
been answered with — when there is such a sentence.**

That line is the unit's one composition of the rule. Every other
document that states it quotes it verbatim — `work/vnews/log.md`,
`work/vdoc/a-disabled-controls-reason-has-one-home.md` and the
disposition rows — rather than re-wording it, and in particular the
qualifier never travels without the rule. A rule about one composition
that is itself composed five ways would be its own first
counterexample, which is what the earlier draft of this item was.

The test is not "is there a refusal type in scope" and not "does this
call `on_disabled_hover_text`". It is one question asked at the
control:

> If the reader got past this gate and the operation ran, what sentence
> would come back?

The tree answers it three ways, not two.

- **A refusal would come back.** Then the pre-click sentence and the
  post-click sentence are *the same sentence*, and they get **one
  composition** — read off the refusal value, or off a wording helper
  the refusal's own `Display` also calls. Minting a second is the
  defect; saying nothing is the same defect with the second copy empty.
- **Nothing would come back**, because no operation can be formed yet —
  the control gates a DRAFT, not a door. Then there is no sentence to
  be the same as, a literal at the control is correct, and the only
  obligation is that it be true.
- **The operation would be formed and would SUCCEED**, and the chrome
  declines it anyway. A **chrome-policy gate**: the door would accept
  what the control refuses to send it. This arm was added by the
  census, because the tree has one and the two-armed rule classified it
  as a draft gate, which is false of it (`pane/create.rs`'s bore/radius
  arm, below).

### What a chrome-policy gate owes

It cannot read a refusal, because there is none to read, so its
sentence is minted at the control by construction and the
one-composition obligation has nothing to attach to. What it owes
instead is three things, and they are weaker:

1. **That the sentence be true**, as for a draft gate.
2. **That the policy have one home** if a second surface ever applies
   it, so the second reads the value rather than re-mints the words.
   Today the tree's one instance has one surface.
3. **That the gate be disclosed as a policy.** A control whose own
   words say the door would *not* refuse is telling a reader the chrome
   overrode the kernel. Whether the chrome may do that at all is a
   design question this census does not settle and does not assume: it
   records that the tree does it once, names the site, and stops.

### This rule's first limb is stated nine times in the tree

It is not minted here, and the attribution is narrower than the census
first wrote. Re-derived, the sites that state the FIRST limb — that the
words shown at a control a reader cannot use are the composition the
click's refusal would use — are **nine, in five files**:

- `crates/viewer/src/session/refuse.rs`: `Refusal::self_instance`
  (*"the chrome's disabled reason is the same value the click would
  have been answered with"*), `Refusal::exists_wording` (*"the
  pre-click notice and the refusal cannot drift apart"*), and
  `Refusal::affordance` (*"composed once and every surface that shows
  it … calls this"*).
- `crates/viewer/src/session/op.rs`: `CancelDoor` and
  `CancelDoor::blocked` (*"the disabled control's words are the refused
  operation's own"*) — one subject, two doc comments.
- `crates/viewer/src/pane/create.rs`: the parts-catalogue entry
  (*"carrying the op's own refusal — read off the entry, not minted
  here"*).
- `crates/viewer/src/parts.rs`: `PartEntry`'s `open_document` mint
  (*"the entry the chooser disables and the id the op refuses are
  decided by one predicate"*).
- `crates/viewer/src/pane/properties.rs`: the free-move probe (*"shown
  where the control would be — the same sentence the op would refuse
  with"*), `Panel::slot_notes_ui` (*"the same string the status line
  shows when the edit is actually attempted"*), and the add-parameter
  form's already-exists notice (*"the same sentence the session's
  refusal would show … ahead of the click"*).

**None of the nine states the second limb**, and the second limb is
what does the classification work here: it is *"— when there is such a
sentence"* that decides twenty-two of the thirty `add_enabled` sites,
and it is what overturns both of the filing's predictions. The third
arm is stated nowhere at all. So the first limb is the tree's and is
cited as the tree's; the qualifier and the third arm are this census's
own and are defended above rather than attributed. The earlier draft's
*"already ratified, three times, in `refuse.rs`"* was wrong twice over
— it undercounted the statements and it claimed ratification for the
half of the rule nothing in the tree states.

### What the rule does NOT say

It does not say every disabled control owes prose. `add_enabled(free &&
index > 0, …)` on a move-up arrow (`pane/profile.rs`) gates a row
reorder that cannot be formed at index 0; there is no op, no refusal,
and no sentence is owed. And it does not say a reason must ride on
`on_disabled_hover_text`: `pane/properties.rs`'s free-move probe draws
`fault.to_string()` **where the control would be** and is the tree's
best-obeying member.

## The population: two passes of different epistemic status

The property is **a control drawn as unusable, together with whatever
the reader is told about why**. Two passes produce it, and **they are
not one number and are not added together here.** P1 is mechanical:
one grep and one stated exclusion, re-derivable by anyone in a minute.
P2 is a judgement about what "in the place a control would occupy"
means, and a judgement is only a census if its members are listed. The
earlier draft of this item asserted a joint total of 35, enumerated
neither the join nor P2's members, and classified six buckets that
summed to 34; that is repaired below by listing both populations and by
not restating a joint total at all.

### P1 — `add_enabled` / `add_enabled_ui`: 30 call sites, mechanical

`grep -rn 'add_enabled' crates/viewer/src` returns 32 lines; **two are
prose**, inside `pane/profile.rs`'s own comment contrasting
`add_enabled` with `add_enabled_ui`. The remaining 30 are the
population. By file: `widgets.rs` 8, `pane/profile.rs` 8, `app.rs` 6,
`pane/create.rs` 6, `pane/properties.rs` 2. Every one is classified
below, by conjunct where its conjuncts have different answers.

There is no third call-shaped pass: `ui.set_enabled`, `ui.disable()`
and a `.enabled(` builder appear nowhere in the crate, and the four
`Sense::` hits are viewport allocations and an `AxisSense` enum.

**What this number owes, per §Q6.** It rests on a measurement and
nothing goes red when it stops being true: a thirty-first `add_enabled`
lands with no gate, no test and no register re-reading it, and this
item is a tracker file that will be deleted when VNEWS closes. It is
**unguarded, not unguardable** — `scripts/gates/viewer-vocab-declared-
once.sh` and `scripts/gates/viewer-module-kinds.sh` are the precedent
for exactly this shape of grep over `crates/viewer/src`, and
`scripts/gates/probe-suite-census.sh` is the precedent for a gate that
holds a census's membership rather than a comment's number.
`scripts/gates/*` is GUARD's, so the guard is filed rather than
written: `work/guard/viewer-disabled-control-population-has-no-gate`.

### P2 — a sentence drawn in the place a control would occupy: 11 sites, judged

Derived by reading all 61 `.weak(` / `colored_label(` sites in the
crate and keeping those that stand **where a control would be** rather
than **beside one**. The test is the branch structure: a site is in
when the sibling arm of the same `if`/`match` draws a control at that
position. Members, all eleven:

| # | site | what the sibling arm draws | arm |
|---|---|---|---|
| 1 | `pane/create.rs`'s `frame_picker`, empty-frames arm (`:52`) | the `ComboBox` | draft |
| 2 | `pane/create.rs`'s parts chooser, `Ok([])` arm (`:277`) | the entry buttons | draft |
| 3 | `pane/features.rs`'s tree row, `through: None` arm (`:113`) | `ui.link(message)` — the same text | draft |
| 4 | `pane/profile.rs`'s `edit_profile_ui` refusal (`:53`) | the whole editor | **obeys** |
| 5 | `pane/properties.rs`'s `standing_ui`, `Node { present: false }` (`:291`) | `delete_button` | draft |
| 6 | `pane/properties.rs`'s `standing_ui`, `Param { present: false }` (`:297`) | the parameter's row | **hit** |
| 7 | `pane/properties.rs`'s param row, *"that parameter is gone"* (`:119`) | the value field and `param_bounds_ui` | **hit** |
| 8 | `pane/properties.rs`'s free-move probe (`:399`) | the free-move control | **obeys** |
| 9 | `pane/properties.rs`'s `Selection::None` (`:34`) | the feature's rows | draft |
| 10 | `pane/properties.rs`'s `Selection::Node`, empty groups (`:39`) | the slot rows | draft |
| 11 | `pane/properties.rs`'s `Selection::Face`/`Edge`, empty groups (`:52`) | the slot rows | draft |

**6 and 7 are genuine hits, and they are the finding enumerating P2
produced.** `Session::begin_param_gesture` refuses a drag on an
undeclared parameter with `Refusal::NoSuchParam`, whose `Display` is
*"no document parameter named X — declare it first"*. The two sites
stand where that drag's field would be and say *"parameter X is no
longer declared"* and *"that parameter is gone"* — two more spellings
of the refusal's own fact. Filed:
`three-spellings-say-a-parameter-is-not-declared`.

**And the near misses, with the reason each is out**, because the rule
that sorts them is the deliverable:

- `pane/properties.rs:353` (*"no evaluation yet to resolve this
  against"*), `:357`, `:362`, `:369` — `entity_standing_ui`'s
  resolution verdict, drawn **after** the delete button the same
  function has already drawn, not in its place. Beside, not instead.
- `pane/properties.rs:560` — the slot fault; the comment at the site
  says the field is drawn anyway *"over the one number it does not
  have"* and the fault is *"said beside the field"*. Beside.
- `pane/properties.rs:183` (`Refusal::offer_wording`) and `:724`
  (`Refusal::affordance` in `slot_notes_ui`) — an offer and a note
  beside controls that are drawn and live.
- `pane/create.rs:266` (*"no directory"*) — a header line naming the
  chooser's directory, in the place of a path, not of a control.
- `pane/create.rs:127`, `:130`, `:133` — the mate tool's pick state,
  beside the radio row it does not replace.
- `pane/view.rs:23`, `:64`, `pane/features.rs:81`, `:91`, `:123`,
  `pane/properties.rs:94`, `:485`, `:487`, `:499`, `:508`, `:518`,
  `:748`, `:802`, `pane/profile.rs:155`, `:201`, `:275`,
  `pane/create.rs:154`, `:301`, `:708`, `:741`, `:754`, `:771`,
  `:788`, `:840`, `:941`, `:942`, `app.rs:218`, `:1385`, `:1588` —
  labels, badges, seat lines and advisories, none standing in a
  control's position.
- **Nine lines are the reason-half of a P1 member** and are classified
  with their control rather than counted here:
  `pane/create.rs:424` (the datum notice, above `:427`), `:601`
  (`blocked`'s sentence, above `:622`), `pane/profile.rs:72`
  (`forms::SHAPE_LOCKED`, above the six `shape.free()` gates),
  `preview_verdict`'s five (`:155`, `:160`, `:166`, `:183`, `:185`,
  read by both `pane/profile.rs:99` and `pane/create.rs:622`), and
  `pane/properties.rs:241` (`Refusal::exists_wording`, above `:249`).
  An earlier draft said *"five more `.weak` sites"* and then named four
  entities; it is nine lines belonging to four controls or control
  groups.

**P1 and P2 do not overlap.** A P1 site is an `add_enabled` call; a P2
site is a branch with no such call in it. So there is nothing to
deduplicate, and equally nothing gained by adding 30 to 11: the sum
would carry P2's epistemic status and hide which half it came from.
**The population statement is: P1 is 30 and mechanical; P2 is the
eleven sites tabulated above, by the sibling-arm test.**

### P3 — `on_disabled_hover_text`, and what it measures

11 call sites (14 grep lines minus three doc comments, in
`platform.rs`, `session/op.rs` and `pane/profile.rs`). **Every one
attaches to a P1 site**, so it contributes no member. That is a
measurement of a *narrower* claim than the register's eighth
proxy-table row makes: the row is about the call-shaped sweep standing
in for a control's DISPOSITION, and what was measured here is
`on_disabled_hover_text ⊆ add_enabled` — the subset relation between
the two call names. The disposition claim needs P2 as well, and P2 is
a judgement, so the row is corroborated in its narrow half and not
mechanically confirmed in its wide one.

## What this pattern could not match

Stated as the instrument requires, before the hit list rather than
after it.

- **A control that is simply not drawn.** A tool panel that returns
  early, a row omitted from a list, a `CollapsingHeader` never opened —
  a reader cannot use it and is told nothing, and no pass above can see
  an absence. `session/op.rs`'s seat tables are the likely population.
  **Ruling: out of scope and not scheduled by this unit.** It is a
  different property (what the chrome does not offer) reached by a
  different instrument (walking `SessionOp`'s seats against the panels
  that seat them), on VSEAM's ground rather than this program's, and a
  row filed here would be a wish rather than a finding — nothing in
  this census names an instance of it.
- **egui's own greying.** A widget inside a `Ui` whose ancestor was
  disabled by a caller several frames of code away is greyed with no
  P1 call at the widget. **Ruling: closed for the tree as it stands,
  and stated so it re-opens visibly.** All six `add_enabled_ui` sites
  in the crate gate on `ShapeEdits::free()` (`widgets.rs:453`, `:588`,
  `:614`, `:628`, `:633`; `pane/profile.rs:320`), every one of them
  inside the locked profile editor, and `pane/profile.rs:72` draws
  `forms::SHAPE_LOCKED` once above all of them. So every nested control
  greyed by an ancestor today is covered by one sentence, and the blind
  spot is empty. A seventh `add_enabled_ui` on a different condition
  re-opens it.
- **A control drawn as usable that refuses on click.** The rule's
  mirror image. `pane/create.rs`'s "Add profile" reaches an
  unreachable-in-practice arm that pushes tool news instead, which is a
  post-click sentence with no pre-click one.
- **A sentence in a tooltip that is not `on_disabled_hover_text`.**
  `on_hover_text` on a control that is sometimes disabled shows nothing
  when it is, so a reader sees a greyed control whose only words are
  for the enabled case. **Swept rather than named**: 17
  `on_hover_text` lines that are not `on_disabled_hover_text`, of which
  **six** have a gated receiver — `pane/create.rs:1059-1060` ("Clear
  picks") and `pane/profile.rs:114-115`, `:278-279`, `:285-286`,
  `:292-293`, `:313-314`. Filed:
  `work/vnews/clear-picks-hover-text-is-invisible-while-disabled`.
- **The non-viewer crates.** The rule is about chrome; a refusal's
  wording in `editor-core` is `EditError`'s business.

## The classified population

Every P1 site appears exactly once below, and a site with conjuncts
that answer differently is split by conjunct — the same treatment
`pane/properties.rs`'s `ready` already gets.

**Genuine hits — 4 controls at 4 P1 sites, on 3 rows.**

1. `app.rs`, the New-document **Create** button (`:1442-1443`) — gated
   on `!typed.is_empty()`; a blank name is what `Refusal::EmptyName`
   refuses. Second composition, and the two sentences already differ.
   Already filed as `the-new-document-button-states-its-refusal-twice`.
2. `app.rs`, **Undo** (`:1496`) and 3. **Redo** (`:1502`) — two
   controls, gated on `history().can_undo()` / `can_redo()`, which is
   exactly the condition `Session::step` refuses with
   `Refusal::NothingToDo` (*"nothing to undo or redo"*). The sentence
   exists, has one home, and both buttons show nothing. Filed:
   `undo-and-redo-are-disabled-in-silence-over-a-refusal-that-has-words`.
4. `pane/properties.rs`, the slot **range button** (`:767-773`) — gated
   on `!row.driver.is_driven() && row.value.is_ok()`. The first
   conjunct is the hit: `Session::probe_bounds` refuses a driven slot
   through `guard_driven` with the ratified affordance, and the button
   mints *"a computed slot has no range of its own to probe"* instead —
   a third spelling in a panel that already calls `Refusal::affordance`
   at `slot_notes_ui`. The second conjunct is a separate complaint, and
   its row states it as unproven rather than asserted. Filed:
   `the-range-button-re-mints-the-ratified-affordance`.

**Obeying the rule already — 4 P1 sites plus 2 conjuncts.** Not
defects; they are the evidence the rule is the tree's.

- `app.rs`'s cancel doors (`:1523-1526`) — `door.blocked`'s `Refusal`,
  rendered by the control.
- `pane/create.rs`'s catalogue entries (`:291-296`) — `entry.refusal()`,
  *"read off the entry, not minted here"*.
- `pane/profile.rs`'s **Apply** (`:99`), `!refused` conjunct — reads
  `preview_verdict`, which ran the commit door's own ladder.
- `pane/create.rs`'s **Add profile** (`:622`), `!refused` conjunct —
  the same `preview_verdict`, the same reason.
- `pane/profile.rs`'s verb combo (`:333-348`) and `widgets.rs::offer`
  (`:494-503`) — both read the lattice's own `admits` answer, worded
  from `sketch::tip_state_words`. Both obey this rule; the defect
  between them is a second composition of the words, filed separately
  (below).

**A chrome-policy gate — 1 conjunct at 1 P1 site.**

`pane/create.rs`'s bore/radius arm (`:546-554`, setting the `blocked`
that `:601` draws and `:622` reads) fires when `profile_bored &&
profile_bore >= profile_radius`. At that point the plane is picked and
the shape is `Circle`, so `Drafts::profile_programs` returns `Ok` — it
refuses only a non-finite field or a path that is not a program's shape
— and `SessionOp::AddProfile` would be formed and committed. The arm's
own sentence says so: *"a larger bore would swap the roles **rather
than refuse**"*. **The filing predicted `blocked` was one of the only
two genuine hits; it is not, and this arm of it is not a draft gate
either.** The door would accept; the chrome declines on a policy of its
own about which loop is the hole. That is the third arm, and the
earlier draft filing it under *"no operation to refuse"* was false of
it.

**Draft gates — no operation can be formed: 14 P1 sites plus 3
conjuncts.**

- `pane/create.rs`'s other three `blocked` arms (`:511`, `:518`,
  `:595`) — *"pick a frame to draw on"*, *"choose a shape to add"*,
  *"add a step to the chain"*. No `SessionOp` can be formed with the
  plane or the loops missing; the commit arm at `:635` says so and
  pushes tool news rather than refusing. Three true form sentences with
  nothing to be a second copy of. They reach P1 through `:622`'s
  `blocked.is_none()` conjunct.
- `pane/create.rs:427` "Add datum" — `!unpicked`; `ui.weak` at `:424`
  above it.
- `pane/create.rs:684-685` "Extrude" — `add_enabled(false, …)` in the
  `None` arm, with *"select the profile to extrude first"*.
- `pane/create.rs:987-988` "Select all edges" — `add_enabled(false, …)`
  where the target/evaluation/index triple is absent, with its own
  sentence.
- `pane/create.rs:1059` "Clear picks" — `count > 0`; nothing to clear.
  Its `on_hover_text` is invisible when it is disabled, which is the
  blind spot above and has its own row.
- `pane/properties.rs:249-253` "Create" parameter — `ready` is
  `!name.is_empty() && dimension.is_some()`. Dimension conjunct: a
  draft gate with a true literal
  (`on_disabled_hover_text("pick a dimension first")`). Name conjunct:
  silent, and `create_param` refuses only `ParamExists`, so by this
  rule nothing is owed — **but the reason is that no door refuses a
  blank name at all**, `editor_core`'s included, which is a live defect
  rather than a gap. Filed:
  `work/edit/no-door-refuses-a-blank-parameter-name`.
- `widgets.rs` and `pane/profile.rs`'s eight `shape.free()` sites
  (`widgets.rs:453`, `:588`, `:614`, `:628`, `:633`, `:638`, `:770`;
  `pane/profile.rs:320`) — a locked editor. `forms::SHAPE_LOCKED` says
  why once, at the top, and no op is attempted, because the document's
  edit vocabulary has no door that rewrites a committed profile's
  program.
- `pane/profile.rs:99`'s `moved` conjunct and `:114` Revert — nothing
  has been changed, so there is nothing to apply or revert.

**Structural — the operation does not exist: 4 P1 sites plus 2
conjuncts.** `pane/profile.rs:278`, `:285`, `:292`, `:313` (remove /
move up / move down / insert). Each is gated on `free`, which is the
locked-editor draft gate above and is where the sentence comes from;
the two index conjuncts (`index > 0` at `:285`, `index < last` at
`:292`) are the structural half. Nothing to move up at index 0 is not a
refusal; it is an operation that does not exist.

**No operation at all — 2 P1 sites.** `app.rs:1464` and `:1483`,
Open… and Save As…, gated on `chooser.usable()` and carrying
`platform::NO_CHOOSER_BACKEND`. **The filing predicted this was the
other genuine hit; it is not.** No `SessionOp` is pushed when the
backend is absent — the button never reaches a door — so there is no
post-click sentence, and `NO_CHOOSER_BACKEND` is a first composition
rather than a second. `platform.rs` says as much at the const: *"the
disabled control carrying this as its `on_disabled_hover_text` is the
read and there is no status-line route beside it."*

**The P1 arithmetic, so the buckets re-derive the count.** 4 hits +
4 obeying + 1 chrome-policy (`pane/create.rs:622`, counted once and
split by conjunct) + 14 draft + 4 structural + 2 no-operation =
**30 sites**, counting `pane/create.rs:622` in the chrome-policy row
and not again in the draft row, and counting `pane/profile.rs:99` in
the obeying row and not again in the draft row. The split conjuncts are classified inside their control's row and
add nothing to the total. P2's eleven are in P2's table and nowhere
else, and are not added to this.

**A separate defect found in passing.** `pane/profile.rs:333-348` and
`widgets.rs:494-503` compose the same sentence — *"X is not well-typed
here — the tip is {}"* over `sketch::tip_state_words` — in two places.
Both READ the lattice, so both obey this rule; what they do not have is
one home for the words. Filed:
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
the filing does not name. `pane/create.rs`'s `blocked` field, all four
numbers in that file: `:445`/`:449-520`/`:524-526`/`:591` →
`:508`/`:511-597`/`:600-601`/`:622`. The free-move probe:
`pane/properties.rs:352-357` → `:394-399`.

## Home

The rule's durable home is a clause in `crates/viewer/README.md`, which
is VDOC's. Filed there as `a-disabled-controls-reason-has-one-home` —
as a request to **generalise** two clauses the page already carries,
not to state the rule for the first time. Nothing under `crates/` was
touched by this census.
