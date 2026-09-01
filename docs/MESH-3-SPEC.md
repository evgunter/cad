# MESH-3 — issue 896: the undeclared-pole guard on walk's classification

**Binding at dispatch** (S-MESH program, `docs/S-MESH-PLAN.md`;
difficulty logged pre-draw: **M**). Read
`docs/prompts/implementer-discipline.md` in full before starting.
Issue 896 is the primary specification; issues 895 (the sibling
guard) and 889 (`poles()` machinery) are its cited neighbors.

## Situation

`walk.rs`'s `pole_v` decides three things with one ε comparison
(how many `UvPoint`s are pushed; whether the coordinate is the
chart's exact analytic pole v or the vertex's measured v; the
`pole` flag that drives `curved.rs`'s fan construction) — so mesh
structure is a function of (body, δ, ε), against D9's letter. The
crate's own prose (`sizing.rs`) discloses this as the structural,
unexercised dependence. #895's guard does not cover it: it compares
declared vertices against declared vertices; an UNDECLARED chart
pole (a sphere face whose boundary excludes the pole) is invisible
to it. Line numbers have drifted twice since the issue (MESH-1/2
edited the file); locate `pole_v` and #895's guard by name.

## Deliverables

1. **The guard the issue names**: no non-pole junction lies within ε
   of a chart pole it is not being identified with — a D2 row-5
   `debug_assert` beside #895's, with a payload naming the junction,
   the pole, the gap, and ε (the file's established assert voice).
2. **The fixture is the hard half and is owed honestly.** No in-tree
   body puts a non-pole vertex within any suite's ε of a pole (lane
   I-e traced 834 zero-lever entries, all genuine poles). The
   issue's own route in is a STEP import. `mesh` has no
   dev-dependency on `step-import`, so the row lives where the
   dependencies allow — `step-import/tests` or a cross-crate suite —
   and the unit records where and why. If no route reaches the
   guard even from import (a measured claim, not an assumption),
   the recorded verdict at the site takes the fixture's place and
   says what would have to change.
3. **The classification's ε reads do not multiply**: this unit adds
   the guard, not a new decision — the pole_v decision itself is
   unchanged, byte-for-byte, on every existing body (the MESH-1
   corpus-stability standard: say it as a checked claim). Issue
   881's named-operations port (MESH-4) inherits the new read —
   name the new consumer in a one-line comment at the site so the
   #881 sweep cannot miss it.
4. **ε posture** (issue-1356): the guard reads ε by definition —
   state the bands the new row(s) exercise, three-outcome honest,
   and whether the gate needs a trailer on the final head.
5. **Class sweep** (discipline §5): other single-ε classification
   reads in `walk.rs` whose failure mode is a mis-shaped mesh
   rather than a wrong number; hit list, dispositions, blind spots.

## Acceptance

- The guard red-first: demonstrated firing on the fixture (or the
  recorded no-route verdict per deliverable 2) before it lands
  green.
- Existing mesh suites green, byte-stable; the tour green.
- Hosted CI green on the final head; gate record in the PR.

## Hard rules

- NO `Co-Authored-By` trailer, no model names in commits. "issue
  896" spelled out; the orchestrator closes it after merge.
- Scope fence: `crates/mesh/src/walk.rs` (the guard beside #895's —
  do NOT touch the ε terminal reads MESH-4 owns, `closing_column`,
  `gap_is_noise`, or `loop_area`), mesh tests, and the fixture's
  home per deliverable 2. No `docs/MODEL-AB-LOG.md`, no
  `docs/S-MESH-*.md`, no SMELL edits.
- Any refusal minted is classified against the D2 addendum (row 5
  is the expected class; a typed refusal instead is a deviation to
  argue).
