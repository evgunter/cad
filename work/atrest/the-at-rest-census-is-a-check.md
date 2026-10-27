---
id: the-at-rest-census-is-a-check
kind: issue
title: D10 stage 5 PR C: the at-rest census is a check resident reporting interference and could-not-look; at-rest contacts go to the unproven-coincidence lint; nothing at rest refuses; A5's gate, assemble and Separation retire
status: closed
opened: 2026-10-08
priority: P0
cost: H
closed: 2026-10-26
branch: atrest/the-at-rest-census-is-a-check
pr: 5810
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
  contact half lands here: a holding `Gap = 0` assertion whose two
  selections are the two cells the census found coincident quiets it.
  The cells may be vertices and edges as well as faces: the contact
  half must quiet a vertex-on-face and an edge-on-face row, the half
  `gap-reads-vertex-and-edge-cells` deferred here.
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
§13). It also named `a-mate-on-a-pinned-copy-refuses` (stage 3's F),
which has landed: it closed
`mate-offset-verified-against-the-solve-is-a-constraint-falling-back-to-an-assertion`
and retired A11 (4)'s declaring mates, since a redundant mate now
refuses `Overconstrained` at the door.

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

## Closed

2026-10-26, by the PR on branch `atrest/the-at-rest-census-is-a-check`
(stage 5 C). `CheckId::AtRest` is a registry resident (knob `Severity`,
default Warn, kind Certified) reporting interference and could-not-look
(`AtRestEvidence`) under the `⊔` that decided each; the contact half of
the quieting rule (`checks::at_rest::quiet_contact`) quiets the lint's
at-rest rows at any cell dimension; `CheckFinding::quiet` is listed and
never refuses; `enforce_checks` skips quiet and could-not-look findings;
`assemble`, `assemble_gathered`, `Assembly`, `AssemblyError`,
`AtRestFinding`, `CheckId::Separation`, its evidence and its config
retire. The PR body is the record of what moved. Rows carried past it:
`an-assertion-quiets-an-operations-unproven-row` (opened),
`at-rest-check-rows-left-unwitnessed` (filed).
