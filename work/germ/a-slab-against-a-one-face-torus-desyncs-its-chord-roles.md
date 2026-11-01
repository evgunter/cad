---
id: a-slab-against-a-one-face-torus-desyncs-its-chord-roles
kind: issue
title: A slab across the one-face torus refuses JoinDesync: every chord arc separates a loose scaffolding pair
status: closed
opened: 2026-10-12
branch: germ/slab-one-face-torus-roles
pr: 4906
closed: 2026-10-12
---


## What

A slab `[−6, 6] × [h, h2] × [−6, 6]` against the one-face torus
`R = 2`, `r = 0.5` about `y`
(`germ_plane_torus_frame::one_face_torus`) refuses
`JoinDesync { what: "every chord arc separates a loose scaffolding
pair" }` in all six ops (default ε), raised by `choose_roles`'
outer-loop arm (`crates/topo/src/boolean/join.rs` :3518). Both of the
slab's faces cut the torus in concentric circles, so the face pair
carries four loops.

| vertex phase | `(h, h2)` refusing |
|---|---|
| 0 | `(−0.4, −0.1)` |
| 1 | `(0.1, 0.3)`, `(−0.2, 0.3)`, `(−0.4, −0.1)` |
| π | `(−0.2, 0.3)` |

Every other combination of those phases with `(0.1, 0.3)`,
`(−0.2, 0.3)`, `(−0.4, −0.1)` and `(0.05, 0.45)` builds, and so do all
four slabs against the torus seamed on `y = 0`. The refusals are
identical on PR 4847's frozen head `59f984cb` and on its fix pass.

The closed `tang/a-wedge-across-a-full-turn-collar-desyncs-its-chord-roles`
reached the same arm through a wedge across a full-turn collar; this is
the torus's instance, where one face carries several section loops.

## Done when

Every slab above builds its true body in all six ops (volume, tier 3,
`point_in_solid`), or refuses with a typed cause.

## Closed (2026-10-12)

PR 4906. Measured at the raising site: every section circle crosses
the face's wrap meridian once, so it is a one-site loop whose segment
(`wrap_site_segments`) runs between one record's two slots, and
`partners` never pairs those. `loose_partners` therefore left every
pending one-site loop's halves partnerless, and `capture_rank` ranked
any arc holding them `Separates`; with four such loops on the face
both arcs of a chord hold one. `loose_partners` now maps a pending
one-site segment's two halves to each other. On main at `cea24921` the
refusals had grown (phase π at `(−0.4, −0.1)`, and the seamed torus
for slabs below its seam); every slab of a 594-op sweep (seven phases,
the seamed torus and the donut, eleven slabs) now builds its true body,
and none that built on main moved. Row:
`germ_plane_torus_chord::a_slab_across_a_torus_of_one_face_pair_builds_its_true_body`.
