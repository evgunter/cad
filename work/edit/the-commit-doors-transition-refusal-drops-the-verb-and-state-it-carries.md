---
id: the-commit-doors-transition-refusal-drops-the-verb-and-state-it-carries
kind: issue
title: The commit door's transition refusal drops the verb and tip state its value carries
status: open
opened: 2026-09-28
priority: P4
cost: E
refs: [two-pickers-spell-one-not-well-typed-sentence-twice]
---

Found by the sweep of `two-pickers-spell-one-not-well-typed-sentence-twice`
(VNEWS), which gave the viewer's "`X` is not well-typed here — the tip
is …" sentence one composition (`viewer::sketch::not_well_typed`).

`crates/editor-core/src/program.rs`'s `impl Display for ProgramRefusal`
renders `Transition` as *"loop {loop_} step {step} is not a legal
chain-lattice walk"*. The variant carries `state: profile::TipState`
and `verb: Option<profile::Verb>`, and neither reaches the sentence;
the comment above the impl says only that `Transition` "holds a
lattice state rather than a refusal". A library caller told a walk is
illegal is not told which verb, or at what tip.

Not the viewer's helper to fix: editor-core sits below the viewer, and
the viewer never shows this sentence for a walk its preview refuses
(both path editors' commit buttons, `pane/profile.rs`'s Apply and
`pane/create.rs`'s Add profile, are disabled on a refused preview). The
variant's own doc says no recording surface produces it (a corrupt or
hand-built program), which is why this is P4. A fix names the verb by
`profile::Verb`'s `Display` and the state in words — which would want
`tip_state_words`' phrasing to live where editor-core can reach it,
the one design point on the row.
