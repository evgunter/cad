---
id: a-disabled-controls-reason-has-one-home
kind: issue
title: crates/viewer/README.md states the disabled-control rule for two families and owes the generalisation, with its sweep rule
status: open
opened: 2026-09-19
---

Filed by a VNEWS census lane at merge base
`2654cc111417da806d9786c40136106469096fec`. An announced crossing:
`crates/viewer/README.md` is VDOC's, so the request is stated here
rather than written across the fence. **Nothing under `crates/` was
touched.**

**This row asks for a GENERALISATION, not a first statement.** Its
first draft said the page did not carry the rule; that was false, and
the correction is the substance of what follows.

## What the page already says, and for what

Two clauses, each ruling on one family:

1. **The cancel doors and the file dialogs**, in the toolbar section:
   *"How it says so is a different precedent from where it is drawn"*,
   and citing one for both is wrong. It then rules on both sides — the
   dialog controls hand `platform::NO_CHOOSER_BACKEND`, *"a `&'static
   str` composed at each button"*, to `on_disabled_hover_text`, which
   is the shape `environmental-facts-answer-usable-as-a-bool-with-the-
   reason-elsewhere` is open about; a cancel door follows
   `pane::create`'s catalogue entry instead, *carrying the op's own
   refusal, read off the entry, not minted here*, so
   `CancelDoor::blocked` is a `Refusal`. It names what holds it:
   `gesture_table.rs`'s
   `a_closed_door_says_what_its_own_operation_refuses`, and says
   *"nothing structural stops a future door composing its own
   sentence; that row is what would red."*
2. **`prefs::Unusable`**, in the preferences section: *"the store is
   the party that words the condition … and the chrome renders its
   words rather than composing its own"*, with the divergence held by
   an assertion, and with its own sweep rule — over the FIELD
   `app::ViewerApp::store`, not over the name, the string or the
   target.

So the argument is the page's already, twice, complete with the
"held by a test, not by the type" caveat and one worked sweep rule.
What is missing is the **generalisation**: nothing on the page says the
two are instances of one rule that governs every control in the crate,
and nothing says what the population of that rule is.

## What the clause would add

The rule, present tense, in the census's one composition — quoted
verbatim rather than re-worded, because a rule about one composition
that is itself composed several ways is its own first counterexample:

> **A control a reader cannot use owes the sentence a click would have
> been answered with — when there is such a sentence.** The question at
> the control is: if the reader got past this gate and the operation
> ran, what sentence would come back? A refusal would → the pre-click
> sentence and the refusal are the same sentence and have one
> composition, read off the refusal value or off a wording helper the
> refusal's `Display` also calls. Nothing would, because no operation
> can be formed yet → the control gates a draft, and a true literal at
> the control is correct. The operation would be formed and would
> SUCCEED, and the chrome declines it anyway → a chrome-policy gate,
> which has no refusal to read and owes instead that its sentence be
> true, that the policy have one home if a second surface ever applies
> it, and that it be disclosed as a policy rather than read as a
> refusal.
>
> **The population is that disposition, not the egui call that usually
> carries it.** The sweep is `add_enabled` / `add_enabled_ui` over the
> crate — 30 sites — unioned with the branches that draw a sentence
> where a control would be, which is a judgement and is therefore
> enumerated rather than counted. `on_disabled_hover_text` finds a
> strict subset of the first half and cannot see a control that is
> disabled in silence, one whose reason is drawn beside it, or one
> whose only words ride on `on_hover_text` and vanish when it greys.

Three things about that text:

- **The third arm is new** and is the census's own, not the page's and
  not `refuse.rs`'s. `pane/create.rs`'s bore/radius arm is the tree's
  one instance. Whether the chrome may decline what the door would
  accept is a design question the census explicitly did not settle, so
  the clause records the case rather than ratifying the practice — or
  VDOC routes it to Ev as a design question. VDOC's call which.
- **The second paragraph is the part this program's rows exist for.**
  VIEW's register records the call-shaped sweep as the eighth entry in
  its proxy-failure table (#2320), and *a universal in prose owes the
  sweep rule that produces its population at the sentence* (#2143).
  The existing `prefs::Unusable` clause already models exactly this.
- **The 30 is a measurement with nothing reading it.** Filed as
  `work/guard/viewer-disabled-control-population-has-no-gate`; if that
  gate lands, the clause cites it the way the `platform` row cites
  `scripts/gates/no-ambient-env.sh`.

## Sequencing

The clause describes what the code does, and **four controls do not
obey it yet** — the New-document Create button, Undo, Redo and the slot
range button, on three rows
(`work/vnews/undo-and-redo-are-disabled-in-silence-over-a-refusal-that-
has-words`, `.../the-range-button-re-mints-the-ratified-affordance`,
`.../the-new-document-button-states-its-refusal-twice`), plus two P2
sites on `.../three-spellings-say-a-parameter-is-not-declared`. A
present-tense clause written today is false at all six. Either the
clause waits for them, or it lands with the exceptions named — VDOC's
call, and the VNEWS plan's exit shape assumes the former.

The census, its rule and its two enumerated populations are in
`work/vnews/a-disabled-control-says-why-in-four-shapes.md`.
