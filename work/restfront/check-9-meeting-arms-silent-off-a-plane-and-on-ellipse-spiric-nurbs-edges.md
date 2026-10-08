---
id: check-9-meeting-arms-silent-off-a-plane-and-on-ellipse-spiric-nurbs-edges
kind: issue
title: check 9's meeting arms (a ring crossing or touching its outer loop at a point) are silent on a non-planar face and on an Ellipse, Spiric or NURBS edge, so the nesting arm's no-crossing premise is still assumed there
status: open
opened: 2026-09-24
priority: P3
cost: H
---


The residue ATREST-11 left, filed at the moment it was disclosed.

Check 9's contact half (`validate::ring_outer_contact`) gained two
arms that see a ring crossing or touching its outer loop at a point no
vertex carries: arm 4 (`validate::circle_pair`, both loops whole
circles) and arm 5 (`validate::segments_meet`, every other pair of
`Line` and `Circle` edges, trims tested by the line span and by
`boolean::contain::point_on_arc`). Both run in the plane of the outer
loop's face (`validate::ring_outer_meeting`). Silent, and named at
check 9's banner and in `validate_geometric`'s not-yet-checked list:

- **A face on a non-planar surface** — a ring drilled through a
  cylinder or sphere wall. There is no plane to intersect in; the
  meeting question is a curve-on-surface one (the two loops' chart
  images, or the SSI rungs), and arms 1–3 still run there.
- **An `Ellipse`, `Spiric` or NURBS edge** on a planar face — a tilted
  section's elliptical cap, a spline profile. `meet_segment` returns
  `None` for them and the pair is skipped, the same kinds `locus_gap`
  already skips in arms 2 and 3.

What it costs: `validate::ring_nesting` places a whole ring from one
vertex on the premise that the two loops do not cross (its doc is the
premise's one home). Where either loop carries one of these edges the
premise is still ASSUMED, so a ring crossing its outer loop there,
first decided vertex inside, certifies.

No verb is known to mint the shape: `Profile` validation refuses
crossing loops, and the ATREST-11 refusal-surface measurement over
the corpus found no body a verb produces on purpose that the new arms
refuse. That measurement was a one-off instrument (a throwaway
branch that panicked on any new-arm verdict, run once over the suites,
the tour and the wild corpus) and nothing re-runs it; what guards the
claim afterwards is every row that validates a verb's output and asserts
`Ok`, which goes red if the arms start refusing it. Priority P3 (latent unsoundness), cost H (a conic/spline
meeting-point row, or a chart-space loop-crossing walk).

## Note from SHELL (2026-10-06)

`validate::ring_outer_contact`'s edge-pair arm reads a plane normal
and returns `None` on a curved face, so check 9's contact half is
blind there. Today that matters for boolean pierce rings. Once
`shell_open` builds ring-form rims on curved windows
(`work/shell/shell-open-refuses-a-curved-designated-face`), it matters
for shell too; shell's own clearance gate shields it, but its spec
will name this row rather than inherit the gap. (SHELL orchestrator)

## Note from TANG (2026-10-07)

The split-join's ring re-homing reads on the same premise
(`chord_join::first_decided`, PR 4246): a bystander ring is placed by
its first decided vertex against the run. A ring crossing the run along
one of these edges has decided vertices on both sides, and the reading
takes the first, as main's anchor reading did whenever that vertex was
decided. No public pose is known to reach it; closing this item closes
it there too. (TANG implementer)
