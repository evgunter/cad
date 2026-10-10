---
id: the-at-rest-census-is-a-check
kind: issue
title: D10 stage 5 PR C: the at-rest census is a check resident reporting interference and could-not-look; at-rest contacts go to the unproven-coincidence lint; nothing at rest refuses; A5's gate, assemble and Separation retire
status: parked
opened: 2026-10-08
priority: P0
cost: H
blocked_on: [interference-at-rest-is-a-finding, mates-declare-no-contact, a-mate-on-a-pinned-copy-refuses]
---

INTENT stage 5, PR C. Ev approved the design in PR 4320 (fork log row
91, FORK-S5C); `docs/INTENT-STAGE5-SPEC.md` §4 predates it, and where
they disagree this row governs.

**Nothing at rest refuses.** A5's at-rest gate (`assembly::assemble`,
`crates/editor-core/src/assembly.rs:1171`, `verdict` at `:1273`) becomes
a check-registry resident `CheckId::AtRest` (DISCIPLINES DS6), defaulting
to Warn. The registry's `enforce_checks` is the one refusing door, at a
caller's `Error`. `assemble`, `Assembly`, `AssemblyError::AtRest` and
the `Separation` resident retire.

**What reports what.**

- An at-rest contact is recorded at the coincidence door (stage 4), and
  one the door cannot prove structural is a finding of the
  `unproven-coincidence` lint, not of `AtRest`. The quieting rule's
  contact half lands here: a holding `Gap = 0` assertion whose two faces
  are the two cells the census found coincident quiets it.
- `AtRest` reports B's interference findings, and a could-not-look
  finding for a pair the census has no lane for.

Each finding is quiet under D10's rule or loud. The viewer's at-rest
badge, Python's `assemble` and the tour gallery's check rows are
restated.

It waits on stages 3 and 4 as well as A and B:

- the contact findings are stage 4's records, and stage 4 retires the
  declared attribution that is the rest of today's gate;
- the resident checks per space, which is stage 3's.

The `blocked_on` names `mates-declare-no-contact` (stage 4's last unit
it needs, which records the at-rest contacts at the door; stage 4 spec
§13) and `a-mate-on-a-pinned-copy-refuses` (stage 3's F, the unit that
closes
`mate-offset-verified-against-the-solve-is-a-constraint-falling-back-to-an-assertion`
and A11 (4)'s declaring mates).

When this unit lands, A5's opening and *Interference.* paragraphs
become: "**A5 — The at-rest check.** Per space, the census examines
every pair of copies the boxes cannot prove apart, and decides each one
apart, in contact, overlapping or undecided. A contact is recorded at the
coincidence door; unless it is structural, the `unproven-coincidence`
lint reports it. An overlap is an `AtRest` interference finding,
localised to the faces bounding it, or loud, unquietable and never
refusing when the intersection refuses. An undecided pair is an
`AtRest` could-not-look finding, which refuses at no severity (DS6). Each finding is quiet under D10's rule or loud. Nothing
refuses: a caller that wants a gate runs `enforce_checks` at `Error`."
