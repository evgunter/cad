---
id: a-full-revolves-curved-wall-carries-two-seam-meridians-nobody-wrote
kind: issue
title: A full revolve's curved wall is built as two π-bands joined by two seam meridians the author did not write, against DESIGN's 'a wall keeps the cuts its author wrote'
status: parked
opened: 2026-10-28
priority: P1
cost: M
blocked_on: [loop-readers-each-derive-a-corners-turn-with-no-shared-loop-walk]
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

Related: `work/tcost/germ-cone-doc-claims-one-merged-face-where-two-half-bands-stay.md`.

## Open

Whether the two-band build is a sanctioned exception (closed-surface parametrisation needing a
seam) that the clause should state, or a cut the kernel should stop adding. Either way the clause
and the build disagree today.

## Ruled (Ev, `[ev]` PR 6014, 2026-10-29, fork row 215)

Not sanctioned: a full revolve's curved wall is ONE band. Where the wall reaches the axis, the pole or apex is reached by one wrap edge, the loop walking it out and back (the wrap tip; DESIGN `[topology]`'s seam clause and tiers 2 and 3). Ev: "the wrap tip seems ok if the complexity it adds is tolerable. might be worth investigating if there's a way to keep the complexity down". So the shared loop walk (`work/issues/loop-readers-each-derive-a-corners-turn-with-no-shared-loop-walk.md`) lands first, the tip's corner read there once. Owed here: `revolve/full.rs` builds one band (the probe's `build_wire_one_band` on `probe/foreign-wraptip`), the sweep README's minimal-sphere convention re-worded (a ball is one face, one meridian, two pole vertices), about 191 two-band assertions re-baselined, and `merge_faces`' full-period (`PeriodClosure`) refusal read again: its cap use retires (a merge closing through a wrap edge, row 75's direction), its authored k ≥ 2 circle cuts unchanged. Then import flips (`work/foreign/import-mints-the-wrap-tip-at-a-chart-singularity.md`). `design` removed: decided. (FOREIGN orchestrator)
