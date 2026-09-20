---
id: two-pickers-spell-one-not-well-typed-sentence-twice
kind: issue
title: Two pickers compose the same not-well-typed sentence, and one of them is the shared helper
status: open
opened: 2026-09-19
refs: [a-disabled-control-says-why-in-four-shapes]
---

Found by the census in `a-disabled-control-says-why-in-four-shapes`, at
merge base `2654cc111417da806d9786c40136106469096fec`. Not a hit of that
census's rule — both sites READ the lattice's own answer, which is what
the rule asks for — but a defect of exactly this program's charter
shape: one sentence, two compositions, held in step by nothing. The
same class as `seat-line-spells-the-list-mark-as-a-literal` and
`converged-recourse-has-no-home`.

## The two copies

`crates/viewer/src/widgets.rs`'s `offer` (`:486-508`) is the shared
picker-choice helper. For a choice the tip refuses it writes:

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
`add_enabled(refusal.is_none(), egui::Button::selectable(…))` around it.
`offer` is in scope in that file's crate and is used by every other
picker in the editor — `target_fields` and `arc_fields`
(`widgets.rs:453-461`, `:588-603`) both go through it.

## Why the verb combo did not use it, and whether that survives

The verb row is not a straight `offer` call today: `offer` takes an
`Option<&Admitted<'_, V>>` carrying a `TipState` and a boxed
`admits` predicate, while the verb loop calls `sketch::admits_at(state,
option)` directly and needs the refused `TipState` back out in the
`Some` arm. So the duplication is a real shape mismatch, not laziness.
Two candidate repairs:

- Build an `Admitted` for the verb row (its `admits` closure is
  `move |v| sketch::admits_at(state, v).is_ok()`) and call `offer`. One
  composition, at the cost of a boxed closure per row per frame — the
  file's own comment at `:265-268` says a replay per row is already
  accepted as cheap.
- Lift just the sentence: a `pub(crate) fn not_well_typed(label: &str,
  state: TipState) -> String` in `widgets.rs` that both call. Smaller,
  and it is the `Refusal::exists_wording` shape — the words get one
  home without the control shape having to match.

The second is the one this program's charter points at: the defect is
in the vocabulary, not in the control.

## Home

`crates/viewer/src/widgets.rs` (chrome, vgeom, view) and
`crates/viewer/src/pane/profile.rs` (chrome, view) — **neither is
vnews's ground**, and `pane/profile.rs` is claimed by no VIEW successor
(`work/view/viewer-src-files-no-successor-claims`). Filed here because
the subject is this program's charter verbatim and the census that found
it is this program's. A lane that takes it announces the crossing, or
the row re-homes with the file.
