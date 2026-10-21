---
id: does-raising-the-ambiguity-factor-k-count-as-loosening-under-d4
kind: issue
title: Does D4 ¶1 (i)'s 'no refusal advises loosening ε' cover WayInZeroBand's 'raise the ambiguity factor K'?
status: open
opened: 2026-10-21
priority: P3
cost: E
design: true
---

## The question

`crates/editor-core/src/names/emit.rs:672`, `AMBIGUITY_FACTOR_LEVER`,
the whole recourse of `NamingError::WayInZeroBand`: "raise the
tolerance's ambiguity factor K". The variant's doc says why: "A
certified edge spans at least the escalate band, K zero bands, so at a
small ambiguity factor K the comparison can fall in the zero band. The
lever is K."

DESIGN D4 ¶1 (i) (`docs/DESIGN.md:515-518`, ruled by Ev on `[ev]` PR
3352): "No refusal advises loosening ε, except one that would otherwise
name no recourse at all (a kernel approximation limit), which names it
as a last resort and says the refusal may indicate a kernel bug." D4's
two-tolerance principle names ε_input = K·ε and says "K stays the one
knob" (`docs/DESIGN.md:494-500`).

Does raising K count as "loosening ε" under that clause?

**For (it is loosening):**
- K·ε is ε_input, "the least precision a user might care about": what
  counts as too close to a coincidence. Raising K coarsens that reading
  across the whole model, which is the global cost Ev's ruling weighed
  against ("messing with epsilon is not something that we generally want
  to be the best path for the user").
- D4 makes ε_input a synonym for K·ε, not a third dial. So advice that
  raises ε_input by turning K is advice that raises ε_input.

**Against (it is not):**
- ε itself, which governs what is built and certified, does not move.
  Raising K widens the escalate band against the zero band, so more
  readings escalate rather than decide, and nothing that failed now
  passes silently.
- The emitter files this under category 5, "a PRECISION LIMIT the model
  controls … Each ends in the model's lever alone". There is no geometry
  lever, so even read as loosening it would be the "no recourse at all"
  case. Then the exception would apply, and the ending would owe D4's
  last-resort wording and the "may indicate a kernel bug" clause, which
  it lacks today.

So either the text stands, or it gains the last-resort shape. If the
owner cannot settle it from the ratified text, it is an `[ev]` question.

Found by the scan in PATHS'
`path-scene-resolution-recourses-advise-a-coarser-tolerance-beside-geometry-levers`
(closed by NURBS build A's `nurbs/scene-resolution-recourse`).
