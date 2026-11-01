---
id: axial-door-builds-a-strict-subset-when-the-caps-meeting-line-enters-the-tube
kind: issue
title: offset_charts_together returns Ok with a strict-subset body when an outward offset carries the caps' meeting line into the tube
status: open
opened: 2026-10-06
priority: P1
---


## What

A silent wrong body: `topo::offset_charts_together` returns `Ok`
with a body that is a strict subset of the true offset. Found by
SHELL's PR 4151 review (R1, finding 1) and reproduced on that PR's
head.

**The setup.** Take the fat klein elbow: the disc of `r = 0.5`
revolved a quarter turn about the spine of radius `R = 1.2`
(`common::torus_walls::klein_elbow`). Offset it outward by `d`. The
true body is the tube of radius `r + d`, cut by the two caps moved
`d` outward.

**Where it goes wrong.** The two moved caps meet in a line parallel
to the axis, `d√2` from it. Once `d√2 > R − r − d` (`d > 0.29`), that
line enters the tube. The true body then gains an edge where the two
caps meet, and each cap plane gains a second region from the far
oval.

**What the door does instead.** Every corner and edge still solves
locally, and the door returns the operand's census unchanged:
4 faces, 6 edges, 4 vertices. Measured:

- one call at `d ∈ {0.30, 0.32, 0.34}`: `Ok`;
- two calls, `d = 0.2` and then `0.12` (the first call's body passes
  `is_axial`, as the gate's roster is closed under the door's output):
  `Ok`.

**Why nothing catches it.** Tier 3 passes checks 1–6 and stops only
at check 7, `VolumeUncomputable` (`Unimplemented` for a
spiric-bounded cap). So nothing downstream names the wrong body.

## Reach

- **Which calls reach it.** The single call at `d = 0.32` builds the
  same body on `f71c22688`, before PR 4151. On that base the two-call
  path refused `TogetherNotAxial` at the second call; PR 4151's gate
  admits it.
- **`shell` cannot reach it.** A cavity moves its caps inward, and
  the rim lift returns them toward the operand's own pose. Only a
  direct call to the door, outward, gets there.

## Owed

A decision before the door builds a body whose global topology the
offset changes. The trigger is the moved caps' meeting line entering
a curved wall's moved reach; any other cap–wall pair in the same
relation would also count. `crates/topo/src/offset_axial.rs`'s module
doc ("What this door does not do") states the window and cites this
item.
