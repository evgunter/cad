---
id: two-pickers-spell-one-not-well-typed-sentence-twice
kind: issue
title: Two pickers compose the same not-well-typed sentence, and one of them is the shared helper
status: open
opened: 2026-09-19
refs: [a-disabled-control-says-why-in-four-shapes]
priority: P3
cost: E
---

Found by the census in `a-disabled-control-says-why-in-four-shapes`, at
merge base `2654cc111417da806d9786c40136106469096fec`. Not a hit of that
census's rule — both sites READ the lattice's own answer, which is what
the rule asks for — but a defect of exactly this program's charter
shape: one sentence, two compositions, held in step by nothing. The
same class as `seat-line-spells-the-list-mark-as-a-literal` and
`converged-recourse-has-no-home`.

## The two copies

`crates/viewer/src/widgets.rs`'s `offer` (`:486-508`) is the
picker-choice helper. It is **private to `widgets.rs`** — a bare `fn`,
not `pub(crate)` — which is the first thing a repair has to change and
which the first draft of this row did not say. For a choice the tip
refuses it writes:

```
row.on_disabled_hover_text(format!(
    "{label} is not well-typed here — the tip is {}",
    sketch::tip_state_words(a.state),
));
```

`crates/viewer/src/pane/profile.rs`'s verb combo (`:333-348`), inside
`path_step_rows`, hand-rolls the same control and the same sentence:

```
row.on_disabled_hover_text(format!(
    "{option} is not well-typed here — the tip is {}",
    sketch::tip_state_words(state),
));
```

Same template, same `sketch::tip_state_words` call, same
`add_enabled(refused.is_none(), egui::Button::selectable(…))` around
it.

**`offer` has exactly two callers, both inside `widgets.rs`**:
`target_fields` (`:459`) and `arc_fields` (`:594`). It is not "every
other picker in the editor" and it is not in scope in
`pane/profile.rs` at all — module-private, in a different module. So
the duplication is not a caller ignoring an available helper; it is a
helper that was never made reachable from the third site.

## Why the verb combo did not use it, and whether that survives

The verb row is not a straight `offer` call today: `offer` takes an
`Option<&Admitted<'_, V>>` carrying a `TipState` and a boxed
`admits` predicate, while the verb loop calls `sketch::admits_at(state,
option)` directly and needs the refused `TipState` back out in the
`Some` arm. So the duplication is a real shape mismatch, not laziness.
Two candidate repairs:

- Make `offer` `pub(crate)`, build an `Admitted` for the verb row
  (its `admits` closure is
  `move |v| sketch::admits_at(state, v).is_ok()`) and call it. One
  composition, at the cost of a boxed closure per row per frame — the
  file's own comment above `states` (`:263-266`) accepts *"a replay per
  row of a hand-authored path is cheap"* for the same loop, which is an
  argument about the same order of cost and not a licence.
- Lift just the sentence: a `pub(crate) fn not_well_typed(label: &str,
  state: TipState) -> String` in `widgets.rs` that both call. Smaller,
  it widens one new name rather than a five-argument generic helper,
  and it is the `Refusal::exists_wording` shape — the words get one
  home without the control shape having to match.

The second is the one this program's charter points at: the defect is
in the vocabulary, not in the control.

## Home

**`widgets.rs`'s WORDS are vnews's**, and the first draft of this row
said the opposite. VGEOM's `keep_out` cedes it in as many words:
*"scene.rs and widgets.rs and display.rs and sketch.rs are worked here
for their numbers and by vnews or vseam for their words and their held
state."* The subject here is a sentence, so the `widgets.rs` half is
this program's ground by that cession, even though the file is not in
this program's `paths` list.

`crates/viewer/src/pane/profile.rs` is the half that is not: it is
claimed by no VIEW successor
(`work/view/viewer-src-files-no-successor-claims`), and CHROME's
carve-out of 2026-09-15 cedes `pane/*` to VIEW, which this program
inherits. A lane that takes that half announces the crossing to CHROME,
or the row re-homes with the file.
