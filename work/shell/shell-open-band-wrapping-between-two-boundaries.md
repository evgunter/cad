---
id: shell-open-band-wrapping-between-two-boundaries
kind: issue
title: shell_open refuses a designated chart that wraps its period between two boundaries (a tube's outer wall); only a pole-touching wrap opens
status: closed
opened: 2026-10-06
priority: P2
cost: M
refs: [shell-open-refuses-a-curved-designated-face]
pr: 4391
closed: 2026-10-09
---


`shell_open` builds a designated chart that wraps its period as a
seamed band only through a POLE (`shell::seamed_band`: every seam runs
from the chart's boundary to one vertex nothing else reaches). A chart
that wraps between two boundaries refuses. Measured on the tube's
outer wall (`common::shell_operands::tube(0.3, 0.5, 0.4)`, its `r = 0.5`
cylinder, one face walking its seam twice), `t = 0.05`:

    OpenFaceRimNotExpressible { what: "the chart's slit loop splits into two sides neither of
      which encloses the other, so neither is the hole" }

That is the counterpart's reduction (`canonicalize_chart`'s slit
split): read in the chart, its two rims are two latitudes, and neither
encloses the other. That is right, because a band has no hole.

The rim here is TWO bands, between each boundary and its cavity twin
(the planar annular cap's two rim regions, one period over). The
pole-touching surgery generalises: cut each seam where the
counterpart's two rims cross it, and re-anchor each piece's inner end
on its ring corner as `seamed_band` does at a pole. The record's
`HoleRim` is the natural row for the second band. The pole sequence's
`kev` collapses need a vertex whose fan is the seam alone, and a seam
running boundary to boundary has none until it is cut.
The design call is whether the second band takes a `HoleRim` row with
one face or with one face per branch.

## Closed

PR 4391. A chart of one face walking one seam between two boundaries
opens as two seamed bands (`shell::band_between_boundaries`): the
guest keeps its seam through `canonicalize_chart`, the glue's ring is
split at it (`kemr`), the host seam is cut twice between the ring
corners, each outer piece re-anchored on its corner (strut `mekr` +
`kev`), the middle killed and the end's side promoted (`mfkrh`, with
the host's bit restored — the parted loop is the host's region, not a
hole). The record's choice: the second band is ONE `HoleRim` row (a row
per rim region, as `RimNaming::rim` names one face for a band of several
branches), and the divided seam's pieces are `RimNaming::seam_pieces`,
which the shell emitter names as pieces by their ends. Pinned in
`shell_curved_mouth.rs` (both tube walls, both void band walls, closed
forms, tier 3, mesh), the record audit in `verbs_shell.rs`, and the
document row in `lib_g17_shell_node.rs`. A band of several branches
between two boundaries still refuses typed (only hand-built operands
reach it: a full revolve wears a tube's wall on one face, and a band of
a genus-0 vessel disconnects its remainder).
