---
id: the-rectangle-and-circle-templates-reopen-as-paths
kind: issue
title: A rectangle profile stores its corners, not the width and height written, and a bored circle stores its centre twice
status: open
opened: 2026-09-19
priority: P1
design: true
cost: M
---


## What happens

The add-profile form offers three shapes (`forms::ShapeKind`): circle
(with an optional bore), rectangle, and path. The edit door
(`ViewerBehavior::edit_profile_ui`, PR #2862) loads every committed
profile as PATH steps (`sketch::held_loops`), because a committed
program records the lowered steps and not which template made them. A
rectangle therefore reopens as `at` + four `line_to`s, and a circle as
one `circle` step: the numbers round-trip exactly
(`tests/profile_edit.rs`, `every_authored_profile_round_trips…`), but
the interface the person authored with — width and height, centre,
radius and bore — is not the one they edit in. "The same interface"
holds for the Path shape only.

## What would close it

Either record the template on the node (a document change: the
program would carry its authoring template, which is DESIGN territory),
or recognise a template's shape on load (a rectangle's four corners
centred on the origin, axis-aligned; a pair of concentric circles) and
offer the template's fields when it matches, falling back to the path
steps when it does not. The second is viewer-only but is a guess about
intent; the first is the honest one.

## 2026-10-21 — re-banded P0 → P1

The profile still opens and its numbers round-trip exactly; what is
lost is the template's interface in the edit door. That is an
annoyance, P1's GUI band, and not a shape the GUI cannot author (P0).

## 2026-10-31 — re-priced D → M, design open

The choice under "What would close it" (record the template on the
node, or recognise it on load) is open, so `design` is set. The first
option is a document change.

## 2026-10-31 — the premise, corrected by the designer pair (`[ev]` PR, fork-log row 234)

The circle already reopens as itself: `circle(centre, r)` stores what
was written, and the edit door shows that one step's centre and radius.
There are two real defects, both of the kind `[recipe.forms]` names:
a derived number stored in place of the one written.

- **The rectangle stores its corners.** `sketch::loop_program` stores
  `(±w/2, ±h/2)`, eight numbers derived from the two that were typed.
  The width cannot be driven, and the template cannot be recovered
  without a guess.
- **The bore stores its centre twice.** "Circle with bore" lowers to two
  `circle` loops, each with its own copy of the centre. Moving one
  leaves the bore off-centre.

Ev's answers on PR 6143:
- `rectangle(centre, width, height)` as a complete-loop form
  (`docs/PATHS-DESIGN.md` §1a), with VQ5 restated.
- **Drop the bore preset.** A profile already holds any number of
  loops, so a bore is a second circle loop authored like any other.
- How part of a profile refers to earlier parts of the same profile
  (written values and derived ones, such as an arc's centre) is its own
  question: `work/vseam/a-profile-cannot-refer-to-its-own-earlier-parts`.
