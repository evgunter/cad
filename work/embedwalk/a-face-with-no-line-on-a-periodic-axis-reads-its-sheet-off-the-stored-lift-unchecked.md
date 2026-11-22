---
id: a-face-with-no-line-on-a-periodic-axis-reads-its-sheet-off-the-stored-lift-unchecked
kind: issue
title: Where face_extent reads no line of a face on an axis, the face's extent on that axis is taken from the stored lift unchecked, so Sheets::Missed and the within clip rest on the lift alone there
status: open
opened: 2026-11-17
priority: P3
cost: M
---


## Finding

From EMBED lane half-sphere (PR 7086), at its reviews.

`face_extent` (`crates/topo/src/injective.rs`) is the one reader of
`Boundary::extent`. It reads the face's coordinate lines, and on a
cone its nappe, against the extent, and refuses `Structural` where
they contradict it. It reads nothing else:
- a row that is no coordinate line (a harmonic, spiric wall or spline
  row);
- a line whose face side or value does not certify;
- a cone face whose nappe is not decided.

**This is on any axis, not only a periodic one.** On an axis where the
face's boundary holds no line `face_extent` reads, the extent is taken
from the stored lift (`pcurves::Lift::read`) unchecked. `sheets`
answers before any line is read, so even a line local to the cell
does not help when its image is absent.

The second review's case: a face over `u ∈ [1, 3]` with extent
`[1, 2]` and the `u = 3` line's image absent reads `Vacuous` in
`u ∈ [2.2, 2.8]`, a cell wholly inside the face.

On such an axis, `Sheets::Missed` (read `Vacuous`), the `within` clip
in `present_chart` and `placed` all rest on the lift alone.

## Why P3

- `work/README.md` puts latent unsoundness at P3.
- The parent row (the half-sphere) was P0 because a real body, the
  freshly revolved ball, carried a row off its sheet.
- No body is known to reach this one. Since ONEDOOR's no-reset lift
  (PR 6900), the revolved ball, the revolved cone, the two-ball union
  and the filleted corner's ball were each probed.
- If a body is built that reaches it, it is P0 that day: a face read
  as holding nothing certifies any pair it is in.

## What is needed

1. Say which at-rest check establishes that the stored lift is the
   face's boundary on one sheet, if any does. Candidates:
   `validate_pcurves`, or a fresh `decide_joint` at a pole or an apex
   checked against the stored element.
2. Build the case first: a loop through a pole or an apex made of
   curves that are not meridians, for instance two small circles
   through a pole. Then read whether its extent holds the face. Also
   build a face with a row whose image is absent.
3. If no check establishes it, cross-check the extent there too. The
   enclosure of a curved row could be read against the face side its
   chart curve gives, and an absent image could refuse the extent.
