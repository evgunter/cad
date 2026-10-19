---
id: a-cup-whose-mouth-lies-on-a-supports-strip-is-not-carved
kind: issue
title: blend: a dimple or a boss whose mouth lies on a support's strip inside the band's region refuses, though S − R / S ∪ F takes it
status: closed
opened: 2026-10-14
closed: 2026-10-15
pr: 5338
priority: P2
cost: H
---

Step 3's second half, from
`a-concave-fill-and-a-whole-shell-inside-a-bands-region` (its
`## Findings (step 3)`). Ev's band ruling makes a blend `S − R` (convex)
or `S ∪ F` (concave), so a dimple whose mouth lies on a support's strip
inside the band's extent and whose floor lies inside `R`, or a boss on
a strip wholly inside `F`, goes with the band. Step 3 (PR 5161) built the end-wall half only: a cup whose mouth is a cycle
on a cut-off's cap, read by the cap's three-way sliver meter.

What refuses today, and where:
- a blind hole entering from a support face inside the region refuses
  at the battery's clearance screen (`FaceClearanceUncertified`), its
  ring nearer the edge than the band sets back
  (`blend_recourse_followability::a_blind_hole_from_the_support_face_inside_the_region_refuses_at_the_screen`);
  the screen and the surgery's ring meter (`surgery.rs`,
  `ring_clearance_pass`, arm (d), the strip meter) read a support's
  rings two-way, clear or not.

What it needs:
- a three-way reading of a ring on a support against the strip the band
  removes or covers (inside the strip / crossing / outside), as the cap
  meter's `CycleReading` reads a cap's sliver, feeding `reach::Unclear`
  with the ring's crease and a depth the prism reading can project (the
  strip lies on the prism's boundary, so the projection's depth is the
  ring's distance from the trimline and from the edge, in the support's
  plane);
- the screen to stand down on a ring the three-way reading takes, so the
  verdict, not the screen, decides;
- `consume`: the ring's support face gives up its ring, as a cap does
  (the cup is a disk; the support then loses its strip to the band
  surgery, whose strip carve must find no ring there).
`reach::prism_inside` and `consume::plan` already take a cup on one
cycle; only the cycle's reading and its face's surgery are missing.

## Closed by (`band/strip-mouth-cups`)

Built as the item asks, with one correction: the mouth's depth is read
in the band's exact core less the support's side, not as its in-plane
distance from the trimline and the edge, which the fillet's arc,
tangent to the support at the trimline, does not bound. The strip
meter's three-way reading is `surgery::strip_reading`; the screen's
stand-down `battery::strip_decided`; the reach's reading
`reach::prism_inside` with a `Prism::strip`; `consume` was already
general in the face a cup's cycle lies on. Rows: `band_strip_mouth_cups`,
`editor-core` `edit_strip_mouth_cups`. A ring beside a band that is not
one link cut off at both ends still refuses at the screen:
`a-cup-on-a-strip-beside-a-band-not-cut-off-at-both-ends-is-not-carved`.
