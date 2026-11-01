---
id: mate-tool-commits-one-mate
kind: issue
title: the mate tool commits one mate and its values, not a placement's whole bundle
status: open
opened: 2026-10-31
priority: P1
design: true
cost: H
---


## Finding

Ev (in chat, 2026-10-31), trying to align the corners of two stacked
bodies, found three things in the mate tool:

1. **The tool commits one mate.** Its draft is one mate plus values on
   the freedoms that mate's kind leaves (`matetool::MateFreedom`,
   `crates/viewer/src/pane/create.rs`). To place a copy by two mates the
   person commits one, then opens the tool again for the second.
2. **The second mate is refused** (`MateFault::Overconstrained`): every
   bundle pins its copy, so the first commit writes `0` values on what it
   left, and the second mate then takes freedoms values already hold.
   The kernel half of this (whether consistent mates compose, and the
   missing edge-aligning kind) is
   `work/msolve/a-consistent-second-mate-after-a-rest-is-refused.md`.
3. **The tool offers two of the five kinds and picks faces only**:
   `work/placing/the-mate-tool-offers-two-of-the-five-mate-kinds.md`.

## Ev's direction

Keep the property that no half-finished placement is ever committed
(every bundle pins its copy). Make the mate tool like the profile
editor: a draft that holds **several** mates and values for one
placement and commits them together as one bundle, one undo step. A
mate may stand **in place of** a slide or a spin the earlier mates
leave, rather than the panel offering only numbers for those freedoms.

## Also found

- The tree's `remove` button sits on every constraint row, value rows
  included. On a value row it is a no-op: `remove_constraint`
  (`crates/viewer/src/session.rs`) rewrites the bundle without it and
  writes the freed freedom as `0`, which is the value it removed. The
  startup `Place` is six value rows of `0`, so all six buttons record a
  history state and change nothing, and nothing says so (reproduced
  with `examples/drive.rs`: `click "remove" 0`, history 1 → 2 states,
  tree unchanged).
- The flip control is the checkbox "faces rest against each other"
  (`crates/viewer/src/pane/create.rs`, `drafts.mate_flip`), which reads
  as a yes/no about contact rather than as the mate's sense. Ev's
  direction (2026-10-31): a **sense** choice with two named values,
  along the lines of `flush` (normals the same way, faces level) and
  `opposed` (normals facing each other, faces resting together),
  writing the same `Flip`.
