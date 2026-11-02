---
id: a-profile-cannot-refer-to-its-own-earlier-parts
kind: issue
title: A profile cannot refer to its own earlier parts, so a bore's centre is written twice
status: open
opened: 2026-11-02
priority: P1
cost: X
points: 30
refs: [the-rectangle-and-circle-templates-reopen-as-paths]
---

Filed by the VSEAM orchestrator from Ev's comment on `[ev]` PR 6143
(2026-11-02): "it generally seems useful to let part of a profile refer
to previous parts of the same profile (not just the ones the user has
written but also … things like the centers of arcs)".

## The gap

A profile's program is loops of steps, and each step's arguments are
written values: typed numbers, or expressions over document variables.
Nothing in one loop or step can read anything earlier in the same
profile, with one exception: a leg aimed at `Start`, the loop's own
first point. So:

- **A bore is two `circle` loops, each with its own centre.** That is
  one stated relation stored as two numbers, the shape
  `[recipe.forms]` and `[intent.checked]` name. Moving one centre leaves
  the bore off-centre. The template designers on PR 6143 agreed this is
  a defect (sure).
- **A sharing variable must be named.** An unnamed variable may have
  only one reader (`[variables]`, VR2/VR7/VR9, `Doc::shared_unnamed_vars`),
  so the only way to share a value today is for the person to name a
  document variable.
- **Derived values cannot be read at all.** An arc's centre, a leg's
  end, a fillet's tangent point, a loop's centroid: none of these is a
  stated number anywhere, so nothing can read them.

## What the designers weighed on 6143, as context and not as the option set

- An in-program address, "the centre of loop k", resolved in program
  order, as `Start` is. It holds no variable, so it is no second reader.
- Each loop defining outputs (its centre) as an operation defines output
  variables. Ev asked whether that is "the same way as bodies creating
  variables".
- Facts from that round: `profile::replay` returns one loop per step
  list; PP2 keeps one record per loop; PATHS-DESIGN §1 reads loops at
  LOOP granularity. Profiles already hold any number of loops, and
  containment decides which are holes.

## What needs deciding

What can be referred to (written arguments only, or derived points and
directions too). How a reference is spelled and stored. Whether it is a
variable, an address, or an output. What it may reach, both within the
profile and from outside it. How it interacts with step ids, names
(N1), the slot walk and range probing. And what it refuses.

## 2026-11-02 — in front of Ev (`[ev]` PR, fork-log row 236)

The designer pair converged on the core: a by-address read
`Of { step, role }` of an earlier step's emitted geometry, holding no
slot and lowered at evaluation. A third designer was dispatched after
three crossovers on two sub-questions: direction reads now or later,
and the spelling of a centre at the entry. The diff states the answer
in `docs/PATHS-DESIGN.md` §1b, `[variables.poses]` and VQ5.

## 2026-11-02 — ruled (Ev, `[ev]` PR 6289): B, 2-D pose variables in the profile's coordinates

The design is `docs/PATHS-DESIGN.md` §1b and `[variables.poses]` as
merged. A profile's points and directions are `Point2`/`Direction2`
variables in the profile's own coordinates. Written points are
anonymous `Point2`s (name one to share it). Derived values are
definitions reading an earlier piece by N1 locator (`Centre`, `Vertex`,
`Heading`), in program order. `AlongTrace` folds in as a `Direction2`
read off a traced face. A value lifts to a 3-D pose only where the
profile is drawn on a frame. Ev's reason for B: "the center of the arc
on the face at the end of the extrude" is a 3-D read off the body with
its own z, a different value from the profile's 2-D centre.

**Priced X, 30 points.** It crosses `crates/editor-core` (variables,
kinds, definitions, VR3's step-granular order rule, the slot walk,
persist), `crates/profile`, `pncad-py` and the viewer (the editor's pick
mode; it needs
`the-profile-editor-holds-kernel-steps-not-the-authored-program`
first). Cut it into units on the owning programs before dispatch. The
two mechanics the designers left unchecked are pinned in the build:
that no later step moves a piece's start once drawn, and that a read
ray is stored verbatim.
