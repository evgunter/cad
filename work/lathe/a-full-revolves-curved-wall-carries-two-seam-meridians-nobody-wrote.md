---
id: a-full-revolves-curved-wall-carries-two-seam-meridians-nobody-wrote
kind: issue
title: A full revolve's curved wall is built as two π-bands joined by two seam meridians the author did not write, against DESIGN's 'a wall keeps the cuts its author wrote'
status: closed
opened: 2026-10-28
priority: P1
cost: M
closed: 2026-11-26
pr: 7414
---


Found by the SPLITDOOR cone-apex designer pair (2026-10-28), off their question; not measured
beyond what the probe shows.

## What

`docs/DESIGN.md` [topology]: "a wall keeps the cuts its author wrote: the kernel adds none, except
where a face or edge carrier is not C¹ by its knot structure". A `Full` revolve of a profile
segment builds its curved wall as two half-turn faces joined by two seam meridians (azimuths 0
and π): on the probe cone (`splitdoor/cone-apex-probe`, `zz_probe_cone_apex` with `ZZ_PROBE=1`)
the apex vertex has an orbit of two half-edges and two faces. Those meridians are cuts nobody wrote,
and the cone is C¹ across them. They are why a cone's apex has two half-turn sectors, and they
shape every split and Boolean that meets a revolved wall.

Related: `work/green/germ-cone-doc-claims-one-merged-face-where-two-half-bands-stay.md`.

## Open

Whether the two-band build is a sanctioned exception (closed-surface parametrisation needing a
seam) that the clause should state, or a cut the kernel should stop adding. Either way the clause
and the build disagree today.

## Ruled (Ev, `[ev]` PR 6014, 2026-10-29, fork row 215)

Not sanctioned: a full revolve's curved wall is ONE band. Where the wall reaches the axis, the pole or apex is reached by one wrap edge, the loop walking it out and back (the wrap tip; DESIGN `[topology]`'s seam clause and tiers 2 and 3). Ev: "the wrap tip seems ok if the complexity it adds is tolerable. might be worth investigating if there's a way to keep the complexity down". So the shared loop walk (`work/restfront/loop-readers-each-derive-a-corners-turn-with-no-shared-loop-walk.md`) lands first, the tip's corner read there once. Owed here: `revolve/full.rs` builds one band (the probe's `build_wire_one_band` on `probe/foreign-wraptip`), the sweep README's minimal-sphere convention re-worded (a ball is one face, one meridian, two pole vertices), about 191 two-band assertions re-baselined, and `merge_faces`' full-period (`PeriodClosure`) refusal read again: its cap use retires (a merge closing through a wrap edge, row 75's direction), its authored k ≥ 2 circle cuts unchanged. Then import flips (`work/importgate/import-mints-the-wrap-tip-at-a-chart-singularity.md`). `design` removed: decided. (FOREIGN orchestrator)

## Released (2026-10-30)

`loop-readers-each-derive-a-corners-turn-with-no-shared-loop-walk` closed with PR 6088. The shared structure is `Body::both_sides_face`, read through two views: `surface_cycles` (region) and `chart_image` (spur-free: integrators and chart readers). Read the tip through these, never by a turn of your own. (RESTCURVE orchestrator)

## Held (2026-10-30)

Re-parked on `sphere-faces-take-a-per-face-chart-or-pole-vertices` (a ruling open on `[ev]` PR 6085). Ev pointed out that a sphere face other than the whole sphere has a singularity-free chart, which may retire the pole vertex and the wrap tip for spheres. Do not build on the wrap tip until it is ruled. (RESTCURVE orchestrator)

## Released (2026-10-31)

Ev ruled on `[ev]` PR 6085, decision 3 (fork-log row 222): keep the wrap tip. A tip is any valence-1 end of a wrap edge, and the wrap edge is the face's seam kept to its end. A chart reader reads a tip as the seam, through the lift's reset at the singular vertex. A chart-free reader drops it, keeping the singular vertex as the side's datum. The hold is lifted. (RESTCURVE orchestrator)

## Closed

2026-11-26, PR 7414 (LATHE orchestrator). A full revolve's curved wall is one band, and each pole or apex is that band's wrap edge's tip (Ev on `[ev]` PRs 6014 and 6085). Every consumer the change broke is fixed in the stack, each by its own reviewed PR: 7484, 7473, 7435, 7476, 7487, 7440, 7532, 7556, 7752, 7585, 7744, 7575, 7581, 7774, 7736, 7802 and 7803. The stack carries main through `41b8b180fc`. Its `ops.rs` port onto ORBIT's closed cut was dual-reviewed. The crossings-path cut-in is gated on `close_cuts`' premises, and its capability is `close-cuts-is-not-total-on-crossings-path-cut-ins` (P1). Reviews: a dual at every round (two Opus, Fable outage), both approving at the end.
