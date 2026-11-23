---
id: a-tip-refusal-in-the-sketch-preview-names-no-verb-the-tip-takes
kind: issue
title: The sketch preview's not-well-typed sentence names the refused verb and the tip, not the verbs the tip takes
status: open
opened: 2026-11-06
priority: P3
cost: E
---


Found by the sweep of `fillet-step-discoverability-six-door-refusals`
(ROUND), which made the kernel's lattice refusal name what its tip
state takes: `profile::TipState::takes`, read off the transition
table's own arms (`Verb::states`), now ends `profile::ReplayError`'s
and editor-core's `ProgramRefusal::Transition` sentence in a
`Recourse:` naming the verbs that state accepts.

`crates/viewer/src/sketch.rs`'s `not_well_typed` — the one composition
the sketch preview's refusal of a written verb and the greyed pickers
say — renders *"`choice` is not well-typed here — the tip is
`tip_state_words(state)`"*: the bad choice and the state, never what
the state takes. For a picker the un-greyed entries show the accepted
set; for the preview's refusal of a written verb nothing does.

The repair is the same clause in the author's spelling: list
`state.verbs()` (filtering `Verb::is_loop_form` in a chain) through
`Verb`'s `Display` (`line_to`, `fillet`), which is the vocabulary the
preview's sentence already uses. Not done in the ROUND unit because
the sentence is the viewer's and its pickers share it (whether a
picker's greyed-entry hint should carry the list too is the design
point).
