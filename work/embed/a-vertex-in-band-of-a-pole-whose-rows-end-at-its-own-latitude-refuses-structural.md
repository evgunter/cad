---
id: a-vertex-in-band-of-a-pole-whose-rows-end-at-its-own-latitude-refuses-structural
kind: issue
title: A face holding a vertex the band reads at a pole or an apex, whose rows end at the vertex's own latitude farther than chart_slack from the singular one, refuses Structural in every cell
status: open
opened: 2026-11-18
priority: P2
cost: M
---


## Finding

From the fix pass of PR 7115 (EMBED, lane pole, 2026-11-18).

`face_extent` (`crates/topo/src/injective.rs`) moves a face's extent to
a singular latitude the face holds a vertex at
(`reaching_its_singular_points`), and since the review of PR 7115 it
does so only across a gap of at most `chart_slack` of the extent,
about `1.1e-14`. A stored end farther off refuses `Structural`: the
extent and the vertex are read as not of one face.

**The two scales differ by five decades.** The vertex is at the pole by
`vertex_at_singular_point`, which reads its distance from the pole zero
within the band's zero, `1e-9` m at the default ε. A vertex anywhere in
between is at the pole to the kernel, and its meridians' pcurves end at
its own latitude, honestly, with no contradiction in the body.

- **Unit probe (uncommitted):** on the unit sphere, a lone vertex an
  arc `a` off the north pole with the stored extent ending at
  `π/2 − a`. For `a = 1e-15` and `1e-14` the extent moves to the
  latitude. For `a = 1e-13`, `1e-12`, `1e-11` and `5e-10` the face
  refuses `Structural`.
- **Not reached by any body measured:** the cavity rows, the
  injectivity suite at three ε, the tour's 85 bodies and the cone sweep
  of PR 7115 all hold. A kernel-built pole vertex sits within rounding
  of its pole. An imported body, or one built at a looser ε, is where a
  vertex a few `1e-12` off its pole would come from.
- **It is a refusal, and no worse than main**, where the same face ran
  out of pairs with its pole unplaced.

The cone has the same gap at its apex, and
`a-small-cone-tips-apex-end-may-round-past-the-nappe-slack` is the
same scale question from the nappe's side.

## What is needed

A decision on which scale says the stored end and the vertex agree:
- the extent's rounding (`chart_slack`), as now, which refuses the
  in-band vertex;
- the vertex's own angular distance from the singular point, added to
  the slack: the gap the incidence already accepted, in the chart;
- no bound, as PR 7115 first had it: a superset extent is sound for
  every reader, and the contradiction goes unreported.

Then a body with a pole vertex a few `1e-12` off its pole, pinned at
whichever answer is taken.
