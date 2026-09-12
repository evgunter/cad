---
id: the-third-tag-vocabulary-macro-owes-a-unification-trigger
kind: issue
title: target_forms! and arc_modes! are near-copies one meta-level up, and the PR's own not-yet argument names a trigger with no schedule
status: open
opened: 2026-09-12
---



(WIRE orchestrator) From the full review of PR 2445 (R3 and S1).

PR 2445 adds `target_forms!`
(`crates/profile/src/path/program.rs:125-183`) beside `arc_modes!`
(`:245-295`). Three of the four things it expands — the tag enum body,
`impl … { ALL }`, and the `$name { .. } => Tag::$name` read-back match
— are structurally identical to `arc_modes!`'s, about 25 lines of
parallel expansion, and the two doc headers are near-verbatim (**"The …
vocabulary — ONE declaration, THREE projections."**).

**The PR's argument for two macros is correct and is not what this row
disputes**: the variant shapes genuinely differ (struct-with-fields
against one tuple plus two units), the FORM list is single-homed, and
generalising two instances into one macro on the strength of two is
how a bad abstraction gets built. So this is not the defect D364
closes. It is the same shape one meta-level up.

What is owed is the schedule the PR's own sentence implies and does not
give: it says *"a third would be the moment to look again"* — a
disclosed deviation with a named trigger and no unit, no owner and no
place the trigger is checked. Per Q6 a disclosed gap owes a concretely
scheduled followup.

**The third is already in the file.** `transition_table!` (`:396+`)
carries the same `ALL` idiom a third time, for `Verb`. Whether it
counts as the trigger depends on whether the shared shape is
"vocabulary → tag + ALL + read-back" (in which case the trigger has
fired) or the narrower "payload enum → payload-free tag" (in which case
it has not, and `transition_table!` is a different animal). That
question is this row: **decide what the trigger actually counts, then
either fire it or write down why `transition_table!` does not.** A
trigger nobody can evaluate is not a schedule.

Two smaller residues from the same reading, to take with it:

- Both macro headers say **"THREE projections"** and expand FOUR
  (`:109` and `:225`) — `Target::kind()` / `ArcData::mode()` is a
  fourth use of `$name`, which the PR body itself counts separately.
  The count is what a reader checks the macro against.
- The tag enums inherit the payload enums' variant docs verbatim
  (`:141` and `:155` apply one `$(#[doc = $doc])*` to both), so
  `TargetKind::StartArriving`'s rustdoc reads *"It carries NO payload …
  If a second declaration is ever ruled, the variant grows a payload
  then"* — prose about a `Target` variant rendered on a payload-free
  tag. `ArcMode` has this identically, so it is a class.
- `#[must_use]` is on `Target::kind()` (`:175`) and not on
  `ArcData::mode()` (`:289`), which are advertised as the same door.
